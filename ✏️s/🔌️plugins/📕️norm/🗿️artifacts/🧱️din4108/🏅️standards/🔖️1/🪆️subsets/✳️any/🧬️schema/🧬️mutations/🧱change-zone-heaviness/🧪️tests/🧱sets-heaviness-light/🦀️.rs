//! 🧫️ Canonical test of the committed `change-zone-heaviness` vector `🧱sets-heaviness-light` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🧱change-zone-heaviness", "🧱sets-heaviness-light");
}
