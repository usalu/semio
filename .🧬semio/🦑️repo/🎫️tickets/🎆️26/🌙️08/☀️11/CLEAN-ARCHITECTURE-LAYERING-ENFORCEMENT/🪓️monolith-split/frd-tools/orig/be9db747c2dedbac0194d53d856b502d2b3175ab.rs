//! 🧪️ `apply-video` inverse law on its committed wrapped-arm vector: the wrapped mutation is undone by its own inverse rows, replayed
//! last-to-first, and those rows' diffs sum to the negative of the forward diff.

use crate::io::text::mutations::decode_semio_mutation_json;
use crate::io::text::snapshot::decode_semio_snapshot_json;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| Box::leak(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🎬️apply-video-applied/⬅️before.json")).expect("the committed fixture is readable").into_boxed_str()));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| Box::leak(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🎬️apply-video-applied/🦠️mutation.json")).expect("the committed fixture is readable").into_boxed_str()));

#[semio_framework_async_macros::async_test]
async fn the_wrapped_video_mutation_inverts_and_sums_to_the_negative_diff() {
    let base = decode_semio_snapshot_json(*BEFORE).expect("committed before-envelope decodes");
    let mutation = decode_semio_mutation_json(*MUTATION).expect("committed wrapped mutation decodes");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
