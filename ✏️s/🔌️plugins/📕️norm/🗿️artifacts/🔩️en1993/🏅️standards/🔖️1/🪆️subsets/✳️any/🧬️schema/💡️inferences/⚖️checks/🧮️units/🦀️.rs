//! 🧮️ EN 1993 check units (shared by editor and viewer): the full evaluation is the ordered fold of one integrity unit and one unit per checked row, in exactly the order of `check_full_steel_structure`.

use super::{check_entity, check_integrity};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, EntityEdge, EntityRef, ReadSet, UnitId, UnitSource};

/// 🧮️ The unit source of the EN 1993 steel subject: `SharedEvaluation<En1993Units>` is the one evaluation path of editor and viewer.
pub struct En1993Units;

const SHARED_READS: [&str; 6] = ["annex", "reliabilityClass", "materials", "sections", "loadCases", "memberActions"];

struct Row<'a> {
    tag: &'static str,
    collection: &'static str,
    id: &'a str,
    index: usize,
    tier: CostTier,
    reads_members: bool,
}

fn all<T>(_: &T) -> bool {
    true
}

fn unit(row: Row<'_>) -> CheckUnit<En1993Snapshot> {
    let mut reads = ReadSet::new(SHARED_READS).with(format!("{}.{}", row.collection, row.index));
    if row.reads_members {
        reads = reads.with("members");
    }
    if matches!(row.collection, "frameBars" | "storeys") {
        reads = reads.with("frameNodes").with("frameBars");
    }
    if row.collection == "fatigueStresses" {
        reads = reads.with("fatigueDetails");
    }
    if row.collection == "formingGains" {
        reads = reads.with("coldFormedMembers");
    }
    if matches!(row.tag, "members" | "vidMaterials") {
        reads = reads.with("materialDuctility");
    }
    let (tag, id) = (row.tag, row.id.to_string());
    CheckUnit::new(UnitId::new(tag, row.id), vec![EntityRef::new(row.collection, row.id)], reads, row.tier, move |document| check_entity(document, tag, &id))
}

fn rows<'a, T>(items: &'a [T], tag: &'static str, collection: &'static str, id: impl Fn(&T) -> &str + 'a, keep: impl Fn(&T) -> bool + 'a, tier: CostTier, reads_members: bool) -> impl Iterator<Item = CheckUnit<En1993Snapshot>> + 'a {
    items.iter().enumerate().filter(move |(_, item)| keep(item)).map(move |(index, item)| unit(Row { tag, collection, id: id(item), index, tier, reads_members }))
}

