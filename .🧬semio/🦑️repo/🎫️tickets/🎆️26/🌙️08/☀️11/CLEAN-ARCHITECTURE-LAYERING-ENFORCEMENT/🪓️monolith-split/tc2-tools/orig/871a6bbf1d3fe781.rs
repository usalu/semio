//! 🔀️ The `transform` subset of the drawing artifact: the placement and geometry mutation leaves (transform, drag, rotate, scale, path points, shape coordinates, boolean operation, trace parameters).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod drag_layers {
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/🧪️tests/✋️drags-a-child/🦀️.rs"]
    mod tests_drags_a_child;
}
#[path = "."]
pub mod drag_path_points {
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/🧪️tests/📍️drags/🦀️.rs"]
    mod tests_drags_two_anchors;
}
#[path = "."]
pub mod rotate_layers {
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/🧪️tests/🧭️quarter-turn/🦀️.rs"]
    mod tests_quarter_turn;
}
#[path = "."]
pub mod scale_layers {
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/🧪️tests/📐️doubles/🦀️.rs"]
    mod tests_doubles_a_rect;
}
#[path = "."]
pub mod set_layer_boolean_operation {
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/🧪️tests/➖️union/🦀️.rs"]
    mod tests_union_to_subtract;
}
#[path = "."]
pub mod shape_coordinate {
    #[path = "🧬️schema/🧬️mutations/🔷️shape-coordinate/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔷️shape-coordinate/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔷️shape-coordinate/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🔷️shape-coordinate/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
#[path = "."]
pub mod update_layer_trace_params {
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer/🧪️tests/🔍️sharpens/🦀️.rs"]
    mod tests_sharpens_the_trace;
}
#[path = "."]
pub mod update_layer_transform {
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer/🧪️tests/📐️translates/🦀️.rs"]
    mod tests_translates_and_scales_shape_a;
}
#[path = "."]
pub mod update_path_geometry {
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
//#endregion 🪢️TaxonomyMounts
