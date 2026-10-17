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
    pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::cas::simplify;
    pub use semio_s_artifact_mathematical_equation_mut_family::cas::integrate;
    #[path = "../../../../💡️inferences/🌿️cas-internals/➕️sums/🦀️.rs"]
    pub mod sums;
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
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
            #[path = "."]
            pub mod graph {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::graph::schema::mutations::change_graph_directed;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::graph::schema::mutations::create_node;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::graph::schema::mutations::delete_node;
                        #[path = "."]
                        pub mod delete_nodes {
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🦀️.rs"]
                            mod component;
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_nodes {
                            pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::move_nodes::*;
                            pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::move_nodes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🚚️move-nodes/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_node_positions {
                            pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::set_node_positions::*;
                            pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::set_node_positions::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/📍️set-node-positions/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::change_node_label;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::move_node;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::connect_nodes;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::disconnect_nodes;
                        #[cfg(test)]
                        #[path = "../../../../../../🕸️graph/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod leaf_tests;
                    }
                }
            }
            #[path = "."]
            pub mod geometry {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::insert_point;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::remove_point;
                        #[path = "."]
                        pub mod move_points {
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/🎯️move-points/🦀️.rs"]
                            mod component;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/🎯️move-points/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/🎯️move-points/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::set_point_positions;
                    }
                }
            }
            #[path = "."]
            pub mod equation {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_coefficient {
                            pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::equation::schema::mutations::change_coefficient::*;
                            pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::equation::schema::mutations::change_coefficient::diff;
                            pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::equation::schema::mutations::change_coefficient::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../➗️equation/🧬️schema/🧬️mutations/🎚️change/🧪️tests/🧪️raises/🦀️.rs"]
                            pub mod tests_raises_the_leading_coefficient_to_three_halves;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_mathematical_equation_inferences::diff;
pub use semio_s_artifact_mathematical_equation_snapshot::snapshot;
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_mathematical_equation_inferences::viewer::equation::modes::view::*;
                pub use semio_s_artifact_mathematical_equation_inferences::viewer::equation::modes::view::windows;
            }
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
