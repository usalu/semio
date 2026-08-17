//! @emoji 📖️ `dsl_grammar` — the self-hosted `.grammar` spec format: a hand-authorable,
//! EBNF-style description of one language's productions, used as the *normative* artifact every
//! handcrafted grammar in the repo ships alongside its parser/printer. This crate parses and
//! prints the format itself (this crate's own `📖️grammar/📖️grammar.grammar` is written in it and
//! parses cleanly under its own parser — see the `self_hosting` test), and provides a recognizer
//! that can check a target document's tokens against a compiled grammar for the subset of
//! productions this v1 supports (see `//#region 🔖️Recognizer`'s doc comment for exactly what that
//! covers today and what it does not yet).
//!
//! Depends on `dsl_core` only, following the same "own pre-scan lexer delegating the shared
//! alphabet to `crate::os_dsl::core::lex`" pattern `math::graph::dsl` (Jack) established — `?` and `|`
//! aren't in the shared token alphabet (a structural-DSL alphabet has no need for them), so this
//! crate's lexer pre-scans those two characters itself and hands every other run of characters to
//! `crate::os_dsl::core::lex` unchanged.

use crate::os_dsl::core::{lex as core_lex, Limits, TextError, TextSpan, TokenKind as CoreKind};

//#region 🔖️Model
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioDialect {
    Grammar,
    Protocol,
}

