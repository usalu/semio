# Phase 9 G3 Go Coordinator Third Independent Audit — 2026-08-25

## Verdict

**GREEN — accept G3.** The two earlier RED audits are remediated in the live coordinator-root module. This third audit independently reproduced the stated hostile laws and reran every prescribed gate. No P0 blocker remains.

The live boundary is `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator`; the old nested package path is not the Go module. This audit made no source, Cargo, Nx, Wasm, browser, or product-configuration change.

## Prior RED Remediation Revalidated

### Projection cursor, allocation, cancellation, and last-valid laws

All eight projection families use the retained `projectionTraversal`: tickets, ticket lookup, scopes, claims, warnings, breachs, conflicts, and contributors. `projectionResultCapacity` is now the exact bounded minimum of source size, `MaxResults`, and remaining `MaxItems`. `projectionKeys` also limits its backing allocation to remaining items. The owned bottom-up merge sort reserves its temporary buffer against that same cursor and checks cancellation/progress for every merge and copy step; no projection uses a synchronous library sort.

`TestG3EveryProjectionFamilyRetainsOneBudgetedCursor` executes every family through exact work/result limits, max-plus-one failure, `MaxItems: 1` with `MaxResults: 1_000_000`, cancellation during lookup/sort work, and a subsequent unchanged last-valid result. `TestG3ProjectionBoundsProgressAndDuringCancellationPreserveLastValid` additionally verifies the ticket traversal and direct tiny-work/huge-result allocation law. The current source and the complete 25-group test listing confirm that the second-audit allocation and unstepped-sort P0s are closed.

### Faithful injected filesystem and all durability phases

The non-host `memoryStoreOperations` implements directory preparation; create/open/read/write; file stat/sync/close/truncate; rename replacement; remove; parent sync; and lock heartbeat. It keeps in-memory bytes and metadata only, records every operation before a configurable injected fault, and never delegates its filesystem behavior to the host.

`TestG3FaithfulFilesystemFaultsEveryPrecommitOperation` faults the twenty pre-commit candidates individually: lock create/write/file-sync/close; `.next` create/write/file-sync/close/parent-sync; stage-next create/write/file-sync/close/parent-sync; stage activation and parent sync; prior backup rename and parent sync; and the exact `.next` to live replacement and live parent sync. Every candidate proves exact prior live bytes, clear-fault reopen, last-valid event, and no recovery artifacts. The separate lock-removal/heartbeat law makes both phases observable.

The post-commit fault laws inject cleanup remove and cleanup parent-sync failures. Each returns `*PendingCleanupError`, with `AppendResult{Committed: true, PendingCleanup: true}`; recovery reports `StoreRecoveryStatus{Recovered: true, Action: "committed-cleanup"}` and retains the committed event. The repository applies that committed event before propagating the maintenance failure and replays it after `Reopen`.

### Windows durability source

`🐹️durability_windows.go` uses `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH`, opens the parent directory through the standard-library Windows syscall implementation, and calls `syscall.FlushFileBuffers`. `nativeStoreOperations.SyncParent` converts any parent-flush failure to the explicit owned `ErrDurabilityUnsupported`; there is no `runtime.GOOS` success shortcut. Windows compilation passes. Native Windows runtime execution was not available on this Darwin worker, so this is source and cross-compilation validation, as requested, rather than a claimed native-runtime run.

## Fresh Gates

All module commands ran in the live coordinator root with `GOWORK=off`.

| Gate | Result |
| --- | --- |
| `go test -count=1 ./...` | PASS |
| `go test -count=3 ./...` | PASS |
| `go test -race -count=1 ./...` | PASS |
| `go test -list '^TestG3' ./...` | PASS — exactly 25 top-level G3 groups |
| `go vet ./...` | PASS |
| `go mod tidy -diff` | PASS — empty output/diff |
| `go mod verify` | PASS — `all modules verified` |
| `GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| `GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS — compile-only on this host |
| `gofmt -d *.go` | PASS — empty output |
| `git diff --check -- <G3 module>` | PASS |
| `GOWORK=off go list -m all` | PASS — server plus genuine local `repo/go` replacement only |
| root `bun ./📜️script.ts verify dependencies list go --literal-external --format json` | PASS — exact output `[]` |

`go.sum` remains absent. The live module has no event/stage/next/backup/lock/database artifact after the gates. Production imports are the standard library plus the genuine local `github.com/usalu/semio/repo/go` module; no third-party module identity is present.

## Acceptance Scope And Remaining Constraint

G3 is GREEN against the stated coordinator contract. A Windows runner would add useful native execution evidence for `MoveFileExW` and directory `FlushFileBuffers`, but its absence here is not a code or gate failure: the requested source check and Windows compilation both pass, and unsupported durability is explicit rather than falsely successful.
