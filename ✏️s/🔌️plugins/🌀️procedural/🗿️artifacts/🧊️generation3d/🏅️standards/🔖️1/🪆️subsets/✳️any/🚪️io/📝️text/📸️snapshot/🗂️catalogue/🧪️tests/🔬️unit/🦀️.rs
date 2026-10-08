use super::*;
use semio_framework_value::{FromValue, ToValue};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../🧬️schema/🗂️catalogue/🧫️fixtures/🔣️.json")).expect("catalogue fixture parses")
}

fn catalogue_folder() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue")
}

fn step<'a>(node: &'a mut Value, key: &Value) -> &'a mut Value {
    match key {
        Value::String(name) => node.get_mut(name.as_str()).expect("patch path member exists"),
        Value::Number(index) => node.get_mut(index.as_u64().expect("patch index") as usize).expect("patch path item exists"),
        other => panic!("patch key {other}"),
    }
}

fn assign(node: &mut Value, key: &Value, value: Value) {
    match (node, key) {
        (Value::Object(entries), Value::String(name)) => {
            entries.insert(name.clone(), value);
        }
        (Value::Array(items), Value::Number(index)) => items[index.as_u64().expect("patch index") as usize] = value,
        (node, key) => panic!("cannot assign {key} in {node}"),
    }
}

fn patched(base: &Value, patches: &[Value]) -> CategoryFile {
    let mut copy = base.clone();
    for patch in patches {
        let path = patch["path"].as_array().expect("patch path");
        let (last, parents) = path.split_last().expect("patch path is not empty");
        let mut node = &mut copy;
        for key in parents {
            node = step(node, key);
        }
        if patch.get("delete").is_some() {
            node.as_object_mut().expect("deleting from an object").remove(last.as_str().expect("deleted key"));
        } else {
            assign(node, last, patch["value"].clone());
        }
    }
    from_json_str(&serde_json::to_string(&copy).expect("patched fixture serializes"), JsonMemberPolicy::Reject).expect("patched fixture decodes")
}

fn triples(findings: &[Finding]) -> Vec<(String, String, Option<String>)> {
    let mut triples: Vec<_> = findings.iter().map(|finding| (finding.code.to_string(), finding.owner.clone(), finding.port.clone())).collect();
    triples.sort();
    triples
}

fn expected_triples(expected: &Value) -> Vec<(String, String, Option<String>)> {
    let mut triples: Vec<_> = expected.as_array().expect("expected findings").iter().map(|finding| (finding["code"].as_str().unwrap().to_string(), finding["owner"].as_str().unwrap().to_string(), finding["port"].as_str().map(str::to_string))).collect();
    triples.sort();
    triples
}

#[test]
fn catalogue_bundle_parses_and_has_no_findings() {
    let bundled = bundled_catalogue().expect("the bundled catalogue parses");
    assert_eq!(bundled.findings(), Vec::new());
    assert!(bundled.kinds().count() >= fixture()["minimumKinds"].as_u64().unwrap() as usize);
    assert_eq!(catalogue().kinds().count(), bundled.kinds().count());
}

#[test]
fn catalogue_loader_agrees_with_an_independent_json_parser() {
    for (slug, text) in CATEGORY_SOURCES {
        let oracle: Value = serde_json::from_str(text).unwrap_or_else(|error| panic!("{slug}: {error}"));
        let file: CategoryFile = from_json_str(text, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{slug}: {error}"));
        assert_eq!(oracle["category"]["id"].as_str(), Some(file.category.id.as_str()));
        let kinds = oracle["kinds"].as_array().unwrap();
        assert_eq!(kinds.len(), file.kinds.len(), "{slug}");
        for (expected, actual) in kinds.iter().zip(&file.kinds) {
            assert_eq!(expected["id"].as_str(), Some(actual.id.as_str()));
            assert_eq!(expected["label"]["de"].as_str(), Some(actual.label.de.as_str()));
            assert_eq!(expected["inputs"].as_array().unwrap().len(), actual.inputs.len(), "{}", actual.id);
            assert_eq!(expected["outputs"].as_array().unwrap().len(), actual.outputs.len(), "{}", actual.id);
            assert_eq!(expected["preview"].as_bool(), Some(actual.preview));
        }
    }
}

#[test]
fn catalogue_bundle_lists_every_category_file_of_the_folder() {
    let mut on_disk: Vec<String> = std::fs::read_dir(catalogue_folder()).unwrap().filter_map(|entry| entry.ok()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("🔣️") && name != "🔣️.json" && name.ends_with(".json")).collect();
    on_disk.sort();
    let mut bundled: Vec<String> = CATEGORY_SOURCES.iter().map(|(slug, _)| format!("🔣️{slug}.json")).collect();
    bundled.sort();
    assert_eq!(on_disk, bundled);
    for (slug, text) in CATEGORY_SOURCES {
        let file: CategoryFile = from_json_str(text, JsonMemberPolicy::Reject).unwrap();
        assert_eq!(file.category.id.replace('.', "-"), slug);
    }
}

#[test]
fn catalogue_finding_laws_hold_for_every_mutated_copy() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let mut files = vec![patched(&fixture["base"], case["patches"].as_array().unwrap())];
        if let Some(append) = case.get("appendFile") {
            files.push(patched(&fixture["base"], append.as_array().unwrap()));
        }
        assert_eq!(triples(&check(&files)), expected_triples(&case["expected"]), "{}", case["name"]);
    }
}

