//! 🛠️ Laws of the transform tool itself, driven dispatch by dispatch exactly as the retained step threads it: a
//! one-shot is ONE transaction, a streamed gesture accumulates ONE net leaf in its open transaction until its commit,
//! every host abort leaves zero trace, and the persisted state round-trips through the window transient's value form.

#[allow(unused_imports)] use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
#[allow(unused_imports)] use semio_plugin::Emit;
#[allow(unused_imports)] use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transform::*;

fn document() -> LayoutSnapshot {
    semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document()
}

fn drag(dx: f64, dy: f64) -> LayoutFrameRecord {
    LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Drag { dx, dy } }
}

fn step(phase: LayoutTransformPhase, record: Option<LayoutFrameRecord>, open: Option<&LayoutTransformToolState>, revision: &str) -> LayoutTransformDispatch {
    layout_transform_dispatch("translateSelection", phase, record, &document(), open, "seed-1", revision, Some("locked"))
}

fn opened(dispatch: LayoutTransformDispatch) -> LayoutTransformToolState {
    *dispatch.tool.expect("the transient changed").expect("the gesture is open")
}

#[test]
fn transform_utility_id_and_options() {
    assert_eq!(UTILITY_ID, "transform");
    assert_eq!(definition().id, UTILITY_ID);
    let options = options();
    assert!(options.move_axes && options.rotate && options.scale_axes && options.scale_uniform);
}

/// ➕️ Ticks compose into one net transform: offsets and angles add, factors multiply — only about one pivot and over
/// the same targets.
#[test]
fn ticks_compose_into_one_net_transform() {
    assert_eq!(drag(1.0, 2.0).then(&drag(3.0, -1.0)), Some(drag(4.0, 1.0)));
    let turn = |angle: f64, pivot: f64| LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Rotate { pivot_x: pivot, pivot_y: pivot, angle } };
    assert_eq!(turn(0.25, 5.0).then(&turn(0.5, 5.0)).map(|record| record.motion), Some(LayoutFrameMotion::Rotate { pivot_x: 5.0, pivot_y: 5.0, angle: 0.75 }));
    assert_eq!(turn(0.25, 5.0).then(&turn(0.5, 6.0)), None, "another pivot is another gesture");
    let scaling = |sx: f64| LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, sx, sy: 1.0 } };
    assert_eq!(scaling(2.0).then(&scaling(1.5)).map(|record| record.motion), Some(LayoutFrameMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, sx: 3.0, sy: 1.0 }));
    assert_eq!(drag(1.0, 0.0).then(&scaling(2.0)), None, "another kind is another gesture");
}

/// ✋️ A one-shot is ONE committed transaction: the parametric leaf, the tool `<appId>#<verb>`, a `tx-` id minted from
/// the admission's seed, no coalesce key, and no window state.
#[test]
fn a_one_shot_is_one_transaction() {
    let dispatch = step(LayoutTransformPhase::Once, Some(drag(5.0, -2.0)), None, "rev-1");
    assert!(dispatch.tool.is_none(), "a one-shot leaves the window transient alone");
    let transaction = dispatch.emit.transaction.clone().expect("the commit carries its transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.layout.layout@1/*#editor#translateSelection", "{transaction:?}");
    assert_eq!(dispatch.emit.artifact_mutations, vec![drag(5.0, -2.0).mutation()]);
}

/// 🌊️ A streamed gesture: every tick lands in ONE open transaction the window persists — no edit — and the commit
/// publishes the net leaf under the ref minted at the first tick, clearing the window state.
#[test]
fn a_stream_publishes_nothing_until_its_commit_publishes_one_transaction() {
    let first = step(LayoutTransformPhase::Stream, Some(drag(10.0, 0.0)), None, "rev-1");
    assert!(first.emit.artifact_mutations.is_empty() && first.emit.transaction.is_none(), "a tick is no edit");
    let open = opened(first);
    let second = opened(step(LayoutTransformPhase::Stream, Some(drag(15.0, 5.0)), Some(&open), "rev-1"));
    assert_eq!(second.transaction, open.transaction, "every tick rides the one open transaction");
    assert_eq!(second.entries.len(), 1, "the open transaction holds ONE net leaf");
    let commit = step(LayoutTransformPhase::Commit, Some(drag(0.0, 0.0)), Some(&second), "rev-1");
    assert_eq!(commit.emit.artifact_mutations, vec![drag(25.0, 5.0).mutation()], "the commit publishes the net leaf");
    assert_eq!(commit.emit.transaction, Some(open.transaction), "under the ref minted when the gesture opened");
    assert_eq!(commit.tool, Some(None), "the commit clears the persisted gesture");
}

