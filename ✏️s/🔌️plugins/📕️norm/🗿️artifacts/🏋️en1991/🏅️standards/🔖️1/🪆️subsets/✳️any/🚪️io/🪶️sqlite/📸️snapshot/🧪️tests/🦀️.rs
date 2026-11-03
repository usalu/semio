//! 🏋️ Complete EN1991 snapshot capability and independent domain evidence.
use semio_framework_test_original_snapshot_owners as original_owners;

#[test]
fn sqlite_snapshot_en1991_actual_bare_factory_exposes_owned_relational_capability() {
    let codec = store::bare_artifact_codec::<semio_s_artifact_norm_en1991_model::En1991Snapshot, semio_s_artifact_norm_en1991_mutations::En1991Mutation>(semio_s_artifact_norm_en1991_model::EN1991_DOCUMENT_SCHEMA);
    assert!(codec.snapshot_sqlite.is_some(), "EN1991 has no owned relational capability");
}

use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
#[test]
fn sqlite_snapshot_en1991_owned_native_decoder_controls_both_physical_payloads(){
 use store::io::IoPayload;
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(&snapshot))};
  let restored=original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap();assert_eq!(restored,snapshot);
  assert!(original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits), native_owner)).is_err());
  assert!(original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits}), native_owner)).is_err());
  assert!(original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits}), native_owner)).is_err());
 }
}
use store::{os_store::serialization::ArtifactSqliteSnapshotHost, sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}};

