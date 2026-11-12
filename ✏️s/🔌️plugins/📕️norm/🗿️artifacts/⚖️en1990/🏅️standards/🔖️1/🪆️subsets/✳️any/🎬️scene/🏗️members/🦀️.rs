//! 🏗️ Members: simply supported members with their SLS deflection components w_c, w_1, w_2, w_3 (Fig. A1.1), the vibration mode below the minimum frequency, bridge decks with deformation and vibration, supports, failure glyphs and the reliability badges.

use super::scene::projection_kit::{loc, mul, unit, along, entity_ref, clause_of, tone_of, polyline};
use super::combinations::{self, Column};
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::{handles, model, norm, SceneInput};

const MAX_EXAGGERATION: f64 = 200.0;
const PEAK_FRACTION: f64 = 25.0;
const SAMPLE_COUNT: usize = 10;
const DECK_THICKNESS: f64 = 0.25;
const DIMENSION_LANE: f64 = 0.25;
const MIN_DIMENSION: f64 = 1e-6;

/// 🔍️ The exaggeration that shows a deflection `peak` (m) at about L/25, between ×1 and ×200.
pub(crate) fn exaggeration(span: f64, peak: f64) -> f64 {
    if peak <= 1e-9 { 1.0 } else { (span / PEAK_FRACTION / peak).clamp(1.0, MAX_EXAGGERATION) }
}

fn samples(direction: &Vec3, peak: f64, shape: fn(f64) -> f64) -> Vec<DisplacementSample> {
    (0..=SAMPLE_COUNT).map(|i| i as f64 / SAMPLE_COUNT as f64).map(|t| DisplacementSample { at: t, delta: mul(direction, peak * shape(t)) }).collect()
}

/// 📐️ Elastic line of a simply supported prismatic member under uniform load, `w(s) = 16/5 · (s − 2s³ + s⁴)` scaled to a unit peak at midspan; the same shape the SLS deflection stations of the checks follow.
fn elastic_line(t: f64) -> f64 {
    3.2 * t * (1.0 - 2.0 * t * t + t * t * t)
}

fn half_sine(t: f64) -> f64 {
    (std::f64::consts::PI * t).sin()
}

fn overlay(kind: DeformationKind, index: u32, frequency: Option<f64>, participation: f64, span: f64, peak: f64, direction: &Vec3, shape: fn(f64) -> f64) -> DeformationOverlay {
    DeformationOverlay { surface: Vec::new(), motion: if matches!(kind, DeformationKind::Mode) { Motion::Harmonic } else { Motion::Static }, load_factor: None, kind, index, frequency: frequency.map(|f| Quantity::new(QuantityKind::Dimensionless, f)), participation, scale: exaggeration(span, peak.abs()), samples: samples(direction, peak, shape) }
}

fn member_geometry(frame: &Frame, span: f64) -> Geometry {
    let end = along(&frame.origin, &frame.x, span);
    Geometry::Member(MemberGeometry {
        axis: polyline(&[frame.origin.clone(), end]),
        frame: frame.clone(),
        section: section_shape(span),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Pinned,
        end_release: EndRelease::Pinned,
        stations: Vec::new(),
    })
}

fn failure_mode(check_id: &str) -> FailureMode {
    if check_id.starts_with("en1990.str.") || check_id.starts_with("en1990.6.1") {
        FailureMode::Yielding
    } else if check_id.starts_with("en1990.equ.") {
        FailureMode::Overturning
    } else if check_id.starts_with("en1990.geo.") {
        FailureMode::Sliding
    } else if check_id.starts_with("en1990.fat.") {
        FailureMode::Fatigue
    } else {
        FailureMode::Other
    }
}

