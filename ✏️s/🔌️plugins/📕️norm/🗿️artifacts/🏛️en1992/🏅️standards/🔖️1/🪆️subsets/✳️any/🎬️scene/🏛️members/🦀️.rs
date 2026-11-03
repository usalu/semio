//! 🏛️ Members: RC beams, columns, walls and slabs as concrete entities with cover shell, single rebar entities, link cages, tendons, supports, load glyphs, deflected shape, crack pattern, fire isotherm, the N-My-Mz interaction surface with its load point, dimensions and handles.

use super::handle_set::*;
use super::kit::*;
use super::layout::{class_of, Class, Layout, MemberPose};
use super::checks::{self, fire_field};
use super::mechanics;
use super::model::{BarPoint, TendonPoint, MemberKind, RcMember, SupportCondition};
use super::norm;
use super::reinforcement::{bar_hooks, bar_layout, bend_mode, centred_span, hook, hook_end, is_top, lap_spans, offset_inward, position_name, BendMode, HookEnd, STOCK_LENGTH};
use super::scene::*;
use super::SceneInput;
use std::f64::consts::PI;

const SURFACE_SCALE: f64 = 1.5;
const SURFACE_LIFT: f64 = 2.0;
const SURFACE_ANGLES: usize = 24;
const MAX_CRACKS: usize = 40;
const CRACK_EXAGGERATION: f64 = 10.0;
const STATIONS: usize = 10;
const ISOTHERM_RAYS: usize = 72;
const TENDON_SAMPLES: usize = 12;

struct Ctx<'a> {
    input: &'a SceneInput<'a>,
    m: &'a RcMember,
    pose: &'a MemberPose,
    sec: Option<checks::Sec>,
    w: f64,
    path: String,
}

impl Ctx<'_> {
    fn id(&self) -> &str {
        &self.m.id
    }

    fn f(&self) -> &Frame {
        &self.pose.frame
    }

    fn eref(&self, kind: EntityKind, sub: &str, field: &str) -> EntityRef {
        entity_ref(kind, &part(self.id(), sub), &format!("{}{field}", self.path))
    }

    fn fails(&self, check_id: &str) -> bool {
        self.input.report.checks.iter().any(|r| r.id == check_id && r.status == norm::CheckStatus::Fail)
    }

    fn bound(&self, prefixes: &[&str]) -> Option<UtilisationBinding> {
        binding(self.input.report, self.id(), prefixes)
    }

    fn name(&self, en: &str, de: &str) -> LocalizedCopy {
        let l = &self.m;
        loc(&format!("{} {en}", l.label_en), &format!("{} {de}", l.label_de))
    }

    fn push(&self, out: &mut Out, spec: Spec, points: &[Vec3]) {
        out.push(spec, points);
    }
}

fn part(id: &str, sub: &str) -> String {
    format!("{id}:{sub}")
}

fn plate_width(m: &RcMember) -> f64 {
    if m.kind == MemberKind::FlatSlab { m.span } else { m.width }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        if !out.wants(&m.id) {
            continue;
        }
        let Some(pose) = layout.member(&m.id) else { continue };
        if m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let w = plate_width(m);
        let ctx = Ctx { input, m, pose, sec: mechanics::member_sec(input.document, m), w, path: format!("members[id={}]", m.id) };
        let mut grips: Vec<Grip> = Vec::new();
        concrete(&ctx, out, &mut grips);
        cover(&ctx, out, &mut grips);
        bars(&ctx, out, &mut grips);
        links(&ctx, out, &mut grips);
        tendon(&ctx, out, &mut grips);
        supports(&ctx, out);
        loads(&ctx, out);
        cracks(&ctx, out);
        isotherm(&ctx, out, &mut grips);
        interaction(&ctx, out);
        dimensions(&ctx, out);
        super::integrity::annotate(input, m, &at(ctx.f(), 0.0, 0.0, m.height / 2.0 + 0.1), out);
        out.handles.extend(grips.into_iter().map(Grip::build));
    }
}

fn dimensions(c: &Ctx, out: &mut Out) {
    let (m, f) = (c.m, c.f());
    let (b, h, l) = (c.w, m.height, m.span);
    let bind = |field: &str| Some(FieldBinding::new(format!("{}.{field}", c.path), Affine::IDENTITY, "m"));
    let specs: [(&str, Vec3, Vec3, Vec3, f64, Option<FieldBinding>); 3] = [
        ("width", at(f, 0.0, -b / 2.0, h / 2.0), at(f, 0.0, b / 2.0, h / 2.0), mul(&f.z, 0.2), b, bind("width").filter(|_| m.kind != MemberKind::FlatSlab)),
        ("height", at(f, 0.0, -b / 2.0, -h / 2.0), at(f, 0.0, -b / 2.0, h / 2.0), mul(&f.y, -0.2), h, bind("height")),
        ("span", at(f, 0.0, 0.0, -h / 2.0), at(f, l, 0.0, -h / 2.0), mul(&f.z, -0.3), l, bind("span")),
    ];
    for (key, from, to, offset, value, binding) in specs {
        let anchor = from.clone();
        out.annotate(Annotation::Dimension { id: part(c.id(), &format!("dim-{key}")), from, to, offset, value: Quantity::new(QuantityKind::Length, value), binding }, &anchor);
    }
}

