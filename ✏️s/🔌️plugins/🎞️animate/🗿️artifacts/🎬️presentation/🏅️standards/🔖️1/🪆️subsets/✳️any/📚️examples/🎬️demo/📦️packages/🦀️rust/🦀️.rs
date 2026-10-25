//! 🎬️ examples of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

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
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
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
                #[path = "../../../../✏️editor/⚙️engine/🎬️scene/📑️sections/🦀️.rs"]
                pub mod section;
                #[path = "../../../../✏️editor/⚙️engine/🎬️scene/🧩️sobjects/🦀️.rs"]
                pub mod sobject;
            }
            #[path = "../../../../✏️editor/⚙️engine/📐️geometry/🦀️.rs"]
            pub mod geometry;
            #[path = "../../../../✏️editor/⚙️engine/⏱️rate/🦀️.rs"]
            pub mod rate;
            #[path = "../../../../✏️editor/⚙️engine/🔤️text/🦀️.rs"]
            pub mod text;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
