//! ↩️ Retract a stored response through an undoable document event.
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "discard-response")]
pub struct DiscardResponse { pub id: String }

pub fn handle(payload: &DiscardResponse, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if !doc.snapshot.responses.iter().any(|response| response.id == payload.id) { return Err(Fault::from("forms-response-missing")); }
    Ok(Emit { artifact_mutations: vec![FormMutation::DiscardResponse(crate::mutations::discard_response::mutation::DiscardResponse { id: payload.id.clone() })], ..Default::default() })
}
