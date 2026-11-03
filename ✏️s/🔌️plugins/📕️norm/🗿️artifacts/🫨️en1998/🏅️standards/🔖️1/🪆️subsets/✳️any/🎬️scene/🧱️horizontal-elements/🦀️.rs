//! 🧱️ Horizontal elements in the scene: the diaphragm force flow (inertia, wall shear flow and chord forces of EN 1998-1 4.4.2.5), the animated build-up of the base shear over the storeys, the overturning base with its moment, the 3D response spectrum surface over period and damping and the accelerogram traces of a time-history set. Every quantity comes from `schema::horizontal` and `response`, the functions the checks use.

use super::handles::Index;
use super::kit::*;
use super::layout::Layout;
use super::model::{En1998Accelerogram, En1998Building, En1998Diaphragm};
use super::response::{respond, Response, Site};
use super::scene::*;
use super::schema::horizontal::{diaphragm, overturning, spectrum_surface, time_history};
use super::schema::part_1;
use super::SceneInput;

const FLOW_ARROWS: usize = 6;
const EDGE_ARROWS: usize = 5;
const FLOW_LIFT: f64 = 0.4;
pub const DISTRIBUTION_STEPS: usize = 8;
pub const DISTRIBUTION_FIELD: &str = "distribution";
const SHEAR_BLOCK_DEPTH: f64 = 2.0;
const SURFACE_PERIODS: usize = 33;
const SURFACE_T_MAX: f64 = 4.0;
const SURFACE_DAMPING_PERCENT: [f64; 9] = [2.0, 3.0, 5.0, 7.5, 10.0, 15.0, 20.0, 25.0, 30.0];
const SURFACE_X_PER_SECOND: f64 = 2.0;
const SURFACE_Y_PER_ROW: f64 = 0.75;
const SURFACE_Z_PER_MS2: f64 = 1.0;
const SURFACE_GAP: f64 = 8.0;
const TRACE_MAX_COLUMNS: usize = 120;
const TRACE_METRES_PER_SECOND: f64 = 0.5;
pub const TRACE_METRES_PER_MS2: f64 = 0.4;
const TRACE_WIDTH: f64 = 0.4;
const BASE_THICKNESS: f64 = 0.4;
const TRANSFER_FACTOR: &str = "ndp.en1998.en1.diaphragm.transfer-factor";
const STABILISING_FACTOR: &str = "ndp.en1998.en1.overturning.stabilising-factor";

/// 🎬 Projects the horizontal element visuals of every building, diaphragm and accelerogram the part owns.
pub fn project(input: &SceneInput, layout: &Layout, site: &Site, handles: &Index, out: &mut Out) {
    let d = input.document;
    for b in &d.buildings {
        if !out.own("buildings", &b.id) {
            continue;
        }
        let r = respond(site, b);
        let o = layout.origin(&b.id);
        base_shear_blocks(input, b, &r, &o, out);
        spectrum_surface_of(input, site, b, &r, &o, out);
        overturning_base(input, handles, b, &o, out);
    }
    for dia in &d.diaphragms {
        if !out.own("diaphragms", &dia.id) {
            continue;
        }
        let Some(b) = d.buildings.iter().find(|b| b.id == dia.building_id) else { continue };
        let Some(index) = b.storeys.iter().position(|s| s.id == dia.storey_id) else { continue };
        diaphragm_flow(input, layout, site, handles, b, index, dia, out);
    }
    for rec in &d.accelerograms {
        if !out.own("accelerograms", &rec.id) {
            continue;
        }
        let slot = d.accelerograms.iter().filter(|x| x.building_id == rec.building_id).position(|x| x.id == rec.id).unwrap_or(0);
        accelerogram_trace(input, layout, handles, slot, rec, out);
    }
}

fn gamma_d_of(input: &SceneInput, d: &En1998Diaphragm) -> f64 {
    if d.gamma_d > 0.0 {
        return d.gamma_d;
    }
    let annex = if input.document.annex == semio_s_plugin_norm_compliance::AnnexChoice::En { norm::AnnexChoice::En } else { norm::AnnexChoice::De };
    norm::ndp_number(TRANSFER_FACTOR, annex)
}

fn material_role(material: semio_s_artifact_norm_en1998_model::DiaphragmMaterial) -> MaterialRole {
    match material {
        semio_s_artifact_norm_en1998_model::DiaphragmMaterial::Timber => MaterialRole::Timber,
        semio_s_artifact_norm_en1998_model::DiaphragmMaterial::SteelDeck => MaterialRole::Steel,
        semio_s_artifact_norm_en1998_model::DiaphragmMaterial::Concrete => MaterialRole::Concrete,
    }
}

fn force_arrow(id: String, path: &str, at: Vec3, direction: Vec3, newtons: f64, case: &str, name: LocalizedCopy, binding: &Option<UtilisationBinding>) -> Spec {
    let mut spec = Spec::new(EntityKind::Load, id, path, Geometry::Load(LoadGlyph::Point { at, direction, magnitude: Quantity::new(QuantityKind::Force, newtons), case: load_case(case) }), MaterialRole::Load { kind: LoadKind::Seismic }, name, LAYER_DIAPHRAGM_FLOW);
    spec.binding = binding.clone();
    spec
}

#[allow(clippy::too_many_arguments)]
fn diaphragm_flow(input: &SceneInput, layout: &Layout, site: &Site, handles: &Index, b: &En1998Building, index: usize, d: &En1998Diaphragm, out: &mut Out) {
    let o = layout.origin(&b.id);
    let r = respond(site, b);
    let top = r.elevations[index];
    let dp = path_of("diaphragms", &d.id);
    let gamma_d = gamma_d_of(input, d);
    let thickness = d.thickness_m.max(0.05);
    let corner = v3(o.x, o.y, o.z + top);
    let worst = binding_where(input.report, |c| c.subject.entity_id == d.id && c.id.contains(".diaphragm."));
    let shear = binding_where(input.report, |c| c.subject.entity_id == d.id && c.id.contains(".diaphragm.") && c.id.ends_with(".shear"));
    let chord = binding_where(input.report, |c| c.subject.entity_id == d.id && c.id.contains(".diaphragm.") && c.id.ends_with(".chord"));
    let mut plate = Spec::new(EntityKind::Plate, d.id.clone(), dp.clone(), Geometry::Plate(PlateGeometry { frame: world_frame(corner.clone()), outline: rect_ring(b.plan_width_m.max(0.1), b.plan_length_m.max(0.1)), thickness, offset: 0.0, stiffeners: Vec::new() }), material_role(d.material), loc(&format!("Diaphragm {} of storey {}", d.id, d.storey_id), &format!("Scheibe {} von Geschoss {}", d.id, d.storey_id)), LAYER_DIAPHRAGMS);
    plate.binding = worst.clone();
    plate.opacity = 0.7;
    plate.handles = handles.of(&d.id);
    plate.detail = loc(&format!("{:?} diaphragm, t = {:.3} m, shear strength {:.2} MPa, openings {:.2} m, chord {:.5} m² at {:.0} MPa", d.material, d.thickness_m, d.shear_strength_mpa, d.opening_width_m, d.chord_area_m2, d.chord_strength_mpa), &format!("Scheibe aus {:?}, t = {:.3} m, Schubfestigkeit {:.2} MPa, Öffnungen {:.2} m, Gurt {:.5} m² mit {:.0} MPa", d.material, d.thickness_m, d.shear_strength_mpa, d.opening_width_m, d.chord_area_m2, d.chord_strength_mpa));
    out.push(plate, &[corner.clone(), v3(o.x + b.plan_width_m, o.y + b.plan_length_m, o.z + top + thickness)]);
    let lift = thickness + FLOW_LIFT;
    for (k, system) in r.systems.iter().enumerate() {
        let f_i = system.forces.get(index).copied().unwrap_or(0.0) / system.delta.max(1e-9);
        let (plan_span, plan_depth) = if system.along_y { (b.plan_length_m, b.plan_width_m) } else { (b.plan_width_m, b.plan_length_m) };
        let (span, depth) = diaphragm::extent_m(d.span_m, d.depth_m, plan_span, plan_depth);
        if f_i <= 0.0 || span <= 0.0 || depth <= 0.0 {
            continue;
        }
        let (dir, cross) = if system.along_y { (Vec3::Y, Vec3::X) } else { (Vec3::X, Vec3::Y) };
        let at = |s: f64, c: f64| v3(corner.x + dir.x * s + cross.x * c, corner.y + dir.y * s + cross.y * c, corner.z + lift);
        let (f_d, v_ed) = (diaphragm::design_force_n(f_i, gamma_d), diaphragm::support_shear_n(diaphragm::design_force_n(f_i, gamma_d)));
        let n_ed = diaphragm::chord_force_n(f_d, span, depth);
        let case = format!("seismic-diaphragm-{}", system.id);
        let tag = |what: &str, i: usize| format!("{}:{}:{what}-{i}", d.id, system.id);
        for i in 0..FLOW_ARROWS {
            let s = span * (i as f64 + 0.5) / FLOW_ARROWS as f64;
            let name = loc(&format!("Inertia force F_d/{FLOW_ARROWS} = {:.1} kN of {} ({})", f_d / FLOW_ARROWS as f64 / 1000.0, d.id, system.id), &format!("Trägheitskraft F_d/{FLOW_ARROWS} = {:.1} kN von {} ({})", f_d / FLOW_ARROWS as f64 / 1000.0, d.id, system.id));
            out.push(force_arrow(tag("inertia", i), &dp, at(s, depth / 2.0), dir.clone(), f_d / FLOW_ARROWS as f64, &case, name, &None), &[at(s, depth / 2.0)]);
        }
        for (end, s) in [(0, 0.0), (1, span)] {
            for i in 0..EDGE_ARROWS {
                let c = depth * (i as f64 + 0.5) / EDGE_ARROWS as f64;
                let name = loc(&format!("Shear flow along wall {end}: V_Ed/{EDGE_ARROWS} = {:.1} kN (q = {:.1} kN/m)", v_ed / EDGE_ARROWS as f64 / 1000.0, diaphragm::shear_flow_n_per_m(v_ed, depth, d.opening_width_m) / 1000.0), &format!("Schubfluss an Wand {end}: V_Ed/{EDGE_ARROWS} = {:.1} kN (q = {:.1} kN/m)", v_ed / EDGE_ARROWS as f64 / 1000.0, diaphragm::shear_flow_n_per_m(v_ed, depth, d.opening_width_m) / 1000.0));
                out.push(force_arrow(tag(&format!("edge{end}"), i), &dp, at(s, c), v3(-dir.x, -dir.y, 0.0), v_ed / EDGE_ARROWS as f64, &case, name, &shear), &[at(s, c)]);
            }
        }
        for (what, sign, c, en, de) in [("chord-t", 1.0, 0.0, "Chord tension", "Gurtzug"), ("chord-c", -1.0, depth, "Chord compression", "Gurtdruck")] {
            let name = loc(&format!("{en} N_Ed = {:.1} kN at mid span ({})", n_ed / 1000.0, system.id), &format!("{de} N_Ed = {:.1} kN in Feldmitte ({})", n_ed / 1000.0, system.id));
            out.push(force_arrow(tag(what, k), &dp, at(span / 2.0, c), v3(dir.x * sign, dir.y * sign, 0.0), n_ed, &case, name, &chord), &[at(span / 2.0, c)]);
        }
        if k == 0 {
            let from = v3(corner.x, corner.y - 1.0, corner.z);
            let along = if system.along_y { v3(from.x, from.y + span, from.z) } else { v3(from.x + span, from.y, from.z) };
            out.annotate(Annotation::Dimension { id: format!("{}:span", d.id), from: from.clone(), to: along, offset: v3(0.0, -0.8, 0.0), value: Quantity::new(QuantityKind::Length, span), binding: Some(FieldBinding::new(format!("{dp}.spanM"), Affine::IDENTITY, "m")) }, &from);
        }
    }
    let label = shear.as_ref().map_or_else(|| format!("{} · t = {:.2} m", d.id, d.thickness_m), |s| format!("{} · t = {:.2} m · V_Ed/V_Rd = {:.2}", d.id, d.thickness_m, s.ratio));
    out.annotate(Annotation::Label { id: format!("{}:label", d.id), anchor: corner.clone(), text: loc(&label, &label), tone: tone_of(&worst) }, &corner);
    if let Some(clause) = worst.as_ref().and_then(|x| clause_of(input.report, &x.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", d.id), anchor: entity_ref(EntityKind::Plate, &d.id, &dp), clause }, &corner);
    }
    if worst.as_ref().is_some_and(|x| x.status == scene::CheckStatus::Fail) {
        out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", d.id), at: entity_ref(EntityKind::Plate, &d.id, &dp), mode: FailureMode::Other, extent: None }, &corner);
    }
}

fn base_shear_blocks(input: &SceneInput, b: &En1998Building, r: &Response, o: &Vec3, out: &mut Out) {
    let Some(system) = r.systems.iter().max_by(|a, c| a.f_b.total_cmp(&c.f_b)) else { return };
    let n = b.storeys.len();
    if system.f_b <= 0.0 || n == 0 || system.forces.len() != n {
        return;
    }
    let mut shear = vec![0.0; n];
    let mut run = 0.0;
    for i in (0..n).rev() {
        run += system.forces[i];
        shear[i] = run;
    }
    let v_b = shear[0].max(1e-9);
    let f_sum: f64 = system.forces.iter().sum::<f64>().max(1e-9);
    let (x0, y0) = (o.x - 6.0, o.y + b.plan_length_m / 2.0 - SHEAR_BLOCK_DEPTH / 2.0);
    let mut previous = 0.0;
    for (i, s) in b.storeys.iter().enumerate() {
        let top = r.elevations[i];
        let ratio = shear[i] / v_b;
        let width = 0.4 + 3.6 * ratio;
        let origin = v3(x0 - width / 2.0, y0, o.z + previous);
        let geometry = Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: world_frame(origin.clone()), base: rect_ring(width, SHEAR_BLOCK_DEPTH), height: (top - previous).max(0.1) } });
        let sp = format!("{}.storeys[id={}]", path_of("buildings", &b.id), s.id);
        let revealed = |k: usize| ((k as f64 / DISTRIBUTION_STEPS as f64) * n as f64).ceil() as usize;
        let field = |id: String, value: f64| ScalarField { id, unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: vec![value; 8] } };
        let mut fields: Vec<ScalarField> = (0..=DISTRIBUTION_STEPS).map(|k| field(format!("{DISTRIBUTION_FIELD}-{k}"), if i + revealed(k) >= n { ratio } else { 0.0 })).collect();
        fields.push(field("storey-force".into(), system.forces[i] / f_sum));
        let binding = binding_exact(input.report, &format!("en1998.1.{}.{}.storeyForces.{}", b.id, system.id, s.id));
        let mut spec = Spec::new(EntityKind::Solid, format!("{}:{}:shear-block", b.id, s.id), sp, geometry, MaterialRole::Neutral, loc(&format!("Storey shear of {} ({}): V = {:.0} kN of {:.0} kN", s.id, system.id, shear[i] / 1000.0, v_b / 1000.0), &format!("Geschossquerkraft von {} ({}): V = {:.0} kN von {:.0} kN", s.id, system.id, shear[i] / 1000.0, v_b / 1000.0)), LAYER_BASE_SHEAR);
        spec.binding = binding;
        spec.opacity = 0.85;
        spec.channel = Some(ColourChannel::Fields { active: format!("{DISTRIBUTION_FIELD}-{DISTRIBUTION_STEPS}"), fields });
        spec.detail = loc(&format!("F_i = {:.0} kN at storey {}, V_i = {:.0} kN, {:.0} % of the base shear", system.forces[i] / 1000.0, s.id, shear[i] / 1000.0, ratio * 100.0), &format!("F_i = {:.0} kN in Geschoss {}, V_i = {:.0} kN, {:.0} % der Gesamtquerkraft", system.forces[i] / 1000.0, s.id, shear[i] / 1000.0, ratio * 100.0));
        out.push(spec, &[origin.clone(), v3(origin.x + width, origin.y + SHEAR_BLOCK_DEPTH, o.z + top)]);
        let at = v3(x0 + width / 2.0 + 0.3, y0, o.z + (previous + top) / 2.0);
        let text = format!("V = {:.0} kN · F = {:.0} kN", shear[i] / 1000.0, system.forces[i] / 1000.0);
        out.annotate(Annotation::Label { id: format!("{}:{}:shear-label", b.id, s.id), anchor: at.clone(), text: loc(&text, &text), tone: tone_of(&binding_exact(input.report, &format!("en1998.1.{}.{}.storeyForces.{}", b.id, system.id, s.id))) }, &at);
        previous = top;
    }
    if !out.legends.iter().any(|l| l.id == "base-shear") {
        out.legends.push(Legend { id: "base-shear".into(), title: loc("Storey shear over base shear V_i / V_b", "Geschossquerkraft zur Gesamtquerkraft V_i / V_b"), scale: LegendScale::Scalar { field_id: format!("{DISTRIBUTION_FIELD}-{DISTRIBUTION_STEPS}"), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.0 }, unit: QuantityKind::Dimensionless } });
    }
}

fn spectrum_surface_of(input: &SceneInput, site: &Site, b: &En1998Building, r: &Response, o: &Vec3, out: &mut Out) {
    if site.a_gr <= 0.0 {
        return;
    }
    let a_g = site.gamma_i * site.a_gr;
    let periods: Vec<f64> = (0..SURFACE_PERIODS).map(|i| SURFACE_T_MAX * i as f64 / (SURFACE_PERIODS - 1) as f64).collect();
    let grid = spectrum_surface::grid(a_g, site.s, site.tb, site.tc, site.td, &periods, &SURFACE_DAMPING_PERCENT);
    let origin = v3(o.x, o.y + b.plan_length_m + SURFACE_GAP, o.z);
    let mut points = Vec::new();
    let mut values = Vec::new();
    let mut peak = 0.0_f64;
    for (row, line) in grid.iter().enumerate() {
        for (column, sa) in line.iter().enumerate() {
            points.push(v3(origin.x + periods[column] * SURFACE_X_PER_SECOND, origin.y + row as f64 * SURFACE_Y_PER_ROW, origin.z + sa * SURFACE_Z_PER_MS2));
            values.push(*sa);
            peak = peak.max(*sa);
        }
    }
    let path = path_of("buildings", &b.id);
    let id = format!("{}:spectrum-surface", b.id);
    let mut spec = Spec::new(EntityKind::Surface, id.clone(), path.clone(), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: SURFACE_DAMPING_PERCENT.len() as u32, columns: SURFACE_PERIODS as u32, points: points.clone() } } }), MaterialRole::Neutral, loc("Elastic response spectrum over period and damping", "Elastisches Antwortspektrum über Periode und Dämpfung"), LAYER_SPECTRUM_SURFACE);
    spec.channel = Some(ColourChannel::Fields { active: "spectrum-ordinate".into(), fields: vec![ScalarField { id: "spectrum-ordinate".into(), unit: QuantityKind::Acceleration, samples: FieldSamples::Vertex { values } }] });
    spec.opacity = 0.85;
    spec.detail = loc(&format!("S_e(T, ξ) of EN 1998-1 3.2.2.2 for a_g = {a_g:.2} m/s² and S = {:.2}, damping from {:.0} to {:.0} %, peak {peak:.2} m/s²", site.s, SURFACE_DAMPING_PERCENT[0], SURFACE_DAMPING_PERCENT[SURFACE_DAMPING_PERCENT.len() - 1]), &format!("S_e(T, ξ) nach EN 1998-1 3.2.2.2 für a_g = {a_g:.2} m/s² und S = {:.2}, Dämpfung von {:.0} bis {:.0} %, Spitze {peak:.2} m/s²", site.s, SURFACE_DAMPING_PERCENT[0], SURFACE_DAMPING_PERCENT[SURFACE_DAMPING_PERCENT.len() - 1]));
    let anchor = points.first().cloned().unwrap_or_else(|| origin.clone());
    out.push(spec, &points);
    let reference_row = SURFACE_DAMPING_PERCENT.iter().position(|xi| (*xi - 5.0).abs() < 1e-9).unwrap_or(0);
    let t1 = r.t1.clamp(0.0, SURFACE_T_MAX);
    let s_e = part_1::elastic_response_spectrum(a_g, site.s, site.tb, site.tc, site.td, t1);
    let at = v3(origin.x + t1 * SURFACE_X_PER_SECOND, origin.y + reference_row as f64 * SURFACE_Y_PER_ROW, origin.z + s_e * SURFACE_Z_PER_MS2);
    let mut marker = Spec::new(EntityKind::Solid, format!("{id}:t1"), path, box_solid(v3(at.x - 0.2, at.y - 0.2, at.z), v3(0.4, 0.4, 0.4)), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("T1 = {t1:.2} s on the 5 % curve: S_e = {s_e:.2} m/s²"), &format!("T1 = {t1:.2} s auf der 5-%-Kurve: S_e = {s_e:.2} m/s²")), LAYER_SPECTRUM_SURFACE);
    marker.binding = binding_exact(input.report, &format!("en1998.1.{}.{}.baseShear", b.id, r.systems.first().map_or("", |s| s.id.as_str())));
    out.push(marker, &[at.clone()]);
    let text = format!("T1 = {t1:.2} s · S_e(5 %) = {s_e:.2} m/s²");
    out.annotate(Annotation::Label { id: format!("{id}:t1-label"), anchor: v3(at.x, at.y, at.z + 0.6), text: loc(&text, &text), tone: Tone::Info }, &at);
    out.annotate(Annotation::Dimension { id: format!("{id}:period"), from: anchor.clone(), to: v3(origin.x + SURFACE_T_MAX * SURFACE_X_PER_SECOND, anchor.y, origin.z), offset: v3(0.0, -0.8, 0.0), value: Quantity::new(QuantityKind::Time, SURFACE_T_MAX), binding: None }, &anchor);
    out.annotate(Annotation::Label { id: format!("{id}:damping"), anchor: v3(origin.x - 0.5, origin.y + (SURFACE_DAMPING_PERCENT.len() as f64 - 1.0) * SURFACE_Y_PER_ROW, origin.z), text: loc(&format!("damping ξ from {:.0} to {:.0} %", SURFACE_DAMPING_PERCENT[0], SURFACE_DAMPING_PERCENT[SURFACE_DAMPING_PERCENT.len() - 1]), &format!("Dämpfung ξ von {:.0} bis {:.0} %", SURFACE_DAMPING_PERCENT[0], SURFACE_DAMPING_PERCENT[SURFACE_DAMPING_PERCENT.len() - 1])), tone: Tone::Neutral }, &anchor);
    if !out.legends.iter().any(|l| l.id == "spectrum-ordinate") {
        out.legends.push(Legend { id: "spectrum-ordinate".into(), title: loc("Spectral acceleration S_e(T, ξ)", "Spektrale Beschleunigung S_e(T, ξ)"), scale: LegendScale::Scalar { field_id: "spectrum-ordinate".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: peak.max(0.1) }, unit: QuantityKind::Acceleration } });
    }
}

