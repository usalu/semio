//! 🕳️ Punching: flat-slab columns as solids and the control perimeters `u0` (column face) and `u1` (2d) as translucent rings at slab mid-height, reduced at free edges; the effective-depth dimension is bound to the document.

use super::checks;
use super::handle_set::*;
use super::kit::*;
use super::layout::{Layout, COLUMN_UNDER_SLAB};
use super::model::{MemberKind, PunchingBetaMethod, RcMember};
use super::scene::*;
use super::SceneInput;
use std::f64::consts::FRAC_PI_2;

const ARC_SEGMENTS: usize = 8;
const LIFT: f64 = 0.002;
const PERIMETER_FIELD: &str = "punching-perimeter";

fn rounded_ring(cx: f64, cy: f64, hw: f64, hd: f64, k: f64) -> Vec<(f64, f64)> {
    if k <= 0.0 {
        return vec![(cx - hw, cy - hd), (cx + hw, cy - hd), (cx + hw, cy + hd), (cx - hw, cy + hd)];
    }
    let corners = [(cx + hw, cy + hd, 0.0), (cx - hw, cy + hd, FRAC_PI_2), (cx - hw, cy - hd, 2.0 * FRAC_PI_2), (cx + hw, cy - hd, 3.0 * FRAC_PI_2)];
    corners
        .iter()
        .flat_map(|&(x, y, start)| (0..=ARC_SEGMENTS).map(move |i| {
            let a = start + FRAC_PI_2 * i as f64 / ARC_SEGMENTS as f64;
            (x + k * a.cos(), y + k * a.sin())
        }))
        .collect()
}

