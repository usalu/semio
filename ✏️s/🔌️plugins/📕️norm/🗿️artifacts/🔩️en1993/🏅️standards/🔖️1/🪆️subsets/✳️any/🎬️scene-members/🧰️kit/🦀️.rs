//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use super::scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, add, mul, unit, along, member_frame, entity_ref, clause_of, tone_of, utilisation_words, polyline, envelope, binding_of_subjects as binding};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub const LAYER_MEMBERS: &str = "members";
pub const LAYER_JOINTS: &str = "joints";
pub const LAYER_PLATES: &str = "plates";
pub const LAYER_SHELLS: &str = "shells";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_SUPPORTS: &str = "supports";
pub const LAYER_ANNOTATIONS: &str = "annotations";
pub const LAYER_TORSION: &str = "torsion";
pub const LAYER_FATIGUE: &str = "fatigue";
pub const LAYER_GRADES: &str = "grades";
pub const LAYER_CASE_PREFIX: &str = "case:";
pub const LAYER_FRAME_ENVELOPE: &str = "frame-envelope";
pub const LAYER_FRAME_IMPERFECTION: &str = "frame-imperfection";

const LAYERS: [(&str, &str, &str); 15] = [
    (LAYER_MEMBERS, "Members", "Bauteile"),
    (LAYER_JOINTS, "Joints", "Anschlüsse"),
    (LAYER_PLATES, "Plates and panels", "Bleche und Felder"),
    (LAYER_SHELLS, "Shells and piles", "Schalen und Pfähle"),
    (LAYER_LOADS, "Loads", "Lasten"),
    (LAYER_SUPPORTS, "Supports", "Lager"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
    (LAYER_TORSION, "Warping torsion", "Wölbkrafttorsion"),
    (LAYER_FATIGUE, "Fatigue", "Ermüdung"),
    (LAYER_GRADES, "Steel grades", "Stahlgüten"),
    (LAYER_FRAME_ENVELOPE, "Frame envelope", "Rahmeneinhüllende"),
    (LAYER_FRAME_IMPERFECTION, "Frame imperfection forces", "Rahmen-Imperfektionskräfte"),
    (super::tracks::LAYER_MODE_FLEXURAL, "Flexural buckling mode", "Biegeknickform"),
    (super::tracks::LAYER_MODE_LTB, "Lateral-torsional buckling mode", "Biegedrillknickform"),
    (super::tracks::LAYER_MODE_FRAME, "Frame buckling mode", "Rahmenknickform"),
];

/// 🏷️ One entity kind of the EN 1993 outliner, interaction domain and insert palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityKindSpec {
    pub id: &'static str,
    pub collection: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub icon: &'static str,
    pub scene_kind: EntityKind,
}

const fn kind(id: &'static str, collection: &'static str, label_en: &'static str, label_de: &'static str, icon: &'static str, scene_kind: EntityKind) -> EntityKindSpec {
    EntityKindSpec { id, collection, label_en, label_de, icon, scene_kind }
}

