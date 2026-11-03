//! 🏗️ Piles: pile group cylinders under a cap, shaft resistance per layer, base resistance and the cap load.

use super::checks::catalogue::piles as pile_lines;
use super::checks::lateral_and_seismic as lateral;
use super::kit::*;
use super::mechanics as mech;
use super::scene::*;
use super::Ctx;

const CAP_THICKNESS: f64 = 0.8;
const CAP_BELOW_GROUND: f64 = 0.2;

fn point(out: &mut Out, id: String, name: LocalizedCopy, at: Vec3, up: bool, magnitude: f64, kind: LoadKind, case: &str) {
    let direction = Vec3::new(0.0, 0.0, if up { 1.0 } else { -1.0 });
    out.push(Spec::new(EntityKind::Load, id, "piles", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction, magnitude: force(magnitude), case: load_case(case, kind) }), MaterialRole::Load { kind }, name, LAYER_LOADS), &[at]);
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for p in &d.piles {
        if !out.own("piles", &p.id) {
            continue;
        }
        if p.diameter <= 1e-6 || p.length <= 1e-6 {
            continue;
        }
        let pose = layout.pose(&p.id);
        let overall = ctx.binding(&p.id, &[]);
        let grid = mech::pile_grid(p);
        let mut members = Vec::new();
        for (i, g) in grid.iter().enumerate() {
            let id = format!("{}:pile-{}", p.id, i + 1);
            members.push(entity_ref(EntityKind::Solid, &id, "piles"));
            out.push(
                Spec::new(EntityKind::Solid, id, "piles", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: pose.upright([g[0], g[1], -p.length]), radius: p.diameter / 2.0, height: p.length } }), MaterialRole::Concrete, loc(&format!("Pile {} of {}", i + 1, p.id), &format!("Pfahl {} von {}", i + 1, p.id)), LAYER_PILES)
                    .bound(overall.clone()),
                &[pose.at(g[0], g[1], -p.length), pose.at(g[0], g[1], 0.0)],
            );
        }
        let extent = grid.iter().fold(0.0_f64, |m, g| m.max(g[0].abs()).max(g[1].abs())) + p.diameter;
        let top = CAP_BELOW_GROUND;
        out.push(
            Spec::new(EntityKind::Solid, format!("{}:cap", p.id), "piles", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, 0.0, top - CAP_THICKNESS]), base: polygon2(&[[-extent, -extent], [extent, -extent], [extent, extent], [-extent, extent]]), height: CAP_THICKNESS } }), MaterialRole::Concrete, loc(&format!("Pile cap of {}", p.id), &format!("Pfahlkopfplatte von {}", p.id)), LAYER_PILES)
                .bound(overall.clone()),
            &[pose.at(-extent, -extent, top - CAP_THICKNESS), pose.at(extent, extent, top)],
        );
        out.push(
            Spec::new(EntityKind::Group, p.id.clone(), "piles", Geometry::Group { members }, MaterialRole::Concrete, loc(&format!("Pile group {}", p.id), &format!("Pfahlgruppe {}", p.id)), LAYER_PILES)
                .bound(overall.clone())
                .detail(format!("{} piles, diameter {:.2} m, length {:.2} m", p.count.max(1), p.diameter, p.length), format!("{} Pfähle, Durchmesser {:.2} m, Länge {:.2} m", p.count.max(1), p.diameter, p.length))
                .handles(ctx.handles_of(&p.id)),
            &[],
        );
        if let Some(clause) = overall.as_ref().and_then(|b| clause_of(report, &b.check_id)) {
            let anchor = pose.at(0.0, 0.0, top);
            out.annotate(Annotation::Clause { id: format!("{}:clause", p.id), anchor: entity_ref(EntityKind::Group, &p.id, "piles"), clause }, &anchor);
        }
        let count = f64::from(p.count.max(1));
        let shaft = projection_kit::check_with(report, &format!("en1997.7.6.2.compression.{}", p.id)).filter(|c| c.station_field == "pile-shaft" && !c.stations.is_empty());
        let shafts: Vec<(String, f64, f64)> = match shaft {
            Some(check) => check.stations.iter().filter_map(|st| {
                let z = st.at.first()? * p.length;
                let layer = d.layers.iter().find(|l| z >= l.depth_top && z <= l.depth_bottom)?;
                Some((layer.id.clone(), z, st.values.first()?.value))
            }).collect(),
            None => d.layers.iter().filter_map(|layer| {
                let (a, b) = (layer.depth_top.max(0.0), layer.depth_bottom.min(p.length));
                (b - a > 1e-6).then(|| (layer.id.clone(), (a + b) / 2.0, std::f64::consts::PI * p.diameter * (b - a) * p.unit_shaft_resistance * count))
            }).collect(),
        };
        for (layer_id, z, value) in shafts {
            if value > 1.0 {
                point(out, format!("{}:shaft-{}", p.id, layer_id), loc(&format!("Shaft resistance of {} in {}", p.id, layer_id), &format!("Mantelwiderstand von {} in {}", p.id, layer_id)), pose.at(extent + 0.3, 0.0, -z), true, value, LoadKind::Other, "shaft-resistance");
            }
        }
        let base = mech::pile_base_resistance(p);
        if base > 1.0 {
            point(out, format!("{}:base", p.id), loc(&format!("Base resistance of {}", p.id), &format!("Spitzenwiderstand von {}", p.id)), pose.at(0.0, 0.0, -p.length), true, base, LoadKind::Other, "base-resistance");
        }
        let cap_top = pose.at(0.0, 0.0, top);
        let compression = p.compression_permanent + p.compression_variable;
        if compression > 1.0 {
            point(out, format!("{}:load-n", p.id), loc(&format!("Compression on {}", p.id), &format!("Drucklast auf {}", p.id)), cap_top.clone(), false, compression, LoadKind::Permanent, "compression");
        }
        if p.settling_thickness > 1e-6 {
            if let Some(drag) = projection_kit::check_with(report, &format!("en1997.pile-05.f_neg_k.{}", p.id)) {
                let depth = 0.5 * p.settling_thickness.min(p.length);
                point(out, format!("{}:negative-friction", p.id), loc(&format!("Negative skin friction on {}", p.id), &format!("Negative Mantelreibung auf {}", p.id)), pose.at(-(extent + 0.3), 0.0, -depth), false, drag.computed.value, LoadKind::Permanent, "negative-skin-friction");
            }
        }
        deflection_line(ctx, p, &pose, grid.first().copied().unwrap_or([0.0, 0.0]), out);
        let tension = p.tension_permanent + p.tension_variable;
        if tension > 1.0 {
            point(out, format!("{}:load-t", p.id), loc(&format!("Tension on {}", p.id), &format!("Zuglast auf {}", p.id)), cap_top, true, tension, LoadKind::Variable, "tension");
        }
    }
}

