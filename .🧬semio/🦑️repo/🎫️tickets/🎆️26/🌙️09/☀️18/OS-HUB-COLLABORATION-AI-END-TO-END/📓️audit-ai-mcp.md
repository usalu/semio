# 📓️ Audit — "AI integration over MCP"

Read-only audit. All commands run from `/Users/ueli/Documents/semio`. Raw captures under
`🗑️generated/mcp-repo-client-raw.txt`, `🗑️generated/mcp-os-raw.txt`, `🗑️generated/mcp-repo-test-quick.txt`.

**Headline**: both `.mcp.json` servers are currently broken, for two *different, independent* reasons, and
neither reason is "MCP itself is unfinished" — the underlying `semio-os-mcp` gateway is a large, real,
previously-verified implementation (22 tools). What's broken is (a) a stale build-output/consumer path
mismatch for the `repo` server, and (b) an overly-broad anti-credential-leak guard in the `semio` (os)
server that trips on Claude Code's own harness environment variables. Separately, no LLM/model-provider is
wired anywhere in this repo — "AI integration" here means *exposing the OS to an external AI client* (Claude
Code, etc.) over MCP, not embedding a model call inside the app. The one "hub inference" pipeline that is
fully wired end-to-end is a native, non-LLM, domain-specific GIS computation.

---

## 1. Reproducing the failures

### 1a. `repo` server — `bun ./📜️script.ts dev mcp stdio client`

```
error: repo MCP client binary is missing at /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/mcp;
run: bun nx run @semio-tech/repo-mcp-go:build
    at requireRepoMcpBinary (/Users/ueli/Documents/semio/📜️script.ts:235:62)
```
Full capture: `🗑️generated/mcp-repo-client-raw.txt`.

**Root cause — a path mismatch between the build target and the consumer, not a missing build step:**

