//! 🧬️ `dsl_schema` — the data-driven declarative grammar engine: technologies describe their
//! document/op grammar as `RecordSpec`/`Shape` DATA (not code), and this crate parses text against
//! that data into a generic `Cst` (walked by typed binders that `dsl_derive` will generate) and
//! prints it back via a chunk `Writer` that structurally guarantees the newline law: every
//! grammar renders both as multi-line canonical `Document` text and as one space-joined `Inline`
//! line, and both re-parse to the same value.

#[cfg(test)]
extern crate self as semio_framework_dsl_record;
use semio_framework_dsl::format_f64;
use semio_framework_dsl::lex;
use semio_framework_dsl::parse_f64;
pub use semio_framework_diagnostic::{Limits, TextError, TextSpan};
use semio_framework_dsl::SpannedToken;


use semio_framework_dsl::TokenClass;
use semio_framework_dsl::TokenKind;
use std::collections::{HashSet};
#[cfg(test)] use std::collections::{HashMap};
use crate::notation as dsl_notation;
pub use semio_framework_dsl_notation as notation;
use semio_framework_value::{DslValue,NativeDecodeControl,NativeEncodeControl,Number};
use semio_framework_value::{ValueError,ValueRefusalKind};

#[path = "🛬️decoding/🦀️.rs"]
mod controlled_decoding;
pub use controlled_decoding::{parse_exact_controlled,parse_expr_text_controlled};

pub use semio_framework_dsl_record_emission::*;





pub use semio_framework_dsl_record_model::*;

//#region 🔖️Cursor
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
    // 🚫️async: E1 pure in-memory cursor consumed by `Iterator::position` sync closures (`:1345`, `:1354` via `at_keyword`) — see R9.
    // The whole impl block is one call graph (`peek`/`peek_at`/`span`/`advance`/`expect`/`at_attr_key`/`at_keyword` all call each
    // other with no suspension point ever possible), so the language barrier on `at_keyword` propagates to every method here.
    //
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
            Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected {:?}, found {:?} '{}'", kind, self.peek().kind, self.peek().text.as_str()),self.span()))
        }
    }

    /// 🔎️ Whether the next token is an `Ident` that is followed by `=` — the LL(2)
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

    fn dynamic_attr_key(&self) -> Result<Option<String>, TextError> {
        if self.peek().kind == TokenKind::Text && self.peek_at(1).kind == TokenKind::Equals {
            semio_framework_dsl::unescape_text(&self.peek().text.as_str(), false).map(Some).map_err(|message| TextError::new(ValueRefusalKind::InvalidValue,message,self.span()))
        } else {
            Ok(self.at_attr_key())
        }
    }
}
//#endregion 🔖️Cursor

//#region 🔖️Parser
pub struct ParseOptions {
    pub limits: Limits,
    pub mode: SourceMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self { limits: Limits::default(), mode: SourceMode::Document }
    }
}

/// ✂️ The structural seam between lexing and parsing: everything downstream of a token
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

