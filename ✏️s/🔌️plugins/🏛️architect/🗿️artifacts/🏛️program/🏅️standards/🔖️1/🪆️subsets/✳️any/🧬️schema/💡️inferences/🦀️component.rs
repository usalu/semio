//! 💡️ ProgramSnapshot inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).
//!
//! Architectural-programming elements are non-spatial (no x/y/z — `area`/`volume`/`height` are
//! target BANDS, not measured geometry), so `flat-position`/`bounds`-style derivations don't
//! apply; `elements[].parentId` is the one real structural relationship on the snapshot, so a
//! topology summary over it is the honest whole-snapshot derivation. Whole-snapshot scalar, not
//! per-entity, so this uses the plain `protocol::Inference<P>` shape (no `InferredField`/caching
//! machinery — see `🧭topology/🦀️component.rs` for the derivation).

use crate::artifacts::program::ProgramSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_topology, ProgramTopology};

//#region 🔖️DerivedComputeImports
/// 🧭️ Dissolved out of the former `⚙️engine` topic files (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every `fn(&ProgramSnapshot, ...) ->
/// Value`-style read-only projection that used to live on the artifact-tree engine hub. Mutating /
/// constructing counterparts (marked in each region below) moved to `crate::apps::architect`'s own
/// `//#region 🔧️Behavior` instead, since they take `&mut ProgramSnapshot`.
use crate::artifacts::program::kernel::{DiagnosticSeverity, EntityHeader, EntityId, LifecycleStatus, Priority, ProgramDiagnostic, PluginError};
use crate::artifacts::program::registers::{AdjacencyKind, AnalysisKind, AuditEvent, RelationshipKind, ReportKind, RiskLevel, SearchFilter, SeparationKind, ValidationStatus};
use crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA;
use semio_s_plugin_stdio::artifacts::csv as stdio_csv;
use semio_s_plugin_stdio::artifacts::tsv as stdio_tsv;
use semio_s_plugin_stdio::artifacts::tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_engine;
use semio_s_plugin_stdio::artifacts::tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_line_ending;
use std::collections::{HashMap, HashSet};
//#endregion 🔖️DerivedComputeImports

//#region 🔖️Inference
/// 💡️ Everything inferable from an architect program snapshot. One field per named inference
/// under `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.architect.program.inference")]
pub struct ProgramInference {
    #[derived]
    pub topology: ProgramTopology,
}

impl Inference<ProgramSnapshot> for ProgramInference {
    fn infer(snapshot: &ProgramSnapshot) -> Self {
        Self { topology: compute_topology(&snapshot.elements) }
    }
}

/// 🌉️ Hand impl (not derived): `ProgramTopology` has no meaningful `#[derive(Default)]` shape of
/// its own beyond the zero-element case it already matches (see its own `Default` impl) — this
/// exists only so `ProgramInference` itself has a `Default` without requiring `ProgramTopology` to
/// derive one, and to make the "default == infer(default snapshot)" law explicit at this level too.
impl Default for ProgramInference {
    fn default() -> Self {
        Self::infer(&ProgramSnapshot::default())
    }
}

