# UI Feature Test Full Original Inputs

These exact authored inputs precede the feature-owner split. Each original test function is retained byte-identically once; the parent helper/fixture source is unchanged.

## 🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs

SHA-256: 4ad5788bc4c6d1a4efca3a2a621e8d46f76eb6cf1945fd3e3f7f5d99f9f686b0

```rust
use super::*;

#[test]
fn closing_input_reports_each_admitted_action_receipt_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️correlated-action-receipts/🔣️.json")).unwrap();
    let rows = fixture["receipts"].as_array().unwrap();
    let mut input = InputState::<()>::default();
    let mut batch = input.reserve_actions(rows.len(), rows.len() * 128).unwrap();
    for row in rows {
        batch
            .action(fixture["controller"].as_str().unwrap(), row["action"].as_str().unwrap(), 128, |builder| {
                builder.set_receipt(crate::wgpu::ActionQueueReceipt {
                    token: std::num::NonZeroU64::new(row["token"].as_u64().unwrap()).unwrap(),
                    member: row["member"].as_u64().unwrap() as u8,
                    abort_correlation_on_error: row["abort"].as_bool().unwrap(),
                })
            })
            .unwrap();
    }
    batch.publish().unwrap();
    let mut cancelled = Vec::new();
    for _ in 0..64 {
        if input.close_step_with_receipt(|receipt| cancelled.push(format!("{}:{}", receipt.token, receipt.member))).unwrap() {
            break;
        }
    }
    assert!(input.terminal_is_empty());
    assert_eq!(serde_json::json!(cancelled), fixture["closed"]);
    assert!(input.close_step_with_receipt(|_| panic!("receipt settled twice")).unwrap());
}

#[test]
fn a_reopened_text_owner_drains_the_prior_projection_before_projecting_its_current_value() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️text-owner-lifecycle/🔣️.json")).expect("text owner lifecycle fixture");
    assert_eq!(fixture["sequence"], serde_json::json!(["focus", "advance-to-projection", "blur", "refocus", "drain"]));
    let owner = fixture["owner"].as_str().expect("owner").to_string();
    let value = fixture["value"].as_str().expect("value").to_string();
    let mut input = InputState::<()>::default();
    input.focus_input_owned(owner.clone(), value.clone());
    for _ in 0..256 {
        input.drive_text_step().expect("initial text step");
        if input.text_projection_pending {
            break;
        }
    }
    assert!(input.text_projection_pending, "the first owner has checked out a real bounded projection");
    input.blur_input();
    input.focus_input_owned(owner.clone(), value.clone());
    for turn in 0..2048 {
        let pending = input.drive_text_step().expect("close/reopen must not manufacture a protocol fault");
        if !pending && input.text_buffer.reserved_bytes() == 0 && !input.text_projection_pending {
            assert!(turn > 0, "the checked-out projection is drained incrementally");
            break;
        }
        assert!(turn < 2047, "the current owner settles under a fixed bound");
    }
    assert_eq!(input.focused_id.as_deref(), fixture["expected"]["owner"].as_str());
    assert_eq!(input.text_view(), fixture["expected"]["value"].as_str().expect("expected value"));

    let mut authority = ui_contract::TextEditAuthority::default();
    authority.start_projection(0, 1).expect("first projection");
    assert_eq!(authority.start_projection(0, 1), Err(ui_contract::TextEditFault::Protocol), "a genuine concurrent projection remains a precise protocol refusal");
}

#[test]
fn an_up_flow_tree_section_registers_its_header_at_the_painted_bottom_edge() {
    let section = Rect::new(100.0, 200.0, 300.0, 240.0);
    assert_eq!(retained_tree_section_header_band(section, 24.0, true), Rect::new(100.0, 416.0, 300.0, 24.0));
    assert_eq!(retained_tree_section_header_band(section, 24.0, false), Rect::new(100.0, 200.0, 300.0, 24.0));
}

#[test]
fn a_childful_up_flow_tree_item_registers_label_and_gutter_on_its_painted_bottom_row() {
    use crate::wgpu::chrome::UiDriverDrag;
    use crate::wgpu::component::ui::{UiNode, UiPresence, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
    use crate::wgpu::layout::TreeRowMetrics;
    use crate::wgpu::tree::{Node, NodeKey, UiTree, WidgetSpec};
    use crate::wgpu::Label;

    let child = UiTreeItemNode::base("child", Label::data("Child"));
    let mut branch = UiTreeItemNode::base("branch", Label::data("Branch"));
    branch.items = Some(vec![child]);
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "section".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![branch] };
    let mut tree = UiTree::new();
    let owner = tree.insert_child(
        None,
        Node::new(NodeKey::Explicit("tree".into()), WidgetSpec(UiNode::Tree(UiTreeNode { presentation: Default::default(), sections: vec![section], presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None }))),
    );
    let row =
        |id: &str| UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
    let section_row = tree.insert_child(Some(owner), Node::new(NodeKey::Explicit("section".into()), WidgetSpec(row("section"))));
    let branch_row = tree.insert_child(Some(section_row), Node::new(NodeKey::Explicit("branch".into()), WidgetSpec(row("branch"))));
    let _child_row = tree.insert_child(Some(branch_row), Node::new(NodeKey::Explicit("child".into()), WidgetSpec(row("child"))));
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let subtree = Rect::new(100.0, 200.0, 300.0, metrics.row_height * 2.0);

    let label = retained_hit_registration(&tree, branch_row, subtree, &metrics, UiDriverDrag::Handle, true).expect("branch label");
    let gutter = retained_tree_chevron_registration(&tree, branch_row, subtree, &metrics, true).expect("branch gutter");
    assert_eq!(label.control_id, "tree.label.branch");
    assert_eq!(gutter.control_id, "tree.chevron.branch");
    assert_eq!(label.rect.y, subtree.y + metrics.row_height);
    assert_eq!(gutter.rect.y, label.rect.y);
    assert_eq!(gutter.rect.h, metrics.row_height);
    assert!(gutter.rect.w > 0.0 && gutter.rect.w < label.rect.w);
}

#[test]
fn hit_at_prefers_content_registered_after_scroll_region() {
    let mut input = InputState::<()>::default();
    let scroll = Rect::new(0.0, 0.0, 200.0, 200.0);
    let row = Rect::new(0.0, 24.0, 200.0, 24.0);
    input.register_hit(HitTarget { rect: scroll, event: None, control_id: Some("scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    input.register_hit(HitTarget { rect: row, event: None, control_id: Some("tree.label.item-1".into()), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
    assert!(input.hit_at(10.0, 36.0).is_none(), "a registry the frame build has not published yet resolves nothing");
    input.publish_hits();
    let hit = input.hit_at(10.0, 36.0).expect("row point should hit");
    assert_eq!(hit.control_id.as_deref(), Some("tree.label.item-1"));
    assert_eq!(hit.kind, HitKind::TreeItem);
}

#[test]
fn event_queue_has_fixed_credits_and_transfers_one_fifo_item() {
    let mut input = InputState::<ActionDescriptor>::default();
    for index in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        let action = format!("event-{index}");
        input.reserve_action("controller", &action, 128).expect("reservation").publish().expect("queue credit");
    }
    assert_eq!(input.pending_actions.len(), crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY);
    for expected in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        assert_eq!(input.take_action_step().expect("authority").expect("event").into_descriptor().expect("materialized").action, format!("event-{expected}"));
    }
    assert!(input.take_action_step().expect("authority").is_none());
}

#[test]
fn event_queue_close_retires_one_owned_slot_per_step() {
    let mut input = InputState::<ActionDescriptor>::default();
    input.reserve_action("controller", "first", 128).expect("first").publish().expect("first queue");
    input.reserve_action("controller", "second", 128).expect("second").publish().expect("second queue");
    assert!(!input.close_step().expect("first retirement step"));
    assert_eq!(input.pending_actions.len(), 1);
    assert!(!input.close_step().expect("second retirement step"));
    assert!(input.pending_actions.is_empty());
}

#[test]
fn saturated_reservation_does_not_consume_semantic_source_and_retry_keeps_fifo() {
    let mut input = InputState::<ActionDescriptor>::default();
    for index in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        let action = format!("event-{index}");
        input.reserve_action("controller", &action, 128).expect("reservation").publish().expect("publish");
    }
    let semantic_source = String::from("retry-owned");
    let result = input.publish_action("controller", "retry", 128, |builder, _| {
        builder.begin_object(None)?;
        builder.string(Some("value"), &semantic_source)?;
        builder.end_container()
    });
    assert_eq!(result, Err(BoundedActionFault::ItemCredits));
    assert_eq!(semantic_source, "retry-owned");
    assert_eq!(input.take_action_step().expect("authority").expect("first").into_descriptor().expect("descriptor").action, "event-0");
    input
        .publish_action("controller", "retry", 128, |builder, _| {
            builder.begin_object(None)?;
            builder.string(Some("value"), &semantic_source)?;
            builder.end_container()
        })
        .expect("retry publication");
    let mut last = None;
    while let Some(action) = input.take_action_step().expect("authority") {
        last = Some(action.into_descriptor().expect("descriptor"));
    }
    assert_eq!(last.expect("last").action, "retry");
}

#[test]
fn action_fault_is_observed_before_another_queued_owner() {
    let mut input = InputState::<ActionDescriptor>::default();
    input.reserve_action("controller", "queued", 128).expect("reservation").publish().expect("publish");
    input.record_action_fault(BoundedActionFault::ByteCredits);
    assert!(matches!(input.take_action_step(), Err(BoundedActionFault::ByteCredits)));
    assert_eq!(input.take_action_step().expect("authority").expect("queued").into_descriptor().expect("descriptor").action, "queued");
}

#[test]
fn batch_length_step_observes_fault_then_preserves_the_complete_source_slice() {
    let mut input = InputState::<ActionDescriptor>::default();
    let mut batch = input.reserve_actions(2, 64).unwrap();
    batch.action("controller", "first", 32, |_| Ok(())).unwrap();
    batch.action("controller", "second", 32, |_| Ok(())).unwrap();
    batch.publish().unwrap();
    input.record_action_fault(BoundedActionFault::ByteCredits);
    assert_eq!(input.take_action_batch_len_step(), Err(BoundedActionFault::ByteCredits));
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(2)));
    assert_eq!(input.take_action_step().unwrap().unwrap().into_descriptor().unwrap().action, "first");
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(1)));
}

#[test]
fn shared_tree_drag_fixture_projects_driver_specific_row_and_handle_targets() {
    use crate::wgpu::chrome::UiDriverDrag;
    use crate::wgpu::component::ui::{UiNode, UiPresence, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
    use crate::wgpu::layout::TreeRowMetrics;
    use crate::wgpu::tree::{Node, NodeKey, UiTree, WidgetSpec};
    use crate::wgpu::Label;
    use std::collections::HashMap;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌳️tree-drag-handles/🔣️.json")).expect("tree drag fixture");
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let row_width = fixture["rowWidth"].as_f64().expect("row width") as f32;
    for row in fixture["rows"].as_array().expect("rows") {
        let id = row["id"].as_str().expect("row id");
        let drag_data = row["dragData"].as_object().map(|map| map.iter().map(|(key, value)| (key.clone(), value.as_str().expect("payload").to_string())).collect::<HashMap<_, _>>());
        let mut item = UiTreeItemNode::base(id, Label::data(row["label"].as_str().expect("label")));
        item.draggable = row["draggable"].as_bool();
        item.drag_data = drag_data;
        let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "fixture".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };
        let mut tree = UiTree::new();
        let owner = tree.insert_child(
            None,
            Node::new(NodeKey::Explicit("tree".into()), WidgetSpec(UiNode::Tree(UiTreeNode { presentation: Default::default(), sections: vec![section], presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None }))),
        );
        let stack = UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None };
        let row_node = tree.insert_child(Some(owner), Node::new(NodeKey::Explicit(id.into()), WidgetSpec(UiNode::Stack(stack))));
        let rect = Rect::new(0.0, 0.0, row_width, metrics.row_height);

        for driver in fixture["drivers"].as_array().expect("drivers") {
            let mode = if driver["drag"].as_str() == Some("surface") { UiDriverDrag::Surface } else { UiDriverDrag::Handle };
            let row_hit = retained_hit_registration(&tree, row_node, rect, &metrics, mode, false).expect("row hit");
            let handle_hit = retained_tree_drag_handle_registration(&tree, row_node, rect, &metrics, mode);
            let visible = driver["visibleHandles"].as_array().expect("visible handles").iter().any(|value| value.as_str() == Some(id));
            let label_starts = driver["labelStarts"].as_array().expect("label starts").iter().any(|value| value.as_str() == Some(id));
            let handle_starts = driver["handleStarts"].as_array().expect("handle starts").iter().any(|value| value.as_str() == Some(id));
            assert_eq!(row_hit.drag_data.is_some(), label_starts, "{id}: row initiation under {:?}", mode);
            assert_eq!(handle_hit.is_some(), visible, "{id}: handle visibility under {:?}", mode);
            assert_eq!(handle_hit.as_ref().is_some_and(|hit| hit.drag_data.is_some()), handle_starts, "{id}: handle initiation under {:?}", mode);
            if let (Some(hit), Some(role)) = (handle_hit, row["role"].as_str()) {
                assert_eq!(hit.kind, HitKind::TreeDragHandle);
                assert_eq!(hit.control_id, format!("tree.drag.{role}.{id}"));
                assert!(rect.contains(hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5));
            }
        }
    }
}

//#region 🔢️InputConstraints
// 🔢️ LAW: `InputProps`' `min`/`max`/`step` are enforced, not decoration. React hands them to a real
// `<input type="number" min max step>` (`🗣️Interpreter/🟦️.tsx`'s `InputView`) and the browser refuses
// an out-of-range commit for it; an immediate-mode canvas has no browser, so this target enforces
// them itself — at the one commit authority, and in the `InputMeta` the chrome's own commit reads.

#[test]
fn number_constraints_clamp_and_snap_exactly_once_each() {
    use crate::wgpu::events::constrain_number_input;
    assert_eq!(constrain_number_input(999.0, Some(0.0), Some(10.0), None), 10.0);
    assert_eq!(constrain_number_input(-4.0, Some(2.0), Some(10.0), None), 2.0);
    assert_eq!(constrain_number_input(3.3, None, None, Some(0.5)), 3.5, "snapping is to the nearest step, not a floor");
    assert_eq!(constrain_number_input(3.4, Some(1.0), None, Some(2.0)), 3.0, "the step ladder starts at `min`, not at zero");
    assert_eq!(constrain_number_input(7.0, None, None, Some(0.0)), 7.0, "a zero step is no step, never a division");
    assert_eq!(constrain_number_input(7.25, None, None, None), 7.25);
    assert!(constrain_number_input(f64::NAN, Some(0.0), Some(10.0), Some(1.0)).is_nan(), "an unparseable buffer stays a refusal — never an invented in-range number");
}

#[test]
fn an_input_metas_commit_value_carries_its_own_constraints() {
    let text = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "text".into(), min: None, max: None, step: None, accept: None };
    assert_eq!(text.commit_value("12abc"), Some(semio_framework_value::DslValue::String("12abc".into())), "a text field commits its text verbatim");

    let number = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "number".into(), min: Some(0.0), max: Some(10.0), step: Some(0.5), accept: None };
    assert_eq!(number.commit_value("99"), Some(semio_framework_value::DslValue::float(10.0)));
    assert_eq!(number.commit_value("3.3"), Some(semio_framework_value::DslValue::float(3.5)));
    assert_eq!(number.commit_value(""), None, "an empty number buffer commits nothing at all");

    let file = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "file".into(), min: None, max: None, step: None, accept: Some("image/*".into()) };
    assert_eq!(file.accept.as_deref(), Some("image/*"), "a file field's `accept` reaches the host picker instead of being dropped at the render call site");
}
//#endregion 🔢️InputConstraints

```

