use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputsConstruction;
use super::super::GeometryComputeContext;
use semio_s_artifact_procedural_3d_io_geometry::NativeGeometryComputeContext;
fn clock()->Option<u64>{Some(1)}
pub fn grant(c:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:c["items"].as_u64().unwrap()as usize,maximum_copy_bytes:c["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:c["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:c["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:c["depth"].as_u64().unwrap()as usize}}
fn vector(value:&serde_json::Value)->[f64;3]{std::array::from_fn(|i|value[i].as_f64().unwrap())}
fn value(kind:&str,input:&serde_json::Value)->GeometryValue{match kind{"math.integer"=>GeometryValue::Integer(input.as_i64().unwrap()),"math.boolean"=>GeometryValue::Boolean(input.as_bool().unwrap()),"math.vector"=>GeometryValue::Vector(vector(input)),"math.point"=>GeometryValue::Point(vector(input)),"math.plane"=>GeometryValue::Plane(semio_s_artifact_procedural_3d_geometry_value::PlaneValue{origin:vector(&input["origin"]),normal:vector(&input["normal"])}),_=>GeometryValue::Number(input.as_f64().unwrap())}}
pub fn accounted<T>(caller:&mut OriginalJobFixtureCaller,policy:RetainedCloneGrant,born:&mut usize,released:&mut usize,turn:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let before=caller.cumulative_receipt();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(policy,turn));let after=caller.cumulative_receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));*born+=heap.requested_bytes;*released+=heap.released_bytes;result.unwrap()
}
pub fn close<T:RetireOwned>(original:T,caller:&mut OriginalJobFixtureCaller,stock:RetainedCloneGrant,born:&mut usize,released:&mut usize){let mut owner=ControlledRetirement::new(original).ok().unwrap();for _ in 0..100000{if owner.terminal_is_empty(){break}accounted(caller,stock,born,released,|cx|{let receipt=owner.step(cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.progress();post(cx,receipt)})}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}

#[test]
fn original_math_values_construct_actual_outputs_close_inputs_and_conserve_system(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let c=&law["originalCaller"];let stock=grant(c);let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let mut born=0;let mut released=0;let mut original_bytes=0;
 for row in law["cases"].as_array().unwrap(){
  let id=row["kind"].as_str().unwrap();let kind=catalogue.kind(id).unwrap();let(inputs,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_s_artifact_procedural_3d_geometry_value::outputs([("value",value(id,&row["input"]))]));assert_eq!(heap.released_bytes,0);original_bytes+=heap.requested_bytes;let mut inputs=Some(inputs);let mut construction=WidgetInputsConstruction::default();let mut resolved=None;
  for _ in 0..10000{resolved=accounted(&mut caller,stock,&mut born,&mut released,|cx|WidgetInputs::new("original-value",kind,&mut inputs,&mut construction,cx));if resolved.is_some(){break}}
  assert!(inputs.is_none());assert!(resolved.is_some());
  for denied in[RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,..Default::default()}]{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,denied,&mut born,&mut released,|cx|OriginalFixedOutputJob::admit(kind,&mut resolved,cx)).is_none());assert!(resolved.is_some());assert_eq!(before,caller.cumulative_receipt());}
  let before=caller.cumulative_receipt();let mut job=accounted(&mut caller,stock,&mut born,&mut released,|cx|NativeGeometryComputeContext.start_original(kind,&mut resolved,cx)).unwrap();assert!(resolved.is_none());assert_eq!(caller.cumulative_receipt().retained_capacity_bytes,before.retained_capacity_bytes);
  let before=caller.cumulative_receipt();assert!(accounted(&mut caller,Default::default(),&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());
  let mut output=None;
  for _ in 0..100000{if let Some(WidgetStep::Done(result))=accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)){output=Some(result);break}}
  let output=output.expect("actual original scalar output");assert_eq!(output.outputs.get("value").unwrap(),&value(id,&row["output"]));assert!(job.terminal_is_empty());assert!(accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)).is_none());close((construction,job,output),&mut caller,stock,&mut born,&mut released);
 }
 assert_eq!(original_bytes+born,released);caller.retire(stock);
}

