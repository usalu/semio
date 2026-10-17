use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
fn clock()->Option<u64>{Some(1)}

#[test]
fn original_widget_inputs_constructor_quotes_utf8_pages_and_moves_actual_values_once(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let c=&law["originalCaller"];
 let stock=RetainedCloneGrant{maximum_items:c["items"].as_u64().unwrap()as usize,maximum_copy_bytes:c["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:c["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:c["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:c["depth"].as_u64().unwrap()as usize};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());
 let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let mut kind=catalogue.kind("math.number").unwrap().clone();let template=kind.inputs[0].clone();
 kind.inputs=law["ports"].as_array().unwrap().iter().map(|row|{let mut port=template.clone();port.name=row["name"].as_str().unwrap().into();port.label=Localized{en:row["en"].as_str().unwrap().into(),de:row["de"].as_str().unwrap().into()};port.port_type=PortType::Text;port.default=None;port.optional=true;port}).collect();
 let(values,input_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_s_artifact_procedural_3d_geometry_value::outputs([("alpha",GeometryValue::Text(law["valueText"].as_str().unwrap().into()))]));assert_eq!(input_heap.released_bytes,0);
 let pointer=match values.get("alpha").unwrap(){GeometryValue::Text(text)=>text.as_ptr(),_=>unreachable!()};let mut values=Some(values);let mut construction=WidgetInputsConstruction::default();let widget=law["widget"].as_str().unwrap();
 let before=caller.cumulative_receipt();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(Default::default(),|cx|WidgetInputs::new(widget,&kind,&mut values,&mut construction,cx)));assert!(result.unwrap().is_none());assert!(values.is_some());assert_eq!(caller.cumulative_receipt(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let mut born=0;let mut released=0;let mut output=None;
 for _ in 0..10000{
  let before=caller.cumulative_receipt();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|WidgetInputs::new(widget,&kind,&mut values,&mut construction,cx)));let result=result.unwrap();let after=caller.cumulative_receipt();
  assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
  if result.is_some(){output=result;break;}
 }
 let output=output.expect("original finite widget inputs construction");assert!(values.is_none());assert_eq!(output.widget(),widget);
 assert_eq!(output.labels.iter().map(|(name,_)|name.as_str()).collect::<Vec<_>>(),law["expectedOrder"].as_array().unwrap().iter().map(|name|name.as_str().unwrap()).collect::<Vec<_>>());
 for row in law["ports"].as_array().unwrap(){let label=output.labels.get(row["name"].as_str().unwrap()).unwrap();assert_eq!(label.en,row["en"].as_str().unwrap());assert_eq!(label.de,row["de"].as_str().unwrap());}
 assert_eq!(match output.get("alpha").unwrap(){GeometryValue::Text(text)=>text.as_ptr(),_=>unreachable!()},pointer);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{assert_eq!(output.text("alpha").unwrap(),law["valueText"].as_str().unwrap());});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 assert!(construction.terminal_is_empty());assert!(caller.turn(stock,|cx|WidgetInputs::new(widget,&kind,&mut values,&mut construction,cx)).unwrap().is_none());
 let mut owner=ControlledRetirement::new((construction,output)).ok().unwrap();
 for _ in 0..10000{if owner.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{if cx.should_yield(){return Ok(None)}let step=owner.step(cx.retained_grant())?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(Some(step))}));let step=step.unwrap().unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}
 assert!(owner.terminal_is_empty());assert_eq!(input_heap.requested_bytes+born,released);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 caller.retire(stock);
}