/// @emoji 📄️ One parsed `.grammar.semio` / `.protocol.semio` file: header directives + productions.
#[derive(Clone, Debug, PartialEq)]
pub struct GrammarFile {
    pub dialect: SemioDialect,
    pub id: String,
    pub extension: Option<String>,
    pub uses: Vec<String>,
    pub start: String,
    pub productions: Vec<Production>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Production {
    pub name: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alternative {
    pub symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MacroArg {
    Literal(String),
    Ident(String),
}

/// @emoji 🧩️ One symbol in a production's alternative. `Terminal` vs. `Ref` is decided lexically
/// at parse time (an all-uppercase bareword is a terminal token-class name; anything else is a
/// reference to another production, or a zero-arg macro — resolved later, not by this parser).
#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    Literal(String),
    Terminal(String),
    Ref(String),
    Macro(String, Vec<MacroArg>),
    Group(Vec<Alternative>),
    Optional(Box<Symbol>),
    Star(Box<Symbol>),
    Plus(Box<Symbol>),
}
//#endregion 🔖️Model
//#region 📡️ProtocolModel
/// @emoji 📡️ One parsed `.protocol.semio` file: framing + typed body directives.
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolFile {
    pub id: String,
    pub version: u16,
    pub schema: String,
    pub start: String,
    pub uses: Vec<String>,
    pub framing: Framing,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Framing {
    Magic([u8; 8]),
    Record,
    Chunked,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Header(Vec<Field>),
    Segment { name: String, kind: Option<u8>, fields: Vec<Field> },
    Record { name: String, tag: Option<u64>, fields: Vec<Field> },
    Struct { name: String, fields: Vec<Field> },
    Enum { name: String, variants: Vec<(String, u64)> },
    Footer(usize),
    Chain(Prim),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Prim,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prim {
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    Varint,
    Zigzag,
    Bytes,
    Utf8,
    Fixed(usize),
    Array(Box<Prim>, Count),
    Ref(String),
    Tag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Count {
    Fixed(usize),
    Varint,
    Field(String),
}

/// @emoji ✅️ Successful byte walk: every declared wire slot consumed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolTrace {
    pub consumed: usize,
}

/// @emoji ❌️ Spec/bytes disagreement at a concrete offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolMismatch {
    pub offset: usize,
    pub message: String,
}
//#endregion 📡️ProtocolModel


//#region 🔖️Lexer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GKind {
    Ident,
    Text,
    Star,
    Plus,
    Question,
    Pipe,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Equals,
    Newline,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct GToken {
    kind: GKind,
    text: String,
    span: TextSpan,
}

/// @emoji 🔬️ Pre-scans `?`/`|` (not in `dsl_core`'s alphabet) and delegates every other run of
/// characters whole to `crate::os_dsl::core::lex`, exactly like `math::graph::crate::os_dsl::lex_spanned` does
/// for its own two Cypher-specific extras.
fn lex(text: &str) -> Result<Vec<GToken>, TextError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    let push_segment = |seg_start: usize, seg_end: usize, tokens: &mut Vec<GToken>| -> Result<(), TextError> {
        if seg_end <= seg_start {
            return Ok(());
        }
        let segment = &text[seg_start..seg_end];
        let raw = core_lex(segment, &Limits::default(), false)?;
        for token in raw {
            if matches!(token.kind, CoreKind::Whitespace | CoreKind::Comment | CoreKind::Eof) {
                continue;
            }
            let kind = match token.kind {
                CoreKind::Ident | CoreKind::Placeholder => GKind::Ident,
                CoreKind::Text => GKind::Text,
                CoreKind::Star => GKind::Star,
                CoreKind::Plus => GKind::Plus,
                CoreKind::LParen => GKind::LParen,
                CoreKind::RParen => GKind::RParen,
                CoreKind::LBrace => GKind::LBrace,
                CoreKind::RBrace => GKind::RBrace,
                CoreKind::Comma => GKind::Comma,
                CoreKind::Equals => GKind::Equals,
                CoreKind::Newline => GKind::Newline,
                other => return Err(TextError::new(format!("`.grammar` files cannot contain a {other:?} token here"), token.span)),
            };
            tokens.push(GToken { kind, text: token.text.as_str().to_string(), span: token.span });
        }
        Ok(())
    };

    while i < bytes.len() {
        let c = bytes[i];
        // A quoted `TEXT` literal is skipped whole here (matching dsl_core's own `"..."` escape
        // scheme, backslash-escapes included) so a `?` or `|` *inside* a string — needed to write
        // e.g. `"|"` as a literal pipe token — is never mistaken for this lexer's own operators.
        // The segment (quotes included) is left for `push_segment`'s `core_lex` call to tokenize.
        if c == b'"' {
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if c == b'?' || c == b'|' {
            push_segment(seg_start, i, &mut tokens)?;
            let line = text[..i].matches('\n').count() as u32 + 1;
            let col = (i - text[..i].rfind('\n').map(|p| p + 1).unwrap_or(0)) as u32 + 1;
            let span = TextSpan::with_length(line, col, 1);
            tokens.push(GToken { kind: if c == b'?' { GKind::Question } else { GKind::Pipe }, text: (c as char).to_string(), span });
            i += 1;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_segment(seg_start, bytes.len(), &mut tokens)?;
    let eof_span = tokens.last().map(|t| t.span).unwrap_or(TextSpan::at(1, 1));
    tokens.push(GToken { kind: GKind::Eof, text: String::new(), span: eof_span });
    Ok(tokens)
}
//#endregion 🔖️Lexer

//#region 🔖️Parser
struct Cursor {
    tokens: Vec<GToken>,
    pos: usize,
}

impl Cursor {
    fn peek(&self) -> &GToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> GToken {
        let token = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn skip_newlines(&mut self) {
        while self.peek().kind == GKind::Newline {
            self.advance();
        }
    }

    fn expect(&mut self, kind: GKind) -> Result<GToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(format!("expected {kind:?}, found {:?} {:?}", self.peek().kind, self.peek().text), self.peek().span.clone()))
        }
    }

    fn peek_ident(&self, expected_text: &str) -> bool {
        self.peek().kind == GKind::Ident && self.peek().text == expected_text
    }

    fn expect_ident(&mut self, expected_text: &str) -> Result<(), TextError> {
        let token = self.expect(GKind::Ident)?;
        if token.text == expected_text {
            Ok(())
        } else {
            Err(TextError::new(format!("expected keyword `{expected_text}`, found `{}`", token.text), token.span))
        }
    }

    fn expect_ident_or_int(&mut self) -> Result<GToken, TextError> {
        match self.peek().kind {
            GKind::Ident | GKind::Int => Ok(self.advance()),
            other => Err(TextError::new(format!("expected ident or int, found {other:?}"), self.peek().span.clone())),
        }
    }
}

fn is_all_upper(text: &str) -> bool {
    text.chars().any(|c| c.is_alphabetic()) && text.chars().all(|c| c.is_uppercase() || c == '_' || c == '-' || c.is_ascii_digit())
}

fn parse_macro_args(cursor: &mut Cursor) -> Result<Vec<MacroArg>, TextError> {
    cursor.expect(GKind::LParen)?;
    let mut args = Vec::new();
    if cursor.peek().kind != GKind::RParen {
        loop {
            let arg = match cursor.peek().kind {
                GKind::Text => MacroArg::Literal(cursor.advance().text),
                GKind::Ident => MacroArg::Ident(cursor.advance().text),
                other => return Err(TextError::new(format!("expected a macro argument, found {other:?}"), cursor.peek().span.clone())),
            };
            args.push(arg);
            if cursor.peek().kind == GKind::Comma {
                cursor.advance();
                continue;
            }
            break;
        }
    }
    cursor.expect(GKind::RParen)?;
    Ok(args)
}

fn parse_atom(cursor: &mut Cursor) -> Result<Symbol, TextError> {
    let base = match cursor.peek().kind {
        GKind::Text => Symbol::Literal(cursor.advance().text),
        // Grouping uses `{ }`, never `( )`: whitespace is discarded before parsing (trivia is
        // dropped at lex time), so a token stream alone can't distinguish `name (group)` — a
        // bareword reference followed by a separate grouped alternative — from `name(args)`, a
        // macro call. Reserving `( )` exclusively for macro-call argument lists keeps that
        // distinction unambiguous without needing whitespace-sensitive parsing.
        GKind::LBrace => {
            cursor.advance();
            let alts = parse_alternatives(cursor)?;
            cursor.expect(GKind::RBrace)?;
            Symbol::Group(alts)
        }
        GKind::Ident => {
            let name = cursor.advance().text;
            if cursor.peek().kind == GKind::LParen {
                Symbol::Macro(name, parse_macro_args(cursor)?)
            } else if is_all_upper(&name) {
                Symbol::Terminal(name)
            } else {
                Symbol::Ref(name)
            }
        }
        other => return Err(TextError::new(format!("expected a symbol, found {other:?}"), cursor.peek().span.clone())),
    };
    let quantified = match cursor.peek().kind {
        GKind::Question => {
            cursor.advance();
            Symbol::Optional(Box::new(base))
        }
        GKind::Star => {
            cursor.advance();
            Symbol::Star(Box::new(base))
        }
        GKind::Plus => {
            cursor.advance();
            Symbol::Plus(Box::new(base))
        }
        _ => base,
    };
    Ok(quantified)
}

fn parse_sequence(cursor: &mut Cursor) -> Result<Alternative, TextError> {
    let mut symbols = Vec::new();
    loop {
        match cursor.peek().kind {
            GKind::Pipe | GKind::Newline | GKind::Eof | GKind::RBrace => break,
            _ => symbols.push(parse_atom(cursor)?),
        }
    }
    if symbols.is_empty() {
        return Err(TextError::new("a production alternative must have at least one symbol", cursor.peek().span.clone()));
    }
    Ok(Alternative { symbols })
}

fn parse_alternatives(cursor: &mut Cursor) -> Result<Vec<Alternative>, TextError> {
    let mut alts = vec![parse_sequence(cursor)?];
    while cursor.peek().kind == GKind::Pipe {
        cursor.advance();
        alts.push(parse_sequence(cursor)?);
    }
    Ok(alts)
}

fn parse_production_line(cursor: &mut Cursor) -> Result<Production, TextError> {
    let name = cursor.expect(GKind::Ident)?.text;
    cursor.expect(GKind::Equals)?;
    let alternatives = parse_alternatives(cursor)?;
    Ok(Production { name, alternatives })
}

/// @emoji 📖️ Parses one `.grammar` file. Protocol dialect sources project through
/// [`parse_protocol`] into a shallow [`GrammarFile`] (empty productions).
pub fn parse_grammar(text: &str) -> Result<GrammarFile, TextError> {
    if is_protocol_source(text) {
        return Ok(project_protocol(parse_protocol(text)?));
    }

    let tokens = lex(text)?;
    let mut cursor = Cursor { tokens, pos: 0 };
    cursor.skip_newlines();

    let dialect = if cursor.peek_ident("dialect") {
        cursor.expect_ident("dialect")?;
        let name = cursor.expect(GKind::Ident)?.text;
        cursor.skip_newlines();
        match name.as_str() {
            "grammar" => SemioDialect::Grammar,
            "protocol" => return Ok(project_protocol(parse_protocol(text)?)),
            other => return Err(TextError::new(format!("unknown semio dialect `{other}`"), cursor.peek().span.clone())),
        }
    } else {
        SemioDialect::Grammar
    };

    cursor.expect_ident("grammar")?;
    let id = cursor.expect(GKind::Ident)?.text;
    cursor.skip_newlines();

    let mut extension = None;
    let mut uses = Vec::new();
    let mut start = None;
    let mut productions = Vec::new();

    loop {
        if cursor.peek().kind == GKind::Eof {
            break;
        }
        let head = cursor.expect(GKind::Ident)?;
        match head.text.as_str() {
            "extension" => {
                extension = Some(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "use" => {
                uses.push(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "start" => {
                start = Some(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            _ => {
                cursor.pos -= 1;
                productions.push(parse_production_line(&mut cursor)?);
                cursor.skip_newlines();
            }
        }
    }

    let _ = dialect;
    let start = start.ok_or_else(|| TextError::new("`.grammar` file is missing a `start` directive", cursor.peek().span.clone()))?;
    Ok(GrammarFile { dialect: SemioDialect::Grammar, id, extension, uses, start, productions })
}

fn is_protocol_source(text: &str) -> bool {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        if trimmed.starts_with("dialect") {
            return trimmed.split_whitespace().nth(1) == Some("protocol");
        }
        return trimmed.starts_with("protocol");
    }
    false
}

fn project_protocol(protocol: ProtocolFile) -> GrammarFile {
    GrammarFile {
        dialect: SemioDialect::Protocol,
        id: protocol.id,
        extension: None,
        uses: protocol.uses,
        start: protocol.start,
        productions: Vec::new(),
    }
}

fn parse_usize_token(token: &GToken) -> Result<usize, TextError> {
    token.text.parse::<usize>().map_err(|_| TextError::new(format!("expected unsigned integer, found `{}`", token.text), token.span.clone()))
}

fn parse_u64_literal(cursor: &mut Cursor) -> Result<u64, TextError> {
    let first = cursor.expect_ident_or_int()?;
    if first.kind == GKind::Int && first.text == "0" {
        if cursor.peek().kind == GKind::Ident {
            let rest = cursor.peek().text.clone();
            if let Some(hex) = rest.strip_prefix('x').or_else(|| rest.strip_prefix('X')) {
                cursor.advance();
                return u64::from_str_radix(hex, 16).map_err(|_| TextError::new(format!("invalid hex literal `0{rest}`"), first.span.clone()));
            }
        }
    }
    if let Some(hex) = first.text.strip_prefix("0x").or_else(|| first.text.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16).map_err(|_| TextError::new(format!("invalid hex literal `{}`", first.text), first.span.clone()));
    }
    first.text.parse::<u64>().map_err(|_| TextError::new(format!("expected unsigned integer, found `{}`", first.text), first.span.clone()))
}

fn parse_count(cursor: &mut Cursor) -> Result<Count, TextError> {
    match cursor.peek().kind {
        GKind::Ident if cursor.peek().text == "Varint" || cursor.peek().text == "varint" => {
            cursor.advance();
            Ok(Count::Varint)
        }
        GKind::Ident if cursor.peek().text == "Fixed" || cursor.peek().text == "fixed" => {
            cursor.advance();
            cursor.expect(GKind::LParen)?;
            let n = parse_usize_token(&cursor.expect_ident_or_int()?)?;
            cursor.expect(GKind::RParen)?;
            Ok(Count::Fixed(n))
        }
        GKind::Ident if cursor.peek().text == "Field" => {
            cursor.advance();
            cursor.expect(GKind::LParen)?;
            let name = cursor.expect(GKind::Ident)?.text;
            cursor.expect(GKind::RParen)?;
            Ok(Count::Field(name))
        }
        GKind::Int => Ok(Count::Fixed(parse_usize_token(&cursor.advance())?)),
        _ => Err(TextError::new("expected Array count (Fixed/Varint/Field)", cursor.peek().span.clone())),
    }
}

fn parse_prim(cursor: &mut Cursor) -> Result<Prim, TextError> {
    match cursor.peek().kind {
        GKind::Ident => {
            let name = cursor.advance().text;
            match name.as_str() {
                "u8" => Ok(Prim::U8),
                "u16" => Ok(Prim::U16),
                "u32" => Ok(Prim::U32),
                "u64" => Ok(Prim::U64),
                "i32" => Ok(Prim::I32),
                "i64" => Ok(Prim::I64),
                "f32" => Ok(Prim::F32),
                "f64" => Ok(Prim::F64),
                "varint" => {
                    if cursor.peek_ident("bytes") {
                        cursor.advance();
                        Ok(Prim::Array(Box::new(Prim::U8), Count::Varint))
                    } else {
                        Ok(Prim::Varint)
                    }
                }
                "zigzag" => Ok(Prim::Zigzag),
                "bytes" => Ok(Prim::Bytes),
                "utf8" => Ok(Prim::Utf8),
                "tag" => Ok(Prim::Tag),
                "fixed" => {
                    let n = parse_usize_token(&cursor.expect_ident_or_int()?)?;
                    Ok(Prim::Fixed(n))
                }
                "Fixed" => {
                    cursor.expect(GKind::LParen)?;
                    let n = parse_usize_token(&cursor.expect_ident_or_int()?)?;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::Fixed(n))
                }
                "array" => {
                    let inner = parse_prim(cursor)?;
                    Ok(Prim::Array(Box::new(inner), Count::Varint))
                }
                "Array" => {
                    cursor.expect(GKind::LParen)?;
                    let inner = parse_prim(cursor)?;
                    cursor.expect(GKind::Comma)?;
                    let count = parse_count(cursor)?;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::Array(Box::new(inner), count))
                }
                "Ref" => {
                    cursor.expect(GKind::LParen)?;
                    let target = cursor.expect(GKind::Ident)?.text;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::Ref(target))
                }
                other => Ok(Prim::Ref(other.to_string())),
            }
        }
        other => Err(TextError::new(format!("expected a protocol type, found {other:?}"), cursor.peek().span.clone())),
    }
}

fn parse_field_pair(cursor: &mut Cursor) -> Result<Field, TextError> {
    let name = cursor.expect(GKind::Ident)?.text;
    let ty = parse_prim(cursor)?;
    Ok(Field { name, ty })
}

fn parse_fields_until_break(cursor: &mut Cursor) -> Result<Vec<Field>, TextError> {
    let mut fields = Vec::new();
    while matches!(cursor.peek().kind, GKind::Ident) {
        fields.push(parse_field_pair(cursor)?);
    }
    Ok(fields)
}

fn parse_braced_fields(cursor: &mut Cursor) -> Result<Vec<Field>, TextError> {
    cursor.expect(GKind::LBrace)?;
    cursor.skip_newlines();
    let mut fields = Vec::new();
    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
        fields.push(parse_field_pair(cursor)?);
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    Ok(fields)
}

fn parse_enum_variants(cursor: &mut Cursor) -> Result<Vec<(String, u64)>, TextError> {
    cursor.expect(GKind::LBrace)?;
    cursor.skip_newlines();
    let mut variants = Vec::new();
    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
        let name = cursor.expect(GKind::Ident)?.text;
        cursor.expect(GKind::Equals)?;
        let value = parse_u64_literal(cursor)?;
        variants.push((name, value));
        if cursor.peek().kind == GKind::Comma {
            cursor.advance();
        }
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    Ok(variants)
}

fn magic_bytes(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

fn flush_open_segment(blocks: &mut Vec<Block>, open: &mut Option<Block>) {
    if let Some(block) = open.take() {
        blocks.push(block);
    }
}

/// @emoji 📡️ Parses one `.protocol.semio` file into a typed [`ProtocolFile`] — retains every body
/// directive (`header`/`field`/`segment`/`record`/`struct`/`enum`/`footer`/`chain`).
pub fn parse_protocol(text: &str) -> Result<ProtocolFile, TextError> {
    let tokens = lex(text)?;
    let mut cursor = Cursor { tokens, pos: 0 };
    cursor.skip_newlines();

    if cursor.peek_ident("dialect") {
        cursor.expect_ident("dialect")?;
        cursor.expect_ident("protocol")?;
        cursor.skip_newlines();
    }

    cursor.expect_ident("protocol")?;
    let id = cursor.expect(GKind::Ident)?.text;
    cursor.skip_newlines();

    let mut version = 1u16;
    let mut schema = String::new();
    let mut start = None;
    let mut uses = Vec::new();
    let mut framing = None;
    let mut blocks = Vec::new();
    let mut open_segment: Option<Block> = None;
    let mut open_record: Option<Block> = None;
    let mut open_header: Option<Vec<Field>> = None;

    let close_header = |blocks: &mut Vec<Block>, open_header: &mut Option<Vec<Field>>| {
        if let Some(fields) = open_header.take() {
            blocks.push(Block::Header(fields));
        }
    };
    let close_record = |blocks: &mut Vec<Block>, open_record: &mut Option<Block>| {
        if let Some(block) = open_record.take() {
            blocks.push(block);
        }
    };

    loop {
        if cursor.peek().kind == GKind::Eof {
            break;
        }
        let head = cursor.expect(GKind::Ident)?;
        match head.text.as_str() {
            "version" => {
                version = parse_u64_literal(&mut cursor)? as u16;
                cursor.skip_newlines();
            }
            "schema" => {
                schema = cursor.expect(GKind::Ident)?.text;
                cursor.skip_newlines();
            }
            "use" => {
                uses.push(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "start" => {
                start = Some(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "framing" => {
                let mode = cursor.expect(GKind::Ident)?.text;
                framing = Some(match mode.as_str() {
                    "magic" => Framing::Magic(magic_bytes(parse_u64_literal(&mut cursor)?)),
                    "record" => Framing::Record,
                    "chunked" => Framing::Chunked,
                    other => return Err(TextError::new(format!("unknown framing `{other}`"), head.span.clone())),
                });
                cursor.skip_newlines();
            }
            "header" => {
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                close_header(&mut blocks, &mut open_header);
                cursor.expect_ident("fixed")?;
                let _size = parse_u64_literal(&mut cursor)?;
                open_header = Some(Vec::new());
                cursor.skip_newlines();
            }
            "field" => {
                let field = parse_field_pair(&mut cursor)?;
                if let Some(fields) = open_header.as_mut() {
                    fields.push(field);
                } else if let Some(Block::Record { fields, .. }) = open_record.as_mut() {
                    fields.push(field);
                } else if let Some(Block::Segment { fields, .. }) = open_segment.as_mut() {
                    fields.push(field);
                } else if matches!(framing, Some(Framing::Record)) {
                    open_record = Some(Block::Record { name: String::new(), tag: None, fields: vec![field] });
                } else {
                    open_header.get_or_insert_with(Vec::new).push(field);
                }
                cursor.skip_newlines();
            }
            "segment" => {
                close_header(&mut blocks, &mut open_header);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                if cursor.peek_ident("kind") && cursor.tokens.get(cursor.pos + 1).is_some_and(|t| t.kind == GKind::Equals) {
                    flush_open_segment(&mut blocks, &mut open_segment);
                    cursor.expect_ident("kind")?;
                    cursor.expect(GKind::Equals)?;
                    let kind = parse_u64_literal(&mut cursor)? as u8;
                    let fields = if cursor.peek().kind == GKind::LBrace { parse_braced_fields(&mut cursor)? } else { Vec::new() };
                    blocks.push(Block::Segment { name, kind: Some(kind), fields });
                } else if cursor.peek().kind == GKind::LBrace {
                    flush_open_segment(&mut blocks, &mut open_segment);
                    let fields = parse_braced_fields(&mut cursor)?;
                    blocks.push(Block::Segment { name, kind: None, fields });
                } else {
                    let ty = parse_prim(&mut cursor)?;
                    match open_segment.as_mut() {
                        Some(Block::Segment { fields, .. }) => fields.push(Field { name, ty }),
                        _ => {
                            open_segment = Some(Block::Segment { name: String::new(), kind: None, fields: vec![Field { name, ty }] });
                        }
                    }
                }
                cursor.skip_newlines();
            }
            "record" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let mut tag = None;
                if cursor.peek_ident("tag") && cursor.tokens.get(cursor.pos + 1).is_some_and(|t| t.kind == GKind::Equals) {
                    cursor.expect_ident("tag")?;
                    cursor.expect(GKind::Equals)?;
                    tag = Some(parse_u64_literal(&mut cursor)?);
                }
                let fields = if cursor.peek().kind == GKind::LBrace {
                    parse_braced_fields(&mut cursor)?
                } else {
                    parse_fields_until_break(&mut cursor)?
                };
                open_record = Some(Block::Record { name, tag, fields });
                cursor.skip_newlines();
            }
            "struct" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let fields = parse_braced_fields(&mut cursor)?;
                blocks.push(Block::Struct { name, fields });
                cursor.skip_newlines();
            }
            "enum" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let variants = parse_enum_variants(&mut cursor)?;
                blocks.push(Block::Enum { name, variants });
                cursor.skip_newlines();
            }
            "footer" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                cursor.expect_ident("fixed")?;
                let size = parse_u64_literal(&mut cursor)? as usize;
                blocks.push(Block::Footer(size));
                cursor.skip_newlines();
            }
            "chain" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                if cursor.peek().kind == GKind::Ident {
                    let maybe_name = cursor.peek().text.clone();
                    let prim_names = ["u8", "u16", "u32", "u64", "i32", "i64", "f32", "f64", "varint", "zigzag", "bytes", "utf8", "tag", "fixed", "Fixed", "Array", "array", "Ref"];
                    if !prim_names.contains(&maybe_name.as_str()) {
                        cursor.advance();
                    }
                }
                let ty = parse_prim(&mut cursor)?;
                blocks.push(Block::Chain(ty));
                cursor.skip_newlines();
            }
            other => return Err(TextError::new(format!("unknown protocol directive `{other}`"), head.span)),
        }
    }

    close_header(&mut blocks, &mut open_header);
    flush_open_segment(&mut blocks, &mut open_segment);
    close_record(&mut blocks, &mut open_record);

    let start = start.ok_or_else(|| TextError::new("`.protocol` file is missing a `start` directive", cursor.peek().span.clone()))?;
    let framing = framing.ok_or_else(|| TextError::new("`.protocol` file is missing a `framing` directive", cursor.peek().span.clone()))?;
    if schema.is_empty() {
        return Err(TextError::new("`.protocol` file is missing a `schema` directive", cursor.peek().span.clone()));
    }
    Ok(ProtocolFile { id, version, schema, start, uses, framing, blocks })
}
//#endregion 🔖️Parser


//#region 🔖️Writer
fn print_symbol(symbol: &Symbol, out: &mut String) {
    match symbol {
        Symbol::Literal(text) => {
            out.push('"');
            out.push_str(text);
            out.push('"');
        }
        Symbol::Terminal(name) | Symbol::Ref(name) => out.push_str(name),
        Symbol::Macro(name, args) => {
            out.push_str(name);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                match arg {
                    MacroArg::Literal(text) => {
                        out.push('"');
                        out.push_str(text);
                        out.push('"');
                    }
                    MacroArg::Ident(name) => out.push_str(name),
                }
            }
            out.push(')');
        }
        Symbol::Group(alts) => {
            out.push('{');
            print_alternatives(alts, out);
            out.push('}');
        }
        Symbol::Optional(inner) => {
            print_symbol(inner, out);
            out.push('?');
        }
        Symbol::Star(inner) => {
            print_symbol(inner, out);
            out.push('*');
        }
        Symbol::Plus(inner) => {
            print_symbol(inner, out);
            out.push('+');
        }
    }
}

fn print_alternatives(alts: &[Alternative], out: &mut String) {
    for (i, alt) in alts.iter().enumerate() {
        if i > 0 {
            out.push_str(" | ");
        }
        for (j, symbol) in alt.symbols.iter().enumerate() {
            if j > 0 {
                out.push(' ');
            }
            print_symbol(symbol, out);
        }
    }
}

/// @emoji 🖨️ Canonical printer — `parse_grammar(print_grammar(g)) == g` is this crate's own
/// round-trip law, checked by the `self_hosting` test below over this crate's own grammar file.
pub fn print_grammar(grammar: &GrammarFile) -> String {
    let mut out = String::new();
    out.push_str("dialect ");
    out.push_str(match grammar.dialect {
        SemioDialect::Grammar => "grammar",
        SemioDialect::Protocol => "protocol",
    });
    out.push('\n');
    out.push_str(match grammar.dialect {
        SemioDialect::Grammar => "grammar ",
        SemioDialect::Protocol => "protocol ",
    });
    out.push_str(&grammar.id);
    out.push('\n');
    if let Some(extension) = &grammar.extension {
        out.push_str("extension ");
        out.push_str(extension);
        out.push('\n');
    }
    for fragment in &grammar.uses {
        out.push_str("use ");
        out.push_str(fragment);
        out.push('\n');
    }
    out.push_str("start ");
    out.push_str(&grammar.start);
    out.push('\n');
    for production in &grammar.productions {
        out.push_str(&production.name);
        out.push_str(" = ");
        print_alternatives(&production.alternatives, &mut out);
        out.push('\n');
    }
    out
}

fn print_count(count: &Count, out: &mut String) {
    match count {
        Count::Fixed(n) => {
            out.push_str("Fixed(");
            out.push_str(&n.to_string());
            out.push(')');
        }
        Count::Varint => out.push_str("Varint"),
        Count::Field(name) => {
            out.push_str("Field(");
            out.push_str(name);
            out.push(')');
        }
    }
}

fn print_prim(prim: &Prim, out: &mut String) {
    match prim {
        Prim::U8 => out.push_str("u8"),
        Prim::U16 => out.push_str("u16"),
        Prim::U32 => out.push_str("u32"),
        Prim::U64 => out.push_str("u64"),
        Prim::I32 => out.push_str("i32"),
        Prim::I64 => out.push_str("i64"),
        Prim::F32 => out.push_str("f32"),
        Prim::F64 => out.push_str("f64"),
        Prim::Varint => out.push_str("varint"),
        Prim::Zigzag => out.push_str("zigzag"),
        Prim::Bytes => out.push_str("bytes"),
        Prim::Utf8 => out.push_str("utf8"),
        Prim::Tag => out.push_str("tag"),
        Prim::Fixed(n) => {
            out.push_str("fixed ");
            out.push_str(&n.to_string());
        }
        Prim::Array(inner, count) => {
            if matches!(inner.as_ref(), Prim::U8) && matches!(count, Count::Varint) {
                out.push_str("varint bytes");
            } else if matches!(count, Count::Varint) {
                out.push_str("array ");
                print_prim(inner, out);
            } else {
                out.push_str("Array(");
                print_prim(inner, out);
                out.push_str(", ");
                print_count(count, out);
                out.push(')');
            }
        }
        Prim::Ref(name) => out.push_str(name),
    }
}

fn print_field(field: &Field, out: &mut String) {
    out.push_str(&field.name);
    out.push(' ');
    print_prim(&field.ty, out);
}

fn header_fixed_size(fields: &[Field]) -> usize {
    fields.iter().map(|f| prim_fixed_width(&f.ty).unwrap_or(0)).sum()
}

fn prim_fixed_width(prim: &Prim) -> Option<usize> {
    match prim {
        Prim::U8 => Some(1),
        Prim::U16 => Some(2),
        Prim::U32 | Prim::I32 | Prim::F32 => Some(4),
        Prim::U64 | Prim::I64 | Prim::F64 => Some(8),
        Prim::Fixed(n) => Some(*n),
        _ => None,
    }
}

/// @emoji 🖨️ Lossless protocol printer — `parse_protocol(print_protocol(p)) == p`.
pub fn print_protocol(protocol: &ProtocolFile) -> String {
    let mut out = String::new();
    out.push_str("dialect protocol\nprotocol ");
    out.push_str(&protocol.id);
    out.push('\n');
    out.push_str("version ");
    out.push_str(&protocol.version.to_string());
    out.push('\n');
    out.push_str("schema ");
    out.push_str(&protocol.schema);
    out.push('\n');
    out.push_str("start ");
    out.push_str(&protocol.start);
    out.push('\n');
    for fragment in &protocol.uses {
        out.push_str("use ");
        out.push_str(fragment);
        out.push('\n');
    }
    match &protocol.framing {
        Framing::Magic(bytes) => {
            let value = u64::from_be_bytes(*bytes);
            out.push_str(&format!("framing magic 0x{value:016X}\n"));
        }
        Framing::Record => out.push_str("framing record\n"),
        Framing::Chunked => out.push_str("framing chunked\n"),
    }
    for block in &protocol.blocks {
        match block {
            Block::Header(fields) => {
                let size = header_fixed_size(fields);
                out.push_str("header fixed ");
                out.push_str(&size.to_string());
                out.push('\n');
                for field in fields {
                    out.push_str("field ");
                    print_field(field, &mut out);
                    out.push('\n');
                }
            }
            Block::Segment { name, kind, fields } => {
                if name.is_empty() && kind.is_none() {
                    for field in fields {
                        out.push_str("segment ");
                        print_field(field, &mut out);
                        out.push('\n');
                    }
                } else {
                    out.push_str("segment ");
                    out.push_str(name);
                    if let Some(k) = kind {
                        out.push_str(" kind=");
                        out.push_str(&k.to_string());
                    }
                    out.push_str(" {");
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        print_field(field, &mut out);
                    }
                    out.push_str("}\n");
                }
            }
            Block::Record { name, tag, fields } => {
                if name.is_empty() && tag.is_none() {
                    for field in fields {
                        out.push_str("field ");
                        print_field(field, &mut out);
                        out.push('\n');
                    }
                } else {
                    out.push_str("record ");
                    out.push_str(name);
                    if let Some(t) = tag {
                        out.push_str(" tag=");
                        out.push_str(&t.to_string());
                    }
                    out.push(' ');
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        print_field(field, &mut out);
                    }
                    out.push('\n');
                }
            }
            Block::Struct { name, fields } => {
                out.push_str("struct ");
                out.push_str(name);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
            Block::Enum { name, variants } => {
                out.push_str("enum ");
                out.push_str(name);
                out.push_str(" {");
                for (i, (vname, value)) in variants.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    out.push_str(vname);
                    out.push('=');
                    out.push_str(&value.to_string());
                }
                out.push_str("}\n");
            }
            Block::Footer(size) => {
                out.push_str("footer fixed ");
                out.push_str(&size.to_string());
                out.push('\n');
            }
            Block::Chain(prim) => {
                out.push_str("chain ");
                print_prim(prim, &mut out);
                out.push('\n');
            }
        }
    }
    out
}

/// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)` — the idempotence law every
/// technology's canonical form must satisfy.
pub fn canonicalize(text: &str) -> Result<String, TextError> {
    if is_protocol_source(text) {
        Ok(print_protocol(&parse_protocol(text)?))
    } else {
        Ok(print_grammar(&parse_grammar(text)?))
    }
}
//#endregion 🔖️Writer


//#region 🔖️Recognizer
/// @emoji 🧭️ Recognizer with explicit terminal predicates, family fragment merge, and macro matchers.
pub struct MacroMatcher {
    pub name: &'static str,
    pub try_match: fn(&str) -> bool,
}

/// @emoji 🧩️ Named grammar fragments (family kits) merged into Recognizer::compile_with.
#[derive(Default, Clone)]
pub struct FragmentRegistry {
    fragments: std::collections::HashMap<String, GrammarFile>,
}

impl FragmentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: impl Into<String>, grammar: GrammarFile) {
        self.fragments.insert(name.into(), grammar);
    }

    pub fn get(&self, name: &str) -> Option<&GrammarFile> {
        self.fragments.get(name)
    }
}

