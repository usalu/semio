//! 🔗️ Joints: an end plate, the bolt grid or the fillet weld, and the group that selects them as one verification subject.
//!
//! 🔩️ Pattern convention: for a fastener axis along the member axis the renderer lays `nx` along world `Z` and `ny` across, so `nx` carries the bolt rows (pitch) and `ny` the bolts per row (gauge).

use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::{handles, SceneInput};

fn rect(width: f64, height: f64) -> Polygon2 {
    let (w, h) = (width / 2.0, height / 2.0);
    Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)])
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for joint in &d.joints {
        if !out.own("joints", &joint.id) {
            continue;
        }
        let (Some(pose), Some(member)) = (layout.member(&joint.member_id), d.members.iter().find(|m| m.id == joint.member_id)) else { continue };
        let section = d.sections.iter().find(|s| s.id == member.section_id);
        let (depth, width) = section.map_or((0.2, 0.2), |s| (s.h, s.b));
        let end = along(&pose.frame.origin, &pose.frame.x, pose.length);
        let welded = joint.kind == semio_s_artifact_norm_en1993_model::JointKind::Welded;
        let thickness = joint.plate_thickness.max(0.004);
        let (plate_w, plate_h) = if welded {
            (width, depth)
        } else {
            let across = joint.gauge * (joint.bolts_per_row.max(1) - 1) as f64 + 2.0 * joint.edge_distance;
            let down = joint.pitch * (joint.bolt_rows.max(1) - 1) as f64 + 2.0 * joint.end_distance;
            (across.max(0.05), down.max(0.05))
        };
        let binding = binding(input.report, &[joint.id.as_str()]);
        let plate_frame = Frame { origin: end.clone(), x: pose.frame.y.clone(), y: pose.frame.z.clone(), z: pose.frame.x.clone() };
        let plate_id = format!("{}:plate", joint.id);
        let bolts_id = format!("{}:bolts", joint.id);
        let weld_id = format!("{}:weld", joint.id);
        let label = loc(&format!("Joint {}", joint.id), &format!("Anschluss {}", joint.id));
        let mut children = vec![entity_ref(EntityKind::Plate, &plate_id, "joints")];
        let centre = along(&end, &plate_frame.z, thickness / 2.0);
        let rim = [along(&centre, &plate_frame.x, plate_w / 2.0), along(&centre, &plate_frame.x, -plate_w / 2.0), along(&centre, &plate_frame.y, plate_h / 2.0), along(&centre, &plate_frame.y, -plate_h / 2.0)];
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: plate_id,
                collection: "joints",
                geometry: Geometry::Plate(PlateGeometry { frame: plate_frame.clone(), outline: rect(plate_w, plate_h), thickness, offset: 0.0, stiffeners: Vec::new() }),
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: loc(&format!("Plate of {}", joint.id), &format!("Anschlussblech {}", joint.id)),
                detail: loc(&format!("{:.0} × {:.0} × {:.0} mm", plate_w * 1e3, plate_h * 1e3, thickness * 1e3), &format!("{:.0} × {:.0} × {:.0} mm", plate_w * 1e3, plate_h * 1e3, thickness * 1e3)),
                layer: LAYER_JOINTS,
                handles: Vec::new(),
                deformation: None,
            },
            &rim,
        );
        if welded {
            if joint.weld_length > 1e-6 && joint.weld_throat > 1e-6 {
                let from = along(&along(&end, &pose.frame.z, -depth / 2.0), &pose.frame.y, -joint.weld_length / 2.0);
                let to = along(&from, &pose.frame.y, joint.weld_length);
                children.push(entity_ref(EntityKind::Fastener, &weld_id, "joints"));
                out.push(
                    Spec {
                        kind: EntityKind::Fastener,
                        id: weld_id,
                        collection: "joints",
                        geometry: Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Weld { throat: joint.weld_throat, weld: WeldKind::Fillet }, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: joint.weld_throat, head: HeadShape::None, pattern: None, hole_clearance: None }),
                        role: MaterialRole::Weld,
                        binding: binding.clone(),
                        name: loc(&format!("Weld of {}", joint.id), &format!("Schweißnaht {}", joint.id)),
                        detail: loc(&format!("fillet a = {:.1} mm, length {:.0} mm", joint.weld_throat * 1e3, joint.weld_length * 1e3), &format!("Kehlnaht a = {:.1} mm, Länge {:.0} mm", joint.weld_throat * 1e3, joint.weld_length * 1e3)),
                        layer: LAYER_JOINTS,
                        handles: Vec::new(),
                        deformation: None,
                    },
                    &[from, to],
                );
            }
        } else if joint.bolt_diameter > 1e-6 && !d.joint_rows.iter().any(|r| r.joint_id == joint.id) {
            let d_bolt = joint.bolt_diameter;
            let from = along(&end, &pose.frame.x, thickness);
            let to = along(&end, &pose.frame.x, -0.6 * d_bolt);
            let preload = (joint.preload_force > 0.0).then(|| Quantity::new(QuantityKind::Force, joint.preload_force));
            children.push(entity_ref(EntityKind::Fastener, &bolts_id, "joints"));
            out.push(
                Spec {
                    kind: EntityKind::Fastener,
                    id: bolts_id,
                    collection: "joints",
                    geometry: Geometry::Fastener(FastenerGeometry {
                        kind: FastenerKind::Bolt { class: joint.bolt_class.clone(), preload },
                        axis: Segment3 { from: from.clone(), to: to.clone() },
                        diameter: d_bolt,
                        head: HeadShape::Hex,
                        pattern: Some(Pattern::Grid { nx: joint.bolt_rows.max(1), ny: joint.bolts_per_row.max(1), sx: joint.pitch.max(1e-3), sy: joint.gauge.max(1e-3) }),
                        hole_clearance: Some(if d_bolt <= 0.024 { 0.002 } else { 0.003 }),
                    }),
                    role: MaterialRole::Bolt,
                    binding: binding.clone(),
                    name: loc(&format!("Bolts of {}", joint.id), &format!("Schrauben {}", joint.id)),
                    detail: loc(
                        &format!("{} × {} M{:.0} class {}", joint.bolt_rows, joint.bolts_per_row, d_bolt * 1e3, joint.bolt_class),
                        &format!("{} × {} M{:.0} Festigkeitsklasse {}", joint.bolt_rows, joint.bolts_per_row, d_bolt * 1e3, joint.bolt_class),
                    ),
                    layer: LAYER_JOINTS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[from, to],
            );
        }
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: joint.id.clone(),
                collection: "joints",
                geometry: Geometry::Group { members: children },
                role: MaterialRole::Steel,
                binding,
                name: label.clone(),
                detail: loc(&format!("{} joint on {}", joint.kind, joint.member_id), &format!("{}-Anschluss an {}", joint.kind, joint.member_id)),
                layer: LAYER_JOINTS,
                handles: handles::ids(d, layout, &joint.id),
                deformation: None,
            },
            &[end],
        );
    }
}
