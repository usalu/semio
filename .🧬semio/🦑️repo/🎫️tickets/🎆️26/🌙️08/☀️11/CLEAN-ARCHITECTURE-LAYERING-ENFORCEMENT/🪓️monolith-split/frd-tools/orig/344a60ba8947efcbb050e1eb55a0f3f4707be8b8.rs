#[allow(unused_imports)]
use semio_framework_plugin::ExampleSource;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
use crate::examples::demo::*;
#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}

#[test]
fn sqlite_snapshot_pdf14_demo_assets_share_the_owned_page_snapshot() {
    use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
    use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
    let expected = PdfSnapshot { schema: crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: "Semio Demo".into() }] };
    assert_eq!(PdfSnapshot::parse_dsl(PRIMARY_TEXT).unwrap(), expected);
    let bytes = expected.encode_pack();
    let published = runtime_fixture_bytes("../../../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");
    let owned_hex = bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(published.as_slice(), bytes, "expected owned pack {owned_hex}");
    assert_eq!(PdfSnapshot::decode_pack(published).unwrap(), expected);
}

fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
