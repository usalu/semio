//! Standalone verification harness for the F1 `stdio.json` (rfc8259) rewrite's pure logic
//! (parser/serializer, JsonValueDiff/apply/between/absorb, mutation diff/inverse), copied nearly
//! verbatim from the real implementation but stripped of `store`/`dsl`/`schema`/`protocol`
//! framework trait wiring — that wiring can't compile right now because the `protocol` facade
//! (`🧰️framework/…/📡️spr/🦀️component.rs`, an explicitly out-of-scope "frozen contract" file for
//! this ticket) doesn't yet re-export the new `DiffAlgebra` trait S1 added next to `MutationDiff`.
//! This harness exists purely to give real `cargo test` confidence in the ALGORITHMS (especially
//! the absorb symbolic-position-simulation) independent of that external blocker. See
//! `f1-json-report.md` in this ticket folder for the full writeup.

use std::collections::{HashMap, HashSet};

//#region JsonValue model (verbatim logic from snapshot/component.rs)
#[derive(Clone, Debug, PartialEq)]
pub struct JsonMember { pub key: String, pub value: JsonValue }

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number { lexeme: String },
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<JsonMember>),
}

struct Parser<'a> { bytes: &'a [u8], pos: usize }
impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self { Self { bytes: text.as_bytes(), pos: 0 } }
    fn peek(&self) -> Option<u8> { self.bytes.get(self.pos).copied() }
    fn advance(&mut self) -> Option<u8> { let b = self.peek()?; self.pos += 1; Some(b) }
    fn err(&self, msg: impl Into<String>) -> String { format!("{} at byte {}", msg.into(), self.pos) }
    fn skip_ws(&mut self) { while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) { self.advance(); } }
    fn expect(&mut self, byte: u8) -> Result<(), String> {
        match self.peek() {
            Some(b) if b == byte => { self.advance(); Ok(()) }
            _ => Err(self.err(format!("expected '{}'", byte as char))),
        }
    }
    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Ok(JsonValue::String(self.parse_string()?)),
            Some(b't') => self.parse_literal("true", JsonValue::Bool(true)),
            Some(b'f') => self.parse_literal("false", JsonValue::Bool(false)),
            Some(b'n') => self.parse_literal("null", JsonValue::Null),
            Some(b'-') | Some(b'0'..=b'9') => self.parse_number(),
            Some(other) => Err(self.err(format!("unexpected character '{}'", other as char))),
            None => Err(self.err("unexpected end of input")),
        }
    }
    fn parse_literal(&mut self, literal: &str, value: JsonValue) -> Result<JsonValue, String> {
        for expected in literal.bytes() {
            match self.advance() { Some(b) if b == expected => {}, _ => return Err(self.err(format!("expected literal '{literal}'"))) }
        }
        Ok(value)
    }
    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.expect(b'{')?;
        let mut members = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') { self.advance(); return Ok(JsonValue::Object(members)); }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') { return Err(self.err("expected a string member key")); }
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect(b':')?;
            let value = self.parse_value()?;
            members.push(JsonMember { key, value });
            self.skip_ws();
            match self.peek() {
                Some(b',') => { self.advance(); }
                Some(b'}') => { self.advance(); break; }
                _ => return Err(self.err("expected ',' or '}'")),
            }
        }
        Ok(JsonValue::Object(members))
    }
    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') { self.advance(); return Ok(JsonValue::Array(items)); }
        loop {
            let value = self.parse_value()?;
            items.push(value);
            self.skip_ws();
            match self.peek() {
                Some(b',') => { self.advance(); }
                Some(b']') => { self.advance(); break; }
                _ => return Err(self.err("expected ',' or ']'")),
            }
        }
        Ok(JsonValue::Array(items))
    }
    fn parse_string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            match self.advance() {
                None => return Err(self.err("unterminated string")),
                Some(b'"') => break,
                Some(b'\\') => match self.advance() {
                    Some(b'"') => out.push('"'), Some(b'\\') => out.push('\\'), Some(b'/') => out.push('/'),
                    Some(b'b') => out.push('\u{0008}'), Some(b'f') => out.push('\u{000C}'),
                    Some(b'n') => out.push('\n'), Some(b'r') => out.push('\r'), Some(b't') => out.push('\t'),
                    Some(b'u') => out.push(self.parse_unicode_escape()?),
                    _ => return Err(self.err("invalid escape")),
                },
                Some(b) if b < 0x20 => return Err(self.err("unescaped control char")),
                Some(b) if b < 0x80 => out.push(b as char),
                Some(lead) => {
                    let extra = if lead >= 0xF0 { 3 } else if lead >= 0xE0 { 2 } else { 1 };
                    let mut buf = vec![lead];
                    for _ in 0..extra { buf.push(self.advance().ok_or_else(|| self.err("truncated utf8"))?); }
                    out.push_str(std::str::from_utf8(&buf).map_err(|_| self.err("invalid utf8"))?);
                }
            }
        }
        Ok(out)
    }
    fn parse_unicode_escape(&mut self) -> Result<char, String> {
        let high = self.parse_hex4()?;
        if (0xD800..=0xDBFF).contains(&high) {
            if self.advance() != Some(b'\\') || self.advance() != Some(b'u') { return Err(self.err("expected low surrogate")); }
            let low = self.parse_hex4()?;
            if !(0xDC00..=0xDFFF).contains(&low) { return Err(self.err("invalid low surrogate")); }
            let combined = 0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00);
            char::from_u32(combined).ok_or_else(|| self.err("invalid surrogate pair"))
        } else {
            char::from_u32(high).ok_or_else(|| self.err("invalid \\u escape"))
        }
    }
    fn parse_hex4(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        for _ in 0..4 {
            let byte = self.advance().ok_or_else(|| self.err("eof in \\u escape"))?;
            let digit = match byte { b'0'..=b'9' => (byte - b'0') as u32, b'a'..=b'f' => (byte - b'a' + 10) as u32, b'A'..=b'F' => (byte - b'A' + 10) as u32, _ => return Err(self.err("invalid hex digit")) };
            value = value * 16 + digit;
        }
        Ok(value)
    }
    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') { self.advance(); }
        match self.peek() {
            Some(b'0') => { self.advance(); }
            Some(b'1'..=b'9') => { while matches!(self.peek(), Some(b'0'..=b'9')) { self.advance(); } }
            _ => return Err(self.err("invalid number")),
        }
        if self.peek() == Some(b'.') {
            self.advance();
            if !matches!(self.peek(), Some(b'0'..=b'9')) { return Err(self.err("invalid number frac")); }
            while matches!(self.peek(), Some(b'0'..=b'9')) { self.advance(); }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            self.advance();
            if matches!(self.peek(), Some(b'+') | Some(b'-')) { self.advance(); }
            if !matches!(self.peek(), Some(b'0'..=b'9')) { return Err(self.err("invalid number exp")); }
            while matches!(self.peek(), Some(b'0'..=b'9')) { self.advance(); }
        }
        Ok(JsonValue::Number { lexeme: std::str::from_utf8(&self.bytes[start..self.pos]).unwrap().to_string() })
    }
}

