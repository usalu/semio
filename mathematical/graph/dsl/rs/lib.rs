//! 🃏 Shared Jack query language for mathematical graph frameworks.

// #region ⚠️ Errors
/// 🚧 Unified failure mode for jack parsing/execution, wire-literal parsing, and fixture ingestion.
#[derive(Debug, thiserror::Error)]
pub enum GraphDslError {
    /// 🧾 Fixture or query-result JSON failed to parse or serialize.
    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),
    /// 🔤 A string literal was never closed.
    #[error("unterminated string literal")]
    UnterminatedString,
    /// ❓ A byte outside the token grammar was found.
    #[error("unexpected character '{0}'")]
    UnexpectedChar(char),
    /// 🔢 A numeric literal's bytes were not valid utf-8.
    #[error("invalid number literal: {0}")]
    NumberUtf8(#[from] std::str::Utf8Error),
    /// 🔢 A numeric literal did not parse as a float.
    #[error("invalid number literal: {0}")]
    NumberFormat(#[from] std::num::ParseFloatError),
    /// ➡️ Parser expected one token shape and found another.
    #[error("expected {expected}, got {found}")]
    UnexpectedToken { expected: String, found: String },
    /// 🪝 A wire-literal edge's target endpoint was missing its `@port`.
    #[error("edge target requires @port")]
    EdgeTargetMissingPort,
    /// 🕸️ A jack pattern had no nodes.
    #[error("empty pattern")]
    EmptyPattern,
    /// 🚫 CREATE/DELETE/SET/MERGE are not supported on read-only queryable graphs.
    #[error("mutating jack clauses are not supported on this graph domain")]
    UnsupportedMutation,
}
// #endregion ⚠️ Errors

pub mod queryable {
    // #region queryable
    //! 🔍 Queryable graph interface for Jack.

    use crate::GraphDslError;
    use mathematical_graph_manifest::{manifest_by_id, GraphManifest, PropertyBag, PropertyValue};
    use serde_json::Value;
    use std::collections::{BTreeMap, BTreeSet};

    // #region 🔖QueryableEdge
    /// 🪢 Edge row exposed to Jack matching.
    #[derive(Clone, Debug, PartialEq)]
    pub struct QueryableEdge {
        pub id: String,
        pub kind: String,
        pub source_node_id: String,
        pub target_node_id: String,
        pub source_port: Option<String>,
        pub target_port: Option<String>,
        pub properties: PropertyBag,
    }
    // #endregion 🔖QueryableEdge

    // #region 🔖QueryableGraph
    /// 🕸️ Read-only graph surface for Jack query execution.
    pub trait QueryableGraph {
        fn manifest(&self) -> Option<&GraphManifest>;
        fn node_ids(&self) -> Vec<String>;
        fn node_kind(&self, id: &str) -> Option<String>;
        fn node_name(&self, id: &str) -> Option<String>;
        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue>;
        fn edges(&self) -> Vec<QueryableEdge>;
        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String>;
    }

    pub fn manifest_node_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for id in graph.node_ids() {
            if let Some(kind) = graph.node_kind(id.as_str()) {
                kinds.insert(kind);
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.node_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub fn manifest_edge_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            kinds.insert(edge.kind.clone());
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.edge_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub fn manifest_property_names(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
        for id in graph.node_ids() {
            for key in ["label", "text"] {
                if graph.node_property(id.as_str(), key).is_some() {
                    props.insert(key.to_string());
                }
            }
            if let Some(PropertyValue::Object(map)) = graph.node_property(id.as_str(), "__all") {
                for key in map.keys() {
                    props.insert(key.clone());
                }
            }
        }
        props.into_iter().collect()
    }

    pub fn manifest_port_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            if let Some(port) = &edge.source_port {
                kinds.insert(port.clone());
            }
            if let Some(port) = &edge.target_port {
                kinds.insert(port.clone());
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.port_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }
    // #endregion 🔖QueryableGraph

    // #region 🔖BoardQueryableGraph
    fn json_to_property_bag(value: &Value) -> PropertyBag {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    fn split_endpoint(endpoint: &str, handle_to_node: &BTreeMap<String, String>) -> (String, Option<String>) {
        if let Some(node_id) = handle_to_node.get(endpoint) {
            return (node_id.clone(), None);
        }
        if let Some((node, port)) = endpoint.split_once('@') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once(':') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once('.') {
            if handle_to_node.contains_key(endpoint) {
                return (handle_to_node[endpoint].clone(), None);
            }
            return (node.to_string(), Some(port.to_string()));
        }
        (endpoint.to_string(), None)
    }

    /// 🧩 Jack query target over board/scene fixture JSON.
    pub struct BoardQueryableGraph {
        manifest: Option<GraphManifest>,
        nodes: BTreeMap<String, (String, String, PropertyBag)>,
        edges: Vec<QueryableEdge>,
        raw_fixture: Value,
    }

    impl BoardQueryableGraph {
        pub fn from_fixture_json(json: &str, manifest_id: Option<&str>) -> Result<Self, GraphDslError> {
            let raw: Value = serde_json::from_str(json)?;
            let manifest = manifest_id.and_then(manifest_by_id).or_else(|| raw.get("manifestId").and_then(|v| v.as_str()).and_then(manifest_by_id)).or_else(|| raw.get("manifest_id").and_then(|v| v.as_str()).and_then(manifest_by_id));
            let mut nodes = BTreeMap::new();
            let mut handle_to_node = BTreeMap::new();
            if let Some(rows) = raw.get("nodes").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let name = obj.get("text").or_else(|| obj.get("name")).or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id).to_string();
                    let mut properties = obj.get("userData").or_else(|| obj.get("user_data")).map(json_to_property_bag).unwrap_or_default();
                    for (key, value) in obj.iter() {
                        if matches!(key.as_str(), "id" | "nodeKind" | "node_kind" | "kind" | "text" | "name" | "label" | "handles" | "x" | "y" | "shape" | "radius" | "width" | "height" | "userData" | "user_data") {
                            continue;
                        }
                        if let Ok(prop) = serde_json::from_value::<PropertyValue>(value.clone()) {
                            properties.insert(key.clone(), prop);
                        }
                    }
                    nodes.insert(id.to_string(), (kind, name, properties));
                    if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
                        for handle in handles {
                            if let Some(hid) = handle.get("id").and_then(|v| v.as_str()) {
                                handle_to_node.insert(hid.to_string(), id.to_string());
                            }
                        }
                    }
                }
            }
            let mut edges = Vec::new();
            if let Some(rows) = raw.get("edges").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let Some(source) = obj.get("source").and_then(|v| v.as_str()) else { continue };
                    let Some(target) = obj.get("target").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("edgeKind").or_else(|| obj.get("edge_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let properties = obj.get("userData").or_else(|| obj.get("user_data")).map(json_to_property_bag).unwrap_or_default();
                    let (source_node_id, source_port) = split_endpoint(source, &handle_to_node);
                    let (target_node_id, target_port) = split_endpoint(target, &handle_to_node);
                    edges.push(QueryableEdge { id: id.to_string(), kind, source_node_id, target_node_id, source_port, target_port, properties });
                }
            }
            Ok(Self { manifest, nodes, edges, raw_fixture: raw })
        }

        pub fn from_dag_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            Self::from_fixture_json(json, Some("flow-dag"))
        }

        pub fn from_puzzle2d_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            Self::from_fixture_json(json, Some("puzzle2d-default"))
        }

        pub fn from_puzzle3d_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            let raw: Value = serde_json::from_str(json)?;
            let mut fixture = raw.clone();
            if fixture.get("nodes").and_then(|v| v.as_array()).is_none() {
                if let Some(objects) = raw.get("objects").and_then(|v| v.as_array()) {
                    let nodes: Vec<Value> = objects
                        .iter()
                        .filter_map(|row| {
                            let obj = row.as_object()?;
                            let id = obj.get("id").and_then(|v| v.as_str())?;
                            let kind = obj.get("objectKind").or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("Object");
                            let name = obj.get("name").or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id);
                            Some(serde_json::json!({
                                "id": id,
                                "nodeKind": kind,
                                "text": name,
                            }))
                        })
                        .collect();
                    fixture["nodes"] = Value::Array(nodes);
                }
            }
            Self::from_fixture_json(&serde_json::to_string(&fixture)?, Some("puzzle3d-default"))
        }

        pub fn from_puzzle5d_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            Self::from_fixture_json(json, Some("puzzle5d-default"))
        }
    }

    impl QueryableGraph for BoardQueryableGraph {
        fn manifest(&self) -> Option<&GraphManifest> {
            self.manifest.as_ref()
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.keys().cloned().collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(kind, _, _)| kind.clone())
        }

        fn node_name(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(_, name, _)| name.clone())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let (_, name, properties) = self.nodes.get(id)?;
            match key {
                "id" => Some(PropertyValue::String(id.to_string())),
                "name" | "label" | "text" => Some(PropertyValue::String(name.clone())),
                "kind" => self.node_kind(id).map(PropertyValue::String),
                "__all" => Some(PropertyValue::Object(properties.clone())),
                _ => properties.get(key).cloned(),
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            self.edges.clone()
        }

        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
            let mut fixture = self.raw_fixture.clone();
            if let Some(nodes) = fixture.get_mut("nodes").and_then(|v| v.as_array_mut()) {
                nodes.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| node_ids.contains(id)));
            }
            if let Some(edges) = fixture.get_mut("edges").and_then(|v| v.as_array_mut()) {
                edges.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| edge_ids.contains(id)));
            }
            serde_json::to_string(&fixture).ok()
        }
    }
    // #endregion 🔖BoardQueryableGraph
    // #endregion queryable
}

