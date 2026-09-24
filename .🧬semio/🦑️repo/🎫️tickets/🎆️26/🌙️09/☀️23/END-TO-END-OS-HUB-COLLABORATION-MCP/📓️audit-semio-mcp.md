# Semio OS MCP Audit

**Ticket:** `2026/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`  
**Date:** 2026-09-23  
**Mode:** read-only static analysis + attempted live probe  
**Scope:** `.mcp.json` → `semio` server (`bun ./📜️script.ts dev mcp stdio os`), not `repo` MCP

---

## 1. Entry path and binary

| Step | Path / artifact |
|------|-----------------|
| Client config | `.mcp.json` → `semio` server: `bun ./📜️script.ts dev mcp stdio os --folder . --scopes …` |
| Router | `📜️script.ts` → `DevScript.runMcpOs("stdio", extra)` |
| Binary resolver | `🧰️framework/…/🌉️mcp/🟦️.ts` → `requireMcpBinary()` |
| Expected binary | `🧰️framework/…/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp` |
| Cargo package | `semio-framework-os-mcp` / binary `semio-os-mcp` |
| Bootstrap (argv only) | `🧰️framework/…/🌉️mcp/🏗️bootstrap/🦀️.rs` |
| Library + `run_stdio` | `🧰️framework/…/🌉️mcp/🦀️.rs`, `🚚️transport/🦀️.rs` |

**Build command:** `bun nx run @semio-tech/framework-os-mcp-rs:build`

**Audit observation:** At audit time the staged binary was **missing**. `bun nx run @semio-tech/framework-os-mcp-rs:build` was started and remained in cargo compile/link for **>38 minutes** (heartbeat only after `wasmtime-wasi`; no `dist/build/semio-os-mcp` yet). `bun ./📜️script.ts dev mcp stdio os` fails immediately with `semio-os-mcp binary gate failed`.

---

## 2. Architecture map

```
MCP client (Cursor / Claude)
  │ stdio JSON-RPC (2025-06-18)
  ▼
semio-os-mcp (Rust)
  ├─ McpServer (protocol/transport)
  ├─ ToolRegistry ← compiled Catalog + gateway tools (28 names)
  ├─ ResourceRegistry ← WorkspaceResourceRegistry
  ├─ PromptRegistry ← 5 bilingual protocol prompts
  ├─ PolicyEngine ← AgentPrincipal + MCP_SCOPE_TABLE
  ├─ ActionAdapter ← mutation protocol (prepare/invoke/undo/…)
  │     └─ ArtifactChannels
  │           ├─ Unbound (no --folder/--hub)
  │           ├─ ShellRouted (folder/hub + optional live shell)
  │           └─ PluginArtifactChannel (per-plugin wasm guest)
  ├─ HeadlessWorkspace (--folder local | --hub remote)
  │     ├─ folder: local event-sourced artifact store
  │     └─ hub: 🏠️workspace/🔗️remote → hub directory stream + canonical pair
  ├─ BridgeSlot (optional)
  │     ├─ stdio: loopback ws `/bridge` + 🛰️rendezvous offer when `dev s` session live
  │     └─ shell dials bridge → ShellState mirror, ShellCommand dispatch
  └─ AuditSinks → ~/.semio/agent/audit (file)
```

### 2.1 `ui.observe` / `ui.control`

| MCP scope | Kernel capabilities | Tools / resources |
|-----------|---------------------|-------------------|
| `ui.observe` | `shell.observe` | Read `semio://window`, `semio://ui/active-context`, `semio://ui/selection`, `semio://ui/agent-messages` from `BridgeHandle::last_shell_state` |
| `ui.control` | `shell.control`, `ui.window`, `ui.dialog`, `shell.navigate` | `ui_focus`, `ui_reveal` → `GatewayToShell::ShellCommand` over `/bridge` |

**Bridge path (stdio, default):**

