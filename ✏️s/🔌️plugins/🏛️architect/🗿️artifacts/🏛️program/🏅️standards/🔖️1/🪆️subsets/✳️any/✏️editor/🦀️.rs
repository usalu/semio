//! 🏛️ Architect editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the five window
//! surfaces in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, view state in
//! `🦀️config.rs`, presentation factories in `🦀️chrome.rs`, the register bridge in `🦀️catalog.rs`;
//! pure derived reads over the document live in the artifact's own `🧬️schema` / `🧬️schema/💡️inferences`
//! (see `//#region 🔧️Behavior` below for the app-scoped, `&mut`-taking counterpart).

use crate::editor::architect::catalog::{analysis_kind_picker_options, parse_entity_id, parse_entity_id_from_args, parse_register_id, report_kind_picker_options, REGISTER_IDS};
use crate::editor::architect::commands::adjacency::{set_adjacency_field, set_adjacency_filter, set_adjacency_kind};
use crate::editor::architect::commands::analysis::{run_analysis, run_report, run_validation};
use crate::editor::architect::commands::element::{add_element, remove_element};
use crate::editor::architect::commands::exchange::{export_program, export_registers_csv, import_program, import_program_request, import_registers_csv};
use crate::editor::architect::commands::graph::{node_graph_edit, node_graph_viewport};
use crate::editor::architect::commands::register::{add_register_item, patch_register_item, remove_register_item, select_register};
use crate::editor::architect::commands::search::query;
use crate::editor::architect::commands::template::apply;
use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
use crate::editor::architect::modes::edit as edit_mode;
use crate::editor::architect::modes::edit::windows::{adjacency as adjacency_window, graph as graph_window, register as register_window, report as report_window, trace as trace_window};
use crate::editor::architect::modes::{report as report_mode, review as review_mode};
use crate::editor::architect::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::architect::presence::{ArchitectPresence, ArchitectPresenceMutation};
use crate::op::ProgramMutation;
use crate::{sample_plugin, ProgramSnapshot, ARCHITECT_PROGRAM_SCHEMA};
// 🚧️ `Dialect`/`InteractionView` are only reachable through `app`, not yet in the crate-root
// re-export list (see the identical note in the sibling viewer surface's root `🦀️.rs`).
use dsl::DslValue as Value;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label,
    LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
};
use store::EngineHandles;

//#region 🔖️Constants
pub const ARCHITECT_APP_ID: &str = "s.architect.program@1/*#editor";

/// 🏷️ Admits a semantic architect label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| ui_capacity_error())
}

