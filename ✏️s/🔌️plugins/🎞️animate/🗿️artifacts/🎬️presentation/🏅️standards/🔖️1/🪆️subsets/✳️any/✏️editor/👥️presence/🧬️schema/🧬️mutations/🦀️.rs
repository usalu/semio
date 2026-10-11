//! 🧬️ Presentation presentation.presence mutation collection.

use super::*;
#[path = "📸️replace-presence/🦀️.rs"]
mod replace_presence;
pub use replace_presence::ReplacePresence;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = PresentationPresence, diff = PresentationPresenceDiff, schema = "presentation.presence")]
pub enum PresentationPresenceMutation {
    #[dsl(key = "replace-presence")]
    ReplacePresence(ReplacePresence),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
