# Phase 9 G2 Go MCP Zero-External Contract

Date: 2026-08-25

Owner: G2 Go MCP packet

Status: second independent-audit RED remediated; scoped gates green; ready for final re-audit

## Scope Correction

The Phase 9 packet map names the nested path:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go`

Live validation showed that path contains only the pre-existing zero-byte `📋️project.json` and `📜️script.ts` scaffolds. It has no Go module, source, imports, build edge, or runtime reachability. The actual production module is its parent:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp`

The implementation was therefore moved to the live parent module. The temporary nested `go.mod` and Go sources created during discovery were removed; the two original empty scaffold files remain byte-identical. `go.work`, the accepted G1 CLI module, coordinator, shared scripts, root manifests, Rust, JavaScript, Cargo, Nx, Wasm, browser, and other ecosystems were not edited by G2.

## Exact Dependency Census

| Measure | Before (`HEAD`) | After |
| --- | ---: | ---: |
| Total `require` rows | 59 | 2 |
| First-party local rows | 1 | 2 |
| Third-party rows | 58 | 0 |
| Third-party direct rows | 0 | 0 |
| Third-party indirect rows | 58 | 0 |
| `go.sum` lines | 153 | absent, naturally unused |
| Third-party source import roots in the module | 0 | 0 |
| Third-party module-backed packages including tests | not independently reconstructed | 0 |

The two retained rows are genuine local first-party modules:

```text
github.com/usalu/semio/repo/client v0.0.0 => ../⌨️cli
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library
```

The second explicit local replacement is required because dependency-module `replace` directives do not propagate when this parent module is isolated with `GOWORK=off`. The later `📓️codex-p9-g1-go-cli-second-independent-audit-2026-08-25.md` records G1 GREEN. G2 reuses only its public repository-domain primitives through the narrow owned `RepositoryHandlers` interface; no G1 protocol/server type crosses the boundary.

All former parent manifest roots were removed together, including `github.com/mark3labs/mcp-go`, both JSON-schema providers, URI template/protobuf roots, and the stale G1 CLI/search/template/glob/YAML/SQLite closure. No removed-provider import, declaration, public type, or fallback remains in the G2 module. The manifest and test dependency closures contain only the three first-party module identities above.

## Owned Protocol Contract

The production package now contains concise, standard-library-only protocol sources organized by regions:

| Source | Contract |
| --- | --- |
| `🐹️protocol.go` | Exact JSON-RPC request/response/error/notification envelopes; string/integer IDs; MCP initialize/capabilities/tools/resources/templates/prompts/progress/cancellation schemas; exact object decoding. |
| `🐹️server.go` | Initialize/initialized/ready lifecycle; deterministic dispatch/errors; sorted bounded pagination; handler interfaces; concurrent cancellation and progress; peer generation ownership; duplicate/stale/ABA-safe request tracking; reconnect/drop; interruptible close; bounded structural decode/encode. |
| `🐹️event.go` | `semio.mcp.event/1` append-only request/response/notification/session events, atomic multi-event commits, monotonic sequences, SHA-256 hash chaining, deterministic JSONL encoding, bounded corruption-checked replay. |
| `🐹️transport.go` | Context-aware newline-framed transport with bounded scanner allocation, fixed handler workers, reader-side control ingestion, serialized output, queue overload errors, peer-drop signaling, cancellation, and reconnect through fresh generation ownership. |
| `🐹️repository.go` | Sole production registry and narrow `RepositoryHandlers` boundary; G2 schemas and handlers for six repository tools, eight resources, and four prompts; standard-stream transport. |
| `🐹️component.go` | No-argument process constructs `NewRepositoryServer` and drives `Server.Serve`; CLI arguments alone continue through the first-party CLI command primitive. |

Defaults are explicit: 1 MiB payloads, depth 64, 64 items per page, 4,096 registry items, 4,096 recent request IDs, 64 MiB event bytes, 100,000 events, and eight fixed handler workers with an eight-request queue. Negative limits are rejected. Payload maximum is accepted; maximum-plus-one is rejected. The complete success or error envelope is byte- and depth-checked before emission. Oversized success, error-data, and page results become the small owned `response too large` error; if even the ID-bearing minimal error cannot fit, `ErrPayloadTooLarge` is returned without an event write. Tool arguments and all MCP params must be exact JSON objects. Unknown handler errors are reduced to the fixed owned `internal error`; only explicitly owned `HandlerError` values may expose server error data.

