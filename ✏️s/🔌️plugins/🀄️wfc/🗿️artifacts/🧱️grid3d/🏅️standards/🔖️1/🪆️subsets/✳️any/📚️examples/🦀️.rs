//! 📚️ The bundled tile sets this subset ships — one architectural block set on a non-uniform
//! 4×3×4 grid, one pipe network. Each slug owns its own directory under
//! `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`, and the Rust builder there is the authority its
//! `🗣️.dsl.semio` asset is printed from.

#[path = "🧱️blocks/🦀️.rs"]
pub mod blocks;
#[path = "🪠️pipes-3d/🦀️.rs"]
pub mod pipes_3d;

/// 📇️ Every bundled example, in the order the editor's example picker offers them. Append-only:
/// a new example is added at the END so a stored picker index never re-points.
pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
    vec![blocks::source(), pipes_3d::source()]
}

/// 📇️ The same roster as a leaked `&'static` slice, for the subset declaration.
pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
    static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
    EXAMPLES.get_or_init(sources).as_slice()
}
