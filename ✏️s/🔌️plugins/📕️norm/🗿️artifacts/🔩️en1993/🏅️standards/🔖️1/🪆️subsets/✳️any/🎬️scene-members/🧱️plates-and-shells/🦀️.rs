//! 🧱️ Plated panels, silo shells, tension components, cold-formed members and piles: the special elements of EN 1993-1-3 to -6 that stand in their own rows.

use super::kit::*;
use super::layout::Layout;
use super::model::SteelSection;
use super::scene::*;
use super::{handles, plate_details, SceneInput};

const CABLE_LENGTH: f64 = 3.0;
const COLD_FORMED_LENGTH: f64 = 1.5;
const PARAMETRIC_LENGTH: f64 = 1.0;
const NOMINAL_WIRE_STRENGTH: f64 = 1.0e9;

struct Bar<'a> {
    id: &'a str,
    collection: &'static str,
    from: Vec3,
    to: Vec3,
    section: SectionShape,
    release: EndRelease,
    name: LocalizedCopy,
    detail: LocalizedCopy,
    layer: &'static str,
}

fn emit_bar(input: &SceneInput, layout: &Layout, bar: Bar, out: &mut Out) {
    let axis = Vec3::new(bar.to.x - bar.from.x, bar.to.y - bar.from.y, bar.to.z - bar.from.z);
    let frame = member_frame(&bar.from, &axis);
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: bar.id.to_string(),
            collection: bar.collection,
            geometry: Geometry::Member(MemberGeometry {
                axis: polyline(&[bar.from.clone(), bar.to.clone()]),
                frame,
                section: bar.section,
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: bar.release,
                end_release: bar.release,
                stations: Vec::new(),
            }),
            role: MaterialRole::Steel,
            binding: binding(input.report, &[bar.id]),
            name: bar.name,
            detail: bar.detail,
            layer: bar.layer,
            handles: handles::ids(input.document, layout, bar.id),
            deformation: None,
        },
        &[bar.from, bar.to],
    );
}

fn longitudinal_section(p: &super::model::PlatedPanel) -> SectionShape {
    let (area, inertia) = (p.stiffener_area, p.stiffener_inertia);
    let (h, b) = if area > 0.0 && inertia > 0.0 { ((12.0 * inertia / area).sqrt(), area / (12.0 * inertia / area).sqrt()) } else { (0.08, 0.01) };
    SectionShape::parametric(ProfileKind::Rect, &[("h", h.max(1e-3)), ("b", b.max(1e-3))])
}

fn stiffeners(p: &super::model::PlatedPanel) -> Vec<Stiffener> {
    if p.stiffener_spacing <= 1e-6 || p.stiffener_spacing >= p.b {
        return Vec::new();
    }
    let count = ((p.b - 1e-9) / p.stiffener_spacing).floor() as usize;
    (1..=count.min(40)).map(|k| { let y = -p.b / 2.0 + k as f64 * p.stiffener_spacing; Stiffener { path: Polyline2 { points: vec![Vec2::new(-p.a / 2.0, y), Vec2::new(p.a / 2.0, y)] }, section: longitudinal_section(p) } }).collect()
}

fn stiffened_words(p: &super::model::PlatedPanel) -> (String, String) {
    let longitudinal = stiffeners(p).len();
    let (mut en, mut de) = (String::new(), String::new());
    if longitudinal > 0 {
        en += &format!(", {longitudinal} longitudinal stiffeners");
        de += &format!(", {longitudinal} Längssteifen");
    }
    let (shape_en, shape_de) = match p.stiffener_shape {
        semio_s_artifact_norm_en1993_model::StiffenerShape::None => ("", ""),
        semio_s_artifact_norm_en1993_model::StiffenerShape::Flat => ("flat bar", "Flachstahl"),
        semio_s_artifact_norm_en1993_model::StiffenerShape::Tee => ("tee", "T-Profil"),
        semio_s_artifact_norm_en1993_model::StiffenerShape::Angle => ("angle", "Winkel"),
    };
    if !shape_en.is_empty() {
        en += &format!(", transverse stiffener {shape_en}");
        de += &format!(", Quersteife {shape_de}");
    }
    (en, de)
}

