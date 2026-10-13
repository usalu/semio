//! 🧬️ ProgramSnapshot snapshot schema — artifact-lane fields only.

use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::*;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::schema::ARCHITECT_PROGRAM_SCHEMA;


//#region 🔖️Snapshot
/// 📸️ Persisted architect program snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "architect", layout = "lines")]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub meta: ProgramMeta,
    #[dsl(block)]
    #[state(artifact)]
    pub project: ProjectDefinition,
    #[dsl(table)]
    #[state(artifact)]
    pub stakeholders: Vec<Stakeholder>,
    #[dsl(table)]
    #[state(artifact)]
    pub users: Vec<UserProfile>,
    #[dsl(table)]
    #[state(artifact)]
    pub activities: Vec<Activity>,
    #[dsl(table)]
    #[state(artifact)]
    pub functions: Vec<Function>,
    #[dsl(table)]
    #[state(artifact)]
    pub elements: Vec<ProgramElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub quantities: Vec<QuantityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub relationships: Vec<Relationship>,
    #[dsl(table)]
    #[state(artifact)]
    pub adjacencies: Vec<Adjacency>,
    #[dsl(table)]
    #[state(artifact)]
    pub processes: Vec<Process>,
    #[dsl(table)]
    #[state(artifact)]
    pub flows: Vec<FlowRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub access_rules: Vec<AccessRule>,
    #[dsl(table)]
    #[state(artifact)]
    pub operations: Vec<OperationalRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub equipment: Vec<Equipment>,
    #[dsl(table)]
    #[state(artifact)]
    pub resources: Vec<Resource>,
    #[dsl(table)]
    #[state(artifact)]
    pub storage: Vec<StorageRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub environmental: Vec<EnvironmentalRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub human_factors: Vec<HumanFactorRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub accessibility: Vec<AccessibilityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub privacy: Vec<PrivacyRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub safety: Vec<SafetyRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub security: Vec<SecurityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub regulatory: Vec<RegulatoryRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub site_context: Vec<SiteContext>,
    #[dsl(table)]
    #[state(artifact)]
    pub organizational: Vec<OrganizationalRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub services: Vec<ServiceRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub infrastructure: Vec<InfrastructureRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub information: Vec<InformationRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub communication: Vec<CommunicationRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub wayfinding: Vec<WayfindingRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub schedules: Vec<ScheduleRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub flexibility: Vec<FlexibilityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub growth: Vec<GrowthPlan>,
    #[dsl(table)]
    #[state(artifact)]
    pub sustainability: Vec<SustainabilityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub resilience: Vec<ResilienceRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub costs: Vec<CostRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub delivery: Vec<DeliveryConstraint>,
    #[dsl(table)]
    #[state(artifact)]
    pub risks: Vec<Risk>,
    #[dsl(table)]
    #[state(artifact)]
    pub conflicts: Vec<Conflict>,
    #[dsl(table)]
    #[state(artifact)]
    pub requirements: Vec<Requirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub priorities: Vec<PriorityRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub scenarios: Vec<Scenario>,
    #[dsl(table)]
    #[state(artifact)]
    pub options: Vec<OptionEvaluation>,
    #[dsl(table)]
    #[state(artifact)]
    pub decisions: Vec<Decision>,
    #[dsl(table)]
    #[state(artifact)]
    pub validations: Vec<ValidationRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub performance: Vec<PerformanceCriterion>,
    #[dsl(table)]
    #[state(artifact)]
    pub quality: Vec<QualityRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub artifacts: Vec<ArtifactRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub assumptions: Vec<Assumption>,
    #[dsl(table)]
    #[state(artifact)]
    pub constraints: Vec<ConstraintRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub compliance_records: Vec<ComplianceRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub approvals: Vec<ApprovalRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub meetings: Vec<MeetingRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub changes: Vec<ChangeRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub collaboration: Vec<CollaborationRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub analyses: Vec<AnalysisRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub reports: Vec<ReportRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub search_filters: Vec<SearchFilter>,
    #[dsl(table)]
    #[state(artifact)]
    pub status_records: Vec<StatusRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub workshops: Vec<Workshop>,
    #[dsl(table)]
    #[state(artifact)]
    pub surveys: Vec<Survey>,
    #[dsl(table)]
    #[state(artifact)]
    pub issues: Vec<Issue>,
    #[dsl(table)]
    #[state(artifact)]
    pub audit_events: Vec<AuditEvent>,
    #[dsl(table)]
    #[state(artifact)]
    pub templates: Vec<TemplateRecord>,
    /// 📚️ The rows the composed `knowledge` child is derived from — the PERSISTED payload of that
    /// child slot (`🏭️process`'s `stock_payload`/`step_payloads` pattern). A composed child's content
    /// never travels inside `store::os_store::artifact_child::ArtifactChild`, and the react shell answers `Effect::LoadDocument`
    /// with an EMPTY member roster, so `crate::standards::v1::subsets::any::schema::snapshot::genesis_program_child_pack` can only derive the table
    /// from a field the parent's own pack/DSL round-trips.
    #[dsl(table)]
    #[state(artifact)]
    pub knowledge_payload: Vec<KnowledgeRecord>,
    #[dsl(block)]
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub knowledge: crate::standards::v1::subsets::any::schema::snapshot::ProgramKnowledgeChild,
    /// 🏁️ The rows the composed `benchmarks` child is derived from — see
    /// [`ProgramSnapshot::knowledge_payload`].
    #[dsl(table)]
    #[state(artifact)]
    pub benchmarks_payload: Vec<BenchmarkRecord>,
    #[dsl(block)]
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub benchmarks: crate::standards::v1::subsets::any::schema::snapshot::ProgramBenchmarksChild,
    #[dsl(table)]
    #[state(artifact)]
    pub traces: Vec<TraceLink>,
    #[dsl(block)]
    #[state(artifact)]
    pub governance: Governance,
}

