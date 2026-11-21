//! 🪪 Original Store snapshot authority proof using the repository's actual paid lease registry.
use semio_framework_value_snapshot_read::SnapshotReadLeaseRegistry;
use serde_json::Value;

fn revision(value: &Value) -> [u8; 32] {
    let hex = value.as_str().unwrap().as_bytes();
    let digits = |byte| match byte { b'0'..=b'9' => byte-b'0', b'a'..=b'f' => 10+byte-b'a', _ => panic!("noncanonical hex") };
    std::array::from_fn(|i| digits(hex[2*i])*16 + digits(hex[2*i+1]))
}

#[test]
fn genesis_and_next_generation_publish_complete_revision() {
    let fixture: Value = serde_json::from_slice(&std::fs::read(std::env::var_os("PDF_STORE_AUTHORITY_LAW").unwrap()).unwrap()).unwrap();
    let registry = SnapshotReadLeaseRegistry::new();
    let generation = fixture["generation"].as_u64().unwrap();
    let expected = revision(&fixture["revisionHex"]);
    assert!(!registry.authority_matches(generation, expected), "the earlier cold constructor left this exact stamp unknown");
    assert!(registry.publish_authority(generation, expected));
    assert!(registry.authority_matches(generation, expected));
    for i in 0..expected.len() {
        let mut changed = expected;
        changed[i] ^= 1;
        assert!(!registry.authority_matches(generation, changed));
    }
    let next = fixture["nextGeneration"].as_u64().unwrap();
    let successor = revision(&fixture["nextRevisionHex"]);
    assert!(!registry.authority_matches(next, expected));
    assert!(registry.publish_authority(next, successor));
    assert!(!registry.authority_matches(generation, expected));
    assert!(registry.authority_matches(next, successor));
}
