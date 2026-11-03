//! 🌍️ Geotechnical diagrams of EN 1998-5: the liquefaction profile with F_L over the depth (Annex B and Youd et al. 2001), the Annex F seismic bearing surface with the design point, and the inertial and kinematic pile moments over the depth. Every diagram carries a table twin and is derived with the schema functions of the checks.

use super::model::{En1998Foundation, En1998Slope, En1998Snapshot};
use super::norm::{ndp_number, CheckReport};
use super::response::{annex_params, site, Site};
use super::scene::projection_kit::diagram_parts::{self, column, stroke};
use super::scene::projection_kit::loc;
use super::scene::*;
use super::schema::{liquefaction_hazard as hazard, part_5};

const FACTOR_MAX: f64 = 4.0;
const RATIO_MAX: f64 = 1.0;
const DEPTH_STEP: f64 = 0.5;
const SURFACE_STEPS: usize = 40;
const SLOPE_STEPS: usize = 24;

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: Option<(f64, f64)>) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, domain)
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn check_of(report: &CheckReport, id: &str) -> (Option<f64>, Option<String>) {
    let found = report.checks.iter().find(|c| c.id == id);
    (found.map(|c| c.utilization).filter(|u| u.is_finite()), found.map(|c| c.id.clone()))
}

fn focus(f: &En1998Foundation) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Solid, &f.id, format!("foundations[id={}]", f.id)))
}

/// 🌍️ The geotechnical diagrams of every foundation that carries the inputs, in document order.
pub fn diagrams(document: &En1998Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    let site = site(document);
    document.foundations.iter().flat_map(|f| [liquefaction_profile(document, report, &site, f), soil_profile_diagram(document, report, &site, f), liquefaction_ratios(document, report, &site, f), annex_f_surface(document, report, &site, f), pile_profile(document, report, &site, f)]).flatten().chain(document.slopes.iter().filter_map(|sl| slope_factor_curve(document, report, &site, sl))).collect()
}

