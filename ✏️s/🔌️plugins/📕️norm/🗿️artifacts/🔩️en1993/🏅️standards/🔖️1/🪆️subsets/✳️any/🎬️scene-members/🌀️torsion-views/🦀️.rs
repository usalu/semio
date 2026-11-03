//! 🌀️ Warping torsion and hybrid girders in 3D: the bimoment diagram B(x) along the member, the warping normal stress as a colour field on a tinted member shell, and the flange and web steel grades of a hybrid girder as colour fields.

use super::kit::*;
use super::layout::Layout;
use super::model::{SteelMaterial, SteelMember, SteelSection};
use super::scene::*;
use super::{checks, SceneInput};
use checks::clauses::ends_of;
use checks::clauses::warping_torsion::{warping_torsion, TorsionProfile};

const STATIONS: usize = 40;
const DIAGRAM_AMPLITUDE: f64 = 0.6;
const RIBBON_HALF_WIDTH: f64 = 0.012;
const TINT: f64 = 1.03;
const STRESS_FIELD: &str = "warping-stress";
const GRADE_FIELD: &str = "yield-strength";

struct Solved {
    profile: TorsionProfile,
    torque: f64,
    modulus: f64,
}

fn solve(d: &super::model::En1993Snapshot, member: &SteelMember, section: &SteelSection, material: &SteelMaterial) -> Option<Solved> {
    let effects = checks::combine_member_actions(d, &member.id);
    let torque = checks::uls_rows(&effects).iter().map(|row| row.action.t.abs()).fold(0.0, f64::max);
    if torque <= 0.0 || section.iw <= 0.0 || section.it <= 0.0 || member.length <= 1e-6 || !checks::part_1_1::is_i_section(section.kind) || section.tw <= 0.0 || section.tf <= 0.0 {
        return None;
    }
    let udl = member.torsion_load == semio_s_artifact_norm_en1993_model::TorsionLoad::Udl;
    let l = member.length;
    let load = if udl { torque / l } else { torque };
    let at = member.torque_position.clamp(0.0, 1.0) * l;
    let profile = warping_torsion(l, material.e_modulus * section.iw, material.g_modulus * section.it, load, at, udl, ends_of(member.warping_support), STATIONS)?;
    Some(Solved { profile, torque, modulus: section.iw / (section.b * (section.h - section.tf) / 4.0) })
}

fn i_shape(section: &SteelSection, scale: f64) -> SectionShape {
    SectionShape::parametric(ProfileKind::IShape, &[("h", section.h * scale), ("b", section.b * scale), ("tw", section.tw * scale), ("tf", section.tf * scale)])
}

fn rectangle(y0: f64, z0: f64, y1: f64, z1: f64) -> SectionShape {
    SectionShape::Polygon { polygon: Polygon2::solid(vec![Vec2::new(y0, z0), Vec2::new(y1, z0), Vec2::new(y1, z1), Vec2::new(y0, z1)]) }
}

fn shell(member: &SteelMember, tag: &str, frame: &Frame, section: SectionShape, name: LocalizedCopy, detail: LocalizedCopy, layer: &'static str) -> Spec {
    let end = along(&frame.origin, &frame.x, member.length);
    Spec {
        kind: EntityKind::Member,
        id: format!("{}:{tag}", member.id),
        collection: "members",
        geometry: Geometry::Member(MemberGeometry { axis: polyline(&[frame.origin.clone(), end]), frame: frame.clone(), section, section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Rigid, stations: Vec::new() }),
        role: MaterialRole::Steel,
        binding: None,
        name,
        detail,
        layer,
        handles: Vec::new(),
        deformation: None,
    }
}

fn diagram(input: &SceneInput, member: &SteelMember, section: &SteelSection, frame: &Frame, solved: &Solved, out: &mut Out) {
    let profile = &solved.profile;
    let peak = profile.bimoment.iter().fold(0.0_f64, |m, b| m.max(b.abs()));
    if peak <= 0.0 {
        return;
    }
    let scale = DIAGRAM_AMPLITUDE / peak;
    let curve: Vec<Vec2> = profile.x.iter().zip(&profile.bimoment).map(|(x, b)| Vec2::new(*x, b * scale)).collect();
    let ribbon: Vec<Vec2> = curve.iter().map(|q| Vec2::new(q.x, q.y + RIBBON_HALF_WIDTH)).chain(curve.iter().rev().map(|q| Vec2::new(q.x, q.y - RIBBON_HALF_WIDTH))).collect();
    let origin = along(&frame.origin, &frame.z, section.h / 2.0 + 2.0 * DIAGRAM_AMPLITUDE);
    let plate_frame = Frame { origin: origin.clone(), x: frame.x.clone(), y: frame.z.clone(), z: frame.x.cross(&frame.z) };
    let at_peak = profile.bimoment.iter().zip(&profile.x).max_by(|a, b| a.0.abs().total_cmp(&b.0.abs())).map_or(0.0, |(_, x)| *x);
    let text = format!("B_max = {:.2} kNm² ({:?} supports)", peak / 1e3, member.warping_support);
    let text_de = format!("B_max = {:.2} kNm² (Lagerung {:?})", peak / 1e3, member.warping_support);
    out.push(
        Spec {
            kind: EntityKind::Plate,
            id: format!("{}:bimoment", member.id),
            collection: "members",
            geometry: Geometry::Plate(PlateGeometry { frame: plate_frame, outline: Polygon2::solid(ribbon), thickness: 0.01, offset: 0.0, stiffeners: Vec::new() }),
            role: MaterialRole::Load { kind: LoadKind::Other },
            binding: None,
            name: loc(&format!("Bimoment diagram of {}", member.label), &format!("Bimomentenverlauf von {}", member.label)),
            detail: loc(&text, &text_de),
            layer: LAYER_TORSION,
            handles: Vec::new(),
            deformation: None,
        },
        &[origin.clone(), along(&origin, &frame.x, member.length)],
    );
    let anchor = along(&along(&frame.origin, &frame.x, at_peak), &frame.z, section.h / 2.0 + 2.0 * DIAGRAM_AMPLITUDE + 0.2);
    out.annotate(Annotation::Label { id: format!("{}:bimoment-label", member.id), anchor: anchor.clone(), text: loc(&text, &text_de), tone: tone_of(&binding(input.report, &[member.id.as_str()])) }, &anchor);
}

