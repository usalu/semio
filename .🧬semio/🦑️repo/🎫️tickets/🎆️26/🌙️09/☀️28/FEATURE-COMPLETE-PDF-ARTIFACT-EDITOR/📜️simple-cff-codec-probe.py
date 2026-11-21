"""📝️ Checks native raw Type1C names, built-in encoding and PDF Differences."""

from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
probe = ticket / "📜️cff-file-codec-probe.py"
stage = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else None
exercise = runpy.run_path(str(probe))["exercise"]
base = sys.argv[1]
body = r'''
    use crate::{ArtifactRef, PdfObject, font_model::{PdfBaseEncoding, PdfEncodingDifference, PdfFont, PdfFontKind, PdfFontProgram}, fonts::{FontCodec, truetype::TrueTypeOutlineSegment}, foreign_artifacts::PdfArtifactResourcePort, xref::{Resolver, build_xref}};
    use crate::standards::v1_7::subsets::base::io::error::PResult;
    struct AdmittedFont(PdfObject);
    impl PdfArtifactResourcePort for AdmittedFont {
        fn admit(&mut self, _: &str, _: PdfObject) -> PResult<ArtifactRef> { unreachable!() }
        fn resolve(&self, _: &ArtifactRef) -> PResult<PdfObject> { Ok(self.0.clone()) }
    }

    let input = include_bytes!("@CFF_PDF@");
    let mut resolver = Resolver::new(input, build_xref(input).unwrap().entries);
    let program = AdmittedFont(resolver.resolve(7).unwrap());
    let mut font = PdfFont::standard("F1", "LibertinusSerif-Regular");
    let PdfFontKind::Type1 { encoding, first_char, widths, program: slot, .. } = &mut font.kind else { unreachable!() };
    if @DECLARED@ {
        encoding.base = Some(PdfBaseEncoding::WinAnsi);
        encoding.differences = ["A", "g", "R", "eacute"].into_iter().enumerate().map(|(i, glyph)| PdfEncodingDifference { code: 70 + i as u32, glyph: glyph.into() }).collect();
    } else {
        encoding.base = None;
    }
    *first_char = 70;
    *widths = vec![500.0; 4];
    *slot = Some(PdfFontProgram::Cff { reference: ArtifactRef });
    let codec = FontCodec::new(&font, &program).unwrap();
    let text = "AgRé";
    let bytes = b"FGHI";
    assert_eq!(codec.decode_text(bytes).as_deref(), Some(text));
    let encoded = codec.encode(text).unwrap();
    if !@DECLARED@ { assert_eq!(encoded, bytes); }
    assert_eq!(codec.decode_text(&encoded).as_deref(), Some(text));
    let expected = include_bytes!("@GLYPHS@");
    let glyphs = codec.decode(bytes);
    assert_eq!(glyphs.len(), expected.len());
    for (glyph, &id) in glyphs.into_iter().zip(expected) {
        assert_eq!(glyph.width, 500.0);
        assert_eq!(glyph.glyph_id, Some(u16::from(id)));
        let outline = codec.glyph_outline(u16::from(id)).unwrap();
        assert!(outline.iter().any(|segment| matches!(segment, TrueTypeOutlineSegment::CubicTo { .. })));
    }
    println!("[DEBUG] Raw Type1C PDF codec matched FontTools IDs, four PDF widths and Unicode for @KIND@ encoding");
}
'''

if stage and stage.name == "text-font-edit":
    body = body.replace("    let glyphs = codec.decode(bytes);", '    assert!(codec.shows_text(text));\n    assert!(!codec.shows_text("Q"));\n    let glyphs = codec.decode(bytes);')

for kind in ["declared", "builtin"]:
    check = body.replace("@DECLARED@", str(kind == "declared").lower()).replace("@KIND@", kind)
    check = check.replace("@GLYPHS@", (ticket / f"🗑️generated/raw-simple-{kind}-glyphs.bin").as_posix())
    exercise(base, f"raw-simple-{kind}.pdf", check, stage)