impl protocol::InferenceSpec<ProgramSnapshot> for ProgramInference {
    fn inference_schema_id() -> &'static str {
        "s.architect.program.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.nodeCount", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.rootCount", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.maxDepth", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.cycleFree", reads: &["elements"] },
            protocol::InferenceFieldSpec { id: "s.architect.program.inference.topology.topoOrder", reads: &["elements"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::program::standards::v1::subsets::any::schema::ProgramBuilder {
    type Snapshot = ProgramSnapshot;
    type Inference = ProgramInference;

    /// 🎯️ Whole-snapshot scalar — nothing here is per-entity, so the cache/session are unused
    /// (same "plain `Inference`" shape the family doc calls out as correct for `dimensions`/
    /// `outline`/`bounds`-style facets).
    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = (cache, session);
        <ProgramInference as Inference<ProgramSnapshot>>::infer(snapshot)
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.architect.program.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `program_artifact_schema_descriptor`'s registration.
pub fn program_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.architect.program.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔀️AdjacencyViews
/// 🔢️ Dense lower-triangle adjacency matrix keyed by element id order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjacencyMatrix {
    pub element_ids: Vec<EntityId>,
    pub cells: Vec<Vec<Option<AdjacencyCell>>>,
}

/// 🟦️ One matrix cell summarizing the undirected link between two elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
//#region 🧪️AdjacencyTests
mod tests_adjacency {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn sample_plugin_matrix_has_one_cell() {
        let program = sample_plugin();
        let matrix = adjacency_matrix(&program);
        assert_eq!(matrix.element_ids.len(), 2);
        let populated: usize = matrix.cells.iter().flat_map(|row| row.iter()).filter(|cell| cell.is_some()).count();
        assert_eq!(populated, 1);
    }

    #[test]
    fn detects_distance_min_max_violation() {
        let mut program = sample_plugin();
        program.adjacencies[0].distance_min_m = Some(10.0);
        program.adjacencies[0].distance_max_m = Some(5.0);
        let conflicts = detect_adjacency_conflicts(&program);
        assert!(conflicts.iter().any(|c| c.message.contains("distance_min")));
    }
}
//#endregion 🧪️AdjacencyTests

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
    for e in &crate::artifacts::program::program_knowledge(program) {
        register("knowledge", &e.header.id, &e.header.name);
    }
    for e in &crate::artifacts::program::program_benchmarks(program) {
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

#[cfg(test)]
//#region 🧪️ValidateTests
mod tests_validate {
    use super::*;
    use crate::artifacts::program::kernel::EntityHeader;
    use crate::artifacts::program::{empty_plugin, sample_plugin};
    use crate::artifacts::program::registers::Requirement;

    #[test]
    fn sample_plugin_passes_validation() {
        let diagnostics = validate_plugin(&sample_plugin());
        assert!(diagnostics.iter().all(|d| d.severity != DiagnosticSeverity::Error));
    }

    #[test]
    fn empty_plugin_warns_on_title() {
        let diagnostics = validate_plugin(&empty_plugin());
        assert!(diagnostics.iter().any(|d| d.code == "meta.empty_title"));
    }

    #[test]
    fn detects_orphan_requirement() {
        let mut program = sample_plugin();
        program.requirements.push(Requirement {
            header: EntityHeader::new(EntityId::new_serial("requirement", "Orphan"), "Orphan"),
            code: "OR-1".into(),
            kind: crate::artifacts::program::registers::RequirementKind::Functional,
            statement: crate::artifacts::program::kernel::TextField::plain("orphan req"),
            rationale: None,
            source: None,
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
        let diagnostics = validate_plugin(&program);
        assert!(diagnostics.iter().any(|d| d.code == "requirement.orphan"));
    }

    #[test]
    fn detects_broken_relationship_target() {
        let mut program = sample_plugin();
        program.relationships.push(crate::artifacts::program::registers::Relationship {
            header: EntityHeader::new(EntityId::new_serial("relationship", "broken"), "broken"),
            source_id: program.elements[0].header.id.clone(),
            target_id: EntityId("missing-target".into()),
            kind: crate::artifacts::program::registers::RelationshipKind::DependsOn,
            strength: Some(1.0),
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
        let diagnostics = validate_plugin(&program);
        assert!(diagnostics.iter().any(|d| d.code == "relationship.missing_target"));
    }
}
//#endregion 🧪️ValidateTests

//#region 🎁️Outputs
/// 📦️ Abstract output kind for architectural program deliverables.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputKind {
    RequirementLists,
    FunctionalHierarchies,
    ActivityTaxonomies,
    RelationshipMatrices,
    AdjacencyMatrices,
    DependencyNetworks,
    PriorityMatrices,
    ResponsibilityMatrices,
    DecisionTrees,
    ProcessMaps,
    WorkflowDescriptions,
    UserJourneys,
    ScenarioNarratives,
    RiskMatrices,
    ComplianceMatrices,
    CapacitySchedules,
    EquipmentSchedules,
    EvaluationFrameworks,
    PerformanceSpecifications,
    ProgramReports,
}

/// 📄️ Structured abstract output payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramOutput {
    pub kind: OutputKind,
    pub title: String,
    pub lines: Vec<String>,
    pub entity_ids: Vec<EntityId>,
}

/// 🏗️ Builds the requested abstract output from a plugin snapshot.
pub fn build_output(program: &ProgramSnapshot, kind: OutputKind) -> ProgramOutput {
    match kind {
        OutputKind::RequirementLists => requirement_lists(program),
        OutputKind::FunctionalHierarchies => functional_hierarchies(program),
        OutputKind::ActivityTaxonomies => activity_taxonomies(program),
        OutputKind::RelationshipMatrices => relationship_matrices(program),
        OutputKind::AdjacencyMatrices => adjacency_matrices(program),
        OutputKind::DependencyNetworks => dependency_networks(program),
        OutputKind::PriorityMatrices => priority_matrices(program),
        OutputKind::ResponsibilityMatrices => responsibility_matrices(program),
        OutputKind::DecisionTrees => decision_trees(program),
        OutputKind::ProcessMaps => process_maps(program),
        OutputKind::WorkflowDescriptions => workflow_descriptions(program),
        OutputKind::UserJourneys => user_journeys(program),
        OutputKind::ScenarioNarratives => scenario_narratives(program),
        OutputKind::RiskMatrices => risk_matrices(program),
        OutputKind::ComplianceMatrices => compliance_matrices(program),
        OutputKind::CapacitySchedules => capacity_schedules(program),
        OutputKind::EquipmentSchedules => equipment_schedules(program),
        OutputKind::EvaluationFrameworks => evaluation_frameworks(program),
        OutputKind::PerformanceSpecifications => performance_specifications(program),
        OutputKind::ProgramReports => program_reports(program),
    }
}

fn requirement_lists(program: &ProgramSnapshot) -> ProgramOutput {
    ProgramOutput {
        kind: OutputKind::RequirementLists,
        title: "Requirement Lists".into(),
        lines: program.requirements.iter().map(|r| format!("[{:?}] {} — {}", r.kind, r.header.name, r.statement.text)).collect(),
        entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn functional_hierarchies(program: &ProgramSnapshot) -> ProgramOutput {
    let roots: Vec<_> = program.functions.iter().filter(|f| f.hierarchy_parent_id.is_none()).collect();
    let mut lines = Vec::new();
    for root in roots {
        lines.push(root.header.name.clone());
        for child in program.functions.iter().filter(|f| f.hierarchy_parent_id.as_ref() == Some(&root.header.id)) {
            lines.push(format!("  └️─️ {}", child.header.name));
        }
    }
    ProgramOutput { kind: OutputKind::FunctionalHierarchies, title: "Functional Hierarchies".into(), lines, entity_ids: program.functions.iter().map(|f| f.header.id.clone()).collect() }
}

fn activity_taxonomies(program: &ProgramSnapshot) -> ProgramOutput {
    let mut lines = Vec::new();
    for activity in &program.activities {
        lines.push(format!("{} / {} / {}", activity.category, activity.activity_type, activity.header.name));
    }
    ProgramOutput { kind: OutputKind::ActivityTaxonomies, title: "Activity Taxonomies".into(), lines, entity_ids: program.activities.iter().map(|a| a.header.id.clone()).collect() }
}

fn relationship_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.relationships.iter().map(|r| format!("{:?}: {} → {}", r.kind, r.source_id, r.target_id)).collect();
    ProgramOutput { kind: OutputKind::RelationshipMatrices, title: "Relationship Matrices".into(), lines, entity_ids: program.relationships.iter().map(|r| r.header.id.clone()).collect() }
}

fn adjacency_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let report = build_report(program, ReportKind::AdjacencyMatrix);
    ProgramOutput { kind: OutputKind::AdjacencyMatrices, title: "Adjacency Matrices".into(), lines: report.sections.into_iter().flat_map(|s| s.bullets).collect(), entity_ids: report.entity_ids }
}

fn dependency_networks(program: &ProgramSnapshot) -> ProgramOutput {
    let analysis = run_analysis(program, AnalysisKind::Dependency);
    ProgramOutput { kind: OutputKind::DependencyNetworks, title: "Dependency Networks".into(), lines: analysis.findings, entity_ids: analysis.entity_ids }
}

fn priority_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.priorities.iter().map(|p| format!("{} — {:?} rank {:?} weight {:?}", p.header.name, p.ranked_priority, p.rank, p.weight)).collect();
    ProgramOutput { kind: OutputKind::PriorityMatrices, title: "Priority Matrices".into(), lines, entity_ids: program.priorities.iter().map(|p| p.header.id.clone()).collect() }
}

fn responsibility_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.governance.responsibilities.iter().chain(program.governance.roles.iter()).cloned().collect();
    ProgramOutput { kind: OutputKind::ResponsibilityMatrices, title: "Responsibility Matrices".into(), lines, entity_ids: vec![program.governance.id.clone()] }
}

fn decision_trees(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.decisions.iter().map(|d| format!("{} → option {:?} ({})", d.header.name, d.selected_option_id, d.decision_statement.text)).collect();
    ProgramOutput { kind: OutputKind::DecisionTrees, title: "Decision Trees".into(), lines, entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect() }
}

fn process_maps(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.processes.iter().map(|p| format!("{}: {}", p.header.name, p.steps.join(" → "))).collect();
    ProgramOutput { kind: OutputKind::ProcessMaps, title: "Process Maps".into(), lines, entity_ids: program.processes.iter().map(|p| p.header.id.clone()).collect() }
}

fn workflow_descriptions(program: &ProgramSnapshot) -> ProgramOutput {
    let analysis = run_analysis(program, AnalysisKind::Workflow);
    ProgramOutput { kind: OutputKind::WorkflowDescriptions, title: "Workflow Descriptions".into(), lines: analysis.findings, entity_ids: analysis.entity_ids }
}

fn user_journeys(program: &ProgramSnapshot) -> ProgramOutput {
    let mut lines = Vec::new();
    for user in &program.users {
        let activities: Vec<_> = program.activities.iter().filter(|a| a.user_profile_ids.contains(&user.header.id)).map(|a| a.header.name.as_str()).collect();
        lines.push(format!("{}: {}", user.header.name, activities.join(" → ")));
    }
    ProgramOutput { kind: OutputKind::UserJourneys, title: "User Journeys".into(), lines, entity_ids: program.users.iter().map(|u| u.header.id.clone()).collect() }
}

fn scenario_narratives(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.scenarios.iter().map(|s| format!("{} — {}", s.header.name, s.hypothesis.text)).collect();
    ProgramOutput { kind: OutputKind::ScenarioNarratives, title: "Scenario Narratives".into(), lines, entity_ids: program.scenarios.iter().map(|s| s.header.id.clone()).collect() }
}

fn risk_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.risks.iter().map(|r| format!("{} — {:?}/{:?}", r.header.name, r.probability, r.impact)).collect();
    ProgramOutput { kind: OutputKind::RiskMatrices, title: "Risk Matrices".into(), lines, entity_ids: program.risks.iter().map(|r| r.header.id.clone()).collect() }
}

fn compliance_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.regulatory.iter().map(|r| format!("{} {} — {:?}", r.code, r.title, r.verification_status)).collect();
    ProgramOutput { kind: OutputKind::ComplianceMatrices, title: "Compliance Matrices".into(), lines, entity_ids: program.regulatory.iter().map(|r| r.header.id.clone()).collect() }
}

fn capacity_schedules(program: &ProgramSnapshot) -> ProgramOutput {
    let analysis = run_analysis(program, AnalysisKind::Capacity);
    let schedule_lines: Vec<String> = program.schedules.iter().map(|s| s.header.name.clone()).collect();
    ProgramOutput { kind: OutputKind::CapacitySchedules, title: "Capacity Schedules".into(), lines: analysis.findings.into_iter().chain(schedule_lines).collect(), entity_ids: program.schedules.iter().map(|s| s.header.id.clone()).collect() }
}

fn equipment_schedules(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.equipment.iter().map(|e| format!("{} — qty {:?}", e.header.name, e.quantity.target)).collect();
    ProgramOutput { kind: OutputKind::EquipmentSchedules, title: "Equipment Schedules".into(), lines, entity_ids: program.equipment.iter().map(|e| e.header.id.clone()).collect() }
}

fn evaluation_frameworks(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.performance.iter().map(|p| format!("{} — {}", p.header.name, p.criterion)).collect();
    ProgramOutput { kind: OutputKind::EvaluationFrameworks, title: "Evaluation Frameworks".into(), lines, entity_ids: program.performance.iter().map(|p| p.header.id.clone()).collect() }
}

fn performance_specifications(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.performance.iter().map(|p| format!("{} target {:?} {:?}", p.header.name, p.target, p.unit)).collect();
    ProgramOutput { kind: OutputKind::PerformanceSpecifications, title: "Performance Specifications".into(), lines, entity_ids: program.performance.iter().map(|p| p.header.id.clone()).collect() }
}

fn program_reports(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.reports.iter().map(|r| format!("{:?} — {}", r.kind, r.title)).collect();
    ProgramOutput { kind: OutputKind::ProgramReports, title: "ProgramSnapshot Reports".into(), lines, entity_ids: program.reports.iter().map(|r| r.header.id.clone()).collect() }
}
//#endregion 🎁️Outputs

#[cfg(test)]
//#region 🧪️OutputsTests
mod tests_outputs {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn requirement_lists_output_nonempty_for_sample() {
        let output = build_output(&sample_plugin(), OutputKind::RequirementLists);
        assert_eq!(output.kind, OutputKind::RequirementLists);
    }

    #[test]
    fn adjacency_matrices_output_uses_matrix_cells() {
        let output = build_output(&sample_plugin(), OutputKind::AdjacencyMatrices);
        assert!(!output.lines.is_empty());
    }
}
//#endregion 🧪️OutputsTests

//#region 📄️Report
/// 📑️ Structured report payload for export and program rendering.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramReport {
    pub kind: ReportKind,
    pub title: String,
    pub generated_at: String,
    pub sections: Vec<ReportSection>,
    pub entity_ids: Vec<EntityId>,
}

/// 📎️ One titled section within a plugin report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportSection {
    pub heading: String,
    pub body: String,
    pub bullets: Vec<String>,
}

/// 🖨️ Builds a structured report for the requested kind.
pub fn build_report(program: &ProgramSnapshot, kind: ReportKind) -> ProgramReport {
    match kind {
        ReportKind::ExecutiveSummary => executive_summary(program),
        ReportKind::ProgramOverview => program_overview(program),
        ReportKind::StakeholderSummary => stakeholder_summary(program),
        ReportKind::RequirementsMatrix => requirements_matrix(program),
        ReportKind::AdjacencyMatrix => adjacency_matrix_report(program),
        ReportKind::GapAnalysis => gap_report(program),
        ReportKind::RiskRegister => risk_register(program),
        ReportKind::DecisionLog => decision_log(program),
        ReportKind::ValidationSummary => validation_summary(program),
        ReportKind::Recommendation => recommendation(program),
        ReportKind::UserSummary => user_summary(program),
        ReportKind::FunctionalSummary => functional_summary(program),
        ReportKind::CapacitySummary => capacity_summary(program),
        ReportKind::WorkflowSummary => workflow_summary(program),
        ReportKind::ComplianceSummary => compliance_summary(program),
        ReportKind::CostSummary => cost_summary(program),
        ReportKind::ScheduleSummary => schedule_summary(program),
        ReportKind::ChangeSummary => change_summary(program),
        ReportKind::OpenIssueSummary => open_issue_summary(program),
        ReportKind::PrioritySummary => priority_summary(program),
        ReportKind::ScenarioSummary => scenario_summary(program),
    }
}

