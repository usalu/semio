# wgpu boot divergence — where `shell-boot` blocks, what React does that wgpu does not

Lane: read-only audit (resumes `📓️wgpu-shell-boot-silence-2026-09-10.md`, killed mid-write).
Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-10.

Live context: wgpu serve on `http://127.0.0.1:6118/?plugin=generation3d` (HTTP 200 at audit time).
Boot #6 (12:25): no watchdog kill; banner frozen at `shell-boot 86 %` for ≥5 min, no fault, no console.

---

## 1. TL;DR

| question | verdict |
|---|---|
| Where does `shell-boot` block? | One `await` in `🎞️frame-worker/🟦️.ts:490` → `boot_shell()` → `ShellState::boot().await` (`🌐️browser-worker/🦀️.rs:659`). Inside that: `create_app` (plugin-bridge lifecycle + retained UI intake) then `settle_boot()` → `refresh_ui()` (one `renderDocument` per live window, panel tab, and catalogue section — each a multi‑hundred‑thousand‑step retained intake). **Not** an infinite `poll` waiting on `Event::Completed` during boot. |
| Extension-await hypothesis | **Refuted as the boot blocker.** `flowEvalTick` is never dispatched during boot (deferred actions are queued but not flushed; `InvokeExtension` effects are dropped). **Confirmed as a critical post-boot gap** — even if boot completes, evaluation cannot work until extension dispatch + paged answers exist. |
| Paged extension-answer contract | **Not shared.** wgpu `🐚️plugin-bridge.ts` has zero imports of `guestAnswerPages` / `GUEST_HOST_ANSWER_CEILING_BYTES`. Wiring extension completion without paging would reintroduce the boot #12 `cabi_realloc` abort (`📓️extension-result-realloc-2026-09-10.md`). |

---

## 2. Why `86 %` is silent (inherited from incomplete lane)

| site | role |
|---|---|
| `🌐️browser-worker/🦀️.rs:643` | phase 6 → `{ stage: "shell-boot", progress: 0.7, shell_boot: true }` |
| `🎞️frame-worker/🟦️.ts:488-490` | `progress(..., 0.65 + 0.7 × 0.3) = 0.86`; then `await monitoredSuspension("shell-boot", () => bootstrap.bootShell())` |
| `🌐️browser-worker/🦀️.rs:655-659` | `boot_shell` → `ShellState::boot().await` — **single opaque await** |

Watchdog law (`📓️wgpu-boot-watchdog-2026-09-10.md`): declared `shell-boot` inside 900 000 ms ceiling → **BUSY, never kill**. Silence is correct; missing sub-phase instrumentation is the defect.

---

## 3. `ShellState::boot()` step trace (wgpu, current tree)

```
boot_shell()                                                          [🌐️browser-worker/🦀️.rs:655-659]
  ShellState::boot()                                                  [🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3138-3205]
    plugin_faults.clear()
    select_boot_program / host_config branch                          [:3171-3183 / :3141-3169]
    open_boot_instance(plugin, app)                                   [:3124-3135]
      ProgramBridgeEntry::create_app                                  [🌉️ProgramBridge/…/🧊️wgpu/🦀️.rs:437-443]
        create_app_js → plugin-bridge loadPluginModule.createApp      [🐚️plugin-bridge.ts:739-772]
          registry.activate + lifecycle.open + settleInstanceLifecycle
          WgpuOwnedUiInstanceRoute.accept (retained UI intake, per patch)
    session := ActiveSession { view_state.contributions_json: None }  [:3190-3202]
    settle_boot()                                                     [:3205]
      sync_dock / sync_session_chrome                                 [:3212-3213]
      refresh_ui().await                                              [:3218 / :3315-3409]
        live_view_state — sets contributions_json in view ONLY        [:3259-3271]
        FOR each dock window:
          render_with_document(..., Some(&mut refresh_effects))     [:3334]
            render_with_document_js → renderDocument                  [ProgramBridge :717-729]
              renderSurface — surface-visible turns + intake + project  [🐚️plugin-bridge.ts:648-663]
        FOR each panel tab with body_key: same                        [:3350]
        refresh_app_catalogue — another render_with_document          [:3432-3456]
        queue_host_effects(refresh_effects)                           [:3407 / :3549-3606]
        settle_surface_faults
```

**Blocking await:** the outer `boot().await` does not return until `refresh_ui()` finishes all `render_with_document` calls. Each call can hold the worker for minutes (retained intake steps × surfaces × hidden-tab timer throttling — see `📓️wgpu-intake-budget-2026-09-10.md` §1).

**Not on this path during boot:** `flush_deferred_actions()` (only `:5846`, `:6438` — frame loop / tree selection, never `settle_boot`).

