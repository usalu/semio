//! 🎨️ GIS 2D play app command — loading a bundled example map.

use crate::op::GisMapMutation;
use crate::schema::{default_document, positions_operations, regions_operations, routes_operations};
use crate::GisMapSnapshot;
use crate::editor::gis2d::config::{mutations as config_mutations, Gis2dConfig, Gis2dConfigMutation};
use crate::editor::gis2d::maphost::map_host_from;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetActiveExample
/// ✏️ Replaces document content by diffing every collection (positions/routes/regions) into batched
/// create/delete/replace-data operations, so this is an Operation action (not a View one) — an empty
/// `example_id` clears the map, any other id loads the bundled reuse map and frames it. Never a
/// whole-document snapshot swap (that vocabulary is retired by the taxonomy): each batched operation
/// still has a real per-mutation inverse, so undo restores the prior document exactly.
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() { GisMapSnapshot::default() } else { default_document() };
        // 🕹️ The pre-migration layer/feature selection clear that used to live here (`SetSelection {
        // ids: Vec::new() }`) is gone — selection is framework-owned config now, and `Emit` has no
        // channel to touch the framework's `interaction_store` (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). The `"features"` domain is
        // `HierarchyProvider::Flat`, so `validate_state` does not auto-prune it either — a stale
        // selection surviving a document swap is a known, accepted gap of this wave.
        let mut config_mutations = Vec::new();
        if !payload.example_id.is_empty() {
            let mut host = map_host_from(&next, cfg.snapshot);
            host.fit_world_camera();
            config_mutations.push(Gis2dConfigMutation::SetCamera(config_mutations::SetCamera { camera_json: host.camera_json() }));
        }
        let document = doc.snapshot;
        let mut artifact_mutations = positions_operations(&document.positions, &next.positions);
        artifact_mutations.extend(routes_operations(&document.routes, &next.routes));
        artifact_mutations.extend(regions_operations(&document.regions, &next.regions));
        Ok(Emit { artifact_mutations, config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
