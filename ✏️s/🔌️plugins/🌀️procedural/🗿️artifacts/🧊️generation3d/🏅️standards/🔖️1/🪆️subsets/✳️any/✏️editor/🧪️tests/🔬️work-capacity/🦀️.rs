use super::*;

#[test]
fn generation3d_declares_one_work_capacity_for_extent_footprint_and_preflight() {
    let capacity = GENERATION3D_RETAINED_CAPACITY;
    assert_eq!(capacity.work_items(), GENERATION3D_RETAINED_WORK_ITEMS, "the preflight ceiling IS the declared capacity");
    assert_eq!(capacity.rows_for_items(1), Some(store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS), "one durable item is one forward row plus one inverse row");
    assert_eq!(generation3d_one_item_footprint(0).work_items, capacity.rows_for_items(1).expect("one item fits the route capacity"), "the footprint a preflight declares and the extent a work answers are the SAME quantity");
    assert!(capacity.admits(generation3d_one_item_footprint(0).work_items), "a route whose footprint is N rows must admit an extent of N");
    eprintln!(
        "generation3d work capacity: items={} work_items={} one-item rows={} footprint rows={}",
        capacity.invertible_items(),
        capacity.work_items(),
        capacity.rows_for_items(1).expect("one item fits"),
        generation3d_one_item_footprint(0).work_items
    );
}

#[semio_framework_async_macros::async_test]
async fn every_bounded_retained_route_answers_an_admissible_extent() {
    let _serial = crate::test_serial::lock();
    let read = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(<Generation3dPlayApp as ArtifactEditor>::initial_snapshot());
    let interaction = protocol::InteractionState::default();
    let capacity = GENERATION3D_RETAINED_CAPACITY;
    let instance_owner = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(<Generation3dPlayApp as ArtifactEditor>::build_instance_operation_owner());
    for tool_id in GENERATION3D_RETAINED_TOOL_IDS.iter().chain(GENERATION3D_FLOW_EVAL_TOOL_IDS).copied() {
        if tool_id == "flowEvalTick" {
            continue;
        }
        let args = (tool_id == "setWidgetInput").then(|| dsl::DslValue::object([
            ("widgetId".into(), dsl::DslValue::String("shape".into())),
            ("channel".into(), dsl::DslValue::String("width".into())),
            ("value".into(), dsl::DslValue::String("2".into())),
        ]));
        let command = <Generation3dPlayApp as ArtifactEditor>::command_from_action(tool_id, args.as_ref()).unwrap_or_else(|_| panic!("{tool_id} decodes from its own action id"));
        let extent = if GENERATION3D_PREVIEW_TOOL_IDS.contains(&tool_id) {
            Generation3dPreviewCommandWork::new(tool_id, instance_owner.clone()).extent(&command, &read, &interaction, None)
        } else {
            generation3d_bounded_extent(&command, &read, &interaction)
        };
        let extent = extent.unwrap_or_else(|| panic!("{tool_id}: extent refused the command outright — preflight reports that as a capacity fault"));
        assert!(capacity.admits(extent), "{tool_id}: extent {extent} exceeds the declared capacity {}", capacity.work_items());
    }
    let mut closed = false;
    for _ in 0..1_000_000 {
        if instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| Ok(matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)))).expect("the owner lends its close ladder") {
            closed = true;
            break;
        }
    }
    assert!(closed, "the extent fixture owner must close terminal-empty");
}
