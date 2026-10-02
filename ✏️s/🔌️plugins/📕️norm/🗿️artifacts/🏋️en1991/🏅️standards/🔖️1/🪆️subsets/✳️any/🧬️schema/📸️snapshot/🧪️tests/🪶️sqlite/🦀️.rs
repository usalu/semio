//! 🏋️ Complete EN1991 snapshot capability and independent domain evidence.

#[test]
fn sqlite_snapshot_en1991_actual_bare_factory_exposes_owned_relational_capability() {
    let codec = store::ArtifactCodec::bare::<super::En1991Snapshot, crate::En1991Mutation>(crate::EN1991_DOCUMENT_SCHEMA);
    assert!(codec.snapshot_sqlite.is_some(), "EN1991 has no owned relational capability");
}

use super::En1991Snapshot;
#[test]
fn sqlite_snapshot_en1991_owned_native_decoder_controls_both_physical_payloads(){
 use store::io::IoPayload;
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let restored=En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);
  assert!(En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  assert!(En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
  assert!(En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits})).is_err());
 }
}
use store::{ArtifactSqliteSnapshot, sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}};

fn fixture() -> En1991Snapshot { store::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap() }
fn database(snapshot: &En1991Snapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn roundtrip(snapshot: &En1991Snapshot) -> En1991Snapshot {
    let bytes = export_sqlite_database(&database(snapshot), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    En1991Snapshot::from_sqlite_database(&import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap()
}
fn set_every_word(snapshot: &mut En1991Snapshot, value: f64) {
    snapshot.altitude = value; snapshot.en_sk = value; snapshot.en_vb = value; snapshot.mixed_terrain_distance = value; snapshot.orography_factor = value; snapshot.air_density = value;
    snapshot.height = value; snapshot.width = value; snapshot.depth = value; snapshot.assumed_delta_t = value; snapshot.t_max = value; snapshot.t_min = value; snapshot.t_0 = value; snapshot.delta_t_m = value;
    snapshot.fire_duration = value; snapshot.assumed_gas_temperature = value; snapshot.assumed_h_net = value; snapshot.fire_compartment_area = value; snapshot.fire_compartment_height = value; snapshot.fire_opening_factor = value; snapshot.fire_thermal_inertia = value; snapshot.fire_load_density_qf = value; snapshot.assumed_qf_d = value; snapshot.assumed_construction_qk = value;
    snapshot.bridge_span = value; snapshot.bridge_lane_width = value; snapshot.assumed_bridge_tandem = value; snapshot.assumed_bridge_udl = value; snapshot.assumed_bridge_lm2 = value; snapshot.assumed_bridge_footway = value; snapshot.assumed_bridge_lm3 = value; snapshot.assumed_bridge_lm4 = value;
    snapshot.hoisting_speed = value; snapshot.assumed_crane_wheel = value; snapshot.assumed_crane_horizontal = value; snapshot.silo_bulk_density = value; snapshot.silo_height = value; snapshot.silo_hydraulic_radius = value; snapshot.silo_mu = value; snapshot.silo_k = value; snapshot.assumed_silo_pressure = value; snapshot.assumed_silo_patch = value; snapshot.assumed_silo_wall_friction = value;
    for floor in &mut snapshot.floors { floor.area = value; floor.assumed_qk = value; floor.assumed_qk_concentrated = value; floor.assumed_partitions = value; }
    for element in &mut snapshot.self_weight_elements { element.thickness = value; element.assumed_gk = value; }
    for roof in &mut snapshot.roofs { roof.pitch_deg = value; roof.c_e = value; roof.c_t = value; roof.parapet_height = value; roof.drift_obstruction_height = value; roof.assumed_sk = value; }
    for face in &mut snapshot.wind_faces { face.z = value; face.c_pe10 = value; face.c_pe1 = value; face.c_pi = value; face.c_s = value; face.c_d = value; face.loaded_area = value; face.assumed_wp = value; }
    for case in &mut snapshot.accidental_cases { for impact in &mut case.impact { impact.vehicle_mass = value; impact.vehicle_speed = value; impact.assumed_force = value; } for explosion in &mut case.explosion { explosion.explosion_mass = value; explosion.standoff = value; explosion.assumed_pressure = value; } }
}

#[test]
fn sqlite_snapshot_en1991_all_neutral_fields_and_empty_owned_domains() {
    let snapshot = fixture();
    assert_eq!(roundtrip(&snapshot), snapshot);
    assert_eq!(serde_json::to_value(&snapshot).unwrap(), serde_json::to_value(serde_json::from_str::<En1991Snapshot>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()).unwrap());
    let mut empty = snapshot;
    empty.snow_zone.clear(); empty.floors.clear(); empty.self_weight_elements.clear(); empty.roofs.clear(); empty.wind_faces.clear(); empty.accidental_cases.clear();
    assert_eq!(database(&empty).tables.len(), 8);
    assert_eq!(roundtrip(&empty), empty);
}

#[test]
fn sqlite_snapshot_en1991_neutral_unsigned8_widths_and_choices_agree_with_serde() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🛂️choices.json")).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    for field in corpus["unsigned8Fields"].as_array().unwrap() {
        let field = field.as_str().unwrap();
        for value in corpus["invalidUnsigned8Values"].as_array().unwrap() { let mut invalid = fixture.clone(); invalid[field] = value.clone(); let text = serde_json::to_string(&invalid).unwrap(); assert!(serde_json::from_str::<En1991Snapshot>(&text).is_err()); assert!(store::json::from_json_str::<En1991Snapshot>(&text).is_err()); }
        for value in corpus["validUnsigned8Values"].as_array().unwrap() { let mut valid = fixture.clone(); valid[field] = value.clone(); let text = serde_json::to_string(&valid).unwrap(); assert_eq!(serde_json::from_str::<En1991Snapshot>(&text).unwrap(), store::json::from_json_str::<En1991Snapshot>(&text).unwrap()); }
    }
    for field in ["annex", "fireMode", "fireCurve", "structureKind"] {
        for value in corpus[field].as_array().unwrap() { let mut valid = fixture.clone(); valid[field] = value.clone(); let text = serde_json::to_string(&valid).unwrap(); let snapshot = store::json::from_json_str::<En1991Snapshot>(&text).unwrap(); assert_eq!(serde_json::from_str::<En1991Snapshot>(&text).unwrap(), snapshot); assert_eq!(roundtrip(&snapshot), snapshot); }
        let mut invalid = fixture.clone(); invalid[field] = serde_json::Value::String("unknown".into()); let text = serde_json::to_string(&invalid).unwrap(); assert!(serde_json::from_str::<En1991Snapshot>(&text).is_err()); assert!(store::json::from_json_str::<En1991Snapshot>(&text).is_err());
    }
}

#[test]
fn sqlite_snapshot_en1991_every_numeric_domain_retains_each_neutral_binary64_word() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();
    for word in corpus["ieee754Binary64Bits"].as_array().unwrap() { let bits = word.as_str().unwrap().parse::<u64>().unwrap(); let mut snapshot = fixture(); set_every_word(&mut snapshot, f64::from_bits(bits)); assert_eq!(database(&roundtrip(&snapshot)), database(&snapshot)); }
}

