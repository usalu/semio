//! 🏛️ Architect program artifact — the typed register entities for all 65 feature areas (the
//! document model's row types; the `Program` document that holds them lives in `🦀️component.rs`).

//! 🏛️ Architectural programming register entities — typed domain model for all 65 feature areas.

use crate::artifacts::program::kernel::*;
use protocol::{Identified, Patchable};
use serde::{Deserialize, Serialize};

// #region 🔖️PatchHelpers
/// @emoji 🩹️ Per-field patch application (`apply_row`) and full-snapshot forward-diff
/// (`diff_row`) — the frozen `protocol::Patchable` contract splits `vcs::Patchable`'s single
/// mutate-and-return-inverse `apply_patch` into a mutate-only `apply_patch` plus a separate
/// `diff_patch(&self, other)` that computes the patch turning `self` into `other`; `diff_row`
/// always snapshots `other`'s value (never gated on inequality) so the recovered patch is exact
/// even for fields the underlying `Option<T>` representation otherwise can't express clearing to
/// `None` (a pre-existing representation limit of this macro, unchanged from `vcs::Patchable`'s
/// same `Option<T>`-typed patch fields).
trait PatchRow<T: Clone> {
    fn apply_row(&mut self, patch: &Option<T>);
    fn diff_row(&self, other: &Self, out: &mut Option<T>);
}

impl<T: Clone> PatchRow<T> for T {
    fn apply_row(&mut self, patch: &Option<T>) {
        if let Some(value) = patch {
            *self = value.clone();
        }
    }

    fn diff_row(&self, other: &Self, out: &mut Option<T>) {
        *out = Some(other.clone());
    }
}

impl<T: Clone> PatchRow<T> for Option<T> {
    fn apply_row(&mut self, patch: &Option<T>) {
        if let Some(value) = patch {
            *self = Some(value.clone());
        }
    }

    fn diff_row(&self, other: &Self, out: &mut Option<T>) {
        *out = other.clone();
    }
}

macro_rules! impl_identified_header {
    ($ty:ty) => {
        impl Identified<EntityId> for $ty {
            fn id(&self) -> &EntityId {
                &self.header.id
            }
        }
    };
}

macro_rules! impl_patchable {
    ($entity:ty, $patch:ty, { $( [ $($path:ident).+ ] => $f:ident ),+ $(,)? }) => {
        impl Patchable<$patch> for $entity {
            fn apply_patch(&mut self, patch: &$patch) {
                $( PatchRow::apply_row(&mut self$(.$path)+, &patch.$f); )+
            }

            fn diff_patch(&self, other: &Self) -> Option<$patch> {
                let mut patch = <$patch>::default();
                $( PatchRow::diff_row(&self$(.$path)+, &other$(.$path)+, &mut patch.$f); )+
                Some(patch)
            }
        }
    };
}
// #endregion

// #region 🔖️SharedEnums
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum InfluenceLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum EngagementLevel {
    Unaware,
    Resistant,
    Neutral,
    Supportive,
    Leading,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum UserCategory {
    Primary,
    Secondary,
    Occasional,
    Service,
    Visitor,
    Staff,
    Public,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum ProgramElementKind {
    Building,
    Campus,
    Floor,
    Zone,
    Room,
    Suite,
    Department,
    System,
    Circulation,
    Support,
    Outdoor,
    FurnitureGroup,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Primary,
    Secondary,
    Support,
    Administrative,
    Service,
    Technical,
    Public,
    Private,
    Shared,
    Restricted,
    Temporary,
    Future,
    Operational,
    Circulation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum FlowKind {
    People,
    Material,
    Information,
    Service,
    Equipment,
    Waste,
    Emergency,
    Vehicle,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum PrivacyKind {
    Public,
    SemiPublic,
    SemiPrivate,
    Private,
    Confidential,
    Restricted,
    Anonymous,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum SafetyDomain {
    LifeSafety,
    OccupationalHealth,
    Fire,
    Structural,
    Electrical,
    Chemical,
    Radiation,
    Ergonomics,
    Biological,
    Environmental,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum SecurityControlKind {
    AccessControl,
    Surveillance,
    Perimeter,
    Cyber,
    Personnel,
    Information,
    Physical,
    Procedural,
    Screening,
    KeyManagement,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum StorageClass {
    General,
    Secure,
    ClimateControlled,
    Hazardous,
    Archive,
    Mobile,
    Fixed,
    Shared,
    ColdChain,
    Flammable,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum EnvironmentalParameter {
    Temperature,
    Humidity,
    AirQuality,
    Lighting,
    Acoustics,
    Ventilation,
    Radiation,
    Vibration,
    Pressure,
    Iaq,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum HumanFactorAspect {
    Ergonomics,
    Cognition,
    Sensory,
    Social,
    Cultural,
    Behavioral,
    Physical,
    Psychological,
    Fatigue,
    Stress,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum AccessMode {
    Unrestricted,
    CardControlled,
    Biometric,
    Keyed,
    EscortRequired,
    TimeRestricted,
    RoleBased,
    EmergencyOnly,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipKind {
    Contains,
    Serves,
    Supports,
    DependsOn,
    ConflictsWith,
    EquivalentTo,
    AdjacentTo,
    Feeds,
    Receives,
    Controls,
    Monitors,
    Functional,
    Operational,
    Organizational,
    User,
    Service,
    Information,
    Access,
    Security,
    Supervision,
    Communication,
    Dependency,
    Sequential,
    SharedResource,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum AdjacencyKind {
    Required,
    Preferred,
    Optional,
    Prohibited,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionKind {
    Direct,
    Indirect,
    Controlled,
    SharedAccess,
    None,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum SeparationKind {
    Acoustic,
    Visual,
    Security,
    Olfactory,
    Thermal,
    Fire,
    Hygienic,
    Circulation,
    Operational,
    InfectionControl,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum FlowDirection {
    OneWay,
    TwoWay,
    BidirectionalPeak,
    Restricted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum AccessLevel {
    Public,
    Restricted,
    Controlled,
    Private,
    Secure,
    EmergencyOnly,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum RiskLevel {
    Negligible,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum ConflictKind {
    Adjacency,
    Capacity,
    Schedule,
    Budget,
    Regulatory,
    Operational,
    Environmental,
    Security,
    Priority,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum RequirementKind {
    Functional,
    Spatial,
    Performance,
    Regulatory,
    Operational,
    Technical,
    Aesthetic,
    Sustainability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum ValidationStatus {
    Pending,
    Passed,
    Failed,
    Waived,
    Deferred,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum AnalysisKind {
    Gap,
    Conflict,
    Dependency,
    Capacity,
    Demand,
    Utilization,
    Workflow,
    Risk,
    Cost,
    Scenario,
    Sensitivity,
    Impact,
    Trend,
    RequirementComparison,
    RequirementClustering,
    RequirementFiltering,
    RequirementSorting,
    RequirementScoring,
    RequirementWeighting,
    RelationshipAnalysis,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum ReportKind {
    ExecutiveSummary,
    ProgramOverview,
    StakeholderSummary,
    RequirementsMatrix,
    AdjacencyMatrix,
    GapAnalysis,
    RiskRegister,
    DecisionLog,
    ValidationSummary,
    Recommendation,
    UserSummary,
    FunctionalSummary,
    CapacitySummary,
    WorkflowSummary,
    ComplianceSummary,
    CostSummary,
    ScheduleSummary,
    ChangeSummary,
    OpenIssueSummary,
    PrioritySummary,
    ScenarioSummary,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum IssueSeverity {
    Cosmetic,
    Minor,
    Major,
    Critical,
    Blocker,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum AuditAction {
    Created,
    Updated,
    Deleted,
    Reviewed,
    Approved,
    Rejected,
    Exported,
    Imported,
    Merged,
    Archived,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum CostBasis {
    Capital,
    Operational,
    Lifecycle,
    Replacement,
    Maintenance,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DeliveryPhase {
    Concept,
    Schematic,
    DesignDevelopment,
    ConstructionDocuments,
    Procurement,
    Construction,
    Commissioning,
    Occupancy,
}
// #endregion

// #region 🔖️ProgramMeta
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ProgramMeta {
    pub schema: String,
    pub document_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub purpose: TextField,
    pub terminology: Vec<String>,
    pub classification: Vec<String>,
    pub industry_sector: String,
    pub project_type: String,
    pub locale: String,
    pub revision: String,
    pub author_ids: Vec<EntityId>,
    pub source_system: Option<String>,
    pub export_profile: Option<String>,
    pub timestamps: TimestampMeta,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramMetaPatch {
    pub schema: Option<String>,
    pub document_id: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub purpose: Option<TextField>,
    pub terminology: Option<Vec<String>>,
    pub classification: Option<Vec<String>>,
    pub industry_sector: Option<String>,
    pub project_type: Option<String>,
    pub locale: Option<String>,
    pub revision: Option<String>,
    pub author_ids: Option<Vec<EntityId>>,
    pub source_system: Option<String>,
    pub export_profile: Option<String>,
    pub timestamps: Option<TimestampMeta>,
}

impl Patchable<ProgramMetaPatch> for ProgramMeta {
    fn apply_patch(&mut self, patch: &ProgramMetaPatch) {
        PatchRow::apply_row(&mut self.schema, &patch.schema);
        PatchRow::apply_row(&mut self.document_id, &patch.document_id);
        PatchRow::apply_row(&mut self.title, &patch.title);
        PatchRow::apply_row(&mut self.subtitle, &patch.subtitle);
        PatchRow::apply_row(&mut self.purpose, &patch.purpose);
        PatchRow::apply_row(&mut self.terminology, &patch.terminology);
        PatchRow::apply_row(&mut self.classification, &patch.classification);
        PatchRow::apply_row(&mut self.industry_sector, &patch.industry_sector);
        PatchRow::apply_row(&mut self.project_type, &patch.project_type);
        PatchRow::apply_row(&mut self.locale, &patch.locale);
        PatchRow::apply_row(&mut self.revision, &patch.revision);
        PatchRow::apply_row(&mut self.author_ids, &patch.author_ids);
        PatchRow::apply_row(&mut self.source_system, &patch.source_system);
        PatchRow::apply_row(&mut self.export_profile, &patch.export_profile);
        PatchRow::apply_row(&mut self.timestamps, &patch.timestamps);
    }

    fn diff_patch(&self, other: &Self) -> Option<ProgramMetaPatch> {
        let mut patch = ProgramMetaPatch::default();
        PatchRow::diff_row(&self.schema, &other.schema, &mut patch.schema);
        PatchRow::diff_row(&self.document_id, &other.document_id, &mut patch.document_id);
        PatchRow::diff_row(&self.title, &other.title, &mut patch.title);
        PatchRow::diff_row(&self.subtitle, &other.subtitle, &mut patch.subtitle);
        PatchRow::diff_row(&self.purpose, &other.purpose, &mut patch.purpose);
        PatchRow::diff_row(&self.terminology, &other.terminology, &mut patch.terminology);
        PatchRow::diff_row(&self.classification, &other.classification, &mut patch.classification);
        PatchRow::diff_row(&self.industry_sector, &other.industry_sector, &mut patch.industry_sector);
        PatchRow::diff_row(&self.project_type, &other.project_type, &mut patch.project_type);
        PatchRow::diff_row(&self.locale, &other.locale, &mut patch.locale);
        PatchRow::diff_row(&self.revision, &other.revision, &mut patch.revision);
        PatchRow::diff_row(&self.author_ids, &other.author_ids, &mut patch.author_ids);
        PatchRow::diff_row(&self.source_system, &other.source_system, &mut patch.source_system);
        PatchRow::diff_row(&self.export_profile, &other.export_profile, &mut patch.export_profile);
        PatchRow::diff_row(&self.timestamps, &other.timestamps, &mut patch.timestamps);
        Some(patch)
    }
}
// #endregion

// #region 🔖️ProjectDefinition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDefinition {
    pub id: EntityId,
    pub code: String,
    pub client_name: String,
    pub owner_organization: String,
    pub brief_summary: TextField,
    pub problem_statement: TextField,
    pub vision: TextField,
    pub mission: TextField,
    pub objectives: Vec<String>,
    pub success_criteria: Vec<String>,
    pub project_priorities: Vec<Priority>,
    pub completion_criteria: Vec<String>,
    pub decision_criteria: Vec<String>,
    pub scope_inclusions: Vec<String>,
    pub scope_exclusions: Vec<String>,
    pub assumptions: Vec<String>,
    pub constraints_summary: Vec<String>,
    pub dependencies: Vec<String>,
    pub deliverables: Vec<String>,
    pub phases: Vec<String>,
    pub geographic_context: TextField,
    pub development_context: TextField,
    pub operational_context: TextField,
    pub regulatory_context: Vec<String>,
    pub funding_model: String,
    pub ownership: Ownership,
    pub timestamps: TimestampMeta,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDefinitionPatch {
    pub id: Option<EntityId>,
    pub code: Option<String>,
    pub client_name: Option<String>,
    pub owner_organization: Option<String>,
    pub brief_summary: Option<TextField>,
    pub problem_statement: Option<TextField>,
    pub vision: Option<TextField>,
    pub mission: Option<TextField>,
    pub objectives: Option<Vec<String>>,
    pub success_criteria: Option<Vec<String>>,
    pub project_priorities: Option<Vec<Priority>>,
    pub completion_criteria: Option<Vec<String>>,
    pub decision_criteria: Option<Vec<String>>,
    pub scope_inclusions: Option<Vec<String>>,
    pub scope_exclusions: Option<Vec<String>>,
    pub assumptions: Option<Vec<String>>,
    pub constraints_summary: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub deliverables: Option<Vec<String>>,
    pub phases: Option<Vec<String>>,
    pub geographic_context: Option<TextField>,
    pub development_context: Option<TextField>,
    pub operational_context: Option<TextField>,
    pub regulatory_context: Option<Vec<String>>,
    pub funding_model: Option<String>,
    pub ownership: Option<Ownership>,
    pub timestamps: Option<TimestampMeta>,
}

impl Identified<EntityId> for ProjectDefinition {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl_patchable!(
    ProjectDefinition,
    ProjectDefinitionPatch,
    {
        [id] => id,
        [code] => code,
        [client_name] => client_name,
        [owner_organization] => owner_organization,
        [brief_summary] => brief_summary,
        [problem_statement] => problem_statement,
        [vision] => vision,
        [mission] => mission,
        [objectives] => objectives,
        [success_criteria] => success_criteria,
        [project_priorities] => project_priorities,
        [completion_criteria] => completion_criteria,
        [decision_criteria] => decision_criteria,
        [scope_inclusions] => scope_inclusions,
        [scope_exclusions] => scope_exclusions,
        [assumptions] => assumptions,
        [constraints_summary] => constraints_summary,
        [dependencies] => dependencies,
        [deliverables] => deliverables,
        [phases] => phases,
        [geographic_context] => geographic_context,
        [development_context] => development_context,
        [operational_context] => operational_context,
        [regulatory_context] => regulatory_context,
        [funding_model] => funding_model,
        [ownership] => ownership,
        [timestamps] => timestamps,
    }
);
// #endregion

// #region 🔖️Stakeholder
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Stakeholder {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub role: String,
    pub organization: String,
    pub department: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub influence: InfluenceLevel,
    pub interest: InfluenceLevel,
    pub engagement: EngagementLevel,
    pub expectations: Vec<String>,
    pub concerns: Vec<String>,
    pub requirement_ids: Vec<EntityId>,
    pub decision_authority: bool,
    pub communication_preferences: Vec<String>,
    pub reporting_frequency: Option<String>,
    pub involvement_phases: Vec<String>,
    pub availability: Option<String>,
    pub representative_of: Option<EntityId>,
    pub delegated_to: Option<EntityId>,
    pub relationship_to_client: Option<String>,
    pub power_interest_notes: Vec<TaggedNote>,
    pub stakeholder_type: String,
    pub influence_strategy: Option<String>,
    pub communication_channels: Vec<String>,
    pub success_metrics: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakeholderPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub role: Option<String>,
    pub organization: Option<String>,
    pub department: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub influence: Option<InfluenceLevel>,
    pub interest: Option<InfluenceLevel>,
    pub engagement: Option<EngagementLevel>,
    pub expectations: Option<Vec<String>>,
    pub concerns: Option<Vec<String>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub decision_authority: Option<bool>,
    pub communication_preferences: Option<Vec<String>>,
    pub reporting_frequency: Option<String>,
    pub involvement_phases: Option<Vec<String>>,
    pub availability: Option<String>,
    pub representative_of: Option<EntityId>,
    pub delegated_to: Option<EntityId>,
    pub relationship_to_client: Option<String>,
    pub power_interest_notes: Option<Vec<TaggedNote>>,
    pub stakeholder_type: Option<String>,
    pub influence_strategy: Option<String>,
    pub communication_channels: Option<Vec<String>>,
    pub success_metrics: Option<Vec<String>>,
}

impl_identified_header!(Stakeholder);

impl_patchable!(
    Stakeholder,
    StakeholderPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [role] => role,
        [organization] => organization,
        [department] => department,
        [contact_email] => contact_email,
        [contact_phone] => contact_phone,
        [influence] => influence,
        [interest] => interest,
        [engagement] => engagement,
        [expectations] => expectations,
        [concerns] => concerns,
        [requirement_ids] => requirement_ids,
        [decision_authority] => decision_authority,
        [communication_preferences] => communication_preferences,
        [reporting_frequency] => reporting_frequency,
        [involvement_phases] => involvement_phases,
        [availability] => availability,
        [representative_of] => representative_of,
        [delegated_to] => delegated_to,
        [relationship_to_client] => relationship_to_client,
        [power_interest_notes] => power_interest_notes,
        [stakeholder_type] => stakeholder_type,
        [influence_strategy] => influence_strategy,
        [communication_channels] => communication_channels,
        [success_metrics] => success_metrics,
    }
);
// #endregion

// #region 🔖️UserProfile
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub category: UserCategory,
    pub demographic: Option<String>,
    pub age_range: Option<String>,
    pub abilities: Vec<String>,
    pub disabilities: Vec<String>,
    pub occupation: Option<String>,
    pub role_title: Option<String>,
    pub department: Option<String>,
    pub mobility_profile: Vec<String>,
    pub sensory_profile: Vec<String>,
    pub cognitive_profile: Vec<String>,
    pub behavioral_patterns: Vec<String>,
    pub usage_frequency: Option<String>,
    pub usage_duration: Option<String>,
    pub peak_usage_times: Vec<String>,
    pub technology_proficiency: Option<String>,
    pub preferences: Vec<String>,
    pub pain_points: Vec<String>,
    pub goals: Vec<String>,
    pub activity_ids: Vec<EntityId>,
    pub research_method: Option<String>,
    pub persona_archetype: Option<String>,
    pub validated: bool,
    pub stakeholder_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfilePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub category: Option<UserCategory>,
    pub demographic: Option<String>,
    pub age_range: Option<String>,
    pub abilities: Option<Vec<String>>,
    pub disabilities: Option<Vec<String>>,
    pub occupation: Option<String>,
    pub role_title: Option<String>,
    pub department: Option<String>,
    pub mobility_profile: Option<Vec<String>>,
    pub sensory_profile: Option<Vec<String>>,
    pub cognitive_profile: Option<Vec<String>>,
    pub behavioral_patterns: Option<Vec<String>>,
    pub usage_frequency: Option<String>,
    pub usage_duration: Option<String>,
    pub peak_usage_times: Option<Vec<String>>,
    pub technology_proficiency: Option<String>,
    pub preferences: Option<Vec<String>>,
    pub pain_points: Option<Vec<String>>,
    pub goals: Option<Vec<String>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub research_method: Option<String>,
    pub persona_archetype: Option<String>,
    pub validated: Option<bool>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(UserProfile);

impl_patchable!(
    UserProfile,
    UserProfilePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [category] => category,
        [demographic] => demographic,
        [age_range] => age_range,
        [abilities] => abilities,
        [disabilities] => disabilities,
        [occupation] => occupation,
        [role_title] => role_title,
        [department] => department,
        [mobility_profile] => mobility_profile,
        [sensory_profile] => sensory_profile,
        [cognitive_profile] => cognitive_profile,
        [behavioral_patterns] => behavioral_patterns,
        [usage_frequency] => usage_frequency,
        [usage_duration] => usage_duration,
        [peak_usage_times] => peak_usage_times,
        [technology_proficiency] => technology_proficiency,
        [preferences] => preferences,
        [pain_points] => pain_points,
        [goals] => goals,
        [activity_ids] => activity_ids,
        [research_method] => research_method,
        [persona_archetype] => persona_archetype,
        [validated] => validated,
        [stakeholder_ids] => stakeholder_ids,
    }
);
// #endregion

// #region 🔖️Activity
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub intensity: Option<String>,
    pub participants: QuantitySpec,
    pub equipment_ids: Vec<EntityId>,
    pub space_requirements: Vec<String>,
    pub environmental_needs: Vec<String>,
    pub privacy_needs: Vec<String>,
    pub accessibility_needs: Vec<String>,
    pub adjacent_activities: Vec<EntityId>,
    pub sequencing: Vec<String>,
    pub peak_periods: Vec<String>,
    pub workflow_steps: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub user_profile_ids: Vec<EntityId>,
    pub function_ids: Vec<EntityId>,
    pub performance_indicators: Vec<String>,
    pub activity_type: String,
    pub location_context: Option<String>,
    pub temporal_pattern: Option<String>,
    pub supervision_level: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub intensity: Option<String>,
    pub participants: Option<QuantitySpec>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub space_requirements: Option<Vec<String>>,
    pub environmental_needs: Option<Vec<String>>,
    pub privacy_needs: Option<Vec<String>>,
    pub accessibility_needs: Option<Vec<String>>,
    pub adjacent_activities: Option<Vec<EntityId>>,
    pub sequencing: Option<Vec<String>>,
    pub peak_periods: Option<Vec<String>>,
    pub workflow_steps: Option<Vec<String>>,
    pub inputs: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub function_ids: Option<Vec<EntityId>>,
    pub performance_indicators: Option<Vec<String>>,
    pub activity_type: Option<String>,
    pub location_context: Option<String>,
    pub temporal_pattern: Option<String>,
    pub supervision_level: Option<String>,
}

impl_identified_header!(Activity);

impl_patchable!(
    Activity,
    ActivityPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [frequency] => frequency,
        [duration] => duration,
        [intensity] => intensity,
        [participants] => participants,
        [equipment_ids] => equipment_ids,
        [space_requirements] => space_requirements,
        [environmental_needs] => environmental_needs,
        [privacy_needs] => privacy_needs,
        [accessibility_needs] => accessibility_needs,
        [adjacent_activities] => adjacent_activities,
        [sequencing] => sequencing,
        [peak_periods] => peak_periods,
        [workflow_steps] => workflow_steps,
        [inputs] => inputs,
        [outputs] => outputs,
        [user_profile_ids] => user_profile_ids,
        [function_ids] => function_ids,
        [performance_indicators] => performance_indicators,
        [activity_type] => activity_type,
        [location_context] => location_context,
        [temporal_pattern] => temporal_pattern,
        [supervision_level] => supervision_level,
    }
);
// #endregion

// #region 🔖️Function
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub kind: FunctionKind,
    pub purpose: TextField,
    pub criticality: Priority,
    pub performance_targets: Vec<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: QuantitySpec,
    pub equipment_ids: Vec<EntityId>,
    pub resource_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub dependencies: Vec<EntityId>,
    pub interfaces: Vec<String>,
    pub constraints: Vec<String>,
    pub quality_criteria: Vec<String>,
    pub regulatory_refs: Vec<String>,
    pub future_changes: Vec<String>,
    pub owner_stakeholder_id: Option<EntityId>,
    pub success_metrics: Vec<String>,
    pub hierarchy_parent_id: Option<EntityId>,
    pub conflict_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub kind: Option<FunctionKind>,
    pub purpose: Option<TextField>,
    pub criticality: Option<Priority>,
    pub performance_targets: Option<Vec<String>>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: Option<QuantitySpec>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub resource_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub interfaces: Option<Vec<String>>,
    pub constraints: Option<Vec<String>>,
    pub quality_criteria: Option<Vec<String>>,
    pub regulatory_refs: Option<Vec<String>>,
    pub future_changes: Option<Vec<String>>,
    pub owner_stakeholder_id: Option<EntityId>,
    pub success_metrics: Option<Vec<String>>,
    pub hierarchy_parent_id: Option<EntityId>,
    pub conflict_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(Function);

impl_patchable!(
    Function,
    FunctionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [kind] => kind,
        [purpose] => purpose,
        [criticality] => criticality,
        [performance_targets] => performance_targets,
        [service_level] => service_level,
        [operating_hours] => operating_hours,
        [staffing] => staffing,
        [equipment_ids] => equipment_ids,
        [resource_ids] => resource_ids,
        [activity_ids] => activity_ids,
        [element_ids] => element_ids,
        [dependencies] => dependencies,
        [interfaces] => interfaces,
        [constraints] => constraints,
        [quality_criteria] => quality_criteria,
        [regulatory_refs] => regulatory_refs,
        [future_changes] => future_changes,
        [owner_stakeholder_id] => owner_stakeholder_id,
        [success_metrics] => success_metrics,
        [hierarchy_parent_id] => hierarchy_parent_id,
        [conflict_ids] => conflict_ids,
    }
);
// #endregion

// #region 🔖️ProgramElement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ProgramElement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub kind: ProgramElementKind,
    pub parent_id: Option<EntityId>,
    pub level: Option<String>,
    pub area: QuantitySpec,
    pub volume: QuantitySpec,
    pub height: QuantitySpec,
    pub occupancy: QuantitySpec,
    pub function_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub user_profile_ids: Vec<EntityId>,
    pub adjacency_ids: Vec<EntityId>,
    pub quantity_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub location_hint: Option<String>,
    pub orientation: Option<String>,
    pub daylight_requirement: Option<String>,
    pub acoustic_class: Option<String>,
    pub security_zone: Option<String>,
    pub flexibility_notes: Vec<String>,
    pub growth_allocation: Option<String>,
    pub circulation_role: Option<String>,
    pub visibility_level: Option<String>,
    pub adjacency_preferences: Vec<EntityId>,
    pub environmental_zone: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramElementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub kind: Option<ProgramElementKind>,
    pub parent_id: Option<EntityId>,
    pub level: Option<String>,
    pub area: Option<QuantitySpec>,
    pub volume: Option<QuantitySpec>,
    pub height: Option<QuantitySpec>,
    pub occupancy: Option<QuantitySpec>,
    pub function_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub adjacency_ids: Option<Vec<EntityId>>,
    pub quantity_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub location_hint: Option<String>,
    pub orientation: Option<String>,
    pub daylight_requirement: Option<String>,
    pub acoustic_class: Option<String>,
    pub security_zone: Option<String>,
    pub flexibility_notes: Option<Vec<String>>,
    pub growth_allocation: Option<String>,
    pub circulation_role: Option<String>,
    pub visibility_level: Option<String>,
    pub adjacency_preferences: Option<Vec<EntityId>>,
    pub environmental_zone: Option<String>,
}

impl_identified_header!(ProgramElement);

impl_patchable!(
    ProgramElement,
    ProgramElementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [kind] => kind,
        [parent_id] => parent_id,
        [level] => level,
        [area] => area,
        [volume] => volume,
        [height] => height,
        [occupancy] => occupancy,
        [function_ids] => function_ids,
        [activity_ids] => activity_ids,
        [user_profile_ids] => user_profile_ids,
        [adjacency_ids] => adjacency_ids,
        [quantity_ids] => quantity_ids,
        [requirement_ids] => requirement_ids,
        [location_hint] => location_hint,
        [orientation] => orientation,
        [daylight_requirement] => daylight_requirement,
        [acoustic_class] => acoustic_class,
        [security_zone] => security_zone,
        [flexibility_notes] => flexibility_notes,
        [growth_allocation] => growth_allocation,
        [circulation_role] => circulation_role,
        [visibility_level] => visibility_level,
        [adjacency_preferences] => adjacency_preferences,
        [environmental_zone] => environmental_zone,
    }
);
// #endregion

// #region 🔖️QuantityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct QuantityRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub target_element_id: EntityId,
    pub metric: String,
    pub quantity: QuantitySpec,
    pub basis: Option<String>,
    pub calculation_method: Option<String>,
    pub source: Option<String>,
    pub benchmark_ref: Option<EntityId>,
    #[dsl(unit = "pct")]
    pub tolerance_percent: Option<f64>,
    pub peak_factor: Option<f64>,
    pub growth_factor: Option<f64>,
    pub unit_cost: Option<f64>,
    pub currency: Option<String>,
    pub verification_method: Option<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub assumptions: Vec<String>,
    pub constraints: Vec<String>,
    pub schedule_phase: Option<String>,
    pub responsible_party: Option<EntityId>,
    pub last_verified: Option<String>,
    pub variance_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub target_element_id: Option<EntityId>,
    pub metric: Option<String>,
    pub quantity: Option<QuantitySpec>,
    pub basis: Option<String>,
    pub calculation_method: Option<String>,
    pub source: Option<String>,
    pub benchmark_ref: Option<EntityId>,
    pub tolerance_percent: Option<f64>,
    pub peak_factor: Option<f64>,
    pub growth_factor: Option<f64>,
    pub unit_cost: Option<f64>,
    pub currency: Option<String>,
    pub verification_method: Option<String>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub assumptions: Option<Vec<String>>,
    pub constraints: Option<Vec<String>>,
    pub schedule_phase: Option<String>,
    pub responsible_party: Option<EntityId>,
    pub last_verified: Option<String>,
    pub variance_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(QuantityRequirement);

impl_patchable!(
    QuantityRequirement,
    QuantityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [target_element_id] => target_element_id,
        [metric] => metric,
        [quantity] => quantity,
        [basis] => basis,
        [calculation_method] => calculation_method,
        [source] => source,
        [benchmark_ref] => benchmark_ref,
        [tolerance_percent] => tolerance_percent,
        [peak_factor] => peak_factor,
        [growth_factor] => growth_factor,
        [unit_cost] => unit_cost,
        [currency] => currency,
        [verification_method] => verification_method,
        [related_requirement_ids] => related_requirement_ids,
        [assumptions] => assumptions,
        [constraints] => constraints,
        [schedule_phase] => schedule_phase,
        [responsible_party] => responsible_party,
        [last_verified] => last_verified,
        [variance_notes] => variance_notes,
    }
);
// #endregion

// #region 🔖️Relationship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub source_id: EntityId,
    pub target_id: EntityId,
    pub kind: RelationshipKind,
    pub strength: Option<f64>,
    pub directional: bool,
    pub rationale: Option<TextField>,
    pub constraints: Vec<String>,
    pub conditions: Vec<String>,
    pub relationship_priority: Priority,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub evidence: Vec<String>,
    pub conflict_ids: Vec<EntityId>,
    pub trace_links: Vec<TraceLink>,
    pub bidirectional: bool,
    #[dsl(unit = "m")]
    pub distance_constraint_m: Option<f64>,
    pub capacity_constraint: Option<String>,
    pub regulatory_basis: Vec<String>,
    pub review_cycle: Option<String>,
    pub owner_id: Option<EntityId>,
    pub proximity_requirement: Option<TextField>,
    pub compatibility_requirement: Option<TextField>,
    pub incompatibility_requirement: Option<TextField>,
    pub separation_requirements: Vec<SeparationKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub source_id: Option<EntityId>,
    pub target_id: Option<EntityId>,
    pub kind: Option<RelationshipKind>,
    pub strength: Option<f64>,
    pub directional: Option<bool>,
    pub rationale: Option<TextField>,
    pub constraints: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub relationship_priority: Option<Priority>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub evidence: Option<Vec<String>>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub bidirectional: Option<bool>,
    pub distance_constraint_m: Option<f64>,
    pub capacity_constraint: Option<String>,
    pub regulatory_basis: Option<Vec<String>>,
    pub review_cycle: Option<String>,
    pub owner_id: Option<EntityId>,
    pub proximity_requirement: Option<TextField>,
    pub compatibility_requirement: Option<TextField>,
    pub incompatibility_requirement: Option<TextField>,
    pub separation_requirements: Option<Vec<SeparationKind>>,
}

impl_identified_header!(Relationship);

impl_patchable!(
    Relationship,
    RelationshipPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [source_id] => source_id,
        [target_id] => target_id,
        [kind] => kind,
        [strength] => strength,
        [directional] => directional,
        [rationale] => rationale,
        [constraints] => constraints,
        [conditions] => conditions,
        [relationship_priority] => relationship_priority,
        [valid_from] => valid_from,
        [valid_until] => valid_until,
        [evidence] => evidence,
        [conflict_ids] => conflict_ids,
        [trace_links] => trace_links,
        [bidirectional] => bidirectional,
        [distance_constraint_m] => distance_constraint_m,
        [capacity_constraint] => capacity_constraint,
        [regulatory_basis] => regulatory_basis,
        [review_cycle] => review_cycle,
        [owner_id] => owner_id,
        [proximity_requirement] => proximity_requirement,
        [compatibility_requirement] => compatibility_requirement,
        [incompatibility_requirement] => incompatibility_requirement,
        [separation_requirements] => separation_requirements,
    }
);
// #endregion

// #region 🔖️Adjacency
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Adjacency {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub element_a_id: EntityId,
    pub element_b_id: EntityId,
    pub kind: AdjacencyKind,
    pub connection: ConnectionKind,
    pub separations: Vec<SeparationKind>,
    pub weight: f64,
    pub rationale: Option<TextField>,
    #[dsl(unit = "m")]
    pub distance_max_m: Option<f64>,
    #[dsl(unit = "m")]
    pub distance_min_m: Option<f64>,
    pub level_constraint: Option<String>,
    pub access_path: Option<String>,
    pub shared_wall: bool,
    pub shared_entry: bool,
    pub traffic_isolation: bool,
    pub circulation_overlap: bool,
    pub conflict_ids: Vec<EntityId>,
    pub normalized: bool,
    pub verification_status: ValidationStatus,
    pub source_relationship_id: Option<EntityId>,
    pub internal_external_access: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjacencyPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub element_a_id: Option<EntityId>,
    pub element_b_id: Option<EntityId>,
    pub kind: Option<AdjacencyKind>,
    pub connection: Option<ConnectionKind>,
    pub separations: Option<Vec<SeparationKind>>,
    pub weight: Option<f64>,
    pub rationale: Option<TextField>,
    pub distance_max_m: Option<f64>,
    pub distance_min_m: Option<f64>,
    pub level_constraint: Option<String>,
    pub access_path: Option<String>,
    pub shared_wall: Option<bool>,
    pub shared_entry: Option<bool>,
    pub traffic_isolation: Option<bool>,
    pub circulation_overlap: Option<bool>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub normalized: Option<bool>,
    pub verification_status: Option<ValidationStatus>,
    pub source_relationship_id: Option<EntityId>,
    pub internal_external_access: Option<String>,
}

impl_identified_header!(Adjacency);

impl_patchable!(
    Adjacency,
    AdjacencyPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [element_a_id] => element_a_id,
        [element_b_id] => element_b_id,
        [kind] => kind,
        [connection] => connection,
        [separations] => separations,
        [weight] => weight,
        [rationale] => rationale,
        [distance_max_m] => distance_max_m,
        [distance_min_m] => distance_min_m,
        [level_constraint] => level_constraint,
        [access_path] => access_path,
        [shared_wall] => shared_wall,
        [shared_entry] => shared_entry,
        [traffic_isolation] => traffic_isolation,
        [circulation_overlap] => circulation_overlap,
        [conflict_ids] => conflict_ids,
        [normalized] => normalized,
        [verification_status] => verification_status,
        [source_relationship_id] => source_relationship_id,
        [internal_external_access] => internal_external_access,
    }
);
// #endregion

// #region 🔖️Process
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub trigger: Option<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub steps: Vec<String>,
    pub actors: Vec<EntityId>,
    pub equipment_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub duration: Option<String>,
    pub frequency: Option<String>,
    pub critical_path: bool,
    pub bottlenecks: Vec<String>,
    pub dependencies: Vec<EntityId>,
    pub kpis: Vec<String>,
    pub automation_level: Option<String>,
    pub failure_modes: Vec<String>,
    pub improvement_opportunities: Vec<String>,
    pub regulatory_refs: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub workflow_type: Option<String>,
    pub handoff_points: Vec<String>,
    pub quality_gates: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub trigger: Option<String>,
    pub inputs: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
    pub actors: Option<Vec<EntityId>>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub duration: Option<String>,
    pub frequency: Option<String>,
    pub critical_path: Option<bool>,
    pub bottlenecks: Option<Vec<String>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub kpis: Option<Vec<String>>,
    pub automation_level: Option<String>,
    pub failure_modes: Option<Vec<String>>,
    pub improvement_opportunities: Option<Vec<String>>,
    pub regulatory_refs: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub workflow_type: Option<String>,
    pub handoff_points: Option<Vec<String>>,
    pub quality_gates: Option<Vec<String>>,
}

impl_identified_header!(Process);

impl_patchable!(
    Process,
    ProcessPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [trigger] => trigger,
        [inputs] => inputs,
        [outputs] => outputs,
        [steps] => steps,
        [actors] => actors,
        [equipment_ids] => equipment_ids,
        [element_ids] => element_ids,
        [duration] => duration,
        [frequency] => frequency,
        [critical_path] => critical_path,
        [bottlenecks] => bottlenecks,
        [dependencies] => dependencies,
        [kpis] => kpis,
        [automation_level] => automation_level,
        [failure_modes] => failure_modes,
        [improvement_opportunities] => improvement_opportunities,
        [regulatory_refs] => regulatory_refs,
        [owner_id] => owner_id,
        [workflow_type] => workflow_type,
        [handoff_points] => handoff_points,
        [quality_gates] => quality_gates,
    }
);
// #endregion

// #region 🔖️FlowRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FlowRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub from_element_id: EntityId,
    pub to_element_id: EntityId,
    pub kind: FlowKind,
    pub flow_type: String,
    pub direction: FlowDirection,
    pub volume: QuantitySpec,
    pub peak_rate: Option<f64>,
    #[dsl(unit = "m")]
    pub clear_width_m: Option<f64>,
    #[dsl(unit = "m")]
    pub clear_height_m: Option<f64>,
    pub separation_requirements: Vec<SeparationKind>,
    pub access_level: AccessLevel,
    pub time_windows: Vec<String>,
    pub equipment_clearance: Option<String>,
    pub signage_required: bool,
    pub escort_required: bool,
    pub emergency_route: bool,
    pub barrier_free: bool,
    pub monitoring_required: bool,
    pub process_id: Option<EntityId>,
    pub conflict_ids: Vec<EntityId>,
    pub verification_method: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub from_element_id: Option<EntityId>,
    pub to_element_id: Option<EntityId>,
    pub kind: Option<FlowKind>,
    pub flow_type: Option<String>,
    pub direction: Option<FlowDirection>,
    pub volume: Option<QuantitySpec>,
    pub peak_rate: Option<f64>,
    pub clear_width_m: Option<f64>,
    pub clear_height_m: Option<f64>,
    pub separation_requirements: Option<Vec<SeparationKind>>,
    pub access_level: Option<AccessLevel>,
    pub time_windows: Option<Vec<String>>,
    pub equipment_clearance: Option<String>,
    pub signage_required: Option<bool>,
    pub escort_required: Option<bool>,
    pub emergency_route: Option<bool>,
    pub barrier_free: Option<bool>,
    pub monitoring_required: Option<bool>,
    pub process_id: Option<EntityId>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub verification_method: Option<String>,
}

impl_identified_header!(FlowRequirement);

impl_patchable!(
    FlowRequirement,
    FlowRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [from_element_id] => from_element_id,
        [to_element_id] => to_element_id,
        [kind] => kind,
        [flow_type] => flow_type,
        [direction] => direction,
        [volume] => volume,
        [peak_rate] => peak_rate,
        [clear_width_m] => clear_width_m,
        [clear_height_m] => clear_height_m,
        [separation_requirements] => separation_requirements,
        [access_level] => access_level,
        [time_windows] => time_windows,
        [equipment_clearance] => equipment_clearance,
        [signage_required] => signage_required,
        [escort_required] => escort_required,
        [emergency_route] => emergency_route,
        [barrier_free] => barrier_free,
        [monitoring_required] => monitoring_required,
        [process_id] => process_id,
        [conflict_ids] => conflict_ids,
        [verification_method] => verification_method,
    }
);
// #endregion

// #region 🔖️AccessRule
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AccessRule {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub subject_ids: Vec<EntityId>,
    pub resource_ids: Vec<EntityId>,
    pub access_level: AccessLevel,
    pub access_mode: AccessMode,
    pub authentication: Vec<String>,
    pub authorization: Vec<String>,
    pub time_restrictions: Vec<String>,
    pub escort_policy: Option<String>,
    pub visitor_policy: Option<String>,
    pub emergency_override: bool,
    pub audit_required: bool,
    pub badge_required: bool,
    pub biometric_required: bool,
    pub zone_ids: Vec<EntityId>,
    pub exceptions: Vec<String>,
    pub regulatory_basis: Vec<String>,
    pub enforcement_method: Option<String>,
    pub revocation_policy: Option<String>,
    pub training_required: bool,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessRulePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_ids: Option<Vec<EntityId>>,
    pub resource_ids: Option<Vec<EntityId>>,
    pub access_level: Option<AccessLevel>,
    pub access_mode: Option<AccessMode>,
    pub authentication: Option<Vec<String>>,
    pub authorization: Option<Vec<String>>,
    pub time_restrictions: Option<Vec<String>>,
    pub escort_policy: Option<String>,
    pub visitor_policy: Option<String>,
    pub emergency_override: Option<bool>,
    pub audit_required: Option<bool>,
    pub badge_required: Option<bool>,
    pub biometric_required: Option<bool>,
    pub zone_ids: Option<Vec<EntityId>>,
    pub exceptions: Option<Vec<String>>,
    pub regulatory_basis: Option<Vec<String>>,
    pub enforcement_method: Option<String>,
    pub revocation_policy: Option<String>,
    pub training_required: Option<bool>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(AccessRule);

impl_patchable!(
    AccessRule,
    AccessRulePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_ids] => subject_ids,
        [resource_ids] => resource_ids,
        [access_level] => access_level,
        [access_mode] => access_mode,
        [authentication] => authentication,
        [authorization] => authorization,
        [time_restrictions] => time_restrictions,
        [escort_policy] => escort_policy,
        [visitor_policy] => visitor_policy,
        [emergency_override] => emergency_override,
        [audit_required] => audit_required,
        [badge_required] => badge_required,
        [biometric_required] => biometric_required,
        [zone_ids] => zone_ids,
        [exceptions] => exceptions,
        [regulatory_basis] => regulatory_basis,
        [enforcement_method] => enforcement_method,
        [revocation_policy] => revocation_policy,
        [training_required] => training_required,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️OperationalRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct OperationalRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub operation: String,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: QuantitySpec,
    pub maintenance_interval: Option<String>,
    pub cleaning_regime: Option<String>,
    pub turnaround_time: Option<String>,
    pub redundancy: Option<String>,
    pub uptime_target: Option<f64>,
    pub response_time: Option<String>,
    pub equipment_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub process_ids: Vec<EntityId>,
    pub utilities: Vec<String>,
    pub waste_streams: Vec<String>,
    pub contingency_plan: Vec<String>,
    pub training_requirements: Vec<String>,
    pub sop_references: Vec<String>,
    pub kpi_targets: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub service_category: Option<String>,
    pub shift_pattern: Option<String>,
    pub sla_target: Option<String>,
    pub escalation_contact_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationalRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub operation: Option<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: Option<QuantitySpec>,
    pub maintenance_interval: Option<String>,
    pub cleaning_regime: Option<String>,
    pub turnaround_time: Option<String>,
    pub redundancy: Option<String>,
    pub uptime_target: Option<f64>,
    pub response_time: Option<String>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub process_ids: Option<Vec<EntityId>>,
    pub utilities: Option<Vec<String>>,
    pub waste_streams: Option<Vec<String>>,
    pub contingency_plan: Option<Vec<String>>,
    pub training_requirements: Option<Vec<String>>,
    pub sop_references: Option<Vec<String>>,
    pub kpi_targets: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub service_category: Option<String>,
    pub shift_pattern: Option<String>,
    pub sla_target: Option<String>,
    pub escalation_contact_id: Option<EntityId>,
}

impl_identified_header!(OperationalRequirement);

impl_patchable!(
    OperationalRequirement,
    OperationalRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [operation] => operation,
        [service_level] => service_level,
        [operating_hours] => operating_hours,
        [staffing] => staffing,
        [maintenance_interval] => maintenance_interval,
        [cleaning_regime] => cleaning_regime,
        [turnaround_time] => turnaround_time,
        [redundancy] => redundancy,
        [uptime_target] => uptime_target,
        [response_time] => response_time,
        [equipment_ids] => equipment_ids,
        [element_ids] => element_ids,
        [process_ids] => process_ids,
        [utilities] => utilities,
        [waste_streams] => waste_streams,
        [contingency_plan] => contingency_plan,
        [training_requirements] => training_requirements,
        [sop_references] => sop_references,
        [kpi_targets] => kpi_targets,
        [owner_id] => owner_id,
        [service_category] => service_category,
        [shift_pattern] => shift_pattern,
        [sla_target] => sla_target,
        [escalation_contact_id] => escalation_contact_id,
    }
);
// #endregion

// #region 🔖️Equipment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub quantity: QuantitySpec,
    pub dimensions: Option<String>,
    #[dsl(unit = "kg")]
    pub weight_kg: Option<f64>,
    #[dsl(unit = "kW")]
    pub power_kw: Option<f64>,
    pub utility_connections: Vec<String>,
    pub ventilation: Option<String>,
    pub noise_level_db: Option<f64>,
    pub clearance: Option<String>,
    pub mounting: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub maintenance_access: Vec<String>,
    pub lifecycle_years: Option<u32>,
    pub replacement_cost: Option<f64>,
    pub standards: Vec<String>,
    pub supplier: Option<String>,
    pub activity_link_ids: Vec<EntityId>,
    pub installation_requirements: Vec<String>,
    pub commissioning_notes: Vec<String>,
    pub spare_parts: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub quantity: Option<QuantitySpec>,
    pub dimensions: Option<String>,
    pub weight_kg: Option<f64>,
    pub power_kw: Option<f64>,
    pub utility_connections: Option<Vec<String>>,
    pub ventilation: Option<String>,
    pub noise_level_db: Option<f64>,
    pub clearance: Option<String>,
    pub mounting: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub maintenance_access: Option<Vec<String>>,
    pub lifecycle_years: Option<u32>,
    pub replacement_cost: Option<f64>,
    pub standards: Option<Vec<String>>,
    pub supplier: Option<String>,
    pub activity_link_ids: Option<Vec<EntityId>>,
    pub installation_requirements: Option<Vec<String>>,
    pub commissioning_notes: Option<Vec<String>>,
    pub spare_parts: Option<Vec<String>>,
}

impl_identified_header!(Equipment);

impl_patchable!(
    Equipment,
    EquipmentPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [manufacturer] => manufacturer,
        [model] => model,
        [quantity] => quantity,
        [dimensions] => dimensions,
        [weight_kg] => weight_kg,
        [power_kw] => power_kw,
        [utility_connections] => utility_connections,
        [ventilation] => ventilation,
        [noise_level_db] => noise_level_db,
        [clearance] => clearance,
        [mounting] => mounting,
        [element_ids] => element_ids,
        [activity_ids] => activity_ids,
        [maintenance_access] => maintenance_access,
        [lifecycle_years] => lifecycle_years,
        [replacement_cost] => replacement_cost,
        [standards] => standards,
        [supplier] => supplier,
        [activity_link_ids] => activity_link_ids,
        [installation_requirements] => installation_requirements,
        [commissioning_notes] => commissioning_notes,
        [spare_parts] => spare_parts,
    }
);
// #endregion

// #region 🔖️Resource
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub resource_type: String,
    pub quantity: QuantitySpec,
    pub mobility: Option<String>,
    pub sharing_model: Option<String>,
    pub allocation: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub user_profile_ids: Vec<EntityId>,
    pub storage_requirement_id: Option<EntityId>,
    pub durability: Option<String>,
    pub cleaning_requirements: Vec<String>,
    pub replacement_cycle: Option<String>,
    pub cost_per_unit: Option<f64>,
    pub supplier: Option<String>,
    pub standards: Vec<String>,
    pub ergonomic_notes: Vec<String>,
    pub customization: Vec<String>,
    pub disposal_notes: Vec<String>,
    pub furniture_class: Option<String>,
    pub ergonomics_rating: Option<String>,
    pub sharing_ratio: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub resource_type: Option<String>,
    pub quantity: Option<QuantitySpec>,
    pub mobility: Option<String>,
    pub sharing_model: Option<String>,
    pub allocation: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub storage_requirement_id: Option<EntityId>,
    pub durability: Option<String>,
    pub cleaning_requirements: Option<Vec<String>>,
    pub replacement_cycle: Option<String>,
    pub cost_per_unit: Option<f64>,
    pub supplier: Option<String>,
    pub standards: Option<Vec<String>>,
    pub ergonomic_notes: Option<Vec<String>>,
    pub customization: Option<Vec<String>>,
    pub disposal_notes: Option<Vec<String>>,
    pub furniture_class: Option<String>,
    pub ergonomics_rating: Option<String>,
    pub sharing_ratio: Option<f64>,
}

impl_identified_header!(Resource);

impl_patchable!(
    Resource,
    ResourcePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [resource_type] => resource_type,
        [quantity] => quantity,
        [mobility] => mobility,
        [sharing_model] => sharing_model,
        [allocation] => allocation,
        [element_ids] => element_ids,
        [activity_ids] => activity_ids,
        [user_profile_ids] => user_profile_ids,
        [storage_requirement_id] => storage_requirement_id,
        [durability] => durability,
        [cleaning_requirements] => cleaning_requirements,
        [replacement_cycle] => replacement_cycle,
        [cost_per_unit] => cost_per_unit,
        [supplier] => supplier,
        [standards] => standards,
        [ergonomic_notes] => ergonomic_notes,
        [customization] => customization,
        [disposal_notes] => disposal_notes,
        [furniture_class] => furniture_class,
        [ergonomics_rating] => ergonomics_rating,
        [sharing_ratio] => sharing_ratio,
    }
);
// #endregion

// #region 🔖️StorageRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct StorageRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub stored_item: String,
    pub storage_class: StorageClass,
    pub quantity: QuantitySpec,
    #[dsl(unit = "m3")]
    pub volume_m3: Option<f64>,
    #[dsl(unit = "kg")]
    pub weight_kg: Option<f64>,
    pub temperature_range: Option<String>,
    pub humidity_range: Option<String>,
    pub security_level: AccessLevel,
    pub hazard_class: Option<String>,
    pub retention_period: Option<String>,
    pub access_frequency: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub equipment_ids: Vec<EntityId>,
    pub handling_equipment: Vec<String>,
    pub fire_protection: Vec<String>,
    pub ventilation: Option<String>,
    pub organization_system: Option<String>,
    pub growth_allowance: Option<f64>,
    pub regulatory_refs: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub stored_item: Option<String>,
    pub storage_class: Option<StorageClass>,
    pub quantity: Option<QuantitySpec>,
    pub volume_m3: Option<f64>,
    pub weight_kg: Option<f64>,
    pub temperature_range: Option<String>,
    pub humidity_range: Option<String>,
    pub security_level: Option<AccessLevel>,
    pub hazard_class: Option<String>,
    pub retention_period: Option<String>,
    pub access_frequency: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub handling_equipment: Option<Vec<String>>,
    pub fire_protection: Option<Vec<String>>,
    pub ventilation: Option<String>,
    pub organization_system: Option<String>,
    pub growth_allowance: Option<f64>,
    pub regulatory_refs: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(StorageRequirement);

impl_patchable!(
    StorageRequirement,
    StorageRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [stored_item] => stored_item,
        [storage_class] => storage_class,
        [quantity] => quantity,
        [volume_m3] => volume_m3,
        [weight_kg] => weight_kg,
        [temperature_range] => temperature_range,
        [humidity_range] => humidity_range,
        [security_level] => security_level,
        [hazard_class] => hazard_class,
        [retention_period] => retention_period,
        [access_frequency] => access_frequency,
        [element_ids] => element_ids,
        [equipment_ids] => equipment_ids,
        [handling_equipment] => handling_equipment,
        [fire_protection] => fire_protection,
        [ventilation] => ventilation,
        [organization_system] => organization_system,
        [growth_allowance] => growth_allowance,
        [regulatory_refs] => regulatory_refs,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️EnvironmentalRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentalRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub parameter_kind: EnvironmentalParameter,
    pub parameter: String,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub comfort_band: Option<String>,
    pub measurement_method: Option<String>,
    pub monitoring_frequency: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub occupancy_basis: Option<String>,
    pub seasonal_variation: Vec<String>,
    pub energy_implications: Vec<String>,
    pub standards: Vec<String>,
    pub certification_targets: Vec<String>,
    pub outdoor_conditions: Vec<String>,
    pub ventilation_strategy: Option<String>,
    pub daylight_target: Option<String>,
    pub acoustic_target: Option<String>,
    pub iaq_target: Option<String>,
    pub verification_plan: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentalRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub parameter_kind: Option<EnvironmentalParameter>,
    pub parameter: Option<String>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub comfort_band: Option<String>,
    pub measurement_method: Option<String>,
    pub monitoring_frequency: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub occupancy_basis: Option<String>,
    pub seasonal_variation: Option<Vec<String>>,
    pub energy_implications: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub certification_targets: Option<Vec<String>>,
    pub outdoor_conditions: Option<Vec<String>>,
    pub ventilation_strategy: Option<String>,
    pub daylight_target: Option<String>,
    pub acoustic_target: Option<String>,
    pub iaq_target: Option<String>,
    pub verification_plan: Option<String>,
}

impl_identified_header!(EnvironmentalRequirement);

impl_patchable!(
    EnvironmentalRequirement,
    EnvironmentalRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [parameter_kind] => parameter_kind,
        [parameter] => parameter,
        [target_value] => target_value,
        [unit] => unit,
        [min_value] => min_value,
        [max_value] => max_value,
        [comfort_band] => comfort_band,
        [measurement_method] => measurement_method,
        [monitoring_frequency] => monitoring_frequency,
        [element_ids] => element_ids,
        [occupancy_basis] => occupancy_basis,
        [seasonal_variation] => seasonal_variation,
        [energy_implications] => energy_implications,
        [standards] => standards,
        [certification_targets] => certification_targets,
        [outdoor_conditions] => outdoor_conditions,
        [ventilation_strategy] => ventilation_strategy,
        [daylight_target] => daylight_target,
        [acoustic_target] => acoustic_target,
        [iaq_target] => iaq_target,
        [verification_plan] => verification_plan,
    }
);
// #endregion

// #region 🔖️HumanFactorRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct HumanFactorRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub aspect: HumanFactorAspect,
    pub factor: String,
    pub user_profile_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub ergonomic_criteria: Vec<String>,
    pub cognitive_load: Option<String>,
    pub visual_demands: Vec<String>,
    pub auditory_demands: Vec<String>,
    pub posture_requirements: Vec<String>,
    pub reach_envelope: Option<String>,
    pub lighting_for_tasks: Vec<String>,
    pub thermal_comfort: Vec<String>,
    pub privacy_needs: Vec<String>,
    pub social_interaction: Vec<String>,
    pub stress_factors: Vec<String>,
    pub mitigation_measures: Vec<String>,
    pub training_needs: Vec<String>,
    pub standards: Vec<String>,
    pub research_basis: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub verification_method: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanFactorRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub aspect: Option<HumanFactorAspect>,
    pub factor: Option<String>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub ergonomic_criteria: Option<Vec<String>>,
    pub cognitive_load: Option<String>,
    pub visual_demands: Option<Vec<String>>,
    pub auditory_demands: Option<Vec<String>>,
    pub posture_requirements: Option<Vec<String>>,
    pub reach_envelope: Option<String>,
    pub lighting_for_tasks: Option<Vec<String>>,
    pub thermal_comfort: Option<Vec<String>>,
    pub privacy_needs: Option<Vec<String>>,
    pub social_interaction: Option<Vec<String>>,
    pub stress_factors: Option<Vec<String>>,
    pub mitigation_measures: Option<Vec<String>>,
    pub training_needs: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub research_basis: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub verification_method: Option<String>,
}

impl_identified_header!(HumanFactorRequirement);

impl_patchable!(
    HumanFactorRequirement,
    HumanFactorRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [aspect] => aspect,
        [factor] => factor,
        [user_profile_ids] => user_profile_ids,
        [activity_ids] => activity_ids,
        [ergonomic_criteria] => ergonomic_criteria,
        [cognitive_load] => cognitive_load,
        [visual_demands] => visual_demands,
        [auditory_demands] => auditory_demands,
        [posture_requirements] => posture_requirements,
        [reach_envelope] => reach_envelope,
        [lighting_for_tasks] => lighting_for_tasks,
        [thermal_comfort] => thermal_comfort,
        [privacy_needs] => privacy_needs,
        [social_interaction] => social_interaction,
        [stress_factors] => stress_factors,
        [mitigation_measures] => mitigation_measures,
        [training_needs] => training_needs,
        [standards] => standards,
        [research_basis] => research_basis,
        [element_ids] => element_ids,
        [verification_method] => verification_method,
    }
);
// #endregion

// #region 🔖️AccessibilityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub standard: String,
    pub level: Option<String>,
    pub user_profile_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub route_ids: Vec<EntityId>,
    #[dsl(unit = "m")]
    pub clear_width_m: Option<f64>,
    #[dsl(unit = "m")]
    pub clear_height_m: Option<f64>,
    #[dsl(unit = "m")]
    pub turning_circle_m: Option<f64>,
    pub ramp_slope: Option<f64>,
    pub lift_required: bool,
    pub tactile_guidance: bool,
    pub hearing_loop: bool,
    pub visual_contrast: bool,
    pub signage_requirements: Vec<String>,
    pub controls_height: Option<String>,
    pub emergency_evacuation: Vec<String>,
    pub service_animal_policy: Option<String>,
    pub companion_seating: bool,
    pub verification_plan: Option<String>,
    pub exceptions: Vec<String>,
    pub wcag_conformance: Option<String>,
    pub universal_design_principles: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub standard: Option<String>,
    pub level: Option<String>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub route_ids: Option<Vec<EntityId>>,
    pub clear_width_m: Option<f64>,
    pub clear_height_m: Option<f64>,
    pub turning_circle_m: Option<f64>,
    pub ramp_slope: Option<f64>,
    pub lift_required: Option<bool>,
    pub tactile_guidance: Option<bool>,
    pub hearing_loop: Option<bool>,
    pub visual_contrast: Option<bool>,
    pub signage_requirements: Option<Vec<String>>,
    pub controls_height: Option<String>,
    pub emergency_evacuation: Option<Vec<String>>,
    pub service_animal_policy: Option<String>,
    pub companion_seating: Option<bool>,
    pub verification_plan: Option<String>,
    pub exceptions: Option<Vec<String>>,
    pub wcag_conformance: Option<String>,
    pub universal_design_principles: Option<Vec<String>>,
}

impl_identified_header!(AccessibilityRequirement);

impl_patchable!(
    AccessibilityRequirement,
    AccessibilityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [standard] => standard,
        [level] => level,
        [user_profile_ids] => user_profile_ids,
        [element_ids] => element_ids,
        [route_ids] => route_ids,
        [clear_width_m] => clear_width_m,
        [clear_height_m] => clear_height_m,
        [turning_circle_m] => turning_circle_m,
        [ramp_slope] => ramp_slope,
        [lift_required] => lift_required,
        [tactile_guidance] => tactile_guidance,
        [hearing_loop] => hearing_loop,
        [visual_contrast] => visual_contrast,
        [signage_requirements] => signage_requirements,
        [controls_height] => controls_height,
        [emergency_evacuation] => emergency_evacuation,
        [service_animal_policy] => service_animal_policy,
        [companion_seating] => companion_seating,
        [verification_plan] => verification_plan,
        [exceptions] => exceptions,
        [wcag_conformance] => wcag_conformance,
        [universal_design_principles] => universal_design_principles,
    }
);
// #endregion

// #region 🔖️PrivacyRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub privacy_kind: PrivacyKind,
    pub privacy_type: String,
    pub level: Option<String>,
    pub subject_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub visual_privacy: Vec<String>,
    pub acoustic_privacy: Vec<String>,
    pub data_privacy: Vec<String>,
    pub screening_required: bool,
    pub enclosure_required: bool,
    pub access_restrictions: Vec<String>,
    pub observation_risk: Option<String>,
    pub regulatory_basis: Vec<String>,
    pub cultural_considerations: Vec<String>,
    pub technology_controls: Vec<String>,
    pub signage: Vec<String>,
    pub monitoring_restrictions: Vec<String>,
    pub retention_policy: Option<String>,
    pub breach_response: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub privacy_kind: Option<PrivacyKind>,
    pub privacy_type: Option<String>,
    pub level: Option<String>,
    pub subject_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub visual_privacy: Option<Vec<String>>,
    pub acoustic_privacy: Option<Vec<String>>,
    pub data_privacy: Option<Vec<String>>,
    pub screening_required: Option<bool>,
    pub enclosure_required: Option<bool>,
    pub access_restrictions: Option<Vec<String>>,
    pub observation_risk: Option<String>,
    pub regulatory_basis: Option<Vec<String>>,
    pub cultural_considerations: Option<Vec<String>>,
    pub technology_controls: Option<Vec<String>>,
    pub signage: Option<Vec<String>>,
    pub monitoring_restrictions: Option<Vec<String>>,
    pub retention_policy: Option<String>,
    pub breach_response: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(PrivacyRequirement);

impl_patchable!(
    PrivacyRequirement,
    PrivacyRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [privacy_kind] => privacy_kind,
        [privacy_type] => privacy_type,
        [level] => level,
        [subject_ids] => subject_ids,
        [element_ids] => element_ids,
        [visual_privacy] => visual_privacy,
        [acoustic_privacy] => acoustic_privacy,
        [data_privacy] => data_privacy,
        [screening_required] => screening_required,
        [enclosure_required] => enclosure_required,
        [access_restrictions] => access_restrictions,
        [observation_risk] => observation_risk,
        [regulatory_basis] => regulatory_basis,
        [cultural_considerations] => cultural_considerations,
        [technology_controls] => technology_controls,
        [signage] => signage,
        [monitoring_restrictions] => monitoring_restrictions,
        [retention_policy] => retention_policy,
        [breach_response] => breach_response,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️SafetyRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub safety_domain: SafetyDomain,
    pub hazard: String,
    pub risk_level: RiskLevel,
    pub affected_element_ids: Vec<EntityId>,
    pub affected_user_ids: Vec<EntityId>,
    pub mitigation_measures: Vec<String>,
    pub ppe_requirements: Vec<String>,
    pub emergency_procedures: Vec<String>,
    pub evacuation_requirements: Vec<String>,
    pub fire_protection: Vec<String>,
    pub structural_safety: Vec<String>,
    pub slip_trip_fall: Vec<String>,
    pub chemical_safety: Vec<String>,
    pub electrical_safety: Vec<String>,
    pub machinery_safety: Vec<String>,
    pub standards: Vec<String>,
    pub inspection_frequency: Option<String>,
    pub training_requirements: Vec<String>,
    pub incident_reporting: Vec<String>,
    pub residual_risk: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub safety_domain: Option<SafetyDomain>,
    pub hazard: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub affected_element_ids: Option<Vec<EntityId>>,
    pub affected_user_ids: Option<Vec<EntityId>>,
    pub mitigation_measures: Option<Vec<String>>,
    pub ppe_requirements: Option<Vec<String>>,
    pub emergency_procedures: Option<Vec<String>>,
    pub evacuation_requirements: Option<Vec<String>>,
    pub fire_protection: Option<Vec<String>>,
    pub structural_safety: Option<Vec<String>>,
    pub slip_trip_fall: Option<Vec<String>>,
    pub chemical_safety: Option<Vec<String>>,
    pub electrical_safety: Option<Vec<String>>,
    pub machinery_safety: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub inspection_frequency: Option<String>,
    pub training_requirements: Option<Vec<String>>,
    pub incident_reporting: Option<Vec<String>>,
    pub residual_risk: Option<String>,
}

impl_identified_header!(SafetyRequirement);

impl_patchable!(
    SafetyRequirement,
    SafetyRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [safety_domain] => safety_domain,
        [hazard] => hazard,
        [risk_level] => risk_level,
        [affected_element_ids] => affected_element_ids,
        [affected_user_ids] => affected_user_ids,
        [mitigation_measures] => mitigation_measures,
        [ppe_requirements] => ppe_requirements,
        [emergency_procedures] => emergency_procedures,
        [evacuation_requirements] => evacuation_requirements,
        [fire_protection] => fire_protection,
        [structural_safety] => structural_safety,
        [slip_trip_fall] => slip_trip_fall,
        [chemical_safety] => chemical_safety,
        [electrical_safety] => electrical_safety,
        [machinery_safety] => machinery_safety,
        [standards] => standards,
        [inspection_frequency] => inspection_frequency,
        [training_requirements] => training_requirements,
        [incident_reporting] => incident_reporting,
        [residual_risk] => residual_risk,
    }
);
// #endregion

// #region 🔖️SecurityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub control_kind: SecurityControlKind,
    pub threat: String,
    pub risk_level: RiskLevel,
    pub asset_ids: Vec<EntityId>,
    pub zone_ids: Vec<EntityId>,
    pub access_level: AccessLevel,
    pub perimeter_controls: Vec<String>,
    pub surveillance: Vec<String>,
    pub intrusion_detection: Vec<String>,
    pub cybersecurity: Vec<String>,
    pub screening: Vec<String>,
    pub visitor_management: Vec<String>,
    pub key_management: Vec<String>,
    pub standards: Vec<String>,
    pub response_procedures: Vec<String>,
    pub drill_frequency: Option<String>,
    pub liaison_contacts: Vec<String>,
    pub classified_level: Option<String>,
    pub redundancy: Vec<String>,
    pub audit_requirements: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub control_kind: Option<SecurityControlKind>,
    pub threat: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub asset_ids: Option<Vec<EntityId>>,
    pub zone_ids: Option<Vec<EntityId>>,
    pub access_level: Option<AccessLevel>,
    pub perimeter_controls: Option<Vec<String>>,
    pub surveillance: Option<Vec<String>>,
    pub intrusion_detection: Option<Vec<String>>,
    pub cybersecurity: Option<Vec<String>>,
    pub screening: Option<Vec<String>>,
    pub visitor_management: Option<Vec<String>>,
    pub key_management: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub response_procedures: Option<Vec<String>>,
    pub drill_frequency: Option<String>,
    pub liaison_contacts: Option<Vec<String>>,
    pub classified_level: Option<String>,
    pub redundancy: Option<Vec<String>>,
    pub audit_requirements: Option<Vec<String>>,
}

impl_identified_header!(SecurityRequirement);

impl_patchable!(
    SecurityRequirement,
    SecurityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [control_kind] => control_kind,
        [threat] => threat,
        [risk_level] => risk_level,
        [asset_ids] => asset_ids,
        [zone_ids] => zone_ids,
        [access_level] => access_level,
        [perimeter_controls] => perimeter_controls,
        [surveillance] => surveillance,
        [intrusion_detection] => intrusion_detection,
        [cybersecurity] => cybersecurity,
        [screening] => screening,
        [visitor_management] => visitor_management,
        [key_management] => key_management,
        [standards] => standards,
        [response_procedures] => response_procedures,
        [drill_frequency] => drill_frequency,
        [liaison_contacts] => liaison_contacts,
        [classified_level] => classified_level,
        [redundancy] => redundancy,
        [audit_requirements] => audit_requirements,
    }
);
// #endregion

