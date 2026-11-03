use crate::projection::*;
use crate::tessellation::Lod;
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MockMember {
    pub id: String,
    pub x: f64,
    pub length: f64,
    pub depth: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockLoad {
    pub id: String,
    pub member: String,
    pub kn: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MockDoc {
    pub members: Vec<MockMember>,
    pub loads: Vec<MockLoad>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MockProjection {
    pub chained: bool,
}

const KINDS: [EntityKindSpec; 2] = [
    EntityKindSpec { id: "member", kind: EntityKind::Member, label_en: "Member", label_de: "Bauteil", icon: "beam", collection: "members", pick: PickPolicy::Whole },
    EntityKindSpec { id: "load", kind: EntityKind::Load, label_en: "Load", label_de: "Last", icon: "arrow-down", collection: "loads", pick: PickPolicy::Whole },
];

pub fn member_ref(id: &str) -> EntityRef {
    EntityRef::new(EntityKind::Member, id, format!("members[id={id}]"))
}

pub fn load_ref(id: &str) -> EntityRef {
    EntityRef::new(EntityKind::Load, id, format!("loads[id={id}]"))
}

pub fn member(id: &str, length: f64) -> MockMember {
    MockMember { id: id.into(), x: 0.0, length, depth: 0.3 }
}

pub fn doc(members: usize) -> MockDoc {
    let mut d = MockDoc::default();
    for i in 0..members {
        d.members.push(MockMember { x: i as f64 * 5.0, ..member(&format!("m{i}"), 4.0 + i as f64 * 0.1) });
        d.loads.push(MockLoad { id: format!("l{i}"), member: format!("m{i}"), kn: 10.0 + i as f64 });
    }
    d
}

impl MockProjection {
    pub fn x_of(&self, d: &MockDoc, index: usize) -> f64 {
        if self.chained { d.members[..index].iter().map(|m| m.length + 1.0).sum() } else { d.members[index].x }
    }

    fn member_geometry(&self, d: &MockDoc, index: usize) -> MemberGeometry {
        let m = &d.members[index];
        let x = self.x_of(d, index);
        MemberGeometry {
            axis: Path3::Polyline(Polyline3 { points: vec![Vec3::new(x, 0.0, 0.0), Vec3::new(x + m.length, 0.0, 0.0)] }),
            frame: Frame::WORLD,
            section: SectionShape::parametric(ProfileKind::IShape, &[("h", m.depth), ("b", 0.1), ("tw", 0.006), ("tf", 0.01)]),
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Rigid,
            stations: Vec::new(),
        }
    }
}

impl StructuralProjection for MockProjection {
    type Document<'a> = MockDoc;

    fn schema() -> &'static str {
        "semio.norm.mock/v1"
    }

    fn entity_kinds() -> &'static [EntityKindSpec] {
        &KINDS
    }

    fn entities(&self, d: &MockDoc) -> Vec<EntityRef> {
        d.members.iter().map(|m| member_ref(&m.id)).chain(d.loads.iter().map(|l| load_ref(&l.id))).collect()
    }

    fn fingerprints(&self, d: &MockDoc, entities: &[EntityRef]) -> Vec<String> {
        entities
            .iter()
            .map(|e| match e.kind {
                EntityKind::Member => d.members.iter().position(|m| m.id == e.id).map(|i| format!("{:?}|{}", d.members[i], self.x_of(d, i))).unwrap_or_default(),
                _ => d.loads.iter().find(|l| l.id == e.id).map(|l| format!("{l:?}")).unwrap_or_default(),
            })
            .collect()
    }

    fn entity_dependencies(&self, d: &MockDoc, e: &EntityRef) -> Vec<String> {
        match e.kind {
            EntityKind::Member if self.chained => d.members.iter().position(|m| m.id == e.id).filter(|i| *i > 0).map(|i| vec![d.members[i - 1].id.clone()]).unwrap_or_default(),
            EntityKind::Load => d.loads.iter().find(|l| l.id == e.id).map(|l| vec![l.member.clone()]).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn project_entity(&self, d: &MockDoc, e: &EntityRef, ev: &EntityEvaluation, _rows: &norm::CheckReport, _ctx: &ProjectionContext, out: &mut SceneBuilder) {
        match e.kind {
            EntityKind::Member => {
                let Some(i) = d.members.iter().position(|m| m.id == e.id) else { return };
                let g = self.member_geometry(d, i);
                let (x, len) = (self.x_of(d, i), d.members[i].length);
                out.member(e, g).named(&format!("Member {}", e.id), &format!("Bauteil {}", e.id)).detail(&format!("I-shape, {len:.2} m"), &format!("I-Profil, {len:.2} m")).role(MaterialRole::Steel).handle(&format!("{}/length", e.id)).utilise(ev).add();
                out.dimension(&format!("dim-{}-length", e.id), Vec3::new(x, 0.0, 0.0), Vec3::new(x + len, 0.0, 0.0), Vec3::new(0.0, 0.0, -0.5), Quantity::new(QuantityKind::Length, len), None);
            }
            _ => {
                let Some(l) = d.loads.iter().find(|l| l.id == e.id) else { return };
                let Some(i) = d.members.iter().position(|m| m.id == l.member) else { return };
                let mid = self.x_of(d, i) + d.members[i].length / 2.0;
                let case = LoadCaseRef { id: "g".into(), kind: LoadKind::Permanent, psi: None };
                let glyph = LoadGlyph::Point { at: Vec3::new(mid, 0.0, d.members[i].depth / 2.0), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: Quantity::new(QuantityKind::Force, l.kn * 1e3), case };
                out.load(e, glyph).named(&format!("Load {}", e.id), &format!("Last {}", e.id)).detail(&format!("{} kN", l.kn), &format!("{} kN", l.kn)).role(MaterialRole::Load { kind: LoadKind::Permanent }).add();
            }
        }
    }

    fn project_global(&self, _d: &MockDoc, _eval: &Evaluation, _ctx: &ProjectionContext, out: &mut SceneBuilder) {
        let ring = |pts: [(f64, f64); 4]| Ring3 { points: pts.iter().map(|&(x, y)| Vec3::new(x, y, -1.0)).collect() };
        let ground = EntityRef::new(EntityKind::Surface, "ground", "ground");
        out.surface(&ground, SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: Polygon3 { outer: ring([(-1.0, -5.0), (100.0, -5.0), (100.0, 5.0), (-1.0, 5.0)]), holes: vec![] } } })
            .named("Ground", "Baugrund").detail("Reference plane", "Bezugsebene").role(MaterialRole::Soil { class: "ground".into() }).pick(PickPolicy::None).add();
        out.legend(Legend { id: "util".into(), title: LocalizedCopy::new("Utilisation", "Ausnutzung"), scale: LegendScale::Utilisation { map: ColourMapRef { id: "default".into(), min: 0.0, max: 1.0 } } });
    }

    fn diagrams(&self, _d: &MockDoc, _eval: &Evaluation, _focus: &Focus, _ctx: &ProjectionContext) -> Vec<Diagram2d> {
        Vec::new()
    }

    fn handles(&self, _d: &MockDoc, _selection: &[EntityRef], _ctx: &ProjectionContext) -> Vec<Handle> {
        Vec::new()
    }
}

pub fn eval(utils: &[(&str, f64)]) -> Evaluation {
    let mut e = Evaluation { report_seq: 1, ..Evaluation::default() };
    for (id, u) in utils {
        let status = if *u > 1.0 { CheckStatus::Fail } else { CheckStatus::Pass };
        e.entities.insert((*id).into(), EntityEvaluation { checks: vec![CheckSample { check_id: format!("{id}.c1"), utilisation: *u, status }], stale: false, stations: None });
    }
    e
}

pub fn ctx(seq: u64) -> ProjectionContext {
    ProjectionContext::new(seq, Lod::Lod1)
}
