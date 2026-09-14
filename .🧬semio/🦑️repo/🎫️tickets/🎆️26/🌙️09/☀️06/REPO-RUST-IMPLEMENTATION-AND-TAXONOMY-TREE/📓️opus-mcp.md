# 📓️ Opus `mcp` — `🔨️modules/🔌️mcp`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp` (new).
Evidence: `$TICKET/🗑️generated/mcp/*.txt`.

## 1. What exists now

```
🔨️modules/🔌️mcp/
├── 🧬️schema/🔣️.json                 JSON Schema 2020-12: wire types, error codes, tool argument schemas,
│                                    the profile table, the event record, the limits, and the OPEN/CLOSED rule
├── 🧬️schema/🔣️descriptions.json     the 38-key × 6-profile description table (extracted from the Go snapshot's
│                                    `mcpDescriptionTable`, plus three new `tool_goal_*` keys)
├── 🧫️fixtures/                      2️⃣g2-contract.json, 🚪️entrypoint-contract.json (moved),
│                                    🤝️initialize-lenient.json, 📋️surface.json, 🔗️event-chain.json (new)
├── 🔮️oracle/🔣️.json                 the owner's v2 oracle registry (two oracles, see §4)
├── 🔗️graphql/                       moved from 💻️client/🔌️mcp/🔗️graphql
├── 🧪️tests/{🤝️jsonrpc-handshake, 📋️capability-listing, 📞️tool-call-roundtrip, 🔗️event-log-chain}/
│                                    🥒️.feature + 🐹️.go/🦀️.rs subjects + 🟦️.ts oracle
└── 📦️packages/
    ├── 🐹️go/    go.mod (`github.com/usalu/semio/repo/mcp`, go 1.25), 🐹️.go (2069 lines, `package main`),
    │            🧪️_test.go (1374), 📋️project.json (`@semio-tech/repo-mcp-go`), 📜️script.ts
    └── 🦀️rust/  Cargo.toml (`semio-framework-repo-mcp`), 🦀️.rs (2247, lib), 📦️main.rs (bin
                 `semio-repo-mcp`), 📋️project.json (`@semio-tech/repo-mcp-rs`), 📜️script.ts
```

`🔨️modules/💻️client/🔌️mcp/` is gone, including its `📦️packages/🐹️go` wrapper (which held two empty files).

### Go godfile regions

`📜️Protocol`, `📡️Event`, `🚚️Transport`, `🔐️Session`, `🚦️Routing`, `🗄️Repository`, `🦀️Entrypoint` — assembled by
`$TICKET/🏗️assemble-go-mcp.ts` from the six original files, then hand-edited for the four behaviour changes
below. `package main` and `main()` are kept, so `go build -o <path> .` yields `semio-repo-mcp`.

### The `initialize` defect is fixed

`DecodeParams` no longer disallows unknown fields. Three decoders now exist and the rule is written into
`🧬️schema/🔣️.json`'s `$comment`:

| decoder | strict? | owns |
| --- | --- | --- |
| `decodeExact` | yes | the JSON-RPC envelope, the event log, the fixture files |
| `DecodeParams` (`decodeOpen`) | **no** | every MCP `params` object, `initialize` included |
| `DecodeArguments` | yes | `tools/call` `arguments` only |

Proof — the exact payload Claude Code sends (`clientInfo.title`, `capabilities.tasks`, `capabilities.elicitation`)
against the built Go binary, `🗑️generated/mcp/06-stdio-probe.txt`:

```
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-06-18","capabilities":{"prompts":{},"resources":{},
"tools":{}},"serverInfo":{"name":"repo-kiro","version":"1.0.0"},"instructions":"Use repository tools and
resources through their owned schemas."}}
```

The envelope stays closed: `{"jsonrpc":"2.0","id":4,"method":"ping","unexpected":true}` is still
`-32600 invalid request` (g2 golden vector, asserted byte-for-byte by both implementations).

### Nine tools, not six