/// 🧯️ Every host abort of an open gesture — and a document that moved under it — publishes no edit and clears it; a
/// commit, an abort or a stray tail with no open gesture leaves zero trace.
#[test]
fn every_host_abort_leaves_zero_trace() {
    let open = opened(step(LayoutTransformPhase::Stream, Some(drag(10.0, 0.0)), None, "rev-1"));
    for reason in ToolAbortReason::ALL {
        let aborted = step(LayoutTransformPhase::Abort(reason), None, Some(&open), "rev-1");
        assert!(aborted.emit.artifact_mutations.is_empty() && aborted.emit.transaction.is_none(), "{reason:?}: no edit");
        assert_eq!(aborted.tool, Some(None), "{reason:?}: the gesture is cleared");
    }
    let moved = step(LayoutTransformPhase::Commit, Some(drag(1.0, 0.0)), Some(&open), "rev-2");
    assert!(moved.emit.artifact_mutations.is_empty(), "a gesture on a moved base commits nothing");
    assert_eq!(moved.tool, Some(None), "baseMoved clears the gesture");
    for phase in [LayoutTransformPhase::Commit, LayoutTransformPhase::Abort(ToolAbortReason::Blur)] {
        let stray = step(phase, Some(drag(3.0, 0.0)), None, "rev-1");
        assert!(stray.emit.artifact_mutations.is_empty() && stray.tool.is_none(), "{phase:?} at rest leaves zero trace");
    }
}

/// 🔀️ A one-shot or another verb interrupting an open gesture aborts it (`captureLost`): only the interrupting
/// transform counts, under a fresh ref minted from its own admission's seed.
#[test]
fn an_interruption_aborts_the_open_gesture() {
    let open = opened(step(LayoutTransformPhase::Stream, Some(drag(10.0, 0.0)), None, "rev-1"));
    let once = layout_transform_dispatch("translateSelection", LayoutTransformPhase::Once, Some(drag(1.0, 0.0)), &document(), Some(&open), "seed-2", "rev-1", None);
    assert_eq!(once.emit.artifact_mutations, vec![drag(1.0, 0.0).mutation()], "the interrupted gesture contributed nothing");
    assert_ne!(once.emit.transaction.as_ref().map(|transaction| &transaction.id), Some(&open.transaction.id));
    assert_eq!(once.tool, Some(None));
    let turn = LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Rotate { pivot_x: 0.0, pivot_y: 0.0, angle: 0.5 } };
    let rotated = layout_transform_dispatch("rotateSelection", LayoutTransformPhase::Stream, Some(turn), &document(), Some(&open), "seed-3", "rev-1", None);
    let reopened = opened(rotated);
    assert_eq!((reopened.verb.as_str(), reopened.entries.len()), ("rotateSelection", 1), "another verb's tick opens its own gesture");
    assert_ne!(reopened.transaction.id, open.transaction.id);
}

/// 🔒️ A request whose every target is locked raises the one localized notice and leaves zero trace.
#[test]
fn a_fully_locked_request_raises_one_notice() {
    let mut base = document();
    if let semio_s_artifact_layout_layout_model::Frame::Rect { locked, .. } = base.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").expect("demo rect") {
        *locked = Some(true);
    }
    let dispatch = layout_transform_dispatch("translateSelection", LayoutTransformPhase::Once, Some(drag(5.0, 0.0)), &base, None, "seed-1", "rev-1", Some("locked"));
    assert!(dispatch.emit.artifact_mutations.is_empty());
    assert!(matches!(dispatch.emit.effects.as_slice(), [semio_framework::kernel::Effect::Notify { message }] if message == "locked"), "one localized notice");
}

