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
            "🩹️renames",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/🩹️renames/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/🩹️renames/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/🩹️renames/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/🩹️renames/🔺️diff/🔣️.json"))
        ),
        (
            "✏️labels",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/✏️labels/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/✏️labels/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/✏️labels/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/✏️labels/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⏸️same/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⛔️missing/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️empty",
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⚗️empty/📸️snapshot/⬅️before/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⚗️empty/🦠️mutation/🔣️.json"),
            repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⚗️empty/📸️snapshot/➡️after/🔣️.json"),
            Some(repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⚗️empty/🔺️diff/🔣️.json"))
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem3dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("change-combination-name field intent");
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
            for inverse in operation.inverse(&before).expect("inverse change-combination-name field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