fn fixture() -> En1991Snapshot { semio_framework_pack_json::from_json_str(include_str!("🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }
fn database(snapshot: &En1991Snapshot) -> SqliteDatabase { original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).unwrap() }
fn roundtrip(snapshot: &En1991Snapshot) -> En1991Snapshot {
    let bytes = export_sqlite_database(&database(snapshot), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|En1991Snapshot::from_sqlite_database(&import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), sqlite_control)).unwrap()
}
fn set_every_word(snapshot: &mut En1991Snapshot, value: f64) {
    snapshot.altitude = value; snapshot.en_sk = value; snapshot.en_vb = value; snapshot.mixed_terrain_distance = value; snapshot.orography_factor = value; snapshot.air_density = value;
    snapshot.height = value; snapshot.width = value; snapshot.depth = value; snapshot.assumed_delta_t = value; snapshot.t_max = value; snapshot.t_min = value; snapshot.t_0 = value; snapshot.delta_t_m = value;
    snapshot.fire_duration = value; snapshot.assumed_gas_temperature = value; snapshot.assumed_h_net = value; snapshot.fire_compartment_area = value; snapshot.fire_compartment_height = value; snapshot.fire_opening_factor = value; snapshot.fire_thermal_inertia = value; snapshot.fire_load_density_qf = value; snapshot.assumed_qf_d = value; snapshot.assumed_construction_qk = value;
    snapshot.bridge_span = value; snapshot.bridge_lane_width = value; snapshot.assumed_bridge_tandem = value; snapshot.assumed_bridge_udl = value; snapshot.assumed_bridge_lm2 = value; snapshot.assumed_bridge_footway = value; snapshot.assumed_bridge_lm3 = value; snapshot.assumed_bridge_lm4 = value;
    snapshot.hoisting_speed = value; snapshot.assumed_crane_wheel = value; snapshot.assumed_crane_horizontal = value; snapshot.silo_bulk_density = value; snapshot.silo_height = value; snapshot.silo_hydraulic_radius = value; snapshot.silo_mu = value; snapshot.silo_k = value; snapshot.assumed_silo_pressure = value; snapshot.assumed_silo_patch = value; snapshot.assumed_silo_wall_friction = value; snapshot.bridge_tandem_x = value; snapshot.rail_alpha = value; snapshot.assumed_rail_lm71 = value; snapshot.assumed_rail_lm71_udl = value; snapshot.assumed_rail_sw0 = value; snapshot.assumed_rail_sw2 = value; snapshot.crane_x = value; snapshot.crane_runway_length = value; snapshot.crane_wheel_base = value; snapshot.silo_diameter = value; snapshot.silo_eccentricity = value; snapshot.silo_hopper_angle_deg = value; snapshot.bridge_curve_radius = value;
    for floor in &mut snapshot.floors { floor.area = value; floor.assumed_qk = value; floor.assumed_qk_concentrated = value; floor.assumed_partitions = value; }
    for element in &mut snapshot.self_weight_elements { element.thickness = value; element.assumed_gk = value; }
    for roof in &mut snapshot.roofs { roof.pitch_deg = value; roof.c_e = value; roof.c_t = value; roof.parapet_height = value; roof.drift_obstruction_height = value; roof.assumed_sk = value; }
    for face in &mut snapshot.wind_faces { face.z = value; face.c_pe10 = value; face.c_pe1 = value; face.c_pi = value; face.c_s = value; face.c_d = value; face.loaded_area = value; face.assumed_wp = value; }
    for row in &mut snapshot.bridge_lanes { row.width = value; row.offset = value; }
    for row in &mut snapshot.construction_stages { row.duration_days = value; row.return_years = value; row.assumed_qk = value; }
    for row in &mut snapshot.roof_surfaces { row.pitch_deg = value; row.extent = value; }
    for row in &mut snapshot.placements { row.x = value; row.y = value; row.z = value; row.yaw = value; }
    for row in &mut snapshot.special_vehicles { row.x = value; row.y = value; }
    for row in &mut snapshot.wind_bridges { row.deck_width = value; row.deck_depth = value; row.parapet_height = value; row.height_above_ground = value; row.span = value; row.assumed_force = value; }
    for row in &mut snapshot.dynamic_elements { row.height = value; row.width = value; row.first_frequency = value; row.log_decrement = value; row.mass_per_length = value; row.strouhal = value; row.assumed_cscd = value; }
    for row in &mut snapshot.impact_actions { row.mass = value; row.speed = value; row.stiffness = value; row.height = value; row.x = value; row.y = value; row.assumed_force = value; }
    for row in &mut snapshot.internal_explosions { row.volume = value; row.vent_area = value; row.vent_pressure = value; row.assumed_pressure = value; }
    for row in &mut snapshot.thermal_sections { row.length = value; row.depth = value; row.alpha_micro = value; row.assumed_range = value; }
    for row in &mut snapshot.helicopter_landings { row.mass = value; row.x = value; row.y = value; row.height = value; row.assumed_force = value; }
    for row in &mut snapshot.tie_runs { row.spacing = value; row.span = value; row.dead_load = value; row.imposed_load = value; row.psi = value; row.x = value; row.y = value; row.provided_force = value; }
    for row in &mut snapshot.construction_loads { row.magnitude = value; }
    for row in &mut snapshot.falling_loads { row.mass = value; row.drop_height = value; row.stopping_distance = value; row.x = value; row.y = value; row.assumed_force = value; }
    for row in &mut snapshot.execution_imperfections { row.height = value; row.tolerance = value; row.vertical_load = value; row.x = value; row.y = value; row.assumed_force = value; }
    for row in &mut snapshot.derailment_cases { row.distance = value; row.design_force_x = value; row.design_force_y = value; row.x = value; row.y = value; row.assumed_force = value; }
    for row in &mut snapshot.crane_hoists { row.rated_load = value; row.release_fraction = value; }
    for row in &mut snapshot.footbridges { row.loaded_length = value; row.deck_width = value; row.natural_frequency = value; }
    for row in &mut snapshot.rail_dynamics { row.determinant_length = value; }
    for row in &mut snapshot.load_dispersals { row.wheel_length = value; row.wheel_width = value; row.pavement_thickness = value; row.slab_thickness = value; }
    for row in &mut snapshot.bulk_solids { row.internal_friction = value; }
    for row in &mut snapshot.rotating_machines { row.mass = value; row.eccentricity = value; row.frequency = value; }
    for case in &mut snapshot.accidental_cases { for impact in &mut case.impact { impact.vehicle_mass = value; impact.vehicle_speed = value; impact.assumed_force = value; } for explosion in &mut case.explosion { explosion.explosion_mass = value; explosion.standoff = value; explosion.assumed_pressure = value; } }
}

#[test]
fn sqlite_snapshot_en1991_all_neutral_fields_and_empty_owned_domains() {
    let snapshot = fixture();
    assert_eq!(roundtrip(&snapshot), snapshot);
    assert_eq!(serde_json::to_value(&snapshot).unwrap(), serde_json::to_value(serde_json::from_str::<En1991Snapshot>(include_str!("🧫️fixtures/🔣️.json")).unwrap()).unwrap());
    let mut empty = snapshot;
    empty.snow_zone.clear(); empty.floors.clear(); empty.self_weight_elements.clear(); empty.roofs.clear(); empty.wind_faces.clear(); empty.accidental_cases.clear(); empty.rotating_machines.clear(); empty.bulk_solids.clear(); empty.load_dispersals.clear(); empty.rail_dynamics.clear(); empty.footbridges.clear(); empty.crane_hoists.clear(); empty.derailment_cases.clear(); empty.execution_imperfections.clear(); empty.falling_loads.clear(); empty.construction_loads.clear(); empty.tie_runs.clear(); empty.helicopter_landings.clear(); empty.thermal_sections.clear(); empty.internal_explosions.clear(); empty.impact_actions.clear(); empty.dynamic_elements.clear(); empty.wind_bridges.clear(); empty.special_vehicles.clear(); empty.roof_surfaces.clear(); empty.placements.clear(); empty.construction_stages.clear(); empty.bridge_lanes.clear();
    assert_eq!(database(&empty).tables.len(), 33);
    assert_eq!(roundtrip(&empty), empty);
}

#[test]
fn sqlite_snapshot_en1991_neutral_unsigned8_widths_and_choices_agree_with_serde() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🛂️choices.json")).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for field in corpus["unsigned8Fields"].as_array().unwrap() {
        let field = field.as_str().unwrap();
        for value in corpus["invalidUnsigned8Values"].as_array().unwrap() { let mut invalid = fixture.clone(); invalid[field] = value.clone(); let text = serde_json::to_string(&invalid).unwrap(); assert!(serde_json::from_str::<En1991Snapshot>(&text).is_err()); assert!(semio_framework_pack_json::from_json_str::<En1991Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err()); }
        for value in corpus["validUnsigned8Values"].as_array().unwrap() { let mut valid = fixture.clone(); valid[field] = value.clone(); let text = serde_json::to_string(&valid).unwrap(); assert_eq!(serde_json::from_str::<En1991Snapshot>(&text).unwrap(), semio_framework_pack_json::from_json_str::<En1991Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()); }
    }
    for field in ["annex", "fireMode", "fireCurve", "structureKind", "fireLoadBasis"] {
        for value in corpus[field].as_array().unwrap() { let mut valid = fixture.clone(); valid[field] = value.clone(); let text = serde_json::to_string(&valid).unwrap(); let snapshot = semio_framework_pack_json::from_json_str::<En1991Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); assert_eq!(serde_json::from_str::<En1991Snapshot>(&text).unwrap(), snapshot); assert_eq!(roundtrip(&snapshot), snapshot); }
        let mut invalid = fixture.clone(); invalid[field] = serde_json::Value::String("unknown".into()); let text = serde_json::to_string(&invalid).unwrap(); assert!(serde_json::from_str::<En1991Snapshot>(&text).is_err()); assert!(semio_framework_pack_json::from_json_str::<En1991Snapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
}

