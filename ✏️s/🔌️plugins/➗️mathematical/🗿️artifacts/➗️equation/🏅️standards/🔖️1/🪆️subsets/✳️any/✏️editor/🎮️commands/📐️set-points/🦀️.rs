//! 📐️ 📐️ Equation play app commands command — `set-points`.

use crate::artifacts::equation::op::EquationMutation;
use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::replace_points::mutation::ReplacePoints;
use crate::artifacts::equation::{EquationGeometry, EquationSnapshot};
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-points")]
pub struct SetPoints {
    #[dsl(block)]
    pub geometry: EquationGeometry,
}

pub async fn handle(payload: &SetPoints, _doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![EquationMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::equation::{EquationGeometry, EquationPoint};
    use crate::editor::equation::testkit::{dispatch, math_app};
    use crate::editor::equation::EquationCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_points_replaces_geometry() {
        let mut app = math_app();
        let geometry = EquationGeometry { points: vec![EquationPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, EquationCommand::SetPoints(SetPoints { geometry: geometry.clone() }));
        assert_eq!(crate::artifacts::equation::equation_geometry(&app.snapshot().expect("projection")), geometry);
    }
}
//#endregion 🧪️Tests
