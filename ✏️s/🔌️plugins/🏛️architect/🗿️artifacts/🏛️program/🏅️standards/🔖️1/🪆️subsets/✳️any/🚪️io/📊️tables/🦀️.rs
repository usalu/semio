//! 📊️ Register table projection shared by native exchange codecs.
use crate::standards::v1::subsets::any::schema::{snapshot::ProgramSnapshot, kernel::{EntityHeader, EntityId, PluginError}};
use crate::standards::v1::subsets::any::schema::mutations::{self as leaves, ProgramMutation};
use crate::standards::v1::subsets::any::schema::registers::{RegisterKey, RegisterRow};
use crate::standards::v1::subsets::any::schema::registers::*;
use crate::standards::v1::subsets::any::schema::kernel::*;
use semio_s_artifact_stdio_csv as stdio_csv;
use semio_s_artifact_stdio_tsv as stdio_tsv;

/// 🏷️ Fixed 7-column header shared by the CSV and TSV register exchange shape.
pub(crate) const REGISTER_ROW_COLUMNS: [&str; 7] = ["register", "id", "name", "status", "priority", "tags", "source"];

/// 📊️ One CSV/TSV row representing a register entity for spreadsheet round-trip.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    pub(crate) fn columns(&self) -> [String; 7] {
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

pub(crate) fn collect_rows(program: &ProgramSnapshot) -> Vec<RegisterCsvRow> {
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
    for rel in &program.relationships {
        rows.push(RegisterCsvRow {
            register: "relationships".into(),
            id: rel.header.id.clone(),
            name: rel.header.name.clone(),
            status: format!("{:?}", rel.header.status),
            priority: format!("{:?}", rel.header.priority),
            tags: rel.header.tags.join(";"),
            source: format!("{}>{}", rel.source_id, rel.target_id),
        });
    }
    for adj in &program.adjacencies {
        rows.push(RegisterCsvRow {
            register: "adjacencies".into(),
            id: adj.header.id.clone(),
            name: adj.header.name.clone(),
            status: format!("{:?}", adj.header.status),
            priority: format!("{:?}", adj.header.priority),
            tags: adj.header.tags.join(";"),
            source: format!("{}>{}", adj.element_a_id, adj.element_b_id),
        });
    }
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
    push_rows!("knowledge", &program.knowledge_payload);
    push_rows!("benchmarks", &program.benchmarks_payload);
    rows
}

fn header_row(register: &str, header: &EntityHeader, source: Option<String>) -> RegisterCsvRow {
    RegisterCsvRow { register: register.into(), id: header.id.clone(), name: header.name.clone(), status: format!("{:?}", header.status), priority: format!("{:?}", header.priority), tags: header.tags.join(";"), source: source.unwrap_or_default() }
}

//#region 📤️ExchangeImport
/// 🔀️ Strategy for merging imported register rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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

/// 📥️ The outcome of a register import: the mutations that perform it, the program they produce when folded through the central
/// applier, and the ids the import touched.
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterImport {
    pub snapshot: ProgramSnapshot,
    pub mutations: Vec<ProgramMutation>,
    pub touched: Vec<EntityId>,
}

