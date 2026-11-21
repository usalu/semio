"""🖋️ Tests PDF decoding and outlining of an embedded raw CID-keyed font."""

from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
probe = ticket / "📜️cff-file-codec-probe.py"
staged = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else None

check = r'''
    use crate::{ArtifactRef, PdfObject, font_model::{PdfCMap, PdfCidFont, PdfFont, PdfFontKind, PdfFontProgram, PdfToUnicode, PdfToUnicodeMapping}, fonts::{FontCodec, truetype::TrueTypeOutlineSegment}, foreign_artifacts::PdfArtifactResourcePort, xref::{Resolver, build_xref}};
    use crate::standards::v1_7::subsets::base::io::error::PResult;
    struct AdmittedFont(PdfObject);
    impl PdfArtifactResourcePort for AdmittedFont {
        fn admit(&mut self, _: &str, _: PdfObject) -> PResult<ArtifactRef> { unreachable!() }
        fn resolve(&self, _: &ArtifactRef) -> PResult<PdfObject> { Ok(self.0.clone()) }
    }

    let pdf = include_bytes!("@CFF_PDF@");
    let mut source = Resolver::new(pdf, build_xref(pdf).unwrap().entries);
    let admitted = AdmittedFont(source.resolve(7).unwrap());
    let text = "中文語日";
    let bytes = include_bytes!("@CIDS@");
    let cids: Vec<_> = bytes.chunks_exact(2).map(|pair| u32::from(u16::from_be_bytes(pair.try_into().unwrap()))).collect();
    let mappings = cids.iter().copied().zip(text.chars()).map(|(code, ch)| PdfToUnicodeMapping::Char { code, text: ch.to_string() }).collect();
    let font = PdfFont {
        id: "F1".into(), to_unicode: Some(PdfToUnicode { byte_width: 2, mappings }), extra: Vec::new(),
        kind: PdfFontKind::Type0 {
            base_font: "NotoSansCJKsc-Regular-Identity-H".into(),
            cmap: PdfCMap::Predefined { name: "Identity-H".into() },
            descendant: PdfCidFont {
                true_type: false, base_font: "NotoSansCJKsc-Regular".into(), system_info: Default::default(),
                descriptor: Default::default(), default_width: 1000.0, widths: Vec::new(), default_vertical: None,
                vertical_metrics: Vec::new(), cid_to_gid: None, program: Some(PdfFontProgram::CidCff { reference: ArtifactRef }), extra: Vec::new(),
            },
        },
    };
    let codec = FontCodec::new(&font, &admitted).unwrap();
    assert_eq!(codec.decode_text(bytes).as_deref(), Some(text));
    assert_eq!(codec.encode(text).as_deref(), Some(bytes.as_slice()));
    let gid_bytes = include_bytes!("@GIDS@");
    let gids = gid_bytes.chunks_exact(2).map(|pair| u16::from_be_bytes(pair.try_into().unwrap()));
    let mut reference = include_str!("@OUTLINE@").lines();
    for ((glyph, cid), expected) in codec.decode(bytes).into_iter().zip(cids).zip(gids) {
        let id = glyph.glyph_id.unwrap();
        assert_eq!((glyph.cid, glyph.code, glyph.width), (cid, cid, 1000.0));
        assert_eq!(id, expected, "glyph for CID {cid}");
        let path = codec.glyph_outline(id).unwrap();
        assert!(path.len() > 15);
        assert!(matches!(path[0], TrueTypeOutlineSegment::MoveTo([x, y]) if x.is_finite() && y.is_finite()));
        for segment in path {
            use TrueTypeOutlineSegment::*;
            let (name, points) = match segment {
                MoveTo(to) => ("moveTo", vec![to]),
                LineTo(to) => ("lineTo", vec![to]),
                QuadraticTo { control, to } => ("qCurveTo", vec![control, to]),
                CubicTo { first, second, to } => ("curveTo", vec![first, second, to]),
                Close => ("closePath", Vec::new()),
            };
            let mut row = reference.next().unwrap().split_ascii_whitespace();
            assert_eq!(row.next().unwrap().parse::<u16>().unwrap(), id);
            assert_eq!(row.next(), Some(name));
            for value in points.into_iter().flatten() {
                let expected: f64 = row.next().unwrap().parse().unwrap();
                assert!((value - expected).abs() <= 1.0e-8, "{value} != {expected}");
            }
            assert!(row.next().is_none());
        }
    }
    assert!(reference.next().is_none());
    println!("[DEBUG] Raw CID-keyed PDF font mapped four sparse CIDs to distinct glyph IDs and rendered every outline");
}
'''.replace("@CIDS@", (ticket / "🗑️generated/raw-cff-cids.bin").as_posix()).replace("@GIDS@", (ticket / "🗑️generated/raw-cff-gids.bin").as_posix()).replace("@OUTLINE@", (ticket / "🗑️generated/raw-cff-outline.txt").as_posix())

runpy.run_path(str(probe))["exercise"](sys.argv[1], "raw-cff.pdf", check, staged)
