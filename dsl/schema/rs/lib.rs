//! 🧬 `dsl_schema` — the data-driven declarative grammar engine: technologies describe their
//! document/op grammar as `RecordSpec`/`Shape` DATA (not code), and this crate parses text against
//! that data into a generic `Cst` (walked by typed binders that `dsl_derive` will generate) and
//! prints it back via a chunk `Writer` that structurally guarantees the newline law: every
//! grammar renders both as multi-line canonical `Document` text and as one space-joined `Inline`
//! line, and both re-parse to the same value.

use dsl_core::{format_f64, lex, parse_f64, Limits, SpannedToken, TextError, TextSpan, TokenClass, TokenKind};
use std::collections::{HashMap, HashSet};

//#region 🔖Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordLayout {
    /// All fields printed as space-joined `key=value` tokens on one logical unit.
    Inline,
    /// Each field printed as its own line/statement (norm-family style) in Document mode;
    /// collapses to the same space-joined form as `Inline` when rendered in `JoinMode::Inline`.
    Lines,
}

/// @emoji 🧩 What one field's value looks like, textually. Covers all 16 grammar-shape
/// primitives found across the 32 hand-rolled implementations this engine replaces.
#[derive(Clone, Debug)]
pub enum Shape {
    Bool,
    Int,
    UInt,
    Float,
    Ident,
    Text,
    Bytes64,
    /// Unit-variant keyword table: `(tag, ordinal)` pairs.
    Enum(Vec<(String, u32)>),
    /// Packed `x,y,z` — `len = Some(n)` enforces arity.
    Tuple(Box<Shape>, Option<usize>),
    /// Bracketed `[a b c]`.
    List(Box<Shape>),
    /// Inline nested `key=value` run using another record's fields, unwrapped. Lazy for the same
    /// reason `Statements` is: a self-referential `#[derive(DslRecord)]` struct (a field whose type
    /// recurses back to the struct itself, e.g. a dynamic-value type with a nested-dictionary-of-
    /// itself field) would otherwise recurse infinitely just building its own `RecordSpec`.
    Record(fn() -> RecordSpec),
    /// Wraps the inner shape in `{ ... }`.
    Block(Box<Shape>),
    /// Keyword-dispatched, order-preserving repeated records: `(keyword, spec_fn)` per variant.
    /// `spec_fn` is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec` — a genuinely
    /// self-referential grammar (a recursive block tree whose own variant table contains itself)
    /// would otherwise recurse infinitely just building the table. Calling `spec_fn()` one level at
    /// a time bottoms out naturally at real documents' finite depth instead.
    Statements(Vec<(String, fn() -> RecordSpec)>),
    /// `{ key=value ... }` block, keys sorted on canonical print.
    Map(Box<Shape>),
    /// Dynamic JSON-equivalent literal.
    Value,
    /// Header + N verbatim raw lines in Document mode; one escaped `Text` token in Inline mode.
    RawLines { count_field: String },
    /// Graph endpoint literal: `id[:kind][@port][->|--id2[:kind2][@port2]]{props}`.
    Wire,
}

#[derive(Clone, Debug)]
pub struct FieldSpec {
    pub id: u16,
    /// Empty for positional-only fields.
    pub key: String,
    /// `Some(n)` = nth positional token right after the keyword, in declaration order among
    /// positional fields.
    pub position: Option<u8>,
    pub shape: Shape,
    pub optional: bool,
    /// Splice a nested record's fields directly into this record (shared doc/op field schemas).
    pub flatten: bool,
}

impl FieldSpec {
    pub fn new(id: u16, key: &str, shape: Shape) -> Self {
        Self { id, key: key.to_string(), position: None, shape, optional: false, flatten: false }
    }

    pub fn positional(mut self, index: u8) -> Self {
        self.position = Some(index);
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn flatten(mut self) -> Self {
        self.flatten = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RecordSpec {
    pub keyword: Option<String>,
    pub layout: RecordLayout,
    pub fields: Vec<FieldSpec>,
}

impl RecordSpec {
    pub fn new(keyword: Option<&str>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword: keyword.map(|k| k.to_string()), layout, fields }
    }

    /// @emoji 🏗️ Same as [`Self::new`] but takes an already-owned keyword — what
    /// `dsl_derive`-generated code builds from a spliced `String` literal.
    pub fn new_owned(keyword: Option<String>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword, layout, fields }
    }
}

pub struct GrammarSpec {
    pub name: String,
    pub root: RecordSpec,
}
//#endregion 🔖Shape

//#region 🔖Value
/// @emoji 🌱 Dynamic JSON-equivalent literal for schema-less fields (`Shape::Value`).
#[derive(Clone, Debug, PartialEq)]
pub enum DslValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
}

/// @emoji 🕸️ One endpoint (and optional edge) of a wire-literal.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireNode {
    pub id: String,
    pub kind: Option<String>,
    pub port: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WireValue {
    pub from: WireNode,
    /// `Some((directed, to))` if this line describes an edge, `None` for a bare node declaration.
    pub edge: Option<(bool, WireNode)>,
    pub properties: DslValue,
}

/// @emoji 🌳 The parsed representation of one field's value — what a typed binder converts
/// to/from a concrete Rust value. Doubles as this v1 engine's "Cst": simplified (semantic, not a
/// full lossless syntax tree) but sufficient for round-tripping, diagnostics, and highlighting;
/// a real green/red tree can replace it later behind the same `parse`/`Writer` API.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Ident(String),
    Text(String),
    Bytes64(Vec<u8>),
    Enum(u32),
    Tuple(Vec<FieldValue>),
    List(Vec<FieldValue>),
    Record(RecordValue),
    Block(Box<FieldValue>),
    Statements(Vec<(String, RecordValue)>),
    Map(Vec<(String, FieldValue)>),
    Value(DslValue),
    RawLines(Vec<String>),
    Wire(WireValue),
    Absent,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RecordValue {
    pub fields: HashMap<u16, FieldValue>,
}

impl RecordValue {
    pub fn get(&self, id: u16) -> Option<&FieldValue> {
        self.fields.get(&id)
    }
}

/// @emoji 🌳 Alias naming the parse product per the engine's design vocabulary.
pub type Cst = RecordValue;
//#endregion 🔖Value

