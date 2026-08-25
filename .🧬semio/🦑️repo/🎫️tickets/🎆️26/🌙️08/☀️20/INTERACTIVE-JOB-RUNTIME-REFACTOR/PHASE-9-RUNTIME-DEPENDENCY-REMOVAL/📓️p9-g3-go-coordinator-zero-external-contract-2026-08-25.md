# Phase 9 G3 Go Coordinator Zero-External Contract — 2026-08-25

## Verdict

**GREEN and audit-ready.** The live Go coordinator production/test closure now contains only the Go standard library and the genuine local `github.com/usalu/semio/repo/go` module. `modernc.org/sqlite`, `database/sql`, its entire transitive module closure, and `go.sum` are removed. Coordinator persistence is an owned schema-first, local-first, append-only event store with CQRS commands, deterministic replayed projections, optimistic concurrency, bounded/cancellable work, durable staged replacement, rollback, and recovery.

No `go.work`, G1/G2 file, shared script, launch configuration, other module, other ecosystem, baseline, or ticket metadata was changed.

The first independent G3 audit was RED on three source-observed P0 gaps: a Windows directory-sync success shortcut, discarded live persistence errors/cache ordering, and unbounded projection traversals. The second independent audit found three remaining proof gaps: result allocation and sort work did not share the item budget, the durability harness delegated unmodeled file/lock operations to the host, and committed cleanup failure was not observable. Every finding from both audits is now remediated in the live module and converted into hostile executable laws before the fresh gates below.

## Live Boundary Validation

