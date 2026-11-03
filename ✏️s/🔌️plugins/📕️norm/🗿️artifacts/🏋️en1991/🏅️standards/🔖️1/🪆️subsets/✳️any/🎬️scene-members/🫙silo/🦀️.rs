//! 🫙 Silo and tank: the cylindrical shell with the horizontal pressure profile, the wall friction profile and the patch load of a silo (Janssen, EN 1991-4) or the hydrostatic profile of a tank.

use super::kit::*;
use super::layout::silo_radius;
use super::model::En1991Snapshot;
use super::scene::*;
use super::{checks, handles, Ctx};
use checks::part_4;

const SHELL_THICKNESS: f64 = 0.02;
const SAMPLES: usize = 12;
const PATCH_FRACTION: f64 = 0.4;
const FIELD_AZIMUTHS: usize = 8;
const FIELD_BANDS: usize = 4;
pub const OUTLET_RADIUS: f64 = 0.3;
pub const HOPPER_TAN_LIMIT: f64 = 11.43;

/// 🔻 Depth of the conical hopper below the transition for a wall angle to the horizontal (clamped below 85°).
pub fn hopper_depth(radius: f64, angle_deg: f64) -> f64 {
    (radius - OUTLET_RADIUS.min(radius)) * angle_deg.to_radians().tan().clamp(0.0, HOPPER_TAN_LIMIT)
}

/// 🫙 Horizontal wall pressure at `depth` below the top in Pa: Janssen for a silo, hydrostatic for a tank.
pub fn horizontal_pa(d: &En1991Snapshot, depth: f64) -> f64 {
    if d.silo_kind == "tank" {
        part_4::tank_hydrostatic_pa(d.silo_bulk_density, depth)
    } else {
        part_4::janssen_horizontal_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, depth)
    }
}

