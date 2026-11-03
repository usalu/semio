//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ Member length, lateral restraint spacing and section choice for members; pitch, gauge, edge distances and counts for bolt grids; weld length and throat; panel, shell and pile dimensions.

use super::kit::{along, base_id, entity_ref, loc, member_frame, mul, v3};
use super::layout::{Layout, Placement};
use super::model::{self, En1993Snapshot};
use super::scene::*;
use super::scene::projection_kit::handle_kit::{choice_handle, field_binding, grid, handle, HandleSpec as Spec, identity_binding as binding};

/// 🧱️ The unit string a section-choice write carries: the value is the catalogue section height in m and the editor resolves it to the catalogue section.
pub const SECTION_HEIGHT_UNIT: &str = "section-height";

/// 🎨️ Pascals of yield strength per metre of height of a material column and its grip.
pub const STRENGTH_PER_METRE: f64 = 355.0e6;

/// 🧱️ Pascals of lateral pressure per metre of grip travel.
pub const PRESSURE_PER_METRE: f64 = 1.0e5;

fn member_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "members");
    let f = &pose.frame;
    let height = d.sections.iter().find(|s| s.id == member.section_id).map_or(0.2, |s| s.h);
    let heights: Vec<f64> = model::rolled_heb_catalogue().iter().map(|s| s.h).collect();
    let mut out = vec![
        handle(Spec {
            entity: &entity,
            tag: "length",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&f.origin, &f.x, member.length),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("members[id={id}].length"), "m", Some(Quantise::Round { step: 0.01 }))],
            snap: grid(0.05),
            limits: (0.5, 60.0),
            name: loc(&format!("Length of {}", member.label), &format!("Länge von {}", member.label)),
        }),
        handle(Spec {
            entity: &entity,
            tag: "section",
            kind: HandleKind::FaceOffset { normal: f.z.clone() },
            anchor: along(&along(&f.origin, &f.x, member.length / 2.0), &f.z, height / 2.0),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("members[id={id}].sectionId"), SECTION_HEIGHT_UNIT, Some(Quantise::Choice { values: heights.clone() }))],
            snap: SnapRule { catalogue: heights.clone(), tolerance: 0.05, ..SnapRule::NONE },
            limits: (heights.first().copied().unwrap_or(0.1), heights.last().copied().unwrap_or(0.5)),
            name: loc(&format!("Section of {}", member.label), &format!("Querschnitt von {}", member.label)),
        }),
    ];
    if member.ltb_restraint_spacing > 1e-6 && member.ltb_restraint_spacing < member.length {
        out.push(handle(Spec {
            entity: &entity,
            tag: "restraint",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&along(&f.origin, &f.x, member.ltb_restraint_spacing), &f.z, height / 2.0),
            grip: GripStyle::Ring,
            writes: vec![binding(format!("members[id={id}].ltbRestraintSpacing"), "m", Some(Quantise::Round { step: 0.01 }))],
            snap: grid(0.05),
            limits: (0.1, member.length),
            name: loc(&format!("Lateral restraint spacing of {}", member.label), &format!("Abstand der seitlichen Halterungen von {}", member.label)),
        }));
    }
    out
}

fn joint_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(joint), Some(pose)) = (d.joints.iter().find(|j| j.id == id), d.joints.iter().find(|j| j.id == id).and_then(|j| layout.member(&j.member_id))) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "joints");
    let f = &pose.frame;
    let end = along(&f.origin, &f.x, pose.length + joint.plate_thickness);
    let top = joint.pitch * (joint.bolt_rows.max(1) - 1) as f64 / 2.0;
    let half = joint.gauge * (joint.bolts_per_row.max(1) - 1) as f64 / 2.0;
    let int = Some(Quantise::Int);
    if joint.kind == semio_s_artifact_norm_en1993_model::JointKind::Welded {
        return vec![
            handle(Spec { entity: &entity, tag: "weld-length", kind: HandleKind::ScalarAlong { axis: f.y.clone() }, anchor: along(&end, &f.y, joint.weld_length / 2.0), grip: GripStyle::Arrow, writes: vec![binding(format!("joints[id={id}].weldLength"), "m", Some(Quantise::Round { step: 0.005 }))], snap: grid(0.01), limits: (0.02, 2.0), name: loc("Weld length", "Nahtlänge") }),
            handle(Spec { entity: &entity, tag: "weld-throat", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: along(&end, &f.z, -0.1), grip: GripStyle::Diamond, writes: vec![binding(format!("joints[id={id}].weldThroat"), "m", Some(Quantise::Round { step: 0.0005 }))], snap: grid(0.0005), limits: (0.002, 0.03), name: loc("Weld throat thickness", "Nahtdicke a") }),
        ];
    }
    vec![
        handle(Spec { entity: &entity, tag: "pitch", kind: HandleKind::DimensionLine, anchor: along(&along(&end, &f.z, top), &f.y, half + joint.edge_distance), grip: GripStyle::Arrow, writes: vec![binding(format!("joints[id={id}].pitch"), "m", Some(Quantise::Round { step: 0.005 }))], snap: grid(0.005), limits: (0.03, 0.4), name: loc("Bolt pitch p1", "Lochabstand p1") }),
        handle(Spec { entity: &entity, tag: "gauge", kind: HandleKind::DimensionLine, anchor: along(&along(&end, &f.z, top + joint.end_distance), &f.y, half), grip: GripStyle::Arrow, writes: vec![binding(format!("joints[id={id}].gauge"), "m", Some(Quantise::Round { step: 0.005 }))], snap: grid(0.005), limits: (0.03, 0.4), name: loc("Bolt gauge p2", "Lochabstand p2") }),
        handle(Spec { entity: &entity, tag: "end-distance", kind: HandleKind::ScalarAlong { axis: f.z.clone() }, anchor: along(&end, &f.z, top + joint.end_distance), grip: GripStyle::Ring, writes: vec![binding(format!("joints[id={id}].endDistance"), "m", Some(Quantise::Round { step: 0.005 }))], snap: grid(0.005), limits: (0.01, 0.2), name: loc("End distance e1", "Randabstand e1") }),
        handle(Spec { entity: &entity, tag: "edge-distance", kind: HandleKind::ScalarAlong { axis: f.y.clone() }, anchor: along(&end, &f.y, half + joint.edge_distance), grip: GripStyle::Ring, writes: vec![binding(format!("joints[id={id}].edgeDistance"), "m", Some(Quantise::Round { step: 0.005 }))], snap: grid(0.005), limits: (0.01, 0.2), name: loc("Edge distance e2", "Randabstand e2") }),
        handle(Spec { entity: &entity, tag: "rows", kind: HandleKind::CountStepper, anchor: along(&end, &f.z, -top - joint.end_distance), grip: GripStyle::Diamond, writes: vec![binding(format!("joints[id={id}].boltRows"), "1", int.clone())], snap: SnapRule::NONE, limits: (1.0, 8.0), name: loc("Number of bolt rows", "Anzahl Schraubenreihen") }),
        handle(Spec { entity: &entity, tag: "per-row", kind: HandleKind::CountStepper, anchor: along(&end, &f.y, -half - joint.edge_distance), grip: GripStyle::Diamond, writes: vec![binding(format!("joints[id={id}].boltsPerRow"), "1", int)], snap: SnapRule::NONE, limits: (1.0, 6.0), name: loc("Bolts per row", "Schrauben je Reihe") }),
    ]
}

