//! 🚂 Rail and dust rows in the scene: derailment line loads beside their track strip in the bridge gallery (EN 1991-2, 6.7.1), rail load groups with the LM71 axles, the distributed load, the longitudinal and the lateral force of the group (Table 6.11) and dust explosions as a compartment cube with its vent and the reduced explosion pressure (EN 1991-4, EN 14491).
//! Every entity is toned by every check its row owns and carries the drag handles of `handles`; the geometry follows the grips (`handles::rail_derailment`, `rail_group`, `dust_explosion`).

use super::kit::*;
use super::scene::*;
use super::{handles, Ctx};

const LIFT: f64 = 0.05;
const LOAD_HEIGHT: f64 = 1.0;
const AXLE_HEIGHT: f64 = 1.5;
const FORCE_HEIGHT: f64 = 0.6;
const AXLES: usize = 4;
const UDL_LENGTH: f64 = 10.0;

fn strip(pose: &super::layout::Pose, x0: f64, x1: f64, y: f64, z: f64) -> Vec<Vec3> {
    rect2(x0, y, x1, y + TRACK_GAUGE).iter().map(|p| pose.at(p[0], p[1], z)).collect()
}

/// 🚆 Derailment rows: the track strip of the loaded length and the line load of the design situation beside it at the entered offset from the track axis.
pub fn derailments(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.bridge;
    for (index, row) in d.rail_derailments.iter().enumerate() {
        if !out.own(&row.id) || !(row.loaded_length.is_finite() && row.loaded_length > 0.0 && row.offset.is_finite() && row.offset >= 0.0) {
            continue;
        }
        let y = gallery_y(d, "railDerailments", index);
        let line_y = y + TRACK_GAUGE / 2.0 + row.offset;
        let ends = [pose.at(0.0, line_y, LOAD_HEIGHT), pose.at(row.loaded_length, line_y, LOAD_HEIGHT)];
        let (bound, grips) = (ctx.binding(&row.id, &[]), handles::ids(d, layout, &row.id));
        let name = bi(|de| format!("{} {}", if de { "Entgleisung" } else { "Derailment" }, row.id));
        let detail = bi(|de| format!("{} {}, q = {} kN/m, L = {} m, {} {} m", if de { "Situation" } else { "situation" }, row.situation.to_uppercase(), num(row.assumed_line_load / 1000.0, 1, de), num(row.loaded_length, 1, de), if de { "Abstand" } else { "offset" }, num(row.offset, 2, de)));
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "railDerailments", Geometry::Load(LoadGlyph::Line { path: polyline(&ends), direction: mul(&Vec3::Z, -1.0), profile: LoadProfile::Uniform { value: force(row.assumed_line_load) }, case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), name.clone(), LAYER_RAIL_DERAILMENTS)
                .bound(bound.clone())
                .detail(detail.clone())
                .handles(grips.clone()),
            &ends,
        );
        let track = strip(pose, 0.0, row.loaded_length, y, LIFT);
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:track", row.id), "railDerailments", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&track) } }), load(LoadKind::Accidental), name, LAYER_RAIL_DERAILMENTS)
                .bound(bound.clone())
                .channel(ColourChannel::Role)
                .opacity(0.5)
                .detail(detail)
                .handles(grips),
            &track,
        );
        out.label(format!("{}:label", row.id), pose.at(row.loaded_length / 2.0, line_y, LOAD_HEIGHT + 0.8), bi(|de| format!("q = {} kN/m", num(row.assumed_line_load / 1000.0, 1, de))), tone_of(&bound));
    }
}

