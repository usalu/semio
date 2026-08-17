#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlLexicalDocument { #[serde(default)] pub tokens: Vec<XmlLexicalToken> }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "raw", rename_all = "camelCase")]
pub enum XmlLexicalToken {
    Declaration(String), Doctype(String), Comment(String), ProcessingInstruction(String),
    StartTag(String), EndTag(String), Text(String), CData(String),
}

fn lexical_end(text: &str, start: usize, terminator: &str) -> Result<usize, String> {
    text[start..].find(terminator).map(|offset| start + offset + terminator.len()).ok_or_else(|| format!("unterminated XML lexical token at byte {start}"))
}
fn markup_end(text: &str, start: usize, doctype: bool) -> Result<usize, String> {
    let bytes = text.as_bytes(); let mut pos = start; let mut quote = None; let mut depth = 0usize;
    while pos < bytes.len() { let byte = bytes[pos]; if let Some(active) = quote { if byte == active { quote = None; } } else { match byte { b'\'' | b'"' => quote = Some(byte), b'[' if doctype => depth += 1, b']' if doctype && depth > 0 => depth -= 1, b'>' if depth == 0 => return Ok(pos + 1), _ => {} } } pos += 1; }
    Err(format!("unterminated XML markup at byte {start}"))
}
pub fn xml_lexical_document_from_text(text: &str) -> Result<XmlLexicalDocument, String> {
    let mut tokens = Vec::new(); let mut pos = 0usize;
    while pos < text.len() { let tail = &text[pos..]; let (end, token) = if tail.starts_with("<!--") { let end = lexical_end(text, pos + 4, "-->")?; (end, XmlLexicalToken::Comment(text[pos..end].into())) } else if tail.starts_with("<![CDATA[") { let end = lexical_end(text, pos + 9, "]]>")?; (end, XmlLexicalToken::CData(text[pos..end].into())) } else if tail.starts_with("<!DOCTYPE") { let end = markup_end(text, pos + 9, true)?; (end, XmlLexicalToken::Doctype(text[pos..end].into())) } else if tail.starts_with("<?") { let end = lexical_end(text, pos + 2, "?>")?; let raw = text[pos..end].to_string(); let token = if tail[2..].starts_with("xml") { XmlLexicalToken::Declaration(raw) } else { XmlLexicalToken::ProcessingInstruction(raw) }; (end, token) } else if tail.starts_with("</") { let end = markup_end(text, pos + 2, false)?; (end, XmlLexicalToken::EndTag(text[pos..end].into())) } else if tail.starts_with('<') { let end = markup_end(text, pos + 1, false)?; (end, XmlLexicalToken::StartTag(text[pos..end].into())) } else { let end = tail.find('<').map(|offset| pos + offset).unwrap_or(text.len()); (end, XmlLexicalToken::Text(text[pos..end].into())) }; if end <= pos { return Err(format!("empty XML lexical token at byte {pos}")); } tokens.push(token); pos = end; }
    Ok(XmlLexicalDocument { tokens })
}
pub fn xml_lexical_document_to_text(document: &XmlLexicalDocument) -> String {
    let mut text = String::new(); for token in &document.tokens { let raw = match token { XmlLexicalToken::Declaration(raw) | XmlLexicalToken::Doctype(raw) | XmlLexicalToken::Comment(raw) | XmlLexicalToken::ProcessingInstruction(raw) | XmlLexicalToken::StartTag(raw) | XmlLexicalToken::EndTag(raw) | XmlLexicalToken::Text(raw) | XmlLexicalToken::CData(raw) => raw }; text.push_str(raw); } text
}
fn lexical_hex(bytes: &[u8]) -> String { const HEX: &[u8; 16] = b"0123456789abcdef"; let mut text = String::with_capacity(bytes.len() * 2); for byte in bytes { text.push(HEX[(byte >> 4) as usize] as char); text.push(HEX[(byte & 15) as usize] as char); } text }
fn lexical_unhex(text: &str) -> Result<Vec<u8>, String> { if text.len() % 2 != 0 { return Err("odd lexical hex length".into()); } text.as_bytes().chunks_exact(2).map(|pair| { let nibble = |byte: u8| match byte { b'0'..=b'9' => Ok(byte - b'0'), b'a'..=b'f' => Ok(byte - b'a' + 10), b'A'..=b'F' => Ok(byte - b'A' + 10), _ => Err("invalid lexical hex digit".to_string()) }; Ok((nibble(pair[0])? << 4) | nibble(pair[1])?) }).collect() }
pub(crate) fn encode_xml_lexical_state(document: &XmlLexicalDocument) -> String { document.tokens.iter().map(|token| { let (tag, raw) = match token { XmlLexicalToken::Declaration(raw) => ('d', raw), XmlLexicalToken::Doctype(raw) => ('o', raw), XmlLexicalToken::Comment(raw) => ('c', raw), XmlLexicalToken::ProcessingInstruction(raw) => ('p', raw), XmlLexicalToken::StartTag(raw) => ('s', raw), XmlLexicalToken::EndTag(raw) => ('e', raw), XmlLexicalToken::Text(raw) => ('t', raw), XmlLexicalToken::CData(raw) => ('x', raw) }; format!("{tag}{}", lexical_hex(raw.as_bytes())) }).collect::<Vec<_>>().join(".") }
pub(crate) fn decode_xml_lexical_state(state: &str) -> Result<XmlLexicalDocument, String> { if state.is_empty() { return Ok(XmlLexicalDocument::default()); } let tokens = state.split('.').map(|item| { let (tag, hex) = item.split_at(1); let raw = String::from_utf8(lexical_unhex(hex)?).map_err(|error| error.to_string())?; match tag { "d" => Ok(XmlLexicalToken::Declaration(raw)), "o" => Ok(XmlLexicalToken::Doctype(raw)), "c" => Ok(XmlLexicalToken::Comment(raw)), "p" => Ok(XmlLexicalToken::ProcessingInstruction(raw)), "s" => Ok(XmlLexicalToken::StartTag(raw)), "e" => Ok(XmlLexicalToken::EndTag(raw)), "t" => Ok(XmlLexicalToken::Text(raw)), "x" => Ok(XmlLexicalToken::CData(raw)), _ => Err(format!("unknown XML lexical token tag {tag:?}")) } }).collect::<Result<Vec<_>, String>>()?; Ok(XmlLexicalDocument { tokens }) }
