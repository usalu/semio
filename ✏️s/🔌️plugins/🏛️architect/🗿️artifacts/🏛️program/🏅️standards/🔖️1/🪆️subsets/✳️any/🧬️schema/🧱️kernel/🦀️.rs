//! 🧱️ Architect program artifact — shared kernel types for program entities: ids, headers,
//! quantities, traces, and diagnostics.

//! 🧱️ Shared kernel types for architect program entities — ids, headers, quantities, traces, and diagnostics.

use graph::{orient_endpoints, Undirected};
use protocol::list_delta::RowPatch;
use protocol::Identified;
use std::cmp::Ordering;
use std::fmt;
// #region 🔖️EntityId
/// 🆔️ Stable string identity for any program entity or register row.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(transparent)]
#[cfg_attr(test, serde(transparent))]
pub struct EntityId(pub String);

impl EntityId {
    /// 🔢️ Process-local unique id under `prefix` (monotonic serial).
    ///
    /// `material` is retained for call-site clarity but uniqueness comes from a process-wide
    /// counter — many creators pass a constant label and still need distinct ids.
    pub fn new_serial(prefix: &str, _material: impl AsRef<[u8]>) -> Self {
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

/// 🔗️ Hand-written (not derived): `EntityId` is a tuple struct — `#[derive(dsl::DslRecord)]`
/// only supports named fields, and `#[derive(dsl::DslScalar)]` only unit-variant enums — so its
/// `dsl::DslField` binding is written directly, bridging straight to `Shape::Text` like `String`'s
/// own blanket impl does.
impl semio_framework_dsl_record::BorrowedDslField for EntityId {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Text;
}

impl semio_framework_dsl_record::DslField for EntityId {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Text
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        <String as semio_framework_dsl_record::DslField>::shape_controlled(control)
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Text(self.0.clone())
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        <String as semio_framework_dsl_record::DslField>::to_value_controlled(&self.0, control)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Text(s) => Ok(EntityId(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        <String as semio_framework_dsl_record::DslField>::from_value_controlled(value, control).map(Self)
    }
}
// #endregion

// #region 🔖️Priority
/// 🎚️ Relative importance band for requirements, relationships, and entities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
/// 🔄️ Lifecycle and workflow status for register entities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
/// 👥️ Ownership and authority roles attached to an entity header.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Ownership {
    pub owner_id: Option<EntityId>,
    pub authority_id: Option<EntityId>,
    pub consultant_ids: Vec<EntityId>,
    pub participant_ids: Vec<EntityId>,
}
// #endregion

// #region 🔖️Text
/// 📝️ Rich or plain text payload with optional format hint.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TextField {
    pub text: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub format: Option<String>,
}

impl TextField {
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), format: None }
    }
}

/// 🏷️ Tagged free-text note on an entity.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TaggedNote {
    pub tag: String,
    pub text: String,
}

/// 🕒️ Created/updated audit timestamps on an entity header.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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
/// 📋️ Common header shared by all register entities via serde flatten.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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
    pub fn new(id: EntityId, name: impl Into<String>) -> Self {
        Self { id, name: name.into(), description: None, status: LifecycleStatus::Draft, priority: Priority::Preferred, ownership: Ownership::default(), tags: Vec::new(), notes: Vec::new(), timestamps: TimestampMeta::default() }
    }
}
// #endregion

// #region 🔖️QuantitySpec
/// 📐️ Numeric quantity with min/max/target bands and unit.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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
    pub fn target_unit(target: f64, unit: impl Into<String>) -> Self {
        Self { target: Some(target), unit: unit.into(), ..Default::default() }
    }
}
// #endregion

// #region 🔖️Trace
/// 🔗️ Semantic trace link between two entities for auditability.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

/// 🧭️ Directed trace edge stored in the plugin trace register.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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
    pub fn new(from_id: EntityId, to_id: EntityId, kind: TraceKind) -> Self {
        Self { id: EntityId::new_serial("trace", "trace"), from_id, to_id, kind, label: None }
    }
}

