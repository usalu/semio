use crate::editor::bim::gestures::plane::signed_area;
use crate::editor::bim::gestures::session::{Shape, REJECTED, TYPE_MISSING};
use crate::editor::bim::gestures::tests::fixture::{model, room, Rig};
use crate::{ModelMutation, RoofShape};

fn ring(vertices: &[crate::Vertex]) -> Vec<[f64; 2]> {
    vertices.iter().map(|vertex| [vertex.point.x, vertex.point.y]).collect()
}

#[semio_framework_async_macros::async_test]
async fn a_slab_polygon_is_written_counter_clockwise_when_it_finishes() {
    let mut rig = Rig::plan("slab", model());
    for (x, y) in [(0.0, 0.0), (0.0, 4.0), (5.0, 4.0), (5.0, 0.0)] {
        assert!(rig.click(x, y).mutations.is_empty(), "a corner only extends the polygon");
    }
    assert!(rig.mv(1.0, 1.0).mutations.is_empty() && rig.shows(Shape::Path));
    let step = rig.finish();
    let [ModelMutation::CreateSlab(create)] = step.mutations.as_slice() else { panic!("one create-slab, got {:?}", step.mutations) };
    assert_eq!((create.slab.slab_type.as_str(), create.slab.storey.as_str(), create.slab.offset), ("sl-200", "st-ground", 0.0));
    assert_eq!(signed_area(&ring(&create.slab.boundary)), 20.0, "a clockwise click order is written counter-clockwise");
    assert!(rig.click(9.0, 9.0).mutations.is_empty(), "the polygon is over");
}

#[semio_framework_async_macros::async_test]
async fn clicking_the_first_corner_or_double_clicking_closes_the_polygon() {
    let mut rig = Rig::plan("slab", model());
    for (x, y) in [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)] {
        rig.click(x, y);
    }
    let step = rig.click(0.004, 0.003);
    let [ModelMutation::CreateSlab(create)] = step.mutations.as_slice() else { panic!("closing on the first corner writes the slab: {step:?}") };
    assert_eq!(create.slab.boundary.len(), 3);
    let mut rig = Rig::plan("slab", model());
    for (x, y) in [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)] {
        rig.click(x, y);
    }
    assert_eq!(rig.double(4.0, 3.0).mutations.len(), 1, "a double click finishes");
}

#[semio_framework_async_macros::async_test]
async fn pressing_one_corner_and_releasing_the_opposite_one_writes_a_rectangle() {
    let mut rig = Rig::plan("slab", model());
    rig.down(1.0, 1.0);
    rig.mv(5.0, 4.0);
    assert!(rig.shows(Shape::Path), "the rectangle ghost follows the drag");
    let step = rig.up(5.0, 4.0);
    let [ModelMutation::CreateSlab(create)] = step.mutations.as_slice() else { panic!("one create-slab") };
    assert_eq!(ring(&create.slab.boundary), vec![[1.0, 1.0], [5.0, 1.0], [5.0, 4.0], [1.0, 4.0]]);
}

#[semio_framework_async_macros::async_test]
async fn escape_drops_the_polygon_and_a_missing_type_is_refused() {
    let mut rig = Rig::plan("slab", model());
    for (x, y) in [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)] {
        rig.click(x, y);
    }
    rig.escape();
    assert!(rig.finish().mutations.is_empty());
    let mut snapshot = model();
    snapshot.slab_types.clear();
    let mut rig = Rig::plan("slab", snapshot);
    for (x, y) in [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)] {
        rig.click(x, y);
    }
    assert_eq!(rig.finish().refused, Some(TYPE_MISSING));
}

#[semio_framework_async_macros::async_test]
async fn a_roof_takes_the_gable_preset_along_its_longest_edge() {
    let mut rig = Rig::plan("roof", model());
    rig.down(0.0, 0.0);
    let step = rig.up(6.0, 4.0);
    let [ModelMutation::CreateRoof(create)] = step.mutations.as_slice() else { panic!("one create-roof, got {:?}", step.mutations) };
    let RoofShape::Gable { pitch, ridge_direction } = create.roof.shape else { panic!("a gable") };
    assert!((pitch - std::f64::consts::PI / 6.0).abs() < 1e-12 && ridge_direction.sin().abs() < 1e-9, "30 degrees, ridge along x");
    assert_eq!((create.roof.roof_type.as_str(), create.roof.overhang), ("rf-200", 0.3));
}

#[semio_framework_async_macros::async_test]
async fn the_slab_from_walls_takes_the_closed_wall_loop_around_the_click() {
    let mut rig = Rig::plan("slab-walls", room());
    let step = rig.down(4.0, 3.0);
    let [ModelMutation::CreateSlab(create)] = step.mutations.as_slice() else { panic!("one create-slab, got {:?}", step.mutations) };
    let area = signed_area(&ring(&create.slab.boundary));
    assert!((area - 7.7 * 5.7).abs() < 1e-6, "the loop is the inner face of the 0.3 m walls: {area}");
    assert_eq!(rig.down(30.0, 30.0).refused, Some(REJECTED), "outside no loop is closed");
}
