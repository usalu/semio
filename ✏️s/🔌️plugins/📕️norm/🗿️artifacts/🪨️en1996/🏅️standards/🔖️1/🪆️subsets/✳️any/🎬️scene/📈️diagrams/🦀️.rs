//! 📈️ Diagrams: eccentricity, reduction factor Φ and N_Ed against N_Rd along the wall height, Φ_m over the slenderness, f_k over f_b and f_m, the shear strength line,
//! the lateral coefficient α₁ and capacity over the aspect ratio, the tabulated fire thickness and the M–N curve of reinforced walls, the chase table and the movement joint panels; each with a table twin.

use super::kit::{base_id, is_document_check, is_drawn_family, owner_entity, wall_ref};
use super::masonry as m;
use super::model::{ChaseKind, En1996Snapshot, MasonryWall, WallType};
use super::norm::{CheckReport, CheckResult};
use super::profile::{self, Profile};
use super::scene::diagram_kit::{curves, Xy};
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column, stroke};
use super::scene::projection_kit::diagram_parts;

const SAMPLES: usize = 40;
const FIRE_CLASSES: [f64; 6] = [30.0, 60.0, 90.0, 120.0, 180.0, 240.0];

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn p(point: Xy) -> Vec2 {
    Vec2::new(point[0], point[1])
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: (f64, f64)) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, Some(domain))
}

fn utilisation_tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn known(report: &CheckReport, id: &str) -> Option<String> {
    report.checks.iter().any(|c| c.id == id).then(|| id.to_string())
}

fn find<'a>(report: &'a CheckReport, id: &str) -> Option<&'a CheckResult> {
    report.checks.iter().find(|c| c.id == id)
}

struct Point<'a> {
    id: &'a str,
    at: (f64, f64),
    label: (String, String),
    utilisation: Option<f64>,
    bound: Option<String>,
    binding: Option<FieldBinding>,
}

fn marker(point: Point) -> Marker {
    Marker { id: point.id.to_string(), at: Vec2::new(point.at.0, point.at.1), shape: MarkerShape::Circle, tone: point.utilisation.map_or(Tone::Neutral, utilisation_tone), label: loc(&point.label.0, &point.label.1), utilisation: point.utilisation, bound_check: point.bound, binding: point.binding }
}

fn binding(wall: &MasonryWall, leaf: &str, map: Affine, unit: &str, quantise: Quantise) -> FieldBinding {
    FieldBinding { path: format!("walls[id={}].{leaf}", wall.id), map, unit: unit.to_string(), quantise: Some(quantise) }
}

struct Draft {
    id: String,
    title: LocalizedCopy,
    description: LocalizedCopy,
    axes: Vec<Axis>,
    series: Vec<Series>,
    markers: Vec<Marker>,
    notes: Vec<DiagramNote>,
    table: Vec<DataColumn>,
}

fn finish(wall: &MasonryWall, draft: Draft) -> Diagram2d {
    Diagram2d { id: draft.id, title: draft.title, description: draft.description, axes: draft.axes, series: draft.series, markers: draft.markers, annotations: draft.notes, focus: Some(wall_ref(&wall.id)), legend: true, table_twin: DataTable { columns: draft.table } }
}

fn heights(wall: &MasonryWall) -> [f64; 3] {
    [wall.height_m, 0.5 * wall.height_m, 0.0]
}

fn along_height(values: [f64; 3], wall: &MasonryWall) -> Vec<Vec2> {
    heights(wall).iter().zip(values).map(|(z, v)| Vec2::new(v, *z)).collect()
}

fn station_name(station: usize) -> (&'static str, &'static str) {
    match station {
        0 => ("top", "Kopf"),
        1 => ("middle", "Mitte"),
        _ => ("bottom", "Fuß"),
    }
}

