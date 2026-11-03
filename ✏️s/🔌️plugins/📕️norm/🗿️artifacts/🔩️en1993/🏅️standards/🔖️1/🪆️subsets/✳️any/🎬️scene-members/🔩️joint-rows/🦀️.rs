//! 🔩️ Joint rows: every explicit bolt row of an end plate joint as its bolts and its T-stub effective-length strip on the end plate, plus the fillet welds of the beam flanges and web, so the component method is visible in 3D.
//!
//! 🔗️ Layout: the plate lies across the member end, its normal is the member axis, `z` is up; the compression centre sits at the middle of the bottom flange and row lever arms are measured from it.

use super::checks::clauses::joint_components as jc;
use super::kit::*;
use super::layout::Layout;
use super::model::{JointRow, SteelJoint};
use super::scene::*;
use super::{handles, SceneInput};

const STRIP_THICKNESS: f64 = 0.002;

fn rect(width: f64, height: f64) -> Polygon2 {
    let (w, h) = (width / 2.0, height / 2.0);
    Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)])
}

fn strip_length(joint: &SteelJoint, row: &JointRow) -> f64 {
    let (m, e, mx, ex) = (row.bolt_to_web * 1e3, row.edge_distance * 1e3, row.extension_to_flange * 1e3, row.extension_edge * 1e3);
    let leff = match row.row_type {
        semio_s_artifact_norm_en1993_model::JointRowType::Extended => jc::l_eff_end_plate_extended(mx, ex, e, joint.gauge * 1e3, joint.plate_width * 1e3),
        semio_s_artifact_norm_en1993_model::JointRowType::Inner => jc::l_eff_end_plate_inner(m, row.alpha_factor),
        _ => jc::l_eff_end_plate_first_below(m, row.alpha_factor),
    };
    (leff.l1() / 1e3).clamp(0.01, joint.plate_width.max(0.05))
}

