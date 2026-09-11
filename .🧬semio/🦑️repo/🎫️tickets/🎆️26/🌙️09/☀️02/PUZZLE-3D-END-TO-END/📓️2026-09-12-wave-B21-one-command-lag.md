# Wave B21 — the one-command lag is `take_typed_operation_completion` reading the dirty set BEFORE the backfill

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B21, 2026-09-12. Assignment: repair the `KeyedTestApp`
`compositeEdit` fixture so B19's law goes red on the lag, fix the lag, make B16's two laws non-vacuous, and
pin it on the puzzle 3d guest. No git write, ticket not opened/closed, `🗑️generated` written to and never
deleted, **no wasm build run** (the coordinator owns #48), every command foreground, every `[DEBUG] b21` tap
removed before finishing (`rg -a "\[DEBUG\] b21"` over all four touched files → no match).

**Headline.** The lag is one line, and it is not scheduling at all. A typed operation **never calls
`record_command`** — its edit reaches the command log exclusively through `backfill_command_log`, which runs
inside `refresh_cache`, which `history_patch` runs FIRST. `take_typed_operation_completion` gated on
`history_dirty_sequences.is_empty()` **before** that backfill, so the terminal completion of the very call
whose publication had just landed answered "nothing dirty" and carried `history_patch: None`. The row then
surfaced on whatever later call happened to refresh the cache — the next command. B16's scheduling work was
never wrong; it just could not have closed this, and its laws could not see it because the fixture they drive
was refused before it published anything.

Three defects were found and fixed, each proven red-then-green:

| # | defect | site |
| --- | --- | --- |
| 1 | **the one-command lag** — the completion's history-patch guard reads the dirty set before the backfill that fills it | `🔌️plugin/🦀️.rs:26168` `VcsArtifactApp::take_typed_operation_completion` |
| 2 | **a latest-wins rebase starves itself and the key registry** — it retired a several-hundred-character key one character per host continuation and rebuilt it, and drove the shared key registry only when the registry happened to be busy with ITS OWN operation | `🔌️plugin/🦀️.rs:23550` `advance_latest_wins_admission_unit` |
| 3 | **six puzzle3d import laws were green ON the lag** — they asserted a MUTATING verb's command-log delta on `InvocationResult::history_patch`, a channel that only ever held it by accident (what they read was the PREVIOUS command's un-delivered row) | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` |

---

## 1 Fixture repair — `compositeEdit` now reaches `Terminal`

`TestCountOneItemPreparationFactory::preflight`
(`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:167`) refused every mutation carrying a
`description`:

```rust
if !matches!(mutation, TestMutation::SetCount(..)) || description.is_some() || lane != HistoryLane::Document
```

and `KeyedTestJob::step` emits `description: Some("retained composite edit")`. **No real app's preflight looks
at the description** — `✏️s/🔌️plugins/{🌿️vcs,🏗️fem,🕸️dag,🖨️raster}/…/✏️editor/🦀️.rs` all ignore it, because
a description is an edit LABEL, not a mutation shape. The clause was dropped (`_description`), and the
fixture's prepared `Edit` now carries `request.description.clone()` instead of a hard-coded `None`, so the
ladder publishes a real described edit the way puzzle3d does. `rg` over the repo: nothing asserts either
string, and only `TestApp::<true>` and `KeyedTestApp` install this factory — every `description: Some(..)`
dispatch in that file is on `TestApp::<false>`, which installs none.

Before (B19's reading, reproduced verbatim at HEAD):

```
[DEBUG] actual the admitting call handed back the Fault lane ("test count accepts exactly one scalar
mutation") and 0 completion witness(es) within 32 continuations
```

After the repair, still green, but now over a ladder that PUBLISHES:

```
[DEBUG] actual the admitting call handed back the Terminal lane ("typed-operation-complete") and 1
completion witness(es) within 50 continuations
```

**So the terminal lane and its completion witness were never late.** B19's law as written could not go red,
because the thing that is late is what the witness CARRIES.

## 2 The law that does go red — the witness must carry the row

`an_admitting_host_call_hands_back_the_terminal_lane_it_earned` grew its second half:
`HostCallCensus` now counts `history_patches` beside `completions`
(`drive_host_call_with_deferred_acks`, `🔌️plugin/🦀️.rs:17520`), and a `Terminal` page owes exactly one of
each — because the completion's `history_patch` is what the renderer turns into the `OperationCompleted`
frame's history patch, i.e. the `create-object` row (`🔌️PluginRuntime/🟦️.tsx`
`subscribeOperationCompletions`).

**Red at HEAD, with the fixture repaired and nothing else changed:**

```
assertion `left == right` failed: the admitting call of 58 continuations handed back a Terminal page
("typed-operation-complete") and 1 completion witness(es) but 0 of them carried a history patch; the
`create-object` row is what the renderer turns into the document frame, so a Terminal completion owes
exactly one; it left 0 of 64 typed-operation slots live after 58 host continuations []
  left: 0
 right: 1
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 661 filtered out; finished in 0.06s
```

That is B19 §3's browser line — `historyUpserts:0`, `create-object` inside the NEXT command's settle —
reproduced natively, with deferred ACKs, no browser and no wasm build.

## 3 The lag mechanism, file:line

| step | site | what happens |
| --- | --- | --- |
| the edit is staged and published | `🔌️plugin/🦀️.rs:23928` `publish_mounted_typed_operation_unit` → `store.begin_apply_batch(…, HistoryLane::Document, …)` | one batched `Edit`, one undo step. On `ArtifactStoreOneItemAdvance::Published(receipt)` (`:23832`) it updates generations and queues an `Artifact` lane page — and **calls no `record_command`**. `record_command`'s 14 call sites (`:21554`, `:21598`, `:21852`, …) are all on the `dispatch_emit`/reserved/config paths; the typed-operation ladder is on none of them. |
| the row reaches the log | `:20986` `backfill_command_log` | "Appends a command-log entry for every VCS edit not yet referenced by the log". It is the ONLY thing that logs a typed operation's edit, and it runs exclusively inside `refresh_cache` (`:21141`). |
| the patch is minted | `:21105` `history_patch` | `self.refresh_cache().await?` **first**, then `std::mem::take(&mut self.history_dirty_sequences)`. |
| **the guard** | `:26170` `take_typed_operation_completion` | `let history_patch = if self.history_dirty_sequences.is_empty() { None } else { … }` — evaluated **before** the backfill that would have filled it. On the admitting call the set is empty, so `history_patch: None`. |

The row therefore waits for the next caller that refreshes the cache for its own reasons — `dispatch_typed`'s
own `refresh_cache` before command scheduling, or a history-panel render — which bumps `log_generation`, and
is then swept up by the FOLLOWING command's `finish_recorded` or terminal completion. **One command late,
exactly.** It also explains the browser's row TEXT: `create-object object { id=… }` is
`backfill_command_log`'s `edit.forwards.first().print_op()` label under `action_id: "apply"`, not a verb row —
see §7.

### The fix

```rust
async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault> {
    let Some(witness) = self.typed_completion_outbox.pop() else { return Ok(None) };
    self.refresh_cache().await?;
    let history_patch = if self.history_dirty_sequences.is_empty() { None } else { Some(self.history_patch(false).await?) };
```

One line, with the docstring that states the contract: **the dirty set is only authoritative after the
backfill.** It costs nothing when nothing changed — `refresh_cache` early-returns on its
`(store generation, config generation, log generation, filter)` key — and when something did change, the
patch was owed anyway. Nothing else moved: no scheduling constant, no fold, no ACK protocol, B8's single
retirement site and B0's admission untouched.

**Green:**

```
[DEBUG] actual the admitting call handed back the Terminal lane ("typed-operation-complete"), 1 completion
witness(es) and 1 history patch(es) within 54 continuations
test …::an_admitting_host_call_hands_back_the_terminal_lane_it_earned ... ok
```

## 4 B16's two laws are no longer vacuous

New shared guard `assert_landing_terminal_lane` (`🔌️plugin/🦀️.rs:17570`), called by both:

> "No continuation was parked" and "nothing is left pending" are satisfied TRIVIALLY by a ladder the app
> refused before it published anything — wave B19 found both of wave B16's laws green over exactly that. A
> law about scheduling a LANDING mutation must therefore state which lane its ladder earned.

`a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work` asserts the admitting call's
terminal lane is `Terminal`; `a_status_only_host_call_finishes_every_typed_operation_it_admitted` asserts it
across its status-only call and its drain call (`terminal.or(drain.terminal)`). Both fail loudly on a `Fault`
lane and on no terminal lane at all. Both green over the repaired fixture.

`released_slot_census` was also widened: it printed only MOUNTED operations, so the three other authorities
`typed_operation_slot_is_vacant` folds (a bare reservation, a pending latest-wins command, a segmented
download/closure) showed up as an empty list — which is precisely the shape §5's leak had.

## 5 Second defect the repaired fixture exposed — the latest-wins rebase starves itself

With `compositeEdit` publishing for real, B8's
`a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot` went **red**:

```
assertion `left == right` failed: a latest-wins replacement left 1 of 64 typed-operation slots live after
512 host continuations ["2:reserved 130+latest-wins 130"]
```

Instrumented (taps added, run, removed): op 130 sat in `restarting=true` for 165 of its units with
`active_operation = Some(129)` — a sibling that had been accepted and started, leaving its closing update
behind in `ToolLatestWinsRegistry`. Two compounding faults in `advance_latest_wins_admission_unit`
(`🔌️plugin/🦀️.rs:23550`):

1. **The restart discarded the KEY.** The key is `(instance, envelope id, controller id, tool id, target)` —
   not one of which depends on the document revision or generation a rebase re-bases. The branch nonetheless
   drained the in-flight `ToolLatestWinsKeyCopy` through `close_key_step`, which pops **one character per host
   continuation**, then rebuilt it from scratch: several hundred round trips per rebase, and a rebase fires
   every time a sibling publishes.
2. **It drove the shared registry only when the registry was busy with ITS OWN operation**
   (`if self.latest_wins_keys.active_operation == Some(operation)`). `active_operation` is cleared solely by
   `ToolLatestWinsRegistry::advance`, so a front-of-FIFO command that is restarting waited forever on a
   registry no one would advance. The `key` branch three lines below already drives it whenever `begin`
   fails — the restart branch simply did not.

Both replaced by the predicate that branch actually means, `!self.latest_wins_keys.can_begin()`, keeping the
key:

```rust
if pending.restarting {
    self.latest_wins_keys.cancel(operation);
    self.latest_wins_keys.take_outcome(operation);
    if !self.latest_wins_keys.can_begin() {
        self.latest_wins_keys.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES);
        return Ok(false);
    }
    …rebind_keyed…
```

```
[DEBUG] actual settled, latest-wins-replaced and cancelled typed operations each released their exact slot
within 512 host continuations
test …::a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot ... ok
```

**Three laws B16 recorded as failing now pass** (§8's name diff): `retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document`,
`retained_operation_continues_after_command_admission_until_publication_and_retirement`,
`microsecond_registered_factory_dispatch_preserves_exact_half_ms_fake_clock`. This is the same rebase churn
B8 measured in the browser as 39 `interactive-job.typed-operation-capacity` refusals.

## 6 The puzzle 3d pin — red then green on the real guest

`a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1171`, region `🔖️Operations`). For `addObjectKind` and
`duplicateSelection`: `dispatch_unsettled` then ONE `settle` — no second dispatch — and the object must land
AND exactly one completion must carry a non-empty history patch.

**Red with the one line reverted and nothing else changed** — and the failure prints the defect in full: the
completion tells the shell to repaint `framework.body.history` while carrying no patch to fill it with.

```
assertion `left == right` failed: addObjectKind must hand back exactly one completion carrying its history
patch inside its own settle, got [Puzzle3dCompletion { operation: 192, ui_scope: Partial { window_bodies:
["puzzle3d.play.composite"], panel_bodies: ["puzzle.3d.play.inspector", "puzzle.3d.play.document",
"framework.body.history", "puzzle.3d.play.kinds"], utilities: false, tools: false, engagements: false,
measures: true, labels: false }, history_patch: false }]
  left: 0
 right: 1
```

**Green with the fix:**

```
test editor::puzzle3d::component::tests::a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 710 filtered out; finished in 0.93s
```

So the testkit DOES reproduce the lag — it always drained the completion lane, it just never asserted what
the witness carried. `Puzzle3dCompletion::history_patch` is now the real `Option<HistoryPatch>` instead of a
`bool`, and `testkit::history_rows(&Puzzle3dSettled)` counts the `historyUpserts` a browser client counts.

## 7 Third defect — six import laws were green ON the lag

`InvocationResult::history_patch` is the delta the **accepted invocation** recorded (`finish_recorded`,
`🔌️plugin/🦀️.rs:24467`, which early-returns unless `log_generation` moved during that dispatch). A typed
mutation's row is NOT on that channel — it is on the completion lane, exactly as the browser reads it. Six
puzzle3d import laws asserted `imported.history_patch.is_some()` and were green only because the row they
picked up was the PREVIOUS command's un-delivered one. With the lag closed they all went red; each now reads
`history_rows(&settled)` through the new `testkit::dispatch_reporting` (`dispatch` plus its settle census;
`dispatch` itself is unchanged and still returns just the invocation):

`import_fixture_reproduces_the_exported_document`, `open_import_fixture_requests_file_open_then_import_applies_payload`,
`import_fixture_of_the_live_document_records_whether_identical_content_is_an_edit` (`== 0`, the identity
no-op), `import_fixture_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations`,
`exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity`,
`leftover_import_fixture_replaces_live_document_with_distinct_two_object_json`.

A fold of the completion patch into `InvocationResult::history_patch` was tried first and **rejected**: it
made `fill_build_tick_is_ignored_when_fill_tool_is_inactive` red, because a View verb's completion legitimately
sweeps up rows belonging to earlier verbs, and attributing those to the verb being dispatched is a false
positive. The two channels stay separate; only the laws moved.

**Residual, not this wave's:** a typed operation's row is still the backfill's anonymous
`action_id: "apply"` with an `edit.forwards.first().print_op()` label, not a verb row with its declared
`ActionKind` and label. That is why the browser's history reads `create-object object { id=… }` instead of
"Add Object". Fixing it means recording the command at `Published(receipt)` — a `record_command` from a
**sync** publication unit, plus the undo/inverse semantics of an edit-linked row, which is a wave of its own.

## 8 Verification (foreground, tails quoted)

| command | tail |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 typed_operation an_admitting_host_call reserved_undo a_nakagin_scale_mixed_surface_turn never_parks status_only` | `…::a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns ... ok` — `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 646 filtered out; finished in 0.96s`. Includes B8's two slot laws, B0's pre-admission law, B16's two, B19's, B2's mixed-surface law and the whole `reserved_undo` set. |
| the same after the fixture repair, **before** the fix | `test result: FAILED. 0 passed; 1 failed` (§2's tail) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 microsecond_registered_factory_dispatch retained_latest_wins_registered_dispatch retained_operation_continues_after_command_admission` | `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 659 filtered out; finished in 0.95s` — all three were in B16's failure list |
| whole plugin crate (`--skip a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`, B8 §6's order-dependent flake) | `test result: FAILED. 517 passed; 144 failed; 0 ignored; 0 measured; 1 filtered out; finished in 28.54s` — `🗑️generated/b21-plugin-lib-wave.txt` |
| **failure-NAME diff vs B16's `wave-B16-names-wave3.txt`** | **regressions: (empty)**. B16-only (i.e. now green): the three latest-wins laws above. The remaining 144 are the peer refactor B0 §5 / B2 §4 / B8 §6 recorded, unchanged in membership. |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 a_mutating_verb_lands import_fixture exported_fixture paste duplicate delete add_object catalogue` | `test result: ok. 54 passed; 0 failed; 0 ignored; 0 measured; 657 filtered out; finished in 16.57s` |
| whole puzzle3d lib, **with** the wave | `test result: FAILED. 703 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 58.80s` — `🗑️generated/b21-puzzle3d-lib3.txt` |
| whole puzzle3d lib, **fix neutralised** (baseline) | `test result: FAILED. 697 passed; 14 failed` — `🗑️generated/b21-puzzle3d-lib-baseline.txt` |
| **failure-NAME diff, wave vs baseline** | wave-only: **none**. Baseline-only: this wave's new law and the six repaired import laws — the red-then-green proof for all seven. The 8 that remain are 3 pre-existing (B16 §5's `every_advertised_engagement_verb_is_implemented`, `open_vortex_suggestions_…ceiling`, `two_instances_converge_…backbone`) plus 5 in the panels/window region peers are editing RIGHT NOW (`📌️panels/⚙️settings/🦀️.rs`, `📌️panels/🔍️inspection/🦀️.rs`, `🪟️window/🦀️.rs`, `🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` all show unstaged modifications): `selected_object_inspector_renders_that_object_field_group`, `settings_panel_steppers_carry_their_value_…`, `the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`, `kinds_tree_object_drag_data_carries_object_kind_and_mesh_url`, `the_catalogue_pages_an_over_wide_kind_catalog_…`. **All five fail identically with the fix neutralised**, and four of the five pass in isolation. |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 89 warnings` / `Finished dev profile [unoptimized] target(s) in 2.40s` — **0 errors** |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0` / `Finished dev profile [unoptimized] target(s) in 24.24s` — **0 errors**, guest target |
| `cargo check -p semio-framework-plugin --lib` | `warning: semio-framework-plugin (lib) generated 6 warnings` / `Finished dev profile … in 5.14s` — **0 errors** |

No wasm build was run, no browser was opened, no probe was started.

## 9 Expected #48 verdict flips

The fix is guest Rust: nothing changes in the browser until `component-release` is rebuilt AND materialised.
Predictions, split by how directly each verdict reads the channel that moved.

**Confident — these read the history row, which is what the fix delivers:**

| verdict | at #47 | predicted on #48 |
| --- | --- | --- |
| late-battery `undo-unwind` | FAIL `example=Nakagin` | **PASS** — B19 §4 proved it is not broken, it is the backlog. Each mutation's own settle now finishes its command-log delta instead of leaving it for the next command, so the per-command settle no longer owes work to its successor and the `undo` presses stop queueing behind it. |
| `undo-redo` | vacuous PASS | **PASS, non-vacuously** |
| history-panel row after any mutation | one command late | **inside the mutation's own settle** |

**Expected to improve, but verify rather than assume — these read `before=`/`after=` counts from a panel the
completion's `UiDirtyScope` refreshes, and that scope was already arriving at #47 (§6's red output shows
`panel_bodies` naming the document/outliner while `history_patch: false`):**

| verdict | at #47 | predicted on #48 |
| --- | --- | --- |
| `catalogue-add-object-kind` | FAIL `before=1 after=1` | **PASS in an `--only=` run**; in a full battery it is still B19 §3.1's latency gate, now with one less round trip owed per command |
| `delete-selection` | FAIL `before=4 after=4` | as above |
| `clipboard` (paste half) | FAIL | **PASS for paste's own row.** Paste is framework-reserved (`handle_action` → `settle_reserved`, `finish_recorded`), so its row was never on the lagging channel — what changes is that it no longer inherits the previous command's undelivered row. The copy half is host clipboard plumbing this wave does not touch. |
| `gumball-scene-delta`, `relocate-pose-delta` | FAIL | **likely PASS** — same mutating ladder; if they stay red the remaining defect is the transform publication lane (B5/B13's region), not the completion |
| `duplicate-selection`, `volume-brush-add-target-volume` | PASS in `--only=` on #47 | **stay PASS**, and for their own reason now |

**Compare like with like** (B19 §8.2): a battery against a battery, an `--only=` run against the same
`--only=` run. And read B8's `[DEBUG] typed-operation slots instance=N live=L/64 peak=P` on #48 — §5's
rebase fix should show a visibly smaller peak, because a restarting latest-wins command no longer holds its
reservation for hundreds of continuations while it rebuilds a key it never needed to discard.

## 10 Remaining risks

- **`refresh_cache` now runs on every completion pop.** It early-returns on its own memoised key when nothing
  advanced, and when something did advance the patch was owed anyway, so the worst case is one
  `build_history_view` per landing mutation — the same cost `finish_recorded` already pays on every
  non-typed dispatch. `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` is green;
  `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` is red and was red
  before this wave (B16 §5).
- **The anonymous `apply` history row** (§7) is untouched and is the obvious next wave.
- **Five puzzle3d panel/catalogue reds are live peer churn**, not this wave's (§8). They will resolve
  themselves or belong to whoever owns `📌️panels/*` and `🪟️window`.
- `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` remains the plugin crate's
  order-dependent binary-killing flake (B8 §6), which is why the crate comparison uses `--skip`.

## 11 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `take_typed_operation_completion` refreshes the
  cache before reading the dirty set (the lag fix, with its docstring); `advance_latest_wins_admission_unit`'s
  restart branch keeps its key and drives the registry on `!can_begin()`; `HostCallCensus::history_patches`
  and `drive_host_call_with_deferred_acks`; `assert_landing_terminal_lane`; the terminal-lane assertions in
  B16's two law bodies; the history-patch assertion in B19's law body; `released_slot_census` widened to all
  five slot authorities.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — `TestCountOneItemPreparationFactory::preflight` no longer refuses a described mutation; the prepared
  `Edit` carries the request's description.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
  — `Puzzle3dCompletion::history_patch` is the real `Option<HistoryPatch>`; `settle_into_reporting`,
  `dispatch_reporting`, `history_rows`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  — `a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle`; the six import laws moved to
  the completion lane.
- `🗑️generated/` — `b21-plugin-lib-wave.txt`, `b21-names-wave.txt`, `b21-puzzle3d-lib.txt`,
  `b21-puzzle3d-lib2.txt`, `b21-puzzle3d-lib3.txt`, `b21-puzzle3d-lib-baseline.txt`.
