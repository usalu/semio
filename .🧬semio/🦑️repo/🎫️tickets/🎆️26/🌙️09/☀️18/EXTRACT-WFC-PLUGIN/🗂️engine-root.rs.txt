//! 🀄️ Wave function collapse engine shared by every `wfc` plugin artifact. Domain agnostic: it
//! speaks only `PatternId`/`NodeId`/`RelationId`, never an artifact's document types. Every module
//! is production code — the only test-gated ones are the differential oracle (`oracle`) and the
//! shared model vectors (`model_vectors`), which exist purely to cross-check the engine.

#![allow(clippy::result_large_err)]

#[path = "🔦️beam/🦀️.rs"]
pub mod beam;
#[path = "🎛️bitset/🦀️.rs"]
pub mod bitset;
#[path = "🍰️chunk/🦀️.rs"]
pub mod chunk;
#[path = "⛓️constraint/🦀️.rs"]
pub mod constraint;
#[path = "🔢️constraints-card/🦀️.rs"]
pub mod constraints_card;
#[path = "🔗️constraints-conn/🦀️.rs"]
pub mod constraints_conn;
#[path = "🩺️diag/🦀️.rs"]
pub mod diag;
#[path = "🌐️domain/🦀️.rs"]
pub mod domain;
#[path = "⚠️error/🦀️.rs"]
pub mod error;
#[path = "🧬️evolve/🦀️.rs"]
pub mod evolve;
#[path = "⛏️extract/🦀️.rs"]
pub mod extract;
#[path = "🌊️flow/🦀️.rs"]
pub mod flow;
#[path = "🔲️grid-2d/🦀️.rs"]
pub mod grid2d;
#[path = "🧊️grid-3d/🦀️.rs"]
pub mod grid3d;
#[path = "🧭️heuristics/🦀️.rs"]
pub mod heuristics;
#[path = "🪜️hierarchy/🦀️.rs"]
pub mod hierarchy;
#[path = "🆔️ids/🦀️.rs"]
pub mod ids;
#[path = "💼️job/🦀️.rs"]
pub mod job;
#[path = "🏗️model/🦀️.rs"]
pub mod model;
#[cfg(test)]
#[path = "🧪️tests/🧮️model-vectors/🦀️.rs"]
pub mod model_vectors;
#[path = "🎼️motif/🦀️.rs"]
pub mod motif;
#[path = "🚫️nogood/🦀️.rs"]
pub mod nogood;
#[cfg(test)]
#[path = "🔮️oracles/🦀️.rs"]
pub mod oracle;
#[path = "🏁️outcome/🦀️.rs"]
pub mod outcome;
#[path = "🧵️parallel/🦀️.rs"]
pub mod parallel;
#[path = "🔁️prop-ac3/🦀️.rs"]
pub mod prop_ac3;
#[path = "🔄️prop-ac4/🦀️.rs"]
pub mod prop_ac4;
#[path = "📣️propagate/🦀️.rs"]
pub mod propagate;
#[path = "🔧️repair/🦀️.rs"]
pub mod repair;
#[path = "🎲️sample/🦀️.rs"]
pub mod sample;
#[path = "🔍️search/🦀️.rs"]
pub mod search;
#[path = "💾️serial/🦀️.rs"]
pub mod serial;
#[path = "🪶️soft/🦀️.rs"]
pub mod soft;
#[path = "🕸️solver-graph/🦀️.rs"]
pub mod solver_graph;
#[path = "🔳️solver-grid-2d/🦀️.rs"]
pub mod solver_grid2d;
#[path = "🧱️solver-grid-3d/🦀️.rs"]
pub mod solver_grid3d;
#[path = "🕳️sparse-3d/🦀️.rs"]
pub mod sparse3d;
#[path = "🪞️symmetry/🦀️.rs"]
pub mod symmetry;
#[path = "🀄️tiled/🦀️.rs"]
pub mod tiled;
#[path = "🗺️topology/🦀️.rs"]
pub mod topology;
#[path = "🐾️trail/🦀️.rs"]
pub mod trail;
#[path = "⚖️weights/🦀️.rs"]
pub mod weights;

#[cfg(test)]
#[path = "🧪️tests/🔬️grid-job-drive/🦀️.rs"]
mod grid_job_drive_tests;