`goal_open`, `goal_close`, `goal_reopen` were added to **both** implementations with the argument shape of the
Go CLI goal commands (`ToolGoalCreate` / `ToolGoalClose` / `ToolGoalReopen`, `🧩️component.go:26373/26431/26452`
and the `goalOpen`/`goalClose`/`goalReopen` MCP request handlers at `:35192/:35209/:35226`):

| tool | required | optional |
| --- | --- | --- |
| `goal_open` | `title`, `prompt` | `description`, `due_date`, `llm`, `client`, `parent`, `milestone`, `no_management` |
| `goal_close` | `id`, `summary` | `no_management` |
| `goal_reopen` | `id`, `prompt`, `llm`, `client` | `title`, `description`, `due_date`, `no_management` |

Tool descriptions, resource descriptions and prompt descriptions are now read from
`🧬️schema/🔣️descriptions.json` instead of being hard-coded — Go resolves it at first use through
`runtime.Caller`-relative path resolution (with a `SEMIO_REPO_MCP_DESCRIPTIONS` override and a walk-up
fallback, because `go:embed` cannot cross a parent directory), Rust with
`include_str!("../../🧬️schema/🔣️descriptions.json")`.

### Rust crate

`semio-framework-repo-mcp`, `serde`/`serde_json` (`.workspace = true`) and `std` only. Regions:
`🔏️Digest` (hand-rolled SHA-256), `🧾️Json` (a Go-`encoding/json`-compatible ordered writer: field order,
`omitempty`, HTML escaping of `<`/`>`/`&`, ` `/` `, whitespace-stripping compaction),
`📜️Protocol`, `📡️Event`, `🔐️Session`, `🚦️Routing`, `🗄️Repository`, `🦀️Entrypoint`, `🧪️Tests`.

Ported behaviour: JSON-RPC 2.0 types; all twelve error codes; version negotiation (`2025-11-25` preferred,
four older accepted and echoed); line-delimited stdio transport with a bounded worker pool
(`-32005` when the queue is full); session phases; request-id dedup and pre-cancellation
(`-32003`/`-32800`); `commitExchange`; progress notifications; the hash-chained `semio.mcp.event/1` JSONL log
with `replay_events` chain validation; `initialize`, `notifications/initialized`, `notifications/cancelled`,
`ping`, `tools/list`, `tools/call`, `resources/list`, `resources/read`, `resources/templates/list`,
`prompts/list`, `prompts/get`; the six profiles with their server names and `plan_id`/`spec_id` differences.

## 2. Verification (real output)

| command | result |
| --- | --- |
| `go build -o .🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp.exe ./…/🔌️mcp/📦️packages/🐹️go` | ok |
| `go test ./...` in `📦️packages/🐹️go` | `ok github.com/usalu/semio/repo/mcp 1.096s` (all pre-existing g2 tests plus two new golden tests) |
| `cargo test -p semio-framework-repo-mcp` (`RUSTC_WRAPPER=""`) | `test result: ok. 11 passed; 0 failed` |
| `bun …/🧪️test/📜️script.ts oracle quick --owner …/🔌️mcp` | `cases=4 executed=8 passed=8 failed=0 errored=0` |
| `… subject quick --owner …/🔌️mcp --implementation rust` | `cases=4 executed=8 passed=8 failed=0 errored=0` |
| `… subject quick --owner …/🔌️mcp --implementation go` | `cases=4 executed=4 passed=4 failed=0 errored=0 not-exercised=2` (see §5) |
| `… parity quick --owner …/🔌️mcp` | **`cases=4 executed=20 passed=20 failed=0 errored=0 parity=16/16`** |
| `bun nx run @semio-tech/repo-mcp-go:test` | `Successfully ran target test` |
| `bun ./📜️script.ts dev mcp stdio client` (root script) | builds into the new cache path and answers on stdio |

Every one of the 20 harness results, `🗑️generated/mcp/05-harness-results.txt`:

