//! 🧱️ Collections: the construction stage timeline and the roof surfaces over the building; the bridge lanes with their LM1 patches stand on the deck in the traffic module.

use super::kit::*;
use super::model::En1991Snapshot;
use super::scene::*;
use super::checks;
use super::checks::part_1_3;
use super::{handles, Ctx};

const STAGE_GAP: f64 = 6.0;
const STAGE_DEPTH: f64 = 2.0;
const STAGE_Z: f64 = 0.05;
const STAGE_MIN_LENGTH: f64 = 10.0;
const SURFACE_LIFT: f64 = 0.05;

/// ⏱️ Start and end along x of every stage on the timeline strip: the stages follow one another in list order, scaled to the length of the strip.
pub fn stage_slots(d: &En1991Snapshot) -> Vec<(f64, f64)> {
    let scale = 1.0 / days_per_metre(d);
    let mut days = 0.0;
    d.construction_stages
        .iter()
        .map(|s| {
            let start = days * scale;
            days += s.duration_days.max(0.0);
            (start, days * scale)
        })
        .collect()
}

/// ⏱️ Days one metre of the timeline strip stands for.
pub fn days_per_metre(d: &En1991Snapshot) -> f64 {
    let total: f64 = d.construction_stages.iter().map(|s| s.duration_days.max(0.0)).sum();
    (total / d.depth.max(STAGE_MIN_LENGTH)).max(1e-6)
}

/// 📍️ Along-x start of the timeline strip north of the building.
pub fn stage_row_y(d: &En1991Snapshot) -> f64 {
    d.width + STAGE_GAP
}

/// 🏠 The span along x of roof surface `index`: it follows the earlier surfaces of the same face.
pub fn surface_span(d: &En1991Snapshot, index: usize) -> (f64, f64) {
    let surface = &d.roof_surfaces[index];
    let before: f64 = d.roof_surfaces[..index].iter().filter(|o| o.face == surface.face).map(|o| o.extent.max(0.0)).sum();
    (before, before + surface.extent.max(0.0))
}

/// 🏠 Height of the roof surface `index` at `x`: flat for flat roofs, else rising with its pitch from the eaves of the building.
pub fn surface_z(d: &En1991Snapshot, index: usize, x: f64) -> f64 {
    let surface = &d.roof_surfaces[index];
    let tan = if surface.shape.eq_ignore_ascii_case("flat") { 0.0 } else { surface.pitch_deg.clamp(0.0, 60.0).to_radians().tan() };
    d.height + SURFACE_LIFT + tan * (x - surface_span(d, index).0)
}

pub fn stages(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    let y0 = stage_row_y(d);
    for (stage, (x0, x1)) in d.construction_stages.iter().zip(stage_slots(d)) {
        if !out.own(&stage.id) {
            continue;
        }
        let points: Vec<Vec3> = rect2(x0, y0, x1.max(x0 + 0.05), y0 + STAGE_DEPTH).iter().map(|p| pose.at(p[0], p[1], STAGE_Z)).collect();
        let geometry = LoadGlyph::Area { polygon: polygon3(&points), direction: mul(&Vec3::Z, -1.0), intensity: pressure(stage.assumed_qk), case: case(&stage.id, LoadKind::Variable) };
        let check = format!("en1991.1-6.stage-return-period.{}", stage.id);
        out.push(
            Spec::new(EntityKind::Load, stage.id.clone(), "constructionStages", Geometry::Load(geometry), load(LoadKind::Variable), bi(|de| format!("{} {}", if de { "Bauzustand" } else { "Stage" }, stage.id)), LAYER_STAGES)
                .bound(binding_where(report, |c| c.id == check))
                .opacity(0.8)
                .detail(bi(|de| format!("{} d, T = {} {}, q = {} kN/m²", num(stage.duration_days, 0, de), num(stage.return_years, 1, de), if de { "Jahre" } else { "years" }, num(stage.assumed_qk / 1000.0, 2, de))))
                .handles(handles::ids(d, layout, &stage.id)),
            &points,
        );
        out.label(format!("{}:label", stage.id), pose.at((x0 + x1) / 2.0, y0 + STAGE_DEPTH / 2.0, STAGE_Z), bi(|de| format!("{} d", num(stage.duration_days, 0, de))), Tone::Neutral);
    }
}

