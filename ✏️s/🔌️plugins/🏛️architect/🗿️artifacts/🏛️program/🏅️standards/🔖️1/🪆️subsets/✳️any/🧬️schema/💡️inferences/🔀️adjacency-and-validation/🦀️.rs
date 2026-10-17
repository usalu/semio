//! 🔀️ Program adjacency views and conflicts, and the structural validation of a program snapshot.

use super::*;

//#region 🔀️AdjacencyViews
/// 🔢️ Dense lower-triangle adjacency matrix keyed by element id order.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AdjacencyMatrix {
    pub element_ids: Vec<EntityId>,
    pub cells: Vec<Vec<Option<AdjacencyCell>>>,
}

/// 🟦️ One matrix cell summarizing the undirected link between two elements.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AdjacencyCell {
    pub adjacency_id: EntityId,
    pub kind: AdjacencyKind,
    pub weight: f64,
    pub separations: Vec<SeparationKind>,
}

/// 📊️ Builds a lower-triangle matrix view over program elements and adjacencies.
pub fn adjacency_matrix(program: &ProgramSnapshot) -> AdjacencyMatrix {
    let mut element_ids: Vec<EntityId> = program.elements.iter().map(|e| e.header.id.clone()).collect();
    element_ids.sort();
    let n = element_ids.len();
    let mut cells = vec![vec![None; n]; n];
    for adjacency in &program.adjacencies {
        let Ok(a) = element_ids.binary_search(&adjacency.element_a_id) else {
            continue;
        };
        let Ok(b) = element_ids.binary_search(&adjacency.element_b_id) else {
            continue;
        };
        let (row, col) = if a > b { (a, b) } else { (b, a) };
        cells[row][col] = Some(AdjacencyCell { adjacency_id: adjacency.header.id.clone(), kind: adjacency.kind.clone(), weight: adjacency.weight, separations: adjacency.separations.clone() });
    }
    AdjacencyMatrix { element_ids, cells }
}

/// 🕸️ Undirected edge list for graph rendering (`a`, `b`, weight).
pub fn undirected_edges(program: &ProgramSnapshot) -> Vec<(EntityId, EntityId, f64)> {
    program.adjacencies.iter().map(|adjacency| (adjacency.element_a_id.clone(), adjacency.element_b_id.clone(), adjacency.weight)).collect()
}
//#endregion 🔀️AdjacencyViews

//#region ⚡️AdjacencyConflicts
/// ⚡️ Adjacency pair ids that violate required/prohibited or separation rules.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AdjacencyConflict {
    pub adjacency_a_id: EntityId,
    pub adjacency_b_id: EntityId,
    pub message: String,
}

/// 🔍️ Detects duplicate pairs, kind conflicts, separation/distance/level violations.
pub fn detect_adjacency_conflicts(program: &ProgramSnapshot) -> Vec<AdjacencyConflict> {
    let mut conflicts = Vec::new();
    for (i, left) in program.adjacencies.iter().enumerate() {
        if let (Some(min), Some(max)) = (left.distance_min_m, left.distance_max_m) {
            if min > max {
                conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: left.header.id.clone(), message: format!("distance_min_m ({min}) exceeds distance_max_m ({max})") });
            }
        }
        for right in program.adjacencies.iter().skip(i + 1) {
            let same_pair = (left.element_a_id == right.element_a_id && left.element_b_id == right.element_b_id) || (left.element_a_id == right.element_b_id && left.element_b_id == right.element_a_id);
            if !same_pair {
                continue;
            }
            conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: right.header.id.clone(), message: "duplicate adjacency pair".into() });
            if left.kind == AdjacencyKind::Required && right.kind == AdjacencyKind::Prohibited {
                conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: right.header.id.clone(), message: "required adjacency conflicts with prohibited".into() });
            }
            if let (Some(a), Some(b)) = (&left.level_constraint, &right.level_constraint) {
                if a != b {
                    conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: right.header.id.clone(), message: format!("conflicting level constraints: {a} vs {b}") });
                }
            }
            if separation_incompatible(&left.separations, &right.separations) {
                conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: right.header.id.clone(), message: "incompatible separation requirements on same pair".into() });
            }
            if let (Some(min_a), Some(max_b)) = (left.distance_min_m, right.distance_max_m) {
                if min_a > max_b {
                    conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: right.header.id.clone(), message: format!("distance min {min_a} exceeds paired max {max_b}") });
                }
            }
        }
        if left.kind == AdjacencyKind::Required {
            for other in &program.adjacencies {
                if other.header.id == left.header.id {
                    continue;
                }
                if other.element_a_id == left.element_a_id && other.element_b_id == left.element_b_id && other.kind == AdjacencyKind::Prohibited {
                    conflicts.push(AdjacencyConflict { adjacency_a_id: left.header.id.clone(), adjacency_b_id: other.header.id.clone(), message: "required adjacency conflicts with prohibited".into() });
                }
            }
        }
    }
    conflicts
}

