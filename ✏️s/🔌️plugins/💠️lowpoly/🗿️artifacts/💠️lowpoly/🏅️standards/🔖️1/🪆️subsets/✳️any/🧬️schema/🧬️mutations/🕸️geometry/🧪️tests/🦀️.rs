//! 🧺️ The `🕸️geometry` mutation group's leaf fixture tests, run as one target of its group crate.

extern crate semio_framework_os_kernel as protocol;
#[path = "../../🧪️tests/⚖️laws/🦀️.rs"]
pub mod laws;
#[path = "../../🕸️create-mesh/🧪️tests/🕸️attaches/🦀️.rs"]
mod tests_attaches_a_mesh_child_handle_to_obj_fin;
#[path = "../../🧨delete-mesh/🧪️tests/✂️detaches/🦀️.rs"]
mod tests_detaches_the_mesh_child_handle_from_obj_hull;
#[path = "../../🚚️move-selection/🧪️tests/🚚️moves/🦀️.rs"]
mod tests_moves;
#[path = "../../🚚️move-selection/🧪️tests/📌️pins/🦀️.rs"]
mod tests_pins;
#[path = "../../🚚️move-selection/🧪️tests/🧩️part/🦀️.rs"]
mod tests_part;
#[path = "../../🚚️move-selection/🧪️tests/⏸️still/🦀️.rs"]
mod tests_still;
#[path = "../../🚚️move-selection/🧪️tests/⛔️ghost/🦀️.rs"]
mod tests_ghost;
#[path = "../../🚚️move-selection/🧪️tests/🕳️bare/🦀️.rs"]
mod tests_bare;
#[path = "../../🚚️move-selection/🧪️tests/🔁️twice/🦀️.rs"]
mod tests_twice;
#[path = "../../🌀️rotate-selection/🧪️tests/🌀️turns/🦀️.rs"]
mod tests_turns;
#[path = "../../🌀️rotate-selection/🧪️tests/📐️lifts/🦀️.rs"]
mod tests_lifts;
#[path = "../../🌀️rotate-selection/🧪️tests/⏸️still/🦀️.rs"]
mod tests_still;
#[path = "../../🌀️rotate-selection/🧪️tests/🫥️void/🦀️.rs"]
mod tests_void;
#[path = "../../🌀️rotate-selection/🧪️tests/⛔️ghost/🦀️.rs"]
mod tests_ghost;
#[path = "../../🔍️scale-selection/🧪️tests/🔍️wides/🦀️.rs"]
mod tests_wides;
#[path = "../../🔍️scale-selection/🧪️tests/🧩️part/🦀️.rs"]
mod tests_part;
#[path = "../../🔍️scale-selection/🧪️tests/⏸️still/🦀️.rs"]
mod tests_still;
#[path = "../../🔍️scale-selection/🧪️tests/🫓️flat/🦀️.rs"]
mod tests_flat;
#[path = "../../📍️set-vertex-positions/🧪️tests/📍️sets/🦀️.rs"]
mod tests_sets;
