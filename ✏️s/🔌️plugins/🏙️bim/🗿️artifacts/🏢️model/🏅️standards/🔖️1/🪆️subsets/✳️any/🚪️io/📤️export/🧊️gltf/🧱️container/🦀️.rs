//! 🧱️ The binary glTF (GLB) container: a 12-byte header, one JSON chunk and an optional binary chunk, every chunk padded to four bytes.
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#binary-gltf-layout

const MAGIC: u32 = 0x4654_6C67;
const VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

fn padded(mut bytes: Vec<u8>, filler: u8) -> Vec<u8> {
    while bytes.len() % 4 != 0 {
        bytes.push(filler);
    }
    bytes
}

fn chunk(out: &mut Vec<u8>, kind: u32, body: &[u8]) {
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&kind.to_le_bytes());
    out.extend_from_slice(body);
}

/// 📦️ The GLB bytes of a JSON document and its binary buffer; an empty buffer writes no binary chunk.
pub fn pack_glb(json: &str, buffer: &[u8]) -> Vec<u8> {
    let json = padded(json.as_bytes().to_vec(), b' ');
    let buffer = padded(buffer.to_vec(), 0);
    let length = 12 + 8 + json.len() + if buffer.is_empty() { 0 } else { 8 + buffer.len() };
    let mut out = Vec::with_capacity(length);
    out.extend_from_slice(&MAGIC.to_le_bytes());
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(length as u32).to_le_bytes());
    chunk(&mut out, JSON_CHUNK, &json);
    if !buffer.is_empty() {
        chunk(&mut out, BIN_CHUNK, &buffer);
    }
    out
}

fn word(bytes: &[u8], at: usize) -> Result<u32, String> {
    bytes.get(at..at + 4).map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]])).ok_or_else(|| format!("glb: truncated at byte {at}"))
}

/// 🔍️ The JSON text and the binary chunk of a GLB (an absent binary chunk is empty); the header, version, total length and chunk framing are checked.
pub fn split_glb(bytes: &[u8]) -> Result<(&str, &[u8]), String> {
    if word(bytes, 0)? != MAGIC {
        return Err("glb: the magic is not glTF".into());
    }
    if word(bytes, 4)? != VERSION {
        return Err("glb: only container version 2 is read".into());
    }
    if word(bytes, 8)? as usize != bytes.len() {
        return Err("glb: the declared length differs from the byte count".into());
    }
    let (mut at, mut json, mut buffer) = (12, None, &bytes[0..0]);
    while at < bytes.len() {
        let (length, kind) = (word(bytes, at)? as usize, word(bytes, at + 4)?);
        let body = bytes.get(at + 8..at + 8 + length).ok_or("glb: a chunk runs past the end")?;
        match kind {
            JSON_CHUNK if json.is_none() => json = Some(std::str::from_utf8(body).map_err(|error| format!("glb: the JSON chunk is not UTF-8: {error}"))?),
            BIN_CHUNK if buffer.is_empty() => buffer = body,
            _ => {}
        }
        at += 8 + length;
    }
    Ok((json.ok_or("glb: no JSON chunk")?, buffer))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
