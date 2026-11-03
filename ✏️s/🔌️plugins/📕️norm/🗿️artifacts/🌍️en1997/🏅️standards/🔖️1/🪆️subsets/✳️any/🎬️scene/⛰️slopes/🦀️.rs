//! ⛰️ Slopes: embankment body, critical slip circle from a deterministic grid search and its slices coloured by local demand.

use super::checks::catalogue::slopes::Circle;
use super::kit::*;
use super::layout::{SLOPE_CREST, SLOPE_TOE};
use super::mechanics as mech;
use super::scene::*;
use super::Ctx;

const SLICE_WARNING: f64 = 0.8;

fn slice_status(ratio: f64) -> scene::CheckStatus {
    scene::utilisation_status(ratio, SLICE_WARNING)
}

/// 🌀️ The proto-circle of a slope: ground, water and weights of its governing layer; centre and radius are searched.
pub fn proto(document: &super::model::En1997Snapshot, slope: &super::model::Slope) -> Option<(Circle, f64, f64)> {
    let layer = document.layers.iter().find(|l| l.id == slope.governing_layer_id).or_else(|| mech::layer_at(&document.layers, 0.5))?;
    let circle = Circle { h: slope.height, beta_deg: slope.angle_deg, xc: 0.0, yc: 0.0, r: 1.0, gamma: layer.gamma, gamma_sat: layer.gamma_prime + mech::GAMMA_W, gwl_level: slope.height - document.groundwater_level, ru: 0.0, q: 0.0 };
    Some((circle, layer.phi_prime_deg.to_radians().tan(), layer.cohesion_effective))
}

