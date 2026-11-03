//! 🎯️ Typed wiring of the blessed EN 1996 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use semio_s_artifact_norm_en1996_model::{MortarClass, MortarType, UnitMaterial};
use semio_s_artifact_norm_en1996_schema::checks::masonry::annex_e::standin_alpha1;
use semio_s_artifact_norm_en1996_schema::checks::masonry::numerical_models::{yield_line_pressure, Edges, MnSection};
use semio_s_artifact_norm_en1996_schema::checks::masonry::{evaluate_outcomes, f_vk0_mpa, phi_m_terms, Case, Outcome};
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_compliance::AnnexChoice;

const BAR_MODULUS_PA: f64 = 200.0e9;
const MORTAR_CLASSES: [MortarClass; 6] = [MortarClass::M1, MortarClass::M2_5, MortarClass::M5, MortarClass::M10, MortarClass::M15, MortarClass::M20];

/// 🧾️ The outcomes of one catalogue entry for a case of the EN recommended values; unusable inputs panic and become a gap of the case.
fn outcomes(entry: &str, case: Case) -> Vec<Outcome> {
    evaluate_outcomes(entry, &case).unwrap_or_else(|reason| panic!("catalogue entry {entry}: {reason}"))
}

/// 🔎️ The derived value of the catalogue outcome `key`: a limit outcome carries the resistance as capacity, an info outcome the value.
fn pick(outcomes: &[Outcome], key: &str) -> f64 {
    outcomes.iter().find(|outcome| outcome.key == key).map(|outcome| outcome.capacity).unwrap_or_else(|| panic!("catalogue outcome {key} is missing"))
}

/// 🧈️ The mortar strength [Pa] of the class whose Table 3.4 value of f_vk0 (clay units, general-purpose mortar) equals the case input.
fn mortar_strength_pa(f_vk0_pa: f64) -> f64 {
    MORTAR_CLASSES
        .into_iter()
        .find(|class| (f_vk0_mpa(AnnexChoice::En, UnitMaterial::Clay, MortarType::GeneralPurpose, *class) * 1.0e6 - f_vk0_pa).abs() < 1.0e-6)
        .map(|class| class.f_m_mpa() * 1.0e6)
        .unwrap_or_else(|| panic!("no general-purpose mortar class of Table 3.4 has f_vk0 = {f_vk0_pa} Pa"))
}

/// 🧱️ Reinforced strip in bending by plane-section strain compatibility (stress block 0.8 x, ε_mu = 3.5 permille, bilinear bar): M_Rd at N = 0 through `MnSection::m_rd` and the neutral axis x from the root of `MnSection::at`.
fn flexure_reinforced(inputs: &Inputs) -> Outputs {
    let section = MnSection { b: inputs.num("b"), depth: inputs.num("h"), f_d: inputs.num("f_d"), f_yd: inputs.num("f_yd"), e_s: BAR_MODULUS_PA, bars: vec![(inputs.num("d"), inputs.num("As"))] };
    let (mut low, mut high) = (1.0e-4 * section.depth, 60.0 * section.depth);
    for _ in 0..100 {
        let mid = 0.5 * (low + high);
        if section.at(mid).0 < 0.0 {
            low = mid;
        } else {
            high = mid;
        }
    }
    Outputs::default().with("M_Rd", section.m_rd(0.0)).with("x", 0.5 * (low + high))
}

/// 📉️ Mid-height reduction factor Φ_m of Annex G (λ = (h_ef/t)/√(E/f_k)) with its terms λ, A_1 and u through the clause function `phi_m_terms`.
fn phi_m_annex_g(inputs: &Inputs) -> Outputs {
    let t = inputs.num("t");
    let (lambda, a1, u, phi) = phi_m_terms(inputs.num("h_ef") / t, inputs.num("e_mk"), t, inputs.num("E") / inputs.num("f_k"));
    Outputs::default().with("lambda", lambda).with("A_1", a1).with("u", u).with("Phi_m", phi)
}

