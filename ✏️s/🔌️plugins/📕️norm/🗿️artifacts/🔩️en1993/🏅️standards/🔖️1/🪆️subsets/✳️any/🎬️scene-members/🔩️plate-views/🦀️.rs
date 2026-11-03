//! 🔩️ Joint plates and foundation blocks in 3D: fin, web cover and flange cover plates with their bolts, the foundation block with its grout layer and anchors. Each document entity owns its parts and is coloured by its own checks.

use super::kit::*;
use super::layout::Layout;
use super::model::{FoundationBlock, JointPlate, SteelJoint};
use super::plate_layout::{block_pose, bolt_positions, plate_pose, BlockPose, PlatePose};
use super::scene::*;
use super::{handles, SceneInput};

fn rect(width: f64, height: f64) -> Polygon2 {
    let (w, h) = (width / 2.0, height / 2.0);
    Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)])
}

fn part(kind: EntityKind, id: String, collection: &'static str, geometry: Geometry, role: MaterialRole, binding: &Option<UtilisationBinding>, name: LocalizedCopy, detail: LocalizedCopy) -> Spec {
    Spec { kind, id, collection, geometry, role, binding: binding.clone(), name, detail, layer: LAYER_JOINTS, handles: Vec::new(), deformation: None }
}

fn plates(out: &mut Out, p: &JointPlate, pose: &PlatePose, binding: &Option<UtilisationBinding>, handle_ids: Vec<String>) {
    let mut members = Vec::new();
    for (i, frame) in pose.frames.iter().enumerate() {
        let id = format!("{}:plate-{}", p.id, i + 1);
        out.push(
            part(EntityKind::Plate, id.clone(), "jointPlates", Geometry::Plate(PlateGeometry { frame: frame.clone(), outline: rect(p.plate_height, p.plate_width), thickness: pose.thickness, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Steel, binding, loc(&format!("Plate {} of {}", i + 1, p.label), &format!("Blech {} von {}", i + 1, p.label)), loc(&format!("{:.0} × {:.0} × {:.0} mm, {}", p.plate_height * 1e3, p.plate_width * 1e3, p.plate_thickness * 1e3, p.role), &format!("{:.0} × {:.0} × {:.0} mm, {}", p.plate_height * 1e3, p.plate_width * 1e3, p.plate_thickness * 1e3, p.role))),
            &[frame.origin.clone()],
        );
        members.push(entity_ref(EntityKind::Plate, &id, "jointPlates"));
    }
    for (i, at) in bolt_positions(pose, p).into_iter().enumerate() {
        let id = format!("{}:bolt-{}", p.id, i + 1);
        let (from, to) = (along(&at, &pose.normal, pose.top), along(&at, &pose.normal, -pose.bottom));
        out.push(
            part(
                EntityKind::Fastener,
                id.clone(),
                "jointPlates",
                Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Bolt { class: p.bolt_class.clone(), preload: None }, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: p.bolt_diameter.max(1e-3), head: HeadShape::Hex, pattern: None, hole_clearance: Some((p.hole_diameter - p.bolt_diameter).max(0.0)) }),
                MaterialRole::Bolt,
                binding,
                loc(&format!("Bolt {} of {}", i + 1, p.label), &format!("Schraube {} von {}", i + 1, p.label)),
                loc(&format!("M{:.0} class {}", p.bolt_diameter * 1e3, p.bolt_class), &format!("M{:.0} Festigkeitsklasse {}", p.bolt_diameter * 1e3, p.bolt_class)),
            ),
            &[from, to],
        );
        members.push(entity_ref(EntityKind::Fastener, &id, "jointPlates"));
    }
    out.push(
        Spec { handles: handle_ids, ..part(EntityKind::Group, p.id.clone(), "jointPlates", Geometry::Group { members }, MaterialRole::Steel, binding, loc(&p.label, &p.label), loc(&format!("{} of {}", p.role, p.joint_id), &format!("{} von {}", p.role, p.joint_id))) },
        &[pose.bolt_centre.clone()],
    );
}

