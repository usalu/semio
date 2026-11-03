//! 🎯️ Typed wiring of the blessed EN 1990 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use semio_s_artifact_norm_en1990_checks::{combinations, en1993_cut_off_ranges, exact_failure_probability, governing, governing_610ab, k_n, miner_damage, phi_inv, test_design_value, Action, ActionClass, Context, Dist, Domain, Equation, FactorSet, PsiRow, PSI_TABLE};
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_compliance::AnnexChoice;

/// 🔎️ The ψ table category of the EN annex whose row equals `(ψ0, ψ1, ψ2)`; the oracle cases carry ψ values, our generator takes categories.
fn category_of(psi: (f64, f64, f64)) -> &'static str {
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    PSI_TABLE
        .iter()
        .find(|entry| {
            let PsiRow { psi_0, psi_1, psi_2 } = entry.row(AnnexChoice::En);
            near(psi_0, psi.0) && near(psi_1, psi.1) && near(psi_2, psi.2)
        })
        .map(|entry| entry.key)
        .expect("a ψ table row for the oracle ψ values")
}

/// 📈️ β = −Φ⁻¹(P_f) through [`phi_inv`].
fn reliability_index(inputs: &Inputs) -> Outputs {
    Outputs::default().with("beta", -phi_inv(inputs.num("pf")))
}

/// 📉️ FORM of g = R − E for two normal variables through `form_r_minus_e`.
fn form_linear(inputs: &Inputs) -> Outputs {
    let resistance = Dist::Normal { mean: inputs.num("mu_R"), sd: inputs.num("sigma_R") };
    let action = Dist::Normal { mean: inputs.num("mu_E"), sd: inputs.num("sigma_E") };
    let form = semio_s_artifact_norm_en1990_checks::form_r_minus_e(&resistance, &action);
    Outputs::default()
        .with("beta", form.beta)
        .with("alpha_R", form.alpha_r)
        .with("alpha_E", form.alpha_e)
        .with("R_d", form.design_r)
        .with("E_d", form.design_e)
        .with("P_f", form.failure_probability())
}

/// 📐️ FORM of g = R − E for a lognormal resistance and a Gumbel action through `form_r_minus_e`.
fn form_nonnormal(inputs: &Inputs) -> Outputs {
    let resistance = Dist::LogNormal { mean: inputs.num("mu_R"), sd: inputs.num("sigma_R") };
    let action = Dist::Gumbel { mean: inputs.num("mu_E"), sd: inputs.num("sigma_E") };
    let form = semio_s_artifact_norm_en1990_checks::form_r_minus_e(&resistance, &action);
    Outputs::default().with("beta", form.beta).with("R_d", form.design_r).with("E_d", form.design_e).with("beta_exact", -phi_inv(exact_failure_probability(&resistance, &action)))
}

/// 🧮️ The combination effects of Eq. 6.10, 6.10a/b, 6.14b-6.16b for permanent and variable actions of the EN building annex.
fn combination_effects(inputs: &Inputs) -> Outputs {
    let permanent = inputs.nums("G_k");
    let variable = inputs.nums("Q_k");
    let (psi_0, psi_1, psi_2) = (inputs.nums("psi_0"), inputs.nums("psi_1"), inputs.nums("psi_2"));
    let mut actions: Vec<Action> = permanent.iter().enumerate().map(|(index, value)| Action::new(format!("G{index}"), ActionClass::Permanent, *value)).collect();
    actions.extend(variable.iter().enumerate().map(|(index, value)| Action::new(format!("Q{index}"), ActionClass::Variable, *value).category(category_of((psi_0[index], psi_1[index], psi_2[index])))));
    let context = Context::new(AnnexChoice::En, Domain::Building, FactorSet::StrB, 1.0);
    let worst = |equation: Equation| governing(&combinations(&context, &actions, equation).expect("combinations"), false).expect("a governing combination").total();
    let (a, b) = (combinations(&context, &actions, Equation::E610a).expect("6.10a"), combinations(&context, &actions, Equation::E610b).expect("6.10b"));
    Outputs::default()
        .with("E_6_10", worst(Equation::E610))
        .with("E_6_10a", worst(Equation::E610a))
        .with("E_6_10b", worst(Equation::E610b))
        .with("E_d", governing_610ab(&a, &b).expect("max of 6.10a and 6.10b"))
        .with("E_char", worst(Equation::Characteristic))
        .with("E_freq", worst(Equation::Frequent))
        .with("E_qp", worst(Equation::QuasiPermanent))
}

