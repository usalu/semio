//! 🃏 Cypher-inspired query language for trinity graphs.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use trinity_ram::{
    Edge, EntityRef, Graph, Node, Port, PortDirection, PropertyBag, PropertyValue, port_key,
};

// #region 🔖Ast
/// 🌳 Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
}
// #endregion 🔖Ast

// #region 🔖Lexer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSpan {
    pub class: TokenClass,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    Ident(String),
    Number(f64),
    StringLit(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Dash,
    Arrow,
    And,
    Or,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct SpannedToken {
    token: Token,
    start: usize,
    end: usize,
}

fn token_class(token: &Token) -> TokenClass {
    match token {
        Token::KwMatch
        | Token::KwWhere
        | Token::KwReturn
        | Token::KwCreate
        | Token::KwDelete
        | Token::KwSet
        | Token::KwMerge
        | Token::And
        | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Dash | Token::Arrow => TokenClass::Operator,
        Token::LParen
        | Token::RParen
        | Token::LBracket
        | Token::RBracket
        | Token::Colon
        | Token::Comma
        | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, String> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        match c {
            b'(' => {
                push_spanned(&mut tokens, Token::LParen, start, start + 1);
                i += 1;
            }
            b')' => {
                push_spanned(&mut tokens, Token::RParen, start, start + 1);
                i += 1;
            }
            b'[' => {
                push_spanned(&mut tokens, Token::LBracket, start, start + 1);
                i += 1;
            }
            b']' => {
                push_spanned(&mut tokens, Token::RBracket, start, start + 1);
                i += 1;
            }
            b':' => {
                push_spanned(&mut tokens, Token::Colon, start, start + 1);
                i += 1;
            }
            b',' => {
                push_spanned(&mut tokens, Token::Comma, start, start + 1);
                i += 1;
            }
            b'.' => {
                push_spanned(&mut tokens, Token::Dot, start, start + 1);
                i += 1;
            }
            b'=' => {
                push_spanned(&mut tokens, Token::Eq, start, start + 1);
                i += 1;
            }
            b'\'' | b'"' => {
                let quote = c;
                i += 1;
                let lit_start = i;
                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }
                if i >= bytes.len() {
                    if forgiving {
                        let s = String::from_utf8_lossy(&bytes[lit_start..i]).into_owned();
                        push_spanned(&mut tokens, Token::StringLit(s), start, i);
                        break;
                    }
                    return Err("unterminated string".into());
                }
                let s = String::from_utf8_lossy(&bytes[lit_start..i]).into_owned();
                i += 1;
                push_spanned(&mut tokens, Token::StringLit(s), start, i);
            }
            b'-' if i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                push_spanned(&mut tokens, Token::Arrow, start, start + 2);
                i += 2;
            }
            b'0'..=b'9' | b'-' if i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() => {
                let num_start = i;
                if bytes[i] == b'-' {
                    i += 1;
                }
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
                let num: f64 = match std::str::from_utf8(&bytes[num_start..i]) {
                    Ok(s) => match s.parse() {
                        Ok(n) => n,
                        Err(_e) if forgiving => {
                            push_spanned(&mut tokens, Token::Ident(s.to_string()), num_start, i);
                            continue;
                        }
                        Err(e) => return Err(e.to_string()),
                    },
                    Err(_e) if forgiving => {
                        push_spanned(&mut tokens, Token::Ident(String::new()), num_start, i);
                        continue;
                    }
                    Err(e) => return Err(e.to_string()),
                };
                push_spanned(&mut tokens, Token::Number(num), num_start, i);
            }
            b'-' => {
                push_spanned(&mut tokens, Token::Dash, start, start + 1);
                i += 1;
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let word = std::str::from_utf8(&bytes[start..i]).unwrap().to_ascii_uppercase();
                let tok = match word.as_str() {
                    "MATCH" => Token::KwMatch,
                    "WHERE" => Token::KwWhere,
                    "RETURN" => Token::KwReturn,
                    "CREATE" => Token::KwCreate,
                    "DELETE" => Token::KwDelete,
                    "SET" => Token::KwSet,
                    "MERGE" => Token::KwMerge,
                    "AND" => Token::And,
                    "OR" => Token::Or,
                    _ => Token::Ident(std::str::from_utf8(&bytes[start..i]).unwrap().to_string()),
                };
                push_spanned(&mut tokens, tok, start, i);
            }
            _ if forgiving => {
                push_spanned(&mut tokens, Token::Ident(String::from(c as char)), start, start + 1);
                i += 1;
            }
            _ => return Err(format!("unexpected char {}", c as char)),
        }
    }
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    lex_spanned(input, false)
        .map(|spanned| spanned.into_iter().map(|row| row.token).collect())
}

