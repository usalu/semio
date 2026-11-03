//! 🏗️ Members: aluminium extrusions along the frame with HAZ bands of the longitudinal welds, the effective section ghost of the local buckling reduction, supports, lateral restraints, design-action glyphs, dimensions, buckling and LTB mode overlays and failure glyphs.

use super::facts::{self, Band};
use super::kit::*;
use super::layout::{is_upright, Layout};
use super::model::{AluminiumMember, AluminiumSection, MemberAction, SupportCondition};
use super::scene::*;
use super::tracks::{self, Mode};
use super::{checks, eigen, handles, norm, SceneInput};
use super::model::{PlateElement, WeldRun};

const BUCKLING_SHOWN_FROM: f64 = 0.5;
const MAX_RESTRAINTS: usize = 12;
const MAX_HALF_WAVES: f64 = 12.0;
const HAZ_SKIN: f64 = 0.0005;
const MIN_FACTOR: f64 = 1e-3;

/// 🌊️ Local plate-buckling mode: the exact half-sine of a long compressed plate, whose half-wave length equals the plate width, so the member carries `L / b` half waves (at least one).
fn local_mode(direction: &Vec3, length: f64, plate_width: f64) -> Vec<DisplacementSample> {
    let half_waves = (length / plate_width.max(1e-3)).round().clamp(1.0, MAX_HALF_WAVES);
    (0..=40).map(|i| i as f64 / 40.0).map(|t| DisplacementSample { at: t, delta: mul(direction, (half_waves * std::f64::consts::PI * t).sin()) }).collect()
}

fn exact<'a>(report: &'a norm::CheckReport, id: &str) -> Option<&'a norm::CheckResult> {
    report.checks.iter().find(|c| c.id == id && c.status != norm::CheckStatus::NotApplicable && c.utilization.is_finite())
}

fn largest_value(check: &norm::CheckResult, index: usize) -> Option<f64> {
    check.stations.iter().filter_map(|s| s.values.get(index)).map(|q| q.value.abs()).filter(|v| v.is_finite() && *v > 0.0).reduce(f64::max)
}

