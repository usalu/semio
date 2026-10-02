//! 📏 Canonical test of the committed `insert-thermal-bridge` vector `📏clamp` — the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🌉️insert-thermal-bridge", "📏clamp");
}
