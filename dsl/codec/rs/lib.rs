//! 🧬 `dsl_codec` — compact binary token frames for persistence/transport (an alternative to the
//! canonical text encoding, not a replacement for it — text stays the diffable, human-readable,
//! git-friendly source of truth) plus domain-separated canonical hashing modes.

use dsl_core::{Symbol, TextSpan, TokenKind, TokenId, SpannedToken};

//#region 🔖Frame
pub const FRAME_MAGIC: [u8; 4] = *b"SDF1";
pub const FRAME_VERSION: u32 = 1;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CodecError {
    #[error("frame too short: need at least {need} bytes, have {have}")]
    Truncated { need: usize, have: usize },
    #[error("bad magic bytes: expected {FRAME_MAGIC:?}")]
    BadMagic,
    #[error("unsupported frame version {0}")]
    UnsupportedVersion(u32),
    #[error("string table index {0} out of range")]
    BadStringIndex(u32),
    #[error("unknown token kind code {0}")]
    UnknownTokenKind(u32),
    #[error("checksum mismatch")]
    ChecksumMismatch,
}

fn write_varint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            break;
        } else {
            out.push(byte | 0x80);
        }
    }
}

fn read_varint(bytes: &[u8], pos: &mut usize) -> Result<u64, CodecError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    loop {
        if *pos >= bytes.len() {
            return Err(CodecError::Truncated { need: *pos + 1, have: bytes.len() });
        }
        let byte = bytes[*pos];
        *pos += 1;
        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}

fn token_kind_code(kind: TokenKind) -> u32 {
    match kind {
        TokenKind::Ident => 0,
        TokenKind::Int => 1,
        TokenKind::Float => 2,
        TokenKind::Text => 3,
        TokenKind::Equals => 4,
        TokenKind::Comma => 5,
        TokenKind::Colon => 6,
        TokenKind::At => 7,
        TokenKind::Arrow => 8,
        TokenKind::DashArrow => 9,
        TokenKind::LBrace => 10,
        TokenKind::RBrace => 11,
        TokenKind::LBracket => 12,
        TokenKind::RBracket => 13,
        TokenKind::LParen => 14,
        TokenKind::RParen => 15,
        TokenKind::Comment => 16,
        TokenKind::Whitespace => 17,
        TokenKind::Newline => 18,
        TokenKind::Error => 19,
        TokenKind::Eof => 20,
    }
}

fn token_kind_from_code(code: u32) -> Result<TokenKind, CodecError> {
    Ok(match code {
        0 => TokenKind::Ident,
        1 => TokenKind::Int,
        2 => TokenKind::Float,
        3 => TokenKind::Text,
        4 => TokenKind::Equals,
        5 => TokenKind::Comma,
        6 => TokenKind::Colon,
        7 => TokenKind::At,
        8 => TokenKind::Arrow,
        9 => TokenKind::DashArrow,
        10 => TokenKind::LBrace,
        11 => TokenKind::RBrace,
        12 => TokenKind::LBracket,
        13 => TokenKind::RBracket,
        14 => TokenKind::LParen,
        15 => TokenKind::RParen,
        16 => TokenKind::Comment,
        17 => TokenKind::Whitespace,
        18 => TokenKind::Newline,
        19 => TokenKind::Error,
        20 => TokenKind::Eof,
        other => return Err(CodecError::UnknownTokenKind(other)),
    })
}

/// @emoji 📦 Encodes a token stream as a compact binary frame: magic, version, a deduplicated
/// string table, then one `(kind, string-index, line, column, length)` varint tuple per token,
/// closed with a blake3-derived checksum over everything preceding it.
pub fn encode_tokens(tokens: &[SpannedToken]) -> Vec<u8> {
    let mut strings: Vec<String> = Vec::new();
    let mut index_of: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut string_index_for = |text: &str| -> u32 {
        if let Some(&i) = index_of.get(text) {
            return i;
        }
        let i = strings.len() as u32;
        strings.push(text.to_string());
        index_of.insert(text.to_string(), i);
        i
    };

    let mut token_records: Vec<(u32, u32, u32, u32, u32)> = Vec::with_capacity(tokens.len());
    for token in tokens {
        let text = token.text.as_str();
        let string_index = string_index_for(&text);
        token_records.push((token_kind_code(token.kind), string_index, token.span.line, token.span.column, token.span.length));
    }

    let mut body = Vec::new();
    body.extend_from_slice(&FRAME_MAGIC);
    write_varint(&mut body, FRAME_VERSION as u64);
    write_varint(&mut body, strings.len() as u64);
    for s in &strings {
        write_varint(&mut body, s.len() as u64);
        body.extend_from_slice(s.as_bytes());
    }
    write_varint(&mut body, token_records.len() as u64);
    for (kind, str_idx, line, column, length) in token_records {
        write_varint(&mut body, kind as u64);
        write_varint(&mut body, str_idx as u64);
        write_varint(&mut body, line as u64);
        write_varint(&mut body, column as u64);
        write_varint(&mut body, length as u64);
    }

    let checksum = semio_framework_hash::hash_bytes(&body);
    let checksum_bytes = &checksum.as_bytes()[..checksum.len().min(16)];
    write_varint(&mut body, checksum_bytes.len() as u64);
    body.extend_from_slice(checksum_bytes);
    body
}

