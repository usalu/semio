# Wave B7 — Undo/redo regression in the puzzle 3d guest laws

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Implementation wave. No git write, no worktree, no wasm build, no
browser battery, nothing killed. Every command below ran in the FOREGROUND and its tail is quoted verbatim.

**Verdict in one line: the production undo route is fine. The puzzle3d FIXTURE stopped playing the host's
half of the framework-reserved spawn route that landed in HEAD `46c3cb9de0`, so `undo`, `redo`,
`interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`,
`setInteractionGranularity`, `setActiveTool` and `setActiveUtility` were all admitted and never run.
One 8-line fix in the testkit took the crate from 638 passed / 35 failed to 668 passed / 5 failed.**

---

## 1. Repro (before the fix)

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib undo -- --test-threads=1`

```
running 5 tests
test editor::puzzle3d::component::tests::cut_undoes_as_one_step ... FAILED
test editor::puzzle3d::component::tests::relocate_target_volume_undoes_and_redoes_as_one_mutation ... FAILED
test editor::puzzle3d::component::tests::set_active_example_swaps_the_document_and_undo_restores_it ... FAILED
test editor::puzzle3d::component::tests::world_relocate_undoes_and_redoes_as_one_mutation ... FAILED
test editor::puzzle3d::panels::document::tests::an_outliner_flag_row_undoes_itself_on_the_second_click ... ok

---- …cut_undoes_as_one_step stdout ----
…🧪️tests/🔬️unit/🦀️.rs:4389:5:
assertion `left == right` failed: cut removes the selection
  left: 1
 right: 0

---- …set_active_example_swaps_the_document_and_undo_restores_it stdout ----
…🧪️tests/🔬️unit/🦀️.rs:1032:5:
assertion `left == right` failed: undo restores the concrete-forest objects
  left: 0
 right: 1

test result: FAILED. 1 passed; 4 failed; 0 ignored; 0 measured; 668 filtered out; finished in 2.92s
```

**B5's report is slightly off on one of them and it matters.** `cut_undoes_as_one_step` does NOT fail on its
undo assertion — it fails two lines earlier, on `cut removes the selection`. That is the tell: `cut` itself is
a clipboard route that still commits inline, but the `select_id(...)` that precedes it dispatches
`interactionSelect`, which is a framework-reserved verb — so the selection never landed and `cut` cut nothing.
The bug is not in the history lane at all; it is in the whole framework-reserved verb family.

To size it, the FULL suite was run before touching anything
(`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1`):

```
test result: FAILED. 638 passed; 35 failed; 0 ignored; 0 measured; 0 filtered out; finished in 114.25s
```

and the failure list reads like a list of every law that needs a selection or an activation —
`world_pick_object_replaces_vortex_selection`, `world_pick_null_clears_without_reselecting_first_object`,
`world_vortex_select_clears_object_selection`, `world_vortices_carry_their_own_selected_and_hovered_flags`,
`select_same_kind_widens_the_selection_to_every_object_of_that_kind`, `selected_object_inspector_renders_that_object_field_group`,
`selected_vortex_inspector_renders_the_vortex_field_group`, `gumball_active_only_for_transform_utilities_with_object_selection`,
all four `gumball_*` coalesce laws, `duplicate_selection_reselects_the_created_clones`,
`copy_then_paste_clones_selection_as_one_mutation`, `inspection_flag_rows_toggle_back_off`,
`inspector_field_actions_resolve_selection_without_embedding_ids`,
`set_active_example_lands_as_one_edit_and_republishes_the_world_scene`, `import_fixture_reproduces_the_exported_document`,
plus the four undo laws. One cause, thirty symptoms.

---

## 2. Attribution — it is HEAD `46c3cb9de0`, not a peer's uncommitted hunk and not a wave's

### 2.1 Not the working tree

The failing route never reaches a single uncommitted hunk:

| suspect (uncommitted) | file | verdict |
|---|---|---|
| UNKNOWN PEER: removal of the six `_ if failed => … publication is retiring a rejected authority` guards | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` @ the `PendingArtifactStorePublication::{Artifact,Config,Draft,Presence,Transient,WindowConfig,WindowTransient}` close ladders (diff hunks at `-23090,7`…`-23173,14`) | **NOT the cause.** That ladder is inside `advance_typed_operation_publication`, i.e. the TYPED operation lane. `undo` never enters it — `commit_framework_history_route` writes through `self.store.dispatch(ArtifactCommand::Undo)` directly and answers `Self::empty_result(...)`. Proven green with those hunks in the tree: all 9 framework `reserved_undo` laws pass (§4.2). |
| UNKNOWN PEER: `mint_extension_invocations` moved from the head of `ArtifactToolCompletionValue::Emit(Ok(emit), …)` into the `else` arm (diff hunk at `-23267,12 +23329,6` / `+23455,9`) | same file | **NOT the cause.** Same lane as above; the history route emits no `Emit`. |
| UNKNOWN PEER: `window_transient_authority.as_mut()` + `self.window_transient_store.refresh(authority)?` | same file + `🪟️window/🫧️transient/🦀️.rs` | **NOT the cause.** Window-transient publication only. |
| B0: `admit_typed_operation_slot` / `allocate_operation_id_in_slot` / `slot_is_vacant` | same file | **NOT the cause.** Pre-admission of the TYPED operation slot. The reserved route mints its id through `admit_framework_reserved_spawn` → plain `semio_framework_job::allocate_operation_id()`, untouched by B0. |
| B2: `⚛️reactor/🩹️patches`, `⚛️reactor/🔄️turn`, `RetiredTableRowsView::close_step(items, bytes)` | same file + reactor | **NOT the cause.** Retirement pricing only. |
| B3: `⏳️precompute/📐️geometry` | guest | **NOT the cause** for undo (it does own 2 of the 5 residual failures, §4.4). |
| B5/B6: guest editor / panels | guest | **NOT the cause.** B5's own `world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice` passes; its hunks do not touch dispatch. |

