//! 📊️ Program status summary, register search and the requirement and cost analyses.

use super::*;

//#region 📊️StatusSummary
/// 📈️ Aggregated status histogram across all header-bearing registers.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    let knowledge_records = &program.knowledge_payload;
    collect("knowledge", knowledge_records.iter().map(|e| &e.header).collect());
    let benchmark_records = &program.benchmarks_payload;
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
//#region 🔍️Search
/// 🎯️ Ad-hoc search query with optional structured filters.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SearchQuery {
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub keywords: Vec<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub categories: Vec<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub owner_ids: Vec<EntityId>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub statuses: Vec<LifecycleStatus>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub priorities: Vec<Priority>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub entity_kinds: Vec<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub tag_filters: Vec<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub sources: Vec<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub date_from: Option<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub date_to: Option<String>,
}

/// 📌️ One search hit with register kind and display name.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    search_register!("knowledge", &program.knowledge_payload);
    search_register!("benchmarks", &program.benchmarks_payload);
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
//#region 🔬️Analyze
/// 📈️ Structured output from `run_analysis`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

pub(super) fn risk_score(level: &RiskLevel) -> f64 {
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
    let mandatory = program.requirements.iter().filter(|r| r.header.priority == Priority::Mandatory).count();
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

fn priority_weight(priority: &Priority) -> f64 {
    match priority {
        Priority::Mandatory => 5.0,
        Priority::Essential => 4.0,
        Priority::Preferred => 3.0,
        Priority::Optional => 2.0,
        Priority::Deferred => 1.0,
        Priority::Prohibited => 0.0,
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