## 🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs

SHA-256: f2dbd14ff5e37f2b2c346b7c9f2d3e359d3e1cd99dcf034a603de6672afbb6a4

```rust
use super::*;
use semio_framework_job::{drive_step, root_cancel_token, Generation, InteractiveStage, OperationId, StepBudget};

#[test]
fn prepared_animation_receipt_measures_actual_normal_and_overlay_primitives() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🖌️render/⏱️schedule/🧫️fixtures/🎞️animation/🔣️.json")).unwrap();
    for overlay_route in [false, true] {
        for row in fixture["primitives"].as_array().unwrap() {
            let mut draw = DrawList::default();
            let mut overlay = DrawList::default();
            for (list, field) in [(&mut draw, "draw"), (&mut overlay, "overlay")] {
                for kind in row[field].as_array().unwrap() {
                    let mut instance = crate::wgpu::draw_types::UiInstance::solid([0.0, 0.0, 24.0, 24.0], crate::wgpu::theme::Rgba::new(1.0, 1.0, 1.0, 1.0));
                    instance.params[2] = kind.as_f64().unwrap() as f32;
                    if overlay_route {
                        list.layers[0].overlay_ui_instances.push(instance);
                    } else {
                        list.layers[0].ui_instances.push(instance);
                    }
                }
            }
            let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, Some(overlay), 0.0), 1);
            assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
            let mut packet = job.take_packet().expect("measured packet");
            assert_eq!(packet.has_animated_primitives(), row["active"].as_bool().unwrap(), "{}", row["id"]);
            while !packet.retire_step() {}
            while !job.close_step() {}
        }
    }
}

static PREPARED_PROCESS_TEST_LOCK: Mutex<()> = Mutex::new(());

/// 🔒️ EVERY law in this case takes this guard, first statement, no exceptions.
///
/// The prepared ladder's credits are PROCESS-wide by design — `PREPARED_RENDER_PROCESS_PERMITS`, the
/// atlas page pool and the four abandonment rings `drain_abandoned_preparations` drains are one
/// budget for the whole binary, because that is the budget a real process has. A law that reads a
/// process counter (`PREPARED_RENDER_PROCESS_PERMITS.load(..) == 0`) or counts retirement grants
/// therefore measures every other law that is running at the same time, and under `cargo test` with
/// more than one test thread it flakes: `packet_drop_retires_nested_backings_and_permit_scalars_separately`
/// and `pending_presenter_witness_rejects_superseding_packet_with_exact_owner` were the two that
/// showed it (`📓️w9a`), but only eleven of this case's thirty-nine laws took the lock, so ANY of the
/// other twenty-eight could have been the perturber. Serialising the whole case is what makes the
/// measurements mean what they say, and it costs nothing: none of these laws blocks.
///
/// Holding the lock is only half of it. An owner a law drops lands in an ABANDONMENT ring and keeps
/// its permits until something grants it a close step, so a law that ends with owners still queued
/// hands the next one an already-spent budget (`prepared render process permits exhausted (held items
/// 4/64 …)`). The guard therefore drains both rings before it hands the lock back: every law starts
/// from a quiescent process, whatever the law before it left behind and whatever order the runner
/// picked.
fn prepared_process_guard() -> std::sync::MutexGuard<'static, ()> {
    let guard = match PREPARED_PROCESS_TEST_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    drain_abandoned_preparations();
    drain_abandoned_atlases();
    guard
}

fn drain_abandoned_atlases() {
    while !PreparedAtlasPages::close_abandoned_step() {}
}

fn drain_abandoned_preparations() {
    loop {
        let inputs = PreparedRenderInput::close_abandoned_step();
        let jobs = PreparedRenderJob::close_abandoned_step();
        let mailboxes = PreparedRenderReceiver::close_abandoned_step();
        let packets = PreparedRenderPacket::close_abandoned_step();
        if inputs && jobs && mailboxes && packets {
            break;
        }
    }
}

fn now_ms() -> Option<u64> {
    Some(1)
}

fn drive_preparation_until_terminal(job: &mut PreparedRenderJob) -> StepOutcome {
    let mut preview = 0;
    for _ in 0..4_096 {
        let outcome = drive_step(job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
        if !matches!(outcome, StepOutcome::Yield) {
            return outcome;
        }
    }
    panic!("prepared render job did not reach a terminal outcome within fixed test credits");
}

fn packet(revision: u64, generation: u64) -> PreparedRenderPacket {
    let mut directives = PreparedRenderDirectives::default();
    assert!(directives.try_push(RenderDirective::PreservePreviousOnFailure).is_ok());
    let packet = PreparedRenderPacket {
        has_animated_primitives: false,
        scene_revision: revision,
        preview_generation: generation,
        damage: PreparedRenderScissors::default(),
        clips: PreparedRenderScissors::default(),
        directives,
        uploads: PreparedRenderUploads::default(),
        evictions: PreparedRenderEvictions::default(),
        draw: DrawList::default(),
        overlay: None,
        commands: PreparedRenderCommandPages::default(),
        time_seconds: 0.0,
        usage: PreparedRenderUsage::default(),
        limits: PreparedRenderLimits::default(),
        permit: PreparedRenderProcessPermit::try_reserve(0, 0),
        retirement_phase: 0,
        abandonment_slot: u8::MAX,
    };
    match packet.try_arm_abandonment() {
        Ok(packet) => packet,
        Err(_) => panic!("test packet abandonment slot"),
    }
}

fn retire_raster_upload(mut upload: PreparedRenderUpload) {
    let PreparedRenderUpload::RasterPages { key, pixels } = &mut upload else { panic!("paged raster upload") };
    while !pixels.retire_with_key_step(key) {}
    assert!(pixels.terminal_is_empty());
}

#[test]
fn paged_raster_producer_advances_one_page_and_moves_page_identity() {
    let _guard = prepared_process_guard();
    let source = vec![7; PREPARED_RASTER_PAGE_BYTES * 2];
    let source_pointer = source.as_ptr();
    let (mut producer, published_key) = PreparedRasterProducer::try_admit("two-pages".into(), source, 4_096, 2).expect("exact two-page admission");
    assert_eq!(published_key, "two-pages");
    assert!(producer.bind_frame_generation(9));
    assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending));
    assert_eq!(producer.pages.as_ref().expect("retained pages").slots.len(), 1);
    assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending));
    assert_eq!(producer.pages.as_ref().expect("retained pages").slots.len(), 2);
    assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending), "source backing retires on its own grant");
    let first = producer.pages.as_ref().and_then(|pages| pages.page_pointer(0)).expect("first page identity");
    let upload = match producer.step(9) {
        PreparedRasterProducerStep::Complete(upload) => upload,
        _ => panic!("completed page handoff"),
    };
    assert_eq!(first, source_pointer, "page view borrows the exact decoder backing");
    assert!(matches!(&upload, PreparedRenderUpload::RasterPages { pixels, .. } if pixels.page_pointer(0) == Some(first) && pixels.frame_generation() == 9));
    retire_raster_upload(upload);
}