impl Default for ProgramSnapshot {
    fn default() -> Self {
        crate::standards::v1::subsets::any::schema::snapshot::empty_plugin()
    }
}

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🔖️ExternalBridges



//#endregion 🔖️ExternalBridges

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4 batch Db (`architect→C:table,graph
/// R:model`): `program.benchmarks` (an id-keyed register that is NOT one of the 8 registers wired
/// into `patch_register_item_operation`'s reflection dispatch, unlike e.g. `stakeholders`/
/// `elements`) is the first — smallest, lowest-blast-radius — of this plugin's 68 register
/// collections to compose stdio's `table`
/// subset, proving the pattern before a wider follow-up pass. Every `BenchmarkRecord` row nests a
/// rich `EntityHeader` (id/name/status/priority/ownership/tags/notes/timestamps) that has no clean
/// native `table`-column mapping, so the converter below follows `🕸️dag`'s "honest string boundary"
/// precedent: `id`/`name` are ALSO projected onto native columns for genuine table-tooling, but the
/// full row (source of truth) round-trips as one JSON cell — nothing is silently dropped.
///
/// Every one of the four existing `create`/`replace`/`delete`/`rename` mutation triads for
/// `benchmarks` keeps its exact public payload/wire shape (`CreateBenchmarkRecord`,
/// `ReplaceBenchmarkRecord`, …) — only the internal `🔺️diff`/`↩️inverse` bodies are rewired to read
/// the exact child owner below and re-mint a fresh content-addressed child handle for the identical per-entry
/// mutation-rich shape (`📓️migration-recipe.md` §3/§4 — no `LinkResolver`/child-dispatch seam
/// exists in `ArtifactApp::handle` yet, checked directly against `🔌️plugin/🦀️.rs`,
/// W1-owned, read-only).
//#region 🔖️ChildTypes
pub type ProgramBenchmarksChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: `benchmarks` rows <-> `table` rows — three columns (`id: Str`,
/// `name: Str`, `json: Str`). `json` is the FULL `serde_json` serialization of the row (source of
/// truth on decode); `id`/`name` are a redundant native-column projection for table-shaped tooling
/// that only understands the neutral subset — the same split `🕸️dag`'s node/edge converter uses for
/// its own richer-than-native domain type.
pub fn benchmark_table_from_records(records: &[BenchmarkRecord]) -> semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio_table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "name".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "json".into(), kind: SemioTableCellKind::Str }],
        rows: records
            .iter()
            .map(|record| SemioTableRow { cells: vec![SemioValue::Str { value: record.header.id.0.clone() }, SemioValue::Str { value: record.header.name.clone() }, SemioValue::Str { value: semio_framework_pack_json::to_json_string(record) }] })
            .collect(),
    }
}

