//! 🪚 Process 3d document VCS on `vcs` — subtractive/additive processing steps on a stock solid.

use serde::{Deserialize, Serialize};
use vcs::{apply_collection_operation, invert_collection_operation, CollectionOperation, DocumentDsl, DocumentVcsEnvelope, DocumentVcsStore, Identified, OperationDiff, Patchable};

pub const PROCESS_3D_SCHEMA: &str = "process.3d";

//#region 🔖Document
fn default_axis_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

fn default_true() -> bool {
    true
}

/// 🧭 Position + axis-angle rotation applied via the brep kernel's `rotate_sync`/`translate_sync`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pose {
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default = "default_axis_z")]
    pub axis: [f64; 3],
    #[serde(default)]
    pub angle: f64,
}

/// 📦 Primitive solid spec resolvable via `BrepkitKernel::*_prim_sync`, or a non-parametric imported
/// reference (mesh or real B-Rep solid) resolved by the app's own kernel session instead of a primitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SolidSpec {
    Box {
        width: f64,
        depth: f64,
        height: f64,
    },
    Cylinder {
        radius: f64,
        height: f64,
    },
    Sphere {
        radius: f64,
    },
    /// 🖼️ Non-parametric GLB-imported reference mesh — tessellation-only, no real B-Rep topology
    /// (mirrors `cad`'s `meshUrl` pattern); cannot serve as a Cut/Drill/Attach tool.
    ImportedMesh {
        mesh_url: String,
    },
    /// 🧊 STEP/OBJ/STL-imported solid with real B-Rep topology, resolved through the app's kernel
    /// session by handle id (mirrors `cad`'s `solidHandle` pattern); ephemeral to that session.
    ImportedSolid {
        solid_handle: String,
    },
}

/// 🪵 The raw workpiece the process starts from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    pub id: String,
    pub label: String,
    pub solid: SolidSpec,
    #[serde(default)]
    pub pose: Pose,
}

impl Default for Stock {
    fn default() -> Self {
        Self { id: "stock".into(), label: "Stock".into(), solid: SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }
    }
}

/// 🪚 One processing measure: subtractive (cut/drill via `cut_sync`) or additive (attach via `fuse_sync`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "measure", rename_all = "camelCase")]
pub enum ProcessMeasure {
    /// ✂️ Subtractive: subtracts an arbitrary tool solid (e.g. a thin box as a saw blade).
    Cut { tool: SolidSpec, pose: Pose },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    Drill { radius: f64, depth: f64, pose: Pose },
    /// 🔩 Additive: fuses another component solid at `pose`.
    Attach { component: SolidSpec, pose: Pose },
}

/// 🏭 Provenance: which module/machine/modification-kind produced a step (display + future re-validation).
/// Purely informational — kernel replay only ever reads `ProcessMeasure`, never resolves this back to a
/// catalog entry, so an older/renamed catalog can never retroactively change already-authored geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOrigin {
    pub module_id: String,
    pub machine_id: String,
    pub modification_kind_id: String,
}

/// 🎞️ One ordered step of the process timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStep {
    pub id: String,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<StepOrigin>,
    #[serde(flatten)]
    pub measure: ProcessMeasure,
}

impl Identified<String> for ProcessStep {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹 Sparse edit for a `ProcessStep` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStepPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measure: Option<ProcessMeasure>,
    /// 🏭 Outer `Option` = "this patch touches origin"; inner `Option` = the new value (`None` clears it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Option<StepOrigin>>,
}

impl Patchable<ProcessStepPatch> for ProcessStep {
    fn apply_patch(&mut self, patch: &ProcessStepPatch) -> ProcessStepPatch {
        let inverse = ProcessStepPatch {
            label: patch.label.as_ref().map(|_| self.label.clone()),
            enabled: patch.enabled.as_ref().map(|_| self.enabled),
            measure: patch.measure.as_ref().map(|_| self.measure.clone()),
            origin: patch.origin.as_ref().map(|_| self.origin.clone()),
        };
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
        if let Some(measure) = &patch.measure {
            self.measure = measure.clone();
        }
        if let Some(origin) = &patch.origin {
            self.origin = origin.clone();
        }
        inverse
    }
}

