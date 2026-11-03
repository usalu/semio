//! 🔩️ EN 1999-1-1 alloys, HAZ, classification, effective sections and member stability: pure clause functions. See [`super`].

use crate::*;
use semio_s_artifact_norm_en1999_model::{AluminiumSection, PlateElement};
use semio_s_plugin_norm_mechanics as mechanics;
use std::f64::consts::PI;

//#region 🔖️Alloys
/// 🔩 EN 1999-1-1 Table 3.2 properties of one alloy temper and thickness band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlloyProps {
    pub f_o_pa: f64,
    pub f_u_pa: f64,
    pub buckling_class_a: bool,
    pub rho_o_haz: f64,
    pub rho_u_haz: f64,
    pub t_max_m: f64,
}

const fn row(f_o: f64, f_u: f64, class_a: bool, rho_o: f64, rho_u: f64, t_max: f64) -> AlloyProps {
    AlloyProps { f_o_pa: f_o * 1.0e6, f_u_pa: f_u * 1.0e6, buckling_class_a: class_a, rho_o_haz: rho_o, rho_u_haz: rho_u, t_max_m: t_max }
}

/// 🧱 Thickness bands per alloy key, ascending in `t_max_m`; the 6082-T6 bands follow the spec worked example E2-01, the others the repo rows (unverified).
pub const ALLOY_BANDS: &[(&str, AlloyProps)] = &[
    ("aw6060-t6", row(160.0, 215.0, true, 0.48, 0.56, 0.025)),
    ("aw6061-t6", row(240.0, 260.0, true, 0.48, 0.67, 0.025)),
    ("aw6063-t6", row(160.0, 195.0, false, 0.41, 0.56, 0.025)),
    ("aw6082-t6", row(250.0, 290.0, true, 0.48, 0.64, 0.005)),
    ("aw6082-t6", row(260.0, 310.0, true, 0.48, 0.60, 0.025)),
    ("aw5083-o", row(125.0, 275.0, false, 1.0, 1.0, 0.1)),
    ("aw5083-h111", row(125.0, 275.0, false, 1.0, 1.0, 0.1)),
];

/// 🔩 One catalogue row per alloy (the thickest band), shared by `resolve_alloy` and the catalogue table.
pub const CATALOGUE_ALLOY_ROWS: &[(&str, AlloyProps)] = &[
    ("aw6060-t6", row(160.0, 215.0, true, 0.48, 0.56, 0.025)),
    ("aw6061-t6", row(240.0, 260.0, true, 0.48, 0.67, 0.025)),
    ("aw6063-t6", row(160.0, 195.0, false, 0.41, 0.56, 0.025)),
    ("aw6082-t6", row(260.0, 310.0, true, 0.48, 0.60, 0.025)),
    ("aw5083-o", row(125.0, 275.0, false, 1.0, 1.0, 0.1)),
    ("aw5083-h111", row(125.0, 275.0, false, 1.0, 1.0, 0.1)),
];

pub const CATALOGUE_ALLOYS: &[&str] = &["aw6060-t6", "aw6061-t6", "aw6063-t6", "aw6082-t6", "aw5083-o", "aw5083-h111"];

fn alloy_key(designation: &str) -> String {
    let key = designation.to_ascii_lowercase().replace('_', "-");
    match key.as_str() {
        "aw6060t6" | "en-aw-6060-t6" => "aw6060-t6".into(),
        "aw6061t6" => "aw6061-t6".into(),
        "aw6063t6" => "aw6063-t6".into(),
        "aw6082t6" => "aw6082-t6".into(),
        "aw5083o" => "aw5083-o".into(),
        "aw5083h111" => "aw5083-h111".into(),
        _ => key,
    }
}

pub fn resolve_alloy(designation: &str) -> Option<AlloyProps> {
    let key = alloy_key(designation);
    CATALOGUE_ALLOY_ROWS.iter().find(|(id, _)| *id == key).map(|(_, p)| *p)
}

/// 📏 The band whose upper thickness limit holds `t` (a thickness exactly on the limit belongs to the lower band).
pub fn resolve_alloy_for_thickness(designation: &str, t_m: f64) -> Option<AlloyProps> {
    let key = alloy_key(designation);
    let bands: Vec<AlloyProps> = ALLOY_BANDS.iter().filter(|(id, _)| *id == key).map(|(_, p)| *p).collect();
    bands.iter().copied().find(|p| t_m <= p.t_max_m + 1e-12).or_else(|| bands.last().copied())
}

/// 🔩 The weakest band over every wall thickness of the section (f_o rises with thickness for 6082-T6), so a thin web is never credited with the thick flange's strength.
pub fn resolve_alloy_for_section(designation: &str, section: &AluminiumSection) -> Option<AlloyProps> {
    let mut thicknesses: Vec<f64> = section.elements.iter().map(|e| e.thickness).collect();
    thicknesses.push(section.flange_thickness);
    thicknesses.push(section.web_thickness);
    thicknesses.retain(|t| *t > 0.0);
    thicknesses.iter().filter_map(|t| resolve_alloy_for_thickness(designation, *t)).min_by(|a, b| a.f_o_pa.partial_cmp(&b.f_o_pa).unwrap_or(std::cmp::Ordering::Equal))
}

pub fn max_thickness(section: &AluminiumSection) -> f64 {
    let el = section.elements.iter().map(|e| e.thickness).fold(0.0, f64::max);
    el.max(section.flange_thickness).max(section.web_thickness)
}
//#endregion 🔖️Alloys

//#region 🔖️Classification
pub fn epsilon(f_o_pa: f64) -> f64 { (250.0e6 / f_o_pa).sqrt() }

/// 🔥 HAZ extent b_haz [m] on each side of a MIG weld (EN 1999-1-1 §6.1.6.3).
pub fn haz_width(t_m: f64) -> f64 {
    if t_m <= 0.006 { 0.020 } else if t_m <= 0.012 { 0.030 } else if t_m <= 0.025 { 0.035 } else { 0.040 }
}

/// 🔥 HAZ extent of one welded element: the MIG step value, ×1.5 for TIG and ×1.33 at a T-junction (both factors unverified).
pub fn haz_width_for(el: &PlateElement) -> f64 {
    let process = if el.weld_process == semio_s_artifact_norm_en1999_model::WeldProcess::Tig { 1.5 } else { 1.0 };
    let junction = if el.weld_t_junction { 1.33 } else { 1.0 };
    haz_width(el.thickness) * process * junction
}

/// 📐 An outstand with a complete edge stiffener (lip depth ≥ 5·t_s, t_s ≥ t) behaves like an internal element (unverified rule for §6.1.4.3).
pub fn stiffened_outstand(el: &PlateElement) -> bool {
    el.outstand && el.edge_stiffener_thickness > 0.0 && el.edge_stiffener_thickness >= el.thickness && el.edge_stiffener_depth >= 5.0 * el.edge_stiffener_thickness
}

