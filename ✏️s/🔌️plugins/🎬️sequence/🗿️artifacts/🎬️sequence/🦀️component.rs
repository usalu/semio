//! 🎬️ Sequence artifact — the document entity this plugin's app edits (constitutional: general).

use neural_engine::{Atom, Dictionary, Value};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub const SEQUENCE_DOCUMENT_SCHEMA: &str = "sequence.sequence";
pub use crate::artifacts::sequence::snapshot::schema::{default_snapshot, SequenceSnapshot};

//#region 🔖️Constants
//#endregion 🔖️Constants

//#region 🔖️Domain
/// 📦️ Local newtype around {@link neural_engine::Dictionary} — dynamic/schema-less step params
/// can't be shape-derived field-by-field (arbitrary keys, recursive `Value`), and `Dictionary`
/// itself can't gain a `dsl::DslField` impl directly (foreign trait, foreign type, no local anchor
/// for the orphan rule). Wrapping it as one opaque JSON-text field reuses the exact `serde_json`
/// round trip {@link SequenceHost::to_json}/{@link SequenceHost::load_json} already depend on for
/// fidelity — unlike a schema-less `dsl::Shape::Value`, this never collapses `Atom::Integer` and
/// `Atom::Decimal` into the same wire number. Deliberately `dsl::Shape::Text` (escaped quoted
/// string), NOT `dsl::Shape::Embed("json")` (fenced block): this field is only ever reached as a
/// `#[dsl(table)]` column (`SequenceStep` is `SequenceSnapshotDsl.steps`'s row type), and an
/// `Embed`'s Document-mode fence needs its closing ` ``` ` on its own line — the table row printer
/// glues the remaining row cells (`x y slot collapsed`) onto that same line right after it,
/// producing a fence the lexer can't close and a confirmed parse failure ("unterminated fenced
/// block"). Genuine ENGINE GAP (`Shape::Embed` inside a `Shape::Table` column), out of scope here —
/// verified empirically, not worked around.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StepParams(pub Dictionary);

impl StepParams {
    pub fn new() -> Self {
        Self(Dictionary::new())
    }

    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
        Self(self.0.insert(key, value))
    }
}

impl std::ops::Deref for StepParams {
    type Target = Dictionary;
    fn deref(&self) -> &Dictionary {
        &self.0
    }
}

impl dsl::DslField for StepParams {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(serde_json::to_string(&self.0).unwrap_or_else(|_| "{}".into()))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(text) => serde_json::from_str(text).map(Self).map_err(|err| err.to_string()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🎥️ Camera state for the sequence canvas — the DAG kernel's own `DagCamera` conversions
/// live in the artifact's `⚙️engine` (see its doc comment), not here: `dag`'s `From`/`Into` impls
/// would require this file to depend on the DAG layout kernel just to move a camera in and out,
/// which would pull graph-layout machinery into the plain entity component for no reason a data
/// schema needs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for SequenceCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🎯️ Only ever embedded `#[dsl(block)]`-wrapped (on `SequenceStep::slot`), so it carries no
/// `#[dsl(keyword = "...")]` of its own — the embedding field already supplies the bare `slot`
/// leading keyword.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SlotRef {
    #[dsl(refs = "step")]
    pub owner: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceStep {
    #[dsl(defines = "step")]
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: StepParams,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    #[dsl(block)]
    pub slot: Option<SlotRef>,
    #[serde(default)]
    pub collapsed: bool,
}

/// 🔌️ Runtime edge shape (id/from/to step ids) — kept plain `Serialize`/`Deserialize` only; the
/// `.sequence` DSL text and op-log representations go through the `SequenceEdgeDsl` mirror (see
/// `🗣️dsl`) instead of deriving `dsl::DslRecord` here directly, so this struct (and every consumer
/// matching on `.from`/`.to` — `connect_steps`, `sync_edges_from_dag`, ...) stays untouched by the
/// unified `dsl::Wire` connection syntax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdge {
    pub id: String,
    pub from: String,
    pub to: String,
}
//#endregion 🔖️Domain

//#region 🔖️Collections
impl protocol::Identified<String> for SequenceStep {
    fn id(&self) -> &String {
        &self.id
    }
}

impl protocol::Identified<String> for SequenceEdge {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse patch for a step — only the fields user actions ever mutate after creation (kind/slot
/// are fixed for a step's lifetime, so add/remove carries those instead). Only ever embedded
/// `#[dsl(block)]`-wrapped (on `SequenceMutation::StepsPatch`, in `🔧️op`), so it carries no
/// `#[dsl(keyword = "...")]` of its own.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceStepPatch {
    pub params: Option<StepParams>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub collapsed: Option<bool>,
}

impl protocol::Patchable<SequenceStepPatch> for SequenceStep {
    fn apply_patch(&mut self, patch: &SequenceStepPatch) {
        if let Some(params) = &patch.params {
            self.params = params.clone();
        }
        if let Some(x) = patch.x {
            self.x = x;
        }
        if let Some(y) = patch.y {
            self.y = y;
        }
        if let Some(collapsed) = patch.collapsed {
            self.collapsed = collapsed;
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<SequenceStepPatch> {
        let patch = SequenceStepPatch {
            params: (self.params != other.params).then(|| other.params.clone()),
            x: (self.x != other.x).then_some(other.x),
            y: (self.y != other.y).then_some(other.y),
            collapsed: (self.collapsed != other.collapsed).then_some(other.collapsed),
        };
        (patch != SequenceStepPatch::default()).then_some(patch)
    }
}

/// 🩹️ Sparse patch for an edge endpoint rewire. Only ever embedded `#[dsl(block)]`-wrapped (on
/// `SequenceMutation::EdgesPatch`, in `🔧️op`), so it carries no `#[dsl(keyword = "...")]` of its
/// own.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdgePatch {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl protocol::Patchable<SequenceEdgePatch> for SequenceEdge {
    fn apply_patch(&mut self, patch: &SequenceEdgePatch) {
        if let Some(from) = &patch.from {
            self.from = from.clone();
        }
        if let Some(to) = &patch.to {
            self.to = to.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<SequenceEdgePatch> {
        let patch = SequenceEdgePatch { from: (self.from != other.from).then(|| other.from.clone()), to: (self.to != other.to).then(|| other.to.clone()) };
        (patch != SequenceEdgePatch::default()).then_some(patch)
    }
}
//#endregion 🔖️Collections

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::sequence::create_sequence_app`'s `🔖️Manifest` region. Lifted verbatim out of the
/// old `.artifact_kind(...)` builder call.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.sequence".into(),
        name: "Sequence".into(),
        source_format: "sequence.sequence".into(),
        component_kind: "sequence".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Sequence },
        schema: "sequence.sequence".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_snapshot_has_steps() {
        assert_eq!(default_snapshot().steps.len(), 2);
    }

    #[test]
    fn artifact_kind_keeps_the_media_schema_consistent_with_the_store_schema() {
        assert_eq!(artifact_kind().schema, "sequence.sequence");
        assert_eq!(SEQUENCE_DOCUMENT_SCHEMA, "sequence.sequence");
    }
}
//#endregion 🧪️Tests
