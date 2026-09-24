# GJ2 — AI over the semio MCP (local path)

Slice: GJ2 · Session 9 · 2026-09-23
Scope: zero-touch MCP binary staging; prove GJ1 epoch/watchdog fix (client-e2e + wfc bitmap);
G19 reach gaps (stdio algorithms, cad-extension inference via MCP, two-way chat panel).
Does NOT own: M10b "agent commits a hub edit".

## 0. Headline

1. **Zero-touch MCP staging landed and proven live.** `ensureMcpBinary` content-hashes the MCP
   module's Rust/schema sources, stages via `bun ./📜️script.ts build` (Nx fallback), stamps
   `.content-hash`, reports progress on stderr. `dev mcp stdio|http os` and `bun ./📜️script.ts setup`
   call it. Cold initialize no longer dies on a missing binary (audit waited 39+ min). Probe
   transcript: `initialize` → `tools/list` (28) → `resources/list` (7) → `prompts/list` (5) →
   read-only tools — `wp-gj2/generated/mcp-probe/probe-transcript.json`.
2. **G19 reach gaps for algorithms + cad + chat are closed in the live roster.** Same probe's
   `inference_list` returned **74** declared inferences: `stdio` ×67 geometric algorithms,
   `cad-extension-aec-building` ×1 (`s.cad-extension-aec-building.building-structure-summary` on
   `s.cad.cad`), `wfc` ×5, `gis` ×1. `tools/list` includes `conversation_reply` (AC1's two-way chat).
3. **GJ1 host currency separation is law-green; live `inference_run` of `s.wfc.bitmap.solve` still
   hangs after the first progress notification** (same ceiling that stops `client-e2e` at the
   inference step). Watchdog + trap-classifier laws pass; the guest solve did not finish inside
   600 s / 240 s budgets. Honest gap — not claimed green.

## 1. Inherited state

| source | takeaway |
| --- | --- |
| audit-semio-mcp | staged binary missing; `requireMcpBinary()` hard-failed |
| GJ1 | `RELAY_JOB_BUDGET.deadline_ms=2` was armed as wasmtime epoch; `GUEST_JOB_WATCHDOG_MS=30_000` + pool pump + Infer→headless — §§6–10 still filling in their report |
| AC1 | `AgentReply` + `conversation_reply` + `conversation.write` already in tree / `.mcp.json` |
| G19 | gaps 1–3 were chat / stdio descriptor / cad roster — all three now observable via MCP |

## 2. Zero-touch MCP binary staging

### 2.1 Design

| piece | location |
| --- | --- |
| `mcpSourceContentHash` / `ensureMcpBinary` / `MCP_BINARY_TARGET` | `🌉️mcp/🟦️.ts` BinaryPath region |
| Verb | root `📜️script.ts` `DevScript.runMcpOs` → `ensureMcpBinary` |
| Setup prebuild | `SetupScript.runFull` calls `ensureMcpBinary` so `.mcp.json` initialize never waits on a first compile |
| Staging | package `bun ./📜️script.ts build` first (avoids Nx daemon socket stalls); Nx target fallback |
| Stamp | `<binary>.content-hash` — sha256 of sorted path+bytes under the MCP module (`.rs`, `Cargo.toml`, binary-gate + schema JSON) |
| Progress | every staging line on **stderr** (`[semio-os-mcp] …`) |
| Overrides | `SEMIO_OS_MCP_BIN` still skips staging (tests / release pins) |
| `spawnRawMcp.request` | skips server→client notifications until the matching response id; default wait 120 s |

### 2.2 Measured

| check | result | capture |
| --- | --- | --- |
| First cold stage | Finished `dev` in **2m 47s**, binary 163 MB | `generated/gj2-ensure-build.txt` |
| Second call (same hash) | `staged binary is fresh (content-hash b952afab025a…)` | probe stderr |
| Probe E2E | initialize + lists + read-only tools green (~12 s warm) | `generated/mcp-probe/probe-transcript.json` |
| Vitest laws | `ensureMcpBinary` stages once then skips on stamp; override skips staging | `🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts` |

Handshake policy: **prebuild in `setup`** (chosen over a shim mini-server). On-demand ensure still blocks spawn until the binary exists, with stderr progress — after one `setup` / first ensure, open is milliseconds.

## 3. Probe driver transcript

`bun .tmp-ticket/mcp-probe/probe.ts` (folder temp workspace, `--no-bridge`, full scopes including `conversation.write`):

| step | result |
| --- | --- |
| initialize | protocol `2025-06-18`, caps prompts/resources/tools |
| tools/list | **28** tools including `conversation_reply` |
| resources/list | 7 URIs (`semio://capability`, workspace, window, ui/*, …) |
| prompts/list | 5 bilingual prompts |
| context_resolve | `channel=headless`, scopes include `shell.converse` |
| inference_list | **74** declared (stdio 67 + wfc 5 + gis 1 + cad-extension 1) |

## 4. GJ1 root fix verification

### 4.1 Host laws (private `CARGO_TARGET_DIR=…/wp-gj2/target`, `CARGO_INCREMENTAL=0`)

| law | result | capture |
| --- | --- | --- |
| `the_host_watchdog_is_not_the_guests_cooperative_grant` | **ok** | `generated/gj2-host-laws.txt` |
| `a_fuel_cut_and_an_epoch_cut_are_named_rather_than_reported_as_a_backtrace` | **ok** | `generated/gj2-host-laws-fuel.txt` / terminal 153902 |
| `a_real_trap_keeps_its_whole_source_chain_not_its_first_line` | **ok** | `generated/gj2-host-laws-trap.txt` |

Source confirms `GUEST_JOB_WATCHDOG_MS = 30_000` and `arm_guest_job_watchdog` on start/step/cancel (`🔌️plugin/🖥️host/🦀️.rs`), plus `pump_process_worker_pool` inside `InteractiveInferenceJob::pump`.

### 4.2 `client-e2e`

`bun ./📜️script.ts client-e2e` under `framework-os-mcp` TS package with staged binary:

- **30 PASS** through transaction rollback, catalog health, note mutation, undo/redo, freshness
  (`note` + `wfc` wasm sha match committed descriptors).
- Then **`inference_run` / `tools/call` timed out at 240 s** — same hang class as the dedicated solve.
- Capture: `generated/gj2-client-e2e.txt`. **Not 38/38.**

### 4.3 WFC bitmap solve

`gj2-wfc-solve.ts`: headless `inference_run` of `s.wfc.bitmap.solve` with WI1/GJ1 hand-written
4×4→6×4 stripes snapshot.

| attempt | result |
| --- | --- |
| run2 (naive request) | first stdout line was `notifications/progress` @ 0.05 |
| run3 (skip notifications, 600 s) | **FAILED** after 609.8 s — no further stdout; only the initial progress ever arrived |

Captures: `generated/gj2-wfc-solve-run3.txt`, `gj2-wfc-solve-error.txt`.

Wasm inventory at probe time: shared `target/.../wasm-dev` (22 Sep 23:37, 129 383 519 B, matches
descriptor) vs newer `component-dev` (23 Sep 10:54, 129 648 400 B). A mutex rebuild+describe was
attempted; the inherited GJ1 script produced no capture under this slice (exit 101 / empty log) —
**gap remains: live solve not proven.**

## 5. G19 reach: algorithms / cad / two-way chat

| gap | status | evidence |
| --- | --- | --- |
| #1 two-way chat | **closed** (AC1) | `conversation_reply` in tools/list; `AgentReply` in bridge; `.mcp.json` has `conversation.write` |
| #2 stdio 68 algorithms | **reachable** | inference_list: 67 `stdio` rows on `s.stdio.gltf` (descriptor present) |
| #3 cad-extension inference | **reachable** | inference_list: `cad-extension-aec-building` / `building-structure-summary` on `s.cad.cad` |

No further roster code changes were required this slice — `contributed_inference_descriptors` already
walks extension contributions (see `🌉️mcp/💡️inference/🦀️.rs`).

## 6. Honest gaps

1. **`s.wfc.bitmap.solve` through `inference_run` does not finish** inside 240–600 s on this machine
   with the currently freshness-matched wasm-dev artifact. Host watchdog laws pass; live guest solve
   does not. Blocks claiming client-e2e 38/38 and a visible solved bitmap.
2. **WFC rebuild+describe through fleet mutex** not completed by this slice (script exit without
   capture). Peer `o1` held the wasm lock earlier today.
3. **Language-agnostic ensure laws** are Vitest (TS) against injectable staging — binary-gate JSON
   path cases remain the cross-platform fixture; no separate third-party binary oracle added beyond
   copying `process.execPath` as a stand-in executable.

## 7. Files changed

| path | change |
| --- | --- |
| `🌉️mcp/🟦️.ts` | `ensureMcpBinary`, content-hash, staging, notification-aware `request` |
| `📜️script.ts` | `runMcpOs` + `SetupScript.runFull` use `ensureMcpBinary` |
| `🌉️mcp/🧪️tests/🧪resolvemcpbinarypath/🟦️.ts` | ensure freshness laws |
| `.tmp-ticket/mcp-probe/probe.ts` | ensure + longer timeout + inference_list |
| `.tmp-ticket/wp-gj2/gj2-wfc-solve.ts` | dedicated solve probe |
| `.tmp-ticket/wp-gj2.md` → renamed audit twin | this report |

## 8. Captures (under `.tmp-ticket/wp-gj2/generated/`)

- `gj2-ensure-build.txt`, `mcp-probe/probe-transcript.json`, `gj2-client-e2e.txt`
- `gj2-host-laws*.txt`, `gj2-wfc-solve-run3.txt`, `gj2-wfc-solve-error.txt`