fn parametric_shape(p: &super::model::ParametricSection) -> Option<SectionShape> {
    let (h, b, t, tf) = (p.height, p.width, p.web_thickness, p.flange_thickness);
    match p.shape {
        semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI if t > 0.0 && tf > 0.0 && 2.0 * tf < h && t < b => Some(SectionShape::parametric(ProfileKind::IShape, &[("h", h), ("b", b), ("tw", t), ("tf", tf)])),
        semio_s_artifact_norm_en1993_model::ParametricShape::Rhs if t > 0.0 && 2.0 * t < h.min(b) => Some(SectionShape::parametric(ProfileKind::Rhs, &[("h", h), ("b", b), ("t", t)])),
        semio_s_artifact_norm_en1993_model::ParametricShape::Chs if t > 0.0 && 2.0 * t < h => Some(SectionShape::parametric(ProfileKind::Chs, &[("d", h), ("t", t)])),
        semio_s_artifact_norm_en1993_model::ParametricShape::Angle if t > 0.0 && t < h.min(b) => Some(SectionShape::parametric(ProfileKind::Angle, &[("h", h), ("b", b), ("t", t)])),
        semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI | semio_s_artifact_norm_en1993_model::ParametricShape::Rhs | semio_s_artifact_norm_en1993_model::ParametricShape::Chs | semio_s_artifact_norm_en1993_model::ParametricShape::Angle => None,
    }
}

