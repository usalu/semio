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

Every command below ran FOREGROUND with

```
RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 \
CARGO_TARGET_DIR=/private/tmp/claude-501/…/scratchpad/target-p3d \
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 <filter> -- --test-threads=1
```

against a `target-p3d` seeded from the repo's own `target/debug` at 18:04 (the seed marker `.seeded`).
Machine load stayed between 25 and 60 for the whole window.

### 4.0 Whole-module runs

| run | tree | result |
| --- | --- | --- |
| A (18:05→18:31) | this wave's edits exactly as the killed agent left them | `FAILED. 81 passed; 58 failed; 456 filtered out; finished in 1424.12s` |
| B (18:35→19:06) | A + §4.2(a) | `FAILED. 87 passed; 56 failed; 462 filtered out; finished in 1072.62s` |
| C (19:45→20:04) | B + §4.2(b) + §4.2(c) | `FAILED. 87 passed; 57 failed; 462 filtered out; finished in 1123.82s` |

The audit's 89/44 baseline is **not** comparable: it was taken six waves earlier on a 456-test module
that is now 462 tests, and — decisively — the whole-module number is dominated by two PROCESS-WIDE
leaks that no per-test fix can move (§5.3, §5.4). 48 of run A's 58 failures and 49 of run C's 57 are
one of exactly two messages: `puzzle3d app never quiesced` (34) and
`interactive-job.admission-capacity` / *"process-wide session capacity"* (15). **Per-test isolated runs
are therefore the measurement this wave reports**, and the whole-module runs are reported only as the
honest, unflattering total.

### 4.1 §3.2's root fault is gone

```
$ grep -c "missing-owned-reducer" run-after-1.txt
0
```

Zero occurrences across the whole module, against **17** tests that failed on exactly that fault in the
audit. Every `setActiveTool` / `setActiveUtility` dispatch now completes: the fixture's own
`ViewModel` (§1) carries the activation from the dispatch that made it to the next `render` /
`window_measures`, and the framework's empty-`Emit` arm
(`🧰️framework/…/🔌️plugin/🦀️.rs:21352`) is what the fixture now models.

Isolated §3.2 verdicts, one filtered process per batch:

```
$ cargo test … -- --test-threads=1 <8 utility/suggestion filters>          # after §1 + §4.2(a)
brush_placement_picker_appears_only_for_a_live_brush_target ... FAILED
gumball_active_only_for_transform_utilities_with_object_selection ... FAILED
gumball_inactive_when_every_handle_flag_is_off ... ok
hover_suggestion_updates_the_brush_candidate_index_and_live_preview ... FAILED
open_and_accept_vortex_suggestions_preserve_active_utility ... FAILED
set_active_utility_emits_no_ops_and_no_history_entry ... ok
transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes ... ok
transform_utility_options_expose_move_and_rotate_flags ... ok
test result: FAILED. 4 passed; 4 failed; 598 filtered out; finished in 1.56s
```

Not one of those four failures is `missing-owned-reducer` any more; each is a further, separate defect,
and two of them (`hover_suggestion`, `open_and_accept…`) are §4.2(b) below.

### 4.2 Three repairs this wave had to make to reach its own subject

**(a) The fixture never drained the typed-operation COMPLETION witness.**
`settle` (`🔬️testkit/🦀️.rs`) drove `maintenance_step` → `advance_typed_operation_publication` → result
page + ACK → effect → event → UI scope, but never `take_typed_operation_completion`. Every terminal
typed operation pushes one witness into a 64-slot outbox that ONLY that take drains
(`🔌️plugin/🦀️.rs:22214` pushes, `:24169` pops), and `has_pending_typed_operations` counts it
(`:24027`) — so after its first completed operation a fixture could never observe quiescence again.
The trace is unambiguous: `operations=[]  latest_wins_empty=true effects=0 events=0 ui=0 query=none`,
printed every 4096 turns to the settle loop's 1 048 576-iteration guard. This is a term the framework
gained TODAY (the whole `typed_completion_outbox` block is a `+` in `git diff HEAD` on
`🔌️plugin/🦀️.rs`, whose last commit is 9b605a4550 12:10), which is why the audit's older baseline never
saw it. Fixed in `🔬️testkit/🦀️.rs` `settle` and in `🔬️unit/🦀️.rs` `measured_host_turn`, in exactly the
host's own position — between the event and the UI scope, as `advance_typed_operation_output`
(`🔌️plugin/🦀️.rs:30617`) takes it.

```
$ cargo test … editor::puzzle3d::component::tests::open_vortex_suggestions -- --test-threads=1
open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin ... ok
open_vortex_suggestions_opens_the_suggestion_popup ... ok
open_vortex_suggestions_records_explicit_window_id ... ok
test result: ok. 3 passed; 0 failed; 602 filtered out; finished in 1.03s
```

