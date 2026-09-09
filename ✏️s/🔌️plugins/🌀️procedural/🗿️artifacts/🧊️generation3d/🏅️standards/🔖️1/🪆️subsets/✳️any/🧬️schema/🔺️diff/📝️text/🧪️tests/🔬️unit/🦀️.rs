use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot;

#[test]
fn diff_absorb_prefers_incoming_fixture_and_preserves_generation() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧲️absorb/🔣️.json")).unwrap();
    let base = empty_generation3d_snapshot();
    let mut first_fixture = base.fixture.clone();
    first_fixture.camera = pack::from_json_str(&oracle["firstCamera"].to_string()).unwrap();
    let mut incoming = base.fixture.clone();
    incoming.camera = pack::from_json_str(&oracle["incomingCamera"].to_string()).unwrap();
    let mut first = Generation3dDiff { fixture: Some(first_fixture), generation: Some(base.generation.clone()), ..Default::default() };
    first.absorb(Generation3dDiff { fixture: Some(incoming.clone()), ..Default::default() });
    assert_eq!(first.fixture.as_ref(), Some(&incoming));
    assert_eq!(first.generation.as_ref(), Some(&base.generation));
    let mut expected = base.clone();
    expected.fixture = incoming;
    let next = first.apply(&base).expect("absorbed diff applies");
    assert_eq!(next, expected);
    let camera: serde_json::Value = serde_json::from_str(&pack::to_json_string(&next.fixture.camera)).unwrap();
    assert_eq!(camera, oracle["incomingCamera"]);
}