```
passed typescript oracle  📋️capability-listing   generic-profile-surface / ide-profile-surface
passed rust      subject  📋️capability-listing   generic-profile-surface / ide-profile-surface
passed go        subject  📋️capability-listing   generic-profile-surface / ide-profile-surface
passed typescript oracle  📞️tool-call-roundtrip  tool-resource-prompt-roundtrip / protocol-error-vectors
passed rust      subject  📞️tool-call-roundtrip  tool-resource-prompt-roundtrip / protocol-error-vectors
passed typescript oracle  🔗️event-log-chain      chain-digests-match-the-golden / a-tampered-chain-is-refused
passed rust      subject  🔗️event-log-chain      chain-digests-match-the-golden / a-tampered-chain-is-refused
passed typescript oracle  🤝️jsonrpc-handshake    initialize-accepts-unknown-members / initialize-echoes-a-supported-version
passed rust      subject  🤝️jsonrpc-handshake    initialize-accepts-unknown-members / initialize-echoes-a-supported-version
passed go        subject  🤝️jsonrpc-handshake    initialize-accepts-unknown-members / initialize-echoes-a-supported-version
```

### Go and Rust are byte-identical on the wire

`🗑️generated/mcp/07-go-rust-listing-parity.txt` — both binaries driven over real stdio, `initialize` +
`tools/list` + `resources/list` + `prompts/list` + `resources/templates/list` + `ping`, per profile, compared
as raw canonical JSON:

```
identical client  (8044 bytes of canonical listing JSON)
identical cursor  (8750 bytes)
identical kiro    (8694 bytes)
identical copilot (8801 bytes)
identical claude  (8625 bytes)
identical codex   (8520 bytes)
```

The Rust crate additionally replays the g2 golden vectors **byte-for-byte** against the Go-authored
`2️⃣g2-contract.json` (`tests::the_g2_golden_vectors_match_byte_for_byte`), which is what forced the
Go-compatible JSON writer rather than plain `serde_json` output.

## 3. The `Repository` trait the `⌨️cli` agent must implement

`semio_framework_repo_mcp::Repository` (mirror of the Go `RepositoryHandlers`). It is `Send + Sync`; the
server takes an `Arc<dyn Repository>`.

```rust
pub trait Repository: Send + Sync {
    fn call(&self, context: &Context, name: &str, arguments: &serde_json::Value) -> Result<RepositoryResult, HandlerError>;
    fn read(&self, context: &Context, uri: &str) -> Result<ResourceContent, HandlerError>;
    fn prompt(&self, context: &Context, name: &str, arguments: &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError>;
}
```

* `call` receives one of `TOOL_NAMES` (9) and the raw `arguments` object; use `decode_arguments(&arguments, &[…])`
  to keep the closed-argument rule, and return `HandlerError { code: -32010, … }` (`HandlerError::tool`) for a
  domain failure. `RepositoryResult { text, structured: Option<String>, is_error }` mirrors the Go
  `repositoryResult(client.ToolResult)` shaping (joined output lines, then the error string, `is_error` when
  the error is non-empty or the exit code is non-zero).
* `read` receives one of `RESOURCE_URIS` (8) and returns one `ResourceContent` (`mime_type` `"text/plain"`).
* `prompt` receives one of `PROMPT_NAMES` (4); `prompt_instruction(name)` returns the fixed instruction string
  the Go implementation prefixes.
* `Context::is_cancelled()` is the cancellation check; long operations must poll it.

Entry points already provided:

```rust
pub fn repository_server(repository: Arc<dyn Repository>, profile: Profile, limits: Limits) -> Result<Server, Error>;
pub fn serve_stdio(repository: impl Repository + 'static, profile: Profile) -> Result<(), Error>;
pub fn run_with(argv: &[String], repository: impl Repository + 'static) -> i32;
pub fn run(argv: &[String]) -> i32;   // used by 📦️main.rs today
```

`run` currently wires `UnwiredRepository`, whose `call`/`read` return
`-32010 "repository domain is not wired into this binary"` (prompts already answer, they are pure text).
**The `⌨️cli` agent's job is to replace that one line**: either call `run_with(argv, RealRepository)` from a
`semio mcp` verb, or repoint `📦️main.rs`. `RecordingRepository` is the in-memory test double.

