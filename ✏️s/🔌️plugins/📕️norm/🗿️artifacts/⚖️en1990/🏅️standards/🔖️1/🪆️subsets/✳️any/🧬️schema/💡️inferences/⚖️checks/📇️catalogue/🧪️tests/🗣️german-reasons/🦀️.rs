use super::*;
use semio_s_plugin_norm_compliance::{explain_reason, german_check_leaks, UNMAPPED_REASON};

#[test]
fn every_catalogue_entry_explains_missing_input_in_german_with_a_coded_reason() {
    let mut leaks = Vec::new();
    let mut unmapped = Vec::new();
    for entry in registry() {
        for annex in [AnnexChoice::En, AnnexChoice::De] {
            let case = Case::new("probe", annex);
            if let Err(reason) = evaluate_outcomes(entry.id, &case) {
                if explain_reason(&reason).code == UNMAPPED_REASON {
                    unmapped.push(format!("{}: {reason}", entry.id));
                }
            }
            leaks.extend(german_check_leaks(&evaluate_case(entry.id, &case)));
        }
    }
    assert!(unmapped.is_empty(), "reasons without a rule in the shared reason table: {unmapped:#?}");
    assert!(leaks.is_empty(), "English sentences in German check texts: {leaks:#?}");
}
