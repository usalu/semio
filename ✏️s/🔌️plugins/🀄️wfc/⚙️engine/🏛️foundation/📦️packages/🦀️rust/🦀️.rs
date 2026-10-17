//! 🏛️ Foundation of the wfc engine: ids, bitsets, weights, the compiled model, domains, topology and the propagation primitives every solver builds on.

#![allow(clippy::result_large_err)]

#[path = "../../../🎛️bitset/🦀️.rs"]
pub mod bitset;
#[path = "../../../🌐️domain/🦀️.rs"]
pub mod domain;
#[path = "../../../⚠️error/🦀️.rs"]
pub mod error;
#[path = "../../../🧭️heuristics/🦀️.rs"]
pub mod heuristics;
#[path = "../../../🆔️ids/🦀️.rs"]
pub mod ids;
#[path = "../../../🏗️model/🦀️.rs"]
pub mod model;
#[path = "../../../📣️propagate/🦀️.rs"]
pub mod propagate;
#[path = "../../../🎲️sample/🦀️.rs"]
pub mod sample;
#[path = "../../../🪶️soft/🦀️.rs"]
pub mod soft;
#[path = "../../../🗺️topology/🦀️.rs"]
pub mod topology;
#[path = "../../../⚖️weights/🦀️.rs"]
pub mod weights;