pub mod wire {
    // #region wire
    //! 🔌 Wire-literal compiled DAG text notation.

    use crate::GraphDslError;
    use mathematical_graph_manifest::{PropertyBag, PropertyValue};

    // #region 🔖WireTypes
    /// 🧩 Neutral node row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireNode {
        pub id: String,
        pub kind: String,
        pub port: Option<String>,
        pub properties: PropertyBag,
    }

    /// 🪢 Neutral edge row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireEdge {
        pub from: String,
        pub from_port: String,
        pub to: String,
        pub to_port: String,
        pub directed: bool,
        pub properties: PropertyBag,
    }
    // #endregion 🔖WireTypes

    // #region 🔖WireLiteral
    fn format_properties(properties: &PropertyBag) -> String {
        if properties.is_empty() {
            return String::new();
        }
        let mut parts = Vec::new();
        for (key, value) in properties.iter() {
            parts.push(format!("{key}: {}", property_value_literal(value)));
        }
        format!("{{{}}}", parts.join(", "))
    }

    fn property_value_literal(value: &PropertyValue) -> String {
        match value {
            PropertyValue::String(s) => format!("'{s}'"),
            PropertyValue::Number(n) => n.to_string(),
            PropertyValue::Bool(b) => b.to_string(),
            PropertyValue::Null => "null".into(),
            PropertyValue::Object(map) => {
                let inner = map.iter().map(|(k, v)| format!("{k}: {}", property_value_literal(v))).collect::<Vec<_>>().join(", ");
                format!("{{{inner}}}")
            }
            PropertyValue::Array(items) => {
                let inner = items.iter().map(property_value_literal).collect::<Vec<_>>().join(", ");
                format!("[{inner}]")
            }
        }
    }

    fn format_node_ref(id: &str, kind: &str, port: Option<&str>) -> String {
        match port {
            Some(port) => format!("{id}:{kind}@{port}"),
            None => format!("{id}:{kind}"),
        }
    }

    /// 📝 Render wire-literal text from neutral node/edge rows.
    pub fn wire_literal_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> String {
        let mut lines = Vec::new();
        for node in nodes {
            let props = format_properties(&node.properties);
            if props.is_empty() {
                lines.push(format_node_ref(&node.id, &node.kind, node.port.as_deref()));
            } else {
                lines.push(format!("{}{}", format_node_ref(&node.id, &node.kind, node.port.as_deref()), props));
            }
        }
        for edge in edges {
            let from_kind = nodes.iter().find(|n| n.id == edge.from).map_or("node", |n| n.kind.as_str());
            let to_kind = nodes.iter().find(|n| n.id == edge.to).map_or("node", |n| n.kind.as_str());
            let connector = if edge.directed { "->" } else { "-" };
            let props = format_properties(&edge.properties);
            lines.push(format!("{}:{}@{}{}{}:{}@{}{}", edge.from, from_kind, edge.from_port, connector, edge.to, to_kind, edge.to_port, props));
        }
        lines.join("\n")
    }

    #[derive(Clone, Debug, PartialEq)]
    enum WireTok {
        Ident(String),
        Colon,
        At,
        Arrow,
        Dash,
        LBrace,
        RBrace,
        Comma,
        StringLit(String),
        Number(f64),
        Eof,
    }

    fn lex_wire(input: &str) -> Result<Vec<WireTok>, GraphDslError> {
        let mut out = Vec::new();
        let bytes = input.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i];
            if c.is_ascii_whitespace() {
                i += 1;
                continue;
            }
            match c {
                b':' => {
                    out.push(WireTok::Colon);
                    i += 1;
                }
                b'@' => {
                    out.push(WireTok::At);
                    i += 1;
                }
                b'-' if i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                    out.push(WireTok::Arrow);
                    i += 2;
                }
                b'-' => {
                    out.push(WireTok::Dash);
                    i += 1;
                }
                b'{' => {
                    out.push(WireTok::LBrace);
                    i += 1;
                }
                b'}' => {
                    out.push(WireTok::RBrace);
                    i += 1;
                }
                b',' => {
                    out.push(WireTok::Comma);
                    i += 1;
                }
                b'\'' | b'"' => {
                    let quote = c;
                    i += 1;
                    let start = i;
                    while i < bytes.len() && bytes[i] != quote {
                        i += 1;
                    }
                    if i >= bytes.len() {
                        return Err(GraphDslError::UnterminatedString);
                    }
                    let s = String::from_utf8_lossy(&bytes[start..i]).into_owned();
                    i += 1;
                    out.push(WireTok::StringLit(s));
                }
                b'0'..=b'9' => {
                    let start = i;
                    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                        i += 1;
                    }
                    let n: f64 = std::str::from_utf8(&bytes[start..i])?.parse::<f64>()?;
                    out.push(WireTok::Number(n));
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let start = i;
                    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.') {
                        i += 1;
                    }
                    // 🔒 ascii-only alphanumerics were matched byte-by-byte above, so utf-8 decoding is infallible here
                    out.push(WireTok::Ident(std::str::from_utf8(&bytes[start..i]).expect("scanned bytes are ascii alphanumerics/underscore/dot").to_string()));
                }
                _ => return Err(GraphDslError::UnexpectedChar(c as char)),
            }
        }
        out.push(WireTok::Eof);
        Ok(out)
    }

    struct WireParser {
        tokens: Vec<WireTok>,
        pos: usize,
    }

    impl WireParser {
        fn new(tokens: Vec<WireTok>) -> Self {
            Self { tokens, pos: 0 }
        }

        fn peek(&self) -> &WireTok {
            self.tokens.get(self.pos).unwrap_or(&WireTok::Eof)
        }

        fn bump(&mut self) -> WireTok {
            let t = self.peek().clone();
            if !matches!(t, WireTok::Eof) {
                self.pos += 1;
            }
            t
        }

        fn expect_ident(&mut self) -> Result<String, GraphDslError> {
            match self.bump() {
                WireTok::Ident(s) => Ok(s),
                other => Err(GraphDslError::UnexpectedToken { expected: "ident".into(), found: format!("{other:?}") }),
            }
        }

        fn parse_properties(&mut self) -> Result<PropertyBag, GraphDslError> {
            let mut bag = PropertyBag::new();
            if !matches!(self.peek(), WireTok::LBrace) {
                return Ok(bag);
            }
            self.bump();
            while !matches!(self.peek(), WireTok::RBrace | WireTok::Eof) {
                let key = self.expect_ident()?;
                let tok = self.bump();
                if !matches!(tok, WireTok::Colon) {
                    return Err(GraphDslError::UnexpectedToken { expected: ":".into(), found: format!("{tok:?}") });
                }
                let value = self.parse_value()?;
                bag.insert(key, value);
                if matches!(self.peek(), WireTok::Comma) {
                    self.bump();
                }
            }
            let tok = self.bump();
            if !matches!(tok, WireTok::RBrace) {
                return Err(GraphDslError::UnexpectedToken { expected: "}".into(), found: format!("{tok:?}") });
            }
            Ok(bag)
        }

        fn parse_value(&mut self) -> Result<PropertyValue, GraphDslError> {
            match self.bump() {
                WireTok::StringLit(s) => Ok(PropertyValue::String(s)),
                WireTok::Number(n) => Ok(PropertyValue::Number(n)),
                WireTok::Ident(s) if s == "true" => Ok(PropertyValue::Bool(true)),
                WireTok::Ident(s) if s == "false" => Ok(PropertyValue::Bool(false)),
                WireTok::Ident(s) if s == "null" => Ok(PropertyValue::Null),
                other => Err(GraphDslError::UnexpectedToken { expected: "value".into(), found: format!("{other:?}") }),
            }
        }

        fn expect_port(&mut self) -> Result<String, GraphDslError> {
            match self.bump() {
                WireTok::Ident(s) => Ok(s),
                WireTok::Number(n) => {
                    let mut port = if (n - n.round()).abs() < 1e-9 { format!("{}", n.round() as i64) } else { n.to_string() };
                    if let WireTok::Ident(suffix) = self.peek() {
                        port.push_str(suffix);
                        self.bump();
                    }
                    Ok(port)
                }
                other => Err(GraphDslError::UnexpectedToken { expected: "port".into(), found: format!("{other:?}") }),
            }
        }

        fn parse_node_ref(&mut self) -> Result<(String, String, Option<String>), GraphDslError> {
            let id = self.expect_ident()?;
            let tok = self.bump();
            if !matches!(tok, WireTok::Colon) {
                return Err(GraphDslError::UnexpectedToken { expected: ":".into(), found: format!("{tok:?}") });
            }
            let kind = self.expect_ident()?;
            let port = if matches!(self.peek(), WireTok::At) {
                self.bump();
                Some(self.expect_port()?)
            } else {
                None
            };
            Ok((id, kind, port))
        }

        fn parse_statement(&mut self) -> Result<(Option<WireNode>, Option<WireEdge>), GraphDslError> {
            let (id, kind, port) = self.parse_node_ref()?;
            if let Some(from_port) = port {
                let directed = if matches!(self.peek(), WireTok::Arrow) {
                    self.bump();
                    true
                } else if matches!(self.peek(), WireTok::Dash) {
                    self.bump();
                    false
                } else {
                    return Ok((Some(WireNode { id, kind, port: Some(from_port), properties: self.parse_properties()? }), None));
                };
                let (to, _to_kind, to_port) = self.parse_node_ref()?;
                let to_port = to_port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                let properties = self.parse_properties()?;
                Ok((None, Some(WireEdge { from: id, from_port, to, to_port, directed, properties })))
            } else {
                let properties = self.parse_properties()?;
                Ok((Some(WireNode { id, kind, port: None, properties }), None))
            }
        }

        fn parse_document(&mut self) -> Result<(Vec<WireNode>, Vec<WireEdge>), GraphDslError> {
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            while !matches!(self.peek(), WireTok::Eof) {
                let (node, edge) = self.parse_statement()?;
                if let Some(node) = node {
                    nodes.push(node);
                }
                if let Some(edge) = edge {
                    edges.push(edge);
                }
            }
            Ok((nodes, edges))
        }
    }

    /// 🔍 Parse wire-literal text into neutral node/edge rows.
    pub fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), GraphDslError> {
        let tokens = lex_wire(text)?;
        WireParser::new(tokens).parse_document()
    }
    // #endregion 🔖WireLiteral

    // #region 🔖Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn wire_literal_roundtrip_simple() {
            let nodes = vec![WireNode { id: "p".into(), kind: "Puzzle3d".into(), port: None, properties: PropertyBag::new() }];
            let edges = vec![WireEdge { from: "p".into(), from_port: "3d".into(), to: "s".into(), to_port: "3d".into(), directed: true, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&nodes, &edges);
            assert!(text.contains("p:Puzzle3d"));
            assert!(text.contains("p:Puzzle3d@3d->s:node@3d"));
            let parsed = dag_from_wire_literal(&text).unwrap();
            assert_eq!(parsed.1.len(), 1);
        }

        #[test]
        fn wire_literal_undirected() {
            let edges = vec![WireEdge { from: "a".into(), from_port: "out".into(), to: "b".into(), to_port: "in".into(), directed: false, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&[], &edges);
            assert!(text.contains('@'));
            assert!(text.contains('-'));
        }

        #[test]
        fn wire_literal_with_properties() {
            let mut props = PropertyBag::new();
            props.insert("value".into(), PropertyValue::Number(3.0));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            assert!(text.contains("{value: 3"));
        }
    }
    // #endregion 🔖Tests
    // #endregion wire
}