- `📜️script.ts:150-160` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:151-160`) — `resolveMcpBin()` /
  `defaultMcpBin()` expect the binary at the **flat** path
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/mcp` (sibling of `💻️client/client`, the CLI binary).
- `bun nx run @semio-tech/repo-mcp-go:build` (project at
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/📋️project.json:6-11`) instead runs
  `🔌️mcp/📦️packages/🐹️go/📜️script.ts`'s `BuildScript` (lines 20-24), which builds the Go source under
  `🔌️mcp/📦️packages/🐹️go/🚀️bin` into `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp` (line 11-14, 24) — a
  **different filename in a different directory**, never staged/copied/symlinked to the flat path above.
  Grepping the whole tree for `defaultMcpBin`/`💻️client/mcp` finds no staging step at all.
- Compounding this: there are **two separate Go modules both declaring `module github.com/usalu/semio/repo/mcp`**:
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🐹️.go` — a monolithic single-file
    implementation, **the one `go.work` actually lists** (`go.work:23`) and the one `@semio-tech/repo-mcp-go`
    builds.
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/{🖥️server,🗄️repository,🚚️transport,🧩️component,📜️protocol,📡️event}/🐹️.go`
    — a modular rewrite (this is where the actual current tool/resource/prompt surface lives, see §2), **not
    listed in `go.work`**, and self-contained only via its own `replace` directives in
    `💻️client/🔌️mcp/go.mod`.
  - Because both claim the same module path, you cannot simply `go work use ./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp`
    without first removing the older duplicate — this is a genuine module-path collision, not just a missing
    `go.work` entry.
- Running the project's own `test-quick` target reproduces a symptom of the same drift:
  ```
  # .
  current directory is contained in a module that is not one of the workspace modules listed in go.work.
  You can add the module to the workspace using: go work use .
  FAIL	. [setup failed]
  ```
  (full capture `🗑️generated/mcp-repo-test-quick.txt`) — the nx `test-quick` target for `repo-mcp`
  (`💻️client/🔌️mcp/📋️project.json:6-11`) `cd`s into `💻️client/🔌️mcp`, which is a `go.mod`-rooted module
  `go.work` never lists.
- **Historical context that explains the drift**: ticket
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE` (closed 2026-09-06)
  claims the whole repo product was restructured with Go+Rust twins under `🔨️modules/**`, that
  `🔨️modules/💻️client/**` was **removed**, and that its closing check confirmed both the Rust-default and
  `SEMIO_REPO_IMPLEMENTATION=go` paths answered `initialize`/`tools/list` with 9 identical tools
  (`file_integrate, goal_close, goal_open, goal_reopen, section_extract, section_move, ticket_close,
  ticket_open, ticket_reopen`). **None of that is true of the current tree**: `grep -n
  "SEMIO_REPO_IMPLEMENTATION\|semio-framework-repo-cli" 📜️script.ts` returns nothing, `💻️client/**` is
  very much present, and `git log -L` on the `REPO_CLIENT_DIR` line in `📜️script.ts` blames it to
  2026-08-04 — before that ticket even opened. Either the migration never actually landed on this branch or
  it was reverted afterward; the ticket's "closed" status and summary do not reflect the live tree (see §6).

### 1b. `semio` (os) server — `bun ./📜️script.ts dev mcp stdio os`

```
[semio-os-mcp] protected parent environment was not sealed
error: .../📦️packages/🦀️rust/dist/build/semio-os-mcp stdio exited with status 1
    at runMcpOs (/Users/ueli/Documents/semio/📜️script.ts:603:5)
```
Full capture: `🗑️generated/mcp-os-raw.txt`. **This is not a build problem** — the binary exists at
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`, is executable,
starts, and deliberately exits 1.

**Root cause — a credential-leak guard that is broader than intended and trips on Claude Code's own
environment**, `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`:
- `main()` (lines 141-157) calls `protected_credential_environment_is_absent()` (lines 104-118) before
  doing anything else and exits 1 with exactly the observed message if it returns `false`.
- That function scans **every** environment variable and fails if any key (other than
  `S_LOCAL_CREDENTIAL_FD=3`) contains `TOKEN`, `SESSION`, `CREDENTIAL`, `BEARER`, `CAPABILITY`,
  `AUTHORIZATION`, or `COOKIE` as a substring, or equals `S_USER`/`VITE_S_USER`/`S_HUB_URL` (lines 105-117).
  The intent (per the file's header comment, lines 8-11) is to stop a hub credential fd/env leaking into a
  spawned MCP process. It is not scoped to `S_`-prefixed names.
- Verified directly: `env | grep -iE "session|token|..."` in this very shell shows
  `CLAUDE_CODE_HOST_SESSION_ID`, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ID`,
  `CLAUDE_CODE_SESSION_ATTENDED` — none of these carry a hub credential, but each contains `SESSION` or
  `TOKEN`, so the guard fires unconditionally for any process Claude Code (or, generically, almost any
  session-oriented dev/CI harness) spawns as a child. This is very likely also why the parent Claude Code
  session that is auditing this ticket itself reports `repo`/`semio` MCP servers as `CONNECTION_CLOSED` —
  the identical mechanism applies to its own `.mcp.json`-launched child process.
- This is a real, load-bearing security check (see §3 "Safety") — not a bug to "just delete" — but as
  written it makes the `semio` MCP server **unusable as a child of Claude Code itself**, which is the
  primary intended client per `.mcp.json`. That is a self-defeating combination for "AI integration over
  MCP" specifically.

---

## 2. Server implementations — transport, tools, resources, prompts, schema-first-ness

### `repo` (`.mcp.json` → `bun ./📜️script.ts dev mcp stdio client`)

- **Language/transport**: hand-rolled Go, line-delimited JSON-RPC over stdio — no
  `@modelcontextprotocol/sdk` or any npm/Go MCP SDK dependency (`go.mod` files show only intra-repo
  `replace` targets). Consistent with the repo's "no runtime dependencies on external libraries" rule cited
  throughout (`AGENTS.md`, `CLAUDE.md`).
- **Current tool surface** (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗄️repository/🐹️.go:297-302`,
  the modular/newer implementation — see §1a for why this is *not* what's actually wired today):
  `ticket_open`, `ticket_close`, `ticket_reopen`, `section_move`, `file_integrate`, `section_extract` (6
  tools — no `goal_*` tools here, unlike the monolithic `🔌️mcp/📦️packages/🐹️go/🐹️.go` variant that `go.work`
  actually builds, and unlike the 9-tool set the 2026-09-06 ticket claims).
- **Resources**: `repo://`, `repo://bundles`, `repo://folders`, `repo://files`, `repo://tickets`,
  `repo://goals`, `repo://policies`, `repo://contributors` (lines 323-330); a `repo://ticket/{id}` resource
  *template* exists only in the test harness (`🧪️tests/🤝️protocol-contract/🐹️.go:509`), not in production
  registration.
- **Prompts**: `enhance`, `refactor`, `test`, `comply` (lines 347-355), each taking a required `prompt`
  argument, backed by `repository.Prompt(...)`.
- **Schema-first?** Partially — `🔌️mcp/🧬️schema/🔣️.json` + `🔌️mcp/🧬️schema/🔗️.graphql` exist as JSON/GraphQL
  schema artifacts, and `📦️packages/🟦️typescript` (project `@semio-tech/repo-mcp-schema`) mirrors it for
  TypeScript consumers, but tool `InputSchema`s themselves are still hand-built inline in Go
  (`object(openProperties, ...)` etc., `🗄️repository/🐹️.go:297-302`) rather than generated from one
  registry the way the `os` server is (below).
- **Protocol contract tests** exist and are substantial:
  `🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go` (600+ lines) exercises `initialize` →
  `notifications/initialized` → `resources/list` → `resources/read:repo://goals` → `tools/list` →
  `tools/call`, pagination (`tools/list` cursors), progress reporting, tool-limit enforcement, and
  concurrent dispatch — but this suite lives under the module `go.work` doesn't register (§1a), so it isn't
  exercised by the standard `nx test` path today.

### `semio` (os) (`.mcp.json` → `bun ./📜️script.ts dev mcp stdio os`)

- **Language/transport**: Rust, dual-era JSON-RPC (serves `2026-07-28` "modern" stateless, `2025-11-25` and
  `2025-06-18` "legacy" `initialize`-handshake eras from one handler — deliberately, see
  `🌉️mcp/README.md` "Dual-era protocol" section and design decision D1 in
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️design-decisions.md:5-9`).
  Also serves Streamable HTTP on port 6300 (`🚚️transport`, axum). No `@modelcontextprotocol/sdk` runtime
  dependency in the server itself — the *test* harness under `📦️packages/🟦️typescript` uses the installed
  SDK (1.30.0, legacy-only) as an independent third-party oracle only (`💡️inference-bridge/🟦️.ts` header
  comment, `🌉️mcp/🟦️.ts:1-7`).
- **Tool surface** — README (`🌉️mcp/README.md:27-35`) documents "twenty stable tools": discovery
  (`capabilities_search`, `capabilities_describe`, `context_resolve`), authoring (`action_prepare`,
  `action_invoke`, `action_cancel`, `transaction_begin|commit|rollback`), history (`history_undo`,
  `history_redo`), artifacts (`artifact_create|open|validate|export|snapshot`), jobs/UI (`job_get`,
  `job_cancel`, `ui_focus`, `ui_reveal`), plus 4 inference-job tools registered separately
  (`inference_submit`, `inference_events`, `inference_cancel`, `inference_approve` —
  `💡️inference/🦀️.rs:1135-1188`, also independently named in
  `💡️inference-bridge/🟦️.ts:9-10`'s `INFERENCE_JOB_TOOLS`). The closing summary of ticket
  `AI-MCP-END-TO-END` (§6) counts 22 (3 core + 8 mutation-protocol + 5 artifact + 2 inference + 4 ui/job).
  The "long tail" of plugin-specific capability is intentionally reached through the catalog
  (`capabilities_search`/`describe`), not by registering thousands of tools.
- **Resources**: `semio://…` URIs — workspace, artifact (+ schema/snapshot/selection/validation/history/diff),
  window, `ui/active-context`, capability, plugin, extension, transaction, job, audit
  (`🌉️mcp/README.md:37-38`).
- **Prompts**: bilingual (EN/DE) protocol-teaching prompts (`💬️prompts/🦀️.rs`), added per the
  `AI-MCP-END-TO-END` closing summary, replacing a previously-empty registry.
- **Schema-first — yes, genuinely**: `🌉️mcp/README.md` "Schemas" section: every wire type, protocol type,
  and tool `inputSchema`/`outputSchema` lives in `🧬️schema/🦀️.rs`'s `schemas()` function — the single
  source of truth. `🧬️schema/🔣️.json` (draft-07) and `🧬️schema/🟦️.ts` are **generated** from that registry
  via `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror` (which runs the binary's own `semio-os-mcp
  schemas` subcommand — `🏗️bootstrap/🦀️.rs:145-149`), and a `schema-mirror-check` / Rust law
  `the_json_mirror_publishes_exactly_the_registry_exports` gates drift. Schema enforcement is centralized at
  `ToolRegistry::register` (normalizes boolean sub-schemas, upgrades draft-07 to 2020-12) because "the
  official SDK's validation rejects both" otherwise (README "Layout" table).

---

## 3. `semio` (os) MCP for AI integration — can an LLM actually drive the OS?

**Yes, by design, through the real action/mutation system — not a side channel.** Per the README: "It is
not a second application runtime: it never touches a store or a React state directly, and every write it
performs travels the ordinary action → mutation → VCS → backbone path, so a live shell sees an agent's edit
exactly as it sees a human collaborator's." Mutation protocol is `Observe → Prepare → Preview → Approve →
Commit → Verify` over `PureCommand`/two-phase `TransactionPrepare`/`TransactionCommit` frames, with
`expectedRevision` conflict detection, idempotency keys, and `undoToken` → `TransactionUndo{group_id}`
(`README.md` "Mutation protocol"). The `AI-MCP-END-TO-END` ticket's closing summary (§6) describes exactly
this being wired for real: real `artifact_*` tools against a live workspace, real mutation round-trips
through `PluginArtifactChannel` (replacing blanket `channel.not-wired` rejections), UI information/actions
over the existing shell bridge (`ui_focus`, `ui_reveal`, `job_get`/`job_cancel`), and three progressive
tiers (bare / headless `--folder`|`--hub` / attached to a live shell over `/bridge`) so tool *presence* in
`tools/list` never depends on whether a shell is attached — only results do.

**Safety model** (README "Safety"): the agent is an ordinary OS principal (never admin); its scopes map to
kernel `Broker` `CapabilityId`s; over-scoped calls get `PERMISSION_DENIED` + an audit row; destructive
capabilities require approval (MCP elicitation, or a parked approvals-dialog request); `ui.raw.*` is a
separate privileged scope; plugin-authored text is untrusted (can influence search ranking, never policy).

### Hub inference path — real, but not an LLM call

`🌎️hub/💡️inference/` is a full production pipeline — `✉️command` (bounded canonical protocol bytes),
`🏃️runtime` (owner-private proposal runtime: frozen binding, ledger, per-document gate, typed approval),
`🛂️authorization` (rechecks live Author membership — `🛂️authorization/🦀️.rs:10-16`), `🧾️wal` (fenced
document scope + cancellation cleanup), `🪶️sqlite` (durable idempotent job ledger,
`state IN ('accepted','running','succeeded','failed','cancelled')`), `🧬️schema` (schema-authoritative,
mirrored to JSON+TS). **But it is bound to exactly one native, non-LLM service**:
`🌎️hub/💡️inference/📇️catalog/🦀️.rs:22` — `GIS_MAP_NATIVE_EXECUTABLE = "semio_s_plugin_gis::gis_map_inference_service"`
— a first-party geometry/plugin computation (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`), not a call to
any external model provider. `grep -rn "anthropic|openai|gemini|model_provider|api_key" 🌎️hub/💡️inference`
returns **zero hits**. No credentials for any model provider exist anywhere in the hub.

The os MCP's own `💡️inference/🦀️.rs` module documents this gap explicitly in its header comment
(lines 1-20): "an inference cannot be EXECUTED through this crate today" for the general case — declared
inference metadata (`DeclaredInference`, from each plugin's own `🔣️.json`
`contributions.inference_services`) can be *listed/described* (`inference_list`, `inference_get`), but
actually *running* one goes through a plugin's own wasm-guest reactor (`job_infer` /
`ArtifactInferenceRouter`, owned by the separate `run` process), which this crate's `HeadlessWorkspace`
channel does not expose (`channel.not-wired`, `💡️inference/🦀️.rs:159`). The **one exception** is exactly
the GIS Map service: `gis_map_hub_inference_read`/`submit_gis_map_job` (`💡️inference/🦀️.rs:906,1440`) is
"the hub-backed replacement for `channel.not-wired` on the ONE inference service this gateway [handles]" —
i.e. one specific domain algorithm was wired end-to-end as a proof of the pattern, not general LLM
inference.

### Model providers and credentials

**None are wired.** A repo-wide, non-vendor-dir grep for `anthropic`/`openai`/`claude` turns up only: (a)
`CLAUDE.md`/`AGENTS.md` as *this repo's own agent-instructions files*, cited constantly in code comments as
a style/architecture authority (e.g. "CLAUDE.md forbids CRDTs", "no runtime dependencies on external
libraries"); (b) `"client": "claude-code"` / `"llm": "opus-5"` as **ticket-authorship metadata** (which AI
coding tool/model opened a ticket) in fixtures like
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧫️fixtures/✉️payload-vectors.json:43`; (c)
`README.md:651,667` documenting that the team uses Claude Code (Pro plan) as a *developer tool*. There is
no `openai` hit at all. Credential handling for any model provider is a non-topic in this codebase — by
explicit policy (CLAUDE.md "no runtime dependencies on external libraries"), the pattern here is "expose the
OS as a controllable substrate to whichever external AI client the developer runs (Claude Code, Codex,
etc.) via MCP", not "the app calls out to a model itself."

---

## 4. In-OS AI surface — chat pane / reasoning plugin / playbook plugin

- **`AgentChatPanel`** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx:16-27`)
  is real UI, docked right-middle in the shell, toggled from the navbar (`ui.panelToggle.chat`,
  per ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/CHAT-MCP-NAVBAR-PANEL/📓️implementation.md`). It renders
  an `AgentPresence` header (`🚦️AgentPresence`, driven by the `GatewayToShell::AgentPresence{active,
  label, invocation_id}` bridge frame that the `semio` MCP server's `🧵️bridge` module actually emits when a
  live MCP client — labelled e.g. `"claude-code"` — is connected) plus a `BasicChatPanel` body.
- **`BasicChatPanel` is a client-side mock, not a real chat**:
  `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:3979-4046` — comment: "Shared side-panel chat UI with
  **local-only message storage**." `sendDraft()` (lines 4030-4039) literally echoes the user's own message
  back, truncated, via `savedLocally(responsePreview)` — no network call, no MCP tool invocation, no model.
  So today's in-shell "chat" panel shows *whether* an external MCP agent is present, but does not let a
  human converse with any model through the OS, and does not itself drive MCP tools.
- **`💡️reasoning` plugin** (`✏️s/🔌️plugins/💡️reasoning/AGENTS.md`): "Structured reasoning over graphs,
  mindmaps, and specialized notations" — a domain-specific mindmap/graph-notation *editor* app (with a
  `🔌️wires` node-diagram artifact), not an LLM reasoning/agent surface. Ticket history confirms this
  (`REASONING-PLUGIN-SHAPE-V2-TREE-PURITY-RETROFIT`, `NORMALIZE-REASONING-MINDMAP-PLUGIN-TO-CONSISTENCY-CONTRACT`,
  `FIX-REASONING-WIRES-REPLACE-DOCUMENT-OP-TEXT-ORDERING-BUG` — all about the mindmap/graph editor's data
  model, none about model inference).
- **`📖️playbook` plugin** (`✏️s/🔌️plugins/📖️playbook/AGENTS.md`): actually named/described internally as
  "Protocol" — "A Blockly-like visual editor for generating code/data: a strict, ordered list of steps
  containing typed blocks, module-extensible via contributed block kinds." Also not an LLM/agent surface;
  it's a procedural step-builder UI reused by `forms`'s "Blueprint" mode.
- **Conclusion for Q4**: there is exactly one real "AI" surface in the shell today — the MCP `AgentPresence`
  indicator plus a decorative, non-networked chat mock. Neither `reasoning` nor `playbook` is an agent/LLM
  feature despite their evocative names. None of the three connects to the hub inference pipeline; only the
  chat panel's presence header connects (read-only) to the MCP bridge.

---

## 5. Tests covering MCP/JSON-RPC

- **Repo (`repo-mcp`)**: rich Go suite at `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go`
  (initialize/handshake, resources, tools/list pagination, tools/call, progress, concurrency, tool-count
  limits). **State: cannot currently run via the standard `nx test-quick` entrypoint** — reproduced above
  (§1a): `go.work` doesn't list the module the test lives in, so `go test` fails at setup
  ("current directory is contained in a module that is not one of the workspace modules listed in go.work").
  The *other*, `go.work`-registered `@semio-tech/repo-mcp-go` project has its own `test-quick` target
  (`🔌️mcp/📦️packages/🐹️go/📋️project.json:15-19`) that presumably does run, but it tests the stale monolithic
  implementation, not the tool surface actually intended to ship (§1a/§2).
- **OS (`semio-os-mcp`)**: extensive Rust test tree — `🔬️quick` suites per facet (`⚠️errors`, `🎫️handles`,
  `🏠️workspace`, `💡️inference`, `💬️prompts`, `📇️registry`, `📒️audit`, `🔀️dispatch`, `🔎️search`, `🖥️ui`,
  `🗂️catalog`, `🗿️artifact`, `🚚️transport`, `🛡️policy`, `🧠️context`, `🧪️conformance`, `🧬️schema`,
  `🧭️protocol`, `🧵️bridge`), plus top-level `🧪️tests/{🌅️modern-era, 🏛️legacy-conformance,
  💡️inference-bridge, 🔄️end-to-end, 🔐️authenticated-hub-workspace, 🔬️bin-quick, 🧱️source-builders,
  🧹️hygiene}` and a TypeScript conformance harness under `📦️packages/🟦️typescript` driving the *real
  compiled binary* over stdio with the installed `@modelcontextprotocol/sdk` as an independent oracle. Per
  ticket `AI-MCP-END-TO-END`'s closing summary (§6), the last verified run was vitest 33/33 (12 new e2e
  tests) plus Rust `workspace::quick 24/24`, `artifact::quick 9/9`, `bridge 40/40` — **not independently
  re-run in this audit** (a full workspace `cargo test`/`cargo build` was out of scope for the time budget
  and this audit's read-only/no-build mandate); note the closing summary itself flags the full 284-test lib
  suite as never having completed in one process due to a concurrent framework-wide migration. Separately,
  none of that suite exercises the *failure this audit reproduced* (§1b) — the credential-seal guard runs
  in `main()` before any library code, so it's a black-box CLI-boundary behavior orthogonal to `cargo test`.
- I did not attempt to build/run the Rust suite live (would need a `cargo build`, budget-risky per project
  norms — 20-minute cargo budgets are a known constraint here) or the `@semio-tech/repo-mcp-go` project's
  own `test-quick` (would build/run the *other*, non-canonical Go module). The one test invocation I did run
  (`bun ./📜️script.ts test quick repo-mcp`) is captured in full in `🗑️generated/mcp-repo-test-quick.txt`.

---

## 6. Known gaps from tickets (26/07–26/09, MCP/AI/inference/reasoning/agent)

- **`26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`** — status **open**. This is the *plan-of-record*
  ticket for the whole `semio-os-mcp` build-out (Claude Opus 5-coordinated). Its `📓️design-decisions.md`
  records D1 (dual-era protocol, binding, see §2) through at least D8; W0/W1 waves landed
  `GatewayBackend`/`NullBackend`, a 63-variant `ShellCommand` + pure `reduce` + TS twin with 75 shared
  fixtures, Streamable HTTP + handle table + audit lane + shell-bridge codec, `ArgSchema`/`ActionSemantics`
  manifest regions, and the first independent MCP-SDK-client conformance pass (26/26). Still open as the
  umbrella.
- **`26/08/29/AI-MCP-END-TO-END`** — status **closed**. Most detailed and most directly relevant ticket.
  Baseline audit at open time found the gateway **11 real / 9 hard-stubbed** tools, a hardcoded note+cad
  fixture catalog, artifact channel hardcoded to the `note` plugin, only 3 resource URIs actually reachable,
  every mutation returning `channel.not-wired`, zero inference access, zero prompts, one fixed session id.
  Closing summary claims all of that fixed: 22 real tools (zero stubs), plugin-independent catalog compiled
  from the real registry, real `PluginArtifactChannel` mutations, real `artifact_*` tools, inference
  list/get + `semio://artifact/{id}/inference[/{field}]`, UI info/actions via the bridge, 3-tier progressive
  enhancement, bilingual prompts, and two real production bugs fixed along the way (workspace store drain
  panic; single-plugin-only channel resolver). Explicitly **not verified**: the full 284-test lib suite in
  one process (blocked by a concurrent serde-removal migration elsewhere), the transport half of a heap-ring
  fix, and `DEFAULT_SESSION_ID` remains process-fixed (deliberately deferred). This audit's own
  reproduction (§1b) shows a *new*, later-introduced blocker (the credential-seal guard) that this ticket's
  scope never covered — i.e. the gateway got built out functionally, then a security hardening pass made it
  refuse to start under Claude Code's own environment.
- **`26/09/06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE`** — status **closed**, but its summary
  (Rust-default repo binary, `💻️client/**` removed, 9-tool parity across Go/Rust) does not match the live
  tree (§1a). Its own `📓️baseline-and-mcp-initialize-defect.md` additionally documents a **separate,
  already-fixed-once** MCP defect worth knowing about if it recurs: a strict `decodeExact` on `initialize`
  params rejected modern clients' extra fields (`clientInfo.title`, `roots.listChanged`, etc.) with
  `-32602 invalid initialize params`, closed session before `tools/list`. If the `repo` server's decoder
  regressed back to strict decoding when the Go/Rust duplication (§1a) gets resolved, watch for this
  reappearing.
- **`26/09/13/CHAT-MCP-NAVBAR-PANEL`** and **`26/09/14/CHAT-TOP-RIGHT-DETAILS-PANEL`** — no `🎫️ticket.json`
  (informal/fragment folders), single implementation notes each. Confirms the chat-dock UI work (§4) but
  neither claims real model wiring — `CHAT-MCP-NAVBAR-PANEL`'s own note only describes docking/toggle
  behavior and an `AgentPresence` header.
- **`26/07/03/REPAIR-REPO-CLI-MCP-TRANSPORT-AND-NATIVE-METADATA`**, **`26/07/10/REMOVE-BROKEN-REPO-SEARCH-MCP-TOOL`**,
  **`26/08/03/MAKE-REPO-CLI-AND-MCP-WORK-END-TO-END`** — earlier rounds of repo-mcp transport/tool repair;
  not read in full for this audit (time-boxed) but their existence corroborates that the `repo` server has
  broken and been re-fixed multiple times across the last two months — consistent with the current
  path-mismatch regression being another cycle of the same instability rather than a one-off.
- **`26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`**,
  **`26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`** — the generic `s.stdio`
  artifact/inference schema machinery (§ task's `🗄️stdio` area); this is the document-format contract
  system referenced by `📜️script.ts:13912+`'s `stdio*` validators, not an AI-specific surface. Not the
  bottleneck for AI-over-MCP; noted for completeness since the task asked about it.

---

## 7. Prioritized gap list + minimal end-to-end slice

### P0 — the servers do not start at all

1. **Fix the `repo` MCP binary path mismatch** (blocks `.mcp.json`'s `repo` entry entirely).
   - Either (a) change `defaultMcpBin()`
     (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:151-153`) to point at
     `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp` (what `@semio-tech/repo-mcp-go:build` actually produces),
     or (b) change the `@semio-tech/repo-mcp-go` build target
     (`🔌️mcp/📦️packages/🐹️go/📜️script.ts:20-24`) to build from/into the `💻️client/🔌️mcp` module and stage
     its output to the flat `💻️client/mcp` path `resolveMcpBin()` expects. (b) is almost certainly the
     intended direction given `💻️client/🔌️mcp` has the richer, tested, tool-complete implementation (§2).
   - Resolve the duplicate `github.com/usalu/semio/repo/mcp` module path (§1a) before adding
     `💻️client/🔌️mcp` to `go.work` — either delete/retire `🔌️mcp/📦️packages/🐹️go` (and its `@semio-tech/repo-mcp-go`
     nx project) or rename one module path.
   - Files to touch: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` (`defaultMcpBin`,
     `resolveMcpBin`), `go.work`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/*` (retire
     or repoint), `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📋️project.json` (add a `build`
     target if missing one for nx), `📜️script.ts` (`requireRepoMcpBinary`/`REPO_MCP_GO` if the module moves).
2. **Fix the `semio` (os) MCP credential-seal false positive** (blocks `.mcp.json`'s `semio` entry when run
   from Claude Code, and apparently from this very auditing session too).
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:104-118` — narrow
     `protected_credential_environment_is_absent()` to the *actual* hub-credential variable names/prefixes
     (e.g. `S_*` documented ones) instead of a bare substring match against `SESSION`/`TOKEN`/etc. across
     the whole environment, or add a documented allowlist for known-benign host-harness variables
     (`CLAUDE_CODE_*`, and by extension similar prefixes from other IDE/agent hosts) validated to never carry
     the hub fd/token shape. Keep the check (it is legitimate defense-in-depth per the file's own header
     comment) but make it precise rather than a blanket substring scan.
   - Add a regression test spawning the binary with a `CLAUDE_CODE_SESSION_ID`-shaped (or similarly
     generic-"SESSION"-named) benign env var present and asserting it still starts.

### P1 — missing/limited paths once the servers start

3. **`repo` server tool-surface drift** (§2): reconcile which tool set is canonical — the 6-tool
   `🗄️repository/🐹️.go` set (no `goal_*`), the monolithic `🔌️mcp/📦️packages/🐹️go` set, and the 9-tool set the
   2026-09-06 ticket claims — before wiring the fix in P0.1, so the fix doesn't just make the *wrong* tool
   set reachable.
4. **General inference execution is still `channel.not-wired`** for every plugin-declared inference service
   except GIS Map (§3). If "AI integration" is meant to include letting an agent *run* a plugin's own
   inference (not just discover it), `HeadlessWorkspace`'s `ArtifactChannel`
   (`🌉️mcp/🏠️workspace/🦀️.rs` / `🔀️dispatch/🦀️.rs`) needs an `Infer` command variant routed to
   `ArtifactInferenceRouter`, mirroring how `job_infer` already does it inside `run`'s plugin reactor.
5. **In-shell chat panel is decorative** (§4): `BasicChatPanel` never calls anything. If the goal is a human
   collaborator watching/steering an MCP-agent conversation from inside the OS, the panel needs a real data
   source — most naturally, surface the MCP bridge's actual tool-call/approval traffic (the `🧵️bridge`
   frames already carry `AgentPresence`; extending that family to carry call/approval events would let
   `AgentChatPanel` show the *real* agent conversation instead of a local echo).
6. **`go.work` module-path collision** (P0.1) also currently blocks running the `repo-mcp`
   `🤝️protocol-contract` Go test suite (§1a/§5) — once resolved, wire it into the standard nx `test`/`test-quick`
   targets so this class of regression (§6, repeated repo-mcp breakage) gets caught by CI rather than by
   manual reproduction.

### P2 — polish

7. Reconcile the closed-but-inaccurate `26/09/06` ticket summary against the live tree, or reopen/annotate
   it — right now it actively misleads anyone trusting ticket status over the working tree (per this repo's
   own "live predicate, not derived artifact" convention).
8. Decide and document whether `reasoning`/`playbook` are meant to ever become literal AI-agent surfaces (the
   names invite the assumption) or are permanently just structured-editor plugins that happen to share
   AI-adjacent vocabulary; a one-line disambiguation in their `README.md`/`AGENTS.md` would prevent future
   confusion (this audit initially assumed they were AI features from names alone).

### Minimal end-to-end slice (Claude Code ↔ semio MCP ↔ running os ↔ verified state change)

Given P0 is what's actually blocking everything, the smallest slice that proves "AI integration over MCP"
end-to-end is:
1. Apply P0.2 (env-guard fix) — this alone should make `.mcp.json`'s `semio` server connect from this very
   Claude Code session.
2. Boot a real headless workspace: `bun ./📜️script.ts dev mcp stdio os -- --folder <a real space dir>`.
3. From Claude Code (once connected): `tools/list` → confirm the 20+ tool census; `capabilities_search` for
   an action on a known simple artifact (`note`, or `cad`'s `translateSelection` per design decision D2 in
   the LLM-FIRST-OS ticket, §6); `artifact_open` that artifact; `action_prepare` → `action_invoke` the
   mutation (e.g. move a note or translate a cad selection); `artifact_snapshot`/a `semio://artifact/{id}`
   resource read to verify the state actually changed; `history_undo` to confirm the VCS round-trip.
4. This exercises transport (P0.2), catalog/discovery, the mutation protocol, and resource reads — the exact
   surface `AI-MCP-END-TO-END` (§6) already claims is real — with zero dependency on the still-broken `repo`
   server (P0.1) or on any general/GIS-only inference wiring (P1.4).
