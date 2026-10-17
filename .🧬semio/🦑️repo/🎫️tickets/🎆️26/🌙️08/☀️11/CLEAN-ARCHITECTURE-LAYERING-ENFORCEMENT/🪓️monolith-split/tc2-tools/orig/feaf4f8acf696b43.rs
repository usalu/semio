//! 🧩️ Drawing retained mutation laws over the candidate: variant coverage, image asset replay, hostile fields, container saturation and rebuild and reorder rollback.

use super::*;

#[test]
fn retained_drawing_mutation_candidate_covers_all_variants_and_returns_exact_owners() {
    let source = nested_snapshot();
    let group = semio_s_artifact_draw_drawing_model::layer_id(source.layers.last().expect("group")).clone();
    let (shape, boolean, trace) = match source.layers.last().expect("group") {
        DrawingLayerNode::Group(value) => (semio_s_artifact_draw_drawing_model::layer_id(&value.children[0]).clone(), semio_s_artifact_draw_drawing_model::layer_id(&value.children[1]).clone(), semio_s_artifact_draw_drawing_model::layer_id(&value.children[2]).clone()),
        _ => unreachable!("Drawing fixture group remains exact"),
    };
    let mutations = vec![
        semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation(group.clone(),true),
        DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: shape.clone(), visible: false }),
        DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: shape.clone(), locked: true }),
        DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: shape.clone(), opacity: 0.5 }),
        DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id:shape.clone(),fill_rule:crate::FillRule::Nonzero}),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: shape.clone(), blend_mode: "multiply".into() }),
        DrawingMutation::RenameLayer(RenameLayer { layer_id: shape.clone(), new_name: "Renamed".into() }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: shape.clone(), transform: semio_s_artifact_draw_drawing_model::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5, shear: 0.75 } }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill {
            layer_id: shape.clone(),
            fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }].into() }),
        }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: shape.clone(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Bevel, dash: Some(vec![1.0, 2.0].into()) }) }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: boolean, boolean_operation: "difference".into() }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: trace, params: semio_s_artifact_draw_drawing_model::DrawingTraceParams { threshold: 0.4, simplify_epsilon: 1.2 } }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group.clone()), index: Some(1), layer: Box::new(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Created")).to_string().into()).expect("nonempty authored identity"), "Created", vec![PathSegment::Move { to: [0.0, 0.0] }].into())) }),
        DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&shape), layer_id: shape.clone() }),
        DrawingMutation::DeleteLayer(DeleteLayer { layer_id: shape.clone() }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: shape, parent_id: None, index: 0 }),
    ];
    for mutation in mutations {
        let value = apply(nested_snapshot(), &mutation).expect("retained Drawing mutation applies");
        drain_snapshot(value);
    }
    drain_snapshot(source);
}

#[test]
fn retained_image_asset_mutations_replay_sparse_delta_and_reference_refusal(){
    let before:DrawingSnapshot=serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after:DrawingSnapshot=serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/➡️after/🔣️.json")).unwrap();
    let mutation=semio_s_artifact_draw_drawing_structure::import_image_asset::mutation::import_image_asset("bitmap".into(),after.assets.get("bitmap").unwrap().clone());let imported=apply(before.clone(),&mutation).unwrap();assert_eq!(imported,after);drain_mutation(mutation);
    let removal=semio_s_artifact_draw_drawing_structure::remove_image_asset::mutation::remove_image_asset("bitmap".into());let restored=apply(imported,&removal).unwrap();assert_eq!(restored,before);drain_snapshot(restored);
    let mut referenced=after.clone();let mut image=semio_s_artifact_draw_drawing_model::create_drawing_image_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image","bitmap");semio_s_artifact_draw_drawing_model::layer_base_mut(&mut image).id="image".into();referenced.layers.push(image);let expected=referenced.clone();let (rejected,error)=apply(referenced,&removal).unwrap_err();assert_eq!(error,"drawing-store.asset-still-referenced");assert_eq!(rejected,expected);drain_snapshot(rejected);
    let mut referenced=after.clone();let mut trace=semio_s_artifact_draw_drawing_model::create_drawing_trace_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace","bitmap");semio_s_artifact_draw_drawing_model::layer_base_mut(&mut trace).id="trace".into();referenced.layers.push(trace);let expected=referenced.clone();let (rejected,error)=apply(referenced,&removal).unwrap_err();assert_eq!(error,"drawing-store.asset-still-referenced");assert_eq!(rejected,expected);drain_snapshot(rejected);drain_mutation(removal);drain_snapshot(before);drain_snapshot(after);
    eprintln!("[DEBUG] Retained native image asset import/remove exact neutral delta and unchanged image/trace reference refusals");
}

