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
    /// Structure-of-Arrays columnar table: `key [col:TYPE ...] { v11 v12 ...  v21 v22 ... }`.
    /// `fn() -> RecordSpec` is the SAME lazy self-referential seam `Record`/`Statements` use.
    /// Parses to `FieldValue::List(Vec<FieldValue::Record>)` — identical to `List(Record)` — so
    /// no binder/diff/derive path needs to know a field is a table rather than a verbose AoS list;
    /// only the parser's alternate-input detection and the printer's always-SoA output differ.
    Table(fn() -> RecordSpec),
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

struct Cursor {
    tokens: Vec<SpannedToken>,
    pos: usize,
    limits: Limits,
}

impl Cursor {
    // `SourceMode` no longer participates in parsing (its only consumer, `RawLines`, is gone —
    // `Shape::Text` now accepts `Ident|Text` identically regardless of Document/Inline); it stays
    // a `ParseOptions`/`parse` public-API distinction only, still meaningful to callers choosing
    // between `dsl::__rt::parse_document_record`/`parse_inline_record`.
    fn new(tokens: Vec<SpannedToken>, limits: Limits) -> Self {
        let tokens: Vec<SpannedToken> = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, pos: 0, limits }
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

/// @emoji ✂️ The structural seam between lexing and parsing: everything downstream of a token
/// vector is grammar-only and needs no raw source bytes (the parser is token-only — no shape
/// still consumes verbatim source text the way the deleted `RawLines` shape once did). Exists so
/// a caller that already has tokens (e.g. an incremental relexer) can skip `parse`'s own lex pass.
pub fn parse_tokens(tokens: Vec<SpannedToken>, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(tokens, opts.limits);
    parse_record_body(&mut cursor, spec, 0)
}

pub fn parse(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let tokens = lex(text, &opts.limits, false)?;
    parse_tokens(tokens, spec, opts)
}

fn ident_like_text(token: &SpannedToken) -> String {
    token.text.as_str().to_string()
}

fn parse_scalar(cursor: &mut Cursor, shape: &Shape) -> Result<FieldValue, TextError> {
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
        Shape::Text => match cursor.peek().kind {
            TokenKind::Text => {
                let token = cursor.advance();
                let text = dsl_core::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(e, token.span))?;
                Ok(FieldValue::Text(text))
            }
            TokenKind::Ident => {
                let token = cursor.advance();
                Ok(FieldValue::Text(ident_like_text(&token)))
            }
            other => Err(TextError::new(format!("expected Text, found {other:?} '{}'", cursor.peek().text.as_str()), cursor.span())),
        },
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

fn parse_shape(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) => parse_scalar(cursor, shape),
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
        // Reached directly whenever a `Table` shape is parsed via the generic `key=` dispatch
        // (the AoS-verbose alternate input, `name= [ key=val ... ]`) or nested inside another
        // shape — both are structurally identical to `List(Record)`, so delegate to that arm
        // rather than duplicating it. The bare SoA form (`name [col:TYPE ...] { rows }`) is
        // recognized earlier, in `parse_record_body`, and calls `parse_table_soa` directly since
        // its grammar (a header, then count-delimited rows) isn't reachable through `parse_shape`.
        Shape::Table(spec_fn) => {
            validate_table_columns(&spec_fn())?;
            let list_shape = Shape::List(Box::new(Shape::Record(*spec_fn)));
            parse_shape(cursor, &list_shape, depth)
        }
        Shape::Wire => Ok(FieldValue::Wire(parse_wire(cursor)?)),
    }
}

fn current_keyword(cursor: &Cursor) -> Option<String> {
    if cursor.peek().kind == TokenKind::Ident && cursor.at_attr_key().is_none() {
        Some(cursor.peek().text.as_str().to_string())
    } else {
        None
    }
}

