//! 🧵️ Unicode-safe bounded text spans preserve complete story content.

use std::ops::Range;

/// 📏️ One original UTF-8 byte span is addressed without retaining a story-sized index.
#[derive(Clone)]pub struct TextChunk{pub bytes:Range<usize>,pub start:usize,pub end:usize}
#[derive(Clone,Copy)]pub struct TextChunks<'a>{content:&'a str}
impl TextChunks<'_>{
 pub fn len(&self)->usize{self.content.len().saturating_sub(1)/252+1}
 pub fn get(&self,index:usize)->Option<TextChunk>{if index>=self.len(){return None}let boundary=|mut byte:usize|{while !self.content.is_char_boundary(byte){byte-=1}byte};let start=boundary(index*252);let end=boundary(index.saturating_add(1).saturating_mul(252).min(self.content.len()));Some(TextChunk{bytes:start..end,start,end})}
 pub fn iter(&self)->impl ExactSizeIterator<Item=TextChunk>+'_ {(0..self.len()).map(move|index|self.get(index).unwrap())}
}
/// 🪟️ Direct byte slots borrow at most 255 bytes and inspect at most three boundary bytes.
pub fn ranges(content:&str)->TextChunks<'_>{TextChunks{content}}
/// 🪪️ Source fingerprints prevent positional drafts from applying to newer content.
pub fn fingerprint(bytes: &[u8]) -> String { store::content_id("layout-span", bytes) }

/// 🧾️ Includes intrinsic object member order because dictionary addresses use ordered positions.
pub fn dictionary_stamp(document: &semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot) -> String {
    let Some(dictionary) = &document.data_fields else { return fingerprint(b"null") };
    let mut content = semio_framework_pack_json::to_json_string(dictionary);
    let mut pending: Vec<_> = dictionary.entries.iter().rev().map(|entry| &entry.value).collect();
    while let Some(value) = pending.pop() {
        match value {
            semio_framework_value::DslValue::Object(entries) => {
                content.push('\0');
                for (key, _) in entries { content.push_str(&format!("{}:{key}", key.len())); }
                pending.extend(entries.iter().rev().map(|(_, value)| value));
            }
            semio_framework_value::DslValue::Array(entries) => pending.extend(entries.iter().rev()),
            _ => {}
        }
    }
    fingerprint(content.as_bytes())
}

/// ⚠️ A stale draft remains unapplied so the host can retain it for a refreshed retry.
pub fn stale() -> semio_framework_plugin::Fault {
    semio_framework_plugin::Fault::new(semio_framework_diagnostic::FaultOrigin::App, "layout.chunk.stale", "Content changed after this span was displayed. Refresh the editor and retry your change.")
}

/// 🛂️ Span and fingerprint admission is shared by text and typed byte inputs.
pub fn guarded_range(field: &str, kind: &str, stamp: &str, length: usize) -> Result<Range<usize>, semio_framework_plugin::Fault> {
    let invalid = || semio_framework_plugin::Fault::new(semio_framework_diagnostic::FaultOrigin::App, "layout.chunk.invalid", "The editing span is invalid. Refresh the editor and retry.");
    let address = field.strip_prefix(kind).and_then(|rest| rest.strip_prefix('.')).ok_or_else(invalid)?;
    let mut parts = address.splitn(3, '.');
    let start = parts.next().and_then(|value| value.parse::<usize>().ok()).ok_or_else(invalid)?;
    let end = parts.next().and_then(|value| value.parse::<usize>().ok()).ok_or_else(invalid)?;
    let expected = parts.next().ok_or_else(invalid)?;
    if expected != stamp { return Err(stale()) }
    if start > end || end > length { return Err(invalid()) }
    Ok(start..end)
}

/// ✂️ A fresh bounded UTF-8 draft becomes a minimal native character splice.
pub fn splice(content: &str, field: &str, value: &str, stamp: &str) -> Result<Option<(usize, usize, String)>, semio_framework_plugin::Fault> {
    let span=guarded_range(field,"chunk",stamp,content.len())?;
    let old=content.get(span.clone()).ok_or_else(||semio_framework_plugin::Fault::new(semio_framework_diagnostic::FaultOrigin::App,"layout.chunk.invalid","The editing span must follow UTF-8 boundaries."))?;
    let prefix=content[..span.start].chars().count();
    Ok(crate::patch_frame::story_splice(old,value).map(|(offset,delete,insert)|(prefix+offset,delete,insert)))
}

/// 🧱️ Hex inputs expose every byte in aligned windows with insertion headroom.
#[derive(Clone,Copy)]pub struct ByteChunks{length:usize}
impl ByteChunks{
 pub fn len(&self)->usize{self.length.saturating_sub(1)/128+1}
 pub fn get(&self,index:usize)->Option<Range<usize>>{if index>=self.len(){return None}let start=index*128;Some(start..start.saturating_add(128).min(self.length))}
 pub fn iter(&self)->impl ExactSizeIterator<Item=Range<usize>>+'_ {(0..self.len()).map(move|index|self.get(index).unwrap())}
}
pub fn byte_ranges(length:usize)->ByteChunks{ByteChunks{length}}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn direct_span_queries_hold_no_story_sized_index(){let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){let content=row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let spans=ranges(&content);assert!(std::mem::size_of_val(&spans)<=std::mem::size_of::<&str>());let mut cursor=0;for span in spans.iter(){assert_eq!(span.start,cursor);assert!(span.end-span.start<=law["textMaximumSpanBytes"].as_u64().unwrap()as usize);assert!(content.is_char_boundary(span.start)&&content.is_char_boundary(span.end));cursor=span.end}assert_eq!(cursor,content.len());assert!(spans.get(spans.len()).is_none())}}
 #[test]fn byte_addressed_commits_refuse_partial_utf8_scalars(){let text="Aé🌍";let stamp=fingerprint(text.as_bytes());assert!(splice(text,&format!("chunk.2.3.{stamp}"),"X",&stamp).is_err());let result=splice(text,&format!("chunk.1.3.{stamp}"),"ö",&stamp).unwrap().unwrap();assert_eq!((result.0,result.1,result.2),(1,1,"ö".into()))}
}
