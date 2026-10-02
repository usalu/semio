//! 🌬️ Complete building subject fidelity, independent semantic SQL and actual declaration-owned I/O laws.
use super::Din16798Snapshot;
#[test]
fn sqlite_snapshot_din16798_owned_native_decoder_controls_both_physical_payloads(){
 use store::io::IoPayload;
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let restored=Din16798Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);
  assert!(Din16798Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  assert!(Din16798Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
  assert!(Din16798Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits})).is_err());
 }
}
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SnapshotEncoding,SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteValue}};

fn fixture()->Din16798Snapshot{store::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn database(snapshot:&Din16798Snapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn roundtrip(snapshot:&Din16798Snapshot)->Din16798Snapshot{let bytes=export_sqlite_database(&database(snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();Din16798Snapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn set_every_word(s:&mut Din16798Snapshot,value:f64){
    s.theta_rm_c=value;s.outdoor_co2_ppm=value;s.envelope_n50_h_inv=value;s.envelope_volume_m3=value;s.cellar_area_m2=value;s.cellar_ventilation_m3_h=value;s.night_setback_k=value;
    for z in &mut s.zones{z.floor_area_m2=value;z.t_op_winter_c=value;z.t_op_summer_c=value;z.air_speed_m_s=value;z.clothing_clo=value;z.metabolic_rate_met=value;z.rh_percent=value;z.outdoor_air_supplied_m3_h=value;z.co2_ppm=value;z.illuminance_lx=value;z.noise_db=value;z.turbulence_intensity_percent=value;}
    for v in &mut s.vent_systems{v.sfp_w_m3_s=value;v.heat_recovery_eta=value;v.humidification_required_kg_h=value;v.humidification_provided_kg_h=value;v.fan_q_v_m3_s=value;v.fan_t_run_h=value;v.duct_test_pressure_pa=value;v.duct_leakage_m3_s_m2=value;v.design_airflow_m3_h=value;}
}

#[test]
fn sqlite_snapshot_din16798_actual_bare_factory_exposes_owned_relational_capability(){let codec=store::ArtifactCodec::bare::<Din16798Snapshot,crate::Din16798Mutation>(crate::DIN16798_DOCUMENT_SCHEMA);assert!(codec.snapshot_sqlite.is_some(),"DIN16798 has no owned relational capability");}

#[test]
fn sqlite_snapshot_din16798_all_neutral_fields_agree_with_serde_and_empty_domains(){let snapshot=fixture();assert_eq!(serde_json::to_value(&snapshot).unwrap(),serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());assert_eq!(roundtrip(&snapshot),snapshot);let mut empty=snapshot;empty.zones.clear();empty.vent_systems.clear();assert_eq!(database(&empty).tables.len(),3);assert_eq!(roundtrip(&empty),empty);}

#[test]
fn sqlite_snapshot_din16798_native_unsigned_widths_and_annex_agree_with_serde(){
    let choices:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🛂️choices.json")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    for(owner,field,width)in[("zones","occupants","Unsigned32"),("ventSystems","yearsSinceInspection","Unsigned32"),("ventSystems","sfpRequiredClass","Unsigned8")]{
        for value in choices[format!("invalid{width}")].as_array().unwrap(){let mut invalid=fixture.clone();invalid[owner][0][field]=value.clone();let text=serde_json::to_string(&invalid).unwrap();assert!(serde_json::from_str::<Din16798Snapshot>(&text).is_err());assert!(store::json::from_json_str::<Din16798Snapshot>(&text).is_err());}
        for value in choices[format!("valid{width}")].as_array().unwrap(){let mut valid=fixture.clone();valid[owner][0][field]=value.clone();let text=serde_json::to_string(&valid).unwrap();assert_eq!(serde_json::from_str::<Din16798Snapshot>(&text).unwrap(),store::json::from_json_str::<Din16798Snapshot>(&text).unwrap());}
    }
    for value in choices["annex"].as_array().unwrap(){let mut valid=fixture.clone();valid["annex"]=value.clone();let text=serde_json::to_string(&valid).unwrap();let snapshot=store::json::from_json_str::<Din16798Snapshot>(&text).unwrap();assert_eq!(serde_json::from_str::<Din16798Snapshot>(&text).unwrap(),snapshot);assert_eq!(roundtrip(&snapshot),snapshot);}
    let mut invalid=fixture;invalid["annex"]=serde_json::Value::String("unknown".into());let text=serde_json::to_string(&invalid).unwrap();assert!(serde_json::from_str::<Din16798Snapshot>(&text).is_err());assert!(store::json::from_json_str::<Din16798Snapshot>(&text).is_err());
}

#[test]
fn sqlite_snapshot_din16798_every_numeric_domain_preserves_all_neutral_binary64_words(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();for word in corpus["ieee754Binary64Bits"].as_array().unwrap(){let mut snapshot=fixture();set_every_word(&mut snapshot,f64::from_bits(word.as_str().unwrap().parse().unwrap()));assert_eq!(database(&roundtrip(&snapshot)),database(&snapshot));}}

#[test]
fn sqlite_snapshot_din16798_independent_sqlite_joins_and_domain_edits(){
    use std::{io::Write,process::{Command,Stdio}};
    let mut expected=fixture();let bytes=export_sqlite_database(&database(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';import{Buffer}from'node:buffer';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT z.ordinal,z.floor_area_m2,v.ordinal AS system_ordinal,v.sfp_w_m3_s FROM din16798_zone z JOIN din16798_vent_system v ON v.logical_id=z.vent_system_logical_id WHERE z.id=1 ORDER BY v.ordinal').all();if(rows.length!==2||rows[0].floor_area_m2!==120.25||rows[0].sfp_w_m3_s!==1500.25||rows[1].sfp_w_m3_s!==-1500.25)throw Error('ordered ambiguous logical relationships');if(db.query('SELECT vent_system_logical_id FROM din16798_zone WHERE id=2').get().vent_system_logical_id!=='missing system')throw Error('missing logical target');const word=value=>{const b=Buffer.alloc(8);b.writeDoubleBE(value);return BigInt.asIntN(64,b.readBigUInt64BE())};db.run(\"UPDATE din16798_zone SET floor_area_m2=42.5,floor_area_m2_ieee754_bits=?,floor_area_m2_ieee754_class='finite',name='Geändert 世界',occupants=4294967294 WHERE id=1\",[word(42.5)]);db.run(\"UPDATE din16798_vent_system SET sfp_required_class=254,years_since_inspection=4294967293 WHERE id=1\");db.run(\"UPDATE din16798_document SET annex='En'\");await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    expected.zones[0].floor_area_m2=42.5;expected.zones[0].name="Geändert 世界".into();expected.zones[0].occupants=4294967294;expected.vent_systems[0].sfp_required_class=254;expected.vent_systems[0].years_since_inspection=4294967293;expected.annex=crate::document::AnnexChoice::En;
    assert_eq!(Din16798Snapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
}

#[test]
fn sqlite_snapshot_din16798_refuses_malformed_annex_widths_ordinals_parents_and_words(){
    let database=database(&fixture());for(table,index,column,value)in[("din16798_document",0,1,SqliteValue::Text("unknown".into())),("din16798_zone",0,7,SqliteValue::Integer(4294967296)),("din16798_vent_system",0,7,SqliteValue::Integer(256)),("din16798_vent_system",0,11,SqliteValue::Integer(-1)),("din16798_zone",0,1,SqliteValue::Integer(99)),("din16798_zone",1,2,SqliteValue::Integer(0)),("din16798_zone",0,2,SqliteValue::Integer(99)),("din16798_vent_system",0,1,SqliteValue::Integer(99)),("din16798_vent_system",1,2,SqliteValue::Integer(0)),("din16798_document",0,10,SqliteValue::Text("nan".into()))]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows[index].values[column]=value;assert!(Din16798Snapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}
    let mut invalid=database.clone();invalid.table_mut("din16798_document").unwrap().rows.clear();assert!(Din16798Snapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_din16798_initial_and_nested_operations_support_cancellation_and_native_admission(){
    let mut snapshot=fixture();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()})).is_err());assert!(Din16798Snapshot::from_sqlite_database(&database(&snapshot),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:0,..SqliteDatabaseLimits::default()})).is_err());
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..SqliteDatabaseLimits::default()})).unwrap_err().contains("native encoding exceeds file byte limit"));}
    let controls:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️control.json")).unwrap();snapshot.zones=vec![snapshot.zones[0].clone();controls["entityCount"].as_u64().unwrap()as usize];let cancel_at=controls["cancelAt"].as_u64().unwrap()as usize;
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut reached=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut|event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cancel_at{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);}
    let large=database(&snapshot);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=cancel_at{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());assert!(if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).is_err()}else{Din16798Snapshot::from_sqlite_database(&large,&mut control).is_err()});assert!(reached);}
}