pub use queryable::{manifest_edge_kinds, manifest_node_kinds, manifest_port_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph};
pub use wire::{dag_from_wire_literal, wire_literal_from_dag, WireEdge, WireNode};

use mathematical_graph_manifest::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// #region jack_impl

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
    pub port: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    #[serde(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_fixture_json: Option<String>,
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_fixture_json: None }
    }

    pub fn graph(columns: Vec<String>, graph_fixture_json: String) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_fixture_json: Some(graph_fixture_json) }
    }
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
    At,
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
        Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::And | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Dash | Token::Arrow | Token::At => TokenClass::Operator,
        Token::LParen | Token::RParen | Token::LBracket | Token::RBracket | Token::Colon | Token::Comma | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, GraphDslError> {
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
            b'@' => {
                push_spanned(&mut tokens, Token::At, start, start + 1);
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
            b'!' if i + 1 < bytes.len() && bytes[i + 1] == b'=' => {
                push_spanned(&mut tokens, Token::Ne, start, start + 2);
                i += 2;
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
                    return Err(GraphDslError::UnterminatedString);
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
                        Err(e) => return Err(e.into()),
                    },
                    Err(_e) if forgiving => {
                        push_spanned(&mut tokens, Token::Ident(String::new()), num_start, i);
                        continue;
                    }
                    Err(e) => return Err(e.into()),
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
                // 🔒 ascii-only alphanumerics/underscore were matched byte-by-byte above, so utf-8 decoding is infallible here
                let word = std::str::from_utf8(&bytes[start..i]).expect("scanned bytes are ascii alphanumerics/underscore").to_ascii_uppercase();
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
                    _ => Token::Ident(std::str::from_utf8(&bytes[start..i]).expect("scanned bytes are ascii alphanumerics/underscore").to_string()),
                };
                push_spanned(&mut tokens, tok, start, i);
            }
            _ if forgiving => {
                push_spanned(&mut tokens, Token::Ident(String::from(c as char)), start, start + 1);
                i += 1;
            }
            _ => return Err(GraphDslError::UnexpectedChar(c as char)),
        }
    }
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

