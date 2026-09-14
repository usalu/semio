# 🫀️ A runtime-owned settle pump for the wgpu host — lane `wgpu-host-settle-pump` (2026-09-14)

Three lanes handed this one the same blocking dependency in three different words:

* `📓️wgpu-progress-visibility-2026-09-14.md` §8.1 — *the boot example converges before anything paints*,
  owning layer named (`settle_boot` → `flush_deferred_actions` to a fixed point), **not fixed**;
* the same report §8.2 — *the 6118 preview wedges mid-tessellation and then goes deaf to
  `setActiveExample`*, **not fixed**;
* `📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4 — `UiDirtyScope` narrowing is *implemented, measured,
  and switched off*, because a window body's render is also the guest crossing that funds the guest's
  own background solve. Its §4.4 names the fix in one sentence: **"the guest's evaluation needs a pump
  of its own — the frame loop's, not the refresh's."**

This lane builds that pump, switches the narrowing on, and proves both on 6118.

Evidence: `🗑️generated/wgpu-settle/`, `🗑️generated/wgpu-verify/scoreboard.json`.
Probe (new): `🐍️wgpu-settle-pump-probe.mjs`.

<!-- RUNTIME -->

---

## 1. Root cause — one authority, three symptoms

The wgpu shell had **no settle lane at all.** Every guest chain was converged inside the call that
started it, and the three callers were the boot, the input path, and nothing else:

| caller | what it did | symptom |
|---|---|---|
| `settle_boot` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) | `refresh_ui(Full)` then `flush_deferred_actions()` — `settle_ui_chain` to a fixed point, up to `SHELL_SETTLE_ROUNDS` whole-shell refreshes | an `?example=` boot ran its whole evaluation inside `boot_shell`: `boot_shell leave 7 226 ms`, chrome at 9 212 ms, against 1 880 ms / 4 101 ms for the same build with no example (§8.1) |
| `handle_pointer_button` / `handle_pointer_move` / the retained press | the same synchronous `flush_deferred_actions` | post-boot, a chain converged **only while the user moved the mouse**; a booted example never finished for a user who did not |
| — | nothing drove a chain the guest re-armed on its own | a run left non-terminal blocked every restart, so the preview froze at `meshingFaces 36/56 ratio=0.64912283` and ignored `setActiveExample` for 200 s (§8.2) |

And because the ONLY thing that crossed into the guest was `refresh_ui`, the refresh could not narrow:
withdrawing a render withdrew compute from a brep solve that had nothing to do with the UI — 14 of 16
examples frozen mid-solve, `box-fillet-preview` never converging in 180 s at 36 renders against
21.13 s at 96 renders with the scope forced to `Full` (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4.3).

**One authority was missing: a lane that a producer DECLARES into and the frame loop DRIVES.** React
has had exactly that since `createUiRefreshCoalescerV1` (`🛠️ShellHelpers/🟦️.tsx`): at most one pass in
flight, at most one owed follow-up carrying the union, and — property 3 — *a pass may ASK for another
pass; it may never WAIT for one*. Its drain loop runs on the browser's own event loop, so React never
converges a chain inside the call that started it and the page paints between passes. The wgpu shell
had the DECLARING half of that (this ticket's `owed_refresh_scope`, added by the dirty-scope lane) and
none of the DRIVING half.

---

## 2. The design

### 2.1 The two halves, and who owns each

```
producer  ──owe_settle()──▶  ShellSettlePump (declared)
                                   │
frame-finish boundary ──settle_pump_pending()──▶ FrameFinishCursor.settle
                                   │
browser tick ──has_pending_settle()──▶ request_frame          ← keeps an event-driven shell ticking
                                   │
