#[path = "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs"]
mod original_owners;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1::subsets::any::io::sqlite::snapshot::FlowSnapshot;
use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::*};
fn fixture()->FlowSnapshot{let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let s=&f["snapshot"];let c=&s["content"];let t=&c["target"];let d=&t["dialect"];FlowSnapshot{schema:s["schema"].as_str().unwrap().into(),content:store::os_store::artifact_child::ArtifactChild::new(c["childId"].as_str().unwrap().into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:t["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:d["artifactKind"].as_str().unwrap().into(),standard:d["standard"].as_str().unwrap().into(),subset:d["subset"].as_str().unwrap().into()}})}}
fn project(s:&FlowSnapshot)->SqliteDatabase{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|s.to_sqlite_database(sqlite_control)).unwrap()}
fn restore(d:&SqliteDatabase)->Result<FlowSnapshot,semio_framework_value::ValueError>{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|FlowSnapshot::from_sqlite_database(d, sqlite_control))}
#[test]fn sqlite_snapshot_flow_owned_child_and_independent_edit(){use std::io::Write;use std::process::{Command,Stdio};let s=fixture();let bytes=export_sqlite_database(&project(&s),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const c=d.query('SELECT child_id,artifact_id FROM flow_content').get();if(c.child_id===c.artifact_id)throw Error('distinct child target identity');d.query(\"UPDATE flow_content SET child_id='independent child 日本'\").run();await Bun.write(Bun.stdout,d.serialize());";let mut c=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();c.stdin.take().unwrap().write_all(&bytes).unwrap();let o=c.wait_with_output().unwrap();assert!(o.status.success(),"{}",String::from_utf8_lossy(&o.stderr));let back=restore(&import_sqlite_database(&o.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();assert_eq!(back.content.child_id,"independent child 日本");assert_eq!(back.content.target,s.content.target);assert_eq!(back.schema,s.schema);}
#[test]fn sqlite_snapshot_flow_bad_owned_rows_and_cancellation(){let d=project(&fixture());for case in 0..4{let mut bad=d.clone();match case{0=>bad.table_mut("flow_content").unwrap().rows.clear(),1=>bad.table_mut("flow_content").unwrap().rows[0].values[0]=SqliteValue::Integer(2),2=>bad.table_mut("flow_document").unwrap().rows[0].values[1]=SqliteValue::Blob(vec![0]),_=>bad.table_mut("flow_document").unwrap().rows.push(SqliteRow{rowid:2,values:vec![SqliteValue::Integer(2),SqliteValue::Text("orphan".into())]})}assert!(restore(&bad).is_err());}assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits{max_value_bytes:16,..SqliteDatabaseLimits::default()},&mut |_|true,|sqlite_control|fixture().to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|false,|sqlite_control|FlowSnapshot::from_sqlite_database(&d, sqlite_control)).is_err());let mut huge=fixture();huge.schema="文".repeat(40000);assert!(huge.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |p|p.completed==0,SqliteDatabaseLimits::default())).is_err());}
#[test]fn sqlite_snapshot_flow_actual_bare_erased_encodings(){let s=fixture();let codec=store::bare_artifact_codec::<FlowSnapshot,crate::FlowMutation>(crate::FLOW_DOCUMENT_SCHEMA);let provider=codec.snapshot_sqlite.unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.flow.flow".into(),standard:"1".into(),subset:"*".into()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(<FlowSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&s)),SnapshotEncoding::Text=>store::io::IoPayload::Text(<FlowSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&s))};let d=original_owners::with_decoding(|native_owner|(provider.export)(crate::FLOW_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(project(&restore(&d).unwrap()),project(&s));assert_eq!(original_owners::with_encoding(|native_owner|(provider.import)(crate::FLOW_DOCUMENT_SCHEMA,&dialect, &mut Some(d),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value,payload);}}

#[test]fn sqlite_snapshot_flow_exact_wildcard_guard(){let s=fixture();let d=project(&s);for(artifact_kind,standard,subset)in[("s.flow.other","1","*"),("s.flow.flow","v1","*"),("s.flow.flow","1","flow")]{let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:artifact_kind.into(),standard:standard.into(),subset:subset.into()};assert!(s.validate_sqlite_snapshot_subset(&dialect,&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_flow_actual_declaration_owned_io(){use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("flow").label("Flow SQLite declaration").version("0.0.1").package_id("semio:flow").artifact(crate::declaration().unwrap()).try_build().unwrap();let source=fixture();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.flow.flow".into(),standard:"1".into(),subset:"*".into()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let bytes=async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |p|{phases.push(p.phase);true};semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;let back=async{let original_file_dialect=&dialect;let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<FlowSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;assert_eq!(back,source);assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::EncodeNative)));}}

#[test]
fn sqlite_snapshot_flow_all_literal_reference_fields_match_native_and_sqlite(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["literalIdentities"].as_array().unwrap(){let mut source=fixture();source.content.child_id=case["childId"].as_str().unwrap().into();source.content.target.artifact_id=case["artifactId"].as_str().unwrap().into();source.content.target.dialect.artifact_kind=case["artifactKind"].as_str().unwrap().into();source.content.target.dialect.standard=case["standard"].as_str().unwrap().into();source.content.target.dialect.subset=case["subset"].as_str().unwrap().into();assert_eq!(<FlowSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack::snapshot::ArtifactPack::encode_pack(&source)).unwrap(),source);assert_eq!(<FlowSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&pack::snapshot::ArtifactDsl::print_dsl(&source)).unwrap(),source);assert_eq!(restore(&project(&source)).unwrap(),source);}
}
#[test]
fn sqlite_snapshot_flow_controlled_native_full_string_fields(){
 let source=fixture();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&source)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(&source))};assert_eq!(original_owners::with_decoding(|native_owner|FlowSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap(),source);assert_eq!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap(),payload);assert!(original_owners::with_decoding(|native_owner|FlowSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default()), native_owner)).is_err());assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default()), native_owner)).is_err());}
}
#[test]
fn sqlite_snapshot_flow_native_each_owned_string_has_interior_admission(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let controls=&corpus["controlledAdmission"];let text=controls["token"].as_str().unwrap().repeat(controls["repeat"].as_u64().unwrap()as usize);let cancel=controls["cancelAfterBytes"].as_u64().unwrap()as usize;
 for field in 0..6{let mut source=fixture();match field{0=>source.schema=text.clone(),1=>source.content.child_id=text.clone(),2=>source.content.target.artifact_id=text.clone(),3=>source.content.target.dialect.artifact_kind=text.clone(),4=>source.content.target.dialect.standard=text.clone(),_=>source.content.target.dialect.subset=text.clone()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&source)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(&source))};for phase in[SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut hit=false;let mut callback=|p:SqliteSnapshotProgress|{if p.phase==phase&&p.total>cancel&&p.completed>=cancel&&p.completed<p.total{hit=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let result=if phase==SqliteSnapshotPhase::DecodeNative{original_owners::with_decoding(|native_owner|FlowSnapshot::decode_sqlite_snapshot_native(&payload,&mut control, native_owner)).map(|_|())}else{original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut control, native_owner)).map(|_|())};assert!(result.is_err(),"field {field} {phase:?}");assert!(hit,"field {field} {phase:?}");}let limits=SqliteDatabaseLimits{max_value_bytes:controls["smallValueBudget"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()};assert!(original_owners::with_decoding(|native_owner|FlowSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).is_err());assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).is_err());}}
}

