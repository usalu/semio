//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_bitmap_inferences::host::inferences;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_bitmap_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                        }
                        #[path = "."]
                        pub mod resize_input {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::resize_input::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::resize_input::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::resize_input::inverse;
                        }
                        #[path = "."]
                        pub mod set_input_pixels {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::inverse;
                        }
                        #[path = "."]
                        pub mod add_palette_color {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::add_palette_color::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::add_palette_color::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::add_palette_color::inverse;
                        }
                        #[path = "."]
                        pub mod change_palette_color {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_palette_color::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_palette_color::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_palette_color::inverse;
                        }
                        #[path = "."]
                        pub mod remove_palette_color {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::remove_palette_color::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::remove_palette_color::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::remove_palette_color::inverse;
                        }
                        #[path = "."]
                        pub mod resize_output {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::resize_output::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::resize_output::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::resize_output::inverse;
                        }
                        #[path = "."]
                        pub mod change_model {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_model::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_model::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_model::inverse;
                        }
                        #[path = "."]
                        pub mod pin_pixel {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::inverse;
                        }
                        #[path = "."]
                        pub mod unpin_pixel {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::unpin_pixel::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::unpin_pixel::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::unpin_pixel::inverse;
                        }
                        #[path = "."]
                        pub mod paint_input_stroke {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::inverse;
                        }
                    }
                    pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::inferences;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::standards::v1::subsets::any::io::binary::*;
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bitmap {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_wfc_bitmap_transient::editor::bitmap::transient;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod set_active_example {
                pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::editor::bitmap::commands::set_active_example::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod pin_solution {
                pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::editor::bitmap::commands::pin_solution::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📌️pin-solution/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod tools {
                    #[path = "../../../🎭️modes/✏️edit/🛠️tools/🌡fill/🦀️.rs"]
                    pub mod fill;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod input {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🖼️input/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config/🦀️.rs"]
                        pub mod config;
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../🎭️modes/✏️edit/🪟️windows/🖼️input/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                        }
                    }
                    #[path = "."]
                    pub mod output {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧩️output/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config/🦀️.rs"]
                        pub mod config;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bitmap {
        pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::viewer::bitmap::*;
    }
}
pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::examples;
