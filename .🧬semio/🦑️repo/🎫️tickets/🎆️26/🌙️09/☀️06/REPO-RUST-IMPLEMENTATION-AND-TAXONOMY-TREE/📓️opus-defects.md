# 📓️ Defect closure — D1, D2, D4–D9 of `📓️audit-final.md`

Run 2026-09-06, ~13:40–15:00 UTC, Windows 11 host. `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`,
`SEMIO_TEST_BUDGET_MS=600000`–`900000`. Evidence under `🗑️generated/defects/`
(`parity-*.txt`, `contract-repo.txt`, `go-workspace.txt`, `daemon-tests.txt`, `daemon-run.txt`).

**D3 (`tree` CLI verb) is deliberately NOT in this pass** — it is deferred until the concurrent
`final-fixes` agent releases `⌨️cli`. `📐️model`, `📊️metrics`, `🗂️codebase`, `🚚️move`, `🧩️vscode` and
`⌨️cli/📦️packages/🦀️rust` were not touched for the same reason; the two places where that changed a
deliverable are called out under D9 and in §"Not done".

## D1 — Go MCP server dropped queued requests on stdin EOF (HIGH) — FIXED

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🐹️.go`, `Server.Serve`.

`finish(cause)` ran `session.drop(cause)` **before** `close(jobs)` / `workers.Wait()`. `drop` sets the
session phase to `phaseClosing`, and `beginRequest` refuses everything from that point with
`-32004 session closed`. A peer that writes its whole burst and closes stdin therefore reached EOF
while its requests were still sitting in the `jobs` channel, and got `session closed` for requests the
server had already received in full.

The EOF path now drains before it drops; the error paths (`ctx` cancelled, oversized payload, scanner
error) keep dropping first, because there the peer really is gone:

```go
drain := func(cause error) error {
	close(jobs)
	workers.Wait()
	session.drop(cause)
	...
}
finish := func(cause error) error {
	session.drop(cause)
	return drain(cause)
}
```
…with the clean end of input calling `return drain(ErrPeerDropped)` instead of `finish(...)`.

This is the semantics the Rust twin already had (`🦀️.rs` `serve` sets `closed`, joins its workers and
only then calls `session.close()`), so the two implementations now agree by construction.

### Evidence

Real `printf | binary` run against the built Go server — the exact shape the audit reproduced the
defect with:

```
$ printf '{…"method":"initialize"…}\n{…"notifications/initialized"…}\n{…"id":2,"method":"tools/list"…}\n' \
    | .🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp.exe
{"id": 1, "ok": true, "error": null}
{"id": 2, "ok": true, "error": null}
```

New Go unit test `TestG2TransportEOFCompletesQueuedRequests` (`🧪️_test.go`) drives the same burst over
a half-closed transport — a reader that ends, a writer that stays — and asserts both ids are answered
and neither carries `CodeStaleSession`. Reverting only `drain` → `finish` in the EOF path fails it:

```
--- FAIL: TestG2TransportEOFCompletesQueuedRequests (0.00s)
    🧪️_test.go:304: EOF refused a delivered request: {"jsonrpc":"2.0","id":2,"error":{"code":-32004,"message":"session closed"}}
```

`go test ./... -count=1` for `github.com/usalu/semio/repo/mcp`: **ok**, 0.6 s.

The same burst through the dev entry point, which also proves the rebuilt `🚀️bin` wiring:

```
$ SEMIO_REPO_IMPLEMENTATION=go printf '<burst>' | bun ./📜️script.ts dev mcp stdio client
{"jsonrpc":"2.0","id":2,"error":{"code":-32002,"message":"session not initialized"}}
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25",…}}
```

**A separate, pre-existing hazard this exposes.** Both implementations hand queued requests to a
worker POOL, so a pipelined burst can be dispatched out of order and `tools/list` can reach `route`
before `initialize` has promoted the session — `-32002 session not initialized`. That is a different
bug from D1 (`-32004 session closed`), it exists in the Rust twin by the same construction, and it is
not what D1 asked for, so it was left alone. It IS why the new scenario asserts "every delivered
request is answered and none is refused as a closed session" rather than "`tools/list` succeeded":
that phrasing is deterministic under both implementations, and it is the property D1 actually broke.
Worth its own ticket.

### The one existing test whose contract changed

`TestG2ProductionPipeCloseDuringSaturation` used a repository handler that blocks on `<-ctx.Done()`
forever and relied on the peer close cancelling it. Under a drain the loop waits for those handlers,
so the test now closes an explicit `release` channel after the close (and the handler selects on
`release` or `ctx.Done()`). Its assertions are otherwise unchanged — a peer that closes BOTH
directions still cannot receive anything, so the still-queued calls are refused rather than served,
which the comment above the test now states. That case and the new half-closed one are the two halves
of the contract and sit next to each other.

## D7 (mcp + statutes part) — Go adapters — DONE for `🔌️mcp`, NOT DONE for `📜️statutes`

### The Go MCP package is now importable

`🔌️mcp/📦️packages/🐹️go/🐹️.go` was `package main`, so no test adapter could link it. It is now
`package mcp` (library), mirroring `⌨️cli` exactly:

- `📦️packages/🐹️go/🐹️.go` — `package mcp`, `main()` removed, `🦀️Entrypoint` region exports
  `MCPProfileEnvironment`, `ResolveMCPProfile`, `RunStdio`, `RunMCP`, `RunMCPForProfile`, `ServeMCP`
  and `StdioTransport` (fields exported).
- **new** `📦️packages/🐹️go/🚀️bin/{go.mod, 📦️main.go}` — `github.com/usalu/semio/repo/mcp/bin`,
  `package main`, one `mcp.RunStdio(...)` call, exactly like `⌨️cli/📦️packages/🐹️go/🚀️bin`.
- **new** `🧪️Projection` region in the godfile: `NewContractServer(Limits)`, the Go twin of the Rust
  crate's `contract_server`, so a language-agnostic adapter observes the same fixed tool/resource/
  prompt surface in both implementations.
- `🧪️_test.go` moved to `package mcp` and to the exported names.

Build wiring updated to the new entry point: `go.work` (new `…/🐹️go/🚀️bin` member), root
`📜️script.ts` (`REPO_MCP_ENTRY_GO`, used by `setup` and by `test repo-mcp`), `📚️library`'s
`buildRepoMcpBin`, the module's own `📜️script.ts` (`build` and `run` now `cwd` into `🚀️bin`), and the
two existing Go test adapters that build the binary themselves
(`🧪️tests/📋️capability-listing/🐹️.go`, `🧪️tests/🤝️jsonrpc-handshake/🐹️.go`). `.vscode` needed no
change — the launch entries go through the nx targets, which call the package script.

### New Go adapters

- `🔌️mcp/🧪️tests/🔗️event-log-chain/🐹️.go` — commits the fixture's inputs through `mcp.NewEventLog`
  and projects the per-record digests plus `log.Snapshot()` as the JSONL rendering; replays the golden
  chain intact and with one record's kind altered through `mcp.ReplayEvents`.
- `🔌️mcp/🧪️tests/📞️tool-call-roundtrip/🐹️.go` — dispatches each `2️⃣g2-contract.json` vector against a
  fresh `mcp.NewContractServer` session and projects the parsed result / the JSON-RPC error code.

### New scenario `eof-after-burst-completes-queued-requests`

Added to `🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/🥒️.feature` (`@level-fundamental`, `@mode-differential`,
under the file's existing `@capability-mcp-initialize-handshake` / `@oracle-modelcontextprotocol-sdk`)
with all three adapters:

- `🐹️.go` — spawns the real `semio-repo-mcp` binary, writes the burst, **closes stdin**, and only then
  reads every reply.
- `🦀️.rs` — `mcp::serve` over a `Cursor` that ends after the burst and a capturing writer.
- `🟦️.ts` — the reference SDK `Server` on a real `StdioServerTransport` whose `Readable` ends after the
  burst; the oracle answers from Anthropic's implementation, not from ours.

All three project `{answered: [sorted request ids], sessionClosedRefusals: n}` — the invariant is that
every delivered request is answered and none is refused as a closed session, which is exactly D1 and
is race-free with respect to the (pre-existing, both-implementation) pipelining order of
`initialize` vs `tools/list`.

### Evidence

```
$ bun 🧪️test/📜️script.ts parity fundamental --owner …/🔌️mcp
[test] level=fundamental cases=4 executed=21 passed=21 failed=0 errored=0 parity=21/21