#[test]
fn original_math_value_cancellation_retains_partial_output_for_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let c=&law["originalCaller"];let stock=grant(c);let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let kind=catalogue.kind("math.number").unwrap();let(inputs,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_s_artifact_procedural_3d_geometry_value::outputs([("value",GeometryValue::Number(2.5))]));let original=heap.requested_bytes;let mut inputs=Some(inputs);let mut construction=WidgetInputsConstruction::default();let mut resolved=None;let mut born=0;let mut released=0;
 for _ in 0..10000{resolved=accounted(&mut caller,stock,&mut born,&mut released,|cx|WidgetInputs::new("cancelled-original-value",kind,&mut inputs,&mut construction,cx));if resolved.is_some(){break}}
 let mut job=accounted(&mut caller,stock,&mut born,&mut released,|cx|OriginalFixedOutputJob::admit(kind,&mut resolved,cx)).unwrap();for _ in 0..100000{if job.state.phase==5{break}assert!(accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)).is_none())}assert_eq!(job.state.phase,5);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let before=caller.cumulative_receipt();assert!(accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());close((construction,job),&mut caller,stock,&mut born,&mut released);assert_eq!(original+born,released);caller.retire(stock);
}

fn geometry_inputs(row:&serde_json::Value,kind:&str)->Outputs{
 use semio_framework_3d_brep_engine::{ShapeRoot,ShapeValue};
 use {semio_framework_3d_brep_curve::{Curve3, bspline::KnotVector}, semio_framework_3d_brep_representation::topology::Body, semio_framework_3d_brep_vector::Pnt3};
 use semio_s_artifact_procedural_3d_geometry_value::outputs;
 use semio_s_artifact_procedural_3d_geometry_value::SelectionValue;
 use semio_s_artifact_procedural_3d_geometry_value::SelectionKind;
 if kind=="mesh.inspect.vertex"{let positions:Vec<f32>=row["positions"].as_array().unwrap().iter().flat_map(|p|p.as_array().unwrap().iter().map(|v|v.as_f64().unwrap()as f32)).collect();let indices:Vec<u32>=row["indices"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u32).collect();let mesh=semio_framework_3d::mesh::HalfedgeMesh::from_indexed_triangles(&positions,&indices).unwrap();return outputs([("mesh",GeometryValue::mesh(mesh)),("vertex",GeometryValue::Selection(SelectionValue{component:SelectionKind::Vertex,ids:vec![row["vertex"].as_u64().unwrap()]}))])}
 let controls:Vec<Pnt3>=row["controls"].as_array().unwrap().iter().map(|v|{let p=vector(v);Pnt3::new(p[0],p[1],p[2])}).collect();let weights=row["weights"].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect();let knots=KnotVector::new(row["knots"].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect(),row["degree"].as_u64().unwrap()as usize,controls.len()).unwrap();let mut body=Body::new();let label=body.labels.next_label();let shape=ShapeValue{root:ShapeRoot::Curve{curve:Curve3::Nurbs{knots,controls,weights},label},body};shape.check().unwrap();outputs([("curve",GeometryValue::shape(shape)),("parameter",GeometryValue::Number(row["parameter"].as_f64().unwrap()))])
}

#[test]
fn original_geometry_fixed_output_reads_actual_mesh_and_nurbs_on_same_caller(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../⚙️compute/🎟️original/🧪️tests/🧫️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let c=&policy["originalCaller"];let stock=grant(c);let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let mut born=0;let mut released=0;let mut original=0;
 for(name,cancel)in[("mesh",false),("curve",false),("curve",true)]{
  let row=&law[name];let id=row["kind"].as_str().unwrap();let kind=catalogue.kind(id).unwrap();let(inputs,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||geometry_inputs(row,id));original+=heap.requested_bytes-heap.released_bytes;let mut inputs=Some(inputs);let mut construction=WidgetInputsConstruction::default();let mut resolved=None;
  for _ in 0..10000{resolved=accounted(&mut caller,stock,&mut born,&mut released,|cx|WidgetInputs::new("original-geometry-query",kind,&mut inputs,&mut construction,cx));if resolved.is_some(){break}}
  let mut job=accounted(&mut caller,stock,&mut born,&mut released,|cx|NativeGeometryComputeContext.start_original(kind,&mut resolved,cx)).unwrap();let mut output=None;let mut denied_capacity=false;let mut denied_copy=false;
  for _ in 0..100000{
   if name=="curve"&&job.state.curve.as_ref().is_some_and(|curve|curve.scratch_pending())&&!denied_capacity{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,RetainedCloneGrant{maximum_capacity_bytes:0,..stock},&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());denied_capacity=true;}
   if name=="curve"&&job.state.curve.as_ref().is_some_and(|curve|curve.scratch_copy_pending())&&!denied_copy{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,RetainedCloneGrant{maximum_copy_bytes:31,..stock},&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());denied_copy=true;if cancel{let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break}}
   if let Some(WidgetStep::Done(result))=accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)){output=Some(result);break}
  }
  if name=="curve"{assert!(denied_capacity&&denied_copy)}
  if cancel{assert!(output.is_none());let before=caller.cumulative_receipt();assert!(accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());close((construction,job),&mut caller,stock,&mut born,&mut released)}
  else{let output=output.unwrap();let key=if name=="mesh"{"position"}else{"point"};let GeometryValue::Point(point)=output.outputs.get(key).unwrap()else{panic!("actual original geometric point")};for(i,v)in point.iter().enumerate(){assert!((v-row["output"][i].as_f64().unwrap()).abs()<law["tolerance"].as_f64().unwrap())}assert!(job.terminal_is_empty());close((construction,job,output),&mut caller,stock,&mut born,&mut released)}
 }
 assert_eq!(original+born,released);caller.retire(stock);
}