/// ✂️ Shear wall with a no-tension compressed zone: l_c and V_Rd through catalogue entry M-20 (shear wall), f_vk through M-03 (characteristic shear strength) at σ_d = N_Ed/(t l_c).
fn shear_wall(inputs: &Inputs) -> Outputs {
    let (length, thickness, normal, strength_pa) = (inputs.num("l"), inputs.num("t"), inputs.num("N_Ed"), inputs.num("f_b"));
    let mortar_pa = mortar_strength_pa(inputs.num("f_vk0"));
    let wall = outcomes(
        "M-20",
        Case::new("oracle-shear-wall", AnnexChoice::En)
            .with_n("l_m", length)
            .with_n("t_m", thickness)
            .with_n("n_ed_n", normal)
            .with_n("m_ed_nm", inputs.num("e") * normal)
            .with_n("v_ed_n", 0.0)
            .with_n("fb_pa", strength_pa)
            .with_n("fm_pa", mortar_pa)
            .with_n("gamma_m", inputs.num("gamma_M")),
    );
    let compressed = pick(&wall, "lc");
    let sigma = normal / (thickness * compressed);
    let characteristic = outcomes("M-03", Case::new("oracle-shear-strength", AnnexChoice::En).with_n("fb_pa", strength_pa).with_n("fm_pa", mortar_pa).with_n("sigma_pa", sigma));
    Outputs::default().with("l_c", compressed).with("sigma_d", sigma).with("f_vk", pick(&characteristic, "fvk")).with("V_Rd", pick(&wall, "v-rd"))
}

/// 🔤️ The edge conditions of a case: `top`, `bottom`, `left`, `right` as 1 simple and 2 fixed.
fn panel_edges(inputs: &Inputs) -> Edges {
    let code: String = ["top", "bottom", "left", "right"].iter().map(|side| if inputs.num(side) > 1.5 { 'X' } else { 'S' }).collect();
    Edges::parse(&code).unwrap_or_else(|| panic!("edge code {code}"))
}

/// 📐️ Johansen collapse pressure of a laterally loaded panel through `yield_line_pressure`.
fn yield_line(inputs: &Inputs) -> Outputs {
    Outputs::default().with("w_Rd", yield_line_pressure(panel_edges(inputs), inputs.num("l"), inputs.num("h"), inputs.num("m1"), inputs.num("m2")))
}

/// 📐️ Annex E stand-in coefficient alpha_1 = m1 / (w l^2) through `standin_alpha1`.
fn alpha1_standin(inputs: &Inputs) -> Outputs {
    Outputs::default().with("alpha1", standin_alpha1(panel_edges(inputs), inputs.num("mu"), inputs.num("h_over_l")))
}

/// 🗂️ One adapter per oracle check of EN 1996 that has an evaluator entry point.
static ADAPTERS: [Adapter; 5] = [
    Adapter { check: "en1996.6.6.1.flexure-reinforced", evaluator: "masonry::numerical_models::MnSection::{at, m_rd}", run: flexure_reinforced },
    Adapter { check: "en1996.G.phi-m", evaluator: "masonry::phi_m_terms", run: phi_m_annex_g },
    Adapter { check: "en1996.E.yield-line-pressure", evaluator: "masonry::numerical_models::yield_line_pressure", run: yield_line },
    Adapter { check: "en1996.E.alpha1-standin", evaluator: "masonry::annex_e::standin_alpha1", run: alpha1_standin },
    Adapter { check: "en1996.6.2.shear-wall", evaluator: "masonry catalogue M-20 and M-03", run: shear_wall },
];

/// ⚠️ No documented differences: every deviation of our evaluators from the blessed values stays a visible mismatch.
static KNOWN: [KnownDifference; 0] = [];

/// 🧮️ The EN 1996 adapter registry.
static REGISTRY: Registry = Registry { artifact: "en1996", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1996 oracle case is reproduced by our evaluators within its tolerance profile; gaps and mismatches fail the test.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1996");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
