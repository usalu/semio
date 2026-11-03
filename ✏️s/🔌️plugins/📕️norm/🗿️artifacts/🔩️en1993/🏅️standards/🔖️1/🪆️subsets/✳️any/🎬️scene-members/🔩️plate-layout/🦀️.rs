//! 🔩️ Poses of the joint plates, their bolts, the foundation blocks and their anchors in the member layout: the single geometry the plate views and the plate grips share.
//!
//! 🧭️ A fin plate lies against the web and ends at the member end, its bolt line at the eccentricity `z` before it; web cover plates sit on the web on both sides of the splice; flange cover plates lie on the top flange. A foundation block sits below the member start, the grout between its top and the base plate, the anchors on the tension side.

use super::kit::along;
use super::layout::Layout;
use super::model::{En1993Snapshot, FoundationBlock, JointPlate};
use super::scene::*;

/// 🔩️ Where one joint plate, its second plate and its bolts are.
pub struct PlatePose {
    pub frames: Vec<Frame>,
    pub normal: Vec3,
    pub line: Vec3,
    pub across: Vec3,
    pub bolt_centre: Vec3,
    pub thickness: f64,
    pub height: f64,
    pub width: f64,
    pub top: f64,
    pub bottom: f64,
}

/// 🧱️ Where one foundation block, its grout and its anchors are.
pub struct BlockPose {
    pub block: Frame,
    pub grout: Frame,
    pub size: Vec3,
    pub grout_size: Vec3,
    pub anchors: Vec<(Vec3, Vec3)>,
    pub plate_top: Vec3,
    pub length_axis: Vec3,
    pub width_axis: Vec3,
    pub up: Vec3,
}

/// 🔩️ The pose of `p`, `None` while its joint, member or section is missing.
pub fn plate_pose(d: &En1993Snapshot, layout: &Layout, p: &JointPlate) -> Option<PlatePose> {
    let joint = d.joints.iter().find(|j| j.id == p.joint_id)?;
    let pose = layout.member(&joint.member_id)?;
    let section = d.members.iter().find(|m| m.id == joint.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))?;
    let (f, t, bolt) = (&pose.frame, p.plate_thickness.max(0.002), 0.5 * p.bolt_diameter.max(0.008));
    let end = along(&f.origin, &f.x, pose.length);
    if p.role == "flangeCover" {
        let centre = along(&end, &f.z, section.h / 2.0 + t / 2.0);
        return Some(PlatePose { frames: vec![Frame { origin: centre.clone(), x: f.x.clone(), y: f.y.clone(), z: f.z.clone() }], normal: f.z.clone(), line: f.x.clone(), across: f.y.clone(), bolt_centre: centre, thickness: t, height: p.plate_height, width: p.plate_width, top: t / 2.0 + bolt, bottom: section.tf + t / 2.0 + bolt });
    }
    let fin = p.role == "finPlate";
    let side = section.tw / 2.0 + t / 2.0;
    let base = if fin { along(&end, &f.x, -p.plate_width / 2.0) } else { end.clone() };
    let frames = (0..p.plate_count.clamp(1, 2)).map(|i| Frame { origin: along(&base, &f.y, if i == 0 { side } else { -side }), x: f.z.clone(), y: f.x.clone(), z: f.y.clone() }).collect();
    let bolt_centre = along(&end, &f.x, if fin { -p.eccentricity } else { p.eccentricity });
    let other = if p.plate_count >= 2 { side + t / 2.0 + bolt } else { section.tw / 2.0 + bolt };
    Some(PlatePose { frames, normal: f.y.clone(), line: f.z.clone(), across: f.x.clone(), bolt_centre, thickness: t, height: p.plate_height, width: p.plate_width, top: side + t / 2.0 + bolt, bottom: other })
}

/// 🔩️ The world position of every bolt of `p`, in the order of `checks::plate_checks::bolt_points`.
pub fn bolt_positions(pose: &PlatePose, p: &JointPlate) -> Vec<Vec3> {
    super::checks::plate_checks::bolt_points(p).into_iter().map(|(x, y)| along(&along(&pose.bolt_centre, &pose.across, x / 1e3), &pose.line, y / 1e3)).collect()
}

/// 🧱️ The pose of `b`, `None` while its joint or member is missing.
pub fn block_pose(d: &En1993Snapshot, layout: &Layout, b: &FoundationBlock) -> Option<BlockPose> {
    let joint = d.joints.iter().find(|j| j.id == b.joint_id)?;
    let pose = layout.member(&joint.member_id)?;
    let f = &pose.frame;
    let (tp, tg) = (joint.plate_thickness.max(0.005), b.grout_thickness.max(0.005));
    let (lp, bp) = (joint.base_plate_length.max(0.1), joint.plate_width.max(0.1));
    let frame = |depth: f64| Frame { origin: along(&f.origin, &f.x, -depth), x: f.y.clone(), y: f.z.clone(), z: f.x.clone() };
    let n = b.anchor_count.clamp(1, 8) as usize;
    let spacing = (bp * 0.7 / n as f64).min(0.12);
    let side = -joint.base_lever_left.max(0.25 * lp);
    let anchors = (0..n)
        .map(|i| {
            let at = along(&along(&f.origin, &f.y, side), &f.z, (i as f64 - (n as f64 - 1.0) / 2.0) * spacing);
            (along(&at, &f.x, tp + 0.05), along(&at, &f.x, -(tp + tg + b.anchor_embedment.max(0.05))))
        })
        .collect();
    Some(BlockPose { block: frame(tp + tg + b.block_depth), grout: frame(tp + tg), size: Vec3::new(b.block_length, b.block_width, b.block_depth), grout_size: Vec3::new(lp, bp, tg), anchors, plate_top: along(&f.origin, &f.x, tp), length_axis: f.y.clone(), width_axis: f.z.clone(), up: f.x.clone() })
}
