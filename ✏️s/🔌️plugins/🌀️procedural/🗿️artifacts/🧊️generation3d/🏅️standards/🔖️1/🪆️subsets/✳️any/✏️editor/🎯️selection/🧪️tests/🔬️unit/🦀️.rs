use super::*;

#[test]
fn exact_analytic_labels_require_current_source_membership() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["analyticAdmission"].as_array().unwrap() {
        let ids: Vec<String> = serde_json::from_value(case["ids"].clone()).unwrap();
        let references = serde_json::from_value(case["references"].clone()).unwrap();
        let actual = validate_analytic_source(&ids, case["revision"].as_str().unwrap(), case["handle"].as_str().unwrap(), &references);
        if case["accepted"] == true { assert_eq!(serde_json::to_value(actual.unwrap()).unwrap(), fixtures["analyticGroups"][0]["labels"]); } else { assert!(actual.is_err()); }
    }
}

#[test]
fn analytic_highlight_remaps_exact_labels_and_refuses_changed_source() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let id = fixtures["analyticGroups"][0]["ids"][0].as_str().unwrap();
    let target = ComponentTarget::parse(id).unwrap();
    let source = target.analytic.as_ref().unwrap();
    let instances = semio_framework_pack_json::parse(&serde_json::json!([{ "id": target.instance, "meshId": "mesh", "componentSource": {"handle":source.handle,"revision":source.revision} }]).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let meshes = semio_framework_pack_json::parse(&serde_json::json!([{ "id":"mesh", "data": {"componentReferences": {"edge":[source.label,"1"]}} }]).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let remapped = projected_target(target.clone(), &instances, &meshes).unwrap();
    assert_eq!(remapped.component, 0);
    assert!(remapped.interaction_id().ends_with(&format!("~{}~{}~{}",source.handle,source.label,source.revision)));
    let changed = semio_framework_pack_json::parse(&serde_json::json!([{ "id": target.instance, "meshId": "mesh", "componentSource": {"handle":"c".repeat(64),"revision":source.revision} }]).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(projected_target(target, &changed, &meshes).is_none());
}

#[test]
fn component_addresses_keep_authored_widget_and_channel() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["valid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let target = ComponentTarget::parse(id).expect(id);
        assert_eq!(target.widget, case["widget"].as_str().unwrap());
        assert_eq!(target.channel(), case["channel"].as_str().unwrap());
        assert_eq!(target.interaction_id(), id);
    }

    let case = &fixtures["valid"][0];
    let source = &fixtures["analyticAdmission"][0];
    let id = format!("{}~{}~1~{}", case["id"].as_str().unwrap(), source["handle"].as_str().unwrap(), source["revision"].as_str().unwrap());
    let target = ComponentTarget::parse(&id).expect(&id);
    assert_eq!(target.widget, case["widget"].as_str().unwrap());
    assert_eq!(target.interaction_id(), id);
}
