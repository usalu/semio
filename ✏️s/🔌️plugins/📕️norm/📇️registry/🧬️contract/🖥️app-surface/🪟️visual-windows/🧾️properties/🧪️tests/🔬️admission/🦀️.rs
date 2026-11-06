//! 🛡️ Rendered scalar admission follows the shared EN/DE fixture without exposing unsupported mutations.

use crate::*;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_compliance::{CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};

const LAW: &str = include_str!("../../../../../../../🧪️tests/🧫️fixtures/🛡️inspector-admission/🔣️.json");
const CHOICES: &[NormFieldChoice] = &[NormFieldChoice { value: "beam", label_en: "Beam", label_de: "Träger" }];
const CLAUSE: ClauseRef = ClauseRef { family: "EN 1990", part: "A2", section: "A2.4" };

fn metadata(path: &str) -> Option<FieldSpec> {
    let key = path.rsplit('.').next().unwrap_or(path);
    let spec = match key {
        "identifier" => FieldSpec::new("Identifier", "Kennung"),
        "quantity" => FieldSpec { ndp: Some(NdpRef { id: Some("ndp.en1990.fixture"), symbol: "γ", clause: CLAUSE, default: Some(0.002) }), ..FieldSpec::quantity("Length", "Länge", UnitId::Metre) },
        "integer" => FieldSpec { kind: FieldKind::Int, ..FieldSpec::new("Count", "Anzahl") },
        "enabled" => FieldSpec { kind: FieldKind::Bool, ..FieldSpec::new("Enabled", "Aktiv") },
        "choice" => FieldSpec::choice("Kind", "Art", CHOICES),
        "missing" => FieldSpec::new("Optional value", "Optionaler Wert"),
        "editable" => FieldSpec::quantity("Editable length", "Bearbeitbare Länge", UnitId::Metre),
        _ => return None,
    };
    Some(FieldSpec { read_only: key != "editable" || path.contains("[id=b2]"), ..spec })
}

fn value(source: &serde_json::Value) -> DslValue {
    match source {
        serde_json::Value::String(text) => DslValue::String(text.clone()),
        serde_json::Value::Bool(on) => DslValue::Bool(*on),
        serde_json::Value::Number(number) => DslValue::float(number.as_f64().expect("number")),
        serde_json::Value::Null => DslValue::Null,
        _ => panic!("scalar fixture"),
    }
}

fn projection(node: BuiltNode) -> String {
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(node)).expect("rendered UI projection")
}

fn readonly_report() -> CheckReport {
    let subject = SubjectRef::new("readonly-quantity", "quantity", LocalizedCopy::new("Length", "Länge"));
    let mut report = CheckReport::default();
    report.push(CheckResult::assess("readonly.quantity", "EN 1990", ClauseId::new("EN 1990", "A2", "A2.4"), subject.clone(), LocalizedCopy::new("Minimum length", "Mindestlänge"))
        .minimum(Quantity::length_m(0.0015), Quantity::length_m(0.002))
        .remedy(Remedy::at_least(subject, Quantity::length_m(0.0015), Quantity::length_m(0.002), LocalizedCopy::new("Increase length", "Länge erhöhen")))
        .build());
    report
}

#[test]
fn readonly_scalars_keep_accessible_localized_values_and_have_no_mutation_affordances() {
    let law: serde_json::Value = serde_json::from_str(LAW).expect("neutral admission fixture");
    let rows = law["fields"].as_array().expect("fields").iter().filter(|row| row["editable"] == false).collect::<Vec<_>>();
    let tree = DslValue::object(rows.iter().map(|row| (row["path"].as_str().unwrap().to_owned(), value(&row["value"]))));
    for (locale, tag) in [(Locale::En, "en"), (Locale::De, "de")] {
        let model = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let windows = semio_plugin::TreeWindows::for_body(&model, "norm.admission.inputs");
        let report = readonly_report();
        let node = render_inspector_value(&tree, &report, locale, "s.norm.en1990@1/*#editor", Some(metadata), &windows, &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert)).expect("read-only inspector");
        let json = projection(node);
        let copy = law["locales"].as_array().unwrap().iter().find(|copy| copy["id"] == tag).unwrap();
        assert!(json.contains(copy["readOnly"].as_str().unwrap()), "{json}");
        for row in &rows {
            assert!(json.contains(row["text"][tag].as_str().unwrap()), "{tag}: {row}: {json}");
            assert!(json.contains(&format!("norm-inputs.{}", row["path"].as_str().unwrap())), "{json}");
        }
        for action in ["setField", "setFields", "applyRemedy", "previewRemedy", "clearPreview", ".ndp-reset", ".input", ".stepper", ".toggle", ".select", ".slider"] {
            assert!(!json.contains(action), "read-only fields offer no {action}: {json}");
        }
    }
}

#[test]
fn admitted_scalar_controls_keep_their_original_typed_commit() {
    let scalar = DslValue::float(0.0015);
    for locale in [Locale::En, Locale::De] {
        let field = field_view("editable", &scalar, &scalar, locale, Some(metadata), UnitSystem::Engineering);
        assert!(!field.read_only);
        let controls = field_controls(&field, &scalar, "norm-inputs.editable", "norm.mock", locale).expect("admitted controls");
        assert!(controls.into_iter().any(|control| projection(control).contains("setField")));
    }
}

#[test]
fn bulk_commit_requires_admission_for_every_selected_row() {
    let row = |id: &str, amount: f64| DslValue::object([("id".into(), DslValue::String(id.into())), ("editable".into(), DslValue::float(amount))]);
    let tree = DslValue::object([("items".into(), DslValue::Array(vec![row("b1", 0.0015), row("b2", 0.0025), row("b3", 0.0015)]))]);
    for locale in [Locale::En, Locale::De] {
        let blocked = projection(render_bulk_inspector(&tree, "items", &["b1".into(), "b2".into()], locale, "norm.mock", Some(metadata), UnitSystem::Engineering).expect("read-only bulk"));
        assert!(!blocked.contains("setFields"), "{blocked}");
        assert!(blocked.contains(if locale == Locale::En { "Read only" } else { "Schreibgeschützt" }), "{blocked}");
        let admitted = projection(render_bulk_inspector(&tree, "items", &["b1".into(), "b3".into()], locale, "norm.mock", Some(metadata), UnitSystem::Engineering).expect("admitted bulk"));
        assert!(admitted.contains("setFields"), "{admitted}");
    }
}
