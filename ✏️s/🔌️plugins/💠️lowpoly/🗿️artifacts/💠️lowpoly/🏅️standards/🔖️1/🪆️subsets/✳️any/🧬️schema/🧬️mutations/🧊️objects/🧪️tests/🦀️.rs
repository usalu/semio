//! 🧺️ The `🧊️objects` mutation group's leaf fixture tests, run as one target of its group crate.

extern crate semio_framework_os_kernel as protocol;
use semio_s_artifact_lowpoly_lowpoly_mut_laws as laws;
#[path = "../../🌱️create-object/🧪️tests/⛵️inserts/🦀️.rs"]
mod tests_inserts_obj_mast_between_hull_and_fin;
#[path = "../../💀️delete-object/🧪️tests/🚫️removes/🦀️.rs"]
mod tests_removes_obj_fin_without_touching_the_order;
#[path = "../../🔀️reorder-objects/🧪️tests/🔀️moves/🦀️.rs"]
mod tests_moves_obj_fin_in_front_of_obj_hull;
#[path = "../../🏷️rename-object/🧪️tests/🏷️retitles/🦀️.rs"]
mod tests_retitles_obj_hull;
#[path = "../../🔘️change-object-smooth/🧪️tests/🟢️turns/🦀️.rs"]
mod tests_turns_on_smooth_shading_for_obj_hull;
#[path = "../../↗️move-object/🧪️tests/📍️translates/🦀️.rs"]
mod tests_translates_obj_hull_along_x_and_z;
#[path = "../../🔄️rotate-object/🧪️tests/🔄️yaws/🦀️.rs"]
mod tests_yaws_obj_hull_about_the_y_axis;
#[path = "../../📐️scale-object/🧪️tests/📐️halves/🦀️.rs"]
mod tests_halves_obj_hull_uniformly;
