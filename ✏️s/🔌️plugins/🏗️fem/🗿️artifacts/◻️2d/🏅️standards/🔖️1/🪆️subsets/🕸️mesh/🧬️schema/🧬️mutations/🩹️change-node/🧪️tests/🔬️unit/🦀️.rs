use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::Mutation;
use protocol::ColdMutation;

/// 🧫️ Every node field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn node_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "🩹️changes",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/🔺️diff/🔣️.json"))
        ),
        (
            "📐️coordinates",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/📐️coordinates/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⛔️missing/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️empty",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-node/⚗️empty/🔺️diff/🔣️.json"))
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem2dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem2dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("node field intent");
        let after = semio_framework_pack_json::from_json_str::<Fem2dSnapshotValue>(after, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot");
        let outcome = operation.diff(&before);
        if let Some(diff) = diff {
            let expected = semio_framework_pack_json::from_json_str::<Fem2dDiff>(diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored sparse diff");
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
    assert!(semio_framework_pack_json::from_json_str::<Fem2dMutation>(r#"{"mutation":"changeNode","id":"steel","patch":{"e":190000000000},"newNode":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodePatch;
/// 📨️ Sparse scalar setters omit untouched properties and reject explicit nulls and identity fields.
#[test]
fn structural_patch_wire_distinguishes_absence_from_null() {
    let patch = Fem2dNodePatch { x: Some(4.0), ..Default::default() };
    let encoded = semio_framework_pack_json::to_json_string(&patch);
    assert_eq!(encoded, r#"{"x":4.0}"#);
    assert_eq!(semio_framework_pack_json::from_json_str::<Fem2dNodePatch>(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("one-field patch"), patch);
    assert_eq!(semio_framework_pack_json::from_json_str::<Fem2dNodePatch>("{}", semio_framework_pack_json::JsonMemberPolicy::Reject).expect("no fields"), Fem2dNodePatch::default());
    let value = semio_framework_value::ToValue::to_value(&patch);
    let mut allow = |_| true;
    let mut control = semio_framework_value::NativeDecodeControl::new(65536, &mut allow);
    let decoded = <Fem2dNodePatch as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).expect("controlled sparse field decode");
    assert_eq!(decoded, patch);
    semio_framework_value::FromValue::retire_decoded(decoded);

    for wire in [r#"{"x":null}"#, r#"{"x":null}"#, r#"{"id":"another"}"#] {
        assert!(semio_framework_pack_json::from_json_str::<Fem2dNodePatch>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{wire} must be refused");
        let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("wire tree");
        let mut allow = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(65536, &mut allow);
        assert!(<Fem2dNodePatch as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).is_err(), "{wire} controlled decode must be refused");
    }
}

