//! ↩️ Retract a stored response through an undoable document event.
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "discard-response")]
pub struct DiscardResponse { pub id: String }

pub fn handle(payload: &DiscardResponse, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if !doc.snapshot.responses.iter().any(|response| response.id == payload.id) { return Err(Fault::from("forms-response-missing")); }
    Ok(Emit { artifact_mutations: vec![FormMutation::DiscardResponse(crate::mutations::discard_response::mutation::DiscardResponse { id: payload.id.clone() })], ..Default::default() })
}