---

## 4. React equivalent (gets past this work)

| step | React (`🏛️ShellHost/🟦️.tsx` + `🔌️PluginRuntime/🟦️.tsx`) | wgpu |
|---|---|---|
| Open app | `loadPluginModule` → same shard lifecycle (`PluginRuntime/🟦️.tsx:2197-2226`) | `🐚️plugin-bridge.ts:748-762` — same shape |
| First UI refresh | `refreshUi` → `submitTurn` surface-visible batch + `settlePluginTurn` + **`routeHostEffects`** (`:2296-2327`) | **No `refreshUi` on handle.** Shell calls `renderDocument` per surface (`ProgramBridge :717`) — no effect routing |
| `pending_effects` / `flowEvalTick` | `response.requestedEffects` → `applyHostEffects` (`ShellHost :4311`) | Effects from render turns **never collected** (`render_with_document_js` `:712-715` explicitly leaves `refresh_effects` empty) |
| `setContributions` (paged, locale) | Pushed after refresh (`ShellHost :4320-4360`), `resolvedTargetViewState` carries locale + window roster (`:3925`) | **`contributions_json` only in `live_view_state` (`Shell :3263`) — no `setContributions` command ever sent** |
| Contributions re-arm | React path re-arms via `applyHostEffects` chain (`📓️contributions-rearm-2026-09-10.md`) | No equivalent |
| `InvokeExtension` | `dispatchInvokeExtensionEffect` (`ShellHost :5011-5027`) → `captureExtensionCompletion().complete()` with **`guestAnswerPages`** (`PluginRuntime :2331-2379`) | `queue_host_effects` **`_ => {}` drops it** (`Shell :3604`); bridge has **no** `captureExtensionCompletion` |
| Extension result paging | `GUEST_HOST_ANSWER_CEILING_BYTES` + `guestAnswerPages` (`⏱️trace/🧮️memory/🟦️.ts`, `PluginRuntime :2343-2358`) | **Absent** — would abort on first large answer |
| `applyHostEffects` loop | Full host effect algebra incl. spawn, navigate, media, extension | `queue_host_effects` handles only `SetActiveUtility`, `Navigate`, `LoadDocument`, `DispatchAction`, `RequestMediaFrames` (`Shell :3551-3604`) |
| Post-contributions eval | Registry installed → `flowEvalTick` chain runs → extension dispatch answers `Event::Completed` | Registry never installed at boot; even if boot finishes, chain cannot evaluate |

---

## 5. Divergence table (React does X @ file:line / wgpu does not)

| # | React | wgpu gap |
|---|---|---|
| D1 | **`refreshUi` batches surfaces + returns `requestedEffects`** — `PluginRuntime/🟦️.tsx:2296-2327` | No handle method; `ProgramBridge` calls `renderDocument` per surface (`🌉️ProgramBridge/…/🧊️wgpu/🦀️.rs:717-729`) |
| D2 | **`applyHostEffects(requestedEffects)`** after refresh — `ShellHost/🟦️.tsx:4311` | `refresh_effects` sink unfilled in `render_with_document_js` (`:712-715`); `queue_host_effects` never sees render-time effects |
| D3 | **Paged `setContributions` push** (73 pages) — `ShellHost/🟦️.tsx:4320-4360` | Missing entirely in `ShellState::refresh_ui` / `settle_boot` |
| D4 | **`contributions_json` + locale + `windowInstances` in push viewState** — `ShellHost/🟦️.tsx:4258-4266`, `resolvedTargetViewState :3925` | `live_view_state` sets `contributions_json` (`Shell/🧊️wgpu/🦀️.rs:3263`) but never dispatches; boot session starts with `contributions_json: None` (`:3195`) |
| D5 | **`dispatchInvokeExtensionEffect`** — `ShellHost/🟦️.tsx:1688-1703`, `:5011-5027` | No call site in wgpu tree; `queue_host_effects` catch-all `:3604` |
| D6 | **`captureExtensionCompletion` + paged `Event::Completed`** — `PluginRuntime/🟦️.tsx:2331-2379` | Not exported from `🐚️plugin-bridge.ts`; `WgpuPluginHandle` interface (`:584-597`) lacks it |
| D7 | **`guestAnswerPages` / `GUEST_HOST_ANSWER_CEILING_BYTES`** — `⏱️trace/🧮️memory/🟦️.ts:18-26` | Not imported anywhere under `🎯️targets/🧊️wgpu/` |
| D8 | **`flush_deferred_actions` during effect closure** — implicit via `applyHostEffects` re-entrancy | Boot queues `DispatchAction` to `deferred_actions` (`Shell :3580-3581`) but **`settle_boot` never flushes** |
| D9 | **Diagnostics: `[DEBUG] contributions push`** — `ShellHost/🟦️.tsx:4336-4341` | No boot-phase logging in shell or bridge |