/// 🛑️ Parses one terminal record and rejects every token outside its schema-owned body.
pub fn parse_exact(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(lex(text, &opts.limits, false)?, opts.limits);
    let record = parse_record_body(&mut cursor, spec, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(record)
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
                other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected 'true' or 'false', found '{other}'"),token.span)),
            }
        }
        Shape::Int => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: i64 = token.text.as_str().parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::Int(value))
        }
        Shape::UInt => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: u64 = token.text.as_str().parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid unsigned integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        Shape::Float => {
            let is_float_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
            if !is_float_token {
                return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a float, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Float(value))
        }
        Shape::Text => parse_scalar_text(cursor),
        Shape::Bytes64 => {
            let token = cursor.expect(TokenKind::Text)?;
            let bytes = base64_decode(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Bytes64(bytes))
        }
        Shape::Enum(variants) => {
            let token = cursor.expect(TokenKind::Ident)?;
            let text = token.text.as_str();
            variants.iter().find(|(tag, _)| tag == text.as_ref()).map(|(_, ordinal)| FieldValue::Enum(*ordinal)).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("unknown enum tag '{text}'"),token.span))
        }
        Shape::Quantity(declared) | Shape::Angle(declared) => parse_quantity(cursor, declared),
        Shape::Ref(_) => parse_scalar_text(cursor),
        Shape::Embed(declared_lang) => parse_embed(cursor, declared_lang),
        Shape::EmbedFrom(_) => Err(TextError::new(ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Count => {
            if cursor.peek().kind != TokenKind::Ident {
                return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let text = token.text.as_str();
            let digits = text.strip_prefix('x').ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found '{text}'"),token.span))?;
            let value: u64 = digits.parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid count literal 'x{digits}'"),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        other => Err(TextError::new(ValueRefusalKind::UnsupportedOwner,format!("shape {other:?} is not a scalar"),cursor.span())),
    }
}

/// 🧮️ Precedence-climbing entry point for `Shape::Expr`'s body (called with the caller's
/// outer `(`/`)` already consumed). `min_prec` is the lowest operator precedence this call is
/// willing to keep consuming at — the standard technique for turning a flat token stream into a
/// precedence-correct tree without a separate tokenize-then-shunting-yard pass.
fn parse_expr(cursor: &mut Cursor, min_prec: u8) -> Result<ExprValue, TextError> {
    let lhs = parse_expr_unary(cursor)?;
    parse_expr_continue(cursor, min_prec, lhs)
}

/// 🧮️ The loop body of `parse_expr`, factored out so the glued-negative-number case below
/// can re-enter it with an ALREADY-PARSED left operand instead of calling `parse_expr_unary` again
/// (which would re-consume nothing, since the token was already consumed to build that operand).
fn parse_expr_continue(cursor: &mut Cursor, min_prec: u8, mut lhs: ExprValue) -> Result<ExprValue, TextError> {
    loop {
        // The shared lexer glues a leading `-` onto an immediately-following digit as ONE negative
        // number token (`y=-2`'s existing, load-bearing behavior — see dsl_core's lexer) — so
        // `10-2` lexes as `Int(10), Int(-2)`, not `Int(10), Minus, Int(2)`. Detect that shape here
        // and reinterpret it as `Sub` with a positive right operand, rather than requiring authors
        // to always space out `-` (canonical PRINT output always does; hand-written input may not).
        let glued_negative = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) && cursor.peek().text.as_str().starts_with('-');
        let (op, prec) = if glued_negative {
            (ExprOp::Sub, ExprOp::Sub.precedence())
        } else {
            match cursor.peek().kind {
                TokenKind::Plus => (ExprOp::Add, ExprOp::Add.precedence()),
                TokenKind::Minus => (ExprOp::Sub, ExprOp::Sub.precedence()),
                TokenKind::Star => (ExprOp::Mul, ExprOp::Mul.precedence()),
                TokenKind::Slash => (ExprOp::Div, ExprOp::Div.precedence()),
                _ => break,
            }
        };
        if prec < min_prec {
            break;
        }
        let rhs = if glued_negative {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            parse_expr_continue(cursor, prec + 1, ExprValue::Num(-value))?
        } else {
            cursor.advance();
            parse_expr(cursor, prec + 1)?
        };
        lhs = ExprValue::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_expr_unary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    if cursor.peek().kind == TokenKind::Minus {
        cursor.advance();
        return Ok(ExprValue::Neg(Box::new(parse_expr_unary(cursor)?)));
    }
    parse_expr_primary(cursor)
}

fn parse_expr_primary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Float | TokenKind::Int => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(ExprValue::Num(value))
        }
        TokenKind::Ident | TokenKind::Text => {
            let token = cursor.advance();
            let name = if token.kind==TokenKind::Text{semio_framework_dsl::unescape_text(&token.text.as_str(),false).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error,token.span))?}else{ident_like_text(&token)};
            if cursor.peek().kind == TokenKind::LParen {
                cursor.advance();
                let mut args = Vec::new();
                if cursor.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(parse_expr(cursor, 0)?);
                        if cursor.peek().kind == TokenKind::Comma {
                            cursor.advance();
                            continue;
                        }
                        break;
                    }
                }
                cursor.expect(TokenKind::RParen)?;
                Ok(ExprValue::Call(name, args))
            } else {
                Ok(ExprValue::Var(name))
            }
        }
        TokenKind::LParen => {
            cursor.advance();
            let inner = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(inner)
        }
        other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a number, variable, or '(', found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🧮️ Standalone entry point for parsing a bare expression body (no surrounding `(`/`)`,
/// unlike `Shape::Expr`'s own field-value grammar) — what `pack_value`'s decoder calls to turn the
/// canonical string it stored back into an `ExprValue`, since decode has no `Cursor` of its own.
pub fn parse_expr_text(text: &str) -> Result<ExprValue, TextError> {
    let tokens = lex(text, &Limits::default(), false)?;
    let mut cursor = Cursor::new(tokens, Limits::default());
    let value = parse_expr(&mut cursor, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(value)
}

fn parse_scalar_text(cursor: &mut Cursor) -> Result<FieldValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Text => {
            let token = cursor.advance();
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Text(text))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            Ok(FieldValue::Text(ident_like_text(&token)))
        }
        other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected Text, found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🗣️ `Shape::Embed`'s parse: a `Fence` token (Document mode — see `dsl_core`'s lexer for
/// the `lang\u{0}content` encoding) with an empty or matching lang tag, OR anything
/// `parse_scalar_text` already accepts (Inline mode's escaped-quoted fallback) — both converge on
/// the same `FieldValue::Text`, which is what makes Document/Inline renders agree.
fn parse_embed(cursor: &mut Cursor, declared_lang: &str) -> Result<FieldValue, TextError> {
    if cursor.peek().kind == TokenKind::Fence {
        let token = cursor.advance();
        let raw = token.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,"malformed fence token (missing separator)",token.span))?;
        if !lang.is_empty() && !declared_lang.is_empty() && lang != declared_lang {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("fence declares lang '{lang}', field expects '{declared_lang}'"),token.span));
        }
        return Ok(FieldValue::Text(content.to_string()));
    }
    parse_scalar_text(cursor)
}