1. Unless `--no-bridge`, gateway looks for live OS sessions in `🛰️rendezvous` sessions directory.
2. If found, binds loopback HTTP transport **bridge-only** and publishes a rendezvous offer (`ws://127.0.0.1:<port>/bridge`).
3. Running `dev s` frontend dials `/bridge`, sends `ShellToGateway::Hello`, receives `GatewayToShell::Welcome`.
4. UI reads/writes go through `🧵️bridge/🦀️.rs` codec — **not** hub WebSocket directly.

**Headless:** No shell → `PLUGIN_UNAVAILABLE` (retryable). Tools/resources still **listed**.

**Implementation:** `🖥️ui/🦀️.rs`, `🧵️bridge/🦀️.rs`, `run_stdio` → `attach_stdio_bridge` in `🦀️.rs`.

### 2.2 `artifact.write`

| Stage | Mechanism |
|-------|-----------|
| Discovery | `capabilities_search` / `capabilities_describe` over compiled plugin catalog |
| Prepare | `action_prepare` → `ActionAdapter::prepare` → `PureCommand` dry-run on plugin guest |
| Commit | `action_invoke` → `TransactionPrepare` + `TransactionCommit` with `MutationOrigin::Agent { principal, invocation_id }` |
| Storage | `HeadlessWorkspace` → `PluginArtifactChannel` → plugin wasm → `.spr` event log |
| Hub sync | `open_hub` → `🏠️workspace/🔗️remote` directory stream; edits propagate as hub document events |
| Notify | `resources/subscribe` + `notifications/resources/updated` on `semio://artifact/{id}` |

**Requires:** `--folder <path>` or `--hub <url> --space <id>`. Without binding → `PLUGIN_UNAVAILABLE`.

### 2.3 `inference.execute`

| Tool | Route |
|------|--------|
| `inference_list`, `inference_get` | Descriptor metadata (no guest) |
| `inference_run` | Local: `AppCommand::Infer` → plugin wasm guest |
| `inference_submit`, `inference_events`, `inference_cancel`, `inference_approve` | Hub-backed jobs (`🌎️hub/💡️inference`) when workspace is hub-bound; GIS Map has dedicated hub wire schemas |

MCP scope `inference.execute` expands to `artifacts.read`, `artifacts.write`, `jobs.spawn` — **local admission only**; hub re-checks author/session on each hub route.

**Implementation:** `💡️inference/🦀️.rs`, `🏠️workspace/🔗️remote/🦀️.rs` (hub GIS routes).

### 2.4 `conversation.write`

| Tool | Scope | Path |
|------|-------|------|
| `conversation_reply` | `shell.converse` | Publishes agent prose to attached shell via bridge; pushes `semio://ui/agent-messages` updates |

Requires live `/bridge` + shell (same as UI control tier).

**Implementation:** `🖥️ui/🦀️.rs` (`conversation_reply_handler`), `🧵️bridge/🦀️.rs` (`AgentConversation`).

### 2.5 Identity / auth / attribution

| Mode | Principal | Auth |
|------|-----------|------|
| Local `--folder` | `--principal` or default `agent:local` | Scopes from `--scopes` only |
| Hub + delegation | `agent:<delegation id>` after `POST /auth/agent-sessions` | `--credential-file` (0600 JSON); token wiped after exchange |
| Hub + sealed child | Inherited fd-3 / process-entry credential | `claimed_local_hub_credential("mcp")` |
| Mutations | `MutationOrigin::Agent { principal, invocation_id }` on `TransactionPrepare` | Audited in `AuditSinks`; `principal_id` on prepared actions |

**Collaboration:** Hub agent delegations get their own principal (`agent:<id>`), visible in rosters/undo — documented in `🌉️mcp/README.md`. Folder mode uses configured principal string; no per-user login in stdio alone.

