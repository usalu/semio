//! 🚨️ Every way building a model/topology/constraint or configuring a solve can fail validation.
//! Kept flat (no nested `source()` chain, no external error crate) so callers can match
//! exhaustively — the entropy crate's convention. `Contradiction`/`Unsatisfiable` are normal
//! [`crate::wfc_engine::outcome::SolveOutcome`] variants, never errors: a search finding no solution is not a
//! bug, a malformed model or an internal invariant violation is.

// #region 🔖️ModelError
/// 🚨️ Everything that can go wrong while building or compiling a [`crate::wfc_engine::model::CompiledModel`].
#[derive(Clone, PartialEq, Debug)]
pub enum ModelError {
    /// 🚨️ A model was compiled with zero patterns.
    EmptyPatternUniverse,
    /// 🚨️ A `PatternId`/`TileId`/`RelationId`/`PortId` referenced during building was never added.
    UnknownPattern(crate::wfc_engine::ids::PatternId),
    UnknownTile(crate::wfc_engine::ids::TileId),
    UnknownRelation(crate::wfc_engine::ids::RelationId),
    UnknownPort(crate::wfc_engine::ids::PortId),
    /// 🚨️ The same relation name/id was registered twice.
    DuplicateRelation(crate::wfc_engine::ids::RelationId),
    /// 🚨️ A weight failed validation (`NaN`, infinite, or negative).
    InvalidWeight {
        pattern_index: usize,
        value: f64,
    },
    /// 🚨️ `allowed[r][a].get(b) != allowed[inv(r)][b].get(a)` — the declared inverse relation is
    /// not actually the transpose of the forward relation's compatibility table.
    AsymmetricInverse {
        relation: crate::wfc_engine::ids::RelationId,
    },
    /// 🚨️ A checked multiplication/addition needed to size an internal table overflowed.
    CapacityOverflow {
        what: &'static str,
    },
    /// 🚨️ A symmetry transform did not close under composition/inverse (generator set is broken).
    InvalidSymmetryGroup {
        reason: &'static str,
    },
    /// 🚨️ A socket rule referenced a socket label that was never declared compatible with anything.
    IncompatibleSocketRule {
        reason: &'static str,
    },
    /// 🚨️ A [`crate::wfc_engine::serial::SourceModelDoc`]'s schema version does not match this build's. No
    /// migration — this crate has no users yet, so an unrecognized version is simply rejected.
    SchemaVersionMismatch {
        expected: u32,
        actual: u32,
    },
}

