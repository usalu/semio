//! 🔄️ editor of `semio-s-artifact-sequence-sequence` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_infinite as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::snapshot::schema::SequenceHostSnapshot;
pub use crate::snapshot::schema::SequenceSnapshot;
pub use crate::snapshot::schema::default_host_snapshot;
pub use crate::snapshot::schema::default_snapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_sequence_sequence_schema::{SequenceCamera, SequenceStep};

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
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
pub use semio_s_artifact_sequence_sequence_schema::snapshot;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
        #[cfg(target_arch = "wasm32")]
        #[path = "."]
        pub mod wasm {
            #[path = "."]
            pub mod protocol {
                pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::wasm::protocol::*;
                #[cfg(test)]
                #[path = "../../../../🌉️wasm/🧪️tests/🔬️protocol-unit/🦀️.rs"]
                pub mod tests;
            }
        }
        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::terminology;
        pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::base;
        pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::host;
        #[path = "."]
        pub mod commands {
            #[path = "../../🦀️.rs"]
            pub mod layout;
            #[path = "../../../🏃️run/🦀️.rs"]
            pub mod playback;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️.rs"]
                    pub mod compiled;
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📽️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs"]
                        pub mod config;
                    }
                    #[path = "."]
                    pub mod script {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📜️script/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient/🦀️.rs"]
                        pub mod transient;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