/// 🧭 Whether the element is classified and reduced as an outstand.
pub fn behaves_outstand(el: &PlateElement) -> bool { el.outstand && !stiffened_outstand(el) }

/// 🧭 β1, β2, β3 of Table 6.2 by buckling class, element support and weld state.
pub fn class_limits(class_a: bool, outstand: bool, welded: bool) -> [f64; 3] {
    match (class_a, outstand, welded) {
        (true, false, false) => [11.0, 16.0, 22.0],
        (true, false, true) => [9.0, 13.0, 18.0],
        (true, true, false) => [3.0, 4.5, 6.0],
        (true, true, true) => [2.5, 4.0, 5.0],
        (false, false, false) => [13.0, 16.5, 18.0],
        (false, false, true) => [10.0, 13.5, 15.0],
        (false, true, false) => [3.5, 4.5, 5.0],
        (false, true, true) => [3.0, 3.5, 4.0],
    }
}

/// 📈 η of an internal element under a stress gradient ψ (EN 1999-1-1 Table 6.2 note).
pub fn stress_gradient_eta(psi: f64) -> f64 {
    if psi > -1.0 { 0.70 + 0.30 * psi } else { 0.80 / (1.0 - psi) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SectionClass { Class1 = 1, Class2 = 2, Class3 = 3, Class4 = 4 }

pub fn class_of(beta_over_eps: f64, limits: [f64; 3]) -> SectionClass {
    if beta_over_eps <= limits[0] { SectionClass::Class1 }
    else if beta_over_eps <= limits[1] { SectionClass::Class2 }
    else if beta_over_eps <= limits[2] { SectionClass::Class3 }
    else { SectionClass::Class4 }
}

pub fn beta(element: &PlateElement) -> f64 {
    if element.thickness <= 0.0 { f64::INFINITY } else { element.width / element.thickness }
}

pub fn element_beta_over_eps(element: &PlateElement, f_o_pa: f64, eta: f64) -> f64 {
    let eta = if !element.outstand && element.id.to_ascii_lowercase().contains("web") { eta } else { 1.0 };
    eta * beta(element) / epsilon(f_o_pa)
}

pub fn classify_element(element: &PlateElement, f_o_pa: f64, class_a: bool, eta: f64) -> SectionClass {
    class_of(element_beta_over_eps(element, f_o_pa, eta), class_limits(class_a, behaves_outstand(element), element.welded))
}

/// 📉 C1, C2 of Table 6.3 for the local buckling reduction.
pub fn rho_constants(class_a: bool, outstand: bool, welded: bool) -> (f64, f64) {
    match (class_a, welded, outstand) {
        (true, false, false) => (32.0, 220.0),
        (true, false, true) => (10.0, 24.0),
        (true, true, false) | (false, false, false) => (29.0, 198.0),
        (true, true, true) | (false, false, true) => (9.0, 20.0),
        (false, true, false) => (25.0, 150.0),
        (false, true, true) => (8.0, 16.0),
    }
}

/// 📉 ρ_c = C1/(β/ε) − C2/(β/ε)² for class 4 elements, else 1 (EN 1999-1-1 §6.1.5).
pub fn rho_c(beta_over_eps: f64, class_a: bool, outstand: bool, welded: bool) -> f64 {
    let limits = class_limits(class_a, outstand, welded);
    if beta_over_eps <= limits[2] { return 1.0; }
    let (c1, c2) = rho_constants(class_a, outstand, welded);
    (c1 / beta_over_eps - c2 / (beta_over_eps * beta_over_eps)).clamp(0.0, 1.0)
}

/// 🔵 Round tube slenderness β = 3·√(D/t) classified with the internal limits.
pub fn tube_beta(section: &AluminiumSection) -> f64 {
    let d = section.outer_diameter.max(section.width);
    let t = section.web_thickness.max(section.flange_thickness);
    if t <= 0.0 { f64::INFINITY } else { 3.0 * (d / t).sqrt() }
}

pub fn is_tube(section: &AluminiumSection) -> bool { matches!(section.kind, semio_s_artifact_norm_en1999_model::SectionKind::Chs) }

pub fn section_class(section: &AluminiumSection, f_o_pa: f64, class_a: bool, eta: f64) -> SectionClass {
    if is_tube(section) {
        return class_of(tube_beta(section) / epsilon(f_o_pa), class_limits(class_a, false, false));
    }
    section.elements.iter().map(|e| classify_element(e, f_o_pa, class_a, eta)).max().unwrap_or(SectionClass::Class1)
}

/// 🧭 The element with the largest β/ε relative to its class-3 limit, with that ratio.
pub fn governing_element(section: &AluminiumSection, f_o_pa: f64, class_a: bool, eta: f64) -> Option<(usize, f64, [f64; 3])> {
    section.elements.iter().enumerate().map(|(i, e)| {
        let lim = class_limits(class_a, behaves_outstand(e), e.welded);
        (i, element_beta_over_eps(e, f_o_pa, eta), lim)
    }).max_by(|a, b| (a.1 / a.2[2]).partial_cmp(&(b.1 / b.2[2])).unwrap_or(std::cmp::Ordering::Equal))
}
//#endregion 🔖️Classification

//#region 🔖️Geometry
/// 📐 Analytic gross properties (A, I_y, I_z, W_el,y, W_el,z, I_t, I_w).
pub fn section_geometry(section: &AluminiumSection) -> (f64, f64, f64, f64, f64, f64, f64) {
    let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
    let hw = (h - 2.0 * tf).max(0.0);
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs => {
            let d = section.outer_diameter.max(section.width);
            let t = section.web_thickness.max(section.flange_thickness);
            let di = (d - 2.0 * t).max(0.0);
            let a = PI / 4.0 * (d * d - di * di);
            let i = PI / 64.0 * (d.powi(4) - di.powi(4));
            let wel = if d > 0.0 { 2.0 * i / d } else { 0.0 };
            let it = PI / 32.0 * (d.powi(4) - di.powi(4));
            (a, i, i, wel, wel, it, 0.0)
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => {
            let a = 2.0 * b * tf + 2.0 * hw * tw;
            let iy = (b * h.powi(3) - (b - 2.0 * tw).max(0.0) * hw.powi(3)) / 12.0;
            let iz = (h * b.powi(3) - hw * (b - 2.0 * tw).max(0.0).powi(3)) / 12.0;
            let wy = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
            let wz = if b > 0.0 { 2.0 * iz / b } else { 0.0 };
            let am = (b - tw).max(0.0) * (h - tf).max(0.0);
            let s_over_t = 2.0 * (b - tw).max(0.0) / tf.max(1e-12) + 2.0 * (h - tf).max(0.0) / tw.max(1e-12);
            let bredt = 4.0 * am * am / s_over_t.max(1e-12);
            (a, iy, iz, wy, wz, solid_constants(section).map_or(bredt, |c| c.j), 0.0)
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => {
            let t = tf.max(tw);
            let a = (b + h - t) * t;
            let iy = t * h.powi(3) / 12.0 + b * t.powi(3) / 12.0;
            let iz = t * b.powi(3) / 12.0 + h * t.powi(3) / 12.0;
            let wy = if h > 0.0 { iy / (h * 0.5) } else { 0.0 };
            let wz = if b > 0.0 { iz / (b * 0.5) } else { 0.0 };
            let (it, iw) = torsion_constants(section);
            (a, iy, iz, wy, wz, it, iw)
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => {
            let a = 2.0 * b * tf + hw * tw;
            let iy = (b * h.powi(3) - (b - tw).max(0.0) * hw.powi(3)) / 12.0;
            let yg = (2.0 * b * tf * (b * 0.5 - tw * 0.5) ) / a.max(1e-12);
            let iz = 2.0 * tf * b.powi(3) / 12.0 + hw * tw.powi(3) / 12.0 + 2.0 * b * tf * (b * 0.5 - tw * 0.5 - yg).powi(2) + hw * tw * yg * yg;
            let wy = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
            let wz = iz / (b - tw * 0.5 - yg).max(1e-12);
            let (it, iw) = torsion_constants(section);
            (a, iy, iz, wy, wz, it, iw)
        }
        _ => {
            let a = 2.0 * b * tf + hw * tw;
            let iy = (b * h.powi(3) - (b - tw).max(0.0) * hw.powi(3)) / 12.0;
            let iz = 2.0 * (tf * b.powi(3) / 12.0) + hw * tw.powi(3) / 12.0;
            let wy = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
            let wz = if b > 0.0 { 2.0 * iz / b } else { 0.0 };
            let (it, iw) = torsion_constants(section);
            (a, iy, iz, wy, wz, it, iw)
        }
    }
}

/// 🔲️ Outline `(outer, holes)` of an extruded section kind in metres (sharp corners, the plate sizes of the snapshot); `None` for round tubes and degenerate sizes.
pub fn solid_outline(section: &AluminiumSection) -> Option<mechanics::section::profiles::Outline> {
    use mechanics::section::profiles as p;
    let (h, b, tf, tw) = (section.height, section.width, section.flange_thickness, section.web_thickness);
    if h <= 0.0 || b <= 0.0 || tw <= 0.0 || tf < 0.0 { return None; }
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs => None,
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs if tf > 0.0 && (tf - tw).abs() < 1e-12 && 2.0 * tf < h.min(b) => Some(p::rectangular_hollow_section(h, b, tf, 0.0, 2)),
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => None,
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => Some(p::angle_section(h, b, tf.max(tw), 0.0, 0.0, 2)),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel if tf > 0.0 => Some(p::channel_section(h, b, tf, tw, 0.0, 2)),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => None,
        _ if tf > 0.0 && 2.0 * tf < h => Some(p::i_section(h, b, tf, tw, 0.0, 2)),
        _ => None,
    }
}

/// 🔲️ Exact St-Venant torsion constant, warping constant, shear centre and shear areas of an extruded section by the finite-element solver [`mechanics::analyse_section`] (EN 1999-1-1 Annex I.1 and 6.3.2.2 take I_t and I_w of the gross section; the thin-walled sum Σ b·t³/3 overestimates fillet-free junctions by 1-4 %). Memoised per kind and plate sizes because the solve meshes the outline.
pub fn solid_constants(section: &AluminiumSection) -> Option<mechanics::SectionConstants> {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<HashMap<(u8, [u64; 4]), Option<mechanics::SectionConstants>>>> = OnceLock::new();
    let key = (section.kind as u8, [section.height.to_bits(), section.width.to_bits(), section.flange_thickness.to_bits(), section.web_thickness.to_bits()]);
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(hit) = cache.lock().ok().and_then(|c| c.get(&key).cloned()) { return hit; }
    let alloy = mechanics::Material { id: "alloy".into(), e: na_de::E_PA, g: na_de::G_PA, rho: 2700.0 };
    let solved = solid_outline(section).and_then(|(outer, holes)| mechanics::analyse_section(&outer, &holes, &alloy).ok());
    if let Ok(mut c) = cache.lock() { if c.len() > 512 { c.clear(); } c.insert(key, solved.clone()); }
    solved
}

/// 🔲️ Points per quarter arc of the filleted extrusion outlines (the discretisation of the blessed `sectionproperties` cases).
pub const ARC_POINTS: usize = 16;

/// 🔲️ Elastic and plastic constants, shear centre and principal moments of a filleted or elliptical extrusion outline by the finite-element solver [`mechanics::analyse_section`]; the profile constructors below take the root and toe radii the sharp-corner [`section_geometry`] ignores.
pub fn outline_constants(outline: &mechanics::section::profiles::Outline) -> Option<mechanics::SectionConstants> {
    let alloy = mechanics::Material { id: "alloy".into(), e: na_de::E_PA, g: na_de::G_PA, rho: 2700.0 };
    mechanics::analyse_section(&outline.0, &outline.1, &alloy).ok()
}

/// 🔱️ Constants of an extruded tee (flange `b` x `tf` on top, stem `tw`, overall depth `h`, root radius `r`).
pub fn tee_constants(h: f64, b: f64, tf: f64, tw: f64, r: f64) -> Option<mechanics::SectionConstants> {
    outline_constants(&mechanics::section::profiles::tee_section(h, b, tf, tw, r, ARC_POINTS))
}

/// ⭕️ Constants of an extruded elliptical hollow section with outer axes `d_x` x `d_y` and wall `t`.
pub fn elliptical_constants(d_x: f64, d_y: f64, t: f64) -> Option<mechanics::SectionConstants> {
    outline_constants(&mechanics::section::profiles::elliptical_hollow_section(d_x, d_y, t, 8 * ARC_POINTS))
}

/// 🚪️ Constants of an extruded channel with root radius `r`; `shear_centre_y - centroid_y` is the shear-centre offset y_sc.
pub fn channel_constants(h: f64, b: f64, tf: f64, tw: f64, r: f64) -> Option<mechanics::SectionConstants> {
    outline_constants(&mechanics::section::profiles::channel_section(h, b, tf, tw, r, ARC_POINTS))
}

/// 📐️ Constants of an extruded angle with root radius `r_root` and toe radius `r_toe`; `i_major` and `i_minor` are the principal moments.
pub fn angle_constants(h: f64, b: f64, t: f64, r_root: f64, r_toe: f64) -> Option<mechanics::SectionConstants> {
    outline_constants(&mechanics::section::profiles::angle_section(h, b, t, r_root, r_toe, ARC_POINTS))
}

/// 🔲️ `(I_t, I_w)` of an extruded section: the finite-element solution of [`solid_constants`], else the thin-walled centreline routine.
pub fn torsion_constants(section: &AluminiumSection) -> (f64, f64) {
    solid_constants(section).map_or_else(|| thin_walled_torsion(section), |c| (c.j, c.iw))
}

/// 📜 `(I_t, I_w)` by the thin-walled plate idealisation the worked example E2-07 prescribes: I_t = Σ b·t³/3 over the plates (web clear height h − 2·t_f) and I_w = t_f·b³/24·(h − t_f)² for the doubly symmetric I; other open kinds use [`thin_walled_torsion`]. Overestimates the exact [`torsion_constants`] by 1-4 %.
pub fn thin_walled_idealisation(section: &AluminiumSection) -> (f64, f64) {
    let (h, b, tf, tw) = (section.height, section.width, section.flange_thickness, section.web_thickness);
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Channel | semio_s_artifact_norm_en1999_model::SectionKind::Angle | semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => thin_walled_torsion(section),
        _ => ((2.0 * b * tf.powi(3) + (h - 2.0 * tf).max(0.0) * tw.powi(3)) / 3.0, tf * b.powi(3) / 24.0 * (h - tf).powi(2)),
    }
}

/// 🪶️ Centreline walls of an open thin-walled extrusion (`extrudedI` and the default I/H, `channel`, `angle`) in the axes of [`mechanics::ThinWall`] (y horizontal, z up); `None` for closed and round sections.
pub fn thin_walls(section: &AluminiumSection) -> Option<Vec<mechanics::ThinWall>> {
    let (h, b, tf, tw) = (section.height, section.width, section.flange_thickness, section.web_thickness);
    if h <= 0.0 || b <= 0.0 || tw <= 0.0 { return None; }
    let wall = |y1: f64, z1: f64, y2: f64, z2: f64, thickness: f64| mechanics::ThinWall { y1, z1, y2, z2, thickness };
    let zt = 0.5 * (h - tf);
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => None,
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => {
            let t = tf.max(tw);
            Some(vec![wall(b, 0.5 * t, 0.5 * t, 0.5 * t, t), wall(0.5 * t, 0.5 * t, 0.5 * t, h, t)])
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Channel if tf > 0.0 => Some(vec![wall(b, -zt, 0.5 * tw, -zt, tf), wall(0.5 * tw, -zt, 0.5 * tw, zt, tw), wall(0.5 * tw, zt, b, zt, tf)]),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => None,
        _ if tf > 0.0 => Some(vec![wall(-0.5 * b, -zt, 0.5 * b, -zt, tf), wall(0.0, -zt, 0.0, zt, tw), wall(-0.5 * b, zt, 0.5 * b, zt, tf)]),
        _ => None,
    }
}

/// 🪶️ Section constants of an open thin-walled extrusion from [`mechanics::analyse_thin_walled`]: St Venant I_t, warping I_w about the shear centre and the shear centre itself.
pub fn thin_walled_constants(section: &AluminiumSection) -> Option<mechanics::SectionConstants> {
    let alloy = mechanics::Material { id: "alloy".into(), e: na_de::E_PA, g: na_de::G_PA, rho: 2700.0 };
    mechanics::analyse_thin_walled(&thin_walls(section)?, &alloy).ok()
}

/// 🪶️ `(I_t, I_w)` of an open thin-walled extrusion by the mechanics kernel; `(0, 0)` for a degenerate wall set.
pub fn thin_walled_torsion(section: &AluminiumSection) -> (f64, f64) {
    thin_walled_constants(section).map_or((0.0, 0.0), |c| (c.j, c.iw))
}

/// 🪶️ Distance `(e_y, e_z)` [m] from the centroid to the shear centre of an extrusion (finite-element solution, thin-walled walls as fallback); zero for doubly symmetric and closed sections.
pub fn shear_centre_offset(section: &AluminiumSection) -> (f64, f64) {
    solid_constants(section).or_else(|| thin_walled_constants(section)).map_or((0.0, 0.0), |c| (c.shear_centre_y - c.centroid_y, c.shear_centre_z - c.centroid_z))
}

/// 🧮 Plastic moduli (W_pl,y, W_pl,z); sections without a closed form return their elastic moduli.
pub fn plastic_moduli(section: &AluminiumSection) -> (f64, f64) {
    let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
    let hw = (h - 2.0 * tf).max(0.0);
    let (_, _, _, wy, wz, _, _) = section_geometry(section);
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs => {
            let d = section.outer_diameter.max(section.width);
            let t = section.web_thickness.max(section.flange_thickness);
            let w = (d.powi(3) - (d - 2.0 * t).max(0.0).powi(3)) / 6.0;
            (w, w)
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => (b * tf * (h - tf) + tw * hw * hw / 2.0, hw * tw * (b - tw) + tf * b * b / 2.0),
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => (wy, wz),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => (b * tf * (h - tf) + tw * hw * hw / 4.0, wz),
        _ => (b * tf * (h - tf) + tw * hw * hw / 4.0, tf * b * b / 2.0 + hw * tw * tw / 4.0),
    }
}
//#endregion 🔖️Geometry

//#region 🔖️EffectiveSection
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role { Flange, Web, Solid }

/// 🟫️ A rectangular strip of the section; `thin_z` names the direction in which the effective thickness reduces.
#[derive(Clone, Copy, Debug)]
pub struct Strip { pub yc: f64, pub zc: f64, pub dy: f64, pub dz: f64, pub thin_z: bool, pub factor: f64, pub role: Role }

impl Strip {
    fn extents(&self) -> (f64, f64) {
        if self.thin_z { (self.dy, self.dz * self.factor) } else { (self.dy * self.factor, self.dz) }
    }
}

/// 🎛️ What reduces the thickness of a strip: local buckling (ρ_c) and/or a HAZ factor on the strips inside the HAZ; `haz_extents` lists the plate elements whose weld runs declare their own b_haz [m] instead of the §6.1.6.3 table.
#[derive(Clone, Copy, Debug)]
pub struct EffCtx<'a> { pub f_o_pa: f64, pub class_a: bool, pub eta_web: f64, pub local: bool, pub haz_rho: Option<f64>, pub haz_extents: &'a [(String, f64)] }

impl EffCtx<'_> {
    pub fn gross(f_o_pa: f64, class_a: bool) -> Self { Self { f_o_pa, class_a, eta_web: 1.0, local: false, haz_rho: None, haz_extents: &[] } }

    /// 🔥 b_haz [m] per side of one welded element: the declared extent of its weld run, else the §6.1.6.3 table value.
    pub fn haz_extent_of(&self, el: &PlateElement) -> f64 {
        self.haz_extents.iter().find(|(id, _)| *id == el.id).map_or_else(|| haz_width_for(el), |(_, b)| *b)
    }
}

