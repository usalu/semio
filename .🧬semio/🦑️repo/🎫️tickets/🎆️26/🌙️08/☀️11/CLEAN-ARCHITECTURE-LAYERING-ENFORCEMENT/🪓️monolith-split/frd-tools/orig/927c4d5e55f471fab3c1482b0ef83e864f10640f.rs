//! 🧪️ Conformance and publication laws of stdio.jpg (jfif-1.01/🧾️document): grammar and protocol parseability, recognition of real printed output, protocol walks over real bytes, fixture honesty and the native-codec publication receipt.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::JpgSnapshot;
use crate::schema::snapshot::demo_jpg_snapshot;
use crate::standards::v_jfif_1_01::subsets::document::io::{decode_jpg, encode_jpg, JpgEncodeOptions};
/// 🧬️ Publishes the actual compiled JPG factory only after independent protocol and complete logical-carrier laws.
#[test]
fn owned_fixture_publication_reports_canonical_logical_carriers(){
 use store::{pack::snapshot::ArtifactDsl,pack::snapshot::ArtifactPack,os_store::serialization::ArtifactSqliteSnapshotHost};use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,export_sqlite_database};use std::{io::Write,process::{Command,Stdio}};
 let source=runtime_fixture_text("../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json");let snapshot:JpgSnapshot=semio_framework_pack_json::from_json_str(source,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let demo=demo_jpg_snapshot();
 assert_eq!(demo.print_dsl(),runtime_fixture_text("../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));assert_eq!(demo.encode_pack_with(&Default::default()).unwrap(),runtime_fixture_bytes("../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio"));
 let factories=crate::native_codecs();assert_eq!(factories.len(),1);let factory=&factories[0];let codec=(factory.codec)();let kind=(factory.kind)();let bare=store::bare_artifact_codec::<JpgSnapshot,crate::JpgMutation>(crate::STDIO_JPG_DOCUMENT_SCHEMA);let provider=codec.snapshot_sqlite.expect("original JPG SQLite owner");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<JpgSnapshot>()));assert_eq!(bare.snapshot_sqlite.unwrap().snapshot_type,provider.snapshot_type);assert_eq!(codec.pack_schema_hash,bare.pack_schema_hash);assert_eq!(factory.id,"stdio.native.jpg.v1");assert_eq!(kind.id,crate::JPG_ARTIFACT_SCHEMA_ID);assert_eq!(codec.schema,crate::STDIO_JPG_DOCUMENT_SCHEMA);assert_eq!(codec.extension,"jpg");let dialect:semio_framework_artifact_reference::ArtifactDialect=crate::JPG_ANY_DIALECT.into();
 for owned in [&snapshot,&demo]{let text=owned.print_dsl();let binary=owned.encode_pack_with(&Default::default()).unwrap();assert_eq!(JpgSnapshot::parse_dsl(&text).unwrap(),*owned);assert_eq!(JpgSnapshot::decode_pack_with(&binary,&Default::default()).unwrap(),*owned);let expected=owned.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for(encoding,input)in[(SnapshotEncoding::Text,store::io_schema::IoPayload::Text(text)),(SnapshotEncoding::Binary,store::io_schema::IoPayload::Binary(binary))]{let database=semio_s_artifact_stdio_contract::testing::with_decoding(|native_owner| (provider.export)(&codec.schema,&dialect,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(database,expected);let restored=semio_s_artifact_stdio_contract::testing::with_encoding(|native_owner| (provider.import)(&codec.schema,&dialect,&mut Some(database),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(restored,input);}}
 let compiled=runtime_fixture_bytes("../../../../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio");let current=std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();assert_eq!(current.as_slice(),compiled,"publication requires unchanged compiled protocol");let digest=semio_framework_hash::Sha256::digest(compiled);assert_eq!(codec.pack_schema_hash,digest);let hex=|bytes:&[u8]|bytes.iter().map(|byte|format!("{byte:02x}")).collect::<String>();
 let mut child=Command::new("bun").args(["-e","import{createHash}from'node:crypto';process.stdout.write(createHash('sha256').update(new Uint8Array(await Bun.stdin.arrayBuffer())).digest('hex')); "]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(compiled).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap(),hex(&digest));
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let fixture_path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json");
 let script=r#"import{Database}from'bun:sqlite';import{deepStrictEqual as eq}from'node:assert';const expected=await Bun.file(process.argv[1]).json(),v=expected.image,db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));try{eq(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});eq(db.query('PRAGMA foreign_key_check').all(),[]);eq(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(r=>r.name),['jpg_document','jpg_rgba_pixel','jpg_segment','jpg_segment_octet','jpg_thumbnail','jpg_thumbnail_rgb_pixel']);eq(db.query('SELECT * FROM jpg_document').all(),[{id:1,schema:expected.schema,width:v.width,height:v.height,jfif_major:v.jfifVersion[0],jfif_minor:v.jfifVersion[1],density_units:v.jfifDensityUnits,x_density:v.jfifXDensity,y_density:v.jfifYDensity}]);eq(db.query('SELECT red,green,blue,alpha FROM jpg_rgba_pixel ORDER BY ordinal').all().flatMap(r=>[r.red,r.green,r.blue,r.alpha].filter(x=>x!==null)),v.pixels);eq(db.query('SELECT width,height FROM jpg_thumbnail').all(),[{width:v.jfifThumbnail.width,height:v.jfifThumbnail.height}]);eq(db.query('SELECT red,green,blue FROM jpg_thumbnail_rgb_pixel ORDER BY ordinal').all().flatMap(r=>[r.red,r.green,r.blue].filter(x=>x!==null)),v.jfifThumbnail.rgbData);eq(db.query('SELECT id,ordinal,marker FROM jpg_segment ORDER BY ordinal').all().map((r,index)=>({marker:r.marker,data:db.query('SELECT octet FROM jpg_segment_octet WHERE segment_id=? ORDER BY ordinal').all(r.id).map(o=>o.octet)})),v.otherSegments);process.stdout.write('complete');}finally{db.close()}"#;
 let mut child=Command::new("bun").args(["-e",script]).arg(fixture_path).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(output.stdout,b"complete");
 let receipt=serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha256":hex(&digest)});eprintln!("[DEBUG] native-codec-publication={receipt}");
}

#[test]fn neutral_diff_framing_refuses_noncanonical_domains_without_allocation_panics(){
 use protocol::DiffBinary;let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🚪️io/💾️binary/🔺️diff/🧪️tests/🧫️fixtures/🛡️framing/🔣️.json")).unwrap();for vector in fixture["reject"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(vector["bytes"].clone()).unwrap();let refusal=std::panic::catch_unwind(||crate::JpgDiff::decode_diff(&bytes));assert!(refusal.is_ok(),"{} allocated from an unadmitted count",vector["name"]);assert!(refusal.unwrap().is_err(),"{}",vector["name"]);}
}

#[test]
fn independently_encoded_file_reopens_all_imported_semantic_metadata(){
 let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-image/🔣️.json")).unwrap();let image:crate::JpgImage=semio_framework_pack_json::from_json_str(&fixture["importedImage"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let independent=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_encode_rgba(image.width,image.height,&image.pixels,90).unwrap();assert_eq!(&independent[..4],&[255,216,255,224]);let old_end=4+usize::from(u16::from_be_bytes([independent[4],independent[5]]));
 let thumbnail=image.jfif_thumbnail.as_ref().unwrap();let mut payload=b"JFIF\0".to_vec();payload.extend_from_slice(&[image.jfif_version.0,image.jfif_version.1,2]);payload.extend_from_slice(&image.jfif_x_density.to_be_bytes());payload.extend_from_slice(&image.jfif_y_density.to_be_bytes());payload.extend_from_slice(&[thumbnail.width,thumbnail.height]);payload.extend_from_slice(&thumbnail.rgb_data);
 let mut native=vec![255,216,255,224];native.extend_from_slice(&u16::try_from(payload.len()+2).unwrap().to_be_bytes());native.extend(payload);for segment in &image.other_segments{native.extend_from_slice(&[255,segment.marker]);native.extend_from_slice(&u16::try_from(segment.data.len()+2).unwrap().to_be_bytes());native.extend_from_slice(&segment.data);}native.extend_from_slice(&independent[old_end..]);
 let reopened=decode_jpg(&native).unwrap();assert_eq!((reopened.image.width,reopened.image.height),(image.width,image.height));assert_eq!(reopened.image.jfif_version,image.jfif_version);assert_eq!(reopened.image.jfif_density_units,image.jfif_density_units);assert_eq!((reopened.image.jfif_x_density,reopened.image.jfif_y_density),(image.jfif_x_density,image.jfif_y_density));assert_eq!(reopened.image.jfif_thumbnail,image.jfif_thumbnail);assert_eq!(reopened.image.other_segments,image.other_segments);
 let restored=decode_jpg(&encode_jpg(&reopened,&JpgEncodeOptions::default()).unwrap()).unwrap();assert_eq!(restored.image.jfif_thumbnail,image.jfif_thumbnail);assert_eq!(restored.image.other_segments,image.other_segments);assert_eq!(restored.image.jfif_x_density,image.jfif_x_density);
}


//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (the recipe's §4 deliverable checklist item 6) —
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
/// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
/// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — mirrors png's own
/// identically-named module exactly (same six laws, same structure, only the demo-case helpers
/// differ per the recipe's own note that every pilot's `conformance_laws` module is
/// near-identical).
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, snapshot};
    use crate::JpgMutation;
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary, OpText};

    /// 🧫️ Admits the committed native mutation corpus and covers every declared variant.
    fn demo_mutation_cases() -> Vec<JpgMutation> {
        let mut cases = vec![];
        for text in [
            runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪪️change-jfif/🎯️direct/🦠️mutation/🔣️.json"),
            runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📥️insert-other/🎯️direct/🦠️mutation/🔣️.json"),
            runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-other/🎯️direct/🦠️mutation/🔣️.json"),
            runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔲️replace-pixels/🎯️direct/🦠️mutation/🔣️.json"),
            runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-image/🎯️direct/🦠️mutation/🔣️.json"),
        ] {
            cases.push(semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("admit committed mutation case"));
        }
        let actual = cases.iter().map(|case| <JpgMutation as protocol::SemanticMutation<JpgSnapshot>>::semantics(case).kind).collect::<Vec<_>>();
        let declared = <JpgMutation as protocol::SemanticMutation<JpgSnapshot>>::kinds().iter().map(|kind| kind.kind).collect::<Vec<_>>();
        assert_eq!(actual, declared, "native cases cover every declared variant in order");
        cases
    }

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect — independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", crate::standards::v_jfif_1_01::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (hex-dump grammar of the TEXT DSL
    /// form — jpg's real internal marker structure is `../💾️binary/📡️.protocol.semio`'s
    /// job, not this leaf's, per the recipe's own png precedent) recognizes real `print_dsl`
    /// output for the demo snapshot — same preamble-stripped body reconstruction
    /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_host_snapshot` uses.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        let text = pack::snapshot::ArtifactDsl::print_dsl(&demo_jpg_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `JpgMutation` variant (`demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `JpgDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
    /// every tri-state/`JpgFrameChange`/collection-triple shape.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_jfif_1_01::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = semio_framework_dsl::parse_protocol(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = pack::snapshot::ArtifactPack::encode_pack(&demo_jpg_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = semio_framework_dsl::parse_protocol(crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// 🧪️ Shipped Text/Pack preserve every owned demo field and equal their declared codec output.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        static FIXTURE_DSL: std::sync::LazyLock<&str> = std::sync::LazyLock::new(|| super::runtime_fixture_text("../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));
        static FIXTURE_PACK: std::sync::LazyLock<&[u8]> = std::sync::LazyLock::new(|| super::runtime_fixture_bytes("../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio"));

        let demo = demo_jpg_snapshot();

        assert_eq!(pack::snapshot::ArtifactDsl::print_dsl(&demo), (*FIXTURE_DSL), "print_dsl(demo_jpg_snapshot()) drifted from the shipped .dsl.semio fixture");
        assert_eq!(pack::snapshot::ArtifactPack::encode_pack(&demo), (*FIXTURE_PACK), "encode_pack(demo_jpg_snapshot()) drifted from the shipped .pack.semio fixture");

        let parsed = <JpgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl((*FIXTURE_DSL)).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed,demo);
        let decoded=<JpgSnapshot as pack::snapshot::ArtifactPack>::decode_pack((*FIXTURE_PACK)).expect("decode owned Pack fixture");
        assert_eq!(decoded,demo);
    }
}
//#endregion 🔖️ConformanceLaws

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}

fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
