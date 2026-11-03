//! 🪵️ Members: solid, glulam, LVL and CLT members as extruded rectangular sections with lamella lines, supports and bearings, lateral restraints, end notch void, loads, the char layer and zero-strength layer over the fire duration,
//! the deformed shape or the floor and footbridge vibration mode, and failure glyphs per verification group.

use super::facts::{self, Fire};
use super::kit::*;
use super::layout::{is_upright, Layout};
use super::model::{MemberRole, SupportType, TimberMember, TimberProduct, TimberProperties};
use super::scene::*;
use super::{handles, modes, norm, tracks, SceneInput};

const MAX_RESTRAINTS: usize = 12;
const MODE_AMPLITUDE: f64 = 0.04;
const DEFORMATION_SHOWN_FROM: f64 = 0.5;
const LAMELLA: f64 = 0.04;
const MAX_LAMELLAE: usize = 24;
const PAD: f64 = 0.03;
const CHAR_RGB: (u8, u8, u8) = (38, 32, 30);
const ZSL_RGB: (u8, u8, u8) = (150, 92, 48);
pub(crate) use entities::{FRONT_RGB, FRONT_THICKNESS};
const VOID_RGB: (u8, u8, u8) = (214, 64, 52);

const FAILURE_GROUPS: [(&str, &str, FailureMode); 8] = [
    ("en1995.6.1.6.", "bending", FailureMode::Yielding),
    ("en1995.6.1.7.", "shear", FailureMode::Other),
    ("en1995.6.1.5.", "bearing", FailureMode::Crushing),
    ("en1995.6.2.3.", "tension", FailureMode::Yielding),
    ("en1995.6.3.2.", "buckling", FailureMode::FlexuralBuckling),
    ("en1995.6.5.2.", "notch", FailureMode::Other),
    ("en1995.1-2.", "fire", FailureMode::Other),
    ("en1995.2.", "bridge", FailureMode::Fatigue),
];

/// 🏷️ The product word of a strength class for names and details.
pub(crate) fn product_words(product: Option<TimberProduct>) -> (&'static str, &'static str) {
    match product {
        Some(TimberProduct::Glulam) => ("glulam", "Brettschichtholz"),
        Some(TimberProduct::Lvl) => ("LVL", "Furnierschichtholz"),
        Some(TimberProduct::Clt) => ("CLT", "Brettsperrholz"),
        _ => ("solid timber", "Vollholz"),
    }
}

fn section(m: &TimberMember, product: Option<TimberProduct>, fire: &Option<Fire>) -> SectionShape {
    if let Some(f) = fire {
        return section_shape(f.h_fi, f.b_fi);
    }
    if product != Some(TimberProduct::Glulam) || m.h_m < 2.0 * LAMELLA {
        return section_shape(m.h_m, m.b_m);
    }
    let count = ((m.h_m / LAMELLA).round() as usize).clamp(2, MAX_LAMELLAE);
    let lamella = m.h_m / count as f64;
    SectionShape::Composite {
        parts: (0..count).map(|i| CompositePart { shape: Box::new(section_shape(lamella, m.b_m)), role: MaterialRole::Timber, offset: Vec2::new(0.0, -m.h_m / 2.0 + (i as f64 + 0.5) * lamella) }).collect(),
        rebar: Vec::new(),
    }
}

fn load_kind(kind: semio_s_artifact_norm_en1995_model::ActionKind) -> LoadKind {
    match kind {
        semio_s_artifact_norm_en1995_model::ActionKind::Permanent => LoadKind::Permanent,
        semio_s_artifact_norm_en1995_model::ActionKind::Snow => LoadKind::Snow,
        semio_s_artifact_norm_en1995_model::ActionKind::Wind => LoadKind::Wind,
        semio_s_artifact_norm_en1995_model::ActionKind::Accidental => LoadKind::Accidental,
        semio_s_artifact_norm_en1995_model::ActionKind::Imposed => LoadKind::Variable,
    }
}