fn panel_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(panel) = d.plated_panels.iter().find(|p| p.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Plate, id, "platedPanels");
    let o = layout.origin(id);
    vec![
        handle(Spec { entity: &entity, tag: "a", kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: v3(o.x + panel.a, o.y + panel.b / 2.0, o.z), grip: GripStyle::Arrow, writes: vec![binding(format!("platedPanels[id={id}].a"), "m", Some(Quantise::Round { step: 0.01 }))], snap: grid(0.05), limits: (0.1, 20.0), name: loc("Panel length a", "Feldlänge a") }),
        handle(Spec { entity: &entity, tag: "b", kind: HandleKind::ScalarAlong { axis: Vec3::Y }, anchor: v3(o.x + panel.a / 2.0, o.y + panel.b, o.z), grip: GripStyle::Arrow, writes: vec![binding(format!("platedPanels[id={id}].b"), "m", Some(Quantise::Round { step: 0.01 }))], snap: grid(0.05), limits: (0.1, 20.0), name: loc("Panel width b", "Feldbreite b") }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: Vec3::Z }, anchor: v3(o.x + panel.a / 2.0, o.y + panel.b / 2.0, o.z + panel.thickness), grip: GripStyle::Diamond, writes: vec![binding(format!("platedPanels[id={id}].thickness"), "m", Some(Quantise::Round { step: 0.001 }))], snap: grid(0.001), limits: (0.002, 0.1), name: loc("Panel thickness", "Blechdicke") }),
    ]
}

fn silo_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(silo) = d.silo_shells.iter().find(|s| s.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Shell, id, "siloShells");
    let o = layout.origin(id);
    vec![
        handle(Spec { entity: &entity, tag: "radius", kind: HandleKind::Radial { centre: o.clone() }, anchor: v3(o.x + silo.radius, o.y, o.z + silo.depth / 2.0), grip: GripStyle::Arrow, writes: vec![binding(format!("siloShells[id={id}].radius"), "m", Some(Quantise::Round { step: 0.01 }))], snap: grid(0.05), limits: (0.2, 30.0), name: loc("Silo radius", "Silo-Radius") }),
        handle(Spec { entity: &entity, tag: "depth", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x, o.y, o.z + silo.depth), grip: GripStyle::Arrow, writes: vec![binding(format!("siloShells[id={id}].depth"), "m", Some(Quantise::Round { step: 0.05 }))], snap: grid(0.1), limits: (0.5, 80.0), name: loc("Fill depth", "Füllhöhe") }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: Vec3::X }, anchor: v3(o.x + silo.radius, o.y, o.z + silo.depth / 2.0), grip: GripStyle::Diamond, writes: vec![binding(format!("siloShells[id={id}].thickness"), "m", Some(Quantise::Round { step: 0.0005 }))], snap: grid(0.0005), limits: (0.002, 0.1), name: loc("Wall thickness", "Wanddicke") }),
    ]
}

fn pile_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(pile) = d.piles.iter().find(|p| p.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "piles");
    let o = layout.origin(id);
    vec![handle(Spec { entity: &entity, tag: "embedded", kind: HandleKind::ScalarAlong { axis: mulz(-1.0) }, anchor: along(&o, &Vec3::Z, -pile.embedded_length), grip: GripStyle::Arrow, writes: vec![binding(format!("piles[id={id}].embeddedLength"), "m", Some(Quantise::Round { step: 0.1 }))], snap: grid(0.5), limits: (0.5, 80.0), name: loc("Embedded length", "Einbindelänge") })]
}

fn mulz(k: f64) -> Vec3 {
    v3(0.0, 0.0, k)
}

fn leaf(entity: &EntityRef, tag: &'static str, kind: HandleKind, anchor: Vec3, grip: GripStyle, path: String, limits: (f64, f64), en: &str, de: &str) -> Handle {
    handle(Spec { entity, tag, kind, anchor, grip, writes: vec![binding(path, "m", Some(Quantise::Round { step: 0.001 }))], snap: grid(0.005), limits, name: loc(en, de) })
}

fn storey_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(storey), Some(pose)) = (d.storeys.iter().find(|s| s.id == id), d.storeys.iter().find(|s| s.id == id).and_then(|s| layout.member(&s.member_id))) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "storeys");
    let top = along(&pose.frame.origin, &pose.frame.x, pose.length);
    vec![
        leaf(&entity, "storey-height", HandleKind::ScalarAlong { axis: pose.frame.x.clone() }, along(&top, &pose.frame.x, 0.0), GripStyle::Arrow, format!("storeys[id={id}].height"), (0.5, 60.0), "Storey height h", "Geschosshöhe h"),
        leaf(&entity, "drift", HandleKind::ScalarAlong { axis: Vec3::Y }, along(&top, &Vec3::Y, storey.drift * 50.0), GripStyle::Ring, format!("storeys[id={id}].drift"), (0.0001, 1.0), "Horizontal drift δ_H,Ed", "Horizontale Kopfverschiebung δ_H,Ed"),
        leaf(&entity, "structure-height", HandleKind::ScalarAlong { axis: pose.frame.x.clone() }, along(&top, &pose.frame.x, storey.building_height - storey.height), GripStyle::Diamond, format!("storeys[id={id}].buildingHeight"), (0.5, 200.0), "Structure height h", "Bauwerkshöhe h"),
    ]
}

fn member_length_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "members");
    let f = &pose.frame;
    let height = d.sections.iter().find(|s| s.id == member.section_id).map_or(0.2, |s| s.h);
    [("ltbLength", "ltb-length", member.ltb_length, 0.45, "Lateral-torsional buckling length", "Kipplänge"), ("bucklingLengthY", "buckling-y", member.buckling_length_y, 0.55, "Buckling length about y", "Knicklänge um y"), ("bucklingLengthZ", "buckling-z", member.buckling_length_z, 0.65, "Buckling length about z", "Knicklänge um z")]
        .into_iter()
        .filter(|(_, _, value, _, _, _)| *value > 1e-6)
        .map(|(field, tag, value, lift, en, de)| leaf(&entity, tag, HandleKind::ScalarAlong { axis: f.x.clone() }, along(&along(&f.origin, &f.x, value), &f.z, height / 2.0 + lift), GripStyle::Ring, format!("members[id={id}].{field}"), (0.1, 200.0), en, de))
        .collect()
}

fn warping_support_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    if !d.member_actions.iter().any(|a| a.member_id == id && a.action.t != 0.0) {
        return Vec::new();
    }
    let entity = entity_ref(EntityKind::Member, id, "members");
    let height = d.sections.iter().find(|s| s.id == member.section_id).map_or(0.2, |s| s.h);
    let anchor = along(&along(&pose.frame.origin, &pose.frame.x, member.length), &pose.frame.z, -height);
    vec![choice_handle(&entity, "warping-support", anchor, format!("members[id={id}].warpingSupport"), &warping_support_options(), loc("Torsion and warping end conditions", "Torsions- und Wölblagerung"))]
}

fn joint_plate_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(joint) = d.joints.iter().find(|j| j.id == id) else { return Vec::new() };
    let Some(pose) = layout.member(&joint.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "joints");
    let f = &pose.frame;
    let end = along(&f.origin, &f.x, pose.length);
    vec![
        leaf(&entity, "plate-thickness", HandleKind::FaceOffset { normal: f.x.clone() }, along(&end, &f.x, joint.plate_thickness), GripStyle::Diamond, format!("joints[id={id}].plateThickness"), (0.002, 0.1), "Plate thickness", "Blechdicke"),
        leaf(&entity, "bolt-diameter", HandleKind::FaceOffset { normal: f.y.clone() }, along(&along(&end, &f.x, joint.plate_thickness), &f.y, joint.bolt_diameter), GripStyle::Ring, format!("joints[id={id}].boltDiameter"), (0.006, 0.048), "Bolt diameter", "Schraubendurchmesser"),
    ]
}

