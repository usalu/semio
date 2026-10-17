//! 🧪️ The pptx model laws, as an integration target over the artifact's public API.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
mod inferences_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
mod outline_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️shadow/🦀️.rs"]
mod shadow_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧭️transform/🧪️tests/🦀️.rs"]
mod transform_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_tests;

/// 📥️ Run-time readers of the committed test fixtures, with paths relative to the artifact folder: the mounted laws read them here instead of embedding them with `include_bytes!` or `include_str!`, so the test closure stays inside the source gate.
/// @see <https://doc.rust-lang.org/std/macro.include_bytes.html>
mod runtime_fixture {
    fn path(relative: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(relative)
    }

    pub(crate) fn bytes(relative: &str) -> Vec<u8> {
        let path = path(relative);
        std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
    }

    pub(crate) fn text(relative: &str) -> String {
        let path = path(relative);
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
    }
}
