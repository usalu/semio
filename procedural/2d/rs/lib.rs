//! 📏 Procedural 2d document model on `vcs`.

use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use protocol::{apply_generation_operation, invert_generation_operation, FormGeneration, GenerationOperation, GenerationPlayState};
use serde::{Deserialize, Serialize};
use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";

//#region 🔖Document
/// 🧾 Persistent procedural-2d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, show mode, preview evaluations) lives in the plugin app struct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDocument {
    pub fixture: FlowFixture,
    #[serde(default)]
    pub generation: GenerationPlayState,
}

/// 🪪 A flow widget's stable id, across every widget variant (mirrors flow_core's private accessor).
fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖Document

//#region 🔖Collections
/// 🩹 Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id already
/// exists, else insert at the recorded index). Disjoint `set`s on different ids merge cleanly, which
/// is what lets two backbone peers converge on concurrent edits to different widgets/synapses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Widget)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapsesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, SynapseSpec)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDiff {
    pub removed: Vec<String>,
    pub set: Vec<(String, WidgetLayout)>,
}

fn apply_widgets_diff(widgets: &mut Vec<Widget>, diff: &WidgetsDiff) {
    for id in &diff.removed {
        widgets.retain(|widget| widget_id(widget) != id);
    }
    for (index, widget) in &diff.set {
        if let Some(pos) = widgets.iter().position(|entry| widget_id(entry) == widget_id(widget)) {
            widgets[pos] = widget.clone();
        } else {
            widgets.insert((*index).min(widgets.len()), widget.clone());
        }
    }
}

fn apply_synapses_diff(synapses: &mut Vec<SynapseSpec>, diff: &SynapsesDiff) {
    for id in &diff.removed {
        synapses.retain(|synapse| synapse.id != *id);
    }
    for (index, synapse) in &diff.set {
        if let Some(pos) = synapses.iter().position(|entry| entry.id == synapse.id) {
            synapses[pos] = synapse.clone();
        } else {
            synapses.insert((*index).min(synapses.len()), synapse.clone());
        }
    }
}
//#endregion 🔖Collections

//#region 🔖Operations
/// 🩹 Sparse procedural-2d diff over the flow fixture's collections plus scalar canvas/schema fields
/// and an ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
        let mut next = projection.clone();
        apply_widgets_diff(&mut next.fixture.widgets, &self.widgets);
        apply_synapses_diff(&mut next.fixture.synapses, &self.synapses);
        for id in &self.layout.removed {
            next.fixture.layout.remove(id);
        }
        for (id, layout) in &self.layout.set {
            next.fixture.layout.insert(id.clone(), layout.clone());
        }
        if let Some(camera) = &self.camera {
            next.fixture.camera = camera.clone();
        }
        if let Some(schema) = &self.schema {
            next.fixture.schema = schema.clone();
        }
        for operation in &self.generation {
            apply_generation_operation(&mut next.generation, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.widgets.removed.extend(other.widgets.removed);
        self.widgets.set.extend(other.widgets.set);
        self.synapses.removed.extend(other.synapses.removed);
        self.synapses.set.extend(other.synapses.set);
        self.layout.removed.extend(other.layout.removed);
        self.layout.set.extend(other.layout.set);
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.generation.extend(other.generation);
    }
}

/// 🧮 Procedural-2d operation: id-keyed widget/synapse/layout collection edits, the scalar canvas
/// camera and fixture schema, and a single {@link GenerationOperation} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Procedural2dOperation {
    SetWidget { index: usize, widget: Widget },
    RemoveWidget { id: String },
    SetSynapse { index: usize, synapse: SynapseSpec },
    RemoveSynapse { id: String },
    SetLayout { id: String, layout: WidgetLayout },
    RemoveLayout { id: String },
    SetCamera { camera: CameraJson },
    SetSchema { schema: String },
    Generation(GenerationOperation),
}

fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}

