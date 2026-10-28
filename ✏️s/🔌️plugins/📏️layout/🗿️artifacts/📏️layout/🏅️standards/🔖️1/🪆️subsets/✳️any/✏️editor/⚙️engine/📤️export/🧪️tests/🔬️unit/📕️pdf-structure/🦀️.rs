/// 📕️ The pdf structure oracle shared by the export laws and the editor's download law.

fn find_bytes(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack.get(from..).and_then(|tail| tail.windows(needle.len()).position(|window| window == needle)).map(|at| from + at)
}

fn ascii_number(bytes: &[u8], from: usize) -> usize {
    let digits: String = bytes[from..].iter().take_while(|byte| byte.is_ascii_digit()).map(|byte| *byte as char).collect();
    digits.parse().unwrap_or_else(|_| panic!("ascii number at {from}: {digits:?}"))
}

/// 📕️ Parses the xref table the way a viewer does — on raw bytes, since the font stream is binary:
/// every `N 0 obj` offset must land exactly, every stream's `/Length` (direct or indirect) must equal
/// its byte count, and `startxref` must point at `xref`. Returns the stream count.
fn assert_pdf_structure(bytes: &[u8]) -> usize {
    assert!(bytes.starts_with(b"%PDF-1.4\n"), "pdf header");
    let startxref = find_bytes(bytes, b"startxref\n", 0).expect("startxref");
    let xref_offset = ascii_number(bytes, startxref + 10);
    assert!(bytes[xref_offset..].starts_with(b"xref\n0 "), "startxref lands on the xref table");
    let size = ascii_number(bytes, xref_offset + 7);
    let mut row = find_bytes(bytes, b"\n", xref_offset + 7).expect("xref rows") + 1;
    for id in 0..size {
        let offset = ascii_number(bytes, row);
        if id != 0 {
            let head = format!("{id} 0 obj\n");
            assert!(bytes[offset..].starts_with(head.as_bytes()), "object {id} offset {offset} lands on its header, found {:?}", String::from_utf8_lossy(&bytes[offset..(offset + 12).min(bytes.len())]));
        }
        row += 20;
    }
    let mut cursor = 0;
    let mut streams = 0;
    while let Some(start) = find_bytes(bytes, b">>\nstream\n", cursor) {
        let stream_start = start + 10;
        let dictionary_start = bytes[..start].windows(2).rposition(|window| window == b"<<").expect("stream dictionary");
        let dictionary = String::from_utf8_lossy(&bytes[dictionary_start..start]).to_string();
        let length_text = dictionary.split("/Length ").nth(1).expect("stream length").split_whitespace().take(3).collect::<Vec<_>>();
        let length: usize = if length_text.get(1) == Some(&"0") && length_text.get(2) == Some(&"R") {
            let id: usize = length_text[0].parse().expect("indirect length id");
            let head = format!("{id} 0 obj\n");
            let at = find_bytes(bytes, head.as_bytes(), 0).expect("length object") + head.len();
            ascii_number(bytes, at)
        } else {
            length_text[0].trim_end_matches('/').parse().expect("direct length")
        };
        assert!(bytes[stream_start + length..].starts_with(b"\nendstream"), "stream length {length} ends exactly at endstream");
        streams += 1;
        cursor = stream_start + length;
    }
    streams
}