/// 💾️ The persisted gesture survives the window transient's value form and resumes as the same open gesture; the
/// window previews its provisional leaf, never the document.
#[test]
fn the_persisted_gesture_round_trips_and_previews() {
    let open = opened(step(LayoutTransformPhase::Stream, Some(drag(10.0, 4.0)), None, "rev-1"));
    let restored: LayoutTransformToolState = semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&open)).expect("the tool state decodes from its value form");
    assert_eq!(restored, open);
    let tool = LayoutTransformTool::resume(&restored).expect("the gesture resumes");
    assert!(!tool.at_rest(), "a resumed gesture is still streaming");
    let base = document();
    let preview = layout_transform_tool_preview(&base, &restored).expect("the gesture changes a frame");
    let frame = base.pages[0].frames.iter().position(|frame| frame.id() == "frame-1").expect("demo rect");
    let original = base.pages[0].frames[frame].bounds();
    let bounds = preview.bounds(&base, 0, frame).expect("the preview applies the provisional leaf");
    assert_eq!((bounds.x, bounds.y), (original.x + 10.0, original.y + 4.0));
    assert_eq!(original, document().pages[0].frames[frame].bounds(), "the document never moves while the gesture is open");
}

#[test]
fn portable_gesture_finishes_publish_only_the_eligible_net_transform() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let record = |raw: &serde_json::Value, page: &str, target: &str, pivot: f64| {
        let value = raw[1].as_f64().unwrap();
        let motion = match raw[0].as_str().unwrap() {
            "drag" => LayoutFrameMotion::Drag { dx: value, dy: 0.0 },
            "rotate" => LayoutFrameMotion::Rotate { pivot_x: pivot, pivot_y: 0.0, angle: value },
            "scale" => LayoutFrameMotion::Scale { pivot_x: pivot, pivot_y: 0.0, sx: value, sy: 1.0 },
            other => panic!("{other}"),
        };
        LayoutFrameRecord { page_id: page.into(), targets: vec![target.into()], motion }
    };
    for case in fixture["cases"].as_array().unwrap() {
        let open = opened(step(LayoutTransformPhase::Stream, Some(record(&case["open"], "page-1", "frame-1", 0.0)), None, "rev-1"));
        let tail = (!case["tail"].is_null()).then(|| record(
            &case["tail"], case["page"].as_str().unwrap_or("page-1"), case["target"].as_str().unwrap_or("frame-1"), case["pivot"].as_f64().unwrap_or(0.0),
        ));
        let mut base = document();
        if case["missing"].as_bool() == Some(true) {
            base.pages[0].frames.retain(|frame| frame.id() != "frame-1");
        }
        if case["locked"].as_bool() == Some(true) {
            let frame = base.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap();
            let semio_s_artifact_layout_layout_model::Frame::Rect { locked, .. } = frame else { panic!("demo rect") };
            *locked = Some(true);
        }
        let revision = case["revision"].as_str().unwrap_or("rev-1");
        let phase = LayoutTransformPhase::parse(case["phase"].as_str(), None).unwrap();
        let dispatch = layout_transform_dispatch("translateSelection", phase, tail, &base, Some(&open), "seed-1", revision, Some("locked"));
        let expected = case["committed"].as_f64().map(|value| {
            record(&serde_json::json!([case["open"][0], value]), "page-1", "frame-1", 0.0).mutation()
        }).into_iter().collect::<Vec<_>>();
        assert_eq!(dispatch.emit.artifact_mutations, expected, "{}", case["name"]);
        assert_eq!(dispatch.emit.transaction, (!expected.is_empty()).then_some(open.transaction), "{}", case["name"]);
        assert_eq!(dispatch.tool, Some(None), "{}", case["name"]);
    }
}

