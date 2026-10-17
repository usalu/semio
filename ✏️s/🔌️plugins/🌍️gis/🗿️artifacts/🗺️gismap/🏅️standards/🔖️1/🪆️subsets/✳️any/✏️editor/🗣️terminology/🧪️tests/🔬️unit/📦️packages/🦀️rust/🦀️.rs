//! 🔬️ terminology-tests of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis2d {
        pub use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::*;
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::terminology::*;
            #[cfg(test)]
            #[path = "../../🦀️.rs"]
            pub mod tests;
        }
    }
}