#[test]
fn paged_raster_content_identity_is_stable_and_changes_with_one_pixel_byte() {
    fn complete(mut producer: PreparedRasterProducer, generation: u64) -> PreparedRenderUpload {
        assert!(producer.bind_frame_generation(generation));
        for _ in 0..8 {
            if let PreparedRasterProducerStep::Complete(upload) = producer.step(generation) {
                return upload;
            }
        }
        panic!("bounded two-page producer did not complete");
    }

    let _guard = prepared_process_guard();
    let first = vec![7; PREPARED_RASTER_PAGE_BYTES * 2];
    let same = first.clone();
    let mut changed = first.clone();
    *changed.last_mut().expect("changed byte") = 8;
    let (first, _) = PreparedRasterProducer::try_admit("first".into(), first, 4_096, 2).expect("first producer");
    let (same, _) = PreparedRasterProducer::try_admit("same".into(), same, 4_096, 2).expect("same producer");
    let (changed, _) = PreparedRasterProducer::try_admit("changed".into(), changed, 4_096, 2).expect("changed producer");
    let first = complete(first, 31);
    let same = complete(same, 32);
    let changed = complete(changed, 33);
    let PreparedRenderUpload::RasterPages { pixels: first_pixels, .. } = &first else { panic!("first pages") };
    let PreparedRenderUpload::RasterPages { pixels: same_pixels, .. } = &same else { panic!("same pages") };
    let PreparedRenderUpload::RasterPages { pixels: changed_pixels, .. } = &changed else { panic!("changed pages") };
    assert_eq!(first_pixels.content_identity(), same_pixels.content_identity());
    assert_ne!(first_pixels.content_identity(), changed_pixels.content_identity());
    retire_raster_upload(first);
    retire_raster_upload(same);
    retire_raster_upload(changed);
}

#[test]
fn stale_generation_does_not_consume_a_prepared_raster_page() {
    let _guard = prepared_process_guard();
    let (mut producer, _) = PreparedRasterProducer::try_admit("stale".into(), vec![3; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page admission");
    assert!(producer.bind_frame_generation(11));
    let retained = producer.source.len();
    assert!(matches!(producer.step(12), PreparedRasterProducerStep::Fault(_)));
    assert_eq!(producer.source.len(), retained);
    assert!(producer.pages.as_ref().expect("retained pages").slots.is_empty());
    producer.begin_close();
    while !producer.close_step() {}
    assert!(producer.terminal_is_empty());
}

#[test]
fn raster_cap_plus_one_rejects_the_exact_source_before_page_allocation() {
    let _guard = prepared_process_guard();
    let source = vec![5; PREPARED_RASTER_PAGE_BYTES + 4];
    let pointer = source.as_ptr();
    let mut rejected = PreparedRasterProducer::try_admit("wide".into(), source, 4_097, 1).expect_err("row cap plus one");
    assert_eq!(rejected.source.as_ptr(), pointer);
    assert!(rejected.fault().contains("fixed item or byte credits"));
    assert!(!rejected.close_step(), "one rejection grant retires one source page only");
    while !rejected.close_step() {}
}

#[test]
fn atlas_page_cap_plus_one_faults_before_process_credit_transfer() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let height = 33_554_433_u32;
    let byte_len = 33_554_433_usize;
    assert!(matches!(PreparedAtlasPages::try_new(1, height, 1, byte_len), Err("atlas page or byte credits exceeded")));
    let mut admitted = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
        Ok(admitted) => admitted,
        Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
    };
    let mut turns = 0;
    while !admitted.close_step() {
        turns += 1;
    }
    assert!(turns >= 6, "fixed backing and four permit scalars close independently");
    assert!(admitted.terminal_is_empty());
}

#[test]
fn atlas_close_releases_one_fixed_page_then_its_exact_credit() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let source = [9_u8; 32];
    let mut pages = match PreparedAtlasPages::try_new(4, 2, 4, source.len()) {
        Ok(pages) => pages,
        Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
    };
    assert!(matches!(pages.push_page(&source, 0), Ok(true)));
    assert_eq!(pages.page(0), Some((&source[..], 0, 2)));
    assert!(!pages.close_step());
    assert_eq!(pages.len(), 0);
    assert!(!pages.terminal_is_empty());
    let mut turns = 0;
    while !pages.close_step() {
        turns += 1;
    }
    assert!(turns >= 6, "slot backing and permit fields each consume a distinct grant");
    assert!(pages.terminal_is_empty());
}

#[test]
fn atlas_process_item_max_plus_one_is_nonblocking_and_recovers_every_permit() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let mut owners: [Option<PreparedAtlasPages>; PREPARED_ATLAS_PROCESS_ITEMS] = std::array::from_fn(|_| None);
    for owner in &mut owners {
        *owner = Some(match PreparedAtlasPages::try_new(1, 1, 1, 1) {
            Ok(owner) => owner,
            Err(fault) => panic!("one exact process item: {fault}"),
        });
    }
    assert!(matches!(PreparedAtlasPages::try_new(1, 1, 1, 1), Err("atlas process permit credits exhausted")));
    for owner in owners.iter_mut().filter_map(Option::as_mut) {
        while !owner.close_step() {}
        assert!(owner.terminal_is_empty());
    }
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn abandoned_atlas_schedules_the_same_incremental_close_authority() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let mut owner = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
        Ok(owner) => owner,
        Err(fault) => panic!("abandonment owner: {fault}"),
    };
    assert!(owner.push_page(&[7; 32], 0).is_ok());
    drop(owner);
    let mut turns = 0;
    while !PreparedAtlasPages::close_abandoned_step() {
        turns += 1;
        assert!(turns < 16, "one page, backing owner, and four permit fields must converge");
    }
    assert!(turns >= 7);
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn interrupted_atlas_close_rejoins_the_same_abandonment_authority() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let source = vec![3_u8; PREPARED_ATLAS_PAGE_BYTES + 1];
    let mut owner = match PreparedAtlasPages::try_new(1, u32::try_from(source.len()).unwrap_or(u32::MAX), 1, source.len()) {
        Ok(owner) => owner,
        Err(fault) => panic!("two-page abandonment owner: {fault}"),
    };
    assert!(matches!(owner.push_page(&source, 0), Ok(false)));
    assert!(matches!(owner.push_page(&source, owner.next_row()), Ok(true)));
    assert!(!owner.close_step());
    assert_eq!(owner.len(), 1);
    drop(owner);
    let mut turns = 0;
    while !PreparedAtlasPages::close_abandoned_step() {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(turns >= 7);
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn atlas_allocation_refusal_preserves_the_packed_permit_ledger() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let before = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
    assert!(matches!(PreparedAtlasPages::try_new(0, 1, 4, 4), Err("atlas dimensions do not fit fixed page credits")));
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), before);
}

#[test]
fn atlas_contended_permit_attempts_are_nonblocking_and_poison_free() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let handles = std::array::from_fn::<_, 8, _>(|_| {
        std::thread::spawn(|| match PreparedAtlasPages::try_new(1, 1, 1, 1) {
            Ok(mut owner) => {
                while !owner.close_step() {}
                owner.terminal_is_empty()
            }
            Err("atlas process permit credits exhausted") => true,
            Err(_) => false,
        })
    });
    for handle in handles {
        assert!(match handle.join() {
            Ok(closed_or_refused) => closed_or_refused,
            Err(_) => false,
        });
    }
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn raster_item_bytes_exact_and_plus_one_are_claimed_before_materialization() {
    let _guard = prepared_process_guard();
    let reservation = PreparedRasterReservation::try_reserve("item-exact".into()).expect("initial exact reservation");
    let reservation = reservation.claim(4_096, 1_024).expect("sixteen MiB operation claim");
    let mut rejected = reservation.reject("test retirement", Vec::new());
    while !rejected.close_step() {}
    assert!(rejected.terminal_is_empty());

    let reservation = PreparedRasterReservation::try_reserve("item-plus-one".into()).expect("initial plus-one reservation");
    let mut rejected = reservation.claim(4_096, 1_025).expect_err("sixteen MiB plus one row");
    assert_eq!(rejected.fault(), "raster producer exceeded fixed item or byte credits");
    while !rejected.close_step() {}
    assert!(rejected.terminal_is_empty());
}

#[test]
fn raster_simultaneous_source_decode_peak_exact_and_plus_one() {
    let _guard = prepared_process_guard();
    let height = 1_023usize;
    let decoded_bytes = PREPARED_RASTER_PAGE_BYTES * height;
    let page_slot_bytes = size_of::<PreparedRasterPage>() * height;
    let source_peak_bytes = PREPARED_RASTER_PRODUCER_BYTES - decoded_bytes - page_slot_bytes;
    assert_eq!(source_peak_bytes % 2, 0, "exact source workspace boundary");
    let source_bytes = source_peak_bytes / 2;

    let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes).expect("exact simultaneous source reservation");
    let reservation = reservation.claim(4_096, height as u32).expect("source plus retained parse plus decoded backing and page slots exactly fit");
    assert_eq!(reservation.credit.as_ref().unwrap().bytes, PREPARED_RASTER_PRODUCER_BYTES);
    let mut rejected = reservation.reject("exact peak retirement", Vec::new());
    while !rejected.close_step() {}

    let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes + 1).expect("plus one source is initially retained");
    let mut rejected = reservation.claim(4_096, height as u32).expect_err("simultaneous source and decoded peak plus one must fail before decode");
    assert_eq!(rejected.fault(), "raster producer exact credit resize failed");
    while !rejected.close_step() {}
}

