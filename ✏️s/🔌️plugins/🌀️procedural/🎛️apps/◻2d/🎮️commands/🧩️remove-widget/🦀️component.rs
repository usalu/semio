//! 🧩️ 🧩️ Procedural2d play app commands command — `remove-widget`.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_from_fixture;
use crate::artifacts::procedural2d::op::{procedural2d_fixture_operations, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::artifacts::procedural2d::schema::host_operations;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String}

pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let target_id = &payload.widget_id;
    let operations = host_operations(fixture, |host| {
        let _ = host.remove_widget(target_id);
    });
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    let remaining: Vec<String> = cfg.snapshot.selected_ids.iter().filter(|id| *id != target_id).cloned().collect();
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Procedural2dConfigMutation::SetSelection { ids: remaining }], ..Default::default() })
}