fn cold_formed_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(c) = d.cold_formed_members.iter().find(|c| c.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "coldFormedMembers");
    let o = layout.origin(id);
    vec![
        leaf(&entity, "flange-width", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(o.x, o.y + c.flange_width, o.z + c.web_height), GripStyle::Arrow, format!("coldFormedMembers[id={id}].flangeWidth"), (0.01, 0.5), "Flange width b_p", "Gurtbreite b_p"),
        leaf(&entity, "web-height", HandleKind::ScalarAlong { axis: Vec3::Z }, v3(o.x, o.y, o.z + c.web_height), GripStyle::Arrow, format!("coldFormedMembers[id={id}].webHeight"), (0.02, 1.0), "Web height h_w", "Steghöhe h_w"),
        leaf(&entity, "lip-length", HandleKind::ScalarAlong { axis: Vec3::Z }, v3(o.x, o.y + c.flange_width, o.z + c.web_height - c.lip_length), GripStyle::Ring, format!("coldFormedMembers[id={id}].lipLength"), (0.0, 0.1), "Edge fold c_p", "Randlippe c_p"),
        leaf(&entity, "thickness", HandleKind::FaceOffset { normal: Vec3::X }, v3(o.x + c.thickness, o.y, o.z + c.web_height / 2.0), GripStyle::Diamond, format!("coldFormedMembers[id={id}].thickness"), (0.0005, 0.01), "Thickness t", "Dicke t"),
    ]
}

fn restraint_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(restraint) = d.restraints.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some(pose) = layout.member(&restraint.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "restraints");
    let f = &pose.frame;
    let height = d.members.iter().find(|m| m.id == restraint.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)).map_or(0.2, |s| s.h);
    let mut out = vec![leaf(&entity, "intermediate-support", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&along(&f.origin, &f.x, restraint.intermediate_position.max(pose.length / 2.0)), &f.z, -height / 2.0), GripStyle::Ring, format!("restraints[id={id}].intermediatePosition"), (0.0, pose.length), "Intermediate support position", "Position der Zwischenlagerung")];
    if restraint.lateral_spring_stiffness > 0.0 {
        out.push(leaf(&entity, "lateral-spring", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&along(&f.origin, &f.x, restraint.lateral_spring_position), &f.y, height / 2.0), GripStyle::Diamond, format!("restraints[id={id}].lateralSpringPosition"), (0.0, pose.length), "Lateral spring position", "Position der seitlichen Feder"));
    }
    out.push(leaf(&entity, "load-height", HandleKind::ScalarAlong { axis: f.z.clone() }, along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, restraint.load_height), GripStyle::Arrow, format!("restraints[id={id}].loadHeight"), (-1.0, 1.0), "Load height z_g", "Lastangriffshöhe z_g"));
    if restraint.hinge_to_restraint > 0.0 {
        out.push(leaf(&entity, "hinge-distance", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&f.origin, &f.x, restraint.hinge_to_restraint), GripStyle::Arrow, format!("restraints[id={id}].hingeToRestraint"), (0.0, pose.length * 10.0), "Plastic hinge to next restraint", "Fließgelenk bis zur nächsten Halterung"));
    }
    out
}

fn built_up_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.built_up_members.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some(pose) = layout.member(&row.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "builtUpMembers");
    let f = &pose.frame;
    let mid = along(&f.origin, &f.x, pose.length / 2.0);
    vec![
        leaf(&entity, "chord-distance", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&mid, &f.y, row.chord_distance / 2.0), GripStyle::Arrow, format!("builtUpMembers[id={id}].chordDistance"), (0.05, 3.0), "Chord distance h_0", "Gurtabstand h_0"),
        leaf(&entity, "panel-spacing", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&f.origin, &f.x, row.panel_spacing), GripStyle::Ring, format!("builtUpMembers[id={id}].panelSpacing"), (0.05, pose.length), "Connection spacing a", "Abstand der Bindebleche a"),
        leaf(&entity, "chain-spacing", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&f.origin, &f.x, row.chain_spacing), GripStyle::Diamond, format!("builtUpMembers[id={id}].chainSpacing"), (0.0, pose.length), "Connection spacing of closely spaced members", "Verbindungsabstand eng gestellter Stäbe"),
    ]
}

fn weld_group_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(weld) = d.weld_groups.iter().find(|w| w.id == id) else { return Vec::new() };
    let Some(pose) = layout.member(&weld.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "weldGroups");
    let f = &pose.frame;
    let end = along(&f.origin, &f.x, pose.length);
    vec![
        leaf(&entity, "weld-throat", HandleKind::FaceOffset { normal: f.x.clone() }, along(&end, &f.x, weld.throat), GripStyle::Diamond, format!("weldGroups[id={id}].throat"), (0.002, 0.03), "Throat thickness a", "Nahtdicke a"),
        leaf(&entity, "weld-width", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&end, &f.y, weld.width / 2.0), GripStyle::Arrow, format!("weldGroups[id={id}].width"), (0.0, 2.0), "Group width b", "Gruppenbreite b"),
        leaf(&entity, "weld-depth", HandleKind::ScalarAlong { axis: f.z.clone() }, along(&end, &f.z, weld.depth / 2.0), GripStyle::Arrow, format!("weldGroups[id={id}].depth"), (0.02, 2.0), "Group depth d", "Gruppentiefe d"),
        leaf(&entity, "weld-eccentricity", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&end, &f.y, weld.eccentricity), GripStyle::Ring, format!("weldGroups[id={id}].eccentricity"), (0.0, 3.0), "Eccentricity e", "Ausmitte e"),
    ]
}

fn hollow_joint_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(joint) = d.hollow_joints.iter().find(|j| j.id == id) else { return Vec::new() };
    let Some(pose) = layout.member(&joint.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "hollowJoints");
    let f = &pose.frame;
    let mid = along(&f.origin, &f.x, pose.length / 2.0);
    vec![
        leaf(&entity, "gap", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&mid, &f.x, joint.gap / 2.0), GripStyle::Arrow, format!("hollowJoints[id={id}].gap"), (0.0, 1.0), "Gap g", "Spalt g"),
        leaf(&entity, "chord-wall", HandleKind::FaceOffset { normal: f.z.clone() }, along(&mid, &f.z, joint.chord_height / 2.0), GripStyle::Diamond, format!("hollowJoints[id={id}].chordThickness"), (0.002, 0.04), "Chord wall thickness t_0", "Gurtwanddicke t_0"),
        leaf(&entity, "brace1-width", HandleKind::FaceOffset { normal: f.y.clone() }, along(&mid, &f.y, joint.brace1_width / 2.0), GripStyle::Ring, format!("hollowJoints[id={id}].brace1Width"), (0.02, 0.5), "Brace 1 width b_1", "Streben-1-Breite b_1"),
        leaf(&entity, "brace2-width", HandleKind::FaceOffset { normal: f.y.clone() }, along(&mid, &f.y, -joint.brace2_width / 2.0), GripStyle::Ring, format!("hollowJoints[id={id}].brace2Width"), (0.02, 0.5), "Brace 2 width b_2", "Streben-2-Breite b_2"),
    ]
}

fn parametric_section_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(p) = d.parametric_sections.iter().find(|p| p.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "parametricSections");
    let origin = layout.origin(id);
    let f = member_frame(&origin, &Vec3::X);
    let mid = along(&origin, &f.x, 0.5);
    let field = |name: &str| format!("parametricSections[id={id}].{name}");
    let (height, width, wall) = match p.shape {
        semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI => (("Height h", "Höhe h"), Some(("Width b", "Breite b")), ("Web thickness t_w", "Stegdicke t_w")),
        semio_s_artifact_norm_en1993_model::ParametricShape::Rhs => (("Height h", "Höhe h"), Some(("Width b", "Breite b")), ("Wall thickness t", "Wanddicke t")),
        semio_s_artifact_norm_en1993_model::ParametricShape::Chs => (("Diameter D", "Durchmesser D"), None, ("Wall thickness t", "Wanddicke t")),
        semio_s_artifact_norm_en1993_model::ParametricShape::Angle => (("Long leg h", "Langer Schenkel h"), Some(("Short leg b", "Kurzer Schenkel b")), ("Leg thickness t", "Schenkeldicke t")),
    };
    let mut out = vec![
        leaf(&entity, "section-height", HandleKind::FaceOffset { normal: f.z.clone() }, along(&mid, &f.z, p.height / 2.0), GripStyle::Diamond, field("height"), (0.02, 3.0), height.0, height.1),
        leaf(&entity, "web-thickness", HandleKind::FaceOffset { normal: f.y.clone() }, along(&mid, &f.y, p.web_thickness / 2.0), GripStyle::Ring, field("webThickness"), (0.002, 0.1), wall.0, wall.1),
    ];
    if let Some(width) = width {
        out.push(leaf(&entity, "section-width", HandleKind::FaceOffset { normal: f.y.clone() }, along(&mid, &f.y, p.width / 2.0), GripStyle::Diamond, field("width"), (0.02, 3.0), width.0, width.1));
    }
    if p.shape == semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI {
        out.push(leaf(&entity, "flange-thickness", HandleKind::FaceOffset { normal: f.z.clone() }, along(&along(&mid, &f.z, p.height / 2.0 - p.flange_thickness), &f.y, p.width / 2.0), GripStyle::Ring, format!("parametricSections[id={id}].flangeThickness"), (0.002, 0.1), "Flange thickness t_f", "Flanschdicke t_f"));
    }
    out
}

