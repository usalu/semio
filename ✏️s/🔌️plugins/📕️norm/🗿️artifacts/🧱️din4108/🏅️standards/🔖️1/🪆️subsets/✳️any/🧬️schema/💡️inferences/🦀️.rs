//! 💡️ Din4108 inference schema — outline + compliance evaluate.

use crate::Din4108Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din4108 snapshot.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108.inference")]
pub struct Din4108Inference {
    #[derived]
    pub outline: Din4108Outline,
}

impl protocol::Inference<Din4108Snapshot> for Din4108Inference {
    fn infer(snapshot: &Din4108Snapshot) -> Self {
        Self { outline: Din4108Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Din4108Snapshot> for Din4108Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.din4108.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.din4108.inference.outline",
            reads: &[
                "climateZone",
                "usage",
                "tIntC",
                "rhInt",
                "hasMechanicalVentilation",
                "airtightnessN50",
                "bb2DetailsConform",
                "zones",
                "elements",
                "thermalBridges",
            ],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Din4108Builder {
    type Snapshot = Din4108Snapshot;
    type Inference = Din4108Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din4108.inference` facet leaves.
pub fn din4108_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.din4108.inference",
        inference: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::document::CheckReport;
use crate::standards::v1::subsets::any::schema::{bb_2, opaque_envelope_area, part_2, part_3, part_4, part_6, part_7, part_10};


fn loc(en: impl Into<String>, de: impl Into<String>) -> crate::document::LocalizedCopy {
    crate::document::LocalizedCopy::new(en, de)
}

fn push_duplicate_ids(report: &mut CheckReport, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = crate::document::SubjectRef::new(id.clone(), &path, loc(format!("Duplicate {table} id"), format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = crate::document::CheckResult::assess(
            format!("din4108.integrity.duplicate.{table}.{id}"),
            "DIN 4108 integrity",
            crate::document::ClauseId::new("DIN 4108", "§1", "id"),
            subject.clone(),
            loc(format!("Unique {table} id"), format!("Eindeutige {table}-Id")),
        )
        .annex(crate::document::AnnexChoice::De)
        .explanation(loc(
            format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(crate::document::CheckStatus::Fail);
        builder = builder.remedy(crate::document::Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            loc(
                format!("Rename the duplicated '{id}' entry to a free id."),
                format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

fn push_dangling_ref(report: &mut CheckReport, check_id: String, path: String, subject_id: &str, label_en: &str, label_de: &str, current: &str, options: Vec<String>) {
    let subject = crate::document::SubjectRef::new(subject_id, &path, loc(label_en, label_de));
    let opts = if options.is_empty() { vec![format!("{current}-fix")] } else { options };
    report.push(
        crate::document::CheckResult::assess(
            check_id,
            "DIN 4108 integrity",
            crate::document::ClauseId::new("DIN 4108", "§1", "reference"),
            subject.clone(),
            loc(format!("Referential integrity — {label_en}"), format!("Referenzintegrität — {label_de}")),
        )
        .annex(crate::document::AnnexChoice::De)
        .explanation(loc(
            format!("'{current}' does not reference a valid target."),
            format!("'{current}' verweist auf kein gültiges Ziel."),
        ))
        .status(crate::document::CheckStatus::Fail)
        .remedy(crate::document::Remedy::one_of(
            subject,
            opts,
            loc(
                format!("Set {label_en} to one of the valid target ids."),
                format!("{label_de} auf eine der gültigen Ziel-Ids setzen."),
            ),
        ))
        .build(),
    );
}

/// 🔗 Referential integrity + unique entity ids before clause checks (CORRECTION 14:42).
fn push_referential_integrity(report: &mut CheckReport, document: &Din4108Snapshot) {
    let zone_ids: Vec<String> = document.zones.iter().map(|z| z.id.clone()).collect();
    let zone_set: std::collections::BTreeSet<&str> = zone_ids.iter().map(String::as_str).collect();
    let catalogue = part_4::design_lambda_material_ids();
    let catalogue_set: std::collections::BTreeSet<&str> = catalogue.iter().map(String::as_str).collect();

    push_duplicate_ids(report, "zones", &zone_ids, &|id| format!("zones[id={id}].id"));
    push_duplicate_ids(report, "elements", &document.elements.iter().map(|e| e.id.clone()).collect::<Vec<_>>(), &|id| format!("elements[id={id}].id"));
    push_duplicate_ids(report, "thermalBridges", &document.thermal_bridges.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("thermalBridges[id={id}].id"));

    for zone in &document.zones {
        push_duplicate_ids(
            report,
            "windows",
            &zone.windows.iter().map(|w| w.id.clone()).collect::<Vec<_>>(),
            &|id| format!("zones[id={}].windows[id={id}].id", zone.id),
        );
    }
    for element in &document.elements {
        push_duplicate_ids(
            report,
            "layers",
            &element.layers.iter().map(|l| l.id.clone()).collect::<Vec<_>>(),
            &|id| format!("elements[id={}].layers[id={id}].id", element.id),
        );
        for layer in &element.layers {
            push_duplicate_ids(
                report,
                "segments",
                &layer.segments.iter().map(|s| s.id.clone()).collect::<Vec<_>>(),
                &|id| format!("elements[id={}].layers[id={}].segments[id={id}].id", element.id, layer.id),
            );
        }
    }

    for element in &document.elements {
        if !zone_set.contains(element.zone_id.as_str()) {
            push_dangling_ref(
                report,
                format!("din4108.integrity.elements.{}.zoneId", element.id),
                format!("elements[id={}].zoneId", element.id),
                &element.id,
                "Element zoneId",
                "Bauteil zoneId",
                &element.zone_id,
                zone_ids.clone(),
            );
        }
        for layer in &element.layers {
            if !layer.material_id.is_empty() && !catalogue_set.contains(layer.material_id.as_str()) {
                push_dangling_ref(
                    report,
                    format!("din4108.integrity.layers.{}.{}.materialId", element.id, layer.id),
                    format!("elements[id={}].layers[id={}].materialId", element.id, layer.id),
                    &layer.id,
                    "Layer materialId",
                    "Schicht materialId",
                    &layer.material_id,
                    catalogue.clone(),
                );
            }
            for seg in &layer.segments {
                if !seg.material_id.is_empty() && !catalogue_set.contains(seg.material_id.as_str()) {
                    push_dangling_ref(
                        report,
                        format!("din4108.integrity.segments.{}.{}.{}.materialId", element.id, layer.id, seg.id),
                        format!("elements[id={}].layers[id={}].segments[id={}].materialId", element.id, layer.id, seg.id),
                        &seg.id,
                        "Segment materialId",
                        "Abschnitt materialId",
                        &seg.material_id,
                        catalogue.clone(),
                    );
                }
            }
        }
    }
}

/// 📋️ Evaluate the complete DIN 4108 envelope subject.
pub fn evaluate(document: &Din4108Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    push_referential_integrity(&mut report, document);
    let t_ext = document.climate_zone.design_external_temperature_c();

    for zone in &document.zones {
        report.push(part_2::check_summer_heat(zone, document.climate_zone, &document.elements));
        report.push(part_2::check_zone_transmission_loss(zone, &document.elements, &document.usage, document.t_int_c));
    }
    if document.zones.is_empty() {
        report.push(
            crate::document::CheckResult::assess(
                "din4108-2.summer.none",
                "DIN 4108-2",
                crate::document::ClauseId::new("DIN 4108-2", "§8", "8.3"),
                crate::document::SubjectRef::whole(crate::document::LocalizedCopy::new("Building", "Gebäude")),
                crate::document::LocalizedCopy::new("Summer heat protection S_vorh", "Sommerlicher Wärmeschutz S_vorh"),
            )
            .status(crate::document::CheckStatus::Fail)
            .explanation(crate::document::LocalizedCopy::new(
                "No thermal zones defined — summer heat protection cannot be demonstrated.",
                "Keine thermischen Zonen definiert — sommerlicher Wärmeschutz ist nicht nachweisbar.",
            ))
            .remedy({
                let mut remedy = crate::document::Remedy::at_least(
                    crate::document::SubjectRef::new(
                        "",
                        "zones",
                        crate::document::LocalizedCopy::new("Thermal zones", "Thermische Zonen"),
                    ),
                    crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.0),
                    crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
                    crate::document::LocalizedCopy::new(
                        "Insert at least one thermal zone (use insertItem on `zones`).",
                        "Mindestens eine thermische Zone einfügen (insertItem auf `zones`).",
                    ),
                );
                remedy.applicable = false;
                remedy
            })
            .annex(crate::document::AnnexChoice::De)
            .build(),
        );
    }

    let opaque_area = opaque_envelope_area(&document.elements);
    for element in &document.elements {
        report.push(part_2::check_minimum_r(element, &document.usage, document.t_int_c));
        report.push(part_2::check_f_rsi(element, document.t_int_c, t_ext));
        report.push(part_3::check_glaser(element, document.t_int_c, document.rh_int, document.climate_zone));
        report.push(part_6::check_u_value(element, &document.usage, document.t_int_c));
        report.push(part_6::check_u_prime(element, &document.thermal_bridges, opaque_area, &document.usage, document.t_int_c));
        for li in 0..element.layers.len() {
            report.push(part_4::check_design_lambda(element, li));
            report.push(part_10::check_application(element, li));
            for si in 0..element.layers[li].segments.len() {
                report.push(part_4::check_segment_design_lambda(element, li, si));
            }
        }
    }
    if document.elements.is_empty() {
        report.push(
            crate::document::CheckResult::assess(
                "din4108-2.table3.none",
                "DIN 4108-2",
                crate::document::ClauseId::new("DIN 4108-2", "Table 3", "R_min"),
                crate::document::SubjectRef::whole(crate::document::LocalizedCopy::new("Building", "Gebäude")),
                crate::document::LocalizedCopy::new("Minimum thermal resistance (Table 3)", "Mindestwärmedurchlasswiderstand (Tabelle 3)"),
            )
            .status(crate::document::CheckStatus::Fail)
            .explanation(crate::document::LocalizedCopy::new(
                "No envelope elements defined — Table 3 minimum R cannot be demonstrated.",
                "Keine Hüllflächenbauteile definiert — Mindest-R nach Tabelle 3 ist nicht nachweisbar.",
            ))
            .remedy({
                let mut remedy = crate::document::Remedy::at_least(
                    crate::document::SubjectRef::new(
                        "",
                        "elements",
                        crate::document::LocalizedCopy::new("Envelope elements", "Hüllflächen"),
                    ),
                    crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.0),
                    crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
                    crate::document::LocalizedCopy::new(
                        "Insert at least one opaque envelope element (use insertItem on `elements`).",
                        "Mindestens ein opakes Hüllflächenbauteil einfügen (insertItem auf `elements`).",
                    ),
                );
                remedy.applicable = false;
                remedy
            })
            .annex(crate::document::AnnexChoice::De)
            .build(),
        );
    }

    report.push(part_7::check_airtightness(document.airtightness_n50, document.has_mechanical_ventilation));
    for bridge in &document.thermal_bridges {
        report.push(bb_2::check_bridge_category(bridge));
    }
    report.push(bb_2::check_equivalence(&document.thermal_bridges, opaque_area, document.bb2_details_conform));
    report
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
pub use super::outline::Din4108Outline;
//#endregion 🔁️Re-exports