$ bun 🧪️test/📜️script.ts parity quick --owner …/🔌️mcp
[test] level=quick cases=4 executed=30 passed=30 failed=0 errored=0 parity=30/30
```

`discover` now reports `[rust,typescript,go]` for all four `🔌️mcp` cases, including
`🔗️event-log-chain` and `📞️tool-call-roundtrip`.

Every `go.work` member still builds, vets and is gofmt-clean after the split: **27/27**
(`🗑️generated/defects/go-workspace.txt`), the two new members being `🔌️mcp/…/🚀️bin` and the split
library.

### `📜️statutes/🗜️breach-cache-envelope` — NOT DONE

Left open deliberately. Writing that adapter means calling the Go statutes module's gzip + SHA-256
envelope, and `📜️statutes` is one hop from the `🗂️codebase`/`📐️model` surface the concurrent
`final-fixes` agent is editing; starting a Go adapter against a moving package would have produced a
false red. `📜️statutes` parity is green as it stands (rust + typescript oracle) and the case is
listed in the "what is left" section below.

## D5 — `🧩️providers/🌿️git-version-control` "not exercised" — EXPLAINED, NOT A DEFECT

Nothing is broken. All four scenarios of the case carry `@level-long`, and the feature file says why:

> The scenarios are `@level-long` because they spawn a real version control system.

`parity fundamental` and `parity quick` therefore have no scenario to serve, and the harness reports
"no implementation served the requested phase(s) oracle, subject" — which is the correct message for
a case whose whole scenario set sits at a level the run did not ask for. Run at its declared level it
executes for all three implementations:

```
$ bun 🧪️test/📜️script.ts parity long --case 🌿️git-version-control
[test] level=long cases=1 executed=12 passed=12 failed=0 errored=0 parity=12/12
```

12 = 4 scenarios × (go subject + rust subject + `git-cli` oracle). The `git-cli` oracle is registered
in `🧩️providers/🔮️oracle/🔣️.json`, declares `repo.providers.git-version-control`, and resolves on this
host. The `fundamental` level is legitimately empty for this case; that is now stated in
`🧩️providers/README.md` so the next audit does not read it as a gap.

## D4 — `.NET` subject host could not build — FIXED

`🧪️test/📦️packages/🔷️dotnet/🧪️Semio.Repo.Test.csproj`: `<Compile Include="🔷️host.cs" />` →
`<Compile Include="🔷️.cs" />`.

```
$ dotnet build 🧪️Semio.Repo.Test.csproj
  🧪️Semio.Repo.Test -> …\bin\Debug\net8.0\Semio.Repo.Test.dll
Build succeeded.  0 Warning(s)  0 Error(s)

$ bun 🧪️test/📜️script.ts parity fundamental --case 🖥️host-protocol-parity
[test] level=fundamental cases=1 executed=10 passed=10 failed=0 errored=0 parity=20/20
```

10 executed = 5 subject implementations × 2 scenarios; 20 parity pairs = C(5,2) × 2. The dotnet
subject's own result file was written by this run and both scenarios are `"status":"passed"`
(`⚡️cache/tests/results/…-host-protocol-parity-subject-dotnet/📤️results.jsonl`). `dotnet` on this host
is 10.0.303.

## D2 — Windows server-side listener for the dashboard daemon — IMPLEMENTED

`🎛️dashboard/🌀️daemon/🦀️.rs`. std only; the two — now three — Win32 entry points a named-pipe *server*
needs are hand-declared in an `extern "system"` block, which is an operating-system call, not a
dependency (the client half already reaches the same object through `std::fs`).

- `ipc::listen(root)` — creates the cache directory and records the pipe name in
  `socket_path(root)` (`daemon.pipe.name`), so `status` can print it.
- `ipc::accept(name)` — `CreateNamedPipeW` (`PIPE_ACCESS_DUPLEX`, byte type, wait mode,
  `PIPE_UNLIMITED_INSTANCES`) + `ConnectNamedPipe`, treating `ERROR_PIPE_CONNECTED` as success, and
  hands back a `std::fs::File` over the raw handle.
- `ipc::peek(&File)` — `PeekNamedPipe` readiness.
- `supervisor::ClientReader` (windows) — the same bounded cursor the unix loop uses, sitting directly
  beside it, with `peek` in place of the unix non-blocking read.
- `supervisor::serve` (windows) — one accept thread (a named pipe has no non-blocking accept worth
  having; `PIPE_NOWAIT` is a compatibility relic) feeding the same single-threaded turn the unix loop
  takes: poll every client, apply what they said, `tick()`.
- `supervisor::stop` — `taskkill /PID … /T /F` on windows, mirroring the unix `kill -TERM`, and the
  socket/pipe-name file is now removed on both.
- `start_detached` — the child's stdio is `null` on both platforms so `semio daemon start` returns
  instead of holding the parent's console.

### The bug that made the first attempt only half work

The first version gave each client a blocking reader thread. A pipe opened without
`FILE_FLAG_OVERLAPPED` **serialises every operation on the file object**, so the parked `ReadFile`
blocked every *write* to the same pipe — including writes through a duplicated handle from the serve
thread. The greeting (written before the reader thread starts) arrived; nothing after it ever did.
`PeekNamedPipe` + a short read is what removes the parked read, and it is also what makes the windows
loop structurally identical to the unix one.

### Evidence

Two new `#[cfg(windows)]` unit tests in the daemon's own `🔖️Tests` region:

