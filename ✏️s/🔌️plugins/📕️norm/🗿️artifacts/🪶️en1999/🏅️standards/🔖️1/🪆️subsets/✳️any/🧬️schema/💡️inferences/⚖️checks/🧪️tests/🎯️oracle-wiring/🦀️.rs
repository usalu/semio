//! 🎯️ Typed wiring of the blessed EN 1999 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use crate::joints::pin_bending_moment;
use crate::part_1_1::{angle_constants, channel_constants, chi_from_lambda, elliptical_constants, tee_constants, lambda_bar, m_cr_ltb, n_cr, plastic_moduli, section_geometry};
use crate::polygon_props;
use semio_s_plugin_norm_mechanics::{analyse_section, Material, SectionConstants};
use semio_s_artifact_norm_en1999_model::AluminiumSection;
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};

/// 📐️ The extruded section of a kind with the given plate sizes; unused sizes are zero.
fn section(kind: &str, height: f64, width: f64, flange: f64, web: f64, diameter: f64) -> AluminiumSection {
    AluminiumSection { id: String::new(), kind: match kind { "tube" | "chs" => semio_s_artifact_norm_en1999_model::SectionKind::Chs, "rhs" => semio_s_artifact_norm_en1999_model::SectionKind::Rhs, "box" => semio_s_artifact_norm_en1999_model::SectionKind::Box, "angle" => semio_s_artifact_norm_en1999_model::SectionKind::Angle, "channel" => semio_s_artifact_norm_en1999_model::SectionKind::Channel, "free" => semio_s_artifact_norm_en1999_model::SectionKind::Free, _ => semio_s_artifact_norm_en1999_model::SectionKind::ExtrudedI }, height, width, flange_thickness: flange, web_thickness: web, outer_diameter: diameter, elements: Vec::new() }
}

/// 📐️ The doubly symmetric I-section of a case that carries top and bottom flanges; unequal flanges have no analytic section.
fn extruded_i(inputs: &Inputs) -> AluminiumSection {
    assert!((inputs.num("b_t") - inputs.num("b_b")).abs() < 1e-12 && (inputs.num("t_ft") - inputs.num("t_fb")).abs() < 1e-12, "the analytic I-section has equal flanges");
    section("extrudedI", inputs.num("h"), inputs.num("b_t"), inputs.num("t_ft"), inputs.num("tw"), 0.0)
}

/// 🅸️ I-section constants through `section_geometry` and `plastic_moduli`.
fn section_i(inputs: &Inputs) -> Outputs {
    let s = extruded_i(inputs);
    let (a, iy, iz, _, _, it, iw) = section_geometry(&s);
    Outputs::default().with("A", a).with("Iy", iy).with("Iz", iz).with("It", it).with("Iw", iw).with("Wpl_y", plastic_moduli(&s).0)
}

/// 🔲️ Sharp-cornered rectangular hollow section constants through `section_geometry` and `plastic_moduli`.
fn section_rhs(inputs: &Inputs) -> Outputs {
    let s = section("rhs", inputs.num("h"), inputs.num("b"), inputs.num("t"), inputs.num("t"), 0.0);
    let (a, iy, iz, wel_y, wel_z, _, _) = section_geometry(&s);
    let (wpl_y, wpl_z) = plastic_moduli(&s);
    Outputs::default().with("A", a).with("Iy", iy).with("Iz", iz).with("Wel_y", wel_y).with("Wel_z", wel_z).with("Wpl_y", wpl_y).with("Wpl_z", wpl_z)
}

/// ⭕️ Circular hollow section constants through `section_geometry` and `plastic_moduli`.
fn section_chs(inputs: &Inputs) -> Outputs {
    let s = section("chs", 0.0, 0.0, 0.0, inputs.num("t"), inputs.num("d"));
    let (a, iy, iz, wel_y, _, it, _) = section_geometry(&s);
    Outputs::default().with("A", a).with("Iy", iy).with("Iz", iz).with("It", it).with("Wel_y", wel_y).with("Wpl_y", plastic_moduli(&s).0)
}

