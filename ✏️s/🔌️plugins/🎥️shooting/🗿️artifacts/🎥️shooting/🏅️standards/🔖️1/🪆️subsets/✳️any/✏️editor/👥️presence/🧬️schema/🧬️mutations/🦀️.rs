//! 🧬️ Shooting shooting.presence mutation collection.

use super::*;
#[path = "📸️replace-presence/🦀️.rs"]
mod replace_presence;
pub use replace_presence::ReplacePresence;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ShootingPresence, diff = ShootingPresence, schema = "shooting.presence")]
pub enum ShootingPresenceMutation {
    #[dsl(key = "replace-presence")]
    ReplacePresence(ReplacePresence),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