fn joint_row_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.joint_rows.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some(joint) = d.joints.iter().find(|j| j.id == row.joint_id) else { return Vec::new() };
    let Some(pose) = layout.member(&joint.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "jointRows");
    let f = &pose.frame;
    let depth = d.members.iter().find(|m| m.id == joint.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)).map_or(0.3, |s| s.h);
    let centre = along(&along(&f.origin, &f.x, pose.length + joint.plate_thickness), &f.z, row.lever - depth / 2.0 + joint.beam_flange_thickness / 2.0);
    let half = joint.gauge * (row.bolts.max(1) - 1) as f64 / 2.0;
    vec![
        leaf(&entity, "row-lever", HandleKind::ScalarAlong { axis: f.z.clone() }, centre.clone(), GripStyle::Arrow, format!("jointRows[id={id}].lever"), (0.02, 2.0), "Lever arm h_r", "Hebelarm h_r"),
        leaf(&entity, "row-m", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&centre, &f.y, half + row.bolt_to_web), GripStyle::Ring, format!("jointRows[id={id}].boltToWeb"), (0.01, 0.2), "Bolt to web distance m", "Schraubenabstand zum Steg m"),
        leaf(&entity, "row-edge", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&centre, &f.y, -half - row.edge_distance), GripStyle::Ring, format!("jointRows[id={id}].edgeDistance"), (0.01, 0.2), "Edge distance e", "Randabstand e"),
        handle(Spec { entity: &entity, tag: "row-bolts", kind: HandleKind::CountStepper, anchor: along(&centre, &f.z, 0.03), grip: GripStyle::Diamond, writes: vec![binding(format!("jointRows[id={id}].bolts"), "1", Some(Quantise::Int))], snap: SnapRule::NONE, limits: (1.0, 6.0), name: loc("Bolts in the row", "Schrauben in der Reihe") }),
    ]
}

fn plate_row_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(p) = d.joint_plates.iter().find(|p| p.id == id) else { return Vec::new() };
    let Some(pose) = super::plate_layout::plate_pose(d, layout, p) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "jointPlates");
    let (frame, count) = (&pose.frames[0], Some(Quantise::Int));
    let top = along(&frame.origin, &pose.line, p.plate_height / 2.0);
    vec![
        leaf(&entity, "plate-thickness", HandleKind::FaceOffset { normal: pose.normal.clone() }, along(&frame.origin, &pose.normal, p.plate_thickness / 2.0), GripStyle::Ring, format!("jointPlates[id={id}].plateThickness"), (0.004, 0.06), "Plate thickness t_p", "Blechdicke t_p"),
        leaf(&entity, "plate-height", HandleKind::FaceOffset { normal: pose.line.clone() }, top.clone(), GripStyle::Diamond, format!("jointPlates[id={id}].plateHeight"), (0.05, 1.0), "Plate height h_p", "Blechhöhe h_p"),
        leaf(&entity, "plate-eccentricity", HandleKind::ScalarAlong { axis: pose.across.clone() }, pose.bolt_centre.clone(), GripStyle::Arrow, format!("jointPlates[id={id}].eccentricity"), (0.0, 0.5), "Eccentricity z of the bolt line", "Exzentrizität z der Schraubenlinie"),
        leaf(&entity, "plate-pitch", HandleKind::ScalarAlong { axis: pose.line.clone() }, along(&pose.bolt_centre, &pose.line, p.pitch * (p.bolt_rows.max(1) - 1) as f64 / 2.0), GripStyle::Ring, format!("jointPlates[id={id}].pitch"), (0.03, 0.2), "Pitch p_1", "Lochabstand p_1"),
        handle(Spec { entity: &entity, tag: "plate-rows", kind: HandleKind::CountStepper, anchor: along(&top, &pose.line, 0.03), grip: GripStyle::Diamond, writes: vec![binding(format!("jointPlates[id={id}].boltRows"), "1", count.clone())], snap: SnapRule::NONE, limits: (1.0, 8.0), name: loc("Bolt rows n_1", "Schraubenreihen n_1") }),
        handle(Spec { entity: &entity, tag: "plate-per-row", kind: HandleKind::CountStepper, anchor: along(&pose.bolt_centre, &pose.across, 0.04 + p.gauge), grip: GripStyle::Diamond, writes: vec![binding(format!("jointPlates[id={id}].boltsPerRow"), "1", count)], snap: SnapRule::NONE, limits: (1.0, 4.0), name: loc("Bolts per row n_2", "Schrauben je Reihe n_2") }),
    ]
}

fn foundation_block_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(b) = d.foundation_blocks.iter().find(|b| b.id == id) else { return Vec::new() };
    let Some(pose) = super::plate_layout::block_pose(d, layout, b) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "foundationBlocks");
    let bottom = pose.block.origin.clone();
    let count = Some(Quantise::Int);
    vec![
        leaf(&entity, "block-depth", HandleKind::FaceOffset { normal: along(&Vec3::ZERO, &pose.up, -1.0) }, bottom.clone(), GripStyle::Diamond, format!("foundationBlocks[id={id}].blockDepth"), (0.2, 3.0), "Block depth h", "Blockhöhe h"),
        leaf(&entity, "block-length", HandleKind::FaceOffset { normal: pose.length_axis.clone() }, along(&along(&bottom, &pose.up, b.block_depth / 2.0), &pose.length_axis, b.block_length / 2.0), GripStyle::Diamond, format!("foundationBlocks[id={id}].blockLength"), (0.2, 4.0), "Block length a_2", "Blocklänge a_2"),
        leaf(&entity, "block-width", HandleKind::FaceOffset { normal: pose.width_axis.clone() }, along(&along(&bottom, &pose.up, b.block_depth / 2.0), &pose.width_axis, b.block_width / 2.0), GripStyle::Diamond, format!("foundationBlocks[id={id}].blockWidth"), (0.2, 4.0), "Block width b_2", "Blockbreite b_2"),
        leaf(&entity, "grout-thickness", HandleKind::FaceOffset { normal: pose.up.clone() }, pose.grout.origin.clone(), GripStyle::Ring, format!("foundationBlocks[id={id}].groutThickness"), (0.005, 0.15), "Grout thickness t_g", "Dicke des Vergussmörtels t_g"),
        leaf(&entity, "base-plate-thickness", HandleKind::FaceOffset { normal: pose.up.clone() }, pose.plate_top.clone(), GripStyle::Diamond, format!("joints[id={}].plateThickness", b.joint_id), (0.005, 0.1), "Base plate thickness t_p", "Fußplattendicke t_p"),
        leaf(&entity, "anchor-embedment", HandleKind::ScalarAlong { axis: pose.up.clone() }, pose.anchors.first().map_or(bottom, |a| a.1.clone()), GripStyle::Arrow, format!("foundationBlocks[id={id}].anchorEmbedment"), (0.05, 2.0), "Embedment depth h_ef", "Verankerungstiefe h_ef"),
        handle(Spec { entity: &entity, tag: "anchor-count", kind: HandleKind::CountStepper, anchor: pose.anchors.first().map_or(pose.plate_top.clone(), |a| a.0.clone()), grip: GripStyle::Diamond, writes: vec![binding(format!("foundationBlocks[id={id}].anchorCount"), "1", count)], snap: SnapRule::NONE, limits: (1.0, 8.0), name: loc("Anchors in tension", "Zugbeanspruchte Anker") }),
    ]
}

