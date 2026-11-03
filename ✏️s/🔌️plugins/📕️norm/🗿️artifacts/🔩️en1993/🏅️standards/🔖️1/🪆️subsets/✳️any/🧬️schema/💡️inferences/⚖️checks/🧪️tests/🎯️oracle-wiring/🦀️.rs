//! 🎯️ Typed wiring of the blessed EN 1993 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use crate::clauses::{global, parametric_sections, resistance, stability, weld_group};
use crate::{part_1_1, part_1_12, AnnexParams};
use semio_s_artifact_norm_en1993_model::rolled_heb_catalogue;
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_mechanics::{self as mech, section::profiles};

/// 🔲️ The shared cross-section adapters `mechanics.analyse-section` and `mechanics.analyse-thin-walled` (mounted from the mechanics tree).
#[path = "../../../../../../../../../../../🧮️mechanics/🎯️oracle-wiring/🦀️.rs"]
#[allow(dead_code)]
mod section_wiring;

/// 🧮️ The partial factors of a case on top of the EN recommended values.
fn params(gamma_m0: f64, gamma_m1: f64) -> AnnexParams {
    AnnexParams { gamma_m0, gamma_m1, ..AnnexParams::en() }
}

/// 🏠️ Fixed-base portal of the oracle inputs solved by the mechanics kernel under the horizontal force `H` at the left beam-column joint (global X, frame in the X-Z plane).
fn portal_under_h(inputs: &Inputs) -> mech::StaticResult {
    let (e, area, h, span) = (inputs.num("E"), inputs.num("A"), inputs.num("h"), inputs.num("span"));
    let node = |id: &str, x: f64, z: f64| mech::Node { id: id.into(), x, y: 0.0, z };
    let section = |id: &str, i: f64| mech::Section { id: id.into(), a: area, iy: i, iz: i, j: 1.0e-6, ..Default::default() };
    let bar = |id: &str, from: &str, to: &str, section: &str| mech::Member { id: id.into(), node_i: from.into(), node_j: to.into(), material: "S".into(), section: section.into(), ..Default::default() };
    let base = |id: &str| mech::Support { node: id.into(), fixed: [true; 6], ..Default::default() };
    let model = mech::Model {
        nodes: vec![node("a", 0.0, 0.0), node("b", 0.0, h), node("c", span, h), node("d", span, 0.0)],
        materials: vec![mech::Material { id: "S".into(), e, g: e / 2.6, rho: 7850.0 }],
        sections: vec![section("C", inputs.num("I_col")), section("B", inputs.num("I_beam"))],
        members: vec![bar("col1", "a", "b", "C"), bar("beam", "b", "c", "B"), bar("col2", "d", "c", "C")],
        supports: vec![base("a"), base("d")],
        load_cases: vec![mech::LoadCase { id: "h".into(), nodal: vec![mech::NodalLoad { node: "b".into(), force: [inputs.num("H"), 0.0, 0.0, 0.0, 0.0, 0.0] }], ..Default::default() }],
        ..Default::default()
    };
    let control = mech::Control::none();
    mech::Analysis::new(&model, &control).expect("portal analysis").solve_case(&model.load_cases[0], &control).expect("portal load case")
}

/// 🌐️ The first-order sway of the portal at the left joint in metres.
fn portal_sway(inputs: &Inputs) -> f64 {
    let result = portal_under_h(inputs);
    result.displacements.iter().find(|displacement| displacement.node == "b").expect("sway node").u[0]
}

/// 🏠️ Sway, base moments and base shear of the fixed-base portal through the mechanics kernel.
fn portal_sway_frame(inputs: &Inputs) -> Outputs {
    let result = portal_under_h(inputs);
    let reaction = |node: &str| result.reactions.iter().find(|reaction| reaction.node == node).expect("base reaction").force;
    let sway = result.displacements.iter().find(|displacement| displacement.node == "b").expect("sway node").u[0];
    Outputs::default()
        .with("sway", sway)
        .with("M_base_left", reaction("a")[4].abs())
        .with("M_base_right", reaction("d")[4].abs())
        .with("H_base_left", reaction("a")[0].abs())
}

