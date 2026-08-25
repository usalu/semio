# Phase 9 G3 Go Coordinator Fresh Independent Audit — 2026-08-25

## Verdict

**RED — do not accept G3 as cross-platform production-ready.**

The actual coordinator-root Go module has achieved the zero-third-party dependency boundary and its hosted G3 hostile suite passes. The initial G3 report's stated live path is correct: the old nested `🎛️coordinator/📦️packages/🐹️go` packet path has drifted; the live module is `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator`.

However, two concrete production failures and one interaction gap contradict the required durable, local-first, cross-platform coordinator contract:

1. `🐹️event_store.go:825-835` explicitly returns success without a directory sync on Windows. A staged commit therefore reports its durable commit point after `os.Rename` without durably ordering the rename on Windows. The Linux/macOS `directory.Sync()` path does not establish the required Windows guarantee. The Windows gate only cross-compiles; no Windows bytes/recovery execution occurred.
2. Live persistence calls discard errors while proceeding with non-durable state: `🐹️component.go:1094` updates the indexing cache after ignored `recordScopes` failure; lines 1315, 1391, and 1405 also discard checkpoint/contributor event failures. On a shortage or cancellation, the server can report/use ephemeral state that disappears on replay/restart, so the integration is not fully event-sourced and callers lack a recovery signal.
3. Projection queries have no per-item cancellation, progress, or result bound. `🐹️repository.go:394-540` checks context only before taking the lock, then copies/sorts unbounded projection maps. The single cancellation test cancels before `projectTickets`, not during a large query. This does not meet cancellation/progress/bounds for expensive coordinator operations.

These are source-observed conditions, not test-flake speculation. No source was edited for this audit.

## Inputs Independently Read

- root and coordinator `AGENTS.md`;
- `go.work`, live root `go.mod`, live Go production/tests/fixtures;
- Phase 9 Z0 dependency truth;
- the latest G1 and G2 independent GREEN audit packets; and
- the G3 contract report, treated as a claim and revalidated rather than trusted.

## Boundary And Dependency Census

The actual live coordinator module contains:

```text
github.com/usalu/semio/repo/server
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library
```

`GOWORK=off go list -deps -test` resolves only those two module identities. Production import census is Go standard library plus the genuine local `github.com/usalu/semio/repo/go`; `go.sum` is absent. `modernc.org/sqlite`, `database/sql`, SQLite driver imports, SQL calls, external `require` rows, and a copied SQLite/CRUD compatibility store were not found in production source. The only historical prohibited-token occurrences are in the hostile source test's string list.

The root verifier independently returned exactly `[]` for:

```text
bun ./📜️script.ts verify dependencies list go --literal-external --format json
```

No coordinator-root `*.events`, `*.stage`, `*.stage.next`, `*.next`, `*.backup`, `*.lock`, `*.db`, or `go.sum` artifact existed after the gates.

## Verified Positive Properties

The owned store encodes the ordered schema envelope (`stream`, `sequence`, `id`, `generation`, `type`, `payload`, `checksum`) in deterministic JSONL/LF and checks SHA-256, sequence, ID uniqueness, canonical payload bytes, payload depth, per-append/replay/log bounds, and partial-tail truncation to the last valid newline.

Its tested local-host behavior covers empty/append/replay/golden bytes, 24 writers and expected-sequence conflict, idempotent canonical duplicate and mismatch rejection, injected interruption after each named phase, corrupt/checksum/sequence/full-record failures, partial tail, unavailable storage, lock deadline, close/reopen, cancellation before/during/after commit and replay, exact/max-plus-one limits, deterministic replayed projections, and hostile source assertions. The staged `.next`/`.stage`/`.backup` recovery source is present and the Linux/macOS directory-sync path is explicit.

## Executed Gates

All commands below executed against the actual coordinator root, with `GOWORK=off` where shown:

| Gate | Result |
| --- | --- |
| `go test -count=1 ./...` | PASS |
| `go test -count=3 ./...` | PASS |
| `go test -race -count=1 ./...` | PASS |
| `go test -list '^TestG3' ./...` | PASS — 15 G3 groups listed |
| focused all-G3 hostile run | PASS |
| `go vet ./...` | PASS |
| `go mod tidy -diff` | PASS — no diff |
| `go mod verify` | PASS — all modules verified |
| Linux cross compile (`GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...`) | PASS |
| Windows cross compile (`GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...`) | PASS, compile only |
| `gofmt -d *.go` | PASS — empty |
| `git diff --check -- .` | PASS |
| exact Go literal-external verifier | PASS — `[]` |

## Required Remediation Before GREEN

1. Implement and execute a Windows-safe durable rename/commit protocol, or explicitly use a cross-platform filesystem primitive that establishes the claimed commit guarantee; add real Windows recovery/byte tests.
2. Propagate or durably queue every coordinator command failure before mutating caches/observable state, especially scope indexing, checkpoint, and contributor paths; exercise interruption/unavailable recovery through these live handlers.
3. Add bounded, cancellable, progress-reporting projection traversal/query behavior and tests that cancel during a large projection, preserving the previous valid projection/result.

The positive zero-external dependency result may be accepted independently; the full G3 coordinator acceptance cannot.
