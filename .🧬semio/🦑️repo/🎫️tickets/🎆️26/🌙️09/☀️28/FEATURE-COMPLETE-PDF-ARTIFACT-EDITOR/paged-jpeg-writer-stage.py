"""🌊️ Stages the exact PDF DCT emitter and its focused live-writer harness."""
import runpy
from pathlib import Path

TICKET = Path(__file__).resolve().parent
jpeg_stage = runpy.run_path(str(TICKET / "paged-jpeg-stage.py"))
jpeg_stage["stage"]()
one = jpeg_stage["one"]
BASE = jpeg_stage["PDF"].parent.parent
JPEG_STAGE = jpeg_stage["STAGE"]
OUT = TICKET / "🗑️generated" / "paged-jpeg-writer"


def source(relative):
    return (BASE / relative).read_text()


def span(text, first, last):
    start = text.index(first)
    return text[start:text.index(last, start)]


def stage():
    foreign = (JPEG_STAGE / "pdf-foreign-artifacts.rs").read_text()
    foreign = one(foreign, 'const DCT_KIND: &str', '''#[path = "📖️dct-view.rs"]
pub mod dct_view;

const DCT_KIND: &str''')
    editor = (JPEG_STAGE / "pdf-baseline-resource.rs").read_text()
    editor = one(editor, "use semio_framework_artifact_reference::ArtifactRef;", """use semio_framework_artifact_reference::ArtifactRef;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::dct_view::{
    PdfDctImageColor, PdfDctImageView,
};""")
    editor = one(editor, "    /// 📤️ Transfers the admitted retained owner", """    /// 🪟️ Lends proved DCT graph identity, color metadata, and exact encoded custody together.
    pub fn stream_view(&self) -> Option<PdfDctImageView<'_>> {
        let color = match self.metadata.components {
            1 => PdfDctImageColor::Gray { transform: self.metadata.color_transform.map(|value| value != 0) },
            3 => PdfDctImageColor::Rgb { transform: self.metadata.color_transform.map(|value| value != 0) },
            _ => return None,
        };
        let dimensions = (u16::try_from(self.metadata.width).ok()?, u16::try_from(self.metadata.height).ok()?);
        PdfDctImageView::from_admitted(self.encoded()?, dimensions, color, self.digest)
    }

    /// 📤️ Transfers the admitted retained owner""")

    writer = source("🔨️modules/📝️writer/🦀️.rs")
    writer = one(writer, "use std::collections::BTreeMap;", '''use std::collections::BTreeMap;

#[path = "🌊️dct-object.rs"]
pub mod dct_object;''')
    lexer = source("🔨️modules/🔤️lexer/🦀️.rs")
    filters = source("🔨️modules/🗜️filters/🦀️.rs")
    xref = source("🔨️modules/🔗️xref/🦀️.rs")
    read = Path.cwd() / "🧰️framework/🔨️modules/🌱️value/📦️paged/📖️read/🎟️.rs"
    closure = {
        "writer.rs": writer,
        "🌊️dct-object.rs": (TICKET / "paged-pdf-dct-writer.rs").read_text(),
        "foreign-artifacts.rs": foreign,
        "📖️dct-view.rs": (TICKET / "paged-pdf-dct-view.rs").read_text(),
        "document-admission.rs": editor,
        "probe.rs": (TICKET / "paged-jpeg-writer-probe.rs").read_text(),
        "identity.rs": span(foreign, "const DCT_KIND:", "/// 🛂️ Native admission"),
        "lexer.rs": span(lexer, "pub fn is_delim", "// 🚫️async:") + span(lexer, "pub fn number_text", "//#endregion 🔖️Writer"),
        "filters.rs": span(filters, "pub fn encode_stream", "//#endregion 🔖️Pipeline"),
        "xref.rs": next(line for line in xref.splitlines() if line.startswith("pub const WRITER_TRAILER_KEYS:")),
        "byte-source.rs": read.read_text(),
        "hash.rs": (JPEG_STAGE / "live-hash.rs").read_text(),
    }
    OUT.mkdir(parents=True, exist_ok=True)
    for name, contents in closure.items():
        (OUT / name).write_text(contents)


if __name__ == "__main__":
    stage()
