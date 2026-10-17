use crate::{forms_steps, FormsSnapshot};
use semio_framework_test_fixtures::fixture_text;

#[test]
fn definition_and_answers_survive_json_text_and_pack_restart() {
    let input = fixture_text!("../../../🧪️tests/🧫️fixtures/🔬️contract/💾️persistence/🔣️.json");
    let snapshot: FormsSnapshot = semio_framework_pack_json::from_json_str(input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(forms_steps(&snapshot).len(), 1);
    assert_eq!(snapshot.responses.len(), 1);
    let json = semio_framework_pack_json::to_json_string(&snapshot);
    assert_eq!(semio_framework_pack_json::from_json_str::<FormsSnapshot>(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), snapshot);
    let text = <FormsSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snapshot);
    let from_text = <FormsSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).unwrap();
    assert_eq!(from_text, snapshot);
    let packed = <FormsSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let decoded = <FormsSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&packed).unwrap();
    assert_eq!(decoded.definition, snapshot.definition);
    assert_eq!(decoded.responses, snapshot.responses);
    assert_eq!(forms_steps(&decoded), forms_steps(&snapshot));
}
