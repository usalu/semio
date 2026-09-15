use super::*;
use crate::standards::v1::subsets::any::schema::default_generation3d_snapshot;

#[test]
fn diff_absorb_prefers_incoming_fixture_and_preserves_generation() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧲️absorb/🔣️.json")).unwrap();
    let base = default_generation3d_snapshot();
    let mut first_fixture = base.host_snapshot.clone();
    first_fixture.camera = pack::from_json_str(&oracle["firstCamera"].to_string()).unwrap();
    let mut incoming = base.host_snapshot.clone();
    incoming.camera = pack::from_json_str(&oracle["incomingCamera"].to_string()).unwrap();
    let mut first = Generation3dDiff { host_snapshot: Some(first_fixture), generation: Some(base.generation.clone()), ..Default::default() };
    first.absorb(Generation3dDiff { host_snapshot: Some(incoming.clone()), ..Default::default() });
    assert_eq!(first.host_snapshot.as_ref(), Some(&incoming));
    assert_eq!(first.generation.as_ref(), Some(&base.generation));
    let mut expected = base.clone();
    expected.host_snapshot = incoming;
    let next = first.apply(&base).expect("absorbed diff applies");
    assert_eq!(next, expected);
    let camera: serde_json::Value = serde_json::from_str(&pack::to_json_string(&next.host_snapshot.camera)).unwrap();
    assert_eq!(camera, oracle["incomingCamera"]);
}