struct Shown {
    overlay: Option<DeformationOverlay>,
    shapes: Vec<(Mode, DeformationOverlay)>,
    factor: Option<(&'static str, f64)>,
    modes: Vec<(FailureMode, scene::CheckStatus)>,
}

fn overlay_of(kind: DeformationKind, load_factor: Option<f64>, scale: f64, shape: eigen::Eigen) -> DeformationOverlay {
    let mode = matches!(kind, DeformationKind::Mode);
    DeformationOverlay { surface: Vec::new(), motion: if mode { Motion::Harmonic } else { Motion::Static }, load_factor, kind, index: 1, frequency: None, participation: 1.0, scale, samples: shape.samples }
}

fn shown(input: &SceneInput, member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, depth: f64) -> Shown {
    let report = input.report;
    let id = &member.id;
    let flexural = [exact(report, &format!("en1999.6.3.1.fb.{id}")), exact(report, &format!("en1999.6.3.1.fb.z.{id}"))].into_iter().flatten().max_by(|a, b| a.utilization.total_cmp(&b.utilization));
    let ltb = exact(report, &format!("en1999.6.3.2.ltb.{id}"));
    let deflection = exact(report, &format!("en1999.7.2.defl.{id}"));
    let udl = member.actions.iter().any(|a| a.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl && (a.g_k_line + a.q_k_line).abs() > 1e-12);
    let amplitude = eigen::MODE_AMPLITUDE * member.length;
    let governing = [flexural.map(|c| (c.utilization, true)), ltb.map(|c| (c.utilization, false))].into_iter().flatten().max_by(|a, b| a.0.total_cmp(&b.0)).filter(|m| m.0 >= BUCKLING_SHOWN_FROM).map(|m| m.1);
    let flex = flexural.and_then(|c| eigen::flexural(member, section, frame).map(|e| (largest_value(c, 0).map(|n| e.factor / n), e)));
    let twist = ltb.and_then(|c| largest_value(c, 0)).and_then(|m| eigen::lateral_torsional(member, section, frame, udl, m, depth).map(|e| (Some(e.factor / m), e)));
    let torsional = exact(report, &format!("en1999.6.3.1.tb.{id}"));
    let tf = torsional.and_then(|c| eigen::torsional_flexural(member, section, frame, depth).map(|e| (largest_value(c, 0).map(|n| e.factor / n), e)));
    let factor = match governing {
        Some(true) => flex.as_ref().and_then(|(a, _)| *a).map(|a| ("N", a)),
        Some(false) => twist.as_ref().and_then(|(a, _)| *a).map(|a| ("M", a)),
        None => None,
    };
    let shapes = [(Mode::Flexural, flex), (Mode::LateralTorsional, twist), (Mode::TorsionalFlexural, tf)]
        .into_iter()
        .filter_map(|(mode, found)| found.map(|(alpha, e)| (mode, DeformationOverlay { index: mode.index(), ..overlay_of(DeformationKind::Mode, alpha, amplitude, e) })))
        .collect();
    let overlay = deflection.and_then(|c| eigen::deflection(member, section, frame, udl, c.computed.value.abs())).map(|e| overlay_of(DeformationKind::Deformed, None, e.factor, e));
    let mut modes: Vec<(FailureMode, scene::CheckStatus)> = Vec::new();
    for (prefix, mode) in [("en1999.6.3.1.fb", FailureMode::FlexuralBuckling), ("en1999.6.3.2", FailureMode::LateralTorsionalBuckling), ("en1999.6.2.", FailureMode::Yielding), ("en1999.6.3.3", FailureMode::Yielding)] {
        if let Some(c) = worst(report, id, prefix).filter(|c| matches!(c.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
            if !modes.iter().any(|(m, _)| *m == mode) {
                modes.push((mode, to_scene_status(c.status)));
            }
        }
    }
    Shown { overlay, shapes, factor, modes }
}

fn rect_polygon(centre: (f64, f64), size: (f64, f64)) -> Polygon2 {
    let (w, h) = (size.0 / 2.0, size.1 / 2.0);
    Polygon2::solid(vec![Vec2::new(centre.0 - w, centre.1 - h), Vec2::new(centre.0 + w, centre.1 - h), Vec2::new(centre.0 + w, centre.1 + h), Vec2::new(centre.0 - w, centre.1 + h)])
}

fn effective_shape(parts: &[Band]) -> Option<SectionShape> {
    let shapes: Vec<CompositePart> = parts
        .iter()
        .filter_map(|b| {
            let (w, h) = b.effective();
            (w > 1e-6 && h > 1e-6 && b.factor > MIN_FACTOR).then(|| CompositePart { shape: Box::new(SectionShape::parametric(ProfileKind::Rect, &[("h", h), ("b", w)])), role: MaterialRole::Aluminium, offset: Vec2::new(b.y, b.z) })
        })
        .collect();
    (!shapes.is_empty()).then(|| SectionShape::Composite { parts: shapes, rebar: Vec::new() })
}

fn point_on_section(frame: &Frame, at: &Vec3, y: f64, z: f64) -> Vec3 {
    along(&along(at, &frame.y, y), &frame.z, z)
}

fn load_kind(kind: semio_s_artifact_norm_en1999_model::ActionKind) -> LoadKind {
    match kind {
        semio_s_artifact_norm_en1999_model::ActionKind::Permanent => LoadKind::Permanent,
        semio_s_artifact_norm_en1999_model::ActionKind::Imposed => LoadKind::Variable,
        semio_s_artifact_norm_en1999_model::ActionKind::Snow => LoadKind::Snow,
        semio_s_artifact_norm_en1999_model::ActionKind::Wind => LoadKind::Wind,
        semio_s_artifact_norm_en1999_model::ActionKind::Temperature => LoadKind::Thermal,
        semio_s_artifact_norm_en1999_model::ActionKind::Fire => LoadKind::Accidental,
    }
}

/// 🧭️ The infix of a load glyph id between its owner and its action row: `<owner>:load-<action>:<tag>`.
pub const ACTION_MARK: &str = ":load-";
const FORCE_PER_METRE: f64 = 1.0e5;
const MOMENT_PER_METRE: f64 = 1.0e5;

/// 🏹 The magnitude grips of the nested action rows of an owner (`<collection>[id=OWNER].actions[id=ACTION].nK|vZK|mYK|qKLine`): one per drawn glyph, standing at the glyph point and reading 100 kN, 100 kNm or 100 kN of line load per metre of drag.
pub fn action_grips(owner: &str, collection: &str, actions: &[MemberAction], frame: &Frame, length: f64, depth: f64) -> Vec<Handle> {
    let mut out = Vec::new();
    for (index, action) in actions.iter().enumerate() {
        let shift = 0.2 * (index % 4) as f64;
        let end = along(&along(&frame.origin, &frame.x, length), &frame.y, shift);
        let mid = along(&along(&along(&frame.origin, &frame.x, length / 2.0), &frame.z, depth / 2.0), &frame.y, shift);
        let path = |field: &str| format!("{collection}[id={owner}].actions[id={}].{field}", action.id);
        let entity = |tag: &str| EntityRef::new(EntityKind::Load, format!("{owner}{ACTION_MARK}{}:{tag}", action.id), format!("{collection}[id={owner}]"));
        let name = |en: &str, de: &str| loc(&format!("{en} ({})", action.id), &format!("{de} ({})", action.id));
        let arrow = super::scene::projection_kit::handle_kit::load_arrow;
        if action.n_k.abs() > 1e-9 {
            out.push(arrow(&entity("n"), "magnitude", mul(&frame.x, if action.n_k > 0.0 { -1.0 } else { 1.0 }), &end, path("nK"), action.n_k, FORCE_PER_METRE, "N", name("Axial force N_k", "Normalkraft N_k")));
        }
        if action.v_z_k.abs() > 1e-9 {
            out.push(arrow(&entity("vz"), "magnitude", mul(&frame.z, if action.v_z_k > 0.0 { -1.0 } else { 1.0 }), &mid, path("vZK"), action.v_z_k, FORCE_PER_METRE, "N", name("Shear force V_z,k", "Querkraft V_z,k")));
        }
        if action.m_y_k.abs() > 1e-9 {
            out.push(arrow(&entity("my"), "magnitude", frame.z.clone(), &end, path("mYK"), action.m_y_k, MOMENT_PER_METRE, "Nm", name("Bending moment M_y,k", "Biegemoment M_y,k")));
        }
        let line = (action.g_k_line + action.q_k_line) * length;
        if action.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl && line.abs() > 1e-9 {
            let at = along(&mid, &frame.z, 0.15);
            let mut grip = arrow(&entity("udl"), "magnitude", mul(&frame.z, -1.0), &at, path("qKLine"), line, FORCE_PER_METRE, "N/m", name("Line load q_k", "Streckenlast q_k"));
            let per_metre = FORCE_PER_METRE / length.max(1e-9);
            grip.writes[0].map = Affine { base: action.q_k_line, k: line.signum() * per_metre };
            grip.writes[0].quantise = Some(Quantise::Round { step: per_metre / 200.0 });
            out.push(grip);
        }
    }
    out
}

pub fn action_glyphs(owner: &str, collection: &'static str, actions: &[MemberAction], frame: &Frame, length: f64, depth: f64, out: &mut Out) {
    let grips = action_grips(owner, collection, actions, frame, length, depth);
    for (index, action) in actions.iter().enumerate() {
        let kind = load_kind(action.kind);
        let case = LoadCaseRef { id: action.id.clone(), kind, psi: None };
        let shift = 0.2 * (index % 4) as f64;
        let end = along(&along(&frame.origin, &frame.x, length), &frame.y, shift);
        let mid = along(&along(&along(&frame.origin, &frame.x, length / 2.0), &frame.z, depth / 2.0), &frame.y, shift);
        let push = |tag: &str, glyph: LoadGlyph, en: String, de: String, at: Vec3, out: &mut Out| {
            let name = loc(&en, &de);
            out.push(Spec { kind: EntityKind::Load, id: format!("{owner}:load-{}:{tag}", action.id), collection, geometry: Geometry::Load(glyph), role: MaterialRole::Load { kind }, binding: None, detail: name.clone(), name, layer: LAYER_LOADS, handles: grips.iter().filter(|h| h.id.starts_with(&format!("{owner}{ACTION_MARK}{}:{tag}:", action.id))).map(|h| h.id.clone()).collect(), deformation: None, style: None }, &[at]);
        };
        if action.n_k.abs() > 1e-9 {
            let direction = mul(&frame.x, if action.n_k > 0.0 { -1.0 } else { 1.0 });
            push("n", LoadGlyph::Point { at: end.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, action.n_k.abs()), case: case.clone() }, format!("N_k {:.1} kN ({})", action.n_k / 1e3, action.id), format!("N_k {:.1} kN ({})", action.n_k / 1e3, action.id), end.clone(), out);
        }
        if action.v_z_k.abs() > 1e-9 {
            let direction = mul(&frame.z, if action.v_z_k > 0.0 { -1.0 } else { 1.0 });
            push("vz", LoadGlyph::Point { at: mid.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, action.v_z_k.abs()), case: case.clone() }, format!("V_z,k {:.1} kN ({})", action.v_z_k / 1e3, action.id), format!("V_z,k {:.1} kN ({})", action.v_z_k / 1e3, action.id), mid.clone(), out);
        }
        if action.m_y_k.abs() > 1e-9 {
            push("my", LoadGlyph::Moment { at: end.clone(), axis: frame.y.clone(), magnitude: Quantity::new(QuantityKind::Moment, action.m_y_k.abs()), case: case.clone() }, format!("M_y,k {:.2} kNm ({})", action.m_y_k / 1e3, action.id), format!("M_y,k {:.2} kNm ({})", action.m_y_k / 1e3, action.id), end.clone(), out);
        }
        let line = (action.g_k_line + action.q_k_line) * length;
        if action.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl && line.abs() > 1e-9 {
            let at = along(&mid, &frame.z, 0.15);
            push("udl", LoadGlyph::Point { at: at.clone(), direction: mul(&frame.z, -1.0), magnitude: Quantity::new(QuantityKind::Force, line.abs()), case }, format!("(g_k + q_k)·L = {:.1} kN ({})", line / 1e3, action.id), format!("(g_k + q_k)·L = {:.1} kN ({})", line / 1e3, action.id), at, out);
        }
    }
}