/// 🏁️ The label naming the combination that governs a member (largest utilisation over every check of the member that belongs to a generated combination); the dot marks that the active combination is the governing one.
fn governing_label(report: &norm::CheckReport, member: &str, columns: &[Column], active: Option<&Column>) -> Option<(LocalizedCopy, Tone)> {
    let (check, column) = report.checks.iter().filter(|c| c.subject.entity_id == member && c.utilization.is_finite()).filter_map(|c| combinations::column_for_check(columns, c).map(|found| (c, found))).max_by(|a, b| a.0.utilization.total_cmp(&b.0.utilization))?;
    let u = check.utilization.max(0.0);
    let dot = if active.is_some_and(|a| a.id == column.id) { "● " } else { "" };
    let (family_en, family_de) = column.family.title();
    let tone = super::scene::utilisation_tone(u);
    Some((loc(&format!("{dot}Governing: {family_en} {} · lead {} · {:.0} %", column.combination.equation.label(), column.leading(), u * 100.0), &format!("{dot}Maßgebend: {family_de} {} · führend {} · {:.0} %", column.combination.equation.label(), column.leading(), u * 100.0)), tone))
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, columns: &[Column], active: Option<&Column>, out: &mut Out) {
    let d = input.document;
    for member in &d.members {
        if !out.wants(&member.id) {
            continue;
        }
        let (Some(pose), true) = (layout.member(&member.id), member.span > 1e-6) else { continue };
        let frame = pose.frame.clone();
        let span = member.span;
        let depth = member_depth(span);
        let end = along(&frame.origin, &frame.x, span);
        let mid = along(&frame.origin, &frame.x, span / 2.0);
        let binding = binding(d, input.report, &dependents(d, &member.id));
        let net = member.deflection_w1 + member.deflection_w2 + member.deflection_w3 - member.precamber;
        let shape = (net.abs() > 1e-9).then(|| overlay(DeformationKind::Deformed, 0, None, 1.0, span, net, &mul(&frame.z, -1.0), elastic_line));
        let name = loc(&member.label_en, &member.label_de);
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: member.id.clone(),
                collection: "members",
                geometry: member_geometry(&frame, span),
                role: MaterialRole::Neutral,
                binding: binding.clone(),
                name: name.clone(),
                detail: loc(&format!("span {span:.2} m, w_net,fin {:.1} mm", net * 1e3), &format!("Stützweite {span:.2} m, w_net,fin {:.1} mm", net * 1e3)),
                layer: LAYER_MEMBERS,
                handles: handles::ids(d, layout, &member.id),
                deformation: shape,
                ghost: false,
            },
            &[frame.origin.clone(), end.clone()],
        );
        let entity = entity_ref(EntityKind::Member, &member.id, "members");
        let label_at = along(&mid, &frame.z, depth / 2.0 + 0.15);
        out.annotate(Annotation::Label { id: format!("{}:label", member.id), anchor: label_at.clone(), text: name, tone: tone_of(&binding) }, &label_at);
        let dimension_at = along(&frame.origin, &frame.z, -(depth / 2.0 + 0.4));
        out.annotate(
            Annotation::Dimension {
                id: format!("{}:span", member.id),
                from: dimension_at.clone(),
                to: along(&dimension_at, &frame.x, span),
                offset: Vec3::ZERO,
                value: Quantity::new(QuantityKind::Length, span),
                binding: Some(FieldBinding::new(format!("members[id={}].span", member.id), Affine::IDENTITY, "m")),
            },
            &dimension_at,
        );
        if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", member.id), anchor: entity.clone(), clause }, &mid);
        }
        if let Some((text, tone)) = governing_label(input.report, &member.id, columns, active) {
            let at = along(&label_at, &frame.z, 0.3);
            out.annotate(Annotation::Label { id: format!("{}:governing", member.id), anchor: at.clone(), text, tone }, &at);
        }
        for (slot, analysis) in d.reliability_analyses.iter().filter(|a| a.member_id == member.id).enumerate() {
            let Some(check) = input.report.checks.iter().find(|c| c.id == format!("en1990.annex-c.form.{}", analysis.id) && c.status != norm::CheckStatus::NotApplicable) else { continue };
            let at = along(&label_at, &frame.z, 0.6 + 0.3 * slot as f64);
            let (beta, target) = (check.computed.value, check.limit.value);
            let text = loc(&format!("β = {beta:.2} (target {target:.2}) · {}", analysis.id), &format!("β = {beta:.2} (Ziel {target:.2}) · {}", analysis.id));
            out.annotate(Annotation::Label { id: format!("{}:reliability:{}", member.id, analysis.id), anchor: at.clone(), text, tone: check_tone(input.report, &check.id) }, &at);
        }
        let mut shown: Vec<FailureMode> = Vec::new();
        for check in input.report.checks.iter().filter(|c| c.subject.entity_id == member.id && c.status == norm::CheckStatus::Fail) {
            let mode = failure_mode(&check.id);
            if !shown.contains(&mode) {
                shown.push(mode);
                out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-{mode:?}", member.id), at: entity.clone(), mode, extent: None }, &mid);
            }
        }
        deflection_stack(input, member, &frame, depth, net, out);
        vibration(member, &frame, out);
        supports(member, depth, &frame, out);
    }
    decks(input, layout, out);
}