/// 🪚 Process 3d projection: stock + ordered steps + timeline cursor.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDocument {
    #[serde(default)]
    pub stock: Stock,
    #[serde(default)]
    pub steps: Vec<ProcessStep>,
    /// ⏱️ Number of enabled steps replayed (0..=steps.len()); `None` applies all.
    #[serde(default)]
    pub resolved_up_to: Option<usize>,
}

pub fn empty_process3d_projection() -> Process3dDocument {
    Process3dDocument::default()
}
//#endregion 🔖Document

//#region 🔖Operations
/// 🪚 Process 3d document operation: an ordered-step collection edit, a stock swap, or a cursor move.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Process3dOperation {
    Steps {
        collection: CollectionOperation<String, ProcessStep, ProcessStepPatch>,
    },
    SetStock {
        stock: Stock,
    },
    SetCursor {
        resolved_up_to: Option<usize>,
    },
    /// 🔁 Wholesale document swap (loading a different example fixture) — a true inverse restores the
    /// exact prior document, mirroring `ShootingOperation::SetFixture`.
    SetDocument {
        document: Process3dDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<CollectionOperation<String, ProcessStep, ProcessStepPatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stock: Option<Stock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Process3dDocument>,
}

impl OperationDiff<Process3dDocument> for Process3dDiff {
    fn apply(&self, projection: &Process3dDocument) -> Process3dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(operation) = &self.steps {
            apply_collection_operation(&mut next.steps, operation);
        }
        if let Some(stock) = &self.stock {
            next.stock = stock.clone();
        }
        if let Some(cursor) = &self.cursor {
            next.resolved_up_to = *cursor;
        }
        if let Some(cursor) = next.resolved_up_to {
            next.resolved_up_to = Some(cursor.min(next.steps.len()));
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
            self.steps = None;
            self.stock = None;
            self.cursor = None;
            return;
        }
        if other.steps.is_some() {
            self.steps = other.steps;
        }
        if other.stock.is_some() {
            self.stock = other.stock;
        }
        if other.cursor.is_some() {
            self.cursor = other.cursor;
        }
    }
}

impl vcs::Operation<Process3dDocument> for Process3dOperation {
    type Diff = Process3dDiff;

