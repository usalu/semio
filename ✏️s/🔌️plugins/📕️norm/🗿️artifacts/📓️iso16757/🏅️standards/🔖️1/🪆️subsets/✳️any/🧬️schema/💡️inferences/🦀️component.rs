//! 💡️ Iso16757 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::iso16757::Iso16757Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Iso16757Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a iso16757 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757.inference")]
pub struct Iso16757Inference {
    #[state(inferred)]
    pub outline: Iso16757Outline,
}

impl protocol::Inference<Iso16757Snapshot> for Iso16757Inference {
    fn infer(snapshot: &Iso16757Snapshot) -> Self {
        Self { outline: Iso16757Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Iso16757Snapshot> for Iso16757Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.iso16757.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.iso16757.inference.outline", reads: &["part_number_inputs"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::iso16757::standards::v1::subsets::any::schema::Iso16757Builder {
    type Snapshot = Iso16757Snapshot;
    type Inference = Iso16757Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.iso16757.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `iso16757_artifact_schema_descriptor`'s registration.
pub fn iso16757_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.iso16757.inference",
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

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = Iso16757Snapshot::default();
        assert_eq!(Iso16757Inference::infer(&snapshot), Iso16757Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Iso16757Inference::infer(&Iso16757Snapshot::default()), Iso16757Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
/// 📋️ Full ISO 16757 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `Iso16757Snapshot -> CheckReport` projection; everything it
/// composes is a pure helper living in the parent `🧬️schema`.
use crate::artifacts::iso16757::CatalogueValue;
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity, QuantityKind};
use crate::artifacts::iso16757::standards::v1::subsets::any::schema::{part_1, part_2, part_4, part_5};
use std::collections::{HashMap, HashSet};

fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new("ISO 16757", part, section)
}

fn check_count(report: &mut CheckReport, clause: ClauseId, actual: f64, expected: f64, message: impl Into<String>) {
    report.push(CheckResult::from_utilization(clause, Quantity::new(QuantityKind::Dimensionless, actual), Quantity::new(QuantityKind::Dimensionless, expected), message, AnnexChoice::En));
}

pub fn evaluate(document: &Iso16757Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = AnnexChoice::En;

    let structure_issues = part_1::validate_catalogue_structure(&document.catalogue);
    check_count(&mut report, clause("1", "3.1"), if structure_issues.is_empty() { 1.0 } else { 2.0 }, 1.0, "catalogue structure");
    for issue in &structure_issues {
        report.push(CheckResult::fail(clause("1", "3.1"), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, issue.clone(), annex));
    }

    let selection = part_1::select_products(&document.catalogue, &document.selection);
    let expected_matches: f64 = if document.selection.class_id == "class.valve" { 1.0 } else { 0.0 };
    check_count(&mut report, clause("1", "4.2"), selection.matches.len() as f64, expected_matches.max(1.0), "product selection");
    if selection.ambiguity {
        report.push(CheckResult::fail(clause("1", "4.2"), Quantity::new(QuantityKind::Dimensionless, selection.matches.len() as f64), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, String::from("ambiguous selection"), annex));
    }

    if let Ok(embedding) = part_1::resolve_bim_embedding(&document.catalogue, "index.cv50", HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })])) {
        let has_geometry = embedding.resolved_geometry_id.is_some();
        report.push(if has_geometry {
            CheckResult::pass(clause("1", "10"), Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, "BIM embedding resolved geometry", annex)
        } else {
            CheckResult::fail(clause("1", "10"), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 0.0, "missing geometry in BIM embedding", annex)
        });
    }

    if let Some(geom) = document.geometry.objects.get("geom.valve.50") {
        if let Some(shape) = &geom.shape {
            match part_2::evaluate_bounding_box(shape, &document.geometry) {
                Ok(bbox) => {
                    let volume = bbox.volume_m3();
                    report.push(CheckResult::from_utilization(clause("2", "7.1"), Quantity::new(QuantityKind::Volume, volume), Quantity::new(QuantityKind::Volume, 0.003), format!("primitive bbox volume {volume:.4} m3"), annex));
                    let step = part_2::project_step_entity(geom, bbox);
                    if step.contains("IFCBOUNDINGBOX") {
                        report.push(CheckResult::pass(clause("2", "7.4"), Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, "STEP/IFC geometry projection", annex));
                    }
                }
                Err(err) => {
                    report.push(CheckResult::fail(clause("2", "6.1"), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, err.to_string(), annex));
                }
            }
        }
        let mut visited = HashSet::new();
        let geom_issues = part_2::validate_geometry_graph(geom, &document.geometry, &mut visited);
        if !geom_issues.is_empty() {
            for issue in geom_issues {
                report.push(CheckResult::fail(clause("2", "6.1"), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, issue, annex));
            }
        }
        if let Some(install_space) = geom.spaces.iter().find(|s| s.kind == crate::artifacts::iso16757::part_2::SpaceKind::Installation) {
            let product_bbox = crate::artifacts::iso16757::part_2::BoundingBox::from_size(0.15, 0.20, 0.10);
            let clearance_ok = !product_bbox.overlaps(install_space.bounds, 0.05);
            report.push(if clearance_ok {
                CheckResult::pass(clause("2", "5.3.5"), Quantity::new(QuantityKind::Length, 0.05), Quantity::new(QuantityKind::Length, 0.05), 1.0, "installation clearance", annex)
            } else {
                CheckResult::fail(clause("2", "5.3.5"), Quantity::new(QuantityKind::Length, 0.0), Quantity::new(QuantityKind::Length, 0.05), 0.0, "insufficient installation clearance", annex)
            });
        }
    }

    let dict_issues = part_4::validate_dictionary(&document.dictionary);
    check_count(&mut report, clause("4", "4.3"), if dict_issues.is_empty() { 1.0 } else { 2.0 }, 1.0, "dictionary structure");
    let allowed = part_4::filter_controlled_values(document.dictionary.controlled_lists.first().expect("fixture list"), "subject.valve", &document.dictionary);
    if allowed.contains(&"50".to_string()) {
        report.push(CheckResult::pass(clause("4", "6.3.2"), Quantity::new(QuantityKind::Dimensionless, 50.0), Quantity::new(QuantityKind::Dimensionless, 50.0), 1.0, "context-filtered controlled value", annex));
    }
    let mappings = part_4::to_iso12006_mappings(&document.dictionary);
    if !mappings.is_empty() {
        report.push(CheckResult::pass(clause("4", "5.1"), Quantity::new(QuantityKind::Dimensionless, mappings.len() as f64), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, "ISO 12006-3 mapping", annex));
    }

    let ifc = part_5::build_ifc_catalogue(&document.catalogue);
    let exchange_issues = part_5::validate_exchange(&document.catalogue, &ifc);
    check_count(&mut report, clause("5", "6.1"), if exchange_issues.is_empty() { 1.0 } else { 2.0 }, 1.0, "IFC catalogue structure");
    let step = part_5::export_ifc_step(&ifc);
    if step.contains("IFCPRODUCT") || step.contains("IfcProduct") {
        report.push(CheckResult::pass(clause("5", "6.1"), Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, "IFC STEP export", annex));
    }

    let runtime = part_5::DefaultScriptRuntime;
    use part_5::ScriptRuntime;
    match part_5::calculate_part_number(&document.part_number_rule, &document.part_number_inputs, &runtime) {
        Ok(part_no) => {
            let expected = 550.0;
            let actual: f64 = part_no.parse().unwrap_or(0.0);
            report.push(CheckResult::from_utilization(clause("5", "6.10"), Quantity::new(QuantityKind::Dimensionless, actual), Quantity::new(QuantityKind::Dimensionless, expected), format!("part number script result {part_no}"), annex));
        }
        Err(err) => {
            report.push(CheckResult::fail(clause("5", "6.10"), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, err.to_string(), annex));
        }
    }

    match runtime.execute("1/(0)", &HashMap::new(), document.script_limits) {
        Err(crate::artifacts::iso16757::part_5::ScriptError::InvalidExpression(_)) => {
            report.push(CheckResult::pass(clause("5", "8"), Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, "script division-by-zero guard", annex));
        }
        _ => {
            report.push(CheckResult::fail(clause("5", "8"), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, "script should reject division by zero", annex));
        }
    }

    report
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;
    use crate::document::CheckStatus;

    #[test]
    fn evaluate_exercises_all_parts_with_numeric_checks() {
        let report = evaluate(&Iso16757Snapshot::default());
        assert!(!report.checks.is_empty());
        let clauses: HashSet<String> = report.checks.iter().map(|c| format!("{} {}", c.clause.part, c.clause.section)).collect();
        assert!(clauses.iter().any(|c| c.starts_with("1 ")));
        assert!(clauses.iter().any(|c| c.starts_with("2 ")));
        assert!(clauses.iter().any(|c| c.starts_with("4 ")));
        assert!(clauses.iter().any(|c| c.starts_with("5 ")));
        let part_number_check = report.checks.iter().find(|c| c.clause.section == "6.10").expect("part number check");
        assert_eq!(part_number_check.status, CheckStatus::Pass);
        assert!((part_number_check.computed.value - 550.0).abs() < 1e-6);
    }
}
//#endregion 🧪️ComplianceReportTests