/// 📥️ Decodes CSV via stdio's real RFC 4180 codec, then merges rows into matching
/// register collections via `MergeStrategy`.
pub fn import_registers_csv(program: &ProgramSnapshot, csv: &str, strategy: MergeStrategy) -> Result<RegisterImport, PluginError> {
    let snapshot = stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv_with(csv, true);
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
pub fn import_registers_tsv(program: &ProgramSnapshot, tsv: &str, strategy: MergeStrategy) -> Result<RegisterImport, PluginError> {
    let snapshot = stdio_tsv::standards::iana::subsets::any::io::text::snapshot::decode_tsv(tsv);
    import_rows(program, tsv_snapshot_to_rows(&snapshot)?, strategy)
}

/// 🧬️ One import run: the working program every row decision reads, and the mutations emitted so far. A mutation enters the run only
/// through [`ImportRun::emit`], which folds its diff into the working program via the central applier.
struct ImportRun {
    work: ProgramSnapshot,
    mutations: Vec<ProgramMutation>,
}

impl ImportRun {
    fn emit(&mut self, mutation: ProgramMutation) {
        use protocol::Mutation;
        if let Ok(next) = protocol::apply_diff(mutation.diff(&self.work).diff(), &self.work) {
            self.work = next;
            self.mutations.push(mutation);
        }
    }
}

/// 🔀️ Applies `MergeStrategy` upsert semantics to already-decoded rows — shared by the
/// CSV and TSV import paths, the decode step itself lives entirely in stdio's real codecs.
fn import_rows(program: &ProgramSnapshot, rows: Vec<RegisterCsvRow>, strategy: MergeStrategy) -> Result<RegisterImport, PluginError> {
    let mut run = ImportRun { work: program.clone(), mutations: Vec::new() };
    let mut touched = Vec::new();
    let mut seen: HashSet<(String, EntityId)> = HashSet::new();
    for row in rows {
        let key = (row.register.clone(), row.id.clone());
        if !seen.insert(key.clone()) {
            return Err(PluginError::Csv(format!("duplicate import id {} in register {}", row.id, row.register)));
        }
        if strategy == MergeStrategy::SkipDuplicates && register_contains(&run.work, &row.register, &row.id) {
            continue;
        }
        if strategy == MergeStrategy::Replace {
            remove_register_item(&mut run, &row.register, &row.id);
        }
        upsert_register_row(&mut run, row.clone())?;
        touched.push(row.id);
    }
    Ok(RegisterImport { snapshot: run.work, mutations: run.mutations, touched })
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

fn remove_register_item(run: &mut ImportRun, register: &str, id: &EntityId) {
    if !register_contains(&run.work, register, id) {
        return;
    }
    let id = id.clone();
    match register {
        "elements" => run.emit(ProgramMutation::DeleteRegisterRow(leaves::delete_register_row::DeleteRegisterRow { register: RegisterKey::ProgramElement, id })),
        "stakeholders" => run.emit(ProgramMutation::DeleteRegisterRow(leaves::delete_register_row::DeleteRegisterRow { register: RegisterKey::Stakeholder, id })),
        "requirements" => run.emit(ProgramMutation::DeleteRegisterRow(leaves::delete_register_row::DeleteRegisterRow { register: RegisterKey::Requirement, id })),
        "relationships" => run.emit(ProgramMutation::DeleteRegisterRow(leaves::delete_register_row::DeleteRegisterRow { register: RegisterKey::Relationship, id })),
        "adjacencies" => run.emit(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id })),
        _ => {}
    }
}

fn upsert_register_row(run: &mut ImportRun, row: RegisterCsvRow) -> Result<(), PluginError> {
    match row.register.as_str() {
        "elements" => upsert_element(run, row),
        "stakeholders" => upsert_stakeholder(run, row),
        "requirements" => upsert_requirement(run, row),
        "relationships" => upsert_relationship(run, row),
        "adjacencies" => upsert_adjacency(run, row),
        "knowledge" => upsert_knowledge(run, row),
        "benchmarks" => upsert_benchmark(run, row),
        other => {
            return Err(PluginError::Csv(format!("unsupported register import: {other}")));
        }
    }
    Ok(())
}

fn upsert_element(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(element) = run.work.elements.iter().find(|e| e.header.id == row.id) {
        if element.header.name != row.name {
            run.emit(ProgramMutation::RenameRegisterRow(leaves::rename_register_row::RenameRegisterRow { register: RegisterKey::ProgramElement, id: row.id, new_name: row.name }));
        }
        return;
    }
    let program_element = ProgramElement {
        header: EntityHeader::new(row.id, row.name),
        code: String::new(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: None,
        area: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
        volume: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
        height: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
        occupancy: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
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
    run.emit(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::ProgramElement(program_element), index: None }));
}

fn upsert_stakeholder(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(stakeholder) = run.work.stakeholders.iter().find(|s| s.header.id == row.id) {
        if stakeholder.header.name != row.name {
            run.emit(ProgramMutation::RenameRegisterRow(leaves::rename_register_row::RenameRegisterRow { register: RegisterKey::Stakeholder, id: row.id, new_name: row.name }));
        }
        return;
    }
    let stakeholder = Stakeholder {
        header: EntityHeader::new(row.id, row.name),
        role: String::new(),
        organization: String::new(),
        department: None,
        contact_email: None,
        contact_phone: None,
        influence: crate::standards::v1::subsets::any::schema::registers::InfluenceLevel::Medium,
        interest: crate::standards::v1::subsets::any::schema::registers::InfluenceLevel::Medium,
        engagement: crate::standards::v1::subsets::any::schema::registers::EngagementLevel::Neutral,
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
    };
    run.emit(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(stakeholder), index: None }));
}

fn upsert_requirement(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(existing) = run.work.requirements.iter().find(|r| r.header.id == row.id) {
        let source = (!row.source.is_empty()).then(|| row.source.clone());
        if existing.header.name != row.name || source.as_ref().is_some_and(|source| existing.source.as_ref() != Some(source)) {
            let mut requirement = existing.clone();
            requirement.header.name = row.name;
            if source.is_some() {
                requirement.source = source;
            }
            run.emit(ProgramMutation::ReplaceRegisterRow(leaves::replace_register_row::ReplaceRegisterRow { row: RegisterRow::Requirement(requirement) }));
        }
        return;
    }
    let requirement = Requirement {
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
    };
    run.emit(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Requirement(requirement), index: None }));
}

