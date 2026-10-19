use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::Mutation;
use protocol::ColdMutation;

/// 🧫️ Every node field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn node_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "🩹️changes",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/🔺️diff/🔣️.json"))
        ),
        (
            "📐️coordinates",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⛔️missing/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️empty",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/🔺️diff/🔣️.json"))
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem3dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("node field intent");
        let after = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(after, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot");
        let outcome = operation.diff(&before);
        if let Some(diff) = diff {
            let expected = semio_framework_pack_json::from_json_str::<Fem3dDiff>(diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored sparse diff");
            assert_eq!(outcome.diff(), &expected, "{id} authored fields");
        } else {
            assert!(!outcome.messages().is_empty(), "{id} has a refusal");
        }
        let (mut replayed, _) = vcs::apply_mutation(&before, &operation).expect("central applier");
        assert_eq!(replayed, after, "{id} projection");
        if diff.is_some() {
            for inverse in operation.inverse(&before).expect("inverse node field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

/// 🛂️ The semantic envelope refuses undeclared outer fields before history admission.
#[test]
fn node_field_intent_wire_refuses_unknown_outer_fields() {
    assert!(semio_framework_pack_json::from_json_str::<Fem3dMutation>(r#"{"mutation":"changeNode","id":"steel","patch":{"e":190000000000},"newNode":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodePatch;
/// 📨️ Sparse scalar setters omit untouched properties and reject explicit nulls and identity fields.
#[test]
fn structural_patch_wire_distinguishes_absence_from_null() {
    let patch = Fem3dNodePatch { x: Some(4.0), ..Default::default() };
    let encoded = semio_framework_pack_json::to_json_string(&patch);
    assert_eq!(encoded, r#"{"x":4.0}"#);
    assert_eq!(semio_framework_pack_json::from_json_str::<Fem3dNodePatch>(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("one-field patch"), patch);
    assert_eq!(semio_framework_pack_json::from_json_str::<Fem3dNodePatch>("{}", semio_framework_pack_json::JsonMemberPolicy::Reject).expect("no fields"), Fem3dNodePatch::default());
    let value = semio_framework_value::ToValue::to_value(&patch);
    let mut allow = |_| true;
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536, &mut allow);
    let decoded = <Fem3dNodePatch as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).expect("controlled sparse field decode");
    assert_eq!(decoded, patch);
    semio_framework_value::FromValue::retire_decoded(decoded);

    for wire in [r#"{"x":null}"#, r#"{"x":null}"#, r#"{"id":"another"}"#] {
        assert!(semio_framework_pack_json::from_json_str::<Fem3dNodePatch>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{wire} must be refused");
        let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("wire tree");
        let mut allow = |_| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536, &mut allow);
        assert!(<Fem3dNodePatch as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).is_err(), "{wire} controlled decode must be refused");
    }
}