#[test]
fn retained_codec_source_moves_once_and_retires_one_page_per_governed_step() {
    let _guard = prepared_process_guard();
    let decoded = vec![7; PREPARED_RASTER_PAGE_BYTES];
    let retained_source = vec![9; PREPARED_RASTER_PAGE_BYTES * 2];
    let retained_pointer = retained_source.as_ptr();
    let reservation = PreparedRasterReservation::try_reserve_source("retained-source".into(), retained_source.capacity()).expect("source workspace admitted before decode");
    let reservation = reservation.claim(4_096, 1).expect("decoded owner credited in addition to retained source");
    let (mut producer, _) = reservation.finalize(decoded, retained_source, 4_096, 1).expect("exact retained source owner");
    assert_eq!(producer.retained_source.as_ptr(), retained_pointer);
    assert!(producer.bind_frame_generation(3));

    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    for expected in [PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES, 0] {
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(11), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
        assert!(matches!(outcome, StepOutcome::Yield));
        assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.len(), expected);
    }
    assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.as_ptr(), retained_pointer);
    while !job.close_step() {}
    assert!(job.terminal_is_empty());
}

/// 🌑️ LAW: the bounded caster prepass completes before every receiver, then the color pass
/// keeps React's textured underlay → opaque → lines → translucent order.
#[test]
fn an_enabled_shadow_pass_measures_every_caster_before_its_receivers() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::{Instance3d, LineDraw3d, LineVertex3d, SceneCurvilinear3d, SceneDraw3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneShadowRole3d, TexturedDraw3d, TexturedInstance3d};
    let mut draw = DrawList::default();
    let instance = Instance3d { id: String::new(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    draw.push_scene_pass(ScenePass3d {
        viewport: [0.0, 0.0, 100.0, 40.0],
        shadow: crate::wgpu::kernel_3d_scene::SceneShadow3d { enabled: true, ..Default::default() },
        shadow_draws: vec![SceneDraw3d { mesh_key: "caster".into(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: SceneShadowRole3d { casts: true, receives: true } }],
        draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: Default::default() }],
        translucent_draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: Default::default() }],
        material_draws: vec![
            SceneMaterialDraw3d { mesh_key: "painted".into(), mesh_version: 3, first_index: 0, index_count: u32::MAX, instances: vec![instance.clone()], material: SceneMaterialKind3d::Painted { texture_key: "paint-map".into() }, translucent: false },
            SceneMaterialDraw3d {
                mesh_key: "celebrated".into(),
                mesh_version: 4,
                first_index: 0,
                index_count: u32::MAX,
                instances: vec![instance.clone()],
                material: SceneMaterialKind3d::Celebration { stops: [[1.0; 4]; 3], angle: 0.5 },
                translucent: true,
            },
        ],
        line_draws: vec![LineDraw3d { vertices: vec![LineVertex3d { position: [0.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] }] }],
        textured_draws: vec![TexturedDraw3d { instances: vec![TexturedInstance3d { texture_key: String::new(), model: instance.model, background: [0.0; 4], appearance: [0.85, 0.0, 0.0, 0.0] }] }],
        curvilinear: Some(SceneCurvilinear3d { fov_radians: 2.0, strength: 0.75 }),
        ..Default::default()
    });

    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut order = Vec::new();
    for _ in 0..64 {
        let label = match cursor {
            DrawMeasureCursor::PassShadowBegin(..) | DrawMeasureCursor::PassShadowInstance { .. } => "shadow",
            DrawMeasureCursor::PassTextured { .. } | DrawMeasureCursor::PassTexturedInstance { .. } | DrawMeasureCursor::PassTexturedKey { .. } => "textured",
            DrawMeasureCursor::PassDraw { translucent, .. } | DrawMeasureCursor::PassDrawKey { translucent, .. } | DrawMeasureCursor::PassInstance { translucent, .. } | DrawMeasureCursor::PassInstanceKey { translucent, .. } => {
                if translucent {
                    "translucent"
                } else {
                    "opaque"
                }
            }
            DrawMeasureCursor::PassMaterial { translucent, .. }
            | DrawMeasureCursor::PassMaterialMeshKey { translucent, .. }
            | DrawMeasureCursor::PassMaterialTextureKey { translucent, .. }
            | DrawMeasureCursor::PassMaterialInstance { translucent, .. }
            | DrawMeasureCursor::PassMaterialInstanceKey { translucent, .. } => {
                if translucent {
                    "material-translucent"
                } else {
                    "material-opaque"
                }
            }
            DrawMeasureCursor::PassLine { .. } | DrawMeasureCursor::PassLineVertex { .. } => "lines",
            DrawMeasureCursor::PassPostprocess { .. } => "world-postprocess",
            _ => "other",
        };
        if order.last() != Some(&label) && label != "other" {
            order.push(label);
        }
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() || matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    assert_eq!(order, vec!["shadow", "textured", "material-opaque", "opaque", "lines", "translucent", "material-translucent", "world-postprocess"], "the image-space remap runs after every world color receiver");
}

/// 🌐️ LAW: the prepared scalar ladder owns exactly one grid item after textured references
/// and before opaque geometry, while ordinary overlays retain the line-vertex lane.
#[test]
fn a_procedural_grid_is_one_prepared_scalar_between_textures_and_opaque_geometry() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::{Instance3d, LineDraw3d, LineVertex3d, ProceduralGrid3d, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d};
    let instance = Instance3d { id: String::new(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
    let grid = ProceduralGrid3d { plane_z: 0.001, camera_plane_projection: [2.0, -3.0, 0.001], cell_size: 1.0, fade_distance: 8.0, cell_color: [0.2, 0.3, 0.4] };
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d {
        procedural_grid: Some(grid),
        textured_draws: vec![TexturedDraw3d { instances: vec![TexturedInstance3d { texture_key: String::new(), model: instance.model, background: [0.0; 4], appearance: [1.0, 0.0, 0.0, 0.0] }] }],
        draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance], shadow_role: Default::default() }],
        line_draws: vec![LineDraw3d { vertices: vec![LineVertex3d { position: [0.0; 3], color: [1.0; 4] }, LineVertex3d { position: [1.0, 0.0, 0.0], color: [1.0; 4] }] }],
        ..Default::default()
    });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut measured = Vec::new();
    for _ in 0..64 {
        let current = cursor;
        let Some(usage) = PreparedRenderJob::next_draw_usage(&draw, &mut cursor) else { break };
        measured.push((current, usage));
        if matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    let grid_rows: Vec<_> = measured.iter().enumerate().filter(|(_, (cursor, _))| matches!(cursor, DrawMeasureCursor::PassGrid { pass: 0 })).collect();
    assert_eq!(grid_rows.len(), 1, "one retained grid becomes exactly one prepared scalar");
    assert_eq!(grid_rows[0].1 .1, PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<ProceduralGrid3d>(), ..Default::default() }, "the grid scalar owns its exact retained bytes in one cancellable step");
    let textured = measured.iter().position(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassTexturedInstance { .. })).expect("textured reference scalar");
    let grid_index = grid_rows[0].0;
    let opaque = measured.iter().position(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassInstance { translucent: false, .. })).expect("opaque instance scalar");
    let line_vertices = measured.iter().filter(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassLineVertex { .. })).count();
    assert!(textured < grid_index && grid_index < opaque, "textured references precede the grid and opaque geometry follows it");
    assert_eq!(line_vertices, 2, "ordinary overlays retain the line-vertex lane");
}

#[test]
fn ellipse_scene_pass_snapshots_backdrop_clears_inside_then_composites_last() {
    use crate::wgpu::kernel_3d_scene::{ScenePass3d, SceneViewportMask3d};
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d { viewport_mask: SceneViewportMask3d::Ellipse, clear_color: Some([0.1, 0.2, 0.3, 0.7]), ..Default::default() });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut scalars = Vec::new();
    for _ in 0..8 {
        let Some(_) = PreparedRenderJob::next_draw_usage(&draw, &mut cursor) else { break };
        scalars.push(cursor);
        if matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    assert_eq!(scalars[0], DrawMeasureCursor::PassBackdropSnapshot { pass: 0 });
    assert_eq!(scalars[1], DrawMeasureCursor::PassSceneClear { pass: 0 });
    assert!(scalars.contains(&DrawMeasureCursor::PassPostprocess { pass: 0 }));
    assert_eq!(scalars.iter().filter(|cursor| matches!(cursor, DrawMeasureCursor::PassPostprocess { .. })).count(), 1);

    let mut rectangular = DrawList::default();
    rectangular.push_scene_pass(ScenePass3d::default());
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut postprocess = 0;
    while PreparedRenderJob::next_draw_usage(&rectangular, &mut cursor).is_some() && !matches!(cursor, DrawMeasureCursor::Complete) {
        postprocess += usize::from(matches!(cursor, DrawMeasureCursor::PassPostprocess { .. } | DrawMeasureCursor::PassBackdropSnapshot { .. } | DrawMeasureCursor::PassSceneClear { .. }));
    }
    assert_eq!(postprocess, 0, "ordinary rectangular worlds add no capture, clear, or postprocess scalar");
}

/// 🖥️ LAW: grid uniforms stay in logical scene units at every device scale; only the
/// encoder viewport and scissor cross the logical-to-physical boundary.
#[test]
fn procedural_grid_uniforms_are_dpr_invariant_while_viewport_and_scissor_scale_physically() {
    let _guard = prepared_process_guard();
    use crate::wgpu::draw::{physical_scissor_rect, physical_viewport_rect, World3dGridUniforms};
    use crate::wgpu::draw_types::ScissorRect;
    use crate::wgpu::kernel_3d_scene::ProceduralGrid3d;
    let grid = ProceduralGrid3d { plane_z: 2.001, camera_plane_projection: [7.0, -4.0, 2.001], cell_size: 2.5, fade_distance: 18.0, cell_color: [0.2, 0.3, 0.4] };
    let at_one = World3dGridUniforms::from_grid(&grid);
    let at_two = World3dGridUniforms::from_grid(&grid);
    assert_eq!(size_of::<World3dGridUniforms>(), 256, "one fixed GPU uniform allocation owns the grid scalar");
    assert_eq!(bytemuck::bytes_of(&at_one), bytemuck::bytes_of(&at_two), "DPR cannot enter logical grid uniforms");
    assert_eq!(at_one.plane_cell, [2.001, 2.5, 0.6, 18.0]);
    assert_eq!(at_one.camera_fade, [7.0, -4.0, 2.001, 1.5]);
    assert_eq!(at_one.cell_color, [0.2, 0.3, 0.4, 0.0]);
    let viewport = [11.0, 13.0, 120.0, 80.0];
    assert_eq!(physical_viewport_rect(viewport, 1.0), viewport);
    assert_eq!(physical_viewport_rect(viewport, 2.0), [22.0, 26.0, 240.0, 160.0]);
    let scissor = ScissorRect { x: 11, y: 13, w: 120, h: 80 };
    assert_eq!(physical_scissor_rect(scissor, 1.0), scissor);
    assert_eq!(physical_scissor_rect(scissor, 2.0), ScissorRect { x: 22, y: 26, w: 240, h: 160 });
}

#[test]
fn a_disabled_shadow_never_publishes_a_gpu_shadow_scalar() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};
    let instance = Instance3d { id: String::new(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d { draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance], shadow_role: Default::default() }], ..Default::default() });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    for _ in 0..32 {
        assert!(!matches!(cursor, DrawMeasureCursor::PassShadowBegin(..) | DrawMeasureCursor::PassShadowInstance { .. }));
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() || matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
}