/// 🏷️ The single registry of EN 1993 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("member", "members", "Member", "Bauteil", "minus", EntityKind::Member),
    kind("section", "sections", "Section", "Querschnitt", "ruler", EntityKind::Member),
    kind("memberAction", "memberActions", "Member action", "Stabeinwirkung", "weight", EntityKind::Load),
    kind("joint", "joints", "Joint", "Anschluss", "link", EntityKind::Group),
    kind("platedPanel", "platedPanels", "Plated panel", "Beulfeld", "square", EntityKind::Plate),
    kind("siloShell", "siloShells", "Silo shell", "Silo-Schale", "cylinder", EntityKind::Shell),
    kind("tensionComponent", "tensionComponents", "Tension component", "Zugglied", "activity", EntityKind::Member),
    kind("coldFormedMember", "coldFormedMembers", "Cold-formed member", "Kaltprofil", "layers", EntityKind::Member),
    kind("pile", "piles", "Pile", "Pfahl", "arrow-down", EntityKind::Member),
    kind("towerLeg", "towerLegs", "Tower leg", "Turmstab", "triangle", EntityKind::Load),
    kind("craneRunway", "craneRunways", "Crane runway", "Kranbahn", "truck", EntityKind::Load),
    kind("foundationBlock", "foundationBlocks", "Foundation block", "Fundamentblock", "layers", EntityKind::Group),
    kind("jointPlate", "jointPlates", "Joint plate", "Anschlussblech", "plate", EntityKind::Group),
    kind("frameBar", "frameBars", "Frame bar", "Rahmenstab", "minus", EntityKind::Member),
    kind("frameNode", "frameNodes", "Frame node", "Rahmenknoten", "anchor", EntityKind::Group),
    kind("jointRow", "jointRows", "Joint bolt row", "Anschluss-Schraubenreihe", "bolt", EntityKind::Group),
    kind("hollowJoint", "hollowJoints", "Hollow section joint", "Hohlprofilknoten", "link", EntityKind::Group),
    kind("parametricSection", "parametricSections", "Parametric section", "Parametrischer Querschnitt", "ruler", EntityKind::Group),
    kind("weldGroup", "weldGroups", "Weld group", "Schweißnahtgruppe", "link", EntityKind::Group),
    kind("builtUpMember", "builtUpMembers", "Built-up member", "Mehrteiliger Stab", "layers", EntityKind::Group),
    kind("restraint", "restraints", "Restraint", "Halterung", "anchor", EntityKind::Group),
    kind("storey", "storeys", "Storey", "Geschoss", "layers", EntityKind::Group),
    kind("fatigueDetail", "fatigueDetails", "Fatigue detail", "Ermüdungsdetail", "repeat", EntityKind::Group),
    kind("fireExposure", "fireExposures", "Fire exposure", "Brandbeanspruchung", "flame", EntityKind::Load),
    kind("bridgeFatigue", "bridgeFatigue", "Bridge fatigue", "Brückenermüdung", "repeat", EntityKind::Group),
    kind("material", "materials", "Material", "Werkstoff", "palette", EntityKind::Plate),
    kind("loadCase", "loadCases", "Load case", "Lastfall", "weight", EntityKind::Plate),
    kind("frameAction", "frameActions", "Frame action", "Rahmeneinwirkung", "weight", EntityKind::Load),
    kind("frameMemberBasis", "frameMemberBases", "Member force basis", "Kraftgrundlage des Bauteils", "activity", EntityKind::Group),
    kind("fatigueStress", "fatigueStresses", "Stress range", "Spannungsschwingbreite", "repeat", EntityKind::Group),
    kind("bucklingLength", "bucklingLengths", "Buckling length", "Knicklänge", "ruler", EntityKind::Group),
    kind("formingGain", "formingGains", "Forming gain", "Kaltverfestigung", "layers", EntityKind::Group),
    kind("fastenedJoint", "fastenedJoints", "Fastened joint", "Befestigungsverbindung", "link", EntityKind::Group),
    kind("materialDuctility", "materialDuctility", "Material ductility", "Werkstoffduktilität", "palette", EntityKind::Group),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1993Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let d = document;
    let found: [(&str, bool); 35] = [
        ("members", d.members.iter().any(|e| e.id == base)),
        ("sections", d.sections.iter().any(|e| e.id == base)),
        ("memberActions", d.member_actions.iter().any(|e| e.id == base)),
        ("joints", d.joints.iter().any(|e| e.id == base)),
        ("platedPanels", d.plated_panels.iter().any(|e| e.id == base)),
        ("siloShells", d.silo_shells.iter().any(|e| e.id == base)),
        ("tensionComponents", d.tension_components.iter().any(|e| e.id == base)),
        ("coldFormedMembers", d.cold_formed_members.iter().any(|e| e.id == base)),
        ("piles", d.piles.iter().any(|e| e.id == base)),
        ("towerLegs", d.tower_legs.iter().any(|e| e.id == base)),
        ("craneRunways", d.crane_runways.iter().any(|e| e.id == base)),
        ("storeys", d.storeys.iter().any(|e| e.id == base)),
        ("restraints", d.restraints.iter().any(|e| e.id == base)),
        ("builtUpMembers", d.built_up_members.iter().any(|e| e.id == base)),
        ("weldGroups", d.weld_groups.iter().any(|e| e.id == base)),
        ("parametricSections", d.parametric_sections.iter().any(|e| e.id == base)),
        ("hollowJoints", d.hollow_joints.iter().any(|e| e.id == base)),
        ("jointRows", d.joint_rows.iter().any(|e| e.id == base)),
        ("frameNodes", d.frame_nodes.iter().any(|e| e.id == base)),
        ("frameBars", d.frame_bars.iter().any(|e| e.id == base)),
        ("jointPlates", d.joint_plates.iter().any(|e| e.id == base)),
        ("foundationBlocks", d.foundation_blocks.iter().any(|e| e.id == base)),
        ("fatigueDetails", d.fatigue_details.iter().any(|e| e.id == base)),
        ("fireExposures", d.fire_exposures.iter().any(|e| e.id == base)),
        ("bridgeFatigue", d.bridge_fatigue.iter().any(|e| e.id == base)),
        ("materials", d.materials.iter().any(|e| e.id == base)),
        ("loadCases", d.load_cases.iter().any(|e| e.id == base)),
        ("frameActions", d.frame_actions.iter().any(|e| e.id == base)),
        ("frameMemberBases", d.frame_member_bases.iter().any(|e| e.id == base)),
        ("placements", d.placements.iter().any(|e| e.id == base)),
        ("fatigueStresses", d.fatigue_stresses.iter().any(|e| e.id == base)),
        ("bucklingLengths", d.buckling_lengths.iter().any(|e| e.id == base)),
        ("formingGains", d.forming_gains.iter().any(|e| e.id == base)),
        ("fastenedJoints", d.fastened_joints.iter().any(|e| e.id == base)),
        ("materialDuctility", d.material_ductility.iter().any(|e| e.id == base)),
    ];
    found.iter().find(|(_, hit)| *hit).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1993Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let d = document;
    let mut out: BTreeSet<String> = BTreeSet::new();
    if let Some(m) = d.members.iter().find(|e| e.id == base) {
        out.insert(m.section_id.clone());
        out.insert(m.material_id.clone());
        out.extend(d.member_actions.iter().filter(|a| a.member_id == base).flat_map(|a| [a.id.clone(), a.load_case_id.clone()]));
        out.extend(d.joints.iter().filter(|j| j.member_id == base).map(|j| j.id.clone()));
        out.extend(dependents(d, base).into_iter().map(str::to_owned));
    }
    out.extend(d.joints.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    for action in d.member_actions.iter().filter(|e| e.id == base) {
        out.extend([action.member_id.clone(), action.load_case_id.clone()]);
        out.extend(d.members.iter().filter(|m| m.id == action.member_id).map(|m| m.section_id.clone()));
    }
    out.extend(d.fatigue_details.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.fire_exposures.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.bridge_fatigue.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.tower_legs.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.crane_runways.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.storeys.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.frame_actions.iter().filter(|e| e.id == base).flat_map(|e| [e.target_id.clone(), e.load_case_id.clone()]));
    out.extend(d.frame_member_bases.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    for stress in d.fatigue_stresses.iter().filter(|e| e.id == base) {
        out.insert(stress.detail_id.clone());
        for detail in d.fatigue_details.iter().filter(|x| x.id == stress.detail_id) {
            out.insert(detail.member_id.clone());
            out.extend(d.members.iter().filter(|m| m.id == detail.member_id).map(|m| m.section_id.clone()));
            out.extend(d.member_actions.iter().filter(|a| a.member_id == detail.member_id).flat_map(|a| [a.id.clone(), a.load_case_id.clone()]));
        }
    }
    out.extend(d.buckling_lengths.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.forming_gains.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.fastened_joints.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.material_ductility.iter().filter(|e| e.id == base).map(|e| e.material_id.clone()));
    if d.frame_bars.iter().any(|e| e.id == base) || d.frame_nodes.iter().any(|e| e.id == base) {
        out.extend(d.frame_actions.iter().map(|e| e.id.clone()));
    }
    out.extend(d.restraints.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.built_up_members.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.weld_groups.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.hollow_joints.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.joint_rows.iter().filter(|e| e.id == base).map(|e| e.joint_id.clone()));
    out.extend(d.joint_rows.iter().filter(|e| e.joint_id == base).map(|e| e.id.clone()));
    out.extend(d.joint_plates.iter().filter(|e| e.id == base).map(|e| e.joint_id.clone()));
    out.extend(d.joint_plates.iter().filter(|e| e.joint_id == base).map(|e| e.id.clone()));
    out.extend(d.foundation_blocks.iter().filter(|e| e.id == base).map(|e| e.joint_id.clone()));
    out.extend(d.foundation_blocks.iter().filter(|e| e.joint_id == base).map(|e| e.id.clone()));
    if d.materials.iter().any(|e| e.id == base) {
        out.extend(d.members.iter().filter(|m| m.material_id == base || m.web_material_id == base).map(|m| m.id.clone()));
    }
    if d.load_cases.iter().any(|e| e.id == base) {
        out.extend(d.member_actions.iter().filter(|a| a.load_case_id == base).flat_map(|a| [a.id.clone(), a.member_id.clone()]));
    }
    if d.frame_bars.iter().any(|e| e.id == base) || d.frame_nodes.iter().any(|e| e.id == base) {
        out.extend(d.frame_bars.iter().flat_map(|bar| [bar.id.clone(), bar.member_id.clone(), bar.start_node_id.clone(), bar.end_node_id.clone()]));
        out.extend(d.frame_nodes.iter().map(|n| n.id.clone()));
        for member in d.frame_bars.iter().filter_map(|bar| d.members.iter().find(|m| m.id == bar.member_id)) {
            out.extend([member.section_id.clone(), member.material_id.clone()]);
        }
    }
    for pile in d.piles.iter().filter(|e| e.id == base) {
        out.extend([pile.section_id.clone(), pile.material_id.clone()]);
    }
    out.remove(base);
    out.into_iter().collect()
}

/// 🔗️ The member id plus every special-element id that verifies the same member (fatigue, fire, bridge, tower, crane): the subjects whose checks colour the member.
pub fn dependents<'a>(d: &'a model::En1993Snapshot, member: &'a str) -> Vec<&'a str> {
    std::iter::once(member)
        .chain(d.fatigue_details.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.fire_exposures.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.bridge_fatigue.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.tower_legs.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.crane_runways.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.storeys.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.restraints.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.built_up_members.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.weld_groups.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.hollow_joints.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.frame_bars.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.members.iter().filter(|m| m.id == member).flat_map(|m| d.materials.iter().filter(move |x| x.id == m.material_id || x.id == m.web_material_id)).map(|e| e.id.as_str()))
        .collect()
}

/// 🧱️ The section outline source: I and H sections as `IShape`, rectangular hollow sections as `Rhs`, circular hollow sections as `Chs` (D = h, wall t), angles as `Angle` (legs h and b, thickness t, sharp corner), anything with unusable wall data as its bounding `Rect`.
pub fn section_shape(s: &model::SteelSection) -> SectionShape {
    let (h, b) = (s.h.max(1e-3), s.b.max(1e-3));
    let t = s.tw.max(s.tf);
    let bounding = || SectionShape::parametric(ProfileKind::Rect, &[("h", h), ("b", b)]);
    match s.kind {
        semio_s_artifact_norm_en1993_model::SectionKind::Hss if s.tw > 0.0 && 2.0 * s.tw < h.min(b) => SectionShape::parametric(ProfileKind::Rhs, &[("h", h), ("b", b), ("t", s.tw)]),
        semio_s_artifact_norm_en1993_model::SectionKind::Chs if t > 0.0 && 2.0 * t < h => SectionShape::parametric(ProfileKind::Chs, &[("d", h), ("t", t)]),
        semio_s_artifact_norm_en1993_model::SectionKind::Angle if t > 0.0 && t < h.min(b) => SectionShape::parametric(ProfileKind::Angle, &[("h", h), ("b", b), ("t", t)]),
        semio_s_artifact_norm_en1993_model::SectionKind::Hss | semio_s_artifact_norm_en1993_model::SectionKind::Chs | semio_s_artifact_norm_en1993_model::SectionKind::Angle => bounding(),
        semio_s_artifact_norm_en1993_model::SectionKind::RolledI | semio_s_artifact_norm_en1993_model::SectionKind::WeldedI => {
            if s.tw > 0.0 && s.tf > 0.0 && 2.0 * s.tf < h && s.tw < b {
                SectionShape::parametric(ProfileKind::IShape, &[("h", h), ("b", b), ("tw", s.tw), ("tf", s.tf)])
            } else {
                bounding()
            }
        }
    }
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub collection: &'static str,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub deformation: Option<DeformationOverlay>,
}

impl Spec {
    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, self.collection),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
                style: if failing { StyleState::Error } else { StyleState::Normal },
                opacity: 1.0,
                hatch: failing.then_some(Hatch::Diagonal),
                deformation: self.deformation,
            },
            utilisation: self.binding,
            label: Some(self.name.clone()),
            accessible_name: self.name,
            accessible_detail: LocalizedCopy::new(format!("{}{more_en}", self.detail.en), format!("{}{more_de}", self.detail.de)),
            layer: self.layer.to_string(),
            pick: PickPolicy::Whole,
            handles: self.handles,
            content_hash: String::new(),
        }
        .sealed()
    }
}

