use super::description::{describe, describe_entities};
use super::*;
use crate::diff::{apply, diff};
use crate::mock_projection::*;
use crate::tessellation::Lod;
use crate::*;

fn ids(delta: &SceneDelta) -> BTreeSet<String> {
    delta.entities.iter().map(|p| p.id().to_owned()).chain(delta.annotations.iter().map(|p| p.id().to_owned())).collect()
}

fn set(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn projector(chained: bool) -> SceneProjector<MockProjection> {
    SceneProjector::new(MockProjection { chained })
}

struct Rng(u64);

impl Rng {
    fn next(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

#[test]
fn full_projection_is_a_valid_scene_with_bilingual_names() {
    let d = doc(3);
    let scene = projector(false).project(&d, &eval(&[("m1", 0.64)]), &ctx(1)).unwrap();
    assert!(scene.validate().is_ok());
    assert_eq!(scene.entities.iter().map(|e| e.entity.id.as_str()).collect::<Vec<_>>(), ["ground", "m0", "m1", "m2", "l0", "l1", "l2"]);
    assert!(scene.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty()));
    assert_eq!(scene.annotations.len(), 3);
    assert_eq!(scene.revision, SceneRevision { document_seq: 1, report_seq: 1 });
    let m1 = scene.entities.iter().find(|e| e.entity.id == "m1").unwrap();
    assert_eq!(m1.utilisation.as_ref().unwrap().ratio, 0.64);
    assert_eq!(m1.appearance.channel, ColourChannel::Utilisation);
    assert!(scene.bounds.max.x > 10.0 && scene.bounds.min.z < 0.0);
}

#[test]
fn builder_reports_missing_names_and_duplicate_ids() {
    let mut b = SceneBuilder::new();
    let e = member_ref("m0");
    let geometry = MemberGeometry {
        axis: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::X] }),
        frame: Frame::WORLD,
        section: SectionShape::parametric(ProfileKind::Rect, &[("h", 0.1), ("b", 0.1)]),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: vec![],
    };
    b.member(&e, geometry.clone()).add();
    b.member(&e, geometry.clone()).named("A", "A").detail("a", "a").add();
    let violations = assemble(&[std::sync::Arc::new(b.into_fragment())], SceneRevision { document_seq: 0, report_seq: 0 }, &ctx(0)).unwrap_err();
    assert!(violations.iter().any(|v| v.rule == "non-empty" && v.path.contains("accessibleName")));
    assert!(violations.iter().any(|v| v.rule == "unique-entity-ids"));
}

#[test]
fn worst_check_wins_and_stale_marks_the_style() {
    let ev = EntityEvaluation {
        checks: vec![CheckSample { check_id: "a".into(), utilisation: 0.4, status: CheckStatus::Pass }, CheckSample { check_id: "b".into(), utilisation: 1.2, status: CheckStatus::Fail }, CheckSample { check_id: "c".into(), utilisation: 1.2, status: CheckStatus::Fail }],
        stale: true,
        stations: None,
    };
    assert_eq!(ev.worst().unwrap().check_id, "b");
    let mut b = SceneBuilder::new();
    let geometry = LoadGlyph::Point { at: Vec3::ZERO, direction: Vec3::Z, magnitude: Quantity::new(QuantityKind::Force, 1.0), case: LoadCaseRef { id: "g".into(), kind: LoadKind::Permanent, psi: None } };
    b.load(&load_ref("l"), geometry).named("L", "L").detail("l", "l").utilise(&ev).add();
    let fragment = b.into_fragment();
    let e = &fragment.entities[0];
    assert_eq!(e.appearance.style, StyleState::Stale);
    assert!(e.utilisation.as_ref().unwrap().stale && e.compute_hash() == e.content_hash);
}

#[test]
fn diff_of_identical_scenes_is_empty() {
    let scene = projector(false).project(&doc(4), &eval(&[]), &ctx(1)).unwrap();
    let delta = diff(&scene, &scene);
    assert!(delta.is_empty() && delta.validate().is_ok());
    assert_eq!(apply(&delta, &scene).unwrap(), scene);
}

#[test]
fn editing_one_member_touches_only_its_closure() {
    let mut p = projector(false);
    let mut d = doc(8);
    let e = eval(&[]);
    p.project(&d, &e, &ctx(1)).unwrap();
    p.cache_mut().reset_counters();
    d.members[3].length = 4.5;
    let (scene, delta) = p.project_delta(&d, &ChangedSet::of(["m3"]), &e, &ctx(2)).unwrap();
    assert_eq!(ids(&delta), set(&["m3", "l3", "dim-m3-length"]));
    assert_eq!((p.cache().misses(), p.cache().hits()), (3, 14));
    assert_eq!(scene, projector(false).project(&d, &e, &ctx(2)).unwrap());
}

