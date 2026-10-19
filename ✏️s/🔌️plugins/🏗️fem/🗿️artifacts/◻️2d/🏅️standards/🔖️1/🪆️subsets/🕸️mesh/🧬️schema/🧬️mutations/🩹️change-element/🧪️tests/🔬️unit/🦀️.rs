use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::Mutation;
use protocol::ColdMutation;

/// 🧫️ Every element field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn element_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "🩹️changes",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🩹️changes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🩹️changes/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🩹️changes/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🩹️changes/🔺️diff/🔣️.json"))
        ),
        (
            "🔀️variant",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🔀️variant/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🔀️variant/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🔀️variant/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/🔀️variant/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⏸️same/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⛔️missing/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️invalid",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⚗️invalid/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⚗️invalid/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-element/⚗️invalid/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem2dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem2dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("element field intent");
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
            for inverse in operation.inverse(&before).expect("inverse element field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

/// 🛂️ The semantic envelope refuses undeclared outer fields before history admission.
#[test]
fn element_field_intent_wire_refuses_unknown_outer_fields() {
    assert!(semio_framework_pack_json::from_json_str::<Fem2dMutation>(r#"{"mutation":"changeElement","id":"steel","patch":{"e":190000000000},"newElement":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

/// 📨️ Sparse wire fields retain absence, authored empty collections and controlled decode ownership.
#[test]
fn sparse_patch_codec_preserves_authored_presence(){
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementPatch;
    use semio_framework_value::FromValue;
    use semio_framework_value::ToValue;
    let patch=semio_framework_pack_json::from_json_str::<Fem2dElementPatch>(r#"{"start":"n3"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("sparse patch");
    let encoded=semio_framework_pack_json::to_json_string(&patch);
    let actual:semio_framework_value::DslValue=semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let expected:semio_framework_value::DslValue=semio_framework_pack_json::from_json_str(r#"{"start":"n3"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(actual,expected);
    let value=patch.to_value();let mut allow=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    let decoded=<Fem2dElementPatch as FromValue>::from_value_controlled(&value,&mut control).expect("controlled sparse patch");
    assert_eq!(decoded,patch);<Fem2dElementPatch as FromValue>::retire_decoded(decoded);
    for invalid in[r#"{"start":null}"#,r#"{"id":"other"}"#]{assert!(semio_framework_pack_json::from_json_str::<Fem2dElementPatch>(invalid, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());let value=semio_framework_pack_json::from_json_str(invalid, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut allow=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);assert!(<Fem2dElementPatch as FromValue>::from_value_controlled(&value,&mut control).is_err());}
}