#[test]
fn a_rotate_finish_whose_target_has_no_pivot_clears_the_gesture() {
    use semio_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig};
    use semio_s_artifact_layout_layout_editor_commands::gumball::{transform, RotateSelection};

    let base = document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&base, &history);
    let config = ConfigView { snapshot: &NoConfig {}, window: None };
    let first = RotateSelection { ids: vec!["frame-1".into()], angle: 0.5, phase: Some("stream".into()), reason: None };
    let open = opened(transform(first.verb(), &doc, &config, &[], None, None));
    let stale = RotateSelection { ids: vec!["removed-frame".into()], angle: 0.0, phase: Some("commit".into()), reason: None };
    let finished = transform(stale.verb(), &doc, &config, &[], Some(&open), None);
    assert!(finished.emit.artifact_mutations.is_empty());
    assert!(finished.emit.transaction.is_none());
    assert_eq!(finished.tool, Some(None));
}

#[test]
fn sparse_transform_preview_matches_committed_bounds_and_preserves_live_inputs() {
    use semio_os_kernel::{ColdMutation, Mutation};
    use semio_s_artifact_layout_layout_model::LayoutBounds;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let base=fixture["previewBase"].as_array().unwrap();
    for case in fixture["previews"].as_array().unwrap() {
        let mut doc=document();
        let original=LayoutBounds{x:base[0].as_f64().unwrap(),y:base[1].as_f64().unwrap(),width:base[2].as_f64().unwrap(),height:base[3].as_f64().unwrap(),rotation:base[4].as_f64().unwrap()};
        let first=&mut doc.pages[0].frames[0];
        match first {
            semio_s_artifact_layout_layout_model::Frame::Rect{bounds,locked,..}
            |semio_s_artifact_layout_layout_model::Frame::Text{bounds,locked,..}
            |semio_s_artifact_layout_layout_model::Frame::Image{bounds,locked,..}=>{
                *bounds=original.clone();
                *locked=case["locked"].as_bool();
            }
        }
        let frame=first.id().to_owned();
        let motion=case["motion"].as_array().unwrap();
        let a=motion[1].as_f64().unwrap();
        let b=motion[2].as_f64().unwrap();
        let motion=match motion[0].as_str().unwrap(){
            "drag"=>LayoutFrameMotion::Drag{dx:a,dy:b},
            "rotate"=>LayoutFrameMotion::Rotate{pivot_x:0.0,pivot_y:0.0,angle:a},
            "scale"=>LayoutFrameMotion::Scale{pivot_x:0.0,pivot_y:0.0,sx:a,sy:b},
            _=>unreachable!(),
        };
        let record=LayoutFrameRecord{page_id:doc.pages[0].id.clone(),targets:vec![frame],motion};
        let state=opened(step(LayoutTransformPhase::Stream,Some(drag(1.0,0.0)),None,"rev-1"));
        let mut state=state;
        state.entries[0].mutation=semio_framework_value::ToValue::to_value(&record.mutation());
        let preview=layout_transform_tool_preview(&doc,&state);
        assert_eq!(doc.pages[0].frames[0].bounds(),&original);
        if let Some(bounds)=case["bounds"].as_array(){
            let preview=preview.as_ref().expect("typed sparse bounds");
            let result=preview.bounds(&doc,0,0).expect("only the selected frame changes");
            assert!(preview.bounds(&doc,0,1).is_none());
            assert!(preview.bounds(&doc.clone(),0,0).is_none());
            let commit=record.mutation().diff(&doc);
            let committed=semio_os_kernel::apply_diff_cold(commit.diff(),&doc).unwrap();
            assert_eq!(result,committed.pages[0].frames[0].bounds());
            let resolved=semio_s_artifact_layout_layout_model::resolve_page_with_preview(
                &doc,&doc.pages[0],Some(preview),|id,bounds|id==doc.pages[0].frames[0].id()&&bounds==result);
            assert_eq!(resolved.len(),1);
            assert_eq!(resolved[0].frame_index,0);
            assert_eq!(resolved[0].frame.bounds(),result);
            for (actual,expected) in [result.x,result.y,result.width,result.height,result.rotation].into_iter().zip(bounds){
                assert!((actual-expected.as_f64().unwrap()).abs()<1e-10);
            }
        }else{
            assert!(preview.is_none());
        }
    }
}
