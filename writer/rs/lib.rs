//! ✍️ Writer WASM package: re-exports framework editor and writer document VCS.

pub use framework_editor::*;

pub type WriterHost = EditorHost;

#[cfg(target_arch = "wasm32")]
pub type WriterSession = EditorSession;

mod document_vcs {
    // #region document_vcs
    // #region 🔖DocumentVcs
    #[cfg(target_arch = "wasm32")]
    use std::cell::RefCell;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::prelude::*;

    use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

    /// 📷 Editor viewport transform persisted in the document projection.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WriterCamera {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default = "default_zoom")]
        pub zoom: f64,
    }

    fn default_zoom() -> f64 {
        1.0
    }

    fn default_uri() -> String {
        "writer://empty".into()
    }

    fn default_camera() -> WriterCamera {
        WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
    }

    /// 📝 The full writer document projection: identity, language, source text and camera.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WriterProjection {
        pub schema: String,
        pub id: String,
        pub language_id: String,
        #[serde(default = "default_uri")]
        pub uri: String,
        #[serde(default)]
        pub text: String,
        #[serde(default = "default_camera")]
        pub camera: WriterCamera,
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "operation", rename_all = "camelCase")]
    pub enum WriterOperation {
        SetText { text: String },
        SetCamera { camera: WriterCamera },
        SetDocument { document: WriterProjection },
    }

    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WriterDiff {
        pub text: Option<String>,
        pub camera: Option<WriterCamera>,
        pub document: Option<WriterProjection>,
    }

    impl OperationDiff<WriterProjection> for WriterDiff {
        fn apply(&self, projection: &WriterProjection) -> WriterProjection {
            if let Some(document) = &self.document {
                return document.clone();
            }
            WriterProjection { text: self.text.clone().unwrap_or_else(|| projection.text.clone()), camera: self.camera.clone().unwrap_or_else(|| projection.camera.clone()), ..projection.clone() }
        }

        fn absorb(&mut self, other: Self) {
            if other.document.is_some() {
                *self = other;
                return;
            }
            if other.text.is_some() {
                self.text = other.text;
            }
            if other.camera.is_some() {
                self.camera = other.camera;
            }
        }
    }

    impl Operation<WriterProjection> for WriterOperation {
        type Diff = WriterDiff;

        fn diff(&self, _projection: &WriterProjection) -> WriterDiff {
            match self {
                WriterOperation::SetText { text } => WriterDiff { text: Some(text.clone()), ..Default::default() },
                WriterOperation::SetCamera { camera } => WriterDiff { camera: Some(camera.clone()), ..Default::default() },
                WriterOperation::SetDocument { document } => WriterDiff { document: Some(document.clone()), ..Default::default() },
            }
        }

        fn backwards(&self, projection: &WriterProjection) -> Vec<Self> {
            match self {
                WriterOperation::SetText { .. } => vec![WriterOperation::SetText { text: projection.text.clone() }],
                WriterOperation::SetCamera { .. } => vec![WriterOperation::SetCamera { camera: projection.camera.clone() }],
                WriterOperation::SetDocument { .. } => vec![WriterOperation::SetDocument { document: projection.clone() }],
            }
        }
    }

    // #region 🔖Dsl
    /// 📜 Hand-rolled lexer/printer shared by `WriterProjection`'s `.writer` DSL (`🔖Dsl`) and by
    /// `WriterOperation`'s one-line op text (`🔖OpText`) — both use the same `@marker key=value ...
    /// "trailing text"` header grammar as `vcs`'s own structural lines, hand-rolled locally since
    /// `vcs`'s escaping helpers are private to that crate.
    mod writer_dsl {
        use super::{WriterCamera, WriterOperation, WriterProjection};
        use std::collections::HashMap;
        use vcs::{TextError, TextSpan};

        //#region Lexer
        /// 🔐 Escapes `\`, `"` and newlines so arbitrary source text fits inside one quoted field.
        fn escape_text(value: &str) -> String {
            let mut out = String::with_capacity(value.len());
            for ch in value.chars() {
                match ch {
                    '\\' => out.push_str("\\\\"),
                    '"' => out.push_str("\\\""),
                    '\n' => out.push_str("\\n"),
                    _ => out.push(ch),
                }
            }
            out
        }

        fn unescape_text(value: &str) -> String {
            let mut out = String::with_capacity(value.len());
            let mut chars = value.chars();
            while let Some(ch) = chars.next() {
                if ch == '\\' {
                    match chars.next() {
                        Some('n') => out.push('\n'),
                        Some('"') => out.push('"'),
                        Some('\\') => out.push('\\'),
                        Some(other) => {
                            out.push('\\');
                            out.push(other);
                        }
                        None => out.push('\\'),
                    }
                } else {
                    out.push(ch);
                }
            }
            out
        }

        /// 🔎 Finds the char index of the unescaped opening `"` of a trailing quoted field, mirroring
        /// `vcs`'s private `find_unescaped_trailing_quote` (kept in lock-step, see that doc comment).
        fn find_unescaped_trailing_quote(chars: &[char]) -> Option<usize> {
            if chars.is_empty() || *chars.last().unwrap() != '"' {
                return None;
            }
            let last = chars.len() - 1;
            let mut i = last;
            while i > 0 {
                i -= 1;
                if chars[i] == '"' {
                    let mut backslashes = 0;
                    let mut j = i;
                    while j > 0 && chars[j - 1] == '\\' {
                        backslashes += 1;
                        j -= 1;
                    }
                    if backslashes % 2 == 0 {
                        return Some(i);
                    }
                }
            }
            None
        }

        /// 🧾 One parsed `@marker key=value ...` line plus its optional trailing quoted text field.
        struct KvLine {
            marker: String,
            fields: HashMap<String, String>,
            text: Option<String>,
        }

        fn parse_kv_line(line: &str) -> Result<KvLine, TextError> {
            let chars: Vec<char> = line.chars().collect();
            let (head, text) = match find_unescaped_trailing_quote(&chars) {
                Some(open) => {
                    let content: String = chars[open + 1..chars.len() - 1].iter().collect();
                    let head: String = chars[..open].iter().collect();
                    (head.trim_end().to_string(), Some(unescape_text(&content)))
                }
                None => (line.to_string(), None),
            };
            let mut tokens = head.split_whitespace();
            let marker = tokens
                .next()
                .ok_or_else(|| TextError::new("expected a marker or operation name", TextSpan::at(1, 1)))?
                .to_string();
            let mut fields = HashMap::new();
            for token in tokens {
                let (key, value) = token
                    .split_once('=')
                    .ok_or_else(|| TextError::new(format!("expected key=value token, got '{token}'"), TextSpan::at(1, 1)))?;
                fields.insert(key.to_string(), value.to_string());
            }
            Ok(KvLine { marker, fields, text })
        }

        fn field<'a>(fields: &'a HashMap<String, String>, key: &str) -> Result<&'a str, TextError> {
            fields
                .get(key)
                .map(|value| value.as_str())
                .ok_or_else(|| TextError::new(format!("missing field '{key}'"), TextSpan::at(1, 1)))
        }

        fn parse_f64(value: &str, key: &str) -> Result<f64, TextError> {
            value
                .parse::<f64>()
                .map_err(|_| TextError::new(format!("expected number for '{key}', got '{value}'"), TextSpan::at(1, 1)))
        }

        fn parse_usize(value: &str, key: &str) -> Result<usize, TextError> {
            value
                .parse::<usize>()
                .map_err(|_| TextError::new(format!("expected integer for '{key}', got '{value}'"), TextSpan::at(1, 1)))
        }

        /// 🔢 Prints an `f64` via Rust's shortest round-trippable `Display` form (`"0"`, not `"0.0"`).
        fn fmt_num(value: f64) -> String {
            value.to_string()
        }
        //#endregion Lexer

        //#region Document
        /// 📥 Parses a full `.writer` document: one `@writer` header line (schema/id/language/uri/
        /// camera/declared body-line count) followed by exactly that many raw lines of verbatim source
        /// text — no escaping inside the body, so the document reads as plain source code on disk. A
        /// single trailing blank line past the declared count (a hand-saved file's final newline) is
        /// tolerated; any other trailing content is an error.
        pub fn parse_document(source: &str) -> Result<WriterProjection, TextError> {
            let mut lines = source.split('\n');
            let header = lines.next().unwrap_or_default();
            let parsed = parse_kv_line(header)?;
            if parsed.marker != "@writer" {
                return Err(TextError::new(format!("expected a '@writer' header line, got '{}'", parsed.marker), TextSpan::at(1, 1)));
            }
            let schema = field(&parsed.fields, "schema")?.to_string();
            let id = field(&parsed.fields, "id")?.to_string();
            let language_id = field(&parsed.fields, "language")?.to_string();
            let uri = field(&parsed.fields, "uri")?.to_string();
            let x = parse_f64(field(&parsed.fields, "x")?, "x")?;
            let y = parse_f64(field(&parsed.fields, "y")?, "y")?;
            let zoom = parse_f64(field(&parsed.fields, "zoom")?, "zoom")?;
            let line_count = parse_usize(field(&parsed.fields, "lines")?, "lines")?;

            let remaining: Vec<&str> = lines.collect();
            if remaining.len() < line_count {
                return Err(TextError::new(
                    format!("expected {line_count} body line(s), found {}", remaining.len()),
                    TextSpan::at(2, 1),
                ));
            }
            if remaining[line_count..].iter().any(|line| !line.is_empty()) {
                return Err(TextError::new(
                    format!("unexpected trailing content after {line_count} declared body line(s)"),
                    TextSpan::at(line_count as u32 + 2, 1),
                ));
            }
            let text = remaining[..line_count].join("\n");

            Ok(WriterProjection { schema, id, language_id, uri, text, camera: WriterCamera { x, y, zoom } })
        }

        /// 📤 Prints a `WriterProjection` back to its `.writer` DSL form (see {@link parse_document}).
        pub fn print_document(projection: &WriterProjection) -> String {
            let body: Vec<&str> = projection.text.split('\n').collect();
            let mut out = format!(
                "@writer schema={} id={} language={} uri={} x={} y={} zoom={} lines={}",
                projection.schema,
                projection.id,
                projection.language_id,
                projection.uri,
                fmt_num(projection.camera.x),
                fmt_num(projection.camera.y),
                fmt_num(projection.camera.zoom),
                body.len(),
            );
            for line in body {
                out.push('\n');
                out.push_str(line);
            }
            out
        }
        //#endregion Document

        //#region Operation
        /// 📥 Parses a single one-line `WriterOperation`: `setText "..."`, `setCamera x=.. y=.. zoom=..`
        /// or `setDocument schema=.. id=.. language=.. uri=.. x=.. y=.. zoom=.. "..."`.
        pub fn parse_operation(line: &str) -> Result<WriterOperation, TextError> {
            let parsed = parse_kv_line(line)?;
            match parsed.marker.as_str() {
                "setText" => {
                    let text = parsed
                        .text
                        .ok_or_else(|| TextError::new("setText requires a quoted text field", TextSpan::at(1, 1)))?;
                    Ok(WriterOperation::SetText { text })
                }
                "setCamera" => {
                    let x = parse_f64(field(&parsed.fields, "x")?, "x")?;
                    let y = parse_f64(field(&parsed.fields, "y")?, "y")?;
                    let zoom = parse_f64(field(&parsed.fields, "zoom")?, "zoom")?;
                    Ok(WriterOperation::SetCamera { camera: WriterCamera { x, y, zoom } })
                }
                "setDocument" => {
                    let schema = field(&parsed.fields, "schema")?.to_string();
                    let id = field(&parsed.fields, "id")?.to_string();
                    let language_id = field(&parsed.fields, "language")?.to_string();
                    let uri = field(&parsed.fields, "uri")?.to_string();
                    let x = parse_f64(field(&parsed.fields, "x")?, "x")?;
                    let y = parse_f64(field(&parsed.fields, "y")?, "y")?;
                    let zoom = parse_f64(field(&parsed.fields, "zoom")?, "zoom")?;
                    let text = parsed.text.unwrap_or_default();
                    Ok(WriterOperation::SetDocument { document: WriterProjection { schema, id, language_id, uri, text, camera: WriterCamera { x, y, zoom } } })
                }
                other => Err(TextError::expected(format!("unknown writer operation '{other}'"), TextSpan::at(1, 1), "setText | setCamera | setDocument")),
            }
        }

        /// 📤 Prints a `WriterOperation` back to its one-line op text (see {@link parse_operation}).
        pub fn print_operation(operation: &WriterOperation) -> String {
            match operation {
                WriterOperation::SetText { text } => format!("setText \"{}\"", escape_text(text)),
                WriterOperation::SetCamera { camera } => format!("setCamera x={} y={} zoom={}", fmt_num(camera.x), fmt_num(camera.y), fmt_num(camera.zoom)),
                WriterOperation::SetDocument { document } => format!(
                    "setDocument schema={} id={} language={} uri={} x={} y={} zoom={} \"{}\"",
                    document.schema,
                    document.id,
                    document.language_id,
                    document.uri,
                    fmt_num(document.camera.x),
                    fmt_num(document.camera.y),
                    fmt_num(document.camera.zoom),
                    escape_text(&document.text),
                ),
            }
        }
        //#endregion Operation
    }

    impl vcs::DocumentDsl for WriterProjection {
        const EXTENSION: &'static str = "writer";

        fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
            writer_dsl::parse_document(text)
        }

        fn print_dsl(&self) -> String {
            writer_dsl::print_document(self)
        }
    }
    // #endregion 🔖Dsl

    // #region 🔖OpText
    impl vcs::OpText for WriterOperation {
        fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
            writer_dsl::parse_operation(line)
        }

        fn print_op(&self) -> String {
            writer_dsl::print_operation(self)
        }
    }
    // #endregion 🔖OpText

    pub type WriterEnvelope = DocumentVcsEnvelope<WriterProjection, WriterOperation>;
    pub type WriterStore = DocumentVcsStore<WriterProjection, WriterOperation>;

    pub fn empty_writer_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "empty".into(), language_id: "plaintext".into(), uri: "writer://empty".into(), text: String::new(), camera: default_camera() }
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub struct WriterDocumentVcs {
        store: RefCell<WriterStore>,
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    impl WriterDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<WriterDocumentVcs, JsValue> {
            let envelope: WriterEnvelope = serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(WriterStore::new(envelope)) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }

    #[cfg(test)]
    mod writer_vcs_tests {
        use super::*;
        use vcs::{create_document_vcs_envelope, DocumentVcsCommand};

        fn seeded_store() -> WriterStore {
            WriterStore::new(create_document_vcs_envelope("writer.document", "writer", empty_writer_projection(), None))
        }

        #[test]
        fn writer_document_vcs_replays_text_operations() {
            let mut store = seeded_store();
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
            assert_eq!(store.projection().expect("projection").text, "hello");
        }

        #[test]
        fn writer_document_vcs_replays_camera_and_document_operations() {
            let mut store = seeded_store();
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetCamera { camera: WriterCamera { x: 4.0, y: 5.0, zoom: 2.0 } }], description: None }).expect("apply camera");
            let projection = store.projection().expect("projection");
            assert_eq!(projection.camera.x, 4.0);
            assert_eq!(projection.camera.zoom, 2.0);

            let replacement = WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a) RETURN a".into(), camera: default_camera() };
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetDocument { document: replacement }], description: None }).expect("apply document");
            let projection = store.projection().expect("projection");
            assert_eq!(projection.id, "jack");
            assert_eq!(projection.text, "MATCH (a) RETURN a");
        }

        #[test]
        fn writer_document_vcs_undoes_text_operation() {
            let mut store = seeded_store();
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
            store.dispatch(DocumentVcsCommand::Undo).expect("undo");
            assert_eq!(store.projection().expect("projection").text, "");
        }

        //#region 🔖DslAndOpText
        fn jack_projection() -> WriterProjection {
            WriterProjection {
                schema: "writer.document".into(),
                id: "jack".into(),
                language_id: "jack".into(),
                uri: "writer://jack".into(),
                text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name".into(),
                camera: WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            }
        }

        #[test]
        fn writer_dsl_round_trips_empty_and_jack_projections() {
            vcs::test_support::assert_dsl_round_trip(&empty_writer_projection());
            vcs::test_support::assert_dsl_round_trip(&jack_projection());
        }

        #[test]
        fn writer_dsl_prints_readable_multiline_text() {
            let printed = jack_projection().print_dsl();
            assert!(printed.starts_with("@writer schema=writer.document id=jack language=jack uri=writer://jack x=0 y=0 zoom=1 lines=3"));
            assert!(printed.contains("MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name"));
        }

        #[test]
        fn writer_op_text_round_trips_every_variant() {
            vcs::test_support::assert_op_line_round_trip(&WriterOperation::SetText { text: "line one\nline two".into() });
            vcs::test_support::assert_op_line_round_trip(&WriterOperation::SetCamera { camera: WriterCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
            vcs::test_support::assert_op_line_round_trip(&WriterOperation::SetDocument { document: jack_projection() });
        }

        #[test]
        fn writer_document_text_round_trips_through_the_store() {
            let mut store = seeded_store();
            store
                .dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetDocument { document: jack_projection() }], description: None })
                .expect("apply");
            vcs::test_support::assert_document_text_round_trip(&store);
        }
        //#endregion 🔖DslAndOpText
    }
    // #endregion 🔖DocumentVcs
    // #endregion document_vcs
}

pub use document_vcs::*;
