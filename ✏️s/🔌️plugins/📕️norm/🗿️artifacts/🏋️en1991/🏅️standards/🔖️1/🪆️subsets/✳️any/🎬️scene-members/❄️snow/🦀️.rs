//! ❄️ Snow: the shape coefficients μ₁ (and the duo-pitch load arrangements) as area loads on every roof slope, parapet and obstruction walls and the drift triangles behind them (EN 1991-1-3).

use super::envelope::{world, RoofKind, RoofShape};
use super::kit::*;
use super::model::{En1991Snapshot, RoofArea};
use super::scene::*;
use super::{checks, fields, handles, Ctx};
use checks::part_1_3;
use fields::snow::Drift;

const LIFT: f64 = 0.05;
const PARAPET_THICKNESS: f64 = 0.2;
const OBSTRUCTION_DEPTH: f64 = 0.5;
const OBSTRUCTION_WIDTH: f64 = 8.0;
const OBSTRUCTION_POSITION: f64 = 0.3;

/// ❄️ Characteristic ground snow load s_k in Pa of the site, `None` for an unknown snow zone.
pub fn sk_of(d: &En1991Snapshot) -> Option<f64> {
    part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk)
}

struct Roof<'a> {
    roof: &'a RoofArea,
    shape: RoofShape,
    pose: &'a super::layout::Pose,
    sk: f64,
    annex: super::norm::AnnexChoice,
    bound: Option<UtilisationBinding>,
    grips: Vec<String>,
}

impl Roof<'_> {
    fn s(&self, mu: f64) -> f64 {
        part_1_3::roof_snow_pa(self.sk, mu, self.roof.c_e, self.roof.c_t)
    }

    fn plan_points(&self, plan: &[[f64; 2]]) -> Vec<Vec3> {
        world(self.pose, &plan.iter().map(|p| self.shape.lift(*p, LIFT)).collect::<Vec<_>>())
    }

    fn area(&self, id: String, plan: &[[f64; 2]], intensity: f64, name: LocalizedCopy, detail: LocalizedCopy, layer: &'static str, out: &mut Out) -> Vec<Vec3> {
        let points = self.plan_points(plan);
        out.push(
            Spec::new(
                EntityKind::Load,
                id,
                "roofs",
                Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: self.pose.dir(&mul(&Vec3::Z, -1.0)), intensity: pressure(intensity), case: case(&format!("snow-{}", self.roof.id), LoadKind::Snow) }),
                load(LoadKind::Snow),
                name,
                layer,
            )
            .bound(self.bound.clone())
            .channel(ColourChannel::Role)
            .opacity(0.8)
            .detail(detail)
            .handles(self.grips.clone()),
            &points,
        );
        points
    }
}

fn arrangements(kind: RoofKind) -> Vec<(&'static str, &'static str, Vec<f64>)> {
    match kind {
        RoofKind::Duo => vec![("", LAYER_SNOW, vec![1.0, 1.0]), ("alt1-", LAYER_SNOW_ALT, vec![0.5, 1.0]), ("alt2-", LAYER_SNOW_ALT, vec![1.0, 0.5])],
        RoofKind::Hip => vec![("", LAYER_SNOW, vec![1.0; 4])],
        RoofKind::Flat | RoofKind::Mono => vec![("", LAYER_SNOW, vec![1.0])],
    }
}

fn slopes_and_labels(r: &Roof, out: &mut Out) {
    let mu1 = part_1_3::mu_1(r.roof.pitch_deg);
    for (prefix, layer, factors) in arrangements(r.shape.kind) {
        for ((tag, plan), factor) in r.shape.slopes().into_iter().zip(factors) {
            let s = r.s(mu1 * factor);
            if s <= 1e-9 {
                continue;
            }
            let arrangement = if prefix.is_empty() { loc("undrifted", "unverweht") } else { loc("alternative arrangement", "alternative Anordnung") };
            let name = bi(|de| format!("{} {} {}", if de { "Schnee" } else { "Snow" }, r.roof.id, tag));
            let detail = bi(|de| format!("μ = {}, s = {} kN/m², {}", num(mu1 * factor, 2, de), num(s / 1000.0, 3, de), if de { &arrangement.de } else { &arrangement.en }));
            let points = r.area(format!("{}:{prefix}{tag}", r.roof.id), &plan, s, name, detail, layer, out);
            if prefix.is_empty() {
                out.label(format!("{}:{tag}:mu", r.roof.id), centroid(&points), bi(|de| format!("μ₁ = {}, s = {} kN/m²", num(mu1 * factor, 2, de), num(s / 1000.0, 2, de))), tone_of(&r.bound));
            }
        }
    }
}