/// 🌉️ Simply supported single-span beam under a uniform downward load through the mechanics kernel, split at midspan.
fn simply_supported_beam(inputs: &Inputs) -> Outputs {
    let (spans, loads) = (inputs.nums("spans"), inputs.nums("w"));
    assert_eq!(spans.len(), 1, "the oracle wiring solves one span");
    let (length, load) = (spans[0], loads[0]);
    let node = |id: &str, x: f64| mech::Node { id: id.into(), x, y: 0.0, z: 0.0 };
    let bar = |id: &str, from: &str, to: &str| mech::Member { id: id.into(), node_i: from.into(), node_j: to.into(), material: "S".into(), section: "X".into(), ..Default::default() };
    let line = |member: &str| mech::DistributedLoad { member: member.into(), frame: mech::LoadFrame::Global, start: [0.0, 0.0, -load], end: [0.0, 0.0, -load] };
    let e = inputs.num("E");
    let model = mech::Model {
        nodes: vec![node("a", 0.0), node("m", length / 2.0), node("b", length)],
        materials: vec![mech::Material { id: "S".into(), e, g: e / 2.6, rho: 7850.0 }],
        sections: vec![mech::Section { id: "X".into(), a: inputs.num("A"), iy: inputs.num("I"), iz: inputs.num("I"), j: 1.0e-6, ..Default::default() }],
        members: vec![bar("m1", "a", "m"), bar("m2", "m", "b")],
        supports: vec![
            mech::Support { node: "a".into(), fixed: [true, true, true, true, false, false], ..Default::default() },
            mech::Support { node: "b".into(), fixed: [false, true, true, false, false, false], ..Default::default() },
        ],
        load_cases: vec![mech::LoadCase { id: "q".into(), distributed: vec![line("m1"), line("m2")], ..Default::default() }],
        ..Default::default()
    };
    let control = mech::Control::none();
    let result = mech::Analysis::new(&model, &control).expect("beam analysis").solve_case(&model.load_cases[0], &control).expect("beam load case");
    let moment = result.members.iter().flat_map(|member| member.diagram.iter()).map(|point| point.my.abs()).fold(0.0, f64::max);
    let reaction = result.reactions.iter().find(|reaction| reaction.node == "a").expect("left reaction").force[2];
    let deflection = result.displacements.iter().find(|displacement| displacement.node == "m").expect("midspan node").u[2].abs();
    Outputs::default().with("R_0", reaction).with("M_sag_0", moment).with("delta_0", deflection)
}

/// 🏢️ α_cr of Eq. 5.2 with the storey drift of the portal solved by the mechanics kernel through `global::alpha_cr_storey`.
fn alpha_cr_storey_portal(inputs: &Inputs) -> Outputs {
    let sway = portal_sway(inputs);
    let alpha_cr = global::alpha_cr_storey(inputs.num("H"), inputs.num("V"), inputs.num("h"), sway).expect("alpha_cr of Eq. 5.2");
    Outputs::default().with("alpha_cr_formula", alpha_cr).with("sway", sway)
}

/// 🌀️ α_cr as the sway buckling eigenvalue of the fixed-base portal with V/2 per column through `global::alpha_cr_portal_fixed`.
fn alpha_cr_eigen(inputs: &Inputs) -> Outputs {
    let eigen = global::alpha_cr_portal_fixed(inputs.num("E"), inputs.num("I_col"), inputs.num("I_beam"), inputs.num("h"), inputs.num("span"), inputs.num("V") / 2.0);
    Outputs::default().with("alpha_cr_eigen", eigen.expect("sway eigenvalue"))
}

/// 📐️ Sway imperfection α_h, φ and the equivalent horizontal force through `global::sway_alpha_h`, `global::sway_phi` and `global::sway_equivalent_force`.
fn sway_imperfection(inputs: &Inputs) -> Outputs {
    let phi = global::sway_phi(inputs.num("h"), inputs.num("m"));
    Outputs::default().with("alpha_h", global::sway_alpha_h(inputs.num("h"))).with("phi", phi).with("H_eq", global::sway_equivalent_force(phi, inputs.num("V")))
}

/// 🧵️ Box weld group under shear at an eccentricity (per unit throat) through `weld_group::group_properties` and `weld_group::max_force_per_length`.
fn weld_group_box(inputs: &Inputs) -> Outputs {
    let group = weld_group::group_properties(weld_group::WeldPattern::Box, inputs.num("b"), inputs.num("d"));
    Outputs::default()
        .with("L_w", group.length)
        .with("I_p", group.i_polar)
        .with("f_max", weld_group::max_force_per_length(&group, inputs.num("F"), inputs.num("e")))
}