fn overlay(input: &SceneInput, m: &TimberMember, props: &TimberProperties, frame: &Frame, length: f64) -> Option<DeformationOverlay> {
    let down = mul(&frame.z, -1.0);
    if let Some(dynamics) = facts::dynamics(input.document.annex, m, props) {
        let mode = modes::first_vertical(m.support, m.span_m, facts::bending_stiffness(m, props), dynamics.linear_mass(m, props))?;
        let samples = mode.shape.iter().map(|&(t, u)| DisplacementSample { at: t, delta: mul(&frame.z, MODE_AMPLITUDE * length * u) }).collect();
        return Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Frequency, mode.frequency)), participation: 1.0, scale: 1.0, samples });
    }
    if is_upright(m) {
        return None;
    }
    let check = worst(input.report, &m.id, "en1995.7.2.wfin")?;
    let shown = worst(input.report, &m.id, "en1995.7.2.").is_some_and(|c| c.utilization >= DEFORMATION_SHOWN_FROM);
    let samples: Vec<DisplacementSample> = check.stations.iter().filter(|s| s.at.len() == 1 && s.is_well_formed()).map(|s| DisplacementSample { at: s.at[0], delta: mul(&down, s.values.first().map_or(0.0, |q| q.value)) }).collect();
    let w = samples.iter().map(|s| s.delta.length()).fold(0.0, f64::max);
    (shown && w > 0.0).then(|| DeformationOverlay { surface: Vec::new(), motion: Motion::Static, load_factor: None, kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale: (MODE_AMPLITUDE * length / w).clamp(1.0, 500.0), samples })
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for member in &d.members {
        if !out.wants(&member.id) {
            continue;
        }
        let Some(pose) = layout.member(&member.id) else { continue };
        if pose.length <= 1e-6 || member.b_m <= 1e-6 || member.h_m <= 1e-6 {
            continue;
        }
        let frame = pose.frame.clone();
        let length = pose.length;
        let end = along(&frame.origin, &frame.x, length);
        let props = facts::properties(member.strength_class.id());
        let product = props.as_ref().map(|p| p.product);
        let fire = props.as_ref().and_then(|p| facts::fire_of(d.annex, member, p));
        let binding = binding_of(input.report, &member.id, "");
        let overlay = props.as_ref().and_then(|p| overlay(input, member, p, &frame, length));
        let (word_en, word_de) = product_words(product);
        let name = loc(&member.label_en, &member.label_de);
        let residual = fire.map_or(String::new(), |f| format!(", residual {:.0} × {:.0} mm after {:.0} min", f.b_fi * 1e3, f.h_fi * 1e3, f.minutes));
        let residual_de = fire.map_or(String::new(), |f| format!(", Restquerschnitt {:.0} × {:.0} mm nach {:.0} min", f.b_fi * 1e3, f.h_fi * 1e3, f.minutes));
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: member.id.clone(),
                collection: "members",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[frame.origin.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: section(member, product, &fire),
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                appearance: None,
                role: MaterialRole::Timber,
                binding: binding.clone(),
                name: name.clone(),
                detail: loc(
                    &format!("{} {word_en}, {:.0} × {:.0} mm, span {:.2} m, service class {}{residual}", member.strength_class.id(), member.b_m * 1e3, member.h_m * 1e3, member.span_m, member.service_class),
                    &format!("{} {word_de}, {:.0} × {:.0} mm, Stützweite {:.2} m, Nutzungsklasse {}{residual_de}", member.strength_class.id(), member.b_m * 1e3, member.h_m * 1e3, member.span_m, member.service_class),
                ),
                layer: LAYER_MEMBERS,
                handles: handles::tagged(d, layout, &member.id, &["span", "width", "depth", "restraint"]),
                deformation: overlay,
            },
            &[frame.origin.clone(), end.clone()],
        );
        let entity = entity_ref(EntityKind::Member, &member.id, "members");
        let mid = along(&frame.origin, &frame.x, length / 2.0);
        annotations(input, member, &frame, length, &binding, &entity, &mid, out);
        if let Some(f) = fire {
            fire_layers(input, layout, member, &frame, length, &f, (LAYER_FIRE, ""), out);
            if let Some(product) = product {
                tracks::char_growth(input, layout, member, product, &frame, length, out);
            }
        }
        supports(member, &frame, length, input, layout, out);
        restraints(member, &frame, out);
        notch(input, member, &frame, layout, out);
        loads(member, &frame, length, out);
    }
}

