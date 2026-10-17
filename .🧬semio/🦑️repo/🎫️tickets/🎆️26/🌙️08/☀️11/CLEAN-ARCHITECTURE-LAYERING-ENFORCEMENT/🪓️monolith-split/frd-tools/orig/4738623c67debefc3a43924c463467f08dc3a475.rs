use crate::standards::v2_0::subsets::any::schema::mutations::contract_tests::{assert_laws, decode, scene_snapshot};
use crate::standards::v2_0::subsets::any::schema::mutations::create_scene as mutation;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../../🎬️scene/🌱️create/📜️contract/🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfCreateScenePayload = decode(&vector["payload"]);
        let base = scene_snapshot(&vector["base"]);
        let expected = scene_snapshot(&vector["after"]);
        assert_eq!(scene_snapshot(&vector["undo"]), base);
        assert_eq!(protocol::apply_diff(&mutation::plan(&payload, &base).unwrap(), &base).unwrap(), expected);
        assert_laws(&mutation::CreateSceneMutation::Apply(payload.clone()), &base, &expected);
        if let Some(malformed) = vector.get("malformedPayload") {
            let text = malformed["encoded"].as_str().unwrap();
            assert!(semio_framework_pack_json::from_json_str::<mutation::GltfCreateScenePayload>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
            assert!(serde_json::from_str::<serde_json::Value>(text).is_err());
        }
        for key in ["outOfRangePosition", "invalidDefaultReference"] {
            if let Some(rejection) = vector["rejections"].get(key) {
                let rejected = rejection.get("payload").map(decode).unwrap_or_else(|| payload.clone());
                let rejected_base = rejection.get("base").map(scene_snapshot).unwrap_or_else(|| base.clone());
                assert_eq!(mutation::plan(&rejected, &rejected_base).unwrap_err().code, rejection["code"].as_str().unwrap());
            }
        }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
