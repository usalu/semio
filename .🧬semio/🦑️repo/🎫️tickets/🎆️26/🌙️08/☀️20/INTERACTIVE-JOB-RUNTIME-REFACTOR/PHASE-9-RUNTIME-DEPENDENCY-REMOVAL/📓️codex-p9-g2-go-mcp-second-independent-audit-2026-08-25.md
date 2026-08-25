# Phase 9 G2 Go MCP Second Independent Audit

Date: 2026-08-25  
Auditor: Codex independent re-audit  
Verdict: **RED — do not accept G2 yet**

## Scope And Inputs Re-read

Re-read the Phase 9 plan, Z0 dependency-verifier truth, G1 second independent **GREEN** audit, G2 zero-external contract, the prior G2 **RED** audit, and the actual live diff. The live production module is:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp`

The nominal `📦️packages/🐹️go` child remains inert, pre-existing scaffolding: its two files are each zero bytes. It has no Go module or implementation. This audit did not edit production, G1, shared, workspace, Cargo, Nx, Wasm, or browser files.

## Reproduced Remediation: GREEN

The former decisive production-path blocker is resolved. The no-argument branch in `🐹️component.go` exclusively calls the owned `runMCP`; that function constructs `NewRepositoryServer(repository)` and invokes `server.Serve(ctx, "stdio", transport)`. Production source contains no `client.RunMCP` call. There is one owned `Server.Serve` implementation and one owned repository-server constructor; no legacy provider (`mcp-go`/Mark3Labs) marker or alternate MCP server remains.

`TestG2ProductionOwnedRuntimePipe` executes this exact `runMCP → NewRepositoryServer → Server.Serve` path over `net.Pipe`, observes initialize/tool behavior, reaches the fake repository handler, and inspects the owned event log. `TestG2ProductionComponentRejectsDirectDelegationMutation` rejects a restored `client.RunMCP` token and requires all three owned-path markers. The narrow `RepositoryHandlers` interface is the G2 boundary; `ClientRepository` uses G1 only through public repository-domain primitives (`Tool*` functions and `ToolResult`), not G1 protocol/server types.

`TestG2SamePipeCancellationAndProgress` is a real same-`net.Pipe` proof of the central liveness case: a blocking production repository tool emits progress, the reader accepts a `notifications/cancelled` message on that same pipe, the response is the owned `-32800`, and the matching `request.received`/`response.sent` event pair is adjacent. `Server.Serve` creates exactly `MaxHandlers` workers and a bounded `MaxHandlers` request channel, rather than a goroutine per request. `TestG2InterruptedCloseCompletesWithoutPartialExchange` and the bounded-error test additionally reject partial exchange commits.

Complete-envelope enforcement is present: `encodeResult` and `encodeError` both validate final serialized byte size and nesting. `TestG2ErrorEnvelopeMaximumAndMaximumPlusOne` accepts an exactly 512-byte error response, turns the constructed 513-byte error into the owned bounded error, bounds an oversized list page, and checks adjacent event pairs. Invalid `HandlerError.Data` is deterministically reduced to the owned internal-error envelope by `TestG2OwnedErrorsAndStructuralOutputBounds`.

## Decisive Remaining Blocker

The required real-pipe overload proof is absent. No test invokes `Server.Serve` with a deliberately saturated `MaxHandlers` worker set and queue, and no test asserts `CodeServerBusy` / `"handler queue full"`. A source census finds the busy code only in `🐹️transport.go`; `🧪️contract_test.go` has no `CodeServerBusy`, `handler queue full`, or constrained-`MaxHandlers` assertion.

Thus the implementation has a bounded-worker/queue design, but the demanded hostile matrix does **not demonstrate** its full-queue response or that reader-side cancellation remains live while saturation is present. The existing same-pipe test proves one blocked handler and cancellation/progress, not queue saturation. This is an acceptance blocker because the packet expressly requires executable bounded-queue/busy behavior, not a source-only claim.

Required resolution: add a real `net.Pipe` test that configures a small worker/queue bound, blocks every admitted handler, sends one additional request, receives owned `-32005`, proves no extra handler began, then sends cancellation on the same pipe and receives the owned `-32800` for the blocked request. Keep the existing atomic event-pair assertions for both busy and cancelled request paths.

## Independent Gate Evidence

All commands below ran from the live parent module with `GOWORK=off`:

| Gate | Result |
| --- | --- |
| `go test -count=1 ./...` | PASS |
| `go test -race -count=1 ./...` | PASS |
| `go test -v -run '^TestG2' ./...` | PASS: 16 groups, including canonical vectors and hostile lifecycle/cancellation/bounds/replay cases |
| `go vet ./...` | PASS |
| `go mod tidy -diff` | PASS: empty diff |
| `go mod verify` | PASS: `all modules verified` |
| `go list -m all` | PASS: MCP, first-party client, first-party repo library only |
| `go list -deps -test ...` | PASS: same three first-party module identities only |
| Linux cross gate | PASS: `GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...` |
| Windows cross gate | PASS: `GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...` |
| `git diff --check -- .` | PASS |

The direct manifest has two first-party local requirements and replacements only; `go.sum` is absent. The module has no nested Go manifest, vendor tree, Cargo manifest, package manifest, or lock file. Its source/test imports are standard library plus `github.com/usalu/semio/repo/client`, a local first-party replacement.

## Census

| Item | Count / Result |
| --- | --- |
| Live G2 Go module | 1 parent module |
| Nested Go modules | 0 |
| `go.sum` | absent |
| Direct third-party requirements | 0 |
| Resolved third-party module identities with `GOWORK=off` | 0 |
| Production `client.RunMCP` calls | 0 |
| Owned `runMCP` / `NewRepositoryServer` / `Server.Serve` implementations | 1 / 1 / 1 |
| Real-pipe production lifecycle test | 1 |
| Real-pipe progress/cancel test | 1 |
| Real-pipe busy/saturation test | **0 — blocker** |

All other prior G2 RED findings are remediated and executable. The sole remaining blocker is the missing hostile, real-pipe bounded-queue/`-32005` proof.
