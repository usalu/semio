//! 🕳️ Ground investigation points as vertical columns at their x/y with the readings as a profile ribbon (EN 1997-2, GI-01, GI-03), and borehole logs of the laboratory series: a narrow stratigraphic column through every layer beside the specimen, a sample disc at the middle of the tested layer coloured by the series utilisation, the depth dimension and the fitted strength (GI-11 stress path, GI-12 direct shear).

use super::checks::catalogue::investigation;
use super::kit::*;
use super::scene::*;
use super::Ctx;

const BOREHOLE_HALF_WIDTH: f64 = 0.15;
const OFFSET: f64 = 0.8;
const SAMPLE_RADIUS: f64 = 0.3;
const SAMPLE_HEIGHT: f64 = 0.12;
const COLUMN_RADIUS: f64 = 0.12;
const RIBBON_WIDTH: f64 = 2.0;
const RIBBON_FIELD: &str = "investigation-reading";

pub fn project(ctx: &Ctx, out: &mut Out) {
    investigations(ctx, out);
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    let cap = if d.investigation_depth > 1e-6 { d.investigation_depth } else { f64::INFINITY };
    for t in &d.lab_tests {
        if !out.own("labTests", &t.id) {
            continue;
        }
        let mut strata = Vec::new();
        let mut cursor = 0.0_f64;
        for layer in &d.layers {
            let bottom = layer.depth_bottom.min(cap);
            if bottom > cursor + 1e-6 {
                let label = loc(&format!("{} ({})", layer.id, layer.soil_type), &format!("{} ({})", layer.id, layer.soil_type));
                strata.push(Stratum { thickness: bottom - cursor, role: soil(layer.soil_type.as_str()), label });
                cursor = bottom;
            }
        }
        if strata.is_empty() {
            continue;
        }
        let pose = layout.pose(&t.id);
        let overall = ctx.binding(&t.id, &[]);
        let half = BOREHOLE_HALF_WIDTH;
        let top = pose.at(OFFSET, 0.0, 0.0);
        out.push(
            Spec::new(
                EntityKind::Solid,
                format!("{}:log", t.id),
                "labTests",
                Geometry::Solid(SolidGeometry { shape: SolidShape::StrataColumn { frame: pose.upright([OFFSET, 0.0, 0.0]), footprint: polygon2(&[[-half, -half], [half, -half], [half, half], [-half, half]]), strata } }),
                soil("borehole"),
                loc(&format!("Borehole log of {}", t.id), &format!("Bohrprofil von {}", t.id)),
                LAYER_GROUND,
            )
            .channel(ColourChannel::Role)
            .detail(format!("{cursor:.2} m deep, {} layers", d.layers.len()), format!("{cursor:.2} m tief, {} Schichten", d.layers.len())),
            &[top.clone(), pose.at(OFFSET, 0.0, -cursor)],
        );
        let Some(layer) = d.layers.iter().find(|l| l.id == t.layer_id) else { continue };
        let mid = 0.5 * (layer.depth_top + layer.depth_bottom.min(cap));
        out.push(
            Spec::new(
                EntityKind::Solid,
                format!("{}:sample", t.id),
                "labTests",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: pose.upright([OFFSET, 0.0, -mid - 0.5 * SAMPLE_HEIGHT]), radius: SAMPLE_RADIUS, height: SAMPLE_HEIGHT } }),
                soil(layer.soil_type.as_str()),
                loc(&format!("Sample of {} at {mid:.2} m", t.id), &format!("Probe von {} bei {mid:.2} m", t.id)),
                LAYER_GROUND,
            )
            .bound(overall.clone())
            .channel(ColourChannel::Utilisation)
            .detail(format!("{} series with {} points in {}", t.kind, t.points.len(), t.layer_id), format!("{}-Reihe mit {} Punkten in {}", t.kind, t.points.len(), t.layer_id)),
            &[pose.at(OFFSET, 0.0, -mid)],
        );
        let (normal, shear): (Vec<f64>, Vec<f64>) = t.points.iter().map(|p| (p.normal_stress, p.shear_stress)).unzip();
        let text = match investigation::lab_strength(t.kind.as_str(), &normal, &shear) {
            Some((phi, c)) => loc(&format!("{}: phi' {phi:.1} deg, c' {:.1} kPa from {} points", t.kind, c / 1000.0, t.points.len()), &format!("{}: phi' {phi:.1} Grad, c' {:.1} kPa aus {} Punkten", t.kind, c / 1000.0, t.points.len())),
            None => loc(&format!("{} series, {} points", t.kind, t.points.len()), &format!("{}-Reihe, {} Punkte", t.kind, t.points.len())),
        };
        let anchor = pose.at(OFFSET + 0.4, 0.0, -mid);
        out.annotate(Annotation::Label { id: format!("{}:log-label", t.id), anchor: anchor.clone(), text, tone: tone_of(&overall) }, &anchor);
        let (from, to) = (pose.at(OFFSET + 0.3, 0.0, 0.0), pose.at(OFFSET + 0.3, 0.0, -mid));
        out.annotate(Annotation::Dimension { id: format!("{}:log-depth", t.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, mid), binding: None }, &from);
    }
}

