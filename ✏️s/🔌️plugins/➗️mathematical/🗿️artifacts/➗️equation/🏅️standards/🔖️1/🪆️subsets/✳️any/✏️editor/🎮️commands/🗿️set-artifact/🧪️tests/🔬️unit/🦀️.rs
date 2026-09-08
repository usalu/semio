
use super::*;
use crate::EquationGeometry;
use crate::editor::equation::EquationCommand;
use crate::editor::equation::testkit::{dispatch, math_app};

#[semio_framework_async_macros::async_test]
async fn set_artifact_replaces_graph_and_geometry() {
    let mut app = math_app().await;
    let geometry = EquationGeometry { points: vec![crate::EquationPoint { x: 1.0, y: 2.0 }] };
    dispatch(&mut app, EquationCommand::SetArtifact(SetArtifact { graph: crate::document_dsl::math_graph_to_dsl(&crate::EquationGraph { algorithm: "components".into(), ..Default::default() }), geometry: geometry.clone() })).await;
    let projection = app.snapshot().expect("projection");
    assert_eq!(crate::equation_graph(&projection).algorithm, "components");
    assert_eq!(crate::equation_geometry(&projection), geometry);
}