impl protocol::Identified<EntityId> for TraceLink {
    fn id(&self) -> &EntityId {
        &self.id
    }
}
// #endregion

// #region 🔖️Diagnostics
/// ⚠️ Severity band for validation and analysis diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// 🩺️ Non-fatal program validation or analysis finding.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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

impl fmt::Display for PluginError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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



#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TraceLinkPatch {
    pub from_id: Option<EntityId>,
    pub to_id: Option<EntityId>,
    pub kind: Option<TraceKind>,
    pub label: Option<String>,
}

//#region 🔖️DocumentHelpers
/// 📐️ Canonical undirected endpoint order for an adjacency pair — dissolved out of the former
/// `⚙️engine/↔️adjacency` topic (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a
/// pure document helper with no `&mut` and no app coupling, so it lives in the shared kernel beside
/// [`EntityId`], where the mutation leaves and the snapshot read it without reaching the schema root.
pub fn normalize_pair(a: &EntityId, b: &EntityId) -> (EntityId, EntityId) {
    let (left, right) = orient_endpoints::<&str, Undirected>(&a.0, &b.0);
    (EntityId(left.to_string()), EntityId(right.to_string()))
}
//#endregion 🔖️DocumentHelpers

// #region 🔖️PatchHelpers
/// 🩹️ Per-field patch algebra of one register row: `commit_row` writes a patched value and `capture_row` reads the current value so the
/// patch that undoes a patch can be built (an absent `Option` field captures nothing — a patch field `Option<T>` cannot express a clear,
/// so a clear travels as a row replacement, never as a patch).
pub trait PatchRow<T: Clone> {
    fn commit_row(&mut self, patch: &Option<T>);
    fn capture_row(&self) -> Option<T>;
}

impl<T: Clone> PatchRow<T> for T {
    fn commit_row(&mut self, patch: &Option<T>) {
        if let Some(value) = patch {
            *self = value.clone();
        }
    }

    fn capture_row(&self) -> Option<T> {
        Some(self.clone())
    }
}

impl<T: Clone> PatchRow<T> for Option<T> {
    fn commit_row(&mut self, patch: &Option<T>) {
        if let Some(value) = patch {
            *self = Some(value.clone());
        }
    }

    fn capture_row(&self) -> Option<T> {
        self.clone()
    }
}

#[macro_export]
macro_rules! impl_identified_header {
    ($ty:ty) => {
        impl Identified<EntityId> for $ty {
            fn id(&self) -> &EntityId {
                &self.header.id
            }
        }
    };
}

/// 🧬️ Implements the framework's [`RowPatch`] for one register row: every listed field is an absolute setter, `inverse` captures the
/// current value of exactly the fields the patch carries, and `absorb` lets the later patch's fields win.
#[macro_export]
macro_rules! impl_patchable {
    ($entity:ty, $patch:ty, { $( [ $($path:ident).+ ] => $f:ident ),+ $(,)? }) => {
        impl RowPatch<$entity> for $patch {
            fn commit_into(&self, row: &mut $entity, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $( PatchRow::commit_row(&mut row$(.$path)+, &self.$f); )+
                Ok(())
            }

            fn absorb(&mut self, later: Self) {
                $( if later.$f.is_some() { self.$f = later.$f; } )+
            }

            fn inverse(&self, row: &$entity) -> Self {
                Self { $( $f: if self.$f.is_some() { PatchRow::capture_row(&row$(.$path)+) } else { None } ),+, ..Self::default() }
            }

            fn is_empty(&self) -> bool {
                true $( && self.$f.is_none() )+
            }
        }
    };
}
pub use impl_identified_header;
pub use impl_patchable;
// #endregion

impl_patchable!(
    TraceLink,
    TraceLinkPatch,
    {
        [from_id] => from_id,
        [to_id] => to_id,
        [kind] => kind,
        [label] => label,
    }
);
