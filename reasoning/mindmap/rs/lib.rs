//! 🧠 Mindmap graph extension: topics and relationships on a property graph.

pub use infinite_cavas as cavas;
pub use infinite_board_normal_directed as graph;

// #region 🔖MindmapExtension
/// 🧠 Mindmap semantics over a property graph canvas.
pub trait MindmapExtension: graph::GraphExtension {
    fn topic_label(&self, node_id: graph::NodeId) -> Option<&str>;
}

/// 🧩 Topic is a graph node; relationship is a graph edge.
pub type TopicId = graph::NodeId;
pub type RelationshipId = graph::EdgeId;

/// 🧭 Default mindmap extension stub.
#[derive(Clone, Debug, Default)]
pub struct DefaultMindmapExtension {
    pub topics: std::collections::BTreeMap<TopicId, String>,
}

impl cavas::CanvasExtension for DefaultMindmapExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/default"
    }
}

impl graph::GraphExtension for DefaultMindmapExtension {}

impl MindmapExtension for DefaultMindmapExtension {
    fn topic_label(&self, node_id: TopicId) -> Option<&str> {
        self.topics.get(&node_id).map(String::as_str)
    }
}
// #endregion 🔖MindmapExtension

// #region 🔖DocumentVcs
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use vcs::{Operation, OperationDiff};

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";

// #region 🔖Document
/// 🧠 The mindmap-wires document: the semantic wires fixture (identities/relationships/kind catalogs)
/// paired with its own `reasoning.mindmap.fixture` board fixture (nodes/edges/camera). Both are kept
/// as opaque JSON so this crate stays free of any board-engine schema types, while operations still address
/// board nodes/edges and wires relationships by id for mergeable, granular edits.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapWiresDocument {
    pub wires_fixture: Value,
    pub board_fixture: Value,
}

pub fn empty_board_fixture() -> Value {
    serde_json::json!({
        "schema": MINDMAP_BOARD_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": [],
        "wires": []
    })
}

pub fn empty_wires_fixture() -> Value {
    serde_json::json!({
        "schema": MINDMAP_WIRES_SCHEMA,
        "identities": [],
        "relationships": [],
        "board": empty_board_fixture()
    })
}

pub fn empty_mindmap_wires_document() -> MindmapWiresDocument {
    MindmapWiresDocument { wires_fixture: empty_wires_fixture(), board_fixture: empty_board_fixture() }
}

fn array_mut<'a>(fixture: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    let object = fixture.as_object_mut().expect("mindmap fixture must be a JSON object");
    object.entry(key.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    object
        .get_mut(key)
        .and_then(|value| {
            if !value.is_array() {
                *value = Value::Array(Vec::new());
            }
            value.as_array_mut()
        })
        .expect("array coerced above")
}

fn entity_id<'a>(entity: &'a Value, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

pub fn find_board_node<'a>(document: &'a MindmapWiresDocument, node_id: &str) -> Option<&'a Value> {
    document
        .board_fixture
        .get("nodes")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|node| entity_id(node, "id") == Some(node_id))
}

fn find_board_edge<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a Value> {
    document
        .board_fixture
        .get("edges")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|edge| entity_id(edge, "id") == Some(edge_id))
}

fn find_relationship<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a Value> {
    document
        .wires_fixture
        .get("relationships")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|relationship| entity_id(relationship, "edgeId") == Some(edge_id))
}
// #endregion 🔖Document

// #region 🔖Steps
/// 🧩 One atomic, absorb-concatenatable board/wires mutation — the building block of {@link MindmapWiresDiff}.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum MindmapWiresStep {
    AddNode { node: Value },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: Map<String, Value> },
    AddEdge { edge: Value, relationship: Value },
    RemoveEdge { edge_id: String },
}

