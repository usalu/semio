//! 🧩️ 🧩️ Procedural3d play app commands command — `delete-selection`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::{procedural3d_fixture_operations, Procedural3dMutation};
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let selected = cfg.snapshot.selected_node_ids.clone();
    let mut host = host_from_fixture(fixture);
    let mut cleared = false;
    for id in &selected {
        if host.remove_widget(id).is_ok() {
            cleared = true;
        }
    }
    let operations = commit_fixture(fixture, &host.fixture);
    let config_mutations = if cleared { vec![Procedural3dConfigMutation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
    Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
}