fn deflection_stack(input: &SceneInput, member: &model::Member, frame: &Frame, depth: f64, net: f64, out: &mut Out) {
    let scale = exaggeration(member.span, net.abs());
    let mid = along(&frame.origin, &frame.x, member.span / 2.0);
    let soffit = along(&mid, &frame.z, -depth / 2.0);
    let side = member_depth(member.span) / 4.0 + 0.2;
    let mut cumulative = 0.0;
    let parts = [("w1", member.deflection_w1, "deflectionW1"), ("w2", member.deflection_w2, "deflectionW2"), ("w3", member.deflection_w3, "deflectionW3")];
    for (lane, (tag, value, field)) in parts.into_iter().enumerate() {
        if value.abs() * scale < MIN_DIMENSION {
            continue;
        }
        let from = along(&along(&soffit, &frame.y, -(side + DIMENSION_LANE * lane as f64)), &frame.z, -cumulative * scale);
        let to = along(&from, &frame.z, -value * scale);
        cumulative += value;
        out.annotate(
            Annotation::Dimension { id: format!("{}:{tag}", member.id), from, to: to.clone(), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, value), binding: Some(FieldBinding::new(format!("members[id={}].{field}", member.id), Affine::IDENTITY, "m")) },
            &to,
        );
    }
    if member.precamber.abs() * scale >= MIN_DIMENSION {
        let from = along(&soffit, &frame.y, side + DIMENSION_LANE);
        let to = along(&from, &frame.z, member.precamber * scale);
        out.annotate(
            Annotation::Dimension { id: format!("{}:wc", member.id), from, to: to.clone(), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, member.precamber), binding: Some(FieldBinding::new(format!("members[id={}].precamber", member.id), Affine::IDENTITY, "m")) },
            &to,
        );
    }
    if member.deflection_limit_ratio > 0.0 {
        let limit = member.span / member.deflection_limit_ratio;
        let tone = check_tone(input.report, &format!("en1990.a14.deflection.{}", member.id));
        let at = along(&along(&soffit, &frame.z, -(net.max(0.0) * scale + 0.3)), &frame.x, member.span / 4.0);
        out.annotate(
            Annotation::Label {
                id: format!("{}:deflection-limit", member.id),
                anchor: at.clone(),
                text: loc(&format!("w_net,fin = {:.1} mm, limit L/{:.0} = {:.1} mm (×{scale:.0})", net * 1e3, member.deflection_limit_ratio, limit * 1e3), &format!("w_net,fin = {:.1} mm, Grenze L/{:.0} = {:.1} mm (×{scale:.0})", net * 1e3, member.deflection_limit_ratio, limit * 1e3)),
                tone,
            },
            &at,
        );
    }
}

fn vibration(member: &model::Member, frame: &Frame, out: &mut Out) {
    if member.vibration_frequency <= 0.0 || member.vibration_frequency_min <= 0.0 || member.vibration_frequency >= member.vibration_frequency_min {
        return;
    }
    let peak = member.span / PEAK_FRACTION;
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: format!("{}:mode", member.id),
            collection: "members",
            geometry: member_geometry(frame, member.span),
            role: MaterialRole::Neutral,
            binding: None,
            name: loc(&format!("First mode of {} ({:.2} Hz)", member.label_en, member.vibration_frequency), &format!("Erste Eigenform von {} ({:.2} Hz)", member.label_de, member.vibration_frequency)),
            detail: loc(&format!("f₁ = {:.2} Hz below f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min), &format!("f₁ = {:.2} Hz unter f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min)),
            layer: LAYER_MEMBERS,
            handles: Vec::new(),
            deformation: Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Dimensionless, member.vibration_frequency)), participation: 1.0, scale: 1.0, samples: samples(&frame.z, peak, half_sine) }),
            ghost: true,
        },
        &[frame.origin.clone(), along(&frame.origin, &frame.x, member.span)],
    );
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
            ghost: false,
        },
        &[at],
    );
}

fn supports(member: &model::Member, depth: f64, frame: &Frame, out: &mut Out) {
    let end = along(&frame.origin, &frame.x, member.span);
    let drop = |p: &Vec3| along(p, &Vec3::Z, -depth / 2.0);
    support_entity(&member.id, "support-a", drop(&frame.origin), SupportKind::Pinned, loc("Pinned support", "Gelenklager"), out);
    support_entity(&member.id, "support-b", drop(&end), SupportKind::Roller { normal: Vec3::Z }, loc("Roller support", "Rollenlager"), out);
}