fn timestamp(program: &ProgramSnapshot) -> String {
    program.meta.timestamps.updated.clone()
}

fn executive_summary(program: &ProgramSnapshot) -> ProgramReport {
    let summary = status_summary(program);
    ProgramReport {
        kind: ReportKind::ExecutiveSummary,
        title: program.meta.title.clone(),
        generated_at: timestamp(program),
        sections: vec![
            ReportSection {
                heading: "Overview".into(),
                body: program.meta.purpose.text.clone(),
                bullets: vec![format!("{} elements", program.elements.len()), format!("{} requirements", program.requirements.len()), format!("{} stakeholders", program.stakeholders.len())],
            },
            ReportSection { heading: "Status".into(), body: format!("{} total entities tracked", summary.total_entities), bullets: summary.by_status.iter().map(|(status, count)| format!("{status:?}: {count}")).collect() },
        ],
        entity_ids: Vec::new(),
    }
}

fn program_overview(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ProgramOverview,
        title: format!("{} — Overview", program.meta.title),
        generated_at: timestamp(program),
        sections: vec![
            ReportSection { heading: "Project".into(), body: program.project.brief_summary.text.clone(), bullets: program.project.objectives.clone() },
            ReportSection { heading: "Scope".into(), body: format!("{} inclusions, {} exclusions", program.project.scope_inclusions.len(), program.project.scope_exclusions.len()), bullets: program.project.deliverables.clone() },
        ],
        entity_ids: vec![program.project.id.clone()],
    }
}

fn stakeholder_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::StakeholderSummary,
        title: "Stakeholder Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Stakeholders".into(),
            body: format!("{} stakeholder(s)", program.stakeholders.len()),
            bullets: program.stakeholders.iter().map(|s| format!("{} — {} ({:?}/{:?})", s.header.name, s.role, s.influence, s.engagement)).collect(),
        }],
        entity_ids: program.stakeholders.iter().map(|s| s.header.id.clone()).collect(),
    }
}

fn requirements_matrix(program: &ProgramSnapshot) -> ProgramReport {
    let element_names: Vec<String> = program.elements.iter().map(|e| e.header.name.clone()).collect();
    let header = format!("{}\t{}", "Requirement", element_names.join("\t"));
    let mut rows = vec![header];
    for requirement in &program.requirements {
        let cells: Vec<String> = program.elements.iter().map(|element| if requirement.element_ids.contains(&element.header.id) { "X".into() } else { "-".into() }).collect();
        rows.push(format!("{}\t{}", requirement.header.name, cells.join("\t")));
    }
    ProgramReport {
        kind: ReportKind::RequirementsMatrix,
        title: "Requirements Matrix".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Requirement × Element Grid".into(), body: format!("{}×{} matrix", program.requirements.len(), program.elements.len()), bullets: rows }],
        entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn adjacency_matrix_report(program: &ProgramSnapshot) -> ProgramReport {
    let matrix = adjacency_matrix(program);
    let header: String = format!("{}\t{}", "", matrix.element_ids.iter().map(|id| program.elements.iter().find(|e| &e.header.id == id).map_or(id.0.as_str(), |e| e.header.name.as_str())).collect::<Vec<_>>().join("\t"));
    let mut rows = vec![header];
    for (row_idx, row_id) in matrix.element_ids.iter().enumerate() {
        let name = program.elements.iter().find(|e| &e.header.id == row_id).map_or(row_id.0.as_str(), |e| e.header.name.as_str());
        let cells: Vec<String> = (0..matrix.element_ids.len())
            .map(|col_idx| {
                if row_idx == col_idx {
                    return ".".into();
                }
                let (r, c) = if row_idx > col_idx { (row_idx, col_idx) } else { (col_idx, row_idx) };
                matrix.cells[r][c].as_ref().map_or_else(|| "-".into(), |cell| format!("{:?}/{:.1}", cell.kind, cell.weight))
            })
            .collect();
        rows.push(format!("{name}\t{}", cells.join("\t")));
    }
    ProgramReport {
        kind: ReportKind::AdjacencyMatrix,
        title: "Adjacency Matrix".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Adjacency Cells".into(), body: format!("{}×{} element matrix", matrix.element_ids.len(), matrix.element_ids.len()), bullets: rows }],
        entity_ids: matrix.element_ids,
    }
}

fn gap_report(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Gap);
    ProgramReport {
        kind: ReportKind::GapAnalysis,
        title: "Gap Analysis".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: analysis.entity_ids,
    }
}

fn risk_register(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::RiskRegister,
        title: "Risk Register".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Risks".into(), body: format!("{} risk(s)", program.risks.len()), bullets: program.risks.iter().map(|r| format!("{} — {:?}/{:?}", r.header.name, r.probability, r.impact)).collect() }],
        entity_ids: program.risks.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn decision_log(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::DecisionLog,
        title: "Decision Log".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Decisions".into(),
            body: format!("{} decision(s)", program.decisions.len()),
            bullets: program.decisions.iter().map(|d| format!("{} — {:?} ({})", d.header.name, d.approval_status, d.decision_statement.text)).collect(),
        }],
        entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect(),
    }
}

fn validation_summary(program: &ProgramSnapshot) -> ProgramReport {
    let diagnostics = validate_plugin(program);
    ProgramReport {
        kind: ReportKind::ValidationSummary,
        title: "Validation Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Diagnostics".into(), body: format!("{} diagnostic(s)", diagnostics.len()), bullets: diagnostics.iter().map(|d| format!("[{:?}] {}: {}", d.severity, d.code, d.message)).collect() }],
        entity_ids: Vec::new(),
    }
}

fn recommendation(program: &ProgramSnapshot) -> ProgramReport {
    let gap = run_analysis(program, AnalysisKind::Gap);
    let conflict = run_analysis(program, AnalysisKind::Conflict);
    ProgramReport {
        kind: ReportKind::Recommendation,
        title: "Recommendations".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Gaps".into(), body: gap.summary, bullets: gap.findings }, ReportSection { heading: "Conflicts".into(), body: conflict.summary, bullets: conflict.findings }],
        entity_ids: Vec::new(),
    }
}

fn user_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::UserSummary,
        title: "User Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "User Profiles".into(), body: format!("{} user profile(s)", program.users.len()), bullets: program.users.iter().map(|u| format!("{} — {:?}", u.header.name, u.category)).collect() }],
        entity_ids: program.users.iter().map(|u| u.header.id.clone()).collect(),
    }
}

fn functional_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::FunctionalSummary,
        title: "Functional Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Functions".into(),
            body: format!("{} function(s), {} activit(ies)", program.functions.len(), program.activities.len()),
            bullets: program.functions.iter().map(|f| format!("{} — {:?} ({})", f.header.name, f.kind, f.purpose.text)).collect(),
        }],
        entity_ids: program.functions.iter().map(|f| f.header.id.clone()).collect(),
    }
}

fn capacity_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Capacity);
    ProgramReport {
        kind: ReportKind::CapacitySummary,
        title: "Capacity Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: Vec::new(),
    }
}

fn workflow_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Workflow);
    ProgramReport {
        kind: ReportKind::WorkflowSummary,
        title: "Workflow Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: analysis.entity_ids,
    }
}

fn compliance_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ComplianceSummary,
        title: "Compliance Summary".into(),
        generated_at: timestamp(program),
        sections: vec![
            ReportSection { heading: "Regulatory".into(), body: format!("{} regulatory requirement(s)", program.regulatory.len()), bullets: program.regulatory.iter().map(|r| r.header.name.clone()).collect() },
            ReportSection { heading: "Validations".into(), body: format!("{} validation record(s)", program.validations.len()), bullets: program.validations.iter().map(|v| format!("{} — {:?}", v.header.name, v.result)).collect() },
        ],
        entity_ids: program.regulatory.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn cost_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Cost);
    ProgramReport {
        kind: ReportKind::CostSummary,
        title: "Cost Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: Vec::new(),
    }
}

fn schedule_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ScheduleSummary,
        title: "Schedule Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Schedules".into(),
            body: format!("{} schedule requirement(s), {} delivery constraints", program.schedules.len(), program.delivery.len()),
            bullets: program.schedules.iter().map(|s| s.header.name.clone()).collect(),
        }],
        entity_ids: program.schedules.iter().map(|s| s.header.id.clone()).collect(),
    }
}

fn change_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ChangeSummary,
        title: "Change Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Changes".into(), body: format!("{} change record(s)", program.changes.len()), bullets: program.changes.iter().map(|c| format!("{} — {}", c.header.name, c.header.timestamps.updated)).collect() }],
        entity_ids: program.changes.iter().map(|c| c.header.id.clone()).collect(),
    }
}

fn open_issue_summary(program: &ProgramSnapshot) -> ProgramReport {
    let open: Vec<_> = program.issues.iter().filter(|i| !matches!(i.header.status, crate::artifacts::program::kernel::LifecycleStatus::Closed | crate::artifacts::program::kernel::LifecycleStatus::Complete)).collect();
    ProgramReport {
        kind: ReportKind::OpenIssueSummary,
        title: "Open Issue Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Open Issues".into(),
            body: format!("{} open of {} total issues", open.len(), program.issues.len()),
            bullets: open.iter().map(|i| format!("{} — {:?}/{:?}", i.header.name, i.severity, i.issue_priority)).collect(),
        }],
        entity_ids: open.iter().map(|i| i.header.id.clone()).collect(),
    }
}

fn priority_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::PrioritySummary,
        title: "Priority Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Priorities".into(),
            body: format!("{} priority record(s)", program.priorities.len()),
            bullets: program.priorities.iter().map(|p| format!("{} — {:?} weight {:?}", p.header.name, p.ranked_priority, p.weight)).collect(),
        }],
        entity_ids: program.priorities.iter().map(|p| p.header.id.clone()).collect(),
    }
}

