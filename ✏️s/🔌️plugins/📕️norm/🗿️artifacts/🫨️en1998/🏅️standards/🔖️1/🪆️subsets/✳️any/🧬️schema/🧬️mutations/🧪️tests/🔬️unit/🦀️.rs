use semio_s_artifact_norm_en1998_edit_rules::EDIT_RULES;
use semio_s_artifact_norm_en1998_mutations::*;
use protocol::{ColdMutation, Mutation};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

fn every_mutation() -> Vec<En1998Mutation> {
    semio_s_artifact_norm_en1998_mutations_demo::demo_mutation_cases()
}

#[semio_framework_async_macros::async_test]
async fn every_variant_has_diff_and_inverse() {
    let base = En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for mutation in every_mutation() {
        let _ = mutation.diff(&base);
        let _ = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    }
}

#[semio_framework_async_macros::async_test]
async fn change_site_s_ap_r_applies_plateau_acceleration() {
    let base = En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let mut site = base.site.clone();
    site.s_ap_r = 2.0;
    let mutation = En1998Mutation::ChangeSiteSApR(semio_s_artifact_norm_en1998_mut_buildings::change_site_s_ap_r::ChangeSiteSApR { new_s_ap_r: 2.0 });
    let outcome = mutation.diff(&base);
    let applied = protocol::apply_diff_cold(outcome.diff(), &base).expect("apply");
    assert_eq!(applied.site.s_ap_r, 2.0);
}

fn leaf_templates(prefix: &str, value: &semio_framework_value::DslValue, out: &mut Vec<String>) {
    match value {
        semio_framework_value::DslValue::Object(entries) => {
            for (key, child) in entries {
                let path = if prefix.is_empty() { key.clone() } else { format!("{prefix}.{key}") };
                leaf_templates(&path, child, out);
            }
        }
        semio_framework_value::DslValue::Array(items) => {
            for item in items {
                leaf_templates(&format!("{prefix}[]"), item, out);
            }
        }
        _ => out.push(prefix.to_string()),
    }
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_has_a_set_field_rule() {
    let tree = semio_framework_value::ToValue::to_value(&En1998Snapshot::compliant_de_multipart());
    let mut leaves = Vec::new();
    leaf_templates("", &tree, &mut leaves);
    leaves.sort();
    leaves.dedup();
    let missing: Vec<&String> = leaves
        .iter()
        .filter(|path| !path.ends_with(".id") && !path.contains("variables[]"))
        .filter(|path| !EDIT_RULES.set_field.iter().any(|rule| rule.path == path.as_str()))
        .collect();
    assert!(missing.is_empty(), "leaves without a set-field rule: {missing:?}");
}

#[semio_framework_async_macros::async_test]
async fn every_set_field_rule_names_a_catalogued_kind() {
    for rule in EDIT_RULES.set_field {
        assert!(KINDS.contains(&rule.kind), "rule {} names unknown kind {}", rule.path, rule.kind);
    }
}
