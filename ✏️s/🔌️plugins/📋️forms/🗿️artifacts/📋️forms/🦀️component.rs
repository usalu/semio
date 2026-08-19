//! 📋️ Forms artifact — the document entity this plugin's app edits.
//!
//! Domain step/block/expr types live in the shared `playbook` kernel crate and are re-exported here under
//! forms' historical names. `FormsSnapshot` is defined in `📸️snapshot/🧬️schema` and re-exported here.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use std::cell::RefCell;
use std::collections::HashMap;

//#region 🔖️Types
pub use crate::playbook::{
    PlaybookBlock as FormQuestion, PlaybookBlockOption as FormQuestionOption, PlaybookExpr as FormExpr, PlaybookStep as FormStep, PlaybookValidationError as FormValidationError, PlaybookVectorField as FormVectorField,
    PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS,
};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";
/// 🪪️ This artifact's canonical `(artifact_kind, standard, subset)` coordinate (contract §1) — lives
/// at the ARTIFACT level, not under `editor`/`viewer`, specifically so a viewer file can read it
/// without ever importing through the sibling editor module. `artifact_kind` matches
/// `#[artifact_schema(id = "s.forms.forms")]` on `FormsArtifact` (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const FORMS_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.forms.forms", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub use crate::artifacts::forms::schema::diff::FormsDiff;
pub use crate::artifacts::forms::schema::mutations::FormMutation;
pub use crate::artifacts::forms::schema::snapshot::FormsSnapshot;
//#endregion 🔖️Types

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the document's
/// `steps: Vec<FormStep>` tree (each step's id-keyed `blocks`, each block a `FormQuestion` with
/// 15+ optional config fields plus a recursive `condition` expression tree) is no longer an inline
/// `FormsSnapshot` field — it composes stdio's `s.stdio.semio.value`/`table` subsets as two fixed
/// child slots (`structure`/`results`). `structure` (`value`) is the LOSSLESS source of truth: the
/// full step/block tree folded into one structured `SemioValue::Map`, honestly reflecting that a
/// form question's config (`default`/`params`/`condition`/`options`/`fields`) is exactly
/// "structured/computed values," not prose or a flat table. `results` (`table`) is a DERIVED,
/// non-reconstructive projection — one row per block, flattened in step order (`id`/`stepId`/
/// `label`/`kind`/`required`) — for tabular scan/display convenience; it is always regenerated
/// alongside `structure` from the SAME steps (never an independent source), so the two never
/// diverge. Reconstruction (`forms_steps_from_structure`) reads `structure` only.
///
/// Per this ticket's own corrected precedent (norm/mathematical round 2): composing these two
/// children does NOT regress this plugin's already-granular per-field mutation triads
/// (`create-step`/`delete-step`/`reorder-step`/`rename-step`/`change-step-description`/
/// `create-block`/`delete-block`/`move-block-to-step`/`replace-block`/`change-form-title`) into a
/// whole-blob replace. Every triad's mutation PAYLOAD shape is untouched; `FormsStepsDelta`/
/// `FormsStepPatch` (`🔺️diff/🦀️component.rs`) stay the id-keyed sparse delta types they always
/// were, applied via `apply_steps_delta` against the WORKING-SCENE steps (`forms_steps`, not a
/// snapshot field) — only the diff's own OUTER wire representation of "what changed" becomes a
/// pair of regenerated content-addressed child handles, exactly like every other composed plugin.

