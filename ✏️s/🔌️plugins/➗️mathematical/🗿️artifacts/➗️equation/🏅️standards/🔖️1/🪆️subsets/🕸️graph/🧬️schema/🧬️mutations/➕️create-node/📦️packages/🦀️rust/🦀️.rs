//! ➕️ mut-family of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::EquationDiff;
pub use semio_s_artifact_mathematical_equation_snapshot::{EquationNode, EquationPoint};

#[path = "."]
pub mod polynomial {
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::univariate;
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::roots;
}
#[path = "."]
pub mod cas {
    pub use semio_s_artifact_mathematical_equation_examples::cas::fnkind;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::expr;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::assume;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::visit;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::polybridge;
    pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::cas::simplify;
    pub use semio_s_artifact_mathematical_equation_schema::cas::diff;
    pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::cas::limits;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::rootof;
    #[path = "../../../../../../✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🧠️solve/🦀️.rs"]
    pub mod solve;
    #[path = "../../../../../../✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🔢️matrix/🦀️.rs"]
    pub mod matrix;
    #[path = "../../../../../../✳️any/🧬️schema/💡️inferences/🌿️cas-internals/➗️integrate/🦀️.rs"]
    pub mod integrate;
    #[path = "../../../../../../✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🌊️ode/🦀️.rs"]
    pub mod ode;
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
            }
            #[path = "."]
            pub mod graph {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_node {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "../../../❌️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "../../../❌️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../❌️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::connect_nodes;
                    }
                }
            }
            #[path = "."]
            pub mod geometry {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod insert_point {
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🦀️.rs"]
                            mod component;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_point {
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/🦀️.rs"]
                            mod component;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod set_point_positions {
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/📌️set-point-positions/🦀️.rs"]
                            mod component;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/📌️set-point-positions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../📐️geometry/🧬️schema/🧬️mutations/📌️set-point-positions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
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
                            #[path = "../../../../../../➗️equation/🧬️schema/🧬️mutations/🎚️change/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../➗️equation/🧬️schema/🧬️mutations/🎚️change/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../➗️equation/🧬️schema/🧬️mutations/🎚️change/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_mathematical_equation_inferences::diff;