/// 🎨 Tokenize jack source for editor highlighting (never fails).
pub fn tokenize(input: &str) -> Vec<TokenSpan> {
    lex_spanned(input, true)
        .unwrap_or_default()
        .into_iter()
        .filter(|row| !matches!(row.token, Token::Eof))
        .map(|row| {
            let mut class = token_class(&row.token);
            if matches!(row.token, Token::StringLit(_)) {
                let quote = input.as_bytes().get(row.start);
                if quote == Some(&b'\'') || quote == Some(&b'"') {
                    let closed = input.as_bytes().get(row.end.saturating_sub(1)) == quote;
                    if !closed {
                        class = TokenClass::Error;
                    }
                }
            }
            TokenSpan {
                class,
                start: row.start,
                end: row.end,
            }
        })
        .collect()
}
// #endregion 🔖Lexer

// #region 🔖Language
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub label: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub insert: String,
}

const CLAUSE_KEYWORDS: &[&str] = &["MATCH", "WHERE", "RETURN", "CREATE", "DELETE", "SET", "MERGE"];
const LOGIC_KEYWORDS: &[&str] = &["AND", "OR"];

fn completion_prefix(source: &str, cursor: usize) -> String {
    let cursor = cursor.min(source.len());
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' {
            start -= 1;
        } else {
            break;
        }
    }
    source[start..cursor].to_string()
}

fn tokens_before_cursor<'a>(tokens: &'a [SpannedToken], cursor: usize) -> &'a [SpannedToken] {
    let mut end = tokens.len();
    for (i, row) in tokens.iter().enumerate() {
        if row.start >= cursor && !matches!(row.token, Token::Eof) {
            end = i;
            break;
        }
    }
    &tokens[..end]
}

fn after_colon_kind_context(source: &str, cursor: usize) -> Option<bool> {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let colon = before.rfind(':')?;
    let after = &before[colon + 1..];
    if after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',')) {
        return None;
    }
    let left = &before[..colon];
    let bracket = left.rfind('[');
    let paren = left.rfind('(');
    let in_bracket = match (bracket, paren) {
        (Some(b), Some(p)) => b > p,
        (Some(_), None) => true,
        _ => false,
    };
    Some(in_bracket)
}

fn after_dot_property_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(dot) = before.rfind('.') else {
        return false;
    };
    let after = &before[dot + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | ':'))
}
fn open_bracket_kind(tokens: &[SpannedToken]) -> Option<char> {
    let mut paren = 0i32;
    let mut bracket = 0i32;
    for row in tokens.iter().rev() {
        match row.token {
            Token::RParen => paren += 1,
            Token::LParen if paren > 0 => paren -= 1,
            Token::LParen if paren == 0 && bracket == 0 => return Some('('),
            Token::RBracket => bracket += 1,
            Token::LBracket if bracket > 0 => bracket -= 1,
            Token::LBracket if bracket == 0 && paren == 0 => return Some('['),
            _ => {}
        }
    }
    None
}

fn collect_bound_vars(tokens: &[SpannedToken]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    let mut i = 0;
    while i + 2 < tokens.len() {
        if matches!(tokens[i].token, Token::LParen | Token::LBracket) {
            if let Token::Ident(var) = &tokens[i + 1].token {
                if matches!(tokens[i + 2].token, Token::Colon) {
                    vars.insert(var.clone());
                }
            }
        }
        i += 1;
    }
    vars
}

