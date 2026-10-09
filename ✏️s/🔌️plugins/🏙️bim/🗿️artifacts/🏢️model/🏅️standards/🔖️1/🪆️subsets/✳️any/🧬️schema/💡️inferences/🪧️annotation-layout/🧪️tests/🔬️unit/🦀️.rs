use super::*;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanKind;
use crate::{AnnotationStylePatch, Axis, ColumnPatch, Entry, ModelDiff, ModelInference, StoreyPatch, WallPatch};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ROOM: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json");

fn room() -> ModelSnapshot {
    from_json_str(ROOM, JsonMemberPolicy::Reject).expect("the room decodes")
}

fn annotations(snapshot: &ModelSnapshot) -> StoreyAnnotations {
    ModelInference::infer(snapshot).expect("infers").annotations.remove("st-ground").expect("the storey is annotated")
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn line(from: (f64, f64), to: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: from.0, y: from.1 }, end: Point2 { x: to.0, y: to.1 } }
}

fn codes(set: &StoreyAnnotations, id: &str) -> Vec<DiagnosticCode> {
    set.findings.iter().filter(|finding| finding.elements == [id]).map(|finding| finding.code).collect()
}

#[test]
fn a_dimension_measures_the_current_geometry_of_its_anchors() {
    let set = annotations(&room());
    let measured = |id: &str| set.dimensions[id].segments.iter().map(|segment| segment.length).collect::<Vec<_>>();
    assert_eq!(measured("dim-south"), vec![8.0]);
    assert_eq!(measured("dim-chain").len(), 2);
    assert!(close(measured("dim-chain")[0], 2.0) && close(measured("dim-chain")[1], 6.0), "the window centre splits the wall at 2 m: {:?}", measured("dim-chain"));
    assert!(close(measured("dim-clear")[0], 5.7), "the clear width between the interior faces is 6 - 0.3: {:?}", measured("dim-clear"));
    assert!(close(measured("dim-thickness")[0], 0.3), "the faces of a wall are its thickness apart: {:?}", measured("dim-thickness"));
    assert!(close(measured("dim-column")[0], 3.0), "the column stands 3 m from the grid line: {:?}", measured("dim-column"));
    assert_eq!(set.dimensions["dim-south"].segments[0].text, "8.00");
    assert_eq!(set.dimensions["dim-chain"].total_text, "8.00");
    assert!(set.dimensions.values().filter(|layout| layout.complete).all(|layout| layout.anchors.iter().all(|anchor| anchor.reason.is_none())));
}

#[test]
fn the_dimension_line_sits_the_offset_beside_the_first_anchor_and_the_text_above_it() {
    let set = annotations(&room());
    let south = &set.dimensions["dim-south"];
    assert!(south.anchors.iter().all(|anchor| close(anchor.mark.y, -1.0)), "an offset of -1 puts the line one metre to the right of the measuring direction, below the wall");
    assert!(close(south.anchors[0].mark.x, 0.0) && close(south.anchors[1].mark.x, 8.0));
    assert!(south.anchors[0].extension.is_some_and(|line| close(line.start.y, -0.1) && close(line.end.y, -1.2)), "the extension line starts a gap from the anchor and passes the line by the overshoot");
    assert!(close(south.segments[0].text_at.at.x, 4.0) && close(south.segments[0].text_at.at.y, -1.0 + 0.25 * 0.6), "the text is centred and raised by 0.6 of its height");
    assert_eq!(south.segments[0].text_at.rotation, 0.0);
}

#[test]
fn texts_read_left_to_right_whatever_the_measuring_direction() {
    assert_eq!(dimensions::readable(0.0), 0.0);
    assert!(close(dimensions::readable(std::f64::consts::PI), 0.0 + 2.0 * std::f64::consts::PI) || close(dimensions::readable(std::f64::consts::PI), 2.0 * std::f64::consts::PI));
    assert!(close(dimensions::readable(-std::f64::consts::FRAC_PI_2), std::f64::consts::FRAC_PI_2));
    assert!(close(dimensions::readable(std::f64::consts::FRAC_PI_2), std::f64::consts::FRAC_PI_2));
}

#[test]
fn a_value_prints_in_the_unit_and_the_precision_of_its_style() {
    assert_eq!(print(3.5, DimensionUnit::Metre, 2), "3.50");
    assert_eq!(print(3.5, DimensionUnit::Centimetre, 1), "350.0");
    assert_eq!(print(3.5, DimensionUnit::Millimetre, 0), "3500");
    assert_eq!(print(-0.0001, DimensionUnit::Metre, 2), "0.00", "negative zero prints as zero");
}

