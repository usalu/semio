//! 🏛️ Architect play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the five window
//! surfaces in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, view state in
//! `🦀️config.rs`, presentation factories in `🦀️chrome.rs`, the register bridge in `🦀️catalog.rs`;
//! pure derived reads over the document live in the artifact's own `🧬️schema` / `🧬️schema/💡️inferences`
//! (see `//#region 🔧️Behavior` below for the app-scoped, `&mut`-taking counterpart).

use crate::apps::architect::catalog::{analysis_kind_picker_options, parse_entity_id, parse_entity_id_from_args, parse_register_id, report_kind_picker_options, REGISTER_IDS};
use crate::apps::architect::commands::adjacency::{set_adjacency_field, set_adjacency_filter, set_adjacency_kind};
use crate::apps::architect::commands::analysis::{run_analysis, run_report, run_validation};
use crate::apps::architect::commands::element::{add_element, remove_element};
use crate::apps::architect::commands::exchange::{export_program, export_registers_csv, import_program, import_program_request, import_registers_csv};
use crate::apps::architect::commands::graph::{node_graph_edit, node_graph_viewport};
use crate::apps::architect::commands::register::{add_register_item, patch_register_item, remove_register_item, select_register};
use crate::apps::architect::commands::search::query;
use crate::apps::architect::commands::selection::set_selection;
use crate::apps::architect::commands::template::apply;
use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
use crate::apps::architect::presence::{ArchitectPresence, ArchitectPresenceMutation};
use crate::apps::architect::modes::edit as edit_mode;
use crate::apps::architect::modes::edit::windows::{adjacency as adjacency_window, graph as graph_window, register as register_window, report as report_window, trace as trace_window};
use crate::apps::architect::modes::{report as report_mode, review as review_mode};
use crate::apps::architect::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::artifacts::program::op::ProgramMutation;
use crate::artifacts::program::{empty_plugin, sample_plugin, ProgramSnapshot, ARCHITECT_PROGRAM_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, UiNode};
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const ARCHITECT_APP_ID: &str = "architect";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🪟️windows/*`) builds its item/on-change actions with.
pub fn architect_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(ARCHITECT_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️ResetDocument
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (the whole-document
/// replace variant — see `📓️taxonomy.md`'s forbidden vocabulary), so import/exchange flows build a
/// `HostEffect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry —
/// same mechanism `✏️s/🔌️plugins/🗒️note`'s `reset_document_effect` already established.
pub fn reset_document_effect(document: &ProgramSnapshot) -> semio_framework_plugin::HostEffect {
    let pack = <ProgramSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<ProgramSnapshot, ProgramMutation>(ARCHITECT_PROGRAM_SCHEMA, ARCHITECT_APP_ID, document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("architect program document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::HostEffect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔌️Registration
/// 🗂️ Dissolved out of the former artifact-tree `⚙️engine` root (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — `register_architect_exports` is still
/// the live plugin setup entry (`Plugin::builder(...).setup(crate::register_architect_exports)` in
/// the plugin root `🦀️component.rs`, re-exported crate-root-wide by a `pub use` in `📦️glue.rs`'s
/// out-of-scope shim section), so unlike block2d's fully
/// superseded `register()` this one is NOT dead — it is app-scoped registration wiring (registers
/// this app's document codec, app schema, pilot languages, artifact schema/inference descriptors
/// and the io composer table), so it belongs here rather than on the artifact.
pub fn register() {
    crate::artifacts::program::io_registry::register();

    register_pilot_languages();
    register_artifact_schema();
    register_artifact_inference();
    crate::apps::architect::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<ArchitectPlayApp>(crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA);
}

/// 🗂️ Plugin setup entry — same as `register`, named for `Plugin::builder(...).setup(...)`.
pub fn register_architect_exports() {
    register();
}

/// 📎 Registers the program artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::program::schema::program_artifact_schema_descriptor());
}

/// 💡️ Registers the program artifact `💡️inference` descriptor into the OS-wide inference catalog
/// — sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inference() {
    ::schema::register_artifact_inference_descriptor(
        crate::artifacts::program::standards::v1::subsets::any::schema::inferences::program_artifact_inference_descriptor(),
    );
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program",
        extension: Some("architect"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::program::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("architect.program"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::program::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("architect.program.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::program::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("architect.program.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "program.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("program.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "program.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("program.spr"),
    });
}
//#endregion 🔌️Registration

//#region 🔧️Behavior
/// 🔧️ Dissolved out of the former artifact-tree `⚙️engine` topic files (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every function that takes `&mut
/// ProgramSnapshot` (constructs/mutates) rather than merely reading it. `🎛️apps/🏛️architect/⚙️engine`
/// exists on disk as the reserved machine-slot stub, but `📦️glue.rs` (out of scope for this ticket)
/// never mounts it into the module tree — only two `pub mod engine { ... }` shims exist there and
/// both live in the ARTIFACT-tree shim section, not the Apps section — so populating that directory
/// would produce dead, uncompiled code. Per this packet's own fallback this behavior lands directly
/// on the app top-level instead; the reserved directory is left empty and untouched. This does NOT
/// invent a new state machine: every item below is a plain function/struct, unchanged in shape from
/// its former engine-topic file, just relocated.
pub mod behavior {
    use crate::artifacts::program::kernel::{EntityHeader, EntityId, PluginError, TextField, TraceKind, TraceLink};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::registers::{
        Activity, Adjacency, AdjacencyKind, AnalysisKind, AnalysisRecord, ConnectionKind, Equipment, Function, FunctionKind, Process, ProgramElement, ProgramElementKind, Relationship, RelationshipKind, ReportKind, ReportRecord,
        Requirement, RequirementKind, Risk, RiskLevel, Stakeholder, TemplateRecord, UserCategory, UserProfile, ValidationStatus,
    };
    use crate::artifacts::program::ProgramSnapshot;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::{
        build_report, run_analysis, RegisterCsvRow,
    };
    use crate::artifacts::program::standards::v1::subsets::any::schema::normalize_pair;
    use semio_s_plugin_stdio::artifacts::csv as stdio_csv;
    use semio_s_plugin_stdio::artifacts::tsv as stdio_tsv;
    use semio_s_plugin_stdio::artifacts::tsv::standards::iana::engine as stdio_tsv_engine;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    /// 🔀 Absolute path to the `🧬️mutations` facet's per-register leaves, kept as one alias so the
    /// semantic-mutations-overhaul rename (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL`)
    /// only needs updating here if `📦️glue.rs`'s directory wiring ever changes.
    use crate::artifacts::program::schema::mutations as leaves;

    //#region ↔️AdjacencyMutations
    /// ➕️ Upserts an adjacency row with normalized endpoints; replaces same pair if present.
    pub fn set_adjacency(program: &mut ProgramSnapshot, mut adjacency: Adjacency) {
        let (a, b) = normalize_pair(&adjacency.element_a_id, &adjacency.element_b_id);
        adjacency.element_a_id = a;
        adjacency.element_b_id = b;
        adjacency.normalized = true;
        if let Some(existing) = program.adjacencies.iter().position(|row| row.element_a_id == adjacency.element_a_id && row.element_b_id == adjacency.element_b_id) {
            program.adjacencies[existing] = adjacency;
        } else {
            program.adjacencies.push(adjacency);
        }
    }

    /// ➖️ Removes an adjacency by id or by normalized element pair.
    pub fn clear_adjacency(program: &mut ProgramSnapshot, id: &EntityId) {
        if let Some(index) = program.adjacencies.iter().position(|row| &row.header.id == id) {
            program.adjacencies.remove(index);
            return;
        }
        program.adjacencies.retain(|row| &row.element_a_id != id && &row.element_b_id != id);
    }
    //#endregion ↔️AdjacencyMutations

    //#region 📐️Template
    /// 📋️ Result of applying a template to a program.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TemplateApplyResult {
        pub template_id: EntityId,
        pub created_entity_ids: Vec<EntityId>,
        pub messages: Vec<String>,
    }

    /// 🧩️ Applies a template record and returns replayable `ProgramMutation`s.
    pub fn apply_template(program: &mut ProgramSnapshot, template: &TemplateRecord) -> Vec<ProgramMutation> {
        let mut operations = Vec::new();
        let mut element_ids = Vec::new();
        for field in &template.default_fields {
            let id = EntityId::new_serial("template-entity", "template-entity");
            match field.as_str() {
                "stakeholder" => {
                    let item = Stakeholder {
                        header: EntityHeader::new(id.clone(), format!("{} Stakeholder", template.header.name)),
                        role: "Template".into(),
                        organization: template.source_organization.clone().unwrap_or_default(),
                        department: None,
                        contact_email: None,
                        contact_phone: None,
                        influence: crate::artifacts::program::registers::InfluenceLevel::Medium,
                        interest: crate::artifacts::program::registers::InfluenceLevel::Medium,
                        engagement: crate::artifacts::program::registers::EngagementLevel::Neutral,
                        expectations: template.checklists.clone(),
                        concerns: Vec::new(),
                        requirement_ids: Vec::new(),
                        decision_authority: false,
                        communication_preferences: Vec::new(),
                        reporting_frequency: None,
                        involvement_phases: Vec::new(),
                        availability: None,
                        representative_of: None,
                        delegated_to: None,
                        relationship_to_client: None,
                        power_interest_notes: Vec::new(),
                        stakeholder_type: "Template".into(),
                        influence_strategy: None,
                        communication_channels: Vec::new(),
                        success_metrics: Vec::new(),
                    };
                    operations.push(ProgramMutation::CreateStakeholder(leaves::create_stakeholder::mutation::CreateStakeholder { stakeholder: item.clone() }));
                    program.stakeholders.push(item);
                }
                "user" => {
                    let item = UserProfile {
                        header: EntityHeader::new(id.clone(), format!("{} User", template.header.name)),
                        category: UserCategory::Primary,
                        demographic: None,
                        age_range: None,
                        abilities: Vec::new(),
                        disabilities: Vec::new(),
                        occupation: None,
                        role_title: None,
                        department: None,
                        mobility_profile: Vec::new(),
                        sensory_profile: Vec::new(),
                        cognitive_profile: Vec::new(),
                        behavioral_patterns: Vec::new(),
                        usage_frequency: None,
                        usage_duration: None,
                        peak_usage_times: Vec::new(),
                        technology_proficiency: None,
                        preferences: Vec::new(),
                        pain_points: Vec::new(),
                        goals: template.checklists.clone(),
                        activity_ids: Vec::new(),
                        research_method: None,
                        persona_archetype: None,
                        validated: false,
                        stakeholder_ids: Vec::new(),
                    };
                    operations.push(ProgramMutation::CreateUserProfile(leaves::create_user_profile::mutation::CreateUserProfile { user_profile: item.clone() }));
                    program.users.push(item);
                }
                "activity" => {
                    let item = Activity {
                        header: EntityHeader::new(id.clone(), format!("{} Activity", template.header.name)),
                        code: "ACT".into(),
                        category: template.sector.clone().unwrap_or_else(|| "general".into()),
                        frequency: None,
                        duration: None,
                        intensity: None,
                        participants: crate::artifacts::program::kernel::QuantitySpec::default(),
                        equipment_ids: Vec::new(),
                        space_requirements: Vec::new(),
                        environmental_needs: Vec::new(),
                        privacy_needs: Vec::new(),
                        accessibility_needs: Vec::new(),
                        adjacent_activities: Vec::new(),
                        sequencing: Vec::new(),
                        peak_periods: Vec::new(),
                        workflow_steps: template.checklists.clone(),
                        inputs: Vec::new(),
                        outputs: Vec::new(),
                        user_profile_ids: Vec::new(),
                        function_ids: Vec::new(),
                        performance_indicators: Vec::new(),
                        activity_type: "template".into(),
                        location_context: None,
                        temporal_pattern: None,
                        supervision_level: None,
                    };
                    operations.push(ProgramMutation::CreateActivity(leaves::create_activity::mutation::CreateActivity { activity: item.clone() }));
                    program.activities.push(item);
                }
                "function" => {
                    let item = Function {
                        header: EntityHeader::new(id.clone(), format!("{} Function", template.header.name)),
                        code: "FN".into(),
                        kind: FunctionKind::Primary,
                        purpose: TextField::plain(template.standards.join(", ")),
                        criticality: crate::artifacts::program::kernel::Priority::Preferred,
                        performance_targets: Vec::new(),
                        service_level: None,
                        operating_hours: None,
                        staffing: crate::artifacts::program::kernel::QuantitySpec::default(),
                        equipment_ids: Vec::new(),
                        resource_ids: Vec::new(),
                        activity_ids: Vec::new(),
                        element_ids: Vec::new(),
                        dependencies: Vec::new(),
                        interfaces: Vec::new(),
                        constraints: Vec::new(),
                        quality_criteria: Vec::new(),
                        regulatory_refs: template.standards.clone(),
                        future_changes: Vec::new(),
                        owner_stakeholder_id: None,
                        success_metrics: Vec::new(),
                        hierarchy_parent_id: None,
                        conflict_ids: Vec::new(),
                    };
                    operations.push(ProgramMutation::CreateFunction(leaves::create_function::mutation::CreateFunction { function: item.clone() }));
                    program.functions.push(item);
                }
                "element" | "room" => {
                    let item = ProgramElement {
                        header: EntityHeader::new(id.clone(), format!("{} Space", template.header.name)),
                        code: "TPL".into(),
                        kind: ProgramElementKind::Room,
                        parent_id: None,
                        level: None,
                        area: crate::artifacts::program::kernel::QuantitySpec::default(),
                        volume: crate::artifacts::program::kernel::QuantitySpec::default(),
                        height: crate::artifacts::program::kernel::QuantitySpec::default(),
                        occupancy: crate::artifacts::program::kernel::QuantitySpec::default(),
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
                    };
                    operations.push(ProgramMutation::CreateProgramElement(leaves::create_program_element::mutation::CreateProgramElement { program_element: item.clone() }));
                    program.elements.push(item);
                    element_ids.push(id.clone());
                }
                "requirement" => {
                    let item = Requirement {
                        header: EntityHeader::new(id.clone(), format!("{} Requirement", template.header.name)),
                        code: String::new(),
                        kind: RequirementKind::Functional,
                        statement: TextField::plain(template.standards.join(", ")),
                        rationale: None,
                        source: template.source_organization.clone(),
                        stakeholder_ids: Vec::new(),
                        element_ids: Vec::new(),
                        function_ids: Vec::new(),
                        parent_requirement_id: None,
                        child_requirement_ids: Vec::new(),
                        acceptance_criteria: template.checklists.clone(),
                        verification_method: None,
                        validation_status: ValidationStatus::Pending,
                        conflict_ids: Vec::new(),
                        risk_ids: Vec::new(),
                        cost_estimate: None,
                        schedule_constraint: None,
                        regulatory_refs: template.standards.clone(),
                        trace_links: Vec::new(),
                        superseded_by: None,
                    };
                    operations.push(ProgramMutation::CreateRequirement(leaves::create_requirement::mutation::CreateRequirement { requirement: item.clone() }));
                    program.requirements.push(item);
                }
                "risk" => {
                    let item = Risk {
                        header: EntityHeader::new(id.clone(), format!("{} Risk", template.header.name)),
                        risk_statement: TextField::plain(template.checklists.join("; ")),
                        category: "template".into(),
                        probability: RiskLevel::Medium,
                        impact: RiskLevel::Medium,
                        risk_score: None,
                        causes: Vec::new(),
                        effects: Vec::new(),
                        affected_element_ids: Vec::new(),
                        affected_requirement_ids: Vec::new(),
                        mitigation: Vec::new(),
                        contingency: Vec::new(),
                        owner_id: None,
                        review_date: None,
                        trigger_indicators: Vec::new(),
                        residual_probability: None,
                        residual_impact: None,
                        related_conflict_ids: Vec::new(),
                        escalation_path: Vec::new(),
                        monitoring_plan: None,
                    };
                    operations.push(ProgramMutation::CreateRisk(leaves::create_risk::mutation::CreateRisk { risk: item.clone() }));
                    program.risks.push(item);
                }
                "process" => {
                    let item = Process {
                        header: EntityHeader::new(id.clone(), format!("{} Process", template.header.name)),
                        code: "PRC".into(),
                        category: template.sector.clone().unwrap_or_else(|| "general".into()),
                        trigger: None,
                        inputs: Vec::new(),
                        outputs: Vec::new(),
                        steps: template.checklists.clone(),
                        actors: Vec::new(),
                        equipment_ids: Vec::new(),
                        element_ids: Vec::new(),
                        duration: None,
                        frequency: None,
                        critical_path: false,
                        bottlenecks: Vec::new(),
                        dependencies: Vec::new(),
                        kpis: Vec::new(),
                        automation_level: None,
                        failure_modes: Vec::new(),
                        improvement_opportunities: Vec::new(),
                        regulatory_refs: template.standards.clone(),
                        owner_id: None,
                        workflow_type: Some("template".into()),
                        handoff_points: Vec::new(),
                        quality_gates: Vec::new(),
                    };
                    operations.push(ProgramMutation::CreateProcess(leaves::create_process::mutation::CreateProcess { process: item.clone() }));
                    program.processes.push(item);
                }
                "equipment" => {
                    let item = Equipment {
                        header: EntityHeader::new(id.clone(), format!("{} Equipment", template.header.name)),
                        code: "EQ".into(),
                        category: template.sector.clone().unwrap_or_else(|| "general".into()),
                        manufacturer: None,
                        model: None,
                        quantity: crate::artifacts::program::kernel::QuantitySpec::default(),
                        dimensions: None,
                        weight_kg: None,
                        power_kw: None,
                        utility_connections: Vec::new(),
                        ventilation: None,
                        noise_level_db: None,
                        clearance: None,
                        mounting: None,
                        element_ids: Vec::new(),
                        activity_ids: Vec::new(),
                        maintenance_access: Vec::new(),
                        lifecycle_years: None,
                        replacement_cost: None,
                        standards: template.standards.clone(),
                        supplier: None,
                        activity_link_ids: Vec::new(),
                        installation_requirements: Vec::new(),
                        commissioning_notes: Vec::new(),
                        spare_parts: Vec::new(),
                    };
                    operations.push(ProgramMutation::CreateEquipment(leaves::create_equipment::mutation::CreateEquipment { equipment: item.clone() }));
                    program.equipment.push(item);
                }
                "adjacency" | "adjacency_bundle" if element_ids.len() >= 2 => {
                    let (a, b) = normalize_pair(&element_ids[0], &element_ids[1]);
                    let adjacency = Adjacency {
                        header: EntityHeader::new(id.clone(), format!("{} Adjacency", template.header.name)),
                        element_a_id: a,
                        element_b_id: b,
                        kind: AdjacencyKind::Preferred,
                        connection: ConnectionKind::Direct,
                        separations: Vec::new(),
                        weight: 1.0,
                        rationale: Some(TextField::plain("template bundle")),
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
                    };
                    operations.push(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::mutation::ConnectAdjacency { adjacency: adjacency.clone() }));
                    set_adjacency(program, adjacency);
                }
                _ => {}
            }
        }
        if let Some(existing) = program.templates.iter_mut().find(|t| t.header.id == template.header.id) {
            existing.usage_count += 1;
            existing.last_applied = Some(program.meta.timestamps.updated.clone());
        }
        operations
    }
    //#endregion 📐️Template

    //#region 📄️ReportRecord
    /// 📝️ Builds a report and appends a `ReportRecord` to the program.
    pub fn build_report_and_record(program: &mut ProgramSnapshot, kind: ReportKind) -> crate::artifacts::program::standards::v1::subsets::any::schema::inferences::ProgramReport {
        let report = build_report(program, kind);
        let record = ReportRecord {
            header: EntityHeader::new(EntityId::new_serial("report", "report"), report.title.clone()),
            kind,
            title: report.title.clone(),
            audience: Vec::new(),
            sections: report.sections.iter().map(|s| s.heading.clone()).collect(),
            generated_at: Some(report.generated_at.clone()),
            generated_by: None,
            analysis_ids: Vec::new(),
            format: Some("structured".into()),
            file_ref: None,
            distribution_list: Vec::new(),
            approval_status: ValidationStatus::Pending,
            approver_id: None,
            version: program.meta.revision.clone(),
            template_id: None,
            parameters: Vec::new(),
            confidentiality: None,
            expiry_date: None,
            related_decision_ids: Vec::new(),
        };
        program.reports.push(record);
        report
    }
    //#endregion 📄️ReportRecord

    //#region 🔬️AnalysisRecord
    /// 📝️ Runs analysis and appends an `AnalysisRecord` to the program.
    pub fn run_analysis_and_record(program: &mut ProgramSnapshot, kind: AnalysisKind) -> crate::artifacts::program::standards::v1::subsets::any::schema::inferences::AnalysisResult {
        let result = run_analysis(program, kind);
        let record = AnalysisRecord {
            header: EntityHeader::new(EntityId::new_serial("analysis", "analysis"), result.title.clone()),
            kind,
            title: result.title.clone(),
            parameters: Vec::new(),
            input_entity_ids: result.entity_ids.clone(),
            output_summary: TextField::plain(&result.summary),
            findings: result.findings.clone(),
            metrics: result.metrics.iter().map(|m| format!("{}={}{}", m.name, m.value, m.unit.as_deref().unwrap_or(""))).collect(),
            charts: Vec::new(),
            run_by: None,
            run_at: Some(program.meta.timestamps.updated.clone()),
            duration_ms: None,
            tool_version: None,
            scenario_id: None,
            report_id: None,
            confidence: None,
            limitations: Vec::new(),
            recommendations: result.findings.clone(),
            raw_result_ref: None,
        };
        program.analyses.push(record);
        result
    }
    //#endregion 🔬️AnalysisRecord

    //#region 📤️ExchangeImport
    /// 🔀️ Strategy for merging imported register rows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum MergeStrategy {
        Replace,
        SkipDuplicates,
        Upsert,
    }

    fn csv_snapshot_to_rows(snapshot: &stdio_csv::CsvSnapshot) -> Result<Vec<RegisterCsvRow>, PluginError> {
        let mut records = snapshot.records.iter();
        let header = records.next().ok_or_else(|| PluginError::Csv("empty delimited file".into()))?;
        let header_values: Vec<&str> = header.fields.iter().map(|f| f.value.as_str()).collect();
        if header_values != ["register", "id", "name", "status", "priority", "tags", "source"] {
            return Err(PluginError::Csv(format!("unexpected header: {}", header_values.join(","))));
        }
        let mut rows = Vec::new();
        for record in records {
            if record.fields.len() == 1 && record.fields[0].value.trim().is_empty() {
                continue;
            }
            let values: Vec<String> = record.fields.iter().map(|f| f.value.clone()).collect();
            rows.push(RegisterCsvRow::from_columns(&values)?);
        }
        Ok(rows)
    }

    /// 📥️ Decodes CSV via stdio's real RFC 4180 codec, then merges rows into matching
    /// register collections via `MergeStrategy`.
    pub fn import_registers_csv(program: &mut ProgramSnapshot, csv: &str, strategy: MergeStrategy) -> Result<Vec<EntityId>, PluginError> {
        let snapshot = stdio_csv::schema::snapshot::decode_csv_with(csv, true);
        import_rows(program, csv_snapshot_to_rows(&snapshot)?, strategy)
    }

    fn tsv_snapshot_to_rows(snapshot: &stdio_tsv::TsvSnapshot) -> Result<Vec<RegisterCsvRow>, PluginError> {
        let mut records = snapshot.records.iter();
        let header = records.next().ok_or_else(|| PluginError::Csv("empty delimited file".into()))?;
        let header_values: Vec<&str> = header.iter().map(|s| s.as_str()).collect();
        if header_values != ["register", "id", "name", "status", "priority", "tags", "source"] {
            return Err(PluginError::Csv(format!("unexpected header: {}", header.join("\t"))));
        }
        let mut rows = Vec::new();
        for record in records {
            if record.len() == 1 && record[0].trim().is_empty() {
                continue;
            }
            rows.push(RegisterCsvRow::from_columns(record)?);
        }
        Ok(rows)
    }

    /// 📥️ Decodes TSV via stdio's real IANA TSV codec, then merges rows into matching
    /// register collections via `MergeStrategy`.
    pub fn import_registers_tsv(program: &mut ProgramSnapshot, tsv: &str, strategy: MergeStrategy) -> Result<Vec<EntityId>, PluginError> {
        let snapshot = stdio_tsv_engine::decode_tsv(tsv);
        import_rows(program, tsv_snapshot_to_rows(&snapshot)?, strategy)
    }

    /// 🔀️ Applies `MergeStrategy` upsert semantics to already-decoded rows — shared by the
    /// CSV and TSV import paths, the decode step itself lives entirely in stdio's real codecs.
    fn import_rows(program: &mut ProgramSnapshot, rows: Vec<RegisterCsvRow>, strategy: MergeStrategy) -> Result<Vec<EntityId>, PluginError> {
        let mut touched = Vec::new();
        let mut seen: HashSet<(String, EntityId)> = HashSet::new();
        for row in rows {
            let key = (row.register.clone(), row.id.clone());
            if !seen.insert(key.clone()) {
                return Err(PluginError::Csv(format!("duplicate import id {} in register {}", row.id, row.register)));
            }
            if strategy == MergeStrategy::SkipDuplicates && register_contains(program, &row.register, &row.id) {
                continue;
            }
            if strategy == MergeStrategy::Replace {
                remove_register_item(program, &row.register, &row.id);
            }
            upsert_register_row(program, row.clone())?;
            touched.push(row.id);
        }
        Ok(touched)
    }

    fn register_contains(program: &ProgramSnapshot, register: &str, id: &EntityId) -> bool {
        match register {
            "elements" => program.elements.iter().any(|e| &e.header.id == id),
            "stakeholders" => program.stakeholders.iter().any(|s| &s.header.id == id),
            "requirements" => program.requirements.iter().any(|r| &r.header.id == id),
            "relationships" => program.relationships.iter().any(|r| &r.header.id == id),
            "adjacencies" => program.adjacencies.iter().any(|a| &a.header.id == id),
            _ => false,
        }
    }

    fn remove_register_item(program: &mut ProgramSnapshot, register: &str, id: &EntityId) {
        match register {
            "elements" => program.elements.retain(|e| &e.header.id != id),
            "stakeholders" => program.stakeholders.retain(|s| &s.header.id != id),
            "requirements" => program.requirements.retain(|r| &r.header.id != id),
            "relationships" => program.relationships.retain(|r| &r.header.id != id),
            "adjacencies" => program.adjacencies.retain(|a| &a.header.id != id),
            _ => {}
        }
    }

    fn upsert_register_row(program: &mut ProgramSnapshot, row: RegisterCsvRow) -> Result<(), PluginError> {
        match row.register.as_str() {
            "elements" => upsert_element(program, row),
            "stakeholders" => upsert_stakeholder(program, row),
            "requirements" => upsert_requirement(program, row),
            "relationships" => upsert_relationship_stub(program, row),
            "adjacencies" => upsert_adjacency_stub(program, row),
            other => {
                return Err(PluginError::Csv(format!("unsupported register import: {other}")));
            }
        }
        Ok(())
    }

    fn upsert_element(program: &mut ProgramSnapshot, row: RegisterCsvRow) {
        if let Some(element) = program.elements.iter_mut().find(|e| e.header.id == row.id) {
            element.header.name = row.name;
            return;
        }
        program.elements.push(ProgramElement {
            header: EntityHeader::new(row.id, row.name),
            code: String::new(),
            kind: ProgramElementKind::Room,
            parent_id: None,
            level: None,
            area: crate::artifacts::program::kernel::QuantitySpec::default(),
            volume: crate::artifacts::program::kernel::QuantitySpec::default(),
            height: crate::artifacts::program::kernel::QuantitySpec::default(),
            occupancy: crate::artifacts::program::kernel::QuantitySpec::default(),
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
    }

    fn upsert_stakeholder(program: &mut ProgramSnapshot, row: RegisterCsvRow) {
        if let Some(stakeholder) = program.stakeholders.iter_mut().find(|s| s.header.id == row.id) {
            stakeholder.header.name = row.name;
            return;
        }
        program.stakeholders.push(Stakeholder {
            header: EntityHeader::new(row.id, row.name),
            role: String::new(),
            organization: String::new(),
            department: None,
            contact_email: None,
            contact_phone: None,
            influence: crate::artifacts::program::registers::InfluenceLevel::Medium,
            interest: crate::artifacts::program::registers::InfluenceLevel::Medium,
            engagement: crate::artifacts::program::registers::EngagementLevel::Neutral,
            expectations: Vec::new(),
            concerns: Vec::new(),
            requirement_ids: Vec::new(),
            decision_authority: false,
            communication_preferences: Vec::new(),
            reporting_frequency: None,
            involvement_phases: Vec::new(),
            availability: None,
            representative_of: None,
            delegated_to: None,
            relationship_to_client: None,
            power_interest_notes: Vec::new(),
            stakeholder_type: String::new(),
            influence_strategy: None,
            communication_channels: Vec::new(),
            success_metrics: Vec::new(),
        });
    }

    fn upsert_requirement(program: &mut ProgramSnapshot, row: RegisterCsvRow) {
        if let Some(requirement) = program.requirements.iter_mut().find(|r| r.header.id == row.id) {
            requirement.header.name = row.name;
            if !row.source.is_empty() {
                requirement.source = Some(row.source);
            }
            return;
        }
        program.requirements.push(Requirement {
            header: EntityHeader::new(row.id, row.name),
            code: String::new(),
            kind: RequirementKind::Functional,
            statement: TextField::plain(""),
            rationale: None,
            source: if row.source.is_empty() { None } else { Some(row.source) },
            stakeholder_ids: Vec::new(),
            element_ids: Vec::new(),
            function_ids: Vec::new(),
            parent_requirement_id: None,
            child_requirement_ids: Vec::new(),
            acceptance_criteria: Vec::new(),
            verification_method: None,
            validation_status: ValidationStatus::Pending,
            conflict_ids: Vec::new(),
            risk_ids: Vec::new(),
            cost_estimate: None,
            schedule_constraint: None,
            regulatory_refs: Vec::new(),
            trace_links: Vec::new(),
            superseded_by: None,
        });
    }

    fn upsert_relationship_stub(program: &mut ProgramSnapshot, row: RegisterCsvRow) {
        if program.relationships.iter().any(|r| r.header.id == row.id) {
            return;
        }
        let fallback = program.elements.first().map_or_else(|| EntityId::new_serial("element", "element"), |e| e.header.id.clone());
        program.relationships.push(Relationship {
            header: EntityHeader::new(row.id, row.name),
            source_id: fallback.clone(),
            target_id: fallback,
            kind: RelationshipKind::AdjacentTo,
            strength: None,
            directional: true,
            rationale: None,
            constraints: Vec::new(),
            conditions: Vec::new(),
            relationship_priority: crate::artifacts::program::kernel::Priority::Preferred,
            valid_from: None,
            valid_until: None,
            evidence: Vec::new(),
            conflict_ids: Vec::new(),
            trace_links: Vec::new(),
            bidirectional: false,
            distance_constraint_m: None,
            capacity_constraint: None,
            regulatory_basis: Vec::new(),
            review_cycle: None,
            owner_id: None,
            proximity_requirement: None,
            compatibility_requirement: None,
            incompatibility_requirement: None,
            separation_requirements: Vec::new(),
        });
    }

    fn upsert_adjacency_stub(program: &mut ProgramSnapshot, row: RegisterCsvRow) {
        if program.adjacencies.iter().any(|a| a.header.id == row.id) {
            return;
        }
        let a = program.elements.first().map_or_else(|| EntityId::new_serial("element", "element"), |e| e.header.id.clone());
        let b = program.elements.get(1).map_or_else(|| a.clone(), |e| e.header.id.clone());
        let (left, right) = normalize_pair(&a, &b);
        program.adjacencies.push(Adjacency {
            header: EntityHeader::new(row.id, row.name),
            element_a_id: left,
            element_b_id: right,
            kind: AdjacencyKind::Preferred,
            connection: ConnectionKind::Direct,
            separations: Vec::new(),
            weight: 1.0,
            rationale: None,
            distance_max_m: None,
            distance_min_m: None,
            level_constraint: None,
            access_path: None,
            shared_wall: false,
            shared_entry: false,
            traffic_isolation: false,
            circulation_overlap: false,
            conflict_ids: Vec::new(),
            normalized: true,
            verification_status: ValidationStatus::Pending,
            source_relationship_id: None,
            internal_external_access: None,
        });
    }
    //#endregion 📤️ExchangeImport

    //#region 🧭️Trace
    /// ⛓️ Ordered chain of trace links from a root entity.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraceChain {
        pub root_id: EntityId,
        pub links: Vec<TraceLink>,
    }

    /// 💥️ Reverse impact set from trace links pointing at an entity.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ImpactTrace {
        pub target_id: EntityId,
        pub upstream_ids: Vec<EntityId>,
        pub links: Vec<TraceLink>,
    }

    /// 🔗️ Builds a forward trace chain from `root_id` following kind-appropriate links.
    pub fn trace_chain(program: &mut ProgramSnapshot, root_id: &EntityId) -> TraceChain {
        embed_requirement_traces(program);
        let adjacency = trace_adjacency(&program.traces);
        let mut visited = HashSet::new();
        let mut links = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root_id.clone());
        visited.insert(root_id.clone());
        while let Some(current) = queue.pop_front() {
            if let Some(outgoing) = adjacency.get(&current) {
                for link in outgoing {
                    if !follows_kind_chain(&link.kind) {
                        continue;
                    }
                    links.push(link.clone());
                    if visited.insert(link.to_id.clone()) {
                        queue.push_back(link.to_id.clone());
                    }
                }
            }
        }
        TraceChain { root_id: root_id.clone(), links }
    }

    /// 🔍️ Finds trace links touching `entity_id` (from or to).
    pub fn trace_links_for(program: &mut ProgramSnapshot, entity_id: &EntityId) -> Vec<TraceLink> {
        embed_requirement_traces(program);
        program.traces.iter().filter(|link| &link.from_id == entity_id || &link.to_id == entity_id).cloned().collect()
    }

    /// ↩️ Reverse impact trace — entities that depend on or satisfy `target_id`.
    pub fn trace_impact(program: &mut ProgramSnapshot, target_id: &EntityId) -> ImpactTrace {
        embed_requirement_traces(program);
        let mut upstream = HashSet::new();
        let mut links = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(target_id.clone());
        while let Some(current) = queue.pop_front() {
            for link in &program.traces {
                if link.to_id != current {
                    continue;
                }
                if matches!(link.kind, TraceKind::ObjectiveToRequirement | TraceKind::StakeholderToRequirement | TraceKind::FunctionToProgramElement | TraceKind::RequirementToDecision | TraceKind::RequirementToRisk | TraceKind::ConstraintToImpact) {
                    links.push(link.clone());
                    if upstream.insert(link.from_id.clone()) {
                        queue.push_back(link.from_id.clone());
                    }
                }
            }
        }
        ImpactTrace { target_id: target_id.clone(), upstream_ids: upstream.into_iter().collect(), links }
    }

    /// ➕️ Appends a trace link to the plugin trace register.
    pub fn add_trace_link(program: &mut ProgramSnapshot, from_id: EntityId, to_id: EntityId, kind: TraceKind) {
        program.traces.push(TraceLink::new(from_id, to_id, kind));
    }

    /// 🧷️ Copies requirement-embedded trace links into the plugin trace register.
    fn embed_requirement_traces(program: &mut ProgramSnapshot) {
        for requirement in &program.requirements {
            for link in &requirement.trace_links {
                if program.traces.iter().any(|t| t.id == link.id) {
                    continue;
                }
                program.traces.push(link.clone());
            }
        }
    }

    fn follows_kind_chain(kind: &TraceKind) -> bool {
        !matches!(kind, TraceKind::FullAuditTrail)
    }

    fn trace_adjacency(traces: &[TraceLink]) -> HashMap<EntityId, Vec<TraceLink>> {
        let mut map: HashMap<EntityId, Vec<TraceLink>> = HashMap::new();
        for link in traces {
            map.entry(link.from_id.clone()).or_default().push(link.clone());
        }
        map
    }
    //#endregion 🧭️Trace

    #[cfg(test)]
    //#region 🧪️BehaviorTests
    mod tests {
        use super::*;
        use crate::artifacts::program::sample_plugin;
        use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::{export_registers_csv, export_registers_tsv};

        #[test]
        fn apply_template_returns_plugin_operations() {
            let mut program = crate::artifacts::program::empty_plugin();
            let template = TemplateRecord {
                header: EntityHeader::new(EntityId::new_serial("template", "Clinic Starter"), "Clinic Starter"),
                template_type: "sector".into(),
                sector: Some("healthcare".into()),
                project_type: None,
                version: "1".into(),
                content_ref: None,
                entity_kinds: vec!["stakeholder".into(), "element".into()],
                default_fields: vec!["stakeholder".into(), "room".into(), "requirement".into()],
                checklists: vec!["intake".into()],
                standards: vec!["ISO-41001".into()],
                applicability: Vec::new(),
                author_id: None,
                approval_status: ValidationStatus::Passed,
                usage_count: 0,
                last_applied: None,
                customization_notes: Vec::new(),
                related_knowledge_ids: Vec::new(),
                benchmark_ids: Vec::new(),
                license: None,
                source_organization: Some("Semio".into()),
            };
            let operations = apply_template(&mut program, &template);
            assert!(!operations.is_empty());
            assert_eq!(program.stakeholders.len(), 1);
            assert_eq!(program.elements.len(), 1);
            assert_eq!(program.requirements.len(), 1);
        }

        #[test]
        fn template_ops_replay_on_empty_plugin() {
            let mut source = crate::artifacts::program::empty_plugin();
            let template = TemplateRecord {
                header: EntityHeader::new(EntityId::new_serial("template", "Replay"), "Replay"),
                template_type: "sector".into(),
                sector: None,
                project_type: None,
                version: "1".into(),
                content_ref: None,
                entity_kinds: vec!["function".into()],
                default_fields: vec!["function".into()],
                checklists: Vec::new(),
                standards: Vec::new(),
                applicability: Vec::new(),
                author_id: None,
                approval_status: ValidationStatus::Passed,
                usage_count: 0,
                last_applied: None,
                customization_notes: Vec::new(),
                related_knowledge_ids: Vec::new(),
                benchmark_ids: Vec::new(),
                license: None,
                source_organization: None,
            };
            let operations = apply_template(&mut source, &template);
            let mut target = crate::artifacts::program::empty_plugin();
            for operation in &operations {
                use protocol::{Mutation, MutationDiff};
                target = operation.diff(&target).apply(&target);
            }
            assert_eq!(target.functions.len(), 1);
        }

        #[test]
        fn build_report_and_record_persists() {
            let mut program = sample_plugin();
            let before = program.reports.len();
            build_report_and_record(&mut program, ReportKind::AdjacencyMatrix);
            assert_eq!(program.reports.len(), before + 1);
        }

        #[test]
        fn run_analysis_and_record_persists() {
            let mut program = sample_plugin();
            let before = program.analyses.len();
            run_analysis_and_record(&mut program, AnalysisKind::Risk);
            assert_eq!(program.analyses.len(), before + 1);
        }

        #[test]
        fn csv_round_trip_preserves_element_names() {
            let program = sample_plugin();
            let csv = export_registers_csv(&program).expect("csv export");
            let mut reloaded = crate::artifacts::program::empty_plugin();
            import_registers_csv(&mut reloaded, &csv, MergeStrategy::Upsert).expect("csv import");
            assert_eq!(reloaded.elements.len(), program.elements.len());
        }

        #[test]
        fn quoted_csv_parses_commas_in_name() {
            let csv = "register,id,name,status,priority,tags,source\nelements,e1,\"Room, A\",Draft,Preferred,,src\n";
            let snapshot = stdio_csv::schema::snapshot::decode_csv_with(csv, true);
            let rows = csv_snapshot_to_rows(&snapshot).expect("parse");
            assert_eq!(rows[0].name, "Room, A");
            assert_eq!(rows[0].source, "src");
        }

        #[test]
        fn duplicate_import_is_rejected() {
            let csv = "register,id,name,status,priority,tags,source\nelements,e1,A,Draft,Preferred,,\nelements,e1,B,Draft,Preferred,,\n";
            let mut program = crate::artifacts::program::empty_plugin();
            assert!(import_registers_csv(&mut program, csv, MergeStrategy::Upsert).is_err());
        }

        #[test]
        fn tsv_round_trip_preserves_element_names() {
            let program = sample_plugin();
            let tsv = export_registers_tsv(&program).expect("tsv export");
            let mut reloaded = crate::artifacts::program::empty_plugin();
            import_registers_tsv(&mut reloaded, &tsv, MergeStrategy::Upsert).expect("tsv import");
            assert_eq!(reloaded.elements.len(), program.elements.len());
        }

        #[test]
        fn trace_chain_follows_links() {
            let mut program = sample_plugin();
            let a = program.elements[0].header.id.clone();
            let b = program.elements[1].header.id.clone();
            add_trace_link(&mut program, a.clone(), b.clone(), TraceKind::FunctionToProgramElement);
            let chain = trace_chain(&mut program, &a);
            assert_eq!(chain.links.len(), 1);
            assert_eq!(chain.links[0].to_id, b);
        }

        #[test]
        fn trace_impact_collects_upstream() {
            let mut program = sample_plugin();
            let req_id = EntityId::new_serial("requirement", "requirement");
            let elem_id = program.elements[0].header.id.clone();
            add_trace_link(&mut program, req_id.clone(), elem_id.clone(), TraceKind::ObjectiveToRequirement);
            let impact = trace_impact(&mut program, &elem_id);
            assert!(impact.upstream_ids.contains(&req_id));
        }
    }
    //#endregion 🧪️BehaviorTests
}
pub use behavior::*;
//#endregion 🔧️Behavior

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ B1: `ArchitectPlayApp::Command` — the sole typed dispatch surface, one row per action declared
    /// on `create_architect_app`'s `AppBuilder`. Row order IS the binary variant ordinal: appending is
    /// safe, reordering is a wire-format break. Each row's first literal is the camelCase manifest action
    /// id (`command_id()`); the second is the kebab `#[dsl(key)]` wire keyword the codec uses — both are
    /// copied verbatim off the pre-migration `ArchitectCommand` enum, never derived from one another.
    ///
    /// JSON blob arguments (patches, CSV, DSL payloads, node-graph edit lists, viewport JSON) stay
    /// `String`-typed and are parsed inside each handler — mirrors `gis2d`'s `positions_json`/`camera_json`
    /// convention for the same reason (their shapes have no `dsl::DslField` binding of their own).
    pub enum ArchitectCommand for ProgramSnapshot, ProgramMutation, ArchitectConfig, ArchitectConfigMutation {
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "selectRegister" as "select-register" => select_register::SelectRegister,
        "addRegisterItem" as "add-register-item" => add_register_item::AddRegisterItem,
        "removeRegisterItem" as "remove-register-item" => remove_register_item::RemoveRegisterItem,
        "patchRegisterItem" as "patch-register-item" => patch_register_item::PatchRegisterItem,
        "setAdjacencyField" as "set-adjacency-field" => set_adjacency_field::SetAdjacencyField,
        "applyTemplate" as "apply-template" => apply::ApplyTemplate,
        "exportRegistersCsv" as "export-registers-csv" => export_registers_csv::ExportRegistersCsv,
        "importRegistersCsv" as "import-registers-csv" => import_registers_csv::ImportRegistersCsv,
        "addElement" as "add-element" => add_element::AddElement,
        "removeElement" as "remove-element" => remove_element::RemoveElement,
        "runValidation" as "run-validation" => run_validation::RunValidation,
        "runAnalysis" as "run-analysis" => run_analysis::RunAnalysis,
        "runReport" as "run-report" => run_report::RunReport,
        "exportProgram" as "export-program" => export_program::ExportProgram,
        "importProgramRequest" as "import-program-request" => import_program_request::ImportProgramRequest,
        "importProgram" as "import-program" => import_program::ImportProgram,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setAdjacencyKind" as "set-adjacency-kind" => set_adjacency_kind::SetAdjacencyKind,
        "search" as "search" => query::Search,
        "setAdjacencyFilter" as "set-adjacency-filter" => set_adjacency_filter::SetAdjacencyFilter,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ArchitectPlayApp
/// 🧪️ B1: unit struct — every former `RefCell<ArchitectPlayRuntime>` field now lives in
/// `crate::apps::architect::config::ArchitectConfig`, written through `ArchitectConfigMutation`s.
#[derive(Default)]
pub struct ArchitectPlayApp;

impl ArtifactApp for ArchitectPlayApp {
    type Snapshot = ProgramSnapshot;
    type Mutation = ProgramMutation;
    type Config = ArchitectConfig;
    type ConfigMutation = ArchitectConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ArchitectPresence;
    type PresenceMutation = ArchitectPresenceMutation;

    type Command = ArchitectCommand;

    const APP_ID: &'static str = ARCHITECT_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;

    fn initial_snapshot() -> ProgramSnapshot {
        sample_plugin()
    }

    fn initial_config() -> ArchitectConfig {
        ArchitectConfig { active_register: "elements".into(), ..ArchitectConfig::default() }
    }

    fn command_id(command: &ArchitectCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `ArchitectCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly (mirrors `gis2d`'s `command_from_action`).
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<ArchitectCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_bool);
        match action {
            "setSelection" => Ok(ArchitectCommand::SetSelection(set_selection::SetSelection {
                ids: args.and_then(|value| value.get("ids")).and_then(Value::as_array).map(|ids| ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect()).unwrap_or_default(),
            })),
            "selectRegister" => Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: parse_register_id(args).unwrap_or_default() })),
            "addRegisterItem" => Ok(ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                name: str_field("name").unwrap_or_else(|| "New Item".into()),
                template_id: str_field("templateId"),
            })),
            "removeRegisterItem" => Ok(ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
            })),
            "patchRegisterItem" => Ok(ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                patch_json: args.and_then(|value| value.get("patch")).map_or_else(|| "null".into(), Value::to_string),
            })),
            "setAdjacencyField" => Ok(ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField {
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value_json: args.and_then(|value| value.get("value")).map_or_else(|| "null".into(), Value::to_string),
            })),
            "applyTemplate" => Ok(ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: parse_entity_id_from_args(args, "templateId").map(|id| id.0).unwrap_or_default() })),
            "exportRegistersCsv" => Ok(ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})),
            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv {
                csv: str_field("csv").unwrap_or_default(),
                strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()),
            })),
            "addElement" => Ok(ArchitectCommand::AddElement(add_element::AddElement { name: str_field("name").unwrap_or_else(|| "New Room".into()) })),
            "removeElement" => Ok(ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: str_field("elementId").or_else(|| str_field("id")).unwrap_or_default() })),
            "runValidation" => Ok(ArchitectCommand::RunValidation(run_validation::RunValidation {})),
            "runAnalysis" => Ok(ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: str_field("analysisKind").unwrap_or_else(|| "gap".into()) })),
            "runReport" => Ok(ArchitectCommand::RunReport(run_report::RunReport { report_kind: str_field("reportKind").unwrap_or_else(|| "executiveSummary".into()) })),
            "exportProgram" => Ok(ArchitectCommand::ExportProgram(export_program::ExportProgram {})),
            "importProgramRequest" => Ok(ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})),
            "importProgram" => Ok(ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), Value::to_string),
            })),
            "nodeGraphViewport" => Ok(ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: str_field("viewportJson").unwrap_or_default() })),
            "setAdjacencyKind" => Ok(ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind {
                element_a_id: parse_entity_id(args, "elementAId").map(|id| id.0).unwrap_or_default(),
                element_b_id: parse_entity_id(args, "elementBId").map(|id| id.0).unwrap_or_default(),
                kind: str_field("kind"),
                cycle: bool_field("cycle").unwrap_or(false),
            })),
            "search" => Ok(ArchitectCommand::Search(query::Search { query: str_field("query").unwrap_or_default() })),
            "setAdjacencyFilter" => Ok(ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: str_field("kind") })),
            other => Err(Fault::from(format!("architect: unhandled action id {other}"))),
        }
    }

    fn handle(command: &ArchitectCommand, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> UiNode {
        let program = doc.snapshot;
        let config = cfg.snapshot;
        match body_key {
            adjacency_window::ARCHITECT_BODY_ADJACENCY => adjacency_window::render(program, config),
            graph_window::ARCHITECT_BODY_GRAPH => graph_window::render(program, config),
            register_window::ARCHITECT_BODY_REGISTER => register_window::render(program, config),
            report_window::ARCHITECT_BODY_REPORT => report_window::render(config),
            trace_window::ARCHITECT_BODY_TRACE => trace_window::render(program, config),
            document_panel::ARCHITECT_BODY_DOCUMENT => document_panel::render(program, config),
            catalogue_panel::ARCHITECT_BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::ARCHITECT_BODY_INSPECTION => inspection_panel::render(program, config),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ArchitectPlayApp

//#region 🔖️Manifest
pub fn create_architect_app() -> App {
    App::from_builder(
        App::builder(ARCHITECT_APP_ID, LocalizedLabel::native("Architect", "Architekt"))
            .document(["semio", "architect"])
            .icon_id("architect")
            .mode_def(edit_mode::definition())
            .mode_def(review_mode::definition())
            .mode_def(report_mode::definition())
            .default_mode_id(edit_mode::ARCHITECT_MODE_EDIT)
            .window_kind_def(adjacency_window::definition())
            .window_kind_def(graph_window::definition())
            .window_kind_def(register_window::definition())
            .window_kind_def(report_window::definition())
            .window_kind_def(trace_window::definition())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("setAdjacencyKind", LocalizedLabel::native("Set Adjacency Kind", "Adjazenzart festlegen"))
            .mutation("addRegisterItem", LocalizedLabel::native("Add Register Item", "Registereintrag hinzufügen"))
            .mutation("removeRegisterItem", LocalizedLabel::native("Remove Register Item", "Registereintrag entfernen"))
            .mutation("patchRegisterItem", LocalizedLabel::native("Patch Register Item", "Registereintrag patchen"))
            .mutation("importProgram", LocalizedLabel::native("Import ProgramSnapshot", "Programm importieren"))
            .mutation("importRegistersCsv", LocalizedLabel::native("Import Registers CSV", "Register CSV importieren"))
            .mutation("applyTemplate", LocalizedLabel::native("Apply Template", "Vorlage anwenden"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .view_action("selectRegister", LocalizedLabel::native("Select Register", "Register wählen"))
            .view_action("addElement", LocalizedLabel::native("Add Element", "Element hinzufügen"))
            .view_action("removeElement", LocalizedLabel::native("Remove Element", "Element entfernen"))
            .view_action("setAdjacencyField", LocalizedLabel::native("Set Adjacency Field", "Adjazenzfeld setzen"))
            .view_action("runValidation", LocalizedLabel::native("Run Validation", "Validierung ausführen"))
            .view_action("runAnalysis", LocalizedLabel::native("Run Analysis", "Analyse ausführen"))
            .view_action("runReport", LocalizedLabel::native("Run Report", "Bericht erzeugen"))
            .view_action("search", LocalizedLabel::native("Search", "Suchen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .shell_action("exportProgram", LocalizedLabel::native("Export ProgramSnapshot", "Programm exportieren"))
            .shell_action("exportRegistersCsv", LocalizedLabel::native("Export Registers CSV", "Register CSV exportieren"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setAdjacencyFilter", LocalizedLabel::native("Set Adjacency Filter", "Adjazenzfilter setzen"), ActionKind::View) })
            .action_args("selectRegister", vec![ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect())])
            .action_args(
                "addRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID")),
                ],
            )
            .action_args(
                "removeRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                ],
            )
            .action_args(
                "patchRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                    ActionArgDef::text("patch", LocalizedLabel::native("Patch JSON", "Patch-JSON")),
                ],
            )
            .action_args("applyTemplate", vec![ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID"))])
            .action_args(
                "importRegistersCsv",
                vec![
                    ActionArgDef::text("csv", LocalizedLabel::native("CSV", "CSV")),
                    ActionArgDef::select(
                        "strategy",
                        LocalizedLabel::native("Strategy", "Strategie"),
                        vec![
                            ActionArgOption::new("upsert", LocalizedLabel::native("Upsert", "Upsert")),
                            ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
                            ActionArgOption::new("skipDuplicates", LocalizedLabel::native("Skip Duplicates", "Duplikate überspringen")),
                        ],
                    ),
                ],
            )
            .action_args(
                "setAdjacencyKind",
                vec![ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    vec![
                        ActionArgOption::new("required", LocalizedLabel::native("Required", "Erforderlich")),
                        ActionArgOption::new("preferred", LocalizedLabel::native("Preferred", "Bevorzugt")),
                        ActionArgOption::new("optional", LocalizedLabel::native("Optional", "Optional")),
                        ActionArgOption::new("prohibited", LocalizedLabel::native("Prohibited", "Verboten")),
                    ],
                )],
            )
            .action_args("runAnalysis", vec![ActionArgDef::select("analysisKind", LocalizedLabel::native("Analysis", "Analyse"), analysis_kind_picker_options())])
            .action_args("runReport", vec![ActionArgDef::select("reportKind", LocalizedLabel::native("Report", "Bericht"), report_kind_picker_options())])
            .action_args("search", vec![ActionArgDef::text("query", LocalizedLabel::native("Query", "Suchanfrage"))])
            .action_args("importProgram", vec![ActionArgDef::text("payload", LocalizedLabel::native("ProgramSnapshot DSL", "Programm-DSL"))])
            .default_layout(edit_mode::layout()),
    )
    .example("sample", LocalizedLabel::native("Sample Clinic", "Beispielklinik"), serde_json::to_string(&sample_plugin()).expect("sample_plugin is a static hand-built fixture with no non-finite floats or non-UTF8 keys"), "cylinder")
    .example("empty", LocalizedLabel::native("Empty ProgramSnapshot", "Leeres Programm"), serde_json::to_string(&empty_plugin()).expect("empty_plugin is a static hand-built fixture with no non-finite floats or non-UTF8 keys"), "file")
    .workflow("architect", "Architect", "data")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{HistoryView, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type ArchitectApp = VcsArtifactApp<ArchitectPlayApp>;

    pub fn new_app() -> ArchitectApp {
        sdk_new_app::<ArchitectPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> ArchitectApp {
        new_app_with_registry::<ArchitectPlayApp>(create_architect_app)
    }

    pub fn dispatch(app: &mut ArchitectApp, command: ArchitectCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut ArchitectApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🔀️ Drives a typed `ArchitectCommand` straight through `handle` against a bare
    /// `ArchitectPlayApp` — mirrors `cad`'s `drive`/`drive_with_config` harness.
    pub fn drive(command: &ArchitectCommand, program: &ProgramSnapshot) -> Emit<ProgramMutation, ArchitectConfigMutation> {
        drive_with_config(command, program, &ArchitectPlayApp::initial_config())
    }

    pub fn drive_with_config(command: &ArchitectCommand, program: &ProgramSnapshot, config: &ArchitectConfig) -> Emit<ProgramMutation, ArchitectConfigMutation> {
        let history = HistoryView::empty();
        let doc = ArtifactView { snapshot: program, history: &history };
        let cfg = ConfigView { snapshot: config };
        let draft_snapshot = NoDraft::default();
        let draft = DraftView { snapshot: &draft_snapshot };
        let engines = EngineHandles::empty();
        ArchitectPlayApp::handle(command, &doc, &cfg, &draft, &engines).expect("handle")
    }

    /// 🧮️ Folds an `Emit`'s `config_mutations` onto a base `ArchitectConfig` — mirrors what
    /// `VcsArtifactApp`'s config store does when it dispatches them.
    pub fn config_after(emit: &Emit<ProgramMutation, ArchitectConfigMutation>, base: &ArchitectConfig) -> ArchitectConfig {
        use protocol::Mutation;
        let mut next = base.clone();
        for operation in &emit.config_mutations {
            next = operation.diff(&next);
        }
        next
    }

    pub fn render_direct(body_key: &str, program: &ProgramSnapshot, config: &ArchitectConfig) -> UiNode {
        let history = HistoryView::empty();
        ArchitectPlayApp::render(body_key, &ArtifactView { snapshot: program, history: &history }, &ConfigView { snapshot: config })
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::architect::catalog::{analysis_kind_from_str, register_entities};
    use crate::apps::architect::testkit;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::export_registers_csv;
    use crate::artifacts::program::registers::{AdjacencyKind, AnalysisKind};
    use semio_framework_plugin::PluginApp;
    use serde_json::json;

    //#region 🔖️CommandSurface
    /// 🎯️ One value per `app_commands!` row — the fixture behind the wire laws below.
    fn every_command() -> Vec<ArchitectCommand> {
        vec![
            ArchitectCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into(), "b".into()] }),
            ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "risks".into() }),
            ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: None }),
            ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem { register_id: "elements".into(), entity_id: "e1".into() }),
            ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem { register_id: "elements".into(), entity_id: "e1".into(), patch_json: "{\"name\":\"X\"}".into() }),
            ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField { entity_id: "a1".into(), field: "kind".into(), value_json: "\"required\"".into() }),
            ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: "t1".into() }),
            ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {}),
            ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv: "a,b".into(), strategy: "upsert".into() }),
            ArchitectCommand::AddElement(add_element::AddElement { name: "Room".into() }),
            ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: "e1".into() }),
            ArchitectCommand::RunValidation(run_validation::RunValidation {}),
            ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: "gap".into() }),
            ArchitectCommand::RunReport(run_report::RunReport { report_kind: "executiveSummary".into() }),
            ArchitectCommand::ExportProgram(export_program::ExportProgram {}),
            ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {}),
            ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: "text".into() }),
            ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }),
            ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: None, cycle: true }),
            ArchitectCommand::Search(query::Search { query: "hall".into() }),
            ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None }),
        ]
    }

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(ArchitectCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 22, "every ArchitectCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
            let printed = protocol::OpText::print_op(&command);
            let keyword = printed.split_whitespace().next().unwrap_or_default().to_string();
            assert!(keyword.contains('-') || keyword == "search", "row {} printed a non-kebab keyword {printed:?}", command.command_id());
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for every row whose `Option`/`bool` fields make the
    /// `None`/`Some` cases distinct on the wire — copied verbatim out of the ticket's
    /// `🧪️wire-baseline-before.txt`, captured from the pre-migration hand-written `ArchitectCommand`.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &ArchitectCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&ArchitectCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() })), "01000001000c00");
        assert_eq!(hex(&ArchitectCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into(), "b".into()] })), "0100020161016201000c0206000601");
        assert_eq!(
            hex(&ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: None })),
            "01020204526f6f6d08656c656d656e747302000601010600"
        );
        assert_eq!(
            hex(&ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: Some("t1".into()) })),
            "01020304526f6f6d08656c656d656e747302743103000601010600020602"
        );
        assert_eq!(hex(&ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})), "01070000");
        assert_eq!(hex(&ArchitectCommand::RunValidation(run_validation::RunValidation {})), "010b0000");
        assert_eq!(hex(&ArchitectCommand::ExportProgram(export_program::ExportProgram {})), "010e0000");
        assert_eq!(hex(&ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})), "010f0000");
        assert_eq!(
            hex(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: None, cycle: true })),
            "01130201610162030006000106010302"
        );
        assert_eq!(
            hex(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: Some("required".into()), cycle: false })),
            "01130301610162087265717569726564040006000106010206020301"
        );
        assert_eq!(hex(&ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None })), "01150000");
        assert_eq!(hex(&ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: Some("required".into()) })), "01150108726571756972656401000600");
    }

    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<ArchitectPlayApp>(create_architect_app);
        assert!(ArchitectPlayApp::command_from_action("notARealAction", None).is_err());
    }

    /// 🎯️ Spot-check a representative sample of action ids round-tripping into the expected typed
    /// `ArchitectCommand` variant.
    #[test]
    fn command_from_action_bridges_declared_actions() {
        let app = ArchitectPlayApp;
        assert!(matches!(ArchitectPlayApp::command_from_action("runValidation", None), Ok(ArchitectCommand::RunValidation(_))));
        assert!(matches!(ArchitectPlayApp::command_from_action("search", Some(&json!({ "query": "hall" }))), Ok(ArchitectCommand::Search(query::Search { query })) if query == "hall"));
        assert!(matches!(
            ArchitectPlayApp::command_from_action("selectRegister", Some(&json!({ "registerId": "risks" }))),
            Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id })) if register_id == "risks"
        ));
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_architect_app().definition;
        assert_eq!(definition.modes.len(), 3);
        assert_eq!(definition.window_kinds.len(), 5);
        for body_key in [document_panel::ARCHITECT_BODY_DOCUMENT, catalogue_panel::ARCHITECT_BODY_CATALOGUE, inspection_panel::ARCHITECT_BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        for window in [
            adjacency_window::ARCHITECT_WINDOW_ADJACENCY,
            graph_window::ARCHITECT_WINDOW_GRAPH,
            register_window::ARCHITECT_WINDOW_REGISTER,
            report_window::ARCHITECT_WINDOW_REPORT,
            trace_window::ARCHITECT_WINDOW_TRACE,
        ] {
            assert!(definition.window_kinds.iter().any(|kind| kind.id == window), "window kind {window} is stitched into the manifest");
        }
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, "architect.nope").contains("Unknown body"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    #[test]
    fn adjacency_matrix_renders_triangle_strip() {
        let program = sample_plugin();
        let json = serde_json::to_string(&testkit::render_direct(adjacency_window::ARCHITECT_BODY_ADJACENCY, &program, &ArchitectPlayApp::initial_config())).expect("json");
        assert!(json.contains('▲'));
        assert!(json.contains("Reception"));
    }

    #[test]
    fn graph_body_emits_node_graph_scene() {
        let program = sample_plugin();
        let json = serde_json::to_string(&testkit::render_direct(graph_window::ARCHITECT_BODY_GRAPH, &program, &ArchitectPlayApp::initial_config())).expect("json");
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn set_adjacency_kind_cycles_required_to_preferred() {
        let program = sample_plugin();
        let adjacency = program.adjacencies.first().expect("adjacency");
        let emit = testkit::drive(
            &ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: adjacency.element_a_id.0.clone(), element_b_id: adjacency.element_b_id.0.clone(), kind: None, cycle: true }),
            &program,
        );
        assert!(matches!(
            emit.artifact_mutations.first(),
            Some(ProgramMutation::ConnectAdjacency(payload)) if payload.adjacency.kind == AdjacencyKind::Preferred
        ));
    }

    #[test]
    fn run_validation_populates_last_result_json() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp::initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::RunValidation(run_validation::RunValidation {}), &program, &initial);
        assert!(!testkit::config_after(&emit, &initial).last_result_json.is_empty());
    }

    #[test]
    fn search_finds_sample_elements() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp::initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::Search(query::Search { query: "Reception".into() }), &program, &initial);
        let config = testkit::config_after(&emit, &initial);
        assert!(!config.selected_ids.is_empty());
        assert!(!config.search_history_json.is_empty());
    }

    #[test]
    fn select_register_switches_active_register() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp::initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "stakeholders".into() }), &program, &initial);
        assert_eq!(testkit::config_after(&emit, &initial).active_register, "stakeholders");
        assert!(!register_entities(&program, "stakeholders").is_empty());
    }

    #[test]
    fn patch_register_item_updates_element_name() {
        let program = sample_plugin();
        let element_id = program.elements[0].header.id.clone();
        let emit = testkit::drive(
            &ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem { register_id: "elements".into(), entity_id: element_id.0, patch_json: json!({ "name": "Updated Reception" }).to_string() }),
            &program,
        );
        assert!(matches!(
            emit.artifact_mutations.first(),
            Some(ProgramMutation::ReplaceProgramElement(payload)) if payload.program_element.header.name == "Updated Reception"
        ));
    }

    #[test]
    fn formatted_report_renders_section_headings() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp::initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::RunReport(run_report::RunReport { report_kind: "executiveSummary".into() }), &program, &initial);
        let config = testkit::config_after(&emit, &initial);
        let json = serde_json::to_string(&testkit::render_direct(report_window::ARCHITECT_BODY_REPORT, &program, &config)).expect("json");
        assert!(json.contains("Overview"));
        assert!(json.contains("architect-report.section"));
    }

    #[test]
    fn analysis_kind_picker_maps_all_variants() {
        let options = analysis_kind_picker_options();
        assert_eq!(options.len(), 20);
        for option in &options {
            let kind = analysis_kind_from_str(&option.value);
            assert!(!format!("{kind:?}").is_empty(), "missing mapping for {}", option.value);
        }
        assert_eq!(analysis_kind_from_str("relationshipAnalysis"), AnalysisKind::RelationshipAnalysis);
    }

    #[test]
    fn import_registers_csv_action_sets_plugin() {
        let program = sample_plugin();
        let csv = export_registers_csv(&program).expect("export csv");
        let emit = testkit::drive(&ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv, strategy: "upsert".into() }), &program);
        assert!(emit.artifact_mutations.is_empty(), "whole-document load must not go through the Mutation enum");
        assert!(matches!(emit.effects.first(), Some(semio_framework_plugin::HostEffect::LoadDocument { .. })), "importRegistersCsv must emit a LoadDocument effect");
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app();
        let before = app.snapshot().expect("projection").elements.len();
        testkit::dispatch(&mut app, ArchitectCommand::AddElement(add_element::AddElement { name: "Ward".into() }));
        assert_eq!(app.snapshot().expect("projection").elements.len(), before + 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("projection").elements.len(), before);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("projection").elements.len(), before + 1);
    }

    /// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
    /// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
    /// `testkit::app_with_registry` exists.
    #[test]
    fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, ArchitectCommand::SetSelection(set_selection::SetSelection { ids: vec!["e1".into()] }));
        assert!(result.mutations.is_empty(), "setSelection is a view action and must never reach document operations under kind discipline");
    }
    //#endregion 🔖️Behavior
}
//#endregion 🧪️Tests