fn support_entity(member: &str, tag: &str, at: Vec3, kind: SupportKind, name: LocalizedCopy, out: &mut Out) {
    out.push(
        Spec {
            kind: EntityKind::Support,
            id: format!("{member}:{tag}"),
            collection: "members",
            geometry: Geometry::Support(SupportGeometry { kind, frame: Frame { origin: at.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, at: at.clone() }),
            role: MaterialRole::Support,
            binding: None,
            detail: name.clone(),
            name,
            layer: LAYER_SUPPORTS,
            handles: Vec::new(),
            deformation: None,
            style: None,
        },
        &[at],
    );
}

fn supports(member: &AluminiumMember, depth: f64, frame: &Frame, out: &mut Out) {
    if is_upright(member) {
        support_entity(&member.id, "support-a", frame.origin.clone(), SupportKind::Fixed, loc("Fixed base", "Einspannung"), out);
        return;
    }
    let drop = |p: &Vec3| along(p, &Vec3::Z, -depth / 2.0);
    let end = along(&frame.origin, &frame.x, member.length);
    support_entity(&member.id, "support-a", drop(&frame.origin), SupportKind::Pinned, loc("Pinned support", "Gelenklager"), out);
    if member.support == SupportCondition::Continuous {
        let mid = along(&frame.origin, &frame.x, member.length / 2.0);
        support_entity(&member.id, "support-mid", drop(&mid), SupportKind::Roller { normal: Vec3::Z }, loc("Intermediate support", "Zwischenlager"), out);
    }
    support_entity(&member.id, "support-b", drop(&end), SupportKind::Roller { normal: Vec3::Z }, loc("Roller support", "Rollenlager"), out);
}

fn restraints(member: &AluminiumMember, depth: f64, frame: &Frame, out: &mut Out) {
    if is_upright(member) {
        return;
    }
    let distances: Vec<f64> = if member.restrained_ltb {
        [0.25, 0.5, 0.75].iter().map(|f| f * member.length).collect()
    } else if member.ltb_length > 1e-6 && member.ltb_length < member.length - 1e-6 {
        (1..=MAX_RESTRAINTS).map(|k| k as f64 * member.ltb_length).take_while(|d| *d < member.length - 1e-6).collect()
    } else {
        Vec::new()
    };
    for (k, distance) in distances.iter().enumerate() {
        let at = along(&along(&frame.origin, &frame.x, *distance), &frame.z, depth / 2.0);
        support_entity(&member.id, &format!("restraint-{}", k + 1), at, SupportKind::Roller { normal: frame.y.clone() }, loc("Lateral restraint", "Seitliche Halterung"), out);
    }
}

fn haz_entities(member: &AluminiumMember, section: &AluminiumSection, alloy: &super::checks::part_1_1::AlloyProps, frame: &Frame, end: &Vec3, out: &mut Out) {
    let bands = facts::haz_bands(section, alloy);
    for (k, band) in bands.iter().enumerate() {
        let size = (band.dy + 2.0 * HAZ_SKIN, band.dz + 2.0 * HAZ_SKIN);
        let name = loc(&format!("Heat-affected zone {} of {}", k + 1, member.id), &format!("Wärmeeinflusszone {} von {}", k + 1, member.id));
        let extent = if band.thin_z { band.dy } else { band.dz };
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: format!("{}:haz-{}", member.id, k + 1),
                collection: "members",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[frame.origin.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: SectionShape::Polygon { polygon: rect_polygon((band.y, band.z), size) },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Weld,
                binding: None,
                detail: loc(&format!("b_haz = {:.0} mm, ρ_o,haz = {:.2}, ρ_u,haz = {:.2}", extent * 5e2, alloy.rho_o_haz, alloy.rho_u_haz), &format!("b_haz = {:.0} mm, ρ_o,haz = {:.2}, ρ_u,haz = {:.2}", extent * 5e2, alloy.rho_o_haz, alloy.rho_u_haz)),
                name,
                layer: LAYER_HAZ,
                handles: Vec::new(),
                deformation: None,
                style: Some((StyleState::Normal, 0.55, Some(Hatch::Dots))),
            },
            &[frame.origin.clone(), end.clone()],
        );
    }
    if let Some(band) = bands.first() {
        let mid = along(&frame.origin, &frame.x, member.length * 0.15);
        let at = point_on_section(frame, &mid, band.y, band.z);
        let extent = if band.thin_z { band.dy } else { band.dz };
        out.annotate(
            Annotation::Leader { id: format!("{}:haz-label", member.id), anchor: at.clone(), text: loc(&format!("HAZ b_haz = {:.0} mm, ρ_o,haz = {:.2}", extent * 1e3 / 2.0, alloy.rho_o_haz), &format!("WEZ b_haz = {:.0} mm, ρ_o,haz = {:.2}", extent * 1e3 / 2.0, alloy.rho_o_haz)), offset_px: vec![24, -24] },
            &at,
        );
    }
}

