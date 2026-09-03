//! 🧩️ 🧩️ Generation3d play app commands command — `delete-selection`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

fn delete_selected(fixture: &flow::FlowFixture, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let mut host = host_from_fixture(fixture);
    for id in selected {
        let _ = host.remove_widget(id);
    }
    let operations = commit_fixture(fixture, &host.fixture);
    Emit { artifact_mutations: operations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so it degrades to treating the selection as empty.
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(delete_selected(&doc.snapshot.fixture, &[]))
}

/// 🕹️ Reads the `graph` domain's current selection instead of a deleted config field — no config
/// mutation needed afterwards, the framework auto-prunes the deleted ids out of `graph`'s selection
/// via `interaction_topology`.
pub fn apply(
    _payload: &DeleteSelection,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(delete_selected(&doc.snapshot.fixture, &interaction.selection("graph").ids))
}
