use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::Mutation;
use protocol::ColdMutation;

/// 🧫️ Every section field-intent fixture preserves its authored forward result and exact inverse.
#[test]
fn section_field_intent_follows_committed_vectors() {
    let vectors = [
        (
            "🩹️changes",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/🩹️changes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/🩹️changes/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/🩹️changes/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/🩹️changes/🔺️diff/🔣️.json"))
        ),
        (
            "📐️coordinates",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/📐️coordinates/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/📐️coordinates/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/📐️coordinates/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/📐️coordinates/🔺️diff/🔣️.json"))
        ),
        (
            "⏸️same",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⏸️same/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⏸️same/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⏸️same/📸️snapshot/➡️after/🔣️.json"),
            Some(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⏸️same/🔺️diff/🔣️.json"))
        ),
        (
            "⛔️missing",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⛔️missing/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⛔️missing/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⛔️missing/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
        (
            "⚗️invalid",
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⚗️invalid/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⚗️invalid/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/⚗️invalid/📸️snapshot/➡️after/🔣️.json"),
            None
        ),
    ];
    for (id, before, mutation, after, diff) in vectors {
        let before = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot");
        let operation = semio_framework_pack_json::from_json_str::<Fem3dMutation>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("section field intent");
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
            for inverse in operation.inverse(&before).expect("inverse section field intent").iter().rev() {
                replayed = vcs::apply_mutation(&replayed, inverse).expect("central inverse").0;
            }
            assert_eq!(replayed, before, "{id} exact inverse");
        }
    }
}

/// 🛂️ The semantic envelope refuses undeclared outer fields before history admission.
#[test]
fn section_field_intent_wire_refuses_unknown_outer_fields() {
    assert!(semio_framework_pack_json::from_json_str::<Fem3dMutation>(r#"{"mutation":"changeSection","id":"steel","patch":{"e":190000000000},"newSection":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionPatch;
/// 📨️ Sparse scalar setters omit untouched properties and reject explicit nulls and identity fields.
#[test]
fn structural_patch_wire_distinguishes_absence_from_null() {
    let patch = Fem3dSectionPatch { area: Some(0.03), ..Default::default() };
    let encoded = semio_framework_pack_json::to_json_string(&patch);
    assert_eq!(encoded, r#"{"area":0.03}"#);
    assert_eq!(semio_framework_pack_json::from_json_str::<Fem3dSectionPatch>(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("one-field patch"), patch);
    assert_eq!(semio_framework_pack_json::from_json_str::<Fem3dSectionPatch>("{}", semio_framework_pack_json::JsonMemberPolicy::Reject).expect("no fields"), Fem3dSectionPatch::default());
    let value = semio_framework_value::ToValue::to_value(&patch);
    let mut allow = |_| true;
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536, &mut allow);
    let decoded = <Fem3dSectionPatch as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).expect("controlled sparse field decode");
    assert_eq!(decoded, patch);
    semio_framework_value::FromValue::retire_decoded(decoded);

    for wire in [r#"{"area":null}"#, r#"{"area":null}"#, r#"{"id":"another"}"#] {
        assert!(semio_framework_pack_json::from_json_str::<Fem3dSectionPatch>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{wire} must be refused");
        let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("wire tree");
        let mut allow = |_| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536, &mut allow);
        assert!(<Fem3dSectionPatch as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).is_err(), "{wire} controlled decode must be refused");
    }
}

