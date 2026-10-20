use super::*;
use semio_s_artifact_procedural_3d_geometry_value::*;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{retained_clone::{RetainedCloneGrant}, retirement::controlled::ControlledRetirement};

fn wire_shape(law: &serde_json::Value) -> GeometryValue {
    use {semio_framework_3d_brep_representation::topology::{Body, Vertex}, semio_framework_3d_brep_vector::Pnt3, semio_framework_3d_brep_tolerance::Tol};
    use semio_framework_3d_brep_engine::{ShapeRoot,ShapeWire};
    let mut body=Body::new();let label=body.labels.next_label();let point=&law["cancellation"]["wirePoint"];
    let id=body.vertices.insert(Vertex{position:Pnt3{x:point[0].as_f64().unwrap(),y:point[1].as_f64().unwrap(),z:point[2].as_f64().unwrap()},tol:Tol::default(),label});
    let shape=ShapeValue{root:ShapeRoot::Wire(ShapeWire{members:Vec::new(),vertices:vec![id],closed:false,label}),body};shape.check().unwrap();GeometryValue::shape(shape)
}

fn clock() -> Option<u64> { Some(1) }
fn law() -> serde_json::Value { serde_json::from_str(include_str!("🧫️.json")).unwrap() }
fn grant(law: &serde_json::Value) -> RetainedCloneGrant {
    let c = &law["originalCaller"];
    RetainedCloneGrant { maximum_items:c["items"].as_u64().unwrap() as usize, maximum_copy_bytes:c["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes:c["capacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes:c["releaseBytes"].as_u64().unwrap() as usize, maximum_depth:c["depth"].as_u64().unwrap() as usize }
}

#[test]
fn original_widget_output_moves_once_on_same_caller_and_retires_all_fields() {
    let law=law();let stock=grant(&law);let c=&law["originalCaller"];
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());
    let (evaluation,input_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||WidgetEvaluation::ok(law["ports"].as_array().unwrap().iter().map(|port|(port.as_str().unwrap().to_string(),GeometryValue::Text(law["text"].as_str().unwrap().to_string()))).collect(),Quality::ExactAnalytic));
    assert_eq!(input_heap.released_bytes,0);
    let pointer=match evaluation.outputs.get("alpha").unwrap(){GeometryValue::Text(text)=>text.as_ptr(),_=>unreachable!()};
    assert_eq!(evaluation.outputs.iter().map(|(port,_)|port.as_str()).collect::<Vec<_>>(),law["expectedPorts"].as_array().unwrap().iter().map(|port|port.as_str().unwrap()).collect::<Vec<_>>());
    let mut original=Some(evaluation);
    for denied in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,..Default::default()}] {
        let before=caller.cumulative_receipt();let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|ReadyJob::admit(&mut original,cx)));
        assert!(result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(original.is_some());assert_eq!(caller.cumulative_receipt(),before);
    }
    let handoff=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(handoff,|cx|ReadyJob::admit(&mut original,cx)));
    let mut job=result.unwrap().unwrap();assert!(original.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for denied in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,..Default::default()}] {
        let before=caller.cumulative_receipt();let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|job.step_on_original(cx)));
        assert!(result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.cumulative_receipt(),before);
    }
    let before=caller.cumulative_receipt();let mut denied_output=None;
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|cx.with_original_work_loan(0,cx.deadline_us(),|cx|{denied_output=job.step_on_original(cx)?;Ok(())})));
    result.unwrap();assert!(denied_output.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.cumulative_receipt(),before);
    let mut foreign=OriginalJobFixtureCaller::admit(OperationId(742),Generation(3),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(stock,|cx|Ok(job.step_on_original(cx))));
    assert!(result.unwrap().is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));foreign.retire(stock);
    let before=caller.cumulative_receipt();let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(handoff,|cx|job.step_on_original(cx)));
    let WidgetStep::Done(evaluation)=result.unwrap().unwrap() else {panic!("single original Ready output")};
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.cumulative_receipt().copied_items-before.copied_items,1);
    assert_eq!(caller.cumulative_receipt().copied_bytes,before.copied_bytes);
    assert_eq!(match evaluation.outputs.get("alpha").unwrap(){GeometryValue::Text(text)=>text.as_ptr(),_=>unreachable!()},pointer);
    assert!(caller.turn(handoff,|cx|job.step_on_original(cx)).unwrap().is_none());
    let mut owner=ControlledRetirement::new((job,evaluation)).ok().unwrap();let mut born=0;let mut released=0;
    for _ in 0..10000 {
        if owner.terminal_is_empty(){break;}
        let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{if cx.should_yield(){return Ok(None)}let step=owner.step(cx.retained_grant())?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(Some(step))}));
        let step=step.unwrap().unwrap();let receipt=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
    }
    assert!(owner.terminal_is_empty());assert_eq!(input_heap.requested_bytes+born,released);
    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    caller.retire(stock);
}

#[test]
fn original_widget_output_cancel_keeps_nested_payload_for_same_caller_retirement() {
    let law=law();let stock=grant(&law);let c=&law["originalCaller"];
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());
    let (evaluation,input_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||WidgetEvaluation::ok(outputs([("nested",GeometryValue::List(vec![GeometryValue::Text(law["cancellation"]["text"].as_str().unwrap().into()),GeometryValue::Selection(SelectionValue{component:SelectionKind::Face,ids:law["cancellation"]["selectionIds"].as_array().unwrap().iter().map(|id|id.as_u64().unwrap()).collect()}),GeometryValue::mesh(HalfedgeMesh::empty()),wire_shape(&law)]))]),Quality::PolygonMesh));
    assert_eq!(input_heap.released_bytes,0);let mut evaluation=Some(evaluation);
    let handoff=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
    let mut job=caller.turn(handoff,|cx|ReadyJob::admit(&mut evaluation,cx)).unwrap().unwrap();assert!(evaluation.is_none());
    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(caller.cancel().cancel()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let before=caller.cumulative_receipt();assert!(caller.turn(stock,|cx|job.step_on_original(cx)).unwrap().is_none());assert_eq!(caller.cumulative_receipt(),before);
    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let before=caller.cumulative_receipt();assert!(caller.turn(stock,|cx|job.step_on_original(cx)).unwrap().is_none());assert_eq!(caller.cumulative_receipt(),before);
    let mut owner=ControlledRetirement::new(job).ok().unwrap();let mut born=0;let mut released=0;
    for _ in 0..10000 {
        if owner.terminal_is_empty(){break;}
        let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{if cx.should_yield(){return Ok(None)}let step=owner.step(cx.retained_grant())?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(Some(step))}));
        let step=step.unwrap().unwrap();let receipt=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
    }
    assert!(owner.terminal_is_empty());assert_eq!(input_heap.requested_bytes+born,released);
    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    caller.retire(stock);
}
