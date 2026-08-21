//! 🎥️ Shooting plugin — icon-studio play app bundled as a hot-swappable WASM component.
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
// `Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🗿️artifacts/🎥️shooting/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;

                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-asset/🧪️tests/appends-asset-detail/🦀️component.rs"]
                                    mod tests_appends_asset_detail;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-asset/🧪️tests/removes-trailing-asset-prop/🦀️component.rs"]
                                    mod tests_removes_trailing_asset_prop;
                                }
                                #[path = "."]
                                pub mod rename_asset {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/renames-asset-hero-to-lead/🦀️component.rs"]
                                    mod tests_renames_asset_hero_to_lead;
                                }
                                #[path = "."]
                                pub mod change_asset_url {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️change-asset-url/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️change-asset-url/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️change-asset-url/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️change-asset-url/🧪️tests/points-asset-prop-at-v2-mesh/🦀️component.rs"]
                                    mod tests_points_asset_prop_at_v2_mesh;
                                }
                                #[path = "."]
                                pub mod reorder_assets {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-assets/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-assets/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-assets/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-assets/🧪️tests/moves-asset-hero-behind-asset-prop/🦀️component.rs"]
                                    mod tests_moves_asset_hero_behind_asset_prop;
                                }
                                #[path = "."]
                                pub mod drag_assets {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️drag-assets/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️drag-assets/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️drag-assets/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️drag-assets/🧪️tests/offsets-both-assets-and-skips-a-ghost/🦀️component.rs"]
                                    mod tests_offsets_both_assets_and_skips_a_ghost;
                                }
                                #[path = "."]
                                pub mod rotate_assets {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-assets/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-assets/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-assets/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-assets/🧪️tests/spins-asset-hero-about-z/🦀️component.rs"]
                                    mod tests_spins_asset_hero_about_z;
                                }
                                #[path = "."]
                                pub mod scale_assets {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↕️scale-assets/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↕️scale-assets/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↕️scale-assets/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↕️scale-assets/🧪️tests/doubles-asset-hero-scale/🦀️component.rs"]
                                    mod tests_doubles_asset_hero_scale;
                                }
                                #[path = "."]
                                pub mod create_shot {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️create-shot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️create-shot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️create-shot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️create-shot/🧪️tests/appends-shot-macro/🦀️component.rs"]
                                    mod tests_appends_shot_macro;
                                }
                                #[path = "."]
                                pub mod delete_shot {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-shot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-shot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-shot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-shot/🧪️tests/removes-trailing-shot-close/🦀️component.rs"]
                                    mod tests_removes_trailing_shot_close;
                                }
                                #[path = "."]
                                pub mod rename_shot {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🧪️tests/relabels-shot-close-to-detail/🦀️component.rs"]
                                    mod tests_relabels_shot_close_to_detail;
                                }
                                #[path = "."]
                                pub mod change_shot_width {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-shot-width/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-shot-width/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-shot-width/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-shot-width/🧪️tests/widens-shot-close-to-1024/🦀️component.rs"]
                                    mod tests_widens_shot_close_to_1024;
                                }
                                #[path = "."]
                                pub mod change_shot_height {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-shot-height/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-shot-height/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-shot-height/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-shot-height/🧪️tests/heightens-shot-close-to-768/🦀️component.rs"]
                                    mod tests_heightens_shot_close_to_768;
                                }
                                #[path = "."]
                                pub mod change_shot_format {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-shot-format/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-shot-format/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-shot-format/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-shot-format/🧪️tests/switches-shot-wide-to-svg/🦀️component.rs"]
                                    mod tests_switches_shot_wide_to_svg;
                                }
                                #[path = "."]
                                pub mod change_shot_shape {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-shot-shape/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-shot-shape/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-shot-shape/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-shot-shape/🧪️tests/rounds-shot-wide-to-ellipse/🦀️component.rs"]
                                    mod tests_rounds_shot_wide_to_ellipse;
                                }
                                #[path = "."]
                                pub mod reorder_shots {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️reorder-shots/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️reorder-shots/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️reorder-shots/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️reorder-shots/🧪️tests/moves-shot-close-to-front/🦀️component.rs"]
                                    mod tests_moves_shot_close_to_front;
                                }
                                #[path = "."]
                                pub mod replace_shot_camera {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️replace-shot-camera/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️replace-shot-camera/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️replace-shot-camera/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️replace-shot-camera/🧪️tests/rewrites-cam-wide-through-shot-wide/🦀️component.rs"]
                                    mod tests_rewrites_cam_wide_through_shot_wide;
                                }
                                #[path = "."]
                                pub mod create_saved_camera {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥️create-saved-camera/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥️create-saved-camera/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥️create-saved-camera/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥️create-saved-camera/🧪️tests/appends-saved-camera-top/🦀️component.rs"]
                                    mod tests_appends_saved_camera_top;
                                }
                                #[path = "."]
                                pub mod delete_saved_camera {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-saved-camera/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-saved-camera/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-saved-camera/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-saved-camera/🧪️tests/removes-trailing-cam-close/🦀️component.rs"]
                                    mod tests_removes_trailing_cam_close;
                                }
                                #[path = "."]
                                pub mod rename_saved_camera {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-saved-camera/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-saved-camera/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-saved-camera/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-saved-camera/🧪️tests/relabels-cam-close-to-tight/🦀️component.rs"]
                                    mod tests_relabels_cam_close_to_tight;
                                }
                                #[path = "."]
                                pub mod replace_saved_camera_view {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️replace-saved-camera-view/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️replace-saved-camera-view/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️replace-saved-camera-view/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️replace-saved-camera-view/🧪️tests/repositions-cam-close-view/🦀️component.rs"]
                                    mod tests_repositions_cam_close_view;
                                }
                                #[path = "."]
                                pub mod reorder_saved_cameras {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️reorder-saved-cameras/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️reorder-saved-cameras/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️reorder-saved-cameras/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️reorder-saved-cameras/🧪️tests/moves-cam-close-to-front/🦀️component.rs"]
                                    mod tests_moves_cam_close_to_front;
                                }
                                #[path = "."]
                                pub mod set_active_shot {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️set-active-shot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️set-active-shot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️set-active-shot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️set-active-shot/🧪️tests/activates-shot-close/🦀️component.rs"]
                                    mod tests_activates_shot_close;
                                }
                                #[path = "."]
                                pub mod set_active_asset {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-active-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-active-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-active-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-active-asset/🧪️tests/activates-asset-prop/🦀️component.rs"]
                                    mod tests_activates_asset_prop;
                                }
                                #[path = "."]
                                pub mod change_scene_sun_enabled {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-scene-sun-enabled/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-scene-sun-enabled/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-scene-sun-enabled/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-scene-sun-enabled/🧪️tests/switches-scene-sun-off/🦀️component.rs"]
                                    mod tests_switches_scene_sun_off;
                                }
                                #[path = "."]
                                pub mod change_scene_sun_azimuth {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-scene-sun-azimuth/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-scene-sun-azimuth/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-scene-sun-azimuth/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-scene-sun-azimuth/🧪️tests/turns-scene-sun-to-315-degrees/🦀️component.rs"]
                                    mod tests_turns_scene_sun_to_315_degrees;
                                }
                                #[path = "."]
                                pub mod change_scene_sun_elevation {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-scene-sun-elevation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-scene-sun-elevation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-scene-sun-elevation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-scene-sun-elevation/🧪️tests/raises-scene-sun-to-60-degrees/🦀️component.rs"]
                                    mod tests_raises_scene_sun_to_60_degrees;
                                }
                                #[path = "."]
                                pub mod change_scene_sun_intensity {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️change-scene-sun-intensity/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️change-scene-sun-intensity/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️change-scene-sun-intensity/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️change-scene-sun-intensity/🧪️tests/dims-scene-sun-to-half/🦀️component.rs"]
                                    mod tests_dims_scene_sun_to_half;
                                }
                                #[path = "."]
                                pub mod change_scene_ambient_intensity {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-scene-ambient-intensity/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-scene-ambient-intensity/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-scene-ambient-intensity/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-scene-ambient-intensity/🧪️tests/dims-scene-ambient-to-quarter/🦀️component.rs"]
                                    mod tests_dims_scene_ambient_to_quarter;
                                }
                                #[path = "."]
                                pub mod change_scene_shadow_enabled {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌑️change-scene-shadow-enabled/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌑️change-scene-shadow-enabled/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌑️change-scene-shadow-enabled/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌑️change-scene-shadow-enabled/🧪️tests/switches-scene-shadows-off/🦀️component.rs"]
                                    mod tests_switches_scene_shadows_off;
                                }
                                #[path = "."]
                                pub mod change_scene_material_roughness {
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️change-scene-material-roughness/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️change-scene-material-roughness/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️change-scene-material-roughness/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️change-scene-material-roughness/🧪️tests/polishes-scene-material-to-quarter/🦀️component.rs"]
                                    mod tests_polishes_scene_material_to_quarter;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod gif {
                                            #[path = "."]
                                            pub mod v87a {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod jpg {
                                            #[path = "."]
                                            pub mod v_jfif_1_01 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️jpg/🔖️jfif-1.01/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod bmp {
                                            #[path = "."]
                                            pub mod v_v3 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖼️bmp/🔖️v3/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod tiff {
                                            #[path = "."]
                                            pub mod v6_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️component.rs"]
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
                                        pub mod gif {
                                            #[path = "."]
                                            pub mod v87a {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod jpg {
                                            #[path = "."]
                                            pub mod v_jfif_1_01 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️jpg/🔖️jfif-1.01/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod bmp {
                                            #[path = "."]
                                            pub mod v_v3 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️bmp/🔖️v3/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod tiff {
                                            #[path = "."]
                                            pub mod v6_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️component.rs"]
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
            pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod pack {
            pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::binary::*;
        }
        pub mod mutations {
            pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
                mod tests;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️asset/🦀️component.rs"]
            pub mod asset;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖨️export/🦀️component.rs"]
            pub mod export;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️gumball/🦀️component.rs"]
            pub mod gumball;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️scene/🦀️component.rs"]
            pub mod scene;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️shot/🦀️component.rs"]
            pub mod shot;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🌫️ambient/🦀️component.rs"]
                            pub mod ambient;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🎯️center-model/🦀️component.rs"]
                            pub mod center_model;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/✨️roughness/🦀️component.rs"]
                            pub mod roughness;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🌑️shadow/🦀️component.rs"]
                            pub mod shadow;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🧭️sun-azimuth/🦀️component.rs"]
                            pub mod sun_azimuth;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/📐️sun-elevation/🦀️component.rs"]
                            pub mod sun_elevation;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/☀️sun-enabled/🦀️component.rs"]
                            pub mod sun_enabled;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/💡️sun-intensity/🦀️component.rs"]
                            pub mod sun_intensity;
                        }
                    }

                    #[path = "."]
                    pub mod icon {
                        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/🗂️format/🦀️component.rs"]
                            pub mod format;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/🔷️shape/🦀️component.rs"]
                            pub mod shape;
                            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/📷️shot/🦀️component.rs"]
                            pub mod shot;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🎥️scene/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
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
    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_shooting_demo_session;
    #[path = "../../🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_shooting_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
