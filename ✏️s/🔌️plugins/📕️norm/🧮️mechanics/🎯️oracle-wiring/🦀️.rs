//! 🎯️ Oracle wiring of the cross-section kernel: the checks `mechanics.analyse-section` and `mechanics.analyse-thin-walled` of the sectionproperties adapter, fed to [`analyse_section`] and [`analyse_thin_walled`]. The file is mounted with `#[path]` by every artifact oracle wiring that holds such cases, so the mechanics crate keeps no dependency on the compliance kernel.
//!
//! See <https://sectionproperties.readthedocs.io/>.

use semio_s_plugin_norm_compliance::oracle_driver::{Adapter, Inputs, Outputs};
use semio_s_plugin_norm_mechanics::{analyse_section, analyse_thin_walled, continuous_beam_envelope, Material, SectionConstants, SpanBeam, SpanFactors, ThinWall};

/// 🧪️ The unit material of the python adapter: `E = 1 + nu`, `G = 1 / 2` so that Poisson's ratio is `nu` and the constants stay geometric.
fn unit_material(inputs: &Inputs) -> Material {
    Material { id: "unit".into(), e: 1.0 + inputs.opt("nu").unwrap_or(0.0), g: 0.5, rho: 0.0 }
}

/// 📐️ A `[y, z]` vertex list of the case file.
fn ring(value: &serde_json::Value) -> Vec<[f64; 2]> {
    value.as_array().expect("ring is a list").iter().map(|point| [point[0].as_f64().expect("y"), point[1].as_f64().expect("z")]).collect()
}

/// 📤️ Every quantity of [`SectionConstants`] under the field names of the python adapter; the moduli are included only when the kernel derived them.
fn constants(c: &SectionConstants, with_moduli: bool) -> Outputs {
    let mut out = Outputs::default()
        .with("area", c.area)
        .with("centroid_y", c.centroid_y)
        .with("centroid_z", c.centroid_z)
        .with("iy", c.iy)
        .with("iz", c.iz)
        .with("iyz", c.iyz)
        .with("i_major", c.i_major)
        .with("i_minor", c.i_minor)
        .with("j", c.j)
        .with("iw", c.iw)
        .with("shear_centre_y", c.shear_centre_y)
        .with("shear_centre_z", c.shear_centre_z)
        .with("a_vy", c.a_vy)
        .with("a_vz", c.a_vz);
    if with_moduli {
        out = out.with("w_el_y_pos", c.w_el_y_pos).with("w_el_y_neg", c.w_el_y_neg).with("w_el_z_pos", c.w_el_z_pos).with("w_el_z_neg", c.w_el_z_neg).with("w_pl_y", c.w_pl_y).with("w_pl_z", c.w_pl_z);
    }
    out
}

/// 🔲️ `mechanics.analyse-section`: `outer` ring, optional `holes` rings, `nu`; the mesh density is the kernel default.
pub fn analyse_section_case(inputs: &Inputs) -> Outputs {
    let holes: Vec<Vec<[f64; 2]>> = inputs.0.get("holes").and_then(|value| value.as_array()).map(|rings| rings.iter().map(ring).collect()).unwrap_or_default();
    let solved = analyse_section(&ring(inputs.json("outer")), &holes, &unit_material(inputs)).expect("analyse_section");
    constants(&solved, inputs.opt("nu").unwrap_or(0.0) == 0.0)
}

/// 🪶️ `mechanics.analyse-thin-walled`: centreline `walls` with `y1 z1 y2 z2 thickness`; no moduli (the walls carry no outline).
pub fn analyse_thin_walled_case(inputs: &Inputs) -> Outputs {
    let walls: Vec<ThinWall> = inputs
        .json("walls")
        .as_array()
        .expect("walls is a list")
        .iter()
        .map(|wall| ThinWall { y1: wall["y1"].as_f64().expect("y1"), z1: wall["z1"].as_f64().expect("z1"), y2: wall["y2"].as_f64().expect("y2"), z2: wall["z2"].as_f64().expect("z2"), thickness: wall["thickness"].as_f64().expect("thickness") })
        .collect();
    constants(&analyse_thin_walled(&walls, &unit_material(inputs)).expect("analyse_thin_walled"), false)
}

const SAGGING: [&str; 3] = ["M_sag_0", "M_sag_1", "M_sag_2"];

/// 🪜️ `mechanics.beam-span-envelope`: the load arrangement envelope of a continuous beam (up to three spans, uniform `g` and `q`) with the permanent load of every span taking `gamma_G` or `gamma_G_inf` (EN 1990 A1.3.1) and the variable load `gamma_Q` on any span subset (6.4.3.2), through [`continuous_beam_envelope`]: `M_sag_k`, `M_hog_support`, `R_max_support` (support 1) and `R_min_0`.
pub fn beam_span_envelope_case(inputs: &Inputs) -> Outputs {
    let spans = inputs.nums("spans");
    let factors = SpanFactors { gamma_g: inputs.num("gamma_G"), gamma_g_inf: inputs.num("gamma_G_inf"), gamma_q: inputs.num("gamma_Q") };
    let envelope = continuous_beam_envelope(&SpanBeam { spans: &spans, e: inputs.num("E"), area: inputs.num("A"), inertia: inputs.num("I") }, inputs.num("g"), inputs.num("q"), &factors).expect("span envelope");
    let mut outputs = Outputs::default().with("M_hog_support", envelope.hogging).with("R_max_support", envelope.reaction_max[1]).with("R_min_0", envelope.reaction_min[0]);
    for (span, sagging) in envelope.sagging.iter().enumerate() {
        outputs = outputs.with(SAGGING[span], *sagging);
    }
    outputs
}

/// 🧩️ The load arrangement envelope adapter row shared by every Eurocode artifact with continuous members.
pub const SPAN_ENVELOPE: Adapter = Adapter { check: "mechanics.beam-span-envelope", evaluator: "semio_s_plugin_norm_mechanics::continuous_beam_envelope", run: beam_span_envelope_case };

/// 🧩️ The two section adapter rows, ready to be spliced into an artifact adapter table.
pub const ADAPTERS: [Adapter; 2] = [
    Adapter { check: "mechanics.analyse-section", evaluator: "semio_s_plugin_norm_mechanics::analyse_section", run: analyse_section_case },
    Adapter { check: "mechanics.analyse-thin-walled", evaluator: "semio_s_plugin_norm_mechanics::analyse_thin_walled", run: analyse_thin_walled_case },
];
