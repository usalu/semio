//! 📸️ snapshot of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
pub use crate::cas::expr::artifact_items_2::{EquationEdge, EquationGeometry, EquationGraph, EquationNode, EquationPoint};
pub use crate::cas::expr::artifact_items_3::{EquationComputedChild, EquationNotationChild, EquationResultsChild};
pub use crate::cas::expr::artifact_items_4::{equation_carrier_snapshot, equation_children, equation_computed_from_state, equation_derived_child, equation_derived_pack, equation_notation_from_graph, equation_results_from_graph, equation_snapshot_from_host_snapshot, equation_snapshot_with_state, genesis_equation_child_pack};

#[path = "."]
pub mod polynomial {
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::univariate;
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::multivariate;
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::factor;
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::roots;
    pub use semio_s_artifact_mathematical_equation_examples::polynomial::algebraic;
}
#[path = "."]
pub mod cas {
    pub use semio_s_artifact_mathematical_equation_examples::cas::fnkind;
    #[path = "../../../💡️inferences/🌿️cas-internals/🧮️expr/🦀️.rs"]
    pub mod expr;
    #[path = "../../../💡️inferences/🌿️cas-internals/📏️canon/🦀️.rs"]
    pub mod canon;
    #[path = "../../../💡️inferences/🌿️cas-internals/🧷️assume/🦀️.rs"]
    pub mod assume;
    #[path = "../../../💡️inferences/🌿️cas-internals/🧭️visit/🦀️.rs"]
    pub mod visit;
    #[path = "../../../💡️inferences/🌿️cas-internals/🖨️fmt/🦀️.rs"]
    pub mod fmt;
    #[path = "../../../💡️inferences/🌿️cas-internals/🌉️polybridge/🦀️.rs"]
    pub mod polybridge;
    #[path = "../../../💡️inferences/🌿️cas-internals/🌱️rootof/🦀️.rs"]
    pub mod rootof;
    #[path = "../../../💡️inferences/🌿️cas-internals/🔄️transforms/🦀️.rs"]
    pub mod transforms;
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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
        }
    }
}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