Also exported for reuse: `Profile` (`parse`/`kind`/`server_name`), `describe(profile, key)`, `descriptions()`,
`tool_schemas`/`resource_schemas`/`prompt_schemas`, `contract_server(limits)` (the synthetic g2 server),
`sha256`/`sha256_hex`, `compact`, `EventLog`/`replay_events`, `PROFILE_ENVIRONMENT`.

## 4. Oracles

`🔌️mcp/🔮️oracle/🔣️.json` (the harness discovers the manifest at `<owner>/🔮️oracle/🔣️.json`, **not** at
`<owner>/🔣️oracle.json` as the plan text suggested):

| id | kind | what it is |
| --- | --- | --- |
| `modelcontextprotocol-sdk` | `third-party-library` | Anthropic's TypeScript SDK 1.30.0. The oracle adapters run its `Server` over its own `InMemoryTransport` and speak raw JSON-RPC to it. It never links this repository's code — the expected handshake, listing and error codes come from the specification authors' own implementation. |
| `node-crypto-sha256` | `standards-reference-tool` | Node `crypto` SHA-256, the FIPS 180-4 reference for the hand-rolled digest in both implementations. |

`@modelcontextprotocol/sdk@1.30.0` was **already installed** in the root `node_modules` (declared by
`💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript`), so no `bun install` was needed and no private copy was
added — that is the harness's documented "one lockfile" model.

## 5. What is deliberately not covered, and why

1. **No Go adapter for `📞️tool-call-roundtrip` and `🔗️event-log-chain`.** The Go implementation is
   `package main`, so no external Go adapter can import it, and `materializeGoHost` has no `rustSutCrate`
   equivalent (recorded as a gap in `📓️harness-verification.md` §2). The two cases the Go adapter *can*
   serve go through the real `semio-repo-mcp` binary over stdio — a stronger check than an in-process call,
   but it only reaches surfaces the production server exposes, which the synthetic g2 `echo` server and the
   raw event log are not. Both are instead asserted inside `go test`:
   `TestG2CanonicalGoldenVectors` (byte-exact, pre-existing) and
   `TestGoldenEventChainMatchesTheLanguageAgnosticFixture` (new — commits the same
   `🔗️event-chain.json` inputs and compares all seven digests and the whole JSONL against the
   Node-`crypto`-generated golden, then checks the tampered chain is refused).
   `TestAdvertisedSurfaceMatchesTheAuthoredTable` (new) pins the whole 9/8/4 surface and every description
   against `📋️surface.json` + `🔣️descriptions.json` for all six profiles.
2. **The `invalid-params` vector is not in the differential.** JSON-RPC 2.0 requires `-32602` for a params
   object that fails the method's own shape; both implementations return it and the byte-exact g2 vector pins
   it. The reference SDK answers `-32603` there, so comparing that vector would encode the reference's
   deviation as this repository's contract. This is written into the feature file as a comment.
3. **`.mcp.json` and the IDE `mcp.json` files were not touched** — that is the `wiring` agent's step.
   They still call `bun ./📜️script.ts dev mcp stdio <profile>`, which now builds and runs the moved Go
   package, so the repo MCP server should connect again as soon as an IDE restarts it.

## 6. References repointed

