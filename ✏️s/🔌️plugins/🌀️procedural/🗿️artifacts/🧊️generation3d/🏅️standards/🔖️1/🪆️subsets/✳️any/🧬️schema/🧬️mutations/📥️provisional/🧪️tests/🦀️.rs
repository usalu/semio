use super::*;
#[test]
fn every_provisional_mutation_matches_its_canonical_schema_wire(){
 fn native<T:semio_framework_value_native_owned::native_decoding::owned::NativeReceive<semio_framework_value::DslValue>>(){}
 native::<Generation3dMutation>();
 let rows:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for row in rows.as_array().unwrap(){
  let input=serde_json::to_string(&row["wire"]).unwrap();
  let decoded:Generation3dMutation=semio_framework_pack_json::from_json_str(&input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error|panic!("{}: {error}",row["name"]));
  let encoded=semio_framework_pack_json::to_json_string(&decoded);
  let actual:serde_json::Value=serde_json::from_str(&encoded).unwrap();
  let expected=row["expected"].clone();
  decoded.retire_cold();
  assert_eq!(actual,expected,"{}",row["name"]);
 }
}