#[test]
fn evaluation_change_patches_only_the_entity_and_dependents_do_not_change() {
    let mut p = projector(false);
    let d = doc(4);
    p.project(&d, &eval(&[("m1", 0.5)]), &ctx(1)).unwrap();
    let (_, delta) = p.project_delta(&d, &ChangedSet::none(), &eval(&[("m1", 0.9)]), &ctx(1)).unwrap();
    assert_eq!(ids(&delta), set(&["m1"]));
    assert!(matches!(delta.entities[0], EntityPatch::Replace { .. }));
}

#[test]
fn chained_layout_propagates_through_the_dependency_closure() {
    let mut p = projector(true);
    let mut d = doc(5);
    let e = eval(&[]);
    p.project(&d, &e, &ctx(1)).unwrap();
    d.members[1].length += 1.0;
    let (scene, delta) = p.project_delta(&d, &ChangedSet::of(["m1"]), &e, &ctx(2)).unwrap();
    assert_eq!(ids(&delta), set(&["m1", "m2", "m3", "m4", "l1", "l2", "l3", "l4", "dim-m1-length", "dim-m2-length", "dim-m3-length", "dim-m4-length"]));
    assert_eq!(scene, projector(true).project(&d, &e, &ctx(2)).unwrap());
}

#[test]
fn removal_addition_and_context_change() {
    let mut p = projector(false);
    let mut d = doc(3);
    let e = eval(&[]);
    let (first, delta) = p.project_delta(&d, &ChangedSet::all(), &e, &ctx(1)).unwrap();
    assert_eq!(delta.entities.len(), first.entities.len());
    assert_eq!(apply(&delta, &empty_scene()).unwrap(), first);
    d.loads.retain(|l| l.id != "l1");
    d.members.retain(|m| m.id != "m0");
    d.loads.retain(|l| l.member != "m0");
    let (second, delta) = p.project_delta(&d, &ChangedSet::of(["m0", "l1", "l0"]), &e, &ctx(2)).unwrap();
    assert_eq!(ids(&delta), set(&["m0", "l0", "l1", "dim-m0-length"]));
    assert_eq!(apply(&delta, &first).unwrap(), second);
    let finer = ProjectionContext::new(3, Lod::Lod3);
    let (third, delta) = p.project_delta(&d, &ChangedSet::none(), &e, &finer).unwrap();
    assert!(delta.is_empty() && delta.revision.document_seq == 3 && third.revision.document_seq == 3);
    p.cache_mut().reset_counters();
    p.project(&d, &e, &ProjectionContext { deformation_scale: 5.0, ..finer.clone() }).unwrap();
    assert_eq!(p.cache().hits(), 0);
}

#[test]
fn base_mismatch_and_unknown_ids_are_rejected() {
    let a = projector(false).project(&doc(2), &eval(&[]), &ctx(1)).unwrap();
    let b = projector(false).project(&doc(3), &eval(&[]), &ctx(2)).unwrap();
    let delta = diff(&a, &b);
    assert_eq!(apply(&delta, &b), Err(crate::diff::DiffError::BaseMismatch));
    let mut bad = delta.clone();
    bad.entities.push(EntityPatch::Remove { id: "nope".into() });
    assert_eq!(apply(&bad, &a), Err(crate::diff::DiffError::UnknownId("nope".into())));
}

#[test]
fn reordering_is_expressed_as_upserts() {
    let a = projector(false).project(&doc(5), &eval(&[]), &ctx(1)).unwrap();
    let mut b = a.clone();
    b.entities.swap(1, 4);
    b.revision.document_seq = 2;
    let delta = diff(&a, &b);
    assert!(delta.entities.len() <= 2 && delta.validate().is_ok());
    assert_eq!(apply(&delta, &a).unwrap(), b);
}

#[test]
fn random_mutation_sequences_keep_both_laws() {
    for chained in [false, true] {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let mut inc = projector(chained);
        let mut d = doc(5);
        let mut utils: Vec<(String, f64)> = vec![];
        let mut counter = 100;
        let mut prev = inc.project(&d, &eval(&[]), &ctx(0)).unwrap();
        for step in 1..=80u64 {
            let mut changed: Vec<String> = Vec::new();
            match rng.next(7) {
                0 if !d.members.is_empty() => {
                    let i = rng.next(d.members.len());
                    d.members[i].length = 2.0 + rng.next(50) as f64 / 10.0;
                    changed.push(d.members[i].id.clone());
                }
                1 => {
                    counter += 1;
                    let id = format!("m{counter}");
                    let x = d.members.len() as f64 * 6.0;
                    d.members.push(MockMember { x, ..member(&id, 3.0) });
                    changed.push(id);
                }
                2 if !d.members.is_empty() => {
                    let i = rng.next(d.members.len());
                    let gone = d.members.remove(i).id;
                    d.loads.retain(|l| l.member != gone);
                    changed.extend(d.members.iter().map(|m| m.id.clone()));
                    changed.push(gone);
                }
                3 if !d.members.is_empty() => {
                    counter += 1;
                    let target = d.members[rng.next(d.members.len())].id.clone();
                    let id = format!("l{counter}");
                    d.loads.push(MockLoad { id: id.clone(), member: target, kn: 5.0 });
                    changed.push(id);
                }
                4 if !d.loads.is_empty() => {
                    let i = rng.next(d.loads.len());
                    d.loads[i].kn += 1.5;
                    changed.push(d.loads[i].id.clone());
                }
                5 if !d.loads.is_empty() => {
                    let gone = d.loads.remove(rng.next(d.loads.len())).id;
                    changed.push(gone);
                }
                _ if !d.members.is_empty() => {
                    let id = d.members[rng.next(d.members.len())].id.clone();
                    utils.retain(|(k, _)| *k != id);
                    utils.push((id, rng.next(150) as f64 / 100.0));
                }
                _ => {}
            }
            let pairs: Vec<(&str, f64)> = utils.iter().map(|(k, v)| (k.as_str(), *v)).collect();
            let e = eval(&pairs);
            let (scene, delta) = inc.project_delta(&d, &ChangedSet::of(changed.clone()), &e, &ctx(step)).unwrap();
            assert_eq!(scene, projector(chained).project(&d, &e, &ctx(step)).unwrap(), "incremental equals full at step {step} chained={chained}");
            assert_eq!(apply(&delta, &prev).unwrap(), scene, "apply(diff(a,b),a)==b at step {step}");
            assert!(delta.validate().is_ok() && scene.validate().is_ok());
            assert_eq!(diff(&scene, &scene).entities.len(), 0);
            prev = scene;
        }
    }
}