fn concrete(c: &Ctx, out: &mut Out, grips: &mut Vec<Grip>) {
    let (m, f) = (c.m, c.f());
    let (b, h, l) = (c.w, m.height, m.span);
    let end = at(f, l, 0.0, 0.0);
    let geometry = if c.pose.class == Class::Slab {
        let outline = Polygon2::solid(vec![Vec2::new(0.0, -b / 2.0), Vec2::new(l, -b / 2.0), Vec2::new(l, b / 2.0), Vec2::new(0.0, b / 2.0)]);
        Geometry::Plate(PlateGeometry { frame: f.clone(), outline, thickness: h, offset: 0.0, stiffeners: Vec::new() })
    } else {
        Geometry::Member(MemberGeometry {
            axis: polyline(&[at(f, 0.0, 0.0, 0.0), end.clone()]),
            frame: f.clone(),
            section: member_section(m, b, h),
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Rigid,
            stations: (0..=STATIONS).map(|i| i as f64 / STATIONS as f64).collect(),
        })
    };
    let kind = geometry.kind();
    let ent = entity_ref(kind, &m.id, &c.path);
    grips.push(length_grip(c.id(), &ent, &m.label_en, "span", end, f.x.clone(), "span", l, 1.0, (0.5, 30.0), ("span", "Spannweite"), along));
    if m.kind != MemberKind::FlatSlab {
        grips.push(length_grip(c.id(), &ent, &m.label_en, "width", at(f, l / 2.0, b / 2.0, 0.0), f.y.clone(), "width", b, 2.0, (0.1, 3.0), ("width", "Breite"), face));
    }
    let (top, k) = if c.pose.class == Class::Upright { (at(f, l, 0.0, h / 2.0), 2.0) } else { (at(f, l / 2.0, 0.0, h / 2.0), 1.0) };
    grips.push(length_grip(c.id(), &ent, &m.label_en, "height", top, f.z.clone(), "height", h, k, (0.1, 2.0), ("depth", "Höhe"), face));
    if m.effective_depth > 0.0 {
        grips.push(length_grip(c.id(), &ent, &m.label_en, "effective-depth", at(f, l / 2.0, 0.0, h / 2.0 - m.effective_depth), mul(&f.z, -1.0), "effectiveDepth", m.effective_depth, 1.0, (0.05, (h - m.cover).max(0.06)), ("effective depth", "Nutzhöhe"), along));
    }
    if m.section_points.len() >= 3 {
        let reach = 2.0 * m.width.max(h);
        for (point, (y, z)) in m.section_points.iter().zip(m.outline()) {
            let corner = at(f, 0.0, y, z);
            grips.push(length_grip(c.id(), &ent, &m.label_en, &format!("outline-{}-y", point.id), corner.clone(), f.y.clone(), &format!("sectionPoints[id={}].y", point.id), point.y, 1.0, (point.y - reach, point.y + reach), ("outline point y", "Umrisspunkt y"), along));
            grips.push(length_grip(c.id(), &ent, &m.label_en, &format!("outline-{}-z", point.id), corner, f.z.clone(), &format!("sectionPoints[id={}].z", point.id), point.z, 1.0, (point.z - reach, point.z + reach), ("outline point z", "Umrisspunkt z"), along));
        }
    }
    let moves = super::projection_kit::handle_kit::move_handles(&ent, &m.id, &f.origin, c.input.placements, 0.0, true);
    let move_ids: Vec<String> = moves.iter().map(|h| h.id.clone()).collect();
    out.handles.extend(moves);
    let governing = envelope_binding(c);
    if let Some(clause) = governing.as_ref().and_then(|b| clause_of(c.input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: part(&m.id, "clause"), anchor: ent.clone(), clause }, &at(f, l / 2.0, 0.0, h / 2.0));
    }
    let spec = Spec::new(kind, m.id.clone(), c.path.clone(), geometry, MaterialRole::Concrete, LAYER_CONCRETE, loc(&m.label_en, &m.label_de), c.name(&format!("concrete {} x {} mm", mm(b), mm(h)), &format!("Beton {} x {} mm", mm(b), mm(h))))
        .bound(governing)
        .translucent(0.35)
        .deformed(deflection_overlay(c))
        .with_handles(grips.iter().map(|g| g.id.clone()).chain(move_ids).collect());
    let bounds = [at(f, 0.0, -b / 2.0, -h / 2.0), at(f, l, b / 2.0, h / 2.0), at(f, l, -b / 2.0, -h / 2.0), at(f, 0.0, b / 2.0, h / 2.0)];
    c.push(out, spec, &bounds);
}

fn deflection_overlay(c: &Ctx) -> Option<DeformationOverlay> {
    if c.pose.class == Class::Upright {
        return None;
    }
    let id = format!("en1992.7.4.deflection.{}", c.id());
    let check = c.input.report.checks.iter().find(|r| r.id == id && !r.stations.is_empty())?;
    let samples: Vec<DisplacementSample> = check
        .stations
        .iter()
        .filter(|s| s.at.len() == 1 && s.is_well_formed())
        .map(|s| DisplacementSample { at: s.at[0], delta: mul(&c.f().z, -s.values.first().map_or(0.0, |q| q.value)) })
        .collect();
    let peak = samples.iter().map(|s| s.delta.length()).fold(0.0, f64::max);
    (peak > 0.0).then(|| DeformationOverlay { surface: Vec::new(), motion: Motion::Static, load_factor: None, kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale: (0.06 * c.m.span / peak).clamp(1.0, 500.0), samples })
}

