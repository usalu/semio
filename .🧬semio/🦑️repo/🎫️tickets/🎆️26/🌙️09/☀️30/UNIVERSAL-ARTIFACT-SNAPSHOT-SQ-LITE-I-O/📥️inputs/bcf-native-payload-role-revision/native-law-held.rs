use super::*;
#[test]
fn sqlite_snapshot_bcf_manual_variant_metadata_retains_neutral_choices_under_both_controls(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏭️schema/🔣️.json")).unwrap();let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;let tiny=fixture["tinyBytes"].as_u64().unwrap()as usize;
 let mut admitted=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(maximum,&mut admitted);let semio_framework_dsl_record::Shape::Statements(encoded)=<BcfCamera as semio_framework_dsl_record::DslField>::shape_controlled(&mut encoding).unwrap()else{panic!("declared choices")};let mut output=Vec::new();for(keyword,producer)in &encoded{let record=producer.encode(&mut encoding).unwrap();output.push(serde_json::json!([keyword,record.fields.len()]));}assert_eq!(serde_json::json!(output),fixture["variants"]);let exact=encoding.owned_bytes();assert!(exact>0);
 let mut admitted=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(maximum,&mut admitted);let semio_framework_dsl_record::Shape::Statements(decoded)=<BcfCamera as semio_framework_dsl_record::DslField>::shape_controlled(&mut decoding).unwrap()else{panic!("declared choices")};let mut output=Vec::new();for(keyword,producer)in &decoded{let record=producer.decode(&mut decoding).unwrap();output.push(serde_json::json!([keyword,record.fields.len()]));}assert_eq!(serde_json::json!(output),fixture["variants"]);
 let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(tiny,&mut admitted);assert!(<BcfCamera as semio_framework_dsl_record::DslField>::shape_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);assert!(<BcfCamera as semio_framework_dsl_record::DslField>::shape_controlled(&mut semio_framework_value::NativeDecodeControl::new(tiny,&mut |_|true)).is_err());assert!(<BcfCamera as semio_framework_dsl_record::DslField>::shape_controlled(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut |_|false)).is_err());assert!(<BcfCamera as semio_framework_dsl_record::DslField>::shape_controlled(&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut |_|false)).is_err());
}
#[test]
fn sqlite_snapshot_bcf_controlled_native_owner_preserves_full_fixture_and_enforces_caller_limits(){
 use semio_framework_os_kernel::{io::IoPayload,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl},ArtifactSqliteSnapshot};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for payload in[IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))]{
  assert_eq!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
  assert!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  assert!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
  assert!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits})).is_err());
 }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_bcf_actual_erased_codec_preserves_every_owned_binary64_word() {
    use semio_framework_os_kernel::{io::{ArtifactDialect, IoPayload}, sqlite_snapshot::SnapshotEncoding};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("BCF binary64 SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let codec = store::document_codec("stdio.bcf").await.unwrap().unwrap();
    let provider = codec.snapshot_sqlite.as_ref().unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.bcf".into(), standard: "2.1".into(), subset: "*".into() };
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();
    for word in corpus["ieee754Binary64Bits"].as_array().unwrap() {
        let value = f64::from_bits(word.as_str().unwrap().parse().unwrap());
        let point = BcfPoint3 { x: value, y: value, z: value };
        let mut snapshot = fixture();
        snapshot.topics[0].viewpoints[0].camera = Some(BcfCamera::Perspective { view_point: point, direction: point, up_vector: point, field_of_view: value });
        snapshot.topics[0].viewpoints[1].camera = Some(BcfCamera::Orthogonal { view_point: point, direction: point, up_vector: point, view_to_world_scale: value });
        let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = match encoding {
                SnapshotEncoding::Binary => IoPayload::Binary(<BcfSnapshot as store::ArtifactPack>::encode_pack_with(&snapshot, &store::PackEncodeOptions::default()).unwrap()),
                SnapshotEncoding::Text => IoPayload::Text(<BcfSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)),
            };
            let database = (provider.export)(&codec.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database, expected);
            let restored = (provider.import)(&codec.schema, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let restored = match restored {
                IoPayload::Binary(bytes) => <BcfSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),
                IoPayload::Text(text) => <BcfSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),
            };
            assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), expected);
        }
    }
}