impl Operation<Procedural2dDocument> for Procedural2dOperation {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        let mut diff = Procedural2dDiff::default();
        match self {
            Procedural2dOperation::SetWidget { index, widget } => diff.widgets.set.push((*index, widget.clone())),
            Procedural2dOperation::RemoveWidget { id } => diff.widgets.removed.push(id.clone()),
            Procedural2dOperation::SetSynapse { index, synapse } => diff.synapses.set.push((*index, synapse.clone())),
            Procedural2dOperation::RemoveSynapse { id } => diff.synapses.removed.push(id.clone()),
            Procedural2dOperation::SetLayout { id, layout } => diff.layout.set.push((id.clone(), layout.clone())),
            Procedural2dOperation::RemoveLayout { id } => diff.layout.removed.push(id.clone()),
            Procedural2dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Procedural2dOperation::SetSchema { schema } => diff.schema = Some(schema.clone()),
            Procedural2dOperation::Generation(operation) => diff.generation.push(operation.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural2dOperation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural2dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural2dOperation::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural2dOperation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural2dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural2dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural2dOperation::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural2dOperation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural2dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural2dOperation::RemoveLayout { id: id.clone() }],
            },
            Procedural2dOperation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetCamera { .. } => vec![Procedural2dOperation::SetCamera { camera: fixture.camera.clone() }],
            Procedural2dOperation::SetSchema { .. } => vec![Procedural2dOperation::SetSchema { schema: fixture.schema.clone() }],
            Procedural2dOperation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural2dOperation::Generation).collect(),
        }
    }
}

