
use super::*;

#[test]
fn id_index_roundtrip() {
    let p = PatternId::from_index(7);
    assert_eq!(p.index(), 7);
    assert_eq!(p.get(), 7);
    assert_eq!(format!("{p}"), "7");
}

#[test]
fn id_ordering_and_equality() {
    let a = NodeId(1);
    let b = NodeId(2);
    assert!(a < b);
    assert_eq!(a, NodeId(1));
    assert_ne!(a, b);
}

#[test]
fn id_serde_roundtrip() {
    let r = RelationId(42);
    let json = protocol::json::to_json_string(&r);
    let back: RelationId = protocol::json::from_json_str(&json).unwrap();
    assert_eq!(r, back);
}
