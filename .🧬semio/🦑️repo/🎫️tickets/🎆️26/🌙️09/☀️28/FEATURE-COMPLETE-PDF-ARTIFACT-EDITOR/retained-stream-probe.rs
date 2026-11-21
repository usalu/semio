    use crate::{
        content::{parse_content, NoFonts},
        controlled_graph::{PdfByteGraph, PdfByteGraphCursor, PdfByteGraphEvent},
        lexer::{dict_get, CosGrant},
        content_operators::PdfOp,
        foreign_artifacts::PdfArtifactResourcePort,
        model::PdfStreamFilter,
        standards::v1_7::subsets::base::{io::error::{PResult, PdfEngineError}, schema::graph_source::ObjectSource},
        xobjects::PdfImageBody,
        xref::{build_xref, normalize_pdf_object, Resolver},
    };
    @COLD_READER@
    const FIXED: bool = @FIXED@;
    for (name, pdf, transform) in [
        ("mirrored JFIF", include_bytes!("@INPUT@/jfif.pdf").as_slice(), None),
        ("Adobe RGB", include_bytes!("@INPUT@/adobe.pdf").as_slice(), Some(1)),
    ] {
        let id = ObjRef { num: 5, gen: 0 };
        let mut resolver = Resolver::new(pdf, build_xref(pdf).unwrap().entries);
        let encoded = resolver.resolve_reference(id).unwrap();
        let PdfObject::Stream { filters, .. } = &encoded else { unreachable!() };
        assert!(filters.is_empty() && encoded.dict_get("Filter").is_some());
        let original = ObjectSource::get(&mut resolver, id).unwrap();
        let PdfObject::Stream { dict, data, filters } = &original else { panic!("{name}: expected image stream") };
        assert_eq!(original.dict_get("Subtype").and_then(PdfObject::as_name), Some("Image"));
        assert!(data.starts_with(&[0xff, 0xd8]) && data.ends_with(&[0xff, 0xd9]));
        assert!(!dict.iter().any(|entry| matches!(entry.key.as_str(), "Filter" | "Length" | "DecodeParms")));
        assert_eq!(*filters, vec![PdfStreamFilter::Dct { color_transform: transform }]);
        assert_eq!(normalize_pdf_object(original.clone()).unwrap() == original, FIXED);
        let clear = read_clear_pdf_graph(pdf).unwrap().unwrap();
        let cold = clear.graph.objects.iter().find(|obj| obj.id == id).expect("cold image object");
        assert_eq!(cold.value == original, FIXED);
        assert_eq!(cold.value.dict_get("Filter").is_none(), FIXED);
        let retained = resolver.resolve_all(None).unwrap().objects.into_iter()
            .find(|object| object.id == id).expect("retained image object");
        assert_eq!(retained.value, original);
        println!("[DEBUG] {name}: cold DCT provenance and repeated normalization preserved = {FIXED}");
    }

    for (name, pdf) in [
        ("non-image reference stream", include_bytes!("@INPUT@/not-image.pdf").as_slice()),
        ("unknown image codec", include_bytes!("@INPUT@/unknown.pdf").as_slice()),
    ] {
        let id = ObjRef { num: 5, gen: 0 };
        let mut resolver = Resolver::new(pdf, build_xref(pdf).unwrap().entries);
        let encoded = resolver.resolve_reference(id).unwrap();
        let clear = read_clear_pdf_graph(pdf).unwrap().unwrap();
        let preserved = &clear.graph.objects.iter().find(|obj| obj.id == id).unwrap().value;
        let PdfObject::Stream { data, filters, .. } = preserved else { unreachable!() };
        let PdfObject::Stream { data: raw_data, .. } = &encoded else { unreachable!() };
        assert_eq!(data, raw_data, "{name}");
        assert!(filters.is_empty());
        assert_eq!(preserved.dict_get("Filter"), encoded.dict_get("Filter"));
        println!("[DEBUG] {name} kept original bytes and raw filter dictionary");
    }

    let raw = PdfObject::Stream {
        dict: vec![PdfDictEntry::new("Filter", PdfObject::name("FlateDecode"))],
        data: include_bytes!("@INPUT@/encoded").to_vec(),
        filters: Vec::new(),
    };
    let normalized = normalize_pdf_object(raw.clone()).unwrap();
    let PdfObject::Stream { dict, data, filters } = &normalized else { unreachable!() };
    assert!(dict.is_empty());
    assert_eq!(data, b"cold flate stream");
    assert_eq!(*filters, vec![PdfStreamFilter::Flate { predictor: None }]);
    assert_eq!(normalize_pdf_object(normalized.clone()).unwrap() == normalized, FIXED);
    let nested = PdfObject::Stream {
        dict: vec![PdfDictEntry::new("Nested", raw)],
        data: data.clone(),
        filters: filters.clone(),
    };
    let normalized_nested = normalize_pdf_object(nested).unwrap();
    assert_eq!(normalized_nested.dict_get("Nested"), Some(&normalized));
    assert_eq!(normalize_pdf_object(normalized_nested.clone()).unwrap() == normalized_nested, FIXED);
    println!("[DEBUG] raw Flate decoded once, repeated normalization preserved pipeline = {FIXED}");

    let raw_predicted = PdfObject::Stream {
        dict: vec![
            PdfDictEntry::new("Filter", PdfObject::name("FlateDecode")),
            PdfDictEntry::new("DecodeParms", PdfObject::Dict(vec![
                PdfDictEntry::new("Predictor", PdfObject::Int(12)),
                PdfDictEntry::new("Columns", PdfObject::Int(4)),
                PdfDictEntry::new("Colors", PdfObject::Int(1)),
                PdfDictEntry::new("BitsPerComponent", PdfObject::Int(8)),
            ])),
        ],
        data: include_bytes!("@INPUT@/predictor").to_vec(),
        filters: Vec::new(),
    };
    let predicted = normalize_pdf_object(raw_predicted).unwrap();
    let PdfObject::Stream { dict, data, filters } = &predicted else { unreachable!() };
    assert!(dict.is_empty() && matches!(filters.as_slice(), [PdfStreamFilter::Flate { predictor: Some(_) }]));
    assert_eq!(data, &[2, 4, 6, 8, 3, 7, 11, 15]);
    assert_eq!(normalize_pdf_object(predicted.clone()).unwrap() == predicted, FIXED);
    println!("[DEBUG] Flate predictor decoded once, DecodeParms provenance preserved = {FIXED}");

    #[derive(Default)]
    struct ImageResources(Vec<PdfObject>);
    impl PdfArtifactResourcePort for ImageResources {
        fn admit(&mut self, kind: &str, value: PdfObject) -> PResult<ArtifactRef> {
            assert_eq!(kind, "s.stdio.jpeg");
            self.0.push(value);
            Ok(ArtifactRef)
        }
    }
    let pdf = include_bytes!("@INPUT@/inline.pdf");
    let mut resolver = Resolver::new(pdf, build_xref(pdf).unwrap().entries);
    let content = ObjectSource::get(&mut resolver, ObjRef { num: 4, gen: 0 }).unwrap();
    let PdfObject::Stream { data, .. } = content else { unreachable!() };
    let mut resources = ImageResources::default();
    let ops = parse_content(&data, &NoFonts, &mut resources).unwrap();
    let image = ops.iter().find_map(|op| match op { PdfOp::InlineImage { image } => Some(image), _ => None }).unwrap();
    assert!(matches!(image.body, PdfImageBody::Artifact { .. }));
    assert_eq!((image.width, image.height, image.bits_per_component), (16, 8, 8));
    assert_eq!(image.decode, vec![1.0, 0.0]);
    let [PdfObject::Stream { data, filters, .. }] = resources.0.as_slice() else { unreachable!() };
    assert_eq!(data, include_bytes!("@INPUT@/gray.jpg"));
    assert_eq!(filters, &[PdfStreamFilter::AsciiHex, PdfStreamFilter::Dct { color_transform: None }]);
    println!("[DEBUG] inline ASCIIHex wrapper removed exactly once and JPEG/Decode retained");
}
