# Flow Eval Tick Addressing — The Self-Dispatched Tick Never Named Its Window

Ticket 2026/09/09/PROCEDURAL-3D-END-TO-END, lane "flowEvalTick addressing / eval continuation".
Session ⚪9f5f6952 (Fable 5.1). Repo MCP down all session (`invalid initialize params`); ticket
bookkeeping is on disk and no ticket was opened/closed/reopened.

Closes the ONE defect `📓️work-capacity-2026-09-10.md` §7 deliberately left, and answers
`📓️eval-continuation-runtime-2026-09-10.md`'s sibling question: that lane fixed the EXTENSION address
on the wire, this one fixes the WINDOW address on the command.

---

## 1. TL;DR

`flowEvalTick` is a **window-scoped** retained route — it publishes the evaluation into ONE preview
window's retained transient — but the whole chain is **self-dispatched**, and an
`Effect::DispatchAction` carries no window. `Generation3dPlayApp::pending_effects` armed
`Effect::DispatchAction { action: "flowEvalTick", args: None }`, the shell redispatched it under
whichever window was current (the flow window `procedural-main` on a served boot), and
`Generation3dFlowEvalWindowWork::extent` — which gates on the preview window — answered `None`. The
route refused every tick, on every refresh, forever: no example ever evaluated, `extrusion-axis`
stayed `computing`, `data-meshes-json` stayed `[]`.

**Fix:** the tick names its window ON THE PAYLOAD (`FlowEvalTick { window_id }`), the app declares
`retained_window_transient_target` off that field so the runtime validates it against the trusted
ViewModel roster and captures THAT window's transient authority, and `pending_effects` learns the
attached-window roster (new `view: Option<&ViewModel>` argument on `ArtifactApp::pending_effects`) so
it can arm one addressed chain per attached preview window — and arm **nothing at all** before the
first `Event::SurfaceVisible`, instead of spinning an address that cannot land.

The precedent is `TrinityJackCommand::RunQuery { results_window_id }` (`🔌️jack/…/✏️editor/🦀️.rs:523`),
the repo's only other `retained_window_transient_target` implementation, plus its job's roster check
in `▶️run-query/🧵️job/🦀️.rs:84-98`.

---

## 2. Root cause, by file and line

| # | file:line (before) | what |
|---|---|---|
| 1 | `…/🧊️generation3d/…/✏️editor/🦀️.rs` `pending_effects` | `Effect::DispatchAction { action: "flowEvalTick", args: None, … }` — the chain's ONLY entry point, with no window on it and no roster available to put one there. |
| 2 | same, `Generation3dFlowEvalWindowWork::extent` | gated on `view.window_id == window.window_id()` **and** the preview kind. `view_state` is `meta.view_state` — the SHELL's current window, never the target — so a preview-owned route could only ever be admitted while the user had the preview window focused. |
| 3 | `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`, `✅️flow-eval-resolve/🦀️.rs`, `🔺️flow-tessellate-resolve/🦀️.rs` | three more unaddressed self-redispatch sites (`RequestId(103)`, `(102)`, `(107)`), so even an addressed first tick lost its address on the second hop. |
| 4 | `🧰️framework/…/🔌️plugin/🦀️.rs` `ArtifactApp::pending_effects(doc, cfg)` | the signature itself: an app asked to arm background work was given the document and the config and **not** the attached-window roster, so no app could address window-scoped work from there even in principle. |
| 5 | `…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` `drain_flow_eval_ticks_with_view` | why the suite was green against all of the above: it HAND-BUILT `FlowEvalTick {}` and handed it a preview-narrowed `ViewModel` directly. The armed effect, its args and the shell's redispatch were never exercised. |

### 2.1 The wire hop the reproduction turned up

`makeEffectDispatchOne` (`🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx:494`) routes a
`DispatchAction` whose id is a declared **app command** through `handleCommand` (the typed command
channel) and everything else through `handleAction`. generation3d declares `flowEvalTick` with
`.command(migrated_command(…))`, so the served chain re-enters through `handle_command` with
`baseSession.viewState` attached. A test that redispatches with `handle_action` is answered
`interactive-job.unknown-key: UI dispatch rejected unknown action key 'flowEvalTick' before command
construction` — measured, not assumed (`🗑️generated/tick-1.txt`). The new drain therefore mirrors
`makeEffectDispatchOne` exactly (`testkit::dispatch_effect_command`).

---

## 3. The fix

### 3.1 Schema-first — the command payload

```rust
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick { pub window_id: String }
```