#[test]
fn original_geometry_plane_constructs_actual_topology_and_conserves_cancelled_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../⚙️compute/🎟️original/🧪️tests/🧫️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let row=&law["plane"];let c=&policy["originalCaller"];let stock=grant(c);let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let kind=catalogue.kind(row["kind"].as_str().unwrap()).unwrap();let mut born=0;let mut released=0;let mut original=0;
 for cancel in[false,true]{
  let(inputs,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_s_artifact_procedural_3d_geometry_value::outputs([("width",GeometryValue::Number(row["width"].as_f64().unwrap())),("depth",GeometryValue::Number(row["depth"].as_f64().unwrap()))]));original+=heap.requested_bytes-heap.released_bytes;let mut inputs=Some(inputs);let mut construction=WidgetInputsConstruction::default();let mut resolved=None;
  for _ in 0..10000{resolved=accounted(&mut caller,stock,&mut born,&mut released,|cx|WidgetInputs::new("original-plane",kind,&mut inputs,&mut construction,cx));if resolved.is_some(){break}}
  let mut job=accounted(&mut caller,stock,&mut born,&mut released,|cx|NativeGeometryComputeContext.start_original(kind,&mut resolved,cx)).unwrap();let mut output=None;let mut denied_backing=false;
  for _ in 0..100000{
   if let Some(plane)=job.state.plane.as_ref(){if plane.demand().capacity_bytes>0&&!denied_backing{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,RetainedCloneGrant{maximum_capacity_bytes:0,..stock},&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());denied_backing=true;}if cancel&&job.state.plane.as_ref().unwrap().progress()>=3.0/7.0{let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break}}
   if let Some(WidgetStep::Done(result))=accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)){output=Some(result);break}
  }
  assert!(denied_backing);
  if cancel{assert!(output.is_none());close((construction,job),&mut caller,stock,&mut born,&mut released)}
  else{let output=output.unwrap();let GeometryValue::Mesh(mesh)=output.outputs.get("mesh").unwrap()else{panic!("actual original plane mesh")};assert_eq!((mesh.vertex_count(),mesh.face_count(),mesh.halfedge_count()),(4,row["faces"].as_u64().unwrap()as usize,row["halfedges"].as_u64().unwrap()as usize));for i in 0..4{assert_eq!(mesh.vertex_position(semio_framework_3d::mesh::VertexId(i)).unwrap().0.map(f64::from),vector(&row["positions"][i as usize]));assert_eq!(mesh.vertex_normal(semio_framework_3d::mesh::VertexId(i)).unwrap().0.map(f64::from),vector(&row["normal"]))}assert!(job.terminal_is_empty());close((construction,job,output),&mut caller,stock,&mut born,&mut released)}
 }
 assert_eq!(original+born,released);caller.retire(stock);
}

