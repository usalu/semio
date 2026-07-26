//! 🧮 Combined mathematical framework playground — graph algorithms and computational geometry as one hot-swappable WASM plugin.

use semio_framework_plugin::{
    app_labels, create_default_layout, is_de_locale, localized_label_map, resolve_labels, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, AppLabelsOverlay, AppLabelsOverlayExt, Canvas2dScene, DocumentApp,
    DocumentView, NodeGraphScene, SurfaceKind, UiComponentSceneNode, UiNode, UiPresence, ViewState,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use vcs::{DocumentDsl, Operation, OperationDiff};

//#region 🔖Constants
const MATH_APP_ID: &str = "mathematical-play";
const MATH_WINDOW_GRAPH: &str = "math-graph";
const MATH_WINDOW_GEOMETRY: &str = "math-geometry";
const MATH_BODY_GRAPH: &str = "mathematical.play.graph";
const MATH_BODY_GEOMETRY: &str = "mathematical.play.geometry";
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathNode {
    id: String,
    label: String,
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathEdge {
    id: String,
    source: String,
    target: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathCamera {
    x: f64,
    y: f64,
    zoom: f64,
}

impl Default for MathCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🕸️ Graph playground state: quadrant toggle, retained layout, and the active algorithm overlay.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathGraph {
    directed: bool,
    nodes: Vec<MathNode>,
    edges: Vec<MathEdge>,
    camera: MathCamera,
    algorithm: String,
    #[serde(default)]
    algorithm_seed: Option<String>,
}

impl Default for MathGraph {
    fn default() -> Self {
        Self {
            directed: true,
            nodes: vec![
                MathNode { id: "a".into(), label: "A".into(), x: 40.0, y: 60.0 },
                MathNode { id: "b".into(), label: "B".into(), x: 240.0, y: 20.0 },
                MathNode { id: "c".into(), label: "C".into(), x: 240.0, y: 180.0 },
                MathNode { id: "d".into(), label: "D".into(), x: 440.0, y: 100.0 },
            ],
            edges: vec![
                MathEdge { id: "e1".into(), source: "a".into(), target: "b".into() },
                MathEdge { id: "e2".into(), source: "a".into(), target: "c".into() },
                MathEdge { id: "e3".into(), source: "b".into(), target: "d".into() },
                MathEdge { id: "e4".into(), source: "c".into(), target: "d".into() },
            ],
            camera: MathCamera::default(),
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

/// 📐 Geometry playground state: a point cloud for convex-hull/centroid demonstration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathGeometry {
    points: Vec<(f64, f64)>,
}

impl Default for MathGeometry {
    fn default() -> Self {
        Self { points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)] }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathProjection {
    graph: MathGraph,
    geometry: MathGeometry,
}
//#endregion 🔖Document

//#region 🔖Operation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathDiff {
    #[serde(default)]
    graph: Option<MathGraph>,
    #[serde(default)]
    geometry: Option<MathGeometry>,
}

impl OperationDiff<MathProjection> for MathDiff {
    fn apply(&self, projection: &MathProjection) -> MathProjection {
        let mut next = projection.clone();
        if let Some(graph) = &self.graph {
            next.graph = graph.clone();
        }
        if let Some(geometry) = &self.geometry {
            next.geometry = geometry.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.graph.is_some() {
            self.graph = other.graph;
        }
        if other.geometry.is_some() {
            self.geometry = other.geometry;
        }
    }
}

/// 📤 Coarse-grained operations: each replaces one top-level projection slice; `backwards` snapshots the pre-state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
enum MathOperation {
    SetGraph { graph: MathGraph },
    SetGeometry { geometry: MathGeometry },
}

impl Operation<MathProjection> for MathOperation {
    type Diff = MathDiff;

    fn diff(&self, _projection: &MathProjection) -> MathDiff {
        match self {
            MathOperation::SetGraph { graph } => MathDiff { graph: Some(graph.clone()), geometry: None },
            MathOperation::SetGeometry { geometry } => MathDiff { graph: None, geometry: Some(geometry.clone()) },
        }
    }

    fn backwards(&self, projection: &MathProjection) -> Vec<Self> {
        match self {
            MathOperation::SetGraph { .. } => vec![MathOperation::SetGraph { graph: projection.graph.clone() }],
            MathOperation::SetGeometry { .. } => vec![MathOperation::SetGeometry { geometry: projection.geometry.clone() }],
        }
    }
}
//#endregion 🔖Operation

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer for the `.mathematical` DSL and `MathOperation`'s single-line
/// op-log encoding. `MathGraph`'s nodes/edges are a plain id/label/position + id/source/target shape (no
/// port/kind/property bag), so it does not match `mathematical/graph/dsl`'s wire-literal notation
/// (`id:kind@port -> id:kind@port {props}`) closely enough to reuse verbatim — this grammar instead
/// mirrors that precedent in spirit (`source -> target` arrows for directed edges) plus this codebase's
/// own `note` DSL conventions (`key=value` header tokens, comma-pair `x,y` point lists). Whitespace
/// (including newlines) is never significant to the parser — `print_dsl` inserts newlines purely for
/// readability, `print_op` renders the identical grammar space-joined on one line. See
/// {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod math_text {
    use super::{MathCamera, MathEdge, MathGeometry, MathGraph, MathNode, MathProjection};
    use std::collections::HashMap;

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Word(String),
        Str(String),
        Eof,
    }

    #[derive(Clone, Debug)]
    struct Lexed {
        tok: Tok,
        span: vcs::TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`"`, so `key=value`,
    /// `source->target`-free arrows (`->` is its own bareword token, space-delimited) and `x,y` point
    /// pairs all collapse into single tokens handled downstream — only a quoted value forces a boundary.
    fn lex(input: &str) -> Result<Vec<Lexed>, vcs::TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line = 1u32;
        let mut col = 1u32;
        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' | '\r' => {
                    i += 1;
                    col += 1;
                }
                '\n' => {
                    i += 1;
                    line += 1;
                    col = 1;
                }
                '"' => {
                    let (start_line, start_col) = (line, col);
                    i += 1;
                    col += 1;
                    let mut s = String::new();
                    let mut closed = false;
                    while i < chars.len() {
                        let ch = chars[i];
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => s.push('\n'),
                                '"' => s.push('"'),
                                '\\' => s.push('\\'),
                                other => {
                                    s.push('\\');
                                    s.push(other);
                                }
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '"' {
                            i += 1;
                            col += 1;
                            closed = true;
                            break;
                        } else if ch == '\n' {
                            s.push(ch);
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            s.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    if !closed {
                        return Err(vcs::TextError::new("unterminated string literal", vcs::TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: vcs::TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: vcs::TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: vcs::TextSpan::at(line, col) });
        Ok(out)
    }
    //#endregion Lexer

    //#region Parser
    #[derive(Clone, Debug)]
    enum FieldValue {
        Str(String),
        Word(String),
    }

    struct Parser {
        toks: Vec<Lexed>,
        pos: usize,
    }

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> vcs::TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_eof(&self) -> bool {
            matches!(self.peek(), Tok::Eof)
        }

        fn peek_word(&self) -> Option<&str> {
            match self.peek() {
                Tok::Word(w) => Some(w.as_str()),
                _ => None,
            }
        }

        fn expect_word(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(vcs::TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_keyword(&mut self, keyword: &str) -> Result<(), vcs::TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            if word != keyword {
                return Err(vcs::TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
            }
            Ok(())
        }

        fn expect_str(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(vcs::TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }

        /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one — shared
        /// by the `graph`/`camera`/`node` header lines.
        fn parse_kv_map(&mut self) -> Result<HashMap<String, (FieldValue, vcs::TextSpan)>, vcs::TextError> {
            let mut map = HashMap::new();
            loop {
                let word = match self.peek() {
                    Tok::Word(w) if w.contains('=') => w.clone(),
                    _ => break,
                };
                let span = self.span();
                self.bump();
                let (key, rest) = word.split_once('=').expect("word already checked to contain '='");
                let value = if rest.is_empty() {
                    FieldValue::Str(self.expect_str()?)
                } else {
                    FieldValue::Word(rest.to_string())
                };
                map.insert(key.to_string(), (value, span));
            }
            Ok(map)
        }

        /// ✒️ Consumes the current token only if it is an `x,y` coordinate pair — the terminator for the
        /// geometry `points` list is simply the first token that doesn't match.
        fn try_point(&mut self) -> Option<(f64, f64)> {
            if let Tok::Word(w) = self.peek() {
                if let Some((a, b)) = w.split_once(',') {
                    if let (Ok(x), Ok(y)) = (a.parse::<f64>(), b.parse::<f64>()) {
                        self.bump();
                        return Some((x, y));
                    }
                }
            }
            None
        }
    }

    type FieldMap = HashMap<String, (FieldValue, vcs::TextSpan)>;

    fn kv_word(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => Ok(w.clone()),
            Some((FieldValue::Str(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_str(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(s.clone()),
            Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_num(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<f64, vcs::TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<f64>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a number"), span, "number"))
    }

    fn kv_bool(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<bool, vcs::TextError> {
        match kv_word(map, key, span)?.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(vcs::TextError::expected(format!("field '{key}' must be 'true' or 'false'"), span, "true|false")),
        }
    }

    fn kv_opt_str(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<Option<String>, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(Some(s.clone())),
            Some((FieldValue::Word(w), _)) if w == "-" => Ok(None),
            Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string or '-'"), *field_span, "string|-")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    /// 🕸️ Parses the `graph directed=.. algorithm=.. seed=..` header plus any following `camera`/`node`/
    /// `edge` lines (order-independent, each zero-or-more except `camera` which is required exactly
    /// once) — shared by {@link parse_document} and {@link parse_operation}'s `SetGraph` case, since a
    /// `SetGraph` op-line is precisely this same grammar on its own.
    fn parse_graph(p: &mut Parser) -> Result<MathGraph, vcs::TextError> {
        let span = p.span();
        p.expect_keyword("graph")?;
        let map = p.parse_kv_map()?;
        let directed = kv_bool(&map, "directed", span)?;
        let algorithm = kv_str(&map, "algorithm", span)?;
        let algorithm_seed = kv_opt_str(&map, "seed", span)?;

        let mut camera = None;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        loop {
            let Some(keyword) = p.peek_word().map(|w| w.to_string()) else { break };
            match keyword.as_str() {
                "camera" => {
                    let cam_span = p.span();
                    p.bump();
                    let cam_map = p.parse_kv_map()?;
                    camera = Some(MathCamera {
                        x: kv_num(&cam_map, "x", cam_span)?,
                        y: kv_num(&cam_map, "y", cam_span)?,
                        zoom: kv_num(&cam_map, "zoom", cam_span)?,
                    });
                }
                "node" => {
                    p.bump();
                    let node_span = p.span();
                    let id = p.expect_word()?;
                    let label = p.expect_str()?;
                    let node_map = p.parse_kv_map()?;
                    nodes.push(MathNode {
                        id,
                        label,
                        x: kv_num(&node_map, "x", node_span)?,
                        y: kv_num(&node_map, "y", node_span)?,
                    });
                }
                "edge" => {
                    p.bump();
                    let id = p.expect_word()?;
                    let source = p.expect_word()?;
                    p.expect_keyword("->")?;
                    let target = p.expect_word()?;
                    edges.push(MathEdge { id, source, target });
                }
                _ => break,
            }
        }

        let camera = camera.ok_or_else(|| vcs::TextError::new("missing 'camera' line", span))?;
        Ok(MathGraph { directed, nodes, edges, camera, algorithm, algorithm_seed })
    }

    /// 📐 Parses the `points x,y x,y ...` line (present or entirely omitted — an absent line means no
    /// points, mirrored by {@link print_geometry} only emitting it when non-empty).
    fn parse_geometry(p: &mut Parser) -> Result<MathGeometry, vcs::TextError> {
        p.expect_keyword("points")?;
        let mut points = Vec::new();
        while let Some(point) = p.try_point() {
            points.push(point);
        }
        Ok(MathGeometry { points })
    }

    /// 📥 Parses a full `.mathematical` document: the `graph` block (with its nested `camera`/`node`/
    /// `edge` lines) followed by an optional `points` line for the geometry playground.
    pub(super) fn parse_document(text: &str) -> Result<MathProjection, vcs::TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };
        let graph = parse_graph(&mut p)?;
        let geometry = if p.peek_word() == Some("points") { parse_geometry(&mut p)? } else { MathGeometry { points: Vec::new() } };
        if !p.at_eof() {
            return Err(vcs::TextError::new(format!("unexpected trailing input near {:?}", p.peek()), p.span()));
        }
        Ok(MathProjection { graph, geometry })
    }

    /// ⚡ Parses one op-log line — dispatches on its leading keyword, since `SetGraph`/`SetGeometry`
    /// reprint exactly the `graph`/`points` grammar shared with {@link parse_document}.
    pub(super) fn parse_operation(line: &str) -> Result<super::MathOperation, vcs::TextError> {
        use super::MathOperation;
        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let operation = match p.peek_word() {
            Some("graph") => MathOperation::SetGraph { graph: parse_graph(&mut p)? },
            Some("points") => MathOperation::SetGeometry { geometry: parse_geometry(&mut p)? },
            _ => return Err(vcs::TextError::expected("expected 'graph' or 'points'", p.span(), "graph|points")),
        };
        if !p.at_eof() {
            return Err(vcs::TextError::new(format!("unexpected trailing input near {:?}", p.peek()), p.span()));
        }
        Ok(operation)
    }
    //#endregion Parser

    //#region Printer
    fn quote(value: &str) -> String {
        let mut out = String::with_capacity(value.len() + 2);
        out.push('"');
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    fn fmt_num(value: f64) -> String {
        value.to_string()
    }

    /// 🕸️ Renders `graph`'s header line, then its `camera` line, then one `node ...` line per node and
    /// one `edge id source -> target` arrow line per edge (see module docs for why an arrow rather than
    /// the wire-literal grammar) — newline-joined when `pretty`, space-joined on one line otherwise.
    fn print_graph(graph: &MathGraph, pretty: bool) -> String {
        let mut parts = Vec::new();
        let seed = graph.algorithm_seed.as_deref().map(quote).unwrap_or_else(|| "-".to_string());
        parts.push(format!("graph directed={} algorithm={} seed={seed}", graph.directed, quote(&graph.algorithm)));
        parts.push(format!("camera x={} y={} zoom={}", fmt_num(graph.camera.x), fmt_num(graph.camera.y), fmt_num(graph.camera.zoom)));
        for node in &graph.nodes {
            parts.push(format!("node {} {} x={} y={}", node.id, quote(&node.label), fmt_num(node.x), fmt_num(node.y)));
        }
        for edge in &graph.edges {
            parts.push(format!("edge {} {} -> {}", edge.id, edge.source, edge.target));
        }
        parts.join(if pretty { "\n" } else { " " })
    }

    /// 📐 Renders the geometry point cloud as `points x,y x,y ...`, wrapping to a fresh line every 8
    /// points when `pretty` (mirrors `note`'s stroke `points` printer) — omitted by callers entirely
    /// when there are no points.
    fn print_geometry(geometry: &MathGeometry, pretty: bool) -> String {
        if geometry.points.is_empty() {
            return "points".to_string();
        }
        let mut out = String::from("points");
        for (index, &(x, y)) in geometry.points.iter().enumerate() {
            if pretty && index > 0 && index % 8 == 0 {
                out.push('\n');
            } else {
                out.push(' ');
            }
            out.push_str(&format!("{},{}", fmt_num(x), fmt_num(y)));
        }
        out
    }

    /// 📤 Renders the full document: the `graph` block, then a `points` line only if the geometry
    /// playground has any points (see {@link parse_document} for the mirrored grammar).
    pub(super) fn print_document(projection: &MathProjection, pretty: bool) -> String {
        let mut parts = vec![print_graph(&projection.graph, pretty)];
        if !projection.geometry.points.is_empty() {
            parts.push(print_geometry(&projection.geometry, pretty));
        }
        parts.join(if pretty { "\n" } else { " " })
    }

    /// ⚡ Renders one `MathOperation` as a single line, reusing the compact (space-joined) form of
    /// {@link print_graph}/{@link print_geometry}.
    pub(super) fn print_operation(operation: &super::MathOperation) -> String {
        use super::MathOperation;
        match operation {
            MathOperation::SetGraph { graph } => print_graph(graph, false),
            MathOperation::SetGeometry { geometry } => print_geometry(geometry, false),
        }
    }
    //#endregion Printer
}

impl DocumentDsl for MathProjection {
    const EXTENSION: &'static str = "mathematical";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        math_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        math_text::print_document(self, true)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for MathOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        math_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        math_text::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
fn algorithm_overlay(graph: &MathGraph) -> std::collections::HashMap<String, String> {
    use mathematical_graph::algorithms::{adjacency, bfs_distances, connected_components, strongly_connected_components, topo_sort, IdIndex};

    let index = IdIndex::from_ids(graph.nodes.iter().map(|n| n.id.as_str()));
    let edge_pairs: Vec<(usize, usize)> = graph.edges.iter().filter_map(|e| Some((index.index_of(&e.source)?, index.index_of(&e.target)?))).collect();
    let adj = adjacency(index.len(), &edge_pairs, graph.directed);
    let mut overlay = std::collections::HashMap::new();

    match graph.algorithm.as_str() {
        "topo" => match topo_sort(&adj) {
            Ok(order) => {
                for (rank, &i) in order.iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" #{rank}"));
                    }
                }
            }
            Err(_) => {
                for node in &graph.nodes {
                    overlay.insert(node.id.clone(), " ⟲".into());
                }
            }
        },
        "components" => {
            for (i, label) in connected_components(&adj).into_iter().enumerate() {
                if let Some(id) = index.id_of(i) {
                    overlay.insert(id.to_string(), format!(" ⬤{label}"));
                }
            }
        }
        "scc" => {
            for (group, component) in strongly_connected_components(&adj).into_iter().enumerate() {
                for i in component {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" ⬤{group}"));
                    }
                }
            }
        }
        "bfs" => {
            if let Some(seed) = graph.algorithm_seed.as_deref().and_then(|s| index.index_of(s)) {
                for (i, dist) in bfs_distances(&adj, seed).into_iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), dist.map(|d| format!(" d{d}")).unwrap_or_else(|| " ∞".into()));
                    }
                }
            }
        }
        _ => {}
    }
    overlay
}

fn media_graph_json(graph: &MathGraph) -> (String, String) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<Value> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            json!({
                "id": node.id,
                "label": format!("{}{}", node.label, suffix),
                "x": node.x,
                "y": node.y,
                "width": 72.0,
                "height": 40.0,
                "inputs": [],
                "outputs": [],
            })
        })
        .collect();
    let edges: Vec<Value> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "sourceNodeId": edge.source,
                "sourcePortId": "out",
                "targetNodeId": edge.target,
                "targetPortId": "in",
            })
        })
        .collect();
    (serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()))
}
//#endregion 🔖GraphAlgorithms