/// 🎟️ A raster producer must be bound to the frame it is published into BEFORE it is pushed, and the
/// job's refusal must be readable — `StepOutcome::Fault` carries an empty payload here, so
/// [`PreparedRenderJob::fault`] is the only way a driver can name it
/// (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn an_unbound_raster_producer_refuses_its_prepared_job_with_a_readable_fault() {
    let _guard = prepared_process_guard();
    let (unbound, _) = PreparedRasterProducer::try_admit("unbound".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(unbound).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(4, u64::MAX), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert_eq!(job.fault(), Some("raster producer generation is stale"));
    while !job.close_step() {}

    let (mut bound, _) = PreparedRasterProducer::try_admit("bound".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(bound.bind_frame_generation(3));
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(bound).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(4, u64::MAX), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(outcome, StepOutcome::Yield));
    assert_eq!(job.fault(), None);
    while !job.close_step() {}
}

#[test]
fn raster_ledger_exact_item_and_generation_slot_caps_reject_plus_one() {
    let _guard = prepared_process_guard();
    let mut ledger = PreparedRasterLedger::default();
    let item = ledger.reserve(PREPARED_RASTER_PRODUCER_ITEMS, 1).expect("exact aggregate items");
    assert!(ledger.reserve(1, 0).is_none(), "aggregate item cap plus one");
    assert!(ledger.release(&item));

    let bytes = ledger.reserve(1, PREPARED_RASTER_PRODUCER_BYTES).expect("exact aggregate bytes");
    assert!(ledger.reserve(0, 1).is_none(), "aggregate byte cap plus one");
    assert!(ledger.release(&bytes));

    let mut credits = Vec::with_capacity(PREPARED_RASTER_PRODUCER_CAPACITY);
    for _ in 0..PREPARED_RASTER_PRODUCER_CAPACITY {
        credits.push(ledger.reserve(1, 1).expect("exact fixed generation slot"));
    }
    assert!(ledger.reserve(1, 1).is_none(), "generation slot cap plus one");
    for credit in credits {
        assert!(ledger.release(&credit));
    }
    assert_eq!((ledger.items, ledger.bytes), (0, 0));
}

#[test]
fn raster_credit_epoch_rejects_aba_and_cancel_retires_one_owner_per_grant() {
    let _guard = prepared_process_guard();
    let (mut first, _) = PreparedRasterProducer::try_admit("first".into(), vec![1; PREPARED_RASTER_PAGE_BYTES * 2], 4_096, 2).expect("first admission");
    let first_epoch = first.pages.as_ref().expect("first pages").source_generation();
    assert!(first.bind_frame_generation(3));
    assert!(matches!(first.step(3), PreparedRasterProducerStep::Pending));
    first.begin_close();
    assert!(!first.close_step());
    assert!(first.pages.as_ref().expect("closing pages").slots.is_empty(), "first close grant retires only the built page");
    assert_eq!(first.source.len(), PREPARED_RASTER_PAGE_BYTES * 2, "source remains fully owned after the page grant");
    while !first.close_step() {}
    let (mut second, _) = PreparedRasterProducer::try_admit("second".into(), vec![2; 4], 1, 1).expect("reused slot admission");
    let second_epoch = second.pages.as_ref().expect("second pages").source_generation();
    assert_eq!(second_epoch.slot(), first_epoch.slot(), "released fixed slot is reused");
    assert!(second_epoch.epoch() > first_epoch.epoch(), "reused fixed slot advances its generation");
    second.begin_close();
    while !second.close_step() {}
}

#[test]
fn zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation() {
    let _guard = prepared_process_guard();
    let (mut producer, _) = PreparedRasterProducer::try_admit("governed".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(producer.bind_frame_generation(3));
    let source_pointer = producer.source.as_ptr();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;

    let zero = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(0, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(zero, StepOutcome::Yield));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.source.as_ptr(), source_pointer);
    assert!(retained.pages.as_ref().unwrap().slots.is_empty());

    let expired = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 1), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(expired, StepOutcome::Yield));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.source.as_ptr(), source_pointer);
    assert!(retained.pages.as_ref().unwrap().slots.is_empty());

    while !job.close_step() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn cancellation_retires_large_upload_incrementally_before_terminal_empty() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4_096], width: 64, height: 64 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    assert!(!job.close_step());
    assert!(!job.terminal_is_empty());
    let mut turns = 1;
    while !job.close_step() {
        turns += 1;
        assert!(turns < 5_000);
    }
    assert!(turns > 4_096);
    assert!(job.terminal_is_empty());
}

fn assert_send<T: Send>() {}

#[test]
fn prepared_packet_is_send_owned_data() {
    let _guard = prepared_process_guard();
    assert_send::<PreparedRenderPacket>();
    assert_send::<PreparedRenderJob>();
    assert_send::<PreparedRenderReceiver>();
    assert_send::<PreparedRenderGate>();
    let packet = packet(7, 3);
    let identity = std::thread::spawn(move || (packet.scene_revision, packet.preview_generation)).join().expect("worker packet");
    assert_eq!(identity, (7, 3));
}

#[test]
fn receiver_survives_worker_ownership_of_the_job() {
    let _guard = prepared_process_guard();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let receiver = job.receiver().expect("prepared receiver clone");
    std::thread::spawn(move || {
        let mut job = job;
        let mut preview = 0;
        loop {
            let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
            if outcome.is_terminal() {
                assert!(matches!(outcome, StepOutcome::Complete(_)));
                break;
            }
        }
    })
    .join()
    .expect("worker preparation");
    let packet = receiver.take_latest().expect("prepared packet handoff");
    assert_eq!((packet.scene_revision(), packet.preview_generation()), (7, 3));
    assert!(receiver.take_latest().is_none());
    drop(packet);
    drain_abandoned_preparations();
}

#[test]
fn preparation_yields_at_the_configured_item_budget() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers.extend((0..4).map(|_| DrawLayer::default()));
    let input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let first = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(first, StepOutcome::Yield));
    drop(job);
    drain_abandoned_preparations();
}

#[test]
fn preparation_completes_across_bounded_steps() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers.extend((0..2).map(|_| DrawLayer::default()));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("prepared packet");
    assert_eq!((packet.scene_revision, packet.preview_generation), (7, 3));
    while !packet.retire_step() {}
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn preparation_rejects_a_stale_generation_before_publication() {
    let _guard = prepared_process_guard();
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 2, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    drop(job);
    drain_abandoned_preparations();
}

#[semio_framework_async_macros::async_test]
async fn preparation_observes_cancellation_without_replacing_a_packet() {
    let cancel = root_cancel_token();
    cancel.cancel().await;
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), cancel, now_ms, &mut preview, &mut None);
    assert!(matches!(outcome, StepOutcome::Cancelled));
    assert!(job.take_packet().is_none());
    drop(job);
    drain_abandoned_preparations();
}

#[test]
fn stale_packet_rejection_preserves_the_last_valid_packet() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    assert!(gate.acknowledge_presented(witness).expect("first presenter acknowledgement").is_empty());
    let stale = packet(6, 3);
    assert!(matches!(gate.validate(&stale, 7, 3), Err(PreparedRenderRejection::StaleRevision { .. })));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
}

#[test]
fn generation_rejection_happens_before_presentation() {
    let _guard = prepared_process_guard();
    let gate = PreparedRenderGate::default();
    assert!(matches!(gate.validate(&packet(7, 2), 7, 3), Err(PreparedRenderRejection::StaleGeneration { .. })));
}

#[test]
fn device_loss_retains_the_last_valid_packet() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert_eq!(gate.retain_after_device_loss(), Some((7, 3)));
}

#[test]
fn presenter_ack_is_exact_one_shot_and_preserves_old_until_acknowledged() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let second = gate.stage_presented(packet(8, 4)).ok().expect("second presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "candidate is not visible before acknowledgement");
    let stale = PreparedPresenterWitness { sequence: second.sequence.saturating_add(1), scene_revision: second.scene_revision, preview_generation: second.preview_generation };
    assert!(gate.acknowledge_presented(stale).is_err());
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let duplicate = PreparedPresenterWitness { sequence: second.sequence, scene_revision: second.scene_revision, preview_generation: second.preview_generation };
    let mut replacement = gate.acknowledge_presented(second).expect("exact second acknowledgement");
    assert_eq!(gate.last_valid_identity(), Some((8, 4)));
    assert_eq!(replacement.previous.as_ref().map(|packet| (packet.scene_revision, packet.preview_generation)), Some((7, 3)));
    assert!(gate.acknowledge_presented(duplicate).is_err(), "duplicate acknowledgement is stale after publication");
    let mut previous = replacement.take_previous().expect("old last-valid owner");
    while !previous.retire_step() {}
    assert!(previous.retirement_is_empty());
}

#[test]
fn accepted_a_stale_b_abort_and_accepted_c_preserve_exact_presenter_owners() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let _missing = gate.stage_presented(packet(8, 4)).ok().expect("pending presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let mut stale = gate.abort_pending().expect("exact stale B packet handback");
    assert_eq!((stale.scene_revision, stale.preview_generation), (8, 4));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "aborting B preserves accepted A");

    let third = gate.stage_presented(packet(9, 5)).ok().expect("successor C presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "C remains private before acknowledgement");
    let mut replacement = gate.acknowledge_presented(third).expect("successor C acknowledgement");
    assert_eq!(gate.last_valid_identity(), Some((9, 5)), "C alone replaces A after exact acknowledgement");
    let mut first = replacement.take_previous().expect("accepted A owner handback");
    assert_eq!((first.scene_revision, first.preview_generation), (7, 3));
    while !stale.retire_step() {}
    while !first.retire_step() {}
    let mut third = gate.take_last_valid().expect("accepted C owner handback");
    while !third.retire_step() {}
    while !gate.close_step() {}
    assert!(gate.terminal_is_empty());
}

#[test]
fn pending_presenter_witness_rejects_superseding_packet_with_exact_owner() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let _pending = gate.stage_presented(packet(7, 3)).ok().expect("pending presenter witness");
    let mut superseding = packet(8, 4);
    assert!(superseding.uploads.try_push(PreparedRenderUpload::GlyphAtlas { pixels: vec![7; 16_385], width: 1, height: 1 }).is_ok());
    let pixels = match superseding.uploads.get(0) {
        Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) => pixels.as_ptr(),
        _ => unreachable!(),
    };
    let mut returned = match gate.stage_presented(superseding) {
        Ok(_) => panic!("second presenter witness must fail closed"),
        Err(packet) => packet,
    };
    assert_eq!((returned.scene_revision, returned.preview_generation), (8, 4));
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels: returned_pixels, .. }) if returned_pixels.as_ptr() == pixels));
    assert!(!returned.retire_step(), "one close grant retires only one admitted pixel page");
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) if pixels.len() == 1));
}