// #region 🔖️RegulatoryRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RegulatoryRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub jurisdiction: String,
    pub code: String,
    pub clause: Option<String>,
    pub title: String,
    pub requirement_text: TextField,
    pub applicability: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub compliance_method: Option<String>,
    pub evidence_required: Vec<String>,
    pub authority: Option<String>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub penalties: Vec<String>,
    pub exemptions: Vec<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub interpretation_notes: Vec<TaggedNote>,
    pub verification_status: ValidationStatus,
    pub consultant_refs: Vec<EntityId>,
    pub update_source: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegulatoryRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub jurisdiction: Option<String>,
    pub code: Option<String>,
    pub clause: Option<String>,
    pub title: Option<String>,
    pub requirement_text: Option<TextField>,
    pub applicability: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub compliance_method: Option<String>,
    pub evidence_required: Option<Vec<String>>,
    pub authority: Option<String>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub penalties: Option<Vec<String>>,
    pub exemptions: Option<Vec<String>>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub interpretation_notes: Option<Vec<TaggedNote>>,
    pub verification_status: Option<ValidationStatus>,
    pub consultant_refs: Option<Vec<EntityId>>,
    pub update_source: Option<String>,
}

impl_identified_header!(RegulatoryRequirement);

impl_patchable!(
    RegulatoryRequirement,
    RegulatoryRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [jurisdiction] => jurisdiction,
        [code] => code,
        [clause] => clause,
        [title] => title,
        [requirement_text] => requirement_text,
        [applicability] => applicability,
        [element_ids] => element_ids,
        [compliance_method] => compliance_method,
        [evidence_required] => evidence_required,
        [authority] => authority,
        [effective_date] => effective_date,
        [expiry_date] => expiry_date,
        [penalties] => penalties,
        [exemptions] => exemptions,
        [related_requirement_ids] => related_requirement_ids,
        [interpretation_notes] => interpretation_notes,
        [verification_status] => verification_status,
        [consultant_refs] => consultant_refs,
        [update_source] => update_source,
    }
);
// #endregion

// #region 🔖️SiteContext
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SiteContext {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub site_name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[dsl(unit = "m")]
    pub elevation_m: Option<f64>,
    pub climate_zone: Option<String>,
    pub seismic_zone: Option<String>,
    pub flood_risk: Option<String>,
    pub soil_conditions: Vec<String>,
    pub utilities_available: Vec<String>,
    pub access_roads: Vec<String>,
    pub public_transit: Vec<String>,
    pub neighbors: Vec<String>,
    pub views: Vec<String>,
    pub noise_sources: Vec<String>,
    pub environmental_constraints: Vec<String>,
    pub heritage_constraints: Vec<String>,
    pub zoning: Option<String>,
    #[dsl(unit = "m")]
    pub max_height_m: Option<f64>,
    pub max_coverage: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteContextPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub site_name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation_m: Option<f64>,
    pub climate_zone: Option<String>,
    pub seismic_zone: Option<String>,
    pub flood_risk: Option<String>,
    pub soil_conditions: Option<Vec<String>>,
    pub utilities_available: Option<Vec<String>>,
    pub access_roads: Option<Vec<String>>,
    pub public_transit: Option<Vec<String>>,
    pub neighbors: Option<Vec<String>>,
    pub views: Option<Vec<String>>,
    pub noise_sources: Option<Vec<String>>,
    pub environmental_constraints: Option<Vec<String>>,
    pub heritage_constraints: Option<Vec<String>>,
    pub zoning: Option<String>,
    pub max_height_m: Option<f64>,
    pub max_coverage: Option<f64>,
}

impl_identified_header!(SiteContext);

impl_patchable!(
    SiteContext,
    SiteContextPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [site_name] => site_name,
        [address] => address,
        [latitude] => latitude,
        [longitude] => longitude,
        [elevation_m] => elevation_m,
        [climate_zone] => climate_zone,
        [seismic_zone] => seismic_zone,
        [flood_risk] => flood_risk,
        [soil_conditions] => soil_conditions,
        [utilities_available] => utilities_available,
        [access_roads] => access_roads,
        [public_transit] => public_transit,
        [neighbors] => neighbors,
        [views] => views,
        [noise_sources] => noise_sources,
        [environmental_constraints] => environmental_constraints,
        [heritage_constraints] => heritage_constraints,
        [zoning] => zoning,
        [max_height_m] => max_height_m,
        [max_coverage] => max_coverage,
    }
);
// #endregion

// #region 🔖️OrganizationalRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationalRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub department: String,
    pub reporting_line: Option<String>,
    pub headcount: QuantitySpec,
    pub growth_plan_id: Option<EntityId>,
    pub work_patterns: Vec<String>,
    pub collaboration_model: Option<String>,
    pub hierarchy_levels: Vec<String>,
    pub decision_making: Vec<String>,
    pub culture_notes: Vec<String>,
    pub change_readiness: Option<String>,
    pub union_considerations: Vec<String>,
    pub training_needs: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub service_requirement_ids: Vec<EntityId>,
    pub branding_requirements: Vec<String>,
    pub wellness_plugins: Vec<String>,
    pub diversity_goals: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationalRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub department: Option<String>,
    pub reporting_line: Option<String>,
    pub headcount: Option<QuantitySpec>,
    pub growth_plan_id: Option<EntityId>,
    pub work_patterns: Option<Vec<String>>,
    pub collaboration_model: Option<String>,
    pub hierarchy_levels: Option<Vec<String>>,
    pub decision_making: Option<Vec<String>>,
    pub culture_notes: Option<Vec<String>>,
    pub change_readiness: Option<String>,
    pub union_considerations: Option<Vec<String>>,
    pub training_needs: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub service_requirement_ids: Option<Vec<EntityId>>,
    pub branding_requirements: Option<Vec<String>>,
    pub wellness_plugins: Option<Vec<String>>,
    pub diversity_goals: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(OrganizationalRequirement);

impl_patchable!(
    OrganizationalRequirement,
    OrganizationalRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [department] => department,
        [reporting_line] => reporting_line,
        [headcount] => headcount,
        [growth_plan_id] => growth_plan_id,
        [work_patterns] => work_patterns,
        [collaboration_model] => collaboration_model,
        [hierarchy_levels] => hierarchy_levels,
        [decision_making] => decision_making,
        [culture_notes] => culture_notes,
        [change_readiness] => change_readiness,
        [union_considerations] => union_considerations,
        [training_needs] => training_needs,
        [element_ids] => element_ids,
        [stakeholder_ids] => stakeholder_ids,
        [service_requirement_ids] => service_requirement_ids,
        [branding_requirements] => branding_requirements,
        [wellness_plugins] => wellness_plugins,
        [diversity_goals] => diversity_goals,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️ServiceRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub service_name: String,
    pub service_type: String,
    pub provider: Option<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub capacity: QuantitySpec,
    pub response_time: Option<String>,
    pub queue_management: Vec<String>,
    pub customer_profiles: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub equipment_ids: Vec<EntityId>,
    pub staffing: QuantitySpec,
    pub quality_metrics: Vec<String>,
    pub cost_model: Option<String>,
    pub contract_refs: Vec<String>,
    pub dependencies: Vec<EntityId>,
    pub failure_impact: Option<String>,
    pub backup_service: Vec<String>,
    pub feedback_channels: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub service_name: Option<String>,
    pub service_type: Option<String>,
    pub provider: Option<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub capacity: Option<QuantitySpec>,
    pub response_time: Option<String>,
    pub queue_management: Option<Vec<String>>,
    pub customer_profiles: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub staffing: Option<QuantitySpec>,
    pub quality_metrics: Option<Vec<String>>,
    pub cost_model: Option<String>,
    pub contract_refs: Option<Vec<String>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub failure_impact: Option<String>,
    pub backup_service: Option<Vec<String>>,
    pub feedback_channels: Option<Vec<String>>,
}

impl_identified_header!(ServiceRequirement);

impl_patchable!(
    ServiceRequirement,
    ServiceRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [service_name] => service_name,
        [service_type] => service_type,
        [provider] => provider,
        [service_level] => service_level,
        [operating_hours] => operating_hours,
        [capacity] => capacity,
        [response_time] => response_time,
        [queue_management] => queue_management,
        [customer_profiles] => customer_profiles,
        [element_ids] => element_ids,
        [equipment_ids] => equipment_ids,
        [staffing] => staffing,
        [quality_metrics] => quality_metrics,
        [cost_model] => cost_model,
        [contract_refs] => contract_refs,
        [dependencies] => dependencies,
        [failure_impact] => failure_impact,
        [backup_service] => backup_service,
        [feedback_channels] => feedback_channels,
    }
);
// #endregion

