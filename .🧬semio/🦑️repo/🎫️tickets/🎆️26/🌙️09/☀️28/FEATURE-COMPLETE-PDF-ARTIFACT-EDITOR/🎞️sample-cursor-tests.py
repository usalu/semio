"""🔎️ Runs the precise sampled-image admission source with the native Rust toolchain."""

from pathlib import Path
from tempfile import TemporaryDirectory
import re
import subprocess
import sys


def bare_schema(path):
    text = re.sub(r"#\[(?:canonical_json|value)\([^\]]*\)\]\s*", "", path.read_text())
    macros = r"(?:semio_framework_value::\w+|value_derive::\w+)"
    text = re.sub(macros + r"\s*,\s*", "", text)
    return re.sub(r",\s*" + macros, "", text)


def exercise(base, ticket):
    output = ticket / "🗑️generated"
    source = base / "🔨️modules/🖼️images"
    parent = output / "🎞️images-parent.rs"
    cursor = output / "🎞️image-cursor.rs"
    if (source / "🎞️controlled/🦀️.rs").exists() or not parent.exists():
        parent = source / "🦀️.rs"
        cursor = source / "🎞️controlled/🦀️.rs"
    schema = base / "🧬️schema/📸️snapshot/📐️model"
    with TemporaryDirectory(dir=output, prefix="controlled-images-") as directory:
        scratch = Path(directory)
        model = bare_schema(schema / "🔗️object-model/🦀️.rs")
        (scratch / "object.rs").write_text(model)
        xobjects = bare_schema(schema / "🖼️xobjects/🦀️.rs")
        (scratch / "xobjects.rs").write_text(xobjects)
        images = parent.read_text()
        images = images.replace("🎞️controlled/🦀️.rs", cursor.as_posix())
        images = images.replace("🎭️sampled-alpha/🦀️.rs", (source / "🎭️sampled-alpha/🦀️.rs").as_posix())
        (scratch / "images.rs").write_text(images)
        main = scratch / "main.rs"
        lexer = base / "🔨️modules/🔤️lexer/🦀️.rs"
        main.write_text(f'#[path = "{lexer}"] pub mod lexer;\n' + HARNESS)
        binary = scratch / ("samples.exe" if sys.platform == "win32" else "samples")
        subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                        "--edition=2021", "-Awarnings", "--test", str(main), "-o", str(binary)], check=True)
        subprocess.run([str(binary), "images::sample_tests::", "--nocapture"], check=True)


