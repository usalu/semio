use semio_s_artifact_norm_en1995_model::{CaseRow, En1995Snapshot};
use semio_s_artifact_norm_en1995_schema::checks::timber::{catalogue_demo_cases, catalogue_demo_document, entity_demo_document, evaluate_structure, registry, steel_connection_demo_document, En1995Cases};
use semio_s_artifact_norm_en1995_schema::spec_trace::TABLE;
use semio_s_plugin_norm_compliance::design_case::{seed, CaseCatalogue};
use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, Emitted, Realisation};
use semio_s_plugin_norm_compliance::AnnexChoice;
use std::collections::BTreeSet;

fn example_documents() -> Vec<En1995Snapshot> {
    semio_s_artifact_norm_en1995_examples::examples()
        .into_iter()
        .map(|example| semio_s_artifact_norm_en1995_io::text::snapshot::decode_en1995_dsl(&example.document()).unwrap_or_else(|e| panic!("{}: {e}", example.id())))
        .chain([En1995Snapshot::compliant_building_beam(), En1995Snapshot::noncompliant_building(), catalogue_demo_document(), entity_demo_document(), steel_connection_demo_document()])
        .collect()
}

#[test]
fn the_trace_covers_every_registry_entry() {
    let spec: BTreeSet<&str> = TABLE.spec_ids.iter().copied().collect();
    let registered: BTreeSet<&str> = registry().iter().map(|e| e.id).collect();
    assert!(registered.is_subset(&spec), "registry entries missing in the spec trace: {:?}", registered.difference(&spec).collect::<Vec<_>>());
}

#[test]
fn every_spec_id_is_traced_to_a_check_an_example_or_design_case_emits() {
    let emitted = example_documents()
        .iter()
        .fold(Emitted::default(), |emitted, document| emitted.with_checks(evaluate_structure(document).checks.into_iter().map(|c| c.id)))
        .with_case_catalogue::<En1995Cases>(AnnexChoice::De);
    assert_traced(&TABLE, &emitted);
}

#[test]
fn every_entry_runs_from_a_case_row_of_a_document() {
    for entry in En1995Cases::entries() {
        let mut document = En1995Snapshot::empty(AnnexChoice::De);
        document.cases = vec![CaseRow::from(&seed::<En1995Cases>(&entry, "traced"))];
        let tag = format!(".{}.", entry.id.to_ascii_lowercase());
        let report = evaluate_structure(&document);
        assert!(report.checks.iter().any(|c| c.id.contains(&tag)), "{} produced no check from its case row", entry.id);
    }
}

#[test]
fn the_demo_document_holds_one_evaluable_case_per_catalogue_entry() {
    let rows = catalogue_demo_cases();
    let entries = En1995Cases::entries();
    assert_eq!(rows.len(), entries.len());
    let ids: BTreeSet<&str> = rows.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids.len(), rows.len(), "demo case ids are unique");
    let report = evaluate_structure(&catalogue_demo_document());
    for entry in &entries {
        let tag = format!(".{}.", entry.id.to_ascii_lowercase());
        assert!(report.checks.iter().any(|c| c.id.contains(&tag)), "{} produced no check from the demo document", entry.id);
    }
}

#[test]
fn every_catalogue_prefix_row_is_emitted_by_the_demo_documents() {
    let entries: BTreeSet<String> = En1995Cases::entries().iter().map(|entry| entry.id.to_ascii_lowercase()).collect();
    let emitted = [catalogue_demo_document(), entity_demo_document(), steel_connection_demo_document()]
        .iter()
        .fold(Emitted::default(), |emitted, document| emitted.with_checks(evaluate_structure(document).checks.into_iter().map(|c| c.id)));
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