fn sibling_text_field<'a>(record: &'a RecordValue, spec: &RecordSpec, lang_key: &str) -> Option<&'a str> {
    let field = spec.fields.iter().find(|f| f.key == lang_key)?;
    match record.get(field.id)? {
        FieldValue::Text(text) => Some(text.as_str()),
        _ => None,
    }
}

fn parse_field_shape(cursor: &mut Cursor, field: &FieldSpec, spec: &RecordSpec, record: &RecordValue, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::EmbedFrom(lang_key) = &field.shape {
        let declared = sibling_text_field(record, spec, lang_key).unwrap_or("");
        return parse_embed(cursor, declared);
    }
    parse_shape(cursor, &field.shape, depth)
}

/// 📐️ Shared parse for `Shape::Quantity`/`Shape::Angle`: a number, optionally followed by a
/// GLUED (no whitespace between — the lexer already ends a numeric token exactly where the next
/// `Ident` token begins for input like `210GPa`) unit-symbol ident. No suffix means the number is
/// already expressed in `declared`'s unit; a suffix converts, erroring if the dimensions differ.
fn parse_quantity(cursor: &mut Cursor, declared: &'static semio_framework_dsl::UnitSpec) -> Result<FieldValue, TextError> {
    let is_number_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
    if !is_number_token {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a quantity, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let number_token = cursor.advance();
    let value = parse_f64(&number_token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,number_token.span))?;
    let suffix = cursor.peek();
    if suffix.kind == TokenKind::Ident && suffix.byte_range.0 == number_token.byte_range.1 {
        let suffix_token = cursor.advance();
        let symbol = suffix_token.text.as_str().to_string();
        let suffix_unit = semio_framework_dsl::unit_by_symbol(&symbol).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("unknown unit '{symbol}'"),suffix_token.span))?;
        let converted = if value.is_nan(){if suffix_unit.dimension!=declared.dimension{return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span));}value}else{semio_framework_dsl::convert(value,suffix_unit,declared).ok_or_else(||TextError::new(ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span))?};
        Ok(FieldValue::Float(converted))
    } else {
        Ok(FieldValue::Float(value))
    }
}

/// 🔢️ Reads a complete numeric token for coordinate, direction, dimension and range components.
fn parse_plain_number(cursor: &mut Cursor) -> Result<f64, TextError> {
    if !(matches!(cursor.peek().kind,TokenKind::Float|TokenKind::Int)||(cursor.peek().kind==TokenKind::Ident&&matches!(cursor.peek().text.as_str().as_ref(),"nan"|"inf"|"-inf"))) {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a number, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let token = cursor.advance();
    parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))
}

/// 📍️ Shared body for `Shape::Coord`/`Shape::Dir`: a fixed-arity comma-separated run of
/// plain numbers, with no delimiter of its own (the caller already consumed the `@`/`^` sigil).
fn parse_fixed_number_tuple(cursor: &mut Cursor, arity: usize, what: &str) -> Result<FieldValue, TextError> {
    let mut items = Vec::with_capacity(arity);
    loop {
        items.push(FieldValue::Float(parse_plain_number(cursor)?));
        if items.len() == arity {
            break;
        }
        cursor.expect(TokenKind::Comma)?;
    }
    if cursor.peek().kind == TokenKind::Comma {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("{what} literal expects exactly {arity} components"),cursor.span()));
    }
    Ok(FieldValue::Tuple(items))
}