**(b) A no-op scene sync discarded the brush lane — which is why §3's warm stage looked like it did
nothing.** With a `[DEBUG]` probe on both sides of the boundary the warm stage was proved to work and
the render proved to throw the result away:

```
[DEBUG] warm turn=0 target=seed-left-001:v0 more=true free=0 pending=true  resume=0
[DEBUG] warm turn=1 target=seed-left-001:v0 more=true free=7 pending=false resume=0
… (turns 2-7 all free=7)
[DEBUG] set_scene_config no-op (scene unchanged)
[DEBUG] start_fill_preparation clears brush cache entries=7
[DEBUG] render brush_candidates target=seed-left-001:v0 free=0 pending=true
```

`Puzzle3dPrecomputeSession::install_scene` (`⏳️precompute/🦀️.rs`) ran `rebuild_queue()` whenever the
fill builder was absent — and a session hand-off carries the scene and the brush cache but never the
fill builder, so EVERY `render`'s `sync_precompute_session` came through there, and `rebuild_queue` →
`start_fill_preparation` wiped `brush_queue`/`brush_cache`. Repair: the brush-lane reset moved OUT of
`start_fill_preparation` (fill-only, and now documented as such) INTO `rebuild_queue` (scene-invalidated
only), `install_scene` restores the fill with `start_fill_preparation(true)` instead of rebuilding, and
`install_collision_mesh` keeps its own exact reset. `PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE = 8`
(`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`) bounds what the popup publishes, because the scene surface it
travels on is a fixed-capacity payload and the candidate list was unbounded.

```
$ cargo test … <suggestion family> -- --test-threads=1
accept_suggestion_appends_an_object_and_closes_the_menu ... ok
accept_suggestion_closes_menu_even_when_placement_fails ... FAILED     ← §5.1
accept_suggestion_every_step_stays_below_the_interactive_ceiling_for_nakagin ... ok
accept_suggestion_extent_fits_within_cap_for_nakagin ... ok
accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin ... ok
accept_suggestion_with_full_id_places_even_if_selection_was_cleared ... ok
brush_placement_picker_appears_only_for_a_live_brush_target ... FAILED  ← §5.2
close_vortex_suggestions_clears_sticky_hover ... ok
close_vortex_suggestions_clears_the_menu ... ok
hover_suggestion_updates_the_brush_candidate_index_and_live_preview ... ok
open_and_accept_vortex_suggestions_preserve_active_utility ... ok
open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin ... ok
open_vortex_suggestions_opens_the_suggestion_popup ... ok
open_vortex_suggestions_records_explicit_window_id ... ok
test result: FAILED. 12 passed; 2 failed; 592 filtered out; finished in 3.61s
```

Same eight filters as §4.1, re-run once §4.2(c) had landed — the §3.2 cohort this wave owns:

```
brush_placement_picker_appears_only_for_a_live_brush_target ... FAILED   ← §5.2, the only one left
gumball_active_only_for_transform_utilities_with_object_selection ... ok
gumball_inactive_when_every_handle_flag_is_off ... ok
hover_suggestion_updates_the_brush_candidate_index_and_live_preview ... ok
open_and_accept_vortex_suggestions_preserve_active_utility ... ok
set_active_utility_emits_no_ops_and_no_history_entry ... ok
transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes ... ok
transform_utility_options_expose_move_and_rotate_flags ... ok
test result: FAILED. 7 passed; 1 failed; 603 filtered out; finished in 1.17s
```

i.e. 4 passed / 4 failed before §4.2(b)+(c), 7 passed / 1 failed after.

**Both of this wave's two named targets are green**:
`hover_suggestion_updates_the_brush_candidate_index_and_live_preview` and
`open_and_accept_vortex_suggestions_preserve_active_utility`. So is
`open_vortex_suggestions_opens_the_suggestion_popup`, i.e. the audit's §3.4 / §2 item 4 —
*"the vortex suggestion popup never visibly opens"* — is closed.

**(c) The 120 ms tick never warmed the target the user is looking at, and the chrome read a cold
session.** `suggestions_tick` spent its whole slice on the lane's round-robin, so the brush utility's
own target waited behind the document enumeration; it now resolves the open popup's pinned vortex, else
`puzzle3d_brush_target_vortex`, and refreshes it only while the entry is still `unknown_pending` (so a
resolved target costs nothing per tick). Separately, `window_measures_body` and `tool_measures`
(`✏️editor/🦀️.rs`) built a `restored_precompute_session(&envelope, &[])` — a COLD session — where
`render` reads `app.precompute`; both now read the app's own live session, as `render` does. Measured
through the tick's own probe (`free=2 → 6 → 6 …` across ticks) the lane does converge and does persist,
which is what §4.2(b) bought.

