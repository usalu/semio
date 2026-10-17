import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
start = s.index("//#region 🧪️RetainedConfigOracle")
end = s.index("//#endregion 🧪️RetainedConfigOracle") + len("//#endregion 🧪️RetainedConfigOracle")
assert s.count("//#region 🧪️RetainedConfigOracle") == 1
new = '''//#region 🧪️RetainedConfigOracle
#[test]
fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
    let base = SpaceConfig::default();
    let base_value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&base));
    let mut expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&base_value)).expect("third-party JSON decode");
    expected["workflowEngagementInput"] = serde_json::json!("draft");
    let draft = SpaceConfigMutation::SetWorkflowEngagementInput(WorkflowEngagementInputSetting { value: "draft".into() });
    let (post, _messages) = store::apply_mutation(&base, &draft).expect("bounded config candidate");
    let post_value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&post));
    let post_oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&post_value)).expect("third-party JSON decode");
    assert_eq!(post_oracle, expected);
    let inverse = protocol::Mutation::inverse(&draft, &base).expect("bounded config inverse");
    assert!(matches!(inverse.as_slice(), [SpaceConfigMutation::SetWorkflowEngagementInput(WorkflowEngagementInputSetting { value })] if *value == base.workflow_engagement_input));
    let factory = space_retained_store_preparation::<SpaceConfig, SpaceConfigMutation>("space-studio-config-retained-law", SPACE_CONFIG_RETAINED_BYTES).expect("space lanes always supply a preparation factory");
    let lane = store::os_store::store_schema::HistoryLane::Document;
    let sized = |bytes: usize| factory.preflight(&SpaceConfigMutation::SetWorkflowEngagementInput(WorkflowEngagementInputSetting { value: "x".repeat(bytes) }), lane);
    let maximum = (0..=SPACE_CONFIG_RETAINED_BYTES).partition_point(|bytes| sized(bytes).is_ok()) - 1;
    assert!(maximum > 8_192, "the retained config lane must admit a text well beyond one toggle, admitted {maximum}");
    assert!(sized(maximum).expect("maximum text is admitted").retained_bytes <= SPACE_CONFIG_RETAINED_BYTES);
    assert!(sized(maximum + 1).is_err(), "the retained config lane must refuse maximum plus one");
    for admitted in [
        SpaceConfigMutation::SetClipboard(ClipboardSetting { node_ids: Vec::new() }),
        SpaceConfigMutation::SetSpaceId(SpaceIdSetting { space_id: Some("demo".into()) }),
        SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: Some("node-1".into()) }),
        SpaceConfigMutation::SetFocusedNode(FocusedNodeSetting { node_id: Some("node-1".into()) }),
    ] {
        assert!(factory.preflight(&admitted, lane).is_ok(), "the retained config lane must admit the session mutation published by openSpace/openInstance/spawnApp");
        assert!(store::apply_mutation(&base, &admitted).is_ok());
    }
    let active = SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: Some("node-1".into()) });
    let footprint = factory.preflight(&active, lane).expect("active node footprint");
    assert!(footprint.retained_bytes <= 4_096, "one real session config turn must fit the host's fixed 4 KiB typed-operation page grant, priced {}", footprint.retained_bytes);
    assert_eq!(footprint.work_items, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
}
//#endregion 🧪️RetainedConfigOracle'''
open(p, "w", encoding="utf-8").write(s[:start] + new + s[end:])
