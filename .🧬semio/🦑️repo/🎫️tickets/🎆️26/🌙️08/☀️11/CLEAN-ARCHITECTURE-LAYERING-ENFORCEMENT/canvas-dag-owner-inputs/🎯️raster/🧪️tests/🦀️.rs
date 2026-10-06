use super::Target;

#[test]
fn owned_canvas_raster_target_matches_shared_exact_extent_and_byte_contract() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture.get("cases").unwrap().as_array().unwrap() {
        let width = u32::try_from(row.get("width").unwrap().as_u64().unwrap()).unwrap();
        let height = u32::try_from(row.get("height").unwrap().as_u64().unwrap()).unwrap();
        match Target::try_new(width, height) {
            Ok(target) => {
                assert_eq!(target.width(), width);
                assert_eq!(target.height(), height);
                assert_eq!(Some(target.bytes().to_string()).as_deref(), row.get("bytes").unwrap().as_str());
                assert!(row.get("refusal").unwrap().is_null());
            }
            Err(fault) => {
                assert_eq!(Some(fault.code()), row.get("refusal").unwrap().as_str());
                assert!(row.get("bytes").unwrap().is_null());
            }
        }
    }
    println!("[DEBUG] owned Canvas raster target eight shared controls");
}