fn separation_incompatible(left: &[SeparationKind], right: &[SeparationKind]) -> bool {
    let fire_acoustic = |s: &SeparationKind| matches!(s, SeparationKind::Fire | SeparationKind::Acoustic);
    let has_fire = left.iter().any(fire_acoustic) || right.iter().any(fire_acoustic);
    let has_circulation = left.contains(&SeparationKind::Circulation) || right.contains(&SeparationKind::Circulation);
    has_fire && has_circulation && !(left.contains(&SeparationKind::Fire) && right.contains(&SeparationKind::Fire))
}
//#endregion ⚡️AdjacencyConflicts
//#region ✅️Validate
struct EntityIndex {
    locations: HashMap<EntityId, (String, String)>,
    duplicates: Vec<(EntityId, String, String)>,
}

fn build_entity_index(program: &ProgramSnapshot) -> EntityIndex {
    let mut locations: HashMap<EntityId, (String, String)> = HashMap::new();
    let mut duplicates = Vec::new();
    let mut register = |name: &str, id: &EntityId, label: &str| {
        if let Some((prev_reg, _)) = locations.get(id) {
            duplicates.push((id.clone(), prev_reg.clone(), name.to_string()));
        } else {
            locations.insert(id.clone(), (name.to_string(), label.to_string()));
        }
    };
    for e in &program.stakeholders {
        register("stakeholders", &e.header.id, &e.header.name);
    }
    for e in &program.users {
        register("users", &e.header.id, &e.header.name);
    }
    for e in &program.activities {
        register("activities", &e.header.id, &e.header.name);
    }
    for e in &program.functions {
        register("functions", &e.header.id, &e.header.name);
    }
    for e in &program.elements {
        register("elements", &e.header.id, &e.header.name);
    }
    for e in &program.quantities {
        register("quantities", &e.header.id, &e.header.name);
    }
    for e in &program.relationships {
        register("relationships", &e.header.id, &e.header.name);
    }
    for e in &program.adjacencies {
        register("adjacencies", &e.header.id, &e.header.name);
    }
    for e in &program.processes {
        register("processes", &e.header.id, &e.header.name);
    }
    for e in &program.flows {
        register("flows", &e.header.id, &e.header.name);
    }
    for e in &program.access_rules {
        register("access_rules", &e.header.id, &e.header.name);
    }
    for e in &program.operations {
        register("operations", &e.header.id, &e.header.name);
    }
    for e in &program.equipment {
        register("equipment", &e.header.id, &e.header.name);
    }
    for e in &program.resources {
        register("resources", &e.header.id, &e.header.name);
    }
    for e in &program.storage {
        register("storage", &e.header.id, &e.header.name);
    }
    for e in &program.environmental {
        register("environmental", &e.header.id, &e.header.name);
    }
    for e in &program.human_factors {
        register("human_factors", &e.header.id, &e.header.name);
    }
    for e in &program.accessibility {
        register("accessibility", &e.header.id, &e.header.name);
    }
    for e in &program.privacy {
        register("privacy", &e.header.id, &e.header.name);
    }
    for e in &program.safety {
        register("safety", &e.header.id, &e.header.name);
    }
    for e in &program.security {
        register("security", &e.header.id, &e.header.name);
    }
    for e in &program.regulatory {
        register("regulatory", &e.header.id, &e.header.name);
    }
    for e in &program.site_context {
        register("site_context", &e.header.id, &e.header.name);
    }
    for e in &program.organizational {
        register("organizational", &e.header.id, &e.header.name);
    }
    for e in &program.services {
        register("services", &e.header.id, &e.header.name);
    }
    for e in &program.infrastructure {
        register("infrastructure", &e.header.id, &e.header.name);
    }
    for e in &program.information {
        register("information", &e.header.id, &e.header.name);
    }
    for e in &program.communication {
        register("communication", &e.header.id, &e.header.name);
    }
    for e in &program.wayfinding {
        register("wayfinding", &e.header.id, &e.header.name);
    }
    for e in &program.schedules {
        register("schedules", &e.header.id, &e.header.name);
    }
    for e in &program.flexibility {
        register("flexibility", &e.header.id, &e.header.name);
    }
    for e in &program.growth {
        register("growth", &e.header.id, &e.header.name);
    }
    for e in &program.sustainability {
        register("sustainability", &e.header.id, &e.header.name);
    }
    for e in &program.resilience {
        register("resilience", &e.header.id, &e.header.name);
    }
    for e in &program.costs {
        register("costs", &e.header.id, &e.header.name);
    }
    for e in &program.delivery {
        register("delivery", &e.header.id, &e.header.name);
    }
    for e in &program.risks {
        register("risks", &e.header.id, &e.header.name);
    }
    for e in &program.conflicts {
        register("conflicts", &e.header.id, &e.header.name);
    }
    for e in &program.requirements {
        register("requirements", &e.header.id, &e.header.name);
    }
    for e in &program.priorities {
        register("priorities", &e.header.id, &e.header.name);
    }
    for e in &program.scenarios {
        register("scenarios", &e.header.id, &e.header.name);
    }
    for e in &program.options {
        register("options", &e.header.id, &e.header.name);
    }
    for e in &program.decisions {
        register("decisions", &e.header.id, &e.header.name);
    }
    for e in &program.validations {
        register("validations", &e.header.id, &e.header.name);
    }
    for e in &program.performance {
        register("performance", &e.header.id, &e.header.name);
    }
    for e in &program.quality {
        register("quality", &e.header.id, &e.header.name);
    }
    for e in &program.artifacts {
        register("documents", &e.header.id, &e.header.name);
    }
    for e in &program.changes {
        register("changes", &e.header.id, &e.header.name);
    }
    for e in &program.collaboration {
        register("collaboration", &e.header.id, &e.header.name);
    }
    for e in &program.analyses {
        register("analyses", &e.header.id, &e.header.name);
    }
    for e in &program.reports {
        register("reports", &e.header.id, &e.header.name);
    }
    for e in &program.search_filters {
        register("search_filters", &e.header.id, &e.header.name);
    }
    for e in &program.status_records {
        register("status_records", &e.header.id, &e.header.name);
    }
    for e in &program.workshops {
        register("workshops", &e.header.id, &e.header.name);
    }
    for e in &program.surveys {
        register("surveys", &e.header.id, &e.header.name);
    }
    for e in &program.issues {
        register("issues", &e.header.id, &e.header.name);
    }
    for e in &program.audit_events {
        register("audit_events", &e.header.id, &e.header.name);
    }
    for e in &program.templates {
        register("templates", &e.header.id, &e.header.name);
    }
    for e in &program.knowledge_payload {
        register("knowledge", &e.header.id, &e.header.name);
    }
    for e in &program.benchmarks_payload {
        register("benchmarks", &e.header.id, &e.header.name);
    }
    register("project", &program.project.id, &program.project.code);
    register("governance", &program.governance.id, "governance");
    for link in &program.traces {
        register("traces", &link.id, &format!("{}→{}", link.from_id, link.to_id));
    }
    EntityIndex { locations, duplicates }
}