fn frame_node_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(n) = d.frame_nodes.iter().find(|n| n.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "frameNodes");
    let at = layout.frame_point(n.x, n.z);
    vec![
        leaf(&entity, "node-x", HandleKind::ScalarAlong { axis: Vec3::X }, at.clone(), GripStyle::Arrow, format!("frameNodes[id={id}].x"), (-200.0, 200.0), "Coordinate x", "Koordinate x"),
        leaf(&entity, "node-z", HandleKind::ScalarAlong { axis: Vec3::Z }, along(&at, &Vec3::Z, 0.0), GripStyle::Arrow, format!("frameNodes[id={id}].z"), (-200.0, 200.0), "Coordinate z", "Koordinate z"),
    ]
}

fn annex_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let mut out = Vec::new();
    if let Some(silo) = d.silo_shells.iter().find(|s| s.id == id && s.ring_spacing > 0.0) {
        let entity = entity_ref(EntityKind::Shell, id, "siloShells");
        let o = layout.origin(id);
        out.push(leaf(&entity, "ring-spacing", HandleKind::ScalarAlong { axis: Vec3::Z }, v3(o.x + silo.radius, o.y, o.z + silo.ring_spacing), GripStyle::Ring, format!("siloShells[id={id}].ringSpacing"), (0.1, silo.depth.max(0.2)), "Ring stiffener spacing l_r", "Ringsteifenabstand l_r"));
    }
    if let Some(deck) = d.bridge_fatigue.iter().find(|b| b.id == id && b.deck_thickness > 0.0) {
        let entity = entity_ref(EntityKind::Group, id, "bridgeFatigue");
        let o = layout.origin(id);
        out.push(leaf(&entity, "deck-thickness", HandleKind::FaceOffset { normal: Vec3::Z }, v3(o.x + deck.crossbeam_spacing / 2.0, o.y + deck.trough_spacing * 2.5, o.z + deck.deck_thickness), GripStyle::Diamond, format!("bridgeFatigue[id={id}].deckThickness"), (0.006, 0.04), "Deck plate thickness t_d", "Fahrbahnblechdicke t_d"));
        out.push(leaf(&entity, "trough-spacing", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(o.x, o.y + deck.trough_spacing, o.z), GripStyle::Arrow, format!("bridgeFatigue[id={id}].troughSpacing"), (0.2, 1.2), "Trough spacing e", "Trogabstand e"));
        out.push(leaf(&entity, "crossbeam-spacing", HandleKind::ScalarAlong { axis: Vec3::X }, v3(o.x + deck.crossbeam_spacing, o.y, o.z), GripStyle::Arrow, format!("bridgeFatigue[id={id}].crossbeamSpacing"), (0.5, 8.0), "Cross beam spacing a", "Querträgerabstand a"));
    }
    if let Some(p) = d.plated_panels.iter().find(|p| p.id == id && p.corrugation_flat > 0.0) {
        let entity = entity_ref(EntityKind::Plate, id, "platedPanels");
        let o = layout.origin(id);
        let at = v3(o.x, o.y + p.b + 1.0, o.z);
        out.push(leaf(&entity, "corrugation-depth", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(at.x, at.y + p.corrugation_depth, at.z + p.web_height), GripStyle::Arrow, format!("platedPanels[id={id}].corrugationDepth"), (0.01, 0.3), "Corrugation depth d", "Wellentiefe d"));
        out.push(leaf(&entity, "web-height", HandleKind::ScalarAlong { axis: Vec3::Z }, v3(at.x, at.y, at.z + p.web_height), GripStyle::Arrow, format!("platedPanels[id={id}].webHeight"), (0.1, 5.0), "Corrugated web height h_w", "Steghöhe h_w des Trapezstegs"));
    }
    if let Some(p) = d.plated_panels.iter().find(|p| p.id == id && p.stiffener_spacing > 0.0) {
        let entity = entity_ref(EntityKind::Plate, id, "platedPanels");
        let o = layout.origin(id);
        out.push(leaf(&entity, "stiffener-spacing", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(o.x + p.a / 2.0, o.y + p.stiffener_spacing, o.z + p.thickness), GripStyle::Ring, format!("platedPanels[id={id}].stiffenerSpacing"), (0.05, p.b.max(0.1)), "Longitudinal stiffener spacing s", "Längssteifenabstand s"));
    }
    if let Some(p) = d.plated_panels.iter().find(|p| p.id == id && p.stiffener_shape != semio_s_artifact_norm_en1993_model::StiffenerShape::None && p.stiffener_depth > 0.0) {
        out.extend(stiffener_outline_handles(p, layout.origin(id)));
    }
    if let Some(p) = d.plated_panels.iter().find(|p| p.id == id && p.lateral_pressure > 0.0 && p.a > 0.0 && p.b > 0.0) {
        let entity = entity_ref(EntityKind::Plate, id, "platedPanels");
        let o = layout.origin(id);
        let writes = vec![field_binding(format!("platedPanels[id={id}].lateralPressure"), Affine { base: 0.0, k: PRESSURE_PER_METRE }, "Pa", Some(Quantise::Round { step: 100.0 }))];
        out.push(handle(Spec { entity: &entity, tag: "pressure", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x + p.a / 2.0, o.y + p.b / 2.0, o.z + p.thickness / 2.0 + p.lateral_pressure / PRESSURE_PER_METRE), grip: GripStyle::Arrow, writes, snap: grid(0.001), limits: (0.0, 10.0), name: loc("Lateral pressure p", "Querdruck p") }));
    }
    out
}

fn half_leaf(entity: &EntityRef, tag: &'static str, kind: HandleKind, anchor: Vec3, grip: GripStyle, path: String, factor: f64, limits: (f64, f64), en: &str, de: &str) -> Handle {
    let writes = vec![field_binding(path, Affine { base: 0.0, k: factor }, "m", Some(Quantise::Round { step: 0.001 }))];
    handle(Spec { entity, tag, kind, anchor, grip, writes, snap: grid(0.001), limits: (limits.0 / factor, limits.1 / factor), name: loc(en, de) })
}

fn stiffener_shape_options() -> Vec<(String, &'static str, &'static str)> {
    model::StiffenerShape::ALL.iter().map(|v| (format!("{v:?}"), match v {
        model::StiffenerShape::None => ("none", "keine"),
        model::StiffenerShape::Flat => ("flat bar", "Flachstahl"),
        model::StiffenerShape::Tee => ("tee", "T-Profil"),
        model::StiffenerShape::Angle => ("angle", "Winkel"),
    })).map(|(t, (en, de))| (t, en, de)).collect()
}

fn warping_support_options() -> Vec<(String, &'static str, &'static str)> {
    model::WarpingSupport::ALL.iter().map(|v| (format!("{v:?}"), match v {
        model::WarpingSupport::Fork => ("fork at both ends", "Gabellagerung beidseitig"),
        model::WarpingSupport::Fixed => ("restrained at both ends", "beidseitig eingespannt"),
        model::WarpingSupport::FixedFork => ("restrained and fork", "eingespannt und Gabel"),
        model::WarpingSupport::Cantilever => ("cantilever", "Kragarm"),
    })).map(|(t, (en, de))| (t, en, de)).collect()
}

