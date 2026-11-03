//! 🔩️ Members: beams, columns, braces and ties as extruded section entities with end supports, lateral restraints, length dimension, buckling mode overlay and failure glyphs.

use super::kit::*;
use super::layout::{is_upright, Layout};
use super::scene::*;
use super::tracks::{self, Mode};
use super::{eigen, handles, norm, SceneInput};

const BUCKLING_SHOWN_FROM: f64 = 0.5;
const MAX_RESTRAINTS: usize = 12;

fn release(member_type: &str) -> EndRelease {
    match member_type.to_ascii_lowercase().as_str() {
        "brace" | "tie" => EndRelease::Pinned,
        _ => EndRelease::Rigid,
    }
}

fn largest_value(check: &norm::CheckResult, index: usize) -> Option<f64> {
    check.stations.iter().filter_map(|s| s.values.get(index)).map(|q| q.value).filter(|v| v.is_finite() && *v > 0.0).reduce(f64::max)
}

struct Shown {
    overlay: Option<DeformationOverlay>,
    shapes: Vec<(Mode, DeformationOverlay)>,
    factor: Option<(&'static str, f64)>,
    modes: Vec<(FailureMode, f64, scene::CheckStatus)>,
}

fn overlay_of(kind: DeformationKind, load_factor: Option<f64>, scale: f64, shape: eigen::Eigen) -> DeformationOverlay {
    let mode = matches!(kind, DeformationKind::Mode);
    DeformationOverlay { surface: Vec::new(), motion: if mode { Motion::Harmonic } else { Motion::Static }, load_factor, kind, index: 1, frequency: None, participation: 1.0, scale, samples: shape.samples }
}

fn shown(input: &SceneInput, member: &super::model::SteelMember, frame: &Frame) -> Shown {
    let d = input.document;
    let pick = |prefix: &str| input.report.checks.iter().filter(|c| c.subject.entity_id == member.id && c.id.starts_with(prefix) && c.utilization.is_finite()).max_by(|a, b| a.utilization.total_cmp(&b.utilization));
    let (flexural, ltb, sls) = (pick("en1993.6.3.1.nb."), pick("en1993.6.3.2.mb."), pick("en1993.7.sls."));
    let to_scene = |c: &norm::CheckResult| match c.status {
        norm::CheckStatus::Fail => scene::CheckStatus::Fail,
        norm::CheckStatus::Warning => scene::CheckStatus::Warning,
        norm::CheckStatus::Pass => scene::CheckStatus::Pass,
        norm::CheckStatus::NotApplicable => scene::CheckStatus::NotApplicable,
        norm::CheckStatus::UnverifiedBasis => scene::CheckStatus::Warning,
        norm::CheckStatus::Info => scene::CheckStatus::NotApplicable,
    };
    let modes: Vec<(FailureMode, f64, scene::CheckStatus)> = [(FailureMode::FlexuralBuckling, flexural), (FailureMode::LateralTorsionalBuckling, ltb)]
        .into_iter()
        .filter_map(|(mode, c)| c.map(|c| (mode, c.utilization, to_scene(c))))
        .collect();
    let (Some(section), Some(material)) = (d.sections.iter().find(|s| s.id == member.section_id), d.materials.iter().find(|m| m.id == member.material_id)) else {
        return Shown { overlay: None, shapes: Vec::new(), factor: None, modes };
    };
    let amplitude = eigen::MODE_AMPLITUDE * member.length;
    let governing = modes.iter().max_by(|a, b| a.1.total_cmp(&b.1)).filter(|m| m.1 >= BUCKLING_SHOWN_FROM).map(|m| m.0);
    let flex = flexural.and_then(|c| eigen::flexural(member, section, material, frame).map(|e| (largest_value(c, 0).map(|n| e.factor / n), e)));
    let twist = ltb.and_then(|c| largest_value(c, 0)).and_then(|m| eigen::lateral_torsional(member, section, material, frame, m).map(|e| (Some(e.factor / m), e)));
    let factor = match governing {
        Some(FailureMode::FlexuralBuckling) => flex.as_ref().and_then(|(a, _)| *a).map(|a| ("N", a)),
        Some(FailureMode::LateralTorsionalBuckling) => twist.as_ref().and_then(|(a, _)| *a).map(|a| ("M", a)),
        _ => None,
    };
    let shapes = [(Mode::Flexural, flex), (Mode::LateralTorsional, twist)]
        .into_iter()
        .filter_map(|(mode, found)| found.map(|(alpha, e)| (mode, DeformationOverlay { index: mode.index(), ..overlay_of(DeformationKind::Mode, alpha, amplitude, e) })))
        .collect();
    let overlay = sls.and_then(|c| largest_value(c, 1)).and_then(|m| eigen::deflection(member, section, material, frame, m)).map(|e| overlay_of(DeformationKind::Deformed, None, e.factor, e));
    Shown { overlay, shapes, factor, modes }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for member in &d.members {
        if !out.own("members", &member.id) {
            continue;
        }
        let (Some(pose), Some(section)) = (layout.member(&member.id), d.sections.iter().find(|s| s.id == member.section_id)) else { continue };
        if member.length <= 1e-6 {
            continue;
        }
        let frame = pose.frame.clone();
        let end = along(&frame.origin, &frame.x, member.length);
        let binding = binding(input.report, &dependents(d, &member.id));
        let Shown { overlay, shapes, factor, modes } = shown(input, member, &frame);
        let stations = envelope(input.report.checks.iter().filter(|c| c.subject.entity_id == member.id));
        let binding = binding.map(|b| UtilisationBinding { per_station: Some(stations.clone()).filter(|v| !v.is_empty()).or(b.per_station), ..b });
        let name = loc(&member.label, &format!("Bauteil {}", member.label));
        let release = release(member.member_type.as_str());
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: member.id.clone(),
                collection: "members",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[frame.origin.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: section_shape(section),
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: release,
                    end_release: release,
                    stations: stations.iter().map(|s| s.at).collect(),
                }),
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: name.clone(),
                detail: loc(&format!("{}, {}, {:.2} m", section.designation, member.member_type, member.length), &format!("{}, {}, {:.2} m", section.designation, member.member_type, member.length)),
                layer: LAYER_MEMBERS,
                handles: handles::ids(d, layout, &member.id),
                deformation: overlay,
            },
            &[frame.origin.clone(), end.clone()],
        );
        for (mode, shape) in shapes {
            tracks::push_mode(out, mode, "members", &member.id, &member.label, section_shape(section), &frame, member.length, shape, release);
        }
        let entity = entity_ref(EntityKind::Member, &member.id, "members");
        let mid = along(&frame.origin, &frame.x, member.length / 2.0);
        let label_at = along(&mid, &frame.z, section.h / 2.0 + 0.15);
        out.annotate(Annotation::Label { id: format!("{}:label", member.id), anchor: label_at.clone(), text: loc(&format!("{} {}", member.label, section.designation), &format!("{} {}", member.label, section.designation)), tone: tone_of(&binding) }, &label_at);
        let dimension_at = along(&frame.origin, &frame.z, -(section.h / 2.0 + 0.4));
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
        if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", member.id), anchor: entity.clone(), clause }, &mid);
        }
        if let Some((symbol, alpha)) = factor {
            let at = along(&mid, &frame.z, section.h / 2.0 + 0.45);
            let text = loc(&format!("α_cr = {alpha:.2} ({symbol}_cr)"), &format!("α_cr = {alpha:.2} ({symbol}_cr)"));
            out.annotate(Annotation::Label { id: format!("{}:alpha-cr", member.id), anchor: at.clone(), text, tone: tone_of(&binding) }, &at);
        }
        for (mode, _, _) in modes.iter().filter(|m| matches!(m.2, scene::CheckStatus::Fail | scene::CheckStatus::Warning)) {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-{mode:?}", member.id), at: entity.clone(), mode: *mode, extent: None }, &mid);
        }
        let toughness_id = format!("en1993.1-10.t.{}", member.id);
        if let Some(check) = input.report.checks.iter().find(|c| c.id == toughness_id && matches!(c.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
            let at = along(&mid, &frame.z, -(section.h / 2.0 + 0.15));
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-brittle-fracture", member.id), at: entity.clone(), mode: FailureMode::Other, extent: None }, &mid);
            out.annotate(Annotation::Label { id: format!("{}:brittle-fracture", member.id), anchor: at.clone(), text: loc(&format!("Brittle fracture: {}", check.explanation.en), &format!("Sprödbruch: {}", check.explanation.de)), tone: tone_of(&binding) }, &at);
        }
        supports(member, section.h, &frame, out);
        restraints(member, section, &frame, out);
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
        },
        &[at],
    );
}

