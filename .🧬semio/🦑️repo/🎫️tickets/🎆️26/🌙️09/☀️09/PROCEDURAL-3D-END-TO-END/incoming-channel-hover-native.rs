extern crate self as semio_s_artifact_procedural_3d_model;
extern crate self as semio_framework_artifact_flow_flow;
extern crate self as ui_contract;
pub use graph_codec::{graph_port_id,parse_graph_port_id,GraphTargetKind};
pub struct SynapseSpec { id:String, from:String, to:String, from_port:String, to_port:String }
pub struct FlowHostSnapshot { synapses:Vec<SynapseSpec> }
#[derive(Debug,PartialEq,Eq,Default)] enum UiState { #[default] Normal, Disabled, Introducing, Celebrating, Previewed }
#[derive(Default)] enum UiStatus { #[default] Idle, Waiting, Loading, Finished }
#[derive(Default)] struct UiPresence { state:UiState,status:UiStatus,selected:bool }
enum Activity { Idle, Waiting, Loading, Finished }
enum TransitionHint { Introducing, Celebrating }
struct Style { highlighted:bool }
struct UiNodeRecord { disabled:bool, transition:Option<TransitionHint>, activity:Activity, style:Style }
fn main() {
    use semio_framework_pack_json::{parse,JsonMemberPolicy,Value};
    let fixture=parse(&std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(),JsonMemberPolicy::Reject).unwrap();
    let s=|value:&Value,key:&str|value.get(key).unwrap().as_str().unwrap().to_owned();
    let host=FlowHostSnapshot{synapses:fixture["wires"].as_array().unwrap().iter().map(|wire|SynapseSpec{id:s(wire,"id"),from:s(wire,"from"),from_port:s(wire,"fromPort"),to:s(wire,"to"),to_port:s(wire,"toPort")}).collect()};
    let candidates=fixture["candidates"].as_array().unwrap();
    for row in candidates {
        assert_eq!(row["id"].as_str().unwrap(),graph_port_id(row["widget"].as_str().unwrap(),row["channel"].as_str().unwrap(),row["input"].as_bool().unwrap()));
    }
    println!("[");
    let cases=fixture["cases"].as_array().unwrap();
    for (index,case) in cases.iter().enumerate() {
        let graph=case["graph"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let marks=PreviewChannelHover::new(&host,&graph,case.get("component").and_then(Value::as_str).map(str::to_owned));
        let matched=marks.matching(candidates.iter().map(|row|row["id"].as_str().unwrap()));
        let expected=case["expected"].as_array().unwrap().iter().map(|row|row.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(matched,expected,"{}",case["name"]);
        assert!(!marks.matches("not a graph port"));
        println!("{}{}",semio_framework_pack_json::to_string(&Value::Array(matched.into_iter().map(Value::String).collect())),if index+1==cases.len(){""}else{","});
    }
    println!("]");
    let mut record=UiNodeRecord{disabled:false,transition:None,activity:Activity::Idle,style:Style{highlighted:true}};
    let presence=record_presence(&record);
    assert_eq!(presence.state,UiState::Previewed);
    assert!(!presence.selected);
    record.style.highlighted=false;
    assert_eq!(record_presence(&record).state,UiState::Normal);
    record.style.highlighted=true;
    record.disabled=true;
    assert_eq!(record_presence(&record).state,UiState::Disabled);
}