const DEFLECTION_ROWS: usize = 41;
const DRAWN_HEAD_SHIFT: f64 = 0.08;

/// 🪵️ Deflection line of the pile under its lateral head load (PILE-04): a ribbon of the pile width along the Winkler deflection y(x), exaggerated so the head shift reads as a fixed share of the length, coloured by the deflection over its limit or by the moment over its extremum, with the head load glyph and the labelled extremes.
fn deflection_line(ctx: &Ctx, p: &super::model::Pile, pose: &super::layout::Pose, at: [f64; 2], out: &mut Out) {
    let Some(state) = lateral::lateral_pile(p) else { return };
    let d = ctx.input.document;
    let (limit, h) = (lateral::lateral_deflection_limit(d.annex), p.lateral_head_load);
    let scale = (DRAWN_HEAD_SHIFT * p.length / state.head_deflection.abs().max(1e-9)).max(1.0);
    let half = 0.6 * p.diameter;
    let (mut points, mut deflections, mut moments) = (Vec::new(), Vec::new(), Vec::new());
    for row in 0..DEFLECTION_ROWS {
        let x = p.length * row as f64 / (DEFLECTION_ROWS - 1) as f64;
        let y = pile_lines::lateral_deflection(h, state.lambda, p.horizontal_subgrade_modulus, p.diameter, x);
        let m = pile_lines::lateral_moment(h, state.lambda, x);
        for side in [-half, half] {
            points.push(pose.at(at[0] + scale * y, at[1] + side, -x));
            deflections.push(y.abs() / limit);
            moments.push(m.abs() / state.max_moment.max(1e-9));
        }
    }
    let bound = ctx.binding(&p.id, &["en1997.pile-04"]);
    let fields = vec![
        ScalarField { id: "pile-deflection".into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: deflections } },
        ScalarField { id: "pile-moment".into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: moments } },
    ];
    out.push(
        Spec::new(
            EntityKind::Surface,
            format!("{}:lateral-line", p.id),
            "piles",
            Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: DEFLECTION_ROWS as u32, columns: 2, points: points.clone() } } }),
            soil("mechanism"),
            loc(&format!("Lateral deflection line of {}", p.id), &format!("Horizontale Biegelinie von {}", p.id)),
            LAYER_PILES,
        )
        .bound(bound.clone())
        .channel(ColourChannel::Fields { active: "pile-deflection".into(), fields })
        .opacity(0.95)
        .detail(
            format!("head deflection {:.1} mm of {:.0} mm, M_max {:.1} kN m, lambda {:.3} 1/m, drawn x{scale:.0}", state.head_deflection * 1000.0, limit * 1000.0, state.max_moment / 1000.0, state.lambda),
            format!("Kopfverschiebung {:.1} mm von {:.0} mm, M_max {:.1} kN m, lambda {:.3} 1/m, {scale:.0}-fach gezeichnet", state.head_deflection * 1000.0, limit * 1000.0, state.max_moment / 1000.0, state.lambda),
        ),
        &points,
    );
    for (field, title) in [("pile-deflection", loc("Pile deflection over limit", "Pfahlverschiebung zur Grenze")), ("pile-moment", loc("Pile moment over extremum", "Pfahlmoment zum Größtwert"))] {
        if !out.legends.iter().any(|l| l.id == field) {
            out.legends.push(Legend { id: field.into(), title, scale: LegendScale::Scalar { field_id: field.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.2 }, unit: QuantityKind::Dimensionless } });
        }
    }
    let head = pose.at(at[0] + scale * state.head_deflection, at[1], 0.0);
    out.push(
        Spec::new(
            EntityKind::Load,
            format!("{}:load-h", p.id),
            "piles",
            Geometry::Load(LoadGlyph::Point { at: head.clone(), direction: pose.dir(&Vec3::X), magnitude: force(h), case: load_case("lateral-head", LoadKind::Variable) }),
            MaterialRole::Load { kind: LoadKind::Variable },
            loc(&format!("Lateral head load on {}", p.id), &format!("Horizontale Kopflast auf {}", p.id)),
            LAYER_LOADS,
        ),
        std::slice::from_ref(&head),
    );
    let peak = pose.at(at[0] + scale * pile_lines::lateral_deflection(h, state.lambda, p.horizontal_subgrade_modulus, p.diameter, std::f64::consts::FRAC_PI_4 / state.lambda), at[1], -(std::f64::consts::FRAC_PI_4 / state.lambda).min(p.length));
    let text = loc(
        &format!("y0 = {:.1} mm, M_max = {:.1} kN m at {:.2} m{}", state.head_deflection * 1000.0, state.max_moment / 1000.0, std::f64::consts::FRAC_PI_4 / state.lambda, if state.long_pile { "" } else { " (short pile: long-pile solution unconservative)" }),
        &format!("y0 = {:.1} mm, M_max = {:.1} kN m bei {:.2} m{}", state.head_deflection * 1000.0, state.max_moment / 1000.0, std::f64::consts::FRAC_PI_4 / state.lambda, if state.long_pile { "" } else { " (kurzer Pfahl: Lösung für lange Pfähle unsicher)" }),
    );
    out.annotate(Annotation::Label { id: format!("{}:lateral-label", p.id), anchor: peak.clone(), text, tone: tone_of(&bound) }, &peak);
}