fn block(out: &mut Out, b: &FoundationBlock, pose: &BlockPose, diameter: f64, binding: &Option<UtilisationBinding>, handle_ids: Vec<String>, zones: &Zones) {
    let solid = |frame: &Frame, size: &Vec3| Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: frame.clone(), size: size.clone() } });
    let (block_id, grout_id) = (format!("{}:block", b.id), format!("{}:grout", b.id));
    out.push(part(EntityKind::Solid, block_id.clone(), "foundationBlocks", solid(&pose.block, &pose.size), MaterialRole::Concrete, binding, loc("Concrete block", "Betonblock"), loc(&format!("{:.2} × {:.2} × {:.2} m, f_ck {:.0} MPa", b.block_length, b.block_width, b.block_depth, b.concrete_fck / 1e6), &format!("{:.2} × {:.2} × {:.2} m, f_ck {:.0} MPa", b.block_length, b.block_width, b.block_depth, b.concrete_fck / 1e6))), &[pose.block.origin.clone()]);
    out.push(part(EntityKind::Solid, grout_id.clone(), "foundationBlocks", solid(&pose.grout, &pose.grout_size), MaterialRole::Neutral, binding, loc("Grout layer", "Vergussschicht"), loc(&format!("t_g {:.0} mm, f_ck {:.0} MPa", b.grout_thickness * 1e3, b.grout_fck / 1e6), &format!("t_g {:.0} mm, f_ck {:.0} MPa", b.grout_thickness * 1e3, b.grout_fck / 1e6))), &[pose.grout.origin.clone()]);
    let mut members = vec![entity_ref(EntityKind::Solid, &block_id, "foundationBlocks"), entity_ref(EntityKind::Solid, &grout_id, "foundationBlocks")];
    for (i, (from, to)) in pose.anchors.iter().enumerate() {
        let id = format!("{}:anchor-{}", b.id, i + 1);
        out.push(
            part(
                EntityKind::Fastener,
                id.clone(),
                "foundationBlocks",
                Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Anchor, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: diameter.max(1e-3), head: HeadShape::Hex, pattern: None, hole_clearance: None }),
                MaterialRole::Bolt,
                binding,
                loc(&format!("Anchor {}", i + 1), &format!("Anker {}", i + 1)),
                loc(&format!("M{:.0} class {}, h_ef {:.0} mm", b.anchor_diameter * 1e3, b.anchor_class, b.anchor_embedment * 1e3), &format!("M{:.0} Festigkeitsklasse {}, h_ef {:.0} mm", b.anchor_diameter * 1e3, b.anchor_class, b.anchor_embedment * 1e3)),
            ),
            &[from.clone(), to.clone()],
        );
        members.push(entity_ref(EntityKind::Fastener, &id, "foundationBlocks"));
    }
    members.extend(zones.push(out, b, pose, diameter));
    out.push(Spec { handles: handle_ids, ..part(EntityKind::Group, b.id.clone(), "foundationBlocks", Geometry::Group { members }, MaterialRole::Concrete, binding, loc(&b.label, &b.label), loc(&format!("under {}", b.joint_id), &format!("unter {}", b.joint_id))) }, &[pose.plate_top.clone()]);
}

const RING: u32 = 16;

struct Zones {
    strip: Option<(f64, f64, f64, f64)>,
    strip_binding: Option<UtilisationBinding>,
    cone_binding: Option<UtilisationBinding>,
}

fn ring(centre: &Vec3, r: f64, u: &Vec3, v: &Vec3) -> Vec<Vec3> {
    (0..RING).map(|i| {
        let t = std::f64::consts::TAU * f64::from(i) / f64::from(RING);
        along(&along(centre, u, r * t.cos()), v, r * t.sin())
    }).collect()
}

fn frustum(top: &Vec3, bottom: &Vec3, r_top: f64, r_bottom: f64, u: &Vec3, v: &Vec3) -> Geometry {
    const N: u32 = RING;
    let vertices: Vec<Vec3> = ring(top, r_top, u, v).into_iter().chain(ring(bottom, r_bottom, u, v)).chain([top.clone(), bottom.clone()]).collect();
    let (ct, cb) = (2 * N, 2 * N + 1);
    let triangles = (0..N).flat_map(|i| {
        let j = (i + 1) % N;
        [Triangle { a: i, b: j, c: N + i }, Triangle { a: j, b: N + j, c: N + i }, Triangle { a: ct, b: j, c: i }, Triangle { a: cb, b: N + i, c: N + j }]
    });
    Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices, triangles: triangles.collect() } })
}