fn scenario_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Scenario);
    ProgramReport {
        kind: ReportKind::ScenarioSummary,
        title: "Scenario Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: analysis.entity_ids,
    }
}
//#endregion 📄️Report

#[cfg(test)]
//#region 🧪️ReportTests
mod tests_report {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn executive_summary_includes_counts() {
        let report = build_report(&sample_plugin(), ReportKind::ExecutiveSummary);
        assert_eq!(report.kind, ReportKind::ExecutiveSummary);
        assert!(!report.sections.is_empty());
    }

    #[test]
    fn requirements_matrix_has_grid_rows() {
        let report = build_report(&sample_plugin(), ReportKind::RequirementsMatrix);
        assert!(!report.sections[0].bullets.is_empty());
        assert!(report.sections[0].bullets[0].contains('\t'));
    }
}
//#endregion 🧪️ReportTests

//#region 📊️StatusSummary
/// 📈️ Aggregated status histogram across all header-bearing registers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSummary {
    pub total_entities: usize,
    pub by_status: Vec<(LifecycleStatus, usize)>,
    pub by_register: Vec<RegisterStatusCount>,
    pub compliance_status: Vec<(ValidationStatus, usize)>,
    pub validation_status: Vec<(ValidationStatus, usize)>,
    pub decision_status: Vec<(ValidationStatus, usize)>,
    pub action_status: Vec<(LifecycleStatus, usize)>,
}

/// 📁️ Per-register entity count and dominant status.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStatusCount {
    pub register: String,
    pub count: usize,
    pub draft_count: usize,
    pub approved_count: usize,
}

fn bump_status(tallies: &mut Vec<(LifecycleStatus, usize)>, status: LifecycleStatus) {
    if let Some((_, count)) = tallies.iter_mut().find(|(s, _)| *s == status) {
        *count += 1;
    } else {
        tallies.push((status, 1));
    }
}

fn bump_validation(tallies: &mut Vec<(ValidationStatus, usize)>, status: ValidationStatus) {
    if let Some((_, count)) = tallies.iter_mut().find(|(s, _)| *s == status) {
        *count += 1;
    } else {
        tallies.push((status, 1));
    }
}

/// 🧮️ Aggregates lifecycle status counts from every program register collection.
pub fn status_summary(program: &ProgramSnapshot) -> StatusSummary {
    let mut tallies: Vec<(LifecycleStatus, usize)> = Vec::new();
    let mut registers = Vec::new();
    let mut total = 0usize;

    let mut collect = |name: &str, headers: Vec<&EntityHeader>| {
        let count = headers.len();
        total += count;
        let draft_count = headers.iter().filter(|h| h.status == LifecycleStatus::Draft).count();
        let approved_count = headers.iter().filter(|h| h.status == LifecycleStatus::Approved).count();
        for header in headers {
            bump_status(&mut tallies, header.status);
        }
        registers.push(RegisterStatusCount { register: name.into(), count, draft_count, approved_count });
    };

    collect("stakeholders", program.stakeholders.iter().map(|e| &e.header).collect());
    collect("users", program.users.iter().map(|e| &e.header).collect());
    collect("activities", program.activities.iter().map(|e| &e.header).collect());
    collect("functions", program.functions.iter().map(|e| &e.header).collect());
    collect("elements", program.elements.iter().map(|e| &e.header).collect());
    collect("quantities", program.quantities.iter().map(|e| &e.header).collect());
    collect("relationships", program.relationships.iter().map(|e| &e.header).collect());
    collect("adjacencies", program.adjacencies.iter().map(|e| &e.header).collect());
    collect("processes", program.processes.iter().map(|e| &e.header).collect());
    collect("flows", program.flows.iter().map(|e| &e.header).collect());
    collect("access_rules", program.access_rules.iter().map(|e| &e.header).collect());
    collect("operations", program.operations.iter().map(|e| &e.header).collect());
    collect("equipment", program.equipment.iter().map(|e| &e.header).collect());
    collect("resources", program.resources.iter().map(|e| &e.header).collect());
    collect("storage", program.storage.iter().map(|e| &e.header).collect());
    collect("environmental", program.environmental.iter().map(|e| &e.header).collect());
    collect("human_factors", program.human_factors.iter().map(|e| &e.header).collect());
    collect("accessibility", program.accessibility.iter().map(|e| &e.header).collect());
    collect("privacy", program.privacy.iter().map(|e| &e.header).collect());
    collect("safety", program.safety.iter().map(|e| &e.header).collect());
    collect("security", program.security.iter().map(|e| &e.header).collect());
    collect("regulatory", program.regulatory.iter().map(|e| &e.header).collect());
    collect("site_context", program.site_context.iter().map(|e| &e.header).collect());
    collect("organizational", program.organizational.iter().map(|e| &e.header).collect());
    collect("services", program.services.iter().map(|e| &e.header).collect());
    collect("infrastructure", program.infrastructure.iter().map(|e| &e.header).collect());
    collect("information", program.information.iter().map(|e| &e.header).collect());
    collect("communication", program.communication.iter().map(|e| &e.header).collect());
    collect("wayfinding", program.wayfinding.iter().map(|e| &e.header).collect());
    collect("schedules", program.schedules.iter().map(|e| &e.header).collect());
    collect("flexibility", program.flexibility.iter().map(|e| &e.header).collect());
    collect("growth", program.growth.iter().map(|e| &e.header).collect());
    collect("sustainability", program.sustainability.iter().map(|e| &e.header).collect());
    collect("resilience", program.resilience.iter().map(|e| &e.header).collect());
    collect("costs", program.costs.iter().map(|e| &e.header).collect());
    collect("delivery", program.delivery.iter().map(|e| &e.header).collect());
    collect("risks", program.risks.iter().map(|e| &e.header).collect());
    collect("conflicts", program.conflicts.iter().map(|e| &e.header).collect());
    collect("requirements", program.requirements.iter().map(|e| &e.header).collect());
    collect("priorities", program.priorities.iter().map(|e| &e.header).collect());
    collect("scenarios", program.scenarios.iter().map(|e| &e.header).collect());
    collect("options", program.options.iter().map(|e| &e.header).collect());
    collect("decisions", program.decisions.iter().map(|e| &e.header).collect());
    collect("validations", program.validations.iter().map(|e| &e.header).collect());
    collect("performance", program.performance.iter().map(|e| &e.header).collect());
    collect("quality", program.quality.iter().map(|e| &e.header).collect());
    collect("documents", program.artifacts.iter().map(|e| &e.header).collect());
    collect("changes", program.changes.iter().map(|e| &e.header).collect());
    collect("collaboration", program.collaboration.iter().map(|e| &e.header).collect());
    collect("analyses", program.analyses.iter().map(|e| &e.header).collect());
    collect("reports", program.reports.iter().map(|e| &e.header).collect());
    collect("search_filters", program.search_filters.iter().map(|e| &e.header).collect());
    collect("status_records", program.status_records.iter().map(|e| &e.header).collect());
    collect("workshops", program.workshops.iter().map(|e| &e.header).collect());
    collect("surveys", program.surveys.iter().map(|e| &e.header).collect());
    collect("issues", program.issues.iter().map(|e| &e.header).collect());
    collect("audit_events", program.audit_events.iter().map(|e| &e.header).collect());
    collect("templates", program.templates.iter().map(|e| &e.header).collect());
    let knowledge_records = crate::artifacts::program::program_knowledge(program);
    collect("knowledge", knowledge_records.iter().map(|e| &e.header).collect());
    let benchmark_records = crate::artifacts::program::program_benchmarks(program);
    collect("benchmarks", benchmark_records.iter().map(|e| &e.header).collect());

    let mut compliance_status = Vec::new();
    for item in &program.regulatory {
        bump_validation(&mut compliance_status, item.verification_status);
    }
    for item in &program.reports {
        bump_validation(&mut compliance_status, item.approval_status);
    }

    let mut validation_status = Vec::new();
    for item in &program.requirements {
        bump_validation(&mut validation_status, item.validation_status);
    }
    for item in &program.validations {
        bump_validation(&mut validation_status, item.result);
    }

    let mut decision_status = Vec::new();
    for item in &program.decisions {
        bump_validation(&mut decision_status, item.approval_status);
    }

    let mut action_status = Vec::new();
    for item in &program.status_records {
        bump_status(&mut action_status, item.record_status);
    }
    for item in &program.issues {
        bump_status(&mut action_status, item.header.status);
    }

    tallies.sort_by_key(|(status, _)| format!("{status:?}"));

    StatusSummary { total_entities: total, by_status: tallies, by_register: registers, compliance_status, validation_status, decision_status, action_status }
}
//#endregion 📊️StatusSummary

#[cfg(test)]
//#region 🧪️StatusSummaryTests
mod tests_status_summary {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn sample_plugin_status_summary_counts_elements() {
        let summary = status_summary(&sample_plugin());
        assert!(summary.total_entities >= 2);
        let elements = summary.by_register.iter().find(|r| r.register == "elements").expect("elements");
        assert_eq!(elements.count, 2);
    }

    #[test]
    fn status_summary_includes_all_major_registers() {
        let summary = status_summary(&sample_plugin());
        for register in ["elements", "stakeholders", "adjacencies", "status_records"] {
            assert!(summary.by_register.iter().any(|r| r.register == register));
        }
    }
}
//#endregion 🧪️StatusSummaryTests

//#region 🔍️Search
/// 🎯️ Ad-hoc search query with optional structured filters.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub owner_ids: Vec<EntityId>,
    #[serde(default)]
    pub statuses: Vec<LifecycleStatus>,
    #[serde(default)]
    pub priorities: Vec<Priority>,
    #[serde(default)]
    pub entity_kinds: Vec<String>,
    #[serde(default)]
    pub tag_filters: Vec<String>,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
}

