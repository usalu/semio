//! ⚖️ The exact EN1990 factory must expose its owned relational snapshot provider.

#[test]
fn sqlite_snapshot_en1990_actual_bare_factory_exposes_owned_relational_capability() {
    let codec = store::ArtifactCodec::bare::<crate::standards::v1::subsets::any::io::sqlite::snapshot::En1990Snapshot, crate::En1990Mutation>(crate::EN1990_DOCUMENT_SCHEMA);
    assert!(codec.snapshot_sqlite.is_some(), "EN1990 has no owned relational capability");
}

use crate::standards::v1::subsets::any::io::sqlite::snapshot::En1990Snapshot;
#[test]
fn sqlite_snapshot_en1990_owned_native_decoder_controls_both_physical_payloads(){
 use store::io::IoPayload;
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let restored=En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);
  assert!(En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  assert!(En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
  assert!(En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits})).is_err());
 }
}
use store::{ArtifactSqliteSnapshot, sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}};

fn fixture() -> En1990Snapshot { semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }
fn database(snapshot: &En1990Snapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn roundtrip(snapshot: &En1990Snapshot) -> En1990Snapshot {
    let bytes = export_sqlite_database(&database(snapshot), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    En1990Snapshot::from_sqlite_database(&import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap()
}
fn set_every_word(snapshot: &mut En1990Snapshot, value: f64) {
    snapshot.altitude_m = value; snapshot.design_working_life_years = value; snapshot.reference_period_years = value; snapshot.k_fi_declared = value; snapshot.beta_computed = value;
    for action in &mut snapshot.permanents { action.gk = value; }
    for action in &mut snapshot.variables { action.qk = value; }
    for action in &mut snapshot.accidentals { action.ad = value; }
    for action in &mut snapshot.seismics { action.a_ek = value; }
    for member in &mut snapshot.members { member.rd_str = value; member.rd_geo = value; member.rd_equ_stab = value; member.rd_equ_destab = value; member.rd_fat = value; member.span = value; member.deflection_w = value; member.deflection_limit_ratio = value; member.vibration_frequency = value; member.vibration_frequency_min = value; }
    for bridge in &mut snapshot.bridge_sls { bridge.deck_acceleration = value; bridge.deck_acceleration_limit = value; bridge.deck_twist = value; bridge.deck_twist_limit = value; bridge.bridge_deflection = value; bridge.bridge_deflection_limit = value; }
    for effect in &mut snapshot.effects { effect.influence = value; }
}

#[test]
fn sqlite_snapshot_en1990_all_neutral_fields_and_empty_owned_domains() {
    let snapshot = fixture();
    assert_eq!(roundtrip(&snapshot), snapshot);
    assert_eq!(serde_json::to_value(&snapshot).unwrap(), serde_json::from_str::<serde_json::Value>(include_str!("../🧫️fixtures/🔣️.json")).unwrap());
    let mut empty = snapshot;
    empty.project_id.clear(); empty.permanents.clear(); empty.variables.clear(); empty.accidentals.clear(); empty.seismics.clear(); empty.members.clear(); empty.bridge_sls.clear(); empty.effects.clear();
    assert_eq!(database(&empty).tables.len(), 9);
    assert_eq!(roundtrip(&empty), empty);
}

#[test]
fn sqlite_snapshot_en1990_neutral_unsigned8_widths_agree_with_serde() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛂️invalid-u8.json")).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for field in corpus["fields"].as_array().unwrap() {
        let field = field.as_str().unwrap();
        for value in corpus["invalidValues"].as_array().unwrap() { let mut invalid = fixture.clone(); invalid[field] = value.clone(); let text = serde_json::to_string(&invalid).unwrap(); assert!(serde_json::from_str::<En1990Snapshot>(&text).is_err()); assert!(semio_framework_pack_json::from_json_str::<En1990Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err()); }
        for value in corpus["validValues"].as_array().unwrap() { let mut valid = fixture.clone(); valid[field] = value.clone(); let text = serde_json::to_string(&valid).unwrap(); assert_eq!(serde_json::from_str::<En1990Snapshot>(&text).unwrap(), semio_framework_pack_json::from_json_str::<En1990Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()); }
    }
}

#[test]
fn sqlite_snapshot_en1990_every_numeric_domain_retains_each_neutral_binary64_word() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️binary64.json")).unwrap();
    for word in corpus["ieee754Binary64Bits"].as_array().unwrap() {
        let bits = word.as_str().unwrap().parse::<u64>().unwrap(); let mut snapshot = fixture(); set_every_word(&mut snapshot, f64::from_bits(bits));
        assert_eq!(database(&roundtrip(&snapshot)), database(&snapshot));
    }
}

