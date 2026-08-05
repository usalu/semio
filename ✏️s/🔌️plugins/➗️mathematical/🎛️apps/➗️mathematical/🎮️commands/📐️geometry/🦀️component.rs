//! 📐️ Mathematical play app commands — replacing the geometry playground's point cloud.

use crate::apps::mathematical::config::{MathConfig, MathConfigOperation};
use crate::artifacts::mathematical::op::MathOperation;
use crate::artifacts::mathematical::{MathGeometry, MathProjection};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetPoints
pub mod set_points {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-points")]
    pub struct SetPoints {
        #[dsl(block)]
        pub geometry: MathGeometry,
    }

    pub fn handle(payload: &SetPoints, _doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathOperation, MathConfigOperation>, Fault> {
        Ok(Emit::operations(vec![MathOperation::SetGeometry { geometry: payload.geometry.clone() }]))
    }
}
//#endregion 🔖️SetPoints

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_points;
    use crate::apps::mathematical::testkit::{dispatch, math_app};
    use crate::apps::mathematical::MathCommand;
    use crate::artifacts::mathematical::{MathGeometry, MathPoint};

    #[test]
    fn set_points_replaces_geometry() {
        let mut app = math_app();
        let geometry = MathGeometry { points: vec![MathPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathCommand::SetPoints(set_points::SetPoints { geometry: geometry.clone() }));
        assert_eq!(app.projection().expect("projection").geometry, geometry);
    }
}
//#endregion 🧪️Tests