/// 🌉 Inverse of the converter above — real reconstruction from the `json` cell (source of truth),
/// never a stub. A row whose `json` cell is missing or fails to parse is honestly SKIPPED (not
/// fabricated from `id`/`name` alone, since `BenchmarkRecord` has no `Default` and a partial
/// reconstruction would silently invent data) — documented here rather than hidden.
pub fn benchmark_records_from_table(table: &semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot) -> Vec<BenchmarkRecord> {
    use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;
    table
        .rows
        .iter()
        .filter_map(|row| match row.cells.get(2) {
            Some(SemioValue::Str { value }) => semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).ok(),
            _ => None,
        })
        .collect()
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral benchmark rows owned by one exact table child. Equal wire identities never
/// share rows, and the value retires with its owner.
#[derive(Clone)]
pub struct ProgramBenchmarksWorkingTable {
    pub records: Vec<BenchmarkRecord>,
}

fn program_benchmarks_scene_id(records: &[BenchmarkRecord]) -> String {
    let content_json = semio_framework_pack_json::to_json_string(&records.to_vec());
    store::content_id("architect-benchmarks", content_json.as_bytes())
}

fn program_benchmarks_target(scene_id: &str) -> semio_framework_artifact_reference::ArtifactRef {
    semio_framework_artifact_reference::ArtifactRef { artifact_id: scene_id.into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

/// 🏗️ Mints the composed-child handle and transfers rows into that exact owner.
pub fn benchmarks_child_from_records(records: &[BenchmarkRecord]) -> ProgramBenchmarksChild {
    let scene_id = program_benchmarks_scene_id(records);
    let target = program_benchmarks_target(&scene_id);
    store::os_store::artifact_child::ArtifactChild::new(scene_id, target).with_local_owner(std::sync::Arc::new(ProgramBenchmarksWorkingTable { records: records.to_vec() }))
}

//#endregion 🔖️WorkingScene

//#region 🔖️Knowledge
/// 🧩️ `program.knowledge` — second proof-of-pattern field (also outside
/// `patch_register_item_operation`'s reflection dispatch), composed identically to `benchmarks`
/// above. `KnowledgeRecord` also nests a rich `EntityHeader` with no clean native `table`-column
/// mapping, so it follows the identical `id`/`name`-native-plus-full-`json` converter shape.
//#region 🔖️ChildTypes
pub type ProgramKnowledgeChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
pub fn knowledge_table_from_records(records: &[KnowledgeRecord]) -> semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio_table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "name".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "json".into(), kind: SemioTableCellKind::Str }],
        rows: records
            .iter()
            .map(|record| SemioTableRow { cells: vec![SemioValue::Str { value: record.header.id.0.clone() }, SemioValue::Str { value: record.header.name.clone() }, SemioValue::Str { value: semio_framework_pack_json::to_json_string(record) }] })
            .collect(),
    }
}

pub fn knowledge_records_from_table(table: &semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot) -> Vec<KnowledgeRecord> {
    use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;
    table
        .rows
        .iter()
        .filter_map(|row| match row.cells.get(2) {
            Some(SemioValue::Str { value }) => semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).ok(),
            _ => None,
        })
        .collect()
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral knowledge rows owned by one exact table child.
#[derive(Clone)]
pub struct ProgramKnowledgeWorkingTable {
    pub records: Vec<KnowledgeRecord>,
}

fn program_knowledge_scene_id(records: &[KnowledgeRecord]) -> String {
    let content_json = semio_framework_pack_json::to_json_string(&records.to_vec());
    store::content_id("architect-knowledge", content_json.as_bytes())
}