/// 📥️ Append-only scene accumulator of one projection part: entities in document order, owned annotations and tracks, legends and the points that bound the part.
#[derive(Default)]
pub struct Out {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    pub tracks: Vec<ParameterTrack>,
    pub legends: Vec<Legend>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    scope: Scope,
    owner: Option<EntityRef>,
    active: bool,
}

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one document entity, or the global rest, at a time.
    pub fn scoped(part: Part<'_>) -> Self {
        let scope = match part {
            Part::All => Scope::All,
            Part::Entity(id) => Scope::Entity(id.to_owned()),
            Part::Global => Scope::Global,
        };
        Self { scope, ..Self::default() }
    }

    fn part(&self) -> Part<'_> {
        match &self.scope {
            Scope::All => Part::All,
            Scope::Entity(id) => Part::Entity(id),
            Scope::Global => Part::Global,
        }
    }

    /// 🎯️ Starts the document entity `id` of `collection`: everything pushed until the next call belongs to it; returns whether this part wants it, so the caller can skip its row.
    pub fn own(&mut self, collection: &str, id: &str) -> bool {
        self.owner = Some(owner_ref(collection, id));
        self.active = self.part().wants(id);
        self.active
    }

    pub fn push(&mut self, spec: Spec, points: &[Vec3]) {
        if !self.active {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(spec.build());
    }

    /// 🎨️ Like [`Out::push`], but lets `tune` change the sealed entity first, e.g. a colour field or an opacity that [`Spec`] does not carry.
    pub fn push_tuned(&mut self, spec: Spec, points: &[Vec3], tune: impl FnOnce(&mut SceneEntity)) {
        if !self.active {
            return;
        }
        self.points.extend_from_slice(points);
        let mut entity = spec.build();
        tune(&mut entity);
        self.entities.push(entity.sealed());
    }

    fn link(&mut self, kind: OwnedKind, id: &str) -> bool {
        if !self.active {
            return false;
        }
        if let Some(owner) = &self.owner {
            self.owners.push(Ownership { kind, id: id.to_owned(), owner: owner.clone() });
        }
        true
    }

    pub fn annotate(&mut self, annotation: Annotation, anchor: &Vec3) {
        if self.link(OwnedKind::Annotation, annotation.id()) {
            self.points.push(anchor.clone());
            self.annotations.push(annotation);
        }
    }

    pub fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: self.legends, layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

/// 🧭️ The reference of a document entity of `collection`, with the scene kind of its registry row.
fn owner_ref(collection: &str, id: &str) -> EntityRef {
    let kind = ENTITY_KINDS.iter().find(|k| k.collection == collection).map_or(EntityKind::Group, |k| k.scene_kind);
    entity_ref(kind, id, collection)
}

/// 🗂️ The ids of every collection of the document, in outliner order; the single enumeration behind [`entities_of`].
fn rows_of(d: &model::En1993Snapshot) -> [(&'static str, Vec<&str>); 34] {
    [
        ("members", d.members.iter().map(|e| e.id.as_str()).collect()),
        ("sections", d.sections.iter().map(|e| e.id.as_str()).collect()),
        ("memberActions", d.member_actions.iter().map(|e| e.id.as_str()).collect()),
        ("joints", d.joints.iter().map(|e| e.id.as_str()).collect()),
        ("platedPanels", d.plated_panels.iter().map(|e| e.id.as_str()).collect()),
        ("siloShells", d.silo_shells.iter().map(|e| e.id.as_str()).collect()),
        ("tensionComponents", d.tension_components.iter().map(|e| e.id.as_str()).collect()),
        ("coldFormedMembers", d.cold_formed_members.iter().map(|e| e.id.as_str()).collect()),
        ("piles", d.piles.iter().map(|e| e.id.as_str()).collect()),
        ("towerLegs", d.tower_legs.iter().map(|e| e.id.as_str()).collect()),
        ("craneRunways", d.crane_runways.iter().map(|e| e.id.as_str()).collect()),
        ("frameBars", d.frame_bars.iter().map(|e| e.id.as_str()).collect()),
        ("jointPlates", d.joint_plates.iter().map(|e| e.id.as_str()).collect()),
        ("foundationBlocks", d.foundation_blocks.iter().map(|e| e.id.as_str()).collect()),
        ("frameNodes", d.frame_nodes.iter().map(|e| e.id.as_str()).collect()),
        ("jointRows", d.joint_rows.iter().map(|e| e.id.as_str()).collect()),
        ("hollowJoints", d.hollow_joints.iter().map(|e| e.id.as_str()).collect()),
        ("parametricSections", d.parametric_sections.iter().map(|e| e.id.as_str()).collect()),
        ("weldGroups", d.weld_groups.iter().map(|e| e.id.as_str()).collect()),
        ("builtUpMembers", d.built_up_members.iter().map(|e| e.id.as_str()).collect()),
        ("restraints", d.restraints.iter().map(|e| e.id.as_str()).collect()),
        ("storeys", d.storeys.iter().map(|e| e.id.as_str()).collect()),
        ("fatigueDetails", d.fatigue_details.iter().map(|e| e.id.as_str()).collect()),
        ("fireExposures", d.fire_exposures.iter().map(|e| e.id.as_str()).collect()),
        ("bridgeFatigue", d.bridge_fatigue.iter().map(|e| e.id.as_str()).collect()),
        ("materials", d.materials.iter().map(|e| e.id.as_str()).collect()),
        ("loadCases", d.load_cases.iter().map(|e| e.id.as_str()).collect()),
        ("frameActions", d.frame_actions.iter().map(|e| e.id.as_str()).collect()),
        ("frameMemberBases", d.frame_member_bases.iter().map(|e| e.id.as_str()).collect()),
        ("fatigueStresses", d.fatigue_stresses.iter().map(|e| e.id.as_str()).collect()),
        ("bucklingLengths", d.buckling_lengths.iter().map(|e| e.id.as_str()).collect()),
        ("formingGains", d.forming_gains.iter().map(|e| e.id.as_str()).collect()),
        ("fastenedJoints", d.fastened_joints.iter().map(|e| e.id.as_str()).collect()),
        ("materialDuctility", d.material_ductility.iter().map(|e| e.id.as_str()).collect()),
    ]
}

/// 🆔️ Every document entity in outliner order: the canonical order of the scene.
pub fn entities_of(document: &model::En1993Snapshot) -> Vec<EntityRef> {
    rows_of(document).iter().flat_map(|(collection, ids)| ids.iter().map(move |id| owner_ref(collection, id))).collect()
}

/// 📍️ What an entity's pose derives from beyond its own row: the default layout slot of a row entity, the member pose and the positions of the member's action rows (their stagger follows the row index).
pub fn layout_stamps(document: &model::En1993Snapshot, layout: &Layout) -> std::collections::BTreeMap<String, String> {
    let members = document.members.iter().map(|m| {
        let indices: Vec<usize> = document.member_actions.iter().enumerate().filter(|(_, a)| a.member_id == m.id).map(|(i, _)| i).collect();
        (m.id.clone(), format!("{:?}|{indices:?}", layout.member(&m.id)))
    });
    let rows = rows_of(document).into_iter().skip(1).flat_map(|(_, ids)| ids).map(|id| (id.to_owned(), format!("{:?}", layout.origin(id))));
    let actions = document.member_actions.iter().enumerate().map(|(index, a)| (a.id.clone(), format!("{:?}|{index}", layout.member(&a.member_id))));
    members.chain(rows).chain(actions).collect()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends, the description and the buckling-mode track.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let cases = case_layers(&scene.entities);
    let table: Vec<(&str, &str, &str)> = LAYERS.iter().copied().chain(cases.iter().map(|(id, en, de)| (id.as_str(), en.as_str(), de.as_str()))).collect();
    scene.layers = projection_kit::legends::layers(&table, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(field_legends(&scene.entities));
    scene.description = description;
    super::tracks::finish(scene);
}

const FIELD_LEGENDS: [(&str, &str, &str, QuantityKind); 2] = [("warping-stress", "Warping normal stress σ_w", "Wölbnormalspannung σ_w", QuantityKind::Stress), ("yield-strength", "Yield strength f_y", "Streckgrenze f_y", QuantityKind::Stress)];

/// 🧾️ The layer of a load case: its member-action glyphs live on `case:<id>`, so the load-case track shows one case at a time.
pub fn case_layer(case: &str) -> String {
    format!("{LAYER_CASE_PREFIX}{case}")
}

/// 🧾️ The `(layer id, English, German)` rows of the load-case layers in use, named after the card entity of the case.
fn case_layers(entities: &[SceneEntity]) -> Vec<(String, String, String)> {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    for layer in entities.iter().map(|e| e.layer.as_str()).filter(|l| l.starts_with(LAYER_CASE_PREFIX)) {
        if rows.iter().any(|(id, ..)| id == layer) {
            continue;
        }
        let id = &layer[LAYER_CASE_PREFIX.len()..];
        let (en, de) = entities.iter().find(|e| e.entity.id == id).and_then(|e| e.label.as_ref()).map_or((id.to_owned(), id.to_owned()), |l| (l.en.clone(), l.de.clone()));
        rows.push((layer.to_owned(), format!("Load case {en}"), format!("Lastfall {de}")));
    }
    rows
}

/// 🎞️ The load-case index track: one stop per load-case layer in use, each revealing its own layer; `None` below two cases.
pub fn case_track(scene: &StructuralScene) -> Option<ParameterTrack> {
    let stops = scene.layers.iter().filter(|l| l.id.starts_with(LAYER_CASE_PREFIX)).map(|l| (l.name.en.clone(), l.name.de.clone(), None, Some(l.id.clone()))).collect();
    projection_kit::tracks::indexed(projection_kit::tracks::COMBINATION_TRACK, TrackKind::Combination, ("Load case", "Lastfall"), stops)
}

/// 🌈️ One scalar legend per colour field the entities carry, its range the extremes of the samples.
fn field_legends(entities: &[SceneEntity]) -> Vec<Legend> {
    FIELD_LEGENDS
        .iter()
        .filter_map(|(id, en, de, unit)| {
            let values: Vec<f64> = entities
                .iter()
                .filter_map(|e| match &e.appearance.channel {
                    ColourChannel::Fields { fields, .. } => Some(fields),
                    _ => None,
                })
                .flatten()
                .filter(|f| f.id == *id)
                .flat_map(|f| match &f.samples {
                    FieldSamples::Station { values } => values.iter().map(|v| v.value).collect::<Vec<_>>(),
                    FieldSamples::Vertex { values } | FieldSamples::Instance { values } => values.clone(),
                })
                .collect();
            let (min, max) = values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
            (min <= max).then(|| Legend { id: (*id).into(), title: loc(en, de), scale: LegendScale::Scalar { field_id: (*id).into(), map: ColourMapRef { id: "viridis".into(), min, max: if max > min { max } else { min + 1.0 } }, unit: unit.clone() } })
        })
        .collect()
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Steel => ("Steel", "Stahl"),
            MaterialRole::Bolt => ("Bolts", "Schrauben"),
            MaterialRole::Weld => ("Welds", "Schweißnähte"),
            MaterialRole::Support => ("Supports", "Lager"),
            MaterialRole::Load { .. } => ("Loads", "Lasten"),
            MaterialRole::Soil { .. } => ("Ground", "Baugrund"),
            _ => return None,
        };
        let key = match role {
            MaterialRole::Load { .. } => MaterialRole::Load { kind: LoadKind::Other },
            other => other.clone(),
        };
        Some((label, key))
    })
}