/// 🦴️ Highlight of every edge stiffener (lip) of the member's outstands: a lip bar along the member, coloured by its own flexural check (`en1999.6.7.7.edge.<member>.<element>`, EN 1999-1-1 §6.7.7) with a label of λ̄_s; flanges of the parametric sections only.
fn stiffener_entities(input: &SceneInput, layout: &Layout, member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, end: &Vec3, out: &mut Out) {
    for (index, el) in section.elements.iter().enumerate().filter(|(_, e)| e.outstand && e.edge_stiffener_depth > 0.0 && e.edge_stiffener_thickness > 0.0) {
        let Some((y, z, false)) = handles::weld_anchor(section, index) else { continue };
        let check_id = format!("en1999.6.7.7.edge.{}.{}", member.id, el.id);
        let binding = projection_kit::binding_of_subject(input.report, &member.id, &[check_id.as_str()]);
        let sign = if z < 0.0 { -1.0 } else { 1.0 };
        let (tip, ts, d) = (y - el.weld_position + el.width, el.edge_stiffener_thickness, el.edge_stiffener_depth);
        let centre = (tip - 0.5 * ts, z + sign * (0.5 * el.thickness + 0.5 * d));
        let id = format!("{}:edge-{}", member.id, el.id);
        let name = loc(&format!("Edge stiffener of {}: {:.0} × {:.0} mm", el.id, d * 1e3, ts * 1e3), &format!("Randsteife von {}: {:.0} × {:.0} mm", el.id, d * 1e3, ts * 1e3));
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: id.clone(),
                collection: "members",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[frame.origin.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: SectionShape::Polygon { polygon: rect_polygon(centre, (ts, d)) },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                detail: name.clone(),
                name,
                layer: LAYER_STIFFENERS,
                handles: handles::ids(input.document, layout, &id),
                deformation: None,
                style: Some((StyleState::Selected, 0.9, Some(Hatch::Diagonal))),
            },
            &[frame.origin.clone(), end.clone()],
        );
        let at = point_on_section(frame, &along(&frame.origin, &frame.x, member.length * 0.5), centre.0, centre.1 + sign * 0.5 * d);
        let words = projection_kit::check_with(input.report, &check_id).map(|c| c.explanation.clone());
        if let Some(words) = words {
            out.annotate(Annotation::Leader { id: format!("{id}:label"), anchor: at.clone(), text: loc(&format!("Edge stiffener {}: {}", el.id, words.en), &format!("Randsteife {}: {}", el.id, words.de)), offset_px: vec![24, -24] }, &at);
        }
        if binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail) {
            out.annotate(Annotation::FailureGlyph { id: format!("{id}:failure"), at: entity_ref(EntityKind::Member, &id, "members"), mode: FailureMode::Other, extent: None }, &at);
        }
    }
}

