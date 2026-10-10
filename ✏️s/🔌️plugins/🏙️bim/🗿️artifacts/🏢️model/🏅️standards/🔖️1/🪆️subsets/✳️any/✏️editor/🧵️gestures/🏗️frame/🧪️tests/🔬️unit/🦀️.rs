use crate::editor::bim::gestures::session::{Modifiers, Shape, Surface, ToolEvent};
use crate::editor::bim::gestures::tests::fixture::{room, Rig};
use crate::{Axis, Column, CurtainGrid, CurtainPanel, CurtainWall, ModelMutation, ModelSnapshot, Phase, Point2, Slope, TopConstraint};

fn framed() -> ModelSnapshot {
    let mut snapshot = room();
    snapshot.columns.insert("c-1".into(), Column { storey: "st-ground".into(), column_type: "col-30".into(), position: Point2 { x: 2.0, y: 2.0 }, rotation: 0.0, tilt: None, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, name: "Column".into() });
    snapshot.curtain_walls.insert(
        "cw-1".into(),
        CurtainWall {
            storey: "st-ground".into(),
            curtain_wall_type: "cw-fa".into(),
            axis: Axis::Line { start: Point2 { x: 0.0, y: 8.0 }, end: Point2 { x: 6.0, y: 8.0 } },
            base_offset: 0.0,
            top: TopConstraint::StoreyTop { offset: 0.0 },
            u_grid: None,
            v_grid: None,
            phase: Phase::New,
            name: "Facade".into(),
        },
    );
    snapshot
}

fn shifted(rig: &mut Rig, x: f64, y: f64) -> crate::editor::bim::gestures::session::Step {
    let pointer = rig.pointer(x, y, Modifiers { shift: true, ..Modifiers::default() });
    rig.send(ToolEvent::Down(pointer))
}

#[semio_framework_async_macros::async_test]
async fn a_column_and_then_the_point_over_its_top_lean_the_column_there() {
    let mut rig = Rig::plan("column-tilt", framed());
    assert!(rig.down(2.0, 2.0).mutations.is_empty(), "the first click takes the column");
    rig.mv(2.3, 2.0);
    assert!(rig.shows(Shape::Label), "the lean shows as an angle while the pointer moves");
    let step = rig.down(2.3, 2.0);
    let [ModelMutation::SetColumnTilt(tilt)] = step.mutations.as_slice() else { panic!("one set-column-tilt, got {:?}", step.mutations) };
    let Some(Slope { direction, angle }) = tilt.tilt else { panic!("a lean") };
    assert!(direction.abs() < 1e-9 && (angle - (0.3_f64 / 3.0).atan()).abs() < 1e-9, "{direction} {angle}");
    assert_eq!(rig.snapshot.columns["c-1"].tilt, tilt.tilt);
}

#[semio_framework_async_macros::async_test]
async fn the_point_over_the_foot_makes_the_column_plumb_and_a_click_elsewhere_takes_none() {
    let mut rig = Rig::plan("column-tilt", framed());
    rig.snapshot.columns.get_mut("c-1").expect("column").tilt = Some(Slope { direction: 0.0, angle: 0.1 });
    rig.down(2.0, 2.0);
    rig.down(2.0, 2.0);
    assert_eq!(rig.snapshot.columns["c-1"].tilt, None);
    assert!(rig.down(5.0, 5.0).mutations.is_empty(), "no column there");
    assert!(rig.down(5.0, 5.0).mutations.is_empty(), "still no column there");
}