#[test]
fn dependency_index_closures_are_transitive_and_cycle_safe() {
    let mut ix = DependencyIndex::new();
    ix.set("c", ["b".to_owned()]);
    ix.set("b", ["a".to_owned()]);
    ix.set("a", ["c".to_owned()]);
    ix.set("d", ["a".to_owned(), "d".to_owned()]);
    assert_eq!(ix.dependencies("d"), set(&["a", "b", "c"]));
    assert_eq!(ix.dependents(&set(&["b"])), set(&["a", "b", "c", "d"]));
    ix.set("a", Vec::<String>::new());
    assert_eq!(ix.dependents(&set(&["b"])), set(&["b", "c"]));
    ix.remove("c");
    assert!(ix.dependencies("c").is_empty() && ix.direct("b") == set(&["a"]));
}

#[test]
fn text_alternative_is_bilingual_and_names_the_governing_element() {
    let scene = projector(false).project(&doc(3), &eval(&[("m0", 0.3), ("m2", 1.25)]), &ctx(1)).unwrap();
    let text = describe(&scene);
    assert_eq!(text, scene.description);
    assert!(text.en.starts_with("Scene with 7 elements. Members: 3; Loads: 3; Surfaces: 1."), "{}", text.en);
    assert!(text.en.contains("Governing utilisation 125 % at Member m2 (fail). 1 element fails."), "{}", text.en);
    assert!(text.de.starts_with("Szene mit 7 Elementen. Bauteile: 3; Lasten: 3; Flächen: 1."), "{}", text.de);
    assert!(text.de.contains("Maßgebende Ausnutzung 125 % bei Bauteil m2 (nicht erfüllt). 1 Element nicht erfüllt."), "{}", text.de);
    let lines = describe_entities(&scene);
    assert_eq!(lines.en.lines().count(), 7);
    assert!(lines.en.contains("Member m0: I-shape, 4.00 m, utilisation 30 %, pass") && lines.de.contains("Bauteil m0: I-Profil, 4.00 m, Ausnutzung 30 %, erfüllt"));
    let empty = describe(&empty_scene());
    assert_eq!(empty.en, "Scene with 0 elements.");
    assert_eq!(empty.de, "Szene mit 0 Elementen.");
}

#[test]
fn layer_visibility_follows_the_context_without_changing_fragments() {
    let mut p = projector(false);
    let d = doc(2);
    let e = eval(&[]);
    let all = p.project(&d, &e, &ctx(1)).unwrap();
    assert!(all.layers.iter().all(|l| l.visible));
    let hidden = ProjectionContext { visible_layers: Some(vec!["members".into()]), ..ctx(1) };
    let some = p.project(&d, &e, &hidden).unwrap();
    assert!(some.layers.iter().filter(|l| l.id != "members").all(|l| !l.visible) && some.layers.iter().any(|l| l.id == "members" && l.visible));
}

#[test]
fn no_placeholder_description_leaves_the_projection() {
    let merged = merge(&[], SceneRevision { document_seq: 0, report_seq: 0 });
    for scene in [merged, empty_scene()] {
        assert!(!scene.description.en.trim().is_empty() && !scene.description.de.trim().is_empty());
        assert_eq!(scene.description, describe(&scene));
    }
}

#[test]
fn german_leak_detectors_cover_scenes_diagrams_and_handles() {
    let mut scene = empty_scene();
    assert!(description::german_scene_leaks(&scene).is_empty());
    scene.description = LocalizedCopy::new("The scene is not valid", "The scene is not valid for the check");
    assert_eq!(description::german_scene_leaks(&scene).len(), 1);
}
