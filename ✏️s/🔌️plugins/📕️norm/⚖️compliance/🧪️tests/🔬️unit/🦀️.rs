use crate::*;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

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
    assert_eq!(copy.resolve(&semio_ui_locale::Locale::En), "Pass");
    assert_eq!(copy.resolve(&semio_ui_locale::Locale::De), "Bestanden");
}

#[semio_framework_async_macros::async_test]
async fn check_report_value_round_trip_matches_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔬️check-report/🔣️.json")).expect("fixture");
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
    report.push(
        CheckResult::assess(
            "din4108.2.psi.stations",
            "DIN 4108-2",
            ClauseId::new("DIN 4108", "2", "6.4"),
            SubjectRef::whole(LocalizedCopy::new("Building", "Gebäude")),
            LocalizedCopy::new("Thermal bridge ψ", "Wärmebrücke ψ"),
        )
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("Within limit.", "Innerhalb der Grenze."))
        .stations(vec![
            Station::ratio_of(&[0.0], Quantity::new(QuantityKind::Moment, 20000.0), Quantity::new(QuantityKind::Moment, 100000.0)),
            Station::ratio_of(&[0.5], Quantity::new(QuantityKind::Moment, 70000.0), Quantity::new(QuantityKind::Moment, 100000.0)),
            Station::along(1.0, 0.4),
        ])
        .build(),
    );
    report.push(
        CheckResult::assess(
            "din4108.2.psi.unverified",
            "DIN 4108-2",
            ClauseId::new("DIN 4108", "2", "6.3"),
            SubjectRef::whole(LocalizedCopy::new("Building", "Gebäude")),
            LocalizedCopy::new("Thermal bridge ψ", "Wärmebrücke ψ"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, 0.4), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("Within limit.", "Innerhalb der Grenze."))
        .ndp("ndp.en1992.e-12.1")
        .build(),
    );
    report
}

#[semio_framework_async_macros::async_test]
async fn table_lookup_linear_interpolates() {
    let table = [TableEntry1D { x: 0.0, y: 1.0 }, TableEntry1D { x: 10.0, y: 2.0 }];
    assert!((table_lookup_linear(&table, 5.0) - 1.5).abs() < 1e-9);
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[dsl(layout = "lines")]
#[artifact(id = "norm.demo", extension = "demo-norm")]
struct DemoDocument {
    value: f64,
}

impl_norm_artifact_record!(DemoDocument, extension = "demo-norm", envelope_id = "norm.demo");

#[semio_framework_async_macros::async_test]
async fn demo_document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&DemoDocument { value: 4.5 });
    store::os_store::test_support::assert_dsl_pack_equivalence(&DemoDocument { value: 4.5 });
}

fn dimensionless(value: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, value)
}

