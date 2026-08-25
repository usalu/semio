# Phase 9 G2 Go MCP Third Independent Audit

Date: 2026-08-25  
Auditor: Codex independent G2 re-audit  
Verdict: **GREEN — accept the owned G2 Go MCP packet.**

## Scope Re-read

This fresh read-only audit re-read the Phase 9 G2 zero-external contract, the first G2 RED audit, the second G2 RED audit, the G1 second independent GREEN audit, and the live worktree. It then independently exercised the current parent Go module:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp`

The historical blockers are resolved in live source. `🐹️component.go` takes the no-argument executable path through `runMCP → NewRepositoryServer → Server.Serve`; it has no production `client.RunMCP` delegation. The G1 predecessor remains GREEN for its owned packet according to its second independent audit. G2's only G1 boundary is the narrow `RepositoryHandlers` adapter using first-party repository-domain primitives, not a G1 MCP/server type.

The nominal `📦️packages/🐹️go` child remains inert scaffolding with no nested Go module. This audit made no production, G1, shared, workspace, Cargo, Nx, Wasm, or browser edit.

## Fresh Production Pipe Evidence

`GOWORK=off go test -count=10 -v -run '^(TestG2ProductionOwnedRuntimePipe|TestG2SamePipeCancellationAndProgress|TestG2ProductionPipeSaturationControlAndRecovery|TestG2ProductionPipeCloseDuringSaturation|TestG2TransportRejectsBusyBranchMutation|TestG2ErrorEnvelopeMaximumAndMaximumPlusOne)$' ./...` passed all 60 invocations.

The tested path is a real `net.Pipe`, a production repository-server constructor, and `Server.Serve`—not direct `Session.Dispatch` substitution:

| Required hostile condition | Fresh executable evidence |
| --- | --- |
| Owned executable lifecycle | `TestG2ProductionOwnedRuntimePipe` runs `runMCP`, initializes over the pipe, calls `ticket_open`, reaches the repository double, and inspects the owned event log. |
| Exactly two active workers and a capacity-two queue | `TestG2ProductionPipeSaturationControlAndRecovery` sets `MaxHandlers=2`; `HandlerStats` observes `Workers=2, Active=2, Queued=2, Credits=0`. The transport owns `make(chan []byte, MaxHandlers)` and starts exactly `MaxHandlers` workers. |
| Full queue rejects max+1 without admission | The overflow request returns its own response ID, `-32005`, and `"handler queue full"`. The repository call counter remains 2, so no third handler starts; stats remain active 2/queued 2/credits 0. Its response receives the atomic `request.received`/`response.sent` pair. |
| Same-pipe control stays live | While both workers and both queue slots are saturated, `notifications/cancelled` is read on the same pipe. The selected blocking handler returns owned `-32800`; its queued successor emits progress and starts. |
| Exact credit recovery and post-drain recovery | After each promotion, stats show queued 1/credits 1 then queued 0/credits 2. Cancelling the promoted requests drains all work; a later `after` request emits progress and returns success. Peak active work is 2, final active count is 0, and historical repository calls are exactly 5. |
| Close at active 2 / queued 2 | `TestG2ProductionPipeCloseDuringSaturation` closes the client at that point. It observes workers 0, active 0, queued 0, credits 2, session active-request map length 0, repository active count 0, and no queued repository invocation. The historical repository-call count is correctly 2 (the two already-active calls), not a leaked outstanding call. |

The same-pipe blocking-handler progress/cancellation probe also passes separately. `TestG2TransportRejectsBusyBranchMutation` rejects hostile source mutations that make admission blocking, change the owned busy code, or fail to return a queue credit; its structural verifier also requires the bounded admission increment, `select`, jobs send, `default`, decrement, `CodeServerBusy`, and owned message.

## Preserved Runtime And Bounds

There is one owned production constructor family (`NewRepositoryServer` / limit-injectable `NewRepositoryServerWithLimits`) and one owned `Server.Serve` implementation. The only `go func` in transport is the fixed worker body started by the `for range MaxHandlers` loop; the reader does not create a goroutine per request. Notifications remain reader-side control work, so cancellation can reach an active worker during saturation.

`encodeResult` and `encodeError` both validate final serialized response size and nesting. The former RED error-envelope gap is covered by `TestG2ErrorEnvelopeMaximumAndMaximumPlusOne`, included in the ten-iteration probe. `commitExchange` commits request/response event pairs together; saturation, cancellation, close, and envelope tests assert adjacency.

`GOWORK=off go test -list '^TestG2' ./...` lists 19 G2 top-level test groups. They cover golden vectors, lifecycle, registry/payload/nesting/response bounds, cancellation before/during/after, same-pipe cancellation/progress, saturation/recovery/close, duplicate/ABA reconnect, interrupted close, dropped peer recovery, structural output/errors, deterministic event replay, and mutation resistance.

## Zero-Dependency And Topology Census

| Measure | Fresh result |
| --- | --- |
| Parent manifests | One `go.mod`; no `go.sum`, Cargo manifest, package manifest, Bun lock, or nested Go module |
| Direct requirements | 2, both local first-party: `repo/client` and indirect `repo/go` |
| Replacements | `../⌨️cli` and `../../📚️library`, both local first-party |
| `GOWORK=off go list -m all` | MCP, client, and repo-go only |
| `GOWORK=off go list -deps -test` module identities | The same three first-party module paths only |
| Third-party module identities | 0 |
| Source/test imports | Standard library plus `github.com/usalu/semio/repo/client` only |
| Legacy/alternate provider census | No production `client.RunMCP`, Mark3Labs, `mcp-go`, vendor, compatibility, or generated/embed provider marker; the sole `client.RunMCP` string is the hostile-mutation test sentinel |
| Actual G2 worktree slice | Modified `go.mod`, removed `go.sum`, modified `🐹️component.go`, and new owned protocol/event/repository/server/transport/tests/fixture; no nested scaffolding implementation |

## Executed Gates

All commands ran from the live G2 parent module with `GOWORK=off` where applicable.

| Gate | Result |
| --- | --- |
| Focused production/saturation/mutation/bounds net.Pipe probe, count 10 | PASS — 60 invocations |
| Full module tests: `go test -count=1 ./...` | PASS |
| Race tests: `go test -race -count=1 ./...` | PASS |
| Repeated full tests: `go test -count=3 ./...` | PASS |
| `go vet ./...` | PASS |
| `go mod tidy -diff` | PASS — empty output/diff |
| `go mod verify` | PASS — `all modules verified` |
| Module and test-dependency lists | PASS — first-party-only census above |
| Linux compile gate: `GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| Windows compile gate: `GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...` | PASS |
| `git diff --check -- .` from G2 module | PASS |

## Conclusion

The previously missing real-pipe saturation proof now demonstrates the precise bounded admission, owned busy response, same-pipe control liveness, cancellation, promotion, credit recovery, post-drain reuse, atomic exchange records, and saturated-close cleanup required by G2. The live production path, bounds, fixed-worker topology, first-party-only dependency graph, and required host gates are independently revalidated. There are **no G2 acceptance blockers**.
