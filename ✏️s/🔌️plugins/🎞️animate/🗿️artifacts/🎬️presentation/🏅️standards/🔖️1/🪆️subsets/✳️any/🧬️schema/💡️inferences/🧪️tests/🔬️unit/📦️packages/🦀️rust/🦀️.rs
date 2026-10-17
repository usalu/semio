//! 🔬️ tests of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::demo_snapshot;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame};
pub use semio_s_artifact_animate_presentation_inferences::{PRESENTATION_CANVAS_SCALE, presentation_canvas_camera, presentation_snapshot_with_tiles, presentation_working_scene};
pub use crate::editor::animate::modes::main::artifact_items_8::{default_presentation_snapshot};

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
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod engine {
            #[path = "."]
            pub mod scene {
                #[allow(clippy::module_inception)]
                #[path = "../../../../../../✏️editor/⚙️engine/🎬️scene/🎭️scene-timeline/🦀️.rs"]
                pub mod scene;
                pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::scene::section;
                pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::scene::sobject;
            }
            #[path = "."]
            pub mod video {
                #[path = "../../../../../../✏️editor/⚙️engine/🎥️video/🎪️built-in-scenes/🦀️.rs"]
                pub mod scenes;
            }
            pub use semio_s_artifact_animate_presentation_animation::editor::animate::engine::animation;
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::camera;
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::config;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::rate;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                #[path = "../../../../../../✏️editor/🎭️modes/🖊️main/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../✏️editor/🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🦀️.rs"]
                    pub mod tile_editor;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️tile-editor/🦀️.rs"]
                    pub mod tile_editor;
                }
            }
        }
    }
}