//#region 🔖Geometry
fn geometry_layers_json(geometry: &MathGeometry) -> String {
    let points: Vec<mathematical_geometry::Point> = geometry.points.iter().map(|&(x, y)| mathematical_geometry::Point::new(x, y)).collect();
    let hull = mathematical_geometry::convex_hull(&points);
    let centroid = mathematical_geometry::polygon_centroid(&hull);

    let mut layers: Vec<Value> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        layers.push(json!({ "kind": "circle", "id": format!("point-{i}"), "x": p.x() - 5.0, "y": p.y() - 5.0, "width": 10.0, "height": 10.0, "color": "#38bdf8" }));
    }
    if hull.len() >= 2 {
        let mut hull_points: Vec<[f64; 2]> = Vec::new();
        for i in 0..hull.len() {
            let a = hull[i];
            let b = hull[(i + 1) % hull.len()];
            hull_points.push([a.x(), a.y()]);
            hull_points.push([b.x(), b.y()]);
        }
        layers.push(json!({ "kind": "polyline", "id": "hull", "points": hull_points, "color": "#facc15" }));
    }
    layers.push(json!({ "kind": "circle", "id": "centroid", "x": centroid.x() - 4.0, "y": centroid.y() - 4.0, "width": 8.0, "height": 8.0, "color": "#f472b6" }));
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖Geometry

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the mathematical app; one field per label makes every locale combination compile-checked.
/// 🧮 Graph/node/geometry vocabulary here is pure math terminology, not building-assembly terminology, so no reuse variant applies.
app_labels! {
    struct MathematicalLabels {
        window_graph: &'static str = en: "Graph", de: "Graph";
        window_geometry: &'static str = en: "Geometry", de: "Geometrie";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
        example_demo: &'static str = en: "Demo", de: "Demo";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation declared in `create_mathematical_app`'s static manifest —
/// the manifest itself has no `view_state`/locale parameter, so this overlay is how the command palette and Actions
/// rail get a translated label without threading locale through the whole builder chain.
fn mathematical_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("setDocument", "Set Document", "Dokument festlegen"),
            ("setAlgorithm", "Set Algorithm", "Algorithmus festlegen"),
            ("setDirected", "Set Directed", "Gerichtet festlegen"),
            ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
            ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
            ("setPoints", "Set Points", "Punkte festlegen"),
        ],
    )
}
//#endregion 🔖CommandLabels

