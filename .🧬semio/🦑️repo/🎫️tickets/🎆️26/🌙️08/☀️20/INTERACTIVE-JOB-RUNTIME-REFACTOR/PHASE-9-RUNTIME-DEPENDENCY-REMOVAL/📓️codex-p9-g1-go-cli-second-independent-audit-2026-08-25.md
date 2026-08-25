# Phase 9 G1 Go CLI Second Independent Audit

Date: 2026-08-25  
Auditor: Codex  
Verdict: **GREEN for the owned G1 packet.** The repository-wide short suite remains baseline **RED**, with no observed G1 regression.

## Scope Read

Audited live module:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli`

Read the Phase 9 plan/contract, Z0 dependency material, first independent RED audit, current owned diff, canonical G1 fixture, remediation source, and tests. The nominal nested Go package is not the live CLI Go module.

No source, `go.work`, Cargo, Nx, Wasm, browser, or shared configuration file was edited by this audit.

## P0 Reproduction

### Event Log Export

The former `os.Remove(outputPath)` path is absent from `🐹️event_export.go`. A snapshot is first fully canonicalized, hashed, and assigned snapshot-scoped IDs, then passed to `eventstore.Store.Append`. The store validates the existing replay before construction, detects duplicate IDs before staging, stages a durable prior-byte boundary, appends with `O_APPEND`, syncs, and only then retires its stage record. It does not pre-delete, pre-truncate, or rewrite an existing valid log.

Executed hostile preservation cases passed:

- cancellation at `encoded`, `staged`, `appended`, and `synced` left the previously committed bytes identical and left no stage record;
- replay cancellation left log bytes unchanged and a later replay succeeded;
- failed over-maximum export and interrupted export retained the existing bytes;
- repeated deterministic snapshot export returned explicit `eventstore.ErrDuplicate` and changed no bytes;
- changed snapshot appended sequence 2 after sequence 1 and retained history;
- staged crash recovery retained a complete synced append, or restored only the recorded valid prefix for stage-only and partial append cases.

### Search Reindex

`ensureCacheIndexed` now uses `computeCompositeFingerprintContext`, a cancellable bounded lock wait, iterative bounded collection, `IndexContext`, `CloseContext`, staged validation, and swap commit. `searchMonorepoTreeWithCache` returns reindex/query/open errors instead of silently using an in-memory result. The matching-fingerprint corrupt current index path returns wrapped `search.ErrCorrupt`.

Executed cases passed:

- cancellation during collection, indexing, and persistence preserved the last-valid `events.jsonl` and `meta.json`, with staged output removed;
- lock-wait cancellation returned `context.Canceled` and retained the valid index;
- maximum-plus-one document rejection returned `search.ErrTooLarge` and retained the valid index;
- corrupt current index returned `search.ErrCorrupt` rather than being swallowed;
- search cancellation, bounded query flow, and index persistence/replay passed.

## Dependency And Source Census

| Check | Result |
| --- | --- |
| `go.mod` requirements | One first-party local module only: `github.com/usalu/semio/repo/go v0.0.0` |
| `replace` | `../../📚️library`, a local first-party path |
| Third-party manifest rows | 0 |
| `go.sum` | Absent; naturally unused |
| `vendor/` | Absent |
| Module census | `github.com/usalu/semio/repo/client`, `github.com/usalu/semio/repo/go` only |
| Test dependency census | Same two first-party modules only |
| Non-standard owned/test import scan | 0 third-party paths |
| Removed-root/copied-vendor marker scan | 0 removed dependency roots or external-copy markers; one unrelated user-facing SPDX policy string only |
| Scoped whitespace diff | `git diff --check` passed |

The implementations are local first-party `internal/` packages. No public API needs an external package type.

## Executed Gates

| Gate | Result |
| --- | --- |
| Expanded G1 hostile/preservation plus command, glob, template, GraphQL, MCP, config, export, and cache subset | PASS (client 4.701s; eventstore 0.718s) |
| P0-focused event/reindex preservation subset | PASS (client 3.454s; eventstore 0.468s) |
| `GOWORK=off go test -run '^$' ./...` | PASS |
| `GOWORK=off go vet ./...` | PASS |
| `GOWORK=off go mod tidy -diff` | PASS, no diff |
| `GOWORK=off go mod verify` | PASS |
| `GOWORK=off go list -m all` | PASS, two first-party modules only |
| `GOWORK=off go list -deps -test ...` | PASS, two first-party modules only |
| `GOWORK=off GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| `GOWORK=off GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| Scoped `git diff --check` | PASS |

## Full Short Baseline Classification

`GOWORK=off go test -short -count=1 -timeout 90s ./...` completed **RED** in 10.118s. It reproduces repository-root fixture/configuration drift: stale MCP bootstrap arguments, outdated taxonomy and emoji expectations, removed autofix expectations, absent legacy paths/PostgreSQL fixture, and logging/config assumptions. The internal event store passed and no `TestG1*`, P0-preservation, command/template/glob/GraphQL/MCP G1, or reindex test failed.

This is the same baseline class recorded before remediation; it is outside the owned G1 dependency-removal scope and not a blocker for this packet.

## Blockers

None for the owned G1 acceptance criteria. The unrelated full-short baseline remains a repository-level blocker to a whole-module green claim, not to G1 packet acceptance.
