//! 🪵️ Process 3d play app commands — swap the stock kind (resets the process timeline).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::apps::process3d::terminology::process3d_labels;
use crate::artifacts::process3d::{op::Process3dMutation, Pose, Process3dSnapshot, SolidSpec, Stock};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetStock
pub mod set_stock {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "stock")]
    pub struct SetStock {
        pub kind: String,
    }

    /// 🪵️ Swapping the stock kind resets the whole process timeline (steps + cursor), which is a
    /// whole-document replace, not a targeted edit — no in-history mutation exists for that (see
    /// `📓️taxonomy.md`'s forbidden vocabulary), so this routes through
    /// `apps::process3d::reset_process3d_document_effect` (a `HostEffect::LoadDocument`) instead.
    pub fn handle(payload: &SetStock, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        let solid = match payload.kind.as_str() {
            "cylinder" => SolidSpec::Cylinder { radius: 0.3, height: 1.0 },
            "sphere" => SolidSpec::Sphere { radius: 0.5 },
            _ => SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 },
        };
        let stock = Stock { id: fixture.stock.id.clone(), label: process3d_labels(config).stock.into(), solid, pose: Pose::default() };
        let snapshot = Process3dSnapshot { workshop: fixture.workshop.clone(), stock, steps: Vec::new(), resolved_up_to: None };
        Ok(Emit {
            effects: vec![crate::apps::process3d::reset_process3d_document_effect(&snapshot)],
            config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetStock
