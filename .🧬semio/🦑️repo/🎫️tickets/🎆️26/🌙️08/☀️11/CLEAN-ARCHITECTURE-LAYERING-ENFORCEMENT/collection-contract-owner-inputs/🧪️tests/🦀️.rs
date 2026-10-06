//! 🧪️ The original additive collection witness binds the shared neutral corpus.
use super::{Identified,Patchable};
struct Item{id:String,value:i64}
impl Identified<String> for Item{fn id(&self)->&String{&self.id}}
impl Patchable<i64> for Item{fn apply_patch(&mut self,patch:&i64){self.value+=patch;}fn diff_patch(&self,other:&Self)->Option<i64>{let delta=other.value-self.value;if delta==0{None}else{Some(delta)}}}
#[test]
fn original_additive_item_obeys_shared_identity_and_patch_witness(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let id=row["id"].as_str().unwrap();let mut item=Item{id:id.into(),value:row["value"].as_i64().unwrap()};let other=Item{id:id.into(),value:row["other"].as_i64().unwrap()};let delta=item.diff_patch(&other);assert_eq!(delta,row["delta"].as_i64());assert_eq!(item.id(),id);if let Some(delta)=delta{item.apply_patch(&delta);}assert_eq!(item.value,other.value);assert_eq!(item.id(),id);}
 eprintln!("[DEBUG] six original additive collection witnesses retain stable identity");
}
