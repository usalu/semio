//! 🧪️ Native typed input admission equals its defining schema and original admissible operation.
use semio_framework_value::FromValue as _;
use super::retained_typed_input_admission::TypedInputAdmission;
use semio_s_artifact_procedural_3d_mutations::{Generation3dMutation,ChangeWidgetInput,WidgetInputValue};
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::retirement::RetireOwned;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}}
fn run(input:&ChangeWidgetInput)->bool{
 let mut work=TypedInputAdmission::new();
 for _ in 0..200{
  let demand=work.next_demands(input,64).unwrap();
  for denied in [
   RetainedCloneGrant{maximum_items:0,..grant()},
   RetainedCloneGrant{maximum_depth:0,..grant()},
   RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant()},
  ]{
   if denied.maximum_items>0&&denied.maximum_depth>0&&demand.copy_bytes==0{continue}
   let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.advance(input,denied));
   assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!turn.result.unwrap());assert_eq!(turn.work,0);assert_eq!(work.step_progress(),Default::default());
  }
  let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.advance(input,grant()));
  assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(turn.work<=1);assert!(work.step_progress().fits(grant()));
  if turn.result.unwrap(){return work.result().unwrap()}
 }
 panic!("typed input admission exceeded its original opportunity")
}
#[test]
fn typed_input_admission_matches_original_for_all_native_variants(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){
  let mutation:Generation3dMutation=semio_framework_pack_json::from_json_str(&case["input"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let Generation3dMutation::ChangeWidgetInput(input)=mutation else{panic!("typed input fixture")};
  assert_eq!(input.admissible(),case["expected"].as_bool().unwrap());assert_eq!(run(&input),input.admissible(),"{}",case["name"]);
  let other=input.clone();let mut work=TypedInputAdmission::new();work.advance(&input,grant()).result.unwrap();assert!(work.next_demands(&other,64).is_err());
  other.retire_decoded();input.retire_decoded();
 }
}
#[test]
fn typed_input_admission_preserves_nonfinite_numeric_refusals(){
 for input in [WidgetInputValue::Number(f64::NAN),WidgetInputValue::Vector([0.0,f64::INFINITY,1.0]),WidgetInputValue::NumberList(vec![1.0,f64::NEG_INFINITY])]{
  let input=ChangeWidgetInput{id:"n".into(),channel:"value".into(),input};assert!(!input.admissible());assert!(!run(&input));input.retire_decoded();
 }
}