/// 🧱️ The five grips of the transverse stiffener outline at the far panel edge: shape choice, height, thickness and, for tee and angle, flange width and thickness; a half-extent grip writes twice its distance from the centreline.
fn stiffener_outline_handles(p: &model::PlatedPanel, o: Vec3) -> Vec<Handle> {
    let id = &p.id;
    let entity = entity_ref(EntityKind::Plate, id, "platedPanels");
    let (x, y, base) = (o.x + p.a, o.y + p.b / 2.0, o.z + p.thickness / 2.0);
    let top = base + p.stiffener_depth;
    let path = |field: &str| format!("platedPanels[id={id}].{field}");
    let mut out = vec![
        leaf(&entity, "stiffener-depth", HandleKind::ScalarAlong { axis: Vec3::Z }, v3(x, y, top), GripStyle::Arrow, path("stiffenerDepth"), (0.02, 0.6), "Transverse stiffener height h_s", "Höhe h_s der Quersteife"),
        half_leaf(&entity, "stiffener-thickness", HandleKind::FaceOffset { normal: Vec3::X }, v3(x + p.stiffener_thickness / 2.0, y, base + p.stiffener_depth / 2.0), GripStyle::Ring, path("stiffenerThickness"), 2.0, (0.001, 0.025), "Transverse stiffener thickness t_s", "Dicke t_s der Quersteife"),
        choice_handle(&entity, "stiffener-shape", v3(x, y, top + 0.05), path("stiffenerShape"), &stiffener_shape_options(), loc("Transverse stiffener outline shape", "Konturform der Quersteife")),
    ];
    if p.stiffener_flange_width > 0.0 && p.stiffener_flange_thickness > 0.0 {
        let reach = if p.stiffener_shape == semio_s_artifact_norm_en1993_model::StiffenerShape::Angle { p.stiffener_flange_width } else { p.stiffener_flange_width / 2.0 };
        let factor = if p.stiffener_shape == semio_s_artifact_norm_en1993_model::StiffenerShape::Angle { 1.0 } else { 2.0 };
        out.push(half_leaf(&entity, "stiffener-flange-width", HandleKind::ScalarAlong { axis: Vec3::X }, v3(x + reach, y, top), GripStyle::Arrow, path("stiffenerFlangeWidth"), factor, (0.02, 0.4), "Stiffener flange width b_f", "Flanschbreite b_f der Steife"));
        out.push(half_leaf(&entity, "stiffener-flange-thickness", HandleKind::FaceOffset { normal: Vec3::Z }, v3(x, y, top + p.stiffener_flange_thickness / 2.0), GripStyle::Ring, path("stiffenerFlangeThickness"), 2.0, (0.001, 0.03), "Stiffener flange thickness t_f", "Flanschdicke t_f der Steife"));
    }
    out
}

/// 📏️ Newtons of action per metre of arrow travel on a load glyph grip.
pub const FORCE_PER_METRE: f64 = 1.0e5;
/// 📏️ Newton metres of action per metre of arrow travel on a moment glyph grip.
pub const MOMENT_PER_METRE: f64 = 1.0e5;

fn dimension(entity: &EntityRef, tag: &str, kind: HandleKind, anchor: Vec3, grip: GripStyle, path: String, base: f64, k: f64, field_limits: (f64, f64), name: (&str, &str)) -> Handle {
    let (a, b) = ((field_limits.0 - base) / k, (field_limits.1 - base) / k);
    handle(Spec { entity, tag, kind, anchor, grip, writes: vec![field_binding(path, Affine { base, k }, "m", Some(Quantise::Round { step: 0.001 }))], snap: grid(0.005), limits: (a.min(b), a.max(b)), name: loc(name.0, name.1) })
}

fn dimension_pair(entity: &EntityRef, tag: &str, kind: HandleKind, anchor: Vec3, grip: GripStyle, paths: [String; 2], base: f64, k: f64, field_limits: (f64, f64), name: (&str, &str)) -> Handle {
    let (a, b) = ((field_limits.0 - base) / k, (field_limits.1 - base) / k);
    let writes = paths.into_iter().map(|path| field_binding(path, Affine { base, k }, "m", Some(Quantise::Round { step: 0.001 }))).collect();
    handle(Spec { entity, tag, kind, anchor, grip, writes, snap: grid(0.005), limits: (a.min(b), a.max(b)), name: loc(name.0, name.1) })
}

fn action_arrow(entity: &EntityRef, tag: &str, axis: Vec3, at: &Vec3, path: String, value: f64, per_metre: f64, unit: &str, name: (String, String)) -> Handle {
    super::scene::projection_kit::handle_kit::load_arrow(entity, tag, axis, at, path, value, per_metre, unit, loc(&name.0, &name.1))
}

fn section_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    let Some(s) = d.sections.iter().find(|s| s.id == member.section_id && s.h > 0.0 && s.b > 0.0) else { return Vec::new() };
    section_grips(&entity_ref(EntityKind::Member, id, "members"), s, &pose.frame, &along(&pose.frame.origin, &pose.frame.x, member.length / 2.0))
}

fn section_row_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(s) = d.sections.iter().find(|s| s.id == id && s.h > 0.0 && s.b > 0.0) else { return Vec::new() };
    let f = member_frame(&layout.origin(id), &Vec3::X);
    section_grips(&entity_ref(EntityKind::Member, id, "sections"), s, &f, &along(&f.origin, &f.x, 0.5))
}

/// 📐️ The dimension grips of a section row, anchored on the section drawn around `mid` in the frame `f`; shared by the member that uses the section and by the section row itself.
fn section_grips(entity: &EntityRef, s: &model::SteelSection, f: &Frame, mid: &Vec3) -> Vec<Handle> {
    let path = |field: &str| format!("sections[id={}].{field}", s.id);
    let top = along(mid, &f.z, s.h / 2.0);
    match s.kind {
        semio_s_artifact_norm_en1993_model::SectionKind::RolledI | semio_s_artifact_norm_en1993_model::SectionKind::WeldedI | semio_s_artifact_norm_en1993_model::SectionKind::Hss => vec![
        dimension(entity, "section-h", HandleKind::ScalarAlong { axis: f.z.clone() }, top.clone(), GripStyle::Arrow, path("h"), s.h, 2.0, (0.05, 1.5), ("Section height h", "Profilhöhe h")),
        dimension(entity, "section-b", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&top, &f.y, s.b / 2.0), GripStyle::Arrow, path("b"), s.b, 2.0, (0.03, 0.8), ("Flange width b", "Flanschbreite b")),
        dimension(entity, "section-tw", HandleKind::FaceOffset { normal: f.y.clone() }, along(mid, &f.y, s.tw / 2.0), GripStyle::Diamond, path("tw"), s.tw, 2.0, (0.002, 0.06), ("Web thickness t_w", "Stegdicke t_w")),
        dimension(entity, "section-tf", HandleKind::ScalarAlong { axis: mul(&f.z, -1.0) }, along(&along(mid, &f.z, s.h / 2.0 - s.tf), &f.y, s.b / 2.0), GripStyle::Diamond, path("tf"), s.tf, 1.0, (0.002, 0.1), ("Flange thickness t_f", "Flanschdicke t_f")),
        ],
        semio_s_artifact_norm_en1993_model::SectionKind::Chs => vec![
            dimension_pair(entity, "section-d", HandleKind::ScalarAlong { axis: f.z.clone() }, top.clone(), GripStyle::Arrow, [path("h"), path("b")], s.h, 2.0, (0.03, 1.5), ("Diameter D", "Durchmesser D")),
            dimension_pair(entity, "section-t", HandleKind::FaceOffset { normal: f.y.clone() }, along(mid, &f.y, s.h / 2.0), GripStyle::Diamond, [path("tw"), path("tf")], s.tw, 1.0, (0.002, 0.06), ("Wall thickness t", "Wanddicke t")),
        ],
        semio_s_artifact_norm_en1993_model::SectionKind::Angle => vec![
            dimension(entity, "section-h", HandleKind::ScalarAlong { axis: f.z.clone() }, top.clone(), GripStyle::Arrow, path("h"), s.h, 2.0, (0.03, 1.0), ("Long leg h", "Langer Schenkel h")),
            dimension(entity, "section-b", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&along(mid, &f.z, -s.h / 2.0), &f.y, s.b / 2.0), GripStyle::Arrow, path("b"), s.b, 2.0, (0.03, 1.0), ("Short leg b", "Kurzer Schenkel b")),
            dimension_pair(entity, "section-t", HandleKind::FaceOffset { normal: f.y.clone() }, along(mid, &f.y, s.tw / 2.0), GripStyle::Diamond, [path("tw"), path("tf")], s.tw, 1.0, (0.002, 0.06), ("Leg thickness t", "Schenkeldicke t")),
        ],
    }
}