fn stress_shell(member: &SteelMember, section: &SteelSection, frame: &Frame, solved: &Solved, out: &mut Out) {
    let values: Vec<StationValue> = solved.profile.x.iter().zip(&solved.profile.bimoment).map(|(x, b)| StationValue { at: x / member.length, value: b.abs() / solved.modulus / 1e6 }).collect();
    let peak = values.iter().fold(0.0_f64, |m, v| m.max(v.value));
    let text = format!("σ_w,max = {peak:.1} MPa, T_Ed = {:.2} kNm", solved.torque / 1e3);
    let spec = shell(member, "warping-stress", frame, i_shape(section, TINT), loc(&format!("Warping stress of {}", member.label), &format!("Wölbspannung von {}", member.label)), loc(&text, &text), LAYER_TORSION);
    let field = ScalarField { id: STRESS_FIELD.into(), unit: QuantityKind::Stress, samples: FieldSamples::Station { values } };
    out.push_tuned(spec, &[frame.origin.clone(), along(&frame.origin, &frame.x, member.length)], |entity| {
        entity.appearance.channel = ColourChannel::Fields { active: STRESS_FIELD.into(), fields: vec![field] };
        entity.appearance.opacity = 0.92;
    });
}

fn hybrid(input: &SceneInput, member: &SteelMember, section: &SteelSection, frame: &Frame, out: &mut Out) {
    let d = input.document;
    let (Some(flange), Some(web)) = (d.materials.iter().find(|m| m.id == member.material_id), d.materials.iter().find(|m| m.id == member.web_material_id)) else { return };
    if member.web_material_id.is_empty() || member.web_material_id == member.material_id || section.tw <= 0.0 || section.tf <= 0.0 || 2.0 * section.tf >= section.h {
        return;
    }
    let (half_b, half_tf, inner) = (section.b / 2.0 * TINT, section.tf / 2.0 * TINT, section.h / 2.0 - section.tf);
    let centre = section.h / 2.0 - section.tf / 2.0;
    let parts = [
        ("grade-flange-top", rectangle(-half_b, centre - half_tf, half_b, centre + half_tf), flange, "Top flange", "Obergurt"),
        ("grade-flange-bottom", rectangle(-half_b, -centre - half_tf, half_b, -centre + half_tf), flange, "Bottom flange", "Untergurt"),
        ("grade-web", rectangle(-section.tw / 2.0 * TINT, -inner, section.tw / 2.0 * TINT, inner), web, "Web", "Steg"),
    ];
    for (tag, shape, material, en, de) in parts {
        let text = format!("{}, f_y = {:.0} MPa", material.grade, material.fy / 1e6);
        let spec = shell(member, tag, frame, shape, loc(&format!("{en} of {}: {}", member.label, material.grade), &format!("{de} von {}: {}", member.label, material.grade)), loc(&text, &text), LAYER_GRADES);
        let field = ScalarField { id: GRADE_FIELD.into(), unit: QuantityKind::Stress, samples: FieldSamples::Instance { values: vec![material.fy / 1e6] } };
        out.push_tuned(spec, &[frame.origin.clone(), along(&frame.origin, &frame.x, member.length)], |entity| {
            entity.appearance.channel = ColourChannel::Fields { active: GRADE_FIELD.into(), fields: vec![field] };
        });
    }
    let at = along(&along(&frame.origin, &frame.x, member.length / 2.0), &frame.z, section.h / 2.0 + 0.3);
    let text = format!("flanges {}, web {}", flange.grade, web.grade);
    let text_de = format!("Gurte {}, Steg {}", flange.grade, web.grade);
    out.annotate(Annotation::Label { id: format!("{}:grades", member.id), anchor: at.clone(), text: loc(&text, &text_de), tone: Tone::Neutral }, &at);
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
        if let Some(solved) = solve(d, member, section, material) {
            diagram(input, member, section, &pose.frame, &solved, out);
            stress_shell(member, section, &pose.frame, &solved, out);
        }
        hybrid(input, member, section, &pose.frame, out);
    }
}
