//! 📄️ 📄️ Equation play app commands command — `set-artifact`.

use crate::artifacts::equation::dsl::EquationGraphDsl;
use crate::artifacts::equation::op::EquationMutation;
use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::replace_points::mutation::ReplacePoints;
use crate::artifacts::equation::{EquationGeometry, EquationSnapshot};
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

pub async fn handle(payload: &SetArtifact, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    let Ok(graph) = crate::artifacts::equation::dsl::math_graph_from_dsl(payload.graph.clone()) else {
        return Ok(Emit::default());
    };
    let mut operations = Vec::new();
    if graph != crate::artifacts::equation::equation_graph(doc.snapshot) {
        operations.push(EquationMutation::ReplaceGraph(ReplaceGraph { graph }));
    }
    if payload.geometry != crate::artifacts::equation::equation_geometry(doc.snapshot) {
        operations.push(EquationMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() }));
    }
    Ok(Emit::mutations(operations))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::equation::EquationGeometry;
    use crate::editor::equation::testkit::{dispatch, math_app};
    use crate::editor::equation::EquationCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_artifact_replaces_graph_and_geometry() {
        let mut app = math_app();
        let geometry = EquationGeometry { points: vec![crate::artifacts::equation::EquationPoint { x: 1.0, y: 2.0 }] };
        dispatch(
            &mut app,
            EquationCommand::SetArtifact(SetArtifact {
                graph: crate::artifacts::equation::dsl::math_graph_to_dsl(&crate::artifacts::equation::EquationGraph { algorithm: "components".into(), ..Default::default() }),
                geometry: geometry.clone(),
            }),
        );
        let projection = app.snapshot().expect("projection");
        assert_eq!(crate::artifacts::equation::equation_graph(&projection).algorithm, "components");
        assert_eq!(crate::artifacts::equation::equation_geometry(&projection), geometry);
    }
}
//#endregion 🧪️Tests