fn profile(pressure_at: impl Fn(f64) -> f64, height: f64) -> LoadProfile {
    LoadProfile::Sampled { samples: (0..=SAMPLES).map(|i| ProfileSample { at: i as f64 / SAMPLES as f64, value: pressure(pressure_at(height * (1.0 - i as f64 / SAMPLES as f64)).max(0.0)) }).collect() }
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    if !out.own(global::SILO) || !d.silo_claimed || !matches!(d.silo_kind.as_str(), "silo" | "tank") {
        return;
    }
    let pose = &layout.silo;
    let (radius, height) = (silo_radius(d), d.silo_height.max(0.5));
    let tank = d.silo_kind == "tank";
    let grips = handles::ids(d, layout, "silo");
    let (base, top) = (pose.at(0.0, 0.0, 0.0), pose.at(0.0, 0.0, height));
    out.push(
        Spec::new(
            EntityKind::Shell,
            "silo",
            "@site",
            Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: base.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(radius, 0.0), Vec2::new(radius, height)] }, sweep: std::f64::consts::TAU }, thickness: SHELL_THICKNESS }),
            MaterialRole::Steel,
            if tank { loc("Tank", "Tank") } else { loc("Silo", "Silo") },
            LAYER_SILO,
        )
        .bound(binding_prefixed(report, &["en1991.4."]))
        .opacity(0.45)
        .detail(bi(|de| format!("R = {} m, H = {} m, γ = {} kN/m³", num(radius, 2, de), num(height, 2, de), num(d.silo_bulk_density / 1000.0, 2, de))))
        .handles(grips.clone()),
        &[base.clone(), top.clone(), pose.at(radius, 0.0, 0.0), pose.at(-radius, 0.0, 0.0), pose.at(0.0, radius, 0.0), pose.at(0.0, -radius, 0.0)],
    );
    let depth = hopper_depth(radius, d.silo_hopper_angle_deg);
    if !tank && depth > 0.0 {
        let outlet = pose.at(OUTLET_RADIUS.min(radius), 0.0, -depth);
        out.push(
            Spec::new(
                EntityKind::Shell,
                "silo:hopper",
                "@site",
                Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: base.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(radius, 0.0), Vec2::new(OUTLET_RADIUS.min(radius), -depth)] }, sweep: std::f64::consts::TAU }, thickness: SHELL_THICKNESS }),
                MaterialRole::Steel,
                loc("Hopper", "Trichter"),
                LAYER_SILO,
            )
            .bound(binding_prefixed(report, &["en1991.4."]))
            .opacity(0.45)
            .detail(bi(|de| format!("α = {}°, {} = {} m", num(d.silo_hopper_angle_deg, 1, de), if de { "Tiefe" } else { "depth" }, num(depth, 2, de))))
            .handles(grips.clone()),
            &[base.clone(), outlet],
        );
    }
    if d.silo_eccentricity > 1e-9 {
        let at = pose.at(d.silo_eccentricity, 0.0, height);
        out.annotate(Annotation::Dimension { id: "silo:eccentricity".into(), from: top.clone(), to: at, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, d.silo_eccentricity), binding: Some(FieldBinding::new("siloEccentricity", Affine::IDENTITY, "m")) }, &top);
    }
    let line = |out: &mut Out, id: &str, at: [f64; 2], direction: Vec3, pressure_at: &dyn Fn(f64) -> f64, check: &str, name: LocalizedCopy| {
        let (a, b) = (pose.at(at[0], at[1], 0.0), pose.at(at[0], at[1], height));
        out.push(
            Spec::new(EntityKind::Load, id, "@site", Geometry::Load(LoadGlyph::Line { path: polyline(&[a.clone(), b.clone()]), direction, profile: profile(pressure_at, height), case: case(id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_SILO)
                .bound(binding_where(report, |c| c.id == check))
                .channel(ColourChannel::Role)
                .detail(bi(|de| format!("p({} m) = {} kN/m²", num(height, 1, de), num(pressure_at(height) / 1000.0, 2, de))))
                .handles(grips.clone()),
            &[a, b],
        );
    };
    out.annotate(Annotation::Dimension { id: "silo:height".into(), from: pose.at(radius + 0.6, 0.0, 0.0), to: pose.at(radius + 0.6, 0.0, height), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, height), binding: Some(FieldBinding::new("siloHeight", Affine::IDENTITY, "m")) }, &top);
    let check = if tank { "en1991.4.tank" } else { "en1991.4.silo-pressure" };
    line(out, "silo:pressure", [radius, 0.0], pose.dir(&mul(&Vec3::X, -1.0)), &|z| horizontal_pa(d, z), check, if tank { loc("Hydrostatic pressure", "Hydrostatischer Druck") } else { loc("Janssen horizontal pressure p_h", "Janssen-Horizontaldruck p_h") });
    let (step, band) = (std::f64::consts::TAU / FIELD_AZIMUTHS as f64, height / FIELD_BANDS as f64);
    for i in 0..FIELD_AZIMUTHS {
        let (a0, a1) = (i as f64 * step, (i as f64 + 1.0) * step);
        for j in 0..FIELD_BANDS {
            let (z_top, z_bottom, id) = (height - j as f64 * band, height - (j as f64 + 1.0) * band, format!("silo:field-{i}-{j}"));
            let p = horizontal_pa(d, (j as f64 + 0.5) * band).max(0.0);
            let rim = radius + 0.03;
            let points: Vec<Vec3> = [(a0, z_top), (a1, z_top), (a1, z_bottom), (a0, z_bottom)].iter().map(|(a, z)| pose.at(rim * a.cos(), rim * a.sin(), *z)).collect();
            let normal = (a0 + a1) / 2.0;
            out.push(
                Spec::new(
                    EntityKind::Load,
                    id.clone(),
                    "@site",
                    Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: pose.dir(&v3(-normal.cos(), -normal.sin(), 0.0)), intensity: pressure(p), case: case(&id, LoadKind::Variable) }),
                    load(LoadKind::Variable),
                    loc("Wall pressure field p_h(θ, z)", "Wanddruckfeld p_h(θ, z)"),
                    LAYER_SILO,
                )
                .bound(binding_where(report, |c| c.id == check))
                .channel(ColourChannel::Role)
                .opacity(0.5)
                .detail(bi(|de| format!("p_h = {} kN/m²", num(p / 1000.0, 2, de))))
                .handles(grips.clone()),
                &points,
            );
        }
    }
    if tank {
        return;
    }
    line(out, "silo:friction", [0.0, radius], mul(&Vec3::Z, -1.0), &|z| part_4::wall_friction_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, z), "en1991.4.silo-wall-friction", loc("Wall friction traction p_w", "Wandreibungstraktion p_w"));
    line(out, "silo:discharge", [-radius, 0.0], pose.dir(&Vec3::X), &|z| part_4::discharge_horizontal_max_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, z), "en1991.4.silo-discharge", loc("Discharge pressure p_he = C_h p_hf,max", "Entleerungsdruck p_he = C_h p_hf,max"));
    let side = (PATCH_FRACTION * radius).min(height);
    let patch_p = part_4::patch_pressure_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, height, d.annex);
    let mid = height / 2.0;
    let points: Vec<Vec3> = [[-side / 2.0, mid - side / 2.0], [side / 2.0, mid - side / 2.0], [side / 2.0, mid + side / 2.0], [-side / 2.0, mid + side / 2.0]].iter().map(|p| pose.at(radius + 0.02, p[0], p[1])).collect();
    out.push(
        Spec::new(
            EntityKind::Load,
            "silo:patch",
            "@site",
            Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: pose.dir(&mul(&Vec3::X, -1.0)), intensity: pressure(patch_p), case: case("silo:patch", LoadKind::Variable) }),
            load(LoadKind::Variable),
            loc("Patch load p_patch", "Patchlast p_patch"),
            LAYER_SILO,
        )
        .bound(binding_where(report, |c| c.id == "en1991.4.silo-patch"))
        .channel(ColourChannel::Role)
        .opacity(0.8)
        .detail(bi(|de| format!("p_patch = {} kN/m²", num(patch_p / 1000.0, 2, de))))
        .handles(grips),
        &points,
    );
}
