//! ❓ Canonical test of the committed `remove-vent-system` vector `❓gone` — re-applying the canonical remove addresses the record it already removed, a `mutation.target-missing`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🗑️remove-vent-system", "❓gone");
}