/// 🌊️ F_L over the depth: the Annex B factor and the Youd et al. factor of the layer resistance, solid inside the layer and dotted where the resistance of the layer is only extrapolated, with the required factor and the layer marker.
pub fn liquefaction_profile(document: &En1998Snapshot, report: &CheckReport, site: &Site, f: &En1998Foundation) -> Option<Diagram2d> {
    if f.liq_n1_60 <= 0.0 || f.liq_unit_weight_n_m3 <= 0.0 || f.liq_depth_m <= 0.0 || f.liq_magnitude <= 0.0 {
        return None;
    }
    let annex = annex_params(document).choice();
    let layer = part_5::LiquefactionLayer { n1_60: f.liq_n1_60, fines_percent: f.liq_fines_percent, unit_weight_n_m3: f.liq_unit_weight_n_m3, water_depth_m: f.liq_water_depth_m, magnitude: f.liq_magnitude, a_g_s_ms2: site.gamma_i * site.a_gr * site.s, csr_factor: ndp_number("ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex) };
    let required = ndp_number("ndp.en1998.en5.liquefaction.min-safety-factor", annex);
    let half = 0.5 * f.liq_layer_thickness_m.max(0.0);
    let band = ((f.liq_depth_m - half).max(0.1), f.liq_depth_m + half);
    let bottom = (band.1 + 3.0).max(10.0);
    let depths: Vec<f64> = (1..=(bottom / DEPTH_STEP) as usize).map(|i| i as f64 * DEPTH_STEP).collect();
    let clip = |v: f64| v.clamp(0.0, FACTOR_MAX);
    let point = |z: f64, value: f64| Vec2::new(clip(value), -z);
    let extended: Vec<Vec2> = depths.iter().map(|z| point(*z, layer.factor_annex_b(*z))).collect();
    let in_band = |z: &&f64| **z >= band.0 - 1e-9 && **z <= band.1 + 1e-9;
    let annex_band: Vec<Vec2> = depths.iter().filter(in_band).map(|z| point(*z, layer.factor_annex_b(*z))).collect();
    let youd_band: Vec<Vec2> = depths.iter().filter(in_band).map(|z| point(*z, layer.factor_youd(*z))).collect();
    let (utilisation, check) = check_of(report, &format!("en1998.5.{}.liquefaction", f.id));
    let mut series = vec![Series::Polyline { id: "annex-b-extended".into(), label: loc("F_L Annex B, layer resistance extrapolated", "F_L Anhang B, Schichtwiderstand extrapoliert"), points: extended, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) }];
    if annex_band.len() >= 2 {
        series.push(Series::Polyline { id: "annex-b-layer".into(), label: loc("F_L Annex B in the layer", "F_L Anhang B in der Schicht"), points: annex_band, style: stroke(Tone::Info, 2.0, Dash::Solid) });
        series.push(Series::Polyline { id: "youd-layer".into(), label: loc("F_L Youd et al. 2001 (with r_d)", "F_L Youd et al. 2001 (mit r_d)"), points: youd_band, style: stroke(Tone::Warning, 1.5, Dash::Dashed) });
    }
    series.push(Series::Limit { id: "required".into(), label: loc(&format!("Required F_L = {required}"), &format!("Erforderlich F_L = {required}")), axis: AxisRef::X, value: required, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    series.push(Series::Limit { id: "unity".into(), label: loc("F_L = 1", "F_L = 1"), axis: AxisRef::X, value: 1.0, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    let at_layer = layer.factor_annex_b(f.liq_depth_m);
    let markers = vec![Marker { id: "layer".into(), at: point(f.liq_depth_m, at_layer), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Layer at {:.1} m: F_L = {at_layer:.2}, (N1)60 = {:.1}", f.liq_depth_m, f.liq_n1_60), &format!("Schicht in {:.1} m: F_L = {at_layer:.2}, (N1)60 = {:.1}", f.liq_depth_m, f.liq_n1_60)), utilisation, bound_check: check, binding: None }];
    let columns = vec![
        column("Depth z", "Tiefe z", "m", depths.clone()),
        column("F_L Annex B", "F_L Anhang B", "1", depths.iter().map(|z| clip(layer.factor_annex_b(*z))).collect()),
        column("F_L Youd et al.", "F_L Youd et al.", "1", depths.iter().map(|z| clip(layer.factor_youd(*z))).collect()),
        column("CSR Annex B", "CSR Anhang B", "1", depths.iter().map(|z| { let (t, e) = layer.stresses(*z); part_5::liquefaction_csr_annex_b(layer.csr_factor, layer.a_g_s_ms2, t, e).min(10.0) }).collect()),
    ];
    Some(Diagram2d {
        id: format!("liquefaction-profile:{}", f.id),
        title: loc(&format!("Liquefaction profile \u{2013} {}", f.id), &format!("Verfl\u{fc}ssigungsprofil \u{2013} {}", f.id)),
        description: loc("Factor of safety against liquefaction F_L = CRR\u{b7}MSF/CSR over the depth (EN 1998-5, 4.1.4 and Annex B): the resistance of the described layer is held constant, the stress ratio follows the stresses of the uniform profile. Annex B uses no depth reduction r_d, Youd et al. (2001) does; the difference is the reduction. Values are clipped at 4.", "Sicherheit gegen Bodenverfl\u{fc}ssigung F_L = CRR\u{b7}MSF/CSR \u{fc}ber der Tiefe (EN 1998-5, 4.1.4 und Anhang B): der Widerstand der beschriebenen Schicht wird konstant gehalten, das Spannungsverh\u{e4}ltnis folgt den Spannungen des gleichf\u{f6}rmigen Profils. Anhang B verwendet keine Tiefenabminderung r_d, Youd et al. (2001) schon; der Unterschied ist die Abminderung. Werte sind bei 4 begrenzt."),
        axes: vec![axis("Safety factor F_L", "Sicherheit F_L", QuantityKind::Dimensionless, "1", Some((0.0, FACTOR_MAX))), axis("Elevation below the surface", "H\u{f6}he unter der Oberfl\u{e4}che", QuantityKind::Length, "m", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(f),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🪨️ The layered soil profile of a foundation (rows of `soilLayers`, top to bottom) for the Annex B factor and the site response, `None` while a layer is incomplete or the magnitude is missing.
pub fn soil_profile_of<'a>(document: &'a En1998Snapshot, site: &Site, f: &En1998Foundation) -> Option<(part_5::SoilProfile, Vec<&'a super::model::En1998SoilLayer>)> {
    let rows: Vec<&super::model::En1998SoilLayer> = document.soil_layers.iter().filter(|l| l.foundation_id == f.id).collect();
    if rows.is_empty() || f.liq_magnitude <= 0.0 || rows.iter().any(|l| l.unit_weight_n_m3 <= 0.0 || l.thickness_m <= 0.0) {
        return None;
    }
    let annex = annex_params(document).choice();
    let layers = rows.iter().map(|l| part_5::ProfileLayer { thickness_m: l.thickness_m, unit_weight_n_m3: l.unit_weight_n_m3, shear_wave_velocity_m_s: l.shear_wave_velocity_m_s, n1_60: l.n1_60, fines_percent: l.fines_percent }).collect();
    Some((part_5::SoilProfile { layers, water_depth_m: f.liq_water_depth_m, magnitude: f.liq_magnitude, a_g_s_ms2: site.gamma_i * site.a_gr * site.s, csr_factor: ndp_number("ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex) }, rows))
}

/// 🪨️ F_L over the depth of the whole layered profile (Annex B, each layer with its own (N1)60 and fines content over the integrated stresses): the profile curve, the layer interfaces as depth lines, the required factor and one marker per assessed layer at its governing depth, coloured by the layer check. Table twin: depth, layer number, F_L, CSR, σ'_v and v_s.
pub fn soil_profile_diagram(document: &En1998Snapshot, report: &CheckReport, site: &Site, f: &En1998Foundation) -> Option<Diagram2d> {
    let (profile, rows) = soil_profile_of(document, site, f)?;
    let annex = annex_params(document).choice();
    let required = ndp_number("ndp.en1998.en5.liquefaction.min-safety-factor", annex);
    let total = profile.depth_m();
    let depths: Vec<f64> = (1..=((total / DEPTH_STEP).ceil() as usize)).map(|i| (i as f64 * DEPTH_STEP).min(total)).collect();
    let clip = |v: f64| v.clamp(0.0, FACTOR_MAX);
    let point = |z: f64, value: f64| Vec2::new(clip(value), -z);
    let curve: Vec<Vec2> = depths.iter().map(|z| point(*z, profile.factor(*z))).collect();
    let mut series = vec![Series::Polyline { id: "profile".into(), label: loc("F_L Annex B over all layers", "F_L Anhang B \u{fc}ber alle Schichten"), points: curve, style: stroke(Tone::Info, 2.0, Dash::Solid) }];
    for (index, bottom) in profile.bottoms().iter().enumerate() {
        series.push(Series::Limit { id: format!("interface-{}", rows[index].id), label: loc(&format!("Base of layer {} at {bottom:.1} m", rows[index].id), &format!("Unterkante Schicht {} in {bottom:.1} m", rows[index].id)), axis: AxisRef::Y, value: -*bottom, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    }
    series.push(Series::Limit { id: "required".into(), label: loc(&format!("Required F_L = {required}"), &format!("Erforderlich F_L = {required}")), axis: AxisRef::X, value: required, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    series.push(Series::Limit { id: "unity".into(), label: loc("F_L = 1", "F_L = 1"), axis: AxisRef::X, value: 1.0, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    let mut markers = Vec::new();
    for (index, row) in rows.iter().enumerate().filter(|(i, _)| profile.layers[*i].n1_60 > 0.0) {
        let (z, f_l) = profile.layer_governing(index);
        let (utilisation, check) = check_of(report, &format!("en1998.5.{}.layer.{}.liquefaction", f.id, row.id));
        markers.push(Marker { id: format!("layer-{}", row.id), at: point(z, f_l), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Layer {} at {z:.1} m: F_L = {f_l:.2}, (N1)60 = {:.1}", row.id, row.n1_60), &format!("Schicht {} in {z:.1} m: F_L = {f_l:.2}, (N1)60 = {:.1}", row.id, row.n1_60)), utilisation, bound_check: check, binding: None });
    }
    let layer_number = |z: f64| (profile.layer_at(z) + 1) as f64;
    let columns = vec![
        column("Depth z", "Tiefe z", "m", depths.clone()),
        column("Layer (top = 1)", "Schicht (oben = 1)", "1", depths.iter().map(|z| layer_number(*z)).collect()),
        column("F_L Annex B", "F_L Anhang B", "1", depths.iter().map(|z| clip(profile.factor(*z))).collect()),
        column("CSR Annex B", "CSR Anhang B", "1", depths.iter().map(|z| { let (t, e) = profile.stresses(*z); part_5::liquefaction_csr_annex_b(profile.csr_factor, profile.a_g_s_ms2, t, e).min(10.0) }).collect()),
        column("Effective stress", "Effektive Spannung", "Pa", depths.iter().map(|z| profile.stresses(*z).1).collect()),
        column("Shear wave velocity v_s", "Scherwellengeschwindigkeit v_s", "m/s", depths.iter().map(|z| profile.layers[profile.layer_at(*z)].shear_wave_velocity_m_s).collect()),
    ];
    Some(Diagram2d {
        id: format!("soil-profile:{}", f.id),
        title: loc(&format!("Soil profile liquefaction \u{2013} {}", f.id), &format!("Bodenprofil Verfl\u{fc}ssigung \u{2013} {}", f.id)),
        description: loc("Factor of safety against liquefaction F_L = CRR\u{b7}MSF/CSR over the depth of the layered profile (EN 1998-5, 4.1.4 and Annex B): every layer has its own (N1)60 and fines content, the stresses integrate the unit weights of the layers, a layer with (N1)60 = 0 is not assessed. The dotted lines are the layer interfaces; drag the interfaces in the 3D soil column.", "Sicherheit gegen Bodenverfl\u{fc}ssigung F_L = CRR\u{b7}MSF/CSR \u{fc}ber die Tiefe des Schichtenprofils (EN 1998-5, 4.1.4 und Anhang B): jede Schicht hat ihr eigenes (N1)60 und ihren Feinkornanteil, die Spannungen integrieren die Wichten der Schichten, eine Schicht mit (N1)60 = 0 wird nicht bewertet. Die gepunkteten Linien sind die Schichtgrenzen; sie lassen sich in der 3D-Bodens\u{e4}ule ziehen."),
        axes: vec![axis("Safety factor F_L", "Sicherheit F_L", QuantityKind::Dimensionless, "1", Some((0.0, FACTOR_MAX))), axis("Elevation below the surface", "H\u{f6}he unter der Oberfl\u{e4}che", QuantityKind::Length, "m", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(f),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🌊️ The cyclic stress ratio and the cyclic resistance over the depth of the layered profile: CSR of Annex B (no r_d) and of Youd et al. (2001, with r_d), the resistance CRR·MSF of each layer as a step curve, and the excess pore pressure ratio r_u; where CSR crosses the resistance the layer liquefies. Markers stand at the governing depth of each assessed layer, coloured by its check. Table twin: depth, both stress ratios, the resistance and r_u.
pub fn liquefaction_ratios(document: &En1998Snapshot, report: &CheckReport, site: &Site, f: &En1998Foundation) -> Option<Diagram2d> {
    let (profile, rows) = soil_profile_of(document, site, f)?;
    let annex = annex_params(document).choice();
    let (alpha, exponent) = (ndp_number("ndp.en1998.en5.liquefaction.ru.alpha", annex), ndp_number("ndp.en1998.en5.liquefaction.ru.cycle-exponent", annex));
    let total = profile.depth_m();
    let depths: Vec<f64> = (1..=((total / DEPTH_STEP).ceil() as usize)).map(|i| (i as f64 * DEPTH_STEP).min(total)).collect();
    let clip = |v: f64| v.clamp(0.0, RATIO_MAX);
    let point = |z: f64, value: f64| Vec2::new(clip(value), -z);
    let stress_ratio = |z: f64| {
        let (t, e) = profile.stresses(z);
        part_5::liquefaction_csr_annex_b(profile.csr_factor, profile.a_g_s_ms2, t, e)
    };
    let stress_ratio_youd = |z: f64| {
        let (t, e) = profile.stresses(z);
        part_5::liquefaction_csr(profile.a_g_s_ms2, t, e, z)
    };
    let resistance = |z: f64| profile.resistance(profile.layer_at(z));
    let pore_ratio = |z: f64| hazard::excess_pore_pressure_ratio(profile.factor(z), alpha, exponent);
    let line = |id: &str, en: &str, de: &str, value: &dyn Fn(f64) -> f64, tone: Tone, dash: Dash| Series::Polyline { id: id.into(), label: loc(en, de), points: depths.iter().map(|z| point(*z, value(*z))).collect(), style: stroke(tone, 2.0, dash) };
    let mut series = vec![
        line("csr", "CSR Annex B", "CSR Anhang B", &stress_ratio, Tone::Info, Dash::Solid),
        line("csr-youd", "CSR Youd et al. (with r_d)", "CSR Youd et al. (mit r_d)", &stress_ratio_youd, Tone::Warning, Dash::Dashed),
        line("crr", "Resistance CRR\u{b7}MSF", "Widerstand CRR\u{b7}MSF", &resistance, Tone::Neutral, Dash::Solid),
        line("r-u", "Excess pore pressure ratio r_u", "Porenwasserüberdruckverhältnis r_u", &pore_ratio, Tone::Error, Dash::Dotted),
    ];
    for (index, bottom) in profile.bottoms().iter().enumerate() {
        series.push(Series::Limit { id: format!("interface-{}", rows[index].id), label: loc(&format!("Base of layer {} at {bottom:.1} m", rows[index].id), &format!("Unterkante Schicht {} in {bottom:.1} m", rows[index].id)), axis: AxisRef::Y, value: -*bottom, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    }
    let mut markers = Vec::new();
    for (index, row) in rows.iter().enumerate().filter(|(i, _)| profile.layers[*i].n1_60 > 0.0) {
        let (z, f_l) = profile.layer_governing(index);
        if !f_l.is_finite() {
            continue;
        }
        let (utilisation, check) = check_of(report, &format!("en1998.5.{}.layer.{}.liquefaction", f.id, row.id));
        markers.push(Marker { id: format!("layer-{}", row.id), at: point(z, stress_ratio(z)), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Layer {} at {z:.1} m: CSR {:.3}, CRR\u{b7}MSF {:.3}, F_L = {f_l:.2}, r_u = {:.2}", row.id, stress_ratio(z), resistance(z), pore_ratio(z)), &format!("Schicht {} in {z:.1} m: CSR {:.3}, CRR\u{b7}MSF {:.3}, F_L = {f_l:.2}, r_u = {:.2}", row.id, stress_ratio(z), resistance(z), pore_ratio(z))), utilisation, bound_check: check, binding: None });
    }
    let columns = vec![
        column("Depth z", "Tiefe z", "m", depths.clone()),
        column("CSR Annex B", "CSR Anhang B", "1", depths.iter().map(|z| stress_ratio(*z).min(10.0)).collect()),
        column("CSR Youd et al.", "CSR Youd et al.", "1", depths.iter().map(|z| stress_ratio_youd(*z).min(10.0)).collect()),
        column("Resistance CRR\u{b7}MSF", "Widerstand CRR\u{b7}MSF", "1", depths.iter().map(|z| resistance(*z).min(10.0)).collect()),
        column("Excess pore pressure ratio r_u", "Porenwasserüberdruckverhältnis r_u", "1", depths.iter().map(|z| pore_ratio(*z)).collect()),
    ];
    Some(Diagram2d {
        id: format!("liquefaction-ratios:{}", f.id),
        title: loc(&format!("Cyclic stress and resistance \u{2013} {}", f.id), &format!("Zyklische Beanspruchung und Widerstand \u{2013} {}", f.id)),
        description: loc("Cyclic stress ratio CSR (Annex B without r_d, Youd et al. 2001 with r_d) against the cyclic resistance CRR\u{b7}MSF of each layer over the depth (EN 1998-5, 4.1.4 and Annex B); where the stress ratio exceeds the resistance the layer liquefies (F_L < 1) and the excess pore pressure ratio r_u reaches 1. Values are clipped at 1.", "Zyklisches Spannungsverh\u{e4}ltnis CSR (Anhang B ohne r_d, Youd et al. 2001 mit r_d) gegen den zyklischen Widerstand CRR\u{b7}MSF jeder Schicht \u{fc}ber der Tiefe (EN 1998-5, 4.1.4 und Anhang B); \u{fc}bersteigt das Spannungsverh\u{e4}ltnis den Widerstand, verfl\u{fc}ssigt die Schicht (F_L < 1) und das Porenwasser\u{fc}berdruckverh\u{e4}ltnis r_u erreicht 1. Werte sind bei 1 abgeschnitten."),
        axes: vec![axis("Ratio", "Verhältnis", QuantityKind::Dimensionless, "1", Some((0.0, RATIO_MAX))), axis("Elevation below the surface", "H\u{f6}he unter der Oberfl\u{e4}che", QuantityKind::Length, "m", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(f),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🏗️ The Annex F failure surface in the sections M̄ = 0 (V̄ over N̄) and V̄ = 0 (M̄ over N̄) with the vertical bound at the soil inertia and the design point of both loads; the markers are draggable (V_Ed, M_Ed).
pub fn annex_f_surface(document: &En1998Snapshot, report: &CheckReport, site: &Site, f: &En1998Foundation) -> Option<Diagram2d> {
    if f.footing_width_m <= 0.0 || f.bearing_n_max_n <= 0.0 || f.soil_friction_angle_deg <= 0.0 {
        return None;
    }
    let k = part_5::AnnexF::from_registry(annex_params(document).choice());
    let f_bar = part_5::AnnexF::soil_inertia(site.gamma_i * site.a_gr * site.s, f.soil_friction_angle_deg);
    let bound = k.vertical_bound(f_bar);
    if bound <= 0.0 || !bound.is_finite() {
        return None;
    }
    let (n, v, m) = k.normalise(f.bearing_axial_n, f.bearing_shear_n, f.bearing_moment_nm, f.footing_width_m, f.bearing_n_max_n)?;
    let levels: Vec<f64> = (1..SURFACE_STEPS).map(|i| bound * i as f64 / SURFACE_STEPS as f64).collect();
    let horizontal: Vec<f64> = levels.iter().map(|l| k.horizontal_limit(*l, f_bar)).collect();
    let moment: Vec<f64> = levels.iter().map(|l| k.moment_limit(*l, f_bar)).collect();
    let (utilisation, check) = check_of(report, &format!("en1998.5.{}.annexF", f.id));
    let path = format!("foundations[id={}]", f.id);
    let mut shear_binding = FieldBinding::new(format!("{path}.bearingShearN"), Affine { base: 0.0, k: f.bearing_n_max_n / k.gamma_rd }, "N");
    shear_binding.quantise = Some(Quantise::Round { step: 1000.0 });
    let mut moment_binding = FieldBinding::new(format!("{path}.bearingMomentNm"), Affine { base: 0.0, k: f.footing_width_m * f.bearing_n_max_n / k.gamma_rd }, "N*m");
    moment_binding.quantise = Some(Quantise::Round { step: 1000.0 });
    let columns = vec![column("N\u{304}", "N\u{304}", "1", levels.clone()), column("V\u{304} limit (M\u{304} = 0)", "V\u{304}-Grenze (M\u{304} = 0)", "1", horizontal.clone()), column("M\u{304} limit (V\u{304} = 0)", "M\u{304}-Grenze (V\u{304} = 0)", "1", moment.clone())];
    Some(Diagram2d {
        id: format!("annex-f:{}", f.id),
        title: loc(&format!("Seismic bearing surface \u{2013} {}", f.id), &format!("Erdbebengrundbruch-Fl\u{e4}che \u{2013} {}", f.id)),
        description: loc("Generalised failure surface of EN 1998-5 Annex F for cohesionless soil in the sections M\u{304} = 0 and V\u{304} = 0 over the normalised vertical force N\u{304} = \u{3b3}Rd\u{b7}N/Nmax, with the vertical bound (1 \u{2212} m\u{b7}F\u{304}^k)^k' reduced by the soil inertia F\u{304} = ag\u{b7}S/(g\u{b7}tan \u{3c6}'). Points inside the curves are safe; the table constants are unverified registry rows.", "Verallgemeinerte Bruchfl\u{e4}che nach EN 1998-5 Anhang F f\u{fc}r nichtbindigen Boden in den Schnitten M\u{304} = 0 und V\u{304} = 0 \u{fc}ber der normierten Vertikalkraft N\u{304} = \u{3b3}Rd\u{b7}N/Nmax, mit der durch die Bodentr\u{e4}gheit F\u{304} = ag\u{b7}S/(g\u{b7}tan \u{3c6}') verminderten vertikalen Grenze (1 \u{2212} m\u{b7}F\u{304}^k)^k'. Punkte innerhalb der Kurven sind sicher; die Tabellenkonstanten sind ungepr\u{fc}fte Registerzeilen."),
        axes: vec![axis("Normalised V\u{304} or M\u{304}", "Normiertes V\u{304} oder M\u{304}", QuantityKind::Dimensionless, "1", None), axis("Normalised vertical force N\u{304}", "Normierte Vertikalkraft N\u{304}", QuantityKind::Dimensionless, "1", Some((0.0, 1.0)))],
        series: vec![
            Series::Polyline { id: "surface-v".into(), label: loc("Surface, M\u{304} = 0", "Fl\u{e4}che, M\u{304} = 0"), points: horizontal.iter().zip(&levels).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "surface-m".into(), label: loc("Surface, V\u{304} = 0", "Fl\u{e4}che, V\u{304} = 0"), points: moment.iter().zip(&levels).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
            Series::Limit { id: "vertical-bound".into(), label: loc(&format!("Vertical bound {bound:.2}"), &format!("Vertikale Grenze {bound:.2}")), axis: AxisRef::Y, value: bound, style: stroke(Tone::Error, 1.0, Dash::Dashed) },
        ],
        markers: vec![
            Marker { id: "design-v".into(), at: Vec2::new(v, n), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Design V\u{304} = {v:.3}, N\u{304} = {n:.3}"), &format!("Bemessung V\u{304} = {v:.3}, N\u{304} = {n:.3}")), utilisation, bound_check: check.clone(), binding: Some(shear_binding) },
            Marker { id: "design-m".into(), at: Vec2::new(m, n), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Design M\u{304} = {m:.3}, N\u{304} = {n:.3}"), &format!("Bemessung M\u{304} = {m:.3}, N\u{304} = {n:.3}")), utilisation, bound_check: check, binding: Some(moment_binding) },
        ],
        annotations: Vec::new(),
        focus: focus(f),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🏗️ The inertial pile moment over the depth (Winkler solution of the head shear), the kinematic moment at the stiffness interface and the bending resistance as limits.
pub fn pile_profile(document: &En1998Snapshot, report: &CheckReport, site: &Site, f: &En1998Foundation) -> Option<Diagram2d> {
    if f.pile_diameter_m <= 0.0 || f.pile_length_m <= 0.0 || f.pile_modulus_pa <= 0.0 || f.pile_subgrade_modulus_n_m3 <= 0.0 {
        return None;
    }
    let annex = annex_params(document).choice();
    let fixed = f.pile_head_fixed >= 0.5;
    let inertial = part_5::pile_inertial(f.pile_modulus_pa, f.pile_diameter_m, f.pile_length_m, f.pile_subgrade_modulus_n_m3, f.pile_head_shear_n, fixed);
    let reach = f.pile_length_m.min(6.0 / inertial.beta.max(1e-6));
    let depths: Vec<f64> = (0..=40).map(|i| reach * i as f64 / 40.0).collect();
    let moments: Vec<f64> = depths.iter().map(|z| part_5::pile_inertial_moment_at(&inertial, f.pile_head_shear_n, fixed, *z) / 1000.0).collect();
    let (utilisation, check) = check_of(report, &format!("en1998.5.{}.pile.moment", f.id));
    let peak_depth = if fixed { 0.0 } else { std::f64::consts::FRAC_PI_4 / inertial.beta };
    let mut markers = vec![Marker { id: "inertial".into(), at: Vec2::new(inertial.max_moment_nm / 1000.0, -peak_depth), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Inertial M = {:.0} kN\u{b7}m at {peak_depth:.1} m", inertial.max_moment_nm / 1000.0), &format!("Tr\u{e4}gheit M = {:.0} kN\u{b7}m in {peak_depth:.1} m", inertial.max_moment_nm / 1000.0)), utilisation, bound_check: check, binding: None }];
    let contrast = part_5::stiffness_contrast(f.pile_upper_vs_m_s, f.pile_lower_vs_m_s);
    if f.pile_interface_depth_m > 0.0 && f.pile_upper_vs_m_s > 0.0 && f.pile_lower_vs_m_s > 0.0 && f.liq_unit_weight_n_m3 > 0.0 && contrast >= ndp_number("ndp.en1998.en5.pile.stiffness-contrast", annex) {
        let tau = part_5::interface_shear_stress_pa(site.gamma_i * site.a_gr * site.s, f.liq_unit_weight_n_m3, f.pile_interface_depth_m);
        let e_1 = part_5::soil_young_modulus_pa(f.liq_unit_weight_n_m3, f.pile_upper_vs_m_s);
        let kinematic = part_5::pile_kinematic_moment_nm(&part_5::Nikolaou::from_registry(annex), tau, f.pile_diameter_m, f.pile_length_m, f.pile_modulus_pa, e_1, f.pile_upper_vs_m_s, f.pile_lower_vs_m_s);
        markers.push(Marker { id: "reference:kinematic".into(), at: Vec2::new(kinematic / 1000.0, -f.pile_interface_depth_m), shape: MarkerShape::Diamond, tone: Tone::Warning, label: loc(&format!("Kinematic M = {:.0} kN\u{b7}m at {:.1} m", kinematic / 1000.0, f.pile_interface_depth_m), &format!("Kinematik M = {:.0} kN\u{b7}m in {:.1} m", kinematic / 1000.0, f.pile_interface_depth_m)), utilisation: None, bound_check: None, binding: None });
    }
    let columns = vec![column("Depth below the head z", "Tiefe unter dem Kopf z", "m", depths.clone()), column("Inertial moment", "Tr\u{e4}gheitsmoment", "kN\u{b7}m", moments.clone())];
    Some(Diagram2d {
        id: format!("pile-moments:{}", f.id),
        title: loc(&format!("Pile moments \u{2013} {}", f.id), &format!("Pfahlmomente \u{2013} {}", f.id)),
        description: loc("Bending moment of the long pile on the Winkler bed under the inertial head shear (head fixed or free, EN 1998-5, 5.4.2) and the kinematic moment at the stiffness interface of the soil layers (reference formula of Nikolaou et al. 2001); the dashed lines are \u{b1}M_Rd.", "Biegemoment des langen Pfahls auf Winkler-Bettung unter der Tr\u{e4}gheitskraft am Kopf (Kopf eingespannt oder frei, EN 1998-5, 5.4.2) und kinematisches Moment am Steifigkeitssprung der Bodenschichten (Bezugsformel von Nikolaou et al. 2001); die gestrichelten Linien sind \u{b1}M_Rd."),
        axes: vec![axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN\u{b7}m", None), axis("Elevation below the pile head", "H\u{f6}he unter dem Pfahlkopf", QuantityKind::Length, "m", None)],
        series: vec![
            Series::Polyline { id: "inertial".into(), label: loc("Inertial moment M(z)", "Tr\u{e4}gheitsmoment M(z)"), points: moments.iter().zip(&depths).map(|(m, z)| Vec2::new(*m, -z)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "m-rd".into(), label: loc("M_Rd", "M_Rd"), axis: AxisRef::X, value: f.pile_moment_rd_nm / 1000.0, style: stroke(Tone::Error, 1.0, Dash::Dashed) },
            Series::Limit { id: "m-rd-negative".into(), label: loc("\u{2212}M_Rd", "\u{2212}M_Rd"), axis: AxisRef::X, value: -f.pile_moment_rd_nm / 1000.0, style: stroke(Tone::Error, 1.0, Dash::Dashed) },
        ],
        markers,
        annotations: Vec::new(),
        focus: focus(f),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// ⛰️ The factor of safety F_s of the given slip circle over the horizontal seismic coefficient F_H / W from 0 to twice the design value, with the limit 1.0 and the design point (EN 1998-5 4.1.3.3); the vertical share and the M2 strengths are those of the check.
pub fn slope_factor_curve(document: &En1998Snapshot, report: &CheckReport, site: &Site, sl: &En1998Slope) -> Option<Diagram2d> {
    let forces = part_5::slope_forces(document, sl, annex_params(document).choice(), site.gamma_i * site.a_gr, site.s)?;
    let design = forces.pseudostatic.horizontal;
    let top = (2.0 * design).max(0.1);
    let points: Vec<Vec2> = (0..=SLOPE_STEPS).filter_map(|i| {
        let k = top * i as f64 / SLOPE_STEPS as f64;
        part_5::slope_factor_at(&forces, sl, k).map(|f| Vec2::new(k, f.min(FACTOR_MAX)))
    }).collect();
    let at_design = part_5::slope_factor_at(&forces, sl, design)?;
    if points.len() < 2 {
        return None;
    }
    let (utilisation, check) = check_of(report, &format!("en1998.5.{}.slope", sl.id));
    let marker = Marker { id: format!("slope-design-{}", sl.id), at: Vec2::new(design, at_design.min(FACTOR_MAX)), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Info, tone), label: loc(&format!("Design: F_H/W = {design:.3}, F_s = {at_design:.2}"), &format!("Bemessung: F_H/W = {design:.3}, F_s = {at_design:.2}")), utilisation, bound_check: check, binding: None };
    Some(Diagram2d {
        id: format!("slope-factor:{}", sl.id),
        title: loc(&format!("Slope factor of safety \u{2013} {}", sl.id), &format!("Böschungssicherheit \u{2013} {}", sl.id)),
        description: loc("Simplified Bishop factor of safety F_s of the given slip circle with the M2 strengths over the horizontal seismic coefficient F_H / W (EN 1998-5, 4.1.3.3); the design point is marked and values below 1.0 violate the limit.", "Vereinfachter Bishop-Sicherheitsfaktor F_s des gegebenen Gleitkreises mit den M2-Festigkeiten über dem horizontalen Erdbebenbeiwert F_H / W (EN 1998-5, 4.1.3.3); der Bemessungspunkt ist markiert, Werte unter 1,0 verletzen die Grenze."),
        axes: vec![axis("Seismic coefficient F_H / W", "Erdbebenbeiwert F_H / W", QuantityKind::Dimensionless, "1", Some((0.0, top))), axis("Factor of safety F_s", "Sicherheitsfaktor F_s", QuantityKind::Dimensionless, "1", Some((0.0, FACTOR_MAX)))],
        series: vec![
            Series::Polyline { id: "factor".into(), label: loc("F_s (Bishop, given circle)", "F_s (Bishop, gegebener Kreis)"), points: points.clone(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "limit".into(), label: loc("Limit 1.0", "Grenze 1,0"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![marker],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &sl.id, format!("slopes[id={}]", sl.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("F_H / W", "F_H / W", "1", points.iter().map(|q| q.x).collect()), column("F_s", "F_s", "1", points.iter().map(|q| q.y).collect())] },
    })
}
