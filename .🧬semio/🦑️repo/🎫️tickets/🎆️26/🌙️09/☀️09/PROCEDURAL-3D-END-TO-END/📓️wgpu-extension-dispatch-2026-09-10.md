# wgpu extension dispatch — React parity packet (steps 1–6)

Lane: execution of `📓️wgpu-boot-divergence-2026-09-10.md` §9 (ordered plan). wgpu only; React serve on 6018 was not touched. Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-10.

Live confirmation: `http://127.0.0.1:6118/?plugin=generation3d` probe **wgpu-boot-14**.

---

## 1. TL;DR

| question | verdict |
|---|---|
| Did wgpu boot complete? | **Yes.** `wgpu-worker boot_shell leave 2632 ms` then `boot-phase shell-boot 2634 ms` then `runtime-ready`. Not the boot-#6 ≥5 min freeze at 86 %. |
| Per-phase progress visible? | **Yes.** `createApp` open/leave, per-surface `render begin/leave` with ms, `boot-phase shell-boot:refresh-ui 1895 ms`, `shell-boot:select-program 2632 ms`. Nested names inherit the 900 s `shell-boot` ceiling (watchdog law; no extra kill clocks). |
| Did wgpu dispatch `InvokeExtension`? | **Wiring is live; this boot emitted 0 effects.** Every `renderSurface` logged `effects=0 tags=-`. Console count `invokeExtension` = 0, `extension completion submitted` = 0, `cabi_realloc` = 0. |
| Blank bodies / empty chrome? | **Yes — same shared reason as React.** Probe settle: `windows=[]`, `windowIds=[]`, `chrome.navbar=false`, `meshes=0`, `bodyText=""`. `push_contributions` saw empty/`[]` registry JSON and returned before paging. Sibling owns `windowUi` / contribution content. **This lane stops here.** |
| Paging contract? | **Ported.** Bridge imports `guestAnswerPages` + `GUEST_HOST_ANSWER_CEILING_BYTES` (8_388_608). 1 MiB answer → 15 prologue + 65536-byte terminal = 16 events. Unpaged answers would reintroduce boot #12 `cabi_realloc`. |

---

## 2. What landed (plan §9)

### 2.1 plugin-bridge.ts (step 1)

- `captureExtensionCompletion` + paged `http-chunk` prologue + terminal `completed` (React PluginRuntime twin).
- `dispatchInvokeExtension` looks up `loadedWgpuHandles`; missing `invoke` completes with paged fault `extension.invoke-unavailable` (still a dispatch).
- `renderSurface` returns leftover effects via `wireEffectToFriendly`; `renderDocument` JSON is `{ document, effects }` (`req` bigint → Number).
- `runQueuedTurn` logs leftover effect tags.
- `declareWgpuBootSubphase` → `globalThis.semioDeclareBootSubphase`.
- Exports: `wgpuGuestAnswerPages`, `wgpuHostAnswerCeilingBytes`, `WgpuPluginExtensionCompletion`.
- JS handle: `dispatchInvokeExtension(...)`.

### 2.2 Shell wgpu Rust (step 2)

- `settle_boot`: `refresh_ui` → `push_contributions` → `flush_deferred_actions` (contributions **before** first deferred `flowEvalTick`).
- Paged `setContributions` via `public_invocation_string_pages` (4096 char-cost, same as TS `PUBLIC_INVOCATION_STRING_BYTES`).
- `queue_host_effects` matches `Effect::InvokeExtension { req, extension_id, capability, request_json, .. }` (`#[non_exhaustive]`).
- wasm32: `spawn_app_task` → `plugin.dispatch_invoke_extension`.
- Unmatched effects: `[DEBUG] wgpu-shell effect dropped tag=…`.
- Per-window/panel `render begin/leave` + nested `shell-boot:render:<id>` declarations.
- Empty registry: `[DEBUG] contributions push skipped empty`.

### 2.3 ProgramBridge wgpu Rust (step 3)

