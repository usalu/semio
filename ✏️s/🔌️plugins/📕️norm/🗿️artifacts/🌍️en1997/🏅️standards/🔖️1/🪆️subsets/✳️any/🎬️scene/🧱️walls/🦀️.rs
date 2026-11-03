//! 🧱️ Walls: inverted-T wall body, backfill, active and passive wedges, earth-pressure diagrams with resultants, surcharge and dimensions.

use super::checks::lateral_and_seismic as seismic;
use super::kit::*;
use super::layout::{wall_reach, wall_stem_x, WALL_LENGTH};
use super::mechanics as mech;
use super::model::RetainingWall;
use super::scene::*;
use super::Ctx;

pub const PRESSURE_SCALE: f64 = 40_000.0;
const MIN_SLAB: f64 = 0.4;
const MAX_REINFORCEMENT_LAYERS: usize = 40;

fn slab(w: &RetainingWall) -> f64 {
    (0.12 * mech::total_height(w)).max(MIN_SLAB).min(0.5 * mech::total_height(w).max(MIN_SLAB))
}

fn profile(w: &RetainingWall) -> Vec<[f64; 2]> {
    let (toe, t, tb) = (wall_stem_x(w), w.stem_thickness.max(0.05), slab(w));
    let end = (w.base_width - toe).max(t + 0.1);
    let base = -w.embedment;
    vec![[-toe, base], [end, base], [end, base + tb], [t, base + tb], [t, w.height], [0.0, w.height], [0.0, base + tb], [-toe, base + tb]]
}

