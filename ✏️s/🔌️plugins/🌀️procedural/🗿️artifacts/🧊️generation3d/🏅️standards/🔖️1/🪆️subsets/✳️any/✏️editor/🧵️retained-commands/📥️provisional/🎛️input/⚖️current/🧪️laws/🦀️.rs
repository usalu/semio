//! 🧪️ Bounded current-value comparison matches genuine original landing semantics and physical receipts.
use semio_os_flow::neural::ColdRetire as _;
use semio_framework_value::FromValue as _;
use super::retained_typed_current::{ProvisionalInputComparison,ProvisionalInputMatch};
use super::retained_scratch_testing::ScratchCaller;
use semio_s_artifact_procedural_3d_mutations::{WidgetInputValue,ChangeWidgetInput};
use semio_framework_artifact_flow_flow::{Widget,neural::{Value,Atom,Dictionary}};
use semio_framework_job::close_original_retained_step;
use semio_framework_value::retirement::RetireOwned;
fn original(current:&Value,input:&WidgetInputValue)->ProvisionalInputMatch{
 let widget=Widget::Neuron{id:"n".into(),neuron_kind:"fixture".into(),params:Dictionary::new().insert("value",current.clone()),input_ports:Vec::new(),output_ports:Vec::new(),preview:true};
 let payload=ChangeWidgetInput{id:"n".into(),channel:"value".into(),input:input.clone()};
 let result=match payload.landing(&widget,false){Ok(None)=>ProvisionalInputMatch::Equal,Ok(Some(widget))=>{widget.retire_cold();ProvisionalInputMatch::Different},Err(_)=>ProvisionalInputMatch::Refused};
 payload.retire_decoded();widget.retire_cold();result
}
fn run(current:&Value,input:&WidgetInputValue,cut:Option<usize>)->Option<ProvisionalInputMatch>{
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 let mut caller=ScratchCaller::new(&policy,1,0);let mut work=caller.turn(|cx|Ok(ProvisionalInputComparison::new(cx)));let mut calls=0;
 loop{if cut==Some(calls){caller.cancel();break}calls+=1;let ready=caller.turn(|cx|work.step(current,input,cx));if ready{break}assert!(calls<32768)}
 let result=work.result();work.begin_close();
 for _ in 0..32768{if work.terminal_is_empty(){break}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}
 assert!(work.terminal_is_empty());caller.finish();result
}
#[test]
fn provisional_current_input_matches_original_landing_with_prefix_custody(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){
  let current:Value=semio_framework_pack_json::from_json_str(&case["current"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let input:WidgetInputValue=semio_framework_pack_json::from_json_str(&case["input"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let expected=original(&current,&input);let name=match expected{ProvisionalInputMatch::Equal=>"equal",ProvisionalInputMatch::Different=>"different",ProvisionalInputMatch::Refused=>"refused"};assert_eq!(name,case["expected"].as_str().unwrap());
  assert_eq!(run(&current,&input,None),Some(expected),"{}",case["name"]);run(&current,&input,Some(0));run(&current,&input,Some(1));
  current.retire_cold();input.retire_decoded();
 }
}
#[test]
fn provisional_current_input_preserves_scalar_vs_list_nonfinite_policy(){
 for list in [false,true]{
  let literal=Value::Dictionary(Dictionary::new().insert("$schema",Value::Atom(Atom::String("number".into()))).insert("value",Value::Atom(Atom::Decimal(f64::NAN))));
  let(current,input)=if list{(Value::Dictionary(Dictionary::new().insert("$schema",Value::Atom(Atom::String("list".into()))).insert("0",literal)),WidgetInputValue::NumberList(vec![1.0]))}else{(literal,WidgetInputValue::Number(1.0))};
  let expected=if list{ProvisionalInputMatch::Refused}else{ProvisionalInputMatch::Different};assert_eq!(original(&current,&input),expected);assert_eq!(run(&current,&input,None),Some(expected));current.retire_cold();input.retire_decoded();
 }
}