// BEGIN check-owners (generated by check_owners.py)
/// 🧭️ The collection whose entity draws each check family, from the `entity` of the spec-trace item that lists the family; longer prefixes first.
pub const CHECK_OWNERS: &[(&str, &str)] = &[
    ("en1993.1-8.3.10.plateblock", "jointPlates"),
    ("en1993.1-8.3.6.interaction", "joints"),
    ("en1993.1-8.4.5.directional", "joints"),
    ("en1993.1-1.6.2.3.platenet", "jointPlates"),
    ("en1993.1-8.4.5.simplified", "joints"),
    ("en1993.1-1.5.3.3.bracing", "storeys"),
    ("en1993.1-1.6.3.4.general", "storeys"),
    ("en1993.1-1.6.3.4.general", "restraints"),
    ("en1993.1-1.6.4.4.spacing", "builtUpMembers"),
    ("en1993.1-8.4.5.plateweld", "jointPlates"),
    ("en1993.1-8.3.6.punching", "joints"),
    ("en1993.1-1.6.3.2.mcrfe", "members"),
    ("en1993.1-1.6.3.2.mcrfe", "restraints"),
    ("en1993.1-8.3.6.bearing", "joints"),
    ("en1993.1-8.3.6.tension", "joints"),
    ("en1993.1-8.6.2.anchort", "foundationBlocks"),
    ("en1993.1-8.6.2.anchorv", "foundationBlocks"),
    ("en1993.1-8.6.2.bearing", "foundationBlocks"),
    ("en1993.3.2.2.ductility", "materials"),
    ("en1993.3.2.6.constants", "materials"),
    ("en1993.1-1.5.3.2.sway", "storeys"),
    ("en1993.1-1.5.5.hybrid", "members"),
    ("en1993.1-11.6.2.cable", "tensionComponents"),
    ("en1993.1-8.3.10.block", "joints"),
    ("en1993.1-8.3.10.block", "jointPlates"),
    ("en1993.6.3.2.3.rolled", "members"),
    ("en1993.sections.param", "members"),
    ("en1993.1-1.5.2.alpha", "storeys"),
    ("en1993.1-1.5.2.frame", "frameBars"),
    ("en1993.1-1.6.4.chord", "builtUpMembers"),
    ("en1993.1-2.4.2.3.int", "fireExposures"),
    ("en1993.1-2.4.2.3.ltb", "fireExposures"),
    ("en1993.1-8.3.6.plate", "jointPlates"),
    ("en1993.1-8.3.12.ic", "jointPlates"),
    ("en1993.1-9.5.range", "fatigueStresses"),
    ("en1993.1-1.6.3.1.3.lcr", "bucklingLengths"),
    ("en1993.1-3.3.2.2.fya", "formingGains"),
    ("en1993.1-3.8.3.shear", "fastenedJoints"),
    ("en1993.1-3.8.3.net", "fastenedJoints"),
    ("en1993.1-3.8.3.tension", "fastenedJoints"),
    ("en1993.1-3.8.3.interaction", "fastenedJoints"),
    ("en1993.1-3.8.3.spacing", "fastenedJoints"),
    ("en1993.3.2.2.elongation", "materialDuctility"),
    ("en1993.3.2.2.strain", "materialDuctility"),
    ("en1993.5.6.ductility", "members"),
    ("en1993.1-8.3.6.shear", "joints"),
    ("en1993.1-8.3.6.shear", "jointPlates"),
    ("en1993.1-8.4.11.beff", "joints"),
    ("en1993.1-8.4.5.group", "weldGroups"),
    ("en1993.1-8.6.2.cbend", "foundationBlocks"),
    ("en1993.1-8.6.2.grout", "foundationBlocks"),
    ("en1993.1-8.6.2.tbend", "foundationBlocks"),
    ("en1993.1-1.6.3.5.lm", "members"),
    ("en1993.1-1.6.3.5.lm", "restraints"),
    ("en1993.1-3.5.4.curl", "coldFormedMembers"),
    ("en1993.1-3.5.5.dist", "coldFormedMembers"),
    ("en1993.1-8.3.9.slip", "joints"),
    ("en1993.1-8.6.2.base", "foundationBlocks"),
    ("en1993.1-8.6.2.comp", "joints"),
    ("en1993.1-8.6.2.cone", "foundationBlocks"),
    ("en1993.1-8.6.2.mjrd", "foundationBlocks"),
    ("en1993.3-1.5.lambda", "towerLegs"),
    ("en1993.3-2.ovalling", "towerLegs"),
    ("en1993.1-5.5.panel", "platedPanels"),
    ("en1993.1-5.D.shear", "platedPanels"),
    ("en1993.1-6.8.5.int", "siloShells"),
    ("en1993.1-7.lateral", "platedPanels"),
    ("en1993.1-8.3.6.rod", "tensionComponents"),
    ("en1993.1-8.6.2.row", "joints"),
    ("en1993.1-8.6.2.row", "jointRows"),
    ("en1993.1-8.7.4.gap", "hollowJoints"),
    ("en1993.3.2.1.grade", "materials"),
    ("en1993.5.2.plastic", "members"),
    ("en1993.5.6.plastic", "members"),
    ("en1993.1-3.10.1.5", "coldFormedMembers"),
    ("en1993.1-3.6.1.11", "coldFormedMembers"),
    ("en1993.1-6.8.thin", "siloShells"),
    ("en1993.1-6.D.ring", "siloShells"),
    ("en1993.1-8.5.2.sj", "joints"),
    ("en1993.1-8.6.3.mj", "joints"),
    ("en1993.1-8.6.3.mj", "jointRows"),
    ("en1993.1-9.6.cafl", "fatigueDetails"),
    ("en1993.3-2.vortex", "towerLegs"),
    ("en1993.4-1.5.2.ph", "siloShells"),
    ("en1993.6.3.2.4.cf", "members"),
    ("en1993.7.2.3.freq", "members"),
    ("en1993.1-2.theta", "fireExposures"),
    ("en1993.1-3.6.1.5", "coldFormedMembers"),
    ("en1993.1-3.6.1.7", "coldFormedMembers"),
    ("en1993.1-3.6.1.9", "coldFormedMembers"),
    ("en1993.1-5.5.vbw", "platedPanels"),
    ("en1993.1-5.plate", "platedPanels"),
    ("en1993.1-6.8.5.x", "siloShells"),
    ("en1993.1-6.D.ext", "siloShells"),
    ("en1993.1-6.shell", "siloShells"),
    ("en1993.4-3.5.pcr", "siloShells"),
    ("en1993.5.2.class", "members"),
    ("en1993.5.3.2.bow", "members"),
    ("en1993.6.3.3.int", "members"),
    ("en1993.1-11.6.2", "tensionComponents"),
    ("en1993.1-12.hss", "members"),
    ("en1993.1-2.prot", "fireExposures"),
    ("en1993.1-3.6.1.", "coldFormedMembers"),
    ("en1993.1-4.fire", "fireExposures"),
    ("en1993.1-5.3.sl", "platedPanels"),
    ("en1993.1-8.3.5.", "joints"),
    ("en1993.4-1.silo", "siloShells"),
    ("en1993.4-3.5.vm", "siloShells"),
    ("en1993.5.5.2.il", "piles"),
    ("en1993.6.2.3.nt", "members"),
    ("en1993.6.2.5.my", "members"),
    ("en1993.6.2.6.av", "members"),
    ("en1993.6.2.9.mn", "members"),
    ("en1993.6.3.1.nb", "members"),
    ("en1993.6.3.1.tf", "members"),
    ("en1993.6.3.2.mb", "members"),
    ("en1993.6.5.7.vm", "craneRunways"),
    ("en1993.1-2.nfi", "fireExposures"),
    ("en1993.1-4.5.5", "members"),
    ("en1993.1-4.5.5", "materials"),
    ("en1993.1-5.4.5", "platedPanels"),
    ("en1993.1-5.7.1", "platedPanels"),
    ("en1993.1-5.7.2", "platedPanels"),
    ("en1993.1-5.9.2", "platedPanels"),
    ("en1993.1-5.9.3", "platedPanels"),
    ("en1993.1-9.fat", "fatigueDetails"),
    ("en1993.4-3.5.t", "siloShells"),
    ("en1993.5.5.2.m", "piles"),
    ("en1993.5.5.2.v", "piles"),
    ("en1993.6.2.4.n", "members"),
    ("en1993.6.2.6.v", "members"),
    ("en1993.6.2.7.t", "members"),
    ("en1993.6.2.7.w", "members"),
    ("en1993.6.9.fat", "craneRunways"),
    ("en1993.1-10.t", "members"),
    ("en1993.1-3.cf", "coldFormedMembers"),
    ("en1993.1-4.ss", "members"),
    ("en1993.1-5.10", "platedPanels"),
    ("en1993.1-5.6.", "craneRunways"),
    ("en1993.1-5.9.", "platedPanels"),
    ("en1993.1-5.6", "platedPanels"),
    ("en1993.1-5.8", "platedPanels"),
    ("en1993.2.9.4", "bridgeFatigue"),
    ("en1993.2.9.5", "bridgeFatigue"),
    ("en1993.4-2.6", "siloShells"),
    ("en1993.7.sls", "members"),
];

