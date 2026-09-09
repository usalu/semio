# Wave D3 — the fixture's host `ViewModel`, where activation scratch belongs, and the empty suggestion popup

Scope: `📓️2026-09-09-remaining-test-failures-audit.md` §3.2 (the 15 `setActiveTool`/`setActiveUtility`
tests) and §3.4's residue as W-D2 left it (`📓️2026-09-09-wave-D2-window-config-lane-and-scratch.md`
"Defect 3": the popup renders but `session.brush_candidates(…)` is empty), plus W-D2 item 4's open
question — where the `setActiveTool` scratch clearing belongs now that the verb never reaches the app.

Toolchain for every command below:

```
RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 \
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d \
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 \
  editor::puzzle3d::component -- --test-threads=1
```

---

## 0. Conditions this wave ran under (read this before any number below)

The crate did not compile for the first ~50 minutes of this wave: a peer's
`ArtifactCompositionFields` sweep had reached `ArtifactEditor::Snapshot`'s bound but not
`Puzzle3dPlaySnapshot`. Their own delegating impl landed at
`🧬️schema/🧬️mutations/🦀️.rs:515-524` (it forwards to `Puzzle3dSnapshot`, which gets the trait from
`#[derive(ArtifactSchema)]`), and nothing of this wave's was needed to unblock it.

The workspace then broke twice more while this wave verified — once in
`🧰️framework/…/🔌️plugin/🦀️.rs` (`ArtifactStoreReplacementAdmissionTarget` missing
`child_content_generation`), once in `🧰️framework/…/🔌️plugin/⚛️reactor/📮️requests/🦀️.rs`
(non-exhaustive `Slot::Continuation` match). Neither is this wave's file and neither was touched.

Machine load stayed between **60 and 104** for the whole window (peer `rustc`/`node`/`bun`). That is
the exact condition W-D2 §5 demonstrated the `job-session.terminal-fault` step-budget quarantine to be
sensitive to, so every failure count below is a floor, not a measurement of this tree.

---

## 1. Item 1 — the testkit owns ONE host `ViewModel`

### The defect

`✏️editor/🧪️tests/🔬️testkit/🦀️.rs` minted a **fresh** `ViewModel` inside every `dispatch` /
`render_body` / measures call (the free `window_view(window_id)` helper, deleted by this wave). The
framework dispatches `setActiveTool`/`setActiveUtility` as an empty `Emit`
(`🧰️framework/…/🔌️plugin/🦀️.rs:21083-21087`, the `matches!(action, SET_ACTIVE_TOOL_ACTION_ID |
SET_ACTIVE_UTILITY_ACTION_ID)` arm) — the app writes **nothing**, by design, because active tool and
active utility are host `ViewModel` state (`📓️2026-09-09-peer-config-runtime-split.md` §1(d)). So the
activation had nowhere at all to live between two fixture calls, and the next `render`/`window_measures`
saw `active_tool_id: None` and an empty `active_utility_by_window_id` — a state no real host can be in.

### The fix

| file:line | change |
| --- | --- |
| `🔬️testkit/🦀️.rs:125-139` | `Puzzle3dApp` gains one owned `view: ViewModel` — the host's session: window roster, mode-wide `active_tool_id`, per-window `active_utility_by_window_id`. Docstring names the empty-`Emit` dispatch arm as the reason it must live here. |
| `🔬️testkit/🦀️.rs:141-144` | `resolve_activation(current, requested)` — `resolveUtilityActivation` (`🛠️ShellHelpers/🟦️.tsx:2978`) verbatim: an empty request, or re-requesting what is already active, deactivates. |
| `🔬️testkit/🦀️.rs:161-166` | `ensure_window` — the roster grows exactly as the shell's `sessionWindowInstances` does when a pane opens or splits. Idempotent. |
| `🔬️testkit/🦀️.rs:170-176` | `Puzzle3dApp::window_view(window_id)` replaces the free fn: the whole live session, addressed at one instance through the framework's own `ViewModel::for_window_instance` (which is what stamps `active_utility_id` for that pane). |
| `🔬️testkit/🦀️.rs:183-203` | `activate` — `🏛️ShellHost/🟦️.tsx`'s two branches (`SET_ACTIVE_UTILITY_ACTION_ID` at ~5187, `SET_ACTIVE_TOOL_ACTION_ID` at ~5232): resolve, write the session, then enforce the mutual exclusion (a tool clears every window's utility; a utility clears the tool). The window is `args.windowId` when present, else the dispatch's own window — the shell's `activeWindowIdRef` fallback. |
| `🔬️testkit/🦀️.rs:357-364` | `dispatch` registers the window, applies `activate` for the two host verbs **before** forwarding, and builds `ActionMeta.view_state` from the updated session — the shell's own order, so the very call that activates already carries the activation. |
| `🔬️testkit/🦀️.rs:428-431` | `render_body` renders through the same session instead of a fresh `ViewModel`. |
| `🔬️testkit/🦀️.rs:581-583` | `fill_ready` reads `tool_measures` through the session (this is how the fill tool is seen as active at all). |
| `🔬️testkit/🦀️.rs:620-622` | `context_menu_for_selection` stopped passing `ViewModel::default()`; a host never asks for a context menu without its own session. |
| `🔬️unit/🦀️.rs` (20 sites) | every `window_view(…)` call site now reads `app.window_view(…)`; the `window_measures`/`tool_measures` sites bind the view first because those accessors take `&mut self`. |

