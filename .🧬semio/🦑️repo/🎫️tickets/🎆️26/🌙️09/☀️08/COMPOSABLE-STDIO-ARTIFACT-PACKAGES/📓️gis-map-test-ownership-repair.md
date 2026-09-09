# GIS Map Test Ownership Repair

## Scope

The existing GIS Map native test binary reported 132 of 150 tests passing. This bounded repair owns the three failures caused by tests crossing retained ownership boundaries without installing or closing the exact owners already supplied by the GIS Map artifact.

## Repair

The document text/pack round-trip test and document VCS replay test now install `gis_map_document_store_owners()` immediately after constructing their store. This is the artifact's production catalog of `GisMapSnapshotRetirementFactory`, `GisMapMutationRetirementFactory`, and `ArtifactStoreCursorDisposer`; the tests do not substitute a generic or unbounded retirement path. Both stores are then cooperatively drained through `ArtifactDocumentStoreDisposer` before Drop.

The stale-generation initializer test now retains its `StepOutcome::Fault`, proves a zero-item grant releases no payload owner, drains at most one payload page per subsequent grant, verifies terminal emptiness, and only then drops the outcome. The initialization authority's existing terminal-empty assertion remains in place.

The first full component replay reached both strict disposers and exposed a lower Store ownership cycle. `Apply` retained the pre-edit `Arc` in `tail_undo_cache`, while `replace_current_retained` also queued another owner for the same root in the displaced-retirement cursor. The cursor must empty displaced owners before it reaches `TailSnapshot`; an exact `Arc::try_unwrap` retirement therefore blocked on the tail owner that the same cursor had not yet reached. `replace_current_retained` now recognizes that exact pointer transfer and releases only the redundant current handle, leaving the tail as the sole retirement owner. Redo now establishes the tail before replacing current so it obeys the same transfer order.

Two focused framework Store regressions preserve the boundary. An exact root factory proves Apply → Undo → Redo retires the displaced post-apply root, retained pre-redo tail, and live post-redo root once each. A separate live-reader case proves close blocks while a snapshot lease is outstanding, then resumes only after that exact reader returns to the Store registry. The production cursor and terminal-empty witnesses remain unchanged.

## Files

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`

## Validation

The coordinator's pre-repair full Map component replay ran 241 tests: 207 passed and 34 failed. The two owned Store cases both reached the strict disposer and failed specifically at the duplicated-tail root described above; that run is red evidence, not a pass. The coordinator owns the already queued current Map validation, so this lane did not launch a duplicate native build.

The first focused route was rejected as an ownership mismatch before it compiled: `@semio-tech/framework-rs:test-wire-retirement-native` targets the `semio-framework` facade, while these mounted Store laws belong to `semio-framework-os-kernel`. The wrong queued process chain was retired without touching other Cargo processes.

The coordinator's current durable-group build produced `semio_framework_os_kernel-00d4a62f6e957ed3`. Its test inventory contains both exact laws under `os_store::component::tests`. Replaying that current binary directly with exact filters gave:

```text
apply_undo_redo_transfers_each_snapshot_root_to_one_exact_retirement_owner: 1 passed, 0 failed, 0.06s
store_close_waits_for_a_live_snapshot_read_then_retires_its_returned_owner: 1 passed, 0 failed, 0.01s
overall exit: 0
```

Durable log: `🗑️generated/store-snapshot-ownership-current-kernel-binary.txt`.
