//! 🧪️ Page-domain observations of the PDF 1.4 sqlite snapshot against the resolved page corpus.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
use store::sqlite_snapshot;
#[allow(unused_imports)]
use semio_framework_value::{ValueError, ValueRefusalKind};
#[allow(unused_imports)]
use store::sqlite_snapshot::{SqliteDatabase as Db, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::Cell as C, artifact::Projection, artifact::FloatColumn, artifact::FloatRow, artifact::insert_ieee754};
use crate::standards::v1_4::subsets::base::io::sqlite::snapshot::*;
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
static CORPUS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🧪️tests/🧫️fixtures/📄️resolved-page-domain/🔣️.json"));
fn owner(edited: bool) -> PdfSnapshot {
    let fixture: serde_json::Value = serde_json::from_str((*CORPUS)).unwrap();
    let pages = fixture["pages"].as_array().unwrap().iter().enumerate().map(|(index, page)| {
        let values = if edited && index == fixture["sqlEdit"]["page"].as_u64().unwrap() as usize { &fixture["sqlEdit"] } else { page };
        PageDoc { width: values["width"].as_f64().unwrap(), height: values["height"].as_f64().unwrap(), text: values["text"].as_str().unwrap().into() }
    }).collect();
    PdfSnapshot { schema: fixture["schema"].as_str().unwrap().into(), pages }
}
fn independent_edit(bytes: &[u8], encoding: &str) -> Vec<u8> {
    use std::{io::Write, process::{Command, Stdio}};
    let script = r#"import {Database} from 'bun:sqlite';import {Buffer} from 'node:buffer';const fixture=JSON.parse(Bun.argv.at(-2)),db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});const same=(a,b)=>{if(JSON.stringify(a)!==JSON.stringify(b))throw Error('complete independent PDF14 owner query mismatch');};same(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});same(db.query('PRAGMA foreign_key_check').all(),[]);same(db.query("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").all().map(x=>x.name),['pdf14_document','pdf14_page','semio_snapshot']);same(db.query('SELECT CAST(id AS TEXT) id,artifact_kind,standard,subset,CAST(schema_version AS TEXT) schema_version,native_encoding FROM semio_snapshot').all(),[{id:'1',artifact_kind:'s.stdio.pdf',standard:'1.4',subset:'*',schema_version:'1',native_encoding:Bun.argv.at(-1)}]);const word=value=>{const b=Buffer.alloc(8);b.writeDoubleLE(value);return b.readBigInt64LE().toString();};same(db.query('SELECT d.schema,CAST(p.ordinal AS TEXT) ordinal,p.width,p.height,p.text,CAST(p.width_bits AS TEXT) widthBits,CAST(p.height_bits AS TEXT) heightBits,p.width_class widthClass,p.height_class heightClass FROM pdf14_document d JOIN pdf14_page p ON p.document_id=d.id ORDER BY p.ordinal').all(),fixture.pages.map((p,i)=>({schema:fixture.schema,ordinal:String(i),width:p.width,height:p.height,text:p.text,widthBits:word(p.width),heightBits:word(p.height),widthClass:'finite',heightClass:'finite'})));const e=fixture.sqlEdit;db.query("UPDATE pdf14_page SET width=?,height=?,text=?,width_bits=CAST(? AS INTEGER),height_bits=CAST(? AS INTEGER),width_class='finite',height_class='finite' WHERE ordinal=?").run(e.width,e.height,e.text,word(e.width),word(e.height),e.page);same(db.query('PRAGMA foreign_key_check').all(),[]);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
    let mut child = Command::new("bun").args(["-e", script, (*CORPUS), encoding]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pdf14_populated_actual_owner_public_both_forms_and_independent_native_edit() {
    use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot};
    crate::root_tests::register_sqlite_test_declaration();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.pdf".into(), standard: "1.4".into(), subset: "*".into() };
    let expected = owner(false);
    let edited = owner(true);
    let limits = sqlite_snapshot::SqliteDatabaseLimits::default();
    for encoding in [sqlite_snapshot::SnapshotEncoding::Binary, sqlite_snapshot::SnapshotEncoding::Text] {
        let file = async{let original_file_dialect=&dialect;let original_file_input=&expected;let original_file_encoding=encoding;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;
        assert_eq!(&file[..16], b"SQLite format 3\0");
        let restored = async{let original_file_dialect=&dialect;let original_file_input=&file;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<PdfSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;
        assert_eq!(restored, expected);
        restored.retire_sqlite_snapshot();
        let raw = independent_edit(&file, match encoding { sqlite_snapshot::SnapshotEncoding::Binary => "binary", sqlite_snapshot::SnapshotEncoding::Text => "text" });
        let changed = async{let original_file_dialect=&dialect;let original_file_input=&raw;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<PdfSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;
        assert_eq!(changed, edited);
        let native = crate::standards::v1_4::subsets::base::io::encode_pdf(&changed).unwrap();
        assert!(native.starts_with(b"%PDF-1.4"));
        let parsed = crate::standards::v1_4::subsets::base::io::decode_pdf(&native).unwrap();
        assert_eq!(parsed, edited);
        parsed.retire_sqlite_snapshot();
        changed.retire_sqlite_snapshot();
    }
    assert_eq!(expected, owner(false));
    expected.retire_sqlite_snapshot();
    edited.retire_sqlite_snapshot();
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