```
running 3 tests
test daemon::tests::a_named_pipe_round_trips_one_frame_each_way ... ok
test daemon::tests::the_windows_serve_loop_greets_and_answers_a_client ... ok
test daemon::tests::unknown_subcommand_returns_usage_without_side_effects ... ok
test result: ok. 3 passed; 0 failed
```

`a_named_pipe_round_trips_one_frame_each_way` is the frame round trip over a real pipe (a control
frame each way through `ipc::write_control` / `ipc::read_frame`).
`the_windows_serve_loop_greets_and_answers_a_client` runs the whole `supervisor::serve` loop against a
temporary root, connects with the production `ipc::connect`, and asserts the `Attached` greeting and a
`Pong` for a `Ping` — the transport `semio daemon attach` rides on.

`cargo clippy --no-deps -p semio-framework-repo-dashboard --all-targets`: **0 warnings** attributable
to the crate.

Real end-to-end run of the release binary is recorded in `🗑️generated/defects/daemon-run.txt`.

`attach` itself is `crate::terminal::run(&root)`, a full-screen TUI that cannot be driven from a
non-interactive shell; what it needs from this module — connect, greeting, framed request/answer — is
what the serve-loop test and the recorded run prove. `🗑️generated/defects/pipe-client.ps1` is a
20-line client kept as an input file for re-running that probe by hand.

## D6 — Gherkin continuation lines — FIXED

`🎛️dashboard/🧪️tests/🌳️command-tree-projection/🥒️.feature` lines 23/31/33: three step texts wrapped
onto a continuation line without `And`/`But`. Each is now one line. A full `contract` run reports
**zero** breaches for `🎛️dashboard` and zero for `🔌️mcp`.

Note for the next pass: the same authoring mistake now exists in
`🗂️codebase/🧪️tests/🚶️workspace-walk/🥒️.feature` (7 `Unrecognized line` breaches, lines 34–37 and
46–48). That module belongs to the concurrent agent in this window and was left alone.

## D8 — Oracle manifests missing a `capability` — FIXED

The six entries did declare capabilities — but spelled with dots (`repo.identity.leading-grapheme`)
while their own feature files tag them with dashes (`@capability-repo-identity-leading-grapheme`), and
the registry match is exact. The manifests are now spelled the way their features are, in the oracle
entries **and** in the `noOracleDecisions` of the same file, so each module is internally consistent:

| Manifest | changed |
| --- | --- |
| `🪪️identity/🔮️oracle/🔣️.json` | `intl-segmenter` → `repo-identity-leading-grapheme` |
| `🔎️search/🔮️oracle/🔣️.json` | `semio-search-reference-ts` + decision → `repo-search-{rank,append-replay}` |
| `📜️statutes/🔮️oracle/🔣️.json` | `node-zlib-crypto` + decision → `repo-statutes-*` |
| `🏠️workspace/🔮️oracle/🔣️.json` | `micromatch`, `gitignore-js` + 3 decisions → `repo-workspace-*` |
| `🧾️yaml/🔮️oracle/🔣️.json` | `yaml-js` + decision → `repo-yaml-*` |

Modules whose features tag capabilities with dots (`🌳️tree`, `🎫️tickets`, `🎯️goals`, `🎛️dashboard`, …)
were left alone — they already match.

```
$ grep -c "does not declare capability" <full contract run>
0
```