/// 🔲️ The quantities of the finite-element section solver: constants, shear-centre offset from the centroid and principal moments; a case names the ones it compares.
fn kernel_outputs(c: Option<SectionConstants>) -> Outputs {
    let c = c.expect("finite-element section analysis");
    Outputs::default().with("A", c.area).with("Iy", c.iy).with("Iz", c.iz).with("I_major", c.i_major).with("I_minor", c.i_minor).with("It", c.j).with("Iw", c.iw).with("Wel_y", c.w_el_y_pos).with("Wel_z", c.w_el_z_pos).with("Wpl_y", c.w_pl_y).with("Wpl_z", c.w_pl_z).with("y_sc", c.shear_centre_y - c.centroid_y).with("z_sc", c.shear_centre_z - c.centroid_z)
}

/// 🚪️ Channel with root radius through `channel_constants`, including the shear-centre offset y_sc.
fn section_channel(inputs: &Inputs) -> Outputs {
    kernel_outputs(channel_constants(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r")))
}

/// 📐️ Angle with root and toe radii through `angle_constants`, including the principal moments.
fn section_angle(inputs: &Inputs) -> Outputs {
    kernel_outputs(angle_constants(inputs.num("h"), inputs.num("b"), inputs.num("t"), inputs.num("r_root"), inputs.num("r_toe")))
}

/// 🔱️ Tee with root radius through `tee_constants`.
fn section_tee(inputs: &Inputs) -> Outputs {
    kernel_outputs(tee_constants(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r")))
}

/// ⭕️ Elliptical hollow section through `elliptical_constants`.
fn section_ehs(inputs: &Inputs) -> Outputs {
    kernel_outputs(elliptical_constants(inputs.num("d_x"), inputs.num("d_y"), inputs.num("t")))
}

/// 🔷️ Free outline with holes through `polygon_props`; the torsion and warping constants come from the finite-element section kernel `analyse_section` (the same call as the free-outline torsion check).
fn section_polygon(inputs: &Inputs) -> Outputs {
    let outer: Vec<(f64, f64)> = inputs.json("outer").as_array().expect("outer ring").iter().map(|point| (point[0].as_f64().expect("y"), point[1].as_f64().expect("z"))).collect();
    let holes: Vec<Vec<(f64, f64)>> = inputs
        .json("holes")
        .as_array()
        .expect("hole rings")
        .iter()
        .map(|ring| ring.as_array().expect("hole ring").iter().map(|point| (point[0].as_f64().expect("y"), point[1].as_f64().expect("z"))).collect())
        .collect();
    let hole_refs: Vec<&[(f64, f64)]> = holes.iter().map(Vec::as_slice).collect();
    let p = polygon_props(&outer, &hole_refs);
    let ring = |points: &[(f64, f64)]| -> Vec<[f64; 2]> { points.iter().map(|point| [point.0, point.1]).collect() };
    let alloy = Material { id: "alloy".into(), e: 70_000.0e6, g: 27_000.0e6, rho: 2700.0 };
    let solved = analyse_section(&ring(&outer), &holes.iter().map(|hole| ring(hole)).collect::<Vec<_>>(), &alloy).expect("analyse_section");
    Outputs::default().with("A", p.a).with("Iy", p.iy).with("Iz", p.iz).with("Wel_y", p.wy).with("Wel_z", p.wz).with("It", solved.j).with("Iw", solved.iw)
}

/// 🪶️ Flexural buckling about the minor axis through `section_geometry`, `n_cr`, `lambda_bar` and `chi_from_lambda`; N_b,Rd = χ A f_o/γ_M1 is composed from our χ and A.
fn flexural_buckling(inputs: &Inputs) -> Outputs {
    let s = section("extrudedI", inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), 0.0);
    let (a, _, iz, _, _, _, _) = section_geometry(&s);
    let (length, f_o) = (inputs.num("L"), inputs.num("f_o"));
    let lambda = lambda_bar(length, iz, a, f_o);
    let chi = chi_from_lambda(lambda, inputs.num("alpha"), inputs.num("lambda_0"));
    Outputs::default().with("A", a).with("Iz", iz).with("N_cr", n_cr(inputs.num("E"), iz, length)).with("lambda", lambda).with("chi", chi).with("N_b_Rd", chi * a * f_o / inputs.num("gamma_M1"))
}

/// 🦋️ Lateral-torsional buckling under uniform moment through `section_geometry`, `m_cr_ltb` (C1 = 1) and `chi_from_lambda`; λ_LT = √(W_el,y f_o/M_cr) and M_b,Rd = χ_LT W_el,y f_o/γ_M1 are composed from our M_cr, χ_LT and W_el,y.
fn lateral_torsional_buckling(inputs: &Inputs) -> Outputs {
    let s = section("extrudedI", inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), 0.0);
    let (_, _, iz, wel_y, _, it, iw) = section_geometry(&s);
    let f_o = inputs.num("f_o");
    let m_cr = m_cr_ltb(1.0, inputs.num("E"), iz, iw, it, inputs.num("L"));
    let lambda = (wel_y * f_o / m_cr).sqrt();
    let chi = chi_from_lambda(lambda, inputs.num("alpha_LT"), inputs.num("lambda_0"));
    Outputs::default().with("Iz", iz).with("It", it).with("Iw", iw).with("M_cr", m_cr).with("lambda_LT", lambda).with("chi_LT", chi).with("M_b_Rd", chi * wel_y * f_o / inputs.num("gamma_M1"))
}

/// 📌️ Pin bending moment of a double-shear pin through `joints::pin_bending_moment`, which models plates without gaps (a = c = 0, inner plate b).
fn pin_bending(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_Ed", pin_bending_moment(inputs.num("F"), inputs.num("b")))
}

/// 🗂️ One adapter per oracle check of EN 1999 that has an evaluator entry point; the tee and elliptical hollow sections have none.
static ADAPTERS: [Adapter; 11] = [
    Adapter { check: "en1999.section.i", evaluator: "crate::part_1_1::{section_geometry, plastic_moduli}", run: section_i },
    Adapter { check: "en1999.section.rhs-sharp", evaluator: "crate::part_1_1::{section_geometry, plastic_moduli}", run: section_rhs },
    Adapter { check: "en1999.section.chs", evaluator: "crate::part_1_1::{section_geometry, plastic_moduli}", run: section_chs },
    Adapter { check: "en1999.section.channel", evaluator: "crate::part_1_1::channel_constants", run: section_channel },
    Adapter { check: "en1999.section.angle", evaluator: "crate::part_1_1::angle_constants", run: section_angle },
    Adapter { check: "en1999.section.tee", evaluator: "crate::part_1_1::tee_constants", run: section_tee },
    Adapter { check: "en1999.section.ehs", evaluator: "crate::part_1_1::elliptical_constants", run: section_ehs },
    Adapter { check: "en1999.section.polygon-holes", evaluator: "crate::polygon_props", run: section_polygon },
    Adapter { check: "en1999.6.3.1.flexural-buckling", evaluator: "crate::part_1_1::{section_geometry, n_cr, lambda_bar, chi_from_lambda}", run: flexural_buckling },
    Adapter { check: "en1999.6.3.2.ltb", evaluator: "crate::part_1_1::{section_geometry, m_cr_ltb, chi_from_lambda}", run: lateral_torsional_buckling },
    Adapter { check: "en1999.8.5.pin-bending-moment", evaluator: "crate::joints::pin_bending_moment", run: pin_bending },
];

/// ⚠️ No documented differences: every deviation of our evaluators from the blessed values stays a visible mismatch.
static KNOWN: [KnownDifference; 0] = [];

/// 🧮️ The EN 1999 adapter registry.
static REGISTRY: Registry = Registry { artifact: "en1999", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1999 oracle case is reproduced by our evaluators within its tolerance profile; gaps and mismatches fail the test.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1999");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
