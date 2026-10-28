//! ✅️ Layout export JSON validation: the bounded, resumable JSON validation cursor (see [`JsonValidationCursor`]) that checks the package fragments of the export job.

use super::{MAX_LAYOUT_EXPORT_JSON_NODES, MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES, MAX_LAYOUT_EXPORT_STRING_BYTES};
use semio_framework_value_derive::{FromValue, ToValue};

pub(super) const JSON_INPUT_BYTES_PER_UNIT: usize = 2_048;

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonRootState {
    Value,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum JsonArrayState {
    FirstValueOrEnd,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum JsonObjectState {
    FirstKeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum JsonContainer {
    Array(JsonArrayState),
    Object(JsonObjectState),
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonStringEscape {
    None,
    Escape,
    Unicode { digits: u8, value: u16 },
    LowSlash,
    LowU,
    LowUnicode { digits: u8, value: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonNumberState {
    Sign,
    Zero,
    Integer,
    FractionStart,
    Fraction,
    ExponentStart,
    ExponentSign,
    Exponent,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonLiteral {
    True,
    False,
    Null,
}

impl JsonLiteral {
    fn bytes(&self) -> &'static [u8] {
        match self {
            Self::True => b"true",
            Self::False => b"false",
            Self::Null => b"null",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonToken {
    None,
    String { key: bool, decoded_bytes: usize, escape: JsonStringEscape },
    Number(JsonNumberState),
    Literal { literal: JsonLiteral, cursor: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub struct JsonValidationCursor {
    pub byte_cursor: usize,
    nodes: usize,
    require_array: bool,
    root: JsonRootState,
    stack: Vec<JsonContainer>,
    token: JsonToken,
}

impl JsonValidationCursor {
    pub fn new(text: &str, require_array: bool) -> Result<Self, String> {
        if text.len() > MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES {
            return Err("layout-export-json-byte-limit".into());
        }
        Ok(Self { byte_cursor: 0, nodes: 0, require_array, root: JsonRootState::Value, stack: Vec::new(), token: JsonToken::None })
    }

    fn add_node(&mut self) -> Result<(), String> {
        self.nodes = self.nodes.checked_add(1).ok_or("layout-export-json-node-limit")?;
        if self.nodes > MAX_LAYOUT_EXPORT_JSON_NODES || self.stack.len() > 64 {
            return Err("layout-export-json-node-limit".into());
        }
        Ok(())
    }

    fn start_value(&mut self, byte: u8) -> Result<(), String> {
        if self.require_array && self.root == JsonRootState::Value && self.stack.is_empty() && byte != b'[' {
            return Err("layout-export-preflight-schema".into());
        }
        match self.stack.last_mut() {
            Some(JsonContainer::Array(state @ (JsonArrayState::FirstValueOrEnd | JsonArrayState::Value))) => *state = JsonArrayState::CommaOrEnd,
            Some(JsonContainer::Object(state @ JsonObjectState::Value)) => *state = JsonObjectState::CommaOrEnd,
            None if self.root == JsonRootState::Value => self.root = JsonRootState::Done,
            _ => return Err("layout-export-json-structure".into()),
        }
        self.add_node()?;
        match byte {
            b'{' => self.stack.push(JsonContainer::Object(JsonObjectState::FirstKeyOrEnd)),
            b'[' => self.stack.push(JsonContainer::Array(JsonArrayState::FirstValueOrEnd)),
            b'"' => self.token = JsonToken::String { key: false, decoded_bytes: 0, escape: JsonStringEscape::None },
            b'-' => self.token = JsonToken::Number(JsonNumberState::Sign),
            b'0' => self.token = JsonToken::Number(JsonNumberState::Zero),
            b'1'..=b'9' => self.token = JsonToken::Number(JsonNumberState::Integer),
            b't' => self.token = JsonToken::Literal { literal: JsonLiteral::True, cursor: 1 },
            b'f' => self.token = JsonToken::Literal { literal: JsonLiteral::False, cursor: 1 },
            b'n' => self.token = JsonToken::Literal { literal: JsonLiteral::Null, cursor: 1 },
            _ => return Err("layout-export-json-value".into()),
        }
        if self.stack.len() > 64 {
            return Err("layout-export-json-depth-limit".into());
        }
        Ok(())
    }

    fn finish_string(&mut self, key: bool) -> Result<(), String> {
        if key {
            match self.stack.last_mut() {
                Some(JsonContainer::Object(state @ (JsonObjectState::FirstKeyOrEnd | JsonObjectState::Key))) => {
                    *state = JsonObjectState::Colon;
                    Ok(())
                }
                _ => Err("layout-export-json-key".into()),
            }
        } else {
            Ok(())
        }
    }

    fn add_string_bytes(decoded_bytes: &mut usize, bytes: usize) -> Result<(), String> {
        *decoded_bytes = decoded_bytes.checked_add(bytes).ok_or("layout-export-json-string-limit")?;
        if *decoded_bytes > MAX_LAYOUT_EXPORT_STRING_BYTES {
            return Err("layout-export-json-string-limit".into());
        }
        Ok(())
    }

    fn hex(byte: u8) -> Option<u16> {
        match byte {
            b'0'..=b'9' => Some(u16::from(byte - b'0')),
            b'a'..=b'f' => Some(u16::from(byte - b'a' + 10)),
            b'A'..=b'F' => Some(u16::from(byte - b'A' + 10)),
            _ => None,
        }
    }

    fn finish_unicode(decoded_bytes: &mut usize, value: u16, low: bool) -> Result<JsonStringEscape, String> {
        if low {
            if !(0xdc00..=0xdfff).contains(&value) {
                return Err("layout-export-json-unicode".into());
            }
            Self::add_string_bytes(decoded_bytes, 4)?;
            return Ok(JsonStringEscape::None);
        }
        if (0xd800..=0xdbff).contains(&value) {
            return Ok(JsonStringEscape::LowSlash);
        }
        if (0xdc00..=0xdfff).contains(&value) {
            return Err("layout-export-json-unicode".into());
        }
        let scalar = char::from_u32(u32::from(value)).ok_or("layout-export-json-unicode")?;
        Self::add_string_bytes(decoded_bytes, scalar.len_utf8())?;
        Ok(JsonStringEscape::None)
    }

    fn number_delimiter(byte: u8) -> bool {
        byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}')
    }

    fn advance_token(&mut self, bytes: &[u8]) -> Result<bool, String> {
        match &mut self.token {
            JsonToken::None => Ok(false),
            JsonToken::String { key, decoded_bytes, escape } => {
                let byte = *bytes.get(self.byte_cursor).ok_or("layout-export-json-string")?;
                let low = matches!(escape, JsonStringEscape::LowUnicode { .. });
                match escape {
                    JsonStringEscape::None => match byte {
                        b'"' => {
                            let key = *key;
                            self.byte_cursor += 1;
                            self.token = JsonToken::None;
                            self.finish_string(key)?;
                        }
                        b'\\' => {
                            self.byte_cursor += 1;
                            *escape = JsonStringEscape::Escape;
                        }
                        0..=0x1f => return Err("layout-export-json-string".into()),
                        _ => {
                            self.byte_cursor += 1;
                            Self::add_string_bytes(decoded_bytes, 1)?;
                        }
                    },
                    JsonStringEscape::Escape => {
                        self.byte_cursor += 1;
                        match byte {
                            b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {
                                Self::add_string_bytes(decoded_bytes, 1)?;
                                *escape = JsonStringEscape::None;
                            }
                            b'u' => *escape = JsonStringEscape::Unicode { digits: 0, value: 0 },
                            _ => return Err("layout-export-json-escape".into()),
                        }
                    }
                    JsonStringEscape::Unicode { digits, value } | JsonStringEscape::LowUnicode { digits, value } => {
                        let digit = Self::hex(byte).ok_or("layout-export-json-unicode")?;
                        self.byte_cursor += 1;
                        *value = (*value << 4) | digit;
                        *digits += 1;
                        if *digits == 4 {
                            *escape = Self::finish_unicode(decoded_bytes, *value, low)?;
                        }
                    }
                    JsonStringEscape::LowSlash => {
                        if byte != b'\\' {
                            return Err("layout-export-json-unicode".into());
                        }
                        self.byte_cursor += 1;
                        *escape = JsonStringEscape::LowU;
                    }
                    JsonStringEscape::LowU => {
                        if byte != b'u' {
                            return Err("layout-export-json-unicode".into());
                        }
                        self.byte_cursor += 1;
                        *escape = JsonStringEscape::LowUnicode { digits: 0, value: 0 };
                    }
                }
                Ok(true)
            }
            JsonToken::Literal { literal, cursor } => {
                let expected = literal.bytes();
                let byte = *bytes.get(self.byte_cursor).ok_or("layout-export-json-literal")?;
                if expected.get(*cursor).copied() != Some(byte) {
                    return Err("layout-export-json-literal".into());
                }
                self.byte_cursor += 1;
                *cursor += 1;
                if *cursor == expected.len() {
                    self.token = JsonToken::None;
                }
                Ok(true)
            }
            JsonToken::Number(state) => {
                let Some(byte) = bytes.get(self.byte_cursor).copied() else {
                    return Ok(false);
                };
                let next = match *state {
                    JsonNumberState::Sign => match byte {
                        b'0' => JsonNumberState::Zero,
                        b'1'..=b'9' => JsonNumberState::Integer,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Zero => match byte {
                        b'.' => JsonNumberState::FractionStart,
                        b'e' | b'E' => JsonNumberState::ExponentStart,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Integer => match byte {
                        b'0'..=b'9' => JsonNumberState::Integer,
                        b'.' => JsonNumberState::FractionStart,
                        b'e' | b'E' => JsonNumberState::ExponentStart,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::FractionStart => match byte {
                        b'0'..=b'9' => JsonNumberState::Fraction,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Fraction => match byte {
                        b'0'..=b'9' => JsonNumberState::Fraction,
                        b'e' | b'E' => JsonNumberState::ExponentStart,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::ExponentStart => match byte {
                        b'+' | b'-' => JsonNumberState::ExponentSign,
                        b'0'..=b'9' => JsonNumberState::Exponent,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::ExponentSign => match byte {
                        b'0'..=b'9' => JsonNumberState::Exponent,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Exponent => match byte {
                        b'0'..=b'9' => JsonNumberState::Exponent,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                };
                *state = next;
                self.byte_cursor += 1;
                Ok(true)
            }
        }
    }

    fn start_key(&mut self) {
        self.token = JsonToken::String { key: true, decoded_bytes: 0, escape: JsonStringEscape::None };
    }

    fn close_container(&mut self) -> Result<(), String> {
        self.stack.pop().ok_or_else(|| "layout-export-json-structure".into()).map(|_| ())
    }

    pub fn advance(&mut self, text: &str) -> Result<bool, String> {
        let bytes = text.as_bytes();
        let end = self.byte_cursor.saturating_add(JSON_INPUT_BYTES_PER_UNIT).min(bytes.len());
        while self.byte_cursor < end {
            if !matches!(self.token, JsonToken::None) && self.advance_token(bytes)? {
                continue;
            }
            let byte = bytes[self.byte_cursor];
            if byte.is_ascii_whitespace() {
                self.byte_cursor += 1;
                continue;
            }
            match self.stack.last().cloned() {
                None => match self.root {
                    JsonRootState::Value => {
                        self.byte_cursor += 1;
                        self.start_value(byte)?;
                    }
                    JsonRootState::Done => return Err("layout-export-json-trailing".into()),
                },
                Some(JsonContainer::Array(JsonArrayState::FirstValueOrEnd)) => {
                    if byte == b']' {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    } else {
                        self.byte_cursor += 1;
                        self.start_value(byte)?;
                    }
                }
                Some(JsonContainer::Array(JsonArrayState::Value)) => {
                    self.byte_cursor += 1;
                    self.start_value(byte)?;
                }
                Some(JsonContainer::Array(JsonArrayState::CommaOrEnd)) => match byte {
                    b',' => {
                        self.byte_cursor += 1;
                        if let Some(JsonContainer::Array(state)) = self.stack.last_mut() {
                            *state = JsonArrayState::Value;
                        }
                    }
                    b']' => {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    }
                    _ => return Err("layout-export-json-array".into()),
                },
                Some(JsonContainer::Object(JsonObjectState::FirstKeyOrEnd)) => {
                    if byte == b'}' {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    } else if byte == b'"' {
                        self.byte_cursor += 1;
                        self.start_key();
                    } else {
                        return Err("layout-export-json-key".into());
                    }
                }
                Some(JsonContainer::Object(JsonObjectState::Key)) => {
                    if byte != b'"' {
                        return Err("layout-export-json-key".into());
                    }
                    self.byte_cursor += 1;
                    self.start_key();
                }
                Some(JsonContainer::Object(JsonObjectState::Colon)) => {
                    if byte != b':' {
                        return Err("layout-export-json-colon".into());
                    }
                    self.byte_cursor += 1;
                    if let Some(JsonContainer::Object(state)) = self.stack.last_mut() {
                        *state = JsonObjectState::Value;
                    }
                }
                Some(JsonContainer::Object(JsonObjectState::Value)) => {
                    self.byte_cursor += 1;
                    self.start_value(byte)?;
                }
                Some(JsonContainer::Object(JsonObjectState::CommaOrEnd)) => match byte {
                    b',' => {
                        self.byte_cursor += 1;
                        if let Some(JsonContainer::Object(state)) = self.stack.last_mut() {
                            *state = JsonObjectState::Key;
                        }
                    }
                    b'}' => {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    }
                    _ => return Err("layout-export-json-object".into()),
                },
            }
        }
        if self.byte_cursor != bytes.len() {
            return Ok(false);
        }
        match &self.token {
            JsonToken::None => {}
            JsonToken::Number(JsonNumberState::Zero | JsonNumberState::Integer | JsonNumberState::Fraction | JsonNumberState::Exponent) => self.token = JsonToken::None,
            _ => return Err("layout-export-json-incomplete".into()),
        }
        if self.stack.is_empty() && self.root == JsonRootState::Done {
            Ok(true)
        } else {
            Err("layout-export-json-incomplete".into())
        }
    }
}

impl JsonValidationCursor{
 pub fn retirement_bytes(&self)->usize{self.stack.capacity().saturating_mul(std::mem::size_of::<JsonContainer>())}
 pub fn close_one(&mut self,maximum_bytes:usize)->Option<usize>{let bytes=self.retirement_bytes();if bytes==0{return None;}if bytes>maximum_bytes{return Some(0);}drop(std::mem::take(&mut self.stack));Some(bytes)}
}
