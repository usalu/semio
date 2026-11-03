//! 🌪️ Wave 3 collections in the scene: LM3 special vehicles on the deck, the wind pressure map of bridge decks, slender elements with the pressure profile of c_s c_d and the vortex shedding label, impact
//! patches, internal gas explosion rooms with their wall pressures and vent, and the temperature bands of bridge sections.

use super::kit::*;
use super::layout::{deck_width, DECK_Z, FOOTWAY, GAP};
use super::model::{DynamicElement, En1991Snapshot, ImpactAction, InternalExplosion, WindBridge};
use super::scene::*;
use super::norm;
use super::wind::qp_at;
use super::{checks, handles, Ctx};
use checks::{part_1_4, part_1_5, part_1_7, part_2};

const LIFT: f64 = 0.02;
const FIELD_COLUMNS: usize = 8;
const FIELD_BANDS: usize = 4;
const PROFILE_BANDS: usize = 6;
const TEMPERATURE_BANDS: usize = 6;
const ROW_GAP: f64 = 6.0;
const ROOM_HEIGHT: f64 = 3.0;
const ROOM_DEPTH: f64 = 6.0;
const PATCH_WIDTH: f64 = 1.5;
const PATCH_HEIGHT: f64 = 0.5;
const SLICE_X: f64 = -2.0;
const SLICE_THICKNESS: f64 = 0.3;

/// 🚛 Footprint of a special vehicle on the deck: length along the span, width across and the number of axle lines.
pub fn vehicle_footprint(designation: &str) -> Option<(f64, f64, usize)> {
    let (_, _, lines) = part_2::special_vehicle(designation)?;
    Some((part_2::special_vehicle_length_m(lines), part_2::lm3_vehicle_width_m(), lines))
}

/// 🌉 The vertical extent (z0, z1) of the loaded side face of a bridge wind case in the deck frame and its depth d_tot.
pub fn wind_face(row: &WindBridge) -> Option<(f64, f64, f64)> {
    let depth_tot = part_1_4::bridge_depth_m(row.deck_depth, row.parapet_kind, row.parapet_height, row.traffic_kind)?;
    let z0 = DECK_Z - row.deck_depth;
    Some((z0, z0 + depth_tot, depth_tot))
}

/// 〰️ Plan position (x, y) of slender element `index` in the building frame: the elements stand in a row east of the silo.
pub fn element_origin(d: &En1991Snapshot, index: usize) -> (f64, f64) {
    let before: f64 = d.dynamic_elements[..index].iter().map(|e| e.width.max(0.5) + ROW_GAP).sum();
    (d.depth + 2.0 * GAP + before, d.width / 2.0)
}

/// 🏠 Plan size (length along x, depth along y) of an explosion room: fixed storey height and depth, so the volume is linear in the length.
pub fn room_size(row: &InternalExplosion) -> (f64, f64) {
    (row.volume.max(1.0) / (ROOM_HEIGHT * ROOM_DEPTH), ROOM_DEPTH)
}

/// 🏠 Plan origin (x, y) of explosion room `index` in the building frame: the rooms stand in a row north of the slender elements.
pub fn room_origin(d: &En1991Snapshot, index: usize) -> (f64, f64) {
    let before: f64 = d.internal_explosions[..index].iter().map(|e| room_size(e).0 + ROW_GAP).sum();
    (d.depth + 2.0 * GAP + before, d.width / 2.0 + 25.0)
}

/// 🌡️ Position along the span of the slice of thermal section `index`: the slices stand in a row before the first abutment.
pub fn section_x(index: usize) -> f64 {
    SLICE_X - index as f64 * (SLICE_THICKNESS + 1.0)
}