`git diff HEAD -- …/🧪️tests/🔬️testkit/🦀️.rs` was **empty** before this wave: the fixture is exactly HEAD's.

### 2.2 It is HEAD's own commit

`git log --date=iso` (memory: the commit *message* date is frozen and fake, the author date is not):

```
46c3cb9de0 2026-09-11 12:39:02 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️609     ← HEAD
f39d4b0db3 2026-09-10 13:54:54 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️608
```

`git diff f39d4b0db3 46c3cb9de0 -- 🧰️framework/…/🔌️plugin/🦀️.rs` introduces, in that one commit:

- `initialize_framework_reserved_jobs()`, `FRAMEWORK_RESERVED_JOB_KIND`, `encode_framework_reserved_job_input`,
  `framework_reserved_job_factory` (plugin `🦀️.rs` **~15988**);
- `admit_framework_reserved_spawn` (**22300**) and `complete_reserved_spawned_job_inner` (**22321**);
- the rewrite of `dispatch_framework_reserved_action` (**22262**) so that every **non-clipboard** reserved verb
  now returns an ADMISSION instead of a result:

```rust
// 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22283-22297
initialize_framework_reserved_jobs();
let permit = self.admit_framework_reserved_spawn(action, &raw, decoded_items, meta).await?;
let job = permit.operation.operation.0;
…
self.pending_reserved.insert_admitted(job, PendingFrameworkReserved { … });
let mut admitted = Self::empty_result(action, meta, vec![Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), … placement: JobPlacement::Isolated }], Vec::new(), UiDirtyScope::None).await;
```

- and the host-half driver `settle_framework_reserved_admission` (**24289**), which `start_job`s that spawn,
  `step_job`s it to a terminal step and calls `complete_reserved_spawned_job_inner` → `commit_framework_history_route`.

The same commit added `settle_reserved` / `dispatch_reserved_unsettled` / `select_id_unsettled` /
`hover_id_unsettled` to the puzzle3d testkit (`git diff f39d4b0db3 46c3cb9de0 -- …/🔬️testkit/🦀️.rs`, `+514,31`)
**but never wired `settle_reserved` into `dispatch` itself**. So the fixture kept doing:

```rust
let reserved = app.handle_action(action, dsl_args.as_ref(), &action_meta).await;
return settle_into(app, reserved).await;
```

`settle_into` → `settle` loops on `has_pending_typed_operations` (plugin `🦀️.rs:25377`), which counts
`tool_operations`, `latest_wins_commands` and the four typed outboxes — and deliberately **not**
`pending_reserved`. Nothing in the fixture ever drove the spawned job, so `dispatch("undo")` returned `Ok`
with a `SpawnJob` effect and the store never moved. Exactly B5's "succeeds and the document is unchanged".