HARNESS = r'''
extern crate self as semio_s_artifact_stdio_pdf_model;
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;
extern crate self as semio_framework_artifact_reference;
#[derive(Clone, Debug, PartialEq, Eq)] pub struct ArtifactRef;
pub mod document_metadata { #[derive(Clone, Debug, PartialEq)] pub struct PdfDate; }
#[path = "object.rs"] pub mod object_model;
#[path = "xobjects.rs"] pub mod xobjects;
#[path = "images.rs"] pub mod images;
pub use lexer::{hex_val, is_ws, is_delim, parse_indirect_at, PdfByteStringCursor, StringStep, CosGrant, CosLimits, IndirectFrameCursor, IndirectFrameStatus, StreamSpan, StreamSpanCursor};
pub use object_model::{ObjRef, PdfDecimal, PdfDictEntry, PdfObject};
pub mod content_operators {
    #[derive(Clone, Debug, PartialEq)] pub struct PdfOp;
    #[derive(Clone, Debug, PartialEq)] pub struct PdfInlineImage {
        pub image_mask: bool, pub color_space: Option<crate::colour_and_graphics_state::PdfColorSpace>,
        pub width: u32, pub height: u32, pub bits_per_component: u32, pub decode: Vec<f64>,
        pub body: crate::xobjects::PdfImageBody,
    }
}
pub mod colour_and_graphics_state {
    #[derive(Clone, Debug, PartialEq)] pub enum PdfColorSpace { DeviceGray, DeviceRgb, DeviceCmyk }
    impl PdfColorSpace {
        pub fn components(&self) -> Option<u32> { Some(match self { Self::DeviceGray => 1, Self::DeviceRgb => 3, Self::DeviceCmyk => 4 }) }
    }
}
pub mod standards { pub mod v1_7 { pub mod subsets { pub mod base {
    pub mod schema {
        pub mod snapshot {
            pub use crate::{object_model::*, xobjects::*, content_operators::*, colour_and_graphics_state::*};
            #[derive(Default)] pub struct PdfSnapshot {
                pub images: Vec<PdfImage>, pub admitted_stream_roles: Vec<super::stream_roles::PdfAdmittedStreamRole>,
            }
        }
        pub mod stream_roles {
            use super::snapshot::{ObjRef, PdfImage};
            #[derive(Clone)] pub struct PdfGraphIdentity { pub owner: ObjRef, pub path: Vec<()> }
            pub struct PdfAdmittedStreamRole {
                pub identity: PdfGraphIdentity, pub dependencies: Vec<PdfGraphIdentity>,
                pub value: PdfStreamRoleValue,
            }
            pub enum PdfStreamRoleValue { Image { image: PdfImage } }
        }
        pub mod graph_source { pub trait ObjectSource { fn inspect(&mut self, owner: crate::ObjRef, read: &mut dyn FnMut(Option<&crate::PdfObject>)); } }
    }
    pub mod io {
        pub mod error {
            #[derive(Debug)] pub enum PdfEngineError { Malformed(String), Unsupported(String) }
            pub type PResult<T> = Result<T, PdfEngineError>;
            pub fn malformed<T>(reason: impl Into<String>) -> PResult<T> { Err(PdfEngineError::Malformed(reason.into())) }
        }
        pub mod foreign_artifacts {
            use super::error::PResult;
            use crate::{ArtifactRef, PdfObject};
            pub trait PdfArtifactResourcePort { fn admit(&mut self, _: &str, _: PdfObject) -> PResult<ArtifactRef>; }
            #[derive(Default)] pub struct NativePdfArtifactResources;
            impl PdfArtifactResourcePort for NativePdfArtifactResources { fn admit(&mut self, _: &str, _: PdfObject) -> PResult<ArtifactRef> { unreachable!() } }
        }
        pub mod text_string { pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() } }
        pub mod text { pub mod snapshot { pub mod date {
            pub fn print_pdf_date(_: &crate::document_metadata::PdfDate) -> String { String::new() }
        } } }
    }
    pub mod modules {
        pub use crate::{lexer, images};
        pub mod xref {
            use crate::{ArtifactRef, PdfObject};
            use super::super::io::error::PResult;
            pub trait ObjectSink { fn resolve_artifact(&self, _: &ArtifactRef) -> PResult<PdfObject>; }
        }
        pub mod colour {
            use crate::{PdfDictEntry, PdfObject};
            use crate::colour_and_graphics_state::PdfColorSpace;
            use super::super::{io::error::PResult, schema::graph_source::ObjectSource};
            use super::xref::ObjectSink;
            pub fn extra_entries(_: &[PdfDictEntry], _: &[&str]) -> Vec<PdfDictEntry> { unreachable!() }
            pub fn lift_colour_space(_: &PdfObject, _: &mut dyn ObjectSource) -> PdfColorSpace { unreachable!() }
            pub fn lower_colour_space(_: &PdfColorSpace, _: &mut dyn ObjectSink) -> PResult<PdfObject> { unreachable!() }
            pub fn numbers_of(_: Option<&PdfObject>) -> Vec<f64> { unreachable!() }
            pub fn push_opt(_: &mut Vec<PdfDictEntry>, _: &str, _: Option<PdfObject>) { unreachable!() }
            pub fn raw_stream(_: Vec<PdfDictEntry>, _: Vec<u8>) -> PdfObject { unreachable!() }
        }
    }
} } } }
'''


if __name__ == "__main__":
    exercise(Path(sys.argv[1]).resolve(), Path(__file__).resolve().parent)