/// 📝️ Admits a fixed architect UI string.
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Finalizes a architect UI node with explicit identity.
pub fn ui_node<B: semio_framework_ui_contract::HasBase + semio_framework_ui_contract::Buildable>(builder: B, id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_id(id).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

/// 👶️ Admits a complete collection of architect UI children.
pub fn ui_children<B: semio_framework_ui_contract::HasChildren>(builder: B, children: impl IntoIterator<Item = semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_children(children).map_err(|_| ui_capacity_error())
}

/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("architect.ui.capacity", "architect UI admission failed")
}

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🪟️windows/*`) builds its item/on-change actions with.
pub fn architect_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(ARCHITECT_APP_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "program" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction
/// domain this app declares: `HierarchyProvider::Flat` (the 68 registers are flat lists of
/// `EntityId`-keyed rows, no parent/child nesting), one granularity ("entity") covering every
/// register kind uniformly (mirrors note's single "block" granularity over several block kinds).
/// The document panel's element rows are the sole real pick surface today (`📌️panels/🗿️artifact`);
/// the register/graph/trace windows lost their per-selection detail entirely (`render` carries no
/// `InteractionView` — see each window's own doc comment) rather than being force-wired to a
/// surface (`BlockListScene`/`NodeGraphScene`) that cannot show it yet.
pub const ARCHITECT_INTERACTION_PROGRAM: &str = "program";
pub const ARCHITECT_INTERACTION_GRANULARITY_ENTITY: &str = "entity";
//#endregion 🔖️Interaction

//#region 🔖️ResetDocument
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (the whole-document
/// replace variant — see `📓️taxonomy.md`'s forbidden vocabulary), so import/exchange flows build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry —
/// same mechanism `✏️s/🔌️plugins/🗒️note`'s `reset_document_effect` already established.
pub fn reset_document_effect(document: &ProgramSnapshot) -> semio_framework_plugin::Effect {
    let pack = <ProgramSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<ProgramSnapshot, ProgramMutation>(ARCHITECT_PROGRAM_SCHEMA, ARCHITECT_APP_ID, document.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("architect program document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔧️Behavior
/// 🔧️ Dissolved out of the former artifact-tree `⚙️engine` topic files (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every function that takes `&mut
/// ProgramSnapshot` (constructs/mutates) rather than merely reading it. `🎛️apps/🏛️architect/⚙️engine`
/// exists on disk as the reserved machine-slot stub, but `🦀️.rs` (out of scope for this ticket)
/// never mounts it into the module tree — only two `pub mod engine { ... }` shims exist there and
/// both live in the ARTIFACT-tree shim section, not the Apps section — so populating that directory
/// would produce dead, uncompiled code. Per this packet's own fallback this behavior lands directly
/// on the app top-level instead; the reserved directory is left empty and untouched. This does NOT
/// invent a new state machine: every item below is a plain function/struct, unchanged in shape from
/// its former engine-topic file, just relocated.
pub mod behavior {
    use crate::kernel::{EntityHeader, EntityId, PluginError, TextField, TraceKind, TraceLink};
    use crate::op::ProgramMutation;
    use crate::registers::{
        Activity, Adjacency, AdjacencyKind, AnalysisKind, AnalysisRecord, ConnectionKind, Equipment, Function, FunctionKind, Process, ProgramElement, ProgramElementKind, Relationship, RelationshipKind, ReportKind, ReportRecord, Requirement,
        RequirementKind, Risk, RiskLevel, Stakeholder, TemplateRecord, UserCategory, UserProfile, ValidationStatus,
    };
    use crate::standards::v1::subsets::any::schema::inferences::{build_report, run_analysis, RegisterCsvRow};
    use crate::standards::v1::subsets::any::schema::normalize_pair;
    use crate::ProgramSnapshot;
    use semio_s_artifact_stdio_csv as stdio_csv;
    use semio_s_artifact_stdio_tsv as stdio_tsv;
    use semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_engine;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    /// 🔀 Absolute path to the `🧬️mutations` facet's per-register leaves, kept as one alias so the
    /// semantic-mutations-overhaul rename (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL`)
    /// only needs updating here if `🦀️.rs`'s directory wiring ever changes.
    use crate::schema::mutations as leaves;

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
    #[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
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
                        influence: crate::registers::InfluenceLevel::Medium,
                        interest: crate::registers::InfluenceLevel::Medium,
                        engagement: crate::registers::EngagementLevel::Neutral,
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
                    operations.push(ProgramMutation::CreateStakeholder(leaves::create_stakeholder::CreateStakeholder { stakeholder: item.clone() }));
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
                    operations.push(ProgramMutation::CreateUserProfile(leaves::create_user_profile::CreateUserProfile { user_profile: item.clone() }));
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
                        participants: crate::kernel::QuantitySpec::default(),
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
                    operations.push(ProgramMutation::CreateActivity(leaves::create_activity::CreateActivity { activity: item.clone() }));
                    program.activities.push(item);
                }
                "function" => {
                    let item = Function {
                        header: EntityHeader::new(id.clone(), format!("{} Function", template.header.name)),
                        code: "FN".into(),
                        kind: FunctionKind::Primary,
                        purpose: TextField::plain(template.standards.join(", ")),
                        criticality: crate::kernel::Priority::Preferred,
                        performance_targets: Vec::new(),
                        service_level: None,
                        operating_hours: None,
                        staffing: crate::kernel::QuantitySpec::default(),
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
                    operations.push(ProgramMutation::CreateFunction(leaves::create_function::CreateFunction { function: item.clone() }));
                    program.functions.push(item);
                }
                "element" | "room" => {
                    let item = ProgramElement {
                        header: EntityHeader::new(id.clone(), format!("{} Space", template.header.name)),
                        code: "TPL".into(),
                        kind: ProgramElementKind::Room,
                        parent_id: None,
                        level: None,
                        area: crate::kernel::QuantitySpec::default(),
                        volume: crate::kernel::QuantitySpec::default(),
                        height: crate::kernel::QuantitySpec::default(),
                        occupancy: crate::kernel::QuantitySpec::default(),
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
                    operations.push(ProgramMutation::CreateProgramElement(leaves::create_program_element::CreateProgramElement { program_element: item.clone() }));
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
                    operations.push(ProgramMutation::CreateRequirement(leaves::create_requirement::CreateRequirement { requirement: item.clone() }));
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
                    operations.push(ProgramMutation::CreateRisk(leaves::create_risk::CreateRisk { risk: item.clone() }));
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
                    operations.push(ProgramMutation::CreateProcess(leaves::create_process::CreateProcess { process: item.clone() }));
                    program.processes.push(item);
                }
                "equipment" => {
                    let item = Equipment {
                        header: EntityHeader::new(id.clone(), format!("{} Equipment", template.header.name)),
                        code: "EQ".into(),
                        category: template.sector.clone().unwrap_or_else(|| "general".into()),
                        manufacturer: None,
                        model: None,
                        quantity: crate::kernel::QuantitySpec::default(),
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
                    operations.push(ProgramMutation::CreateEquipment(leaves::create_equipment::CreateEquipment { equipment: item.clone() }));
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
                    operations.push(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency: adjacency.clone() }));
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
    pub fn build_report_and_record(program: &mut ProgramSnapshot, kind: ReportKind) -> crate::standards::v1::subsets::any::schema::inferences::ProgramReport {
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
    pub fn run_analysis_and_record(program: &mut ProgramSnapshot, kind: AnalysisKind) -> crate::standards::v1::subsets::any::schema::inferences::AnalysisResult {
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::ToValue, dsl::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
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
            area: crate::kernel::QuantitySpec::default(),
            volume: crate::kernel::QuantitySpec::default(),
            height: crate::kernel::QuantitySpec::default(),
            occupancy: crate::kernel::QuantitySpec::default(),
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
            influence: crate::registers::InfluenceLevel::Medium,
            interest: crate::registers::InfluenceLevel::Medium,
            engagement: crate::registers::EngagementLevel::Neutral,
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
            relationship_priority: crate::kernel::Priority::Preferred,
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
    #[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct TraceChain {
        pub root_id: EntityId,
        pub links: Vec<TraceLink>,
    }

    /// 💥️ Reverse impact set from trace links pointing at an entity.
    #[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    include!("🧪️tests/🔬️behavior-unit/🦀️.rs");
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
/// `crate::editor::architect::config::ArchitectConfig`, written through `ArchitectConfigMutation`s.
#[derive(Default)]
pub struct ArchitectPlayApp;

impl ArtifactEditor for ArchitectPlayApp {
    type Snapshot = ProgramSnapshot;
    type Mutation = ProgramMutation;
    type Config = ArchitectConfig;
    type ConfigMutation = ArchitectConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ArchitectPresence;
    type PresenceMutation = ArchitectPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = ArchitectCommand;

    const DIALECT: Dialect = crate::ARCHITECT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::architect::config::schema::app_schema_descriptor())
    }

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
            "selectRegister" => Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: parse_register_id(args).unwrap_or_default() })),
            "addRegisterItem" => {
                Ok(ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: parse_register_id(args).unwrap_or_default(), name: str_field("name").unwrap_or_else(|| "New Item".into()), template_id: str_field("templateId") }))
            }
            "removeRegisterItem" => {
                Ok(ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem { register_id: parse_register_id(args).unwrap_or_default(), entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default() }))
            }
            "patchRegisterItem" => Ok(ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                patch_json: args.and_then(|value| value.get("patch")).map_or_else(|| "null".into(), dsl::json::to_json_string),
            })),
            "setAdjacencyField" => Ok(ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField {
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value_json: args.and_then(|value| value.get("value")).map_or_else(|| "null".into(), dsl::json::to_json_string),
            })),
            "applyTemplate" => Ok(ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: parse_entity_id_from_args(args, "templateId").map(|id| id.0).unwrap_or_default() })),
            "exportRegistersCsv" => Ok(ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})),
            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv: str_field("csv").unwrap_or_default(), strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()) })),
            "addElement" => Ok(ArchitectCommand::AddElement(add_element::AddElement { name: str_field("name").unwrap_or_else(|| "New Room".into()) })),
            "removeElement" => Ok(ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: str_field("elementId").or_else(|| str_field("id")).unwrap_or_default() })),
            "runValidation" => Ok(ArchitectCommand::RunValidation(run_validation::RunValidation {})),
            "runAnalysis" => Ok(ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: str_field("analysisKind").unwrap_or_else(|| "gap".into()) })),
            "runReport" => Ok(ArchitectCommand::RunReport(run_report::RunReport { report_kind: str_field("reportKind").unwrap_or_else(|| "executiveSummary".into()) })),
            "exportProgram" => Ok(ArchitectCommand::ExportProgram(export_program::ExportProgram {})),
            "importProgramRequest" => Ok(ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})),
            "importProgram" => Ok(ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), dsl::json::to_json_string) })),
            "nodeGraphViewport" => {
                let value = args.and_then(|value| value.get("viewport")).cloned().ok_or_else(|| Fault::from("nodeGraphViewport requires viewport"))?;
                let viewport = dsl::from_dsl_value::<semio_framework::Viewport2d>(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))?;
                Ok(ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport }))
            }
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

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no `ArchitectCommand` row reads
    /// the "program" domain's selection — the document panel's own tree click (its sole real pick
    /// surface) is intercepted by the framework's injected `interactionSelect` before it ever
    /// reaches `handle` (see `dispatch_action`'s reserved-verb interception), and every remaining
    /// command derives its ids from explicit args, not the live selection (mirrors `note`'s
    /// `app_commands!`-dispatched leaves that never needed `InteractionView` either).
    fn handle(
        command: &ArchitectCommand,
        doc: &ArtifactView<'_, ProgramSnapshot>,
        cfg: &ConfigView<'_, ArchitectConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let program = doc.snapshot;
        let config = cfg.snapshot;
        match body_key {
            adjacency_window::ARCHITECT_BODY_ADJACENCY => adjacency_window::render(program, config),
            graph_window::ARCHITECT_BODY_GRAPH => graph_window::render(program, config),
            register_window::ARCHITECT_BODY_REGISTER => register_window::render(program, config),
            report_window::ARCHITECT_BODY_REPORT => report_window::render(config),
            trace_window::ARCHITECT_BODY_TRACE => trace_window::render(program),
            document_panel::ARCHITECT_BODY_DOCUMENT => document_panel::render(program, config),
            catalogue_panel::ARCHITECT_BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::ARCHITECT_BODY_INSPECTION => inspection_panel::render(program, config),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️ArchitectPlayApp

//#region 🔖️Manifest
pub fn create_architect_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::ARCHITECT_DIALECT)
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
            .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View, "camera"))
            .view_action("selectRegister", LocalizedLabel::native("Select Register", "Register wählen"))
            .view_action("addElement", LocalizedLabel::native("Add Element", "Element hinzufügen"))
            .view_action("removeElement", LocalizedLabel::native("Remove Element", "Element entfernen"))
            .view_action("setAdjacencyField", LocalizedLabel::native("Set Adjacency Field", "Adjazenzfeld setzen"))
            .view_action("runValidation", LocalizedLabel::native("Run Validation", "Validierung ausführen"))
            .view_action("runAnalysis", LocalizedLabel::native("Run Analysis", "Analyse ausführen"))
            .view_action("runReport", LocalizedLabel::native("Run Report", "Bericht erzeugen"))
            .action_with(ActionDefinition::new("search", LocalizedLabel::native("Search", "Suchen"), ActionKind::View, "search"))
            .action_with(ActionDefinition::new("exportProgram", LocalizedLabel::native("Export ProgramSnapshot", "Programm exportieren"), ActionKind::Shell, "download"))
            .action_with(ActionDefinition::new("exportRegistersCsv", LocalizedLabel::native("Export Registers CSV", "Register CSV exportieren"), ActionKind::Shell, "download"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setAdjacencyFilter", LocalizedLabel::native("Set Adjacency Filter", "Adjazenzfilter setzen"), ActionKind::View) })
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
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "program" interaction
            // domain — one granularity ("entity") over the 68 flat registers, `HierarchyProvider::Flat`
            // (no parent/child nesting), single-select Pick/Replace only (the document panel's element
            // rows are the sole real pick surface today — see its own doc comment). Scoped to the graph
            // window (the node-graph canvas, this app's closest analog to a primary editing surface),
            // mirroring `dag`'s main window's identical declaration.
            .interaction(InteractionDefinition {
                id: ARCHITECT_INTERACTION_PROGRAM.into(),
                label: LocalizedLabel::native("Program", "Programm"),
                granularities: vec![GranularityDefinition { id: ARCHITECT_INTERACTION_GRANULARITY_ENTITY.into(), label: LocalizedLabel::native("Entity", "Entität"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            })
            .window_kind_interactions(graph_window::ARCHITECT_WINDOW_GRAPH, vec![InteractionRef::new(ARCHITECT_INTERACTION_PROGRAM)])
            .default_layout(edit_mode::layout())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
            // `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old `"sample"`/`"empty"` example
            // registrations and the no-op `.workflow("architect", "Architect", "data")` call are dropped
            // here (not silently: reported in the packet's migration report). The subset's own
            // `📚️examples/🎬️demo`/`📚️examples/🎬️demo-session` facets are the modern, role-agnostic
            // replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests
