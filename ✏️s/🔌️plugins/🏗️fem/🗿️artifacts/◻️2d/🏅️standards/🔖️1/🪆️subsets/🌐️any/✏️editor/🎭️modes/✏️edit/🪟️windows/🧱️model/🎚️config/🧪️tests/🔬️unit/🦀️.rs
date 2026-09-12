use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn fem2d_window_config_model_matches_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json")).expect("FEM window fixture");
    let base: Fem2dModelWindowConfig = dsl::json::from_json_str(&fixture["valid"][0].to_string()).expect("neutral FEM window config");
    let mutation = Fem2dModelWindowConfigMutation::Snapshot { config: Box::new(base.clone()) };
    let after = mutation.diff(&base).diff().apply(&base).expect("FEM window diff");
    assert_eq!(after, base);
    assert_eq!(Fem2dModelWindowConfigMutation::parse_op(&mutation.print_op()).expect("FEM text mutation"), mutation);
    assert_eq!(Fem2dModelWindowConfigMutation::decode_op(&mutation.encode_op().expect("FEM binary mutation")).expect("FEM decoded mutation"), mutation);
    let text = store::ArtifactDsl::print_dsl(&base);
    assert_eq!(<Fem2dModelWindowConfig as store::ArtifactDsl>::parse_dsl(&text).expect("FEM window text"), base);
    let bytes = store::ArtifactPack::encode_pack(&base);
    assert_eq!(<Fem2dModelWindowConfig as store::ArtifactPack>::decode_pack(&bytes).expect("FEM window pack"), base);
    let mut admitted_invalid = Vec::new();
    for row in fixture["invalid"].as_array().expect("neutral invalid cases") {
        let kind = row["kind"].as_str().expect("neutral invalid case kind");
        let mut candidate = fixture["valid"][0].clone();
        match kind {
            "unknown" => candidate["locale"] = serde_json::json!("de"),
            "camera-zero" => candidate["camera"]["zoom"] = serde_json::json!(0),
            "camera-negative" => candidate["camera"]["zoom"] = serde_json::json!(-1),
            "camera-null" => candidate["camera"] = serde_json::Value::Null,
            "camera-unknown" => candidate["camera"]["extra"] = serde_json::json!(true),
            "bad-mode" => candidate["resultMode"] = serde_json::json!("harmonic"),
            _ => panic!("unknown neutral invalid case {kind}"),
        }
        if dsl::json::from_json_str::<Fem2dModelWindowConfig>(&candidate.to_string()).is_ok() {
            admitted_invalid.push(kind);
        }
    }
    assert!(admitted_invalid.is_empty(), "FEM native admitted invalid neutral cases: {admitted_invalid:?}");
    eprintln!("[DEBUG] FEM 2D model window config matched neutral fixture and codecs");
}