#[test]
fn gate_close_requires_pending_and_last_valid_packet_handback_before_terminal_scalars() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert!(!gate.close_step(), "last-valid owner prevents gate terminalization");
    let mut last = gate.take_last_valid().expect("last-valid owner handback");
    while !last.retire_step() {}
    assert!(!gate.close_step(), "first scalar grant retires only the sequence");
    assert!(gate.close_step(), "second scalar grant publishes the terminal witness");
    assert!(gate.terminal_is_empty());
    assert!(matches!(gate.validate(&packet(8, 4), 8, 4), Err(PreparedRenderRejection::Closing)));
}

#[test]
fn upload_byte_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4], width: 2, height: 2 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn eviction_byte_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_eviction(PreparedRenderEviction::Mesh { key: "mesh".into() }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn draw_item_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers[0].ui_instances.push(crate::wgpu::draw_types::UiInstance::solid([0.0; 4], crate::wgpu::theme::Rgba::new(0.0, 0.0, 0.0, 0.0)));
    let mut input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    input.limits.max_draw_items = 0;
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn input_drop_hands_back_exact_process_permits_for_incremental_close() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    drop(input);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    let mut turns = 0;
    while !PreparedRenderInput::close_abandoned_step() {
        turns += 1;
        assert!(turns < 128);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn worker_panic_hands_back_the_exact_job_and_mailbox_owners() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owner = job;
        panic!("hostile worker interruption");
    }));
    assert!(result.is_err());
    let mut turns = 0;
    while !PreparedRenderJob::close_abandoned_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    assert!(PREPARED_RENDER_MAILBOX.iter().all(|slot| slot.packet.load(Ordering::Acquire).is_null()));
}

#[test]
fn packet_drop_retires_nested_backings_and_permit_scalars_separately() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let mut owner = packet(7, 3);
    owner.draw.push_solid([0.0, 0.0, 8.0, 8.0], crate::wgpu::theme::Rgba::new(1.0, 0.0, 0.0, 1.0));
    drop(owner);
    let mut turns = 0;
    while !PreparedRenderPacket::close_abandoned_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 8);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn fixed_command_pages_reject_max_plus_one_without_consuming_the_owner() {
    let _guard = prepared_process_guard();
    let mut commands = PreparedRenderCommandPages::default();
    for source in 0..PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS {
        let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source, digest: source as u64, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: false };
        assert!(commands.try_push(command).is_ok());
    }
    let rejected = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source: usize::MAX, digest: u64::MAX, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: true };
    let returned = match commands.try_push(rejected) {
        Ok(()) => panic!("command cap plus one must refuse"),
        Err(returned) => returned,
    };
    assert_eq!((returned.source, returned.digest, returned.packet_overlay), (usize::MAX, u64::MAX, true));
    let mut turns = 0;
    while !commands.close_step() {
        turns += 1;
    }
    assert!(turns >= PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS);
    assert!(commands.terminal_is_empty());
}

#[test]
fn tessellation_commands_retain_exact_scalar_and_overlay_cursors() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 4.0, 4.0], crate::wgpu::theme::Rgba::new(0.0, 1.0, 0.0, 1.0));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let mut preview = 0;
    let mut steps = 0;
    loop {
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(41), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
        steps += 1;
        if outcome.is_terminal() {
            assert!(matches!(outcome, StepOutcome::Complete(_)));
            break;
        }
        assert!(steps < 128);
    }
    let mut packet = match job.take_packet() {
        Some(packet) => packet,
        None => panic!("prepared packet handoff"),
    };
    assert!((0..packet.commands.len())
        .filter_map(|index| packet.commands.get(index))
        .any(|command| { command.kind == PreparedRenderCommandKind::Tessellate && command.draw_cursor == Some(DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false }) && !command.packet_overlay }));
    while !packet.retire_step() {}
    while !job.close_step() {}
}

#[test]
fn prepared_measurement_retains_scene_and_overlay_raster_cursors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    let key = fixture["image"]["sharedKey"].as_str().expect("shared raster key");
    let mut draw = DrawList::default();
    draw.push_raster_quad(key, [0.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.begin_overlay_route();
    draw.push_raster_quad(key, [4.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.end_overlay_route();
    let mut cursor = DrawMeasureCursor::LayerHeader(0);
    let mut measured = Vec::new();
    for _ in 0..64 {
        let current = cursor;
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() {
            break;
        }
        measured.push(current);
        if cursor == DrawMeasureCursor::Complete {
            break;
        }
    }
    assert!(measured.contains(&DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: false }));
    assert!(measured.contains(&DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: true }));
    let raster_order: Vec<_> = measured
        .iter()
        .filter_map(|cursor| match cursor {
            DrawMeasureCursor::LayerRaster { overlay, .. } => Some(*overlay),
            _ => None,
        })
        .collect();
    for row in fixture["image"]["draws"].as_array().expect("draw order rows") {
        let ordinal = usize::try_from(row["expectedOrdinal"].as_u64().expect("expected ordinal")).expect("ordinal fits");
        assert_eq!(raster_order.get(ordinal).copied(), Some(row["route"].as_str() == Some("overlay")));
    }
}

#[test]
fn prepared_packet_publishes_main_overlay_and_top_overlay_raster_owners() {
    let _guard = prepared_process_guard();
    let mut packet = packet(7, 3);
    packet.draw.push_raster_quad("main-image", [0.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.draw.begin_overlay_route();
    packet.draw.push_raster_quad("inline-overlay-image", [4.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.draw.end_overlay_route();
    let mut overlay = DrawList::default();
    overlay.push_raster_quad("top-overlay-image", [8.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.overlay = Some(overlay);
    let mut cursor = PreparedRasterKeepCursorV1::default();
    let mut keys = Vec::new();
    for _ in 0..64 {
        match packet.raster_keep_step(&mut cursor) {
            PreparedRasterKeepStepV1::Pending => {}
            PreparedRasterKeepStepV1::Key(key) => keys.push(key.to_owned()),
            PreparedRasterKeepStepV1::Complete => break,
        }
    }
    assert_eq!(keys, ["main-image", "inline-overlay-image", "top-overlay-image"]);
    while !packet.retire_step() {}
}

#[test]
fn a_scene_pass_is_prepared_between_the_ui_scalars_authored_around_it() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌌️prepared-scene-ui-stacking/🔣️.json")).expect("neutral scene/UI stacking fixture");
    let steps = fixture["steps"].as_array().expect("authored stacking steps");
    let mut draw = DrawList::default();
    for step in steps {
        let rect = std::array::from_fn(|index| step["rect"][index].as_f64().expect("rect scalar") as f32);
        let rgba = std::array::from_fn::<_, 4, _>(|index| step["rgba"][index].as_u64().expect("color scalar") as f32 / 255.0);
        match step["op"].as_str().expect("operation") {
            "solid" => draw.push_solid(rect, crate::wgpu::theme::Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3])),
            "scene" => draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d { viewport: rect, ..Default::default() }),
            op => panic!("unknown fixture operation {op}"),
        }
    }

    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("accepted packet");
    let mut order = Vec::new();
    for index in 0..packet.command_pages().len() {
        let Some(command) = packet.command_pages().get(index) else { continue };
        let authored = match command.draw_cursor() {
            Some(DrawMeasureCursor::LayerUi { layer, item, overlay: false }) => {
                let value = &packet.draw.layers[layer].ui_instances[item];
                steps.iter().find(|step| step["op"] == "solid" && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.rect[index]) && (0..4).all(|index| step["rgba"][index].as_u64().unwrap() as f32 / 255.0 == value.color[index]))
            }
            Some(DrawMeasureCursor::PassHeader(pass)) if pass < packet.draw.scene_passes.len() => {
                let value = &packet.draw.scene_passes[pass];
                steps.iter().find(|step| step["op"] == "scene" && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.viewport[index]))
            }
            _ => None,
        };
        if let Some(step) = authored {
            order.push(step["id"].as_str().expect("step id").to_string());
        }
    }

    let width = fixture["size"][0].as_u64().expect("width") as u32;
    let height = fixture["size"][1].as_u64().expect("height") as u32;
    let mut reference = tiny_skia::Pixmap::new(width, height).expect("independent raster oracle");
    for step in steps {
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(step["rgba"][0].as_u64().unwrap() as u8, step["rgba"][1].as_u64().unwrap() as u8, step["rgba"][2].as_u64().unwrap() as u8, step["rgba"][3].as_u64().unwrap() as u8);
        let rect =
            tiny_skia::Rect::from_xywh(step["rect"][0].as_f64().unwrap() as f32, step["rect"][1].as_f64().unwrap() as f32, step["rect"][2].as_f64().unwrap() as f32, step["rect"][3].as_f64().unwrap() as f32).expect("nondegenerate fixture rect");
        reference.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
    }
    for sample in fixture["samples"].as_array().expect("pixel samples") {
        let color = reference.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).expect("sample in bounds");
        assert_eq!(serde_json::json!([color.red(), color.green(), color.blue(), color.alpha()]), sample["rgba"], "independent tiny-skia authored-order sample");
    }
    let expected = fixture["expectedOrder"].as_array().expect("expected order").iter().map(|id| id.as_str().expect("expected id").to_string()).collect::<Vec<_>>();
    while !packet.retire_step() {}
    while !job.close_step() {}
    assert_eq!(order, expected, "an opaque pane authored after World must remain after that exact scene pass in prepared commands");
}

