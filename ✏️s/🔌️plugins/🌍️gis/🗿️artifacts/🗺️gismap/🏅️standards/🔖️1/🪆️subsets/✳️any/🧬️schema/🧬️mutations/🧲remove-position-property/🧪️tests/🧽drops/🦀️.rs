//! 🧪️ `remove-position-property` fixture — `🧽drops`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1). The fixture edits the MIDDLE key of a
//! three-key position payload, so the position-exact inverse is exercised: removing `label` and undoing it must put it back BEFORE the third key.
//!
//! 🧩️ Committed snapshots preserve the stable drawing and value child identities across edits.

use crate::diff::GisMapDiff;
use crate::mutations::{inverse_gis_map_mutation, GisMapMutation};
use crate::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation;
use crate::GisMapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲remove-position-property/🧽drops/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲remove-position-property/🧽drops/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲remove-position-property/🧽drops/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲remove-position-property/🧽drops/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲remove-position-property/🧽drops/🎯️outcome/🔣️.json");
const LABEL: &str = "remove-position-property/drops-position-middle-property";

fn decode(text: &str) -> GisMapSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
}
fn before() -> GisMapSnapshot {
    decode(BEFORE)
}
fn expected_after() -> GisMapSnapshot {
    decode(AFTER)
}
fn mutation() -> GisMapMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn applied(base: &GisMapSnapshot, mutation: &GisMapMutation) -> GisMapSnapshot {
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, mutation).expect("the mutation applies");
    snapshot
}
fn data_keys(snapshot: &GisMapSnapshot) -> Vec<String> {
    snapshot.positions[0].data.as_object().expect("the payload is an object").iter().map(|(key, _)| key.clone()).collect()
}

#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = applied(&base, &mutation());
    assert_eq!(snapshot, expected_after(), "{LABEL}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.drawing.child_id, base.drawing.child_id, "{LABEL}: editing positions must preserve the stable drawing identity");
    assert_eq!(snapshot.value.child_id, base.value.child_id, "{LABEL}: editing positions must preserve the stable value identity");
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_gis_map_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = applied(&base, &mutation);
    for step in inverse.iter().rev() {
        apply_gis_map_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "{LABEL}: inverse did not restore the before-snapshot");
    assert_eq!(data_keys(&snapshot), data_keys(&base), "{LABEL}: the inverse must restore the payload's key order");
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decode(text))).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{LABEL}: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{LABEL}: committed mutation JSON is not canonical");
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "{LABEL}: this fixture pins an applied outcome");
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "{LABEL}: an applied outcome must emit no messages, got {:?}", produced.messages());
    assert_ne!(applied(&before(), &mutation()), before(), "{LABEL}: an applied mutation must change the document");
}

#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{LABEL}: produced diff differs from the committed 🔺️diff/🔣️.json");
}

#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_applies_to_after() {
    let decoded: GisMapDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{LABEL}: committed diff JSON is not canonical");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{LABEL}: committed diff did not carry before to after");
}

#[semio_framework_async_macros::async_test]
async fn diff_carries_exactly_one_property_edit() {
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let delta = produced.diff().positions.as_ref().expect("a position property edit writes a positions delta");
    assert_eq!(delta.modified.len(), 1, "{LABEL}: exactly one feature is patched, got {delta:?}");
    assert_eq!(delta.modified[0].id, "position-harbor", "{LABEL}: the patch is addressed by the feature id");
    assert!(delta.modified[0].patch.data.is_none(), "{LABEL}: a property edit must never carry the whole payload");
    assert_eq!(delta.modified[0].patch.properties.len(), 1, "{LABEL}: exactly one property edit");
    assert_eq!(delta.modified[0].patch.properties[0].key, "label");
    assert!(delta.inserted.is_empty() && delta.removed.is_empty() && delta.moved.is_empty(), "{LABEL}: a property edit must not add, remove or reorder features, got {delta:?}");
    let others = [produced.diff().positions.is_some(), produced.diff().routes.is_some(), produced.diff().regions.is_some()];
    assert_eq!(others.iter().filter(|touched| **touched).count(), 1, "{LABEL}: only the positions collection is touched");
}

#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

#[semio_framework_async_macros::async_test]
async fn a_missing_feature_is_a_target_missing_error() {
    let mut gone = mutation();
    if let GisMapMutation::RemovePositionProperty(inner) = &mut gone { inner.feature = "gone".into(); }
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&before(), &gone).await;
}

/// ⚖️ Removing the LAST key restores by append; removing the FIRST key restores before the old second key. Both obey the sum law.
#[semio_framework_async_macros::async_test]
async fn removing_the_first_or_last_key_restores_its_position() {
    let base = before();
    for key in ["kind", "note"] {
        let removal = GisMapMutation::RemovePositionProperty(crate::mutations::remove_position_property::RemovePositionProperty { feature: "position-harbor".into(), key: key.into() });
        let snapshot = applied(&base, &removal);
        assert!(!data_keys(&snapshot).iter().any(|present| present == key), "{LABEL}: the key is gone");
        let inverse = inverse_gis_map_mutation(&base, &removal).expect("inverse");
        let mut restored = snapshot;
        for step in inverse.iter().rev() {
            apply_gis_map_mutation(&mut restored, step).expect("inverse step applies");
        }
        assert_eq!(restored, base, "{LABEL}: removing `{key}` must undo to the original order");
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&removal, &base).await;
    }
}

/// ⚖️ Removing an absent key is a diagnosed no-op with no inverse.
#[semio_framework_async_macros::async_test]
async fn removing_an_absent_key_is_a_noop() {
    let base = before();
    let absent = GisMapMutation::RemovePositionProperty(crate::mutations::remove_position_property::RemovePositionProperty { feature: "position-harbor".into(), key: "absent".into() });
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&absent, &base);
    assert!(produced.messages().iter().any(|message| format!("{:?}", message.level) == "Warning"), "{LABEL}: got {:?}", produced.messages());
    assert!(inverse_gis_map_mutation(&base, &absent).expect("inverse").is_empty());
}
