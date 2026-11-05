//! 🧪️ A splice graph borrows the actual native base and row records without physical allocation.
use super::*;
use semio_framework_artifact_flow_flow::widget_id_for;
fn indices(value:&serde_json::Value)->Vec<usize>{value.as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize).collect()}
fn optional(value:&serde_json::Value)->Vec<Option<usize>>{value.as_array().unwrap().iter().map(|value|value.as_u64().map(|value|value as usize)).collect()}
#[test]
fn gumball_graph_queries_match_ordered_native_splice_rows_without_copying(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let base:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&corpus["base"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let rows:Vec<Generation3dMutation>=semio_framework_pack_json::from_json_str(&corpus["rows"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for case in corpus["cases"].as_array().unwrap(){let created=indices(&case["created"]);let updated=optional(&case["updated"]);let rewired=optional(&case["rewired"]);let links=indices(&case["links"]);
  let(matched,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let graph=GumballGraphRead::new(&base,&rows,&created,&updated,&rewired,&links).unwrap();let mut matched=graph.widget_count()==case["widgets"].as_array().unwrap().len()&&graph.synapse_count()==case["wires"].as_array().unwrap().len();
   for index in 0..graph.widget_count(){let widget=graph.widget(index).unwrap().unwrap();matched&=Some(widget_id_for(widget))==case["widgets"][index].as_str();matched&=matches!(widget,Widget::Neuron{preview,..}if Some(*preview)==case["previews"][index].as_bool());let original=if let Some(row)=updated[index]{match &rows[row]{Generation3dMutation::UpdateWidget(v)=>&v.widget,_=>unreachable!()}}else if index<base.widgets.len(){&base.widgets[index]}else{match &rows[created[index-base.widgets.len()]]{Generation3dMutation::CreateWidget(v)=>&v.widget,_=>unreachable!()}};matched&=std::ptr::eq(widget,original)}
   for index in 0..graph.synapse_count(){let wire=graph.synapse(index).unwrap().unwrap();for(field,expected)in [&wire.id,&wire.from,&wire.from_port,&wire.to,&wire.to_port].into_iter().zip(case["wires"][index].as_array().unwrap()){matched&=Some(field.as_str())==expected.as_str()}}
   matched&&graph.widget(graph.widget_count()).unwrap().is_none()&&graph.synapse(graph.synapse_count()).unwrap().is_none()
  });assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(matched,"{}",case["name"]);
 }
 for row in rows{row.retire_cold()}base.retire_cold();
}
#[test]
fn gumball_graph_rejects_missing_indices_and_wrong_native_row_domains(){
 let base=FlowHostSnapshot::default();let rows:Vec<Generation3dMutation>=vec![];let created=[0];let updated=[None];let empty=GumballGraphRead::new(&base,&rows,&[],&[],&[],&[]).unwrap();let invalid=GumballGraphRead::new(&base,&rows,&created,&updated,&[],&[]).unwrap();
 let(refused,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||GumballGraphRead::new(&base,&rows,&created,&[],&[],&[]).is_err()&&invalid.widget(0).is_err()&&empty.identity()!=invalid.identity());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));base.retire_cold();assert!(refused);
}