fn decks(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for sls in &d.bridge_sls {
        if !out.wants(&sls.id) {
            continue;
        }
        let (Some(pose), Some(member)) = (layout.member(&sls.member_id), d.members.iter().find(|m| m.id == sls.member_id)) else { continue };
        if member.span <= 1e-6 {
            continue;
        }
        let f = &pose.frame;
        let span = member.span;
        let width = (span / 4.0).clamp(3.0, 12.0);
        let centre = along(&along(&f.origin, &f.x, span / 2.0), &f.z, member_depth(span) / 2.0 + DECK_THICKNESS / 2.0);
        let (hl, hw) = (span / 2.0, width / 2.0);
        let outline = Polygon2::solid(vec![Vec2::new(-hl, -hw), Vec2::new(hl, -hw), Vec2::new(hl, hw), Vec2::new(-hl, hw)]);
        let plate = || Geometry::Plate(PlateGeometry { frame: Frame { origin: centre.clone(), x: f.x.clone(), y: f.y.clone(), z: f.z.clone() }, outline: outline.clone(), thickness: DECK_THICKNESS, offset: 0.0, stiffeners: Vec::new() });
        let binding = binding(input.document, input.report, &[sls.id.as_str()]);
        let points = [along(&centre, &f.y, -hw), along(&along(&centre, &f.y, hw), &f.x, hl)];
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: sls.id.clone(),
                collection: "bridgeSls",
                geometry: plate(),
                role: MaterialRole::Concrete,
                binding: binding.clone(),
                name: loc(&format!("Bridge deck of {}", member.label_en), &format!("Brückenfahrbahn von {}", member.label_de)),
                detail: loc(&format!("{span:.1} × {width:.1} m, deflection {:.1} mm, limit {:.1} mm", sls.bridge_deflection * 1e3, sls.bridge_deflection_limit * 1e3), &format!("{span:.1} × {width:.1} m, Durchbiegung {:.1} mm, Grenze {:.1} mm", sls.bridge_deflection * 1e3, sls.bridge_deflection_limit * 1e3)),
                layer: LAYER_DECKS,
                handles: handles::ids(d, layout, &sls.id),
                deformation: (sls.bridge_deflection.abs() > 1e-9).then(|| overlay(DeformationKind::Deformed, 0, None, 1.0, span, sls.bridge_deflection, &mul(&f.z, -1.0), elastic_line)),
                ghost: false,
            },
            &points,
        );
        if sls.deck_acceleration > 0.0 {
            let ratio = if sls.deck_acceleration_limit > 0.0 { sls.deck_acceleration / sls.deck_acceleration_limit } else { 1.0 };
            let frequency = member.vibration_frequency.max(0.0);
            out.push(
                Spec {
                    kind: EntityKind::Plate,
                    id: format!("{}:mode", sls.id),
                    collection: "bridgeSls",
                    geometry: plate(),
                    role: MaterialRole::Concrete,
                    binding: None,
                    name: loc(&format!("Deck vibration of {}", member.label_en), &format!("Fahrbahnschwingung von {}", member.label_de)),
                    detail: loc(&format!("acceleration {:.3} m/s² of {:.3} m/s²", sls.deck_acceleration, sls.deck_acceleration_limit), &format!("Beschleunigung {:.3} m/s² von {:.3} m/s²", sls.deck_acceleration, sls.deck_acceleration_limit)),
                    layer: LAYER_DECKS,
                    handles: Vec::new(),
                    deformation: Some(overlay(DeformationKind::Mode, 1, (frequency > 0.0).then_some(frequency), ratio.clamp(0.0, 1.5), span, span / PEAK_FRACTION, &f.z, half_sine)),
                    ghost: true,
                },
                &points,
            );
        }
        let notes = [
            ("acceleration", sls.deck_acceleration, sls.deck_acceleration_limit, "a", "m/s²", 0.0),
            ("twist", sls.deck_twist, sls.deck_twist_limit, "t", "", 0.5),
            ("deflection", sls.bridge_deflection, sls.bridge_deflection_limit, "δ", "m", 1.0),
        ];
        for (key, value, limit, symbol, unit, lane) in notes {
            if limit <= 0.0 && value <= 0.0 {
                continue;
            }
            let at = along(&along(&centre, &f.x, -hl + span * (0.2 + 0.3 * lane)), &f.z, DECK_THICKNESS / 2.0 + 0.3);
            let text = format!("{symbol} = {value:.4} {unit} (limit {limit:.4})");
            out.annotate(Annotation::Label { id: format!("{}:{key}", sls.id), anchor: at.clone(), text: loc(&text, &text), tone: check_tone(input.report, &format!("en1990.a2.{key}.{}.{}", member.id, sls.id)) }, &at);
        }
    }
}