#[test]
fn sqlite_snapshot_en1991_independent_sqlite_joins_and_full_domain_edits() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut expected = fixture();
    let bytes = export_sqlite_database(&database(&expected), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import{Database}from'bun:sqlite';import{Buffer}from'node:buffer';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT c.logical_id,i.ordinal,i.vehicle_mass,e.assumed_pressure FROM en1991_accidental_case c JOIN en1991_impact i ON i.case_id=c.id JOIN en1991_explosion e ON e.case_id=c.id AND e.ordinal=i.ordinal WHERE c.id=1 ORDER BY i.ordinal').all();if(rows.length!==2||rows[0].vehicle_mass!==1000.5||rows[0].assumed_pressure!==5000.75||rows[1].vehicle_mass!==-1.5||rows[1].assumed_pressure!==-5000.75)throw Error('ordered case relationships');const word=value=>{const b=Buffer.alloc(8);b.writeDoubleBE(value);return BigInt.asIntN(64,b.readBigUInt64BE())};db.run(\"UPDATE en1991_impact SET vehicle_mass=42.5,vehicle_mass_ieee754_bits=?,vehicle_mass_ieee754_class='finite' WHERE id=1\",[word(42.5)]);db.run(\"UPDATE en1991_roof SET roof_type='Geändert 世界' WHERE id=1\");db.run(\"UPDATE en1991_document SET fire_curve='external',storey_count=255\");await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    expected.accidental_cases[0].impact[0].vehicle_mass = 42.5; expected.roofs[0].roof_type = "Geändert 世界".into(); expected.fire_curve = crate::part_1_2::FireCurve::External; expected.storey_count = 255;
    assert_eq!(En1991Snapshot::from_sqlite_database(&import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), expected);
}