fn lex(input: &str) -> Result<Vec<Token>, GraphDslError> {
    lex_spanned(input, false).map(|spanned| spanned.into_iter().map(|row| row.token).collect())
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
            TokenSpan { class, start: row.start, end: row.end }
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

fn tokens_before_cursor(tokens: &[SpannedToken], cursor: usize) -> &[SpannedToken] {
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

fn graph_node_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_node_kinds(graph)
}

fn graph_edge_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_edge_kinds(graph)
}

fn graph_property_names(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_property_names(graph)
}

fn graph_port_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_port_kinds(graph)
}

fn after_at_port_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(at) = before.rfind('@') else {
        return false;
    };
    let after = &before[at + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '-' | '>' | '@'))
}

fn filter_completions(candidates: impl IntoIterator<Item = (String, String, Option<String>)>, prefix: &str) -> Vec<Completion> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut out = Vec::new();
    for (label, kind, detail) in candidates {
        if prefix.is_empty() || label.to_ascii_lowercase().starts_with(&prefix_lower) {
            out.push(Completion { insert: label.clone(), label, kind, detail });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// 🔎 Context-aware jack completions for the editor.
pub fn complete(graph: &dyn QueryableGraph, source: &str, cursor: usize) -> Vec<Completion> {
    let cursor = cursor.min(source.len());
    let prefix = completion_prefix(source, cursor);
    let tokens = lex_spanned(source, true).unwrap_or_default();
    let before = tokens_before_cursor(&tokens, cursor);

    if let Some(in_bracket) = after_colon_kind_context(source, cursor) {
        let kinds = if in_bracket { graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>() } else { graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>() };
        return filter_completions(kinds, &prefix);
    }

    if after_dot_property_context(source, cursor) {
        let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
        return filter_completions(props, &prefix);
    }

    if after_at_port_context(source, cursor) {
        let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
        return filter_completions(ports, &prefix);
    }

    if let Some(last) = before.last() {
        if matches!(last.token, Token::At) {
            let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
            return filter_completions(ports, &prefix);
        }
        if matches!(last.token, Token::Colon) {
            let kinds = if open_bracket_kind(before) == Some('[') {
                graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>()
            } else {
                graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>()
            };
            return filter_completions(kinds, &prefix);
        }
        if matches!(last.token, Token::Dot) {
            let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
            return filter_completions(props, &prefix);
        }
    }

    if in_where_clause(before) {
        let logic = filter_completions(LOGIC_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix);
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

    filter_completions(CLAUSE_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix)
}
// #endregion 🔖Language

// #region 🔖LanguageService
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hover {
    pub start: usize,
    pub end: usize,
    pub contents: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticToken {
    pub start: usize,
    pub end: usize,
    pub class: String,
}

fn collect_pattern_vars(pattern: &Pattern, out: &mut BTreeSet<String>) {
    for node in &pattern.nodes {
        out.insert(node.var.clone());
    }
    if let Some(edge) = &pattern.edge {
        if let Some(var) = &edge.var {
            out.insert(var.clone());
        }
        out.insert(edge.right.var.clone());
    }
}

fn collect_clause_bound_vars(clauses: &[Clause]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    for clause in clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    collect_pattern_vars(pattern, &mut vars);
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => collect_pattern_vars(pattern, &mut vars),
            _ => {}
        }
    }
    vars
}

fn collect_referenced_vars(clauses: &[Clause]) -> Vec<(String, usize, usize)> {
    let mut refs = Vec::new();
    for clause in clauses {
        match clause {
            Clause::Return(items) => {
                for item in items {
                    match item {
                        ReturnItem::Var(v) => refs.push((v.clone(), 0, v.len())),
                        ReturnItem::Property { var, .. } => refs.push((var.clone(), 0, var.len())),
                    }
                }
            }
            Clause::Delete(vars) => {
                for var in vars {
                    refs.push((var.clone(), 0, var.len()));
                }
            }
            Clause::Set(assignments) => {
                for assignment in assignments {
                    refs.push((assignment.var.clone(), 0, assignment.var.len()));
                }
            }
            Clause::Where(expr) => collect_expr_vars(expr, &mut refs),
            _ => {}
        }
    }
    refs
}

fn collect_expr_vars(expr: &Expr, refs: &mut Vec<(String, usize, usize)>) {
    match expr {
        Expr::Eq { var, .. } | Expr::Ne { var, .. } => refs.push((var.clone(), 0, var.len())),
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_expr_vars(a, refs);
            collect_expr_vars(b, refs);
        }
    }
}

fn semantic_lints(graph: &dyn QueryableGraph, query: &Query, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let node_kinds = graph_node_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let edge_kinds = graph_edge_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let bound = collect_clause_bound_vars(&query.clauses);
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    for node in &pattern.nodes {
                        if !node_kinds.contains(&node.kind) {
                            if let Some((start, end)) = find_kind_span(source, &node.kind) {
                                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                            }
                        }
                    }
                    if let Some(edge) = &pattern.edge {
                        if let Some(kind) = &edge.kind {
                            if !edge_kinds.contains(kind) {
                                if let Some((start, end)) = find_kind_span(source, kind) {
                                    out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown edge kind '{}'", kind), code: Some("jack/unknown-edge-kind".into()) });
                                }
                            }
                        }
                    }
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => {
                for node in &pattern.nodes {
                    if !node_kinds.contains(&node.kind) {
                        if let Some((start, end)) = find_kind_span(source, &node.kind) {
                            out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for (var, _, _) in collect_referenced_vars(&query.clauses) {
        if !bound.contains(&var) {
            if let Some((start, end)) = find_ident_span(source, &var) {
                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("variable '{var}' is not bound by MATCH"), code: Some("jack/unbound-variable".into()) });
            }
        }
    }
    out
}

fn find_kind_span(source: &str, kind: &str) -> Option<(usize, usize)> {
    let needle = format!(":{kind}");
    let start = source.find(&needle)?;
    Some((start + 1, start + needle.len()))
}

fn find_ident_span(source: &str, ident: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = source[from..].find(ident) {
        let start = from + rel;
        let end = start + ident.len();
        let before = source.as_bytes().get(start.wrapping_sub(1));
        let after = source.as_bytes().get(end);
        let boundary_before = before.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        let boundary_after = after.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        if boundary_before && boundary_after {
            return Some((start, end));
        }
        from = end;
    }
    None
}

/// 🩺 Lint jack source with syntax and semantic diagnostics.
pub fn lint(graph: &dyn QueryableGraph, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for span in tokenize(source) {
        if span.class == TokenClass::Error {
            out.push(Diagnostic { start: span.start, end: span.end, severity: DiagnosticSeverity::Error, message: "unterminated string literal".into(), code: Some("jack/unterminated-string".into()) });
        }
    }
    match parse(source) {
        Ok(query) => out.extend(semantic_lints(graph, &query, source)),
        Err(err) => {
            let end = source.len().max(1);
            out.push(Diagnostic { start: 0, end, severity: DiagnosticSeverity::Error, message: err.to_string(), code: Some("jack/parse-error".into()) });
        }
    }
    out
}

fn format_token(tok: &Token) -> String {
    match tok {
        Token::KwMatch => "MATCH".into(),
        Token::KwWhere => "WHERE".into(),
        Token::KwReturn => "RETURN".into(),
        Token::KwCreate => "CREATE".into(),
        Token::KwDelete => "DELETE".into(),
        Token::KwSet => "SET".into(),
        Token::KwMerge => "MERGE".into(),
        Token::And => "AND".into(),
        Token::Or => "OR".into(),
        Token::Ident(s) => s.clone(),
        Token::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        Token::StringLit(s) => format!("'{s}'"),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::Colon => ":".into(),
        Token::Comma => ",".into(),
        Token::Dot => ".".into(),
        Token::Eq => "=".into(),
        Token::Ne => "!=".into(),
        Token::Dash => "-".into(),
        Token::Arrow => "->".into(),
        Token::At => "@".into(),
        Token::Eof => String::new(),
    }
}

/// 🪞 Format jack source canonically (idempotent).
pub fn format(source: &str) -> Result<String, GraphDslError> {
    let tokens = lex_spanned(source, false)?;
    let mut out = String::new();
    let mut line_open = false;
    let mut i = 0;
    while i < tokens.len() {
        let row = &tokens[i];
        if matches!(row.token, Token::Eof) {
            break;
        }
        match &row.token {
            Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format_token(&row.token));
                out.push(' ');
                line_open = true;
            }
            Token::Comma => {
                out.push_str(", ");
            }
            Token::Arrow => {
                out.push_str("->");
            }
            Token::And | Token::Or => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            Token::Eq | Token::Ne => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            _ => {
                if line_open && !out.ends_with(' ') && !out.ends_with('\n') && !matches!(row.token, Token::RParen | Token::RBracket | Token::Comma | Token::Dot) {
                    let prev = tokens.get(i.saturating_sub(1)).map(|t| &t.token);
                    if !matches!(prev, Some(Token::LParen | Token::LBracket | Token::Colon | Token::Dot | Token::Dash)) {
                        out.push(' ');
                    }
                }
                out.push_str(&format_token(&row.token));
            }
        }
        i += 1;
    }
    Ok(out.trim().to_string())
}

fn hover_word_at(source: &str, cursor: usize) -> Option<(usize, usize, String)> {
    let cursor = cursor.min(source.len());
    if cursor > source.len() {
        return None;
    }
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    let mut end = cursor;
    while end < bytes.len() {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            end += 1;
        } else {
            break;
        }
    }
    if start == end {
        return None;
    }
    Some((start, end, source[start..end].to_string()))
}