#[test]
fn original_geometry_brep_line_constructs_shape_and_conserves_cancelled_shared_root(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../⚙️compute/🎟️original/🧪️tests/🧫️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let row=&law["line"];let c=&policy["originalCaller"];let stock=grant(c);let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let kind=catalogue.kind(row["kind"].as_str().unwrap()).unwrap();let mut born=0;let mut released=0;let mut original=0;
 for cancel in[false,true]{
  let(inputs,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_s_artifact_procedural_3d_geometry_value::outputs([("start",GeometryValue::Point(vector(&row["start"]))),("end",GeometryValue::Point(vector(&row["end"])))]));original+=heap.requested_bytes-heap.released_bytes;let mut inputs=Some(inputs);let mut construction=WidgetInputsConstruction::default();let mut resolved=None;
  for _ in 0..10000{resolved=accounted(&mut caller,stock,&mut born,&mut released,|cx|WidgetInputs::new("original-brep-line",kind,&mut inputs,&mut construction,cx));if resolved.is_some(){break}}
  let mut job=accounted(&mut caller,stock,&mut born,&mut released,|cx|NativeGeometryComputeContext.start_original(kind,&mut resolved,cx)).unwrap();let mut output=None;let mut denied_birth=false;
  for _ in 0..100000{
   if job.state.shape.is_some()&&!denied_birth{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,RetainedCloneGrant{maximum_capacity_bytes:0,..stock},&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());denied_birth=true;}
   if cancel&&job.state.phase==1{let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break}
   if let Some(WidgetStep::Done(result))=accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx)){output=Some(result);break}
  }
  assert!(denied_birth);
  if cancel{assert!(output.is_none());close((construction,job),&mut caller,stock,&mut born,&mut released)}
  else{let output=output.unwrap();let GeometryValue::Shape(shape)=output.outputs.get("shape").unwrap()else{panic!("actual original BRep line")};let ShapeRoot::Curve{curve:Curve3::Line{origin,dir},..}=&shape.root else{panic!("actual original line root")};let point=*origin+*dir*row["parameter"].as_f64().unwrap();assert_eq!([point.x,point.y,point.z],vector(&row["output"]));assert!(job.terminal_is_empty());close((construction,job,output),&mut caller,stock,&mut born,&mut released)}
 }
 assert_eq!(original+born,released);caller.retire(stock);
}