pub fn parse_json_text(text: &str) -> Result<JsonValue, String> {
    let mut parser = Parser::new(text);
    let value = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != parser.bytes.len() { return Err(parser.err("trailing characters")); }
    Ok(value)
}

pub fn write_json_text(value: &JsonValue) -> String {
    let mut out = String::new();
    write_value_compact(value, &mut out);
    out
}
fn write_value_compact(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(true) => out.push_str("true"),
        JsonValue::Bool(false) => out.push_str("false"),
        JsonValue::Number { lexeme } => out.push_str(lexeme),
        JsonValue::String(s) => write_string_escaped(s, out),
        JsonValue::Array(items) => { out.push('['); for (i, item) in items.iter().enumerate() { if i > 0 { out.push(','); } write_value_compact(item, out); } out.push(']'); }
        JsonValue::Object(members) => { out.push('{'); for (i, m) in members.iter().enumerate() { if i > 0 { out.push(','); } write_string_escaped(&m.key, out); out.push(':'); write_value_compact(&m.value, out); } out.push('}'); }
    }
}
fn write_string_escaped(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"), '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
//#endregion

//#region JsonValueDiff model + apply/between (verbatim logic from diff/component.rs)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonArrayDiff { pub removed: Vec<usize>, pub modified: Vec<JsonArrayModified>, pub added: Vec<JsonArrayAdded> }
#[derive(Clone, Debug, PartialEq)]
pub struct JsonArrayModified { pub index: usize, pub diff: JsonValueDiff }
#[derive(Clone, Debug, PartialEq)]
pub struct JsonArrayAdded { pub index: usize, pub item: JsonValue }
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonObjectDiff { pub removed: Vec<String>, pub modified: Vec<JsonObjectModified>, pub added: Vec<JsonObjectAdded> }
#[derive(Clone, Debug, PartialEq)]
pub struct JsonObjectModified { pub key: String, pub diff: JsonValueDiff }
#[derive(Clone, Debug, PartialEq)]
pub struct JsonObjectAdded { pub index: usize, pub key: String, pub item: JsonValue }

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValueDiff {
    Replace { value: JsonValue },
    Bool { value: bool },
    Number { lexeme: String },
    String { value: String },
    Array { diff: JsonArrayDiff },
    Object { diff: JsonObjectDiff },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonDiff { pub value: Option<JsonValueDiff> }

impl JsonDiff {
    pub fn apply(&self, base: &JsonValue) -> JsonValue {
        match &self.value { Some(d) => apply_value_diff(d, base), None => base.clone() }
    }
    pub fn absorb(&mut self, other: Self) {
        self.value = match (self.value.take(), other.value) {
            (None, None) => None,
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (Some(d1), Some(d2)) => Some(absorb_value_diff(d1, d2)),
        };
    }
    pub fn inverse(&self, base: &JsonValue) -> Self {
        let mid = self.apply(base);
        Self::between(&mid, base)
    }
    pub fn between(base: &JsonValue, other: &JsonValue) -> Self {
        JsonDiff { value: value_diff_between(base, other) }
    }
    pub fn is_empty(&self) -> bool { self.value.is_none() }
}

pub fn apply_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> JsonValue {
    match diff {
        JsonValueDiff::Replace { value } => value.clone(),
        JsonValueDiff::Bool { value } => JsonValue::Bool(*value),
        JsonValueDiff::Number { lexeme } => JsonValue::Number { lexeme: lexeme.clone() },
        JsonValueDiff::String { value } => JsonValue::String(value.clone()),
        JsonValueDiff::Array { diff } => { let items: &[JsonValue] = match base { JsonValue::Array(items) => items.as_slice(), _ => &[] }; JsonValue::Array(apply_array_diff(diff, items)) }
        JsonValueDiff::Object { diff } => { let members: &[JsonMember] = match base { JsonValue::Object(members) => members.as_slice(), _ => &[] }; JsonValue::Object(apply_object_diff(diff, members)) }
    }
}
pub fn apply_array_diff(diff: &JsonArrayDiff, base: &[JsonValue]) -> Vec<JsonValue> {
    let mut items: Vec<JsonValue> = base.to_vec();
    for m in &diff.modified { if let Some(old) = base.get(m.index) { if let Some(slot) = items.get_mut(m.index) { *slot = apply_value_diff(&m.diff, old); } } }
    let mut removed_sorted = diff.removed.clone(); removed_sorted.sort_unstable(); removed_sorted.dedup();
    for idx in removed_sorted.into_iter().rev() { if idx < items.len() { items.remove(idx); } }
    let mut added_sorted = diff.added.clone(); added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted { let pos = a.index.min(items.len()); items.insert(pos, a.item); }
    items
}
pub fn apply_object_diff(diff: &JsonObjectDiff, base: &[JsonMember]) -> Vec<JsonMember> {
    let mut members: Vec<JsonMember> = base.to_vec();
    for m in &diff.modified { if let Some(pos) = members.iter().position(|mem| mem.key == m.key) { let old = members[pos].value.clone(); members[pos].value = apply_value_diff(&m.diff, &old); } }
    for key in &diff.removed { if let Some(pos) = members.iter().position(|mem| &mem.key == key) { members.remove(pos); } }
    let mut added_sorted = diff.added.clone(); added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted { let pos = a.index.min(members.len()); members.insert(pos, JsonMember { key: a.key, value: a.item }); }
    members
}

pub fn value_diff_between(a: &JsonValue, b: &JsonValue) -> Option<JsonValueDiff> {
    if a == b { return None; }
    match (a, b) {
        (JsonValue::Bool(_), JsonValue::Bool(next)) => Some(JsonValueDiff::Bool { value: *next }),
        (JsonValue::Number { .. }, JsonValue::Number { lexeme }) => Some(JsonValueDiff::Number { lexeme: lexeme.clone() }),
        (JsonValue::String(_), JsonValue::String(next)) => Some(JsonValueDiff::String { value: next.clone() }),
        (JsonValue::Array(av), JsonValue::Array(bv)) => { let diff = array_diff_between(av, bv); if is_array_diff_empty(&diff) { None } else { Some(JsonValueDiff::Array { diff }) } }
        (JsonValue::Object(am), JsonValue::Object(bm)) => { let diff = object_diff_between(am, bm); if is_object_diff_empty(&diff) { None } else { Some(JsonValueDiff::Object { diff }) } }
        _ => Some(JsonValueDiff::Replace { value: b.clone() }),
    }
}
fn array_diff_between(a: &[JsonValue], b: &[JsonValue]) -> JsonArrayDiff {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min { if let Some(diff) = value_diff_between(&a[i], &b[i]) { modified.push(JsonArrayModified { index: i, diff }); } }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<JsonArrayAdded> = if b.len() > a.len() { (a.len()..b.len()).map(|i| JsonArrayAdded { index: i, item: b[i].clone() }).collect() } else { Vec::new() };
    JsonArrayDiff { removed, modified, added }
}
fn object_diff_between(a: &[JsonMember], b: &[JsonMember]) -> JsonObjectDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for am in a {
        match b.iter().find(|bm| bm.key == am.key) {
            Some(bm) => { if let Some(diff) = value_diff_between(&am.value, &bm.value) { modified.push(JsonObjectModified { key: am.key.clone(), diff }); } }
            None => removed.push(am.key.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, bm) in b.iter().enumerate() { if !a.iter().any(|am| am.key == bm.key) { added.push(JsonObjectAdded { index: i, key: bm.key.clone(), item: bm.value.clone() }); } }
    JsonObjectDiff { removed, modified, added }
}
fn is_array_diff_empty(d: &JsonArrayDiff) -> bool { d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty() }
fn is_object_diff_empty(d: &JsonObjectDiff) -> bool { d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty() }

fn absorb_value_diff(d1: JsonValueDiff, d2: JsonValueDiff) -> JsonValueDiff {
    if matches!(d2, JsonValueDiff::Replace { .. }) { return d2; }
    if let JsonValueDiff::Replace { value } = d1 { return JsonValueDiff::Replace { value: apply_value_diff(&d2, &value) }; }
    match (d1, d2) {
        (JsonValueDiff::Bool { .. }, JsonValueDiff::Bool { value }) => JsonValueDiff::Bool { value },
        (JsonValueDiff::Number { .. }, JsonValueDiff::Number { lexeme }) => JsonValueDiff::Number { lexeme },
        (JsonValueDiff::String { .. }, JsonValueDiff::String { value }) => JsonValueDiff::String { value },
        (JsonValueDiff::Array { diff: a1 }, JsonValueDiff::Array { diff: a2 }) => JsonValueDiff::Array { diff: absorb_array_diff(a1, a2) },
        (JsonValueDiff::Object { diff: o1 }, JsonValueDiff::Object { diff: o2 }) => JsonValueDiff::Object { diff: absorb_object_diff(o1, o2) },
        (_, other) => other,
    }
}

fn absorb_array_diff(d1: JsonArrayDiff, d2: JsonArrayDiff) -> JsonArrayDiff {
    #[derive(Clone, Copy)] enum Origin { Base(usize), D1Added(usize) }
    enum AfterSlot { Base { orig: usize, diff: Option<JsonValueDiff> }, D1Added { tag: usize, patch: Option<JsonValueDiff> }, D2Added(JsonValue) }

    let max_ref = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).chain(d1.added.iter().map(|a| a.index))
        .chain(d2.removed.iter().copied()).chain(d2.modified.iter().map(|m| m.index)).chain(d2.added.iter().map(|a| a.index))
        .max().unwrap_or(0);
    let n = max_ref + d1.removed.len() + d2.removed.len() + 64;

    let mut mid: Vec<Origin> = (0..n).map(Origin::Base).collect();
    let mut d1_removed_sorted = d1.removed.clone(); d1_removed_sorted.sort_unstable(); d1_removed_sorted.dedup();
    for idx in d1_removed_sorted.iter().rev() { if *idx < mid.len() { mid.remove(*idx); } }
    let mut d1_added_order: Vec<usize> = (0..d1.added.len()).collect();
    d1_added_order.sort_by_key(|&tag| d1.added[tag].index);
    for tag in d1_added_order { let pos = d1.added[tag].index.min(mid.len()); mid.insert(pos, Origin::D1Added(tag)); }
    let d1_modified: HashMap<usize, JsonValueDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();

    let mut after: Vec<AfterSlot> = mid.iter().map(|origin| match origin {
        Origin::Base(orig) => AfterSlot::Base { orig: *orig, diff: d1_modified.get(orig).cloned() },
        Origin::D1Added(tag) => AfterSlot::D1Added { tag: *tag, patch: None },
    }).collect();

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut d2_removed_sorted = d2.removed.clone(); d2_removed_sorted.sort_unstable(); d2_removed_sorted.dedup();
    for idx in d2_removed_sorted.iter().rev() {
        if *idx < after.len() {
            match after.remove(*idx) {
                AfterSlot::Base { orig, .. } => final_removed.push(orig),
                AfterSlot::D1Added { .. } => {}
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    for m in &d2.modified {
        if let Some(slot) = after.get_mut(m.index) {
            match slot {
                AfterSlot::Base { diff, .. } => { *diff = Some(match diff.take() { Some(existing) => absorb_value_diff(existing, m.diff.clone()), None => m.diff.clone() }); }
                AfterSlot::D1Added { patch, .. } => { *patch = Some(match patch.take() { Some(existing) => absorb_value_diff(existing, m.diff.clone()), None => m.diff.clone() }); }
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    let mut d2_added_order: Vec<usize> = (0..d2.added.len()).collect();
    d2_added_order.sort_by_key(|&tag| d2.added[tag].index);
    for tag in d2_added_order { let pos = d2.added[tag].index.min(after.len()); after.insert(pos, AfterSlot::D2Added(d2.added[tag].item.clone())); }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (pos, slot) in after.into_iter().enumerate() {
        match slot {
            AfterSlot::Base { orig, diff: Some(diff) } => modified.push(JsonArrayModified { index: orig, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::D1Added { tag, patch } => { let mut item = d1.added[tag].item.clone(); if let Some(patch) = patch { item = apply_value_diff(&patch, &item); } added.push(JsonArrayAdded { index: pos, item }); }
            AfterSlot::D2Added(item) => added.push(JsonArrayAdded { index: pos, item }),
        }
    }
    final_removed.sort_unstable(); final_removed.dedup();
    JsonArrayDiff { removed: final_removed, modified, added }
}

fn absorb_object_diff(d1: JsonObjectDiff, d2: JsonObjectDiff) -> JsonObjectDiff {
    let mut removed: Vec<String> = d1.removed;
    let mut modified: Vec<JsonObjectModified> = d1.modified;
    let mut added: Vec<JsonObjectAdded> = d1.added;
    let mut merged_removed: HashSet<String> = HashSet::new();

    for key in d2.removed {
        if let Some(pos) = added.iter().position(|a| a.key == key) { added.remove(pos); }
        else if let Some(pos) = modified.iter().position(|m| m.key == key) { modified.remove(pos); if merged_removed.insert(key.clone()) { removed.push(key); } }
        else if merged_removed.insert(key.clone()) { removed.push(key); }
    }
    for m in d2.modified {
        if let Some(a) = added.iter_mut().find(|a| a.key == m.key) { a.item = apply_value_diff(&m.diff, &a.item); }
        else if let Some(existing) = modified.iter_mut().find(|e| e.key == m.key) { existing.diff = absorb_value_diff(existing.diff.clone(), m.diff.clone()); }
        else { modified.push(JsonObjectModified { key: m.key, diff: m.diff }); }
    }
    for a in d2.added { added.push(a); }
    added.sort_by_key(|a| a.index);
    removed.sort(); removed.dedup();
    JsonObjectDiff { removed, modified, added }
}
//#endregion

//#region JsonPath + mutation diff/inverse (verbatim logic from mutations/component.rs)
#[derive(Clone, Debug, PartialEq)]
pub enum JsonPathSegment { Key(String), Index(usize) }
pub type JsonPath = Vec<JsonPathSegment>;

fn resolve<'a>(root: &'a JsonValue, path: &[JsonPathSegment]) -> Option<&'a JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (JsonPathSegment::Key(key), JsonValue::Object(members)) => &members.iter().find(|m| &m.key == key)?.value,
            (JsonPathSegment::Index(index), JsonValue::Array(items)) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsonMutation {
    NoMutation,
    SetMember { path: JsonPath, key: String, value: JsonValue },
    RemoveMember { path: JsonPath, key: String },
    InsertArrayElement { path: JsonPath, index: usize, value: JsonValue },
    RemoveArrayElement { path: JsonPath, index: usize },
    SetScalar { path: JsonPath, value: JsonValue },
}

fn diff_at_path(path: &[JsonPathSegment], leaf: Option<JsonValueDiff>) -> JsonDiff {
    JsonDiff { value: leaf.map(|leaf| wrap_at_path(path, leaf)) }
}
fn wrap_at_path(path: &[JsonPathSegment], leaf: JsonValueDiff) -> JsonValueDiff {
    match path.split_first() {
        None => leaf,
        Some((JsonPathSegment::Key(key), rest)) => JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] } },
        Some((JsonPathSegment::Index(index), rest)) => JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonArrayModified { index: *index, diff: wrap_at_path(rest, leaf) }] } },
    }
}

impl JsonMutation {
    pub fn diff(&self, base: &JsonValue) -> JsonDiff {
        match self {
            JsonMutation::NoMutation => JsonDiff::default(),
            JsonMutation::SetMember { path, key, value } => match resolve(base, path) {
                Some(JsonValue::Object(members)) => match members.iter().find(|m| &m.key == key) {
                    Some(existing) => {
                        let leaf = value_diff_between(&existing.value, value);
                        diff_at_path(path, leaf.map(|diff| JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: key.clone(), diff }] } }))
                    }
                    None => diff_at_path(path, Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonObjectAdded { index: members.len(), key: key.clone(), item: value.clone() }] } })),
                },
                _ => JsonDiff::default(),
            },
            JsonMutation::RemoveMember { path, key } => match resolve(base, path) {
                Some(JsonValue::Object(members)) if members.iter().any(|m| &m.key == key) => diff_at_path(path, Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: vec![key.clone()], modified: Vec::new(), added: Vec::new() } })),
                _ => JsonDiff::default(),
            },
            JsonMutation::InsertArrayElement { path, index, value } => match resolve(base, path) {
                Some(JsonValue::Array(items)) => diff_at_path(path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonArrayAdded { index: (*index).min(items.len()), item: value.clone() }] } })),
                _ => JsonDiff::default(),
            },
            JsonMutation::RemoveArrayElement { path, index } => match resolve(base, path) {
                Some(JsonValue::Array(items)) if *index < items.len() => diff_at_path(path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() } })),
                _ => JsonDiff::default(),
            },
            JsonMutation::SetScalar { path, value } => match resolve(base, path) {
                Some(old) if old != value => diff_at_path(path, Some(JsonValueDiff::Replace { value: value.clone() })),
                _ => JsonDiff::default(),
            },
        }
    }

    pub fn inverse(&self, base: &JsonValue) -> Vec<Self> {
        match self {
            JsonMutation::NoMutation => vec![JsonMutation::NoMutation],
            JsonMutation::SetMember { path, key, .. } => match resolve(base, path) {
                Some(JsonValue::Object(members)) => match members.iter().find(|m| &m.key == key) {
                    Some(existing) => vec![JsonMutation::SetMember { path: path.clone(), key: key.clone(), value: existing.value.clone() }],
                    None => vec![JsonMutation::RemoveMember { path: path.clone(), key: key.clone() }],
                },
                _ => vec![JsonMutation::NoMutation],
            },
            JsonMutation::RemoveMember { path, key } => match resolve(base, path) {
                Some(JsonValue::Object(members)) => match members.iter().find(|m| &m.key == key) {
                    Some(existing) => vec![JsonMutation::SetMember { path: path.clone(), key: key.clone(), value: existing.value.clone() }],
                    None => vec![JsonMutation::NoMutation],
                },
                _ => vec![JsonMutation::NoMutation],
            },
            JsonMutation::InsertArrayElement { path, index, .. } => match resolve(base, path) {
                Some(JsonValue::Array(items)) => vec![JsonMutation::RemoveArrayElement { path: path.clone(), index: (*index).min(items.len()) }],
                _ => vec![JsonMutation::NoMutation],
            },
            JsonMutation::RemoveArrayElement { path, index } => match resolve(base, path) {
                Some(JsonValue::Array(items)) => match items.get(*index) {
                    Some(item) => vec![JsonMutation::InsertArrayElement { path: path.clone(), index: *index, value: item.clone() }],
                    None => vec![JsonMutation::NoMutation],
                },
                _ => vec![JsonMutation::NoMutation],
            },
            JsonMutation::SetScalar { path, .. } => match resolve(base, path) {
                Some(old) => vec![JsonMutation::SetScalar { path: path.clone(), value: old.clone() }],
                None => vec![JsonMutation::NoMutation],
            },
        }
    }
}