fn cover(c: &Ctx, out: &mut Out, grips: &mut Vec<Grip>) {
    let (m, f) = (c.m, c.f());
    let (b, h, l, cv) = (c.w, m.height, m.span, m.cover);
    if cv <= 0.0 || b - 2.0 * cv <= 1e-3 || h - 2.0 * cv <= 1e-3 || l - 2.0 * cv <= 1e-3 {
        return;
    }
    let ent = c.eref(EntityKind::Member, "cover", ".cover");
    let grip = length_grip(c.id(), &ent, &m.label_en, "cover", at(f, l / 2.0, 0.0, -h / 2.0), f.z.clone(), "cover", cv, 1.0, (0.01, (h / 4.0).clamp(0.02, 0.1)), ("cover", "Betondeckung"), along);
    let id = grip.id.clone();
    grips.push(grip);
    let geometry = Geometry::Member(MemberGeometry {
        axis: polyline(&[at(f, cv, 0.0, 0.0), at(f, l - cv, 0.0, 0.0)]),
        frame: f.clone(),
        section: rect_section(b - 2.0 * cv, h - 2.0 * cv),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: Vec::new(),
    });
    let spec = Spec::new(EntityKind::Member, part(&m.id, "cover"), format!("{}.cover", c.path), geometry, MaterialRole::Concrete, LAYER_COVER, c.name("cover", "Betondeckung"), c.name(&format!("cover shell, c = {} mm", mm(cv)), &format!("Deckungsschale, c = {} mm", mm(cv))))
        .bound(c.bound(&["en1992.4.4.cover."]))
        .translucent(0.12)
        .ghosted()
        .with_handles(vec![id]);
    c.push(out, spec, &[]);
    out.annotate(Annotation::Label { id: part(&m.id, "cover-label"), anchor: at(f, 0.0, -b / 2.0, -h / 2.0), text: loc(&format!("cnom = {} mm", mm(cv)), &format!("cnom = {} mm", mm(cv))), tone: tone_of(&c.bound(&["en1992.4.4.cover."])) }, &at(f, 0.0, -b / 2.0, -h / 2.0));
}

fn flexure_binding(c: &Ctx) -> Option<UtilisationBinding> {
    let base = c.bound(&["en1992.6.1.flexure.", "en1992.9.minas.", "en1992.9.maxas.", "en1992.5.8.slender."])?;
    if base.per_station.is_some() || c.pose.class == Class::Upright {
        return Some(base);
    }
    let flexure = projection_kit::check_with(c.input.report, &format!("en1992.6.1.flexure.{}", c.id()));
    Some(UtilisationBinding { per_station: flexure.map(projection_kit::station_values).filter(|v| !v.is_empty()), ..base })
}

fn envelope_binding(c: &Ctx) -> Option<UtilisationBinding> {
    let base = c.bound(&[])?;
    let mut merged: Vec<StationValue> = Vec::new();
    for check in c.input.report.checks.iter().filter(|k| k.subject.entity_id == c.m.id && k.station_field.is_empty()) {
        for s in projection_kit::station_values(check) {
            match merged.iter_mut().find(|m| (m.at - s.at).abs() < 1e-9) {
                Some(m) => m.value = m.value.max(s.value),
                None => merged.push(s),
            }
        }
    }
    merged.sort_by(|a, b| a.at.total_cmp(&b.at));
    Some(UtilisationBinding { per_station: Some(merged).filter(|v| !v.is_empty()), ..base })
}

