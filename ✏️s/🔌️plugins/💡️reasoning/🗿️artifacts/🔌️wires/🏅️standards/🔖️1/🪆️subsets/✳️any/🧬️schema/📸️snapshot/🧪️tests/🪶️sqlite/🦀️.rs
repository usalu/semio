use super::*;
use semio_framework_value::Number;
#[path="🧬️semantic/🦀️.rs"]
mod semantic;
fn native_fixture()->WiresSnapshot{
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let child=&f["content"];let target=&child["target"];let dialect=&target["dialect"];
 WiresSnapshot{content:store::ArtifactChild::new(child["childId"].as_str().unwrap().into(),store::os_io::ArtifactRef{artifact_id:target["artifactId"].as_str().unwrap().into(),dialect:store::os_io::ArtifactDialect{artifact_kind:dialect["artifactKind"].as_str().unwrap().into(),standard:dialect["standard"].as_str().unwrap().into(),subset:dialect["subset"].as_str().unwrap().into()}}),
 wires_fixture:semio_framework_value::DslValue::Object(vec![
 ("same".into(),semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(f["unsigned"][2].as_str().unwrap().parse().unwrap()))),
 ("same".into(),semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(f["signed"][0].as_str().unwrap().parse().unwrap()))),
 ("".into(),semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(u64::from_str_radix(f["floatWords"][5].as_str().unwrap(),16).unwrap())))),
 ("Unicode 🪐\0".into(),semio_framework_value::DslValue::Bytes(f["byteValues"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect()))
 ]),
 meta:semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::Null,semio_framework_value::DslValue::Bool(true),semio_framework_value::DslValue::String(f["texts"][1].as_str().unwrap().into())])}
}
fn assert_intrinsic(expected:&semio_framework_value::DslValue,actual:&semio_framework_value::DslValue){
 let mut pending=vec![(expected,actual)];while let Some((a,b))=pending.pop(){match(a,b){
 (semio_framework_value::DslValue::Null,semio_framework_value::DslValue::Null)=>{},
 (semio_framework_value::DslValue::Bool(a),semio_framework_value::DslValue::Bool(b))=>assert_eq!(a,b),
 (semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(a)),semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(b)))=>assert_eq!(a,b),
 (semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(a)),semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(b)))=>assert_eq!(a,b),
 (semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(a)),semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),
 (semio_framework_value::DslValue::String(a),semio_framework_value::DslValue::String(b))=>assert_eq!(a,b),
 (semio_framework_value::DslValue::Bytes(a),semio_framework_value::DslValue::Bytes(b))=>assert_eq!(a,b),
 (semio_framework_value::DslValue::Array(a),semio_framework_value::DslValue::Array(b))=>{assert_eq!(a.len(),b.len());pending.extend(a.iter().zip(b));},
 (semio_framework_value::DslValue::Object(a),semio_framework_value::DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((name_a,value_a),(name_b,value_b))in a.iter().zip(b){assert_eq!(name_a,name_b);pending.push((value_a,value_b));}},
 _=>panic!("native Wires intrinsic variant changed")
 }}
}
#[test]
fn sqlite_snapshot_wires_exact_native_records_preserve_complete_intrinsic_and_handle(){
 let expected=native_fixture();
 for payload in[store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&expected)),store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(&expected))]{
  let restored=match payload{store::os_io::IoPayload::Binary(bytes)=><WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),store::os_io::IoPayload::Text(text)=><WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),_=>panic!("owned native encoding")};
  assert_eq!(restored.content.child_id,expected.content.child_id);assert_eq!(restored.content.target,expected.content.target);
  assert_intrinsic(&expected.wires_fixture,&restored.wires_fixture);assert_intrinsic(&expected.meta,&restored.meta);
 }
}
#[test]
fn sqlite_snapshot_wires_deep_complete_native_records_preserve_allowed_intrinsic_depth(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let mut snapshot=native_fixture();snapshot.wires_fixture=semio_framework_value::DslValue::Null;snapshot.meta=semio_framework_value::DslValue::Null;
 let mut expected=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(snapshot,<WiresSnapshot as semio_framework_value::FromValue>::retire_decoded);
 for _ in 0..fixture["deepLevels"].as_u64().unwrap(){let old=std::mem::replace(&mut expected.as_mut().meta,semio_framework_value::DslValue::Null);expected.as_mut().meta=semio_framework_value::DslValue::Array(vec![old]);}
 for payload in[store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(expected.as_mut())),store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(expected.as_mut()))]{
  let restored=match payload{store::os_io::IoPayload::Binary(bytes)=><WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),store::os_io::IoPayload::Text(text)=><WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),_=>panic!("owned native encoding")};
  let mut restored=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(restored,<WiresSnapshot as semio_framework_value::FromValue>::retire_decoded);
  assert_intrinsic(&expected.as_mut().meta,&restored.as_mut().meta);
  assert_eq!(expected.as_mut().content,restored.as_mut().content);
 }
}
#[test]
fn sqlite_snapshot_wires_deep_actual_erased_native_records_have_no_syntax_depth_quota(){
 use store::sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding};
 let codec=<WiresSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().expect("Wires owning native pack must declare its actual SQLite capability");
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let mut expected=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(native_fixture(),<WiresSnapshot as semio_framework_value::FromValue>::retire_decoded);
 for _ in 0..f["deepLevels"].as_u64().unwrap(){let old=std::mem::replace(&mut expected.as_mut().meta,semio_framework_value::DslValue::Null);expected.as_mut().meta=semio_framework_value::DslValue::Array(vec![old]);}
 let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.reasoning.wires".into(),standard:"1".into(),subset:"*".into()};let limits=SqliteDatabaseLimits::default();let mut reference=None;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(expected.as_mut())),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(expected.as_mut()))};
  let database=(codec.export)("s.reasoning.wires",&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
  if let Some(reference)=&reference{assert_eq!(&database,reference);}else{reference=Some(database.clone());}
  let encoded=(codec.import)("s.reasoning.wires",&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
  assert_eq!((codec.export)("s.reasoning.wires",&dialect,&encoded,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,database);
 }
}

#[path="🧮️census/🦀️.rs"]
mod native_semantic_census;
