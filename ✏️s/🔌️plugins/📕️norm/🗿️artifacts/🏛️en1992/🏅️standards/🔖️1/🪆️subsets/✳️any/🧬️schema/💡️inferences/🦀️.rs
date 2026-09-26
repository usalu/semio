//! 💡️ En1992 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1992Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1992 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992.inference")]
pub struct En1992Inference {
    #[derived]
    pub outline: En1992Outline,
}

impl protocol::Inference<En1992Snapshot> for En1992Inference {
    fn infer(snapshot: &En1992Snapshot) -> Self {
        Self { outline: En1992Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1992Snapshot> for En1992Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1992.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1992.inference.outline", reads: &["annex","members","anchors","concreteGrades","reinforcementGrades","deltaCDev"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1992Builder {
    type Snapshot = En1992Snapshot;
    type Inference = En1992Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1992.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1992_artifact_schema_descriptor`'s registration.
pub fn en1992_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1992.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::document::{CheckReport, CheckResult};
use crate::standards::v1::subsets::any::schema::{evaluate_anchor, evaluate_member};

/// 📋️ `En1992Snapshot -> CheckReport` — full hierarchical structure assessment.
pub fn evaluate(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    push_duplicate_ids(&mut report, document, "members", &document.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("members[id={id}].id"));
    push_duplicate_ids(&mut report, document, "anchors", &document.anchors.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("anchors[id={id}].id"));
    push_duplicate_ids(&mut report, document, "concreteGrades", &document.concrete_grades.iter().map(|g| g.id.clone()).collect::<Vec<_>>(), &|id| format!("concreteGrades[id={id}].id"));
    push_duplicate_ids(&mut report, document, "reinforcementGrades", &document.reinforcement_grades.iter().map(|g| g.id.clone()).collect::<Vec<_>>(), &|id| format!("reinforcementGrades[id={id}].id"));
    push_duplicate_ids(&mut report, document, "prestressSteels", &document.prestress_steels.iter().map(|g| g.id.clone()).collect::<Vec<_>>(), &|id| format!("prestressSteels[id={id}].id"));
    if document.members.is_empty() && document.anchors.is_empty() {
        return report;
    }
    for member in &document.members {
        report.extend(evaluate_member(document, member));
    }
    for anchor in &document.anchors {
        report.extend(evaluate_anchor(document, anchor));
    }
    report
}

fn push_duplicate_ids(report: &mut CheckReport, document: &En1992Snapshot, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    use crate::document::{CheckStatus, ClauseId, LocalizedCopy, Remedy, SubjectRef};
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, LocalizedCopy::new(format!("Duplicate {table} id"), format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = CheckResult::assess(
            format!("en1992.integrity.duplicate.{table}.{id}"),
            "EN 1992 integrity",
            ClauseId::new("EN 1992-1-1", "§1", "id"),
            subject.clone(),
            LocalizedCopy::new(format!("Unique {table} id"), format!("Eindeutige {table}-Id")),
        )
        .annex(document.annex)
        .explanation(LocalizedCopy::new(
            format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            LocalizedCopy::new(
                format!("Rename the duplicated '{id}' entry to a free id."),
                format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1992Outline;
//#endregion 🔁️Re-exports
