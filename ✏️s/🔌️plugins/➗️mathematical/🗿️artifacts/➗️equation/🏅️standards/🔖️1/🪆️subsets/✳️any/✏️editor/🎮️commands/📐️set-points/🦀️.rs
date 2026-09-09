//! 📐️ 📐️ Equation play app commands command — `set-points`.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::geometry::schema::mutations::replace_points::ReplacePoints;
use crate::{EquationGeometry, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-points")]
pub struct SetPoints {
    #[dsl(block)]
    pub geometry: EquationGeometry,
}

pub fn handle(payload: &SetPoints, _doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![EquationMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
