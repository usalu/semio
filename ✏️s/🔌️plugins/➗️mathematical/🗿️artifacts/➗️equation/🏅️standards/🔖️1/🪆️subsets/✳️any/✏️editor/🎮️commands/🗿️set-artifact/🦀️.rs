//! 📄️ 📄️ Equation play app commands command — `set-artifact`.

use crate::standards::v1::subsets::any::io::text::snapshot::EquationGraphDsl;
use crate::op::EquationMutation;
use crate::standards::v1::subsets::geometry::schema::mutations::replace_points::ReplacePoints;
use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
use crate::{EquationGeometry, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-artifact")]
pub struct SetArtifact {
    #[dsl(block)]
    pub graph: EquationGraphDsl,
    #[dsl(block)]
    pub geometry: EquationGeometry,
}

pub fn handle(payload: &SetArtifact, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    let Ok(graph) = crate::standards::v1::subsets::any::io::text::snapshot::math_graph_from_dsl(payload.graph.clone()) else {
        return Ok(Emit::default());
    };
    let mut operations = Vec::new();
    if graph != doc.snapshot.graph.clone() {
        operations.push(EquationMutation::ReplaceGraph(ReplaceGraph { graph }));
    }
    if payload.geometry != doc.snapshot.geometry.clone() {
        operations.push(EquationMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() }));
    }
    Ok(Emit::mutations(operations))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
