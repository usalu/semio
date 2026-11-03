//! 📉️ Settlement: the superposed settlement dish of all footings and a settlement label per footing.

use super::kit::*;
use super::mechanics as mech;
use super::scene::*;
use super::tracks;
use super::Ctx;

pub const EXAGGERATION: f64 = 25.0;
pub const COLUMNS: u32 = 21;
pub const ROWS: u32 = 13;
const LIFT: f64 = 0.01;

/// 📉️ Settlement in metres of `footing` at a world point, using its pose.
pub fn settlement_of(ctx: &Ctx, footing: &super::model::SpreadFoundation, x: f64, y: f64) -> f64 {
    let pose = ctx.layout.pose(&footing.id);
    let (dx, dy) = (x - pose.origin.x, y - pose.origin.y);
    let (s, c) = (-pose.yaw).sin_cos();
    mech::settlement_at(footing, &ctx.input.document.layers, mech::net_pressure(footing, &ctx.input.document.layers), dx * c - dy * s, dx * s + dy * c)
}

/// 📉️ Elastic settlement in metres of `footing` at a world point, using its pose.
pub fn elastic_settlement_of(ctx: &Ctx, footing: &super::model::SpreadFoundation, x: f64, y: f64) -> f64 {
    let pose = ctx.layout.pose(&footing.id);
    let (dx, dy) = (x - pose.origin.x, y - pose.origin.y);
    let (s, c) = (-pose.yaw).sin_cos();
    mech::elastic_settlement_at(footing, &ctx.input.document.layers, dx * c - dy * s, dx * s + dy * c)
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let d = ctx.input.document;
    let footings: Vec<_> = d.footings.iter().filter(|f| f.width > 1e-6 && f.length > 1e-6).collect();
    if footings.is_empty() {
        return;
    }
    let (mut lo, mut hi) = ([f64::MAX; 2], [f64::MIN; 2]);
    for f in &footings {
        let o = ctx.layout.pose(&f.id).origin;
        let r = 1.5 * f.width.max(f.length);
        lo = [lo[0].min(o.x - r), lo[1].min(o.y - r)];
        hi = [hi[0].max(o.x + r), hi[1].max(o.y + r)];
    }
    let mut points = Vec::new();
    let mut depths = Vec::new();
    let mut deepest = 0.0_f64;
    for j in 0..ROWS {
        for i in 0..COLUMNS {
            let (x, y) = (lo[0] + (hi[0] - lo[0]) * f64::from(i) / f64::from(COLUMNS - 1), lo[1] + (hi[1] - lo[1]) * f64::from(j) / f64::from(ROWS - 1));
            let w: f64 = footings.iter().map(|f| settlement_of(ctx, f, x, y)).sum();
            deepest = deepest.max(w);
            depths.push(w);
            points.push(v3(x, y, LIFT - EXAGGERATION * w));
        }
    }
    let moduli: Vec<f64> = d.layers.iter().map(|l| l.oedometric_modulus).collect();
    let lab_cv = tracks::laboratory_cv(&d.lab_tests).map(|(cv, _)| cv);
    let mut fields = Vec::new();
    let mut stages = Vec::new();
    if let Some(((mut earlier, track), stops)) = footings.first().and_then(|f| tracks::consolidation_track(f, &moduli, &depths, lab_cv).zip(tracks::stops(f, &moduli, lab_cv))) {
        fields.append(&mut earlier);
        out.track(track);
        stages = stops;
    }
    fields.push(ScalarField { id: "settlement".into(), unit: QuantityKind::Length, samples: FieldSamples::Vertex { values: depths.clone() } });
    let anchor = points.first().cloned().unwrap_or(Vec3::ZERO);
    out.push(
        Spec::new(EntityKind::Surface, "settlement-field", "@footings", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: ROWS, columns: COLUMNS, points: points.clone() } } }), soil("settlement"), loc("Settlement dish", "Setzungsmulde"), LAYER_SETTLEMENT)
            .channel(ColourChannel::Fields { active: "settlement".into(), fields: fields.clone() })
            .opacity(0.8)
            .detail(format!("maximum {:.1} mm, drawn exaggerated x{EXAGGERATION:.0}", deepest * 1000.0), format!("Maximum {:.1} mm, {EXAGGERATION:.0}-fach überhöht dargestellt", deepest * 1000.0)),
        &points,
    );
    let level: Vec<Vec3> = points.iter().map(|p| v3(p.x, p.y, LIFT)).collect();
    out.push(
        Spec::new(EntityKind::Surface, "settlement-field.ghost", "@footings", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: ROWS, columns: COLUMNS, points: level.clone() } } }), MaterialRole::Neutral, loc("Undeformed ground ghost", "Geist des unverformten Bodens"), LAYER_SETTLEMENT)
            .opacity(0.18)
            .detail(format!("original ground level above the settlement dish, maximum settlement {:.1} mm drawn exaggerated x{EXAGGERATION:.0}", deepest * 1000.0), format!("Ursprüngliche Geländehöhe über der Setzungsmulde, Maximalsetzung {:.1} mm {EXAGGERATION:.0}-fach überhöht dargestellt", deepest * 1000.0)),
        &level,
    );
    let ids = tracks::field_ids(&stages);
    for (i, (d, u)) in stages.iter().enumerate().take(stages.len().saturating_sub(1)) {
        let trough: Vec<Vec3> = points.iter().zip(&depths).map(|(p, w)| v3(p.x, p.y, LIFT - EXAGGERATION * w * u)).collect();
        out.push(
            Spec::new(EntityKind::Surface, format!("settlement-field.stage{i}"), "@footings", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: ROWS, columns: COLUMNS, points: trough.clone() } } }), soil("settlement"), loc(&format!("Settlement trough after {d:.0} d"), &format!("Setzungsmulde nach {d:.0} d")), STAGE_LAYERS[i].0)
                .channel(ColourChannel::Fields { active: ids[i].clone(), fields: fields.clone() })
                .opacity(0.8)
                .detail(format!("{:.0} % of the final settlement ({:.1} mm), drawn exaggerated x{EXAGGERATION:.0}", u * 100.0, u * deepest * 1000.0), format!("{:.0} % der Endsetzung ({:.1} mm), {EXAGGERATION:.0}-fach überhöht dargestellt", u * 100.0, u * deepest * 1000.0)),
            &trough,
        );
    }
    out.legends.push(Legend { id: "settlement".into(), title: loc("Settlement", "Setzung"), scale: LegendScale::Scalar { field_id: "settlement".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: (deepest * 1000.0).max(1.0) }, unit: QuantityKind::Length } });
    let label_at = anchor.clone();
    out.annotate(Annotation::Label { id: "settlement-field:label".into(), anchor, text: loc(&format!("Settlement, exaggerated x{EXAGGERATION:.0}"), &format!("Setzung, {EXAGGERATION:.0}-fach überhöht")), tone: Tone::Neutral }, &label_at);
    for (i, a) in footings.iter().enumerate() {
        for b in footings.iter().skip(i + 1) {
            let (pa, pb) = (ctx.layout.pose(&a.id).origin, ctx.layout.pose(&b.id).origin);
            let length = (pb.x - pa.x).hypot(pb.y - pa.y);
            if length < 1e-6 {
                continue;
            }
            let beta = (settlement_of(ctx, b, pb.x, pb.y) - settlement_of(ctx, a, pa.x, pa.y)).abs() / length;
            let mid = v3(0.5 * (pa.x + pb.x), 0.5 * (pa.y + pb.y), LIFT);
            let limit = super::norm::ndp_number("ndp.en1997.sls.angular-distortion-limit", d.annex);
            let text = format!("{} – {}: beta = 1/{:.0}", a.id, b.id, 1.0 / beta.max(1e-9));
            out.annotate(Annotation::Label { id: format!("{}-{}:angular", a.id, b.id), anchor: mid.clone(), text: loc(&text, &text), tone: if beta > limit { Tone::Error } else if beta > 0.8 * limit { Tone::Warning } else { Tone::Info } }, &mid);
        }
    }
    for f in footings {
        let pose = ctx.layout.pose(&f.id);
        let s = settlement_of(ctx, f, pose.origin.x, pose.origin.y);
        let ratio = if f.settlement_limit > 0.0 { s / f.settlement_limit } else { 0.0 };
        let at = pose.at(0.0, 0.0, 0.8);
        out.annotate(
            Annotation::Label {
                id: format!("{}:settlement", f.id),
                anchor: at.clone(),
                text: loc(&format!("s = {:.1} mm (limit {:.1} mm)", s * 1000.0, f.settlement_limit * 1000.0), &format!("s = {:.1} mm (Grenzwert {:.1} mm)", s * 1000.0, f.settlement_limit * 1000.0)),
                tone: super::scene::utilisation_tone(ratio),
            },
            &at,
        );
    }
}