#[semio_framework_async_macros::async_test]
async fn a_click_on_a_curtain_wall_adds_a_grid_line_and_a_shift_click_takes_the_nearest_away() {
    let mut rig = Rig::plan("curtain-grid", framed());
    let step = rig.down(2.0, 8.0);
    let [ModelMutation::SetCurtainWallGrid(grid)] = step.mutations.as_slice() else { panic!("one set-curtain-wall-grid, got {:?}", step.mutations) };
    assert!(grid.v_grid.is_none(), "the plan runs along the wall only");
    assert_eq!(rig.snapshot.curtain_walls["cw-1"].u_grid, Some(CurtainGrid::Lines { positions: vec![1.5, 2.0, 3.0, 4.5] }));
    assert!(rig.down(2.02, 8.0).mutations.is_empty(), "a line next to a line is not a cell");
    assert!(!shifted(&mut rig, 3.1, 8.0).mutations.is_empty());
    assert_eq!(rig.snapshot.curtain_walls["cw-1"].u_grid, Some(CurtainGrid::Lines { positions: vec![1.5, 2.0, 4.5] }));
    assert!(shifted(&mut rig, 5.9, 8.0).mutations.is_empty(), "no line within reach of the click");
    assert!(rig.down(5.0, 5.0).mutations.is_empty(), "no curtain wall there");
}

#[semio_framework_async_macros::async_test]
async fn a_section_along_the_wall_adds_a_line_up_the_wall() {
    let surface = Surface::Section { start: [0.0, 8.0], end: [6.0, 8.0] };
    let mut rig = Rig::on("curtain-grid", framed(), surface);
    let step = rig.down(1.0, 1.0);
    let [ModelMutation::SetCurtainWallGrid(grid)] = step.mutations.as_slice() else { panic!("one set-curtain-wall-grid, got {:?}", step.mutations) };
    assert!(grid.u_grid.is_none() && grid.v_grid.is_some());
    let Some(CurtainGrid::Lines { positions }) = rig.snapshot.curtain_walls["cw-1"].v_grid.clone() else { panic!("explicit lines") };
    assert!(positions.contains(&1.0), "{positions:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_click_on_a_cell_opens_it_and_a_second_click_gives_it_back_to_the_type() {
    let mut rig = Rig::plan("curtain-cell", framed());
    let step = rig.down(0.5, 8.0);
    let [ModelMutation::CreateCurtainPanelOverride(create)] = step.mutations.as_slice() else { panic!("one create, got {:?}", step.mutations) };
    assert_eq!((create.curtain_panel_override.curtain.as_str(), create.curtain_panel_override.u, create.curtain_panel_override.v), ("cw-1", 0, 0));
    assert_eq!(create.curtain_panel_override.panel, CurtainPanel::Empty);
    let step = rig.down(0.6, 8.0);
    assert!(matches!(step.mutations.as_slice(), [ModelMutation::DeleteCurtainPanelOverride(_)]));
    assert!(rig.snapshot.curtain_panel_overrides.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_shift_click_makes_the_cell_a_door_and_changes_an_open_cell_into_one() {
    let mut rig = Rig::plan("curtain-cell", framed());
    let step = shifted(&mut rig, 3.5, 8.0);
    let [ModelMutation::CreateCurtainPanelOverride(create)] = step.mutations.as_slice() else { panic!("one create, got {:?}", step.mutations) };
    assert_eq!((create.curtain_panel_override.u, create.curtain_panel_override.panel.clone()), (2, CurtainPanel::Door { door_type: "door-09".into() }));
    rig.down(0.5, 8.0);
    assert_eq!(rig.snapshot.curtain_panel_overrides.len(), 2);
    let step = shifted(&mut rig, 0.5, 8.0);
    assert!(matches!(step.mutations.as_slice(), [ModelMutation::SetCurtainPanelOverride(set)] if set.panel == CurtainPanel::Door { door_type: "door-09".into() }));
}

#[semio_framework_async_macros::async_test]
async fn the_arc_beam_takes_a_start_an_end_and_a_point_on_the_arc() {
    let mut rig = Rig::plan("beam-arc", framed());
    assert!(rig.down(0.0, 4.0).mutations.is_empty());
    assert!(rig.down(4.0, 4.0).mutations.is_empty(), "the end waits for the bulge");
    let step = rig.down(2.0, 5.0);
    let [ModelMutation::CreateBeam(create)] = step.mutations.as_slice() else { panic!("one create-beam, got {:?}", step.mutations) };
    let Axis::Arc { bulge, .. } = create.beam.axis else { panic!("an arc axis") };
    assert!(bulge.abs() > 1e-6, "{bulge}");
    assert!(create.beam.end_top_offset.is_none());
}
