"""🔤️ Compose the staged PDF OpenType ownership path without editing the workspace."""

from pathlib import Path
import difflib
import json
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "font-opentype"


def lift_integration(source):
    first = source.index("        let count = self.snapshot_mut().fonts.len();", source.index("    fn font("))
    last = source.index("\n    fn ", first)
    source = source[:first] + "        let snapshot = self.snapshot.as_mut().expect(\"active PDF font snapshot\");\n        let turn = self.font_resources.admit(&mut |id, read| source.inspect(id, read), snapshot, owner, name, grant);\n        let Some(turn) = turn else { return self.defer(Barrier::Resources); };\n        if let Ok(CosStatus::Complete { end: index }) = turn.status {\n            if resume == Resume::Inherit { self.page_font = Some((resources, index)); }\n            self.resume(resume);\n        }\n        image_turn(turn)\n    }\n" + source[last:]
    start = source.index("fn standard_type1(")
    end = source.index("fn root_identity(", start)
    source = source[:start] + source[end:]
    for old, new, count in (
        ("content::{bounded_number, FontTable}, fonts::{standard_font, FontCodec},", "content::{bounded_number, FontTable, PdfFixedCodeWidth}, fonts::{ControlledFontResources, FontCodec},", 1),
        ("{PdfBaseEncoding, PdfFont, PdfFontKind, PdfSimpleEncoding};", "{PdfBaseEncoding, PdfFont, PdfFontKind, PdfFontProgram};", 1),
        ("const MAX_FONTS: usize = 8;\n", "", 1),
        ("    font_owners: [ObjRef; MAX_FONTS],", "    font_resources: ControlledFontResources,", 1),
        ("font_owners: [ObjRef::default(); MAX_FONTS], page_font: None,", "font_resources: Default::default(), page_font: None,", 1),
        ("            Stage::Annotation(..) => size_of::<PdfAnnotation>(),", "            Stage::Annotation(..) => size_of::<PdfAnnotation>(),\n            Stage::Font { .. } => self.font_resources.inline_body_bytes(),", 1),
        ("        self.images.begin_close();", "        self.images.begin_close();\n        self.font_resources.begin_close();", 1),
        ("        if image.status != Ok(CosStatus::Closed) { return image_turn(image); }", "        if image.status != Ok(CosStatus::Closed) { return image_turn(image); }\n        let font = self.font_resources.close_step(grant);\n        if font.status != Ok(CosStatus::Closed) || font.work != 0 { return image_turn(font); }", 1),
        ("        let roles = count + ControlledImageResources::MAX_COUNT;", "        let roles = count + ControlledImageResources::MAX_COUNT + ControlledFontResources::EXTRA_ROLES;", 1),
        ("self.font_owners", "self.font_resources.owners", 3),
        ("            let capacity = self.owners_len + 1 + count + usize::from(context.is_some());", "            let extra = self.page_font.map_or(&[][..], |(_, index)| self.font_resources.dependencies(index));\n            let capacity = self.owners_len + 1 + count + usize::from(context.is_some()) + extra.len();", 1),
        ("            let content = self.program.as_mut().and_then(PageProgramCursor::take_program).expect(\"ready admitted PDF operators\");", "            dependencies.extend(extra.iter().copied().map(root_identity));\n            let content = self.program.as_mut().and_then(PageProgramCursor::take_program).expect(\"ready admitted PDF operators\");", 1),
        ("let PdfFontKind::Type1 { encoding, .. } = &self.0.kind else { return None; };", "let PdfFontKind::Type1 { encoding, program: None, .. } = &self.0.kind else { return None; };", 1),
        ("        (self.0.id == name).then_some(encoding.base.unwrap_or(PdfBaseEncoding::Standard))\n    }\n}", "        (self.0.id == name).then_some(encoding.base.unwrap_or(PdfBaseEncoding::Standard))\n    }\n\n    fn controlled_code_width(&self, name: &str) -> Option<PdfFixedCodeWidth> {\n        (self.0.id == name && matches!(&self.0.kind, PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { .. }), .. }))\n            .then_some(PdfFixedCodeWidth::One)\n    }\n}", 1),
    ):
        assert source.count(old) == count, (old, source.count(old))
        source = source.replace(old, new)
    return source


