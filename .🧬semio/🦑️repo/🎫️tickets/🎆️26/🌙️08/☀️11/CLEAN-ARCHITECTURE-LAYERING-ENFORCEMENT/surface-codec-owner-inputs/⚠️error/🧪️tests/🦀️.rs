//! 🧯️ Original messages and categories agree with the shared neutral corpus.
use super::*;
#[derive(Debug)]
struct Cause(String);
impl fmt::Display for Cause {fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {f.write_str(&self.0)}}
impl Error for Cause {}
#[test]
fn graph_errors_keep_original_category_message_and_owned_cause(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=match row["kind"].as_str().unwrap(){"Json"=>NodeGraphErrorKind::Json,"Pack"=>NodeGraphErrorKind::Pack,"Dag"=>NodeGraphErrorKind::Dag,_=>unreachable!()};
  let message=row["message"].as_str().unwrap();let error=NodeGraphError::from_cause(kind,Cause(message.into()));
  assert_eq!(error.kind(),kind);assert_eq!(error.to_string(),message);assert_eq!(error.source().unwrap().to_string(),message);
 }
 eprintln!("[DEBUG] six NodeGraph diagnostic categories/messages retain the neutral corpus");
}