/// 👥️ Demands simultaneous native SQLite ownership for the genuine composed document and framework host.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_flow_joint_distinct_persisted_owners_retain_public_io() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::any::TypeId;
    use store::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot, io_route, native_snapshot_sqlite_schema};
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,store::io_schema::IoFidelity,store::io_schema::SQLITE_SNAPSHOT};
    type Host = semio_framework_artifact_flow_flow::FlowHostSnapshot;
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/👥️joint-owners/🔣️.json")).unwrap();
    let declaration = crate::declaration().expect("actual composed Flow declaration");
    let bindings = declaration.document_codec_bindings();
    let matching: Vec<_> = bindings.iter().filter(|(dialect,_)|*dialect==crate::FLOW_DIALECT).collect();
    assert_eq!(matching.len(),1,"one actual composed Flow native owner");
    let plugin_codec = matching[0].1.clone();
    let host_codec = store::bare_artifact_codec::<Host,semio_framework_artifact_flow_flow::FlowMutation>(semio_framework_artifact_flow_flow::FLOW_DOCUMENT_SCHEMA);
    assert_eq!(plugin_codec.snapshot_sqlite.as_ref().unwrap().snapshot_type,Some(TypeId::of::<FlowSnapshot>()));
    assert_eq!(host_codec.snapshot_sqlite.as_ref().unwrap().snapshot_type,Some(TypeId::of::<Host>()));
    assert_ne!(plugin_codec.snapshot_sqlite.as_ref().unwrap().snapshot_type,host_codec.snapshot_sqlite.as_ref().unwrap().snapshot_type);
    let preflight = store::os_store::codec_registry::preflight_document_codecs(&[plugin_codec.clone(),host_codec.clone()]).await;
    eprintln!("[DEBUG] Flow joint actual composed_schema={} framework_schema={} preflight={preflight:?}",plugin_codec.schema,host_codec.schema);
    preflight.expect("both genuine distinct persisted Flow domains must coexist");
    assert_eq!(plugin_codec.schema,law["owners"][0]["schema"].as_str().unwrap());
    assert_eq!(FlowSnapshot::default().schema,law["owners"][0]["schema"].as_str().unwrap());
    assert_eq!(host_codec.schema,law["owners"][1]["schema"].as_str().unwrap());
    let plugin_dialect: ArtifactDialect = matching[0].0.into();
    let host_native = Dialect {artifact_kind:"flow.host_snapshot",standard:StandardId("1"),subset:SubsetId("*")};
    let host_dialect: ArtifactDialect = host_native.into();
    for (index,dialect) in [&plugin_dialect,&host_dialect].into_iter().enumerate() {
        let expected=&law["owners"][index]["dialect"];
        assert_eq!(dialect.artifact_kind,expected["artifactKind"].as_str().unwrap());
        assert_eq!(dialect.standard,expected["standard"].as_str().unwrap());
        assert_eq!(dialect.subset,expected["subset"].as_str().unwrap());
    }
    semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("flow").label("Flow joint SQLite owners").version("0.0.1").package_id("semio:flow").artifact(declaration).try_build().unwrap();
    store::io::register_native_document_codec(host_native,host_codec.clone()).unwrap();
    for (dialect,codec) in [(&plugin_dialect,&plugin_codec),(&host_dialect,&host_codec)] {
        let installed=store::os_store::codec_registry::document_codec(&codec.schema).await.unwrap().unwrap();
        assert_eq!(installed.snapshot_sqlite.as_ref().unwrap().snapshot_type,codec.snapshot_sqlite.as_ref().unwrap().snapshot_type);
        assert_eq!(native_snapshot_sqlite_schema(dialect).unwrap(),codec.snapshot_sqlite.as_ref().unwrap().schema);
        let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);
        for (from,into) in [(dialect,&sqlite),(&sqlite,dialect)] {
            let route=io_route(from,into,1).await.unwrap().value;
            assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,IoFidelity::Exact);
        }
    }
    macro_rules! owning_io {
        ($snapshot:expr,$owner:ty,$dialect:expr) => {{
            let source=$snapshot;
            for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
                let bytes=async{let original_file_dialect=$dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;
                let back=async{let original_file_dialect=$dialect;let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<$owner>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;
                assert_eq!(back,source);
                eprintln!("[DEBUG] Flow joint physical owner={} native={encoding:?} bytes={}",stringify!($owner),bytes.len());
            }
        }};
    }
    owning_io!(fixture(),FlowSnapshot,&plugin_dialect);
    owning_io!(Host::default(),Host,&host_dialect);
    owning_io!(fixture(),FlowSnapshot,&plugin_dialect);
}
