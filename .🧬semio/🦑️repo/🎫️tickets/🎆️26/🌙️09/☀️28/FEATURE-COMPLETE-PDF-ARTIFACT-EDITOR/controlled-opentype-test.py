"""🧪️ Composes the staged OpenType resource with the actual graph and document cursors."""

import importlib.util
import json
from pathlib import Path
import re
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-opentype"


def exercise(base, patch_extra=None):
    spec = importlib.util.spec_from_file_location("integration", TICKET / "controlled-dct-integration.py")
    integration = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(integration)
    manifest = {file: Path(path) for file, path in json.loads((OUT / "stages.json").read_text()).items()}
    expected = json.loads((OUT / "expect.json").read_text())["fixtures"]
    fonts = "🔨️modules/🔤️fonts"
    lift = "🔨️modules/⬇️lift/🧩️lifter-and-document"
    body = (TICKET / "controlled-opentype-contract.rs").read_text()
    body = body.replace("@ASSETS@", OUT.as_posix()).replace("@RAW@", expected[0]["identity"]).replace("@FLATE@", expected[1]["identity"])

    def path(file):
        return (base / file).resolve().as_posix()

    def patch(scratch, main, base):
        source = main.read_text()
        foreign = manifest["🚪️io/📦️foreign-artifacts/🦀️.rs"].read_text()
        artifact = foreign[foreign.index("const DCT_KIND:"):foreign.index("/// 🛂️ Native admission")]
        start = source.index("const DCT_KIND:")
        end = source.index("    use super::error::PResult;", start)
        source = source[:start] + artifact + source[end:]
        original_content = "🔨️modules/🖋️content/🦀️.rs"
        original_program = "🔨️modules/🖋️content/🧵️program/🦀️.rs"
        content = scratch / "staged-content.rs"
        modified = manifest[original_content].read_text().replace("🧵️program/🦀️.rs", manifest[original_program].as_posix())
        content.write_text(modified)
        source = source.replace(path(original_content), content.as_posix())
        source = source.replace("    document_stage::run_coded_image_pipeline();", "    document_stage::run_embedded_opentype();\n    document_stage::run_coded_image_pipeline();")
        guard = foreign[foreign.index("pub fn require_pdf_artifact_kind("):]
        guard = guard.replace("PdfFontProgram", "crate::font_model::PdfFontProgram").replace("reference:&ArtifactRef", "reference:&crate::ArtifactRef").replace("PdfEngineError::", "super::error::PdfEngineError::")
        source = source.replace("pub fn require_font_artifact_kind(_: &crate::font_model::PdfFontProgram) -> PResult<()> { unreachable!() }", guard, 1)
        replacement = """
            #[derive(Default)]
            pub struct NativePdfArtifactResources(Vec<(ArtifactRef, PdfObject)>);
            impl NativePdfArtifactResources { pub fn from_objects(_: &[PdfObject]) -> Self { Self::default() } }
            impl PdfArtifactResourcePort for NativePdfArtifactResources {
                fn admit(&mut self, kind: &str, object: PdfObject) -> PResult<ArtifactRef> {
                    let PdfObject::Stream { data, filters, .. } = &object else { unreachable!() };
                    let mut hash = crate::Sha256::new();
                    hash.update(kind.as_bytes());
                    hash.update(data);
                    hash.update(if filters.is_empty() { b"[]" } else { b"[{\\"kind\\":\\"flate\\",\\"predictor\\":null}]" });
                    let reference = artifact_reference(kind, hash.finalize());
                    self.0.push((reference.clone(), object));
                    Ok(reference)
                }
                fn resolve(&self, reference: &ArtifactRef) -> PResult<PdfObject> {
                    Ok(self.0.iter().find(|(key, _)| key == reference).unwrap().1.clone())
                }
            }
"""
        start = source.rfind("\n", 0, source.index("pub struct NativePdfArtifactResources;"))
        end = source.index("\n", source.index("impl PdfArtifactResourcePort for NativePdfArtifactResources", start))
        source = source[:start] + replacement + source[end:]
        owner_test = (TICKET / "controlled-opentype-owner-test.rs").read_text().replace("#[test]", "")
        owner_test = owner_test.replace("📐️simple-opentype.txt", (OUT / "simple-opentype.txt").as_posix())
        source += "\nuse actual_fonts::{FontCodec, PdfSimpleOpenTypeCursor};\nuse font_model::PdfFontProgram;\n" + owner_test
        source = source.replace("    document_stage::run_embedded_opentype();", "    borrowed_simple_opentype_uses_the_original_program_and_codes();\n    document_stage::run_embedded_opentype();")
        main.write_text(source)

        page = scratch / "page_content.rs"
        page_original = lift + "/📜️page-content/🧺️program/🦀️.rs"
        page.write_text(page.read_text().replace(path(page_original), manifest[page_original].as_posix()))

        document = scratch / "document_lift.rs"
        doc_original = lift + "/🧭️document-lift/🧭️controlled/🦀️.rs"
        document.write_text(document.read_text().replace(path(doc_original), manifest[doc_original].as_posix()))
        harness = scratch / "document.rs"
        model = harness.read_text().replace("    fonts: Vec<crate::font_model::PdfFont>", "    pub fonts: Vec<crate::font_model::PdfFont>", 1)
        model = model.replace("pub enum PdfStreamRoleValue {", """pub enum PdfStreamRoleValue {
    FontProgram { program: crate::font_model::PdfFontProgram },
    UnicodeMap { mapping: crate::font_model::PdfToUnicode },
""", 1)
        harness.write_text(model + body)

        cmap = scratch / "qualified-cmap.rs"
        cmap_file = fonts + "/🗺️cmap/🦀️.rs"
        cmap_impl = fonts + "/🗺️cmap/🎚️controlled/🦀️.rs"
        cmap.write_text(manifest[cmap_file].read_text().replace("🎚️controlled/🦀️.rs", manifest[cmap_impl].as_posix()))
        font = scratch / "font-codec.rs"
        implementation = "🎚️controlled-opentype/🦀️.rs"
        codec = font.read_text().replace(path(cmap_file), cmap.as_posix())
        live_implementation = path(fonts + "/" + implementation)
        assert codec.count(live_implementation) == 1
        font.write_text(codec.replace(live_implementation, manifest[fonts + "/" + implementation].as_posix()))
        if patch_extra is not None:
            patch_extra(scratch, main, base)

    integration.exercise(base, patch)


if __name__ == "__main__":
    sys.dont_write_bytecode = True
    exercise(Path(sys.argv[1]).resolve())
