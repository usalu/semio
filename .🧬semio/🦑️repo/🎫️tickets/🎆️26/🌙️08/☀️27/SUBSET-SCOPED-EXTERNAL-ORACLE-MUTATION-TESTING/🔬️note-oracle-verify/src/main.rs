fn main() {
    verify_dxf();
    verify_svg();
    verify_pdf();
    println!("ALL OK");
}

fn verify_dxf() {
    let text = "0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n0.0\n20\n0.0\n30\n0.0\n11\n10.0\n21\n20.0\n31\n0.0\n0\nENDSEC\n0\nEOF\n";
    let drawing = dxf::Drawing::load(&mut text.as_bytes()).expect("dxf parses minimal ENTITIES section");
    let lines: Vec<_> = drawing
        .entities()
        .filter_map(|e| match &e.specific {
            dxf::entities::EntityType::Line(l) => Some((l.p1.x, l.p1.y, l.p2.x, l.p2.y)),
            _ => None,
        })
        .collect();
    assert_eq!(lines.len(), 1, "expected exactly one LINE entity");
    assert_eq!(lines[0], (0.0, 0.0, 10.0, 20.0));
    println!("[dxf] OK: parsed 1 LINE entity start=(0,0) end=(10,20)");
}

fn verify_svg() {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"><g id=\"layer-0\"><g transform=\"matrix(1,0,0,1,5,10)\"><path d=\"M0,0 L1,1\"/></g></g></svg>";
    let mut reader = Reader::from_str(svg);
    let mut tags = Vec::new();
    let mut transform_attr: Option<String> = None;
    loop {
        match reader.read_event().expect("quick-xml reads well-formed SVG") {
            Event::Start(e) | Event::Empty(e) => {
                let name: String = e.name().as_ref().to_string();
                if name == "g" {
                    for attr in e.attributes().flatten() {
                        let key: String = attr.key.as_ref().to_string();
                        if key == "transform" {
                            transform_attr = Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                }
                tags.push(name);
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(tags, vec!["svg", "g", "g", "path"]);
    assert_eq!(transform_attr.as_deref(), Some("matrix(1,0,0,1,5,10)"));
    println!("[svg] OK: walked {:?}, inner transform={:?}", tags, transform_attr);
}

fn verify_pdf() {
    use lopdf::{dictionary, Document, Object, Stream};
    let mut document = Document::with_version("1.4");
    let pages_id = document.new_object_id();
    let content = lopdf::content::Content { operations: vec![lopdf::content::Operation::new("Tj", vec![Object::string_literal("hello from note")])] };
    let content_id = document.add_object(Stream::new(dictionary! {}, content.encode().expect("encode content stream")));
    let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => vec![0.into(), 0.into(), 200.into(), 100.into()] });
    let pages = dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 };
    document.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    document.trailer.set("Root", catalog_id);
    let mut bytes = Vec::new();
    document.save_to(&mut bytes).expect("lopdf saves the document it just built");

    let reread = Document::load_mem(&bytes).expect("lopdf reads back its own document");
    let pages_map = reread.get_pages();
    assert_eq!(pages_map.len(), 1);
    let (_, page_id) = pages_map.iter().next().unwrap();
    let page_content = reread.get_page_content(*page_id);
    let decoded = lopdf::content::Content::decode(&page_content).expect("decode content stream");
    let texts: Vec<String> = decoded
        .operations
        .iter()
        .filter(|op| op.operator == "Tj")
        .flat_map(|op| op.operands.iter())
        .filter_map(|operand| operand.as_str().ok().map(|b| String::from_utf8_lossy(b).to_string()))
        .collect();
    assert_eq!(texts, vec!["hello from note".to_string()]);
    println!("[pdf] OK: round-tripped Tj text {:?}", texts);
}