fn element_factors(el: &PlateElement, ctx: &EffCtx) -> Vec<(f64, f64, f64)> {
    let w = el.width.max(0.0);
    let rho = if ctx.local { rho_c(element_beta_over_eps(el, ctx.f_o_pa, ctx.eta_web), ctx.class_a, behaves_outstand(el), el.welded) } else { 1.0 };
    let (Some(rho_haz), true) = (ctx.haz_rho, el.welded) else { return vec![(0.0, w, rho)] };
    let bhaz = ctx.haz_extent_of(el);
    let transverse = el.weld_direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse;
    let h0 = if transverse { 0.0 } else { (el.weld_position - bhaz).max(0.0).min(w) };
    let h1 = if transverse { w } else { (el.weld_position + bhaz).max(0.0).min(w) };
    let mut parts = Vec::new();
    if h0 > 1e-12 { parts.push((0.0, h0, rho)); }
    if h1 > h0 { parts.push((h0, h1, rho.min(rho_haz))); }
    if w - h1 > 1e-12 { parts.push((h1, w, rho)); }
    if parts.is_empty() { parts.push((0.0, w, rho)); }
    parts
}

fn pick<'a>(elements: &'a [PlateElement], outstand: bool, which: usize) -> Option<&'a PlateElement> {
    let list: Vec<&PlateElement> = elements.iter().filter(|e| e.outstand == outstand).collect();
    list.get(which).or_else(|| list.first()).copied()
}