fn clip(ring: &[(f64, f64)], inside: impl Fn((f64, f64)) -> f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..ring.len() {
        let (p, q) = (ring[i], ring[(i + 1) % ring.len()]);
        let (fp, fq) = (inside(p), inside(q));
        if fp >= 0.0 {
            out.push(p);
        }
        if (fp >= 0.0) != (fq >= 0.0) {
            let t = fp / (fp - fq);
            out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
    }
    out
}

fn length(ring: &[(f64, f64)]) -> f64 {
    (0..ring.len()).map(|i| (ring[(i + 1) % ring.len()].0 - ring[i].0).hypot(ring[(i + 1) % ring.len()].1 - ring[i].1)).sum()
}

fn column_centre(m: &RcMember, cw: f64, cd: f64) -> (f64, f64) {
    let (l, w) = (m.span, m.span);
    match m.punching.as_ref().map_or(semio_s_artifact_norm_en1992_model::ColumnPosition::Interior, |p| p.column_position) {
        semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => (l - cw / 2.0, 0.0),
        semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => (l - cw / 2.0, w / 2.0 - cd / 2.0),
        _ => (l / 2.0, 0.0),
    }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in input.document.members.iter().filter(|m| m.kind == MemberKind::FlatSlab) {
        if !out.wants(&m.id) {
            continue;
        }
        let (Some(pose), Some(p)) = (layout.member(&m.id), m.punching.as_ref()) else { continue };
        if p.column_width <= 0.0 || p.column_depth <= 0.0 || m.span <= 0.0 {
            continue;
        }
        let f = &pose.frame;
        let path = format!("members[id={}].punching", m.id);
        let (cx, cy) = column_centre(m, p.column_width, p.column_depth);
        let (hw, hd) = (p.column_width / 2.0, p.column_depth / 2.0);
        let binding = binding(input.report, &m.id, &["en1992.6.4.", "en1992.p-01."]);
        let field_check = input.report.checks.iter().find(|c| c.id == format!("en1992.6.4.punching.{}", m.id) && c.station_field == PERIMETER_FIELD && !c.stations.is_empty());
        let label = |en: &str, de: &str| loc(&format!("{} {en}", m.label_en), &format!("{} {de}", m.label_de));
        let base_centre = at(f, cx, cy, -m.height / 2.0 - COLUMN_UNDER_SLAB);
        let column_ref = entity_ref(EntityKind::Solid, &format!("{}:column", m.id), &path);
        let grips = [
            punching_grip(&m.id, &column_ref, "column-width", f.y.clone(), at(f, cx, cy + hd, -m.height / 2.0), "columnWidth", p.column_width, ("width", "Breite")),
            punching_grip(&m.id, &column_ref, "column-depth", f.x.clone(), at(f, cx + hw, cy, -m.height / 2.0), "columnDepth", p.column_depth, ("depth", "Tiefe")),
        ];
        let mut ids: Vec<String> = grips.iter().map(|g| g.id.clone()).collect();
        out.handles.extend(grips.into_iter().map(Grip::build));
        let moment_top = m.height / 2.0 + 0.05;
        let beta_handle = projection_kit::handle_kit::choice_handle(&column_ref, "beta-method", at(f, cx + hw, cy + hd, moment_top), format!("{path}.betaMethod"), &beta_method_options(), loc(&format!("Load increase factor method of {}", m.label_en), &format!("Methode des Lasterhöhungsfaktors von {}", m.label_de)));
        ids.push(beta_handle.id.clone());
        out.handles.push(beta_handle);
        let mut moment_glyphs = Vec::new();
        if p.beta_method == PunchingBetaMethod::UnbalancedMoments {
            for (key, value, axis, drag, field, name) in [("moment-c1", p.moment_c1, f.y.clone(), f.x.clone(), "momentC1", "M_Ed,1"), ("moment-c2", p.moment_c2, f.x.clone(), f.y.clone(), "momentC2", "M_Ed,2")] {
                let (dx, dy) = if key == "moment-c1" { (hw + 0.15, 0.0) } else { (0.0, hd + 0.15) };
                let grip = punching_moment_grip(&m.id, &column_ref, key, drag, at(f, cx + dx, cy + dy, moment_top), field, value, (name, name));
                ids.push(grip.id.clone());
                out.handles.push(grip.build());
                if value != 0.0 {
                    moment_glyphs.push((key, value, axis, name));
                }
            }
        }
        let column = Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: base_centre.clone(), ..f.clone() }, base: Polygon2::solid(vec![Vec2::new(-hw, -hd), Vec2::new(hw, -hd), Vec2::new(hw, hd), Vec2::new(-hw, hd)]), height: COLUMN_UNDER_SLAB } });
        out.push(Spec::new(EntityKind::Solid, format!("{}:column", m.id), path.clone(), column, MaterialRole::Concrete, LAYER_CONCRETE, label("column", "Stütze"), label(&format!("column {} x {} mm, {}", mm(p.column_width), mm(p.column_depth), format!("{:?}", p.column_position).to_lowercase()), &format!("Stütze {} x {} mm, {}", mm(p.column_width), mm(p.column_depth), format!("{:?}", p.column_position).to_lowercase()))).translucent(0.5).with_handles(ids), &[base_centre]);
        for (key, value, axis, name) in moment_glyphs {
            let glyph = LoadGlyph::Moment { at: at(f, cx, cy, moment_top), axis, magnitude: Quantity::new(QuantityKind::Moment, value), case: LoadCaseRef { id: format!("{}-{key}", m.id), kind: LoadKind::Other, psi: None } };
            let spec = Spec::new(EntityKind::Load, format!("{}:punching-{key}", m.id), path.clone(), Geometry::Load(glyph), MaterialRole::Load { kind: LoadKind::Other }, LAYER_LOADS, label(&format!("unbalanced column moment {name}"), &format!("Stützenmoment {name}")), label(&format!("unbalanced column moment {name} = {:.1} kNm", value / 1e3), &format!("Stützenmoment {name} = {:.1} kNm", value / 1e3))).bound(binding.clone());
            out.push(spec, &[at(f, cx, cy, moment_top)]);
        }
        let d = m.effective_depth.max(1e-3);
        let column_position = match p.column_position {
            semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => checks::punching_and_struts::Position::Edge,
            semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => checks::punching_and_struts::Position::Corner,
            _ => checks::punching_and_struts::Position::Interior,
        };
        let rects: Vec<checks::punching_and_struts::OpeningRect> = m.openings.iter().map(|o| checks::punching_and_struts::OpeningRect { x: o.x, y: o.y, width: o.width, depth: o.depth }).collect();
        let wedges = checks::punching_and_struts::effective_wedges(p.column_width, p.column_depth, d, &rects).unwrap_or_default();
        let edge_x = m.span;
        let edge_y = m.span / 2.0;
        let mut perimeters = vec![("u0", 0.0, "column perimeter u0", "Stützenumfang u0"), ("u1", 2.0 * d, "control perimeter u1 at 2d", "Rundschnitt u1 im Abstand 2d")];
        let reach = m.stud_reach().map(|outer| outer + checks::ndp(input.document.annex, "ndp.en1992.punch-k-out") * d);
        if let Some(uout) = input.report.checks.iter().find(|c| c.id == format!("en1992.6.4.uout.{}", m.id)).zip(reach) {
            perimeters.push(("u-out", uout.0.utilization * uout.1, "outer control perimeter u_out", "Äußerer Rundschnitt u_out"));
        }
        for (key, k, en, de) in perimeters {
            let mut ring = rounded_ring(cx, cy, hw, hd, k);
            ring = match p.column_position {
                semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => clip(&ring, |q| edge_x - q.0),
                semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => clip(&clip(&ring, |q| edge_x - q.0), |q| edge_y - q.1),
                _ => ring,
            };
            let cut = key != "u0" && !wedges.is_empty();
            if cut {
                ring = cut_ring(&ring, (cx, cy), &wedges);
            }
            if ring.len() < 3 {
                continue;
            }
            let points: Vec<Vec3> = ring.iter().map(|&(x, y)| at(f, x, y, LIFT)).collect();
            let u = if cut { let (total, lost) = checks::punching_and_struts::perimeter_lengths(column_position, p.column_width, p.column_depth, k, &wedges); total - lost } else { length(&ring) };
            let mut ent = Spec::new(EntityKind::Surface, format!("{}:punching-{key}", m.id), path.clone(), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: ring3(points.clone()) } }), MaterialRole::Neutral, LAYER_PUNCHING, label(en, de), label(&format!("{en}, length {:.2} m", u), &format!("{de}, Länge {:.2} m", u))).bound(binding.clone()).translucent(0.45).ghosted();
            if key == "u1" {
                if let Some(check) = field_check {
                    let local: Vec<[f64; 3]> = ring.iter().map(|&(x, y)| [x, y, 0.0]).collect();
                    let values = projection_kit::field_at_points(check, &local);
                    ent = ent.bound(Some(projection_kit::binding_of_check(check))).channel(ColourChannel::Fields { active: PERIMETER_FIELD.into(), fields: vec![ScalarField { id: PERIMETER_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }] });
                    field_legend(&mut out.legends, PERIMETER_FIELD, loc("Shear utilisation on the control perimeter", "Schubausnutzung am Rundschnitt"));
                }
            }
            out.push(ent, &points);
        }
        for o in &m.openings {
            let spec_path = format!("members[id={}].openings[id={}]", m.id, o.id);
            let (ox, oy) = (cx + o.x, cy + o.y);
            let (ow, od) = (o.width / 2.0, o.depth / 2.0);
            let bottom = at(f, ox, oy, -m.height / 2.0 - LIFT);
            let opening_ref = entity_ref(EntityKind::Solid, &format!("{}:opening-{}", m.id, o.id), &spec_path);
            let anchor_z = m.height / 2.0 + LIFT;
            let key = |name: &str| format!("opening-{}-{name}", o.id);
            let grips = [
                length_grip(&m.id, &opening_ref, &m.label_en, &key("x"), at(f, ox, oy, anchor_z), f.x.clone(), &format!("openings[id={}].x", o.id), o.x, 1.0, (-m.span, m.span), ("opening x", "Öffnung x"), along),
                length_grip(&m.id, &opening_ref, &m.label_en, &key("y"), at(f, ox, oy, anchor_z), f.y.clone(), &format!("openings[id={}].y", o.id), o.y, 1.0, (-m.span, m.span), ("opening y", "Öffnung y"), along),
                length_grip(&m.id, &opening_ref, &m.label_en, &key("width"), at(f, ox + ow, oy, anchor_z), f.x.clone(), &format!("openings[id={}].width", o.id), o.width, 2.0, (0.05, m.span), ("opening width", "Öffnungsbreite"), face),
                length_grip(&m.id, &opening_ref, &m.label_en, &key("depth"), at(f, ox, oy + od, anchor_z), f.y.clone(), &format!("openings[id={}].depth", o.id), o.depth, 2.0, (0.05, m.span), ("opening depth", "Öffnungstiefe"), face),
            ];
            let handle_ids: Vec<String> = grips.iter().map(|g| g.id.clone()).collect();
            out.handles.extend(grips.into_iter().map(Grip::build));
            let cutout = Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: bottom.clone(), ..f.clone() }, base: Polygon2::solid(vec![Vec2::new(-ow, -od), Vec2::new(ow, -od), Vec2::new(ow, od), Vec2::new(-ow, od)]), height: m.height + 2.0 * LIFT } });
            let clear = checks::punching_and_struts::clear_distance(p.column_width, p.column_depth, &checks::punching_and_struts::OpeningRect { x: o.x, y: o.y, width: o.width, depth: o.depth });
            let counts = clear <= 6.0 * d;
            out.push(Spec::new(EntityKind::Solid, format!("{}:opening-{}", m.id, o.id), spec_path.clone(), cutout, MaterialRole::Neutral, LAYER_PUNCHING, label(&format!("opening {}", o.id), &format!("Öffnung {}", o.id)), label(&format!("opening {} x {} mm, clear distance {} mm ({})", mm(o.width), mm(o.depth), mm(clear), if counts { "within 6 d: tangent rule applies" } else { "beyond 6 d: no effect" }), &format!("Öffnung {} x {} mm, lichter Abstand {} mm ({})", mm(o.width), mm(o.depth), mm(clear), if counts { "innerhalb 6 d: Tangentenregel wirkt" } else { "jenseits 6 d: ohne Wirkung" }))).translucent(0.35).ghosted().with_handles(handle_ids), &[bottom]);
        }
        let stud_binding = binding(input.report, &m.id, &["en1992.6.4.", "en1992.9.4.3.studs."]);
        for sr in m.stud_rows.iter().filter(|r| r.count > 0 && r.diameter > 0.0) {
            let (z0, z1) = (-m.height / 2.0 + m.cover, m.height / 2.0 - m.cover);
            let spec_path = format!("members[id={}].studRows[id={}]", m.id, sr.id);
            let (cos, sin) = (sr.angle.cos(), sr.angle.sin());
            let rail = add(&mul(&f.x, cos), &mul(&f.y, sin));
            let tangent = add(&mul(&f.x, -sin), &mul(&f.y, cos));
            for k in 0..sr.count {
                let (dx, dy) = sr.position(k, p.column_width, p.column_depth);
                let (x, y) = (cx + dx, cy + dy);
                let id = format!("{}:stud-{}-{k}", m.id, sr.id);
                let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: None }, axis: Segment3 { from: at(f, x, y, z0), to: at(f, x, y, z1) }, diameter: sr.diameter, head: HeadShape::None, pattern: None, hole_clearance: None });
                let from_face = sr.spacing_first + f64::from(k) * sr.spacing_radial;
                let mut stud = Spec::new(EntityKind::Fastener, id.clone(), spec_path.clone(), geometry, MaterialRole::Rebar, LAYER_REBAR, label("stud", "Dübel"), label(&format!("stud Ø{} {} of {} at {} mm from the column face, rail at {:.0}°", mm(sr.diameter), k + 1, sr.count, mm(from_face), sr.angle.to_degrees()), &format!("Dübel Ø{} {} von {} bei {} mm vom Stützenrand, Schiene bei {:.0}°", mm(sr.diameter), k + 1, sr.count, mm(from_face), sr.angle.to_degrees()))).bound(stud_binding.clone());
                let stud_ref = entity_ref(EntityKind::Fastener, &id, &spec_path);
                let mut grips = Vec::new();
                if k == 0 {
                    grips.push(length_grip(&m.id, &stud_ref, &m.label_en, &format!("stud-{}-first", sr.id), at(f, x, y, z1), rail.clone(), &format!("studRows[id={}].spacingFirst", sr.id), sr.spacing_first, 1.0, (0.0, 4.0 * d), ("first stud distance s0", "erster Dübelabstand s0"), along));
                    let radius = dx.hypot(dy).max(1e-3);
                    grips.push(length_grip(&m.id, &stud_ref, &m.label_en, &format!("stud-{}-angle", sr.id), at(f, x, y, z1 + 0.03), tangent.clone(), &format!("studRows[id={}].angle", sr.id), sr.angle, 1.0 / radius, (-std::f64::consts::TAU, 2.0 * std::f64::consts::TAU), ("rail angle (tangential drag)", "Schienenwinkel (tangentiales Ziehen)"), along));
                }
                if k == 1 {
                    grips.push(length_grip(&m.id, &stud_ref, &m.label_en, &format!("stud-{}-radial", sr.id), at(f, x, y, z1), rail.clone(), &format!("studRows[id={}].spacingRadial", sr.id), sr.spacing_radial, 1.0, (0.0, 4.0 * d), ("radial stud spacing s_r", "radialer Dübelabstand s_r"), along));
                    grips.push(stud_row_count_grip(&m.id, &stud_ref, &sr.id, at(f, x, y, z1 + 0.03), sr.count));
                }
                if !grips.is_empty() {
                    stud = stud.with_handles(grips.iter().map(|g| g.id.clone()).collect());
                    out.handles.extend(grips.into_iter().map(Grip::build));
                }
                out.push(stud, &[at(f, x, y, z0), at(f, x, y, z1)]);
            }
        }
        let top = at(f, cx, cy, m.height / 2.0);
        out.annotate(Annotation::Dimension { id: format!("{}:dim-d", m.id), from: top.clone(), to: at(f, cx, cy, m.height / 2.0 - d), offset: mul(&f.x, 0.4), value: Quantity::new(QuantityKind::Length, d), binding: Some(FieldBinding::new(format!("members[id={}].effectiveDepth", m.id), Affine::IDENTITY, "m")) }, &top);
    }
}

fn beta_method_options() -> Vec<(String, &'static str, &'static str)> {
    vec![("Simplified".into(), "Simplified (fixed β of 6.4.3(6))", "Vereinfacht (festes β nach 6.4.3(6))"), ("UnbalancedMoments".into(), "From unbalanced moments (6.4.3(3)-(5))", "Aus Stützenmomenten (6.4.3(3)-(5))")]
}
