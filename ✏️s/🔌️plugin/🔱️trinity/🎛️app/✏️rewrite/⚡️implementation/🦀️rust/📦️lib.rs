//! ♻️ Trinity Rewrite app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use trinity_ram::PropertyValue;

//#region ⚠️ Errors
/// ⚠️ Trinity rewrite-engine errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRewriteError {
    /// 🧩️ Trinity graph fixture load/validation/mutation failure.
    #[error(transparent)]
    Graph(#[from] trinity_ram::TrinityRamError),
    /// 🧭️ VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 🧬️ JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🔤️ Jack query parse/execute failure (`trinity_jack`'s own API is not yet thiserror-migrated).
    #[error("{0}")]
    Jack(String),
    /// 📐️ Force-directed layout failure (`infinite_board_port_directed`'s own API is not yet thiserror-migrated).
    #[error("{0}")]
    Layout(String),
    /// 🎨️ Canvas theme merge failure (`infinite_board_port_directed`'s own API is not yet thiserror-migrated).
    #[error("{0}")]
    CanvasTheme(String),
    #[error("force layout fixture missing nodes")]
    ForceLayoutFixtureMissingNodes,
}
//#endregion ⚠️ Errors

//#region 🔖️Types
/// 📍️ Local `{x, y}` twin for a bare `(f64, f64)` tuple — the DSL engine's `DslField` binding has no
/// impl for raw Rust tuples (only named `DslRecord`/`DslScalar` types can bind), so `rule_layout`'s
/// value type is this named record instead, with `From`/`Into` conversions at this crate's own
/// remaining `(f64, f64)` call sites (tests only — no production logic reads `rule_layout` today).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for LayoutPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<LayoutPoint> for (f64, f64) {
    fn from(point: LayoutPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📐️ The full rewrite-rule document: before fixture, LHS/RHS patterns, parameter bindings, and
/// rule-graph layout overrides. Every field binds directly through the `dsl::` engine: `rhs_json` is
/// `#[dsl(lang = "json")]` (`Shape::Embed("json")`) so its pretty-printed JSON blob prints as a fenced
/// verbatim block instead of an escaped quoted string — `before_fixture_json`/`lhs_json` deliberately
/// stay plain `Shape::Text` (bare `String`, no attribute): the engine's `Lines`-layout printer defers
/// EVERY `Shape::Embed` field to print after all non-embed fields, in declaration order among
/// themselves, with no separator forced between consecutive embed fields, so annotating more than one
/// `_json` field here glues one field's closing fence to the next field's key on the same text line
/// and breaks the fence lexer's "closing ``` must be alone on its line" rule — confirmed by a failing
/// round-trip test, reverted to only the last field. `parameter_bindings` is `BTreeMap<String,
/// PropertyValue>` (bare `HashMap` has no blanket `DslField` impl, only `BTreeMap` does, and nothing
/// here relies on `HashMap`'s unordered iteration), and `rule_layout` uses the `LayoutPoint` twin
/// above in place of a bare tuple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "rewrite", layout = "lines")]
pub struct RewriteRuleState {
    pub before_fixture_json: String,
    pub lhs_json: String,
    #[dsl(lang = "json")]
    pub rhs_json: String,
    #[serde(default)]
    pub parameter_bindings: BTreeMap<String, PropertyValue>,
    #[serde(default)]
    pub rule_layout: BTreeMap<String, LayoutPoint>,
}

pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";
//#endregion 🔖️Types

//#region 🔖️Dsl
// 📜️ `RewriteRuleState`/`RewriteRuleOperation` derive their `store::DocumentDsl`/`protocol::OpText`
// impls directly (see `#[derive(dsl::DslDocument)]` here and `#[derive(dsl::DslOps)]` in the `op`
// crate) — every field already binds through the `dsl::` engine with no foreign types, so no
// hand-written parser/printer or twin type is needed anywhere in this app.
//#endregion 🔖️Dsl
