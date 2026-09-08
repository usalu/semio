//! 📋️ Forms artifact — the document entity this plugin's app edits.
//!
//! Domain step/block/expr types live in the shared `playbook` kernel crate and are re-exported here under
//! forms' historical names. `FormsSnapshot` is defined in `📸️snapshot/🧬️schema` and re-exported here.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
mod art_forms_demo_tests;
extern crate semio_framework_schema as framework_schema;
use semio_framework_artifact_playbook_playbook as playbook;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FormMutation, FormsConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
extern crate self as semio_s_artifact_forms_forms;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};

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
pub const FORMS_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.forms.forms", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub use crate::schema::diff::FormsDiff;
pub use crate::schema::mutations::FormMutation;
pub use crate::schema::snapshot::FormsSnapshot;
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
/// `FormsStepPatch` (`🔺️diff/🦀️.rs`) stay the id-keyed sparse delta types they always
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
fn semio_value_from_dsl(value: &dsl::DslValue) -> SemioValue {
    match value {
        dsl::DslValue::Null => SemioValue::Null,
        dsl::DslValue::Bool(v) => SemioValue::Bool { value: *v },
        dsl::DslValue::Number(n) => match n {
            dsl::Number::UInt(v) => SemioValue::Int { lexeme: v.to_string() },
            dsl::Number::Int(v) => SemioValue::Int { lexeme: v.to_string() },
            dsl::Number::Float(v) if v.fract() == 0.0 && v.abs() < 1e15 => SemioValue::Int { lexeme: format!("{}", *v as i64) },
            dsl::Number::Float(v) => SemioValue::Float { lexeme: format!("{v}") },
        },
        dsl::DslValue::String(s) => SemioValue::Str { value: s.clone() },
        dsl::DslValue::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_dsl).collect() },
        dsl::DslValue::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(k, v)| SemioValueEntry { key: k.clone(), value: semio_value_from_dsl(v) }).collect() },
    }
}
fn dsl_from_semio_value(value: &SemioValue) -> dsl::DslValue {
    match value {
        SemioValue::Null => dsl::DslValue::Null,
        SemioValue::Bool { value } => dsl::DslValue::Bool(*value),
        SemioValue::Int { lexeme } => dsl::DslValue::int(lexeme.parse().unwrap_or(0)),
        SemioValue::Float { lexeme } => dsl::DslValue::float(lexeme.parse().unwrap_or(0.0)),
        SemioValue::Str { value } => dsl::DslValue::String(value.clone()),
        SemioValue::List { items } => dsl::DslValue::Array(items.iter().map(dsl_from_semio_value).collect()),
        SemioValue::Map { entries } => dsl::DslValue::Object(entries.iter().map(|entry| (entry.key.clone(), dsl_from_semio_value(&entry.value))).collect()),
        SemioValue::Bytes { .. } | SemioValue::Ref { .. } => dsl::DslValue::Null,
    }
}

fn semio_value_map_get<'v>(value: &'v SemioValue, key: &str) -> Option<&'v SemioValue> {
    match value {
        SemioValue::Map { entries } => entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value),
        _ => None,
    }
}
fn semio_str(value: Option<&SemioValue>) -> Option<String> {
    match value {
        Some(SemioValue::Str { value }) => Some(value.clone()),
        _ => None,
    }
}
fn semio_bool(value: Option<&SemioValue>) -> Option<bool> {
    match value {
        Some(SemioValue::Bool { value }) => Some(*value),
        _ => None,
    }
}
fn semio_f64(value: Option<&SemioValue>) -> Option<f64> {
    match value {
        Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().ok(),
        _ => None,
    }
}

