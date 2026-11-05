//! 🪆️ Language-neutral descendant writes checked against an independent JSON Pointer oracle.

use super::*;
use semio_framework_value::DslValue;

const PARENTS: [SetFieldRule; 4] = [
    SetFieldRule { path: "partNumberRule", kind: "replace-part-number-rule", selectors: &[], value: "newRule" },
    SetFieldRule { path: "catalog", kind: "replace-catalogue", selectors: &[], value: "newCatalog" },
    SetFieldRule { path: "catalog.products[].configuration", kind: "change-product-configuration", selectors: &[SelectorKey::Id("id")], value: "newConfiguration" },
    SetFieldRule { path: "catalog.products[].configuration", kind: "clear-product-configuration", selectors: &[SelectorKey::Id("id")], value: "" },
];
const EXACT: [SetFieldRule; 5] = [PARENTS[0], PARENTS[1], PARENTS[2], PARENTS[3], SetFieldRule { path: "catalog.products[].configuration.attributes.heatExponentN", kind: "change-heat-exponent", selectors: &[SelectorKey::Id("id")], value: "newExponent" }];
const CLEAR: [SetFieldRule; 5] = [PARENTS[0], PARENTS[1], PARENTS[2], PARENTS[3], SetFieldRule { path: "catalog.products[].configuration.geometryRef", kind: "clear-geometry-reference", selectors: &[SelectorKey::Id("id")], value: "" }];

#[test]
fn inspector_editability_follows_the_declared_typed_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json")).expect("language-neutral cases");
    const BOUNDARIES: [SetFieldRule; 2] = [PARENTS[0], PARENTS[2]];
    let rules = NormEditRules { set_field_arguments: &[], set_field: &BOUNDARIES, insert_item: &[], remove_item: &[], move_item: &[] };
    for (group, expected) in [("editable", true), ("readonly", false)] {
        for path in fixture["access"][group].as_array().expect("access cases") {
            let path = path.as_str().expect("path");
            assert_eq!(rules.can_set_field(path), expected, "{path}");
        }
    }
}

#[test]
fn nested_set_fields_use_the_nearest_typed_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json")).expect("language-neutral cases");
    let tree = DslValue::from(&fixture["document"]);
    let before = tree.clone();
    for case in fixture["cases"].as_array().expect("cases") {
        let rules = NormEditRules { set_field_arguments: &[], set_field: match case["rules"].as_str().expect("rules") { "parent" => &PARENTS, "exact" => &EXACT, "clear" => &CLEAR, _ => panic!("unknown rule set") }, insert_item: &[], remove_item: &[], move_item: &[] };
        let edit = NormEdit::SetField { path: case["path"].as_str().expect("path").to_owned(), value: DslValue::from(&case["value"]) };
        let result = rules.payload(&[], &tree, &edit);
        if let Some(error) = case["error"].as_str() {
            assert!(result.expect_err("refusal").contains(error), "{}", case["name"]);
            assert_eq!(tree, before, "refusal must not edit the document");
            continue;
        }
        let (kind, entries) = result.unwrap_or_else(|error| panic!("{}: {error}", case["name"]));
        let expected = &case["expected"];
        assert_eq!(kind, expected["kind"].as_str().expect("kind"), "{}", case["name"]);
        let mut payload = expected["selectors"].as_object().expect("selectors").clone();
        let field = expected["valueField"].as_str().expect("value field");
        if !field.is_empty() {
            let mut replacement = fixture["document"].pointer(expected["source"].as_str().expect("source pointer")).expect("existing boundary").clone();
            *replacement.pointer_mut(expected["pointer"].as_str().expect("child pointer")).expect("existing child") = case["value"].clone();
            payload.insert(field.to_owned(), replacement);
        }
        assert_eq!(DslValue::object(entries), DslValue::from(&serde_json::Value::Object(payload)), "{}", case["name"]);
        assert_eq!(tree, before, "resolution must not edit the document");
    }
}