/// 📏️ `Shape::Dim`'s `WxHxD` grammar: the FIRST number is an ordinary `Float|Int` token;
/// every number after it is glued (no whitespace, no comma) onto an `x`-prefixed ident — the
/// lexer has no notion of a bare `x` operator (digits/`.` are ident-continue, so `x0.12x0.24`
/// lexes as ONE `Ident` token), so this splits that single glued token on `x` itself rather than
/// looping token-by-token the way `parse_fixed_number_tuple` does.
fn parse_dim(cursor: &mut Cursor, dims: usize) -> Result<FieldValue, TextError> {
    let first_token = cursor.peek().clone();
    let first = parse_plain_number(cursor)?;
    let mut items = vec![FieldValue::Float(first)];
    if dims > 1 {
        let suffix = cursor.peek();
        if suffix.kind != TokenKind::Ident || suffix.byte_range.0 != first_token.byte_range.1 {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x' (e.g. '2x3'), found only one"),cursor.span()));
        }
        let suffix_token = cursor.advance();
        let suffix_text = suffix_token.text.as_str();
        let parts: Vec<&str> = suffix_text.split('x').collect();
        // `"x0.12x0.24".split('x')` yields `["", "0.12", "0.24"]` — the leading empty piece is the
        // text before the first `x`, which is always empty since the suffix itself starts with it.
        if parts.first() != Some(&"") || parts.len() != dims {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x', found '{}{}'", format_f64(first), suffix_text),suffix_token.span));
        }
        for part in &parts[1..] {
            let value = parse_f64(part).map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid dimension component '{part}'"),suffix_token.span))?;
            items.push(FieldValue::Float(value));
        }
    }
    Ok(FieldValue::Tuple(items))
}

fn parse_shape(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) | Shape::Quantity(_) | Shape::Angle(_) | Shape::Ref(_) | Shape::Count | Shape::Embed(_) => parse_scalar(cursor, shape),
        Shape::EmbedFrom(_) => Err(TextError::new(ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Coord(dims) => {
            cursor.expect(TokenKind::At)?;
            parse_fixed_number_tuple(cursor, *dims as usize, "coordinate")
        }
        Shape::Dir => {
            cursor.expect(TokenKind::Caret)?;
            parse_fixed_number_tuple(cursor, 3, "direction")
        }
        Shape::Dim(dims) => parse_dim(cursor, *dims as usize),
        Shape::Range => {
            cursor.expect(TokenKind::LParen)?;
            let lo = parse_plain_number(cursor)?;
            cursor.expect(TokenKind::DotDot)?;
            let hi = parse_plain_number(cursor)?;
            let mut items = vec![FieldValue::Float(lo), FieldValue::Float(hi)];
            if cursor.peek().kind == TokenKind::Comma {
                cursor.advance();
                items.push(FieldValue::Float(parse_plain_number(cursor)?));
            }
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Tuple(items))
        }
        Shape::Expr => {
            cursor.expect(TokenKind::LParen)?;
            let value = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Expr(value))
        }
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
                    return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("tuple expects {} elements, found {}", expected_len, items.len()),cursor.span()));
                }
            }
            Ok(FieldValue::Tuple(items))
        }
        Shape::List(elem) => {
            cursor.expect(TokenKind::LBracket)?;
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                let pos_before = cursor.pos;
                let value = if let Shape::Record(make) = elem.as_ref() {
                    cursor.expect(TokenKind::LBrace)?;
                    let record = parse_record_body(cursor, &(make.ordinary)(), depth + 1)?;
                    cursor.expect(TokenKind::RBrace)?;
                    FieldValue::Record(record)
                } else {
                    parse_shape(cursor, elem, depth + 1)?
                };
                items.push(value);
                if cursor.pos == pos_before {
                    return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("list element made no progress at {:?} '{}' — likely an unrecognized field key", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
                }
                cursor.limits.check_nodes(items.len(), cursor.span())?;
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(FieldValue::List(items))
        }
        Shape::Record(spec_fn) => Ok(FieldValue::Record(parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?)),
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
                let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
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
            while let Some(key) = cursor.dynamic_attr_key()? {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                let record = matches!(inner.as_ref(), Shape::Record(_));
                if record { cursor.expect(TokenKind::LBrace)?; }
                let value = parse_shape(cursor, inner, depth + 1)?;
                if record { cursor.expect(TokenKind::RBrace)?; }
                entries.push((key, value));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Map(entries))
        }
        Shape::Value => Ok(FieldValue::Value(parse_dsl_value(cursor, depth + 1)?)),
        Shape::Table(spec_fn) => {
            validate_table_columns(&(spec_fn.ordinary)())?;
            parse_table_list(cursor, *spec_fn, depth)
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
            while let Some(key) = cursor.dynamic_attr_key()? {
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
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::String(text))
        }
        TokenKind::Int => {
            let token = cursor.advance();
            let text = token.text.as_str();
            if let Ok(v) = text.parse::<u64>() {
                Ok(DslValue::Number(Number::UInt(v)))
            } else if let Ok(v) = text.parse::<i64>() {
                Ok(DslValue::Number(Number::Int(v)))
            } else {
                let value = parse_f64(&text).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
                Ok(DslValue::Number(Number::Float(value)))
            }
        }
        TokenKind::Float => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::Number(Number::Float(value)))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            match token.text.as_str().as_ref() {
                "int64"=>{cursor.expect(TokenKind::LParen)?;let token=cursor.expect(TokenKind::Int)?;let value=token.text.as_str().parse::<i64>().map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error.to_string(),token.span))?;cursor.expect(TokenKind::RParen)?;Ok(DslValue::Number(Number::Int(value)))},
                "bytes64"=>{cursor.expect(TokenKind::LParen)?;let token=cursor.expect(TokenKind::Text)?;let bytes=base64_decode(&token.text.as_str()).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error,token.span))?;cursor.expect(TokenKind::RParen)?;Ok(DslValue::Bytes(bytes))},
                "null" => Ok(DslValue::Null),
                "true" => Ok(DslValue::Bool(true)),
                "false" => Ok(DslValue::Bool(false)),
                "nan"|"inf"=>Ok(DslValue::Number(Number::Float(parse_f64(&token.text.as_str()).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error,token.span))?))),
                other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a value literal, found ident '{other}'"),token.span)),
            }
        }
        other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a value literal, found {other:?}"),cursor.span())),
    }
}