// #region 🔖️InfrastructureRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct InfrastructureRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub system: String,
    pub category: String,
    pub capacity: QuantitySpec,
    pub redundancy: Option<String>,
    pub distribution: Vec<String>,
    pub entry_points: Vec<String>,
    pub utility_source: Option<String>,
    pub standby_power: bool,
    pub monitoring: Vec<String>,
    pub maintenance_access: Vec<String>,
    pub standards: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub peak_demand: Option<f64>,
    pub diversity_factor: Option<f64>,
    pub future_expansion: Vec<String>,
    pub interface_requirements: Vec<String>,
    pub commissioning: Vec<String>,
    pub lifecycle_cost: Option<f64>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfrastructureRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub system: Option<String>,
    pub category: Option<String>,
    pub capacity: Option<QuantitySpec>,
    pub redundancy: Option<String>,
    pub distribution: Option<Vec<String>>,
    pub entry_points: Option<Vec<String>>,
    pub utility_source: Option<String>,
    pub standby_power: Option<bool>,
    pub monitoring: Option<Vec<String>>,
    pub maintenance_access: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub peak_demand: Option<f64>,
    pub diversity_factor: Option<f64>,
    pub future_expansion: Option<Vec<String>>,
    pub interface_requirements: Option<Vec<String>>,
    pub commissioning: Option<Vec<String>>,
    pub lifecycle_cost: Option<f64>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(InfrastructureRequirement);

impl_patchable!(
    InfrastructureRequirement,
    InfrastructureRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [system] => system,
        [category] => category,
        [capacity] => capacity,
        [redundancy] => redundancy,
        [distribution] => distribution,
        [entry_points] => entry_points,
        [utility_source] => utility_source,
        [standby_power] => standby_power,
        [monitoring] => monitoring,
        [maintenance_access] => maintenance_access,
        [standards] => standards,
        [element_ids] => element_ids,
        [peak_demand] => peak_demand,
        [diversity_factor] => diversity_factor,
        [future_expansion] => future_expansion,
        [interface_requirements] => interface_requirements,
        [commissioning] => commissioning,
        [lifecycle_cost] => lifecycle_cost,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️InformationRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct InformationRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub information_type: String,
    pub format: Option<String>,
    pub source_system: Option<String>,
    pub destination_systems: Vec<String>,
    pub update_frequency: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Vec<String>,
    pub classification: Option<String>,
    pub quality_criteria: Vec<String>,
    pub metadata_requirements: Vec<String>,
    pub integration_points: Vec<String>,
    pub backup_requirements: Vec<String>,
    pub disaster_recovery: Vec<String>,
    pub privacy_controls: Vec<String>,
    pub audit_trail: bool,
    pub element_ids: Vec<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub standards: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InformationRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub information_type: Option<String>,
    pub format: Option<String>,
    pub source_system: Option<String>,
    pub destination_systems: Option<Vec<String>>,
    pub update_frequency: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Option<Vec<String>>,
    pub classification: Option<String>,
    pub quality_criteria: Option<Vec<String>>,
    pub metadata_requirements: Option<Vec<String>>,
    pub integration_points: Option<Vec<String>>,
    pub backup_requirements: Option<Vec<String>>,
    pub disaster_recovery: Option<Vec<String>>,
    pub privacy_controls: Option<Vec<String>>,
    pub audit_trail: Option<bool>,
    pub element_ids: Option<Vec<EntityId>>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub standards: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(InformationRequirement);

impl_patchable!(
    InformationRequirement,
    InformationRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [information_type] => information_type,
        [format] => format,
        [source_system] => source_system,
        [destination_systems] => destination_systems,
        [update_frequency] => update_frequency,
        [retention_period] => retention_period,
        [access_controls] => access_controls,
        [classification] => classification,
        [quality_criteria] => quality_criteria,
        [metadata_requirements] => metadata_requirements,
        [integration_points] => integration_points,
        [backup_requirements] => backup_requirements,
        [disaster_recovery] => disaster_recovery,
        [privacy_controls] => privacy_controls,
        [audit_trail] => audit_trail,
        [element_ids] => element_ids,
        [stakeholder_ids] => stakeholder_ids,
        [standards] => standards,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️CommunicationRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub channel: String,
    pub audience_ids: Vec<EntityId>,
    pub message_types: Vec<String>,
    pub frequency: Option<String>,
    pub medium: Vec<String>,
    pub language: Vec<String>,
    pub accessibility: Vec<String>,
    pub emergency_use: bool,
    pub two_way: bool,
    pub recording_policy: Option<String>,
    pub signage_locations: Vec<String>,
    pub technology: Vec<String>,
    pub escalation_path: Vec<String>,
    pub feedback_loop: bool,
    pub privacy_controls: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub standards: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub templates: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub channel: Option<String>,
    pub audience_ids: Option<Vec<EntityId>>,
    pub message_types: Option<Vec<String>>,
    pub frequency: Option<String>,
    pub medium: Option<Vec<String>>,
    pub language: Option<Vec<String>>,
    pub accessibility: Option<Vec<String>>,
    pub emergency_use: Option<bool>,
    pub two_way: Option<bool>,
    pub recording_policy: Option<String>,
    pub signage_locations: Option<Vec<String>>,
    pub technology: Option<Vec<String>>,
    pub escalation_path: Option<Vec<String>>,
    pub feedback_loop: Option<bool>,
    pub privacy_controls: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub standards: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub templates: Option<Vec<EntityId>>,
}

impl_identified_header!(CommunicationRequirement);

impl_patchable!(
    CommunicationRequirement,
    CommunicationRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [channel] => channel,
        [audience_ids] => audience_ids,
        [message_types] => message_types,
        [frequency] => frequency,
        [medium] => medium,
        [language] => language,
        [accessibility] => accessibility,
        [emergency_use] => emergency_use,
        [two_way] => two_way,
        [recording_policy] => recording_policy,
        [signage_locations] => signage_locations,
        [technology] => technology,
        [escalation_path] => escalation_path,
        [feedback_loop] => feedback_loop,
        [privacy_controls] => privacy_controls,
        [element_ids] => element_ids,
        [standards] => standards,
        [owner_id] => owner_id,
        [templates] => templates,
    }
);
// #endregion

// #region 🔖️WayfindingRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WayfindingRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub user_profile_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub destination_types: Vec<String>,
    pub signage_types: Vec<String>,
    pub languages: Vec<String>,
    pub tactile_required: bool,
    pub audio_required: bool,
    pub digital_wayfinding: bool,
    pub landmark_strategy: Vec<String>,
    pub color_coding: Vec<String>,
    pub symbol_standards: Vec<String>,
    pub decision_points: Vec<String>,
    #[dsl(unit = "m")]
    pub maximum_signage_distance_m: Option<f64>,
    pub lighting_requirements: Vec<String>,
    pub maintenance_plan: Option<String>,
    pub emergency_egress: Vec<String>,
    pub visitor_journey: Vec<String>,
    pub staff_journey: Vec<String>,
    pub brand_integration: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WayfindingRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub destination_types: Option<Vec<String>>,
    pub signage_types: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub tactile_required: Option<bool>,
    pub audio_required: Option<bool>,
    pub digital_wayfinding: Option<bool>,
    pub landmark_strategy: Option<Vec<String>>,
    pub color_coding: Option<Vec<String>>,
    pub symbol_standards: Option<Vec<String>>,
    pub decision_points: Option<Vec<String>>,
    pub maximum_signage_distance_m: Option<f64>,
    pub lighting_requirements: Option<Vec<String>>,
    pub maintenance_plan: Option<String>,
    pub emergency_egress: Option<Vec<String>>,
    pub visitor_journey: Option<Vec<String>>,
    pub staff_journey: Option<Vec<String>>,
    pub brand_integration: Option<Vec<String>>,
}

impl_identified_header!(WayfindingRequirement);

impl_patchable!(
    WayfindingRequirement,
    WayfindingRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [user_profile_ids] => user_profile_ids,
        [element_ids] => element_ids,
        [destination_types] => destination_types,
        [signage_types] => signage_types,
        [languages] => languages,
        [tactile_required] => tactile_required,
        [audio_required] => audio_required,
        [digital_wayfinding] => digital_wayfinding,
        [landmark_strategy] => landmark_strategy,
        [color_coding] => color_coding,
        [symbol_standards] => symbol_standards,
        [decision_points] => decision_points,
        [maximum_signage_distance_m] => maximum_signage_distance_m,
        [lighting_requirements] => lighting_requirements,
        [maintenance_plan] => maintenance_plan,
        [emergency_egress] => emergency_egress,
        [visitor_journey] => visitor_journey,
        [staff_journey] => staff_journey,
        [brand_integration] => brand_integration,
    }
);
// #endregion

// #region 🔖️ScheduleRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub milestone: String,
    pub phase: DeliveryPhase,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub duration: Option<String>,
    pub dependencies: Vec<EntityId>,
    pub predecessors: Vec<EntityId>,
    pub successors: Vec<EntityId>,
    pub critical: bool,
    pub float_days: Option<u32>,
    pub resource_requirements: Vec<String>,
    pub occupancy_impact: Vec<String>,
    pub phasing_strategy: Option<String>,
    pub decant_requirements: Vec<String>,
    pub commissioning_window: Option<String>,
    pub stakeholder_ids: Vec<EntityId>,
    pub risk_ids: Vec<EntityId>,
    pub contingency_days: Option<u32>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub milestone: Option<String>,
    pub phase: Option<DeliveryPhase>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub duration: Option<String>,
    pub dependencies: Option<Vec<EntityId>>,
    pub predecessors: Option<Vec<EntityId>>,
    pub successors: Option<Vec<EntityId>>,
    pub critical: Option<bool>,
    pub float_days: Option<u32>,
    pub resource_requirements: Option<Vec<String>>,
    pub occupancy_impact: Option<Vec<String>>,
    pub phasing_strategy: Option<String>,
    pub decant_requirements: Option<Vec<String>>,
    pub commissioning_window: Option<String>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub contingency_days: Option<u32>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(ScheduleRequirement);

impl_patchable!(
    ScheduleRequirement,
    ScheduleRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [milestone] => milestone,
        [phase] => phase,
        [start_date] => start_date,
        [end_date] => end_date,
        [duration] => duration,
        [dependencies] => dependencies,
        [predecessors] => predecessors,
        [successors] => successors,
        [critical] => critical,
        [float_days] => float_days,
        [resource_requirements] => resource_requirements,
        [occupancy_impact] => occupancy_impact,
        [phasing_strategy] => phasing_strategy,
        [decant_requirements] => decant_requirements,
        [commissioning_window] => commissioning_window,
        [stakeholder_ids] => stakeholder_ids,
        [risk_ids] => risk_ids,
        [contingency_days] => contingency_days,
        [reporting_cadence] => reporting_cadence,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️FlexibilityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FlexibilityRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub flexibility_type: String,
    pub element_ids: Vec<EntityId>,
    pub adaptation_scenarios: Vec<String>,
    pub modularity_level: Option<String>,
    pub reconfiguration_time: Option<String>,
    pub cost_of_change: Option<f64>,
    pub technology_readiness: Option<String>,
    pub future_function_ids: Vec<EntityId>,
    pub demountable_partitions: bool,
    pub raised_floor: bool,
    pub overhead_services: bool,
    pub expansion_direction: Vec<String>,
    pub contraction_scenario: Vec<String>,
    pub multi_use_potential: Vec<String>,
    pub furniture_strategy: Vec<String>,
    pub infrastructure_spare_capacity: Vec<String>,
    pub lease_implications: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibilityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub flexibility_type: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub adaptation_scenarios: Option<Vec<String>>,
    pub modularity_level: Option<String>,
    pub reconfiguration_time: Option<String>,
    pub cost_of_change: Option<f64>,
    pub technology_readiness: Option<String>,
    pub future_function_ids: Option<Vec<EntityId>>,
    pub demountable_partitions: Option<bool>,
    pub raised_floor: Option<bool>,
    pub overhead_services: Option<bool>,
    pub expansion_direction: Option<Vec<String>>,
    pub contraction_scenario: Option<Vec<String>>,
    pub multi_use_potential: Option<Vec<String>>,
    pub furniture_strategy: Option<Vec<String>>,
    pub infrastructure_spare_capacity: Option<Vec<String>>,
    pub lease_implications: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(FlexibilityRequirement);

impl_patchable!(
    FlexibilityRequirement,
    FlexibilityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [flexibility_type] => flexibility_type,
        [element_ids] => element_ids,
        [adaptation_scenarios] => adaptation_scenarios,
        [modularity_level] => modularity_level,
        [reconfiguration_time] => reconfiguration_time,
        [cost_of_change] => cost_of_change,
        [technology_readiness] => technology_readiness,
        [future_function_ids] => future_function_ids,
        [demountable_partitions] => demountable_partitions,
        [raised_floor] => raised_floor,
        [overhead_services] => overhead_services,
        [expansion_direction] => expansion_direction,
        [contraction_scenario] => contraction_scenario,
        [multi_use_potential] => multi_use_potential,
        [furniture_strategy] => furniture_strategy,
        [infrastructure_spare_capacity] => infrastructure_spare_capacity,
        [lease_implications] => lease_implications,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️GrowthPlan
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GrowthPlan {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub horizon_years: u32,
    pub growth_rate: Option<f64>,
    pub headcount_growth: QuantitySpec,
    pub area_growth: QuantitySpec,
    pub phases: Vec<String>,
    pub trigger_events: Vec<String>,
    pub expansion_element_ids: Vec<EntityId>,
    pub reserve_areas: Vec<String>,
    pub infrastructure_headroom: Vec<String>,
    pub budget_envelope: Option<f64>,
    pub funding_sources: Vec<String>,
    pub risk_factors: Vec<EntityId>,
    pub decision_points: Vec<EntityId>,
    pub scenario_ids: Vec<EntityId>,
    pub decommission_plan: Vec<String>,
    pub relocation_strategy: Vec<String>,
    pub stakeholder_impact: Vec<String>,
    pub regulatory_considerations: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowthPlanPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub horizon_years: Option<u32>,
    pub growth_rate: Option<f64>,
    pub headcount_growth: Option<QuantitySpec>,
    pub area_growth: Option<QuantitySpec>,
    pub phases: Option<Vec<String>>,
    pub trigger_events: Option<Vec<String>>,
    pub expansion_element_ids: Option<Vec<EntityId>>,
    pub reserve_areas: Option<Vec<String>>,
    pub infrastructure_headroom: Option<Vec<String>>,
    pub budget_envelope: Option<f64>,
    pub funding_sources: Option<Vec<String>>,
    pub risk_factors: Option<Vec<EntityId>>,
    pub decision_points: Option<Vec<EntityId>>,
    pub scenario_ids: Option<Vec<EntityId>>,
    pub decommission_plan: Option<Vec<String>>,
    pub relocation_strategy: Option<Vec<String>>,
    pub stakeholder_impact: Option<Vec<String>>,
    pub regulatory_considerations: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(GrowthPlan);

impl_patchable!(
    GrowthPlan,
    GrowthPlanPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [horizon_years] => horizon_years,
        [growth_rate] => growth_rate,
        [headcount_growth] => headcount_growth,
        [area_growth] => area_growth,
        [phases] => phases,
        [trigger_events] => trigger_events,
        [expansion_element_ids] => expansion_element_ids,
        [reserve_areas] => reserve_areas,
        [infrastructure_headroom] => infrastructure_headroom,
        [budget_envelope] => budget_envelope,
        [funding_sources] => funding_sources,
        [risk_factors] => risk_factors,
        [decision_points] => decision_points,
        [scenario_ids] => scenario_ids,
        [decommission_plan] => decommission_plan,
        [relocation_strategy] => relocation_strategy,
        [stakeholder_impact] => stakeholder_impact,
        [regulatory_considerations] => regulatory_considerations,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️SustainabilityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SustainabilityRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub topic: String,
    pub target: Option<String>,
    pub metric: Option<String>,
    pub baseline: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub certification: Vec<String>,
    pub standards: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub strategies: Vec<String>,
    pub materials_preferences: Vec<String>,
    pub energy_strategy: Vec<String>,
    pub water_strategy: Vec<String>,
    pub waste_strategy: Vec<String>,
    pub biodiversity: Vec<String>,
    pub embodied_carbon: Option<f64>,
    pub operational_carbon: Option<f64>,
    pub reporting_requirements: Vec<String>,
    pub verification_plan: Option<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SustainabilityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub topic: Option<String>,
    pub target: Option<String>,
    pub metric: Option<String>,
    pub baseline: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub certification: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub strategies: Option<Vec<String>>,
    pub materials_preferences: Option<Vec<String>>,
    pub energy_strategy: Option<Vec<String>>,
    pub water_strategy: Option<Vec<String>>,
    pub waste_strategy: Option<Vec<String>>,
    pub biodiversity: Option<Vec<String>>,
    pub embodied_carbon: Option<f64>,
    pub operational_carbon: Option<f64>,
    pub reporting_requirements: Option<Vec<String>>,
    pub verification_plan: Option<String>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(SustainabilityRequirement);

impl_patchable!(
    SustainabilityRequirement,
    SustainabilityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [topic] => topic,
        [target] => target,
        [metric] => metric,
        [baseline] => baseline,
        [target_value] => target_value,
        [unit] => unit,
        [certification] => certification,
        [standards] => standards,
        [element_ids] => element_ids,
        [strategies] => strategies,
        [materials_preferences] => materials_preferences,
        [energy_strategy] => energy_strategy,
        [water_strategy] => water_strategy,
        [waste_strategy] => waste_strategy,
        [biodiversity] => biodiversity,
        [embodied_carbon] => embodied_carbon,
        [operational_carbon] => operational_carbon,
        [reporting_requirements] => reporting_requirements,
        [verification_plan] => verification_plan,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️ResilienceRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ResilienceRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub hazard: String,
    pub risk_level: RiskLevel,
    pub scenario: Option<String>,
    pub recovery_time: Option<String>,
    pub recovery_point: Option<String>,
    pub redundancy: Vec<String>,
    pub hardening_measures: Vec<String>,
    pub backup_systems: Vec<String>,
    pub alternate_sites: Vec<String>,
    pub supply_chain: Vec<String>,
    pub communication_plan: Vec<String>,
    pub drill_requirements: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub infrastructure_ids: Vec<EntityId>,
    pub standards: Vec<String>,
    pub insurance_implications: Vec<String>,
    pub climate_adaptation: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResilienceRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub hazard: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub scenario: Option<String>,
    pub recovery_time: Option<String>,
    pub recovery_point: Option<String>,
    pub redundancy: Option<Vec<String>>,
    pub hardening_measures: Option<Vec<String>>,
    pub backup_systems: Option<Vec<String>>,
    pub alternate_sites: Option<Vec<String>>,
    pub supply_chain: Option<Vec<String>>,
    pub communication_plan: Option<Vec<String>>,
    pub drill_requirements: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub infrastructure_ids: Option<Vec<EntityId>>,
    pub standards: Option<Vec<String>>,
    pub insurance_implications: Option<Vec<String>>,
    pub climate_adaptation: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
}

impl_identified_header!(ResilienceRequirement);

impl_patchable!(
    ResilienceRequirement,
    ResilienceRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [hazard] => hazard,
        [risk_level] => risk_level,
        [scenario] => scenario,
        [recovery_time] => recovery_time,
        [recovery_point] => recovery_point,
        [redundancy] => redundancy,
        [hardening_measures] => hardening_measures,
        [backup_systems] => backup_systems,
        [alternate_sites] => alternate_sites,
        [supply_chain] => supply_chain,
        [communication_plan] => communication_plan,
        [drill_requirements] => drill_requirements,
        [element_ids] => element_ids,
        [infrastructure_ids] => infrastructure_ids,
        [standards] => standards,
        [insurance_implications] => insurance_implications,
        [climate_adaptation] => climate_adaptation,
        [owner_id] => owner_id,
        [verification_plan] => verification_plan,
    }
);
// #endregion

// #region 🔖️CostRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CostRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub cost_item: String,
    pub basis: CostBasis,
    pub amount: Option<f64>,
    pub currency: String,
    pub quantity_basis: Option<String>,
    pub unit_cost: Option<f64>,
    #[dsl(unit = "pct")]
    pub contingency_percent: Option<f64>,
    pub escalation_rate: Option<f64>,
    pub funding_source: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub phase: Option<DeliveryPhase>,
    pub cash_flow_profile: Vec<String>,
    pub value_engineering_notes: Vec<String>,
    pub benchmark_ref: Option<EntityId>,
    pub approval_status: ValidationStatus,
    pub owner_id: Option<EntityId>,
    pub assumptions: Vec<String>,
    pub sensitivity_factors: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub cost_item: Option<String>,
    pub basis: Option<CostBasis>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub quantity_basis: Option<String>,
    pub unit_cost: Option<f64>,
    pub contingency_percent: Option<f64>,
    pub escalation_rate: Option<f64>,
    pub funding_source: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub phase: Option<DeliveryPhase>,
    pub cash_flow_profile: Option<Vec<String>>,
    pub value_engineering_notes: Option<Vec<String>>,
    pub benchmark_ref: Option<EntityId>,
    pub approval_status: Option<ValidationStatus>,
    pub owner_id: Option<EntityId>,
    pub assumptions: Option<Vec<String>>,
    pub sensitivity_factors: Option<Vec<String>>,
}

impl_identified_header!(CostRequirement);

impl_patchable!(
    CostRequirement,
    CostRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [cost_item] => cost_item,
        [basis] => basis,
        [amount] => amount,
        [currency] => currency,
        [quantity_basis] => quantity_basis,
        [unit_cost] => unit_cost,
        [contingency_percent] => contingency_percent,
        [escalation_rate] => escalation_rate,
        [funding_source] => funding_source,
        [element_ids] => element_ids,
        [requirement_ids] => requirement_ids,
        [phase] => phase,
        [cash_flow_profile] => cash_flow_profile,
        [value_engineering_notes] => value_engineering_notes,
        [benchmark_ref] => benchmark_ref,
        [approval_status] => approval_status,
        [owner_id] => owner_id,
        [assumptions] => assumptions,
        [sensitivity_factors] => sensitivity_factors,
    }
);
// #endregion

// #region 🔖️DeliveryConstraint
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryConstraint {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub constraint_type: String,
    pub constraint_details: TextField,
    pub phase: DeliveryPhase,
    pub hard_deadline: Option<String>,
    pub soft_deadline: Option<String>,
    pub impacted_element_ids: Vec<EntityId>,
    pub impacted_requirement_ids: Vec<EntityId>,
    pub work_hours: Option<String>,
    pub noise_restrictions: Vec<String>,
    pub access_restrictions: Vec<String>,
    pub site_logistics: Vec<String>,
    pub procurement_lead_time: Option<String>,
    pub approval_gates: Vec<String>,
    pub occupancy_constraints: Vec<String>,
    pub weather_windows: Vec<String>,
    pub penalty_clauses: Vec<String>,
    pub mitigation_options: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub risk_ids: Vec<EntityId>,
    pub constraint_status: LifecycleStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryConstraintPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub constraint_type: Option<String>,
    pub constraint_details: Option<TextField>,
    pub phase: Option<DeliveryPhase>,
    pub hard_deadline: Option<String>,
    pub soft_deadline: Option<String>,
    pub impacted_element_ids: Option<Vec<EntityId>>,
    pub impacted_requirement_ids: Option<Vec<EntityId>>,
    pub work_hours: Option<String>,
    pub noise_restrictions: Option<Vec<String>>,
    pub access_restrictions: Option<Vec<String>>,
    pub site_logistics: Option<Vec<String>>,
    pub procurement_lead_time: Option<String>,
    pub approval_gates: Option<Vec<String>>,
    pub occupancy_constraints: Option<Vec<String>>,
    pub weather_windows: Option<Vec<String>>,
    pub penalty_clauses: Option<Vec<String>>,
    pub mitigation_options: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub constraint_status: Option<LifecycleStatus>,
}

impl_identified_header!(DeliveryConstraint);

impl_patchable!(
    DeliveryConstraint,
    DeliveryConstraintPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [constraint_type] => constraint_type,
        [constraint_details] => constraint_details,
        [phase] => phase,
        [hard_deadline] => hard_deadline,
        [soft_deadline] => soft_deadline,
        [impacted_element_ids] => impacted_element_ids,
        [impacted_requirement_ids] => impacted_requirement_ids,
        [work_hours] => work_hours,
        [noise_restrictions] => noise_restrictions,
        [access_restrictions] => access_restrictions,
        [site_logistics] => site_logistics,
        [procurement_lead_time] => procurement_lead_time,
        [approval_gates] => approval_gates,
        [occupancy_constraints] => occupancy_constraints,
        [weather_windows] => weather_windows,
        [penalty_clauses] => penalty_clauses,
        [mitigation_options] => mitigation_options,
        [owner_id] => owner_id,
        [risk_ids] => risk_ids,
        [constraint_status] => constraint_status,
    }
);
// #endregion

// #region 🔖️Risk
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub risk_statement: TextField,
    pub category: String,
    pub probability: RiskLevel,
    pub impact: RiskLevel,
    pub risk_score: Option<f64>,
    pub causes: Vec<String>,
    pub effects: Vec<String>,
    pub affected_element_ids: Vec<EntityId>,
    pub affected_requirement_ids: Vec<EntityId>,
    pub mitigation: Vec<String>,
    pub contingency: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub review_date: Option<String>,
    pub trigger_indicators: Vec<String>,
    pub residual_probability: Option<RiskLevel>,
    pub residual_impact: Option<RiskLevel>,
    pub related_conflict_ids: Vec<EntityId>,
    pub escalation_path: Vec<String>,
    pub monitoring_plan: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub risk_statement: Option<TextField>,
    pub category: Option<String>,
    pub probability: Option<RiskLevel>,
    pub impact: Option<RiskLevel>,
    pub risk_score: Option<f64>,
    pub causes: Option<Vec<String>>,
    pub effects: Option<Vec<String>>,
    pub affected_element_ids: Option<Vec<EntityId>>,
    pub affected_requirement_ids: Option<Vec<EntityId>>,
    pub mitigation: Option<Vec<String>>,
    pub contingency: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub review_date: Option<String>,
    pub trigger_indicators: Option<Vec<String>>,
    pub residual_probability: Option<RiskLevel>,
    pub residual_impact: Option<RiskLevel>,
    pub related_conflict_ids: Option<Vec<EntityId>>,
    pub escalation_path: Option<Vec<String>>,
    pub monitoring_plan: Option<String>,
}

impl_identified_header!(Risk);

impl_patchable!(
    Risk,
    RiskPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [risk_statement] => risk_statement,
        [category] => category,
        [probability] => probability,
        [impact] => impact,
        [risk_score] => risk_score,
        [causes] => causes,
        [effects] => effects,
        [affected_element_ids] => affected_element_ids,
        [affected_requirement_ids] => affected_requirement_ids,
        [mitigation] => mitigation,
        [contingency] => contingency,
        [owner_id] => owner_id,
        [review_date] => review_date,
        [trigger_indicators] => trigger_indicators,
        [residual_probability] => residual_probability,
        [residual_impact] => residual_impact,
        [related_conflict_ids] => related_conflict_ids,
        [escalation_path] => escalation_path,
        [monitoring_plan] => monitoring_plan,
    }
);
// #endregion

// #region 🔖️Conflict
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Conflict {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub kind: ConflictKind,
    pub summary: TextField,
    pub entity_a_id: EntityId,
    pub entity_b_id: EntityId,
    pub severity: IssueSeverity,
    pub detected_by: Option<String>,
    pub detection_date: Option<String>,
    pub trade_off_options: Vec<String>,
    pub recommended_resolution: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub quality_impact: Vec<String>,
    pub resolution_status: ValidationStatus,
    pub owner_id: Option<EntityId>,
    pub escalation_level: Option<String>,
    pub related_risk_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub kind: Option<ConflictKind>,
    pub summary: Option<TextField>,
    pub entity_a_id: Option<EntityId>,
    pub entity_b_id: Option<EntityId>,
    pub severity: Option<IssueSeverity>,
    pub detected_by: Option<String>,
    pub detection_date: Option<String>,
    pub trade_off_options: Option<Vec<String>>,
    pub recommended_resolution: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub quality_impact: Option<Vec<String>>,
    pub resolution_status: Option<ValidationStatus>,
    pub owner_id: Option<EntityId>,
    pub escalation_level: Option<String>,
    pub related_risk_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(Conflict);

impl_patchable!(
    Conflict,
    ConflictPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [kind] => kind,
        [summary] => summary,
        [entity_a_id] => entity_a_id,
        [entity_b_id] => entity_b_id,
        [severity] => severity,
        [detected_by] => detected_by,
        [detection_date] => detection_date,
        [trade_off_options] => trade_off_options,
        [recommended_resolution] => recommended_resolution,
        [decision_id] => decision_id,
        [stakeholder_ids] => stakeholder_ids,
        [requirement_ids] => requirement_ids,
        [cost_impact] => cost_impact,
        [schedule_impact] => schedule_impact,
        [quality_impact] => quality_impact,
        [resolution_status] => resolution_status,
        [owner_id] => owner_id,
        [escalation_level] => escalation_level,
        [related_risk_ids] => related_risk_ids,
    }
);
// #endregion

// #region 🔖️Requirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub kind: RequirementKind,
    pub statement: TextField,
    pub rationale: Option<TextField>,
    pub source: Option<String>,
    pub stakeholder_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub function_ids: Vec<EntityId>,
    pub parent_requirement_id: Option<EntityId>,
    pub child_requirement_ids: Vec<EntityId>,
    pub acceptance_criteria: Vec<String>,
    pub verification_method: Option<String>,
    pub validation_status: ValidationStatus,
    pub conflict_ids: Vec<EntityId>,
    pub risk_ids: Vec<EntityId>,
    pub cost_estimate: Option<f64>,
    pub schedule_constraint: Option<String>,
    pub regulatory_refs: Vec<String>,
    pub trace_links: Vec<TraceLink>,
    pub superseded_by: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub kind: Option<RequirementKind>,
    pub statement: Option<TextField>,
    pub rationale: Option<TextField>,
    pub source: Option<String>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub function_ids: Option<Vec<EntityId>>,
    pub parent_requirement_id: Option<EntityId>,
    pub child_requirement_ids: Option<Vec<EntityId>>,
    pub acceptance_criteria: Option<Vec<String>>,
    pub verification_method: Option<String>,
    pub validation_status: Option<ValidationStatus>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub cost_estimate: Option<f64>,
    pub schedule_constraint: Option<String>,
    pub regulatory_refs: Option<Vec<String>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub superseded_by: Option<EntityId>,
}

impl_identified_header!(Requirement);

impl_patchable!(
    Requirement,
    RequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [kind] => kind,
        [statement] => statement,
        [rationale] => rationale,
        [source] => source,
        [stakeholder_ids] => stakeholder_ids,
        [element_ids] => element_ids,
        [function_ids] => function_ids,
        [parent_requirement_id] => parent_requirement_id,
        [child_requirement_ids] => child_requirement_ids,
        [acceptance_criteria] => acceptance_criteria,
        [verification_method] => verification_method,
        [validation_status] => validation_status,
        [conflict_ids] => conflict_ids,
        [risk_ids] => risk_ids,
        [cost_estimate] => cost_estimate,
        [schedule_constraint] => schedule_constraint,
        [regulatory_refs] => regulatory_refs,
        [trace_links] => trace_links,
        [superseded_by] => superseded_by,
    }
);
// #endregion

// #region 🔖️PriorityRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PriorityRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub ranked_priority: Priority,
    pub rank: Option<u32>,
    pub weight: Option<f64>,
    pub rationale: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
    pub review_cycle: Option<String>,
    pub dependencies: Vec<EntityId>,
    pub conflicts: Vec<EntityId>,
    pub scoring_method: Option<String>,
    pub score: Option<f64>,
    pub criteria: Vec<String>,
    pub approved_by: Option<EntityId>,
    pub approval_date: Option<String>,
    pub ranking_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub ranked_priority: Option<Priority>,
    pub rank: Option<u32>,
    pub weight: Option<f64>,
    pub rationale: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
    pub review_cycle: Option<String>,
    pub dependencies: Option<Vec<EntityId>>,
    pub conflicts: Option<Vec<EntityId>>,
    pub scoring_method: Option<String>,
    pub score: Option<f64>,
    pub criteria: Option<Vec<String>>,
    pub approved_by: Option<EntityId>,
    pub approval_date: Option<String>,
    pub ranking_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(PriorityRecord);

impl_patchable!(
    PriorityRecord,
    PriorityRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [ranked_priority] => ranked_priority,
        [rank] => rank,
        [weight] => weight,
        [rationale] => rationale,
        [decision_id] => decision_id,
        [stakeholder_ids] => stakeholder_ids,
        [effective_from] => effective_from,
        [effective_until] => effective_until,
        [review_cycle] => review_cycle,
        [dependencies] => dependencies,
        [conflicts] => conflicts,
        [scoring_method] => scoring_method,
        [score] => score,
        [criteria] => criteria,
        [approved_by] => approved_by,
        [approval_date] => approval_date,
        [ranking_notes] => ranking_notes,
    }
);
// #endregion

// #region 🔖️Scenario
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub code: String,
    pub hypothesis: TextField,
    pub assumptions: Vec<String>,
    pub variables: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub growth_plan_id: Option<EntityId>,
    pub probability: Option<f64>,
    pub impact_summary: Option<TextField>,
    pub cost_delta: Option<f64>,
    pub area_delta: Option<f64>,
    pub headcount_delta: Option<f64>,
    pub schedule_delta: Option<String>,
    pub risk_ids: Vec<EntityId>,
    pub option_ids: Vec<EntityId>,
    pub baseline: bool,
    pub preferred: bool,
    pub analysis_ids: Vec<EntityId>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub hypothesis: Option<TextField>,
    pub assumptions: Option<Vec<String>>,
    pub variables: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub growth_plan_id: Option<EntityId>,
    pub probability: Option<f64>,
    pub impact_summary: Option<TextField>,
    pub cost_delta: Option<f64>,
    pub area_delta: Option<f64>,
    pub headcount_delta: Option<f64>,
    pub schedule_delta: Option<String>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub option_ids: Option<Vec<EntityId>>,
    pub baseline: Option<bool>,
    pub preferred: Option<bool>,
    pub analysis_ids: Option<Vec<EntityId>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(Scenario);

impl_patchable!(
    Scenario,
    ScenarioPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [hypothesis] => hypothesis,
        [assumptions] => assumptions,
        [variables] => variables,
        [element_ids] => element_ids,
        [requirement_ids] => requirement_ids,
        [growth_plan_id] => growth_plan_id,
        [probability] => probability,
        [impact_summary] => impact_summary,
        [cost_delta] => cost_delta,
        [area_delta] => area_delta,
        [headcount_delta] => headcount_delta,
        [schedule_delta] => schedule_delta,
        [risk_ids] => risk_ids,
        [option_ids] => option_ids,
        [baseline] => baseline,
        [preferred] => preferred,
        [analysis_ids] => analysis_ids,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️OptionEvaluation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct OptionEvaluation {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub option_name: String,
    pub option_description: TextField,
    pub scenario_id: Option<EntityId>,
    pub criteria_ids: Vec<EntityId>,
    pub scores: Vec<f64>,
    pub weighted_score: Option<f64>,
    pub cost_estimate: Option<f64>,
    pub schedule_estimate: Option<String>,
    pub risk_summary: Vec<String>,
    pub benefits: Vec<String>,
    pub drawbacks: Vec<String>,
    pub assumptions: Vec<String>,
    pub dependencies: Vec<EntityId>,
    pub stakeholder_feedback: Vec<TaggedNote>,
    pub recommendation: Option<String>,
    pub decision_id: Option<EntityId>,
    pub evaluation_status: ValidationStatus,
    pub evaluator_ids: Vec<EntityId>,
    pub evaluation_date: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionEvaluationPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub option_name: Option<String>,
    pub option_description: Option<TextField>,
    pub scenario_id: Option<EntityId>,
    pub criteria_ids: Option<Vec<EntityId>>,
    pub scores: Option<Vec<f64>>,
    pub weighted_score: Option<f64>,
    pub cost_estimate: Option<f64>,
    pub schedule_estimate: Option<String>,
    pub risk_summary: Option<Vec<String>>,
    pub benefits: Option<Vec<String>>,
    pub drawbacks: Option<Vec<String>>,
    pub assumptions: Option<Vec<String>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub stakeholder_feedback: Option<Vec<TaggedNote>>,
    pub recommendation: Option<String>,
    pub decision_id: Option<EntityId>,
    pub evaluation_status: Option<ValidationStatus>,
    pub evaluator_ids: Option<Vec<EntityId>>,
    pub evaluation_date: Option<String>,
}

impl_identified_header!(OptionEvaluation);

impl_patchable!(
    OptionEvaluation,
    OptionEvaluationPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [option_name] => option_name,
        [option_description] => option_description,
        [scenario_id] => scenario_id,
        [criteria_ids] => criteria_ids,
        [scores] => scores,
        [weighted_score] => weighted_score,
        [cost_estimate] => cost_estimate,
        [schedule_estimate] => schedule_estimate,
        [risk_summary] => risk_summary,
        [benefits] => benefits,
        [drawbacks] => drawbacks,
        [assumptions] => assumptions,
        [dependencies] => dependencies,
        [stakeholder_feedback] => stakeholder_feedback,
        [recommendation] => recommendation,
        [decision_id] => decision_id,
        [evaluation_status] => evaluation_status,
        [evaluator_ids] => evaluator_ids,
        [evaluation_date] => evaluation_date,
    }
);
// #endregion

// #region 🔖️Decision
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Decision {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub decision_statement: TextField,
    pub context: TextField,
    pub options_considered: Vec<EntityId>,
    pub selected_option_id: Option<EntityId>,
    pub rationale: TextField,
    pub decision_maker_ids: Vec<EntityId>,
    pub consulted_ids: Vec<EntityId>,
    pub informed_ids: Vec<EntityId>,
    pub decision_date: Option<String>,
    pub effective_date: Option<String>,
    pub reversal_conditions: Vec<String>,
    pub impacted_requirement_ids: Vec<EntityId>,
    pub impacted_element_ids: Vec<EntityId>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Vec<String>,
    pub approval_status: ValidationStatus,
    pub meeting_ref: Option<EntityId>,
    pub document_refs: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub decision_statement: Option<TextField>,
    pub context: Option<TextField>,
    pub options_considered: Option<Vec<EntityId>>,
    pub selected_option_id: Option<EntityId>,
    pub rationale: Option<TextField>,
    pub decision_maker_ids: Option<Vec<EntityId>>,
    pub consulted_ids: Option<Vec<EntityId>>,
    pub informed_ids: Option<Vec<EntityId>>,
    pub decision_date: Option<String>,
    pub effective_date: Option<String>,
    pub reversal_conditions: Option<Vec<String>>,
    pub impacted_requirement_ids: Option<Vec<EntityId>>,
    pub impacted_element_ids: Option<Vec<EntityId>>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Option<Vec<String>>,
    pub approval_status: Option<ValidationStatus>,
    pub meeting_ref: Option<EntityId>,
    pub document_refs: Option<Vec<EntityId>>,
}

