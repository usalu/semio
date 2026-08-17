//! 📄️ Process 3d play app commands — wholesale document swaps (load example / set document).

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::schema::{default_document, plate_document};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetDocument
pub mod set_snapshot {
    use super::*;

    /// 📄️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see
    /// `📓️taxonomy.md`'s forbidden vocabulary), so this builds
    /// `editor::process3d::reset_process3d_document_effect` (a `Effect::LoadDocument`, outside undo
    /// history) instead of an `artifact_mutations` entry.
    ///
    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `Process3dSnapshot` dropped
    /// its `dsl::DslRecord` derive (composed `ArtifactChild<S>` fields have no `dsl::DslField` impl
    /// — see the snapshot facet's own doc comment), so this payload carries the snapshot as JSON
    /// text now, parsed at the handler — matches the migration recipe's `SetSnapshot`/
    /// `SetSnapshotJson` collapse.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document")]
    pub struct SetDocument {
        pub json: String,
    }

    pub fn handle(payload: &SetDocument, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let snapshot: Process3dSnapshot = serde_json::from_str(&payload.json).map_err(|e| Fault::from(e.to_string()))?;
        Ok(Emit { effects: vec![crate::editor::process3d::reset_process3d_document_effect(&snapshot)], ..Default::default() })
    }
}
//#endregion 🔖️SetDocument

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    /// 📄️ Loading a bundled example replaces the whole document, so it routes through
    /// `editor::process3d::reset_process3d_document_effect` (a `Effect::LoadDocument`) rather than
    /// the banned whole-snapshot mutation — see `set_snapshot::SetDocument`'s doc comment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let snapshot = match payload.example_id.as_str() {
            crate::editor::process3d::PROCESS3D_EXAMPLE_PLATE | "plate" => plate_document(),
            "" => Process3dSnapshot::default(),
            _ => default_document(),
        };
        Ok(Emit { effects: vec![crate::editor::process3d::reset_process3d_document_effect(&snapshot)], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample
