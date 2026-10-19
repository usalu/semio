use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::Mutation;
use protocol::ColdMutation;

/// 🧫️ Every material field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn load_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "⚖️weights",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚖️weights/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚖️weights/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚖️weights/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚖️weights/🔺️diff/🔣️.json"))
        ),
        (
            "🗑️removes",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/🗑️removes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/🗑️removes/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/🗑️removes/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/🗑️removes/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⏸️same/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⛔️missing/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️invalid",
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚗️invalid/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚗️invalid/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚗️invalid/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem3dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("change-combination-term field intent");
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
            for inverse in operation.inverse(&before).expect("inverse change-combination-term field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