fn annotations(input: &SceneInput, member: &TimberMember, frame: &Frame, length: f64, binding: &Option<UtilisationBinding>, entity: &EntityRef, mid: &Vec3, out: &mut Out) {
    let (h, b) = (member.h_m, member.b_m);
    let label_at = along(mid, &frame.z, h / 2.0 + 0.2);
    out.annotate(Annotation::Label { id: format!("{}:label", member.id), anchor: label_at.clone(), text: loc(&format!("{} {}", member.label_en, member.strength_class.id()), &format!("{} {}", member.label_de, member.strength_class.id())), tone: tone_of(binding) }, &label_at);
    let spans = if member.support == SupportType::ContinuousTwoSpan { 0.5 } else { 1.0 };
    let from = along(&frame.origin, &frame.z, -(h / 2.0 + 0.35));
    out.annotate(
        Annotation::Dimension { id: format!("{}:span", member.id), from: from.clone(), to: along(&from, &frame.x, length), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, length), binding: Some(FieldBinding::new(format!("members[id={}].spanM", member.id), Affine { base: 0.0, k: spans }, "m")) },
        &from,
    );
    let depth_from = along(&along(&frame.origin, &frame.y, b / 2.0 + 0.12), &frame.z, -h / 2.0);
    out.annotate(
        Annotation::Dimension { id: format!("{}:depth", member.id), from: depth_from.clone(), to: along(&depth_from, &frame.z, h), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, h), binding: Some(FieldBinding::new(format!("members[id={}].hM", member.id), Affine::IDENTITY, "m")) },
        &depth_from,
    );
    let width_from = along(&along(&frame.origin, &frame.z, h / 2.0 + 0.08), &frame.y, -b / 2.0);
    out.annotate(
        Annotation::Dimension { id: format!("{}:width", member.id), from: width_from.clone(), to: along(&width_from, &frame.y, b), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, b), binding: Some(FieldBinding::new(format!("members[id={}].bM", member.id), Affine::IDENTITY, "m")) },
        &width_from,
    );
    if let Some(clause) = binding.as_ref().and_then(|x| clause_of(input.report, &x.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", member.id), anchor: entity.clone(), clause }, mid);
    }
    for (prefix, tag, mode) in FAILURE_GROUPS {
        if worst(input.report, &member.id, prefix).is_some_and(|c| matches!(c.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-{tag}", member.id), at: entity.clone(), mode, extent: None }, mid);
        }
    }
    if member.role == MemberRole::Bridge && member.bridge_crowd_per_m2 > 0.0 {
        let at = along(mid, &frame.z, h / 2.0 + 0.5);
        out.annotate(
            Annotation::Label { id: format!("{}:crowd", member.id), anchor: at.clone(), text: loc(&format!("Pedestrian crowd {:.1} per m²", member.bridge_crowd_per_m2), &format!("Fußgängerdichte {:.1} je m²", member.bridge_crowd_per_m2)), tone: Tone::Neutral },
            &at,
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn fire_layers(input: &SceneInput, layout: &Layout, member: &TimberMember, frame: &Frame, length: f64, fire: &Fire, slot: (&'static str, &str), out: &mut Out) {
    let (b, h) = (member.b_m, member.h_m);
    let front = FRONT_THICKNESS.min(fire.d_char / 2.0);
    let char_inner = ((b - 2.0 * fire.d_char).max(1e-3), (h - 2.0 * fire.d_char).max(1e-3));
    let front_outer = ((b - 2.0 * (fire.d_char - front)).max(1e-3), (h - 2.0 * (fire.d_char - front)).max(1e-3));
    let eff_inner = ((b - 2.0 * fire.d_ef).max(1e-3), (h - 2.0 * fire.d_ef).max(1e-3));
    let end = along(&frame.origin, &frame.x, length);
    let tag = |id: &str, name: LocalizedCopy, detail: LocalizedCopy, polygon: Polygon2, rgb: (u8, u8, u8), opacity: f64, handles: Vec<String>, out: &mut Out| {
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: format!("{}:{id}{}", member.id, slot.1),
                collection: "members",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[frame.origin.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: SectionShape::Polygon { polygon },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                appearance: Some(tinted(rgb, opacity, StyleState::Normal)),
                role: MaterialRole::Neutral,
                binding: None,
                name,
                detail,
                layer: slot.0,
                handles,
                deformation: None,
            },
            &[frame.origin.clone(), end.clone()],
        );
    };
    tag(
        "char",
        loc(&format!("Char layer of {}", member.label_en), &format!("Abbrandschicht von {}", member.label_de)),
        loc(&format!("char depth {:.1} mm after {:.0} min", fire.d_char * 1e3, fire.minutes), &format!("Abbrandtiefe {:.1} mm nach {:.0} min", fire.d_char * 1e3, fire.minutes)),
        hull((b, h), front_outer),
        CHAR_RGB,
        0.92,
        if slot.1.is_empty() { handles::tagged(input.document, layout, &member.id, &["fire-time"]) } else { Vec::new() },
        out,
    );
    tag(
        "front",
        loc(&format!("Char front of {}", member.label_en), &format!("Abbrandfront von {}", member.label_de)),
        loc(&format!("char front at {:.1} mm after {:.0} min, advancing at β_n = {:.2} mm/min", fire.d_char * 1e3, fire.minutes, fire.beta_n * 6e4), &format!("Abbrandfront bei {:.1} mm nach {:.0} min, fortschreitend mit β_n = {:.2} mm/min", fire.d_char * 1e3, fire.minutes, fire.beta_n * 6e4)),
        hull(front_outer, char_inner),
        FRONT_RGB,
        1.0,
        Vec::new(),
        out,
    );
    tag(
        "zsl",
        loc(&format!("Zero-strength layer of {}", member.label_en), &format!("Nullfestigkeitsschicht von {}", member.label_de)),
        loc(&format!("effective depth {:.1} mm, zero-strength layer {:.1} mm", fire.d_ef * 1e3, (fire.d_ef - fire.d_char) * 1e3), &format!("wirksame Tiefe {:.1} mm, Nullfestigkeitsschicht {:.1} mm", fire.d_ef * 1e3, (fire.d_ef - fire.d_char) * 1e3)),
        hull(char_inner, eff_inner),
        ZSL_RGB,
        0.7,
        Vec::new(),
        out,
    );
}

fn support_entity(member: &str, tag: &str, at: Vec3, kind: SupportKind, name: LocalizedCopy, frame: &Frame, out: &mut Out) {
    out.push(
        Spec {
            kind: EntityKind::Support,
            id: format!("{member}:{tag}"),
            collection: "members",
            geometry: Geometry::Support(SupportGeometry { kind, frame: Frame { origin: at.clone(), x: frame.x.clone(), y: frame.y.clone(), z: frame.z.clone() }, at: at.clone() }),
            appearance: None,
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

fn supports(member: &TimberMember, frame: &Frame, length: f64, input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    let drop = |distance: f64| along(&along(&frame.origin, &frame.x, distance), &frame.z, -member.h_m / 2.0);
    let stations: Vec<(f64, SupportKind, LocalizedCopy)> = if is_upright(member) {
        vec![(0.0, SupportKind::Pinned, loc("Pinned base", "Gelenkiger Fuß")), (length, SupportKind::Roller { normal: frame.y.clone() }, loc("Sliding head", "Verschieblicher Kopf"))]
    } else {
        match member.support {
            SupportType::SimplySupported => vec![(0.0, SupportKind::Pinned, loc("Pinned support", "Gelenklager")), (length, SupportKind::Roller { normal: frame.z.clone() }, loc("Roller support", "Rollenlager"))],
            SupportType::Cantilever => vec![(0.0, SupportKind::Fixed, loc("Fixed support", "Einspannung"))],
            SupportType::ContinuousTwoSpan => vec![
                (0.0, SupportKind::Pinned, loc("Pinned support", "Gelenklager")),
                (length / 2.0, SupportKind::Roller { normal: frame.z.clone() }, loc("Interior support", "Mittelauflager")),
                (length, SupportKind::Roller { normal: frame.z.clone() }, loc("Roller support", "Rollenlager")),
            ],
        }
    };
    let bearing = binding_of(input.report, &member.id, "en1995.6.1.5.");
    for (index, (distance, kind, name)) in stations.into_iter().enumerate() {
        let tag = (b'a' + index as u8) as char;
        let at = if is_upright(member) { along(&frame.origin, &frame.x, distance) } else { drop(distance) };
        support_entity(&member.id, &format!("support-{tag}"), at.clone(), kind, name, frame, out);
        if is_upright(member) || member.bearing_length_m <= 1e-6 {
            continue;
        }
        let pad_start = (distance - if distance > length / 2.0 { member.bearing_length_m } else if distance > 0.0 { member.bearing_length_m / 2.0 } else { 0.0 }).max(0.0);
        let corner = along(&along(&along(&frame.origin, &frame.x, pad_start), &frame.y, -member.b_m / 2.0), &frame.z, -member.h_m / 2.0 - PAD);
        out.push(
            Spec {
                kind: EntityKind::Solid,
                id: format!("{}:bearing-{tag}", member.id),
                collection: "members",
                geometry: Geometry::Solid(SolidGeometry { shape: projection_kit::solid::box_from_corner(&Frame { origin: corner.clone(), x: frame.x.clone(), y: frame.y.clone(), z: frame.z.clone() }, Vec3::new(member.bearing_length_m, member.b_m, PAD)) }),
                appearance: None,
                role: MaterialRole::Support,
                binding: bearing.clone(),
                name: loc(&format!("Bearing {} of {}", tag.to_ascii_uppercase(), member.label_en), &format!("Auflager {} von {}", tag.to_ascii_uppercase(), member.label_de)),
                detail: loc(&format!("bearing length {:.0} mm, contact area {:.0} cm²", member.bearing_length_m * 1e3, member.bearing_length_m * member.b_m * 1e4), &format!("Auflagerlänge {:.0} mm, Pressungsfläche {:.0} cm²", member.bearing_length_m * 1e3, member.bearing_length_m * member.b_m * 1e4)),
                layer: LAYER_SUPPORTS,
                handles: handles::tagged(d, layout, &member.id, &["bearing"]),
                deformation: None,
            },
            &[corner],
        );
    }
}

fn restraints(member: &TimberMember, frame: &Frame, out: &mut Out) {
    let spacing = member.lateral_restraint_spacing_m;
    let length = member.span_m;
    if is_upright(member) || spacing <= 1e-6 || spacing >= length - 1e-6 {
        return;
    }
    for k in 1..=MAX_RESTRAINTS {
        let distance = k as f64 * spacing;
        if distance >= length - 1e-6 {
            break;
        }
        let at = along(&along(&frame.origin, &frame.x, distance), &frame.z, member.h_m / 2.0);
        support_entity(&member.id, &format!("restraint-{k}"), at, SupportKind::Roller { normal: frame.y.clone() }, loc("Lateral restraint", "Seitliche Halterung"), frame, out);
    }
}

fn notch(input: &SceneInput, member: &TimberMember, frame: &Frame, layout: &Layout, out: &mut Out) {
    if member.notch_depth_m <= 1e-6 || is_upright(member) {
        return;
    }
    let reach = (member.bearing_length_m / 2.0 + member.notch_distance_m).max(0.02);
    let corner = along(&along(&frame.origin, &frame.y, -member.b_m / 2.0), &frame.z, -member.h_m / 2.0);
    let binding = binding_of(input.report, &member.id, "en1995.6.5.2.").or_else(|| binding_of(input.report, &member.id, "en1995.6.1.7."));
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: format!("{}:notch", member.id),
            collection: "members",
            geometry: Geometry::Solid(SolidGeometry { shape: projection_kit::solid::box_from_corner(&Frame { origin: corner.clone(), x: frame.x.clone(), y: frame.y.clone(), z: frame.z.clone() }, Vec3::new(reach, member.b_m, member.notch_depth_m)) }),
            appearance: Some(tinted(VOID_RGB, 0.45, if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) { StyleState::Error } else { StyleState::Ghost })),
            role: MaterialRole::Neutral,
            binding: binding.clone(),
            name: loc(&format!("End notch of {}", member.label_en), &format!("Endausklinkung von {}", member.label_de)),
            detail: loc(&format!("notch depth {:.0} mm at {:.0} mm from the support, residual depth {:.0} mm", member.notch_depth_m * 1e3, member.notch_distance_m * 1e3, (member.h_m - member.notch_depth_m) * 1e3), &format!("Ausklinkungstiefe {:.0} mm im Abstand {:.0} mm vom Auflager, Resthöhe {:.0} mm", member.notch_depth_m * 1e3, member.notch_distance_m * 1e3, (member.h_m - member.notch_depth_m) * 1e3)),
            layer: LAYER_MEMBERS,
            handles: handles::tagged(input.document, layout, &member.id, &["notch-depth", "notch-distance"]),
            deformation: None,
        },
        &[corner],
    );
    let at = along(&along(&frame.origin, &frame.x, reach), &frame.z, -member.h_m / 2.0 + member.notch_depth_m);
    out.annotate(
        Annotation::Dimension {
            id: format!("{}:notch-depth", member.id),
            from: along(&at, &frame.z, -member.notch_depth_m),
            to: at.clone(),
            offset: Vec3::ZERO,
            value: Quantity::new(QuantityKind::Length, member.notch_depth_m),
            binding: Some(FieldBinding::new(format!("members[id={}].notchDepthM", member.id), Affine::IDENTITY, "m")),
        },
        &at,
    );
}

fn loads(member: &TimberMember, frame: &Frame, length: f64, out: &mut Out) {
    let top = along(&frame.origin, &frame.z, member.h_m / 2.0 + 0.06);
    let down = mul(&frame.z, -1.0);
    for action in &member.actions {
        let kind = load_kind(action.kind);
        let case = LoadCaseRef { id: action.id.clone(), kind, psi: None };
        let push = |tag: &str, glyph: LoadGlyph, name: LocalizedCopy, at: Vec3, out: &mut Out| {
            out.push(
                Spec {
                    kind: EntityKind::Load,
                    id: format!("{}:load-{}{tag}", member.id, action.id),
                    collection: "members",
                    geometry: Geometry::Load(glyph),
                    appearance: None,
                    role: MaterialRole::Load { kind },
                    binding: None,
                    detail: name.clone(),
                    name,
                    layer: LAYER_LOADS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[at],
            );
        };
        if is_upright(member) {
            let force = action.f_point_n + action.q_line_n_per_m * length;
            if force.abs() > 0.0 {
                let at = along(&frame.origin, &frame.x, length);
                push(":point", LoadGlyph::Point { at: at.clone(), direction: mul(&frame.x, -1.0), magnitude: Quantity::new(QuantityKind::Force, force.abs()), case }, loc(&format!("Axial load {}", action.id), &format!("Normalkraft {}", action.id)), at, out);
            }
            continue;
        }
        if action.q_line_n_per_m.abs() > 0.0 {
            let (from, to) = (top.clone(), along(&top, &frame.x, length));
            push("", LoadGlyph::Line { path: polyline(&[from.clone(), to]), direction: down.clone(), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, action.q_line_n_per_m.abs()) }, case: case.clone() }, loc(&format!("Line load {} {:.2} kN/m", action.id, action.q_line_n_per_m / 1e3), &format!("Streckenlast {} {:.2} kN/m", action.id, action.q_line_n_per_m / 1e3)), from, out);
        }
        if action.f_point_n.abs() > 0.0 {
            let x = match member.support {
                SupportType::Cantilever => length,
                SupportType::SimplySupported => length / 2.0,
                SupportType::ContinuousTwoSpan => length / 4.0,
            };
            let at = along(&top, &frame.x, x);
            push(":point", LoadGlyph::Point { at: at.clone(), direction: down.clone(), magnitude: Quantity::new(QuantityKind::Force, action.f_point_n.abs()), case }, loc(&format!("Point load {} {:.2} kN", action.id, action.f_point_n / 1e3), &format!("Einzellast {} {:.2} kN", action.id, action.f_point_n / 1e3)), at, out);
        }
    }
}
