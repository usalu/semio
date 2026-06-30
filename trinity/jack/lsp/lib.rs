//! 📡 Jack language server — JSON-RPC LSP subset over trinity graphs.

use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use trinity_jack::{
    complete, format, hover, lint, semantic_tokens, Completion, Diagnostic, DiagnosticSeverity, Hover,
};
use trinity_ram::{CameraV1, Edge, Graph, GraphFixtureV1, Manifest, Node, Port, PortDirection, PropertyBag};

// #region 🔖LspTypes
#[derive(Clone, Debug, Deserialize)]
pub struct JsonRpcMessage {
    #[serde(default)]
    id: Option<Value>,
    method: Option<String>,
    params: Option<Value>,
    #[allow(dead_code)]
    result: Option<Value>,
    #[allow(dead_code)]
    error: Option<Value>,
}

#[derive(Clone, Debug, Deserialize)]
struct TextDocumentItem {
    uri: String,
    #[serde(rename = "languageId")]
    #[allow(dead_code)]
    language_id: String,
    version: i64,
    text: String,
}

#[derive(Clone, Debug, Deserialize)]
struct DidOpenParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentItem,
}

#[derive(Clone, Debug, Deserialize)]
struct DidChangeParams {
    #[serde(rename = "textDocument")]
    text_document: VersionedTextDocument,
    #[serde(rename = "contentChanges")]
    content_changes: Vec<ContentChange>,
}

#[derive(Clone, Debug, Deserialize)]
struct VersionedTextDocument {
    uri: String,
    version: i64,
}

#[derive(Clone, Debug, Deserialize)]
struct ContentChange {
    text: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TextDocumentPositionParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentRef,
    position: Position,
}

#[derive(Clone, Debug, Deserialize)]
struct TextDocumentRef {
    uri: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Position {
    line: u32,
    character: u32,
}

#[derive(Clone, Debug, Deserialize)]
struct FormattingParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentRef,
}
// #endregion 🔖LspTypes

// #region 🔖JackLanguageServer
pub struct JackLanguageServer {
    graph: Graph,
    documents: BTreeMap<String, DocumentState>,
}

#[derive(Clone, Debug)]
struct DocumentState {
    version: i64,
    text: String,
}

impl Default for JackLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}