fn minimum_check(computed: f64, minimum: f64) -> CheckResult {
    CheckResult::assess("min", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .minimum(dimensionless(computed), dimensionless(minimum))
        .remedy(remedy_for(subject()))
        .build()
}

#[semio_framework_async_macros::async_test]
async fn minimum_failure_has_utilization_above_one() {
    let low = minimum_check(0.5, 1.0);
    assert_eq!(low.status, CheckStatus::Fail);
    assert!((low.utilization - 2.0).abs() < 1e-12);
    let zero = minimum_check(0.0, 1.0);
    assert_eq!(zero.status, CheckStatus::Fail);
    assert_eq!(zero.utilization, UTILIZATION_CAP);
    let nan = minimum_check(f64::NAN, 1.0);
    assert_eq!(nan.status, CheckStatus::Fail);
    assert!(nan.utilization >= 1.0 && nan.utilization.is_finite());
    let ok = minimum_check(2.0, 1.0);
    assert_eq!(ok.status, CheckStatus::Pass);
    assert!((ok.utilization - 0.5).abs() < 1e-12);
    let equal = minimum_check(1.0, 1.0);
    assert_eq!(equal.status, CheckStatus::Pass);
    assert!((equal.utilization - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn zero_limit_cannot_hide_a_demand() {
    let check = CheckResult::assess("zero", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .utilization(dimensionless(3.0), dimensionless(0.0))
        .remedy(remedy_for(subject()))
        .build();
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.utilization >= 1.0);
    let idle = CheckResult::assess("idle", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .utilization(dimensionless(0.0), dimensionless(0.0))
        .build();
    assert_eq!(idle.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn worst_utilization_never_hides_failures() {
    let mut report = CheckReport::default();
    report.push(minimum_check(0.9, 1.0));
    report.push(minimum_check(5.0, 1.0));
    report.push(
        CheckResult::assess("explicit", "Part", ClauseId::new("Z", "1", "2"), subject(), title())
            .status(CheckStatus::Fail)
            .remedy(remedy_for(subject()))
            .build(),
    );
    assert_eq!(report.summary.fail, 2);
    assert!(report.worst_utilization() >= 1.0);
    assert!(report.failing().all(|check| check.utilization >= 1.0));
    assert!(report.summary.parts.iter().all(|part| part.fail == 0 || part.worst_utilization >= 1.0));
}

#[semio_framework_async_macros::async_test]
async fn registry_is_sorted_unique_and_looked_up_statically() {
    assert!(NDP_TABLE.windows(2).all(|pair| pair[0].id < pair[1].id));
    let first = &NDP_TABLE[0];
    assert!(std::ptr::eq(ndp(first.id).expect("first"), first));
    assert!(ndp("ndp.does.not.exist").is_none());
    assert!(!ndp_is_verified("ndp.does.not.exist"));
    assert!(NDP_TABLE.iter().all(|entry| entry.status == NdpStatus::Unverified || (!entry.source.edition.is_empty() && !entry.source.document.is_empty())));
    assert_eq!(unverified_ndp_entries().count(), NDP_TABLE.iter().filter(|entry| entry.status == NdpStatus::Unverified).count());
    assert!(NDP_TABLE.iter().any(|entry| entry.status == NdpStatus::Verified) && unverified_ndp_entries().next().is_some());
    assert!(NDP_TABLE.iter().all(|entry| ndp_is_verified(entry.id) == (entry.status == NdpStatus::Verified)));
    assert!(NDP_TABLE.iter().all(|entry| {
        let status = pass_with(&[entry.id]).status;
        if entry.status == NdpStatus::Verified { status == CheckStatus::Pass } else { status == CheckStatus::UnverifiedBasis }
    }));
}

#[semio_framework_async_macros::async_test]
async fn registry_matches_the_language_agnostic_json() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../../📜️annex-and-tables/🔖️ndp/🔣️.json")).expect("registry json");
    let entries = data["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), NDP_TABLE.len());
    for (json, entry) in entries.iter().zip(NDP_TABLE) {
        assert_eq!(json["id"], entry.id);
        assert_eq!(json["code"], entry.code);
        assert_eq!(json["clause"], entry.clause);
        assert_eq!(json["symbol"], entry.symbol);
        assert_eq!(json["source"]["edition"], entry.source.edition);
        assert_eq!(json["source"]["document"], entry.source.document);
        assert_eq!(json["status"] == "verified", entry.status == NdpStatus::Verified);
        let annex_side = |key: &str| json.get("annexStatus").map_or(json["status"] == "verified", |per| per[key]["status"] == "verified");
        assert_eq!(annex_side("en"), entry.status_en == NdpStatus::Verified, "{} EN", entry.id);
        assert_eq!(annex_side("naDe"), entry.status_de == NdpStatus::Verified, "{} NA-DE", entry.id);
        assert_eq!(entry.status == NdpStatus::Verified, entry.status_en == NdpStatus::Verified && entry.status_de == NdpStatus::Verified, "{} joint", entry.id);
        for key in ["en", "naDe"] {
            if json.get("annexStatus").is_some_and(|per| per[key]["status"] == "verified") {
                assert!(json["annexStatus"][key]["evidence"].as_str().is_some_and(|evidence| !evidence.trim().is_empty()), "verified annex {key} of {} has no evidence", entry.id);
            }
        }
        assert!(entry.status == NdpStatus::Unverified || json["evidence"].as_str().is_some_and(|evidence| !evidence.trim().is_empty()), "verified row {} has no evidence", entry.id);
        fn number(value: &serde_json::Value, expected: NdpValue) {
            match value {
                serde_json::Value::Null => assert_eq!(expected, NdpValue::Unset),
                serde_json::Value::Number(n) => assert_eq!(expected, NdpValue::Number(n.as_f64().expect("f64"))),
                serde_json::Value::String(s) => assert!(matches!(expected, NdpValue::Text(text) if text == s.as_str()), "{expected:?} != {s}"),
                other => panic!("bad value {other}"),
            }
        }
        number(&json["recommended"], entry.recommended);
        number(&json["naDe"], entry.na_de);
    }
}

fn pass_with(ids: &[&str]) -> CheckResult {
    let mut builder = CheckResult::assess("ndp", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .utilization(dimensionless(0.5), dimensionless(1.0));
    for id in ids {
        builder = builder.ndp(id);
    }
    builder.build()
}

#[semio_framework_async_macros::async_test]
async fn unverified_ndp_blocks_a_compliant_verdict() {
    let id = NDP_TABLE[0].id;
    let clean = pass_with(&[]);
    assert_eq!(clean.status, CheckStatus::Pass);
    assert!(clean.ndps.is_empty() && !clean.unverified_basis());
    let blocked = pass_with(&[id, id]);
    assert_eq!(blocked.status, CheckStatus::UnverifiedBasis);
    assert_eq!(blocked.ndps, vec![id.to_owned()]);
    assert!(blocked.unverified_basis());
    assert!(blocked.explanation.en.contains(id) && blocked.explanation.de.contains(id));
    assert!((blocked.utilization - 0.5).abs() < 1e-12);
    let mut report = CheckReport::default();
    report.push(clean);
    assert!(report.complies());
    report.push(blocked);
    assert!(!report.complies());
    assert_eq!(report.summary.unverified, 1);
    assert_eq!(report.summary.parts[0].unverified, 1);
    assert!(!report.summary.parts[0].complies);
    assert_eq!(report.unverified_ndps(), vec![id]);
    assert_eq!(report.unverified().count(), 1);
}

#[semio_framework_async_macros::async_test]
async fn a_failure_stays_a_failure_on_an_unverified_basis() {
    let check = CheckResult::assess("f", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .utilization(dimensionless(2.0), dimensionless(1.0))
        .remedy(remedy_for(subject()))
        .ndp(NDP_TABLE[0].id)
        .build();
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.unverified_basis());
}

#[semio_framework_async_macros::async_test]
async fn annex_numbers_come_from_the_registry_are_logged_and_switch_with_the_annex() {
    let _ = drain_ndp_reads();
    assert_eq!(ndp_number("ndp.en1993.gamma-m1", AnnexChoice::En), 1.0);
    assert_eq!(ndp_number("ndp.en1993.gamma-m1", AnnexChoice::De), 1.1);
    assert_eq!(ndp_common("ndp.en1993.gamma-m2"), 1.25);
    assert!(ndp_number_opt("ndp.does.not.exist", AnnexChoice::En).is_none());
    assert_eq!(drain_ndp_reads(), vec!["ndp.en1993.gamma-m1", "ndp.en1993.gamma-m2"]);
    assert!(drain_ndp_reads().is_empty());
    let (_, inner) = record_ndp_reads(|| ndp_number("ndp.en1993.gamma-m0", AnnexChoice::De));
    assert_eq!(inner, vec!["ndp.en1993.gamma-m0"]);
    let _ = ndp_number("ndp.en1993.gamma-m3", AnnexChoice::En);
    let read = CheckResult::assess("r", "Part", ClauseId::new("Z", "1", "1"), subject(), title()).utilization(dimensionless(0.5), dimensionless(1.0)).ndps(&["ndp.en1993.gamma-m1"]).ndp_reads().build();
    assert_eq!(read.ndps, vec!["ndp.en1993.gamma-m1".to_owned(), "ndp.en1993.gamma-m3".to_owned()]);
    assert_eq!(read.status, CheckStatus::UnverifiedBasis);
}

#[semio_framework_async_macros::async_test]
async fn a_read_without_a_value_for_the_annex_is_refused_with_a_localized_notice_and_never_defaulted() {
    let entry = NDP_TABLE.iter().find(|e| e.recommended == NdpValue::Unset && matches!(e.na_de, NdpValue::Number(_))).expect("a DE-only row");
    let _ = drain_ndp_gaps();
    let gap = ndp_value(entry.id, AnnexChoice::En).unwrap_err();
    assert_eq!(gap, NdpGap { id: entry.id.to_owned(), annex: AnnexChoice::En, reason: NdpGapReason::NoValue });
    assert_eq!(gap.notice(), LocalizedCopy::new(format!("NDP {} has no value for annex EN", entry.id), format!("NDP {} hat keinen Wert für den Anhang EN", entry.id)));
    assert!(ndp_value(entry.id, AnnexChoice::De).is_ok());
    assert_eq!(ndp_value("ndp.does.not.exist", AnnexChoice::En).unwrap_err().reason, NdpGapReason::Unregistered);
    assert!(drain_ndp_gaps().is_empty(), "the typed read leaves the caller in charge and logs no gap");
    assert!(ndp_number(entry.id, AnnexChoice::En).is_nan());
    assert!(ndp_number("ndp.does.not.exist", AnnexChoice::De).is_nan());
    assert!(ndp_common(entry.id).is_nan());
    assert!(ndp_number_or_gap(entry.id, AnnexChoice::En).is_none() && ndp_number_or_gap(entry.id, AnnexChoice::De).is_some());
    assert!(ndp_number_opt(entry.id, AnnexChoice::En).is_none());
    let gaps = drain_ndp_gaps();
    assert_eq!(gaps.iter().map(|g| g.reason).collect::<Vec<_>>(), vec![NdpGapReason::NoValue, NdpGapReason::Unregistered]);
    assert!(drain_ndp_gaps().is_empty());
    let text = NDP_TABLE.iter().find(|e| matches!(e.recommended, NdpValue::Text(_))).expect("a text row");
    assert_eq!(ndp_value(text.id, AnnexChoice::En).unwrap_err().reason, NdpGapReason::NotNumeric);
    let divergent = NDP_TABLE.iter().find(|e| matches!((e.recommended, e.na_de), (NdpValue::Number(a), NdpValue::Number(b)) if a != b)).expect("a differing row");
    assert!(ndp_common(divergent.id).is_nan());
    assert_eq!(drain_ndp_gaps()[0].reason, NdpGapReason::Divergent);
    let _ = ndp_number(entry.id, AnnexChoice::En);
    let refused = CheckResult::assess("g", "Part", ClauseId::new("Z", "1", "1"), subject(), title()).utilization(dimensionless(0.5), dimensionless(1.0)).build();
    assert_eq!(refused.status, CheckStatus::UnverifiedBasis);
    assert!(refused.explanation.en.contains(&format!("NDP {} has no value for annex EN", entry.id)));
    assert!(refused.explanation.de.contains(&format!("NDP {} hat keinen Wert für den Anhang EN", entry.id)));
    assert_eq!(refused.ndps, vec![entry.id.to_owned()]);
    let _ = ndp_number(entry.id, AnnexChoice::En);
    let failing = CheckResult::assess("h", "Part", ClauseId::new("Z", "1", "1"), subject(), title()).utilization(dimensionless(2.0), dimensionless(1.0)).remedy(remedy_for(subject())).build();
    assert_eq!(failing.status, CheckStatus::UnverifiedBasis, "a failure computed from a missing value is refused too");
    let clean = CheckResult::assess("i", "Part", ClauseId::new("Z", "1", "1"), subject(), title()).utilization(dimensionless(0.5), dimensionless(1.0)).build();
    assert_eq!(clean.status, CheckStatus::Pass, "the refusal is drained with the check that took it");
}

#[semio_framework_async_macros::async_test]
async fn annex_parameter_rows_list_the_values_of_both_annexes_and_flag_the_differences() {
    let rows = annex_parameter_rows("ndp.en1993.gamma-m", AnnexChoice::De);
    let ids: Vec<&str> = rows.iter().map(|row| row.entry.id).collect();
    assert_eq!(ids, vec!["ndp.en1993.gamma-m0", "ndp.en1993.gamma-m1", "ndp.en1993.gamma-m2", "ndp.en1993.gamma-m3", "ndp.en1993.gamma-mf"]);
    let differing: Vec<&str> = rows.iter().filter(|row| row.differs).map(|row| row.entry.id).collect();
    assert_eq!(differing, vec!["ndp.en1993.gamma-m1"]);
    let m1 = &rows[1];
    assert_eq!((m1.active, m1.other), (NdpValue::Number(1.1), NdpValue::Number(1.0)));
    let en = annex_parameter_rows("ndp.en1993.gamma-m1", AnnexChoice::En);
    assert_eq!((en[0].active, en[0].other), (NdpValue::Number(1.0), NdpValue::Number(1.1)));
}

#[semio_framework_async_macros::async_test]
async fn stations_make_the_maximum_the_governing_utilisation() {
    let check = CheckResult::assess("beam", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .sampled(4, |s| (1.2 - (2.0 * s - 1.0).powi(2), vec![dimensionless(s)]))
        .remedy(remedy_for(subject()))
        .build();
    assert_eq!(check.stations.len(), 5);
    assert_eq!(check.stations[2].at, vec![0.5]);
    assert!((check.utilization - 1.2).abs() < 1e-12);
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.stations_consistent());
    assert_eq!(check.max_station_utilization(), Some(check.utilization));
    let plate = CheckResult::assess("soil", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .field("soil-pressure", vec![Station::point(0.0, 0.0, 0.3), Station::point(1.0, 2.0, 0.8).with(Quantity::stress_mpa(0.2))])
        .build();
    assert_eq!((plate.status, plate.station_field.as_str()), (CheckStatus::Pass, "soil-pressure"));
    assert!((plate.utilization - 0.8).abs() < 1e-12 && plate.stations_consistent());
    let bad = CheckResult { stations: vec![Station::along(1.5, 0.1)], ..plate.clone() };
    assert!(!bad.stations_consistent());
    let none = CheckResult::assess("na", "Part", ClauseId::new("Z", "1", "1"), subject(), title()).stations(vec![Station::along(0.5, 0.9)]).not_applicable(LocalizedCopy::new("n/a", "n. a.")).build();
    assert!(none.stations.is_empty() && none.utilization == 0.0);
    let round = CheckResult::from_value(check.to_value()).expect("FromValue");
    assert_eq!(round, check);
}

#[semio_framework_async_macros::async_test]
async fn positive_demand_against_non_positive_capacity_fails_with_a_message() {
    for capacity in [-5.0, 0.0] {
        let check = CheckResult::assess("neg", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
            .utilization(dimensionless(2.0), dimensionless(capacity))
            .remedy(remedy_for(subject()))
            .build();
        assert_eq!(check.status, CheckStatus::Fail);
        assert_eq!(check.utilization, UTILIZATION_CAP);
        assert!(check.explanation.en.contains("non-positive capacity") && check.explanation.de.contains("nicht positivem Widerstand"));
    }
    let idle = CheckResult::assess("neg0", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .utilization(dimensionless(-1.0), dimensionless(-5.0))
        .build();
    assert_eq!(idle.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn informative_rows_are_info_not_failures() {
    let info = CheckResult::assess("info", "Part", ClauseId::new("Z", "1", "1"), subject(), title())
        .informative(dimensionless(42.0))
        .build();
    assert_eq!(info.status, CheckStatus::Info);
    assert_eq!(info.utilization, 0.0);
    let mut report = CheckReport::default();
    report.push(info);
    report.push(minimum_check(2.0, 1.0));
    assert!(report.complies());
    assert_eq!(report.summary.info, 1);
    assert_eq!(report.summary.parts[0].info, 1);
    assert!((report.worst_utilization() - 0.5).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn near_limit_threshold_is_one_minus_the_reserve() {
    assert!((NearLimit::default().threshold() - 0.95).abs() < 1e-12);
    assert!((NearLimit::with_reserve(0.2).threshold() - 0.8).abs() < 1e-12);
    assert_eq!(NearLimit::with_reserve(f64::NAN), NearLimit::default());
    assert_eq!(NearLimit::with_reserve(-1.0).threshold(), 1.0);
    assert_eq!(NearLimit::with_reserve(7.0).threshold(), 0.0);
    assert_eq!(near_limit(), NearLimit::default());
}

#[semio_framework_async_macros::async_test]
async fn a_verified_annex_value_supports_a_verdict_while_the_other_annex_stays_open() {
    let id = "ndp.en1997.r3.a";
    let entry = ndp(id).expect("r3.a");
    assert_eq!((entry.status_en, entry.status_de, entry.status), (NdpStatus::Verified, NdpStatus::Unverified, NdpStatus::Unverified));
    assert!(ndp_is_verified_for(id, AnnexChoice::En) && !ndp_is_verified_for(id, AnnexChoice::De) && !ndp_is_verified(id));
    let verdict = |annex: AnnexChoice| CheckResult::assess("ndp", "Part", ClauseId::new("Z", "1", "1"), subject(), title()).utilization(dimensionless(0.5), dimensionless(1.0)).annex(annex).ndp(id).build();
    assert_eq!(verdict(AnnexChoice::En).status, CheckStatus::Pass);
    assert_eq!(verdict(AnnexChoice::De).status, CheckStatus::UnverifiedBasis);
}

/// 🧪️ The shared action factors read the EN 1990 registry rows: K_FI scales γ_G,sup and γ_Q of persistent situations only, γ_G,inf and ξ stay, exceptional situations and serviceability carry 1.0, ψ follow the category and bridges split γ_Q by traffic family.
#[test]
fn action_factors_follow_the_registry_class_situation_and_domain() {
    use crate::action_factors::{Domain, PsiRow};
    let value = |id: &str, annex: AnnexChoice| ndp_number(id, annex);
    for annex in [AnnexChoice::En, AnnexChoice::De] {
        for class in ReliabilityClass::ALL {
            let factors = ActionFactors::new(annex, class, DesignSituation::Persistent);
            let k_fi = value(&format!("ndp.en1990.kfi.{}", class.number()), annex);
            assert_eq!(factors.k_fi(), k_fi);
            assert_eq!(factors.gamma_g_sup(), k_fi * value("ndp.en1990.gG_sup.str.A1", annex));
            assert_eq!(factors.gamma_g_inf(), value("ndp.en1990.gG_inf.str.A1", annex));
            assert_eq!(factors.gamma_q_building(), k_fi * value("ndp.en1990.gQ.str.A1", annex));
            assert_eq!(factors.gamma_g_sup_610b(), value("ndp.en1990.xi.str.A1", annex) * factors.gamma_g_sup());
        }
        let persistent = ActionFactors::new(annex, ReliabilityClass::RC3, DesignSituation::Persistent);
        for exceptional in [persistent.in_situation(DesignSituation::Accidental), persistent.in_situation(DesignSituation::Seismic), persistent.serviceability()] {
            assert_eq!((exceptional.gamma_g_sup(), exceptional.gamma_g_inf(), exceptional.gamma_q_building(), exceptional.xi()), (1.0, 1.0, 1.0, 1.0));
        }
        let bridge = persistent.domain(Domain::Bridge);
        assert_eq!(bridge.gamma_q("road_traffic"), persistent.k_fi() * value("ndp.en1990.gQ.road.str.A2", annex));
        assert_eq!(bridge.gamma_q("rail_traffic"), persistent.k_fi() * value("ndp.en1990.gQ.rail.str.A2", annex));
        assert_eq!(bridge.gamma_q("wind_bridge"), persistent.k_fi() * value("ndp.en1990.gQ.other.str.A2", annex));
        let office = persistent.psi("B");
        assert_eq!((office.psi_0, office.psi_1, office.psi_2), (value("ndp.en1990.psi.office.0", annex), value("ndp.en1990.psi.office.1", annex), value("ndp.en1990.psi.office.2", annex)));
        assert_eq!(persistent.psi("A"), persistent.psi("residential"));
        assert_eq!(persistent.psi("not-a-category"), PsiRow::UNITY);
        assert!(persistent.psi_known("not-a-category").is_none());
    }
    assert_eq!(ReliabilityClass::from_number(3), Some(ReliabilityClass::RC3));
    assert_eq!(ReliabilityClass::from_number(0), None);
}

#[semio_framework_async_macros::async_test]
async fn utilization_boundary_cases_from_the_shared_fixture() {
    #[derive(serde::Deserialize)]
    struct Case {
        id: String,
        utilization: f64,
        status: String,
    }
    #[derive(serde::Deserialize)]
    struct Cases {
        tolerance: f64,
        cases: Vec<Case>,
    }
    let fixture: Cases = serde_json::from_str(include_str!("../🎚️utilization-boundary/🔣️.json")).unwrap();
    assert_eq!(fixture.tolerance, UTILIZATION_TOLERANCE);
    for case in fixture.cases {
        let expected = if case.status == "pass" { CheckStatus::Pass } else { CheckStatus::Fail };
        assert_eq!(limit_status(case.utilization), expected, "{}", case.id);
        let built = CheckResult::assess(&case.id, "EN 1990", ClauseId::new("EN 1990", "1", "6.4"), subject(), title())
            .utilization(Quantity::force_kn(case.utilization * 100.0), Quantity::force_kn(100.0))
            .annex(AnnexChoice::En)
            .explanation(LocalizedCopy::new("u <= 1", "u <= 1"))
            .remedy(remedy_for(subject()))
            .build();
        assert_eq!(built.status, expected, "{}", case.id);
    }
}

fn failing_builder(id: &str) -> CheckBuilder {
    CheckResult::assess(id, "DIN EN 1992-1-1", ClauseId::new("EN 1992", "1-1", "6.2.2"), subject(), title())
        .utilization(Quantity::force_kn(120.0), Quantity::force_kn(100.0))
        .annex(AnnexChoice::De)
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_try_build_rejects_a_failing_check_without_remedy() {
    let error = failing_builder("en1992.law.missing").try_build().expect_err("a failing check without a remedy is rejected");
    assert_eq!(error, MissingRemedy { check_id: "en1992.law.missing".to_owned() });
    assert!(error.to_string().contains("en1992.law.missing"));
    let panicked = std::panic::catch_unwind(|| failing_builder("en1992.law.panic").build());
    assert!(panicked.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_build_never_panics_and_attaches_the_missing_remedy_diagnostic() {
    let check = failing_builder("en1992.law.diagnostic").build();
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.remedies.is_empty());
    assert!(check.explanation.en.contains("No remedy defined for this failing check"));
    assert!(check.explanation.de.contains("Für diese nicht erfüllte Nachweisführung ist keine Abhilfe hinterlegt"));
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_accepts_a_remedy_or_a_named_post_pass() {
    let with_remedy = failing_builder("en1992.law.remedy").remedy(remedy_for(subject())).try_build().expect("remedy attached");
    assert_eq!(with_remedy.status, CheckStatus::Fail);
    let deferred = failing_builder("en1992.law.deferred").remedies_attached_by("attach_minimal_remedies").try_build().expect("post-pass named");
    assert!(deferred.remedies.is_empty());
    let passing = CheckResult::assess("en1992.law.pass", "DIN EN 1992-1-1", ClauseId::new("EN 1992", "1-1", "6.2.2"), subject(), title())
        .utilization(Quantity::force_kn(80.0), Quantity::force_kn(100.0))
        .try_build()
        .expect("a passing check needs no remedy");
    assert_eq!(passing.status, CheckStatus::Pass);
}