#[test]
fn sqlite_snapshot_bcf_native_factory_has_the_actual_structural_identity() {
    let codec = (crate::native_codecs()[0].codec)();
    let actual = store::ArtifactCodec::bare::<BcfSnapshot, crate::BcfMutation>(crate::STDIO_BCF_DOCUMENT_SCHEMA);
    assert_eq!(semio_framework_hash::hex_lower(&codec.pack_schema_hash), semio_framework_hash::hex_lower(&actual.pack_schema_hash));
    let manifest: serde_json::Value = serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).unwrap();
    assert_eq!(semio_framework_hash::hex_lower(&codec.pack_schema_hash), manifest["codecs"][0]["native_factory"]["pack_schema_hash"].as_str().unwrap());
}
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteValue},ArtifactSqliteSnapshot};

fn fixture()->BcfSnapshot{semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn roundtrip(snapshot:&BcfSnapshot)->BcfSnapshot{let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();BcfSnapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}

#[test]
fn sqlite_snapshot_bcf_full_neutral_corpus_and_optional_empty_domains(){
 let snapshot=fixture();assert_eq!(roundtrip(&snapshot),snapshot);let oracle:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();assert_eq!(oracle,serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());assert_eq!(roundtrip(&BcfSnapshot{schema:"".into(),version:"".into(),topics:Vec::new(),parts:Vec::new()}),BcfSnapshot{schema:"".into(),version:"".into(),topics:Vec::new(),parts:Vec::new()});
}

#[test]
fn sqlite_snapshot_bcf_every_camera_binary64_word_survives_without_native_codec(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();for word in corpus["ieee754Binary64Bits"].as_array().unwrap(){let bits:u64=word.as_str().unwrap().parse().unwrap();let value=f64::from_bits(bits);let point=BcfPoint3{x:value,y:value,z:value};let mut snapshot=fixture();snapshot.topics[0].viewpoints[0].camera=Some(BcfCamera::Perspective{view_point:point,direction:point,up_vector:point,field_of_view:value});snapshot.topics[0].viewpoints[1].camera=Some(BcfCamera::Orthogonal{view_point:point,direction:point,up_vector:point,view_to_world_scale:value});let result=roundtrip(&snapshot);for view in &result.topics[0].viewpoints[..2]{let(point,direction,up_vector,parameter)=match view.camera.as_ref().unwrap(){BcfCamera::Perspective{view_point,direction,up_vector,field_of_view}=>(view_point,direction,up_vector,*field_of_view),BcfCamera::Orthogonal{view_point,direction,up_vector,view_to_world_scale}=>(view_point,direction,up_vector,*view_to_world_scale)};for value in [point.x,point.y,point.z,direction.x,direction.y,direction.z,up_vector.x,up_vector.y,up_vector.z,parameter]{assert_eq!(value.to_bits(),bits);}}}
}

#[test]
fn sqlite_snapshot_bcf_independent_sqlite_joins_edits_and_integrity(){
 use std::{io::Write,process::{Command,Stdio}};
 let mut expected=fixture();let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT t.title,v.guid,p.field_of_view,c.color,s.component_guid FROM bcf_topic t JOIN bcf_viewpoint v ON v.topic_id=t.id JOIN bcf_perspective_camera p ON p.id=v.id JOIN bcf_components x ON x.id=v.id JOIN bcf_coloring c ON c.components_id=x.id JOIN bcf_coloring_component s ON s.coloring_id=c.id WHERE v.ordinal=0 AND c.ordinal=0 AND s.ordinal=0').get();if(row.field_of_view!==60.5||row.color!=='FFFF0000'||row.component_guid!=='colored')throw Error('domain');db.run(\"UPDATE bcf_comment SET text='edited 世界',viewpoint_reference='' WHERE id=1\");db.run(\"UPDATE bcf_perspective_camera SET field_of_view=42.5,field_of_view_ieee754_bits=4631178160564600832,field_of_view_ieee754_class='finite' WHERE id=1\");db.run(\"UPDATE bcf_coloring_component SET component_guid='changed' WHERE id=1\");db.run(\"UPDATE bcf_snapshot_image SET image_bytes=x'00ff' WHERE id=2\");await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));expected.topics[0].comments[0].text="edited 世界".into();expected.topics[0].comments[0].viewpoint_ref=Some("".into());if let Some(BcfCamera::Perspective{field_of_view,..})=&mut expected.topics[0].viewpoints[0].camera{*field_of_view=42.5;}expected.topics[0].viewpoints[0].components.as_mut().unwrap().coloring[0].components[0]="changed".into();expected.topics[0].viewpoints[1].snapshot=Some(vec![0,255]);assert_eq!(BcfSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
}

#[test]
fn sqlite_snapshot_bcf_refuses_malformed_owner_order_choice_and_limits(){
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for(table,row,column,value)in [("bcf_topic",0,1,SqliteValue::Integer(99)),("bcf_topic_label",0,2,SqliteValue::Integer(99)),("bcf_comment",0,1,SqliteValue::Integer(99)),("bcf_components",0,1,SqliteValue::Integer(2)),("bcf_camera",0,1,SqliteValue::Text("unknown".into())),("bcf_perspective_camera",0,3,SqliteValue::Text("nan".into())),("bcf_raw_part",0,2,SqliteValue::Integer(99))]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows[row].values[column]=value;assert!(BcfSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}for table in ["bcf_camera","bcf_perspective_camera","bcf_components"]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows.clear();assert!(BcfSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()})).is_err());assert!(BcfSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:0,..SqliteDatabaseLimits::default()})).is_err());let mut large=snapshot;large.topics[0].labels=vec!["label".into();2000];let mut reached=false;assert!(large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |event|{if event.completed>=256{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_bcf_actual_declaration_preserves_owned_fields(){
 use semio_framework_os_kernel::{io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}},sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase}};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("BCF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let snapshot=fixture();let dialect=ArtifactDialect{artifact_kind:"s.stdio.bcf".into(),standard:"2.1".into(),subset:"*".into()};let mut phases=Vec::new();let output=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap();assert_eq!(io_import_sqlite_snapshot::<BcfSnapshot>(&dialect,&output.value,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
}

#[test]
fn sqlite_snapshot_bcf_large_intrinsic_image_copies_cancel_before_owned_allocation(){
 use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
 let controls:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️control.json")).unwrap();let mut snapshot=fixture();snapshot.topics[0].viewpoints[0].snapshot=Some(vec![255;controls["largeBlobBytes"].as_u64().unwrap() as usize]);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut reached=false;let mut callback=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>0&&event.total==0{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let refused=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).is_err()}else{BcfSnapshot::from_sqlite_database(&database,&mut control).is_err()};assert!(refused);assert!(reached);}
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_bcf_actual_erased_capability_preserves_owned_binary_and_text(){use semio_framework_os_kernel::{io::{ArtifactDialect,IoPayload},sqlite_snapshot::SnapshotEncoding};let mut snapshot=fixture();snapshot.schema="owned 世界\0".into();semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("bcf erased SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let codec=store::document_codec(<BcfSnapshot as store::ArtifactDsl>::envelope_id()).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.stdio.bcf".into(),standard:"2.1".into(),subset:"*".into()};let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(<BcfSnapshot as store::ArtifactPack>::encode_pack_with(&snapshot,&store::PackEncodeOptions::default()).unwrap()),SnapshotEncoding::Text=>IoPayload::Text(<BcfSnapshot as store::ArtifactDsl>::print_dsl(&snapshot))};let database=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(database,expected);let restored=(provider.import)(&codec.schema,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=match restored{IoPayload::Binary(bytes)=><BcfSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=><BcfSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()};assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);}}