**Process seal:** `🏗️bootstrap/🦀️.rs` rejects unsealed parent env (hub tokens in env vars) unless `SEMIO_DIRECT_CHILD_BENIGN=preserved` (MCP client harness vars allowed).

---

## 3. MCP scope table (`.mcp.json` scopes → kernel)

From `🛡️policy/🦀️.rs` `MCP_SCOPE_TABLE`:

| MCP scope (`.mcp.json`) | Expands to kernel capabilities |
|-------------------------|--------------------------------|
| `workspace.read` | `registry.query`, `artifacts.read` |
| `artifact.write` | `artifacts.write`, `jobs.spawn` |
| `inference.execute` | `artifacts.read`, `artifacts.write`, `jobs.spawn` |
| `ui.observe` | `shell.observe` |
| `ui.control` | `shell.control`, `ui.window`, `ui.dialog`, `shell.navigate` |
| `conversation.write` | `shell.converse` |

Enforcement: `PolicyEngine::authorize_scopes` on each capability's `policy.scopes` before tool execution.

---

## 4. Tools (28 gateway names + plugin capabilities)

**Declared in** `GATEWAY_TOOL_NAMES` (`🦀️.rs`). Plugin mutations are **not** separate tool names — they are invoked via `action_prepare`/`action_invoke` with `capabilityId` from catalog.

| Tool | Category | Scope / tier notes |
|------|----------|-------------------|
| `context_resolve` | Meta | Always works; reports channel (`headless`/`shell`), principal, scopes, catalog hash |
| `capabilities_search` | Meta | Always works |
| `capabilities_describe` | Meta | Always works |
| `action_prepare` | Mutation | Needs workspace + scopes + plugin wasm staged |
| `action_invoke` | Mutation | Same |
| `action_cancel` | Mutation | Same |
| `transaction_begin` | Mutation | Same |
| `transaction_commit` | Mutation | Same |
| `transaction_rollback` | Mutation | Same |
| `history_undo` | History | Same |
| `history_redo` | History | Same |
| `artifact_open` | Artifact | Workspace bound |
| `artifact_create` | Artifact | Workspace bound |
| `artifact_validate` | Artifact | Workspace bound |
| `artifact_snapshot` | Artifact | Workspace bound |
| `artifact_export` | Artifact | Workspace bound |
| `inference_list` | Inference | Workspace for live rows; descriptors always from registry |
| `inference_get` | Inference | Same |
| `inference_run` | Inference | Guest execution; needs `inference.execute` |
| `inference_submit` | Hub job | Hub workspace |
| `inference_events` | Hub job | Hub workspace |
| `inference_cancel` | Hub job | Hub workspace |
| `inference_approve` | Hub job | Hub workspace |
| `ui_focus` | UI | Bridge + shell; `ui.control` |
| `ui_reveal` | UI | Bridge + shell; `ui.control` |
| `conversation_reply` | Conversation | Bridge + shell; `conversation.write` |
| `job_get` | Jobs | Job registry (inference, shell commands) |
| `job_cancel` | Jobs | Same |

**Runtime status:** Not live-verified — binary missing. Static: all 28 are implemented (no stubs per code comments ticket 26/08/29).

---

## 5. Resources

**Static list** (`WorkspaceResourceRegistry` + UI + inference):

| URI | Tier without workspace | Tier without bridge |
|-----|------------------------|---------------------|
| `semio://capability` | Works | Works |
| `semio://capability/{id}` | Works | Works |
| `semio://workspace` | `PLUGIN_UNAVAILABLE` | — |
| `semio://workspace/artifacts` | `PLUGIN_UNAVAILABLE` | — |
| `semio://artifact/{id}` | `PLUGIN_UNAVAILABLE` | — |
| `semio://artifact/{id}/history` | bound | — |
| `semio://artifact/{id}/validation` | bound | — |
| `semio://artifact/{id}/inference` | bound | — |
| `semio://artifact/{id}/inference/{field}` | bound | — |
| `semio://window` | listed | `PLUGIN_UNAVAILABLE` |
| `semio://window/{windowId}` | template | bridge |
| `semio://ui/active-context` | listed | bridge |
| `semio://ui/selection` | listed | bridge |
| `semio://ui/agent-messages` | listed | bridge |
| `semio://job/{jobId}` | template | job registry |

