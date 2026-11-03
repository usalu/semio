//! 💡️ En1991 inference schema — the fourth schema family alongside snapshot/diff/mutations
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

use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use framework_schema::ArtifactSchema;

extern crate semio_s_artifact_norm_en1991_inferences_rail_silo as rail_silo;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1991 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991.inference")]
pub struct En1991Inference {
    #[derived]
    pub outline: En1991Outline,
}

impl protocol::Inference<En1991Snapshot> for En1991Inference {
    fn infer(snapshot: &En1991Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: En1991Outline::compute(snapshot) }
    
        })
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

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1991.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1991_artifact_schema_descriptor`'s registration.
pub fn en1991_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.en1991.inference",
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
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
#[cfg(test)] use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1991_checks::{assess_covers, assess_covers_on, invalid_input, na_de, part_1_1, part_1_2, part_1_3, part_1_4, part_1_5, part_1_6, part_1_7, part_2, part_3, part_4, raise_energy_remedy, raise_force_remedy, raise_power_remedy, raise_pressure_remedy, raise_temp_remedy};

const OPENING_TOLERANCE: f64 = 0.005;
const CPE_TOLERANCE: f64 = 0.05;
fn lm1_axle_spacing_m() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.lm1-axle-spacing")
}
fn kn_pa(kn_m2: f64) -> f64 { kn_m2 * 1000.0 }
fn kn_n(kn: f64) -> f64 { kn * 1000.0 }

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
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            copy(
                &format!("Rename the duplicated '{id}' entry to a free id."),
                &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.ndp_reads().build());
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
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
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
    push_duplicate_ids(report, document, "bridgeLanes", &document.bridge_lanes.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeLanes[id={id}].id"));
    push_duplicate_ids(report, document, "constructionStages", &document.construction_stages.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("constructionStages[id={id}].id"));
    push_duplicate_ids(report, document, "roofSurfaces", &document.roof_surfaces.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("roofSurfaces[id={id}].id"));
    push_duplicate_ids(report, document, "placements", &document.placements.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));
    push_duplicate_ids(report, document, "specialVehicles", &document.special_vehicles.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("specialVehicles[id={id}].id"));
    push_duplicate_ids(report, document, "windBridges", &document.wind_bridges.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("windBridges[id={id}].id"));
    push_duplicate_ids(report, document, "dynamicElements", &document.dynamic_elements.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("dynamicElements[id={id}].id"));
    push_duplicate_ids(report, document, "impactActions", &document.impact_actions.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("impactActions[id={id}].id"));
    push_duplicate_ids(report, document, "internalExplosions", &document.internal_explosions.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("internalExplosions[id={id}].id"));
    push_duplicate_ids(report, document, "thermalSections", &document.thermal_sections.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("thermalSections[id={id}].id"));
    push_duplicate_ids(report, document, "cases", &document.cases.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={id}].id"));
    push_duplicate_ids(report, document, "helicopterLandings", &document.helicopter_landings.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("helicopterLandings[id={id}].id"));
    push_duplicate_ids(report, document, "tieRuns", &document.tie_runs.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("tieRuns[id={id}].id"));
    push_duplicate_ids(report, document, "constructionLoads", &document.construction_loads.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("constructionLoads[id={id}].id"));
    push_duplicate_ids(report, document, "fallingLoads", &document.falling_loads.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("fallingLoads[id={id}].id"));
    push_duplicate_ids(report, document, "executionImperfections", &document.execution_imperfections.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("executionImperfections[id={id}].id"));
    push_duplicate_ids(report, document, "derailmentCases", &document.derailment_cases.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("derailmentCases[id={id}].id"));
    push_duplicate_ids(report, document, "craneHoists", &document.crane_hoists.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("craneHoists[id={id}].id"));
    push_duplicate_ids(report, document, "footbridges", &document.footbridges.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("footbridges[id={id}].id"));
    push_duplicate_ids(report, document, "railDynamics", &document.rail_dynamics.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("railDynamics[id={id}].id"));
    push_duplicate_ids(report, document, "loadDispersals", &document.load_dispersals.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("loadDispersals[id={id}].id"));
    push_duplicate_ids(report, document, "bulkSolids", &document.bulk_solids.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("bulkSolids[id={id}].id"));
    push_duplicate_ids(report, document, "rotatingMachines", &document.rotating_machines.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("rotatingMachines[id={id}].id"));
    push_duplicate_ids(report, document, "dustExplosions", &document.dust_explosions.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("dustExplosions[id={id}].id"));
    push_duplicate_ids(report, document, "railLoadGroups", &document.rail_load_groups.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("railLoadGroups[id={id}].id"));
    push_duplicate_ids(report, document, "railDerailments", &document.rail_derailments.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("railDerailments[id={id}].id"));
    push_duplicate_ids(report, document, "fatigueModels", &document.fatigue_models.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("fatigueModels[id={id}].id"));
    push_duplicate_ids(report, document, "supportCollisions", &document.support_collisions.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("supportCollisions[id={id}].id"));
    push_duplicate_ids(report, document, "localFires", &document.local_fires.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("localFires[id={id}].id"));
    push_duplicate_ids(report, document, "snowGuards", &document.snow_guards.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("snowGuards[id={id}].id"));
    collection_checks::push_collection_integrity(report, document);
}

/// 🧪 Evaluate assumed design loads against EN 1991 / DIN EN NA required characteristic actions.
pub fn check_full_actions(document: &En1991Snapshot) -> CheckReport {
    check_full_actions_scoped(document, Scope::All)
}

/// 🎯️ The actions check restricted to `scope`: the parts `integrity`, `floors-none`, `self-weight-none`, `fire`, `snow`, `wind`, `thermal`, `construction`, `accidental-none`, `traffic` or one row of `floors`, `selfWeightElements`, `roofs`, `windFaces`, `accidentalCases`, `bridgeLanes`, `constructionStages`, `roofSurfaces`; the ordered concatenation of the units in [`En1991Units`] is [`check_full_actions`].
pub fn check_full_actions_scoped(document: &En1991Snapshot, scope: Scope<'_>) -> CheckReport {
    let annex = document.annex;
    let mut report = CheckReport::default();
    if scope.part("integrity") {
        push_referential_integrity(&mut report, document);
    }
    check_imposed_and_fire_actions(document, annex, &mut report, scope);
    check_wind_thermal_and_accidental_actions(document, annex, &mut report, scope);
    if scope.part("traffic") {
        check_traffic_and_crane_actions(document, annex, &mut report);
    }
    collection_checks::check_collection_rows(document, annex, &mut report, scope);
    wave3_checks::check_rows(document, annex, &mut report, scope);
    hazard_checks::check_rows(document, annex, &mut report, scope);
    rail_silo::check_rows(document, annex, &mut report, scope);
    wave3_rows::check_rows(document, annex, &mut report, scope);
    for (_, case) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(semio_s_artifact_norm_en1991_checks::catalogue::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), annex));
    }
    report
}

pub(crate) use rail_silo::{info_check, invalid};

/// 🧯 Design fire load q_f,d of the chosen basis, or an invalid-input check on the field the basis needs (the occupancy class for the table, a positive q_f,k for the user value) and no invented fallback.
fn fire_load_or_invalid(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport) -> Option<f64> {
    use semio_s_artifact_norm_en1991_model::FireLoadBasis;
    let qfd = part_1_2::design_fire_load_j_m2(document.fire_load_basis, document.fire_load_density_qf, annex, document.fire_occupancy, document.fire_compartment_area, 1.0);
    if qfd.is_none() {
        let clause = ClauseId::new("EN 1991-1-2", "Annex E", "E.1");
        match document.fire_load_basis {
            FireLoadBasis::OccupancyTable => invalid(report, annex, "en1991.input.fireOccupancy".into(), "DIN EN 1991-1-2", clause, "", "fireOccupancy".into(), ("occupancy class of the fire load table", "Nutzungsklasse der Brandlasttabelle"), document.fire_occupancy, part_1_2::OCCUPANCIES.iter().map(|o| o.to_string()).collect()),
            FireLoadBasis::UserSpecified => invalid(report, annex, "en1991.input.fireLoadDensityQf".into(), "DIN EN 1991-1-2", clause, "", "fireLoadDensityQf".into(), ("user-specified fire load density q_f,k (positive)", "vorgegebene Brandlastdichte q_f,k (positiv)"), &format!("{} J/m²", document.fire_load_density_qf), vec!["> 0 J/m²".into()]),
        }
    }
    qfd
}

/// 🧱️ Checks the self weight, imposed, fire and snow actions of the full actions check.
fn check_imposed_and_fire_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    let storeys = document.storey_count;

    if document.floors.is_empty() {
        if scope.part("floors-none") { report.push(CheckResult::assess("en1991.1-1.imposed.none", "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "§6.3", "6.1"), SubjectRef::whole(copy("Floors", "Geschosse")), copy("Imposed floor loads", "Nutzlasten auf Decken")).not_applicable(copy("No floor areas defined.", "Keine Deckenflächen definiert.")).annex(annex).build()); }
    }
    for (_, floor) in document.floors.iter().enumerate().filter(|(index, _)| scope.admits("floors", *index)) {
        let Some((q_kn, big_q_kn)) = part_1_1::imposed_row(&floor.category, annex) else {
            invalid(report, annex, format!("en1991.input.floors.{}.category", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Table 6.1 / 6.2", "6.3.1"), &floor.id, format!("floors[id={}].category", floor.id), ("use category", "Nutzungskategorie"), &floor.category, part_1_1::categories(annex));
            continue;
        };
        let required = kn_pa(q_kn) * part_1_1::alpha_a(&floor.category, floor.area, annex) * part_1_1::alpha_n(&floor.category, storeys, annex);
        let path = format!("floors[id={}].assumedQk", floor.id);
        let alpha_a = part_1_1::alpha_a(&floor.category, floor.area, annex);
        let alpha_n = part_1_1::alpha_n(&floor.category, storeys, annex);
        let remedy = if floor.assumed_qk + 1e-9 < required { Some(raise_pressure_remedy(&path, &floor.id, "imposed q_k", "Nutzlast q_k", floor.assumed_qk, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.imposed.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Table 6.1 / Eq. 6.1–6.2", "6.3.1"), SubjectRef::new(&floor.id, &path, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Imposed characteristic load q_k (α_A·α_n)", "Charakteristische Nutzlast q_k (α_A·α_n)"), pressure(floor.assumed_qk), pressure(required), copy(&format!("Required q_k = {:.3} kN/m² (α_A={:.3}, α_n={:.3}, A={:.1} m², n={}) for {}; assumed {:.3} kN/m².", required / 1000.0, alpha_a, alpha_n, floor.area, storeys, floor.category, floor.assumed_qk / 1000.0), &format!("Erforderliches q_k = {:.3} kN/m² (α_A={:.3}, α_n={:.3}, A={:.1} m², n={}) für {}; angenommen {:.3} kN/m².", required / 1000.0, alpha_a, alpha_n, floor.area, storeys, floor.category, floor.assumed_qk / 1000.0)), annex, remedy));
        let assumed_f = floor.assumed_qk * floor.area;
        let required_f = required * floor.area;
        let remedy_a = if floor.assumed_qk + 1e-9 < required { Some(raise_pressure_remedy(&path, &floor.id, "floor imposed q_k", "Deckennutzlast q_k", floor.assumed_qk, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.floor-force.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "§6.3.1", "6.3.1"), SubjectRef::new(&floor.id, &path, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Total imposed force q_k·A", "Gesamte Nutzlastkraft q_k·A"), force(assumed_f), force(required_f), copy(&format!("Required F = q_k·A = {:.1} kN on A={:.1} m²; assumed {:.1} kN.", required_f / 1000.0, floor.area, assumed_f / 1000.0), &format!("Erforderliches F = q_k·A = {:.1} kN auf A={:.1} m²; angenommen {:.1} kN.", required_f / 1000.0, floor.area, assumed_f / 1000.0)), annex, remedy_a));
        let req_q = kn_n(big_q_kn);
        let path_q = format!("floors[id={}].assumedQkConcentrated", floor.id);
        let remedy_q = if floor.assumed_qk_concentrated + 1e-9 < req_q { Some(raise_force_remedy(&path_q, &floor.id, "concentrated Q_k", "Einzellast Q_k", floor.assumed_qk_concentrated, req_q)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.concentrated.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Table 6.2", "6.3.1"), SubjectRef::new(&floor.id, &path_q, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Concentrated imposed load Q_k", "Charakteristische Einzellast Q_k"), force(floor.assumed_qk_concentrated), force(req_q), copy(&format!("Required Q_k = {:.2} kN; assumed {:.2} kN.", req_q / 1000.0, floor.assumed_qk_concentrated / 1000.0), &format!("Erforderliches Q_k = {:.2} kN; angenommen {:.2} kN.", req_q / 1000.0, floor.assumed_qk_concentrated / 1000.0)), annex, remedy_q));
        let req_p = part_1_1::partitions_allowance_pa(annex);
        let path_p = format!("floors[id={}].assumedPartitions", floor.id);
        let remedy_p = if floor.assumed_partitions + 1e-9 < req_p { Some(raise_pressure_remedy(&path_p, &floor.id, "partition load", "Trennwandlast", floor.assumed_partitions, req_p)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.partitions.{}", floor.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "§6.3.1.2", "6.3.1.2"), SubjectRef::new(&floor.id, &path_p, copy(&format!("Floor {}", floor.id), &format!("Decke {}", floor.id))), copy("Partition allowance", "Zuschlag für leichte Trennwände"), pressure(floor.assumed_partitions), pressure(req_p), copy(&format!("Required partition allowance = {:.2} kN/m² (lightest wall band); assumed {:.2} kN/m².", req_p / 1000.0, floor.assumed_partitions / 1000.0), &format!("Erforderlicher Trennwandzuschlag = {:.2} kN/m² (leichtestes Wandband); angenommen {:.2} kN/m².", req_p / 1000.0, floor.assumed_partitions / 1000.0)), annex, remedy_p));
    }

    if document.self_weight_elements.is_empty() {
        if scope.part("self-weight-none") { report.push(CheckResult::assess("en1991.1-1.self-weight.none", "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Annex A", "5.1"), SubjectRef::whole(copy("Self-weight", "Eigengewicht")), copy("Permanent self-weight", "Ständige Einwirkungen aus Eigengewicht")).not_applicable(copy("No self-weight elements defined.", "Keine Eigengewichtselemente definiert.")).annex(annex).build()); }
    }
    for (_, el) in document.self_weight_elements.iter().enumerate().filter(|(index, _)| scope.admits("selfWeightElements", *index)) {
        let Some(required) = part_1_1::self_weight_pa(&el.material, el.thickness, annex) else {
            invalid(report, annex, format!("en1991.input.selfWeightElements.{}.material", el.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Annex A", "5.1"), &el.id, format!("selfWeightElements[id={}].material", el.id), ("construction material", "Baustoff"), &el.material, part_1_1::MATERIALS.iter().map(|m| m.to_string()).collect());
            continue;
        };
        let path = format!("selfWeightElements[id={}].assumedGk", el.id);
        let remedy = if el.assumed_gk + 1e-9 < required { Some(raise_pressure_remedy(&path, &el.id, "self-weight g_k", "Eigengewicht g_k", el.assumed_gk, required)) } else { None };
        report.push(assess_covers(&format!("en1991.1-1.self-weight.{}", el.id), "DIN EN 1991-1-1", ClauseId::new("EN 1991-1-1", "Annex A", "5.1"), SubjectRef::new(&el.id, &path, copy(&format!("Element {}", el.id), &format!("Element {}", el.id))), copy("Permanent self-weight g_k", "Charakteristisches Eigengewicht g_k"), pressure(el.assumed_gk), pressure(required), copy(&format!("Required g_k = {:.2} kN/m² for {} @ {:.3} m; assumed {:.2} kN/m².", required / 1000.0, el.material, el.thickness, el.assumed_gk / 1000.0), &format!("Erforderliches g_k = {:.2} kN/m² für {} @ {:.3} m; angenommen {:.2} kN/m².", required / 1000.0, el.material, el.thickness, el.assumed_gk / 1000.0)), annex, remedy));
    }

    if scope.part("fire") {
        check_fire_actions(document, annex, report);
    }

    if document.roofs.is_empty() {
        if scope.part("snow") { report.push(CheckResult::assess("en1991.1-3.snow.none", "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§5.2", "5.2"), SubjectRef::whole(copy("Roofs", "Dächer")), copy("Snow on roofs", "Schneelast auf Dächern")).not_applicable(copy("No roof areas defined.", "Keine Dachflächen definiert.")).annex(annex).build()); }
    } else if let Some(sk) = part_1_3::design_ground_snow_pa(annex, &document.snow_zone, document.altitude, document.en_sk) {
        for (_, roof) in document.roofs.iter().enumerate().filter(|(index, _)| scope.admits("roofs", *index)) {
            let mu = part_1_3::shape_mu(&roof.roof_type, roof.pitch_deg, roof.has_parapet, roof.parapet_height, roof.drift_obstruction_height, roof.multi_span, sk, annex);
            let required = part_1_3::roof_snow_pa(sk, mu, roof.c_e, roof.c_t);
            let path = format!("roofs[id={}].assumedSk", roof.id);
            let remedy = if roof.assumed_sk + 1e-9 < required { Some(raise_pressure_remedy(&path, &roof.id, "snow s_k", "Schneelast s", roof.assumed_sk, required)) } else { None };
            report.push(with_stations(assess_covers(&format!("en1991.1-3.snow.{}", roof.id), "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§5.2", "5.2"), SubjectRef::new(&roof.id, &path, copy(&format!("Roof {}", roof.id), &format!("Dach {}", roof.id))), copy("Snow load on roof", "Schneelast auf dem Dach"), pressure(roof.assumed_sk), pressure(required), copy(&format!("Required s = {:.3} kN/m² (µ={mu:.2}, s_k={:.3} kN/m²); assumed {:.3} kN/m².", required / 1000.0, sk / 1000.0, roof.assumed_sk / 1000.0), &format!("Erforderliches s = {:.3} kN/m² (µ={mu:.2}, s_k={:.3} kN/m²); angenommen {:.3} kN/m².", required / 1000.0, sk / 1000.0, roof.assumed_sk / 1000.0)), annex, remedy), snow_profile_stations(roof, document.depth, mu, sk, document.annex)));
            if annex == semio_s_plugin_norm_compliance::AnnexChoice::De && document.north_german_lowland_snow {
                let s_ad = part_1_3::roof_snow_pa(part_1_3::exceptional_snow_pa(sk), mu, roof.c_e, roof.c_t);
                report.push(info_check(format!("en1991.1-3.snow-exceptional.{}", roof.id), "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§4.3 / NA", "4.3"), SubjectRef::new(&roof.id, &path, copy(&format!("Roof {}", roof.id), &format!("Dach {}", roof.id))), copy("Exceptional snow load s_Ad (North German lowland)", "Außergewöhnliche Schneelast s_Ad (Norddeutsches Tiefland)"), pressure(s_ad), copy(&format!("Exceptional design value s_Ad = μ·C_e·C_t·2.3·s_k = {:.3} kN/m².", s_ad / 1000.0), &format!("Außergewöhnlicher Bemessungswert s_Ad = μ·C_e·C_t·2,3·s_k = {:.3} kN/m².", s_ad / 1000.0)), annex));
            }
        }
    } else {
        if scope.part("snow") { invalid(report, annex, "en1991.input.snowZone".into(), "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§4.1 / NA", "4.1"), "", "snowZone".into(), ("snow load zone", "Schneelastzone"), &document.snow_zone, na_de::SnowZone::ADMISSIBLE.iter().map(|z| z.to_string()).collect()); }
    }
}

/// 🔥 Checks the thermal fire actions of the selected fire design mode.
fn check_fire_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport) {
    match document.fire_mode {
        semio_s_artifact_norm_en1991_model::FireMode::None => {
            report.push(CheckResult::assess("en1991.1-2.fire.none", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "§3", "3.1"), SubjectRef::whole(copy("Fire", "Brand")), copy("Fire thermal actions", "Thermische Einwirkungen im Brandfall")).not_applicable(copy("Fire design mode is none.", "Brandbemessungsmodus ist keiner.")).annex(annex).build());
        }
        semio_s_artifact_norm_en1991_model::FireMode::Nominal => {
            let curve = document.fire_curve;
            let Some(theta_g) = part_1_2::nominal_gas_temp_k(curve, document.fire_duration) else {
                invalid(report, annex, "en1991.input.fireCurve".into(), "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "§3.2", "3.2"), "", "fireCurve".into(), ("nominal fire curve", "nominelle Brandkurve"), "parametric", vec!["standard".into(), "external".into(), "hydrocarbon".into()]);
                return;
            };
            let path_g = "assumedGasTemperature";
            let remedy_g = if document.assumed_gas_temperature + 1e-9 < theta_g { Some(raise_temp_remedy(path_g, "", "gas temperature", "Gastemperatur", document.assumed_gas_temperature, theta_g)) } else { None };
            report.push(assess_covers("en1991.1-2.gas-temperature", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Eq. (3.4)–(3.6)", "3.2"), SubjectRef::new("", path_g, copy("Fire gas temperature", "Brandgastemperatur")), copy("Fire gas temperature θ_g", "Brandgastemperatur θ_g"), temp(document.assumed_gas_temperature), temp(theta_g), copy(&format!("Required θ_g = {:.1} K ({:.1} °C) at t = {:.0} s; assumed {:.1} K.", theta_g, theta_g - 273.15, document.fire_duration, document.assumed_gas_temperature), &format!("Erforderliches θ_g = {:.1} K ({:.1} °C) bei t = {:.0} s; angenommen {:.1} K.", theta_g, theta_g - 273.15, document.fire_duration, document.assumed_gas_temperature)), annex, remedy_g));
            push_h_net(document, annex, report, theta_g, part_1_2::alpha_c_for_curve(curve));
            let Some(qfd) = fire_load_or_invalid(document, annex, report) else {
                return;
            };
            push_fire_load(document, annex, report, qfd, None);
        }
        semio_s_artifact_norm_en1991_model::FireMode::Parametric => {
            let Some(qfd) = fire_load_or_invalid(document, annex, report) else {
                return;
            };
            let o_geom = part_1_2::compartment_opening_factor(document.fire_compartment_area, document.fire_compartment_height);
            let opening = if document.fire_opening_factor > 1e-9 { Some(document.fire_opening_factor) } else { None };
            let (theta_k, fire) = part_1_2::parametric_gas_temp_k(document.fire_duration, document.fire_compartment_area, document.fire_compartment_height, document.fire_thermal_inertia, qfd, opening, 20.0 / 60.0);
            let path_o = "fireOpeningFactor";
            let deviation = (document.fire_opening_factor - o_geom).abs();
            let remedy_o = if deviation > OPENING_TOLERANCE { Some(Remedy::exactly(SubjectRef::new("", path_o, copy("Opening factor O", "Öffnungsfaktor O")), Quantity::new(QuantityKind::Dimensionless, document.fire_opening_factor), Quantity::new(QuantityKind::Dimensionless, o_geom), copy(&format!("Set O to geometry-derived {o_geom:.4} m½ from A_f={:.1} m², H={:.2} m.", document.fire_compartment_area, document.fire_compartment_height), &format!("O auf geometriebasierten Wert {o_geom:.4} m½ aus A_f={:.1} m², H={:.2} m setzen.", document.fire_compartment_area, document.fire_compartment_height)))) } else { None };
            let mut b = CheckResult::assess("en1991.1-2.opening-factor", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex A", "A.2"), SubjectRef::new("", path_o, copy("Opening factor", "Öffnungsfaktor")), copy("Opening factor O against compartment geometry", "Öffnungsfaktor O gegen Brandabschnittsgeometrie")).utilization(Quantity::new(QuantityKind::Dimensionless, deviation), Quantity::new(QuantityKind::Dimensionless, OPENING_TOLERANCE)).warn_near_limit().annex(annex).explanation(copy(&format!("Geometry O = {o_geom:.4} (A_f={:.1} m², H={:.2} m); entered {:.4}; tolerance {OPENING_TOLERANCE}.", document.fire_compartment_area, document.fire_compartment_height, document.fire_opening_factor), &format!("Geometrie-O = {o_geom:.4} (A_f={:.1} m², H={:.2} m); eingegeben {:.4}; Toleranz {OPENING_TOLERANCE}.", document.fire_compartment_area, document.fire_compartment_height, document.fire_opening_factor)));
            if let Some(r) = remedy_o { b = b.remedy(r); }
            report.push(b.ndp_reads().build());
            let required_theta = theta_k.max(fire.theta_max_c + 273.15);
            let path_g = "assumedGasTemperature";
            let remedy_g = if document.assumed_gas_temperature + 1e-9 < required_theta { Some(raise_temp_remedy(path_g, "", "gas temperature", "Gastemperatur", document.assumed_gas_temperature, required_theta)) } else { None };
            report.push(assess_covers("en1991.1-2.gas-temperature", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex A", "A.2"), SubjectRef::new("", path_g, copy("Fire gas temperature", "Brandgastemperatur")), copy("Parametric fire gas temperature θ_g / θ_max", "Parametrische Brandgastemperatur θ_g / θ_max"), temp(document.assumed_gas_temperature), temp(required_theta), copy(&format!("Required θ = {:.1} K (t={:.0} s, O={:.4}, t_max={:.3} h {}, q_t,d={:.0} MJ/m²); assumed {:.1} K.", required_theta, document.fire_duration, fire.o, fire.t_max_h, if fire.fuel_controlled { "fuel controlled" } else { "ventilation controlled" }, fire.q_td_mj, document.assumed_gas_temperature), &format!("Erforderliches θ = {:.1} K (t={:.0} s, O={:.4}, t_max={:.3} h {}, q_t,d={:.0} MJ/m²); angenommen {:.1} K.", required_theta, document.fire_duration, fire.o, fire.t_max_h, if fire.fuel_controlled { "brennstoffkontrolliert" } else { "ventilationskontrolliert" }, fire.q_td_mj, document.assumed_gas_temperature)), annex, remedy_g));
            push_h_net(document, annex, report, required_theta, part_1_2::alpha_c_for_curve(semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Parametric));
            push_fire_load(document, annex, report, qfd, Some(fire.q_td_mj));
            let validity = (fire.q_td_mj / 1000.0).max(50.0 / fire.q_td_mj).max(fire.o / 0.20).max(0.02 / fire.o);
            let mut validity_check = CheckResult::assess("en1991.1-2.validity", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex A", "A.1"), SubjectRef::new("", path_o, copy("Parametric fire validity", "Gültigkeit der parametrischen Brandkurve")), copy("Validity range of the parametric fire (O within 0.02–0.20, q_t,d within 50–1000 MJ/m²)", "Gültigkeitsbereich der parametrischen Brandkurve (O 0,02–0,20, q_t,d 50–1000 MJ/m²)")).utilization(Quantity::new(QuantityKind::Dimensionless, validity), Quantity::new(QuantityKind::Dimensionless, 1.0)).status(semio_s_plugin_norm_compliance::CheckStatus::Pass).warn_above(1.0).annex(annex).explanation(copy(&format!("O = {:.4}, q_t,d = {:.1} MJ/m²; worst ratio to the validity limits {validity:.3}.", fire.o, fire.q_td_mj), &format!("O = {:.4}, q_t,d = {:.1} MJ/m²; ungünstigstes Verhältnis zu den Gültigkeitsgrenzen {validity:.3}.", fire.o, fire.q_td_mj)));
            if validity > 1.0 {
                let dimensionless = |value: f64| Quantity::new(QuantityKind::Dimensionless, value);
                let (subject_o, subject_q) = (SubjectRef::new("", path_o, copy("Opening factor O", "Öffnungsfaktor O")), SubjectRef::new("", "fireLoadDensityQf", copy("Fire load density q_f,k", "Brandlastdichte q_f,k")));
                validity_check = if fire.o < 0.02 || fire.o > 0.20 {
                    let target = fire.o.clamp(0.02, 0.20);
                    validity_check.remedy(Remedy::exactly(subject_o, dimensionless(fire.o), dimensionless(target), copy(&format!("Change the opening factor O from {:.4} to {target:.4} m½ (enlarge or reduce the openings) so that the parametric fire curve is valid.", fire.o), &format!("Öffnungsfaktor O von {:.4} auf {target:.4} m½ ändern (Öffnungen vergrößern oder verkleinern), damit die parametrische Brandkurve gültig ist.", fire.o))))
                } else {
                    let target = fire.q_td_mj.clamp(50.0, 1000.0);
                    validity_check.remedy(Remedy::exactly(subject_q, dimensionless(document.fire_load_density_qf), dimensionless(document.fire_load_density_qf * target / fire.q_td_mj), copy(&format!("Change the fire load so that q_t,d moves from {:.1} to {target:.1} MJ/m² (inside 50 to 1000 MJ/m²).", fire.q_td_mj), &format!("Brandlast so ändern, dass q_t,d von {:.1} auf {target:.1} MJ/m² (innerhalb 50 bis 1000 MJ/m²) wechselt.", fire.q_td_mj))))
                };
            }
            report.push(validity_check.build());
        }
    }
}

fn push_h_net(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport, theta_g: f64, alpha_c: f64) {
    let h_net = part_1_2::h_net_default_w_m2(theta_g, alpha_c);
    let path_h = "assumedHNet";
    let remedy_h = if document.assumed_h_net + 1e-9 < h_net { Some(raise_power_remedy(path_h, "", "net heat flux", "Nettowärmestrom", document.assumed_h_net, h_net)) } else { None };
    report.push(assess_covers("en1991.1-2.h-net", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Eq. (3.1)/(3.2)", "3.1"), SubjectRef::new("", path_h, copy("Net heat flux", "Nettowärmestrom")), copy("Net heat flux ĥ_net", "Nettowärmestrom ĥ_net"), Quantity::new(QuantityKind::Power, document.assumed_h_net), Quantity::new(QuantityKind::Power, h_net), copy(&format!("Required ĥ_net = {:.0} W/m² (α_c={alpha_c}); assumed {:.0} W/m².", h_net, document.assumed_h_net), &format!("Erforderliches ĥ_net = {:.0} W/m² (α_c={alpha_c}); angenommen {:.0} W/m².", h_net, document.assumed_h_net)), annex, remedy_h));
}

fn push_fire_load(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport, qfd: f64, qt_d_mj: Option<f64>) {
    use semio_s_artifact_norm_en1991_model::FireLoadBasis;
    let path_q = "assumedQfD";
    let remedy_q = if document.assumed_qf_d + 1e-9 < qfd { Some(raise_energy_remedy(path_q, "", "design fire load", "Bemessungsbrandlastdichte", document.assumed_qf_d, qfd)) } else { None };
    let tail_en = qt_d_mj.map(|q| format!(" (q_t,d={q:.0} MJ/m²)")).unwrap_or_default();
    let (source_en, source_de) = part_1_2::fire_load_source(document.fire_load_basis, document.fire_load_density_qf, annex, document.fire_occupancy).unwrap_or_default();
    let (basis_en, basis_de) = match document.fire_load_basis { FireLoadBasis::OccupancyTable => ("occupancy table", "Nutzungstabelle"), FireLoadBasis::UserSpecified => ("user-specified", "vorgegeben") };
    report.push(assess_covers_on("en1991.1-2.fire-load", "DIN EN 1991-1-2", ClauseId::new("EN 1991-1-2", "Annex E", "E.1"), SubjectRef::new("", path_q, copy("Design fire load", "Bemessungsbrandlastdichte")), copy("Design fire load density q_f,d", "Bemessungsbrandlastdichte q_f,d"), Quantity::new(QuantityKind::Energy, document.assumed_qf_d), Quantity::new(QuantityKind::Energy, qfd), copy(&format!("Basis {basis_en}: {source_en}. Required q_f,d = {:.0} MJ/m²{tail_en}; assumed {:.0} MJ/m².", qfd / 1e6, document.assumed_qf_d / 1e6), &format!("Grundlage {basis_de}: {source_de}. Erforderliches q_f,d = {:.0} MJ/m²{tail_en}; angenommen {:.0} MJ/m².", qfd / 1e6, document.assumed_qf_d / 1e6)), annex, remedy_q, &part_1_2::fire_load_ndp_ids(document.fire_load_basis, document.fire_occupancy).iter().map(String::as_str).collect::<Vec<_>>()));
}

/// 🪵️ Checks the wind, thermal, construction and accidental actions of the full actions check.
fn check_wind_thermal_and_accidental_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    check_wind_actions(document, annex, report, scope);

    if scope.part("thermal") {
    hazard_checks::check_isotherm(document, annex, report);
    match part_1_5::required_delta_t(&document.thermal_element_type, document.t_max, document.t_min, document.t_0, document.thermal_bridge_type, document.delta_t_m) {
        Some(req_dt) => {
            let path_dt = "assumedDeltaT";
            let remedy_dt = if document.assumed_delta_t + 1e-9 < req_dt { Some(raise_temp_remedy(path_dt, "", "temperature difference", "Temperaturdifferenz", document.assumed_delta_t, req_dt)) } else { None };
            report.push(assess_covers("en1991.1-5.thermal", "DIN EN 1991-1-5", ClauseId::new("EN 1991-1-5", "§5 / §6.1", "5"), SubjectRef::new("", path_dt, copy("Thermal action", "Temperatureinwirkung")), copy("Temperature difference ΔT", "Temperaturdifferenz ΔT"), temp(document.assumed_delta_t), temp(req_dt), copy(&format!("Required ΔT = {:.1} K from T_max={:.1} °C, T_min={:.1} °C, T_0={:.1} °C, ΔT_M={:.1} K ({}); assumed {:.1} K.", req_dt, document.t_max, document.t_min, document.t_0, document.delta_t_m, document.thermal_element_type, document.assumed_delta_t), &format!("Erforderliches ΔT = {:.1} K aus T_max={:.1} °C, T_min={:.1} °C, T_0={:.1} °C, ΔT_M={:.1} K ({}); angenommen {:.1} K.", req_dt, document.t_max, document.t_min, document.t_0, document.delta_t_m, document.thermal_element_type, document.assumed_delta_t)), annex, remedy_dt));
        }
        None if !part_1_5::ELEMENT_TYPES.contains(&document.thermal_element_type.as_str()) => invalid(report, annex, "en1991.input.thermalElementType".into(), "DIN EN 1991-1-5", ClauseId::new("EN 1991-1-5", "§5", "5"), "", "thermalElementType".into(), ("thermal element type", "Temperatur-Bauteiltyp"), &document.thermal_element_type, part_1_5::ELEMENT_TYPES.iter().map(|t| t.to_string()).collect()),
        None => invalid(report, annex, "en1991.input.thermalBridgeType".into(), "DIN EN 1991-1-5", ClauseId::new("EN 1991-1-5", "§6.1", "6.1"), "", "thermalBridgeType".into(), ("bridge deck type", "Brückendecktyp"), &document.thermal_bridge_type.to_string(), vec!["1".into(), "2".into(), "3".into()]),
    }
    bridge_effects_checks::check_thermal_combination(document, annex, report);

    }

    if scope.part("construction") {
    match part_1_6::construction_qk_pa(&document.construction_activity) {
        Some(req_c) => {
            let path_c = "assumedConstructionQk";
            let remedy_c = if document.assumed_construction_qk + 1e-9 < req_c { Some(raise_pressure_remedy(path_c, "", "construction q_k", "Bauzustandslast", document.assumed_construction_qk, req_c)) } else { None };
            report.push(assess_covers("en1991.1-6.construction", "DIN EN 1991-1-6", ClauseId::new("EN 1991-1-6", "§4.11", "4.11"), SubjectRef::new("", path_c, copy("Construction loads", "Einwirkungen während der Ausführung")), copy("Construction imposed load", "Nutzlast im Bauzustand"), pressure(document.assumed_construction_qk), pressure(req_c), copy(&format!("Required q_k = {:.2} kN/m² for {}; assumed {:.2} kN/m².", req_c / 1000.0, document.construction_activity, document.assumed_construction_qk / 1000.0), &format!("Erforderliches q_k = {:.2} kN/m² für {}; angenommen {:.2} kN/m².", req_c / 1000.0, document.construction_activity, document.assumed_construction_qk / 1000.0)), annex, remedy_c));
        }
        None => invalid(report, annex, "en1991.input.constructionActivity".into(), "DIN EN 1991-1-6", ClauseId::new("EN 1991-1-6", "§4.11", "4.11"), "", "constructionActivity".into(), ("construction activity", "Bauzustandstätigkeit"), &document.construction_activity, part_1_6::ACTIVITIES.iter().map(|a| a.to_string()).collect()),
    }

    }

    if document.accidental_cases.is_empty() {
        if scope.part("accidental-none") { report.push(CheckResult::assess("en1991.1-7.accidental.none", "DIN EN 1991-1-7", ClauseId::new("EN 1991-1-7", "§4", "4"), SubjectRef::whole(copy("Accidental", "Außergewöhnlich")), copy("Accidental actions", "Außergewöhnliche Einwirkungen")).not_applicable(copy("No accidental cases claimed.", "Keine außergewöhnlichen Fälle angegeben.")).annex(annex).build()); }
    }
    for (_, case) in document.accidental_cases.iter().enumerate().filter(|(index, _)| scope.admits("accidentalCases", *index)) {
        for (i, imp) in case.impact.iter().enumerate() {
            let required = part_1_7::vehicle_impact_force_n(imp.vehicle_mass, imp.vehicle_speed);
            let path = format!("accidentalCases[id={}].impact[{i}].assumedForce", case.id);
            let remedy = if imp.assumed_force + 1e-9 < required { Some(raise_force_remedy(&path, &case.id, "impact force", "Anprallkraft", imp.assumed_force, required)) } else { None };
            report.push(assess_covers(&format!("en1991.1-7.impact.{}.{i}", case.id), "DIN EN 1991-1-7", ClauseId::new("EN 1991-1-7", "Annex C", "C"), SubjectRef::new(&case.id, &path, copy(&format!("Case {}", case.id), &format!("Fall {}", case.id))), copy("Vehicle impact force F = v√(km)", "Fahrzeuganprallkraft F = v√(km)"), force(imp.assumed_force), force(required), copy(&format!("Required F = {:.1} kN (m={:.0} kg, v={:.1} m/s, k={:.0} kN/m); assumed {:.1} kN.", required / 1000.0, imp.vehicle_mass, imp.vehicle_speed, part_1_7::VEHICLE_STIFFNESS_N_M / 1000.0, imp.assumed_force / 1000.0), &format!("Erforderliches F = {:.1} kN (m={:.0} kg, v={:.1} m/s, k={:.0} kN/m); angenommen {:.1} kN.", required / 1000.0, imp.vehicle_mass, imp.vehicle_speed, part_1_7::VEHICLE_STIFFNESS_N_M / 1000.0, imp.assumed_force / 1000.0)), annex, remedy));
        }
        for (i, ex) in case.explosion.iter().enumerate() {
            let required = part_1_7::explosion_pressure_pa(ex.explosion_mass, ex.standoff);
            let path = format!("accidentalCases[id={}].explosion[{i}].assumedPressure", case.id);
            let remedy = if ex.assumed_pressure + 1e-9 < required { Some(raise_pressure_remedy(&path, &case.id, "explosion pressure", "Explosionsdruck", ex.assumed_pressure, required)) } else { None };
            report.push(assess_covers(&format!("en1991.1-7.explosion.{}.{i}", case.id), "DIN EN 1991-1-7", ClauseId::new("EN 1991-1-7", "§4.1", "4.1"), SubjectRef::new(&case.id, &path, copy(&format!("Case {}", case.id), &format!("Fall {}", case.id))), copy("Incident blast overpressure", "Einfallender Explosionsüberdruck"), pressure(ex.assumed_pressure), pressure(required), copy(&format!("Required p = {:.2} kN/m² (W={:.1} kg, R={:.1} m); assumed {:.2} kN/m².", required / 1000.0, ex.explosion_mass, ex.standoff, ex.assumed_pressure / 1000.0), &format!("Erforderliches p = {:.2} kN/m² (W={:.1} kg, R={:.1} m); angenommen {:.2} kN/m².", required / 1000.0, ex.explosion_mass, ex.standoff, ex.assumed_pressure / 1000.0)), annex, remedy));
        }
    }
}

/// 🌬️ Checks the wind pressure coefficients and net wind pressure of every wind face.
fn check_wind_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    if document.wind_faces.is_empty() {
        if scope.part("wind") { report.push(CheckResult::assess("en1991.1-4.wind.none", "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "§5.2", "5.2"), SubjectRef::whole(copy("Wind faces", "Windflächen")), copy("Wind pressure", "Winddruck")).not_applicable(copy("No wind faces defined.", "Keine Windflächen definiert.")).annex(annex).build()); }
        return;
    }
    let Some(vb) = part_1_4::design_vb(annex, document.wind_zone, document.en_vb) else {
        if scope.part("wind") { invalid(report, annex, "en1991.input.windZone".into(), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "§4.2 / NA", "4.2"), "", "windZone".into(), ("wind zone", "Windzone"), &document.wind_zone.to_string(), na_de::WindZone::ADMISSIBLE.iter().map(|z| z.to_string()).collect()); }
        return;
    };
    let terrain_options = || (0..=4).map(|t| t.to_string()).collect::<Vec<_>>();
    if part_1_4::TerrainCategory::from_u8(document.terrain_category).is_none() {
        if scope.part("wind") { invalid(report, annex, "en1991.input.terrainCategory".into(), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Table 4.1", "4.3.2"), "", "terrainCategory".into(), ("terrain category", "Geländekategorie"), &document.terrain_category.to_string(), terrain_options()); }
        return;
    }
    if document.mixed_terrain_distance > 0.0 && part_1_4::TerrainCategory::from_u8(document.mixed_terrain_upwind).is_none() {
        if scope.part("wind") { invalid(report, annex, "en1991.input.mixedTerrainUpwind".into(), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Annex A.2", "A.2"), "", "mixedTerrainUpwind".into(), ("upwind terrain category", "Geländekategorie luvseitig"), &document.mixed_terrain_upwind.to_string(), terrain_options()); }
        return;
    }
    let e_ref = part_1_4::reference_e(document.width, document.height);
    let h_over_d = document.height / document.depth.max(0.1);
    for (_, face) in document.wind_faces.iter().enumerate().filter(|(index, _)| scope.admits("windFaces", *index)) {
        let roof = document.roofs.iter().find(|r| face.id.starts_with(&r.id)).or(document.roofs.first());
        let hp_over_h = roof.filter(|r| r.has_parapet).map(|r| r.parapet_height / document.height.max(0.1)).unwrap_or(0.0);
        let hipped = roof.is_some_and(|r| part_1_4::is_hipped_roof(&r.roof_type)) && part_1_4::is_hipped_zone(&face.zone);
        let candidates = if hipped { Vec::new() } else {
            let Some(candidates) = part_1_4::face_cpe_candidates(&face.zone, h_over_d, roof.map(|r| r.roof_type.as_str()), roof.map(|r| r.pitch_deg).unwrap_or(0.0), hp_over_h) else {
                invalid(report, annex, format!("en1991.input.windFaces.{}.zone", face.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Tables 7.1–7.4", "7.2"), &face.id, format!("windFaces[id={}].zone", face.id), ("pressure zone (walls A–E, roofs F–J with a roof defined, hipped roofs F–N)", "Druckzone (Wände A–E, Dächer F–J mit definiertem Dach, Walmdächer F–N)"), &face.zone, ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"].iter().map(|z| z.to_string()).collect());
                continue;
            };
            candidates
        };
        for (suffix, user, pick, path_leaf, label) in [("cpe10", face.c_pe10, 0usize, "cPe10", "c_pe,10"), ("cpe1", face.c_pe1, 1usize, "cPe1", "c_pe,1")] {
            if hipped {
                report.push(hipped_checks::cpe_check(annex, face, suffix, user, label, path_leaf));
                continue;
            }
            let table: Vec<f64> = candidates.iter().map(|p| if pick == 0 { p.0 } else { p.1 }).collect();
            let nearest = table.iter().copied().min_by(|a, b| (a - user).abs().partial_cmp(&(b - user).abs()).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(user);
            let deviation = (user - nearest).abs();
            let path = format!("windFaces[id={}].{path_leaf}", face.id);
            let remedy = if deviation > CPE_TOLERANCE { Some(Remedy::exactly(SubjectRef::new(&face.id, &path, copy(label, label)), Quantity::new(QuantityKind::Dimensionless, user), Quantity::new(QuantityKind::Dimensionless, nearest), copy(&format!("Set {label} to the tabulated {nearest:.3} for zone {} (h/d={h_over_d:.2}, e={e_ref:.2} m).", face.zone), &format!("{label} auf den Tabellenwert {nearest:.3} für Zone {} setzen (h/d={h_over_d:.2}, e={e_ref:.2} m).", face.zone)))) } else { None };
            let mut b = CheckResult::assess(format!("en1991.1-4.{suffix}.{}", face.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Tables 7.1–7.4", "7.2"), SubjectRef::new(&face.id, &path, copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id))), copy(&format!("External pressure coefficient {label}"), &format!("Außendruckbeiwert {label}")))
                .utilization(Quantity::new(QuantityKind::Dimensionless, deviation), Quantity::new(QuantityKind::Dimensionless, CPE_TOLERANCE))
                .warn_near_limit()
                .annex(annex)
                .explanation(copy(&format!("User {label}={user:.3}; tabulated {table:.3?} for zone {} (deviation {deviation:.3}, tolerance {CPE_TOLERANCE}).", face.zone), &format!("Benutzer-{label}={user:.3}; Tabellenwerte {table:.3?} für Zone {} (Abweichung {deviation:.3}, Toleranz {CPE_TOLERANCE}).", face.zone)));
            if let Some(r) = remedy { b = b.remedy(r); }
            report.push(b.ndp_reads().build());
        }
        let ze = if face.zone.trim().eq_ignore_ascii_case("D") { part_1_4::reference_height_ze(face.z, document.height, document.width) } else { document.height };
        let Some(qp) = part_1_4::peak_velocity_pressure_pa(annex, document.wind_zone, document.terrain_category, ze, vb, document.air_density, document.orography_factor, document.mixed_terrain_upwind, document.mixed_terrain_distance, document.coast_or_island) else { continue };
        let cpe_a = part_1_4::area_cpe(face.c_pe1, face.c_pe10, face.loaded_area);
        let required = part_1_4::governing_wind_pressure_pa(qp, cpe_a, face.c_pi, face.c_s, face.c_d);
        let path = format!("windFaces[id={}].assumedWp", face.id);
        let remedy = if face.assumed_wp + 1e-9 < required { Some(raise_pressure_remedy(&path, &face.id, "wind pressure", "Winddruck", face.assumed_wp, required)) } else { None };
        let stations = wind_height_stations(document, annex, face, vb, ze, qp);
        report.push(with_stations(assess_covers(&format!("en1991.1-4.wind.{}", face.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "§5.2 / Fig. 7.2", "5.2"), SubjectRef::new(&face.id, &path, copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id))), copy("Wind pressure |w| = q_p·|c_pe − c_pi|·c_s·c_d", "Winddruck |w| = q_p·|c_pe − c_pi|·c_s·c_d"), pressure(face.assumed_wp), pressure(required), copy(&format!("Required |w| = {:.3} kN/m² (q_p={:.3}, z_e={ze:.2}, e={e_ref:.2}, signed c_pe(A={:.1})={cpe_a:.3}, c_pi in +0.2, −0.3 and {:.2}); assumed {:.3} kN/m².", required / 1000.0, qp / 1000.0, face.loaded_area, face.c_pi, face.assumed_wp / 1000.0), &format!("Erforderliches |w| = {:.3} kN/m² (q_p={:.3}, z_e={ze:.2}, e={e_ref:.2}, vorzeichenbehaftetes c_pe(A={:.1})={cpe_a:.3}, c_pi in +0,2, −0,3 und {:.2}); angenommen {:.3} kN/m².", required / 1000.0, qp / 1000.0, face.loaded_area, face.c_pi, face.assumed_wp / 1000.0)), annex, remedy), stations));
    }
}

/// 🔩️ Checks the bridge traffic, crane and silo actions of the full actions check.
fn check_traffic_and_crane_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport) {
    check_bridge_actions(document, annex, report);
    check_crane_actions(document, annex, report);
    check_silo_actions(document, annex, report);
}

fn ratio(required: f64, assumed: f64) -> f64 { if required <= 0.0 { 0.0 } else { required / assumed.max(1e-9) } }

/// 🌉 Checks the bridge traffic load models and the selected load group.
fn check_bridge_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport) {
    if document.structure_kind != semio_s_artifact_norm_en1991_model::StructureKind::Bridge {
        report.push(CheckResult::assess("en1991.2.bridge.none", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4", "4"), SubjectRef::whole(copy("Bridge", "Brücke")), copy("Bridge traffic loads", "Verkehrslasten auf Brücken")).not_applicable(copy("Structure kind is building — bridge traffic not applicable.", "Tragwerksart ist Gebäude — Brückenverkehr nicht anwendbar.")).annex(annex).build());
        return;
    }
    let (n_lanes, w_lane, w_rem) = part_2::lane_layout(document.bridge_lane_width, &document.bridge_lanes.iter().map(|l| l.width).collect::<Vec<_>>());
    let lane = document.bridge_lane.min(n_lanes).max(1);
    let lane_ok = document.bridge_lane >= 1 && document.bridge_lane <= n_lanes;
    let remedy_lane = if lane_ok { None } else { Some(Remedy::exactly(SubjectRef::new("", "bridgeLane", copy("Notional lane", "Ideeller Fahrstreifen")), Quantity::new(QuantityKind::Dimensionless, f64::from(document.bridge_lane)), Quantity::new(QuantityKind::Dimensionless, f64::from(lane)), copy(&format!("Set bridgeLane to 1..{n_lanes} for carriageway width {:.2} m (Table 4.1 → n₁={n_lanes}, w_lane={w_lane:.2} m, remaining={w_rem:.2} m).", document.bridge_lane_width), &format!("bridgeLane auf 1..{n_lanes} setzen bei Fahrbahnbreite {:.2} m (Tabelle 4.1 → n₁={n_lanes}, w_lane={w_lane:.2} m, Rest={w_rem:.2} m).", document.bridge_lane_width)))) };
    let mut lane_check = CheckResult::assess("en1991.2.notional-lanes", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.1", "4.2.3"), SubjectRef::new("", "bridgeLane", copy("Selected lane", "Gewählter Fahrstreifen")), copy("Selected lane exists in the notional lane layout", "Gewählter Fahrstreifen liegt im ideellen Fahrstreifenraster"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, if lane_ok { f64::from(document.bridge_lane) } else { f64::from(n_lanes) + 1.0 }), Quantity::new(QuantityKind::Dimensionless, f64::from(n_lanes.max(1))))
        .annex(annex)
        .explanation(copy(&format!("w={:.2} m → n₁={n_lanes}, w_lane={w_lane:.2} m, remaining={w_rem:.2} m; selected lane {}.", document.bridge_lane_width, document.bridge_lane), &format!("w={:.2} m → n₁={n_lanes}, w_lane={w_lane:.2} m, Rest={w_rem:.2} m; gewählter Streifen {}.", document.bridge_lane_width, document.bridge_lane)));
    if let Some(r) = remedy_lane { lane_check = lane_check.remedy(r); }
    report.push(lane_check.build());
    let span = document.bridge_span;
    let req_t = part_2::lm1_tandem_n(annex, lane);
    let req_m = part_2::lm1_tandem_moment_nm(annex, lane, span);
    let assumed_m = part_2::tandem_moment_nm(document.assumed_bridge_tandem, span);
    let path_t = "assumedBridgeTandem";
    let remedy_t = if assumed_m + 1e-9 < req_m { Some(raise_force_remedy(path_t, "", "LM1 tandem", "LM1-Tandem", document.assumed_bridge_tandem, req_t)) } else { None };
    report.push(assess_covers("en1991.2.lm1-tandem", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.2", "4.3.2"), SubjectRef::new("", path_t, copy("Bridge LM1 tandem", "Brücke LM1 Tandem")), copy("LM1 tandem moment, two axles α_Q·Q at 1.2 m in the worst position", "LM1-Tandemmoment, zwei Achsen α_Q·Q im Abstand 1,2 m in ungünstigster Stellung"), Quantity::new(QuantityKind::Moment, assumed_m), Quantity::new(QuantityKind::Moment, req_m), copy(&format!("Required M = {:.1} kNm (Q={:.0} kN, L={span:.1} m, lane {lane}); assumed M = {:.1} kNm.", req_m / 1000.0, req_t / 1000.0, assumed_m / 1000.0), &format!("Erforderliches M = {:.1} kNm (Q={:.0} kN, L={span:.1} m, Streifen {lane}); angenommenes M = {:.1} kNm.", req_m / 1000.0, req_t / 1000.0, assumed_m / 1000.0)), annex, remedy_t));
    let half_axle = lm1_axle_spacing_m() / 2.0;
    let tandem_x = document.bridge_tandem_x;
    let on_deck = tandem_x - half_axle >= 0.0 && tandem_x + half_axle <= span;
    let influence = |x: f64| x.min(span - x).max(0.0) / 2.0;
    let eta_sum = influence(tandem_x - half_axle) + influence(tandem_x + half_axle);
    let eta_governing = (span / 2.0).max(1e-6);
    let mut position_check = CheckResult::assess("en1991.2.lm1-tandem-position", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.2", "4.3.2"), SubjectRef::new("", "bridgeTandemX", copy("LM1 tandem position", "LM1-Tandemlage")), copy("LM1 tandem stands on the deck: mid-span moment influence Σ η of both axles against the governing mid-span position", "LM1-Tandem steht auf dem Deck: Einflussordinate Σ η beider Achsen für das Feldmoment gegenüber der maßgebenden Feldmitte"))
        .utilization(Quantity::new(QuantityKind::Length, if on_deck { eta_sum } else { span }), Quantity::new(QuantityKind::Length, eta_governing))
        .annex(annex)
        .explanation(copy(&format!("Axles at x = {:.2} m and {:.2} m on a span of {span:.2} m; Σ η = {eta_sum:.3} m of the governing {eta_governing:.3} m at mid-span.", tandem_x - half_axle, tandem_x + half_axle), &format!("Achsen bei x = {:.2} m und {:.2} m auf einer Stützweite von {span:.2} m; Σ η = {eta_sum:.3} m von maßgebend {eta_governing:.3} m in Feldmitte.", tandem_x - half_axle, tandem_x + half_axle)));
    if !on_deck {
        position_check = position_check.remedy(Remedy::exactly(SubjectRef::new("", "bridgeTandemX", copy("LM1 tandem position", "LM1-Tandemlage")), Quantity::new(QuantityKind::Length, tandem_x), Quantity::new(QuantityKind::Length, span / 2.0), copy("Move the tandem onto the deck, e.g. to mid-span.", "Tandem auf das Deck verschieben, z. B. in Feldmitte.")));
    }
    report.push(position_check.build());
    bridge_effects_checks::check_braking(document, annex, report);
    bridge_effects_checks::check_centrifugal(document, annex, report);
    if !part_2::rail_alphas().iter().any(|a| (a - document.rail_alpha).abs() < 1e-9) {
        invalid(report, annex, "en1991.input.railAlpha".into(), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§6.3.2", "6.3.2"), "", "railAlpha".into(), ("rail classification factor α", "Klassifizierungsbeiwert α der Bahnlasten"), &format!("{}", document.rail_alpha), part_2::rail_alphas().iter().map(|a| format!("{a}")).collect());
    } else {
        let alpha = document.rail_alpha;
        let rail_rows: [(&str, &str, &str, &str, &str, f64, f64, &str, &str); 4] = [
            ("en1991.2.rail.lm71-axle", "assumedRailLm71", "LM71 axle load α·Q_vk", "LM71-Achslast α·Q_vk", "§6.3.2", document.assumed_rail_lm71, part_2::rail_lm71_axle_n(alpha), "ndp.en1991.e2-14.1", "axle"),
            ("en1991.2.rail.lm71-udl", "assumedRailLm71Udl", "LM71 distributed load α·q_vk", "LM71-Streckenlast α·q_vk", "§6.3.2", document.assumed_rail_lm71_udl, part_2::rail_lm71_udl_n_m(alpha), "ndp.en1991.e2-14.1", "line"),
            ("en1991.2.rail.sw0", "assumedRailSw0", "SW/0 load α·q_vk", "SW/0-Last α·q_vk", "§6.3.3", document.assumed_rail_sw0, part_2::rail_sw0_n_m(alpha), "ndp.en1991.e2-14.2", "line"),
            ("en1991.2.rail.sw2", "assumedRailSw2", "SW/2 load q_vk", "SW/2-Last q_vk", "§6.3.3", document.assumed_rail_sw2, part_2::rail_sw2_n_m(), "ndp.en1991.e2-14.3", "line"),
        ];
        for (id, path, en, de, clause, assumed, required, ndp, kind) in rail_rows {
            let unit = if kind == "axle" { "kN" } else { "kN/m" };
            let remedy = if assumed + 1e-9 < required { Some(raise_force_remedy(path, "", en, de, assumed, required)) } else { None };
            report.push(assess_covers_on(id, "DIN EN 1991-2", ClauseId::new("EN 1991-2", clause, "6.3"), SubjectRef::new("", path, copy(en, de)), copy(en, de), force(assumed), force(required), copy(&format!("Required {:.1} {unit}; assumed {:.1} {unit} (α = {alpha}).", required / 1000.0, assumed / 1000.0), &format!("Erforderlich {:.1} {unit}; angesetzt {:.1} {unit} (α = {alpha}).", required / 1000.0, assumed / 1000.0)), annex, remedy, &[ndp]));
        }
    }
    let req_u = part_2::lm1_udl_pa(annex, lane);
    let path_u = "assumedBridgeUdl";
    let remedy_u = if document.assumed_bridge_udl + 1e-9 < req_u { Some(raise_pressure_remedy(path_u, "", "LM1 UDL", "LM1-Gleichlast", document.assumed_bridge_udl, req_u)) } else { None };
    report.push(assess_covers("en1991.2.lm1-udl", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.3.2"), SubjectRef::new("", path_u, copy("Bridge LM1 UDL", "Brücke LM1 Gleichlast")), copy("LM1 UDL q_ak on notional lane", "LM1-Gleichlast q_ak auf ideellem Fahrstreifen"), pressure(document.assumed_bridge_udl), pressure(req_u), copy(&format!("Required q_ak = {:.2} kN/m² (lane {lane}, w_lane={w_lane:.2}); assumed {:.2} kN/m².", req_u / 1000.0, document.assumed_bridge_udl / 1000.0), &format!("Erforderliches q_ak = {:.2} kN/m² (Streifen {lane}, w_lane={w_lane:.2}); angenommen {:.2} kN/m².", req_u / 1000.0, document.assumed_bridge_udl / 1000.0)), annex, remedy_u));
    if w_rem > 0.05 {
        let req_rem = part_2::remaining_area_udl_pa(annex);
        let remedy_rem = if document.assumed_bridge_udl + 1e-9 < req_rem { Some(raise_pressure_remedy(path_u, "", "LM1 UDL", "LM1-Gleichlast", document.assumed_bridge_udl, req_rem)) } else { None };
        report.push(assess_covers("en1991.2.lm1-remaining", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.3.2"), SubjectRef::new("", path_u, copy("Remaining area", "Restfläche")), copy("LM1 remaining-area UDL", "LM1-Gleichlast Restfläche"), pressure(document.assumed_bridge_udl), pressure(req_rem), copy(&format!("Remaining width {w_rem:.2} m requires q = {:.2} kN/m².", req_rem / 1000.0), &format!("Restbreite {w_rem:.2} m erfordert q = {:.2} kN/m².", req_rem / 1000.0)), annex, remedy_rem));
    }
    let req_lm2 = part_2::lm2_axle_n(annex);
    let path_lm2 = "assumedBridgeLm2";
    let remedy_lm2 = if document.assumed_bridge_lm2 + 1e-9 < req_lm2 { Some(raise_force_remedy(path_lm2, "", "LM2 axle", "LM2-Achse", document.assumed_bridge_lm2, req_lm2)) } else { None };
    report.push(assess_covers("en1991.2.lm2", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.3", "4.3.3"), SubjectRef::new("", path_lm2, copy("Bridge LM2", "Brücke LM2")), copy("LM2 single axle", "LM2-Einzelachse"), force(document.assumed_bridge_lm2), force(req_lm2), copy(&format!("Required Q = {:.0} kN; assumed {:.0} kN.", req_lm2 / 1000.0, document.assumed_bridge_lm2 / 1000.0), &format!("Erforderliches Q = {:.0} kN; angenommen {:.0} kN.", req_lm2 / 1000.0, document.assumed_bridge_lm2 / 1000.0)), annex, remedy_lm2));
    let req_fw = part_2::footway_pa(annex);
    let path_fw = "assumedBridgeFootway";
    let remedy_fw = if document.assumed_bridge_footway + 1e-9 < req_fw { Some(raise_pressure_remedy(path_fw, "", "footway load", "Gehweglast", document.assumed_bridge_footway, req_fw)) } else { None };
    report.push(assess_covers("en1991.2.footway", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§5.3", "5.3"), SubjectRef::new("", path_fw, copy("Bridge footway", "Brückengehweg")), copy("Footway / cycle-track load", "Geh- und Radweglast"), pressure(document.assumed_bridge_footway), pressure(req_fw), copy(&format!("Required q = {:.2} kN/m²; assumed {:.2} kN/m².", req_fw / 1000.0, document.assumed_bridge_footway / 1000.0), &format!("Erforderliches q = {:.2} kN/m²; angenommen {:.2} kN/m².", req_fw / 1000.0, document.assumed_bridge_footway / 1000.0)), annex, remedy_fw));
    let req_lm3 = part_2::lm3_axle_n(annex);
    let path_lm3 = "assumedBridgeLm3";
    let remedy_lm3 = if document.assumed_bridge_lm3 + 1e-9 < req_lm3 { Some(raise_force_remedy(path_lm3, "", "LM3 special vehicle", "LM3-Sonderfahrzeug", document.assumed_bridge_lm3, req_lm3)) } else { None };
    report.push(assess_covers("en1991.2.lm3", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.4 / Annex A", "4.3.4"), SubjectRef::new("", path_lm3, copy("Bridge LM3", "Brücke LM3")), copy("LM3 special vehicle heaviest axle line", "LM3-Sonderfahrzeug schwerste Achslinie"), force(document.assumed_bridge_lm3), force(req_lm3), copy(&format!("Required Q = {:.0} kN; assumed {:.0} kN.", req_lm3 / 1000.0, document.assumed_bridge_lm3 / 1000.0), &format!("Erforderliches Q = {:.0} kN; angenommen {:.0} kN.", req_lm3 / 1000.0, document.assumed_bridge_lm3 / 1000.0)), annex, remedy_lm3));
    let req_lm4 = part_2::lm4_crowd_pa(annex);
    let path_lm4 = "assumedBridgeLm4";
    let remedy_lm4 = if document.assumed_bridge_lm4 + 1e-9 < req_lm4 { Some(raise_pressure_remedy(path_lm4, "", "LM4 crowd", "LM4-Menschenmenge", document.assumed_bridge_lm4, req_lm4)) } else { None };
    report.push(assess_covers("en1991.2.lm4", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.3.5", "4.3.5"), SubjectRef::new("", path_lm4, copy("Bridge LM4", "Brücke LM4")), copy("LM4 crowd loading", "LM4-Menschenansammlung"), pressure(document.assumed_bridge_lm4), pressure(req_lm4), copy(&format!("Required q = {:.2} kN/m²; assumed {:.2} kN/m².", req_lm4 / 1000.0, document.assumed_bridge_lm4 / 1000.0), &format!("Erforderliches q = {:.2} kN/m²; angenommen {:.2} kN/m².", req_lm4 / 1000.0, document.assumed_bridge_lm4 / 1000.0)), annex, remedy_lm4));

    let selected = document.bridge_load_group.trim();
    let Some(_) = part_2::group_members(selected) else {
        invalid(report, annex, "en1991.input.bridgeLoadGroup".into(), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.4a", "4.5"), "", "bridgeLoadGroup".into(), ("load group", "Lastgruppe"), selected, part_2::GROUPS.iter().map(|g| g.to_string()).collect());
        return;
    };
    let mut scores: Vec<(&str, f64, &str)> = Vec::new();
    for group in part_2::GROUPS {
        let m = part_2::group_members(group).unwrap_or(part_2::GroupMembers { lm1: false, lm2: false, lm3: false, lm4: false, footway: false });
        let mut demand: f64 = 0.0;
        let mut worst_path = path_u;
        let mut take = |r: f64, path: &'static str| { if r > demand { demand = r; worst_path = path; } };
        if m.lm1 { take(ratio(req_t, document.assumed_bridge_tandem), path_t); take(ratio(req_u, document.assumed_bridge_udl), path_u); }
        if m.lm2 { take(ratio(req_lm2, document.assumed_bridge_lm2), path_lm2); }
        if m.lm3 { take(ratio(req_lm3, document.assumed_bridge_lm3), path_lm3); }
        if m.lm4 { take(ratio(req_lm4, document.assumed_bridge_lm4), path_lm4); }
        if m.footway { take(ratio(if group == "gr1a" { part_2::footway_reduced_pa() } else { req_fw }, document.assumed_bridge_footway), path_fw); }
        scores.push((group, demand, worst_path));
    }
    let max_d = scores.iter().map(|s| s.1).fold(0.0_f64, f64::max);
    let governors: Vec<&str> = scores.iter().filter(|s| (s.1 - max_d).abs() < 1e-6).map(|s| s.0).collect();
    let (_, selected_demand, fail_path) = scores.iter().find(|s| s.0 == selected).copied().unwrap_or((selected, 0.0, path_u));
    let governs = governors.iter().any(|g| *g == selected);
    let governing = governors.first().copied().unwrap_or("gr1a");
    let covered = semio_s_plugin_norm_compliance::within_limit(selected_demand);
    let remedy_g = if !covered {
        Some(match fail_path {
            "assumedBridgeTandem" => raise_force_remedy(path_t, "", "LM1 tandem", "LM1-Tandem", document.assumed_bridge_tandem, req_t),
            "assumedBridgeLm2" => raise_force_remedy(path_lm2, "", "LM2 axle", "LM2-Achse", document.assumed_bridge_lm2, req_lm2),
            "assumedBridgeLm3" => raise_force_remedy(path_lm3, "", "LM3 special vehicle", "LM3-Sonderfahrzeug", document.assumed_bridge_lm3, req_lm3),
            "assumedBridgeLm4" => raise_pressure_remedy(path_lm4, "", "LM4 crowd", "LM4-Menschenmenge", document.assumed_bridge_lm4, req_lm4),
            "assumedBridgeFootway" => raise_pressure_remedy(path_fw, "", "footway load", "Gehweglast", document.assumed_bridge_footway, req_fw),
            _ => raise_pressure_remedy(path_u, "", "LM1 UDL", "LM1-Gleichlast", document.assumed_bridge_udl, req_u),
        })
    } else if !governs {
        Some(Remedy::one_of(SubjectRef::new("", "bridgeLoadGroup", copy("Load group", "Lastgruppe")), governors.iter().map(|g| g.to_string()).collect(), copy(&format!("Select a governing group ({governing})."), &format!("Eine maßgebende Gruppe ({governing}) wählen."))))
    } else { None };
    let mut group_check = CheckResult::assess("en1991.2.group.selected", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.4a", "4.5"), SubjectRef::new("", "bridgeLoadGroup", copy("Load group", "Lastgruppe")), copy("Selected load group covers its constituents and governs", "Gewählte Lastgruppe deckt ihre Anteile ab und ist maßgebend"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, selected_demand), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(copy(&format!("Selected {selected}: required/assumed = {selected_demand:.3}; governing group(s) {governors:?} at {max_d:.3}."), &format!("Gewählt {selected}: erforderlich/angenommen = {selected_demand:.3}; maßgebende Gruppe(n) {governors:?} mit {max_d:.3}.")));
    if covered && !governs { group_check = group_check.status(semio_s_plugin_norm_compliance::CheckStatus::Warning); }
    if let Some(r) = remedy_g { group_check = group_check.remedy(r); }
    report.push(group_check.build());
}

/// 🏗️ Checks the crane wheel load and horizontal force.
fn check_crane_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport) {
    if !document.crane_claimed {
        report.push(CheckResult::assess("en1991.3.crane.none", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2", "2"), SubjectRef::whole(copy("Crane", "Kran")), copy("Crane loads", "Kraneinwirkungen")).not_applicable(copy("Crane runway not claimed.", "Kranbahn nicht beansprucht.")).annex(annex).build());
        return;
    }
    let (runway, base, trolley) = (document.crane_runway_length, document.crane_wheel_base, document.crane_x);
    let reach = (trolley - runway / 2.0).abs() + base / 2.0;
    let half = (runway / 2.0).max(1e-6);
    let mut position_check = CheckResult::assess("en1991.3.crane-position", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.5", "2.5"), SubjectRef::new("", "craneX", copy("Crane position", "Kranposition")), copy("Both crane wheels stand on the runway", "Beide Kranräder stehen auf der Kranbahn"))
        .utilization(Quantity::new(QuantityKind::Length, reach), Quantity::new(QuantityKind::Length, half))
        .annex(annex)
        .explanation(copy(&format!("Wheels at x = {:.2} m and {:.2} m on a runway of {runway:.2} m; reach {reach:.2} m of {half:.2} m.", trolley - base / 2.0, trolley + base / 2.0), &format!("Räder bei x = {:.2} m und {:.2} m auf einer Kranbahn von {runway:.2} m; Reichweite {reach:.2} m von {half:.2} m.", trolley - base / 2.0, trolley + base / 2.0)));
    if reach > half + 1e-9 {
        position_check = position_check.remedy(if runway > base {
            Remedy::exactly(SubjectRef::new("", "craneX", copy("Crane position", "Kranposition")), Quantity::new(QuantityKind::Length, trolley), Quantity::new(QuantityKind::Length, runway / 2.0), copy("Move the crane onto the runway, e.g. to mid-span.", "Kran auf die Kranbahn verschieben, z. B. in Feldmitte."))
        } else {
            Remedy::exactly(SubjectRef::new("", "craneRunwayLength", copy("Runway length", "Kranbahnlänge")), Quantity::new(QuantityKind::Length, runway), Quantity::new(QuantityKind::Length, 2.0 * base), copy("The runway must be longer than the wheel base.", "Die Kranbahn muss länger als der Radstand sein."))
        });
    }
    report.push(position_check.build());
    crane_silo_checks::check_crane_runway(document, annex, report);
    let classes = ["HC1", "HC2", "HC3", "HC4"].iter().map(|c| c.to_string()).collect::<Vec<_>>();
    let (Some(req_v), Some(req_h)) = (part_3::design_vertical_wheel_n(&document.crane_class, &document.hoist_class, document.hoisting_speed), part_3::design_horizontal_force_n(&document.crane_class, annex)) else {
        if part_3::crane_vertical_wheel_n(&document.crane_class).is_none() { invalid(report, annex, "en1991.input.craneClass".into(), "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.5", "2.5"), "", "craneClass".into(), ("crane class", "Kranklasse"), &document.crane_class, classes.clone()); }
        if part_3::phi_2_params(&document.hoist_class).is_none() { invalid(report, annex, "en1991.input.hoistClass".into(), "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.5", "2.5"), "", "hoistClass".into(), ("hoisting class", "Hubklasse"), &document.hoist_class, classes); }
        return;
    };
    let path_v = "assumedCraneWheel";
    let remedy_v = if document.assumed_crane_wheel + 1e-9 < req_v { Some(raise_force_remedy(path_v, "", "crane wheel", "Kranradlast", document.assumed_crane_wheel, req_v)) } else { None };
    report.push(assess_covers_on("en1991.3.crane-vertical", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.5.3", "2.5.3"), SubjectRef::new("", path_v, copy("Crane vertical", "Kran vertikal")), copy("Vertical wheel load φ₂·Q_r", "Vertikale Radlast φ₂·Q_r"), force(document.assumed_crane_wheel), force(req_v), copy(&format!("Required Q = {:.1} kN; assumed {:.1} kN.", req_v / 1000.0, document.assumed_crane_wheel / 1000.0), &format!("Erforderliches Q = {:.1} kN; angenommen {:.1} kN.", req_v / 1000.0, document.assumed_crane_wheel / 1000.0)), annex, remedy_v, &["ndp.en1991.e3-01.4"]));
    let path_h = "assumedCraneHorizontal";
    let remedy_h = if document.assumed_crane_horizontal + 1e-9 < req_h { Some(raise_force_remedy(path_h, "", "crane horizontal", "Kranhorizontallast", document.assumed_crane_horizontal, req_h)) } else { None };
    report.push(assess_covers_on("en1991.3.crane-horizontal", "DIN EN 1991-3", ClauseId::new("EN 1991-3", "§2.7", "2.7"), SubjectRef::new("", path_h, copy("Crane horizontal", "Kran horizontal")), copy("Longitudinal force H_L = φ₅·μ·Q_r", "Längskraft H_L = φ₅·μ·Q_r"), force(document.assumed_crane_horizontal), force(req_h), copy(&format!("Required H = {:.1} kN; assumed {:.1} kN.", req_h / 1000.0, document.assumed_crane_horizontal / 1000.0), &format!("Erforderliches H = {:.1} kN; angenommen {:.1} kN.", req_h / 1000.0, document.assumed_crane_horizontal / 1000.0)), annex, remedy_h, &["ndp.en1991.e3-01.4"]));
}

/// 🫙 Checks the silo wall pressures or the tank hydrostatic pressure.
fn check_silo_actions(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, report: &mut CheckReport) {
    if !document.silo_claimed {
        report.push(CheckResult::assess("en1991.4.silo.none", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§5", "5"), SubjectRef::whole(copy("Silo", "Silo")), copy("Silo / tank actions", "Silo-/Tankeinwirkungen")).not_applicable(copy("Silo/tank not claimed.", "Silo/Tank nicht beansprucht.")).annex(annex).build());
    } else if document.silo_kind == "tank" {
        let required = part_4::tank_hydrostatic_pa(document.silo_bulk_density, document.silo_height);
        let path = "assumedSiloPressure";
        let remedy = if document.assumed_silo_pressure + 1e-9 < required { Some(raise_pressure_remedy(path, "", "tank pressure", "Tankdruck", document.assumed_silo_pressure, required)) } else { None };
        report.push(assess_covers("en1991.4.tank", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§7", "7"), SubjectRef::new("", path, copy("Tank", "Tank")), copy("Tank hydrostatic pressure", "Tank-Hydrostatik"), pressure(document.assumed_silo_pressure), pressure(required), copy(&format!("Required p = {:.1} kN/m²; assumed {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0), &format!("Erforderliches p = {:.1} kN/m²; angenommen {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0)), annex, remedy));
    } else if document.silo_kind != "silo" {
        invalid(report, annex, "en1991.input.siloKind".into(), "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§1", "1"), "", "siloKind".into(), ("silo kind", "Silo-Art"), &document.silo_kind, vec!["silo".into(), "tank".into()]);
    } else {
        let diameter = document.silo_diameter;
        let hydraulic = diameter / 4.0;
        let mismatch = (document.silo_hydraulic_radius - hydraulic).abs();
        let mut radius_check = CheckResult::assess("en1991.4.silo-hydraulic-radius", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§1.5.2", "1.5"), SubjectRef::new("", "siloHydraulicRadius", copy("Silo hydraulic radius", "Hydraulischer Silo-Radius")), copy("Hydraulic radius A/U equals d_c/4 of the circular cross-section", "Hydraulischer Radius A/U entspricht d_c/4 des Kreisquerschnitts"))
            .utilization(Quantity::new(QuantityKind::Length, document.silo_hydraulic_radius), Quantity::new(QuantityKind::Length, hydraulic.max(1e-6)))
            .annex(annex)
            .explanation(copy(&format!("d_c = {diameter:.2} m gives A/U = {hydraulic:.3} m; the model uses {:.3} m.", document.silo_hydraulic_radius), &format!("d_c = {diameter:.2} m ergibt A/U = {hydraulic:.3} m; das Modell nutzt {:.3} m.", document.silo_hydraulic_radius)));
        if mismatch > 1e-6 {
            radius_check = radius_check.status(semio_s_plugin_norm_compliance::CheckStatus::Warning).remedy(Remedy::exactly(SubjectRef::new("", "siloHydraulicRadius", copy("Silo hydraulic radius", "Hydraulischer Silo-Radius")), Quantity::new(QuantityKind::Length, document.silo_hydraulic_radius), Quantity::new(QuantityKind::Length, hydraulic), copy("Set A/U to d_c/4.", "A/U auf d_c/4 setzen.")));
        }
        report.push(radius_check.build());
        let eccentricity_ratio = document.silo_eccentricity / diameter.max(1e-6);
        let inside = document.silo_eccentricity <= diameter / 2.0 + 1e-9;
        let mut eccentricity_check = CheckResult::assess("en1991.4.silo-eccentricity", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§5.2.4", "5.2.4"), SubjectRef::new("", "siloEccentricity", copy("Filling eccentricity", "Befüllungsausmitte")), copy("Filling point lies inside the cross-section", "Befüllpunkt liegt innerhalb des Querschnitts"))
            .utilization(Quantity::new(QuantityKind::Length, document.silo_eccentricity), Quantity::new(QuantityKind::Length, (diameter / 2.0).max(1e-6)))
            .annex(annex)
            .explanation(copy(&format!("e_f/d_c = {eccentricity_ratio:.3}; the eccentric-filling pressure increase is not evaluated (unverified)."), &format!("e_f/d_c = {eccentricity_ratio:.3}; die Druckerhöhung bei exzentrischer Befüllung wird nicht ausgewertet (ungeprüft).")))
            .ndp("ndp.en1991.e4-02.1");
        if !inside {
            eccentricity_check = eccentricity_check.remedy(Remedy::exactly(SubjectRef::new("", "siloEccentricity", copy("Filling eccentricity", "Befüllungsausmitte")), Quantity::new(QuantityKind::Length, document.silo_eccentricity), Quantity::new(QuantityKind::Length, diameter / 4.0), copy("Move the filling point inside the cross-section.", "Befüllpunkt in den Querschnitt verlegen.")));
        }
        report.push(eccentricity_check.build());
        let hopper_ok = document.silo_hopper_angle_deg > 0.0 && document.silo_hopper_angle_deg < 90.0;
        let mut hopper_check = CheckResult::assess("en1991.4.silo-hopper", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§6", "6"), SubjectRef::new("", "siloHopperAngleDeg", copy("Hopper wall angle", "Trichterneigung")), copy("Hopper wall angle between flat and vertical", "Trichterneigung zwischen waagerecht und senkrecht"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, document.silo_hopper_angle_deg.abs()), Quantity::new(QuantityKind::Dimensionless, 90.0))
            .annex(annex)
            .explanation(copy("Hopper pressures (steep or shallow hopper) are not evaluated; only the geometry is checked (unverified).", "Trichterdrücke (steiler oder flacher Trichter) werden nicht ausgewertet; nur die Geometrie wird geprüft (ungeprüft)."))
            .ndp("ndp.en1991.e4-02.1");
        if !hopper_ok {
            hopper_check = hopper_check.remedy(Remedy::exactly(SubjectRef::new("", "siloHopperAngleDeg", copy("Hopper wall angle", "Trichterneigung")), Quantity::new(QuantityKind::Dimensionless, document.silo_hopper_angle_deg), Quantity::new(QuantityKind::Dimensionless, 60.0), copy("Choose an angle between 0° and 90°.", "Winkel zwischen 0° und 90° wählen.")));
        }
        report.push(hopper_check.build());
        let required = part_4::horizontal_max_pa(document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k, document.silo_height);
        let path = "assumedSiloPressure";
        let remedy = if document.assumed_silo_pressure + 1e-9 < required { Some(raise_pressure_remedy(path, "", "silo pressure", "Silodruck", document.assumed_silo_pressure, required)) } else { None };
        report.push(assess_covers("en1991.4.silo-pressure", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "Eq. (5.1)", "5.2"), SubjectRef::new("", path, copy("Silo", "Silo")), copy("Janssen horizontal pressure", "Janssen-Horizontaldruck"), pressure(document.assumed_silo_pressure), pressure(required), copy(&format!("Required p_h = {:.1} kN/m²; assumed {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0), &format!("Erforderliches p_h = {:.1} kN/m²; angenommen {:.1} kN/m².", required / 1000.0, document.assumed_silo_pressure / 1000.0)), annex, remedy));
        let req_p = part_4::patch_factor() * required;
        let path_p = "assumedSiloPatch";
        let remedy_p = if document.assumed_silo_patch + 1e-9 < req_p { Some(raise_pressure_remedy(path_p, "", "silo patch", "Silo-Patch", document.assumed_silo_patch, req_p)) } else { None };
        report.push(assess_covers("en1991.4.silo-patch", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "§5.2.4", "5.2.4"), SubjectRef::new("", path_p, copy("Silo patch", "Silo Patch")), copy("Patch pressure", "Patchdruck"), pressure(document.assumed_silo_patch), pressure(req_p), copy(&format!("Required p_patch = {:.1} kN/m²; assumed {:.1} kN/m².", req_p / 1000.0, document.assumed_silo_patch / 1000.0), &format!("Erforderliches p_patch = {:.1} kN/m²; angenommen {:.1} kN/m².", req_p / 1000.0, document.assumed_silo_patch / 1000.0)), annex, remedy_p));
        let req_w = part_4::wall_friction_max_pa(document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k, document.silo_height);
        let path_w = "assumedSiloWallFriction";
        let remedy_w = if document.assumed_silo_wall_friction + 1e-9 < req_w { Some(raise_pressure_remedy(path_w, "", "wall friction", "Wandreibung", document.assumed_silo_wall_friction, req_w)) } else { None };
        report.push(assess_covers("en1991.4.silo-wall-friction", "DIN EN 1991-4", ClauseId::new("EN 1991-4", "Eq. (5.3)", "5.2"), SubjectRef::new("", path_w, copy("Silo wall friction", "Silowandreibung")), copy("Wall friction traction", "Wandreibungstraktion"), pressure(document.assumed_silo_wall_friction), pressure(req_w), copy(&format!("Required p_w = {:.1} kN/m²; assumed {:.1} kN/m².", req_w / 1000.0, document.assumed_silo_wall_friction / 1000.0), &format!("Erforderliches p_w = {:.1} kN/m²; angenommen {:.1} kN/m².", req_w / 1000.0, document.assumed_silo_wall_friction / 1000.0)), annex, remedy_w));
        crane_silo_checks::check_silo_class(document, annex, report);
        crane_silo_checks::check_silo_discharge(document, annex, report);
    }
}

pub fn evaluate(document: &En1991Snapshot) -> CheckReport {
    check_full_actions(document)
}
//#endregion 🔖️ComplianceReport

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::outline::En1991Outline;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts

#[path = "🧮️units/🦀️.rs"]
mod units;

#[path = "🧱️collection-checks/🦀️.rs"]
mod collection_checks;
#[path = "🌪️wave3-checks/🦀️.rs"]
mod wave3_checks;
#[path = "🚁hazard-checks/🦀️.rs"]
mod hazard_checks;
#[path = "🛖️hipped-checks/🦀️.rs"]
mod hipped_checks;
#[path = "🛤️bridge-effects-checks/🦀️.rs"]
mod bridge_effects_checks;
#[path = "🏭️crane-silo-checks/🦀️.rs"]
mod crane_silo_checks;
#[path = "🧷️wave3-rows/🦀️.rs"]
mod wave3_rows;
pub use units::*;

//#region 🛰️LiveEvaluation
semio_s_plugin_norm_compliance::live_report!(pub mod live(En1991Snapshot) = En1991Units);
//#endregion 🛰️LiveEvaluation


/// 📍️ Attaches stations to a built check; the peak must equal its utilisation, so the verdict never changes.
fn with_stations(mut check: CheckResult, stations: Vec<semio_s_plugin_norm_compliance::Station>) -> CheckResult {
    if !stations.is_empty() && stations.iter().all(|s| s.is_well_formed()) {
        let peak = stations.iter().map(|s| s.utilization).fold(0.0, f64::max);
        if (peak - check.utilization).abs() <= 1e-9 * check.utilization.abs().max(1.0) {
            check.stations = stations;
        }
    }
    check
}

/// ❄️ Snow load along the roof slope (s = eave 0 to ridge 1): the drift triangle behind a parapet or obstruction falls linearly from μ₂ to μ₁ over l_s (EN 1991-1-3 5.3.6), otherwise the shape coefficient is constant.
fn snow_profile_stations(roof: &semio_s_artifact_norm_en1991_model::RoofArea, depth: f64, mu_peak: f64, sk: f64, annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Vec<semio_s_plugin_norm_compliance::Station> {
    let h = if roof.has_parapet { roof.parapet_height.max(roof.drift_obstruction_height) } else { roof.drift_obstruction_height };
    let base = part_1_3::mu_1(roof.pitch_deg).min(mu_peak);
    let length = part_1_3::obstruction_drift(h, sk, annex).2;
    let util = if roof.assumed_sk > 0.0 { part_1_3::roof_snow_pa(sk, mu_peak, roof.c_e, roof.c_t) / roof.assumed_sk } else { return Vec::new() };
    if mu_peak <= 1e-12 {
        return Vec::new();
    }
    let run = (depth.max(0.1)) / 2.0;
    (0..=10)
        .map(|i| {
            let s = f64::from(i) / 10.0;
            let mu = if h > 0.0 { (mu_peak - (mu_peak - base) * (s * run / length).min(1.0)).max(base) } else { mu_peak };
            let load = part_1_3::roof_snow_pa(sk, mu, roof.c_e, roof.c_t);
            semio_s_plugin_norm_compliance::Station::new(&[s], util * mu / mu_peak, vec![pressure(load), pressure(roof.assumed_sk)])
        })
        .collect()
}

/// 🌬️ Peak velocity pressure over the height of a windward wall (zone D) as stations (s = z / z_e): the pressure follows q_p(z), the peak sits at the reference height.
fn wind_height_stations(document: &En1991Snapshot, annex: semio_s_plugin_norm_compliance::AnnexChoice, face: &semio_s_artifact_norm_en1991_model::WindFace, vb: f64, ze: f64, qp: f64) -> Vec<semio_s_plugin_norm_compliance::Station> {
    if face.zone != "D" || qp <= 0.0 || ze <= 0.0 || face.assumed_wp <= 0.0 {
        return Vec::new();
    }
    let cpe = part_1_4::area_cpe(face.c_pe1, face.c_pe10, face.loaded_area);
    (0..=10)
        .filter_map(|i| {
            let s = f64::from(i) / 10.0;
            let q = part_1_4::peak_velocity_pressure_pa(annex, document.wind_zone, document.terrain_category, s * ze, vb, document.air_density, document.orography_factor, document.mixed_terrain_upwind, document.mixed_terrain_distance, document.coast_or_island)?;
            let w = part_1_4::governing_wind_pressure_pa(q.min(qp), cpe, face.c_pi, face.c_s, face.c_d);
            Some(semio_s_plugin_norm_compliance::Station::new(&[s], w / face.assumed_wp, vec![pressure(w), pressure(face.assumed_wp)]))
        })
        .collect()
}