/// 🕳️ Vertical column of every investigation point from the surface to its depth at its position, and the readings as a ribbon beside it whose width follows the reading (normalised by the largest reading of the point); the column carries the position and depth grips.
fn investigations(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    for i in &d.investigations {
        if !out.own("investigations", &i.id) {
            continue;
        }
        let pose = layout.pose(&i.id);
        let depth = i.depth.max(0.1);
        let extent = ctx.binding(&i.id, &["en1997.gi-01.", "en1997.2.4.2.investigation."]);
        let reading = |r: &super::model::InvestigationReading| format!("{:.2} m: {:.1}", r.depth, r.measurement);
        let last = i.readings.iter().max_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal)).map(reading).unwrap_or_default();
        out.push(
            Spec::new(
                EntityKind::Solid,
                i.id.clone(),
                "investigations",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: pose.upright([0.0, 0.0, -depth]), radius: COLUMN_RADIUS, height: depth } }),
                soil(i.kind.as_str()),
                loc(&format!("Ground investigation {} ({})", i.id, i.kind), &format!("Baugrunderkundung {} ({})", i.id, i.kind)),
                LAYER_GROUND,
            )
            .bound(extent.clone())
            .channel(if extent.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role })
            .handles(ctx.handles_of(&i.id))
            .detail(format!("{} to {depth:.2} m, {} readings, last {last}", i.kind, i.readings.len()), format!("{} bis {depth:.2} m, {} Messwerte, letzter {last}", i.kind, i.readings.len())),
            &[pose.at(0.0, 0.0, 0.0), pose.at(0.0, 0.0, -depth)],
        );
        let mut readings: Vec<&super::model::InvestigationReading> = i.readings.iter().filter(|r| r.depth >= 0.0 && r.depth <= depth + 1e-9).collect();
        readings.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));
        let peak = readings.iter().map(|r| r.measurement).fold(0.0, f64::max);
        if readings.len() < 2 || peak <= 0.0 {
            continue;
        }
        let (mut points, mut values) = (Vec::new(), Vec::new());
        for r in &readings {
            let share = (r.measurement / peak).clamp(0.0, 1.0);
            points.push(pose.at(COLUMN_RADIUS, 0.0, -r.depth));
            points.push(pose.at(COLUMN_RADIUS + RIBBON_WIDTH * share, 0.0, -r.depth));
            values.extend([share, share]);
        }
        let fields = vec![ScalarField { id: RIBBON_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }];
        out.push(
            Spec::new(
                EntityKind::Surface,
                format!("{}:ribbon", i.id),
                "investigations",
                Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: readings.len() as u32, columns: 2, points: points.clone() } } }),
                soil("profile"),
                loc(&format!("Profile ribbon of {}", i.id), &format!("Profilband von {}", i.id)),
                LAYER_GROUND,
            )
            .channel(ColourChannel::Fields { active: RIBBON_FIELD.into(), fields })
            .opacity(0.9)
            .detail(format!("{} readings, widest {peak:.1} = {RIBBON_WIDTH:.1} m", readings.len()), format!("{} Messwerte, größter {peak:.1} = {RIBBON_WIDTH:.1} m", readings.len())),
            &points,
        );
        if !out.legends.iter().any(|l| l.id == RIBBON_FIELD) {
            out.legends.push(Legend { id: RIBBON_FIELD.into(), title: loc("Reading relative to the largest of its point", "Messwert relativ zum größten seines Punktes"), scale: LegendScale::Scalar { field_id: RIBBON_FIELD.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.0 }, unit: QuantityKind::Dimensionless } });
        }
        let anchor = pose.at(COLUMN_RADIUS, 0.0, 0.0);
        out.annotate(Annotation::Label { id: format!("{}:label", i.id), anchor: anchor.clone(), text: loc(&format!("{} {}", i.id, i.kind), &format!("{} {}", i.id, i.kind)), tone: Tone::Neutral }, &anchor);
    }
}
