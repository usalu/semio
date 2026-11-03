//! 🧷 Wave 3 collections in the scene: road fatigue vehicles (FLM1 tandem with its distributed load, FLM3 single vehicle) in the bridge gallery, vehicle collisions on supports and soffits at their position on the deck, localised fires with the flame, the ceiling and the Hasemi heat flux, and snow guards on the eaves of their roof.

use super::kit::*;
use super::layout::{deck_width, GAP};
use super::scene::*;
use super::{checks, handles, Ctx};
use checks::{part_1_2, part_1_3, part_2};

const LIFT: f64 = 0.05;
const AXLE_HEIGHT: f64 = 1.5;
const CONTACT_PAD: f64 = 0.4;
const FIRE_SPACING: f64 = 10.0;
const CEILING_SIDE: f64 = 8.0;
const FLAME_SIDES: usize = 16;
const GUARD_HEIGHT: f64 = 0.4;

/// 🔥 Plan origin (x, y) of localised fire `index` in the building frame: the fires stand in a row south-east of the slender elements.
pub fn fire_origin(d: &super::model::En1991Snapshot, index: usize) -> (f64, f64) {
    (d.depth + 2.0 * GAP + FIRE_SPACING * index as f64, d.width / 2.0 - 25.0)
}

/// 🛡️ Along-slope position of the eaves of a roof (its lower end) and the sign of the down-slope direction in the roof frame.
pub fn guard_eaves(shape: &super::envelope::RoofShape) -> (f64, f64) {
    if shape.z(0.0, shape.b / 2.0) <= shape.z(shape.d, shape.b / 2.0) { (0.0, -1.0) } else { (shape.d, 1.0) }
}

/// 🔁 Axle positions (m from the front axle) and axle load (N) of a fatigue model, `None` for an unknown model or lane.
pub fn fatigue_axles(row: &super::model::FatigueModel, annex: super::norm::AnnexChoice) -> Option<(Vec<f64>, f64)> {
    if !(1..=3).contains(&row.lane) {
        return None;
    }
    match row.model.as_str() {
        "flm1" => Some((vec![0.0, part_2::lm1_axle_spacing_m()], part_2::fatigue_flm1_axle_n(annex, row.lane))),
        "flm3" => Some((part_2::fatigue_flm3_spacings_m().iter().scan(0.0, |x, s| { *x += s; Some(*x) }).fold(vec![0.0], |mut v, x| { v.push(x); v }), part_2::fatigue_flm3_axle_n())),
        _ => None,
    }
}

/// 🔁 Fatigue vehicles: the footprint strip of the vehicle in the bridge gallery with a point load per axle and, for FLM1, the distributed load of the lane (EN 1991-2, 4.6).
pub fn fatigue_models(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    for (index, row) in d.fatigue_models.iter().enumerate() {
        let Some((axles, axle_n)) = fatigue_axles(row, d.annex).filter(|_| out.own(&row.id)) else { continue };
        let (length, width) = (axles.last().copied().unwrap_or(0.0) + CONTACT_PAD, part_2::fatigue_flm3_track_m() + CONTACT_PAD);
        let y = gallery_y(d, "fatigueModels", index);
        let points: Vec<Vec3> = rect2(-CONTACT_PAD / 2.0, y, length - CONTACT_PAD / 2.0, y + width).iter().map(|p| layout.bridge.at(p[0], p[1], 0.0)).collect();
        let bound = binding_where(report, |c| c.id == format!("en1991.2.fatigue-axle.{}", row.id));
        let grips = handles::ids(d, layout, &row.id);
        let name = bi(|de| format!("{} {}", if de { "Ermüdungsmodell" } else { "Fatigue model" }, row.id));
        let detail = bi(|de| format!("{}, {} {}, {} {} kN", row.model.to_uppercase(), if de { "Fahrstreifen" } else { "lane" }, row.lane, if de { "Achslast" } else { "axle load" }, num(axle_n / 1000.0, 1, de)));
        surface(out, &row.id, "fatigueModels", &points, name.clone(), detail.clone(), LAYER_FATIGUE, bound.clone(), grips.clone());
        for (k, x) in axles.iter().enumerate() {
            let at = layout.bridge.at(*x, y + width / 2.0, AXLE_HEIGHT);
            out.push(
                Spec::new(EntityKind::Load, format!("{}:axle-{k}", row.id), "fatigueModels", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: force(axle_n), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name.clone(), LAYER_FATIGUE)
                    .bound(bound.clone())
                    .channel(ColourChannel::Role)
                    .detail(detail.clone())
                    .handles(grips.clone()),
                &[at],
            );
        }
        if row.model == "flm1" {
            let udl = part_2::fatigue_flm1_udl_pa(d.annex, row.lane);
            out.push(
                Spec::new(EntityKind::Load, format!("{}:udl", row.id), "fatigueModels", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: mul(&Vec3::Z, -1.0), intensity: pressure(udl), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_FATIGUE)
                    .bound(binding_where(report, |c| c.id == format!("en1991.2.fatigue-udl.{}", row.id)))
                    .channel(ColourChannel::Role)
                    .opacity(0.6)
                    .detail(detail)
                    .handles(grips),
                &points,
            );
        }
    }
}