/// 🕸️ Parses one wire literal. `<-` is accepted sugar only: normalized here by swapping
/// the two endpoints, so the stored `WireValue` (and everything reprinted from it) only ever
/// holds `->`/`--` or fused labeled arrows — `b<-a` and `a->b` parse to the identical value.
fn parse_wire(cursor: &mut Cursor) -> Result<WireValue, TextError> {
    fn parse_wire_label(cursor: &mut Cursor) -> Result<WireEdgeLabel, TextError> {
        cursor.expect(TokenKind::LBracket)?;
        let id = if cursor.peek().kind == TokenKind::Ident { Some(ident_like_text(&cursor.advance())) } else { None };
        let kind = if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
        } else {
            None
        };
        let label = WireEdgeLabel { id, kind };
        if label.is_empty() {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,"edge label `[...]` must name an id and/or a `:kind`",cursor.span()));
        }
        cursor.expect(TokenKind::RBracket)?;
        Ok(label)
    }

    let mut from = parse_wire_node(cursor)?;
    let mut edge_label = WireEdgeLabel::default();
    let edge = match cursor.peek().kind {
        TokenKind::Arrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((true, to))
        }
        TokenKind::DashArrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((false, to))
        }
        TokenKind::BackArrow => {
            cursor.advance();
            if cursor.peek().kind == TokenKind::LBracket {
                edge_label = parse_wire_label(cursor)?;
                cursor.expect(TokenKind::Minus)?;
            }
            let to = parse_wire_node(cursor)?;
            let swapped_to = std::mem::replace(&mut from, to);
            Some((true, swapped_to))
        }
        TokenKind::Minus if cursor.peek_at(1).kind == TokenKind::LBracket => {
            cursor.advance();
            edge_label = parse_wire_label(cursor)?;
            let directed = match cursor.peek().kind {
                TokenKind::Arrow => {
                    cursor.advance();
                    true
                }
                TokenKind::DashArrow => {
                    cursor.advance();
                    false
                }
                other => {
                    return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected `->` or `--` to close a labeled edge, found {other:?}"),cursor.span()));
                }
            };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        TokenKind::EdgeArrow => {
            let token = cursor.advance();
            let (directed, label) = dsl_notation::decode_fused_edge_arrow(&token.text.as_str())?;
            edge_label = WireEdgeLabel { id: label.id, kind: label.kind };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        _ => None,
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, edge_label, properties })
}

/// 🔌️ Small public entry point other crates (the graph wire module, trinity) can call
/// directly to lex + parse one standalone wire literal, without needing a `RecordSpec` around it.
pub fn parse_wire_text(text: &str) -> Result<WireValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false)?;
    let mut cursor = Cursor::new(tokens, limits);
    parse_wire(&mut cursor)
}