//#region 🔖️ChildTypes
pub type FormsStructureChild = store::ArtifactChild<SemioValueSnapshot>;
pub type FormsResultsChild = store::ArtifactChild<SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 `dsl::DslValue` (JSON-equivalent: used by `default`/`params`) <-> `SemioValue` — real,
/// bidirectional. `Bytes`/`Ref` are never produced by `semio_value_from_dsl` (DslValue has no
/// binary/graph-reference primitive), so `dsl_from_semio_value` degrades them to `Null` — a
/// documented gap only reachable if a foreign composer ever wrote a `Bytes`/`Ref` value into this
/// plugin's own `structure` child, never by this plugin's own round trip.
async fn semio_value_from_dsl(value: &dsl::DslValue) -> SemioValue {
    match value {
        dsl::DslValue::Null => SemioValue::Null,
        dsl::DslValue::Bool(v) => SemioValue::Bool { value: *v },
        dsl::DslValue::Number(n) => {
            if n.fract() == 0.0 && n.abs() < 1e15 {
                SemioValue::Int { lexeme: format!("{}", *n as i64) }
            } else {
                SemioValue::Float { lexeme: format!("{n}") }
            }
        }
        dsl::DslValue::String(s) => SemioValue::Str { value: s.clone() },
        dsl::DslValue::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_dsl).collect() },
        dsl::DslValue::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(k, v)| SemioValueEntry { key: k.clone(), value: semio_value_from_dsl(v) }).collect() },
    }
}
async fn dsl_from_semio_value(value: &SemioValue) -> dsl::DslValue {
    match value {
        SemioValue::Null => dsl::DslValue::Null,
        SemioValue::Bool { value } => dsl::DslValue::Bool(*value),
        SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => dsl::DslValue::Number(lexeme.parse().unwrap_or(0.0)),
        SemioValue::Str { value } => dsl::DslValue::String(value.clone()),
        SemioValue::List { items } => dsl::DslValue::Array(items.iter().map(dsl_from_semio_value).collect()),
        SemioValue::Map { entries } => dsl::DslValue::Object(entries.iter().map(|entry| (entry.key.clone(), dsl_from_semio_value(&entry.value))).collect()),
        SemioValue::Bytes { .. } | SemioValue::Ref { .. } => dsl::DslValue::Null,
    }
}

async fn semio_value_map_get<'v>(value: &'v SemioValue, key: &str) -> Option<&'v SemioValue> {
    match value {
        SemioValue::Map { entries } => entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value),
        _ => None,
    }
}
async fn semio_str(value: Option<&SemioValue>) -> Option<String> {
    match value {
        Some(SemioValue::Str { value }) => Some(value.clone()),
        _ => None,
    }
}
async fn semio_bool(value: Option<&SemioValue>) -> Option<bool> {
    match value {
        Some(SemioValue::Bool { value }) => Some(*value),
        _ => None,
    }
}
async fn semio_f64(value: Option<&SemioValue>) -> Option<f64> {
    match value {
        Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().ok(),
        _ => None,
    }
}

/// 🌉 `PlaybookExpr` (the block `condition` recursive boolean tree) <-> `SemioValue` — real,
/// bidirectional; each variant becomes a tagged `Map{kind, ...}`.
async fn semio_value_from_expr(expr: &FormExpr) -> SemioValue {
    match expr {
        FormExpr::Const { value } => SemioValue::Map { entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "const".into() } }, SemioValueEntry { key: "value".into(), value: semio_value_from_dsl(value) }] },
        FormExpr::Var { name } => SemioValue::Map { entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "var".into() } }, SemioValueEntry { key: "name".into(), value: SemioValue::Str { value: name.clone() } }] },
        FormExpr::Eq { left, right } => SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "eq".into() } },
                SemioValueEntry { key: "left".into(), value: semio_value_from_expr(left) },
                SemioValueEntry { key: "right".into(), value: semio_value_from_expr(right) },
            ],
        },
        FormExpr::And { items } => SemioValue::Map {
            entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "and".into() } }, SemioValueEntry { key: "items".into(), value: SemioValue::List { items: items.iter().map(semio_value_from_expr).collect() } }],
        },
        FormExpr::Or { items } => SemioValue::Map {
            entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "or".into() } }, SemioValueEntry { key: "items".into(), value: SemioValue::List { items: items.iter().map(semio_value_from_expr).collect() } }],
        },
        FormExpr::Truthy { expr } => SemioValue::Map { entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "truthy".into() } }, SemioValueEntry { key: "expr".into(), value: semio_value_from_expr(expr) }] },
    }
}
async fn expr_from_semio_value(value: &SemioValue) -> Option<FormExpr> {
    let kind = semio_str(semio_value_map_get(value, "kind"))?;
    match kind.as_str() {
        "const" => Some(FormExpr::Const { value: semio_value_map_get(value, "value").map(dsl_from_semio_value).unwrap_or(dsl::DslValue::Null) }),
        "var" => Some(FormExpr::Var { name: semio_str(semio_value_map_get(value, "name")).unwrap_or_default() }),
        "eq" => {
            let left = expr_from_semio_value(semio_value_map_get(value, "left")?)?;
            let right = expr_from_semio_value(semio_value_map_get(value, "right")?)?;
            Some(FormExpr::Eq { left: Box::new(left), right: Box::new(right) })
        }
        "and" => {
            let items = match semio_value_map_get(value, "items") {
                Some(SemioValue::List { items }) => items.iter().filter_map(expr_from_semio_value).collect(),
                _ => Vec::new(),
            };
            Some(FormExpr::And { items })
        }
        "or" => {
            let items = match semio_value_map_get(value, "items") {
                Some(SemioValue::List { items }) => items.iter().filter_map(expr_from_semio_value).collect(),
                _ => Vec::new(),
            };
            Some(FormExpr::Or { items })
        }
        "truthy" => Some(FormExpr::Truthy { expr: Box::new(expr_from_semio_value(semio_value_map_get(value, "expr")?)?) }),
        _ => None,
    }
}

