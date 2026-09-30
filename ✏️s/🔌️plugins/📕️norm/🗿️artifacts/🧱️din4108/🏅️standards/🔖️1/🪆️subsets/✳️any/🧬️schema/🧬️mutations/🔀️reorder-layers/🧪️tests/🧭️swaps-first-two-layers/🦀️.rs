//! 🧫️ Canonical test of the committed `reorder-layers` vector `🧭️swaps-first-two-layers` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🔀️reorder-layers", "🧭️swaps-first-two-layers");
}