fn bars(c: &Ctx, out: &mut Out, grips: &mut Vec<Grip>) {
    let (m, f) = (c.m, c.f());
    let (l, cv) = (m.span, m.cover);
    let binding = flexure_binding(c);
    for layer in bar_layout(m, c.w) {
        let spec = &m.longitudinal[layer.layer];
        let (from_s, to_s) = if l - 2.0 * cv > 0.1 { (cv, l - cv) } else { (0.0, l) };
        let last = layer.points.last().copied().unwrap_or((0.0, 0.0));
        let first = layer.points.first().copied().unwrap_or((0.0, 0.0));
        let ent = entity_ref(EntityKind::Fastener, &format!("{}:bar-{}-0", m.id, spec.id), &format!("{}.longitudinal[id={}]", c.path, spec.id));
        let layer_grips = [
            count_grip(c.id(), &ent, &spec.id, at(f, to_s, last.0, last.1 + if is_top(spec.position) { 0.04 } else { -0.04 }), spec.count),
            diameter_grip(c.id(), &ent, &spec.id, at(f, to_s, first.0, first.1), at(f, to_s, first.0, first.1 + spec.diameter / 2.0), spec.diameter),
        ];
        let ids: Vec<String> = layer_grips.iter().map(|g| g.id.clone()).collect();
        grips.extend(layer_grips);
        let explicit: Vec<&BarPoint> = if layer.explicit { m.bars.iter().filter(|b| b.layer_id == spec.id).collect() } else { Vec::new() };
        let (top, upright) = (is_top(spec.position), class_of(m.kind) == Class::Upright);
        let toward_centre = if top { -1.0 } else { 1.0 };
        let pieces = lap_spans(from_s, to_s, STOCK_LENGTH, if spec.lap_share > 0.0 { spec.lap_length } else { 0.0 });
        let side = hook_end(&f.x, &mul(&f.z, toward_centre));
        let hooked = if upright || !c.fails(&format!("en1992.8.4.anchorage.{}.{}", m.id, spec.id)) { HookEnd::Neither } else { side };
        let lap_binding = c.bound(&["en1992.8.7.lap."]);
        for (k, &(y, z)) in layer.points.iter().enumerate() {
            let bar_d = layer.diameters.get(k).copied().unwrap_or(spec.diameter);
            let shape = hook(bar_d);
            let own = explicit.get(k).copied();
            let bar_pieces = own.and_then(|b| centred_span(from_s, to_s, b.length)).map_or_else(|| pieces.clone(), |span| vec![span]);
            let (bar_hooked, both_ends) = bar_hooks(own.map_or(BendMode::Auto, |b| bend_mode(b.bend)), hooked, side, bar_pieces.len() == 1);
            let bend = BendShape { bend_angle: shape.angle, bend_radius: shape.radius, leg_length: shape.leg, both_ends };
            let detail = loc(&format!("bar {} of {} Ø{} {}", k + 1, spec.count, mm(spec.diameter), position_name(spec.position)), &format!("Stab {} von {} Ø{} {}", k + 1, spec.count, mm(spec.diameter), position_name(spec.position)));
            for (j, &(a, b)) in bar_pieces.iter().enumerate() {
                let z_piece = z + if j % 2 == 1 { toward_centre * bar_d } else { 0.0 };
                let (mut p0, mut p1, mut piece_bend) = (at(f, a, y, z_piece), at(f, b, y, z_piece), None);
                if bar_hooked != HookEnd::Neither && j == if bar_hooked == HookEnd::Near { 0 } else { bar_pieces.len() - 1 } {
                    if bar_hooked == HookEnd::Near {
                        (p0, p1) = (p1, p0);
                    }
                    piece_bend = Some(bend.clone());
                }
                let id = if j == 0 { format!("{}:bar-{}-{k}", m.id, spec.id) } else { format!("{}:lap-{}-{k}-{j}", m.id, spec.id) };
                let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: piece_bend.clone() }, axis: Segment3 { from: p0.clone(), to: p1.clone() }, diameter: bar_d, head: HeadShape::None, pattern: None, hole_clearance: None });
                let detail = if j == 0 && piece_bend.is_none() { detail.clone() } else if j > 0 {
                    loc(&format!("bar {} of {} Ø{} {}, lapped piece {} (lap {} mm, 8.7)", k + 1, spec.count, mm(spec.diameter), position_name(spec.position), j + 1, mm(spec.lap_length)), &format!("Stab {} von {} Ø{} {}, gestoßenes Stück {} (Übergreifung {} mm, 8.7)", k + 1, spec.count, mm(spec.diameter), position_name(spec.position), j + 1, mm(spec.lap_length)))
                } else {
                    loc(&format!("bar {} of {} Ø{} {}, hooked end (8.3)", k + 1, spec.count, mm(spec.diameter), position_name(spec.position)), &format!("Stab {} von {} Ø{} {}, Endhaken (8.3)", k + 1, spec.count, mm(spec.diameter), position_name(spec.position)))
                };
                let mut s = Spec::new(EntityKind::Fastener, id, format!("{}.longitudinal[id={}]", c.path, spec.id), geometry, MaterialRole::Rebar, LAYER_REBAR, c.name(&format!("bar Ø{}", mm(spec.diameter)), &format!("Stab Ø{}", mm(spec.diameter))), detail).bound(if j > 0 { lap_binding.clone() } else { binding.clone() });
                if k == 0 && j == 0 {
                    s = s.with_handles(ids.clone());
                }
                if j == 0 {
                    if let Some(bp) = explicit.get(k) {
                        let bar_ref = entity_ref(EntityKind::Fastener, &format!("{}:bar-{}-{k}", m.id, spec.id), &format!("{}.bars[id={}]", c.path, bp.id));
                        let gy = length_grip(c.id(), &bar_ref, &m.label_en, &format!("bar-{}-y", bp.id), at(f, l / 2.0, y, z), f.y.clone(), &format!("bars[id={}].y", bp.id), bp.y, 1.0, (-c.w / 2.0 + cv, c.w / 2.0 - cv), ("bar y", "Stab y"), along);
                        let gz = length_grip(c.id(), &bar_ref, &m.label_en, &format!("bar-{}-z", bp.id), at(f, l / 2.0, y, z), f.z.clone(), &format!("bars[id={}].z", bp.id), bp.z, 1.0, (-m.height / 2.0 + cv, m.height / 2.0 - cv), ("bar z", "Stab z"), along);
                        let span = bar_pieces.first().copied().unwrap_or((from_s, to_s));
                        let gl = length_grip(c.id(), &bar_ref, &m.label_en, &format!("bar-{}-length", bp.id), at(f, span.1, y, z), f.x.clone(), &format!("bars[id={}].length", bp.id), span.1 - span.0, 2.0, (0.1, l), ("bar length", "Stablänge"), along);
                        s = s.with_handles(vec![gy.id.clone(), gz.id.clone(), gl.id.clone()]);
                        grips.push(gy);
                        grips.push(gz);
                        grips.push(gl);
                    }
                }
                c.push(out, s, &[p0, p1]);
            }
            if k == 0 && pieces.len() > 1 {
                let (from, to) = (at(f, pieces[1].0, y, z), at(f, pieces[0].1, y, z));
                out.annotate(Annotation::Dimension { id: part(&m.id, &format!("lap-{}", spec.id)), from: from.clone(), to, offset: mul(&f.y, 0.25), value: Quantity::new(QuantityKind::Length, pieces[0].1 - pieces[1].0), binding: None }, &from);
            }
        }
        if let Some(&(y, z)) = layer.points.get(layer.points.len() / 2) {
            let tag = format!("{}Ø{}", spec.count, mm(spec.diameter));
            out.annotate(Annotation::Label { id: part(&m.id, &format!("label-{}", spec.id)), anchor: at(f, l / 2.0, y, z), text: loc(&tag, &tag), tone: tone_of(&binding) }, &at(f, l / 2.0, y, z));
        }
        if spec.anchorage_length > 0.0 && spec.anchorage_length < l {
            if let Some(&(y, z)) = layer.points.first() {
                let from = at(f, 0.0, y, z);
                out.annotate(Annotation::Dimension { id: part(&m.id, &format!("lbd-{}", spec.id)), from: from.clone(), to: at(f, spec.anchorage_length, y, z), offset: mul(&f.y, -0.25), value: Quantity::new(QuantityKind::Length, spec.anchorage_length), binding: None }, &from);
            }
        }
    }
}

