//! 🧩️ Sequence app — document entities (constitutional: general).

use neural_engine::{Atom, Dictionary, Value};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const SEQUENCE_FIXTURE_SCHEMA: &str = "sequence.fixture";
//#endregion 🔖️Constants

//#region 🔖️Fixture
/// 📦️ Local newtype around {@link neural_engine::Dictionary} — dynamic/schema-less step params
/// can't be shape-derived field-by-field (arbitrary keys, recursive `Value`), and `Dictionary`
/// itself can't gain a `dsl::DslField` impl directly (foreign trait, foreign type, no local anchor
/// for the orphan rule). Wrapping it as one opaque JSON-text field reuses the exact `serde_json`
/// round trip {@link SequenceHost::to_json}/{@link SequenceHost::load_json} already depend on for
/// fidelity — unlike a schema-less `dsl::Shape::Value`, this never collapses `Atom::Integer` and
/// `Atom::Decimal` into the same wire number. Deliberately `dsl::Shape::Text` (escaped quoted
/// string), NOT `dsl::Shape::Embed("json")` (fenced block): this field is only ever reached as a
/// `#[dsl(table)]` column (`SequenceStep` is `SequenceFixtureDsl.steps`'s row type), and an
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
/// live in `sequence_engine` (see its doc comment), not here: `dag`'s `From`/`Into` impls would
/// require this crate to depend on the DAG layout kernel just to move a camera in and out, which
/// would pull graph-layout machinery into the plain entity crate for no reason a data schema needs.
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
/// `🔖️Dsl`) instead of deriving `dsl::DslRecord` here directly, so this struct (and every consumer
/// matching on `.from`/`.to` — `connect_steps`, `sync_edges_from_dag`, ...) stays untouched by the
/// unified `dsl::Wire` connection syntax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdge {
    pub id: String,
    pub from: String,
    pub to: String,
}

/// 🧾️ Runtime fixture shape — kept plain `Serialize`/`Deserialize` only; see `SequenceFixtureDsl`
/// (`🔖️Dsl` region) for the `.sequence` DSL text mirror (SoA `steps`/`edges` tables, `edges` as
/// `dsl::Wire` links) and the hand-written `impl store::DocumentDsl for SequenceFixture` that
/// converts through it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFixture {
    pub schema: String,
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

impl Default for SequenceFixture {
    fn default() -> Self {
        default_fixture()
    }
}

pub fn default_fixture() -> SequenceFixture {
    SequenceFixture {
        schema: "sequence.fixture".into(),
        steps: vec![
            SequenceStep {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: StepParams::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Decimal(0.0))),
                x: 0.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
            SequenceStep { id: "step-2".into(), kind: "log.print".into(), params: StepParams::new().insert("message", Value::Atom(Atom::String("hello sequence".into()))), x: 280.0, y: 0.0, slot: None, collapsed: false },
        ],
        edges: vec![SequenceEdge { id: "edge-1".into(), from: "step-1".into(), to: "step-2".into() }],
    }
}
//#endregion 🔖️Fixture

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
/// `#[dsl(block)]`-wrapped (on `SequenceOperation::StepsPatch`, in `sequence_op`), so it carries no
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
/// `SequenceOperation::EdgesPatch`, in `sequence_op`), so it carries no `#[dsl(keyword = "...")]` of
/// its own.
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

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `SequenceEdge` — models the `from`/`to` step-id pair as a single unified
/// `dsl::Wire` literal (`from->to`) instead of two separate string fields, per the unified syntax
/// law for graph edges/connections. Converts at the `store::DocumentDsl`/`protocol::OpText` boundary
/// only (`sequence_fixture_to_dsl` here and `sequence_op`'s `sequence_operation_to_dsl`, and their
/// inverses); `SequenceEdge` itself (and every consumer matching on its `from`/`to` fields directly)
/// is completely untouched. `SequenceEdgePatch` stays a plain sparse two-`Option<String>` patch
/// rather than a `Wire` — a `Wire`'s two endpoints are not independently optional, but `EdgesPatch`
/// legitimately needs to rewire only `from` OR only `to`. `pub` (unlike the document-only
/// `SequenceFixtureDsl` below) because `sequence_op`'s `SequenceOperationDsl::EdgesAdd` embeds it too.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct SequenceEdgeDsl {
    pub id: String,
    pub link: dsl::Wire,
}

pub fn sequence_edge_to_dsl(edge: &SequenceEdge) -> SequenceEdgeDsl {
    let from = dsl::WireNode { id: edge.from.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.to.clone(), kind: None, port: None };
    SequenceEdgeDsl { id: edge.id.clone(), link: dsl::Wire(dsl::WireValue { from, edge: Some((true, to)), properties: dsl::DslValue::Object(Vec::new()) }) }
}

pub fn sequence_edge_from_dsl(edge: SequenceEdgeDsl) -> Result<SequenceEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.link.0;
    let (directed, to) = link.ok_or_else(|| "sequence edge wire literal must have a target".to_string())?;
    if !directed {
        return Err("sequence edge wire literal must be directed".into());
    }
    Ok(SequenceEdge { id: edge.id, from: from.id, to: to.id })
}

/// 📄️ DSL-only mirror of `SequenceFixture` — `steps`/`edges` print as SoA `#[dsl(table)]` columns
/// instead of the old array-of-structures form, and `edges` goes through `SequenceEdgeDsl` for the
/// unified wire syntax. See this region's opening doc comment on `SequenceEdgeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "sequence")]
#[dsl(layout = "lines")]
struct SequenceFixtureDsl {
    schema: String,
    #[dsl(table)]
    steps: Vec<SequenceStep>,
    #[dsl(table)]
    edges: Vec<SequenceEdgeDsl>,
}

fn sequence_fixture_to_dsl(fixture: &SequenceFixture) -> SequenceFixtureDsl {
    SequenceFixtureDsl { schema: fixture.schema.clone(), steps: fixture.steps.clone(), edges: fixture.edges.iter().map(sequence_edge_to_dsl).collect() }
}

fn sequence_fixture_dsl_to_fixture(fixture: SequenceFixtureDsl) -> Result<SequenceFixture, String> {
    Ok(SequenceFixture { schema: fixture.schema, steps: fixture.steps, edges: fixture.edges.into_iter().map(sequence_edge_from_dsl).collect::<Result<Vec<_>, _>>()? })
}

impl store::DocumentDsl for SequenceFixture {
    const EXTENSION: &'static str = "sequence";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_fixture = <SequenceFixtureDsl as store::DocumentDsl>::parse_dsl(text)?;
        sequence_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <SequenceFixtureDsl as store::DocumentDsl>::print_dsl(&sequence_fixture_to_dsl(self))
    }
}

/// 📦️ Hand-written `store::DocumentPack` mirror of the `DocumentDsl` impl above — `SequenceFixture`
/// itself doesn't derive `dsl::DslDocument` (see `SequenceFixtureDsl`'s doc comment), so it doesn't
/// pick up the blanket derive-emitted `DocumentPack` impl either; this converts through the same
/// `SequenceFixtureDsl` mirror, which does derive it.
impl store::DocumentPack for SequenceFixture {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <SequenceFixtureDsl as store::DocumentPack>::encode_pack_with(&sequence_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let dsl_fixture = <SequenceFixtureDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        sequence_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️Dsl
