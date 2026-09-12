use crate::editor::sequence::unit_tests::context::{dispatch, new_app};
use crate::editor::sequence::SequenceCommand;

use super::reorganize::Reorganize;
use super::set_orientation::SetOrientation;
use move_step_helper::move_all_steps_to_origin;

mod move_step_helper {
    use crate::editor::sequence::commands::step::move_step::MoveStep;
    use crate::editor::sequence::unit_tests::context::{dispatch, SequenceApp};
    use crate::editor::sequence::SequenceCommand;

    pub async fn move_all_steps_to_origin(app: &mut SequenceApp) {
        let ids: Vec<String> = app.snapshot().expect("projection").to_fixture().steps.iter().map(|step| step.id.clone()).collect();
        for id in &ids {
            dispatch(app, SequenceCommand::MoveStep(MoveStep { node_id: id.clone(), x: 0.0, y: 0.0 })).await;
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn set_orientation_command_changes_reorganize_layout_axis() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::SetOrientation(SetOrientation { value: "topBottom".into() })).await;
    move_all_steps_to_origin(&mut app).await;
    dispatch(&mut app, SequenceCommand::Reorganize(Reorganize {})).await;
    let ys: Vec<f64> = app.snapshot().expect("projection").to_fixture().steps.iter().map(|step| step.y).collect();
    assert!(ys.iter().any(|y| *y != 0.0), "topBottom orientation should spread steps vertically, got {ys:?}");
}

#[semio_framework_async_macros::async_test]
async fn reorganize_command_spreads_step_positions_apart() {
    let mut app = new_app().await;
    move_all_steps_to_origin(&mut app).await;
    dispatch(&mut app, SequenceCommand::Reorganize(Reorganize {})).await;
    let xs: Vec<f64> = app.snapshot().expect("projection").to_fixture().steps.iter().map(|step| step.x).collect();
    assert!(xs.iter().any(|x| *x != 0.0), "reorganize should spread steps apart, got {xs:?}");
}
