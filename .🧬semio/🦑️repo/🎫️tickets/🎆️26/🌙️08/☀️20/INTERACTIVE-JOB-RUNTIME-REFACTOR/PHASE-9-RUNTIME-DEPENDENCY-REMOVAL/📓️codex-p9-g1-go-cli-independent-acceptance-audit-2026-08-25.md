# Phase 9 G1 Go CLI Independent Acceptance Audit

Date: 2026-08-25  
Auditor: Codex independent G1 audit  
Verdict: **RED — do not accept G1 yet**

## Boundary And Dependency Census

The actual owned module is `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli`, not the packet's nominal nested `📦️packages/🐹️go` path. Its module path is `github.com/usalu/semio/repo/client`.

`go.mod` has exactly one requirement:

```text
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library
```

That is a genuine local first-party module. There are no third-party `require` or `replace` rows and `go.sum` is absent. `GOWORK=off go list -m all` and the full test dependency census respectively returned only:

```text
github.com/usalu/semio/repo/client
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library

github.com/usalu/semio/repo/client
github.com/usalu/semio/repo/go
```

The owned Go files import only standard library packages, the CLI's own internal packages, and that first-party library. The direct scan found no old dependency import or declaration. New owned sources contain no upstream copyright, licence, SPDX, source-attribution, or former-provider URL marker. The implementations are partial local implementations rather than visibly vendored copies. The client public API contains no third-party type; all replacement types are in `internal/` packages.

There are stale comments/UI strings referring to Bleve and Cobra in `🐹️component.go` (lines 1343, 4368, 6982, 9327). They are not dependency edges, but they should be cleaned before calling this replacement complete.

## Executed Gates

All commands ran from the actual module, with `GOWORK=off` where stated.

| Gate | Result |
| --- | --- |
| Canonical hostile packet: `go test -count=1 -run '^TestG1' .` | PASS |
| Existing preserved CLI/MCP/GraphQL/export subset | PASS |
| `GOWORK=off go test -run '^$' ./...` | PASS |
| `GOWORK=off go vet ./...` | PASS |
| `GOWORK=off go mod tidy -diff` | PASS (empty output/diff) |
| `GOWORK=off go mod verify` | PASS |
| Isolated module/dependency census | PASS (two first-party modules only) |
| `GOWORK=off GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| `GOWORK=off GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| `git diff --check -- <owned module>` | PASS |

The G1 fixture exercises unknown flags, recursive globbing, a bad template, corrupt index/log, duplicate and interrupted append, cancellation, maximum and maximum-plus-one event payload, deterministic replay, and YAML decoding. Those checks are real but insufficient to establish the required live-operation semantics below.

## Acceptance Blockers

### 1. Export Is Destructive, Not Append-Only

`🐹️event_export.go:84` executes `os.Remove(outputPath)` immediately before `eventstore.Store.Append`. A second export therefore deletes the committed event stream, resets sequences to one, and bypasses the store's duplicate-ID detection. This contradicts the required append/replay event-log replacement and event-sourcing/CQRS semantics. It is not an atomic replacement either: a cancellation or later write error after removal has already lost the prior committed log.

The behavior is directly evident in the owned export flow:

```go
if err := os.Remove(outputPath); err != nil && !os.IsNotExist(err) { ... }
if _, err := (eventstore.Store{Path: outputPath}).Append(ctx, inputs, progress); err != nil { ... }
```

The current test only runs a first export. It does not export again and assert retained history, monotonic sequence continuation, duplicate protection, or cancellation preserving the previously committed log.

### 2. Expensive Search Reindex Ignores Cancellation, Progress, And Errors

`ensureCacheIndexed(ctx, root)` in `🐹️component.go:7246–7358` never reads `ctx`, provides no progress callback, and uses unbounded recursive `indexNodes`. Its lock acquisition can also sleep for up to 11.8 seconds without checking cancellation. All `idx.Delete` and `idx.Index` errors are discarded (lines 7295, 7310, and 7346). This is an owned replacement path for recursive search/indexing, so it fails the required cancellation/progress/bounded-operation gate even though `search.SearchContext` itself observes context.

### 3. Replacement Is Still A Compatibility-Shaped Internal Surface

This is not a separate hard blocker because the old package types are not exported, but it needs deliberate review: `internal/command`, `internal/graphql`, `internal/search`, and `internal/mcp` deliberately reproduce substantial external-provider-shaped APIs and naming (`Command`/`FlagSet`, `NewObject`/`ResolveParams`, `Index`/`NewMatchQuery`, MCP constructors). There is no external type leakage, but the implementation has not been reorganized around an owned domain schema. The maintained source still labels search as Bleve and a command as Cobra. This weakens the claimed “no legacy fallback/compatibility shim” conclusion.

## Full-Suite Classification

`GOWORK=off go test -short -count=1 -timeout 90s ./...` completed RED in 2.932 seconds. Its visible failures include MCP bootstrap JSON expected under repository-root assets, obsolete taxonomy/path fixtures, emoji variation assumptions, absent legacy fixtures and PostgreSQL schema paths, removed autofix behavior, and repository configuration/logging drift. No `TestG1*` failure appeared.

Those failures are outside the four tracked G1 module files and are consistent with existing whole-repository fixture drift, so they are not evidence of a new G1 test regression. They do not clear the two owned blockers above.

## Scope Check

The owned tracked diff is limited to the CLI module's `go.mod`, removed `go.sum`, `🐹️component.go`, and `🧪️component_test.go`; untracked owned additions are the internal replacements, `🐹️event_export.go`, `🧪️g1_contract_test.go`, and its fixture. `go.work` itself is unchanged. The shared worktree does contain concurrent unrelated changes in other modules, scripts, ticket areas, and `go.work.sum`; they are not attributable to G1 from this uncommitted shared state and must remain excluded from the G1 packet.

## Required Resolution Before Green

1. Remove destructive export reset. Model snapshot/export events with distinct event identities and append them atomically; add repeat-export, interrupted-export, retained-history, and deterministic replay tests.
2. Thread `ctx` through cache lock waiting, traversal, index mutation, and persistence; emit bounded progress; stop/clean up on cancellation; propagate index errors. Add a cancelled reindex test with no committed partial index and a bounded-progress test.
3. Remove former-provider terminology and either justify the internal compatibility-shaped surface against the owned schema or reshape it before acceptance.

