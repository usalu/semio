//! 🧺️ The `🎨️paint` mutation group's leaf fixture tests, run as one target of its group crate.

extern crate semio_framework_os_kernel as protocol;
use semio_s_artifact_lowpoly_lowpoly_mut_laws as laws;
#[path = "../../➕️insert-paint-layer/🧪️tests/🪜️stacks/🦀️.rs"]
mod tests_stacks_a_detail_layer_above_the_base_layer;
#[path = "../../➖️remove-paint-layer/🧪️tests/➖️drops/🦀️.rs"]
mod tests_drops_the_detail_layer_at_index_1;
#[path = "../../🔖️rename-paint-layer/🧪️tests/🏷️retitles/🦀️.rs"]
mod tests_retitles_the_base_layer_to_undercoat;
#[path = "../../👁️change-paint-layer-visible/🧪️tests/🙈️hides/🦀️.rs"]
mod tests_hides_the_base_layer;
#[path = "../../🌫️change-paint-layer-opacity/🧪️tests/🌫️fades/🦀️.rs"]
mod tests_fades_the_base_layer_to_half;
#[path = "../../🎛️change-paint-layer-blend-mode/🧪️tests/✖️switches/🦀️.rs"]
mod tests_switches_the_base_layer_to_multiply;
#[path = "../../🎨️edit-paint-layer/🧪️tests/🖌️paints/🦀️.rs"]
mod tests_paints_red_over_the_second_half_of_the_base_layer;
#[path = "../../🖌️apply-paint-stroke/🧪️tests/🖌️dabs/🦀️.rs"]
mod tests_dabs_red_across_the_base;
#[path = "../../🖌️apply-paint-stroke/🧪️tests/🧽️erases/🦀️.rs"]
mod tests_erases_a_hole_in_the_base;
#[path = "../../🖌️apply-paint-stroke/🧪️tests/⏸️erases/🦀️.rs"]
mod tests_erases_the_clear_layer;
#[path = "../../🖌️apply-paint-stroke/🧪️tests/⛔️paints/🦀️.rs"]
mod tests_paints_a_missing_layer;
#[path = "../../🖌️apply-paint-stroke/🧪️tests/🫓️paints/🦀️.rs"]
mod tests_paints_with_no_radius;
#[path = "../../🖌️apply-paint-stroke/🧪️tests/📐️dabs/🦀️.rs"]
mod tests_dabs_off_the_texture;
