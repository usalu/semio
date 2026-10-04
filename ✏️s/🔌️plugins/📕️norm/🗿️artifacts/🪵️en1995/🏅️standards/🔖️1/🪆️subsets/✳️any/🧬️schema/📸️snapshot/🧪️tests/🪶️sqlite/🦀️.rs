//! 🪵️ Complete timber persistence, independent semantic SQL and actual declaration-owned I/O laws.
use super::En1995Snapshot;
#[test]
fn sqlite_snapshot_en1995_owned_native_decoder_controls_both_physical_payloads(){
 use store::io::IoPayload;
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let restored=En1995Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);
  assert!(En1995Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  assert!(En1995Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
  assert!(En1995Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits})).is_err());
 }
}
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SnapshotEncoding,SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteValue}};

fn fixture()->En1995Snapshot{semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&En1995Snapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn roundtrip(snapshot:&En1995Snapshot)->En1995Snapshot{let bytes=export_sqlite_database(&database(snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();En1995Snapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn set_every_word(snapshot:&mut En1995Snapshot,value:f64){
    for m in &mut snapshot.members{m.b_m=value;m.h_m=value;m.span_m=value;m.support_length_m=value;m.bearing_length_m=value;m.buckling_length_y_m=value;m.buckling_length_z_m=value;m.lateral_restraint_spacing_m=value;m.notch_depth_m=value;m.notch_distance_m=value;m.m_crit_nm=value;m.mass_kg_per_m=value;m.mass_kg_per_m2=value;m.damping_xi=value;m.fire_duration_s=value;m.bridge_n_obs=value;m.bridge_t_l_years=value;m.bridge_beta=value;m.bridge_a=value;m.bridge_b=value;m.bridge_crowd_per_m2=value;for a in &mut m.actions{a.q_line_n_per_m=value;a.f_point_n=value;a.m_k_nm=value;a.v_k_n=value;a.n_k_n=value;a.n_t_k_n=value;a.f_c90_k_n=value;}}
    for c in &mut snapshot.connections{c.diameter_m=value;c.spacing_m=value;c.edge_distance_m=value;c.end_distance_m=value;c.t1_m=value;c.t2_m=value;c.steel_plate_thickness_m=value;c.f_u_k=value;for a in &mut c.actions{a.f_k_n=value;}}
}

#[test]
fn sqlite_snapshot_en1995_actual_bare_factory_exposes_owned_relational_capability(){let codec=store::ArtifactCodec::bare::<En1995Snapshot,crate::En1995Mutation>(crate::EN1995_DOCUMENT_SCHEMA);assert!(codec.snapshot_sqlite.is_some(),"EN1995 has no owned relational capability");}

#[test]
fn sqlite_snapshot_en1995_all_neutral_fields_agree_with_serde_and_empty_domains(){let snapshot=fixture();assert_eq!(serde_json::to_value(&snapshot).unwrap(),serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());assert_eq!(roundtrip(&snapshot),snapshot);let mut empty=snapshot;empty.members.clear();empty.connections.clear();assert_eq!(database(&empty).tables.len(),5);assert_eq!(roundtrip(&empty),empty);}

#[test]
fn sqlite_snapshot_en1995_native_unsigned_widths_and_all_choices_agree_with_serde(){
    let choices:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🛂️choices.json")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    for(owner,field,width)in[("members","serviceClass","Unsigned8"),("connections","serviceClass","Unsigned8"),("connections","number","Unsigned32"),("connections","rows","Unsigned32"),("connections","shearPlanes","Unsigned32")]{
        for value in choices[format!("invalid{width}")].as_array().unwrap(){let mut invalid=fixture.clone();invalid[owner][0][field]=value.clone();let text=serde_json::to_string(&invalid).unwrap();assert!(serde_json::from_str::<En1995Snapshot>(&text).is_err());assert!(semio_framework_pack_json::from_json_str::<En1995Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());}
        for value in choices[format!("valid{width}")].as_array().unwrap(){let mut valid=fixture.clone();valid[owner][0][field]=value.clone();let text=serde_json::to_string(&valid).unwrap();assert_eq!(serde_json::from_str::<En1995Snapshot>(&text).unwrap(),semio_framework_pack_json::from_json_str::<En1995Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());}
    }
    for(field,root)in[("annex",true),("role",false),("support",false)]{
        for value in choices[field].as_array().unwrap(){let mut valid=fixture.clone();if root{valid[field]=value.clone();}else{valid["members"][0][field]=value.clone();}let text=serde_json::to_string(&valid).unwrap();let snapshot=semio_framework_pack_json::from_json_str::<En1995Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(serde_json::from_str::<En1995Snapshot>(&text).unwrap(),snapshot);assert_eq!(roundtrip(&snapshot),snapshot);}
        let mut invalid=fixture.clone();if root{invalid[field]=serde_json::Value::String("unknown".into());}else{invalid["members"][0][field]=serde_json::Value::String("unknown".into());}let text=serde_json::to_string(&invalid).unwrap();assert!(serde_json::from_str::<En1995Snapshot>(&text).is_err());assert!(semio_framework_pack_json::from_json_str::<En1995Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
}

#[test]
fn sqlite_snapshot_en1995_every_numeric_domain_preserves_all_neutral_binary64_words(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();for word in corpus["ieee754Binary64Bits"].as_array().unwrap(){let mut snapshot=fixture();set_every_word(&mut snapshot,f64::from_bits(word.as_str().unwrap().parse().unwrap()));assert_eq!(database(&roundtrip(&snapshot)),database(&snapshot));}}

#[test]
fn sqlite_snapshot_en1995_independent_sqlite_joins_and_domain_edits(){
    use std::{io::Write,process::{Command,Stdio}};
    let mut expected=fixture();let bytes=export_sqlite_database(&database(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';import{Buffer}from'node:buffer';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT m.label_de,a.kind,a.ordinal,a.q_line_n_per_m FROM en1995_member m JOIN en1995_member_action a ON a.member_id=m.id WHERE m.id=1 ORDER BY a.ordinal').all();if(rows.length!==2||rows[0].q_line_n_per_m!==2500.5||rows[1].q_line_n_per_m!==-2500.5||rows[0].kind!=='arbitrary persisted kind')throw Error('ordered timber relationships');const joints=db.query('SELECT c.fastener_type,a.f_k_n FROM en1995_connection c JOIN en1995_connection_action a ON a.connection_id=c.id WHERE c.id=1 ORDER BY a.ordinal').all();if(joints.length!==2||joints[0].f_k_n!==42000.25||joints[1].f_k_n!==-42000.25)throw Error('ordered connection relationships');const word=value=>{const b=Buffer.alloc(8);b.writeDoubleBE(value);return BigInt.asIntN(64,b.readBigUInt64BE())};db.run(\"UPDATE en1995_member_action SET q_line_n_per_m=42.5,q_line_n_per_m_ieee754_bits=?,q_line_n_per_m_ieee754_class='finite' WHERE id=1\",[word(42.5)]);db.run(\"UPDATE en1995_connection SET label_de='Geändert 世界',rows=4294967293,service_class=254 WHERE id=1\");db.run(\"UPDATE en1995_document SET annex='En'\");await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    expected.members[0].actions[0].q_line_n_per_m=42.5;expected.connections[0].label_de="Geändert 世界".into();expected.connections[0].rows=4294967293;expected.connections[0].service_class=254;expected.annex=crate::document::AnnexChoice::En;
    assert_eq!(En1995Snapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
}

#[test]
fn sqlite_snapshot_en1995_refuses_malformed_choices_widths_booleans_ordinals_parents_and_words(){
    let database=database(&fixture());for(table,index,column,value)in[("en1995_document",0,1,SqliteValue::Text("unknown".into())),("en1995_member",0,8,SqliteValue::Integer(256)),("en1995_member",0,6,SqliteValue::Text("unknown".into())),("en1995_member",0,9,SqliteValue::Text("unknown".into())),("en1995_member",0,1,SqliteValue::Integer(99)),("en1995_member",1,2,SqliteValue::Integer(0)),("en1995_member_action",0,1,SqliteValue::Integer(99)),("en1995_member_action",0,2,SqliteValue::Integer(99)),("en1995_connection",0,8,SqliteValue::Integer(-1)),("en1995_connection",0,10,SqliteValue::Integer(4294967296)),("en1995_connection",0,11,SqliteValue::Integer(-1)),("en1995_connection",0,19,SqliteValue::Integer(4294967296)),("en1995_connection",0,17,SqliteValue::Integer(2)),("en1995_connection",0,1,SqliteValue::Integer(99)),("en1995_connection",1,2,SqliteValue::Integer(0)),("en1995_connection_action",0,1,SqliteValue::Integer(99)),("en1995_connection_action",0,2,SqliteValue::Integer(99)),("en1995_member",0,32,SqliteValue::Text("nan".into()))]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows[index].values[column]=value;assert!(En1995Snapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}
    let mut invalid=database.clone();invalid.table_mut("en1995_document").unwrap().rows.clear();assert!(En1995Snapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_en1995_initial_and_nested_operations_support_cancellation_and_native_admission(){
    let mut snapshot=fixture();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()})).is_err());assert!(En1995Snapshot::from_sqlite_database(&database(&snapshot),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:0,..SqliteDatabaseLimits::default()})).is_err());
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..SqliteDatabaseLimits::default()})).unwrap_err().to_string().contains("native encoding exceeds file byte limit"));}
    let controls:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️control.json")).unwrap();snapshot.members[0].actions=vec![snapshot.members[0].actions[0].clone();controls["entityCount"].as_u64().unwrap()as usize];let cancel_at=controls["cancelAt"].as_u64().unwrap()as usize;
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut reached=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut|event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cancel_at{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);}
    let large=database(&snapshot);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=cancel_at{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());assert!(if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).is_err()}else{En1995Snapshot::from_sqlite_database(&large,&mut control).is_err()});assert!(reached);}
}

