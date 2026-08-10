//! 📐️ Mathematical play app commands — replacing the geometry playground's point cloud.

use crate::apps::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetPoints
pub mod set_points {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-points")]
    pub struct SetPoints {
        #[dsl(block)]
        pub geometry: MathematicalGeometry,
    }

    pub fn handle(payload: &SetPoints, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![MathematicalMutation::SetGeometry { geometry: payload.geometry.clone() }]))
    }
}
//#endregion 🔖️SetPoints

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_points;
    use crate::apps::mathematical::testkit::{dispatch, math_app};
    use crate::apps::mathematical::MathematicalCommand;
    use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalPoint};

    #[test]
    fn set_points_replaces_geometry() {
        let mut app = math_app();
        let geometry = MathematicalGeometry { points: vec![MathematicalPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathematicalCommand::SetPoints(set_points::SetPoints { geometry: geometry.clone() }));
        assert_eq!(app.snapshot().expect("projection").geometry, geometry);
    }
}
//#endregion 🧪️Tests
