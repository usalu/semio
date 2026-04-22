// Load the Metabolism fixture (C#/JS export) and assert semio DTOs + KitStore materialize.
use semio::kit::{KitFullDto, KitStore};

#[test]
fn metabolism_kit_json_deserializes_and_hydrates() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets")
        .join("semio")
        .join("metabolism.kit.semio.json");
    let s = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let dto: KitFullDto = serde_json::from_str(&s).expect("metabolism JSON must deserialize to KitFullDto");
    let kit = KitStore::from_full_dto(dto);
    drop(kit);
}