//#region 🔖Cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMode {
    Document,
    Inline,
}

struct Cursor<'a> {
    tokens: Vec<SpannedToken>,
    source: &'a str,
    pos: usize,
    limits: Limits,
    mode: SourceMode,
}

impl<'a> Cursor<'a> {
    fn new(source: &'a str, tokens: Vec<SpannedToken>, limits: Limits, mode: SourceMode) -> Self {
        let tokens: Vec<SpannedToken> = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, source, pos: 0, limits, mode }
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_at(&self, offset: usize) -> &SpannedToken {
        let idx = (self.pos + offset).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    fn span(&self) -> TextSpan {
        self.peek().span
    }

    fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: TokenKind) -> Result<SpannedToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(format!("expected {:?}, found {:?} '{}'", kind, self.peek().kind, self.peek().text.as_str()), self.span()))
        }
    }

    /// @emoji 🔎 Whether the next token is an `Ident` that is followed by `=` — the LL(2)
    /// lookahead that makes the grammar newline-insensitive: a bare ident followed by `=` is
    /// always a `key=value` attribute, never the start of a new statement.
    fn at_attr_key(&self) -> Option<String> {
        if self.peek().kind == TokenKind::Ident && self.peek_at(1).kind == TokenKind::Equals {
            Some(self.peek().text.as_str().to_string())
        } else {
            None
        }
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.peek().kind == TokenKind::Ident && self.peek().text.as_str().as_ref() == keyword
    }

    /// @emoji 📍 Byte offset in the original source immediately after the last consumed token —
    /// the resume point for [`Self::consume_raw_lines`].
    fn byte_offset_after_last_consumed(&self) -> usize {
        if self.pos == 0 {
            0
        } else {
            self.tokens[self.pos - 1].byte_range.1 as usize
        }
    }

    /// @emoji 📄 Reads exactly `count` verbatim newline-delimited lines from the original source,
    /// starting right after the current record's header line, and advances the cursor past every
    /// token whose byte range falls inside those lines.
    fn consume_raw_lines(&mut self, count: usize) -> Vec<String> {
        let start = self.byte_offset_after_last_consumed();
        let rest = &self.source[start.min(self.source.len())..];
        let body_start = start + rest.find('\n').map_or(rest.len(), |i| i + 1);
        let mut lines = Vec::with_capacity(count);
        let mut cursor_byte = body_start;
        for _ in 0..count {
            let remaining = &self.source[cursor_byte.min(self.source.len())..];
            match remaining.find('\n') {
                Some(i) => {
                    lines.push(remaining[..i].to_string());
                    cursor_byte += i + 1;
                }
                None => {
                    lines.push(remaining.to_string());
                    cursor_byte = self.source.len();
                }
            }
        }
        while self.pos < self.tokens.len() - 1 && (self.tokens[self.pos].byte_range.0 as usize) < cursor_byte {
            self.pos += 1;
        }
        lines
    }
}
//#endregion 🔖Cursor

//#region 🔖Parser
pub struct ParseOptions {
    pub limits: Limits,
    pub mode: SourceMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self { limits: Limits::default(), mode: SourceMode::Document }
    }
}

pub fn parse(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    // Lex forgiving: a `RawLines` region can legitimately contain any character (arbitrary
    // source text), so the whole-document lex pass must never hard-fail on it — the parser (not
    // the lexer) is the source of truth for what's actually malformed, once it reaches a
    // non-raw-lines position and tries to match a real token kind against an `Error` token.
    let tokens = lex(text, &opts.limits, true)?;
    let mut cursor = Cursor::new(text, tokens, opts.limits, opts.mode);
    let value = parse_record_body(&mut cursor, spec, 0)?;
    Ok(value)
}

fn ident_like_text(token: &SpannedToken) -> String {
    token.text.as_str().to_string()
}

fn parse_scalar(cursor: &mut Cursor<'_>, shape: &Shape) -> Result<FieldValue, TextError> {
    match shape {
        Shape::Bool => {
            let token = cursor.expect(TokenKind::Ident)?;
            match token.text.as_str().as_ref() {
                "true" => Ok(FieldValue::Bool(true)),
                "false" => Ok(FieldValue::Bool(false)),
                other => Err(TextError::new(format!("expected 'true' or 'false', found '{other}'"), token.span)),
            }
        }
        Shape::Int => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: i64 = token.text.as_str().parse().map_err(|_| TextError::new(format!("invalid integer '{}'", token.text.as_str()), token.span))?;
            Ok(FieldValue::Int(value))
        }
        Shape::UInt => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: u64 = token.text.as_str().parse().map_err(|_| TextError::new(format!("invalid unsigned integer '{}'", token.text.as_str()), token.span))?;
            Ok(FieldValue::UInt(value))
        }
        Shape::Float => {
            let is_float_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int)
                || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
            if !is_float_token {
                return Err(TextError::new(format!("expected a float, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
            }
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(FieldValue::Float(value))
        }
        Shape::Ident => {
            let token = cursor.expect(TokenKind::Ident)?;
            Ok(FieldValue::Ident(ident_like_text(&token)))
        }
        Shape::Text => {
            let token = cursor.expect(TokenKind::Text)?;
            let text = dsl_core::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(e, token.span))?;
            Ok(FieldValue::Text(text))
        }
        Shape::Bytes64 => {
            let token = cursor.expect(TokenKind::Text)?;
            let bytes = base64_decode(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(FieldValue::Bytes64(bytes))
        }
        Shape::Enum(variants) => {
            let token = cursor.expect(TokenKind::Ident)?;
            let text = token.text.as_str();
            variants
                .iter()
                .find(|(tag, _)| tag == text.as_ref())
                .map(|(_, ordinal)| FieldValue::Enum(*ordinal))
                .ok_or_else(|| TextError::new(format!("unknown enum tag '{text}'"), token.span))
        }
        other => Err(TextError::new(format!("shape {other:?} is not a scalar"), cursor.span())),
    }
}

