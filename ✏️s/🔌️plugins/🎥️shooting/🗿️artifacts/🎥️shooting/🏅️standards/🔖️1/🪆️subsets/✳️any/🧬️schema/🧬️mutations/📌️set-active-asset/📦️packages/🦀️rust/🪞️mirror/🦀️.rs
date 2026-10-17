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
                        #[path = "."]
                        pub mod set_active_asset {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_enabled {
                            #[path = "../../../../☀️change-scene-sun/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../☀️change-scene-sun/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../☀️change-scene-sun/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_azimuth {
                            #[path = "../../../../🧭️change-scene-sun/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧭️change-scene-sun/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧭️change-scene-sun/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_elevation {
                            #[path = "../../../../🌅️change-scene-sun/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌅️change-scene-sun/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌅️change-scene-sun/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_intensity {
                            #[path = "../../../../💡️change-scene-sun/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../💡️change-scene-sun/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../💡️change-scene-sun/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_ambient_intensity {
                            #[path = "../../../../🔅️change-scene-ambient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔅️change-scene-ambient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔅️change-scene-ambient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_shadow_enabled {
                            #[path = "../../../../🌑️change-scene-shadow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌑️change-scene-shadow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌑️change-scene-shadow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_scene_material_roughness {
                            #[path = "../../../../🪨️change-scene/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🪨️change-scene/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🪨️change-scene/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            #[path = "."]
                            pub mod ambient {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::ambient::*;
                                #[cfg(test)]
                                #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🌫️ambient/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod center_model {
                                pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::center_model::*;
                                #[cfg(test)]
                                #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🎯️center-model/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::roughness;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::shadow;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_azimuth;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_elevation;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_enabled;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_intensity;
                        }
                    }
                }
            }
        }
    }
}
