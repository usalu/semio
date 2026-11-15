//! 🧬️ Shooting shooting.presence mutation collection.

use super::*;
#[path = "📸️replace-presence/🦀️.rs"]
mod replace_presence;
pub use replace_presence::ReplacePresence;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone, semio_framework_value::NativeReceive, semio_framework_dsl_record_derive::DslOwnedReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = ShootingPresence, diff = ShootingPresenceDiff, schema = "shooting.presence", reference = ShootingPresence, reference_diff = ShootingPresenceDiff, retained)]
pub enum ShootingPresenceMutation {
    #[dsl(key = "replace-presence")]
    ReplacePresence(ReplacePresence),
}
dsl::original_ordinal_operation_receiving!(ShootingPresenceMutation);





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
