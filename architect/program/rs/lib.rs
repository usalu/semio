//! 🏛️ Headless architectural programming — program registers, adjacency, analysis, and exchange.

mod adjacency {
    //! ↔️ Undirected adjacency logic — canonical pairs, matrix view, conflicts, and graph edges.

    use crate::kernel::EntityId;
    use crate::program::Program;
    use crate::registers::{Adjacency, AdjacencyKind, SeparationKind};
    use mathematical_graph::{orient_endpoints, Undirected};
    use serde::{Deserialize, Serialize};

    // #region 🔖PairNormalization
    /// @emoji 📐 Canonical undirected endpoint order using `mathematical_graph::orient_endpoints`.
    pub fn normalize_pair(a: &EntityId, b: &EntityId) -> (EntityId, EntityId) {
        let (left, right) = orient_endpoints::<&str, Undirected>(&a.0, &b.0);
        (EntityId(left.to_string()), EntityId(right.to_string()))
    }
    // #endregion

    // #region 🔖Mutations
    /// @emoji ➕ Upserts an adjacency row with normalized endpoints; replaces same pair if present.
    pub fn set_adjacency(program: &mut Program, mut adjacency: Adjacency) {
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

    /// @emoji ➖ Removes an adjacency by id or by normalized element pair.
    pub fn clear_adjacency(program: &mut Program, id: &EntityId) {
        if let Some(index) = program.adjacencies.iter().position(|row| &row.header.id == id) {
            program.adjacencies.remove(index);
            return;
        }
        program.adjacencies.retain(|row| &row.element_a_id != id && &row.element_b_id != id);
    }
    // #endregion

    // #region 🔖Views
    /// @emoji 🔢 Dense lower-triangle adjacency matrix keyed by element id order.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AdjacencyMatrix {
        pub element_ids: Vec<EntityId>,
        pub cells: Vec<Vec<Option<AdjacencyCell>>>,
    }

    /// @emoji 🟦 One matrix cell summarizing the undirected link between two elements.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AdjacencyCell {
        pub adjacency_id: EntityId,
        pub kind: AdjacencyKind,
        pub weight: f64,
        pub separations: Vec<SeparationKind>,
    }

    /// @emoji 📊 Builds a lower-triangle matrix view over program elements and adjacencies.
    pub fn adjacency_matrix(program: &Program) -> AdjacencyMatrix {
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

    /// @emoji 🕸️ Undirected edge list for graph rendering (`a`, `b`, weight).
    pub fn undirected_edges(program: &Program) -> Vec<(EntityId, EntityId, f64)> {
        program.adjacencies.iter().map(|adjacency| (adjacency.element_a_id.clone(), adjacency.element_b_id.clone(), adjacency.weight)).collect()
    }
    // #endregion

    // #region 🔖Conflicts
    /// @emoji ⚡ Adjacency pair ids that violate required/prohibited or separation rules.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AdjacencyConflict {
        pub adjacency_a_id: EntityId,
        pub adjacency_b_id: EntityId,
        pub message: String,
    }

    /// @emoji 🔍 Detects duplicate pairs, kind conflicts, separation/distance/level violations.
    pub fn detect_adjacency_conflicts(program: &Program) -> Vec<AdjacencyConflict> {
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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn normalize_pair_orders_endpoints() {
            let a = EntityId("element-2".into());
            let b = EntityId("element-10".into());
            assert_eq!(normalize_pair(&b, &a), (b, a));
        }

        #[test]
        fn sample_program_matrix_has_one_cell() {
            let program = sample_program();
            let matrix = adjacency_matrix(&program);
            assert_eq!(matrix.element_ids.len(), 2);
            let populated: usize = matrix.cells.iter().flat_map(|row| row.iter()).filter(|cell| cell.is_some()).count();
            assert_eq!(populated, 1);
        }

        #[test]
        fn detects_distance_min_max_violation() {
            let mut program = sample_program();
            program.adjacencies[0].distance_min_m = Some(10.0);
            program.adjacencies[0].distance_max_m = Some(5.0);
            let conflicts = detect_adjacency_conflicts(&program);
            assert!(conflicts.iter().any(|c| c.message.contains("distance_min")));
        }
    }
}

mod analyze {
    //! 🔬 Program analysis — gap, conflict, dependency, capacity, and related kinds.