fn check_ref(diagnostics: &mut Vec<ProgramDiagnostic>, index: &EntityIndex, target: &EntityId, source_id: &EntityId, register: &str, code: &str) {
    if !index.locations.contains_key(target) {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: code.into(), message: format!("{register} references missing entity {target}"), entity_id: Some(source_id.clone()), register: Some(register.into()) });
    }
}

/// 🩺️ Validates a plugin document and returns all diagnostics (non-fatal).
pub fn validate_plugin(program: &ProgramSnapshot) -> Vec<ProgramDiagnostic> {
    let mut diagnostics = Vec::new();
    if program.schema != ARCHITECT_PROGRAM_SCHEMA {
        diagnostics.push(ProgramDiagnostic {
            severity: DiagnosticSeverity::Error,
            code: "schema.mismatch".into(),
            message: format!("expected schema {ARCHITECT_PROGRAM_SCHEMA}, got {}", program.schema),
            entity_id: None,
            register: Some("meta".into()),
        });
    }
    if program.meta.title.trim().is_empty() {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Warning, code: "meta.empty_title".into(), message: "program title is empty".into(), entity_id: None, register: Some("meta".into()) });
    }

    let index = build_entity_index(program);
    for (id, first, second) in &index.duplicates {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "entity.duplicate_id".into(), message: format!("entity id {id} appears in both {first} and {second}"), entity_id: Some(id.clone()), register: None });
    }

    let element_ids: HashSet<EntityId> = program.elements.iter().map(|e| e.header.id.clone()).collect();

    for element in &program.elements {
        if let Some(parent) = &element.parent_id {
            check_ref(&mut diagnostics, &index, parent, &element.header.id, "elements", "element.missing_parent");
        }
        for id in &element.function_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_function");
        }
        for id in &element.activity_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_activity");
        }
        for id in &element.user_profile_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_user");
        }
        for id in &element.adjacency_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_adjacency");
        }
        for id in &element.quantity_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_quantity");
        }
        for id in &element.requirement_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_requirement");
        }
    }

    for function in &program.functions {
        for id in &function.activity_ids {
            check_ref(&mut diagnostics, &index, id, &function.header.id, "functions", "function.missing_activity");
        }
        for id in &function.element_ids {
            check_ref(&mut diagnostics, &index, id, &function.header.id, "functions", "function.missing_element");
        }
        for id in &function.dependencies {
            check_ref(&mut diagnostics, &index, id, &function.header.id, "functions", "function.missing_dependency");
        }
        if let Some(parent) = &function.hierarchy_parent_id {
            check_ref(&mut diagnostics, &index, parent, &function.header.id, "functions", "function.missing_parent");
        }
    }

    for activity in &program.activities {
        for id in &activity.function_ids {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_function");
        }
        for id in &activity.adjacent_activities {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_adjacent_activity");
        }
        for id in &activity.user_profile_ids {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_user");
        }
        for id in &activity.equipment_ids {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_equipment");
        }
    }

    for requirement in &program.requirements {
        if requirement.element_ids.is_empty() && requirement.function_ids.is_empty() {
            diagnostics.push(ProgramDiagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "requirement.orphan".into(),
                message: format!("requirement {} is not linked to elements or functions", requirement.header.id),
                entity_id: Some(requirement.header.id.clone()),
                register: Some("requirements".into()),
            });
        }
        if let Some(parent) = &requirement.parent_requirement_id {
            check_ref(&mut diagnostics, &index, parent, &requirement.header.id, "requirements", "requirement.missing_parent");
        }
        for id in &requirement.child_requirement_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_child");
        }
        for id in &requirement.stakeholder_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_stakeholder");
        }
        for id in &requirement.element_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_element");
        }
        for id in &requirement.function_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_function");
        }
        for id in &requirement.conflict_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_conflict");
        }
        for id in &requirement.risk_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_risk");
        }
        if let Some(superseded) = &requirement.superseded_by {
            check_ref(&mut diagnostics, &index, superseded, &requirement.header.id, "requirements", "requirement.missing_superseded_by");
        }
    }

    for relationship in &program.relationships {
        check_ref(&mut diagnostics, &index, &relationship.source_id, &relationship.header.id, "relationships", "relationship.missing_source");
        check_ref(&mut diagnostics, &index, &relationship.target_id, &relationship.header.id, "relationships", "relationship.missing_target");
    }

    for adjacency in &program.adjacencies {
        for endpoint in [&adjacency.element_a_id, &adjacency.element_b_id] {
            if !element_ids.contains(endpoint) {
                diagnostics.push(ProgramDiagnostic {
                    severity: DiagnosticSeverity::Error,
                    code: "adjacency.missing_element".into(),
                    message: format!("adjacency references missing element {endpoint}"),
                    entity_id: Some(adjacency.header.id.clone()),
                    register: Some("adjacencies".into()),
                });
            }
        }
        if let Some(rel_id) = &adjacency.source_relationship_id {
            check_ref(&mut diagnostics, &index, rel_id, &adjacency.header.id, "adjacencies", "adjacency.missing_relationship");
        }
    }

    for process in &program.processes {
        for id in &process.actors {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_actor");
        }
        for id in &process.equipment_ids {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_equipment");
        }
        for id in &process.element_ids {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_element");
        }
        for id in &process.dependencies {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_dependency");
        }
    }

    for equipment in &program.equipment {
        for id in &equipment.element_ids {
            check_ref(&mut diagnostics, &index, id, &equipment.header.id, "equipment", "equipment.missing_element");
        }
        for id in &equipment.activity_ids {
            check_ref(&mut diagnostics, &index, id, &equipment.header.id, "equipment", "equipment.missing_activity");
        }
    }

    for quantity in &program.quantities {
        check_ref(&mut diagnostics, &index, &quantity.target_element_id, &quantity.header.id, "quantities", "quantity.missing_element");
        for id in &quantity.related_requirement_ids {
            check_ref(&mut diagnostics, &index, id, &quantity.header.id, "quantities", "quantity.missing_requirement");
        }
    }

    for conflict in &program.conflicts {
        check_ref(&mut diagnostics, &index, &conflict.entity_a_id, &conflict.header.id, "conflicts", "conflict.missing_entity_a");
        check_ref(&mut diagnostics, &index, &conflict.entity_b_id, &conflict.header.id, "conflicts", "conflict.missing_entity_b");
        if conflict.entity_a_id == conflict.entity_b_id {
            diagnostics.push(ProgramDiagnostic {
                severity: DiagnosticSeverity::Error,
                code: "conflict.self_reference".into(),
                message: "conflict references the same entity for both sides".into(),
                entity_id: Some(conflict.header.id.clone()),
                register: Some("conflicts".into()),
            });
        }
        if let Some(decision_id) = &conflict.decision_id {
            check_ref(&mut diagnostics, &index, decision_id, &conflict.header.id, "conflicts", "conflict.missing_decision");
        }
    }

    for status in &program.status_records {
        check_ref(&mut diagnostics, &index, &status.subject_id, &status.header.id, "status_records", "status.missing_subject");
    }

    for validation in &program.validations {
        check_ref(&mut diagnostics, &index, &validation.subject_id, &validation.header.id, "validations", "validation.missing_subject");
        if validation.result == ValidationStatus::Failed && validation.corrective_actions.is_empty() {
            diagnostics.push(ProgramDiagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "validation.failed_without_actions".into(),
                message: format!("validation {} failed without corrective actions", validation.header.id),
                entity_id: Some(validation.header.id.clone()),
                register: Some("validations".into()),
            });
        }
    }

    for conflict in detect_adjacency_conflicts(program) {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "adjacency.conflict".into(), message: conflict.message, entity_id: Some(conflict.adjacency_a_id), register: Some("adjacencies".into()) });
    }

    diagnostics
}
//#endregion ✅️Validate