#[test]
fn binding_lookups_preserve_structural_capabilities_and_concrete_boundaries() {
    const SET: [SetFieldRule; 3] = [
        SetFieldRule { path: "catalog.products[].configuration", kind: "replace-configuration", selectors: &[], value: "value" },
        SetFieldRule { path: "catalog.products[].configuration.exact", kind: "set-exact", selectors: &[], value: "value" },
        SetFieldRule { path: "clearOnly", kind: "clear-only", selectors: &[], value: "" },
    ];
    const INSERT: [InsertItemRule; 2] = [
        InsertItemRule { path: "catalog.products", kind: "add-product", selectors: &[], index: "index", item: "product" },
        InsertItemRule { path: "catalog.products[].configuration.items", kind: "add-item", selectors: &[], index: "index", item: "item" },
    ];
    const REMOVE: [RemoveItemRule; 1] = [
        RemoveItemRule { path: "catalog.products[].configuration.items", kind: "remove-item", selectors: &[], index: "index" },
    ];
    let rules = NormEditRules { set_field_arguments: &[], set_field: &SET, insert_item: &INSERT, remove_item: &REMOVE, move_item: &[] };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧭️edit-bindings/🧫️cases/🔣️.json")).expect("binding cases");
    for case in fixture["cases"].as_array().expect("cases") {
        let path = case["path"].as_str().expect("path");
        assert_eq!(rules.insertion_binding(path).map(|rule| rule.kind), case["insert"].as_str(), "{path}");
        assert_eq!(rules.can_insert_item(path), case["insert"].is_string(), "{path}");
        assert_eq!(rules.can_remove_item(path, ["catalog.products"]), case["remove"].as_bool().expect("remove"), "{path}");
        let found = rules.setter_binding(path);
        match &case["setter"] {
            serde_json::Value::Null => assert!(found.is_none(), "{path}"),
            expected => {
                let found = found.expect(path);
                assert_eq!(found.rule.kind, expected["kind"].as_str().expect("kind"), "{path}");
                assert_eq!(found.boundary, expected["boundary"].as_str().expect("boundary"), "{path}");
                assert_eq!(found.relative, expected["relative"].as_str().expect("relative"), "{path}");
            }
        }
    }
}

#[test]
fn declared_companion_arguments_complete_exact_payloads_from_the_same_addressed_row() {
    const SET: [SetFieldRule; 2] = [
        SetFieldRule { path: "members[].bucklingLengthZ", kind: "change-member-buckling-length", selectors: &[SelectorKey::Id("memberId")], value: "newLength" },
        SetFieldRule { path: "connections[].bolts.rows", kind: "change-bolt-count", selectors: &[SelectorKey::Id("connectionId")], value: "newRows" },
    ];
    const ARGUMENTS: [SetFieldArgument; 2] = [
        SetFieldArgument { path: SET[0].path, kind: SET[0].kind, argument: "axis", source: SetFieldArgumentSource::Text("z") },
        SetFieldArgument { path: SET[1].path, kind: SET[1].kind, argument: "newBoltsPerRow", source: SetFieldArgumentSource::Sibling("boltsPerRow") },
    ];
    let rules = NormEditRules { set_field_arguments: &ARGUMENTS, set_field: &SET, insert_item: &[], remove_item: &[], move_item: &[] };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧭️payload-bindings/🧫️cases/🔣️.json")).expect("closed companion payload cases");
    let declarations = fixture["arguments"].as_array().expect("declared companions");
    assert_eq!(declarations.len(), ARGUMENTS.len());
    for argument in &ARGUMENTS {
        let declared = declarations.iter().find(|row| row["path"] == argument.path && row["kind"] == argument.kind && row["argument"] == argument.argument).expect("native companion appears in neutral vocabulary");
        match argument.source {
            SetFieldArgumentSource::Text(value) => {
                assert_eq!(declared["source"]["type"], "text");
                assert_eq!(declared["source"]["value"], value);
            }
            SetFieldArgumentSource::Sibling(path) => {
                assert_eq!(declared["source"]["type"], "sibling");
                assert_eq!(declared["source"]["path"], path);
            }
        }
    }
    for case in fixture["cases"].as_array().expect("cases") {
        let tree = DslValue::from(&case["tree"]);
        let original = tree.clone();
        let edit = NormEdit::SetField { path: case["edit"]["path"].as_str().expect("path").into(), value: DslValue::from(&case["edit"]["value"]) };
        let result = rules.payload(&[], &tree, &edit);
        if let Some(error) = case["errorContains"].as_str() {
            assert!(result.expect_err("missing companion refuses").contains(error), "{}", case["name"]);
        } else {
            let (kind, entries) = result.expect("complete typed payload");
            assert_eq!(kind, case["expected"]["kind"].as_str().expect("kind"));
            let payload = DslValue::object(entries);
            assert_eq!(serde_json::Value::from(&payload), case["expected"]["payload"], "{}", case["name"]);
            if let Some(pointer) = case["siblingPointer"].as_str() {
                let sibling = case["tree"].pointer(pointer).expect("independent sibling pointer");
                assert_eq!(payload.get("newBoltsPerRow"), Some(&DslValue::from(sibling)));
                assert_eq!(sibling, &case["siblingValue"]);
            }
        }
        assert_eq!(tree, original, "payload resolution does not edit its source");
    }
}

