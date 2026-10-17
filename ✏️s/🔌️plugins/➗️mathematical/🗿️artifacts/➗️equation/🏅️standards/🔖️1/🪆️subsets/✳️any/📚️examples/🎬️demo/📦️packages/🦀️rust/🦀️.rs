//! 🎬️ examples of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;

#[path = "."]
pub mod polynomial {
    #[path = "../../../../🧬️schema/💡️inferences/📈️polynomial-internals/📏️univariate/🦀️.rs"]
    pub mod univariate;
    #[path = "../../../../🧬️schema/💡️inferences/📈️polynomial-internals/🕸️multivariate/🦀️.rs"]
    pub mod multivariate;
    #[path = "../../../../🧬️schema/💡️inferences/📈️polynomial-internals/🧮️finite/🦀️.rs"]
    pub mod finite;
    #[path = "../../../../🧬️schema/💡️inferences/📈️polynomial-internals/🔍️factor/🦀️.rs"]
    pub mod factor;
    #[path = "../../../../🧬️schema/💡️inferences/📈️polynomial-internals/🌱️roots/🦀️.rs"]
    pub mod roots;
    #[path = "../../../../🧬️schema/💡️inferences/📈️polynomial-internals/🔢️algebraic/🦀️.rs"]
    pub mod algebraic;
}
#[path = "."]
pub mod cas {
    #[path = "../../../../🧬️schema/💡️inferences/🌿️cas-internals/🏷️fnkind/🦀️.rs"]
    pub mod fnkind;
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
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
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
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod equation {
        pub use semio_s_artifact_mathematical_equation_editor::editor::equation::*;
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
