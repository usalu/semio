//! 🧬️ ProgramSnapshot artifact schema — every field of the artifact with its state class.

use crate::artifacts::program::kernel::*;
use crate::artifacts::program::registers::*;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use crate::artifacts::program::ProgramSnapshot;
use math::graph::{orient_endpoints, Undirected};

//#region 🔖️Artifact
/// 🧬️ Full program artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub meta: ProgramMeta,
    #[state(persistent)] pub project: ProjectDefinition,
    #[state(persistent)] pub stakeholders: Vec<Stakeholder>,
    #[state(persistent)] pub users: Vec<UserProfile>,
    #[state(persistent)] pub activities: Vec<Activity>,
    #[state(persistent)] pub functions: Vec<Function>,
    #[state(persistent)] pub elements: Vec<ProgramElement>,
    #[state(persistent)] pub quantities: Vec<QuantityRequirement>,
    #[state(persistent)] pub relationships: Vec<Relationship>,
    #[state(persistent)] pub adjacencies: Vec<Adjacency>,
    #[state(persistent)] pub processes: Vec<Process>,
    #[state(persistent)] pub flows: Vec<FlowRequirement>,
    #[state(persistent)] pub access_rules: Vec<AccessRule>,
    #[state(persistent)] pub operations: Vec<OperationalRequirement>,
    #[state(persistent)] pub equipment: Vec<Equipment>,
    #[state(persistent)] pub resources: Vec<Resource>,
    #[state(persistent)] pub storage: Vec<StorageRequirement>,
    #[state(persistent)] pub environmental: Vec<EnvironmentalRequirement>,
    #[state(persistent)] pub human_factors: Vec<HumanFactorRequirement>,
    #[state(persistent)] pub accessibility: Vec<AccessibilityRequirement>,
    #[state(persistent)] pub privacy: Vec<PrivacyRequirement>,
    #[state(persistent)] pub safety: Vec<SafetyRequirement>,
    #[state(persistent)] pub security: Vec<SecurityRequirement>,
    #[state(persistent)] pub regulatory: Vec<RegulatoryRequirement>,
    #[state(persistent)] pub site_context: Vec<SiteContext>,
    #[state(persistent)] pub organizational: Vec<OrganizationalRequirement>,
    #[state(persistent)] pub services: Vec<ServiceRequirement>,
    #[state(persistent)] pub infrastructure: Vec<InfrastructureRequirement>,
    #[state(persistent)] pub information: Vec<InformationRequirement>,
    #[state(persistent)] pub communication: Vec<CommunicationRequirement>,
    #[state(persistent)] pub wayfinding: Vec<WayfindingRequirement>,
    #[state(persistent)] pub schedules: Vec<ScheduleRequirement>,
    #[state(persistent)] pub flexibility: Vec<FlexibilityRequirement>,
    #[state(persistent)] pub growth: Vec<GrowthPlan>,
    #[state(persistent)] pub sustainability: Vec<SustainabilityRequirement>,
    #[state(persistent)] pub resilience: Vec<ResilienceRequirement>,
    #[state(persistent)] pub costs: Vec<CostRequirement>,
    #[state(persistent)] pub delivery: Vec<DeliveryConstraint>,
    #[state(persistent)] pub risks: Vec<Risk>,
    #[state(persistent)] pub conflicts: Vec<Conflict>,
    #[state(persistent)] pub requirements: Vec<Requirement>,
    #[state(persistent)] pub priorities: Vec<PriorityRecord>,
    #[state(persistent)] pub scenarios: Vec<Scenario>,
    #[state(persistent)] pub options: Vec<OptionEvaluation>,
    #[state(persistent)] pub decisions: Vec<Decision>,
    #[state(persistent)] pub validations: Vec<ValidationRecord>,
    #[state(persistent)] pub performance: Vec<PerformanceCriterion>,
    #[state(persistent)] pub quality: Vec<QualityRecord>,
    #[state(persistent)] pub artifacts: Vec<ArtifactRecord>,
    #[state(persistent)] pub assumptions: Vec<Assumption>,
    #[state(persistent)] pub constraints: Vec<ConstraintRecord>,
    #[state(persistent)] pub compliance_records: Vec<ComplianceRecord>,
    #[state(persistent)] pub approvals: Vec<ApprovalRecord>,
    #[state(persistent)] pub meetings: Vec<MeetingRecord>,
    #[state(persistent)] pub changes: Vec<ChangeRecord>,
    #[state(persistent)] pub collaboration: Vec<CollaborationRecord>,
    #[state(persistent)] pub analyses: Vec<AnalysisRecord>,
    #[state(persistent)] pub reports: Vec<ReportRecord>,
    #[state(persistent)] pub search_filters: Vec<SearchFilter>,
    #[state(persistent)] pub status_records: Vec<StatusRecord>,
    #[state(persistent)] pub workshops: Vec<Workshop>,
    #[state(persistent)] pub surveys: Vec<Survey>,
    #[state(persistent)] pub issues: Vec<Issue>,
    #[state(persistent)] pub audit_events: Vec<AuditEvent>,
    #[state(persistent)] pub templates: Vec<TemplateRecord>,
    #[state(persistent)] pub knowledge: Vec<KnowledgeRecord>,
    #[state(persistent)] pub benchmarks: Vec<BenchmarkRecord>,
    #[state(persistent)] pub traces: Vec<TraceLink>,
    #[state(persistent)] pub governance: Governance,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_register: String,
    #[state(shared_ui)] pub adjacency_kind_filter: Option<AdjacencyKind>,
    #[state(shared_ui)] pub active_report_json: String,
    #[state(local_ui)] pub search_query: String,
    #[state(local_ui)] pub search_history_json: String,
    #[state(local_ui)] pub last_result_json: String,
    #[state(local_ui)] pub last_analysis_json: String,
    #[state(local_ui)] pub graph_camera_x: f64,
    #[state(local_ui)] pub graph_camera_y: f64,
    #[state(local_ui)] pub graph_camera_zoom: f64,
}
//#endregion 🔖️Artifact