fn parse_dsl_value(cursor: &mut Cursor, depth: usize) -> Result<DslValue, TextError> {
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

/// @emoji 🕸️ Parses one wire literal. `<-` is accepted sugar only: normalized here by swapping
/// the two endpoints, so the stored `WireValue` (and everything reprinted from it) only ever
/// holds `->`/`--` — `b<-a` and `a->b` parse to the identical value and print identically.
fn parse_wire(cursor: &mut Cursor) -> Result<WireValue, TextError> {
    let mut from = parse_wire_node(cursor)?;
    let edge = if matches!(cursor.peek().kind, TokenKind::Arrow | TokenKind::DashArrow | TokenKind::BackArrow) {
        let arrow_kind = cursor.advance().kind;
        let to = parse_wire_node(cursor)?;
        match arrow_kind {
            TokenKind::BackArrow => {
                let swapped_to = std::mem::replace(&mut from, to);
                Some((true, swapped_to))
            }
            other => Some((other == TokenKind::Arrow, to)),
        }
    } else {
        None
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, properties })
}

/// @emoji 🔌 Small public entry point other crates (the graph wire module, trinity) can call
/// directly to lex + parse one standalone wire literal, without needing a `RecordSpec` around it.
pub fn parse_wire_text(text: &str) -> Result<WireValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false)?;
    let mut cursor = Cursor::new(tokens, limits);
    parse_wire(&mut cursor)
}

