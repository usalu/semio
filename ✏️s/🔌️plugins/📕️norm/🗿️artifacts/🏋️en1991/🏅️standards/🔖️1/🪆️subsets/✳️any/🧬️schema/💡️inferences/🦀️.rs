//! 💡️ En1991 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1991Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1991 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991.inference")]
pub struct En1991Inference {
    #[derived]
    pub outline: En1991Outline,
}

impl protocol::Inference<En1991Snapshot> for En1991Inference {
    fn infer(snapshot: &En1991Snapshot) -> Self {
        Self { outline: En1991Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1991Snapshot> for En1991Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1991.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1991.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1991Builder {
    type Snapshot = En1991Snapshot;
    type Inference = En1991Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1991.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1991_artifact_schema_descriptor`'s registration.
pub fn en1991_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1991.inference",
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
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::standards::v1::subsets::any::schema::{
    assess_covers, part_1_1, part_1_2, part_1_3, part_1_4, part_1_5, part_1_6, part_1_7, part_2, part_3, part_4,
    raise_energy_remedy, raise_force_remedy, raise_power_remedy, raise_pressure_remedy, raise_temp_remedy,
};

fn copy(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }
fn pressure(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }
fn force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
fn temp(k: f64) -> Quantity { Quantity::new(QuantityKind::Temperature, k) }

fn push_duplicate_ids(report: &mut CheckReport, document: &En1991Snapshot, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, copy(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = CheckResult::assess(
            format!("en1991.integrity.duplicate.{table}.{id}"),
            "EN 1991 integrity",
            ClauseId::new("EN 1991", "§2", "id"),
            subject.clone(),
            copy(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
        )
        .annex(document.annex)
        .explanation(copy(
            &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(crate::document::CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            copy(
                &format!("Rename the duplicated '{id}' entry to a free id."),
                &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

#[allow(dead_code)]
fn push_dangling_ref(report: &mut CheckReport, document: &En1991Snapshot, check_id: String, path: String, subject_id: &str, label_en: &str, label_de: &str, current: &str, options: Vec<String>) {
    let subject = SubjectRef::new(subject_id, &path, copy(label_en, label_de));
    let opts = if options.is_empty() { vec![format!("{current}-missing")] } else { options };
    report.push(
        CheckResult::assess(
            check_id,
            "EN 1991 integrity",
            ClauseId::new("EN 1991", "§2", "reference"),
            subject.clone(),
            copy(&format!("Referential integrity — {label_en}"), &format!("Referenzintegrität — {label_de}")),
        )
        .annex(document.annex)
        .explanation(copy(
            &format!("'{current}' does not reference an existing target; dependent checks for this row must not Pass."),
            &format!("'{current}' verweist auf kein vorhandenes Ziel; abhängige Nachweise für diese Zeile dürfen nicht bestehen."),
        ))
        .status(crate::document::CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            opts,
            copy(
                &format!("Set {label_en} to one of the existing target ids."),
                &format!("{label_de} auf eine der vorhandenen Ziel-Ids setzen."),
            ),
        ))
        .build(),
    );
}

/// 🔗 Unique entity ids + dangling cross-refs (CORRECTION 14:42). No cross-ref leaves on the subject today; helper retained for any that appear.
fn push_referential_integrity(report: &mut CheckReport, document: &En1991Snapshot) {
    push_duplicate_ids(report, document, "floors", &document.floors.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("floors[id={id}].id"));
    push_duplicate_ids(report, document, "selfWeightElements", &document.self_weight_elements.iter().map(|e| e.id.clone()).collect::<Vec<_>>(), &|id| format!("selfWeightElements[id={id}].id"));
    push_duplicate_ids(report, document, "roofs", &document.roofs.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("roofs[id={id}].id"));
    push_duplicate_ids(report, document, "windFaces", &document.wind_faces.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("windFaces[id={id}].id"));
    push_duplicate_ids(report, document, "accidentalCases", &document.accidental_cases.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), &|id| format!("accidentalCases[id={id}].id"));
}

/// 🧪 Evaluate assumed design loads against EN 1991 / DIN EN NA required characteristic actions.
pub fn check_full_actions(document: &En1991Snapshot) -> CheckReport {
    let annex = document.annex;
    let mut report = CheckReport::default();
    push_referential_integrity(&mut report, document);
    let storeys = document.storey_count;

    if document.floors.is_empty() {
        report.push(CheckResult::assess("en1991.1-1.imposed.none", "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "§6.3", "6.1"), SubjectRef::whole(copy("Floors", "Geschosse")), copy("Imposed floor loads", "Nutzlasten auf Decken")).not_applicable(copy("No floor areas defined.", "Keine Deckenflächen definiert.")).annex(annex).build());
    }
    for floor in &document.floors {
        let required = part_1_1::imposed_qk_reduced_pa(&floor.category, annex, floor.area, storeys);
        let path = format!("floors[id={}].assumedQk", floor.id);
        let alpha_a = part_1_1::alpha_a(&floor.category, floor.area, annex);
        let alpha_n = part_1_1::alpha_n(&floor.category, storeys, annex);
        let remedy = if floor.assumed_qk + 1e-9 < required { Some(raise_pressure_remedy(&path, &floor.id, "imposed q_k", "Nutzlast q_k", floor.assumed_qk, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.imposed.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Table 6.1 / Eq. 6.1DE–6.2DE", "6.3.1"), SubjectRef::new(&floor.id, &path, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Imposed characteristic load q_k (α_A·α_n)", "Charakteristische Nutzlast q_k (α_A·α_n)"), pressure(floor.assumed_qk), pressure(required), copy(&format!("Required q_k = {:.3} kN/m² (α_A={:.3}, α_n={:.3}, A={:.1} m², n={}) for {}; assumed {:.3} kN/m².", required / 1000.0, alpha_a, alpha_n, floor.area, storeys, floor.category, floor.assumed_qk / 1000.0), &format!("Erforderliches q_k = {:.3} kN/m² (α_A={:.3}, α_n={:.3}, A={:.1} m², n={}) für {}; angenommen {:.3} kN/m².", required / 1000.0, alpha_a, alpha_n, floor.area, storeys, floor.category, floor.assumed_qk / 1000.0)), annex, remedy));
        let assumed_f = floor.assumed_qk * floor.area;
        let required_f = required * floor.area;
        let path_a = format!("floors[id={}].assumedQk", floor.id);
        let remedy_a = if floor.assumed_qk + 1e-9 < required {
            Some(raise_force_remedy(&path_a, &floor.id, "floor imposed q_k", "Deckennutzlast q_k", floor.assumed_qk, required))
        } else {
            None
        };
        report.push(assess_covers(&format!("en1991.1-1.floor-force.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "§6.3.1", "6.3.1"), SubjectRef::new(&floor.id, &path_a, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Total imposed force q_k·A", "Gesamte Nutzlastkraft q_k·A"), force(assumed_f), force(required_f), copy(&format!("Required F = q_k·A = {:.1} kN on A={:.1} m²; assumed {:.1} kN.", required_f / 1000.0, floor.area, assumed_f / 1000.0), &format!("Erforderliches F = q_k·A = {:.1} kN auf A={:.1} m²; angenommen {:.1} kN.", required_f / 1000.0, floor.area, assumed_f / 1000.0)), annex, remedy_a));
        let req_q = part_1_1::imposed_qk_concentrated_n(&floor.category, annex);
        let path_q = format!("floors[id={}].assumedQkConcentrated", floor.id);
        let remedy_q = if floor.assumed_qk_concentrated + 1e-9 < req_q { Some(raise_force_remedy(&path_q, &floor.id, "concentrated Q_k", "Einzellast Q_k", floor.assumed_qk_concentrated, req_q)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.concentrated.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Table 6.2", "6.3.1"), SubjectRef::new(&floor.id, &path_q, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Concentrated imposed load Q_k", "Charakteristische Einzellast Q_k"), force(floor.assumed_qk_concentrated), force(req_q), copy(&format!("Required Q_k = {:.2} kN; assumed {:.2} kN.", req_q / 1000.0, floor.assumed_qk_concentrated / 1000.0), &format!("Erforderliches Q_k = {:.2} kN; angenommen {:.2} kN.", req_q / 1000.0, floor.assumed_qk_concentrated / 1000.0)), annex, remedy_q));
        let req_p = part_1_1::partitions_allowance_pa(annex);
        let path_p = format!("floors[id={}].assumedPartitions", floor.id);
        let remedy_p = if floor.assumed_partitions + 1e-9 < req_p { Some(raise_pressure_remedy(&path_p, &floor.id, "partition load", "Trennwandlast", floor.assumed_partitions, req_p)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.partitions.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "§6.3.1.2", "6.3.1.2"), SubjectRef::new(&floor.id, &path_p, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Partition allowance", "Zuschlag für leichte Trennwände"), pressure(floor.assumed_partitions), pressure(req_p), copy(&format!("Required partition allowance = {:.2} kN/m²; assumed {:.2} kN/m².", req_p / 1000.0, floor.assumed_partitions / 1000.0), &format!("Erforderlicher Trennwandzuschlag = {:.2} kN/m²; angenommen {:.2} kN/m².", req_p / 1000.0, floor.assumed_partitions / 1000.0)), annex, remedy_p));
    }

    if document.self_weight_elements.is_empty() {
        report.push(CheckResult::assess("en1991.1-1.self-weight.none", "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Annex A", "5.1"), SubjectRef::whole(copy("Self-weight", "Eigengewicht")), copy("Permanent self-weight", "Ständige Einwirkungen aus Eigengewicht")).not_applicable(copy("No self-weight elements defined.", "Keine Eigengewichtselemente definiert.")).annex(annex).build());
    }
    for el in &document.self_weight_elements {
        let required = part_1_1::self_weight_pa(&el.material, el.thickness);
        let path = format!("selfWeightElements[id={}].assumedGk", el.id);
        let remedy = if el.assumed_gk + 1e-9 < required { Some(raise_pressure_remedy(&path, &el.id, "self-weight g_k", "Eigengewicht g_k", el.assumed_gk, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.self-weight.{}", el.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Annex A", "5.1"), SubjectRef::new(&el.id, &path, copy(&format!("Element {}", el.id), &format!("Element {}", el.id))), copy("Permanent self-weight g_k", "Charakteristisches Eigengewicht g_k"), pressure(el.assumed_gk), pressure(required), copy(&format!("Required g_k = {:.2} kN/m² for {} @ {:.3} m; assumed {:.2} kN/m².", required / 1000.0, el.material, el.thickness, el.assumed_gk / 1000.0), &format!("Erforderliches g_k = {:.2} kN/m² für {} @ {:.3} m; angenommen {:.2} kN/m².", required / 1000.0, el.material, el.thickness, el.assumed_gk / 1000.0)), annex, remedy));
    }

    match document.fire_mode {
        crate::FireMode::None => {
            report.push(CheckResult::assess("en1991.1-2.fire.none", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "§3", "3.1"), SubjectRef::whole(copy("Fire", "Brand")), copy("Fire thermal actions", "Thermische Einwirkungen im Brandfall")).not_applicable(copy("Fire design mode is none.", "Brandbemessungsmodus ist none.")).annex(annex).build());
        }
        crate::FireMode::Nominal => {
            let curve = document.fire_curve;
            let theta_g = part_1_2::nominal_gas_temp_k(curve, document.fire_duration);
            let path_g = "assumedGasTemperature";
            let remedy_g = if document.assumed_gas_temperature + 1e-9 < theta_g { Some(raise_temp_remedy(path_g, "", "gas temperature", "Gastemperatur", document.assumed_gas_temperature, theta_g)) } else { None };
            report.push(assess_covers("en1991.1-2.gas-temperature", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "§3.2", "3.2"), SubjectRef::new("", path_g, copy("Fire gas temperature", "Brandgastemperatur")), copy("Fire gas temperature θ_g", "Brandgastemperatur θ_g"), temp(document.assumed_gas_temperature), temp(theta_g), copy(&format!("Required θ_g = {:.1} K at t = {:.0} s; assumed {:.1} K.", theta_g, document.fire_duration, document.assumed_gas_temperature), &format!("Erforderliches θ_g = {:.1} K bei t = {:.0} s; angenommen {:.1} K.", theta_g, document.fire_duration, document.assumed_gas_temperature)), annex, remedy_g));
            let alpha_c = part_1_2::alpha_c_for_curve(curve);
            let h_net = part_1_2::h_net_w_m2(theta_g, 293.15, alpha_c, 1.0, 0.8, 1.0);
            let path_h = "assumedHNet";
            let remedy_h = if document.assumed_h_net + 1e-9 < h_net { Some(raise_power_remedy(path_h, "", "net heat flux", "Nettowärmestrom", document.assumed_h_net, h_net)) } else { None };
            report.push(assess_covers("en1991.1-2.h-net", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Eq. (3.1)/(3.2)", "3.1"), SubjectRef::new("", path_h, copy("Net heat flux", "Nettowärmestrom")), copy("Net heat flux ĥ_net", "Nettowärmestrom ĥ_net"), Quantity::new(QuantityKind::Power, document.assumed_h_net), Quantity::new(QuantityKind::Power, h_net), copy(&format!("Required ĥ_net = {:.0} W/m² (α_c={alpha_c}); assumed {:.0} W/m².", h_net, document.assumed_h_net), &format!("Erforderliches ĥ_net = {:.0} W/m² (α_c={alpha_c}); angenommen {:.0} W/m².", h_net, document.assumed_h_net)), annex, remedy_h));
            let qfd = part_1_2::design_fire_load_j_m2(document.fire_load_density_qf, annex, &document.fire_occupancy, 0.8);
            let path_q = "assumedQfD";
            let remedy_q = if document.assumed_qf_d + 1e-9 < qfd { Some(raise_energy_remedy(path_q, "", "design fire load", "Bemessungsbrandlastdichte", document.assumed_qf_d, qfd)) } else { None };
            report.push(assess_covers("en1991.1-2.fire-load", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex E / NA", "E.1"), SubjectRef::new("", path_q, copy("Design fire load", "Bemessungsbrandlastdichte")), copy("Design fire load density q_f,d", "Bemessungsbrandlastdichte q_f,d"), Quantity::new(QuantityKind::Energy, document.assumed_qf_d), Quantity::new(QuantityKind::Energy, qfd), copy(&format!("Required q_f,d = {:.0} MJ/m²; assumed {:.0} MJ/m².", qfd / 1e6, document.assumed_qf_d / 1e6), &format!("Erforderliches q_f,d = {:.0} MJ/m²; angenommen {:.0} MJ/m².", qfd / 1e6, document.assumed_qf_d / 1e6)), annex, remedy_q));
        }
        crate::FireMode::Parametric => {
            let o_geom = part_1_2::compartment_opening_factor(document.fire_compartment_area, document.fire_compartment_height);
            let o = if document.fire_opening_factor > 1e-9 { document.fire_opening_factor } else { o_geom };
            let path_o = "fireOpeningFactor";
            let remedy_o = if (document.fire_opening_factor - o_geom).abs() > 0.005 + 1e-9 {
                Some(crate::document::Remedy::at_least(SubjectRef::new("", path_o, copy("Opening factor O", "Öffnungsfaktor O")), Quantity::new(QuantityKind::Dimensionless, document.fire_opening_factor.abs()), Quantity::new(QuantityKind::Dimensionless, o_geom.abs()), copy(&format!("Set O to geometry-derived {o_geom:.4} m½ from A_f={:.1} m², H={:.2} m.", document.fire_compartment_area, document.fire_compartment_height), &format!("O auf geometriebasierten Wert {o_geom:.4} m½ aus A_f={:.1} m², H={:.2} m setzen.", document.fire_compartment_area, document.fire_compartment_height))))
            } else { None };
            report.push(assess_covers("en1991.1-2.opening-factor", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex A", "A.2"), SubjectRef::new("", path_o, copy("Opening factor", "Öffnungsfaktor")), copy("Opening factor O from compartment geometry", "Öffnungsfaktor O aus Brandabschnittsgeometrie"), Quantity::new(QuantityKind::Dimensionless, 1.0 - (document.fire_opening_factor - o_geom).abs().min(1.0)), Quantity::new(QuantityKind::Dimensionless, 1.0), copy(&format!("Geometry O = {o_geom:.4} (A_f={:.1} m², H={:.2} m); entered {:.4}.", document.fire_compartment_area, document.fire_compartment_height, document.fire_opening_factor), &format!("Geometrie-O = {o_geom:.4} (A_f={:.1} m², H={:.2} m); eingegeben {:.4}.", document.fire_compartment_area, document.fire_compartment_height, document.fire_opening_factor)), annex, remedy_o));
            let theta_g = part_1_2::parametric_gas_temp_k(document.fire_duration, o, document.fire_thermal_inertia);
            let qfd = part_1_2::design_fire_load_j_m2(document.fire_load_density_qf, annex, &document.fire_occupancy, 0.8);
            let qt_d = part_1_2::design_fire_load_qt_d_j_m2(qfd, document.fire_compartment_area, document.fire_compartment_height);
            let theta_max = part_1_2::parametric_theta_max_k(o, document.fire_thermal_inertia, qt_d);
            let required_theta = theta_g.max(theta_max);
            let path_g = "assumedGasTemperature";
            let remedy_g = if document.assumed_gas_temperature + 1e-9 < required_theta { Some(raise_temp_remedy(path_g, "", "gas temperature", "Gastemperatur", document.assumed_gas_temperature, required_theta)) } else { None };
            report.push(assess_covers("en1991.1-2.gas-temperature", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex A", "A.2"), SubjectRef::new("", path_g, copy("Fire gas temperature", "Brandgastemperatur")), copy("Parametric fire gas temperature θ_g / θ_max", "Parametrische Brandgastemperatur θ_g / θ_max"), temp(document.assumed_gas_temperature), temp(required_theta), copy(&format!("Required θ = {:.1} K (t={:.0} s, O={o:.4}, A_f={:.1} m², H={:.2} m, q_t,d={:.0} MJ/m²); assumed {:.1} K.", required_theta, document.fire_duration, document.fire_compartment_area, document.fire_compartment_height, qt_d / 1e6, document.assumed_gas_temperature), &format!("Erforderliches θ = {:.1} K (t={:.0} s, O={o:.4}, A_f={:.1} m², H={:.2} m, q_t,d={:.0} MJ/m²); angenommen {:.1} K.", required_theta, document.fire_duration, document.fire_compartment_area, document.fire_compartment_height, qt_d / 1e6, document.assumed_gas_temperature)), annex, remedy_g));
            let alpha_c = part_1_2::alpha_c_for_curve(crate::part_1_2::FireCurve::Parametric);
            let h_net = part_1_2::h_net_w_m2(required_theta, 293.15, alpha_c, 1.0, 0.8, 1.0);
            let path_h = "assumedHNet";
            let remedy_h = if document.assumed_h_net + 1e-9 < h_net { Some(raise_power_remedy(path_h, "", "net heat flux", "Nettowärmestrom", document.assumed_h_net, h_net)) } else { None };
            report.push(assess_covers("en1991.1-2.h-net", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Eq. (3.1)/(3.2)", "3.1"), SubjectRef::new("", path_h, copy("Net heat flux", "Nettowärmestrom")), copy("Net heat flux ĥ_net", "Nettowärmestrom ĥ_net"), Quantity::new(QuantityKind::Power, document.assumed_h_net), Quantity::new(QuantityKind::Power, h_net), copy(&format!("Required ĥ_net = {:.0} W/m²; assumed {:.0} W/m².", h_net, document.assumed_h_net), &format!("Erforderliches ĥ_net = {:.0} W/m²; angenommen {:.0} W/m².", h_net, document.assumed_h_net)), annex, remedy_h));
            let path_q = "assumedQfD";
            let remedy_q = if document.assumed_qf_d + 1e-9 < qfd { Some(raise_energy_remedy(path_q, "", "design fire load", "Bemessungsbrandlastdichte", document.assumed_qf_d, qfd)) } else { None };
            report.push(assess_covers("en1991.1-2.fire-load", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex E / NA", "E.1"), SubjectRef::new("", path_q, copy("Design fire load", "Bemessungsbrandlastdichte")), copy("Design fire load density q_f,d → q_t,d", "Bemessungsbrandlastdichte q_f,d → q_t,d"), Quantity::new(QuantityKind::Energy, document.assumed_qf_d), Quantity::new(QuantityKind::Energy, qfd), copy(&format!("Required q_f,d = {:.0} MJ/m² (q_t,d={:.0} MJ/m² from A_f/H); assumed {:.0} MJ/m².", qfd / 1e6, qt_d / 1e6, document.assumed_qf_d / 1e6), &format!("Erforderliches q_f,d = {:.0} MJ/m² (q_t,d={:.0} MJ/m² aus A_f/H); angenommen {:.0} MJ/m².", qfd / 1e6, qt_d / 1e6, document.assumed_qf_d / 1e6)), annex, remedy_q));
        }
    }

    if document.roofs.is_empty() {
        report.push(CheckResult::assess("en1991.1-3.snow.none", "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§5.2", "5.2"), SubjectRef::whole(copy("Roofs", "Dächer")), copy("Snow on roofs", "Schneelast auf Dächern")).not_applicable(copy("No roof areas defined.", "Keine Dachflächen definiert.")).annex(annex).build());
    }
    let sk = part_1_3::design_ground_snow_pa(annex, &document.snow_zone, document.altitude, document.en_sk);
    for roof in &document.roofs {
        let mu = part_1_3::shape_mu(&roof.roof_type, roof.pitch_deg, roof.has_parapet, roof.parapet_height, roof.drift_obstruction_height, roof.multi_span, document.exceptional_snow_north_german_lowlands);
        let required = part_1_3::roof_snow_pa(sk, mu, roof.c_e, roof.c_t);
        let path = format!("roofs[id={}].assumedSk", roof.id);
        let remedy = if roof.assumed_sk + 1e-9 < required { Some(raise_pressure_remedy(&path, &roof.id, "snow s_k", "Schneelast s", roof.assumed_sk, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-3.snow.{}", roof.id), "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§5.2", "5.2"), SubjectRef::new(&roof.id, &path, copy(&format!("Roof {}", roof.id), &format!("Dach {}", roof.id))), copy("Snow load on roof", "Schneelast auf dem Dach"), pressure(roof.assumed_sk), pressure(required), copy(&format!("Required s = {:.3} kN/m² (µ={mu:.2}); assumed {:.3} kN/m².", required / 1000.0, roof.assumed_sk / 1000.0), &format!("Erforderliches s = {:.3} kN/m² (µ={mu:.2}); angenommen {:.3} kN/m².", required / 1000.0, roof.assumed_sk / 1000.0)), annex, remedy));
    }

    if document.wind_faces.is_empty() {
        report.push(CheckResult::assess("en1991.1-4.wind.none", "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "§5.2", "5.2"), SubjectRef::whole(copy("Wind faces", "Windflächen")), copy("Wind pressure", "Winddruck")).not_applicable(copy("No wind faces defined.", "Keine Windflächen definiert.")).annex(annex).build());
    }
    let vb = part_1_4::design_vb(annex, document.wind_zone, document.en_vb);
    let e_ref = part_1_4::reference_e(document.width, document.height);
    let h_over_d = document.height / document.depth.max(0.1);
    for face in &document.wind_faces {
        let is_roof = matches!(face.zone.trim().to_ascii_uppercase().as_str(), "F" | "G" | "H" | "I" | "J");
        let pitch = document.roofs.first().map(|r| r.pitch_deg).unwrap_or(0.0);
        let tab_cpe10 = part_1_4::tabulated_cpe10(&face.zone, h_over_d, is_roof, pitch);
        let tab_cpe1 = part_1_4::tabulated_cpe1(&face.zone, h_over_d, is_roof, pitch);
        let path_cpe10 = format!("windFaces[id={}].cPe10", face.id);
        let path_cpe1 = format!("windFaces[id={}].cPe1", face.id);
        let cpe10_ok = (face.c_pe10 - tab_cpe10).abs() <= 0.05 + 1e-9;
        let cpe1_ok = (face.c_pe1 - tab_cpe1).abs() <= 0.05 + 1e-9;
        let remedy10 = if !cpe10_ok {
            Some(crate::document::Remedy::at_least(
                SubjectRef::new(&face.id, &path_cpe10, copy("c_pe,10", "c_pe,10")),
                Quantity::new(QuantityKind::Dimensionless, face.c_pe10.abs()),
                Quantity::new(QuantityKind::Dimensionless, tab_cpe10.abs()),
                copy(
                    &format!("Set c_pe,10 to tabulated {tab_cpe10:.2} for zone {} (h/d={h_over_d:.2}, e={e_ref:.2} m).", face.zone),
                    &format!("c_pe,10 auf tabellierten Wert {tab_cpe10:.2} für Zone {} setzen (h/d={h_over_d:.2}, e={e_ref:.2} m).", face.zone),
                ),
            ))
        } else { None };
        report.push(assess_covers(
            &format!("en1991.1-4.cpe10.{}", face.id),
            "DIN EN 1991-1-4",
            ClauseId::new("EN 1991-1-4", "Tables 7.1–7.4", "7.2"),
            SubjectRef::new(&face.id, &path_cpe10, copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id))),
            copy("External pressure coefficient c_pe,10", "Außendruckbeiwert c_pe,10"),
            Quantity::new(QuantityKind::Dimensionless, if cpe10_ok { 1.0 } else { 1.0 - (face.c_pe10 - tab_cpe10).abs() }),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            copy(
                &format!("User c_pe,10={:.2} vs tabulated {tab_cpe10:.2} for zone {} (e=min(b,2h)={e_ref:.2} m).", face.c_pe10, face.zone),
                &format!("Benutzer-c_pe,10={:.2} vs Tabellenwert {tab_cpe10:.2} für Zone {} (e=min(b,2h)={e_ref:.2} m).", face.c_pe10, face.zone),
            ),
            annex,
            remedy10,
        ));
        let remedy1 = if !cpe1_ok {
            Some(crate::document::Remedy::at_least(
                SubjectRef::new(&face.id, &path_cpe1, copy("c_pe,1", "c_pe,1")),
                Quantity::new(QuantityKind::Dimensionless, face.c_pe1.abs()),
                Quantity::new(QuantityKind::Dimensionless, tab_cpe1.abs()),
                copy(
                    &format!("Set c_pe,1 to tabulated {tab_cpe1:.2} for zone {} (1 m² / Tables 7.1–7.4).", face.zone),
                    &format!("c_pe,1 auf tabellierten Wert {tab_cpe1:.2} für Zone {} setzen (1 m² / Tabellen 7.1–7.4).", face.zone),
                ),
            ))
        } else { None };
        report.push(assess_covers(
            &format!("en1991.1-4.cpe1.{}", face.id),
            "DIN EN 1991-1-4",
            ClauseId::new("EN 1991-1-4", "Tables 7.1–7.4 / Fig. 7.2", "7.2"),
            SubjectRef::new(&face.id, &path_cpe1, copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id))),
            copy("External pressure coefficient c_pe,1", "Außendruckbeiwert c_pe,1"),
            Quantity::new(QuantityKind::Dimensionless, if cpe1_ok { 1.0 } else { 1.0 - (face.c_pe1 - tab_cpe1).abs() }),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            copy(
                &format!("User c_pe,1={:.2} vs tabulated {tab_cpe1:.2} for zone {} (A={:.1} m²).", face.c_pe1, face.zone, face.loaded_area),
                &format!("Benutzer-c_pe,1={:.2} vs Tabellenwert {tab_cpe1:.2} für Zone {} (A={:.1} m²).", face.c_pe1, face.zone, face.loaded_area),
            ),
            annex,
            remedy1,
        ));
        let ze = part_1_4::reference_height_ze(face.z, document.height, e_ref);
        let qp = part_1_4::peak_velocity_pressure_pa(annex, document.wind_zone, document.terrain_category, ze, vb, document.air_density, document.orography_factor, document.mixed_terrain_upwind, document.mixed_terrain_distance, document.coast_or_island);
        let cpe_a = part_1_4::area_cpe(face.c_pe1, face.c_pe10, face.loaded_area);
        let required = part_1_4::wind_pressure_pa(qp, cpe_a.abs(), face.c_pi, face.c_s, face.c_d);
        let path = format!("windFaces[id={}].assumedWp", face.id);
        let remedy = if face.assumed_wp + 1e-9 < required { Some(raise_pressure_remedy(&path, &face.id, "wind pressure", "Winddruck", face.assumed_wp, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-4.wind.{}", face.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "§5.2 / Fig. 7.2", "5.2"), SubjectRef::new(&face.id, &path, copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id))), copy("Wind pressure w", "Winddruck w"), pressure(face.assumed_wp), pressure(required), copy(&format!("Required w = {:.3} kN/m² (q_p={:.3}, z_e={ze:.2}, e={e_ref:.2}, c_pe(A={:.1})={cpe_a:.3}); assumed {:.3} kN/m².", required / 1000.0, qp / 1000.0, face.loaded_area, face.assumed_wp / 1000.0), &format!("Erforderliches w = {:.3} kN/m² (q_p={:.3}, z_e={ze:.2}, e={e_ref:.2}, c_pe(A={:.1})={cpe_a:.3}); angenommen {:.3} kN/m².", required / 1000.0, qp / 1000.0, face.loaded_area, face.assumed_wp / 1000.0)), annex, remedy));
    }

    let req_dt = part_1_5::required_delta_t(annex, &document.thermal_element_type, document.t_max, document.t_min, document.t_0, document.thermal_bridge_type, document.delta_t_m);
    let path_dt = "assumedDeltaT";
    let remedy_dt = if document.assumed_delta_t + 1e-9 < req_dt { Some(raise_temp_remedy(path_dt, "", "temperature difference", "Temperaturdifferenz", document.assumed_delta_t, req_dt)) } else { None };
    report.push(assess_covers("en1991.1-5.thermal", "DIN EN 1991-1-5", ClauseId::new("EN 1991-1-5", "§5 / §6", "5"), SubjectRef::new("", path_dt, copy("Thermal action", "Temperatureinwirkung")), copy("Temperature difference ΔT", "Temperaturdifferenz ΔT"), temp(document.assumed_delta_t), temp(req_dt), copy(&format!("Required ΔT = {:.1} K from T_max/T_min/T_0 / bridge type; assumed {:.1} K.", req_dt, document.assumed_delta_t), &format!("Erforderliches ΔT = {:.1} K aus T_max/T_min/T_0 / Brückentyp; angenommen {:.1} K.", req_dt, document.assumed_delta_t)), annex, remedy_dt));

    let req_c = part_1_6::construction_qk_pa(&document.construction_activity);
    let path_c = "assumedConstructionQk";
    let remedy_c = if document.assumed_construction_qk + 1e-9 < req_c { Some(raise_pressure_remedy(path_c, "", "construction q_k", "Bauzustandslast", document.assumed_construction_qk, req_c)) } else { None };
    report.push(assess_covers("en1991.1-6.construction", "DIN EN 1991-1-6", ClauseId::new("EN 1991-1-6", "§4.11", "4.11"), SubjectRef::new("", path_c, copy("Construction loads", "Einwirkungen während der Ausführung")), copy("Construction imposed load", "Nutzlast im Bauzustand"), pressure(document.assumed_construction_qk), pressure(req_c), copy(&format!("Required q_k = {:.2} kN/m² for {}; assumed {:.2} kN/m².", req_c / 1000.0, document.construction_activity, document.assumed_construction_qk / 1000.0), &format!("Erforderliches q_k = {:.2} kN/m² für {}; angenommen {:.2} kN/m².", req_c / 1000.0, document.construction_activity, document.assumed_construction_qk / 1000.0)), annex, remedy_c));

    if document.accidental_cases.is_empty() {
        report.push(CheckResult::assess("en1991.1-7.accidental.none", "DIN EN 1991-1-7", ClauseId::new("EN 1991-1-7", "§4", "4"), SubjectRef::whole(copy("Accidental", "Außergewöhnlich")), copy("Accidental actions", "Außergewöhnliche Einwirkungen")).not_applicable(copy("No accidental cases claimed.", "Keine außergewöhnlichen Fälle angegeben.")).annex(annex).build());
    }
    for case in &document.accidental_cases {
        if let Some(imp) = case.impact.first() {
            let required = part_1_7::vehicle_impact_force_n(imp.vehicle_mass, imp.vehicle_speed);
            let path = format!("accidentalCases[id={}].impact[0].assumedForce", case.id);
            let remedy = if imp.assumed_force + 1e-9 < required { Some(raise_force_remedy(&path, &case.id, "impact force", "Anprallkraft", imp.assumed_force, required)) } else { None };
            report.push(assess_covers(&format!("en1991.1-7.impact.{}", case.id), "DIN EN 1991-1-7", ClauseId::new("EN 1991-1-7", "Annex C", "C"), SubjectRef::new(&case.id, &path, copy(&format!("Case {}", case.id), &format!("Fall {}", case.id))), copy("Vehicle impact force", "Fahrzeuganprallkraft"), force(imp.assumed_force), force(required), copy(&format!("Required F = {:.1} kN; assumed {:.1} kN.", required / 1000.0, imp.assumed_force / 1000.0), &format!("Erforderliches F = {:.1} kN; angenommen {:.1} kN.", required / 1000.0, imp.assumed_force / 1000.0)), annex, remedy));
        }
        if let Some(ex) = case.explosion.first() {
            let required = part_1_7::explosion_pressure_pa(ex.explosion_mass, ex.standoff);
            let path = format!("accidentalCases[id={}].explosion[0].assumedPressure", case.id);
            let remedy = if ex.assumed_pressure + 1e-9 < required { Some(raise_pressure_remedy(&path, &case.id, "explosion pressure", "Explosionsdruck", ex.assumed_pressure, required)) } else { None };
            report.push(assess_covers(&format!("en1991.1-7.explosion.{}", case.id), "DIN EN 1991-1-7", ClauseId::new("EN 1991-1-7", "Annex D", "D"), SubjectRef::new(&case.id, &path, copy(&format!("Case {}", case.id), &format!("Fall {}", case.id))), copy("Explosion pressure", "Explosionsdruck"), pressure(ex.assumed_pressure), pressure(required), copy(&format!("Required p = {:.2} kN/m²; assumed {:.2} kN/m².", required / 1000.0, ex.assumed_pressure / 1000.0), &format!("Erforderliches p = {:.2} kN/m²; angenommen {:.2} kN/m².", required / 1000.0, ex.assumed_pressure / 1000.0)), annex, remedy));
        }
    }



    if document.structure_kind != crate::StructureKind::Bridge {
        report.push(CheckResult::assess("en1991.2.bridge.none", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4", "4"), SubjectRef::whole(copy("Bridge", "Brücke")), copy("Bridge traffic loads", "Verkehrslasten auf Brücken")).not_applicable(copy("Structure kind is building — bridge traffic not applicable.", "Tragwerksart ist Gebäude — Brückenverkehr nicht anwendbar.")).annex(annex).build());
    } else {
        let (n_lanes, w_lane, w_rem) = part_2::notional_lanes(document.bridge_lane_width);
        let lane = document.bridge_lane.min(n_lanes).max(1);
        let path_lane = "bridgeLane";
        let lane_ok = document.bridge_lane >= 1 && document.bridge_lane <= n_lanes;
        let remedy_lane = if !lane_ok {
            Some(crate::document::Remedy::at_least(SubjectRef::new("", path_lane, copy("Notional lane", "Ideeller Fahrstreifen")), Quantity::new(QuantityKind::Dimensionless, f64::from(document.bridge_lane)), Quantity::new(QuantityKind::Dimensionless, 1.0), copy(&format!("Set bridgeLane to 1..{n_lanes} for carriageway width {:.2} m (Table 4.1 → n₁={n_lanes}, w_lane={w_lane:.2} m, remaining={w_rem:.2} m).", document.bridge_lane_width), &format!("bridgeLane auf 1..{n_lanes} setzen bei Fahrbahnbreite {:.2} m (Tabelle 4.1 → n₁={n_lanes}, w_lane={w_lane:.2} m, Rest={w_rem:.2} m).", document.bridge_lane_width))))
        } else { None };
        report.push(assess_covers("en1991.2.notional-lanes", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.1", "4.2.3"), SubjectRef::new("", "bridgeLaneWidth", copy("Carriageway width", "Fahrbahnbreite")), copy("Notional lane layout from carriageway width", "Ideelle Fahrstreifen aus Fahrbahnbreite"), Quantity::new(QuantityKind::Dimensionless, if lane_ok { 1.0 } else { 0.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0), copy(&format!("w={:.2} m → n₁={n_lanes}, w_lane={w_lane:.2} m, remaining={w_rem:.2} m; selected lane {}.", document.bridge_lane_width, document.bridge_lane), &format!("w={:.2} m → n₁={n_lanes}, w_lane={w_lane:.2} m, Rest={w_rem:.2} m; gewählter Streifen {}.", document.bridge_lane_width, document.bridge_lane)), annex, remedy_lane));
        let span = document.bridge_span;
        let req_t = part_2::lm1_tandem_n(annex, lane);
        let req_m = part_2::lm1_tandem_moment_nm(annex, lane, span);
        let assumed_m = document.assumed_bridge_tandem * span / 4.0;
        let path_t = "assumedBridgeTandem";
        let remedy_t = if assumed_m + 1e-9 < req_m { Some(raise_force_remedy(path_t, "", "LM1 tandem", "LM1-Tandem", document.assumed_bridge_tandem, req_t)) } else { None };
        report.push(assess_covers("en1991.2.lm1-tandem", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.2", "4.3.2"), SubjectRef::new("", path_t, copy("Bridge LM1 tandem", "Brücke LM1 Tandem")), copy("LM1 tandem moment α_Q·Q·L/4", "LM1-Tandemmoment α_Q·Q·L/4"), Quantity::new(QuantityKind::Moment, assumed_m), Quantity::new(QuantityKind::Moment, req_m), copy(&format!("Required M = {:.1} kNm (Q={:.0} kN, L={span:.1} m, lane {lane}); assumed M = {:.1} kNm.", req_m / 1000.0, req_t / 1000.0, assumed_m / 1000.0), &format!("Erforderliches M = {:.1} kNm (Q={:.0} kN, L={span:.1} m, Streifen {lane}); angenommenes M = {:.1} kNm.", req_m / 1000.0, req_t / 1000.0, assumed_m / 1000.0)), annex, remedy_t));
        let req_u = part_2::lm1_udl_pa(annex, lane);
        let path_u = "assumedBridgeUdl";
        let remedy_u = if document.assumed_bridge_udl + 1e-9 < req_u { Some(raise_pressure_remedy(path_u, "", "LM1 UDL", "LM1-Gleichlast", document.assumed_bridge_udl, req_u)) } else { None };
        report.push(assess_covers("en1991.2.lm1-udl", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.3.2"), SubjectRef::new("", path_u, copy("Bridge LM1 UDL", "Brücke LM1 Gleichlast")), copy("LM1 UDL q_ak on notional lane", "LM1-Gleichlast q_ak auf ideellem Fahrstreifen"), pressure(document.assumed_bridge_udl), pressure(req_u), copy(&format!("Required q_ak = {:.2} kN/m² (lane {lane}, w_lane={w_lane:.2}); assumed {:.2} kN/m².", req_u / 1000.0, document.assumed_bridge_udl / 1000.0), &format!("Erforderliches q_ak = {:.2} kN/m² (Streifen {lane}, w_lane={w_lane:.2}); angenommen {:.2} kN/m².", req_u / 1000.0, document.assumed_bridge_udl / 1000.0)), annex, remedy_u));
        if w_rem > 0.05 {
            let req_rem = part_2::remaining_area_udl_pa(annex);
            report.push(assess_covers("en1991.2.lm1-remaining", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.3.2"), SubjectRef::new("", path_u, copy("Remaining area", "Restfläche")), copy("LM1 remaining-area UDL", "LM1-Gleichlast Restfläche"), pressure(document.assumed_bridge_udl), pressure(req_rem), copy(&format!("Remaining width {w_rem:.2} m requires q = {:.2} kN/m².", req_rem / 1000.0), &format!("Restbreite {w_rem:.2} m erfordert q = {:.2} kN/m².", req_rem / 1000.0)), annex, None));
        }
        let req_lm2 = part_2::lm2_axle_n(annex);
        let path_lm2 = "assumedBridgeLm2";
        let remedy_lm2 = if document.assumed_bridge_lm2 + 1e-9 < req_lm2 { Some(raise_force_remedy(path_lm2, "", "LM2 axle", "LM2-Achse", document.assumed_bridge_lm2, req_lm2)) } else { None };
        report.push(assess_covers("en1991.2.lm2", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.3", "4.3.3"), SubjectRef::new("", path_lm2, copy("Bridge LM2", "Brücke LM2")), copy("LM2 single axle", "LM2-Einzelachse"), force(document.assumed_bridge_lm2), force(req_lm2), copy(&format!("Required Q = {:.0} kN; assumed {:.0} kN.", req_lm2 / 1000.0, document.assumed_bridge_lm2 / 1000.0), &format!("Erforderliches Q = {:.0} kN; angenommen {:.0} kN.", req_lm2 / 1000.0, document.assumed_bridge_lm2 / 1000.0)), annex, remedy_lm2));
        let req_fw = part_2::footway_pa(annex);
        let path_fw = "assumedBridgeFootway";
        let remedy_fw = if document.assumed_bridge_footway + 1e-9 < req_fw { Some(raise_pressure_remedy(path_fw, "", "footway load", "Gehweglast", document.assumed_bridge_footway, req_fw)) } else { None };
        report.push(assess_covers("en1991.2.footway", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§5.2", "5.2"), SubjectRef::new("", path_fw, copy("Bridge footway", "Brückengehweg")), copy("Footway / cycle-track load", "Geh- und Radweglast"), pressure(document.assumed_bridge_footway), pressure(req_fw), copy(&format!("Required q = {:.2} kN/m²; assumed {:.2} kN/m².", req_fw / 1000.0, document.assumed_bridge_footway / 1000.0), &format!("Erforderliches q = {:.2} kN/m²; angenommen {:.2} kN/m².", req_fw / 1000.0, document.assumed_bridge_footway / 1000.0)), annex, remedy_fw));
        let req_lm3 = part_2::lm3_axle_n(annex);
        let path_lm3 = "assumedBridgeLm3";
        let remedy_lm3 = if document.assumed_bridge_lm3 + 1e-9 < req_lm3 { Some(raise_force_remedy(path_lm3, "", "LM3 special vehicle", "LM3-Sonderfahrzeug", document.assumed_bridge_lm3, req_lm3)) } else { None };
        report.push(assess_covers("en1991.2.lm3", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.4 / NA", "4.3.4"), SubjectRef::new("", path_lm3, copy("Bridge LM3", "Brücke LM3")), copy("LM3 special vehicle axle", "LM3-Sonderfahrzeugachse"), force(document.assumed_bridge_lm3), force(req_lm3), copy(&format!("Required Q = {:.0} kN; assumed {:.0} kN.", req_lm3 / 1000.0, document.assumed_bridge_lm3 / 1000.0), &format!("Erforderliches Q = {:.0} kN; angenommen {:.0} kN.", req_lm3 / 1000.0, document.assumed_bridge_lm3 / 1000.0)), annex, remedy_lm3));
        let req_lm4 = part_2::lm4_crowd_pa(annex);
        let path_lm4 = "assumedBridgeLm4";
        let remedy_lm4 = if document.assumed_bridge_lm4 + 1e-9 < req_lm4 { Some(raise_pressure_remedy(path_lm4, "", "LM4 crowd", "LM4-Menschenmenge", document.assumed_bridge_lm4, req_lm4)) } else { None };
        report.push(assess_covers("en1991.2.lm4", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.5", "4.3.5"), SubjectRef::new("", path_lm4, copy("Bridge LM4", "Brücke LM4")), copy("LM4 crowd loading", "LM4-Menschenansammlung"), pressure(document.assumed_bridge_lm4), pressure(req_lm4), copy(&format!("Required q = {:.2} kN/m²; assumed {:.2} kN/m².", req_lm4 / 1000.0, document.assumed_bridge_lm4 / 1000.0), &format!("Erforderliches q = {:.2} kN/m²; angenommen {:.2} kN/m².", req_lm4 / 1000.0, document.assumed_bridge_lm4 / 1000.0)), annex, remedy_lm4));

        let groups = ["gr1a", "gr1b", "gr3", "gr4", "gr5"];
        let mut scores: Vec<(&str, f64, bool, &str)> = Vec::new();
        for group in groups {
            let mut demand: f64 = 0.0;
            let mut ok = true;
            let mut fail_path: &str = path_u;
            if part_2::load_group_includes_lm1(group) {
                let u_t = if req_t > 0.0 { req_t / document.assumed_bridge_tandem.max(1.0) } else { 0.0 };
                let u_u = if req_u > 0.0 { req_u / document.assumed_bridge_udl.max(1.0) } else { 0.0 };
                demand = demand.max(u_t).max(u_u);
                if document.assumed_bridge_tandem + 1e-9 < req_t { ok = false; fail_path = path_t; }
                if document.assumed_bridge_udl + 1e-9 < req_u { ok = false; fail_path = path_u; }
            }
            if part_2::load_group_includes_lm2(group) {
                let u = if req_lm2 > 0.0 { req_lm2 / document.assumed_bridge_lm2.max(1.0) } else { 0.0 };
                demand = demand.max(u);
                if document.assumed_bridge_lm2 + 1e-9 < req_lm2 { ok = false; fail_path = path_lm2; }
            }
            if part_2::load_group_includes_lm3(group) {
                let u = if req_lm3 > 0.0 { req_lm3 / document.assumed_bridge_lm3.max(1.0) } else { 0.0 };
                demand = demand.max(u);
                if document.assumed_bridge_lm3 + 1e-9 < req_lm3 { ok = false; fail_path = path_lm3; }
            }
            if part_2::load_group_includes_lm4(group) {
                let u = if req_lm4 > 0.0 { req_lm4 / document.assumed_bridge_lm4.max(1.0) } else { 0.0 };
                demand = demand.max(u);
                if document.assumed_bridge_lm4 + 1e-9 < req_lm4 { ok = false; fail_path = path_lm4; }
            }
            if part_2::load_group_includes_footway(group) {
                let u = if req_fw > 0.0 { req_fw / document.assumed_bridge_footway.max(1.0) } else { 0.0 };
                demand = demand.max(u);
                if document.assumed_bridge_footway + 1e-9 < req_fw { ok = false; fail_path = path_fw; }
            }
            scores.push((group, demand, ok, fail_path));
        }
        let max_d = scores.iter().map(|s| s.1).fold(0.0_f64, f64::max);
        let governors: Vec<&str> = scores.iter().filter(|s| (s.1 - max_d).abs() < 1e-6).map(|s| s.0).collect();
        let selected = document.bridge_load_group.trim();
        let selected_row = scores.iter().find(|s| s.0 == selected);
        let selected_ok = selected_row.map(|s| s.2).unwrap_or(false);
        let fail_path = selected_row.map(|s| s.3).unwrap_or(path_u);
        let governs = governors.iter().any(|g| *g == selected);
        let governing = governors.first().copied().unwrap_or("gr1a");
        let remedy_g = if !selected_ok {
            Some(match fail_path {
                "assumedBridgeTandem" => raise_force_remedy(path_t, "", "LM1 tandem", "LM1-Tandem", document.assumed_bridge_tandem, req_t),
                "assumedBridgeLm2" => raise_force_remedy(path_lm2, "", "LM2 axle", "LM2-Achse", document.assumed_bridge_lm2, req_lm2),
                "assumedBridgeLm3" => raise_force_remedy(path_lm3, "", "LM3 special vehicle", "LM3-Sonderfahrzeug", document.assumed_bridge_lm3, req_lm3),
                "assumedBridgeLm4" => raise_pressure_remedy(path_lm4, "", "LM4 crowd", "LM4-Menschenmenge", document.assumed_bridge_lm4, req_lm4),
                "assumedBridgeFootway" => raise_pressure_remedy(path_fw, "", "footway load", "Gehweglast", document.assumed_bridge_footway, req_fw),
                _ => raise_pressure_remedy(path_u, "", "LM1 UDL", "LM1-Gleichlast", document.assumed_bridge_udl, req_u),
            })
        } else if !governs {
            Some(crate::document::Remedy::at_least(SubjectRef::new("", "bridgeLoadGroup", copy("Load group", "Lastgruppe")), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), copy(&format!("Set bridgeLoadGroup to a governing group ({governing})."), &format!("bridgeLoadGroup auf eine maßgebende Gruppe ({governing}) setzen."))))
        } else { None };
        let selected_demand = selected_row.map(|s| s.1).unwrap_or(0.0);
        // Utilization of selected Table 4.4a group; inflate when not covered / not governing so status fails.
        let computed_g = if !selected_ok { selected_demand.max(1.0) + 1.0 } else if !governs { selected_demand.max(1.0) + 0.5 } else { selected_demand };
        let mut group_check = CheckResult::assess(
            "en1991.2.group.selected",
            "DIN EN 1991-2",
            ClauseId::new("EN 1991-2", "Table 4.4a", "4.5"),
            SubjectRef::new("", "bridgeLoadGroup", copy("Load group", "Lastgruppe")),
            copy("Selected load group governs Table 4.4a", "Gewählte Lastgruppe ist maßgebend nach Tabelle 4.4a"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, computed_g), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(copy(
            &format!("Selected {selected} demand={selected_demand:.3}; governing {:?}; constituents covered: {selected_ok}.", governors),
            &format!("Gewählt {selected} Ausnutzung={selected_demand:.3}; maßgebend {:?}; Anteile abgedeckt: {selected_ok}.", governors),
        ));
        if let Some(r) = remedy_g { group_check = group_check.remedy(r); }
        report.push(group_check.build());
    }


    if !document.crane_claimed {
        report.push(CheckResult::assess("en1991.3.crane.none", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2", "2"), SubjectRef::whole(copy("Crane", "Kran")), copy("Crane loads", "Kraneinwirkungen")).not_applicable(copy("Crane runway not claimed.", "Kranbahn nicht beansprucht.")).annex(annex).build());
    } else {
        let req_v = part_3::design_vertical_wheel_n(&document.crane_class, &document.hoist_class, document.hoisting_speed);
        let path_v = "assumedCraneWheel";
        let remedy_v = if document.assumed_crane_wheel + 1e-9 < req_v { Some(raise_force_remedy(path_v, "", "crane wheel", "Kranradlast", document.assumed_crane_wheel, req_v)) } else { None };
        report.push(assess_covers("en1991.3.crane-vertical", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.3", "2.3"), SubjectRef::new("", path_v, copy("Crane vertical", "Kran vertikal")), copy("Vertical wheel load", "Vertikale Radlast"), force(document.assumed_crane_wheel), force(req_v), copy(&format!("Required Q = {:.1} kN; assumed {:.1} kN.", req_v / 1000.0, document.assumed_crane_wheel / 1000.0), &format!("Erforderliches Q = {:.1} kN; angenommen {:.1} kN.", req_v / 1000.0, document.assumed_crane_wheel / 1000.0)), annex, remedy_v));
        let req_h = part_3::design_horizontal_force_n(&document.crane_class, annex);
        let path_h = "assumedCraneHorizontal";
        let remedy_h = if document.assumed_crane_horizontal + 1e-9 < req_h { Some(raise_force_remedy(path_h, "", "crane horizontal", "Kranhorizontallast", document.assumed_crane_horizontal, req_h)) } else { None };
        report.push(assess_covers("en1991.3.crane-horizontal", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.5", "2.5"), SubjectRef::new("", path_h, copy("Crane horizontal", "Kran horizontal")), copy("Horizontal crane force", "Horizontale Krankraft"), force(document.assumed_crane_horizontal), force(req_h), copy(&format!("Required H = {:.1} kN; assumed {:.1} kN.", req_h / 1000.0, document.assumed_crane_horizontal / 1000.0), &format!("Erforderliches H = {:.1} kN; angenommen {:.1} kN.", req_h / 1000.0, document.assumed_crane_horizontal / 1000.0)), annex, remedy_h));
    }

    if !document.silo_claimed {
        report.push(CheckResult::assess("en1991.4.silo.none", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§5", "5"), SubjectRef::whole(copy("Silo", "Silo")), copy("Silo / tank actions", "Silo-/Tankeinwirkungen")).not_applicable(copy("Silo/tank not claimed.", "Silo/Tank nicht beansprucht.")).annex(annex).build());
    } else if document.silo_kind == "tank" {
        let required = part_4::tank_hydrostatic_pa(document.silo_bulk_density, document.silo_height);
        let path = "assumedSiloPressure";
        let remedy = if document.assumed_silo_pressure + 1e-9 < required { Some(raise_pressure_remedy(path, "", "tank pressure", "Tankdruck", document.assumed_silo_pressure, required)) } else { None };
        report.push(assess_covers("en1991.4.tank", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§7", "7"), SubjectRef::new("", path, copy("Tank", "Tank")), copy("Tank hydrostatic pressure", "Tank-Hydrostatik"), pressure(document.assumed_silo_pressure), pressure(required), copy(&format!("Required p = {:.1} kN/m²; assumed {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0), &format!("Erforderliches p = {:.1} kN/m²; angenommen {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0)), annex, remedy));
    } else {
        let required = part_4::janssen_horizontal_pa(document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k, document.silo_height);
        let path = "assumedSiloPressure";
        let remedy = if document.assumed_silo_pressure + 1e-9 < required { Some(raise_pressure_remedy(path, "", "silo pressure", "Silodruck", document.assumed_silo_pressure, required)) } else { None };
        report.push(assess_covers("en1991.4.silo-pressure", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "Eq. (5.1)", "5.2"), SubjectRef::new("", path, copy("Silo", "Silo")), copy("Janssen horizontal pressure", "Janssen-Horizontaldruck"), pressure(document.assumed_silo_pressure), pressure(required), copy(&format!("Required p_h = {:.1} kN/m²; assumed {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0), &format!("Erforderliches p_h = {:.1} kN/m²; angenommen {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0)), annex, remedy));
        let req_p = part_4::patch_pressure_pa(document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k, document.silo_height, annex);
        let path_p = "assumedSiloPatch";
        let remedy_p = if document.assumed_silo_patch + 1e-9 < req_p { Some(raise_pressure_remedy(path_p, "", "silo patch", "Silo-Patch", document.assumed_silo_patch, req_p)) } else { None };
        report.push(assess_covers("en1991.4.silo-patch", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§5.2.1.2", "5.2.1.2"), SubjectRef::new("", path_p, copy("Silo patch", "Silo Patch")), copy("Patch pressure", "Patchdruck"), pressure(document.assumed_silo_patch), pressure(req_p), copy(&format!("Required p_patch = {:.1} kN/m²; assumed {:.1} kN/m².", req_p / 1000.0, document.assumed_silo_patch / 1000.0), &format!("Erforderliches p_patch = {:.1} kN/m²; angenommen {:.1} kN/m².", req_p / 1000.0, document.assumed_silo_patch / 1000.0)), annex, remedy_p));
        let req_w = part_4::wall_friction_pa(document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k, document.silo_height);
        let path_w = "assumedSiloWallFriction";
        let remedy_w = if document.assumed_silo_wall_friction + 1e-9 < req_w { Some(raise_pressure_remedy(path_w, "", "wall friction", "Wandreibung", document.assumed_silo_wall_friction, req_w)) } else { None };
        report.push(assess_covers("en1991.4.silo-wall-friction", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "Eq. (5.3)", "5.2"), SubjectRef::new("", path_w, copy("Silo wall friction", "Silowandreibung")), copy("Wall friction traction", "Wandreibungstraktion"), pressure(document.assumed_silo_wall_friction), pressure(req_w), copy(&format!("Required p_w = {:.1} kN/m²; assumed {:.1} kN/m².", req_w / 1000.0, document.assumed_silo_wall_friction / 1000.0), &format!("Erforderliches p_w = {:.1} kN/m²; angenommen {:.1} kN/m².", req_w / 1000.0, document.assumed_silo_wall_friction / 1000.0)), annex, remedy_w));
    }

    report
}


pub fn evaluate(document: &En1991Snapshot) -> CheckReport {
    check_full_actions(document)
}
//#endregion 🔖️ComplianceReport


//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1991Outline;
//#endregion 🔁️Re-exports