impl JackLanguageServer {
    pub fn new() -> Self {
        let fixture = GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.into(),
            name: "jack-lsp".into(),
            manifest: Manifest::nakagin_default(),
            camera: CameraV1::default(),
            root_node_id: Some("root".into()),
            nodes: vec![Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port {
                    id: "out".into(),
                    kind: "Connector".into(),
                    direction: PortDirection::Out,
                    properties: PropertyBag::new(),
                }],
            }],
            edges: vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root:out".into(),
                target: "child:in".into(),
                properties: PropertyBag::new(),
            }],
        };
        let graph = Graph::from_fixture(fixture).expect("jack lsp default fixture");
        Self {
            graph,
            documents: BTreeMap::new(),
        }
    }

    pub fn load_fixture_json(&mut self, json: &str) -> Result<(), String> {
        let fixture: GraphFixtureV1 = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.graph = Graph::from_fixture(fixture)?;
        self.refresh_all();
        Ok(())
    }

    pub fn handle_message_json(&mut self, raw: &str) -> String {
        let message: JsonRpcMessage = match serde_json::from_str(raw) {
            Ok(row) => row,
            Err(err) => {
                return serde_json::to_string(&vec![error_response(None, -32700, err.to_string())]).unwrap_or_else(|_| "[]".into());
            }
        };
        let replies = self.handle_message(message);
        serde_json::to_string(&replies).unwrap_or_else(|_| "[]".into())
    }

    pub fn handle_message(&mut self, message: JsonRpcMessage) -> Vec<Value> {
        if message.method.is_none() {
            return Vec::new();
        }
        let method = message.method.unwrap_or_default();
        let id = message.id.clone();
        if method == "initialize" {
            return vec![ok_response(
                id,
                json!({
                    "capabilities": {
                        "textDocumentSync": 1,
                        "completionProvider": { "triggerCharacters": [".", ":"] },
                        "hoverProvider": true,
                        "documentFormattingProvider": true,
                        "semanticTokensProvider": { "legend": { "tokenTypes": ["keyword","ident","number","string","operator","punctuation","error"], "tokenModifiers": [] } }
                    },
                    "serverInfo": { "name": "jack-lsp", "version": "1" }
                }),
            )];
        }
        if method == "initialized" {
            return Vec::new();
        }
        if method == "textDocument/didOpen" {
            let params: DidOpenParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            self.documents.insert(
                params.text_document.uri.clone(),
                DocumentState {
                    version: params.text_document.version,
                    text: params.text_document.text,
                },
            );
            return self.publish_for_uri(&params.text_document.uri, id);
        }
        if method == "textDocument/didChange" {
            let params: DidChangeParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let text = params.content_changes.last().map(|c| c.text.clone()).unwrap_or_default();
            if let Some(doc) = self.documents.get_mut(&params.text_document.uri) {
                doc.version = params.text_document.version;
                doc.text = text;
            }
            return self.publish_for_uri(&params.text_document.uri, id);
        }
        if method == "textDocument/completion" {
            let params: TextDocumentPositionParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let Some(doc) = self.documents.get(&params.text_document.uri) else {
                return vec![ok_response(id, json!({ "isIncomplete": false, "items": [] }))];
            };
            let offset = position_to_offset(&doc.text, &params.position);
            let items = complete(&self.graph, &doc.text, offset)
                .into_iter()
                .map(completion_to_lsp)
                .collect::<Vec<_>>();
            return vec![ok_response(id, json!({ "isIncomplete": false, "items": items }))];
        }
        if method == "textDocument/hover" {
            let params: TextDocumentPositionParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let Some(doc) = self.documents.get(&params.text_document.uri) else {
                return vec![ok_response(id, Value::Null)];
            };
            let offset = position_to_offset(&doc.text, &params.position);
            let result = hover(&self.graph, &doc.text, offset).map(hover_to_lsp);
            return vec![ok_response(id, result.unwrap_or(Value::Null))];
        }
        if method == "textDocument/formatting" {
            let params: FormattingParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let Some(doc) = self.documents.get(&params.text_document.uri) else {
                return vec![ok_response(id, json!([]))];
            };
            let formatted = match format(&doc.text) {
                Ok(text) => vec![json!({
                    "range": {
                        "start": { "line": 0, "character": 0 },
                        "end": offset_to_position(&doc.text, doc.text.len())
                    },
                    "newText": text
                })],
                Err(err) => return vec![error_response(id, -32603, err)],
            };
            return vec![ok_response(id, Value::Array(formatted))];
        }
        if method == "textDocument/semanticTokens/full" {
            let params: TextDocumentRef = match message.params.and_then(|p| serde_json::from_value(p).ok()) {
                Some(row) => row,
                None => return vec![error_response(id, -32602, "missing params".into())],
            };
            let Some(doc) = self.documents.get(&params.uri) else {
                return vec![ok_response(id, json!({ "data": [] }))];
            };
            let tokens = semantic_tokens(&doc.text);
            return vec![ok_response(id, json!({ "tokens": tokens }))];
        }
        vec![error_response(id, -32601, format!("method not found: {method}"))]
    }

    fn refresh_all(&mut self) {
        let uris = self.documents.keys().cloned().collect::<Vec<_>>();
        for uri in uris {
            let _ = self.publish_for_uri(&uri, None);
        }
    }

    fn publish_for_uri(&self, uri: &str, request_id: Option<Value>) -> Vec<Value> {
        let Some(doc) = self.documents.get(uri) else {
            return if let Some(id) = request_id {
                vec![ok_response(Some(id), Value::Null)]
            } else {
                Vec::new()
            };
        };
        let diagnostics = lint(&self.graph, &doc.text).into_iter().map(|d| diagnostic_to_lsp(&doc.text, d)).collect::<Vec<_>>();
        let tokens = semantic_tokens(&doc.text);
        let mut out = Vec::new();
        if let Some(id) = request_id {
            out.push(ok_response(Some(id), Value::Null));
        }
        out.push(notification(
            "textDocument/publishDiagnostics",
            json!({ "uri": uri, "diagnostics": diagnostics }),
        ));
        out.push(notification(
            "writer/semanticTokens",
            json!({ "uri": uri, "tokens": tokens }),
        ));
        out
    }
}
// #endregion 🔖JackLanguageServer