---

## 6. Extension hypothesis — confirm / refute

**Hypothesis:** guest's first `flowEvalTick` awaits `Event::Completed`; wgpu never dispatches/answers extension invocations → `boot` never returns.

**Mechanism (SDK):** `flowEvalTick` emits `Emit::extension_invocations` → `queue_extension_invocation` mints `Effect::InvokeExtension` in the **same turn's** `turn-result.effects` (`⚛️reactor/🦀️.rs:708-718`). The turn **completes**; continuation parks until `Event::Completed` / `HttpChunk` pages (`⚛️reactor/🔄️turn/🦀️.rs:542-573). The guest does **not** block inside `invoke_extension` import for this path.

**Refuted as boot blocker:** during `ShellState::boot()`:

1. `flowEvalTick` is only armed via `pending_effects` → shell `applyHostEffects` (`generation3d/✏️editor/🦀️.rs:1657-1669` + React `ShellHost :4311`). wgpu never runs that loop at boot.
2. `InvokeExtension` effects from any render turn are **discarded** (`Shell :3604`) and **not collected** from `renderDocument` (`ProgramBridge :712-715`).
3. `deferred_actions` holding `DispatchAction{flowEvalTick}` are **not flushed** in `settle_boot`.

**Confirmed as critical gap:** once boot completes (or if an action path runs `flowEvalTick`), wgpu has **no** extension dispatch and **no** paged completion — evaluation stays faulted exactly like React boot #12 before `📓️extension-result-realloc-2026-09-10.md`.

---

## 7. Paged extension-answer contract

| layer | React (fixed) | wgpu |
|---|---|---|
| Schema | `hostAnswerCeilingBytes: 8388608` in `⏱️trace/🧮️memory/🧬️schema/🔣️.json` | Same schema; **wgpu bridge does not read it** |
| TS constants | `GUEST_HOST_ANSWER_CEILING_BYTES`, `guestAnswerPages()` — `⏱️trace/🧮️memory/🟦️.ts:18-26` | **Not imported** in `🐚️plugin-bridge.ts` |
| Completion submit | `http-chunk` prologue + terminal `completed` — `PluginRuntime/🟦️.tsx:2355-2358` | **Missing** — would submit whole pack → `cabi_realloc` abort on answers >64 KiB contiguous |
| Guest receive | `append_extension_response_page` — `⚛️reactor/🔄️turn/🦀️.rs:570-573` | Guest side ready; **host never sends pages** |

**Verdict:** wiring extension dispatch without porting `captureExtensionCompletion` + `guestAnswerPages` would reintroduce the unbounded realloc abort.

---

## 8. Instrumentation proposal (watchdog-safe)

Law (`📓️wgpu-boot-watchdog-2026-09-10.md` §3): **declare before block**, `boot-phase enter/leave` with elapsed; never add a second kill clock inside a declared busy phase.

### 8.1 Frame worker — sub-phases under `shell-boot`

Keep the existing `monitoredSuspension("shell-boot", …)` envelope (900 000 ms ceiling). **Inside** `boot_shell` / bridge, add nested declarations via a new helper (e.g. `declareBootSubphase(name)`) that posts `boot-phase` with names like:

| sub-phase | when |
|---|---|
| `shell-boot:select-program` | before `select_boot_program` |
| `shell-boot:create-app:<plugin>` | before `create_app` |
| `shell-boot:refresh-ui` | before `refresh_ui` |
| `shell-boot:render:<surfaceId>` | before each `render_with_document` |
| `shell-boot:contributions-push` | before paged `setContributions` (once implemented) |
| `shell-boot:flush-deferred` | before deferred action drain |

Each posts `enter` before work and `leave` with `elapsedMs` in `finally`. Log when `elapsedMs > 1000`: `[DEBUG] boot-phase <name> <ms> ms`.

**Do not** add a watchdog shorter than `shell-boot`'s declared ceiling for these sub-phases unless each gets its **own** entry in `FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs` (same file as parent law).

### 8.2 `🐚️plugin-bridge.ts`

| log site | content |
|---|---|
| `createApp` open start/leave | plugin id, instance id, intake steps from `WgpuUiIntakeCursor.steps` |
| `renderSurface` per turn | surface id, turn index, `current.effects.length`, tags of effects (count `invokeExtension` / `dispatchAction`) |
| `runQueuedTurn` / `performInvocation` | `[DEBUG] wgpu-bridge effects leftover N tags=…` |
| `submitTurn` fault | existing trap log + phase name |

### 8.3 `ShellState` (Rust)

