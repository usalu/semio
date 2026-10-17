//! 🧪️ `apply-document` inverse law on its committed wrapped-arm vector: the wrapped mutation is undone by its own inverse rows, replayed
//! last-to-first, and those rows' diffs sum to the negative of the forward diff.

use crate::io::text::mutations::decode_semio_mutation_json;
use crate::io::text::snapshot::decode_semio_snapshot_json;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/📑️apply-document-applied/⬅️before.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/📑️apply-document-applied/🦠️mutation.json"));

#[semio_framework_async_macros::async_test]
async fn the_wrapped_document_mutation_inverts_and_sums_to_the_negative_diff() {
    let base = decode_semio_snapshot_json((*BEFORE)).expect("committed before-envelope decodes");
    let mutation = decode_semio_mutation_json((*MUTATION)).expect("committed wrapped mutation decodes");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
