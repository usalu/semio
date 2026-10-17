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
                        pub use semio_s_artifact_shooting_shooting_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_asset {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::inverse;
                        }
                        #[path = "."]
                        pub mod rename_asset {
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::*;
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::inverse;
                        }
                        #[path = "."]
                        pub mod change_asset_url {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::inverse;
                        }
                        #[path = "."]
                        pub mod drag_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::inverse;
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
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_dims_scene_sun_to_half;
                        }
                        #[path = "."]
                        pub mod change_scene_ambient_intensity {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🔅️change-scene-ambient/🧪️tests/🔅️dims/🦀️.rs"]
                            pub mod tests_dims_scene_ambient_to_quarter;
                        }
                        #[path = "."]
                        pub mod change_scene_shadow_enabled {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🌑️change-scene-shadow/🧪️tests/🌑️switches/🦀️.rs"]
                            pub mod tests_switches_scene_shadows_off;
                        }
                        #[path = "."]
                        pub mod change_scene_material_roughness {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🪨️change-scene/🧪️tests/✨️polishes/🦀️.rs"]
                            pub mod tests_polishes_scene_material_to_quarter;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod hexagonal_cut_concrete_forest_left {
        pub use semio_s_artifact_shooting_shooting_mut_family::examples::hexagonal_cut_concrete_forest_left::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config;
        pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::ui_helpers_and_io;
        #[path = "../../../../../../../../✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../✏️editor/🎮️commands/📦️asset/🦀️.rs"]
            pub mod asset;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🎥️camera/🦀️.rs"]
            pub mod camera;
            #[path = "../../../../../../../../✏️editor/🎮️commands/📄️document/🦀️.rs"]
            pub mod document;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🧭️gumball/🦀️.rs"]
            pub mod gumball;
            #[path = "../../../../../../../../✏️editor/🎮️commands/☀️scene/🦀️.rs"]
            pub mod scene;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🗂️selection/🦀️.rs"]
            pub mod selection;
            #[path = "../../../../../../../../✏️editor/🎮️commands/📷️shot/🦀️.rs"]
            pub mod shot;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::*;
                #[cfg(test)]
                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        pub use semio_s_artifact_shooting_shooting_set_active_asset::editor::shooting::modes::edit::windows::scene::*;
                    }
                    #[path = "."]
                    pub mod icon {
                        pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::windows::icon::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod shooting {
        #[path = "../../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_shooting_shooting_tests::viewer::shooting::modes::view::*;
                pub use semio_s_artifact_shooting_shooting_tests::viewer::shooting::modes::view::windows;
            }
        }
    }
}