#[test]
fn sqlite_snapshot_en1990_independent_sqlite_joins_and_full_domain_edits() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut expected = fixture();
    let bytes = export_sqlite_database(&database(&expected), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import{Database}from'bun:sqlite';import{Buffer}from'node:buffer';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT m.label_de,a.logical_id,v.category,e.influence FROM en1990_effect e JOIN en1990_member m ON m.id=e.resolved_member_id JOIN en1990_action a ON a.id=e.resolved_action_id JOIN en1990_variable_action v ON v.id=a.id').get();if(row.label_de!=='Träger'||row.logical_id!=='Q'||row.category!=='office'||row.influence!==1.25||db.query('SELECT count(*) AS n FROM en1990_effect WHERE resolved_member_id IS NULL AND resolved_action_id IS NULL').get().n!==2)throw Error('domain relationships');const word=value=>{const b=Buffer.alloc(8);b.writeDoubleBE(value);return BigInt.asIntN(64,b.readBigUInt64BE())};db.run(\"UPDATE en1990_variable_action SET qk=42.5,qk_ieee754_bits=?,qk_ieee754_class='finite' WHERE id=3\",word(42.5));db.run(\"UPDATE en1990_member SET label_de='Geändert 世界' WHERE id=1\");db.run(\"UPDATE en1990_bridge_sls SET deck_twist=-7.25,deck_twist_ieee754_bits=?,deck_twist_ieee754_class='finite' WHERE id=1\",word(-7.25));await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    expected.variables[0].qk = 42.5; expected.members[0].label_de = "Geändert 世界".into(); expected.bridge_sls[0].deck_twist = -7.25;
    assert_eq!(En1990Snapshot::from_sqlite_database(&import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), expected);
}

#[test]
fn sqlite_snapshot_en1990_refuses_malformed_choice_coverage_and_semantic_relationships() {
    let database = database(&fixture());
    for (table, index, column, value) in [("en1990_document", 0, 5, SqliteValue::Integer(256)), ("en1990_document", 0, 1, SqliteValue::Text("unknown".into())), ("en1990_action", 0, 2, SqliteValue::Text("unknown".into())), ("en1990_action", 0, 1, SqliteValue::Integer(99)), ("en1990_action", 0, 3, SqliteValue::Integer(99)), ("en1990_action", 1, 3, SqliteValue::Integer(0)), ("en1990_seismic_action", 0, 2, SqliteValue::Text("V".into())), ("en1990_member", 0, 2, SqliteValue::Integer(99)), ("en1990_bridge_sls", 0, 5, SqliteValue::Integer(2)), ("en1990_effect", 1, 6, SqliteValue::Integer(1)), ("en1990_effect", 0, 5, SqliteValue::Null), ("en1990_member", 0, 17, SqliteValue::Text("nan".into()))] {
        let mut invalid = database.clone(); invalid.table_mut(table).unwrap().rows[index].values[column] = value;
        assert!(En1990Snapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "{table}");
    }
    for table in ["en1990_document", "en1990_seismic_action"] { let mut invalid = database.clone(); invalid.table_mut(table).unwrap().rows.clear(); assert!(En1990Snapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err()); }
}

#[test]
fn sqlite_snapshot_en1990_initial_cancellation_and_borrowed_native_admission() {
    let mut snapshot = fixture();
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: 1, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(En1990Snapshot::from_sqlite_database(&database(&snapshot), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() })).is_err());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: 1024, ..SqliteDatabaseLimits::default() })).unwrap_err().to_string().contains("native encoding exceeds file byte limit"));
    }
    let controls: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎛️control.json")).unwrap();
    snapshot.members = vec![snapshot.members[0].clone(); controls["entityCount"].as_u64().unwrap() as usize];
    let cancel_at = controls["cancelAt"].as_u64().unwrap() as usize;
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { let mut reached = false; assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |event| { if event.phase == SqliteSnapshotPhase::EncodeNative && event.completed >= cancel_at { reached = true; false } else { true } }, SqliteDatabaseLimits::default())).is_err()); assert!(reached); }
    let large = database(&snapshot);
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] { let mut reached = false; let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if event.phase == phase && event.completed >= cancel_at { reached = true; false } else { true } }; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()); assert!(if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).is_err() } else { En1990Snapshot::from_sqlite_database(&large, &mut control).is_err() }); assert!(reached); }
}