#[test]
fn sqlite_snapshot_en1991_every_numeric_domain_retains_each_neutral_binary64_word() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔢️binary64.json")).unwrap();
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
    expected.accidental_cases[0].impact[0].vehicle_mass = 42.5; expected.roofs[0].roof_type = "Geändert 世界".into(); expected.fire_curve = semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::External; expected.storey_count = 255;
    assert_eq!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|En1991Snapshot::from_sqlite_database(&import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), sqlite_control)).unwrap(), expected);
}

#[test]
fn sqlite_snapshot_en1991_refuses_malformed_choices_widths_relationships_and_words() {
    let database = database(&fixture());
    for (table, index, column, value) in [("en1991_document", 0, 6, SqliteValue::Integer(256)), ("en1991_document", 0, 22, SqliteValue::Integer(-1)), ("en1991_document", 0, 24, SqliteValue::Integer(256)), ("en1991_document", 0, 1, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 25, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 26, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 39, SqliteValue::Text("unknown".into())), ("en1991_document", 0, 12, SqliteValue::Integer(2)), ("en1991_floor", 0, 1, SqliteValue::Integer(99)), ("en1991_floor", 0, 2, SqliteValue::Integer(99)), ("en1991_floor", 1, 2, SqliteValue::Integer(0)), ("en1991_roof", 0, 8, SqliteValue::Integer(2)), ("en1991_impact", 0, 1, SqliteValue::Integer(99)), ("en1991_impact", 1, 2, SqliteValue::Integer(0)), ("en1991_explosion", 0, 2, SqliteValue::Integer(99)), ("en1991_document", 0, 67, SqliteValue::Text("nan".into()))] {
        let mut invalid = database.clone(); invalid.table_mut(table).unwrap().rows[index].values[column] = value; assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|En1991Snapshot::from_sqlite_database(&invalid, sqlite_control)).is_err(), "{table}");
    }
    let mut invalid = database.clone(); invalid.table_mut("en1991_document").unwrap().rows.clear(); assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|En1991Snapshot::from_sqlite_database(&invalid, sqlite_control)).is_err());
}