/// 🔀 Diffs two fixtures into a minimal, invertible, mergeable operation set: removed/added/patched widgets
/// and synapses (keyed by id), layout entries, and the fixture schema. The canvas camera is ephemeral
/// view state (plugin runtime), never a document operation. Lets action handlers keep computing the target
/// fixture via `FlowHost` while emitting granular operations.
pub fn procedural2d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dOperation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural2dOperation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural2dOperation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural2dOperation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural2dOperation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural2dOperation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural2dOperation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖Operations

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer for `Procedural2dDocument`'s `.procedural2d` DSL and for
/// `Procedural2dOperation`'s compact single-line op encoding. The outer `document`/`widget`/`synapse`/
/// `layout`/`generation` grammar (`key=value` header fields, `{ }` body) is entirely hand-rolled;
/// genuinely free-form leaf payloads (`Dictionary` params/preview, port/expanded lists, `Tree`/`FlowGui`
/// cluster subtrees, generation `values`/`value`) are carried as an already-JSON-shaped quoted string
/// re-parsed with `serde_json` (already a workspace dependency — mirrors `protocol`'s `kv_json`). See
/// {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod procedural2d_text {
    use super::{Procedural2dDocument, Procedural2dOperation};
    use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
    use protocol::{FormGeneration, GenerationOperation, GenerationPlayState};
    use std::collections::{BTreeMap, HashMap};
    use vcs::{TextError, TextSpan};

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Word(String),
        Str(String),
        LBrace,
        RBrace,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct Lexed {
        tok: Tok,
        span: TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` is an
    /// ordinary word character — `key=value` collapses into one token, split later by `parse_kv_map`.
    fn lex(input: &str) -> Result<Vec<Lexed>, TextError> {
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
                '{' => {
                    out.push(Lexed { tok: Tok::LBrace, span: TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
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
                        return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: TextSpan::at(line, col) });
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

    type FieldMap = HashMap<String, (FieldValue, TextSpan)>;

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
        }

        fn expect_word(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_keyword(&mut self, keyword: &str) -> Result<(), TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            if word != keyword {
                return Err(TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
            }
            Ok(())
        }

        fn expect_lbrace(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        fn expect_eof(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Eof => Ok(()),
                other => Err(TextError::expected(format!("expected end of input, found {other:?}"), span, "eof")),
            }
        }

        /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one — the
        /// generic header-field reader every construct (document/widget/synapse/layout/generation) is
        /// built on. A `set-*` op line's `index`/`id` fields share the same flat map as the nested
        /// widget/synapse/generation fields, since the whole line is a single `parse_kv_map` call.
        fn parse_kv_map(&mut self) -> Result<FieldMap, TextError> {
            let mut map = HashMap::new();
            loop {
                let word = match self.peek() {
                    Tok::Word(w) if w.contains('=') => w.clone(),
                    _ => break,
                };
                let span = self.span();
                self.bump();
                let (key, rest) = word.split_once('=').expect("word already checked to contain '='");
                let value = if rest.is_empty() { FieldValue::Str(self.expect_str()?) } else { FieldValue::Word(rest.to_string()) };
                map.insert(key.to_string(), (value, span));
            }
            Ok(map)
        }

        fn expect_str(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }
    }

    fn kv_str(map: &FieldMap, key: &str, span: TextSpan) -> Result<String, TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(s.clone()),
            Some((FieldValue::Word(_), field_span)) => Err(TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
            None => Err(TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_str(map: &FieldMap, key: &str) -> Option<String> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    }

    fn kv_word(map: &FieldMap, key: &str, span: TextSpan) -> Result<String, TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => Ok(w.clone()),
            Some((FieldValue::Str(_), field_span)) => Err(TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
            None => Err(TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_num(map: &FieldMap, key: &str, span: TextSpan) -> Result<f64, TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<f64>().map_err(|_| TextError::expected(format!("field '{key}' must be a number"), span, "number"))
    }

    fn kv_bool(map: &FieldMap, key: &str, span: TextSpan) -> Result<bool, TextError> {
        match kv_word(map, key, span)?.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(TextError::expected(format!("field '{key}' must be 'true' or 'false'"), span, "true|false")),
        }
    }

    fn kv_usize(map: &FieldMap, key: &str, span: TextSpan) -> Result<usize, TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<usize>().map_err(|_| TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "usize"))
    }

    /// 🧬 Reads a required free-form JSON field (`Dictionary` params/preview, port/expanded lists,
    /// `Tree`/`FlowGui` cluster subtrees, generation `values`/`value`): the field's quoted string
    /// content is itself compact JSON text, re-parsed with `serde_json` (already a workspace
    /// dependency — only the surrounding `.procedural2d`/op-line grammar is hand-rolled).
    fn kv_json<T: serde::de::DeserializeOwned>(map: &FieldMap, key: &str, span: TextSpan) -> Result<T, TextError> {
        let text = kv_str(map, key, span)?;
        serde_json::from_str(&text).map_err(|error| TextError::expected(format!("field '{key}' must be valid JSON: {error}"), span, "json"))
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

    /// 🧬 Prints a value already carried as an opaque JSON payload (see `kv_json`) — compact JSON text
    /// quoted as a single DSL string field.
    fn print_json<T: serde::Serialize>(value: &T) -> String {
        quote(&serde_json::to_string(value).unwrap_or_else(|_| "null".into()))
    }
    //#endregion Printer

    //#region Widget
    fn widget_from_map(map: &FieldMap, span: TextSpan) -> Result<Widget, TextError> {
        let kind = kv_word(map, "kind", span)?;
        let id = kv_str(map, "id", span)?;
        match kind.as_str() {
            "neuron" => Ok(Widget::Neuron {
                id,
                neuron_kind: kv_word(map, "neuronKind", span)?,
                params: kv_json(map, "params", span)?,
                input_ports: kv_json(map, "inputPorts", span)?,
                output_ports: kv_json(map, "outputPorts", span)?,
                preview: kv_bool(map, "preview", span)?,
            }),
            "inputSlider" => Ok(Widget::InputSlider { id, value: kv_num(map, "value", span)?, min: kv_num(map, "min", span)?, max: kv_num(map, "max", span)?, step: kv_num(map, "step", span)? }),
            "inputNote" => Ok(Widget::InputNote { id, text: kv_str(map, "text", span)? }),
            "inputImage" => Ok(Widget::InputImage { id, src: kv_str(map, "src", span)? }),
            "variable" => Ok(Widget::Variable { id, name: kv_str(map, "name", span)?, schema: kv_word(map, "schema", span)? }),
            "outputPreview" => Ok(Widget::OutputPreview { id, preview: kv_json(map, "preview", span)?, expanded: kv_json(map, "expanded", span)? }),
            "outputAction" => Ok(Widget::OutputAction { id, action: kv_str(map, "action", span)? }),
            "outputExport" => Ok(Widget::OutputExport { id, format: kv_word(map, "format", span)? }),
            "cluster" => Ok(Widget::Cluster { id, name: kv_str(map, "name", span)?, tree: kv_json(map, "tree", span)?, flow: kv_json(map, "flow", span)? }),
            other => Err(TextError::expected(
                format!("unknown widget kind '{other}'"),
                span,
                "neuron|inputSlider|inputNote|inputImage|variable|outputPreview|outputAction|outputExport|cluster",
            )),
        }
    }

    fn parse_widget(p: &mut Parser) -> Result<Widget, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        widget_from_map(&map, span)
    }

    fn print_widget_fields(widget: &Widget) -> String {
        match widget {
            Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
                format!(
                    "kind=neuron id={} neuronKind={} preview={} inputPorts={} outputPorts={} params={}",
                    quote(id),
                    neuron_kind,
                    preview,
                    print_json(input_ports),
                    print_json(output_ports),
                    print_json(params)
                )
            }
            Widget::InputSlider { id, value, min, max, step } => format!("kind=inputSlider id={} value={} min={} max={} step={}", quote(id), fmt_num(*value), fmt_num(*min), fmt_num(*max), fmt_num(*step)),
            Widget::InputNote { id, text } => format!("kind=inputNote id={} text={}", quote(id), quote(text)),
            Widget::InputImage { id, src } => format!("kind=inputImage id={} src={}", quote(id), quote(src)),
            Widget::Variable { id, name, schema } => format!("kind=variable id={} name={} schema={}", quote(id), quote(name), schema),
            Widget::OutputPreview { id, preview, expanded } => format!("kind=outputPreview id={} preview={} expanded={}", quote(id), print_json(preview), print_json(expanded)),
            Widget::OutputAction { id, action } => format!("kind=outputAction id={} action={}", quote(id), quote(action)),
            Widget::OutputExport { id, format } => format!("kind=outputExport id={} format={}", quote(id), format),
            Widget::Cluster { id, name, tree, flow } => format!("kind=cluster id={} name={} tree={} flow={}", quote(id), quote(name), print_json(tree), print_json(flow)),
        }
    }
    //#endregion Widget

    //#region Synapse
    fn synapse_from_map(map: &FieldMap, span: TextSpan) -> Result<SynapseSpec, TextError> {
        Ok(SynapseSpec { id: kv_str(map, "id", span)?, from: kv_str(map, "from", span)?, to: kv_str(map, "to", span)?, from_port: kv_str(map, "fromPort", span)?, to_port: kv_str(map, "toPort", span)? })
    }

    fn parse_synapse(p: &mut Parser) -> Result<SynapseSpec, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        synapse_from_map(&map, span)
    }

    fn print_synapse_fields(synapse: &SynapseSpec) -> String {
        format!("id={} from={} to={} fromPort={} toPort={}", quote(&synapse.id), quote(&synapse.from), quote(&synapse.to), quote(&synapse.from_port), quote(&synapse.to_port))
    }
    //#endregion Synapse

    //#region Layout
    fn layout_from_map(map: &FieldMap, span: TextSpan) -> Result<WidgetLayout, TextError> {
        Ok(WidgetLayout { x: kv_num(map, "x", span)?, y: kv_num(map, "y", span)? })
    }

    fn parse_layout(p: &mut Parser) -> Result<(String, WidgetLayout), TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok((kv_str(&map, "id", span)?, layout_from_map(&map, span)?))
    }

    fn print_layout_fields(id: &str, layout: &WidgetLayout) -> String {
        format!("id={} x={} y={}", quote(id), fmt_num(layout.x), fmt_num(layout.y))
    }
    //#endregion Layout

    //#region Generation
    fn generation_from_map(map: &FieldMap, span: TextSpan) -> Result<FormGeneration, TextError> {
        Ok(FormGeneration { id: kv_str(map, "id", span)?, name: kv_str(map, "name", span)?, values: kv_json(map, "values", span)? })
    }

    fn parse_generation(p: &mut Parser) -> Result<FormGeneration, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        generation_from_map(&map, span)
    }

    fn print_generation_fields(generation: &FormGeneration) -> String {
        format!("id={} name={} values={}", quote(&generation.id), quote(&generation.name), print_json(&generation.values))
    }
    //#endregion Generation

    //#region DocumentText
    /// 📥 Parses a full `.procedural2d` document: `document schema=/camera.x=/camera.y=/camera.zoom=
    /// [selectedGeneration=] [previewText=]` header, then a mandatory `{ }` body of `widget`/`synapse`/
    /// `layout`/`generation` records (see `print_document` for the mirrored grammar).
    pub(super) fn parse_document(text: &str) -> Result<Procedural2dDocument, TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        p.expect_keyword("document")?;
        let map = p.parse_kv_map()?;
        let schema = kv_str(&map, "schema", span)?;
        let camera = CameraJson { x: kv_num(&map, "camera.x", span)?, y: kv_num(&map, "camera.y", span)?, zoom: kv_num(&map, "camera.zoom", span)? };
        let selected_generation_id = kv_opt_str(&map, "selectedGeneration");
        let preview_text = kv_opt_str(&map, "previewText");
        p.expect_lbrace()?;
        let mut widgets = Vec::new();
        let mut synapses = Vec::new();
        let mut layout = BTreeMap::new();
        let mut generations = Vec::new();
        while !p.at_rbrace() {
            let keyword_span = p.span();
            let keyword = p.expect_word()?;
            match keyword.as_str() {
                "widget" => widgets.push(parse_widget(&mut p)?),
                "synapse" => synapses.push(parse_synapse(&mut p)?),
                "layout" => {
                    let (id, entry) = parse_layout(&mut p)?;
                    layout.insert(id, entry);
                }
                "generation" => generations.push(parse_generation(&mut p)?),
                other => return Err(TextError::expected(format!("unknown document record '{other}'"), keyword_span, "widget|synapse|layout|generation")),
            }
        }
        p.expect_rbrace()?;
        p.expect_eof()?;
        Ok(Procedural2dDocument {
            fixture: FlowFixture { schema, camera, widgets, synapses, layout },
            generation: GenerationPlayState { generations, selected_generation_id, preview_text },
        })
    }

    pub(super) fn print_document(document: &Procedural2dDocument) -> String {
        let fixture = &document.fixture;
        let generation = &document.generation;
        let mut out = format!(
            "document schema={} camera.x={} camera.y={} camera.zoom={}",
            quote(&fixture.schema),
            fmt_num(fixture.camera.x),
            fmt_num(fixture.camera.y),
            fmt_num(fixture.camera.zoom)
        );
        if let Some(id) = &generation.selected_generation_id {
            out.push_str(&format!(" selectedGeneration={}", quote(id)));
        }
        if let Some(text) = &generation.preview_text {
            out.push_str(&format!(" previewText={}", quote(text)));
        }
        out.push_str(" {\n");
        for widget in &fixture.widgets {
            out.push_str("  widget ");
            out.push_str(&print_widget_fields(widget));
            out.push('\n');
        }
        for synapse in &fixture.synapses {
            out.push_str("  synapse ");
            out.push_str(&print_synapse_fields(synapse));
            out.push('\n');
        }
        for (id, entry) in &fixture.layout {
            out.push_str("  layout ");
            out.push_str(&print_layout_fields(id, entry));
            out.push('\n');
        }
        for entry in &generation.generations {
            out.push_str("  generation ");
            out.push_str(&print_generation_fields(entry));
            out.push('\n');
        }
        out.push_str("}\n");
        out
    }
    //#endregion DocumentText

    //#region OpText
    /// ⚡ Parses one op-log line: a keyword (`set-widget`/`remove-widget`/`set-synapse`/`remove-synapse`/
    /// `set-layout`/`remove-layout`/`set-camera`/`set-schema`/`generation-add`/`generation-remove`/
    /// `generation-rename`/`generation-update-values`) then its own `key=value` fields.
    pub(super) fn parse_operation(line: &str) -> Result<Procedural2dOperation, TextError> {
        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        let keyword = p.expect_word()?;
        let operation = match keyword.as_str() {
            "set-widget" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::SetWidget { index: kv_usize(&map, "index", span)?, widget: widget_from_map(&map, span)? }
            }
            "remove-widget" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::RemoveWidget { id: kv_str(&map, "id", span)? }
            }
            "set-synapse" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::SetSynapse { index: kv_usize(&map, "index", span)?, synapse: synapse_from_map(&map, span)? }
            }
            "remove-synapse" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::RemoveSynapse { id: kv_str(&map, "id", span)? }
            }
            "set-layout" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::SetLayout { id: kv_str(&map, "id", span)?, layout: layout_from_map(&map, span)? }
            }
            "remove-layout" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::RemoveLayout { id: kv_str(&map, "id", span)? }
            }
            "set-camera" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::SetCamera { camera: CameraJson { x: kv_num(&map, "x", span)?, y: kv_num(&map, "y", span)?, zoom: kv_num(&map, "zoom", span)? } }
            }
            "set-schema" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::SetSchema { schema: kv_str(&map, "schema", span)? }
            }
            "generation-add" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::Generation(GenerationOperation::Add { generation: generation_from_map(&map, span)? })
            }
            "generation-remove" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::Generation(GenerationOperation::Remove { id: kv_str(&map, "id", span)? })
            }
            "generation-rename" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::Generation(GenerationOperation::Rename { id: kv_str(&map, "id", span)?, name: kv_str(&map, "name", span)? })
            }
            "generation-update-values" => {
                let map = p.parse_kv_map()?;
                Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id: kv_str(&map, "id", span)?, question_id: kv_str(&map, "questionId", span)?, value: kv_json(&map, "value", span)? })
            }
            other => {
                return Err(TextError::expected(
                    format!("unknown operation '{other}'"),
                    span,
                    "set-widget|remove-widget|set-synapse|remove-synapse|set-layout|remove-layout|set-camera|set-schema|generation-add|generation-remove|generation-rename|generation-update-values",
                ))
            }
        };
        p.expect_eof()?;
        Ok(operation)
    }

    pub(super) fn print_operation(operation: &Procedural2dOperation) -> String {
        match operation {
            Procedural2dOperation::SetWidget { index, widget } => format!("set-widget index={} {}", index, print_widget_fields(widget)),
            Procedural2dOperation::RemoveWidget { id } => format!("remove-widget id={}", quote(id)),
            Procedural2dOperation::SetSynapse { index, synapse } => format!("set-synapse index={} {}", index, print_synapse_fields(synapse)),
            Procedural2dOperation::RemoveSynapse { id } => format!("remove-synapse id={}", quote(id)),
            Procedural2dOperation::SetLayout { id, layout } => format!("set-layout {}", print_layout_fields(id, layout)),
            Procedural2dOperation::RemoveLayout { id } => format!("remove-layout id={}", quote(id)),
            Procedural2dOperation::SetCamera { camera } => format!("set-camera x={} y={} zoom={}", fmt_num(camera.x), fmt_num(camera.y), fmt_num(camera.zoom)),
            Procedural2dOperation::SetSchema { schema } => format!("set-schema schema={}", quote(schema)),
            Procedural2dOperation::Generation(GenerationOperation::Add { generation }) => format!("generation-add {}", print_generation_fields(generation)),
            Procedural2dOperation::Generation(GenerationOperation::Remove { id }) => format!("generation-remove id={}", quote(id)),
            Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }) => format!("generation-rename id={} name={}", quote(id), quote(name)),
            Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
                format!("generation-update-values id={} questionId={} value={}", quote(id), quote(question_id), print_json(value))
            }
        }
    }
    //#endregion OpText
}

