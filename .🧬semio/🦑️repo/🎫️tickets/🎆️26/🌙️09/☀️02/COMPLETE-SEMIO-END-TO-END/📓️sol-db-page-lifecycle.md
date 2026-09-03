# DB Page Lifecycle Repair

## Outcome

The retained DB output lifecycle now has one explicit publication invariant:

- A blocking output writer that returns `Yield` leaves every retained output page in the current `Executing` phase. The generic driver can therefore perform its exact `Executing -> Queued -> Executing` turn transition.
- After all retained pages have been validated, the writer transitions the complete retained set and transfers ownership in the same call: task output uses `Executing -> TerminalResult`; direct writer output uses `CheckedOutWriter -> CheckedOutInput`.
- Generation, operation, and phase disagreement remains an error. Generation disagreement is the exact `StaleGeneration` variant; operation and phase errors now name the expected and actual identity instead of printing equal generations as a false stale-reuse diagnosis.

Cancellation and abandonment during a blocking execution turn also retain one final driver resubmission. That final turn observes cancellation, transitions queued pages to `TerminalResult`, publishes the cancelled terminal, and makes the owner eligible for exact close/credit retirement. Normal yields were already resubmitted; this closes the race where cancellation arrived while the previous turn was executing.

No Hub, SQLite-driver, replication, artifact-bootstrap, actor-return, plugin, WGPU, manifest, goal, ticket-lifecycle, or AGENTS source was edited in this packet. The shared `storage/🦀️.rs` already contained substantial concurrent DB work; it was preserved.

## Test-Driven Evidence

### Retained output publication

The initial focused regression used 4,097 deterministic bytes and two retained pages. Before the repair it ran one test and failed because page zero was already `TerminalResult` while `seal_retained_step` had returned `Yield`; the simulated generic `Executing -> Queued` transition could not proceed. An earlier `--exact` invocation selected zero tests and is deliberately not counted as evidence.

Current-tree command:

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/db-page-lifecycle-target" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db \
  db_io_executing_output_seal_keeps_every_page_executing_until_atomic_publication -- --nocapture
```

Result: `1 passed; 0 failed; 579 filtered out`. The test forces more than one sealing yield, checks that all pages remain in `Executing` at each yield, performs exact queue/resume transitions, verifies the complete 4,097-byte result, checks terminal-result publication, closes it, and compares the operation ledger with its baseline.

### Success, cancellation, abandonment, and retirement

Current-tree command:

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/db-page-lifecycle-target" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db \
  db_io_output_task_yield_cancel_abandon_and_close_retire_exactly_once -- --nocapture
```

Result: `1 passed; 0 failed; 579 filtered out`. A scenario-gated blocking executor checks that every resumed output turn enters with all pages in `Executing`. The successful operation spans two pages and multiple writer-seal yields, returns the exact 4,097 bytes, and explicitly closes the result. Cancellation and abandonment are injected only after the writer has filled both pages and yielded again; each reaches `Cancelled`, owns only `TerminalResult` pages, drains task/backend close, and returns the ledger to its exact starting witness.

### Exact identity rejection

Current-tree command:

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/db-page-lifecycle-target" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db \
  db_io_page_identity_rejects_generation_operation_and_phase_mismatches_exactly -- --nocapture
```

Result: `1 passed; 0 failed; 579 filtered out`. The test injects and restores generation and operation disagreement, then requests a transition from the wrong phase. It observes the exact stale-generation pair, distinct operation-mismatch detail, distinct expected/actual phase detail, and exact ledger restoration after close.

### Language-neutral fixture and independent hash oracle

`storage/🧪️fixtures/🧬️page-lifecycle/🔣️.json` defines language-neutral byte-pattern parameters and boundary lengths `1`, `4096`, and `4097`. The SQLite law uses those bytes as the independent roundtrip oracle and compares the returned storage hash with the repository-owned BLAKE3 implementation.

The BLAKE3 implementation was separately checked against the third-party `blake3` crate, which remains a test-only dependency of the hash package and covers the same three boundary lengths among its vectors:

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/db-page-lifecycle-target" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-hash \
  hash_bytes_agrees_with_the_blake3_oracle_across_lengths -- --nocapture
```

Result: `1 passed; 0 failed; 9 filtered out`.

The SQLite law compiled but is intentionally left red for the separately assigned SQLite production packet:

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/db-page-lifecycle-target" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db --all-features \
  sqlite_payload_roundtrip_obeys_the_neutral_page_lifecycle_fixture -- --nocapture
```

Result: `0 passed; 1 failed; 629 filtered out`. Exact failure after successful PUT/hash/contains/len: `Internal("io error: Invalid column type Text at index: 0, name: substr(bytes, ?2, ?3)")`. An earlier leading-zero fixture also observed persisted length `0` instead of `1`. Source attribution is the SQLite driver's `bytes = bytes || ?2` stage append, which coerces the staged BLOB to TEXT. The coordinator assigned that production repair separately; this packet did not edit `storage/🪶️sqlite/🦀️.rs`.

## Compile Gate

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/db-page-lifecycle-target" \
bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db --all-features --message-format short
```

Result: exit `0`; `semio-framework-os-kernel-db` completed the dev profile with zero errors. Cargo reported 417 existing DB/dependency warnings; no warning was suppressed.

## Hub Runtime Gates

Every command used `RUST_MIN_STACK=16777216`, `CARGO_BUILD_JOBS=2`, the isolated ticket target above, `bun nx run os-hub:test-quick --skip-nx-cache -- <filter> -- --nocapture`, and current shared sources.

| Filter | Result | Exact boundary reached |
| --- | --- | --- |
| `blob_put_get_head_round_trip` | `0/1`, 23 filtered, nextest 39 skipped | PUT hash/size, existing GET byte equality, existing HEAD 200, and missing HEAD 404 all completed. Only the final missing GET failed: expected 404, got 500 at `📦️bin.rs:2306`. The handler is called directly and returns only `Err(StatusCode)`, so there is no response body to capture. |
| `share_token_is_scoped_read_only_and_revocable` | `1/1`, 23 filtered, nextest 39 skipped | Full law passed. Valid scoped share receives `Welcome`/`Session`; tokenless, cross-space, and revoked cases retain exact `unauthorized` assertions. |
| `document_open_rejects_missing_or_conflicting_descriptor_before_db_creation` | `1/1`, 23 filtered, nextest 39 skipped | Full law passed. Valid announced document receives `Welcome`; missing remains `document-not-announced`, conflict remains `schema-hash-mismatch`, and the negative cases preserve their no-DB-creation assertions. |
| `directory_ws_isolates_private_realtime_activity_and_global_identity` | `0/1`, 23 filtered, nextest 39 skipped | The first private document completed `Welcome`, `Session`, presence, and the private-directory non-leak check. The second valid `mine_doc` connection then received a non-`Welcome` first frame at `📦️bin.rs:2588`. The test consumes that decoded value inside `matches!` without rendering it, and the server emits no matching log, so its exact code/message is not exposed by existing evidence. |

These results establish that the original multi-page DB output fault no longer blocks the Hub blob payload roundtrip, share Welcome, or descriptor Welcome paths. They do not claim the final missing-blob mapping, second private connection, or separately assigned SQLite BLOB append issue is repaired.

## Integrity And Cleanup

Scoped `git diff --check` over `storage/🦀️.rs` and the lifecycle fixture produced no output. No temporary debug logging is present. The packet's 5.9 GiB isolated target was removed from ticket-generated storage by moving it recoverably to `/Users/ueli/.Trash/semio-db-page-lifecycle-target-20260903`; other agents' generated material is untouched.