fn run_geometry(d: &En1999Snapshot, member: &AluminiumMember, section: &AluminiumSection, run: &WeldRun, frame: &Frame) -> Option<(Vec3, Vec3, f64, f64, bool, PlateElement)> {
    let element = section.elements.iter().find(|e| e.id == run.element_id)?.clone();
    let rings = checks::rings(&d.outline_vertices, &section.id);
    let (y, z, web) = if checks::outline_props(&rings).is_some() {
        let ring = rings.iter().find(|r| r.elements.iter().any(|e| *e == run.element_id))?;
        let i = ring.elements.iter().position(|e| *e == run.element_id)?;
        let (p, q) = (ring.points[i], ring.points[(i + 1) % ring.points.len()]);
        let len = ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt().max(1e-9);
        let t = (run.offset / len).clamp(0.0, 1.0);
        (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1), (q.1 - p.1).abs() > (q.0 - p.0).abs())
    } else {
        let (_, resolved) = checks::resolved(member, section, &[run]);
        let index = resolved.elements.iter().position(|e| e.id == run.element_id)?;
        handles::weld_anchor(&resolved, index)?
    };
    let at = |x: f64, dy: f64, dz: f64| along(&along(&along(&frame.origin, &frame.x, x), &frame.y, y + dy), &frame.z, z + dz);
    let (a, b) = if run.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse {
        let half = element.width / 2.0;
        let (dy, dz) = if web { (0.0, half) } else { (half, 0.0) };
        (at(run.start, -dy, -dz), at(run.start, dy, dz))
    } else {
        (at(run.start, 0.0, 0.0), at((run.start + run.length).min(member.length), 0.0, 0.0))
    };
    Some((a, b, y, z, web, element))
}