fn pick_named<'a>(elements: &'a [PlateElement], web: bool, which: usize) -> Option<&'a PlateElement> {
    let list: Vec<&PlateElement> = elements.iter().filter(|e| e.id.to_ascii_lowercase().contains("web") == web).collect();
    list.get(which).or_else(|| list.first()).copied()
}

fn flange_strips(out: &mut Vec<Strip>, el: Option<&PlateElement>, ctx: &EffCtx, root: f64, dir: f64, zc: f64, t: f64, width: f64) {
    match el {
        Some(e) => {
            for (s0, s1, f) in element_factors(e, ctx) {
                let (a, b) = (root + dir * s0, root + dir * s1);
                out.push(Strip { yc: 0.5 * (a + b), zc, dy: (s1 - s0).abs(), dz: t, thin_z: true, factor: f, role: Role::Flange });
            }
        }
        None => out.push(Strip { yc: root + dir * 0.5 * width, zc, dy: width, dz: t, thin_z: true, factor: 1.0, role: Role::Flange }),
    }
}

fn web_strips(out: &mut Vec<Strip>, el: Option<&PlateElement>, ctx: &EffCtx, yc: f64, z_root: f64, t: f64, height: f64) {
    match el {
        Some(e) => {
            for (s0, s1, f) in element_factors(e, ctx) {
                let (a, b) = (z_root - s0, z_root - s1);
                out.push(Strip { yc, zc: 0.5 * (a + b), dy: t, dz: (s1 - s0).abs(), thin_z: false, factor: f, role: Role::Web });
            }
        }
        None => out.push(Strip { yc, zc: z_root - 0.5 * height, dy: t, dz: height, thin_z: false, factor: 1.0, role: Role::Web }),
    }
}

