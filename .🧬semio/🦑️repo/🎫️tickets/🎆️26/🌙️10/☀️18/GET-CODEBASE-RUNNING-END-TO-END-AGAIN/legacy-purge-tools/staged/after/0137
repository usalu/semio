//! 🧮️ Equation artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_equation_demo_tests;
extern crate semio_framework_schema as framework_schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<EquationMutation, EquationGraphWindowConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
// 🚚 Wave M3a (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS): `cas`/
// `polynomial` migrated verbatim from `🧮️math`'s crate root (files physically relocated under
// `🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`, the facet's
// Rust-only compute internals a named inference's `compute()` delegates into — mirrors stdio's
// `📐️step` io facet's `🪜️ladder`/`📐️part21`/`🧱️brep` precedent for deep Rust-only helper dirs under a
// facet). Mounted DIRECTLY at crate root, exactly as `🧮️math`'s own glue.rs mounted them — every
// `crate::cas::…`/`crate::polynomial::…` self-reference inside those two files (including references
// to non-`pub` inner modules, e.g. `crate::cas::canon`) is untouched, and privacy is structural in
// Rust: a re-export/alias layer (`pub use … as cas`) does NOT leak private inner items back out, so
// only a direct mount preserves them. `crate::algebra`'s two call sites became `math::algebra::MatG`/
// `math::algebra::VecG` against a since-removed `semio_framework_math as math` dependency — briefly a
// genuine pre-existing breakage (wave M3d had moved `algebra` out of `semio_framework_math` into
// `📸️remodel`, not knowing this wave had just created a second consumer here). Wave FIXALG (same
// ticket) relocated `VecG`/`MatG` out of `📸️remodel` into `semio_framework_number`'s own `algebra`
// module and repointed both sites at `number::MatG`/`number::VecG`, so `math` is no longer a
// dependency of this crate at all. `crate::number` became `number::` (wave MATHEND) against the
// `semio_framework_number` dependency below — `number` was relocated out of `🧮️math` into its own
// framework module, so it is a real top-level extern crate now, not a submodule of `math`.
#[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs"]
pub mod cas;
#[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs"]
pub mod polynomial;

#[cfg(test)]
use semio_framework_value::{FromValue, ToValue};

use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;

pub use semio_s_artifact_mathematical_equation_mutations::{EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};

pub use semio_s_artifact_mathematical_equation_snapshot::{EquationEdge, EquationGeometry, EquationGraph, EquationNode, EquationPoint};

//#region 🔖️CameraType
pub use crate::editor::equation::modes::edit::windows::graph::config::EquationCamera;
//#endregion 🔖️CameraType

//#region 🔖️CarrierTypes
pub use crate::snapshot::schema::{EquationExprSnapshot, EquationCarrierSnapshot};
//#endregion 🔖️CarrierTypes

pub use semio_s_artifact_mathematical_equation_snapshot::{EquationComputedChild, EquationNotationChild, EquationResultsChild};

pub use semio_s_artifact_mathematical_equation_snapshot::{equation_carrier_snapshot, equation_children, equation_computed_from_state, equation_derived_child, equation_derived_pack, equation_notation_from_graph, equation_results_from_graph, equation_snapshot_from_host_snapshot, equation_snapshot_with_state, genesis_equation_child_pack};

pub use semio_s_artifact_mathematical_equation_editor::{artifact_kind};

pub use semio_s_artifact_mathematical_equation_leaf_tests::{EquationApplication, artifact, definition, pilot_languages};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_mathematical_equation_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_mathematical_equation_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_mathematical_equation_examples::standards::v1::subsets::any::io::binary::inferences;
}
pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::io::sqlite;
pub mod text {
    pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_mathematical_equation_examples::standards::v1::subsets::any::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::csv;
                                pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::md;
                                pub use semio_s_artifact_mathematical_equation_leaf_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::md;
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
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
                        pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::graph::schema::mutations::change_graph_directed;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::graph::schema::mutations::create_node;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::graph::schema::mutations::delete_node;
                        pub use semio_s_artifact_mathematical_equation_mutations::standards::v1::subsets::graph::schema::mutations::delete_nodes;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::move_nodes;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::set_node_positions;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::change_node_label;
                        pub use semio_s_artifact_mathematical_equation_update_graph_algorithm::standards::v1::subsets::graph::schema::mutations::move_node;
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
                        pub use semio_s_artifact_mathematical_equation_mutations::standards::v1::subsets::geometry::schema::mutations::move_points;
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

                        }
                    }
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
}

pub use semio_s_artifact_mathematical_equation_inferences::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_mathematical_equation_snapshot::snapshot;
pub use crate::standards::v1::subsets::any::schema::diff::EquationDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;

pub use semio_s_artifact_mathematical_equation_examples::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod equation {
        pub use semio_s_artifact_mathematical_equation_editor::editor::equation::*;
pub use semio_s_artifact_mathematical_equation_examples::editor::equation::demo_session;

        pub use semio_s_artifact_mathematical_equation_editor::editor::equation::commands;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::windows::geometry;
                    pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::windows::graph;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        pub use semio_s_artifact_mathematical_equation_mutations::viewer::equation::*;

        pub use semio_s_artifact_mathematical_equation_examples::viewer::equation::presence;
        pub use semio_s_artifact_mathematical_equation_examples::viewer::equation::config;

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

pub use semio_s_artifact_mathematical_equation_mutations::{equation_child_restore_projection};

pub use semio_s_artifact_mathematical_equation_mutations::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_3;
