//! 🧫️ Canonical test of the committed `change-zone-window-inclination-deg` vector `📐tilts-south-window-to-60-degrees` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("📐change-zone-window-inclination-deg", "📐tilts-south-window-to-60-degrees");
}