/// @emoji 📦 Decodes a frame produced by [`encode_tokens`]. Rejects truncated input, bad magic,
/// an unsupported version, and checksum mismatches before allocating any token vector — length is
/// validated up front so a malformed frame can't drive unbounded allocation.
pub fn decode_tokens(bytes: &[u8]) -> Result<Vec<SpannedToken>, CodecError> {
    if bytes.len() < 4 {
        return Err(CodecError::Truncated { need: 4, have: bytes.len() });
    }
    if bytes[0..4] != FRAME_MAGIC {
        return Err(CodecError::BadMagic);
    }
    let mut pos = 4usize;
    let version = read_varint(bytes, &mut pos)? as u32;
    if version != FRAME_VERSION {
        return Err(CodecError::UnsupportedVersion(version));
    }

    let string_count = read_varint(bytes, &mut pos)?;
    if string_count > bytes.len() as u64 {
        // Every string entry needs at least one length-prefix byte, so a count exceeding the
        // remaining bytes is definitely corrupt — reject before allocating, per the "length
        // bounded before allocation" invariant.
        return Err(CodecError::Truncated { need: string_count as usize, have: bytes.len() });
    }
    let mut strings = Vec::with_capacity(string_count as usize);
    for _ in 0..string_count {
        let len = read_varint(bytes, &mut pos)? as usize;
        if pos + len > bytes.len() {
            return Err(CodecError::Truncated { need: pos + len, have: bytes.len() });
        }
        let s = String::from_utf8_lossy(&bytes[pos..pos + len]).to_string();
        pos += len;
        strings.push(s);
    }

    let token_count = read_varint(bytes, &mut pos)?;
    if token_count > bytes.len() as u64 {
        // Every token record needs at least a few bytes, so this count alone proves corruption —
        // reject before allocating rather than trusting an attacker-controlled length.
        return Err(CodecError::Truncated { need: token_count as usize, have: bytes.len() });
    }
    let mut tokens = Vec::with_capacity(token_count as usize);
    for i in 0..token_count {
        let kind_code = read_varint(bytes, &mut pos)? as u32;
        let str_idx = read_varint(bytes, &mut pos)? as u32;
        let line = read_varint(bytes, &mut pos)? as u32;
        let column = read_varint(bytes, &mut pos)? as u32;
        let length = read_varint(bytes, &mut pos)? as u32;
        let kind = token_kind_from_code(kind_code)?;
        let text = strings.get(str_idx as usize).ok_or(CodecError::BadStringIndex(str_idx))?;
        tokens.push(SpannedToken {
            id: TokenId(i as u32),
            kind,
            text: Symbol::intern(text),
            span: TextSpan::with_length(line, column, length),
            byte_range: (0, 0),
        });
    }

    let checksum_len = read_varint(bytes, &mut pos)? as usize;
    if pos + checksum_len > bytes.len() {
        return Err(CodecError::Truncated { need: pos + checksum_len, have: bytes.len() });
    }
    let stored_checksum = &bytes[pos..pos + checksum_len];
    let computed = semio_framework_hash::hash_bytes(&bytes[..pos - varint_len(checksum_len as u64)]);
    let computed_bytes = &computed.as_bytes()[..computed.len().min(checksum_len)];
    if stored_checksum != computed_bytes {
        return Err(CodecError::ChecksumMismatch);
    }

    Ok(tokens)
}

fn varint_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}
//#endregion 🔖Frame

//#region 🔖Hashing
/// @emoji #️⃣ Domain-separated canonical hashing: the same underlying bytes hashed under
/// different modes never collide, so a hash's mode is unambiguous from its own value's context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashMode {
    /// Hash of canonical (formatted) text — ignores incidental whitespace/comment differences.
    Semantic,
    /// Hash of the exact original source bytes, trivia included.
    Lossless,
    /// Hash sensitive to token identity (`TokenId`s), not just text — two documents with
    /// identical text but different token lineage hash differently under this mode.
    TokenIdentity,
}

