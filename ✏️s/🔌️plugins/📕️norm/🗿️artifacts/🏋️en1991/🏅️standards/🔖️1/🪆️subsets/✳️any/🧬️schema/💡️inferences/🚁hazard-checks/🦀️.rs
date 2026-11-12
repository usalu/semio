//! 🚁 Checks of the six hazard and execution collections: helicopter hard landings (EN 1991-1-7, 4.7), horizontal ties (Annex A), derailment (4.4), the construction loads of a stage (EN 1991-1-6, Annex A.2), falling
//! loads and execution imperfections, and the altitude-corrected isotherm reference of EN 1991-1-5 Annex A. Registry values are unverified and reported as such; mechanics-only checks carry no normative factor.

use semio_s_artifact_norm_en1991_inferences_rail_silo::{covers, first_non_positive, positive};
use semio_s_artifact_norm_en1991_checks::{copy, force_n as force, part_1_6, pressure_pa as pressure, raise_force_remedy, raise_pressure_remedy, temp_k as temp};
use semio_s_plugin_norm_compliance::{CheckReport, ClauseId, SubjectRef};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_artifact_norm_en1991_model::{ConstructionLoad, DerailmentCase, ExecutionImperfection, FallingLoad, HelicopterLanding, TieRun};
use semio_s_plugin_norm_compliance::{ndp_common, AnnexChoice};

#[path = "../🧷️wave3-rows/🦀️.rs"]
pub mod wave3_rows;

const G: f64 = 9.80665;
const KIND_INTERNAL: u8 = 1;
const LOAD_KINDS: [&str; 6] = ["Q_ca", "Q_cb", "Q_cc", "Q_cd", "Q_ce", "Q_cf"];


/// 🚁 Equivalent static force F_d = C √m (N) of a helicopter hard landing, C from the registry.
pub fn helicopter_force_n(mass_kg: f64) -> f64 { ndp_common("ndp.en1991.1-7.helicopter-c") * mass_kg.sqrt() }

/// 🪢 Required tie force (N): factor·(g_k + ψ q_k)·s·L limited by the registry cap, the lesser value governs.
pub fn tie_force_n(kind: u8, g_pa: f64, q_pa: f64, psi: f64, s_m: f64, l_m: f64) -> (f64, f64) {
    let factor = ndp_common(if kind == KIND_INTERNAL { "ndp.en1991.1-7.tie-internal" } else { "ndp.en1991.1-7.tie-peripheral" });
    let t = factor * (g_pa + psi * q_pa) * s_m * l_m;
    (t, t.min(ndp_common("ndp.en1991.1-7.tie-cap")))
}

/// 🧱 Combined construction load (Pa) of a stage: ψ₀ Σ Q_c,i over its loads.
pub fn stage_load_pa(document: &En1991Snapshot, stage: &str) -> f64 { part_1_6::combined_construction_load_pa(document.construction_loads.iter().filter(|l| l.stage == stage).map(|l| l.magnitude)) }

/// 🌡️ Altitude-corrected isotherm references (T_max, T_min) in °C from the registry base values and the lapse per 100 m.
pub fn isotherm_reference(altitude_m: f64) -> (f64, f64) {
    let lapse = ndp_common("ndp.en1991.1-5.isotherm-lapse") * altitude_m.max(0.0) / 100.0;
    (ndp_common("ndp.en1991.1-5.isotherm-t-max-base") - lapse, ndp_common("ndp.en1991.1-5.isotherm-t-min-base") - lapse)
}

/// 🚁 The row checks of the six collections and the isotherm reference, in the order of the units of [`En1991Units`].
pub fn check_rows(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, row) in document.helicopter_landings.iter().enumerate().filter(|(i, _)| scope.admits("helicopterLandings", *i)) { check_helicopter(annex, report, row); }
    for (_, row) in document.tie_runs.iter().enumerate().filter(|(i, _)| scope.admits("tieRuns", *i)) { check_tie(annex, report, row); }
    for (index, row) in document.construction_loads.iter().enumerate().filter(|(i, _)| scope.admits("constructionLoads", *i)) { check_construction_load(document, annex, report, index, row); }
    for (_, row) in document.falling_loads.iter().enumerate().filter(|(i, _)| scope.admits("fallingLoads", *i)) { check_falling(annex, report, row); }
    for (_, row) in document.execution_imperfections.iter().enumerate().filter(|(i, _)| scope.admits("executionImperfections", *i)) { check_imperfection(annex, report, row); }
    for (_, row) in document.derailment_cases.iter().enumerate().filter(|(i, _)| scope.admits("derailmentCases", *i)) { check_derailment(annex, report, row); }
}

