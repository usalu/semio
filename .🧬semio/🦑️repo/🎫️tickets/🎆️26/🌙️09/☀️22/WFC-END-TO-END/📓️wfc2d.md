# 🀄️ wfc2d fill slice

## Outcome

Interactive non-mutating `fill` / `s.wfc.wfc2d.fill.run` under `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d`.

- Live preview paints the fill tick payload while the run is non-terminal (`ToolRunView::payload` → preview window).
- Completion writes the existing `SetSolve` transient via `commit-fill`; abort does not.
- Tick assignments read `WfcJob::preview()` / `observed()` / `domain_masks()` in process — not the truncated published `incomplete_grid`.
- Edit mode lists `fill` (`ModeDefinition.tools`); editor registers `.tool(fill::definition())`.
- Child publication uses one payload-page opportunity per step: after `PreviewReady`/`CheckpointReady` the parent defers its tick (`pending_tick`) and after `Complete` defers `finish_success` (`pending_finish`), matching grid2d. On `Yield` the child is restored and the parent returns `Yield` without publishing.

## Tests added (fill)

- `the_tool_declares_the_read_only_wfc2d_fill_run`
- `the_language_agnostic_partial_vector_decodes_to_the_normative_shape`
- `stepping_publishes_a_strictly_increasing_partial_before_the_finish`
- `aborting_mid_run_cancels_and_never_writes_set_solve`
- `the_final_payload_matches_solve_with_job_for_hex_ring`

Did not duplicate live solver outcome tests for `two-room-corridor`, `wall-roof-facade-strip`, `hex-ring`, `terrain-ring`.

## Tests under `🚪️io/`

- `io_declares_no_foreign_stdio_hops`
- `native_snapshot_text_round_trips_every_example`
- `native_snapshot_pack_round_trips_every_example`
- `native_mutation_text_and_binary_round_trip_change_seed`

## Oracle

`📜️wfc2d-fill-oracle.py` validates the language-agnostic partial-tick fixture; Rust vector test keeps the same shape.

## Cargo

```
export RUST_MIN_STACK=33554432
cargo test -p semio-s-artifact-wfc-2d --features component-app-assembly --lib -j 4 -- --test-threads=4
```

test result: ok. 200 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

## Blockers

None.
