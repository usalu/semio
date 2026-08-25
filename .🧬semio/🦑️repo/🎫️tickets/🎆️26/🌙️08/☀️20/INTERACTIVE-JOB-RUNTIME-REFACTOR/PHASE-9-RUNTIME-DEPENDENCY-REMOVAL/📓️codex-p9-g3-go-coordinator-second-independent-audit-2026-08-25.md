# Phase 9 G3 Go Coordinator Second Independent Audit — 2026-08-25

## Verdict

**RED — G3 is not accepted.**

The prior RED findings have substantial real remediation: the current Windows file uses `MoveFileExW` with `MOVEFILE_WRITE_THROUGH`, opens the parent directory through the Windows syscall implementation, and calls `FlushFileBuffers`; the generic durability adapter wraps parent-flush failures in `ErrDurabilityUnsupported`. The command path also now persists before applying repository projections, updating the index cache, or notifying handlers. Linux and Windows compilation, the ordinary/race suites, and the hosted hostile suite pass.

This second audit nevertheless finds two unmet required laws and one failed mandatory gate. These are current-source findings, not historical drift or a test flake.

## Blocking Findings

### P0 — Projection Allocation Is Not Bounded By `MaxItems`

All five list projections allocate their result before taking a traversal step. For example, `🐹️repository.go:429` calls `projectionResultCapacity` before `projectionKeys`; the same pattern is at lines 477, 514, 540, and 567. `projectionResultCapacity` at lines 724–729 caps only by `MaxResults`, never by `MaxItems` or the remaining traversal budget.

