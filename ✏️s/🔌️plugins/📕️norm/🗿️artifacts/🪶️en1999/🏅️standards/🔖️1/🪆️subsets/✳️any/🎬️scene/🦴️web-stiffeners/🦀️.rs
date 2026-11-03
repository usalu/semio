//! 🦴️ Transverse web stiffeners of plate girders: a flat-bar plate on the web (two plates for a double-sided stiffener) at its position along the girder, coloured by its own checks (`en1999.6.7.6.web-*.<id>`, EN 1999-1-1 §6.7.6) with a label and a failure glyph; position, width and thickness grips live in [`super::handles`].

use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::{handles, SceneInput};

/// 🪞️ Suffix of the scene id of the second plate of a double-sided stiffener; it carries no handles of its own.
pub const MIRROR_MARK: &str = ":mirror";

fn plate(centre: (f64, f64), size: (f64, f64)) -> Polygon2 {
    let (w, h) = (size.0 / 2.0, size.1 / 2.0);
    Polygon2::solid(vec![Vec2::new(centre.0 - w, centre.1 - h), Vec2::new(centre.0 + w, centre.1 - h), Vec2::new(centre.0 + w, centre.1 + h), Vec2::new(centre.0 - w, centre.1 + h)])
}

/// 🦴️ The plates of every transverse stiffener whose girder, section and dimensions are usable.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for st in &d.transverse_stiffeners {
        let Some(member) = d.members.iter().find(|m| m.id == st.girder_id) else { continue };
        let (Some(pose), Some(section)) = (layout.member(&member.id), d.sections.iter().find(|s| s.id == member.section_id)) else { continue };
        let h_w = section.height - 2.0 * section.flange_thickness;
        if matches!(section.kind, semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Free) || h_w <= 1e-6 || section.web_thickness <= 1e-6 || st.width <= 1e-6 || st.thickness <= 1e-6 || member.length <= 1e-6 {
            continue;
        }
        if !out.own("transverseStiffeners", &st.id) {
            continue;
        }
        let f = &pose.frame;
        let binding = out.binding(input, &[st.id.as_str()]);
        let at = |x: f64, y: f64, z: f64| along(&along(&along(&f.origin, &f.x, x), &f.y, y), &f.z, z);
        let (start, end) = (at((st.position - 0.5 * st.thickness).max(0.0), 0.0, 0.0), at((st.position + 0.5 * st.thickness).min(member.length), 0.0, 0.0));
        let sides: &[(f64, &str)] = if st.sides == semio_s_artifact_norm_en1999_model::StiffenerSides::Two { &[(1.0, ""), (-1.0, MIRROR_MARK)] } else { &[(1.0, "")] };
        let name = loc(&format!("Transverse stiffener {} at x = {:.2} m: {:.0} × {:.0} mm, {}", st.id, st.position, st.width * 1e3, st.thickness * 1e3, if st.sides == semio_s_artifact_norm_en1999_model::StiffenerSides::Two { "two-sided" } else { "one-sided" }), &format!("Quersteife {} bei x = {:.2} m: {:.0} × {:.0} mm, {}", st.id, st.position, st.width * 1e3, st.thickness * 1e3, if st.sides == semio_s_artifact_norm_en1999_model::StiffenerSides::Two { "zweiseitig" } else { "einseitig" }));
        for (sign, mark) in sides {
            let id = format!("{}{mark}", st.id);
            out.push(
                Spec {
                    kind: EntityKind::Member,
                    id: id.clone(),
                    collection: "transverseStiffeners",
                    geometry: Geometry::Member(MemberGeometry {
                        axis: polyline(&[start.clone(), end.clone()]),
                        frame: Frame { origin: start.clone(), ..f.clone() },
                        section: SectionShape::Polygon { polygon: plate((sign * (0.5 * section.web_thickness + 0.5 * st.width), 0.0), (st.width, h_w)) },
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
                    name: name.clone(),
                    layer: LAYER_STIFFENERS,
                    handles: if mark.is_empty() { handles::ids(d, layout, &st.id) } else { Vec::new() },
                    deformation: None,
                    style: None,
                },
                &[start.clone(), end.clone()],
            );
        }
        let top = at(st.position, 0.5 * section.web_thickness + st.width, 0.5 * h_w);
        if let Some(check) = projection_kit::check_with(input.report, &format!("en1999.6.7.6.web-stiffness.{}", st.id)) {
            out.annotate(Annotation::Leader { id: format!("{}:stiffener-label", st.id), anchor: top.clone(), text: loc(&format!("Stiffener {}: {}", st.id, check.explanation.en), &format!("Steife {}: {}", st.id, check.explanation.de)), offset_px: vec![24, -24] }, &top);
        }
        if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", st.id), at: entity_ref(EntityKind::Member, &st.id, "transverseStiffeners"), mode: FailureMode::Other, extent: None }, &top);
        }
    }
}
