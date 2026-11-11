//! 🌱️ Demo values of the design cases beyond the vocabulary examples: the numbers, text choices and vectors an evaluator reads without a default. Seeded demo inputs of a plausible EN1990 design situation, not normative values. Generated from `./🔣️.json` by `🧰️scripts/demo-values/demo_gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::CaseValues;

struct Demo {
    entry: &'static str,
    numbers: &'static [(&'static str, f64)],
    texts: &'static [(&'static str, &'static str)],
    vectors: &'static [(&'static str, &'static [f64])],
}

const DEMOS: &[Demo] = &[
    Demo { entry: "E90.01", numbers: &[], texts: &[("use", "building")], vectors: &[] },
    Demo { entry: "E90.02", numbers: &[("cc", 3.0), ("gamma_f", 1.5), ("k_fi_declared", 1.1)], texts: &[], vectors: &[] },
    Demo { entry: "E90.03", numbers: &[("category", 2.0), ("years", 40.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.04", numbers: &[("years", 50.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.05", numbers: &[("dsl", 2.0), ("il", 3.0), ("rc", 3.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.06", numbers: &[], texts: &[("situations", "persistent,accidental,seismic,fatigue")], vectors: &[] },
    Demo { entry: "E90.07", numbers: &[], texts: &[("spatial", "free"), ("variation", "variable")], vectors: &[] },
    Demo { entry: "E90.08", numbers: &[("g_mean", 100.0), ("q_cov", 0.3), ("q_mean", 2.0), ("v_g", 0.1)], texts: &[], vectors: &[] },
    Demo { entry: "E90.09", numbers: &[], texts: &[("category", "B")], vectors: &[] },
    Demo { entry: "E90.10", numbers: &[("e_d", 250.0), ("eta", 0.9), ("x_k", 355.0)], texts: &[("material", "steel")], vectors: &[] },
    Demo { entry: "E90.11", numbers: &[("a_nom", 300.0), ("delta_a", -5.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.12", numbers: &[("alpha_cr", 8.5)], texts: &[("analysis", "second-order")], vectors: &[] },
    Demo { entry: "E90.13", numbers: &[("mean", 50.0), ("n", 5.0), ("v", 0.1), ("v_known", 1.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.20", numbers: &[("eta", 0.85), ("f_rep", 80.0), ("gamma_f", 1.35), ("gamma_m", 1.5), ("gamma_rd", 1.1), ("gamma_sd", 1.05), ("section", 4.0), ("x_k", 30.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.21", numbers: &[("delta_sigma_c", 71.0), ("gamma_ff", 1.0), ("gamma_mf", 1.15)], texts: &[("spectrum", "90:200000;60:2000000;40:6000000;20:50000000")], vectors: &[] },
    Demo { entry: "E90.15", numbers: &[("t_d", 0.0)], texts: &[("actions", "Gs|G|-400;Gd|G|120;Qd|Q|60|office")], vectors: &[] },
    Demo { entry: "E90.16", numbers: &[("r_d", 120.0)], texts: &[("actions", "G|G|50;Q1|Q|20|office;W|Q|10|wind;S|Q|8|snow")], vectors: &[] },
    Demo { entry: "E90.17", numbers: &[("r_d", 300.0)], texts: &[("actions", "G|G|50;Q1|Q|20|office;W|Q|10|wind;S|Q|8|snow;Ad|A|100")], vectors: &[] },
    Demo { entry: "E90.18", numbers: &[("r_d", 300.0)], texts: &[("actions", "G|G|50;Q1|Q|20|office;W|Q|10|wind;S|Q|8|snow;E1|E|40;E2|E|25")], vectors: &[] },
    Demo { entry: "E90.19", numbers: &[("cd_char", 100.0), ("cd_freq", 80.0), ("cd_qp", 60.0)], texts: &[("actions", "G|G|50;Q1|Q|20|office;W|Q|10|wind;S|Q|8|snow")], vectors: &[] },
    Demo { entry: "E90.22", numbers: &[], texts: &[("category", "G")], vectors: &[] },
    Demo { entry: "E90.24", numbers: &[("f1", 2.5), ("h_storey", 3.2), ("height", 12.0), ("span", 6.0), ("u", 30.0), ("u_storey", 5.0), ("w1", 6.0), ("w2", 3.0), ("w3", 11.25), ("w_c", 5.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.25", numbers: &[], texts: &[("category", "temperature_bridge")], vectors: &[] },
    Demo { entry: "E90.26", numbers: &[("r_d", 3000.0)], texts: &[("actions", "G|G|1000;TS|Q|300|road_traffic|gr1a;UDL|Q|150|road_udl|gr1a;SA|Q|200|road_single_axle|gr1b;W|Q|80|wind_bridge;T|Q|50|temperature_bridge")], vectors: &[] },
    Demo { entry: "E90.27", numbers: &[("acc_h", 0.3), ("acc_v", 0.5), ("crowd", 1.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.28", numbers: &[("beta", 3.7), ("rc", 2.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.29", numbers: &[("beta_t", 3.8), ("e2_k", 25.0), ("e2_mean", 20.0), ("e2_sd", 6.0), ("e_k", 65.0), ("e_mean", 60.0), ("e_sd", 12.0), ("r_k", 85.0), ("r_mean", 100.0), ("r_sd", 10.0)], texts: &[], vectors: &[] },
    Demo { entry: "E90.30", numbers: &[("eta_d", 0.9), ("gamma_m", 1.25), ("mean", 100.0), ("n", 6.0), ("v", 0.08)], texts: &[], vectors: &[] },
];

/// 🌱️ The demo values of one catalogue entry on top of the vocabulary examples.
pub fn demo_values(entry_id: &str, values: CaseValues) -> CaseValues {
    let Some(demo) = DEMOS.iter().find(|demo| demo.entry == entry_id) else { return values };
    let values = demo.numbers.iter().fold(values, |values, (key, value)| values.with_number(key, *value));
    let values = demo.texts.iter().fold(values, |values, (key, value)| values.with_text(key, value));
    demo.vectors.iter().fold(values, |values, (key, items)| values.with_vector(key, items))
}
