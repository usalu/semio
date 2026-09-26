# E2E User Path — Norm Artifacts (read-only audit)

**Auditor:** Composer 2.5 (Wave A, cross-cutting) · **Date:** 2026-09-26 · **Repo:** `/Users/ueli/Documents/semio`

## Executive summary

A developer or power user **can** open a norm family editor today (15 playground variants, dedicated `launch.json` entries, or full `s` host with palette spawn), load the bundled `🎬️demo` example, edit subject JSON in the Inputs window, and see a one-line-per-check Results table after `evaluate`. The path is **real but demo-grade**: subjects are flat scalar snapshots (not complete building models), Results are English-only one-liners with no remediation, Inputs are read-only pretty JSON (not field editors), and `setSnapshot` is capped at 8 KiB (`NORM_RETAINED_RAW_BYTES`). **Norm is shipped** as a dynamic WASM plugin (`semio_s_plugin_norm.wasm`) in the generated registry and activates on artifact kind — not as the default `s` host plugin (that is `space`). **MCP headless create → evaluate → read report** is architecturally supported (`artifact_create` + `action_invoke` on `evaluate`) but requires a built norm WASM, a writable `--folder` workspace, and (for live UI verification) a running shell bridge; no committed MCP law fixture targets a norm kind yet (writer/note only). **No norm-specific Playwright/Storybook e2e** exists; coverage is Rust unit/oracle tests per family, TS example tests, one B2B interaction probe (`din4108`), and generic MCP gates. Seed spaces under `.🧬semio/🔗space/` contain no norm artifacts.

---

## 1. Plugin loading — where `norm` runs

### 1.1 Registration list (authoritative)

