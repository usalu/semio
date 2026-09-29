use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, SemioDrawingSnapshot};

fn plan(text: &str) -> LayoutSnapshot {
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    let content = SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Text { value: text.into(), at: point(0.0, 0.0), style: None }] },
        }],
    };
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &content));
    document
}

#[test]
fn set_drawing_text_replaces_the_label_and_inverse_restores_it() {
    let base = plan("Plan");
    let mutation = LayoutMutation::SetDrawingText(SetDrawingText { index: 0, text: "Title".into() });
    let next = mutation.diff(&base).diff().apply(&base).expect("rename");
    assert_eq!(first_drawing_text(&next).as_deref(), Some("Title"));
    let mut engine = crate::editor::layout::engine::scene::LayoutEngine::new();
    let list = crate::editor::layout::engine::scene::build_display_list_for_page(&mut engine, &next, &next.pages[0], "", &[], None, false);
    assert!(list.text_runs.iter().any(|run| run.content == "Title"));
    let restored = mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(first_drawing_text(&restored).as_deref(), Some("Plan"));
}

#[test]
fn set_drawing_text_replaces_a_later_label_and_leaves_the_first() {
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    let content = SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Text { value: "Plan".into(), at: point(0.0, 0.0), style: None },
                    DrawNode::Text { value: "Note".into(), at: point(1.0, 0.0), style: None },
                ],
            },
        }],
    };
    let mut base = crate::standards::v1::subsets::any::schema::default_document();
    base.background_drawing = Some(crate::background_drawing_child_handle("dwg", &content));
    let mutation = LayoutMutation::SetDrawingText(SetDrawingText { index: 1, text: "Caption".into() });
    let next = mutation.diff(&base).diff().apply(&base).expect("rename");
    assert_eq!(drawing_labels(&next), vec!["Plan".to_string(), "Caption".to_string()]);
    let mut engine = crate::editor::layout::engine::scene::LayoutEngine::new();
    let list = crate::editor::layout::engine::scene::build_display_list_for_page(&mut engine, &next, &next.pages[0], "", &[], None, false);
    let contents: Vec<_> = list.text_runs.iter().map(|run| run.content.as_str()).collect();
    assert!(contents.contains(&"Plan") && contents.contains(&"Caption"), "{contents:?}");
    let restored = mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(drawing_labels(&restored), vec!["Plan".to_string(), "Note".to_string()]);
}