def stage(base):
    OUT.mkdir(parents=True, exist_ok=True)
    dct = json.loads((GENERATED / "dct-stage" / "stages.json").read_text())
    fonts = "🔨️modules/🔤️fonts"
    foreign = "🚪️io/📦️foreign-artifacts/🦀️.rs"
    fonts_root = fonts + "/🦀️.rs"
    crate = fonts + "/📦️packages/🦀️rust/🦀️.rs"
    cmap = fonts + "/🗺️cmap/🦀️.rs"
    cmap_cursor = fonts + "/🗺️cmap/🎚️controlled/🦀️.rs"
    new = fonts + "/🎚️controlled-opentype/🦀️.rs"
    layers = {
        foreign: (Path(dct[foreign]).read_text(), lambda s: s.replace(
            "fn artifact_reference(kind:", """/// 📏️ Reports storage for the four native artifact reference strings.
pub const fn artifact_reference_bytes(kind: &str) -> usize {
    "pdf::sha256:".len() + kind.len() + 64 + RAW_KIND.len() + RAW_STANDARD.len() + RAW_SUBSET.len()
}

/// 🪪️ Materializes one PDF foreign artifact reference from a complete native digest.
pub fn artifact_reference(kind:""", 1)),
        fonts_root: ((base / fonts_root).read_text(), lambda s: s.replace("pub mod cmap;", """pub mod cmap;
#[path = "🎚️controlled-opentype/🦀️.rs"]
mod controlled_opentype;
pub use controlled_opentype::{AdmittedSimpleOpenType, ControlledFontResources, PdfSimpleOpenTypeCursor};""", 1)),
        crate: ((base / crate).read_text(), lambda s: s.replace("::base::schema::snapshot;", "::base::schema::{snapshot, stream_roles};", 1)),
        cmap: ((GENERATED / "font-unicode/cmap.rs").read_text(), lambda s: s),
        cmap_cursor: ((GENERATED / "font-unicode/controlled.rs").read_text(), lambda s: s),
        new: ((TICKET / "controlled-opentype.rs").read_text(), lambda s: s),
    }
    tests = "🔨️modules/🔤️fonts/🧪️tests/🔬️unit"
    test = f"{tests}/🦀️.rs"
    fixture = f"{tests}/📐️simple-opentype.txt"
    layers[test] = ((base / test).read_text(), lambda s: s + "\n" + (TICKET / "controlled-opentype-owner-test.rs").read_text())
    layers[fixture] = ((OUT / "simple-opentype.txt").read_text(), lambda s: s)
    lift = "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs"
    layers[lift] = ((base / lift).read_text(), lift_integration)
    codes = json.loads((GENERATED / "font-codes" / "stages.json").read_text())
    for relative, path in codes.items():
        layers[relative] = (Path(path).read_text(), lambda source: source)
    manifest = {}
    patch = []
    for index, (relative, (source, change)) in enumerate(layers.items()):
        before_path = base / relative
        before = before_path.read_text() if before_path.exists() else ""
        result = change(source)
        output = OUT / f"{index}{before_path.suffix}"
        output.write_text(result)
        manifest[relative] = str(output)
        patch.extend(difflib.unified_diff(before.splitlines(True), result.splitlines(True), "a/" + str(before_path) if before_path.exists() else "/dev/null", "b/" + str(before_path)))
    (OUT / "stages.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    (OUT / "embedded-font.patch").write_text("".join(patch))


if __name__ == "__main__":
    stage(Path(sys.argv[1]))