/// 🌡️ Informative: the entered T_max and T_min against the altitude-corrected isotherm reference (the national maps are not reproduced).
pub fn check_isotherm(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    let (t_max, t_min) = isotherm_reference(document.altitude);
    let subject = SubjectRef::new("thermal", "tMax", copy("Isotherm reference", "Isothermen-Referenz"));
    let margin = (document.t_max - t_max).min(t_min - document.t_min);
    let severity = if margin + 1e-9 >= 0.0 { ("at least as severe as", "mindestens so ungünstig wie") } else { ("less severe than", "weniger ungünstig als") };
    report.push(info_check("en1991.1-5.isotherm".into(), "DIN EN 1991-1-5", ClauseId::new("EN 1991-1-5", "Annex A", "A"), subject, copy("Entered T_max, T_min against the altitude-corrected isotherm", "Eingegebene T_max, T_min gegen die höhenkorrigierte Isotherme"), temp(margin.abs()), copy(&format!("Reference at {:.0} m: T_max = {t_max:.1} °C, T_min = {t_min:.1} °C; entered {:.1} / {:.1} °C are {} the reference. The national isotherm maps are not reproduced, the base values and the lapse are unverified.", document.altitude, document.t_max, document.t_min, severity.0), &format!("Referenz auf {:.0} m: T_max = {t_max:.1} °C, T_min = {t_min:.1} °C; eingegeben {:.1} / {:.1} °C sind {} die Referenz. Die nationalen Isothermenkarten sind nicht wiedergegeben, Grundwerte und Höhengradient sind ungeprüft.", document.altitude, document.t_max, document.t_min, severity.1)), annex));
}

