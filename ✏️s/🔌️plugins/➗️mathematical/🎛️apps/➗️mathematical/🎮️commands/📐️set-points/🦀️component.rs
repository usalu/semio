//! 📐️ 📐️ Mathematical play app commands command — `set-points`.

use crate::apps::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_points::mutation::ReplacePoints;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-points")]
pub struct SetPoints {
    #[dsl(block)]
    pub geometry: MathematicalGeometry,
}

pub fn handle(payload: &SetPoints, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![MathematicalMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::mathematical::testkit::{dispatch, math_app};
    use crate::apps::mathematical::MathematicalCommand;
    use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalPoint};

    #[test]
    fn set_points_replaces_geometry() {
        let mut app = math_app();
        let geometry = MathematicalGeometry { points: vec![MathematicalPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathematicalCommand::SetPoints(SetPoints { geometry: geometry.clone() }));
        assert_eq!(crate::artifacts::mathematical::mathematical_geometry(&app.snapshot().expect("projection")), geometry);
    }
}
//#endregion 🧪️Tests