fn parse_wire_node(cursor: &mut Cursor) -> Result<WireNode, TextError> {
    let FieldValue::Text(id) = parse_scalar_text(cursor)? else { unreachable!() };
    let kind = if cursor.peek().kind == TokenKind::Colon {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    let port = if cursor.peek().kind == TokenKind::At {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    Ok(WireNode { id, kind, port })
}

/// 🧾️ Parses one record: its own leading keyword if `spec.keyword` declares one (the
/// `Statements` dispatcher only peeks to choose a variant — consuming it is always this
/// function's job, so a spec is self-contained regardless of whether it's reached via `parse`
/// directly, `Shape::Record`, or a `Statements` variant), positional fields in declaration order,
/// then order-independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a
/// key), until a token that is neither a known key nor an unfilled positional slot — which ends
/// the record (it belongs to whatever comes next: a new statement, a closing brace, or EOF).
fn parse_record_body(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    if spec.layout == RecordLayout::Call {
        return parse_call_record(cursor, spec, depth);
    }
    if let Some(keyword) = &spec.keyword {
        if cursor.at_keyword(keyword) {
            cursor.advance();
        } else {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected keyword '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
        }
    }
    parse_record_fields(cursor, spec, depth)
}

/// 📛️ Parses a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The parenthesized
/// argument list is parsed by the exact same [`parse_record_fields`] loop every other layout uses
/// — it naturally stops at the first token that matches neither a positional slot nor a known
/// key (here, always `)`), so no special "bounded sub-cursor" is needed to keep it from reading
/// past the closing paren.
fn parse_call_record(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let name_field = spec.fields.iter().find(|f| f.is_call_name).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,"RecordLayout::Call requires exactly one field marked call_name()",cursor.span()))?;
    let name = ident_like_text(&cursor.expect(TokenKind::Ident)?);
    cursor.expect(TokenKind::Equals)?;
    let keyword = spec.keyword.as_deref().ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,"RecordLayout::Call requires RecordSpec.keyword (the call target)",cursor.span()))?;
    if !cursor.at_keyword(keyword) {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected call target '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    cursor.advance();
    cursor.expect(TokenKind::LParen)?;
    let mut record = parse_record_fields(cursor, spec, depth)?;
    cursor.expect(TokenKind::RParen)?;
    record.fields.insert(name_field.id, FieldValue::Text(name));
    Ok(record)
}

