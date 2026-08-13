//! 🗂️ 🗂️ Wires play app commands command — `document-select`.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "document-select")]
pub struct DocumentSelect {
    pub ids: Vec<String>,
}

pub fn handle(payload: &DocumentSelect, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::config(vec![WiresConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}