Repo-attributable `contract` breaches went from the audit's 85 to **83**: the 6 oracle-capability and
3 dashboard-Gherkin breaches are gone, and 7 new ones appeared in `🗂️codebase/🚶️workspace-walk` (the
concurrent agent's module, see D6). The remaining 76 are the pre-existing `📚️library`
`testing/dependency` pattern the audit already characterised.

## D9 — Convention deviations

### `serde`/`serde_json` `{ workspace = true }`

Nothing to do outside the concurrent agent's crates. A sweep of every `Cargo.toml` under
`🧰️framework/🛍️products/🦑️repo` finds exactly two that pin versions instead of using the workspace
entry — `📐️model` and `⌨️cli` — and **both belong to the `final-fixes` agent**. Every other
`semio-framework-repo-*` crate already uses `serde = { workspace = true }` /
`serde_json = { workspace = true }`.

### Module `README.md`

Written for **22** modules that lacked one: `⌨️cli`, `🌳️tree`, `🎛️dashboard`, `🎫️tickets`, `🎯️goals`,
`🏃️test-runner`, `🏠️workspace`, `📚️library`, `📜️statutes`, `📝️todos`, `📡️events`, `🔌️mcp`, `🔎️search`,
`🔗️graphql`, `🔩️native`, `🖥️server`, `🗣️languages`, `🧑️contributors`, `🧩️providers`, `🧾️yaml`,
`🪝️hooks`, `🪪️identity`. Each is short and emoji-headed: what the module owns, its packages with the
Go module path and Rust crate name, and its test cases with the oracle situation.

Still missing on purpose: `📊️metrics`, `📐️model`, `🗂️codebase`, `🚚️move` — the concurrent agent's
modules. (`🧩️vscode`, `🧪️test`, `🪶️sqlite` already had one.) No `AGENTS.md` was touched.

### Coordinator durability

**Decision: leave it inline.** Plan §2 describes coordinator durability under a `🛡️durability/`
subdirectory; the Rust crate implements it in the single `🦀️.rs` godfile. The godfile convention
(`📋️plan.md` §3: one `🦀️.rs` per crate, regions inside it) is the binding rule, and a subdirectory
would be the deviation, not the reverse — `🎛️dashboard` is the only module that splits across
sibling files, and it does so because it carries several independent surfaces. Coordinator durability
is one region of one crate. The plan prose is the thing that is out of date.

## Verification summary

| Check | Result |
| --- | --- |
| `go build` / `go vet` / `gofmt -l` over every `go.work` member | 27/27 clean |
| `go test ./... -count=1` for `github.com/usalu/semio/repo/mcp` | ok |
| `printf '<burst>' \| semio-repo-mcp.exe` | both request ids answered |
| `parity fundamental --owner 🔌️mcp` | 21/21, parity 21/21 |
| `parity quick --owner 🔌️mcp` | 30/30, parity 30/30 |
| `parity long --case 🌿️git-version-control` | 12/12, parity 12/12 |
| `parity fundamental --case 🖥️host-protocol-parity` (incl. dotnet) | 10/10, parity 20/20 |
| `cargo test -p semio-framework-repo-dashboard daemon::tests` | 3/3 |
| `cargo clippy --no-deps -p …-dashboard --all-targets` | 0 crate warnings |
| `contract` — "does not declare capability" | 0 |
| `contract` — `🎛️dashboard` / `🔌️mcp` breaches | 0 / 0 |

## Not done / left for the next pass

1. **D3** — no standalone `tree` CLI verb. Deferred to after `final-fixes` releases `⌨️cli`.
2. **`📜️statutes/🗜️breach-cache-envelope` Go adapter** — see D7 above.
3. **`🗂️codebase/🚶️workspace-walk/🥒️.feature`** — 7 new Gherkin continuation-line breaches, same fix
   as D6, in a module owned by the concurrent agent this window.
4. **`README.md` for `📊️metrics`, `📐️model`, `🗂️codebase`, `🚚️move`** — same reason.
5. **`serde` workspace pin for `📐️model` and `⌨️cli`** — the `final-fixes` agent's crates.
6. **Pipelined-burst dispatch order** — a request that follows `initialize` in the same burst can be
   picked up by a second worker before `initialize` promotes the session and gets
   `-32002 session not initialized`. Present in BOTH implementations by the same worker-pool
   construction; out of scope for D1. See the note under D1's evidence.
7. `🔌️mcp/📦️packages/🐹️go/mcp.exe` and `⌨️cli/📦️packages/🐹️go/semio-repo.exe` are stale untracked
   build outputs left in the tree by earlier runs; the mcp one is now meaningless (that directory is a
   library). Neither is tracked by git, so neither was removed here.