fn links(c: &Ctx, out: &mut Out, grips: &mut Vec<Grip>) {
    let (m, f) = (c.m, c.f());
    let Some(s) = m.stirrups.as_ref().filter(|s| s.spacing > 1e-4 && s.diameter > 0.0) else { return };
    let (l, cv) = (m.span, m.cover);
    if l - 2.0 * cv <= 0.05 {
        return;
    }
    let (half_w, half_h) = (((c.w - 2.0 * cv - s.diameter) / 2.0).max(0.01), ((m.height - 2.0 * cv - s.diameter) / 2.0).max(0.01));
    let outline = if m.section_points.len() >= 3 {
        Ring2 { points: offset_inward(&m.outline(), cv + s.diameter / 2.0).into_iter().map(|(y, z)| Vec2::new(y, z)).collect() }
    } else {
        Ring2 { points: vec![Vec2::new(-half_w, -half_h), Vec2::new(half_w, -half_h), Vec2::new(half_w, half_h), Vec2::new(-half_w, half_h)] }
    };
    let ent = c.eref(EntityKind::Fastener, "links", ".stirrups");
    let grip = length_grip(c.id(), &ent, &m.label_en, "link-spacing", at(f, cv + s.spacing, 0.0, m.height / 2.0), f.x.clone(), "stirrups.spacing", s.spacing, 1.0, (0.05, 0.4), ("link spacing", "Bügelabstand"), along);
    let ids = vec![grip.id.clone()];
    grips.push(grip);
    let count = ((l - 2.0 * cv) / s.spacing).floor() as u32 + 1;
    let bend = BendShape { bend_angle: 3.0 * PI / 4.0, bend_radius: 2.0 * s.diameter, leg_length: 10.0 * s.diameter, both_ends: false };
    let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: Some(bend) }, axis: Segment3 { from: at(f, cv, 0.0, 0.0), to: at(f, l - cv, 0.0, 0.0) }, diameter: s.diameter, head: HeadShape::None, pattern: Some(Pattern::Cage { outline, zones: vec![LinkZone { start: 0.0, spacing: s.spacing, count }], reference: f.y.clone() }), hole_clearance: None });
    let spec = Spec::new(EntityKind::Fastener, part(&m.id, "links"), format!("{}.stirrups", c.path), geometry, MaterialRole::Rebar, LAYER_REBAR, c.name("links", "Bügel"), c.name(&format!("{} links Ø{} at {} mm, {} legs", count, mm(s.diameter), mm(s.spacing), s.legs), &format!("{} Bügel Ø{} im Abstand {} mm, {} Schnitte", count, mm(s.diameter), mm(s.spacing), s.legs)))
        .bound(c.bound(&["en1992.6.2.shear.", "en1992.6.3.torsion.", "en1992.6.3.torsion-links.", "en1992.6.3.torsion-longitudinal.", "en1992.6.3.interaction."]))
        .with_handles(ids);
    c.push(out, spec, &[at(f, cv, 0.0, 0.0), at(f, l - cv, 0.0, 0.0)]);
}

fn tendon(c: &Ctx, out: &mut Out, grips: &mut Vec<Grip>) {
    let (m, f) = (c.m, c.f());
    let Some(p) = m.prestress.as_ref().filter(|p| p.area > 0.0) else { return };
    let (l, e) = (m.span, m.tendon_eccentricity_mid().unwrap_or(p.eccentricity));
    let reach = (m.height / 2.0 - m.cover).max(0.02);
    let ent = c.eref(EntityKind::Fastener, "tendon", ".prestress");
    let grip = Grip {
        id: format!("{}/tendon/eccentricity", m.id),
        entity: ent,
        kind: along(mul(&f.z, -1.0)),
        style: GripStyle::Diamond,
        anchor: at(f, l / 2.0, 0.0, -e),
        path: format!("{}.prestress.eccentricity", c.path),
        unit: "m",
        base: e,
        k: 1.0,
        range: (-reach, reach),
        step: 0.005,
        quantise: Some(Quantise::Round { step: 0.001 }),
        name: loc(&format!("{} tendon eccentricity handle", m.label_en), &format!("{} Spanngliedexzentrizität Griff", m.label_de)),
    };
    let mut ids = Vec::new();
    let mut points: Vec<&TendonPoint> = m.tendon_points.iter().collect();
    points.sort_by(|a, b| a.x.total_cmp(&b.x));
    let control = if points.len() >= 2 {
        for tp in &points {
            let gz = length_grip(c.id(), &ent, &m.label_en, &format!("tendon-{}-z", tp.id), at(f, tp.x, tp.y, tp.z), f.z.clone(), &format!("tendonPoints[id={}].z", tp.id), tp.z, 1.0, (-reach, reach), ("tendon point z", "Spanngliedpunkt z"), along);
            let gx = length_grip(c.id(), &ent, &m.label_en, &format!("tendon-{}-x", tp.id), at(f, tp.x, tp.y, tp.z), f.x.clone(), &format!("tendonPoints[id={}].x", tp.id), tp.x, 1.0, (0.0, l), ("tendon point x", "Spanngliedpunkt x"), along);
            ids.push(gz.id.clone());
            ids.push(gx.id.clone());
            grips.push(gz);
            grips.push(gx);
        }
        if points.len() >= 3 { m.tendon_profile(TENDON_SAMPLES).into_iter().map(|(x, y, z)| at(f, x, y, z)).collect::<Vec<_>>() } else { points.iter().map(|tp| at(f, tp.x, tp.y, tp.z)).collect::<Vec<_>>() }
    } else {
        ids.push(grip.id.clone());
        grips.push(grip);
        vec![at(f, 0.0, 0.0, 0.0), at(f, l / 2.0, 0.0, -2.0 * e), at(f, l, 0.0, 0.0)]
    };
    let profile = if points.len() >= 3 { Path3::Polyline(Polyline3 { points: control.clone() }) } else { Path3::Spline(Spline3 { control: control.clone(), degree: if control.len() >= 3 { 2 } else { 1 } }) };
    let diameter = 2.0 * (p.area / PI).sqrt();
    let geometry = Geometry::Fastener(FastenerGeometry {
        kind: FastenerKind::Tendon { profile, duct: Some((1.8 * diameter).max(0.05)) },
        axis: Segment3 { from: control[0].clone(), to: control[control.len() - 1].clone() },
        diameter,
        head: HeadShape::None,
        pattern: None,
        hole_clearance: None,
    });
    let force = p.force * (1.0 - p.loss_ratio);
    let detail = loc(&format!("tendon, P0 = {} kN, Pm,inf = {} kN, e = {} mm", kn(p.force), kn(force), mm(e)), &format!("Spannglied, P0 = {} kN, Pm,inf = {} kN, e = {} mm", kn(p.force), kn(force), mm(e)));
    let binding = c.bound(&["en1992.5.10.", "en1992.a-16.", "en1992.pr-01.", "en1992.pr-02."]);
    let spec = Spec::new(EntityKind::Fastener, part(&m.id, "tendon"), format!("{}.prestress", c.path), geometry, MaterialRole::Steel, LAYER_PRESTRESS, c.name("tendon", "Spannglied"), detail.clone()).bound(binding.clone()).with_handles(ids);
    c.push(out, spec, &control);
    out.annotate(Annotation::Label { id: part(&m.id, "tendon-label"), anchor: at(f, l / 2.0, 0.0, -e), text: detail, tone: tone_of(&binding) }, &at(f, l / 2.0, 0.0, -e));
}

