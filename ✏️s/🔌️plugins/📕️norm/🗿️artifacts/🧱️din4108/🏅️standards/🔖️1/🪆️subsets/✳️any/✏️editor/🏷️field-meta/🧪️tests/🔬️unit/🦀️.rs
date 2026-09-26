use super::*;
use crate::Din4108Snapshot;
use dsl::ToValue;

fn collect_leaf_paths(value: &dsl::DslValue, prefix: &str, out: &mut Vec<String>) {
    match value {
        dsl::DslValue::Object(map) => {
            for (k, v) in map {
                let p = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                match v {
                    dsl::DslValue::Object(_) | dsl::DslValue::Array(_) => collect_leaf_paths(v, &p, out),
                    _ => out.push(p),
                }
            }
        }
        dsl::DslValue::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                let p = format!("{prefix}[{i}]");
                match item {
                    dsl::DslValue::Object(_) | dsl::DslValue::Array(_) => collect_leaf_paths(item, &p, out),
                    _ => out.push(p),
                }
            }
        }
        _ => out.push(prefix.to_string()),
    }
}

#[test]
fn field_meta_covers_every_editable_leaf_on_default_snapshot() {
    let snap = Din4108Snapshot::default();
    let tree = snap.to_value();
    let mut paths = Vec::new();
    collect_leaf_paths(&tree, "", &mut paths);
    assert!(!paths.is_empty());
    for path in &paths {
        let meta = din4108_field_meta(path).unwrap_or_else(|| panic!("missing field meta for {path}"));
        assert!(!meta.label_en.is_empty(), "{path} en");
        assert!(!meta.label_de.is_empty(), "{path} de");
        assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{path}");
        if let Some(choices) = meta.choices {
            for c in choices {
                assert_ne!(c.label_en, c.value, "{path} choice {} must be human en", c.value);
                assert_ne!(c.label_de, c.value, "{path} choice {} must be human de", c.value);
            }
        }
    }
}

#[test]
fn climate_and_usage_choices_are_human_localized() {
    let climate = din4108_field_meta("climateZone").expect("climate");
    let c0 = &climate.choices.unwrap()[0];
    assert_eq!(c0.label_en, "Summer climate region A");
    assert_eq!(c0.label_de, "Sommerklimaregion A");
    let usage = din4108_field_meta("usage").expect("usage");
    let u0 = &usage.choices.unwrap()[0];
    assert_eq!(u0.label_en, "Residential");
    assert_eq!(u0.label_de, "Wohngebäude");
    let heavy = din4108_field_meta("zones[0].heaviness").expect("heaviness");
    let h0 = &heavy.choices.unwrap()[0];
    assert_eq!(h0.label_en, "Heavy construction");
    assert_eq!(h0.label_de, "Schwere Bauweise");
    let yes = din4108_field_meta("hasMechanicalVentilation").expect("bool");
    assert_eq!(yes.choices.unwrap()[0].label_en, "Yes");
    assert_eq!(yes.choices.unwrap()[0].label_de, "Ja");
}