#[test]
fn moving_an_anchored_wall_changes_the_printed_value_through_inference() {
    let before = room();
    let mut session = ModelInferenceSession::new();
    session.refresh(&before);
    let moved = ModelDiff::walls("w-south", Entry::Patched(WallPatch { axis: Some(line((0.0, 0.0), (9.0, 0.0))), ..Default::default() }));
    let after = protocol::apply_diff(&moved, &before).expect("applies");
    let incremental = session.update(&after, &moved).clone();
    let set = &incremental.annotations["st-ground"];
    assert!(close(set.dimensions["dim-south"].total, 9.0) && set.dimensions["dim-south"].segments[0].text == "9.00", "the dimension of the wall prints the new length");
    assert!(close(set.dimensions["dim-chain"].segments[1].length, 7.0), "the span after the window grew with the wall");
    assert_eq!(session.report().computed_by_kind.get("annotation"), Some(&1), "one annotation node is recomputed: {:?}", session.report());
    assert_eq!(incremental, ModelInference::infer(&after).expect("infers"), "the incremental result equals a fresh inference");
    assert!(after.dimensions["dim-south"].anchors.len() == 2 && after.dimensions == before.dimensions, "the authored dimension is untouched: only the geometry moved");
}

#[test]
fn an_edit_that_none_of_the_annotations_read_does_not_recompute_them() {
    let before = room();
    let mut session = ModelInferenceSession::new();
    session.refresh(&before);
    let raised = ModelDiff::storeys("st-first", Entry::Patched(StoreyPatch { height: Some(3.2), ..Default::default() }));
    let after = protocol::apply_diff(&raised, &before).expect("applies");
    let incremental = session.update(&after, &raised).clone();
    assert_eq!(session.report().computed_by_kind.get("annotation"), None, "raising the storey above recomputes no annotation: {:?}", session.report());
    assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
}

#[test]
fn a_style_edit_recomputes_the_annotations_of_the_storey_and_no_wall_layout() {
    let before = room();
    let mut session = ModelInferenceSession::new();
    session.refresh(&before);
    let restyled = ModelDiff::annotation_styles("as-plan", Entry::Patched(AnnotationStylePatch { precision: Some(0), ..Default::default() }));
    let after = protocol::apply_diff(&restyled, &before).expect("applies");
    let incremental = session.update(&after, &restyled).clone();
    assert_eq!(incremental.annotations["st-ground"].dimensions["dim-south"].segments[0].text, "8");
    assert_eq!(session.report().computed_by_kind.get("annotation"), Some(&1));
    assert_eq!(session.report().computed_by_kind.get("wall-layout"), None);
    assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
}

#[test]
fn tags_read_their_element_and_follow_it() {
    let before = room();
    let set = annotations(&before);
    assert_eq!(set.tags["tag-name"].text, "South");
    assert_eq!(set.tags["tag-type"].text, "Brick 300");
    assert_eq!(set.tags["tag-size"].text, "1.20 \u{d7} 1.20");
    assert_eq!(set.tags["tag-column"].text, "0.30 \u{d7} 0.30");
    assert!(close(set.tags["tag-column"].at.at.x, 4.5) && close(set.tags["tag-column"].at.at.y, 3.5), "the tag sits at its offset from the column position");
    let moved = ModelDiff::columns("col-1", Entry::Patched(ColumnPatch { position: Some(Point2 { x: 5.0, y: 2.0 }), ..Default::default() }));
    let after = protocol::apply_diff(&moved, &before).expect("applies");
    let next = annotations(&after);
    assert!(close(next.tags["tag-column"].at.at.x, 5.5) && close(next.tags["tag-column"].at.at.y, 2.5), "moving the column moves its tag by inference");
    assert_eq!(next.tags["tag-column"].text, set.tags["tag-column"].text);
    assert!(after.tags == before.tags, "the authored tag is untouched");
}

#[test]
fn a_leader_points_at_its_anchor_and_a_note_stays_where_it_was_put() {
    let set = annotations(&room());
    let leader = &set.leaders["lead-1"];
    assert!(leader.reason.is_none() && close(leader.tip.x, 4.0) && close(leader.tip.y, -0.15), "the tip is the middle of the exterior face: {:?}", leader.tip);
    assert!(close(leader.at.at.x, 5.0) && close(leader.at.at.y, -1.15));
    let note = &set.notes["note-1"];
    assert!(close(note.at.at.x, 2.0) && close(note.at.at.y, 2.0) && note.text == "Verify on site");
}

#[test]
fn a_lock_is_checked_and_never_moves_anything() {
    let before = room();
    let set = annotations(&before);
    assert_eq!(codes(&set, "dim-lock-broken"), vec![DiagnosticCode::DimensionLockViolated]);
    assert!(codes(&set, "dim-locked").is_empty(), "a kept lock raises nothing");
    let broken = &set.dimensions["dim-lock-broken"];
    assert!(close(broken.total, 3.0) && broken.lock_difference.is_some_and(|difference| close(difference, 0.5)), "the value stays what the geometry says");
    let finding = set.findings.iter().find(|finding| finding.code == DiagnosticCode::DimensionLockViolated).expect("reported");
    assert!(close(finding.values["difference"], 0.5) && close(finding.values["lock"], 2.5) && close(finding.values["length"], 3.0));
    assert!(finding.text("en").is_some_and(|text| text.contains("locked to 2.5 m")) && finding.text("de").is_some_and(|text| text.contains("gesperrt")));
    assert!(before.dimensions["dim-lock-broken"].anchors.iter().all(|anchor| anchor.free_point().is_some()), "the anchors are where they were authored");
}