#[test]
fn sqlite_snapshot_bcf_owned_encoding_preflight_checks_bounds_before_allocation(){use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut snapshot=fixture();snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..SqliteDatabaseLimits::default()})).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);snapshot.parts[0].data=vec![255;131073];let mut reached=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>0{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);}}

#[test]
fn sqlite_snapshot_bcf_genuine_output_admits_exact_row_and_file_frontiers(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let rows=expected.tables.iter().map(|table|table.rows.len()).sum::<usize>();assert!(rows>0);
 let bytes=export_sqlite_database(&expected,limits,&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();const rows=tables.reduce((n,t)=>n+Number(db.query('SELECT COUNT(*) AS count FROM "'+t.name.replaceAll('"','""')+'"').get().count),0);await Bun.write(Bun.stdout,String(rows));db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap().parse::<usize>().unwrap(),rows);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits})).unwrap();
  let physical=match &payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};assert!(physical>0);
  let repeated=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,max_file_bytes:physical,..limits})).unwrap();assert_eq!(repeated,payload);
  let restored=BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}

#[test]
fn sqlite_snapshot_bcf_native_payload_extent_is_distinct_from_relational_cells() {
 fn bytes(value:&serde_json::Value,key:&str)->usize {
  match value {
   serde_json::Value::Null=>0,
   serde_json::Value::Bool(_)=>1,
   serde_json::Value::Number(_)=>8,
   serde_json::Value::String(text)=>if key=="kind" {1} else {text.len()},
   serde_json::Value::Array(values)=>if matches!(key,"data"|"snapshot") {values.len()} else {values.iter().map(|value|bytes(value,"")).sum()},
   serde_json::Value::Object(fields)=>fields.iter().map(|(key,value)|bytes(value,key)).sum(),
  }
 }
 let source:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let controls:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️control.json")).unwrap();
 let exact=controls["nativePayloadBytes"].as_u64().unwrap() as usize;
 assert_eq!(bytes(&source,""),exact);
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();
 for encoding in [store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text] {
  let admitted=SqliteDatabaseLimits{max_value_bytes:exact,max_rows:rows,..limits};
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let relational:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️semantic.json")).unwrap();assert!(exact<(relational["cases"][0]["valueBytes"].as_u64().unwrap()as usize));
  assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,admitted)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
  assert_eq!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,admitted)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
  let refused=SqliteDatabaseLimits{max_value_bytes:exact-1,..limits};
  assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,refused)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
  assert_eq!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,refused)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
  assert!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows-1,..limits})).is_err());
  eprintln!("[DEBUG] BCF {encoding:?}: original native payload {exact} bytes is distinct from full relational cells; both original {exact} and one-less grants refuse");
 }
}

