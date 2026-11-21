#[test]
fn borrowed_simple_opentype_uses_the_original_program_and_codes() {
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources, PdfArtifactResourcePort};
    use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosStatus, Lexer};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfObject, PdfStreamFilter};
    use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleValue;

    let mut fixture = include_str!("📐️simple-opentype.txt").lines();
    assert_eq!(fixture.next(), Some("pdf-simple-opentype-v1"));
    let font_dict = fixture.next().unwrap();
    let descriptor_dict = fixture.next().unwrap();
    let decode = |line: &str| {
        (0..line.len()).step_by(2).map(|i| u8::from_str_radix(&line[i..i + 2], 16).unwrap()).collect::<Vec<_>>()
    };
    let program = decode(fixture.next().unwrap());
    let cmap = decode(fixture.next().unwrap());
    let codes = decode(fixture.next().unwrap());
    let expected = fixture.next().unwrap();
    let program_sha256 = [fixture.next().unwrap(), fixture.next().unwrap()];
    assert!(fixture.next().is_none());
    let owner = |num| ObjRef { num, gen: 0 };
    let stream = |dict, data, filters| PdfObject::Stream { dict, data, filters };
    let parse = |body: &str| Lexer::new(body.as_bytes()).parse_object().unwrap();
    let font_object = parse(font_dict);
    for (index, filters) in [vec![], vec![PdfStreamFilter::Flate { predictor: None }]].into_iter().enumerate() {
        let native = [
            parse(descriptor_dict),
            stream(vec![PdfDictEntry::new("Subtype", PdfObject::name("OpenType"))], program.clone(), filters),
            stream(vec![], cmap.clone(), vec![]),
        ];
        let mut inspect = |id: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)| {
            read(if id == owner(5) { Some(&font_object) } else { id.num.checked_sub(6).and_then(|index| native.get(index as usize)) });
        };
        let mut cursor = PdfSimpleOpenTypeCursor::new(owner(5), &font_object).unwrap();
        let grant = CosGrant { work: 1, allocation_bytes: 32_640, release_bytes: 32_768 };
        let mut admitted = false;
        for _ in 0..32_768 {
            let turn = cursor.advance(&mut inspect, grant);
            assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            match turn.status.unwrap() {
                CosStatus::Complete { .. } => { admitted = true; break; }
                CosStatus::More => {}
                status => panic!("unexpected font cursor status {status:?}"),
            }
        }
        assert!(admitted);
        assert!(cursor.publication_bytes("F1") <= grant.allocation_bytes);
        let admitted = cursor.take("F1").unwrap();
        assert_eq!(admitted.dependencies, [owner(6), owner(8), owner(7)]);
        assert!(matches!(&admitted.roles[0].value, PdfStreamRoleValue::UnicodeMap { mapping }
            if Some(mapping) == admitted.font.to_unicode.as_ref()));
        let PdfStreamRoleValue::FontProgram { program: PdfFontProgram::OpenType { reference } } = &admitted.roles[1].value else { unreachable!() };
        assert_eq!(reference.artifact_id, format!("pdf:s.stdio.font.opentype:sha256:{}", program_sha256[index]));
        let mut resources = NativePdfArtifactResources::default();
        assert_eq!(resources.admit("s.stdio.font.opentype", native[1].clone()).unwrap(), *reference);
        let codec = FontCodec::new(&admitted.font, &resources).unwrap();
        assert_eq!(codec.decode_text(&codes).as_deref(), Some(expected));
        assert_eq!(codec.encode(expected).as_deref(), Some(codes.as_slice()));
        for glyph in codec.decode(&codes) {
            assert!(codec.glyph_outline(glyph.glyph_id.unwrap()).is_some_and(|segments| !segments.is_empty()));
        }
    }
}