fn pile_section(sections: &[SteelSection], id: &str) -> Option<SectionShape> {
    sections.iter().find(|s| s.id == id).map(section_shape)
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for panel in d.plated_panels.iter().filter(|p| p.a > 1e-6 && p.b > 1e-6 && p.thickness > 1e-6) {
        if !out.own("platedPanels", &panel.id) {
            continue;
        }
        let o = layout.origin(&panel.id);
        let centre = v3(o.x + panel.a / 2.0, o.y + panel.b / 2.0, o.z);
        let (w, h) = (panel.a / 2.0, panel.b / 2.0);
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: panel.id.clone(),
                collection: "platedPanels",
                geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: centre.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, outline: Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)]), thickness: panel.thickness, offset: 0.0, stiffeners: stiffeners(panel) }),
                role: MaterialRole::Steel,
                binding: binding(input.report, &[panel.id.as_str()]),
                name: loc(&format!("Plated panel {}", panel.id), &format!("Beulfeld {}", panel.id)),
                detail: loc(&format!("{:.2} × {:.2} m, t = {:.0} mm{}", panel.a, panel.b, panel.thickness * 1e3, stiffened_words(panel).0), &format!("{:.2} × {:.2} m, t = {:.0} mm{}", panel.a, panel.b, panel.thickness * 1e3, stiffened_words(panel).1)),
                layer: LAYER_PLATES,
                handles: handles::ids(d, layout, &panel.id),
                deformation: plate_details::deflection(panel).or_else(|| plate_details::shear_mode(panel).filter(|_| panel.shear_force != 0.0)),
            },
            &[o.clone(), v3(o.x + panel.a, o.y + panel.b, o.z)],
        );
    }
    for silo in d.silo_shells.iter().filter(|s| s.radius > 1e-6 && s.depth > 1e-6 && s.thickness > 1e-6) {
        if !out.own("siloShells", &silo.id) {
            continue;
        }
        let o = layout.origin(&silo.id);
        out.push(
            Spec {
                kind: EntityKind::Shell,
                id: silo.id.clone(),
                collection: "siloShells",
                geometry: Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: o.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(silo.radius, 0.0), Vec2::new(silo.radius, silo.depth)] }, sweep: std::f64::consts::TAU }, thickness: silo.thickness }),
                role: MaterialRole::Steel,
                binding: binding(input.report, &[silo.id.as_str()]),
                name: loc(&format!("Silo shell {}", silo.id), &format!("Silo-Schale {}", silo.id)),
                detail: loc(&format!("R = {:.2} m, height {:.2} m, t = {:.0} mm", silo.radius, silo.depth, silo.thickness * 1e3), &format!("R = {:.2} m, Höhe {:.2} m, t = {:.0} mm", silo.radius, silo.depth, silo.thickness * 1e3)),
                layer: LAYER_SHELLS,
                handles: handles::ids(d, layout, &silo.id),
                deformation: None,
            },
            &[v3(o.x - silo.radius, o.y - silo.radius, o.z), v3(o.x + silo.radius, o.y + silo.radius, o.z + silo.depth)],
        );
    }
    for cable in &d.tension_components {
        if !out.own("tensionComponents", &cable.id) {
            continue;
        }
        let o = layout.origin(&cable.id);
        let diameter = (4.0 * cable.f_k.abs() / (std::f64::consts::PI * NOMINAL_WIRE_STRENGTH)).sqrt().max(0.01);
        emit_bar(
            input,
            layout,
            Bar {
                id: &cable.id,
                collection: "tensionComponents",
                from: o.clone(),
                to: along(&o, &Vec3::Z, if cable.cable_length > 0.0 { cable.cable_length } else { CABLE_LENGTH }),
                section: SectionShape::parametric(ProfileKind::Circle, &[("d", diameter)]),
                release: EndRelease::Pinned,
                name: loc(&format!("Tension component {}", cable.id), &format!("Zugglied {}", cable.id)),
                detail: loc(&format!("F_k = {:.0} kN", cable.f_k / 1e3), &format!("F_k = {:.0} kN", cable.f_k / 1e3)),
                layer: LAYER_MEMBERS,
            },
            out,
        );
    }
    for cf in d.cold_formed_members.iter().filter(|c| c.b_bar > 1e-6 && c.thickness > 1e-6 && 2.0 * c.thickness < c.b_bar) {
        if !out.own("coldFormedMembers", &cf.id) {
            continue;
        }
        let o = layout.origin(&cf.id);
        emit_bar(
            input,
            layout,
            Bar {
                id: &cf.id,
                collection: "coldFormedMembers",
                from: o.clone(),
                to: along(&o, &Vec3::X, COLD_FORMED_LENGTH),
                section: SectionShape::parametric(ProfileKind::Cee, &[("h", cf.b_bar), ("b", 0.6 * cf.b_bar), ("t", cf.thickness), ("c", 0.2 * cf.b_bar)]),
                release: EndRelease::Pinned,
                name: loc(&format!("Cold-formed member {}", cf.id), &format!("Kaltprofil {}", cf.id)),
                detail: loc(&format!("b = {:.0} mm, t = {:.1} mm", cf.b_bar * 1e3, cf.thickness * 1e3), &format!("b = {:.0} mm, t = {:.1} mm", cf.b_bar * 1e3, cf.thickness * 1e3)),
                layer: LAYER_MEMBERS,
            },
            out,
        );
    }
    for p in &d.parametric_sections {
        if !out.own("parametricSections", &p.id) {
            continue;
        }
        let Some(shape) = parametric_shape(p) else { continue };
        let o = layout.origin(&p.id);
        emit_bar(
            input,
            layout,
            Bar {
                id: &p.id,
                collection: "parametricSections",
                from: o.clone(),
                to: along(&o, &Vec3::X, PARAMETRIC_LENGTH),
                section: shape,
                release: EndRelease::Pinned,
                name: loc(&format!("Parametric section {}", p.id), &format!("Parametrischer Querschnitt {}", p.id)),
                detail: loc(&format!("{}: h = {:.0} mm, b = {:.0} mm, t = {:.1} mm", p.shape, p.height * 1e3, p.width * 1e3, p.web_thickness * 1e3), &format!("{}: h = {:.0} mm, b = {:.0} mm, t = {:.1} mm", p.shape, p.height * 1e3, p.width * 1e3, p.web_thickness * 1e3)),
                layer: LAYER_MEMBERS,
            },
            out,
        );
    }
    for s in &d.sections {
        if !out.own("sections", &s.id) {
            continue;
        }
        let o = layout.origin(&s.id);
        let words = |name: &str, verb: &str| format!("{name} {}: {verb} h = {:.0} mm, b = {:.0} mm, t_w = {:.1} mm, t_f = {:.1} mm", s.designation, s.h * 1e3, s.b * 1e3, s.tw * 1e3, s.tf * 1e3);
        emit_bar(
            input,
            layout,
            Bar {
                id: &s.id,
                collection: "sections",
                from: o.clone(),
                to: along(&o, &Vec3::X, PARAMETRIC_LENGTH),
                section: section_shape(s),
                release: EndRelease::Pinned,
                name: loc(&format!("Section {}", s.designation), &format!("Querschnitt {}", s.designation)),
                detail: loc(&words("Section", "dimensions"), &words("Querschnitt", "Abmessungen")),
                layer: LAYER_MEMBERS,
            },
            out,
        );
    }
    for pile in d.piles.iter().filter(|p| p.embedded_length > 1e-6) {
        if !out.own("piles", &pile.id) {
            continue;
        }
        let (o, Some(section)) = (layout.origin(&pile.id), pile_section(&d.sections, &pile.section_id)) else { continue };
        let tip = along(&o, &Vec3::Z, -pile.embedded_length);
        emit_bar(
            input,
            layout,
            Bar {
                id: &pile.id,
                collection: "piles",
                from: o.clone(),
                to: tip,
                section,
                release: EndRelease::Rigid,
                name: loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id)),
                detail: loc(&format!("embedded {:.1} m", pile.embedded_length), &format!("Einbindung {:.1} m", pile.embedded_length)),
                layer: LAYER_SHELLS,
            },
            out,
        );
        out.push(
            Spec {
                kind: EntityKind::Support,
                id: format!("{}:head", pile.id),
                collection: "piles",
                geometry: Geometry::Support(SupportGeometry { kind: SupportKind::PileHead, at: o.clone(), frame: Frame { origin: o.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z } }),
                role: MaterialRole::Support,
                binding: None,
                name: loc("Pile head", "Pfahlkopf"),
                detail: loc("Pile head at ground level", "Pfahlkopf in Geländehöhe"),
                layer: LAYER_SUPPORTS,
                handles: Vec::new(),
                deformation: None,
            },
            &[o],
        );
    }
}