fn parse_wire_node(cursor: &mut Cursor) -> Result<WireNode, TextError> {
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
fn parse_record_body(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
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
        if field.optional {
            // An explicit `_` placeholder always means "absent, but consume the slot" — this is
            // what keeps LATER positionals aligned when an earlier optional one is skipped (see
            // `print_record`'s matching print-side logic). Only positional contexts ever see a
            // `Placeholder` token; keyed optionals are simply omitted instead.
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
            if !can_start_positional(cursor, &field.shape) {
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
        }
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block` fields are also excluded from the `key=value` loop below: their own key acts as a
    // bare leading keyword (`children { ... }`, no `=`) — `Table` fields (bare `key [...] {...}`
    // SoA form) are handled the same way, via their own lookahead branch below.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !matches!(f.shape, Shape::Statements(_))).collect();

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
        // `Table`'s bare SoA form: the keyword directly followed by `[` (no `=`) — distinct from
        // the AoS-verbose `key=[...]` form already handled by the `at_attr_key` branch above.
        if let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Table(_)) && cursor.at_keyword(&f.key) && cursor.peek_at(1).kind == TokenKind::LBracket) {
            let field = keyed.remove(index);
            let Shape::Table(spec_fn) = &field.shape else { unreachable!() };
            let spec_fn = *spec_fn;
            cursor.advance();
            let value = parse_table_soa(cursor, spec_fn, depth + 1)?;
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

    Ok(record)
}

fn can_start_positional(cursor: &Cursor, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        // Only `Text|Placeholder` — NOT bare `Ident` — may start an optional positional `Text`
        // field: an unquoted bare-ident value here would be indistinguishable from the next
        // statement's leading keyword, so this deliberately narrower check (versus `Shape::Text`
        // parsing `Ident|Text` everywhere else) resolves that ambiguity.
        Shape::Text => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Bytes64 => cursor.peek().kind == TokenKind::Text,
        Shape::List(_) => cursor.peek().kind == TokenKind::LBracket,
        Shape::Block(_) | Shape::Map(_) => cursor.peek().kind == TokenKind::LBrace,
        _ => true,
    }
}

//#region 🔖Table
/// @emoji 🚧 Which shapes have a fixed/bounded token extent and may therefore be a `Table`
/// column: an unbounded `Tuple` (`len: None`, comma-separated until... forever) and `Statements`
/// (repeats until a non-matching keyword) both need an external delimiter to know where they end
/// — fine inside `[ ]`/`{ }` brackets, fatal inside a table row where the ONLY thing marking a
/// row boundary is "we've now read exactly `columns.len()` values".
fn shape_is_self_delimiting(shape: &Shape) -> bool {
    !matches!(shape, Shape::Statements(_) | Shape::Tuple(_, None))
}

/// @emoji 🚧 Spec-build-time validation for a `Table`'s element `RecordSpec` — called wherever a
/// `Shape::Table(spec_fn)` is first evaluated (both parse paths, and printing), since `spec_fn` is
/// a lazy pointer rather than an eagerly-built value there is no earlier moment to check it at.
fn validate_table_columns(spec: &RecordSpec) -> Result<(), TextError> {
    for field in &spec.fields {
        if !shape_is_self_delimiting(&field.shape) {
            return Err(TextError::new(
                format!("table column '{}' has a non-self-delimiting shape ({}) and cannot be a table column", field.key, shape_type_name(&field.shape)),
                TextSpan::at(1, 1),
            ));
        }
    }
    Ok(())
}

/// @emoji 🏷️ UPPERCASE schema type tag for a `Shape` — what a `Table` header prints per column
/// (`id:TEXT`), per the unified syntax law (`UPPERCASE` for engine shapes, `PascalCase` reserved
/// for technology-declared domain kinds).
pub fn shape_type_name(shape: &Shape) -> &'static str {
    match shape {
        Shape::Bool => "BOOL",
        Shape::Int => "INT",
        Shape::UInt => "UINT",
        Shape::Float => "NUM",
        Shape::Text => "TEXT",
        Shape::Bytes64 => "BYTES",
        Shape::Enum(_) => "ENUM",
        Shape::Tuple(_, _) => "TUPLE",
        Shape::List(_) => "LIST",
        Shape::Record(_) => "REC",
        Shape::Block(_) => "BLOCK",
        Shape::Statements(_) => "STMT",
        Shape::Map(_) => "MAP",
        Shape::Value => "VAL",
        Shape::Table(_) => "TABLE",
        Shape::Wire => "WIRE",
    }
}

/// @emoji 📊 Parses the bare SoA form of a `Table` field: `[col:TYPE ...] { v11 v12 ...  v21 v22
/// ... }`, cursor positioned right after the field's own keyword has already been consumed. The
/// header names columns (in the order values then appear per row); a `:TYPE` suffix is accepted
/// but not required to resolve a column (it's a human/printer-facing tag, not load-bearing for
/// parsing — the column's real shape always comes from the element `RecordSpec`), which is what
/// lets a hand-written header omit types the engine can already infer. Rows have NO separator —
/// reading exactly `columns.len()` values per row is what makes a row self-delimiting, which is
/// also why every column shape must itself be self-delimiting (`validate_table_columns`).
fn parse_table_soa(cursor: &mut Cursor, spec_fn: fn() -> RecordSpec, depth: usize) -> Result<FieldValue, TextError> {
    let element_spec = spec_fn();
    validate_table_columns(&element_spec)?;
    cursor.expect(TokenKind::LBracket)?;
    let mut columns: Vec<&FieldSpec> = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        let key_token = cursor.expect(TokenKind::Ident)?;
        let key = key_token.text.as_str().to_string();
        if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            cursor.expect(TokenKind::Ident)?; // type tag — documentation only, not re-validated here
        }
        let field_spec = element_spec.fields.iter().find(|f| f.key == key).ok_or_else(|| TextError::new(format!("unknown table column '{key}'"), key_token.span))?;
        columns.push(field_spec);
    }
    cursor.expect(TokenKind::RBracket)?;
    cursor.expect(TokenKind::LBrace)?;
    let mut rows = Vec::new();
    while cursor.peek().kind != TokenKind::RBrace {
        let mut record = RecordValue::default();
        for field_spec in &columns {
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field_spec.id, FieldValue::Absent);
                continue;
            }
            let value = parse_shape(cursor, &field_spec.shape, depth + 1)?;
            record.fields.insert(field_spec.id, value);
        }
        for field_spec in &element_spec.fields {
            record.fields.entry(field_spec.id).or_insert(FieldValue::Absent);
        }
        rows.push(FieldValue::Record(record));
        cursor.limits.check_nodes(rows.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBrace)?;
    Ok(FieldValue::List(rows))
}
//#endregion 🔖Table

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
///
/// @emoji 📏 Canonical spacing rules (both join modes, structurally guaranteed — never hand-tuned
/// per callsite): never a space adjacent to `=` (`key=[ a b ]`, not `key= [ a b ]` — the printer
/// achieves this by pushing a bare `key=` atom, calling [`Writer::glue`], then printing the
/// value); exactly one space between sibling atoms; exactly one space just inside `[ ]`/`{ }` when
/// rendered inline (`[ a b ]`, not `[a b]`) — EXCEPT a `Table` header's `[ ]`, which is glued
/// tight on both sides (`[id:TEXT x:NUM]`) since it's a fixed one-shot header, not a
/// space-joined element list; a space appears before a keyword-led block's `{` (`children {
/// ... }`) but never before a glued composite's `{` (`data={ ... }`).
pub struct Writer {
    chunks: Vec<Chunk>,
    indent: usize,
}

enum Chunk {
    Atom(String),
    OpenBlock,
    CloseBlock,
    NewRecord,
    /// @emoji 🧲 One-shot marker: the very next `Atom`/`OpenBlock` chunk renders with NO
    /// preceding separator (space in Inline mode, space-or-newline-continuation in Document mode)
    /// — consumed by that one chunk, then normal spacing resumes. See [`Writer::glue`].
    Glue,
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

    /// @emoji 🧲 Fuses the next pushed chunk onto whatever precedes it, with no separator, in
    /// BOTH join modes — the mechanism behind every `key=value`/`key=[...]`/`key={...}` fusion in
    /// this printer. Replaces the old approach of mutating an already-pushed atom's string in
    /// place (which only worked for single-atom scalar values): `glue()` composes with arbitrarily
    /// structured values (nested blocks, lists, whole sub-records) since it's a rendering-time
    /// join, not a string-splice.
    pub fn glue(&mut self) {
        self.chunks.push(Chunk::Glue);
    }

    pub fn render(&self, mode: JoinMode) -> String {
        match mode {
            JoinMode::Inline => {
                let mut parts: Vec<String> = Vec::new();
                let mut glued = false;
                let mut push = |piece: String, glued: &mut bool| {
                    if *glued {
                        if let Some(last) = parts.last_mut() {
                            last.push_str(&piece);
                        } else {
                            parts.push(piece);
                        }
                    } else {
                        parts.push(piece);
                    }
                    *glued = false;
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => push(s.clone(), &mut glued),
                        Chunk::OpenBlock => push("{".to_string(), &mut glued),
                        Chunk::CloseBlock => push("}".to_string(), &mut glued),
                        Chunk::NewRecord => {}
                    }
                }
                parts.join(" ")
            }
            JoinMode::Document => {
                let mut out = String::new();
                let mut indent = 0usize;
                let mut line_open = false;
                let mut glued = false;
                let push_indent = |out: &mut String, indent: usize| {
                    for _ in 0..indent {
                        out.push_str("  ");
                    }
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => {
                            if !line_open {
                                push_indent(&mut out, indent);
                                line_open = true;
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str(s);
                            glued = false;
                        }
                        Chunk::OpenBlock => {
                            if glued {
                                out.push('{');
                            } else {
                                out.push_str(" {");
                            }
                            out.push('\n');
                            line_open = false;
                            indent += 1;
                            glued = false;
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

/// @emoji 🥇 Field print order within one record — NOT declaration order: keyword, then
/// positionals (unchanged), then keyed fields grouped scalar-before-composite-before-table-
/// before-statements, ties broken by original declaration order (a stable sort over an
/// already-declaration-order slice achieves this for free). Metadata/scalars land before large
/// nested/tabular blocks, which is friendlier to lazy loading/streaming readers — parsing stays
/// completely order-independent, so this is a print-only change.
fn keyed_field_rank(shape: &Shape) -> u8 {
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) | Shape::Tuple(_, _) => 0,
        Shape::List(_) | Shape::Map(_) | Shape::Record(_) | Shape::Block(_) | Shape::Value | Shape::Wire => 1,
        Shape::Table(_) => 2,
        Shape::Statements(_) => 3,
    }
}

pub fn print_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    let mut positional: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some()).collect();
    positional.sort_by_key(|f| f.position.unwrap());
    for (index, field) in positional.iter().enumerate() {
        match value.get(field.id) {
            Some(fv) if !matches!(fv, FieldValue::Absent) => print_shape(fv, &field.shape, writer),
            _ => {
                // An absent OPTIONAL positional prints as `_` only if some LATER positional in
                // this same record is actually present — that's what keeps slots aligned for the
                // reader (and reparse). A run of trailing absents needs no placeholder at all.
                let later_present = positional[index + 1..].iter().any(|f| matches!(value.get(f.id), Some(fv) if !matches!(fv, FieldValue::Absent)));
                if later_present {
                    writer.atom("_");
                }
            }
        }
    }

    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty()).collect();
    keyed.sort_by_key(|f| keyed_field_rank(&f.shape));
    for field in keyed {
        match value.get(field.id) {
            Some(FieldValue::Absent) | None => continue,
            Some(fv) => match &field.shape {
                // `Statements` items each carry their own leading keyword — no field-level key at
                // all is ever printed for this shape.
                Shape::Statements(_) => print_shape(fv, &field.shape, writer),
                // `Block`'s and `Table`'s own key is a bare leading keyword, not a `key=value`
                // attribute (`children { ... }`, `nodes [...] { ... }`, never `children={...}`).
                Shape::Block(_) | Shape::Table(_) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    print_shape(fv, &field.shape, writer);
                }
                _ => {
                    writer.atom(format!("{}=", field.key));
                    print_key_value(field, fv, writer);
                }
            },
        }
    }
}

/// @emoji 🧲 `key=` was just pushed by the caller — glue the value onto it with no separator,
/// then print it normally (composed, not string-spliced, so this handles arbitrarily structured
/// values exactly like a bare `print_shape` call would).
fn print_key_value(field: &FieldSpec, value: &FieldValue, writer: &mut Writer) {
    writer.glue();
    match (&field.shape, value) {
        (Shape::Enum(variants), FieldValue::Enum(ordinal)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        _ => print_shape(value, &field.shape, writer),
    }
}

fn scalar_to_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Bool(b) => b.to_string(),
        FieldValue::Int(i) => i.to_string(),
        FieldValue::UInt(u) => u.to_string(),
        FieldValue::Float(f) => format_f64(*f),
        // Bare (unquoted) whenever the text lexes back as exactly this one ident — the printer's
        // half of the "strings bare-preferred" law; `is_bare_ident` also excludes reserved literal
        // idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and number-shaped text, which always fall
        // through to the quoted+escaped form instead.
        FieldValue::Text(s) => {
            if dsl_core::is_bare_ident(s) {
                s.clone()
            } else {
                format!("\"{}\"", dsl_core::escape_text(s))
            }
        }
        FieldValue::Bytes64(bytes) => format!("\"{}\"", base64_encode(bytes)),
        FieldValue::Enum(_) => String::new(), // resolved by caller via variants table when needed
        _ => String::new(),
    }
}

pub fn print_shape(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    match (value, shape) {
        (FieldValue::Bool(_) | FieldValue::Int(_) | FieldValue::UInt(_) | FieldValue::Float(_) | FieldValue::Text(_) | FieldValue::Bytes64(_), _) => {
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
        (FieldValue::List(items), Shape::Table(spec_fn)) => print_table(*spec_fn, items, writer),
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
                writer.glue();
                print_shape(value, inner, writer);
            }
            writer.close_block();
        }
        (FieldValue::Value(dsl_value), Shape::Value) => print_dsl_value(dsl_value, writer),
        (FieldValue::Wire(wire), Shape::Wire) => print_wire(wire, writer),
        _ => {}
    }
}

