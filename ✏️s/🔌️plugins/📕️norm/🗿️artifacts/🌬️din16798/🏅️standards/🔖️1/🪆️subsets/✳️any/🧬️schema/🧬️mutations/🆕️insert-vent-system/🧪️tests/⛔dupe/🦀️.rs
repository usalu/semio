//! ⛔ Canonical test of the committed `insert-vent-system` vector `⛔dupe` — re-applying the canonical insert repeats an id the document already holds, a `mutation.duplicate-id`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🆕️insert-vent-system", "⛔dupe");
}