#[test]
fn sqlite_snapshot_en1990_large_owned_text_cancels_before_projection_reconstruction_and_encoding_copy() {
    let mut snapshot = fixture(); snapshot.project_id = "x".repeat(131073); let database = database(&snapshot);
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::EncodeNative] { let mut reached = false; let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if event.phase == phase && event.total>=65536&&event.completed>=65536&&event.completed<event.total { reached = true; false } else { true } }; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()); let rejected = match phase { SqliteSnapshotPhase::ProjectSnapshot => snapshot.to_sqlite_database(&mut control).is_err(), SqliteSnapshotPhase::ReconstructSnapshot => En1990Snapshot::from_sqlite_database(&database, &mut control).is_err(), _ => snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut control).is_err() }; assert!(rejected); assert!(reached); }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_en1990_actual_erased_codec_and_declaration_preserve_all_owned_words() {
    use {semio_framework_artifact_reference::ArtifactDialect,store::io::IoPayload,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("norm").label("EN1990 owned SQLite").version("0.0.1").package_id("semio:norm").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let codec = store::document_codec(crate::EN1990_DOCUMENT_SCHEMA).await.unwrap().unwrap(); let provider = codec.snapshot_sqlite.as_ref().unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.norm.en1990".into(), standard: "1".into(), subset: "*".into() };
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️binary64.json")).unwrap();
    for word in corpus["ieee754Binary64Bits"].as_array().unwrap() {
        let mut snapshot = fixture(); set_every_word(&mut snapshot, f64::from_bits(word.as_str().unwrap().parse().unwrap())); let expected = database(&snapshot);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = match encoding { SnapshotEncoding::Binary => IoPayload::Binary(<En1990Snapshot as store::ArtifactPack>::encode_pack_with(&snapshot, &store::PackEncodeOptions::default()).unwrap()), SnapshotEncoding::Text => IoPayload::Text(<En1990Snapshot as store::ArtifactDsl>::print_dsl(&snapshot)) };
            let result = (provider.export)(&codec.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value; assert_eq!(result, expected);
            let result = (provider.import)(&codec.schema, &dialect, result, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let restored = match result { IoPayload::Binary(bytes) => <En1990Snapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => <En1990Snapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap() }; assert_eq!(database(&restored), expected);
        }
    }
    let snapshot = fixture(); let output = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap();
    assert_eq!(io_import_sqlite_snapshot::<En1990Snapshot>(&dialect, &output.value, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value, snapshot);
}

#[test]
fn sqlite_snapshot_en1990_genuine_output_admits_exact_row_and_file_frontiers(){
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
  let restored=En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}

fn norm_complete_source(case:&serde_json::Value)->En1990Snapshot{let mut source=fixture();if case["id"]=="emptyCollections"{source.permanents.clear();source.variables.clear();source.accidentals.clear();source.seismics.clear();source.members.clear();source.bridge_sls.clear();source.effects.clear();}source}
fn norm_independent_complete_extent(source:&En1990Snapshot)->serde_json::Value{
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_en1990_complete_independent_native_semantic_limits(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:9,max_columns:36,..SqliteDatabaseLimits::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(En1990Snapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:8,..limits},SqliteDatabaseLimits{max_columns:35,..limits}]{assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"relational copied limits {short:?}");assert!(En1990Snapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }eprintln!("[DEBUG] EN1990 independently measured complete copied native semantic limits");
}
#[test]
fn sqlite_snapshot_en1990_complete_independent_copied_columns_admission(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let source=fixture();let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_columns:35,..SqliteDatabaseLimits::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(En1990Snapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"encoder copied columns {encoding:?}");assert!(En1990Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"decoder copied columns {encoding:?}");}
}

#[test]
fn sqlite_snapshot_en1990_borrowed_complete_gate_copied_limits_exact_and_one_short(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};
 let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️semantic.json")).unwrap();let defaults=SqliteDatabaseLimits::default();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(<En1990Snapshot as store::ArtifactPack>::encode_pack_with(&source,&Default::default()).unwrap()),SnapshotEncoding::Text=>store::io::IoPayload::Text(<En1990Snapshot as store::ArtifactDsl>::print_dsl(&source))};
 for(role,total)in[("rows",case["rows"].as_u64().unwrap()as usize),("bytes",case["valueBytes"].as_u64().unwrap()as usize),("schema",contract["schemaBytes"].as_u64().unwrap()as usize),("tables",9),("columns",36)]{for accepted in[true,false]{let maximum=total-usize::from(!accepted);let limits=match role{"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:maximum,..defaults},"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},_=>unreachable!()};let result=store::decode_sqlite_snapshot_record_native(&payload,<En1990Snapshot as store::ArtifactDsl>::envelope_id(),En1990Snapshot::__dsl_spec_producer(),|record,native|crate::standards::v1::subsets::any::io::sqlite::snapshot::admission::admit(record,native,limits),&mut SqliteSnapshotControl::new(&mut |_|true,defaults));assert_eq!(result.is_ok(),accepted,"borrowed full CST before typed ownership {encoding:?} {role} {limits:?}");}}}
 source.retire_sqlite_snapshot();}
}