This also explains why the wave-Y table on 2026-09-10 (`📓️2026-09-10-wave-Y-feature-completion.md` §
`cut_undoes_as_one_step | ok`) was honest — it ran against `f39d4b0db3`, before the route split — and why the
**browser** undo battery still passes on the 14:30 wasm: the real host DOES drive the spawn (that is
`plugin_complete_reserved_spawned_job` / `driveSpawnedJob`), and the framework's own `reserved_undo` laws
cover it (§4.2). Only the in-process fixture was left behind.

---

## 3. The fix

One file, one call site, plus a docstring. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`, in `dispatch`'s reserved arm (now **:486-492**):

```rust
let dsl_args = args.map(json::to_dsl_value);
let admitted = app.handle_action(action, dsl_args.as_ref(), &action_meta).await;
let reserved = match admitted {
    Ok(admitted) => settle_reserved(app, admitted).await,
    Err(fault) => Err(fault),
};
return settle_into(app, reserved).await;
```

Why this and not something else:

- **It restores the intent of both halves instead of reverting anyone's code.** The admission half stays an
  admission (the browser contract, pinned by `reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route`
  and `reserved_undo_reaches_done_within_host_drive_contract`); the fixture simply plays the host it was always
  supposed to play, using HEAD's own `settle_framework_reserved_admission`.
- **It is a no-op for the routes that did not change.** `settle_framework_reserved_admission` returns the
  admission unchanged when there is no `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }` — so `copy`,
  `cut` and `paste`, which still commit inline through `CLIPBOARD_ACTION_IDS`, pass straight through.
- **It leaves the deliberately-unsettled helpers alone.** `dispatch_reserved_unsettled` / `select_id_unsettled` /
  `hover_id_unsettled` still stop at the admission, because the hover-storm laws at `🔬️unit/🦀️.rs:1700-1780`
  need exactly that.

`settle_reserved` is defined below `dispatch` in the same module; no import moved.

---

## 4. Laws and quoted outputs

### 4.1 New law

`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1037` — `undo_admits_one_reserved_job_that_alone_restores_the_document`.
It pins **both** halves of the failure mode, so neither can go silent again:

- the ADMISSION half must emit exactly **one** Isolated `FRAMEWORK_RESERVED_JOB_KIND` spawn job, carry
  `UiDirtyScope::None`, and leave `object_count` at `0` — a route that quietly commits nothing while reporting
  success fails here;
- the HOST half (`settle_reserved` → `complete_reserved_spawned_job` → `commit_framework_history_route`) must be
  the thing that restores the document and must answer `UiDirtyScope::Full` — a driver that stops committing
  fails here.

It uses `dispatch_reserved_unsettled` on purpose: it is the only law in the crate that asserts the admission is
*inert*, which is precisely the property whose absence made every other undo law degrade into a passing no-op.

### 4.2 Framework host half — unchanged and green WITH the peers' hunks in the tree

`RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib reserved_undo -- --test-threads=1`

```
test …reserved_undo_actor_ingress_admits_undeclared_window_kind ... ok
test …reserved_undo_browser_note_without_inverse_pops_chrome_resize ... ok
test …reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route ... ok
test …reserved_undo_host_json_export_admits_isolated_spawn_job ... ok
test …reserved_undo_invocation_does_not_require_window_ownership ... ok
test …reserved_undo_pops_chrome_top_shell_then_publishes_history_patch ... ok
test …reserved_undo_pops_shell_then_falls_through_to_document_store ... ok
test …reserved_undo_reaches_done_within_host_drive_contract ... ok
test …reserved_undo_replay_shell_command_survives_wire_roundtrip ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 643 filtered out; finished in 0.05s
```

**`RUST_MIN_STACK` is mandatory here too** — without it `reserved_undo_host_json_export_admits_isolated_spawn_job`
dies with `has overflowed its stack / fatal runtime error: stack overflow, aborting` (SIGABRT), same family as
`📓️2026-09-10-order-dependent-tests-audit.md` family 3. A bare `cargo test -p semio-framework-plugin --lib reserved_undo`
is the broken invocation, not the code.

### 4.3 Guest undo laws — all green, including the new one

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib undo -- --test-threads=1`

```
running 6 tests
test editor::puzzle3d::component::tests::cut_undoes_as_one_step ... ok
test editor::puzzle3d::component::tests::relocate_target_volume_undoes_and_redoes_as_one_mutation ... ok
test editor::puzzle3d::component::tests::set_active_example_swaps_the_document_and_undo_restores_it ... ok
test editor::puzzle3d::component::tests::undo_admits_one_reserved_job_that_alone_restores_the_document ... ok
test editor::puzzle3d::component::tests::world_relocate_undoes_and_redoes_as_one_mutation ... ok
test editor::puzzle3d::panels::document::tests::an_outliner_flag_row_undoes_itself_on_the_second_click ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 668 filtered out; finished in 0.49s
```

