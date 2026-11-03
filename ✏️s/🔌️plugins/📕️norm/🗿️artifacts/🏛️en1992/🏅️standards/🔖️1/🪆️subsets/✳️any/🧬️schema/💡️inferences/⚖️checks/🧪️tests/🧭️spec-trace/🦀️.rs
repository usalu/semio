use crate::spec_trace::TABLE;
use crate::En1992Cases;
use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, Emitted};
use semio_s_plugin_norm_compliance::AnnexChoice;

#[test]
fn every_spec_id_is_traced_to_an_entity_check_or_a_design_case_entry_that_produces_checks() {
    assert_traced(&TABLE, &Emitted::default().with_case_catalogue::<En1992Cases>(AnnexChoice::De).with_checks(crate::reach_tests::report_ids()));
}

#[test]
fn no_emitted_check_family_is_untraced() {
    use semio_s_plugin_norm_compliance::spec_trace::Realisation;
    let fragments: Vec<&str> = TABLE.items.iter().filter_map(|item| match item.realisation { Realisation::Check(fragments) => Some(fragments), _ => None }).flatten().copied().collect();
    let untraced: std::collections::BTreeSet<String> = crate::reach_tests::report_ids().into_iter().filter(|id| !fragments.iter().any(|fragment| id.contains(fragment))).map(|id| id.rsplit_once('.').map_or(id.clone(), |(family, _)| family.to_string())).collect();
    assert!(untraced.is_empty(), "emitted check families without a spec id: {untraced:?}");
}

#[test]
fn every_catalogue_prefix_row_is_emitted_by_the_demo_documents() {
    use semio_s_plugin_norm_compliance::design_case::CaseCatalogue;
    use semio_s_plugin_norm_compliance::spec_trace::Realisation;
    let entries: std::collections::BTreeSet<String> = En1992Cases::entries().iter().map(|entry| entry.id.to_ascii_lowercase()).collect();
    let mut spaced = semio_s_artifact_norm_en1992_model::En1992Snapshot::compliant_office_frame();
    for layer in spaced.members.iter_mut().flat_map(|member| member.longitudinal.iter_mut()) {
        layer.count = layer.count.max(3);
    }
    let spaced_ids = spaced.members.iter().flat_map(|member| crate::evaluate_member(&spaced, member)).map(|check| check.id);
    let emitted = Emitted::default().with_checks(crate::reach_tests::report_ids()).with_checks(spaced_ids);
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