/// 📐 The base widths of the overturning check: the entered values, else the plan extents.
pub fn base_widths(b: &En1998Building) -> (f64, f64) {
    (if b.overturning_base_x_m > 0.0 { b.overturning_base_x_m } else { b.plan_width_m }, if b.overturning_base_y_m > 0.0 { b.overturning_base_y_m } else { b.plan_length_m })
}

fn overturning_base(input: &SceneInput, handles: &Index, b: &En1998Building, o: &Vec3, out: &mut Out) {
    if b.plan_width_m <= 0.0 || b.plan_length_m <= 0.0 || b.storeys.is_empty() {
        return;
    }
    let (bx, by) = base_widths(b);
    let path = path_of("buildings", &b.id);
    let id = format!("{}:base", b.id);
    let tail = ".overturning";
    let governing = input.report.checks.iter().filter(|c| c.subject.entity_id == b.id && c.id.ends_with(tail)).max_by(|a, c| a.utilization.total_cmp(&c.utilization));
    let binding = governing.map(projection_kit::binding_of_check);
    let origin = v3(o.x + (b.plan_width_m - bx) / 2.0, o.y + (b.plan_length_m - by) / 2.0, o.z - BASE_THICKNESS);
    let mut spec = Spec::new(EntityKind::Plate, id.clone(), path.clone(), Geometry::Plate(PlateGeometry { frame: world_frame(origin.clone()), outline: rect_ring(bx.max(0.1), by.max(0.1)), thickness: BASE_THICKNESS, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Support, loc(&format!("Base of {} for the overturning check, {bx:.1} m by {by:.1} m", b.name), &format!("Fundament von {} für den Kippnachweis, {bx:.1} m mal {by:.1} m", b.name)), LAYER_OVERTURNING);
    spec.binding = binding.clone();
    spec.opacity = 0.6;
    spec.handles = handles.of(&id);
    out.push(spec, &[origin.clone(), v3(origin.x + bx, origin.y + by, o.z)]);
    let centre = v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m / 2.0, o.z);
    if let Some(check) = governing {
        let along_y = b.systems.iter().find(|s| check.id.ends_with(&format!(".{}{tail}", s.id))).is_some_and(|s| s.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y);
        let m_ov = check.computed.value;
        let m_stab = check.limit.value;
        out.push(
            Spec::new(EntityKind::Load, format!("{id}:moment"), path.clone(), Geometry::Load(LoadGlyph::Moment { at: centre.clone(), axis: if along_y { Vec3::X } else { Vec3::Y }, magnitude: Quantity::new(QuantityKind::Moment, m_ov), case: load_case("seismic-overturning") }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Overturning moment M_ov = {:.0} kN·m against M_stab = {:.0} kN·m", m_ov / 1000.0, m_stab / 1000.0), &format!("Kippmoment M_ov = {:.0} kN·m gegen M_stab = {:.0} kN·m", m_ov / 1000.0, m_stab / 1000.0)), LAYER_OVERTURNING),
            &[centre.clone()],
        );
        let text = format!("M_ov / M_stab = {:.2} (γ = {:.2})", check.utilization, norm::ndp_number(STABILISING_FACTOR, if input.document.annex == semio_s_plugin_norm_compliance::AnnexChoice::En { norm::AnnexChoice::En } else { norm::AnnexChoice::De }));
        let at = v3(centre.x, centre.y, o.z + 0.8);
        out.annotate(Annotation::Label { id: format!("{id}:label"), anchor: at.clone(), text: loc(&text, &text), tone: tone_of(&binding) }, &at);
    }
    for (tag, from, to, field, value) in [("bx", v3(origin.x, origin.y - 1.0, o.z), v3(origin.x + bx, origin.y - 1.0, o.z), "overturningBaseXM", bx), ("by", v3(origin.x - 1.0, origin.y, o.z), v3(origin.x - 1.0, origin.y + by, o.z), "overturningBaseYM", by)] {
        out.annotate(Annotation::Dimension { id: format!("{id}:{tag}"), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, value), binding: Some(FieldBinding::new(format!("{path}.{field}"), Affine::IDENTITY, "m")) }, &from);
    }
}

/// 🎼 Start of the trace of the `slot`-th accelerogram of a building: south of the plan, 3 m apart per record.
pub fn trace_base(building_origin: &Vec3, slot: usize) -> Vec3 {
    v3(building_origin.x, building_origin.y - 8.0 - 3.0 * slot as f64, building_origin.z + 2.0)
}

fn accelerogram_trace(input: &SceneInput, layout: &Layout, handles: &Index, slot: usize, rec: &En1998Accelerogram, out: &mut Out) {
    let o = layout.origin(&rec.building_id);
    let path = path_of("accelerograms", &rec.id);
    let base = trace_base(&o, slot);
    let binding = binding_exact(input.report, &format!("en1998.1.{}.timeHistory.record.{}", rec.building_id, rec.id));
    let samples = time_history::parse_samples(&rec.samples).filter(|s| s.len() >= 2 && rec.dt_s > 0.0);
    let Some(samples) = samples else {
        let mut marker = Spec::new(EntityKind::Solid, rec.id.clone(), path, box_solid(base.clone(), v3(0.5, 0.5, 0.5)), MaterialRole::Neutral, loc(&format!("Accelerogram {} (no usable samples)", rec.label), &format!("Akzelerogramm {} (keine verwendbaren Werte)", rec.label)), LAYER_ACCELEROGRAMS);
        marker.binding = binding;
        marker.style = Some(StyleState::Error);
        out.push(marker, &[base]);
        return;
    };
    let stride = samples.len().div_ceil(TRACE_MAX_COLUMNS).max(1);
    let kept: Vec<(f64, f64)> = samples.iter().enumerate().step_by(stride).map(|(i, a)| (i as f64 * rec.dt_s, a * rec.scale_factor)).collect();
    let mut points = Vec::new();
    let mut values = Vec::new();
    for row in 0..2 {
        for (t, a) in &kept {
            points.push(v3(base.x + t * TRACE_METRES_PER_SECOND, base.y + row as f64 * TRACE_WIDTH, base.z + a * TRACE_METRES_PER_MS2));
            values.push(a.abs());
        }
    }
    let peak = kept.iter().fold(0.0_f64, |p, (_, a)| p.max(a.abs()));
    let mut spec = Spec::new(EntityKind::Surface, rec.id.clone(), path, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: 2, columns: kept.len() as u32, points: points.clone() } } }), MaterialRole::Neutral, loc(&format!("Accelerogram {}: PGA {peak:.2} m/s², {} samples at {:.4} s", rec.label, samples.len(), rec.dt_s), &format!("Akzelerogramm {}: PGA {peak:.2} m/s², {} Werte im Abstand {:.4} s", rec.label, samples.len(), rec.dt_s)), LAYER_ACCELEROGRAMS);
    spec.binding = binding;
    spec.handles = handles.of(&rec.id);
    spec.channel = Some(ColourChannel::Fields { active: "ground-acceleration".into(), fields: vec![ScalarField { id: "ground-acceleration".into(), unit: QuantityKind::Acceleration, samples: FieldSamples::Vertex { values } }] });
    out.push(spec, &points);
    if !out.legends.iter().any(|l| l.id == "ground-acceleration") {
        out.legends.push(Legend { id: "ground-acceleration".into(), title: loc("Ground acceleration |a|", "Bodenbeschleunigung |a|"), scale: LegendScale::Scalar { field_id: "ground-acceleration".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: peak.max(0.1) }, unit: QuantityKind::Acceleration } });
    }
    let at = v3(base.x, base.y, base.z + peak * TRACE_METRES_PER_MS2 + 0.6);
    let text = format!("{} · ×{:.2} · PGA {peak:.2} m/s²", rec.label, rec.scale_factor);
    out.annotate(Annotation::Label { id: format!("{}:label", rec.id), anchor: at.clone(), text: loc(&text, &text), tone: Tone::Neutral }, &at);
}

