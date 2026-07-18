//! 🏛️ Architectural programming register entities — typed domain model for all 65 feature areas.

use crate::kernel::*;
use serde::{Deserialize, Serialize};
use vcs::{Identified, Patchable};

// #region 🔖PatchHelpers
trait PatchRow<T: Clone> {
    fn patch_row(&mut self, patch: &Option<T>, inverse: &mut Option<T>);
}

impl<T: Clone> PatchRow<T> for T {
    fn patch_row(&mut self, patch: &Option<T>, inverse: &mut Option<T>) {
        if let Some(value) = patch {
            *inverse = Some(self.clone());
            *self = value.clone();
        }
    }
}

impl<T: Clone> PatchRow<T> for Option<T> {
    fn patch_row(&mut self, patch: &Option<T>, inverse: &mut Option<T>) {
        if let Some(value) = patch {
            *inverse = self.clone();
            *self = Some(value.clone());
        }
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
            fn apply_patch(&mut self, patch: &$patch) -> $patch {
                let mut inverse = <$patch>::default();
                $( PatchRow::patch_row(&mut self$(.$path)+, &patch.$f, &mut inverse.$f); )+
                inverse
            }
        }
    };
}
// #endregion

// #region 🔖SharedEnums
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InfluenceLevel { Low, Medium, High, Critical }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EngagementLevel { Unaware, Resistant, Neutral, Supportive, Leading }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserCategory { Primary, Secondary, Occasional, Service, Visitor, Staff, Public }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramElementKind { Building, Campus, Floor, Zone, Room, Suite, Department, System, Circulation, Support, Outdoor, FurnitureGroup, Other }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FlowKind { People, Material, Information, Service, Equipment, Waste, Emergency, Vehicle }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PrivacyKind { Public, SemiPublic, SemiPrivate, Private, Confidential, Restricted, Anonymous }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SafetyDomain { LifeSafety, OccupationalHealth, Fire, Structural, Electrical, Chemical, Radiation, Ergonomics, Biological, Environmental }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecurityControlKind { AccessControl, Surveillance, Perimeter, Cyber, Personnel, Information, Physical, Procedural, Screening, KeyManagement }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StorageClass { General, Secure, ClimateControlled, Hazardous, Archive, Mobile, Fixed, Shared, ColdChain, Flammable }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnvironmentalParameter { Temperature, Humidity, AirQuality, Lighting, Acoustics, Ventilation, Radiation, Vibration, Pressure, Iaq }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HumanFactorAspect { Ergonomics, Cognition, Sensory, Social, Cultural, Behavioral, Physical, Psychological, Fatigue, Stress }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessMode { Unrestricted, CardControlled, Biometric, Keyed, EscortRequired, TimeRestricted, RoleBased, EmergencyOnly }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputKind { Hierarchy, Taxonomy, Matrix, Network, Journey, Schedule, Dashboard, Summary, Register, Diagram }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdjacencyKind { Required, Preferred, Optional, Prohibited }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionKind { Direct, Indirect, Controlled, SharedAccess, None }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SeparationKind { Acoustic, Visual, Security, Olfactory, Thermal, Fire, Hygienic, Circulation, Operational, InfectionControl }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FlowDirection { OneWay, TwoWay, BidirectionalPeak, Restricted }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessLevel { Public, Restricted, Controlled, Private, Secure, EmergencyOnly }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiskLevel { Negligible, Low, Medium, High, Critical }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConflictKind { Adjacency, Capacity, Schedule, Budget, Regulatory, Operational, Environmental, Security, Priority }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequirementKind { Functional, Spatial, Performance, Regulatory, Operational, Technical, Aesthetic, Sustainability }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValidationStatus { Pending, Passed, Failed, Waived, Deferred }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IssueSeverity { Cosmetic, Minor, Major, Critical, Blocker }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuditAction { Created, Updated, Deleted, Reviewed, Approved, Rejected, Exported, Imported, Merged, Archived }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CostBasis { Capital, Operational, Lifecycle, Replacement, Maintenance }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeliveryPhase { Concept, Schematic, DesignDevelopment, ConstructionDocuments, Procurement, Construction, Commissioning, Occupancy }
// #endregion

