"""🧭️ Checks the exact prepared PDF semantic sources with a borrowed native image fixture."""

from pathlib import Path
from tempfile import TemporaryDirectory
import os
import re
import runpy
import sys


TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"
IMAGE_NATIVE = runpy.run_path(str(TICKET / "🎞️sample-cursor-tests.py"))


def patch_harness(scratch, main, base):
    lift = base / "🔨️modules/⬇️lift/🧩️lifter-and-document"
    image_root = base / "🔨️modules/🖼️images"
    schema = base / "🧬️schema/📸️snapshot/📐️model"
    font_root = base / "🔨️modules/🔤️fonts"
    font_copy = scratch / "font-codec.rs"
    font_copy.write_text(re.sub(r'#\[path = "([^"]+)"\]', lambda m: f'#[path = "{(font_root / m[1]).as_posix()}"]', font_copy.read_text()))
    bare_schema = IMAGE_NATIVE["bare_schema"]
    image_module = output_source("🎞️images-parent.rs", image_root / "🦀️.rs").read_text()
    image_module = image_module.replace("🎞️controlled/🦀️.rs", output_source("🎞️image-cursor.rs", image_root / "🎞️controlled/🦀️.rs").as_posix())
    image_module = image_module.replace("🎭️sampled-alpha/🦀️.rs", (image_root / "🎭️sampled-alpha/🦀️.rs").as_posix())
    (scratch / "images.rs").write_text(image_module)
    (scratch / "xobjects.rs").write_text(bare_schema(schema / "🖼️xobjects/🦀️.rs"))

    program_root = lift / "📜️page-content"
    compiler = (program_root / "🦀️.rs").read_text()
    for relative in ("🧭️controlled/🦀️.rs", "🧺️program/🦀️.rs"):
        file = program_root / relative
        selected = output_source("🎞️program-integration.rs", file) if relative.startswith("🧺") else file
        compiler = compiler.replace(relative, selected.as_posix())
    (scratch / "page_content.rs").write_text(compiler)
    document_root = lift / "🧭️document-lift"
    document = (document_root / "🦀️.rs").read_text()
    document = document.replace("🧭️controlled/🦀️.rs", output_source("🎞️document-integration.rs", document_root / "🧭️controlled/🦀️.rs").as_posix())
    stage = scratch / "document_lift.rs"
    stage.write_text(document)

    document_shim = scratch / "document.rs"
    text = document_shim.read_text()
    text = text.replace((document_root / "🦀️.rs").as_posix(), stage.as_posix())
    text = text.replace("#[derive(Debug, PartialEq)]\npub enum PdfColorSpace { DeviceGray, DeviceRgb, DeviceCmyk }", "pub use crate::colour_and_graphics_state::PdfColorSpace;")
    text = text.replace("pub enum PdfStreamRoleValue { Operators { content: Vec<PdfOp> } }", "pub enum PdfStreamRoleValue { Image { image: crate::xobjects::PdfImage }, Operators { content: Vec<PdfOp> } }")
    for field in ["owner", "path", "identity", "dependencies", "value"]:
        text = text.replace(f" {{ {field}:", f" {{ pub {field}:", 1) if field in ["owner", "identity"] else text
    text = text.replace(", path: Vec<PdfGraphPath> }", ", pub path: Vec<PdfGraphPath> }", 1)
    text = text.replace(", dependencies: Vec<PdfGraphIdentity>, value: PdfStreamRoleValue }", ", pub dependencies: Vec<PdfGraphIdentity>, pub value: PdfStreamRoleValue }", 1)
    text = text.replace("    fonts: Vec<crate::font_model::PdfFont>,", "    fonts: Vec<crate::font_model::PdfFont>, pub images: Vec<crate::xobjects::PdfImage>,")
    text = text.replace("admitted_stream_roles: Vec<PdfAdmittedStreamRole>, objects:", "pub admitted_stream_roles: Vec<PdfAdmittedStreamRole>, objects:", 1)
    text = text.replace("objects: Vec::new(), fonts: Vec::new(),", "objects: Vec::new(), fonts: Vec::new(), images: Vec::new(),")
    text = text.replace("next_annotation_body_bytes()", "next_inline_body_bytes()")
    text = text.replace("let PdfStreamRoleValue::Operators { content } = &role.value;", "let PdfStreamRoleValue::Operators { content } = &role.value else { unreachable!(); };")
    text = text.replace("2 * (std::mem::size_of::<PdfPage>() + std::mem::size_of::<PdfAdmittedStreamRole>())", "2 * std::mem::size_of::<PdfPage>() + snapshot.admitted_stream_roles.capacity() * std::mem::size_of::<PdfAdmittedStreamRole>()")
    document_shim.write_text(text + (TICKET / "🎞️document-contract.rs").read_text())

    text = main.read_text()
    text = text.replace((program_root / "🦀️.rs").as_posix(), "page_content.rs")
    start = text.index("pub mod xobjects {")
    end = text.index('#[path = "object.rs"]', start)
    text = text[:start] + '#[path = "xobjects.rs"] pub mod xobjects;\n#[path = "images.rs"] pub mod images;\n' + text[end:]
    text = text.replace("pub struct PdfColorSpace;", "pub enum PdfColorSpace { DeviceGray, DeviceRgb, DeviceCmyk }")
    text = text.replace("xobjects::PdfImageBody, document_metadata::PdfDate", "xobjects::*, colour_and_graphics_state::PdfColorSpace, document_stage::PdfSnapshot, document_metadata::PdfDate")
    text = text.replace("        pub mod graph_source {", "        pub mod stream_roles { pub use crate::document_stage::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue}; }\n        pub mod graph_source {", 1)
    text = text.replace("        pub struct NativePdfArtifactResources;", "        #[derive(Default)] pub struct NativePdfArtifactResources;")
    start = text.index("        pub mod images {")
    end = text.index("        pub mod fonts {", start)
    sample = IMAGE_NATIVE["HARNESS"]
    stubs = sample[sample.index("        pub mod xref {"):sample.index("\n    }\n} } } }")]
    stubs = stubs.replace("        pub mod colour {", "        pub mod image_colour {")
    colour_start = stubs.index("        pub mod image_colour {")
    colour_body = stubs[stubs.index("\n", colour_start) : stubs.rindex("\n        }")]
    stubs = stubs[:colour_start]
    colour_body = colour_body.replace("            use crate::{PdfDictEntry, PdfObject};", "            use crate::PdfDictEntry;").replace("            use crate::colour_and_graphics_state::PdfColorSpace;\n", "").replace("            use super::super::{io::error::PResult, schema::graph_source::ObjectSource};", "            use super::super::schema::graph_source::ObjectSource;")
    colour_position = text.index("        pub mod filters {")
    previous = text.rindex("        }", 0, colour_position)
    text = text[:start] + "        pub use crate::images;\n" + stubs + text[end:]
    text = text[:previous] + colour_body + "\n" + text[previous:]
    main.write_text(text)


def output_source(name, fallback):
    staged = OUTPUT / name
    return staged if staged.exists() and os.environ.get("PDF_CONTROLLED_LIVE") != "1" else fallback


def exercise(base):
    runner = TICKET / "📜️content-program-native.py"
    source = runner.read_text()
    source = source.replace("        binary = scratch /", "        patch_harness(scratch, source, base)\n        binary = scratch /")
    variables = {"__file__": str(runner), "patch_harness": patch_harness}
    exec(compile(source, str(runner), "exec"), variables)
    lift = base / "🔨️modules/⬇️lift/🧩️lifter-and-document"
    os.environ["PDF_CONTROLLED_IMAGE_FIXTURE"] = str(OUTPUT / "🎞️controlled-images.pdf")
    variables["exercise"](base / "🔨️modules/🖋️content", OUTPUT, lift / "📜️page-content", lift / "🧭️document-lift/🦀️.rs")


if __name__ == "__main__":
    exercise(Path(sys.argv[1]).resolve())
