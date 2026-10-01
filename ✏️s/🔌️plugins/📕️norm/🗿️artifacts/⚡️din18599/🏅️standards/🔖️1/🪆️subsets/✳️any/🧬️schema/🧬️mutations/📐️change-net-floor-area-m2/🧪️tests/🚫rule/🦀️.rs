//! 🚫 Canonical test of the committed `change-net-floor-area-m2` vector `🚫rule` — the canonical change asked for a value below its leaf schema's bound is a `mutation.invariant` refusal.

#[test]
fn committed_vector_holds() {
    super::assert_vector("📐️change-net-floor-area-m2", "🚫rule");
}