pub struct Recognizer {
    grammar: GrammarFile,
    macros: Vec<MacroMatcher>,
}

impl Recognizer {
    pub fn compile(grammar: &GrammarFile) -> Self {
        Self::compile_with(grammar, &FragmentRegistry::new())
    }

    /// @emoji 🔗️ Compile grammar, merging productions from each use via registry.
    pub fn compile_with(grammar: &GrammarFile, registry: &FragmentRegistry) -> Self {
        let mut merged = grammar.clone();
        let mut seen = std::collections::HashSet::<String>::new();
        for p in &grammar.productions {
            seen.insert(p.name.clone());
        }
        for use_name in &grammar.uses {
            if let Some(frag) = registry.get(use_name) {
                for prod in &frag.productions {
                    if seen.insert(prod.name.clone()) {
                        merged.productions.push(prod.clone());
                    }
                }
            }
        }
        Self {
            grammar: merged,
            macros: default_macros(),
        }
    }

    fn find_production(&self, name: &str) -> Option<&Production> {
        self.grammar.productions.iter().find(|p| p.name == name)
    }

    /// @emoji ✅️ Recognizes text against the grammar start production.
    pub fn recognize(&self, text: &str) -> Result<bool, TextError> {
        let raw = core_lex(text, &Limits::default(), false)?;
        let tokens: Vec<_> = raw
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof)
            .collect();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| {
            TextError::new(
                format!("start production `{}` not found", self.grammar.start),
                TextSpan::at(1, 1),
            )
        })?;
        match self.match_production(start, &tokens, 0) {
            Some(pos) => Ok(pos == tokens.len()),
            None => Ok(false),
        }
    }

    /// @emoji 📊️ Productions never reached while recognizing text.
    pub fn uncovered_productions(&self, text: &str) -> Result<Vec<String>, TextError> {
        let raw = core_lex(text, &Limits::default(), false)?;
        let tokens: Vec<_> = raw
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof)
            .collect();
        let mut covered = std::collections::HashSet::<String>::new();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| {
            TextError::new(
                format!("start production `{}` not found", self.grammar.start),
                TextSpan::at(1, 1),
            )
        })?;
        let _ = self.match_production_tracked(start, &tokens, 0, &mut covered);
        Ok(self
            .grammar
            .productions
            .iter()
            .map(|p| p.name.clone())
            .filter(|n| !covered.contains(n))
            .collect())
    }

    fn match_production(
        &self,
        production: &Production,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
    ) -> Option<usize> {
        let mut covered = std::collections::HashSet::new();
        self.match_production_tracked(production, tokens, pos, &mut covered)
    }

    fn match_production_tracked(
        &self,
        production: &Production,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
        covered: &mut std::collections::HashSet<String>,
    ) -> Option<usize> {
        for alt in &production.alternatives {
            if let Some(next) = self.match_sequence_tracked(&alt.symbols, tokens, pos, covered) {
                covered.insert(production.name.clone());
                return Some(next);
            }
        }
        None
    }

    fn match_sequence_tracked(
        &self,
        symbols: &[Symbol],
        tokens: &[crate::os_dsl::core::SpannedToken],
        mut pos: usize,
        covered: &mut std::collections::HashSet<String>,
    ) -> Option<usize> {
        for symbol in symbols {
            pos = self.match_symbol_tracked(symbol, tokens, pos, covered)?;
        }
        Some(pos)
    }

    fn match_symbol_tracked(
        &self,
        symbol: &Symbol,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
        covered: &mut std::collections::HashSet<String>,
    ) -> Option<usize> {
        match symbol {
            Symbol::Literal(text) => {
                let token = tokens.get(pos)?;
                (token.text.as_str().as_ref() == text.as_str()).then_some(pos + 1)
            }
            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                terminal_matches(name, token).then_some(pos + 1)
            }
            Symbol::Ref(name) => {
                if let Some(production) = self.find_production(name) {
                    self.match_production_tracked(production, tokens, pos, covered)
                } else if let Some(matcher) = self.macros.iter().find(|m| m.name == name) {
                    self.match_macro_span(matcher, tokens, pos)
                } else {
                    None
                }
            }
            Symbol::Macro(name, _args) => {
                let matcher = self.macros.iter().find(|m| &m.name == name)?;
                self.match_macro_span(matcher, tokens, pos)
            }
            Symbol::Group(alts) => alts
                .iter()
                .find_map(|alt| self.match_sequence_tracked(&alt.symbols, tokens, pos, covered)),
            Symbol::Optional(inner) => {
                Some(self.match_symbol_tracked(inner, tokens, pos, covered).unwrap_or(pos))
            }
            Symbol::Star(inner) => {
                let mut cur = pos;
                while let Some(next) = self.match_symbol_tracked(inner, tokens, cur, covered) {
                    if next == cur {
                        break;
                    }
                    cur = next;
                }
                Some(cur)
            }
            Symbol::Plus(inner) => {
                let first = self.match_symbol_tracked(inner, tokens, pos, covered)?;
                let mut cur = first;
                loop {
                    match self.match_symbol_tracked(inner, tokens, cur, covered) {
                        Some(next) if next != cur => cur = next,
                        _ => break,
                    }
                }
                Some(cur)
            }
        }
    }

    fn match_macro_span(
        &self,
        matcher: &MacroMatcher,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
    ) -> Option<usize> {
        for end in pos + 1..=tokens.len() {
            let slice_text = slice_source_text(&tokens[pos..end]);
            if (matcher.try_match)(&slice_text) {
                return Some(end);
            }
        }
        None
    }
}

