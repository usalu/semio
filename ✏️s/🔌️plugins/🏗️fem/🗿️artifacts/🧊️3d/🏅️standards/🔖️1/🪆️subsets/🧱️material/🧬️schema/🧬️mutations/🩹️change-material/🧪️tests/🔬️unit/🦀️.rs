use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::Mutation;
use protocol::ColdMutation;

/// 🧫️ Every material field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn material_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "🩹️renames",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/🩹️renames/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/🩹️renames/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/🩹️renames/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/🩹️renames/🔺️diff/🔣️.json"))
        ),
        (
            "📉️softens",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/📉️softens/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/📉️softens/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/📉️softens/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/📉️softens/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⏸️same/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⛔️missing/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️invalid",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⚗️invalid/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⚗️invalid/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/⚗️invalid/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem3dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("material field intent");
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
            for inverse in operation.inverse(&before).expect("inverse material field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

/// 🛂️ The semantic envelope refuses undeclared outer fields before history admission.
#[test]
fn material_field_intent_wire_refuses_unknown_outer_fields() {
    assert!(semio_framework_pack_json::from_json_str::<Fem3dMutation>(r#"{"mutation":"changeMaterial","id":"steel","patch":{"e":190000000000},"newMaterial":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}