fn apply_step(wires: &mut Value, board: &mut Value, step: &MindmapWiresStep) {
    match step {
        MindmapWiresStep::AddNode { node } => array_mut(board, "nodes").push(node.clone()),
        MindmapWiresStep::RemoveNode { node_id } => {
            array_mut(board, "nodes").retain(|node| entity_id(node, "id") != Some(node_id.as_str()));
        }
        MindmapWiresStep::PatchNode { node_id, patch } => {
            if let Some(node) = array_mut(board, "nodes")
                .iter_mut()
                .find(|node| entity_id(node, "id") == Some(node_id.as_str()))
            {
                if let Some(object) = node.as_object_mut() {
                    for (key, value) in patch {
                        object.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        MindmapWiresStep::AddEdge { edge, relationship } => {
            array_mut(board, "edges").push(edge.clone());
            if !relationship.is_null() {
                array_mut(wires, "relationships").push(relationship.clone());
            }
        }
        MindmapWiresStep::RemoveEdge { edge_id } => {
            array_mut(board, "edges").retain(|edge| entity_id(edge, "id") != Some(edge_id.as_str()));
            array_mut(wires, "relationships").retain(|relationship| entity_id(relationship, "edgeId") != Some(edge_id.as_str()));
        }
    }
}
// #endregion 🔖Steps

// #region 🔖Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum MindmapWiresOperation {
    AddNode { node: Value },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: Map<String, Value> },
    AddRelationship { edge: Value, relationship: Value },
    RemoveEdge { edge_id: String },
    ReplaceDocument { wires_fixture: Value, board_fixture: Value },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapWiresDiff {
    pub steps: Vec<MindmapWiresStep>,
    pub replace: Option<Box<MindmapWiresDocument>>,
}

impl OperationDiff<MindmapWiresDocument> for MindmapWiresDiff {
    fn apply(&self, projection: &MindmapWiresDocument) -> MindmapWiresDocument {
        let base = self.replace.as_ref().map(|document| (**document).clone()).unwrap_or_else(|| projection.clone());
        let mut wires = base.wires_fixture;
        let mut board = base.board_fixture;
        for step in &self.steps {
            apply_step(&mut wires, &mut board, step);
        }
        MindmapWiresDocument { wires_fixture: wires, board_fixture: board }
    }

    fn absorb(&mut self, other: Self) {
        if let Some(replace) = other.replace {
            self.replace = Some(replace);
            self.steps.clear();
        }
        self.steps.extend(other.steps);
    }
}

fn steps_diff(steps: Vec<MindmapWiresStep>) -> MindmapWiresDiff {
    MindmapWiresDiff { steps, replace: None }
}

impl Operation<MindmapWiresDocument> for MindmapWiresOperation {
    type Diff = MindmapWiresDiff;

    fn diff(&self, _projection: &MindmapWiresDocument) -> MindmapWiresDiff {
        match self {
            MindmapWiresOperation::AddNode { node } => steps_diff(vec![MindmapWiresStep::AddNode { node: node.clone() }]),
            MindmapWiresOperation::RemoveNode { node_id } => steps_diff(vec![MindmapWiresStep::RemoveNode { node_id: node_id.clone() }]),
            MindmapWiresOperation::PatchNode { node_id, patch } => {
                steps_diff(vec![MindmapWiresStep::PatchNode { node_id: node_id.clone(), patch: patch.clone() }])
            }
            MindmapWiresOperation::AddRelationship { edge, relationship } => {
                steps_diff(vec![MindmapWiresStep::AddEdge { edge: edge.clone(), relationship: relationship.clone() }])
            }
            MindmapWiresOperation::RemoveEdge { edge_id } => steps_diff(vec![MindmapWiresStep::RemoveEdge { edge_id: edge_id.clone() }]),
            MindmapWiresOperation::ReplaceDocument { wires_fixture, board_fixture } => MindmapWiresDiff {
                steps: Vec::new(),
                replace: Some(Box::new(MindmapWiresDocument {
                    wires_fixture: wires_fixture.clone(),
                    board_fixture: board_fixture.clone(),
                })),
            },
        }
    }

    fn backwards(&self, projection: &MindmapWiresDocument) -> Vec<Self> {
        match self {
            MindmapWiresOperation::AddNode { node } => entity_id(node, "id")
                .map(|node_id| vec![MindmapWiresOperation::RemoveNode { node_id: node_id.to_string() }])
                .unwrap_or_default(),
            MindmapWiresOperation::RemoveNode { node_id } => find_board_node(projection, node_id)
                .map(|node| vec![MindmapWiresOperation::AddNode { node: node.clone() }])
                .unwrap_or_default(),
            MindmapWiresOperation::PatchNode { node_id, patch } => {
                let node = find_board_node(projection, node_id);
                let inverse: Map<String, Value> = patch
                    .keys()
                    .map(|key| {
                        let prior = node.and_then(|node| node.get(key)).cloned().unwrap_or(Value::Null);
                        (key.clone(), prior)
                    })
                    .collect();
                vec![MindmapWiresOperation::PatchNode { node_id: node_id.clone(), patch: inverse }]
            }
            MindmapWiresOperation::AddRelationship { edge, .. } => entity_id(edge, "id")
                .map(|edge_id| vec![MindmapWiresOperation::RemoveEdge { edge_id: edge_id.to_string() }])
                .unwrap_or_default(),
            MindmapWiresOperation::RemoveEdge { edge_id } => find_board_edge(projection, edge_id)
                .map(|edge| MindmapWiresOperation::AddRelationship {
                    edge: edge.clone(),
                    relationship: find_relationship(projection, edge_id).cloned().unwrap_or(Value::Null),
                })
                .into_iter()
                .collect(),
            MindmapWiresOperation::ReplaceDocument { .. } => vec![MindmapWiresOperation::ReplaceDocument {
                wires_fixture: projection.wires_fixture.clone(),
                board_fixture: projection.board_fixture.clone(),
            }],
        }
    }
}

pub type MindmapWiresEnvelope = vcs::DocumentVcsEnvelope<MindmapWiresDocument, MindmapWiresOperation>;
pub type MindmapWiresStore = vcs::DocumentVcsStore<MindmapWiresDocument, MindmapWiresOperation>;
// #endregion 🔖Operations
// #endregion 🔖DocumentVcs

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, recursive-descent parser and printer for the mindmap-wires textual DSL and
/// op-text. `MindmapWiresDocument` keeps `wires_fixture`/`board_fixture` as opaque `serde_json::Value`
/// by design (see {@link MindmapWiresDocument}), so the grammar is a generic JSON-shaped value literal
/// (object/array/string/number/bool/null) with its own tokenizer — never `serde_json::from_str`/
/// `to_string` on the DSL text itself. Whitespace (including newlines) and the `,` separator are never
/// significant to the parser — `print_dsl` inserts newlines/indentation purely for readability,
/// `print_op` renders the identical grammar compactly on one line. See {@link vcs::DocumentDsl} and
/// {@link vcs::OpText}.
mod mindmap_text {
    use super::{MindmapWiresDocument, MindmapWiresOperation};
    use serde_json::{Map, Value};
    use vcs::{TextError, TextSpan};

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Ident(String),
        Str(String),
        Num(String),
        Colon,
        Comma,
        LBrace,
        RBrace,
        LBracket,
        RBracket,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct SpannedTok {
        tok: Tok,
        line: u32,
        column: u32,
    }

    /// 🔍 Char-by-char tokenizer tracking line/column so parse errors carry a jumpable `TextSpan`.
    fn lex(input: &str) -> Result<Vec<SpannedTok>, TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line: u32 = 1;
        let mut col: u32 = 1;
        while i < chars.len() {
            let c = chars[i];
            if c == '\n' {
                i += 1;
                line += 1;
                col = 1;
                continue;
            }
            if c.is_whitespace() {
                i += 1;
                col += 1;
                continue;
            }
            let (start_line, start_col) = (line, col);
            match c {
                ':' => {
                    out.push(SpannedTok { tok: Tok::Colon, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                ',' => {
                    out.push(SpannedTok { tok: Tok::Comma, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '{' => {
                    out.push(SpannedTok { tok: Tok::LBrace, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(SpannedTok { tok: Tok::RBrace, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '[' => {
                    out.push(SpannedTok { tok: Tok::LBracket, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                ']' => {
                    out.push(SpannedTok { tok: Tok::RBracket, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '"' => {
                    i += 1;
                    col += 1;
                    let mut value = String::new();
                    loop {
                        if i >= chars.len() {
                            return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
                        }
                        let ch = chars[i];
                        if ch == '"' {
                            i += 1;
                            col += 1;
                            break;
                        }
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => value.push('\n'),
                                't' => value.push('\t'),
                                '"' => value.push('"'),
                                '\\' => value.push('\\'),
                                other => value.push(other),
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '\n' {
                            value.push('\n');
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            value.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    out.push(SpannedTok { tok: Tok::Str(value), line: start_line, column: start_col });
                }
                '-' | '0'..='9' => {
                    let start = i;
                    if c == '-' {
                        i += 1;
                        col += 1;
                    }
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                        col += 1;
                    }
                    if i < chars.len() && chars[i] == '.' {
                        i += 1;
                        col += 1;
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                            col += 1;
                        }
                    }
                    if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                        i += 1;
                        col += 1;
                        if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                            i += 1;
                            col += 1;
                        }
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                            col += 1;
                        }
                    }
                    let text: String = chars[start..i].iter().collect();
                    out.push(SpannedTok { tok: Tok::Num(text), line: start_line, column: start_col });
                }
                other if other.is_ascii_alphabetic() || other == '_' => {
                    let start = i;
                    while i < chars.len() && (chars[i].is_ascii_alphanumeric() || matches!(chars[i], '_' | '-' | '.' | '/')) {
                        i += 1;
                        col += 1;
                    }
                    let text: String = chars[start..i].iter().collect();
                    out.push(SpannedTok { tok: Tok::Ident(text), line: start_line, column: start_col });
                }
                other => return Err(TextError::new(format!("unexpected character '{other}'"), TextSpan::at(start_line, start_col))),
            }
        }
        out.push(SpannedTok { tok: Tok::Eof, line, column: col });
        Ok(out)
    }

    /// 🔢 Reconstructs a `serde_json::Number` from its literal text, preserving the int/float distinction
    /// (no `.`/exponent ⇒ integer) so `.as_u64()`/`.as_i64()` keep working on parsed ids like `identityId`.
    fn number_from_text(text: &str) -> Result<serde_json::Number, String> {
        if text.contains('.') || text.contains('e') || text.contains('E') {
            let value: f64 = text.parse().map_err(|_| format!("invalid number '{text}'"))?;
            serde_json::Number::from_f64(value).ok_or_else(|| format!("non-finite number '{text}'"))
        } else if let Ok(value) = text.parse::<i64>() {
            Ok(serde_json::Number::from(value))
        } else if let Ok(value) = text.parse::<u64>() {
            Ok(serde_json::Number::from(value))
        } else {
            let value: f64 = text.parse().map_err(|_| format!("invalid number '{text}'"))?;
            serde_json::Number::from_f64(value).ok_or_else(|| format!("non-finite number '{text}'"))
        }
    }
    //#endregion Lexer

    //#region Parser
    struct Parser {
        toks: Vec<SpannedTok>,
        pos: usize,
    }

    impl Parser {
        fn new(toks: Vec<SpannedTok>) -> Self {
            Self { toks, pos: 0 }
        }

        fn peek(&self) -> &SpannedTok {
            &self.toks[self.pos]
        }

        fn advance(&mut self) -> SpannedTok {
            let tok = self.toks[self.pos].clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn skip_comma(&mut self) {
            if matches!(self.peek().tok, Tok::Comma) {
                self.advance();
            }
        }

        fn error(&self, message: impl Into<String>) -> TextError {
            let tok = self.peek();
            TextError::new(message.into(), TextSpan::at(tok.line, tok.column))
        }

        fn expect_ident(&mut self, word: &str) -> Result<(), TextError> {
            match &self.peek().tok {
                Tok::Ident(text) if text == word => {
                    self.advance();
                    Ok(())
                }
                _ => Err(self.error(format!("expected '{word}'"))),
            }
        }

        fn expect_eof(&mut self) -> Result<(), TextError> {
            match &self.peek().tok {
                Tok::Eof => Ok(()),
                _ => Err(self.error("expected end of input")),
            }
        }

        fn parse_key(&mut self) -> Result<String, TextError> {
            match self.peek().tok.clone() {
                Tok::Ident(word) => {
                    self.advance();
                    Ok(word)
                }
                Tok::Str(text) => {
                    self.advance();
                    Ok(text)
                }
                _ => Err(self.error("expected an object key")),
            }
        }

        fn parse_value(&mut self) -> Result<Value, TextError> {
            match self.peek().tok.clone() {
                Tok::LBrace => self.parse_object(),
                Tok::LBracket => self.parse_array(),
                Tok::Str(text) => {
                    self.advance();
                    Ok(Value::String(text))
                }
                Tok::Num(text) => {
                    let number = number_from_text(&text).map_err(|message| self.error(message))?;
                    self.advance();
                    Ok(Value::Number(number))
                }
                Tok::Ident(word) => {
                    self.advance();
                    Ok(match word.as_str() {
                        "true" => Value::Bool(true),
                        "false" => Value::Bool(false),
                        "null" => Value::Null,
                        _ => Value::String(word),
                    })
                }
                _ => Err(self.error("expected a value")),
            }
        }

        fn parse_object(&mut self) -> Result<Value, TextError> {
            self.advance();
            let mut map = Map::new();
            loop {
                if matches!(self.peek().tok, Tok::RBrace) {
                    self.advance();
                    break;
                }
                let key = self.parse_key()?;
                match &self.peek().tok {
                    Tok::Colon => {
                        self.advance();
                    }
                    _ => return Err(self.error("expected ':' after object key")),
                }
                let value = self.parse_value()?;
                map.insert(key, value);
                self.skip_comma();
            }
            Ok(Value::Object(map))
        }

        fn parse_array(&mut self) -> Result<Value, TextError> {
            self.advance();
            let mut items = Vec::new();
            loop {
                if matches!(self.peek().tok, Tok::RBracket) {
                    self.advance();
                    break;
                }
                items.push(self.parse_value()?);
                self.skip_comma();
            }
            Ok(Value::Array(items))
        }
    }

    fn parse_string_arg(parser: &mut Parser) -> Result<String, TextError> {
        let span = TextSpan::at(parser.peek().line, parser.peek().column);
        match parser.parse_value()? {
            Value::String(text) => Ok(text),
            _ => Err(TextError::new("expected a string argument", span)),
        }
    }

    fn parse_object_arg(parser: &mut Parser) -> Result<Map<String, Value>, TextError> {
        let span = TextSpan::at(parser.peek().line, parser.peek().column);
        match parser.parse_value()? {
            Value::Object(map) => Ok(map),
            _ => Err(TextError::new("expected an object argument", span)),
        }
    }
    //#endregion Parser

    //#region Printer
    /// 🔤 A value string is printed bare (unquoted) when it reads as an identifier — letters/digits plus
    /// `_-./` — and isn't a reserved literal; everything else is quoted with `"`.
    fn is_bare_word(text: &str) -> bool {
        if text.is_empty() || matches!(text, "true" | "false" | "null") {
            return false;
        }
        let mut chars = text.chars();
        let first = chars.next().expect("non-empty checked above");
        if !(first.is_ascii_alphabetic() || first == '_') {
            return false;
        }
        text.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/'))
    }

    fn escape_str(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                '\t' => out.push_str("\\t"),
                _ => out.push(ch),
            }
        }
        out
    }

    fn print_string(text: &str) -> String {
        if is_bare_word(text) {
            text.to_string()
        } else {
            format!("\"{}\"", escape_str(text))
        }
    }

    fn indent_str(indent: usize) -> String {
        "  ".repeat(indent)
    }

    /// 📐 Newline+indented printer used by `print_dsl` for human-readable documents.
    fn print_value_pretty(value: &Value, indent: usize) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(flag) => flag.to_string(),
            Value::Number(number) => number.to_string(),
            Value::String(text) => print_string(text),
            Value::Array(items) => {
                if items.is_empty() {
                    return "[]".to_string();
                }
                let mut out = String::from("[\n");
                for item in items {
                    out.push_str(&indent_str(indent + 1));
                    out.push_str(&print_value_pretty(item, indent + 1));
                    out.push('\n');
                }
                out.push_str(&indent_str(indent));
                out.push(']');
                out
            }
            Value::Object(map) => {
                if map.is_empty() {
                    return "{}".to_string();
                }
                let mut out = String::from("{\n");
                for (key, value) in map {
                    out.push_str(&indent_str(indent + 1));
                    out.push_str(&print_string(key));
                    out.push_str(": ");
                    out.push_str(&print_value_pretty(value, indent + 1));
                    out.push('\n');
                }
                out.push_str(&indent_str(indent));
                out.push('}');
                out
            }
        }
    }

    /// ➖ Single-space-joined printer (no newlines) used by `print_op` to embed a whole value on one line.
    fn print_value_compact(value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(flag) => flag.to_string(),
            Value::Number(number) => number.to_string(),
            Value::String(text) => print_string(text),
            Value::Array(items) => {
                let body: Vec<String> = items.iter().map(print_value_compact).collect();
                format!("[{}]", body.join(" "))
            }
            Value::Object(map) => {
                let body: Vec<String> = map.iter().map(|(key, value)| format!("{}: {}", print_string(key), print_value_compact(value))).collect();
                format!("{{{}}}", body.join(" "))
            }
        }
    }
    //#endregion Printer

    //#region DocumentText
    pub(crate) fn parse_document(text: &str) -> Result<MindmapWiresDocument, TextError> {
        let toks = lex(text)?;
        let mut parser = Parser::new(toks);
        parser.expect_ident("wires")?;
        let wires_fixture = parser.parse_value()?;
        parser.expect_ident("board")?;
        let board_fixture = parser.parse_value()?;
        parser.expect_eof()?;
        Ok(MindmapWiresDocument { wires_fixture, board_fixture })
    }

    pub(crate) fn print_document(document: &MindmapWiresDocument) -> String {
        format!(
            "wires {}\nboard {}\n",
            print_value_pretty(&document.wires_fixture, 0),
            print_value_pretty(&document.board_fixture, 0),
        )
    }
    //#endregion DocumentText

    //#region OpText
    pub(crate) fn parse_operation(line: &str) -> Result<MindmapWiresOperation, TextError> {
        let toks = lex(line)?;
        let mut parser = Parser::new(toks);
        let command = match parser.peek().tok.clone() {
            Tok::Ident(word) => {
                parser.advance();
                word
            }
            _ => return Err(parser.error("expected an operation keyword")),
        };
        let operation = match command.as_str() {
            "add-node" => MindmapWiresOperation::AddNode { node: parser.parse_value()? },
            "remove-node" => MindmapWiresOperation::RemoveNode { node_id: parse_string_arg(&mut parser)? },
            "patch-node" => {
                let node_id = parse_string_arg(&mut parser)?;
                let patch = parse_object_arg(&mut parser)?;
                MindmapWiresOperation::PatchNode { node_id, patch }
            }
            "add-relationship" => {
                let edge = parser.parse_value()?;
                let relationship = parser.parse_value()?;
                MindmapWiresOperation::AddRelationship { edge, relationship }
            }
            "remove-edge" => MindmapWiresOperation::RemoveEdge { edge_id: parse_string_arg(&mut parser)? },
            "replace-document" => {
                let wires_fixture = parser.parse_value()?;
                let board_fixture = parser.parse_value()?;
                MindmapWiresOperation::ReplaceDocument { wires_fixture, board_fixture }
            }
            other => return Err(parser.error(format!("unknown operation '{other}'"))),
        };
        parser.expect_eof()?;
        Ok(operation)
    }

    pub(crate) fn print_operation(operation: &MindmapWiresOperation) -> String {
        match operation {
            MindmapWiresOperation::AddNode { node } => format!("add-node {}", print_value_compact(node)),
            MindmapWiresOperation::RemoveNode { node_id } => format!("remove-node {}", print_string(node_id)),
            MindmapWiresOperation::PatchNode { node_id, patch } => {
                format!("patch-node {} {}", print_string(node_id), print_value_compact(&Value::Object(patch.clone())))
            }
            MindmapWiresOperation::AddRelationship { edge, relationship } => {
                format!("add-relationship {} {}", print_value_compact(edge), print_value_compact(relationship))
            }
            MindmapWiresOperation::RemoveEdge { edge_id } => format!("remove-edge {}", print_string(edge_id)),
            MindmapWiresOperation::ReplaceDocument { wires_fixture, board_fixture } => {
                format!("replace-document {} {}", print_value_compact(wires_fixture), print_value_compact(board_fixture))
            }
        }
    }
    //#endregion OpText
}

/// 📜 `.wires` textual document: `wires <value>` then `board <value>`, one hand-rolled JSON-shaped
/// value literal per section (see {@link mindmap_text}).
impl vcs::DocumentDsl for MindmapWiresDocument {
    const EXTENSION: &'static str = "wires";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        mindmap_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        mindmap_text::print_document(self)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
/// ⚡ One-line op-text: `<kebab-command> <value>...` — command names mirror the `MindmapWiresOperation`
/// variants (`add-node`, `remove-node`, `patch-node`, `add-relationship`, `remove-edge`,
/// `replace-document`), args reuse the same value grammar as `🔖Dsl`.
impl vcs::OpText for MindmapWiresOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        mindmap_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        mindmap_text::print_operation(self)
    }
}
//#endregion 🔖OpText

// #region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vcs::{apply_operation, create_document_vcs_envelope, test_support, DocumentVcsCommand, DocumentDsl, OpText};

    fn node(id: &str, text: &str) -> Value {
        json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })
    }

    fn round_trip(document: &MindmapWiresDocument, operation: &MindmapWiresOperation) -> MindmapWiresDocument {
        let forward = apply_operation(document, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(document) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, document, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn add_remove_patch_node_round_trip() {
        let document = empty_mindmap_wires_document();
        let with_node = round_trip(&document, &MindmapWiresOperation::AddNode { node: node("node-1", "Alpha") });
        assert_eq!(with_node.board_fixture["nodes"].as_array().unwrap().len(), 1);
        let mut patch = Map::new();
        patch.insert("text".into(), json!("Renamed"));
        let patched = round_trip(&with_node, &MindmapWiresOperation::PatchNode { node_id: "node-1".into(), patch });
        assert_eq!(find_board_node(&patched, "node-1").unwrap()["text"], json!("Renamed"));
        let removed = round_trip(&patched, &MindmapWiresOperation::RemoveNode { node_id: "node-1".into() });
        assert!(removed.board_fixture["nodes"].as_array().unwrap().is_empty());
    }

    #[test]
    fn add_remove_relationship_round_trip() {
        let mut document = empty_mindmap_wires_document();
        document = apply_operation(&document, &MindmapWiresOperation::AddNode { node: node("node-1", "A") });
        document = apply_operation(&document, &MindmapWiresOperation::AddNode { node: node("node-2", "B") });
        let edge = json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" });
        let relationship = json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 });
        let with_edge = round_trip(&document, &MindmapWiresOperation::AddRelationship { edge, relationship });
        assert_eq!(with_edge.board_fixture["edges"].as_array().unwrap().len(), 1);
        assert_eq!(with_edge.wires_fixture["relationships"].as_array().unwrap().len(), 1);
        let removed = round_trip(&with_edge, &MindmapWiresOperation::RemoveEdge { edge_id: "edge-1".into() });
        assert!(removed.board_fixture["edges"].as_array().unwrap().is_empty());
        assert!(removed.wires_fixture["relationships"].as_array().unwrap().is_empty());
    }

    #[test]
    fn store_applies_node_add() {
        let mut store = MindmapWiresStore::new(create_document_vcs_envelope(
            MINDMAP_WIRES_SCHEMA,
            "mindmap-wires",
            empty_mindmap_wires_document(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![MindmapWiresOperation::AddNode { node: node("node-1", "Alpha") }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").board_fixture["nodes"].as_array().unwrap().len(), 1);
    }

    //#region 🔖DslTests
    #[test]
    fn dsl_round_trip_empty_document() {
        test_support::assert_dsl_round_trip(&empty_mindmap_wires_document());
    }

    #[test]
    fn dsl_round_trip_metabolism_fixture() {
        let text = include_str!("../wires/example/metabolism.wires");
        let document = MindmapWiresDocument::parse_dsl(text).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(document.wires_fixture["identities"].as_array().unwrap().len(), 7);
        assert_eq!(document.wires_fixture["relationships"].as_array().unwrap().len(), 9);
        assert_eq!(document.board_fixture["nodes"].as_array().unwrap().len(), 7);
        test_support::assert_dsl_round_trip(&document);
    }
    //#endregion 🔖DslTests

    //#region 🔖OpTextTests
    #[test]
    fn op_text_round_trip_add_node() {
        test_support::assert_op_line_round_trip(&MindmapWiresOperation::AddNode { node: node("node-1", "Alpha") });
    }

    #[test]
    fn op_text_round_trip_remove_node() {
        test_support::assert_op_line_round_trip(&MindmapWiresOperation::RemoveNode { node_id: "node-1".into() });
    }

    #[test]
    fn op_text_round_trip_patch_node() {
        let mut patch = Map::new();
        patch.insert("text".into(), json!("Renamed"));
        patch.insert("x".into(), json!(12.5));
        test_support::assert_op_line_round_trip(&MindmapWiresOperation::PatchNode { node_id: "node-1".into(), patch });
    }

    #[test]
    fn op_text_round_trip_add_relationship() {
        let edge = json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" });
        let relationship = json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 });
        test_support::assert_op_line_round_trip(&MindmapWiresOperation::AddRelationship { edge, relationship });
    }

    #[test]
    fn op_text_round_trip_remove_edge() {
        test_support::assert_op_line_round_trip(&MindmapWiresOperation::RemoveEdge { edge_id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trip_replace_document() {
        test_support::assert_op_line_round_trip(&MindmapWiresOperation::ReplaceDocument {
            wires_fixture: empty_wires_fixture(),
            board_fixture: empty_board_fixture(),
        });
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = MindmapWiresStore::new(create_document_vcs_envelope(
            MINDMAP_WIRES_SCHEMA,
            "mindmap-wires",
            empty_mindmap_wires_document(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![MindmapWiresOperation::AddNode { node: node("node-1", "Alpha") }],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
// #endregion 🧪Tests
