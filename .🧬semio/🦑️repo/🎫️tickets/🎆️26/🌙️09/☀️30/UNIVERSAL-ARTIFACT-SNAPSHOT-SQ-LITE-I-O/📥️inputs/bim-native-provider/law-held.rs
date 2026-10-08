#[test]
fn sqlite_snapshot_bim_original_bare_and_public_io_require_native_provider(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪶️native-provider/🔣️.json")).unwrap();assert_eq!(fixture["artifactSchema"],crate::BIM_MODEL_DOCUMENT_SCHEMA);assert_eq!(fixture["nativeRootFields"],26);assert_eq!(fixture["sortedCollections"],24);
    let codec=store::ArtifactCodec::bare::<crate::ModelSnapshot,crate::ModelMutation>(crate::BIM_MODEL_DOCUMENT_SCHEMA);assert!(codec.snapshot_sqlite.is_some(),"original BIM bare artifact has no authored SQLite snapshot provider");let declaration=io();assert_eq!(declaration.native.codec.schema,crate::BIM_MODEL_DOCUMENT_SCHEMA);assert!(declaration.native.codec.snapshot_sqlite.is_some(),"original BIM public IO omits its actual native SQLite provider");eprintln!("[DEBUG] original BIM bare/public IO owns the authored Native SQLite capability; complete relational/native family runtime remains separate");
}