The closeout packet map names a historical nested boundary at `🎛️coordinator/📦️packages/🐹️go`. That path does not contain the live module. The actual live boundary is the coordinator root:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/go.mod`;
- its former sibling `go.sum`;
- root Go production and test files; and
- language-neutral `🧫️fixtures/g3-event-schema.json` and `g3-event-log.jsonl`.

Before G3, this live `go.mod` directly required `modernc.org/sqlite v1.50.1` and listed nine indirect third-party requirements. `🐹️component.go` imported both `database/sql` and the SQLite driver, then implemented mutable SQL tables and SQL query/update methods. There were no coordinator Go tests. The boundary was clean of peer edits before implementation.

The accepted prerequisite state was also revalidated: the latest G1 audits are GREEN and the latest G2 audit is GREEN after its saturation remediation. Z0 correctly recognises genuine local Go modules/replacements.

## Owned Contract

### Language-Neutral Schema

Every persisted JSONL record uses exactly the ordered envelope fields:

`stream`, `sequence`, `id`, `generation`, `type`, `payload`, `checksum`.

The checksum is SHA-256 over NUL-separated nonempty scalar fields followed by canonical JSON payload bytes; the schema rejects NUL inside a scalar, so the boundaries are unambiguous. Payload objects are decoded with `json.Number` and re-encoded so key ordering and whitespace are deterministic. Records always use LF and the replay reader rejects any non-canonical full record. The language-neutral schema fixture fixes the ordered fields, scalar restriction, encoding, and checksum formula; the JSONL fixture fixes the exact bytes and checksum for a representative event.

### CQRS And Projection Ownership

`CoordinatorRepository` is the owned application boundary. Production callers submit semantic record/release/checkpoint commands; the repository serializes each command into one immutable coordinator-stream event. Ticket, scope, claim, warning, breach, conflict, and contributor reads use deterministic in-memory projections derived solely by replay. Reopening discards and reconstructs the projection. No snapshot is persisted; any future snapshot can only be a disposable derived acceleration.

Identical command retries derive the same event ID and are idempotent even after an acknowledged result is lost. A reused ID with different generation/type/payload is an explicit `ErrDuplicateEvent`. New events require the caller's exact expected stream sequence; stale writers receive `ErrSequenceConflict`. Repository writers retry only after bounded replay and never expose a third-party type.

The repository installs projection changes only after `Append` returns a durable commit. Every live coordinator persistence call now returns its failure to the HTTP caller or synchronous event publisher. Indexing persists scopes and the index event before taking the cache write lock, so an unavailable, cancelled, or interrupted append retains the old cache. Checkpoint and contributor handlers return repository failures, and contributor notifications occur only after the contributor event is durable. Event bus dispatch carries handler acknowledgement back to `Publish`; it no longer swallows a downstream command failure.

`AppendResult` explicitly distinguishes `Committed` and `PendingCleanup`. A `PendingCleanupError` states that the event is already durable while recovery artifacts still require maintenance. The repository applies those committed events to its projection before propagating the maintenance error, so it never presents a durable event as uncommitted or invites an unsafe non-idempotent interpretation. `RecoveryStatus` makes the subsequent committed-cleanup or prior-restoration action observable.

### Atomic Durability And Recovery

An append performs these durable transitions under context-cancellable in-process admission and a cross-process lock lease:

1. validate/canonicalize/bound/replay and encode the proposed events;
2. write the full prior prefix plus append to `.next`, sync its bytes, and durably sync its parent entry;
3. write and sync `.stage.next`, rename it to `.stage`, and durably sync the parent metadata;
4. rename the valid prior log to `.backup` when present and durably sync that metadata;
5. rename `.next` to the live log and durably sync the replacement metadata as the commit point; and
6. remove recovery artifacts, which are safe to recover again if cleanup is interrupted.

The owned `storeOperations` boundary covers directory preparation, open/create, read/write, file sync, close, truncate, rename, remove, parent sync, stat, lock create/remove, and lock heartbeat. Production contains no direct event-store or lock mutation outside this boundary. Unix opens and syncs the parent directory. Windows performs replacements through `MoveFileExW` with write-through semantics, opens the parent directory through the standard-library `syscall` package, and calls `FlushFileBuffers`; an access, filesystem, or platform failure returns `ErrDurabilityUnsupported` and cannot be reported as a commit. No runtime branch returns false success.

The faithful injected filesystem is a complete in-memory file/handle/metadata model and does not delegate to the Darwin host filesystem. It models exclusive creation, offsets, reads/writes, file sync, close, truncate, rename replacement, removal, metadata timestamps, and parent sync. Fault laws cover lock create/write/file-sync/close, every `.next` and stage create/write/file-sync/close/parent-sync transition, stage activation, prior backup, the exact `.next`-to-live replacement, live parent sync, cleanup remove/parent sync, lock removal, and heartbeat. Every pre-commit fault retains exact prior bytes and reopens without artifacts.

Normal cancellation, injected interruption, unsupported durability, or I/O failure before commit rolls back to the exact prior bytes. Crash recovery accepts a checksum-valid committed log or restores a checksum-valid backup; it fails closed if neither is valid. A trailing non-newline fragment is recoverable and truncated only to the exact last valid record. Malformed, checksum-invalid, sequence-invalid, or otherwise corrupt complete records return explicit errors without mutating the evidence. Once the durable metadata commit point is reached, late cancellation or a synthetic interruption cannot turn it into an uncommitted result. Cleanup failure is synchronously returned as the owned committed/pending-cleanup state; a retry observes `committed-cleanup`, retains the new event, and removes all artifacts.

The lock file has a heartbeat-backed stale lease for process failure recovery. Both file-lock waiting and same-process admission observe caller cancellation/deadlines, so a short shortage does not freeze a caller. `Close` makes repository availability explicit; `Reopen` recovers the store and rebuilds projections.

### Bounds And Interaction

Production defaults explicitly bound payload bytes, append bytes, events per append, replayed events, JSON depth, total log bytes, projection traversal work, and projection result count. Append and replay check cancellation during bounded loops. `ProjectionQuery` supplies owned max-items/max-results bounds and one retained `ProjectionProgress` cursor across each query.

Every result backing allocation is capped by the minimum of source size, `MaxResults`, and remaining `MaxItems`. Key materialization is capped by remaining work. Deterministic ordering uses the owned bottom-up merge strategy: it reserves its scratch allocation against the same cursor before allocating, then checks cancellation, work, and progress for every merge/copy step. Derived claim/conflict maps consume the same cursor. No query family calls a synchronous library sort. Exact, max-plus-one, tiny-item/huge-result, cancellation-during-sort/fold, and last-valid laws execute for tickets, ticket lookup, scopes, claims, warnings, breachs, conflicts, and contributors.

## Executable Coverage

`GOWORK=off go test -list '^TestG3' ./...` lists 25 top-level G3 groups covering:

- empty replay, multi-event append, progress, deterministic replay, and exact golden bytes;
- 24 concurrent writers, contiguous sequences, unique IDs, stale expected-sequence conflict, and byte preservation;
- identical/canonical duplicate retries, changed duplicate rejection, and mixed duplicate/new rejection;
- interruption after every pre-commit durable protocol phase with exact rollback, plus the law that a known committed result cannot report failure;
- cross-platform metadata operation ordering, a non-host-delegating in-memory filesystem, faults across every persistence/lock mutation phase, exact-byte rollback/reopen, committed cleanup retry, Windows write-through replacement/`FlushFileBuffers` source enforcement, and Linux/Windows compilation;
- recovery cleanup, exact prior/committed choice, last-valid preservation, partial tail truncation, malformed full record, checksum corruption, sequence corruption, invalid stage, and orphan backup;
- unavailable parent storage, bounded external-lock wait, bounded same-process wait, close/reopen, query not-found, and replayed repository semantics;
- cancellation before append, during staged append, after commit, during replay, and during every projection family's sort or fold;
- exact maximum and maximum-plus-one payload bytes, events per batch, JSON depth, total log bytes, replay event count, projection work/results/allocations, and tiny-item/huge-result mismatch;
- CQRS ticket/scope/claim/warning/contributor commands, deterministic sorted projections, conflicts, checkpoints, idempotent commands, close/reopen, and second-instance replay;
- failed-append laws proving durable bytes, repository projection, server index cache, and handler-visible error state retain the last-valid value; and
- hostile source checks rejecting SQLite/SQL, CRDT, legacy-store/fallback markers, discarded server persistence results, external module rows, Windows durability shortcuts, and a recreated `go.sum`.

## Fresh Gates

Every command below ran in the live coordinator module with `GOWORK=off` where applicable.

| Gate                                                                                  | Fresh result                  |
| ------------------------------------------------------------------------------------- | ----------------------------- |
| `go test -count=1 ./...`                                                              | PASS                          |
| `go test -count=3 ./...`                                                              | PASS                          |
| `go test -race -count=1 ./...`                                                        | PASS                          |
| `go vet ./...`                                                                        | PASS                          |
| `go mod tidy -diff`                                                                   | PASS — empty output/diff      |
| `go mod verify`                                                                       | PASS — `all modules verified` |
| Linux compile, `GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...`           | PASS                          |
| Windows compile, `GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...`       | PASS                          |
| `git diff --check` on the G3 module                                                   | PASS                          |
| root `bun ./📜️script.ts verify dependencies list go --literal-external --format json` | PASS — exact output `[]`      |

The final module census is exactly:

- `github.com/usalu/semio/repo/server`;
- `github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library`.

`go list -deps -test` resolves those same two module identities only. Production imports are standard-library paths plus `github.com/usalu/semio/repo/go`. There are zero third-party direct, indirect, test, build, or production module identities and no `go.sum`.

## Files

- updated `go.mod`;
- removed `go.sum`;
- updated `🐹️component.go` to consume `CoordinatorRepository` commands/projections;
- added `🐹️durability.go`, `🐹️durability_unix.go`, and `🐹️durability_windows.go`;
- added `🐹️event_store.go`;
- added `🐹️repository.go`;
- added `🧪️g3_event_store_test.go`;
- added `🧪️g3_filesystem_test.go`; and
- added `🧫️fixtures/g3-event-schema.json` and `g3-event-log.jsonl`.

No baseline failure was hidden. The only discovered plan discrepancy was the packet-map path drift documented above; all live-module gates executed successfully.
