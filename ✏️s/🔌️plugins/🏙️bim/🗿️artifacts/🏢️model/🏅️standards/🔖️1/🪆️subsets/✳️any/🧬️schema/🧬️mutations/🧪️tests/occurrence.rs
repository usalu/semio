//! 🏷️ Canonical spatial-holder property commands exercise their real diff and semantic inverse through shared neutral inputs.
use crate::standards::v1::subsets::any::schema::mutations::{ModelMutation, apply_model_mutation};
use protocol::Mutation;
use protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law;
use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

fn base() -> ModelSnapshot {
    let mut value = ModelSnapshot::default();
    value.sites.insert("site".into(), from_json_str(r#"{"name":"Site","latitude":47.5,"longitude":8.5,"elevation":400,"true_north":0.2,"boundary":[]}"#, JsonMemberPolicy::Reject).expect("site"));
    value.buildings.insert("building".into(), from_json_str(r#"{"site":"site","name":"Building","origin":{"x":1,"y":2},"rotation":0.3,"elevation":4}"#, JsonMemberPolicy::Reject).expect("building"));
    value.storeys.insert("storey".into(), from_json_str(r#"{"building":"building","name":"Storey","level":0,"height":3,"cut_height":1.2}"#, JsonMemberPolicy::Reject).expect("storey"));
    value
}

#[semio_framework_async_macros::async_test]
async fn occurrence_neutral_commands_apply_and_both_inverse_routes_restore_each_spatial_holder() {
    let commands: Vec<ModelMutation> = from_json_str(fixture_text!("occurrence.json"), JsonMemberPolicy::Reject).expect("closed neutral commands");
    assert_eq!(commands.len(), 12);
    for command in commands {
        let base = base();
        let ModelMutation::SetElementProperty(payload) = &command else { panic!("declared property command") };
        let (diff, messages) = command.diff(&base).into_parts();
        assert!(messages.is_empty(), "{messages:?}");
        let next = apply_model_mutation(&base, &command).expect("canonical mutation");
        assert_eq!(protocol::apply_diff(&diff, &base).expect("canonical diff"), next);
        assert_eq!(next.properties[&payload.id][&payload.pset][&payload.property], payload.value);
        if let PropertyValue::Real { value } = &next.properties[&payload.id][&payload.pset][&payload.property] {
            assert_eq!(value.to_bits(), (-0.0f64).to_bits());
        }
        let mut restored = next;
        for inverse in command.inverse(&base).expect("semantic inverse").iter().rev() {
            restored = apply_model_mutation(&restored, inverse).expect("inverse command");
        }
        assert_eq!(restored, base);
        assert_diff_algebra_inverse_law::<ModelSnapshot, ModelDiff>(&base, &diff).await;
    }
}