#[test]
fn an_inline_overlay_follows_its_layer_scene_and_precedes_the_following_layer() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️prepared-scene-overlay-stacking/🔣️.json")).expect("neutral scene/overlay stacking fixture");
    let steps = fixture["steps"].as_array().expect("source stacking steps");
    let mut draw = DrawList::default();
    for step in steps {
        let rect = std::array::from_fn(|index| step["rect"][index].as_f64().expect("rect scalar") as f32);
        let rgba = std::array::from_fn::<_, 4, _>(|index| step["rgba"][index].as_u64().expect("color scalar") as f32 / 255.0);
        match (step["op"].as_str().expect("operation"), step["route"].as_str().expect("route")) {
            ("solid", "normal") => draw.push_solid(rect, crate::wgpu::theme::Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3])),
            ("solid", "overlay") => {
                draw.begin_overlay_route();
                draw.push_solid(rect, crate::wgpu::theme::Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3]));
                draw.end_overlay_route();
            }
            ("raster", "normal") => draw.push_raster_quad(step["rasterKey"].as_str().expect("raster key"), rect, [0.0, 0.0, 1.0, 1.0], 1.0),
            ("scene", "scene") => draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d { viewport: rect, ..Default::default() }),
            (op, route) => panic!("unknown fixture operation {op}/{route}"),
        }
    }

    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("accepted packet");
    let mut order = Vec::new();
    for index in 0..packet.command_pages().len() {
        let Some(command) = packet.command_pages().get(index) else { continue };
        let authored = match command.draw_cursor() {
            Some(DrawMeasureCursor::LayerUi { layer, item, overlay }) => {
                let values = if overlay { &packet.draw.layers[layer].overlay_ui_instances } else { &packet.draw.layers[layer].ui_instances };
                let value = &values[item];
                steps.iter().find(|step| {
                    step["op"] == "solid"
                        && (step["route"] == "overlay") == overlay
                        && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.rect[index])
                        && (0..4).all(|index| step["rgba"][index].as_u64().unwrap() as f32 / 255.0 == value.color[index])
                })
            }
            Some(DrawMeasureCursor::LayerRaster { layer, raster, overlay: false }) => {
                let (key, value) = &packet.draw.layers[layer].raster_instances[raster];
                steps.iter().find(|step| step["op"] == "raster" && step["rasterKey"] == key.as_str() && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.rect[index]))
            }
            Some(DrawMeasureCursor::PassHeader(pass)) if pass < packet.draw.scene_passes.len() => {
                let value = &packet.draw.scene_passes[pass];
                steps.iter().find(|step| step["op"] == "scene" && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.viewport[index]))
            }
            _ => None,
        };
        if let Some(step) = authored {
            order.push(step["id"].as_str().expect("step id").to_string());
        }
    }

    let width = fixture["size"][0].as_u64().expect("width") as u32;
    let height = fixture["size"][1].as_u64().expect("height") as u32;
    let expected = fixture["expectedOrder"].as_array().expect("expected order");
    let mut reference = tiny_skia::Pixmap::new(width, height).expect("independent raster oracle");
    for id in expected {
        let step = steps.iter().find(|step| step["id"] == *id).expect("expected step exists");
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(step["rgba"][0].as_u64().unwrap() as u8, step["rgba"][1].as_u64().unwrap() as u8, step["rgba"][2].as_u64().unwrap() as u8, step["rgba"][3].as_u64().unwrap() as u8);
        let rect =
            tiny_skia::Rect::from_xywh(step["rect"][0].as_f64().unwrap() as f32, step["rect"][1].as_f64().unwrap() as f32, step["rect"][2].as_f64().unwrap() as f32, step["rect"][3].as_f64().unwrap() as f32).expect("nondegenerate fixture rect");
        reference.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
    }
    for sample in fixture["samples"].as_array().expect("pixel samples") {
        let color = reference.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).expect("sample in bounds");
        assert_eq!(serde_json::json!([color.red(), color.green(), color.blue(), color.alpha()]), sample["rgba"], "independent tiny-skia route-order sample");
    }
    let expected = expected.iter().map(|id| id.as_str().expect("expected id").to_string()).collect::<Vec<_>>();
    while !packet.retire_step() {}
    while !job.close_step() {}
    assert_eq!(order, expected, "an inline overlay follows its own layer's scene but stays behind a later covering layer");
}

#[test]
fn prepared_glass_and_foreground_follow_authored_partial_and_nested_stacking() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️prepared-glass-stacking/🔣️.json")).expect("neutral glass stacking fixture");
    let steps = fixture["steps"].as_array().expect("authored steps");
    let mut draw = DrawList::default();
    let mut current_glass = None;
    for step in steps {
        let rect = || std::array::from_fn(|index| step["rect"][index].as_f64().expect("rect scalar") as f32);
        let rgba = || std::array::from_fn::<_, 4, _>(|index| step["rgba"][index].as_u64().expect("color scalar") as f32 / 255.0);
        match step["op"].as_str().expect("operation") {
            "solid" => {
                let color = rgba();
                draw.push_solid(rect(), crate::wgpu::theme::Rgba::new(color[0], color[1], color[2], color[3]));
            }
            "glass" => {
                let color = rgba();
                current_glass = Some(draw.push_glass(rect(), 0.0, crate::wgpu::theme::GlassStyle { tint: crate::wgpu::theme::Rgba::new(color[0], color[1], color[2], color[3]), alpha: color[3], blur_px: 0.0, saturate: 1.0 }));
            }
            "begin" => draw.begin_glass_content(current_glass.expect("preceding glass")),
            "end" => draw.end_glass_content(),
            op => panic!("unknown fixture operation {op}"),
        }
    }
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("accepted packet");
    let mut order = Vec::new();
    for index in 0..packet.command_pages().len() {
        let Some(command) = packet.command_pages().get(index) else { continue };
        let item = match command.draw_cursor() {
            Some(DrawMeasureCursor::LayerUi { layer, item, overlay: false }) => {
                let value = &packet.draw.layers[layer].ui_instances[item];
                Some((value.rect, value.color))
            }
            Some(DrawMeasureCursor::Glass(region)) if region < packet.draw.glass_regions.len() => {
                let value = &packet.draw.glass_regions[region];
                Some((value.rect, [value.tint.r, value.tint.g, value.tint.b, value.alpha]))
            }
            _ => None,
        };
        if let Some((rect, color)) = item {
            let step = steps
                .iter()
                .find(|step| step.get("id").is_some() && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == rect[index]) && (0..4).all(|index| step["rgba"][index].as_u64().unwrap() as f32 / 255.0 == color[index]))
                .expect("every emitted draw has an authored identity");
            order.push(step["id"].as_str().unwrap().to_string());
        }
    }
    let width = fixture["size"][0].as_u64().unwrap() as u32;
    let height = fixture["size"][1].as_u64().unwrap() as u32;
    let mut reference = tiny_skia::Pixmap::new(width, height).expect("independent raster oracle");
    for step in steps.iter().filter(|step| step.get("rect").is_some()) {
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(step["rgba"][0].as_u64().unwrap() as u8, step["rgba"][1].as_u64().unwrap() as u8, step["rgba"][2].as_u64().unwrap() as u8, step["rgba"][3].as_u64().unwrap() as u8);
        let rect = tiny_skia::Rect::from_xywh(step["rect"][0].as_f64().unwrap() as f32, step["rect"][1].as_f64().unwrap() as f32, step["rect"][2].as_f64().unwrap() as f32, step["rect"][3].as_f64().unwrap() as f32).unwrap();
        reference.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
    }
    for sample in fixture["samples"].as_array().unwrap() {
        let color = reference.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).unwrap();
        assert_eq!(serde_json::json!([color.red(), color.green(), color.blue(), color.alpha()]), sample["rgba"], "independent tiny-skia stacking sample");
    }
    let expected = fixture["expectedOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect::<Vec<_>>();
    while !packet.retire_step() {}
    while !job.close_step() {}
    assert_eq!(order, expected, "a partial popup covers earlier panel text, and resumed parent content follows nested content");
}

```

## 🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-theme-token-parity/🦀️.rs

SHA-256: 99848b4231d1e8016d7a9955b2ba42a46297b0356f67f4d3b967620170548f01

```rust
//! ⚖️ Law: every colour and metric the wgpu targets paint comes from the generated `ui_styling`
//! token crate — the same `🎨️styling/🔣️.json` that emits React's `🎨️palette/🎨️.css`. A hand-written
//! channel literal in a wgpu target is the only way the two renderers can silently drift apart, so
//! it fails here instead of at a screenshot diff.

use super::*;
use std::fs;
use std::path::{Path, PathBuf};

/// 📂️ Repo root, five levels above `semio-framework-ui`'s manifest
/// (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust`).
fn repo_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..5 {
        root = root.parent().expect("manifest dir has five ancestors up to the repo root").to_path_buf();
    }
    root
}

/// 🔎️ Source trees whose `🧊️wgpu` files this law owns: the ui target itself (chrome/widgets/shell/
/// draw/theme), the per-element wgpu targets, and the os renderer's Shell/Dock/Scenes wgpu targets.
const SCAN_ROOTS: &[&str] = &["🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu", "🧰️framework/🔨️modules/🖱️ui/🧱️elements", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer"];

/// 🎨️ The two constructors that turn raw channels into paint. Every other colour must be a
/// `Rgba::from_token` lift of a generated palette entry, or derived from an existing `Rgba`.
const COLOR_CONSTRUCTORS: &[&str] = &["Rgba::new(", "Rgba::from_srgb8("];

/// 🪪️ Renderer-internal paints that are deliberately not tokens, each with the reason it stays a
/// literal. Anything not listed here must come from `ui_styling`.
const ALLOWLIST: &[(&str, &str, &str)] = &[
    (
        "🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs",
        "let white = Rgba::new(1.0, 1.0, 1.0, 1.0);",
        "🎭️ Scissor-mask identity in a `#[cfg(test)]` helper — opaque white is the stencil's \"keep\" value, not a colour anybody sees.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "const CANVAS2D_SELECTION_RING",
        "🟡️ Verbatim port of React `canvas-2d-host.tsx`'s own `rgba(251, 191, 36, …)` literal; the drift lives on the React side, so tokenising only this half would create the divergence it prevents.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "const CANVAS2D_SELECTION_GLOW",
        "🟡️ Same literal as `CANVAS2D_SELECTION_RING`, at the glow alpha.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "fn canvas_color_channels",
        "🧮️ Per-channel default of a pure `[f64]` → `Rgba` payload decoder: what a malformed scene packet gets, not theme paint. The decoder takes no `Theme`; plumbing one in is a Scenes packet, not a token one.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "fn canvas_gradient_color_at",
        "🧮️ Empty-stop-list fallback of the same pure payload decoder — see `canvas_color_channels`.",
    ),
    (
        "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "const LOGO_UNTINTED",
        "⬜️ Identity TINT MULTIPLIER, not a paint: the brand mark is the one icon cell rasterised in its own four hues (`rasterize_svg(svg, tint_mask = id != \"semio-logo\")`), and React paints `<SemioLogo>` untinted. Any token here would multiply the mark down to a single hue — the black disc this const exists to prevent.",
    ),
];

/// 🔢️ True when the three COLOUR channels are plain numeric literals — i.e. the call writes a hue by
/// hand rather than reading one. Alpha is excluded on purpose: `…, 0.9 * opacity)` is still a
/// hand-written colour. `Rgba::new(tip.color.r, …)` and `Rgba::from_srgb8(r, g, b, 255)` (a parsed
/// user hex) carry derived channels and pass.
fn channels_are_hand_written(arguments: &str) -> bool {
    let channels: Vec<&str> = arguments.split(',').take(3).collect();
    if channels.len() < 3 {
        return false;
    }
    channels.iter().all(|argument| {
        let argument = argument.trim().trim_end_matches("_f32").trim_end_matches("_u8");
        !argument.is_empty() && argument.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '_' || c == '-')
    })
}

/// 🔍️ Argument text of the constructor call starting at `open` (the index just past its `(`), or
/// `None` when the call spans past the end of the line.
fn call_arguments(line: &str, open: usize) -> Option<&str> {
    let rest = &line[open..];
    let mut depth = 1_i32;
    for (index, character) in rest.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&rest[..index]);
                }
            }
            _ => {}
        }
    }
    None
}

