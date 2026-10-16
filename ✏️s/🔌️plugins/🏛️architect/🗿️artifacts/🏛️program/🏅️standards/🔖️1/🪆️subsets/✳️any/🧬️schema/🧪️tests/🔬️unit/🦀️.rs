use super::*;

#[semio_framework_async_macros::async_test]
async fn normalize_pair_orders_endpoints() {
    let a = EntityId("element-2".into());
    let b = EntityId("element-10".into());
    assert_eq!(normalize_pair(&b, &a), (b, a));
}

/// 🔏️ The JSON Schema documents the descriptor serves are declared as package assets with their exact length and a well-formed
/// SHA-256 (the `📦️package-asset-constants` law recomputes the digests independently), and a host-less run resolves each of them
/// through the reader the descriptor installs.
#[test]
fn the_served_schema_documents_are_declared_with_their_exact_size() {
    use semio_framework_schema_registry::{resolve_schema_export, SchemaFormat};
    let declarations = schema_leaf_asset_declarations();
    assert_eq!(declarations.len(), 4);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|candidate| candidate.join("nx.json").is_file()).expect("the repository root holds nx.json");
    for declaration in &declarations {
        let bytes = std::fs::read(root.join(&declaration.name)).expect("the declared document exists");
        assert_eq!(bytes.len() as u64, declaration.size_bytes, "{}", declaration.name);
        assert_eq!(declaration.sha256.len(), 64, "{}", declaration.name);
    }
    semio_framework_schema_registry::register_artifact_schema_descriptor(program_artifact_schema_descriptor()).expect("the descriptor registers");
    for export in ["artifact", "snapshot", "diff", "mutations"] {
        let served = resolve_schema_export("s.architect.program", export, SchemaFormat::JsonSchema).unwrap_or_else(|error| panic!("{export}: {error}"));
        assert!(served.trim_start().starts_with('{'), "{export} resolves to the JSON Schema document");
    }
}

