//! 📄️ 📄️ Sourcing curation app commands command — `load-document-json`.

use crate::standards::v1::subsets::any::schema::mutations::SourcingMutation;
use crate::standards::v1::subsets::any::io::snapshot::json::decode_curation_snapshot_json;
use crate::standards::v1::subsets::any::io::text::snapshot::sourcing_json_envelope_is_bounded;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use crate::editor::sourcing::reset_document_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "load-document-json")]
pub struct LoadDocumentJson {
    pub json: String,
}

/// 🛠️ Dev-only document load: an `Effect::LoadDocument` outside history, never a mutation row — kept out of the command palette.
pub fn handle(payload: &LoadDocumentJson, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    if !sourcing_json_envelope_is_bounded(&payload.json) {
        return Err(Fault::from("sourcing.invalid-payload: document JSON exceeds byte, depth, string, or cardinality limit"));
    }
    match decode_curation_snapshot_json(&payload.json) {
        Ok(document) => Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() }),
        Err(_) => Err(Fault::from("sourcing.invalid-payload: document schema mismatch")),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