fn supports(c: &Ctx, out: &mut Out) {
    let (m, f) = (c.m, c.f());
    let (l, h) = (m.span, m.height);
    let ends: Vec<(&str, f64, SupportKind)> = match (c.pose.class, m.support) {
        (Class::Upright, _) => vec![("base", 0.0, SupportKind::Fixed)],
        (_, SupportCondition::SimplySupported) => vec![("start", 0.0, SupportKind::Pinned), ("end", l, SupportKind::Roller { normal: f.z.clone() })],
        (_, SupportCondition::Continuous) => vec![("start", 0.0, SupportKind::Pinned), ("end", l, SupportKind::Roller { normal: f.z.clone() })],
        (_, SupportCondition::Fixed) => vec![("start", 0.0, SupportKind::Fixed), ("end", l, SupportKind::Fixed)],
        (_, SupportCondition::Cantilever) => vec![("start", 0.0, SupportKind::Fixed)],
    };
    for (key, s, kind) in ends {
        let z = if c.pose.class == Class::Upright { 0.0 } else { -h / 2.0 };
        let point = at(f, s, 0.0, z);
        let geometry = Geometry::Support(SupportGeometry { kind, at: point.clone(), frame: Frame { origin: point.clone(), ..f.clone() } });
        let spec = Spec::new(EntityKind::Support, part(&m.id, &format!("support-{key}")), format!("{}.support", c.path), geometry, MaterialRole::Support, LAYER_SUPPORTS, c.name(&format!("support {key}"), &format!("Lager {key}")), c.name("support condition", "Lagerung"));
        c.push(out, spec, &[point]);
    }
}

fn loads(c: &Ctx, out: &mut Out) {
    let (m, f) = (c.m, c.f());
    let (l, h) = (m.span, m.height);
    let down = mul(&f.z, -1.0);
    let upright = c.pose.class == Class::Upright;
    for (i, a) in m.actions.iter().enumerate() {
        let case = LoadCaseRef { id: a.id.clone(), kind: load_kind(a.kind), psi: None };
        let lift = 0.12 * (i as f64 + 1.0);
        let path = format!("{}.actions[id={}]", c.path, a.id);
        let mut glyphs: Vec<(String, LoadGlyph, Vec3)> = Vec::new();
        let line = a.g_k_line + a.q_k_line;
        if line.abs() > 0.0 && !upright {
            let (p0, p1) = (at(f, 0.0, 0.0, h / 2.0 + lift), at(f, l, 0.0, h / 2.0 + lift));
            glyphs.push(("line".into(), LoadGlyph::Line { path: polyline(&[p0.clone(), p1]), direction: down.clone(), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, line) }, case: case.clone() }, p0));
        }
        let top = if upright { at(f, l, 0.0, 0.0) } else { at(f, l / 2.0, 0.0, h / 2.0 + lift) };
        let vertical = if upright { a.n_k } else { a.point_force };
        if vertical.abs() > 0.0 {
            glyphs.push(("point".into(), LoadGlyph::Point { at: top.clone(), direction: if upright { mul(&f.x, -1.0) } else { down.clone() }, magnitude: Quantity::new(QuantityKind::Force, vertical), case: case.clone() }, top.clone()));
        }
        if a.m_k.abs() > 0.0 {
            let p = at(f, l / 2.0, 0.0, h / 2.0 + lift);
            glyphs.push(("moment".into(), LoadGlyph::Moment { at: p.clone(), axis: f.y.clone(), magnitude: Quantity::new(QuantityKind::Moment, a.m_k), case: case.clone() }, p));
        }
        for (key, glyph, anchor) in glyphs {
            let spec = Spec::new(EntityKind::Load, part(&m.id, &format!("load-{}-{key}", a.id)), path.clone(), Geometry::Load(glyph), MaterialRole::Load { kind: load_kind(a.kind) }, LAYER_LOADS, c.name(&format!("action {}", a.id), &format!("Einwirkung {}", a.id)), c.name(&format!("{} action {} ({})", a.kind, a.id, key), &format!("Einwirkung {} {} ({})", a.kind, a.id, key)));
            c.push(out, spec, &[anchor]);
        }
    }
}

fn crack_check<'a>(c: &'a Ctx) -> Option<&'a norm::CheckResult> {
    let id = format!("en1992.7.3.crack.{}", c.id());
    c.input.report.checks.iter().find(|r| r.id == id && r.status != norm::CheckStatus::NotApplicable && !r.stations.is_empty())
}

