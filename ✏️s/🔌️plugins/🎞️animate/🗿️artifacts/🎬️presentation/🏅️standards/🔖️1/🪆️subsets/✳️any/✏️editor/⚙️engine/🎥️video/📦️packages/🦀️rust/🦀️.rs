//! 🎥️ video of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_schema as schema;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod engine {
            #[path = "."]
            pub mod scene {
                pub use semio_s_artifact_animate_presentation_tests::editor::animate::engine::scene::scene;
                pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::scene::section;
                pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::scene::sobject;
            }
            pub use semio_s_artifact_animate_presentation_animation::editor::animate::engine::animation;
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::camera;
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::config;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::geometry;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::text;
            #[path = "../../🦀️.rs"]
            pub mod video;
        }
    }
}
