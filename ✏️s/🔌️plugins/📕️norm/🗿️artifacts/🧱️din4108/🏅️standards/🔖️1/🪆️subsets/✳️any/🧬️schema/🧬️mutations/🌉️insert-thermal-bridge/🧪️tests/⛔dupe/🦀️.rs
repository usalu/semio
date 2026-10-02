//! ⛔ Canonical test of the committed `insert-thermal-bridge` vector `⛔dupe` — re-applying the canonical insert repeats an id the document already holds, a `mutation.duplicate-id`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🌉️insert-thermal-bridge", "⛔dupe");
}