impl_identified_header!(Decision);

impl_patchable!(
    Decision,
    DecisionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [decision_statement] => decision_statement,
        [context] => context,
        [options_considered] => options_considered,
        [selected_option_id] => selected_option_id,
        [rationale] => rationale,
        [decision_maker_ids] => decision_maker_ids,
        [consulted_ids] => consulted_ids,
        [informed_ids] => informed_ids,
        [decision_date] => decision_date,
        [effective_date] => effective_date,
        [reversal_conditions] => reversal_conditions,
        [impacted_requirement_ids] => impacted_requirement_ids,
        [impacted_element_ids] => impacted_element_ids,
        [cost_impact] => cost_impact,
        [schedule_impact] => schedule_impact,
        [risk_impact] => risk_impact,
        [approval_status] => approval_status,
        [meeting_ref] => meeting_ref,
        [document_refs] => document_refs,
    }
);
// #endregion

// #region 🔖️ValidationRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub validation_type: String,
    pub method: Option<String>,
    pub criteria: Vec<String>,
    pub result: ValidationStatus,
    pub evidence: Vec<String>,
    pub validator_ids: Vec<EntityId>,
    pub validation_date: Option<String>,
    pub next_review_date: Option<String>,
    pub findings: Vec<String>,
    pub non_conformities: Vec<String>,
    pub corrective_actions: Vec<String>,
    pub waivers: Vec<String>,
    pub standards: Vec<String>,
    pub trace_links: Vec<TraceLink>,
    pub report_id: Option<EntityId>,
    pub confidence_level: Option<String>,
    pub validation_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub validation_type: Option<String>,
    pub method: Option<String>,
    pub criteria: Option<Vec<String>>,
    pub result: Option<ValidationStatus>,
    pub evidence: Option<Vec<String>>,
    pub validator_ids: Option<Vec<EntityId>>,
    pub validation_date: Option<String>,
    pub next_review_date: Option<String>,
    pub findings: Option<Vec<String>>,
    pub non_conformities: Option<Vec<String>>,
    pub corrective_actions: Option<Vec<String>>,
    pub waivers: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub report_id: Option<EntityId>,
    pub confidence_level: Option<String>,
    pub validation_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(ValidationRecord);

impl_patchable!(
    ValidationRecord,
    ValidationRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [validation_type] => validation_type,
        [method] => method,
        [criteria] => criteria,
        [result] => result,
        [evidence] => evidence,
        [validator_ids] => validator_ids,
        [validation_date] => validation_date,
        [next_review_date] => next_review_date,
        [findings] => findings,
        [non_conformities] => non_conformities,
        [corrective_actions] => corrective_actions,
        [waivers] => waivers,
        [standards] => standards,
        [trace_links] => trace_links,
        [report_id] => report_id,
        [confidence_level] => confidence_level,
        [validation_notes] => validation_notes,
    }
);
// #endregion

// #region 🔖️PerformanceCriterion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceCriterion {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub criterion: String,
    pub metric: String,
    pub target: Option<f64>,
    pub unit: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub measurement_method: Option<String>,
    pub frequency: Option<String>,
    pub requirement_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub baseline: Option<f64>,
    pub benchmark_ref: Option<EntityId>,
    pub weight: Option<f64>,
    pub data_source: Option<String>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
    pub penalty_threshold: Option<f64>,
    pub incentive_threshold: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceCriterionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub criterion: Option<String>,
    pub metric: Option<String>,
    pub target: Option<f64>,
    pub unit: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub measurement_method: Option<String>,
    pub frequency: Option<String>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub baseline: Option<f64>,
    pub benchmark_ref: Option<EntityId>,
    pub weight: Option<f64>,
    pub data_source: Option<String>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
    pub penalty_threshold: Option<f64>,
    pub incentive_threshold: Option<f64>,
}

impl_identified_header!(PerformanceCriterion);

impl_patchable!(
    PerformanceCriterion,
    PerformanceCriterionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [criterion] => criterion,
        [metric] => metric,
        [target] => target,
        [unit] => unit,
        [minimum] => minimum,
        [maximum] => maximum,
        [measurement_method] => measurement_method,
        [frequency] => frequency,
        [requirement_ids] => requirement_ids,
        [element_ids] => element_ids,
        [baseline] => baseline,
        [benchmark_ref] => benchmark_ref,
        [weight] => weight,
        [data_source] => data_source,
        [reporting_cadence] => reporting_cadence,
        [owner_id] => owner_id,
        [verification_plan] => verification_plan,
        [penalty_threshold] => penalty_threshold,
        [incentive_threshold] => incentive_threshold,
    }
);
// #endregion

// #region 🔖️QualityRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct QualityRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub quality_topic: String,
    pub standard: Option<String>,
    pub target_level: Option<String>,
    pub inspection_points: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub testing_requirements: Vec<String>,
    pub sample_rate: Option<String>,
    pub defect_categories: Vec<String>,
    pub corrective_action_process: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub supplier_requirements: Vec<String>,
    pub documentation_requirements: Vec<String>,
    pub training_requirements: Vec<String>,
    pub audit_schedule: Option<String>,
    pub kpis: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub certification_targets: Vec<String>,
    pub continuous_improvement: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub quality_topic: Option<String>,
    pub standard: Option<String>,
    pub target_level: Option<String>,
    pub inspection_points: Option<Vec<String>>,
    pub acceptance_criteria: Option<Vec<String>>,
    pub testing_requirements: Option<Vec<String>>,
    pub sample_rate: Option<String>,
    pub defect_categories: Option<Vec<String>>,
    pub corrective_action_process: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub supplier_requirements: Option<Vec<String>>,
    pub documentation_requirements: Option<Vec<String>>,
    pub training_requirements: Option<Vec<String>>,
    pub audit_schedule: Option<String>,
    pub kpis: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub certification_targets: Option<Vec<String>>,
    pub continuous_improvement: Option<Vec<String>>,
}

impl_identified_header!(QualityRecord);

impl_patchable!(
    QualityRecord,
    QualityRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [quality_topic] => quality_topic,
        [standard] => standard,
        [target_level] => target_level,
        [inspection_points] => inspection_points,
        [acceptance_criteria] => acceptance_criteria,
        [testing_requirements] => testing_requirements,
        [sample_rate] => sample_rate,
        [defect_categories] => defect_categories,
        [corrective_action_process] => corrective_action_process,
        [element_ids] => element_ids,
        [requirement_ids] => requirement_ids,
        [supplier_requirements] => supplier_requirements,
        [documentation_requirements] => documentation_requirements,
        [training_requirements] => training_requirements,
        [audit_schedule] => audit_schedule,
        [kpis] => kpis,
        [owner_id] => owner_id,
        [certification_targets] => certification_targets,
        [continuous_improvement] => continuous_improvement,
    }
);
// #endregion

// #region 🔖️DocumentRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub document_type: String,
    pub title: String,
    pub version: String,
    pub file_ref: Option<String>,
    pub format: Option<String>,
    pub author_ids: Vec<EntityId>,
    pub reviewer_ids: Vec<EntityId>,
    pub approver_ids: Vec<EntityId>,
    pub issue_date: Option<String>,
    pub revision_date: Option<String>,
    pub distribution_list: Vec<EntityId>,
    pub related_entity_ids: Vec<EntityId>,
    pub classification: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Vec<String>,
    pub supersedes: Option<EntityId>,
    pub document_status: LifecycleStatus,
    pub checksum: Option<String>,
    pub source_system: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub document_type: Option<String>,
    pub title: Option<String>,
    pub version: Option<String>,
    pub file_ref: Option<String>,
    pub format: Option<String>,
    pub author_ids: Option<Vec<EntityId>>,
    pub reviewer_ids: Option<Vec<EntityId>>,
    pub approver_ids: Option<Vec<EntityId>>,
    pub issue_date: Option<String>,
    pub revision_date: Option<String>,
    pub distribution_list: Option<Vec<EntityId>>,
    pub related_entity_ids: Option<Vec<EntityId>>,
    pub classification: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Option<Vec<String>>,
    pub supersedes: Option<EntityId>,
    pub document_status: Option<LifecycleStatus>,
    pub checksum: Option<String>,
    pub source_system: Option<String>,
}

impl_identified_header!(DocumentRecord);

impl_patchable!(
    DocumentRecord,
    DocumentRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [document_type] => document_type,
        [title] => title,
        [version] => version,
        [file_ref] => file_ref,
        [format] => format,
        [author_ids] => author_ids,
        [reviewer_ids] => reviewer_ids,
        [approver_ids] => approver_ids,
        [issue_date] => issue_date,
        [revision_date] => revision_date,
        [distribution_list] => distribution_list,
        [related_entity_ids] => related_entity_ids,
        [classification] => classification,
        [retention_period] => retention_period,
        [access_controls] => access_controls,
        [supersedes] => supersedes,
        [document_status] => document_status,
        [checksum] => checksum,
        [source_system] => source_system,
    }
);
// #endregion

// #region 🔖️ChangeRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub change_type: String,
    pub summary: TextField,
    pub reason: TextField,
    pub requested_by: Option<EntityId>,
    pub approved_by: Option<EntityId>,
    pub change_date: Option<String>,
    pub effective_date: Option<String>,
    pub impacted_entity_ids: Vec<EntityId>,
    pub before_snapshot: Option<String>,
    pub after_snapshot: Option<String>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Vec<String>,
    pub approval_status: ValidationStatus,
    pub rollback_plan: Vec<String>,
    pub communication_plan: Vec<String>,
    pub version_from: Option<String>,
    pub version_to: Option<String>,
    pub audit_event_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub change_type: Option<String>,
    pub summary: Option<TextField>,
    pub reason: Option<TextField>,
    pub requested_by: Option<EntityId>,
    pub approved_by: Option<EntityId>,
    pub change_date: Option<String>,
    pub effective_date: Option<String>,
    pub impacted_entity_ids: Option<Vec<EntityId>>,
    pub before_snapshot: Option<String>,
    pub after_snapshot: Option<String>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Option<Vec<String>>,
    pub approval_status: Option<ValidationStatus>,
    pub rollback_plan: Option<Vec<String>>,
    pub communication_plan: Option<Vec<String>>,
    pub version_from: Option<String>,
    pub version_to: Option<String>,
    pub audit_event_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(ChangeRecord);

impl_patchable!(
    ChangeRecord,
    ChangeRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [change_type] => change_type,
        [summary] => summary,
        [reason] => reason,
        [requested_by] => requested_by,
        [approved_by] => approved_by,
        [change_date] => change_date,
        [effective_date] => effective_date,
        [impacted_entity_ids] => impacted_entity_ids,
        [before_snapshot] => before_snapshot,
        [after_snapshot] => after_snapshot,
        [cost_impact] => cost_impact,
        [schedule_impact] => schedule_impact,
        [risk_impact] => risk_impact,
        [approval_status] => approval_status,
        [rollback_plan] => rollback_plan,
        [communication_plan] => communication_plan,
        [version_from] => version_from,
        [version_to] => version_to,
        [audit_event_ids] => audit_event_ids,
    }
);
// #endregion

// #region 🔖️CollaborationRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub session_type: String,
    pub title: String,
    pub participants: Vec<EntityId>,
    pub facilitator_id: Option<EntityId>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub agenda: Vec<String>,
    pub outcomes: Vec<String>,
    pub action_items: Vec<String>,
    pub decision_ids: Vec<EntityId>,
    pub issue_ids: Vec<EntityId>,
    pub document_ids: Vec<EntityId>,
    pub recording_ref: Option<String>,
    pub feedback: Vec<TaggedNote>,
    pub follow_up_date: Option<String>,
    pub workshop_id: Option<EntityId>,
    pub survey_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub session_type: Option<String>,
    pub title: Option<String>,
    pub participants: Option<Vec<EntityId>>,
    pub facilitator_id: Option<EntityId>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub agenda: Option<Vec<String>>,
    pub outcomes: Option<Vec<String>>,
    pub action_items: Option<Vec<String>>,
    pub decision_ids: Option<Vec<EntityId>>,
    pub issue_ids: Option<Vec<EntityId>>,
    pub document_ids: Option<Vec<EntityId>>,
    pub recording_ref: Option<String>,
    pub feedback: Option<Vec<TaggedNote>>,
    pub follow_up_date: Option<String>,
    pub workshop_id: Option<EntityId>,
    pub survey_id: Option<EntityId>,
}

impl_identified_header!(CollaborationRecord);

impl_patchable!(
    CollaborationRecord,
    CollaborationRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [session_type] => session_type,
        [title] => title,
        [participants] => participants,
        [facilitator_id] => facilitator_id,
        [start_time] => start_time,
        [end_time] => end_time,
        [location] => location,
        [agenda] => agenda,
        [outcomes] => outcomes,
        [action_items] => action_items,
        [decision_ids] => decision_ids,
        [issue_ids] => issue_ids,
        [document_ids] => document_ids,
        [recording_ref] => recording_ref,
        [feedback] => feedback,
        [follow_up_date] => follow_up_date,
        [workshop_id] => workshop_id,
        [survey_id] => survey_id,
    }
);
// #endregion

// #region 🔖️AnalysisRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub kind: AnalysisKind,
    pub title: String,
    pub parameters: Vec<String>,
    pub input_entity_ids: Vec<EntityId>,
    pub output_summary: TextField,
    pub findings: Vec<String>,
    pub metrics: Vec<String>,
    pub charts: Vec<String>,
    pub run_by: Option<EntityId>,
    pub run_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub tool_version: Option<String>,
    pub scenario_id: Option<EntityId>,
    pub report_id: Option<EntityId>,
    pub confidence: Option<String>,
    pub limitations: Vec<String>,
    pub recommendations: Vec<String>,
    pub raw_result_ref: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub kind: Option<AnalysisKind>,
    pub title: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub input_entity_ids: Option<Vec<EntityId>>,
    pub output_summary: Option<TextField>,
    pub findings: Option<Vec<String>>,
    pub metrics: Option<Vec<String>>,
    pub charts: Option<Vec<String>>,
    pub run_by: Option<EntityId>,
    pub run_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub tool_version: Option<String>,
    pub scenario_id: Option<EntityId>,
    pub report_id: Option<EntityId>,
    pub confidence: Option<String>,
    pub limitations: Option<Vec<String>>,
    pub recommendations: Option<Vec<String>>,
    pub raw_result_ref: Option<String>,
}

impl_identified_header!(AnalysisRecord);

impl_patchable!(
    AnalysisRecord,
    AnalysisRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [kind] => kind,
        [title] => title,
        [parameters] => parameters,
        [input_entity_ids] => input_entity_ids,
        [output_summary] => output_summary,
        [findings] => findings,
        [metrics] => metrics,
        [charts] => charts,
        [run_by] => run_by,
        [run_at] => run_at,
        [duration_ms] => duration_ms,
        [tool_version] => tool_version,
        [scenario_id] => scenario_id,
        [report_id] => report_id,
        [confidence] => confidence,
        [limitations] => limitations,
        [recommendations] => recommendations,
        [raw_result_ref] => raw_result_ref,
    }
);
// #endregion

// #region 🔖️ReportRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ReportRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub kind: ReportKind,
    pub title: String,
    pub audience: Vec<String>,
    pub sections: Vec<String>,
    pub generated_at: Option<String>,
    pub generated_by: Option<EntityId>,
    pub analysis_ids: Vec<EntityId>,
    pub format: Option<String>,
    pub file_ref: Option<String>,
    pub distribution_list: Vec<EntityId>,
    pub approval_status: ValidationStatus,
    pub approver_id: Option<EntityId>,
    pub version: String,
    pub template_id: Option<EntityId>,
    pub parameters: Vec<String>,
    pub confidentiality: Option<String>,
    pub expiry_date: Option<String>,
    pub related_decision_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub kind: Option<ReportKind>,
    pub title: Option<String>,
    pub audience: Option<Vec<String>>,
    pub sections: Option<Vec<String>>,
    pub generated_at: Option<String>,
    pub generated_by: Option<EntityId>,
    pub analysis_ids: Option<Vec<EntityId>>,
    pub format: Option<String>,
    pub file_ref: Option<String>,
    pub distribution_list: Option<Vec<EntityId>>,
    pub approval_status: Option<ValidationStatus>,
    pub approver_id: Option<EntityId>,
    pub version: Option<String>,
    pub template_id: Option<EntityId>,
    pub parameters: Option<Vec<String>>,
    pub confidentiality: Option<String>,
    pub expiry_date: Option<String>,
    pub related_decision_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(ReportRecord);

impl_patchable!(
    ReportRecord,
    ReportRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [kind] => kind,
        [title] => title,
        [audience] => audience,
        [sections] => sections,
        [generated_at] => generated_at,
        [generated_by] => generated_by,
        [analysis_ids] => analysis_ids,
        [format] => format,
        [file_ref] => file_ref,
        [distribution_list] => distribution_list,
        [approval_status] => approval_status,
        [approver_id] => approver_id,
        [version] => version,
        [template_id] => template_id,
        [parameters] => parameters,
        [confidentiality] => confidentiality,
        [expiry_date] => expiry_date,
        [related_decision_ids] => related_decision_ids,
    }
);
// #endregion

// #region 🔖️SearchFilter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilter {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub filter_name: String,
    pub filter_description: Option<TextField>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub owner_ids: Vec<EntityId>,
    pub statuses: Vec<LifecycleStatus>,
    pub priorities: Vec<Priority>,
    pub sources: Vec<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub entity_kinds: Vec<String>,
    pub tag_filters: Vec<String>,
    pub sort_field: Option<String>,
    pub sort_direction: Option<String>,
    pub is_public: bool,
    pub created_by: Option<EntityId>,
    pub last_used: Option<String>,
    pub use_count: u64,
    pub pinned: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilterPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub filter_name: Option<String>,
    pub filter_description: Option<TextField>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub owner_ids: Option<Vec<EntityId>>,
    pub statuses: Option<Vec<LifecycleStatus>>,
    pub priorities: Option<Vec<Priority>>,
    pub sources: Option<Vec<String>>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub entity_kinds: Option<Vec<String>>,
    pub tag_filters: Option<Vec<String>>,
    pub sort_field: Option<String>,
    pub sort_direction: Option<String>,
    pub is_public: Option<bool>,
    pub created_by: Option<EntityId>,
    pub last_used: Option<String>,
    pub use_count: Option<u64>,
    pub pinned: Option<bool>,
}

impl_identified_header!(SearchFilter);

impl_patchable!(
    SearchFilter,
    SearchFilterPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [filter_name] => filter_name,
        [filter_description] => filter_description,
        [keywords] => keywords,
        [categories] => categories,
        [owner_ids] => owner_ids,
        [statuses] => statuses,
        [priorities] => priorities,
        [sources] => sources,
        [date_from] => date_from,
        [date_to] => date_to,
        [entity_kinds] => entity_kinds,
        [tag_filters] => tag_filters,
        [sort_field] => sort_field,
        [sort_direction] => sort_direction,
        [is_public] => is_public,
        [created_by] => created_by,
        [last_used] => last_used,
        [use_count] => use_count,
        [pinned] => pinned,
    }
);
// #endregion

// #region 🔖️StatusRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct StatusRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub record_status: LifecycleStatus,
    pub previous_status: Option<LifecycleStatus>,
    pub changed_by: Option<EntityId>,
    pub changed_at: Option<String>,
    pub reason: Option<TextField>,
    pub blockers: Vec<String>,
    pub next_actions: Vec<String>,
    pub due_date: Option<String>,
    #[dsl(unit = "pct")]
    pub progress_percent: Option<f64>,
    pub health: Option<String>,
    pub escalation_level: Option<String>,
    pub related_issue_ids: Vec<EntityId>,
    pub related_risk_ids: Vec<EntityId>,
    pub milestone_id: Option<EntityId>,
    pub reporting_period: Option<String>,
    pub status_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub record_status: Option<LifecycleStatus>,
    pub previous_status: Option<LifecycleStatus>,
    pub changed_by: Option<EntityId>,
    pub changed_at: Option<String>,
    pub reason: Option<TextField>,
    pub blockers: Option<Vec<String>>,
    pub next_actions: Option<Vec<String>>,
    pub due_date: Option<String>,
    pub progress_percent: Option<f64>,
    pub health: Option<String>,
    pub escalation_level: Option<String>,
    pub related_issue_ids: Option<Vec<EntityId>>,
    pub related_risk_ids: Option<Vec<EntityId>>,
    pub milestone_id: Option<EntityId>,
    pub reporting_period: Option<String>,
    pub status_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(StatusRecord);

impl_patchable!(
    StatusRecord,
    StatusRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [record_status] => record_status,
        [previous_status] => previous_status,
        [changed_by] => changed_by,
        [changed_at] => changed_at,
        [reason] => reason,
        [blockers] => blockers,
        [next_actions] => next_actions,
        [due_date] => due_date,
        [progress_percent] => progress_percent,
        [health] => health,
        [escalation_level] => escalation_level,
        [related_issue_ids] => related_issue_ids,
        [related_risk_ids] => related_risk_ids,
        [milestone_id] => milestone_id,
        [reporting_period] => reporting_period,
        [status_notes] => status_notes,
    }
);
// #endregion

// #region 🔖️Workshop
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Workshop {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub workshop_type: String,
    pub objectives: Vec<String>,
    pub agenda: Vec<String>,
    pub facilitator_id: Option<EntityId>,
    pub participants: Vec<EntityId>,
    pub scheduled_start: Option<String>,
    pub scheduled_end: Option<String>,
    pub location: Option<String>,
    pub materials: Vec<String>,
    pub methods: Vec<String>,
    pub outputs: Vec<String>,
    pub decisions: Vec<EntityId>,
    pub issues: Vec<EntityId>,
    pub follow_up_actions: Vec<String>,
    pub feedback: Vec<TaggedNote>,
    pub recording_ref: Option<String>,
    pub budget: Option<f64>,
    pub workshop_status: LifecycleStatus,
    pub survey_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub workshop_type: Option<String>,
    pub objectives: Option<Vec<String>>,
    pub agenda: Option<Vec<String>>,
    pub facilitator_id: Option<EntityId>,
    pub participants: Option<Vec<EntityId>>,
    pub scheduled_start: Option<String>,
    pub scheduled_end: Option<String>,
    pub location: Option<String>,
    pub materials: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub decisions: Option<Vec<EntityId>>,
    pub issues: Option<Vec<EntityId>>,
    pub follow_up_actions: Option<Vec<String>>,
    pub feedback: Option<Vec<TaggedNote>>,
    pub recording_ref: Option<String>,
    pub budget: Option<f64>,
    pub workshop_status: Option<LifecycleStatus>,
    pub survey_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(Workshop);

impl_patchable!(
    Workshop,
    WorkshopPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [workshop_type] => workshop_type,
        [objectives] => objectives,
        [agenda] => agenda,
        [facilitator_id] => facilitator_id,
        [participants] => participants,
        [scheduled_start] => scheduled_start,
        [scheduled_end] => scheduled_end,
        [location] => location,
        [materials] => materials,
        [methods] => methods,
        [outputs] => outputs,
        [decisions] => decisions,
        [issues] => issues,
        [follow_up_actions] => follow_up_actions,
        [feedback] => feedback,
        [recording_ref] => recording_ref,
        [budget] => budget,
        [workshop_status] => workshop_status,
        [survey_ids] => survey_ids,
    }
);
// #endregion

// #region 🔖️Survey
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub survey_type: String,
    pub title: String,
    pub objectives: Vec<String>,
    pub questions: Vec<String>,
    pub target_audience: Vec<EntityId>,
    pub distribution_channels: Vec<String>,
    pub launch_date: Option<String>,
    pub close_date: Option<String>,
    pub response_count: u32,
    pub response_rate: Option<f64>,
    pub findings: Vec<String>,
    pub themes: Vec<String>,
    pub recommendations: Vec<String>,
    pub confidentiality: Option<String>,
    pub consent_process: Vec<String>,
    pub analysis_id: Option<EntityId>,
    pub workshop_id: Option<EntityId>,
    pub owner_id: Option<EntityId>,
    pub survey_status: LifecycleStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurveyPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub survey_type: Option<String>,
    pub title: Option<String>,
    pub objectives: Option<Vec<String>>,
    pub questions: Option<Vec<String>>,
    pub target_audience: Option<Vec<EntityId>>,
    pub distribution_channels: Option<Vec<String>>,
    pub launch_date: Option<String>,
    pub close_date: Option<String>,
    pub response_count: Option<u32>,
    pub response_rate: Option<f64>,
    pub findings: Option<Vec<String>>,
    pub themes: Option<Vec<String>>,
    pub recommendations: Option<Vec<String>>,
    pub confidentiality: Option<String>,
    pub consent_process: Option<Vec<String>>,
    pub analysis_id: Option<EntityId>,
    pub workshop_id: Option<EntityId>,
    pub owner_id: Option<EntityId>,
    pub survey_status: Option<LifecycleStatus>,
}

impl_identified_header!(Survey);

impl_patchable!(
    Survey,
    SurveyPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [survey_type] => survey_type,
        [title] => title,
        [objectives] => objectives,
        [questions] => questions,
        [target_audience] => target_audience,
        [distribution_channels] => distribution_channels,
        [launch_date] => launch_date,
        [close_date] => close_date,
        [response_count] => response_count,
        [response_rate] => response_rate,
        [findings] => findings,
        [themes] => themes,
        [recommendations] => recommendations,
        [confidentiality] => confidentiality,
        [consent_process] => consent_process,
        [analysis_id] => analysis_id,
        [workshop_id] => workshop_id,
        [owner_id] => owner_id,
        [survey_status] => survey_status,
    }
);
// #endregion

// #region 🔖️Issue
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub issue_type: String,
    pub summary: TextField,
    pub issue_description: TextField,
    pub severity: IssueSeverity,
    pub issue_priority: Priority,
    pub reporter_id: Option<EntityId>,
    pub assignee_id: Option<EntityId>,
    pub affected_entity_ids: Vec<EntityId>,
    pub root_cause: Option<TextField>,
    pub resolution: Option<TextField>,
    pub workaround: Option<TextField>,
    pub due_date: Option<String>,
    pub resolved_date: Option<String>,
    pub related_conflict_ids: Vec<EntityId>,
    pub related_risk_ids: Vec<EntityId>,
    pub decision_id: Option<EntityId>,
    pub comments: Vec<TaggedNote>,
    pub attachments: Vec<EntityId>,
    pub escalation_level: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub issue_type: Option<String>,
    pub summary: Option<TextField>,
    pub issue_description: Option<TextField>,
    pub severity: Option<IssueSeverity>,
    pub issue_priority: Option<Priority>,
    pub reporter_id: Option<EntityId>,
    pub assignee_id: Option<EntityId>,
    pub affected_entity_ids: Option<Vec<EntityId>>,
    pub root_cause: Option<TextField>,
    pub resolution: Option<TextField>,
    pub workaround: Option<TextField>,
    pub due_date: Option<String>,
    pub resolved_date: Option<String>,
    pub related_conflict_ids: Option<Vec<EntityId>>,
    pub related_risk_ids: Option<Vec<EntityId>>,
    pub decision_id: Option<EntityId>,
    pub comments: Option<Vec<TaggedNote>>,
    pub attachments: Option<Vec<EntityId>>,
    pub escalation_level: Option<String>,
}

impl_identified_header!(Issue);

impl_patchable!(
    Issue,
    IssuePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [issue_type] => issue_type,
        [summary] => summary,
        [issue_description] => issue_description,
        [severity] => severity,
        [issue_priority] => issue_priority,
        [reporter_id] => reporter_id,
        [assignee_id] => assignee_id,
        [affected_entity_ids] => affected_entity_ids,
        [root_cause] => root_cause,
        [resolution] => resolution,
        [workaround] => workaround,
        [due_date] => due_date,
        [resolved_date] => resolved_date,
        [related_conflict_ids] => related_conflict_ids,
        [related_risk_ids] => related_risk_ids,
        [decision_id] => decision_id,
        [comments] => comments,
        [attachments] => attachments,
        [escalation_level] => escalation_level,
    }
);
// #endregion

// #region 🔖️AuditEvent
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub action: AuditAction,
    pub actor_id: Option<EntityId>,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub timestamp: String,
    pub details: TextField,
    pub before_state: Option<String>,
    pub after_state: Option<String>,
    pub ip_address: Option<String>,
    pub client: Option<String>,
    pub session_id: Option<String>,
    pub change_record_id: Option<EntityId>,
    pub trace_link: Option<TraceLink>,
    pub success: bool,
    pub error_message: Option<String>,
    pub correlation_id: Option<String>,
    pub compliance_tags: Vec<String>,
    pub retention_until: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub action: Option<AuditAction>,
    pub actor_id: Option<EntityId>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub timestamp: Option<String>,
    pub details: Option<TextField>,
    pub before_state: Option<String>,
    pub after_state: Option<String>,
    pub ip_address: Option<String>,
    pub client: Option<String>,
    pub session_id: Option<String>,
    pub change_record_id: Option<EntityId>,
    pub trace_link: Option<TraceLink>,
    pub success: Option<bool>,
    pub error_message: Option<String>,
    pub correlation_id: Option<String>,
    pub compliance_tags: Option<Vec<String>>,
    pub retention_until: Option<String>,
}

impl_identified_header!(AuditEvent);

impl_patchable!(
    AuditEvent,
    AuditEventPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [action] => action,
        [actor_id] => actor_id,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [timestamp] => timestamp,
        [details] => details,
        [before_state] => before_state,
        [after_state] => after_state,
        [ip_address] => ip_address,
        [client] => client,
        [session_id] => session_id,
        [change_record_id] => change_record_id,
        [trace_link] => trace_link,
        [success] => success,
        [error_message] => error_message,
        [correlation_id] => correlation_id,
        [compliance_tags] => compliance_tags,
        [retention_until] => retention_until,
    }
);
// #endregion

// #region 🔖️TemplateRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct TemplateRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub template_type: String,
    pub sector: Option<String>,
    pub project_type: Option<String>,
    pub version: String,
    pub content_ref: Option<String>,
    pub entity_kinds: Vec<String>,
    pub default_fields: Vec<String>,
    pub checklists: Vec<String>,
    pub standards: Vec<String>,
    pub applicability: Vec<String>,
    pub author_id: Option<EntityId>,
    pub approval_status: ValidationStatus,
    pub usage_count: u64,
    pub last_applied: Option<String>,
    pub customization_notes: Vec<String>,
    pub related_knowledge_ids: Vec<EntityId>,
    pub benchmark_ids: Vec<EntityId>,
    pub license: Option<String>,
    pub source_organization: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub template_type: Option<String>,
    pub sector: Option<String>,
    pub project_type: Option<String>,
    pub version: Option<String>,
    pub content_ref: Option<String>,
    pub entity_kinds: Option<Vec<String>>,
    pub default_fields: Option<Vec<String>>,
    pub checklists: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub applicability: Option<Vec<String>>,
    pub author_id: Option<EntityId>,
    pub approval_status: Option<ValidationStatus>,
    pub usage_count: Option<u64>,
    pub last_applied: Option<String>,
    pub customization_notes: Option<Vec<String>>,
    pub related_knowledge_ids: Option<Vec<EntityId>>,
    pub benchmark_ids: Option<Vec<EntityId>>,
    pub license: Option<String>,
    pub source_organization: Option<String>,
}

impl_identified_header!(TemplateRecord);

impl_patchable!(
    TemplateRecord,
    TemplateRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [template_type] => template_type,
        [sector] => sector,
        [project_type] => project_type,
        [version] => version,
        [content_ref] => content_ref,
        [entity_kinds] => entity_kinds,
        [default_fields] => default_fields,
        [checklists] => checklists,
        [standards] => standards,
        [applicability] => applicability,
        [author_id] => author_id,
        [approval_status] => approval_status,
        [usage_count] => usage_count,
        [last_applied] => last_applied,
        [customization_notes] => customization_notes,
        [related_knowledge_ids] => related_knowledge_ids,
        [benchmark_ids] => benchmark_ids,
        [license] => license,
        [source_organization] => source_organization,
    }
);
// #endregion

// #region 🔖️KnowledgeRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub topic: String,
    pub category: String,
    pub summary: TextField,
    pub content: TextField,
    pub sources: Vec<String>,
    pub references: Vec<String>,
    pub lessons_learned: Vec<String>,
    pub best_practices: Vec<String>,
    pub applicable_sectors: Vec<String>,
    pub related_entity_kinds: Vec<String>,
    pub author_ids: Vec<EntityId>,
    pub expertise_level: Option<String>,
    pub validation_status: ValidationStatus,
    pub last_reviewed: Option<String>,
    pub keywords: Vec<String>,
    pub attachments: Vec<EntityId>,
    pub citations: Vec<String>,
    pub usage_count: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub topic: Option<String>,
    pub category: Option<String>,
    pub summary: Option<TextField>,
    pub content: Option<TextField>,
    pub sources: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub lessons_learned: Option<Vec<String>>,
    pub best_practices: Option<Vec<String>>,
    pub applicable_sectors: Option<Vec<String>>,
    pub related_entity_kinds: Option<Vec<String>>,
    pub author_ids: Option<Vec<EntityId>>,
    pub expertise_level: Option<String>,
    pub validation_status: Option<ValidationStatus>,
    pub last_reviewed: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub attachments: Option<Vec<EntityId>>,
    pub citations: Option<Vec<String>>,
    pub usage_count: Option<u64>,
}

impl_identified_header!(KnowledgeRecord);

impl_patchable!(
    KnowledgeRecord,
    KnowledgeRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [topic] => topic,
        [category] => category,
        [summary] => summary,
        [content] => content,
        [sources] => sources,
        [references] => references,
        [lessons_learned] => lessons_learned,
        [best_practices] => best_practices,
        [applicable_sectors] => applicable_sectors,
        [related_entity_kinds] => related_entity_kinds,
        [author_ids] => author_ids,
        [expertise_level] => expertise_level,
        [validation_status] => validation_status,
        [last_reviewed] => last_reviewed,
        [keywords] => keywords,
        [attachments] => attachments,
        [citations] => citations,
        [usage_count] => usage_count,
    }
);
// #endregion

// #region 🔖️BenchmarkRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub benchmark_name: String,
    pub sector: String,
    pub metric: String,
    pub value: f64,
    pub unit: String,
    pub sample_size: Option<u32>,
    pub source: Option<String>,
    pub collection_year: Option<u32>,
    pub geography: Option<String>,
    pub building_type: Option<String>,
    pub confidence: Option<String>,
    pub methodology: Option<String>,
    pub applicable_element_kinds: Vec<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub comparison_notes: Vec<String>,
    pub limitations: Vec<String>,
    pub license: Option<String>,
    pub knowledge_id: Option<EntityId>,
    pub last_verified: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub benchmark_name: Option<String>,
    pub sector: Option<String>,
    pub metric: Option<String>,
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub sample_size: Option<u32>,
    pub source: Option<String>,
    pub collection_year: Option<u32>,
    pub geography: Option<String>,
    pub building_type: Option<String>,
    pub confidence: Option<String>,
    pub methodology: Option<String>,
    pub applicable_element_kinds: Option<Vec<String>>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub comparison_notes: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
    pub license: Option<String>,
    pub knowledge_id: Option<EntityId>,
    pub last_verified: Option<String>,
}

impl_identified_header!(BenchmarkRecord);

impl_patchable!(
    BenchmarkRecord,
    BenchmarkRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [benchmark_name] => benchmark_name,
        [sector] => sector,
        [metric] => metric,
        [value] => value,
        [unit] => unit,
        [sample_size] => sample_size,
        [source] => source,
        [collection_year] => collection_year,
        [geography] => geography,
        [building_type] => building_type,
        [confidence] => confidence,
        [methodology] => methodology,
        [applicable_element_kinds] => applicable_element_kinds,
        [related_requirement_ids] => related_requirement_ids,
        [comparison_notes] => comparison_notes,
        [limitations] => limitations,
        [license] => license,
        [knowledge_id] => knowledge_id,
        [last_verified] => last_verified,
    }
);
// #endregion

// #region 🔖️Assumption
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Assumption {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub statement: TextField,
    pub basis: Option<TextField>,
    pub confidence_level: Option<String>,
    pub impact_if_false: Option<TextField>,
    pub related_entity_ids: Vec<EntityId>,
    pub validation_status: ValidationStatus,
    pub validated_by: Option<EntityId>,
    pub validation_date: Option<String>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub dependencies: Vec<String>,
    pub mitigation: Vec<String>,
    pub linked_requirement_ids: Vec<EntityId>,
    pub linked_risk_ids: Vec<EntityId>,
    pub expiration_date: Option<String>,
    pub status_notes: Vec<TaggedNote>,
    pub document_refs: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssumptionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub statement: Option<TextField>,
    pub basis: Option<TextField>,
    pub confidence_level: Option<String>,
    pub impact_if_false: Option<TextField>,
    pub related_entity_ids: Option<Vec<EntityId>>,
    pub validation_status: Option<ValidationStatus>,
    pub validated_by: Option<EntityId>,
    pub validation_date: Option<String>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub mitigation: Option<Vec<String>>,
    pub linked_requirement_ids: Option<Vec<EntityId>>,
    pub linked_risk_ids: Option<Vec<EntityId>>,
    pub expiration_date: Option<String>,
    pub status_notes: Option<Vec<TaggedNote>>,
    pub document_refs: Option<Vec<String>>,
}

impl_identified_header!(Assumption);

