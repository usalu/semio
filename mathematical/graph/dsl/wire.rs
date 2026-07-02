//! 🔌 Wire-literal compiled DAG text notation.

use mathematical_graph_manifest::{PropertyBag, PropertyValue};
use std::collections::BTreeMap;

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
            let inner = map
                .iter()
                .map(|(k, v)| format!("{k}: {}", property_value_literal(v)))
                .collect::<Vec<_>>()
                .join(", ");
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
        let from_kind = nodes.iter().find(|n| n.id == edge.from).map(|n| n.kind.as_str()).unwrap_or("node");
        let to_kind = nodes.iter().find(|n| n.id == edge.to).map(|n| n.kind.as_str()).unwrap_or("node");
        let connector = if edge.directed { "->" } else { "-" };
        let props = format_properties(&edge.properties);
        lines.push(format!(
            "{}:{}@{}{}{}:{}@{}{}",
            edge.from, from_kind, edge.from_port, connector, edge.to, to_kind, edge.to_port, props
        ));
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

fn lex_wire(input: &str) -> Result<Vec<WireTok>, String> {
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
                    return Err("unterminated string".into());
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
                let n: f64 = std::str::from_utf8(&bytes[start..i])
                    .map_err(|e| e.to_string())?
                    .parse::<f64>()
                    .map_err(|e| e.to_string())?;
                out.push(WireTok::Number(n));
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.') {
                    i += 1;
                }
                out.push(WireTok::Ident(std::str::from_utf8(&bytes[start..i]).unwrap().to_string()));
            }
            _ => return Err(format!("unexpected char {}", c as char)),
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

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.bump() {
            WireTok::Ident(s) => Ok(s),
            other => Err(format!("expected ident, got {other:?}")),
        }
    }

    fn parse_properties(&mut self) -> Result<PropertyBag, String> {
        let mut bag = PropertyBag::new();
        if !matches!(self.peek(), WireTok::LBrace) {
            return Ok(bag);
        }
        self.bump();
        while !matches!(self.peek(), WireTok::RBrace | WireTok::Eof) {
            let key = self.expect_ident()?;
            if !matches!(self.bump(), WireTok::Colon) {
                return Err("expected : in property".into());
            }
            let value = self.parse_value()?;
            bag.insert(key, value);
            if matches!(self.peek(), WireTok::Comma) {
                self.bump();
            }
        }
        if !matches!(self.bump(), WireTok::RBrace) {
            return Err("expected }".into());
        }
        Ok(bag)
    }

    fn parse_value(&mut self) -> Result<PropertyValue, String> {
        match self.bump() {
            WireTok::StringLit(s) => Ok(PropertyValue::String(s)),
            WireTok::Number(n) => Ok(PropertyValue::Number(n)),
            WireTok::Ident(s) if s == "true" => Ok(PropertyValue::Bool(true)),
            WireTok::Ident(s) if s == "false" => Ok(PropertyValue::Bool(false)),
            WireTok::Ident(s) if s == "null" => Ok(PropertyValue::Null),
            other => Err(format!("expected value, got {other:?}")),
        }
    }

    fn expect_port(&mut self) -> Result<String, String> {
        match self.bump() {
            WireTok::Ident(s) => Ok(s),
            WireTok::Number(n) => {
                let mut port = if (n - n.round()).abs() < 1e-9 {
                    format!("{}", n.round() as i64)
                } else {
                    n.to_string()
                };
                if let WireTok::Ident(suffix) = self.peek() {
                    port.push_str(suffix);
                    self.bump();
                }
                Ok(port)
            }
            other => Err(format!("expected port, got {other:?}")),
        }
    }

    fn parse_node_ref(&mut self) -> Result<(String, String, Option<String>), String> {
        let id = self.expect_ident()?;
        if !matches!(self.bump(), WireTok::Colon) {
            return Err("expected : after node id".into());
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

    fn parse_statement(&mut self) -> Result<(Option<WireNode>, Option<WireEdge>), String> {
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
            let to_port = to_port.ok_or_else(|| "edge target requires @port".to_string())?;
            let properties = self.parse_properties()?;
            Ok((
                None,
                Some(WireEdge {
                    from: id,
                    from_port,
                    to,
                    to_port,
                    directed,
                    properties,
                }),
            ))
        } else {
            let properties = self.parse_properties()?;
            Ok((Some(WireNode { id, kind, port: None, properties }), None))
        }
    }

    fn parse_document(&mut self) -> Result<(Vec<WireNode>, Vec<WireEdge>), String> {
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
pub fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), String> {
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
        let nodes = vec![WireNode {
            id: "p".into(),
            kind: "Puzzle3d".into(),
            port: None,
            properties: PropertyBag::new(),
        }];
        let edges = vec![WireEdge {
            from: "p".into(),
            from_port: "3d".into(),
            to: "s".into(),
            to_port: "3d".into(),
            directed: true,
            properties: PropertyBag::new(),
        }];
        let text = wire_literal_from_dag(&nodes, &edges);
        assert!(text.contains("p:Puzzle3d"));
        assert!(text.contains("p:Puzzle3d@3d->s:node@3d"));
        let parsed = dag_from_wire_literal(&text).unwrap();
        assert_eq!(parsed.1.len(), 1);
    }

    #[test]
    fn wire_literal_undirected() {
        let edges = vec![WireEdge {
            from: "a".into(),
            from_port: "out".into(),
            to: "b".into(),
            to_port: "in".into(),
            directed: false,
            properties: PropertyBag::new(),
        }];
        let text = wire_literal_from_dag(&[], &edges);
        assert!(text.contains('@'));
        assert!(text.contains('-'));
    }

    #[test]
    fn wire_literal_with_properties() {
        let mut props = PropertyBag::new();
        props.insert("value".into(), PropertyValue::Number(3.0));
        let nodes = vec![WireNode {
            id: "n".into(),
            kind: "slider".into(),
            port: None,
            properties: props,
        }];
        let text = wire_literal_from_dag(&nodes, &[]);
        assert!(text.contains("{value: 3"));
    }
}
// #endregion 🔖Tests
