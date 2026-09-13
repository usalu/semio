# 📮️ wgpu RUNTIME MAILBOX → DISPATCH — the four defects between an enqueued input and `Ui::dispatch_event`

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu runtime mailbox / frame-job / winit-app input
dispatch", 2026-09-13. Target: `http://127.0.0.1:6118/?plugin=generation3d[&mode=generate]`, the peer's
`🎯️targets/🧊️wgpu/🌐️server`.

Repo MCP was down again this session (`repo -32602 invalid initialize params`,
`semio CONNECTION_CLOSED`); no ticket was opened, closed or reopened. The procedural plugin was NOT
restaged, the react serve on 6018 was not touched, no git-state-modifying command was run, and nothing
of the two concurrent lanes (`wgpu-chrome-parity` on `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`,
`generate-mode-panels` on the guest) was reverted.

---

## 1. TL;DR

| question | answer |
|---|---|
| **the brief's defect** | `📓️wgpu-server-input-present-2026-09-13.md` §5.2: input crosses the wire, reaches `WindowDelegate::handle_event`, is enqueued as `RuntimeApply::DispatchEvents` — and `winit_app::dispatch_normalized_event` never prints. |
| **who held the interaction state?** | **Nobody.** The brief's hypothesis (a checkout that never returns) is ruled out by measurement: at the moment the hop died, `apply_pending_step` logged `interaction state is available again` and `start_dispatch` took the `terminal_is_empty` branch without checking anything out (§3.1). |
| **what actually blocked it** | The mailbox was never *pumped* again. `apply_pending_step` is driven from ONE phase of ONE frame build (`🧵️frame-job` `ActiveFramePhase::ApplyPending`), a build is admitted only while `self.session.is_none()`, and the one live build was cancelled at its first generation change and then never retired — because its close ladder was driven one step per CALL by a caller that returned early, on a tick that only fires on input. **One superseded build wedged the whole runtime for the life of the page.** Trace: `frame build cancelled: session generation Generation(3) != requested Generation(4)` at 3.6 s, followed by 74 s with no further `frame build admitted`, no `render begin`, and 75 undelivered `DispatchEvents` (§3.2). |
| **three more, each uncovered by fixing the one before** | (b) a frame build that parks on a checked-out interaction state freezes the generation AND keeps asking for frames, so the isolate spins and starves the very suspended turn that returns the state (§3.4); (c) `BrowserRendererBootstrap::enqueueBatch` ASSIGNED the UI isolate's batch counter to `host.frame_generation`, moving it BACKWARDS on most batches, so every build was superseded against a generation it had never been admitted at (§3.5); (d) the host tick pumped no mailbox of its own, so a completion arriving while no build could run waited for one (§3.3). |
| **what that bought, measured on 6118** | `dispatch_normalized_event` prints for pointer move / down / up / wheel / keys with the shell's own coordinates; frame builds complete again (`render begin` 7 → 14, `world3d surface` 0 → 122 in one run); a viewport resize re-plans the dock (`RuntimeApply::Resize` applied, new 258/505/411 geometry, screenshot); the shell paints the full generate-mode workbench where it previously showed bare `#001117` ground. |
| **what is still NOT claimed** | `Add Generation` does not dispatch `addGeneration`, hover does not set `state.hovered`, a Preview click publishes no selection, a wheel does not orbit. All four are ONE hop past this lane's, named with console evidence in §6: the retained window body registers no `HitTarget` for its rows, so `InputState::hit_at(160.696, 138)` resolves the window's `ScrollRegion` instead of the row. |

---

## 2. The hop, and where each defect sits on it

```
DOM listener ─┬─► BrowserFrameTransport.batch ─► Worker.enqueueBatch ─► OsHost::handle_event   ✅ (previous lane)
              │                                        │
              │                                        └─(c) host.frame_generation = batch counter   ← DEFECT
              ▼
OsHost::build_and_publish_snapshot ─► events.drain_page ─► enqueue_apply(DispatchEvents, requires_interaction) ✅
              │
              ├─(d) no host-tick mailbox pump                                                    ← DEFECT
              ▼
AppPresenter::admit_next_frame ─► FrameBuildHandle::poll_runtime_and_resubmit
              │
              ├─(a) superseded session cancelled, close ladder never driven → session wedged      ← DEFECT
              ▼
ActiveFrameBuild::advance ─► ActiveFramePhase::ApplyPending ─► RuntimeMailbox::apply_pending_step
              │                         (once per BUILD, not once per frame)                     ← DEFECT (a)
              ▼
RuntimeApply::apply_step ─► start_dispatch ─► spawn_dispatch_reserved
              │
              ├─(b) the build parks on `!interaction_available()` and spins the isolate           ← DEFECT
              ▼
winit_app::dispatch_normalized_event ─► AppInteractionState::handle_pointer_move/button ✅ (this lane)
              ▼
ShellState::handle_pointer_* ─► InputState::hit_at ─► HitTarget                                  ❌ §6 (next lane)
```