#[test]
fn catalogue_covers_every_exposed_kernel_method_and_mesh_operation_once() {
    let fixture = fixture();
    let bundled = catalogue();
    let mut claimed = std::collections::BTreeSet::new();
    for entry in fixture["kernelCoverage"].as_array().unwrap().iter().chain(fixture["meshCoverage"].as_array().unwrap()) {
        let kind = entry["kind"].as_str().unwrap();
        assert!(bundled.kind(kind).is_some(), "{entry} has no kind");
        assert!(claimed.insert(kind), "{kind} is claimed twice");
    }
    assert_eq!(fixture["kernelCoverage"].as_array().unwrap().len(), 93);
    assert_eq!(fixture["meshCoverage"].as_array().unwrap().len(), 42);
    assert_eq!(fixture["unexposedKernelMethods"].as_array().unwrap().len(), 7);
}

#[test]
fn catalogue_kinds_and_ports_are_found_by_id_and_name() {
    let fixture = fixture();
    let bundled = catalogue();
    for lookup in fixture["lookups"].as_array().unwrap() {
        let kind = bundled.kind(lookup["kind"].as_str().unwrap()).expect("looked-up kind exists");
        assert_eq!(kind.label.en, lookup["label"]["en"].as_str().unwrap());
        assert_eq!(kind.label.de, lookup["label"]["de"].as_str().unwrap());
        assert_eq!(kind.to_value().get("quality").and_then(|quality| quality.as_str()), lookup["quality"].as_str());
        assert_eq!(kind.inputs.iter().map(|port| port.name.as_str()).collect::<Vec<_>>(), lookup["inputs"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect::<Vec<_>>());
        assert_eq!(kind.outputs.iter().map(|port| port.name.as_str()).collect::<Vec<_>>(), lookup["outputs"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect::<Vec<_>>());
        for (name, value) in lookup["defaults"].as_object().unwrap() {
            let default = bundled.port(&kind.id, name).and_then(|port| port.default.as_ref()).and_then(|default| default.as_f64());
            assert_eq!(default, value.as_f64(), "{}.{name}", kind.id);
        }
    }
    assert!(bundled.kind("brep.nowhere.nothing").is_none());
    assert!(bundled.port("brep.primitive.box", "nothing").is_none());
    assert_eq!(bundled.category("brep.primitive").map(|file| file.kinds.len()), Some(6));
    let loop_cut = bundled.kind("mesh.edit.loopCut").unwrap();
    assert_eq!(loop_cut.input("edges").and_then(|port| port.selection.as_ref()).map(|selection| (selection.component, selection.source.as_str())), Some((SelectionComponent::Edge, "mesh")));
    assert_eq!(loop_cut.picks().len(), 2);
    assert_eq!(bundled.kind("brep.transform.rotate").unwrap().gumballs()[0].axis_port.as_deref(), Some("axis"));
}

#[test]
fn catalogue_texts_resolve_per_locale_with_english_as_the_fallback() {
    let label = &catalogue().kind("brep.primitive.box").unwrap().label;
    assert_eq!(label.resolve("de"), "Quader");
    assert_eq!(label.resolve("de-CH"), "Quader");
    assert_eq!(label.resolve("DE_at"), "Quader");
    assert_eq!(label.resolve("en-GB"), "Box");
    assert_eq!(label.resolve("fr"), "Box");
}

#[test]
fn catalogue_values_round_trip_through_the_value_codec() {
    for (_, text) in CATEGORY_SOURCES {
        let file: CategoryFile = from_json_str(text, JsonMemberPolicy::Reject).unwrap();
        assert_eq!(CategoryFile::from_value(file.to_value()).unwrap(), file);
    }
}

#[test]
fn catalogue_refuses_duplicate_ids_unknown_members_and_broken_files() {
    let (slug, text) = CATEGORY_SOURCES[0];
    assert!(parse_catalogue([(slug, text), ("again", text)]).is_err());
    assert!(parse_catalogue([("garbled", "{")]).is_err());
    let mut oracle: Value = serde_json::from_str(text).unwrap();
    oracle["kinds"][0]["surprise"] = Value::Bool(true);
    assert!(parse_catalogue([("unknown", serde_json::to_string(&oracle).unwrap().as_str())]).is_err());
}