`FlowEvalResolve` and `FlowTessellateResolve` gain the same field. They do **not** need it for their
own execution (both are `HostOnly`/session routes) — they need it to re-arm the tick, and they get it
for free: `reactor::extension_response_args` (`⚛️reactor/🦀️.rs:725`) echoes the ORIGINAL invocation
request object's own fields back onto the response action, so the tick putting `windowId` into its
`evaluate`/`tessellate` request json is exactly what carries the window address across the extension
round trip. The SDK never learns what the field means.

Two shared helpers keep the address in one place:

```rust
pub fn window_args(window_id: &str) -> dsl::DslValue;                 // {"windowId": "…"}
pub fn rearm(window_id: &str, req: u32) -> Effect;                    // the addressed self-redispatch
```

**Twins:** none needed, and that is checked rather than assumed. The command has no TS/JSON payload
twin (`GENERATION3D_RETAINED_PAYLOAD_SCHEMA` is a string id, and there is no `📡️host-wire` fixture for
this app), and the manifest's per-command `args` lane is the palette-PROMPT lane, not the payload
schema — every sibling payload-carrying command (`setShowMode { value }`, `setCamera { camera }`,
`flowEvalResolve { nodeHash, outputJson }`) declares `"args": []` there too, and `flowEvalTick` is
`inPalette: false`. `Effect::DispatchAction`'s TS twin (`🎠️kernel/🟦️.ts:1219`) already carries
`args?: unknown`, so no renderer change either. The new law's own twin is the language-agnostic
fixture in §4.

### 3.2 The target declaration

```rust
fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
    match command {
        Generation3dCommand::FlowEvalTick(payload) if !payload.window_id.is_empty() =>
            Some((payload.window_id.as_str(), edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW)),
        _ => None,
    }
}
```

The runtime (`🔌️plugin/🦀️.rs:23183`) then validates the id against `meta.view_state.window_instances`,
validates the kind, and captures the window-transient authority from `view.for_window_instance(id)`
instead of from the shell's current window. `extent`/`step` no longer look at `view.window_id` at
all — they compare the captured window against the PAYLOAD, the same shape
`▶️run-query/🧵️job` uses for its `results_window_id`.

### 3.3 The roster on `pending_effects`

`ArtifactApp::pending_effects` (and `ArtifactEditor`/`ArtifactViewer`/`EditorApp`/`ViewerApp`/
`PluginApp`/`VcsArtifactApp`) gains `view: Option<&ViewModel>`. The two call sites feed it the real
roster: the `refreshUi` pass passes `Some(&request.view_state)`, and the post-mutation drain reads
the instance's last mounted view through a new `SurfaceContexts::view()`. `None` means no surface has
ever mounted.

generation3d then arms one addressed chain per attached preview window and nothing otherwise:

```rust
let windows = generation3d_preview_window_ids(view);   // roster order, preview kind only
if windows.is_empty() { return Vec::new(); }           // pre-mount: wait, never spin
… windows.iter().map(|w| flow_eval_tick::rearm(w, 104)).collect()
```

One chain **per** preview window, not one for "the" preview window: each preview window owns its own
`Generation3dPreviewWindowTransientOwner` publication, so a second preview window would otherwise
stay empty forever.

### 3.4 generation2d, made consistent

generation2d's `flowEvalTick` is a `HostOnly` route — it publishes into no window transient and
declares no `retained_window_transient_target` — so it carries **no** window id; giving it dead
payload would be worse, not more consistent. What it does share is the other half of the law: it now
takes the same roster argument and arms nothing while no window is mounted, with the difference
documented at the declaration site.

---

## 4. Tests (task 1 + task 3)

New module `…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs` over a new
language-agnostic fixture `…/✏️editor/🧫️fixtures/🪟️tick-addressing.json` (4 arming rows + 4 dispatch
rows; the addressing table is declared as DATA, not in Rust).

| law | what it drives |
|---|---|
| `every_armed_tick_names_a_preview_window_that_is_actually_attached` | `pending_effects` over each fixture roster: `[]` → arms nothing, flow-window-only → arms nothing, flow+preview → arms `preview`, two previews → arms both. Plus `view: None` → arms nothing. |
| `only_a_preview_addressed_tick_passes_the_retained_preflight` | each fixture dispatch row through the REAL channel — `handle_command` (as `makeEffectDispatchOne` does) → wire decode → `ArtifactRetainedCommandPhase::Preflight` → work — with the shell focused on the FLOW window throughout. |
| `set_active_example_drives_the_self_dispatched_tick_chain_to_a_rendered_mesh` | **the task's e2e.** `setActiveExample` from the flow window, then every following tick is the `action`+`args` of an effect the app itself armed, replayed the way `ShellHost` feeds `requestedEffects` back. Asserts all seven widgets `ok` (**`extrude: ok`**) and the preview ≥ 1 mesh. |