#[test]
fn sqlite_snapshot_din16798_large_owned_text_cancels_before_projection_reconstruction_and_encoding_copy(){let mut snapshot=fixture();snapshot.zones[0].name="x".repeat(131073);let database=database(&snapshot);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative]{let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.total>=65536&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>Din16798Snapshot::from_sqlite_database(&database,&mut control).is_err(),_=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err()};assert!(rejected);assert!(reached);}}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_din16798_actual_erased_codec_and_declaration_preserve_all_owned_words(){
    use store::io::{ArtifactDialect,IoPayload,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("norm").label("DIN16798 owned SQLite").version("0.0.1").package_id("semio:norm").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let codec=store::document_codec(crate::DIN16798_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.norm.din16798".into(),standard:"1".into(),subset:"*".into()};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();for word in corpus["ieee754Binary64Bits"].as_array().unwrap(){let mut snapshot=fixture();set_every_word(&mut snapshot,f64::from_bits(word.as_str().unwrap().parse().unwrap()));let expected=database(&snapshot);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(<Din16798Snapshot as store::ArtifactPack>::encode_pack_with(&snapshot,&store::PackEncodeOptions::default()).unwrap()),SnapshotEncoding::Text=>IoPayload::Text(<Din16798Snapshot as store::ArtifactDsl>::print_dsl(&snapshot))};let result=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(result,expected);let result=(provider.import)(&codec.schema,&dialect,result,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=match result{IoPayload::Binary(bytes)=><Din16798Snapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=><Din16798Snapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()};assert_eq!(database(&restored),expected);}}
    let snapshot=fixture();let output=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap();assert_eq!(io_import_sqlite_snapshot::<Din16798Snapshot>(&dialect,&output.value,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);
}

#[test]
fn sqlite_snapshot_din16798_genuine_output_admits_exact_row_and_file_frontiers(){
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
  let restored=Din16798Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}
