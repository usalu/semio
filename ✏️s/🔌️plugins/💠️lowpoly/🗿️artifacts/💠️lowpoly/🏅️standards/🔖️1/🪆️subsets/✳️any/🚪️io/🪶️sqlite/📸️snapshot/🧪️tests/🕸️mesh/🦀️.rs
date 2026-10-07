//! 🕸️ The actual Lowpoly owner must retain managed topology independently of literal source text.
use super::*;
use store::ArtifactSqliteSnapshot;
use semio_framework_value::{DslValue,FromValue,ToValue};


struct OrderedValue(DslValue);
impl<'de> serde::Deserialize<'de> for OrderedValue{
 fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{
  struct Ordered;
  impl<'de> serde::de::Visitor<'de> for Ordered{
   type Value=DslValue;
   fn expecting(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("the authored ordered intrinsic JSON")}
   fn visit_unit<E:serde::de::Error>(self)->Result<Self::Value,E>{Ok(DslValue::Null)}
   fn visit_none<E:serde::de::Error>(self)->Result<Self::Value,E>{Ok(DslValue::Null)}
   fn visit_bool<E:serde::de::Error>(self,value:bool)->Result<Self::Value,E>{Ok(DslValue::Bool(value))}
   fn visit_i64<E:serde::de::Error>(self,value:i64)->Result<Self::Value,E>{Ok(DslValue::int(value))}
   fn visit_u64<E:serde::de::Error>(self,value:u64)->Result<Self::Value,E>{Ok(DslValue::uint(value))}
   fn visit_f64<E:serde::de::Error>(self,value:f64)->Result<Self::Value,E>{Ok(DslValue::float(value))}
   fn visit_str<E:serde::de::Error>(self,value:&str)->Result<Self::Value,E>{Ok(DslValue::String(value.into()))}
   fn visit_string<E:serde::de::Error>(self,value:String)->Result<Self::Value,E>{Ok(DslValue::String(value))}
   fn visit_seq<A:serde::de::SeqAccess<'de>>(self,mut source:A)->Result<Self::Value,A::Error>{let mut values=Vec::new();while let Some(value)=source.next_element::<OrderedValue>()?{values.push(value.0);}Ok(DslValue::Array(values))}
   fn visit_map<A:serde::de::MapAccess<'de>>(self,mut source:A)->Result<Self::Value,A::Error>{let mut values=Vec::new();while let Some((name,value))=source.next_entry::<String,OrderedValue>()?{values.push((name,value.0));}Ok(DslValue::Object(values))}
  }
  deserializer.deserialize_any(Ordered).map(Self)
 }
}

#[test]
fn sqlite_snapshot_lowpoly_managed_mesh_complete_owner_and_independent_sql_edits(){
 let neutral=serde_json::from_str::<OrderedValue>(include_str!("../../🧫️fixtures/🕸️mesh/🔣️.json")).unwrap().0;
 let mut input=fixture().to_value();let DslValue::Object(document)=&mut input else{panic!("document owner")};let DslValue::Array(objects)=&mut document.iter_mut().find(|(name,_)|name=="objects").unwrap().1 else{panic!("ordered objects")};let DslValue::Object(object)=&mut objects[0]else{panic!("object owner")};object.iter_mut().find(|(name,_)|name=="meshState").expect("single authored managed mesh field").1=neutral["mesh"].clone();
 let expected=crate::standards::v1::subsets::any::io::text::lowpoly_json_bind::<LowpolySnapshot>(input).expect("declared full managed mesh owner");
 assert_eq!(expected.to_value()["objects"][0]["meshState"],neutral["mesh"].clone(),"actual owning construction must preserve all seven managed mesh fields");
 let limits=SqliteDatabaseLimits::default();let codec=<LowpolySnapshot as store::ArtifactPack>::sqlite_snapshot_codec().expect("real Lowpoly codec");let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.lowpoly.lowpoly".into(),standard:"1".into(),subset:"*".into()};
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&expected)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&expected))};
  let database=(codec.export)("s.lowpoly.lowpoly",&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
  let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();
  use std::{io::Write,process::{Command,Stdio}};
  let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(d.query('PRAGMA foreign_key_check').all().length)throw Error('foreign keys');if(d.query('SELECT COUNT(*) AS n FROM lowpoly_mesh_halfedge').get().n!==4n)throw Error('halfedges');if(d.query('SELECT twin_index,face_index FROM lowpoly_mesh_halfedge WHERE ordinal=3').get().face_index!==null)throw Error('optional topology');const kinds=d.query('SELECT kind FROM lowpoly_mesh_value').all().map(r=>r.kind);for(const k of['null','boolean','unsigned','signed','float','text','bytes','array','object'])if(!kinds.includes(k))throw Error(k);if(d.query('SELECT value FROM lowpoly_mesh_unsigned').get().value!=='18446744073709551615'||d.query('SELECT value FROM lowpoly_mesh_signed').get().value!==-9223372036854775808n)throw Error('integer width');if(d.query(\"SELECT COUNT(*) AS n FROM lowpoly_mesh_object_member WHERE name='same'\").get().n!==2n)throw Error('duplicate ordered members');d.exec(\"UPDATE lowpoly_mesh_vertex SET position_x=2.5,position_x_ieee754_bits=1075838976,position_x_numeric_class='finite' WHERE ordinal=0\");d.exec('UPDATE lowpoly_mesh_texture_octet SET value=17 WHERE ordinal=0');await Bun.write(Bun.stdout,d.serialize());d.close();";
  let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();let written=child.stdin.take().unwrap().write_all(&bytes);let output=child.wait_with_output().unwrap();assert!(written.is_ok()&&output.status.success(),"write={written:?}; {}",String::from_utf8_lossy(&output.stderr));
  let edited=import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();let result=(codec.import)("s.lowpoly.lowpoly",&dialect,edited.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;let restored=LowpolySnapshot::decode_sqlite_snapshot_native(&result,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let value=restored.to_value();assert_eq!(value["objects"][0]["meshState"]["vertices"][0]["position"][0],DslValue::String("40200000".into()));assert_eq!(value["objects"][0]["meshContent"],expected.to_value()["objects"][0]["meshContent"]);assert_eq!((codec.export)("s.lowpoly.lowpoly",&dialect,&result,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,edited);
 }
}
