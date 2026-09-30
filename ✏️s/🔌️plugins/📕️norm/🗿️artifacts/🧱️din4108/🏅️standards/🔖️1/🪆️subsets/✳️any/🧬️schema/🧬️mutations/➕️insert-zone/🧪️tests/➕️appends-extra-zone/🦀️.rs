//! 🧫️ Canonical test of the committed `insert-zone` vector `➕️appends-extra-zone` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("➕️insert-zone", "➕️appends-extra-zone");
}