fn weld(out: &mut Out, id: String, joint: &SteelJoint, binding: &Option<UtilisationBinding>, from: Vec3, to: Vec3, en: &str, de: &str) {
    out.push(
        Spec {
            kind: EntityKind::Fastener,
            id,
            collection: "joints",
            geometry: Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Weld { throat: joint.weld_throat, weld: WeldKind::Fillet }, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: joint.weld_throat, head: HeadShape::None, pattern: None, hole_clearance: None }),
            role: MaterialRole::Weld,
            binding: binding.clone(),
            name: loc(en, de),
            detail: loc(&format!("fillet a = {:.1} mm", joint.weld_throat * 1e3), &format!("Kehlnaht a = {:.1} mm", joint.weld_throat * 1e3)),
            layer: LAYER_JOINTS,
            handles: Vec::new(),
            deformation: None,
        },
        &[from, to],
    );
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for joint in d.joints.iter().filter(|j| j.kind == semio_s_artifact_norm_en1993_model::JointKind::EndPlate && j.weld_throat > 1e-6) {
        if !out.own("joints", &joint.id) {
            continue;
        }
        let rows: Vec<&JointRow> = d.joint_rows.iter().filter(|r| r.joint_id == joint.id).collect();
        let (Some(pose), Some(member)) = (layout.member(&joint.member_id), d.members.iter().find(|m| m.id == joint.member_id)) else { continue };
        if rows.is_empty() {
            continue;
        }
        let Some(section) = d.sections.iter().find(|s| s.id == member.section_id) else { continue };
        let (depth, width) = (section.h, section.b);
        let f = &pose.frame;
        let end = along(&f.origin, &f.x, pose.length);
        let thickness = joint.plate_thickness.max(0.004);
        let plane = Frame { origin: along(&end, &f.x, thickness + STRIP_THICKNESS / 2.0), x: f.y.clone(), y: f.z.clone(), z: f.x.clone() };
        let z0 = -depth / 2.0 + joint.beam_flange_thickness / 2.0;
        let mut components = Vec::new();
        for row in rows {
            let centre = along(&end, &f.z, z0 + row.lever);
            let binding = binding(input.report, &[row.id.as_str()]);
            let bolts_id = format!("{}:bolts", row.id);
            let strip_id = format!("{}:tstub", row.id);
            let length = strip_length(joint, row);
            let m = if row.row_type == semio_s_artifact_norm_en1993_model::JointRowType::Extended { row.extension_to_flange } else { row.bolt_to_web };
            let (from, to) = (along(&centre, &f.x, thickness), along(&centre, &f.x, -0.6 * row.bolt_diameter));
            out.push(
                Spec {
                    kind: EntityKind::Plate,
                    id: strip_id.clone(),
                    collection: "jointRows",
                    geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: along(&plane.origin, &f.z, z0 + row.lever), ..plane.clone() }, outline: rect(length, (2.0 * m).max(0.01)), thickness: STRIP_THICKNESS, offset: 0.0, stiffeners: Vec::new() }),
                    role: MaterialRole::Steel,
                    binding: binding.clone(),
                    name: loc(&format!("T-stub strip of {}", row.id), &format!("T-Stummel-Streifen von {}", row.id)),
                    detail: loc(&format!("effective length {:.0} mm, m = {:.0} mm", length * 1e3, m * 1e3), &format!("wirksame Länge {:.0} mm, m = {:.0} mm", length * 1e3, m * 1e3)),
                    layer: LAYER_JOINTS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[along(&centre, &f.y, length / 2.0), along(&centre, &f.y, -length / 2.0)],
            );
            out.push(
                Spec {
                    kind: EntityKind::Fastener,
                    id: bolts_id.clone(),
                    collection: "jointRows",
                    geometry: Geometry::Fastener(FastenerGeometry {
                        kind: FastenerKind::Bolt { class: row.bolt_class.clone(), preload: None },
                        axis: Segment3 { from: from.clone(), to: to.clone() },
                        diameter: row.bolt_diameter.max(1e-3),
                        head: HeadShape::Hex,
                        pattern: Some(Pattern::Grid { nx: 1, ny: row.bolts.max(1), sx: 0.01, sy: joint.gauge.max(1e-3) }),
                        hole_clearance: Some(if row.bolt_diameter <= 0.024 { 0.002 } else { 0.003 }),
                    }),
                    role: MaterialRole::Bolt,
                    binding: binding.clone(),
                    name: loc(&format!("Bolts of {}", row.id), &format!("Schrauben von {}", row.id)),
                    detail: loc(&format!("{} × M{:.0} class {}, h = {:.0} mm", row.bolts, row.bolt_diameter * 1e3, row.bolt_class, row.lever * 1e3), &format!("{} × M{:.0} Festigkeitsklasse {}, h = {:.0} mm", row.bolts, row.bolt_diameter * 1e3, row.bolt_class, row.lever * 1e3)),
                    layer: LAYER_JOINTS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[from, to],
            );
            out.push(
                Spec {
                    kind: EntityKind::Group,
                    id: row.id.clone(),
                    collection: "jointRows",
                    geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Plate, &strip_id, "jointRows"), entity_ref(EntityKind::Fastener, &bolts_id, "jointRows")] },
                    role: MaterialRole::Steel,
                    binding,
                    name: loc(&format!("Bolt row {}", row.id), &format!("Schraubenreihe {}", row.id)),
                    detail: loc(&format!("{} row of {}", row.row_type, row.joint_id), &format!("Reihe ({}) von {}", row.row_type, row.joint_id)),
                    layer: LAYER_JOINTS,
                    handles: handles::ids(d, layout, &row.id),
                    deformation: None,
                },
                &[centre],
            );
            components.push(entity_ref(EntityKind::Group, &row.id, "jointRows"));
        }
        let joint_binding = binding(input.report, &[joint.id.as_str()]);
        let welds = [
            ("tension", along(&end, &f.z, depth / 2.0), "Weld of the tension flange", "Naht des Zugflanschs"),
            ("compression", along(&end, &f.z, -depth / 2.0), "Weld of the compression flange", "Naht des Druckflanschs"),
        ];
        for (tag, at, en, de) in welds {
            let id = format!("{}:weld-{tag}", joint.id);
            components.push(entity_ref(EntityKind::Fastener, &id, "joints"));
            weld(out, id, joint, &joint_binding, along(&at, &f.y, -width / 2.0), along(&at, &f.y, width / 2.0), en, de);
        }
        let web_id = format!("{}:weld-web", joint.id);
        components.push(entity_ref(EntityKind::Fastener, &web_id, "joints"));
        weld(out, web_id, joint, &joint_binding, along(&end, &f.z, -depth / 2.0 + joint.beam_flange_thickness), along(&end, &f.z, depth / 2.0 - joint.beam_flange_thickness), "Weld of the web", "Naht des Stegs");
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: format!("{}:components", joint.id),
                collection: "joints",
                geometry: Geometry::Group { members: components },
                role: MaterialRole::Steel,
                binding: joint_binding,
                name: loc(&format!("Components of {}", joint.id), &format!("Komponenten von {}", joint.id)),
                detail: loc("Component method: bolt rows with T-stub strips and the flange and web welds.", "Komponentenmethode: Schraubenreihen mit T-Stummel-Streifen sowie Flansch- und Stegnähte."),
                layer: LAYER_JOINTS,
                handles: Vec::new(),
                deformation: None,
            },
            &[end],
        );
    }
}