#[test]
fn companion_arguments_refuse_selector_and_authored_value_collisions() {
    const SET: [SetFieldRule; 1] = [SetFieldRule { path: "members[].length", kind: "set-length", selectors: &[SelectorKey::Id("memberId")], value: "newLength" }];
    static COLLISIONS: [SetFieldArgument; 2] = [
        SetFieldArgument { path: SET[0].path, kind: SET[0].kind, argument: "memberId", source: SetFieldArgumentSource::Text("wrong") },
        SetFieldArgument { path: SET[0].path, kind: SET[0].kind, argument: "newLength", source: SetFieldArgumentSource::Text("wrong") },
    ];
    let tree = DslValue::from(&serde_json::json!({"members":[{"id":"M-1","length":2.0}]}));
    let edit = NormEdit::SetField { path: "members[id=M-1].length".into(), value: DslValue::float(3.0) };
    for companion in [&COLLISIONS[..1], &COLLISIONS[1..]] {
        let rules = NormEditRules { set_field_arguments: companion, set_field: &SET, insert_item: &[], remove_item: &[], move_item: &[] };
        assert!(rules.payload(&[], &tree, &edit).expect_err("collision refuses").contains("duplicate"));
    }
}

#[test]
fn a_parent_replacement_uses_companions_relative_to_its_own_concrete_boundary() {
    const SET: [SetFieldRule; 1] = [SetFieldRule { path: "connections[].bolts", kind: "replace-bolts", selectors: &[SelectorKey::Id("connectionId")], value: "newBolts" }];
    const ARGUMENTS: [SetFieldArgument; 1] = [SetFieldArgument { path: SET[0].path, kind: SET[0].kind, argument: "auditConnectionId", source: SetFieldArgumentSource::Sibling("id") }];
    let rules = NormEditRules { set_field_arguments: &ARGUMENTS, set_field: &SET, insert_item: &[], remove_item: &[], move_item: &[] };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧭️payload-bindings/🧫️cases/🔣️.json")).expect("neutral source");
    let case = fixture["cases"].as_array().expect("cases").iter().find(|row| row["rules"] == "bolts" && row.get("expected").is_some()).expect("bolts source");
    let tree = DslValue::from(&case["tree"]);
    let edit = NormEdit::SetField { path: case["edit"]["path"].as_str().expect("path").into(), value: DslValue::from(&case["edit"]["value"]) };
    let (kind, payload) = rules.payload(&[], &tree, &edit).expect("complete parent payload");
    let mut oracle = case["tree"].clone();
    *oracle.pointer_mut("/connections/0/bolts/rows").expect("independent descendant pointer") = case["edit"]["value"].clone();
    assert_eq!(kind, "replace-bolts");
    assert_eq!(serde_json::Value::from(&DslValue::object(payload)), serde_json::json!({
        "connectionId": oracle["connections"][0]["id"],
        "auditConnectionId": oracle["connections"][0]["id"],
        "newBolts": oracle.pointer("/connections/0/bolts").expect("independent replacement pointer"),
    }));
    assert_eq!(tree, DslValue::from(&case["tree"]));
}
