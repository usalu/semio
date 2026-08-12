//! 📄️ Process 3d play app commands — wholesale document swaps (load example / set document).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::schema::{default_document, plate_document};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetDocument
pub mod set_snapshot {
    use super::*;

    /// 📄️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see
    /// `📓️taxonomy.md`'s forbidden vocabulary), so this builds
    /// `apps::process3d::reset_process3d_document_effect` (a `HostEffect::LoadDocument`, outside undo
    /// history) instead of an `artifact_mutations` entry.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document")]
    pub struct SetDocument {
        #[dsl(block)]
        pub snapshot: Process3dSnapshot,
    }

    pub fn handle(payload: &SetDocument, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit {
            effects: vec![crate::apps::process3d::reset_process3d_document_effect(&payload.snapshot)],
            config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetDocument

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    /// 📄️ Loading a bundled example replaces the whole document, so it routes through
    /// `apps::process3d::reset_process3d_document_effect` (a `HostEffect::LoadDocument`) rather than
    /// the banned whole-snapshot mutation — see `set_snapshot::SetDocument`'s doc comment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let snapshot = match payload.example_id.as_str() {
            crate::apps::process3d::PROCESS3D_EXAMPLE_PLATE | "plate" => plate_document(),
            "" => Process3dSnapshot::default(),
            _ => default_document(),
        };
        Ok(Emit {
            effects: vec![crate::apps::process3d::reset_process3d_document_effect(&snapshot)],
            config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetActiveExample
