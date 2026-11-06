# Snapshot broker law source review

The mounted checked_snapshot_media_export_matches_complete_materialized_record test publishes and asserts AddStream before capturing expected state, drains successful output, decodes the entire record, compares full ToValue JSON, and checks original document identity, revision and materialized state afterward. Serde is used to parse JSON values, not to serialize the snapshot.

Cancellation rows have a concrete coverage gap: the poll loop may complete before a requested cut and the final condition only asserts that a cancellation implies a cut. A cut can therefore pass without cancellation. Require each cut row to reach and observe Cancelled. The owned before and restored snapshots use implicit drops; revisit explicit paid retirement when OrderedMap-backed values require it. No execution or pass claim.

Updated law source now asserts every cancellation cut reached and actual Cancelled terminal, and performs two complete exports from the same unchanged app with canonical byte equality. No additional concrete source defect established; not executed.