/// @emoji 📊 Always prints the compact SoA form — this (not the parser, which still accepts the
/// verbose AoS form too) is what makes `canonicalize` migrate old AoS documents to SoA
/// automatically. Header `[ ]` is glued tight on both sides (`[id:TEXT x:NUM]`); rows have no
/// separator, one row per line in Document mode purely for readability (`new_record` is a no-op
/// in Inline mode).
fn print_table(spec_fn: fn() -> RecordSpec, items: &[FieldValue], writer: &mut Writer) {
    let element_spec = spec_fn();
    writer.atom("[");
    writer.glue();
    for field in &element_spec.fields {
        writer.atom(format!("{}:{}", field.key, shape_type_name(&field.shape)));
    }
    writer.glue();
    writer.atom("]");
    writer.open_block();
    for item in items {
        writer.new_record();
        let FieldValue::Record(record) = item else { continue };
        for field in &element_spec.fields {
            match record.get(field.id) {
                Some(fv) if !matches!(fv, FieldValue::Absent) => print_shape(fv, &field.shape, writer),
                _ => writer.atom("_"),
            }
        }
    }
    writer.close_block();
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
                writer.glue();
                print_dsl_value(value, writer);
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
        RecordSpec::new(Some("layer"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "opacity", Shape::Float)])
    }

    fn document_with_layers_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![FieldSpec::new(0, "schema", Shape::Text), FieldSpec::new(1, "layers", Shape::Statements(vec![("layer".to_string(), layer_variant_spec)]))],
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
                FieldSpec::new(0, "id", Shape::Text).positional(0),
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
                FieldSpec::new(1, "tags", Shape::List(Box::new(Shape::Text))).optional(),
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

    // --- bare-string printing: `is_bare_ident` values print unquoted, reserved/number-shaped/
    // multi-word values stay quoted (unified syntax law: strings bare-preferred) ---
    #[test]
    fn bare_strings_print_unquoted_and_reserved_or_number_shaped_values_stay_quoted() {
        let spec = camera_spec();
        let value = parse("camera x=1 y=2 zoom=3 label=alpha", &spec, &ParseOptions::default()).expect("parse");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("label=alpha"), "a bare-ident-shaped value must print unquoted: {printed}");
        assert!(!printed.contains("\"alpha\""), "must not quote a value that already lexes as a bare ident: {printed}");

        for reserved in ["_", "true", "3", "two words"] {
            let mut writer = Writer::new();
            print_shape(&FieldValue::Text(reserved.to_string()), &Shape::Text, &mut writer);
            let out = writer.render(JoinMode::Inline);
            assert!(out.starts_with('"') && out.ends_with('"'), "{reserved:?} must print quoted, got {out:?}");
        }
    }

    // --- `Writer::glue()`: exact-string spacing assertions for every composite shape's
    // `key=value` fusion (the "key= value" bug this replaces) ---
    fn nested_point_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
    }
    fn marker_spec() -> RecordSpec {
        RecordSpec::new(Some("marker"), RecordLayout::Inline, vec![FieldSpec::new(0, "at", Shape::Record(nested_point_spec))])
    }
    fn edge_keyed_wire_spec() -> RecordSpec {
        RecordSpec::new(Some("edge2"), RecordLayout::Inline, vec![FieldSpec::new(0, "link", Shape::Wire)])
    }
    fn tags_map_spec() -> RecordSpec {
        RecordSpec::new(Some("meta"), RecordLayout::Inline, vec![FieldSpec::new(0, "props", Shape::Map(Box::new(Shape::Text)))])
    }

    #[test]
    fn glue_removes_the_key_equals_space_for_every_composite_shape() {
        // List
        let spec = geometry_spec();
        let value = parse("vertex 1,2,3 tags=[a b c]", &spec, &ParseOptions::default()).expect("parse list");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("tags=[ a b c ]"), "List field must glue key= directly onto '[': {printed}");
        assert!(!printed.contains("tags= ["), "must never leave a stray space after 'key=': {printed}");

        // Value (dynamic)
        let spec = value_spec();
        let value = parse("payload data={a=1}", &spec, &ParseOptions::default()).expect("parse value");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("data={"), "Value field must glue key= directly onto '{{': {printed}");
        assert!(!printed.contains("data= {"), "must never leave a stray space before the glued brace: {printed}");

        // Map
        let spec = tags_map_spec();
        let value = parse("meta props={a=\"x\" b=\"y\"}", &spec, &ParseOptions::default()).expect("parse map");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("props={"), "Map field must glue key= directly onto '{{': {printed}");
        assert_round_trip("meta props={a=\"x\" b=\"y\"}", &spec);

        // Record (nested, un-blocked — prints inline without its own keyword)
        let spec = marker_spec();
        let value = parse("marker at=x=1 y=2", &spec, &ParseOptions::default()).expect("parse record");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("at=x=1"), "Record field must glue key= directly onto its first field: {printed}");
        assert!(!printed.contains("at= x=1"), "must never leave a stray space before a nested record: {printed}");

        // Wire (keyed, not positional)
        let spec = edge_keyed_wire_spec();
        let value = parse("edge2 link=a->b", &spec, &ParseOptions::default()).expect("parse wire");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("link=a->b"), "Wire field must glue key= directly onto the wire literal: {printed}");
        assert!(!printed.contains("link= a"), "must never leave a stray space before a keyed wire literal: {printed}");
    }

    // --- wire `<-` normalization: accepted sugar only, always stored/printed as `->` with
    // endpoints swapped ---
    #[test]
    fn wire_back_arrow_normalizes_to_forward_arrow_with_swapped_endpoints() {
        let spec = wire_spec();
        let backward = parse("edge b<-a", &spec, &ParseOptions::default()).expect("parse backward");
        let forward = parse("edge a->b", &spec, &ParseOptions::default()).expect("parse forward");
        assert_eq!(backward, forward, "'b<-a' must parse to the same value as 'a->b'");
        let printed = print(&backward, &spec, JoinMode::Document);
        assert!(printed.contains("a->b"), "must print using '->': {printed}");
        assert!(!printed.contains("<-"), "must never print '<-': {printed}");
    }

    #[test]
    fn parse_wire_text_parses_a_standalone_wire_literal_with_back_arrow() {
        let value = parse_wire_text("b<-a").expect("parse_wire_text");
        assert_eq!(value.from.id, "a");
        let (directed, to) = value.edge.expect("edge");
        assert!(directed);
        assert_eq!(to.id, "b");
    }

    // --- primitive 17: `Shape::Table` — SoA columnar collection ---
    fn table_row_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "id", Shape::Text),
                FieldSpec::new(1, "x", Shape::Float),
                FieldSpec::new(2, "y", Shape::Float),
                FieldSpec::new(3, "link", Shape::Wire).optional(),
            ],
        )
    }
    fn table_doc_spec() -> RecordSpec {
        RecordSpec::new(Some("scene"), RecordLayout::Inline, vec![FieldSpec::new(0, "nodes", Shape::Table(table_row_spec))])
    }

    #[test]
    fn table_soa_round_trips_with_underscore_absent_cell_and_a_wire_column() {
        let spec = table_doc_spec();
        let text = "scene nodes [id:TEXT x:NUM y:NUM link:WIRE] { a 1 2 _  b 3 4 a@out->b@in }";
        assert_round_trip(text, &spec);
        let value = parse(text, &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected a table (List) value") };
        assert_eq!(rows.len(), 2);
        let FieldValue::Record(row0) = &rows[0] else { panic!("expected a Record row") };
        assert_eq!(row0.get(3), Some(&FieldValue::Absent), "the '_' cell must parse as Absent");
        let FieldValue::Record(row1) = &rows[1] else { panic!("expected a Record row") };
        assert!(matches!(row1.get(3), Some(FieldValue::Wire(_))), "the wire-typed column must parse as FieldValue::Wire");

        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM link:WIRE]"), "header must print tight SoA, no inner spaces: {printed}");
    }

    #[test]
    fn table_accepts_verbose_aos_input_and_canonicalizes_to_soa_output() {
        let spec = table_doc_spec();
        let aos_text = "scene nodes=[ id=a x=1 y=2  id=b x=3 y=4 ]";
        let value = parse(aos_text, &spec, &ParseOptions::default()).expect("parse AoS-verbose");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM link:WIRE]"), "AoS input must canonicalize to the SoA header on print: {printed}");
        assert!(!printed.contains("nodes="), "must never print the old AoS '=' form: {printed}");
        let reparsed = parse(&printed, &spec, &ParseOptions::default()).expect("reparse canonicalized SoA");
        assert_eq!(value, reparsed, "AoS-in/SoA-out must still round trip to the same value");
    }

    #[test]
    fn table_header_without_explicit_type_tags_is_still_parseable() {
        let spec = table_doc_spec();
        let text = "scene nodes [id x y link] { a 1 2 _  b 3 4 a@out->b@in }";
        let value = parse(text, &spec, &ParseOptions::default()).expect("parse header without explicit types");
        let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected a table (List) value") };
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn table_document_and_inline_renders_agree() {
        let spec = table_doc_spec();
        assert_document_inline_agree("scene nodes [id:TEXT x:NUM y:NUM link:WIRE] { a 1 2 _  b 3 4 a@out->b@in }", &spec);
    }

    #[test]
    fn table_rejects_non_self_delimiting_column_shapes_at_spec_build_time() {
        fn unbounded_tuple_row_spec() -> RecordSpec {
            RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "vals", Shape::Tuple(Box::new(Shape::Float), None))])
        }
        fn bad_table_doc_spec() -> RecordSpec {
            RecordSpec::new(Some("bad"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(unbounded_tuple_row_spec))])
        }
        let spec = bad_table_doc_spec();
        let result = parse("bad rows [vals:TUPLE] { 1,2,3 }", &spec, &ParseOptions::default());
        assert!(result.is_err(), "an unbounded Tuple column must be rejected, not silently accepted");
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