/// 〰️ Required c_s c_d, critical velocity and mean velocity at the top of a slender element, `None` when the wind inputs are not defined.
pub fn dynamic_values(d: &En1991Snapshot, row: &DynamicElement) -> Option<(f64, f64, f64)> {
    let vb = part_1_4::design_vb(d.annex, d.wind_zone, d.en_vb)?;
    let terrain = part_1_4::TerrainCategory::from_u8(d.terrain_category)?;
    if !(row.height > 0.0 && row.width > 0.0 && row.log_decrement > 0.0 && row.strouhal > 0.0) {
        return None;
    }
    let n1 = if row.first_frequency > 0.0 { row.first_frequency } else { 46.0 / row.height };
    let required = if part_1_4::cscd_simplified(row.height) { 1.0 } else { part_1_4::structural_factor(row.height, row.width, terrain, vb, row.log_decrement, n1)[8] };
    Some((required, part_1_4::vortex_critical_velocity(row.width, n1, row.strouhal), part_1_4::mean_wind_velocity(row.height, terrain, vb, d.orography_factor)))
}

fn area(out: &mut Out, report: &norm::CheckReport, id: String, collection: &str, points: &[Vec3], direction: Vec3, intensity: f64, name: LocalizedCopy, check: &str, layer: &'static str, grips: &[String]) {
    out.push(
        Spec::new(EntityKind::Load, id.clone(), collection, Geometry::Load(LoadGlyph::Area { polygon: polygon3(points), direction, intensity: pressure(intensity), case: case(&id, LoadKind::Variable) }), load(LoadKind::Variable), name, layer).bound(binding_where(report, |c| c.id == check)).channel(ColourChannel::Role).opacity(0.6).detail(bi(|de| format!("q = {} kN/m²", num(intensity / 1000.0, 2, de)))).handles(grips.to_vec()),
        points,
    );
}

/// 🚛 LM3 special vehicles: the footprint on the deck and one axle line load per axle line, bound to the vehicle check.
pub fn vehicles(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.bridge;
    for vehicle in &d.special_vehicles {
        if !out.own(&vehicle.id) { continue; }
        let Some((total, axle, lines)) = part_2::special_vehicle(&vehicle.designation) else { continue };
        let Some((length, width, _)) = vehicle_footprint(&vehicle.designation) else { continue };
        let (check, grips, centre) = (format!("en1991.2.lm3-vehicle.{}", vehicle.id), handles::ids(d, layout, &vehicle.id), FOOTWAY + vehicle.y);
        let points: Vec<Vec3> = rect2(vehicle.x - length / 2.0, centre - width / 2.0, vehicle.x + length / 2.0, centre + width / 2.0).iter().map(|p| pose.at(p[0], p[1], DECK_Z + LIFT)).collect();
        out.push(
            Spec::new(EntityKind::Surface, vehicle.id.clone(), "specialVehicles", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&points) } }), MaterialRole::Neutral, bi(|de| format!("{} {}", if de { "Sonderfahrzeug" } else { "Special vehicle" }, vehicle.designation)), LAYER_TRAFFIC_ALT).bound(binding_where(report, |c| c.id == check)).opacity(0.4).detail(bi(|de| format!("{} kN, {} × {} kN, L = {} m", num(total / 1000.0, 0, de), lines, num(axle / 1000.0, 0, de), num(length, 1, de)))).handles(grips.clone()),
            &points,
        );
        for j in 0..lines {
            let x = vehicle.x + (j as f64 - (lines as f64 - 1.0) / 2.0) * part_2::lm3_axle_spacing_m();
            let (a, b) = (pose.at(x, centre - width / 2.0, DECK_Z + LIFT), pose.at(x, centre + width / 2.0, DECK_Z + LIFT));
            let id = format!("{}:axle-{j}", vehicle.id);
            out.push(
                Spec::new(
                    EntityKind::Load,
                    id.clone(),
                    "specialVehicles",
                    Geometry::Load(LoadGlyph::Line { path: polyline(&[a.clone(), b.clone()]), direction: mul(&Vec3::Z, -1.0), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, axle / width) }, case: case(&id, LoadKind::Variable) }),
                    load(LoadKind::Variable),
                    loc("LM3 axle line of the special vehicle", "LM3-Achslinie des Sonderfahrzeugs"),
                    LAYER_TRAFFIC_ALT,
                ).bound(binding_where(report, |c| c.id == check)).channel(ColourChannel::Role).opacity(0.8).detail(bi(|de| format!("F = {} kN", num(axle / 1000.0, 0, de)))).handles(grips.clone()),
                &[a, b],
            );
        }
    }
}

