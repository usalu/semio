use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use crate::standards::v1_4::subsets::base::schema::diff::PdfDiff;
use crate::standards::v1_4::subsets::a::schema::mutations::*;

#[test]
fn direct_descriptor_and_catalog_bijection() {
    let kinds: Vec<_> = <PdfA1Mutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🗄️a/🧬️schema/🧬️mutations");
    let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🔮️oracles/🔣️.json")).unwrap()).unwrap();
    assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(kinds));
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📝️set-page-text").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[0]);
        assert!(source.join("📝️set-page-text").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🧹️clear-page-text").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[1]);
        assert!(source.join("🧹️clear-page-text").join("🦀️.rs").is_file());
    }
}
