use super::*;
use semio_framework_value::{ErasedSnapshotRetirement,NativeDecodeRetirementRecipient};

fn drain(mut retirement:BoardSceneRetirement){let mut hops=0;while !retirement.terminal_is_empty(){retirement.close_step(1,65536).unwrap();hops+=1;assert!(hops<100000);}}

#[test]
fn neutral_tree_cases_match_d3_for_every_hop_size(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🌳️tree/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){for maximum_units in[1,2,3,64,256]{
  let p=&case["options"];let direction=match p["direction"].as_str().unwrap(){"downwards"=>TreeDirection::Downwards,"upwards"=>TreeDirection::Upwards,"right"=>TreeDirection::Right,"left"=>TreeDirection::Left,_=>panic!("closed direction")};
  let options=BoardTreeLayoutOptions{layer_spacing:p["layerSpacing"].as_f64().unwrap(),sibling_gap:p["siblingGap"].as_f64().unwrap(),direction,center:Some([p["center"][0].as_f64().unwrap(),p["center"][1].as_f64().unwrap()]),locked_node_ids:p["locked"].as_array().unwrap().iter().map(|id|id.as_u64().unwrap().to_string()).collect(),redraw_handles:false};
  let mut recipient=NativeDecodeRetirementRecipient::new();let mut yes=|_|true;let mut control=NativeDecodeControl::new(32*1024*1024,&mut yes);control.install_retirement_recipient(&mut recipient).unwrap();
  let mut operation=BoardLayoutOperation::from_json(&case["scene"].to_string(),BoardLayoutOptions::Hierarchical(options),&mut control).unwrap();let mut hops=0;while !operation.advance(maximum_units,&mut control).unwrap(){hops+=1;assert!(hops<100000);}
  let result=operation.finish().unwrap();assert_eq!(result.nodes().len(),case["expected"].as_array().unwrap().len());for(index,node)in result.nodes().iter().enumerate(){assert_eq!(node.id,index.to_string());let expected=&case["expected"][index];assert!((node.x-expected[0].as_f64().unwrap()).abs()<1e-8);assert!((node.y-expected[1].as_f64().unwrap()).abs()<1e-8);}
  drain(result.retirement());drop(control);while !recipient.terminal_is_empty(){recipient.close_step(1,65536).unwrap();}
 }}
}

#[test]
fn canceled_layout_advance_returns_operation_owners(){
 let scene=r#"{"mode":"normal","nodes":[{"id":"a","x":0,"y":0},{"id":"b","x":1,"y":0}],"handles":[],"edges":[{"id":"e","source":"a","target":"b"}]}"#;
 let mut recipient=NativeDecodeRetirementRecipient::new();let mut yes=|_|true;let mut control=NativeDecodeControl::new(32*1024*1024,&mut yes);control.install_retirement_recipient(&mut recipient).unwrap();let mut operation=BoardLayoutOperation::from_json(scene,BoardLayoutOptions::Force(BoardForceLayoutOptions::default()),&mut control).unwrap();drop(control);
 let mut no=|_|false;let mut canceled=NativeDecodeControl::new(0,&mut no);assert!(matches!(operation.advance(1,&mut canceled),Err(error)if error.kind==ValueRefusalKind::Canceled));drain(operation.retirement());while !recipient.terminal_is_empty(){recipient.close_step(1,65536).unwrap();}
}
