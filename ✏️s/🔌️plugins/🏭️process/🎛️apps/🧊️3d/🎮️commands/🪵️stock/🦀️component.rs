//! 🪵️ Process 3d play app commands — swap the stock kind (resets the process timeline).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::apps::process3d::terminology::process3d_labels;
use crate::artifacts::process3d::{op::Process3dOperation, Pose, Process3dDocument, SolidSpec, Stock};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetStock
pub mod set_stock {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "stock")]
    pub struct SetStock {
        pub kind: String,
    }

    pub fn handle(payload: &SetStock, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        let solid = match payload.kind.as_str() {
            "cylinder" => SolidSpec::Cylinder { radius: 0.3, height: 1.0 },
            "sphere" => SolidSpec::Sphere { radius: 0.5 },
            _ => SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 },
        };
        let stock = Stock { id: fixture.stock.id.clone(), label: process3d_labels(config).stock.into(), solid, pose: Pose::default() };
        let document = Process3dDocument { workshop: fixture.workshop.clone(), stock, steps: Vec::new(), resolved_up_to: None };
        Ok(Emit { document_operations: vec![Process3dOperation::SetDocument { document }], config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }], ..Default::default() })
    }
}
//#endregion 🔖️SetStock
