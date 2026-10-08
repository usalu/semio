use super::*;
use crate::editor::equation::unit_tests::context::{dispatch, math_app};
use crate::editor::equation::EquationCommand;
use crate::{EquationGeometry, EquationPoint};

#[semio_framework_async_macros::async_test]
async fn edit_points_applies_the_edited_geometry() {
    let mut app = math_app().await;
    let geometry = EquationGeometry { points: vec![EquationPoint { x: 1.0, y: 2.0 }] };
    dispatch(&mut app, EquationCommand::EditPoints(EditPoints { geometry: geometry.clone() })).await;
    assert_eq!((app.snapshot().expect("projection")).geometry.clone(), geometry);
}