/// 🧾️ Parses a record's fields: positional fields in declaration order, then order-
/// independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a key), until a
/// token that is neither a known key nor an unfilled positional slot — which ends the record (it
/// belongs to whatever comes next: a new statement, a closing brace/paren, or EOF). Excludes any
/// field marked `call_name()` from both candidate sets: that field is consumed by the caller
/// (`RecordLayout::Call`'s `<name> =` prefix) before this function ever runs, for a Call-layout
/// spec, and no field is ever marked `call_name()` under any other layout.
fn parse_record_fields(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let mut record = RecordValue::default();
    let positional: Vec<&FieldSpec> = {
        let mut p: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
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
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block` fields are also excluded from the `key=value` loop below: their own key acts as a
    // bare leading keyword (`children { ... }`, no `=`) — `Table` fields (bare `key [...] {...}`
    // SoA form) are handled the same way, via their own lookahead branch below.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name && !matches!(f.shape, Shape::Statements(_))).collect();

    loop {
        if let Some(key) = cursor.dynamic_attr_key()? {
            let Some(index) = keyed.iter().position(|f| !matches!(f.shape, Shape::Block(_)) && f.key == key) else { break };
            let field = keyed.remove(index);
            cursor.advance();
            cursor.expect(TokenKind::Equals)?;
            let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
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
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }
    for field in keyed {
        if !record.fields.contains_key(&field.id){record.fields.insert(field.id,FieldValue::Absent);}
    }

    if let Some(field) = statements_field {
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    Ok(record)
}

// 🚫️async: E1 pure lookahead over the now-sync `Cursor`, called inline in a plain `if` with no
// await anywhere at its one call site — see R9
fn can_start_positional(cursor: &Cursor, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) | Shape::Dim(_) => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        Shape::Ref(_) => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Count => cursor.peek().kind == TokenKind::Ident,
        Shape::Coord(_) => cursor.peek().kind == TokenKind::At,
        Shape::Dir => cursor.peek().kind == TokenKind::Caret,
        Shape::Range | Shape::Expr => cursor.peek().kind == TokenKind::LParen,
        Shape::Embed(_) | Shape::EmbedFrom(_) => matches!(cursor.peek().kind, TokenKind::Fence | TokenKind::Text | TokenKind::Placeholder),
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

//#region 🔖️Table
/// 🚧️ Which shapes have a fixed/bounded token extent and may therefore be a `Table`
/// column: an unbounded `Tuple` (`len: None`, comma-separated until... forever) and `Statements`
/// (repeats until a non-matching keyword) both need an external delimiter to know where they end
/// — fine inside `[ ]`/`{ }` brackets, fatal inside a table row where the ONLY thing marking a
/// row boundary is "we've now read exactly `columns.len()` values".
// 🚫️async: E1 pure, inlined directly into `format!` args alongside `shape_type_name` (Display,
// not Future) — see R9
fn shape_is_self_delimiting(shape: &Shape) -> bool {
    !matches!(shape, Shape::Statements(_) | Shape::Tuple(_, None))
}

/// 🚧️ Spec-build-time validation for a `Table`'s element `RecordSpec` — called wherever a
/// `Shape::Table(spec_fn)` is first evaluated (both parse paths, and printing), since `spec_fn` is
/// a lazy pointer rather than an eagerly-built value there is no earlier moment to check it at.
fn validate_table_columns(spec: &RecordSpec) -> Result<(), TextError> {
    for field in &spec.fields {
        if !shape_is_self_delimiting(&field.shape) {
            return Err(TextError::new(ValueRefusalKind::UnsupportedOwner,format!("table column '{}' has a non-self-delimiting shape ({}) and cannot be a table column", field.key, shape_type_name(&field.shape)),TextSpan::at(1, 1)));
        }
    }
    Ok(())
}

/// 🏷️ UPPERCASE schema type tag for a `Shape` — what a `Table` header prints per column
/// (`id:TEXT`), per the unified syntax law (`UPPERCASE` for engine shapes, `PascalCase` reserved
/// for technology-declared domain kinds).
// 🚫️async: E1 pure, inlined directly into `format!` args at both call sites (Display, not Future) — see R9
/// 📊️ Parses the bare SoA form of a `Table` field: `[col:TYPE ...] { v11 v12 ...  v21 v22
/// ... }`, cursor positioned right after the field's own keyword has already been consumed. The
/// header names columns (in the order values then appear per row); a `:TYPE` suffix is accepted
/// but not required to resolve a column (it's a human/printer-facing tag, not load-bearing for
/// parsing — the column's real shape always comes from the element `RecordSpec`), which is what
/// lets a hand-written header omit types the engine can already infer. Rows have NO separator —
/// reading exactly `columns.len()` values per row is what makes a row self-delimiting, which is
/// also why every column shape must itself be self-delimiting (`validate_table_columns`).
fn parse_table_soa(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    let element_spec = (spec_fn.ordinary)();
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
        let field_spec = element_spec.fields.iter().find(|f| f.key == key).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("unknown table column '{key}'"),key_token.span))?;
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
            let value = parse_table_cell(cursor, &field_spec.shape, depth + 1)?;
            record.fields.insert(field_spec.id, value);
        }
        for field_spec in &element_spec.fields {
            if !record.fields.contains_key(&field_spec.id){record.fields.insert(field_spec.id,FieldValue::Absent);}
        }
        rows.push(FieldValue::Record(record));
        cursor.limits.check_nodes(rows.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBrace)?;
    Ok(FieldValue::List(rows))
}

/// 🧱️ Reads one table cell's value. Every table-safe shape is bounded by its own bracket or
/// a fixed token count (`validate_table_columns`/`shape_is_self_delimiting`) — EXCEPT a bare
/// `Shape::Record` column, which prints as a flat run of `key=value` tokens with no bracket of its
/// own (a table row has no `field=` prefix to give it one, unlike a Record-shaped field elsewhere).
/// Two adjacent columns of the SAME record type (or any two types sharing a field name) are then
/// genuinely ambiguous: `parse_record_body`'s keyed loop for column N keeps matching `key=value`
/// tokens for as long as the key is one of ITS OWN not-yet-filled fields, so a column-N field left
/// absent (never printed) silently lets column N's parse run on and swallow column N+1's
/// same-named token instead of stopping at the column boundary. Braced here for exactly that
/// reason — every other shape already round-trips through the ordinary `parse_shape`.
fn parse_table_cell(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::Record(spec_fn) = shape {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        return Ok(FieldValue::Record(record));
    }
    parse_shape(cursor, shape, depth)
}

/// 📋️ The AoS-list form for a `Table` value reached anywhere other than a record's own
/// leading keyword-prefixed field: `[ {row-fields} {row-fields} ... ]`. Each row is brace-wrapped
/// for the same reason a `Shape::Record` table COLUMN is (`parse_table_cell` above) — a table row
/// type is commonly declared with no keyword of its own (a header already gives every row its
/// column order, so SoA rows don't need one), so without a bracket of its own, one row's absent
/// field could let its parse run on into the next row's same-named token exactly like the
/// column-vs-column case. Bracing every row here removes that ambiguity regardless of whether the
/// row type happens to declare a keyword or not.
fn parse_table_list(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    cursor.expect(TokenKind::LBracket)?;
    let mut items = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        items.push(FieldValue::Record(record));
        cursor.limits.check_nodes(items.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBracket)?;
    Ok(FieldValue::List(items))
}