Thus a query with `MaxItems: 1`, `MaxResults: 10000`, and a projection containing at least 10,000 entries allocates a 10,000-element result backing array before its second key traversal observes the item limit. This violates the required max-items/max-results/**allocation** law and permits a large allocation before per-step cancellation/progress. The present test at `🧪️g3_event_store_test.go:744–789` only tests matched bounds (`8/4`, `7/4`, `8/3`); it cannot expose this counterexample.

`projectClaimsByTicket` and `projectConflicts` also sort derived collections synchronously (`🐹️repository.go:626` and 659; keys at 720) without an item step, cancellation check, or progress tick inside the sort. The fold itself is stepped, but the requirement is every expensive projection operation, not merely map iteration.

### P0 — Durability Harness Does Not Cover Every Metadata Mutation Or Replacement Failure

The injected `durabilityHarness` has only a `failSync` fault (`🧪️g3_event_store_test.go:66–105`). Its `Rename` always delegates to the host implementation, has no fault point, and records an operation only after a successful rename. The test at lines 369–416 therefore establishes an operation sequence and a parent-sync failure only; it never injects a live replacement (`.next` → log) failure and proves rollback/recovery for that failure.

It also models only successful `Rename` and `Remove`. File creation via `os.OpenFile` in `🐹️event_store.go:374–408`, direct failed-write cleanup at line 386, and lock-file create/remove/heartbeat mutations are outside the injected operation interface. Consequently, the claim that *every metadata mutation* is parent-synced is not executable evidence. The current host harness delegates to `nativeStoreDurability`, so on this Darwin runner it is not a Windows operation harness either. This fails the specified faithful injected filesystem/operation-harness gate.

### P0 — A Production Persistence Error Is Explicitly Discarded

After the durable replacement point, `🐹️event_store.go:341` executes `_ = store.cleanupRecoveryArtifacts()`. Cleanup uses `removeDurably`, whose parent-sync error is real persistence/durability failure (`🐹️event_store.go:419–428`), but it is not reported synchronously. The implementation deliberately preserves a successful committed result, which is sensible for retry safety, but it does not meet the stated unconditional rule that all production persistence errors propagate synchronously. The suite only faults a pre-commit parent sync (`failSync: 5`); it has no post-commit cleanup fault assertion.

The staged log remains recoverable if cleanup fails, so this is not evidence of lost committed data. It is still an observable, unreported durability-maintenance failure and prevents acceptance under the requested rule.

### Gate Blocker — Root Literal Dependency Verifier Is Currently Unexecutable

The exact required command failed from the actual repo root:

```text
bun ./📜️script.ts verify dependencies list go --literal-external --format json
error: Cannot find module '../../🔨️modules/🧮️math/🕸️graph/🗣️dsl/🫀️core/📦️packages/🟦️typescript/🟦️typescript/📦️index.ts'
```

This does not establish an external Go dependency: `GOWORK=off go list -m all` and the full test dependency closure resolve only `github.com/usalu/semio/repo/server` and the genuine local replacement `github.com/usalu/semio/repo/go`. But the mandated literal-verifier gate did not pass in the live worktree and cannot be reported GREEN.

## Revalidated Positive Properties

- `🐹️durability_windows.go` uses `MoveFileExW` with both replacement and write-through flags and calls `syscall.FlushFileBuffers`; there is no `runtime.GOOS` success branch. `nativeStoreDurability.SyncParent` wraps any parent-sync error as `ErrDurabilityUnsupported`.
- The Windows implementation cross-compiles. Go's current Windows `syscall.Open` adds `FILE_FLAG_BACKUP_SEMANTICS` for read-only directory opens, so the source has a valid directory-handle path rather than a known compile-only directory-open impossibility.
- Native Windows execution was unavailable on this Darwin host. Cross-compilation plus source inspection is adequate evidence that the Windows source is wired and type-correct, but not adequate evidence of native `MoveFileExW`/directory-`FlushFileBuffers` runtime behavior; the incomplete generic harness cannot close that gap.
- `EventRepository.execute` calls durable `Append` before applying its projection (`🐹️repository.go:330–342`). `updateIndexForFile` persists scopes, then synchronously publishes, then updates the cache (`🐹️component.go:1129–1139`). The G3 pre-commit fault test preserves prior bytes and projection, and the cache/handler fake-repository test passes.
- The active event store is append-only JSONL with deterministic replay, expected-sequence conflicts, idempotent command IDs, bounded record/log limits, recovery artifacts, and no SQL/SQLite/CRDT production store. The module has no `go.sum` and no third-party module identity.

## Fresh Gate Matrix

All Go commands ran in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator` with `GOWORK=off`.

| Gate | Result |
| --- | --- |
| `go test -count=1 ./...` | PASS |
| `go test -count=3 ./...` | PASS |
| `go test -race -count=1 ./...` | PASS |
| `go test -list '^TestG3' ./...` | PASS — 20 groups |
| Focused durability/projection/cache/hostile G3 run | PASS |
| `go vet ./...` | PASS |
| `go mod tidy -diff` | PASS — no output |
| `go mod verify` | PASS — `all modules verified` |
| Linux compile (`GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...`) | PASS |
| Windows compile (`GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...`) | PASS — compile only |
| `gofmt -d` over all live Go files | PASS — empty |
| `git diff --check` over G3 module | PASS — empty |
| `go list -m all` / dependency closure | PASS — only server + local repo/go |
| root literal dependency verifier | **FAIL** — missing imported script module |

## Required Remediation

1. Cap every query allocation by both result and remaining item budgets; make sort/derived-map work bounded, cancellable, and progress-reporting. Add independent exact, max+1, small-`MaxItems`/large-`MaxResults`, cancellation-during-sort/fold, and last-valid laws for every query family.
2. Expand the durability abstraction into a faithful operation model that records create/write/sync/rename/remove/lock metadata operations, injects each operation failure (especially live replacement), and proves exact-byte rollback plus reopen recovery. Retain the Windows source assertion but add native Windows execution when a Windows runner exists.
3. Decide the explicit post-commit cleanup contract. If all persistence errors must propagate, return/report cleanup failure synchronously without ever falsely reporting an uncommitted append; otherwise narrow the acceptance rule and surface pending cleanup/recovery state explicitly.
4. Repair the root dependency-verifier import path and rerun the exact literal command to an empty result.

No source, Cargo, Nx, Wasm, browser, or product configuration file was edited by this audit.