/// 📌️ One search hit with register kind and display name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub register: String,
    pub entity_id: EntityId,
    pub name: String,
    pub score: f64,
}

/// 🔎️ Searches all registers; uses `filter` when provided; records query in `search_history`.
pub fn search_plugin(program: &ProgramSnapshot, query: &SearchQuery, filter: Option<&SearchFilter>, search_history: Option<&mut Vec<SearchQuery>>) -> Vec<SearchHit> {
    let effective = merge_query(query, filter);
    if let Some(history) = search_history {
        history.push(effective.clone());
    }
    let mut hits = Vec::new();
    macro_rules! search_register {
        ($register:literal, $collection:expr) => {
            for item in $collection {
                push_if_match(&mut hits, $register, &item.header, &effective);
            }
        };
    }
    search_register!("stakeholders", &program.stakeholders);
    search_register!("users", &program.users);
    search_register!("activities", &program.activities);
    search_register!("functions", &program.functions);
    search_register!("elements", &program.elements);
    search_register!("quantities", &program.quantities);
    search_register!("relationships", &program.relationships);
    search_register!("adjacencies", &program.adjacencies);
    search_register!("processes", &program.processes);
    search_register!("flows", &program.flows);
    search_register!("access_rules", &program.access_rules);
    search_register!("operations", &program.operations);
    search_register!("equipment", &program.equipment);
    search_register!("resources", &program.resources);
    search_register!("storage", &program.storage);
    search_register!("environmental", &program.environmental);
    search_register!("human_factors", &program.human_factors);
    search_register!("accessibility", &program.accessibility);
    search_register!("privacy", &program.privacy);
    search_register!("safety", &program.safety);
    search_register!("security", &program.security);
    search_register!("regulatory", &program.regulatory);
    search_register!("site_context", &program.site_context);
    search_register!("organizational", &program.organizational);
    search_register!("services", &program.services);
    search_register!("infrastructure", &program.infrastructure);
    search_register!("information", &program.information);
    search_register!("communication", &program.communication);
    search_register!("wayfinding", &program.wayfinding);
    search_register!("schedules", &program.schedules);
    search_register!("flexibility", &program.flexibility);
    search_register!("growth", &program.growth);
    search_register!("sustainability", &program.sustainability);
    search_register!("resilience", &program.resilience);
    search_register!("costs", &program.costs);
    search_register!("delivery", &program.delivery);
    search_register!("risks", &program.risks);
    search_register!("conflicts", &program.conflicts);
    search_register!("requirements", &program.requirements);
    search_register!("priorities", &program.priorities);
    search_register!("scenarios", &program.scenarios);
    search_register!("options", &program.options);
    search_register!("decisions", &program.decisions);
    search_register!("validations", &program.validations);
    search_register!("performance", &program.performance);
    search_register!("quality", &program.quality);
    search_register!("documents", &program.artifacts);
    search_register!("changes", &program.changes);
    search_register!("collaboration", &program.collaboration);
    search_register!("analyses", &program.analyses);
    search_register!("reports", &program.reports);
    search_register!("search_filters", &program.search_filters);
    search_register!("status_records", &program.status_records);
    search_register!("workshops", &program.workshops);
    search_register!("surveys", &program.surveys);
    search_register!("issues", &program.issues);
    search_register!("audit_events", &program.audit_events);
    search_register!("templates", &program.templates);
    search_register!("knowledge", &crate::artifacts::program::program_knowledge(program));
    search_register!("benchmarks", &crate::artifacts::program::program_benchmarks(program));
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits
}

fn merge_query(query: &SearchQuery, filter: Option<&SearchFilter>) -> SearchQuery {
    let Some(filter) = filter else {
        return query.clone();
    };
    SearchQuery {
        keywords: if filter.keywords.is_empty() { query.keywords.clone() } else { filter.keywords.clone() },
        categories: if filter.categories.is_empty() { query.categories.clone() } else { filter.categories.clone() },
        owner_ids: if filter.owner_ids.is_empty() { query.owner_ids.clone() } else { filter.owner_ids.clone() },
        statuses: if filter.statuses.is_empty() { query.statuses.clone() } else { filter.statuses.clone() },
        priorities: if filter.priorities.is_empty() { query.priorities.clone() } else { filter.priorities.clone() },
        entity_kinds: if filter.entity_kinds.is_empty() { query.entity_kinds.clone() } else { filter.entity_kinds.clone() },
        tag_filters: if filter.tag_filters.is_empty() { query.tag_filters.clone() } else { filter.tag_filters.clone() },
        sources: if filter.sources.is_empty() { query.sources.clone() } else { filter.sources.clone() },
        date_from: filter.date_from.clone().or(query.date_from.clone()),
        date_to: filter.date_to.clone().or(query.date_to.clone()),
    }
}

fn push_if_match(hits: &mut Vec<SearchHit>, register: &str, header: &EntityHeader, query: &SearchQuery) {
    if !query.statuses.is_empty() && !query.statuses.contains(&header.status) {
        return;
    }
    if !query.priorities.is_empty() && !query.priorities.contains(&header.priority) {
        return;
    }
    if let Some(owner) = &header.ownership.owner_id {
        if !query.owner_ids.is_empty() && !query.owner_ids.contains(owner) {
            return;
        }
    }
    if !query.entity_kinds.is_empty() && !query.entity_kinds.iter().any(|k| k == register) {
        return;
    }
    if !query.tag_filters.is_empty() && !query.tag_filters.iter().any(|t| header.tags.contains(t)) {
        return;
    }
    if !query.categories.is_empty() && !query.categories.iter().any(|c| header.tags.contains(c) || header.name.contains(c)) {
        return;
    }
    if let Some(from) = &query.date_from {
        if header.timestamps.updated < *from {
            return;
        }
    }
    if let Some(to) = &query.date_to {
        if header.timestamps.updated > *to {
            return;
        }
    }
    if !query.sources.is_empty() {
        let source_match = header.notes.iter().any(|n| query.sources.iter().any(|s| n.tag.contains(s) || n.text.contains(s))) || header.tags.iter().any(|t| query.sources.contains(t));
        if !source_match {
            return;
        }
    }
    let mut score = 0.0;
    let haystack = format!("{} {} {:?}", header.name, header.description.as_ref().map_or("", |d| d.text.as_str()), header.tags).to_lowercase();
    for keyword in &query.keywords {
        if haystack.contains(&keyword.to_lowercase()) {
            score += 1.0;
        }
    }
    if query.keywords.is_empty() || score > 0.0 {
        hits.push(SearchHit { register: register.into(), entity_id: header.id.clone(), name: header.name.clone(), score: if score == 0.0 { 0.1 } else { score } });
    }
}
//#endregion 🔍️Search

#[cfg(test)]
//#region 🧪️SearchTests
mod tests_search {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn search_finds_reception_element() {
        let hits = search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Reception".into()], ..Default::default() }, None, None);
        assert!(hits.iter().any(|h| h.name == "Reception"));
    }

    #[test]
    fn search_history_records_query() {
        let mut history = Vec::new();
        search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Waiting".into()], ..Default::default() }, None, Some(&mut history));
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].keywords, vec!["Waiting".to_string()]);
    }

    #[test]
    fn entity_kind_filter_limits_registers() {
        let hits = search_plugin(&sample_plugin(), &SearchQuery { entity_kinds: vec!["elements".into()], ..Default::default() }, None, None);
        assert!(hits.iter().all(|h| h.register == "elements"));
    }
}
//#endregion 🧪️SearchTests

//#region 🔬️Analyze
/// 📈️ Structured output from `run_analysis`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResult {
    pub kind: AnalysisKind,
    pub title: String,
    pub summary: String,
    pub findings: Vec<String>,
    pub metrics: Vec<AnalysisMetric>,
    pub diagnostics: Vec<ProgramDiagnostic>,
    pub entity_ids: Vec<EntityId>,
}

/// 📊️ Named numeric metric from an analysis run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisMetric {
    pub name: String,
    pub value: f64,
    pub unit: Option<String>,
}

/// 🧮️ Runs the requested analysis kind over a plugin snapshot.
pub fn run_analysis(program: &ProgramSnapshot, kind: AnalysisKind) -> AnalysisResult {
    match kind {
        AnalysisKind::Gap => analyze_gap(program),
        AnalysisKind::Conflict => analyze_conflict(program),
        AnalysisKind::Dependency => analyze_dependency(program),
        AnalysisKind::Capacity => analyze_capacity(program),
        AnalysisKind::Demand => analyze_demand(program),
        AnalysisKind::Utilization => analyze_utilization(program),
        AnalysisKind::Workflow => analyze_workflow(program),
        AnalysisKind::Risk => analyze_risk(program),
        AnalysisKind::Cost => analyze_cost(program),
        AnalysisKind::Scenario => analyze_scenario(program),
        AnalysisKind::Sensitivity => analyze_sensitivity(program),
        AnalysisKind::Impact => analyze_impact(program),
        AnalysisKind::Trend => analyze_trend(program),
        AnalysisKind::RequirementComparison => analyze_requirement_comparison(program),
        AnalysisKind::RequirementClustering => analyze_requirement_clustering(program),
        AnalysisKind::RequirementFiltering => analyze_requirement_filtering(program),
        AnalysisKind::RequirementSorting => analyze_requirement_sorting(program),
        AnalysisKind::RequirementScoring => analyze_requirement_scoring(program),
        AnalysisKind::RequirementWeighting => analyze_requirement_weighting(program),
        AnalysisKind::RelationshipAnalysis => analyze_relationship(program),
    }
}

