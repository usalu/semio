use super::*;
use protocol::{Mutation, MutationDiff};

#[test]
fn app_config_owns_only_the_host_contribution_projection() {
    let base = FormsConfig::default();
    assert_eq!(base.contributions_json, "[]");
    let mutation = FormsConfigMutation::SetContributions(SetContributions { json: "[{\"pluginId\":\"example\"}]".into() });
    let next = mutation.diff(&base).diff().apply(&base).expect("contribution mutation");
    assert_eq!(next.contributions_json, "[{\"pluginId\":\"example\"}]");
}