/// 🌀️ The circle the scene shows and the handles edit: the user override when the radius is positive, otherwise the critical circle of the grid search; `None` when no sliding mass develops.
pub fn current_circle(slope: &super::model::Slope, circle0: &Circle, tan_phi: f64, c: f64) -> Option<(Circle, f64)> {
    if slope.circle_radius > 1e-6 {
        let fixed = Circle { xc: slope.circle_centre_x, yc: slope.circle_centre_z, r: slope.circle_radius, ..*circle0 };
        return super::checks::catalogue::slopes::bishop(&fixed, tan_phi, c, mech::SLICES).filter(|f| f.is_finite() && *f > 0.0).map(|f| (fixed, f));
    }
    mech::critical_circle(circle0, tan_phi, c)
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for s in &d.slopes {
        if !out.own("slopes", &s.id) {
            continue;
        }
        if s.height <= 1e-6 || s.angle_deg <= 1.0 || s.angle_deg >= 89.0 || s.length <= 1e-6 {
            continue;
        }
        let pose = layout.pose(&s.id);
        let span = mech::slope_span(s);
        let run = mech::slope_run(s.height, s.angle_deg);
        let class = d.layers.iter().find(|l| l.id == s.governing_layer_id).map_or("soil", |l| l.soil_type.as_str());
        let overall = ctx.binding(&s.id, &["en1997.11"]).or_else(|| ctx.binding(&s.id, &[]));
        let frame = pose.section([0.0, span / 2.0, 0.0]);
        out.push(
            Spec::new(
                EntityKind::Solid,
                s.id.clone(),
                "slopes",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: frame.clone(), base: polygon2(&[[-SLOPE_CREST, 0.0], [0.0 - SLOPE_CREST, s.height], [0.0, s.height], [run, 0.0]]), height: span } }),
                soil(class),
                loc(&format!("Slope {}", s.id), &format!("Böschung {}", s.id)),
                LAYER_SLOPES,
            )
            .channel(ColourChannel::Role)
            .bound(overall.clone())
            .opacity(0.9)
            .detail(format!("height {:.2} m, angle {:.1} deg", s.height, s.angle_deg), format!("Höhe {:.2} m, Neigung {:.1} Grad", s.height, s.angle_deg))
            .handles(ctx.handles_of(&s.id)),
            &[pose.at(-SLOPE_CREST, -span / 2.0, 0.0), pose.at(run + SLOPE_TOE, span / 2.0, s.height)],
        );
        let entity = entity_ref(EntityKind::Solid, &s.id, "slopes");
        let crest = pose.at(0.0, 0.0, s.height);
        if let Some(clause) = overall.as_ref().and_then(|b| clause_of(report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", s.id), anchor: entity.clone(), clause }, &crest);
        }
        for (tag, from, to, value, field) in [("height", pose.at(-0.5, 0.0, 0.0), pose.at(-0.5, 0.0, s.height), s.height, "height")] {
            out.annotate(Annotation::Dimension { id: format!("{}:dim-{tag}", s.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, value), binding: Some(FieldBinding::new(format!("slopes[id={}].{field}", s.id), Affine::IDENTITY, "m")) }, &from);
        }
        let Some((circle0, tan_phi, c)) = proto(d, s) else { continue };
        let Some((circle, factor)) = current_circle(s, &circle0, tan_phi, c) else { continue };
        let slices = mech::slip_slices(&circle, mech::SLICES);
        let (Some(first), Some(last)) = (slices.first(), slices.last()) else { continue };
        let theta = |x: f64, z: f64| (z - circle.yc).atan2(x - circle.xc);
        let (ts, te) = (theta(first.x0, first.base[0]), theta(last.x1, last.base[1]));
        let arc = Arc { centre: pose.at(circle.xc, span / 2.0, circle.yc), normal: pose.dir(&Vec3::new(0.0, -1.0, 0.0)), start: pose.at(first.x0, span / 2.0, first.base[0]), sweep: te - ts };
        if arc.sweep.abs() > 1e-6 {
            out.push(
                Spec::new(EntityKind::Surface, format!("{}:slip-circle", s.id), "slopes", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Arc { arc, width: span } }), soil("mechanism"), loc(&format!("Critical slip circle of {}", s.id), &format!("Maßgebender Gleitkreis von {}", s.id)), LAYER_MECHANISMS)
                    .bound(overall.clone())
                    .opacity(0.5)
                    .detail(format!("Bishop F = {:.2}, centre ({:.1}, {:.1}) m, radius {:.1} m", factor, circle.xc, circle.yc, circle.r), format!("Bishop F = {:.2}, Mittelpunkt ({:.1}, {:.1}) m, Radius {:.1} m", factor, circle.xc, circle.yc, circle.r)),
                &[pose.at(first.x0, 0.0, first.base[0]), pose.at(last.x1, 0.0, last.base[1])],
            );
        }
        let check_id = overall.as_ref().map_or_else(|| format!("en1997.11.bishop.{}", s.id), |b| b.check_id.clone());
        let field = projection_kit::check_with(report, &check_id).filter(|c| c.station_field == "slip-slice" && !c.stations.is_empty());
        let (span_lo, span_hi) = (first.x0, last.x1);
        for (i, slice) in slices.iter().enumerate() {
            let measured = field.map(|c| projection_kit::field_along(c, &[(0.5 * (slice.x0 + slice.x1) - span_lo) / (span_hi - span_lo).max(1e-9)])[0]);
            let ratio = measured.unwrap_or_else(|| slice.demand(tan_phi, c)).min(3.0);
            let height = (slice.top[0] - slice.base[0]).max(slice.top[1] - slice.base[1]);
            if slice.width() < 1e-6 || height < 1e-4 {
                continue;
            }
            let ring = [[slice.x0, slice.base[0]], [slice.x1, slice.base[1]], [slice.x1, slice.top[1].max(slice.base[1] + 1e-4)], [slice.x0, slice.top[0].max(slice.base[0] + 1e-4)]];
            out.push(
                Spec::new(EntityKind::Solid, format!("{}:slice-{}", s.id, i + 1), "slopes", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: frame.clone(), base: polygon2(&ring), height: span } }), soil(class), loc(&format!("Slice {} of {}", i + 1, s.id), &format!("Lamelle {} von {}", i + 1, s.id)), LAYER_MECHANISMS)
                    .bound(Some(UtilisationBinding { check_id: check_id.clone(), ratio, status: slice_status(ratio), per_station: None, stale: false }))
                    .opacity(0.8)
                    .detail(format!("weight {:.1} kN/m, base angle {:.1} deg", slice.weight / 1000.0, slice.alpha.to_degrees()), format!("Gewicht {:.1} kN/m, Basiswinkel {:.1} Grad", slice.weight / 1000.0, slice.alpha.to_degrees())),
                &[pose.at(slice.x0, 0.0, slice.base[0]), pose.at(slice.x1, 0.0, slice.top[1])],
            );
        }
        let mid = pose.at(circle.xc, 0.0, circle.yc);
        out.annotate(Annotation::Label { id: format!("{}:factor", s.id), anchor: mid.clone(), text: loc(&format!("F_s (Bishop, characteristic) = {factor:.2}"), &format!("F_s (Bishop, charakteristisch) = {factor:.2}")), tone: if factor < 1.0 { Tone::Error } else if factor < 1.3 { Tone::Warning } else { Tone::Info } }, &mid);
        if let Some((_, f3d, gain, volume)) = super::checks::catalogue::slopes::bishop_extruded(&circle, tan_phi, c, span, mech::SLICES) {
            let far = pose.at(circle.xc, span, circle.yc);
            out.annotate(Annotation::Label { id: format!("{}:factor-3d", s.id), anchor: far.clone(), text: loc(&format!("F_3D (extruded {:.1} m, cohesive ends) = {f3d:.2} (+{gain:.3}), body {volume:.0} m3", span), &format!("F_3D (extrudiert {:.1} m, kohäsive Stirnflächen) = {f3d:.2} (+{gain:.3}), Körper {volume:.0} m3", span)), tone: if f3d < 1.0 { Tone::Error } else if f3d < 1.3 { Tone::Warning } else { Tone::Info } }, &far);
        }
        if factor < 1.0 {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-slip", s.id), at: entity, mode: FailureMode::SlipCircle, extent: None }, &mid);
        }
    }
}