fn analyze_gap(program: &ProgramSnapshot) -> AnalysisResult {
    let mut findings = Vec::new();
    if program.requirements.is_empty() {
        findings.push("no requirements registered".into());
    }
    if program.elements.is_empty() {
        findings.push("no program elements defined".into());
    }
    let unlinked: Vec<_> = program.requirements.iter().filter(|req| req.element_ids.is_empty() && req.function_ids.is_empty()).map(|req| req.header.id.clone()).collect();
    for id in &unlinked {
        findings.push(format!("requirement {id} is not linked to elements or functions"));
    }
    let elements_without_functions: Vec<_> = program.elements.iter().filter(|e| e.function_ids.is_empty()).map(|e| e.header.id.clone()).collect();
    for id in &elements_without_functions {
        findings.push(format!("element {id} has no assigned functions"));
    }
    AnalysisResult {
        kind: AnalysisKind::Gap,
        title: "Gap Analysis".into(),
        summary: format!("{} gap finding(s)", findings.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "unlinked_requirements".into(), value: unlinked.len() as f64, unit: None }, AnalysisMetric { name: "elements_without_functions".into(), value: elements_without_functions.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: unlinked,
    }
}

fn analyze_conflict(program: &ProgramSnapshot) -> AnalysisResult {
    let adjacency_conflicts = detect_adjacency_conflicts(program);
    let mut findings: Vec<String> = adjacency_conflicts.iter().map(|c| format!("{}: {}", c.adjacency_a_id, c.message)).collect();
    findings.extend(program.conflicts.iter().map(|c| format!("{} — {:?} between {} and {}", c.header.name, c.kind, c.entity_a_id, c.entity_b_id)));
    let open_conflicts = program.conflicts.iter().filter(|c| c.resolution_status != ValidationStatus::Passed).count();
    let findings_len = findings.len();
    AnalysisResult {
        kind: AnalysisKind::Conflict,
        title: "Conflict Analysis".into(),
        summary: format!("{findings_len} conflict(s) detected, {open_conflicts} unresolved"),
        findings,
        metrics: vec![AnalysisMetric { name: "total_conflicts".into(), value: findings_len as f64, unit: None }, AnalysisMetric { name: "open_conflicts".into(), value: open_conflicts as f64, unit: None }],
        diagnostics: adjacency_conflicts
            .into_iter()
            .map(|c| ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "analysis.conflict".into(), message: c.message, entity_id: Some(c.adjacency_a_id), register: Some("adjacencies".into()) })
            .collect(),
        entity_ids: Vec::new(),
    }
}