/// 🌉 Bridge wind cases: the loaded side face with a pressure map q_p(z) c_f,x over the span and the depth d_tot, and the uplift on the soffit.
pub fn bridge_wind(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.bridge;
    for row in &d.wind_bridges {
        if !out.own(&row.id) { continue; }
        let Some((z0, z1, depth_tot)) = wind_face(row) else { continue };
        if !(row.span > 0.0 && row.deck_width > 0.0 && row.deck_depth > 0.0) { continue; }
        let (check, grips) = (format!("en1991.1-4.bridge-wind.{}", row.id), handles::ids(d, layout, &row.id));
        let c_fx = part_1_4::bridge_c_fx0(row.deck_width, depth_tot);
        let face: Vec<Vec3> = [(0.0, z0), (row.span, z0), (row.span, z1), (0.0, z1)].iter().map(|(x, z)| pose.at(*x, -LIFT, *z)).collect();
        out.push(
            Spec::new(EntityKind::Surface, row.id.clone(), "windBridges", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&face) } }), MaterialRole::Neutral, bi(|de| format!("{} {}", if de { "Brückenwindfall" } else { "Bridge wind case" }, row.id)), LAYER_WIND_BRIDGES).bound(binding_where(report, |c| c.id == check)).opacity(0.25).detail(bi(|de| format!("d_tot = {} m, b/d_tot = {}, c_f,x = {}", num(depth_tot, 2, de), num(row.deck_width / depth_tot, 2, de), num(c_fx, 2, de)))).handles(grips.clone()),
            &face,
        );
        let (dx, dz) = (row.span / FIELD_COLUMNS as f64, depth_tot / FIELD_BANDS as f64);
        for j in 0..FIELD_BANDS {
            let z_abs = (row.height_above_ground - depth_tot / 2.0 + (j as f64 + 0.5) * dz).max(1.0);
            let Some(qp) = qp_at(d, z_abs) else { break };
            for i in 0..FIELD_COLUMNS {
                let points: Vec<Vec3> = [(i as f64 * dx, z0 + j as f64 * dz), ((i + 1) as f64 * dx, z0 + j as f64 * dz), ((i + 1) as f64 * dx, z0 + (j + 1) as f64 * dz), (i as f64 * dx, z0 + (j + 1) as f64 * dz)].iter().map(|(x, z)| pose.at(*x, -2.0 * LIFT, *z)).collect();
                area(out, report, format!("{}:field-{i}-{j}", row.id), "windBridges", &points, pose.dir(&Vec3::Y), qp * c_fx, loc("Wind pressure map q_p c_f,x", "Winddruckfeld q_p c_f,x"), &check, LAYER_WIND_BRIDGES, &grips);
            }
        }
        if let Some(qp) = qp_at(d, row.height_above_ground) {
            let soffit: Vec<Vec3> = rect2(0.0, 0.0, row.span, row.deck_width).iter().map(|p| pose.at(p[0], p[1], z0 - LIFT)).collect();
            area(out, report, format!("{}:uplift", row.id), "windBridges", &soffit, Vec3::Z, part_1_4::bridge_force_z_n_m(qp, 1.0), loc("Vertical wind pressure q_p c_f,z", "Vertikaler Winddruck q_p c_f,z"), &check, LAYER_WIND_BRIDGES, &grips);
        }
    }
}

