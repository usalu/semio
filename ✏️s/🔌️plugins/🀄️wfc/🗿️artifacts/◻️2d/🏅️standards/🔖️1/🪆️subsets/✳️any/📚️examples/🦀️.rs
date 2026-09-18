//! 📚️ The bundled WFC 2D problems this subset ships — a forced path, a cyclic two-relation lattice,
//! a non-rectangular vector ring, and that same ring over 8×8 palette-indexed RASTER tiles (the one
//! example that exercises the `Bitmap` media branch end to end). Each slug owns its own directory; the Rust builder there is the single
//! authority, and `source()` PRINTS the example's document from it rather than carrying a committed
//! `🗣️.dsl.semio` asset beside it.
//!
//! ⚠️ Deviation, deliberate: `🧩️assembly`/`🗒️note` commit a printed text asset and `include_str!` it.
//! That asset is a second copy of the same content with no mechanism keeping it in sync — the
//! remodel ticket's own W8 pass found exactly that drift (a stale `child_id` spelling). Printing
//! from the one Rust authority removes the drift class entirely, and the per-example round-trip test
//! proves the printed text parses back to the very snapshot it came from.

/// 📇️ Every bundled example, in the order the editor's example picker offers them.
pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
    vec![super::two_room_corridor::source(), super::wall_roof_facade_strip::source(), super::hex_ring::source(), super::terrain_ring::source()]
}

/// 📇️ The same roster as a `&'static` slice, for the subset declaration's `examples` field.
pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
    static SOURCES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
    SOURCES.get_or_init(sources).as_slice()
}

/// 📇️ Every bundled example's document, in picker order — the ONE roster a "for every example" test
/// walks, so adding a fifth example cannot quietly leave a render or solve law behind.
pub fn documents() -> Vec<crate::Wfc2dSnapshot> {
    vec![super::two_room_corridor::document(), super::wall_roof_facade_strip::document(), super::hex_ring::document(), super::terrain_ring::document()]
}
