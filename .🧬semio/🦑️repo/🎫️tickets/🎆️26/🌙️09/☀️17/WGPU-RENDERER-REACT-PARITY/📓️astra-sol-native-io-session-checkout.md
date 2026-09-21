# Native I/O Session Checkout

## Actual failure

The services full attempt used Nextest `48218461-6823-4fad-a60b-64b8dc16d97e`: 45 tests ran, 44 passed, and one failed in 426 ms. The retained fixed-file-document law passed. The separate existing `native_io::tests::chunked_read_write_scan_and_modified_round_trip` failed at `native-io-unit/🦀️.rs:35` with `native I/O checked-out job`.

The failure is in the test's `run` driver rather than the read/write implementation. `BatchJobSession::step` leaves the completed owner inside the session. `checked_out_job_mut` can expose it only after `checkout_outcome` transfers that owner into the session's checked-out slot. The native I/O driver accessed the job first. The renderer frame-job test already uses the canonical order: step, `checkout_outcome`, inspect the checked-out job, then `take_outcome` and close every owner.

## Repair

The test driver now requires `session.checkout_outcome()` immediately after every successful step and before reading `NativeIoJob::take_result`. Failure to transfer the exact owner remains an assertion; no fallback result, retry, production I/O change, capacity change, or retained-document change was added. The existing outcome close, session resume, and terminal close ladders remain unchanged.

Rustfmt parsed the edited test source and `git diff --check` passed. No Cargo command was run locally. The root-owned post-fix filter is:

```text
chunked_read_write_scan_and_modified_round_trip
```

The coordinator-owned full services gate later verified the repair: Nextest run `4df2f746-2248-48d3-a42c-64163cfe6bc3` completed 45/45 PASS, 0 skipped, in 261 ms; its Nx command completed in 17.8 seconds. Receipt metadata is retained at `🗑️generated/astra-runtime/services-green-5/metadata/semio-nextest-YCOXQq`.

## Retained-value close correction

The first selected long-profile retry did not complete. The owned process was sampled after more than one minute at about 52% CPU. Its stack was `run` → `BatchJobSession::close_step` → `WorkerJobSession::close_step` → `JobPayloadOperationLedger::terminal_is_empty`. The run was interrupted, so it is RED evidence rather than a completed result.

The checkout order exposed a second fixture defect: the terminal `NativeIoValue::Page` still owned its retained payload while `run` tried to close the operation/session that accounted for that payload. The close ladder therefore correctly waited for an owner the test planned to return later. The test boundary now materializes every successful `NativeIoValue` into a test-only value first. Payloads become plain `Vec<u8>` values through the existing bounded reader and close ladder; fixed path and modified sets are drained into ordinary vectors. Only after those retained owners are empty does the helper close the session. Production I/O values, payload accounting, page limits, and session behavior are unchanged.

The interrupted stack sample was retained by the root lane at `🗑️generated/astra-runtime/native-io-hang.sample.txt`. The subsequent full services gate above proves both this native-I/O lifecycle repair and the retained-document generation-floor law in the same 45/45 run.
