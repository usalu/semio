use super::*;
use semio_framework_value::{DslValue,FromValue};

#[derive(semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
struct FixedDofFixture { version:u32, order:Vec<String>, cases:Vec<FixedDofCase> }
#[derive(semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
struct FixedDofCase { id:String, input:DslValue, canonical:bool }

/// 🧪️ Both actual model crates mount this same cold law and neutral canonical corpus.
#[test]
fn fixed_dof_boundary_matches_neutral_canonical_sets(){
 let fixture:FixedDofFixture=semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 assert_eq!(fixture.version,1);assert_eq!(fixture.order,["Tx","Ty","Tz","Rx","Ry","Rz"]);assert_eq!(fixture.cases.len(),73);
 for row in fixture.cases{
  let result=decode_fixed_dofs(row.input.clone());
  let support=FemSupport::from_value(DslValue::Object(vec![("id".into(),DslValue::String("support".into())),("nodeId".into(),DslValue::String("node".into())),("fixed".into(),row.input)]));
  assert_eq!(result.is_ok(),row.canonical,"{} helper",row.id);assert_eq!(support.is_ok(),row.canonical,"{} support field",row.id);
  if let Ok(fixed)=result{assert!(FemDof::canonical(&fixed));assert_eq!(support.unwrap().fixed,fixed,"{} model field",row.id);retire_fixed_dofs(fixed);}
 }
 println!("[DEBUG] Actual model cold restraint helper and support field match73shared canonical boundaries");
}