fn surface(out: &mut Out, id: String, name: LocalizedCopy, ring: &[Vec3], binding: Option<UtilisationBinding>, opacity: f64, layer: &'static str) {
    out.push(Spec::new(EntityKind::Surface, id, "retainingWalls", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(ring) } }), soil("mechanism"), name, layer).bound(binding).opacity(opacity), ring);
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for w in &d.retaining_walls {
        if !out.own("retainingWalls", &w.id) {
            continue;
        }
        if w.height <= 1e-6 {
            continue;
        }
        let pose = layout.pose(&w.id);
        let overall = ctx.binding(&w.id, &[]);
        let t = w.stem_thickness.max(0.05);
        let origin = pose.at(0.0, WALL_LENGTH / 2.0, 0.0);
        let outline = profile(w);
        out.push(
            Spec::new(
                EntityKind::Solid,
                w.id.clone(),
                "retainingWalls",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.section([0.0, WALL_LENGTH / 2.0, 0.0]), base: polygon2(&outline), height: WALL_LENGTH } }),
                MaterialRole::Concrete,
                loc(&format!("Retaining wall {}", w.id), &format!("Stützwand {}", w.id)),
                LAYER_WALLS,
            )
            .bound(overall.clone())
            .detail(format!("height {:.2} m, embedment {:.2} m, base {:.2} m", w.height, w.embedment, w.base_width), format!("Höhe {:.2} m, Einbindetiefe {:.2} m, Fußbreite {:.2} m", w.height, w.embedment, w.base_width))
            .handles(ctx.handles_of(&w.id)),
            &[origin.clone(), pose.at(-wall_stem_x(w), -WALL_LENGTH / 2.0, -w.embedment), pose.at(t, WALL_LENGTH / 2.0, w.height)],
        );
        let reach = wall_reach(w);
        if w.reinforcement_spacing > 1e-6 {
            let length = (0.7 * w.height).min(reach);
            for k in 1..=((w.height / w.reinforcement_spacing).floor() as usize).min(MAX_REINFORCEMENT_LAYERS) {
                let z = k as f64 * w.reinforcement_spacing;
                out.push(
                    Spec::new(
                        EntityKind::Solid,
                        format!("{}:reinforcement-{k}", w.id),
                        "retainingWalls",
                        Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.section([0.0, WALL_LENGTH / 2.0, 0.0]), base: polygon2(&[[t, z - 0.02], [t + length, z - 0.02], [t + length, z + 0.02], [t, z + 0.02]]), height: WALL_LENGTH } }),
                        MaterialRole::Concrete,
                        loc(&format!("Soil reinforcement layer {k} of {}", w.id), &format!("Bewehrungslage {k} von {}", w.id)),
                        LAYER_WALLS,
                    )
                    .detail(format!("spacing {:.2} m, length {:.2} m", w.reinforcement_spacing, length), format!("Abstand {:.2} m, Länge {:.2} m", w.reinforcement_spacing, length)),
                    &[pose.at(t, WALL_LENGTH / 2.0, z), pose.at(t + length, -WALL_LENGTH / 2.0, z)],
                );
            }
        }
        out.push(
            Spec::new(
                EntityKind::Solid,
                format!("{}:backfill", w.id),
                "retainingWalls",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.section([0.0, WALL_LENGTH / 2.0, 0.0]), base: polygon2(&[[t, 0.0], [t + reach, 0.0], [t + reach, w.height], [t, w.height]]), height: WALL_LENGTH } }),
                soil("backfill"),
                loc(&format!("Backfill of {}", w.id), &format!("Hinterfüllung von {}", w.id)),
                LAYER_WALLS,
            )
            .channel(ColourChannel::Role)
            .opacity(0.7)
            .detail(format!("phi' {:.1} deg, gamma {:.1} kN/m3", w.backfill_phi_deg, w.backfill_gamma / 1000.0), format!("phi' {:.1} Grad, gamma {:.1} kN/m3", w.backfill_phi_deg, w.backfill_gamma / 1000.0)),
            &[pose.at(t, -WALL_LENGTH / 2.0, 0.0), pose.at(t + reach, WALL_LENGTH / 2.0, w.height)],
        );
        let entity = entity_ref(EntityKind::Solid, &w.id, "retainingWalls");
        let at = |x: f64, z: f64| pose.at(x, 0.0, z);
        let hw = mech::total_height(w);
        let theta = std::f64::consts::FRAC_PI_4 + w.backfill_phi_deg.to_radians() / 2.0;
        let (active_bind, passive_bind) = (ctx.binding(&w.id, &["en1997.9"]), ctx.binding(&w.id, &["en1997.9"]));
        surface(out, format!("{}:active-wedge", w.id), loc(&format!("Active wedge of {}", w.id), &format!("Aktiver Erddruckkeil von {}", w.id)), &[at(t, -w.embedment), at(t + hw / theta.tan(), w.height), at(t, w.height)], active_bind, 0.25, LAYER_MECHANISMS);
        if w.embedment > 1e-6 {
            let reach_p = w.embedment / (std::f64::consts::FRAC_PI_4 - w.backfill_phi_deg.to_radians() / 2.0).tan();
            surface(out, format!("{}:passive-wedge", w.id), loc(&format!("Passive wedge of {}", w.id), &format!("Passiver Erdwiderstandskeil von {}", w.id)), &[at(0.0, -w.embedment), at(-reach_p, 0.0), at(0.0, 0.0)], passive_bind, 0.25, LAYER_MECHANISMS);
        }
        seismic_wedge(ctx, w, &pose, out);
        let e_top = mech::driving_pressure(w, 0.0) / PRESSURE_SCALE;
        let e_bot = mech::driving_pressure(w, hw) / PRESSURE_SCALE;
        if e_bot > 1e-9 {
            let ring = [at(t, w.height), at(t + e_top, w.height), at(t + e_bot, -w.embedment), at(t, -w.embedment)];
            let mut spec = Spec::new(EntityKind::Surface, format!("{}:pressure-active", w.id), "retainingWalls", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&ring) } }), soil("mechanism"), loc(&format!("Earth pressure on {}", w.id), &format!("Erddruck auf {}", w.id)), LAYER_WALLS).bound(ctx.binding(&w.id, &["en1997.9"])).opacity(0.6);
            if let Some(check) = projection_kit::check_with(report, &format!("en1997.9.sliding.{}", w.id)).filter(|c| c.station_field == "earth-pressure" && !c.stations.is_empty()) {
                let u = projection_kit::field_along(check, &[0.0, 0.0, 1.0, 1.0]);
                spec = spec.bound(Some(projection_kit::binding_of_check(check))).channel(ColourChannel::Fields { active: "earth-pressure".into(), fields: vec![ScalarField { id: "earth-pressure".into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: u } }] });
            }
            out.push(spec, &ring);
        }
        let pass = mech::passive_pressure(w, w.embedment) / PRESSURE_SCALE;
        if w.embedment > 1e-6 && pass > 1e-9 {
            surface(out, format!("{}:pressure-passive", w.id), loc(&format!("Passive resistance at {}", w.id), &format!("Erdwiderstand an {}", w.id)), &[at(0.0, 0.0), at(0.0, -w.embedment), at(-pass, -w.embedment)], None, 0.6, LAYER_WALLS);
        }
        let (ea, lever_a) = mech::driving_resultant(w);
        let (ep, lever_p) = mech::passive_resultant(w);
        let case = load_case("earth-pressure", LoadKind::Permanent);
        if ea > 1.0 {
            let p = at(t, -w.embedment + lever_a);
            out.push(Spec::new(EntityKind::Load, format!("{}:resultant-active", w.id), "retainingWalls", Geometry::Load(LoadGlyph::Point { at: p.clone(), direction: pose.dir(&Vec3::new(-1.0, 0.0, 0.0)), magnitude: force(ea * WALL_LENGTH), case: case.clone() }), MaterialRole::Load { kind: LoadKind::Permanent }, loc(&format!("Earth pressure resultant of {}", w.id), &format!("Erddruckresultierende von {}", w.id)), LAYER_LOADS), &[p]);
        }
        if ep > 1.0 {
            let p = at(0.0, -w.embedment + lever_p);
            out.push(Spec::new(EntityKind::Load, format!("{}:resultant-passive", w.id), "retainingWalls", Geometry::Load(LoadGlyph::Point { at: p.clone(), direction: pose.dir(&Vec3::X), magnitude: force(ep * WALL_LENGTH), case: load_case("earth-resistance", LoadKind::Permanent) }), MaterialRole::Load { kind: LoadKind::Permanent }, loc(&format!("Passive resultant of {}", w.id), &format!("Erdwiderstandsresultierende von {}", w.id)), LAYER_LOADS), &[p]);
        }
        if w.surcharge > 1.0 {
            let ring = [pose.at(t, -WALL_LENGTH / 2.0, w.height), pose.at(t + reach, -WALL_LENGTH / 2.0, w.height), pose.at(t + reach, WALL_LENGTH / 2.0, w.height), pose.at(t, WALL_LENGTH / 2.0, w.height)];
            out.push(Spec::new(EntityKind::Load, format!("{}:surcharge", w.id), "retainingWalls", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&ring), direction: Vec3::new(0.0, 0.0, -1.0), intensity: Quantity::new(QuantityKind::Pressure, w.surcharge), case: load_case("surcharge", LoadKind::Variable) }), MaterialRole::Load { kind: LoadKind::Variable }, loc(&format!("Surcharge behind {}", w.id), &format!("Auflast hinter {}", w.id)), LAYER_LOADS), &ring);
        }
        let label = at(t + 0.2, w.height + 0.4);
        out.annotate(
            Annotation::Label {
                id: format!("{}:coefficients", w.id),
                anchor: label.clone(),
                text: loc(&format!("K = {:.3}, Kp = {:.2}, Ea = {:.1} kN/m", mech::k_driving(w), mech::k_passive(w), ea / 1000.0), &format!("K = {:.3}, Kp = {:.2}, Ea = {:.1} kN/m", mech::k_driving(w), mech::k_passive(w), ea / 1000.0)),
                tone: tone_of(&overall),
            },
            &label,
        );
        if let Some(clause) = overall.as_ref().and_then(|b| clause_of(report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", w.id), anchor: entity, clause }, &label);
        }
        for (tag, from, to, value, field) in [
            ("height", at(t + 0.5, 0.0), at(t + 0.5, w.height), w.height, "height"),
            ("embedment", at(-wall_stem_x(w) - 0.5, 0.0), at(-wall_stem_x(w) - 0.5, -w.embedment), w.embedment, "embedment"),
            ("base", pose.at(-wall_stem_x(w), -WALL_LENGTH / 2.0 - 0.5, -w.embedment), pose.at(w.base_width - wall_stem_x(w), -WALL_LENGTH / 2.0 - 0.5, -w.embedment), w.base_width, "baseWidth"),
        ] {
            if from.distance(&to) > 1e-9 {
                out.annotate(Annotation::Dimension { id: format!("{}:dim-{tag}", w.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, value), binding: Some(FieldBinding::new(format!("retainingWalls[id={}].{field}", w.id), Affine::IDENTITY, "m")) }, &from);
            }
        }
    }
}