    fn diff(&self, _projection: &Process3dDocument) -> Self::Diff {
        match self {
            Process3dOperation::Steps { collection } => Process3dDiff { steps: Some(collection.clone()), ..Default::default() },
            Process3dOperation::SetStock { stock } => Process3dDiff { stock: Some(stock.clone()), ..Default::default() },
            Process3dOperation::SetCursor { resolved_up_to } => Process3dDiff { cursor: Some(*resolved_up_to), ..Default::default() },
            Process3dOperation::SetDocument { document } => Process3dDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &Process3dDocument) -> Vec<Self> {
        match self {
            Process3dOperation::Steps { collection } => {
                vec![Process3dOperation::Steps { collection: invert_collection_operation(&projection.steps, collection) }]
            }
            Process3dOperation::SetStock { .. } => vec![Process3dOperation::SetStock { stock: projection.stock.clone() }],
            Process3dOperation::SetCursor { .. } => vec![Process3dOperation::SetCursor { resolved_up_to: projection.resolved_up_to }],
            Process3dOperation::SetDocument { .. } => vec![Process3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Process3dEnvelope = DocumentVcsEnvelope<Process3dDocument, Process3dOperation>;
pub type Process3dStore = DocumentVcsStore<Process3dDocument, Process3dOperation>;
//#endregion 🔖Operations

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer for `Process3dDocument`'s `.process3d` DSL and
/// `Process3dOperation`'s compact single-line op encoding (`SetDocument` reprints the same document
/// grammar on one line; `Steps` reuses the same per-step field grammar the document's own `steps`
/// section uses). Whitespace (including newlines) is never significant to the parser — `print_dsl`
/// inserts newlines/indentation purely for readability, `print_op` renders the identical grammar with
/// spaces only. See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod process3d_text {
    use super::*;
    use std::collections::HashMap;

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
        span: vcs::TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` is an
    /// ordinary word character — `key=value` collapses into one token (split later by
    /// {@link Parser::parse_kv_map}), and only a quoted value forces a token boundary right after `key=`.
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
                '{' => {
                    out.push(Lexed { tok: Tok::LBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: vcs::TextSpan::at(line, col) });
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
                        return Err(vcs::TextError::new("unterminated string literal", vcs::TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: vcs::TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
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

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
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

        fn expect_lbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        fn expect_str(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(vcs::TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }

        /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one — the
        /// generic header-field reader every construct (document/stock/step/patch) is built on.
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
    }

    type FieldMap = HashMap<String, (FieldValue, vcs::TextSpan)>;

    fn kv_str(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(s.clone()),
            Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_str(map: &FieldMap, key: &str) -> Option<String> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    }

    fn kv_word(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => Ok(w.clone()),
            Some((FieldValue::Str(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
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

    fn kv_opt_bool(map: &FieldMap, key: &str) -> Option<bool> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) if w == "true" => Some(true),
            Some((FieldValue::Word(w), _)) if w == "false" => Some(false),
            _ => None,
        }
    }

    fn kv_usize(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<usize, vcs::TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<usize>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "usize"))
    }

    fn kv_opt_usize(map: &FieldMap, key: &str) -> Option<usize> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) if w != "-" => w.parse::<usize>().ok(),
            _ => None,
        }
    }

    fn parse_vec3(word: &str, span: vcs::TextSpan) -> Result<[f64; 3], vcs::TextError> {
        let parts: Vec<&str> = word.split(',').collect();
        if parts.len() != 3 {
            return Err(vcs::TextError::expected("expected 3 comma-separated numbers", span, "x,y,z"));
        }
        let mut out = [0.0; 3];
        for (index, part) in parts.iter().enumerate() {
            out[index] = part.parse::<f64>().map_err(|_| vcs::TextError::expected(format!("invalid vector component '{part}'"), span, "number"))?;
        }
        Ok(out)
    }

    fn kv_vec3(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<[f64; 3], vcs::TextError> {
        parse_vec3(&kv_word(map, key, span)?, span)
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

    fn fmt_vec3(value: [f64; 3]) -> String {
        format!("{},{},{}", fmt_num(value[0]), fmt_num(value[1]), fmt_num(value[2]))
    }

    fn fmt_opt_str(value: &Option<String>) -> String {
        value.as_deref().map(quote).unwrap_or_else(|| "-".to_string())
    }

    fn fmt_opt_bool(value: Option<bool>) -> String {
        value.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string())
    }

    fn fmt_opt_usize(value: Option<usize>) -> String {
        value.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string())
    }

    fn indent_str(depth: usize) -> String {
        "  ".repeat(depth)
    }

    /// 🧱 Wraps `items` (each already rendered, without its own leading indentation) in `{ }`, one per
    /// line indented at `depth + 1` when `pretty`, or space-joined on one line otherwise.
    fn wrap_body(items: &[String], depth: usize, pretty: bool) -> String {
        if pretty {
            let inner_pad = indent_str(depth + 1);
            let outer_pad = indent_str(depth);
            let body: String = items.iter().map(|item| format!("{inner_pad}{item}\n")).collect();
            format!("{{\n{body}{outer_pad}}}")
        } else {
            format!("{{ {} }}", items.join(" "))
        }
    }
    //#endregion Printer

    //#region SolidSpec
    /// 📦 Shared solid-spec grammar reused by `stock`'s own solid and a `cut`/`attach` measure's
    /// tool/component solid — each line embeds at most one solid, so the field names below never
    /// collide with a sibling construct on the same line.
    fn print_solid_kv(solid: &SolidSpec) -> String {
        match solid {
            SolidSpec::Box { width, depth, height } => format!("solidKind=box width={} depth={} height={}", fmt_num(*width), fmt_num(*depth), fmt_num(*height)),
            SolidSpec::Cylinder { radius, height } => format!("solidKind=cylinder radius={} height={}", fmt_num(*radius), fmt_num(*height)),
            SolidSpec::Sphere { radius } => format!("solidKind=sphere radius={}", fmt_num(*radius)),
            SolidSpec::ImportedMesh { mesh_url } => format!("solidKind=importedMesh meshUrl={}", quote(mesh_url)),
            SolidSpec::ImportedSolid { solid_handle } => format!("solidKind=importedSolid solidHandle={}", quote(solid_handle)),
        }
    }

    fn parse_solid_kv(map: &FieldMap, span: vcs::TextSpan) -> Result<SolidSpec, vcs::TextError> {
        match kv_word(map, "solidKind", span)?.as_str() {
            "box" => Ok(SolidSpec::Box { width: kv_num(map, "width", span)?, depth: kv_num(map, "depth", span)?, height: kv_num(map, "height", span)? }),
            "cylinder" => Ok(SolidSpec::Cylinder { radius: kv_num(map, "radius", span)?, height: kv_num(map, "height", span)? }),
            "sphere" => Ok(SolidSpec::Sphere { radius: kv_num(map, "radius", span)? }),
            "importedMesh" => Ok(SolidSpec::ImportedMesh { mesh_url: kv_str(map, "meshUrl", span)? }),
            "importedSolid" => Ok(SolidSpec::ImportedSolid { solid_handle: kv_str(map, "solidHandle", span)? }),
            other => Err(vcs::TextError::expected(format!("unknown solid kind '{other}'"), span, "box|cylinder|sphere|importedMesh|importedSolid")),
        }
    }
    //#endregion SolidSpec

    //#region Pose
    fn print_pose_kv(pose: &Pose) -> String {
        format!("pos={} axis={} angle={}", fmt_vec3(pose.position), fmt_vec3(pose.axis), fmt_num(pose.angle))
    }

    fn parse_pose_kv(map: &FieldMap, span: vcs::TextSpan) -> Result<Pose, vcs::TextError> {
        Ok(Pose { position: kv_vec3(map, "pos", span)?, axis: kv_vec3(map, "axis", span)?, angle: kv_num(map, "angle", span)? })
    }
    //#endregion Pose

    //#region StepOrigin
    /// 🏭 `Option<StepOrigin>` as three sibling fields, each `-` when the origin is absent — avoids a
    /// single combined field that would need its own internal delimiter/escaping scheme.
    fn print_origin_kv(origin: &Option<StepOrigin>) -> String {
        match origin {
            Some(o) => format!("originModule={} originMachine={} originKind={}", quote(&o.module_id), quote(&o.machine_id), quote(&o.modification_kind_id)),
            None => "originModule=- originMachine=- originKind=-".to_string(),
        }
    }

    fn parse_origin_kv(map: &FieldMap, span: vcs::TextSpan) -> Result<Option<StepOrigin>, vcs::TextError> {
        match kv_opt_str(map, "originModule") {
            Some(module_id) => Ok(Some(StepOrigin { module_id, machine_id: kv_str(map, "originMachine", span)?, modification_kind_id: kv_str(map, "originKind", span)? })),
            None => Ok(None),
        }
    }
    //#endregion StepOrigin

    //#region ProcessMeasure
    fn print_measure_kv(measure: &ProcessMeasure) -> String {
        match measure {
            ProcessMeasure::Cut { tool, pose } => format!("measure=cut {} {}", print_solid_kv(tool), print_pose_kv(pose)),
            ProcessMeasure::Drill { radius, depth, pose } => format!("measure=drill radius={} depth={} {}", fmt_num(*radius), fmt_num(*depth), print_pose_kv(pose)),
            ProcessMeasure::Attach { component, pose } => format!("measure=attach {} {}", print_solid_kv(component), print_pose_kv(pose)),
        }
    }

    fn parse_measure_kv(map: &FieldMap, span: vcs::TextSpan) -> Result<ProcessMeasure, vcs::TextError> {
        match kv_word(map, "measure", span)?.as_str() {
            "cut" => Ok(ProcessMeasure::Cut { tool: parse_solid_kv(map, span)?, pose: parse_pose_kv(map, span)? }),
            "drill" => Ok(ProcessMeasure::Drill { radius: kv_num(map, "radius", span)?, depth: kv_num(map, "depth", span)?, pose: parse_pose_kv(map, span)? }),
            "attach" => Ok(ProcessMeasure::Attach { component: parse_solid_kv(map, span)?, pose: parse_pose_kv(map, span)? }),
            other => Err(vcs::TextError::expected(format!("unknown measure kind '{other}'"), span, "cut|drill|attach")),
        }
    }
    //#endregion ProcessMeasure

    //#region Stock
    fn print_stock_fields(stock: &Stock) -> String {
        format!("id={} label={} {} {}", quote(&stock.id), quote(&stock.label), print_solid_kv(&stock.solid), print_pose_kv(&stock.pose))
    }

    fn parse_stock_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<Stock, vcs::TextError> {
        Ok(Stock { id: kv_str(map, "id", span)?, label: kv_str(map, "label", span)?, solid: parse_solid_kv(map, span)?, pose: parse_pose_kv(map, span)? })
    }
    //#endregion Stock

    //#region Step
    fn print_step_fields(step: &ProcessStep) -> String {
        format!("id={} label={} enabled={} {} {}", quote(&step.id), quote(&step.label), step.enabled, print_origin_kv(&step.origin), print_measure_kv(&step.measure))
    }

    fn parse_step_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<ProcessStep, vcs::TextError> {
        Ok(ProcessStep { id: kv_str(map, "id", span)?, label: kv_str(map, "label", span)?, enabled: kv_bool(map, "enabled", span)?, origin: parse_origin_kv(map, span)?, measure: parse_measure_kv(map, span)? })
    }

    /// 🩹 `measureSet`/`originTouch` flags gate whether the sparse patch fields are even present on the
    /// line, so a `false` patch line never carries the fields it doesn't touch (mirrors
    /// `shooting_text`'s `cameraSet` flag on `ShootingSavedCameraPatch`).
    fn print_step_patch_fields(patch: &ProcessStepPatch) -> String {
        let mut parts = vec![format!("label={}", fmt_opt_str(&patch.label)), format!("enabled={}", fmt_opt_bool(patch.enabled))];
        match &patch.measure {
            Some(measure) => parts.push(format!("measureSet=true {}", print_measure_kv(measure))),
            None => parts.push("measureSet=false".to_string()),
        }
        match &patch.origin {
            Some(Some(origin)) => parts.push(format!("originTouch=true {}", print_origin_kv(&Some(origin.clone())))),
            Some(None) => parts.push("originTouch=true originModule=- originMachine=- originKind=-".to_string()),
            None => parts.push("originTouch=false".to_string()),
        }
        parts.join(" ")
    }

    fn parse_step_patch_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<ProcessStepPatch, vcs::TextError> {
        let measure_set = kv_bool(map, "measureSet", span)?;
        let origin_touch = kv_bool(map, "originTouch", span)?;
        Ok(ProcessStepPatch {
            label: kv_opt_str(map, "label"),
            enabled: kv_opt_bool(map, "enabled"),
            measure: if measure_set { Some(parse_measure_kv(map, span)?) } else { None },
            origin: if origin_touch { Some(parse_origin_kv(map, span)?) } else { None },
        })
    }
    //#endregion Step

    //#region CollectionOp
    /// 🧺 Shared printer for the `CollectionOperation<String, ProcessStep, ProcessStepPatch>`-wrapped
    /// `Steps` operation variant — `steps-add`/`-remove`/`-move`/`-patch`, reusing `ProcessStep`/
    /// `ProcessStepPatch`'s own field grammar so the collection op never duplicates a parsing rule.
    fn print_collection_op(keyword: &str, op: &CollectionOperation<String, ProcessStep, ProcessStepPatch>) -> String {
        match op {
            CollectionOperation::Add { index, item } => format!("{keyword}-add index={index} {}", print_step_fields(item)),
            CollectionOperation::Remove { id } => format!("{keyword}-remove id={}", quote(id)),
            CollectionOperation::Move { id, to_index } => format!("{keyword}-move id={} to={to_index}", quote(id)),
            CollectionOperation::Patch { id, patch } => format!("{keyword}-patch id={} {}", quote(id), print_step_patch_fields(patch)),
        }
    }

    fn parse_collection_op_from_map(map: &FieldMap, span: vcs::TextSpan, suffix: &str) -> Result<CollectionOperation<String, ProcessStep, ProcessStepPatch>, vcs::TextError> {
        match suffix {
            "add" => Ok(CollectionOperation::Add { index: kv_usize(map, "index", span)?, item: parse_step_fields(map, span)? }),
            "remove" => Ok(CollectionOperation::Remove { id: kv_str(map, "id", span)? }),
            "move" => Ok(CollectionOperation::Move { id: kv_str(map, "id", span)?, to_index: kv_usize(map, "to", span)? }),
            "patch" => Ok(CollectionOperation::Patch { id: kv_str(map, "id", span)?, patch: parse_step_patch_fields(map, span)? }),
            other => Err(vcs::TextError::expected(format!("unknown '{other}' collection operation, expected add|remove|move|patch"), span, "add|remove|move|patch")),
        }
    }
    //#endregion CollectionOp

    //#region Document
    /// 📥 Parses a full `.process3d` document: `process` header (just `resolvedUpTo`), `stock`, then
    /// `steps { step ... }` — a fixed order matching {@link print_document}, since `print_document` is
    /// this grammar's only producer.
    pub(super) fn parse_document(text: &str) -> Result<Process3dDocument, vcs::TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };

        p.expect_keyword("process")?;
        let header_map = p.parse_kv_map()?;
        let resolved_up_to = kv_opt_usize(&header_map, "resolvedUpTo");

        let stock_span = p.span();
        p.expect_keyword("stock")?;
        let stock_map = p.parse_kv_map()?;
        let stock = parse_stock_fields(&stock_map, stock_span)?;

        p.expect_keyword("steps")?;
        p.expect_lbrace()?;
        let mut steps = Vec::new();
        while !p.at_rbrace() {
            let span = p.span();
            p.expect_keyword("step")?;
            let map = p.parse_kv_map()?;
            steps.push(parse_step_fields(&map, span)?);
        }
        p.expect_rbrace()?;

        Ok(Process3dDocument { stock, steps, resolved_up_to })
    }

    /// 📤 Renders `doc` as `process`/`stock` (always present) followed by the `steps` collection
    /// section (always present, possibly with an empty body) — mirrors {@link parse_document}.
    pub(super) fn print_document(doc: &Process3dDocument, pretty: bool) -> String {
        let mut parts = Vec::new();
        parts.push(format!("process resolvedUpTo={}", fmt_opt_usize(doc.resolved_up_to)));
        parts.push(format!("stock {}", print_stock_fields(&doc.stock)));
        let step_items: Vec<String> = doc.steps.iter().map(|step| format!("step {}", print_step_fields(step))).collect();
        parts.push(format!("steps {}", wrap_body(&step_items, 0, pretty)));
        parts.join(if pretty { "\n" } else { " " })
    }
    //#endregion Document

    //#region Operation
    /// ⚡ Renders one `Process3dOperation` as a single line — `SetDocument` reuses the compact
    /// (space-joined) form of {@link print_document}.
    pub(super) fn print_operation(operation: &Process3dOperation) -> String {
        match operation {
            Process3dOperation::Steps { collection } => print_collection_op("steps", collection),
            Process3dOperation::SetStock { stock } => format!("stock {}", print_stock_fields(stock)),
            Process3dOperation::SetCursor { resolved_up_to } => format!("cursor value={}", fmt_opt_usize(*resolved_up_to)),
            Process3dOperation::SetDocument { document } => format!("document {}", print_document(document, false)),
        }
    }

    /// 📥 Parses one op-log line. `document ...` (which embeds a whole compact document — itself a
    /// nested instance of this same grammar) is handled as a direct string slice before tokenizing,
    /// mirroring the "one technology, one grammar" reuse from {@link print_operation}.
    pub(super) fn parse_operation(line: &str) -> Result<Process3dOperation, vcs::TextError> {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("document ") {
            return Ok(Process3dOperation::SetDocument { document: parse_document(rest)? });
        }

        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        let keyword = p.expect_word()?;

        if let Some(suffix) = keyword.strip_prefix("steps-") {
            let map = p.parse_kv_map()?;
            return Ok(Process3dOperation::Steps { collection: parse_collection_op_from_map(&map, span, suffix)? });
        }

        match keyword.as_str() {
            "stock" => {
                let map = p.parse_kv_map()?;
                Ok(Process3dOperation::SetStock { stock: parse_stock_fields(&map, span)? })
            }
            "cursor" => {
                let map = p.parse_kv_map()?;
                Ok(Process3dOperation::SetCursor { resolved_up_to: kv_opt_usize(&map, "value") })
            }
            other => Err(vcs::TextError::expected(format!("unknown operation '{other}'"), span, "operation keyword")),
        }
    }
    //#endregion Operation
}

impl vcs::DocumentDsl for Process3dDocument {
    const EXTENSION: &'static str = "process3d";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        process3d_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        process3d_text::print_document(self, true)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for Process3dOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        process3d_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        process3d_text::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use vcs::create_document_vcs_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Process3dDocumentVcs {
        store: RefCell<Process3dStore>,
    }

    #[wasm_bindgen]
    impl Process3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Process3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Process3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Process3dStore::new(envelope)
                }
                None => Process3dStore::new(create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None)),
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
    use vcs::{create_document_vcs_envelope, test_support, Author, DocumentVcsCommand, Operation};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn new_store() -> Process3dStore {
        Process3dStore::new(create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None))
    }

    #[test]
    fn adds_and_removes_steps() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.steps[0].id, "cut-1");

        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "cut-1".into() } }], description: None }).expect("remove step");
        assert!(store.projection().expect("projection").steps.is_empty());
    }

    #[test]
    fn patches_a_step_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        store
            .dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch: ProcessStepPatch { enabled: Some(false), ..Default::default() } } }], description: None })
            .expect("patch step");
        assert!(!store.projection().expect("projection").steps[0].enabled);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].enabled);
    }

    #[test]
    fn patches_origin_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());

        let origin = StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() };
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch: ProcessStepPatch { origin: Some(Some(origin.clone())), ..Default::default() } } }],
                description: None,
            })
            .expect("patch origin");
        assert_eq!(store.projection().expect("projection").steps[0].origin, Some(origin));

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());
    }

    #[test]
    fn legacy_step_json_without_origin_deserializes_with_none() {
        let legacy_json = r#"{"id":"cut-1","label":"Cut","enabled":true,"measure":"cut","tool":{"kind":"box","width":0.1,"depth":0.1,"height":0.1},"pose":{"position":[0.0,0.0,0.0],"axis":[0.0,0.0,1.0],"angle":0.0}}"#;
        let step: ProcessStep = serde_json::from_str(legacy_json).expect("legacy step json");
        assert!(step.origin.is_none());
        assert_eq!(step.id, "cut-1");
    }

    #[test]
    fn moves_and_clamps_cursor() {
        let mut store = new_store();
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("a") } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 1, item: cut_step("b") } },
                    Process3dOperation::SetCursor { resolved_up_to: Some(2) },
                ],
                description: None,
            })
            .expect("build steps + cursor");
        assert_eq!(store.projection().expect("projection").resolved_up_to, Some(2));

        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "b".into() } }], description: None }).expect("remove step clamps cursor");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.resolved_up_to, Some(1));
    }

    #[test]
    fn sets_stock_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let new_stock = Stock { id: "beam".into(), label: "Beam".into(), solid: SolidSpec::Cylinder { radius: 0.2, height: 2.0 }, pose: Pose::default() };
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::SetStock { stock: new_stock.clone() }], description: None }).expect("set stock");
        assert_eq!(store.projection().expect("projection").stock, new_stock);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    #[test]
    fn imported_mesh_solid_spec_round_trips_json() {
        let solid = SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() };
        let json = serde_json::to_value(&solid).expect("serialize");
        assert_eq!(json["kind"], "importedMesh");
        assert_eq!(json["meshUrl"], "data:model/gltf-binary;base64,AAAA");
        let parsed: SolidSpec = serde_json::from_value(json).expect("deserialize");
        assert_eq!(parsed, solid);
    }

    #[test]
    fn imported_solid_solid_spec_round_trips_json() {
        let solid = SolidSpec::ImportedSolid { solid_handle: "solid-42".into() };
        let json = serde_json::to_value(&solid).expect("serialize");
        assert_eq!(json["kind"], "importedSolid");
        assert_eq!(json["solidHandle"], "solid-42");
        let parsed: SolidSpec = serde_json::from_value(json).expect("deserialize");
        assert_eq!(parsed, solid);
    }

    #[test]
    fn sets_stock_to_imported_solid_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let imported_stock = Stock { id: "stock".into(), label: "Imported STEP".into(), solid: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() };
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::SetStock { stock: imported_stock.clone() }], description: None }).expect("set imported stock");
        assert_eq!(store.projection().expect("projection").stock, imported_stock);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    #[test]
    fn backwards_of_add_is_remove() {
        let projection = empty_process3d_projection();
        let operation = Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("a") } };
        let inverse = operation.backwards(&projection);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dOperation::Steps { collection: CollectionOperation::Remove { id } } => assert_eq!(id, "a"),
            _ => panic!("expected Steps::Remove"),
        }
    }

    //#region 🔖DslTests
    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() }), measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() } }
    }

    fn attach_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Attach".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } } }
    }

    fn imported_mesh_stock() -> Stock {
        Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
    }

    /// 📜 A document exercising every `SolidSpec`/`ProcessMeasure` shape and both `origin` states, so
    /// the DSL round trip covers the full grammar, not just the happy path.
    fn sample_document() -> Process3dDocument {
        Process3dDocument {
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn process3d_dsl_round_trips() {
        test_support::assert_dsl_round_trip(&sample_document());
        test_support::assert_dsl_round_trip(&empty_process3d_projection());
    }

    #[test]
    fn process3d_dsl_round_trips_imported_solid_shapes() {
        let mut document = sample_document();
        document.stock = imported_mesh_stock();
        document.steps.push(ProcessStep { id: "imported-tool".into(), label: "Imported Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() } });
        test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn process3d_dsl_round_trips_with_no_resolved_cursor() {
        let mut document = sample_document();
        document.resolved_up_to = None;
        test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn timber_example_fixture_parses_and_round_trips() {
        let text = include_str!("../example/timber-beam-joinery.process3d");
        let document = Process3dDocument::parse_dsl(text).expect("parse timber example");
        assert_eq!(document.steps.len(), 4);
        assert!(document.resolved_up_to.is_none());
        test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn drilled_plate_example_fixture_parses_and_round_trips() {
        let text = include_str!("../example/drilled-plate.process3d");
        let document = Process3dDocument::parse_dsl(text).expect("parse drilled plate example");
        assert_eq!(document.steps.len(), 3);
        assert_eq!(document.resolved_up_to, Some(2));
        test_support::assert_dsl_round_trip(&document);
    }
    //#endregion 🔖DslTests

    //#region 🔖OpTextTests
    #[test]
    fn process3d_op_text_round_trips_steps_add() {
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_remove() {
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "cut-1".into() } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_move() {
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Move { id: "cut-1".into(), to_index: 2 } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_full() {
        let patch = ProcessStepPatch {
            label: Some("Renamed".into()),
            enabled: Some(false),
            measure: Some(ProcessMeasure::Drill { radius: 0.03, depth: 0.4, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } }),
            origin: Some(Some(StepOrigin { module_id: "wood".into(), machine_id: "tableSaw".into(), modification_kind_id: "crosscut".into() })),
        };
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_clearing_origin() {
        let patch = ProcessStepPatch { label: None, enabled: None, measure: None, origin: Some(None) };
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_empty() {
        let patch = ProcessStepPatch::default();
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_set_stock() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetStock { stock: imported_mesh_stock() });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_some() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetCursor { resolved_up_to: Some(3) });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_none() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetCursor { resolved_up_to: None });
    }

    #[test]
    fn process3d_op_text_round_trips_set_document() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetDocument { document: sample_document() });
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None);
        let mut store = Process3dStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![
                    Process3dOperation::SetStock { stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose::default() } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 1, item: drill_step("drill-1") } },
                    Process3dOperation::SetCursor { resolved_up_to: Some(1) },
                ],
                description: Some("build timeline".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }],
            })
            .expect("commit");
        test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests
