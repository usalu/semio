use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🌐️language/🔣️.json")).unwrap() }

fn vectors() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../../../../🔨️modules/⚙️runtime/🗣️language/🧪️tests/🔣️.json")).unwrap() }

fn id(locale: Option<Locale>) -> serde_json::Value { locale.map_or(serde_json::Value::Null, |locale| locale.as_str().into()) }

#[test]
fn system_values_name_a_supported_language_per_the_shared_vectors() {
    for row in fixture()["negotiate"].as_array().unwrap() {
        assert_eq!(id(negotiate(row["value"].as_str().unwrap())), row["language"], "{row}");
    }
    for pair in vectors()["tags"].as_array().unwrap() {
        assert_eq!(protocol().subtag(pair[0].as_str().unwrap()).as_deref(), pair[1].as_str(), "{pair}");
    }
}

#[test]
fn the_declared_chain_resolves_the_shared_environments() {
    let chain = declared();
    let mut checked = 0;
    for row in vectors()["resolve"].as_array().unwrap().iter().filter(|row| row["args"].as_array().unwrap().is_empty() && row["os"].as_array().unwrap().is_empty()) {
        let environment = row["environment"].as_object().unwrap();
        let lookup = |name: &str| environment.get(name).and_then(|value| value.as_str()).map(String::from);
        assert_eq!(id(chain.resolve(&lookup)), row["expected"]["language"], "{row}");
        checked += 1;
    }
    assert!(checked >= 15, "the shared protocol vectors cover the environment sources");
    assert_eq!(protocol().explicit_variable(), "SEMIO_LANGUAGE");
}

#[test]
fn no_declared_choice_is_selected_when_the_system_names_none() {
    for row in fixture()["choiceOrders"].as_array().unwrap() {
        let chain = Chain::parse(&row["declaration"].to_string()).unwrap();
        assert_eq!(id(chain.resolve(&|_| None)), row["expected"], "{row}");
    }
}

#[test]
fn a_declaration_without_supported_choices_is_refused() {
    for row in fixture()["invalidDeclarations"].as_array().unwrap() { assert!(Chain::parse(&row.to_string()).is_err(), "{row}"); }
    assert!(Chain::parse(DECLARATION).is_ok());
}