| file | change |
| --- | --- |
| `go.work` | `💻️client/🔌️mcp` → `🔌️mcp/📦️packages/🐹️go` |
| `Cargo.toml` | added `…/🔌️mcp/📦️packages/🦀️rust` to `members` |
| root `📜️script.ts` | `REPO_MCP_GO` repointed; new `buildRepoMcpBinPath` creates the cache directory before every `go build -o` |
| `📚️library/…/🟦️.ts` | `defaultMcpBin` → `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp[.exe]` (build output belongs in the marked cache, not in the tree; the tracked `💻️client/mcp.exe` was deleted) |
| `🔩️native/🥾️bootstrap/🐚️.sh`, `🔵️.ps1`, `.devcontainer/post-create.sh` | these three built the **MCP** package and named it `client[.exe]` (the mismatch flagged in `📓️explore-go-mcp-server-library.md` §6). Now they build the CLI (`💻️client/⌨️cli/cmd/repo`) as `client[.exe]` **and** the MCP package as `semio-repo-mcp` into the cache |
| `.vscode/🧩️launch.seed.jsonc` | inspector `serverArgs` repointed; `🛠️dev🧰️repo⌨️client` now builds the CLI; new `🛠️dev🧰️repo🔌️mcp🦀️rust` (order 280.3), `🛠️dev🧰️repo🔌️mcp🐹️go` (280.4), `🧪️test🧰️repo🔌️mcp🦀️rust`, `🧪️test🧰️repo🔌️mcp🐹️go`. Regenerated with `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate` |
| `📚️library/🔣️taxonomy.json` | the `pathPattern`/`path` entry for the old `go.mod` |
| `🔌️mcp/🔗️graphql/📋️project.json` | `$schema` depth corrected for the shallower path |

`grep -rn "💻️client/🔌️mcp"` over `*.ts *.sh *.ps1 *.mjs *.toml *.json *.jsonc` (excluding `node_modules`,
`.nx/`, ticket folders) now returns nothing outside historical ticket snapshots.

## 7. Findings for the coordinator / audit

1. **`testCaseSlugPattern` contradicts the actual convention.** `🔣️taxonomy.json` enforces
   `^[a-z0-9]+(?:-[a-z0-9]+)*$` on the case directory name, so **every** emoji-prefixed case in the repo is a
   `testing/taxonomy case-slug` breach — including the reference example
   `🧪️test/🧪️tests/🖥️host-protocol-parity` and `🔎️search/🧪️tests/🔍️ranked-search`. My four cases keep the
   emoji convention (and the names this ticket specified). The one-line fix is to allow an optional leading
   emoji identity in that pattern; doing it unilaterally would have changed a shared taxonomy file.
2. **`bun ./📜️script.ts test` for any Rust package is broken repo-wide**, unrelated to this module:
   `runCargoTestBudgeted` → `loadTaxonomy()` throws
   `generatorContracts["wgpu-frame-worker"] tracked output "…/🎞️frame-worker/🤖️generated/🟨️.js" is missing`.
   The pre-existing `@semio-tech/repo-cli-rs:test` target fails identically. `cargo test -p …` works.
3. **`bun …/🧪️test/📜️script.ts dependency` exits non-zero** on dozens of unrelated ratchet entries
   (`vscode-*`, `rust:anyhow`, …). My two oracles classify correctly (`test-oracle js:node:crypto@builtin
   (node-crypto,node-crypto-sha256)`). One thing to watch: `@modelcontextprotocol/sdk` is in
   `🔒️dependencies.json` as `production-runtime` / `productionReachable: true` because
   `💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript` declares it as a `dependency` rather than a
   `devDependency`; my registry entry declares it `testOnly`. The current `dependency` phase does not flag
   the disagreement, but a stricter purity pass would.
4. **A pipelining race exists in both implementations** (inherited from the Go design, not introduced here):
   `notifications/initialized` is handled inline on the reader thread while `initialize` is dispatched on a
   worker, so a client that sends both without waiting for the initialize response can leave the session in
   the `Connected` phase and get `-32002` for everything after. Real MCP clients wait for the response, so
   this is latent; a fix would have to land in both implementations together.
5. `📦️packages/🟦️typescript` must **not** exist under this module: `ownerShipsImplementation` reads the
   presence of `📦️packages/<language>` as "the owner ships a subject in that language" and would dispatch the
   oracle-only TypeScript adapter as a subject (8 `adapter has no subject registration` errors). The oracle's
   npm package is declared in `🔮️oracle/🔣️.json`'s `oracleHostPackages` and resolved from the root
   `node_modules` instead. Any later agent adding a TypeScript package here must also add TypeScript
   `subject` registrations.