/// 📜 `.procedural2d` textual document: `document schema=... camera.x=/y=/zoom=
/// [selectedGeneration=] [previewText=] { widget ... synapse ... layout ... generation ... }` — see
/// `procedural2d_text` for the hand-rolled lexer/parser/printer.
impl vcs::DocumentDsl for Procedural2dDocument {
    const EXTENSION: &'static str = "procedural2d";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        procedural2d_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        procedural2d_text::print_document(self)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
/// ⚡ `Procedural2dOperation`'s compact single-line op encoding — see `procedural2d_text`.
impl vcs::OpText for Procedural2dOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        procedural2d_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        procedural2d_text::print_operation(self)
    }
}
//#endregion 🔖OpText

pub type Procedural2dEnvelope = DocumentVcsEnvelope<Procedural2dDocument, Procedural2dOperation>;
pub type Procedural2dStore = DocumentVcsStore<Procedural2dDocument, Procedural2dOperation>;

pub fn empty_procedural2d_projection() -> Procedural2dDocument {
    Procedural2dDocument::default()
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use vcs::create_document_vcs_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural2dDocumentVcs {
        store: RefCell<Procedural2dStore>,
    }

    #[wasm_bindgen]
    impl Procedural2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural2dStore::new(envelope)
                }
                None => Procedural2dStore::new(create_document_vcs_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
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
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::{apply_operation, create_document_vcs_envelope, test_support, DocumentDsl, DocumentVcsCommand, OpText};

    fn round_trip(projection: &Procedural2dDocument, operation: &Procedural2dOperation) -> Procedural2dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural2dOperation::SetCamera { .. })));
    }

    #[test]
    fn remove_and_readd_widget_round_trips() {
        let base = empty_procedural2d_projection();
        let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
        let after = round_trip(&base, &Procedural2dOperation::RemoveWidget { id: removed_id.clone() });
        assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
    }

    #[test]
    fn fixture_ops_capture_widget_add() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Procedural2dOperation::SetWidget { widget, .. } if widget_id(widget) == "note-1")));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural2d_projection();
        let generation = protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    //#region 🔖DslTests
    #[test]
    fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&empty_procedural2d_projection());
    }

    #[test]
    fn dsl_round_trip_example_fixture() {
        let text = include_str!("../example/default.procedural2d");
        let projection = Procedural2dDocument::parse_dsl(text).expect("parse default.procedural2d fixture");
        test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn dsl_round_trip_with_generation_state() {
        let mut projection = empty_procedural2d_projection();
        let mut values = serde_json::Map::new();
        values.insert("count".into(), serde_json::json!(3));
        projection.generation.generations.push(protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_round_trip(&projection);
    }
    //#endregion 🔖DslTests

    //#region 🔖OpTextTests
    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSynapse {
            index: 1,
            synapse: SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() },
        });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveSynapse { id: "s1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetLayout { id: "rect".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveLayout { id: "rect".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural2dStore::new(create_document_vcs_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests
