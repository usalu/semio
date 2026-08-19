//! 🧩️ 🧩️ Procedural3d play app commands command — `delete-selection`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

async fn delete_selected(fixture: &flow::FlowFixture, selected: &[String]) -> Emit<Procedural3dMutation, Procedural3dConfigMutation> {
    let mut host = host_from_fixture(fixture);
    for id in selected {
        let _ = host.remove_widget(id);
    }
    let operations = commit_fixture(fixture, &host.fixture);
    Emit { artifact_mutations: operations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Procedural3dPlayApp::handle` always routes this
/// command through `apply` below instead), so it degrades to treating the selection as empty.
pub async fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(delete_selected(&doc.snapshot.fixture, &[]))
}

/// 🕹️ Reads the `graph` domain's current selection instead of a deleted config field — no config
/// mutation needed afterwards, the framework auto-prunes the deleted ids out of `graph`'s selection
/// via `interaction_topology`.
pub async fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, interaction: &InteractionView<'_>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(delete_selected(&doc.snapshot.fixture, &interaction.selection("graph").ids))
}
