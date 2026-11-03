//! 💡️ En1992 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1992 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992.inference")]
pub struct En1992Inference {
    #[derived]
    pub outline: En1992Outline,
}

impl protocol::Inference<En1992Snapshot> for En1992Inference {
    fn infer(snapshot: &En1992Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: En1992Outline::compute(snapshot) }
    
        })
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
        &[protocol::InferenceFieldSpec { id: "s.norm.en1992.inference.outline", reads: &["annex","reliabilityClass","members","anchors","placements","concreteGrades","reinforcementGrades","deltaCDev"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1992.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1992_artifact_schema_descriptor`'s registration.
pub fn en1992_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.en1992.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult};
use semio_s_artifact_norm_en1992_checks::{evaluate_anchor, evaluate_material_cases, evaluate_member};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;

/// 📋️ `En1992Snapshot -> CheckReport` — full hierarchical structure assessment.
pub fn evaluate(document: &En1992Snapshot) -> CheckReport {
    evaluate_scoped(document, Scope::All)
}

/// 🎯️ The evaluation restricted to `scope` (global integrity, or one member or anchor row); the ordered concatenation of all scopes is [`evaluate`].
pub fn evaluate_scoped(document: &En1992Snapshot, scope: Scope<'_>) -> CheckReport {
    let mut report = CheckReport::default();
    if scope.globals() {
        push_duplicate_ids(&mut report, document, "members", &document.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("members[id={id}].id"));
        push_duplicate_ids(&mut report, document, "anchors", &document.anchors.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("anchors[id={id}].id"));
        push_duplicate_ids(&mut report, document, "placements", &document.placements.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));
        push_duplicate_ids(&mut report, document, "cases", &document.cases.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={id}].id"));
        push_duplicate_ids(&mut report, document, "concreteGrades", &document.concrete_grades.iter().map(|g| g.id.clone()).collect::<Vec<_>>(), &|id| format!("concreteGrades[id={id}].id"));
        push_duplicate_ids(&mut report, document, "reinforcementGrades", &document.reinforcement_grades.iter().map(|g| g.id.clone()).collect::<Vec<_>>(), &|id| format!("reinforcementGrades[id={id}].id"));
        push_duplicate_ids(&mut report, document, "prestressSteels", &document.prestress_steels.iter().map(|g| g.id.clone()).collect::<Vec<_>>(), &|id| format!("prestressSteels[id={id}].id"));
        if document.annex == semio_s_plugin_norm_compliance::AnnexChoice::De {
            report.push(na_edition_check(document));
        }
        report.extend(evaluate_material_cases(document));
    }
    for (index, member) in document.members.iter().enumerate().filter(|(index, _)| scope.admits("members", *index)) {
        report.extend(evaluate_member(document, member));
    }
    for (index, anchor) in document.anchors.iter().enumerate().filter(|(index, _)| scope.admits("anchors", *index)) {
        report.extend(evaluate_anchor(document, anchor));
    }
    for (index, case) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(semio_s_artifact_norm_en1992_checks::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), document.annex));
    }
    report
}

/// 🇩🇪 The national annex edition the DE values rest on (`ndp.en1992.na-de-edition`, unverified), so every DE report states its basis; passes as `UnverifiedBasis` until a human verifies the edition.
fn na_edition_check(document: &En1992Snapshot) -> semio_s_plugin_norm_compliance::CheckResult {
    use semio_s_plugin_norm_compliance::{ndp_number, AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};
    let edition = ndp_number("ndp.en1992.na-de-edition", AnnexChoice::De);
    CheckResult::assess("en1992.annex.na-de-edition", "EN 1992 national annex", ClauseId::new("EN 1992-1-1", "§NA", "NA"), SubjectRef::new("annex".to_string(), "annex".to_string(), LocalizedCopy::new("National annex", "Nationaler Anhang")), LocalizedCopy::new("National annex edition", "Ausgabe des Nationalen Anhangs"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Pass)
        .annex(document.annex)
        .explanation(LocalizedCopy::new(
            format!("The DE values follow DIN EN 1992-1-1/NA:{edition:.0}; the edition and its amendments are not verified."),
            format!("Die DE-Werte folgen DIN EN 1992-1-1/NA:{edition:.0}; Ausgabe und Änderungen sind nicht geprüft."),
        ))
        .ndps(&["ndp.en1992.na-de-edition"])
        .build()
}

fn push_duplicate_ids(report: &mut CheckReport, document: &En1992Snapshot, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    use semio_s_plugin_norm_compliance::{CheckStatus, ClauseId, LocalizedCopy, Remedy, SubjectRef};
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
pub use crate::outline::En1992Outline;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts

#[path = "🧮️units/🦀️.rs"]
mod units;
pub use units::*;

//#region 🛰️LiveEvaluation
semio_s_plugin_norm_compliance::live_report!(pub mod live(En1992Snapshot) = En1992Units);
//#endregion 🛰️LiveEvaluation