FrameDeferredCursor ──FrameDeferredWork::Settle──▶ settle_pump_step()   ← exactly ONE step per frame
```

* **`ShellSettlePump`** (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) — the shell's half. `owe_settle` is the only
  way to arm it, and it is the settle twin of `owe_refresh`: producers DECLARE, they never converge.
* **`FrameDeferredWork::Settle`** (`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`) — the runtime's half. The
  frame-finish boundary re-reads `settle_pump_pending()` every frame (never a latch carried across
  one) and the frame's deferred owner hands the shell exactly one step, **after** that frame's own
  input actions, its chrome maintenance, its sync pump and its tutorial flush.
* **`RuntimeMailbox::has_pending_settle`** + the browser tick's `request_frame` — the term that makes
  the lane real on an event-driven shell. Without it the shell settles the instant the last apply
  drains and the pump would run only while something else happened to ask for a frame.

### 2.2 What one step does

`settle_pump_step` is one bounded step and nothing more:

1. **drain** one round of armed work (deferred actions, parked extension answers, parked file opens,
   an owed shell URI);
2. if anything ran, or a refresh scope is owed, **take that one refresh pass** and stop. The next
   frame takes the next step — which is what lets the chrome, the GPU present and the status pill stay
   live through a convergence that used to run to a fixed point inside `boot_shell`;
3. if nothing was armed, **cross**: for every live World3d surface whose own published status says
   `computing`, ask `refresh_ui` for exactly that producer's window body. That is the crossing the
   guest's background solve is funded by, made once per FRAME rather than once per settle round.

### 2.3 The wedge watchdog

Per producer the pump keeps a **witness** — `phase|unitsDone|unitsTotal|facesDone|facesTotal|inFlight`,
read through the same `world3d_compute_status` parser the status pill reads — and one verdict per step:

| the producer's witness | verdict |
|---|---|
| moved | `Fund` — spend this step's crossing on its window body, and reset the whole watch |
| frozen, fewer than `SHELL_SETTLE_STALL_STEPS` (240) steps | `Fund` |
| frozen past the budget, `cancellable`, drives left | `Terminal` — dispatch the producer's OWN published `cancelAction` with its own `cancelArgs` |
| frozen past the budget, no `cancelAction` or `SHELL_SETTLE_TERMINAL_DRIVES` (3) drives spent | `Stand` — stop paying for crossings until the witness moves again |

Two deliberate refusals:

* **the shell learns no domain verb from code.** The terminal drive is whatever id the surface's own
  status contract published, exactly as the cancel control does. A producer that offers no way to end
  its own run is never driven.
* **`Stand` exists because a producer may lie.** The served generation3d guest publishes
  `computing: true` on **560 consecutive samples** of a window in which nothing evaluated at all
  (`📓️wgpu-progress-visibility-2026-09-14.md` §3.1). A pump that believed that flag would pin the host
  at one guest crossing per frame forever. A stood-down producer is not `computing` as far as
  `settle_pump_pending` is concerned, so the shell goes quiet.

### 2.4 `UiDirtyScope` narrowing, switched on

With the crossing owned by the pump, `refresh_ui` is free to render only what a settle dirtied. It now
gates on the kernel predicates the dirty-scope lane made shared — `wants_window_body`,
`wants_panel_body`, `wants_section(Measures)`, `wants_section(Engagements)` — whose TypeScript twins
React already runs against the same fixture. A surface's retirement stays INSIDE the per-surface loop,
so a skipped surface keeps the exact document it owns instead of being retired and never re-minted.

One widening had to land with it. `apply_ops_inner` already widened to `Full` on a `setPanel`
operation; it now widens on a **`setDocument`** too. The generation3d editor's own `setActiveExample`
is the measured case — it replaces the whole fixture through artifact mutations and declares
`UiDirtyScope::None` (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §3.4) — so a shell that honours the
scope without this would leave every window painting the previous document. A `flowEvalTick` emits no
`setDocument`, so the seven hops of a converging edit still narrow to nothing, which is the whole
116-of-137 `patched=0` saving the dirty-scope lane measured.

---

## 3. Files changed

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
  — `ShellSettleStep`, `ShellSettleVerdict`, `ShellSettleWatch`, `ShellSettlePump`,
  `settle_pump_owes`, `settle_watch_verdict`, `settle_progress_signature`,
  `ShellState::{owe_settle, settle_pump_pending, settle_pump_step, settle_pump_cross,
  live_compute_surfaces, window_body_key}`; `settle_boot` arms instead of converging; `refresh_ui`
  honours the scope.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
  — `FrameDeferredWork::Settle`, the `settle` flag through `FrameFinishCursor` /
  `FrameDeferredCursor::{new, take_next, terminal_is_empty, close_step}`, and
  `RuntimeMailbox::has_pending_settle`.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs`
  — the browser tick's `request_frame` owes a frame while a settle is owed.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧾️frame-action-ledger/🦀️.rs`,
  `…/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` — the new `FrameDeferredCursor::new` arity.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`
  — the new vitest suite.

Created:

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫀️settle-pump/🔣️.json` — the
  language-agnostic oracle.