fn cracks(c: &Ctx, out: &mut Out) {
    let (m, f) = (c.m, c.f());
    let (Some(sec), Some(check)) = (c.sec.as_ref(), crack_check(c)) else { return };
    if c.pose.class == Class::Upright {
        return;
    }
    let Some(state) = mechanics::crack_state(sec, m, &c.input.document.action_factors()) else { return };
    let w_max = check.limit.value;
    let params: Vec<f64> = (0..=200).map(|i| i as f64 / 200.0).collect();
    let widths: Vec<f64> = projection_kit::field_along(check, &params).into_iter().map(|u| u * w_max).collect();
    let zone: Vec<f64> = params.iter().zip(&widths).filter(|(_, w)| **w > 1e-6).map(|(s, _)| *s).collect();
    let (Some(&lo), Some(&hi)) = (zone.first(), zone.last()) else { return };
    let cantilever = m.support == SupportCondition::Cantilever;
    let (span_lo, span_hi) = (lo * m.span, hi * m.span);
    let count = (((span_hi - span_lo) / state.sr_max).floor() as usize).min(MAX_CRACKS);
    let (face, h) = (-c.w / 2.0 - 0.002, m.height);
    let (edge, sign) = if cantilever { (h / 2.0, -1.0) } else { (-h / 2.0, 1.0) };
    let binding = c.bound(&["en1992.7.3.crack."]);
    for i in 0..count {
        let x = span_lo + state.sr_max * (i as f64 + 0.5) + (span_hi - span_lo - state.sr_max * count as f64) / 2.0;
        let wk = widths[((x / m.span) * 200.0).round().clamp(0.0, 200.0) as usize];
        let half = (CRACK_EXAGGERATION * wk).max(0.001) / 2.0;
        let apex = edge + sign * (h - state.x).max(0.05);
        let ring = ring3(vec![at(f, x - half, face, edge), at(f, x + half, face, edge), at(f, x, face, apex)]);
        let spec = Spec::new(EntityKind::Surface, part(&m.id, &format!("crack-{i}")), format!("{}.actions", c.path), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: ring.clone() } }), MaterialRole::Neutral, LAYER_CRACKS, c.name("crack", "Riss"), c.name(&format!("predicted crack, wk = {:.2} mm from the check station, sr,max = {} mm, drawn {}x wider", wk * 1000.0, mm(state.sr_max), CRACK_EXAGGERATION), &format!("berechneter Riss, wk = {:.2} mm aus der Prüfstelle, sr,max = {} mm, {}-fach verbreitert", wk * 1000.0, mm(state.sr_max), CRACK_EXAGGERATION))).bound(binding.as_ref().map(|b| projection_kit::binding_at_ratio(b, wk / w_max.max(f64::EPSILON))));
        c.push(out, spec, &ring.outer.points);
    }
}

fn isotherm(c: &Ctx, out: &mut Out, grips: &mut Vec<Grip>) {
    let (m, f) = (c.m, c.f());
    let Some(fire) = m.fire.as_ref() else { return };
    let minutes = fire_field::rating_minutes(fire.rating);
    let (outline, faces) = (m.outline(), fire_field::exposed_faces(m.kind));
    let field = fire_field::solve_outline(&outline, faces, minutes, fire_field::MOISTURE_DEFAULT);
    let contour = field.isotherm_ring(fire_field::LEVEL_500, ISOTHERM_RAYS);
    if contour.len() < 3 {
        return;
    }
    let depth = field.ring_depth(fire_field::LEVEL_500, ISOTHERM_RAYS);
    let zone = fire_field::zone_section(&outline, faces, &field, fire_field::zones());
    let ent = c.eref(EntityKind::Surface, "isotherm", ".fire");
    let grip = length_grip(c.id(), &ent, &m.label_en, "axis-distance", at(f, -0.003, 0.0, -m.height / 2.0 + fire.axis_distance), f.z.clone(), "fire.axisDistance", fire.axis_distance, 1.0, (0.01, (m.height / 2.0).max(0.02)), ("axis distance", "Achsabstand"), along);
    let ids = vec![grip.id.clone()];
    grips.push(grip);
    let ring = ring3(contour.iter().map(|&(y, z)| at(f, -0.003, y, z)).collect());
    let spec = Spec::new(EntityKind::Surface, part(&m.id, "isotherm"), format!("{}.fire", c.path), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: ring.clone() } }), MaterialRole::Neutral, LAYER_FIRE, c.name("500 °C isotherm", "500-°C-Isotherme"), c.name(&format!("section inside the 500 °C isotherm after {minutes:.0} min from a 2D transient conduction solve (EN 1992-1-2 3.3 properties), deepest advance {} mm", mm(depth)), &format!("Querschnitt innerhalb der 500-°C-Isotherme nach {minutes:.0} min aus einer instationären 2D-Wärmeleitungsrechnung (Stoffwerte EN 1992-1-2 3.3), größte Tiefe {} mm", mm(depth))))
        .bound(c.bound(&["en1992.1-2."]))
        .ghosted()
        .translucent(0.5)
        .with_handles(ids);
    c.push(out, spec, &ring.outer.points);
    if zone.contour.len() >= 3 {
        let a_z = zone.damage.iter().copied().fold(0.0, f64::max);
        let zone_ring = ring3(zone.contour.iter().map(|&(y, z)| at(f, -0.004, y, z)).collect());
        let zone_spec = Spec::new(EntityKind::Surface, part(&m.id, "zone-section"), format!("{}.fire", c.path), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: zone_ring.clone() } }), MaterialRole::Neutral, LAYER_FIRE, c.name("zone method section", "Zonenmethoden-Querschnitt"), c.name(&format!("section of the zone method (EN 1992-1-2 Annex B.2) after {minutes:.0} min: the outline moved in by the damage a_z per exposed edge, up to {} mm", mm(a_z)), &format!("Querschnitt der Zonenmethode (EN 1992-1-2 Anhang B.2) nach {minutes:.0} min: die Kontur um die Schädigung a_z je beflammter Kante eingerückt, bis {} mm", mm(a_z))))
            .bound(c.bound(&["en1992.1-2."]))
            .ghosted()
            .translucent(0.4);
        c.push(out, zone_spec, &zone_ring.outer.points);
    }
    let (l, cv) = (m.span, m.cover);
    let (from_s, to_s) = if l - 2.0 * cv > 0.1 { (cv, l - cv) } else { (0.0, l) };
    let stations = [from_s, to_s];
    let mut tube = Vec::with_capacity(stations.len() * (contour.len() + 1));
    for s in stations {
        for &(y, z) in contour.iter().chain(contour.first()) {
            tube.push(at(f, s, y, z));
        }
    }
    let grid = PointGrid { rows: stations.len() as u32, columns: (contour.len() + 1) as u32, points: tube.clone() };
    let surface = Spec::new(EntityKind::Surface, part(&m.id, "isotherm-surface"), format!("{}.fire", c.path), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }), MaterialRole::Neutral, LAYER_FIRE, c.name("500 °C isotherm surface", "500-°C-Isothermenfläche"), c.name(&format!("surface of the 500 °C isotherm along the member, advance up to {} mm after {} min", mm(depth), minutes as u32), &format!("Fläche der 500-°C-Isotherme entlang des Bauteils, Vordringen bis {} mm nach {} min", mm(depth), minutes as u32)))
        .bound(c.bound(&["en1992.1-2."]))
        .ghosted()
        .translucent(0.3);
    c.push(out, surface, &tube);
}

