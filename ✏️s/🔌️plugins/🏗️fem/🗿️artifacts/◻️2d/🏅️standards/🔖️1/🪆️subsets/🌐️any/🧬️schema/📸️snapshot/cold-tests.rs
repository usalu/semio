//! 🧪️ Cold authoring graphs preserve original rows and refuse invalid references.
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::{snapshot::{Fem2dSnapshot,Fem2dSnapshotValue},structure::{FemElement,FemSection}};
use semio_s_fem_node::planar::FemNodeValue;
use semio_s_fem_material::planar::FemMaterialValue;
fn text(value:&serde_json::Value)->String{value.as_str().unwrap().to_owned()}
#[test]
fn reference_graph_fixture_builds_and_retires_owned_persistent_rows(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧪️tests/🛬️receiving/❄️cold/🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap().iter().filter(|case|case["dimension"]==2){
  let mut input=Fem2dSnapshotValue::default();
  input.nodes=case["nodes"].as_array().unwrap().iter().map(|id|FemNodeValue{id:text(id),x:0.,y:0.,}).collect();
  input.materials=vec![FemMaterialValue{id:text(&case["material"]),name:"material".into(),e:1.,nu:0.2,rho:1.,}];
  input.sections=vec![FemSection{id:text(&case["section"]),name:"section".into(),area:1.,iy:1.,}];
  let e=&case["element"];input.elements=vec![FemElement::Bar{id:"e".into(),start:text(&e["start"]),end:text(&e["end"]),material_id:text(&e["material"]),section_id:text(&e["section"])}];
  let result=Fem2dSnapshot::from_reference(input);
  assert_eq!(result.is_ok(),case["valid"].as_bool().unwrap());
  if let Ok(snapshot)=result{assert_eq!(snapshot.nodes.len(),2);assert_eq!(snapshot.materials.len(),1);<Fem2dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);}
 }
}

#[test]
fn cold_builder_crosses_page_boundaries_and_retires_its_roots(){
 for count in [0,1,16,17,257]{
  let mut value=Fem2dSnapshotValue::default();
  value.nodes=(0..count).map(|i|FemNodeValue{id:format!("n{i}"),x:1.,y:2.,}).collect();
  let snapshot=Fem2dSnapshot::from_reference(value).unwrap();
  assert_eq!(snapshot.nodes.len(),count);
  if count>0{assert_eq!(snapshot.nodes.get(count-1).unwrap().id.as_str(),format!("n{}",count-1));}
  <Fem2dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);
 }
}


fn exact(d:semio_framework_value::RetirementDemand)->semio_framework_value::RetainedCloneGrant{semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn exclusive_cold_cleanup_refuses_aliases_and_allocates_nothing(){
 use semio_framework_value::{RetirementDemand,shared_issuance::{SealedShared,SharedIssuer}};
 use semio_framework_value_persistent::list::persistent::PersistentListBuilder;
 let d=RetirementDemand{capacity_bytes:SealedShared::<u64>::birth_bytes(),depth:1,..Default::default()};
 let (row,_)=SealedShared::admit(7u64,SharedIssuer::owned(),exact(d)).result.unwrap();let alias=row.clone();let mut row=Some(row);let mut builder=PersistentListBuilder::new(1);let d=builder.adopt_demand().unwrap();builder.adopt(&mut row,exact(d)).result.unwrap().unwrap();
 let (failure,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||SealedShared::admit(8u64,SharedIssuer::owned(),exact(RetirementDemand{capacity_bytes:SealedShared::<u64>::birth_bytes(),depth:1,..Default::default()})));assert_eq!(attempts,1);let (error,unadmitted)=failure.result.err().unwrap();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::AllocationFailed);assert_eq!(unadmitted,8);
 let (accepted,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||builder.cold_dismantle());assert!(!accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(alias);
 let (accepted,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||builder.cold_dismantle());assert!(accepted);assert_eq!(heap.requested_bytes,0);assert!(builder.terminal_is_empty());
 let d=RetirementDemand{capacity_bytes:SealedShared::<u64>::birth_bytes(),depth:1,..Default::default()};let (row,_)=SealedShared::admit(9u64,SharedIssuer::owned(),exact(d)).result.unwrap();let mut row=Some(row);let mut builder=PersistentListBuilder::new(1);let d=builder.adopt_demand().unwrap();builder.adopt(&mut row,exact(d)).result.unwrap().unwrap();let d=builder.finish_demand().unwrap();builder.finish(exact(d)).result.unwrap().unwrap();while !builder.output_ready(){let d=builder.demand().unwrap();builder.advance(exact(d)).result.unwrap();}let d=builder.take_demand().unwrap();let (mut list,_)=builder.take(exact(d)).result.unwrap().unwrap();assert!(builder.cold_dismantle());let alias=list.clone();
 let (accepted,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||list.cold_dismantle());assert!(!accepted);assert_eq!(list.len(),1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(alias);let ((accepted,heap),attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||semio_framework_trace::observe_heap_allocations_on_this_thread(||list.cold_dismantle()));assert!(accepted);assert_eq!(heap.requested_bytes,0);assert_eq!(attempts,0);assert!(list.is_empty());
}

#[test]
fn nested_string_alias_refuses_the_unchanged_cold_tree(){
 let mut nodes=semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::cold::nodes2d(vec![FemNodeValue{id:"alpha".into(),x:1.,y:2.}]).unwrap();
 let alias=nodes.get(0).unwrap().id.clone();
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||nodes.cold_dismantle());assert!(!ok);assert_eq!(attempts,0);assert_eq!(nodes.get(0).unwrap().id.as_str(),"alpha");drop(alias);
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||nodes.cold_dismantle());assert!(ok);assert_eq!(attempts,0);assert!(nodes.is_empty());
}

#[test]
fn material_text_alias_refuses_unchanged_then_drains_without_allocation(){
 let mut materials=semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::cold::materials2d(vec![FemMaterialValue{id:"steel".into(),name:"Steel".into(),e:2.,nu:0.2,rho:1.}]).unwrap();
 let alias=materials.get(0).unwrap().name.clone();
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||materials.cold_dismantle());assert!(!ok);assert_eq!(attempts,0);assert_eq!(materials.get(0).unwrap().name.as_str(),"Steel");drop(alias);
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||materials.cold_dismantle());assert!(ok);assert_eq!(attempts,0);assert!(materials.is_empty());
}
