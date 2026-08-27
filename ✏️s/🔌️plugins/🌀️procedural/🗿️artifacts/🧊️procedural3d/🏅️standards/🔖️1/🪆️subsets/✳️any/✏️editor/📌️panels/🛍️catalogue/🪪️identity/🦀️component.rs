//! 🪪️ Semantic identities for procedural catalogue entries sharing a widget family.

//#region 🔖️Identity
pub fn item_key(kind: &str, neuron_kind: Option<&str>, format: Option<&str>, action: Option<&str>) -> String {
    let variant = match kind {
        "neuron" => neuron_kind,
        "outputExport" => format,
        "outputAction" => action,
        _ => None,
    };
    match variant {
        Some(variant) => format!("procedural-play-catalogue.{kind}.{variant}"),
        None => format!("procedural-play-catalogue.{kind}"),
    }
}
//#endregion 🔖️Identity

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_authored_export_format_has_a_distinct_semantic_key() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️catalogue-identities.json")).unwrap();
        let cases = fixture.as_array().unwrap();
        let keys: Vec<_> = cases.iter().map(|case| item_key(case["kind"].as_str().unwrap(), case["neuronKind"].as_str(), case["format"].as_str(), case["action"].as_str())).collect();
        assert_eq!(keys.iter().collect::<std::collections::BTreeSet<_>>().len(), cases.len());
        for (case, key) in cases.iter().zip(keys) {
            assert_eq!(serde_json::to_value(key).unwrap(), case["key"]);
        }
    }
}
//#endregion 🧪️Tests