Testkit additions used by it: `shell_views` (flow current + preview attached),
`dispatch_effect_command` (the `makeEffectDispatchOne` shape), `drain_armed_flow_eval_ticks` (follows
armed effects only, coalescing re-arms the way a shell effect queue does, and asserts no dispatch
faults on the way).

Framework testkit: `settle_extension_invocations` now takes the caller's `ActionMeta` (the shell
answers a continuation with its own live view attached) and returns `SettledExtensionInvocations
{ answered, effects }` — a continuation that re-arms its own chain was previously invisible to the
caller, which is what let an unaddressed re-arm hide. `flow_eval_resolve`'s own unit law now asserts
the re-armed tick still carries `windowId`.

### 4.1 The reproduction, before the fix

The dispatch law is exactly the reproduction the brief asked for: with the payload field removed (the
`unaddressed` fixture row, which is still in the suite as a permanent row) the retained route answers

```
[DEBUG] tick dispatch unaddressed: payloadWindowId="" refusal=Some(Fault { origin: App,
  code: FaultCode("app.message"), message: "registered fixture typed operation fault:
  retained command work refused the command before any capacity was measured" … })
```

and the fixed row answers `refusal=None`:

```
[DEBUG] tick dispatch flow-window-addressed: payloadWindowId="main" refusal=Some(… "targeted window
  transient capture does not match the command owner's expected window kind")
[DEBUG] tick dispatch preview-addressed-from-the-flow-window: payloadWindowId="preview" refusal=None
[DEBUG] tick dispatch detached-window-addressed: payloadWindowId="closed-preview" refusal=Some(…
  "targeted window transient capture requires an attached window instance")
```

and the e2e converges on real geometry:

```
[DEBUG] tick arming no-surface-mounted: attached=[] armed=[]
[DEBUG] tick arming flow-window-only: attached=["main"] armed=[]
[DEBUG] tick arming flow-and-preview: attached=["main", "preview"] armed=["preview"]
[DEBUG] tick arming two-previews: attached=["main", "left", "right"] armed=["left", "right"]
[DEBUG] self-dispatched tick chain finished: ticks=2
  status={"width":{"status":"ok"},"height":{"status":"ok"},"distance":{"status":"ok"},
          "rect":{"status":"ok"},"vector":{"status":"ok"},"extrude":{"status":"ok"},
          "volume":{"status":"ok"}} meshes=1
```

---

## 5. Gates

`CARGO_TARGET_DIR=$S/target-tick` (cloned from this ticket's warm `target-cap` with `cp -Rc`, 16 s,
zero extra bytes — the data volume was at 100 %), `RUSTC_WRAPPER=""`. Raw logs in `🗑️generated/tick-*.txt`.

| gate | command | result | log |
|---|---|---|---|
| new addressing laws + e2e | `cargo test -p …-generation3d --features component-app-assembly --lib -- tick_addressing::` | **3 passed / 0 failed** | `tick-2.txt` |
| lane group (addressing + work-capacity + fold-contract + hex-column + refresh) | same, `-- --test-threads=1 tick_addressing:: work_capacity:: fold_contract:: refresh_pending_effects hex_column_evaluates` | **23 passed / 0 failed** | `tick-7.txt` |
| resolve/tessellate command laws | same, `-- flow_eval_resolve:: flow_tessellate_resolve::` | **8 passed / 0 failed** | `tick-4.txt` |
| framework retained-command laws | `cargo test -p semio-framework-plugin --lib retained_command::` | **6 passed / 0 failed** | `tick-fwlaws.txt` |
| generation2d suite | `cargo test -p …-generation2d --features component-app-assembly --lib -- --test-threads=2` | **228 passed / 2 failed** | `tick-g2d.txt` |
| procedural native | `cargo check -p semio-s-plugin-procedural --keep-going` | **0 errors**, `Finished dev in 33.57s` | `tick-native.txt` |
| procedural wasm | `--target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | **0 errors**, `Finished wasm-dev in 48.69s` | `tick-wasm.txt` |
| peer plugins (signature change) | `cargo check -p semio-s-plugin-{flow,fem,energy,trinity} --keep-going` | **0 errors** | `tick-peers.txt` |

generation2d's 2 failures are the declared fail-closed `module.vcs` remote-merge class
(`two_instances_converge_disjoint_widget_moves`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`) — the exact
pair `📓️work-capacity-2026-09-10.md` §6 recorded, unchanged by this lane.

