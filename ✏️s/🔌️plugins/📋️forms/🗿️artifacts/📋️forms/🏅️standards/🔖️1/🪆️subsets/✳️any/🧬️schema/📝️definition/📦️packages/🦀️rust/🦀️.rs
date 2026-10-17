//! 📝️ definition of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::playbook::PlaybookBlock as FormQuestion;
use semio_framework_artifact_playbook_playbook as playbook;
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
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    pub mod definition;
                    #[path = "../../../📨️response/🦀️.rs"]
                    pub mod response;
                    #[path = "../../../✅️validation/🦀️.rs"]
                    pub mod validation;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "."]
                            pub mod response {
                                #[path="../../../../🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🦀️.rs"]
                                pub mod export;
                            }
                        }
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
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
        #[path = "../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🚪️io/🦀️.rs"]
            pub mod io;
        }
    }
}
