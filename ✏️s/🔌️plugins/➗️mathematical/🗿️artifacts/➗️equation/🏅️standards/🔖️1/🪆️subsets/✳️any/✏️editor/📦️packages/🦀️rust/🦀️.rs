//! ✏️ editor of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
#[cfg(test)]
pub use crate::editor::equation::modes::edit::windows::graph::config::EquationCamera;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
}
pub use semio_s_artifact_mathematical_equation_snapshot::{EquationEdge, EquationGeometry, EquationGraph, EquationNode, EquationPoint, equation_snapshot_with_state, genesis_equation_child_pack};
pub use semio_s_artifact_mathematical_equation_mutations::{EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA, equation_child_restore_projection};
pub use crate::editor::equation::artifact_items_5::{artifact_kind};

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
                    pub use semio_s_artifact_mathematical_equation_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::io::text::snapshot;
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
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::move_nodes;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::set_node_positions;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::change_node_label;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::connect_nodes;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::disconnect_nodes;
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
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::set_point_positions;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_mathematical_equation_examples::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod equation {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod commands {
            #[path = "../../🎮️commands/➕️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../🎮️commands/🕸️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "../../🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "../../🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🎮️commands/🧮️set-algorithm/🦀️.rs"]
            pub mod set_algorithm;
            #[path = "../../🎮️commands/🧾️edit-equation/🦀️.rs"]
            pub mod edit_equation;
            #[path = "../../🎮️commands/🧭️set-directed/🦀️.rs"]
            pub mod set_directed;
            #[path = "../../🎮️commands/📏️edit-points/🦀️.rs"]
            pub mod edit_points;

        #[cfg(test)]
        #[path = "../../🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../🎭️modes/✏️edit/🪟️windows/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                    #[path = "../../🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
                }
            }
        }
    }
}
