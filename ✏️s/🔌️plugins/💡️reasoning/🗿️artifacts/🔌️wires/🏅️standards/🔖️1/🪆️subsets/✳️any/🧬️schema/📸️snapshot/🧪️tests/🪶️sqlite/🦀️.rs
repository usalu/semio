use super::*;
use dsl::schema::Number;
#[path="🧬️semantic/🦀️.rs"]
mod semantic;
fn native_fixture()->WiresSnapshot{
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let child=&f["content"];let target=&child["target"];let dialect=&target["dialect"];
 WiresSnapshot{content:store::ArtifactChild::new(child["childId"].as_str().unwrap().into(),store::os_io::ArtifactRef{artifact_id:target["artifactId"].as_str().unwrap().into(),dialect:store::os_io::ArtifactDialect{artifact_kind:dialect["artifactKind"].as_str().unwrap().into(),standard:dialect["standard"].as_str().unwrap().into(),subset:dialect["subset"].as_str().unwrap().into()}}),
 wires_fixture:dsl::DslValue::Object(vec![
 ("same".into(),dsl::DslValue::Number(Number::UInt(f["unsigned"][2].as_str().unwrap().parse().unwrap()))),
 ("same".into(),dsl::DslValue::Number(Number::Int(f["signed"][0].as_str().unwrap().parse().unwrap()))),
 ("".into(),dsl::DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(f["floatWords"][5].as_str().unwrap(),16).unwrap())))),
 ("Unicode 🪐\0".into(),dsl::DslValue::Bytes(f["byteValues"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect()))
 ]),
 meta:dsl::DslValue::Array(vec![dsl::DslValue::Null,dsl::DslValue::Bool(true),dsl::DslValue::String(f["texts"][1].as_str().unwrap().into())])}
}
fn assert_intrinsic(expected:&dsl::DslValue,actual:&dsl::DslValue){
 let mut pending=vec![(expected,actual)];while let Some((a,b))=pending.pop(){match(a,b){
 (dsl::DslValue::Null,dsl::DslValue::Null)=>{},
 (dsl::DslValue::Bool(a),dsl::DslValue::Bool(b))=>assert_eq!(a,b),
 (dsl::DslValue::Number(Number::UInt(a)),dsl::DslValue::Number(Number::UInt(b)))=>assert_eq!(a,b),
 (dsl::DslValue::Number(Number::Int(a)),dsl::DslValue::Number(Number::Int(b)))=>assert_eq!(a,b),
 (dsl::DslValue::Number(Number::Float(a)),dsl::DslValue::Number(Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),
 (dsl::DslValue::String(a),dsl::DslValue::String(b))=>assert_eq!(a,b),
 (dsl::DslValue::Bytes(a),dsl::DslValue::Bytes(b))=>assert_eq!(a,b),
 (dsl::DslValue::Array(a),dsl::DslValue::Array(b))=>{assert_eq!(a.len(),b.len());pending.extend(a.iter().zip(b));},
 (dsl::DslValue::Object(a),dsl::DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((name_a,value_a),(name_b,value_b))in a.iter().zip(b){assert_eq!(name_a,name_b);pending.push((value_a,value_b));}},
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
 let mut snapshot=native_fixture();snapshot.wires_fixture=dsl::DslValue::Null;snapshot.meta=dsl::DslValue::Null;
 let mut expected=dsl::__rt::DecodedFieldOwner::new(snapshot,<WiresSnapshot as dsl::FromValue>::retire_decoded);
 for _ in 0..fixture["deepLevels"].as_u64().unwrap(){let old=std::mem::replace(&mut expected.as_mut().meta,dsl::DslValue::Null);expected.as_mut().meta=dsl::DslValue::Array(vec![old]);}
 for payload in[store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(expected.as_mut())),store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(expected.as_mut()))]{
  let restored=match payload{store::os_io::IoPayload::Binary(bytes)=><WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),store::os_io::IoPayload::Text(text)=><WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),_=>panic!("owned native encoding")};
  let mut restored=dsl::__rt::DecodedFieldOwner::new(restored,<WiresSnapshot as dsl::FromValue>::retire_decoded);
  assert_intrinsic(&expected.as_mut().meta,&restored.as_mut().meta);
  assert_eq!(expected.as_mut().content,restored.as_mut().content);
 }
}
#[test]
fn sqlite_snapshot_wires_deep_actual_erased_native_records_have_no_syntax_depth_quota(){
 use store::sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding};
 let codec=<WiresSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().expect("Wires owning native pack must declare its actual SQLite capability");
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let mut expected=dsl::__rt::DecodedFieldOwner::new(native_fixture(),<WiresSnapshot as dsl::FromValue>::retire_decoded);
 for _ in 0..f["deepLevels"].as_u64().unwrap(){let old=std::mem::replace(&mut expected.as_mut().meta,dsl::DslValue::Null);expected.as_mut().meta=dsl::DslValue::Array(vec![old]);}
 let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.reasoning.wires".into(),standard:"1".into(),subset:"*".into()};let limits=SqliteDatabaseLimits::default();let mut reference=None;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(expected.as_mut())),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(expected.as_mut()))};
  let database=(codec.export)("s.reasoning.wires",&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
  if let Some(reference)=&reference{assert_eq!(&database,reference);}else{reference=Some(database.clone());}
  let encoded=(codec.import)("s.reasoning.wires",&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
  assert_eq!((codec.export)("s.reasoning.wires",&dialect,&encoded,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,database);
 }
}