/// 〰️ Slender elements: the tower, the windward pressure profile q_p(z) c_s c_d in bands and the label with the structural factor and the critical velocity.
pub fn dynamic(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for (index, row) in d.dynamic_elements.iter().enumerate() {
        if !out.own(&row.id) { continue; }
        if !(row.height > 0.0 && row.width > 0.0) { continue; }
        let (x, y) = element_origin(d, index);
        let (half, grips) = (row.width / 2.0, handles::ids(d, layout, &row.id));
        let suffix = format!(".{}", row.id);
        let binding = binding_where(report, |c| c.id.ends_with(&suffix) && (c.id.starts_with("en1991.1-4.dyn-") || c.id.starts_with("en1991.1-4.vortex-")));
        let top = pose.at(x, y, row.height);
        out.push(
            Spec::new(EntityKind::Solid, row.id.clone(), "dynamicElements", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([x - half, y - half, 0.0]), base: polygon2(&rect2(0.0, 0.0, row.width, row.width)), height: row.height } }), MaterialRole::Steel, bi(|de| format!("{} {}", if de { "Dynamisches Bauteil" } else { "Dynamic element" }, row.id)), LAYER_DYNAMIC).bound(binding.clone()).opacity(0.5).detail(bi(|de| format!("h = {} m, b = {} m, δ_s = {}", num(row.height, 1, de), num(row.width, 2, de), num(row.log_decrement, 3, de)))).handles(grips.clone()),
            &[pose.at(x - half, y - half, 0.0), top.clone()],
        );
        let band = row.height / PROFILE_BANDS as f64;
        for k in 0..PROFILE_BANDS {
            let Some(qp) = qp_at(d, ((k as f64 + 0.5) * band).max(1.0)) else { break };
            let points: Vec<Vec3> = [(-half, k as f64 * band), (half, k as f64 * band), (half, (k + 1) as f64 * band), (-half, (k + 1) as f64 * band)].iter().map(|(dy, z)| pose.at(x - half - LIFT, y + dy, *z)).collect();
            let id = format!("{}:profile-{k}", row.id);
            let intensity = qp * row.assumed_cscd;
            out.push(
                Spec::new(EntityKind::Load, id.clone(), "dynamicElements", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: pose.dir(&Vec3::X), intensity: pressure(intensity), case: case(&id, LoadKind::Variable) }), load(LoadKind::Variable), loc("Wind pressure profile q_p(z) c_s c_d", "Winddruckprofil q_p(z) c_s c_d"), LAYER_DYNAMIC).bound(binding.clone()).channel(ColourChannel::Role).opacity(0.6).detail(bi(|de| format!("q = {} kN/m²", num(intensity / 1000.0, 2, de)))).handles(grips.clone()),
                &points,
            );
        }
        if let Some((required, v_crit, v_m)) = dynamic_values(d, row) {
            out.label(format!("{}:label", row.id), pose.at(x, y, row.height + 1.0), bi(|de| format!("c_s c_d = {}, v_crit = {} m/s, {} v_m = {} m/s", num(required, 3, de), num(v_crit, 1, de), if de { "1,25" } else { "1.25" }, num(1.25 * v_m, 1, de))), tone_of(&binding));
        }
    }
}

/// 💥 Impacts: the point load in the direction of travel at its height and the impact patch on the member.
pub fn impacts(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for row in d.impact_actions.iter().filter(|r| r.kind <= 2) {
        if !out.own(&row.id) { continue; }
        let (check, grips) = (format!("en1991.1-7.impact-force.{}", row.id), handles::ids(d, layout, &row.id));
        let at = pose.at(row.x, row.y, row.height);
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "impactActions", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: pose.dir(&Vec3::X), magnitude: force(row.assumed_force), case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), impact_name(row), LAYER_IMPACT).bound(binding_where(report, |c| c.id == check)).channel(ColourChannel::Role).opacity(0.9).detail(bi(|de| format!("F = {} kN, m = {} kg, v = {} m/s", num(row.assumed_force / 1000.0, 0, de), num(row.mass, 0, de), num(row.speed, 1, de)))).handles(grips.clone()),
            &[at.clone()],
        );
        let z0 = (row.height - PATCH_HEIGHT / 2.0).max(0.0);
        let points: Vec<Vec3> = [(-PATCH_WIDTH / 2.0, z0), (PATCH_WIDTH / 2.0, z0), (PATCH_WIDTH / 2.0, z0 + PATCH_HEIGHT), (-PATCH_WIDTH / 2.0, z0 + PATCH_HEIGHT)].iter().map(|(dy, z)| pose.at(row.x - LIFT, row.y + dy, *z)).collect();
        area(out, report, format!("{}:patch", row.id), "impactActions", &points, pose.dir(&Vec3::X), row.assumed_force / (PATCH_WIDTH * PATCH_HEIGHT), loc("Impact patch", "Anprallfläche"), &check, LAYER_IMPACT, &grips);
    }
}

