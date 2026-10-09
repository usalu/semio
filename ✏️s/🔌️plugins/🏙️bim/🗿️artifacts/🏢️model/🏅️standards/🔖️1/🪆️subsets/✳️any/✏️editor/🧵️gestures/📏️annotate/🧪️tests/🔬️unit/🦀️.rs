use crate::editor::bim::gestures::session::{Modifiers, ToolEvent};
use crate::editor::bim::gestures::tests::fixture::room;
use crate::editor::bim::gestures::tests::fixture::Rig;
use crate::standards::v1::subsets::any::schema::inferences::annotation_layout::StoreyAnnotations;
use crate::{AnchorEnd, AnnotationAnchor, ModelInference, ModelMutation, Point2, TagCategory, WallSide};
use protocol::Inference;

fn annotations(rig: &Rig) -> StoreyAnnotations {
    ModelInference::infer(&rig.snapshot).expect("infers").annotations.remove("st-ground").expect("the storey is annotated")
}

fn shifted(rig: &mut Rig, x: f64, y: f64) {
    let pointer = rig.pointer(x, y, Modifiers { shift: true, ..Modifiers::default() });
    rig.send(ToolEvent::Down(pointer));
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn a_dimension_is_two_anchors_and_a_place_and_the_first_one_creates_the_standard_style() {
    let mut rig = Rig::plan("dimension", room());
    assert!(rig.click(0.0, 0.0).mutations.is_empty() && rig.click(4.0, 0.0).mutations.is_empty(), "picking anchors writes nothing");
    let step = rig.click(4.0, -1.0);
    let [ModelMutation::CreateAnnotationStyle(style), ModelMutation::CreateDimension(create)] = step.mutations.as_slice() else { panic!("the style and the dimension: {:?}", step.mutations) };
    assert_eq!((create.dimension.style.as_str(), create.dimension.storey.as_str(), create.dimension.anchors.len()), (style.id.as_str(), "st-ground", 2));
    assert!(close(create.dimension.angle, 0.0) && close(create.dimension.offset, -1.0), "the line passes the pointer, one metre below the first anchor: {:?}", create.dimension);
    assert!(matches!(create.dimension.anchors[0], AnnotationAnchor::WallEnd { end: AnchorEnd::Start, .. }), "the first click is on a wall end: {:?}", create.dimension.anchors[0]);
    let set = annotations(&rig);
    assert!(close(set.dimensions.values().next().expect("one dimension").total, 4.0), "from the wall end to the point 4 m along the wall");
    rig.click(0.0, 0.0);
    rig.click(8.0, 0.0);
    let second = rig.click(4.0, -1.5);
    assert!(matches!(second.mutations.as_slice(), [ModelMutation::CreateDimension(_)]), "the style exists now: {:?}", second.mutations);
}

#[semio_framework_async_macros::async_test]
async fn two_wall_faces_measure_the_clear_distance_across_them() {
    let mut rig = Rig::plan("dimension", room());
    rig.click(4.0, 0.15);
    rig.click(4.0, 5.85);
    let step = rig.click(6.0, 3.0);
    let Some(ModelMutation::CreateDimension(create)) = step.mutations.last() else { panic!("a dimension: {:?}", step.mutations) };
    assert!(matches!(create.dimension.anchors[0], AnnotationAnchor::WallFace { side: WallSide::Left, .. }) && matches!(create.dimension.anchors[1], AnnotationAnchor::WallFace { side: WallSide::Left, .. }), "{:?}", create.dimension.anchors);
    assert!(close(create.dimension.angle, std::f64::consts::FRAC_PI_2), "across the first face: {}", create.dimension.angle);
    let layout = annotations(&rig).dimensions.values().next().cloned().expect("one dimension");
    assert!(layout.complete && (layout.total - 5.7).abs() < 1e-9, "the clear width of the room: {}", layout.total);
}

#[semio_framework_async_macros::async_test]
async fn shift_chains_more_anchors_onto_one_dimension() {
    let mut rig = Rig::plan("dimension", room());
    rig.click(0.0, 0.0);
    shifted(&mut rig, 2.0, 0.0);
    let waiting = rig.click(8.0, 0.0);
    assert!(waiting.mutations.is_empty(), "the unshifted click ends the picks and waits for the place");
    let step = rig.click(4.0, -1.0);
    let Some(ModelMutation::CreateDimension(create)) = step.mutations.last() else { panic!("a dimension") };
    assert_eq!(create.dimension.anchors.len(), 3, "the shifted click chained one more anchor: {:?}", create.dimension.anchors);
}

#[semio_framework_async_macros::async_test]
async fn enter_ends_the_picks_and_escape_forgets_them() {
    let mut rig = Rig::plan("dimension", room());
    rig.click(0.0, 0.0);
    shifted(&mut rig, 3.0, 3.0);
    rig.finish();
    let step = rig.click(1.0, 4.0);
    assert!(matches!(step.mutations.last(), Some(ModelMutation::CreateDimension(_))), "Enter fixed the anchors, the next click placed the line");
    rig.click(0.0, 0.0);
    rig.escape();
    rig.click(8.0, 0.0);
    assert!(rig.click(4.0, 1.0).mutations.is_empty(), "escape dropped the first anchor, so this click is a first pick again");
}

#[semio_framework_async_macros::async_test]
async fn the_preview_reads_the_value_the_dimension_will_print() {
    let mut rig = Rig::plan("dimension", room());
    rig.click(0.0, 0.0);
    rig.mv(8.0, 0.0);
    assert!(rig.preview.marks.iter().any(|mark| mark.text == "8.00"), "the live value follows the pointer: {:?}", rig.preview.marks);
}

#[semio_framework_async_macros::async_test]
async fn a_tag_names_the_element_under_the_pointer_and_stays_beside_it() {
    let mut rig = Rig::plan("tag", room());
    let step = rig.click(3.0, 0.05);
    let [ModelMutation::CreateAnnotationStyle(_), ModelMutation::CreateTag(create)] = step.mutations.as_slice() else { panic!("a style and a tag: {:?}", step.mutations) };
    assert_eq!((create.tag.element.as_str(), create.tag.category), ("w-south", TagCategory::Name));
    assert!(close(create.tag.offset.x, -1.0) && close(create.tag.offset.y, 0.05), "the offset is measured from the middle of the wall: {:?}", create.tag.offset);
    assert!(rig.click(50.0, 50.0).mutations.is_empty(), "nothing under the pointer, nothing tagged");
    assert_eq!(annotations(&rig).tags.values().next().expect("one tag").text, "South");
}

#[semio_framework_async_macros::async_test]
async fn a_note_is_written_where_it_is_clicked() {
    let mut rig = Rig::plan("text-note", room());
    let step = rig.click(2.0, 2.0);
    let Some(ModelMutation::CreateTextNote(create)) = step.mutations.last() else { panic!("a note: {:?}", step.mutations) };
    assert_eq!((create.text_note.position, create.text_note.text.as_str(), create.text_note.rotation), (Point2 { x: 2.0, y: 2.0 }, "Text note 1", 0.0));
}

#[semio_framework_async_macros::async_test]
async fn a_leader_points_at_a_face_and_places_its_text_beside_the_tip() {
    let mut rig = Rig::plan("leader", room());
    assert!(rig.click(4.0, -0.15).mutations.is_empty(), "the first click only picks what to point at");
    let step = rig.click(5.0, -1.15);
    let Some(ModelMutation::CreateLeader(create)) = step.mutations.last() else { panic!("a leader: {:?}", step.mutations) };
    assert!(matches!(create.leader.anchor, AnnotationAnchor::WallFace { side: WallSide::Right, .. }), "{:?}", create.leader.anchor);
    assert!(close(create.leader.offset.x, 1.0) && close(create.leader.offset.y, -1.0), "{:?}", create.leader.offset);
    let leader = annotations(&rig).leaders.values().next().cloned().expect("one leader");
    assert!(leader.reason.is_none() && close(leader.at.at.x, 5.0) && close(leader.at.at.y, -1.15));
}