#[test]
fn retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty() {
    let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("leaf")).to_string().into()).expect("nonempty authored identity"), "leaf", Default::default());
    for depth in 0..=DRAWING_MAXIMUM_LAYER_DEPTH {
        let mut parent = semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit(((&format!("depth-{depth}"))).to_string().into()).expect("nonempty authored identity"), &format!("depth-{depth}"));
        if let DrawingLayerNode::Group(value) = &mut parent {
            value.children.push(layer);
        }
        layer = parent;
    }
    let mut source = semio_s_artifact_draw_drawing_model::default_drawing_document("drawing-depth-plus-one", None);
    source.layers = vec![layer].into();
    let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "missing".into(), visible: false });
    let (source, error) = apply(source, &mutation).expect_err("retained Drawing depth +1 authority rejects");
    assert_eq!(error, "drawing-store.preflight-depth-capacity");
    drain_snapshot(source);

    let source = nested_snapshot();
    let mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1), new_name: "hostile".into() });
    let (source, _) = apply(source, &mutation).expect_err("hostile Drawing field rejects");
    drain_snapshot(source);
}

#[test]
fn retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner() {
    let mut snapshot = semio_s_artifact_draw_drawing_model::default_drawing_document("rebuild-reservation", None);
    snapshot.layers = vec![semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("first")).to_string().into()).expect("nonempty authored identity"), "first", Default::default()), semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("second")).to_string().into()).expect("nonempty authored identity"), "second", Default::default())].into();
    let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("pending")).to_string().into()).expect("nonempty authored identity"), "pending", Default::default())) });
    let workset = live_workset(&mut snapshot, &mutation).expect("live Drawing rebuild workset admitted");
    let source = std::mem::take(&mut snapshot.layers);
    let DrawingMutation::CreateLayer(mut create) = mutation else { unreachable!() };
    let pending = *std::mem::replace(&mut create.layer, Box::new(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("retired-placeholder")).to_string().into()).expect("nonempty authored identity"), "retired-placeholder", Default::default())));
    drain_mutation(DrawingMutation::CreateLayer(create));
    drain_snapshot(snapshot);
    let mut reverse = Vec::new();
    let mut output = Vec::new();
    reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing reverse arena");
    output.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing output arena");
    let source_owner = native_layer_backing(&source);
    let reverse_owner = reverse.as_ptr();
    let output_owner = output.as_ptr();
    let mut authority = DrawingContainerRebuildAuthority::new(source, Some(0), Some(1), Some(pending), reverse, output, workset).unwrap_or_else(|_| panic!("fixed Drawing rebuild admitted"));
    assert!(authority.take().is_none(), "false terminal cannot expose a partially rebuilt owner");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..3 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(8_005),
            semio_framework_job::Generation(85),
            semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut context_retained,
        );
        assert!(!authority.step(&mut context).expect("Drawing rebuild advances before interruption"));
    }
    let mut rollback_turns = 0;
    while !authority.rollback_step().expect("Drawing rebuild rollback advances one exact owner") {
        rollback_turns += 1;
    }
    assert_eq!(rollback_turns, authority.move_count, "one recorded owner move rolls back per close grant");
    let restored = authority.source.take().expect("original Drawing source owner returns");
    let pending = authority.pending.take().expect("pending Drawing owner returns");
    let reverse = authority.reverse.take().expect("reverse Drawing scratch owner returns");
    let output = authority.output.take().expect("output Drawing scratch owner returns");
    assert_eq!(native_layer_backing(&restored), source_owner);
    assert_eq!(reverse.as_ptr(), reverse_owner);
    assert_eq!(output.as_ptr(), output_owner);
    assert!(authority.removed.is_none());
    authority.finish_handoff().expect("Drawing rebuild rollback reaches exact terminal handoff");
    assert!(authority.terminal_is_empty());
    drop(authority);
    drop(reverse);
    drop(output);
    drain_mutation(DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(pending) }));
    let mut restored_snapshot = semio_s_artifact_draw_drawing_model::default_drawing_document("restored-rebuild", None);
    restored_snapshot.layers = restored;
    drain_snapshot(restored_snapshot);
}