One flake seen and dismissed by re-running: on the first grouped run
`refresh_pending_effects_arms_flow_eval_tick_chain` timed out with `registered fixture typed
operation did not retire within 30 seconds` while a sibling lane's `cargo test -p
semio-s-plugin-procedural` was saturating the box; it passes alone and passed in the 23/23 group
re-run.

---

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `pending_effects` gains `view: Option<&ViewModel>` across `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer`/`EditorApp`/`ViewerApp`/`PluginApp`/`VcsArtifactApp` and the viewer-forwarder macro; both call sites feed the real roster; `settle_extension_invocations` takes an `ActionMeta` and returns `SettledExtensionInvocations { answered, effects }` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs` | `SurfaceContexts::view()` — the last mounted host view, for the post-mutation `pending_effects` drain |
| `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` | `retained_window_transient_target`; `Generation3dFlowEvalWindowWork::extent`/`step` compare the captured window to the PAYLOAD; `generation3d_preview_window_ids`; `pending_effects` arms one addressed chain per attached preview window and nothing pre-mount; `preview_tessellate_invocations` carries `windowId`; wire decode of `windowId` on all three routes |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | `FlowEvalTick { window_id }`; `window_args`/`rearm`; `evaluate` takes the window and stamps it on the invocation request |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs` (+ its unit test) | `window_id` echoed back from the invocation request; addressed re-arm; law asserts the echo |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs` (+ its unit test) | same |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧫️fixtures/🪟️tick-addressing.json` | **new** — the language-agnostic addressing table (4 arming rows, 4 dispatch rows) |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs` | **new** — the three laws, including the task's e2e |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `shell_views`, `dispatch_effect_command`, `drain_armed_flow_eval_ticks`; the hand-addressed drain now takes its window from the view it was given |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `🔬️work-capacity/🦀️.rs`, `🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs` | addressed `FlowEvalTick`/`FlowEvalResolve`/`FlowTessellateResolve` construction; `pending_effects` call sites carry a roster; `node_eval_status` shared |
| `✏️s/…/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs` | `settle_with_meta` — the round trip under the shell's own `ActionMeta`, returning the effects the response actions emitted |
| `✏️s/…/🌀️generation2d/…/✏️editor/🦀️.rs` | roster argument; arms nothing while no window is mounted, with the `HostOnly`/no-window-id difference documented |
| `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs`, `🏗️fem/…/🧊️3d/…/{✏️editor,👁️viewer}/🦀️.rs`, `🏗️fem/…/◻️2d/…/✏️editor/🦀️.rs`, `🔋️energy/…/✏️editor/🦀️.rs` | `pending_effects` signature |

---

## 7. Restage (task 4)

```
CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000 \
  SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react \
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
```

`ps` showed no cargo holding `target/wasm32-wasip2` at launch (06:08:47). One clean run, 10m 8s, no
failed task; the serve on 6018 was NOT restarted.

Freshness — `🧰️framework/…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`:

| | mtime | size | sha256 |
|---|---|---|---|
| before (the work-capacity lane's stage) | `2026-09-10 05:22:09` | 80 597 509 | `34682c9d2bf6f17ba34f3723fee891904b54e6c70bf7d3876e4c00ca450f0e87` |
| after | **`2026-09-10 06:19:14`** | 80 652 384 | **`3b8fccb9fadc3c8775073cf24fffbf2ef83943119a95b80af51ef944c75df312`** |

The addressing is present in the staged wasm (`strings … | grep -c`): `windowId` ×4,
`flow-eval-tick` ×4, `generation3d-flow-eval-window-owner-mismatch` ×1.

```
✔  nx run @semio-tech/procedural-plugin:component-dev
✔  nx run @semio-tech/procedural-plugin:materialize-dev
Activated generation3d react dev: 11 completed components (changed)
```

Activation receipt `…/dist/runtime/dev/generation3d/activation/🔣️receipt.json` rewritten `06:19`,
11 plugins. Log `🗑️generated/tick-restage.txt`.

---

## 8. What the next boot's console should show

Per geometry node, and now with a window on every hop:

`pending_effects` arms `flowEvalTick {"windowId":"<the preview window instance>"}` → the shell
redispatches it through `handleCommand` (it is a declared app command) → the runtime captures that
window's transient authority → `Preflight` admits an extent of 2 rows → the tick evaluates and either
re-arms itself with the SAME `windowId` or parks an `evaluate`/`tessellate` invocation whose request
json carries it → `flowEvalResolve`/`flowTessellateResolve` come back with `windowId` echoed and
re-arm the addressed tick → the flow window's `statusJson` goes all `ok` and the preview window's
`data-meshes-json` is non-empty.

A regression now reads as one of three DISTINCT messages, none of which is a capacity fault:
`retained command work refused the command before any capacity was measured` (the payload named no
window, or named a window whose transient owner is absent),
`targeted window transient capture does not match the command owner's expected window kind` (it named
the flow window), or `targeted window transient capture requires an attached window instance` (it
named a window that has left the roster — the chain ends and the next refresh re-arms it).