- `render_with_document_js` parses `BrowserRenderEnvelope { document, effects }` and extends `refresh_effects`.
- `dispatch_invoke_extension` / `dispatch_invoke_extension_js` (`#[cfg(target_arch = "wasm32")]`).

### 2.4 Frame worker + browser worker (step 4)

- Frame-worker source: `bootPhaseStack`; on nested **leave**, re-**enter** parent so the watchdog never sees an undeclared Worker. `semioDeclareBootSubphase` on `globalThis`.
- Browser worker: `declare_boot_subphase` around `boot_shell` enter/leave.
- Regenerated frame-worker JS via `bun ./script.ts generate-frame-worker` (cache copy now contains `bootPhaseStack` / `semioDeclareBootSubphase`).
- **Did not** add a shorter ceiling than `shell-boot` (900_000 ms). Nested `shell-boot:*` uses the family-before-colon rule.

### 2.5 Language-agnostic + vitest laws (step 5)

New suite `engine/tests/wgpu-extension-dispatch/`:

| file | role |
|---|---|
| `laws.json` | 1 MiB → 15 prologue + 65536 terminal = 16 events; contributions-before-eval timeline; nested `shell-boot:render:…` ceiling 900000 |
| TypeScript twin | twins `guestAnswerPages` / `wgpuGuestAnswerPages`; contributions order; `evaluateBrowserBootLiveness` stays BUSY |

Registered in React `vitest.config.ts` (`engineSuite("wgpu-extension-dispatch")`), wgpu `vitest.config.ts` include list, and wgpu `script.ts` `runVitest` file list.

### 2.6 Bundle (step 6)

- Frame-worker regen (above).
- Trunk on **6118** (pid 45306) already rebuilt wasm after the Rust edits: cache `semio-framework-os-renderer-wgpu_bg.wasm` mtime 22:16:36 local, after Shell 22:08:52.
- Shared `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f5f6952-6c25-4056-a743-773b3668812e/scratchpad/target-wgpu-boot` — no new target clone.

---

## 3. Tests actually run

| command | result |
|---|---|
| `SEMIO_TEST_LEVEL=long bunx vitest run` (React engine config, filter wgpu-extension-dispatch) | **4 passed / 4** (1 file, 1.43 s) |
| same suite via wgpu `vitest.config.ts` from the rust package cwd | **4 passed / 4** (1 file, 1.96 s) |
| `cargo check -p semio-framework-os-renderer-wgpu --offline` native | **exit 0**, 25 warnings, 0 errors (15.68 s) — log `generated/wgpu-ext-cargo-native.txt` |
| `cargo check -p … --target wasm32-unknown-unknown --offline` | **exit 0**, 27 warnings, 0 errors (0.93 s) — log `generated/wgpu-ext-cargo-wasm32.txt` |

Laws checked in-process (not a mock of dispatch):

1. 1 MiB answer pages to 16 events; max block 65536; wgpu wrappers === framework helpers; ceilings 8_388_608 / 65_536.
2. Host-answer ceiling refuses an assembled answer past 8 MiB (size relation only).
3. Timeline `refresh-ui` → contributions pages → `flush-deferred` → `flowEvalTick`.
4. Nested phase `shell-boot:render:procedural-main` prices at 900000 ms; `evaluateBrowserBootLiveness` at 119 s elapsed → `terminate: false`.

---

## 4. Live probe wgpu-boot-14

```
cd <ticket>
bun browser-probe.ts --url=http://127.0.0.1:6118/?plugin=generation3d --label=wgpu-boot-14 --settle=240
```

Artifacts: `generated/probe-wgpu-boot-14-2026-09-10T20-19-50/` and `generated/wgpu-boot-14-*`.

