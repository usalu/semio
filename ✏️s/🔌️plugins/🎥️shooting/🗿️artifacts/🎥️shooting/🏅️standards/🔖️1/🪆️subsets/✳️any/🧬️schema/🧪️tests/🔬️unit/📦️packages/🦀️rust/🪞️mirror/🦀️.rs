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
                    pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod rename_asset {
                            #[path = "../../../../../🧬️mutations/✏️rename-asset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../🧬️mutations/✏️rename-asset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../🧬️mutations/✏️rename-asset/↩️inverse/🦀️.rs"]
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
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[cfg(test)]
        #[path = "../../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config;
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::terminology::*;
            #[cfg(test)]
            #[path = "../../../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "../../../../../../✏️editor/🧰️ui-helpers-and-io/🦀️.rs"]
        pub mod ui_helpers_and_io;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🌫️ambient/🦀️.rs"]
                            pub mod ambient;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🎯️center-model/🦀️.rs"]
                            pub mod center_model;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/✨️roughness/🦀️.rs"]
                            pub mod roughness;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🌑️shadow/🦀️.rs"]
                            pub mod shadow;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/🧭️sun-azimuth/🦀️.rs"]
                            pub mod sun_azimuth;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/📐️sun-elevation/🦀️.rs"]
                            pub mod sun_elevation;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/☀️sun-enabled/🦀️.rs"]
                            pub mod sun_enabled;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/☑️options/💡️sun-intensity/🦀️.rs"]
                            pub mod sun_intensity;
                        }
                    }
                    #[path = "."]
                    pub mod icon {
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/☑️options/🗂️format/🦀️.rs"]
                            pub mod format;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/☑️options/🔷️shape/🦀️.rs"]
                            pub mod shape;
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/☑️options/📷️shot/🦀️.rs"]
                            pub mod shot;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../../✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../../../✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../../../../✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
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
                #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🎥️scene/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
