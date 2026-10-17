//! 📚️ The bundled problem specs this subset ships — a forced corridor path, a cyclic facade strip,
//! and a non-boxed tower. Each slug owns its own directory under `📚️examples/`, and the Rust builder
//! there is the authority its `🗣️.dsl.semio` asset is printed from.

#[path = "🗼️tower-stack/🦀️.rs"]
pub mod tower_stack;
#[path = "🚪️two-room-corridor/🦀️.rs"]
pub mod two_room_corridor;
#[path = "🧱️wall-roof-facade-strip/🦀️.rs"]
pub mod wall_roof_facade_strip;

/// 📇️ Every bundled example, in the order the editor's example picker offers them.
pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
    vec![two_room_corridor::source(), wall_roof_facade_strip::source(), tower_stack::source()]
}

/// 📇️ The same roster as a `&'static` slice, for the subset declaration.
pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
    static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
    EXAMPLES.get_or_init(sources).as_slice()
}
