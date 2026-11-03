//! 🌱️ Demo values of the design cases beyond the scalar examples of the vocabulary: the vector inputs, the text choices an evaluator reads without a default, and the entries whose key means something else than its vocabulary example. Seeded demo inputs of a plausible reinforced-concrete beam, not normative values. Generated from `./🔣️.json` by `🧰️scripts/en1992-v4/demo-gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::CaseValues;

struct Demo {
    entry: &'static str,
    numbers: &'static [(&'static str, f64)],
    texts: &'static [(&'static str, &'static str)],
    vectors: &'static [(&'static str, &'static [f64])],
}

const DEMOS: &[Demo] = &[
    Demo { entry: "A-01", numbers: &[("t", 18250.0)], texts: &[], vectors: &[] },
    Demo { entry: "A-02", numbers: &[("t", 18250.0)], texts: &[], vectors: &[] },
    Demo { entry: "A-04", numbers: &[], texts: &[], vectors: &[("b_i", &[1500.0, 1500.0])] },
    Demo { entry: "A-10", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87])] },
    Demo { entry: "A-13", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87])] },
    Demo { entry: "B-07", numbers: &[("t", 3.0)], texts: &[], vectors: &[] },
    Demo { entry: "BR-02", numbers: &[], texts: &[("exposure", "XC4")], vectors: &[] },
    Demo { entry: "BR-05", numbers: &[], texts: &[("exposure", "XC4")], vectors: &[] },
    Demo { entry: "D-01", numbers: &[], texts: &[("classes", "XC1,XC3")], vectors: &[] },
    Demo { entry: "D-02", numbers: &[], texts: &[("classes", "XC1,XC3")], vectors: &[] },
    Demo { entry: "D-03", numbers: &[], texts: &[("exposure", "XC4")], vectors: &[] },
    Demo { entry: "D-04", numbers: &[], texts: &[("exposure", "XC4")], vectors: &[] },
    Demo { entry: "D-06", numbers: &[], texts: &[("kind", "beam")], vectors: &[] },
    Demo { entry: "F-03", numbers: &[], texts: &[("kind", "straight")], vectors: &[] },
    Demo { entry: "F-04", numbers: &[], texts: &[("kind", "straight")], vectors: &[] },
    Demo { entry: "F-07", numbers: &[], texts: &[("kind", "straight")], vectors: &[] },
    Demo { entry: "FA-01", numbers: &[], texts: &[], vectors: &[("ax", &[-75.0, 75.0, -75.0, 75.0]), ("ay", &[-75.0, -75.0, 75.0, 75.0])] },
    Demo { entry: "FA-03", numbers: &[], texts: &[], vectors: &[("ax", &[-75.0, 75.0, -75.0, 75.0]), ("ay", &[-75.0, -75.0, 75.0, 75.0])] },
    Demo { entry: "FA-05", numbers: &[], texts: &[], vectors: &[("ax", &[-75.0, 75.0, -75.0, 75.0]), ("ay", &[-75.0, -75.0, 75.0, 75.0])] },
    Demo { entry: "FA-08", numbers: &[], texts: &[], vectors: &[("ax", &[-75.0, 75.0, -75.0, 75.0]), ("ay", &[-75.0, -75.0, 75.0, 75.0])] },
    Demo { entry: "FA-11", numbers: &[], texts: &[], vectors: &[("ax", &[-75.0, 75.0, -75.0, 75.0]), ("ay", &[-75.0, -75.0, 75.0, 75.0]), ("ni", &[5000.0, 5000.0, 5000.0, 5000.0])] },
    Demo { entry: "FA-13", numbers: &[], texts: &[], vectors: &[("ax", &[-75.0, 75.0, -75.0, 75.0]), ("ay", &[-75.0, -75.0, 75.0, 75.0])] },
    Demo { entry: "LQ-01", numbers: &[], texts: &[("exposure", "XC4")], vectors: &[] },
    Demo { entry: "LQ-05", numbers: &[], texts: &[("exposure", "XC4")], vectors: &[] },
    Demo { entry: "LQ-07", numbers: &[], texts: &[], vectors: &[("checklist", &[1.0, 1.0, 0.0, 1.0])] },
    Demo { entry: "M-04", numbers: &[("t", 28.0)], texts: &[], vectors: &[] },
    Demo { entry: "M-10", numbers: &[("t", 18250.0)], texts: &[], vectors: &[] },
    Demo { entry: "M-11", numbers: &[("t", 18250.0)], texts: &[], vectors: &[] },
    Demo { entry: "P-04", numbers: &[("asw", 5000.0)], texts: &[], vectors: &[] },
    Demo { entry: "PR-04", numbers: &[("k", 0.005)], texts: &[], vectors: &[("x", &[0.0, 5.0, 10.0]), ("theta", &[0.0, 0.1, 0.2])] },
    Demo { entry: "R-01", numbers: &[], texts: &[("kind", "beam")], vectors: &[] },
    Demo { entry: "R-04", numbers: &[], texts: &[], vectors: &[("zone_theta", &[700.0, 500.0, 350.0, 250.0, 150.0])] },
    Demo { entry: "S-01", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87])] },
    Demo { entry: "S-02", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87])] },
    Demo { entry: "S-04", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87])] },
    Demo { entry: "S-06", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87])] },
    Demo { entry: "S-09", numbers: &[("b", 1000.0), ("hw", 200.0), ("l0", 3000.0), ("e_tot", 20.0)], texts: &[], vectors: &[] },
    Demo { entry: "S-10", numbers: &[], texts: &[], vectors: &[("t_y", &[-60.0, 60.0]), ("t_z", &[-210.0, -210.0]), ("t_a", &[140.0, 140.0])] },
    Demo { entry: "S-11", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87]), ("poly_y", &[-150.0, 150.0, 150.0, -150.0]), ("poly_z", &[-250.0, -250.0, 250.0, 250.0])] },
    Demo { entry: "SG-01", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87]), ("poly_y", &[-150.0, 150.0, 150.0, -150.0]), ("poly_z", &[-250.0, -250.0, 250.0, 250.0])] },
    Demo { entry: "SG-02", numbers: &[], texts: &[], vectors: &[("bar_y", &[-110.0, 110.0, -110.0, 110.0]), ("bar_z", &[-210.0, -210.0, 210.0, 210.0]), ("bar_a", &[490.87, 490.87, 490.87, 490.87]), ("poly_y", &[-150.0, 150.0, 150.0, -150.0]), ("poly_z", &[-250.0, -250.0, 250.0, 250.0])] },
    Demo { entry: "ST-04", numbers: &[], texts: &[], vectors: &[("node_x", &[0.0, 2000.0, 1000.0]), ("node_y", &[0.0, 0.0, 1000.0]), ("member_i", &[0.0, 1.0, 2.0]), ("member_j", &[1.0, 2.0, 0.0]), ("load_x", &[0.0, 0.0, 0.0]), ("load_y", &[0.0, 0.0, -100000.0]), ("support_node", &[0.0, 1.0]), ("support_dof", &[1.0, 1.0])] },
    Demo { entry: "T-03", numbers: &[], texts: &[], vectors: &[("stiffness", &[1.0, 2.0])] },
];

/// 🌱️ The demo values of one catalogue entry on top of the vocabulary examples.
pub fn demo_values(entry_id: &str, values: CaseValues) -> CaseValues {
    let Some(demo) = DEMOS.iter().find(|demo| demo.entry == entry_id) else { return values };
    let values = demo.numbers.iter().fold(values, |values, (key, value)| values.with_number(key, *value));
    let values = demo.texts.iter().fold(values, |values, (key, value)| values.with_text(key, value));
    demo.vectors.iter().fold(values, |values, (key, items)| values.with_vector(key, items))
}