/// 🌉 `PlaybookExpr` (the block `condition` recursive boolean tree) <-> `SemioValue` — real,
/// bidirectional; each variant becomes a tagged `Map{kind, ...}`.
fn semio_value_from_expr(expr: &FormExpr) -> SemioValue {
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
fn expr_from_semio_value(value: &SemioValue) -> Option<FormExpr> {
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
fn semio_value_from_block(block: &FormQuestion) -> SemioValue {
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
fn block_from_semio_value(value: &SemioValue) -> FormQuestion {
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
fn semio_value_from_step(step: &FormStep) -> SemioValue {
    let mut entries = vec![SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: step.id.clone() } }, SemioValueEntry { key: "title".into(), value: SemioValue::Str { value: step.title.clone() } }];
    if let Some(v) = &step.description {
        entries.push(SemioValueEntry { key: "description".into(), value: SemioValue::Str { value: v.clone() } });
    }
    entries.push(SemioValueEntry { key: "blocks".into(), value: SemioValue::List { items: step.blocks.iter().map(semio_value_from_block).collect() } });
    SemioValue::Map { entries }
}
fn step_from_semio_value(value: &SemioValue) -> FormStep {
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
pub fn forms_structure_from_steps(steps: &[FormStep]) -> SemioValueSnapshot {
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map { entries: vec![SemioValueEntry { key: "steps".into(), value: SemioValue::List { items: steps.iter().map(semio_value_from_step).collect() } }] },
        nodes: Vec::new(),
    }
}
pub fn forms_steps_from_structure(structure: &SemioValueSnapshot) -> Vec<FormStep> {
    match semio_value_map_get(&structure.root, "steps") {
        Some(SemioValue::List { items }) => items.iter().map(step_from_semio_value).collect(),
        _ => Vec::new(),
    }
}

/// 🌉 DERIVED, non-reconstructive projection: one row per block, flattened in step order —
/// "tabular/repeating-row data" for scan/display, always regenerated alongside `structure` from
/// the SAME steps (see this region's own doc comment).
pub fn forms_results_from_steps(steps: &[FormStep]) -> SemioTableSnapshot {
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
/// 🌱 Ephemeral representation of one composed child's live step tree. The value belongs to
/// the exact `ArtifactChild`; it is never persisted, never process-global, and retires with that
/// owner. Equal wire identities cannot observe one another's materialization.
#[derive(Clone, Debug, Default)]
pub struct FormsWorkingScene {
    pub steps: Vec<FormStep>,
}

fn forms_scene_id(steps: &[FormStep]) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = dsl::os_pack::json::to_json_string(&steps.to_vec());
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("forms-scene-{:016x}", hasher.finish())
}

/// 📝 Transfers decoded or test-provided steps into one exact structure-child owner.
pub fn materialize_forms_steps(handle: &mut FormsStructureChild, steps: Vec<FormStep>) {
    handle.set_local_owner(std::sync::Arc::new(FormsWorkingScene { steps }));
}

/// 🏗️ Mints both composed-child handles and transfers the same immutable materialization
/// into each exact owner. The shared `Arc` is scoped to this returned pair, never to wire identity.
pub fn forms_children_from_steps(steps: &[FormStep]) -> (FormsStructureChild, FormsResultsChild) {
    let scene_id = forms_scene_id(steps);
    let dialect_for = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| store::os_io::ArtifactRef { artifact_id: format!("forms-{subset}"), dialect: dialect_for(subset) };
    let scene = std::sync::Arc::new(FormsWorkingScene { steps: steps.to_vec() });
    (
        store::ArtifactChild::new(scene_id.clone(), target_for("value")).with_local_owner(scene.clone()),
        store::ArtifactChild::new(scene_id, target_for("table")).with_local_owner(scene),
    )
}

/// 🔎 Reads the materialization owned by this snapshot's exact structure child. A wire-only
/// handle fails soft until the host materializes its child document.
pub fn forms_scene(snapshot: &FormsSnapshot) -> FormsWorkingScene {
    snapshot.structure.local_owner::<FormsWorkingScene>().map(|scene| scene.as_ref().clone()).unwrap_or_default()
}

/// 🔎 The live `steps` tree behind a snapshot's composed children — the single read call site
/// every render/inference/export/command path in this plugin now uses instead of the old `.steps`
/// field.
pub fn forms_steps(snapshot: &FormsSnapshot) -> Vec<FormStep> {
    forms_scene(snapshot).steps
}

/// 🔎 Twin of [`forms_steps`] for the UI-inclusive [`crate::schema::FormsArtifact`]
/// (its own `structure`/`results` fields mirror the snapshot's — see that struct's own doc).
pub fn forms_artifact_steps(artifact: &crate::schema::FormsArtifact) -> Vec<FormStep> {
    artifact.structure.local_owner::<FormsWorkingScene>().map(|scene| scene.steps.clone()).unwrap_or_default()
}

/// 🏗️ Builds a full `FormsSnapshot` from a literal `steps` tree — the standard fixture/import
/// constructor replacing the old struct literal with an inline `steps: Vec<FormStep>` field.
pub fn forms_snapshot_with_state(schema: String, id: String, version: String, title: Option<String>, steps: Vec<FormStep>) -> FormsSnapshot {
    let (structure, results) = forms_children_from_steps(&steps);
    FormsSnapshot { schema, id, version, title, structure, results }
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::forms::create_forms_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
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
/// `LanguagePair` fields stay `{ text: None, binary: None }` (`🚪️io/🦀️.rs`), the same
/// documented deferral every other subset on this ticket carries — the underlying grammar/protocol
/// `.semio` assets themselves are untouched and still compiled into their own facet files.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.forms.forms.standard.v1", "standard", "1", &[], None),
        ("s.forms.forms.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.forms.forms.schema.artifact", "schema", "s.forms.forms", &[("schema", "s.forms.forms")], None),
        ("s.forms.forms.inference.artifact", "inference", "s.forms.forms.inference", &[("schema", "s.forms.forms.inference")], None),
        ("s.forms.forms.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.forms.forms.grammar.document", "grammar", "forms.forms", &[("grammar", "forms.forms")], None),
        ("s.forms.forms.grammar.op", "grammar", "forms.forms.op", &[("grammar", "forms.forms.op")], None),
        ("s.forms.forms.grammar.diff", "grammar", "forms.forms.diff", &[("grammar", "forms.forms.diff")], None),
        ("s.forms.forms.grammar.pack", "grammar", "forms.pack", &[("grammar", "forms.pack")], None),
        ("s.forms.forms.grammar.spr", "grammar", "forms.spr", &[("grammar", "forms.spr")], None),
        ("s.forms.forms.codec.document.v1", "codec", "forms.form:forms", &[("codec", "forms.form"), ("codec-extension", "10:forms.form:forms")], None),
        ("s.forms.forms.localization.en", "localization", "Forms", &[], Some(("en", "Forms"))),
        ("s.forms.forms.localization.de", "localization", "Formulare", &[], Some(("de", "Formulare"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.forms.forms")?);
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
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::FormsApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;

    ArtifactDeclaration { kind: ArtifactKindId::parse("s.forms.forms").expect("canonical forms kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod move_block_to_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/🧪️tests/🧩️no-ops-when-the-55392a/🦀️.rs"]
                                    mod tests_no_ops_when_the_block_stays_at_its_index_in_its_own_step;
                                }
                                #[path = "."]
                                pub mod reorder_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/🧪️tests/🚪️no-ops-when-the-step-d924f1/🦀️.rs"]
                                    mod tests_no_ops_when_the_step_already_sits_at_that_index;
                                }
                                #[path = "."]
                                pub mod create_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/🧪️tests/🧩️rejects-a-block-for-a-5cc5b4/🦀️.rs"]
                                    mod tests_rejects_a_block_for_a_step_that_does_not_exist;
                                }
                                #[path = "."]
                                pub mod create_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/🚫️rejects-a-duplicate-step-id/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_step_id;
                                }
                                #[path = "."]
                                pub mod delete_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/🧪️tests/🧩️rejects-deleting-a-25d9e0/🦀️.rs"]
                                    mod tests_rejects_deleting_a_block_missing_from_an_existing_step;
                                }
                                #[path = "."]
                                pub mod delete_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/🚫️rejects-deleting-a-5162d3/🦀️.rs"]
                                    mod tests_rejects_deleting_a_step_the_scene_does_not_hold;
                                }
                                #[path = "."]
                                pub mod change_form_title {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/🧪️tests/⚓️titles-an-a340b6/🦀️.rs"]
                                    mod tests_titles_an_untitled_survey;
                                }
                                #[path = "."]
                                pub mod replace_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🧪️tests/🧩️no-ops-when-the-563fec/🦀️.rs"]
                                    mod tests_no_ops_when_the_replacement_block_is_identical;
                                }
                                #[path = "."]
                                pub mod change_step_description {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/🧪️tests/📖️no-ops-when-669699/🦀️.rs"]
                                    mod tests_no_ops_when_clearing_an_already_absent_description;
                                }
                                #[path = "."]
                                pub mod rename_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/🧪️tests/📖️no-ops-when-the-step-af133f/🦀️.rs"]
                                    mod tests_no_ops_when_the_step_already_carries_that_title;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::io::mutations::text::*;
            pub use crate::standards::v1::subsets::any::schema::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❓️add-question/🦀️.rs"]
            pub mod add_question;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔘️add-question-option/🦀️.rs"]
            pub mod add_question_option;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📃️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️add-vector-field/🦀️.rs"]
            pub mod add_vector_field;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️drop-question-kind/🦀️.rs"]
            pub mod drop_question_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs"]
            pub mod export_fixture;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-question/🦀️.rs"]
            pub mod move_question;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️next-step/🦀️.rs"]
            pub mod next_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️patch-question-options/🦀️.rs"]
            pub mod patch_question_options;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🦀️.rs"]
            pub mod patch_questions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-step/🦀️.rs"]
            pub mod patch_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩺️patch-vector-field/🦀️.rs"]
            pub mod patch_vector_field;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/◀️previous-step/🦀️.rs"]
            pub mod previous_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️remove-question/🦀️.rs"]
            pub mod remove_question;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⭕️remove-question-option/🦀️.rs"]
            pub mod remove_question_option;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚫️remove-vector-field/🦀️.rs"]
            pub mod remove_vector_field;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️reset-try/🦀️.rs"]
            pub mod reset_try;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔣️set-spec-json/🦀️.rs"]
            pub mod set_spec_json;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs"]
            pub mod set_try_value;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📍️set-try-value-step/🦀️.rs"]
            pub mod set_try_value_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs"]
            pub mod set_try_values;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🦀️.rs"]
            pub mod submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️update-form/🦀️.rs"]
            pub mod update_form;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🦀️.rs"]
                    pub mod builder;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs"]
                    pub mod try_wizard;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs"]
                    pub mod try_wizard;
                }
            }
        }
    }
}