fn translation_inputs(row:&serde_json::Value)->Outputs{
 use semio_framework_3d::mesh::{MeshAttribute,MeshAttributeDomain,MeshAttributeSemantic,MeshAttributeInterpolation,MeshTexture,HistoryFoldIndex,EdgeId};
 let positions:Vec<f32>=row["positions"].as_array().unwrap().iter().flat_map(|p|p.as_array().unwrap().iter().map(|v|v.as_f64().unwrap()as f32)).collect();let indices:Vec<u32>=row["indices"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u32).collect();let mut mesh=HalfedgeMesh::from_indexed_triangles(&positions,&indices).unwrap();let seams:Vec<EdgeId>=row["seams"].as_array().unwrap().iter().map(|v|EdgeId(v.as_u64().unwrap()as u32)).collect();mesh.mark_uv_seam(&seams,true);
 {let name=row["attribute"]["name"].as_str().unwrap();let values=&row["attribute"]["values"];mesh.set_attribute(name.to_string(),MeshAttribute{domain:MeshAttributeDomain::Vertex,semantic:MeshAttributeSemantic::Color,interpolation:MeshAttributeInterpolation::Linear,values:values.as_array().unwrap().iter().cloned().map(semio_framework_value::value::DslValue::from).collect(),indices:None}).unwrap();}
 let materials=[(row["material"]["name"].as_str().unwrap().to_string(),semio_framework_value::value::DslValue::from(row["material"]["value"].clone()))].into_iter().collect::<HistoryFoldIndex<_,_>>();let textures=[(row["texture"]["name"].as_str().unwrap().to_string(),MeshTexture{mime:row["texture"]["mime"].as_str().unwrap().into(),bytes:row["texture"]["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect()})].into_iter().collect::<HistoryFoldIndex<_,_>>();mesh.set_surface_assets(materials,textures).unwrap();semio_s_artifact_procedural_3d_geometry_value::outputs([("mesh",GeometryValue::mesh(mesh)),("offset",GeometryValue::Vector(vector(&row["offset"])) )])
}

fn translation_law(cancel_after:u8){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../⚙️compute/🎟️original/🧪️tests/🧫️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️.json")).unwrap();let row=&law["translation"];let c=&policy["originalCaller"];let stock=grant(c);let mut caller=OriginalJobFixtureCaller::admit(OperationId(c["operation"].as_u64().unwrap()),Generation(c["generation"].as_u64().unwrap()),stock,clock,c["deadlineMicros"].as_u64().unwrap(),c["fuel"].as_u64().unwrap());let catalogue=semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue();let kind=catalogue.kind(row["kind"].as_str().unwrap()).unwrap();let mut born=0;let mut released=0;let mut original=0;
 for cancel in[0,cancel_after]{
  let(inputs,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||translation_inputs(row));original+=heap.requested_bytes-heap.released_bytes;let mut inputs=Some(inputs);let mut construction=WidgetInputsConstruction::default();let mut resolved=None;
  for _ in 0..10000{resolved=accounted(&mut caller,stock,&mut born,&mut released,|cx|WidgetInputs::new("original-metadata-translation",kind,&mut inputs,&mut construction,cx));if resolved.is_some(){break}}
  let mut job=accounted(&mut caller,stock,&mut born,&mut released,|cx|NativeGeometryComputeContext.start_original(kind,&mut resolved,cx)).unwrap();let mut output=None;let mut denied_capacity=false;let mut denied_copy=false;let mut cancelled=false;
  for _ in 0..100000{
   if let Some((demand,translation_progress))=job.state.translation.as_ref().map(|translation|(translation.demand(stock.maximum_copy_bytes).unwrap(),translation.progress())){if demand.capacity_bytes>0&&!denied_capacity{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,RetainedCloneGrant{maximum_capacity_bytes:0,..stock},&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());denied_capacity=true;}if demand.copy_bytes>0&&!denied_copy&&translation_progress>=2.0/7.0{let before=caller.cumulative_receipt();assert!(accounted(&mut caller,RetainedCloneGrant{maximum_copy_bytes:0,..stock},&mut born,&mut released,|cx|job.step_original(cx)).is_none());assert_eq!(before,caller.cumulative_receipt());denied_copy=true;}}
   let allocation_before=born;let result=accounted(&mut caller,stock,&mut born,&mut released,|cx|job.step_original(cx));if let Some(WidgetStep::Done(value))=result{output=Some(value);break}
   let progress=job.state.translation.as_ref().map(|owner|owner.progress());if(cancel==1&&born>allocation_before&&progress.is_some_and(|progress|progress>2.0/7.0&&progress<3.0/7.0))||(cancel==2&&progress.is_some_and(|progress|progress>=3.0/7.0)){let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{job.cancel();semio_framework_async::poll::resolve_ready(caller.cancel().cancel());});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));cancelled=true;break}
  }
  assert!(denied_capacity);if cancel!=0{assert!(cancelled&&output.is_none());close((construction,job),&mut caller,stock,&mut born,&mut released);break}
  assert!(denied_copy);let output=output.unwrap();let GeometryValue::Mesh(mesh)=output.outputs.get("mesh").unwrap()else{panic!("actual original translated mesh")};assert_eq!((mesh.vertex_count(),mesh.halfedge_count(),mesh.face_count()),(3,3,1));for i in 0..3{assert_eq!(mesh.vertex_position(semio_framework_3d::mesh::VertexId(i)).unwrap().0.map(f64::from),vector(&row["output"][i as usize]))}for edge in row["seams"].as_array().unwrap(){assert!(mesh.is_uv_seam(semio_framework_3d::mesh::EdgeId(edge.as_u64().unwrap()as u32)))}
  {let name=row["attribute"]["name"].as_str().unwrap();let values=&row["attribute"]["values"];let attribute=mesh.attributes().get(name).unwrap();assert_eq!(attribute.values.len(),values.as_array().unwrap().len());for(value,expected)in attribute.values.iter().zip(values.as_array().unwrap()){assert_eq!(value,&semio_framework_value::value::DslValue::from(expected.clone()))}}
  assert_eq!(mesh.materials().get(row["material"]["name"].as_str().unwrap()).unwrap(),&semio_framework_value::value::DslValue::from(row["material"]["value"].clone()));let texture=mesh.textures().get(row["texture"]["name"].as_str().unwrap()).unwrap();assert_eq!(texture.mime,row["texture"]["mime"].as_str().unwrap());assert_eq!(texture.bytes,row["texture"]["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>());assert!(job.terminal_is_empty());close((construction,job,output),&mut caller,stock,&mut born,&mut released);
 }
 assert_eq!(original+born,released);caller.retire(stock);
}

#[test]
fn original_geometry_mesh_translation_copies_seven_owners_and_preserves_metadata_on_same_caller(){translation_law(1)}
#[test]
fn original_geometry_mesh_translation_cancel_closes_all_seven_copied_owners_on_same_caller(){translation_law(2)}