/// 🛤️ Rail load group rows: the four LM71 axles of the group, the distributed load beside them, the longitudinal force at the front axle and the lateral force across the track, each with the assumed group action.
pub fn load_groups(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.bridge;
    for (index, row) in d.rail_load_groups.iter().enumerate() {
        if !out.own(&row.id) {
            continue;
        }
        let y = gallery_y(d, "railLoadGroups", index);
        let (mid, end) = (y + TRACK_GAUGE / 2.0, (AXLES - 1) as f64 * LM71_AXLE_SPACING);
        let (bound, grips) = (ctx.binding(&row.id, &[]), handles::ids(d, layout, &row.id));
        let name = bi(|de| format!("{} {}", if de { "Lastgruppe" } else { "Load group" }, row.id));
        let detail = bi(|de| format!("{}, {} / {} / {}", row.load_group, num(row.factor_vertical, 2, de), num(row.factor_longitudinal, 2, de), num(row.factor_lateral, 2, de)));
        let front = pose.at(end, mid, FORCE_HEIGHT);
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "railLoadGroups", Geometry::Load(LoadGlyph::Point { at: front.clone(), direction: pose.dir(&Vec3::X), magnitude: force(row.assumed_longitudinal), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name.clone(), LAYER_RAIL_GROUPS)
                .bound(bound.clone())
                .detail(detail.clone())
                .handles(grips.clone()),
            &[front.clone()],
        );
        let side = pose.at(end / 2.0, y + TRACK_GAUGE, FORCE_HEIGHT);
        out.push(
            Spec::new(EntityKind::Load, format!("{}:lateral", row.id), "railLoadGroups", Geometry::Load(LoadGlyph::Point { at: side.clone(), direction: pose.dir(&Vec3::Y), magnitude: force(row.assumed_lateral), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name.clone(), LAYER_RAIL_GROUPS)
                .bound(bound.clone())
                .detail(detail.clone())
                .handles(grips.clone()),
            &[side],
        );
        for k in 0..AXLES {
            let at = pose.at(k as f64 * LM71_AXLE_SPACING, mid, AXLE_HEIGHT);
            out.push(
                Spec::new(EntityKind::Load, format!("{}:axle-{k}", row.id), "railLoadGroups", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: force(row.assumed_axle_load), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name.clone(), LAYER_RAIL_GROUPS)
                    .bound(bound.clone())
                    .detail(detail.clone())
                    .handles(grips.clone()),
                &[at],
            );
        }
        let udl = [pose.at(end + 2.0, mid, LOAD_HEIGHT), pose.at(end + 2.0 + UDL_LENGTH, mid, LOAD_HEIGHT)];
        out.push(
            Spec::new(EntityKind::Load, format!("{}:udl", row.id), "railLoadGroups", Geometry::Load(LoadGlyph::Line { path: polyline(&udl), direction: mul(&Vec3::Z, -1.0), profile: LoadProfile::Uniform { value: force(row.assumed_udl) }, case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_RAIL_GROUPS)
                .bound(bound.clone())
                .detail(detail)
                .handles(grips),
            &udl,
        );
        out.label(format!("{}:label", row.id), pose.at(end / 2.0, mid, AXLE_HEIGHT + 1.0), bi(|de| format!("{}: {} kN", row.load_group, num(row.assumed_axle_load / 1000.0, 0, de))), tone_of(&bound));
    }
}

/// 💥 Dust explosion rows: the compartment as a cube of the volume V, the vent square of the vent area A_v on its roof and the reduced explosion pressure p_red as an explosion glyph at its centre.
pub fn dust_explosions(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.silo;
    for (index, row) in d.dust_explosions.iter().enumerate() {
        if !out.own(&row.id) || !(row.volume.is_finite() && row.volume > 0.0 && row.vent_area.is_finite() && row.vent_area > 0.0 && row.design_pressure.is_finite() && row.design_pressure >= 0.0) {
            continue;
        }
        let y = gallery_y(d, "dustExplosions", index);
        let side = row.volume.max(1.0).cbrt().clamp(1.0, 30.0);
        let (bound, grips) = (ctx.binding(&row.id, &[]), handles::ids(d, layout, &row.id));
        let name = bi(|de| format!("{} {}", if de { "Staubexplosion" } else { "Dust explosion" }, row.id));
        let detail = bi(|de| format!("V = {} m³, A_v = {} m², p_red = {} bar", num(row.volume, 1, de), num(row.vent_area, 2, de), num(row.design_pressure / 1e5, 2, de)));
        let centre = pose.at(side / 2.0, y + side / 2.0, side / 2.0);
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "dustExplosions", Geometry::Load(LoadGlyph::Explosion { centre: centre.clone(), pressure: pressure(row.design_pressure), shape: PressureWave::Spherical, case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), name.clone(), LAYER_DUST)
                .bound(bound.clone())
                .detail(detail.clone())
                .handles(grips.clone()),
            &[centre],
        );
        let cube = [pose.at(0.0, y, 0.0), pose.at(side, y + side, side)];
        out.push(
            Spec::new(EntityKind::Solid, format!("{}:compartment", row.id), "dustExplosions", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, y, 0.0]), base: polygon2(&rect2(0.0, 0.0, side, side)), height: side } }), load(LoadKind::Accidental), name.clone(), LAYER_DUST)
                .bound(bound.clone())
                .opacity(0.35)
                .detail(detail.clone())
                .handles(grips.clone()),
            &cube,
        );
        let vent_side = row.vent_area.sqrt().min(side);
        let inset = (side - vent_side) / 2.0;
        let vent: Vec<Vec3> = rect2(inset, y + inset, inset + vent_side, y + inset + vent_side).iter().map(|p| pose.at(p[0], p[1], side + LIFT)).collect();
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:vent", row.id), "dustExplosions", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&vent) } }), load(LoadKind::Accidental), name, LAYER_DUST)
                .bound(bound.clone())
                .channel(ColourChannel::Role)
                .opacity(0.7)
                .detail(detail)
                .handles(grips),
            &vent,
        );
        out.label(format!("{}:label", row.id), pose.at(side / 2.0, y + side / 2.0, side + 1.0), bi(|de| format!("p_red = {} bar", num(row.design_pressure / 1e5, 2, de))), tone_of(&bound));
    }
}
