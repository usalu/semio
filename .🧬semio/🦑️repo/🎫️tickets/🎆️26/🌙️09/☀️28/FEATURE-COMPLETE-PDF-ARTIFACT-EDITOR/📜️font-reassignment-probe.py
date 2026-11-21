"""🎚️ Verifies the actual PDF font codec preserves the selected run across encodings."""

from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
probe = ticket / "📜️cff-file-codec-probe.py"
stage = ticket / "🗑️generated/text-font-edit"

base = Path(sys.argv[1]).resolve()
objects = (stage / "object-edits.rs").read_text()
objects = objects[objects.index("pub(super) fn set_font_of_text("):objects.index("//#endregion")]
operations = (stage / "content-ops.rs").read_text()
operations = operations[operations.index("pub(super) fn ensure_font("):operations.index("pub(super) fn replace_shown_text(")]
font_path = "semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts"
metrics = (stage / "metrics.rs").read_text().replace(font_path, "crate::fonts").replace("NativePdfArtifactResources<'a>", "NativePdfArtifactResources")
(stage / "metrics-probe.rs").write_text(metrics)
objects = objects.replace(font_path, "crate::fonts")
operations = operations.replace(font_path, "crate::fonts")

module = r'''
    mod page_edits {
        use crate::{content_operators::PdfOp, font_model::PdfFont};
        #[path = "@METRICS@"] mod metrics;
        use metrics::FontMeasurements;
        pub struct PdfSnapshot { pub fonts: Vec<PdfFont>, pub objects: Vec<()>, pub pages: Vec<PdfPage> }
        pub struct PdfPage { pub content: Vec<PdfOp> }
        impl PdfSnapshot { pub fn font(&self, name: &str) -> Option<&PdfFont> { self.fonts.iter().find(|font| font.id == name) } }
        pub struct SetFont { pub font: PdfFont, pub index: Option<usize> }
        pub struct SetPageContent { pub index: usize, pub content: Vec<PdfOp> }
        pub enum PdfMutation { SetFont(SetFont), SetPageContent(SetPageContent) }
        #[derive(PartialEq)] enum ObjectKind { Text }
        struct PageObject { kind: ObjectKind, font: String, font_size: f64, page: usize, end: usize }
        type Fault = String;
        fn fault(reason: impl Into<String>) -> Fault { reason.into() }
        fn find_object(snapshot: &PdfSnapshot, _: &str) -> Option<PageObject> {
            let content = &snapshot.pages.first()?.content;
            let end = content.iter().position(|op| matches!(op, PdfOp::ShowTextArray { .. }))?;
            Some(PageObject { kind: ObjectKind::Text, font: "Fsrc".into(), font_size: 20., page: 0, end })
        }
        fn text_cursor_after(content: &[PdfOp], object: &PageObject) -> Result<Vec<PdfOp>, Fault> {
            assert!(matches!(content[object.end + 1], PdfOp::NextLine));
            Ok(Vec::new())
        }
        @OPERATIONS@
        @OBJECTS@
    }
'''.replace("@OPERATIONS@", operations).replace("@OBJECTS@", objects).replace("@METRICS@", (stage / "metrics-probe.rs").as_posix())

body = r'''
    use crate::content::{parse_content, print_content};
    use crate::content_operators::{PdfOp, PdfTextArrayItem, PdfTextString};
    use crate::font_model::{PdfBaseEncoding, PdfEncodingDifference, PdfFont, PdfFontKind};
    use crate::fonts::FontCodec;
    use crate::foreign_artifacts::NativePdfArtifactResources;
    use crate::xref::{Resolver, build_xref};
    use std::collections::HashMap;

    let input = include_bytes!("@CFF_PDF@");
    let mut resolver = Resolver::new(input, build_xref(input).unwrap().entries);
    let crate::PdfObject::Stream { data, .. } = resolver.resolve(4).unwrap() else { unreachable!() };
    let resources = NativePdfArtifactResources::from_objects(&[]);
    let mut source = PdfFont::standard("Fsrc", "Helvetica");
    let PdfFontKind::Type1 { encoding, .. } = &mut source.kind else { unreachable!() };
    encoding.differences.push(PdfEncodingDifference { code: 70, glyph: "eacute".into() });
    let mut target = PdfFont::standard("Fdst", "Times-Roman");
    let PdfFontKind::Type1 { encoding, .. } = &mut target.kind else { unreachable!() };
    encoding.base = Some(PdfBaseEncoding::MacRoman);
    let mut fonts = HashMap::new();
    fonts.insert(source.id.clone(), FontCodec::new(&source, &resources).unwrap());
    fonts.insert(target.id.clone(), FontCodec::new(&target, &resources).unwrap());
    let parsed = parse_content(&data, &fonts, &mut NativePdfArtifactResources::from_objects(&[])).unwrap();
    let index = parsed.iter().position(|op| matches!(op, PdfOp::ShowTextArray { .. })).unwrap();
    assert_eq!(parsed[index], PdfOp::ShowTextArray { items: vec![
        PdfTextArrayItem::Codes { codes: vec![233] }, PdfTextArrayItem::Adjust { amount: -50.0 }, PdfTextArrayItem::Codes { codes: vec![233] },
    ]});
    let original = fonts.get("Fsrc").unwrap();
    let replacement = fonts.get("Fdst").unwrap();
    assert_eq!(original.encode("é").as_deref(), Some(b"F".as_slice()));
    assert_eq!(replacement.encode("é").as_deref(), Some([142].as_slice()));
    assert!(replacement.rebind_show(&PdfOp::ShowText { text: PdfTextString::text("中文") }, Some(original)).is_none());
    assert!(replacement.rebind_show(&parsed[index], None).is_none());
    @PAGE_EDITS@
    let snapshot = page_edits::PdfSnapshot { fonts: vec![source, target], pages: vec![page_edits::PdfPage { content: parsed }], objects: Vec::new() };
    assert!(page_edits::ensure_font(&snapshot, "中文").is_err());
    assert_eq!(page_edits::ensure_font(&snapshot, "é\nTail").unwrap().0, "Fsrc");
    assert!(page_edits::set_font_of_text(&snapshot, "selected", "NotAFont").is_err());
    let edits = page_edits::set_font_of_text(&snapshot, "selected", "Times-Roman").unwrap();
    let [page_edits::PdfMutation::SetPageContent(edit)] = edits.as_slice() else { panic!("only the selected page changes"); };
    assert_eq!(edit.index, 0);
    assert!(matches!(&edit.content[index], PdfOp::SetFont { name, size } if name == "Fdst" && *size == 20.));
    assert_eq!(edit.content[index + 1], PdfOp::ShowTextArray { items: vec![
        PdfTextArrayItem::Text { text: "é".into() }, PdfTextArrayItem::Adjust { amount: -50.0 }, PdfTextArrayItem::Text { text: "é".into() },
    ]});
    let output = print_content(&edit.content, &fonts).unwrap();
    std::fs::write("@OUT@", output).unwrap();
    println!("[DEBUG] Native content codec transcoded the two original 233 codes, preserved kerning, and rejected unsupported text");
}
'''.replace("@PAGE_EDITS@", module).replace("@OUT@", (ticket / "🗑️generated/font-reassignment-native.stream").as_posix())
runpy.run_path(str(probe))["exercise"](sys.argv[1], "font-reassignment-before.pdf", body, stage)