The fixed-worker transport exposes an owned atomic `HandlerStats` snapshot for operational evidence: configured capacity, allocated workers, active handlers, queued requests, and remaining bounded queue credits. Admission increments a queue credit before the nonblocking send; worker receipt returns it before handler activation; rejected max-plus-one work returns it before encoding the busy response. The counters therefore remain race-safe and return exactly to zero active/queued work and full credits after recovery or interrupted peer close.

Each request/response exchange is committed as one event batch. Cancellation or maximum-plus-one failure during an event batch leaves the prior log byte-identical. Progress is an event before transport emission. Event replay checks schema, sequence, previous hash, current hash, payload validity, byte limit, count limit, and context cancellation.

Opening the same peer again increments its generation, atomically replaces ownership, and cancels the dropped session. Old-generation requests return `stale session`; old-generation cancellation notifications cannot cancel a reused request ID in the new generation. Within a generation, active duplicates and completed/stale IDs are rejected. A cancellation notification received before its request creates a bounded tombstone; a cancellation received after completion is ignored rather than poisoning a future unrelated ID.

## Independent-Audit RED Remediation

The first audit correctly found that the G2 stack was unattached, that `Serve` synchronously blocked its reader behind a handler, and that `encodeError` skipped response bounds. All three findings are resolved:

1. `component.go` contains no `client.RunMCP` call. Its no-argument branch calls the owned `runMCP`, which constructs `NewRepositoryServer` and calls `server.Serve`. `TestG2ProductionOwnedRuntimePipe` runs that exact function over `net.Pipe`, observes G2 initialize/tool capabilities, dispatches `ticket_open` through a fake `RepositoryHandlers`, and verifies the returned G2 event log. `TestG2ProductionComponentRejectsDirectDelegationMutation` fails if direct delegation is restored or the constructor/serve markers disappear.
2. `Serve` creates a fixed configured worker set once per connection. Its reader continues classifying and synchronously applying notifications while requests run in the bounded worker queue. It never creates a goroutine per request. A full queue returns the owned `handler queue full` error instead of blocking control ingestion. `TestG2SamePipeCancellationAndProgress` uses one real pipe: it initializes, begins a blocking production repository tool with a progress token, observes progress on that pipe, sends cancellation on the same pipe, receives `request cancelled`, and verifies the request/response event pair was committed atomically.
3. `encodeResult` and `encodeError` both check the complete encoded envelope. `TestG2ErrorEnvelopeMaximumAndMaximumPlusOne` constructs an error envelope of exactly 512 bytes and accepts it, constructs the exact 513-byte variant and receives the bounded owned error, forces an oversized list page through the same bound, and checks every request event has its adjacent response event. Invalid owned error data is reduced to a bounded internal error.

## Second Independent-Audit RED Remediation

The second audit found one remaining evidence gap: the fixed queue's overload branch had no executable real-pipe saturation trace. The implementation was already bounded, but G2 had not proven exact capacity, max-plus-one rejection, same-pipe control liveness while every worker and queue slot were occupied, or saturated-close cleanup.

`TestG2ProductionPipeSaturationControlAndRecovery` now constructs the same production `NewRepositoryServerWithLimits` and `Server.Serve` stack used by the component over `net.Pipe`, with two fixed handlers and two queue slots. It proves these exact transitions:

1. worker allocation `0 -> 2`, then active handlers `0 -> 2` while queue occupancy remains zero and both queue credits remain available;
2. queue occupancy `0 -> 1 -> 2`, queue credits `2 -> 1 -> 0`, and no third handler entry;
3. max-plus-one returns request-correlated owned `-32005` / `handler queue full`, leaves occupancy at two, and never invokes the repository handler;
4. cancellation notifications remain readable on that same saturated pipe, each active handler returns owned `-32800`, each queued handler emits its preserved progress token as a worker becomes available, and credits return exactly `0 -> 1 -> 2`;
5. cancelling the promoted queued requests returns active and queued counts to zero, after which a new request starts, emits progress, succeeds, and leaves full credits; and
6. peer close stops exactly both fixed workers with no active request, queued request, repository call, or worker leak. Every busy, cancelled, and successful exchange has one adjacent atomic request/response event pair.

`TestG2ProductionPipeCloseDuringSaturation` separately closes the peer with both workers blocked and both queue slots full. It proves the active contexts cancel, queued work does not reach a repository handler, the queue is drained, the session closes, all request maps and handler counters return to zero, both workers exit, and event pairs remain complete. `TestG2TransportRejectsBusyBranchMutation` executes three hostile mutations: replacing the nonblocking `default`, changing the owned busy code, and suppressing rejected-request credit return. All three mutated sources are rejected by the invariant checker while the live branch passes.