/// 🧭️ The collection that owns the check `check_id` (longest family prefix), `None` for a check outside the spec trace.
pub fn check_owner(check_id: &str) -> Option<&'static str> {
    CHECK_OWNERS.iter().find(|(prefix, _)| check_id.starts_with(prefix)).map(|(_, collection)| *collection)
}
// END check-owners

/// 🧭️ The document row that owns `check`: the row its subject names, else the only row of a collection [`CHECK_OWNERS`] files the family under; `None` for a document-level check.
pub fn owner_entity(document: &model::En1993Snapshot, check: &norm::CheckResult) -> Option<String> {
    let subject = base_id(&check.subject.entity_id);
    if !subject.is_empty() && entity_kind_of(document, subject).is_some() {
        return Some(subject.to_owned());
    }
    let rows = rows_of(document);
    CHECK_OWNERS.iter().filter(|(prefix, _)| check.id.starts_with(prefix)).find_map(|(_, collection)| {
        let ids = &rows.iter().find(|(name, _)| name == collection)?.1;
        (ids.len() == 1).then(|| ids[0].to_owned())
    })
}

/// 📈️ The check families a dedicated diagram draws (buckling, LTB, interaction, class, toughness, S-N, fire, joint, weld, hollow, silo, panel, crane, tower and pile curves).
pub const DRAWN_FAMILIES: &[&str] = &[
    "en1993.1-1.5.2.alpha", "en1993.1-1.5.2.frame", "en1993.1-1.6.3.2.mcrfe", "en1993.1-10.t", "en1993.1-2.4.2.3.ltb", "en1993.1-2.theta", "en1993.1-5.6", "en1993.1-5.D.shear", "en1993.1-6.D.ext",
    "en1993.1-8.3.10.block", "en1993.1-8.4.5.group", "en1993.1-8.6.2.cbend", "en1993.1-8.6.2.comp", "en1993.1-8.6.2.cone", "en1993.1-8.6.2.row", "en1993.1-8.6.3.mj", "en1993.1-8.7.4.gap", "en1993.1-9.fat",
    "en1993.2.9.5", "en1993.5.2.class", "en1993.5.5.2.m", "en1993.6.2.3.nt", "en1993.6.2.4.n", "en1993.6.2.5.my", "en1993.6.2.9.mn", "en1993.6.3.1.nb", "en1993.6.3.1.tf", "en1993.6.3.2.mb", "en1993.6.5.7.vm", "en1993.7.sls",
    "en1993.1-8.3.12.ic", "en1993.1-9.5.range", "en1993.1-1.6.3.1.3.lcr", "en1993.1-3.3.2.2.fya", "en1993.1-3.8.3.shear", "en1993.3.2.2.strain",
];