/// 📈 The diagrams of the horizontal elements of one building: the storey force and shear profile, the diaphragm shear and chord force along the span and the accelerogram set against the elastic spectrum.
pub fn diagrams(d: &super::model::En1998Snapshot, report: &norm::CheckReport, b: &En1998Building, site: &Site) -> Vec<Diagram2d> {
    use super::scene::projection_kit::diagram_parts::{column, stroke};
    let r = respond(site, b);
    let axis = |en: &str, de: &str, q: QuantityKind, unit: &str, domain: Option<(f64, f64)>| super::scene::projection_kit::diagram_parts::axis(en, de, q, unit, AxisScale::Linear, domain);
    let mut out = Vec::new();
    if let Some(system) = r.systems.iter().max_by(|a, c| a.f_b.total_cmp(&c.f_b)).filter(|s| s.f_b > 0.0 && s.forces.len() == r.elevations.len()) {
        let n = system.forces.len();
        let shear: Vec<f64> = (0..n).map(|i| system.forces[i..].iter().sum()).collect();
        let z: Vec<f64> = r.elevations.iter().zip(&r.heights).map(|(e, h)| e - h / 2.0).collect();
        let markers = (0..n).map(|i| {
            let check = format!("en1998.1.{}.{}.storeyForces.{}", b.id, system.id, b.storeys[i].id);
            let utilisation = report.checks.iter().find(|c| c.id == check).map(|c| c.utilization).filter(|u| u.is_finite());
            Marker { id: format!("shear-{}", b.storeys[i].id), at: Vec2::new(shear[i], z[i]), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, |u| super::scene::utilisation_tone(u)), label: loc(&format!("{}: V = {:.0} kN", b.storeys[i].id, shear[i] / 1000.0), &format!("{}: V = {:.0} kN", b.storeys[i].id, shear[i] / 1000.0)), utilisation, bound_check: utilisation.map(|_| check), binding: None }
        }).collect();
        out.push(Diagram2d {
            id: format!("base-shear:{}", b.id),
            title: loc(&format!("Base shear distribution \u{2013} {}", b.name), &format!("Gesamtquerkraftverteilung \u{2013} {}", b.name)),
            description: loc("Storey forces F_i and storey shears V_i = \u{3a3} F_j over the elevation of the storeys (EN 1998-1, 4.3.3.2.3); the 3D scene animates the build-up from the roof down.", "Geschosskr\u{e4}fte F_i und Geschossquerkr\u{e4}fte V_i = \u{3a3} F_j \u{fc}ber der Geschossh\u{f6}he (EN 1998-1, 4.3.3.2.3); die 3D-Szene animiert den Aufbau vom Dach nach unten."),
            axes: vec![axis("Force", "Kraft", QuantityKind::Force, "N", Some((0.0, shear[0].max(1.0)))), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m", None)],
            series: vec![
                Series::Polyline { id: "storey-shear".into(), label: loc("Storey shear V_i", "Geschossquerkraft V_i"), points: shear.iter().zip(&z).map(|(v, h)| Vec2::new(*v, *h)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
                Series::Polyline { id: "storey-force".into(), label: loc("Storey force F_i", "Geschosskraft F_i"), points: system.forces.iter().zip(&z).map(|(f, h)| Vec2::new(*f, *h)).collect(), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
            ],
            markers,
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Group, &b.id, path_of("buildings", &b.id))),
            legend: true,
            table_twin: DataTable { columns: vec![column("Elevation z", "H\u{f6}he z", "m", z.clone()), column("Storey force F_i", "Geschosskraft F_i", "N", system.forces.clone()), column("Storey shear V_i", "Geschossquerkraft V_i", "N", shear)] },
        });
    }
    for dia in d.diaphragms.iter().filter(|x| x.building_id == b.id) {
        let Some(index) = b.storeys.iter().position(|s| s.id == dia.storey_id) else { continue };
        let Some(system) = r.systems.iter().max_by(|a, c| a.f_b.total_cmp(&c.f_b)) else { continue };
        let f_i = system.forces.get(index).copied().unwrap_or(0.0) / system.delta.max(1e-9);
        let (plan_span, plan_depth) = if system.along_y { (b.plan_length_m, b.plan_width_m) } else { (b.plan_width_m, b.plan_length_m) };
        let (span, depth) = diaphragm::extent_m(dia.span_m, dia.depth_m, plan_span, plan_depth);
        if f_i <= 0.0 || span <= 0.0 || depth <= 0.0 {
            continue;
        }
        let f_d = diaphragm::design_force_n(f_i, if dia.gamma_d > 0.0 { dia.gamma_d } else { norm::ndp_number(TRANSFER_FACTOR, if d.annex == semio_s_plugin_norm_compliance::AnnexChoice::En { norm::AnnexChoice::En } else { norm::AnnexChoice::De }) });
        let xs: Vec<f64> = (0..=24).map(|i| span * i as f64 / 24.0).collect();
        let v: Vec<f64> = xs.iter().map(|x| diaphragm::shear_at_n(f_d, span, *x)).collect();
        let n: Vec<f64> = xs.iter().map(|x| diaphragm::chord_force_at_n(f_d, span, depth, *x)).collect();
        let v_rd = diaphragm::shear_resistance_n(dia.shear_strength_mpa, dia.thickness_m, depth, dia.opening_width_m);
        let n_rd = diaphragm::chord_resistance_n(dia.chord_area_m2, dia.chord_strength_mpa);
        let mut series = vec![
            Series::Polyline { id: "shear".into(), label: loc("Shear force V(x)", "Querkraft V(x)"), points: xs.iter().zip(&v).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "chord".into(), label: loc("Chord force N(x)", "Gurtkraft N(x)"), points: xs.iter().zip(&n).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
        ];
        if v_rd > 0.0 {
            series.push(Series::Limit { id: "v-rd".into(), label: loc("V_Rd", "V_Rd"), axis: AxisRef::Y, value: v_rd, style: stroke(Tone::Error, 1.0, Dash::Dotted) });
        }
        if n_rd > 0.0 {
            series.push(Series::Limit { id: "n-rd".into(), label: loc("N_Rd", "N_Rd"), axis: AxisRef::Y, value: n_rd, style: stroke(Tone::Error, 1.0, Dash::Dashed) });
        }
        out.push(Diagram2d {
            id: format!("diaphragm:{}", dia.id),
            title: loc(&format!("Diaphragm forces \u{2013} {}", dia.id), &format!("Scheibenkr\u{e4}fte \u{2013} {}", dia.id)),
            description: loc("Shear force and chord force of the diaphragm along its span under the uniformly distributed force \u{3b3}_d F_i between the two lateral load resisting systems (EN 1998-1, 4.4.2.5) against the in-plane resistances.", "Querkraft und Gurtkraft der Scheibe \u{fc}ber ihre Spannweite unter der gleichm\u{e4}\u{df}ig verteilten Kraft \u{3b3}_d F_i zwischen den beiden aussteifenden Systemen (EN 1998-1, 4.4.2.5) gegen die Widerst\u{e4}nde in der Scheibenebene."),
            axes: vec![axis("Position x along the span", "Position x \u{fc}ber die Spannweite", QuantityKind::Length, "m", Some((0.0, span))), axis("Force", "Kraft", QuantityKind::Force, "N", None)],
            series,
            markers: Vec::new(),
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Plate, &dia.id, path_of("diaphragms", &dia.id))),
            legend: true,
            table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", xs), column("Shear V(x)", "Querkraft V(x)", "N", v), column("Chord N(x)", "Gurtkraft N(x)", "N", n)] },
        });
    }
    let rows: Vec<&En1998Accelerogram> = d.accelerograms.iter().filter(|x| x.building_id == b.id).collect();
    let usable: Vec<(&En1998Accelerogram, Vec<f64>)> = rows.iter().filter_map(|row| time_history::parse_samples(&row.samples).filter(|s| s.len() >= 2 && row.dt_s > 0.0 && row.scale_factor > 0.0).map(|s| (*row, s))).collect();
    out.extend(usable.iter().map(|(row, samples)| accelerogram_history(report, b, row, samples)));
    if !usable.is_empty() && site.a_gr > 0.0 {
        let a_g = site.gamma_i * site.a_gr;
        let periods = time_history::periods_s(r.t1.max(1e-3), time_history::PERIOD_SAMPLES);
        let elastic: Vec<f64> = periods.iter().map(|t| part_1::elastic_response_spectrum(a_g, site.s, site.tb, site.tc, site.td, *t)).collect();
        let spectra_of: Vec<Vec<f64>> = usable.iter().map(|(row, s)| time_history::record_spectrum(s, row.scale_factor, row.dt_s, &periods)).collect();
        let mean: Vec<f64> = (0..periods.len()).map(|p| spectra_of.iter().map(|s| s[p]).sum::<f64>() / spectra_of.len() as f64).collect();
        let line = |id: &str, en: &str, de: &str, values: &[f64], tone: Tone, dash: Dash| Series::Polyline { id: id.into(), label: loc(en, de), points: periods.iter().zip(values).map(|(t, y)| Vec2::new(*t, *y)).collect(), style: stroke(tone, 2.0, dash) };
        let floor: Vec<f64> = elastic.iter().map(|e| 0.9 * e).collect();
        let mut series = vec![line("elastic", "Elastic spectrum S_e(T)", "Elastisches Spektrum S_e(T)", &elastic, Tone::Info, Dash::Solid), line("floor", "90 % of S_e(T)", "90 % von S_e(T)", &floor, Tone::Error, Dash::Dashed), line("mean", "Mean of the records", "Mittel der Aufzeichnungen", &mean, Tone::Warning, Dash::Solid)];
        for ((row, _), values) in usable.iter().zip(&spectra_of) {
            series.push(line(&format!("record-{}", row.id), &format!("Record {}", row.label), &format!("Aufzeichnung {}", row.label), values, Tone::Neutral, Dash::Dotted));
        }
        let check = format!("en1998.1.{}.timeHistory.spectrum", b.id);
        let utilisation = report.checks.iter().find(|c| c.id == check).map(|c| c.utilization).filter(|u| u.is_finite());
        out.push(Diagram2d {
            id: format!("time-history:{}", b.id),
            title: loc(&format!("Accelerogram set against the elastic spectrum \u{2013} {}", b.name), &format!("Akzelerogrammsatz gegen das elastische Spektrum \u{2013} {}", b.name)),
            description: loc("5 % damped spectra of the scaled records, their mean and the 90 % floor of the elastic spectrum between 0.2 T1 and 2 T1 (EN 1998-1, 3.2.3.1.2).", "5-%-ged\u{e4}mpfte Spektren der skalierten Aufzeichnungen, ihr Mittel und die 90-%-Grenze des elastischen Spektrums zwischen 0,2 T1 und 2 T1 (EN 1998-1, 3.2.3.1.2)."),
            axes: vec![axis("Period T", "Periode T", QuantityKind::Time, "s", Some((periods[0], periods[periods.len() - 1]))), axis("Spectral acceleration", "Spektrale Beschleunigung", QuantityKind::Acceleration, "m/s²", None)],
            series,
            markers: vec![Marker { id: "set".into(), at: Vec2::new(periods[0], mean[0]), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, |u| if super::scene::norm::within_limit(u) { Tone::Info } else { Tone::Error }), label: loc("Set acceptance", "Satzabnahme"), utilisation, bound_check: utilisation.map(|_| check), binding: None }],
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Group, &b.id, path_of("buildings", &b.id))),
            legend: true,
            table_twin: DataTable { columns: vec![column("Period T", "Periode T", "s", periods.clone()), column("S_e(T)", "S_e(T)", "m/s²", elastic), column("Mean S_a(T)", "Mittel S_a(T)", "m/s²", mean)] },
        });
    }
    out
}