No plugin-side copy of the host state was introduced anywhere: `Puzzle3dConfig`,
`Puzzle3dWindowConfig` and `Puzzle3dWindowTransient` still carry neither `active_tool_id` nor an
active-utility field, and `window_ownership::runtime` still only *projects* `view.active_tool_id`
read-only into the call-scoped runtime shim.

---

## 2. Item 1b — where the scratch clearing belongs, and why

`✏️editor/🎮️commands/🧰️set-active/🦀️.rs` cleared three things on every tool switch:
`runtime.suggestion_menu`, `runtime.engagement_input`, `runtime.brush_candidate_index`. It was
unreachable: `dispatch_action` never routes `setActiveTool` into `A::handle`, so
`dispatch_puzzle3d_action`'s `SET_ACTIVE_TOOL_ACTION_ID` arm could only fire if something dispatched the
typed `Puzzle3dCommand::SetActiveTool` directly, which nothing does (and `PUZZLE3D_RETAINED_TOOL_IDS`
deliberately excludes it — `🔬️unit/🦀️.rs:155-156` is the law).

**Decision: the window-transient lane, keyed by a view change the app observes on every call.**
`Puzzle3dWindowTransient` gains `activation: String` — the host activation the scratch was captured
under — and every read of the transient drops scratch whose activation has moved on.

