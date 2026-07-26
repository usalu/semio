//! 🌊 Wave Function Collapse as a finite-domain constraint solver: one propagation/search kernel
//! under three solvers (arbitrary graphs, dense 2D grids, dense 3D grids), with compiled
//! tile/pattern models, global constraints, and deterministic replayable search.

// #region 🔖Modules
#[path = "src/ids.rs"]
pub mod ids;
#[path = "src/bitset.rs"]
pub mod bitset;
#[path = "src/weights.rs"]
pub mod weights;
#[path = "src/error.rs"]
pub mod error;
#[path = "src/domain.rs"]
pub mod domain;
#[path = "src/model.rs"]
pub mod model;
#[path = "src/oracle.rs"]
pub mod oracle;
#[path = "src/tiled.rs"]
pub mod tiled;
#[path = "src/topology.rs"]
pub(crate) mod topology;
#[path = "src/propagate.rs"]
pub(crate) mod propagate;
#[path = "src/prop_ac3.rs"]
pub(crate) mod prop_ac3;
#[path = "src/heuristics.rs"]
pub mod heuristics;
#[path = "src/sample.rs"]
pub mod sample;
#[path = "src/trail.rs"]
pub(crate) mod trail;
#[path = "src/search.rs"]
pub mod search;
#[path = "src/outcome.rs"]
pub mod outcome;
#[path = "src/diag.rs"]
pub mod diag;
#[path = "src/solver_graph.rs"]
pub mod solver_graph;
// #endregion 🔖Modules

// #region 🔖Exports
pub use bitset::PatternSet;
pub use diag::{DiagLevel, Event, EventSink, Metrics};
pub use domain::{Domain, DomainStore, RestrictResult};
pub use error::{ConstraintError, ModelError, SolveError, TopologyError};
pub use heuristics::ObserveHeuristic;
pub use ids::{ConstraintId, DecisionId, NodeId, PatternId, PortId, RegionId, RelationId, TileId};
pub use model::{CompiledModel, LintFinding, ModelBuilder, ModelStats, PatternInfo, RelationInfo};
pub use oracle::{ArcSpec, OracleResult, Violation, check_assignment, enumerate};
pub use outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
pub use sample::ValueSampler;
pub use search::{Budget, CancelToken, RestartSchedule, SearchConfig, SearchMode};
pub use solver_graph::{GraphSolver, GraphSolverBuilder};
pub use tiled::TiledModelBuilder;
pub use topology::{GraphTopology, GraphTopologyBuilder, from_graph_view};
pub use trail::Checkpoint;
pub use weights::{WeightMode, WeightTable, ZeroWeightPolicy};
// #endregion 🔖Exports
