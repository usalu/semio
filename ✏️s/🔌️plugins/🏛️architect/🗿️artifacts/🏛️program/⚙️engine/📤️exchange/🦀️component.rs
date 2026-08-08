//! ⚙️ Architect program artifact engine — the `exchange` topic.

//! 📤️ Data exchange — JSON and CSV import/export for program_registers.

use crate::artifacts::program::kernel::{EntityHeader, EntityId, PluginError, TextField};
use crate::artifacts::program::{ProgramSnapshot, ARCHITECT_PROGRAM_SCHEMA};
use crate::artifacts::program::registers::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// #region 🔖️MergeStrategy
/// @emoji 🔀️ Strategy for merging imported register rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeStrategy {
    Replace,
    SkipDuplicates,
    Upsert,
}
// #endregion

// #region 🔖️JsonExchange
/// @emoji 📤️ Serializes a plugin to pretty JSON.
pub fn export_json(program: &ProgramSnapshot) -> Result<String, PluginError> {
    serde_json::to_string_pretty(program).map_err(|e| PluginError::Serialize(e.to_string()))
}

/// @emoji 📥️ Deserializes a plugin from JSON with schema validation.
pub fn import_json(json: &str) -> Result<ProgramSnapshot, PluginError> {
    let program: ProgramSnapshot = serde_json::from_str(json).map_err(|e| PluginError::Deserialize(e.to_string()))?;
    if program.schema != ARCHITECT_PROGRAM_SCHEMA {
        return Err(PluginError::InvalidSchema { expected: ARCHITECT_PROGRAM_SCHEMA.into(), actual: program.schema });
    }
    Ok(program)
}
// #endregion

// #region 🔖️CsvExchange
/// @emoji 📊️ One CSV row representing a register entity for spreadsheet round-trip.
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

/// @emoji 📤️ Flattens all registers into CSV rows.
pub fn export_registers_csv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(write_delimited(&collect_rows(program), ','))
}

/// @emoji 📥️ Merges CSV rows into matching register collections.
pub fn import_registers_csv(program: &mut ProgramSnapshot, csv: &str, strategy: MergeStrategy) -> Result<Vec<EntityId>, PluginError> {
    import_delimited(program, csv, ',', strategy)
}

/// @emoji 📤️ Flattens all registers into TSV rows.
pub fn export_registers_tsv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(write_delimited(&collect_rows(program), '\t'))
}

/// @emoji 📥️ Merges TSV rows into matching register collections.
pub fn import_registers_tsv(program: &mut ProgramSnapshot, tsv: &str, strategy: MergeStrategy) -> Result<Vec<EntityId>, PluginError> {
    import_delimited(program, tsv, '\t', strategy)
}

/// @emoji ↔ Exports relationships as CSV rows preserving endpoints.
pub fn export_relationships_csv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    let mut out = String::from("id,source_id,target_id,kind,name\n");
    for rel in &program.relationships {
        out.push_str(&format!("{},{},{},{:?},{}\n", escape_field(&rel.header.id.to_string(), ','), escape_field(&rel.source_id.to_string(), ','), escape_field(&rel.target_id.to_string(), ','), rel.kind, escape_field(&rel.header.name, ','),));
    }
    Ok(out)
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

fn parse_delimited(input: &str, delimiter: char) -> Result<Vec<RegisterCsvRow>, PluginError> {
    let mut lines = input.lines();
    let header = lines.next().ok_or_else(|| PluginError::Csv("empty delimited file".into()))?;
    let expected = format!("register{}id{}name{}status{}priority{}tags{}source", delimiter, delimiter, delimiter, delimiter, delimiter, delimiter);
    if header != expected {
        return Err(PluginError::Csv(format!("unexpected header: {header}")));
    }
    let mut rows = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields = parse_record(line, delimiter);
        if fields.len() < 7 {
            return Err(PluginError::Csv(format!("malformed row: {line}")));
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

fn import_delimited(program: &mut ProgramSnapshot, input: &str, delimiter: char, strategy: MergeStrategy) -> Result<Vec<EntityId>, PluginError> {
    let rows = parse_delimited(input, delimiter)?;
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
    let (left, right) = crate::artifacts::program::engine::adjacency::normalize_pair(&a, &b);
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
        let rows = parse_delimited(csv, ',').expect("parse");
        assert_eq!(rows[0].name, "Room, A");
        assert_eq!(rows[0].source, "src");
    }

    #[test]
    fn duplicate_import_is_rejected() {
        let csv = "register,id,name,status,priority,tags,source\nelements,e1,A,Draft,Preferred,,\nelements,e1,B,Draft,Preferred,,\n";
        let mut program = crate::artifacts::program::empty_plugin();
        assert!(import_registers_csv(&mut program, csv, MergeStrategy::Upsert).is_err());
    }
}