"""🧪 Exercise exact production controlled TrueType code and the existing PDF raw writer."""

import json
from pathlib import Path
import runpy
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "font-cid-truetype"
FONTS = "🔨️modules/🔤️fonts"
FONT = FONTS + "/🎚️controlled-opentype/🦀️.rs"
LIFT = "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs"


def extend(base, baseline):
    """🔗 Compose both embedded font profiles in one focused source harness."""
    opener = runpy.run_path(str(TICKET / "controlled-opentype-test.py"))
    exact = runpy.run_path(str(TICKET / "📜️controlled-cid-truetype-stage.py"))["once"]
    previous = json.loads((GENERATED / "font-opentype" / "stages.json").read_text(encoding="utf8"))
    change = json.loads((OUT / "stage.json").read_text(encoding="utf8")) if not baseline else {}
    fixtures = json.loads((OUT / "expect.json").read_text(encoding="utf8"))["fixtures"]
    old = (TICKET / "controlled-opentype-contract.rs").read_text(encoding="utf8")
    case = json.loads((TICKET / "🔣️controlled-cid-truetype.json").read_text(encoding="utf8"))
    body = old.replace("run_embedded_opentype", "run_embedded_cid_truetype").replace("OpenType", "CID TrueType")
    adjustments = [
        ("id.num != 7 && id.num != 8", "id.num != 8 && id.num != 9"),
        ("if id.num == 7", "if id.num == 8"),
        ("let PdfFontKind::Type1 { program: Some(PdfFontProgram::CID TrueType { reference }), widths, .. } = &font.kind else { unreachable!() };",
         '''let PdfFontKind::Type0 { cmap: crate::font_model::PdfCMap::Predefined { name }, descendant, .. } = &font.kind else { unreachable!() };
                    assert_eq!(name, "Identity-H");
                    assert!(descendant.true_type);
                    assert_eq!(descendant.default_width, 1000.);
                    assert_eq!(descendant.cid_to_gid, Some(crate::font_model::PdfCidToGid::Identity));
                    let [widths] = descendant.widths.as_slice() else { unreachable!() };
                    assert_eq!(widths.start_cid, 1);
                    let Some(PdfFontProgram::TrueType { reference }) = &descendant.program else { unreachable!() };'''),
        ("s.stdio.font.opentype", "s.stdio.font.truetype"),
        ("assert_eq!(widths, &vec![500.; 169]);", f"assert_eq!(widths.widths, {list(float(g['width']) for g in case['glyphs'])!r});"),
        ("program: PdfFontProgram::CID TrueType", "program: PdfFontProgram::TrueType"),
        ("owner.num), (8, 7)", "owner.num), (9, 8)"),
        ("                        assert!(role.dependencies.iter().any(|identity| identity.owner.num == 8));", '''                        assert!(role.dependencies.iter().any(|identity| identity.owner.num == 8));
                        assert!(role.dependencies.iter().any(|identity| identity.owner.num == 9));'''),
        ("&[65, 103, 82, 233]", "&[1, 2, 3, 4]"),
        ("vec![65, 103]", "vec![1, 2]"),
        ("vec![82, 233]", "vec![3, 4]"),
        ("entry.id.num == 7", "entry.id.num == 8"),
        ("decode_text(&[1, 2, 3, 4])", "decode_text(&[0, 1, 0, 2, 0, 3, 0, 4])"),
    ]
    for before, after in adjustments:
        if before == "&[65, 103, 82, 233]":
            assert body.count(before) == 2
            body = body.replace(before, after)
        else:
            body = exact(body, before, after)
    if "--trace" in sys.argv:
        body = exact(body, "            if id.num == 8 { self.program_reads += 1; }", '''            if id.num == 8 { self.program_reads += 1; }
            let found = self.objects.iter().find(|object| object.id == id);
            if id.num <= 7 && self.program_reads < 1 {
                eprintln!("[DEBUG] CID operand {} {:?}", id.num, found.map(|object| &object.value));
            }
            if id.num == 8 && self.program_reads == 1 {
                if let Some(PdfObject::Stream { dict, data, filters }) = found.map(|object| &object.value) {
                    eprintln!("[DEBUG] CID stream {dict:?}, {} bytes, prefix {:?}, filters {filters:?}", data.len(), data.get(..4));
                }
            }''')
    body = body.replace("@ASSETS@", OUT.as_posix()).replace("@RAW@", fixtures[0]["identity"]).replace("@FLATE@", fixtures[1]["identity"])

    def patch(scratch, entry, _):
        """🪄 Mount only staged source and add the exact same graph/writer checks."""
        source = entry.read_text(encoding="utf8")
        source = exact(source, "    document_stage::run_embedded_opentype();", "    document_stage::run_embedded_opentype();\n    document_stage::run_embedded_cid_truetype();")
        if not baseline:
            prior_tests = (TICKET / "controlled-opentype-owner-test.rs").read_text(encoding="utf8").replace("#[test]", "")
            previous_fixture = (GENERATED / "font-opentype" / "simple-opentype.txt").as_posix()
            prior_tests = prior_tests.replace("📐️simple-opentype.txt", previous_fixture)
            owner_tests = (TICKET / "📜️controlled-font-owner-tests.rs").read_text(encoding="utf8").replace("#[test]", "")
            owner_tests = owner_tests.replace("📐️simple-opentype.txt", previous_fixture).replace("📐️cid-truetype.txt", (OUT / "cid-truetype.txt").as_posix())
            source = exact(source, prior_tests, owner_tests).replace("PdfSimpleOpenTypeCursor", "PdfEmbeddedFontCursor")
            source = exact(source, "    document_stage::run_embedded_cid_truetype();", "    borrowed_identity_cid_truetype_uses_the_original_program_and_codes();\n    document_stage::run_embedded_cid_truetype();")
            codec = scratch / "font-codec.rs"
            fonts = exact(codec.read_text(encoding="utf8"), previous[FONT], change[FONT]["staged"])
            codec.write_text(fonts.replace("PdfSimpleOpenTypeCursor", "PdfEmbeddedFontCursor").replace("AdmittedSimpleOpenType", "AdmittedEmbeddedFont"), encoding="utf8")
            document = scratch / "document_lift.rs"
            document.write_text(exact(document.read_text(encoding="utf8"), previous[LIFT], change[LIFT]["staged"]), encoding="utf8")
        entry.write_text(source, encoding="utf8")
        contracts = scratch / "document.rs"
        contracts.write_text(contracts.read_text(encoding="utf8") + body, encoding="utf8")

    opener["exercise"](base, patch)


if __name__ == "__main__":
    sys.dont_write_bytecode = True
    extend(Path(sys.argv[1]).resolve(), "--baseline" in sys.argv[2:])
