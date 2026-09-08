//! 🏛️ Architect program artifact — the root program document: all 65 feature-area registers plus
//! meta, project, and governance (constitutional: general).
//!
//! Domain row types live under `🧬️schema/🗄️registers`; shared entity primitives under
//! `🧬️schema/🧱️kernel`. The persisted snapshot type is `ProgramSnapshot`.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
mod art_program_demo_tests;
extern crate semio_framework_schema as framework_schema;

pub use crate::kernel::*;
pub use crate::registers::*;
pub use crate::schema::snapshot::ProgramSnapshot;

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
pub type ProgramBenchmarksChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: `benchmarks` rows <-> `table` rows — three columns (`id: Str`,
/// `name: Str`, `json: Str`). `json` is the FULL `serde_json` serialization of the row (source of
/// truth on decode); `id`/`name` are a redundant native-column projection for table-shaped tooling
/// that only understands the neutral subset — the same split `🕸️dag`'s node/edge converter uses for
/// its own richer-than-native domain type.
pub fn benchmark_table_from_records(records: &[BenchmarkRecord]) -> semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "name".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "json".into(), kind: SemioTableCellKind::Str }],
        rows: records
            .iter()
            .map(|record| SemioTableRow { cells: vec![SemioValue::Str { value: record.header.id.0.clone() }, SemioValue::Str { value: record.header.name.clone() }, SemioValue::Str { value: dsl::json::to_json_string(record) }] })
            .collect(),
    }
}

/// 🌉 Inverse of the converter above — real reconstruction from the `json` cell (source of truth),
/// never a stub. A row whose `json` cell is missing or fails to parse is honestly SKIPPED (not
/// fabricated from `id`/`name` alone, since `BenchmarkRecord` has no `Default` and a partial
/// reconstruction would silently invent data) — documented here rather than hidden.
pub fn benchmark_records_from_table(table: &semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot) -> Vec<BenchmarkRecord> {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    table
        .rows
        .iter()
        .filter_map(|row| match row.cells.get(2) {
            Some(SemioValue::Str { value }) => dsl::json::from_json_str(value).ok(),
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
    use std::hash::{Hash, Hasher};
    let content_json = dsl::json::to_json_string(&records.to_vec());
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("architect-benchmarks-{:016x}", hasher.finish())
}

fn program_benchmarks_target() -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: "architect-program-benchmarks".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

/// 🏗️ Mints the composed-child handle and transfers rows into that exact owner.
pub fn benchmarks_child_from_records(records: &[BenchmarkRecord]) -> ProgramBenchmarksChild {
    let scene_id = program_benchmarks_scene_id(records);
    store::ArtifactChild::new(scene_id, program_benchmarks_target()).with_local_owner(std::sync::Arc::new(ProgramBenchmarksWorkingTable { records: records.to_vec() }))
}

/// 🔎 The live `benchmarks` rows behind a snapshot's composed child — the single read call site
/// every mutation-diff/panel/report call path in this artifact now uses instead of a direct
/// `.benchmarks` field. A wire-only child fails soft until host materialization.
pub fn program_benchmarks(snapshot: &ProgramSnapshot) -> Vec<BenchmarkRecord> {
    snapshot.benchmarks.local_owner::<ProgramBenchmarksWorkingTable>().map(|table| table.records.clone()).unwrap_or_default()
}
//#endregion 🔖️WorkingScene

//#region 🔖️Knowledge
/// 🧩️ `program.knowledge` — second proof-of-pattern field (also outside
/// `patch_register_item_operation`'s reflection dispatch), composed identically to `benchmarks`
/// above. `KnowledgeRecord` also nests a rich `EntityHeader` with no clean native `table`-column
/// mapping, so it follows the identical `id`/`name`-native-plus-full-`json` converter shape.
//#region 🔖️ChildTypes
pub type ProgramKnowledgeChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
pub fn knowledge_table_from_records(records: &[KnowledgeRecord]) -> semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "name".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "json".into(), kind: SemioTableCellKind::Str }],
        rows: records
            .iter()
            .map(|record| SemioTableRow { cells: vec![SemioValue::Str { value: record.header.id.0.clone() }, SemioValue::Str { value: record.header.name.clone() }, SemioValue::Str { value: dsl::json::to_json_string(record) }] })
            .collect(),
    }
}