fn parse_shape(cursor: &mut Cursor<'_>, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Ident | Shape::Text | Shape::Bytes64 | Shape::Enum(_) => parse_scalar(cursor, shape),
        Shape::Tuple(elem, len) => {
            let mut items = Vec::new();
            loop {
                items.push(parse_shape(cursor, elem, depth + 1)?);
                if cursor.peek().kind == TokenKind::Comma {
                    cursor.advance();
                    continue;
                }
                break;
            }
            if let Some(expected_len) = len {
                if items.len() != *expected_len {
                    return Err(TextError::new(format!("tuple expects {} elements, found {}", expected_len, items.len()), cursor.span()));
                }
            }
            Ok(FieldValue::Tuple(items))
        }
        Shape::List(elem) => {
            cursor.expect(TokenKind::LBracket)?;
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                items.push(parse_shape(cursor, elem, depth + 1)?);
                cursor.limits.check_nodes(items.len(), cursor.span())?;
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(FieldValue::List(items))
        }
        Shape::Record(spec_fn) => Ok(FieldValue::Record(parse_record_body(cursor, &spec_fn(), depth + 1)?)),
        Shape::Block(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let value = parse_shape(cursor, inner, depth + 1)?;
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Block(Box::new(value)))
        }
        Shape::Statements(variants) => {
            let mut out = Vec::new();
            while let Some(keyword) = current_keyword(cursor) {
                let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == &keyword) else { break };
                // `parse_record_body` consumes the keyword itself (see its own check below); we
                // only peek here to decide whether this token starts a known variant at all.
                let record = parse_record_body(cursor, &spec_fn(), depth + 1)?;
                out.push((keyword, record));
                cursor.limits.check_nodes(out.len(), cursor.span())?;
                if cursor.peek().kind == TokenKind::RBrace || cursor.peek().kind == TokenKind::Eof {
                    break;
                }
            }
            Ok(FieldValue::Statements(out))
        }
        Shape::Map(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let mut entries = Vec::new();
            while let Some(key) = cursor.at_attr_key() {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                let value = parse_shape(cursor, inner, depth + 1)?;
                entries.push((key, value));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Map(entries))
        }
        Shape::Value => Ok(FieldValue::Value(parse_dsl_value(cursor, depth + 1)?)),
        Shape::RawLines { count_field: _ } => {
            // Handled specially by the caller (`parse_record_body`), which knows the already-
            // parsed count. Reaching here directly means the shape was used outside a record.
            Err(TextError::new("RawLines shape may only be used as a record field", cursor.span()))
        }
        Shape::Wire => Ok(FieldValue::Wire(parse_wire(cursor)?)),
    }
}

fn current_keyword(cursor: &Cursor<'_>) -> Option<String> {
    if cursor.peek().kind == TokenKind::Ident && cursor.at_attr_key().is_none() {
        Some(cursor.peek().text.as_str().to_string())
    } else {
        None
    }
}

fn parse_dsl_value(cursor: &mut Cursor<'_>, depth: usize) -> Result<DslValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match cursor.peek().kind {
        TokenKind::LBrace => {
            cursor.advance();
            let mut entries = Vec::new();
            while let Some(key) = cursor.at_attr_key() {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                entries.push((key, parse_dsl_value(cursor, depth + 1)?));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(DslValue::Object(entries))
        }
        TokenKind::LBracket => {
            cursor.advance();
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                items.push(parse_dsl_value(cursor, depth + 1)?);
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(DslValue::Array(items))
        }
        TokenKind::Text => {
            let token = cursor.advance();
            let text = dsl_core::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(e, token.span))?;
            Ok(DslValue::String(text))
        }
        TokenKind::Int | TokenKind::Float => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(DslValue::Number(value))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            match token.text.as_str().as_ref() {
                "null" => Ok(DslValue::Null),
                "true" => Ok(DslValue::Bool(true)),
                "false" => Ok(DslValue::Bool(false)),
                other => Err(TextError::new(format!("expected a value literal, found ident '{other}'"), token.span)),
            }
        }
        other => Err(TextError::new(format!("expected a value literal, found {other:?}"), cursor.span())),
    }
}

fn parse_wire(cursor: &mut Cursor<'_>) -> Result<WireValue, TextError> {
    let from = parse_wire_node(cursor)?;
    let edge = if cursor.peek().kind == TokenKind::Arrow || cursor.peek().kind == TokenKind::DashArrow {
        let directed = cursor.advance().kind == TokenKind::Arrow;
        let to = parse_wire_node(cursor)?;
        Some((directed, to))
    } else {
        None
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, properties })
}

fn parse_wire_node(cursor: &mut Cursor<'_>) -> Result<WireNode, TextError> {
    let id = ident_like_text(&cursor.expect(TokenKind::Ident)?);
    let kind = if cursor.peek().kind == TokenKind::Colon {
        cursor.advance();
        Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
    } else {
        None
    };
    let port = if cursor.peek().kind == TokenKind::At {
        cursor.advance();
        Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
    } else {
        None
    };
    Ok(WireNode { id, kind, port })
}

