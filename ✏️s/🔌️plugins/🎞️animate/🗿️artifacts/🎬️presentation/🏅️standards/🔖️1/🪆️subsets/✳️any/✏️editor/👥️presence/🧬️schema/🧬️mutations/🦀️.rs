//! 🧬️ Presentation presentation.presence mutation collection.

use super::*;
#[path = "📸️replace-presence/🦀️.rs"]
mod replace_presence;
pub use replace_presence::ReplacePresence;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = PresentationPresence, diff = PresentationPresence, schema = "presentation.presence")]
pub enum PresentationPresenceMutation {
    #[dsl(key = "replace-presence")]
    ReplacePresence(ReplacePresence),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