#[test]
fn sqlite_snapshot_en1995_large_owned_text_cancels_before_projection_reconstruction_and_encoding_copy(){let mut snapshot=fixture();snapshot.members[0].label_de="x".repeat(131073);let database=database(&snapshot);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative]{let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.total>=65536&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>En1995Snapshot::from_sqlite_database(&database,&mut control).is_err(),_=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err()};assert!(rejected);assert!(reached);}}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_en1995_actual_erased_codec_and_declaration_preserve_all_owned_words(){
    use store::io::{ArtifactDialect,IoPayload,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("norm").label("EN1995 owned SQLite").version("0.0.1").package_id("semio:norm").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let codec=store::document_codec(crate::EN1995_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.norm.en1995".into(),standard:"1".into(),subset:"*".into()};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();for word in corpus["ieee754Binary64Bits"].as_array().unwrap(){let mut snapshot=fixture();set_every_word(&mut snapshot,f64::from_bits(word.as_str().unwrap().parse().unwrap()));let expected=database(&snapshot);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(<En1995Snapshot as store::ArtifactPack>::encode_pack_with(&snapshot,&store::PackEncodeOptions::default()).unwrap()),SnapshotEncoding::Text=>IoPayload::Text(<En1995Snapshot as store::ArtifactDsl>::print_dsl(&snapshot))};let result=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(result,expected);let result=(provider.import)(&codec.schema,&dialect,result,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=match result{IoPayload::Binary(bytes)=><En1995Snapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=><En1995Snapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()};assert_eq!(database(&restored),expected);}}
    let snapshot=fixture();let output=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap();assert_eq!(io_import_sqlite_snapshot::<En1995Snapshot>(&dialect,&output.value,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);
}

#[test]
fn sqlite_snapshot_en1995_genuine_output_admits_exact_row_and_file_frontiers(){
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
  let restored=En1995Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}