/// 🌋 Seismic active wedge of the wall (RW-08): the critical Mononobe-Okabe plane from the wall base to the surface as a translucent triangle bound to the seismic sliding check, the thrust glyph at 0.6 H and a label with K_ae, E_ae and the increment over the static thrust; nothing without a seismic coefficient.
fn seismic_wedge(ctx: &Ctx, w: &RetainingWall, pose: &super::layout::Pose, out: &mut Out) {
    let Some(state) = seismic::seismic_wall(w, w.base_width) else { return };
    let report = ctx.input.report;
    let t = w.stem_thickness.max(0.05);
    let bound = ctx.binding(&w.id, &["en1998.5.7.3"]);
    let at = |x: f64, z: f64| pose.at(x, 0.0, z);
    let reach = w.height / state.wedge_deg.to_radians().tan();
    let ring = [at(t, 0.0), at(t + reach, w.height), at(t, w.height)];
    surface(
        out,
        format!("{}:seismic-wedge", w.id),
        loc(&format!("Seismic active wedge of {} at {:.1} deg", w.id, state.wedge_deg), &format!("Seismischer Erddruckkeil von {} bei {:.1} Grad", w.id, state.wedge_deg)),
        &ring,
        bound.clone(),
        0.4,
        LAYER_MECHANISMS,
    );
    let point = at(t, 0.6 * w.height);
    out.push(
        Spec::new(
            EntityKind::Load,
            format!("{}:resultant-seismic", w.id),
            "retainingWalls",
            Geometry::Load(LoadGlyph::Point { at: point.clone(), direction: pose.dir(&Vec3::new(-1.0, 0.0, 0.0)), magnitude: force(state.thrust.total() * WALL_LENGTH), case: load_case("seismic-earth-pressure", LoadKind::Seismic) }),
            MaterialRole::Load { kind: LoadKind::Seismic },
            loc(&format!("Seismic thrust on {}", w.id), &format!("Erdbebenerddruck auf {}", w.id)),
            LAYER_LOADS,
        ),
        std::slice::from_ref(&point),
    );
    let anchor = at(t + 0.5 * reach, 0.5 * w.height);
    let delta = (state.thrust.total() - state.static_thrust) / 1000.0;
    out.annotate(
        Annotation::Label {
            id: format!("{}:seismic-label", w.id),
            anchor: anchor.clone(),
            text: loc(
                &format!("k_h {:.3}, k_v {:.3}: K_ae = {:.3}, E_ae = {:.1} kN/m ({delta:+.1} kN/m over static)", w.seismic_kh, w.seismic_kv, state.thrust.coefficient, state.thrust.total() / 1000.0),
                &format!("k_h {:.3}, k_v {:.3}: K_ae = {:.3}, E_ae = {:.1} kN/m ({delta:+.1} kN/m gegenüber statisch)", w.seismic_kh, w.seismic_kv, state.thrust.coefficient, state.thrust.total() / 1000.0),
            ),
            tone: tone_of(&bound),
        },
        &anchor,
    );
}