fn rust_sources(directory: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name == "node_modules" || name == "target" || name.starts_with("🗑️") || name.starts_with('.') {
                continue;
            }
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

/// 🧭️ Nearest preceding `const`/`fn`/`static` item name, so an allowlist entry can name a symbol
/// instead of a line number that every neighbouring edit invalidates.
fn enclosing_item<'a>(lines: &'a [&'a str], index: usize) -> &'a str {
    for line in lines[..=index].iter().rev() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") || trimmed.starts_with("const ") || trimmed.starts_with("pub const ") || trimmed.starts_with("pub(crate) fn ") || trimmed.starts_with("static ") {
            return trimmed;
        }
    }
    ""
}

fn is_allowlisted(relative: &str, line: &str, item: &str) -> bool {
    ALLOWLIST.iter().any(|(path, marker, _)| relative.ends_with(path) && (line.contains(marker) || item.starts_with(marker)))
}

/// 🧪️ Test case directories build expected paints by hand on purpose — that is the oracle,
/// not production paint.
#[test]
fn no_wgpu_target_paints_a_hand_written_colour_literal() {
    let root = repo_root();
    let mut violations: Vec<String> = Vec::new();
    let mut scanned = 0_usize;
    for scan_root in SCAN_ROOTS {
        let absolute = root.join(scan_root);
        assert!(absolute.is_dir(), "scan root {scan_root} is missing — the law would pass vacuously");
        let mut files = Vec::new();
        rust_sources(&absolute, &mut files);
        for file in files {
            let relative = file.strip_prefix(&root).unwrap_or(&file).to_string_lossy().to_string();
            if !relative.contains("🧊️wgpu") {
                continue;
            }
            if relative.contains("🧪️tests") {
                continue;
            }
            scanned += 1;
            let text = fs::read_to_string(&file).expect("wgpu source reads as utf8");
            let lines: Vec<&str> = text.lines().collect();
            for (index, line) in lines.iter().enumerate() {
                if line.trim_start().starts_with("//") {
                    continue;
                }
                for constructor in COLOR_CONSTRUCTORS {
                    let Some(at) = line.find(constructor) else { continue };
                    let Some(arguments) = call_arguments(line, at + constructor.len()) else { continue };
                    if !channels_are_hand_written(arguments) {
                        continue;
                    }
                    let item = enclosing_item(&lines, index);
                    if is_allowlisted(&relative, line, item) {
                        continue;
                    }
                    violations.push(format!("{relative}:{}: {}", index + 1, line.trim()));
                }
            }
        }
    }
    assert!(scanned > 0, "no wgpu sources were scanned — the walk is broken");
    assert!(
        violations.is_empty(),
        "wgpu targets must read every colour from the generated ui_styling tokens (🎨️styling/🔣️.json, the same source as React's 🎨️palette/🎨️.css).\n\
         Add the paint to 🔣️.json and read it with `Rgba::from_token`, or — only for a genuinely renderer-internal paint — extend this law's ALLOWLIST with its reason.\n\
         {} violation(s):\n{}",
        violations.len(),
        violations.join("\n")
    );
}

/// 🎨️ Reads one `--color-*` declaration out of the generated palette CSS React consumes.
fn generated_palette_hex(name: &str) -> String {
    let css = fs::read_to_string(repo_root().join("🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css")).expect("generated palette css is on disk");
    let needle = format!("--color-{name}:");
    let line = css.lines().find(|line| line.trim_start().starts_with(&needle)).unwrap_or_else(|| panic!("generated palette css declares --color-{name}"));
    line.split(':').nth(1).expect("declaration has a value").trim().trim_end_matches(';').to_string()
}

fn hex_of(color: Rgba) -> String {
    let [r, g, b, _] = ui_styling::color::linear_to_rgba8(color.r, color.g, color.b, color.a);
    format!("#{r:02x}{g:02x}{b:02x}")
}

#[test]
fn outcome_paints_decode_to_the_same_hex_react_paints() {
    for (appearance, theme) in [("light", Theme::light()), ("dark", Theme::dark())] {
        for (field, value, token) in [
            ("error", theme.error, "danger"),
            ("diff_added", theme.diff_added, "diff-added"),
            ("success", theme.success, "success"),
            ("warning", theme.warning, "warning"),
            ("progress", theme.progress, "secondary"),
        ] {
            assert_eq!(hex_of(value), generated_palette_hex(token), "{appearance} theme.{field} must decode to --color-{token}");
        }
    }
}

#[test]
fn semantic_panel_decodes_to_reacts_live_css_alias_and_custom_theme_input() {
    assert_eq!(hex_of(Theme::light().panel), generated_palette_hex("light-5-7"));
    assert_eq!(hex_of(Theme::dark().panel), generated_palette_hex("dark-7-9"));
    let mono = Theme::mono(false);
    assert_eq!(mono.panel, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.panel), "a premade theme owns its semantic panel token");
    assert_ne!(mono.panel, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.level_panel), "the semantic --panel alias remains distinct from the hierarchy level ramp");
}

#[test]
fn every_theme_metric_is_a_multiple_of_the_shared_ui_spacing() {
    let theme = Theme::dark();
    let step = ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX as f32;
    for (field, value) in [
        ("navbar_height", theme.navbar_height),
        ("footer_height", theme.footer_height),
        ("control_height", theme.control_height),
        ("control_height_small", theme.control_height_small),
        ("panel_header_height", theme.panel_header_height),
        ("tree_row_height", theme.tree_row_height),
        ("tree_indent_per_level", theme.tree_indent_per_level),
        ("tree_toggle_width", theme.tree_toggle_width),
        ("gap_standard", theme.gap_standard),
        ("padding_standard", theme.padding_standard),
        ("panel_inset", theme.panel_inset),
        ("panel_min_width", theme.panel_min_width),
        ("panel_max_width", theme.panel_max_width),
    ] {
        let multiple = value / step;
        assert!((multiple - multiple.round()).abs() < 1e-3 || (multiple * 1000.0).fract().abs() < 1.0, "theme.{field} = {value} is not a `--ui-spacing` multiple ({multiple})");
    }
    assert_eq!(theme.control_height_small, step * ui_styling::metrics::chrome::CONTROL_HEIGHT_SMALL_UI_SPACING as f32);
    assert_eq!(crate::wgpu::chrome::ICON_TINY, step * ui_styling::metrics::chrome::ICON_INLINE_UI_SPACING as f32);
}

#[test]
fn veil_is_the_dialog_surface_at_the_shared_veil_alpha() {
    for theme in [Theme::light(), Theme::dark()] {
        let veil = theme.veil(Level::Dialog);
        let surface = theme.surface(Level::Dialog);
        assert_eq!((veil.r, veil.g, veil.b), (surface.r, surface.g, surface.b));
        assert_eq!(veil.a, levels::VEIL_ALPHA as f32);
    }
    assert_eq!(Theme::veil_blur_px(), levels::VEIL_BLUR_PX as f32);
}

#[test]
fn spatial_axis_paints_are_the_brand_tokens() {
    for (axis, token) in [(0_u8, ui_styling::colors::PRIMARY), (1, ui_styling::colors::SECONDARY), (2, ui_styling::colors::TERTIARY)] {
        let paint = crate::wgpu::draw_types::gizmo::spatial_axis_rgba(axis, 1.0);
        assert_eq!(hex_of(paint), hex_of(Rgba::from_token(&token)), "axis {axis} must paint its brand token");
    }
}

//#region ⚫️MonoPremade
// ⚫️ W2k: the "mono" premade used to be 20 hand-written `Rgba::from_srgb8` literals in the os wgpu
// Shell, because mono's `chrome` group declared seven keys the default theme lacked and so could not
// share the generated `ChromePalette`. The key sets are reconciled and mono is now projected.

#[test]
fn the_mono_premade_is_a_real_theme_off_its_own_generated_palettes() {
    let light = Theme::mono(false);
    let dark = Theme::mono(true);
    assert_ne!(light.background, dark.background, "mono resolves both appearances");
    assert_ne!(light.background, Theme::light().background, "mono is not an alias of semio");
    assert_ne!(dark.background, Theme::dark().background);
    assert_eq!(light.background, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.base), "mono's floor is its OWN chrome.base, not a hand-resolved canvas");
    assert_eq!(dark.background, Rgba::from_token(&ui_styling::CHROME_MONO_DARK.base));
}

#[test]
fn the_mono_premade_shares_every_derived_surface_rule_with_the_default_theme() {
    let mono = Theme::mono(false);
    assert_eq!(mono.panel, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.panel), "panel follows the premade theme's semantic alias");
    assert_eq!(mono.navbar, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.level_window));
    assert_eq!(mono.temporary, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.level_menu));
    assert_eq!(mono.text_muted, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.muted_foreground), "the hand-port read hover_interactive_fill here — a wrong source");
    assert_eq!(mono.text_element, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.border_element));
    let semio = Theme::light();
    assert_eq!(mono.control_height, semio.control_height, "metrics are shared: a premade only recolors");
    assert_eq!(mono.font_size_body, semio.font_size_body);
    assert_eq!(mono.border_radius, semio.border_radius);
}

#[test]
fn every_appearance_group_carries_a_mono_twin_with_the_default_theme_s_own_shape() {
    assert_ne!(ui_styling::OUTCOME_MONO_LIGHT.error, ui_styling::OUTCOME_LIGHT.error, "mono recolors the outcome palette off its own grayscale tokens");
    assert_eq!(ui_styling::DIAGRAM_MONO_LIGHT.shape_outline.len(), 4, "a premade shares the generated struct, so even the four keys mono used to omit exist");
    assert_ne!(ui_styling::BOARD_MONO_LIGHT.node_stroke_computing, ui_styling::BOARD_LIGHT.node_stroke_computing, "the four board keys mono used to omit now resolve off mono's own palette");
    let muted = Theme::mono(false).muted;
    assert_ne!(muted, Theme::mono(false).separator, "`--muted` is its own token, not the separator stroke");
}
//#endregion ⚫️MonoPremade

//#region 🏠️ShellFloor
// 🏠️ W2k: React's `shellFloorPaints` (`🔨️modules/🏠️shell-floor-presentation/🟦️.ts:14`) — a base-level
// floor nested in a scope that is already base-and-painting drops to `bg-transparent`. The wgpu Shell
// painted the base colour twice per frame (frame setup + the main window) with no such predicate.

#[test]
fn a_base_floor_suppresses_its_fill_only_inside_an_already_painted_base_scope() {
    assert!(shell_floor_paints(None), "no enclosing scope at all still paints");
    assert!(!shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::Surface })), "base-and-painting is the one suppressed case");
    assert!(!shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::Glass })));
    assert!(!shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::Veil })));
    assert!(shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::None })), "a base scope that paints nothing leaves the floor to its descendant");
    for level in [Level::Window, Level::Pane, Level::Panel, Level::Dialog, Level::Menu] {
        assert!(shell_floor_paints(Some(SurfaceScope { level, fill: SurfaceFill::Surface })), "a deeper level is a different colour, so the floor still paints");
    }
}
//#endregion 🏠️ShellFloor

```
