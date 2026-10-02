//! 🟰 Canonical test of the committed `change-has-mechanical-ventilation` vector `🟰noop` — re-applying the canonical change sets the value the field already has, a `mutation.no-op`.

#[test]
fn committed_vector_holds() {
    super::assert_vector("💨change-has-mechanical-ventilation", "🟰noop");
}
