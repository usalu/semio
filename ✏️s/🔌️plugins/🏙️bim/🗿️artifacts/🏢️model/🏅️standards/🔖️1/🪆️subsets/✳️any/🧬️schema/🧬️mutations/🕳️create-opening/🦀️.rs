//! 🕳️ `create-opening` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-window/🦀️.rs"]
mod tests_adds_a_window;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-door-to-a-curtain-wall/🦀️.rs"]
mod tests_adds_a_door_to_a_curtain_wall;
#[cfg(test)]
#[path = "🧪️tests/🟢️adds-a-void/🦀️.rs"]
mod tests_adds_a_void;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/❌️negative-sill/🦀️.rs"]
mod tests_negative_sill;
#[cfg(test)]
#[path = "🧪️tests/❗️non-positive-width/🦀️.rs"]
mod tests_non_positive_width;
#[cfg(test)]
#[path = "🧪️tests/📛️beyond-the-host-end/🦀️.rs"]
mod tests_beyond_the_host_end;
#[cfg(test)]
#[path = "🧪️tests/🔴️overlaps-a-neighbour/🦀️.rs"]
mod tests_overlaps_a_neighbour;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️adds-a-window-with-a-raised-sill/🦀️.rs"]
mod tests_adds_a_window_with_a_raised_sill;
#[cfg(test)]
#[path = "🧪️tests/🧯️adds-a-window-with-a-reveal/🦀️.rs"]
mod tests_adds_a_window_with_a_reveal;
#[cfg(test)]
#[path = "🧪️tests/🧰️negative-reveal-depth/🦀️.rs"]
mod tests_negative_reveal_depth;
#[cfg(test)]
#[path = "🧪️tests/🧲️reveal-material-missing/🦀️.rs"]
mod tests_reveal_material_missing;
//#endregion 🔖️Leaves