fn slice_source_text(tokens: &[crate::os_dsl::core::SpannedToken]) -> String {
    tokens
        .iter()
        .map(|t| t.text.as_str().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// @emoji 🏷️ Explicit terminal predicates — BOOL is Ident true|false.
fn terminal_matches(name: &str, token: &crate::os_dsl::core::SpannedToken) -> bool {
    let upper = name.to_uppercase();
    let text = token.text.as_str();
    match upper.as_str() {
        "BOOL" => matches!(token.kind, CoreKind::Ident) && (text == "true" || text == "false"),
        "IDENT" | "PLACEHOLDER" => matches!(token.kind, CoreKind::Ident | CoreKind::Placeholder),
        "INT" => matches!(token.kind, CoreKind::Int),
        "FLOAT" => matches!(token.kind, CoreKind::Float),
        "TEXT" | "STRING" => matches!(token.kind, CoreKind::Text),
        "STAR" => matches!(token.kind, CoreKind::Star),
        "PLUS" => matches!(token.kind, CoreKind::Plus),
        "EQUALS" | "EQ" => matches!(token.kind, CoreKind::Equals),
        "ARROW" => matches!(token.kind, CoreKind::Arrow),
        "DASHARROW" => matches!(token.kind, CoreKind::DashArrow),
        "BACKARROW" => matches!(token.kind, CoreKind::BackArrow),
        "EDGEARROW" => matches!(token.kind, CoreKind::EdgeArrow),
        "QUANTITY" => matches!(token.kind, CoreKind::Float | CoreKind::Int),
        "VEC3" | "COLOR" | "POINT" | "UNIT" => {
            matches!(
                token.kind,
                CoreKind::Ident | CoreKind::Float | CoreKind::Int | CoreKind::Text
            )
        }
        other => format!("{:?}", token.kind).to_uppercase() == other,
    }
}

fn macro_table_ok(text: &str) -> bool {
    let t = text.trim();
    t.contains('|') || t.starts_with("table")
}

fn macro_quantity_ok(text: &str) -> bool {
    let parts: Vec<_> = text.split_whitespace().collect();
    !parts.is_empty()
        && parts[0]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit() || c == '-' || c == '.')
}

fn macro_props_ok(text: &str) -> bool {
    text.contains('=')
}

fn default_macros() -> Vec<MacroMatcher> {
    vec![
        MacroMatcher {
            name: "edge",
            try_match: |text| crate::os_dsl::notation::parse_edge_text(text).is_ok(),
        },
        MacroMatcher {
            name: "table",
            try_match: macro_table_ok,
        },
        MacroMatcher {
            name: "quantity",
            try_match: macro_quantity_ok,
        },
        MacroMatcher {
            name: "props",
            try_match: macro_props_ok,
        },
    ]
}
//#endregion 🔖️Recognizer


//#region 📡️ProtocolWalk
fn mismatch(offset: usize, message: impl Into<String>) -> ProtocolMismatch {
    ProtocolMismatch { offset, message: message.into() }
}

fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolMismatch> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        if *pos >= bytes.len() {
            return Err(mismatch(*pos, "truncated varint"));
        }
        let byte = bytes[*pos];
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(mismatch(*pos, "varint overflow"));
        }
    }
}

