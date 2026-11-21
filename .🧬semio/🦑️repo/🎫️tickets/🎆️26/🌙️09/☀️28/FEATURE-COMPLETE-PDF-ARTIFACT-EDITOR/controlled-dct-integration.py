"""🔗️ Composes production byte graph and semantic lift against the same admitted DCT PDF."""

import importlib.util
import json
import os
from pathlib import Path
import re
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"


def sibling(file, name):
    spec = importlib.util.spec_from_file_location(name, TICKET / file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def exercise(base, patch_extra=None):
    graph = sibling("🎞️file-graph-tests.py", "file_graph")
    semantic = sibling("🎞️document-tests.py", "document")
    sources = {name: Path(path).resolve() for name, path in json.loads((OUTPUT / "dct-stage" / "stages.json").read_text()).items()}
    staged_image = sources["🔨️modules/🖼️images/🎞️controlled/🦀️.rs"]
    foreign = sources["🚪️io/📦️foreign-artifacts/🦀️.rs"].read_text()
    identity = foreign[foreign.index("const DCT_KIND:"):foreign.index("/// 🛂️ Native admission")]
    decoded = sources["🔨️modules/🗜️filters/🎚️decoded-stream/🦀️.rs"].read_text()
    assets = OUTPUT / "dct-assets"
    expectation = json.loads((assets / "expect.json").read_text())
    expected = [image["identity"] for image in expectation["images"]]
    old = semantic.patch_harness
    source_for = semantic.output_source
    semantic.output_source = lambda name, fallback: staged_image if name == "🎞️image-cursor.rs" else source_for(name, fallback)
    os.environ["PDF_CONTROLLED_LIVE"] = "1"
    os.environ["PDF_DCT_EMITTED"] = str(assets / "emitted.pdf")
    os.environ["PDF_CONTROLLED_IMAGE_FIXTURE"] = str(OUTPUT / "🎞️controlled-images.pdf")
    os.environ["PDF_CONTROLLED_TEXT_FIXTURE"] = str(OUTPUT / "large-mounted-pdf.pdf")
    os.environ["PDF_CONTROLLED_TEXT_STREAMS"] = str(OUTPUT / "type1-streams-current")

    def patch_harness(scratch, main, base):
        old(scratch, main, base)
        text = main.read_text()
        text = text[:text.index("extern crate self as semio_s_artifact_stdio_pdf_xref_stream;")]
        start = text.index("        pub mod filters {")
        end = text.index("        pub use crate::images;", start)
        text = text[:start] + "        pub use crate::{filters, xref};\n" + text[end:]
        start = text.index("        pub mod xref {")
        end = text.index("        pub mod fonts {", start)
        text = text[:start] + """        pub mod encryption {
            pub struct Decryptor;
            impl Decryptor { pub fn decrypt_object(&self, value: crate::PdfObject, _: u32, _: u16) -> crate::PdfObject { value } }
        }
""" + text[end:]
        text = text.replace("pub struct ArtifactRef;", """pub struct ArtifactRef { pub artifact_id: String, pub dialect: ArtifactDialect }
#[derive(Clone, Debug, PartialEq, Eq)] pub struct ArtifactDialect { pub artifact_kind: String, pub standard: String, pub subset: String }""", 1)
        text = text.replace("pub mod foreign_artifacts {", "pub mod foreign_artifacts {\nuse crate::ArtifactDialect;\n"+identity, 1)
        text = text.replace("pub mod standards {", """pub mod standards {
    pub mod v_rfc1950 { pub mod subsets { pub mod any { pub mod io {
        pub fn zlib_compress_deterministic(_: &[u8]) -> Result<Vec<u8>, ()> { unreachable!() }
    } } } }
""", 1)
        text = text.replace('Unsupported(String) }', 'Unsupported(String), NotPdf }', 1)
        text = text.replace("pub mod snapshot {", "pub mod snapshot { pub use crate::PdfEncryption;", 1)
        text += "\n#[derive(Clone, Debug, PartialEq, Eq)] pub struct PdfEncryption;\n"
        text = text.replace("pub struct Decryptor;", """pub struct Decryptor;
            pub fn seal_standard_security(_: &crate::PdfEncryption, _: &[u8], _: &[u8]) -> (Decryptor, Vec<crate::PdfDictEntry>) { unreachable!() }
            impl Decryptor { pub fn encrypt_object(&self, _: crate::PdfObject, _: u32, _: u16) -> crate::PdfObject { unreachable!() } }
""", 1)
        text = re.sub(r"pub fn lower_colour_space\([^\n]+\n", """pub fn lower_colour_space(space: &PdfColorSpace, _: &mut dyn ObjectSink) -> PResult<PdfObject> {
                Ok(PdfObject::name(match space { PdfColorSpace::DeviceGray => "DeviceGray", PdfColorSpace::DeviceRgb => "DeviceRGB", _ => unreachable!() }))
            }
""", text, count=1)
        text = text.replace("pub fn push_opt(_: &mut Vec<PdfDictEntry>, _: &str, _: Option<PdfObject>) { unreachable!() }",
                            "pub fn push_opt(dict: &mut Vec<PdfDictEntry>, key: &str, value: Option<PdfObject>) { if let Some(value) = value { dict.push(PdfDictEntry::new(key, value)); } }")
        text = text.replace("pub fn raw_stream(_: Vec<PdfDictEntry>, _: Vec<u8>) -> PdfObject { unreachable!() }",
                            "pub fn raw_stream(dict: Vec<PdfDictEntry>, data: Vec<u8>) -> PdfObject { PdfObject::Stream { dict, data, filters: Vec::new() } }")
        mounts = dict(
            filters=(base / "🔨️modules/🗜️filters/🦀️.rs"),
            filters_native=(base / "🔨️modules/🗜️filters/📦️packages/🦀️rust/🦀️.rs"),
            xref=(base / "🔨️modules/🔗️xref/🦀️.rs"),
            xref_owner=(base / "🔨️modules/🔗️xref/💧️stream/📦️packages/🦀️rust/🦀️.rs"),
            controlled_graph=(base / "🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs"),
            hash=(Path("🧰️framework/🔨️modules/🔏️hash/🦀️.rs").resolve()),
            writer=(base / "🔨️modules/📝️writer/🦀️.rs"),
        )
        extra = """
extern crate self as semio_framework_hash;
extern crate self as semio_s_artifact_stdio_pdf_filters;
extern crate self as semio_s_artifact_stdio_deflate;
extern crate self as semio_s_artifact_stdio_pdf_xref_stream;
extern crate self as semio_s_artifact_stdio_pdf_lexer;
extern crate self as semio_s_artifact_stdio_pdf_inferences;
pub use hash::Sha256;
pub use filters_native::*;
pub use xref_owner::*;
pub use actual_lexer::*;
"""
        for name, path in mounts.items():
            extra += f'#[path = "{path.as_posix()}"] pub mod {name};\n'
        extra = graph.stage_paths(base, scratch, extra)
        (scratch / "decoded-stream.rs").write_text(decoded)
        legacy_graph = graph.TEST.replace("fn main()", "fn legacy_graph_pipeline()", 1)
        legacy_graph = legacy_graph.replace("crate::lexer::", "crate::actual_lexer::").replace("crate::model::", "crate::object_model::")
        legacy_graph = legacy_graph.replace("@PDF@", (OUTPUT / "🎞️controlled-images.pdf").as_posix())
        legacy_graph = legacy_graph.replace("@PACKED@", OUTPUT.as_posix()).replace("@TEXT@", (OUTPUT / "large-mounted-pdf.pdf").as_posix())
        main.write_text(text + extra + legacy_graph + '''
fn main() {
    document_stage::run_coded_image_pipeline();
    legacy_graph_pipeline();
    document_stage::genuine_pdf_resolves_shared_images_in_order_and_retires_cancelled_samples();
    document_stage::genuine_classic_xref_text_pages_preserve_standard_font_operators_and_roles();
}
''')
        document = scratch / "document.rs"
        contract = (TICKET / "controlled-dct-integration.rs").read_text().replace("@ASSETS@", assets.as_posix())
        contract = contract.replace("@RGB@", expected[0]).replace("@GRAY@", expected[1])
        graph_body = (TICKET / "controlled-dct-graph.rs").read_text()
        common_graph = graph_body[graph_body.index("    const COPY"):graph_body.index("    let inputs")]
        old_cases = document.read_text()
        for name in ("genuine_pdf_resolves_shared_images_in_order_and_retires_cancelled_samples",
                     "genuine_classic_xref_text_pages_preserve_standard_font_operators_and_roles"):
            old_cases = old_cases.replace(f"#[test]\nfn {name}()", f"pub fn {name}()")
        text_start = old_cases.index("pub fn genuine_classic_xref_text_pages_preserve_standard_font_operators_and_roles()")
        text_end = old_cases.index("\n}\n", text_start)
        text_case = old_cases[text_start:text_end]
        grant = "    let grant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };\n"
        text_case = text_case.replace(grant, "")
        turn = '        let turn = cursor.advance(&trailer, "1.7",'
        quote = """        let empty = cursor.reserving_empty_slots();
        let body = cursor.next_inline_body_bytes().max(128);
        let grant = CosGrant { work: 1, allocation_bytes: if empty { 1 << 20 } else { 32_768 - body }, release_bytes: 1 << 20 };
"""
        text_case = text_case.replace(turn, quote + turn)
        old_cases = old_cases[:text_start] + text_case + old_cases[text_end:]
        document.write_text(old_cases + contract.replace("@GRAPH@", common_graph))
        if patch_extra is not None:
            patch_extra(scratch, main, base)

    runner = TICKET / "📜️content-program-native.py"
    source = runner.read_text()
    source = source.replace('        binary = scratch /', '        patch_harness(scratch, source, base)\n        binary = scratch /')
    source = source.replace('"--test", str(source)', 'str(source)')
    source = source[:source.index('        subprocess.run([str(binary), "contract::"')]
    source += '        subprocess.run([str(binary)], check=True)\n'
    env = {"__file__": str(runner), "patch_harness": patch_harness}
    exec(compile(source, str(runner), "exec"), env)
    lift = base / "🔨️modules/⬇️lift/🧩️lifter-and-document"
    env["exercise"](base / "🔨️modules/🖋️content", OUTPUT, lift / "📜️page-content", lift / "🧭️document-lift/🦀️.rs")


if __name__ == "__main__":
    sys.dont_write_bytecode = True
    exercise(Path(sys.argv[1]).resolve())