/// 🚁 One helicopter landing: F_d = C √m against the assumed equivalent force.
fn check_helicopter(annex: AnnexChoice, report: &mut CheckReport, row: &HelicopterLanding) {
    let base = format!("helicopterLandings[id={}]", row.id);
    let subject = SubjectRef::new(&row.id, &format!("{base}.assumedForce"), copy(&format!("Helicopter landing {}", row.id), &format!("Hubschrauberlandung {}", row.id)));
    let clause = ClauseId::new("EN 1991-1-7", "§4.7", "4.7");
    if let Some((leaf, value, label)) = first_non_positive(&[("mass", row.mass, ("take-off mass", "Startmasse"))]) {
        invalid(report, annex, format!("en1991.input.helicopterLandings.{}.{leaf}", row.id), "DIN EN 1991-1-7", clause, &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
        return;
    }
    let required = helicopter_force_n(row.mass);
    let remedy = (row.assumed_force + 1e-9 < required).then(|| raise_force_remedy(&format!("{base}.assumedForce"), &row.id, "helicopter landing force", "Hubschrauber-Landekraft", row.assumed_force, required));
    report.push(covers(&format!("en1991.1-7.helicopter-force.{}", row.id), "DIN EN 1991-1-7", clause, subject, copy("Helicopter hard landing F_d = C √m", "Hubschrauber-Hartlandung F_d = C √m"), force(row.assumed_force), force(required), copy(&format!("m = {:.0} kg: F_d = {:.1} kN; assumed {:.1} kN.", row.mass, required / 1000.0, row.assumed_force / 1000.0), &format!("m = {:.0} kg: F_d = {:.1} kN; angenommen {:.1} kN.", row.mass, required / 1000.0, row.assumed_force / 1000.0)), annex, remedy));
}

/// 🪢 One tie: the required peripheral or internal tie force against the provided force.
fn check_tie(annex: AnnexChoice, report: &mut CheckReport, row: &TieRun) {
    let base = format!("tieRuns[id={}]", row.id);
    let subject = SubjectRef::new(&row.id, &format!("{base}.providedForce"), copy(&format!("Tie {}", row.id), &format!("Zugglied {}", row.id)));
    let clause = ClauseId::new("EN 1991-1-7", "Annex A.5", "A");
    if row.kind > 1 {
        invalid(report, annex, format!("en1991.input.tieRuns.{}.kind", row.id), "DIN EN 1991-1-7", clause, &row.id, format!("{base}.kind"), ("tie kind (0 peripheral, 1 internal)", "Zuggliedart (0 Rand, 1 innen)"), &row.kind.to_string(), vec!["0".into(), "1".into()]);
        return;
    }
    if let Some((leaf, value, label)) = first_non_positive(&[("spacing", row.spacing, ("tie spacing", "Zuggliedabstand")), ("span", row.span, ("tie span", "Zuggliedlänge"))]) {
        invalid(report, annex, format!("en1991.input.tieRuns.{}.{leaf}", row.id), "DIN EN 1991-1-7", clause, &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
        return;
    }
    if !(row.psi.is_finite() && (0.0..=1.0).contains(&row.psi)) {
        invalid(report, annex, format!("en1991.input.tieRuns.{}.psi", row.id), "DIN EN 1991-1-7", clause, &row.id, format!("{base}.psi"), ("combination factor psi", "Kombinationsbeiwert psi"), &format!("{}", row.psi), vec!["0".into(), "0.3".into(), "0.5".into()]);
        return;
    }
    let (uncapped, required) = tie_force_n(row.kind, row.dead_load, row.imposed_load, row.psi, row.spacing, row.span);
    let remedy = (row.provided_force + 1e-9 < required).then(|| raise_force_remedy(&format!("{base}.providedForce"), &row.id, "provided tie force", "vorhandene Zugkraft", row.provided_force, required));
    let (kind_en, kind_de) = if row.kind == KIND_INTERNAL { ("Internal tie T_i", "Innenzugglied T_i") } else { ("Peripheral tie T_p", "Randzugglied T_p") };
    report.push(covers(&format!("en1991.1-7.tie-force.{}", row.id), "DIN EN 1991-1-7", clause, subject, copy(kind_en, kind_de), force(row.provided_force), force(required), copy(&format!("T = {:.1} kN, governing (lesser with the cap) {:.1} kN; provided {:.1} kN.", uncapped / 1000.0, required / 1000.0, row.provided_force / 1000.0), &format!("T = {:.1} kN, maßgebend (kleinerer Wert mit Begrenzung) {:.1} kN; vorhanden {:.1} kN.", uncapped / 1000.0, required / 1000.0, row.provided_force / 1000.0)), annex, remedy));
}

/// 🧱 One construction load: its kind and stage are known; the first load of a stage checks the combined load Σ Q_c,i against the stage's assumed load.
fn check_construction_load(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, index: usize, row: &ConstructionLoad) {
    let base = format!("constructionLoads[id={}]", row.id);
    let clause = ClauseId::new("EN 1991-1-6", "Annex A.2", "A.2");
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Construction load {}", row.id), &format!("Bauzustandslast {}", row.id)));
    if usize::from(row.kind) >= LOAD_KINDS.len() {
        invalid(report, annex, format!("en1991.input.constructionLoads.{}.kind", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.kind"), ("construction load kind", "Bauzustandslastart"), &row.kind.to_string(), (0..LOAD_KINDS.len()).map(|k| k.to_string()).collect());
        return;
    }
    let Some(stage) = document.construction_stages.iter().find(|s| s.id == row.stage) else {
        invalid(report, annex, format!("en1991.input.constructionLoads.{}.stage", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.stage"), ("construction stage id", "Bauzustand-Id"), &row.stage, document.construction_stages.iter().map(|s| s.id.clone()).collect());
        return;
    };
    if !(row.magnitude.is_finite() && row.magnitude >= 0.0) {
        invalid(report, annex, format!("en1991.input.constructionLoads.{}.magnitude", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.magnitude"), ("construction load", "Bauzustandslast"), &format!("{}", row.magnitude), positive());
        return;
    }
    if document.construction_loads.iter().position(|l| l.stage == row.stage) != Some(index) { return; }
    let required = stage_load_pa(document, &row.stage);
    let list = document.construction_loads.iter().filter(|l| l.stage == row.stage).map(|l| format!("{} {:.2}", LOAD_KINDS.get(usize::from(l.kind)).copied().unwrap_or("?"), l.magnitude / 1000.0)).collect::<Vec<_>>().join(" + ");
    let remedy = (stage.assumed_qk + 1e-9 < required).then(|| raise_pressure_remedy(&format!("constructionStages[id={}].assumedQk", stage.id), &stage.id, "stage construction load", "Bauzustandslast", stage.assumed_qk, required));
    report.push(covers(&format!("en1991.1-6.stage-combination.{}", stage.id), "DIN EN 1991-1-6", clause, subject("magnitude"), copy("Combined construction load Q_c = Σ Q_c,i", "Kombinierte Bauzustandslast Q_c = Σ Q_c,i"), pressure(stage.assumed_qk), pressure(required), copy(&format!("Stage {}: {list} kN/m² with ψ₀ = {:.1}: {:.2} kN/m²; assumed {:.2} kN/m².", stage.id, ndp_common("ndp.en1991.1-6.combination-psi0"), required / 1000.0, stage.assumed_qk / 1000.0), &format!("Bauzustand {}: {list} kN/m² mit ψ₀ = {:.1}: {:.2} kN/m²; angenommen {:.2} kN/m².", stage.id, ndp_common("ndp.en1991.1-6.combination-psi0"), required / 1000.0, stage.assumed_qk / 1000.0)), annex, remedy));
}

/// 🪨 One falling load: the mean arrest force of the energy balance against the assumed force.
fn check_falling(annex: AnnexChoice, report: &mut CheckReport, row: &FallingLoad) {
    let base = format!("fallingLoads[id={}]", row.id);
    let clause = ClauseId::new("EN 1991-1-6", "§4.12", "4.12");
    let subject = SubjectRef::new(&row.id, &format!("{base}.assumedForce"), copy(&format!("Falling load {}", row.id), &format!("Herabfallende Last {}", row.id)));
    if let Some((leaf, value, label)) = first_non_positive(&[("mass", row.mass, ("falling mass", "fallende Masse")), ("stoppingDistance", row.stopping_distance, ("stopping distance", "Bremsweg"))]) {
        invalid(report, annex, format!("en1991.input.fallingLoads.{}.{leaf}", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
        return;
    }
    if !(row.drop_height.is_finite() && row.drop_height >= 0.0) {
        invalid(report, annex, format!("en1991.input.fallingLoads.{}.dropHeight", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.dropHeight"), ("drop height", "Fallhöhe"), &format!("{}", row.drop_height), positive());
        return;
    }
    let required = part_1_6::falling_arrest_force_n(row.mass, row.drop_height, row.stopping_distance);
    let remedy = (row.assumed_force + 1e-9 < required).then(|| raise_force_remedy(&format!("{base}.assumedForce"), &row.id, "falling load force", "Kraft der herabfallenden Last", row.assumed_force, required));
    report.push(covers(&format!("en1991.1-6.falling-load.{}", row.id), "DIN EN 1991-1-6", clause, subject, copy("Falling load, mean arrest force m g (h + s)/s", "Herabfallende Last, mittlere Bremskraft m g (h + s)/s"), force(row.assumed_force), force(required), copy(&format!("m = {:.0} kg falls {:.2} m and stops over {:.3} m: E = {:.1} kJ, F = {:.1} kN; assumed {:.1} kN. Energy balance only, the accidental design value is a project decision.", row.mass, row.drop_height, row.stopping_distance, row.mass * G * row.drop_height / 1000.0, required / 1000.0, row.assumed_force / 1000.0), &format!("m = {:.0} kg fällt {:.2} m und wird auf {:.3} m gebremst: E = {:.1} kJ, F = {:.1} kN; angenommen {:.1} kN. Nur Energiebilanz, der Bemessungswert der außergewöhnlichen Einwirkung ist eine Projektentscheidung.", row.mass, row.drop_height, row.stopping_distance, row.mass * G * row.drop_height / 1000.0, required / 1000.0, row.assumed_force / 1000.0)), annex, remedy));
}

/// 📐 One execution imperfection: the equivalent horizontal force N Δ / h against the assumed force.
fn check_imperfection(annex: AnnexChoice, report: &mut CheckReport, row: &ExecutionImperfection) {
    let base = format!("executionImperfections[id={}]", row.id);
    let clause = ClauseId::new("EN 1991-1-6", "§4.9", "4.9");
    let subject = SubjectRef::new(&row.id, &format!("{base}.assumedForce"), copy(&format!("Execution imperfection {}", row.id), &format!("Ausführungsimperfektion {}", row.id)));
    if let Some((leaf, value, label)) = first_non_positive(&[("height", row.height, ("member height", "Bauteilhöhe"))]) {
        invalid(report, annex, format!("en1991.input.executionImperfections.{}.{leaf}", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
        return;
    }
    for (leaf, value, label) in [("tolerance", row.tolerance, ("tolerance", "Toleranz")), ("verticalLoad", row.vertical_load, ("vertical load", "Vertikallast"))] {
        if !(value.is_finite() && value >= 0.0) {
            invalid(report, annex, format!("en1991.input.executionImperfections.{}.{leaf}", row.id), "DIN EN 1991-1-6", clause, &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
            return;
        }
    }
    let required = part_1_6::imperfection_force_n(row.vertical_load, row.tolerance, row.height);
    let remedy = (row.assumed_force + 1e-9 < required).then(|| raise_force_remedy(&format!("{base}.assumedForce"), &row.id, "equivalent horizontal force", "horizontale Ersatzkraft", row.assumed_force, required));
    report.push(covers(&format!("en1991.1-6.execution-imperfection.{}", row.id), "DIN EN 1991-1-6", clause, subject, copy("Equivalent horizontal force H = N Δ/h", "Horizontale Ersatzkraft H = N Δ/h"), force(row.assumed_force), force(required), copy(&format!("Inclination Δ/h = {:.4}, N = {:.1} kN: H = {:.2} kN; assumed {:.2} kN. Statics only, tolerance limits come from the execution standards.", row.tolerance / row.height, row.vertical_load / 1000.0, required / 1000.0, row.assumed_force / 1000.0), &format!("Neigung Δ/h = {:.4}, N = {:.1} kN: H = {:.2} kN; angenommen {:.2} kN. Nur Statik, Toleranzgrenzen stammen aus den Ausführungsnormen.", row.tolerance / row.height, row.vertical_load / 1000.0, required / 1000.0, row.assumed_force / 1000.0)), annex, remedy));
}

/// 🚆 One derailment case: the assumed resistance covers the entered design force F_dx; the transverse force is informative.
fn check_derailment(annex: AnnexChoice, report: &mut CheckReport, row: &DerailmentCase) {
    let base = format!("derailmentCases[id={}]", row.id);
    let clause = ClauseId::new("EN 1991-1-7", "§4.4 Tables 4.4-4.5", "4.4");
    let subject = SubjectRef::new(&row.id, &format!("{base}.assumedForce"), copy(&format!("Derailment case {}", row.id), &format!("Entgleisungsfall {}", row.id)));
    if let Some((leaf, value, label)) = first_non_positive(&[("designForceX", row.design_force_x, ("design force F_dx from the applicable table", "Bemessungskraft F_dx aus der maßgebenden Tabelle"))]) {
        invalid(report, annex, format!("en1991.input.derailmentCases.{}.{leaf}", row.id), "DIN EN 1991-1-7", clause, &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
        return;
    }
    if !(row.distance.is_finite() && row.distance >= 0.0 && row.design_force_y.is_finite() && row.design_force_y >= 0.0) {
        invalid(report, annex, format!("en1991.input.derailmentCases.{}.distance", row.id), "DIN EN 1991-1-7", clause, &row.id, format!("{base}.distance"), ("distance from the track axis", "Abstand von der Gleisachse"), &format!("{}", row.distance), positive());
        return;
    }
    let _ = ndp_common("ndp.en1991.1-7.derailment-table");
    let remedy = (row.assumed_force + 1e-9 < row.design_force_x).then(|| raise_force_remedy(&format!("{base}.assumedForce"), &row.id, "derailment resistance", "Entgleisungswiderstand", row.assumed_force, row.design_force_x));
    report.push(covers(&format!("en1991.1-7.derailment-force.{}", row.id), "DIN EN 1991-1-7", clause, subject, copy("Derailment force F_dx against the assumed resistance", "Entgleisungskraft F_dx gegen den angenommenen Widerstand"), force(row.assumed_force), force(row.design_force_x), copy(&format!("At d = {:.1} m from the track axis: entered F_dx = {:.0} kN, F_dy = {:.0} kN; assumed {:.0} kN. The tables 4.4-4.5 are not reproduced, the forces are entered and unverified.", row.distance, row.design_force_x / 1000.0, row.design_force_y / 1000.0, row.assumed_force / 1000.0), &format!("Bei d = {:.1} m von der Gleisachse: eingegeben F_dx = {:.0} kN, F_dy = {:.0} kN; angenommen {:.0} kN. Die Tabellen 4.4-4.5 sind nicht wiedergegeben, die Kräfte werden eingegeben und sind ungeprüft.", row.distance, row.design_force_x / 1000.0, row.design_force_y / 1000.0, row.assumed_force / 1000.0)), annex, remedy));
}