/// 💬 Hover information at cursor.
pub fn hover(graph: &dyn QueryableGraph, source: &str, cursor: usize) -> Option<Hover> {
    let (start, end, word) = hover_word_at(source, cursor)?;
    let upper = word.to_ascii_uppercase();
    if CLAUSE_KEYWORDS.iter().any(|kw| *kw == upper) || LOGIC_KEYWORDS.iter().any(|kw| *kw == upper) {
        return Some(Hover { start, end, contents: format!("Jack keyword `{upper}`") });
    }
    if graph_node_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Node kind `{word}`") });
    }
    if graph_edge_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Edge kind `{word}`") });
    }
    if graph_property_names(graph).iter().any(|prop| prop == &word) {
        return Some(Hover { start, end, contents: format!("Property `{word}`") });
    }
    if collect_bound_vars(&lex_spanned(source, true).unwrap_or_default()).contains(&word) {
        return Some(Hover { start, end, contents: format!("Bound variable `{word}`") });
    }
    None
}

/// 🎨 Semantic token classes for LSP highlighting.
pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    tokenize(source)
        .into_iter()
        .map(|span| SemanticToken {
            start: span.start,
            end: span.end,
            class: match span.class {
                TokenClass::Keyword => "keyword",
                TokenClass::Ident => "ident",
                TokenClass::Number => "number",
                TokenClass::String => "string",
                TokenClass::Operator => "operator",
                TokenClass::Punctuation => "punctuation",
                TokenClass::Error => "error",
            }
            .into(),
        })
        .collect()
}
// #endregion 🔖LanguageService

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

    fn expect_ident(&mut self) -> Result<String, GraphDslError> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(GraphDslError::UnexpectedToken { expected: "ident".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_query(&mut self) -> Result<Query, GraphDslError> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    fn parse_clause(&mut self) -> Result<Clause, GraphDslError> {
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
            other => Err(GraphDslError::UnexpectedToken { expected: "clause start (MATCH/WHERE/RETURN/CREATE/DELETE/SET/MERGE)".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, GraphDslError> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        if matches!(self.peek(), Token::Dash) {
            self.bump();
            let (edge_var, edge_kind) = if matches!(self.peek(), Token::LBracket) {
                self.bump();
                let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
                let edge_kind = if matches!(self.peek(), Token::Colon) {
                    self.bump();
                    Some(self.expect_ident()?)
                } else {
                    None
                };
                self.expect(&Token::RBracket)?;
                (edge_var, edge_kind)
            } else {
                (None, None)
            };
            let directed = if matches!(self.peek(), Token::Arrow) {
                self.bump();
                true
            } else {
                false
            };
            self.expect(&Token::LParen)?;
            let right = self.parse_pattern_node()?;
            self.expect(&Token::RParen)?;
            Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed, right }) })
        } else {
            Ok(Pattern { nodes: vec![left], edge: None })
        }
    }

    fn parse_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        let port = if matches!(self.peek(), Token::At) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        Ok(PatternNode { var, kind, port })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, GraphDslError> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    fn parse_expr(&mut self) -> Result<Expr, GraphDslError> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<Expr, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(GraphDslError::UnexpectedToken { expected: "= or !=".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_value(&mut self) -> Result<PropertyValue, GraphDslError> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(GraphDslError::UnexpectedToken { expected: "value".into(), found: format!("{other:?}") }),
        }
    }

    fn expect(&mut self, want: &Token) -> Result<(), GraphDslError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(GraphDslError::UnexpectedToken { expected: format!("{want:?}"), found: format!("{:?}", self.peek()) })
        }
    }
}

