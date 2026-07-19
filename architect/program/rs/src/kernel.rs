//! 🧱 Shared kernel types for architect program entities — ids, headers, quantities, traces, and diagnostics.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

// #region 🔖EntityId
static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Stable string identity for any program entity or register row.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityId(pub String);

impl EntityId {
    /// @emoji 🔢 Allocates the next serial id under `prefix` (e.g. `element-1`).
    pub fn new_serial(prefix: &str) -> Self {
        let n = ID_COUNTER.fetch_add(1, AtomicOrdering::Relaxed) + 1;
        Self(format!("{prefix}-{n}"))
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Ord for EntityId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for EntityId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// #endregion

// #region 🔖Priority
/// @emoji 🎚️ Relative importance band for requirements, relationships, and entities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
  Mandatory,
  Essential,
  #[default]
  Preferred,
  Optional,
  Deferred,
  Prohibited,
}
// #endregion

// #region 🔖LifecycleStatus
/// @emoji 🔄 Lifecycle and workflow status for register entities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LifecycleStatus {
  #[default]
  Draft,
  Proposed,
  UnderReview,
  Validated,
  Approved,
  Rejected,
  Deferred,
  Superseded,
  Archived,
  Open,
  Closed,
  AtRisk,
  Blocked,
  InProgress,
  Complete,
}
// #endregion

// #region 🔖Ownership
/// @emoji 👥 Ownership and authority roles attached to an entity header.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ownership {
    pub owner_id: Option<EntityId>,
    pub authority_id: Option<EntityId>,
    pub consultant_ids: Vec<EntityId>,
    pub participant_ids: Vec<EntityId>,
}
// #endregion

// #region 🔖Text
/// @emoji 📝 Rich or plain text payload with optional format hint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextField {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl TextField {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            format: None,
        }
    }
}

/// @emoji 🏷️ Tagged free-text note on an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaggedNote {
    pub tag: String,
    pub text: String,
}

/// @emoji 🕒 Created/updated audit timestamps on an entity header.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampMeta {
    pub created: String,
    pub updated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<EntityId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<EntityId>,
}

impl Default for TimestampMeta {
    fn default() -> Self {
        let stamp: String = "1970-01-01T00:00:00Z".into();
        Self {
            created: stamp.clone(),
            updated: stamp,
            created_by: None,
            updated_by: None,
        }
    }
}
// #endregion

// #region 🔖EntityHeader
/// @emoji 📋 Common header shared by all register entities via serde flatten.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityHeader {
    pub id: EntityId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextField>,
    pub status: LifecycleStatus,
    pub priority: Priority,
    pub ownership: Ownership,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<TaggedNote>,
    pub timestamps: TimestampMeta,
}

impl EntityHeader {
    pub fn new(id: EntityId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: None,
            status: LifecycleStatus::Draft,
            priority: Priority::Preferred,
            ownership: Ownership::default(),
            tags: Vec::new(),
            notes: Vec::new(),
            timestamps: TimestampMeta::default(),
        }
    }
}
// #endregion

// #region 🔖QuantitySpec
/// @emoji 📐 Numeric quantity with min/max/target bands and unit.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantitySpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    pub unit: String,
}

impl QuantitySpec {
    pub fn target_unit(target: f64, unit: impl Into<String>) -> Self {
        Self {
            target: Some(target),
            unit: unit.into(),
            ..Default::default()
        }
    }
}
// #endregion

// #region 🔖Trace
/// @emoji 🔗 Semantic trace link between two entities for auditability.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TraceKind {
    ObjectiveToRequirement,
    StakeholderToRequirement,
    UserToActivity,
    ActivityToFunction,
    FunctionToProgramElement,
    RequirementToDecision,
    RequirementToRisk,
    RequirementToStandard,
    RequirementToValidation,
    RequirementToApproval,
    RequirementToChange,
    EquipmentToActivity,
    ProcessToResource,
    ConstraintToImpact,
    ScenarioToDecision,
    IssueToAction,
    ActionToOwner,
    DecisionToOutcome,
    VersionToChange,
    FullAuditTrail,
}

/// @emoji 🧭 Directed trace edge stored in the program trace register.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceLink {
    pub id: EntityId,
    pub from_id: EntityId,
    pub to_id: EntityId,
    pub kind: TraceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl TraceLink {
    pub fn new(from_id: EntityId, to_id: EntityId, kind: TraceKind) -> Self {
        Self {
            id: EntityId::new_serial("trace"),
            from_id,
            to_id,
            kind,
            label: None,
        }
    }
}

impl vcs::Identified<EntityId> for TraceLink {
    fn id(&self) -> &EntityId {
        &self.id
    }
}
// #endregion

// #region 🔖Diagnostics
/// @emoji ⚠️ Severity band for validation and analysis diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// @emoji 🩺 Non-fatal program validation or analysis finding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDiagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<EntityId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
}

// #endregion

//#region ⚠️ Errors
/// 💥 Fatal program operation or exchange error.
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramError {
    #[error("invalid schema: expected {expected}, got {actual}")]
    InvalidSchema { expected: String, actual: String },
    #[error("missing entity {id}")]
    MissingEntity { id: EntityId },
    #[error("duplicate adjacency {a} — {b}")]
    DuplicateAdjacency { a: EntityId, b: EntityId },
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("deserialize error: {0}")]
    Deserialize(String),
    #[error("csv error: {0}")]
    Csv(String),
}
//#endregion ⚠️ Errors

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_id_orders_lexicographically() {
        let a = EntityId("element-2".into());
        let b = EntityId("element-10".into());
        assert!(a > b);
    }

    #[test]
    fn entity_id_serial_increments() {
        let first = EntityId::new_serial("test");
        let second = EntityId::new_serial("test");
        assert_ne!(first, second);
        assert!(first.to_string().starts_with("test-"));
    }
}
