use semio_s_artifact_norm_din16798_inferences::evaluate;
use semio_s_artifact_norm_din16798_io::text::snapshot::decode_din16798_dsl;

#[test]
fn primary_asset_fails_the_claimed_checks_with_remedies() {
    let report = evaluate(&decode_din16798_dsl(crate::noncompliant_office::PRIMARY_TEXT).expect("noncompliant-office DSL decodes"));
    let failing: Vec<_> = report.failing().collect();
    for id in ["din16798-1.top.summer.zone-office-fail", "din16798-1.vent.zone-office-fail", "din16798-1.co2.zone-office-fail", "din16798-3.sfp.vent-weak", "din16798-7.n50"] {
        assert!(failing.iter().any(|c| c.id == id), "{id} must fail; failing: {:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    }
    for check in &failing {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