pub fn apply_json_mutation(snapshot: &mut JsonValue, mutation: &JsonMutation) -> JsonDiff {
    let diff = mutation.diff(snapshot);
    *snapshot = diff.apply(snapshot);
    diff
}
//#endregion

fn main() {
    println!("run with `cargo test` — this binary just proves the crate builds.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue { JsonValue::Object(pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect()) }
    fn arr(items: Vec<JsonValue>) -> JsonValue { JsonValue::Array(items) }
    fn num(lexeme: &str) -> JsonValue { JsonValue::Number { lexeme: lexeme.into() } }
    fn str_(s: &str) -> JsonValue { JsonValue::String(s.into()) }

    //#region parser/serializer
    #[test]
    fn preserves_number_lexeme_verbatim() {
        for lexeme in ["0", "-0", "3.140", "1e10", "1E+10", "-1.5e-3", "9007199254740993", "100000000000000000000000000000"] {
            let value = parse_json_text(lexeme).unwrap();
            assert_eq!(value, JsonValue::Number { lexeme: lexeme.into() });
            assert_eq!(write_json_text(&value), lexeme);
        }
    }
    #[test]
    fn rejects_leading_zero() { assert!(parse_json_text("01").is_err()); }
    #[test]
    fn preserves_object_member_order() {
        let value = parse_json_text(r#"{"z": 1, "a": 2, "m": 3}"#).unwrap();
        assert_eq!(write_json_text(&value), r#"{"z":1,"a":2,"m":3}"#);
    }
    #[test]
    fn decodes_surrogate_pair() {
        let value = parse_json_text(r#""a\tb\nc\"\\ A 😀""#).unwrap();
        assert_eq!(value, JsonValue::String("a\tb\nc\"\\ A 😀".into()));
    }
    //#endregion

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_scalars_and_kind_change() {
        let cases = [(JsonValue::Null, JsonValue::Bool(true)), (JsonValue::Bool(true), JsonValue::Bool(false)), (num("1"), num("2.5e10")), (str_("a"), str_("b")), (num("1"), str_("1"))];
        for (a, b) in cases {
            assert_eq!(JsonDiff::between(&a, &b).apply(&a), b);
            assert_eq!(JsonDiff::between(&b, &a).apply(&b), a);
        }
    }
    #[test]
    fn between_roundtrip_law_nested() {
        let a = obj(vec![("tags", arr(vec![str_("x"), str_("y")])), ("n", num("1"))]);
        let b = obj(vec![("tags", arr(vec![str_("x"), str_("z"), str_("w")])), ("n", num("2")), ("extra", JsonValue::Bool(true))]);
        assert_eq!(JsonDiff::between(&a, &b).apply(&a), b);
        assert_eq!(JsonDiff::between(&b, &a).apply(&b), a);
    }
    #[test]
    fn between_self_is_empty() { let a = obj(vec![("x", num("1"))]); assert!(JsonDiff::between(&a, &a).is_empty()); }
    //#endregion

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let a = obj(vec![("x", num("1")), ("y", arr(vec![num("1"), num("2")]))]);
        let b = obj(vec![("x", num("2")), ("z", str_("new"))]);
        let d = JsonDiff::between(&a, &b);
        let mid = d.apply(&a);
        assert_eq!(mid, b);
        assert_eq!(d.inverse(&a).apply(&mid), a);
    }
    #[test]
    fn inverse_law_mutation_level() {
        let base = obj(vec![("a", num("1")), ("list", arr(vec![num("1"), num("2")]))]);
        let mutations = vec![
            JsonMutation::SetMember { path: vec![], key: "a".into(), value: num("2") },
            JsonMutation::SetMember { path: vec![], key: "new".into(), value: str_("fresh") },
            JsonMutation::RemoveMember { path: vec![], key: "a".into() },
            JsonMutation::InsertArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 1, value: num("99") },
            JsonMutation::RemoveArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 0 },
            JsonMutation::SetScalar { path: vec![JsonPathSegment::Key("a".into())], value: str_("replaced") },
        ];
        for mutation in mutations {
            let mut state = base.clone();
            apply_json_mutation(&mut state, &mutation);
            for undo in mutation.inverse(&base) { apply_json_mutation(&mut state, &undo); }
            assert_eq!(state, base, "mutation {mutation:?} failed to round-trip");
        }
    }
    //#endregion

    //#region mutation_diff_law
    #[test]
    fn mutation_diff_law_all_variants() {
        let base = obj(vec![("a", num("1")), ("list", arr(vec![num("1"), num("2")]))]);
        let mutations = vec![
            JsonMutation::NoMutation,
            JsonMutation::SetMember { path: vec![], key: "a".into(), value: num("2") },
            JsonMutation::SetMember { path: vec![], key: "new".into(), value: str_("fresh") },
            JsonMutation::RemoveMember { path: vec![], key: "a".into() },
            JsonMutation::InsertArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 1, value: num("99") },
            JsonMutation::RemoveArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 0 },
            JsonMutation::SetScalar { path: vec![JsonPathSegment::Key("a".into())], value: str_("replaced") },
        ];
        for mutation in mutations {
            let mut via_apply = base.clone();
            let returned = apply_json_mutation(&mut via_apply, &mutation);
            let expected = mutation.diff(&base);
            assert_eq!(returned, expected);
            assert_eq!(via_apply, expected.apply(&base));
        }
    }
    //#endregion

    //#region absorb_law (array/index-keyed)
    #[test]
    fn absorb_array_insert_then_remove_before() {
        let base = arr(vec![str_("a"), str_("b"), str_("c")]);
        let mid = arr(vec![str_("a"), str_("b"), str_("f"), str_("c")]);
        let after = arr(vec![str_("b"), str_("f"), str_("c")]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        assert_eq!(combined.apply(&base), d2.apply(&d1.apply(&base)));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => { assert_eq!(diff.removed, vec![0]); assert_eq!(diff.added, vec![JsonArrayAdded { index: 1, item: str_("f") }]); }
            other => panic!("expected array diff, got {other:?}"),
        }
    }
    #[test]
    fn absorb_array_insert_insert_same_index_both_survive() {
        let base = arr(vec![str_("a"), str_("b")]);
        let mid = arr(vec![str_("a"), str_("b"), str_("f")]);
        let after = arr(vec![str_("a"), str_("b"), str_("g"), str_("f")]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.value { Some(JsonValueDiff::Array { diff }) => assert_eq!(diff.added.len(), 2), other => panic!("{other:?}") }
    }
    #[test]
    fn absorb_array_insert_then_remove_cancels() {
        let base = arr(vec![str_("a")]);
        let mid = arr(vec![str_("a"), str_("f")]);
        let after = arr(vec![str_("a")]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), base);
        assert!(combined.is_empty());
    }
    #[test]
    fn absorb_array_add_then_setfield_patches_payload() {
        let base = arr(vec![]);
        let mid = arr(vec![obj(vec![("x", num("1"))])]);
        let after = arr(vec![obj(vec![("x", num("1")), ("y", num("2"))])]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.value { Some(JsonValueDiff::Array { diff }) => { assert!(diff.modified.is_empty()); assert_eq!(diff.added[0].item, obj(vec![("x", num("1")), ("y", num("2"))])); } other => panic!("{other:?}") }
    }
    #[test]
    fn absorb_array_modify_then_remove_drops_patch() {
        let base = arr(vec![num("1"), num("2")]);
        let mid = arr(vec![num("9"), num("2")]);
        let after = arr(vec![num("2")]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.value { Some(JsonValueDiff::Array { diff }) => { assert_eq!(diff.removed, vec![0]); assert!(diff.modified.is_empty()); } other => panic!("{other:?}") }
    }
    #[test]
    fn absorb_array_associativity() {
        let s0 = arr(vec![num("1"), num("2"), num("3")]);
        let s1 = arr(vec![num("1"), num("9"), num("3")]);
        let s2 = arr(vec![num("9"), num("3"), num("4")]);
        let s3 = arr(vec![num("9"), num("4")]);
        let d1 = JsonDiff::between(&s0, &s1);
        let d2 = JsonDiff::between(&s1, &s2);
        let d3 = JsonDiff::between(&s2, &s3);
        let mut left = d1.clone(); left.absorb(d2.clone()); left.absorb(d3.clone());
        let mut right_tail = d2.clone(); right_tail.absorb(d3.clone());
        let mut right = d1.clone(); right.absorb(right_tail);
        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion

    //#region absorb_law (object/name-keyed)
    #[test]
    fn absorb_object_add_then_setfield_patches_payload() {
        let base = obj(vec![]);
        let mid = obj(vec![("config", obj(vec![]))]);
        let after = obj(vec![("config", obj(vec![("x", num("5"))]))]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
    }
    #[test]
    fn absorb_object_modify_then_remove_drops_patch() {
        let base = obj(vec![("a", num("1")), ("b", num("2"))]);
        let mid = obj(vec![("a", num("9")), ("b", num("2"))]);
        let after = obj(vec![("b", num("2"))]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.value { Some(JsonValueDiff::Object { diff }) => { assert_eq!(diff.removed, vec!["a".to_string()]); assert!(diff.modified.is_empty()); } other => panic!("{other:?}") }
    }
    #[test]
    fn absorb_object_insert_insert_both_survive() {
        let base = obj(vec![("a", num("1"))]);
        let mid = obj(vec![("a", num("1")), ("f", num("2"))]);
        let after = obj(vec![("a", num("1")), ("f", num("2")), ("g", num("3"))]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
    }
    #[test]
    fn absorb_object_insert_then_remove_cancels() {
        let base = obj(vec![("a", num("1"))]);
        let mid = obj(vec![("a", num("1")), ("f", num("2"))]);
        let after = obj(vec![("a", num("1"))]);
        let d1 = JsonDiff::between(&base, &mid);
        let d2 = JsonDiff::between(&mid, &after);
        let mut combined = d1.clone(); combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), base);
        assert!(combined.is_empty());
    }
    #[test]
    fn absorb_object_associativity() {
        let s0 = obj(vec![("a", num("1"))]);
        let s1 = obj(vec![("a", num("1")), ("b", num("2"))]);
        let s2 = obj(vec![("a", num("9")), ("b", num("2"))]);
        let s3 = obj(vec![("b", num("2")), ("c", num("3"))]);
        let d1 = JsonDiff::between(&s0, &s1);
        let d2 = JsonDiff::between(&s1, &s2);
        let d3 = JsonDiff::between(&s2, &s3);
        let mut left = d1.clone(); left.absorb(d2.clone()); left.absorb(d3.clone());
        let mut right_tail = d2.clone(); right_tail.absorb(d3.clone());
        let mut right = d1.clone(); right.absorb(right_tail);
        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion

    //#region field_sweep
    fn sweep_a() -> JsonValue {
        obj(vec![
            ("keepBool", JsonValue::Bool(true)), ("keepNumber", num("1")), ("keepString", str_("base")),
            ("kindChange", num("1")), ("nullToValue", JsonValue::Null), ("removedMember", str_("gone")),
            ("modifiedMember", num("10")), ("nestedArray", arr(vec![num("1"), num("2"), num("3")])),
            ("nestedObject", obj(vec![("inner", str_("x"))])),
        ])
    }
    fn sweep_b() -> JsonValue {
        obj(vec![
            ("keepBool", JsonValue::Bool(false)), ("keepNumber", num("2.5e3")), ("keepString", str_("changed")),
            ("kindChange", str_("now a string")), ("nullToValue", JsonValue::Bool(true)),
            ("modifiedMember", num("99")), ("nestedArray", arr(vec![num("1"), num("20"), num("30"), num("4")])),
            ("nestedObject", obj(vec![("inner", str_("y")), ("extra", JsonValue::Bool(true))])), ("addedMember", str_("new")),
        ])
    }
    #[test]
    fn field_sweep_between_roundtrips() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(JsonDiff::between(&a, &b).apply(&a), b);
        assert_eq!(JsonDiff::between(&b, &a).apply(&b), a);
        assert!(JsonDiff::between(&a, &a).is_empty());
    }
    #[test]
    fn field_sweep_every_field_present() {
        let (a, b) = (sweep_a(), sweep_b());
        let diff = JsonDiff::between(&a, &b);
        let object_diff = match diff.value { Some(JsonValueDiff::Object { diff }) => diff, other => panic!("{other:?}") };
        assert_eq!(object_diff.removed, vec!["removedMember".to_string()]);
        assert_eq!(object_diff.added.len(), 1);
        assert_eq!(object_diff.added[0].key, "addedMember");
        let by_key: HashMap<&str, &JsonValueDiff> = object_diff.modified.iter().map(|m| (m.key.as_str(), &m.diff)).collect();
        for key in ["keepBool", "keepNumber", "keepString", "kindChange", "nullToValue", "modifiedMember", "nestedArray", "nestedObject"] {
            assert!(by_key.contains_key(key), "missing modified entry for {key}");
        }
        assert!(matches!(by_key["kindChange"], JsonValueDiff::Replace { .. }));
        assert!(matches!(by_key["nullToValue"], JsonValueDiff::Replace { .. }));
        assert!(matches!(by_key["keepBool"], JsonValueDiff::Bool { .. }));
        assert!(matches!(by_key["nestedArray"], JsonValueDiff::Array { .. }));
        assert!(matches!(by_key["nestedObject"], JsonValueDiff::Object { .. }));
    }
    //#endregion
}