fn drift(r: &Roof, tag: &str, x_start: f64, length: f64, mu: f64, out: &mut Out) {
    if length <= 1e-6 {
        return;
    }
    let b = r.shape.b;
    let peak = r.s(mu.max(part_1_3::mu_1(r.roof.pitch_deg)));
    let base = r.s(part_1_3::mu_1(r.roof.pitch_deg));
    let plan = rect2(x_start, 0.0, x_start + length, b);
    let name = bi(|de| format!("{} {}", if de { "Schneeverwehung" } else { "Snow drift" }, tag));
    let detail = bi(|de| format!("μ = {}, l_s = {} m, s = {} kN/m²", num(mu, 2, de), num(length, 2, de), num(peak / 1000.0, 3, de)));
    r.area(format!("{}:drift-{tag}", r.roof.id), &plan, peak, name.clone(), detail.clone(), LAYER_SNOW, out);
    let from = r.shape.lift([x_start, b / 2.0], LIFT);
    let to = r.shape.lift([x_start + length, b / 2.0], LIFT);
    let (a, e) = (r.pose.at(from.x, from.y, from.z), r.pose.at(to.x, to.y, to.z));
    let (start, end) = if tag == "obstruction" { (peak, base) } else { (base, peak) };
    out.push(
        Spec::new(
            EntityKind::Load,
            format!("{}:drift-{tag}-profile", r.roof.id),
            "roofs",
            Geometry::Load(LoadGlyph::Line { path: polyline(&[a.clone(), e.clone()]), direction: r.pose.dir(&mul(&Vec3::Z, -1.0)), profile: LoadProfile::Linear { start: pressure(start), end: pressure(end) }, case: case(&format!("snow-{}", r.roof.id), LoadKind::Snow) }),
            load(LoadKind::Snow),
            name,
            LAYER_SNOW,
        )
        .bound(r.bound.clone())
        .detail(detail)
        .handles(r.grips.clone()),
        &[a, e],
    );
}

fn parapet(r: &Roof, out: &mut Out) -> Option<Drift> {
    let height = r.roof.parapet_height;
    let (d, b) = (r.shape.d, r.shape.b);
    let loop_points = world(r.pose, &[[0.0, 0.0], [d, 0.0], [d, b], [0.0, b], [0.0, 0.0]].iter().map(|p| r.shape.lift(*p, 0.0)).collect::<Vec<_>>());
    out.push(
        Spec::new(
            EntityKind::Solid,
            format!("{}:parapet", r.roof.id),
            "roofs",
            Geometry::Solid(SolidGeometry { shape: SolidShape::WallAssembly { path: Polyline3 { points: loop_points.clone() }, height, layers: vec![WallLayer { thickness: PARAPET_THICKNESS, role: MaterialRole::Concrete, label: loc("Parapet", "Attika") }], openings: Vec::new() } }),
            MaterialRole::Concrete,
            bi(|de| format!("{} {}", if de { "Attika" } else { "Parapet" }, r.roof.id)),
            LAYER_SNOW,
        )
        .detail(bi(|de| format!("{} {} m", if de { "Höhe" } else { "height" }, num(height, 2, de))))
        .handles(r.grips.clone()),
        &loop_points,
    );
    let limit = r.shape.planar_limit()?;
    let (_, mu, length) = part_1_3::obstruction_drift(height, r.sk, r.annex);
    let start = (r.shape.d - length).max(if r.shape.kind == RoofKind::Duo { limit } else { 0.0 });
    drift(r, "parapet", start, r.shape.d - start, mu, out);
    Some(Drift { start, length: r.shape.d - start, peak: mu, rising: true })
}