fn weld_run_entities(input: &SceneInput, member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, out: &mut Out) {
    let d = input.document;
    for run in d.weld_runs.iter().filter(|r| r.member_id == member.id) {
        let Some((a, b, y, z, web, element)) = run_geometry(d, member, section, run, frame) else { continue };
        let throat = run.throat.max(1e-3);
        let b_haz = checks::haz_extent(run, element.thickness);
        let binding = out.binding(input, &[run.id.as_str()]);
        out.push(
            Spec {
                kind: EntityKind::Fastener,
                id: run.id.clone(),
                collection: "weldRuns",
                geometry: Geometry::Fastener(FastenerGeometry { kind: FastenerKind::WeldRun { throat, weld: WeldKind::Fillet, path: polyline(&[a.clone(), b.clone()]), pitch: None }, axis: Segment3 { from: a.clone(), to: b.clone() }, diameter: throat, head: HeadShape::None, pattern: None, hole_clearance: None }),
                role: MaterialRole::Weld,
                binding,
                name: loc(&format!("Weld run {}", run.id), &format!("Schweißnaht {}", run.id)),
                detail: loc(&format!("{:?} {:?} weld on {}, a = {:.1} mm, b_haz = {:.1} mm", run.process, run.direction, run.element_id, throat * 1e3, b_haz * 1e3), &format!("{:?}-{:?}-Naht an {}, a = {:.1} mm, b_haz = {:.1} mm", run.process, run.direction, run.element_id, throat * 1e3, b_haz * 1e3)),
                layer: LAYER_WELDS,
                handles: handles::ids(d, &Layout::of(d, input.placements), &run.id),
                deformation: None,
                style: None,
            },
            &[a.clone(), b.clone()],
        );
        let rho = d
            .materials
            .iter()
            .find(|m| m.id == member.material_id)
            .and_then(|m| checks::part_1_1::resolve_alloy_for_thickness(&m.designation, element.thickness))
            .map_or_else(String::new, |a| format!(", ρ_o,haz = {:.2}, ρ_u,haz = {:.2}", a.rho_o_haz, a.rho_u_haz));
        let skin = 2.0 * HAZ_SKIN;
        let (x0, x1) = if run.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse { ((run.start - b_haz).max(0.0), (run.start + b_haz).min(member.length)) } else { (run.start, (run.start + run.length).min(member.length)) };
        let size = match (run.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse, web) {
            (true, true) => (element.thickness + skin, element.width),
            (true, false) => (element.width, element.thickness + skin),
            (false, true) => (element.thickness + skin, 2.0 * b_haz),
            (false, false) => (2.0 * b_haz, element.thickness + skin),
        };
        let (span_from, span_to) = (along(&frame.origin, &frame.x, x0), along(&frame.origin, &frame.x, x1));
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: format!("{}:haz", run.id),
                collection: "weldRuns",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[span_from.clone(), span_to.clone()]),
                    frame: Frame { origin: span_from.clone(), ..frame.clone() },
                    section: SectionShape::Polygon { polygon: rect_polygon((y, z), size) },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Weld,
                binding: None,
                name: loc(&format!("Heat-affected zone of run {}", run.id), &format!("Wärmeeinflusszone der Naht {}", run.id)),
                detail: loc(&format!("b_haz = {:.1} mm per side{rho}", b_haz * 1e3), &format!("b_haz = {:.1} mm je Seite{rho}", b_haz * 1e3)),
                layer: LAYER_HAZ,
                handles: Vec::new(),
                deformation: None,
                style: Some((StyleState::Normal, 0.55, Some(Hatch::Dots))),
            },
            &[span_from.clone(), span_to],
        );
        if !rho.is_empty() {
            let text = loc(&format!("HAZ of {}{rho}", run.id), &format!("WEZ von {}{rho}", run.id));
            out.annotate(Annotation::Leader { id: format!("{}:haz-rho", run.id), anchor: span_from.clone(), text, offset_px: vec![24, -24] }, &span_from);
        }
    }
}