/// 🚛 Support collisions: the impact force in driving direction (and across it for a support at deck level) at the position of the struck support, bound to the covering checks (EN 1991-2, 4.7).
pub fn support_collisions(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let (pose, deck) = (&layout.bridge, deck_width(d));
    for row in &d.support_collisions {
        if !out.own(&row.id) || !(row.height > 0.0 && row.position_x.is_finite()) || !part_2::COLLISION_LOCATIONS.contains(&row.location.as_str()) {
            continue;
        }
        let soffit = row.location == "soffit";
        let fx = part_2::collision_force_x_n(soffit);
        let grips = handles::ids(d, layout, &row.id);
        let bound = binding_where(report, |c| c.id == format!("en1991.2.collision-x.{}", row.id));
        let name = bi(|de| format!("{} {}", if de { "Stützenanprall" } else { "Support collision" }, row.id));
        let detail = bi(|de| format!("{}, F_dx = {} kN, {} {} m", if soffit { if de { "Unterseite" } else { "soffit" } } else if de { "Stütze auf Deckebene" } else { "support at deck level" }, num(fx / 1000.0, 0, de), if de { "Höhe" } else { "height" }, num(row.height, 2, de)));
        let at = pose.at(row.position_x, deck / 2.0, row.height);
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "supportCollisions", Geometry::Load(LoadGlyph::Impact { at: at.clone(), direction: pose.dir(&Vec3::X), energy_or_force: force(fx), case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), name.clone(), LAYER_COLLISIONS)
                .bound(bound.clone())
                .detail(detail.clone())
                .handles(grips.clone()),
            &[at.clone()],
        );
        if !soffit {
            let side = pose.at(row.position_x, 0.0, row.height);
            out.push(
                Spec::new(EntityKind::Load, format!("{}:cross", row.id), "supportCollisions", Geometry::Load(LoadGlyph::Impact { at: side.clone(), direction: pose.dir(&Vec3::Y), energy_or_force: force(part_2::collision_force_y_n()), case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), name, LAYER_COLLISIONS)
                    .bound(binding_where(report, |c| c.id == format!("en1991.2.collision-y.{}", row.id)))
                    .detail(detail)
                    .handles(grips),
                &[side],
            );
        }
        out.label(format!("{}:label", row.id), pose.at(row.position_x, deck / 2.0, row.height + 1.0), bi(|de| format!("F_dx = {} kN", num(fx / 1000.0, 0, de))), tone_of(&bound));
    }
}

