use super::*;
use semio_framework_value::collection::Identified;

#[test]
fn defining_dag_identity_matches_neutral_corpus() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::from_str(include_str!("../../🧫️fixtures/🪪️defining-identity/🔣️.json")).expect("closed identity fixture");
    for row in fixture["cases"].as_array().expect("cases") {
        let identity = row["identity"].as_str().expect("identity").to_owned();
        if row["kind"].as_str() == Some("node") {
            let node = DagNodeSpec { id: identity.clone(), ..Default::default() };
            assert_eq!(Identified::id(&node), &identity);
        } else {
            let edge = DagHostSnapshotEdge { id: identity.clone(), ..Default::default() };
            assert_eq!(Identified::id(&edge), &identity);
        }
    }
}