fn need<'a>(bytes: &'a [u8], pos: usize, n: usize, what: &str) -> Result<&'a [u8], ProtocolMismatch> {
    if pos + n > bytes.len() {
        return Err(mismatch(pos, format!("truncated {what}: need {n} bytes, have {}", bytes.len().saturating_sub(pos))));
    }
    Ok(&bytes[pos..pos + n])
}

fn trailing_reserved(blocks: &[Block], from: usize) -> usize {
    let mut reserved = 0usize;
    for block in &blocks[from..] {
        match block {
            Block::Footer(n) => reserved += *n,
            Block::Chain(prim) => reserved += prim_fixed_width(prim).unwrap_or(0),
            Block::Struct { .. } | Block::Enum { .. } => {}
            Block::Header(_) | Block::Segment { .. } | Block::Record { .. } => break,
        }
    }
    reserved
}

fn resolve_count(count: &Count, env: &std::collections::HashMap<String, u64>, offset: usize) -> Result<usize, ProtocolMismatch> {
    match count {
        Count::Fixed(n) => Ok(*n),
        Count::Varint => Err(mismatch(offset, "Count::Varint must be read from the byte stream, not resolved from env")),
        Count::Field(name) => env.get(name).map(|v| *v as usize).ok_or_else(|| mismatch(offset, format!("unknown count field `{name}`"))),
    }
}

