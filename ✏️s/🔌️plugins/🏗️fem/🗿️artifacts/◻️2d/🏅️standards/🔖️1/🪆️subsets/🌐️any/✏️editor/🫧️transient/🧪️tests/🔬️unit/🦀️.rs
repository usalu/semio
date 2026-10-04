use super::*;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️gumball-transient/🔣️.json");

/// 🔣️ The committed language-agnostic transient decodes, re-encodes to the same value, and survives every codec.
#[test]
fn the_committed_transient_is_canonical_and_round_trips_every_codec() {
    let transient: FemGumballTransient = semio_framework_pack_json::from_json_str(FIXTURE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed transient decodes");
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(FIXTURE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed transient parses");
    assert_eq!(semio_framework_value::ToValue::to_value(&transient), committed, "decode→encode is a fixed point");
    let printed = <FemGumballTransient as store::ArtifactDsl>::print_dsl(&transient);
    assert_eq!(<FemGumballTransient as store::ArtifactDsl>::parse_dsl(&printed).expect("text round trip"), transient);
    let packed = <FemGumballTransient as store::ArtifactPack>::encode_pack(&transient);
    assert_eq!(<FemGumballTransient as store::ArtifactPack>::decode_pack(&packed).expect("pack round trip"), transient);
    let operation = FemGumballTransientMutation::Snapshot { transient: transient.clone() };
    assert_eq!(<FemGumballTransientMutation as protocol::OpBinary>::decode_op(&protocol::OpBinary::encode_op(&operation).expect("encodes")).expect("decodes"), operation);
}

/// 🪟️ A window's gesture is replaced or cleared without touching a sibling window's gesture.
#[test]
fn a_window_gesture_is_keyed_by_its_window_alone() {
    let transient: FemGumballTransient = semio_framework_pack_json::from_json_str(FIXTURE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed transient decodes");
    let gesture = transient.gestures["model-left"].clone();
    let both = transient.with_gesture("model-right", Some(gesture.clone()));
    assert_eq!(both.gestures.len(), 2);
    let cleared = both.with_gesture("model-left", None);
    assert_eq!(cleared.gestures.keys().collect::<Vec<_>>(), vec!["model-right"], "clearing one window's gesture keeps the sibling's");
    assert_eq!(cleared.gestures["model-right"], gesture);
}
