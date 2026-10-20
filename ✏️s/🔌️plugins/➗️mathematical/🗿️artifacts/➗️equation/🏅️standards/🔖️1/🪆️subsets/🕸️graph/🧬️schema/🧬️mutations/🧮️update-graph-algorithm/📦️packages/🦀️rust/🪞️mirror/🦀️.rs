//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod polynomial {
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::univariate;
}
#[path = "."]
pub mod cas {
    pub use semio_s_artifact_mathematical_equation_examples::cas::fnkind;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::expr;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::visit;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::polybridge;
    #[path = "../../../../../../../✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🧹️simplify/🦀️.rs"]
    pub mod simplify;
    pub use semio_s_artifact_mathematical_equation_schema::cas::trig;
    pub use semio_s_artifact_mathematical_equation_schema::cas::diff;
    pub use semio_s_artifact_mathematical_equation_inferences::cas::series;
    #[path = "../../../../../../../✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🎯️limits/🦀️.rs"]
    pub mod limits;
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
                    pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod diff {
                            pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::text::diff::*;
                            #[cfg(test)]
                            #[path="../../../../../../../✳️any/🚪️io/📝️text/🔺️diff/🧪️tests/🦀️.rs"]
                            pub mod tests;
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
                        pub mod update_graph_algorithm {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_nodes {
                            #[path = "../../../../🚚️move-nodes/🦀️.rs"]
                            mod component;
                            #[path = "../../../../🚚️move-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🚚️move-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod set_node_positions {
                            #[path = "../../../../📍️set-node-positions/🦀️.rs"]
                            mod component;
                            #[path = "../../../../📍️set-node-positions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📍️set-node-positions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_node_label {
                            #[path = "../../../../🏷️change-node-label/🦀️.rs"]
                            mod component;
                            #[path = "../../../../🏷️change-node-label/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🏷️change-node-label/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "../../../../🕹️move-node/🦀️.rs"]
                            mod component;
                            #[path = "../../../../🕹️move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🕹️move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod connect_nodes {
                            #[path = "../../../../🔗️connect-nodes/🦀️.rs"]
                            mod component;
                            #[path = "../../../../🔗️connect-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔗️connect-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod disconnect_nodes {
                            #[path = "../../../../✂️disconnect-nodes/🦀️.rs"]
                            mod component;
                            #[path = "../../../../✂️disconnect-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✂️disconnect-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_mathematical_equation_inferences::diff;
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_mathematical_equation_inferences::viewer::equation::modes::view::windows::*;
                    #[path = "."]
                    pub mod geometry {
                        pub use semio_s_artifact_mathematical_equation_inferences::viewer::equation::modes::view::windows::geometry::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📐️geometry/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
