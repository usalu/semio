use super::*;
use dsl::{FromValue, ToValue};

fn subject() -> SubjectRef {
    SubjectRef::new("member-B12", "members[0].vEd", LocalizedCopy::new("Member B12", "Bauteil B12"))
}

fn title() -> LocalizedCopy {
    LocalizedCopy::new("Shear resistance", "Querkrafttragfähigkeit")
}

fn remedy_for(target: SubjectRef) -> Remedy {
    Remedy::at_least(
        target,
        Quantity::force_kn(120.0),
        Quantity::force_kn(150.0),
        LocalizedCopy::new("Increase resistance from 120 kN to at least 150 kN.", "Tragwiderstand von 120 kN auf mindestens 150 kN erhöhen."),
    )
}

#[semio_framework_async_macros::async_test]
async fn builder_utilization_pass_and_fail() {
    let clause = ClauseId::new("EN 1992", "1-1", "6.2.2");
    let pass = CheckResult::assess("en1992.6.2.2.pass", "DIN EN 1992-1-1", clause.clone(), subject(), title())
        .utilization(Quantity::force_kn(80.0), Quantity::force_kn(100.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("u ≤ 1", "u ≤ 1"))
        .build();
    assert_eq!(pass.status, CheckStatus::Pass);
    assert!((pass.utilization - 0.8).abs() < 1e-12);

    let fail = CheckResult::assess("en1992.6.2.2.fail", "DIN EN 1992-1-1", clause, subject(), title())
        .utilization(Quantity::force_kn(120.0), Quantity::force_kn(100.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("u > 1", "u > 1"))
        .remedy(remedy_for(subject()))
        .build();
    assert_eq!(fail.status, CheckStatus::Fail);
    assert!(fail.utilization > 1.0);
    assert!(!fail.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn builder_minimum_and_warn_above() {
    let clause = ClauseId::new("DIN 4108-3", "3", "6.1");
    let pass = CheckResult::assess("din4108.6.1", "DIN 4108-3", clause.clone(), subject(), LocalizedCopy::new("f_Rsi", "f_Rsi"))
        .minimum(Quantity::new(QuantityKind::Dimensionless, 0.8), Quantity::new(QuantityKind::Dimensionless, 0.25))
        .annex(AnnexChoice::De)
        .build();
    assert_eq!(pass.status, CheckStatus::Pass);

    let warn = CheckResult::assess("en1990.uls.warn", "DIN EN 1990", ClauseId::new("EN 1990", "1", "6.10"), subject(), title())
        .utilization(Quantity::stress_mpa(290.0), Quantity::stress_mpa(300.0))
        .warn_above(0.95)
        .annex(AnnexChoice::En)
        .explanation(LocalizedCopy::new("near limit", "nahe Grenze"))
        .build();
    assert_eq!(warn.status, CheckStatus::Warning);
    assert!(warn.utilization > 0.95 && warn.utilization <= 1.0);
}

#[semio_framework_async_macros::async_test]
async fn builder_not_applicable_and_explicit_status() {
    let na = CheckResult::assess("en1998.na", "DIN EN 1998-1", ClauseId::new("EN 1998", "1", "4.3"), subject(), title())
        .not_applicable(LocalizedCopy::new("Seismic not required", "Erdbeben nicht erforderlich"))
        .annex(AnnexChoice::De)
        .build();
    assert_eq!(na.status, CheckStatus::NotApplicable);
    assert_eq!(na.explanation.en, "Seismic not required");

    let explicit = CheckResult::assess("iso16757.meta", "ISO 16757-1", ClauseId::new("ISO 16757", "1", "5.2"), subject(), LocalizedCopy::new("Catalogue id", "Katalog-ID"))
        .status(CheckStatus::Pass)
        .explanation(LocalizedCopy::new("id present", "id vorhanden"))
        .annex(AnnexChoice::En)
        .build();
    assert_eq!(explicit.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn report_summary_per_part_and_complies() {
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("a.pass", "Part A", ClauseId::new("X", "1", "1"), subject(), title())
            .utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(AnnexChoice::En)
            .build(),
    );
    report.push(
        CheckResult::assess("a.warn", "Part A", ClauseId::new("X", "1", "2"), subject(), title())
            .utilization(Quantity::new(QuantityKind::Dimensionless, 0.97), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .warn_above(0.95)
            .annex(AnnexChoice::En)
            .build(),
    );
    report.push(
        CheckResult::assess("b.fail", "Part B", ClauseId::new("Y", "1", "1"), subject(), title())
            .utilization(Quantity::new(QuantityKind::Dimensionless, 1.2), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .remedy(remedy_for(subject()))
            .annex(AnnexChoice::De)
            .build(),
    );
    report.push(
        CheckResult::assess("b.na", "Part B", ClauseId::new("Y", "1", "2"), subject(), title())
            .not_applicable(LocalizedCopy::new("n/a", "n/a"))
            .annex(AnnexChoice::De)
            .build(),
    );

    assert_eq!(report.summary.total, 4);
    assert_eq!(report.summary.pass, 1);
    assert_eq!(report.summary.warning, 1);
    assert_eq!(report.summary.fail, 1);
    assert_eq!(report.summary.not_applicable, 1);
    assert!(!report.complies());
    assert!((report.worst_utilization() - 1.2).abs() < 1e-12);
    assert_eq!(report.failing().count(), 1);

    let parts = report.by_part();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].0, "Part A");
    assert_eq!(parts[0].1.len(), 2);
    assert_eq!(parts[1].0, "Part B");
    assert_eq!(report.summary.parts[0].complies, true);
    assert_eq!(report.summary.parts[1].complies, false);
}

#[semio_framework_async_macros::async_test]
async fn fail_requires_remedy_invariant() {
    let check = CheckResult::assess("fail.with.remedy", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .utilization(Quantity::new(QuantityKind::Dimensionless, 2.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .remedy(remedy_for(subject()))
        .annex(AnnexChoice::En)
        .build();
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(!check.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn localized_copy_resolves_en_and_de() {
    let copy = LocalizedCopy::new("Pass", "Bestanden");
    assert_eq!(copy.resolve(&protocol::Locale::En), "Pass");
    assert_eq!(copy.resolve(&protocol::Locale::De), "Bestanden");
}

#[semio_framework_async_macros::async_test]
async fn check_report_value_round_trip_matches_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🎫️fixtures/🔬️check-report/🔣️.json")).expect("fixture");
    let report = sample_fixture_report();
    let encoded = serde_json::to_value(&report).expect("serialize");
    assert_eq!(encoded, fixture);
    let decoded: CheckReport = serde_json::from_value(fixture.clone()).expect("deserialize");
    assert_eq!(decoded, report);
    let via_value = CheckReport::from_value(report.to_value()).expect("FromValue");
    assert_eq!(via_value, report);
}

fn sample_fixture_report() -> CheckReport {
    let mut report = CheckReport::default();
    let target = SubjectRef::new("wall-north", "elements[0].layers[1].thicknessM", LocalizedCopy::new("North wall", "Nordwand"));
    report.push(
        CheckResult::assess(
            "din4108.2.uvalue.wall-north",
            "DIN 4108-2",
            ClauseId::new("DIN 4108", "2", "5.1"),
            target.clone(),
            LocalizedCopy::new("External wall U-value", "U-Wert Außenwand"),
        )
        .utilization(Quantity::u_value_w_m2k(0.28), Quantity::u_value_w_m2k(0.24))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new(
            "Computed U=0.28 W/(m²·K) exceeds limit 0.24 W/(m²·K).",
            "Berechneter U=0.28 W/(m²·K) überschreitet Grenzwert 0.24 W/(m²·K).",
        ))
        .remedy(Remedy::at_least(
            target,
            Quantity::length_m(0.08),
            Quantity::length_m(0.124),
            LocalizedCopy::new(
                "Increase insulation of layer 2 from 80 mm to at least 124 mm.",
                "Dämmstärke von Schicht 2 von 80 mm auf mindestens 124 mm erhöhen.",
            ),
        ))
        .build(),
    );
    report.push(
        CheckResult::assess(
            "din4108.2.psi.pass",
            "DIN 4108-2",
            ClauseId::new("DIN 4108", "2", "6.2"),
            SubjectRef::whole(LocalizedCopy::new("Building", "Gebäude")),
            LocalizedCopy::new("Thermal bridge ψ", "Wärmebrücke ψ"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, 0.4), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("Within limit.", "Innerhalb der Grenze."))
        .build(),
    );
    report
}

#[semio_framework_async_macros::async_test]
async fn table_lookup_linear_interpolates() {
    let table = [TableEntry1D { x: 0.0, y: 1.0 }, TableEntry1D { x: 10.0, y: 2.0 }];
    assert!((table_lookup_linear(&table, 5.0) - 1.5).abs() < 1e-9);
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(id = "norm.demo", extension = "demo-norm", layout = "lines")]
struct DemoDocument {
    value: f64,
}

impl_norm_artifact_record!(DemoDocument, extension = "demo-norm", envelope_id = "norm.demo");

#[semio_framework_async_macros::async_test]
async fn demo_document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&DemoDocument { value: 4.5 });
    store::os_store::test_support::assert_dsl_pack_equivalence(&DemoDocument { value: 4.5 });
}
