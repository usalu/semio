use super::*;
use crate::editor::bim::gestures::session::Shape;
use crate::editor::bim::gestures::tests::fixture::{room, Rig};

fn created(step: &Step) -> &CreateWallSweep {
    match step.mutations.as_slice() {
        [ModelMutation::CreateWallSweep(create)] => create,
        other => panic!("one create-wall-sweep expected, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_click_beside_a_wall_runs_a_baseboard_along_the_face_the_pointer_is_on() {
    let mut rig = Rig::plan("sweep", room());
    rig.mv(2.0, 0.05);
    assert!(rig.shows(Shape::Path), "the band along the face follows the pointer");
    let step = rig.down(2.0, 0.05);
    let sweep = &created(&step).wall_sweep;
    assert_eq!((sweep.host.as_str(), sweep.side, sweep.height, sweep.inset), ("w-south", WallSide::Left, 0.0, 0.0));
    assert_eq!(sweep.profile, baseboard());
    assert_eq!(sweep.name, "Wall sweep 1", "the default name comes from the label block");
    assert!(rig.snapshot.wall_sweeps.contains_key(&created(&step).id));
    let right = Rig::plan("sweep", room()).down(2.0, -0.05);
    assert_eq!(created(&right).wall_sweep.side, WallSide::Right, "the pointer right of the travel stands the sweep on the right face");
}

#[semio_framework_async_macros::async_test]
async fn the_profile_and_the_material_come_from_the_library_selection() {
    let mut rig = Rig::plan("sweep", room());
    rig.library = vec!["m-steel".into(), "bm-20".into()];
    let step = rig.down(2.0, 0.05);
    let sweep = &created(&step).wall_sweep;
    assert_eq!((sweep.material.as_str(), sweep.profile.clone()), ("m-steel", Profile::Rectangle { width: 0.2, depth: 0.4 }));
}

#[semio_framework_async_macros::async_test]
async fn the_ghost_is_the_band_the_sweep_covers_along_the_face() {
    let mut rig = Rig::plan("sweep", room());
    rig.mv(2.0, 0.05);
    let band = rig.preview.marks.iter().find(|mark| mark.shape == Shape::Path).expect("the band");
    assert_eq!(band.style, Style::Ghost);
    let ys: Vec<f64> = band.corners().iter().map(|corner| corner[1]).collect();
    assert!(ys.iter().all(|y| *y >= 0.15 - 1e-9 && *y <= 0.17 + 1e-9), "the band lies between the left face (0.15) and two centimetres out of it: {ys:?}");
    rig.library = vec!["col-30".into()];
    let step = rig.down(2.0, 0.05);
    assert_eq!(created(&step).wall_sweep.profile, Profile::Rectangle { width: 0.3, depth: 0.3 });
}

#[semio_framework_async_macros::async_test]
async fn away_from_every_wall_nothing_happens_and_no_material_is_refused() {
    let mut rig = Rig::plan("sweep", room());
    let step = rig.down(4.0, 3.0);
    assert!(step.mutations.is_empty() && step.refused.is_none());
    rig.mv(4.0, 3.0);
    assert!(rig.preview.marks.is_empty());
    let mut snapshot = room();
    snapshot.materials.clear();
    assert_eq!(Rig::plan("sweep", snapshot).down(2.0, 0.05).refused, Some(TYPE_MISSING));
}

#[semio_framework_async_macros::async_test]
async fn the_wall_search_prefers_the_closest_wall_of_the_storey_and_ignores_curtain_walls() {
    let snapshot = room();
    let (id, _, side) = nearest_wall(&snapshot, "st-ground", [4.0, 0.1], 0.08).expect("the south wall");
    assert_eq!((id.as_str(), side > 0.0), ("w-south", true));
    assert!(nearest_wall(&snapshot, "st-first", [4.0, 0.0], 0.08).is_none(), "another storey has no wall here");
    assert!(nearest_wall(&snapshot, "st-ground", [4.0, 3.0], 0.08).is_none());
}

#[semio_framework_async_macros::async_test]
async fn a_sweep_written_by_the_tool_is_inferred_with_a_length() {
    let mut rig = Rig::plan("sweep", room());
    rig.down(2.0, 0.05);
    let id = rig.snapshot.wall_sweeps.keys().next().cloned().expect("a sweep");
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &rig.snapshot, |inference| inference.clone());
    assert!(inference.quantities.elements.get(&id).is_some_and(|row| row.length > 0.0));
}