// #region 🔖Wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct JackLspSession {
    server: JackLanguageServer,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl JackLspSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            server: JackLanguageServer::new(),
        }
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.server.load_fixture_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = handleMessageJson)]
    pub fn handle_message_json(&mut self, raw: &str) -> String {
        self.server.handle_message_json(raw)
    }
}
// #endregion 🔖Wasm

// #region 🔖Helpers
fn ok_response(id: Option<Value>, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error_response(id: Option<Value>, code: i64, message: String) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn notification(method: &str, params: Value) -> Value {
    json!({ "jsonrpc": "2.0", "method": method, "params": params })
}

fn offset_to_position(text: &str, offset: usize) -> Value {
    let pos = position_to_offset_struct(text, offset);
    json!({ "line": pos.line, "character": pos.character })
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    line: u32,
    character: u32,
}

fn position_to_offset(text: &str, position: &Position) -> usize {
    let mut offset = 0usize;
    for (i, line) in text.split('\n').enumerate() {
        if i as u32 == position.line {
            return offset + (position.character as usize).min(line.len());
        }
        offset += line.len() + 1;
    }
    text.len()
}

fn position_to_offset_struct(text: &str, offset: usize) -> Pos {
    let clamped = offset.min(text.len());
    let mut line = 0u32;
    let mut last = 0usize;
    for (i, ch) in text.char_indices() {
        if i >= clamped {
            break;
        }
        if ch == '\n' {
            line += 1;
            last = i + 1;
        }
    }
    Pos {
        line,
        character: (clamped.saturating_sub(last)) as u32,
    }
}

fn diagnostic_to_lsp(text: &str, diag: Diagnostic) -> Value {
    json!({
        "range": {
            "start": offset_to_position(text, diag.start),
            "end": offset_to_position(text, diag.end)
        },
        "severity": match diag.severity {
            DiagnosticSeverity::Error => 1,
            DiagnosticSeverity::Warning => 2,
            DiagnosticSeverity::Information => 3,
            DiagnosticSeverity::Hint => 4,
        },
        "code": diag.code,
        "message": diag.message
    })
}

fn completion_to_lsp(item: Completion) -> Value {
    json!({
        "label": item.label,
        "kind": 1,
        "detail": item.detail,
        "insertText": item.insert
    })
}

fn hover_to_lsp(item: Hover) -> Value {
    json!({
        "contents": item.contents,
        "range": {
            "start": { "line": 0, "character": item.start },
            "end": { "line": 0, "character": item.end }
        }
    })
}
// #endregion 🔖Helpers

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_returns_capabilities() {
        let mut server = JackLanguageServer::new();
        let raw = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(raw)).unwrap();
        assert_eq!(replies[0]["result"]["serverInfo"]["name"], "jack-lsp");
    }

    #[test]
    fn did_open_publishes_diagnostics() {
        let mut server = JackLanguageServer::new();
        let open = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "writer://jack",
                    "languageId": "jack",
                    "version": 1,
                    "text": "RETURN a.name"
                }
            }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&open.to_string())).unwrap();
        assert!(replies.iter().any(|row| row["method"] == "textDocument/publishDiagnostics"));
    }
}
// #endregion 🔖Tests