/// 🔗 Parses `source_id>target_id` from the CSV `source` column (export fidelity encoding).
fn parse_endpoint_pair(encoded: &str, program: &ProgramSnapshot) -> (EntityId, EntityId) {
    if let Some((left, right)) = encoded.split_once('>') {
        let source = EntityId(left.trim().to_string());
        let target = EntityId(right.trim().to_string());
        if !source.0.is_empty() && !target.0.is_empty() {
            return (source, target);
        }
    }
    let fallback = program.elements.first().map_or_else(|| EntityId::new_serial("element", "element"), |e| e.header.id.clone());
    (fallback.clone(), program.elements.get(1).map_or_else(|| fallback.clone(), |e| e.header.id.clone()))
}

fn upsert_relationship(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(existing) = run.work.relationships.iter().find(|r| r.header.id == row.id) {
        if existing.header.name != row.name {
            run.emit(ProgramMutation::RenameRegisterRow(leaves::rename_register_row::RenameRegisterRow { register: RegisterKey::Relationship, id: row.id, new_name: row.name }));
        }
        return;
    }
    let (source_id, target_id) = parse_endpoint_pair(&row.source, &run.work);
    let relationship = Relationship {
        header: EntityHeader::new(row.id, row.name),
        source_id,
        target_id,
        kind: RelationshipKind::AdjacentTo,
        strength: None,
        directional: true,
        rationale: None,
        constraints: Vec::new(),
        conditions: Vec::new(),
        relationship_priority: crate::standards::v1::subsets::any::schema::kernel::Priority::Preferred,
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
    };
    run.emit(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Relationship(relationship), index: None }));
}

fn upsert_adjacency(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(existing) = run.work.adjacencies.iter().find(|a| a.header.id == row.id) {
        if existing.header.name != row.name {
            let mut adjacency = existing.clone();
            adjacency.header.name = row.name;
            run.emit(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency, index: None }));
        }
        return;
    }
    let (a, b) = parse_endpoint_pair(&row.source, &run.work);
    let (left, right) = normalize_pair(&a, &b);
    let adjacency = Adjacency {
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
    };
    run.emit(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency, index: None }));
}

/// 📚️ Upserts a COMPOSED `knowledge` register row through create/rename mutations.
fn upsert_knowledge(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(existing) = run.work.knowledge_payload.iter().find(|record| record.header.id == row.id) {
        if existing.header.name != row.name {
            run.emit(ProgramMutation::RenameRegisterRow(leaves::rename_register_row::RenameRegisterRow { register: RegisterKey::KnowledgeRecord, id: row.id, new_name: row.name }));
        }
        return;
    }
    let knowledge_record = crate::standards::v1::subsets::any::schema::registers::KnowledgeRecord {
        header: EntityHeader::new(row.id, row.name),
        topic: String::new(),
        category: String::new(),
        summary: TextField::plain(""),
        content: TextField::plain(""),
        sources: Vec::new(),
        references: Vec::new(),
        lessons_learned: Vec::new(),
        best_practices: Vec::new(),
        applicable_sectors: Vec::new(),
        related_entity_kinds: Vec::new(),
        author_ids: Vec::new(),
        expertise_level: None,
        validation_status: ValidationStatus::Pending,
        last_reviewed: None,
        keywords: Vec::new(),
        attachments: Vec::new(),
        citations: Vec::new(),
        usage_count: 0,
    };
    run.emit(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::KnowledgeRecord(knowledge_record), index: None }));
}

/// 🏁️ Upserts a COMPOSED `benchmarks` register row through create/rename mutations.
fn upsert_benchmark(run: &mut ImportRun, row: RegisterCsvRow) {
    if let Some(existing) = run.work.benchmarks_payload.iter().find(|record| record.header.id == row.id) {
        if existing.header.name != row.name {
            run.emit(ProgramMutation::RenameRegisterRow(leaves::rename_register_row::RenameRegisterRow { register: RegisterKey::BenchmarkRecord, id: row.id, new_name: row.name }));
        }
        return;
    }
    let benchmark_record = crate::standards::v1::subsets::any::schema::registers::BenchmarkRecord {
        header: EntityHeader::new(row.id, row.name),
        benchmark_name: String::new(),
        sector: String::new(),
        metric: String::new(),
        value: 0.0,
        unit: String::new(),
        sample_size: None,
        source: None,
        collection_year: None,
        geography: None,
        building_type: None,
        confidence: None,
        methodology: None,
        applicable_element_kinds: Vec::new(),
        related_requirement_ids: Vec::new(),
        comparison_notes: Vec::new(),
        limitations: Vec::new(),
        license: None,
        knowledge_id: None,
        last_verified: None,
    };
    run.emit(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::BenchmarkRecord(benchmark_record), index: None }));
}
//#endregion 📤️ExchangeImport