| clock | event |
|---|---|
| 0.6 s | navigated HTTP 200, title `Semio Wgpu` |
| 1.2 s | `boot_shell enter` → `createApp open start plugin=procedural instance=1` |
| 1.9 s | `createApp open leave intakeSteps=0` → first `render begin surface=procedural-main` |
| 2.0–3.8 s | surfaces: `procedural-main` 926 ms (200_471 intake steps), `procedural-preview` 289 ms, `framework.panel.artifact` 119 ms, `catalogue` 304 ms, `inspection` 47 ms, `history` 185 ms, `framework.section.catalogue` |
| 3.8 s | `boot-phase shell-boot:refresh-ui 1895 ms` → `boot_shell leave 2632 ms` → `runtime-ready` |
| 3.9 s | **post-boot** `RuntimeError: memory access out of bounds` in `EngineSurfaceRegistry::default` / `EngineSurfaceSlot` `[256]` — **not** extension paging, **not** `cabi_realloc` |
| 9.3 s | probe settled (`windows=[]`, 1 canvas, 0 meshes). `settle=240` ended early because the harness saw a rendered shell. |

Console counts (full `console.jsonl`):

| needle | count |
|---|---|
| `wgpu-bridge renderSurface` | 16 |
| `invokeExtension` | **0** |
| `contributions push` | **0** (empty registry; skip path) |
| `cabi_realloc` | **0** |
| `flowEvalTick` | **0** |
| `boot-phase` | 3 (`refresh-ui`, `select-program`, `shell-boot`) |

`flush-deferred` ran after refresh (code order) but finished under the 1000 ms DEBUG threshold, so it did not print. No `effect dropped` lines — render turns produced no leftover host effects to queue.

---

## 5. Why invoke count is zero (not a missing dispatch)

Guest `flowEvalTick` mints `Effect::InvokeExtension` on the same turn. wgpu now:

1. Collects those leftovers from `renderDocument` / `runQueuedTurn`.
2. Extends `refresh_effects` in ProgramBridge.
3. Routes them in `queue_host_effects`.
4. Completes with **paged** `http-chunk` + `completed`.

This boot never reached (1): every surface turn reported `effects=0`. `flush_deferred_actions` therefore had no `flowEvalTick` `DispatchAction` to drain. Same shape as React when `shellState.windowUi` is `{}` and the contribution registry is `[]` — evaluation never arms, so the host has nothing to dispatch.

**Stop condition from the brief:** blank bodies for that shared reason → say so and stop; sibling owns window bodies / contribution content / eval arming.

The post-boot `EngineSurfaceRegistry` OOB is outside this packet (engine-canvas default of 256 slots). Not introduced by paging or `InvokeExtension` matching.

---

## 6. Watchdog

Declared phases only. Nested `shell-boot:*` re-enters the parent on leave. No second kill clock. `evaluateBrowserBootLiveness` on a nested child at 119 s stays BUSY.

---

## 7. Files touched

- `targets/wgpu/packages/rust/typescript/plugin-bridge.ts`
- `targets/wgpu/packages/rust/typescript/frame-worker.js` (generated)
- `targets/wgpu/frame-worker` source
- `targets/wgpu/browser-worker` Rust
- `targets/wgpu/packages/rust/vitest.config.ts`
- `targets/wgpu/packages/rust/script.ts`
- React `vitest.config.ts`
- `elements/ProgramBridge/targets/wgpu` Rust
- `elements/Shell/targets/wgpu` Rust
- `tests/wgpu-extension-dispatch/laws.json` + TypeScript twin
- this report

No `git commit` / checkout / stash / reset / worktree. Repo MCP unavailable (expected); ticket bookkeeping left to the owner lane.

---

## 8. Residual (not this lane)

- Empty `windowUi` / contribution registry / first `flowEvalTick` arming — sibling (React + shared plugin surfaces).
- Post-boot `EngineSurfaceRegistry::default` memory OOB — engine-canvas, not extension dispatch.
- Once contributions are non-empty, re-probe should show `[DEBUG] contributions push` with a page count, leftover `invokeExtension` tags, and `[DEBUG] wgpu-bridge extension completion submitted` with `pages ≥ 1`.
