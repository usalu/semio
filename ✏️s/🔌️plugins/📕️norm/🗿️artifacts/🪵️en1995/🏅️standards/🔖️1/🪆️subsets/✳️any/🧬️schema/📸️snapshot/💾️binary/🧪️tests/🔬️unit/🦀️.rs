use super::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = En1995Snapshot::default();
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[test]
fn regen_example_pack_assets() {
    if std::env::var("EN1995_REGEN_PACKS").ok().as_deref() != Some("1") {
        return;
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    let cases = [
        ("🏠️glulam-floor-beam", En1995Snapshot::compliant_building_beam()),
        ("🌉️glulam-footbridge", En1995Snapshot::compliant_bridge()),
        ("❌️multi-fail-timber", En1995Snapshot::noncompliant_building()),
        ("⚠️overloaded-footbridge", En1995Snapshot::noncompliant_bridge()),
    ];
    for (name, snap) in cases {
        let folder = root.join(name);
        let nested = folder.join(name);
        std::fs::create_dir_all(&nested).unwrap();
        let bytes = crate::standards::v1::subsets::any::schema::snapshot::encode_en1995_pack(&snap);
        let dsl = crate::standards::v1::subsets::any::schema::snapshot::encode_en1995_dsl(&snap);
        for pack_path in [folder.join("🎒️.pack.semio"), nested.join("🎒️.pack.semio")] {
            std::fs::write(&pack_path, &bytes).unwrap();
            eprintln!("wrote {} ({} bytes)", pack_path.display(), bytes.len());
        }
        let dsl_path = nested.join("🗣️.dsl.semio");
        std::fs::write(&dsl_path, &dsl).unwrap();
        eprintln!("wrote {} ({} bytes)", dsl_path.display(), dsl.len());
    }
}