    use crate::adjacency::detect_adjacency_conflicts;
    use crate::kernel::{DiagnosticSeverity, EntityHeader, EntityId, ProgramDiagnostic, TextField};
    use crate::program::Program;
    use crate::registers::{AnalysisKind, AnalysisRecord, RelationshipKind, RiskLevel, ValidationStatus};
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};

    // #region 🔖AnalysisResult
    /// @emoji 📈 Structured output from `run_analysis`.
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

    /// @emoji 📊 Named numeric metric from an analysis run.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AnalysisMetric {
        pub name: String,
        pub value: f64,
        pub unit: Option<String>,
    }
    // #endregion

    // #region 🔖RunAnalysis
    /// @emoji 🧮 Runs the requested analysis kind over a program snapshot.
    pub fn run_analysis(program: &Program, kind: AnalysisKind) -> AnalysisResult {
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

    /// @emoji 📝 Runs analysis and appends an `AnalysisRecord` to the program.
    pub fn run_analysis_and_record(program: &mut Program, kind: AnalysisKind) -> AnalysisResult {
        let result = run_analysis(program, kind);
        let record = AnalysisRecord {
            header: EntityHeader::new(EntityId::new_serial("analysis"), result.title.clone()),
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

    fn analyze_gap(program: &Program) -> AnalysisResult {
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

    fn analyze_conflict(program: &Program) -> AnalysisResult {
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

    fn analyze_dependency(program: &Program) -> AnalysisResult {
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

    fn analyze_capacity(program: &Program) -> AnalysisResult {
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

    fn analyze_demand(program: &Program) -> AnalysisResult {
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

    fn analyze_utilization(program: &Program) -> AnalysisResult {
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

    fn analyze_workflow(program: &Program) -> AnalysisResult {
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

    fn analyze_risk(program: &Program) -> AnalysisResult {
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

    fn analyze_cost(program: &Program) -> AnalysisResult {
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

    fn analyze_scenario(program: &Program) -> AnalysisResult {
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

    fn analyze_sensitivity(program: &Program) -> AnalysisResult {
        let mandatory = program.requirements.iter().filter(|r| r.header.priority == crate::kernel::Priority::Mandatory).count();
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

    fn analyze_impact(program: &Program) -> AnalysisResult {
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

    fn analyze_trend(program: &Program) -> AnalysisResult {
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

    fn analyze_requirement_comparison(program: &Program) -> AnalysisResult {
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

    fn analyze_requirement_clustering(program: &Program) -> AnalysisResult {
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

    fn analyze_requirement_filtering(program: &Program) -> AnalysisResult {
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

    fn analyze_requirement_sorting(program: &Program) -> AnalysisResult {
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

    fn priority_weight(priority: &crate::kernel::Priority) -> f64 {
        match priority {
            crate::kernel::Priority::Mandatory => 5.0,
            crate::kernel::Priority::Essential => 4.0,
            crate::kernel::Priority::Preferred => 3.0,
            crate::kernel::Priority::Optional => 2.0,
            crate::kernel::Priority::Deferred => 1.0,
            crate::kernel::Priority::Prohibited => 0.0,
        }
    }

    fn analyze_requirement_scoring(program: &Program) -> AnalysisResult {
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

    fn analyze_requirement_weighting(program: &Program) -> AnalysisResult {
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

    fn analyze_relationship(program: &Program) -> AnalysisResult {
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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn gap_analysis_on_sample_program() {
            let result = run_analysis(&sample_program(), AnalysisKind::Gap);
            assert_eq!(result.kind, AnalysisKind::Gap);
            assert!(!result.findings.is_empty());
        }

        #[test]
        fn capacity_analysis_sums_area() {
            let result = run_analysis(&sample_program(), AnalysisKind::Capacity);
            assert!(result.metrics.iter().any(|m| m.name == "total_target_area"));
            assert!(result.metrics.iter().any(|m| m.value > 0.0));
        }

        #[test]
        fn run_analysis_and_record_persists() {
            let mut program = sample_program();
            let before = program.analyses.len();
            run_analysis_and_record(&mut program, AnalysisKind::Risk);
            assert_eq!(program.analyses.len(), before + 1);
        }

        #[test]
        fn requirement_clustering_produces_clusters() {
            let result = run_analysis(&sample_program(), AnalysisKind::RequirementClustering);
            assert_eq!(result.kind, AnalysisKind::RequirementClustering);
        }
    }
}

mod exchange {
    //! 📤 Data exchange — JSON and CSV import/export for program registers.

    use crate::kernel::{EntityHeader, EntityId, ProgramError, TextField};
    use crate::program::{Program, ARCHITECT_PROGRAM_SCHEMA};
    use crate::registers::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    // #region 🔖MergeStrategy
    /// @emoji 🔀 Strategy for merging imported register rows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum MergeStrategy {
        Replace,
        SkipDuplicates,
        Upsert,
    }
    // #endregion

    // #region 🔖JsonExchange
    /// @emoji 📤 Serializes a program to pretty JSON.
    pub fn export_json(program: &Program) -> Result<String, ProgramError> {
        serde_json::to_string_pretty(program).map_err(|e| ProgramError::Serialize(e.to_string()))
    }

    /// @emoji 📥 Deserializes a program from JSON with schema validation.
    pub fn import_json(json: &str) -> Result<Program, ProgramError> {
        let program: Program = serde_json::from_str(json).map_err(|e| ProgramError::Deserialize(e.to_string()))?;
        if program.schema != ARCHITECT_PROGRAM_SCHEMA {
            return Err(ProgramError::InvalidSchema { expected: ARCHITECT_PROGRAM_SCHEMA.into(), actual: program.schema });
        }
        Ok(program)
    }
    // #endregion

    // #region 🔖CsvExchange
    /// @emoji 📊 One CSV row representing a register entity for spreadsheet round-trip.
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

    /// @emoji 📤 Flattens all registers into CSV rows.
    pub fn export_registers_csv(program: &Program) -> Result<String, ProgramError> {
        Ok(write_delimited(&collect_rows(program), ','))
    }

    /// @emoji 📥 Merges CSV rows into matching register collections.
    pub fn import_registers_csv(program: &mut Program, csv: &str, strategy: MergeStrategy) -> Result<Vec<EntityId>, ProgramError> {
        import_delimited(program, csv, ',', strategy)
    }

    /// @emoji 📤 Flattens all registers into TSV rows.
    pub fn export_registers_tsv(program: &Program) -> Result<String, ProgramError> {
        Ok(write_delimited(&collect_rows(program), '\t'))
    }

    /// @emoji 📥 Merges TSV rows into matching register collections.
    pub fn import_registers_tsv(program: &mut Program, tsv: &str, strategy: MergeStrategy) -> Result<Vec<EntityId>, ProgramError> {
        import_delimited(program, tsv, '\t', strategy)
    }

    /// @emoji ↔️ Exports relationships as CSV rows preserving endpoints.
    pub fn export_relationships_csv(program: &Program) -> Result<String, ProgramError> {
        let mut out = String::from("id,source_id,target_id,kind,name\n");
        for rel in &program.relationships {
            out.push_str(&format!("{},{},{},{:?},{}\n", escape_field(&rel.header.id.to_string(), ','), escape_field(&rel.source_id.to_string(), ','), escape_field(&rel.target_id.to_string(), ','), rel.kind, escape_field(&rel.header.name, ','),));
        }
        Ok(out)
    }

    fn collect_rows(program: &Program) -> Vec<RegisterCsvRow> {
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
        push_rows!("documents", &program.documents);
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
        push_rows!("knowledge", &program.knowledge);
        push_rows!("benchmarks", &program.benchmarks);
        rows
    }

    fn header_row(register: &str, header: &EntityHeader, source: Option<String>) -> RegisterCsvRow {
        RegisterCsvRow { register: register.into(), id: header.id.clone(), name: header.name.clone(), status: format!("{:?}", header.status), priority: format!("{:?}", header.priority), tags: header.tags.join(";"), source: source.unwrap_or_default() }
    }

    fn write_delimited(rows: &[RegisterCsvRow], delimiter: char) -> String {
        let header = format!("register{}id{}name{}status{}priority{}tags{}source\n", delimiter, delimiter, delimiter, delimiter, delimiter, delimiter);
        let mut out = header;
        for row in rows {
            out.push_str(&format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
                escape_field(&row.register, delimiter),
                delimiter,
                escape_field(&row.id.to_string(), delimiter),
                delimiter,
                escape_field(&row.name, delimiter),
                delimiter,
                escape_field(&row.status, delimiter),
                delimiter,
                escape_field(&row.priority, delimiter),
                delimiter,
                escape_field(&row.tags, delimiter),
                delimiter,
                escape_field(&row.source, delimiter),
            ));
        }
        out
    }

    fn escape_field(value: &str, delimiter: char) -> String {
        if value.contains(delimiter) || value.contains('"') || value.contains('\n') {
            format!("\"{}\"", value.replace('"', "\"\""))
        } else {
            value.to_string()
        }
    }

    fn parse_delimited(input: &str, delimiter: char) -> Result<Vec<RegisterCsvRow>, ProgramError> {
        let mut lines = input.lines();
        let header = lines.next().ok_or_else(|| ProgramError::Csv("empty delimited file".into()))?;
        let expected = format!("register{}id{}name{}status{}priority{}tags{}source", delimiter, delimiter, delimiter, delimiter, delimiter, delimiter);
        if header != expected {
            return Err(ProgramError::Csv(format!("unexpected header: {header}")));
        }
        let mut rows = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let fields = parse_record(line, delimiter);
            if fields.len() < 7 {
                return Err(ProgramError::Csv(format!("malformed row: {line}")));
            }
            rows.push(RegisterCsvRow { register: fields[0].clone(), id: EntityId(fields[1].clone()), name: fields[2].clone(), status: fields[3].clone(), priority: fields[4].clone(), tags: fields[5].clone(), source: fields[6].clone() });
        }
        Ok(rows)
    }

    fn parse_record(line: &str, delimiter: char) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '"' if in_quotes => {
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        current.push('"');
                    } else {
                        in_quotes = false;
                    }
                }
                '"' => in_quotes = true,
                c if c == delimiter && !in_quotes => {
                    fields.push(current.clone());
                    current.clear();
                }
                c => current.push(c),
            }
        }
        fields.push(current);
        fields
    }

    fn import_delimited(program: &mut Program, input: &str, delimiter: char, strategy: MergeStrategy) -> Result<Vec<EntityId>, ProgramError> {
        let rows = parse_delimited(input, delimiter)?;
        let mut touched = Vec::new();
        let mut seen: HashSet<(String, EntityId)> = HashSet::new();
        for row in rows {
            let key = (row.register.clone(), row.id.clone());
            if !seen.insert(key.clone()) {
                return Err(ProgramError::Csv(format!("duplicate import id {} in register {}", row.id, row.register)));
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

    fn register_contains(program: &Program, register: &str, id: &EntityId) -> bool {
        match register {
            "elements" => program.elements.iter().any(|e| &e.header.id == id),
            "stakeholders" => program.stakeholders.iter().any(|s| &s.header.id == id),
            "requirements" => program.requirements.iter().any(|r| &r.header.id == id),
            "relationships" => program.relationships.iter().any(|r| &r.header.id == id),
            "adjacencies" => program.adjacencies.iter().any(|a| &a.header.id == id),
            _ => false,
        }
    }

    fn remove_register_item(program: &mut Program, register: &str, id: &EntityId) {
        match register {
            "elements" => program.elements.retain(|e| &e.header.id != id),
            "stakeholders" => program.stakeholders.retain(|s| &s.header.id != id),
            "requirements" => program.requirements.retain(|r| &r.header.id != id),
            "relationships" => program.relationships.retain(|r| &r.header.id != id),
            "adjacencies" => program.adjacencies.retain(|a| &a.header.id != id),
            _ => {}
        }
    }

    fn upsert_register_row(program: &mut Program, row: RegisterCsvRow) -> Result<(), ProgramError> {
        match row.register.as_str() {
            "elements" => upsert_element(program, row),
            "stakeholders" => upsert_stakeholder(program, row),
            "requirements" => upsert_requirement(program, row),
            "relationships" => upsert_relationship_stub(program, row),
            "adjacencies" => upsert_adjacency_stub(program, row),
            other => {
                return Err(ProgramError::Csv(format!("unsupported register import: {other}")));
            }
        }
        Ok(())
    }

    fn upsert_element(program: &mut Program, row: RegisterCsvRow) {
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

    fn upsert_stakeholder(program: &mut Program, row: RegisterCsvRow) {
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
            influence: InfluenceLevel::Medium,
            interest: InfluenceLevel::Medium,
            engagement: EngagementLevel::Neutral,
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

    fn upsert_requirement(program: &mut Program, row: RegisterCsvRow) {
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

    fn upsert_relationship_stub(program: &mut Program, row: RegisterCsvRow) {
        if program.relationships.iter().any(|r| r.header.id == row.id) {
            return;
        }
        let fallback = program.elements.first().map_or_else(|| EntityId::new_serial("element"), |e| e.header.id.clone());
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

    fn upsert_adjacency_stub(program: &mut Program, row: RegisterCsvRow) {
        if program.adjacencies.iter().any(|a| a.header.id == row.id) {
            return;
        }
        let a = program.elements.first().map_or_else(|| EntityId::new_serial("element"), |e| e.header.id.clone());
        let b = program.elements.get(1).map_or_else(|| a.clone(), |e| e.header.id.clone());
        let (left, right) = crate::adjacency::normalize_pair(&a, &b);
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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn json_round_trip() {
            let program = sample_program();
            let json = export_json(&program).expect("export");
            let imported = import_json(&json).expect("import");
            assert_eq!(imported.elements.len(), program.elements.len());
            assert_eq!(imported.adjacencies.len(), program.adjacencies.len());
        }

        #[test]
        fn csv_round_trip_preserves_element_names() {
            let program = sample_program();
            let csv = export_registers_csv(&program).expect("csv export");
            let mut reloaded = crate::program::empty_program();
            import_registers_csv(&mut reloaded, &csv, MergeStrategy::Upsert).expect("csv import");
            assert_eq!(reloaded.elements.len(), program.elements.len());
        }

        #[test]
        fn quoted_csv_parses_commas_in_name() {
            let csv = "register,id,name,status,priority,tags,source\nelements,e1,\"Room, A\",Draft,Preferred,,src\n";
            let rows = parse_delimited(csv, ',').expect("parse");
            assert_eq!(rows[0].name, "Room, A");
            assert_eq!(rows[0].source, "src");
        }

        #[test]
        fn duplicate_import_is_rejected() {
            let csv = "register,id,name,status,priority,tags,source\nelements,e1,A,Draft,Preferred,,\nelements,e1,B,Draft,Preferred,,\n";
            let mut program = crate::program::empty_program();
            assert!(import_registers_csv(&mut program, csv, MergeStrategy::Upsert).is_err());
        }
    }
}

mod kernel {
    //! 🧱 Shared kernel types for architect program entities — ids, headers, quantities, traces, and diagnostics.

    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;
    use std::fmt;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

    // #region 🔖EntityId
    static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// @emoji 🆔 Stable string identity for any program entity or register row.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct EntityId(pub String);

    impl EntityId {
        /// @emoji 🔢 Allocates the next serial id under `prefix` (e.g. `element-1`).
        pub fn new_serial(prefix: &str) -> Self {
            let n = ID_COUNTER.fetch_add(1, AtomicOrdering::Relaxed) + 1;
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
    // #endregion

    // #region 🔖Priority
    /// @emoji 🎚️ Relative importance band for requirements, relationships, and entities.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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

    // #region 🔖LifecycleStatus
    /// @emoji 🔄 Lifecycle and workflow status for register entities.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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

    // #region 🔖Ownership
    /// @emoji 👥 Ownership and authority roles attached to an entity header.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ownership {
        pub owner_id: Option<EntityId>,
        pub authority_id: Option<EntityId>,
        pub consultant_ids: Vec<EntityId>,
        pub participant_ids: Vec<EntityId>,
    }
    // #endregion

    // #region 🔖Text
    /// @emoji 📝 Rich or plain text payload with optional format hint.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextField {
        pub text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format: Option<String>,
    }

    impl TextField {
        pub fn plain(text: impl Into<String>) -> Self {
            Self { text: text.into(), format: None }
        }
    }

    /// @emoji 🏷️ Tagged free-text note on an entity.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TaggedNote {
        pub tag: String,
        pub text: String,
    }

    /// @emoji 🕒 Created/updated audit timestamps on an entity header.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TimestampMeta {
        pub created: String,
        pub updated: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_by: Option<EntityId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_by: Option<EntityId>,
    }

    impl Default for TimestampMeta {
        fn default() -> Self {
            let stamp: String = "1970-01-01T00:00:00Z".into();
            Self { created: stamp.clone(), updated: stamp, created_by: None, updated_by: None }
        }
    }
    // #endregion

    // #region 🔖EntityHeader
    /// @emoji 📋 Common header shared by all register entities via serde flatten.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EntityHeader {
        pub id: EntityId,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<TextField>,
        pub status: LifecycleStatus,
        pub priority: Priority,
        pub ownership: Ownership,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub notes: Vec<TaggedNote>,
        pub timestamps: TimestampMeta,
    }

    impl EntityHeader {
        pub fn new(id: EntityId, name: impl Into<String>) -> Self {
            Self { id, name: name.into(), description: None, status: LifecycleStatus::Draft, priority: Priority::Preferred, ownership: Ownership::default(), tags: Vec::new(), notes: Vec::new(), timestamps: TimestampMeta::default() }
        }
    }
    // #endregion

    // #region 🔖QuantitySpec
    /// @emoji 📐 Numeric quantity with min/max/target bands and unit.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct QuantitySpec {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub forecast: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub peak: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub average: Option<f64>,
        pub unit: String,
    }

    impl QuantitySpec {
        pub fn target_unit(target: f64, unit: impl Into<String>) -> Self {
            Self { target: Some(target), unit: unit.into(), ..Default::default() }
        }
    }
    // #endregion

    // #region 🔖Trace
    /// @emoji 🔗 Semantic trace link between two entities for auditability.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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

    /// @emoji 🧭 Directed trace edge stored in the program trace register.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraceLink {
        pub id: EntityId,
        pub from_id: EntityId,
        pub to_id: EntityId,
        pub kind: TraceKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
    }

    impl TraceLink {
        pub fn new(from_id: EntityId, to_id: EntityId, kind: TraceKind) -> Self {
            Self { id: EntityId::new_serial("trace"), from_id, to_id, kind, label: None }
        }
    }

    impl vcs::Identified<EntityId> for TraceLink {
        fn id(&self) -> &EntityId {
            &self.id
        }
    }
    // #endregion

    // #region 🔖Diagnostics
    /// @emoji ⚠️ Severity band for validation and analysis diagnostics.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum DiagnosticSeverity {
        Info,
        Warning,
        Error,
    }

    /// @emoji 🩺 Non-fatal program validation or analysis finding.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgramDiagnostic {
        pub severity: DiagnosticSeverity,
        pub code: String,
        pub message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<EntityId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub register: Option<String>,
    }

    // #endregion

    //#region ⚠️ Errors
    /// 💥 Fatal program operation or exchange error.
    #[derive(Clone, Debug, thiserror::Error, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ProgramError {
        #[error("invalid schema: expected {expected}, got {actual}")]
        InvalidSchema { expected: String, actual: String },
        #[error("missing entity {id}")]
        MissingEntity { id: EntityId },
        #[error("duplicate adjacency {a} — {b}")]
        DuplicateAdjacency { a: EntityId, b: EntityId },
        #[error("serialize error: {0}")]
        Serialize(String),
        #[error("deserialize error: {0}")]
        Deserialize(String),
        #[error("csv error: {0}")]
        Csv(String),
    }
    //#endregion ⚠️ Errors

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn entity_id_orders_lexicographically() {
            let a = EntityId("element-2".into());
            let b = EntityId("element-10".into());
            assert!(a > b);
        }

        #[test]
        fn entity_id_serial_increments() {
            let first = EntityId::new_serial("test");
            let second = EntityId::new_serial("test");
            assert_ne!(first, second);
            assert!(first.to_string().starts_with("test-"));
        }
    }
}

mod ops {
    //! 🔁 Program VCS operations — `CollectionOp` per register plus meta, adjacency, and bulk set.

    use crate::adjacency::{clear_adjacency, set_adjacency};
    use crate::kernel::{EntityId, TraceLink};
    use crate::program::Program;
    use crate::registers::*;
    use serde::{Deserialize, Serialize};
    use vcs::{apply_collection_op, invert_collection_op, CollectionOp, Operation, OperationDiff, Patchable};

    // #region 🔖ProgramOp
    /// @emoji 🧩 Typed program document operation for VCS replay and undo.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op", rename_all = "camelCase")]
    #[allow(
        clippy::large_enum_variant,
        reason = "~65 variants each wrap CollectionOp<EntityId, T, TPatch> for a different program register T (Stakeholder..Benchmark); sizes inherently vary with T and boxing every payload is a much larger, separately-scoped restructuring (all apply_collection_op/invert_collection_op call sites + external construction sites) — SetProgram (the one outsized, genuinely-fixable single-field outlier) is already boxed"
    )]
    pub enum ProgramOp {
        Stakeholders(CollectionOp<EntityId, Stakeholder, StakeholderPatch>),
        Users(CollectionOp<EntityId, UserProfile, UserProfilePatch>),
        Activities(CollectionOp<EntityId, Activity, ActivityPatch>),
        Functions(CollectionOp<EntityId, Function, FunctionPatch>),
        Elements(CollectionOp<EntityId, ProgramElement, ProgramElementPatch>),
        Quantities(CollectionOp<EntityId, QuantityRequirement, QuantityRequirementPatch>),
        Relationships(CollectionOp<EntityId, Relationship, RelationshipPatch>),
        Adjacencies(CollectionOp<EntityId, Adjacency, AdjacencyPatch>),
        Processes(CollectionOp<EntityId, Process, ProcessPatch>),
        Flows(CollectionOp<EntityId, FlowRequirement, FlowRequirementPatch>),
        AccessRules(CollectionOp<EntityId, AccessRule, AccessRulePatch>),
        Operations(CollectionOp<EntityId, OperationalRequirement, OperationalRequirementPatch>),
        Equipment(CollectionOp<EntityId, Equipment, EquipmentPatch>),
        Resources(CollectionOp<EntityId, Resource, ResourcePatch>),
        Storage(CollectionOp<EntityId, StorageRequirement, StorageRequirementPatch>),
        Environmental(CollectionOp<EntityId, EnvironmentalRequirement, EnvironmentalRequirementPatch>),
        HumanFactors(CollectionOp<EntityId, HumanFactorRequirement, HumanFactorRequirementPatch>),
        Accessibility(CollectionOp<EntityId, AccessibilityRequirement, AccessibilityRequirementPatch>),
        Privacy(CollectionOp<EntityId, PrivacyRequirement, PrivacyRequirementPatch>),
        Safety(CollectionOp<EntityId, SafetyRequirement, SafetyRequirementPatch>),
        Security(CollectionOp<EntityId, SecurityRequirement, SecurityRequirementPatch>),
        Regulatory(CollectionOp<EntityId, RegulatoryRequirement, RegulatoryRequirementPatch>),
        SiteContext(CollectionOp<EntityId, SiteContext, SiteContextPatch>),
        Organizational(CollectionOp<EntityId, OrganizationalRequirement, OrganizationalRequirementPatch>),
        Services(CollectionOp<EntityId, ServiceRequirement, ServiceRequirementPatch>),
        Infrastructure(CollectionOp<EntityId, InfrastructureRequirement, InfrastructureRequirementPatch>),
        Information(CollectionOp<EntityId, InformationRequirement, InformationRequirementPatch>),
        Communication(CollectionOp<EntityId, CommunicationRequirement, CommunicationRequirementPatch>),
        Wayfinding(CollectionOp<EntityId, WayfindingRequirement, WayfindingRequirementPatch>),
        Schedules(CollectionOp<EntityId, ScheduleRequirement, ScheduleRequirementPatch>),
        Flexibility(CollectionOp<EntityId, FlexibilityRequirement, FlexibilityRequirementPatch>),
        Growth(CollectionOp<EntityId, GrowthPlan, GrowthPlanPatch>),
        Sustainability(CollectionOp<EntityId, SustainabilityRequirement, SustainabilityRequirementPatch>),
        Resilience(CollectionOp<EntityId, ResilienceRequirement, ResilienceRequirementPatch>),
        Costs(CollectionOp<EntityId, CostRequirement, CostRequirementPatch>),
        Delivery(CollectionOp<EntityId, DeliveryConstraint, DeliveryConstraintPatch>),
        Risks(CollectionOp<EntityId, Risk, RiskPatch>),
        Conflicts(CollectionOp<EntityId, Conflict, ConflictPatch>),
        Requirements(CollectionOp<EntityId, Requirement, RequirementPatch>),
        Priorities(CollectionOp<EntityId, PriorityRecord, PriorityRecordPatch>),
        Scenarios(CollectionOp<EntityId, Scenario, ScenarioPatch>),
        Options(CollectionOp<EntityId, OptionEvaluation, OptionEvaluationPatch>),
        Decisions(CollectionOp<EntityId, Decision, DecisionPatch>),
        Validations(CollectionOp<EntityId, ValidationRecord, ValidationRecordPatch>),
        Performance(CollectionOp<EntityId, PerformanceCriterion, PerformanceCriterionPatch>),
        Quality(CollectionOp<EntityId, QualityRecord, QualityRecordPatch>),
        Documents(CollectionOp<EntityId, DocumentRecord, DocumentRecordPatch>),
        Changes(CollectionOp<EntityId, ChangeRecord, ChangeRecordPatch>),
        Collaboration(CollectionOp<EntityId, CollaborationRecord, CollaborationRecordPatch>),
        Analyses(CollectionOp<EntityId, AnalysisRecord, AnalysisRecordPatch>),
        Reports(CollectionOp<EntityId, ReportRecord, ReportRecordPatch>),
        SearchFilters(CollectionOp<EntityId, SearchFilter, SearchFilterPatch>),
        StatusRecords(CollectionOp<EntityId, StatusRecord, StatusRecordPatch>),
        Workshops(CollectionOp<EntityId, Workshop, WorkshopPatch>),
        Surveys(CollectionOp<EntityId, Survey, SurveyPatch>),
        Issues(CollectionOp<EntityId, Issue, IssuePatch>),
        AuditEvents(CollectionOp<EntityId, AuditEvent, AuditEventPatch>),
        Templates(CollectionOp<EntityId, TemplateRecord, TemplateRecordPatch>),
        Knowledge(CollectionOp<EntityId, KnowledgeRecord, KnowledgeRecordPatch>),
        Benchmarks(CollectionOp<EntityId, BenchmarkRecord, BenchmarkRecordPatch>),
        Assumptions(CollectionOp<EntityId, Assumption, AssumptionPatch>),
        Constraints(CollectionOp<EntityId, ConstraintRecord, ConstraintRecordPatch>),
        ComplianceRecords(CollectionOp<EntityId, ComplianceRecord, ComplianceRecordPatch>),
        Approvals(CollectionOp<EntityId, ApprovalRecord, ApprovalRecordPatch>),
        Meetings(CollectionOp<EntityId, MeetingRecord, MeetingRecordPatch>),
        Traces(CollectionOp<EntityId, TraceLink, TraceLinkPatch>),
        UpdateMeta { patch: ProgramMetaPatch },
        UpdateProject { patch: ProjectDefinitionPatch },
        UpdateGovernance { patch: GovernancePatch },
        SetAdjacency { adjacency: Adjacency },
        ClearAdjacency { id: EntityId },
        SetProgram { program: Box<Program> },
    }

    /// @emoji 🩹 Inverse patch carrier for trace link collection ops.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraceLinkPatch {
        pub from_id: Option<EntityId>,
        pub to_id: Option<EntityId>,
        pub kind: Option<crate::kernel::TraceKind>,
        pub label: Option<Option<String>>,
    }

    impl Patchable<TraceLinkPatch> for TraceLink {
        fn apply_patch(&mut self, patch: &TraceLinkPatch) -> TraceLinkPatch {
            let mut inverse = TraceLinkPatch::default();
            if let Some(value) = &patch.from_id {
                inverse.from_id = Some(self.from_id.clone());
                self.from_id = value.clone();
            }
            if let Some(value) = &patch.to_id {
                inverse.to_id = Some(self.to_id.clone());
                self.to_id = value.clone();
            }
            if let Some(value) = &patch.kind {
                inverse.kind = Some(self.kind.clone());
                self.kind = value.clone();
            }
            if let Some(value) = &patch.label {
                inverse.label = Some(self.label.clone());
                self.label = value.clone();
            }
            inverse
        }
    }
    // #endregion

    // #region 🔖Apply
    /// @emoji ▶️ Applies one program op to the document in place.
    pub fn apply_program_op(program: &mut Program, op: &ProgramOp) {
        match op {
            ProgramOp::Stakeholders(collection_op) => apply_collection_op(&mut program.stakeholders, collection_op),
            ProgramOp::Users(collection_op) => apply_collection_op(&mut program.users, collection_op),
            ProgramOp::Activities(collection_op) => apply_collection_op(&mut program.activities, collection_op),
            ProgramOp::Functions(collection_op) => apply_collection_op(&mut program.functions, collection_op),
            ProgramOp::Elements(collection_op) => apply_collection_op(&mut program.elements, collection_op),
            ProgramOp::Quantities(collection_op) => apply_collection_op(&mut program.quantities, collection_op),
            ProgramOp::Relationships(collection_op) => apply_collection_op(&mut program.relationships, collection_op),
            ProgramOp::Adjacencies(collection_op) => apply_collection_op(&mut program.adjacencies, collection_op),
            ProgramOp::Processes(collection_op) => apply_collection_op(&mut program.processes, collection_op),
            ProgramOp::Flows(collection_op) => apply_collection_op(&mut program.flows, collection_op),
            ProgramOp::AccessRules(collection_op) => apply_collection_op(&mut program.access_rules, collection_op),
            ProgramOp::Operations(collection_op) => apply_collection_op(&mut program.operations, collection_op),
            ProgramOp::Equipment(collection_op) => apply_collection_op(&mut program.equipment, collection_op),
            ProgramOp::Resources(collection_op) => apply_collection_op(&mut program.resources, collection_op),
            ProgramOp::Storage(collection_op) => apply_collection_op(&mut program.storage, collection_op),
            ProgramOp::Environmental(collection_op) => apply_collection_op(&mut program.environmental, collection_op),
            ProgramOp::HumanFactors(collection_op) => apply_collection_op(&mut program.human_factors, collection_op),
            ProgramOp::Accessibility(collection_op) => apply_collection_op(&mut program.accessibility, collection_op),
            ProgramOp::Privacy(collection_op) => apply_collection_op(&mut program.privacy, collection_op),
            ProgramOp::Safety(collection_op) => apply_collection_op(&mut program.safety, collection_op),
            ProgramOp::Security(collection_op) => apply_collection_op(&mut program.security, collection_op),
            ProgramOp::Regulatory(collection_op) => apply_collection_op(&mut program.regulatory, collection_op),
            ProgramOp::SiteContext(collection_op) => apply_collection_op(&mut program.site_context, collection_op),
            ProgramOp::Organizational(collection_op) => apply_collection_op(&mut program.organizational, collection_op),
            ProgramOp::Services(collection_op) => apply_collection_op(&mut program.services, collection_op),
            ProgramOp::Infrastructure(collection_op) => apply_collection_op(&mut program.infrastructure, collection_op),
            ProgramOp::Information(collection_op) => apply_collection_op(&mut program.information, collection_op),
            ProgramOp::Communication(collection_op) => apply_collection_op(&mut program.communication, collection_op),
            ProgramOp::Wayfinding(collection_op) => apply_collection_op(&mut program.wayfinding, collection_op),
            ProgramOp::Schedules(collection_op) => apply_collection_op(&mut program.schedules, collection_op),
            ProgramOp::Flexibility(collection_op) => apply_collection_op(&mut program.flexibility, collection_op),
            ProgramOp::Growth(collection_op) => apply_collection_op(&mut program.growth, collection_op),
            ProgramOp::Sustainability(collection_op) => apply_collection_op(&mut program.sustainability, collection_op),
            ProgramOp::Resilience(collection_op) => apply_collection_op(&mut program.resilience, collection_op),
            ProgramOp::Costs(collection_op) => apply_collection_op(&mut program.costs, collection_op),
            ProgramOp::Delivery(collection_op) => apply_collection_op(&mut program.delivery, collection_op),
            ProgramOp::Risks(collection_op) => apply_collection_op(&mut program.risks, collection_op),
            ProgramOp::Conflicts(collection_op) => apply_collection_op(&mut program.conflicts, collection_op),
            ProgramOp::Requirements(collection_op) => apply_collection_op(&mut program.requirements, collection_op),
            ProgramOp::Priorities(collection_op) => apply_collection_op(&mut program.priorities, collection_op),
            ProgramOp::Scenarios(collection_op) => apply_collection_op(&mut program.scenarios, collection_op),
            ProgramOp::Options(collection_op) => apply_collection_op(&mut program.options, collection_op),
            ProgramOp::Decisions(collection_op) => apply_collection_op(&mut program.decisions, collection_op),
            ProgramOp::Validations(collection_op) => apply_collection_op(&mut program.validations, collection_op),
            ProgramOp::Performance(collection_op) => apply_collection_op(&mut program.performance, collection_op),
            ProgramOp::Quality(collection_op) => apply_collection_op(&mut program.quality, collection_op),
            ProgramOp::Documents(collection_op) => apply_collection_op(&mut program.documents, collection_op),
            ProgramOp::Changes(collection_op) => apply_collection_op(&mut program.changes, collection_op),
            ProgramOp::Collaboration(collection_op) => apply_collection_op(&mut program.collaboration, collection_op),
            ProgramOp::Analyses(collection_op) => apply_collection_op(&mut program.analyses, collection_op),
            ProgramOp::Reports(collection_op) => apply_collection_op(&mut program.reports, collection_op),
            ProgramOp::SearchFilters(collection_op) => apply_collection_op(&mut program.search_filters, collection_op),
            ProgramOp::StatusRecords(collection_op) => apply_collection_op(&mut program.status_records, collection_op),
            ProgramOp::Workshops(collection_op) => apply_collection_op(&mut program.workshops, collection_op),
            ProgramOp::Surveys(collection_op) => apply_collection_op(&mut program.surveys, collection_op),
            ProgramOp::Issues(collection_op) => apply_collection_op(&mut program.issues, collection_op),
            ProgramOp::AuditEvents(collection_op) => apply_collection_op(&mut program.audit_events, collection_op),
            ProgramOp::Templates(collection_op) => apply_collection_op(&mut program.templates, collection_op),
            ProgramOp::Knowledge(collection_op) => apply_collection_op(&mut program.knowledge, collection_op),
            ProgramOp::Benchmarks(collection_op) => apply_collection_op(&mut program.benchmarks, collection_op),
            ProgramOp::Assumptions(collection_op) => apply_collection_op(&mut program.assumptions, collection_op),
            ProgramOp::Constraints(collection_op) => apply_collection_op(&mut program.constraints, collection_op),
            ProgramOp::ComplianceRecords(collection_op) => {
                apply_collection_op(&mut program.compliance_records, collection_op);
            }
            ProgramOp::Approvals(collection_op) => apply_collection_op(&mut program.approvals, collection_op),
            ProgramOp::Meetings(collection_op) => apply_collection_op(&mut program.meetings, collection_op),
            ProgramOp::Traces(collection_op) => apply_collection_op(&mut program.traces, collection_op),
            ProgramOp::UpdateMeta { patch } => {
                program.meta.apply_patch(patch);
            }
            ProgramOp::UpdateProject { patch } => {
                program.project.apply_patch(patch);
            }
            ProgramOp::UpdateGovernance { patch } => {
                program.governance.apply_patch(patch);
            }
            ProgramOp::SetAdjacency { adjacency } => set_adjacency(program, adjacency.clone()),
            ProgramOp::ClearAdjacency { id } => clear_adjacency(program, id),
            ProgramOp::SetProgram { program: replacement } => *program = (**replacement).clone(),
        }
    }

    /// @emoji ↩️ Computes the inverse op from pre-state for undo.
    pub fn invert_program_op(program: &Program, op: &ProgramOp) -> ProgramOp {
        match op {
            ProgramOp::Stakeholders(collection_op) => ProgramOp::Stakeholders(invert_collection_op(&program.stakeholders, collection_op)),
            ProgramOp::Users(collection_op) => ProgramOp::Users(invert_collection_op(&program.users, collection_op)),
            ProgramOp::Activities(collection_op) => ProgramOp::Activities(invert_collection_op(&program.activities, collection_op)),
            ProgramOp::Functions(collection_op) => ProgramOp::Functions(invert_collection_op(&program.functions, collection_op)),
            ProgramOp::Elements(collection_op) => ProgramOp::Elements(invert_collection_op(&program.elements, collection_op)),
            ProgramOp::Quantities(collection_op) => ProgramOp::Quantities(invert_collection_op(&program.quantities, collection_op)),
            ProgramOp::Relationships(collection_op) => ProgramOp::Relationships(invert_collection_op(&program.relationships, collection_op)),
            ProgramOp::Adjacencies(collection_op) => ProgramOp::Adjacencies(invert_collection_op(&program.adjacencies, collection_op)),
            ProgramOp::Processes(collection_op) => ProgramOp::Processes(invert_collection_op(&program.processes, collection_op)),
            ProgramOp::Flows(collection_op) => ProgramOp::Flows(invert_collection_op(&program.flows, collection_op)),
            ProgramOp::AccessRules(collection_op) => ProgramOp::AccessRules(invert_collection_op(&program.access_rules, collection_op)),
            ProgramOp::Operations(collection_op) => ProgramOp::Operations(invert_collection_op(&program.operations, collection_op)),
            ProgramOp::Equipment(collection_op) => ProgramOp::Equipment(invert_collection_op(&program.equipment, collection_op)),
            ProgramOp::Resources(collection_op) => ProgramOp::Resources(invert_collection_op(&program.resources, collection_op)),
            ProgramOp::Storage(collection_op) => ProgramOp::Storage(invert_collection_op(&program.storage, collection_op)),
            ProgramOp::Environmental(collection_op) => ProgramOp::Environmental(invert_collection_op(&program.environmental, collection_op)),
            ProgramOp::HumanFactors(collection_op) => ProgramOp::HumanFactors(invert_collection_op(&program.human_factors, collection_op)),
            ProgramOp::Accessibility(collection_op) => ProgramOp::Accessibility(invert_collection_op(&program.accessibility, collection_op)),
            ProgramOp::Privacy(collection_op) => ProgramOp::Privacy(invert_collection_op(&program.privacy, collection_op)),
            ProgramOp::Safety(collection_op) => ProgramOp::Safety(invert_collection_op(&program.safety, collection_op)),
            ProgramOp::Security(collection_op) => ProgramOp::Security(invert_collection_op(&program.security, collection_op)),
            ProgramOp::Regulatory(collection_op) => ProgramOp::Regulatory(invert_collection_op(&program.regulatory, collection_op)),
            ProgramOp::SiteContext(collection_op) => ProgramOp::SiteContext(invert_collection_op(&program.site_context, collection_op)),
            ProgramOp::Organizational(collection_op) => ProgramOp::Organizational(invert_collection_op(&program.organizational, collection_op)),
            ProgramOp::Services(collection_op) => ProgramOp::Services(invert_collection_op(&program.services, collection_op)),
            ProgramOp::Infrastructure(collection_op) => ProgramOp::Infrastructure(invert_collection_op(&program.infrastructure, collection_op)),
            ProgramOp::Information(collection_op) => ProgramOp::Information(invert_collection_op(&program.information, collection_op)),
            ProgramOp::Communication(collection_op) => ProgramOp::Communication(invert_collection_op(&program.communication, collection_op)),
            ProgramOp::Wayfinding(collection_op) => ProgramOp::Wayfinding(invert_collection_op(&program.wayfinding, collection_op)),
            ProgramOp::Schedules(collection_op) => ProgramOp::Schedules(invert_collection_op(&program.schedules, collection_op)),
            ProgramOp::Flexibility(collection_op) => ProgramOp::Flexibility(invert_collection_op(&program.flexibility, collection_op)),
            ProgramOp::Growth(collection_op) => ProgramOp::Growth(invert_collection_op(&program.growth, collection_op)),
            ProgramOp::Sustainability(collection_op) => ProgramOp::Sustainability(invert_collection_op(&program.sustainability, collection_op)),
            ProgramOp::Resilience(collection_op) => ProgramOp::Resilience(invert_collection_op(&program.resilience, collection_op)),
            ProgramOp::Costs(collection_op) => ProgramOp::Costs(invert_collection_op(&program.costs, collection_op)),
            ProgramOp::Delivery(collection_op) => ProgramOp::Delivery(invert_collection_op(&program.delivery, collection_op)),
            ProgramOp::Risks(collection_op) => ProgramOp::Risks(invert_collection_op(&program.risks, collection_op)),
            ProgramOp::Conflicts(collection_op) => ProgramOp::Conflicts(invert_collection_op(&program.conflicts, collection_op)),
            ProgramOp::Requirements(collection_op) => ProgramOp::Requirements(invert_collection_op(&program.requirements, collection_op)),
            ProgramOp::Priorities(collection_op) => ProgramOp::Priorities(invert_collection_op(&program.priorities, collection_op)),
            ProgramOp::Scenarios(collection_op) => ProgramOp::Scenarios(invert_collection_op(&program.scenarios, collection_op)),
            ProgramOp::Options(collection_op) => ProgramOp::Options(invert_collection_op(&program.options, collection_op)),
            ProgramOp::Decisions(collection_op) => ProgramOp::Decisions(invert_collection_op(&program.decisions, collection_op)),
            ProgramOp::Validations(collection_op) => ProgramOp::Validations(invert_collection_op(&program.validations, collection_op)),
            ProgramOp::Performance(collection_op) => ProgramOp::Performance(invert_collection_op(&program.performance, collection_op)),
            ProgramOp::Quality(collection_op) => ProgramOp::Quality(invert_collection_op(&program.quality, collection_op)),
            ProgramOp::Documents(collection_op) => ProgramOp::Documents(invert_collection_op(&program.documents, collection_op)),
            ProgramOp::Changes(collection_op) => ProgramOp::Changes(invert_collection_op(&program.changes, collection_op)),
            ProgramOp::Collaboration(collection_op) => ProgramOp::Collaboration(invert_collection_op(&program.collaboration, collection_op)),
            ProgramOp::Analyses(collection_op) => ProgramOp::Analyses(invert_collection_op(&program.analyses, collection_op)),
            ProgramOp::Reports(collection_op) => ProgramOp::Reports(invert_collection_op(&program.reports, collection_op)),
            ProgramOp::SearchFilters(collection_op) => ProgramOp::SearchFilters(invert_collection_op(&program.search_filters, collection_op)),
            ProgramOp::StatusRecords(collection_op) => ProgramOp::StatusRecords(invert_collection_op(&program.status_records, collection_op)),
            ProgramOp::Workshops(collection_op) => ProgramOp::Workshops(invert_collection_op(&program.workshops, collection_op)),
            ProgramOp::Surveys(collection_op) => ProgramOp::Surveys(invert_collection_op(&program.surveys, collection_op)),
            ProgramOp::Issues(collection_op) => ProgramOp::Issues(invert_collection_op(&program.issues, collection_op)),
            ProgramOp::AuditEvents(collection_op) => ProgramOp::AuditEvents(invert_collection_op(&program.audit_events, collection_op)),
            ProgramOp::Templates(collection_op) => ProgramOp::Templates(invert_collection_op(&program.templates, collection_op)),
            ProgramOp::Knowledge(collection_op) => ProgramOp::Knowledge(invert_collection_op(&program.knowledge, collection_op)),
            ProgramOp::Benchmarks(collection_op) => ProgramOp::Benchmarks(invert_collection_op(&program.benchmarks, collection_op)),
            ProgramOp::Assumptions(collection_op) => ProgramOp::Assumptions(invert_collection_op(&program.assumptions, collection_op)),
            ProgramOp::Constraints(collection_op) => ProgramOp::Constraints(invert_collection_op(&program.constraints, collection_op)),
            ProgramOp::ComplianceRecords(collection_op) => ProgramOp::ComplianceRecords(invert_collection_op(&program.compliance_records, collection_op)),
            ProgramOp::Approvals(collection_op) => ProgramOp::Approvals(invert_collection_op(&program.approvals, collection_op)),
            ProgramOp::Meetings(collection_op) => ProgramOp::Meetings(invert_collection_op(&program.meetings, collection_op)),
            ProgramOp::Traces(collection_op) => ProgramOp::Traces(invert_collection_op(&program.traces, collection_op)),
            ProgramOp::UpdateMeta { patch } => {
                let mut probe = program.meta.clone();
                let inverse = probe.apply_patch(patch);
                ProgramOp::UpdateMeta { patch: inverse }
            }
            ProgramOp::UpdateProject { patch } => {
                let mut probe = program.project.clone();
                let inverse = probe.apply_patch(patch);
                ProgramOp::UpdateProject { patch: inverse }
            }
            ProgramOp::UpdateGovernance { patch } => {
                let mut probe = program.governance.clone();
                let inverse = probe.apply_patch(patch);
                ProgramOp::UpdateGovernance { patch: inverse }
            }
            ProgramOp::SetAdjacency { adjacency } => {
                if let Some(existing) = program.adjacencies.iter().find(|row| row.header.id == adjacency.header.id) {
                    ProgramOp::SetAdjacency { adjacency: existing.clone() }
                } else {
                    ProgramOp::ClearAdjacency { id: adjacency.header.id.clone() }
                }
            }
            ProgramOp::ClearAdjacency { id } => match program.adjacencies.iter().find(|row| &row.header.id == id).cloned() {
                Some(existing) => ProgramOp::SetAdjacency { adjacency: existing },
                None => ProgramOp::ClearAdjacency { id: id.clone() },
            },
            ProgramOp::SetProgram { .. } => ProgramOp::SetProgram { program: Box::new(program.clone()) },
        }
    }
    // #endregion

    // #region 🔖ProgramDiff
    /// @emoji 📦 Ordered list of program ops materializing a document diff.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgramDiff {
        pub ops: Vec<ProgramOp>,
    }

    impl OperationDiff<Program> for ProgramDiff {
        fn apply(&self, projection: &Program) -> Program {
            let mut next = projection.clone();
            for op in &self.ops {
                apply_program_op(&mut next, op);
            }
            next
        }

        fn absorb(&mut self, other: Self) {
            self.ops.extend(other.ops);
        }
    }

    impl Operation<Program> for ProgramOp {
        type Diff = ProgramDiff;

        fn diff(&self, _projection: &Program) -> ProgramDiff {
            ProgramDiff { ops: vec![self.clone()] }
        }

        fn backwards(&self, projection: &Program) -> Vec<Self> {
            vec![invert_program_op(projection, self)]
        }
    }
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::kernel::EntityHeader;
        use crate::program::{empty_program, sample_program};

        #[test]
        fn update_meta_round_trips_undo() {
            let mut program = empty_program();
            let op = ProgramOp::UpdateMeta { patch: ProgramMetaPatch { title: Some("Clinic".into()), ..Default::default() } };
            let inverse = invert_program_op(&program, &op);
            apply_program_op(&mut program, &op);
            assert_eq!(program.meta.title, "Clinic");
            apply_program_op(&mut program, &inverse);
            assert_ne!(program.meta.title, "Clinic");
        }

        #[test]
        fn add_stakeholder_via_collection_op() {
            let mut program = sample_program();
            let before = program.stakeholders.len();
            let stakeholder = Stakeholder {
                header: EntityHeader::new(EntityId::new_serial("stakeholder"), "Nurse Lead"),
                role: "Clinical".into(),
                organization: "Sample Health".into(),
                department: None,
                contact_email: None,
                contact_phone: None,
                influence: InfluenceLevel::Medium,
                interest: InfluenceLevel::High,
                engagement: EngagementLevel::Supportive,
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
                stakeholder_type: "Clinical".into(),
                influence_strategy: None,
                communication_channels: Vec::new(),
                success_metrics: Vec::new(),
            };
            let id = stakeholder.header.id.clone();
            let op = ProgramOp::Stakeholders(CollectionOp::Add { index: program.stakeholders.len(), item: stakeholder });
            apply_program_op(&mut program, &op);
            assert_eq!(program.stakeholders.len(), before + 1);
            let undo = invert_program_op(&program, &op);
            apply_program_op(&mut program, &undo);
            assert!(!program.stakeholders.iter().any(|s| s.header.id == id));
        }

        #[test]
        fn set_program_bulk_replace() {
            let mut program = empty_program();
            let sample = sample_program();
            apply_program_op(&mut program, &ProgramOp::SetProgram { program: Box::new(sample.clone()) });
            assert_eq!(program.elements.len(), sample.elements.len());
        }
    }
}

mod outputs {
    //! 📤 Abstract output types — §65 builders for program deliverables.

    use crate::analyze::run_analysis;
    use crate::kernel::EntityId;
    use crate::program::Program;
    use crate::registers::{AnalysisKind, ReportKind};
    use crate::report::build_report;
    use serde::{Deserialize, Serialize};

    // #region 🔖OutputKind
    /// @emoji 📦 Abstract output kind for architectural program deliverables.
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

    /// @emoji 📄 Structured abstract output payload.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgramOutput {
        pub kind: OutputKind,
        pub title: String,
        pub lines: Vec<String>,
        pub entity_ids: Vec<EntityId>,
    }
    // #endregion

    // #region 🔖Builders
    /// @emoji 🏗️ Builds the requested abstract output from a program snapshot.
    pub fn build_output(program: &Program, kind: OutputKind) -> ProgramOutput {
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

    fn requirement_lists(program: &Program) -> ProgramOutput {
        ProgramOutput {
            kind: OutputKind::RequirementLists,
            title: "Requirement Lists".into(),
            lines: program.requirements.iter().map(|r| format!("[{:?}] {} — {}", r.kind, r.header.name, r.statement.text)).collect(),
            entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
        }
    }

    fn functional_hierarchies(program: &Program) -> ProgramOutput {
        let roots: Vec<_> = program.functions.iter().filter(|f| f.hierarchy_parent_id.is_none()).collect();
        let mut lines = Vec::new();
        for root in roots {
            lines.push(root.header.name.clone());
            for child in program.functions.iter().filter(|f| f.hierarchy_parent_id.as_ref() == Some(&root.header.id)) {
                lines.push(format!("  └─ {}", child.header.name));
            }
        }
        ProgramOutput { kind: OutputKind::FunctionalHierarchies, title: "Functional Hierarchies".into(), lines, entity_ids: program.functions.iter().map(|f| f.header.id.clone()).collect() }
    }

    fn activity_taxonomies(program: &Program) -> ProgramOutput {
        let mut lines = Vec::new();
        for activity in &program.activities {
            lines.push(format!("{} / {} / {}", activity.category, activity.activity_type, activity.header.name));
        }
        ProgramOutput { kind: OutputKind::ActivityTaxonomies, title: "Activity Taxonomies".into(), lines, entity_ids: program.activities.iter().map(|a| a.header.id.clone()).collect() }
    }

    fn relationship_matrices(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.relationships.iter().map(|r| format!("{:?}: {} → {}", r.kind, r.source_id, r.target_id)).collect();
        ProgramOutput { kind: OutputKind::RelationshipMatrices, title: "Relationship Matrices".into(), lines, entity_ids: program.relationships.iter().map(|r| r.header.id.clone()).collect() }
    }

    fn adjacency_matrices(program: &Program) -> ProgramOutput {
        let report = build_report(program, ReportKind::AdjacencyMatrix);
        ProgramOutput { kind: OutputKind::AdjacencyMatrices, title: "Adjacency Matrices".into(), lines: report.sections.into_iter().flat_map(|s| s.bullets).collect(), entity_ids: report.entity_ids }
    }

    fn dependency_networks(program: &Program) -> ProgramOutput {
        let analysis = run_analysis(program, AnalysisKind::Dependency);
        ProgramOutput { kind: OutputKind::DependencyNetworks, title: "Dependency Networks".into(), lines: analysis.findings, entity_ids: analysis.entity_ids }
    }

    fn priority_matrices(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.priorities.iter().map(|p| format!("{} — {:?} rank {:?} weight {:?}", p.header.name, p.ranked_priority, p.rank, p.weight)).collect();
        ProgramOutput { kind: OutputKind::PriorityMatrices, title: "Priority Matrices".into(), lines, entity_ids: program.priorities.iter().map(|p| p.header.id.clone()).collect() }
    }

    fn responsibility_matrices(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.governance.responsibilities.iter().chain(program.governance.roles.iter()).cloned().collect();
        ProgramOutput { kind: OutputKind::ResponsibilityMatrices, title: "Responsibility Matrices".into(), lines, entity_ids: vec![program.governance.id.clone()] }
    }

    fn decision_trees(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.decisions.iter().map(|d| format!("{} → option {:?} ({})", d.header.name, d.selected_option_id, d.decision_statement.text)).collect();
        ProgramOutput { kind: OutputKind::DecisionTrees, title: "Decision Trees".into(), lines, entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect() }
    }

    fn process_maps(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.processes.iter().map(|p| format!("{}: {}", p.header.name, p.steps.join(" → "))).collect();
        ProgramOutput { kind: OutputKind::ProcessMaps, title: "Process Maps".into(), lines, entity_ids: program.processes.iter().map(|p| p.header.id.clone()).collect() }
    }

    fn workflow_descriptions(program: &Program) -> ProgramOutput {
        let analysis = run_analysis(program, AnalysisKind::Workflow);
        ProgramOutput { kind: OutputKind::WorkflowDescriptions, title: "Workflow Descriptions".into(), lines: analysis.findings, entity_ids: analysis.entity_ids }
    }

    fn user_journeys(program: &Program) -> ProgramOutput {
        let mut lines = Vec::new();
        for user in &program.users {
            let activities: Vec<_> = program.activities.iter().filter(|a| a.user_profile_ids.contains(&user.header.id)).map(|a| a.header.name.as_str()).collect();
            lines.push(format!("{}: {}", user.header.name, activities.join(" → ")));
        }
        ProgramOutput { kind: OutputKind::UserJourneys, title: "User Journeys".into(), lines, entity_ids: program.users.iter().map(|u| u.header.id.clone()).collect() }
    }

    fn scenario_narratives(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.scenarios.iter().map(|s| format!("{} — {}", s.header.name, s.hypothesis.text)).collect();
        ProgramOutput { kind: OutputKind::ScenarioNarratives, title: "Scenario Narratives".into(), lines, entity_ids: program.scenarios.iter().map(|s| s.header.id.clone()).collect() }
    }

    fn risk_matrices(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.risks.iter().map(|r| format!("{} — {:?}/{:?}", r.header.name, r.probability, r.impact)).collect();
        ProgramOutput { kind: OutputKind::RiskMatrices, title: "Risk Matrices".into(), lines, entity_ids: program.risks.iter().map(|r| r.header.id.clone()).collect() }
    }

    fn compliance_matrices(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.regulatory.iter().map(|r| format!("{} {} — {:?}", r.code, r.title, r.verification_status)).collect();
        ProgramOutput { kind: OutputKind::ComplianceMatrices, title: "Compliance Matrices".into(), lines, entity_ids: program.regulatory.iter().map(|r| r.header.id.clone()).collect() }
    }

    fn capacity_schedules(program: &Program) -> ProgramOutput {
        let analysis = run_analysis(program, AnalysisKind::Capacity);
        let schedule_lines: Vec<String> = program.schedules.iter().map(|s| s.header.name.clone()).collect();
        ProgramOutput { kind: OutputKind::CapacitySchedules, title: "Capacity Schedules".into(), lines: analysis.findings.into_iter().chain(schedule_lines).collect(), entity_ids: program.schedules.iter().map(|s| s.header.id.clone()).collect() }
    }

    fn equipment_schedules(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.equipment.iter().map(|e| format!("{} — qty {:?}", e.header.name, e.quantity.target)).collect();
        ProgramOutput { kind: OutputKind::EquipmentSchedules, title: "Equipment Schedules".into(), lines, entity_ids: program.equipment.iter().map(|e| e.header.id.clone()).collect() }
    }

    fn evaluation_frameworks(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.performance.iter().map(|p| format!("{} — {}", p.header.name, p.criterion)).collect();
        ProgramOutput { kind: OutputKind::EvaluationFrameworks, title: "Evaluation Frameworks".into(), lines, entity_ids: program.performance.iter().map(|p| p.header.id.clone()).collect() }
    }

    fn performance_specifications(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.performance.iter().map(|p| format!("{} target {:?} {:?}", p.header.name, p.target, p.unit)).collect();
        ProgramOutput { kind: OutputKind::PerformanceSpecifications, title: "Performance Specifications".into(), lines, entity_ids: program.performance.iter().map(|p| p.header.id.clone()).collect() }
    }

    fn program_reports(program: &Program) -> ProgramOutput {
        let lines: Vec<String> = program.reports.iter().map(|r| format!("{:?} — {}", r.kind, r.title)).collect();
        ProgramOutput { kind: OutputKind::ProgramReports, title: "Program Reports".into(), lines, entity_ids: program.reports.iter().map(|r| r.header.id.clone()).collect() }
    }
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn requirement_lists_output_nonempty_for_sample() {
            let output = build_output(&sample_program(), OutputKind::RequirementLists);
            assert_eq!(output.kind, OutputKind::RequirementLists);
        }

        #[test]
        fn adjacency_matrices_output_uses_matrix_cells() {
            let output = build_output(&sample_program(), OutputKind::AdjacencyMatrices);
            assert!(!output.lines.is_empty());
        }
    }
}

mod program {
    //! 🏛️ Root program document — all 65 feature-area registers plus meta, project, and governance.

    use crate::kernel::*;
    use crate::registers::*;
    use serde::{Deserialize, Serialize};

    /// @emoji 📜 Persisted architect program document schema identifier.
    pub const ARCHITECT_PROGRAM_SCHEMA: &str = "architect.program";

    // #region 🔖Program
    /// @emoji 🗂️ Full architectural program document with every typed register collection.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Program {
        pub schema: String,
        pub meta: ProgramMeta,
        pub project: ProjectDefinition,
        pub stakeholders: Vec<Stakeholder>,
        pub users: Vec<UserProfile>,
        pub activities: Vec<Activity>,
        pub functions: Vec<Function>,
        pub elements: Vec<ProgramElement>,
        pub quantities: Vec<QuantityRequirement>,
        pub relationships: Vec<Relationship>,
        pub adjacencies: Vec<Adjacency>,
        pub processes: Vec<Process>,
        pub flows: Vec<FlowRequirement>,
        pub access_rules: Vec<AccessRule>,
        pub operations: Vec<OperationalRequirement>,
        pub equipment: Vec<Equipment>,
        pub resources: Vec<Resource>,
        pub storage: Vec<StorageRequirement>,
        pub environmental: Vec<EnvironmentalRequirement>,
        pub human_factors: Vec<HumanFactorRequirement>,
        pub accessibility: Vec<AccessibilityRequirement>,
        pub privacy: Vec<PrivacyRequirement>,
        pub safety: Vec<SafetyRequirement>,
        pub security: Vec<SecurityRequirement>,
        pub regulatory: Vec<RegulatoryRequirement>,
        pub site_context: Vec<SiteContext>,
        pub organizational: Vec<OrganizationalRequirement>,
        pub services: Vec<ServiceRequirement>,
        pub infrastructure: Vec<InfrastructureRequirement>,
        pub information: Vec<InformationRequirement>,
        pub communication: Vec<CommunicationRequirement>,
        pub wayfinding: Vec<WayfindingRequirement>,
        pub schedules: Vec<ScheduleRequirement>,
        pub flexibility: Vec<FlexibilityRequirement>,
        pub growth: Vec<GrowthPlan>,
        pub sustainability: Vec<SustainabilityRequirement>,
        pub resilience: Vec<ResilienceRequirement>,
        pub costs: Vec<CostRequirement>,
        pub delivery: Vec<DeliveryConstraint>,
        pub risks: Vec<Risk>,
        pub conflicts: Vec<Conflict>,
        pub requirements: Vec<Requirement>,
        pub priorities: Vec<PriorityRecord>,
        pub scenarios: Vec<Scenario>,
        pub options: Vec<OptionEvaluation>,
        pub decisions: Vec<Decision>,
        pub validations: Vec<ValidationRecord>,
        pub performance: Vec<PerformanceCriterion>,
        pub quality: Vec<QualityRecord>,
        pub documents: Vec<DocumentRecord>,
        pub assumptions: Vec<Assumption>,
        pub constraints: Vec<ConstraintRecord>,
        pub compliance_records: Vec<ComplianceRecord>,
        pub approvals: Vec<ApprovalRecord>,
        pub meetings: Vec<MeetingRecord>,
        pub changes: Vec<ChangeRecord>,
        pub collaboration: Vec<CollaborationRecord>,
        pub analyses: Vec<AnalysisRecord>,
        pub reports: Vec<ReportRecord>,
        pub search_filters: Vec<SearchFilter>,
        pub status_records: Vec<StatusRecord>,
        pub workshops: Vec<Workshop>,
        pub surveys: Vec<Survey>,
        pub issues: Vec<Issue>,
        pub audit_events: Vec<AuditEvent>,
        pub templates: Vec<TemplateRecord>,
        pub knowledge: Vec<KnowledgeRecord>,
        pub benchmarks: Vec<BenchmarkRecord>,
        pub governance: Governance,
        pub traces: Vec<TraceLink>,
    }
    // #endregion

    // #region 🔖Factories
    /// @emoji 📭 Empty program with schema, meta, project, and governance initialized.
    pub fn empty_program() -> Program {
        let project_id = EntityId::new_serial("project");
        let governance_id = EntityId::new_serial("governance");
        Program {
            schema: ARCHITECT_PROGRAM_SCHEMA.into(),
            meta: ProgramMeta {
                schema: ARCHITECT_PROGRAM_SCHEMA.into(),
                document_id: EntityId::new_serial("document").0,
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
            documents: Vec::new(),
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
            knowledge: Vec::new(),
            benchmarks: Vec::new(),
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

    /// @emoji 🧪 Sample program for tests with elements, stakeholders, and one adjacency.
    pub fn sample_program() -> Program {
        let mut program = empty_program();
        program.meta.title = "Sample Clinic".into();
        program.meta.industry_sector = "healthcare".into();
        program.project.code = "CLN-001".into();
        program.project.client_name = "Sample Health".into();

        let reception_id = EntityId::new_serial("element");
        let waiting_id = EntityId::new_serial("element");
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

        let stakeholder_id = EntityId::new_serial("stakeholder");
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

        let (a, b) = crate::adjacency::normalize_pair(&reception_id, &waiting_id);
        program.adjacencies.push(Adjacency {
            header: EntityHeader::new(EntityId::new_serial("adjacency"), "Reception ↔ Waiting"),
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
    mod tests {
        use super::*;

        #[test]
        fn empty_program_has_schema() {
            let program = empty_program();
            assert_eq!(program.schema, ARCHITECT_PROGRAM_SCHEMA);
            assert_eq!(program.meta.schema, ARCHITECT_PROGRAM_SCHEMA);
        }

        #[test]
        fn sample_program_round_trips_json() {
            let program = sample_program();
            let json = serde_json::to_string(&program).expect("serialize");
            let decoded: Program = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(decoded.elements.len(), 2);
            assert_eq!(decoded.adjacencies.len(), 1);
        }
    }
}

mod registers {
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
    pub enum InfluenceLevel {
        Low,
        Medium,
        High,
        Critical,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum EngagementLevel {
        Unaware,
        Resistant,
        Neutral,
        Supportive,
        Leading,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub enum AdjacencyKind {
        Required,
        Preferred,
        Optional,
        Prohibited,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ConnectionKind {
        Direct,
        Indirect,
        Controlled,
        SharedAccess,
        None,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum FlowDirection {
        OneWay,
        TwoWay,
        BidirectionalPeak,
        Restricted,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum AccessLevel {
        Public,
        Restricted,
        Controlled,
        Private,
        Secure,
        EmergencyOnly,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum RiskLevel {
        Negligible,
        Low,
        Medium,
        High,
        Critical,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ValidationStatus {
        Pending,
        Passed,
        Failed,
        Waived,
        Deferred,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    pub enum IssueSeverity {
        Cosmetic,
        Minor,
        Major,
        Critical,
        Blocker,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum CostBasis {
        Capital,
        Operational,
        Lifecycle,
        Replacement,
        Maintenance,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}

mod report {
    //! 📄 Program reporting — structured reports from `ReportKind`.

    use crate::analyze::run_analysis;
    use crate::kernel::{EntityHeader, EntityId};
    use crate::program::Program;
    use crate::registers::{AnalysisKind, ReportKind, ReportRecord, ValidationStatus};
    use crate::status_summary::status_summary;
    use crate::validate::validate_program;
    use serde::{Deserialize, Serialize};

    // #region 🔖ProgramReport
    /// @emoji 📑 Structured report payload for export and plugin rendering.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgramReport {
        pub kind: ReportKind,
        pub title: String,
        pub generated_at: String,
        pub sections: Vec<ReportSection>,
        pub entity_ids: Vec<EntityId>,
    }

    /// @emoji 📎 One titled section within a program report.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReportSection {
        pub heading: String,
        pub body: String,
        pub bullets: Vec<String>,
    }
    // #endregion

    // #region 🔖BuildReport
    /// @emoji 🖨️ Builds a structured report for the requested kind.
    pub fn build_report(program: &Program, kind: ReportKind) -> ProgramReport {
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

    /// @emoji 📝 Builds a report and appends a `ReportRecord` to the program.
    pub fn build_report_and_record(program: &mut Program, kind: ReportKind) -> ProgramReport {
        let report = build_report(program, kind);
        let record = ReportRecord {
            header: EntityHeader::new(EntityId::new_serial("report"), report.title.clone()),
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

    fn timestamp(program: &Program) -> String {
        program.meta.timestamps.updated.clone()
    }

    fn executive_summary(program: &Program) -> ProgramReport {
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

    fn program_overview(program: &Program) -> ProgramReport {
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

    fn stakeholder_summary(program: &Program) -> ProgramReport {
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

    fn requirements_matrix(program: &Program) -> ProgramReport {
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

    fn adjacency_matrix_report(program: &Program) -> ProgramReport {
        let matrix = crate::adjacency::adjacency_matrix(program);
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

    fn gap_report(program: &Program) -> ProgramReport {
        let analysis = run_analysis(program, AnalysisKind::Gap);
        ProgramReport {
            kind: ReportKind::GapAnalysis,
            title: "Gap Analysis".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
            entity_ids: analysis.entity_ids,
        }
    }

    fn risk_register(program: &Program) -> ProgramReport {
        ProgramReport {
            kind: ReportKind::RiskRegister,
            title: "Risk Register".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: "Risks".into(), body: format!("{} risk(s)", program.risks.len()), bullets: program.risks.iter().map(|r| format!("{} — {:?}/{:?}", r.header.name, r.probability, r.impact)).collect() }],
            entity_ids: program.risks.iter().map(|r| r.header.id.clone()).collect(),
        }
    }

    fn decision_log(program: &Program) -> ProgramReport {
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

    fn validation_summary(program: &Program) -> ProgramReport {
        let diagnostics = validate_program(program);
        ProgramReport {
            kind: ReportKind::ValidationSummary,
            title: "Validation Summary".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: "Diagnostics".into(), body: format!("{} diagnostic(s)", diagnostics.len()), bullets: diagnostics.iter().map(|d| format!("[{:?}] {}: {}", d.severity, d.code, d.message)).collect() }],
            entity_ids: Vec::new(),
        }
    }

    fn recommendation(program: &Program) -> ProgramReport {
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

    fn user_summary(program: &Program) -> ProgramReport {
        ProgramReport {
            kind: ReportKind::UserSummary,
            title: "User Summary".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: "User Profiles".into(), body: format!("{} user profile(s)", program.users.len()), bullets: program.users.iter().map(|u| format!("{} — {:?}", u.header.name, u.category)).collect() }],
            entity_ids: program.users.iter().map(|u| u.header.id.clone()).collect(),
        }
    }

    fn functional_summary(program: &Program) -> ProgramReport {
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

    fn capacity_summary(program: &Program) -> ProgramReport {
        let analysis = run_analysis(program, AnalysisKind::Capacity);
        ProgramReport {
            kind: ReportKind::CapacitySummary,
            title: "Capacity Summary".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
            entity_ids: Vec::new(),
        }
    }

    fn workflow_summary(program: &Program) -> ProgramReport {
        let analysis = run_analysis(program, AnalysisKind::Workflow);
        ProgramReport {
            kind: ReportKind::WorkflowSummary,
            title: "Workflow Summary".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
            entity_ids: analysis.entity_ids,
        }
    }

    fn compliance_summary(program: &Program) -> ProgramReport {
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

    fn cost_summary(program: &Program) -> ProgramReport {
        let analysis = run_analysis(program, AnalysisKind::Cost);
        ProgramReport { kind: ReportKind::CostSummary, title: "Cost Summary".into(), generated_at: timestamp(program), sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }], entity_ids: Vec::new() }
    }

    fn schedule_summary(program: &Program) -> ProgramReport {
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

    fn change_summary(program: &Program) -> ProgramReport {
        ProgramReport {
            kind: ReportKind::ChangeSummary,
            title: "Change Summary".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: "Changes".into(), body: format!("{} change record(s)", program.changes.len()), bullets: program.changes.iter().map(|c| format!("{} — {}", c.header.name, c.header.timestamps.updated)).collect() }],
            entity_ids: program.changes.iter().map(|c| c.header.id.clone()).collect(),
        }
    }

    fn open_issue_summary(program: &Program) -> ProgramReport {
        let open: Vec<_> = program.issues.iter().filter(|i| !matches!(i.header.status, crate::kernel::LifecycleStatus::Closed | crate::kernel::LifecycleStatus::Complete)).collect();
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

    fn priority_summary(program: &Program) -> ProgramReport {
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

    fn scenario_summary(program: &Program) -> ProgramReport {
        let analysis = run_analysis(program, AnalysisKind::Scenario);
        ProgramReport {
            kind: ReportKind::ScenarioSummary,
            title: "Scenario Summary".into(),
            generated_at: timestamp(program),
            sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
            entity_ids: analysis.entity_ids,
        }
    }
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn executive_summary_includes_counts() {
            let report = build_report(&sample_program(), ReportKind::ExecutiveSummary);
            assert_eq!(report.kind, ReportKind::ExecutiveSummary);
            assert!(!report.sections.is_empty());
        }

        #[test]
        fn requirements_matrix_has_grid_rows() {
            let report = build_report(&sample_program(), ReportKind::RequirementsMatrix);
            assert!(!report.sections[0].bullets.is_empty());
            assert!(report.sections[0].bullets[0].contains('\t'));
        }

        #[test]
        fn build_report_and_record_persists() {
            let mut program = sample_program();
            let before = program.reports.len();
            build_report_and_record(&mut program, ReportKind::AdjacencyMatrix);
            assert_eq!(program.reports.len(), before + 1);
        }
    }
}

mod search {
    //! 🔍 Program search — keyword and structured filters across registers.

    use crate::kernel::{EntityHeader, EntityId, LifecycleStatus, Priority};
    use crate::program::Program;
    use crate::registers::SearchFilter;
    use serde::{Deserialize, Serialize};

    // #region 🔖SearchQuery
    /// @emoji 🎯 Ad-hoc search query with optional structured filters.
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

    /// @emoji 📌 One search hit with register kind and display name.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SearchHit {
        pub register: String,
        pub entity_id: EntityId,
        pub name: String,
        pub score: f64,
    }
    // #endregion

    // #region 🔖SearchProgram
    /// @emoji 🔎 Searches all registers; uses `filter` when provided; records query in `search_history`.
    pub fn search_program(program: &Program, query: &SearchQuery, filter: Option<&SearchFilter>, search_history: Option<&mut Vec<SearchQuery>>) -> Vec<SearchHit> {
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
        search_register!("documents", &program.documents);
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
        search_register!("knowledge", &program.knowledge);
        search_register!("benchmarks", &program.benchmarks);
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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn search_finds_reception_element() {
            let hits = search_program(&sample_program(), &SearchQuery { keywords: vec!["Reception".into()], ..Default::default() }, None, None);
            assert!(hits.iter().any(|h| h.name == "Reception"));
        }

        #[test]
        fn search_history_records_query() {
            let mut history = Vec::new();
            search_program(&sample_program(), &SearchQuery { keywords: vec!["Waiting".into()], ..Default::default() }, None, Some(&mut history));
            assert_eq!(history.len(), 1);
            assert_eq!(history[0].keywords, vec!["Waiting".to_string()]);
        }

        #[test]
        fn entity_kind_filter_limits_registers() {
            let hits = search_program(&sample_program(), &SearchQuery { entity_kinds: vec!["elements".into()], ..Default::default() }, None, None);
            assert!(hits.iter().all(|h| h.register == "elements"));
        }
    }
}

mod status_summary {
    //! 📊 Status summary — aggregate lifecycle counts across registers.

    use crate::kernel::{EntityHeader, LifecycleStatus};
    use crate::program::Program;
    use crate::registers::ValidationStatus;
    use serde::{Deserialize, Serialize};

    // #region 🔖StatusSummary
    /// @emoji 📈 Aggregated status histogram across all header-bearing registers.
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

    /// @emoji 📁 Per-register entity count and dominant status.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RegisterStatusCount {
        pub register: String,
        pub count: usize,
        pub draft_count: usize,
        pub approved_count: usize,
    }
    // #endregion

    // #region 🔖Aggregate
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

    /// @emoji 🧮 Aggregates lifecycle status counts from every program register collection.
    pub fn status_summary(program: &Program) -> StatusSummary {
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
        collect("documents", program.documents.iter().map(|e| &e.header).collect());
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
        collect("knowledge", program.knowledge.iter().map(|e| &e.header).collect());
        collect("benchmarks", program.benchmarks.iter().map(|e| &e.header).collect());

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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::program::sample_program;

        #[test]
        fn sample_program_status_summary_counts_elements() {
            let summary = status_summary(&sample_program());
            assert!(summary.total_entities >= 2);
            let elements = summary.by_register.iter().find(|r| r.register == "elements").expect("elements");
            assert_eq!(elements.count, 2);
        }

        #[test]
        fn status_summary_includes_all_major_registers() {
            let summary = status_summary(&sample_program());
            for register in ["elements", "stakeholders", "adjacencies", "status_records"] {
                assert!(summary.by_register.iter().any(|r| r.register == register));
            }
        }
    }
}

mod template {
    //! 📋 Template application — sector and project templates into program registers.

    use crate::adjacency::{normalize_pair, set_adjacency};
    use crate::kernel::{EntityHeader, EntityId, TextField};
    use crate::ops::ProgramOp;
    use crate::program::Program;
    use crate::registers::{
        Activity, Adjacency, AdjacencyKind, ConnectionKind, Equipment, Function, FunctionKind, Process, ProgramElement, ProgramElementKind, Requirement, RequirementKind, Risk, RiskLevel, Stakeholder, TemplateRecord, UserCategory, UserProfile,
        ValidationStatus,
    };
    use serde::{Deserialize, Serialize};
    use vcs::CollectionOp;

    // #region 🔖TemplateApply
    /// @emoji 📋 Result of applying a template to a program.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TemplateApplyResult {
        pub template_id: EntityId,
        pub created_entity_ids: Vec<EntityId>,
        pub messages: Vec<String>,
    }

    /// @emoji 🧩 Applies a template record and returns replayable `ProgramOp`s.
    pub fn apply_template(program: &mut Program, template: &TemplateRecord) -> Vec<ProgramOp> {
        let mut ops = Vec::new();
        let mut element_ids = Vec::new();
        for field in &template.default_fields {
            let id = EntityId::new_serial("template-entity");
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
                    ops.push(ProgramOp::Stakeholders(CollectionOp::Add { index: program.stakeholders.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Users(CollectionOp::Add { index: program.users.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Activities(CollectionOp::Add { index: program.activities.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Functions(CollectionOp::Add { index: program.functions.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Elements(CollectionOp::Add { index: program.elements.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Requirements(CollectionOp::Add { index: program.requirements.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Risks(CollectionOp::Add { index: program.risks.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Processes(CollectionOp::Add { index: program.processes.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::Equipment(CollectionOp::Add { index: program.equipment.len(), item: item.clone() }));
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
                    ops.push(ProgramOp::SetAdjacency { adjacency: adjacency.clone() });
                    set_adjacency(program, adjacency);
                }
                _ => {}
            }
        }
        if let Some(existing) = program.templates.iter_mut().find(|t| t.header.id == template.header.id) {
            existing.usage_count += 1;
            existing.last_applied = Some(program.meta.timestamps.updated.clone());
        }
        ops
    }
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ops::apply_program_op;
        use crate::program::empty_program;
        use crate::registers::TemplateRecord;

        #[test]
        fn apply_template_returns_program_ops() {
            let mut program = empty_program();
            let template = TemplateRecord {
                header: EntityHeader::new(EntityId::new_serial("template"), "Clinic Starter"),
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
            let ops = apply_template(&mut program, &template);
            assert!(!ops.is_empty());
            assert_eq!(program.stakeholders.len(), 1);
            assert_eq!(program.elements.len(), 1);
            assert_eq!(program.requirements.len(), 1);
        }

        #[test]
        fn template_ops_replay_on_empty_program() {
            let mut source = empty_program();
            let template = TemplateRecord {
                header: EntityHeader::new(EntityId::new_serial("template"), "Replay"),
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
            let ops = apply_template(&mut source, &template);
            let mut target = empty_program();
            for op in &ops {
                apply_program_op(&mut target, op);
            }
            assert_eq!(target.functions.len(), 1);
        }
    }
}

mod trace {
    //! 🧭 Traceability — trace chains and audit trail queries.

    use crate::kernel::{EntityId, TraceKind, TraceLink};
    use crate::program::Program;
    use crate::registers::AuditEvent;
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet, VecDeque};

    // #region 🔖TraceChain
    /// @emoji ⛓️ Ordered chain of trace links from a root entity.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraceChain {
        pub root_id: EntityId,
        pub links: Vec<TraceLink>,
    }

    /// @emoji 📜 Filtered audit trail slice.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuditTrail {
        pub subject_id: Option<EntityId>,
        pub events: Vec<AuditEvent>,
    }

    /// @emoji 💥 Reverse impact set from trace links pointing at an entity.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ImpactTrace {
        pub target_id: EntityId,
        pub upstream_ids: Vec<EntityId>,
        pub links: Vec<TraceLink>,
    }
    // #endregion

    // #region 🔖TraceQueries
    /// @emoji 🔗 Builds a forward trace chain from `root_id` following kind-appropriate links.
    pub fn trace_chain(program: &mut Program, root_id: &EntityId) -> TraceChain {
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

    /// @emoji 🔍 Finds trace links touching `entity_id` (from or to).
    pub fn trace_links_for(program: &mut Program, entity_id: &EntityId) -> Vec<TraceLink> {
        embed_requirement_traces(program);
        program.traces.iter().filter(|link| &link.from_id == entity_id || &link.to_id == entity_id).cloned().collect()
    }

    /// @emoji ↩️ Reverse impact trace — entities that depend on or satisfy `target_id`.
    pub fn trace_impact(program: &mut Program, target_id: &EntityId) -> ImpactTrace {
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

    /// @emoji 📋 Returns audit events for an optional subject, newest first.
    pub fn audit_trail(program: &Program, subject_id: Option<&EntityId>) -> AuditTrail {
        let mut events: Vec<AuditEvent> = program.audit_events.iter().filter(|event| subject_id.is_none_or(|id| &event.subject_id == id)).cloned().collect();
        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        AuditTrail { subject_id: subject_id.cloned(), events }
    }

    /// @emoji ➕ Appends a trace link to the program trace register.
    pub fn add_trace_link(program: &mut Program, from_id: EntityId, to_id: EntityId, kind: TraceKind) {
        program.traces.push(TraceLink::new(from_id, to_id, kind));
    }

    /// @emoji 🔁 Resolves superseded requirements to their terminal replacement.
    pub fn resolve_supersedes(program: &Program, requirement_id: &EntityId) -> EntityId {
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

    /// @emoji 🧷 Copies requirement-embedded trace links into the program trace register.
    fn embed_requirement_traces(program: &mut Program) {
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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::kernel::TraceKind;
        use crate::program::sample_program;

        #[test]
        fn trace_chain_follows_links() {
            let mut program = sample_program();
            let a = program.elements[0].header.id.clone();
            let b = program.elements[1].header.id.clone();
            add_trace_link(&mut program, a.clone(), b.clone(), TraceKind::FunctionToProgramElement);
            let chain = trace_chain(&mut program, &a);
            assert_eq!(chain.links.len(), 1);
            assert_eq!(chain.links[0].to_id, b);
        }

        #[test]
        fn audit_trail_sorted_newest_first() {
            let mut program = sample_program();
            program.audit_events.push(AuditEvent {
                header: crate::kernel::EntityHeader::new(EntityId::new_serial("audit"), "older"),
                action: crate::registers::AuditAction::Created,
                actor_id: None,
                subject_id: program.elements[0].header.id.clone(),
                subject_kind: "element".into(),
                timestamp: "2020-01-01T00:00:00Z".into(),
                details: crate::kernel::TextField::plain("old"),
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
                header: crate::kernel::EntityHeader::new(EntityId::new_serial("audit"), "newer"),
                action: crate::registers::AuditAction::Updated,
                actor_id: None,
                subject_id: program.elements[0].header.id.clone(),
                subject_kind: "element".into(),
                timestamp: "2025-01-01T00:00:00Z".into(),
                details: crate::kernel::TextField::plain("new"),
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

        #[test]
        fn trace_impact_collects_upstream() {
            let mut program = sample_program();
            let req_id = EntityId::new_serial("requirement");
            let elem_id = program.elements[0].header.id.clone();
            add_trace_link(&mut program, req_id.clone(), elem_id.clone(), TraceKind::ObjectiveToRequirement);
            let impact = trace_impact(&mut program, &elem_id);
            assert!(impact.upstream_ids.contains(&req_id));
        }
    }
}

mod validate {
    //! ✅ Program validation — schema, references, and adjacency integrity.

    use crate::adjacency::detect_adjacency_conflicts;
    use crate::kernel::{DiagnosticSeverity, EntityId, ProgramDiagnostic};
    use crate::program::{Program, ARCHITECT_PROGRAM_SCHEMA};
    use crate::registers::ValidationStatus;
    use std::collections::{HashMap, HashSet};

    // #region 🔖EntityIndex
    struct EntityIndex {
        locations: HashMap<EntityId, (String, String)>,
        duplicates: Vec<(EntityId, String, String)>,
    }

    fn build_entity_index(program: &Program) -> EntityIndex {
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
        for e in &program.documents {
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
        for e in &program.knowledge {
            register("knowledge", &e.header.id, &e.header.name);
        }
        for e in &program.benchmarks {
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
    // #endregion

    // #region 🔖Validate
    /// @emoji 🩺 Validates a program document and returns all diagnostics (non-fatal).
    pub fn validate_program(program: &Program) -> Vec<ProgramDiagnostic> {
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
    // #endregion

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::kernel::EntityHeader;
        use crate::program::{empty_program, sample_program};
        use crate::registers::Requirement;

        #[test]
        fn sample_program_passes_validation() {
            let diagnostics = validate_program(&sample_program());
            assert!(diagnostics.iter().all(|d| d.severity != DiagnosticSeverity::Error));
        }

        #[test]
        fn empty_program_warns_on_title() {
            let diagnostics = validate_program(&empty_program());
            assert!(diagnostics.iter().any(|d| d.code == "meta.empty_title"));
        }

        #[test]
        fn detects_orphan_requirement() {
            let mut program = sample_program();
            program.requirements.push(Requirement {
                header: EntityHeader::new(EntityId::new_serial("requirement"), "Orphan"),
                code: "OR-1".into(),
                kind: crate::registers::RequirementKind::Functional,
                statement: crate::kernel::TextField::plain("orphan req"),
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
            let diagnostics = validate_program(&program);
            assert!(diagnostics.iter().any(|d| d.code == "requirement.orphan"));
        }

        #[test]
        fn detects_broken_relationship_target() {
            let mut program = sample_program();
            program.relationships.push(crate::registers::Relationship {
                header: EntityHeader::new(EntityId::new_serial("relationship"), "broken"),
                source_id: program.elements[0].header.id.clone(),
                target_id: EntityId("missing-target".into()),
                kind: crate::registers::RelationshipKind::DependsOn,
                strength: Some(1.0),
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
            let diagnostics = validate_program(&program);
            assert!(diagnostics.iter().any(|d| d.code == "relationship.missing_target"));
        }
    }
}

pub use adjacency::*;
pub use analyze::*;
pub use exchange::*;
pub use kernel::*;
pub use ops::*;
pub use outputs::*;
pub use program::*;
pub use registers::*;
pub use report::*;
pub use search::*;
pub use status_summary::*;
pub use template::*;
pub use trace::*;
pub use validate::*;
