//! 🟰 Canonical test of the committed `change-t-int-c` vector `🟰noop` — re-applying the canonical change sets the value the field already has, a `mutation.no-op`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🌡️change-t-int-c", "🟰noop");
}
