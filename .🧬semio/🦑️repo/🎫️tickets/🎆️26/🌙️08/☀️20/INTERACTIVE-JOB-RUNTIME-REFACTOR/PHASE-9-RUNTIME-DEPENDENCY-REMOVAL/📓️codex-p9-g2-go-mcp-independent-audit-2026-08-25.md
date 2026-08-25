# Phase 9 G2 Go MCP Independent Audit

Date: 2026-08-25  
Auditor: Codex independent G2 audit  
Verdict: **RED — do not accept G2**

## Scope And Predecessor Evidence

This audit read the Phase 9 plan/owner packet, the Z0 independent audit, and the G1 independent acceptance audit, then inspected the live parent module and its actual worktree diff. The live Go module is:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp`

The nominal nested `📦️packages/🐹️go` path remains inert: both original files are still exactly zero bytes, byte-identical to `HEAD`, and contain no module or implementation. That part of the scope correction is true.

The G2 owner packet states that production cleanly delegates to an accepted G1 `client.RunMCP`. The independent G1 acceptance audit actually records **RED**, with unresolved destructive export and non-cancellable/unbounded reindex blockers. Therefore the claimed accepted-target premise is not established.

## Decisive Blocker: The New MCP Runtime Is Dead

The production executable does not use the new protocol implementation:

- `🐹️component.go:41` invokes only `client.RunMCP()` for the no-argument MCP process.
- No production source in this module calls `NewServer`, `Server.Serve`, or `Session.Dispatch`.
- The only `NewServer` reference is the G2 test harness in `🧪️contract_test.go:122`.

The change nevertheless adds 1,448 production lines: `🐹️event.go` (193), `🐹️protocol.go` (356), `🐹️server.go` (827), and `🐹️transport.go` (72). It is a second, unattached JSON-RPC/MCP server/event-log/transport implementation, not an implementation of the executable process. This violates the no-duplicate-dead-implementation requirement and makes the broad G2 contract suite non-evidence for production behavior.

The isolated pipe handshake did run the production executable and returned a valid initialize response. It establishes only that the delegated G1 server initializes; it does not exercise the new G2 runtime.

## Contract Review

The unattached implementation contains explicit JSON-RPC/MCP envelopes, initialize lifecycle, capabilities, tools/resources/templates/prompts, sorted cursor pagination, registry and payload limits, deterministic errors, generation ownership, event hash chaining/replay, progress, and cancellation. Its direct-session test suite exercises malformed/trailing JSON, invalid parameters, unknown method, cancellation before/during/after, duplicate/stale/ABA IDs, maximum-plus-one limits, progress/drop/reconnect, and atomic request/response event pairs.

Those results cannot be promoted to the production process because they construct `NewServer` directly. In particular, the claimed during-handler cancellation is not a transport test: the test concurrently calls `Session.Dispatch` directly (`🧪️contract_test.go:323-329`). `Server.Serve` instead scans a line and synchronously waits for `Session.Dispatch` to finish before reading the next line (`🐹️transport.go:46-58`); a cancellation notification arriving on the same transport during a blocking handler cannot be read and delivered until the handler returns. Thus even wiring this server into production would still fail the claimed live cancellation behavior.

There is an additional untested response-bound gap in the dead implementation: normal results are size-checked (`🐹️server.go:705-725`), but `encodeError` directly marshals an error response without the payload/nesting bound (`🐹️server.go:727-732`). This is not the primary verdict, but it invalidates the owner packet's unconditional “response bytes are bounded before emission” statement.

## Dependency Census And Boundary Review

The manifest cleanup itself is verified:

| Measure | `HEAD` | Live |
| --- | ---: | ---: |
| `require` rows | 59 | 2 |
| Local first-party rows | 1 | 2 |
| External rows | 58 | 0 |
| `go.sum` | 153 lines | absent |

Live `go.mod` has only genuine local replacements:

```text
github.com/usalu/semio/repo/client v0.0.0 => ../⌨️cli
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library
```

With `GOWORK=off`, `go list -m all` returns exactly the parent MCP, client, and library modules. The test dependency graph contains only those first-party module paths; no third-party module-backed package is present. A source/metadata scan found no `mcp-go`, Mark3Labs, vendor, copied-upstream, compatibility, or legacy-provider marker. The sole `fallback` hit is internal default-limit initialization, not a compatibility fallback.

The scoped diff has only the parent module's `go.mod`, removed `go.sum`, and the untracked new runtime/tests/fixture. `go.work`/`go.work.sum`, G1 CLI sources, shared files, Cargo, Nx, Wasm, and browser files were excluded from the G2 finding. `git diff --check -- <parent module>` passed.

## Executed Gates

| Gate | Result |
| --- | --- |
| `go test -count=1 ./...` | PASS — all 13 G2 test groups; dead runtime only |
| `GOWORK=off go test ./...` | PASS |
| `go test -race ./...` | PASS |
| `GOWORK=off go test -race -count=1 ./...` | PASS |
| `go vet ./...` | PASS |
| `go mod tidy -diff` and `GOWORK=off go mod tidy -diff` | PASS — empty output |
| `GOWORK=off go mod verify` | PASS — `all modules verified` |
| `GOWORK=off go list -m all` and dependency census | PASS — three first-party modules only |
| `GOOS=linux GOARCH=amd64 go build` | PASS |
| `GOOS=windows GOARCH=amd64 go build` | PASS |
| Production stdin/stdout initialize pipe | PASS — delegated `client.RunMCP`, protocol `2025-06-18` |

## Required Resolution Before Green

1. Remove the duplicate unattached protocol/server/event/transport implementation, or make it the sole executable production path and remove the conflicting delegated implementation. Do not retain two MCP servers.
2. If the owned runtime is selected, make transport request processing concurrent enough to receive and apply cancellation notifications while a handler is running; add a real pipe/transport during-cancellation test.
3. Bound error-response encoding as well as normal results, then add maximum and maximum-plus-one error-envelope tests.
4. Do not characterize `client.RunMCP` as accepted until the independent G1 blockers are resolved and G1 is re-audited GREEN.

No production source, manifest, lock, workspace, Cargo, Nx, Wasm, browser, or G1/shared file was edited by this audit.
