//! 🟰 Canonical test of the committed `change-airtightness-n50` vector `🟰noop` — re-applying the canonical change sets the value the field already has, a `mutation.no-op`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("💨️change-airtightness-n50", "🟰noop");
}
