#[test]
fn borrowed_simple_opentype_uses_the_original_program_and_codes() {
    verify_embedded_font(include_str!("📐️simple-opentype.txt"), false);
}

#[test]
fn borrowed_identity_cid_truetype_uses_the_original_program_and_codes() {
    verify_embedded_font(include_str!("📐️cid-truetype.txt"), true);
}

fn verify_embedded_font(fixture: &str, composite: bool) {
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources, PdfArtifactResourcePort};
    use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosStatus, Lexer};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfObject, PdfStreamFilter};
    use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleValue;

    let mut fixture = fixture.lines();
    let (title, kind) = if composite {
        ("pdf-cid-truetype-v1", "s.stdio.font.truetype")
    } else {
        ("pdf-simple-opentype-v1", "s.stdio.font.opentype")
    };
    assert_eq!(fixture.next(), Some(title));
    let parse = |body: &str| Lexer::new(body.as_bytes()).parse_object().unwrap();
    let font = parse(fixture.next().unwrap());
    let descendant = composite.then(|| parse(fixture.next().unwrap()));
    let descriptor = parse(fixture.next().unwrap());
    let decode = |line: &str| {
        (0..line.len()).step_by(2).map(|i| u8::from_str_radix(&line[i..i + 2], 16).unwrap()).collect::<Vec<_>>()
    };
    let program = decode(fixture.next().unwrap());
    let cmap = decode(fixture.next().unwrap());
    let codes = decode(fixture.next().unwrap());
    let expected = fixture.next().unwrap();
    let program_sha256 = [fixture.next().unwrap(), fixture.next().unwrap()];
    assert!(fixture.next().is_none());

    let id = |num| ObjRef { num, gen: 0 };
    let stream = |dict, data, filters| PdfObject::Stream { dict, data, filters };
    let descriptor_id = 6 + u32::from(composite);
    let program_id = descriptor_id + 1;
    let cmap_id = descriptor_id + 2;
    let program_key = if composite {
        PdfDictEntry::new("Length1", PdfObject::Int(program.len() as i64))
    } else {
        PdfDictEntry::new("Subtype", PdfObject::name("OpenType"))
    };
    let mut dependencies = vec![id(descriptor_id), id(cmap_id), id(program_id)];
    if composite { dependencies.push(id(6)); }

    for (index, filters) in [vec![], vec![PdfStreamFilter::Flate { predictor: None }]].into_iter().enumerate() {
        let native_program = stream(vec![program_key.clone()], program.clone(), filters);
        let native_cmap = stream(vec![], cmap.clone(), vec![]);
        let mut inspect = |owner: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)| {
            let object = match owner.num {
                5 => Some(&font),
                number if number == descriptor_id => Some(&descriptor),
                number if number == program_id => Some(&native_program),
                number if number == cmap_id => Some(&native_cmap),
                6 => descendant.as_ref(),
                _ => None,
            };
            read(object);
        };
        let mut cursor = PdfEmbeddedFontCursor::new(id(5), &font).unwrap();
        let grant = CosGrant { work: 1, allocation_bytes: 32_640, release_bytes: 32_768 };
        let mut complete = false;
        for _ in 0..32_768 {
            let turn = cursor.advance(&mut inspect, grant);
            assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            match turn.status.unwrap() {
                CosStatus::Complete { .. } => { complete = true; break; }
                CosStatus::More => {}
                status => panic!("unexpected font cursor status {status:?}"),
            }
        }
        assert!(complete && cursor.publication_bytes("F1") <= grant.allocation_bytes);
        let admitted = cursor.take("F1").unwrap();
        assert_eq!(admitted.dependencies(), dependencies);
        assert!(matches!(&admitted.roles[0].value, PdfStreamRoleValue::UnicodeMap { mapping }
            if Some(mapping) == admitted.font.to_unicode.as_ref()));
        let PdfStreamRoleValue::FontProgram { program } = &admitted.roles[1].value else { unreachable!() };
        assert_eq!(matches!(program, PdfFontProgram::TrueType { .. }), composite);
        let reference = program.reference();
        assert_eq!(reference.artifact_id, format!("pdf:{kind}:sha256:{}", program_sha256[index]));
        let mut resources = NativePdfArtifactResources::default();
        assert_eq!(resources.admit(kind, native_program).unwrap(), *reference);
        let codec = FontCodec::new(&admitted.font, &resources).unwrap();
        assert_eq!(codec.decode_text(&codes).as_deref(), Some(expected));
        assert_eq!(codec.encode(expected).as_deref(), Some(codes.as_slice()));
        for glyph in codec.decode(&codes) {
            assert!(codec.glyph_outline(glyph.glyph_id.unwrap()).is_some_and(|segments| !segments.is_empty()));
        }
    }
}