#[test]
fn sqlite_snapshot_en1991_refuses_malformed_choices_widths_relationships_and_words() {
    let database = database(&fixture());
    for (table, index, column, value) in [("en1991_document", 0, 6, SqliteValue::Integer(256)), ("en1991_document", 0, 22, SqliteValue::Integer(-1)), ("en1991_document", 0, 24, SqliteValue::Integer(256)), ("en1991_document", 0, 1, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 25, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 26, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 39, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 12, SqliteValue::Integer(2)), ("en1991_floor", 0, 1, SqliteValue::Integer(99)), ("en1991_floor", 0, 2, SqliteValue::Integer(99)), ("en1991_floor", 1, 2, SqliteValue::Integer(0)), ("en1991_roof", 0, 8, SqliteValue::Integer(2)), ("en1991_impact", 0, 1, SqliteValue::Integer(99)), ("en1991_impact", 1, 2, SqliteValue::Integer(0)), ("en1991_explosion", 0, 2, SqliteValue::Integer(99)), ("en1991_document", 0, 67, SqliteValue::Text("nan".into()))] {
        let mut invalid = database.clone(); invalid.table_mut(table).unwrap().rows[index].values[column] = value; assert!(En1991Snapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "{table}");
    }
    let mut invalid = database.clone(); invalid.table_mut("en1991_document").unwrap().rows.clear(); assert!(En1991Snapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_en1991_initial_and_nested_operations_support_cancellation_and_native_admission() {
    let mut snapshot = fixture();
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: 1, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(En1991Snapshot::from_sqlite_database(&database(&snapshot), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() })).is_err());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(); assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err()); assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: 1024, ..SqliteDatabaseLimits::default() })).unwrap_err().contains("native encoding exceeds file byte limit")); }
    let controls: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️control.json")).unwrap();
    snapshot.accidental_cases[0].impact = vec![snapshot.accidental_cases[0].impact[0].clone(); controls["entityCount"].as_u64().unwrap() as usize]; let cancel_at = controls["cancelAt"].as_u64().unwrap() as usize;
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { let mut reached = false; assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |event| { if event.phase == SqliteSnapshotPhase::EncodeNative && event.completed >= cancel_at { reached = true; false } else { true } }, SqliteDatabaseLimits::default())).is_err()); assert!(reached); }
    let large = database(&snapshot);
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] { let mut reached = false; let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if event.phase == phase && event.completed >= cancel_at { reached = true; false } else { true } }; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()); assert!(if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).is_err() } else { En1991Snapshot::from_sqlite_database(&large, &mut control).is_err() }); assert!(reached); }
}

#[test]
fn sqlite_snapshot_en1991_large_owned_text_cancels_before_projection_reconstruction_and_encoding_copy() {
    let mut snapshot = fixture(); snapshot.snow_zone = "x".repeat(131073); let database = database(&snapshot);
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::EncodeNative] { let mut reached = false; let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if event.phase == phase && event.total>=65536&&event.completed>=65536&&event.completed<event.total { reached = true; false } else { true } }; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()); let rejected = match phase { SqliteSnapshotPhase::ProjectSnapshot => snapshot.to_sqlite_database(&mut control).is_err(), SqliteSnapshotPhase::ReconstructSnapshot => En1991Snapshot::from_sqlite_database(&database, &mut control).is_err(), _ => snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut control).is_err() }; assert!(rejected); assert!(reached); }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_en1991_actual_erased_codec_and_declaration_preserve_all_owned_words() {
    use store::io::{ArtifactDialect, IoPayload, io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("norm").label("EN1991 owned SQLite").version("0.0.1").package_id("semio:norm").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let codec = store::document_codec(crate::EN1991_DOCUMENT_SCHEMA).await.unwrap().unwrap(); let provider = codec.snapshot_sqlite.as_ref().unwrap(); let dialect = ArtifactDialect { artifact_kind: "s.norm.en1991".into(), standard: "1".into(), subset: "*".into() };
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️binary64.json")).unwrap();
    for word in corpus["ieee754Binary64Bits"].as_array().unwrap() {
        let mut snapshot = fixture(); set_every_word(&mut snapshot, f64::from_bits(word.as_str().unwrap().parse().unwrap())); let expected = database(&snapshot);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { let payload = match encoding { SnapshotEncoding::Binary => IoPayload::Binary(<En1991Snapshot as store::ArtifactPack>::encode_pack_with(&snapshot, &store::PackEncodeOptions::default()).unwrap()), SnapshotEncoding::Text => IoPayload::Text(<En1991Snapshot as store::ArtifactDsl>::print_dsl(&snapshot)) }; let result = (provider.export)(&codec.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value; assert_eq!(result, expected); let result = (provider.import)(&codec.schema, &dialect, result, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value; let restored = match result { IoPayload::Binary(bytes) => <En1991Snapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => <En1991Snapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap() }; assert_eq!(database(&restored), expected); }
    }
    let snapshot = fixture(); let output = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap(); assert_eq!(io_import_sqlite_snapshot::<En1991Snapshot>(&dialect, &output.value, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value, snapshot);
}

#[test]
fn sqlite_snapshot_en1991_genuine_output_admits_exact_row_and_file_frontiers(){
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
  let restored=En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}