impl UnitSource for En1993Units {
    type Document = En1993Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["members", "joints", "platedPanels", "siloShells", "tensionComponents", "coldFormedMembers", "piles", "towerLegs", "craneRunways", "frameBars", "frameNodes", "jointRows", "jointPlates", "foundationBlocks", "hollowJoints", "parametricSections", "weldGroups", "builtUpMembers", "restraints", "storeys", "fatigueDetails", "fireExposures", "bridgeFatigue", "materials", "loadCases", "frameActions", "frameMemberBases", "cases", "fatigueStresses", "bucklingLengths", "formingGains", "fastenedJoints", "materialDuctility"], &["en1993.empty", "en1993.integrity."]);

    fn units(d: &En1993Snapshot) -> Vec<CheckUnit<En1993Snapshot>> {
        let integrity = CheckUnit::new(UnitId::new("global", "document"), Vec::new(), ReadSet::new(["*"]), CostTier::T0, check_integrity);
        std::iter::once(integrity)
            .chain(rows(&d.members, "members", "members", |e| e.id.as_str(), all, CostTier::T1, false))
            .chain(rows(&d.joints, "joints", "joints", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.fatigue_details, "fatigueDetails", "fatigueDetails", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.fire_exposures, "fireExposures", "fireExposures", |e| e.id.as_str(), all, CostTier::T2, true))
            .chain(rows(&d.cold_formed_members, "coldFormedMembers", "coldFormedMembers", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.materials, "stainlessMaterials", "materials", |e| e.id.as_str(), |m| m.kind == semio_s_artifact_norm_en1993_model::MaterialKind::Stainless, CostTier::T0, true))
            .chain(rows(&d.plated_panels, "platedPanels", "platedPanels", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.silo_shells, "siloShells", "siloShells", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.members, "memberProperties", "members", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.tension_components, "tensionComponents", "tensionComponents", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.materials, "hssMaterials", "materials", |e| e.id.as_str(), |m| m.fy >= 460.0e6, CostTier::T0, true))
            .chain(rows(&d.bridge_fatigue, "bridgeFatigue", "bridgeFatigue", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.tower_legs, "towerLegs", "towerLegs", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.piles, "piles", "piles", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.crane_runways, "craneRunways", "craneRunways", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.storeys, "storeys", "storeys", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.restraints, "restraints", "restraints", |e| e.id.as_str(), all, CostTier::T1, true))
            .chain(rows(&d.built_up_members, "builtUpMembers", "builtUpMembers", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.weld_groups, "weldGroups", "weldGroups", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.parametric_sections, "parametricSections", "parametricSections", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.hollow_joints, "hollowJoints", "hollowJoints", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.joint_rows, "jointRows", "jointRows", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.frame_nodes, "frameNodes", "frameNodes", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.frame_bars, "frameBars", "frameBars", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.joint_plates, "jointPlates", "jointPlates", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.foundation_blocks, "foundationBlocks", "foundationBlocks", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.fatigue_stresses, "fatigueStresses", "fatigueStresses", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.buckling_lengths, "bucklingLengths", "bucklingLengths", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.forming_gains, "formingGains", "formingGains", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.fastened_joints, "fastenedJoints", "fastenedJoints", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.material_ductility, "materialDuctility", "materialDuctility", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.materials, "vidMaterials", "materials", |e| e.id.as_str(), |m| m.kind != semio_s_artifact_norm_en1993_model::MaterialKind::Stainless, CostTier::T0, false))
            .chain(rows(&d.members, "vidMembers", "members", |e| e.id.as_str(), all, CostTier::T1, false))
            .chain(rows(&d.joints, "vidJoints", "joints", |e| e.id.as_str(), all, CostTier::T0, true))
            .chain(rows(&d.fatigue_details, "vidFatigue", "fatigueDetails", |e| e.id.as_str(), all, CostTier::T0, false))
            .chain(rows(&d.fire_exposures, "vidFire", "fireExposures", |e| e.id.as_str(), all, CostTier::T2, true))
            .chain(rows(&d.cold_formed_members, "vidColdFormed", "coldFormedMembers", |e| e.id.as_str(), all, CostTier::T0, false))
            .collect()
    }

    fn references(d: &En1993Snapshot) -> Vec<EntityEdge> {
        let edge = |from: (&str, &str), to: (&str, &str)| EntityEdge { from: EntityRef::new(from.0, from.1), to: EntityRef::new(to.0, to.1) };
        let mut out = Vec::new();
        for m in &d.members {
            out.push(edge(("members", &m.id), ("sections", &m.section_id)));
            out.push(edge(("members", &m.id), ("materials", &m.material_id)));
        }
        out.extend(d.member_actions.iter().flat_map(|a| [edge(("memberActions", &a.id), ("members", &a.member_id)), edge(("memberActions", &a.id), ("loadCases", &a.load_case_id))]));
        out.extend(d.joints.iter().map(|e| edge(("joints", &e.id), ("members", &e.member_id))));
        out.extend(d.fatigue_details.iter().map(|e| edge(("fatigueDetails", &e.id), ("members", &e.member_id))));
        out.extend(d.fire_exposures.iter().map(|e| edge(("fireExposures", &e.id), ("members", &e.member_id))));
        out.extend(d.bridge_fatigue.iter().map(|e| edge(("bridgeFatigue", &e.id), ("members", &e.member_id))));
        out.extend(d.tower_legs.iter().map(|e| edge(("towerLegs", &e.id), ("members", &e.member_id))));
        out.extend(d.crane_runways.iter().map(|e| edge(("craneRunways", &e.id), ("members", &e.member_id))));
        out.extend(d.storeys.iter().map(|e| edge(("storeys", &e.id), ("members", &e.member_id))));
        out.extend(d.storeys.iter().flat_map(|st| d.members.iter().filter(|m| m.member_type == semio_s_artifact_norm_en1993_model::SteelMemberType::Beam || m.id == st.member_id).flat_map(move |m| [edge(("storeys", &st.id), ("members", &m.id)), edge(("storeys", &st.id), ("sections", &m.section_id)), edge(("storeys", &st.id), ("materials", &m.material_id))])));
        out.extend(d.restraints.iter().map(|e| edge(("restraints", &e.id), ("members", &e.member_id))));
        out.extend(d.built_up_members.iter().map(|e| edge(("builtUpMembers", &e.id), ("members", &e.member_id))));
        out.extend(d.weld_groups.iter().map(|e| edge(("weldGroups", &e.id), ("members", &e.member_id))));
        out.extend(d.hollow_joints.iter().map(|e| edge(("hollowJoints", &e.id), ("members", &e.member_id))));
        out.extend(d.joint_rows.iter().map(|e| edge(("jointRows", &e.id), ("joints", &e.joint_id))));
        for e in &d.joint_plates {
            out.push(edge(("jointPlates", &e.id), ("joints", &e.joint_id)));
            if let Some(member) = d.joints.iter().find(|j| j.id == e.joint_id).and_then(|j| d.members.iter().find(|m| m.id == j.member_id)) {
                out.extend([edge(("jointPlates", &e.id), ("members", &member.id)), edge(("jointPlates", &e.id), ("sections", &member.section_id))]);
            }
        }
        out.extend(d.joints.iter().filter(|e| e.kind == semio_s_artifact_norm_en1993_model::JointKind::BasePlate).filter_map(|e| d.members.iter().find(|m| m.id == e.member_id).map(|m| edge(("joints", &e.id), ("sections", &m.section_id)))));
        for e in &d.foundation_blocks {
            out.push(edge(("foundationBlocks", &e.id), ("joints", &e.joint_id)));
            if let Some(member) = d.joints.iter().find(|j| j.id == e.joint_id).and_then(|j| d.members.iter().find(|m| m.id == j.member_id)) {
                out.extend([edge(("foundationBlocks", &e.id), ("members", &member.id)), edge(("foundationBlocks", &e.id), ("sections", &member.section_id))]);
            }
        }
        out.extend(d.frame_bars.iter().map(|e| edge(("frameBars", &e.id), ("members", &e.member_id))));
        if let Some(anchor) = d.frame_bars.first() {
            out.extend(d.frame_nodes.iter().map(|n| edge(("frameBars", &anchor.id), ("frameNodes", &n.id))));
            out.extend(d.frame_bars.iter().skip(1).flat_map(|o| [edge(("frameBars", &anchor.id), ("frameBars", &o.id)), edge(("frameBars", &anchor.id), ("members", &o.member_id))]));
            out.extend(d.frame_actions.iter().flat_map(|a| [edge(("frameBars", &anchor.id), ("frameActions", &a.id)), edge(("frameBars", &anchor.id), ("loadCases", &a.load_case_id))]));
        }
        for basis in &d.frame_member_bases {
            let member = ("members", basis.member_id.as_str());
            out.push(edge(member, ("frameMemberBases", &basis.id)));
            out.extend(d.frame_bars.iter().map(|b| edge(member, ("frameBars", &b.id))));
            out.extend(d.frame_nodes.iter().map(|n| edge(member, ("frameNodes", &n.id))));
            out.extend(d.frame_actions.iter().flat_map(|a| [edge(member, ("frameActions", &a.id)), edge(member, ("loadCases", &a.load_case_id))]));
        }
        out.extend(d.storeys.iter().flat_map(|st| d.frame_nodes.iter().map(move |n| edge(("storeys", &st.id), ("frameNodes", &n.id))).chain(d.frame_bars.iter().map(move |b| edge(("storeys", &st.id), ("frameBars", &b.id))))));
        out.extend(d.piles.iter().flat_map(|e| [edge(("piles", &e.id), ("sections", &e.section_id)), edge(("piles", &e.id), ("materials", &e.material_id))]));
        for e in &d.fatigue_stresses {
            out.push(edge(("fatigueStresses", &e.id), ("fatigueDetails", &e.detail_id)));
            if let Some(member) = d.fatigue_details.iter().find(|x| x.id == e.detail_id).and_then(|x| d.members.iter().find(|m| m.id == x.member_id)) {
                out.extend([edge(("fatigueStresses", &e.id), ("members", &member.id)), edge(("fatigueStresses", &e.id), ("sections", &member.section_id))]);
            }
        }
        out.extend(d.buckling_lengths.iter().map(|e| edge(("bucklingLengths", &e.id), ("members", &e.member_id))));
        out.extend(d.forming_gains.iter().map(|e| edge(("formingGains", &e.id), ("coldFormedMembers", &e.member_id))));
        out.extend(d.fastened_joints.iter().map(|e| edge(("fastenedJoints", &e.id), ("coldFormedMembers", &e.member_id))));
        for e in &d.material_ductility {
            out.push(edge(("materials", &e.material_id), ("materialDuctility", &e.id)));
            out.extend(d.members.iter().filter(|m| m.material_id == e.material_id).map(|m| edge(("members", &m.id), ("materialDuctility", &e.id))));
        }
        out
    }
}
