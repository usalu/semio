use semio_framework_test_fixtures::repo_text;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::{ColdMutation, Mutation};


/// 🧫️ Every material field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn load_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "🩹️changes",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/🩹️changes/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/🩹️changes/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/🩹️changes/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/🩹️changes/🔺️diff/🔣️.json"))
        ),
        (
            "📍️target",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/📍️target/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/📍️target/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/📍️target/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/📍️target/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⏸️same/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⛔️missing/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️invalid",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⚗️invalid/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⚗️invalid/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/⚗️invalid/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem3dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("change-load field intent");
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
            for inverse in operation.inverse(&before).expect("inverse change-load field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