fn in_where_clause(tokens: &[SpannedToken]) -> bool {
    let mut seen_where = false;
    let mut seen_return = false;
    for row in tokens {
        match row.token {
            Token::KwWhere => seen_where = true,
            Token::KwReturn => seen_return = true,
            _ => {}
        }
    }
    seen_where && !seen_return
}

fn graph_node_kinds(graph: &Graph) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for node in graph.nodes.values() {
        kinds.insert(node.kind.clone());
    }
    for def in &graph.manifest.node_kinds {
        kinds.insert(def.name.clone());
    }
    kinds.into_iter().collect()
}

fn graph_edge_kinds(graph: &Graph) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for edge in graph.edges.values() {
        kinds.insert(edge.kind.clone());
    }
    for def in &graph.manifest.edge_kinds {
        kinds.insert(def.name.clone());
    }
    kinds.into_iter().collect()
}

fn graph_property_names(graph: &Graph) -> Vec<String> {
    let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
    for node in graph.nodes.values() {
        for key in node.properties.keys() {
            props.insert(key.clone());
        }
    }
    props.into_iter().collect()
}

fn filter_completions(candidates: impl IntoIterator<Item = (String, String, Option<String>)>, prefix: &str) -> Vec<Completion> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut out = Vec::new();
    for (label, kind, detail) in candidates {
        if prefix.is_empty() || label.to_ascii_lowercase().starts_with(&prefix_lower) {
            out.push(Completion {
                insert: label.clone(),
                label,
                kind,
                detail,
            });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// 🔎 Context-aware jack completions for the editor.
pub fn complete(graph: &Graph, source: &str, cursor: usize) -> Vec<Completion> {
    let cursor = cursor.min(source.len());
    let prefix = completion_prefix(source, cursor);
    let tokens = lex_spanned(source, true).unwrap_or_default();
    let before = tokens_before_cursor(&tokens, cursor);

    if let Some(in_bracket) = after_colon_kind_context(source, cursor) {
        let kinds = if in_bracket {
            graph_edge_kinds(graph)
                .into_iter()
                .map(|name| (name, "edgeKind".into(), None))
                .collect::<Vec<_>>()
        } else {
            graph_node_kinds(graph)
                .into_iter()
                .map(|name| (name, "nodeKind".into(), None))
                .collect::<Vec<_>>()
        };
        return filter_completions(kinds, &prefix);
    }

    if after_dot_property_context(source, cursor) {
        let props = graph_property_names(graph)
            .into_iter()
            .map(|name| (name, "property".into(), None))
            .collect::<Vec<_>>();
        return filter_completions(props, &prefix);
    }

    if let Some(last) = before.last() {
        if matches!(last.token, Token::Colon) {
            let kinds = if open_bracket_kind(before) == Some('[') {
                graph_edge_kinds(graph)
                    .into_iter()
                    .map(|name| (name, "edgeKind".into(), None))
                    .collect::<Vec<_>>()
            } else {
                graph_node_kinds(graph)
                    .into_iter()
                    .map(|name| (name, "nodeKind".into(), None))
                    .collect::<Vec<_>>()
            };
            return filter_completions(kinds, &prefix);
        }
        if matches!(last.token, Token::Dot) {
            let props = graph_property_names(graph)
                .into_iter()
                .map(|name| (name, "property".into(), None))
                .collect::<Vec<_>>();
            return filter_completions(props, &prefix);
        }
    }

    if in_where_clause(before) {
        let logic = filter_completions(
            LOGIC_KEYWORDS
                .iter()
                .map(|kw| (kw.to_string(), "keyword".into(), None)),
            &prefix,
        );
        if !logic.is_empty() {
            return logic;
        }
    }

    let vars = collect_bound_vars(before);
    if !vars.is_empty() && prefix.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        let var_items = vars.into_iter().map(|name| (name, "variable".into(), None)).collect::<Vec<_>>();
        let filtered = filter_completions(var_items, &prefix);
        if !filtered.is_empty() {
            return filtered;
        }
    }

    filter_completions(
        CLAUSE_KEYWORDS
            .iter()
            .map(|kw| (kw.to_string(), "keyword".into(), None)),
        &prefix,
    )
}
// #endregion 🔖Language

// #region 🔖Parser
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(format!("expected ident, got {other:?}")),
        }
    }

    fn parse_query(&mut self) -> Result<Query, String> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    fn parse_clause(&mut self) -> Result<Clause, String> {
        match self.peek() {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                Ok(Clause::Match(patterns))
            }
            Token::KwWhere => {
                self.bump();
                Ok(Clause::Where(self.parse_expr()?))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::Return(items))
            }
            Token::KwCreate => {
                self.bump();
                Ok(Clause::Create(self.parse_pattern()?))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                Ok(Clause::Delete(vars))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                Ok(Clause::Set(items))
            }
            Token::KwMerge => {
                self.bump();
                Ok(Clause::Merge(self.parse_pattern()?))
            }
            other => Err(format!("unexpected clause start {other:?}")),
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        self.expect(Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(Token::RParen)?;
        if matches!(self.peek(), Token::Dash) {
            self.bump();
            self.expect(Token::LBracket)?;
            let edge_var = if matches!(self.peek(), Token::Ident(_)) {
                Some(self.expect_ident()?)
            } else {
                None
            };
            let edge_kind = if matches!(self.peek(), Token::Colon) {
                self.bump();
                Some(self.expect_ident()?)
            } else {
                None
            };
            self.expect(Token::RBracket)?;
            self.expect(Token::Arrow)?;
            self.expect(Token::LParen)?;
            let right = self.parse_pattern_node()?;
            self.expect(Token::RParen)?;
            Ok(Pattern {
                nodes: vec![left],
                edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right }),
            })
        } else {
            Ok(Pattern { nodes: vec![left], edge: None })
        }
    }

    fn parse_pattern_node(&mut self) -> Result<PatternNode, String> {
        let var = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let kind = self.expect_ident()?;
        Ok(PatternNode { var, kind })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, String> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let var = self.expect_ident()?;
        self.expect(Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<Expr, String> {
        let var = self.expect_ident()?;
        self.expect(Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(format!("expected = or !=, got {other:?}")),
        }
    }

    fn parse_value(&mut self) -> Result<PropertyValue, String> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(format!("expected value, got {other:?}")),
        }
    }

    fn expect(&mut self, want: Token) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&want) {
            self.bump();
            Ok(())
        } else {
            Err(format!("expected {want:?}, got {:?}", self.peek()))
        }
    }
}