fn eccentricity(wall: &MasonryWall, prof: &Profile, report: &CheckReport) -> Diagram2d {
    let (h, t) = (wall.height_m, wall.thickness_m);
    let e = [prof.e_top, prof.e_mk, prof.e_bottom];
    let top = binding(wall, "eccentricityTopM", Affine { base: -(prof.e_slab + prof.e_theta), k: 1.0 }, "m", Quantise::Round { step: 0.001 });
    let bottom = binding(wall, "eccentricityBottomM", Affine::IDENTITY, "m", Quantise::Round { step: 0.001 });
    let bound = |s: usize| known(report, &super::walls::compression_id(&wall.id, &prof.load_case, s));
    let markers = [(0, Some(top)), (1, None), (2, Some(bottom))]
        .into_iter()
        .map(|(s, b)| {
            let (en, de) = station_name(s);
            marker(Point { id: &format!("e-{en}"), at: (e[s], heights(wall)[s]), label: (format!("e at {en} = {:.1} mm", e[s] * 1000.0), format!("e {de} = {:.1} mm", e[s] * 1000.0)), utilisation: Some(prof.utilisation[s]), bound: bound(s), binding: b })
        })
        .collect();
    let x_max = e.iter().fold(0.2 * t, |a, b| a.max(*b)) * 1.15;
    finish(wall, Draft {
        id: format!("ecc:{}", wall.id),
        title: loc(&format!("Eccentricity along the height – {}", wall.label_en), &format!("Ausmitte über die Höhe – {}", wall.label_de)),
        description: loc("Eccentricity of the vertical load at the top, at mid-height including initial, wind and creep parts (e_mk) and at the base (EN 1996-1-1, 6.1.2.2); the markers drag the declared eccentricities.", "Ausmitte der Vertikallast am Kopf, in Wandmitte einschließlich Anfangs-, Wind- und Kriechanteil (e_mk) und am Fuß (EN 1996-1-1, 6.1.2.2); die Marker verschieben die angegebenen Ausmitten."),
        axes: vec![axis("Eccentricity e", "Ausmitte e", QuantityKind::Length, "m", (0.0, x_max)), axis("Height z", "Höhe z", QuantityKind::Length, "m", (0.0, h))],
        series: vec![
            Series::Polyline { id: "e".into(), label: loc("e(z)", "e(z)"), points: along_height(e, wall), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "e-min".into(), label: loc("Minimum 0.05 t", "Minimum 0,05 t"), axis: AxisRef::X, value: 0.05 * t, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
            Series::Limit { id: "e-limit".into(), label: loc("Limit 0.2 t", "Grenze 0,2 t"), axis: AxisRef::X, value: 0.2 * t, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers,
        notes: vec![DiagramNote { id: "e-mk".into(), at: Vec2::new(prof.e_mk, 0.5 * h), text: loc(&format!("e_mk = {:.1} mm (e_init {:.1}, e_hm {:.1}, e_k {:.1})", prof.e_mk * 1000.0, prof.e_init * 1000.0, prof.e_hm * 1000.0, prof.e_k * 1000.0), &format!("e_mk = {:.1} mm (e_init {:.1}, e_hm {:.1}, e_k {:.1})", prof.e_mk * 1000.0, prof.e_init * 1000.0, prof.e_hm * 1000.0, prof.e_k * 1000.0)), tone: Tone::Neutral }],
        table: vec![column("Height z", "Höhe z", "m", heights(wall).to_vec()), column("Eccentricity e", "Ausmitte e", "m", e.to_vec())],
    })
}

/// 📉️ The reduction factor symbol of a verified section: Φ_i at the top and the base, Φ_m at mid-height.
fn symbol(station: usize) -> &'static str {
    if station == 1 { "Φ_m" } else { "Φ_i" }
}

fn reduction(wall: &MasonryWall, prof: &Profile, report: &CheckReport) -> Diagram2d {
    let h = wall.height_m;
    let markers = (0..3)
        .map(|s| {
            let (en, de) = station_name(s);
            marker(Point { id: &format!("phi-{en}"), at: (prof.phi[s], heights(wall)[s]), label: (format!("{} at {en} = {:.3}", symbol(s), prof.phi[s]), format!("{} {de} = {:.3}", symbol(s), prof.phi[s])), utilisation: Some(prof.utilisation[s]), bound: known(report, &super::walls::compression_id(&wall.id, &prof.load_case, s)), binding: None })
        })
        .collect();
    finish(wall, Draft {
        id: format!("phi:{}", wall.id),
        title: loc(&format!("Reduction factor Φ along the height – {}", wall.label_en), &format!("Abminderungsfaktor Φ über die Höhe – {}", wall.label_de)),
        description: loc("Φ_i = 1 − 2e/t at the top and bottom and Φ_m of Annex G at mid-height, both times Φ₂ of the slab span (EN 1996-1-1, 6.1.2.2).", "Φ_i = 1 − 2e/t an Kopf und Fuß und Φ_m nach Anhang G in Wandmitte, jeweils mal Φ₂ der Deckenspannweite (EN 1996-1-1, 6.1.2.2)."),
        axes: vec![axis("Reduction factor Φ", "Abminderungsfaktor Φ", QuantityKind::Dimensionless, "1", (0.0, 1.05)), axis("Height z", "Höhe z", QuantityKind::Length, "m", (0.0, h))],
        series: vec![Series::Polyline { id: "phi".into(), label: loc("Φ(z)", "Φ(z)"), points: along_height(prof.phi, wall), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers,
        notes: vec![DiagramNote { id: "e-mk".into(), at: Vec2::new(prof.phi[1], 0.5 * h), text: loc(&format!("e_mk = {:.1} mm, h_ef/t_ef = {:.1}", prof.e_mk * 1000.0, prof.lambda), &format!("e_mk = {:.1} mm, h_ef/t_ef = {:.1}", prof.e_mk * 1000.0, prof.lambda)), tone: Tone::Neutral }],
        table: vec![column("Height z", "Höhe z", "m", heights(wall).to_vec()), column("Reduction factor Φ", "Abminderungsfaktor Φ", "1", prof.phi.to_vec())],
    })
}

fn capacity(wall: &MasonryWall, prof: &Profile, report: &CheckReport) -> Diagram2d {
    let h = wall.height_m;
    let demand = prof.n.map(|n| n / 1000.0);
    let resistance = [0, 1, 2].map(|s| prof.resistance(s) / 1000.0);
    let x_max = demand.iter().chain(&resistance).fold(1.0_f64, |a, b| a.max(*b)) * 1.1;
    let markers = (0..3)
        .map(|s| {
            let (en, de) = station_name(s);
            marker(Point { id: &format!("n-{en}"), at: (demand[s], heights(wall)[s]), label: (format!("N_Ed at {en} = {:.0} kN of N_Rd = {:.0} kN", demand[s], resistance[s]), format!("N_Ed {de} = {:.0} kN von N_Rd = {:.0} kN", demand[s], resistance[s])), utilisation: Some(prof.utilisation[s]), bound: known(report, &super::walls::compression_id(&wall.id, &prof.load_case, s)), binding: None })
        })
        .collect();
    finish(wall, Draft {
        id: format!("capacity:{}", wall.id),
        title: loc(&format!("Axial force and resistance along the height – {}", wall.label_en), &format!("Normalkraft und Tragfähigkeit über die Höhe – {}", wall.label_de)),
        description: loc("Design force N_Ed (self-weight accumulates downwards) against N_Rd = Φ Φ₂ f_d A at top, middle and bottom (EN 1996-1-1, 6.1.2.1).", "Bemessungskraft N_Ed (Eigengewicht nimmt nach unten zu) gegenüber N_Rd = Φ Φ₂ f_d A an Kopf, Mitte und Fuß (EN 1996-1-1, 6.1.2.1)."),
        axes: vec![axis("Axial force N", "Normalkraft N", QuantityKind::Force, "kN", (0.0, x_max)), axis("Height z", "Höhe z", QuantityKind::Length, "m", (0.0, h))],
        series: vec![
            Series::Polyline { id: "n-ed".into(), label: loc("N_Ed(z)", "N_Ed(z)"), points: along_height(demand, wall), style: stroke(Tone::Warning, 2.0, Dash::Solid) },
            Series::Polyline { id: "n-rd".into(), label: loc("N_Rd(z)", "N_Rd(z)"), points: along_height(resistance, wall), style: stroke(Tone::Info, 2.5, Dash::Dashed) },
        ],
        markers,
        notes: Vec::new(),
        table: vec![column("Height z", "Höhe z", "m", heights(wall).to_vec()), column("N_Ed", "N_Ed", "kN", demand.to_vec()), column("N_Rd", "N_Rd", "kN", resistance.to_vec())],
    })
}

fn slenderness(doc: &En1996Snapshot, wall: &MasonryWall, prof: &Profile, report: &CheckReport) -> Diagram2d {
    let t = wall.thickness_m;
    let k_e = m::k_e(doc.annex);
    let limit = m::normative_limit(&["slenderness"]);
    let ratios = [0.05, 0.1, 0.2, 0.3];
    let mut series = Vec::new();
    let mut table = vec![column("Slenderness h_ef/t_ef", "Schlankheit h_ef/t_ef", "1", curves::sample(|x| x, 0.0, 30.0, SAMPLES).iter().map(|q| q[0]).collect())];
    for r in ratios {
        let points = curves::sample(|lam| m::phi_m(lam, r * t, t, k_e), 0.0, 30.0, SAMPLES);
        table.push(column(&format!("Φ_m at e/t = {r}"), &format!("Φ_m bei e/t = {r}"), "1", points.iter().map(|q| q[1]).collect()));
        series.push(Series::Polyline { id: format!("phi-m-{}", (r * 100.0).round() as u32), label: loc(&format!("e/t = {r}"), &format!("e/t = {r}")), points: points.into_iter().map(p).collect(), style: stroke(Tone::Neutral, 1.0, Dash::Solid) });
    }
    let own = curves::sample(|lam| m::phi_m(lam, prof.e_mk, t, k_e), 0.0, 30.0, SAMPLES);
    table.push(column("Φ_m of this wall", "Φ_m dieser Wand", "1", own.iter().map(|q| q[1]).collect()));
    series.push(Series::Polyline { id: "phi-m-wall".into(), label: loc("This wall (e_mk)", "Diese Wand (e_mk)"), points: own.into_iter().map(p).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) });
    series.push(Series::Limit { id: "lambda-limit".into(), label: loc(&format!("Limit {limit:.0}"), &format!("Grenze {limit:.0}")), axis: AxisRef::X, value: limit, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    let rho = (m::h_ef_m(wall) / wall.height_m.max(1e-9)).max(1e-3);
    let phi_m = prof.phi[1] / prof.phi2.max(1e-9);
    let at = (prof.lambda.min(30.0), phi_m);
    finish(wall, Draft {
        id: format!("slenderness:{}", wall.id),
        title: loc(&format!("Slenderness and Φ_m – {}", wall.label_en), &format!("Schlankheit und Φ_m – {}", wall.label_de)),
        description: loc("Mid-height reduction factor Φ_m over h_ef/t_ef for several e_mk/t (EN 1996-1-1, Annex G); the marker drags the wall height through ρ_n.", "Abminderungsfaktor Φ_m in Wandmitte über h_ef/t_ef für mehrere e_mk/t (EN 1996-1-1, Anhang G); der Marker verschiebt die Wandhöhe über ρ_n."),
        axes: vec![axis("Slenderness h_ef/t_ef", "Schlankheit h_ef/t_ef", QuantityKind::Dimensionless, "1", (0.0, 30.0)), axis("Reduction factor Φ_m", "Abminderungsfaktor Φ_m", QuantityKind::Dimensionless, "1", (0.0, 1.05))],
        series,
        markers: vec![marker(Point { id: "wall", at, label: (format!("h_ef/t = {:.1}, Φ_m = {:.3}", prof.lambda, phi_m), format!("h_ef/t = {:.1}, Φ_m = {:.3}", prof.lambda, phi_m)), utilisation: Some(prof.lambda / limit), bound: known(report, &format!("en1996.5.5.1.slenderness.{}", wall.id)), binding: Some(binding(wall, "heightM", Affine { base: 0.0, k: t / rho }, "m", Quantise::Round { step: 0.05 })) })],
        notes: Vec::new(),
        table,
    })
}

fn strength(doc: &En1996Snapshot, wall: &MasonryWall, report: &CheckReport) -> Diagram2d {
    let (annex, material, group, mortar) = (doc.annex, wall.unit_material, wall.unit_group, wall.mortar_type);
    let delta = m::shape_factor_delta(wall.unit_height_m, wall.unit_width_m.min(wall.unit_length_m));
    let fb = m::f_b_normalised_pa(wall) / 1e6;
    let own_fm = m::f_m_mpa(wall);
    let mut series = Vec::new();
    let mut table = vec![column("Normalised unit strength f_b", "Normierte Steindruckfestigkeit f_b", "MPa", curves::sample(|x| x, 1.0, 60.0, SAMPLES).iter().map(|q| q[0]).collect())];
    for f_m in [2.5, 5.0, 10.0, 20.0] {
        let points = curves::sample(|b| m::f_k_mpa_from(annex, material, group, mortar, b, f_m), 1.0, 60.0, SAMPLES);
        table.push(column(&format!("f_k at f_m = {f_m} MPa"), &format!("f_k bei f_m = {f_m} MPa"), "MPa", points.iter().map(|q| q[1]).collect()));
        series.push(Series::Polyline { id: format!("fk-m{}", (f_m * 10.0).round() as u32), label: loc(&format!("f_m = {f_m} MPa"), &format!("f_m = {f_m} MPa")), points: points.into_iter().map(p).collect(), style: stroke(Tone::Neutral, 1.0, Dash::Solid) });
    }
    let own = curves::sample(|b| m::f_k_mpa_from(annex, material, group, mortar, b, own_fm), 1.0, 60.0, SAMPLES);
    table.push(column("f_k of this wall", "f_k dieser Wand", "MPa", own.iter().map(|q| q[1]).collect()));
    series.push(Series::Polyline { id: "fk-wall".into(), label: loc(&format!("This wall, f_m = {own_fm} MPa"), &format!("Diese Wand, f_m = {own_fm} MPa")), points: own.into_iter().map(p).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) });
    let fk = m::f_k_pa(annex, wall) / 1e6;
    let y_max = (fb.min(60.0) * 0.9).max(fk).max(10.0) * 1.3;
    finish(wall, Draft {
        id: format!("fk:{}", wall.id),
        title: loc(&format!("Characteristic strength f_k – {}", wall.label_en), &format!("Charakteristische Druckfestigkeit f_k – {}", wall.label_de)),
        description: loc("f_k = K f_b^α f_m^β over the normalised unit strength for several mortar strengths (EN 1996-1-1, 3.6.1.2); the marker drags the declared unit strength.", "f_k = K f_b^α f_m^β über der normierten Steindruckfestigkeit für mehrere Mörtelfestigkeiten (EN 1996-1-1, 3.6.1.2); der Marker verschiebt die angegebene Steindruckfestigkeit."),
        axes: vec![axis("Normalised unit strength f_b", "Normierte Steindruckfestigkeit f_b", QuantityKind::Stress, "MPa", (1.0, 60.0)), axis("Characteristic strength f_k", "Charakteristische Druckfestigkeit f_k", QuantityKind::Stress, "MPa", (0.0, y_max))],
        series,
        markers: vec![marker(Point { id: "wall", at: (fb.clamp(1.0, 60.0), fk), label: (format!("f_b = {fb:.1} MPa, f_k = {fk:.2} MPa"), format!("f_b = {fb:.1} MPa, f_k = {fk:.2} MPa")), utilisation: None, bound: known(report, &format!("en1996.3.1.material.{}", wall.id)), binding: Some(binding(wall, "fBPa", Affine { base: 0.0, k: 1e6 / delta.max(1e-9) }, "Pa", Quantise::Round { step: 1e5 })) })],
        notes: Vec::new(),
        table,
    })
}

fn shear(doc: &En1996Snapshot, wall: &MasonryWall, prof: &Profile, report: &CheckReport) -> Diagram2d {
    let annex = doc.annex;
    let sigma_max = (prof.shear_sigma / 1e6 * 1.5).max(2.0);
    let f_vk0 = m::f_vk0_mpa(annex, wall.unit_material, wall.mortar_type, wall.mortar_class);
    let fb = m::f_b_normalised_pa(wall) / 1e6;
    let capped = curves::sample(|s| m::f_vk_pa(annex, wall, s * 1e6) / 1e6, 0.0, sigma_max, SAMPLES);
    let uncapped = curves::sample(|s| f_vk0 + 0.4 * s, 0.0, sigma_max, SAMPLES);
    let cap = curves::sample(|s| m::f_vlt_mpa(annex, wall.unit_group, fb, s), 0.0, sigma_max, SAMPLES);
    let sigma = prof.shear_sigma / 1e6;
    let f_vk = m::f_vk_pa(annex, wall, prof.shear_sigma) / 1e6;
    let table = vec![
        column("Compressive stress σ_d", "Druckspannung σ_d", "MPa", capped.iter().map(|q| q[0]).collect()),
        column("f_vk", "f_vk", "MPa", capped.iter().map(|q| q[1]).collect()),
        column("f_vk0 + 0.4 σ_d", "f_vk0 + 0,4 σ_d", "MPa", uncapped.iter().map(|q| q[1]).collect()),
        column("Upper limit f_vlt", "Obergrenze f_vlt", "MPa", cap.iter().map(|q| q[1]).collect()),
    ];
    let y_max = cap.iter().chain(&uncapped).map(|q| q[1]).fold(f_vk, f64::max) * 1.1;
    let lc_check = format!("en1996.6.2.shear.{}.{}", wall.id, prof.load_case);
    finish(wall, Draft {
        id: format!("shear:{}", wall.id),
        title: loc(&format!("Shear strength f_vk – {}", wall.label_en), &format!("Schubfestigkeit f_vk – {}", wall.label_de)),
        description: loc("Mohr–Coulomb line f_vk = f_vk0 + 0.4 σ_d limited by f_vlt (EN 1996-1-1, 3.6.2); the marker is the compressive stress of the governing shear combination.", "Mohr-Coulomb-Gerade f_vk = f_vk0 + 0,4 σ_d, begrenzt durch f_vlt (EN 1996-1-1, 3.6.2); der Marker ist die Druckspannung der maßgebenden Schubkombination."),
        axes: vec![axis("Compressive stress σ_d", "Druckspannung σ_d", QuantityKind::Stress, "MPa", (0.0, sigma_max)), axis("Shear strength f_vk", "Schubfestigkeit f_vk", QuantityKind::Stress, "MPa", (0.0, y_max))],
        series: vec![
            Series::Polyline { id: "fvk".into(), label: loc("f_vk", "f_vk"), points: capped.into_iter().map(p).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Polyline { id: "fvk-uncapped".into(), label: loc("f_vk0 + 0.4 σ_d", "f_vk0 + 0,4 σ_d"), points: uncapped.into_iter().map(p).collect(), style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
            Series::Polyline { id: "fvlt".into(), label: loc("Upper limit f_vlt", "Obergrenze f_vlt"), points: cap.into_iter().map(p).collect(), style: stroke(Tone::Error, 1.5, Dash::Dotted) },
        ],
        markers: vec![marker(Point { id: "demand", at: (sigma, f_vk), label: (format!("σ_d = {sigma:.3} MPa, f_vk = {f_vk:.3} MPa"), format!("σ_d = {sigma:.3} MPa, f_vk = {f_vk:.3} MPa")), utilisation: find(report, &lc_check).map(|c| c.utilization), bound: known(report, &lc_check), binding: None })],
        notes: Vec::new(),
        table,
    })
}

fn lateral(doc: &En1996Snapshot, wall: &MasonryWall, report: &CheckReport) -> Option<Vec<Diagram2d>> {
    let lat = profile::lateral(doc, wall)?;
    let l = lat.l_ef;
    let ratios: Vec<f64> = curves::sample(|x| x, 0.3, 2.0, 18).iter().map(|q| q[0]).collect();
    let pressures: Vec<f64> = ratios.iter().map(|r| lat.pressure(l, r * l)).collect();
    let alphas: Vec<f64> = pressures.iter().map(|w| lat.m1 / (w.max(1e-9) * l * l)).collect();
    let w_rd = lat.pressure(l, wall.height_m);
    let alpha = lat.m1 / (w_rd.max(1e-9) * l * l);
    let at = wall.height_m / l;
    let check = format!("en1996.m-31.w-rd.{}.{}", wall.id, lat.load_case);
    let on_height = binding(wall, "heightM", Affine { base: 0.0, k: l }, "m", Quantise::Round { step: 0.05 });
    let x_axis = || axis("Aspect ratio h/ℓ", "Seitenverhältnis h/ℓ", QuantityKind::Dimensionless, "1", (0.3, 2.0));
    let class = m::annex_e::class_code(lat.edges);
    let mu = lat.m1 / lat.m2.max(1e-9);
    let all_fixed = {
        let fix = |e: m::numerical_models::Edge| if e == m::numerical_models::Edge::Simple { m::numerical_models::Edge::Fixed } else { e };
        m::numerical_models::Edges { top: fix(lat.edges.top), bottom: fix(lat.edges.bottom), left: fix(lat.edges.left), right: fix(lat.edges.right) }
    };
    let fixed_alphas: Vec<f64> = ratios.iter().map(|r| m::annex_e::standin_alpha1(all_fixed, mu, *r)).collect();
    let coefficient = finish(wall, Draft {
        id: format!("lateral-alpha:{}", wall.id),
        title: loc(&format!("Lateral coefficient α₁ – {}", wall.label_en), &format!("Biegebeiwert α₁ – {}", wall.label_de)),
        description: loc("α₁ = m₁/(w_Rd ℓ²) of the laterally loaded panel with the edge conditions of the held sides over the aspect ratio (EN 1996-1-1, Annex E, by yield lines).", "α₁ = m₁/(w_Rd ℓ²) der horizontal belasteten Wand mit den Randbedingungen der gehaltenen Seiten über dem Seitenverhältnis (EN 1996-1-1, Anhang E, nach Bruchlinientheorie)."),
        axes: vec![x_axis(), axis("Coefficient α₁", "Beiwert α₁", QuantityKind::Dimensionless, "1", (0.0, alphas.iter().fold(0.1_f64, |a, b| a.max(*b)) * 1.1))],
        series: vec![
            Series::Polyline { id: "alpha1".into(), label: loc("α₁(h/ℓ), yield-line stand-in", "α₁(h/ℓ), Bruchlinien-Ersatzwert"), points: ratios.iter().zip(&alphas).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Polyline { id: "alpha1-fixed".into(), label: loc("All supported edges fixed (stand-in, unverified)", "Alle gehaltenen Ränder eingespannt (Ersatzwert, ungeprüft)"), points: ratios.iter().zip(&fixed_alphas).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
        ],
        markers: vec![marker(Point { id: "wall", at: (at.clamp(0.3, 2.0), alpha), label: (format!("h/ℓ = {at:.2}, α₁ = {alpha:.4}"), format!("h/ℓ = {at:.2}, α₁ = {alpha:.4}")), utilisation: Some(lat.w_ed / w_rd.max(1e-9)), bound: known(report, &check), binding: Some(on_height.clone()) })],
        notes: vec![DiagramNote { id: "annex-e".into(), at: Vec2::new(0.35, 0.0), text: loc(&format!("Annex E class {class}: tabulated cells not keyed (unverified); curves are the Johansen yield-line stand-in"), &format!("Anhang-E-Klasse {class}: Tabellenwerte nicht erfasst (ungeprüft); die Kurven sind der Bruchlinien-Ersatzwert")), tone: Tone::Warning }],
        table: vec![column("Aspect ratio h/ℓ", "Seitenverhältnis h/ℓ", "1", ratios.clone()), column("Coefficient α₁", "Beiwert α₁", "1", alphas), column("α₁, all supported edges fixed", "α₁, alle gehaltenen Ränder eingespannt", "1", fixed_alphas)],
    });
    let capacity = finish(wall, Draft {
        id: format!("lateral-capacity:{}", wall.id),
        title: loc(&format!("Lateral capacity w_Rd – {}", wall.label_en), &format!("Horizontale Tragfähigkeit w_Rd – {}", wall.label_de)),
        description: loc("Collapse pressure w_Rd of the panel over the aspect ratio against the design wind pressure w_Ed (EN 1996-1-1, 6.3.2).", "Bruchlast w_Rd der Wand über dem Seitenverhältnis gegenüber dem Bemessungswinddruck w_Ed (EN 1996-1-1, 6.3.2)."),
        axes: vec![x_axis(), axis("Pressure w", "Druck w", QuantityKind::Pressure, "Pa", (0.0, pressures.iter().fold(lat.w_ed, |a, b| a.max(*b)) * 1.1))],
        series: vec![
            Series::Polyline { id: "w-rd".into(), label: loc("w_Rd(h/ℓ)", "w_Rd(h/ℓ)"), points: ratios.iter().zip(&pressures).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "w-ed".into(), label: loc("Design pressure w_Ed", "Bemessungsdruck w_Ed"), axis: AxisRef::Y, value: lat.w_ed, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![marker(Point { id: "wall", at: (at.clamp(0.3, 2.0), w_rd), label: (format!("w_Rd = {w_rd:.0} Pa at h/ℓ = {at:.2}"), format!("w_Rd = {w_rd:.0} Pa bei h/ℓ = {at:.2}")), utilisation: Some(lat.w_ed / w_rd.max(1e-9)), bound: known(report, &check), binding: Some(on_height) })],
        notes: Vec::new(),
        table: vec![column("Aspect ratio h/ℓ", "Seitenverhältnis h/ℓ", "1", ratios), column("w_Rd", "w_Rd", "Pa", pressures)],
    });
    Some(vec![coefficient, capacity])
}

fn fire(wall: &MasonryWall, report: &CheckReport) -> Option<Diagram2d> {
    if wall.fire_rei_min == 0 {
        return None;
    }
    let mut series = Vec::new();
    let mut table = vec![column("Fire class R", "Feuerwiderstandsklasse R", "min", FIRE_CLASSES.to_vec())];
    let values: Vec<f64> = FIRE_CLASSES.iter().map(|r| m::fire_min_thickness_m(*r as u32, wall.unit_material) * 1000.0).collect();
    table.push(column("t_min", "t_min", "mm", values.clone()));
    series.push(Series::Polyline { id: "t-min".into(), label: loc("t_min (tabulated)", "t_min (tabellarisch)"), points: FIRE_CLASSES.iter().zip(values).map(|(x, y)| Vec2::new(*x, y)).collect(), style: stroke(Tone::Warning, 1.5, Dash::Solid) });
    series.push(Series::Limit { id: "thickness".into(), label: loc("Wall thickness t", "Wanddicke t"), axis: AxisRef::Y, value: wall.thickness_m * 1000.0, style: stroke(Tone::Info, 2.0, Dash::Dashed) });
    let check = format!("en1996.1-2.fire.{}", wall.id);
    let result = find(report, &check);
    let t_min = result.map_or(0.0, |c| c.computed.value * 1000.0);
    let rating = f64::from(wall.fire_rei_min);
    let y_max = table.iter().skip(1).flat_map(|c| c.values.iter().copied()).fold(wall.thickness_m * 1000.0, f64::max) * 1.1;
    Some(finish(wall, Draft {
        id: format!("fire:{}", wall.id),
        title: loc(&format!("Tabulated fire thickness – {}", wall.label_en), &format!("Tabellierte Brandwanddicke – {}", wall.label_de)),
        description: loc("Minimum wall thickness for the fire class R for the unit material, group and mortar (EN 1996-1-2, tabulated base row); the marker drags the required class.", "Mindestwanddicke für die Feuerwiderstandsklasse R bei Steinart, -gruppe und Mörtel (EN 1996-1-2, tabellarische Daten); der Marker verschiebt die geforderte Klasse."),
        axes: vec![axis("Fire class R", "Feuerwiderstandsklasse R", QuantityKind::Time, "min", (0.0, 250.0)), axis("Thickness t", "Dicke t", QuantityKind::Length, "mm", (0.0, y_max))],
        series,
        markers: vec![marker(Point { id: "required", at: (rating, t_min), label: (format!("R {rating:.0} needs t ≥ {t_min:.0} mm"), format!("R {rating:.0} braucht t ≥ {t_min:.0} mm")), utilisation: result.map(|c| c.utilization), bound: known(report, &check), binding: Some(binding(wall, "fireReiMin", Affine::IDENTITY, "min", Quantise::Choice { values: FIRE_CLASSES.to_vec() })) })],
        notes: Vec::new(),
        table,
    }))
}

fn interaction(wall: &MasonryWall, prof: &Profile, report: &CheckReport) -> Option<Diagram2d> {
    if !wall.reinforced || wall.as_vertical_m2 <= 0.0 || wall.f_yd_pa <= 0.0 {
        return None;
    }
    let l = m::effective_length_m(wall);
    let section = m::numerical_models::MnSection { b: 1.0, depth: wall.thickness_m, f_d: prof.f_d, f_yd: wall.f_yd_pa, e_s: 200e9, bars: vec![(0.5 * wall.thickness_m, wall.as_vertical_m2 / l)] };
    let curve: Vec<(f64, f64)> = section.curve(48).into_iter().filter(|(n, mo)| n.is_finite() && mo.is_finite()).collect();
    let (n_ed, m_ed) = (prof.n[1] / l, prof.wind * wall.height_m.powi(2) / 8.0);
    let m_rd = section.m_rd(n_ed);
    let kn: Vec<f64> = curve.iter().map(|q| q.0 / 1000.0).collect();
    let knm: Vec<f64> = curve.iter().map(|q| q.1 / 1000.0).collect();
    let (x_lo, x_hi) = (kn.iter().fold(0.0_f64, |a, b| a.min(*b)), kn.iter().fold(n_ed / 1000.0, |a, b| a.max(*b)) * 1.05);
    let y_hi = knm.iter().fold(m_ed / 1000.0, |a, b| a.max(*b)) * 1.1;
    Some(finish(wall, Draft {
        id: format!("mn:{}", wall.id),
        title: loc(&format!("M–N interaction per metre – {}", wall.label_en), &format!("M-N-Interaktion je Meter – {}", wall.label_de)),
        description: loc("Plane-section capacity of the reinforced wall strip with ε_mu = 0.0035 and the rectangular block 0.8x (EN 1996-1-1, 6.6) against the wind moment and the mid-height force per metre.", "Querschnittstragfähigkeit des bewehrten Wandstreifens mit ε_mu = 0,0035 und Rechteckblock 0,8x (EN 1996-1-1, 6.6) gegenüber Windmoment und Normalkraft in Wandmitte je Meter."),
        axes: vec![axis("Axial force N", "Normalkraft N", QuantityKind::ForcePerLength, "kN/m", (x_lo, x_hi)), axis("Moment M", "Moment M", QuantityKind::Force, "kN·m/m", (0.0, y_hi))],
        series: vec![Series::Polyline { id: "mn".into(), label: loc("M_Rd(N)", "M_Rd(N)"), points: kn.iter().zip(&knm).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers: vec![marker(Point { id: "demand", at: (n_ed / 1000.0, m_ed / 1000.0), label: (format!("N = {:.0} kN/m, M = {:.2} kNm/m, M_Rd = {:.2} kNm/m", n_ed / 1000.0, m_ed / 1000.0, m_rd / 1000.0), format!("N = {:.0} kN/m, M = {:.2} kNm/m, M_Rd = {:.2} kNm/m", n_ed / 1000.0, m_ed / 1000.0, m_rd / 1000.0)), utilisation: Some(m_ed / m_rd.max(1e-9)), bound: known(report, &format!("en1996.6.6.reinforced.{}", wall.id)), binding: None })],
        notes: Vec::new(),
        table: vec![column("Axial force N", "Normalkraft N", "kN/m", kn), column("Moment M_Rd", "Moment M_Rd", "kN·m/m", knm)],
    }))
}

fn prestress(wall: &MasonryWall, report: &CheckReport) -> Option<Diagram2d> {
    if wall.prestress.is_empty() {
        return None;
    }
    let force_binding = |id: &str| FieldBinding { path: format!("walls[id={}].prestress[id={id}].forceN", wall.id), map: Affine { base: 0.0, k: 1000.0 }, unit: "N".to_string(), quantise: Some(Quantise::Round { step: 100.0 }) };
    let stages = vec![0.0, 1.0];
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut table = vec![column("Stage (0 transfer, 1 long term)", "Stufe (0 Vorspannen, 1 Langzeit)", "1", stages.clone())];
    let mut y_max = 0.0_f64;
    for tendon in &wall.prestress {
        let (p0, p_inf) = (tendon.force_n / 1000.0, tendon.force_n * (1.0 - tendon.loss_ratio) / 1000.0);
        y_max = y_max.max(p0);
        series.push(Series::Polyline { id: format!("p-{}", tendon.id), label: loc(&format!("Prestress {}", tendon.id), &format!("Vorspannung {}", tendon.id)), points: vec![Vec2::new(0.0, p0), Vec2::new(1.0, p_inf)], style: stroke(Tone::Info, 2.0, Dash::Solid) });
        table.push(column(&format!("P {}", tendon.id), &format!("P {}", tendon.id), "kN", vec![p0, p_inf]));
        let result = report.checks.iter().find(|c| c.id.starts_with("en1996.m-44.") && c.id.ends_with(&format!("{}.prestress.{}", wall.id, tendon.id)));
        markers.push(marker(Point { id: &format!("p0-{}", tendon.id), at: (0.0, p0), label: (format!("P0 = {p0:.1} kN, P∞ = {p_inf:.1} kN"), format!("P0 = {p0:.1} kN, P∞ = {p_inf:.1} kN")), utilisation: result.map(|c| c.utilization), bound: result.map(|c| c.id.clone()), binding: Some(force_binding(&tendon.id)) }));
    }
    Some(finish(wall, Draft {
        id: format!("prestress:{}", wall.id),
        title: loc(&format!("Prestress force over time – {}", wall.label_en), &format!("Vorspannkraft über die Zeit – {}", wall.label_de)),
        description: loc("Initial force P0 and the long-term force after the losses of every tendon (EN 1996-1-1, 3.4 and 6.8); the marker drags P0.", "Anfangskraft P0 und Langzeitkraft nach den Verlusten je Spannglied (EN 1996-1-1, 3.4 und 6.8); die Markierung verschiebt P0."),
        axes: vec![axis("Stage", "Stufe", QuantityKind::Dimensionless, "1", (0.0, 1.0)), axis("Prestressing force P", "Vorspannkraft P", QuantityKind::Force, "kN", (0.0, y_max * 1.1))],
        series,
        markers,
        notes: Vec::new(),
        table,
    }))
}

/// 🧭️ Plan view of the shear walls with the centre of stiffness, the load line and the in-plane share V_i of every wall against V_Rd,i (EN 1996-1-1, 5.4, M-21); the markers drag the plan position of the wall.
fn bracing(document: &En1996Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let case = m::bracing_case(document.annex, &document.walls)?;
    let b = m::bracing_distribution(&case).ok()?;
    let rule = if case.so("diaphragm", "rigid") == "flexible" { ("by tributary width (flexible diaphragm, EN 1998-1 4.3.1(4))", "nach Einflussbreite (verformbare Scheibe, EN 1998-1 4.3.1(4))") } else { ("by stiffness (bending plus shear flexibility)", "nach Steifigkeit (Biege- und Schubverformung)") };
    let walls: Vec<&MasonryWall> = document.walls.iter().filter(|w| matches!(w.wall_type, WallType::Shear) && w.in_plane_shear_n > 0.0).collect();
    let focus = walls.first()?;
    let entry = report.checks.iter().find(|c| c.id.starts_with("en1996.m-21."));
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut table = vec![column("Plan x", "Grundriss x", "m", walls.iter().map(|w| w.plan_x_m).collect()), column("Plan y", "Grundriss y", "m", walls.iter().map(|w| w.plan_y_m).collect())];
    let (mut x_lo, mut x_hi, mut y_lo, mut y_hi) = (b.centre, b.centre, 0.0_f64, 0.0_f64);
    for (i, wall) in walls.iter().enumerate() {
        let angle = wall.plan_angle_deg.to_radians();
        let (x0, y0) = (wall.plan_x_m, wall.plan_y_m);
        let (x1, y1) = (x0 + wall.length_m * angle.cos(), y0 + wall.length_m * angle.sin());
        let ratio = b.shares[i] / b.resistances[i].max(1e-9);
        x_lo = x_lo.min(x0).min(x1);
        x_hi = x_hi.max(x0).max(x1);
        y_lo = y_lo.min(y0).min(y1);
        y_hi = y_hi.max(y0).max(y1);
        series.push(Series::Polyline { id: format!("wall-{}", wall.id), label: loc(&wall.label_en, &wall.label_de), points: vec![Vec2::new(x0, y0), Vec2::new(x1, y1)], style: stroke(utilisation_tone(ratio), 2.0 + 6.0 * wall.thickness_m, Dash::Solid) });
        let at = (0.5 * (x0 + x1), 0.5 * (y0 + y1));
        let (kn, rd) = (b.shares[i] / 1000.0, b.resistances[i] / 1000.0);
        let label = (format!("{}: V = {kn:.1} kN of {rd:.1} kN (direct {:.1} kN), η = {ratio:.2}", wall.label_en, b.direct[i] / 1000.0), format!("{}: V = {kn:.1} kN von {rd:.1} kN (direkt {:.1} kN), η = {ratio:.2}", wall.label_de, b.direct[i] / 1000.0));
        markers.push(marker(Point { id: &format!("share-{}", wall.id), at, label, utilisation: Some(ratio), bound: entry.map(|c| c.id.clone()), binding: Some(binding(wall, "planXM", Affine { base: -0.5 * (x1 - x0), k: 1.0 }, "m", Quantise::Round { step: 0.01 })) }));
    }
    table.push(column("Stiffness k", "Steifigkeit k", "N/m", b.stiffness.clone()));
    table.push(column("Direct share", "Direkter Anteil", "N", b.direct.clone()));
    table.push(column("Share V_i", "Anteil V_i", "N", b.shares.clone()));
    table.push(column("Resistance V_Rd,i", "Widerstand V_Rd,i", "N", b.resistances.clone()));
    series.push(Series::Limit { id: "centre".into(), label: loc("Centre of stiffness", "Steifigkeitsmittelpunkt"), axis: AxisRef::X, value: b.centre, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) });
    series.push(Series::Limit { id: "load-line".into(), label: loc("Load line with accidental eccentricity", "Lastachse mit ungewollter Ausmitte"), axis: AxisRef::X, value: b.centre + b.eccentricity, style: stroke(Tone::Warning, 1.0, Dash::Dotted) });
    let pad = 0.1 * (x_hi - x_lo).max(y_hi - y_lo).max(1.0);
    Some(finish(focus, Draft {
        id: "bracing:document".into(),
        title: loc("Bracing layout and in-plane force distribution", "Aussteifungsanordnung und Verteilung der Scheibenkräfte"),
        description: loc(&format!("Plan of the shear walls: storey shear {:.1} kN distributed {} and the torsion M_t = {:.1} kNm about the centre of stiffness x_c = {:.2} m (eccentricity {:.3} m); wall colour by V_i / V_Rd,i.", case.o("v_ed_n", 0.0) / 1000.0, rule.0, b.torsion / 1000.0, b.centre, b.eccentricity), &format!("Grundriss der Schubwände: Geschossquerkraft {:.1} kN verteilt {} und Torsion M_t = {:.1} kNm um den Steifigkeitsmittelpunkt x_c = {:.2} m (Ausmitte {:.3} m); Wandfarbe nach V_i / V_Rd,i.", case.o("v_ed_n", 0.0) / 1000.0, rule.1, b.torsion / 1000.0, b.centre, b.eccentricity)),
        axes: vec![axis("Plan x", "Grundriss x", QuantityKind::Length, "m", (x_lo - pad, x_hi + pad)), axis("Plan y", "Grundriss y", QuantityKind::Length, "m", (y_lo - pad, y_hi + pad))],
        series,
        markers,
        notes: vec![DiagramNote { id: "torsion".into(), at: Vec2::new(b.centre, 0.5 * (y_lo + y_hi)), text: loc(&format!("M_t = V·e = {:.1} kNm", b.torsion / 1000.0), &format!("M_t = V·e = {:.1} kNm", b.torsion / 1000.0)), tone: Tone::Neutral }],
        table,
    }))
}

/// 🕳️ The chase table of EN 1996-1-1 8.6.2 (unverified registry rows `ndp.en1996.chase.*`): the depth that needs no calculation in Table 8.1 (vertical chases and recesses) and Table 8.2 (horizontal chases, short and long) over the wall thickness, with the design point (depth, wall thickness) of every chase; the marker drags the depth.
fn chase_table(document: &En1996Snapshot, wall: &MasonryWall, report: &CheckReport) -> Option<Diagram2d> {
    let chases: Vec<_> = document.chases.iter().filter(|c| c.wall_id == wall.id).collect();
    if chases.is_empty() {
        return None;
    }
    let annex = document.annex;
    let edges: Vec<f64> = m::chase_band_edges_m(annex).iter().map(|e| e * 1000.0).collect();
    let t_max = edges.last().copied().unwrap_or(0.0).max(wall.thickness_m * 1000.0) * 1.2;
    let uppers: Vec<f64> = edges.iter().skip(1).copied().chain(std::iter::once(t_max)).collect();
    let depth_at = |kind: ChaseKind, length: f64, upper: f64| m::chase_limits(kind, upper * 0.001, length, f64::INFINITY, 1.0, annex).map_or(0.0, |l| l.depth_m) * 1000.0;
    let short = m::chase_rule(&["h", "short-length"], annex);
    let curve = |kind: ChaseKind, length: f64| -> Vec<Vec2> { edges.iter().zip(&uppers).flat_map(|(lo, hi)| [Vec2::new(depth_at(kind, length, *hi), *lo), Vec2::new(depth_at(kind, length, *hi), *hi)]).collect() };
    let series = vec![
        Series::Polyline { id: "table-8-1".into(), label: loc("Table 8.1 vertical chase or recess (unverified)", "Tabelle 8.1 senkrechter Schlitz oder Aussparung (ungeprüft)"), points: curve(ChaseKind::Vertical, 0.0), style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Polyline { id: "table-8-2-short".into(), label: loc("Table 8.2 horizontal chase up to the short length (unverified)", "Tabelle 8.2 waagerechter Schlitz bis zur Kurzlänge (ungeprüft)"), points: curve(ChaseKind::Horizontal, short), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
        Series::Polyline { id: "table-8-2-long".into(), label: loc("Table 8.2 horizontal chase of unlimited length (unverified)", "Tabelle 8.2 waagerechter Schlitz unbegrenzter Länge (ungeprüft)"), points: curve(ChaseKind::Horizontal, f64::INFINITY), style: stroke(Tone::Neutral, 2.0, Dash::Dashed) },
    ];
    let x_max = series.iter().filter_map(|s| if let Series::Polyline { points, .. } = s { Some(points.iter().map(|p| p.x).fold(0.0, f64::max)) } else { None }).chain(chases.iter().map(|c| c.depth_m * 1000.0)).fold(0.0, f64::max) * 1.2;
    let table = vec![
        column("Wall thickness up to", "Wanddicke bis", "mm", uppers.clone()),
        column("Vertical chase depth (Table 8.1)", "Tiefe senkrechter Schlitz (Tabelle 8.1)", "mm", uppers.iter().map(|u| depth_at(ChaseKind::Vertical, 0.0, *u)).collect()),
        column("Horizontal chase depth, short (Table 8.2)", "Tiefe waagerechter Schlitz, kurz (Tabelle 8.2)", "mm", uppers.iter().map(|u| depth_at(ChaseKind::Horizontal, short, *u)).collect()),
        column("Horizontal chase depth, long (Table 8.2)", "Tiefe waagerechter Schlitz, lang (Tabelle 8.2)", "mm", uppers.iter().map(|u| depth_at(ChaseKind::Horizontal, f64::INFINITY, *u)).collect()),
    ];
    let t = wall.thickness_m * 1000.0;
    let markers = chases
        .iter()
        .map(|c| {
            let check = format!("en1996.8.6.chase-depth.{}", c.id);
            let depth = c.depth_m * 1000.0;
            let label = (format!("{} {}: d = {depth:.0} mm in t = {t:.0} mm (remaining {:.0} mm)", m::kind_words(c.kind).0, c.id, t - depth), format!("{} {}: d = {depth:.0} mm bei t = {t:.0} mm (Rest {:.0} mm)", m::kind_words(c.kind).1, c.id, t - depth));
            let drag = FieldBinding { path: format!("chases[id={}].depthM", c.id), map: Affine { base: 0.0, k: 0.001 }, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.005 }) };
            marker(Point { id: &format!("chase-{}", c.id), at: (depth, t), label, utilisation: find(report, &check).map(|r| r.utilization), bound: known(report, &check), binding: Some(drag) })
        })
        .collect();
    Some(finish(wall, Draft {
        id: format!("chase-table:{}", wall.id),
        title: loc(&format!("Chase table – {}", wall.label_en), &format!("Schlitztabelle – {}", wall.label_de)),
        description: loc("Depth of a chase or recess that needs no calculation over the wall thickness: Table 8.1 for vertical chases and recesses, Table 8.2 for horizontal chases (short and long); the circles are the design points (depth, wall thickness) of the chases, a circle left of its curve needs no calculation. EN 1996-1-1 8.6.2, unverified. The markers drag the depth.", "Tiefe eines Schlitzes oder einer Aussparung, die keinen Nachweis braucht, über der Wanddicke: Tabelle 8.1 für senkrechte Schlitze und Aussparungen, Tabelle 8.2 für waagerechte Schlitze (kurz und lang); die Kreise sind die Bemessungspunkte (Tiefe, Wanddicke) der Schlitze, ein Kreis links seiner Kurve braucht keinen Nachweis. EN 1996-1-1 8.6.2, ungeprüft. Die Marker verschieben die Tiefe."),
        axes: vec![axis("Chase depth d", "Schlitztiefe d", QuantityKind::Length, "mm", (0.0, x_max.max(1.0))), axis("Wall thickness t", "Wanddicke t", QuantityKind::Length, "mm", (0.0, t_max))],
        series,
        markers,
        notes: Vec::new(),
        table,
    }))
}

/// 🧱️ The panel widths between the movement joints of the wall against the recommended spacing (EN 1996-2, unverified `ndp.en1996.m52.joints`); the markers drag the joint positions.
fn movement_joints(document: &En1996Snapshot, wall: &MasonryWall, report: &CheckReport) -> Option<Diagram2d> {
    let mut joints: Vec<_> = document.movement_joints.iter().filter(|j| j.wall_id == wall.id).collect();
    if joints.is_empty() {
        return None;
    }
    joints.sort_by(|a, b| a.position_m.total_cmp(&b.position_m));
    let l = wall.length_m;
    let limit = m::joint_spacing_limit_m(wall.unit_material);
    let mut edges = vec![0.0];
    edges.extend(joints.iter().map(|j| j.position_m.clamp(0.0, l)));
    edges.push(l);
    let widths: Vec<f64> = edges.windows(2).map(|w| w[1] - w[0]).collect();
    let mut points = Vec::new();
    for (index, width) in widths.iter().enumerate() {
        points.push(Vec2::new(edges[index], *width));
        points.push(Vec2::new(edges[index + 1], *width));
    }
    let table = vec![column("Panel start", "Feldanfang", "m", edges[..edges.len() - 1].to_vec()), column("Panel width", "Feldbreite", "m", widths.clone())];
    let series = vec![
        Series::Polyline { id: "panels".into(), label: loc("Panel width", "Feldbreite"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Limit { id: "limit".into(), label: loc("Recommended spacing (unverified)", "Empfohlener Abstand (ungeprüft)"), axis: AxisRef::Y, value: limit, style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
    ];
    let check = format!("en1996.8.5.joint-spacing.{}", wall.id);
    let markers = joints
        .iter()
        .enumerate()
        .map(|(index, j)| {
            let at = (edges[index + 1], widths[index].max(widths[index + 1]));
            let label = (format!("Joint {} at x = {:.2} m", j.id, j.position_m), format!("Dehnfuge {} bei x = {:.2} m", j.id, j.position_m));
            let drag = FieldBinding { path: format!("movementJoints[id={}].positionM", j.id), map: Affine::IDENTITY, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.01 }) };
            marker(Point { id: &format!("joint-{}", j.id), at, label, utilisation: find(report, &check).map(|r| r.utilization), bound: known(report, &check), binding: Some(drag) })
        })
        .collect();
    let y_max = widths.iter().copied().fold(limit, f64::max) * 1.1;
    Some(finish(wall, Draft {
        id: format!("movement-joints:{}", wall.id),
        title: loc(&format!("Movement joint spacing – {}", wall.label_en), &format!("Dehnfugenabstand – {}", wall.label_de)),
        description: loc("Width of the panels between the movement joints along the wall against the recommended spacing for the unit material (unverified); the markers drag the joint positions.", "Breite der Felder zwischen den Dehnfugen entlang der Wand gegen den empfohlenen Abstand für die Steinart (ungeprüft); die Marker verschieben die Fugenlage."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", (0.0, l)), axis("Panel width", "Feldbreite", QuantityKind::Length, "m", (0.0, y_max))],
        series,
        markers,
        notes: Vec::new(),
        table,
    }))
}

/// 🏋️ The enhancement β of Eq. 6.10 of every concentrated load of the wall over a1/H: the curve at the loaded area of the load, the envelope for a vanishing loaded area and the lower bound 1; the circle is the design point and drags the distance a1.
fn bearing_enhancement(wall: &MasonryWall, report: &CheckReport) -> Vec<Diagram2d> {
    let h = wall.height_m;
    let mut out = Vec::new();
    for lc in &wall.load_cases {
        for c in &lc.concentrated {
            let (ab, a_ef, beta) = super::spreading::bearing_terms(wall, c);
            let at = c.distance_a1_m.max(0.0) / h.max(1e-9);
            let x_max = (1.2 * at).max(2.0);
            let ratios: Vec<f64> = curves::sample(|x| x, 0.0, x_max, SAMPLES).iter().map(|q| q[0]).collect();
            let own: Vec<f64> = ratios.iter().map(|x| m::concentrated_beta(x * h, h, ab, a_ef)).collect();
            let upper: Vec<f64> = ratios.iter().map(|x| m::concentrated_beta(x * h, h, 0.0, a_ef)).collect();
            let check = format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id);
            let line = |values: &[f64]| ratios.iter().zip(values).map(|(x, y)| Vec2::new(*x, *y)).collect::<Vec<_>>();
            let y_max = upper.iter().copied().fold(beta, f64::max) * 1.05;
            out.push(finish(wall, Draft {
                id: format!("beta:{}-{}-{}", wall.id, lc.id, c.id),
                title: loc(&format!("Concentrated load enhancement β – {} {}", wall.label_en, c.id), &format!("Erhöhungsfaktor β der Einzellast – {} {}", wall.label_de, c.id)),
                description: loc("Enhancement factor β of a concentrated load over the distance a1 from the wall end to the load, in units of the wall height; the curve is β at the loaded area of this load, the dashed curve the envelope for a vanishing loaded area, the dotted line β = 1; the circle is the design point and drags a1.", "Erhöhungsfaktor β einer Einzellast über dem Abstand a1 des Lastrands vom Wandende in Einheiten der Wandhöhe; die Kurve ist β bei der belasteten Fläche dieser Last, die gestrichelte Kurve die Hüllkurve für verschwindende belastete Fläche, die gepunktete Linie β = 1; der Kreis ist der Bemessungspunkt und verschiebt a1."),
                axes: vec![axis("Distance a1 / wall height H", "Abstand a1 / Wandhöhe H", QuantityKind::Dimensionless, "1", (0.0, x_max)), axis("Enhancement β", "Erhöhungsfaktor β", QuantityKind::Dimensionless, "1", (0.9, y_max.max(1.1)))],
                series: vec![
                    Series::Polyline { id: "beta".into(), label: loc("β(a1/H) at the loaded area A_b of this load", "β(a1/H) bei der belasteten Fläche A_b dieser Last"), points: line(&own), style: stroke(Tone::Info, 2.5, Dash::Solid) },
                    Series::Polyline { id: "beta-envelope".into(), label: loc("Envelope for A_b → 0", "Hüllkurve für A_b → 0"), points: line(&upper), style: stroke(Tone::Warning, 1.5, Dash::Dashed) },
                    Series::Limit { id: "beta-one".into(), label: loc("No enhancement β = 1", "Keine Erhöhung β = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
                ],
                markers: vec![marker(Point { id: "load", at: (at.min(x_max), beta), label: (format!("a1/H = {at:.2}, β = {beta:.3} (A_b/A_ef = {:.2})", ab / a_ef.max(1e-9)), format!("a1/H = {at:.2}, β = {beta:.3} (A_b/A_ef = {:.2})", ab / a_ef.max(1e-9))), utilisation: find(report, &check).map(|r| r.utilization), bound: known(report, &check), binding: Some(FieldBinding { path: format!("walls[id={}].loadCases[id={}].concentrated[id={}].distanceA1M", wall.id, lc.id, c.id), map: Affine { base: 0.0, k: h }, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.05 }) }) })],
                notes: Vec::new(),
                table: vec![column("Distance a1 / H", "Abstand a1 / H", "1", ratios.clone()), column("β at the loaded area", "β bei der belasteten Fläche", "1", own), column("Envelope for A_b → 0", "Hüllkurve für A_b → 0", "1", upper)],
            }));
        }
    }
    out
}

/// 📊️ The outcome diagram of a wall: one bar per check owned by the wall or one of its sub-entities whose family no dedicated diagram draws, at its utilisation against the limit line at 1, coloured by the shared utilisation ramp; each bar has a marker that carries its check.
fn outcomes(document: &En1996Snapshot, wall: &MasonryWall, report: &CheckReport) -> Option<Diagram2d> {
    let checks: Vec<&CheckResult> = report
        .checks
        .iter()
        .filter(|c| c.utilization.is_finite() && c.status != super::norm::CheckStatus::NotApplicable && !is_document_check(&c.id) && !is_drawn_family(&c.id))
        .filter(|c| owner_entity(document, c).is_some_and(|owner| base_id(&owner) == wall.id))
        .collect();
    if checks.is_empty() {
        return None;
    }
    let worst = checks.iter().map(|c| c.utilization).fold(0.0, f64::max);
    let mut series: Vec<Series> = checks
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let tone = utilisation_tone(c.utilization);
            let x = (i as f64 + 0.6, i as f64 + 1.4);
            Series::Polygon { id: format!("outcome-{}", i + 1), label: loc(&c.title.en, &c.title.de), ring: vec![Vec2::new(x.0, 0.0), Vec2::new(x.1, 0.0), Vec2::new(x.1, c.utilization), Vec2::new(x.0, c.utilization)], fill: Fill { tone, opacity: 0.55 }, hatch: (tone == Tone::Error).then_some(Hatch::Diagonal) }
        })
        .collect();
    series.push(Series::Limit { id: "limit".into(), label: loc("Limit of the check (utilisation 1)", "Grenze des Nachweises (Ausnutzung 1)"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    let markers = checks.iter().enumerate().map(|(i, c)| marker(Point { id: &format!("check-{}", i + 1), at: (i as f64 + 1.0, c.utilization), label: (format!("{}: utilisation {:.2}", c.title.en, c.utilization), format!("{}: Ausnutzung {:.2}", c.title.de, c.utilization)), utilisation: Some(c.utilization), bound: Some(c.id.clone()), binding: None })).collect();
    Some(finish(wall, Draft {
        id: format!("check-outcomes:{}", wall.id),
        title: loc(&format!("Check outcomes – {}, worst utilisation {worst:.2}", wall.label_en), &format!("Nachweisergebnisse – {}, größte Ausnutzung {worst:.2}", wall.label_de)),
        description: loc("Utilisation of every check of this wall and its members whose family has no dedicated diagram, as bars in check order against the limit at 1; the legend names each check.", "Ausnutzung jedes Nachweises dieser Wand und ihrer Bauteile, dessen Familie kein eigenes Diagramm hat, als Balken in Nachweisreihenfolge gegen die Grenze 1; die Legende nennt jeden Nachweis."),
        axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", (0.0, checks.len() as f64 + 1.0)), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", (0.0, worst.max(1.0) * 1.1))],
        series,
        markers,
        notes: Vec::new(),
        table: vec![column("Check", "Nachweis", "1", (1..=checks.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", checks.iter().map(|c| c.utilization).collect())],
    }))
}

/// 📈️ Every diagram of the document; with `focus` set to an entity id only the diagrams of that wall (derived scene ids resolve to their wall).
pub fn diagrams(document: &En1996Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1996, document.annex))
}

fn diagrams_unbased(document: &En1996Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for wall in document.walls.iter().filter(|w| focus.is_none_or(|f| base_id(f) == w.id)) {
        if let Some(prof) = profile::governing(document, wall) {
            out.push(eccentricity(wall, &prof, report));
            out.push(reduction(wall, &prof, report));
            out.push(capacity(wall, &prof, report));
            out.push(slenderness(document, wall, &prof, report));
            out.push(shear(document, wall, &prof, report));
            out.extend(interaction(wall, &prof, report));
        }
        out.push(strength(document, wall, report));
        out.extend(lateral(document, wall, report).into_iter().flatten());
        out.extend(fire(wall, report));
        out.extend(prestress(wall, report));
        out.extend(chase_table(document, wall, report));
        out.extend(movement_joints(document, wall, report));
        out.extend(bearing_enhancement(wall, report));
        out.extend(outcomes(document, wall, report));
    }
    if focus.is_none_or(|f| document.walls.iter().any(|w| base_id(f) == w.id && matches!(w.wall_type, WallType::Shear))) {
        out.extend(bracing(document, report));
    }
    out.extend(super::diaphragm::diagrams(document, report, focus));
    for case in document.cases.iter().filter(|c| focus.is_none_or(|f| base_id(f) == c.id)) {
        out.extend(super::scene::projection_kit::design_case::case_diagram(&case.id, &loc(&case.label_en, &case.label_de), report, None));
    }
    out
}