pub fn surfaces(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for (index, surface) in d.roof_surfaces.iter().enumerate() {
        if !out.own(&surface.id) {
            continue;
        }
        let (x0, x1) = surface_span(d, index);
        let points: Vec<Vec3> = [(x0, 0.0), (x1, 0.0), (x1, d.width), (x0, d.width)].iter().map(|(x, y)| pose.at(*x, *y, surface_z(d, index, *x))).collect();
        let check = format!("en1991.1-4.roof-surface-cpe.{}", surface.id);
        out.push(
            Spec::new(EntityKind::Surface, surface.id.clone(), "roofSurfaces", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&points) } }), MaterialRole::Neutral, bi(|de| format!("{} {}", if de { "Dachfläche" } else { "Roof surface" }, surface.id)), LAYER_ROOF_SURFACES)
                .bound(binding_where(report, |c| c.id == check))
                .opacity(0.6)
                .detail(bi(|de| format!("{}, {} {}°, {} {} m", surface.shape, if de { "Neigung" } else { "pitch" }, num(surface.pitch_deg, 1, de), if de { "Ausdehnung" } else { "extent" }, num(surface.extent, 2, de))))
                .handles(handles::ids(d, layout, &surface.id)),
            &points,
        );
        let step = d.roof_surfaces[..index].iter().rev().find(|o| o.face == surface.face).map(|o| part_1_3::roof_step_height_m(&o.shape.to_ascii_lowercase(), o.pitch_deg, o.extent)).filter(|h| *h > 1e-6);
        let sk = part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk);
        if let (Some(h), Some(sk), Some(prev)) = (step, sk, d.roof_surfaces[..index].iter().rev().find(|o| o.face == surface.face)) {
            let mu = (part_1_3::sliding_mu_s(prev.pitch_deg) + part_1_3::drift_mu_w(h, prev.extent, surface.extent, sk, d.annex)).max(part_1_3::mu_1(surface.pitch_deg));
            let end = (x0 + (2.0 * h).clamp(5.0, 15.0)).min(x1);
            let wedge: Vec<Vec3> = [(x0, 0.0), (end, 0.0), (end, d.width), (x0, d.width)].iter().map(|(x, y)| pose.at(*x, *y, surface_z(d, index, *x) + SURFACE_LIFT)).collect();
            let (id, check) = (format!("{}:drift", surface.id), format!("en1991.1-3.roof-step-drift.{}", surface.id));
            out.push(Spec::new(EntityKind::Load, id.clone(), "roofSurfaces", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&wedge), direction: mul(&Vec3::Z, -1.0), intensity: pressure(mu * sk), case: case(&id, LoadKind::Variable) }), load(LoadKind::Variable), loc("Snow drift at the roof step", "Schneeverwehung am Dachsprung"), LAYER_SNOW).bound(binding_where(report, |c| c.id == check)).channel(ColourChannel::Role).opacity(0.7).handles(handles::ids(d, layout, &surface.id)), &wedge);
        }
    }
}

fn surface(out: &mut Out, id: &str, collection: &str, points: &[Vec3], name: LocalizedCopy, detail: LocalizedCopy, layer: &'static str, bound: Option<UtilisationBinding>, grips: Vec<String>) {
    out.push(
        Spec::new(EntityKind::Surface, id.to_string(), collection, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(points) } }), load(LoadKind::Variable), name, layer)
            .bound(bound)
            .channel(ColourChannel::Role)
            .opacity(0.55)
            .detail(detail)
            .handles(grips),
        points,
    );
}

