# Remaining puzzle3d test failures — audit (2026-09-09)

Read-only audit. Scope: `semio-s-artifact-puzzle-3d`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`, feature `component-app-assembly`),
module `editor::puzzle3d::component::tests`, after waves X, D, S, O, P, P2, B, P3. No source file was
edited by this audit.

Toolchain for every command below:
```
RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 \
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d \
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 \
  editor::puzzle3d::component -- --test-threads=1 [--nocapture]
```

## 0. The crate is under live edit — two runs, five minutes apart, disagree by 5 tests

`git status --short` at audit start showed 14 modified files under `✏️editor/**` (`🦀️.rs`, `⏳️precompute/**`,
`🪟️window/🦀️.rs`, `🧪️tests/**`), all already **staged**, none with a further unstaged diff — i.e. a
concurrent session is actively landing a wave on top of P3 while this audit ran (mtimes at 11:53-11:54,
during this audit's first full run). Two consecutive full-module runs disagree:

| run | result | delta vs. the other run |
| --- | --- | --- |
| 1 (11:5x) | 88 passed / **45** failed | `add_object_kind_honors_drop_origin`, `close_vortex_suggestions_clears_sticky_hover`, `world_vortex_click_replaces_until_invertive_mode_is_selected` failed here only |
| 2 (12:0x) | 89 passed / **44** failed | `window_transient_is_exact_instance_local_and_resets_on_reload`, `world_pick_locked_object_clears_like_background` failed here only |

89/44 matches W-P3's own recorded baseline exactly, so **run 2 is used as the authoritative snapshot**
below (full list, every failure's assertion message, in
`/private/tmp/claude-501/…/scratchpad/failures-nodebug-2.log`, kept only in the scratchpad per
instructions — delete-on-close applies). The 5 tests that flip between runs are noted in §4 (bucket d).
`✏️editor/🦀️.rs` itself is confirmed **stable** across both runs — no unstaged diff, and the one function
this audit relies on most (`host_configuration_mutation`, §1) is untouched by the staged changes
(`git diff --cached` on that region is empty).

---

## 1. Summary

| bucket | count | description |
| --- | ---: | --- |
| (a) real production defect | 32 | reduces to **4** distinct root causes, all with file:line |
| (b) stale test assumption | 4 | fixture/test drift, cited against a decision or an omission |
| (c) framework-side defect | 8 | framework or cross-cutting infra, not puzzle3d's own logic |
| (d) peer in-flight churn | 5 | flip pass/fail between the two runs above, not counted in the 44 |
| **total (run 2)** | **44** | |

## 2. Ordered (a) items by user impact

1. **Any per-window setting (camera, projection, grid, vortex show/direction, window options) hangs the
   app forever the first time it is touched.** §3.1. Worst possible symptom — not a fault, an infinite
   spin — and it is the single largest reason a user session would be judged broken.
2. **The fill tool and every "utility" (Transform/gumball, etc.) cannot be selected at all.** §3.2.
   17 of the 44 failures. Re-opens P2 (wave D), which is now provably undone.
3. **Adding an object kind to an empty or just-cleared document does nothing, silently.** §3.3. Blocks
   building up a document from scratch — the very first thing a new user would do.
4. **The vortex "suggestion popup" never visibly opens** (its `open`/`candidates`/`windowId` fields are
   dropped before render). §3.4. A real but narrower interaction gap (suggestions, hover-to-preview,
   accept-with-fallback all share it).

---

## 3. Bucket (a) — real production defects, with fix location

### 3.1 WindowConfig-lane publication never quiesces — 6 tests, reproduced in isolation, 100% repeatable

Tests: `camera_actions_are_view_actions_that_emit_no_artifact_mutations`,
`grid_window_options_control_one_visible_grid_spacing`,
`vortex_direction_option_is_local_to_the_window_instance`,
`window_options_are_local_to_the_window_instance_not_shared_across_split_panes`,
`world_vortices_carry_their_own_selected_and_hovered_flags`,
`world_vortices_reveal_in_selected_mode_only_for_the_selected_object`.

All fail identically:
```
✏️editor/🧪️tests/🔬️testkit/🦀️.rs:255:5
puzzle3d app never quiesced: pending typed operations outlived the settle budget
```

Isolated, single-process, single-test reproduction (no cross-test state, `--nocapture`):
```
$ cargo test … camera_actions_are_view_actions_that_emit_no_artifact_mutations -- --test-threads=1 --nocapture
[DEBUG] typed-operation publication turn=4096 operations=["2:Publishing:true:true"] …
[DEBUG] typed-operation publication turn=8192 operations=["2:Publishing:true:true"] …
   … (identical line, 256 times, turn incrementing by 4096 each time) …
[DEBUG] typed-operation publication turn=1048576 operations=["2:Publishing:true:true"] …
thread '…camera_actions…' panicked at …testkit/🦀️.rs:255:5: puzzle3d app never quiesced …
test result: FAILED. 0 passed; 1 failed …; finished in 41.52s
```
The test does exactly **one** `dispatch(&mut live, "setCamera", …, None)` on a freshly booted single-window
app (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1531-1541`) — no multi-window setup is needed to trigger it. Operation
id `2` (the `setCamera` publication) sits in stage `Publishing` for the settle loop's entire
`1_048_576`-iteration runaway bound (`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:238-255`) and never reaches
`AwaitingAck`; it is not a *slow* operation, it is a **stuck** one — the loop bound is what ends the test,
not progress.

`setCamera` (and `setProjection`, `setGridSpacing`, `setVortexShow`, `setVortexDirection`, and the rest of
the `Puzzle3dScalarConfigWork` family — `✏️editor/🦀️.rs:6319`) publish through the **`WindowConfig`**
lane, not the artifact `Config` lane P3 fixed:
```
✏️editor/🦀️.rs:6258
ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] }
```
which is a per-window store built entirely from generic framework machinery puzzle3d only registers:
```
✏️editor/🪟️window/🦀️.rs:151-153   register_config()/register_transient()
✏️editor/🪟️window/🦀️.rs:129-141   Puzzle3dWindowConfigOwner (65 536-byte Snapshot-only mutation)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:332,404,519  advance()
```
`Puzzle3dWindowConfig`/`Puzzle3dWindowTransient` and the `WindowConfigOwner`/`WindowTransientOwner`
plumbing did not exist before the live in-flight `Puzzle3dConfig` → `Puzzle3dRuntime` split this audit
found staged (§0) — no prior wave (X/D/S/O/P/P2/B/P3) exercised or measured this lane at all, so this is a
**new** regression, not a persisting one. The stuck state is inside the generic per-window
`ArtifactStoreOneItemPreparationFactory::advance` cycle; this audit did not isolate which of its steps
never returns a terminal `ArtifactStoreOneItemAdvance` — that needs a next-wave trace with a temporary
probe on `advance()`, following the same method W-P3 §1 used for the artifact-Config lane.

**User-visible symptom:** open the app, touch the camera, the grid spacing slider, the vortex
show/direction toggle, or any per-window option — the UI never responds again (the retained operation
never completes, so nothing else waiting behind it can run either).

### 3.2 `setActiveTool` / `setActiveUtility` are dead again — P2 (wave D) undone — 17 tests

Tests (10 via `setActiveTool` — every fill-tool test — + 7 via `setActiveUtility` — every
transform/gumball test): `fill_build_tick_is_a_view_action_with_narrow_ui_scope`,
`fill_build_tick_is_ignored_when_fill_tool_is_inactive`, `fill_build_tick_only_plans_available_slider_range`,
`fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`,
`fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances`,
`fill_render_reveals_the_full_available_plan_tagged_with_reveal_index`,
`set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up`,
`set_fill_count_declares_narrow_ui_scope`,
`set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count`,
`set_object_kind_weight_declares_fill_options_ui_scope`;
`brush_placement_picker_appears_only_for_a_live_brush_target`,
`gumball_active_only_for_transform_utilities_with_object_selection`,
`gumball_inactive_when_every_handle_flag_is_off`, `open_and_accept_vortex_suggestions_preserve_active_utility`,
`set_active_utility_emits_no_ops_and_no_history_entry`,
`transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes`,
`transform_utility_options_expose_move_and_rotate_flags`.

All fail with the exact same fault, differing only in which verb:
```
Fault { origin: Framework, code: FaultCode("interactive-job.missing-owned-reducer"), …
  message: "typed command 'setActiveTool' has only a generic proof and no exact app-owned retained
  decoder/reducer factory" … }
```
(or `'setActiveUtility'`). Root cause, read off the framework and confirmed against the currently
**staged** (not mid-edit — `git diff --cached` on this region is empty) source:

```rust
// ✏️editor/🦀️.rs:6776
fn host_configuration_mutation(_action: &str, _args: Option<&dsl::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
    Ok(None)
}
```
always returns `None`. `dispatch_action`'s host-configuration branch
(`🧰️framework/…/🔌️plugin/🦀️.rs:19953-19961`) is:
```rust
if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {
    // uses require_tool_operation_authority — a generic Bounded proof satisfies this
} else {
    let admission = self.admit_command_json(action, args).await?;
    self.require_complete_tool_operation_pipeline(&admission)?;   // ← needs factory_type
    …
}
```
Since `host_configuration_mutation` never returns `Some`, `setActiveTool`/`setActiveUtility` always fall
into the `else` arm. The only proof registered for those two ids is the deliberately **generic** one
(`✏️editor/🦀️.rs:6677-6684`, `Puzzle3dHostConfigurationProofs`, `factory: "BoundedFirstStepCommandJobFactory"`,
no `factory_type`), which `require_complete_tool_operation_pipeline` refuses —
`interactive-job.missing-owned-reducer` is exactly that refusal.

This is P2 from wave D, previously fixed (`✏️editor/🦀️.rs`'s `host_configuration_mutation` used to match
`SET_ACTIVE_TOOL_ACTION_ID`/`SET_ACTIVE_UTILITY_ACTION_ID` and return a real
`Puzzle3dConfigMutation::SetActiveTool`/`SetWindowEngagementInput`). It regressed when a peer's
`Puzzle3dConfig` → `Puzzle3dRuntime` split moved `active_tool_id` off `Puzzle3dConfig` onto
`Puzzle3dRuntime` (`✏️editor/🎚️config/🦀️.rs:179`) — `Puzzle3dConfigMutation` no longer has a variant that
can express "set the active tool", so the hook was reduced to a stub rather than re-landed. This
collision — and exactly what re-landing needs — was already flagged by wave D itself
(`📓️2026-09-09-wave-D-production-defects.md` §8), but it is still unresolved as of this run.

**User-visible symptom:** clicking the fill-tool icon, or any utility (move/rotate gumball, etc.), in the
toolbar does nothing — every dispatch of `setActiveTool`/`setActiveUtility` faults closed.

### 3.3 `addObjectKind` silently no-ops on a document with no kind catalogs — 5 tests

Tests: `add_object_kind_materializes_the_declared_kind_default`,
`duplicate_selection_reselects_the_created_clones`, `gumball_translate_drag_coalesces_into_one_edit`,
`gumball_gesture_commits_one_absolute_delta_between_its_host_brackets`,
`select_same_kind_widens_the_selection_to_every_object_of_that_kind`.

All five call `dispatch(…, "setActiveExample", { "exampleId": "" }, …)` (loads the **empty** example,
which clears `meta.kind_catalogs`) and then `addObjectKind` — and get zero objects back, either via a
direct assertion (`add_object_kind_materializes_the_declared_kind_default`:
`assert!(!result.mutations.is_empty(), "addObjectKind is a Mutation that emits mutations")` fails because
`result.mutations` **is** empty) or downstream, via `first_object_id`'s `.expect("first object id")`
panicking on an empty `objects` array.

```rust
// ✏️editor/🦀️.rs:3596-3600  Puzzle3dAddObjectKindWork::step
let Some(catalogs) = snapshot.typed().meta.kind_catalogs.as_ref() else {
    self.stage = Puzzle3dAddObjectKindStage::Complete;
    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
};
```
The docstring right above it (`:3583-3586`) says this branch exists so a no-catalog document completes in
"ONE turn, not a refusal" — a wave D fix for a preflight fault
(`📓️2026-09-09-wave-D-production-defects.md` §3, "not in P1-P10"). That fix stopped the *fault*, but it
also made the branch a **no-op** (`Emit::default()`, no mutation at all) instead of materializing the
declared default kind the way `add_object_kind_materializes_the_declared_kind_default`'s own docstring
says the P1 arg-form contract requires ("firing `addObjectKind` with no args must materialize the declared
`objectKind` default"). Wave D's own report never claimed the no-op branch does the right thing — it only
claimed it does not fault — and this run shows it does not.

**User-visible symptom:** clear the document (or start from empty/blank), click "Add Object" — nothing is
added, no error is shown.

### 3.4 The vortex suggestion popup's rendered state (`open`/`candidates`/`windowId`) never lands — 4 tests

Tests: `open_vortex_suggestions_opens_the_suggestion_popup`,
`open_vortex_suggestions_records_explicit_window_id`, `hover_suggestion_updates_the_brush_candidate_index_and_live_preview`,
`accept_suggestion_closes_menu_even_when_placement_fails`. Representative failure:
```
✏️editor/🧪️tests/🔬️unit/🦀️.rs:1294:5
assertion `left == right` failed
  left: None
 right: Some(true)                    // interaction.pointer("/suggestionMenu/open")
```
The command writes the menu into the in-memory runtime unconditionally:
```rust
// ✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs:20
ctx.scene.runtime.suggestion_menu = Some(Puzzle3dSuggestionMenu { x, y, window_id, vortex_full_id: full_id.clone() });
```
and the render side reads it back the same way (`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:322,372`,
`let suggestion_menu = runtime.suggestion_menu.as_ref()… "suggestionMenu": suggestion_menu`) — but by the
time render runs, `runtime.suggestion_menu` is back to `None`. `openVortexSuggestions` was moved onto the
staged `Puzzle3dWindowCommandWork` route in wave P3 specifically because the prior fallback route
(`BoundedFirstStepCommandWork`) "drops the `EphemeralEmit` the suggestion popup is carried in"
(`📓️2026-09-09-wave-P3-command-prologue.md` §6.1) — that move measurably helped one of the four tests
(`open_vortex_suggestions_opens_the_suggestion_popup` now reaches this render assertion instead of
faulting, per the same report) but did not fix the underlying drop. The most likely remaining gap is the
window-transient diff that is supposed to carry `runtime.suggestion_menu` into a published
`Puzzle3dWindowTransient` mutation (`✏️editor/🦀️.rs:~2918-2920`,
`let transient_after = window_ownership::transient(&scene.runtime); … window_ownership::addressed_transient(view, transient_after)`)
— not isolated further within this audit's budget.

**User-visible symptom:** hovering/right-clicking a vortex to see placement suggestions never visibly
opens the popup (nor updates the live brush-candidate preview, nor closes cleanly when placement fails).

---

## 4. Bucket (b) — stale test assumption, with the decision cited

| test | assertion | why it's stale |
| --- | --- | --- |
| `retained_publication_contracts_are_an_exact_nonempty_tool_bijection` | fixture `toolIds` (left) contains `"setFillCountStep"`; `PUZZLE3D_RETAINED_TOOL_IDS` (right, `✏️editor/🦀️.rs:3044`) does not | `setFillCountStep` was deliberately excluded — "no `build_tool_job` arm; generic reducer's default arm is `_ => {}` — would migrate into a **silent** no-op" (`📓️status.md`, "Wave 2" section). The fixture `✏️s/…/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json`'s `toolIds` array was never updated to match; it carries no uncommitted/staged diff, so this is a settled drift, not a mid-edit artifact. |
| `suggestion_and_precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries` | `assertion failed: suggestion_and_precompute_routes_are_cursorized(source)` | Requires the source to contain `Puzzle3dPrecomputeCommandStage::CheckpointBytes`; that variant does not exist — a peer removed the stage wave P introduced without retargeting the law. Documented as already-red, not this run's regression, in `📓️2026-09-09-wave-P3-command-prologue.md` §5/§6.2, confirmed still red here. |
| `vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards` | `.expect("main window measures")` panics | Calls `app.window_measures(&semio_framework_plugin::ViewModel::default())` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1591`) — a bare default `ViewModel` with **no `window_id`**. `window_measures_body`'s first line (`✏️editor/🦀️.rs:7176`) is `let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };` — no real host call asking "what are this window's measures" would omit the window it's asking about; the test never exercises a reachable state. |
| `vortex_show_window_option_defaults_to_selected_and_switches_to_always` | same shape, `✏️editor/🧪️tests/🔬️unit/🦀️.rs:1571` | same cause |

---

## 5. Bucket (c) — framework-side / cross-cutting, not puzzle3d's own logic

| test | evidence | note |
| --- | --- | --- |
| `set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin` | `setActiveExample work did not reach Complete within its own declared extent` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:640`) | **P4**, `📓️2026-09-09-wave-D-production-defects.md` §4: `os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` is a framework constant with no per-app knob; a retained op publishes one edit per turn and Nakagin needs ~182. |
| `two_instances_converge_disjoint_object_edits_via_backbone` | `Fault { origin: Module, code: "module.vcs", message: "validation failed: remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized" }` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:2691`) | Explicit fail-closed stub in the VCS/backbone remote-merge infrastructure, not a puzzle3d bug; present unchanged since `📓️2026-09-09-wave-X-test-suite.md` §7. |
| `local_interaction_query_return_does_not_fault_the_next_maintenance_step` | `local interaction query never produced a terminal page` after up to 2000 polling turns (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:37`) | Same failure recorded, unresolved, since W-X §7; the local-interaction-query pump's own completion path, not puzzle3d's dispatch. |
| `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls`, `set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched`, `window_transient_is_exact_instance_local_and_resets_on_reload`, `world_pick_locked_object_clears_like_background` | `assertion left != right failed: retained operation faulted: job-session.terminal-fault` — opaque fault body, no detail | Matches wave D's own diagnosis of the same code (§3): `job-session.terminal-fault` is the framework's pre-admitted page for a **step-budget quarantine** (`interactive_step_contract_violated`, `INTERACTIVE_STEP_CEILING_US`) under an unoptimized debug build. Demonstrated flaky here: which tests hit it changed between the two runs in §0 (`close_vortex_suggestions_clears_sticky_hover` hit it in run 1 only; these four in run 2 only). Not deterministic, not attributable to one commit. |
| `document_and_kinds_trees_use_german_reuse_section_labels` | `assert!(document_json.contains("Baukomponenten"), "document tree objects section")` fails (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1197`) | Same terminology/locale-resolution area W-G2 owns (`📓️2026-09-09-wave-G2-locale-and-tree-memo.md`); not isolated further here — could be genuine (locale never resolves to `de` in this render path) or another instance of §4's bare-`ViewModel` test omission; left to W-G2's owner. |

## 6. Bucket (d) — flips between the two runs in §0 (not in the run-2 count of 44)

`add_object_kind_honors_drop_origin`, `close_vortex_suggestions_clears_sticky_hover`,
`world_vortex_click_replaces_until_invertive_mode_is_selected` (red in run 1, green in run 2);
`window_transient_is_exact_instance_local_and_resets_on_reload`, `world_pick_locked_object_clears_like_background`
(green in run 1, red in run 2 — counted under bucket (c)'s `job-session.terminal-fault` row above, since
that is what they failed with). `world_vortex_click_replaces_until_invertive_mode_is_selected`'s run-1
failure was `interactive-job.worker-pump: framework reserved mounted worker transition was rejected` —
the same pool-contention class wave D recorded as **P10, framework, and flaky** (§4 of
`📓️2026-09-09-wave-D-production-defects.md`).

---

## 7. Table

| bucket | count |
| --- | ---: |
| (a) real production defect | 32 |
| (b) stale test assumption | 4 |
| (c) framework-side defect | 8 |
| (d) peer/flaky churn (excluded from the 44) | 5 |
| **run-2 total** | **44** |

### Ordered (a) items by user impact

1. WindowConfig-lane publication hang — camera/projection/grid/vortex-show/vortex-direction/window
   options — **app freezes**, 6 tests, fix near `✏️editor/🦀️.rs:6258` + the generic
   `advance()` at `🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🦀️.rs:519`.
2. `setActiveTool`/`setActiveUtility` dead (P2 undone) — fill tool and every utility unusable, 17 tests,
   fix at `✏️editor/🦀️.rs:6776` (`host_configuration_mutation`), needs the `Puzzle3dRuntime` lane
   decision `📓️2026-09-09-wave-D-production-defects.md` §8 already asked for.
3. `addObjectKind` no-op on an empty document — can't build up a document from scratch, 5 tests, fix at
   `✏️editor/🦀️.rs:3596-3600`.
4. Vortex suggestion popup state dropped before render — suggestions feature invisible, 4 tests, fix
   near `✏️editor/🦀️.rs:~2918-2920` (window-transient diff) or
   `✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs:20`.

## 8. Not verified

1. No runtime/browser confirmation — every verdict above is a `cargo test` run or direct source reading.
2. §3.1's exact stuck step inside the generic `WindowConfig` `advance()` cycle was not isolated (would
   need a temporary probe on `🧰️framework/…/🪟️window/🎚️config/🦀️.rs:519`, per W-P3's own method).
3. §3.4's exact drop point (window-transient diff vs. something upstream of it) was not isolated to a
   single line.
4. The crate was under live edit throughout this audit (§0); a follow-up run after that wave lands may
   show different numbers. Numbers here are anchored to run 2, `finished in 257.65s`,
   `test result: FAILED. 89 passed; 44 failed; 0 ignored; 0 measured; 451 filtered out`.
5. `precompute::component::tests` and `precompute::fill::tests` were not re-run this audit — W-P3
   recorded them at 132 passed / 0 failed and nothing in this audit's diff touches `⏳️precompute` in a
   way that would be expected to move that number, but it was not re-verified.