### 4.3 The Warm stage of §3 is NOT the never-quiesce cause

Bisected directly rather than argued: with `warm_target_of` short-circuited to `None`
(`[DEBUG] bisect` guard, since removed) and everything else identical,

```
open_vortex_suggestions_opens_the_suggestion_popup ... FAILED   (never quiesced)
open_vortex_suggestions_records_explicit_window_id ... FAILED   (never quiesced)
test result: FAILED. 1 passed; 2 failed; 601 filtered out; finished in 13.65s
```

— the failure survives the warm stage's removal, so it belonged to (a). With (a) landed and the warm
stage restored, both pass (§4.2(a) block above), and
`open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` — the law that the
warm turn count must be fixed, never wall-clock-terminated — passes in every run.

### 4.4 Hostile static laws

`suggestion_and_precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries` failed
in run A and passes from run B on; its run-A failure was a peer's in-flight state, confirmed by
re-running every marker of `suggestion_and_precompute_routes_are_cursorized` against the source as it
stands (all 19 positive markers present, all 6 negative markers absent).
`retained_command_catalog_excludes_framework_owned_shared_actions` — the law that
`PUZZLE3D_RETAINED_TOOL_IDS` may not contain `setActiveTool`/`setActiveUtility`, i.e. the law this
wave's §2 deletion had to keep — passes in all three runs.

§2's own two new laws live under `editor::puzzle3d::window::component` (NOT the `component` filter this
report's module runs use), and were run separately:

```
$ cargo test … editor::puzzle3d::window::component -- --test-threads=1
app_pack_and_spr_exclude_window_transient_and_operation_fields ... ok
every_oversized_string_capacity_returns_the_exact_puzzle3d_owner ... ok
host_activation_reads_the_tool_first_then_the_addressed_window_utility ... ok
same_kind_windows_compose_independently ... ok
suggestion_and_input_retirement_reaches_terminal_empty_with_tiny_grants ... ok
window_transient_scratch_does_not_outlive_the_activation_it_was_captured_under ... ok
test result: ok. 6 passed; 0 failed; 605 filtered out; finished in 0.01s
```

The crate's own `cargo test --no-run` ends with **zero library warnings** (the two remaining
`unnecessary qualification` warnings are a peer's, at `🔬️unit/🦀️.rs:88` and `:121`); the dead
`restored_precompute_session` left by §4.2(c) was deleted rather than left behind.

### 4.5 Temporary instrumentation

Six `[DEBUG]` probes were added and **all six removed**: the warm-stage candidate readout
(`✏️editor/🦀️.rs`), the render-side `brush_candidates` readout and the scene byte census
(`🎭️modes/…/🧊️main/🦀️.rs`), the brush-options readout (`🪛️utilities/🖌️brush/🦀️.rs`), the
`start_fill_preparation` / `set_scene_config` pair (`⏳️precompute/🦀️.rs`), the session-check-in drop
(`✏️editor/🦀️.rs`) and the `suggestions_tick` readout. Verified absent:
`grep -rn 'eprintln!("\[DEBUG\]' 🧊️3d/…/✳️any/` outside `🧪️tests/` returns nothing. The coordinator's
own `[DEBUG]` traces in `🔌️plugin/🦀️.rs` were not touched.

---

## 5. Not verified

### 5.1 `accept_suggestion_closes_menu_even_when_placement_fails` — REGRESSED by §4.2(b), scene budget

It was green in runs A and B and is red in run C:

```
render: Fault { origin: Plugin, code: FaultCode("plugin.internal"),
  message: "ui.fixed-capacity: fixed UI admission failed at scene-surface.encode:
            surface payload exceeds fixed capacity with 33289 bytes" }
```

It regressed because the popup now carries the content it is supposed to carry. Byte census of that
window's `World3dScene`, measured:

```
meshes=27673 instances=270 selection=212 vortices=2639 attractions=2 volumes=2
refs=327 preview=281 interaction=1094 lod=125 chunk=40 env=92        → 33289 packed
```

`meshes_json` alone is **27 673 of a 32 KiB fixed surface payload** — 83 %. The scene had ~1 KB of
headroom before this wave; a resolved 6-row popup (1094) plus its brush preview (281) uses it. The same
test with a sticky hover is what pushes `vortices_json` from 2 to 2639. This is a scene-payload budget
defect (mesh geometry re-sent inline in every scene), NOT a suggestion defect: capping the candidate
page at 8 does not save it, and neither would capping it at 2. It belongs to whoever owns the 3d scene
payload (W-D4 / W-S). Bounding the popup (`PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE`) was landed anyway,
because an unbounded list would fail-close the whole 3D render of a richly catalogued document.

