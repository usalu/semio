use crate::catalogue::En1997Cases;
use crate::spec_trace::TABLE;
use crate::{check_project, catalogue_demo_document};
use semio_s_plugin_norm_compliance::design_case::CaseCatalogue;
use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, Emitted, Realisation};
use semio_s_plugin_norm_compliance::AnnexChoice;

/// 🧭️ The compliant package, its showcase variant (placed footings, a seismic wall, a 6 m excavation at the embedded wall) and a variant with an undrained footing layer and a pile under tension, a showcase variant whose first layer takes its c_u from the investigations, and a geotechnics variant (an oedometer series, a pumped well, a settling ground with dynamic tests, a reinforced wall): the entities a spec id traces to are all present in one of them.
fn trace_documents() -> Vec<semio_s_artifact_norm_en1997_model::En1997Snapshot> {
    let mut cohesive = semio_s_artifact_norm_en1997_model::compliant_demo();
    cohesive.layers[0].cohesion_undrained = 60_000.0;
    cohesive.piles[0].tension_variable = 80_000.0;
    let mut geotechnics = semio_s_artifact_norm_en1997_model::compliant_demo();
    let test = &mut geotechnics.lab_tests[0];
    (test.kind, test.compression_index, test.swelling_index, test.stress_increment, test.secondary_compression_index, test.design_life) = ("oedometer".into(), 0.25, 0.04, 100_000.0, 0.01, 1.58e9);
    let water = &mut geotechnics.layer_waters[0];
    (water.well_radius, water.well_water_depth, water.well_capacity) = (0.3, 12.0, 0.01);
    let pile = &mut geotechnics.piles[0];
    (pile.settling_thickness, pile.negative_friction_beta, pile.dynamic_test_count, pile.dynamic_resistance_mean, pile.dynamic_resistance_min) = (4.0, 0.25, 5.0, 2_600_000.0, 2_400_000.0);
    (pile.hammer_weight, pile.drop_height, pile.set_per_blow, pile.elastic_compression, pile.hammer_efficiency) = (100_000.0, 1.0, 0.01, 0.02, 0.9);
    let wall = &mut geotechnics.retaining_walls[0];
    (wall.reinforcement_spacing, wall.reinforcement_width, wall.reinforcement_friction) = (0.6, 0.1, 0.8);
    let mut derived = semio_s_artifact_norm_en1997_model::showcase_demo();
    let layer = derived.layers[0].id.clone();
    derived.characteristic_bases = vec![semio_s_artifact_norm_en1997_model::CharacteristicBasis { id: "basis-1".into(), layer_id: layer, parameter: semio_s_artifact_norm_en1997_model::CharacteristicParameter::UndrainedStrength, basis: semio_s_artifact_norm_en1997_model::ParameterBasis::Derived }];
    vec![semio_s_artifact_norm_en1997_model::compliant_demo(), semio_s_artifact_norm_en1997_model::showcase_demo(), cohesive, geotechnics, derived]
}

fn emitted() -> Emitted {
    trace_documents().iter().fold(Emitted::default().with_case_catalogue::<En1997Cases>(AnnexChoice::De), |emitted, document| emitted.with_checks(check_project(document).checks.iter().map(|check| check.id.clone())))
}

#[test]
fn every_spec_id_is_traced_to_a_check_of_a_typed_entity_or_a_design_case_entry_that_produces_checks() {
    assert_traced(&TABLE, &emitted());
}

#[test]
fn the_demo_document_holds_no_design_case_for_a_spec_id_that_an_entity_check_realises() {
    let realised: Vec<&str> = TABLE.items.iter().filter(|item| matches!(item.realisation, Realisation::Check(_))).map(|item| item.id).collect();
    assert!(!realised.is_empty());
    let document = catalogue_demo_document();
    let duplicates: Vec<&str> = document.cases.iter().map(|case| case.entry_id.as_str()).filter(|entry| realised.contains(entry)).collect();
    assert!(duplicates.is_empty(), "entity-realised ids that still have a demo case: {duplicates:?}");
}

#[test]
fn every_catalogue_prefix_row_is_emitted_by_the_demo_documents() {
    let entries: std::collections::BTreeSet<String> = En1997Cases::entries().iter().map(|entry| entry.id.to_ascii_lowercase()).collect();
    let emitted = trace_documents()
        .iter()
        .chain([catalogue_demo_document()].iter())
        .fold(Emitted::default(), |emitted, document| emitted.with_checks(check_project(document).checks.iter().map(|check| check.id.clone())));
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