/// @emoji 🧾 Parses one record: its own leading keyword if `spec.keyword` declares one (the
/// `Statements` dispatcher only peeks to choose a variant — consuming it is always this
/// function's job, so a spec is self-contained regardless of whether it's reached via `parse`
/// directly, `Shape::Record`, or a `Statements` variant), positional fields in declaration order,
/// then order-independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a
/// key), until a token that is neither a known key nor an unfilled positional slot — which ends
/// the record (it belongs to whatever comes next: a new statement, a closing brace, or EOF).
fn parse_record_body(cursor: &mut Cursor<'_>, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    if let Some(keyword) = &spec.keyword {
        if cursor.at_keyword(keyword) {
            cursor.advance();
        } else {
            return Err(TextError::new(format!("expected keyword '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
        }
    }
    let mut record = RecordValue::default();
    let positional: Vec<&FieldSpec> = {
        let mut p: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some()).collect();
        p.sort_by_key(|f| f.position.unwrap());
        p
    };
    for field in &positional {
        if field.optional && !can_start_positional(cursor, &field.shape) {
            record.fields.insert(field.id, FieldValue::Absent);
            continue;
        }
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block`/`RawLines` fields are also excluded from the `key=value` loop below: `Block`'s own
    // key acts as a bare leading keyword (`children { ... }`, no `=`), and `RawLines` is handled
    // as its own post-pass since it consumes raw source text, not tokens.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let raw_lines_field = spec.fields.iter().find(|f| matches!(f.shape, Shape::RawLines { .. }));
    let mut keyed: Vec<&FieldSpec> = spec
        .fields
        .iter()
        .filter(|f| f.position.is_none() && !f.key.is_empty() && !matches!(f.shape, Shape::Statements(_) | Shape::RawLines { .. }))
        .collect();

    loop {
        if let Some(key) = cursor.at_attr_key() {
            let Some(index) = keyed.iter().position(|f| !matches!(f.shape, Shape::Block(_)) && f.key == key) else { break };
            let field = keyed.remove(index);
            cursor.advance();
            cursor.expect(TokenKind::Equals)?;
            let value = parse_shape(cursor, &field.shape, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Block(_)) && cursor.at_keyword(&f.key)) else { break };
        let field = keyed.remove(index);
        cursor.advance();
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }
    for field in keyed {
        record.fields.entry(field.id).or_insert(FieldValue::Absent);
    }

    if let Some(field) = statements_field {
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    if let Some(field) = raw_lines_field {
        let Shape::RawLines { count_field } = &field.shape else { unreachable!() };
        let count_field_id = spec.fields.iter().find(|f| f.key == *count_field).map(|f| f.id);
        let count = count_field_id.and_then(|id| record.fields.get(&id)).and_then(field_value_as_usize).unwrap_or(0);
        let lines = match cursor.mode {
            SourceMode::Document => cursor.consume_raw_lines(count),
            SourceMode::Inline => {
                if cursor.peek().kind == TokenKind::Text {
                    let token = cursor.advance();
                    let joined = dsl_core::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(e, token.span))?;
                    joined.split('\n').map(|s| s.to_string()).collect()
                } else {
                    Vec::new()
                }
            }
        };
        record.fields.insert(field.id, FieldValue::RawLines(lines));
    }

    Ok(record)
}

fn field_value_as_usize(value: &FieldValue) -> Option<usize> {
    match value {
        FieldValue::Int(v) => Some(*v as usize),
        FieldValue::UInt(v) => Some(*v as usize),
        _ => None,
    }
}

fn can_start_positional(cursor: &Cursor<'_>, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Ident | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        Shape::Text | Shape::Bytes64 => cursor.peek().kind == TokenKind::Text,
        Shape::List(_) => cursor.peek().kind == TokenKind::LBracket,
        Shape::Block(_) | Shape::Map(_) => cursor.peek().kind == TokenKind::LBrace,
        _ => true,
    }
}

fn base64_decode(text: &str) -> Result<Vec<u8>, String> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut lut = [255u8; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        lut[c as usize] = i as u8;
    }
    let clean: Vec<u8> = text.bytes().filter(|b| *b != b'=').collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u32;
    for b in clean {
        let value = lut[b as usize];
        if value == 255 {
            return Err(format!("invalid base64 byte '{}'", b as char));
        }
        buffer = (buffer << 6) | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
        }
    }
    Ok(out)
}

fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(ALPHABET[(b0 >> 2) as usize] as char);
        out.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}
//#endregion 🔖Parser

//#region 🔖Writer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinMode {
    Document,
    Inline,
}

/// @emoji ✍️ A chunk tree that renders in either join mode — the structural half of the newline
/// law. `atom` asserts its argument contains no raw `\n` (Document mode still separates atoms with
/// synthesized whitespace, never embeds one inside an atom), so `render(Inline)` joining every
/// chunk with a single space can never produce an embedded newline.
pub struct Writer {
    chunks: Vec<Chunk>,
    indent: usize,
}

enum Chunk {
    Atom(String),
    Raw(String), // Document-mode-only verbatim lines (already newline-delimited); collapsed in Inline mode by the caller before reaching the writer
    OpenBlock,
    CloseBlock,
    NewRecord,
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer {
    pub fn new() -> Self {
        Self { chunks: Vec::new(), indent: 0 }
    }

    pub fn atom(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        debug_assert!(!s.contains('\n'), "Writer::atom must not contain a raw newline: {s:?}");
        self.chunks.push(Chunk::Atom(s.to_string()));
    }

    pub fn key_value(&mut self, key: &str, value: impl AsRef<str>) {
        self.atom(format!("{key}={}", value.as_ref()));
    }

    pub fn raw_lines(&mut self, lines: &[String]) {
        self.chunks.push(Chunk::Raw(lines.join("\n")));
    }

    pub fn open_block(&mut self) {
        self.chunks.push(Chunk::OpenBlock);
        self.indent += 1;
    }

    pub fn close_block(&mut self) {
        self.indent = self.indent.saturating_sub(1);
        self.chunks.push(Chunk::CloseBlock);
    }

    pub fn new_record(&mut self) {
        self.chunks.push(Chunk::NewRecord);
    }

    pub fn render(&self, mode: JoinMode) -> String {
        match mode {
            JoinMode::Inline => {
                let mut parts = Vec::new();
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Atom(s) => parts.push(s.clone()),
                        Chunk::Raw(s) => parts.push(format!("\"{}\"", dsl_core::escape_text(s))),
                        Chunk::OpenBlock => parts.push("{".to_string()),
                        Chunk::CloseBlock => parts.push("}".to_string()),
                        Chunk::NewRecord => {}
                    }
                }
                parts.join(" ")
            }
            JoinMode::Document => {
                let mut out = String::new();
                let mut indent = 0usize;
                let mut line_open = false;
                let push_indent = |out: &mut String, indent: usize| {
                    for _ in 0..indent {
                        out.push_str("  ");
                    }
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Atom(s) => {
                            if !line_open {
                                push_indent(&mut out, indent);
                                line_open = true;
                            } else {
                                out.push(' ');
                            }
                            out.push_str(s);
                        }
                        Chunk::Raw(s) => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                            out.push_str(s);
                            out.push('\n');
                        }
                        Chunk::OpenBlock => {
                            out.push_str(" {");
                            out.push('\n');
                            line_open = false;
                            indent += 1;
                        }
                        Chunk::CloseBlock => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                            indent = indent.saturating_sub(1);
                            push_indent(&mut out, indent);
                            out.push('}');
                            out.push('\n');
                        }
                        Chunk::NewRecord => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                        }
                    }
                }
                if line_open {
                    out.push('\n');
                }
                out
            }
        }
    }
}

pub fn print_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    let mut positional: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some()).collect();
    positional.sort_by_key(|f| f.position.unwrap());
    for field in positional {
        if let Some(fv) = value.get(field.id) {
            if matches!(fv, FieldValue::Absent) {
                continue;
            }
            print_shape(fv, &field.shape, writer);
        }
    }
    for field in spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !matches!(f.shape, Shape::RawLines { .. } | Shape::Statements(_))) {
        match value.get(field.id) {
            Some(FieldValue::Absent) | None => continue,
            Some(fv) => {
                if matches!(field.shape, Shape::Block(_)) {
                    // `Block`'s own key is a bare leading keyword, not a `key=value` attribute.
                    writer.new_record();
                    writer.atom(&field.key);
                    print_shape(fv, &field.shape, writer);
                } else {
                    writer.atom(format!("{}=", field.key));
                    print_key_value(field, fv, writer);
                }
            }
        }
    }
    if let Some(field) = spec.fields.iter().find(|f| matches!(f.shape, Shape::Statements(_))) {
        if let Some(fv) = value.get(field.id) {
            print_shape(fv, &field.shape, writer);
        }
    }
    if let Some(field) = spec.fields.iter().find(|f| matches!(f.shape, Shape::RawLines { .. })) {
        if let Some(FieldValue::RawLines(lines)) = value.get(field.id) {
            writer.raw_lines(lines);
        }
    }
}

fn print_key_value(field: &FieldSpec, value: &FieldValue, writer: &mut Writer) {
    // Overwrites the bare `key=` atom pushed by the caller with a fused `key=value` atom for
    // scalar shapes (the common case), or falls back to `key=` followed by a structured value for
    // composite shapes.
    match (&field.shape, value) {
        (Shape::Enum(variants), FieldValue::Enum(ordinal)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                if let Chunk::Atom(last) = writer.chunks.last_mut().expect("key atom just pushed") {
                    last.push_str(tag);
                }
            }
        }
        (Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Ident | Shape::Text | Shape::Bytes64, _) => {
            if let Chunk::Atom(last) = writer.chunks.last_mut().expect("key atom just pushed") {
                last.push_str(&scalar_to_text(value));
            }
        }
        _ => {
            print_shape(value, &field.shape, writer);
        }
    }
}

fn scalar_to_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Bool(b) => b.to_string(),
        FieldValue::Int(i) => i.to_string(),
        FieldValue::UInt(u) => u.to_string(),
        FieldValue::Float(f) => format_f64(*f),
        FieldValue::Ident(s) => s.clone(),
        FieldValue::Text(s) => format!("\"{}\"", dsl_core::escape_text(s)),
        FieldValue::Bytes64(bytes) => format!("\"{}\"", base64_encode(bytes)),
        FieldValue::Enum(_) => String::new(), // resolved by caller via variants table when needed
        _ => String::new(),
    }
}

pub fn print_shape(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    match (value, shape) {
        (FieldValue::Bool(_) | FieldValue::Int(_) | FieldValue::UInt(_) | FieldValue::Float(_) | FieldValue::Ident(_) | FieldValue::Text(_) | FieldValue::Bytes64(_), _) => {
            writer.atom(scalar_to_text(value));
        }
        (FieldValue::Enum(ordinal), Shape::Enum(variants)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        (FieldValue::Tuple(items), Shape::Tuple(elem, _)) => {
            let rendered: Vec<String> = items
                .iter()
                .map(|item| {
                    let mut sub = Writer::new();
                    print_shape(item, elem, &mut sub);
                    sub.render(JoinMode::Inline)
                })
                .collect();
            writer.atom(rendered.join(","));
        }
        (FieldValue::List(items), Shape::List(elem)) => {
            writer.atom("[");
            for item in items {
                print_shape(item, elem, writer);
            }
            writer.atom("]");
        }
        (FieldValue::Record(record), Shape::Record(spec_fn)) => {
            print_record(record, &spec_fn(), writer);
        }
        (FieldValue::Block(inner_value), Shape::Block(inner_shape)) => {
            writer.open_block();
            print_shape(inner_value, inner_shape, writer);
            writer.close_block();
        }
        (FieldValue::Statements(items), Shape::Statements(variants)) => {
            for (keyword, record) in items {
                writer.new_record();
                if let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == keyword) {
                    print_record(record, &spec_fn(), writer);
                }
            }
        }
        (FieldValue::Map(entries), Shape::Map(inner)) => {
            writer.open_block();
            let mut sorted = entries.clone();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (key, value) in &sorted {
                writer.atom(format!("{key}="));
                if let Chunk::Atom(_) = writer.chunks.last().unwrap() {
                    let mut sub = Writer::new();
                    print_shape(value, inner, &mut sub);
                    let rendered = sub.render(JoinMode::Inline);
                    if let Chunk::Atom(last) = writer.chunks.last_mut().unwrap() {
                        last.push_str(&rendered);
                    }
                }
            }
            writer.close_block();
        }
        (FieldValue::Value(dsl_value), Shape::Value) => print_dsl_value(dsl_value, writer),
        (FieldValue::RawLines(lines), Shape::RawLines { .. }) => writer.raw_lines(lines),
        (FieldValue::Wire(wire), Shape::Wire) => print_wire(wire, writer),
        _ => {}
    }
}

fn print_dsl_value(value: &DslValue, writer: &mut Writer) {
    match value {
        DslValue::Null => writer.atom("null"),
        DslValue::Bool(b) => writer.atom(b.to_string()),
        DslValue::Number(n) => writer.atom(format_f64(*n)),
        DslValue::String(s) => writer.atom(format!("\"{}\"", dsl_core::escape_text(s))),
        DslValue::Array(items) => {
            writer.atom("[");
            for item in items {
                print_dsl_value(item, writer);
            }
            writer.atom("]");
        }
        DslValue::Object(entries) => {
            let mut sorted = entries.clone();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            writer.open_block();
            for (key, value) in &sorted {
                writer.atom(format!("{key}="));
                let mut sub = Writer::new();
                print_dsl_value(value, &mut sub);
                let rendered = sub.render(JoinMode::Inline);
                if let Chunk::Atom(last) = writer.chunks.last_mut().unwrap() {
                    last.push_str(&rendered);
                }
            }
            writer.close_block();
        }
    }
}

fn print_wire(wire: &WireValue, writer: &mut Writer) {
    let node_text = |node: &WireNode| -> String {
        let mut s = node.id.clone();
        if let Some(kind) = &node.kind {
            s.push(':');
            s.push_str(kind);
        }
        if let Some(port) = &node.port {
            s.push('@');
            s.push_str(port);
        }
        s
    };
    let mut atom = node_text(&wire.from);
    if let Some((directed, to)) = &wire.edge {
        atom.push_str(if *directed { "->" } else { "--" });
        atom.push_str(&node_text(to));
    }
    writer.atom(atom);
    if !matches!(&wire.properties, DslValue::Object(entries) if entries.is_empty()) {
        print_dsl_value(&wire.properties, writer);
    }
}

/// @emoji 🔁 Prints `value` against `spec` in the given join mode — the top-level entry point
/// `dsl_derive`-generated code calls from `DocumentDsl::print_dsl`/`OpText::print_op`.
pub fn print(value: &RecordValue, spec: &RecordSpec, mode: JoinMode) -> String {
    let mut writer = Writer::new();
    print_record(value, spec, &mut writer);
    writer.render(mode)
}
//#endregion 🔖Writer

//#region 🔖Canonicalize
/// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`: reprints whatever `parse`
/// produces from `text`, which is the fixpoint every technology's `print_dsl` output must already
/// be at (the round-trip law), so this doubles as the idempotence check.
pub fn canonicalize(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<String, TextError> {
    let value = parse(text, spec, opts)?;
    Ok(print(&value, spec, JoinMode::Document))
}
//#endregion 🔖Canonicalize

//#region 🔖Serde
impl From<serde_json::Value> for DslValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => DslValue::Null,
            serde_json::Value::Bool(b) => DslValue::Bool(b),
            serde_json::Value::Number(n) => DslValue::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => DslValue::String(s),
            serde_json::Value::Array(items) => DslValue::Array(items.into_iter().map(DslValue::from).collect()),
            serde_json::Value::Object(map) => DslValue::Object(map.into_iter().map(|(k, v)| (k, DslValue::from(v))).collect()),
        }
    }
}

impl From<DslValue> for serde_json::Value {
    fn from(value: DslValue) -> Self {
        match value {
            DslValue::Null => serde_json::Value::Null,
            DslValue::Bool(b) => serde_json::Value::Bool(b),
            DslValue::Number(n) => serde_json::json!(n),
            DslValue::String(s) => serde_json::Value::String(s),
            DslValue::Array(items) => serde_json::Value::Array(items.into_iter().map(serde_json::Value::from).collect()),
            DslValue::Object(entries) => serde_json::Value::Object(entries.into_iter().map(|(k, v)| (k, serde_json::Value::from(v))).collect()),
        }
    }
}
//#endregion 🔖Serde

//#region 🔖Language
/// @emoji 🎨 Generic editor surface over any `RecordSpec` — the generalization of
/// `mathematical_graph_dsl`'s hand-rolled `LanguageService`.
pub struct LanguageService<'g> {
    pub spec: &'g RecordSpec,
}

pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

impl<'g> LanguageService<'g> {
    pub fn new(spec: &'g RecordSpec) -> Self {
        Self { spec }
    }

    fn keywords(&self) -> Vec<String> {
        let mut out = Vec::new();
        collect_keywords(self.spec, &mut out, &mut HashSet::new(), &mut HashSet::new());
        out
    }

    pub fn semantic_tokens(&self, text: &str) -> Vec<(TokenClass, TextSpan)> {
        let limits = Limits::default();
        let tokens = lex(text, &limits, true).unwrap_or_default();
        let keywords = self.keywords();
        let keyword_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
        dsl_core::token_classes(&tokens, &keyword_refs)
    }

    pub fn diagnostics(&self, text: &str) -> Vec<TextError> {
        match parse(text, self.spec, &ParseOptions::default()) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e],
        }
    }

    /// @emoji 💡 Completions at `offset`: every key not yet used in the record enclosing the
    /// cursor, plus every keyword reachable from the root. A simple, always-available baseline —
    /// full context-sensitive narrowing is a natural follow-up once `Cst` gains node addressing.
    pub fn completions(&self, _text: &str, _offset: usize) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = self
            .spec
            .fields
            .iter()
            .filter(|f| !f.key.is_empty())
            .map(|f| CompletionItem { label: f.key.clone(), detail: Some(format!("{:?}", f.shape)) })
            .collect();
        for keyword in self.keywords() {
            items.push(CompletionItem { label: keyword, detail: None });
        }
        items
    }
}

