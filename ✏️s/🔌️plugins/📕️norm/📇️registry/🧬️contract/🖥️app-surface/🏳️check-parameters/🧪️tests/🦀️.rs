//! 🧪️ Check-parameter tests; scenarios in `🥒️.feature`. The independent oracle is the registry table itself (`ndp(id)` values, symbols, status) read without the section code.

use super::*;
use semio_s_plugin_norm_compliance::{ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};

fn text(value: &str) -> DslValue {
    DslValue::String(value.into())
}

fn row(id: &str, factor: &str, target: &str) -> DslValue {
    DslValue::object([("id".into(), text(id)), ("factor".into(), text(factor)), ("target".into(), text(target)), ("value".into(), DslValue::float(0.4))])
}

fn document() -> DslValue {
    DslValue::object([("factorOverrides".into(), DslValue::Array(vec![row("FO1", "psi1", "office"), row("FO2", "gamma", ""), row("FO3", "psi1", "office")]))])
}

fn check(id: &str, entity: &str, annex: AnnexChoice, ndps: &[&str]) -> CheckResult {
    CheckResult::assess(id, "ULS", ClauseId::new("EN 1990", "A1", "A1.3"), SubjectRef::new(entity, format!("factorOverrides[id={entity}].value"), LocalizedCopy::new(entity, entity)), LocalizedCopy::new(id, id))
        .utilization(Quantity::new(QuantityKind::Length, 0.9), Quantity::new(QuantityKind::Length, 1.0))
        .explanation(LocalizedCopy::new("explained", "erläutert"))
        .annex(annex)
        .ndps(ndps)
        .build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    report.push(check("e", "B1", AnnexChoice::De, &["ndp.en1990.psi.office.1", "ndp.en1990.gP_inf", "ndp.en1990.psi.office.0"]));
    report.push(check("f", "FO3", AnnexChoice::En, &[]));
    report.push(check("g", "FO2", AnnexChoice::En, &[]));
    report
}

#[test]
fn every_cited_parameter_is_listed_with_registry_value_clause_and_badge() {
    let report = report();
    let parameters = check_parameters(&report.checks[0], &document(), Locale::En);
    assert_eq!(parameters.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), report.checks[0].ndps.iter().map(String::as_str).collect::<Vec<_>>());
    for parameter in &parameters {
        let entry = ndp(&parameter.id).expect("registered");
        assert_eq!((parameter.symbol.as_str(), parameter.clause.as_str(), parameter.verified, parameter.annex), (entry.symbol, entry.clause, entry.status == NdpStatus::Verified, AnnexChoice::De));
        assert_eq!(parameter.value, value_text(entry.value(AnnexChoice::De), entry.unit, Locale::En));
    }
    assert_eq!(parameters[0].symbol, "ψ1 office");
    assert!(check_parameter_label(&parameters[0], Locale::En).ends_with("[unverified]") && check_parameter_label(&parameters[0], Locale::De).ends_with("[ungeprüft]"));
    let unknown = CheckResult { ndps: vec!["ndp.nobody.unknown".into()], ..report.checks[0].clone() };
    let listed = check_parameters(&unknown, &document(), Locale::En);
    assert_eq!((listed[0].verified, listed[0].value.as_str(), listed[0].override_row.as_deref()), (false, "—", None), "an unknown id is shown as unverified, never hidden");
}

#[test]
fn a_parameter_links_to_the_last_factor_override_that_replaces_it() {
    let parameters = check_parameters(&report().checks[0], &document(), Locale::En);
    assert_eq!(parameters[0].override_row.as_deref(), Some("FO3"), "psi1 of the office category: the later of two rows wins");
    assert_eq!(parameters[1].override_row.as_deref(), Some("FO2"), "a gamma symbol is replaced by the gamma row without target");
    assert_eq!(parameters[2].override_row, None, "no psi0 row exists");
    assert_eq!(override_row_for("ndp.en1990.psi.office.1", "ψ1 office", &DslValue::object([])), None);
}

#[test]
fn the_section_renders_one_row_per_parameter_and_jumps_to_the_governing_check_of_the_override() {
    let report = report();
    let render = |locale| {
        let node = check_parameters_section(&report, 0, &document(), locale, "s.norm.test@1/*#editor").expect("section").expect("some");
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(node)).expect("projection")
    };
    let (en, de) = (render(Locale::En), render(Locale::De));
    assert!(en.contains("Check parameters (3)") && de.contains("Nachweisparameter (3)"));
    for id in &report.checks[0].ndps {
        assert!(en.contains(id.as_str()) && de.contains(id.as_str()), "{id}");
    }
    assert!(en.contains("setSelectedCheckIndex") && en.contains("Annex DE-NA"));
    assert!(check_parameters_section(&report, 1, &document(), Locale::En, "s.norm.test@1/*#editor").expect("section").is_none(), "a check without parameters has no section");
    assert!(check_parameters_section(&report, 9, &document(), Locale::En, "s.norm.test@1/*#editor").expect("section").is_none());
}
