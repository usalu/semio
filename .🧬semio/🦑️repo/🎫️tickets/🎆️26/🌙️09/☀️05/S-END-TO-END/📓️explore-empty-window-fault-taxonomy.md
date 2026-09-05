# 📓️ Framework-level fault taxonomy — "window renders empty / app dead in the shell"

Read for context first: `26/09/01/PROCESS-END-TO-END/🧪️runtime-verification.md` and `…/📓️status.md`.
Everything below was re-read against today's tree (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
39 084 lines) on 2026-09-05, not copied from the peer ticket's notes — several line numbers have
drifted since that ticket's snapshot (same-day commits), and are corrected here.

## 0. Scope note — plugin host file has none of this

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` was grepped for every pattern below
(`cleanup faulted`, `interactive-job.`, `missing-owned-reducer`, `missing-factory`,
`BatchOnlyPendingRewrite`, `plugin_internal_fault`) and matched **zero** times. All instance-fault
emission for "window renders empty" lives exclusively in `🔌️plugin/🦀️.rs`; the host file only does wire
transport / shard scheduling. `plugin_internal_fault` (the helper every "runtime … faulted" message
goes through) is defined once, at `🔌️plugin/🦀️.rs:29063`, private to that file.

---

## 1. Every fault-store/emit site for an instance

### 1a. The `RUNTIME_MAINTENANCE_FAULT` family (the "runtime live cleanup faulted for instance N" string)

`const RUNTIME_MAINTENANCE_FAULT: u8 = 3;` at `🔌️plugin/🦀️.rs:28722`. It is **stored or returned at 12
sites** on the native (non-`wasm32`-gated) cleanup path, plus **1 more** on a `#[cfg(target_arch =
"wasm32")]`-gated path the peer ticket's "twelve" count did not include (worth folding in — see 1a-13).
All 13 collapse to the identical decoded string at the one read site, `plugin_step_live_cleanup`
(`:30322`): `Err(plugin_internal_fault(format!("runtime live cleanup faulted for instance {}", cell.id)))`,
which is `Fault { origin: Plugin, code: "plugin.internal", message: "runtime live cleanup faulted for
instance N" }` — see §3, none of the 13 conditions is distinguishable from the wire fault alone.

| # | line | condition | function |
| - | ---- | --------- | -------- |
| 1 | `:29654` | `runtime_live_cleanup_nonterminal_status`: the pump's zero-progress close-step counter reached `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` — N consecutive turns made no progress draining a `PluginCloseStep::Pending{0,0}`/`Blocked` | `runtime_live_cleanup_nonterminal_status` |
| 2 | `:29666` | `runtime_callback_clock_begin()` returned `Err` — no monotonic clock reading at turn start | `run_runtime_live_cleanup_turn` |
| 3 | `:29672` | `runtime_callback_clock_elapsed()` returned `Err`, **or** `semio_framework_trace::interactive_step_contract_violated(elapsed)` is true — elapsed µs ≥ `INTERACTIVE_STEP_CEILING_US` (8 000). **This is the 8 ms-ceiling branch.** Wraps the *entire* turn: pump-lock acquire + `runtime_live_cleanup_pump_one` + status math | `run_runtime_live_cleanup_turn` |
| 4 | `:29678` | `pump.closing` is true — a close is already in flight for this instance. **Reuses `RUNTIME_MAINTENANCE_FAULT` as the "still closing" signal, not a real crash** (see §3 caveat) | `runtime_live_cleanup_pump_one` |
| 5 | `:29684` | `pump.rejected` finished draining (`terminal_is_empty()`) — a `BatchJobSession::try_new` that was rejected earlier has now closed out | `runtime_live_cleanup_pump_one` |
| 6 | `:29699` | after an outcome close-step drained, not terminal, `session.resume()` returned `Err` | `runtime_live_cleanup_pump_one` |
| 7 | `:29713` | terminal session finished draining (`terminal_is_empty()`) and `pump.faulted` was set by a prior `StepOutcome::Fault`/`Cancelled` — propagates a real prior fault | `runtime_live_cleanup_pump_one` |
| 8 | `:29734` | `pump.session` is `None` where a session was expected (defensive/unreachable-shaped) | `runtime_live_cleanup_pump_one` |
| 9 | `:29736` | **`session.step().is_err()`** — the `BatchJobSession`'s own step failed. **This is the stale-core/wrong-ABI candidate**: a component whose exports no longer match the host's expectations traps here | `runtime_live_cleanup_pump_one` |
| 10 | `:29738` | `session.checkout_outcome()` returned `false` | `runtime_live_cleanup_pump_one` |
| 11 | `:29739` | `session.checked_out_job_mut()` returned `None` | `runtime_live_cleanup_pump_one` |
| 12 | `:29745` | `session.take_outcome()` returned `None` | `runtime_live_cleanup_pump_one` |
| 13 (wasm-only, **not in the peer's count of 12**) | `:30308` | `pump_runtime_live_cooperative_turn` (`#[cfg(target_arch = "wasm32")]` only): `semio_framework_job::default_now_ms()` returned `None` | `pump_runtime_live_cooperative_turn` |

The read/decode site: `:30322` inside `plugin_step_live_cleanup` — `cell.maintenance_status.load() ==
RUNTIME_MAINTENANCE_FAULT` ⇒ `Err(plugin_internal_fault("runtime live cleanup faulted for instance
{id}"))`. Adjacent sibling for the OTHER cleanup family (`RUNTIME_CLOSE_FAULT`, a **separate** atomic on
a **separate** struct, `RuntimeCloseWorkerState`) reads at `:30281`: `Err(plugin_internal_fault(format!(
"runtime close cleanup faulted for instance {instance_id}")))` — a distinct message string
("close cleanup" vs "live cleanup"), so this one at least names which of the two supervisory loops
failed, unlike the 13 sites above which all fold into one string.

**What the user/test sees**: an app-level `Fault` with `origin: "plugin"`, `code: "plugin.internal"`,
`severity: "error"`, `message: "runtime live cleanup faulted for instance N"`. It surfaces to the
console (`[DEBUG] render failed …` / `[DEBUG] readConflicts failed …`, see §2) — never as a distinct
on-screen status, because nothing downstream branches on `origin`/`code` (§2, §3).

### 1b. Typed-command / tool-proof faults (`interactive-job.*`) — a genuinely dead command, not an empty body

These are `FaultOrigin::Framework` (not `Plugin`), a different family from §1a, and they DO discriminate
by condition (though several share one fault **code** and differ only in the formatted message):

| code | line(s) | condition |
| ---- | ------- | --------- |
| `interactive-job.not-ui-safe` | `:12045` (`validate_ui_dispatch_classification`, `:12041`) | UI dispatch rejects an action/command whose declared `InteractiveJobClassification` is anything but `Migrated` — i.e. `BatchOnlyPendingRewrite`, `ForbiddenFromUi`, `Deleted`, `Unclassified`. Message names the classification: `"UI dispatch rejected {owner}:{id} with interactive-job classification {classification:?}"`, so `BatchOnlyPendingRewrite` is legible in the text even though the code is shared |
| `interactive-job.missing-owned-reducer` | `:19376`, `:23048` | a typed command has a generic (bounded) tool proof registered but **no exact app-owned retained decoder/reducer factory** — i.e. the app declared the tool id but never called `.with_factory_type::<Owner, Factory>()` |
| `interactive-job.missing-factory` | `:19378`, `:19394` | a typed command / host-configuration command has **no** controller/owner/factory/tool/schema proof at all — nothing registered it |
| `interactive-job.catalog-authority` | `:12232` (`tool_job_registration`, `:12185`) | the tool-proof catalog's `migrated` set does not exactly join `generated_ids` — the `interactive-job.catalog-authority: tool factory proof rejected tool '…'` hard-abort the 09-02 note hit when 32/33 tool ids were declared as commands instead of actions and `AppBuilder::action_interactive_job` (`:5197`) silently no-op'd on them |
| `interactive-job.catalog-incomplete` | `:12259` | a migrated generated command lacks its exact owner-local bounded reducer proof |

`BatchOnlyPendingRewrite` itself is only ever a **value** of `InteractiveJobClassification` (defined
outside this file, in `semio_framework`), tested at `:7291`/`:7308` — it has no dedicated fault code of
its own; it reaches the user through `interactive-job.not-ui-safe`'s formatted message.

**What the user/test sees**: this class of fault fires at command/action **dispatch time**, before any
render — it looks like "clicking a button/running a command does nothing or throws," not "window mounts
with an empty body." The 09-02 `catalog-authority` hit was a **hard abort at test-fixture construction**
(`interactive-job.catalog-authority`), never reaching the shell at all.

---

## 2. How React `ShellHost` receives and renders these faults

File: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
(≈7 400 lines).

### 2a. The wire decode is generic — code/origin are parsed but never branched on

`@semio-tech/framework-os` (`🧰️framework/🛍️products/💻️os/🟦️.ts:2097-2109`) exposes:
```ts
decodeFaultFromWire(faultBytes, decodePackValue): Fault | null   // → { origin, code, severity, message }
faultDisplayMessage(faultBytes, decodePackValue): string          // → `${code}: ${message}`
```
Grepping `ShellHost/🟦️.tsx` and `🔌️PluginRuntime/🟦️.tsx` for `.origin`/`FaultOrigin` returns **zero**
hits. Every catch site in `ShellHost` extracts only `error.message` (or `String(error)`) — the `code`
and `origin` fields that the wire format actually carries are decoded and then dropped on the floor.
This is the direct answer to "is the fault logged with a discriminating code": **the wire format has
one (`code`), the plugin side already varies it usefully for §1b, but the React shell never reads it.**

### 2b. The three concrete paths, and what each produces on screen

| path | site | on failure | visible effect |
| ---- | ---- | ---------- | --------------- |
| session `refreshUi` (the main per-window body/panel builder) | `ShellHost/🟦️.tsx:3037-3046` | `console.error("[DEBUG] render failed", renderError)` **and** `dispatch({ type: "SET_ERROR", value: renderError.message })` | The **whole canvas** (not just one window body) is replaced by `<p role="alert" data-semio-os-shell-error="">{error}</p>` at `ShellHost/🟦️.tsx:6989-6992` — a full-canvas red-text takeover, not a silently-empty body |
| spawned-window `refreshUi` | `ShellHost/🟦️.tsx:3057-3065` | `console.error("[DEBUG] spawned render failed", renderError)`, `dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null })` | that one spawned window's UI reverts to `null` → its body renders empty, no visible error text |
| `readConflicts` | `ShellHost/🟦️.tsx:4917-4928` | `console.error("[DEBUG] readConflicts failed", commandError)` only — **no dispatch at all** | silently swallowed; Conflicts panel just never populates |

The literal console string the peer ticket quoted — `[DEBUG] PluginRuntime: turn failed for actor
process#1` — could **not** be found verbatim anywhere in current `.ts`/`.tsx` source (grepped
`🧰️framework` for `"turn failed for actor"`, zero hits). The nearest living analogue is
`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1878`:
```ts
const onTurnError = options.onTurnError ?? ((actorId, error) => console.error(`[DEBUG] ActivationRegistry: turn failed for ${actorId}`, error));
```
— note the label is `ActivationRegistry`, not `PluginRuntime`, and the template has no literal word
`actor` before the id. The React shell's own `ActivationRegistry` (constructed at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:104`)
passes no `onTurnError` override, so it uses exactly this default. **Either the quoted line predates a
rename (`PluginRuntime`→`ActivationRegistry`, consistent with the file's own doc-comment history of a
`PluginRuntime`→kernel-`ActivationRegistry` port) or it comes from a call site this exploration did not
find** — flagging as an open discrepancy rather than asserting either way. Critically, `onTurnError`'s
default is **`console.error`-only**: a maintenance-turn fault (the whole `RUNTIME_MAINTENANCE_FAULT`
family in §1a, which fires from a background maintenance turn, not a user-initiated `refreshUi`) never
reaches `SET_ERROR` through this path — it can leave a window's body silently empty with nothing but a
console line, whereas a `refreshUi`-path fault (2b row 1) blanks the *entire* canvas with visible red
text. **This split is the mechanical reason "mounts fine, body empty, nothing on screen" and "canvas
replaced by an error banner" are different bugs today, driven by which of the two call paths the same
underlying `RUNTIME_MAINTENANCE_FAULT` happens to be observed through.**

### 2c. Plugin-crash / quarantine is a fourth, wholly separate status axis

`ShellHost/🟦️.tsx:1827-1828` calls `resolvePluginCanvasStatus` (defined
`…/🐚️Shell/🟦️.tsx:130`, signature `(hasSession, error, pluginStatus?, supervisor?) => "loading" |
undefined`) and, earlier in the same `canvas` `useMemo` (`ShellHost/🟦️.tsx:~6969-6986`), a
`pluginSupervisorById[primaryPluginId] === "crashed" | "quarantined"` check renders a
`<PluginRecoveryPanel>` instead of the canvas. Supervisor state is set to `"crashed"` at
`ShellHost/🟦️.tsx:1910` (module load itself failed) and `:2037` (an exception during
`establishPrimarySession`) — i.e. **only** load/boot-time failures reach "crashed"; a fault raised later
by the maintenance loop (§1a) or a `refreshUi` failure (2b) never touches `pluginSupervisorById`, so an
app that mounted fine and then faulted stays `"running"` with an empty/errored canvas, it does not fall
back to the recovery panel.

---

## 3. Discriminators a smoke harness can read today — and where to add one

**Today: none of (a)-(d) below are distinguishable from the decoded fault alone**, because every one of
the 13 §1a sites (and the ABI-trap case) funnels through the same `plugin_internal_fault(format!(
"runtime live cleanup faulted for instance {}", cell.id))` at `🔌️plugin/🦀️.rs:30322`, producing the
identical `{origin: "plugin", code: "plugin.internal", message: "runtime live cleanup faulted for
instance N"}` regardless of cause. This is exactly the ambiguity `26/09/01/PROCESS-END-TO-END`'s
`🧪️runtime-verification.md` already names for two of the twelve (now 13) branches; it holds for all of
them, not just those two:

| target discriminator | current signal | verdict |
| --- | --- | --- |
| (a) stale core wasm vs current ABI | `session.step().is_err()` at `:29736` → same string as everything else | **not discriminable** |
| (b) interactive ceiling overrun | `interactive_step_contract_violated(elapsed)` at `:29672` → same string | **not discriminable** |
| (c) missing owned reducer/factory (`BatchOnlyPendingRewrite`/no `factory_type`) | `interactive-job.missing-owned-reducer` / `interactive-job.missing-factory` / `interactive-job.not-ui-safe` (§1b) | **discriminable** — different fault **code**, and the classification name (`BatchOnlyPendingRewrite`) is legible in the `not-ui-safe` message text. This class never produces "empty body" though — it kills the command/action dispatch outright, before any render |
| (d) plugin install failure | `pluginStatusById[id] === "failed"` + `pluginSupervisorById[id] === "crashed"` (`ShellHost/🟦️.tsx:1908-1910`) | **discriminable** — a distinct status axis, renders `<PluginRecoveryPanel>` |
| (e) app created but first frame never produced | no dedicated signal found. A spawned window whose `refreshUi` throws sets its UI to `null` (2b row 2) — indistinguishable from "app created, first frame legitimately empty" (e.g. no selection) without inspecting `console.error` output for `"[DEBUG] spawned render failed"` | **not discriminable from UI state alone**; console-only |

**Where to add a code, and the minimal schema-first change**, for (a)/(b), the two that matter most
(they are the two actually reachable through "app mounts, body stays empty" per the peer ticket):

1. `🔌️plugin/🦀️.rs:29736` (`if session.step().is_err() { return RUNTIME_MAINTENANCE_FAULT; }`) — capture
   the `Err` value (currently discarded with `.is_err()`) and thread a distinct maintenance-status byte,
   e.g. `RUNTIME_MAINTENANCE_FAULT_ABI` alongside the existing `RUNTIME_MAINTENANCE_FAULT`, so
   `plugin_step_live_cleanup` (`:30322`) can emit a **different fault code**,
   `"plugin.internal.abi-mismatch"` (or similar), instead of the generic `"plugin.internal"`.
2. `🔌️plugin/🦀️.rs:29672` — when `interactive_step_contract_violated(elapsed)` is the reason (not the
   `elapsed.is_err()` branch, which is a real clock failure), store a third distinct status,
   `RUNTIME_MAINTENANCE_FAULT_CEILING`, and have `plugin_step_live_cleanup` emit
   `"plugin.internal.interactive-ceiling"` with the measured `elapsed` µs embedded in the message.
3. Both need `RuntimeAppCell`'s `maintenance_status: AtomicU8` widened from the current 3-4 value enum
   (`READY`/`QUEUED`/`RUNNING`/`FAULT`) to carry the extra fault subtypes — schema-first change is
   widening that enum (today it's a bare `u8` with `const` values at `:28722` and neighbors, not a real
   Rust `enum`; making it one first, with the new variants, is the honest fix rather than layering more
   magic constants) and updating the one read site at `:30322` to match on the new variants.
4. On the wire/TS side, `Fault.code` is already a free-form string (`FaultCode::new(...)`), so no wire
   schema change is needed once the Rust side emits a distinct code — `decodeFaultFromWire`
   (`🧰️framework/🛍️products/💻️os/🟦️.ts:2097`) already round-trips whatever code string is set. The
   missing half is entirely client-side: **`ShellHost` needs to branch on `fault.code`** (today it only
   reads `.message`, see §2a) to render/log the two cases differently — that is a pure TS change, no
   schema migration.

---

## 4. The 8 ms interactive ceiling

- **Definition**: `pub const INTERACTIVE_STEP_CEILING_US: u64 = 8_000;` —
  `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:90`. Checked by
  `pub fn interactive_step_contract_violated(elapsed_us: u64) -> bool { elapsed_us >=
  INTERACTIVE_STEP_CEILING_US }` (`:93-95`).
- **Configurability**: none. Repo-wide grep for `INTERACTIVE_STEP_CEILING_US` (all `.rs`, excluding
  `target/`/`node_modules/`) finds exactly 3 defining/using files
  (`⏱️trace/🦀️.rs`, `🧵️job/🦀️.rs:7,568`, `🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1778` — a comment) plus one stale
  built-dist copy under `🌎️hub/…/📤️dist/`. No `#[cfg(...)]`, no `env::var`, no feature flag anywhere
  touches it — confirms the peer ticket's "exhaustive grep" claim independently. It is one process-wide
  hard constant shared by every interactive site regardless of dev/ship build, app, or plugin.
- **The nested budget it wraps**: `run_runtime_live_cleanup_turn` (`🔌️plugin/🦀️.rs:29660-29673`) times
  the *whole* turn (pump-lock + `runtime_live_cleanup_pump_one` + status store) against this 8 000 µs
  ceiling, while the inner `BatchJobSession`'s own per-step grant is a separate, smaller constant,
  `RUNTIME_CLOSE_INNER_GRANT_US: u64 = 2_000` (`:29482`) — so the session is nominally budgeted 2 ms but
  the outer wall-clock check that actually flips `RUNTIME_MAINTENANCE_FAULT` allows up to 8 ms total
  including lock contention and bookkeeping.
- **Can a debug-profile, stdio-dependent plugin realistically meet it?** No hard benchmark exists in
  either ticket (the 09-02/09-05 machine was saturated — load average 4.6-112, disk 97-100% — so no
  clean measurement was taken). The evidence that exists points the other way: `26/09/01/…/📓️status.md`
  documents (§"A live-turn cost hazard the fixture fix introduced") that `processed_mesh`
  (`🪟️windows/🪚️workpiece/🦀️.rs:106-113`) "builds a fresh kernel session, replays every enabled step as a
  real CSG boolean, tessellates and remaps face groups," that a sibling call `processed_volume` "replays
  the identical sequence again" (so **one turn pays for the whole CSG replay twice**, uncached), and that
  the host "re-drives the plugin until every surface publishes, multiplying that by every continuation."
  Combined with a 48 MB **debug**-profile (`wasm-dev` profile, not `wasm-release`/ship) component plus
  jco's transpile/dispatch overhead and a dependency on `semio-s-plugin-stdio` (itself large enough to
  overrun a 1 000 000-function wasm link ceiling per that same status.md), a debug-profile plugin with
  any non-trivial real geometry/document workload is a plausible, likely candidate to exceed 8 ms on a
  first (uncached) turn — but this is reasoned inference from the two tickets' own evidence, not a
  measured number. The only lever available if it does: build the component with
  `SEMIO_BUILD_MODE=ship` (the `wasm-release` profile) — the ceiling itself cannot be relaxed for a test
  or a dev build.

---

## Summary of file:line citations used

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `:28722` (const), `:29063` (`plugin_internal_fault`), `:29482` (`RUNTIME_CLOSE_INNER_GRANT_US`), `:29654,29666,29672,29678,29684,29699,29713,29734,29736,29738,29739,29745` (12 native fault stores), `:30308` (13th, wasm-only), `:30281` (`RUNTIME_CLOSE_FAULT` sibling read), `:30322` (`RUNTIME_MAINTENANCE_FAULT` read/decode), `:5197` (`action_interactive_job`), `:12041-12046` (`validate_ui_dispatch_classification`), `:12175` (`migrated_tool_ids`), `:12185-12259` (`tool_job_registration`/`catalog-authority`/`catalog-incomplete`), `:19376-19394` (`missing-owned-reducer`/`missing-factory`), `:23048` (second `missing-owned-reducer` site).
- `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` — `:61-69` (`default_clock_us`), `:90` (`INTERACTIVE_STEP_CEILING_US`), `:93-95` (`interactive_step_contract_violated`).
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs` — `:7,568` (ceiling reference), `:597` (`INTERACTIVE_LANE_WALL_US`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — `:3037-3046` (session `refreshUi` failure → `SET_ERROR`), `:3057-3065` (spawned `refreshUi` failure → UI null), `:4917-4928` (`readConflicts` swallowed), `:6969-6992` (canvas crashed/quarantined/error render), `:1827-1828,1908-1910,2037` (supervisor state).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:130` — `resolvePluginCanvasStatus`.
- `🧰️framework/🛍️products/💻️os/🟦️.ts:2097-2109` — `decodeFaultFromWire`/`faultDisplayMessage`.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1878` — default `onTurnError` (`ActivationRegistry: turn failed for ${actorId}`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:104` — the shell's real `ActivationRegistry` instantiation (no `onTurnError` override).
