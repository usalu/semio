//! 📐️ 📐️ Mathematical play app commands command — `set-points`.

use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_points::mutation::ReplacePoints;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalSnapshot};
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-points")]
pub struct SetPoints {
    #[dsl(block)]
    pub geometry: MathematicalGeometry,
}

pub async fn handle(payload: &SetPoints, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![MathematicalMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalPoint};
    use crate::editor::mathematical::testkit::{dispatch, math_app};
    use crate::editor::mathematical::MathematicalCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_points_replaces_geometry() {
        let mut app = math_app();
        let geometry = MathematicalGeometry { points: vec![MathematicalPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathematicalCommand::SetPoints(SetPoints { geometry: geometry.clone() }));
        assert_eq!(crate::artifacts::mathematical::mathematical_geometry(&app.snapshot().expect("projection")), geometry);
    }
}
//#endregion 🧪️Tests