fn effective_entity(input: &SceneInput, member: &AluminiumMember, section: &AluminiumSection, alloy: &super::checks::part_1_1::AlloyProps, frame: &Frame, end: &Vec3, out: &mut Out) {
    let eta = facts::eta_of(input.report, &member.id, section, alloy);
    let local = facts::local_bands(section, alloy, eta);
    for (k, band) in local.iter().enumerate() {
        let mid = along(&frame.origin, &frame.x, member.length * (0.35 + 0.1 * (k % 3) as f64));
        let at = point_on_section(frame, &mid, band.y, band.z);
        out.annotate(
            Annotation::Leader { id: format!("{}:rho-{}", member.id, k + 1), anchor: at.clone(), text: loc(&format!("ρ_c = {:.2}, t_eff = {:.2} mm", band.factor, band.effective().0.min(band.effective().1) * 1e3), &format!("ρ_c = {:.2}, t_eff = {:.2} mm", band.factor, band.effective().0.min(band.effective().1) * 1e3)), offset_px: vec![-28, -28] },
            &at,
        );
    }
    let all = facts::effective_bands(section, alloy, eta);
    if !all.iter().any(|b| b.factor < 1.0 - 1e-9) {
        return;
    }
    let Some(shape) = effective_shape(&all) else { return };
    let overlay = local.iter().min_by(|a, b| a.factor.total_cmp(&b.factor)).map(|band| {
        let (direction, width) = if band.thin_z { (&frame.z, band.dy) } else { (&frame.y, band.dz) };
        DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: 2, frequency: None, participation: 1.0, scale: 0.004 * member.length, samples: local_mode(direction, member.length, width) }
    });
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: format!("{}:effective", member.id),
            collection: "members",
            geometry: Geometry::Member(MemberGeometry {
                axis: polyline(&[frame.origin.clone(), end.clone()]),
                frame: frame.clone(),
                section: shape,
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: EndRelease::Rigid,
                end_release: EndRelease::Rigid,
                stations: Vec::new(),
            }),
            role: MaterialRole::Aluminium,
            binding: None,
            name: loc(&format!("Effective section of {}", member.id), &format!("Wirksamer Querschnitt von {}", member.id)),
            detail: loc("Plates thinned by ρ_c (local buckling) and ρ_o,haz (welds), EN 1999-1-1 6.1.5 and 6.1.6", "Bleche abgemindert um ρ_c (lokales Beulen) und ρ_o,haz (Schweißnähte), EN 1999-1-1 6.1.5 und 6.1.6"),
            layer: LAYER_EFFECTIVE,
            handles: Vec::new(),
            deformation: overlay,
            style: Some((StyleState::Ghost, 0.35, None)),
        },
        &[frame.origin.clone(), end.clone()],
    );
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for member in &d.members {
        if !out.own("members", &member.id) {
            continue;
        }
        let (Some(pose), Some(section), Some(material)) = (layout.member(&member.id), d.sections.iter().find(|s| s.id == member.section_id), d.materials.iter().find(|m| m.id == member.material_id)) else { continue };
        if member.length <= 1e-6 {
            continue;
        }
        let frame = pose.frame.clone();
        let end = along(&frame.origin, &frame.x, member.length);
        let (depth, section_wide) = section_extent(d, section);
        let binding = out.binding(input, &dependents(d, &member.id));
        let Shown { overlay, shapes, factor, modes } = shown(input, member, section, &frame, depth);
        let stations = envelope(input.report.checks.iter().filter(|c| c.subject.entity_id == member.id));
        let binding = binding.map(|b| UtilisationBinding { per_station: Some(stations.clone()).filter(|v| !v.is_empty()).or(b.per_station), ..b });
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: member.id.clone(),
                collection: "members",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[frame.origin.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: section_shape_of(d, section),
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: if is_upright(member) { EndRelease::Free } else { EndRelease::Rigid },
                    stations: stations.iter().map(|s| s.at).collect(),
                }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                name: loc(&format!("Member {}", member.id), &format!("Bauteil {}", member.id)),
                detail: loc(&format!("{} {:?}, {:.2} m, {}", material.designation, section.kind, member.length, member.support.code()), &format!("{} {:?}, {:.2} m, {}", material.designation, section.kind, member.length, member.support.code())),
                layer: LAYER_MEMBERS,
                handles: handles::ids(d, layout, &member.id),
                deformation: overlay,
                style: None,
            },
            &[frame.origin.clone(), end.clone()],
        );
        let free = if is_upright(member) { EndRelease::Free } else { EndRelease::Rigid };
        for (mode, shape) in shapes {
            tracks::push_mode(out, mode, &member.id, section_shape_of(d, section), &frame, member.length, shape, free);
        }
        let entity = entity_ref(EntityKind::Member, &member.id, "members");
        let mid = along(&frame.origin, &frame.x, member.length / 2.0);
        let label_at = along(&mid, &frame.z, depth / 2.0 + 0.15);
        out.annotate(Annotation::Label { id: format!("{}:label", member.id), anchor: label_at.clone(), text: loc(&format!("{} {}", member.id, material.designation), &format!("{} {}", member.id, material.designation)), tone: tone_of(&binding) }, &label_at);
        let dimension_at = along(&frame.origin, &frame.z, -(depth / 2.0 + 0.4));
        out.annotate(
            Annotation::Dimension {
                id: format!("{}:length", member.id),
                from: dimension_at.clone(),
                to: along(&dimension_at, &frame.x, member.length),
                offset: Vec3::ZERO,
                value: Quantity::new(QuantityKind::Length, member.length),
                binding: Some(FieldBinding::new(format!("members[id={}].length", member.id), Affine::IDENTITY, "m")),
            },
            &dimension_at,
        );
        let depth_from = along(&along(&frame.origin, &frame.y, -(section_wide / 2.0 + 0.1)), &frame.z, -depth / 2.0);
        out.annotate(
            Annotation::Dimension {
                id: format!("{}:depth", member.id),
                from: depth_from.clone(),
                to: along(&depth_from, &frame.z, depth),
                offset: Vec3::ZERO,
                value: Quantity::new(QuantityKind::Length, depth),
                binding: Some(FieldBinding::new(format!("sections[id={}].{}", section.id, if super::checks::part_1_1::is_tube(section) { "outerDiameter" } else { "height" }), Affine::IDENTITY, "m")),
            },
            &depth_from,
        );
        if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", member.id), anchor: entity.clone(), clause }, &mid);
        }
        if let Some((symbol, alpha)) = factor {
            let at = along(&mid, &frame.z, depth / 2.0 + 0.45);
            let text = loc(&format!("α_cr = {alpha:.2} ({symbol}_cr)"), &format!("α_cr = {alpha:.2} ({symbol}_cr)"));
            out.annotate(Annotation::Label { id: format!("{}:alpha-cr", member.id), anchor: at.clone(), text, tone: tone_of(&binding) }, &at);
        }
        for (mode, _) in &modes {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-{mode:?}", member.id), at: entity.clone(), mode: *mode, extent: None }, &mid);
        }
        weld_run_entities(input, member, section, &frame, out);
        if let Some(alloy) = facts::alloy_of(d, &member.material_id, section).filter(|_| !checks::is_free(section)) {
            haz_entities(member, section, &alloy, &frame, &end, out);
            stiffener_entities(input, layout, member, section, &frame, &end, out);
            effective_entity(input, member, section, &alloy, &frame, &end, out);
        }
        supports(member, depth, &frame, out);
        restraints(member, depth, &frame, out);
        action_glyphs(&member.id, "members", &member.actions, &frame, member.length, depth, out);
    }
}