---

## 3. Deliverable 1 — root cause, each step on a trace line

The transition-only `[DEBUG]` traces the brief named were in the tree but had never been built into a
wasm (`dist/wasm-dev` of 03:51 carried `start_dispatch refused: …` and `os_host dispatch_normalized_event`
but not `apply_pending_step blocked` / `start_dispatch enter` / `apply_step DispatchEvents` — verified by
extracting the `[DEBUG]` string table out of the shipped `.wasm`). The first act of this lane was to ship
them: `bun ../../🏗️compiler/🌐️wasm/📜️script.ts build dev`, once the peer's `semio-framework-pack` /
`semio-framework-deflate` extraction landed at 04:13 (§8).

### 3.1 The brief's hypothesis, ruled out by the first run

`🗑️generated/wgpu-dispatch/hit-1/console.txt`, `?mode=generate`, 80 s, one pointer nudge per second:

```
5270 [DEBUG] os_host drain events generation=InputGeneration(1) move=false scroll=false metrics=true discrete=0 enqueue-apply=true
5289 [DEBUG] apply_pending_step: runtime mutex acquired again
5289 [DEBUG] apply_pending_step: interaction state is available again
5289 [DEBUG] apply_step DispatchEvents present=true terminal-empty=Some(true)
5289 [DEBUG] start_dispatch enter
```

and then, for the remaining 75 seconds: 75 × `os_host handle_event …` + 75 × `os_host drain events …
enqueue-apply=true`, and **not one further `apply_step`, `start_dispatch`, `render begin`, `dock plan`
or `world3d` line**. `enqueue-apply=false` count: 0 — so the mailbox never even filled.

So the interaction state was **available**, the queue was **accepting**, and the pump simply stopped
running. `apply_pending_step blocked: head apply requires the interaction state and it is checked out`
never printed once. The brief's "a checkout that is never returned" is not what killed this hop.

### 3.2 What did — `ActiveFramePhase::ApplyPending` is reached once per BUILD, and one build wedged

Two traces added this lane (`frame build admitted`, `frame build superseded`) name it exactly
(`🗑️generated/wgpu-dispatch/hit-2/console.txt`):

```
3148 [DEBUG] os_host frame gate blocked=false pending=false phase=None retirement=false retained-fault=false gate-ack=false generation=Generation(3)
3148 [DEBUG] frame build admitted generation=Generation(3)
3149 [DEBUG] apply_pending_step: interaction state is available again
3150 [DEBUG] start_dispatch enter
3642 [DEBUG] frame build cancelled: session generation Generation(3) != requested Generation(4)
```

`frame build admitted` is a plain `log_debug` — it appears exactly **once** in the whole 45-second run.
The chain, all in `🎯️targets/🧊️wgpu`:

* `🪟️winit-app/🦀️.rs:145` — `redraw_core` advances `frame_generation` on every tick whose `frame_ready`
  is false, and `frame_ready` is set ONLY by the native `HostUserEvent::FrameReady` proxy
  (`🪟️winit-app/🦀️.rs:749`, `#[cfg(not(target_arch = "wasm32"))]`). In the browser worker nothing ever
  sets it, so the generation moved every tick.
* `🧵️frame-job/🦀️.rs` `poll_runtime_and_resubmit` (wasm) — `session.generation() != generation` cancelled
  the session, `begin_close()`d it and **`return None`**, so the close ladder advanced by ONE
  `close_step` per call.
* `FrameTransaction::close_step` (`🧊️renderer/🦀️.rs:11898`) returns `false` on each incremental step and
  `true` only when the last owner (`directives`) is drained — a ladder of a dozen steps.
