//! 🧪️ `🖼️apply-image` fixture — `🚫️refuses`: a wrapped image set-dimensions
//! mutation against a value envelope names another arm, so the envelope refuses it with
//! `mutation.target-missing` and stays exactly as it stood. Source of truth is the committed JSON
//! bundle under `../../../../../🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses/`,
//! whose `🔺️diff/🚫️.absent` records that no diff exists.

use {semio_s_artifact_stdio_semio_base::schema::mutations::outcome::semio_mutation_refusal_codes, crate::schema::mutations::SemioMutation};
use crate::io::text::mutations::decode_semio_mutation_json;
use crate::schema::snapshot::SemioSnapshot;
use crate::io::text::snapshot::decode_semio_snapshot_json;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses/🎯️outcome/🔣️.json"));

fn before() -> SemioSnapshot {
    decode_semio_snapshot_json((*BEFORE)).expect("before envelope decodes")
}
fn mutation() -> SemioMutation {
    decode_semio_mutation_json((*MUTATION)).expect("wrapped image mutation decodes")
}

/// 🚫️ The mismatched arm is refused with exactly the committed code and leaves the envelope untouched.
#[semio_framework_async_macros::async_test]
async fn refuses_with_the_committed_code() {
    let outcome: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "semio-base/apply-image: this fixture declares a rejected outcome");
    let mut snapshot = before();
    let (__next, produced) = semio_s_artifact_stdio_semio_base::applied(&snapshot, &mutation());
    snapshot = __next;
    assert_eq!(semio_mutation_refusal_codes(&produced), vec![outcome.get("code").and_then(serde_json::Value::as_str).expect("outcome carries a code").to_string()], "semio-base/apply-image: the refusal code differs from the committed outcome");
    assert_eq!(snapshot, decode_semio_snapshot_json((*AFTER)).expect("after envelope decodes"), "semio-base/apply-image: a refused mutation must leave the committed envelope as it stood");
    assert_eq!(snapshot, before(), "semio-base/apply-image: the committed after-envelope must equal the before-envelope");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
