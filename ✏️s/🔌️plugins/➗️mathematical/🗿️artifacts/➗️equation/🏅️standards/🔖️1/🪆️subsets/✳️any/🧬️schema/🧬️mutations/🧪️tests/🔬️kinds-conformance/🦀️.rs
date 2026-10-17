use crate::EquationSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;

/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in SOME owning
/// subset's committed oracle manifest. The framework never parses Rust, so this is what keeps
/// the declaration honest in both directions at once.
///
/// 🪆️ Reads all four subset manifests, not just this catalog's own `✳️any`: since ticket
/// 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION
/// split every mutation kind out to its real owning subset (`✳️graph`/`✳️geometry`/
/// `✳️equation`), `✳️any` itself owns none of them any more — this catalog just stays put as
/// the closed enum every subset's manifest is measured against.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifests = [runtime_fixture_text("../../../../🔮️oracles/🔣️.json"), runtime_fixture_text("../../../../../🕸️graph/🔮️oracles/🔣️.json"), runtime_fixture_text("../../../../../📐️geometry/🔮️oracles/🔣️.json"), runtime_fixture_text("../../../../../➗️equation/🔮️oracles/🔣️.json")];
    for kind in KINDS {
        let needle = format!("\"{kind}\"");
        assert!(manifests.iter().any(|manifest| manifest.contains(&needle)), "KINDS entry {kind:?} must also appear in one of the owning subsets' committed oracle manifests");
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
