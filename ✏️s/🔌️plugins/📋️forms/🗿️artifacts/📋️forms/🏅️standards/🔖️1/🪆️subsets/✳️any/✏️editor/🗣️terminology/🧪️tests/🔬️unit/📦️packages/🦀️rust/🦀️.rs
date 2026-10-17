//! 🔬️ terminology-tests of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

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
                        #[path = "."]
                        pub mod snapshot {
                            #[path="../../../../../../🚪️io/📝️text/📸️snapshot/📨️response/🦀️.rs"]
                            pub mod response;
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
    pub mod forms {
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology::*;
            #[cfg(test)]
            #[path = "../../🦀️.rs"]
            pub mod tests;
        }
    }
}