#[test]
fn an_anchor_without_geometry_is_a_finding_and_leaves_no_number() {
    let set = annotations(&room());
    assert!(!set.dimensions["dim-parallel"].complete && set.dimensions["dim-parallel"].segments.is_empty() && set.dimensions["dim-parallel"].total_text.is_empty());
    assert_eq!(set.dimensions["dim-parallel"].anchors[0].reason, Some(Reason::Parallel));
    assert!(codes(&set, "dim-parallel").contains(&DiagnosticCode::AnnotationAnchorUnresolved));
    assert_eq!(codes(&set, "tag-empty"), vec![DiagnosticCode::TagEmpty], "a wall has no number");
}

#[test]
fn a_missing_element_and_a_missing_style_are_errors() {
    let mut snapshot = room();
    snapshot.walls.remove("w-west");
    snapshot.columns.remove("col-1");
    snapshot.annotation_styles.clear();
    let set = annotations(&snapshot);
    assert!(codes(&set, "dim-column").contains(&DiagnosticCode::AnnotationAnchorMissing));
    assert!(codes(&set, "tag-column").contains(&DiagnosticCode::AnnotationAnchorMissing));
    assert!(codes(&set, "dim-south").contains(&DiagnosticCode::AnnotationStyleMissing));
    assert_eq!(set.dimensions["dim-south"].style, StyleMarks::default(), "a missing style draws with the standard one");
    let missing = set.findings.iter().find(|finding| finding.code == DiagnosticCode::AnnotationAnchorMissing && finding.elements == ["dim-column"]).expect("reported");
    assert_eq!(missing.missing, vec!["col-1".to_string()]);
}

#[test]
fn the_plan_draws_what_the_annotations_derived() {
    let inferred = ModelInference::infer(&room()).expect("infers");
    let plan = &inferred.plan_linework["st-ground"];
    assert_eq!(plan.count_of(PlanKind::DimensionText), 8, "south 1, chain 2, clear 1, thickness 1, column 1, kept lock 1, broken lock 1; the parallel dimension draws nothing");
    assert_eq!(plan.count_of(PlanKind::TagText), 4, "the empty tag prints nothing");
    assert_eq!((plan.count_of(PlanKind::NoteText), plan.count_of(PlanKind::LeaderText), plan.count_of(PlanKind::LeaderLine)), (1, 1, 1));
    assert!(plan.texts.iter().any(|text| text.kind == PlanKind::DimensionText && text.label == "8.00" && text.element == "dim-south" && close(text.height, 0.25)));
    assert!(plan.count_of(PlanKind::DimensionMark) >= 2 * 7 && plan.count_of(PlanKind::DimensionLine) == 7);
    assert!(close(plan.length_of(PlanKind::DimensionLine), 8.0 + 8.0 + 5.7 + 0.3 + 3.0 + 3.0 + 3.0), "the dimension lines span the measured distances: {}", plan.length_of(PlanKind::DimensionLine));
}

#[test]
fn the_node_exists_only_for_annotated_storeys_and_names_its_face_walls_as_parents() {
    let snapshot = room();
    assert!(is_annotated(&snapshot, "st-ground") && !is_annotated(&snapshot, "st-first"));
    assert_eq!(face_walls(&snapshot, "st-ground").into_iter().collect::<Vec<_>>(), vec!["w-east", "w-north", "w-south"]);
    let steps = crate::standards::v1::subsets::any::schema::inferences::model_graph::plan::build(&snapshot, crate::standards::v1::subsets::any::schema::inferences::model_graph::kinds::closure(crate::standards::v1::subsets::any::schema::inferences::model_graph::kinds::ANNOTATIONS));
    let node = steps.iter().find(|step| matches!(&step.key, crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelNode::Annotation(_))).expect("planned");
    assert_eq!(node.parents.len(), 3, "the layouts of the three walls whose faces are measured: {:?}", node.parents);
}

#[test]
fn the_layout_is_deterministic_and_the_metrics_are_canonical() {
    let snapshot = room();
    let first = ModelInference::infer(&snapshot).expect("infers");
    let second = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(first.annotations, second.annotations);
    let table = metrics_json(&first.annotations);
    assert!(table.contains("\"dim-south\"") && table.contains("annotation.dimension-lock-violated|dim-lock-broken") && table.contains("\"tag-name\":\"South\""), "{table}");
    assert_eq!(table, metrics_json(&second.annotations));
}
