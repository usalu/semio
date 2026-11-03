use super::*;

/// 🗂️ Every case editing path of EN1992 (add from the catalogue, set a number, typed unit row, set a choice, clear with the null set, duplicate, remove) resolves to the artifact's own mutations, and after each step the live report equals the full evaluation.
#[test]
fn every_case_editing_path_resolves_to_mutations_and_keeps_the_live_report_exact() {
    semio_s_plugin_norm_app_surface::assert_case_edit_paths::<En1992Family, semio_s_artifact_norm_en1992_checks::En1992Cases>(semio_s_artifact_norm_en1992_model::En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), |document| document.cases.clone(), |document, edit| semio_s_artifact_norm_en1992_mutations::resolve_edit(document, edit));
}
