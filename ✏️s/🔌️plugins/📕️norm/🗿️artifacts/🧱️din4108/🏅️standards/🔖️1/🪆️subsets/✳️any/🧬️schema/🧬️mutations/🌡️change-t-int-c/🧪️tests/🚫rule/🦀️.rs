//! 🚫 Canonical test of the committed `change-t-int-c` vector `🚫rule` — the canonical change asked for a value below its leaf schema's bound is a `mutation.invariant` refusal.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🌡️change-t-int-c", "🚫rule");
}
