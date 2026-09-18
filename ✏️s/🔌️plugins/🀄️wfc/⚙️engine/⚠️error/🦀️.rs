//! 🚨️ Every way building a model/topology/constraint or configuring a solve can fail validation.
//! Kept flat (no nested `source()` chain, no external error crate) so callers can match
//! exhaustively — the entropy crate's convention. `Contradiction`/`Unsatisfiable` are normal
//! `SolveOutcome` variants, never errors: a search finding no solution is not a
//! bug, a malformed model or an internal invariant violation is.

// #region 🔖️ModelError
/// 🚨️ Everything that can go wrong while building or compiling a [`crate::model::CompiledModel`].
#[derive(Clone, PartialEq, Debug)]
pub enum ModelError {
    /// 🚨️ A model was compiled with zero patterns.
    EmptyPatternUniverse,
    /// 🚨️ A weight failed validation (`NaN`, infinite, or negative).
    InvalidWeight { pattern_index: usize, value: f64 },
    /// 🚨️ `allowed[r][a].get(b) != allowed[inv(r)][b].get(a)` — the declared inverse relation is
    /// not actually the transpose of the forward relation's compatibility table.
    AsymmetricInverse { relation: crate::ids::RelationId },
    /// 🚨️ A checked multiplication/addition needed to size an internal table overflowed.
    CapacityOverflow { what: &'static str },
    /// 🚨️ A symmetry transform did not close under composition/inverse (generator set is broken).
    InvalidSymmetryGroup { reason: &'static str },
    /// 🚨️ A `SourceModelDoc`'s schema version does not match this build's. No
    /// migration — this crate has no users yet, so an unrecognized version is simply rejected.
    SchemaVersionMismatch { expected: u32, actual: u32 },
}

impl core::fmt::Display for ModelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyPatternUniverse => write!(f, "model has zero patterns"),
            Self::InvalidWeight { pattern_index, value } => {
                write!(f, "invalid weight at pattern index {pattern_index}: {value}")
            }
            Self::AsymmetricInverse { relation } => {
                write!(f, "relation {relation} and its declared inverse disagree on compatibility (not a true transpose)")
            }
            Self::CapacityOverflow { what } => write!(f, "capacity overflow computing {what}"),
            Self::InvalidSymmetryGroup { reason } => write!(f, "invalid symmetry group: {reason}"),
            Self::SchemaVersionMismatch { expected, actual } => {
                write!(f, "source model schema version mismatch: expected {expected}, found {actual}")
            }
        }
    }
}

impl std::error::Error for ModelError {}
// #endregion 🔖️ModelError

// #region 🔖️TopologyError
/// 🚨️ Everything that can go wrong while building a grid or graph topology.
#[derive(Clone, PartialEq, Debug)]
pub enum TopologyError {
    /// 🚨️ A grid dimension was zero where the topology forbids it.
    ZeroDimension { axis: &'static str },
    /// 🚨️ `width * height` (or `* depth`) overflowed its checked integer type.
    SizeOverflow,
    /// 🚨️ A mask's length did not match `width * height` (`* depth`).
    MaskShapeMismatch { expected: usize, actual: usize },
    /// 🚨️ An arc referenced a node that does not exist (e.g. after `from_graph_view` truncation).
    DanglingArc { from: crate::ids::NodeId },
    /// 🚨️ A custom stencil declared the same offset twice, or a self-offset without opting in.
    InvalidStencil { reason: &'static str },
    /// 🚨️ A node count exceeded `u32::MAX`, the limit `crate::ids::NodeId` can address.
    TooManyNodes { count: u64 },
}

impl core::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroDimension { axis } => write!(f, "grid dimension `{axis}` must be nonzero"),
            Self::SizeOverflow => write!(f, "grid size computation overflowed"),
            Self::MaskShapeMismatch { expected, actual } => {
                write!(f, "mask length mismatch: expected {expected}, found {actual}")
            }
            Self::DanglingArc { from } => write!(f, "arc references a nonexistent node from {from}"),
            Self::InvalidStencil { reason } => write!(f, "invalid stencil: {reason}"),
            Self::TooManyNodes { count } => write!(f, "{count} nodes exceeds the u32 node-id capacity"),
        }
    }
}

impl std::error::Error for TopologyError {}
// #endregion 🔖️TopologyError

// #region 🔖️ConstraintError
/// 🚨️ Everything that can go wrong while configuring a global/soft constraint.
#[derive(Clone, PartialEq, Debug)]
pub enum ConstraintError {
    /// 🚨️ A cardinality/distance bound was internally inconsistent (e.g. `min > max`).
    InvalidBounds { reason: &'static str },
    /// 🚨️ A tuple-table constraint was given zero tuples.
    EmptyTupleTable,
}

impl core::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidBounds { reason } => write!(f, "invalid constraint bounds: {reason}"),
            Self::EmptyTupleTable => write!(f, "tuple-table constraint has zero tuples"),
        }
    }
}

impl std::error::Error for ConstraintError {}
// #endregion 🔖️ConstraintError

// #region 🔖️SolveError
/// 🚨️ Everything that can go wrong configuring or resuming a solve (as opposed to the solve
/// itself finding no solution, which is a `SolveOutcome`).
#[derive(Clone, PartialEq, Debug)]
pub enum SolveError {
    /// 🚨️ A solver was built from a model and topology whose relation universes disagree.
    ModelTopologyMismatch { reason: &'static str },
    /// 🚨️ Strict-integer determinism was requested but the model has no integer weight table.
    SeedMissingInStrictMode,
    /// 🚨️ A checkpoint's format/schema version does not match this build.
    CheckpointVersionMismatch { expected: u32, actual: u32 },
    /// 🚨️ A checkpoint failed structural revalidation (bitset length, index bound, or fingerprint).
    CorruptCheckpoint { reason: &'static str },
    /// 🚨️ A fixed pattern/domain restriction was given for a node outside the topology.
    UnknownNode(crate::ids::NodeId),
}

impl core::fmt::Display for SolveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ModelTopologyMismatch { reason } => write!(f, "model/topology mismatch: {reason}"),
            Self::SeedMissingInStrictMode => write!(f, "strict-integer mode requires an all-integer weight table"),
            Self::CheckpointVersionMismatch { expected, actual } => {
                write!(f, "checkpoint version mismatch: expected {expected}, found {actual}")
            }
            Self::CorruptCheckpoint { reason } => write!(f, "corrupt checkpoint: {reason}"),
            Self::UnknownNode(n) => write!(f, "unknown node id {n}"),
        }
    }
}

impl std::error::Error for SolveError {}
// #endregion 🔖️SolveError

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
