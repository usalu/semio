//! 🌲️ Example `concrete-forest` — two Hexagonal Cut Concrete Forest Left pieces stacked on top of
//! each other, modelled with nodes and line (`Frame`) elements only.
//!
//! 🧭️ Member topology follows the CAD play fixture's `aec.building.structure.classic` model
//! (`✏️s/🔌️plugins/📐️cad/…/📚️examples/🖼️assets/🎮️play/🔣️.json`, model 3): per piece two
//! hexagonal columns (`c1` at x 2.7, `c2` at x 8.1, both y 2.338), seven cantilever beams fanning
//! out to the hexagonal-cut edges, and one spine beam joining the column heads; the one-way slab is
//! carried as tributary member UDLs on those beams (thickness 0.265 m from the `aec.building` slab).
//! Node coordinates are the puzzle3d `🌲️concrete-forest` vortices (`seed-left-001:v0..v10`) so the
//! upper piece's `c-b` vortices land exactly on the lower piece's `c-t` vortices — the same joint the
//! puzzle's `c-b`/`c-t` kind-compatibility row expresses.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "concrete-forest";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Concrete Forest", "Betonwald")
}
pub const ICON: &str = "list-tree";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