/// 📄️ The check families that belong to the document as a whole (empty model, referential integrity); they select the global badge.
pub const DOCUMENT_CHECKS: &[&str] = &["en1993.empty", "en1993.integrity."];

/// 🪜️ The collections whose rows already carry the all-checks `ladder:` diagram; the outcome diagram draws the other rows.
pub const LADDER_COLLECTIONS: &[&str] = &["towerLegs", "tensionComponents", "storeys", "builtUpMembers", "craneRunways", "jointRows", "frameBars", "frameNodes", "jointPlates", "foundationBlocks", "piles"];

/// 📊️ True for a check no dedicated diagram draws: its family (longest prefix of [`CHECK_OWNERS`]) is not in [`DRAWN_FAMILIES`]; the ladder of its row or the outcome diagram draws it.
pub fn is_outcome_check(check_id: &str) -> bool {
    CHECK_OWNERS.iter().find(|(prefix, _)| check_id.starts_with(prefix)).is_some_and(|(prefix, _)| !DRAWN_FAMILIES.contains(prefix))
}

/// 🧭️ Where a check is drawn: on one document row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The route of `check`: its owner row, else the document when it sits in a document-level family; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1993Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    owner_entity(document, check).map(CheckRoute::Entity).or_else(|| DOCUMENT_CHECKS.iter().any(|p| check.id.starts_with(p)).then_some(CheckRoute::Document))
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::En1993Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by the row `id` whose id starts with any of `prefixes` (every owned check when empty).
pub fn binding_of_owner(document: &model::En1993Snapshot, report: &norm::CheckReport, id: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
    projection_kit::binding_where(report, |check| owner_entity(document, check).as_deref() == Some(id) && (prefixes.is_empty() || prefixes.iter().any(|p| check.id.starts_with(p))))
}
