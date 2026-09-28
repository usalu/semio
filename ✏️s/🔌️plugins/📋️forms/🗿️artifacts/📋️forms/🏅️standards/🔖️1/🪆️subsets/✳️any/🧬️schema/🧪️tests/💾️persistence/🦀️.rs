use crate::{forms_steps, FormsSnapshot};

#[test]
fn definition_and_answers_survive_json_text_and_pack_restart() {
    let input = include_str!("../../🧫️fixtures/💾️persistence/🔣️.json");
    let snapshot: FormsSnapshot = dsl::json::from_json_str(input).unwrap();
    assert_eq!(forms_steps(&snapshot).len(), 1);
    assert_eq!(snapshot.responses.len(), 1);
    let json = dsl::os_pack::json::to_json_string(&snapshot);
    assert_eq!(dsl::json::from_json_str::<FormsSnapshot>(&json).unwrap(), snapshot);
    let text = <FormsSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let from_text = <FormsSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap();
    assert_eq!(from_text, snapshot);
    let packed = <FormsSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&packed).unwrap();
    assert_eq!(decoded.definition, snapshot.definition);
    assert_eq!(decoded.responses, snapshot.responses);
    assert_eq!(forms_steps(&decoded), forms_steps(&snapshot));
    println!("[DEBUG] Forms definition and responses survived an independent binary decode");
}