/// 🚶 Footbridges: the deck of the loaded length L and the width b beside the bridge with the uniform load q_fk on it, bound to the load check of the row (EN 1991-2, 5.3.2.1).
pub fn footbridges(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    for (index, row) in d.footbridges.iter().enumerate() {
        if !out.own(&row.id) || !(row.loaded_length > 0.0 && row.deck_width > 0.0) {
            continue;
        }
        let y = gallery_y(d, "footbridges", index);
        let points: Vec<Vec3> = rect2(0.0, y, row.loaded_length, y + row.deck_width).iter().map(|p| layout.bridge.at(p[0], p[1], 0.0)).collect();
        let q = checks::part_2::footbridge_q_fk_pa(row.loaded_length);
        let bound = binding_where(report, |c| c.id == format!("en1991.2.footbridge-load.{}", row.id));
        let grips = handles::ids(d, layout, &row.id);
        let name = bi(|de| format!("{} {}", if de { "Fußgängerbrücke" } else { "Footbridge" }, row.id));
        let detail = bi(|de| format!("L = {} m, b = {} m, q_fk = {} kN/m²", num(row.loaded_length, 1, de), num(row.deck_width, 2, de), num(q / 1000.0, 2, de)));
        surface(out, &row.id, "footbridges", &points, name.clone(), detail.clone(), LAYER_FOOTBRIDGES, bound.clone(), grips.clone());
        out.push(
            Spec::new(EntityKind::Load, format!("{}:load", row.id), "footbridges", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: mul(&Vec3::Z, -1.0), intensity: pressure(q), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_FOOTBRIDGES)
                .bound(bound)
                .channel(ColourChannel::Role)
                .opacity(0.8)
                .detail(detail)
                .handles(grips),
            &points,
        );
    }
}

/// 🚆 Rail spans: a track strip of the determinant length L_Φ beside the bridge, labelled with the dynamic factor Φ of the row (EN 1991-2, 6.4.5.2).
pub fn rail_dynamics(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    for (index, row) in d.rail_dynamics.iter().enumerate() {
        if !out.own(&row.id) || !(row.determinant_length > 0.0) {
            continue;
        }
        let y = gallery_y(d, "railDynamics", index);
        let points: Vec<Vec3> = rect2(0.0, y, row.determinant_length, y + TRACK_WIDTH).iter().map(|p| layout.bridge.at(p[0], p[1], 0.0)).collect();
        let phi = checks::part_2::rail_phi_for(&row.maintenance, row.determinant_length);
        let bound = binding_where(report, |c| c.id == format!("en1991.2.rail-dynamic.{}", row.id));
        let name = bi(|de| format!("{} {}", if de { "Eisenbahnfeld" } else { "Rail span" }, row.id));
        let detail = bi(|de| format!("L_Φ = {} m, {}, Φ = {}", num(row.determinant_length, 1, de), row.maintenance, phi.map_or("?".to_string(), |p| num(p, 3, de))));
        surface(out, &row.id, "railDynamics", &points, name, detail, LAYER_RAIL_DYNAMICS, bound.clone(), handles::ids(d, layout, &row.id));
        out.label(format!("{}:label", row.id), points[2].clone(), bi(|de| format!("Φ = {}", phi.map_or("?".to_string(), |p| num(p, 3, de)))), tone_of(&bound));
    }
}

/// 📐 Wheel load dispersals: the contact rectangle and the dispersed footprint at the slab mid-plane around it (EN 1991-2, 4.3.6).
pub fn dispersals(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    for (index, row) in d.load_dispersals.iter().enumerate() {
        if !out.own(&row.id) || !(row.wheel_length > 0.0 && row.wheel_width > 0.0) {
            continue;
        }
        let y = gallery_y(d, "loadDispersals", index);
        let (a, b) = (checks::part_2::dispersed_side_m(row.wheel_length, row.pavement_thickness, row.slab_thickness), checks::part_2::dispersed_side_m(row.wheel_width, row.pavement_thickness, row.slab_thickness));
        let (cx, cy) = (a / 2.0, y + b / 2.0);
        let ring: Vec<Vec3> = rect2(0.0, y, a, y + b).iter().map(|p| layout.bridge.at(p[0], p[1], 0.0)).collect();
        let contact: Vec<Vec3> = rect2(cx - row.wheel_length / 2.0, cy - row.wheel_width / 2.0, cx + row.wheel_length / 2.0, cy + row.wheel_width / 2.0).iter().map(|p| layout.bridge.at(p[0], p[1], SURFACE_LIFT)).collect();
        let bound = binding_where(report, |c| c.id == format!("en1991.2.dispersal-footprint.{}", row.id));
        let grips = handles::ids(d, layout, &row.id);
        let name = bi(|de| format!("{} {}", if de { "Lastausbreitung" } else { "Load dispersal" }, row.id));
        let detail = bi(|de| format!("{} m × {} m → {} m × {} m", num(row.wheel_length, 2, de), num(row.wheel_width, 2, de), num(a, 3, de), num(b, 3, de)));
        surface(out, &row.id, "loadDispersals", &ring, name.clone(), detail.clone(), LAYER_DISPERSALS, bound.clone(), grips.clone());
        surface(out, &format!("{}:contact", row.id), "loadDispersals", &contact, name, detail, LAYER_DISPERSALS, bound, grips);
    }
}

