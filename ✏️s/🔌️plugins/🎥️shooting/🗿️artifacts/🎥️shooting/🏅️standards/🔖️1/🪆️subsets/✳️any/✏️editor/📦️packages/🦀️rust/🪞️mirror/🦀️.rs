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
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_shooting_shooting_mut_family::examples::demo::*;
    }
    #[path = "."]
    pub mod hexagonal_cut_concrete_forest_left {
        pub use semio_s_artifact_shooting_shooting_mut_family::examples::hexagonal_cut_concrete_forest_left::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config;
        pub use semio_s_artifact_shooting_shooting_config::editor::shooting::presence;
        pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::terminology;
        pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::ui_helpers_and_io;
        #[path = "."]
        pub mod wasm {
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::wasm::*;
            #[cfg(test)]
            #[path = "../../../🌉️wasm/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod asset {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::asset::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📦️asset/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod camera {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::camera::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🎥️camera/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "../../../🎮️commands/🖨️export/🦀️.rs"]
            pub mod export;
            #[path = "."]
            pub mod document {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::document::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📄️document/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod gumball {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::gumball::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod scene {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::scene::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/☀️scene/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod selection {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::selection::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🗂️selection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod shot {
                pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::shot::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📷️shot/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📷️shot/🧪️tests/🔬️field-value-contract/🦀️.rs"]
                pub mod field_value_contract;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::windows::*;
                    #[path = "."]
                    pub mod scene {
                        pub use semio_s_artifact_shooting_shooting_set_active_asset::editor::shooting::modes::edit::windows::scene::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🎥️scene/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod icon {
                        pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::windows::icon::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🖼️icon/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod catalogue {
                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::catalogue::*;
                #[cfg(test)]
                #[path = "../../../📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod document {
                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::document::*;
                #[cfg(test)]
                #[path = "../../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
