//! 🧬️ schema of `semio-s-artifact-wfc-grid3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod blocks {
                        pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples::blocks::*;
                        #[cfg(test)]
                        #[path = "../../../📚️examples/🧱️blocks/🧪️tests/🧩️example/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod pipes_3d {
                        pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples::pipes_3d::*;
                        #[cfg(test)]
                        #[path = "../../../📚️examples/🪠️pipes-3d/🧪️tests/🧩️example/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid3d {
        #[path = "."]
        pub mod presence {
            #[path = "../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid3d {
        #[path = "."]
        pub mod presence {
            #[path = "../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
