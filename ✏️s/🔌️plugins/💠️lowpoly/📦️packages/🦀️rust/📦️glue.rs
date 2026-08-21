//! 💠️ Lowpoly plugin — mesh + paint editor bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🗿️artifacts/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;

                                #[path = "."]
                                pub mod create_object {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🧪️tests/inserts-obj-mast-between-hull-and-fin/🦀️component.rs"]
                                    mod tests_inserts_obj_mast_between_hull_and_fin;
                                }
                                #[path = "."]
                                pub mod delete_object {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🧪️tests/removes-obj-fin-without-touching-the-order/🦀️component.rs"]
                                    mod tests_removes_obj_fin_without_touching_the_order;
                                }
                                #[path = "."]
                                pub mod reorder_objects {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/🧪️tests/moves-obj-fin-in-front-of-obj-hull/🦀️component.rs"]
                                    mod tests_moves_obj_fin_in_front_of_obj_hull;
                                }
                                #[path = "."]
                                pub mod rename_object {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/🧪️tests/retitles-obj-hull/🦀️component.rs"]
                                    mod tests_retitles_obj_hull;
                                }
                                #[path = "."]
                                pub mod change_object_smooth_shading {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🧪️tests/turns-on-smooth-shading-for-obj-hull/🦀️component.rs"]
                                    mod tests_turns_on_smooth_shading_for_obj_hull;
                                }
                                #[path = "."]
                                pub mod move_object {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🧪️tests/translates-obj-hull-along-x-and-z/🦀️component.rs"]
                                    mod tests_translates_obj_hull_along_x_and_z;
                                }
                                #[path = "."]
                                pub mod rotate_object {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/🧪️tests/yaws-obj-hull-about-the-y-axis/🦀️component.rs"]
                                    mod tests_yaws_obj_hull_about_the_y_axis;
                                }
                                #[path = "."]
                                pub mod scale_object {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/🧪️tests/halves-obj-hull-uniformly/🦀️component.rs"]
                                    mod tests_halves_obj_hull_uniformly;
                                }
                                #[path = "."]
                                pub mod create_mesh {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/attaches-a-mesh-child-handle-to-obj-fin/🦀️component.rs"]
                                    mod tests_attaches_a_mesh_child_handle_to_obj_fin;
                                }
                                #[path = "."]
                                pub mod delete_mesh {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/detaches-the-mesh-child-handle-from-obj-hull/🦀️component.rs"]
                                    mod tests_detaches_the_mesh_child_handle_from_obj_hull;
                                }
                                #[path = "."]
                                pub mod insert_paint_layer {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🧪️tests/stacks-a-detail-layer-above-the-base-layer/🦀️component.rs"]
                                    mod tests_stacks_a_detail_layer_above_the_base_layer;
                                }
                                #[path = "."]
                                pub mod remove_paint_layer {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🧪️tests/drops-the-detail-layer-at-index-1/🦀️component.rs"]
                                    mod tests_drops_the_detail_layer_at_index_1;
                                }
                                #[path = "."]
                                pub mod rename_paint_layer {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/🧪️tests/retitles-the-base-layer-to-undercoat/🦀️component.rs"]
                                    mod tests_retitles_the_base_layer_to_undercoat;
                                }
                                #[path = "."]
                                pub mod change_paint_layer_visible {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🧪️tests/hides-the-base-layer/🦀️component.rs"]
                                    mod tests_hides_the_base_layer;
                                }
                                #[path = "."]
                                pub mod change_paint_layer_opacity {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🧪️tests/fades-the-base-layer-to-half/🦀️component.rs"]
                                    mod tests_fades_the_base_layer_to_half;
                                }
                                #[path = "."]
                                pub mod change_paint_layer_blend_mode {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🧪️tests/switches-the-base-layer-to-multiply/🦀️component.rs"]
                                    mod tests_switches_the_base_layer_to_multiply;
                                }
                                #[path = "."]
                                pub mod edit_paint_layer {
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🧪️tests/paints-red-over-the-second-half-of-the-base-layer/🦀️component.rs"]
                                    mod tests_paints_red_over_the_second_half_of_the_base_layer;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs"]
        pub mod engine;
        #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️component.rs"]
        pub mod session;
        #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️component.rs"]
        pub mod view;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-primitive/🦀️component.rs"]
            pub mod add_primitive;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️chrome/🦀️component.rs"]
            pub mod chrome;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💬️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️component.rs"]
            pub mod mesh_edit;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️paint/🦀️component.rs"]
            pub mod paint;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-object/🦀️component.rs"]
            pub mod patch_object;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️uv/🦀️component.rs"]
            pub mod uv;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️component.rs"]
                    pub mod model;
                }
            }

            #[path = "."]
            pub mod paint {
                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️component.rs"]
                    pub mod uv;
                }
            }
        }

        #[path = "."]
        pub mod options {
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🖌️paint-params-brush/🦀️component.rs"]
            pub mod paint_params_brush;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🧽️paint-params-eraser/🦀️component.rs"]
            pub mod paint_params_eraser;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🗂️select/🦀️component.rs"]
            pub mod select;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/👁️show-edges/🦀️component.rs"]
            pub mod show_edges;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🧲️snap/🦀️component.rs"]
            pub mod snap;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🌞️sun/🦀️component.rs"]
            pub mod sun;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🦀️component.rs"]
            pub mod layers;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️component.rs"]
                    pub mod model;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_lowpoly_demo_session;
    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_lowpoly_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_lowpoly_demo_tests;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