/// 🔍 Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, GraphDslError> {
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

/// ▶️ Execute a read-only jack query against a queryable graph.
pub fn execute(graph: &dyn QueryableGraph, query: &Query) -> Result<QueryResult, GraphDslError> {
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => bindings = match_patterns(graph, patterns)?,
            Clause::Where(expr) => bindings.retain(|b| eval_expr(graph, b, expr)),
            Clause::Return(items) => return_items = Some(items.clone()),
            Clause::Create(_) | Clause::Delete(_) | Clause::Set(_) | Clause::Merge(_) => {
                return Err(GraphDslError::UnsupportedMutation);
            }
        }
    }
    if let Some(items) = return_items {
        return Ok(build_return(graph, &bindings, &items));
    }
    Ok(QueryResult::table(vec![], vec![]))
}

/// ▶️ Parse and execute jack in one step.
pub fn run_query(graph: &dyn QueryableGraph, source: &str) -> Result<QueryResult, GraphDslError> {
    execute(graph, &parse(source)?)
}

/// ▶️ Execute jack and return JSON result.
pub fn run_query_json(graph: &dyn QueryableGraph, source: &str) -> Result<String, GraphDslError> {
    Ok(serde_json::to_string(&run_query(graph, source)?)?)
}

fn match_patterns(graph: &dyn QueryableGraph, patterns: &[Pattern]) -> Result<Vec<Binding>, GraphDslError> {
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

fn match_pattern(graph: &dyn QueryableGraph, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, GraphDslError> {
    let left = pattern.nodes.first().ok_or(GraphDslError::EmptyPattern)?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for node_id in graph.node_ids() {
            if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id.as_str()) {
                continue;
            }
            for edge in graph.edges() {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let pairs = if edge_pat.directed {
                    vec![(edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref())]
                } else {
                    vec![
                        (edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref()),
                        (edge.target_node_id.as_str(), edge.source_node_id.as_str(), edge.target_port.as_deref(), edge.source_port.as_deref()),
                    ]
                };
                for (src_id, tgt_id, src_port, tgt_port) in pairs {
                    if src_id != node_id {
                        continue;
                    }
                    if left.port.as_ref().is_some_and(|want| src_port != Some(want.as_str())) {
                        continue;
                    }
                    if graph.node_kind(tgt_id).as_deref() != Some(edge_pat.right.kind.as_str()) {
                        continue;
                    }
                    if edge_pat.right.port.as_ref().is_some_and(|want| tgt_port != Some(want.as_str())) {
                        continue;
                    }
                    let mut b = base.clone();
                    b.nodes.insert(left.var.clone(), node_id.clone());
                    if let Some(ev) = &edge_pat.var {
                        b.edges.insert(ev.clone(), edge.id.clone());
                    }
                    if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                        continue;
                    }
                    b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                    out.push(b);
                }
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for node_id in graph.node_ids() {
        if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id.as_str()) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id);
        out.push(b);
    }
    Ok(out)
}

fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

fn eval_expr(graph: &dyn QueryableGraph, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

fn binding_value(graph: &dyn QueryableGraph, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    let node_id = binding.nodes.get(var)?;
    graph.node_property(node_id, prop)
}

fn binding_has_entity(binding: &Binding, var: &str) -> bool {
    binding.nodes.contains_key(var) || binding.edges.contains_key(var)
}

fn return_items_want_graph(items: &[ReturnItem], bindings: &[Binding]) -> bool {
    items.iter().any(|item| {
        let ReturnItem::Var(v) = item else { return false };
        bindings.iter().any(|b| binding_has_entity(b, v))
    })
}

fn collect_graph_entities(bindings: &[Binding], items: &[ReturnItem]) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut node_ids = BTreeSet::new();
    let mut edge_ids = BTreeSet::new();
    for binding in bindings {
        for item in items {
            if let ReturnItem::Var(v) = item {
                if let Some(id) = binding.nodes.get(v) {
                    node_ids.insert(id.clone());
                }
                if let Some(id) = binding.edges.get(v) {
                    edge_ids.insert(id.clone());
                }
            }
        }
    }
    (node_ids, edge_ids)
}

fn build_return(graph: &dyn QueryableGraph, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    if return_items_want_graph(items, bindings) {
        let (node_ids, edge_ids) = collect_graph_entities(bindings, items);
        if let Some(json) = graph.subgraph_fixture_json(&node_ids, &edge_ids) {
            return QueryResult::graph(columns, json);
        }
    }
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            let val = match item {
                ReturnItem::Var(v) => binding.nodes.get(v).and_then(|id| graph.node_name(id)).map_or(PropertyValue::Null, PropertyValue::String),
                ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop).unwrap_or(PropertyValue::Null),
            };
            row.push(val);
        }
        rows.push(row);
    }
    QueryResult::table(columns, rows)
}
// #endregion 🔖Executor

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_match_return() {
        let q = parse("MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    }

    #[test]
    fn run_dag_fixture_query() {
        let fixture = include_str!("../../../../infinite/board/port/directed/dag/example/demo.dag.json");
        let graph = BoardQueryableGraph::from_dag_fixture_json(fixture).unwrap();
        let result = run_query(&graph, "MATCH (n:computation) RETURN n.name").unwrap();
        assert!(!result.rows.is_empty());
    }

    #[test]
    fn parse_match_with_port() {
        let q = parse("MATCH (a:computation@out) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        assert_eq!(patterns[0].nodes[0].port.as_deref(), Some("out"));
    }

    #[test]
    fn parse_undirected_edge() {
        let q = parse("MATCH (a:computation)-(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    }

    #[test]
    fn run_port_filtered_query() {
        let fixture = include_str!("../../../../infinite/board/port/directed/dag/example/demo.dag.json");
        let graph = BoardQueryableGraph::from_dag_fixture_json(fixture).unwrap();
        let result = run_query(&graph, "MATCH (n:computation@out)-[:wire]->(m:slider) RETURN n.name, m.name");
        assert!(result.is_ok());
    }
}
// #endregion 🔖Tests
// #endregion jack_impl