fn material_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(material) = d.materials.iter().find(|m| m.id == id && m.fy > 0.0) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Plate, id, "materials");
    vec![action_arrow(&entity, "yield-strength", Vec3::Z, &layout.origin(id), format!("materials[id={id}].fy"), material.fy, STRENGTH_PER_METRE, "Pa", (format!("Yield strength f_y of {id}"), format!("Streckgrenze f_y von {id}")))]
}

fn tension_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(cable) = d.tension_components.iter().find(|c| c.id == id && c.cable_length > 0.0) else { return Vec::new() };
    let o = layout.origin(id);
    let entity = entity_ref(EntityKind::Member, id, "tensionComponents");
    vec![dimension(&entity, "cable-length", HandleKind::ScalarAlong { axis: Vec3::Z }, along(&o, &Vec3::Z, cable.cable_length), GripStyle::Arrow, format!("tensionComponents[id={id}].cableLength"), cable.cable_length, 1.0, (0.1, 200.0), ("Cable length l", "Seillänge l"))]
}

/// 🏗️ The grips of a frame action glyph: the force components `fx` and `fz` (nodal force or line load) or the moment `my`, as arrows from the point of application written to `frameActions[id=..]`.
fn frame_action_handles(d: &En1993Snapshot, layout: &Layout, action: &model::FrameAction, raw: &str) -> Vec<Handle> {
    let entity = EntityRef::new(EntityKind::Load, raw, "frameActions");
    let node = |id: &str| d.frame_nodes.iter().find(|n| n.id == id).map(|n| layout.frame_point(n.x, n.z));
    let at = match action.kind {
        model::FrameActionKind::BarUniform => {
            let Some(bar) = d.frame_bars.iter().find(|b| b.id == action.target_id) else { return Vec::new() };
            let (Some(a), Some(c)) = (node(&bar.start_node_id), node(&bar.end_node_id)) else { return Vec::new() };
            let k = 0.5 * (action.start_ratio + action.end_ratio);
            v3(a.x + (c.x - a.x) * k, a.y + (c.y - a.y) * k, a.z + (c.z - a.z) * k)
        }
        _ => match node(&action.target_id) {
            Some(at) => at,
            None => return Vec::new(),
        },
    };
    let arrow = |tag: &str, axis: Vec3, field: &str, value: f64, per_metre: f64, unit: &str, en: &str, de: &str| {
        action_arrow(&entity, tag, mul(&axis, if value < 0.0 { -1.0 } else { 1.0 }), &at, format!("frameActions[id={}].{field}", action.id), value, per_metre, unit, (format!("{en} {}", action.id), format!("{de} {}", action.id)))
    };
    match action.kind {
        model::FrameActionKind::NodalMoment => vec![arrow("magnitude", Vec3::Z, "my", action.my, MOMENT_PER_METRE, "Nm", "Moment M_y of", "Moment M_y von")],
        _ => vec![arrow("magnitude-x", Vec3::X, "fx", action.fx, FORCE_PER_METRE, "N", "Component x of", "Komponente x von"), arrow("magnitude-z", Vec3::Z, "fz", action.fz, FORCE_PER_METRE, "N", "Component z of", "Komponente z von")],
    }
}

/// 🏹 The grips of the load glyphs and attachments of one member: the tower leg height and diameter, the crane runway web, flange and wheel eccentricity, the fire protection thickness, and the magnitude of every nested action row (`memberActions[].action.*`, `towerLegs[].actions[].force`, `craneRunways[].actions[].force`).
fn glyph_handles(d: &En1993Snapshot, layout: &Layout, raw: &str) -> Vec<Handle> {
    let section_of = |member: &str| d.members.iter().find(|m| m.id == member).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id));
    let mut out = Vec::new();
    if let Some((index, row)) = d.member_actions.iter().enumerate().find(|(_, r)| raw.strip_prefix(r.id.as_str()).is_some_and(|rest| matches!(rest, ":n" | ":vz" | ":my"))) {
        let (Some(pose), Some(section)) = (layout.member(&row.member_id), section_of(&row.member_id)) else { return out };
        let (f, a) = (&pose.frame, &row.action);
        let shift = 0.2 * (index % 4) as f64;
        let end = along(&along(&f.origin, &f.x, pose.length), &f.y, shift);
        let mid = along(&along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, section.h / 2.0), &f.y, shift);
        let entity = EntityRef::new(EntityKind::Load, raw, "memberActions");
        let path = |field: &str| format!("memberActions[id={}].action.{field}", row.id);
        match raw.rsplit(':').next() {
            Some("n") if a.n.abs() > 1e-9 => out.push(action_arrow(&entity, "magnitude", mul(&f.x, if a.n > 0.0 { -1.0 } else { 1.0 }), &end, path("n"), a.n, FORCE_PER_METRE, "N", (format!("Axial force N_Ed of {}", row.id), format!("Normalkraft N_Ed von {}", row.id)))),
            Some("vz") if a.vz.abs() > 1e-9 => out.push(action_arrow(&entity, "magnitude", mul(&f.z, if a.vz > 0.0 { -1.0 } else { 1.0 }), &mid, path("vz"), a.vz, FORCE_PER_METRE, "N", (format!("Shear force V_z,Ed of {}", row.id), format!("Querkraft V_z,Ed von {}", row.id)))),
            Some("my") if a.my.abs() > 1e-9 => out.push(action_arrow(&entity, "magnitude", f.z.clone(), &end, path("my"), a.my, MOMENT_PER_METRE, "Nm", (format!("Bending moment M_y,Ed of {}", row.id), format!("Biegemoment M_y,Ed von {}", row.id)))),
            _ => {}
        }
        return out;
    }
    if let Some(action) = d.frame_actions.iter().find(|a| matches!(raw.strip_prefix(a.id.as_str()), Some(":force" | ":moment" | ":line"))) {
        return frame_action_handles(d, layout, action, raw);
    }
    if let Some(tower) = d.tower_legs.iter().find(|t| t.id == raw) {
        let (Some(pose), Some(section)) = (layout.member(&tower.member_id), section_of(&tower.member_id)) else { return out };
        let f = &pose.frame;
        let entity = EntityRef::new(EntityKind::Load, raw, "towerLegs");
        let mid = along(&f.origin, &f.x, pose.length / 2.0);
        if tower.height > 0.0 {
            out.push(dimension(&entity, "tower-height", HandleKind::ScalarAlong { axis: f.x.clone() }, along(&f.origin, &f.x, tower.height), GripStyle::Arrow, format!("towerLegs[id={raw}].height"), tower.height, 1.0, (0.5, 300.0), ("Tower height", "Turmhöhe")));
        }
        if tower.diameter > 0.0 {
            out.push(dimension(&entity, "tower-diameter", HandleKind::Radial { centre: mid.clone() }, along(&mid, &f.y, tower.diameter / 2.0), GripStyle::Ring, format!("towerLegs[id={raw}].diameter"), tower.diameter, 2.0, (0.02, 10.0), ("Tower diameter", "Turmdurchmesser")));
        }
        let at = along(&mid, &f.y, section.b / 2.0);
        for a in tower.actions.iter().filter(|a| d.load_cases.iter().any(|lc| lc.id == a.load_case_id && lc.kind == semio_s_artifact_norm_en1993_model::ActionKind::Wind)) {
            out.push(action_arrow(&entity, &format!("action-{}", a.id), Vec3::Y, &at, format!("towerLegs[id={raw}].actions[id={}].force", a.id), a.force, FORCE_PER_METRE, "N", (format!("Wind force {} of {raw}", a.id), format!("Windkraft {} von {raw}", a.id))));
        }
        return out;
    }
    if let Some(crane) = d.crane_runways.iter().find(|c| c.id == raw) {
        let (Some(pose), Some(section)) = (layout.member(&crane.member_id), section_of(&crane.member_id)) else { return out };
        let f = &pose.frame;
        let entity = EntityRef::new(EntityKind::Load, raw, "craneRunways");
        let mid = along(&f.origin, &f.x, pose.length / 2.0);
        let top = along(&mid, &f.z, section.h / 2.0);
        let path = |field: &str| format!("craneRunways[id={raw}].{field}");
        out.push(dimension(&entity, "web-thickness", HandleKind::FaceOffset { normal: f.y.clone() }, along(&mid, &f.y, crane.web_thickness / 2.0), GripStyle::Diamond, path("webThickness"), crane.web_thickness, 2.0, (0.003, 0.06), ("Web thickness t_w", "Stegdicke t_w")));
        out.push(dimension(&entity, "flange-thickness", HandleKind::ScalarAlong { axis: mul(&f.z, -1.0) }, along(&top, &f.z, -crane.flange_thickness), GripStyle::Diamond, path("flangeThickness"), crane.flange_thickness, 1.0, (0.0, 0.1), ("Flange thickness t_f", "Flanschdicke t_f")));
        out.push(dimension(&entity, "wheel-eccentricity", HandleKind::ScalarAlong { axis: f.y.clone() }, along(&top, &f.y, crane.wheel_eccentricity), GripStyle::Ring, path("wheelEccentricity"), crane.wheel_eccentricity, 1.0, (-0.3, 0.3), ("Wheel eccentricity e", "Radlastausmitte e")));
        for a in &crane.actions {
            out.push(action_arrow(&entity, &format!("action-{}", a.id), mul(&Vec3::Z, -1.0), &top, format!("craneRunways[id={raw}].actions[id={}].force", a.id), a.force, FORCE_PER_METRE, "N", (format!("Wheel load {} of {raw}", a.id), format!("Radlast {} von {raw}", a.id))));
        }
        return out;
    }
    if let Some(fire) = d.fire_exposures.iter().find(|x| x.id == raw) {
        let (Some(pose), Some(section)) = (layout.member(&fire.member_id), section_of(&fire.member_id)) else { return out };
        let f = &pose.frame;
        let entity = EntityRef::new(EntityKind::Load, raw, "fireExposures");
        let bottom = along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, -section.h / 2.0);
        out.push(dimension(&entity, "protection-thickness", HandleKind::ScalarAlong { axis: mul(&f.z, -1.0) }, along(&bottom, &f.z, -fire.protection_thickness), GripStyle::Arrow, format!("fireExposures[id={raw}].protectionThickness"), fire.protection_thickness, 1.0, (0.0, 0.15), ("Fire protection thickness", "Dicke des Brandschutzes")));
    }
    out
}