/// 📋️ Prints the braced AoS-list form `parse_table_list` reads back. Ordinary `[ ]` spacing
/// (a space just inside, per the general list rule — NOT the header's own tight-glued exception).
//#endregion 🔖️Table

fn base64_decode(text:&str)->Result<Vec<u8>,String>{semio_framework_value::bytes::decode_base64(text)}
//#endregion 🔖️Parser

//#region 🔖️Canonicalize
/// ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`: reprints whatever `parse`
/// produces from `text`, which is the fixpoint every technology's `print_dsl` output must already
/// be at (the round-trip law), so this doubles as the idempotence check.
pub fn canonicalize(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<String, TextError> {
    let value = parse(text, spec, opts)?;
    Ok(print(&value, spec, JoinMode::Document))
}
//#endregion 🔖️Canonicalize

//#region 🔖️Language
/// 🎨️ Generic editor surface over any `RecordSpec` — the generalization of
/// `math::graph::dsl`'s hand-rolled `LanguageService`.
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
        semio_framework_dsl::token_classes(&tokens, &keyword_refs)
    }

    pub fn diagnostics(&self, text: &str) -> Vec<TextError> {
        match parse(text, self.spec, &ParseOptions::default()) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e],
        }
    }

    /// 💡️ Completions at `offset`: every key not yet used in the record enclosing the
    /// cursor, plus every keyword reachable from the root. A simple, always-available baseline —
    /// full context-sensitive narrowing is a natural follow-up once `Cst` gains node addressing.
    pub fn completions(&self, _text: &str, _offset: usize) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = self.spec.fields.iter().filter(|f| !f.key.is_empty()).map(|f| CompletionItem { label: f.key.clone(), detail: Some(format!("{:?}", f.shape)) }).collect();
        for keyword in self.keywords() {
            items.push(CompletionItem { label: keyword, detail: None });
        }
        items
    }
}

// 🚫️async: E1 pure tree walk, mutually recursive with `collect_shape_keywords` below through match
// arms whose tail expression must resolve to the same `()` type in every arm — see R9
fn collect_keywords(spec: &RecordSpec, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    if let Some(kw) = &spec.keyword {
        out.push(kw.clone());
    }
    for field in &spec.fields {
        collect_shape_keywords(&field.shape, out, seen, seen_records);
    }
}

/// 🔁️ `seen` guards against a genuinely self-referential `Statements` table (a recursive
/// block tree whose own variant list contains itself): each `(spec_fn.ordinary)()` call is only expanded the
/// first time its keyword is reached, so the keyword set — which is always finite, even when the
/// grammar's real nesting isn't — is collected exactly once instead of infinitely. `seen_records`
/// is the same guard for a self-referential `Shape::Record` (a `#[derive(DslRecord)]` struct field
/// whose type recurses back to itself, e.g. a dynamic-value type nesting a map of itself) — a bare
/// Record has no keyword to key on, so this tracks the `fn() -> RecordSpec` pointer's own address
/// instead (two calls to the same generated `__dsl_spec` always share one code address).
// 🚫️async: E1 pure, same mutual-recursion R9 case as `collect_keywords` above
fn collect_shape_keywords(shape: &Shape, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    match shape {
        Shape::Record(spec_fn) => {
            if seen_records.insert(spec_fn.ordinary as usize) {
                collect_keywords(&(spec_fn.ordinary)(), out, seen, seen_records);
            }
        }
        Shape::Block(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        Shape::Statements(variants) => {
            for (kw, spec_fn) in variants {
                out.push(kw.clone());
                if seen.insert(kw.clone()) {
                    collect_keywords(&(spec_fn.ordinary)(), out, seen, seen_records);
                }
            }
        }
        Shape::List(inner) | Shape::Tuple(inner, _) | Shape::Map(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        _ => {}
    }
}
//#endregion 🔖️Language

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests





#[cfg(test)]
#[path = "🧪️tests/⚠️refusal/🦀️.rs"]
mod refusal_tests;

#[cfg(test)]
#[path = "🧪️tests/🪆️refusal/🦀️.rs"]
mod field_refusal_tests;

#[cfg(test)]
#[path = "🧪️tests/🔢️number-refusal/🦀️.rs"]
mod number_refusal_tests;

#[path="🪆️binding/🎮️operation/🦀️.rs"]
pub mod variants_text;
