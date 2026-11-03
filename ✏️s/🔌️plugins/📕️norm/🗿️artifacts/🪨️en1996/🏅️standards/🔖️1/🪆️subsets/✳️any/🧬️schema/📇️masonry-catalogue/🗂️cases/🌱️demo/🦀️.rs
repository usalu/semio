//! 🌱️ Demo values of the design cases beyond the vocabulary examples: the numbers, text choices and vectors an evaluator reads without a default. Seeded demo inputs of a plausible EN1996 design situation, not normative values. Generated from `./🔣️.json` by `🧰️scripts/demo-values/demo_gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::CaseValues;

struct Demo {
    entry: &'static str,
    numbers: &'static [(&'static str, f64)],
    texts: &'static [(&'static str, &'static str)],
    vectors: &'static [(&'static str, &'static [f64])],
}

const DEMOS: &[Demo] = &[
    Demo { entry: "M-44", numbers: &[("a_m2", 0.365), ("ap_m2", 0.00014), ("e_pa", 6000000000.0), ("ep_pa", 195000000000.0), ("eps_cs", 0.0003), ("fd_pa", 5957000.0), ("fpk_pa", 1860000000.0), ("fxk1_pa", 200000.0), ("n_ed_n", 300000.0), ("p0_n", 200000.0), ("phi_inf", 1.5), ("rho_relax", 0.025), ("z_m3", 0.06)], texts: &[], vectors: &[] },
    Demo { entry: "M-45", numbers: &[("diameter_m", 0.012), ("fyd_pa", 435000000.0), ("gamma_m", 1.5), ("provided_m", 1.0)], texts: &[], vectors: &[] },
    Demo { entry: "M-95", numbers: &[("k_diaphragm_n_m", 50000000.0), ("q", 2.0)], texts: &[("system", "confined")], vectors: &[] },
];

/// 🌱️ The demo values of one catalogue entry on top of the vocabulary examples.
pub fn demo_values(entry_id: &str, values: CaseValues) -> CaseValues {
    let Some(demo) = DEMOS.iter().find(|demo| demo.entry == entry_id) else { return values };
    let values = demo.numbers.iter().fold(values, |values, (key, value)| values.with_number(key, *value));
    let values = demo.texts.iter().fold(values, |values, (key, value)| values.with_text(key, value));
    demo.vectors.iter().fold(values, |values, (key, items)| values.with_vector(key, items))
}