fn action_kind_options() -> Vec<(String, &'static str, &'static str)> {
    use semio_s_artifact_norm_en1993_model::ActionKind as K;
    [(K::Permanent, "Permanent (G)", "Ständig (G)"), (K::Imposed, "Imposed (Q)", "Nutzlast (Q)"), (K::Wind, "Wind (W)", "Wind (W)"), (K::Snow, "Snow (S)", "Schnee (S)"), (K::Accidental, "Accidental (A)", "Außergewöhnlich (A)")].into_iter().map(|(v, en, de)| (format!("{v:?}"), en, de)).collect()
}

fn action_category_options() -> Vec<(String, &'static str, &'static str)> {
    [("self", "Self-weight", "Eigengewicht"), ("office", "Office / residential", "Büro / Wohnen"), ("shopping", "Shopping / storage", "Verkauf / Lager"), ("wind", "Wind", "Wind"), ("snow", "Snow", "Schnee")].into_iter().map(|(t, en, de)| (t.to_string(), en, de)).collect()
}

/// 🏋️ The two choice grips of a load case card: the action kind and the ψ category; the magnitudes are gripped on the member action glyphs.
fn case_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(case) = d.load_cases.iter().find(|c| c.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Plate, id, "loadCases");
    let o = layout.origin(id);
    vec![
        choice_handle(&entity, "case-kind", along(&o, &Vec3::Z, 0.1), format!("loadCases[id={id}].kind"), &action_kind_options(), loc(&format!("Action kind of {}", case.name), &format!("Einwirkungsart von {}", case.name))),
        choice_handle(&entity, "case-category", along(&o, &Vec3::Z, 0.2), format!("loadCases[id={id}].category"), &action_category_options(), loc(&format!("ψ category of {}", case.name), &format!("ψ-Kategorie von {}", case.name))),
    ]
}

/// 🧲️ The move grips of a placed entity (members also rotate); a drag writes `placements[id=ENTITY].x|y|yaw`.
fn move_handles(d: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let stored: Vec<_> = d.placements.iter().map(|p| Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    let moves = super::scene::projection_kit::handle_kit::move_handles;
    if let Some(pose) = layout.member(id) {
        return moves(&entity_ref(EntityKind::Member, id, "members"), id, &pose.frame.origin, &stored, 0.0, true);
    }
    let rows: [(&str, Vec<&str>); 10] = [
        ("sections", d.sections.iter().map(|e| e.id.as_str()).collect()),
        ("platedPanels", d.plated_panels.iter().map(|e| e.id.as_str()).collect()),
        ("piles", d.piles.iter().map(|e| e.id.as_str()).collect()),
        ("coldFormedMembers", d.cold_formed_members.iter().map(|e| e.id.as_str()).collect()),
        ("tensionComponents", d.tension_components.iter().map(|e| e.id.as_str()).collect()),
        ("siloShells", d.silo_shells.iter().map(|e| e.id.as_str()).collect()),
        ("parametricSections", d.parametric_sections.iter().map(|e| e.id.as_str()).collect()),
        ("bridgeFatigue", d.bridge_fatigue.iter().map(|e| e.id.as_str()).collect()),
        ("materials", d.materials.iter().map(|e| e.id.as_str()).collect()),
        ("loadCases", d.load_cases.iter().map(|e| e.id.as_str()).collect()),
    ];
    rows.iter().find(|(_, ids)| ids.contains(&id)).map_or_else(Vec::new, |(collection, _)| moves(&EntityRef::new(EntityKind::Group, id, format!("{collection}[id={id}]")), id, &layout.origin(id), &stored, 0.0, false))
}

/// 🕹️ The handles of one document entity, anchored in the given layout.
pub fn build(document: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let glyphs = glyph_handles(document, layout, id);
    let id = base_id(id);
    let dedicated = [member_handles(document, layout, id), joint_handles(document, layout, id), panel_handles(document, layout, id), silo_handles(document, layout, id), pile_handles(document, layout, id), storey_handles(document, layout, id), member_length_handles(document, layout, id), warping_support_handles(document, layout, id), joint_plate_handles(document, layout, id), cold_formed_handles(document, layout, id), restraint_handles(document, layout, id), built_up_handles(document, layout, id), weld_group_handles(document, layout, id), hollow_joint_handles(document, layout, id), joint_row_handles(document, layout, id), plate_row_handles(document, layout, id), foundation_block_handles(document, layout, id), parametric_section_handles(document, layout, id), frame_node_handles(document, layout, id), annex_handles(document, layout, id), move_handles(document, layout, id), section_handles(document, layout, id), section_row_handles(document, layout, id), tension_handles(document, layout, id), material_handles(document, layout, id), case_handles(document, layout, id), glyphs].concat();
    let scalars = super::scalar_grips::handles(document, layout, id, &dedicated);
    [dedicated, scalars].concat()
}

/// 🆔️ The handle ids a scene entity advertises.
pub fn ids(document: &En1993Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their document entity), in selection order, without duplicates.
pub fn handles(document: &En1993Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}
