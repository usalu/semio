use crate::Din4108Family;
use semio_s_artifact_norm_din4108_inferences::evaluate;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub(super) fn documents() -> Vec<Din4108Snapshot> {
    vec![Din4108Snapshot::default(), Din4108Snapshot::failing_thin_insulation(), Din4108Snapshot { zones: vec![], elements: vec![], thermal_bridges: vec![], ..Din4108Snapshot::default() }, Din4108Snapshot::default()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = Din4108Family, full = evaluate, live = semio_s_artifact_norm_din4108_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <Din4108Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("zones.0.id") && global.covers("elements.0.zoneId") && global.covers("thermalBridges.0.id"));
    assert!(!global.overlaps("airtightnessN50") && !global.overlaps("tIntC"), "a scalar edit leaves the integrity unit clean");
    let airtightness = reads("part:airtightness");
    assert!(airtightness.covers("airtightnessN50") && airtightness.covers("hasMechanicalVentilation"));
    assert!(!airtightness.overlaps("elements.0.layers.0.thicknessM"), "a layer edit leaves the airtightness verdict clean");
}

#[test]
fn saved_pressure_cases_keep_their_results_in_the_live_report() {
    use semio_s_artifact_norm_din4108_inferences::live;
    use semio_s_plugin_norm_compliance::CheckStatus;
    let cases = serde_json::from_str(include_str!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/📏️pressure-cases.json")).expect("pressure fixtures");
    let document = Din4108Snapshot { cases, ..Din4108Snapshot::default() };
    let report = live::report(&document);
    assert_eq!(*report, semio_s_plugin_norm_compliance::incremental_evaluation::owned::<Din4108Family>(&document, evaluate(&document)));
    let pressure: Vec<_> = report.checks.iter().filter(|check| check.id.starts_with("gmodg.26.")).collect();
    assert_eq!(pressure.len(), 4);
    assert!(pressure[..2].iter().all(|check| check.status == CheckStatus::Pass));
    assert_eq!(pressure[3].status, CheckStatus::Fail);
}

#[test]
fn catalogue_measurement_cases_can_be_edited_and_removed() {
    use semio_s_artifact_norm_din4108_checks::design_cases::Din4108Cases;
    use semio_s_plugin_norm_compliance::design_case::CaseCatalogue;
    let entry = Din4108Cases::entries().remove(0);
    let fresh = Din4108Cases::new_document_case(&entry, "new-test");
    assert!(fresh.values.numbers.is_empty());
    assert_eq!(fresh.values.texts.get("test_protocol").map(String::as_str), Some("unconfirmed"));
    semio_s_plugin_norm_app_surface::assert_case_edit_paths::<Din4108Family, Din4108Cases>(
        Din4108Snapshot::default(),
        |document| document.cases.clone(),
        semio_s_artifact_norm_din4108_mutations::resolve,
    );
}

#[test]
fn moisture_cases_drive_the_live_period_scope_and_roof_evidence() {
    use semio_s_artifact_norm_din4108_inferences::live;
    use semio_s_plugin_norm_compliance::CheckStatus;
    let cases = serde_json::from_str(include_str!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/💧annex-a-cases.json")).unwrap();
    let mut document = Din4108Snapshot { cases, ..Din4108Snapshot::default() };
    let report = live::report(&document);
    let site = report.checks.iter().find(|c| c.id == "din4108-3.annex-a.basis.site").unwrap();
    let roof = report.checks.iter().find(|c| c.id == "din4108-3.annex-a.basis.roof-exposure").unwrap();
    assert_eq!(site.status, CheckStatus::Info);
    assert!(roof.explanation.en.contains("2000"));
    for (id, expected) in [
        ("slab-case", CheckStatus::Info), ("attic-case", CheckStatus::Pass),
        ("perimeter", CheckStatus::Pass), ("wall", CheckStatus::Pass),
        ("roof-table3", CheckStatus::Pass), ("condensate-materials", CheckStatus::Info),
    ] {
        let result = report.checks.iter().find(|check| check.id == format!("din4108-3.annex-a.basis.{id}")).unwrap();
        assert_eq!(result.status, expected, "{id}");
    }
    let planning: Vec<_> = report.checks.iter().filter(|check| check.id.starts_with("din4108-7.plan.")).collect();
    assert_eq!(planning.len(), 4);
    assert!(planning.iter().all(|check| check.status == CheckStatus::Info));

    let wall = report.checks.iter().find(|c| c.id == "din4108-3.glaser.wall-north").unwrap();
    assert_ne!(wall.status, CheckStatus::NotApplicable);
    document.cases[0].numbers[0].value = 700.01;
    let report = live::report(&document);
    let wall = report.checks.iter().find(|c| c.id == "din4108-3.glaser.wall-north").unwrap();
    assert_eq!(wall.status, CheckStatus::NotApplicable);
}
