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
