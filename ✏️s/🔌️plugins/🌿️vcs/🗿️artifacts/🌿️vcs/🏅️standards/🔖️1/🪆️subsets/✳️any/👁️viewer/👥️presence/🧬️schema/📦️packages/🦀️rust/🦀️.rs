//! 🧬️ viewer of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

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
                    #[path = "../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod vcs {
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📜️history/🦀️.rs"]
                    pub mod history;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod vcs {
        #[path = "."]
        pub mod presence {
            #[path = "../../🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