Hub mode adds hub descriptor/checkpoint URIs via remote workspace (`🏠️workspace/🔗️remote/🦀️.rs`).

---

## 6. Prompts (5)

From `💬️prompts/🦀️.rs` `GATEWAY_PROMPT_NAMES`:

1. `explore_workspace` — context + catalog discovery workflow  
2. `safe_mutation` — prepare → invoke → verify loop  
3. `inspect_artifact` — artifact + inference inspection  
4. `drive_the_ui` — bridge-attached UI read/control  
5. `undo_last_change` — undo/redo in multi-user event-sourced workspace  

Bilingual (`en`/`de`) via optional `locale` argument.

---

## 7. Live probe

**Driver:** `mcp-probe/probe.ts` (ticket folder)  
**Output dir:** `🗑️generated/mcp/`

**Planned steps:** `initialize` → `notifications/initialized` → `tools/list` → `resources/list` → `prompts/list` → `ping` → `context_resolve` → `resources/read semio://workspace` → `capabilities_search`.

**Result:** **Not executed** — `requireMcpBinary()` failed (no `dist/build/semio-os-mcp`). Probe script is ready; re-run after:

```bash
bun nx run @semio-tech/framework-os-mcp-rs:build
bun .🧬semio/…/END-TO-END-OS-HUB-COLLABORATION-MCP/mcp-probe/probe.ts
```

**Existing e2e harness (for reference):** `🌉️mcp/🟦️.ts` → `runOsMcpClientJourney()` — full mutation + inference journey used by nx gates.

---

## 8. Tests

| Target | Command | Status |
|--------|---------|--------|
| Quick unit (Rust) | `bun nx run @semio-tech/framework-os-mcp-rs:test-quick` | **Not run** (blocked on binary / cargo lock with in-flight build) |
| Full test | `bun nx run @semio-tech/framework-os-mcp-rs:test` | Not run |
| Client e2e | via `🟦️.ts` `runMcpClientEndToEnd` | Not run |
| Live agent loop | `live-agent-loop-check` | Requires built binary + live `dev s` |
| Hub agent participant | `hub-agent-participant-check` | Requires hub |
| Agent reply | `agent-reply-check` | Requires bridge |
| Hygiene (TS) | `🧪️tests/🧹️hygiene/🟦️.ts` | Not run |
| Conformance | `🧪️conformance/🦀️.rs` | Compiled with `test-quick` |

**Test locations:** `🧰️framework/…/🌉️mcp/**/🧪️tests/**` (Rust quick/long + TS integration).

---

## 9. Prioritized gaps

### P0 — Blocking end-to-end MCP for developers

1. **Binary not present / build extremely slow**  
   - **Symptom:** `.mcp.json` `semio` server cannot start.  
   - **Paths:** `📦️packages/🦀️rust/dist/build/`, `📜️script.ts` `requireMcpBinary`.  
   - **Fix:** Ensure `bun nx run @semio-tech/framework-os-mcp-rs:build` completes and stages binary; consider documenting minimum RAM/time; investigate >30min compile stall after `wasmtime-wasi`.

2. **Plugin WASM must be staged for mutations**  
   - **Symptom:** `action_prepare` / `inference_run` fail without compiled plugin guests.  
   - **Paths:** `🟦️.ts` `verifyStagedPluginComponent`, `PLUGIN_WASM_TARGET_REL`.  
   - **Fix:** `bun nx run @semio-tech/framework-os-dev:build -- <plugin>` + `describe` hashes; document in onboarding.