fn norm_complete_source(case:&serde_json::Value)->BcfSnapshot{let mut source=fixture();if case["id"]=="metadataRetainingEmpty"{source.topics.clear();}source}
fn norm_independent_complete_extent(source:&BcfSnapshot)->serde_json::Value{
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_bcf_complete_independent_native_semantic_limits(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:15,max_columns:29,..SqliteDatabaseLimits::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(BcfSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:14,..limits},SqliteDatabaseLimits{max_columns:28,..limits}]{assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"relational copied limits {short:?}");assert!(BcfSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }eprintln!("[DEBUG] BCF independently measured complete copied native semantic limits");
}
#[test]
fn sqlite_snapshot_bcf_complete_independent_copied_columns_admission(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let source=fixture();let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_columns:28,..SqliteDatabaseLimits::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(BcfSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"encoder copied columns {encoding:?}");assert!(BcfSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"decoder copied columns {encoding:?}");}
}

/// 🔮️ Interprets every actual public-file cell and declared table through independent SQLite.
fn norm_independent_public_file_extent(bytes:&[u8])->serde_json::Value{use std::{io::Write,process::{Command,Stdio}};let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_bcf_complete_actual_typed_public_io_limits(){
 use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};use store::sqlite_snapshot::*;
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("bcf complete public file scope").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.bcf".into(),standard:"2.1".into(),subset:"*".into()};let contract:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️semantic.json")).unwrap();let public=&contract["publicIo"];
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let domain=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:15,max_columns:29,..SqliteDatabaseLimits::default()};let expected=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,domain)).unwrap();let public_case=public["cases"].as_array().unwrap().iter().find(|value|value["id"]==case["id"]).unwrap();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let encoding_name=match encoding{SnapshotEncoding::Binary=>"binary",SnapshotEncoding::Text=>"text"};let limits=SqliteDatabaseLimits{max_rows:public_case["rows"].as_u64().unwrap()as usize,max_value_bytes:public_case["valueBytes"][encoding_name].as_u64().unwrap()as usize,max_schema_bytes:public["schemaBytes"].as_u64().unwrap()as usize,max_tables:16,max_columns:29,..domain};let file=io_export_sqlite_snapshot(&dialect,&source,encoding,limits,&mut |_|true).await.unwrap().value;assert_eq!(&file[..16],b"SQLite format 3\0");assert_eq!(norm_independent_public_file_extent(&file),serde_json::json!({"rows":public_case["rows"],"valueBytes":public_case["valueBytes"][encoding_name],"schemaBytes":public["schemaBytes"],"tableWidths":public["tableWidths"]}));let restored=io_import_sqlite_snapshot::<BcfSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value;assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:15,..limits},SqliteDatabaseLimits{max_columns:28,..limits}]{assert!(io_export_sqlite_snapshot(&dialect,&source,encoding,short,&mut |_|true).await.is_err());assert!(io_import_sqlite_snapshot::<BcfSnapshot>(&dialect,&file,short,&mut |_|true).await.is_err());}
  }
 }eprintln!("[DEBUG] bcf actual registered typed IO preserved independently complete public files and all five public-file limits");
}