fn interaction(c: &Ctx, out: &mut Out) {
    let (m, pose) = (c.m, c.pose);
    let Some(sec) = c.sec.as_ref().filter(|sec| !sec.bars.is_empty()) else { return };
    let n0 = mechanics::n0(sec);
    if n0 <= 0.0 {
        return;
    }
    let m_ref = 0.25 * n0 * mechanics::size(sec);
    let to_world = |r: &mechanics::Resultant| v3(pose.interaction.x + r.my / m_ref * SURFACE_SCALE, pose.interaction.y + r.mz / m_ref * SURFACE_SCALE, SURFACE_LIFT + r.n / n0 * SURFACE_SCALE);
    let rows = mechanics::surface(sec, SURFACE_ANGLES);
    let points: Vec<Vec3> = rows.iter().flatten().map(&to_world).collect();
    let grid = PointGrid { rows: rows.len() as u32, columns: (SURFACE_ANGLES + 1) as u32, points: points.clone() };
    let binding = c.bound(&["en1992.6.1.flexure.", "en1992.5.8.slender."]);
    let spec = Spec::new(EntityKind::Surface, part(&m.id, "interaction"), format!("{}.longitudinal", c.path), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }), MaterialRole::Neutral, LAYER_INTERACTION, c.name("N-My-Mz interaction surface", "N-My-Mz-Interaktionsfläche"), c.name(&format!("capacity surface, N0 = {} kN, axes normalised by N0 and N0·max(b,h)/4", kn(n0)), &format!("Tragfähigkeitsfläche, N0 = {} kN, Achsen normiert mit N0 und N0·max(b,h)/4", kn(n0)))).bound(binding.clone()).translucent(0.4);
    c.push(out, spec, &points);
    let origin = v3(pose.interaction.x, pose.interaction.y, SURFACE_LIFT);
    for (key, tip, en, de) in [("n", v3(origin.x, origin.y, origin.z + SURFACE_SCALE), "N", "N"), ("my", v3(origin.x + SURFACE_SCALE, origin.y, origin.z), "My", "My"), ("mz", v3(origin.x, origin.y + SURFACE_SCALE, origin.z), "Mz", "Mz")] {
        out.annotate(Annotation::Label { id: part(&m.id, &format!("axis-{key}")), anchor: tip.clone(), text: loc(en, de), tone: Tone::Neutral }, &tip);
    }
    let d = mechanics::reported_effects(c.input.report, m).unwrap_or_else(|| mechanics::design_effects(m, &c.input.document.action_factors()));
    let ring = diagram_kit::interaction_polygon::interaction_polygon(&mechanics::slice(sec, d.n, SURFACE_ANGLES));
    let hit = if ring.len() >= 3 { diagram_kit::interaction_polygon::utilisation_ray(&ring, [0.0, 0.0], [d.my, d.mz]) } else { None };
    let load_point = to_world(&mechanics::Resultant { n: d.n, my: d.my, mz: d.mz });
    let eta = hit.map_or(f64::NAN, |h| h.utilisation);
    let text = if eta.is_finite() { loc(&format!("design point, geometric ηu = {eta:.2}"), &format!("Bemessungspunkt, geometrisch ηu = {eta:.2}")) } else { loc("design point", "Bemessungspunkt") };
    let glyph = LoadGlyph::Point { at: load_point.clone(), direction: unit(&v3(origin.x - load_point.x, origin.y - load_point.y, origin.z - load_point.z)), magnitude: Quantity::new(QuantityKind::Force, d.n), case: LoadCaseRef { id: "design-point".into(), kind: LoadKind::Other, psi: None } };
    let spec = Spec::new(EntityKind::Load, part(&m.id, "load-point"), format!("{}.actions", c.path), Geometry::Load(glyph), MaterialRole::Load { kind: LoadKind::Other }, LAYER_INTERACTION, c.name("design point", "Bemessungspunkt"), LocalizedCopy::new(format!("{} {}", m.label_en, text.en), format!("{} {}", m.label_de, text.de))).bound(binding);
    c.push(out, spec, &[load_point.clone()]);
    if let Some(h) = hit {
        let boundary = to_world(&mechanics::Resultant { n: d.n, my: h.boundary[0], mz: h.boundary[1] });
        out.annotate(Annotation::Dimension { id: part(&m.id, "margin"), from: load_point.clone(), to: boundary, offset: v3(0.0, 0.0, 0.1), value: Quantity::new(QuantityKind::Dimensionless, h.utilisation), binding: None }, &load_point);
    }
}