/// 🌉 One `FormQuestion` (block) <-> a tagged `SemioValue::Map` — every field real, none stubbed.
async fn semio_value_from_block(block: &FormQuestion) -> SemioValue {
    let mut entries = vec![
        SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: block.id.clone() } },
        SemioValueEntry { key: "label".into(), value: SemioValue::Str { value: block.label.clone() } },
        SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: block.kind.clone() } },
    ];
    if let Some(v) = &block.description {
        entries.push(SemioValueEntry { key: "description".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = block.required {
        entries.push(SemioValueEntry { key: "required".into(), value: SemioValue::Bool { value: v } });
    }
    if let Some(v) = &block.placeholder {
        entries.push(SemioValueEntry { key: "placeholder".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.default {
        entries.push(SemioValueEntry { key: "default".into(), value: semio_value_from_dsl(v) });
    }
    if let Some(v) = block.min {
        entries.push(SemioValueEntry { key: "min".into(), value: SemioValue::Float { lexeme: format!("{v}") } });
    }
    if let Some(v) = block.max {
        entries.push(SemioValueEntry { key: "max".into(), value: SemioValue::Float { lexeme: format!("{v}") } });
    }
    if let Some(v) = block.step {
        entries.push(SemioValueEntry { key: "step".into(), value: SemioValue::Float { lexeme: format!("{v}") } });
    }
    if let Some(v) = &block.unit {
        entries.push(SemioValueEntry { key: "unit".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.text {
        entries.push(SemioValueEntry { key: "text".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(options) = &block.options {
        entries.push(SemioValueEntry {
            key: "options".into(),
            value: SemioValue::List {
                items: options
                    .iter()
                    .map(|option| SemioValue::Map {
                        entries: vec![SemioValueEntry { key: "value".into(), value: SemioValue::Str { value: option.value.clone() } }, SemioValueEntry { key: "label".into(), value: SemioValue::Str { value: option.label.clone() } }],
                    })
                    .collect(),
            },
        });
    }
    if let Some(fields) = &block.fields {
        entries.push(SemioValueEntry {
            key: "fields".into(),
            value: SemioValue::List {
                items: fields
                    .iter()
                    .map(|field| {
                        let mut field_entries = vec![SemioValueEntry { key: "key".into(), value: SemioValue::Str { value: field.key.clone() } }];
                        if let Some(label) = &field.label {
                            field_entries.push(SemioValueEntry { key: "label".into(), value: SemioValue::Str { value: label.clone() } });
                        }
                        if let Some(value) = field.value {
                            field_entries.push(SemioValueEntry { key: "value".into(), value: SemioValue::Float { lexeme: format!("{value}") } });
                        }
                        SemioValue::Map { entries: field_entries }
                    })
                    .collect(),
            },
        });
    }
    if let Some(v) = &block.schema {
        entries.push(SemioValueEntry { key: "schema".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.src {
        entries.push(SemioValueEntry { key: "src".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.accept {
        entries.push(SemioValueEntry { key: "accept".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.fixture_slug {
        entries.push(SemioValueEntry { key: "fixtureSlug".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.params {
        entries.push(SemioValueEntry { key: "params".into(), value: semio_value_from_dsl(v) });
    }
    if let Some(v) = &block.condition {
        entries.push(SemioValueEntry { key: "condition".into(), value: semio_value_from_expr(v) });
    }
    SemioValue::Map { entries }
}
async fn block_from_semio_value(value: &SemioValue) -> FormQuestion {
    FormQuestion {
        id: semio_str(semio_value_map_get(value, "id")).unwrap_or_default(),
        label: semio_str(semio_value_map_get(value, "label")).unwrap_or_default(),
        kind: semio_str(semio_value_map_get(value, "kind")).unwrap_or_default(),
        description: semio_str(semio_value_map_get(value, "description")),
        required: semio_bool(semio_value_map_get(value, "required")),
        placeholder: semio_str(semio_value_map_get(value, "placeholder")),
        default: semio_value_map_get(value, "default").map(dsl_from_semio_value),
        min: semio_f64(semio_value_map_get(value, "min")),
        max: semio_f64(semio_value_map_get(value, "max")),
        step: semio_f64(semio_value_map_get(value, "step")),
        unit: semio_str(semio_value_map_get(value, "unit")),
        text: semio_str(semio_value_map_get(value, "text")),
        options: match semio_value_map_get(value, "options") {
            Some(SemioValue::List { items }) => {
                Some(items.iter().map(|item| FormQuestionOption { value: semio_str(semio_value_map_get(item, "value")).unwrap_or_default(), label: semio_str(semio_value_map_get(item, "label")).unwrap_or_default() }).collect())
            }
            _ => None,
        },
        fields: match semio_value_map_get(value, "fields") {
            Some(SemioValue::List { items }) => Some(
                items.iter().map(|item| FormVectorField { key: semio_str(semio_value_map_get(item, "key")).unwrap_or_default(), label: semio_str(semio_value_map_get(item, "label")), value: semio_f64(semio_value_map_get(item, "value")) }).collect(),
            ),
            _ => None,
        },
        schema: semio_str(semio_value_map_get(value, "schema")),
        src: semio_str(semio_value_map_get(value, "src")),
        accept: semio_str(semio_value_map_get(value, "accept")),
        fixture_slug: semio_str(semio_value_map_get(value, "fixtureSlug")),
        params: semio_value_map_get(value, "params").map(dsl_from_semio_value),
        condition: semio_value_map_get(value, "condition").and_then(expr_from_semio_value),
    }
}

/// 🌉 One `FormStep` <-> a tagged `SemioValue::Map` (id/title/description/blocks).
async fn semio_value_from_step(step: &FormStep) -> SemioValue {
    let mut entries = vec![SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: step.id.clone() } }, SemioValueEntry { key: "title".into(), value: SemioValue::Str { value: step.title.clone() } }];
    if let Some(v) = &step.description {
        entries.push(SemioValueEntry { key: "description".into(), value: SemioValue::Str { value: v.clone() } });
    }
    entries.push(SemioValueEntry { key: "blocks".into(), value: SemioValue::List { items: step.blocks.iter().map(semio_value_from_block).collect() } });
    SemioValue::Map { entries }
}
async fn step_from_semio_value(value: &SemioValue) -> FormStep {
    FormStep {
        id: semio_str(semio_value_map_get(value, "id")).unwrap_or_default(),
        title: semio_str(semio_value_map_get(value, "title")).unwrap_or_default(),
        description: semio_str(semio_value_map_get(value, "description")),
        blocks: match semio_value_map_get(value, "blocks") {
            Some(SemioValue::List { items }) => items.iter().map(block_from_semio_value).collect(),
            _ => Vec::new(),
        },
    }
}

/// 🌉 REAL bidirectional converter: the whole `steps` tree <-> one structured `value` Map — the
/// SOLE source of truth for reconstruction (see this region's own doc comment for why `results`
/// is a derived, non-reconstructive projection instead).
pub async fn forms_structure_from_steps(steps: &[FormStep]) -> SemioValueSnapshot {
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map { entries: vec![SemioValueEntry { key: "steps".into(), value: SemioValue::List { items: steps.iter().map(semio_value_from_step).collect() } }] },
        nodes: Vec::new(),
    }
}
pub async fn forms_steps_from_structure(structure: &SemioValueSnapshot) -> Vec<FormStep> {
    match semio_value_map_get(&structure.root, "steps") {
        Some(SemioValue::List { items }) => items.iter().map(step_from_semio_value).collect(),
        _ => Vec::new(),
    }
}

/// 🌉 DERIVED, non-reconstructive projection: one row per block, flattened in step order —
/// "tabular/repeating-row data" for scan/display, always regenerated alongside `structure` from
/// the SAME steps (see this region's own doc comment).
pub async fn forms_results_from_steps(steps: &[FormStep]) -> SemioTableSnapshot {
    let mut rows = Vec::new();
    for step in steps {
        for block in &step.blocks {
            rows.push(SemioTableRow {
                cells: vec![
                    SemioValue::Str { value: block.id.clone() },
                    SemioValue::Str { value: step.id.clone() },
                    SemioValue::Str { value: block.label.clone() },
                    SemioValue::Str { value: block.kind.clone() },
                    match block.required {
                        Some(v) => SemioValue::Bool { value: v },
                        None => SemioValue::Null,
                    },
                ],
            });
        }
    }
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![
            SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "stepId".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "label".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "kind".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "required".into(), kind: SemioTableCellKind::Bool },
        ],
        rows,
    }
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side cache of the live `steps` tree behind a `(structure, results)` child
/// pair — NEVER persisted (matches the `EngineRep` contract: wholly derived, droppable at any
/// instant, rebuilt from base). No `LinkResolver`/child-dispatch seam exists in
/// `ArtifactApp::handle` yet (checked directly against `🔌️plugin/🦀️component.rs`, W1-owned,
/// read-only — same standing gap every prior wave's report documents), so this is the only way a
/// persisted content-addressed handle round-trips to the real steps tree within one process —
/// mirrors mathematical's `MATH_SCRATCH`/writer's `WRITER_SCRATCH`. `structure`/`results` are
/// always minted TOGETHER from the same `steps` (`forms_children_from_steps`), so
/// `structure.child_id == results.child_id` and one cache entry serves both reads.
///
/// ⚠️ Same documented staleness gap as every prior exemplar: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, so a handle can in principle go uncached (fresh process, or an
/// undo past this session's history). `forms_steps` fails soft (empty `Vec`) rather than panicking.
pub struct FormsWorkingScene {
    pub steps: Vec<FormStep>,
}

thread_local! {
    static FORMS_SCRATCH: RefCell<HashMap<String, FormsWorkingScene>> = RefCell::new(HashMap::new());
}

async fn forms_scene_id(steps: &[FormStep]) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(steps).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("forms-scene-{:016x}", hasher.finish())
}

/// 🏗️ Mints both composed-child handles for a `steps` tree AND seeds the scratch cache in one
/// call — the standard way every mutation-diff/fixture builder in this plugin creates
/// `structure`/`results` field values; never construct these handles without also caching, or
/// `forms_steps` will read back empty.
pub async fn forms_children_from_steps(steps: &[FormStep]) -> (FormsStructureChild, FormsResultsChild) {
    let scene_id = forms_scene_id(steps);
    FORMS_SCRATCH.with(|cache| {
        cache.borrow_mut().insert(scene_id.clone(), FormsWorkingScene { steps: steps.to_vec() });
    });
    let dialect_for = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| store::os_io::ArtifactRef { artifact_id: format!("forms-{subset}"), dialect: dialect_for(subset) };
    (store::ArtifactChild::new(scene_id.clone(), target_for("value")), store::ArtifactChild::new(scene_id, target_for("table")))
}

/// 🔎 Reads the cached working scene behind a snapshot's composed children — an empty `steps`
/// (never a panic) on a cache miss, per this region's own doc comment.
pub async fn forms_scene(snapshot: &FormsSnapshot) -> FormsWorkingScene {
    FORMS_SCRATCH.with(|cache| cache.borrow().get(&snapshot.structure.child_id).map(|scene| FormsWorkingScene { steps: scene.steps.clone() })).unwrap_or_else(|| FormsWorkingScene { steps: Vec::new() })
}

/// 🔎 The live `steps` tree behind a snapshot's composed children — the single read call site
/// every render/inference/export/command path in this plugin now uses instead of the old `.steps`
/// field.
pub async fn forms_steps(snapshot: &FormsSnapshot) -> Vec<FormStep> {
    forms_scene(snapshot).steps
}

/// 🔎 Twin of [`forms_steps`] for the UI-inclusive [`crate::artifacts::forms::schema::FormsArtifact`]
/// (its own `structure`/`results` fields mirror the snapshot's — see that struct's own doc).
pub async fn forms_artifact_steps(artifact: &crate::artifacts::forms::schema::FormsArtifact) -> Vec<FormStep> {
    FORMS_SCRATCH.with(|cache| cache.borrow().get(&artifact.structure.child_id).map(|scene| scene.steps.clone())).unwrap_or_default()
}

/// 🏗️ Builds a full `FormsSnapshot` from a literal `steps` tree — the standard fixture/import
/// constructor replacing the old struct literal with an inline `steps: Vec<FormStep>` field.
pub async fn forms_snapshot_with_state(schema: String, id: String, version: String, title: Option<String>, steps: Vec<FormStep>) -> FormsSnapshot {
    let (structure, results) = forms_children_from_steps(&steps);
    FormsSnapshot { schema, id, version, title, structure, results }
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::forms::create_forms_app`'s `🔖️Manifest` region.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "form.dictionary".into(),
        name: "Form Dictionary".into(),
        source_format: "form.dictionary".into(),
        component_kind: "forms".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: "form.dictionary".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1)
/// — kept per debt D1 (`📌️important.md`): not deleted repo-wide until W6. Ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM's `artifact()` below is the sole LIVE
/// registration channel (`.declare_artifact(...)`, plugin root); `definition()` has zero callers
/// left this pass (harmless — it still carries the real en/de localized names the new
/// `ArtifactDeclaration.localization` field does not yet populate, see that field's own doc and
/// `## openQuestions`). The old side-effecting `pilot_languages()`/`declaration()` pair (which
/// wired this artifact's hand-authored grammars into `dsl::register_languages` via the now-deleted
/// `.artifact(...)`/`.languages(...)` chain) is deleted outright, not kept: `NativeCodecs`'s
/// `LanguagePair` fields stay `{ text: None, binary: None }` (`🚪️io/🦀️component.rs`), the same
/// documented deferral every other subset on this ticket carries — the underlying grammar/protocol
/// `.semio` assets themselves are untouched and still compiled into their own facet files.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.forms.standard.v1", "standard", "1", &[], None),
        ("s.forms.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.forms.schema.artifact", "schema", "s.forms.forms", &[("schema", "s.forms.forms")], None),
        ("s.forms.inference.artifact", "inference", "s.forms.forms.inference", &[("schema", "s.forms.forms.inference")], None),
        ("s.forms.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.forms.grammar.document", "grammar", "forms.forms", &[("grammar", "forms.forms")], None),
        ("s.forms.grammar.op", "grammar", "forms.forms.op", &[("grammar", "forms.forms.op")], None),
        ("s.forms.grammar.diff", "grammar", "forms.forms.diff", &[("grammar", "forms.forms.diff")], None),
        ("s.forms.grammar.pack", "grammar", "forms.pack", &[("grammar", "forms.pack")], None),
        ("s.forms.grammar.spr", "grammar", "forms.spr", &[("grammar", "forms.spr")], None),
        ("s.forms.codec.document.v1", "codec", "forms.form:forms", &[("codec", "forms.form"), ("extension", "forms")], None),
        ("s.forms.localization.en", "localization", "Forms", &[], Some(("en", "Forms"))),
        ("s.forms.localization.de", "localization", "Formulare", &[], Some(("de", "Formulare"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.forms")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ This artifact's declaration tree root (design.md §1/§2) — ONE standard (`1`), ONE subset
/// (`any`). Sole registration channel (plugin root `.declare_artifact(artifact())`); the old
/// `.artifact(declaration())`/`.editor::<>()`/`.viewer::<>()` triad is deleted, not kept alongside
/// it (a second parallel registration channel is the compatibility layer CLAUDE.md forbids).
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;

    ArtifactDeclaration { kind: ArtifactKindId::parse("s.forms.forms").expect("canonical forms kind"), localization: &[], standards: vec![crate::artifacts::forms::standards::v1::standard()] }
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn artifact_kind_uses_the_dictionary_media_kind_as_both_id_and_schema() {
        assert_eq!(artifact_kind().id, "form.dictionary");
        assert_eq!(artifact_kind().schema, "form.dictionary");
        assert_eq!(FORMS_DOCUMENT_SCHEMA, "forms.form");
    }

    #[test]
    async fn question_fields_roundtrip() {
        let json = r#"{
            "id":"q1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
        let question: FormQuestion = serde_json::from_str(json).expect("question json");
        assert_eq!(question.min, Some(1.0));
        assert_eq!(question.unit.as_deref(), Some("people"));
        assert!(question.required.unwrap_or(false));
    }
}
//#endregion 🧪️Tests