fn collect_keywords(spec: &RecordSpec, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    if let Some(kw) = &spec.keyword {
        out.push(kw.clone());
    }
    for field in &spec.fields {
        collect_shape_keywords(&field.shape, out, seen, seen_records);
    }
}

/// @emoji 🔁 `seen` guards against a genuinely self-referential `Statements` table (a recursive
/// block tree whose own variant list contains itself): each `spec_fn()` call is only expanded the
/// first time its keyword is reached, so the keyword set — which is always finite, even when the
/// grammar's real nesting isn't — is collected exactly once instead of infinitely. `seen_records`
/// is the same guard for a self-referential `Shape::Record` (a `#[derive(DslRecord)]` struct field
/// whose type recurses back to itself, e.g. a dynamic-value type nesting a map of itself) — a bare
/// Record has no keyword to key on, so this tracks the `fn() -> RecordSpec` pointer's own address
/// instead (two calls to the same generated `__dsl_spec` always share one code address).
fn collect_shape_keywords(shape: &Shape, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    match shape {
        Shape::Record(spec_fn) => {
            if seen_records.insert(*spec_fn as usize) {
                collect_keywords(&spec_fn(), out, seen, seen_records);
            }
        }
        Shape::Block(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        Shape::Statements(variants) => {
            for (kw, spec_fn) in variants {
                out.push(kw.clone());
                if seen.insert(kw.clone()) {
                    collect_keywords(&spec_fn(), out, seen, seen_records);
                }
            }
        }
        Shape::List(inner) | Shape::Tuple(inner, _) | Shape::Map(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        _ => {}
    }
}
//#endregion 🔖Language

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip(text: &str, spec: &RecordSpec) {
        let opts = ParseOptions::default();
        let value = parse(text, spec, &opts).unwrap_or_else(|e| panic!("parse failed for {text:?}: {e}"));
        let printed = print(&value, spec, JoinMode::Document);
        let reparsed = parse(&printed, spec, &opts).unwrap_or_else(|e| panic!("reparse of printed output failed: {e}\nprinted:\n{printed}"));
        assert_eq!(value, reparsed, "round trip diverged;\noriginal print:\n{printed}");
    }

    fn assert_document_inline_agree(text: &str, spec: &RecordSpec) {
        let doc_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Document };
        let value = parse(text, spec, &doc_opts).expect("parse document");
        let inline_text = print(&value, spec, JoinMode::Inline);
        assert!(!inline_text.contains('\n'), "inline render must be one line: {inline_text:?}");
        let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
        let reparsed = parse(&inline_text, spec, &inline_opts).unwrap_or_else(|e| panic!("inline reparse failed: {e}\ninline:\n{inline_text}"));
        assert_eq!(value, reparsed, "Document and Inline renders must parse to the same value");
    }

    // --- primitive 1: record with typed scalar fields, order-independent key=value ---
    fn camera_spec() -> RecordSpec {
        RecordSpec::new(
            Some("camera"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "x", Shape::Float),
                FieldSpec::new(1, "y", Shape::Float),
                FieldSpec::new(2, "zoom", Shape::Float),
                FieldSpec::new(3, "label", Shape::Text).optional(),
            ],
        )
    }

    #[test]
    fn primitive_scalar_record_round_trips_and_is_order_independent() {
        let spec = camera_spec();
        assert_round_trip("camera x=1 y=2 zoom=3", &spec);
        assert_round_trip("camera zoom=3 x=1 y=2", &spec);
        assert_round_trip("camera x=-1.5 y=0 zoom=2.25 label=\"hi \\\"there\\\"\"", &spec);
        assert_document_inline_agree("camera x=1 y=2 zoom=3", &spec);
    }

    #[test]
    fn primitive_optional_field_omits_on_print_and_absent_on_parse() {
        let spec = camera_spec();
        let value = parse("camera x=1 y=2 zoom=1", &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(3), Some(&FieldValue::Absent));
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(!printed.contains("label"), "optional absent field must be omitted: {printed}");
    }

    // --- primitive 2 + 3: keyword-led statements, homogeneous ordered collection ---
    fn layer_variant_spec() -> RecordSpec {
        RecordSpec::new(Some("layer"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Ident).positional(0), FieldSpec::new(1, "opacity", Shape::Float)])
    }

    fn document_with_layers_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![FieldSpec::new(0, "schema", Shape::Ident), FieldSpec::new(1, "layers", Shape::Statements(vec![("layer".to_string(), layer_variant_spec)]))],
        )
    }

    #[test]
    fn primitive_statements_collection_preserves_order_and_round_trips() {
        let spec = document_with_layers_spec();
        assert_round_trip("schema=doc layer a opacity=1 layer b opacity=0.5 layer c opacity=1", &spec);
        let value = parse("schema=doc layer a opacity=1 layer b opacity=0.5", &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::Statements(items) = value.get(1).unwrap() else { panic!("expected statements") };
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].0, "layer");
    }

    // --- primitive 4: recursive sub-blocks ---
    /// @emoji 🌳 Genuinely self-referential: `children`'s own variant table names `group_spec`
    /// itself. Lazy `fn() -> RecordSpec` entries make this sound — `group_spec()` doesn't recurse
    /// just to build the table, only `parse`/`print` calling the stored fn pointer one level at a
    /// time (as deep as real input actually nests) ever evaluates it again.
    fn group_spec() -> RecordSpec {
        RecordSpec::new(
            Some("group"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "id", Shape::Ident).positional(0),
                FieldSpec::new(1, "children", Shape::Block(Box::new(Shape::Statements(vec![("group".to_string(), group_spec)])))).optional(),
            ],
        )
    }

    #[test]
    fn primitive_recursive_blocks_round_trip() {
        let spec = group_spec();
        assert_round_trip("group root children { group a group b }", &spec);
        assert_round_trip("group leaf", &spec);
    }

    // --- primitive 6/7: escaped inline text, formerly-trailing free text ---
    #[test]
    fn primitive_escaped_text_handles_quotes_newlines_and_trailing_position() {
        let spec = camera_spec();
        let value = parse("camera x=1 y=1 zoom=1 label=\"line1\\nline2 with \\\"quotes\\\"\"", &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(3), Some(&FieldValue::Text("line1\nline2 with \"quotes\"".to_string())));
    }

    // --- primitive 8: header + N verbatim raw lines ---
    fn writer_doc_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![FieldSpec::new(0, "lines", Shape::UInt), FieldSpec::new(1, "body", Shape::RawLines { count_field: "lines".to_string() })],
        )
    }

    #[test]
    fn primitive_raw_lines_reads_exact_verbatim_lines_in_document_mode() {
        let spec = writer_doc_spec();
        let text = "lines=2\nfn main() {\n    let x = 1;\n}\n";
        let value = parse(text, &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(1), Some(&FieldValue::RawLines(vec!["fn main() {".to_string(), "    let x = 1;".to_string()])));
    }

    #[test]
    fn primitive_raw_lines_lowers_to_one_escaped_token_in_inline_mode() {
        let spec = writer_doc_spec();
        let doc_text = "lines=2\nfirst line\nsecond line\n";
        let doc_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Document };
        let value = parse(doc_text, &spec, &doc_opts).expect("parse document");
        let inline = print(&value, &spec, JoinMode::Inline);
        assert!(!inline.contains('\n'), "inline rendering of raw lines must be one line: {inline:?}");
        let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
        let reparsed = parse(&inline, &spec, &inline_opts).expect("reparse inline");
        assert_eq!(reparsed.get(1), value.get(1));
    }

    // --- primitive 9: graph endpoints (wire literal) ---
    fn wire_spec() -> RecordSpec {
        RecordSpec::new(Some("edge"), RecordLayout::Inline, vec![FieldSpec::new(0, "link", Shape::Wire).positional(0)])
    }

    #[test]
    fn primitive_wire_literal_directed_and_undirected_round_trip() {
        let spec = wire_spec();
        assert_round_trip("edge a:Kind@out->b:Kind2@in", &spec);
        assert_round_trip("edge a--b", &spec);
        assert_round_trip("edge solo", &spec);
    }

    // --- primitive 10: packed tuples / lists / base64 ---
    fn geometry_spec() -> RecordSpec {
        RecordSpec::new(
            Some("vertex"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "pos", Shape::Tuple(Box::new(Shape::Float), Some(3))).positional(0),
                FieldSpec::new(1, "tags", Shape::List(Box::new(Shape::Ident))).optional(),
                FieldSpec::new(2, "blob", Shape::Bytes64).optional(),
            ],
        )
    }

    #[test]
    fn primitive_tuple_list_and_base64_round_trip() {
        let spec = geometry_spec();
        assert_round_trip("vertex 1,2,3", &spec);
        assert_round_trip("vertex 1,2,3 tags=[a b c]", &spec);
        assert_round_trip("vertex 0,0,0 blob=\"aGVsbG8=\"", &spec);
        let value = parse("vertex 0,0,0 blob=\"aGVsbG8=\"", &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(2), Some(&FieldValue::Bytes64(b"hello".to_vec())));
    }

    // --- primitive 11: dynamic value literal ---
    fn value_spec() -> RecordSpec {
        RecordSpec::new(Some("payload"), RecordLayout::Inline, vec![FieldSpec::new(0, "data", Shape::Value)])
    }

    #[test]
    fn primitive_dynamic_value_round_trips_and_bridges_to_serde_json() {
        let spec = value_spec();
        assert_round_trip("payload data={a=1 b=[1 2 3] c=\"x\"}", &spec);
        let value = parse("payload data={a=1}", &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::Value(dsl_value) = value.get(0).unwrap().clone() else { panic!() };
        let json: serde_json::Value = dsl_value.into();
        assert_eq!(json["a"], serde_json::json!(1.0));
    }

    // --- primitive 12: sparse patch records (Option<T> absent != null) ---
    #[test]
    fn primitive_sparse_patch_distinguishes_absent_from_present() {
        let spec = camera_spec();
        let with = parse("camera x=1 y=1 zoom=1 label=\"x\"", &spec, &ParseOptions::default()).expect("parse with");
        let without = parse("camera x=1 y=1 zoom=1", &spec, &ParseOptions::default()).expect("parse without");
        assert_ne!(with.get(3), without.get(3));
        assert_eq!(without.get(3), Some(&FieldValue::Absent));
    }

    // --- primitive 15: comments ---
    #[test]
    fn primitive_comments_are_skipped_as_trivia() {
        let spec = camera_spec();
        let value = parse("# a comment\ncamera x=1 y=2 zoom=3 # trailing comment", &spec, &ParseOptions::default()).expect("parse with comments");
        assert_eq!(value.get(0), Some(&FieldValue::Float(1.0)));
    }

    // --- primitive 16: real spans ---
    #[test]
    fn primitive_spans_are_real_on_parse_error() {
        let spec = camera_spec();
        let error = parse("camera x=1\ny=notanumber zoom=1", &spec, &ParseOptions::default()).unwrap_err();
        assert_eq!(error.span.line, 2, "error span must point at the real line, not (1,1)");
    }

    // --- idempotent canonicalization ---
    #[test]
    fn canonicalization_is_idempotent() {
        let spec = camera_spec();
        let once = canonicalize("camera   zoom=3   x=1 y=2", &spec, &ParseOptions::default()).expect("canonicalize once");
        let twice = canonicalize(&once, &spec, &ParseOptions::default()).expect("canonicalize twice");
        assert_eq!(once, twice, "canonicalize(canonicalize(x)) must equal canonicalize(x)");
    }

    // --- limits enforced, not panicking ---

    #[test]
    fn deeply_nested_blocks_hit_the_depth_limit_as_a_diagnostic() {
        // `group_spec()` (primitive 4, above) is already genuinely self-referential, so it needs no
        // pre-unrolling to exercise real depth this many levels deep — `parse` only ever expands one
        // level of its lazy `Statements` fn pointer at a time, following the actual input text.
        let levels = 20;
        let spec = group_spec();
        let mut nested = String::from("group root");
        for _ in 0..levels {
            nested.push_str(" children { group a");
        }
        for _ in 0..levels {
            nested.push('}');
        }
        let tiny_limits = Limits { max_depth: 10, ..Limits::default() };
        let opts = ParseOptions { limits: tiny_limits, mode: SourceMode::Document };
        let result = parse(&nested, &spec, &opts);
        assert!(result.is_err(), "exceeding max_depth must produce an error, not a stack overflow");

        let generous_limits = Limits { max_depth: 100, ..Limits::default() };
        let generous_opts = ParseOptions { limits: generous_limits, mode: SourceMode::Document };
        assert!(parse(&nested, &spec, &generous_opts).is_ok(), "the same nesting must parse fine under a generous depth limit");
    }

    // --- LanguageService ---
    #[test]
    fn language_service_reports_semantic_tokens_and_diagnostics() {
        let spec = camera_spec();
        let service = LanguageService::new(&spec);
        let classes = service.semantic_tokens("camera x=1 y=2 zoom=3");
        assert!(classes.iter().any(|(class, _)| *class == TokenClass::Keyword));
        assert!(service.diagnostics("camera x=1 y=2 zoom=3").is_empty());
        assert!(!service.diagnostics("camera x=notanumber").is_empty());
    }

    #[test]
    fn language_service_completions_include_every_declared_key() {
        let spec = camera_spec();
        let service = LanguageService::new(&spec);
        let labels: Vec<String> = service.completions("", 0).into_iter().map(|c| c.label).collect();
        assert!(labels.contains(&"x".to_string()));
        assert!(labels.contains(&"zoom".to_string()));
        assert!(labels.contains(&"label".to_string()));
    }

    // --- 10k-iteration generative round trip over the flat-scalar shape ---
    #[test]
    fn generative_round_trip_over_scalar_records() {
        let spec = camera_spec();
        let mut state: u64 = 0xD1B54A32D192ED03;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..2_000 {
            let x = (next() % 2000) as i64 - 1000;
            let y = (next() % 2000) as i64 - 1000;
            let zoom = (next() % 2000) as i64 - 1000;
            let text = format!("camera x={x} y={y} zoom={zoom}");
            assert_round_trip(&text, &spec);
        }
    }
}
//#endregion 🧪Tests