### P1 — Hub + collaboration path

3. **Hub MCP needs credential file**  
   - **Symptom:** `--hub` without `--credential-file` or fd-3 fails `PermissionDenied`.  
   - **Paths:** `🦀️.rs` `server_for_workspace_options`, `🌉️mcp/README.md`.  
   - **Fix:** Ship delegation UI flow; example `.mcp.json` with `--hub --credential-file`.

4. **UI / conversation require live `dev s` + bridge**  
   - **Symptom:** `ui_*` and `conversation_reply` return `PLUGIN_UNAVAILABLE` headless.  
   - **Paths:** `attach_stdio_bridge`, `🖥️ui/🦀️.rs`.  
   - **Fix:** Document that Cursor users must run OS frontend first; or use `live-agent-loop-check` gate in CI.

5. **Agent attribution in folder mode is coarse**  
   - **Symptom:** Default `agent:local` for all stdio clients on same folder.  
   - **Paths:** `run_stdio` principal default, `MutationOrigin::Agent`.  
   - **Fix:** Require `--principal` per client or hub delegations for multi-agent collaboration.

### P2 — Policy / catalog quality

6. **Capability audit findings (destructive verbs)**  
   - **Symptom:** Some `delete*` capabilities lack `effects.destructive: true` — approval gate skipped.  
   - **Paths:** plugin `🔣️.json` descriptors; `capability-audit-check` nx target.  
   - **Fix:** Run `semio-os-mcp audit --folder .`; fix descriptor `effects` flags.

7. **Inference on hub is service-specific**  
   - **Symptom:** GIS Map has full hub route; other plugins use local `inference_run` only.  
   - **Paths:** `💡️inference/🦀️.rs`, `🌎️hub/💡️inference/`.  
   - **Fix:** Generalize hub inference bridge beyond GIS Map.

### P3 — Developer experience

8. **No published install artifact**  
   - README states no npm/Homebrew/signed release — clone + build only.  
   - **Fix:** `build-release` + `publish` target exists; ship tarball or signed binary.

9. **`.mcp.json` binds `--folder .` (repo root)**  
   - Self-test binding, not user workspaces.  
   - **Fix:** Document user-facing `.mcp.json` should point `--folder` at their space directory.

---

## 10. Related files (quick index)

| Concern | Primary paths |
|---------|----------------|
| MCP router | `📜️script.ts`, `.mcp.json` |
| Server lib | `🌉️mcp/🦀️.rs` |
| Scopes | `🌉️mcp/🛡️policy/🦀️.rs` |
| Mutations | `🌉️mcp/🔀️dispatch/🦀️.rs` |
| Workspace / hub | `🌉️mcp/🏠️workspace/🦀️.rs`, `🏠️workspace/🔗️remote/🦀️.rs` |
| UI + conversation | `🌉️mcp/🖥️ui/🦀️.rs` |
| Bridge | `🌉️mcp/🧵️bridge/🦀️.rs` |
| Inference | `🌉️mcp/💡️inference/🦀️.rs` |
| Resources / context | `🌉️mcp/🧠️context/🦀️.rs` |
| Prompts | `🌉️mcp/💬️prompts/🦀️.rs` |
| TS client / e2e | `🌉️mcp/🟦️.ts` |
| Hub inference backend | `🌎️hub/💡️inference/` |
| User docs | `🌉️mcp/README.md` |

---

## 11. Probe transcript summary

**Status:** No JSON-RPC transcript — server binary unavailable at audit time.

**Prepared artifacts:**
- `mcp-probe/probe.ts` — stdio probe driver  
- `🗑️generated/mcp/` — output directory (empty until probe succeeds)

**Next step after build:** Run probe; expect `initialize` → `serverInfo.name: semio`, `tools/list` ≥28 tools, `context_resolve` → `channel: headless` with `--no-bridge`, `semio://workspace` read succeeds with `--folder`.