//#region 🔖️Conversions
impl Default for ProgramArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::program::empty_plugin())
    }
}

impl ProgramArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::program::ProgramSnapshot {
        crate::artifacts::program::ProgramSnapshot {
            schema: self.schema.clone(), meta: self.meta.clone(), project: self.project.clone(), stakeholders: self.stakeholders.clone(), users: self.users.clone(), activities: self.activities.clone(), functions: self.functions.clone(), elements: self.elements.clone(), quantities: self.quantities.clone(), relationships: self.relationships.clone(), adjacencies: self.adjacencies.clone(), processes: self.processes.clone(), flows: self.flows.clone(), access_rules: self.access_rules.clone(), operations: self.operations.clone(), equipment: self.equipment.clone(), resources: self.resources.clone(), storage: self.storage.clone(), environmental: self.environmental.clone(), human_factors: self.human_factors.clone(), accessibility: self.accessibility.clone(), privacy: self.privacy.clone(), safety: self.safety.clone(), security: self.security.clone(), regulatory: self.regulatory.clone(), site_context: self.site_context.clone(), organizational: self.organizational.clone(), services: self.services.clone(), infrastructure: self.infrastructure.clone(), information: self.information.clone(), communication: self.communication.clone(), wayfinding: self.wayfinding.clone(), schedules: self.schedules.clone(), flexibility: self.flexibility.clone(), growth: self.growth.clone(), sustainability: self.sustainability.clone(), resilience: self.resilience.clone(), costs: self.costs.clone(), delivery: self.delivery.clone(), risks: self.risks.clone(), conflicts: self.conflicts.clone(), requirements: self.requirements.clone(), priorities: self.priorities.clone(), scenarios: self.scenarios.clone(), options: self.options.clone(), decisions: self.decisions.clone(), validations: self.validations.clone(), performance: self.performance.clone(), quality: self.quality.clone(), artifacts: self.artifacts.clone(), assumptions: self.assumptions.clone(), constraints: self.constraints.clone(), compliance_records: self.compliance_records.clone(), approvals: self.approvals.clone(), meetings: self.meetings.clone(), changes: self.changes.clone(), collaboration: self.collaboration.clone(), analyses: self.analyses.clone(), reports: self.reports.clone(), search_filters: self.search_filters.clone(), status_records: self.status_records.clone(), workshops: self.workshops.clone(), surveys: self.surveys.clone(), issues: self.issues.clone(), audit_events: self.audit_events.clone(), templates: self.templates.clone(), knowledge: self.knowledge.clone(), benchmarks: self.benchmarks.clone(), traces: self.traces.clone(), governance: self.governance.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::program::ProgramSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            meta: snapshot.meta,
            project: snapshot.project,
            stakeholders: snapshot.stakeholders,
            users: snapshot.users,
            activities: snapshot.activities,
            functions: snapshot.functions,
            elements: snapshot.elements,
            quantities: snapshot.quantities,
            relationships: snapshot.relationships,
            adjacencies: snapshot.adjacencies,
            processes: snapshot.processes,
            flows: snapshot.flows,
            access_rules: snapshot.access_rules,
            operations: snapshot.operations,
            equipment: snapshot.equipment,
            resources: snapshot.resources,
            storage: snapshot.storage,
            environmental: snapshot.environmental,
            human_factors: snapshot.human_factors,
            accessibility: snapshot.accessibility,
            privacy: snapshot.privacy,
            safety: snapshot.safety,
            security: snapshot.security,
            regulatory: snapshot.regulatory,
            site_context: snapshot.site_context,
            organizational: snapshot.organizational,
            services: snapshot.services,
            infrastructure: snapshot.infrastructure,
            information: snapshot.information,
            communication: snapshot.communication,
            wayfinding: snapshot.wayfinding,
            schedules: snapshot.schedules,
            flexibility: snapshot.flexibility,
            growth: snapshot.growth,
            sustainability: snapshot.sustainability,
            resilience: snapshot.resilience,
            costs: snapshot.costs,
            delivery: snapshot.delivery,
            risks: snapshot.risks,
            conflicts: snapshot.conflicts,
            requirements: snapshot.requirements,
            priorities: snapshot.priorities,
            scenarios: snapshot.scenarios,
            options: snapshot.options,
            decisions: snapshot.decisions,
            validations: snapshot.validations,
            performance: snapshot.performance,
            quality: snapshot.quality,
            artifacts: snapshot.artifacts,
            assumptions: snapshot.assumptions,
            constraints: snapshot.constraints,
            compliance_records: snapshot.compliance_records,
            approvals: snapshot.approvals,
            meetings: snapshot.meetings,
            changes: snapshot.changes,
            collaboration: snapshot.collaboration,
            analyses: snapshot.analyses,
            reports: snapshot.reports,
            search_filters: snapshot.search_filters,
            status_records: snapshot.status_records,
            workshops: snapshot.workshops,
            surveys: snapshot.surveys,
            issues: snapshot.issues,
            audit_events: snapshot.audit_events,
            templates: snapshot.templates,
            knowledge: snapshot.knowledge,
            benchmarks: snapshot.benchmarks,
            traces: snapshot.traces,
            governance: snapshot.governance,
            selected_ids: Vec::new(),
            active_register: "elements".into(),
            adjacency_kind_filter: None,
            active_report_json: String::new(),
            search_query: String::new(),
            search_history_json: "[]".into(),
            last_result_json: String::new(),
            last_analysis_json: String::new(),
            graph_camera_x: 0.0,
            graph_camera_y: 0.0,
            graph_camera_zoom: 1.0,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::program::ProgramSnapshot) {
        self.schema = snapshot.schema;
        self.meta = snapshot.meta;
        self.project = snapshot.project;
        self.stakeholders = snapshot.stakeholders;
        self.users = snapshot.users;
        self.activities = snapshot.activities;
        self.functions = snapshot.functions;
        self.elements = snapshot.elements;
        self.quantities = snapshot.quantities;
        self.relationships = snapshot.relationships;
        self.adjacencies = snapshot.adjacencies;
        self.processes = snapshot.processes;
        self.flows = snapshot.flows;
        self.access_rules = snapshot.access_rules;
        self.operations = snapshot.operations;
        self.equipment = snapshot.equipment;
        self.resources = snapshot.resources;
        self.storage = snapshot.storage;
        self.environmental = snapshot.environmental;
        self.human_factors = snapshot.human_factors;
        self.accessibility = snapshot.accessibility;
        self.privacy = snapshot.privacy;
        self.safety = snapshot.safety;
        self.security = snapshot.security;
        self.regulatory = snapshot.regulatory;
        self.site_context = snapshot.site_context;
        self.organizational = snapshot.organizational;
        self.services = snapshot.services;
        self.infrastructure = snapshot.infrastructure;
        self.information = snapshot.information;
        self.communication = snapshot.communication;
        self.wayfinding = snapshot.wayfinding;
        self.schedules = snapshot.schedules;
        self.flexibility = snapshot.flexibility;
        self.growth = snapshot.growth;
        self.sustainability = snapshot.sustainability;
        self.resilience = snapshot.resilience;
        self.costs = snapshot.costs;
        self.delivery = snapshot.delivery;
        self.risks = snapshot.risks;
        self.conflicts = snapshot.conflicts;
        self.requirements = snapshot.requirements;
        self.priorities = snapshot.priorities;
        self.scenarios = snapshot.scenarios;
        self.options = snapshot.options;
        self.decisions = snapshot.decisions;
        self.validations = snapshot.validations;
        self.performance = snapshot.performance;
        self.quality = snapshot.quality;
        self.artifacts = snapshot.artifacts;
        self.assumptions = snapshot.assumptions;
        self.constraints = snapshot.constraints;
        self.compliance_records = snapshot.compliance_records;
        self.approvals = snapshot.approvals;
        self.meetings = snapshot.meetings;
        self.changes = snapshot.changes;
        self.collaboration = snapshot.collaboration;
        self.analyses = snapshot.analyses;
        self.reports = snapshot.reports;
        self.search_filters = snapshot.search_filters;
        self.status_records = snapshot.status_records;
        self.workshops = snapshot.workshops;
        self.surveys = snapshot.surveys;
        self.issues = snapshot.issues;
        self.audit_events = snapshot.audit_events;
        self.templates = snapshot.templates;
        self.knowledge = snapshot.knowledge;
        self.benchmarks = snapshot.benchmarks;
        self.traces = snapshot.traces;
        self.governance = snapshot.governance;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.architect.program` — twenty handcrafted schema leaves.
pub fn program_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.architect.program",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::program::schema::diff::ProgramDiff;
    use crate::artifacts::program::schema::mutations::ProgramMutation;
    use crate::artifacts::program::schema::snapshot::ProgramSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ProgramBuilderConstruction {
        snapshot: ProgramSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ProgramBuilderConstruction {
        type Snapshot = ProgramSnapshot;
        type Mutation = ProgramMutation;
        type Diff = ProgramDiff;
        fn empty() -> Self { Self { snapshot: ProgramSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ProgramSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <ProgramMutation as protocol::Mutation<ProgramSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <ProgramDiff as protocol::MutationDiff<ProgramSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::program::ProgramSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ProgramParts {
        pub snapshot: Option<ProgramSnapshot>,
    }

    pub struct ProgramAnalyzerAnalysis;

    impl ArtifactAnalysis for ProgramAnalyzerAnalysis {
        type Parts = ProgramParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.program", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ProgramParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ProgramSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec ProgramBuilderFacets {
        construction: derived_construction::ProgramBuilderConstruction,
        analysis: derived_analysis::ProgramAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ProgramComposerComposition,
    }
    builder: ProgramBuilder,
    analyzer: ProgramAnalyzer,
    composer: ProgramComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 📐️ Canonical undirected endpoint order for an adjacency pair — dissolved out of the former
/// `⚙️engine/↔️adjacency` topic (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a
/// pure document helper with no `&mut` and no app coupling, so it lands on the schema root
/// alongside the artifact's other handcrafted document primitives.
pub fn normalize_pair(a: &EntityId, b: &EntityId) -> (EntityId, EntityId) {
    let (left, right) = orient_endpoints::<&str, Undirected>(&a.0, &b.0);
    (EntityId(left.to_string()), EntityId(right.to_string()))
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_pair_orders_endpoints() {
        let a = EntityId("element-2".into());
        let b = EntityId("element-10".into());
        assert_eq!(normalize_pair(&b, &a), (b, a));
    }
}
//#endregion 🧪️Tests