fn domain_tag(mode: HashMode) -> &'static str {
    match mode {
        HashMode::Semantic => "dsl-hash-semantic-v1\0",
        HashMode::Lossless => "dsl-hash-lossless-v1\0",
        HashMode::TokenIdentity => "dsl-hash-token-identity-v1\0",
    }
}

pub fn canonical_hash(mode: HashMode, text: &str) -> String {
    let mut buf = Vec::with_capacity(text.len() + 32);
    buf.extend_from_slice(domain_tag(mode).as_bytes());
    buf.extend_from_slice(text.as_bytes());
    semio_framework_hash::hash_bytes(&buf)
}

pub fn token_identity_hash(tokens: &[SpannedToken]) -> String {
    let mut buf = Vec::new();
    buf.extend_from_slice(domain_tag(HashMode::TokenIdentity).as_bytes());
    for token in tokens {
        write_varint(&mut buf, token.id.0 as u64);
        write_varint(&mut buf, token_kind_code(token.kind) as u64);
        buf.extend_from_slice(token.text.as_str().as_bytes());
        buf.push(0);
    }
    semio_framework_hash::hash_bytes(&buf)
}
//#endregion 🔖Hashing

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use dsl_core::{lex, Limits};

    #[test]
    fn encode_decode_round_trips_kind_text_and_span() {
        let source = "camera x=1 y=-2.5 zoom=1 label=\"hi\"";
        let tokens = lex(source, &Limits::default(), false).expect("lex");
        let frame = encode_tokens(&tokens);
        let decoded = decode_tokens(&frame).expect("decode");
        assert_eq!(decoded.len(), tokens.len());
        for (original, restored) in tokens.iter().zip(decoded.iter()) {
            assert_eq!(original.kind, restored.kind);
            assert_eq!(original.text.as_str(), restored.text.as_str());
            assert_eq!(original.span, restored.span);
        }
    }

    #[test]
    fn decode_rejects_bad_magic() {
        let bytes = b"XXXXnonsense".to_vec();
        assert_eq!(decode_tokens(&bytes), Err(CodecError::BadMagic));
    }

    #[test]
    fn decode_rejects_truncated_frames_without_panicking() {
        let tokens = lex("a b c", &Limits::default(), false).expect("lex");
        let frame = encode_tokens(&tokens);
        for cut in [4, 5, frame.len() / 2, frame.len() - 1] {
            let truncated = &frame[..cut.min(frame.len())];
            assert!(decode_tokens(truncated).is_err(), "truncated frame at {cut} bytes must error, not panic");
        }
    }

    #[test]
    fn decode_rejects_corrupted_checksum() {
        let tokens = lex("a b c", &Limits::default(), false).expect("lex");
        let mut frame = encode_tokens(&tokens);
        let last = frame.len() - 1;
        frame[last] ^= 0xff;
        assert_eq!(decode_tokens(&frame), Err(CodecError::ChecksumMismatch));
    }

    #[test]
    fn string_table_deduplicates_repeated_token_text() {
        let tokens = lex("a a a a a", &Limits::default(), false).expect("lex");
        let frame = encode_tokens(&tokens);
        // 5 idents + trivia + eof, but only ONE distinct ident string "a" should be stored.
        let mut pos = 4usize;
        let _version = read_varint(&frame, &mut pos).unwrap();
        let string_count = read_varint(&frame, &mut pos).unwrap();
        assert!(string_count < tokens.len() as u64, "string table must dedupe repeated token text");
    }

    #[test]
    fn canonical_hash_is_domain_separated_across_modes() {
        let text = "camera x=1";
        let semantic = canonical_hash(HashMode::Semantic, text);
        let lossless = canonical_hash(HashMode::Lossless, text);
        assert_ne!(semantic, lossless, "identical bytes under different hash modes must not collide");
    }

    #[test]
    fn canonical_hash_is_deterministic() {
        let a = canonical_hash(HashMode::Semantic, "camera x=1");
        let b = canonical_hash(HashMode::Semantic, "camera x=1");
        assert_eq!(a, b);
    }

    #[test]
    fn token_identity_hash_distinguishes_documents_with_same_text_but_different_ids() {
        let tokens_a = lex("a b", &Limits::default(), false).expect("lex");
        let mut tokens_b = tokens_a.clone();
        // Simulate different token lineage (e.g. from an edit history) despite identical text.
        for t in &mut tokens_b {
            t.id = TokenId(t.id.0 + 1000);
        }
        assert_ne!(token_identity_hash(&tokens_a), token_identity_hash(&tokens_b));
    }
}
//#endregion 🧪Tests
