use super::*;
use crate::editor::equation::testkit::{dispatch, math_app};
use crate::editor::equation::EquationCommand;
use crate::{EquationGeometry, EquationPoint};

#[semio_framework_async_macros::async_test]
async fn set_points_replaces_geometry() {
    let mut app = math_app().await;
    let geometry = EquationGeometry { points: vec![EquationPoint { x: 1.0, y: 2.0 }] };
    dispatch(&mut app, EquationCommand::SetPoints(SetPoints { geometry: geometry.clone() })).await;
    assert_eq!(crate::equation_geometry(&app.snapshot().expect("projection")), geometry);
}