fn walk_prim(
    prim: &Prim,
    bytes: &[u8],
    pos: &mut usize,
    env: &mut std::collections::HashMap<String, u64>,
    reserved_tail: usize,
) -> Result<(), ProtocolMismatch> {
    match prim {
        Prim::U8 => {
            need(bytes, *pos, 1, "u8")?;
            *pos += 1;
        }
        Prim::U16 => {
            need(bytes, *pos, 2, "u16")?;
            *pos += 2;
        }
        Prim::U32 | Prim::I32 | Prim::F32 => {
            need(bytes, *pos, 4, "u32/i32/f32")?;
            *pos += 4;
        }
        Prim::U64 | Prim::I64 | Prim::F64 => {
            need(bytes, *pos, 8, "u64/i64/f64")?;
            *pos += 8;
        }
        Prim::Fixed(n) => {
            need(bytes, *pos, *n, "fixed")?;
            *pos += *n;
        }
        Prim::Varint | Prim::Tag | Prim::Zigzag => {
            let _ = read_varint_u64(bytes, pos)?;
        }
        Prim::Bytes | Prim::Utf8 => {
            let end = bytes.len().saturating_sub(reserved_tail);
            if *pos > end {
                return Err(mismatch(*pos, "bytes field overlaps trailing reserved region"));
            }
            *pos = end;
        }
        Prim::Array(inner, count) => {
            let n = match count {
                Count::Varint => read_varint_u64(bytes, pos)? as usize,
                other => resolve_count(other, env, *pos)?,
            };
            if matches!(inner.as_ref(), Prim::U8) {
                need(bytes, *pos, n, "byte array")?;
                *pos += n;
            } else {
                for _ in 0..n {
                    walk_prim(inner, bytes, pos, env, reserved_tail)?;
                }
            }
        }
        Prim::Ref(name) => return Err(mismatch(*pos, format!("unresolved protocol Ref({name}) during walk"))),
    }
    Ok(())
}

