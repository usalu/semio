
use super::*;

#[test]
fn display_messages_are_human_readable() {
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
fn errors_are_std_error() {
    fn assert_std_error<E: std::error::Error>(_e: &E) {}
    assert_std_error(&ModelError::EmptyPatternUniverse);
    assert_std_error(&TopologyError::SizeOverflow);
    assert_std_error(&ConstraintError::EmptyTupleTable);
    assert_std_error(&SolveError::SeedMissingInStrictMode);
}
