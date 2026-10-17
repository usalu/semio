use super::*;
use semio_framework_io_ports::testkit::{bind, scene_content};
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};
use semio_framework_io_ports::VectorPoint2;
use semio_framework_io_ports::VectorTransform;
use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene};

fn plan(text: &str) -> LayoutSnapshot {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group { transform: VectorTransform::identity(), children: vec![VectorNode::Text { value: text.into(), at: point(0.0, 0.0), style: None }] },
        }],
    };
    let mut document = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    document.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    document
}

#[test]
fn set_drawing_text_replaces_the_label_and_inverse_restores_it() {
    let base = plan("Plan");
    let mutation = LayoutMutation::SetDrawingText(SetDrawingText { index: 0, text: "Title".into() });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("rename");
    assert_eq!(first_drawing_text(&next).as_deref(), Some("Title"));
    let mut engine = semio_s_artifact_layout_layout_engine::scene::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::build_display_list_for_page(&mut engine, &next, &next.pages[0], "", &[], None, false);
    assert!(list.text_runs.iter().any(|run| run.content == "Title"));
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(first_drawing_text(&restored).as_deref(), Some("Plan"));
}

#[test]
fn set_drawing_text_replaces_a_later_label_and_leaves_the_first() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![
                    VectorNode::Text { value: "Plan".into(), at: point(0.0, 0.0), style: None },
                    VectorNode::Text { value: "Note".into(), at: point(1.0, 0.0), style: None },
                ],
            },
        }],
    };
    let mut base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    base.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let mutation = LayoutMutation::SetDrawingText(SetDrawingText { index: 1, text: "Caption".into() });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("rename");
    assert_eq!(drawing_labels(&next), vec!["Plan".to_string(), "Caption".to_string()]);
    let mut engine = semio_s_artifact_layout_layout_engine::scene::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::build_display_list_for_page(&mut engine, &next, &next.pages[0], "", &[], None, false);
    let contents: Vec<_> = list.text_runs.iter().map(|run| run.content.as_str()).collect();
    assert!(contents.contains(&"Plan") && contents.contains(&"Caption"), "{contents:?}");
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(drawing_labels(&restored), vec!["Plan".to_string(), "Note".to_string()]);
}
