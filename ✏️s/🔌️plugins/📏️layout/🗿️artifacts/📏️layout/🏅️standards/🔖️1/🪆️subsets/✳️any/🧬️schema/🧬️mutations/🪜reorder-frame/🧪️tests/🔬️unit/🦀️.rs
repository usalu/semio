use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn reorder_frame_brings_a_frame_forward_and_inverse_sends_it_back() {
    let mut base = crate::standards::v1::subsets::any::schema::default_document();
    let mut extra = base.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap().clone();
    let crate::Frame::Rect { id, .. } = &mut extra else { panic!("rect") };
    *id = "frame-2".into();
    let pos = base.pages[0].frames.iter().position(|frame| frame.id() == "frame-1").unwrap();
    base.pages[0].frames.insert(pos, extra);
    let ids: Vec<String> = base.pages[0].frames.iter().map(|frame| frame.id().to_string()).collect();
    let mutation = LayoutMutation::ReorderFrame(ReorderFrame { page_id: "page-1".into(), frame_id: "frame-2".into(), forward: true });
    let next = mutation.diff(&base).diff().apply(&base).expect("forward");
    let next_ids: Vec<String> = next.pages[0].frames.iter().map(|frame| frame.id().to_string()).collect();
    assert_eq!(next_ids[pos], "frame-1");
    assert_eq!(next_ids[pos + 1], "frame-2");
    let mut engine = crate::editor::layout::engine::scene::LayoutEngine::new();
    let list = crate::editor::layout::engine::scene::build_display_list_for_page(&mut engine, &next, &next.pages[0], "", &[], None, false);
    let painted: Vec<&str> = list.rects.iter().map(|rect| rect.object_id.as_str()).filter(|id| *id == "frame-1" || *id == "frame-2").collect();
    assert_eq!(painted, vec!["frame-1", "frame-2"]);
    let restored = mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse");
    let restored_ids: Vec<String> = restored.pages[0].frames.iter().map(|frame| frame.id().to_string()).collect();
    assert_eq!(restored_ids, ids);
}

#[test]
fn a_rect_brought_to_the_front_paints_after_the_image() {
    let mut base = crate::standards::v1::subsets::any::schema::default_document();
    for _ in 0..8 {
        let mutation = LayoutMutation::ReorderFrame(ReorderFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), forward: true });
        let outcome = mutation.diff(&base);
        if outcome.diff().pages.is_none() {
            break;
        }
        base = outcome.diff().apply(&base).expect("forward");
    }
    assert_eq!(base.pages[0].frames.last().unwrap().id(), "frame-1");
    let mut engine = crate::editor::layout::engine::scene::LayoutEngine::new();
    let list = crate::editor::layout::engine::scene::build_display_list_for_page(&mut engine, &base, &base.pages[0], "", &[], None, false);
    let order = crate::editor::layout::engine::scene::paint_order_ids(&list);
    let image = order.iter().position(|id| id == "frame-image-1").expect("image");
    let rect = order.iter().position(|id| id == "frame-1").expect("rect");
    assert!(rect > image, "{order:?}");
}

#[test]
fn reorder_frame_refuses_the_front_of_the_stack() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let front = base.pages[0].frames.last().unwrap().id().to_string();
    let mutation = LayoutMutation::ReorderFrame(ReorderFrame { page_id: "page-1".into(), frame_id: front, forward: true });
    assert!(mutation.diff(&base).diff().pages.is_none());
}