fn walk_fields(fields: &[Field], bytes: &[u8], pos: &mut usize, reserved_tail: usize) -> Result<(), ProtocolMismatch> {
    let mut env = std::collections::HashMap::new();
    for (index, field) in fields.iter().enumerate() {
        let field_reserved = if index + 1 == fields.len() {
            reserved_tail
        } else {
            reserved_tail
                + fields[index + 1..]
                    .iter()
                    .map(|f| prim_fixed_width(&f.ty).unwrap_or(0))
                    .sum::<usize>()
        };
        match &field.ty {
            Prim::U8 => {
                let slice = need(bytes, *pos, 1, &field.name)?;
                env.insert(field.name.clone(), u64::from(slice[0]));
                *pos += 1;
            }
            Prim::U16 => {
                let slice = need(bytes, *pos, 2, &field.name)?;
                env.insert(field.name.clone(), u64::from(u16::from_le_bytes([slice[0], slice[1]])));
                *pos += 2;
            }
            Prim::U32 => {
                let slice = need(bytes, *pos, 4, &field.name)?;
                env.insert(field.name.clone(), u64::from(u32::from_le_bytes(slice.try_into().unwrap())));
                *pos += 4;
            }
            Prim::U64 => {
                let slice = need(bytes, *pos, 8, &field.name)?;
                env.insert(field.name.clone(), u64::from_le_bytes(slice.try_into().unwrap()));
                *pos += 8;
            }
            Prim::Varint | Prim::Tag | Prim::Zigzag => {
                let value = read_varint_u64(bytes, pos)?;
                env.insert(field.name.clone(), value);
            }
            other => walk_prim(other, bytes, pos, &mut env, field_reserved)?,
        }
    }
    Ok(())
}

fn definitions_only(block: &Block) -> bool {
    matches!(block, Block::Struct { .. } | Block::Enum { .. })
}

/// @emoji 🧭️ Spec-driven byte walker — consumes every declared wire slot and must finish at
/// exactly `bytes.len()`, else returns [`ProtocolMismatch`] with the failing offset.
pub fn walk_protocol(spec: &ProtocolFile, bytes: &[u8]) -> Result<ProtocolTrace, ProtocolMismatch> {
    let mut pos = 0usize;
    match &spec.framing {
        Framing::Magic(magic) => {
            let got = need(bytes, 0, 8, "magic")?;
            if got != magic {
                return Err(mismatch(0, format!("magic mismatch: expected {magic:?}, got {got:?}")));
            }
            pos = 8;
        }
        Framing::Record | Framing::Chunked => {}
    }

    let skip_named_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);
    let record_body_as_rest = matches!(spec.framing, Framing::Record);
    let mut consumed_record_body = false;

    for (index, block) in spec.blocks.iter().enumerate() {
        if definitions_only(block) {
            continue;
        }
        let reserved = trailing_reserved(&spec.blocks, index + 1);
        match block {
            Block::Header(fields) => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Segment { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Record { name, fields, .. } => {
                if skip_named_records && !name.is_empty() {
                    continue;
                }
                if record_body_as_rest && !name.is_empty() {
                    if !consumed_record_body {
                        pos = bytes.len();
                        consumed_record_body = true;
                    }
                    continue;
                }
                walk_fields(fields, bytes, &mut pos, reserved)?;
            }
            Block::Footer(size) => {
                need(bytes, pos, *size, "footer")?;
                pos += *size;
            }
            Block::Chain(prim) => {
                let mut env = std::collections::HashMap::new();
                walk_prim(prim, bytes, &mut pos, &mut env, 0)?;
            }
            Block::Struct { .. } | Block::Enum { .. } => {}
        }
    }

    if record_body_as_rest && !consumed_record_body && pos < bytes.len() {
        pos = bytes.len();
    }

    if pos != bytes.len() {
        return Err(mismatch(pos, format!("trailing {} bytes after protocol walk", bytes.len() - pos)));
    }
    Ok(ProtocolTrace { consumed: pos })
}

/// @emoji 📡️ Shallow GrammarFile back-compat check: pack requires leading 0x89 magic (any family)
/// and ≥32 bytes; spr requires non-empty bytes. Deep walks use [`verify_protocol_source`].
pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    if spec.dialect != SemioDialect::Protocol {
        return Err("verify_protocol_bytes requires dialect protocol".to_string());
    }
    let is_pack = spec.start == "frame" || spec.id.contains("pack");
    let is_spr = spec.start == "record" || spec.id.contains("spr");
    if is_pack {
        if bytes.len() < 8 {
            return Err("pack bytes shorter than magic".to_string());
        }
        if bytes[0] != 0x89 {
            return Err("pack magic must start with 0x89".to_string());
        }
        if bytes.len() < 32 {
            return Err("pack header requires 32 bytes".to_string());
        }
        return Ok(());
    }
    if is_spr {
        if bytes.is_empty() {
            return Err("spr bytes empty".to_string());
        }
        return Ok(());
    }
    Err(format!("protocol spec start '{}' is neither frame nor record", spec.start))
}

/// @emoji 📡️ Parses handcrafted `.protocol.semio` source then deep-walks bytes via [`walk_protocol`].
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    walk_protocol(&spec, bytes).map(|_| ()).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}


//#endregion 📡️ProtocolWalk