/// 🔥 Localised fires: the flame as a prism of the flame length L_f over the fire disc, the ceiling slab at the height H and, when the flame reaches the ceiling, the Hasemi heat flux on the ceiling; below the ceiling the plume temperature labels the ceiling (EN 1991-1-2, Annex C).
pub fn local_fires(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for (index, row) in d.local_fires.iter().enumerate() {
        if !out.own(&row.id) || !(row.fire_diameter > 0.0 && row.heat_release_rate > 0.0 && row.ceiling_height >= 0.0) {
            continue;
        }
        let (x, y) = fire_origin(d, index);
        let (lf, z0, _) = part_1_2::local_fire(row.fire_diameter, row.heat_release_rate, 1.0);
        let radius = row.fire_diameter / 2.0;
        let grips = handles::ids(d, layout, &row.id);
        let bound = binding_where(report, |c| c.id == format!("en1991.1-2.local-flux.{}", row.id));
        let name = bi(|de| format!("{} {}", if de { "Lokaler Brand" } else { "Localised fire" }, row.id));
        let detail = bi(|de| format!("D = {} m, Q = {} MW, L_f = {} m, z₀ = {} m", num(row.fire_diameter, 2, de), num(row.heat_release_rate / 1e6, 2, de), num(lf, 2, de), num(z0, 2, de)));
        let disc: Vec<[f64; 2]> = (0..FLAME_SIDES).map(|k| { let a = std::f64::consts::TAU * k as f64 / FLAME_SIDES as f64; [radius * (1.0 + a.cos()), radius * (1.0 + a.sin())] }).collect();
        let flame = if row.ceiling_height > 0.0 { lf.min(row.ceiling_height) } else { lf }.max(0.1);
        let foot = pose.at(x - radius, y - radius, 0.0);
        out.push(
            Spec::new(EntityKind::Solid, row.id.clone(), "localFires", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([x - radius, y - radius, 0.0]), base: polygon2(&disc), height: flame } }), load(LoadKind::Accidental), name.clone(), LAYER_LOCAL_FIRES)
                .bound(bound.clone())
                .detail(detail.clone())
                .handles(grips.clone()),
            &[foot, pose.at(x, y, flame)],
        );
        if row.ceiling_height <= 0.0 {
            out.label(format!("{}:label", row.id), pose.at(x, y, flame + 0.5), bi(|de| format!("L_f = {} m", num(lf, 2, de))), tone_of(&bound));
            continue;
        }
        let half = CEILING_SIDE / 2.0;
        let ceiling: Vec<Vec3> = rect2(x - half, y - half, x + half, y + half).iter().map(|p| pose.at(p[0], p[1], row.ceiling_height + LIFT)).collect();
        surface(out, &format!("{}:ceiling", row.id), "localFires", &ceiling, name.clone(), detail.clone(), LAYER_LOCAL_FIRES, bound.clone(), grips.clone());
        if lf >= row.ceiling_height {
            let z_prime = part_1_2::local_fire_virtual_source_m(row.heat_release_rate, row.fire_diameter);
            let l_h = part_1_2::local_fire_horizontal_flame_m(row.heat_release_rate, row.ceiling_height);
            let flux = part_1_2::hasemi_flux_w_m2(part_1_2::hasemi_y(row.horizontal_distance, row.ceiling_height, z_prime, l_h));
            out.push(
                Spec::new(EntityKind::Load, format!("{}:flux", row.id), "localFires", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&ceiling), direction: mul(&Vec3::Z, 1.0), intensity: Quantity::new(QuantityKind::Power, flux), case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), name, LAYER_LOCAL_FIRES)
                    .bound(bound.clone())
                    .channel(ColourChannel::Role)
                    .opacity(0.7)
                    .detail(detail)
                    .handles(grips),
                &ceiling,
            );
            out.label(format!("{}:label", row.id), pose.at(x, y, row.ceiling_height + 0.8), bi(|de| format!("ĥ = {} kW/m²", num(flux / 1000.0, 1, de))), tone_of(&bound));
        } else {
            let (_, _, theta) = part_1_2::local_fire(row.fire_diameter, row.heat_release_rate, row.ceiling_height);
            out.label(format!("{}:label", row.id), pose.at(x, y, row.ceiling_height + 0.8), bi(|de| format!("θ(H) = {} °C", num(theta, 0, de))), Tone::Info);
        }
    }
}

/// 🛡️ Snow guards: the guard bar along the eaves of its roof with the line force F_s = s b sin α acting down the slope (EN 1991-1-3, 6.3).
pub fn snow_guards(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let Some(sk) = part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk) else { return };
    for row in &d.snow_guards {
        let Some((index, roof)) = d.roofs.iter().enumerate().find(|(_, r)| r.id == row.roof_id) else { continue };
        if !out.own(&row.id) || !(row.slope_length > 0.0) {
            continue;
        }
        let (shape, pose) = (super::envelope::RoofShape::of(d, Some(roof)), layout.roof(index));
        let (eaves, down) = guard_eaves(&shape);
        let mu = part_1_3::shape_mu(&roof.roof_type, roof.pitch_deg, roof.has_parapet, roof.parapet_height, roof.drift_obstruction_height, roof.multi_span, sk, d.annex);
        let required = part_1_3::snow_guard_force_n_m(part_1_3::roof_snow_pa(sk, mu, roof.c_e, roof.c_t), row.slope_length, roof.pitch_deg);
        let grips = handles::ids(d, layout, &row.id);
        let bound = binding_where(report, |c| c.id == format!("en1991.1-3.snow-guard.{}", row.id));
        let name = bi(|de| format!("{} {}", if de { "Schneefanggitter" } else { "Snow guard" }, row.id));
        let detail = bi(|de| format!("{} {}, b = {} m, F_s = {} kN/m", if de { "Dach" } else { "roof" }, roof.id, num(row.slope_length, 1, de), num(required / 1000.0, 3, de)));
        let [a, b] = [0.0, shape.b].map(|y| { let p = shape.lift([eaves, y], LIFT); pose.at(p.x, p.y, p.z) });
        let top = [a.clone(), b.clone()].map(|p| v3(p.x, p.y, p.z + GUARD_HEIGHT));
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "snowGuards", Geometry::Load(LoadGlyph::Line { path: polyline(&top), direction: pose.dir(&v3(down, 0.0, 0.0)), profile: LoadProfile::Uniform { value: force(required) }, case: case(&row.id, LoadKind::Snow) }), load(LoadKind::Snow), name, LAYER_SNOW_GUARDS)
                .bound(bound.clone())
                .detail(detail)
                .handles(grips),
            &[a, b],
        );
        let mid = shape.lift([eaves, shape.b / 2.0], LIFT);
        out.label(format!("{}:label", row.id), pose.at(mid.x, mid.y, mid.z + 1.0), bi(|de| format!("F_s = {} kN/m", num(required / 1000.0, 2, de))), tone_of(&bound));
    }
}