//#region 🔖Render
fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: MATH_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
    }
}

fn render_graph_window(graph: &MathGraph) -> UiNode {
    let (nodes_json, edges_json) = media_graph_json(graph);
    let viewport_json = serde_json::to_string(&graph.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let mut scene = empty_component_scene(MATH_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes_json, edges_json, viewport_json) });
    UiNode::ComponentScene(scene)
}

fn render_geometry_window(geometry: &MathGeometry) -> UiNode {
    let mut scene = empty_component_scene(MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d);
    scene.canvas_2d = Some(Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖Render

//#region 🔖MathematicalPlayApp
#[derive(Default)]
struct MathematicalPlayApp;

impl DocumentApp for MathematicalPlayApp {
    type Projection = MathProjection;
    type Operation = MathOperation;

    fn app_id(&self) -> &str {
        MATH_APP_ID
    }

    fn document_schema(&self) -> &str {
        "semio.mathematical/v1"
    }

    fn initial_projection(&self) -> MathProjection {
        MathProjection::default()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, MathProjection>, _view_state: &ViewState) -> ActionEmit<MathOperation> {
        match action {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<MathProjection>(value.clone()).ok()) {
                    let mut operations = Vec::new();
                    if next.graph != doc.projection.graph {
                        operations.push(MathOperation::SetGraph { graph: next.graph });
                    }
                    if next.geometry != doc.projection.geometry {
                        operations.push(MathOperation::SetGeometry { geometry: next.geometry });
                    }
                    return ActionEmit::operations(operations);
                }
            }
            "setAlgorithm" => {
                if let Some(algorithm) = args.and_then(|value| value.get("algorithm")).and_then(Value::as_str) {
                    let mut graph = doc.projection.graph.clone();
                    graph.algorithm = algorithm.to_string();
                    graph.algorithm_seed = args.and_then(|value| value.get("seed")).and_then(Value::as_str).map(str::to_string);
                    return ActionEmit::commit(vec![MathOperation::SetGraph { graph }], "setAlgorithm");
                }
            }
            "setDirected" => {
                if let Some(directed) = args.and_then(|value| value.get("directed")).and_then(Value::as_bool) {
                    let mut graph = doc.projection.graph.clone();
                    graph.directed = directed;
                    return ActionEmit::operations(vec![MathOperation::SetGraph { graph }]);
                }
            }
            "nodeGraphEdit" => {
                let edit_operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let mut graph = doc.projection.graph.clone();
                let mut changed = false;
                for operation in edit_operations {
                    match operation.get("operation").and_then(Value::as_str).unwrap_or("") {
                        "addNode" => {
                            let x = operation.get("x").and_then(Value::as_f64).unwrap_or(0.0);
                            let y = operation.get("y").and_then(Value::as_f64).unwrap_or(0.0);
                            let id = format!("n{}", graph.nodes.len());
                            graph.nodes.push(MathNode { label: id.to_uppercase(), id, x, y });
                            changed = true;
                        }
                        "connect" => {
                            if let (Some(source), Some(target)) = (operation.get("sourceNodeId").and_then(Value::as_str), operation.get("targetNodeId").and_then(Value::as_str)) {
                                let id = format!("e{}", graph.edges.len());
                                graph.edges.push(MathEdge { id, source: source.into(), target: target.into() });
                                changed = true;
                            }
                        }
                        "deleteSelection" => {
                            if let Some(ids) = operation.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                                graph.nodes.retain(|node| !ids.contains(&node.id));
                                graph.edges.retain(|edge| !ids.contains(&edge.source) && !ids.contains(&edge.target));
                                changed = true;
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    return ActionEmit::operations(vec![MathOperation::SetGraph { graph }]);
                }
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(Value::as_str) {
                    if let Ok(camera) = serde_json::from_str::<MathCamera>(viewport_json) {
                        let mut graph = doc.projection.graph.clone();
                        graph.camera = camera;
                        return ActionEmit::amend(vec![MathOperation::SetGraph { graph }], "viewport");
                    }
                }
            }
            "setPoints" => {
                if let Some(points) = args.and_then(|value| value.get("points")).and_then(|value| serde_json::from_value::<Vec<(f64, f64)>>(value.clone()).ok()) {
                    return ActionEmit::operations(vec![MathOperation::SetGeometry { geometry: MathGeometry { points } }]);
                }
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, MathProjection>, _view_state: &ViewState) -> UiNode {
        match body_key {
            MATH_BODY_GRAPH => render_graph_window(&doc.projection.graph),
            MATH_BODY_GEOMETRY => render_geometry_window(&doc.projection.geometry),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<MathematicalLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(MATH_WINDOW_GRAPH, labels.window_graph)
            .window_kind_label(MATH_WINDOW_GEOMETRY, labels.window_geometry)
            .mode_label("edit", labels.mode_edit)
            .action_labels(mathematical_action_labels(is_de))
            .example_labels(std::collections::HashMap::from([("demo".to_string(), labels.example_demo.to_string())]))
    }
}
//#endregion 🔖MathematicalPlayApp

//#region 🔖Manifest
fn create_mathematical_app() -> App {
    App::from_builder(
        App::builder(MATH_APP_ID, "Mathematical")
            .document(["semio", "mathematical"])
            .icon_id("sigma")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(MATH_WINDOW_GRAPH, "Graph", MATH_BODY_GRAPH, SurfaceKind::NodeGraph, "network")
            .window_kind(MATH_WINDOW_GEOMETRY, "Geometry", MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d, "hexagon")
            .default_layout(create_default_layout(&[MATH_WINDOW_GRAPH.into(), MATH_WINDOW_GEOMETRY.into()], "row", Some(&[60.0, 40.0]), Some(&["Graph".into(), "Geometry".into()])))
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setDocument", "Set Document", ActionKind::Operation) })
            .operation("setAlgorithm", "Set Algorithm")
            .operation("setDirected", "Set Directed")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("nodeGraphViewport", "Node Graph Viewport")
            .operation("setPoints", "Set Points")
            // 📝 Staged argument forms for the graph analysis controls.
            .action_args("setAlgorithm", vec![
                ActionArgDef::select("algorithm", "Algorithm", vec![
                    ActionArgOption::new("topo", "Topological Order"),
                    ActionArgOption::new("components", "Connected Components"),
                    ActionArgOption::new("scc", "Strongly Connected Components"),
                    ActionArgOption::new("bfs", "Breadth-First Distances"),
                ]).required(),
            ])
            .action_args("setDirected", vec![
                ActionArgDef::toggle("directed", "Directed").default_value(true),
            ]),
    )
    .example("demo", "Demo", MathProjection::default().print_dsl())
    .program("mathematical", "Mathematical", "graph")
}

fn register_mathematical_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "mathematical", label: "Mathematical", version: "0.1.0",
    setup: register_mathematical_exports,
    apps: [ create_mathematical_app => MathematicalPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topo_algorithm_overlay_orders_dag_nodes() {
        let graph = MathGraph::default();
        let overlay = algorithm_overlay(&graph);
        assert!(overlay.get("a").unwrap().starts_with(" #0"));
        assert!(overlay.get("d").unwrap().starts_with(" #"));
    }

    #[test]
    fn components_algorithm_overlay_groups_disconnected_node() {
        let mut graph = MathGraph::default();
        graph.algorithm = "components".into();
        graph.nodes.push(MathNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
        let overlay = algorithm_overlay(&graph);
        assert_ne!(overlay.get("a"), overlay.get("z"));
    }

    #[test]
    fn bfs_algorithm_overlay_reports_hop_distance() {
        let mut graph = MathGraph::default();
        graph.algorithm = "bfs".into();
        graph.algorithm_seed = Some("a".into());
        let overlay = algorithm_overlay(&graph);
        assert_eq!(overlay.get("a").unwrap(), " d0");
        assert_eq!(overlay.get("b").unwrap(), " d1");
    }

    #[test]
    fn media_graph_json_round_trips_node_count() {
        let graph = MathGraph::default();
        let (nodes_json, edges_json) = media_graph_json(&graph);
        let nodes: Vec<Value> = serde_json::from_str(&nodes_json).unwrap();
        let edges: Vec<Value> = serde_json::from_str(&edges_json).unwrap();
        assert_eq!(nodes.len(), graph.nodes.len());
        assert_eq!(edges.len(), graph.edges.len());
    }

    #[test]
    fn geometry_layers_include_hull_and_centroid() {
        let geometry = MathGeometry::default();
        let layers_json = geometry_layers_json(&geometry);
        assert!(layers_json.contains("\"hull\""));
        assert!(layers_json.contains("\"centroid\""));
    }

    #[test]
    fn renders_node_graph_scene() {
        let app = MathematicalPlayApp;
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None };
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(MATH_BODY_GRAPH, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_canvas_2d_scene() {
        let app = MathematicalPlayApp;
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None };
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(MATH_BODY_GEOMETRY, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    //#region 🔖DslTests
    #[test]
    fn math_projection_dsl_round_trips_default() {
        vcs::test_support::assert_dsl_round_trip(&MathProjection::default());
    }

    #[test]
    fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
        let mut graph = MathGraph::default();
        graph.algorithm = "bfs".into();
        graph.algorithm_seed = Some("a".into());
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathProjection { graph, geometry: MathGeometry { points: Vec::new() } };
        vcs::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn math_set_graph_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&MathOperation::SetGraph { graph: MathGraph::default() });
    }

    #[test]
    fn math_set_geometry_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&MathOperation::SetGeometry { geometry: MathGeometry::default() });
    }

    #[test]
    fn math_document_text_round_trips_through_store() {
        let initial = MathProjection::default();
        let envelope = vcs::create_document_vcs_envelope("semio.mathematical/v1", "math-demo", initial, None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        let mut graph = MathGraph::default();
        graph.algorithm = "components".into();
        store
            .dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![MathOperation::SetGraph { graph }], description: None })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DslTests
}
//#endregion 🧪Tests
