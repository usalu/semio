use super::*;
use crate::projection::{merge, SceneBuilder};
use norm::{AnnexChoice, ClauseId, Quantity, QuantityKind, SubjectRef};
use std::sync::Arc;

fn check(id: &str, owner: &str, demand: f64) -> CheckResult {
    CheckResult::assess(id, "3", ClauseId::new("EN 1993-1-1", "6", "6.2.5"), SubjectRef::new(owner, format!("members[id={owner}]"), LocalizedCopy::new(owner, owner)), LocalizedCopy::new("Bending", "Biegung"))
        .utilization(Quantity::new(QuantityKind::Moment, demand), Quantity::new(QuantityKind::Moment, 100.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("x", "x"))
        .build()
}

fn report(rows: &[(&str, &str, f64)]) -> CheckReport {
    let mut report = CheckReport::default();
    rows.iter().for_each(|(id, owner, demand)| report.push(check(id, owner, *demand)));
    report
}

fn fragment(ids: &[&str]) -> Fragment {
    let mut builder = SceneBuilder::new();
    for id in ids {
        builder.group(&EntityRef::new(EntityKind::Group, *id, format!("members[id={id}]")), Vec::new()).add();
    }
    builder.into_fragment()
}

fn scene_of(fragment: Fragment) -> StructuralScene {
    merge(&[Arc::new(fragment)], crate::projection_kit::test_support::revision())
}

fn ids(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn the_worst_check_of_an_owner_colours_its_entity_and_reseals_it() {
    let rows = report(&[("a.m", "m1", 40.0), ("a.v", "m1", 130.0), ("a.n", "m2", 30.0)]);
    let lit = highlight_owners(fragment(&["m1", "m2", "m3"]), &rows, Part::Entity("m1"));
    let of = |id: &str| lit.entities.iter().find(|e| e.entity.id == id).expect("entity");
    let (m1, m2, m3) = (of("m1"), of("m2"), of("m3"));
    assert_eq!(m1.utilisation.as_ref().map(|u| (u.check_id.as_str(), u.status)), Some(("a.v", CheckStatus::Fail)));
    assert_eq!(m2.utilisation.as_ref().map(|u| u.status), Some(CheckStatus::Pass));
    assert!(m3.utilisation.is_none());
    assert!(matches!(m1.appearance.channel, ColourChannel::Utilisation));
    assert_eq!(m1.content_hash, m1.compute_hash());
}

#[test]
fn a_sub_entity_without_a_main_entity_stands_in_for_its_owner_and_a_stronger_binding_stays() {
    let rows = report(&[("a.m", "m1", 130.0)]);
    let lit = highlight_owners(fragment(&["m1:web", "m1:flange"]), &rows, Part::Entity("m1"));
    assert_eq!(lit.entities.iter().map(|e| e.utilisation.is_some()).collect::<Vec<_>>(), vec![true, false]);
    let mut again = lit.clone();
    again.entities[0].utilisation = Some(UtilisationBinding { check_id: "own".into(), ratio: 5.0, status: CheckStatus::Fail, per_station: None, stale: false });
    let kept = highlight_owners(again, &rows, Part::Entity("m1"));
    assert_eq!(kept.entities[0].utilisation.as_ref().map(|u| u.check_id.as_str()), Some("own"));
}

#[test]
fn checks_filed_under_no_entity_make_one_global_badge_on_the_whole_and_global_runs_only() {
    let rows = report(&[("g.annex", "", 120.0), ("g.title", "", 20.0), ("a.m", "m1", 10.0)]);
    let badge = |part| highlight_owners(fragment(&["m1"]), &rows, part).annotations.iter().find(|a| a.id() == GLOBAL_BADGE).cloned();
    assert!(badge(Part::Entity("m1")).is_none());
    for part in [Part::All, Part::Global] {
        let Some(Annotation::Label { text, tone, .. }) = badge(part) else { panic!("badge") };
        assert_eq!((text.en.as_str(), text.de.as_str()), ("2 document-level checks, worst 1.20", "2 Dokument-Prüfungen, größte Ausnutzung 1.20"));
        assert!(matches!(tone, Tone::Error));
    }
    assert!(global_badge(&report(&[("a.m", "m1", 10.0)])).is_none());
}

#[test]
fn the_audit_names_orphans_unlit_failures_and_missing_badges() {
    let owners = ids(&["m1", "m2"]);
    let rows = report(&[("a.m", "m1", 130.0), ("a.x", "ghost", 10.0), ("g.annex", "", 50.0), ("a.none", "", 50.0)]);
    let document_level = |id: &str| id.starts_with("g.");
    let plain = scene_of(fragment(&["m1", "m2"]));
    let audit = ownership_audit(&rows, &plain, &owners, document_level);
    assert_eq!((audit.orphans, audit.unlit, audit.global, audit.badge), (vec!["a.x".to_owned(), "a.none".to_owned()], vec!["a.m".to_owned()], vec!["g.annex".to_owned()], false));
    let clean = report(&[("a.m", "m1", 130.0), ("g.annex", "", 50.0)]);
    let lit = scene_of(highlight_owners(fragment(&["m1", "m2"]), &clean, Part::All));
    assert!(ownership_audit(&clean, &lit, &owners, document_level).is_clean());
}

#[test]
fn document_level_checks_are_owned_by_the_project_site_that_leads_the_fragment_and_lights_with_the_worst_of_them() {
    let rows = report(&[("g.annex", "", 120.0), ("g.title", "", 20.0), ("a.m", "m1", 10.0)]);
    for part in [Part::All, Part::Global] {
        let led = highlight_owners(fragment(&["m1"]), &rows, part);
        let site = &led.entities[0];
        assert_eq!((site.entity.id.as_str(), site.entity.kind), (crate::projection::SITE_ENTITY, EntityKind::Plate));
        assert_eq!(site.utilisation.as_ref().map(|u| (u.check_id.as_str(), u.status)), Some(("g.annex", CheckStatus::Fail)));
        assert_eq!(led.entities.len(), 2);
    }
    assert_eq!(highlight_owners(fragment(&["m1"]), &rows, Part::Entity("m1")).entities.len(), 1);
    assert_eq!(highlight_owners(fragment(&["m1"]), &report(&[("a.m", "m1", 10.0)]), Part::All).entities.len(), 1);
}

#[test]
fn the_audit_demands_the_project_site_for_document_level_checks_and_a_lit_site_for_failing_ones() {
    let owners = ids(&["m1"]);
    let rows = report(&[("g.annex", "", 120.0)]);
    let document_level = |id: &str| id.starts_with("g.");
    let bare = ownership_audit(&rows, &scene_of(highlight_owners(fragment(&["m1"]), &report(&[]), Part::All)), &owners, document_level);
    assert!(!bare.is_clean() && !bare.site);
    let mut unlit = highlight_owners(fragment(&["m1"]), &rows, Part::All);
    unlit.entities[0].utilisation = None;
    let audit = ownership_audit(&rows, &scene_of(unlit), &owners, document_level);
    assert_eq!(audit.unlit, vec!["g.annex".to_owned()]);
    assert!(ownership_audit(&rows, &scene_of(highlight_owners(fragment(&["m1"]), &rows, Part::All)), &owners, document_level).is_clean());
}