| log site | content |
|---|---|
| `boot()` | `[DEBUG] wgpu-shell boot: program=<id> app=<id>` |
| `refresh_ui` each window/panel | `[DEBUG] wgpu-shell render begin surface=<id> body=<key>` + `leave` with ms |
| `queue_host_effects` | `[DEBUG] wgpu-shell effect dropped tag=<variant>` for `_ => {}` path (until fixed) |
| `setContributions` (once added) | mirror React `[DEBUG] contributions push` JSON line |

### 8.4 Progress banner (optional, non-law)

Post `boot-progress` substeps inside `shell-boot` only for **coarse** milestones (not per intake step): e.g. `shell-boot 86 % · create-app done`, `shell-boot 86 % · window 2/4`. Keeps banner at 86 % base but adds text — avoids implying fake percentage progress through intake.

---

## 9. Ordered fix plan (execution ownership)

| order | owner file(s) | work |
|---|---|---|
| **1** | `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` | Port from `PluginRuntime/🟦️.tsx`: `captureExtensionCompletion` (with `guestAnswerPages`, `GUEST_HOST_ANSWER_CEILING_BYTES`), expose on `WgpuPluginHandle`; drain/reconcile effects in `renderSurface` / `runQueuedTurn` like `retainedUiRefreshEffects`. |
| **2** | `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | Port `ShellHost` contributions push (`setContributions` pages, locale, window roster) into `refresh_ui` or `settle_boot`; extend `queue_host_effects` with `InvokeExtension` dispatch calling into TS bridge (or inline Rust mirror of `dispatchInvokeExtensionEffect`); call `flush_deferred_actions` at end of `settle_boot` after effects applied. |
| **3** | `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | Fill `refresh_effects` in `render_with_document_js` from bridge-returned effect list (requires bridge API change from step 1). |
| **4** | `🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` + `🎞️frame-worker/🟦️.ts` | Sub-phase `boot-phase` declarations + `[DEBUG]` logs (§8); extend `FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs` only if a sub-phase needs a ceiling **lower** than `shell-boot`. |
| **5** | `🧪️tests/🔬️engine-contract/🟦️.ts` (or new wgpu boot contract test) | Law: wgpu bridge submits paged completion for 1 MiB answer; law: contributions push runs before first `flowEvalTick`; replay boot timeline against `evaluateBrowserBootLiveness`. |
| **6** | Bundle regen | `generate-frame-worker` / trunk serve on 6118 — no competing cargo against shared target. |

**Dependency:** steps 1–3 are one logical “React parity” packet; step 4 can land in parallel (diagnostics only) but should merge before the next live boot attribution.

---

## 10. Boot #6 attribution (best current read)

With intake ceiling fixed (`📓️wgpu-intake-budget-2026-09-10.md`) and watchdog declaring `shell-boot` busy (`📓️wgpu-boot-watchdog-2026-09-10.md`), boot #6's ≥5 min silence at 86 % is **consistent with**:

- `create_app` lifecycle intake + **N ×** `refresh_ui` `renderDocument` drives (flow + preview + catalogue + panel tabs), each up to **~163 840 000** step ceiling with **4096**-step frame slices (`🐚️plugin-bridge.ts:232`, `:306-307`), amplified by hidden-tab macrotask throttling (`:241-249`).

**Not consistent with** an unanswered extension `req` blocking `boot().await` — that path is not exercised until something dispatches `flowEvalTick` and something handles `InvokeExtension`.

Runtime confirmation still owed: sub-phase logs from §8 on the next boot.

---

## 11. Files read (audit)

- Ticket: `📓️status.md`, `📓️wgpu-shell-boot-silence-2026-09-10.md` (partial), `📓️wgpu-boot-watchdog-2026-09-10.md`, `📓️wgpu-intake-budget-2026-09-10.md`, `📓️wgpu-shell-boot-2026-09-10.md`, `📓️extension-result-realloc-2026-09-10.md`, `📓️contributions-delivery-2026-09-10.md`, `📓️contributions-rearm-2026-09-10.md`
- Source: `🌐️browser-worker/🦀️.rs`, `🎞️frame-worker/🟦️.ts`, `🐚️plugin-bridge.ts`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`, `🏛️ShellHost/🟦️.tsx`, `🔌️PluginRuntime/🟦️.tsx`, `⏱️trace/🧮️memory/🟦️.ts`, `⚛️reactor/🦀️.rs`, `⚛️reactor/🔄️turn/🦀️.rs`, `generation3d/✏️editor/🦀️.rs`, `flow-eval-tick/🦀️.rs`

No files modified except this report. No cargo/nx/trunk. `curl http://127.0.0.1:6118/` → 200.