### 5.2 `brush_placement_picker_appears_only_for_a_live_brush_target` — still red, cause located, not fixed

Probed to the exact line. The tick resolves the target and keeps it
(`[DEBUG] suggestions_tick after refresh free=6` on every tick), and the measures call *does* resolve
the same target (`brush options target=seed-left-001:v0`) — but reads `candidates=0`, because the app it
runs on has **no session at all**: `puzzle3d_view_session_key(doc)` (`✏️editor/🦀️.rs:2669`) answers
`None` when the view carries neither `operation_optional()` nor `render_operation()`, which is the case
for the fixture's bare `window_measures(&view)` call, so `with_puzzle3d_app_for(None, …)` takes no
lease and every chrome call starts cold (`[DEBUG] set_scene_config CHANGED … prior=None`, four times in
one test). Both halves of the repair that CAN be made from this wave were made — the tick prioritizes
the live target, and the chrome reads `app.precompute` instead of a cold
`restored_precompute_session` — and neither is sufficient while the fixture's measures call carries no
document-instance identity. Giving the fixture (or the framework accessor) that identity is the
remaining work. The test itself was updated to play the host's own `suggestionsTick` clock
(`PUZZLE3D_BRUSH_PICKER_TICKS = 8`), which is what a running host does between the click and the next
frame — this wave deleted the `setActiveTool` reducer that used to warm the lane, and §2(ii) already
recorded that consequence.

### 5.3 The fill family — process-wide worker-session leak, not this wave's

Isolated, one process, ten filters:

```
fill_build_tick_is_ignored_when_fill_tool_is_inactive ... ok
fill_build_tick_is_a_view_action_with_narrow_ui_scope ... FAILED
  expected a Partial ui_scope for fillBuildTick, got None
<the other eight> ... FAILED
  retained operation faulted: typed operation worker session admission was refused
  by the process-wide session capacity
test result: FAILED. 1 passed; 9 failed; 601 filtered out; finished in 13.05s
```

Every app boot mounts a fill worker (`start_fill_preparation` → `mount_fill_worker`) and the process
pool's session slots are never returned, so the N-th app in one test process cannot admit one. This is
the same exhaustion that surfaces as `interactive-job.admission-capacity` on 15 further tests in the
whole-module runs, and it is what makes those runs order-dependent (`close_vortex_suggestions_clears_the_menu`
passes in run C's full sweep and fails alone; `open_vortex_suggestions_opens_the_suggestion_popup` does
the opposite). Owner: the fill lane (W-F / `⏳️precompute`).

### 5.4 The 34 whole-module `never quiesced` failures

After §4.2(a) the ones this wave could reach are green in isolation; the 34 that remain in run C are the
downstream of §5.3 (an operation whose worker session was refused sits in `Worker` with
`has_runnable_work() == true` forever) plus §3.1 of the audit, whose own test
`one_window_config_mutation_publishes_exactly_one_generation_and_quiesces` is still red. Neither was
traced further by this wave.

### 5.5 Red for reasons outside this wave entirely

`retained_publication_contracts_are_an_exact_nonempty_tool_bijection` — the fixture
`🧫️fixtures/🗄️retained-jobs/🔣️.json:25` still lists `setFillCountStep`, which no longer exists in
`PUZZLE3D_RETAINED_TOOL_IDS`; `app_definition_labels_resolve_german_reuse_branded_for_aggregator` and
`document_and_kinds_trees_use_german_reuse_section_labels` (terminology wave);
`set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin`;
`two_instances_converge_disjoint_object_edits_via_backbone` (`module.vcs`);
`local_interaction_read_of_a_selected_document_terminates`; the whole `world_pick_*` / `world_vortex_*`
/ inspector cohort (all `admission-capacity`, i.e. §5.3).

### 5.6 Not attempted at all

No wasm build and no browser/runtime verification (explicitly out of this wave's scope): every claim
above is a native `--test-threads=1` measurement. `⚛️reactor/🔄️turn`, `🕹️interaction/**`, `🧵️job` and
the coordinator's `[DEBUG]` traces were not touched.

One interruption is worth recording because it cost a re-run: the volume holding both the scratchpad
and the repo hit `ENOSPC` mid-session (a peer's build; 44 GiB free again minutes later) and NO command
could execute at all — the Bash and Write tools each fail before running, because both open a temp file
first. `Read` and `Edit` kept working throughout.