fn impact_name(row: &ImpactAction) -> LocalizedCopy {
    match row.kind {
        1 => loc("Ship impact", "Schiffsanprall"),
        2 => loc("Forklift impact", "Gabelstapleranprall"),
        _ => loc("Vehicle impact", "Fahrzeuganprall"),
    }
}

/// 🏠 Internal gas explosions: the room, the design pressure on its walls and roof and the vent on the end wall.
pub fn explosions(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for (index, row) in d.internal_explosions.iter().enumerate() {
        if !out.own(&row.id) { continue; }
        if !(row.volume > 0.0 && row.vent_area > 0.0) { continue; }
        let (length, depth) = room_size(row);
        let (x0, y0) = room_origin(d, index);
        let (x1, y1) = (x0 + length, y0 + depth);
        let (check, grips) = (format!("en1991.1-7.gas-explosion.{}", row.id), handles::ids(d, layout, &row.id));
        let required = part_1_7::gas_explosion_governing_kn_m2(row.vent_pressure / 1000.0, row.vent_area / row.volume) * 1000.0;
        out.push(
            Spec::new(EntityKind::Solid, row.id.clone(), "internalExplosions", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([x0, y0, 0.0]), base: polygon2(&rect2(0.0, 0.0, length, depth)), height: ROOM_HEIGHT } }), MaterialRole::Concrete, bi(|de| format!("{} {}", if de { "Innere Explosion" } else { "Internal explosion" }, row.id)), LAYER_EXPLOSION).bound(binding_where(report, |c| c.id == check)).opacity(0.25).detail(bi(|de| format!("V = {} m³, A_v = {} m², p_d = {} kN/m²", num(row.volume, 1, de), num(row.vent_area, 2, de), num(required / 1000.0, 2, de)))).handles(grips.clone()),
            &[pose.at(x0, y0, 0.0), pose.at(x1, y1, ROOM_HEIGHT)],
        );
        let walls: [(&str, [(f64, f64, f64); 4], Vec3); 5] = [
            ("west", [(x0 + LIFT, y0, 0.0), (x0 + LIFT, y1, 0.0), (x0 + LIFT, y1, ROOM_HEIGHT), (x0 + LIFT, y0, ROOM_HEIGHT)], Vec3::X),
            ("east", [(x1 - LIFT, y0, 0.0), (x1 - LIFT, y1, 0.0), (x1 - LIFT, y1, ROOM_HEIGHT), (x1 - LIFT, y0, ROOM_HEIGHT)], mul(&Vec3::X, -1.0)),
            ("south", [(x0, y0 + LIFT, 0.0), (x1, y0 + LIFT, 0.0), (x1, y0 + LIFT, ROOM_HEIGHT), (x0, y0 + LIFT, ROOM_HEIGHT)], Vec3::Y),
            ("north", [(x0, y1 - LIFT, 0.0), (x1, y1 - LIFT, 0.0), (x1, y1 - LIFT, ROOM_HEIGHT), (x0, y1 - LIFT, ROOM_HEIGHT)], mul(&Vec3::Y, -1.0)),
            ("roof", [(x0, y0, ROOM_HEIGHT - LIFT), (x1, y0, ROOM_HEIGHT - LIFT), (x1, y1, ROOM_HEIGHT - LIFT), (x0, y1, ROOM_HEIGHT - LIFT)], mul(&Vec3::Z, -1.0)),
        ];
        for (name, corners, inward) in walls {
            let points: Vec<Vec3> = corners.iter().map(|c| pose.at(c.0, c.1, c.2)).collect();
            area(out, report, format!("{}:wall-{name}", row.id), "internalExplosions", &points, pose.dir(&inward), required, loc("Explosion design pressure p_d", "Explosions-Bemessungsdruck p_d"), &check, LAYER_EXPLOSION, &grips);
        }
        let side = row.vent_area.sqrt().min(ROOM_HEIGHT - 0.2).min(depth - 0.2).max(0.1);
        let vent: Vec<Vec3> = [(-side / 2.0, ROOM_HEIGHT / 2.0 - side / 2.0), (side / 2.0, ROOM_HEIGHT / 2.0 - side / 2.0), (side / 2.0, ROOM_HEIGHT / 2.0 + side / 2.0), (-side / 2.0, ROOM_HEIGHT / 2.0 + side / 2.0)].iter().map(|(dy, z)| pose.at(x1 + LIFT, y0 + depth / 2.0 + dy, *z)).collect();
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:vent", row.id), "internalExplosions", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&vent) } }), MaterialRole::Neutral, loc("Vent", "Entlastungsöffnung"), LAYER_EXPLOSION).bound(binding_where(report, |c| c.id == check)).opacity(0.7).detail(bi(|de| format!("A_v = {} m², p_v = {} kN/m²", num(row.vent_area, 2, de), num(row.vent_pressure / 1000.0, 2, de)))).handles(grips.clone()),
            &vent,
        );
    }
}