/// 🧩️ Decomposes an open or box section into strips whose thickness carries the element reduction.
pub fn strips(section: &AluminiumSection, ctx: &EffCtx) -> Vec<Strip> {
    let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
    let hw = (h - 2.0 * tf).max(0.0);
    let els = &section.elements;
    let mut out = Vec::new();
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => {
            let clear = (b - 2.0 * tw).max(0.0);
            for (i, sign) in [(0usize, 1.0f64), (1, -1.0)] {
                let zc = sign * (h - tf) * 0.5;
                flange_strips(&mut out, pick_named(els, false, i), ctx, -clear * 0.5, 1.0, zc, tf, clear);
                for side in [-1.0f64, 1.0] {
                    out.push(Strip { yc: side * (b - tw) * 0.5, zc, dy: tw, dz: tf, thin_z: true, factor: 1.0, role: Role::Solid });
                }
            }
            for (i, side) in [(0usize, -1.0f64), (1, 1.0)] {
                web_strips(&mut out, pick_named(els, true, i), ctx, side * (b - tw) * 0.5, hw * 0.5, tw, hw);
            }
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => {
            let t = tf.max(tw);
            out.push(Strip { yc: t * 0.5, zc: t * 0.5, dy: t, dz: t, thin_z: true, factor: 1.0, role: Role::Solid });
            let v = pick(els, true, 0);
            let hz = pick(els, true, 1);
            match v {
                Some(e) => {
                    for (s0, s1, f) in element_factors(e, ctx) {
                        out.push(Strip { yc: t * 0.5, zc: t + 0.5 * (s0 + s1), dy: t, dz: s1 - s0, thin_z: false, factor: f, role: Role::Web });
                    }
                }
                None => out.push(Strip { yc: t * 0.5, zc: t + 0.5 * (h - t), dy: t, dz: (h - t).max(0.0), thin_z: false, factor: 1.0, role: Role::Web }),
            }
            match hz {
                Some(e) => {
                    for (s0, s1, f) in element_factors(e, ctx) {
                        out.push(Strip { yc: t + 0.5 * (s0 + s1), zc: t * 0.5, dy: s1 - s0, dz: t, thin_z: true, factor: f, role: Role::Flange });
                    }
                }
                None => out.push(Strip { yc: t + 0.5 * (b - t), zc: t * 0.5, dy: (b - t).max(0.0), dz: t, thin_z: true, factor: 1.0, role: Role::Flange }),
            }
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => {
            let wo = (b - tw).max(0.0);
            for (i, sign) in [(0usize, 1.0f64), (1, -1.0)] {
                let zc = sign * (h - tf) * 0.5;
                flange_strips(&mut out, pick(els, true, i), ctx, tw * 0.5, 1.0, zc, tf, wo);
                out.push(Strip { yc: 0.0, zc, dy: tw, dz: tf, thin_z: true, factor: 1.0, role: Role::Solid });
            }
            web_strips(&mut out, pick(els, false, 0), ctx, 0.0, hw * 0.5, tw, hw);
        }
        _ => {
            let wo = ((b - tw) * 0.5).max(0.0);
            for (i, sign) in [(0usize, 1.0f64), (1, -1.0)] {
                let zc = sign * (h - tf) * 0.5;
                flange_strips(&mut out, pick(els, true, i), ctx, tw * 0.5, 1.0, zc, tf, wo);
                flange_strips(&mut out, pick(els, true, i), ctx, -tw * 0.5, -1.0, zc, tf, wo);
                out.push(Strip { yc: 0.0, zc, dy: tw, dz: tf, thin_z: true, factor: 1.0, role: Role::Solid });
            }
            web_strips(&mut out, pick(els, false, 0), ctx, 0.0, hw * 0.5, tw, hw);
        }
    }
    out
}

/// 📊 Effective properties of a section: areas, second moments about the centroid, elastic moduli to the nominal outer fibres and the shear area.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Props { pub a: f64, pub iy: f64, pub iz: f64, pub wy: f64, pub wz: f64, pub yg: f64, pub zg: f64, pub a_v: f64, pub a_f: f64 }

pub fn props(section: &AluminiumSection, ctx: &EffCtx) -> Props {
    if is_tube(section) {
        let f = if ctx.local {
            let b = tube_beta(section) / epsilon(ctx.f_o_pa);
            rho_c(b, ctx.class_a, false, false)
        } else { 1.0 };
        let (a, i, _, w, _, _, _) = section_geometry(section);
        return Props { a: a * f, iy: i * f, iz: i * f, wy: w * f, wz: w * f, yg: 0.0, zg: 0.0, a_v: 2.0 * a * f / PI, a_f: 2.0 * a * f / PI };
    }
    let ss = strips(section, ctx);
    let (mut a, mut sy, mut sz, mut a_v, mut a_f) = (0.0, 0.0, 0.0, 0.0, 0.0);
    let (mut ymin, mut ymax, mut zmin, mut zmax) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for s in &ss {
        let (dy, dz) = s.extents();
        let ar = dy * dz;
        a += ar; sy += ar * s.yc; sz += ar * s.zc;
        if s.role == Role::Web { a_v += ar; } else { a_f += ar; }
        ymin = ymin.min(s.yc - s.dy * 0.5); ymax = ymax.max(s.yc + s.dy * 0.5);
        zmin = zmin.min(s.zc - s.dz * 0.5); zmax = zmax.max(s.zc + s.dz * 0.5);
    }
    if a <= 0.0 { return Props::default(); }
    let (yg, zg) = (sy / a, sz / a);
    let (mut iy, mut iz) = (0.0, 0.0);
    for s in &ss {
        let (dy, dz) = s.extents();
        let ar = dy * dz;
        iy += dy * dz.powi(3) / 12.0 + ar * (s.zc - zg).powi(2);
        iz += dz * dy.powi(3) / 12.0 + ar * (s.yc - yg).powi(2);
    }
    let wy = iy / (zmax - zg).max(zg - zmin).max(1e-12);
    let wz = iz / (ymax - yg).max(yg - ymin).max(1e-12);
    Props { a, iy, iz, wy, wz, yg, zg, a_v, a_f }
}

/// 🧮 Plastic-to-elastic shape factor α of §6.2.5 (class 1/2: W_pl/W_el capped at `alpha_max`, class 3: interpolation on β, class 4: 1).
pub fn shape_factor(class: SectionClass, w_pl: f64, w_el: f64, beta_over_eps: f64, limits: [f64; 3]) -> f64 {
    let alpha_max = semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.alpha-max");
    let a12 = if w_el > 0.0 { (w_pl / w_el).clamp(1.0, alpha_max) } else { 1.0 };
    match class {
        SectionClass::Class1 | SectionClass::Class2 => a12,
        SectionClass::Class3 => {
            let span = (limits[2] - limits[1]).max(1e-12);
            (1.0 + (limits[2] - beta_over_eps) / span * (a12 - 1.0)).clamp(1.0, a12)
        }
        SectionClass::Class4 => 1.0,
    }
}
//#endregion 🔖️EffectiveSection

//#region 🔖️Stability
/// 📈 Buckling curve of Table 6.6: (α, λ̄0) for flexural buckling.
pub fn buckling_curve(class_a: bool) -> (f64, f64) {
    if class_a { (0.20, 0.10) } else { (0.32, 0.00) }
}

/// 🌀 Torsional buckling curve: α of the buckling class and λ̄0 = 0.2.
pub fn torsional_curve(class_a: bool) -> (f64, f64) { (buckling_curve(class_a).0, 0.20) }

/// 🌀 Lateral-torsional buckling curve of §6.3.2: class 1/2 (0.10, 0.6), class 3/4 (0.20, 0.4).
pub fn ltb_curve(class: SectionClass) -> (f64, f64) {
    if class <= SectionClass::Class2 { (0.10, 0.6) } else { (0.20, 0.4) }
}

/// 📉 Reduction χ from λ̄ (EN 1999-1-1 §6.3.1) — never an input.
pub fn chi_from_lambda(lambda_bar: f64, alpha: f64, lambda_0: f64) -> f64 {
    if lambda_bar <= lambda_0 { return 1.0; }
    let phi = 0.5 * (1.0 + alpha * (lambda_bar - lambda_0) + lambda_bar * lambda_bar);
    let denom = phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt();
    if denom <= 0.0 { 1.0 } else { (1.0 / denom).min(1.0) }
}

pub fn n_cr(e: f64, i: f64, l_cr: f64) -> f64 {
    if l_cr <= 0.0 { return f64::INFINITY; }
    PI.powi(2) * e * i / (l_cr * l_cr)
}

/// 📏 Relative slenderness λ̄ = √(A·f_o/N_cr) from the buckling length, the second moment and the area.
pub fn lambda_bar(l_cr: f64, i: f64, a: f64, f_o_pa: f64) -> f64 {
    if i <= 0.0 || a <= 0.0 { return 0.0; }
    let n = n_cr(na_de::E_PA, i, l_cr);
    if !n.is_finite() { return 0.0; }
    (a * f_o_pa / n).sqrt()
}

/// 🌀 N_cr,T = (G·I_t + π²·E·I_w/L²)/i_0² with i_0² = (I_y + I_z)/A + e_y² + e_z² (`offset` = centroid to shear centre, see [`shear_centre_offset`]).
pub fn n_cr_torsional(a: f64, i_t: f64, i_w: f64, i_y: f64, i_z: f64, offset: (f64, f64), l_cr_t: f64) -> f64 {
    let i0_sq = (i_y + i_z) / a.max(1e-18) + offset.0 * offset.0 + offset.1 * offset.1;
    if l_cr_t <= 0.0 || i0_sq <= 0.0 { return f64::INFINITY; }
    (na_de::G_PA * i_t + PI.powi(2) * na_de::E_PA * i_w / (l_cr_t * l_cr_t)) / i0_sq
}

/// 📐 M_cr = C1·π²·E·I_z/L²·√(I_w/I_z + L²·G·I_t/(π²·E·I_z)) of a doubly symmetric section with fork supports and the load at the shear centre; see [`m_cr_general`].
pub fn m_cr_ltb(c1: f64, e: f64, i_z: f64, i_w: f64, i_t: f64, l_lt: f64) -> f64 {
    m_cr_general(c1, 0.0, 0.0, e, i_z, i_w, i_t, l_lt, 0.0, 0.0)
}

/// ⚠️ UNVERIFIED SIGN CONVENTION: `📐️specs/en1994-en1999-target.md` line 358 prints only `M_cr = C1·π²·E·I_z/L²·√(I_w/I_z + L²·G·I_t/(π²·E·I_z))` plus "za/zg terms from Annex I"; neither the C2/C3 factors nor the sign of z_g and z_j are stated there, so the convention below (z up from the centroid, compression flange on top, z_j positive towards the compression flange) is the EN 1993 / SN003 one and is not confirmed against EN 1999-1-1 Annex I.
/// 📐 General elastic critical moment of a mono-symmetric section with fork supports (EN 1999-1-1 Annex I.1, form of NCCI SN003 / EN 1993-1-1 Annex F, unverified for EN 1999): M_cr = C1·π²·E·I_z/L²·{√(I_w/I_z + L²·G·I_t/(π²·E·I_z) + (C2·z_g − C3·z_j)²) − (C2·z_g − C3·z_j)}, `z_g` = load level above the shear centre, `z_j` from [`wagner_zj`]; z_g = z_j = 0 gives the doubly symmetric formula.
pub fn m_cr_general(c1: f64, c2: f64, c3: f64, e: f64, i_z: f64, i_w: f64, i_t: f64, l_lt: f64, z_g: f64, z_j: f64) -> f64 {
    if l_lt <= 0.0 || i_z <= 0.0 { return f64::INFINITY; }
    let x = c2 * z_g - c3 * z_j;
    let root = (i_w / i_z + (l_lt * l_lt * na_de::G_PA * i_t) / (PI.powi(2) * e * i_z) + x * x).max(0.0).sqrt();
    c1 * (PI.powi(2) * e * i_z / (l_lt * l_lt)) * (root - x)
}

/// 📐 Monosymmetry constant z_j = z_s − ½/I_y·∫z·(y²+z²) dA [m] of a polygon with holes (`z` up from the centroid, the compression flange on top, `shear_centre_z` in the polygon's own coordinates); the integral is exact (degree-3 triangle rule on the fan from the centroid). Positive when the larger flange is compressed.
pub fn wagner_zj(outer: &[[f64; 2]], holes: &[Vec<[f64; 2]>], shear_centre_z: f64) -> f64 {
    let signed = |r: &[[f64; 2]]| -> f64 { (0..r.len()).map(|i| { let (a, b) = (r[i], r[(i + 1) % r.len()]); a[0] * b[1] - b[0] * a[1] }).sum::<f64>() / 2.0 };
    let rings: Vec<(Vec<[f64; 2]>, f64)> = std::iter::once((outer.to_vec(), 1.0)).chain(holes.iter().map(|h| (h.clone(), -1.0))).map(|(r, w)| { let s = signed(&r).signum(); (r, w * s) }).collect();
    let (mut area, mut cy, mut cz) = (0.0, 0.0, 0.0);
    for (r, w) in &rings {
        for i in 0..r.len() {
            let (a, b) = (r[i], r[(i + 1) % r.len()]);
            let cross = a[0] * b[1] - b[0] * a[1];
            area += w * cross / 2.0; cy += w * cross * (a[0] + b[0]) / 6.0; cz += w * cross * (a[1] + b[1]) / 6.0;
        }
    }
    let (cy, cz) = (cy / area, cz / area);
    let rule = |a: [f64; 2], b: [f64; 2], f: &dyn Fn(f64, f64) -> f64| -> f64 {
        let tri = ((a[0] * b[1] - b[0] * a[1]) / 2.0).abs();
        let mid = f((a[0] + b[0]) / 3.0, (a[1] + b[1]) / 3.0);
        let pts = [[0.6, 0.2, 0.2], [0.2, 0.6, 0.2], [0.2, 0.2, 0.6]];
        let sum: f64 = pts.iter().map(|w| f(w[1] * a[0] + w[2] * b[0], w[1] * a[1] + w[2] * b[1])).sum();
        tri * (-27.0 / 48.0 * mid + 25.0 / 48.0 * sum)
    };
    let (mut iy, mut cubic) = (0.0, 0.0);
    for (r, w) in &rings {
        for i in 0..r.len() {
            let a = [r[i][0] - cy, r[i][1] - cz];
            let b = [r[(i + 1) % r.len()][0] - cy, r[(i + 1) % r.len()][1] - cz];
            let side = w * ((a[0] * b[1] - b[0] * a[1]).signum());
            iy += side * rule(a, b, &|_, z| z * z);
            cubic += side * rule(a, b, &|y, z| z * (y * y + z * z));
        }
    }
    (shear_centre_z - cz) - 0.5 * cubic / iy
}

/// 📐 z_j of an extruded section from its finite-element shear centre and outline; zero for doubly symmetric sections and for channels and tees bent about their axis of symmetry, `0` without an outline.
pub fn section_zj(section: &AluminiumSection) -> f64 {
    match (solid_outline(section), solid_constants(section)) {
        (Some((outer, holes)), Some(c)) => wagner_zj(&outer, &holes, c.shear_centre_z),
        _ => 0.0,
    }
}

/// 📐 `(M_cr, z_j)` of a free outline (`outer`, `holes` in metres, `z` up, compression flange on top) with fork supports of length `l_lt` and the load at the shear centre, from the finite-element I_z, I_t, I_w and shear centre; `None` when the solver refuses the outline.
pub fn m_cr_of_outline(outer: &[[f64; 2]], holes: &[Vec<[f64; 2]>], c1: f64, l_lt: f64) -> Option<(f64, f64)> {
    let alloy = mechanics::Material { id: "alloy".into(), e: na_de::E_PA, g: na_de::G_PA, rho: 2700.0 };
    let c = mechanics::analyse_section(outer, holes, &alloy).ok()?;
    let zj = wagner_zj(outer, holes, c.shear_centre_z);
    Some((m_cr_general(c1, 0.0, 1.0, na_de::E_PA, c.iz, c.iw, c.j, l_lt, 0.0, zj), zj))
}

/// 📐 M_cr of an extruded section under the given moment shape factor `c1` and fork-support length `l_lt`: gross I_z, I_t, I_w of [`section_geometry`] with the monosymmetry term z_j of [`section_zj`], load at the shear centre.
pub fn m_cr_of(section: &AluminiumSection, c1: f64, l_lt: f64) -> f64 {
    let (_, _, i_z, _, _, i_t, i_w) = section_geometry(section);
    m_cr_general(c1, 0.0, 1.0, na_de::E_PA, i_z, i_w, i_t, l_lt, 0.0, section_zj(section))
}

/// 🔗 κ of members with longitudinal welds: κ0 = A_eff,haz/A_eff at λ̄ ≤ 0.2, rising linearly to 1 at λ̄ = 1.2 (unverified interpolation).
pub fn kappa(kappa_0: f64, lambda_bar: f64) -> f64 {
    let k0 = kappa_0.clamp(0.0, 1.0);
    let s = ((lambda_bar - 0.2) / 1.0).clamp(0.0, 1.0);
    k0 + (1.0 - k0) * s
}

/// 🔗 ω_x of a localized transverse weld at distance `xs_from_end` from the nearest end of a buckling length `lc`: the section at the weld keeps ρ_o,haz of the resistance, which governs a stocky member (κ·χ near 1) and fades for a slender one; the effect is weighted by sin(π·xs/lc)^c (unverified, replaces the spec formula that cannot drop below 1).
pub fn omega_x(kappa_chi: f64, rho_haz: f64, xs_from_end: f64, lc: f64, c: f64) -> f64 {
    if lc <= 0.0 { return 1.0; }
    let local = (rho_haz / kappa_chi.max(1e-9)).min(1.0);
    let s = (PI * xs_from_end / lc).sin().abs().powf(c);
    1.0 - (1.0 - local) * s
}

/// 🔀 Exponents (ψ, η, ξ) of the class 1/2 interaction (N/N_Rd)^ψ + (M_y/M_y,Rd)^η + (M_z/M_z,Rd)^ξ ≤ 1 (§6.2.9), derived from the plastic N–M interaction of the shape and rounded to the safe side: I and channel ψ = 2 (M_N ≥ M_pl·(1 − n²)), round tube ψ = 1.5 (cos(πn/2) ≥ 1 − n^1.5), box and angle ψ = 1; η = ξ = 1 (unverified).
pub fn section_interaction_exponents(kind: semio_s_artifact_norm_en1999_model::SectionKind) -> (f64, f64, f64) {
    use semio_s_artifact_norm_en1999_model::SectionKind as K;
    match kind {
        K::Chs => (1.5, 1.0, 1.0),
        K::Box | K::Rhs | K::Angle => (1.0, 1.0, 1.0),
        _ => (2.0, 1.0, 1.0),
    }
}

/// 🔀 η0, γ0, ξ0 of §6.3.3 (unverified; the same base for all classes until the table is transcribed).
pub fn member_interaction_base(_class: SectionClass) -> (f64, f64, f64) { (0.8, 0.8, 0.8) }

/// 🔀 η_c, γ_c, ξ_c of §6.3.3.1, never below 1 (conservative lower bound while η0, γ0, ξ0 are unverified).
pub fn member_interaction_exponents(class: SectionClass, chi_z: f64) -> (f64, f64, f64) {
    let (e0, g0, x0) = member_interaction_base(class);
    ((e0 * chi_z).max(1.0), g0.max(1.0), (x0 * chi_z).max(1.0))
}

/// 🛡️ Shear buckling factor ρ_v of a web (§6.7.4): λ̄_w = 0.35·(h_w/t_w)·√(f_o/E)·√(5.34/k_τ) with k_τ = 5.34 + 4·(h_w/a)² for transverse stiffeners at spacing `a` (a = 0: none).
pub fn shear_buckling_rho(hw_over_tw: f64, f_o_pa: f64, hw: f64, a: f64) -> f64 {
    let k_tau = if a > 0.0 && a >= hw { 5.34 + 4.0 * (hw / a).powi(2) } else if a > 0.0 { 4.0 + 5.34 * (hw / a).powi(2) } else { 5.34 };
    let lw = 0.35 * hw_over_tw * (f_o_pa / na_de::E_PA).sqrt() * (5.34 / k_tau).sqrt();
    if lw <= 0.83 { 1.0 } else { 0.83 / lw }
}

/// 🌀 Torsion modulus W_t [m³]: closed sections 2·A_m·t_min (round tube 2π·r_m²·t), open sections I_t/t_max (St. Venant only, warping neglected).
pub fn torsion_modulus(section: &AluminiumSection) -> f64 {
    let (tf, tw) = (section.flange_thickness, section.web_thickness);
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs => {
            let d = section.outer_diameter.max(section.width);
            let t = tw.max(tf);
            2.0 * PI * ((d - t) * 0.5).powi(2) * t
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => 2.0 * (section.width - tw).max(0.0) * (section.height - tf).max(0.0) * tf.min(tw),
        _ => {
            let (_, _, _, _, _, it, _) = section_geometry(section);
            it / tf.max(tw).max(1e-12)
        }
    }
}

/// 📍 Patch load resistance F_Rd = χ_F·ℓ_y·t_w·f_o/γ_M1 of an unstiffened (or `a`-stiffened) web (EN 1999-1-1 §6.7.5, form of the plate-buckling patch rule; unverified coefficients).
pub fn patch_load_resistance(hw: f64, tw: f64, bf: f64, tf: f64, ss: f64, a: f64, f_o_pa: f64, gamma_m1: f64) -> f64 {
    if hw <= 0.0 || tw <= 0.0 { return 0.0; }
    let kf = 6.0 + if a > 0.0 { 2.0 * (hw / a).powi(2) } else { 0.0 };
    let f_cr = 0.9 * kf * na_de::E_PA * tw.powi(3) / hw;
    let m1 = bf / tw;
    let ly = |m2: f64| (ss + 2.0 * tf * (1.0 + (m1 + m2).sqrt())).min(if a > 0.0 { a } else { f64::INFINITY });
    let lam = |l: f64| (l * tw * f_o_pa / f_cr).sqrt();
    let mut l = ly(0.0);
    let mut lf = lam(l);
    if lf > 0.5 {
        l = ly(0.02 * (hw / tf.max(1e-12)).powi(2));
        lf = lam(l);
    }
    let chi = (0.5 / lf.max(1e-9)).min(1.0);
    f_o_pa * chi * l * tw / gamma_m1
}
//#endregion 🔖️Stability

/// 🏭️ Product forms of Table 3.2 (sheet/strip/plate, extruded profile, extruded tube, extruded rod/bar, drawn tube, forging).
/// 🏭️ Whether a product form can supply a hollow circular section (extruded hollow profile, extruded tube or drawn tube).
pub fn form_supplies_tube(form: semio_s_artifact_norm_en1999_model::ProductForm) -> bool { matches!(form, semio_s_artifact_norm_en1999_model::ProductForm::ExtrudedProfile | semio_s_artifact_norm_en1999_model::ProductForm::ExtrudedTube | semio_s_artifact_norm_en1999_model::ProductForm::DrawnTube) }