/// 🔍 Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, String> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
// #endregion 🔖Parser

// #region 🔖Executor
/// 🎯 Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
}

/// ▶️ Execute a jack query against a graph.
pub fn execute(graph: &mut Graph, query: &Query) -> Result<QueryResult, String> {
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                bindings = match_patterns(graph, patterns)?;
            }
            Clause::Where(expr) => {
                bindings.retain(|b| eval_expr(graph, b, expr));
            }
            Clause::Return(items) => {
                return_items = Some(items.clone());
            }
            Clause::Create(pattern) => {
                apply_create(graph, pattern)?;
            }
            Clause::Delete(vars) => {
                for var in vars {
                    if let Some(id) = bindings.first().and_then(|b| b.nodes.get(var).cloned()) {
                        graph.remove_node(&id);
                    }
                }
                graph.recompute_derived();
            }
            Clause::Set(items) => {
                let b = bindings.first().cloned().unwrap_or_default();
                for item in items {
                    if let Some(node_id) = b.nodes.get(&item.var) {
                        graph.set_property(EntityRef::Node(node_id.clone()), &item.prop, item.value.clone())?;
                    }
                }
                graph.recompute_derived();
            }
            Clause::Merge(pattern) => {
                let existing = match_patterns(graph, std::slice::from_ref(pattern))?;
                if existing.is_empty() {
                    apply_create(graph, pattern)?;
                }
            }
        }
    }
    if let Some(items) = return_items {
        return Ok(build_return(graph, &bindings, &items));
    }
    Ok(QueryResult { columns: vec![], rows: vec![] })
}

