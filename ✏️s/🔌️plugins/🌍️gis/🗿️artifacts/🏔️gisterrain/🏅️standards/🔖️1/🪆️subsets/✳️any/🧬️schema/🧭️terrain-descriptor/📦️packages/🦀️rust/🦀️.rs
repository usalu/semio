//! 🧭️ terrain-descriptor of `semio-s-artifact-gis-gisterrain` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

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
                    pub mod snapshot {
                        #[path="../../../📸️snapshot/🧮️value-admission/🦀️.rs"]
                        pub mod value_admission;
                    }
                    #[path = "../../🦀️.rs"]
                    pub mod terrain_descriptor;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/📥change-imported-features/🦀️.rs"]
                            pub mod change_imported_features;
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/🎚️change-exaggeration/🦀️.rs"]
                            pub mod change_exaggeration;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/📥change-imported-features/🦀️.rs"]
                            pub mod change_imported_features;
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🎚️change-exaggeration/🦀️.rs"]
                            pub mod change_exaggeration;
                        }
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
        #[path = "../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis3d {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod examples {
            #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
            pub mod demo_session;
            #[cfg(test)]
            #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
            pub mod demo_session_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod terrain {
                        #[path = "."]
                        pub mod config {
                            #[path = "../../../../✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gisterrain {
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
