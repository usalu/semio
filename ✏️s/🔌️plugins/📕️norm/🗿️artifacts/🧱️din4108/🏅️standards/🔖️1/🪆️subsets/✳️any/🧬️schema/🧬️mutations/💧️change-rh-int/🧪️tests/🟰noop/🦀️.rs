//! 🟰 Canonical test of the committed `change-rh-int` vector `🟰noop` — re-applying the canonical change sets the value the field already has, a `mutation.no-op`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("💧️change-rh-int", "🟰noop");
}