fn program_knowledge_target(scene_id: &str) -> semio_framework_artifact_reference::ArtifactRef {
    semio_framework_artifact_reference::ArtifactRef { artifact_id: scene_id.into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

pub fn knowledge_child_from_records(records: &[KnowledgeRecord]) -> ProgramKnowledgeChild {
    let scene_id = program_knowledge_scene_id(records);
    let target = program_knowledge_target(&scene_id);
    store::os_store::artifact_child::ArtifactChild::new(scene_id, target).with_local_owner(std::sync::Arc::new(ProgramKnowledgeWorkingTable { records: records.to_vec() }))
}

//#endregion 🔖️WorkingScene
//#endregion 🔖️Knowledge

//#region 🔖️Genesis
/// 🌱️ `ArtifactEditor`/`ArtifactViewer::genesis_child_pack` for the program's composed `knowledge` and
/// `benchmarks` tables — pure functions of the parent's child owners. The react shell's
/// `loadDocumentPair` sends `members: []`, so a whole-document load (`setActiveExample` →
/// `Effect::LoadDocument`) derives both slots here; without it the archive closure completes
/// `Incomplete` and the load is refused (`document-archive-replacement.closure-rejected`).
pub fn genesis_program_child_pack(snapshot: &ProgramSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot;
    use store::os_store::serialization::ArtifactPack;
    match slot {
        "knowledge" if child_id == snapshot.knowledge.child_id => Some(<SemioTableSnapshot as ArtifactPack>::encode_pack(&knowledge_table_from_records(&snapshot.knowledge_payload))),
        "benchmarks" if child_id == snapshot.benchmarks.child_id => Some(<SemioTableSnapshot as ArtifactPack>::encode_pack(&benchmark_table_from_records(&snapshot.benchmarks_payload))),
        _ => None,
    }
}
//#endregion 🔖️Genesis
//#endregion 🔖️Composition

//#region 🔖️Constructors
/// 🌱️ The empty program document: one project, one governance record and no register rows.
pub fn empty_plugin() -> ProgramSnapshot {
    let project_id = EntityId::new_serial("project", "project");
    let governance_id = EntityId::new_serial("governance", "governance");
    ProgramSnapshot {
        schema: ARCHITECT_PROGRAM_SCHEMA.into(),
        meta: ProgramMeta {
            schema: ARCHITECT_PROGRAM_SCHEMA.into(),
            document_id: EntityId::new_serial("document", "document").0,
            title: String::new(),
            subtitle: None,
            purpose: TextField::plain(""),
            terminology: Vec::new(),
            classification: Vec::new(),
            industry_sector: String::new(),
            project_type: String::new(),
            locale: "en".into(),
            revision: "0".into(),
            author_ids: Vec::new(),
            source_system: None,
            export_profile: None,
            timestamps: TimestampMeta::default(),
        },
        project: ProjectDefinition {
            id: project_id,
            code: String::new(),
            client_name: String::new(),
            owner_organization: String::new(),
            brief_summary: TextField::plain(""),
            problem_statement: TextField::plain(""),
            vision: TextField::plain(""),
            mission: TextField::plain(""),
            objectives: Vec::new(),
            success_criteria: Vec::new(),
            project_priorities: Vec::new(),
            completion_criteria: Vec::new(),
            decision_criteria: Vec::new(),
            scope_inclusions: Vec::new(),
            scope_exclusions: Vec::new(),
            assumptions: Vec::new(),
            constraints_summary: Vec::new(),
            dependencies: Vec::new(),
            deliverables: Vec::new(),
            phases: Vec::new(),
            geographic_context: TextField::plain(""),
            development_context: TextField::plain(""),
            operational_context: TextField::plain(""),
            regulatory_context: Vec::new(),
            funding_model: String::new(),
            ownership: Ownership::default(),
            timestamps: TimestampMeta::default(),
        },
        stakeholders: Vec::new(),
        users: Vec::new(),
        activities: Vec::new(),
        functions: Vec::new(),
        elements: Vec::new(),
        quantities: Vec::new(),
        relationships: Vec::new(),
        adjacencies: Vec::new(),
        processes: Vec::new(),
        flows: Vec::new(),
        access_rules: Vec::new(),
        operations: Vec::new(),
        equipment: Vec::new(),
        resources: Vec::new(),
        storage: Vec::new(),
        environmental: Vec::new(),
        human_factors: Vec::new(),
        accessibility: Vec::new(),
        privacy: Vec::new(),
        safety: Vec::new(),
        security: Vec::new(),
        regulatory: Vec::new(),
        site_context: Vec::new(),
        organizational: Vec::new(),
        services: Vec::new(),
        infrastructure: Vec::new(),
        information: Vec::new(),
        communication: Vec::new(),
        wayfinding: Vec::new(),
        schedules: Vec::new(),
        flexibility: Vec::new(),
        growth: Vec::new(),
        sustainability: Vec::new(),
        resilience: Vec::new(),
        costs: Vec::new(),
        delivery: Vec::new(),
        risks: Vec::new(),
        conflicts: Vec::new(),
        requirements: Vec::new(),
        priorities: Vec::new(),
        scenarios: Vec::new(),
        options: Vec::new(),
        decisions: Vec::new(),
        validations: Vec::new(),
        performance: Vec::new(),
        quality: Vec::new(),
        artifacts: Vec::new(),
        assumptions: Vec::new(),
        constraints: Vec::new(),
        compliance_records: Vec::new(),
        approvals: Vec::new(),
        meetings: Vec::new(),
        changes: Vec::new(),
        collaboration: Vec::new(),
        analyses: Vec::new(),
        reports: Vec::new(),
        search_filters: Vec::new(),
        status_records: Vec::new(),
        workshops: Vec::new(),
        surveys: Vec::new(),
        issues: Vec::new(),
        audit_events: Vec::new(),
        templates: Vec::new(),
        knowledge_payload: Vec::new(),
        knowledge: knowledge_child_from_records(&[]),
        benchmarks_payload: Vec::new(),
        benchmarks: benchmarks_child_from_records(&[]),
        governance: Governance {
            id: governance_id,
            framework: String::new(),
            roles: Vec::new(),
            responsibilities: Vec::new(),
            approval_matrix: Vec::new(),
            escalation_paths: Vec::new(),
            meeting_cadence: Vec::new(),
            decision_rights: Vec::new(),
            change_control_process: Vec::new(),
            quality_policy: TextField::plain(""),
            risk_appetite: None,
            compliance_obligations: Vec::new(),
            audit_schedule: None,
            document_control: Vec::new(),
            stakeholder_engagement_plan: Vec::new(),
            ethics_policy: Vec::new(),
            data_governance: Vec::new(),
            owner_id: None,
            review_cycle: None,
            review_hierarchy: Vec::new(),
            policy_ownership_id: None,
            requirement_ownership_id: None,
            risk_ownership_id: None,
            reporting_frequency: None,
            accountability_rules: Vec::new(),
            exception_management: Vec::new(),
            governance_performance: Vec::new(),
        },
        traces: Vec::new(),
    }
}

/// 🧪️ Sample program for tests with elements, stakeholders, and one adjacency.
pub fn sample_plugin() -> ProgramSnapshot {
    let mut program = empty_plugin();
    program.meta.title = "Sample Clinic".into();
    program.meta.industry_sector = "healthcare".into();
    program.project.code = "CLN-001".into();
    program.project.client_name = "Sample Health".into();

    let reception_id = EntityId::new_serial("element", "element");
    let waiting_id = EntityId::new_serial("element", "element");
    program.elements.push(ProgramElement {
        header: EntityHeader::new(reception_id.clone(), "Reception"),
        code: "REC".into(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: Some("L1".into()),
        area: QuantitySpec::target_unit(25.0, "m2"),
        volume: QuantitySpec::default(),
        height: QuantitySpec::default(),
        occupancy: QuantitySpec::target_unit(4.0, "persons"),
        function_ids: Vec::new(),
        activity_ids: Vec::new(),
        user_profile_ids: Vec::new(),
        adjacency_ids: Vec::new(),
        quantity_ids: Vec::new(),
        requirement_ids: Vec::new(),
        location_hint: None,
        orientation: None,
        daylight_requirement: None,
        acoustic_class: None,
        security_zone: None,
        flexibility_notes: Vec::new(),
        growth_allocation: None,
        circulation_role: None,
        visibility_level: None,
        adjacency_preferences: Vec::new(),
        environmental_zone: None,
    });
    program.elements.push(ProgramElement {
        header: EntityHeader::new(waiting_id.clone(), "Waiting"),
        code: "WAI".into(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: Some("L1".into()),
        area: QuantitySpec::target_unit(40.0, "m2"),
        volume: QuantitySpec::default(),
        height: QuantitySpec::default(),
        occupancy: QuantitySpec::target_unit(12.0, "persons"),
        function_ids: Vec::new(),
        activity_ids: Vec::new(),
        user_profile_ids: Vec::new(),
        adjacency_ids: Vec::new(),
        quantity_ids: Vec::new(),
        requirement_ids: Vec::new(),
        location_hint: None,
        orientation: None,
        daylight_requirement: None,
        acoustic_class: None,
        security_zone: None,
        flexibility_notes: Vec::new(),
        growth_allocation: None,
        circulation_role: None,
        visibility_level: None,
        adjacency_preferences: Vec::new(),
        environmental_zone: None,
    });

    let stakeholder_id = EntityId::new_serial("stakeholder", "stakeholder");
    program.stakeholders.push(Stakeholder {
        header: EntityHeader::new(stakeholder_id, "Facilities Director"),
        role: "Owner".into(),
        organization: "Sample Health".into(),
        department: None,
        contact_email: None,
        contact_phone: None,
        influence: InfluenceLevel::High,
        interest: InfluenceLevel::High,
        engagement: EngagementLevel::Leading,
        expectations: vec!["On-time delivery".into()],
        concerns: Vec::new(),
        requirement_ids: Vec::new(),
        decision_authority: true,
        communication_preferences: Vec::new(),
        reporting_frequency: None,
        involvement_phases: Vec::new(),
        availability: None,
        representative_of: None,
        delegated_to: None,
        relationship_to_client: None,
        power_interest_notes: Vec::new(),
        stakeholder_type: "Internal".into(),
        influence_strategy: None,
        communication_channels: Vec::new(),
        success_metrics: Vec::new(),
    });

    let (a, b) = standards::v1::subsets::any::schema::normalize_pair(&reception_id, &waiting_id);
    program.adjacencies.push(Adjacency {
        header: EntityHeader::new(EntityId::new_serial("adjacency", "Reception ↔ Waiting"), "Reception ↔ Waiting"),
        element_a_id: a,
        element_b_id: b,
        kind: AdjacencyKind::Required,
        connection: ConnectionKind::Direct,
        separations: Vec::new(),
        weight: 1.0,
        rationale: None,
        distance_max_m: None,
        distance_min_m: None,
        level_constraint: None,
        access_path: None,
        shared_wall: true,
        shared_entry: false,
        traffic_isolation: false,
        circulation_overlap: true,
        conflict_ids: Vec::new(),
        normalized: true,
        verification_status: ValidationStatus::Pending,
        source_relationship_id: None,
        internal_external_access: None,
    });

    program.knowledge_payload = vec![KnowledgeRecord {
        header: EntityHeader::new(EntityId::new_serial("knowledge", "knowledge"), "Clinic Reception Sightlines"),
        topic: "Reception".into(),
        category: "Wayfinding".into(),
        summary: TextField::plain("Keep the waiting area in direct sight of the reception desk."),
        content: TextField::plain("Staff at the reception desk must see every seat of the waiting area without standing up; this shortens perceived wait times and removes the need for a second supervision post."),
        sources: vec!["Sample Health facility review 2025".into()],
        references: Vec::new(),
        lessons_learned: vec!["A single sightline removes one supervision post.".into()],
        best_practices: vec!["Place the desk on the waiting area's long axis.".into()],
        applicable_sectors: vec!["healthcare".into()],
        related_entity_kinds: vec!["element".into()],
        author_ids: Vec::new(),
        expertise_level: Some("practitioner".into()),
        validation_status: ValidationStatus::Pending,
        last_reviewed: None,
        keywords: vec!["reception".into(), "sightline".into()],
        attachments: Vec::new(),
        citations: Vec::new(),
        usage_count: 1,
    }];
    program.knowledge = knowledge_child_from_records(&program.knowledge_payload);

    program.benchmarks_payload = vec![BenchmarkRecord {
        header: EntityHeader::new(EntityId::new_serial("benchmark", "benchmark"), "Outpatient Waiting Area per Seat"),
        benchmark_name: "Outpatient waiting area per seat".into(),
        sector: "healthcare".into(),
        metric: "area-per-seat".into(),
        value: 3.2,
        unit: "m2".into(),
        sample_size: Some(42),
        source: Some("Sample Health portfolio".into()),
        collection_year: Some(2025),
        geography: Some("DE".into()),
        building_type: Some("clinic".into()),
        confidence: Some("medium".into()),
        methodology: Some("Floor areas measured from as-built plans.".into()),
        applicable_element_kinds: vec!["room".into()],
        related_requirement_ids: Vec::new(),
        comparison_notes: vec!["The sample clinic's 40 m2 waiting area seats 12.".into()],
        limitations: vec!["Single operator, one region.".into()],
        license: Some("CC-BY-4.0".into()),
        knowledge_id: None,
        last_verified: None,
    }];
    program.benchmarks = benchmarks_child_from_records(&program.benchmarks_payload);

    program.audit_events.push(AuditEvent {
        header: EntityHeader::new(EntityId::new_serial("audit", "audit"), "Clinic brief created"),
        action: AuditAction::Created,
        actor_id: None,
        subject_id: reception_id,
        subject_kind: "element".into(),
        timestamp: "2026-08-14T09:30:00Z".into(),
        details: TextField::plain("Reception and waiting area brief authored"),
        before_state: None,
        after_state: None,
        ip_address: None,
        client: None,
        session_id: None,
        change_record_id: None,
        trace_link: None,
        success: true,
        error_message: None,
        correlation_id: None,
        compliance_tags: Vec::new(),
        retention_until: None,
    });

    program
}
//#endregion 🔖️Constructors
