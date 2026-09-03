# DB Fault Taxonomy And Diagnostic Hub Residual

## Outcome

The retained database I/O terminal-fault path now preserves the reachable `DbError` category, exact bounded inner message, and required structured scalars without retaining a `DbError`, `String`, `Vec`, or foreign error owner. `DbIoFaultKind` remains runner provenance. A separate project-owned, fixed `DbIoFaultCause` is `Copy` and covers:

- `Io`, `NotFound`, `AlreadyExists`, `InvalidArgument`, `Conflict`, `Unavailable`, `Timeout`, `Corrupt`, `Closed`, `Unauthorized`, and `Internal`;
- `LimitExceeded(&'static str)` and `Unimplemented(&'static str)`;
- `Fenced { expected, actual }` and `StaleGeneration { expected, actual }`.

`db_io_task_fault` stores string variants' inner payload, not their rendered `Display`, in the existing fixed `DbIoText`. `DbIoFault::into_db_error` reconstructs the original category and structured scalars. The common laws compare the entire reconstructed `DbError` to the fixture value, so a doubled prefix such as `not found: not found: …` would fail.

Synthetic paths are explicit: cancellation becomes `Closed`, saturation becomes `Unavailable`, invalid typed steps become `InvalidArgument`, panic remains runner provenance `Panic` with cause `Internal`, and stale backend/generation failures retain their distinct cause and scalar witnesses.

`DbError::Rejected` is an artifact-engine policy outcome above the storage executor boundary. No production storage executor constructs it. A defensively injected occurrence is classified as bounded `Internal` with its exact layer-violation detail; the retained path never reconstructs `Rejected { messages: Vec::new() }`.

The blocking-lane panic path now catches while it still owns backend admission, clears `admitted_operation`, releases the registry guard, and only then resumes unwinding to the runner catcher. This preserves panic provenance without leaking operation authority or poisoning the repaired registry lock. Existing result handback, shutdown-close, page, task, backend, aggregate, and ledger retirement laws remain intact.

## Neutral Fixture And Independent Oracle

The neutral page-lifecycle fixture contains 15 fault cases:

`io`, `not_found`, `already_exists`, `invalid_argument`, `conflict`, `fenced`, `stale_generation`, `limit_exceeded`, `unavailable`, `timeout`, `corrupt`, `closed`, `unauthorized`, `unimplemented`, and `internal`.

Both a blocking executor and an independently driven async-native executor return these faults through actual retained operations. Each law verifies the terminal-result page phase, exact reconstructed `DbError`, a category oracle independent of strings, structured scalars, and terminal emptiness after result-handback and backend retirement.

SQLite's payload round trip now verifies `NotFound` before the first put and after delete, in addition to the existing arbitrary-byte/page-boundary round trip.

## Hub

`get_blob` lowers every `DbError` through `db_error_status`. Focused status cases assert 400 for `InvalidArgument`, 404 for `NotFound`, 409 for `Conflict`, 503 for `Unavailable`, and 500 for `Internal`. Missing blob GET remains exactly 404.

The private-directory WebSocket regression binds the second document's first decoded frame and accepts only `Welcome`. Any protocol error panic includes the full frame, code, and message, while every other unexpected frame panic includes the full frame. With the repaired taxonomy/lifecycle path, the exact test observes `Welcome`, then `Session`, and retains the existing global-identity and private-realtime isolation assertions.

No schema, migration, directory-authority, or wire-protocol changes were made.

## TDD Evidence

Every command below used this ticket-local build authority:

```sh
RUST_MIN_STACK=16777216
CARGO_BUILD_JOBS=2
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/sol-db-fault-taxonomy/target"
```

The valid pre-fix blocking law was red:

```sh
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_blocking_fault_preserves_exact_category_scalars_and_retires -- --nocapture
```

Result: 1 executed, 0 passed, 1 failed, 581 filtered. The first `NotFound` case returned the old flattened `Internal`/other category.

Final common DB focused commands:

```sh
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_blocking_fault_preserves_exact_category_scalars_and_retires -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_async_native_fault_preserves_exact_category_scalars_and_retires -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_artifact_rejection_is_an_internal_executor_boundary_violation -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_cancellation_before_during_and_receiver_drop_retain_exact_terminal_owners -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_panic_backend_fault_and_shutdown_close_reach_exact_prior_witness -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_executing_output_seal_keeps_every_page_executing_until_atomic_publication -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_output_task_yield_cancel_abandon_and_close_retire_exactly_once -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_page_identity_rejects_generation_operation_and_phase_mismatches_exactly -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db db_io_fixed_page_max_plus_one_and_zero_are_exact -- --nocapture
```

Each command executed exactly 1 test and passed; each reported 582 filtered. The final panic-law rerun after releasing the registry guard completed in 0.62 seconds. Its hostile fixture panic is expected output captured by the runner; the test passed and verified terminal retirement.

SQLite:

```sh
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- -p semio-framework-os-kernel-db --features sqlite payload_roundtrip_obeys_neutral_page_boundaries_and_arbitrary_bytes -- --nocapture
```

Result: 1 executed, 1 passed, 587 filtered.

DB production check, rerun after the final panic cleanup:

```sh
bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- -p semio-framework-os-kernel-db --all-features --message-format short
```

Result: exit 0; `semio-framework-os-kernel-db` finished the all-feature dev check. Existing workspace warnings remain.

Hub all-feature compile and focused tests:

```sh
bun nx run 'os-hub:test-quick' --skip-nx-cache -- db_errors_lower_to_exact_http_status_classes
bun nx run 'os-hub:test-long' --skip-nx-cache -- blob_put_get_head_round_trip
bun nx run 'os-hub:test-long' --skip-nx-cache -- directory_ws_isolates_private_realtime_activity_and_global_identity
```

The hub test script compiles with `--all-features`. Final results were:

- status lowering: 1 executed, 1 passed, 45 skipped, 0.043 seconds;
- blob PUT/GET/HEAD plus exact missing GET: 1 executed, 1 passed, 45 skipped, 0.568 seconds;
- strict private-directory WebSocket: 1 executed, 1 passed, 45 skipped, 0.880 seconds.

An early exploratory invocation placed `--exact` incorrectly and selected 0 tests; it was discarded and is not counted as evidence. Separate fundamental/quick-profile attempts were killed by their 15/30-second wall budgets while the shared machine was compiling or waiting on package-cache locks; the final explicit-count runs above completed green under suitable profiles.

## Hygiene And Residual

`git diff --check` and the scoped temporary-debug-marker search were clean. The 6.4 GB ticket-local `sol-db-fault-taxonomy` target was moved recoverably to `/Users/ueli/.Trash/semio-sol-db-fault-taxonomy-20260903`; no other agent's generated directory was touched.

The optional medium response-copy interactivity packet remains open. `db_io_pages_into_http_bytes` still needs bounded page/chunk yields and cancellation-aware deterministic close. It was not allowed to delay the primary taxonomy, blob, and strict WebSocket result.
