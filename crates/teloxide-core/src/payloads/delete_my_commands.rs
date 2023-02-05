//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BotCommandScope, True};

impl_payload! {
    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands] will be shown to affected users. Returns _True_ on success.
    ///
    /// [higher level commands]: https://core.telegram.org/bots/api#determining-list-of-commands
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub DeleteMyCommands (DeleteMyCommandsSetters) => True {
        optional {
            /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
            pub scope: BotCommandScope,
            /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
            pub language_code: String [into],
        }
    }
}