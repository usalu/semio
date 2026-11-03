use crate::checks::masonry::{detailing_checks, evaluate_building, En1996Cases};
use crate::spec_trace::TABLE;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_plugin_norm_compliance::design_case::CaseCatalogue;
use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, standing, Emitted, Realisation};
use semio_s_plugin_norm_compliance::AnnexChoice;

fn emitted() -> Emitted {
    let documents = [
        En1996Snapshot::compliant_clay_wall(),
        En1996Snapshot::noncompliant_multi_fail(),
        En1996Snapshot::opening_wall_example(),
        En1996Snapshot::members_wall_example(),
        En1996Snapshot::basement_wall_example(),
        En1996Snapshot::concentrated_load_example(),
        En1996Snapshot::reinforced_wall_example(),
        En1996Snapshot::bracing_walls_example(),
    ];
    documents.iter().fold(Emitted::default().with_case_catalogue::<En1996Cases>(AnnexChoice::De), |emitted, d| {
        let walls = evaluate_building(d.annex, d.masonry_class, d.design_situation, d.reliability_class, d.storeys, &d.walls);
        let details = detailing_checks(d.annex, &d.walls, &d.chases, &d.movement_joints);
        emitted.with_checks(walls.checks.iter().chain(details.checks.iter()).map(|c| c.id.as_str()))
    })
}

#[test]
fn every_spec_id_is_traced_to_a_check_of_a_typed_entity_or_to_a_design_case_entry_that_produces_checks() {
    assert_traced(&TABLE, &emitted());
}

#[test]
fn the_ids_realised_by_typed_entities_have_no_demo_design_case() {
    let rows = crate::checks::masonry::catalogue_demo_cases();
    for item in TABLE.items {
        let has_demo = rows.iter().any(|row| row.entry_id == item.id);
        assert_eq!(has_demo, matches!(item.realisation, Realisation::DesignCase(_)), "{}: a check-realised id has no demo case, a design-case id has one", item.id);
    }
}

#[test]
fn the_check_realised_ids_stand_in_the_reports_of_the_examples() {
    let mut report = semio_s_plugin_norm_compliance::CheckReport::default();
    for d in [En1996Snapshot::members_wall_example(), En1996Snapshot::basement_wall_example(), En1996Snapshot::concentrated_load_example(), En1996Snapshot::reinforced_wall_example(), En1996Snapshot::bracing_walls_example()] {
        for c in evaluate_building(d.annex, d.masonry_class, d.design_situation, d.reliability_class, d.storeys, &d.walls).checks.into_iter().chain(detailing_checks(d.annex, &d.walls, &d.chases, &d.movement_joints).checks) {
            report.push(c);
        }
    }
    let standings = standing(&TABLE, &report);
    assert_eq!(standings.len(), TABLE.items.iter().filter(|item| matches!(item.realisation, Realisation::Check(_))).count());
    assert!(standings.iter().all(|s| s.checks > 0), "{standings:?}");
}

#[test]
fn flexure_and_concentrated_bearing_are_emitted_by_their_load_case_paths() {
    let ids = |d: &En1996Snapshot| evaluate_building(d.annex, d.masonry_class, d.design_situation, d.reliability_class, d.storeys, &d.walls).checks.into_iter().map(|c| c.id).collect::<Vec<_>>();
    let plain = ids(&En1996Snapshot::compliant_clay_wall());
    assert!(plain.iter().any(|id| id.starts_with("en1996.6.3.flexure.wall-north.ulsinz")), "every load case reports its lateral flexure");
    assert!(plain.iter().all(|id| !id.contains("6.1.3.")));
    let beam = ids(&En1996Snapshot::concentrated_load_example());
    for family in ["en1996.6.1.3.concentrated.wall-concentrated.ulsinz.beam-B", "en1996.6.1.3.area.wall-concentrated.ulsinz.beam-B"] {
        assert!(beam.iter().any(|id| id == family), "{family} is not emitted");
    }
}

#[test]
fn every_catalogue_prefix_row_is_emitted_by_the_demo_documents() {
    let entries: std::collections::BTreeSet<String> = En1996Cases::entries().iter().map(|entry| entry.id.to_ascii_lowercase()).collect();
    let documents = [
        En1996Snapshot::compliant_clay_wall(),
        En1996Snapshot::noncompliant_multi_fail(),
        En1996Snapshot::opening_wall_example(),
        En1996Snapshot::members_wall_example(),
        En1996Snapshot::basement_wall_example(),
        En1996Snapshot::concentrated_load_example(),
        En1996Snapshot::reinforced_wall_example(),
        En1996Snapshot::bracing_walls_example(),
    ];
    let emitted = documents.iter().fold(Emitted::default(), |emitted, d| {
        let walls = evaluate_building(d.annex, d.masonry_class, d.design_situation, d.reliability_class, d.storeys, &d.walls);
        let details = detailing_checks(d.annex, &d.walls, &d.chases, &d.movement_joints);
        emitted.with_checks(walls.checks.iter().chain(details.checks.iter()).map(|c| c.id.as_str()))
    });
    let rows: Vec<(&str, &str)> = TABLE
        .items
        .iter()
        .filter_map(|item| match item.realisation {
            Realisation::Check(fragments) => Some(fragments.iter().map(move |fragment| (item.id, *fragment))),
            _ => None,
        })
        .flatten()
        .filter(|(_, fragment)| fragment.split('.').take(2).any(|segment| entries.contains(segment)))
        .collect();
    assert!(!rows.is_empty());
    let missing: Vec<_> = rows.iter().filter(|(_, fragment)| !emitted.emits(fragment)).collect();
    assert!(missing.is_empty(), "catalogue-prefix rows no demo document emits: {missing:?}");
}
