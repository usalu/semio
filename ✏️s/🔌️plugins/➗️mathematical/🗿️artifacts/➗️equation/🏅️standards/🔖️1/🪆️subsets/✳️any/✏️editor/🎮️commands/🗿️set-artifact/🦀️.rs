//! 📄️ 📄️ Equation play app commands command — `set-artifact`.

use crate::document_dsl::EquationGraphDsl;
use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
use crate::standards::v1::subsets::geometry::schema::mutations::replace_points::ReplacePoints;
use crate::{EquationGeometry, EquationSnapshot};
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-artifact")]
pub struct SetArtifact {
    #[dsl(block)]
    pub graph: EquationGraphDsl,
    #[dsl(block)]
    pub geometry: EquationGeometry,
}

pub fn handle(payload: &SetArtifact, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    let Ok(graph) = crate::document_dsl::math_graph_from_dsl(payload.graph.clone()) else {
        return Ok(Emit::default());
    };
    let mut operations = Vec::new();
    if graph != crate::equation_graph(doc.snapshot) {
        operations.push(EquationMutation::ReplaceGraph(ReplaceGraph { graph }));
    }
    if payload.geometry != crate::equation_geometry(doc.snapshot) {
        operations.push(EquationMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() }));
    }
    Ok(Emit::mutations(operations))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