impl core::fmt::Display for ModelError {
    async fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyPatternUniverse => write!(f, "model has zero patterns"),
            Self::UnknownPattern(p) => write!(f, "unknown pattern id {p}"),
            Self::UnknownTile(t) => write!(f, "unknown tile id {t}"),
            Self::UnknownRelation(r) => write!(f, "unknown relation id {r}"),
            Self::UnknownPort(p) => write!(f, "unknown port id {p}"),
            Self::DuplicateRelation(r) => write!(f, "relation {r} registered twice"),
            Self::InvalidWeight { pattern_index, value } => {
                write!(f, "invalid weight at pattern index {pattern_index}: {value}")
            }
            Self::AsymmetricInverse { relation } => {
                write!(f, "relation {relation} and its declared inverse disagree on compatibility (not a true transpose)")
            }
            Self::CapacityOverflow { what } => write!(f, "capacity overflow computing {what}"),
            Self::InvalidSymmetryGroup { reason } => write!(f, "invalid symmetry group: {reason}"),
            Self::IncompatibleSocketRule { reason } => write!(f, "incompatible socket rule: {reason}"),
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
    /// 🚨️ A referenced `NodeId` is out of range for this topology.
    UnknownNode(crate::wfc_engine::ids::NodeId),
    /// 🚨️ An arc referenced a node that does not exist (e.g. after `from_graph_view` truncation).
    DanglingArc { from: crate::wfc_engine::ids::NodeId },
    /// 🚨️ A custom stencil declared the same offset twice, or a self-offset without opting in.
    InvalidStencil { reason: &'static str },
    /// 🚨️ A boundary mode is incompatible with the requested grid size (e.g. `Mirror` on a
    /// size-0 axis) or with another configured boundary on the same axis.
    BoundaryIncompatible { reason: &'static str },
    /// 🚨️ A node count exceeded `u32::MAX`, the limit `crate::wfc_engine::ids::NodeId` can address.
    TooManyNodes { count: u64 },
}

impl core::fmt::Display for TopologyError {
    async fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroDimension { axis } => write!(f, "grid dimension `{axis}` must be nonzero"),
            Self::SizeOverflow => write!(f, "grid size computation overflowed"),
            Self::MaskShapeMismatch { expected, actual } => {
                write!(f, "mask length mismatch: expected {expected}, found {actual}")
            }
            Self::UnknownNode(n) => write!(f, "unknown node id {n}"),
            Self::DanglingArc { from } => write!(f, "arc references a nonexistent node from {from}"),
            Self::InvalidStencil { reason } => write!(f, "invalid stencil: {reason}"),
            Self::BoundaryIncompatible { reason } => write!(f, "incompatible boundary configuration: {reason}"),
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
    InvalidBounds {
        reason: &'static str,
    },
    /// 🚨️ A referenced region/tag was never declared.
    UnknownRegion(crate::wfc_engine::ids::RegionId),
    UnknownTag(u32),
    /// 🚨️ A tuple-table constraint was given zero tuples.
    EmptyTupleTable,
    /// 🚨️ A tuple in a tuple-table constraint did not match the declared node-scope arity.
    ArityMismatch {
        expected: usize,
        actual: usize,
    },
    /// 🚨️ A constraint referenced a node outside the topology.
    UnknownNode(crate::wfc_engine::ids::NodeId),
}

impl core::fmt::Display for ConstraintError {
    async fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidBounds { reason } => write!(f, "invalid constraint bounds: {reason}"),
            Self::UnknownRegion(r) => write!(f, "unknown region id {r}"),
            Self::UnknownTag(t) => write!(f, "unknown tag id {t}"),
            Self::EmptyTupleTable => write!(f, "tuple-table constraint has zero tuples"),
            Self::ArityMismatch { expected, actual } => {
                write!(f, "tuple arity mismatch: expected {expected}, found {actual}")
            }
            Self::UnknownNode(n) => write!(f, "unknown node id {n}"),
        }
    }
}

impl std::error::Error for ConstraintError {}
// #endregion 🔖️ConstraintError

// #region 🔖️SolveError
/// 🚨️ Everything that can go wrong configuring or resuming a solve (as opposed to the solve
/// itself finding no solution, which is a [`crate::wfc_engine::outcome::SolveOutcome`]).
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
    UnknownNode(crate::wfc_engine::ids::NodeId),
}

impl core::fmt::Display for SolveError {
    async fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
mod tests {
    use super::*;

    #[test]
    async fn display_messages_are_human_readable() {
        let e = ModelError::InvalidWeight { pattern_index: 3, value: -1.0 };
        assert_eq!(e.to_string(), "invalid weight at pattern index 3: -1");

        let t = TopologyError::ZeroDimension { axis: "width" };
        assert_eq!(t.to_string(), "grid dimension `width` must be nonzero");

        let c = ConstraintError::EmptyTupleTable;
        assert_eq!(c.to_string(), "tuple-table constraint has zero tuples");

        let s = SolveError::CheckpointVersionMismatch { expected: 1, actual: 2 };
        assert_eq!(s.to_string(), "checkpoint version mismatch: expected 1, found 2");
    }

    #[test]
    async fn errors_are_std_error() {
        async fn assert_std_error<E: std::error::Error>(_e: &E) {}
        assert_std_error(&ModelError::EmptyPatternUniverse);
        assert_std_error(&TopologyError::SizeOverflow);
        assert_std_error(&ConstraintError::EmptyTupleTable);
        assert_std_error(&SolveError::SeedMissingInStrictMode);
    }
}
// #endregion 🔖️Tests