impl_patchable!(
    Assumption,
    AssumptionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [statement] => statement,
        [basis] => basis,
        [confidence_level] => confidence_level,
        [impact_if_false] => impact_if_false,
        [related_entity_ids] => related_entity_ids,
        [validation_status] => validation_status,
        [validated_by] => validated_by,
        [validation_date] => validation_date,
        [owner_id] => owner_id,
        [review_cycle] => review_cycle,
        [source] => source,
        [category] => category,
        [dependencies] => dependencies,
        [mitigation] => mitigation,
        [linked_requirement_ids] => linked_requirement_ids,
        [linked_risk_ids] => linked_risk_ids,
        [expiration_date] => expiration_date,
        [status_notes] => status_notes,
        [document_refs] => document_refs,
    }
);
// #endregion

// #region 🔖️ConstraintRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub constraint_type: String,
    pub summary: TextField,
    pub severity: RiskLevel,
    pub affected_entity_ids: Vec<EntityId>,
    pub source: Option<String>,
    pub regulatory_basis: Vec<String>,
    pub mitigation_options: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub waiver_status: Option<String>,
    pub waiver_approver: Option<EntityId>,
    pub impact_assessment: Option<TextField>,
    pub resolution_plan: Vec<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub related_decision_ids: Vec<EntityId>,
    pub monitoring_frequency: Option<String>,
    pub compliance_status: ValidationStatus,
    pub exceptions: Vec<String>,
    pub trace_links: Vec<TraceLink>,
    pub escalation_contact_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub constraint_type: Option<String>,
    pub summary: Option<TextField>,
    pub severity: Option<RiskLevel>,
    pub affected_entity_ids: Option<Vec<EntityId>>,
    pub source: Option<String>,
    pub regulatory_basis: Option<Vec<String>>,
    pub mitigation_options: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub waiver_status: Option<String>,
    pub waiver_approver: Option<EntityId>,
    pub impact_assessment: Option<TextField>,
    pub resolution_plan: Option<Vec<String>>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub related_decision_ids: Option<Vec<EntityId>>,
    pub monitoring_frequency: Option<String>,
    pub compliance_status: Option<ValidationStatus>,
    pub exceptions: Option<Vec<String>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub escalation_contact_id: Option<EntityId>,
}

impl_identified_header!(ConstraintRecord);

impl_patchable!(
    ConstraintRecord,
    ConstraintRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [constraint_type] => constraint_type,
        [summary] => summary,
        [severity] => severity,
        [affected_entity_ids] => affected_entity_ids,
        [source] => source,
        [regulatory_basis] => regulatory_basis,
        [mitigation_options] => mitigation_options,
        [owner_id] => owner_id,
        [effective_date] => effective_date,
        [expiry_date] => expiry_date,
        [waiver_status] => waiver_status,
        [waiver_approver] => waiver_approver,
        [impact_assessment] => impact_assessment,
        [resolution_plan] => resolution_plan,
        [related_requirement_ids] => related_requirement_ids,
        [related_decision_ids] => related_decision_ids,
        [monitoring_frequency] => monitoring_frequency,
        [compliance_status] => compliance_status,
        [exceptions] => exceptions,
        [trace_links] => trace_links,
        [escalation_contact_id] => escalation_contact_id,
    }
);
// #endregion

// #region 🔖️ComplianceRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ComplianceRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub standard_ref: String,
    pub obligation: TextField,
    pub compliance_status: ValidationStatus,
    pub evidence_refs: Vec<String>,
    pub auditor_id: Option<EntityId>,
    pub audit_date: Option<String>,
    pub next_review: Option<String>,
    pub affected_entity_ids: Vec<EntityId>,
    pub gap_analysis: Vec<String>,
    pub remediation_plan: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub severity: RiskLevel,
    pub regulatory_body: Option<String>,
    pub certification_target: Option<String>,
    pub waiver_status: Option<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub monitoring_method: Option<String>,
    pub reporting_frequency: Option<String>,
    pub penalties: Vec<String>,
    pub corrective_actions: Vec<String>,
    pub document_refs: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplianceRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub standard_ref: Option<String>,
    pub obligation: Option<TextField>,
    pub compliance_status: Option<ValidationStatus>,
    pub evidence_refs: Option<Vec<String>>,
    pub auditor_id: Option<EntityId>,
    pub audit_date: Option<String>,
    pub next_review: Option<String>,
    pub affected_entity_ids: Option<Vec<EntityId>>,
    pub gap_analysis: Option<Vec<String>>,
    pub remediation_plan: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub severity: Option<RiskLevel>,
    pub regulatory_body: Option<String>,
    pub certification_target: Option<String>,
    pub waiver_status: Option<String>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub monitoring_method: Option<String>,
    pub reporting_frequency: Option<String>,
    pub penalties: Option<Vec<String>>,
    pub corrective_actions: Option<Vec<String>>,
    pub document_refs: Option<Vec<String>>,
}

impl_identified_header!(ComplianceRecord);

impl_patchable!(
    ComplianceRecord,
    ComplianceRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [standard_ref] => standard_ref,
        [obligation] => obligation,
        [compliance_status] => compliance_status,
        [evidence_refs] => evidence_refs,
        [auditor_id] => auditor_id,
        [audit_date] => audit_date,
        [next_review] => next_review,
        [affected_entity_ids] => affected_entity_ids,
        [gap_analysis] => gap_analysis,
        [remediation_plan] => remediation_plan,
        [owner_id] => owner_id,
        [severity] => severity,
        [regulatory_body] => regulatory_body,
        [certification_target] => certification_target,
        [waiver_status] => waiver_status,
        [related_requirement_ids] => related_requirement_ids,
        [monitoring_method] => monitoring_method,
        [reporting_frequency] => reporting_frequency,
        [penalties] => penalties,
        [corrective_actions] => corrective_actions,
        [document_refs] => document_refs,
    }
);
// #endregion

// #region 🔖️ApprovalRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub approval_type: String,
    pub subject_id: EntityId,
    pub approver_ids: Vec<EntityId>,
    pub approval_date: Option<String>,
    pub conditions: Vec<String>,
    pub approval_status: LifecycleStatus,
    pub expiry_date: Option<String>,
    pub delegation_chain: Vec<EntityId>,
    pub evidence_refs: Vec<String>,
    pub related_decision_id: Option<EntityId>,
    pub related_change_id: Option<EntityId>,
    pub authority_basis: Vec<String>,
    pub signature_method: Option<String>,
    pub rejection_reason: Option<TextField>,
    pub resubmission_date: Option<String>,
    pub notification_list: Vec<EntityId>,
    pub workflow_step: Option<String>,
    pub version: Option<String>,
    pub audit_trail_ref: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub approval_type: Option<String>,
    pub subject_id: Option<EntityId>,
    pub approver_ids: Option<Vec<EntityId>>,
    pub approval_date: Option<String>,
    pub conditions: Option<Vec<String>>,
    pub approval_status: Option<LifecycleStatus>,
    pub expiry_date: Option<String>,
    pub delegation_chain: Option<Vec<EntityId>>,
    pub evidence_refs: Option<Vec<String>>,
    pub related_decision_id: Option<EntityId>,
    pub related_change_id: Option<EntityId>,
    pub authority_basis: Option<Vec<String>>,
    pub signature_method: Option<String>,
    pub rejection_reason: Option<TextField>,
    pub resubmission_date: Option<String>,
    pub notification_list: Option<Vec<EntityId>>,
    pub workflow_step: Option<String>,
    pub version: Option<String>,
    pub audit_trail_ref: Option<String>,
}

impl_identified_header!(ApprovalRecord);

impl_patchable!(
    ApprovalRecord,
    ApprovalRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [approval_type] => approval_type,
        [subject_id] => subject_id,
        [approver_ids] => approver_ids,
        [approval_date] => approval_date,
        [conditions] => conditions,
        [approval_status] => approval_status,
        [expiry_date] => expiry_date,
        [delegation_chain] => delegation_chain,
        [evidence_refs] => evidence_refs,
        [related_decision_id] => related_decision_id,
        [related_change_id] => related_change_id,
        [authority_basis] => authority_basis,
        [signature_method] => signature_method,
        [rejection_reason] => rejection_reason,
        [resubmission_date] => resubmission_date,
        [notification_list] => notification_list,
        [workflow_step] => workflow_step,
        [version] => version,
        [audit_trail_ref] => audit_trail_ref,
    }
);
// #endregion

// #region 🔖️MeetingRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MeetingRecord {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub meeting_type: String,
    pub scheduled_date: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    pub chair_id: Option<EntityId>,
    pub attendee_ids: Vec<EntityId>,
    pub agenda_items: Vec<String>,
    pub minutes: Option<TextField>,
    pub action_items: Vec<String>,
    pub decisions_made: Vec<EntityId>,
    pub document_refs: Vec<String>,
    pub follow_up_date: Option<String>,
    pub recording_ref: Option<String>,
    pub quorum_met: bool,
    pub meeting_status: LifecycleStatus,
    pub workshop_id: Option<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub issue_ids: Vec<EntityId>,
    pub approval_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub meeting_type: Option<String>,
    pub scheduled_date: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    pub chair_id: Option<EntityId>,
    pub attendee_ids: Option<Vec<EntityId>>,
    pub agenda_items: Option<Vec<String>>,
    pub minutes: Option<TextField>,
    pub action_items: Option<Vec<String>>,
    pub decisions_made: Option<Vec<EntityId>>,
    pub document_refs: Option<Vec<String>>,
    pub follow_up_date: Option<String>,
    pub recording_ref: Option<String>,
    pub quorum_met: Option<bool>,
    pub meeting_status: Option<LifecycleStatus>,
    pub workshop_id: Option<EntityId>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub issue_ids: Option<Vec<EntityId>>,
    pub approval_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(MeetingRecord);

impl_patchable!(
    MeetingRecord,
    MeetingRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [meeting_type] => meeting_type,
        [scheduled_date] => scheduled_date,
        [duration] => duration,
        [location] => location,
        [chair_id] => chair_id,
        [attendee_ids] => attendee_ids,
        [agenda_items] => agenda_items,
        [minutes] => minutes,
        [action_items] => action_items,
        [decisions_made] => decisions_made,
        [document_refs] => document_refs,
        [follow_up_date] => follow_up_date,
        [recording_ref] => recording_ref,
        [quorum_met] => quorum_met,
        [meeting_status] => meeting_status,
        [workshop_id] => workshop_id,
        [stakeholder_ids] => stakeholder_ids,
        [requirement_ids] => requirement_ids,
        [issue_ids] => issue_ids,
        [approval_ids] => approval_ids,
    }
);
// #endregion

// #region 🔖️Governance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Governance {
    pub id: EntityId,
    pub framework: String,
    pub roles: Vec<String>,
    pub responsibilities: Vec<String>,
    pub approval_matrix: Vec<String>,
    pub escalation_paths: Vec<String>,
    pub meeting_cadence: Vec<String>,
    pub decision_rights: Vec<String>,
    pub change_control_process: Vec<String>,
    pub quality_policy: TextField,
    pub risk_appetite: Option<String>,
    pub compliance_obligations: Vec<String>,
    pub audit_schedule: Option<String>,
    pub document_control: Vec<String>,
    pub stakeholder_engagement_plan: Vec<String>,
    pub ethics_policy: Vec<String>,
    pub data_governance: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub review_hierarchy: Vec<String>,
    pub policy_ownership_id: Option<EntityId>,
    pub requirement_ownership_id: Option<EntityId>,
    pub risk_ownership_id: Option<EntityId>,
    pub reporting_frequency: Option<String>,
    pub accountability_rules: Vec<String>,
    pub exception_management: Vec<String>,
    pub governance_performance: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernancePatch {
    pub id: Option<EntityId>,
    pub framework: Option<String>,
    pub roles: Option<Vec<String>>,
    pub responsibilities: Option<Vec<String>>,
    pub approval_matrix: Option<Vec<String>>,
    pub escalation_paths: Option<Vec<String>>,
    pub meeting_cadence: Option<Vec<String>>,
    pub decision_rights: Option<Vec<String>>,
    pub change_control_process: Option<Vec<String>>,
    pub quality_policy: Option<TextField>,
    pub risk_appetite: Option<String>,
    pub compliance_obligations: Option<Vec<String>>,
    pub audit_schedule: Option<String>,
    pub document_control: Option<Vec<String>>,
    pub stakeholder_engagement_plan: Option<Vec<String>>,
    pub ethics_policy: Option<Vec<String>>,
    pub data_governance: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub review_hierarchy: Option<Vec<String>>,
    pub policy_ownership_id: Option<EntityId>,
    pub requirement_ownership_id: Option<EntityId>,
    pub risk_ownership_id: Option<EntityId>,
    pub reporting_frequency: Option<String>,
    pub accountability_rules: Option<Vec<String>>,
    pub exception_management: Option<Vec<String>>,
    pub governance_performance: Option<Vec<String>>,
}