The two filters B5 named individually:

```
test editor::puzzle3d::component::tests::cut_undoes_as_one_step ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 674 filtered out; finished in 0.15s
=====
test editor::puzzle3d::component::tests::set_active_example_swaps_the_document_and_undo_restores_it ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 674 filtered out; finished in 0.13s
```

### 4.4 Full guest suite — 638/35 → 668/5

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1`

```
failures:
    editor::puzzle3d::component::tests::open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin
    editor::puzzle3d::component::tests::two_instances_converge_disjoint_object_edits_via_backbone
    editor::puzzle3d::precompute::fill::tests::nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling
    editor::puzzle3d::precompute::geometry::tests::an_owner_whose_middle_sub_page_was_refused_keeps_the_earlier_ones_and_reports_honest_capacity
    editor::puzzle3d::precompute::geometry::tests::every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling

test result: FAILED. 668 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 89.20s
```

**30 laws recovered, zero new failures.** All five residuals were already in the 35-failure BEFORE list, none is
in this wave's territory:

| residual | owner |
|---|---|
| `every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling` (`442368 > 65536` contiguous bytes) | **B3** — live, uncommitted `⏳️precompute/📐️geometry/🦀️.rs` (+411 lines). Matches the known guest contiguous-request ceiling. |
| `an_owner_whose_middle_sub_page_was_refused_keeps_the_earlier_ones_and_reports_honest_capacity` | **B3**, same file. |
| `nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling` (`faulted at stage PrepareFixture after 2 turns`) | fill lane (`⏳️precompute/🪣️fill`), also mid-edit. |
| `two_instances_converge_disjoint_object_edits_via_backbone` (`remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized`) | pre-existing framework fail-closed, unrelated to undo. |
| `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` | a perf ceiling, pre-existing. |

### 4.5 Type check

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`

```
warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings (run `cargo fix --lib -p semio-s-artifact-puzzle-3d` to apply 85 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.49s
```

`grep -c '^error'` over the same run: **0**. The 88 warnings prove the crate actually expanded and type-checked.

---

## 5. For the coordinator

- **The browser `--undo` battery lane does NOT need to be re-run because of this.** The production path was never
  broken: the host half is `plugin_complete_reserved_spawned_job` / `driveSpawnedJob`, it is covered by the nine
  framework `reserved_undo` laws (all green, §4.2, *with* the unknown peer's publication-guard removal and
  `mint_extension_invocations` move already in the tree), and the 14:30 wasm (#44-pre, HEAD `46c3cb9`) already
  passed it. Nothing in this wave touches shipped code — the only edits are a test fixture and a new law.
- **No peer hunk needs reverting or reconciling.** §2.1 clears every one of them explicitly, by route rather than
  by guess.
- **Re-run the `--undo` battery only if you want the *leftover* route re-proven** after B2's reactor-turn and
  patch changes land, since that lane (leftover `Invocation` with `in_reply_to: 0`, host reading history from
  leftover send-messages, `📓️2026-09-10-fill-build-host-tick.md` §8.18-8.20) rides the reactor turn this wave did
  not measure.
- **Warn every other wave:** any law that needs a selection, a hover, an activation or a history step MUST go
  through `testkit::dispatch` (or explicitly through `dispatch_reserved_unsettled` + `settle_reserved`). Before
  this fix, thirty such laws were passing their dispatch and asserting on a document that had never moved. If a
  wave's new law "mysteriously" sees an empty selection, this was why.
- **Residual owners:** the two `📐️geometry` sub-page laws and the Nakagin fill law are live B3/fill territory and
  were red before this wave too; `two_instances_converge_disjoint_object_edits_via_backbone` and
  `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` still need an owner.

## 6. Files touched by this wave

| file | change |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `dispatch` drives the framework-reserved spawn job through `settle_reserved`; docstring states the two-half contract |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | new law `undo_admits_one_reserved_job_that_alone_restores_the_document` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-11-wave-B7-undo-regression.md` | this report |

No temporary `[DEBUG]` logs were added; none remain from this wave. No `🗑️generated/` output was produced or
deleted. The ticket was not opened, closed or reopened.