/// ▶️ Parse and execute jack in one step.
pub fn run(graph: &mut Graph, source: &str) -> Result<QueryResult, String> {
    let query = parse(source)?;
    execute(graph, &query)
}

/// ▶️ Execute jack and return JSON result.
pub fn run_json(graph: &mut Graph, source: &str) -> Result<String, String> {
    let result = run(graph, source)?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

fn match_patterns(graph: &Graph, patterns: &[Pattern]) -> Result<Vec<Binding>, String> {
    let mut bindings = vec![Binding::default()];
    for pattern in patterns {
        let mut next = Vec::new();
        for binding in &bindings {
            next.extend(match_pattern(graph, pattern, binding)?);
        }
        bindings = next;
    }
    Ok(bindings)
}

fn match_pattern(graph: &Graph, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, String> {
    let left = pattern.nodes.first().ok_or_else(|| "empty pattern".to_string())?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for (node_id, node) in &graph.nodes {
            if node.kind != left.kind {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id) {
                continue;
            }
            for (edge_id, edge) in &graph.edges {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let src = trinity_ram::port_node_id(&edge.source);
                let tgt = trinity_ram::port_node_id(&edge.target);
                if src != Some(node_id.as_str()) {
                    continue;
                }
                let Some(tgt_id) = tgt else { continue };
                let Some(tgt_node) = graph.nodes.get(tgt_id) else { continue };
                if tgt_node.kind != edge_pat.right.kind {
                    continue;
                }
                let mut b = base.clone();
                b.nodes.insert(left.var.clone(), node_id.clone());
                if let Some(ev) = &edge_pat.var {
                    b.edges.insert(ev.clone(), edge_id.clone());
                }
                if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                    continue;
                }
                b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                out.push(b);
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for (node_id, node) in &graph.nodes {
        if node.kind != left.kind {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id.clone());
        out.push(b);
    }
    Ok(out)
}

fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

fn eval_expr(graph: &Graph, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

fn binding_value(graph: &Graph, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    let node_id = binding.nodes.get(var)?;
    let node = graph.node(node_id)?;
    match prop {
        "id" => Some(PropertyValue::String(node.id.clone())),
        "name" => Some(PropertyValue::String(node.name.clone())),
        "kind" => Some(PropertyValue::String(node.kind.clone())),
        _ => node.properties.get(prop).cloned(),
    }
}

fn build_return(graph: &Graph, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            let val = match item {
                ReturnItem::Var(v) => binding
                    .nodes
                    .get(v)
                    .and_then(|id| graph.node(id))
                    .map(|n| PropertyValue::String(n.name.clone()))
                    .unwrap_or(PropertyValue::Null),
                ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop).unwrap_or(PropertyValue::Null),
            };
            row.push(val);
        }
        rows.push(row);
    }
    QueryResult { columns, rows }
}

fn apply_create(graph: &mut Graph, pattern: &Pattern) -> Result<(), String> {
    let left = pattern.nodes.first().ok_or_else(|| "empty create pattern".to_string())?;
    let left_id = format!("{}-{}", left.var, graph.nodes.len());
    let mut left_ports = Vec::new();
    if pattern.edge.is_some() {
        left_ports.push(Port {
            id: "out".into(),
            kind: "Connector".into(),
            direction: PortDirection::Out,
            properties: PropertyBag::new(),
        });
    }
    graph.add_node(Node {
        id: left_id.clone(),
        kind: left.kind.clone(),
        name: left.var.clone(),
        x: graph.nodes.len() as f64 * 120.0,
        y: 0.0,
        width: 80.0,
        height: 40.0,
        properties: PropertyBag::new(),
        ports: left_ports,
    });
    if let Some(edge_pat) = &pattern.edge {
        let right_id = format!("{}-{}", edge_pat.right.var, graph.nodes.len());
        graph.add_node(Node {
            id: right_id.clone(),
            kind: edge_pat.right.kind.clone(),
            name: edge_pat.right.var.clone(),
            x: graph.nodes.len() as f64 * 120.0,
            y: 80.0,
            width: 80.0,
            height: 40.0,
            properties: PropertyBag::new(),
            ports: vec![Port {
                id: "in".into(),
                kind: "Connector".into(),
                direction: PortDirection::In,
                properties: PropertyBag::new(),
            }],
        });
        let edge_id = format!("e-{}", graph.edges.len());
        graph.add_edge(Edge {
            id: edge_id,
            kind: edge_pat.kind.clone().unwrap_or_else(|| "Connection".into()),
            source: port_key(&left_id, "out"),
            target: port_key(&right_id, "in"),
            properties: PropertyBag::new(),
        });
    }
    graph.recompute_derived();
    Ok(())
}
// #endregion 🔖Executor

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use trinity_ram::{CameraV1, GraphFixtureV1, Manifest};

    fn mini_graph() -> Graph {
        let fixture = GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.into(),
            name: "mini".into(),
            manifest: Manifest::nakagin_default(),
            camera: CameraV1::default(),
            root_node_id: Some("root".into()),
            nodes: vec![
                Node {
                    id: "root".into(),
                    kind: "Piece".into(),
                    name: "core".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child".into(),
                    kind: "Piece".into(),
                    name: "capsule".into(),
                    x: 120.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            edges: vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root:out".into(),
                target: "child:in".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.0));
                    p.insert("v".into(), PropertyValue::Number(2.0));
                    p
                },
            }],
        };
        let mut g = Graph::from_fixture(fixture).unwrap();
        g.recompute_derived();
        g
    }

    #[test]
    fn parse_match_return() {
        let q = parse("MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    }

    #[test]
    fn run_match_return() {
        let mut g = mini_graph();
        let result = run(&mut g, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name").unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], PropertyValue::String("core".into()));
    }

    #[test]
    fn run_create() {
        let mut g = mini_graph();
        run(&mut g, "CREATE (n:Piece)").unwrap();
        assert_eq!(g.nodes.len(), 3);
    }

    #[test]
    fn run_set() {
        let mut g = mini_graph();
        run(&mut g, "MATCH (a:Piece) WHERE a.name = 'core' SET a.label = 'root-core'").unwrap();
        let node = g.node("root").unwrap();
        assert_eq!(node.properties.get("label"), Some(&PropertyValue::String("root-core".into())));
    }

    #[test]
    fn tokenize_keywords_and_strings() {
        let spans = tokenize("MATCH (a:Piece) WHERE a.name = 'core'");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword && s.start == 0));
        assert!(spans.iter().any(|s| s.class == TokenClass::String));
    }

    #[test]
    fn tokenize_unterminated_string_is_error() {
        let spans = tokenize("MATCH (a:Piece) WHERE a.name = 'core");
        assert!(spans.iter().any(|s| s.class == TokenClass::Error));
    }

    #[test]
    fn complete_clause_keywords() {
        let g = mini_graph();
        let items = complete(&g, "MAT", 3);
        assert!(items.iter().any(|row| row.label == "MATCH"));
    }

    #[test]
    fn complete_node_kinds_after_colon() {
        let g = mini_graph();
        let items = complete(&g, "MATCH (a:P", 11);
        assert!(items.iter().any(|row| row.label == "Piece"));
    }

    #[test]
    fn complete_properties_after_dot() {
        let g = mini_graph();
        let items = complete(&g, "MATCH (a:Piece) WHERE a.n", 25);
        assert!(items.iter().any(|row| row.label == "name"));
    }

    #[test]
    fn complete_bound_variables() {
        let g = mini_graph();
        let items = complete(&g, "MATCH (a:Piece) RETURN a", 24);
        assert!(items.iter().any(|row| row.label == "a"));
    }
}
// #endregion 🔖Tests
