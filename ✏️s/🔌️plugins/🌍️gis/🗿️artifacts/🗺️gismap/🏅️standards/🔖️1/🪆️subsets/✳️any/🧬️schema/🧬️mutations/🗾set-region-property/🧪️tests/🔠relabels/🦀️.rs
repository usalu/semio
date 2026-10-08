//! 🧪️ `set-region-property` fixture — `🔠relabels`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1). The fixture edits the MIDDLE key of a
//! three-key region payload, so the position-exact inverse is exercised: the set is in place, so the key order is unchanged and the inverse restores the old value.
//!
//! 🧩️ Committed snapshots preserve the stable drawing and value child identities across edits.

use crate::diff::GisMapDiff;
use crate::mutations::{inverse_gis_map_mutation, GisMapMutation};
use crate::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation;
use crate::GisMapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗾set-region-property/🔠relabels/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗾set-region-property/🔠relabels/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗾set-region-property/🔠relabels/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗾set-region-property/🔠relabels/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗾set-region-property/🔠relabels/🎯️outcome/🔣️.json");
const LABEL: &str = "set-region-property/relabels-region-middle-property";

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
    snapshot.regions[0].data.as_object().expect("the payload is an object").iter().map(|(key, _)| key.clone()).collect()
}

#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = applied(&base, &mutation());
    assert_eq!(snapshot, expected_after(), "{LABEL}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.drawing.child_id, base.drawing.child_id, "{LABEL}: editing regions must preserve the stable drawing identity");
    assert_eq!(snapshot.value.child_id, base.value.child_id, "{LABEL}: editing regions must preserve the stable value identity");
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
    let delta = produced.diff().regions.as_ref().expect("a region property edit writes a regions delta");
    assert_eq!(delta.modified.len(), 1, "{LABEL}: exactly one feature is patched, got {delta:?}");
    assert_eq!(delta.modified[0].id, "region-old-town", "{LABEL}: the patch is addressed by the feature id");
    assert!(delta.modified[0].patch.data.is_none(), "{LABEL}: a property edit must never carry the whole payload");
    assert_eq!(delta.modified[0].patch.properties.len(), 1, "{LABEL}: exactly one property edit");
    assert_eq!(delta.modified[0].patch.properties[0].key, "label");
    assert!(delta.inserted.is_empty() && delta.removed.is_empty() && delta.moved.is_empty(), "{LABEL}: a property edit must not add, remove or reorder features, got {delta:?}");
    let others = [produced.diff().positions.is_some(), produced.diff().routes.is_some(), produced.diff().regions.is_some()];
    assert_eq!(others.iter().filter(|touched| **touched).count(), 1, "{LABEL}: only the regions collection is touched");
}

#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

#[semio_framework_async_macros::async_test]
async fn a_missing_feature_is_a_target_missing_error() {
    let mut gone = mutation();
    if let GisMapMutation::SetRegionProperty(inner) = &mut gone { inner.feature = "gone".into(); }
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&before(), &gone).await;
}

/// ⚖️ A NEW key appends (or is inserted before the named key) and inverts by removal; both obey the sum law.
#[semio_framework_async_macros::async_test]
async fn a_new_key_appends_or_inserts_before_a_key_and_inverts_by_removal() {
    let base = before();
    for (before_key, expected) in [(None, vec!["kind", "label", "status", "zone"]), (Some("label"), vec!["kind", "zone", "label", "status"])] {
        let fresh = GisMapMutation::SetRegionProperty(crate::mutations::set_region_property::SetRegionProperty {
            feature: "region-old-town".into(),
            key: "zone".into(),
            value: semio_framework_value::DslValue::String("north".into()),
            before: before_key.map(str::to_owned),
        });
        let snapshot = applied(&base, &fresh);
        assert_eq!(data_keys(&snapshot), expected, "{LABEL}: the new key lands at its requested position");
        let inverse = inverse_gis_map_mutation(&base, &fresh).expect("inverse");
        assert!(matches!(inverse.as_slice(), [GisMapMutation::RemoveRegionProperty(_)]), "{LABEL}: a created key is undone by removal, got {inverse:?}");
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&fresh, &base).await;
    }
}

/// ⚖️ Setting an existing key to its current value is a diagnosed no-op and changes nothing.
#[semio_framework_async_macros::async_test]
async fn setting_the_current_value_is_a_noop() {
    let base = before();
    let current = base.regions[0].data.as_object().expect("object")[1].1.clone();
    let same = GisMapMutation::SetRegionProperty(crate::mutations::set_region_property::SetRegionProperty { feature: "region-old-town".into(), key: "label".into(), value: current, before: None });
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&same, &base);
    assert!(produced.messages().iter().any(|message| format!("{:?}", message.level) == "Warning"), "{LABEL}: got {:?}", produced.messages());
    assert!(produced.diff().regions.is_none());
}
