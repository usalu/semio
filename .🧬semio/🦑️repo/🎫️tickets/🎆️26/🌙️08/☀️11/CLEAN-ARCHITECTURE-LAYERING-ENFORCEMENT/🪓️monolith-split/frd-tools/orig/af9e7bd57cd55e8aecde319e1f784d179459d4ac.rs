use crate::NoteBlockNode;
use semio_framework_plugin::app::IoPayload;
use serde_json::Value;
#[allow(unused_imports)]
use crate::{NoteSnapshot, NoteTextParagraph, NoteTextRun};
#[allow(unused_imports)]
use {semio_framework_plugin::app::io_dispatch, semio_framework_artifact_reference::Dialect, semio_framework_plugin::app::ErasedComposeSource, semio_framework_plugin::app::IoDirection, semio_framework_plugin::app::IoKey, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
#[allow(unused_imports)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing;
#[allow(unused_imports)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::io as semio_drawing_composer;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::write_svg_xml;
use crate::standards::v1::subsets::any::io::*;
use semio_framework_os_kernel::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload as ForeignPayload;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::{
    io::{decode_pdf, encode_pdf},
    schema::snapshot::PdfSnapshot,
};

#[semio_framework_async_macros::async_test]
async fn note_pdf14_page_contract_matches_the_json_oracle() {
    let fixture: Value = serde_json::from_str(runtime_fixture_text("../../🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/📖️pdf14-pages.json")).expect("neutral PDF vectors");
    for row in fixture["cases"].as_array().expect("cases") {
        let pdf: PdfSnapshot = semio_framework_pack_json::from_json_str(&serde_json::json!({"schema": semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::STDIO_PDF_DOCUMENT_SCHEMA, "pages": row["pages"]}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned PDF snapshot");
        let bytes = encode_pdf(&pdf).expect("PDF 1.4 writer");
        assert!(bytes.starts_with(b"%PDF-1.4"));
        let note = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf::v1_4::base::PdfIntoNote::deserialize(&ForeignPayload::Binary(bytes)).await.expect("PDF import").value;
        let NoteBlockNode::Text { width, height, content, .. } = &note.blocks[0] else { panic!("PDF page text block") };
        let text: String = crate::note_block_text(content).iter().flat_map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str())).collect();
        let imported = serde_json::json!({"width": width, "height": height, "text": text});
        assert_eq!(imported, row["expectedImport"]);
        let exported = crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::base::NoteIntoPdf::serialize(&note, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("PDF export").value;
        let ForeignPayload::Binary(bytes) = exported else { panic!("binary PDF export") };
        assert!(bytes.starts_with(b"%PDF-1.4"));
        let pdf = decode_pdf(&bytes).expect("exported PDF decode");
        assert_eq!(pdf.pages.len(), 1);
        let actual: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&pdf.pages[0])).expect("independent page JSON oracle");
        assert_eq!(actual["text"], row["expectedExport"]["text"]);
        assert_eq!(actual["width"].as_f64(), row["expectedExport"]["width"].as_f64());
        assert_eq!(actual["height"].as_f64(), row["expectedExport"]["height"].as_f64());
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
