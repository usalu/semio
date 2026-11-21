"""🔧 Type-check the exact PDF content and lexer source and exercise its focused contracts."""
from pathlib import Path
import os
import re
import subprocess
import sys
from tempfile import TemporaryDirectory


def source_without_value_derives(path):
    text = re.sub(r"(?m)^\s*#\[(?:canonical_json|value)\([^\]]*\)\]\s*", "\n", path.read_text())
    text = re.sub(r"#\[value\([^\]]*\)\]\s*", "", text)
    for macro in ("semio_framework_value::RetireOwned", "semio_framework_value::CanonicalJsonTree",
                  "semio_framework_value::RetainedClone", "value_derive::ToValue", "value_derive::FromValue"):
        text = text.replace(macro + ", ", "").replace(", " + macro, "")
    return text


def exercise(content, output, page_content=None, document_stage=None, large_fixture=None, text_fixture=None, text_streams=None):
    base = content.parents[1]
    schema = base / "🧬️schema/📸️snapshot/📐️model"
    with TemporaryDirectory(dir=output, prefix="content-semantic-") as directory:
        scratch = Path(directory)
        for lane, domain in (("object", "🔗️object-model"), ("operators", "🖋️content-operators"), ("font", "🔤️font-model")):
            (scratch / (lane + ".rs")).write_text(source_without_value_derives(schema / domain / "🦀️.rs"))
        lexer = content.parent / "🔤️lexer/🦀️.rs"
        parser = content / "🦀️.rs"
        fonts = content.parent / "🔤️fonts"
        font_codec = (fonts / "🦀️.rs").read_text().rsplit("#[cfg(test)]", 1)[0]
        cmap = (fonts / "🗺️cmap/🦀️.rs").resolve().as_posix()
        cff = (fonts / "✒️cff/🦀️.rs").read_text().replace("#[cfg(test)]\nmod tests {", "#[cfg(cff_package_tests)]\nmod tests {", 1)
        cff_path = (scratch / "cff.rs").resolve()
        cff_path.write_text(cff)
        font_codec = font_codec.replace('🗺️cmap/🦀️.rs', cmap).replace('✒️cff/🦀️.rs', cff_path.as_posix())
        (scratch / "font-codec.rs").write_text(font_codec)
        tests = content / "🧪️tests/🔬️unit/🧪️content-program.rs"
        source = scratch / "main.rs"
        source.write_text(r'''
extern crate self as semio_s_artifact_stdio_pdf_model;
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;
extern crate self as semio_framework_artifact_reference;
extern crate self as semio_s_artifact_stdio_pdf_tables;
extern crate self as semio_s_artifact_stdio_pdf_glyph_list;
#[derive(Clone, Debug, PartialEq, Eq)] pub struct ArtifactRef;
pub mod document_metadata { #[derive(Clone, Debug, PartialEq)] pub struct PdfDate; }
pub mod colour_and_graphics_state {
    #[derive(Clone, Debug, PartialEq)] pub struct PdfColorSpace;
    impl PdfColorSpace { pub fn components(&self) -> Option<u32> { unreachable!() } }
}
pub mod xobjects {
    #[derive(Clone, Debug, PartialEq)]
    pub enum PdfImageBody { Samples { values: Vec<u32> }, Artifact { reference: crate::ArtifactRef } }
}
#[path = "object.rs"] pub mod object_model;
#[path = "operators.rs"] pub mod content_operators;
#[path = "font.rs"] pub mod font_model;
pub mod standards { pub mod v1_7 { pub mod subsets { pub mod base {
    pub mod schema {
        pub mod snapshot { pub use crate::{object_model::*, content_operators::*, font_model::*, xobjects::PdfImageBody}; }
        pub mod graph_source {
            use super::snapshot::{ObjRef, PdfObject};
            pub trait ObjectSource {
                fn get(&mut self, reference: ObjRef) -> Option<PdfObject>;
                fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
                    let value = self.get(reference);
                    read(value.as_ref());
                }
                fn deref(&mut self, value: &PdfObject) -> PdfObject {
                    value.as_ref().map_or_else(|| value.clone(), |reference| self.get(reference).unwrap_or(PdfObject::Null))
                }
                fn admitted_roles(&self) -> &[()] { &[] }
                fn take_admitted_roles(&mut self) -> Vec<()> { Vec::new() }
            }
            pub struct GraphSource<'a> { pub objects: &'a [super::snapshot::PdfIndirectObject] }
            impl<'a> GraphSource<'a> { pub fn new(objects: &'a [super::snapshot::PdfIndirectObject]) -> Self { Self { objects } } }
            impl ObjectSource for GraphSource<'_> {
                fn get(&mut self, reference: ObjRef) -> Option<PdfObject> { self.objects.iter().find(|object| object.id == reference).map(|object| object.value.clone()) }
                fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
                    read(self.objects.iter().find(|object| object.id == reference).map(|object| &object.value));
                }
            }
        }
    }
    pub mod io {
        pub mod error {
            #[derive(Debug)] pub enum PdfEngineError { Malformed(String), Unsupported(String) }
            pub type PResult<T> = Result<T, PdfEngineError>;
            pub fn malformed<T>(reason: impl Into<String>) -> PResult<T> { Err(PdfEngineError::Malformed(reason.into())) }
        }
        pub mod foreign_artifacts {
            use super::error::PResult;
            use crate::{ArtifactRef, object_model::PdfObject};
            pub trait PdfArtifactResourcePort {
                fn admit(&mut self, _: &str, _: PdfObject) -> PResult<ArtifactRef>;
                fn resolve(&self, _: &ArtifactRef) -> PResult<PdfObject> { unreachable!() }
            }
            pub fn require_font_artifact_kind(_: &crate::font_model::PdfFontProgram) -> PResult<()> { unreachable!() }
            pub struct NativePdfArtifactResources;
            impl NativePdfArtifactResources { pub fn from_objects(_: &[PdfObject]) -> Self { Self } }
            impl PdfArtifactResourcePort for NativePdfArtifactResources { fn admit(&mut self, _: &str, _: PdfObject) -> PResult<ArtifactRef> { unreachable!() } }
        }
        pub mod text_string {
            pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() }
            pub fn decode_text_string(bytes: &[u8]) -> String { String::from_utf8_lossy(bytes).into_owned() }
        }
        pub mod text { pub mod snapshot { pub mod date {
            pub fn print_pdf_date(_: &crate::document_metadata::PdfDate) -> String { String::new() }
        } } }
    }
    pub mod modules {
        pub use crate::{actual_lexer as lexer, actual_content as content};
        pub mod colour {
            use crate::{colour_and_graphics_state::PdfColorSpace, object_model::PdfObject};
            use super::super::io::error::PResult;
            pub fn lift_colour_space_inline(_: &PdfObject) -> PdfColorSpace { unreachable!() }
            pub fn lower_colour_space_inline(_: &PdfColorSpace) -> PResult<PdfObject> { unreachable!() }
        }
        pub mod filters {
            use crate::object_model::{PdfDictEntry, PdfStreamFilter};
            use super::super::io::error::PResult;
            pub fn decode_stream(_: &[PdfDictEntry], _: &[u8]) -> PResult<(Vec<u8>, Vec<PdfStreamFilter>)> { unreachable!() }
            pub fn encode_stream(_: &[u8], _: &[PdfStreamFilter]) -> (Vec<u8>, Vec<PdfDictEntry>) { unreachable!() }
        }
        pub mod images {
            use super::super::io::error::PResult;
            pub fn packed_image_geometry(_: u32, _: u32, _: u32, _: u32) -> PResult<(usize, usize)> { unreachable!() }
            pub fn unpack_image_samples(_: u32, _: u32, _: u32, _: u32, _: &[u8]) -> PResult<Vec<u32>> { unreachable!() }
            pub fn pack_image_samples(_: u32, _: u32, _: u32, _: u32, _: &[u32]) -> PResult<Vec<u8>> { unreachable!() }
        }
        pub mod fonts {
            pub use crate::{metrics, tables, glyph_list, truetype};
            pub use crate::actual_fonts::*;
        }

    }
} } } }
pub use actual_lexer::{hex_val, is_delim, is_ws, parse_indirect_at, PdfByteStringCursor, StringStep, CosGrant, CosLimits, IndirectFrameCursor, IndirectFrameStatus, StreamSpan, StreamSpanCursor};
pub use object_model::{ObjRef, PdfDictEntry, PdfObject, PdfDecimal};
''' + f'''
#[path = "{lexer.resolve()}"] pub mod actual_lexer;
#[path = "{parser.resolve()}"] pub mod actual_content;
#[path = "font-codec.rs"] pub mod actual_fonts;
#[path = "{(fonts / "📏️metrics/🦀️.rs").resolve()}"] pub mod metrics;
#[path = "{(fonts / "🅰️tables/🦀️.rs").resolve()}"] pub mod tables;
#[path = "{(fonts / "🔡️glyph-list/🦀️.rs").resolve()}"] pub mod glyph_list;
#[path = "{(fonts / "🔠️truetype/🦀️.rs").resolve()}"] pub mod truetype;
#[path = "{tests.resolve()}"] mod contract;
''')
        if page_content is not None:
            page = page_content / "🦀️.rs"
            contracts = page_content / "🧪️tests/🔬️unit/🦀️.rs"
            modules = source.read_text().replace("    pub mod modules {", "    pub mod modules { pub use crate::page_content as lift;")
            source.write_text(modules + f'\n#[path = "{page.resolve()}"] pub mod page_content;\n#[path = "{contracts.resolve()}"] mod page_content_contracts;\n')
        if document_stage is not None:
            harness = Path(__file__).with_name("📜️document-lift-stage.rs")
            page_tree = document_stage.parents[2] / "🌳️page-tree/🦀️.rs"
            (scratch / "document.rs").write_text(harness.read_text().replace("@DOCUMENT@", document_stage.resolve().as_posix()).replace("@PAGE_TREE@", page_tree.resolve().as_posix()))
            document_source = source.read_text().replace("fn take_admitted_roles(&mut self) -> Vec<()>", "fn take_admitted_roles(&mut self) -> Vec<crate::document_stage::PdfAdmittedStreamRole>")
            for lane, domain in (("navigation", "🎯️navigation"), ("annotations", "📌️annotations-and-forms")):
                text = source_without_value_derives(schema / domain / "🦀️.rs")
                text = re.sub(r"fn retire_pdf_\w+\([^\n]*\) \{.*?^}\n", "", text, flags=re.S | re.M)
                (scratch / (lane + ".rs")).write_text(text)
            date_model = (schema / "📄️document-metadata/🦀️.rs").read_text()
            date_fields = date_model.split("pub struct PdfDate {", 1)[1].split("\n}", 1)[0]
            date_fields = "\n".join(line for line in date_fields.splitlines() if not line.lstrip().startswith("#[value"))
            document_source = document_source.replace("pub struct PdfDate;", "pub struct PdfDate {" + date_fields + "\n}")
            role_dir = base / "🚪️io/📝️text/📸️snapshot"
            start = document_source.index("        pub mod text_string {")
            end = document_source.index("    pub mod modules {", start)
            document_source = document_source[:start] + f'''        #[path = "{base / "🚪️io/🔤️text-string/🦀️.rs"}"] pub mod text_string;
        pub mod text {{ pub mod snapshot {{
            #[path = "{role_dir / "📅️date/🦀️.rs"}"] pub mod date;
            #[path = "{role_dir / "🪪️retained-text/🦀️.rs"}"] pub mod retained_text;
        }} }}
    }}
''' + document_source[end:]
            document_source = document_source.replace("xobjects::PdfImageBody};", "xobjects::PdfImageBody, document_metadata::PdfDate};")
            document_source += '\n#[path = "navigation.rs"] mod navigation;\n#[path = "annotations.rs"] mod annotations;\n'
            source.write_text(document_source + r''' 
#[path = "document.rs"] mod document_stage;
extern crate self as semio_s_artifact_stdio_pdf_xref_stream;
#[derive(Clone, Copy)] pub struct XrefRow { pub number: u32, pub entry: Option<()> }
#[derive(Clone, Copy, Debug, PartialEq)] pub enum XrefIndexStatus {
    More, NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize }, Cancelled, Closed, Inserted, AlreadyKnown,
}
pub struct IndexReceipt { pub status: Result<XrefIndexStatus, &'static str>, pub work: usize, pub allocated_bytes: usize, pub released_bytes: usize }
#[derive(Default)] pub struct XrefIndex { observed: std::collections::BTreeSet<u32>, pending: Option<XrefRow> }
impl XrefIndex {
    pub fn offer(&mut self, row: XrefRow) -> bool { self.pending.replace(row).is_none() }
    pub fn begin_close(&mut self) {}
    pub fn close_step(&mut self, _: CosGrant) -> IndexReceipt { index_turn(XrefIndexStatus::Closed, 0) }
    pub fn advance(&mut self, grant: CosGrant, _: bool) -> IndexReceipt {
        if grant.work == 0 { return index_turn(XrefIndexStatus::More, 0); }
        let row = self.pending.take().expect("admitted before indexing");
        let status = if self.observed.insert(row.number) { XrefIndexStatus::Inserted } else { XrefIndexStatus::AlreadyKnown };
        index_turn(status, 1)
    }
}
fn index_turn(status: XrefIndexStatus, work: usize) -> IndexReceipt {
    IndexReceipt { status: Ok(status), work, allocated_bytes: 0, released_bytes: 0 }
}
''')
        binary = scratch / "content-program-test"
        subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                        "--edition=2021", "-Awarnings", "--test", str(source), "-o", str(binary)], check=True)
        subprocess.run([str(binary), "contract::", "--nocapture"], check=True)
        if page_content is not None:
            subprocess.run([str(binary), "page_content_contracts::", "--nocapture"], check=True)
        if document_stage is not None:
            environment = os.environ.copy()
            if large_fixture is not None:
                environment["PDF_CONTROLLED_LIFT_FIXTURE"] = str(large_fixture)
            if text_fixture is not None and text_streams is not None:
                environment["PDF_CONTROLLED_TEXT_FIXTURE"] = str(text_fixture)
                environment["PDF_CONTROLLED_TEXT_STREAMS"] = str(text_streams)
            subprocess.run([str(binary), "document_stage::", "--skip", "document_stage::tree::", "--nocapture"], check=True, env=environment)


if __name__ == "__main__":
    exercise(*map(Path, sys.argv[1:]))