pub fn knowledge_records_from_table(table: &semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot) -> Vec<KnowledgeRecord> {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    table
        .rows
        .iter()
        .filter_map(|row| match row.cells.get(2) {
            Some(SemioValue::Str { value }) => dsl::json::from_json_str(value).ok(),
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
    use std::hash::{Hash, Hasher};
    let content_json = dsl::json::to_json_string(&records.to_vec());
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("architect-knowledge-{:016x}", hasher.finish())
}

fn program_knowledge_target() -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: "architect-program-knowledge".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

pub fn knowledge_child_from_records(records: &[KnowledgeRecord]) -> ProgramKnowledgeChild {
    let scene_id = program_knowledge_scene_id(records);
    store::ArtifactChild::new(scene_id, program_knowledge_target()).with_local_owner(std::sync::Arc::new(ProgramKnowledgeWorkingTable { records: records.to_vec() }))
}

pub fn program_knowledge(snapshot: &ProgramSnapshot) -> Vec<KnowledgeRecord> {
    snapshot.knowledge.local_owner::<ProgramKnowledgeWorkingTable>().map(|table| table.records.clone()).unwrap_or_default()
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Knowledge
//#endregion 🔖️Composition

#[cfg(test)]
use store::ArtifactDsl;

/// @emoji 📜️ Persisted architect program document schema identifier.
pub use crate::schema::mutations::ProgramMutation;

pub use crate::schema::diff::ProgramDiff;

pub const ARCHITECT_PROGRAM_SCHEMA: &str = "architect.program";

//#region 🔖️Dialect
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// role surface (`✏️editor`, `👁️viewer`) for this subset shares — lives at the ARTIFACT level (not
/// under `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling
/// editor module. `artifact_kind` matches this schema's own `#[artifact_schema(id = "…")]` ("s.architect.program",
/// confirmed at `🧬️schema/🦀️component.rs:14`) and `definition()`'s own schema-capability claim
/// above (`"s.architect.program"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.architect.program@1/*#editor` / `s.architect.program@1/*#viewer` (contract §1 grammar).
pub const ARCHITECT_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.architect.program", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.program`).
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "data.program".into(),
        name: "Architect Program".into(),
        source_format: ARCHITECT_PROGRAM_SCHEMA.into(),
        component_kind: "architect".into(),
        dimension: "data".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        schema: ARCHITECT_PROGRAM_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.xlsx".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.xlsx".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind

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
        knowledge: knowledge_child_from_records(&[]),
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

/// @emoji 🧪️ Sample program for tests with elements, stakeholders, and one adjacency.
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

    program
}
// #endregion

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.architect.program")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.architect.program")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.architect.program")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.architect.program.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.architect.program.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.architect.program@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.architect.program@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.zip")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.zip@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.zip@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.xlsx")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.xlsx@ecma-376/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.xlsx@ecma-376/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"architect.program:architect")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "architect.program")?)?
                .claim(ArtifactIdentityClaim::codec_extension("architect.program", "architect")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Architect")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Architect")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Architekt")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Architekt")?)?)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::program_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::program_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<editor::architect::ArchitectPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️kernel/🦀️.rs"]
                            pub mod kernel;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🦀️.rs"]
                            pub mod registers;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_information_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_information_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_information_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_information_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_sustainability_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_sustainability_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_sustainability_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_sustainability_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_accessibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_accessibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_accessibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_accessibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_conflict {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_conflict {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_conflict {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_conflict {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_option_evaluation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_option_evaluation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_option_evaluation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_option_evaluation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_function {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_function {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_function {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_function {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_risk {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_risk {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_risk {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_risk {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_decision {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_decision {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_decision {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_decision {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_validation_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_validation_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_validation_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_validation_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_priority_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_priority_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_priority_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_priority_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_flow_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_flow_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_flow_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_flow_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_environmental_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_environmental_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_environmental_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_environmental_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_workshop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_workshop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_workshop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_workshop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_scenario {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_scenario {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_scenario {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_scenario {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_benchmark_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_benchmark_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_benchmark_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_benchmark_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_activity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_activity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_activity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_activity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_infrastructure_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_infrastructure_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_infrastructure_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_infrastructure_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_governance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_governance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_organizational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_organizational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_organizational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_organizational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_meta {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_meta {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_issue {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_issue {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_issue {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_issue {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_approval_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_approval_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_approval_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_approval_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_stakeholder {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_stakeholder {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_stakeholder {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_stakeholder {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_quality_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_quality_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_quality_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_quality_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_resilience_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_resilience_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_resilience_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_resilience_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_assumption {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_assumption {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_assumption {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_assumption {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_cost_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_cost_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_cost_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_cost_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_project {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_project {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_document {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_document {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_document {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_document {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_schedule_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_schedule_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_schedule_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_schedule_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_growth_plan {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_growth_plan {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_growth_plan {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_growth_plan {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_performance_criterion {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_performance_criterion {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_performance_criterion {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_performance_criterion {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_operational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_operational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_operational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_operational_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_site_context {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_site_context {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_site_context {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_site_context {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_template_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_template_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_template_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_template_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_report_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_report_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_report_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_report_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_audit_event {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_audit_event {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_audit_event {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_audit_event {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_knowledge_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_knowledge_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_knowledge_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_knowledge_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_regulatory_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_regulatory_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_regulatory_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_regulatory_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_change_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_change_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_change_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_change_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_communication_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_communication_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_communication_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_communication_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_resource {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_resource {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_resource {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_resource {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_status_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_status_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_status_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_status_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_process {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_process {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_process {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_process {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_search_filter {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_search_filter {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_search_filter {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_search_filter {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_access_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_access_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_access_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_access_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_privacy_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_privacy_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_privacy_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_privacy_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_relationship {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_relationship {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_relationship {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_relationship {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_quantity_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_quantity_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_quantity_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_quantity_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_analysis_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_analysis_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_analysis_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_analysis_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_storage_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_storage_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_storage_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_storage_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_meeting_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_meeting_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_meeting_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_meeting_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_survey {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_survey {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_survey {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_survey {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod connect_adjacency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_delivery_constraint {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_delivery_constraint {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_delivery_constraint {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_delivery_constraint {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_constraint_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_constraint_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_constraint_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_constraint_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_compliance_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_compliance_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_compliance_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_compliance_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_service_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_service_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_service_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_service_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_security_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_security_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_security_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_security_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_collaboration_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_collaboration_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_collaboration_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_collaboration_record {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_safety_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_safety_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_safety_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_safety_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_user_profile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_user_profile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_user_profile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_user_profile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_human_factor_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_human_factor_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_human_factor_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_human_factor_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_flexibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_flexibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_flexibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_flexibility_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_wayfinding_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_wayfinding_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_wayfinding_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_wayfinding_requirement {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_program_element {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_program_element {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_program_element {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_program_element {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod connect_trace {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod disconnect_trace {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod disconnect_adjacency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {

            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub mod kernel {
            pub use crate::standards::v1::subsets::any::schema::kernel::*;
        }
        pub mod registers {
            pub use crate::standards::v1::subsets::any::schema::registers::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs"]
        pub mod catalog;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎨️chrome/🦀️.rs"]
        pub mod chrome;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️adjacency/🦀️.rs"]
            pub mod adjacency;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️analysis/🦀️.rs"]
            pub mod analysis;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️element/🦀️.rs"]
            pub mod element;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️exchange/🦀️.rs"]
            pub mod exchange;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️graph/🦀️.rs"]
            pub mod graph;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register/🦀️.rs"]
            pub mod register;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️search/🦀️.rs"]
            pub mod search;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️template/🦀️.rs"]
            pub mod template;
        }

        #[path = "."]
        pub mod modes {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📊️report/🦀️.rs"]
            pub mod report;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️review/🦀️.rs"]
            pub mod review;

            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️.rs"]
                    pub mod adjacency;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🦀️.rs"]
                    pub mod register;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🦀️.rs"]
                    pub mod report;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️.rs"]
                    pub mod trace;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️register/🦀️.rs"]
                    pub mod register;
                }
            }
        }
    }
}
