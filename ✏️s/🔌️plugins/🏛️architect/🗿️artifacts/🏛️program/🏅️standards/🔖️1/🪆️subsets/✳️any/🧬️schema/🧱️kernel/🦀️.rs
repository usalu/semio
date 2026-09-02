//! 🧱️ Architect program artifact — shared kernel types for program entities: ids, headers,
//! quantities, traces, and diagnostics.

//! 🧱️ Shared kernel types for architect program entities — ids, headers, quantities, traces, and diagnostics.

use protocol::Patchable;
use std::cmp::Ordering;
use std::fmt;
// #region 🔖️EntityId
/// @emoji 🆔️ Stable string identity for any program entity or register row.
#[derive(Clone, Debug, PartialEq, Eq, Hash, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(transparent)]
#[cfg_attr(test, serde(transparent))]
pub struct EntityId(pub String);

impl EntityId {
    /// @emoji 🔢️ Process-local unique id under `prefix` (monotonic serial).
    ///
    /// `material` is retained for call-site clarity but uniqueness comes from a process-wide
    /// counter — many creators pass a constant label and still need distinct ids.
    pub async fn new_serial(prefix: &str, _material: impl AsRef<[u8]>) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
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

/// @emoji 🔗️ Hand-written (not derived): `EntityId` is a tuple struct — `#[derive(dsl::DslRecord)]`
/// only supports named fields, and `#[derive(dsl::DslScalar)]` only unit-variant enums — so its
/// `dsl::DslField` binding is written directly, bridging straight to `Shape::Text` like `String`'s
/// own blanket impl does.
impl dsl::DslField for EntityId {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    async fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(EntityId(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}
// #endregion

// #region 🔖️Priority
/// @emoji 🎚️ Relative importance band for requirements, relationships, and entities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

// #region 🔖️LifecycleStatus
/// @emoji 🔄️ Lifecycle and workflow status for register entities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

// #region 🔖️Ownership
/// @emoji 👥️ Ownership and authority roles attached to an entity header.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Ownership {
    pub owner_id: Option<EntityId>,
    pub authority_id: Option<EntityId>,
    pub consultant_ids: Vec<EntityId>,
    pub participant_ids: Vec<EntityId>,
}
// #endregion

// #region 🔖️Text
/// @emoji 📝️ Rich or plain text payload with optional format hint.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TextField {
    pub text: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub format: Option<String>,
}

impl TextField {
    pub async fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), format: None }
    }
}

/// @emoji 🏷️ Tagged free-text note on an entity.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TaggedNote {
    pub tag: String,
    pub text: String,
}

/// @emoji 🕒️ Created/updated audit timestamps on an entity header.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TimestampMeta {
    pub created: String,
    pub updated: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub created_by: Option<EntityId>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub updated_by: Option<EntityId>,
}

impl Default for TimestampMeta {
    fn default() -> Self {
        let stamp: String = "1970-01-01T00:00:00Z".into();
        Self { created: stamp.clone(), updated: stamp, created_by: None, updated_by: None }
    }
}
// #endregion

// #region 🔖️EntityHeader
/// @emoji 📋️ Common header shared by all register entities via serde flatten.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct EntityHeader {
    pub id: EntityId,
    pub name: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<TextField>,
    pub status: LifecycleStatus,
    pub priority: Priority,
    pub ownership: Ownership,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub tags: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub notes: Vec<TaggedNote>,
    pub timestamps: TimestampMeta,
}

impl EntityHeader {
    pub async fn new(id: EntityId, name: impl Into<String>) -> Self {
        Self { id, name: name.into(), description: None, status: LifecycleStatus::Draft, priority: Priority::Preferred, ownership: Ownership::default(), tags: Vec::new(), notes: Vec::new(), timestamps: TimestampMeta::default() }
    }
}
// #endregion

// #region 🔖️QuantitySpec
/// @emoji 📐️ Numeric quantity with min/max/target bands and unit.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct QuantitySpec {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub min: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub max: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub target: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub current: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub forecast: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub peak: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub average: Option<f64>,
    pub unit: String,
}

impl QuantitySpec {
    pub async fn target_unit(target: f64, unit: impl Into<String>) -> Self {
        Self { target: Some(target), unit: unit.into(), ..Default::default() }
    }
}
// #endregion

// #region 🔖️Trace
/// @emoji 🔗️ Semantic trace link between two entities for auditability.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

/// @emoji 🧭️ Directed trace edge stored in the plugin trace register.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TraceLink {
    pub id: EntityId,
    pub from_id: EntityId,
    pub to_id: EntityId,
    pub kind: TraceKind,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
}

impl TraceLink {
    pub async fn new(from_id: EntityId, to_id: EntityId, kind: TraceKind) -> Self {
        Self { id: EntityId::new_serial("trace", "trace"), from_id, to_id, kind, label: None }
    }
}

impl protocol::Identified<EntityId> for TraceLink {
    async fn id(&self) -> &EntityId {
        &self.id
    }
}
// #endregion

// #region 🔖️Diagnostics
/// @emoji ⚠️ Severity band for validation and analysis diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// @emoji 🩺️ Non-fatal program validation or analysis finding.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramDiagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub entity_id: Option<EntityId>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub register: Option<String>,
}

// #endregion

//#region ⚠️ Errors
/// 💥️ Fatal program operation or exchange error.
#[derive(Clone, Debug, PartialEq, Eq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum PluginError {
    InvalidSchema { expected: String, actual: String },
    MissingEntity { id: EntityId },
    DuplicateAdjacency { a: EntityId, b: EntityId },
    Serialize(String),
    Deserialize(String),
    Csv(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSchema { expected, actual } => write!(formatter, "invalid schema: expected {expected}, got {actual}"),
            Self::MissingEntity { id } => write!(formatter, "missing entity {id}"),
            Self::DuplicateAdjacency { a, b } => write!(formatter, "duplicate adjacency {a} — {b}"),
            Self::Serialize(message) => write!(formatter, "serialize error: {message}"),
            Self::Deserialize(message) => write!(formatter, "deserialize error: {message}"),
            Self::Csv(message) => write!(formatter, "csv error: {message}"),
        }
    }
}

impl std::error::Error for PluginError {}
//#endregion ⚠️ Errors

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn entity_id_orders_lexicographically() {
        let a = EntityId("element-2".into());
        let b = EntityId("element-10".into());
        assert!(a > b);
    }

    #[semio_framework_async_macros::async_test]
    async fn entity_id_serial_increments() {
        let first = EntityId::new_serial("test", "test");
        let second = EntityId::new_serial("test", "test");
        assert_ne!(first, second);
        assert!(first.to_string().starts_with("test-"));
    }
}

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TraceLinkPatch {
    pub from_id: Option<EntityId>,
    pub to_id: Option<EntityId>,
    pub kind: Option<TraceKind>,
    pub label: Option<Option<String>>,
}

impl Patchable<TraceLinkPatch> for TraceLink {
    async fn apply_patch(&mut self, patch: &TraceLinkPatch) {
        if let Some(value) = &patch.from_id {
            self.from_id = value.clone();
        }
        if let Some(value) = &patch.to_id {
            self.to_id = value.clone();
        }
        if let Some(value) = &patch.kind {
            self.kind = value.clone();
        }
        if let Some(value) = &patch.label {
            self.label = value.clone();
        }
    }

    async fn diff_patch(&self, other: &Self) -> Option<TraceLinkPatch> {
        Some(TraceLinkPatch { from_id: Some(other.from_id.clone()), to_id: Some(other.to_id.clone()), kind: Some(other.kind.clone()), label: Some(other.label.clone()) })
    }
}