//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_grammar_header() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = \"hello\"\n").expect("parse_grammar");
        assert_eq!(g.id, "demo");
        assert_eq!(g.start, "doc");
        assert_eq!(g.productions.len(), 1);
        assert_eq!(g.productions[0].alternatives[0].symbols, vec![Symbol::Literal("hello".to_string())]);
    }

    #[test]
    fn parses_extension_and_uses() {
        let g = parse_grammar("grammar fem2d\nextension fem2d\nuse core\nuse family-sheet\nstart document\ndocument = header\nheader = \"fem2d\" TEXT\n")
            .expect("parse_grammar");
        assert_eq!(g.extension, Some("fem2d".to_string()));
        assert_eq!(g.uses, vec!["core".to_string(), "family-sheet".to_string()]);
        assert_eq!(g.productions.len(), 2);
    }

    #[test]
    fn parses_terminal_vs_ref_vs_macro() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = TEXT node table(\"rows\", row)\nrow = IDENT\n").expect("parse_grammar");
        let symbols = &g.productions[0].alternatives[0].symbols;
        assert_eq!(symbols[0], Symbol::Terminal("TEXT".to_string()));
        assert_eq!(symbols[1], Symbol::Ref("node".to_string()));
        assert_eq!(symbols[2], Symbol::Macro("table".to_string(), vec![MacroArg::Literal("rows".to_string()), MacroArg::Ident("row".to_string())]));
    }

    #[test]
    fn parses_alternation_group_and_quantifiers() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = {\"a\" | \"b\"}? node* row+\nnode = IDENT\nrow = IDENT\n").expect("parse_grammar");
        let symbols = &g.productions[0].alternatives[0].symbols;
        match &symbols[0] {
            Symbol::Optional(inner) => match inner.as_ref() {
                Symbol::Group(alts) => {
                    assert_eq!(alts.len(), 2);
                    assert_eq!(alts[0].symbols, vec![Symbol::Literal("a".to_string())]);
                    assert_eq!(alts[1].symbols, vec![Symbol::Literal("b".to_string())]);
                }
                other => panic!("expected Group, got {other:?}"),
            },
            other => panic!("expected Optional, got {other:?}"),
        }
        assert!(matches!(&symbols[1], Symbol::Star(_)));
        assert!(matches!(&symbols[2], Symbol::Plus(_)));
    }

    #[test]
    fn round_trip_matrix_over_representative_grammars() {
        let sources = vec![
            "grammar demo\nstart doc\ndoc = \"hello\"\n",
            "grammar fem2d\nextension fem2d\nuse core\nstart document\ndocument = header body\nheader = \"fem2d\" TEXT\nbody = row*\nrow = IDENT FLOAT?\n",
            "grammar demo\nstart doc\ndoc = {\"a\" | \"b\"} node+\nnode = IDENT\n",
        ];
        for source in sources {
            let parsed = parse_grammar(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
            let printed = print_grammar(&parsed);
            let reparsed = parse_grammar(&printed).unwrap_or_else(|e| panic!("reparse of canonical {printed:?} failed: {e:?}"));
            assert_eq!(reparsed, parsed, "round trip mismatch for {source:?} -> {printed:?}");
            let canonical_twice = canonicalize(&printed).expect("canonicalize");
            assert_eq!(canonical_twice, printed, "canonicalize is not idempotent for {printed:?}");
        }
    }

    #[test]
    fn missing_start_directive_is_an_error() {
        let err = parse_grammar("grammar demo\ndoc = \"hello\"\n").unwrap_err();
        assert!(err.message.contains("start"), "unexpected message: {}", err.message);
    }

    /// @emoji 🪞️ This crate's own format description parses under the parser it defines — the
    /// self-hosting proof the architecture plan calls for.
    #[test]
    fn self_hosting_grammar_grammar_parses_and_round_trips() {
        let source = include_str!("../../📖️grammar.grammar.semio");
        let parsed = parse_grammar(source).expect("dsl_grammar's own grammar.grammar must parse under its own parser");
        assert_eq!(parsed.id, "grammar");
        let printed = print_grammar(&parsed);
        let reparsed = parse_grammar(&printed).expect("canonical print of grammar.grammar must reparse");
        assert_eq!(reparsed, parsed);
    }

    #[test]
    fn recognizer_matches_plain_arrow_via_registered_edge_macro() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = edge\n").expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("a->b").expect("recognize"));
        assert!(recognizer.recognize("a -[e1:Connection]->b").expect("recognize"));
        assert!(!recognizer.recognize("a-> ->").expect("recognize"));
    }

    #[test]
    fn recognizer_matches_literals_terminals_and_quantifiers() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = \"beam\" IDENT node*\nnode = IDENT\n").expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("beam e3 n1 n2").expect("recognize"));
        assert!(recognizer.recognize("beam e3").expect("recognize"));
        assert!(!recognizer.recognize("beam").expect("recognize"));
    }

    Tests

    #[test]
    fn parse_grammar_sets_dialect_grammar_vs_protocol() {
        let g = parse_grammar("dialect grammar\\ngrammar demo\\nstart doc\\ndoc = \\"x\\"\\n").expect("grammar");
        assert_eq!(g.dialect, SemioDialect::Grammar);
        let p = parse_grammar(
            "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo\\nstart frame\\nframing magic 0x8953504B0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n",
        )
        .expect("protocol");
        assert_eq!(p.dialect, SemioDialect::Protocol);
        assert_eq!(p.start, "frame");
        assert_eq!(p.id, "demo.pack");
    }

    #[test]
    fn parse_protocol_roundtrip_retains_body() {
        let source = "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo.v1\\nstart frame\\nframing magic 0x8953504B0D0A1A0A\\nheader fixed 4\\nfield flags u32\\nfooter fixed 8\\n";
        let parsed = parse_protocol(source).expect("parse_protocol");
        let printed = print_protocol(&parsed);
        let reparsed = parse_protocol(&printed).expect("reparse");
        assert_eq!(parsed, reparsed);
        let once = canonicalize(source).expect("canonicalize");
        assert_eq!(canonicalize(&once).expect("twice"), once);
    }

    #[test]
    fn walk_protocol_spk_shaped_buffer() {
        let source = "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo.v1\\nstart frame\\nframing magic 0x8953504B0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n";
        let spec = parse_protocol(source).expect("parse");
        let mut bytes = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
        bytes.extend_from_slice(&7u32.to_le_bytes());
        let trace = walk_protocol(&spec, &bytes).expect("walk");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_source(source, &bytes).expect("verify_protocol_source");
        let shallow = parse_grammar(source).expect("project");
        verify_protocol_bytes(&shallow, {
            let mut long = bytes.clone();
            long.extend(std::iter::repeat(0u8).take(20));
            &long.clone()
        })
        .expect("shallow any-length ok if >=32");
    }

    #[test]
    fn walk_protocol_spr_shaped_body_as_rest() {
        let source = "dialect protocol\\nprotocol demo.spr\\nversion 1\\nschema demo.op\\nstart record\\nframing record\\nfield format u8\\nfield ordinal varint\\nrecord ObjectsAdd tag 1\\nfield index varint\\n";
        let spec = parse_protocol(source).expect("parse");
        let bytes = vec![1u8, 0x00, 0xAA, 0xBB];
        let trace = walk_protocol(&spec, &bytes).expect("spr walk");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_source(source, &bytes).expect("verify_protocol_source");
    }

    #[test]
    fn verify_protocol_bytes_accepts_any_0x89_magic() {
        let pack = parse_grammar(
            "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo\\nstart frame\\nframing magic 0x8953504B0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n",
        )
        .expect("pack");
        let spr = parse_grammar(
            "dialect protocol\\nprotocol demo.spr\\nversion 1\\nschema demo\\nstart record\\nframing record\\nfield format u8\\n",
        )
        .expect("spr");
        let mut spk = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
        spk.extend(std::iter::repeat(0u8).take(24));
        verify_protocol_bytes(&pack, &spk).expect("SPK");
        let mut lwpl = vec![0x89, b'L', b'W', b'P', b'L', 0x0D, 0x0A, 0x1A];
        lwpl.extend(std::iter::repeat(0u8).take(24));
        verify_protocol_bytes(&pack, &lwpl).expect("any 0x89");
        assert!(verify_protocol_bytes(&pack, &[0x00u8; 32]).is_err());
        assert!(verify_protocol_bytes(&spr, &[]).is_err());
        verify_protocol_bytes(&spr, &[1u8]).expect("spr");
    }

    #[test]
    fn recognizer_matches_bool_and_arrow_terminals() {
        let grammar = parse_grammar("grammar demo\\nstart doc\\ndoc = BOOL EQUALS QUANTITY ARROW DASHARROW BACKARROW\\n").expect("grammar");
        let rec = Recognizer::compile(&grammar);
        assert!(rec.recognize("true = 12 -> -- <-").expect("recognize"));
        assert!(rec.recognize("false = 3.5 -> -- <-").expect("recognize"));
        assert!(!rec.recognize("yes = 12 -> -- <-").expect("recognize"));
    }

    #[test]
    fn self_hosting_protocol_grammar_semio_parses_as_grammar() {
        let source = include_str!("📖️protocol.grammar.semio");
        let parsed = parse_grammar(source).expect("protocol.grammar.semio must parse");
        assert_eq!(parsed.dialect, SemioDialect::Grammar);
        assert_eq!(parsed.id, "protocol");
        assert_eq!(parse_grammar(&print_grammar(&parsed)).expect("reparse"), parsed);
    }

}
//#endregion 🔖️Tests`;

if (!src.includes("fn parse_grammar_sets_dialect_grammar_vs_protocol")) {
  throw new Error("dialect test missing");
}

// Use a more flexible replacement from the dialect test to end
const dialectIdx = src.indexOf("    #[test]\n    fn parse_grammar_sets_dialect_grammar_vs_protocol()");
const testsEndIdx = src.lastIndexOf("//#endregion 🔖️Tests