/// 🌡️ Bridge sections: the cross-section slice at the abutment with the linear temperature profile in bands (heating, top warmer).
pub fn thermal_sections(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.bridge;
    let width = deck_width(d);
    for (index, row) in d.thermal_sections.iter().enumerate() {
        if !out.own(&row.id) { continue; }
        let Some((heat, cool)) = part_1_5::bridge_delta_t_m(row.deck_type, row.surfacing) else { continue };
        if !(row.depth > 0.0) { continue; }
        let (check, grips) = (format!("en1991.1-5.section-range.{}", row.id), handles::ids(d, layout, &row.id));
        let x0 = section_x(index);
        let bottom = DECK_Z - row.depth;
        out.push(
            Spec::new(EntityKind::Solid, row.id.clone(), "thermalSections", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([x0, 0.0, bottom]), base: polygon2(&rect2(0.0, 0.0, SLICE_THICKNESS, width)), height: row.depth } }), MaterialRole::Concrete, bi(|de| format!("{} {}", if de { "Temperaturquerschnitt" } else { "Thermal section" }, row.id)), LAYER_THERMAL_SECTIONS).bound(binding_where(report, |c| c.id == check)).opacity(0.5).detail(bi(|de| format!("h = {} m, L = {} m, ΔT_M,heat = {} K, ΔT_M,cool = {} K", num(row.depth, 2, de), num(row.length, 0, de), num(heat, 1, de), num(cool, 1, de)))).handles(grips.clone()),
            &[pose.at(x0, 0.0, bottom), pose.at(x0 + SLICE_THICKNESS, width, DECK_Z)],
        );
        for k in 0..TEMPERATURE_BANDS {
            let fraction = (k as f64 + 0.5) / TEMPERATURE_BANDS as f64;
            let value = heat * (fraction - 0.5);
            let anchor = pose.at(x0 - LIFT, width / 2.0, bottom + fraction * row.depth);
            let id = format!("{}:band-{k}", row.id);
            out.push(
                Spec::new(EntityKind::Load, id.clone(), "thermalSections", Geometry::Load(LoadGlyph::Thermal { target: entity_ref(EntityKind::Solid, &row.id, "thermalSections"), delta_t: Quantity::new(QuantityKind::Temperature, value), case: case(&id, LoadKind::Thermal) }), load(LoadKind::Thermal), loc("Linear temperature difference band", "Band der linearen Temperaturdifferenz"), LAYER_THERMAL_SECTIONS).bound(binding_where(report, |c| c.id == check)).detail(bi(|de| format!("ΔT = {} K", num(value, 1, de)))).handles(grips.clone()),
                &[anchor],
            );
        }
        out.label(format!("{}:label", row.id), pose.at(x0, width / 2.0, DECK_Z + 0.5), bi(|de| format!("ΔT_M,heat = {} K, ΔT_M,cool = {} K", num(heat, 1, de), num(cool, 1, de))), tone_of(&binding_where(report, |c| c.id == check)));
    }
}
