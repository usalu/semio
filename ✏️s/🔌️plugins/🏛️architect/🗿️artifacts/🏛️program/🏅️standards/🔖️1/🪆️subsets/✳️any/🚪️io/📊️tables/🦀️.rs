//! 📊️ Register table projection shared by native exchange codecs.
use crate::{ProgramSnapshot,kernel::{EntityHeader,EntityId,PluginError}};

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