#[test]
fn sqlite_snapshot_en1991_initial_and_nested_operations_support_cancellation_and_native_admission() {
    let mut snapshot = fixture();
    assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| false,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).is_err());
    assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits { max_rows: 1, ..SqliteDatabaseLimits::default() },&mut |_| true,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).is_err());
    assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() },&mut |_| true,|sqlite_control|En1991Snapshot::from_sqlite_database(&database(&snapshot), sqlite_control)).is_err());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(); assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err()); assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: 1024, ..SqliteDatabaseLimits::default() })).unwrap_err().to_string().contains("native encoding exceeds file byte limit")); }
    let controls: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🎛️control.json")).unwrap();
    snapshot.accidental_cases[0].impact = vec![snapshot.accidental_cases[0].impact[0].clone(); controls["entityCount"].as_u64().unwrap() as usize]; let cancel_at = controls["cancelAt"].as_u64().unwrap() as usize;
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { let mut reached = false; assert!(original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |event| { if event.phase == SqliteSnapshotPhase::EncodeNative && event.completed >= cancel_at { reached = true; false } else { true } }, SqliteDatabaseLimits::default()), native_owner)).is_err()); assert!(reached); }
    let large = database(&snapshot);
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] { let mut reached = false; let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if event.phase == phase && event.completed >= cancel_at { reached = true; false } else { true } }; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()); assert!(if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).is_err() } else { En1991Snapshot::from_sqlite_database(&large, &mut control).is_err() }); assert!(reached); }
}