* `…/🧪️tests/🫀️settle-pump/🦀️.rs` and `…/🧪️tests/🫀️settle-pump/🟦️.ts` — the two twins.
* `🐍️wgpu-settle-pump-probe.mjs`, this report.

A peer lane (`wgpu-generate-add-port-fit`) extended the same oracle mid-session with its `gestureRows`
and landed the input half — `handle_pointer_button`, `route_retained_pointer_press`,
`dispatch_tree_selection` and the renderer's `handle_pointer_move` now `owe_settle()` and return
instead of calling `flush_deferred_actions`. Their rows are answered by the Rust twin's
`an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch`. Nothing of
theirs was reverted.

---

## 4. Laws

Shared, language-neutral oracle:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫀️settle-pump/🔣️.json` — **7 `owesRows`,
5 `watchRows`, 4 `frameRows`, 4 `gestureRows`, 11 declared laws**. Two independent implementations
answer it.

| # | law | where | what it drives |
|---|---|---|---|
| 1 | `the_frame_owes_a_settle_step_exactly_while_a_chain_is_live` | `🧪️tests/🫀️settle-pump/🦀️.rs` | the shell's own `settle_pump_owes`, over all five terms + the two refusals |
| 2 | `a_producer_is_funded_while_it_advances_and_driven_to_a_terminal_state_when_its_witness_freezes` | same | the shell's own `settle_watch_verdict`, over statuses parsed by the shipped `world3d_compute_status` — the same parser the pill reads |
| 3 | `a_frame_drains_its_input_actions_before_it_takes_a_settle_step` | same | the REAL `FrameDeferredCursor::{new, take_next, terminal_is_empty}` |
| 4 | `the_boot_arms_the_settle_lane_instead_of_converging_and_the_frame_loop_drives_it` | same | `settle_boot`'s body, the frame-finish boundary and the deferred owner, read as source |
| 5 | `the_refresh_honours_the_dirty_scope_now_that_the_pump_funds_the_guest` | same | `refresh_ui`'s four narrowing gates |
| 6 | `an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch` | same | the peer lane's `gestureRows` — four entry points that must not reach `flush_deferred_actions` |
| 7 | `every_declared_law_is_answered_here` | same | the oracle declares ≥ 10 laws and one consumer per language |
| 8 | the TypeScript twin, independent re-derivation | `🧪️tests/🫀️settle-pump/🟦️.ts` | 7 tests; the witness is built through the SHIPPED `world3dComputeStatusV1` React itself reads |

Non-vacuity is pinned in both twins: the oracle carries a settled shell that must owe **nothing**
(`a-settled-shell-owes-no-settle-step-at-all`) and a frame that carries **no** settle work, so a pump
that always said yes — one guest crossing per frame forever — fails the suite.

Counts actually run, foreground:

```
cargo test -p semio-framework-os-renderer-wgpu --lib settle_pump     7 passed
bunx vitest … 🧪️tests/🫀️settle-pump/🟦️.ts                             7 passed
```

**The failing-first check was run, not assumed.** With the `computing` term removed from the
TypeScript `settlePumpOwes`, the suite fails exactly where it should —
`a-live-producer-owes-a-step-even-with-nothing-armed: expected false to be true` — and passes with it
restored.

---

## 5. Runtime

<!-- RUNTIME2 -->

---

## 6. What is NOT claimed

<!-- NOTCLAIMED -->