impl Identified<EntityId> for Governance {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl_patchable!(
    Governance,
    GovernancePatch,
    {
        [id] => id,
        [framework] => framework,
        [roles] => roles,
        [responsibilities] => responsibilities,
        [approval_matrix] => approval_matrix,
        [escalation_paths] => escalation_paths,
        [meeting_cadence] => meeting_cadence,
        [decision_rights] => decision_rights,
        [change_control_process] => change_control_process,
        [quality_policy] => quality_policy,
        [risk_appetite] => risk_appetite,
        [compliance_obligations] => compliance_obligations,
        [audit_schedule] => audit_schedule,
        [document_control] => document_control,
        [stakeholder_engagement_plan] => stakeholder_engagement_plan,
        [ethics_policy] => ethics_policy,
        [data_governance] => data_governance,
        [owner_id] => owner_id,
        [review_cycle] => review_cycle,
        [review_hierarchy] => review_hierarchy,
        [policy_ownership_id] => policy_ownership_id,
        [requirement_ownership_id] => requirement_ownership_id,
        [risk_ownership_id] => risk_ownership_id,
        [reporting_frequency] => reporting_frequency,
        [accountability_rules] => accountability_rules,
        [exception_management] => exception_management,
        [governance_performance] => governance_performance,
    }
);
// #endregion
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stakeholder_patch_round_trips() {
        let mut item = Stakeholder {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("stakeholder", "Base Stakeholder"), "Base Stakeholder") },
            role: String::new(),
            organization: String::new(),
            department: Some(String::new()),
            contact_email: Some(String::new()),
            contact_phone: Some(String::new()),
            influence: InfluenceLevel::Low,
            interest: InfluenceLevel::Low,
            engagement: EngagementLevel::Unaware,
            expectations: Vec::new(),
            concerns: Vec::new(),
            requirement_ids: Vec::new(),
            decision_authority: false,
            communication_preferences: Vec::new(),
            reporting_frequency: Some(String::new()),
            involvement_phases: Vec::new(),
            availability: Some(String::new()),
            representative_of: Some(EntityId::new_serial("base0", "base0")),
            delegated_to: Some(EntityId::new_serial("base0", "base0")),
            relationship_to_client: Some(String::new()),
            power_interest_notes: Vec::new(),
            stakeholder_type: String::new(),
            influence_strategy: Some(String::new()),
            communication_channels: Vec::new(),
            success_metrics: Vec::new(),
        };
        let original = item.clone();
        let patch = StakeholderPatch {
            name: Some("Patched Stakeholder".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            role: Some("patched-0".to_string()),
            organization: Some("patched-0".to_string()),
            department: Some("patched-0".to_string()),
            contact_email: Some("patched-0".to_string()),
            contact_phone: Some("patched-0".to_string()),
            influence: Some(InfluenceLevel::Medium),
            interest: Some(InfluenceLevel::Medium),
            engagement: Some(EngagementLevel::Resistant),
            expectations: Some(vec!["patched-0".to_string()]),
            concerns: Some(vec!["patched-0".to_string()]),
            requirement_ids: Some(vec![EntityId::new_serial("new0", "new0")]),
            decision_authority: Some(true),
            communication_preferences: Some(vec!["patched-0".to_string()]),
            reporting_frequency: Some("patched-0".to_string()),
            involvement_phases: Some(vec!["patched-0".to_string()]),
            availability: Some("patched-0".to_string()),
            representative_of: Some(EntityId::new_serial("new0", "new0")),
            delegated_to: Some(EntityId::new_serial("new0", "new0")),
            relationship_to_client: Some("patched-0".to_string()),
            power_interest_notes: Some(vec![TaggedNote { tag: "new0".into(), text: "new-note0".into() }]),
            stakeholder_type: Some("patched-0".to_string()),
            influence_strategy: Some("patched-0".to_string()),
            communication_channels: Some(vec!["patched-0".to_string()]),
            success_metrics: Some(vec!["patched-0".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Stakeholder");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn user_profile_patch_round_trips() {
        let mut item = UserProfile {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("userprofile", "Base UserProfile"), "Base UserProfile") },
            category: UserCategory::Primary,
            demographic: Some(String::new()),
            age_range: Some(String::new()),
            abilities: Vec::new(),
            disabilities: Vec::new(),
            occupation: Some(String::new()),
            role_title: Some(String::new()),
            department: Some(String::new()),
            mobility_profile: Vec::new(),
            sensory_profile: Vec::new(),
            cognitive_profile: Vec::new(),
            behavioral_patterns: Vec::new(),
            usage_frequency: Some(String::new()),
            usage_duration: Some(String::new()),
            peak_usage_times: Vec::new(),
            technology_proficiency: Some(String::new()),
            preferences: Vec::new(),
            pain_points: Vec::new(),
            goals: Vec::new(),
            activity_ids: Vec::new(),
            research_method: Some(String::new()),
            persona_archetype: Some(String::new()),
            validated: false,
            stakeholder_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = UserProfilePatch {
            name: Some("Patched UserProfile".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            category: Some(UserCategory::Secondary),
            demographic: Some("patched-1".to_string()),
            age_range: Some("patched-1".to_string()),
            abilities: Some(vec!["patched-1".to_string()]),
            disabilities: Some(vec!["patched-1".to_string()]),
            occupation: Some("patched-1".to_string()),
            role_title: Some("patched-1".to_string()),
            department: Some("patched-1".to_string()),
            mobility_profile: Some(vec!["patched-1".to_string()]),
            sensory_profile: Some(vec!["patched-1".to_string()]),
            cognitive_profile: Some(vec!["patched-1".to_string()]),
            behavioral_patterns: Some(vec!["patched-1".to_string()]),
            usage_frequency: Some("patched-1".to_string()),
            usage_duration: Some("patched-1".to_string()),
            peak_usage_times: Some(vec!["patched-1".to_string()]),
            technology_proficiency: Some("patched-1".to_string()),
            preferences: Some(vec!["patched-1".to_string()]),
            pain_points: Some(vec!["patched-1".to_string()]),
            goals: Some(vec!["patched-1".to_string()]),
            activity_ids: Some(vec![EntityId::new_serial("new1", "new1")]),
            research_method: Some("patched-1".to_string()),
            persona_archetype: Some("patched-1".to_string()),
            validated: Some(true),
            stakeholder_ids: Some(vec![EntityId::new_serial("new1", "new1")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched UserProfile");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn activity_patch_round_trips() {
        let mut item = Activity {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("activity", "Base Activity"), "Base Activity") },
            code: String::new(),
            category: String::new(),
            frequency: Some(String::new()),
            duration: Some(String::new()),
            intensity: Some(String::new()),
            participants: QuantitySpec::default(),
            equipment_ids: Vec::new(),
            space_requirements: Vec::new(),
            environmental_needs: Vec::new(),
            privacy_needs: Vec::new(),
            accessibility_needs: Vec::new(),
            adjacent_activities: Vec::new(),
            sequencing: Vec::new(),
            peak_periods: Vec::new(),
            workflow_steps: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            user_profile_ids: Vec::new(),
            function_ids: Vec::new(),
            performance_indicators: Vec::new(),
            activity_type: String::new(),
            location_context: Some(String::new()),
            temporal_pattern: Some(String::new()),
            supervision_level: Some(String::new()),
        };
        let original = item.clone();
        let patch = ActivityPatch {
            name: Some("Patched Activity".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-2".to_string()),
            category: Some("patched-2".to_string()),
            frequency: Some("patched-2".to_string()),
            duration: Some("patched-2".to_string()),
            intensity: Some("patched-2".to_string()),
            participants: Some(QuantitySpec::target_unit(42.0, "m2")),
            equipment_ids: Some(vec![EntityId::new_serial("new2", "new2")]),
            space_requirements: Some(vec!["patched-2".to_string()]),
            environmental_needs: Some(vec!["patched-2".to_string()]),
            privacy_needs: Some(vec!["patched-2".to_string()]),
            accessibility_needs: Some(vec!["patched-2".to_string()]),
            adjacent_activities: Some(vec![EntityId::new_serial("new2", "new2")]),
            sequencing: Some(vec!["patched-2".to_string()]),
            peak_periods: Some(vec!["patched-2".to_string()]),
            workflow_steps: Some(vec!["patched-2".to_string()]),
            inputs: Some(vec!["patched-2".to_string()]),
            outputs: Some(vec!["patched-2".to_string()]),
            user_profile_ids: Some(vec![EntityId::new_serial("new2", "new2")]),
            function_ids: Some(vec![EntityId::new_serial("new2", "new2")]),
            performance_indicators: Some(vec!["patched-2".to_string()]),
            activity_type: Some("patched-2".to_string()),
            location_context: Some("patched-2".to_string()),
            temporal_pattern: Some("patched-2".to_string()),
            supervision_level: Some("patched-2".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Activity");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn function_patch_round_trips() {
        let mut item = Function {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("function", "Base Function"), "Base Function") },
            code: String::new(),
            kind: FunctionKind::Primary,
            purpose: TextField::default(),
            criticality: Priority::Mandatory,
            performance_targets: Vec::new(),
            service_level: Some(String::new()),
            operating_hours: Some(String::new()),
            staffing: QuantitySpec::default(),
            equipment_ids: Vec::new(),
            resource_ids: Vec::new(),
            activity_ids: Vec::new(),
            element_ids: Vec::new(),
            dependencies: Vec::new(),
            interfaces: Vec::new(),
            constraints: Vec::new(),
            quality_criteria: Vec::new(),
            regulatory_refs: Vec::new(),
            future_changes: Vec::new(),
            owner_stakeholder_id: Some(EntityId::new_serial("base3", "base3")),
            success_metrics: Vec::new(),
            hierarchy_parent_id: Some(EntityId::new_serial("base3", "base3")),
            conflict_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = FunctionPatch {
            name: Some("Patched Function".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-3".to_string()),
            kind: Some(FunctionKind::Secondary),
            purpose: Some(TextField::plain("patched-3")),
            criticality: Some(Priority::Essential),
            performance_targets: Some(vec!["patched-3".to_string()]),
            service_level: Some("patched-3".to_string()),
            operating_hours: Some("patched-3".to_string()),
            staffing: Some(QuantitySpec::target_unit(42.0, "m2")),
            equipment_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
            resource_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
            activity_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
            element_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
            dependencies: Some(vec![EntityId::new_serial("new3", "new3")]),
            interfaces: Some(vec!["patched-3".to_string()]),
            constraints: Some(vec!["patched-3".to_string()]),
            quality_criteria: Some(vec!["patched-3".to_string()]),
            regulatory_refs: Some(vec!["patched-3".to_string()]),
            future_changes: Some(vec!["patched-3".to_string()]),
            owner_stakeholder_id: Some(EntityId::new_serial("new3", "new3")),
            success_metrics: Some(vec!["patched-3".to_string()]),
            hierarchy_parent_id: Some(EntityId::new_serial("new3", "new3")),
            conflict_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Function");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn program_element_patch_round_trips() {
        let mut item = ProgramElement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("programelement", "Base ProgramElement"), "Base ProgramElement") },
            code: String::new(),
            kind: ProgramElementKind::Building,
            parent_id: Some(EntityId::new_serial("base4", "base4")),
            level: Some(String::new()),
            area: QuantitySpec::default(),
            volume: QuantitySpec::default(),
            height: QuantitySpec::default(),
            occupancy: QuantitySpec::default(),
            function_ids: Vec::new(),
            activity_ids: Vec::new(),
            user_profile_ids: Vec::new(),
            adjacency_ids: Vec::new(),
            quantity_ids: Vec::new(),
            requirement_ids: Vec::new(),
            location_hint: Some(String::new()),
            orientation: Some(String::new()),
            daylight_requirement: Some(String::new()),
            acoustic_class: Some(String::new()),
            security_zone: Some(String::new()),
            flexibility_notes: Vec::new(),
            growth_allocation: Some(String::new()),
            circulation_role: Some(String::new()),
            visibility_level: Some(String::new()),
            adjacency_preferences: Vec::new(),
            environmental_zone: Some(String::new()),
        };
        let original = item.clone();
        let patch = ProgramElementPatch {
            name: Some("Patched ProgramElement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-4".to_string()),
            kind: Some(ProgramElementKind::Campus),
            parent_id: Some(EntityId::new_serial("new4", "new4")),
            level: Some("patched-4".to_string()),
            area: Some(QuantitySpec::target_unit(42.0, "m2")),
            volume: Some(QuantitySpec::target_unit(42.0, "m2")),
            height: Some(QuantitySpec::target_unit(42.0, "m2")),
            occupancy: Some(QuantitySpec::target_unit(42.0, "m2")),
            function_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
            activity_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
            user_profile_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
            adjacency_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
            quantity_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
            requirement_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
            location_hint: Some("patched-4".to_string()),
            orientation: Some("patched-4".to_string()),
            daylight_requirement: Some("patched-4".to_string()),
            acoustic_class: Some("patched-4".to_string()),
            security_zone: Some("patched-4".to_string()),
            flexibility_notes: Some(vec!["patched-4".to_string()]),
            growth_allocation: Some("patched-4".to_string()),
            circulation_role: Some("patched-4".to_string()),
            visibility_level: Some("patched-4".to_string()),
            adjacency_preferences: Some(vec![EntityId::new_serial("new4", "new4")]),
            environmental_zone: Some("patched-4".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ProgramElement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn quantity_requirement_patch_round_trips() {
        let mut item = QuantityRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("quantityrequirement", "Base QuantityRequirement"), "Base QuantityRequirement") },
            target_element_id: EntityId::new_serial("base5", "base5"),
            metric: String::new(),
            quantity: QuantitySpec::default(),
            basis: Some(String::new()),
            calculation_method: Some(String::new()),
            source: Some(String::new()),
            benchmark_ref: Some(EntityId::new_serial("base5", "base5")),
            tolerance_percent: Some(0.0),
            peak_factor: Some(0.0),
            growth_factor: Some(0.0),
            unit_cost: Some(0.0),
            currency: Some(String::new()),
            verification_method: Some(String::new()),
            related_requirement_ids: Vec::new(),
            assumptions: Vec::new(),
            constraints: Vec::new(),
            schedule_phase: Some(String::new()),
            responsible_party: Some(EntityId::new_serial("base5", "base5")),
            last_verified: Some(String::new()),
            variance_notes: Vec::new(),
        };
        let original = item.clone();
        let patch = QuantityRequirementPatch {
            name: Some("Patched QuantityRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            target_element_id: Some(EntityId::new_serial("new5", "new5")),
            metric: Some("patched-5".to_string()),
            quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
            basis: Some("patched-5".to_string()),
            calculation_method: Some("patched-5".to_string()),
            source: Some("patched-5".to_string()),
            benchmark_ref: Some(EntityId::new_serial("new5", "new5")),
            tolerance_percent: Some(42.0),
            peak_factor: Some(42.0),
            growth_factor: Some(42.0),
            unit_cost: Some(42.0),
            currency: Some("patched-5".to_string()),
            verification_method: Some("patched-5".to_string()),
            related_requirement_ids: Some(vec![EntityId::new_serial("new5", "new5")]),
            assumptions: Some(vec!["patched-5".to_string()]),
            constraints: Some(vec!["patched-5".to_string()]),
            schedule_phase: Some("patched-5".to_string()),
            responsible_party: Some(EntityId::new_serial("new5", "new5")),
            last_verified: Some("patched-5".to_string()),
            variance_notes: Some(vec![TaggedNote { tag: "new5".into(), text: "new-note5".into() }]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched QuantityRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn relationship_patch_round_trips() {
        let mut item = Relationship {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("relationship", "Base Relationship"), "Base Relationship") },
            source_id: EntityId::new_serial("base6", "base6"),
            target_id: EntityId::new_serial("base6", "base6"),
            kind: RelationshipKind::Contains,
            strength: Some(0.0),
            directional: false,
            rationale: Some(TextField::default()),
            constraints: Vec::new(),
            conditions: Vec::new(),
            relationship_priority: Priority::Mandatory,
            valid_from: Some(String::new()),
            valid_until: Some(String::new()),
            evidence: Vec::new(),
            conflict_ids: Vec::new(),
            trace_links: Vec::new(),
            bidirectional: false,
            distance_constraint_m: Some(0.0),
            capacity_constraint: Some(String::new()),
            regulatory_basis: Vec::new(),
            review_cycle: Some(String::new()),
            owner_id: Some(EntityId::new_serial("base6", "base6")),
            proximity_requirement: Some(TextField::default()),
            compatibility_requirement: Some(TextField::default()),
            incompatibility_requirement: Some(TextField::default()),
            separation_requirements: Vec::new(),
        };
        let original = item.clone();
        let patch = RelationshipPatch {
            name: Some("Patched Relationship".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            source_id: Some(EntityId::new_serial("new6", "new6")),
            target_id: Some(EntityId::new_serial("new6", "new6")),
            kind: Some(RelationshipKind::Serves),
            strength: Some(42.0),
            directional: Some(true),
            rationale: Some(TextField::plain("patched-6")),
            constraints: Some(vec!["patched-6".to_string()]),
            conditions: Some(vec!["patched-6".to_string()]),
            relationship_priority: Some(Priority::Essential),
            valid_from: Some("patched-6".to_string()),
            valid_until: Some("patched-6".to_string()),
            evidence: Some(vec!["patched-6".to_string()]),
            conflict_ids: Some(vec![EntityId::new_serial("new6", "new6")]),
            trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom6n", "tfrom6n"), EntityId::new_serial("tto6n", "tto6n"), TraceKind::FullAuditTrail)]),
            bidirectional: Some(true),
            distance_constraint_m: Some(42.0),
            capacity_constraint: Some("patched-6".to_string()),
            regulatory_basis: Some(vec!["patched-6".to_string()]),
            review_cycle: Some("patched-6".to_string()),
            owner_id: Some(EntityId::new_serial("new6", "new6")),
            proximity_requirement: Some(TextField::plain("patched-6")),
            compatibility_requirement: Some(TextField::plain("patched-6")),
            incompatibility_requirement: Some(TextField::plain("patched-6")),
            separation_requirements: Some(vec![SeparationKind::Visual]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Relationship");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn adjacency_patch_round_trips() {
        let mut item = Adjacency {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("adjacency", "Base Adjacency"), "Base Adjacency") },
            element_a_id: EntityId::new_serial("base7", "base7"),
            element_b_id: EntityId::new_serial("base7", "base7"),
            kind: AdjacencyKind::Required,
            connection: ConnectionKind::Direct,
            separations: Vec::new(),
            weight: 0.0,
            rationale: Some(TextField::default()),
            distance_max_m: Some(0.0),
            distance_min_m: Some(0.0),
            level_constraint: Some(String::new()),
            access_path: Some(String::new()),
            shared_wall: false,
            shared_entry: false,
            traffic_isolation: false,
            circulation_overlap: false,
            conflict_ids: Vec::new(),
            normalized: false,
            verification_status: ValidationStatus::Pending,
            source_relationship_id: Some(EntityId::new_serial("base7", "base7")),
            internal_external_access: Some(String::new()),
        };
        let original = item.clone();
        let patch = AdjacencyPatch {
            name: Some("Patched Adjacency".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            element_a_id: Some(EntityId::new_serial("new7", "new7")),
            element_b_id: Some(EntityId::new_serial("new7", "new7")),
            kind: Some(AdjacencyKind::Preferred),
            connection: Some(ConnectionKind::Indirect),
            separations: Some(vec![SeparationKind::Visual]),
            weight: Some(42.0),
            rationale: Some(TextField::plain("patched-7")),
            distance_max_m: Some(42.0),
            distance_min_m: Some(42.0),
            level_constraint: Some("patched-7".to_string()),
            access_path: Some("patched-7".to_string()),
            shared_wall: Some(true),
            shared_entry: Some(true),
            traffic_isolation: Some(true),
            circulation_overlap: Some(true),
            conflict_ids: Some(vec![EntityId::new_serial("new7", "new7")]),
            normalized: Some(true),
            verification_status: Some(ValidationStatus::Passed),
            source_relationship_id: Some(EntityId::new_serial("new7", "new7")),
            internal_external_access: Some("patched-7".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Adjacency");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn process_patch_round_trips() {
        let mut item = Process {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("process", "Base Process"), "Base Process") },
            code: String::new(),
            category: String::new(),
            trigger: Some(String::new()),
            inputs: Vec::new(),
            outputs: Vec::new(),
            steps: Vec::new(),
            actors: Vec::new(),
            equipment_ids: Vec::new(),
            element_ids: Vec::new(),
            duration: Some(String::new()),
            frequency: Some(String::new()),
            critical_path: false,
            bottlenecks: Vec::new(),
            dependencies: Vec::new(),
            kpis: Vec::new(),
            automation_level: Some(String::new()),
            failure_modes: Vec::new(),
            improvement_opportunities: Vec::new(),
            regulatory_refs: Vec::new(),
            owner_id: Some(EntityId::new_serial("base8", "base8")),
            workflow_type: Some(String::new()),
            handoff_points: Vec::new(),
            quality_gates: Vec::new(),
        };
        let original = item.clone();
        let patch = ProcessPatch {
            name: Some("Patched Process".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-8".to_string()),
            category: Some("patched-8".to_string()),
            trigger: Some("patched-8".to_string()),
            inputs: Some(vec!["patched-8".to_string()]),
            outputs: Some(vec!["patched-8".to_string()]),
            steps: Some(vec!["patched-8".to_string()]),
            actors: Some(vec![EntityId::new_serial("new8", "new8")]),
            equipment_ids: Some(vec![EntityId::new_serial("new8", "new8")]),
            element_ids: Some(vec![EntityId::new_serial("new8", "new8")]),
            duration: Some("patched-8".to_string()),
            frequency: Some("patched-8".to_string()),
            critical_path: Some(true),
            bottlenecks: Some(vec!["patched-8".to_string()]),
            dependencies: Some(vec![EntityId::new_serial("new8", "new8")]),
            kpis: Some(vec!["patched-8".to_string()]),
            automation_level: Some("patched-8".to_string()),
            failure_modes: Some(vec!["patched-8".to_string()]),
            improvement_opportunities: Some(vec!["patched-8".to_string()]),
            regulatory_refs: Some(vec!["patched-8".to_string()]),
            owner_id: Some(EntityId::new_serial("new8", "new8")),
            workflow_type: Some("patched-8".to_string()),
            handoff_points: Some(vec!["patched-8".to_string()]),
            quality_gates: Some(vec!["patched-8".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Process");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn flow_requirement_patch_round_trips() {
        let mut item = FlowRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("flowrequirement", "Base FlowRequirement"), "Base FlowRequirement") },
            from_element_id: EntityId::new_serial("base9", "base9"),
            to_element_id: EntityId::new_serial("base9", "base9"),
            kind: FlowKind::People,
            flow_type: String::new(),
            direction: FlowDirection::OneWay,
            volume: QuantitySpec::default(),
            peak_rate: Some(0.0),
            clear_width_m: Some(0.0),
            clear_height_m: Some(0.0),
            separation_requirements: Vec::new(),
            access_level: AccessLevel::Public,
            time_windows: Vec::new(),
            equipment_clearance: Some(String::new()),
            signage_required: false,
            escort_required: false,
            emergency_route: false,
            barrier_free: false,
            monitoring_required: false,
            process_id: Some(EntityId::new_serial("base9", "base9")),
            conflict_ids: Vec::new(),
            verification_method: Some(String::new()),
        };
        let original = item.clone();
        let patch = FlowRequirementPatch {
            name: Some("Patched FlowRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            from_element_id: Some(EntityId::new_serial("new9", "new9")),
            to_element_id: Some(EntityId::new_serial("new9", "new9")),
            kind: Some(FlowKind::Material),
            flow_type: Some("patched-9".to_string()),
            direction: Some(FlowDirection::TwoWay),
            volume: Some(QuantitySpec::target_unit(42.0, "m2")),
            peak_rate: Some(42.0),
            clear_width_m: Some(42.0),
            clear_height_m: Some(42.0),
            separation_requirements: Some(vec![SeparationKind::Visual]),
            access_level: Some(AccessLevel::Restricted),
            time_windows: Some(vec!["patched-9".to_string()]),
            equipment_clearance: Some("patched-9".to_string()),
            signage_required: Some(true),
            escort_required: Some(true),
            emergency_route: Some(true),
            barrier_free: Some(true),
            monitoring_required: Some(true),
            process_id: Some(EntityId::new_serial("new9", "new9")),
            conflict_ids: Some(vec![EntityId::new_serial("new9", "new9")]),
            verification_method: Some("patched-9".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched FlowRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn access_rule_patch_round_trips() {
        let mut item = AccessRule {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("accessrule", "Base AccessRule"), "Base AccessRule") },
            subject_ids: Vec::new(),
            resource_ids: Vec::new(),
            access_level: AccessLevel::Public,
            access_mode: AccessMode::Unrestricted,
            authentication: Vec::new(),
            authorization: Vec::new(),
            time_restrictions: Vec::new(),
            escort_policy: Some(String::new()),
            visitor_policy: Some(String::new()),
            emergency_override: false,
            audit_required: false,
            badge_required: false,
            biometric_required: false,
            zone_ids: Vec::new(),
            exceptions: Vec::new(),
            regulatory_basis: Vec::new(),
            enforcement_method: Some(String::new()),
            revocation_policy: Some(String::new()),
            training_required: false,
            owner_id: Some(EntityId::new_serial("base10", "base10")),
        };
        let original = item.clone();
        let patch = AccessRulePatch {
            name: Some("Patched AccessRule".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            subject_ids: Some(vec![EntityId::new_serial("new10", "new10")]),
            resource_ids: Some(vec![EntityId::new_serial("new10", "new10")]),
            access_level: Some(AccessLevel::Restricted),
            access_mode: Some(AccessMode::CardControlled),
            authentication: Some(vec!["patched-10".to_string()]),
            authorization: Some(vec!["patched-10".to_string()]),
            time_restrictions: Some(vec!["patched-10".to_string()]),
            escort_policy: Some("patched-10".to_string()),
            visitor_policy: Some("patched-10".to_string()),
            emergency_override: Some(true),
            audit_required: Some(true),
            badge_required: Some(true),
            biometric_required: Some(true),
            zone_ids: Some(vec![EntityId::new_serial("new10", "new10")]),
            exceptions: Some(vec!["patched-10".to_string()]),
            regulatory_basis: Some(vec!["patched-10".to_string()]),
            enforcement_method: Some("patched-10".to_string()),
            revocation_policy: Some("patched-10".to_string()),
            training_required: Some(true),
            owner_id: Some(EntityId::new_serial("new10", "new10")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched AccessRule");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn operational_requirement_patch_round_trips() {
        let mut item = OperationalRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("operationalrequirement", "Base OperationalRequirement"), "Base OperationalRequirement") },
            operation: String::new(),
            service_level: Some(String::new()),
            operating_hours: Some(String::new()),
            staffing: QuantitySpec::default(),
            maintenance_interval: Some(String::new()),
            cleaning_regime: Some(String::new()),
            turnaround_time: Some(String::new()),
            redundancy: Some(String::new()),
            uptime_target: Some(0.0),
            response_time: Some(String::new()),
            equipment_ids: Vec::new(),
            element_ids: Vec::new(),
            process_ids: Vec::new(),
            utilities: Vec::new(),
            waste_streams: Vec::new(),
            contingency_plan: Vec::new(),
            training_requirements: Vec::new(),
            sop_references: Vec::new(),
            kpi_targets: Vec::new(),
            owner_id: Some(EntityId::new_serial("base11", "base11")),
            service_category: Some(String::new()),
            shift_pattern: Some(String::new()),
            sla_target: Some(String::new()),
            escalation_contact_id: Some(EntityId::new_serial("base11", "base11")),
        };
        let original = item.clone();
        let patch = OperationalRequirementPatch {
            name: Some("Patched OperationalRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            operation: Some("patched-11".to_string()),
            service_level: Some("patched-11".to_string()),
            operating_hours: Some("patched-11".to_string()),
            staffing: Some(QuantitySpec::target_unit(42.0, "m2")),
            maintenance_interval: Some("patched-11".to_string()),
            cleaning_regime: Some("patched-11".to_string()),
            turnaround_time: Some("patched-11".to_string()),
            redundancy: Some("patched-11".to_string()),
            uptime_target: Some(42.0),
            response_time: Some("patched-11".to_string()),
            equipment_ids: Some(vec![EntityId::new_serial("new11", "new11")]),
            element_ids: Some(vec![EntityId::new_serial("new11", "new11")]),
            process_ids: Some(vec![EntityId::new_serial("new11", "new11")]),
            utilities: Some(vec!["patched-11".to_string()]),
            waste_streams: Some(vec!["patched-11".to_string()]),
            contingency_plan: Some(vec!["patched-11".to_string()]),
            training_requirements: Some(vec!["patched-11".to_string()]),
            sop_references: Some(vec!["patched-11".to_string()]),
            kpi_targets: Some(vec!["patched-11".to_string()]),
            owner_id: Some(EntityId::new_serial("new11", "new11")),
            service_category: Some("patched-11".to_string()),
            shift_pattern: Some("patched-11".to_string()),
            sla_target: Some("patched-11".to_string()),
            escalation_contact_id: Some(EntityId::new_serial("new11", "new11")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched OperationalRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn equipment_patch_round_trips() {
        let mut item = Equipment {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("equipment", "Base Equipment"), "Base Equipment") },
            code: String::new(),
            category: String::new(),
            manufacturer: Some(String::new()),
            model: Some(String::new()),
            quantity: QuantitySpec::default(),
            dimensions: Some(String::new()),
            weight_kg: Some(0.0),
            power_kw: Some(0.0),
            utility_connections: Vec::new(),
            ventilation: Some(String::new()),
            noise_level_db: Some(0.0),
            clearance: Some(String::new()),
            mounting: Some(String::new()),
            element_ids: Vec::new(),
            activity_ids: Vec::new(),
            maintenance_access: Vec::new(),
            lifecycle_years: Some(0),
            replacement_cost: Some(0.0),
            standards: Vec::new(),
            supplier: Some(String::new()),
            activity_link_ids: Vec::new(),
            installation_requirements: Vec::new(),
            commissioning_notes: Vec::new(),
            spare_parts: Vec::new(),
        };
        let original = item.clone();
        let patch = EquipmentPatch {
            name: Some("Patched Equipment".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-12".to_string()),
            category: Some("patched-12".to_string()),
            manufacturer: Some("patched-12".to_string()),
            model: Some("patched-12".to_string()),
            quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
            dimensions: Some("patched-12".to_string()),
            weight_kg: Some(42.0),
            power_kw: Some(42.0),
            utility_connections: Some(vec!["patched-12".to_string()]),
            ventilation: Some("patched-12".to_string()),
            noise_level_db: Some(42.0),
            clearance: Some("patched-12".to_string()),
            mounting: Some("patched-12".to_string()),
            element_ids: Some(vec![EntityId::new_serial("new12", "new12")]),
            activity_ids: Some(vec![EntityId::new_serial("new12", "new12")]),
            maintenance_access: Some(vec!["patched-12".to_string()]),
            lifecycle_years: Some(7),
            replacement_cost: Some(42.0),
            standards: Some(vec!["patched-12".to_string()]),
            supplier: Some("patched-12".to_string()),
            activity_link_ids: Some(vec![EntityId::new_serial("new12", "new12")]),
            installation_requirements: Some(vec!["patched-12".to_string()]),
            commissioning_notes: Some(vec!["patched-12".to_string()]),
            spare_parts: Some(vec!["patched-12".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Equipment");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn resource_patch_round_trips() {
        let mut item = Resource {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("resource", "Base Resource"), "Base Resource") },
            code: String::new(),
            category: String::new(),
            resource_type: String::new(),
            quantity: QuantitySpec::default(),
            mobility: Some(String::new()),
            sharing_model: Some(String::new()),
            allocation: Some(String::new()),
            element_ids: Vec::new(),
            activity_ids: Vec::new(),
            user_profile_ids: Vec::new(),
            storage_requirement_id: Some(EntityId::new_serial("base13", "base13")),
            durability: Some(String::new()),
            cleaning_requirements: Vec::new(),
            replacement_cycle: Some(String::new()),
            cost_per_unit: Some(0.0),
            supplier: Some(String::new()),
            standards: Vec::new(),
            ergonomic_notes: Vec::new(),
            customization: Vec::new(),
            disposal_notes: Vec::new(),
            furniture_class: Some(String::new()),
            ergonomics_rating: Some(String::new()),
            sharing_ratio: Some(0.0),
        };
        let original = item.clone();
        let patch = ResourcePatch {
            name: Some("Patched Resource".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-13".to_string()),
            category: Some("patched-13".to_string()),
            resource_type: Some("patched-13".to_string()),
            quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
            mobility: Some("patched-13".to_string()),
            sharing_model: Some("patched-13".to_string()),
            allocation: Some("patched-13".to_string()),
            element_ids: Some(vec![EntityId::new_serial("new13", "new13")]),
            activity_ids: Some(vec![EntityId::new_serial("new13", "new13")]),
            user_profile_ids: Some(vec![EntityId::new_serial("new13", "new13")]),
            storage_requirement_id: Some(EntityId::new_serial("new13", "new13")),
            durability: Some("patched-13".to_string()),
            cleaning_requirements: Some(vec!["patched-13".to_string()]),
            replacement_cycle: Some("patched-13".to_string()),
            cost_per_unit: Some(42.0),
            supplier: Some("patched-13".to_string()),
            standards: Some(vec!["patched-13".to_string()]),
            ergonomic_notes: Some(vec!["patched-13".to_string()]),
            customization: Some(vec!["patched-13".to_string()]),
            disposal_notes: Some(vec!["patched-13".to_string()]),
            furniture_class: Some("patched-13".to_string()),
            ergonomics_rating: Some("patched-13".to_string()),
            sharing_ratio: Some(42.0),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Resource");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn storage_requirement_patch_round_trips() {
        let mut item = StorageRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("storagerequirement", "Base StorageRequirement"), "Base StorageRequirement") },
            stored_item: String::new(),
            storage_class: StorageClass::General,
            quantity: QuantitySpec::default(),
            volume_m3: Some(0.0),
            weight_kg: Some(0.0),
            temperature_range: Some(String::new()),
            humidity_range: Some(String::new()),
            security_level: AccessLevel::Public,
            hazard_class: Some(String::new()),
            retention_period: Some(String::new()),
            access_frequency: Some(String::new()),
            element_ids: Vec::new(),
            equipment_ids: Vec::new(),
            handling_equipment: Vec::new(),
            fire_protection: Vec::new(),
            ventilation: Some(String::new()),
            organization_system: Some(String::new()),
            growth_allowance: Some(0.0),
            regulatory_refs: Vec::new(),
            owner_id: Some(EntityId::new_serial("base14", "base14")),
        };
        let original = item.clone();
        let patch = StorageRequirementPatch {
            name: Some("Patched StorageRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            stored_item: Some("patched-14".to_string()),
            storage_class: Some(StorageClass::Secure),
            quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
            volume_m3: Some(42.0),
            weight_kg: Some(42.0),
            temperature_range: Some("patched-14".to_string()),
            humidity_range: Some("patched-14".to_string()),
            security_level: Some(AccessLevel::Restricted),
            hazard_class: Some("patched-14".to_string()),
            retention_period: Some("patched-14".to_string()),
            access_frequency: Some("patched-14".to_string()),
            element_ids: Some(vec![EntityId::new_serial("new14", "new14")]),
            equipment_ids: Some(vec![EntityId::new_serial("new14", "new14")]),
            handling_equipment: Some(vec!["patched-14".to_string()]),
            fire_protection: Some(vec!["patched-14".to_string()]),
            ventilation: Some("patched-14".to_string()),
            organization_system: Some("patched-14".to_string()),
            growth_allowance: Some(42.0),
            regulatory_refs: Some(vec!["patched-14".to_string()]),
            owner_id: Some(EntityId::new_serial("new14", "new14")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched StorageRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn environmental_requirement_patch_round_trips() {
        let mut item = EnvironmentalRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("environmentalrequirement", "Base EnvironmentalRequirement"), "Base EnvironmentalRequirement") },
            parameter_kind: EnvironmentalParameter::Temperature,
            parameter: String::new(),
            target_value: Some(0.0),
            unit: Some(String::new()),
            min_value: Some(0.0),
            max_value: Some(0.0),
            comfort_band: Some(String::new()),
            measurement_method: Some(String::new()),
            monitoring_frequency: Some(String::new()),
            element_ids: Vec::new(),
            occupancy_basis: Some(String::new()),
            seasonal_variation: Vec::new(),
            energy_implications: Vec::new(),
            standards: Vec::new(),
            certification_targets: Vec::new(),
            outdoor_conditions: Vec::new(),
            ventilation_strategy: Some(String::new()),
            daylight_target: Some(String::new()),
            acoustic_target: Some(String::new()),
            iaq_target: Some(String::new()),
            verification_plan: Some(String::new()),
        };
        let original = item.clone();
        let patch = EnvironmentalRequirementPatch {
            name: Some("Patched EnvironmentalRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            parameter_kind: Some(EnvironmentalParameter::Humidity),
            parameter: Some("patched-15".to_string()),
            target_value: Some(42.0),
            unit: Some("patched-15".to_string()),
            min_value: Some(42.0),
            max_value: Some(42.0),
            comfort_band: Some("patched-15".to_string()),
            measurement_method: Some("patched-15".to_string()),
            monitoring_frequency: Some("patched-15".to_string()),
            element_ids: Some(vec![EntityId::new_serial("new15", "new15")]),
            occupancy_basis: Some("patched-15".to_string()),
            seasonal_variation: Some(vec!["patched-15".to_string()]),
            energy_implications: Some(vec!["patched-15".to_string()]),
            standards: Some(vec!["patched-15".to_string()]),
            certification_targets: Some(vec!["patched-15".to_string()]),
            outdoor_conditions: Some(vec!["patched-15".to_string()]),
            ventilation_strategy: Some("patched-15".to_string()),
            daylight_target: Some("patched-15".to_string()),
            acoustic_target: Some("patched-15".to_string()),
            iaq_target: Some("patched-15".to_string()),
            verification_plan: Some("patched-15".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched EnvironmentalRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn human_factor_requirement_patch_round_trips() {
        let mut item = HumanFactorRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("humanfactorrequirement", "Base HumanFactorRequirement"), "Base HumanFactorRequirement") },
            aspect: HumanFactorAspect::Ergonomics,
            factor: String::new(),
            user_profile_ids: Vec::new(),
            activity_ids: Vec::new(),
            ergonomic_criteria: Vec::new(),
            cognitive_load: Some(String::new()),
            visual_demands: Vec::new(),
            auditory_demands: Vec::new(),
            posture_requirements: Vec::new(),
            reach_envelope: Some(String::new()),
            lighting_for_tasks: Vec::new(),
            thermal_comfort: Vec::new(),
            privacy_needs: Vec::new(),
            social_interaction: Vec::new(),
            stress_factors: Vec::new(),
            mitigation_measures: Vec::new(),
            training_needs: Vec::new(),
            standards: Vec::new(),
            research_basis: Vec::new(),
            element_ids: Vec::new(),
            verification_method: Some(String::new()),
        };
        let original = item.clone();
        let patch = HumanFactorRequirementPatch {
            name: Some("Patched HumanFactorRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            aspect: Some(HumanFactorAspect::Cognition),
            factor: Some("patched-16".to_string()),
            user_profile_ids: Some(vec![EntityId::new_serial("new16", "new16")]),
            activity_ids: Some(vec![EntityId::new_serial("new16", "new16")]),
            ergonomic_criteria: Some(vec!["patched-16".to_string()]),
            cognitive_load: Some("patched-16".to_string()),
            visual_demands: Some(vec!["patched-16".to_string()]),
            auditory_demands: Some(vec!["patched-16".to_string()]),
            posture_requirements: Some(vec!["patched-16".to_string()]),
            reach_envelope: Some("patched-16".to_string()),
            lighting_for_tasks: Some(vec!["patched-16".to_string()]),
            thermal_comfort: Some(vec!["patched-16".to_string()]),
            privacy_needs: Some(vec!["patched-16".to_string()]),
            social_interaction: Some(vec!["patched-16".to_string()]),
            stress_factors: Some(vec!["patched-16".to_string()]),
            mitigation_measures: Some(vec!["patched-16".to_string()]),
            training_needs: Some(vec!["patched-16".to_string()]),
            standards: Some(vec!["patched-16".to_string()]),
            research_basis: Some(vec!["patched-16".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new16", "new16")]),
            verification_method: Some("patched-16".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched HumanFactorRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn accessibility_requirement_patch_round_trips() {
        let mut item = AccessibilityRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("accessibilityrequirement", "Base AccessibilityRequirement"), "Base AccessibilityRequirement") },
            standard: String::new(),
            level: Some(String::new()),
            user_profile_ids: Vec::new(),
            element_ids: Vec::new(),
            route_ids: Vec::new(),
            clear_width_m: Some(0.0),
            clear_height_m: Some(0.0),
            turning_circle_m: Some(0.0),
            ramp_slope: Some(0.0),
            lift_required: false,
            tactile_guidance: false,
            hearing_loop: false,
            visual_contrast: false,
            signage_requirements: Vec::new(),
            controls_height: Some(String::new()),
            emergency_evacuation: Vec::new(),
            service_animal_policy: Some(String::new()),
            companion_seating: false,
            verification_plan: Some(String::new()),
            exceptions: Vec::new(),
            wcag_conformance: Some(String::new()),
            universal_design_principles: Vec::new(),
        };
        let original = item.clone();
        let patch = AccessibilityRequirementPatch {
            name: Some("Patched AccessibilityRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            standard: Some("patched-17".to_string()),
            level: Some("patched-17".to_string()),
            user_profile_ids: Some(vec![EntityId::new_serial("new17", "new17")]),
            element_ids: Some(vec![EntityId::new_serial("new17", "new17")]),
            route_ids: Some(vec![EntityId::new_serial("new17", "new17")]),
            clear_width_m: Some(42.0),
            clear_height_m: Some(42.0),
            turning_circle_m: Some(42.0),
            ramp_slope: Some(42.0),
            lift_required: Some(true),
            tactile_guidance: Some(true),
            hearing_loop: Some(true),
            visual_contrast: Some(true),
            signage_requirements: Some(vec!["patched-17".to_string()]),
            controls_height: Some("patched-17".to_string()),
            emergency_evacuation: Some(vec!["patched-17".to_string()]),
            service_animal_policy: Some("patched-17".to_string()),
            companion_seating: Some(true),
            verification_plan: Some("patched-17".to_string()),
            exceptions: Some(vec!["patched-17".to_string()]),
            wcag_conformance: Some("patched-17".to_string()),
            universal_design_principles: Some(vec!["patched-17".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched AccessibilityRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn privacy_requirement_patch_round_trips() {
        let mut item = PrivacyRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("privacyrequirement", "Base PrivacyRequirement"), "Base PrivacyRequirement") },
            privacy_kind: PrivacyKind::Public,
            privacy_type: String::new(),
            level: Some(String::new()),
            subject_ids: Vec::new(),
            element_ids: Vec::new(),
            visual_privacy: Vec::new(),
            acoustic_privacy: Vec::new(),
            data_privacy: Vec::new(),
            screening_required: false,
            enclosure_required: false,
            access_restrictions: Vec::new(),
            observation_risk: Some(String::new()),
            regulatory_basis: Vec::new(),
            cultural_considerations: Vec::new(),
            technology_controls: Vec::new(),
            signage: Vec::new(),
            monitoring_restrictions: Vec::new(),
            retention_policy: Some(String::new()),
            breach_response: Vec::new(),
            owner_id: Some(EntityId::new_serial("base18", "base18")),
        };
        let original = item.clone();
        let patch = PrivacyRequirementPatch {
            name: Some("Patched PrivacyRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            privacy_kind: Some(PrivacyKind::SemiPublic),
            privacy_type: Some("patched-18".to_string()),
            level: Some("patched-18".to_string()),
            subject_ids: Some(vec![EntityId::new_serial("new18", "new18")]),
            element_ids: Some(vec![EntityId::new_serial("new18", "new18")]),
            visual_privacy: Some(vec!["patched-18".to_string()]),
            acoustic_privacy: Some(vec!["patched-18".to_string()]),
            data_privacy: Some(vec!["patched-18".to_string()]),
            screening_required: Some(true),
            enclosure_required: Some(true),
            access_restrictions: Some(vec!["patched-18".to_string()]),
            observation_risk: Some("patched-18".to_string()),
            regulatory_basis: Some(vec!["patched-18".to_string()]),
            cultural_considerations: Some(vec!["patched-18".to_string()]),
            technology_controls: Some(vec!["patched-18".to_string()]),
            signage: Some(vec!["patched-18".to_string()]),
            monitoring_restrictions: Some(vec!["patched-18".to_string()]),
            retention_policy: Some("patched-18".to_string()),
            breach_response: Some(vec!["patched-18".to_string()]),
            owner_id: Some(EntityId::new_serial("new18", "new18")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched PrivacyRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn safety_requirement_patch_round_trips() {
        let mut item = SafetyRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("safetyrequirement", "Base SafetyRequirement"), "Base SafetyRequirement") },
            safety_domain: SafetyDomain::LifeSafety,
            hazard: String::new(),
            risk_level: RiskLevel::Negligible,
            affected_element_ids: Vec::new(),
            affected_user_ids: Vec::new(),
            mitigation_measures: Vec::new(),
            ppe_requirements: Vec::new(),
            emergency_procedures: Vec::new(),
            evacuation_requirements: Vec::new(),
            fire_protection: Vec::new(),
            structural_safety: Vec::new(),
            slip_trip_fall: Vec::new(),
            chemical_safety: Vec::new(),
            electrical_safety: Vec::new(),
            machinery_safety: Vec::new(),
            standards: Vec::new(),
            inspection_frequency: Some(String::new()),
            training_requirements: Vec::new(),
            incident_reporting: Vec::new(),
            residual_risk: Some(String::new()),
        };
        let original = item.clone();
        let patch = SafetyRequirementPatch {
            name: Some("Patched SafetyRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            safety_domain: Some(SafetyDomain::OccupationalHealth),
            hazard: Some("patched-19".to_string()),
            risk_level: Some(RiskLevel::Low),
            affected_element_ids: Some(vec![EntityId::new_serial("new19", "new19")]),
            affected_user_ids: Some(vec![EntityId::new_serial("new19", "new19")]),
            mitigation_measures: Some(vec!["patched-19".to_string()]),
            ppe_requirements: Some(vec!["patched-19".to_string()]),
            emergency_procedures: Some(vec!["patched-19".to_string()]),
            evacuation_requirements: Some(vec!["patched-19".to_string()]),
            fire_protection: Some(vec!["patched-19".to_string()]),
            structural_safety: Some(vec!["patched-19".to_string()]),
            slip_trip_fall: Some(vec!["patched-19".to_string()]),
            chemical_safety: Some(vec!["patched-19".to_string()]),
            electrical_safety: Some(vec!["patched-19".to_string()]),
            machinery_safety: Some(vec!["patched-19".to_string()]),
            standards: Some(vec!["patched-19".to_string()]),
            inspection_frequency: Some("patched-19".to_string()),
            training_requirements: Some(vec!["patched-19".to_string()]),
            incident_reporting: Some(vec!["patched-19".to_string()]),
            residual_risk: Some("patched-19".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched SafetyRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn security_requirement_patch_round_trips() {
        let mut item = SecurityRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("securityrequirement", "Base SecurityRequirement"), "Base SecurityRequirement") },
            control_kind: SecurityControlKind::AccessControl,
            threat: String::new(),
            risk_level: RiskLevel::Negligible,
            asset_ids: Vec::new(),
            zone_ids: Vec::new(),
            access_level: AccessLevel::Public,
            perimeter_controls: Vec::new(),
            surveillance: Vec::new(),
            intrusion_detection: Vec::new(),
            cybersecurity: Vec::new(),
            screening: Vec::new(),
            visitor_management: Vec::new(),
            key_management: Vec::new(),
            standards: Vec::new(),
            response_procedures: Vec::new(),
            drill_frequency: Some(String::new()),
            liaison_contacts: Vec::new(),
            classified_level: Some(String::new()),
            redundancy: Vec::new(),
            audit_requirements: Vec::new(),
        };
        let original = item.clone();
        let patch = SecurityRequirementPatch {
            name: Some("Patched SecurityRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            control_kind: Some(SecurityControlKind::Surveillance),
            threat: Some("patched-20".to_string()),
            risk_level: Some(RiskLevel::Low),
            asset_ids: Some(vec![EntityId::new_serial("new20", "new20")]),
            zone_ids: Some(vec![EntityId::new_serial("new20", "new20")]),
            access_level: Some(AccessLevel::Restricted),
            perimeter_controls: Some(vec!["patched-20".to_string()]),
            surveillance: Some(vec!["patched-20".to_string()]),
            intrusion_detection: Some(vec!["patched-20".to_string()]),
            cybersecurity: Some(vec!["patched-20".to_string()]),
            screening: Some(vec!["patched-20".to_string()]),
            visitor_management: Some(vec!["patched-20".to_string()]),
            key_management: Some(vec!["patched-20".to_string()]),
            standards: Some(vec!["patched-20".to_string()]),
            response_procedures: Some(vec!["patched-20".to_string()]),
            drill_frequency: Some("patched-20".to_string()),
            liaison_contacts: Some(vec!["patched-20".to_string()]),
            classified_level: Some("patched-20".to_string()),
            redundancy: Some(vec!["patched-20".to_string()]),
            audit_requirements: Some(vec!["patched-20".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched SecurityRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn regulatory_requirement_patch_round_trips() {
        let mut item = RegulatoryRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("regulatoryrequirement", "Base RegulatoryRequirement"), "Base RegulatoryRequirement") },
            jurisdiction: String::new(),
            code: String::new(),
            clause: Some(String::new()),
            title: String::new(),
            requirement_text: TextField::default(),
            applicability: Vec::new(),
            element_ids: Vec::new(),
            compliance_method: Some(String::new()),
            evidence_required: Vec::new(),
            authority: Some(String::new()),
            effective_date: Some(String::new()),
            expiry_date: Some(String::new()),
            penalties: Vec::new(),
            exemptions: Vec::new(),
            related_requirement_ids: Vec::new(),
            interpretation_notes: Vec::new(),
            verification_status: ValidationStatus::Pending,
            consultant_refs: Vec::new(),
            update_source: Some(String::new()),
        };
        let original = item.clone();
        let patch = RegulatoryRequirementPatch {
            name: Some("Patched RegulatoryRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            jurisdiction: Some("patched-21".to_string()),
            code: Some("patched-21".to_string()),
            clause: Some("patched-21".to_string()),
            title: Some("patched-21".to_string()),
            requirement_text: Some(TextField::plain("patched-21")),
            applicability: Some(vec!["patched-21".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new21", "new21")]),
            compliance_method: Some("patched-21".to_string()),
            evidence_required: Some(vec!["patched-21".to_string()]),
            authority: Some("patched-21".to_string()),
            effective_date: Some("patched-21".to_string()),
            expiry_date: Some("patched-21".to_string()),
            penalties: Some(vec!["patched-21".to_string()]),
            exemptions: Some(vec!["patched-21".to_string()]),
            related_requirement_ids: Some(vec![EntityId::new_serial("new21", "new21")]),
            interpretation_notes: Some(vec![TaggedNote { tag: "new21".into(), text: "new-note21".into() }]),
            verification_status: Some(ValidationStatus::Passed),
            consultant_refs: Some(vec![EntityId::new_serial("new21", "new21")]),
            update_source: Some("patched-21".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched RegulatoryRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn site_context_patch_round_trips() {
        let mut item = SiteContext {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("sitecontext", "Base SiteContext"), "Base SiteContext") },
            site_name: String::new(),
            address: Some(String::new()),
            latitude: Some(0.0),
            longitude: Some(0.0),
            elevation_m: Some(0.0),
            climate_zone: Some(String::new()),
            seismic_zone: Some(String::new()),
            flood_risk: Some(String::new()),
            soil_conditions: Vec::new(),
            utilities_available: Vec::new(),
            access_roads: Vec::new(),
            public_transit: Vec::new(),
            neighbors: Vec::new(),
            views: Vec::new(),
            noise_sources: Vec::new(),
            environmental_constraints: Vec::new(),
            heritage_constraints: Vec::new(),
            zoning: Some(String::new()),
            max_height_m: Some(0.0),
            max_coverage: Some(0.0),
        };
        let original = item.clone();
        let patch = SiteContextPatch {
            name: Some("Patched SiteContext".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            site_name: Some("patched-22".to_string()),
            address: Some("patched-22".to_string()),
            latitude: Some(42.0),
            longitude: Some(42.0),
            elevation_m: Some(42.0),
            climate_zone: Some("patched-22".to_string()),
            seismic_zone: Some("patched-22".to_string()),
            flood_risk: Some("patched-22".to_string()),
            soil_conditions: Some(vec!["patched-22".to_string()]),
            utilities_available: Some(vec!["patched-22".to_string()]),
            access_roads: Some(vec!["patched-22".to_string()]),
            public_transit: Some(vec!["patched-22".to_string()]),
            neighbors: Some(vec!["patched-22".to_string()]),
            views: Some(vec!["patched-22".to_string()]),
            noise_sources: Some(vec!["patched-22".to_string()]),
            environmental_constraints: Some(vec!["patched-22".to_string()]),
            heritage_constraints: Some(vec!["patched-22".to_string()]),
            zoning: Some("patched-22".to_string()),
            max_height_m: Some(42.0),
            max_coverage: Some(42.0),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched SiteContext");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn organizational_requirement_patch_round_trips() {
        let mut item = OrganizationalRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("organizationalrequirement", "Base OrganizationalRequirement"), "Base OrganizationalRequirement") },
            department: String::new(),
            reporting_line: Some(String::new()),
            headcount: QuantitySpec::default(),
            growth_plan_id: Some(EntityId::new_serial("base23", "base23")),
            work_patterns: Vec::new(),
            collaboration_model: Some(String::new()),
            hierarchy_levels: Vec::new(),
            decision_making: Vec::new(),
            culture_notes: Vec::new(),
            change_readiness: Some(String::new()),
            union_considerations: Vec::new(),
            training_needs: Vec::new(),
            element_ids: Vec::new(),
            stakeholder_ids: Vec::new(),
            service_requirement_ids: Vec::new(),
            branding_requirements: Vec::new(),
            wellness_plugins: Vec::new(),
            diversity_goals: Vec::new(),
            owner_id: Some(EntityId::new_serial("base23", "base23")),
        };
        let original = item.clone();
        let patch = OrganizationalRequirementPatch {
            name: Some("Patched OrganizationalRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            department: Some("patched-23".to_string()),
            reporting_line: Some("patched-23".to_string()),
            headcount: Some(QuantitySpec::target_unit(42.0, "m2")),
            growth_plan_id: Some(EntityId::new_serial("new23", "new23")),
            work_patterns: Some(vec!["patched-23".to_string()]),
            collaboration_model: Some("patched-23".to_string()),
            hierarchy_levels: Some(vec!["patched-23".to_string()]),
            decision_making: Some(vec!["patched-23".to_string()]),
            culture_notes: Some(vec!["patched-23".to_string()]),
            change_readiness: Some("patched-23".to_string()),
            union_considerations: Some(vec!["patched-23".to_string()]),
            training_needs: Some(vec!["patched-23".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new23", "new23")]),
            stakeholder_ids: Some(vec![EntityId::new_serial("new23", "new23")]),
            service_requirement_ids: Some(vec![EntityId::new_serial("new23", "new23")]),
            branding_requirements: Some(vec!["patched-23".to_string()]),
            wellness_plugins: Some(vec!["patched-23".to_string()]),
            diversity_goals: Some(vec!["patched-23".to_string()]),
            owner_id: Some(EntityId::new_serial("new23", "new23")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched OrganizationalRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn service_requirement_patch_round_trips() {
        let mut item = ServiceRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("servicerequirement", "Base ServiceRequirement"), "Base ServiceRequirement") },
            service_name: String::new(),
            service_type: String::new(),
            provider: Some(String::new()),
            service_level: Some(String::new()),
            operating_hours: Some(String::new()),
            capacity: QuantitySpec::default(),
            response_time: Some(String::new()),
            queue_management: Vec::new(),
            customer_profiles: Vec::new(),
            element_ids: Vec::new(),
            equipment_ids: Vec::new(),
            staffing: QuantitySpec::default(),
            quality_metrics: Vec::new(),
            cost_model: Some(String::new()),
            contract_refs: Vec::new(),
            dependencies: Vec::new(),
            failure_impact: Some(String::new()),
            backup_service: Vec::new(),
            feedback_channels: Vec::new(),
        };
        let original = item.clone();
        let patch = ServiceRequirementPatch {
            name: Some("Patched ServiceRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            service_name: Some("patched-24".to_string()),
            service_type: Some("patched-24".to_string()),
            provider: Some("patched-24".to_string()),
            service_level: Some("patched-24".to_string()),
            operating_hours: Some("patched-24".to_string()),
            capacity: Some(QuantitySpec::target_unit(42.0, "m2")),
            response_time: Some("patched-24".to_string()),
            queue_management: Some(vec!["patched-24".to_string()]),
            customer_profiles: Some(vec![EntityId::new_serial("new24", "new24")]),
            element_ids: Some(vec![EntityId::new_serial("new24", "new24")]),
            equipment_ids: Some(vec![EntityId::new_serial("new24", "new24")]),
            staffing: Some(QuantitySpec::target_unit(42.0, "m2")),
            quality_metrics: Some(vec!["patched-24".to_string()]),
            cost_model: Some("patched-24".to_string()),
            contract_refs: Some(vec!["patched-24".to_string()]),
            dependencies: Some(vec![EntityId::new_serial("new24", "new24")]),
            failure_impact: Some("patched-24".to_string()),
            backup_service: Some(vec!["patched-24".to_string()]),
            feedback_channels: Some(vec!["patched-24".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ServiceRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn infrastructure_requirement_patch_round_trips() {
        let mut item = InfrastructureRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("infrastructurerequirement", "Base InfrastructureRequirement"), "Base InfrastructureRequirement") },
            system: String::new(),
            category: String::new(),
            capacity: QuantitySpec::default(),
            redundancy: Some(String::new()),
            distribution: Vec::new(),
            entry_points: Vec::new(),
            utility_source: Some(String::new()),
            standby_power: false,
            monitoring: Vec::new(),
            maintenance_access: Vec::new(),
            standards: Vec::new(),
            element_ids: Vec::new(),
            peak_demand: Some(0.0),
            diversity_factor: Some(0.0),
            future_expansion: Vec::new(),
            interface_requirements: Vec::new(),
            commissioning: Vec::new(),
            lifecycle_cost: Some(0.0),
            owner_id: Some(EntityId::new_serial("base25", "base25")),
        };
        let original = item.clone();
        let patch = InfrastructureRequirementPatch {
            name: Some("Patched InfrastructureRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            system: Some("patched-25".to_string()),
            category: Some("patched-25".to_string()),
            capacity: Some(QuantitySpec::target_unit(42.0, "m2")),
            redundancy: Some("patched-25".to_string()),
            distribution: Some(vec!["patched-25".to_string()]),
            entry_points: Some(vec!["patched-25".to_string()]),
            utility_source: Some("patched-25".to_string()),
            standby_power: Some(true),
            monitoring: Some(vec!["patched-25".to_string()]),
            maintenance_access: Some(vec!["patched-25".to_string()]),
            standards: Some(vec!["patched-25".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new25", "new25")]),
            peak_demand: Some(42.0),
            diversity_factor: Some(42.0),
            future_expansion: Some(vec!["patched-25".to_string()]),
            interface_requirements: Some(vec!["patched-25".to_string()]),
            commissioning: Some(vec!["patched-25".to_string()]),
            lifecycle_cost: Some(42.0),
            owner_id: Some(EntityId::new_serial("new25", "new25")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched InfrastructureRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn information_requirement_patch_round_trips() {
        let mut item = InformationRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("informationrequirement", "Base InformationRequirement"), "Base InformationRequirement") },
            information_type: String::new(),
            format: Some(String::new()),
            source_system: Some(String::new()),
            destination_systems: Vec::new(),
            update_frequency: Some(String::new()),
            retention_period: Some(String::new()),
            access_controls: Vec::new(),
            classification: Some(String::new()),
            quality_criteria: Vec::new(),
            metadata_requirements: Vec::new(),
            integration_points: Vec::new(),
            backup_requirements: Vec::new(),
            disaster_recovery: Vec::new(),
            privacy_controls: Vec::new(),
            audit_trail: false,
            element_ids: Vec::new(),
            stakeholder_ids: Vec::new(),
            standards: Vec::new(),
            owner_id: Some(EntityId::new_serial("base26", "base26")),
        };
        let original = item.clone();
        let patch = InformationRequirementPatch {
            name: Some("Patched InformationRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            information_type: Some("patched-26".to_string()),
            format: Some("patched-26".to_string()),
            source_system: Some("patched-26".to_string()),
            destination_systems: Some(vec!["patched-26".to_string()]),
            update_frequency: Some("patched-26".to_string()),
            retention_period: Some("patched-26".to_string()),
            access_controls: Some(vec!["patched-26".to_string()]),
            classification: Some("patched-26".to_string()),
            quality_criteria: Some(vec!["patched-26".to_string()]),
            metadata_requirements: Some(vec!["patched-26".to_string()]),
            integration_points: Some(vec!["patched-26".to_string()]),
            backup_requirements: Some(vec!["patched-26".to_string()]),
            disaster_recovery: Some(vec!["patched-26".to_string()]),
            privacy_controls: Some(vec!["patched-26".to_string()]),
            audit_trail: Some(true),
            element_ids: Some(vec![EntityId::new_serial("new26", "new26")]),
            stakeholder_ids: Some(vec![EntityId::new_serial("new26", "new26")]),
            standards: Some(vec!["patched-26".to_string()]),
            owner_id: Some(EntityId::new_serial("new26", "new26")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched InformationRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn communication_requirement_patch_round_trips() {
        let mut item = CommunicationRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("communicationrequirement", "Base CommunicationRequirement"), "Base CommunicationRequirement") },
            channel: String::new(),
            audience_ids: Vec::new(),
            message_types: Vec::new(),
            frequency: Some(String::new()),
            medium: Vec::new(),
            language: Vec::new(),
            accessibility: Vec::new(),
            emergency_use: false,
            two_way: false,
            recording_policy: Some(String::new()),
            signage_locations: Vec::new(),
            technology: Vec::new(),
            escalation_path: Vec::new(),
            feedback_loop: false,
            privacy_controls: Vec::new(),
            element_ids: Vec::new(),
            standards: Vec::new(),
            owner_id: Some(EntityId::new_serial("base27", "base27")),
            templates: Vec::new(),
        };
        let original = item.clone();
        let patch = CommunicationRequirementPatch {
            name: Some("Patched CommunicationRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            channel: Some("patched-27".to_string()),
            audience_ids: Some(vec![EntityId::new_serial("new27", "new27")]),
            message_types: Some(vec!["patched-27".to_string()]),
            frequency: Some("patched-27".to_string()),
            medium: Some(vec!["patched-27".to_string()]),
            language: Some(vec!["patched-27".to_string()]),
            accessibility: Some(vec!["patched-27".to_string()]),
            emergency_use: Some(true),
            two_way: Some(true),
            recording_policy: Some("patched-27".to_string()),
            signage_locations: Some(vec!["patched-27".to_string()]),
            technology: Some(vec!["patched-27".to_string()]),
            escalation_path: Some(vec!["patched-27".to_string()]),
            feedback_loop: Some(true),
            privacy_controls: Some(vec!["patched-27".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new27", "new27")]),
            standards: Some(vec!["patched-27".to_string()]),
            owner_id: Some(EntityId::new_serial("new27", "new27")),
            templates: Some(vec![EntityId::new_serial("new27", "new27")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched CommunicationRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn wayfinding_requirement_patch_round_trips() {
        let mut item = WayfindingRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("wayfindingrequirement", "Base WayfindingRequirement"), "Base WayfindingRequirement") },
            user_profile_ids: Vec::new(),
            element_ids: Vec::new(),
            destination_types: Vec::new(),
            signage_types: Vec::new(),
            languages: Vec::new(),
            tactile_required: false,
            audio_required: false,
            digital_wayfinding: false,
            landmark_strategy: Vec::new(),
            color_coding: Vec::new(),
            symbol_standards: Vec::new(),
            decision_points: Vec::new(),
            maximum_signage_distance_m: Some(0.0),
            lighting_requirements: Vec::new(),
            maintenance_plan: Some(String::new()),
            emergency_egress: Vec::new(),
            visitor_journey: Vec::new(),
            staff_journey: Vec::new(),
            brand_integration: Vec::new(),
        };
        let original = item.clone();
        let patch = WayfindingRequirementPatch {
            name: Some("Patched WayfindingRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            user_profile_ids: Some(vec![EntityId::new_serial("new28", "new28")]),
            element_ids: Some(vec![EntityId::new_serial("new28", "new28")]),
            destination_types: Some(vec!["patched-28".to_string()]),
            signage_types: Some(vec!["patched-28".to_string()]),
            languages: Some(vec!["patched-28".to_string()]),
            tactile_required: Some(true),
            audio_required: Some(true),
            digital_wayfinding: Some(true),
            landmark_strategy: Some(vec!["patched-28".to_string()]),
            color_coding: Some(vec!["patched-28".to_string()]),
            symbol_standards: Some(vec!["patched-28".to_string()]),
            decision_points: Some(vec!["patched-28".to_string()]),
            maximum_signage_distance_m: Some(42.0),
            lighting_requirements: Some(vec!["patched-28".to_string()]),
            maintenance_plan: Some("patched-28".to_string()),
            emergency_egress: Some(vec!["patched-28".to_string()]),
            visitor_journey: Some(vec!["patched-28".to_string()]),
            staff_journey: Some(vec!["patched-28".to_string()]),
            brand_integration: Some(vec!["patched-28".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched WayfindingRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn schedule_requirement_patch_round_trips() {
        let mut item = ScheduleRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("schedulerequirement", "Base ScheduleRequirement"), "Base ScheduleRequirement") },
            milestone: String::new(),
            phase: DeliveryPhase::Concept,
            start_date: Some(String::new()),
            end_date: Some(String::new()),
            duration: Some(String::new()),
            dependencies: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
            critical: false,
            float_days: Some(0),
            resource_requirements: Vec::new(),
            occupancy_impact: Vec::new(),
            phasing_strategy: Some(String::new()),
            decant_requirements: Vec::new(),
            commissioning_window: Some(String::new()),
            stakeholder_ids: Vec::new(),
            risk_ids: Vec::new(),
            contingency_days: Some(0),
            reporting_cadence: Some(String::new()),
            owner_id: Some(EntityId::new_serial("base29", "base29")),
        };
        let original = item.clone();
        let patch = ScheduleRequirementPatch {
            name: Some("Patched ScheduleRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            milestone: Some("patched-29".to_string()),
            phase: Some(DeliveryPhase::Schematic),
            start_date: Some("patched-29".to_string()),
            end_date: Some("patched-29".to_string()),
            duration: Some("patched-29".to_string()),
            dependencies: Some(vec![EntityId::new_serial("new29", "new29")]),
            predecessors: Some(vec![EntityId::new_serial("new29", "new29")]),
            successors: Some(vec![EntityId::new_serial("new29", "new29")]),
            critical: Some(true),
            float_days: Some(7),
            resource_requirements: Some(vec!["patched-29".to_string()]),
            occupancy_impact: Some(vec!["patched-29".to_string()]),
            phasing_strategy: Some("patched-29".to_string()),
            decant_requirements: Some(vec!["patched-29".to_string()]),
            commissioning_window: Some("patched-29".to_string()),
            stakeholder_ids: Some(vec![EntityId::new_serial("new29", "new29")]),
            risk_ids: Some(vec![EntityId::new_serial("new29", "new29")]),
            contingency_days: Some(7),
            reporting_cadence: Some("patched-29".to_string()),
            owner_id: Some(EntityId::new_serial("new29", "new29")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ScheduleRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn flexibility_requirement_patch_round_trips() {
        let mut item = FlexibilityRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("flexibilityrequirement", "Base FlexibilityRequirement"), "Base FlexibilityRequirement") },
            flexibility_type: String::new(),
            element_ids: Vec::new(),
            adaptation_scenarios: Vec::new(),
            modularity_level: Some(String::new()),
            reconfiguration_time: Some(String::new()),
            cost_of_change: Some(0.0),
            technology_readiness: Some(String::new()),
            future_function_ids: Vec::new(),
            demountable_partitions: false,
            raised_floor: false,
            overhead_services: false,
            expansion_direction: Vec::new(),
            contraction_scenario: Vec::new(),
            multi_use_potential: Vec::new(),
            furniture_strategy: Vec::new(),
            infrastructure_spare_capacity: Vec::new(),
            lease_implications: Vec::new(),
            owner_id: Some(EntityId::new_serial("base30", "base30")),
        };
        let original = item.clone();
        let patch = FlexibilityRequirementPatch {
            name: Some("Patched FlexibilityRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            flexibility_type: Some("patched-30".to_string()),
            element_ids: Some(vec![EntityId::new_serial("new30", "new30")]),
            adaptation_scenarios: Some(vec!["patched-30".to_string()]),
            modularity_level: Some("patched-30".to_string()),
            reconfiguration_time: Some("patched-30".to_string()),
            cost_of_change: Some(42.0),
            technology_readiness: Some("patched-30".to_string()),
            future_function_ids: Some(vec![EntityId::new_serial("new30", "new30")]),
            demountable_partitions: Some(true),
            raised_floor: Some(true),
            overhead_services: Some(true),
            expansion_direction: Some(vec!["patched-30".to_string()]),
            contraction_scenario: Some(vec!["patched-30".to_string()]),
            multi_use_potential: Some(vec!["patched-30".to_string()]),
            furniture_strategy: Some(vec!["patched-30".to_string()]),
            infrastructure_spare_capacity: Some(vec!["patched-30".to_string()]),
            lease_implications: Some(vec!["patched-30".to_string()]),
            owner_id: Some(EntityId::new_serial("new30", "new30")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched FlexibilityRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn growth_plan_patch_round_trips() {
        let mut item = GrowthPlan {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("growthplan", "Base GrowthPlan"), "Base GrowthPlan") },
            horizon_years: 0,
            growth_rate: Some(0.0),
            headcount_growth: QuantitySpec::default(),
            area_growth: QuantitySpec::default(),
            phases: Vec::new(),
            trigger_events: Vec::new(),
            expansion_element_ids: Vec::new(),
            reserve_areas: Vec::new(),
            infrastructure_headroom: Vec::new(),
            budget_envelope: Some(0.0),
            funding_sources: Vec::new(),
            risk_factors: Vec::new(),
            decision_points: Vec::new(),
            scenario_ids: Vec::new(),
            decommission_plan: Vec::new(),
            relocation_strategy: Vec::new(),
            stakeholder_impact: Vec::new(),
            regulatory_considerations: Vec::new(),
            owner_id: Some(EntityId::new_serial("base31", "base31")),
        };
        let original = item.clone();
        let patch = GrowthPlanPatch {
            name: Some("Patched GrowthPlan".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            horizon_years: Some(7),
            growth_rate: Some(42.0),
            headcount_growth: Some(QuantitySpec::target_unit(42.0, "m2")),
            area_growth: Some(QuantitySpec::target_unit(42.0, "m2")),
            phases: Some(vec!["patched-31".to_string()]),
            trigger_events: Some(vec!["patched-31".to_string()]),
            expansion_element_ids: Some(vec![EntityId::new_serial("new31", "new31")]),
            reserve_areas: Some(vec!["patched-31".to_string()]),
            infrastructure_headroom: Some(vec!["patched-31".to_string()]),
            budget_envelope: Some(42.0),
            funding_sources: Some(vec!["patched-31".to_string()]),
            risk_factors: Some(vec![EntityId::new_serial("new31", "new31")]),
            decision_points: Some(vec![EntityId::new_serial("new31", "new31")]),
            scenario_ids: Some(vec![EntityId::new_serial("new31", "new31")]),
            decommission_plan: Some(vec!["patched-31".to_string()]),
            relocation_strategy: Some(vec!["patched-31".to_string()]),
            stakeholder_impact: Some(vec!["patched-31".to_string()]),
            regulatory_considerations: Some(vec!["patched-31".to_string()]),
            owner_id: Some(EntityId::new_serial("new31", "new31")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched GrowthPlan");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn sustainability_requirement_patch_round_trips() {
        let mut item = SustainabilityRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("sustainabilityrequirement", "Base SustainabilityRequirement"), "Base SustainabilityRequirement") },
            topic: String::new(),
            target: Some(String::new()),
            metric: Some(String::new()),
            baseline: Some(0.0),
            target_value: Some(0.0),
            unit: Some(String::new()),
            certification: Vec::new(),
            standards: Vec::new(),
            element_ids: Vec::new(),
            strategies: Vec::new(),
            materials_preferences: Vec::new(),
            energy_strategy: Vec::new(),
            water_strategy: Vec::new(),
            waste_strategy: Vec::new(),
            biodiversity: Vec::new(),
            embodied_carbon: Some(0.0),
            operational_carbon: Some(0.0),
            reporting_requirements: Vec::new(),
            verification_plan: Some(String::new()),
            owner_id: Some(EntityId::new_serial("base32", "base32")),
        };
        let original = item.clone();
        let patch = SustainabilityRequirementPatch {
            name: Some("Patched SustainabilityRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            topic: Some("patched-32".to_string()),
            target: Some("patched-32".to_string()),
            metric: Some("patched-32".to_string()),
            baseline: Some(42.0),
            target_value: Some(42.0),
            unit: Some("patched-32".to_string()),
            certification: Some(vec!["patched-32".to_string()]),
            standards: Some(vec!["patched-32".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new32", "new32")]),
            strategies: Some(vec!["patched-32".to_string()]),
            materials_preferences: Some(vec!["patched-32".to_string()]),
            energy_strategy: Some(vec!["patched-32".to_string()]),
            water_strategy: Some(vec!["patched-32".to_string()]),
            waste_strategy: Some(vec!["patched-32".to_string()]),
            biodiversity: Some(vec!["patched-32".to_string()]),
            embodied_carbon: Some(42.0),
            operational_carbon: Some(42.0),
            reporting_requirements: Some(vec!["patched-32".to_string()]),
            verification_plan: Some("patched-32".to_string()),
            owner_id: Some(EntityId::new_serial("new32", "new32")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched SustainabilityRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn resilience_requirement_patch_round_trips() {
        let mut item = ResilienceRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("resiliencerequirement", "Base ResilienceRequirement"), "Base ResilienceRequirement") },
            hazard: String::new(),
            risk_level: RiskLevel::Negligible,
            scenario: Some(String::new()),
            recovery_time: Some(String::new()),
            recovery_point: Some(String::new()),
            redundancy: Vec::new(),
            hardening_measures: Vec::new(),
            backup_systems: Vec::new(),
            alternate_sites: Vec::new(),
            supply_chain: Vec::new(),
            communication_plan: Vec::new(),
            drill_requirements: Vec::new(),
            element_ids: Vec::new(),
            infrastructure_ids: Vec::new(),
            standards: Vec::new(),
            insurance_implications: Vec::new(),
            climate_adaptation: Vec::new(),
            owner_id: Some(EntityId::new_serial("base33", "base33")),
            verification_plan: Some(String::new()),
        };
        let original = item.clone();
        let patch = ResilienceRequirementPatch {
            name: Some("Patched ResilienceRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            hazard: Some("patched-33".to_string()),
            risk_level: Some(RiskLevel::Low),
            scenario: Some("patched-33".to_string()),
            recovery_time: Some("patched-33".to_string()),
            recovery_point: Some("patched-33".to_string()),
            redundancy: Some(vec!["patched-33".to_string()]),
            hardening_measures: Some(vec!["patched-33".to_string()]),
            backup_systems: Some(vec!["patched-33".to_string()]),
            alternate_sites: Some(vec!["patched-33".to_string()]),
            supply_chain: Some(vec!["patched-33".to_string()]),
            communication_plan: Some(vec!["patched-33".to_string()]),
            drill_requirements: Some(vec!["patched-33".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new33", "new33")]),
            infrastructure_ids: Some(vec![EntityId::new_serial("new33", "new33")]),
            standards: Some(vec!["patched-33".to_string()]),
            insurance_implications: Some(vec!["patched-33".to_string()]),
            climate_adaptation: Some(vec!["patched-33".to_string()]),
            owner_id: Some(EntityId::new_serial("new33", "new33")),
            verification_plan: Some("patched-33".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ResilienceRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn cost_requirement_patch_round_trips() {
        let mut item = CostRequirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("costrequirement", "Base CostRequirement"), "Base CostRequirement") },
            cost_item: String::new(),
            basis: CostBasis::Capital,
            amount: Some(0.0),
            currency: String::new(),
            quantity_basis: Some(String::new()),
            unit_cost: Some(0.0),
            contingency_percent: Some(0.0),
            escalation_rate: Some(0.0),
            funding_source: Some(String::new()),
            element_ids: Vec::new(),
            requirement_ids: Vec::new(),
            phase: Some(DeliveryPhase::Concept),
            cash_flow_profile: Vec::new(),
            value_engineering_notes: Vec::new(),
            benchmark_ref: Some(EntityId::new_serial("base34", "base34")),
            approval_status: ValidationStatus::Pending,
            owner_id: Some(EntityId::new_serial("base34", "base34")),
            assumptions: Vec::new(),
            sensitivity_factors: Vec::new(),
        };
        let original = item.clone();
        let patch = CostRequirementPatch {
            name: Some("Patched CostRequirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            cost_item: Some("patched-34".to_string()),
            basis: Some(CostBasis::Operational),
            amount: Some(42.0),
            currency: Some("patched-34".to_string()),
            quantity_basis: Some("patched-34".to_string()),
            unit_cost: Some(42.0),
            contingency_percent: Some(42.0),
            escalation_rate: Some(42.0),
            funding_source: Some("patched-34".to_string()),
            element_ids: Some(vec![EntityId::new_serial("new34", "new34")]),
            requirement_ids: Some(vec![EntityId::new_serial("new34", "new34")]),
            phase: Some(DeliveryPhase::Schematic),
            cash_flow_profile: Some(vec!["patched-34".to_string()]),
            value_engineering_notes: Some(vec!["patched-34".to_string()]),
            benchmark_ref: Some(EntityId::new_serial("new34", "new34")),
            approval_status: Some(ValidationStatus::Passed),
            owner_id: Some(EntityId::new_serial("new34", "new34")),
            assumptions: Some(vec!["patched-34".to_string()]),
            sensitivity_factors: Some(vec!["patched-34".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched CostRequirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn delivery_constraint_patch_round_trips() {
        let mut item = DeliveryConstraint {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("deliveryconstraint", "Base DeliveryConstraint"), "Base DeliveryConstraint") },
            constraint_type: String::new(),
            constraint_details: TextField::default(),
            phase: DeliveryPhase::Concept,
            hard_deadline: Some(String::new()),
            soft_deadline: Some(String::new()),
            impacted_element_ids: Vec::new(),
            impacted_requirement_ids: Vec::new(),
            work_hours: Some(String::new()),
            noise_restrictions: Vec::new(),
            access_restrictions: Vec::new(),
            site_logistics: Vec::new(),
            procurement_lead_time: Some(String::new()),
            approval_gates: Vec::new(),
            occupancy_constraints: Vec::new(),
            weather_windows: Vec::new(),
            penalty_clauses: Vec::new(),
            mitigation_options: Vec::new(),
            owner_id: Some(EntityId::new_serial("base35", "base35")),
            risk_ids: Vec::new(),
            constraint_status: LifecycleStatus::Draft,
        };
        let original = item.clone();
        let patch = DeliveryConstraintPatch {
            name: Some("Patched DeliveryConstraint".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            constraint_type: Some("patched-35".to_string()),
            constraint_details: Some(TextField::plain("patched-35")),
            phase: Some(DeliveryPhase::Schematic),
            hard_deadline: Some("patched-35".to_string()),
            soft_deadline: Some("patched-35".to_string()),
            impacted_element_ids: Some(vec![EntityId::new_serial("new35", "new35")]),
            impacted_requirement_ids: Some(vec![EntityId::new_serial("new35", "new35")]),
            work_hours: Some("patched-35".to_string()),
            noise_restrictions: Some(vec!["patched-35".to_string()]),
            access_restrictions: Some(vec!["patched-35".to_string()]),
            site_logistics: Some(vec!["patched-35".to_string()]),
            procurement_lead_time: Some("patched-35".to_string()),
            approval_gates: Some(vec!["patched-35".to_string()]),
            occupancy_constraints: Some(vec!["patched-35".to_string()]),
            weather_windows: Some(vec!["patched-35".to_string()]),
            penalty_clauses: Some(vec!["patched-35".to_string()]),
            mitigation_options: Some(vec!["patched-35".to_string()]),
            owner_id: Some(EntityId::new_serial("new35", "new35")),
            risk_ids: Some(vec![EntityId::new_serial("new35", "new35")]),
            constraint_status: Some(LifecycleStatus::Proposed),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched DeliveryConstraint");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn risk_patch_round_trips() {
        let mut item = Risk {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("risk", "Base Risk"), "Base Risk") },
            risk_statement: TextField::default(),
            category: String::new(),
            probability: RiskLevel::Negligible,
            impact: RiskLevel::Negligible,
            risk_score: Some(0.0),
            causes: Vec::new(),
            effects: Vec::new(),
            affected_element_ids: Vec::new(),
            affected_requirement_ids: Vec::new(),
            mitigation: Vec::new(),
            contingency: Vec::new(),
            owner_id: Some(EntityId::new_serial("base36", "base36")),
            review_date: Some(String::new()),
            trigger_indicators: Vec::new(),
            residual_probability: Some(RiskLevel::Negligible),
            residual_impact: Some(RiskLevel::Negligible),
            related_conflict_ids: Vec::new(),
            escalation_path: Vec::new(),
            monitoring_plan: Some(String::new()),
        };
        let original = item.clone();
        let patch = RiskPatch {
            name: Some("Patched Risk".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            risk_statement: Some(TextField::plain("patched-36")),
            category: Some("patched-36".to_string()),
            probability: Some(RiskLevel::Low),
            impact: Some(RiskLevel::Low),
            risk_score: Some(42.0),
            causes: Some(vec!["patched-36".to_string()]),
            effects: Some(vec!["patched-36".to_string()]),
            affected_element_ids: Some(vec![EntityId::new_serial("new36", "new36")]),
            affected_requirement_ids: Some(vec![EntityId::new_serial("new36", "new36")]),
            mitigation: Some(vec!["patched-36".to_string()]),
            contingency: Some(vec!["patched-36".to_string()]),
            owner_id: Some(EntityId::new_serial("new36", "new36")),
            review_date: Some("patched-36".to_string()),
            trigger_indicators: Some(vec!["patched-36".to_string()]),
            residual_probability: Some(RiskLevel::Low),
            residual_impact: Some(RiskLevel::Low),
            related_conflict_ids: Some(vec![EntityId::new_serial("new36", "new36")]),
            escalation_path: Some(vec!["patched-36".to_string()]),
            monitoring_plan: Some("patched-36".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Risk");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn conflict_patch_round_trips() {
        let mut item = Conflict {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("conflict", "Base Conflict"), "Base Conflict") },
            kind: ConflictKind::Adjacency,
            summary: TextField::default(),
            entity_a_id: EntityId::new_serial("base37", "base37"),
            entity_b_id: EntityId::new_serial("base37", "base37"),
            severity: IssueSeverity::Cosmetic,
            detected_by: Some(String::new()),
            detection_date: Some(String::new()),
            trade_off_options: Vec::new(),
            recommended_resolution: Some(TextField::default()),
            decision_id: Some(EntityId::new_serial("base37", "base37")),
            stakeholder_ids: Vec::new(),
            requirement_ids: Vec::new(),
            cost_impact: Some(0.0),
            schedule_impact: Some(String::new()),
            quality_impact: Vec::new(),
            resolution_status: ValidationStatus::Pending,
            owner_id: Some(EntityId::new_serial("base37", "base37")),
            escalation_level: Some(String::new()),
            related_risk_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = ConflictPatch {
            name: Some("Patched Conflict".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            kind: Some(ConflictKind::Capacity),
            summary: Some(TextField::plain("patched-37")),
            entity_a_id: Some(EntityId::new_serial("new37", "new37")),
            entity_b_id: Some(EntityId::new_serial("new37", "new37")),
            severity: Some(IssueSeverity::Minor),
            detected_by: Some("patched-37".to_string()),
            detection_date: Some("patched-37".to_string()),
            trade_off_options: Some(vec!["patched-37".to_string()]),
            recommended_resolution: Some(TextField::plain("patched-37")),
            decision_id: Some(EntityId::new_serial("new37", "new37")),
            stakeholder_ids: Some(vec![EntityId::new_serial("new37", "new37")]),
            requirement_ids: Some(vec![EntityId::new_serial("new37", "new37")]),
            cost_impact: Some(42.0),
            schedule_impact: Some("patched-37".to_string()),
            quality_impact: Some(vec!["patched-37".to_string()]),
            resolution_status: Some(ValidationStatus::Passed),
            owner_id: Some(EntityId::new_serial("new37", "new37")),
            escalation_level: Some("patched-37".to_string()),
            related_risk_ids: Some(vec![EntityId::new_serial("new37", "new37")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Conflict");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn requirement_patch_round_trips() {
        let mut item = Requirement {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("requirement", "Base Requirement"), "Base Requirement") },
            code: String::new(),
            kind: RequirementKind::Functional,
            statement: TextField::default(),
            rationale: Some(TextField::default()),
            source: Some(String::new()),
            stakeholder_ids: Vec::new(),
            element_ids: Vec::new(),
            function_ids: Vec::new(),
            parent_requirement_id: Some(EntityId::new_serial("base38", "base38")),
            child_requirement_ids: Vec::new(),
            acceptance_criteria: Vec::new(),
            verification_method: Some(String::new()),
            validation_status: ValidationStatus::Pending,
            conflict_ids: Vec::new(),
            risk_ids: Vec::new(),
            cost_estimate: Some(0.0),
            schedule_constraint: Some(String::new()),
            regulatory_refs: Vec::new(),
            trace_links: Vec::new(),
            superseded_by: Some(EntityId::new_serial("base38", "base38")),
        };
        let original = item.clone();
        let patch = RequirementPatch {
            name: Some("Patched Requirement".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-38".to_string()),
            kind: Some(RequirementKind::Spatial),
            statement: Some(TextField::plain("patched-38")),
            rationale: Some(TextField::plain("patched-38")),
            source: Some("patched-38".to_string()),
            stakeholder_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
            element_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
            function_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
            parent_requirement_id: Some(EntityId::new_serial("new38", "new38")),
            child_requirement_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
            acceptance_criteria: Some(vec!["patched-38".to_string()]),
            verification_method: Some("patched-38".to_string()),
            validation_status: Some(ValidationStatus::Passed),
            conflict_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
            risk_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
            cost_estimate: Some(42.0),
            schedule_constraint: Some("patched-38".to_string()),
            regulatory_refs: Some(vec!["patched-38".to_string()]),
            trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom38n", "tfrom38n"), EntityId::new_serial("tto38n", "tto38n"), TraceKind::FullAuditTrail)]),
            superseded_by: Some(EntityId::new_serial("new38", "new38")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Requirement");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn priority_record_patch_round_trips() {
        let mut item = PriorityRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("priorityrecord", "Base PriorityRecord"), "Base PriorityRecord") },
            subject_id: EntityId::new_serial("base39", "base39"),
            subject_kind: String::new(),
            ranked_priority: Priority::Mandatory,
            rank: Some(0),
            weight: Some(0.0),
            rationale: Some(TextField::default()),
            decision_id: Some(EntityId::new_serial("base39", "base39")),
            stakeholder_ids: Vec::new(),
            effective_from: Some(String::new()),
            effective_until: Some(String::new()),
            review_cycle: Some(String::new()),
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            scoring_method: Some(String::new()),
            score: Some(0.0),
            criteria: Vec::new(),
            approved_by: Some(EntityId::new_serial("base39", "base39")),
            approval_date: Some(String::new()),
            ranking_notes: Vec::new(),
        };
        let original = item.clone();
        let patch = PriorityRecordPatch {
            name: Some("Patched PriorityRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            subject_id: Some(EntityId::new_serial("new39", "new39")),
            subject_kind: Some("patched-39".to_string()),
            ranked_priority: Some(Priority::Essential),
            rank: Some(7),
            weight: Some(42.0),
            rationale: Some(TextField::plain("patched-39")),
            decision_id: Some(EntityId::new_serial("new39", "new39")),
            stakeholder_ids: Some(vec![EntityId::new_serial("new39", "new39")]),
            effective_from: Some("patched-39".to_string()),
            effective_until: Some("patched-39".to_string()),
            review_cycle: Some("patched-39".to_string()),
            dependencies: Some(vec![EntityId::new_serial("new39", "new39")]),
            conflicts: Some(vec![EntityId::new_serial("new39", "new39")]),
            scoring_method: Some("patched-39".to_string()),
            score: Some(42.0),
            criteria: Some(vec!["patched-39".to_string()]),
            approved_by: Some(EntityId::new_serial("new39", "new39")),
            approval_date: Some("patched-39".to_string()),
            ranking_notes: Some(vec![TaggedNote { tag: "new39".into(), text: "new-note39".into() }]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched PriorityRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn scenario_patch_round_trips() {
        let mut item = Scenario {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("scenario", "Base Scenario"), "Base Scenario") },
            code: String::new(),
            hypothesis: TextField::default(),
            assumptions: Vec::new(),
            variables: Vec::new(),
            element_ids: Vec::new(),
            requirement_ids: Vec::new(),
            growth_plan_id: Some(EntityId::new_serial("base40", "base40")),
            probability: Some(0.0),
            impact_summary: Some(TextField::default()),
            cost_delta: Some(0.0),
            area_delta: Some(0.0),
            headcount_delta: Some(0.0),
            schedule_delta: Some(String::new()),
            risk_ids: Vec::new(),
            option_ids: Vec::new(),
            baseline: false,
            preferred: false,
            analysis_ids: Vec::new(),
            owner_id: Some(EntityId::new_serial("base40", "base40")),
        };
        let original = item.clone();
        let patch = ScenarioPatch {
            name: Some("Patched Scenario".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            code: Some("patched-40".to_string()),
            hypothesis: Some(TextField::plain("patched-40")),
            assumptions: Some(vec!["patched-40".to_string()]),
            variables: Some(vec!["patched-40".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
            requirement_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
            growth_plan_id: Some(EntityId::new_serial("new40", "new40")),
            probability: Some(42.0),
            impact_summary: Some(TextField::plain("patched-40")),
            cost_delta: Some(42.0),
            area_delta: Some(42.0),
            headcount_delta: Some(42.0),
            schedule_delta: Some("patched-40".to_string()),
            risk_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
            option_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
            baseline: Some(true),
            preferred: Some(true),
            analysis_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
            owner_id: Some(EntityId::new_serial("new40", "new40")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Scenario");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn option_evaluation_patch_round_trips() {
        let mut item = OptionEvaluation {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("optionevaluation", "Base OptionEvaluation"), "Base OptionEvaluation") },
            option_name: String::new(),
            option_description: TextField::default(),
            scenario_id: Some(EntityId::new_serial("base41", "base41")),
            criteria_ids: Vec::new(),
            scores: Vec::new(),
            weighted_score: Some(0.0),
            cost_estimate: Some(0.0),
            schedule_estimate: Some(String::new()),
            risk_summary: Vec::new(),
            benefits: Vec::new(),
            drawbacks: Vec::new(),
            assumptions: Vec::new(),
            dependencies: Vec::new(),
            stakeholder_feedback: Vec::new(),
            recommendation: Some(String::new()),
            decision_id: Some(EntityId::new_serial("base41", "base41")),
            evaluation_status: ValidationStatus::Pending,
            evaluator_ids: Vec::new(),
            evaluation_date: Some(String::new()),
        };
        let original = item.clone();
        let patch = OptionEvaluationPatch {
            name: Some("Patched OptionEvaluation".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            option_name: Some("patched-41".to_string()),
            option_description: Some(TextField::plain("patched-41")),
            scenario_id: Some(EntityId::new_serial("new41", "new41")),
            criteria_ids: Some(vec![EntityId::new_serial("new41", "new41")]),
            scores: Some(vec![42.0]),
            weighted_score: Some(42.0),
            cost_estimate: Some(42.0),
            schedule_estimate: Some("patched-41".to_string()),
            risk_summary: Some(vec!["patched-41".to_string()]),
            benefits: Some(vec!["patched-41".to_string()]),
            drawbacks: Some(vec!["patched-41".to_string()]),
            assumptions: Some(vec!["patched-41".to_string()]),
            dependencies: Some(vec![EntityId::new_serial("new41", "new41")]),
            stakeholder_feedback: Some(vec![TaggedNote { tag: "new41".into(), text: "new-note41".into() }]),
            recommendation: Some("patched-41".to_string()),
            decision_id: Some(EntityId::new_serial("new41", "new41")),
            evaluation_status: Some(ValidationStatus::Passed),
            evaluator_ids: Some(vec![EntityId::new_serial("new41", "new41")]),
            evaluation_date: Some("patched-41".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched OptionEvaluation");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn decision_patch_round_trips() {
        let mut item = Decision {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("decision", "Base Decision"), "Base Decision") },
            decision_statement: TextField::default(),
            context: TextField::default(),
            options_considered: Vec::new(),
            selected_option_id: Some(EntityId::new_serial("base42", "base42")),
            rationale: TextField::default(),
            decision_maker_ids: Vec::new(),
            consulted_ids: Vec::new(),
            informed_ids: Vec::new(),
            decision_date: Some(String::new()),
            effective_date: Some(String::new()),
            reversal_conditions: Vec::new(),
            impacted_requirement_ids: Vec::new(),
            impacted_element_ids: Vec::new(),
            cost_impact: Some(0.0),
            schedule_impact: Some(String::new()),
            risk_impact: Vec::new(),
            approval_status: ValidationStatus::Pending,
            meeting_ref: Some(EntityId::new_serial("base42", "base42")),
            document_refs: Vec::new(),
        };
        let original = item.clone();
        let patch = DecisionPatch {
            name: Some("Patched Decision".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            decision_statement: Some(TextField::plain("patched-42")),
            context: Some(TextField::plain("patched-42")),
            options_considered: Some(vec![EntityId::new_serial("new42", "new42")]),
            selected_option_id: Some(EntityId::new_serial("new42", "new42")),
            rationale: Some(TextField::plain("patched-42")),
            decision_maker_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
            consulted_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
            informed_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
            decision_date: Some("patched-42".to_string()),
            effective_date: Some("patched-42".to_string()),
            reversal_conditions: Some(vec!["patched-42".to_string()]),
            impacted_requirement_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
            impacted_element_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
            cost_impact: Some(42.0),
            schedule_impact: Some("patched-42".to_string()),
            risk_impact: Some(vec!["patched-42".to_string()]),
            approval_status: Some(ValidationStatus::Passed),
            meeting_ref: Some(EntityId::new_serial("new42", "new42")),
            document_refs: Some(vec![EntityId::new_serial("new42", "new42")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Decision");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn validation_record_patch_round_trips() {
        let mut item = ValidationRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("validationrecord", "Base ValidationRecord"), "Base ValidationRecord") },
            subject_id: EntityId::new_serial("base43", "base43"),
            subject_kind: String::new(),
            validation_type: String::new(),
            method: Some(String::new()),
            criteria: Vec::new(),
            result: ValidationStatus::Pending,
            evidence: Vec::new(),
            validator_ids: Vec::new(),
            validation_date: Some(String::new()),
            next_review_date: Some(String::new()),
            findings: Vec::new(),
            non_conformities: Vec::new(),
            corrective_actions: Vec::new(),
            waivers: Vec::new(),
            standards: Vec::new(),
            trace_links: Vec::new(),
            report_id: Some(EntityId::new_serial("base43", "base43")),
            confidence_level: Some(String::new()),
            validation_notes: Vec::new(),
        };
        let original = item.clone();
        let patch = ValidationRecordPatch {
            name: Some("Patched ValidationRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            subject_id: Some(EntityId::new_serial("new43", "new43")),
            subject_kind: Some("patched-43".to_string()),
            validation_type: Some("patched-43".to_string()),
            method: Some("patched-43".to_string()),
            criteria: Some(vec!["patched-43".to_string()]),
            result: Some(ValidationStatus::Passed),
            evidence: Some(vec!["patched-43".to_string()]),
            validator_ids: Some(vec![EntityId::new_serial("new43", "new43")]),
            validation_date: Some("patched-43".to_string()),
            next_review_date: Some("patched-43".to_string()),
            findings: Some(vec!["patched-43".to_string()]),
            non_conformities: Some(vec!["patched-43".to_string()]),
            corrective_actions: Some(vec!["patched-43".to_string()]),
            waivers: Some(vec!["patched-43".to_string()]),
            standards: Some(vec!["patched-43".to_string()]),
            trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom43n", "tfrom43n"), EntityId::new_serial("tto43n", "tto43n"), TraceKind::FullAuditTrail)]),
            report_id: Some(EntityId::new_serial("new43", "new43")),
            confidence_level: Some("patched-43".to_string()),
            validation_notes: Some(vec![TaggedNote { tag: "new43".into(), text: "new-note43".into() }]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ValidationRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn performance_criterion_patch_round_trips() {
        let mut item = PerformanceCriterion {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("performancecriterion", "Base PerformanceCriterion"), "Base PerformanceCriterion") },
            criterion: String::new(),
            metric: String::new(),
            target: Some(0.0),
            unit: Some(String::new()),
            minimum: Some(0.0),
            maximum: Some(0.0),
            measurement_method: Some(String::new()),
            frequency: Some(String::new()),
            requirement_ids: Vec::new(),
            element_ids: Vec::new(),
            baseline: Some(0.0),
            benchmark_ref: Some(EntityId::new_serial("base44", "base44")),
            weight: Some(0.0),
            data_source: Some(String::new()),
            reporting_cadence: Some(String::new()),
            owner_id: Some(EntityId::new_serial("base44", "base44")),
            verification_plan: Some(String::new()),
            penalty_threshold: Some(0.0),
            incentive_threshold: Some(0.0),
        };
        let original = item.clone();
        let patch = PerformanceCriterionPatch {
            name: Some("Patched PerformanceCriterion".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            criterion: Some("patched-44".to_string()),
            metric: Some("patched-44".to_string()),
            target: Some(42.0),
            unit: Some("patched-44".to_string()),
            minimum: Some(42.0),
            maximum: Some(42.0),
            measurement_method: Some("patched-44".to_string()),
            frequency: Some("patched-44".to_string()),
            requirement_ids: Some(vec![EntityId::new_serial("new44", "new44")]),
            element_ids: Some(vec![EntityId::new_serial("new44", "new44")]),
            baseline: Some(42.0),
            benchmark_ref: Some(EntityId::new_serial("new44", "new44")),
            weight: Some(42.0),
            data_source: Some("patched-44".to_string()),
            reporting_cadence: Some("patched-44".to_string()),
            owner_id: Some(EntityId::new_serial("new44", "new44")),
            verification_plan: Some("patched-44".to_string()),
            penalty_threshold: Some(42.0),
            incentive_threshold: Some(42.0),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched PerformanceCriterion");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn quality_record_patch_round_trips() {
        let mut item = QualityRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("qualityrecord", "Base QualityRecord"), "Base QualityRecord") },
            quality_topic: String::new(),
            standard: Some(String::new()),
            target_level: Some(String::new()),
            inspection_points: Vec::new(),
            acceptance_criteria: Vec::new(),
            testing_requirements: Vec::new(),
            sample_rate: Some(String::new()),
            defect_categories: Vec::new(),
            corrective_action_process: Vec::new(),
            element_ids: Vec::new(),
            requirement_ids: Vec::new(),
            supplier_requirements: Vec::new(),
            documentation_requirements: Vec::new(),
            training_requirements: Vec::new(),
            audit_schedule: Some(String::new()),
            kpis: Vec::new(),
            owner_id: Some(EntityId::new_serial("base45", "base45")),
            certification_targets: Vec::new(),
            continuous_improvement: Vec::new(),
        };
        let original = item.clone();
        let patch = QualityRecordPatch {
            name: Some("Patched QualityRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            quality_topic: Some("patched-45".to_string()),
            standard: Some("patched-45".to_string()),
            target_level: Some("patched-45".to_string()),
            inspection_points: Some(vec!["patched-45".to_string()]),
            acceptance_criteria: Some(vec!["patched-45".to_string()]),
            testing_requirements: Some(vec!["patched-45".to_string()]),
            sample_rate: Some("patched-45".to_string()),
            defect_categories: Some(vec!["patched-45".to_string()]),
            corrective_action_process: Some(vec!["patched-45".to_string()]),
            element_ids: Some(vec![EntityId::new_serial("new45", "new45")]),
            requirement_ids: Some(vec![EntityId::new_serial("new45", "new45")]),
            supplier_requirements: Some(vec!["patched-45".to_string()]),
            documentation_requirements: Some(vec!["patched-45".to_string()]),
            training_requirements: Some(vec!["patched-45".to_string()]),
            audit_schedule: Some("patched-45".to_string()),
            kpis: Some(vec!["patched-45".to_string()]),
            owner_id: Some(EntityId::new_serial("new45", "new45")),
            certification_targets: Some(vec!["patched-45".to_string()]),
            continuous_improvement: Some(vec!["patched-45".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched QualityRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn document_record_patch_round_trips() {
        let mut item = DocumentRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("documentrecord", "Base DocumentRecord"), "Base DocumentRecord") },
            document_type: String::new(),
            title: String::new(),
            version: String::new(),
            file_ref: Some(String::new()),
            format: Some(String::new()),
            author_ids: Vec::new(),
            reviewer_ids: Vec::new(),
            approver_ids: Vec::new(),
            issue_date: Some(String::new()),
            revision_date: Some(String::new()),
            distribution_list: Vec::new(),
            related_entity_ids: Vec::new(),
            classification: Some(String::new()),
            retention_period: Some(String::new()),
            access_controls: Vec::new(),
            supersedes: Some(EntityId::new_serial("base46", "base46")),
            document_status: LifecycleStatus::Draft,
            checksum: Some(String::new()),
            source_system: Some(String::new()),
        };
        let original = item.clone();
        let patch = DocumentRecordPatch {
            name: Some("Patched DocumentRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            document_type: Some("patched-46".to_string()),
            title: Some("patched-46".to_string()),
            version: Some("patched-46".to_string()),
            file_ref: Some("patched-46".to_string()),
            format: Some("patched-46".to_string()),
            author_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
            reviewer_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
            approver_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
            issue_date: Some("patched-46".to_string()),
            revision_date: Some("patched-46".to_string()),
            distribution_list: Some(vec![EntityId::new_serial("new46", "new46")]),
            related_entity_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
            classification: Some("patched-46".to_string()),
            retention_period: Some("patched-46".to_string()),
            access_controls: Some(vec!["patched-46".to_string()]),
            supersedes: Some(EntityId::new_serial("new46", "new46")),
            document_status: Some(LifecycleStatus::Proposed),
            checksum: Some("patched-46".to_string()),
            source_system: Some("patched-46".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched DocumentRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn change_record_patch_round_trips() {
        let mut item = ChangeRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("changerecord", "Base ChangeRecord"), "Base ChangeRecord") },
            change_type: String::new(),
            summary: TextField::default(),
            reason: TextField::default(),
            requested_by: Some(EntityId::new_serial("base47", "base47")),
            approved_by: Some(EntityId::new_serial("base47", "base47")),
            change_date: Some(String::new()),
            effective_date: Some(String::new()),
            impacted_entity_ids: Vec::new(),
            before_snapshot: Some(String::new()),
            after_snapshot: Some(String::new()),
            cost_impact: Some(0.0),
            schedule_impact: Some(String::new()),
            risk_impact: Vec::new(),
            approval_status: ValidationStatus::Pending,
            rollback_plan: Vec::new(),
            communication_plan: Vec::new(),
            version_from: Some(String::new()),
            version_to: Some(String::new()),
            audit_event_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = ChangeRecordPatch {
            name: Some("Patched ChangeRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            change_type: Some("patched-47".to_string()),
            summary: Some(TextField::plain("patched-47")),
            reason: Some(TextField::plain("patched-47")),
            requested_by: Some(EntityId::new_serial("new47", "new47")),
            approved_by: Some(EntityId::new_serial("new47", "new47")),
            change_date: Some("patched-47".to_string()),
            effective_date: Some("patched-47".to_string()),
            impacted_entity_ids: Some(vec![EntityId::new_serial("new47", "new47")]),
            before_snapshot: Some("patched-47".to_string()),
            after_snapshot: Some("patched-47".to_string()),
            cost_impact: Some(42.0),
            schedule_impact: Some("patched-47".to_string()),
            risk_impact: Some(vec!["patched-47".to_string()]),
            approval_status: Some(ValidationStatus::Passed),
            rollback_plan: Some(vec!["patched-47".to_string()]),
            communication_plan: Some(vec!["patched-47".to_string()]),
            version_from: Some("patched-47".to_string()),
            version_to: Some("patched-47".to_string()),
            audit_event_ids: Some(vec![EntityId::new_serial("new47", "new47")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ChangeRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn collaboration_record_patch_round_trips() {
        let mut item = CollaborationRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("collaborationrecord", "Base CollaborationRecord"), "Base CollaborationRecord") },
            session_type: String::new(),
            title: String::new(),
            participants: Vec::new(),
            facilitator_id: Some(EntityId::new_serial("base48", "base48")),
            start_time: Some(String::new()),
            end_time: Some(String::new()),
            location: Some(String::new()),
            agenda: Vec::new(),
            outcomes: Vec::new(),
            action_items: Vec::new(),
            decision_ids: Vec::new(),
            issue_ids: Vec::new(),
            document_ids: Vec::new(),
            recording_ref: Some(String::new()),
            feedback: Vec::new(),
            follow_up_date: Some(String::new()),
            workshop_id: Some(EntityId::new_serial("base48", "base48")),
            survey_id: Some(EntityId::new_serial("base48", "base48")),
        };
        let original = item.clone();
        let patch = CollaborationRecordPatch {
            name: Some("Patched CollaborationRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            session_type: Some("patched-48".to_string()),
            title: Some("patched-48".to_string()),
            participants: Some(vec![EntityId::new_serial("new48", "new48")]),
            facilitator_id: Some(EntityId::new_serial("new48", "new48")),
            start_time: Some("patched-48".to_string()),
            end_time: Some("patched-48".to_string()),
            location: Some("patched-48".to_string()),
            agenda: Some(vec!["patched-48".to_string()]),
            outcomes: Some(vec!["patched-48".to_string()]),
            action_items: Some(vec!["patched-48".to_string()]),
            decision_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
            issue_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
            document_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
            recording_ref: Some("patched-48".to_string()),
            feedback: Some(vec![TaggedNote { tag: "new48".into(), text: "new-note48".into() }]),
            follow_up_date: Some("patched-48".to_string()),
            workshop_id: Some(EntityId::new_serial("new48", "new48")),
            survey_id: Some(EntityId::new_serial("new48", "new48")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched CollaborationRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn analysis_record_patch_round_trips() {
        let mut item = AnalysisRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("analysisrecord", "Base AnalysisRecord"), "Base AnalysisRecord") },
            kind: AnalysisKind::Gap,
            title: String::new(),
            parameters: Vec::new(),
            input_entity_ids: Vec::new(),
            output_summary: TextField::default(),
            findings: Vec::new(),
            metrics: Vec::new(),
            charts: Vec::new(),
            run_by: Some(EntityId::new_serial("base49", "base49")),
            run_at: Some(String::new()),
            duration_ms: Some(0),
            tool_version: Some(String::new()),
            scenario_id: Some(EntityId::new_serial("base49", "base49")),
            report_id: Some(EntityId::new_serial("base49", "base49")),
            confidence: Some(String::new()),
            limitations: Vec::new(),
            recommendations: Vec::new(),
            raw_result_ref: Some(String::new()),
        };
        let original = item.clone();
        let patch = AnalysisRecordPatch {
            name: Some("Patched AnalysisRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            kind: Some(AnalysisKind::Conflict),
            title: Some("patched-49".to_string()),
            parameters: Some(vec!["patched-49".to_string()]),
            input_entity_ids: Some(vec![EntityId::new_serial("new49", "new49")]),
            output_summary: Some(TextField::plain("patched-49")),
            findings: Some(vec!["patched-49".to_string()]),
            metrics: Some(vec!["patched-49".to_string()]),
            charts: Some(vec!["patched-49".to_string()]),
            run_by: Some(EntityId::new_serial("new49", "new49")),
            run_at: Some("patched-49".to_string()),
            duration_ms: Some(7),
            tool_version: Some("patched-49".to_string()),
            scenario_id: Some(EntityId::new_serial("new49", "new49")),
            report_id: Some(EntityId::new_serial("new49", "new49")),
            confidence: Some("patched-49".to_string()),
            limitations: Some(vec!["patched-49".to_string()]),
            recommendations: Some(vec!["patched-49".to_string()]),
            raw_result_ref: Some("patched-49".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched AnalysisRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn report_record_patch_round_trips() {
        let mut item = ReportRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("reportrecord", "Base ReportRecord"), "Base ReportRecord") },
            kind: ReportKind::ExecutiveSummary,
            title: String::new(),
            audience: Vec::new(),
            sections: Vec::new(),
            generated_at: Some(String::new()),
            generated_by: Some(EntityId::new_serial("base50", "base50")),
            analysis_ids: Vec::new(),
            format: Some(String::new()),
            file_ref: Some(String::new()),
            distribution_list: Vec::new(),
            approval_status: ValidationStatus::Pending,
            approver_id: Some(EntityId::new_serial("base50", "base50")),
            version: String::new(),
            template_id: Some(EntityId::new_serial("base50", "base50")),
            parameters: Vec::new(),
            confidentiality: Some(String::new()),
            expiry_date: Some(String::new()),
            related_decision_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = ReportRecordPatch {
            name: Some("Patched ReportRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            kind: Some(ReportKind::ProgramOverview),
            title: Some("patched-50".to_string()),
            audience: Some(vec!["patched-50".to_string()]),
            sections: Some(vec!["patched-50".to_string()]),
            generated_at: Some("patched-50".to_string()),
            generated_by: Some(EntityId::new_serial("new50", "new50")),
            analysis_ids: Some(vec![EntityId::new_serial("new50", "new50")]),
            format: Some("patched-50".to_string()),
            file_ref: Some("patched-50".to_string()),
            distribution_list: Some(vec![EntityId::new_serial("new50", "new50")]),
            approval_status: Some(ValidationStatus::Passed),
            approver_id: Some(EntityId::new_serial("new50", "new50")),
            version: Some("patched-50".to_string()),
            template_id: Some(EntityId::new_serial("new50", "new50")),
            parameters: Some(vec!["patched-50".to_string()]),
            confidentiality: Some("patched-50".to_string()),
            expiry_date: Some("patched-50".to_string()),
            related_decision_ids: Some(vec![EntityId::new_serial("new50", "new50")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ReportRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn search_filter_patch_round_trips() {
        let mut item = SearchFilter {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("searchfilter", "Base SearchFilter"), "Base SearchFilter") },
            filter_name: String::new(),
            filter_description: Some(TextField::default()),
            keywords: Vec::new(),
            categories: Vec::new(),
            owner_ids: Vec::new(),
            statuses: Vec::new(),
            priorities: Vec::new(),
            sources: Vec::new(),
            date_from: Some(String::new()),
            date_to: Some(String::new()),
            entity_kinds: Vec::new(),
            tag_filters: Vec::new(),
            sort_field: Some(String::new()),
            sort_direction: Some(String::new()),
            is_public: false,
            created_by: Some(EntityId::new_serial("base51", "base51")),
            last_used: Some(String::new()),
            use_count: 0,
            pinned: false,
        };
        let original = item.clone();
        let patch = SearchFilterPatch {
            name: Some("Patched SearchFilter".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            filter_name: Some("patched-51".to_string()),
            filter_description: Some(TextField::plain("patched-51")),
            keywords: Some(vec!["patched-51".to_string()]),
            categories: Some(vec!["patched-51".to_string()]),
            owner_ids: Some(vec![EntityId::new_serial("new51", "new51")]),
            statuses: Some(vec![LifecycleStatus::Proposed]),
            priorities: Some(vec![Priority::Essential]),
            sources: Some(vec!["patched-51".to_string()]),
            date_from: Some("patched-51".to_string()),
            date_to: Some("patched-51".to_string()),
            entity_kinds: Some(vec!["patched-51".to_string()]),
            tag_filters: Some(vec!["patched-51".to_string()]),
            sort_field: Some("patched-51".to_string()),
            sort_direction: Some("patched-51".to_string()),
            is_public: Some(true),
            created_by: Some(EntityId::new_serial("new51", "new51")),
            last_used: Some("patched-51".to_string()),
            use_count: Some(7),
            pinned: Some(true),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched SearchFilter");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn status_record_patch_round_trips() {
        let mut item = StatusRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("statusrecord", "Base StatusRecord"), "Base StatusRecord") },
            subject_id: EntityId::new_serial("base52", "base52"),
            subject_kind: String::new(),
            record_status: LifecycleStatus::Draft,
            previous_status: Some(LifecycleStatus::Draft),
            changed_by: Some(EntityId::new_serial("base52", "base52")),
            changed_at: Some(String::new()),
            reason: Some(TextField::default()),
            blockers: Vec::new(),
            next_actions: Vec::new(),
            due_date: Some(String::new()),
            progress_percent: Some(0.0),
            health: Some(String::new()),
            escalation_level: Some(String::new()),
            related_issue_ids: Vec::new(),
            related_risk_ids: Vec::new(),
            milestone_id: Some(EntityId::new_serial("base52", "base52")),
            reporting_period: Some(String::new()),
            status_notes: Vec::new(),
        };
        let original = item.clone();
        let patch = StatusRecordPatch {
            name: Some("Patched StatusRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            subject_id: Some(EntityId::new_serial("new52", "new52")),
            subject_kind: Some("patched-52".to_string()),
            record_status: Some(LifecycleStatus::Proposed),
            previous_status: Some(LifecycleStatus::Proposed),
            changed_by: Some(EntityId::new_serial("new52", "new52")),
            changed_at: Some("patched-52".to_string()),
            reason: Some(TextField::plain("patched-52")),
            blockers: Some(vec!["patched-52".to_string()]),
            next_actions: Some(vec!["patched-52".to_string()]),
            due_date: Some("patched-52".to_string()),
            progress_percent: Some(42.0),
            health: Some("patched-52".to_string()),
            escalation_level: Some("patched-52".to_string()),
            related_issue_ids: Some(vec![EntityId::new_serial("new52", "new52")]),
            related_risk_ids: Some(vec![EntityId::new_serial("new52", "new52")]),
            milestone_id: Some(EntityId::new_serial("new52", "new52")),
            reporting_period: Some("patched-52".to_string()),
            status_notes: Some(vec![TaggedNote { tag: "new52".into(), text: "new-note52".into() }]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched StatusRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn workshop_patch_round_trips() {
        let mut item = Workshop {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("workshop", "Base Workshop"), "Base Workshop") },
            workshop_type: String::new(),
            objectives: Vec::new(),
            agenda: Vec::new(),
            facilitator_id: Some(EntityId::new_serial("base53", "base53")),
            participants: Vec::new(),
            scheduled_start: Some(String::new()),
            scheduled_end: Some(String::new()),
            location: Some(String::new()),
            materials: Vec::new(),
            methods: Vec::new(),
            outputs: Vec::new(),
            decisions: Vec::new(),
            issues: Vec::new(),
            follow_up_actions: Vec::new(),
            feedback: Vec::new(),
            recording_ref: Some(String::new()),
            budget: Some(0.0),
            workshop_status: LifecycleStatus::Draft,
            survey_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = WorkshopPatch {
            name: Some("Patched Workshop".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            workshop_type: Some("patched-53".to_string()),
            objectives: Some(vec!["patched-53".to_string()]),
            agenda: Some(vec!["patched-53".to_string()]),
            facilitator_id: Some(EntityId::new_serial("new53", "new53")),
            participants: Some(vec![EntityId::new_serial("new53", "new53")]),
            scheduled_start: Some("patched-53".to_string()),
            scheduled_end: Some("patched-53".to_string()),
            location: Some("patched-53".to_string()),
            materials: Some(vec!["patched-53".to_string()]),
            methods: Some(vec!["patched-53".to_string()]),
            outputs: Some(vec!["patched-53".to_string()]),
            decisions: Some(vec![EntityId::new_serial("new53", "new53")]),
            issues: Some(vec![EntityId::new_serial("new53", "new53")]),
            follow_up_actions: Some(vec!["patched-53".to_string()]),
            feedback: Some(vec![TaggedNote { tag: "new53".into(), text: "new-note53".into() }]),
            recording_ref: Some("patched-53".to_string()),
            budget: Some(42.0),
            workshop_status: Some(LifecycleStatus::Proposed),
            survey_ids: Some(vec![EntityId::new_serial("new53", "new53")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Workshop");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn survey_patch_round_trips() {
        let mut item = Survey {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("survey", "Base Survey"), "Base Survey") },
            survey_type: String::new(),
            title: String::new(),
            objectives: Vec::new(),
            questions: Vec::new(),
            target_audience: Vec::new(),
            distribution_channels: Vec::new(),
            launch_date: Some(String::new()),
            close_date: Some(String::new()),
            response_count: 0,
            response_rate: Some(0.0),
            findings: Vec::new(),
            themes: Vec::new(),
            recommendations: Vec::new(),
            confidentiality: Some(String::new()),
            consent_process: Vec::new(),
            analysis_id: Some(EntityId::new_serial("base54", "base54")),
            workshop_id: Some(EntityId::new_serial("base54", "base54")),
            owner_id: Some(EntityId::new_serial("base54", "base54")),
            survey_status: LifecycleStatus::Draft,
        };
        let original = item.clone();
        let patch = SurveyPatch {
            name: Some("Patched Survey".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            survey_type: Some("patched-54".to_string()),
            title: Some("patched-54".to_string()),
            objectives: Some(vec!["patched-54".to_string()]),
            questions: Some(vec!["patched-54".to_string()]),
            target_audience: Some(vec![EntityId::new_serial("new54", "new54")]),
            distribution_channels: Some(vec!["patched-54".to_string()]),
            launch_date: Some("patched-54".to_string()),
            close_date: Some("patched-54".to_string()),
            response_count: Some(7),
            response_rate: Some(42.0),
            findings: Some(vec!["patched-54".to_string()]),
            themes: Some(vec!["patched-54".to_string()]),
            recommendations: Some(vec!["patched-54".to_string()]),
            confidentiality: Some("patched-54".to_string()),
            consent_process: Some(vec!["patched-54".to_string()]),
            analysis_id: Some(EntityId::new_serial("new54", "new54")),
            workshop_id: Some(EntityId::new_serial("new54", "new54")),
            owner_id: Some(EntityId::new_serial("new54", "new54")),
            survey_status: Some(LifecycleStatus::Proposed),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Survey");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn issue_patch_round_trips() {
        let mut item = Issue {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("issue", "Base Issue"), "Base Issue") },
            issue_type: String::new(),
            summary: TextField::default(),
            issue_description: TextField::default(),
            severity: IssueSeverity::Cosmetic,
            issue_priority: Priority::Mandatory,
            reporter_id: Some(EntityId::new_serial("base55", "base55")),
            assignee_id: Some(EntityId::new_serial("base55", "base55")),
            affected_entity_ids: Vec::new(),
            root_cause: Some(TextField::default()),
            resolution: Some(TextField::default()),
            workaround: Some(TextField::default()),
            due_date: Some(String::new()),
            resolved_date: Some(String::new()),
            related_conflict_ids: Vec::new(),
            related_risk_ids: Vec::new(),
            decision_id: Some(EntityId::new_serial("base55", "base55")),
            comments: Vec::new(),
            attachments: Vec::new(),
            escalation_level: Some(String::new()),
        };
        let original = item.clone();
        let patch = IssuePatch {
            name: Some("Patched Issue".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            issue_type: Some("patched-55".to_string()),
            summary: Some(TextField::plain("patched-55")),
            issue_description: Some(TextField::plain("patched-55")),
            severity: Some(IssueSeverity::Minor),
            issue_priority: Some(Priority::Essential),
            reporter_id: Some(EntityId::new_serial("new55", "new55")),
            assignee_id: Some(EntityId::new_serial("new55", "new55")),
            affected_entity_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
            root_cause: Some(TextField::plain("patched-55")),
            resolution: Some(TextField::plain("patched-55")),
            workaround: Some(TextField::plain("patched-55")),
            due_date: Some("patched-55".to_string()),
            resolved_date: Some("patched-55".to_string()),
            related_conflict_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
            related_risk_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
            decision_id: Some(EntityId::new_serial("new55", "new55")),
            comments: Some(vec![TaggedNote { tag: "new55".into(), text: "new-note55".into() }]),
            attachments: Some(vec![EntityId::new_serial("new55", "new55")]),
            escalation_level: Some("patched-55".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Issue");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn audit_event_patch_round_trips() {
        let mut item = AuditEvent {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("auditevent", "Base AuditEvent"), "Base AuditEvent") },
            action: AuditAction::Created,
            actor_id: Some(EntityId::new_serial("base56", "base56")),
            subject_id: EntityId::new_serial("base56", "base56"),
            subject_kind: String::new(),
            timestamp: String::new(),
            details: TextField::default(),
            before_state: Some(String::new()),
            after_state: Some(String::new()),
            ip_address: Some(String::new()),
            client: Some(String::new()),
            session_id: Some(String::new()),
            change_record_id: Some(EntityId::new_serial("base56", "base56")),
            trace_link: Some(TraceLink::new(EntityId::new_serial("tfrom56", "tfrom56"), EntityId::new_serial("tto56", "tto56"), TraceKind::FullAuditTrail)),
            success: false,
            error_message: Some(String::new()),
            correlation_id: Some(String::new()),
            compliance_tags: Vec::new(),
            retention_until: Some(String::new()),
        };
        let original = item.clone();
        let patch = AuditEventPatch {
            name: Some("Patched AuditEvent".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            action: Some(AuditAction::Updated),
            actor_id: Some(EntityId::new_serial("new56", "new56")),
            subject_id: Some(EntityId::new_serial("new56", "new56")),
            subject_kind: Some("patched-56".to_string()),
            timestamp: Some("patched-56".to_string()),
            details: Some(TextField::plain("patched-56")),
            before_state: Some("patched-56".to_string()),
            after_state: Some("patched-56".to_string()),
            ip_address: Some("patched-56".to_string()),
            client: Some("patched-56".to_string()),
            session_id: Some("patched-56".to_string()),
            change_record_id: Some(EntityId::new_serial("new56", "new56")),
            trace_link: Some(TraceLink::new(EntityId::new_serial("tfrom56n", "tfrom56n"), EntityId::new_serial("tto56n", "tto56n"), TraceKind::FullAuditTrail)),
            success: Some(true),
            error_message: Some("patched-56".to_string()),
            correlation_id: Some("patched-56".to_string()),
            compliance_tags: Some(vec!["patched-56".to_string()]),
            retention_until: Some("patched-56".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched AuditEvent");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn template_record_patch_round_trips() {
        let mut item = TemplateRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("templaterecord", "Base TemplateRecord"), "Base TemplateRecord") },
            template_type: String::new(),
            sector: Some(String::new()),
            project_type: Some(String::new()),
            version: String::new(),
            content_ref: Some(String::new()),
            entity_kinds: Vec::new(),
            default_fields: Vec::new(),
            checklists: Vec::new(),
            standards: Vec::new(),
            applicability: Vec::new(),
            author_id: Some(EntityId::new_serial("base57", "base57")),
            approval_status: ValidationStatus::Pending,
            usage_count: 0,
            last_applied: Some(String::new()),
            customization_notes: Vec::new(),
            related_knowledge_ids: Vec::new(),
            benchmark_ids: Vec::new(),
            license: Some(String::new()),
            source_organization: Some(String::new()),
        };
        let original = item.clone();
        let patch = TemplateRecordPatch {
            name: Some("Patched TemplateRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            template_type: Some("patched-57".to_string()),
            sector: Some("patched-57".to_string()),
            project_type: Some("patched-57".to_string()),
            version: Some("patched-57".to_string()),
            content_ref: Some("patched-57".to_string()),
            entity_kinds: Some(vec!["patched-57".to_string()]),
            default_fields: Some(vec!["patched-57".to_string()]),
            checklists: Some(vec!["patched-57".to_string()]),
            standards: Some(vec!["patched-57".to_string()]),
            applicability: Some(vec!["patched-57".to_string()]),
            author_id: Some(EntityId::new_serial("new57", "new57")),
            approval_status: Some(ValidationStatus::Passed),
            usage_count: Some(7),
            last_applied: Some("patched-57".to_string()),
            customization_notes: Some(vec!["patched-57".to_string()]),
            related_knowledge_ids: Some(vec![EntityId::new_serial("new57", "new57")]),
            benchmark_ids: Some(vec![EntityId::new_serial("new57", "new57")]),
            license: Some("patched-57".to_string()),
            source_organization: Some("patched-57".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched TemplateRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn knowledge_record_patch_round_trips() {
        let mut item = KnowledgeRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("knowledgerecord", "Base KnowledgeRecord"), "Base KnowledgeRecord") },
            topic: String::new(),
            category: String::new(),
            summary: TextField::default(),
            content: TextField::default(),
            sources: Vec::new(),
            references: Vec::new(),
            lessons_learned: Vec::new(),
            best_practices: Vec::new(),
            applicable_sectors: Vec::new(),
            related_entity_kinds: Vec::new(),
            author_ids: Vec::new(),
            expertise_level: Some(String::new()),
            validation_status: ValidationStatus::Pending,
            last_reviewed: Some(String::new()),
            keywords: Vec::new(),
            attachments: Vec::new(),
            citations: Vec::new(),
            usage_count: 0,
        };
        let original = item.clone();
        let patch = KnowledgeRecordPatch {
            name: Some("Patched KnowledgeRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            topic: Some("patched-58".to_string()),
            category: Some("patched-58".to_string()),
            summary: Some(TextField::plain("patched-58")),
            content: Some(TextField::plain("patched-58")),
            sources: Some(vec!["patched-58".to_string()]),
            references: Some(vec!["patched-58".to_string()]),
            lessons_learned: Some(vec!["patched-58".to_string()]),
            best_practices: Some(vec!["patched-58".to_string()]),
            applicable_sectors: Some(vec!["patched-58".to_string()]),
            related_entity_kinds: Some(vec!["patched-58".to_string()]),
            author_ids: Some(vec![EntityId::new_serial("new58", "new58")]),
            expertise_level: Some("patched-58".to_string()),
            validation_status: Some(ValidationStatus::Passed),
            last_reviewed: Some("patched-58".to_string()),
            keywords: Some(vec!["patched-58".to_string()]),
            attachments: Some(vec![EntityId::new_serial("new58", "new58")]),
            citations: Some(vec!["patched-58".to_string()]),
            usage_count: Some(7),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched KnowledgeRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn benchmark_record_patch_round_trips() {
        let mut item = BenchmarkRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("benchmarkrecord", "Base BenchmarkRecord"), "Base BenchmarkRecord") },
            benchmark_name: String::new(),
            sector: String::new(),
            metric: String::new(),
            value: 0.0,
            unit: String::new(),
            sample_size: Some(0),
            source: Some(String::new()),
            collection_year: Some(0),
            geography: Some(String::new()),
            building_type: Some(String::new()),
            confidence: Some(String::new()),
            methodology: Some(String::new()),
            applicable_element_kinds: Vec::new(),
            related_requirement_ids: Vec::new(),
            comparison_notes: Vec::new(),
            limitations: Vec::new(),
            license: Some(String::new()),
            knowledge_id: Some(EntityId::new_serial("base59", "base59")),
            last_verified: Some(String::new()),
        };
        let original = item.clone();
        let patch = BenchmarkRecordPatch {
            name: Some("Patched BenchmarkRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            benchmark_name: Some("patched-59".to_string()),
            sector: Some("patched-59".to_string()),
            metric: Some("patched-59".to_string()),
            value: Some(42.0),
            unit: Some("patched-59".to_string()),
            sample_size: Some(7),
            source: Some("patched-59".to_string()),
            collection_year: Some(7),
            geography: Some("patched-59".to_string()),
            building_type: Some("patched-59".to_string()),
            confidence: Some("patched-59".to_string()),
            methodology: Some("patched-59".to_string()),
            applicable_element_kinds: Some(vec!["patched-59".to_string()]),
            related_requirement_ids: Some(vec![EntityId::new_serial("new59", "new59")]),
            comparison_notes: Some(vec!["patched-59".to_string()]),
            limitations: Some(vec!["patched-59".to_string()]),
            license: Some("patched-59".to_string()),
            knowledge_id: Some(EntityId::new_serial("new59", "new59")),
            last_verified: Some("patched-59".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched BenchmarkRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn assumption_patch_round_trips() {
        let mut item = Assumption {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("assumption", "Base Assumption"), "Base Assumption") },
            statement: TextField::default(),
            basis: Some(TextField::default()),
            confidence_level: Some(String::new()),
            impact_if_false: Some(TextField::default()),
            related_entity_ids: Vec::new(),
            validation_status: ValidationStatus::Pending,
            validated_by: Some(EntityId::new_serial("base60", "base60")),
            validation_date: Some(String::new()),
            owner_id: Some(EntityId::new_serial("base60", "base60")),
            review_cycle: Some(String::new()),
            source: Some(String::new()),
            category: Some(String::new()),
            dependencies: Vec::new(),
            mitigation: Vec::new(),
            linked_requirement_ids: Vec::new(),
            linked_risk_ids: Vec::new(),
            expiration_date: Some(String::new()),
            status_notes: Vec::new(),
            document_refs: Vec::new(),
        };
        let original = item.clone();
        let patch = AssumptionPatch {
            name: Some("Patched Assumption".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            statement: Some(TextField::plain("patched-60")),
            basis: Some(TextField::plain("patched-60")),
            confidence_level: Some("patched-60".to_string()),
            impact_if_false: Some(TextField::plain("patched-60")),
            related_entity_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
            validation_status: Some(ValidationStatus::Passed),
            validated_by: Some(EntityId::new_serial("new60", "new60")),
            validation_date: Some("patched-60".to_string()),
            owner_id: Some(EntityId::new_serial("new60", "new60")),
            review_cycle: Some("patched-60".to_string()),
            source: Some("patched-60".to_string()),
            category: Some("patched-60".to_string()),
            dependencies: Some(vec!["patched-60".to_string()]),
            mitigation: Some(vec!["patched-60".to_string()]),
            linked_requirement_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
            linked_risk_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
            expiration_date: Some("patched-60".to_string()),
            status_notes: Some(vec![TaggedNote { tag: "new60".into(), text: "new-note60".into() }]),
            document_refs: Some(vec!["patched-60".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched Assumption");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn constraint_record_patch_round_trips() {
        let mut item = ConstraintRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("constraintrecord", "Base ConstraintRecord"), "Base ConstraintRecord") },
            constraint_type: String::new(),
            summary: TextField::default(),
            severity: RiskLevel::Negligible,
            affected_entity_ids: Vec::new(),
            source: Some(String::new()),
            regulatory_basis: Vec::new(),
            mitigation_options: Vec::new(),
            owner_id: Some(EntityId::new_serial("base61", "base61")),
            effective_date: Some(String::new()),
            expiry_date: Some(String::new()),
            waiver_status: Some(String::new()),
            waiver_approver: Some(EntityId::new_serial("base61", "base61")),
            impact_assessment: Some(TextField::default()),
            resolution_plan: Vec::new(),
            related_requirement_ids: Vec::new(),
            related_decision_ids: Vec::new(),
            monitoring_frequency: Some(String::new()),
            compliance_status: ValidationStatus::Pending,
            exceptions: Vec::new(),
            trace_links: Vec::new(),
            escalation_contact_id: Some(EntityId::new_serial("base61", "base61")),
        };
        let original = item.clone();
        let patch = ConstraintRecordPatch {
            name: Some("Patched ConstraintRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            constraint_type: Some("patched-61".to_string()),
            summary: Some(TextField::plain("patched-61")),
            severity: Some(RiskLevel::Low),
            affected_entity_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
            source: Some("patched-61".to_string()),
            regulatory_basis: Some(vec!["patched-61".to_string()]),
            mitigation_options: Some(vec!["patched-61".to_string()]),
            owner_id: Some(EntityId::new_serial("new61", "new61")),
            effective_date: Some("patched-61".to_string()),
            expiry_date: Some("patched-61".to_string()),
            waiver_status: Some("patched-61".to_string()),
            waiver_approver: Some(EntityId::new_serial("new61", "new61")),
            impact_assessment: Some(TextField::plain("patched-61")),
            resolution_plan: Some(vec!["patched-61".to_string()]),
            related_requirement_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
            related_decision_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
            monitoring_frequency: Some("patched-61".to_string()),
            compliance_status: Some(ValidationStatus::Passed),
            exceptions: Some(vec!["patched-61".to_string()]),
            trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom61n", "tfrom61n"), EntityId::new_serial("tto61n", "tto61n"), TraceKind::FullAuditTrail)]),
            escalation_contact_id: Some(EntityId::new_serial("new61", "new61")),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ConstraintRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn compliance_record_patch_round_trips() {
        let mut item = ComplianceRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("compliancerecord", "Base ComplianceRecord"), "Base ComplianceRecord") },
            standard_ref: String::new(),
            obligation: TextField::default(),
            compliance_status: ValidationStatus::Pending,
            evidence_refs: Vec::new(),
            auditor_id: Some(EntityId::new_serial("base62", "base62")),
            audit_date: Some(String::new()),
            next_review: Some(String::new()),
            affected_entity_ids: Vec::new(),
            gap_analysis: Vec::new(),
            remediation_plan: Vec::new(),
            owner_id: Some(EntityId::new_serial("base62", "base62")),
            severity: RiskLevel::Negligible,
            regulatory_body: Some(String::new()),
            certification_target: Some(String::new()),
            waiver_status: Some(String::new()),
            related_requirement_ids: Vec::new(),
            monitoring_method: Some(String::new()),
            reporting_frequency: Some(String::new()),
            penalties: Vec::new(),
            corrective_actions: Vec::new(),
            document_refs: Vec::new(),
        };
        let original = item.clone();
        let patch = ComplianceRecordPatch {
            name: Some("Patched ComplianceRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            standard_ref: Some("patched-62".to_string()),
            obligation: Some(TextField::plain("patched-62")),
            compliance_status: Some(ValidationStatus::Passed),
            evidence_refs: Some(vec!["patched-62".to_string()]),
            auditor_id: Some(EntityId::new_serial("new62", "new62")),
            audit_date: Some("patched-62".to_string()),
            next_review: Some("patched-62".to_string()),
            affected_entity_ids: Some(vec![EntityId::new_serial("new62", "new62")]),
            gap_analysis: Some(vec!["patched-62".to_string()]),
            remediation_plan: Some(vec!["patched-62".to_string()]),
            owner_id: Some(EntityId::new_serial("new62", "new62")),
            severity: Some(RiskLevel::Low),
            regulatory_body: Some("patched-62".to_string()),
            certification_target: Some("patched-62".to_string()),
            waiver_status: Some("patched-62".to_string()),
            related_requirement_ids: Some(vec![EntityId::new_serial("new62", "new62")]),
            monitoring_method: Some("patched-62".to_string()),
            reporting_frequency: Some("patched-62".to_string()),
            penalties: Some(vec!["patched-62".to_string()]),
            corrective_actions: Some(vec!["patched-62".to_string()]),
            document_refs: Some(vec!["patched-62".to_string()]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ComplianceRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn approval_record_patch_round_trips() {
        let mut item = ApprovalRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("approvalrecord", "Base ApprovalRecord"), "Base ApprovalRecord") },
            approval_type: String::new(),
            subject_id: EntityId::new_serial("base63", "base63"),
            approver_ids: Vec::new(),
            approval_date: Some(String::new()),
            conditions: Vec::new(),
            approval_status: LifecycleStatus::Draft,
            expiry_date: Some(String::new()),
            delegation_chain: Vec::new(),
            evidence_refs: Vec::new(),
            related_decision_id: Some(EntityId::new_serial("base63", "base63")),
            related_change_id: Some(EntityId::new_serial("base63", "base63")),
            authority_basis: Vec::new(),
            signature_method: Some(String::new()),
            rejection_reason: Some(TextField::default()),
            resubmission_date: Some(String::new()),
            notification_list: Vec::new(),
            workflow_step: Some(String::new()),
            version: Some(String::new()),
            audit_trail_ref: Some(String::new()),
        };
        let original = item.clone();
        let patch = ApprovalRecordPatch {
            name: Some("Patched ApprovalRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            approval_type: Some("patched-63".to_string()),
            subject_id: Some(EntityId::new_serial("new63", "new63")),
            approver_ids: Some(vec![EntityId::new_serial("new63", "new63")]),
            approval_date: Some("patched-63".to_string()),
            conditions: Some(vec!["patched-63".to_string()]),
            approval_status: Some(LifecycleStatus::Proposed),
            expiry_date: Some("patched-63".to_string()),
            delegation_chain: Some(vec![EntityId::new_serial("new63", "new63")]),
            evidence_refs: Some(vec!["patched-63".to_string()]),
            related_decision_id: Some(EntityId::new_serial("new63", "new63")),
            related_change_id: Some(EntityId::new_serial("new63", "new63")),
            authority_basis: Some(vec!["patched-63".to_string()]),
            signature_method: Some("patched-63".to_string()),
            rejection_reason: Some(TextField::plain("patched-63")),
            resubmission_date: Some("patched-63".to_string()),
            notification_list: Some(vec![EntityId::new_serial("new63", "new63")]),
            workflow_step: Some("patched-63".to_string()),
            version: Some("patched-63".to_string()),
            audit_trail_ref: Some("patched-63".to_string()),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched ApprovalRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }

    #[test]
    fn meeting_record_patch_round_trips() {
        let mut item = MeetingRecord {
            header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("meetingrecord", "Base MeetingRecord"), "Base MeetingRecord") },
            meeting_type: String::new(),
            scheduled_date: Some(String::new()),
            duration: Some(String::new()),
            location: Some(String::new()),
            chair_id: Some(EntityId::new_serial("base64", "base64")),
            attendee_ids: Vec::new(),
            agenda_items: Vec::new(),
            minutes: Some(TextField::default()),
            action_items: Vec::new(),
            decisions_made: Vec::new(),
            document_refs: Vec::new(),
            follow_up_date: Some(String::new()),
            recording_ref: Some(String::new()),
            quorum_met: false,
            meeting_status: LifecycleStatus::Draft,
            workshop_id: Some(EntityId::new_serial("base64", "base64")),
            stakeholder_ids: Vec::new(),
            requirement_ids: Vec::new(),
            issue_ids: Vec::new(),
            approval_ids: Vec::new(),
        };
        let original = item.clone();
        let patch = MeetingRecordPatch {
            name: Some("Patched MeetingRecord".to_string()),
            description: Some(TextField::plain("desc")),
            status: Some(LifecycleStatus::Approved),
            priority: Some(Priority::Mandatory),
            ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
            tags: Some(vec!["tag".to_string()]),
            notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
            timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
            meeting_type: Some("patched-64".to_string()),
            scheduled_date: Some("patched-64".to_string()),
            duration: Some("patched-64".to_string()),
            location: Some("patched-64".to_string()),
            chair_id: Some(EntityId::new_serial("new64", "new64")),
            attendee_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
            agenda_items: Some(vec!["patched-64".to_string()]),
            minutes: Some(TextField::plain("patched-64")),
            action_items: Some(vec!["patched-64".to_string()]),
            decisions_made: Some(vec![EntityId::new_serial("new64", "new64")]),
            document_refs: Some(vec!["patched-64".to_string()]),
            follow_up_date: Some("patched-64".to_string()),
            recording_ref: Some("patched-64".to_string()),
            quorum_met: Some(true),
            meeting_status: Some(LifecycleStatus::Proposed),
            workshop_id: Some(EntityId::new_serial("new64", "new64")),
            stakeholder_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
            requirement_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
            issue_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
            approval_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        };
        item.apply_patch(&patch);
        let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
        assert_ne!(item, original);
        assert_eq!(item.header.name, "Patched MeetingRecord");
        item.apply_patch(&inverse);
        assert_eq!(item, original);
    }
}