#[test]
fn sqlite_snapshot_en1991_large_owned_text_cancels_before_projection_reconstruction_and_encoding_copy() {
    let mut snapshot = fixture(); snapshot.snow_zone = "x".repeat(131073); let database = database(&snapshot);
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::EncodeNative] { let mut reached = false; let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if event.phase == phase && event.total>=65536&&event.completed>=65536&&event.completed<event.total { reached = true; false } else { true } }; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()); let rejected = match phase { SqliteSnapshotPhase::ProjectSnapshot => snapshot.to_sqlite_database(&mut control).is_err(), SqliteSnapshotPhase::ReconstructSnapshot => En1991Snapshot::from_sqlite_database(&database, &mut control).is_err(), _ => original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut control, native_owner)).is_err() }; assert!(rejected); assert!(reached); }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_en1991_actual_erased_codec_and_declaration_preserve_all_owned_words() {
    use {semio_framework_artifact_reference::ArtifactDialect,store::io::IoPayload,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};
    semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("norm").label("EN1991 owned SQLite").version("0.0.1").package_id("semio:norm").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let codec = store::os_store::codec_registry::document_codec(semio_s_artifact_norm_en1991_model::EN1991_DOCUMENT_SCHEMA).await.unwrap().unwrap(); let provider = codec.snapshot_sqlite.as_ref().unwrap(); let dialect = ArtifactDialect { artifact_kind: "s.norm.en1991".into(), standard: "1".into(), subset: "*".into() };
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔢️binary64.json")).unwrap();
    for word in corpus["ieee754Binary64Bits"].as_array().unwrap() {
        let mut snapshot = fixture(); set_every_word(&mut snapshot, f64::from_bits(word.as_str().unwrap().parse().unwrap())); let expected = database(&snapshot);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] { let payload = match encoding { SnapshotEncoding::Binary => IoPayload::Binary(<En1991Snapshot as pack::snapshot::ArtifactPack>::encode_pack_with(&snapshot, &store::PackEncodeOptions::default()).unwrap()), SnapshotEncoding::Text => IoPayload::Text(<En1991Snapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snapshot)) }; let result = original_owners::with_decoding(|native_owner|(provider.export)(&codec.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).unwrap().value; assert_eq!(result, expected); let result = original_owners::with_encoding(|native_owner|(provider.import)(&codec.schema, &dialect, &mut Some(result), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).unwrap().value; let restored = match result { IoPayload::Binary(bytes) => <En1991Snapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => <En1991Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).unwrap() }; assert_eq!(database(&restored), expected); }
    }
    let snapshot = fixture(); let output = async{let original_file_dialect=&dialect;let original_file_input=&snapshot;let original_file_encoding=SnapshotEncoding::Binary;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap(); assert_eq!(async{let original_file_dialect=&dialect;let original_file_input=&output.value;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<En1991Snapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value, snapshot);
}

#[test]
fn sqlite_snapshot_en1991_genuine_output_admits_exact_row_and_file_frontiers(){
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let expected=original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).unwrap();let rows=expected.tables.iter().map(|table|table.rows.len()).sum::<usize>();assert!(rows>0);
 let bytes=export_sqlite_database(&expected,limits,&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();const rows=tables.reduce((n,t)=>n+Number(db.query('SELECT COUNT(*) AS count FROM "'+t.name.replaceAll('"','""')+'"').get().count),0);await Bun.write(Bun.stdout,String(rows));db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap().parse::<usize>().unwrap(),rows);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits}), native_owner)).unwrap();
  let physical=match &payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};assert!(physical>0);
  let repeated=original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,max_file_bytes:physical,..limits}), native_owner)).unwrap();assert_eq!(repeated,payload);
  let restored=original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap();assert_eq!(original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|restored.to_sqlite_database(sqlite_control)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits), native_owner)).is_err());
 }
}

