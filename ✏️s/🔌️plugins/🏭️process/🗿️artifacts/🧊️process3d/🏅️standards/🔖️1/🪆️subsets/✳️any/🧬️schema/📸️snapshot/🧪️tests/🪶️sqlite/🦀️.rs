//! 🏭️ Genuine Process3d parent capability and complete persisted-state baselines.
use super::Process3dSnapshot;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn pose(v: f64) -> Pose {
    Pose { position: [v; 3], axis: [v; 3], angle: v }
}
fn child<S>(n: usize) -> store::ArtifactChild<S> {
    let l = laws();
    let v = &l["childAddresses"][n];
    store::ArtifactChild::new(
        v["childId"].as_str().unwrap().into(),
        store::os_io::ArtifactRef {
            artifact_id: v["artifactId"].as_str().unwrap().into(),
            dialect: store::os_io::ArtifactDialect { artifact_kind: v["artifactKind"].as_str().unwrap().into(), standard: v["standard"].as_str().unwrap().into(), subset: v["subset"].as_str().unwrap().into() },
        },
    )
}
fn full(word: u64) -> Process3dSnapshot {
    let f = f64::from_bits(word);
    let l = laws();
    let literal = l["literal"].as_str().unwrap();
    let recipes = vec![
        MeasureRecipe::DiscCut { diameter: literal.into(), kerf: String::new() },
        MeasureRecipe::BladeCut { kerf: String::new(), length: literal.into(), depth: String::new() },
        MeasureRecipe::PocketCut { diameter: literal.into(), depth: String::new() },
        MeasureRecipe::BoreDrill { radius: String::new(), depth: literal.into() },
        MeasureRecipe::CylinderAttach { radius: literal.into(), length: String::new() },
        MeasureRecipe::BoxAttach { width: String::new(), depth: literal.into(), height: String::new() },
    ];
    let mut rules = Vec::new();
    for quantity in [StockQuantity::Width, StockQuantity::Depth, StockQuantity::Height, StockQuantity::MaxDimension, StockQuantity::MinDimension] {
        rules.push(CapabilityRule::Min { quantity, parameter: literal.into(), margin: f });
        rules.push(CapabilityRule::Max { quantity, parameter: String::new(), margin: f });
    }
    let capabilities = recipes
        .into_iter()
        .map(|recipe| crate::Capability { id: String::new(), label: literal.into(), icon_id: String::new(), recipe, parameters: vec![CapabilityParameter { id: literal.into(), label: String::new(), value: f }], rules: rules.clone() })
        .collect();
    let solids = vec![
        WorkingSolid::Box { width: f, depth: f, height: f },
        WorkingSolid::Cylinder { radius: f, height: f },
        WorkingSolid::Sphere { radius: f },
        WorkingSolid::ImportedMesh { mesh_url: literal.into() },
        WorkingSolid::ImportedSolid { solid_handle: literal.into() },
        WorkingSolid::Reference { reference_id: literal.into() },
    ];
    let mut steps = Vec::new();
    for solid in solids {
        steps.push(ProcessStep { id: String::new(), label: literal.into(), enabled: false, origin: Some(StepOrigin { machine_id: String::new(), capability_id: literal.into() }), measure: ProcessMeasure::Cut { tool: solid.clone(), pose: pose(f) } });
        steps.push(ProcessStep { id: String::new(), label: literal.into(), enabled: true, origin: None, measure: ProcessMeasure::Attach { component: solid, pose: pose(f) } });
    }
    steps.push(ProcessStep { id: literal.into(), label: String::new(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: f, depth: f, pose: pose(f) } });
    Process3dSnapshot {
        workshop: Workshop {
            machines: vec![
                WorkshopMachine { id: String::new(), label: literal.into(), icon_id: String::new(), catalog_id: Some(String::new()), capabilities },
                WorkshopMachine { id: String::new(), label: String::new(), icon_id: literal.into(), catalog_id: None, capabilities: Vec::new() },
            ],
        },
        stock_id: l["rootStock"]["id"].as_str().unwrap().into(),
        stock_label: l["rootStock"]["label"].as_str().unwrap().into(),
        stock_pose: pose(f),
        stock_payload: Stock { id: l["payloadStock"]["id"].as_str().unwrap().into(), label: l["payloadStock"]["label"].as_str().unwrap().into(), solid: WorkingSolid::Box { width: f, depth: f, height: f }, pose: pose(f) },
        stock_solid: child(0),
        steps: child(1),
        step_payloads: steps,
        tool_solids: vec![child(0), child(0)],
    }
}
fn assert_pose(v: &Pose, w: u64) {
    for f in v.position.into_iter().chain(v.axis).chain([v.angle]) {
        assert_eq!(f.to_bits(), w);
    }
}
fn assert_solid(v: &WorkingSolid, w: u64) {
    match v {
        WorkingSolid::Box { width, depth, height } => {
            for f in [width, depth, height] {
                assert_eq!(f.to_bits(), w)
            }
        }
        WorkingSolid::Cylinder { radius, height } => {
            for f in [radius, height] {
                assert_eq!(f.to_bits(), w)
            }
        }
        WorkingSolid::Sphere { radius } => assert_eq!(radius.to_bits(), w),
        _ => {}
    }
}
fn assert_full(actual: &Process3dSnapshot, expected: &Process3dSnapshot, w: u64) {
    assert_eq!(actual.stock_id, expected.stock_id);
    assert_eq!(actual.stock_label, expected.stock_label);
    assert_eq!(actual.stock_payload.id, expected.stock_payload.id);
    assert_eq!(actual.stock_payload.label, expected.stock_payload.label);
    assert_ne!(actual.stock_id, actual.stock_payload.id);
    assert_pose(&actual.stock_pose, w);
    assert_pose(&actual.stock_payload.pose, w);
    assert_solid(&actual.stock_payload.solid, w);
    assert_eq!(actual.stock_solid, expected.stock_solid);
    assert_eq!(actual.steps, expected.steps);
    assert_eq!(actual.tool_solids, expected.tool_solids);
    assert_eq!(actual.print_dsl(), expected.print_dsl());
    for machine in &actual.workshop.machines {
        for c in &machine.capabilities {
            for p in &c.parameters {
                assert_eq!(p.value.to_bits(), w);
            }
            for r in &c.rules {
                let (CapabilityRule::Min { margin, .. } | CapabilityRule::Max { margin, .. }) = r;
                assert_eq!(margin.to_bits(), w);
            }
        }
    }
    for step in &actual.step_payloads {
        assert_pose(step.measure.pose(), w);
        match &step.measure {
            ProcessMeasure::Cut { tool, .. } => assert_solid(tool, w),
            ProcessMeasure::Attach { component, .. } => assert_solid(component, w),
            ProcessMeasure::Drill { radius, depth, .. } => {
                assert_eq!(radius.to_bits(), w);
                assert_eq!(depth.to_bits(), w);
            }
        }
    }
}
#[test]
fn sqlite_snapshot_process3d_actual_parent_bare_capability_is_present() {
    assert!(<Process3dSnapshot as ArtifactPack>::sqlite_snapshot_codec().is_some());
}
#[test]
fn sqlite_snapshot_process3d_complete_parent_native_formats_preserve_all_fields_and_words() {
    for word in words() {
        let expected = full(word);
        let actual = Process3dSnapshot::parse_dsl(&expected.print_dsl()).unwrap();
        assert_full(&actual, &expected, word);
        let actual = Process3dSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap();
        assert_full(&actual, &expected, word);
    }
}
#[test]
fn sqlite_snapshot_process3d_parent_child_addresses_are_independent_literals() {
    let expected = full(0);
    assert_ne!(expected.stock_solid.child_id, expected.stock_solid.target.artifact_id);
    assert_ne!(expected.steps.child_id, expected.steps.target.artifact_id);
    for actual in [Process3dSnapshot::parse_dsl(&expected.print_dsl()).unwrap(), Process3dSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap()] {
        assert_full(&actual, &expected, 0);
    }
}
#[test]
fn sqlite_snapshot_process3d_genuine_controlled_metadata_and_typed_construction_cover_every_variant() {
    for word in words() {
        let expected = full(word);
        let mut accepted = |_| true;
        let mut encode = semio_framework_value::NativeEncodeControl::new(16 << 20, &mut accepted);
        let producer = Process3dSnapshot::__dsl_spec_producer();
        (producer.encoding)(&mut encode).unwrap();
        let record = expected.__dsl_to_record_controlled(&mut encode).unwrap();
        let mut accepted = |_| true;
        let mut decode = semio_framework_value::NativeDecodeControl::new(16 << 20, &mut accepted);
        (producer.decoding)(&mut decode).unwrap();
        let actual = Process3dSnapshot::__dsl_from_record_controlled(&record, &mut decode).unwrap();
        assert_full(&actual, &expected, word);
    }
}
#[test]
fn sqlite_snapshot_process3d_actual_erased_parent_both_encodings_have_queryable_domain_fields() {
    use store::sqlite_snapshot::*;
    let codec = store::ArtifactCodec::bare::<Process3dSnapshot, crate::Process3dMutation>(crate::PROCESS_3D_SCHEMA);
    let capability = codec.snapshot_sqlite.expect("Process3d parent owned relational capability");
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.process.process3d".into(), standard: "1".into(), subset: "*".into() };
    for word in words() {
        let expected = full(word);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let input = match encoding {
                SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
                SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
            };
            let database = (capability.export)(crate::PROCESS_3D_SCHEMA, &dialect, &input, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.tables.len(), 32);
            assert_eq!(database.table("process3_document").unwrap().single_row().unwrap().text(1).unwrap(), expected.stock_id);
            assert_eq!(database.table("process3_stock_payload").unwrap().single_row().unwrap().text(2).unwrap(), expected.stock_payload.id);
            let output = (capability.import)(crate::PROCESS_3D_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let actual = match output {
                store::io_schema::IoPayload::Text(v) => Process3dSnapshot::parse_dsl(&v).unwrap(),
                store::io_schema::IoPayload::Binary(v) => Process3dSnapshot::decode_pack(&v).unwrap(),
            };
            assert_full(&actual, &expected, word);
        }
    }
}
#[test]
fn sqlite_snapshot_process3d_independent_sqlite_knows_every_handwritten_table_and_scalar_width() {
    let script = r#"import{Database}from'bun:sqlite';const d=new Database(':memory:',{safeIntegers:true});d.exec(process.argv[1]);const l=JSON.parse(process.argv[2]);const tables=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(tables.length!==Object.keys(l.tableWidths).length)throw Error('count');for(const{name}of tables)if(d.query('PRAGMA table_info('+name+')').all().length!==l.tableWidths[name])throw Error(name);d.exec("INSERT INTO process3_document VALUES(1,'root','root');INSERT INTO process3_stock_payload VALUES(1,1,'payload','payload')");for(const hex of l.binary64Bits){const word=BigInt('0x'+hex),bits=BigInt.asIntN(64,word),data=new DataView(new ArrayBuffer(8));data.setBigUint64(0,word);const v=data.getFloat64(0),kind=Number.isNaN(v)?'nan':v===Infinity?'positiveInfinity':v===-Infinity?'negativeInfinity':'finite';const cells=[1n,1n,...Array(7).fill(Number.isNaN(v)?null:v),...Array.from({length:7},()=>[bits,kind]).flat()];d.query('INSERT OR REPLACE INTO process3_stock_pose VALUES('+Array(23).fill('?').join(',')+')').run(...cells);const row=d.query('SELECT position_x_ieee754_bits,position_x_numeric_class FROM process3_stock_pose').get();if(BigInt.asUintN(64,row.position_x_ieee754_bits).toString(16).padStart(16,'0')!==hex||row.position_x_numeric_class!==kind)throw Error('word')}if(d.query('PRAGMA foreign_key_check').all().length)throw Error('fk');d.close();"#;
    let output = std::process::Command::new("bun").args(["-e", script, include_str!("../../🪶️sqlite/🗄️.sql"), &laws().to_string()]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

use semio_framework_plugin::{__semio_dispatch_PluginApp, plugin_app_close_prelude::*, EditorApp, PluginApp, VcsArtifactApp};
semio_framework_dispatch_macros::dyn_enum_close! {
 /// 🏭️ The actual Process3d declared editor app.
 enum SqliteProcessApps:PluginApp{Editor(VcsArtifactApp<EditorApp<crate::editor::process3d::Process3dPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>)}
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_process3d_actual_document_declaration_exposes_parent_capability() {
    semio_framework_plugin::Plugin::<SqliteProcessApps>::builder("process").label("Process owned SQLite").version("0.0.1").package_id("semio:process").artifact(crate::declaration().unwrap()).try_build().unwrap();
    let codec = store::document_codec(crate::PROCESS_3D_SCHEMA).await.unwrap().unwrap();
    assert!(codec.snapshot_sqlite.is_some(), "Actual Process3d declared document has no parent SQLite capability");
}
#[test]
fn sqlite_snapshot_process3d_owned_native_large_fields_have_real_interior_controls() {
    let mut expected = full(0);
    expected.stock_label = "😀".repeat(32768);
    let mut reached = false;
    let mut callback = |event: semio_framework_value::native_encoding::NativeEncodeProgress| {
        if event.total >= 65536 && event.completed >= 65536 && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    let mut encode = semio_framework_value::NativeEncodeControl::new(16 << 20, &mut callback);
    assert!(expected.__dsl_to_record_controlled(&mut encode).is_err());
    drop(encode);
    assert!(reached);
    let record = expected.__dsl_to_record();
    let mut reached = false;
    let mut callback = |event: semio_framework_value::native_decoding::NativeDecodeProgress| {
        if event.total >= 65536 && event.completed >= 65536 && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    let mut decode = semio_framework_value::NativeDecodeControl::new(16 << 20, &mut callback);
    assert!(Process3dSnapshot::__dsl_from_record_controlled(&record, &mut decode).is_err());
    drop(decode);
    assert!(reached);
}

#[path = "🚦️public/🦀️.rs"]
mod public_cohort;

#[path="🧮️census/🦀️.rs"]
mod native_semantic_census;