fn pdf_operations(bytes: &[u8]) -> Vec<lopdf::content::Operation> {
    let document = lopdf::Document::load_mem(bytes).unwrap();
    document.get_pages().values().flat_map(|id| lopdf::content::Content::decode(&document.get_page_content(*id).unwrap()).unwrap().operations).collect()
}
fn pdf_has_operation(bytes:&[u8],operator:&str,expected:&[f64])->bool{
    let mut matrix=[1.0,0.0,0.0,1.0,0.0,0.0];let mut stack=Vec::new();
    for operation in pdf_operations(bytes){let values=operation.operands.iter().map(|v|v.as_float().ok().map(f64::from)).collect::<Option<Vec<_>>>();
        if operation.operator=="q"{stack.push(matrix);}else if operation.operator=="Q"{matrix=stack.pop().unwrap();}else if operation.operator=="cm"{if let Some(b)=values.as_ref().filter(|v|v.len()==6){let a=matrix;matrix=[a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]];}}
        if operation.operator!=operator{continue;}let Some(mut values)=values else{continue};if matches!(operator,"m"|"l"|"c"){for point in values.chunks_exact_mut(2){let(x,y)=(point[0],point[1]);point[0]=matrix[0]*x+matrix[2]*y+matrix[4];point[1]=matrix[1]*x+matrix[3]*y+matrix[5];}}if values.len()==expected.len()&&values.iter().zip(expected).all(|(value,expected)|(value-expected).abs()<0.0001){return true;}
    }false
}
fn pdf_rgb_images(bytes: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
    let document = lopdf::Document::load_mem(bytes).unwrap();
    document.objects.values().filter_map(|object| { let stream = object.as_stream().ok()?; if stream.dict.get(b"Subtype").ok()?.as_name().ok()? != b"Image" || stream.dict.get(b"ColorSpace").ok()?.as_name().ok()? != b"DeviceRGB" { return None; } let mask = document.get_object(stream.dict.get(b"SMask").unwrap().as_reference().unwrap()).unwrap().as_stream().unwrap(); Some((stream.get_plain_content().unwrap(), mask.get_plain_content().unwrap())) }).collect()
}
/// 🔮️ Lopdf's explicit ToUnicode observation avoids its unsupported custom Encoding-stream factory.
fn pdf_extracted_text(bytes: &[u8]) -> String { let mut document = lopdf::Document::load_mem(bytes).unwrap(); for object in document.objects.values_mut() { if let Ok(dict) = object.as_dict_mut() { if dict.has_type(b"Font") && dict.get(b"ToUnicode").is_ok() { dict.remove(b"Encoding"); } } } document.extract_text(&document.get_pages().keys().copied().collect::<Vec<_>>()).unwrap() }
fn pdf_font_size(bytes: &[u8], expected: f64) -> bool { pdf_operations(bytes).iter().any(|op| op.operator == "Tf" && op.operands.last().is_some_and(|value| value.as_float().is_ok_and(|size| (f64::from(size)-expected).abs()<0.0001))) }
fn pdf_has_oblique_text(bytes: &[u8], oblique: bool) -> bool { pdf_operations(bytes).iter().any(|op| op.operator == "Tm" && op.operands.get(2).is_some_and(|value| value.as_float().is_ok_and(|skew| if oblique { skew.abs()>0.2 } else { skew.abs()<0.0001 }))) }
fn pdf_text_compact(bytes: &[u8]) -> String { pdf_extracted_text(bytes).chars().filter(|character| !character.is_whitespace()).collect() }
fn pdf_semantic_styles(bytes: &[u8]) -> Vec<(String, i64, bool)> {
    pdf_operations(bytes).iter().filter(|op| op.operator == "BDC" && op.operands.first().is_some_and(|value| value.as_name().is_ok_and(|tag| tag == b"SemioLayoutText"))).map(|op| {
        let properties = op.operands[1].as_dict().unwrap();
        let family = properties.get(b"SemioFontFamily").unwrap().as_str().unwrap();
        assert_eq!(family.get(..2), Some([0xfe, 0xff].as_slice()));
        let units: Vec<_> = family[2..].chunks_exact(2).map(|unit| u16::from_be_bytes([unit[0], unit[1]])).collect();
        (String::from_utf16(&units).unwrap(), properties.get(b"SemioFontWeight").unwrap().as_i64().unwrap(), properties.get(b"SemioItalic").unwrap().as_bool().unwrap())
    }).collect()
}