#[test]
fn retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot() {
    for phase in 0..=3 {
        for stale in [false, true] {
            let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing rollback pool admits exact owners");
            let mut source = semio_s_artifact_draw_drawing_model::default_drawing_document("rebuild-rollback", None);
            reserve_native_layer_slots(&mut source.layers, DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY);
            for index in 0..3 {
                source.layers.push(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit(((&format!("source-{index}"))).to_string().into()).expect("nonempty authored identity"), &format!("source-{index}"), Default::default()));
            }
            let source_owner = native_layer_backing(&source.layers);
            let source_ids: Vec<_> = source.layers.iter().map(|layer| semio_s_artifact_draw_drawing_model::layer_id(layer).to_string()).collect();
            let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("pending")).to_string().into()).expect("nonempty authored identity"), "pending", Default::default())) });
            let operation = semio_framework_job::OperationId(8_500 + phase as u64);
            let generation = semio_framework_job::Generation(850 + phase as u64);
            let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing rollback candidate borrows one exact pool slot");
            let slot = authority.arena_slot;
            let arena_generation = authority.arena_generation;
            let reverse_owner = authority.container_reverse.as_ref().expect("Drawing rollback reverse owner").as_ptr();
            let output_owner = authority.container_output.as_ref().expect("Drawing rollback output owner").as_ptr();
            let page_catalog_owner = authority.overlay_pages.as_ref().expect("Drawing rollback page catalog owner").as_ptr();
            let page_owners: [usize; DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY] = std::array::from_fn(|index| authority.overlay_pages.as_ref().expect("Drawing rollback page owner")[index].as_ptr() as usize);
            let duplicate_owner = authority.duplicate_id_owner.as_ref().expect("Drawing rollback duplicate owner").as_ptr();
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)) {
                    break;
                }
                let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing rollback fixture reaches every internal rebuild phase"));
            }
            assert!(authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)));
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
                &mut rejected_retained,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
            close_candidate(&mut authority, Some(&mut source));
            assert_eq!(native_layer_backing(&source.layers), source_owner, "rollback restores the exact original live native page backing");
            assert_eq!(source.layers.iter().map(|layer| semio_s_artifact_draw_drawing_model::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "rollback restores exact FIFO layer order");
            drop(authority);

            let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 100), semio_framework_job::Generation(generation.0 + 100), pool.clone())
                .expect("rolled-back Drawing pool slot re-admits immediately");
            assert_eq!(reused.arena_slot, slot);
            assert!(reused.arena_generation > arena_generation);
            assert_eq!(reused.container_reverse.as_ref().expect("returned reverse owner").as_ptr(), reverse_owner);
            assert_eq!(reused.container_output.as_ref().expect("returned output owner").as_ptr(), output_owner);
            assert_eq!(reused.overlay_pages.as_ref().expect("returned page catalog owner").as_ptr(), page_catalog_owner);
            assert_eq!(std::array::from_fn::<_, DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY, _>(|index| reused.overlay_pages.as_ref().expect("returned page owner")[index].as_ptr() as usize), page_owners);
            assert_eq!(reused.duplicate_id_owner.as_ref().expect("returned duplicate owner").as_ptr(), duplicate_owner);
            close_candidate(&mut reused, None);
            drop(reused);
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }
}

#[test]
fn retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots() {
    for stale in [false, true] {
        let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing reorder rollback pool admits exact owners");
        let mut source = nested_snapshot();
        let (group_id, target, source_owner, source_ids) = match source.layers.last_mut().expect("Drawing reorder rollback group") {
            DrawingLayerNode::Group(group) => {
                reserve_native_layer_slots(&mut group.children, DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY);
                (group.base.id.clone(), semio_s_artifact_draw_drawing_model::layer_id(&group.children[0]).clone(), native_layer_backing(&group.children), group.children.iter().map(|layer| semio_s_artifact_draw_drawing_model::layer_id(layer).to_string()).collect::<Vec<_>>())
            }
            _ => unreachable!("Drawing reorder rollback fixture remains a group"),
        };
        let mutation = DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 });
        let operation = semio_framework_job::OperationId(8_700);
        let generation = semio_framework_job::Generation(870);
        let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing reorder rollback candidate borrows one exact pool slot");
        let slot = authority.arena_slot;
        let reverse_owner = authority.container_reverse.as_ref().expect("Drawing reorder reverse owner").as_ptr();
        let output_owner = authority.container_output.as_ref().expect("Drawing reorder output owner").as_ptr();
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            if authority.rebuild_role == Some(DrawingContainerRebuildRole::Destination) && authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(2)) {
                break;
            }
            let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
            assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing reorder rollback fixture reaches destination rebuild after source handoff"));
        }
        assert_eq!(authority.rebuild_role, Some(DrawingContainerRebuildRole::Destination));
        assert!(authority.source_undo.is_some(), "source handoff keeps the exact insertion undo authority until destination publication");
        if !stale {
            cancel.cancel_now();
        }
        let mut rejected_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut rejected = semio_framework_job::StepContext::new(
            operation,
            if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
            semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT),
            cancel,
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut rejected_retained,
        );
        assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
        close_candidate(&mut authority, Some(&mut source));
        drop(authority);
        let DrawingLayerNode::Group(group) = source.layers.last().expect("Drawing reorder rollback group remains retained") else { unreachable!("Drawing reorder rollback group remains a group") };
        assert_eq!(native_layer_backing(&group.children), source_owner, "source undo restores the exact nested live native page backing");
        assert_eq!(group.children.iter().map(|layer| semio_s_artifact_draw_drawing_model::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "source undo restores the exact nested FIFO order");

        let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 1), semio_framework_job::Generation(generation.0 + 1), pool).expect("reorder rollback returns the exact pool slot");
        assert_eq!(reused.arena_slot, slot);
        assert_eq!(reused.container_reverse.as_ref().expect("returned reorder reverse owner").as_ptr(), reverse_owner);
        assert_eq!(reused.container_output.as_ref().expect("returned reorder output owner").as_ptr(), output_owner);
        close_candidate(&mut reused, None);
        drop(reused);
        drain_mutation(mutation);
        drain_snapshot(source);
    }
}
