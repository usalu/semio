//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod create_asset {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::inverse;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_appends_asset_detail;
                        }
                        #[path = "."]
                        pub mod delete_asset {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::delete_asset::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::delete_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::delete_asset::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete-asset/🧪️tests/🗑️removes/🦀️.rs"]
                            pub mod tests_removes_trailing_asset_prop;
                        }
                        #[path = "."]
                        pub mod rename_asset {
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::*;
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../✏️rename-asset/🧪️tests/🏷️renames/🦀️.rs"]
                            pub mod tests_renames_asset_hero_to_lead;
                        }
                        #[path = "."]
                        pub mod change_asset_url {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🌐️change-asset-url/🧪️tests/🌐️points/🦀️.rs"]
                            pub mod tests_points_asset_prop_at_v2_mesh;
                        }
                        #[path = "."]
                        pub mod reorder_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::reorder_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::reorder_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::reorder_assets::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🔀️reorder-assets/🧪️tests/🔀️moves/🦀️.rs"]
                            pub mod tests_moves_asset_hero_behind_asset_prop;
                        }
                        #[path = "."]
                        pub mod drag_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../↔️drag-assets/🧪️tests/🚚️offsets/🦀️.rs"]
                            pub mod tests_offsets_both_assets_and_skips_a_ghost;
                        }
                        #[path = "."]
                        pub mod rotate_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🔄️rotate-assets/🧪️tests/🔄️spins/🦀️.rs"]
                            pub mod tests_spins_asset_hero_about_z;
                        }
                        #[path = "."]
                        pub mod scale_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::scale_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::scale_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::scale_assets::inverse;
                        }
                        #[path = "."]
                        pub mod create_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::create_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::create_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::create_shot::inverse;
                        }
                        #[path = "."]
                        pub mod delete_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::delete_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::delete_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::delete_shot::inverse;
                        }
                        #[path = "."]
                        pub mod rename_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rename_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rename_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rename_shot::inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_width {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_width::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_width::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_width::inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_height {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_height::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_height::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_height::inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_format {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_format::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_format::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_format::inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_shape {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::change_shot_shape::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::change_shot_shape::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::change_shot_shape::inverse;
                        }
                        #[path = "."]
                        pub mod reorder_shots {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_shots::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_shots::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_shots::inverse;
                        }
                        #[path = "."]
                        pub mod replace_shot_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_shot_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_shot_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_shot_camera::inverse;
                        }
                        #[path = "."]
                        pub mod create_saved_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::create_saved_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::create_saved_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::create_saved_camera::inverse;
                        }
                        #[path = "."]
                        pub mod delete_saved_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::delete_saved_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::delete_saved_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::delete_saved_camera::inverse;
                        }
                        #[path = "."]
                        pub mod rename_saved_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_saved_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_saved_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_saved_camera::inverse;
                        }
                        #[path = "."]
                        pub mod replace_saved_camera_view {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::inverse;
                        }
                        #[path = "."]
                        pub mod reorder_saved_cameras {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::inverse;
                        }
                        #[path = "."]
                        pub mod set_active_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::set_active_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::set_active_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::set_active_shot::inverse;
                        }
                        #[path = "."]
                        pub mod set_active_asset {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::set_active_asset::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::set_active_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::set_active_asset::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_enabled {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_azimuth {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_elevation {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_intensity {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_ambient_intensity {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_shadow_enabled {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_material_roughness {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_shooting_shooting_schema::diff;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config;
        pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::terminology;
        pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::ui_helpers_and_io;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        pub use semio_s_artifact_shooting_shooting_set_active_asset::editor::shooting::modes::edit::windows::scene::*;
                        #[path = "."]
                        pub mod options {
                            #[path = "."]
                            pub mod roughness {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::roughness::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/✨️roughness/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod shadow {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::shadow::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🌑️shadow/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod sun_azimuth {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_azimuth::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🧭️sun-azimuth/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod sun_elevation {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_elevation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/📐️sun-elevation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod sun_enabled {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_enabled::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/☀️sun-enabled/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod sun_intensity {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_intensity::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/💡️sun-intensity/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod icon {
                        #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            #[path = "."]
                            pub mod format {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::icon::options::format::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/☑️options/🗂️format/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod shape {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::icon::options::shape::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/☑️options/🔷️shape/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod shot {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::icon::options::shot::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/☑️options/📷️shot/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../../../../../../../✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs"]
                pub mod semantic_contract;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod shooting {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_shooting_shooting_tests::viewer::shooting::modes::view::windows;
            }
        }
    }
}
