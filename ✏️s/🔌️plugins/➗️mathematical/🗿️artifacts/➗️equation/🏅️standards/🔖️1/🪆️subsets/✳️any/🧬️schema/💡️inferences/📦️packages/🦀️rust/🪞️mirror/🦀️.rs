//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod polynomial {
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::univariate;
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::roots;
}
#[path = "."]
pub mod cas {
    pub use semio_s_artifact_mathematical_equation_examples::cas::fnkind;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::expr;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::visit;
    pub use semio_s_artifact_mathematical_equation_schema::cas::diff;
    #[path = "../../../🌿️cas-internals/🎞️series/🦀️.rs"]
    pub mod series;
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
                    pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "../../../🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        // 🚚 Wave M3a: first real `impl InferredField<P>` in this codebase — see its own
                        // doc header for why (every other named inference documents using the plain
                        // whole-snapshot pattern instead).
                        #[path = "."]
                        pub mod roots {
                            #[path = "../../../🌱roots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        // 🚚 Wave M3a (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
                        // `🌿️cas-internals/`'s and `📈️polynomial-internals/`'s Rust-only compute code lives
                        // PHYSICALLY right here, under this facet — but is MOUNTED at crate root as `pub mod cas`/
                        // `pub mod polynomial` (see the top of this file), not nested under this module. Every
                        // `crate::cas::…`/`crate::polynomial::…` self-reference inside those two files is
                        // untouched from the original `🧮️math` crate, and privacy is structural in Rust: a
                        // `mod canon { … }` (non-`pub`) nested here would need `pub use component::*` to leak
                        // it back out, which does NOT re-export private items — `crate::cas::canon` would 404.
                        // Direct crate-root mounting is the only way to preserve every private inner `mod`
                        // unedited, so these two crates deliberately do NOT also appear as a submodule of
                        // `inferences` — see `🌿️cas-internals/🦀️.rs`'s doc header for the full story.
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod graph {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_graph_directed {
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph/🦀️.rs"]
                            mod component;
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                }
            }
        }
    }
}