/// 📚️ Section properties of the rolled HEB catalogue row whose geometry equals the inputs; a geometry outside the catalogue (IPE 300) is solved by the finite-element section solver of the mechanics kernel.
fn rolled_i_catalogue(inputs: &Inputs) -> Outputs {
    let near = |a: f64, key: &str| (a - inputs.num(key)).abs() < 1e-9;
    let catalogue = rolled_heb_catalogue();
    let Some(row) = catalogue.iter().find(|row| near(row.h, "h") && near(row.b, "b") && near(row.tf, "tf") && near(row.tw, "tw") && near(row.r, "r")) else {
        return kernel_section(&profiles::i_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r"), ARC_POINTS));
    };
    Outputs::default()
        .with("A", row.area)
        .with("Iy", row.iy)
        .with("Iz", row.iz)
        .with("It", row.it)
        .with("Iw", row.iw)
        .with("Wel_y", row.w_el_y)
        .with("Wel_z", row.w_el_z)
        .with("Wpl_y", row.w_pl_y)
        .with("Wpl_z", row.w_pl_z)
}

/// 🔲️ Points per quarter arc of the profile outlines, the discretisation of the blessed `sectionproperties` cases.
const ARC_POINTS: usize = 16;

/// 🔲️ Elastic and plastic constants of a profile outline from the finite-element section solver of the mechanics kernel (St Venant torsion, warping, shear centre relative to the centroid, principal moments); only the quantities a case names are compared.
fn kernel_section(outline: &profiles::Outline) -> Outputs {
    let material = mech::Material { id: "steel".into(), e: 210.0e9, g: 81.0e9, rho: 7850.0 };
    let c = mech::analyse_section(&outline.0, &outline.1, &material).expect("finite-element section analysis");
    Outputs::default()
        .with("A", c.area)
        .with("Iy", c.iy)
        .with("Iz", c.iz)
        .with("I_major", c.i_major)
        .with("I_minor", c.i_minor)
        .with("It", c.j)
        .with("Iw", c.iw)
        .with("Wel_y", c.w_el_y_pos)
        .with("Wel_z", c.w_el_z_pos)
        .with("Wpl_y", c.w_pl_y)
        .with("Wpl_z", c.w_pl_z)
        .with("y_sc", c.shear_centre_y - c.centroid_y)
        .with("z_sc", c.shear_centre_z - c.centroid_z)
}

/// 🔱️ Tee section constants through the kernel section solver.
fn tee_section(inputs: &Inputs) -> Outputs {
    kernel_section(&profiles::tee_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r"), ARC_POINTS))
}

/// 🏗️ Welded mono-symmetric I section constants through the kernel section solver.
fn mono_i_section(inputs: &Inputs) -> Outputs {
    kernel_section(&profiles::mono_i_section(inputs.num("h"), [inputs.num("b_t"), inputs.num("t_ft")], [inputs.num("b_b"), inputs.num("t_fb")], inputs.num("tw"), inputs.num("r"), ARC_POINTS))
}

/// ⭕️ Elliptical hollow section constants through the kernel section solver.
fn ehs_section(inputs: &Inputs) -> Outputs {
    kernel_section(&profiles::elliptical_hollow_section(inputs.num("d_x"), inputs.num("d_y"), inputs.num("t"), 128))
}

/// 🔲️ Rectangular hollow section with rounded corners through the kernel section solver.
fn rhs_rounded(inputs: &Inputs) -> Outputs {
    kernel_section(&profiles::rectangular_hollow_section(inputs.num("h"), inputs.num("b"), inputs.num("t"), inputs.num("r_out"), ARC_POINTS))
}

/// 🧲️ Parallel-flange channel constants (shear centre from the centroid) through the kernel section solver.
fn channel_section(inputs: &Inputs) -> Outputs {
    kernel_section(&profiles::channel_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r"), ARC_POINTS))
}

/// 📐️ Angle constants with principal moments through the kernel section solver.
fn angle_section(inputs: &Inputs) -> Outputs {
    kernel_section(&profiles::angle_section(inputs.num("h"), inputs.num("b"), inputs.num("t"), inputs.num("r_root"), inputs.num("r_toe"), ARC_POINTS))
}

/// 🦋️ M_cr of a doubly symmetric rolled I (NCCI SN003) from I_z, I_t and I_w of the kernel section solver through `stability::mcr_closed_form`.
fn mcr_rolled_i(inputs: &Inputs) -> Outputs {
    let section = kernel_section(&profiles::i_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r"), ARC_POINTS));
    let (iz, it, iw) = (section.0["Iz"], section.0["It"], section.0["Iw"]);
    let mcr = stability::mcr_closed_form(inputs.num("E"), inputs.num("G"), iz, it, iw, inputs.num("L"), 1.0, 1.0, inputs.num("C1"), 0.0, 0.0);
    Outputs::default().with("M_cr", mcr).with("Iz", iz).with("It", it).with("Iw", iw)
}

/// 📐️ Sharp-corner rectangular hollow section properties through `parametric_sections::{rhs, rhs_minor_moduli}`.
fn rhs_sharp(inputs: &Inputs) -> Outputs {
    let p = parametric_sections::rhs(inputs.num("h"), inputs.num("b"), inputs.num("t"));
    let (w_el_z, w_pl_z) = parametric_sections::rhs_minor_moduli(inputs.num("h"), inputs.num("b"), inputs.num("t"));
    Outputs::default().with("A", p.area).with("Iy", p.iy).with("Iz", p.iz).with("Wel_y", p.w_el_y).with("Wel_z", w_el_z).with("Wpl_y", p.w_pl_y).with("Wpl_z", w_pl_z)
}

/// ⭕️ Circular hollow section properties through `parametric_sections::{chs, chs_torsion}`; the minor-axis moduli equal the major-axis ones.
fn chs(inputs: &Inputs) -> Outputs {
    let p = parametric_sections::chs(inputs.num("d"), inputs.num("t"));
    Outputs::default().with("A", p.area).with("Iy", p.iy).with("Iz", p.iz).with("It", parametric_sections::chs_torsion(inputs.num("d"), inputs.num("t"))).with("Wel_y", p.w_el_y).with("Wel_z", p.w_el_y).with("Wpl_y", p.w_pl_y).with("Wpl_z", p.w_pl_y)
}

/// 🔗️ Web area fraction a_w of a hollow section (Eq. 6.39) through `resistance::a_w_hollow`.
fn aw_hollow(inputs: &Inputs) -> Outputs {
    Outputs::default().with("a_w", resistance::a_w_hollow(inputs.num("a"), inputs.num("b"), inputs.num("t")))
}

/// ✂️ Shear stress τ_Ed of an I section web (Eq. 6.21) through `resistance::tau_ed_i`.
fn tau_ed_i(inputs: &Inputs) -> Outputs {
    Outputs::default().with("tau_Ed", resistance::tau_ed_i(inputs.num("v_ed"), inputs.num("a_w"), inputs.num("a_f")))
}

/// 🔗️ N_u,Rd = 0.9 A_net f_u/γ_M2 (Eq. 6.7) through `resistance::n_u_rd`.
fn n_u_rd(inputs: &Inputs) -> Outputs {
    Outputs::default().with("N_u_Rd", resistance::n_u_rd(inputs.num("a_net"), inputs.num("f_u"), inputs.num("gamma_m2")))
}

/// 🔗️ N_c,Rd = A f_y/γ_M0 (Eq. 6.10) through `resistance::n_pl_rd`.
fn n_c_rd(inputs: &Inputs) -> Outputs {
    Outputs::default().with("N_c_Rd", resistance::n_pl_rd(inputs.num("a"), inputs.num("f_y"), inputs.num("gamma_m0")))
}

/// 🔗️ N_pl,Rd = A f_y/γ_M0 (Eq. 6.6) through `resistance::n_pl_rd`.
fn n_pl_rd(inputs: &Inputs) -> Outputs {
    Outputs::default().with("N_pl_Rd", resistance::n_pl_rd(inputs.num("a"), inputs.num("f_y"), inputs.num("gamma_m0")))
}

/// 📐️ M_c,Rd = W_el,min f_y/γ_M0 for class 3 (Eq. 6.14) through `part_1_12::elastic_bending_resistance_nm`.
fn mc_rd_class3(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_c_Rd", part_1_12::elastic_bending_resistance_nm(inputs.num("w_el_min"), inputs.num("f_y"), 3, params(inputs.num("gamma_m0"), 1.0)))
}

/// 📐️ M_c,Rd = W_pl f_y/γ_M0 for class 1 and 2 (Eq. 6.13) through `part_1_1::bending_resistance_nm`.
fn mc_rd_class12(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_c_Rd", part_1_1::bending_resistance_nm(inputs.num("w_pl"), inputs.num("f_y"), 1, params(inputs.num("gamma_m0"), 1.0)))
}

/// 🔗️ M_N,Rd = M_pl,Rd (1 − n²) of a solid rectangle (Eq. 6.32) through `resistance::m_n_rd_solid_rect` with n = N_Ed/N_pl,Rd.
fn mn_rd_rectangular(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_N_Rd", resistance::m_n_rd_solid_rect(inputs.num("m_pl_rd"), inputs.num("n_ed") / inputs.num("n_pl_rd")))
}

/// 🔗️ M_N,y,Rd of a hollow section (Eq. 6.39) through `resistance::m_n_rd_rhs`.
fn mn_y_rd_hollow(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_N_y_Rd", resistance::m_n_rd_rhs(inputs.num("mpl_y_rd"), inputs.num("n"), inputs.num("a_w")))
}

/// ✂️ V_pl,Rd = A_v f_y/(√3 γ_M0) (Eq. 6.18) through `resistance::v_pl_rd`.
fn v_pl_rd(inputs: &Inputs) -> Outputs {
    Outputs::default().with("V_pl_Rd", resistance::v_pl_rd(inputs.num("a_v"), inputs.num("f_y"), inputs.num("gamma_m0")))
}

/// ✂️ A_v of a rolled I/H (6.2.6(3)a) through `resistance::av_rolled_i` with the first flange.
fn av_rolled_i(inputs: &Inputs) -> Outputs {
    Outputs::default().with("A_v", resistance::av_rolled_i(inputs.num("a"), inputs.num("b1"), inputs.num("tf1"), inputs.num("tw"), inputs.num("r1"), inputs.num("hw"), inputs.num("eta")))
}

/// ✂️ A_v of a rectangular hollow section for load parallel to the depth through `resistance::av_rhs`.
fn av_rhs_depth(inputs: &Inputs) -> Outputs {
    Outputs::default().with("A_v", resistance::av_rhs(inputs.num("a"), inputs.num("b"), inputs.num("h"), true))
}

/// ✂️ A_v of a circular hollow section through `resistance::av_chs`.
fn av_chs(inputs: &Inputs) -> Outputs {
    Outputs::default().with("A_v", resistance::av_chs(inputs.num("a")))
}

/// ✂️ A_v of a rolled channel through `resistance::av_rolled_u`.
fn av_channel(inputs: &Inputs) -> Outputs {
    Outputs::default().with("A_v", resistance::av_rolled_u(inputs.num("a"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"), inputs.num("r")))
}

/// ✂️ M_y,V,Rd of a doubly symmetric I (6.2.8(5)) through `resistance::m_y_v_rd_i`, capped at the given M_y,c,Rd.
fn my_v_rd(inputs: &Inputs) -> Outputs {
    let (h_w, t_w) = (inputs.num("h_w"), inputs.num("t_w"));
    let reduced = resistance::m_y_v_rd_i(inputs.num("w_pl_y"), inputs.num("rho"), h_w * t_w, t_w, inputs.num("f_y"), inputs.num("gamma_m0"));
    Outputs::default().with("M_y_V_Rd", reduced.min(inputs.num("m_y_c_rd")))
}

/// 🔗️ M_N,y,Rd of a doubly symmetric I/H (Eq. 6.36) through `resistance::m_n_y_rd_i`.
fn my_n_rd(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_N_y_Rd", resistance::m_n_y_rd_i(inputs.num("mpl_y_rd"), inputs.num("n"), inputs.num("a")))
}

/// 🦋️ M_b,Rd = χ_LT W_y f_y/γ_M1 (Eq. 6.55) through `part_1_1::ltb_resistance_nm`.
fn mb_rd(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_b_Rd", part_1_1::ltb_resistance_nm(inputs.num("w_y"), inputs.num("f_y"), inputs.num("chi_lt"), params(1.0, inputs.num("gamma_m1"))))
}

/// 🏢️ α_cr = (H_Ed/V_Ed)(h/δ_H,Ed) (Eq. 5.2) through `global::alpha_cr_storey`.
fn alpha_cr(inputs: &Inputs) -> Outputs {
    Outputs::default().with("alpha_cr", global::alpha_cr_storey(inputs.num("h_ed"), inputs.num("v_ed"), inputs.num("h"), inputs.num("delta_h_ed")).expect("alpha_cr of Eq. 5.2"))
}

/// 🗂️ One adapter per wired oracle check of EN 1993; the section checks tee, mono-i, ehs, rhs (rounded corners), channel and angle, `en1993.6.3.2.mcr-i` and the IPE 300 row of `en1993.section.i-rolled` run on the finite-element section solver of the mechanics kernel (`analyse_section`, validated against sectionproperties within 0.5 percent).
static ADAPTERS: [Adapter; 36] = [
    Adapter { check: "mechanics.analyse-section", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: section_wiring::analyse_section_case },
    Adapter { check: "mechanics.analyse-thin-walled", evaluator: "semio_s_plugin_norm_mechanics::analyse_thin_walled", run: section_wiring::analyse_thin_walled_case },
    Adapter { check: "mechanics.portal-sway", evaluator: "semio_s_plugin_norm_mechanics::Analysis::solve_case", run: portal_sway_frame },
    Adapter { check: "mechanics.beam-continuous", evaluator: "semio_s_plugin_norm_mechanics::Analysis::solve_case", run: simply_supported_beam },
    Adapter { check: "en1993.5.2.alpha-cr-storey", evaluator: "crate::clauses::global::alpha_cr_storey", run: alpha_cr_storey_portal },
    Adapter { check: "en1993.5.2.alpha-cr-eigen", evaluator: "crate::clauses::global::alpha_cr_portal_fixed", run: alpha_cr_eigen },
    Adapter { check: "en1993.5.2.alpha-cr", evaluator: "crate::clauses::global::alpha_cr_storey", run: alpha_cr },
    Adapter { check: "en1993.5.3.2.sway-imperfection", evaluator: "crate::clauses::global::{sway_alpha_h, sway_phi, sway_equivalent_force}", run: sway_imperfection },
    Adapter { check: "en1993.1-8.4.5.weld-group-box", evaluator: "crate::clauses::weld_group::{group_properties, max_force_per_length}", run: weld_group_box },
    Adapter { check: "en1993.section.i-rolled", evaluator: "semio_s_artifact_norm_en1993_model::rolled_heb_catalogue, semio_s_plugin_norm_mechanics::analyse_section", run: rolled_i_catalogue },
    Adapter { check: "en1993.section.tee", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: tee_section },
    Adapter { check: "en1993.section.mono-i", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: mono_i_section },
    Adapter { check: "en1993.section.ehs", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: ehs_section },
    Adapter { check: "en1993.section.rhs", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: rhs_rounded },
    Adapter { check: "en1993.section.channel", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: channel_section },
    Adapter { check: "en1993.section.angle", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: angle_section },
    Adapter { check: "en1993.6.3.2.mcr-i", evaluator: "semio_s_plugin_norm_mechanics::analyse_section, crate::clauses::stability::mcr_closed_form", run: mcr_rolled_i },
    Adapter { check: "en1993.section.rhs-sharp", evaluator: "crate::clauses::parametric_sections::rhs", run: rhs_sharp },
    Adapter { check: "en1993.section.chs", evaluator: "crate::clauses::parametric_sections::chs", run: chs },
    Adapter { check: "en1993.6.7.nu-rd", evaluator: "crate::clauses::resistance::n_u_rd", run: n_u_rd },
    Adapter { check: "en1993.6.10.nc-rd", evaluator: "crate::clauses::resistance::n_pl_rd", run: n_c_rd },
    Adapter { check: "en1993.6.2.npl", evaluator: "crate::clauses::resistance::n_pl_rd", run: n_pl_rd },
    Adapter { check: "en1993.6.14.mc-rd-class3", evaluator: "crate::part_1_12::elastic_bending_resistance_nm", run: mc_rd_class3 },
    Adapter { check: "en1993.6.2.mc-rd-class12", evaluator: "crate::part_1_1::bending_resistance_nm", run: mc_rd_class12 },
    Adapter { check: "en1993.6.32.mnrd-rectangular", evaluator: "crate::clauses::resistance::m_n_rd_solid_rect", run: mn_rd_rectangular },
    Adapter { check: "en1993.6.39.mnyrd-hollow", evaluator: "crate::clauses::resistance::m_n_rd_rhs", run: mn_y_rd_hollow },
    Adapter { check: "en1993.6.2.vpl-rd", evaluator: "crate::clauses::resistance::v_pl_rd", run: v_pl_rd },
    Adapter { check: "en1993.6.2.av-rolled-i", evaluator: "crate::clauses::resistance::av_rolled_i", run: av_rolled_i },
    Adapter { check: "en1993.6.2.av-rhs-depth", evaluator: "crate::clauses::resistance::av_rhs", run: av_rhs_depth },
    Adapter { check: "en1993.6.2.av-chs", evaluator: "crate::clauses::resistance::av_chs", run: av_chs },
    Adapter { check: "en1993.6.2.av-channel", evaluator: "crate::clauses::resistance::av_rolled_u", run: av_channel },
    Adapter { check: "en1993.6.2.my-v-rd", evaluator: "crate::clauses::resistance::m_y_v_rd_i", run: my_v_rd },
    Adapter { check: "en1993.6.2.my-n-rd", evaluator: "crate::clauses::resistance::m_n_y_rd_i", run: my_n_rd },
    Adapter { check: "en1993.6.3.mb-rd", evaluator: "crate::part_1_1::ltb_resistance_nm", run: mb_rd },
    Adapter { check: "en1993.6.39.aw-hollow", evaluator: "crate::clauses::resistance::a_w_hollow", run: aw_hollow },
    Adapter { check: "en1993.6.21.tau-i", evaluator: "crate::clauses::resistance::tau_ed_i", run: tau_ed_i },
];

/// ⚠️ No quantity of OUR evaluators differs from its blessed value; the one known difference of EN 1993 lies between two blessed methods and is asserted by [`eq_5_2_overestimates_the_eigenvalue_for_a_documented_reason`].
static KNOWN: [KnownDifference; 0] = [];

/// 🧭️ The EN 1993 registry.
static REGISTRY: Registry = Registry { artifact: "en1993", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1993 oracle case is reproduced by our evaluators; unwired checks and quantities are reported as gaps.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1993");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}

/// ⚠️ Documented reason of the Eq. 5.2 against eigenvalue difference: α_cr = (H/V)(h/δ_H) (Eq. 5.2) is valid for pitched roofs up to 26 degrees and low beam axial force and ignores the P-Δ stiffness loss of stiff beams; the eigenvalue (PyNite P-Delta agrees to 0.3 percent) is lower, request `oracle → en1993`.
pub const EQ_5_2_VS_EIGENVALUE_REASON: &str = "Eq. 5.2 is limited to roofs up to 26 degrees and low beam axial force; for a fixed-base portal it overestimates the sway buckling eigenvalue (11.70 against 9.97 stiff beam, 7.19 against 6.39 flexible beam)";

/// 🔬️ Both of our evaluators reproduce their own blessed side and the formula exceeds the eigenvalue by more than the method spread, for the stated reason.
#[test]
fn eq_5_2_overestimates_the_eigenvalue_for_a_documented_reason() {
    let cases = oracle_driver::case_files_above(env!("CARGO_MANIFEST_DIR")).iter().flat_map(|file| oracle_driver::load_cases(file)).collect::<Vec<_>>();
    for beam in ["stiff-beam", "flexible-beam"] {
        let inputs = |id: String| cases.iter().find(|case| case.id == id).unwrap_or_else(|| panic!("case {id}")).inputs.clone();
        let formula = alpha_cr_storey_portal(&inputs(format!("en1993.5.2.alpha-cr-formula-vs-eigen-{beam}"))).0["alpha_cr_formula"];
        let eigen = alpha_cr_eigen(&inputs(format!("en1993.5.2.alpha-cr-eigen-{beam}"))).0["alpha_cr_eigen"];
        assert!(formula > 1.03 * eigen, "{EQ_5_2_VS_EIGENVALUE_REASON}: formula {formula} eigenvalue {eigen}");
    }
}
