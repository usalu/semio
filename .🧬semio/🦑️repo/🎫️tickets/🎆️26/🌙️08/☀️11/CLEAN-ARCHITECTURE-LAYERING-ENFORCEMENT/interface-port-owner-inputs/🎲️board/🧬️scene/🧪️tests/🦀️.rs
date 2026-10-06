//! 🧪️ The same neutral scene corpus exercises native admission and actual retained host release.
use super::*;

fn drain(mut retirement:BoardSceneRetirement){
 for _ in 0..8192 {
  let bytes=retirement.next_close_byte_demand().max(4096);
  if matches!(retirement.close_step(1,bytes).unwrap(),SnapshotRetirementStep::Complete){assert!(retirement.terminal_is_empty());return;}
 }
 panic!("neutral scene retirement must reach its actual terminal witness");
}

#[test]
fn neutral_scene_portable_corpus(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let mut yes=|_|true;
  let result=from_json(&case["input"].to_string(),&mut NativeDecodeControl::new(32*1024*1024,&mut yes));
  assert_eq!(result.is_ok(),case["accepted"].as_bool().unwrap(),"{}",case["input"]);
  match result {Ok(scene)=>drain(scene.retirement()),Err(mut error)=>{if let Some(retirement)=error.take_retirement(){drain(*retirement);}}}
 }
}

#[test]
fn neutral_scene_cancellation_and_ownership_are_owned_refusals(){
 let input=r#"{"mode":"normal","nodes":[],"handles":[],"edges":[]}"#;
 let mut no=|_|false;
 assert!(from_json(input,&mut NativeDecodeControl::new(usize::MAX,&mut no)).is_err());
 let mut yes=|_|true;
 assert!(from_json(input,&mut NativeDecodeControl::new(0,&mut yes)).is_err());
}