## Canonical And Hostile Coverage

The language-agnostic fixture is `🧫️fixtures/g2-contract.json` with schema `semio.mcp.contract/1`. It contains byte-exact deterministic vectors for:

- initialize and capabilities;
- tool call;
- resource read;
- prompt get;
- unknown method;
- invalid params;
- invalid envelope;
- malformed JSON; and
- trailing JSON data.

`🧪️contract_test.go` executes the fixture plus:

- the actual owned production `runMCP` initialize/capability/tool/event path;
- the hostile source mutation that restores exclusive `client.RunMCP` delegation;
- same-pipe progress and during-handler cancellation through fixed transport workers;
- real production-server-pipe worker and queue saturation at zero, exact maximum, and maximum plus one;
- owned `-32005` overload without handler admission or partial event commit, plus same-pipe cancellation/progress and exact credit recovery;
- close during full saturation with no request, queue, repository-handler, or fixed-worker leak;
- hostile busy-branch mutations for blocking admission, changed error code, and missing rejected-request credit return;
- initialization ordering, duplicate initialize, sorted tool pages, next cursor, resource templates, and invalid cursor;
- registry maximum plus one;
- cancellation before, during, and after a handler;
- duplicate active ID and completed stale ID;
- same-peer drop/reconnect and generation increment;
- request-ID reuse across generations without ABA cancellation;
- interrupted close that still completes and never records a partial request/response pair;
- exact maximum and maximum-plus-one payloads;
- exact maximum and maximum-plus-one nesting;
- null/fractional IDs and null params;
- progress emission and progress-token preservation;
- dropped progress sink followed by successful short-shortage reconnect;
- deterministic encoding across independent executions;
- successful event replay, corrupt replay rejection, replay bounds, cancelled atomic commit, and maximum-plus-one atomic batch rejection;
- deterministic internal error redaction and structurally oversized handler output; and
- exact-maximum and maximum-plus-one error envelopes and oversized page output.

## Verification Evidence

All commands ran from the live parent MCP module.

### Executable behavior

```text
GOWORK=off go test -count=1 ./...
ok github.com/usalu/semio/repo/mcp 0.578s

GOWORK=off go test -race -count=1 ./...
ok github.com/usalu/semio/repo/mcp 1.597s
```

The focused verbose run printed every named golden and hostile case and completed:

```text
PASS: 19 top-level G2 groups, including three rejected busy-branch mutations
ok github.com/usalu/semio/repo/mcp 0.471s
```

No baseline failure exists in this scoped module suite.

### Static and module gates

```text
GOWORK=off go vet ./...
PASS

GOWORK=off go mod tidy -diff
PASS: empty output and diff

GOWORK=off go mod verify
all modules verified
```

```text
GOWORK=off go list -m all
github.com/usalu/semio/repo/mcp
github.com/usalu/semio/repo/client v0.0.0 => ../⌨️cli
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library
```

```text
GOWORK=off go list -deps -test -f '{{if .Module}}{{.Module.Path}}{{end}}' ./... | sed '/^$/d' | sort -u
github.com/usalu/semio/repo/client
github.com/usalu/semio/repo/go
github.com/usalu/semio/repo/mcp
```

The removed-root declaration/import scan emitted no rows.

### Cross-platform and diff gates

```text
GOWORK=off GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...
ok github.com/usalu/semio/repo/mcp 0.006s

GOWORK=off GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...
ok github.com/usalu/semio/repo/mcp 0.011s

git diff --check -- <live MCP module>
PASS
```

All changed Go files were formatted with `gofmt`. No Cargo, Nx, Wasm, or browser command was run.

## Audit Conclusion

The actual production MCP module, rather than the inert nested scaffold named by the packet map, now has zero third-party manifest rows, zero third-party module-backed test packages, zero third-party source imports, and no `go.sum`. Its only dependency direction is into the accepted first-party CLI and repo library. The owned G2 runtime is the sole MCP process path, while the narrow domain adapter reuses first-party repository operations without importing the G1 protocol implementation. Exact JSON-RPC/MCP schemas, fixed-worker same-pipe cancellation/progress, complete-envelope bounds, generation-safe recovery, and atomic event-sourced protocol logging are executable through the live component. The packet is ready for independent re-audit; terminal `go.work` and repository-wide baseline integration remain outside G2 ownership.