impl Zones {
    fn of(input: &SceneInput, b: &FoundationBlock, joint: &SteelJoint) -> Self {
        let annex = input.report.checks.iter().find(|c| c.id == format!("en1993.1-8.6.2.cbend.{}", b.id)).map(|c| c.annex);
        let flange = super::checks::plate_checks::column_flange_width(input.document, joint);
        let strip = annex.zip(flange).and_then(|(a, bfc)| super::checks::plate_checks::compression_strip(b, joint, bfc, a)).map(|s| (s.b_eff / 1e3, s.l_eff / 1e3, s.c / 1e3, joint.base_lever_right));
        Self { strip, strip_binding: projection_kit::binding_of_subject(input.report, &b.id, &["en1993.1-8.6.2.cbend"]), cone_binding: projection_kit::binding_of_subject(input.report, &b.id, &["en1993.1-8.6.2.cone"]) }
    }

    fn push(&self, out: &mut Out, b: &FoundationBlock, pose: &BlockPose, diameter: f64) -> Vec<EntityRef> {
        let mut members = Vec::new();
        if let Some((b_eff, l_eff, c, offset)) = self.strip {
            let id = format!("{}:c-strip", b.id);
            let origin = along(&along(&pose.grout.origin, &pose.up, pose.grout_size.z), &pose.length_axis, offset);
            let frame = Frame { origin: origin.clone(), x: pose.length_axis.clone(), y: pose.width_axis.clone(), z: pose.up.clone() };
            let size = Vec3::new(b_eff, l_eff, pose.grout_size.z.max(0.01));
            let detail = format!("c = {:.0} mm, b_eff = {:.0} mm, l_eff = {:.0} mm", c * 1e3, b_eff * 1e3, l_eff * 1e3);
            let spec = part(EntityKind::Solid, id.clone(), "foundationBlocks", Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, size } }), MaterialRole::Neutral, &self.strip_binding, loc("Effective c-strip under the column flange", "Wirksamer c-Streifen unter dem Stützenflansch"), loc(&detail, &detail));
            out.push_tuned(spec, &[origin], |e| e.appearance.opacity = 0.45);
            members.push(entity_ref(EntityKind::Solid, &id, "foundationBlocks"));
        }
        let hef = b.anchor_embedment.max(0.05);
        for (i, (_, to)) in pose.anchors.iter().enumerate() {
            let id = format!("{}:cone-{}", b.id, i + 1);
            let top = along(to, &pose.up, hef);
            let detail = format!("s_cr,N = {:.0} mm, h_ef = {:.0} mm", 3.0 * hef * 1e3, hef * 1e3);
            let spec = part(EntityKind::Solid, id.clone(), "foundationBlocks", frustum(&top, to, 1.5 * hef, diameter.max(1e-3), &pose.length_axis, &pose.width_axis), MaterialRole::Concrete, &self.cone_binding, loc(&format!("Concrete cone of anchor {}", i + 1), &format!("Betonausbruchkegel von Anker {}", i + 1)), loc(&detail, &detail));
            out.push_tuned(spec, &[top, to.clone()], |e| e.appearance.opacity = 0.35);
            members.push(entity_ref(EntityKind::Solid, &id, "foundationBlocks"));
        }
        members
    }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for p in &d.joint_plates {
        if !out.own("jointPlates", &p.id) {
            continue;
        }
        if let Some(pose) = plate_pose(d, layout, p) {
            plates(out, p, &pose, &binding(input.report, &[p.id.as_str()]), handles::ids(d, layout, &p.id));
        }
    }
    for b in &d.foundation_blocks {
        if !out.own("foundationBlocks", &b.id) {
            continue;
        }
        if let Some(pose) = block_pose(d, layout, b) {
            let zones = d.joints.iter().find(|j| j.id == b.joint_id).map_or(Zones { strip: None, strip_binding: None, cone_binding: None }, |j| Zones::of(input, b, j));
            block(out, b, &pose, b.anchor_diameter, &binding(input.report, &[b.id.as_str()]), handles::ids(d, layout, &b.id), &zones);
        }
    }
}