/// 🌾 Bulk solids: a plate beside the silo labelled with the mean lateral pressure ratio K_m of the row (EN 1991-4, Annex E).
pub fn bulk_solids(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    for (index, row) in d.bulk_solids.iter().enumerate() {
        if !out.own(&row.id) {
            continue;
        }
        let y = gallery_y(d, "bulkSolids", index);
        let points: Vec<Vec3> = rect2(0.0, y, PLATE_SIDE, y + PLATE_SIDE).iter().map(|p| layout.silo.at(p[0], p[1], 0.0)).collect();
        let k_mean = checks::part_4::k_mean(row.internal_friction);
        let bound = binding_where(report, |c| c.id == format!("en1991.4.bulk-k-mean.{}", row.id));
        let name = bi(|de| format!("{} {}", if de { "Schüttgut" } else { "Bulk solid" }, row.id));
        let detail = bi(|de| format!("φ_i = {}°, K_m = {}", num(row.internal_friction.to_degrees(), 1, de), num(k_mean, 3, de)));
        surface(out, &row.id, "bulkSolids", &points, name, detail, LAYER_BULK_SOLIDS, bound.clone(), handles::ids(d, layout, &row.id));
        out.label(format!("{}:label", row.id), points[2].clone(), bi(|de| format!("K_m = {}", num(k_mean, 3, de))), tone_of(&bound));
    }
}

/// ⚙️ Rotating machines: a base plate beside the silo with the equivalent static force F_dyn = m e ω² of the unbalance acting on it (EN 1991-3, 5).
pub fn machines(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    for (index, row) in d.rotating_machines.iter().enumerate() {
        if !out.own(&row.id) {
            continue;
        }
        let y = gallery_y(d, "rotatingMachines", index);
        let points: Vec<Vec3> = rect2(PLATE_SIDE * 1.5, y, PLATE_SIDE * 2.5, y + PLATE_SIDE).iter().map(|p| layout.silo.at(p[0], p[1], 0.0)).collect();
        let n = checks::part_3::machine_dynamic_force_n(row.mass, row.eccentricity, row.frequency);
        let bound = binding_where(report, |c| c.id == format!("en1991.3.machine-force.{}", row.id));
        let grips = handles::ids(d, layout, &row.id);
        let name = bi(|de| format!("{} {}", if de { "Maschine" } else { "Machine" }, row.id));
        let detail = bi(|de| format!("m = {} kg, e = {} m, f = {} Hz, F_dyn = {} kN", num(row.mass, 0, de), num(row.eccentricity, 3, de), num(row.frequency, 2, de), num(n / 1000.0, 2, de)));
        surface(out, &row.id, "rotatingMachines", &points, name.clone(), detail.clone(), LAYER_MACHINES, bound.clone(), grips.clone());
        let at = layout.silo.at(PLATE_SIDE * 2.0, y + PLATE_SIDE / 2.0, 1.5);
        out.push(
            Spec::new(EntityKind::Load, format!("{}:load", row.id), "rotatingMachines", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: force(n), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_MACHINES)
                .bound(bound)
                .channel(ColourChannel::Role)
                .detail(detail)
                .handles(grips),
            &[at],
        );
    }
}
