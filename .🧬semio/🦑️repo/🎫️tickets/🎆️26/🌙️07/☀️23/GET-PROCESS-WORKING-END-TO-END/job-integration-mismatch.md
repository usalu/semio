# Job Integration Mismatch

## Result

The three heap-observer vs receipt mismatches do not reproduce on the current tree. I made no product change.

I ran `semio-framework-job --features testkit --no-fail-fast` twice from a private script (`job2-cargo.sh`, own `CARGO_BUILD_BUILD_DIR` under `🗑️generated/native/build-job2`):

- Run 1, fresh build: 100 tests ok, exit 0.
- Run 2, after the tree moved on (the job context and `🌱️value` decode, record machine, slot and backing files were edited up to 01:16): 100 tests ok, exit 0. `semio-framework-job` was recompiled.

Both runs passed these three tests:

- `original_job_paged_source_preserves_header_authority_bytes_and_zero_heap_borrows`
- `original_in_place_payload_builder_preserves_fixed_grants_source_pages_and_all_cancellation_frontiers`
- `original_step_context_forwards_independent_work_funding`

## Debug residue check

- Grep of `🧰️framework/🔨️modules` for `[DEBUG]`:
  - `🌱️value/🦀️.rs` holds `debug_histogram`. Recording is thread-local `Cell` arithmetic with no allocation. Only `debug_units_report` allocates, and nothing outside the value crate calls it, so no observed window is affected.
  - The return-escrow `first_work_refusal` prints once on the first work refusal. It does not run on the passing path.
  - A wgpu dispatch `eprintln!` is unrelated to the job tests.
- None of these can add unreceipted heap to the three tests.

## Conclusion

The ~0.9 MB gap came from a transient tree. That is the same conclusion as `job-lib-regressions.md`. The likely cause is a mid-edit state of the slot close ladder or record machine while the brep agent changed it, with the birth-ledger, eager-backing and relay/receipt changes landing before the matching close-side receipts. The current tree is consistent. I could not measure the 0.9 MB itself without a failing tree, so I did not guess at a fix. No assertion or stock was changed.

If it recurs, rebuild in a clean build dir first. A stale incremental dir across concurrent edits can mix object states.

## Changed files

- Ticket only: `job2-cargo.sh` (private build script), `job-integration-mismatch.md`.
- No product files.

## Cleanup

`🗑️generated/native/build-job2` and the job2 logs are deleted.