* The browser tick is event-driven: with the shell settled it fires about once per second (75 drains in
  80 s, §3.1). A dozen ladder steps therefore took a dozen seconds — and `poll_runtime_and_resubmit`
  returned before reaching them at all in most of those ticks.
* A new build is admitted only when `self.session.is_none()`. So: **no session retirement → no new build
  → no `ApplyPending` → no `apply_pending_step` → `DispatchEvents` sits in the mailbox forever.**

### 3.3 Fix (a)+(d) — the mailbox is pumped every frame-build advance AND once per host tick

* `🧵️frame-job/🦀️.rs` `ActiveFrameBuild::pump_runtime_mailbox_step` — one `apply_pending_step` under the
  same `os_renderer.frame.apply_pending` watchdog and quarantine admission the phase used to own, called
  at the TOP of every `advance()`. `ActiveFramePhase::ApplyPending` is now only the "drain before you
  build" hand-off; the pump is phase-independent.
* `🪟️winit-app/🦀️.rs` `build_and_publish_snapshot` — `self.runtime.pump_pending_applies(RUNTIME_APPLY_TICK_CREDITS)`
  (8) before the presentation gate, so a completion arriving while NO build can run (the state is
  checked out, or a presentation is still pending) is still applied within the next frame.
* `🧵️frame-job/🦀️.rs` `poll_runtime_and_resubmit` — a superseded session is no longer parked: the
  mismatch branch cancels and `begin_close()`s, then **falls through to the drive loop**, whose `Closing`
  arm now loops `close_step` until `terminal_is_empty()` (or the caller's deadline) instead of breaking
  after one. The next opportunity admits a fresh build.
* `🌐️browser-worker/🦀️.rs` — `request_frame` gained `host.runtime.has_pending_applies()` and
  `host.frame_build.has_live_session()`, the same shape as the existing `has_pending_world3d_work()`
  term: a live build and an unapplied completion each owe the shell another frame.
* `🪟️winit-app/🦀️.rs` `redraw_core` — the generation no longer advances underneath a live build
  (`else if !self.frame_build.has_live_session()`). Input still advances it through
  `enqueue_host_event`/`enqueue_host_metrics`, which is the only thing that genuinely makes a build stale.

### 3.4 Defect (b), uncovered by that fix — a parked build starves the turn that returns the state

`🗑️generated/wgpu-dispatch/hit-3/console.txt`: `dispatch_normalized_event` printed for the first time
(3 times), and then the isolate answered nothing at all for 30 seconds:

```
11565 [DEBUG] apply_step DispatchEvents present=true terminal-empty=Some(false)
11565 [DEBUG] start_dispatch enter
11565 [DEBUG] apply_pending_step blocked: head needs the interaction state, checked out at Some("dispatch-event") for 1 opportunities
11566 [DEBUG] os_host dispatch_normalized_event PointerMove { … x: 4.0, y: 4.0 }
        ← 30 s of silence: no handle_event, no drain, no frame
```

`AppFrameTransaction::step` returned `Pending` while `!app.interaction_available()`. With the generation
now frozen under a live build and `request_frame` true, the worker isolate spent its whole interactive
share re-entering a transaction that could not move — and `spawn_local`'s microtask, the one that ends
`dispatch_normalized_event` and `finish()`es `ResumeDispatch`, never got a turn.

**Fix:** `!interaction_available()` is no longer a park. It is `AppFrameTransactionStep::Superseded`
(phase → `Terminal`), so the build ends in that same turn, the session retires, the JS stack unwinds,
the suspended turn completes, and the next opportunity admits a build against the returned state.

### 3.5 Defect (c) — the frame generation was moving BACKWARDS

`🗑️generated/wgpu-dispatch/hit-4/console.txt`, one cycle per nudge, 60 of them:

```
20958 [DEBUG] os_host handle_event PointerMove { … x: 3.0, y: 3.0 } gen=20
20958 [DEBUG] os_host drain events generation=InputGeneration(19) … enqueue-apply=true
20958 [DEBUG] apply_step DispatchEvents present=true terminal-empty=Some(false)
20959 [DEBUG] os_host dispatch_normalized_event PointerMove { … x: 3.0, y: 3.0 }
20988 [DEBUG] frame build admitted generation=Generation(21)
21008 [DEBUG] frame build superseded: session generation Generation(21) != requested Generation(20)
```

A session admitted at `Generation(21)` answering a request for `Generation(20)`: the host's counter had
gone **down**. `🌐️browser-worker/🦀️.rs` `enqueueBatch` ended with `host.frame_generation = generation;`
— an ASSIGNMENT of the UI isolate's per-batch counter onto the host's own frame generation. They are
different sequences (the host's advances once per delivered event plus once per idle redraw), and the
batch counter is routinely the lower of the two. Every build was therefore superseded against a
generation it had never been admitted at: `frame build admitted` 1 740 times in 60 s, `render begin`
frozen at its 7 boot surfaces, `dock plan` 0.

**Fix:** the assignment is deleted. Every event in the batch has already advanced the host's generation
through `enqueue_host_event`; the `scheduler.invalidate(INPUT_STATE)` that followed it is kept. The host
generation is now monotonic, which is what a frame build, a presentation witness and a raster witness all
pin themselves to.

### 3.6 The interaction checkout, made observable and bounded anyway

The brief's structural requirement stands on its own merits even though a stale checkout was not the
defect, and it is now enforced rather than assumed:

* `AppRuntime::check_out_interaction(site)` / `AppRuntime::return_interaction(state)` are the ONLY two
  ways the state leaves and re-enters the runtime. Every former `runtime.interaction.take()` /
  `runtime.interaction = Some(..)` goes through them — `start_dispatch`, `start_frame_deferred`, the
  native frame-maintenance refusal hand-back, `submit_interaction`, and all three `ResumeDispatch` /
  `ResumeFrameDeferred` / `RestoreInteraction` arms, including their credit-exhaustion and
  cursor-missing failure paths. (`RestoreInteraction(None)` no longer clobbers a live state to `None`.)
* `runtime_mailbox_core::InteractionCheckoutLedger` ages a live checkout by one per apply opportunity
  whose head it actually blocked. Past `INTERACTION_CHECKOUT_CREDITS` (240) the verdict becomes `Stale`
  and `take_stale_notice()` publishes, exactly once per episode, a typed frame fault naming the site:
  `runtime interaction checkout at {site} outlived its bounded step after {n} apply opportunities`. It
  reaches the page through the existing `take_frame_fault` → fault-banner path, so a leak is a visible
  diagnostic instead of a silent freeze.
* `BoundedCompletionQueue::first_applicable(interaction_available)` replaces the head-only gate: work
  that needs no interaction state is admitted PAST work that does, so the completion carrying the state
  home can never be queued behind the input waiting for it. Order among the state-needing completions is
  untouched. A refused apply is restored at its own index, never dropped and never reordered.

Measured on the live target after all of the above (`🗑️generated/wgpu-dispatch/interaction-1/verdict.json`):
`apply_pending_step blocked` 0, `outlived its bounded step` 0, `surface fault` 0.

---

## 4. Deliverable 2 — the fixture and the two implementations that answer it

### 4.1 The oracle

`🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json` — 8 rows, each a sequence of mailbox operations
(`enqueue` / `reserveInteraction` / `finish` / `checkOut` / `checkIn` / `apply`, the last with a `repeat`)
and, per step, the answer the implementation must give: the admission verdict
(`admitted` / `deferred` / `stale`), the revision the turn applied (or `null`), and the ledger's age of
the live checkout. Each row also declares its final `length`, `readyRevisions` **in order**, and the
`staleNotices` published. `credits` pins `INTERACTION_CHECKOUT_CREDITS`.

The rows are the laws this lane needed: a checked-out state defers the head **without losing it**; work
needing no state is admitted past work that does; a returned state arrives at the front and unblocks the
input behind it; `Resize` and `DispatchEvents` are both applied once the state is home; a second checkout
is refused without disturbing the first one's age; a checkout past its credits is a typed diagnostic
**once** and check-in clears the episode; the fixed bound always keeps one interaction reserve; a keyed
completion coalesces its own predecessor at the bound.

### 4.2 Rust — the law over the live mailbox

`🧪️tests/🎮️wgpu-runtime-mailbox-admission/🦀️.rs`, mounted from `📮️runtime-mailbox-core/🦀️.rs` so it runs
against the very `BoundedCompletionQueue` and `InteractionCheckoutLedger` that
`RuntimeMailbox::apply_pending_step` calls on the browser — not a copy. Row capacities are resolved to
the two mounted const-generic queues; site and key names are resolved to the same `&'static str`s the
renderer mints, so a fixture that names something the renderer never produces fails loudly.

### 4.3 TypeScript — the twin

`🧪️tests/🎮️wgpu-runtime-mailbox-admission/🟦️.ts`, registered in `🧪️tests/🎚️config/🟦️.ts`. Re-derives the
queue and the ledger from the oracle alone — the fixed bound shared by ready and in-flight work, the
one-slot interaction reserve, keyed coalescing, `firstApplicable`, and the ageing/stale/notice rules —
and replays every row, plus five independent cases (admit-past ordering, order preservation among
state-needing work, ageing only on opportunities the head actually blocked, one notice per episode,
check-in clearing it).

### 4.4 Both, run

| command | result |
|---|---|
| `cargo test … --lib runtime_mailbox` (`RUST_MIN_STACK=33554432`) | **5 passed, 0 failed** — `admission_laws::every_fixture_row_replays_on_the_live_mailbox`, `admission_laws::the_fixture_pins_the_credits_the_ledger_actually_spends`, plus the two pre-existing core unit laws and the async-boundary mailbox law |
| `bunx vitest run --config "🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🎮️wgpu-runtime-mailbox-admission/🟦️.ts"` | **13 passed, 0 failed** |

---

## 5. Deliverable 3 — what 6118 says now

Reproduction (unchanged from the previous lane's §5.3; the serve needs no restart for new renderer wasm,
only a page reload):

```
screen -dmS g3dwgpu "<ticket>/📜️serve-generation3d-wgpu.sh"
cd <ticket> && SEMIO_PROBE_SETTLE=40 SEMIO_PROBE_OUT=wgpu-dispatch/hit-N bun 🐍️wgpu-hit-probe.mjs
cd <ticket>/🗑️generated/wgpu-dispatch && SEMIO_PROBE_OUT=interaction-N bun interaction-probe.mjs
cd <ticket>/🗑️generated/wgpu-dispatch && SEMIO_PROBE_OUT=hover-N       bun hover-probe.mjs
```

### 5.1 Claimed and proven

* **`winit_app::dispatch_normalized_event` prints.** `🗑️generated/wgpu-dispatch/hit-5/console.txt`, 65
  occurrences in 80 s, including the full click triple at the derived row point:

  ```
  41551 [DEBUG] os_host handle_event PointerMove { … x: 160.696, y: 138.0 } gen=155
  41551 [DEBUG] os_host handle_event PointerDown { … x: 160.696, y: 138.0, button: Primary } gen=156
  41551 [DEBUG] os_host handle_event PointerUp   { … x: 160.696, y: 138.0, button: Primary } gen=157
  41551 [DEBUG] os_host drain events generation=InputGeneration(42) move=true scroll=false metrics=false discrete=2 enqueue-apply=true
  41552 [DEBUG] apply_step DispatchEvents present=true terminal-empty=Some(false)
  41552 [DEBUG] start_dispatch enter
  41552 [DEBUG] os_host dispatch_normalized_event PointerMove { … x: 160.696, y: 138.0 }
  41568 [DEBUG] os_host dispatch_normalized_event PointerDown { … x: 160.696, y: 138.0, button: Primary }
  41582 [DEBUG] os_host dispatch_normalized_event PointerUp   { … x: 160.696, y: 138.0, button: Primary }
  ```

  Before this lane that line appeared zero times, across every probe the previous two lanes ran.

* **Frame builds complete again.** Same run: `frame build admitted` 190, `frame build superseded` 60
  (transitions), `render begin` 7 → 14, `wgpu-shell dock plan` 1, `world3d surface` 122, `surface fault`
  0, `apply_pending_step blocked` 0.

* **A resize re-plans the dock — `RuntimeApply::Resize` is applied.**
  `🗑️generated/wgpu-dispatch/interaction-1/verdict.json`: `dockPlanDeltaAfterResize = 1` and

  ```
  before 1440×900: generations@315x814+3,54  generate-form@616x814+319,54  generate-preview@502x814+935,54
  after  1180×760: generations@258x674+3,54  generate-form@505x674+261,54  generate-preview@411x674+766,54
  ```

  Screenshot `🗑️generated/wgpu-dispatch/interaction-1/shot-after-resize.png` — the navbar with the
  document title and **Generate**/**Editor**, the Generations window (`Generations`, `<no generations>`,
  `Actions`, `Add Generation`), the Form window's "Add a generation to edit input values.", and the
  Preview window's ground plane, all at the new geometry. (The previous lane's §5.2 recorded exactly the
  opposite: "a resize no longer produces a second `dock plan` line".)

* **The shell presents in response to input.** `🗑️generated/wgpu-dispatch/hover-1/1-idle.png` is the bare
  `#001117` ground after a 35 s settle; `2-hover-row.png`, taken after one pointer move onto the row, is
  the painted workbench. Sampled programmatically: the row band's mean channel value is 13.33 (identical
  to the reference band) while idle and 44.38 after the pointer lands.

### 5.2 NOT claimed

Clicking `Add Generation` does **not** dispatch `addGeneration`; hovering does **not** set
`state.hovered`; a click on a Preview instance publishes **no** selection; a wheel over the Preview does
**not** orbit. `dumpFrameStats("generation3d-generate-preview")` still reports
`scenePasses 1, sceneDraws 1, sceneInstances 0`, and the guest's own status is
`"phase":"faulted", "code":"flow.extension-not-contributed", "extensionId":"brep"` — this lane did not
restage the procedural plugin, so no generation could produce geometry even if the action fired.

---

## 6. The ONE remaining hop, named with console evidence

All four of those landings share a single cause, one step past this lane's:

```
37013 [DEBUG] os_host pointer hit x=6 y=6            targets=31 hit=None
37143 [DEBUG] os_host pointer hit x=160.696 y=138    targets=31 hit=Some((ScrollRegion, Some("generation3d-generations")))
39621 [DEBUG] os_host pointer hit x=161.696 y=138    targets=31 hit=Some((ScrollRegion, Some("generation3d-generations")))
47339 [DEBUG] os_host pointer hit x=160.696 y=138    targets=31 hit=Some((ScrollRegion, Some("generation3d-generations")))
```

(`🗑️generated/wgpu-dispatch/hover-2/console.txt`; the trace is `InputState::hit_targets.len()` plus
`InputState::hit_at(x, y)`, taken immediately after `ShellState::handle_pointer_move` returns.)

**The registry holds 31 hit targets — the shell chrome — and not one row of any retained window body.**
The point `(160.696, 138)` is derived, never guessed, from the shell's own dock plan (`generations`
body at `(3, 54)`) plus `dumpStructure`'s published `mounted_layout` rect for
`…/stack[0]#procedural3d-play-generate.add-generation` = `[0, 72, 315.392, 24]`; it lands inside that
rectangle, and `hit_at` answers the WINDOW's `HitKind::ScrollRegion` instead of a `HitKind::TreeItem`.

That single fact explains every remaining landing:

* `ShellState::handle_pointer_button` finds no row target, so nothing is pushed into
  `InputState<ActionDescriptor>` and the frame transaction's `AppFrameTransactionPhase::InputEvents`
  drains nothing — no `addGeneration`, no `wgpu-shell deferred action`, no `wgpu-shell command`
  (all three counted at 0 across every run).
* `NodeFlags::HOVERED` is set by the `events::EventRouter` hover chain from the same registry, so
  `dumpStructure`'s `state.hovered` stays false on every node of the window — measured with the pointer
  parked on the row for five seconds, then moved 300 px away, then returned
  (`hover-1/verdict.json`: `hoveredIdle`, `hoveredOnRow`, `hoveredAway`, `hoveredBack` all `[]`).
* `AppFrameTransactionPhase::WheelStart` gates on
  `ShellState::wheel_propagates_to_scene_surface(interaction.input.hit_at(x, y))`, so a `ScrollRegion`
  answer ends the wheel before it reaches the world: the world3d camera is byte-identical across all
  121 `world3d surface` traces of the run that wheeled twice —
  `{"fov":45.0,"position":[4.0,-4.0,3.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0]}`.

The owner is the retained document paint/hit registration in `🖱️ui/🎯️targets/🧊️wgpu` together with
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, which the `wgpu-chrome-parity` lane holds. Nothing of it was touched
here. The `[DEBUG] os_host pointer hit …` trace is left in place as that lane's first probe.

---

## 7. Builds and tests, run

| command | result |
|---|---|
| `bun ../../🏗️compiler/🌐️wasm/📜️script.ts build dev` (the exact command `@semio-tech/framework-renderer-wgpu:wasm` runs) | **published 5×** — traces, then each fix in turn; `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`, shared cargo build dir |
| `cargo check --target=wasm32-unknown-unknown --manifest-path …/🦀️rust/Cargo.toml --offline` | `Finished dev` with 31 warnings on `semio-framework-os-renderer-wgpu` (warnings quoted as proof the expansion actually ran); the one warning this lane introduced (`variable does not need to be mutable`) was fixed |
| `cargo test … --lib runtime_mailbox` | **5 passed, 0 failed** |
| `cargo test … --lib frame_job` | **5 passed, 0 failed** — two of them were failing before this lane (`expired_deadline_is_reported`, `not_yet_expired_deadlines_are_kept`, both `panicked … completed directives`) because `🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs` never called `BatchJobSession::checkout_outcome()` before `checked_out_job_mut()`; the job module's two-call checkout protocol (`🧵️job/🦀️.rs:2157`, last touched 2026-09-12 18:50 by another lane) has required it for a day. Fixed to the order `ActiveFrameBuild::advance` itself uses. |
| `cargo test … --lib winit_app` / `surface_lane` / `render_snapshot` / `async_boundary_tests::runtime` | 4 / 4 / 7 / 6 passed, 0 failed |
| `bunx vitest run --config "🧪️tests/🎚️config/🟦️.ts"` (all 15 files) | **164 passed, 6 failed** — the six are `🧪️tests/🧩️package-integration/🟦️.ts`'s `ReferenceError: Bun is not defined`, which is that suite's own known constraint (it must run under `test-preview-generated`, row below) |
| `SEMIO_TEST_LEVEL=long bun ./📜️script.ts test-preview-generated` | **20 passed, 0 failed** |
| `bun ./📜️script.ts test-browser-worker` | **69 passed, 0 failed** (5 files) |
| `bun ./📜️script.ts check-browser-worker` | clean — the generated `🚀️boot.js` / `🎞️frame-worker.js` bytes still match their sources |
| full `cargo test … --lib` (529 tests) | **not claimed green, and not made worse.** The crate is broadly red before this lane. Two tests abort the whole binary with a non-unwinding `panic in a destructor during cleanup` (`async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`, `kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_…`), so the harness never prints a `failures:` summary at all; skipping both still aborts later, and the run to that point reports **38 FAILED** across `interpreter::render_plan_validator_tests` (SVG/PNG decoding), `engine_canvas::node_graph_attach_tests`, `kernel_runtime::semantic_document_tests`, `scenes::raster_frame_cost_tests`, `dock::tests::dock_stack_content_fills_full_bounds_through_one_silhouette_clip`, `deadlines::tests::caret_blink_toggles_and_rearms_on_fire`, `async_boundary_tests::native_binary_owns_exactly_one_entrypoint_driver` and more. Raw list: `🗑️generated/wgpu-dispatch/libtest-failures.txt`. |
| ↳ the two that touch this lane's file, checked individually | **both pre-existing, neither reachable from this lane's diff.** `runtime_publication_tests::runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation` fails because a reader observes `(ready=1, scene_revision=1)` — the half state `enqueue_runtime_completion` exposes between `drop(queue)` and `mark_scene_changed()`; that function is byte-identical in this lane's diff (`git diff … | grep enqueue_runtime_completion\|mark_scene_changed` is empty). `async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` is a SOURCE-SHAPE contract that the PREVIOUS lane's `present_step` fix invalidated: `presenter_retirement_contract` still demands `let expected = self.presentation_authority.current();` **twice** (the tree has 1, since `📓️wgpu-server-input-present-2026-09-13.md` §3.3 deleted the `Acknowledge` re-read), `self.presentation_authority.mark_scene_changed();` **twice** (the tree has 1, at `🧊️renderer/🦀️.rs:9845`; the sibling call at `:9786` is spelled `presentation.mark_scene_changed()` and does not match the literal), and `runtime.presentation_witness_for(self.generation.0)` **once** (the tree has 2, at `:11320` and `:11389`). Three literal counts in `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:806-861`, all about the presentation-freshness rule that lane rewrote — deliberately left for its owner rather than guessed at here. |

### Peer breakages run through, attributed and waited on rather than worked around

1. **`semio-framework-pack`** was red at session start with `use of unresolved module or unlinked crate
   semio_framework_deflate` at `🎒️pack/📐️format/🦀️.rs:1803` — a peer mid-extraction of a new
   `semio-framework-deflate` crate whose dependency `🎒️pack`'s manifest did not yet declare. Waited; the
   peer removed the references at 04:13 and the wasm build went through.
2. **`🌐️browser-worker/🦀️.rs`** was missing this lane's new `AppRuntime::checkout` field for about a
   minute; a concurrent lane added `checkout: crate::runtime_mailbox_core::InteractionCheckoutLedger::default()`
   at 04:22 before this lane could. Kept as landed, not reverted.
3. **`semio-framework-plugin` → `semio-s-artifact-puzzle-3d`** went red at 05:15 and stayed red: another
   lane is mid-refactor of the retained window-config typed state
   (`🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`, rewritten again at 05:19). Its errors walked from
   `O::State::record_spec`/`envelope_id` (E0599) to
   `the trait bound window::component::Puzzle3dWindowConfig: DslField is not satisfied` (E0277) to a
   bare `mismatched closing delimiter` inside four minutes — an actively moving edit, not a completable
   breakage. Fourteen retries over ten minutes never found a green window. **Every result in this
   section was measured before it**, against a tree that compiled; nothing of that lane was touched, and
   the shipped `dist/wasm-dev` (04:56) is newer than every source this lane changed
   (`📮️runtime-mailbox-core` 04:19, `🧵️frame-job` 04:32, `🪟️winit-app` 04:41, `🌐️browser-worker` 04:45,
   `🧊️renderer` 04:55), so the running target on 6118 is this lane's final code.

---

## 8. Files

**Changed**

| file | what |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📮️runtime-mailbox-core/🦀️.rs` | `first_applicable`/`take_at`/`restore_at`/`head_requires_interaction`; `InteractionCheckoutLedger`, `InteractionCheckoutStep`, `INTERACTION_CHECKOUT_CREDITS`; the admission contract in the module docstring |
| `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `AppRuntime::check_out_interaction`/`return_interaction` + the `checkout` ledger field, and every take/restore site routed through them; `apply_pending_step` rewritten (first-applicable admission, ledger ageing, typed stale diagnostic, index-preserving restore); `pump_pending_applies`; `has_pending_applies`; `!interaction_available()` in the frame transaction is now `Superseded`, not a park; `AppPresenter::presentation_gate_shape` |
| `…/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` | `pump_runtime_mailbox_step` called from every `advance()`; a superseded session falls through to the drive loop instead of parking, and the `Closing` arm retires it within the turn; `has_live_session`; `[DEBUG]` traces for admission and supersession |
| `…/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` | the host-tick mailbox pump and `RUNTIME_APPLY_TICK_CREDITS`; the frame generation no longer advances under a live build; `[DEBUG]` traces for the presentation gate, the present-step fault and the pointer hit registry |
| `…/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | the UI-isolate batch counter no longer overwrites `host.frame_generation`; `request_frame` gained the live-build and pending-apply terms |
| `…/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs` | `checkout_outcome()` before `checked_out_job_mut()` — the order the production driver uses |
| `…/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` | registers the new twin |

**Added**

| file | what |
|---|---|
| `…/🧑‍🎨engine/🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json` | the language-neutral oracle, 8 rows |
| `…/🧑‍🎨engine/🧪️tests/🎮️wgpu-runtime-mailbox-admission/🦀️.rs` | the Rust law over the live mailbox |
| `…/🧑‍🎨engine/🧪️tests/🎮️wgpu-runtime-mailbox-admission/🟦️.ts` | the TypeScript twin |

**Generated (disposable, under `<ticket>/🗑️generated/wgpu-dispatch/`)**

`hit-1/` … `hit-5/`, `interaction-1/`, `hover-1/`, `hover-2/`, and the two throwaway drivers
`interaction-probe.mjs` and `hover-probe.mjs`.

**Traces left in the tree.** Every line added for this investigation carries the `[DEBUG] ` prefix and is
removable in one sweep: `os_host frame gate`, `os_host present_step faulted`, `os_host pointer hit`,
`frame build admitted`, `frame build superseded`, `start_dispatch: interaction state checked out for one
event`, and the reworded `apply_pending_step blocked: head needs the interaction state, checked out at …`.
`os_host pointer hit` is the one §6's owner should keep until that hop closes.