fn obstruction(r: &Roof, out: &mut Out) -> Option<Drift> {
    let height = r.roof.drift_obstruction_height;
    let limit = r.shape.planar_limit()?;
    let x = OBSTRUCTION_POSITION * limit;
    let width = OBSTRUCTION_WIDTH.min(r.shape.b);
    let y0 = (r.shape.b - width) / 2.0;
    let low = r.shape.z(x - OBSTRUCTION_DEPTH, r.shape.b / 2.0).min(r.shape.z(x, r.shape.b / 2.0));
    let spread = (r.shape.z(x, r.shape.b / 2.0) - r.shape.z(x - OBSTRUCTION_DEPTH, r.shape.b / 2.0)).abs();
    let frame = r.pose.upright([x - OBSTRUCTION_DEPTH, y0, low]);
    let corners = [r.pose.at(x - OBSTRUCTION_DEPTH, y0, low), r.pose.at(x, y0 + width, low + height + spread)];
    out.push(
        Spec::new(
            EntityKind::Solid,
            format!("{}:obstruction", r.roof.id),
            "roofs",
            Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame, base: polygon2(&rect2(0.0, 0.0, OBSTRUCTION_DEPTH, width)), height: height + spread } }),
            MaterialRole::Concrete,
            bi(|de| format!("{} {}", if de { "Dachaufbau" } else { "Roof obstruction" }, r.roof.id)),
            LAYER_SNOW,
        )
        .detail(bi(|de| format!("{} {} m", if de { "Höhe" } else { "height" }, num(height, 2, de))))
        .handles(r.grips.clone()),
        &corners,
    );
    let top = r.pose.at(x, r.shape.b / 2.0, low + height);
    let foot = r.pose.at(x, r.shape.b / 2.0, low);
    out.annotate(
        Annotation::Dimension { id: format!("{}:obstruction-height", r.roof.id), from: foot, to: top.clone(), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, height), binding: Some(FieldBinding::new(format!("roofs[id={}].driftObstructionHeight", r.roof.id), Affine::IDENTITY, "m")) },
        &top,
    );
    let (_, mu, length) = part_1_3::obstruction_drift(height, r.sk, r.annex);
    drift(r, "obstruction", x, length.min(limit - x), mu, out);
    Some(Drift { start: x, length: length.min(limit - x), peak: mu, rising: false })
}

fn profile(r: &Roof, drifts: &[Drift], out: &mut Out) {
    const STEPS: usize = 12;
    let points: Vec<Vec3> = (0..=STEPS).map(|i| r.shape.lift([r.shape.d * i as f64 / STEPS as f64, r.shape.b / 2.0], 2.0 * LIFT)).map(|p| r.pose.at(p.x, p.y, p.z)).collect();
    let samples = fields::snow::samples(r.shape.d, STEPS, part_1_3::mu_1(r.roof.pitch_deg), drifts, |mu| r.s(mu));
    out.push(
        Spec::new(
            EntityKind::Load,
            format!("{}:profile", r.roof.id),
            "roofs",
            Geometry::Load(LoadGlyph::Line { path: polyline(&points), direction: r.pose.dir(&mul(&Vec3::Z, -1.0)), profile: LoadProfile::Sampled { samples }, case: case(&format!("snow-{}", r.roof.id), LoadKind::Snow) }),
            load(LoadKind::Snow),
            loc("Snow load profile s(x)", "Schneelastprofil s(x)"),
            LAYER_SNOW,
        )
        .bound(r.bound.clone())
        .detail(bi(|de| format!("s = μ s_k C_e C_t {}", if de { "entlang des Dachschnitts" } else { "along the roof section" })))
        .handles(r.grips.clone()),
        &points,
    );
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let Some(sk) = sk_of(d) else { return };
    for (index, roof) in d.roofs.iter().enumerate() {
        if !out.own(&roof.id) {
            continue;
        }
        let r = Roof { roof, shape: RoofShape::of(d, Some(roof)), pose: layout.roof(index), sk, annex: d.annex, bound: ctx.binding(&roof.id, &[]), grips: handles::ids(d, layout, &roof.id) };
        let mut drifts = Vec::new();
        slopes_and_labels(&r, out);
        if roof.has_parapet && roof.parapet_height > 1e-6 {
            drifts.extend(parapet(&r, out));
        }
        if roof.drift_obstruction_height > 1e-6 {
            drifts.extend(obstruction(&r, out));
        }
        profile(&r, &drifts, out);
        let ridge = r.shape.lift([r.shape.d / 2.0, r.shape.b / 2.0], 1.2);
        out.label(format!("{}:sk", roof.id), r.pose.at(ridge.x, ridge.y, ridge.z), bi(|de| format!("s_k = {} kN/m² ({} {})", num(sk / 1000.0, 2, de), if de { "Zone" } else { "zone" }, d.snow_zone)), Tone::Info);
    }
}