// #region 🔖ProgramMeta
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    fn apply_patch(&mut self, patch: &ProgramMetaPatch) -> ProgramMetaPatch {
        let mut inverse = ProgramMetaPatch::default();
        PatchRow::patch_row(&mut self.schema, &patch.schema, &mut inverse.schema);
        PatchRow::patch_row(&mut self.document_id, &patch.document_id, &mut inverse.document_id);
        PatchRow::patch_row(&mut self.title, &patch.title, &mut inverse.title);
        PatchRow::patch_row(&mut self.subtitle, &patch.subtitle, &mut inverse.subtitle);
        PatchRow::patch_row(&mut self.purpose, &patch.purpose, &mut inverse.purpose);
        PatchRow::patch_row(&mut self.terminology, &patch.terminology, &mut inverse.terminology);
        PatchRow::patch_row(&mut self.classification, &patch.classification, &mut inverse.classification);
        PatchRow::patch_row(&mut self.industry_sector, &patch.industry_sector, &mut inverse.industry_sector);
        PatchRow::patch_row(&mut self.project_type, &patch.project_type, &mut inverse.project_type);
        PatchRow::patch_row(&mut self.locale, &patch.locale, &mut inverse.locale);
        PatchRow::patch_row(&mut self.revision, &patch.revision, &mut inverse.revision);
        PatchRow::patch_row(&mut self.author_ids, &patch.author_ids, &mut inverse.author_ids);
        PatchRow::patch_row(&mut self.source_system, &patch.source_system, &mut inverse.source_system);
        PatchRow::patch_row(&mut self.export_profile, &patch.export_profile, &mut inverse.export_profile);
        PatchRow::patch_row(&mut self.timestamps, &patch.timestamps, &mut inverse.timestamps);
        inverse
    }
}
// #endregion

// #region 🔖ProjectDefinition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Stakeholder
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖UserProfile
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Activity
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Function
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ProgramElement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖QuantityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Relationship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Adjacency
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub distance_max_m: Option<f64>,
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

// #region 🔖Process
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖FlowRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub clear_width_m: Option<f64>,
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

// #region 🔖AccessRule
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖OperationalRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Equipment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub weight_kg: Option<f64>,
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

// #region 🔖Resource
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖StorageRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub stored_item: String,
    pub storage_class: StorageClass,
    pub quantity: QuantitySpec,
    pub volume_m3: Option<f64>,
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

// #region 🔖EnvironmentalRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖HumanFactorRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖AccessibilityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityRequirement {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub standard: String,
    pub level: Option<String>,
    pub user_profile_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub route_ids: Vec<EntityId>,
    pub clear_width_m: Option<f64>,
    pub clear_height_m: Option<f64>,
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

// #region 🔖PrivacyRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖SafetyRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖SecurityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖RegulatoryRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖SiteContext
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteContext {
    #[serde(flatten)]
    pub header: EntityHeader,
    pub site_name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
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

// #region 🔖OrganizationalRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub wellness_programs: Vec<String>,
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
    pub wellness_programs: Option<Vec<String>>,
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
        [wellness_programs] => wellness_programs,
        [diversity_goals] => diversity_goals,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖ServiceRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖InfrastructureRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖InformationRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖CommunicationRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖WayfindingRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ScheduleRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖FlexibilityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖GrowthPlan
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖SustainabilityRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ResilienceRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖CostRequirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖DeliveryConstraint
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Risk
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Conflict
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Requirement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖PriorityRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Scenario
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖OptionEvaluation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Decision
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ValidationRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖PerformanceCriterion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖QualityRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖DocumentRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ChangeRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖CollaborationRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖AnalysisRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ReportRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖SearchFilter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖StatusRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Workshop
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Survey
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Issue
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖AuditEvent
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖TemplateRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖KnowledgeRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖BenchmarkRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Assumption
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ConstraintRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ComplianceRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖ApprovalRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖MeetingRecord
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

// #region 🔖Governance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