fn analyze_dependency(program: &ProgramSnapshot) -> AnalysisResult {
    let depends: Vec<String> = program.relationships.iter().filter(|r| matches!(r.kind, RelationshipKind::DependsOn)).map(|r| format!("{} depends on {}", r.source_id, r.target_id)).collect();
    let process_deps: usize = program.processes.iter().map(|p| p.dependencies.len()).sum();
    AnalysisResult {
        kind: AnalysisKind::Dependency,
        title: "Dependency Analysis".into(),
        summary: format!("{} relationship deps, {process_deps} process deps", depends.len()),
        findings: depends,
        metrics: vec![AnalysisMetric { name: "relationship_count".into(), value: program.relationships.len() as f64, unit: None }, AnalysisMetric { name: "process_dependency_count".into(), value: process_deps as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_capacity(program: &ProgramSnapshot) -> AnalysisResult {
    let total_area: f64 = program.elements.iter().filter_map(|e| e.area.target).sum();
    let total_occupancy: f64 = program.elements.iter().filter_map(|e| e.occupancy.target.or(e.occupancy.peak)).sum();
    let area_per_person = if total_occupancy > 0.0 { total_area / total_occupancy } else { 0.0 };
    AnalysisResult {
        kind: AnalysisKind::Capacity,
        title: "Capacity Analysis".into(),
        summary: format!("total target area {total_area:.1} m², {total_occupancy:.0} persons, {:.1} m²/person", area_per_person),
        findings: program.elements.iter().filter_map(|e| e.area.target.map(|a| format!("{}: {a:.1} m²", e.header.name))).collect(),
        metrics: vec![
            AnalysisMetric { name: "element_count".into(), value: program.elements.len() as f64, unit: None },
            AnalysisMetric { name: "total_target_area".into(), value: total_area, unit: Some("m2".into()) },
            AnalysisMetric { name: "area_per_person".into(), value: area_per_person, unit: Some("m2/person".into()) },
        ],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_demand(program: &ProgramSnapshot) -> AnalysisResult {
    let peak_occupancy: f64 = program.elements.iter().filter_map(|e| e.occupancy.peak.or(e.occupancy.target)).sum();
    let schedule_demand = program.schedules.len();
    AnalysisResult {
        kind: AnalysisKind::Demand,
        title: "Demand Analysis".into(),
        summary: format!("aggregate peak/target occupancy {peak_occupancy:.0}, {schedule_demand} schedule constraints"),
        findings: program.schedules.iter().map(|s| format!("schedule {} — {:?}", s.header.name, s.header.status)).collect(),
        metrics: vec![AnalysisMetric { name: "peak_occupancy".into(), value: peak_occupancy, unit: Some("persons".into()) }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_utilization(program: &ProgramSnapshot) -> AnalysisResult {
    let activities = program.activities.len();
    let elements = program.elements.len();
    let ratio = if elements == 0 { 0.0 } else { activities as f64 / elements as f64 };
    let equipped = program.equipment.iter().filter(|e| !e.element_ids.is_empty()).count();
    AnalysisResult {
        kind: AnalysisKind::Utilization,
        title: "Utilization Analysis".into(),
        summary: format!("activity/element ratio {ratio:.2}, {equipped} equipment placements"),
        findings: program.equipment.iter().map(|e| format!("{} serves {} element(s)", e.header.name, e.element_ids.len())).collect(),
        metrics: vec![AnalysisMetric { name: "activity_element_ratio".into(), value: ratio, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_workflow(program: &ProgramSnapshot) -> AnalysisResult {
    let critical: Vec<_> = program.processes.iter().filter(|p| p.critical_path).collect();
    AnalysisResult {
        kind: AnalysisKind::Workflow,
        title: "Workflow Analysis".into(),
        summary: format!("{} processes ({} critical), {} flows", program.processes.len(), critical.len(), program.flows.len()),
        findings: program.processes.iter().map(|p| format!("{} — {} steps, {} actors", p.header.name, p.steps.len(), p.actors.len())).collect(),
        metrics: vec![AnalysisMetric { name: "critical_path_processes".into(), value: critical.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: critical.iter().map(|p| p.header.id.clone()).collect(),
    }
}

fn analyze_risk(program: &ProgramSnapshot) -> AnalysisResult {
    let high: Vec<_> = program.risks.iter().filter(|r| matches!(r.probability, RiskLevel::High | RiskLevel::Critical) || matches!(r.impact, RiskLevel::High | RiskLevel::Critical)).map(|r| r.header.id.clone()).collect();
    let score_sum: f64 = program.risks.iter().map(|r| risk_score(&r.probability) * risk_score(&r.impact)).sum();
    AnalysisResult {
        kind: AnalysisKind::Risk,
        title: "Risk Analysis".into(),
        summary: format!("{} high/critical risk(s), aggregate score {score_sum:.0}", high.len()),
        findings: high.iter().map(|id| format!("risk {id}")).collect(),
        metrics: vec![AnalysisMetric { name: "risk_count".into(), value: program.risks.len() as f64, unit: None }, AnalysisMetric { name: "aggregate_risk_score".into(), value: score_sum, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: high,
    }
}

fn risk_score(level: &RiskLevel) -> f64 {
    match level {
        RiskLevel::Negligible => 0.5,
        RiskLevel::Low => 1.0,
        RiskLevel::Medium => 2.0,
        RiskLevel::High => 3.0,
        RiskLevel::Critical => 4.0,
    }
}

fn analyze_cost(program: &ProgramSnapshot) -> AnalysisResult {
    let total_capital: f64 = program.costs.iter().filter_map(|c| c.amount).sum();
    AnalysisResult {
        kind: AnalysisKind::Cost,
        title: "Cost Analysis".into(),
        summary: format!("{} cost requirements, capital total {total_capital:.0}", program.costs.len()),
        findings: program.costs.iter().filter_map(|c| c.amount.map(|v| format!("{}: {v:.0}", c.header.name))).collect(),
        metrics: vec![AnalysisMetric { name: "capital_cost_total".into(), value: total_capital, unit: Some("currency".into()) }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_scenario(program: &ProgramSnapshot) -> AnalysisResult {
    let evaluated = program.options.iter().filter(|o| o.evaluation_status == ValidationStatus::Passed).count();
    AnalysisResult {
        kind: AnalysisKind::Scenario,
        title: "Scenario Analysis".into(),
        summary: format!("{} scenario(s), {} options ({} selected)", program.scenarios.len(), program.options.len(), evaluated),
        findings: program.scenarios.iter().map(|s| s.header.name.clone()).collect(),
        metrics: vec![AnalysisMetric { name: "selected_options".into(), value: evaluated as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: program.scenarios.iter().map(|s| s.header.id.clone()).collect(),
    }
}

fn analyze_sensitivity(program: &ProgramSnapshot) -> AnalysisResult {
    let mandatory = program.requirements.iter().filter(|r| r.header.priority == crate::artifacts::program::kernel::Priority::Mandatory).count();
    AnalysisResult {
        kind: AnalysisKind::Sensitivity,
        title: "Sensitivity Analysis".into(),
        summary: format!("{mandatory} mandatory requirements drive sensitivity"),
        findings: program.priorities.iter().map(|p| format!("{} — {:?}", p.header.name, p.header.priority)).collect(),
        metrics: vec![AnalysisMetric { name: "mandatory_requirement_count".into(), value: mandatory as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_impact(program: &ProgramSnapshot) -> AnalysisResult {
    let impacted: usize = program.decisions.iter().map(|d| d.impacted_requirement_ids.len() + d.impacted_element_ids.len()).sum();
    AnalysisResult {
        kind: AnalysisKind::Impact,
        title: "Impact Analysis".into(),
        summary: format!("{} decision(s) touching {impacted} requirement/element links", program.decisions.len()),
        findings: program.decisions.iter().map(|d| format!("{} impacts {} requirements", d.header.name, d.impacted_requirement_ids.len())).collect(),
        metrics: vec![AnalysisMetric { name: "impacted_links".into(), value: impacted as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect(),
    }
}

fn analyze_trend(program: &ProgramSnapshot) -> AnalysisResult {
    let change_velocity = program.changes.len();
    AnalysisResult {
        kind: AnalysisKind::Trend,
        title: "Trend Analysis".into(),
        summary: format!("{} analysis records, {} changes, {} audit events", program.analyses.len(), change_velocity, program.audit_events.len()),
        findings: program.changes.iter().take(5).map(|c| format!("change {} — {}", c.header.name, c.header.timestamps.updated)).collect(),
        metrics: vec![AnalysisMetric { name: "change_count".into(), value: change_velocity as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_requirement_comparison(program: &ProgramSnapshot) -> AnalysisResult {
    let mut by_kind: HashMap<String, usize> = HashMap::new();
    for req in &program.requirements {
        *by_kind.entry(format!("{:?}", req.kind)).or_default() += 1;
    }
    let findings: Vec<String> = by_kind.iter().map(|(kind, count)| format!("{kind}: {count} requirement(s)")).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementComparison,
        title: "Requirement Comparison".into(),
        summary: format!("{} requirement kinds compared across {} items", by_kind.len(), program.requirements.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "requirement_kind_count".into(), value: by_kind.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn analyze_requirement_clustering(program: &ProgramSnapshot) -> AnalysisResult {
    let mut clusters: HashMap<String, Vec<EntityId>> = HashMap::new();
    for req in &program.requirements {
        let key = format!("{:?}-{:?}", req.kind, req.header.priority);
        clusters.entry(key).or_default().push(req.header.id.clone());
    }
    let findings: Vec<String> = clusters.iter().map(|(key, ids)| format!("cluster {key}: {} requirement(s)", ids.len())).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementClustering,
        title: "Requirement Clustering".into(),
        summary: format!("{} clusters from {} requirements", clusters.len(), program.requirements.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "cluster_count".into(), value: clusters.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: clusters.values().flatten().cloned().collect(),
    }
}

fn analyze_requirement_filtering(program: &ProgramSnapshot) -> AnalysisResult {
    let pending: Vec<_> = program.requirements.iter().filter(|r| r.validation_status == ValidationStatus::Pending).map(|r| r.header.id.clone()).collect();
    let findings: Vec<String> = pending.iter().map(|id| format!("pending validation: {id}")).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementFiltering,
        title: "Requirement Filtering".into(),
        summary: format!("{} pending of {} total requirements", pending.len(), program.requirements.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "pending_validation_count".into(), value: pending.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: pending,
    }
}

fn analyze_requirement_sorting(program: &ProgramSnapshot) -> AnalysisResult {
    let mut sorted: Vec<_> = program.requirements.iter().collect();
    sorted.sort_by_key(|r| r.header.priority);
    let findings: Vec<String> = sorted.iter().map(|r| format!("{:?} — {}", r.header.priority, r.header.name)).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementSorting,
        title: "Requirement Sorting".into(),
        summary: format!("{} requirements sorted by priority", sorted.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "sorted_requirement_count".into(), value: sorted.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: sorted.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn priority_weight(priority: &crate::artifacts::program::kernel::Priority) -> f64 {
    match priority {
        crate::artifacts::program::kernel::Priority::Mandatory => 5.0,
        crate::artifacts::program::kernel::Priority::Essential => 4.0,
        crate::artifacts::program::kernel::Priority::Preferred => 3.0,
        crate::artifacts::program::kernel::Priority::Optional => 2.0,
        crate::artifacts::program::kernel::Priority::Deferred => 1.0,
        crate::artifacts::program::kernel::Priority::Prohibited => 0.0,
    }
}

fn analyze_requirement_scoring(program: &ProgramSnapshot) -> AnalysisResult {
    let mut scored: Vec<(EntityId, f64)> = program
        .requirements
        .iter()
        .map(|r| {
            let base = priority_weight(&r.header.priority);
            let validation = match r.validation_status {
                ValidationStatus::Passed => 1.0,
                ValidationStatus::Pending => 0.5,
                ValidationStatus::Failed => 0.0,
                ValidationStatus::Waived => 0.25,
                ValidationStatus::Deferred => 0.1,
            };
            (r.header.id.clone(), base * validation)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let findings: Vec<String> = scored.iter().take(10).map(|(id, score)| format!("{id}: score {score:.2}")).collect();
    let total: f64 = scored.iter().map(|(_, s)| s).sum();
    AnalysisResult {
        kind: AnalysisKind::RequirementScoring,
        title: "Requirement Scoring".into(),
        summary: format!("scored {} requirements, total {total:.1}", scored.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "total_requirement_score".into(), value: total, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: scored.into_iter().map(|(id, _)| id).collect(),
    }
}

fn analyze_requirement_weighting(program: &ProgramSnapshot) -> AnalysisResult {
    let mut weights: HashMap<EntityId, f64> = HashMap::new();
    for record in &program.priorities {
        if let Some(weight) = record.weight {
            weights.insert(record.subject_id.clone(), weight);
        }
    }
    let findings: Vec<String> = weights.iter().map(|(id, w)| format!("{id}: weight {w:.2}")).collect();
    let avg = if weights.is_empty() { 0.0 } else { weights.values().sum::<f64>() / weights.len() as f64 };
    AnalysisResult {
        kind: AnalysisKind::RequirementWeighting,
        title: "Requirement Weighting".into(),
        summary: format!("{} weighted subjects, average weight {avg:.2}", weights.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "average_weight".into(), value: avg, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: weights.keys().cloned().collect(),
    }
}

fn analyze_relationship(program: &ProgramSnapshot) -> AnalysisResult {
    let mut nodes: HashSet<EntityId> = HashSet::new();
    for rel in &program.relationships {
        nodes.insert(rel.source_id.clone());
        nodes.insert(rel.target_id.clone());
    }
    let depends = program.relationships.iter().filter(|r| matches!(r.kind, RelationshipKind::DependsOn)).count();
    let conflicts = program.relationships.iter().filter(|r| matches!(r.kind, RelationshipKind::ConflictsWith)).count();
    AnalysisResult {
        kind: AnalysisKind::RelationshipAnalysis,
        title: "Relationship Analysis".into(),
        summary: format!("{} relationships across {} nodes ({} depends, {} conflicts)", program.relationships.len(), nodes.len(), depends, conflicts),
        findings: program.relationships.iter().map(|r| format!("{:?}: {} → {}", r.kind, r.source_id, r.target_id)).collect(),
        metrics: vec![AnalysisMetric { name: "relationship_node_count".into(), value: nodes.len() as f64, unit: None }, AnalysisMetric { name: "dependency_edge_count".into(), value: depends as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: nodes.into_iter().collect(),
    }
}
//#endregion 🔬️Analyze

#[cfg(test)]
//#region 🧪️AnalyzeTests
mod tests_analyze {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn gap_analysis_on_sample_plugin() {
        let result = run_analysis(&sample_plugin(), AnalysisKind::Gap);
        assert_eq!(result.kind, AnalysisKind::Gap);
        assert!(!result.findings.is_empty());
    }

    #[test]
    fn capacity_analysis_sums_area() {
        let result = run_analysis(&sample_plugin(), AnalysisKind::Capacity);
        assert!(result.metrics.iter().any(|m| m.name == "total_target_area"));
        assert!(result.metrics.iter().any(|m| m.value > 0.0));
    }

    #[test]
    fn requirement_clustering_produces_clusters() {
        let result = run_analysis(&sample_plugin(), AnalysisKind::RequirementClustering);
        assert_eq!(result.kind, AnalysisKind::RequirementClustering);
    }
}
//#endregion 🧪️AnalyzeTests

//#region 📤️ExchangeReads
/// 🏷️ Fixed 7-column header shared by the CSV and TSV register exchange shape.
const REGISTER_ROW_COLUMNS: [&str; 7] = ["register", "id", "name", "status", "priority", "tags", "source"];

/// 📊️ One CSV/TSV row representing a register entity for spreadsheet round-trip.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCsvRow {
    pub register: String,
    pub id: EntityId,
    pub name: String,
    pub status: String,
    pub priority: String,
    pub tags: String,
    pub source: String,
}

impl RegisterCsvRow {
    /// 🧵️ This row's 7 columns in `REGISTER_ROW_COLUMNS` order.
    fn columns(&self) -> [String; 7] {
        [self.register.clone(), self.id.to_string(), self.name.clone(), self.status.clone(), self.priority.clone(), self.tags.clone(), self.source.clone()]
    }

    /// 🧵️ Rebuilds a row from 7 ordered column values (inverse of `columns`).
    pub(crate) fn from_columns(fields: &[String]) -> Result<Self, PluginError> {
        if fields.len() < 7 {
            return Err(PluginError::Csv(format!("malformed row: expected 7 columns, got {}", fields.len())));
        }
        Ok(Self { register: fields[0].clone(), id: EntityId(fields[1].clone()), name: fields[2].clone(), status: fields[3].clone(), priority: fields[4].clone(), tags: fields[5].clone(), source: fields[6].clone() })
    }
}

/// 📤️ Serializes a plugin to pretty JSON.
pub fn export_json(program: &ProgramSnapshot) -> Result<String, PluginError> {
    serde_json::to_string_pretty(program).map_err(|e| PluginError::Serialize(e.to_string()))
}

/// 📥️ Deserializes a plugin from JSON with schema validation.
pub fn import_json(json: &str) -> Result<ProgramSnapshot, PluginError> {
    let program: ProgramSnapshot = serde_json::from_str(json).map_err(|e| PluginError::Deserialize(e.to_string()))?;
    if program.schema != ARCHITECT_PROGRAM_SCHEMA {
        return Err(PluginError::InvalidSchema { expected: ARCHITECT_PROGRAM_SCHEMA.into(), actual: program.schema });
    }
    Ok(program)
}

fn csv_record(values: &[&str]) -> stdio_csv::schema::snapshot::CsvRecord {
    stdio_csv::schema::snapshot::CsvRecord { fields: values.iter().map(|v| stdio_csv::schema::snapshot::CsvField { value: (*v).to_string(), quoted: false }).collect() }
}

/// 📤️ Flattens all registers into a `CsvSnapshot`, encoded by stdio's real RFC 4180 codec.
pub fn export_registers_csv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(stdio_csv::schema::snapshot::encode_csv(&rows_to_csv_snapshot(&collect_rows(program))))
}

/// ↔ Exports relationships as a CSV table preserving endpoints, encoded by stdio's real RFC 4180
/// codec.
pub fn export_relationships_csv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    let mut records = vec![csv_record(&["id", "source_id", "target_id", "kind", "name"])];
    for rel in &program.relationships {
        records.push(csv_record(&[&rel.header.id.to_string(), &rel.source_id.to_string(), &rel.target_id.to_string(), &format!("{:?}", rel.kind), &rel.header.name]));
    }
    let snapshot = stdio_csv::CsvSnapshot { schema: stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records };
    Ok(stdio_csv::schema::snapshot::encode_csv(&snapshot))
}

fn rows_to_csv_snapshot(rows: &[RegisterCsvRow]) -> stdio_csv::CsvSnapshot {
    let mut records = vec![csv_record(&REGISTER_ROW_COLUMNS)];
    records.extend(rows.iter().map(|row| { let cols = row.columns(); csv_record(&[&cols[0], &cols[1], &cols[2], &cols[3], &cols[4], &cols[5], &cols[6]]) }));
    stdio_csv::CsvSnapshot { schema: stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records }
}

/// 📤️ Flattens all registers into a `TsvSnapshot`, encoded by stdio's real IANA TSV codec.
pub fn export_registers_tsv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(stdio_tsv_engine::encode_tsv(&rows_to_tsv_snapshot(&collect_rows(program))))
}

fn rows_to_tsv_snapshot(rows: &[RegisterCsvRow]) -> stdio_tsv::TsvSnapshot {
    let mut records: Vec<Vec<String>> = vec![REGISTER_ROW_COLUMNS.iter().map(|c| c.to_string()).collect()];
    records.extend(rows.iter().map(|row| row.columns().to_vec()));
    stdio_tsv::TsvSnapshot { schema: stdio_tsv::STDIO_TSV_DOCUMENT_SCHEMA.into(), records, trailing_newline: true, line_ending: stdio_tsv_line_ending::LineEnding::Lf }
}

fn collect_rows(program: &ProgramSnapshot) -> Vec<RegisterCsvRow> {
    let mut rows = Vec::new();
    macro_rules! push_rows {
        ($register:literal, $collection:expr) => {
            for item in $collection {
                rows.push(header_row($register, &item.header, None));
            }
        };
    }
    push_rows!("stakeholders", &program.stakeholders);
    push_rows!("users", &program.users);
    push_rows!("activities", &program.activities);
    push_rows!("functions", &program.functions);
    push_rows!("elements", &program.elements);
    push_rows!("quantities", &program.quantities);
    push_rows!("relationships", &program.relationships);
    push_rows!("adjacencies", &program.adjacencies);
    push_rows!("processes", &program.processes);
    push_rows!("flows", &program.flows);
    push_rows!("access_rules", &program.access_rules);
    push_rows!("operations", &program.operations);
    push_rows!("equipment", &program.equipment);
    push_rows!("resources", &program.resources);
    push_rows!("storage", &program.storage);
    push_rows!("environmental", &program.environmental);
    push_rows!("human_factors", &program.human_factors);
    push_rows!("accessibility", &program.accessibility);
    push_rows!("privacy", &program.privacy);
    push_rows!("safety", &program.safety);
    push_rows!("security", &program.security);
    push_rows!("regulatory", &program.regulatory);
    push_rows!("site_context", &program.site_context);
    push_rows!("organizational", &program.organizational);
    push_rows!("services", &program.services);
    push_rows!("infrastructure", &program.infrastructure);
    push_rows!("information", &program.information);
    push_rows!("communication", &program.communication);
    push_rows!("wayfinding", &program.wayfinding);
    push_rows!("schedules", &program.schedules);
    push_rows!("flexibility", &program.flexibility);
    push_rows!("growth", &program.growth);
    push_rows!("sustainability", &program.sustainability);
    push_rows!("resilience", &program.resilience);
    push_rows!("costs", &program.costs);
    push_rows!("delivery", &program.delivery);
    push_rows!("risks", &program.risks);
    push_rows!("conflicts", &program.conflicts);
    push_rows!("requirements", &program.requirements);
    push_rows!("priorities", &program.priorities);
    push_rows!("scenarios", &program.scenarios);
    push_rows!("options", &program.options);
    push_rows!("decisions", &program.decisions);
    push_rows!("validations", &program.validations);
    push_rows!("performance", &program.performance);
    push_rows!("quality", &program.quality);
    push_rows!("documents", &program.artifacts);
    push_rows!("changes", &program.changes);
    push_rows!("collaboration", &program.collaboration);
    push_rows!("analyses", &program.analyses);
    push_rows!("reports", &program.reports);
    push_rows!("search_filters", &program.search_filters);
    push_rows!("status_records", &program.status_records);
    push_rows!("workshops", &program.workshops);
    push_rows!("surveys", &program.surveys);
    push_rows!("issues", &program.issues);
    push_rows!("audit_events", &program.audit_events);
    push_rows!("templates", &program.templates);
    push_rows!("knowledge", &crate::artifacts::program::program_knowledge(program));
    push_rows!("benchmarks", &crate::artifacts::program::program_benchmarks(program));
    rows
}

fn header_row(register: &str, header: &EntityHeader, source: Option<String>) -> RegisterCsvRow {
    RegisterCsvRow {
        register: register.into(),
        id: header.id.clone(),
        name: header.name.clone(),
        status: format!("{:?}", header.status),
        priority: format!("{:?}", header.priority),
        tags: header.tags.join(";"),
        source: source.unwrap_or_default(),
    }
}
//#endregion 📤️ExchangeReads

#[cfg(test)]
//#region 🧪️ExchangeReadsTests
mod tests_exchange {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn json_round_trip() {
        let program = sample_plugin();
        let json = export_json(&program).expect("export");
        let imported = import_json(&json).expect("import");
        assert_eq!(imported.elements.len(), program.elements.len());
        assert_eq!(imported.adjacencies.len(), program.adjacencies.len());
    }

    #[test]
    fn relationships_csv_round_trips_via_stdio_codec() {
        let program = sample_plugin();
        let csv = export_relationships_csv(&program).expect("relationships csv export");
        let snapshot = stdio_csv::schema::snapshot::decode_csv_with(&csv, true);
        assert_eq!(snapshot.records.len(), program.relationships.len() + 1, "header + one row per relationship");
    }
}
//#endregion 🧪️ExchangeReadsTests

//#region 🧭️TraceReads
/// 📜️ Filtered audit trail slice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditTrail {
    pub subject_id: Option<EntityId>,
    pub events: Vec<AuditEvent>,
}

/// 📋️ Returns audit events for an optional subject, newest first.
pub fn audit_trail(program: &ProgramSnapshot, subject_id: Option<&EntityId>) -> AuditTrail {
    let mut events: Vec<AuditEvent> = program.audit_events.iter().filter(|event| subject_id.is_none_or(|id| &event.subject_id == id)).cloned().collect();
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    AuditTrail { subject_id: subject_id.cloned(), events }
}

/// 🔁️ Resolves superseded requirements to their terminal replacement.
pub fn resolve_supersedes(program: &ProgramSnapshot, requirement_id: &EntityId) -> EntityId {
    let mut current = requirement_id.clone();
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current.clone()) {
            break;
        }
        let Some(next) = program.requirements.iter().find(|r| r.header.id == current).and_then(|r| r.superseded_by.clone()) else {
            break;
        };
        current = next;
    }
    current
}
//#endregion 🧭️TraceReads

#[cfg(test)]
//#region 🧪️TraceReadsTests
mod tests_trace {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn audit_trail_sorted_newest_first() {
        let mut program = sample_plugin();
        program.audit_events.push(AuditEvent {
            header: crate::artifacts::program::kernel::EntityHeader::new(EntityId::new_serial("audit", "older"), "older"),
            action: crate::artifacts::program::registers::AuditAction::Created,
            actor_id: None,
            subject_id: program.elements[0].header.id.clone(),
            subject_kind: "element".into(),
            timestamp: "2020-01-01T00:00:00Z".into(),
            details: crate::artifacts::program::kernel::TextField::plain("old"),
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
        program.audit_events.push(AuditEvent {
            header: crate::artifacts::program::kernel::EntityHeader::new(EntityId::new_serial("audit", "newer"), "newer"),
            action: crate::artifacts::program::registers::AuditAction::Updated,
            actor_id: None,
            subject_id: program.elements[0].header.id.clone(),
            subject_kind: "element".into(),
            timestamp: "2025-01-01T00:00:00Z".into(),
            details: crate::artifacts::program::kernel::TextField::plain("new"),
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
        let trail = audit_trail(&program, None);
        assert!(trail.events[0].timestamp > trail.events[1].timestamp);
    }
}
//#endregion 🧪️TraceReadsTests

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = ProgramSnapshot::default();
        assert_eq!(ProgramInference::infer(&snapshot), ProgramInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(ProgramInference::infer(&ProgramSnapshot::default()), ProgramInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
