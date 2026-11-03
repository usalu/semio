use super::*;
use crate::projection_kit::{entity_ref, loc};
use crate::tessellation::Lod;
use crate::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct Row {
    id: String,
    length: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct Link {
    id: String,
    row_id: String,
    weight: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct Snap {
    title: String,
    rows: Vec<Row>,
    links: Vec<Link>,
}

#[derive(Default)]
struct Tiny;

fn row_ref(id: &str) -> EntityRef {
    entity_ref(EntityKind::Member, id, "rows")
}

impl SceneKit for Tiny {
    type Snapshot = Snap;

    fn entities(snapshot: &Snap) -> Vec<EntityRef> {
        snapshot.rows.iter().map(|r| row_ref(&r.id)).collect()
    }

    fn dependencies(snapshot: &Snap, id: &str) -> Vec<String> {
        snapshot.links.iter().filter(|l| l.row_id == id).map(|l| l.id.clone()).collect()
    }

    fn fragment(doc: &KitDoc<'_, Snap>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        let mut out = SceneBuilder::new();
        if matches!(part, Part::All | Part::Global) {
            out.owned_by(None);
            out.label("title", Vec3::ZERO, loc(&doc.snapshot.title, &doc.snapshot.title), Tone::Neutral);
        }
        for row in doc.snapshot.rows.iter().filter(|r| part.wants(&r.id)) {
            let entity = row_ref(&row.id);
            out.owned_by(Some(entity.clone()));
            let weight: f64 = doc.snapshot.links.iter().filter(|l| l.row_id == row.id).map(|l| l.weight).sum();
            let reported = rows.checks.iter().filter(|c| c.subject.entity_id == row.id).count();
            let end = Vec3::new(row.length + weight, 0.0, 0.0);
            out.member(&entity, MemberGeometry { axis: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, end] }), frame: Frame::WORLD, section: SectionShape::parametric(ProfileKind::Rect, &[("h", 0.2), ("b", 0.1)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Rigid, stations: Vec::new() })
                .named(&format!("Row {} ({reported})", row.id), &format!("Zeile {} ({reported})", row.id))
                .add();
            out.label(&format!("{}:label", row.id), Vec3::ZERO, loc(&row.id, &row.id), Tone::Info);
        }
        out.into_fragment()
    }

    fn finish(doc: &KitDoc<'_, Snap>, summary: &CheckReport, scene: &mut StructuralScene) {
        scene.description = loc(&format!("{} rows, worst {:.2}", doc.snapshot.rows.len(), summary.summary.worst_utilization), "");
    }
}

fn snap(n: usize) -> Snap {
    Snap { title: "t".into(), rows: (0..n).map(|i| Row { id: format!("r{i}"), length: 1.0 + i as f64 }).collect(), links: Vec::new() }
}

fn whole(snapshot: &Snap) -> StructuralScene {
    project_whole::<Tiny>(&KitDoc { snapshot, placements: &[], view: ViewOptions::none() }, &CheckReport::default(), SceneRevision { document_seq: 1, report_seq: 1 })
}

fn live(projector: &mut SceneProjector<Kit<Tiny>>, snapshot: &Snap) -> StructuralScene {
    projector.project(&KitDoc { snapshot, placements: &[], view: ViewOptions::none() }, &Evaluation::default(), &ProjectionContext::new(1, Lod::Lod1)).expect("scene")
}

#[test]
fn the_full_and_the_fragment_assembled_scene_are_equal() {
    let s = snap(4);
    let mut projector = SceneProjector::new(Kit::<Tiny>::default());
    let mut incremental = live(&mut projector, &s);
    incremental.revision = SceneRevision { document_seq: 1, report_seq: 1 };
    assert_eq!(incremental, whole(&s));
    assert_eq!(incremental.owners.len(), incremental.annotations.len() - 1);
}

#[test]
fn an_edit_rebuilds_the_edited_row_and_the_rows_linked_to_it_only() {
    let mut s = snap(5);
    let mut projector = SceneProjector::new(Kit::<Tiny>::default());
    live(&mut projector, &s);
    projector.cache_mut().reset_counters();
    s.links.push(Link { id: "k0".into(), row_id: "r2".into(), weight: 0.5 });
    let mut after = live(&mut projector, &s);
    assert_eq!(projector.cache().misses(), 2, "the linked row and the global fragment");
    after.revision = SceneRevision { document_seq: 1, report_seq: 1 };
    assert_eq!(after, whole(&s));
}

#[test]
fn the_canonical_order_follows_the_document_order_not_the_emission_order() {
    let s = snap(3);
    let mut scene = whole(&s);
    scene.entities.reverse();
    scene.annotations.reverse();
    canonical_order(&mut scene, &Tiny::entities(&s));
    assert_eq!(scene.entities.iter().map(|e| e.entity.id.as_str()).collect::<Vec<_>>(), ["r0", "r1", "r2"]);
    assert_eq!(scene.annotations.first().map(Annotation::id), Some("title"));
}

#[test]
fn typed_view_options_read_back_in_their_own_type_and_stamp_the_fingerprint() {
    let view = ViewOptions::default().with(ViewKey::Combination, ViewValue::Id("c1".into())).with(ViewKey::TrafficGroup, ViewValue::Id("GR2".into())).with(ViewKey::TrafficGroupItem, ViewValue::Index(1)).with(ViewKey::Track, ViewValue::Id("fire".into())).with(ViewKey::TrackValue, ViewValue::Number(1800.0));
    assert_eq!((view.combination(), view.traffic_group(), view.traffic_group_item(), view.combination_item(), view.track()), (Some("c1"), Some("GR2"), Some(1), None, Some("fire")));
    assert_eq!((view.track_value("fire"), view.track_value("construction")), (Some(1800.0), None));
    assert_eq!(ViewOptions::none().stamp_text(), "");
    assert_ne!(view.stamp_text(), ViewOptions::default().with(ViewKey::TrafficGroup, ViewValue::Id("GR1".into())).stamp_text());
    let s = snap(3);
    let kit = Kit::<Tiny>::default();
    let entities = kit.entities(&KitDoc { snapshot: &s, placements: &[], view: ViewOptions::none() });
    let plain = kit.fingerprints(&KitDoc { snapshot: &s, placements: &[], view: ViewOptions::none() }, &entities);
    let with_group = kit.fingerprints(&KitDoc { snapshot: &s, placements: &[], view: &view }, &entities);
    assert!(plain.iter().zip(&with_group).all(|(a, b)| a != b), "a view option change is part of every entity fingerprint");
}
