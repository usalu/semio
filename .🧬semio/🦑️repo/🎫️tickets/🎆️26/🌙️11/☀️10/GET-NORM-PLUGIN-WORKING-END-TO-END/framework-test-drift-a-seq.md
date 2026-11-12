# drift-a seq: replication sequence and paged-digest tests

## Changed (sources only, not yet verified by a full compile)
- `📡️replication/📦️packages/🦀️rust/Cargo.toml`: dev-deps `semio-framework-value-clone-source-testing` and `semio-framework-value-testing-effect-bank`.
- `🔺️diff/🪢️sequence/🧪️tests/🦀️.rs` migrated:
  - `Snapshot` now derives `SnapshotProjection` (record cursor). `OwnedFieldProjection` has no public constructor, so the counting wrapper and the `PROJECTIONS` counter are gone. The pointer-stability assertions for single-edit sequences remain.
  - `EditApply` and `SequenceApply` implement the new `RetainedDiffApply` (work demands, `preflight_reply`, effect-threaded `commit`, paid `close_reply`). Fields are installed via `take_into`, and the diagnostic is closed with `begin_close` + `close_into`.
  - Sources are `FixtureSource::from_owner`. Owner calls run under `drive_reply`/`drive`, with a `ControlledRetirement` drain helper.
- `🚪️io/.../🔏️seal/🧪️tests/📄️paged-digest/🦀️.rs`: sources come from `admit_owned` on an `EffectBank`, and `cursor.advance` and the retirement run under `drive`, with units taken from `next_work_demand` / `next_computational_work`.

## Library gap
`RetainedDiffSequence` has no `preflight_work_demand` or `close_work_demand`. `SequenceApply` reports the ceiling `max` for preflight and 1 for close; these are not framework quotes.

## Status
Check not completed: the cargo queue stalled and the coordinator asked me to stop my runs. The last fresh combined log (drift-a-all.log, 12:11) showed only closure return-type errors and one `bank` lifetime error, both since fixed. Test results are still pending, and the test run is yet to be done.