/// 🧪️ Annex D.7.2: Table D1 k_n for V unknown and known and the characteristic value of a normal quantity through `k_n` and `test_design_value`.
fn characteristic_from_tests(inputs: &Inputs) -> Outputs {
    let (n, mean) = (inputs.count("n"), inputs.num("mean"));
    let characteristic = |known_variation: f64| test_design_value("normal", n as u32, mean, inputs.num("s"), known_variation, 1.0).expect("Table D1 design value");
    Outputs::default()
        .with("k_n_unknown", k_n(n as f64, false).expect("k_n for V unknown"))
        .with("k_n_known", k_n(n as f64, true).expect("k_n for V known"))
        .with("X_k", characteristic(0.0))
        .with("X_k_known", characteristic(inputs.num("sigma") / mean))
}

/// 🔩️ Palmgren-Miner damage D of the stress-range spectrum through `miner_damage` and the limits Δσ_D, Δσ_L through `en1993_cut_off_ranges`.
fn miner(inputs: &Inputs) -> Outputs {
    let spectrum: Vec<(f64, f64)> = inputs.nums("stress_range").into_iter().zip(inputs.nums("cycles")).collect();
    let (fatigue_limit, cut_off) = en1993_cut_off_ranges(inputs.num("detail_category"));
    Outputs::default().with("D", miner_damage(&spectrum, inputs.num("detail_category"))).with("dsigma_D", fatigue_limit).with("dsigma_L", cut_off)
}

/// 🪜️ The shared load arrangement adapter `mechanics.beam-span-envelope` (mounted from the mechanics tree).
#[path = "../../../../../../../../../../🧮️mechanics/🎯️oracle-wiring/🦀️.rs"]
#[allow(dead_code)]
mod mechanics_wiring;

/// 🗂️ One adapter per oracle check of EN 1990; every check of the blessed case files has an evaluator entry point.
static ADAPTERS: [Adapter; 7] = [
    Adapter { check: "en1990.C.beta", evaluator: "semio_s_artifact_norm_en1990_checks::phi_inv", run: reliability_index },
    Adapter { check: "en1990.C.form-linear", evaluator: "semio_s_artifact_norm_en1990_checks::form_r_minus_e", run: form_linear },
    Adapter { check: "en1990.C.form-nonnormal", evaluator: "semio_s_artifact_norm_en1990_checks::form_r_minus_e", run: form_nonnormal },
    Adapter { check: "en1990.6.4.combinations", evaluator: "semio_s_artifact_norm_en1990_checks::{combinations, governing, governing_610ab}", run: combination_effects },
    Adapter { check: "en1990.D.7.characteristic-from-tests", evaluator: "semio_s_artifact_norm_en1990_checks::{k_n, test_design_value}", run: characteristic_from_tests },
    mechanics_wiring::SPAN_ENVELOPE,
    Adapter { check: "en1990.fatigue.miner", evaluator: "semio_s_artifact_norm_en1990_checks::miner_damage", run: miner },
];

/// ⚠️ No documented OUR-vs-blessed differences remain for EN 1990: Table D1 rounding is covered by per-expectation tolerances of the case file.
static KNOWN: [KnownDifference; 0] = [];

/// 🧭️ The EN 1990 registry.
static REGISTRY: Registry = Registry { artifact: "en1990", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1990 oracle case is reproduced by our evaluators; unwired checks and quantities are reported as gaps.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1990");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