fn supports(member: &super::model::SteelMember, depth: f64, frame: &Frame, out: &mut Out) {
    let end = along(&frame.origin, &frame.x, member.length);
    if is_upright(member) {
        support_entity(&member.id, "support-a", frame.origin.clone(), SupportKind::Fixed, loc("Fixed base", "Einspannung"), out);
        return;
    }
    let drop = |p: &Vec3| along(p, &Vec3::Z, -depth / 2.0);
    let pinned = matches!(member.member_type, super::model::SteelMemberType::Brace | super::model::SteelMemberType::Tie);
    support_entity(&member.id, "support-a", drop(&frame.origin), SupportKind::Pinned, loc("Pinned support", "Gelenklager"), out);
    let far = if pinned { SupportKind::Pinned } else { SupportKind::Roller { normal: Vec3::Z } };
    let far_name = if pinned { loc("Pinned support", "Gelenklager") } else { loc("Roller support", "Rollenlager") };
    support_entity(&member.id, "support-b", drop(&end), far, far_name, out);
}

fn restraints(member: &super::model::SteelMember, section: &super::model::SteelSection, frame: &Frame, out: &mut Out) {
    let spacing = member.ltb_restraint_spacing;
    if is_upright(member) || spacing <= 1e-6 || spacing >= member.length - 1e-6 {
        return;
    }
    for k in 1..=MAX_RESTRAINTS {
        let distance = k as f64 * spacing;
        if distance >= member.length - 1e-6 {
            break;
        }
        let at = along(&along(&frame.origin, &frame.x, distance), &frame.z, section.h / 2.0);
        support_entity(&member.id, &format!("restraint-{k}"), at, SupportKind::Roller { normal: frame.y.clone() }, loc("Lateral restraint", "Seitliche Halterung"), out);
    }
}