fn norm_complete_source(case:&serde_json::Value)->En1991Snapshot{let mut source=fixture();if case["id"]=="emptyCollections"{source.floors.clear();source.self_weight_elements.clear();source.roofs.clear();source.wind_faces.clear();source.accidental_cases.clear();source.bridge_lanes.clear();}source}
fn norm_independent_complete_extent(source:&En1991Snapshot)->serde_json::Value{
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_en1991_complete_independent_native_semantic_limits(){
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:40,max_columns:192,..SqliteDatabaseLimits::default()};let database=original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap();assert_eq!(original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|En1991Snapshot::from_sqlite_database(&database, sqlite_control)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap();assert_eq!(original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap(),source);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:32,..limits},SqliteDatabaseLimits{max_columns:191,..limits}]{assert!(original_owners::with_sqlite_recipient(short,&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).is_err(),"relational copied limits {short:?}");assert!(original_owners::with_sqlite_recipient(short,&mut |_|true,|sqlite_control|En1991Snapshot::from_sqlite_database(&database, sqlite_control)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }
}
#[test]
fn sqlite_snapshot_en1991_complete_independent_copied_columns_admission(){
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};let source=fixture();let database=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap();let limits=SqliteDatabaseLimits{max_columns:191,..SqliteDatabaseLimits::default()};assert!(original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|En1991Snapshot::from_sqlite_database(&database, sqlite_control)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).is_err(),"encoder copied columns {encoding:?}");assert!(original_owners::with_decoding(|native_owner|En1991Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).is_err(),"decoder copied columns {encoding:?}");}
}

#[test]
fn sqlite_snapshot_en1991_borrowed_complete_gate_copied_limits_exact_and_one_short(){
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};
 let contract:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🎛️semantic.json")).unwrap();let defaults=SqliteDatabaseLimits::default();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(<En1991Snapshot as pack::snapshot::ArtifactPack>::encode_pack_with(&source,&Default::default()).unwrap()),SnapshotEncoding::Text=>store::io::IoPayload::Text(<En1991Snapshot as pack::snapshot::ArtifactDsl>::print_dsl(&source))};
 for(role,total)in[("rows",case["rows"].as_u64().unwrap()as usize),("bytes",case["valueBytes"].as_u64().unwrap()as usize),("schema",contract["schemaBytes"].as_u64().unwrap()as usize),("tables",33),("columns",191)]{for accepted in[true,false]{let maximum=total-usize::from(!accepted);let limits=match role{"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:maximum,..defaults},"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},_=>unreachable!()};let result=store::decode_sqlite_snapshot_record_native(&payload,<En1991Snapshot as pack::snapshot::ArtifactDsl>::envelope_id(),En1991Snapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {semio_s_artifact_norm_en1991_io::sqlite::snapshot::admission::admit(record,native,limits)})(); *snapshot_output = Some(constructed?); Ok(()) },&mut SqliteSnapshotControl::new(&mut |_|true,defaults));assert_eq!(result.is_ok(),accepted,"borrowed full CST before typed ownership {encoding:?} {role} {limits:?}");}}}
 source.retire_sqlite_snapshot();}
}

fn norm_semantic_extent_of(extent:&serde_json::Value)->semio_s_plugin_norm_compliance::regen::SemanticExtent{
 semio_s_plugin_norm_compliance::regen::SemanticExtent{schema_bytes:extent["schemaBytes"].as_u64().unwrap(),table_widths:extent["tableWidths"].as_object().unwrap().iter().map(|(table,width)|(table.clone(),width.as_u64().unwrap())).collect(),rows:extent["rows"].as_u64().unwrap(),value_bytes:extent["valueBytes"].as_u64().unwrap()}
}
#[test]
fn sqlite_snapshot_en1991_regen_semantic_contract_from_the_independent_reader_or_bless(){
 let cases:Vec<(&str,_)>=["full","emptyCollections"].into_iter().map(|id|(id,norm_semantic_extent_of(&norm_independent_complete_extent(&norm_complete_source(&serde_json::json!({"id":id}))))))
 .collect();
 semio_s_plugin_norm_compliance::regen::bless_semantic_contract(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎛️semantic.json"),&cases);
}
