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
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod resize_output {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_shrinks_the_output_and_cascades_a_pin;
                        }
                        #[path = "."]
                        pub mod change_model {
                            #[cfg(test)]
                            #[path = "../../../../../../⚙️change-model/🧪️tests/⚙️widens/🦀️.rs"]
                            pub mod tests_widens_the_window_to_three;
                        }
                        #[path = "."]
                        pub mod pin_pixel {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../📌️pin-pixel/🧪️tests/📌️pins/🦀️.rs"]
                            pub mod tests_pins_the_origin_cell_to_colour_1;
                        }
                        #[path = "."]
                        pub mod unpin_pixel {
                            #[cfg(test)]
                            #[path = "../../../../../../📍️unpin-pixel/🧪️tests/📍️releases/🦀️.rs"]
                            pub mod tests_releases_the_pinned_origin_cell;
                        }
                        #[path = "."]
                        pub mod paint_input_stroke {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../✍️paint-input-stroke/🧪️tests/✍️paints/🦀️.rs"]
                            pub mod tests_paints_a_diagonal_stroke_in_colour_1;
                            #[cfg(test)]
                            #[path = "../../../../../../✍️paint-input-stroke/🧪️tests/⚠️clips/🦀️.rs"]
                            pub mod tests_clips_a_stroke_leaving_the_sample;
                            #[cfg(test)]
                            #[path = "../../../../../../✍️paint-input-stroke/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests_unit;
                        }
                    }
                    pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::inferences;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "../../../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::*;
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::json::*;
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
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
                                pub mod txt {
                                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt::*;
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::json::*;
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
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
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bitmap {
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../../../../../../✏️editor/🎮️commands/📌️pin-solution/🦀️.rs"]
            pub mod pin_solution;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bitmap {
        #[path = "../../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::windows::*;
                    #[path = "."]
                    pub mod input {
                        pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::windows::input::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️input/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod output {
                        pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::windows::output::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🧩️output/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "../../../../../../../../📚️examples/🚪️rooms-16/🦀️.rs"]
    pub mod rooms_16;
    #[path = "../../../../../../../../📚️examples/🌸️flowers-24/🦀️.rs"]
    pub mod flowers_24;

    /// 📇️ Every bundled example, in the order the editor's example picker offers them.
    pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![rooms_16::source(), flowers_24::source()]
    }

    /// 📇️ The same roster as a `&'static` slice, which is the shape `SubsetDeclaration` takes.
    pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
        static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
        EXAMPLES.get_or_init(sources).as_slice()
    }

    #[cfg(test)]
    #[path = "../../../../../../../../📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
#[path = "."]
pub mod host {
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_wfc_bitmap_inferences::host::inferences::*;
        #[cfg(test)]
        #[path="../../../../../../../../../../../../🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