| Field | Value | Ref |
|-------|-------|-----|
| Registry file | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` | L896–923 |
| `pluginId` | `norm` | L896 |
| WASM output | `semio_s_plugin_norm.wasm` | L900 |
| Crate | `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust` | L898 |
| Activation | `on-artifact-kind:computation.norm.<family>` × 15 | L908–922 |

Generation: `bun nx run @semio-tech/plugin-registry:generate` (failure message at `🌉️mcp/🏠️workspace/🦀️.rs` L122).

### 1.2 Loading mechanism — dynamic WASM, not static Rust link

`load_plugin_registry()` reads `🔌️plugins.json`, resolves `wasmOut` under `PLUGIN_WASM_TARGET_DIR/{wasm-dev,wasm-release}` (`🏠️workspace/🦀️.rs` L117–149). Plugins are **not** compiled into the OS binary; guests load at activation.

Plugin assembly: `✏️s/🔌️plugins/📕️norm/🦀️.rs` — `plugin()` L59–172 registers 15 artifact families + editor/viewer pairs; `ActivationEvent::OnArtifactKind` per family L154–168; `plugin_exports!` L180.

### 1.3 Host apps

| Host | Loads norm? | How |
|------|-------------|-----|
| **semio OS (`s`)** | Yes (on demand) | Default boot plugin is `space` (`🛠️dev🪐️space⚛️react` → `SEMIO_PLUGIN: "s"`, `launch.json` L3301–3311). Norm WASM activates when user opens/creates a norm artifact kind (S6 measured 35 spawnable kinds inside `s`, `📓️s6-all-plugin-kinds-inside-s.md` L14). |
| **Playground per family** | Yes (direct) | `[[package.metadata.semio.playground]]` in `📦️packages/🦀️rust/Cargo.toml` L17–90: variants `din4108`…`vdi3805`, apps `s.norm.<family>@1/*#editor`, ports react 6091–6105 / wgpu 6191–6205. |
| **Desktop wgpu / web react** | Same OS stack | Renderer chosen via `SEMIO_RENDERER` (`react` or `wgpu`); norm plugin identical. |
| **Hub (`🌎️hub`)** | Indirect | Hub serves shared spaces; norm documents live as hub artifacts. Hub has no norm-specific loader — clients use OS + MCP `--hub` mode. Local `create_plugin_artifact` is refused on hub-bound MCP workspaces (handler at `🗿️artifact/🦀️.rs` L313+). |

**Verdict:** Norm **is** in the shipped plugin set (registry row present). It is **not** the default landing app.

---

## 2. `launch.json` — what devs run

### 2.1 Recommended for trying norms (per family)

Example DIN 4108 React (`launch.json` L5834–5853):

- **Name:** `🛠️dev📕️norm🧱️din4108⚛️react`
- **Command:** `bun nx run workspace:dev -- din4108`
- **Env:** `S_OS_PORT=6091`, `SEMIO_PLUGIN=din4108`, `SEMIO_RENDERER=react`, `SEMIO_APP=s.norm.din4108@1/*#editor`
- **Browser:** auto-opens `http://127.0.0.1:6091`

Pattern repeats for all 15 families × `{react, wgpu}` (~30 configs, e.g. `🛠️dev📕️norm🏛️en1992⚛️react` L5966–5985, port 6096).

### 2.2 Full semio OS (space host, can spawn norm from palette)

- **Name:** `🛠️dev🪐️space⚛️react` (L3301–3321)
- **Command:** `bun nx run workspace:dev -- s`
- **Env:** `SEMIO_PLUGIN=s`, port 6070, `S_DATA_DIR=.🧬semio/🔗space/s-dev`

### 2.3 MCP-related

| Config | Command | Purpose |
|--------|---------|---------|
| `🦑️mcp dev` (L359–367) | `bun ./📜️script.ts dev mcp stdio client` | Repo MCP client smoke |
| `🛠️dev🌉️os-mcp🤝️client-e2e` (L3709+) | `bun nx run @semio-tech/framework-os-mcp:client-e2e` | MCP TypeScript e2e |
| `⚖️gate🌉️os-mcp🤖️live-agent-loop` (L4840+) | `bun nx run @semio-tech/framework-os-mcp-rs:live-agent-loop-check` | Live shell + MCP gate |

`.mcp.json` `semio` server (L14–27): `bun ./📜️script.ts dev mcp stdio os --folder .🧬semio/🔗space/os-mcp --scopes workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write`.

---

## 3. User journey today (concrete, DIN 4108 as template)

### 3.1 Create / open artifact

**Paths a user actually has:**

1. **Playground dev** — launch `🛠️dev📕️norm🧱️din4108⚛️react`; OS boots straight into the din4108 editor with a genesis document (playground wiring via `workspace:dev -- din4108`).
2. **Full `s` host** — launch `🛠️dev🪐️space⚛️react`; use Create Artifact / palette to spawn `s.norm.din4108@1/*#editor` (S6: 32/35 kinds open windows).
3. **Load example** — verb `setActiveExample` with `exampleId: "demo-session"` (`✏️editor/📚️examples/🎬️demo-session/🦀️.rs` L5; wired L45, L134–139 in `✏️editor/🦀️.rs`). Primary demo document: `🖼️assets/🎬️demo/🗣️.dsl.semio` (`📚️examples/🎬️demo/🦀️.rs` L10).
4. **MCP** — `artifact_create` with `kind: "s.norm.din4108"` (schema id from descriptor `🔣️.json` L14; see §4).
5. **Seed spaces** — `.🧬semio/🔗space/os-mcp` exists but holds only `.semio` metadata; **no pre-seeded norm artifacts** (directory listing 2026-09-26).

There is no separate “new document template” file in the space folder; genesis comes from the plugin's artifact declaration + optional examples.

### 3.2 Edit subject

- **UI:** Edit mode layout = Inputs (42%) + Results (58%) row (`✏️editor/🎭️modes/✏️edit/🦀️.rs` L14–16).
- **Inputs window:** Read-only pretty-printed JSON (`🖥️app-surface/🦀️.rs` `render_document_json` L219–222). User cannot edit fields in-place — must use Actions / MCP `setSnapshot`.
- **Verbs** (`✏️editor/🦀️.rs` L98, L130–155): `setSnapshot`, `evaluate`, `setSelectedCheckIndex`, `setActiveExample`.
- **`setSnapshot`:** Expects camelCase JSON of full document (`L141–148`). Payload cap `NORM_RETAINED_RAW_BYTES = 8_192` (`🖥️app-surface/🦀️.rs` L541) — blocks realistic subjects.
- **B2B probe** proves one-field `setSnapshot` works when editor is running (`OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b2b-norm-probe.mjs` L1–27).

### 3.3 Evaluate and read compliance report

- **Trigger:** `evaluate` action → `Din4108Command::Evaluate` (`✏️editor/🦀️.rs` L132).
- **Computation:** `💡️inferences/🦀️.rs` `evaluate(document)` L182 → `CheckReport`.
- **Results window:** `📊️results/🦀️.rs` L22–24 calls `render_report(host.report(), windows)`.
- **Rendering:** One virtualised row per check: `clause — status u=utilization — message` (`🖥️app-surface/🦀️.rs` L204–216). English labels hardcoded (`norm_ui_label("Checks")` L210).
- **Viewer mode:** Separate viewer app renders report window (`👁️viewer/…/📊️report/🦀️.rs` L31).

### 3.4 Viewer

Each family has `s.norm.<family>@1/*#viewer` paired in `🦀️.rs` L96–153. Viewer shows report surface; editing happens in editor app.

### 3.5 What is broken/missing for a real end user

(from `📓️coordination.md` L11–17, confirmed in code):

| Gap | Evidence |
|-----|----------|
| Stub subjects (6–74 scalars, tuned to pass) | Coordinator baseline L13–14 |
| No remediation in `CheckResult` | `⚖️compliance/🦀️.rs` L138–145 — `message: String` only |
| No localization in report rows | `render_report` L213 — English format string |
| JSON-only editing, read-only Inputs | `render_document_json` L219–222 |
| 8 KiB `setSnapshot` cap | `NORM_RETAINED_RAW_BYTES` L541 |
| Catalogue panel placeholder | coordination L16 |
| No norm artifacts in seed spaces | `.🧬semio/🔗space/` inspection |

---

## 4. semio MCP server — tools, norm path, JSON-RPC

### 4.1 Implementation & launch

- **Module:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/`
- **Binary:** `semio-os-mcp` (build: `bun nx run @semio-tech/framework-os-mcp-rs:build`)
- **Config:** `.mcp.json` → folder `.🧬semio/🔗space/os-mcp`

### 4.2 Tool surface (28 tools)

`GATEWAY_TOOL_NAMES` (`🦀️.rs` L282–311): `capabilities_search`, `capabilities_describe`, `context_resolve`, `action_prepare`, `action_invoke`, `action_cancel`, `transaction_*`, `history_undo/redo`, `artifact_create/open/validate/snapshot/export`, `inference_*`, `ui_focus/reveal`, `conversation_reply`, `job_get/cancel`.

README workflow table (`README.md` L35–44) maps user intents to tools.

### 4.3 Can an agent create a norm artifact, evaluate, observe UI?

| Step | Tool | Headless? | Notes |
|------|------|-----------|-------|
| Discover verbs | `capabilities_search` `{ "query": "evaluate din4108" }` | Yes | Catalog grammar: `<plugin>.<appId>.<actionId>` (`🗂️catalog/🦀️.rs` L1–3) |
| Create document | `artifact_create` | Yes* | *Needs built `semio_s_plugin_norm.wasm` + writable folder |
| Edit subject | `action_prepare` → `action_invoke` on `setSnapshot` | Yes | Destructive; may need approval (`effects.destructive: true`, descriptor L146–147) |
| Run checks | `action_invoke` on `evaluate` | Yes | Recomputes projection |
| Read report | `resources/read` `semio://artifact/{id}` or open artifact structured content | Yes | Report in session document / validation resource |
| See Results window | `ui_reveal` / `ui_focus` | **No** without shell | `PLUGIN_UNAVAILABLE` if no bridge session (`README.md` L171–174) |

`artifact_create_handler` (`🗿️artifact/🦀️.rs` L313–373): validates `kind` against `installed_artifact_kinds()`; spawns cancellable plugin guest.

**Norm kinds** (artifact schema): `s.norm.din4108`, `s.norm.en1992`, … (descriptor `🔣️.json` L14 `artifactKind`).

**Evaluate capability id** (by D3 grammar + writer precedent `writer.s.writer.writer@1/*#editor.setText`):

`norm.s.norm.din4108@1/*#editor.evaluate`

### 4.4 Example JSON-RPC sequence (stdio, legacy era)

Assumes MCP process stdin/stdout, one JSON object per line.

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"audit","version":"0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"artifact_create","arguments":{"artifactId":"norm-din4108-1","kind":"s.norm.din4108"}}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"artifact_open","arguments":{"artifactId":"norm-din4108-1"}}}
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"capabilities_search","arguments":{"query":"din4108 evaluate"}}}
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"action_prepare","arguments":{"capabilityId":"norm.s.norm.din4108@1/*#editor.evaluate","input":{}}}}
{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"action_invoke","arguments":{"preparedActionHandle":"<from prepare>"}}}
```

Modern era: first line may be `{"jsonrpc":"2.0","id":0,"method":"server/discover","params":{}}` instead of `initialize` (`README.md` L292–297; `🧪️tests/🧷️untrusted-content/🟦️.ts` L93–94).

**Edit subject via MCP:**

```json
{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"action_prepare","arguments":{"capabilityId":"norm.s.norm.din4108@1/*#editor.setSnapshot","input":{"snapshot":"<camelCase JSON string>"}}}}
```

Pattern mirrors `🧷️untrusted-content/🟦️.ts` `plant()` L75–79.

**UI observation (requires live shell):** start `🛠️dev📕️norm🧱️din4108⚛️react`, then MCP without `--no-bridge`; use `ui_reveal` — gate exercised in `🤖️live-agent-loop/🟦️.ts`.

---

## 5. Existing tests touching norms

| Layer | Location | Command | What it proves |
|-------|----------|---------|----------------|
| Plugin aggregate | `@semio-tech/norm-plugin` | `bun nx run @semio-tech/norm-plugin:test` | Surface/oracle sweep |
| Quick | same | `bun nx run @semio-tech/norm-plugin:test-quick` | Fast regression |
| Results window config | same | `bun nx run @semio-tech/norm-plugin:results-window-config-test` | Window config lane |
| Component budget | same | `bun nx run @semio-tech/norm-plugin:component-budget-check` | WASM ≤ 256 MiB (NB1 fix) |
| Per-family Rust | e.g. `🗿️artifacts/🧱️din4108/…/🧪️tests/` | via plugin `test` target | `evaluate()` numbers, mutations |
| TS example | `📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | part of family tests | Demo asset non-empty |
| Gherkin | `🧱️mutate-din4108-1/🥒️.feature` | cucumber via family harness | Mutation round-trip on demo DSL |
| Python oracle | `🧱️mutate-din4108-1/🐍️.py` | family test script | Third-party parity |
| B2B interaction | `OS-HUB…/🐍️b2b-norm-probe.mjs` | manual with running dev server | 19 checks, setSnapshot, undo/redo |
| MCP Rust quick | `@semio-tech/framework-os-mcp-rs:test-quick` | artifact tool schemas | Generic artifact_create (probe kind) |
| MCP client e2e | `@semio-tech/framework-os-mcp:client-e2e` | launch `🛠️dev🌉️os-mcp🤝️client-e2e` | Protocol conformance |
| Live agent loop | `@semio-tech/framework-os-mcp-rs:live-agent-loop-check` | needs spawned editor + shell | create/open/prepare/invoke chain |

**Not found:** dedicated norm Playwright spec, norm Storybook stories, hub integration test targeting norm kinds.

---

## 6. Known blockers / recent ticket findings

| Issue | Source | Status |
|-------|--------|--------|
| WASM component over 256 MiB bound blocked describe | `📓️nb1-norm-describe-under-bound.md` L14–18 | **Fixed** 2026-09-22 via `strip = "symbols"` profile; 120 MB |
| `setSnapshot` decode for en1990/din18599 | `wp-t12/norm-set-snapshot.py` L1–5 | Fix scripted (JSON decode arm) |
| Stub subjects, no remediation, EN-only UI | `📓️coordination.md` L11–17 | **Open** (this ticket) |
| MCP audit: destructive verbs mis-tagged | `🌉️mcp/README.md` L58–63 (`setSnapshot` etc.) | Partial — gate gaps |
| `descriptor_is_fresh` on norm in sweeps | play-grid tickets | Intermittent when descriptor stale |
| Hub document inside `s` blocked mid-run | `📓️s6-all-plugin-kinds-inside-s.md` L16 | Environmental |
| No `nb1-norm-describe` under END-TO-END-OS-HUB path name user gave | file is under `OS-HUB-COLLABORATION-AI-END-TO-END/` | Located |

---

## 7. RUNBOOK — verify runtime end-to-end (headless-preferred)

### 7.1 Fastest UI smoke (developer, not headless)

```bash
# From repo root — or use launch.json 🛠️dev📕️norm🧱️din4108⚛️react
bun nx run workspace:dev -- din4108
# Browser: http://127.0.0.1:6091
# Actions: setActiveExample → demo-session; evaluate; inspect Results pane
```

### 7.2 Headless computation (no browser)

```bash
bun nx run @semio-tech/norm-plugin:test-quick
# Or single family:
cd "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust" && bun ./📜️script.ts test quick
```

Confirms `evaluate()` oracles for all families without UI.

### 7.3 B2B interaction probe (needs running din4108 dev server)

```bash
# Terminal 1
bun nx run workspace:dev -- din4108
# Terminal 2
node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️b2b-norm-probe.mjs"
```

Expect: 19 evaluated checks, `setSnapshot` commit, undo/redo (`🐍️b2b-norm-probe.mjs` L2–4).

### 7.4 MCP headless artifact lifecycle

```bash
# Prerequisite: norm WASM built (happens via dev/mcp launcher or explicit plugin build)
FOLDER=$(mktemp -d)
bun ./📜️script.ts dev mcp stdio os --folder "$FOLDER" --scopes workspace.read,artifact.write <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"runbook","version":"1"}}}
{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"artifact_create","arguments":{"artifactId":"runbook-din4108","kind":"s.norm.din4108"}}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"artifact_open","arguments":{"artifactId":"runbook-din4108"}}}
EOF
rm -rf "$FOLDER"
```

If `artifact_create` errors with missing WASM → run `bun nx run @semio-tech/norm-plugin:describe` or start a norm dev session once to compile guest.

### 7.5 MCP + live UI (full agent loop)

```bash
# Terminal 1: norm editor
bun nx run workspace:dev -- din4108
# Terminal 2: gate (spawns editor if S_OS_MCP_LIVE_SPAWN set — see live-agent-loop test)
bun nx run @semio-tech/framework-os-mcp-rs:live-agent-loop-check
```

Or launch config `⚖️gate🌉️os-mcp🤖️live-agent-loop`.

### 7.6 Component / describe health

```bash
bun nx run @semio-tech/norm-plugin:component-budget-check
bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check
```

---

## 8. File reference index (quick)

| Topic | Path | Lines |
|-------|------|-------|
| Plugin exports | `✏️s/🔌️plugins/📕️norm/🦀️.rs` | 59–180 |
| Registry row | `🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` | 896–923 |
| WASM loader | `🌉️mcp/🏠️workspace/🦀️.rs` | 117–149 |
| Playgrounds | `📕️norm/📦️packages/🦀️rust/Cargo.toml` | 17–90 |
| Editor verbs | `🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs` | 98–155 |
| evaluate() | `…/💡️inferences/🦀️.rs` | 182 |
| Results UI | `…/📊️results/🦀️.rs` | 22–24 |
| Report render | `🖥️app-surface/🦀️.rs` | 204–222, 541 |
| CheckResult | `⚖️compliance/🦀️.rs` | 138–145 |
| MCP tools | `🌉️mcp/🦀️.rs` | 282–311 |
| artifact_create | `🌉️mcp/🗿️artifact/🦀️.rs` | 313–373 |
| MCP README | `🌉️mcp/README.md` | 1–120, 290–320 |

---

*Read-only audit. No commands executed that modify git state or run `cargo build`.*