| file:line | change |
| --- | --- |
| `✏️editor/🪟️window/🦀️.rs:69-96` | `Puzzle3dWindowTransient.activation`, with the docstring stating explicitly that it is a provenance stamp, not a copy of host state: the app never reads it to answer "what is active", only "is what I am holding still mine". |
| `✏️editor/🪟️window/🦀️.rs:88-96` | `host_activation(view)` — the mode-wide tool, else the addressed window's utility (`for_window_instance` stamps `active_utility_id` from the per-window map), else nothing. |
| `✏️editor/🪟️window/🦀️.rs:224-234` | `live_transient(transient, activation)` — one string comparison, no allocation on the matching path. |
| `✏️editor/🪟️window/🦀️.rs:236-237` | `runtime(…)` runs every captured transient through `live_transient` first, so `handle`, `render`, `window_measures`, `window_engagements`, `tool_measures` and `context_menu` all get the rule for free. |
| `✏️editor/🪟️window/🦀️.rs:262-266` | `transient(runtime, view)` stamps what this turn retains. Both `transient_before` (`✏️editor/🦀️.rs:2867`) and `transient_after` (`:2919`) pass the same `view_state`, so the stamp can never by itself make the publication diff non-empty. |
| `✏️editor/🪟️window/🦀️.rs:154-160` | `puzzle3d_window_transient_retained_bytes` folds the new owned string in. |
| `✏️editor/🪟️window/🦀️.rs:144` | the retirement member list gains `activation`. |
| `✏️editor/🪟️window/🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}` | schema-first: the field is declared in all four language surfaces before the Rust record carries it. |
| `✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs` (appended) | two new laws — `window_transient_scratch_does_not_outlive_the_activation_it_was_captured_under` and `host_activation_reads_the_tool_first_then_the_addressed_window_utility`. |
| removed | `✏️editor/🎮️commands/🧰️set-active/🦀️.rs`, its `pub mod set_active` (`🧊️3d/🦀️.rs`), its import and its `dispatch_puzzle3d_action` arm (`✏️editor/🦀️.rs`), and its now-unused `🧰️set-active` entry in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` `members-of-commands`. |

**Why not an explicit framework hook.** A `host_activation_changed` callback would (a) be a *push* of
host state into the plugin, which is the duplicate the ownership ticket just removed — the app would
have to remember what it was last told; (b) require the framework to re-enter an app for two verbs it
deliberately answers itself with an empty `Emit`, undoing `dispatch_action`'s own simplification; and
(c) still miss every activation change that does NOT arrive as one of those two verbs — a plain
`refresh-ui` carrying a different `activeUtilityByWindowId`, a pane closing, a mode switch. The
view-keyed rule is a pure function of state the host already sends on **every** call, needs no new
framework surface, and is self-healing.

**Two consequences, stated rather than hidden.** (i) The stale record is not *erased* from the transient
store on an activation change — it becomes unreadable, and is overwritten by the next scratch write.
That is the observable semantics the deleted reducer had; erasing it eagerly would need a write from a
turn that has no other reason to publish. (ii) `set_active` also called `drive_precompute` when the
brush utility was activated, warming the brush lane one slice early. That warm now happens on the
first `suggestionsTick` (the host's own 120 ms tick) instead, and — for the suggestion popup, which is
the case a user reads immediately — in the retained warm stage of §3.

---

## 3. Item 2 — the suggestion popup opened with no candidates

### The defect

`open_vortex_suggestions` warms its target inline:
`🎮️commands/🔓️open-vortex-suggestions/🦀️.rs:26-27` calls `invalidate_brush_target` then
`refresh_brush_candidates`. `refresh_brush_candidates`
(`⏳️precompute/🦀️.rs:1470-1482`) takes **one** `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US` slice — 500 µs
(`⏳️precompute/🦀️.rs:888`) — and on an unoptimised build one narrow-phase candidate costs about that
much on its own (the constant's own docstring says so). So the entry it caches is routinely
`free: []`, `unknown_pending: true`, with the target re-queued at the front of the brush queue.

Nothing then drives that queue before the popup is read. `render_body` (`✏️editor/🦀️.rs:7205-7209`)
only **syncs** the precompute session (`sync_precompute_session`) and sets the fill applied count; it
never calls `precompute_step_lane`. `world_interaction_json`
(`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:322-345`) then reads `session.brush_candidates(…)` and emits
`candidates: []` with `pending: true`. The user sees an empty popup.

`acceptSuggestion` did not show the bug because its own arm calls `drive_precompute`
(`🎮️commands/✅️accept-suggestion/🦀️.rs:19`), i.e. a *second* slice of the same lane — two slices are
enough on the default fixture, which is why `accept_suggestion_appends_an_object_and_closes_the_menu`
places an object while `hover_suggestion_updates_the_brush_candidate_index_and_live_preview` renders
nothing.

### The fix

Warm through the retained work's own stages, never inside render.

| file:line | change |
| --- | --- |
| `✏️editor/🦀️.rs:3185-3211` | `Puzzle3dWindowCommandStage::Warm`, plus `PUZZLE3D_SUGGESTION_WARM_UNITS = 8` (lane units per turn — the lane bounds itself on 500 µs, so this is a unit ceiling, not a time one) and `PUZZLE3D_SUGGESTION_WARM_TURNS = 8`. |
| `✏️editor/🦀️.rs:3227-3245` | `Puzzle3dWindowCommandWork` gains `warm_target`, `warm_turns` and a held `emit`, plus `warm_target_of(command)` — the popup's own recorded `fullId`, so only `openVortexSuggestions` warms. |
| `✏️editor/🦀️.rs:3305-3330` | `Dispatch` no longer completes for a popup-opening command: it holds its `Emit` and routes into `Warm`, which spends a **fixed** count of separately-bounded lane turns and then publishes the held emission on its terminal step. |
| `✏️editor/🦀️.rs:3286` | the work's own turn cap becomes `Puzzle3dActionPrologue::WORK_ITEMS + PUZZLE3D_SUGGESTION_WARM_TURNS`. |
| `✏️editor/🦀️.rs:3350-3370` | close cursor and terminal-empty witness extended with `warm_target` and `emit`. |

The warm turn count is **fixed, never "until resolved"**:
`open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`
(`🔬️unit/🦀️.rs:2977`) asserts through `measured_cold_runs` (`:2948`) that every cold run of the same
document takes the *same* number of bounded turns, and a wall-clock-terminated warm would turn that
count into a machine-load reading. A turn whose lane has nothing left to do pops an empty queue and
costs nothing. The 8 ms law holds per turn by construction: each warm turn is one 500 µs lane deadline
plus one `HashMap` lookup.

---

## 4. Verification

*(filled in below)*

---

## 5. Not verified

*(filled in below)*