const HISTORY_MAX_POINTS: usize = 400;

/// 📈 Diagram `accelerogram:<id>`: the scaled ground acceleration of one record over time with its peak marked; the marker is draggable onto `scaleFactor` (EN 1998-1, 3.2.3.1.2).
fn accelerogram_history(report: &norm::CheckReport, b: &En1998Building, row: &En1998Accelerogram, samples: &[f64]) -> Diagram2d {
    use super::scene::projection_kit::diagram_parts::{self, column, stroke};
    let stride = samples.len().div_ceil(HISTORY_MAX_POINTS).max(1);
    let kept: Vec<(f64, f64)> = samples.iter().enumerate().step_by(stride).map(|(i, a)| (i as f64 * row.dt_s, a * row.scale_factor)).collect();
    let (peak_index, raw_peak) = samples.iter().enumerate().fold((0, 0.0_f64), |best, (i, a)| if a.abs() > best.1.abs() { (i, *a) } else { best });
    let peak = raw_peak * row.scale_factor;
    let check = format!("en1998.1.{}.timeHistory.record.{}", b.id, row.id);
    let utilisation = report.checks.iter().find(|c| c.id == check).map(|c| c.utilization).filter(|u| u.is_finite());
    let binding = (raw_peak != 0.0).then(|| {
        let mut binding = FieldBinding::new(format!("{}.scaleFactor", path_of("accelerograms", &row.id)), Affine { base: 0.0, k: 1.0 / raw_peak }, "1");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        binding
    });
    let marker = Marker { id: "peak".into(), at: Vec2::new(peak_index as f64 * row.dt_s, peak), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, |u| super::scene::utilisation_tone(u)), label: loc(&format!("PGA {:.2} m/s\u{b2} (scale {:.2})", peak.abs(), row.scale_factor), &format!("PGA {:.2} m/s\u{b2} (Faktor {:.2})", peak.abs(), row.scale_factor)), utilisation, bound_check: utilisation.map(|_| check), binding };
    Diagram2d {
        id: format!("accelerogram:{}", row.id),
        title: loc(&format!("Accelerogram {} \u{2013} {}", row.label, b.name), &format!("Akzelerogramm {} \u{2013} {}", row.label, b.name)),
        description: loc("Ground acceleration of the record scaled by its factor over time; drag the peak marker to change the scale factor (EN 1998-1, 3.2.3.1.2).", "Bodenbeschleunigung der mit ihrem Faktor skalierten Aufzeichnung \u{fc}ber der Zeit; die Spitzenmarke ziehen \u{e4}ndert den Skalierungsfaktor (EN 1998-1, 3.2.3.1.2)."),
        axes: vec![diagram_parts::axis("Time t", "Zeit t", QuantityKind::Time, "s", AxisScale::Linear, None), diagram_parts::axis("Acceleration", "Beschleunigung", QuantityKind::Acceleration, "m/s²", AxisScale::Linear, None)],
        series: vec![Series::Polyline { id: "history".into(), label: loc(&format!("Record {}", row.label), &format!("Aufzeichnung {}", row.label)), points: kept.iter().map(|(t, a)| Vec2::new(*t, *a)).collect(), style: stroke(Tone::Info, 1.5, Dash::Solid) }],
        markers: vec![marker],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Surface, &row.id, path_of("accelerograms", &row.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Time t", "Zeit t", "s", kept.iter().map(|k| k.0).collect()), column("Acceleration", "Beschleunigung", "m/s²", kept.iter().map(|k| k.1).collect())] },
    }
}
