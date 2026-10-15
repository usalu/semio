//! 🔧️ `set-railing` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️reshapes/🦀️.rs"]
mod tests_reshapes;
#[cfg(test)]
#[path = "🧪️tests/🏷️renames-only/🦀️.rs"]
mod tests_renames_only;
#[cfg(test)]
#[path = "🧪️tests/💤️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/📏️path-too-short/🦀️.rs"]
mod tests_path_too_short;
#[cfg(test)]
#[path = "🧪️tests/💥️non-positive-height/🦀️.rs"]
mod tests_non_positive_height;
#[cfg(test)]
#[path = "🧪️tests/🚧️non-positive-post-spacing/🦀️.rs"]
mod tests_non_positive_post_spacing;
#[cfg(test)]
#[path = "🧪️tests/🧱️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧮️adds-balusters-and-glass/🦀️.rs"]
mod tests_adds_balusters_and_glass;
#[cfg(test)]
#[path = "🧪️tests/🧯️removes-the-balusters/🦀️.rs"]
mod tests_removes_the_balusters;
#[cfg(test)]
#[path = "🧪️tests/🧰️post-profile-degenerate/🦀️.rs"]
mod tests_post_profile_degenerate;
#[cfg(test)]
#[path = "🧪️tests/🧲️hosts-on-a-stair/🦀️.rs"]
mod tests_hosts_on_a_stair;
#[cfg(test)]
#[path = "🧪️tests/🧳️hosts-on-a-ramp/🦀️.rs"]
mod tests_hosts_on_a_ramp;
#[cfg(test)]
#[path = "🧪️tests/🧴️hosts-on-a-slab-edge/🦀️.rs"]
mod tests_hosts_on_a_slab_edge;
#[cfg(test)]
#[path = "🧪️tests/🧶️releases-the-host/🦀️.rs"]
mod tests_releases_the_host;
#[cfg(test)]
#[path = "🧪️tests/🧷️moves-to-another-host/🦀️.rs"]
mod tests_moves_to_another_host;
#[cfg(test)]
#[path = "🧪️tests/🧸️host-without-clearing-the-path/🦀️.rs"]
mod tests_host_without_clearing_the_path;
#[cfg(test)]
#[path = "🧪️tests/🧺️releasing-without-a-path/🦀️.rs"]
mod tests_releasing_without_a_path;
#[cfg(test)]
#[path = "🧪️tests/🧻️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/🧼️host-is-no-stair-ramp-or-slab/🦀️.rs"]
mod tests_host_is_no_stair_ramp_or_slab;
#[cfg(test)]
#[path = "🧪️tests/🪐️slab-edge-missing/🦀️.rs"]
mod tests_slab_edge_missing;
#[cfg(test)]
#[path = "🧪️tests/🪒️slab-edge-curved/🦀️.rs"]
mod tests_slab_edge_curved;
#[cfg(test)]
#[path = "🧪️tests/🪓️stair-host-with-an-edge-index/🦀️.rs"]
mod tests_stair_host_with_an_edge_index;
#[cfg(test)]
#[path = "🧪️tests/🪔️negative-host-inset/🦀️.rs"]
mod tests_negative_host_inset;
//#endregion 🔖️Leaves
