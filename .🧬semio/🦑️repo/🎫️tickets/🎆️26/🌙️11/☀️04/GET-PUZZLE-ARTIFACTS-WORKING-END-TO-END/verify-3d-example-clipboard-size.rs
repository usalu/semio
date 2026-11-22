//! 📏️ Measures the clipboard payload for every placed object in the shipped Nakagin example.

use semio_framework_pack_json::{from_dsl_value, json};
use semio_s_artifact_puzzle_3d_editor_snapshot::{
    puzzle3d_copy_fragment_from, puzzle3d_snapshot_from_host_snapshot, NAKAGIN_EXAMPLE_SNAPSHOT,
};

fn main() {
    let document = puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT)
        .expect("the authored example admits to the production model");
    let objects: Vec<_> = document.objects.iter().collect();
    let fragment = puzzle3d_copy_fragment_from(&document, &objects).expect("copy every object");
    let arguments = json!({
        "fragment": from_dsl_value(&fragment.to_value()),
        "anchor": "centroid",
        "position": [0, 0, 0],
    });
    let payload = semio_framework_pack_json::parse(
        &fragment.dsl_text,
        semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).expect("production fragment is valid JSON");
    let copied_objects = payload.get("objects").and_then(|value| value.as_array()).unwrap().len();
    let copied_attractions = payload.get("attractions").and_then(|value| value.as_array()).unwrap().len();
    assert_eq!(copied_objects, objects.len());
    assert_eq!(copied_attractions, document.attractions.len());
    println!(
        "[DEBUG] {copied_objects} objects, {copied_attractions} attractions, {} fragment bytes, {} JSON paste-argument bytes",
        fragment.dsl_text.len(),
        arguments.to_string().len(),
    );
}
