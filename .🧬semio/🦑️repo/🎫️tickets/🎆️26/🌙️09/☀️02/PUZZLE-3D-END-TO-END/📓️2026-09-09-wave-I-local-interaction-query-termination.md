# 🕹️ Wave I — Local Interaction Query Termination

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, 2026-09-09.

## 1. What the measured defect actually is

The browser trace

```
query=started=true page_sent=false closing=true cancelled=false failed=false terminal_sent=false
      inner=ready=false closing=true retiring=true length=0 terminal_page=true
```

is **not** an empty capture. `length=0` there is the terminal page's bytes *already retired* by
`LocalInteractionQuery::close_step` → `retire_page`, and `closing=true` with `cancelled=false` is
reachable **only** through `LocalInteractionLiveQuery::acknowledge` of a terminal page
(`🕹️interaction/📡️live/🦀️.rs:76-90`; `cancel_authorized`/`begin_close` both set `cancelled=true`).
So the host had already acknowledged the terminal page and the query was stuck **inside closing**,
i.e. inside the retirement of its three captured roots. The exact same state string is produced by
the native reproduction below (the `turn=64` trace line quoted in §5), which confirms the reading.

## 2. Root cause — a registry-capacity-bounded cursor inside a host continuation budget

Closing a read returned the three captured `SnapshotRead` leases and then **waited for the Store to
reclaim the returned registry slot**:

- `🕹️interaction/📖️capture/🦀️.rs` `CapturedRootRetirement::close_step` retained a
  `SnapshotReadReturn` witness and returned `Blocked` until `returned.terminal_is_empty()`.
- `🕹️interaction/🔐️authority/📖️inputs/🦀️.rs` `LocalInteractionInputReads::close_step` did the same
  for the document and config leases.

`SnapshotReadReturn::terminal_is_empty()` is `!registry.contains(index, generation)`
(`🏪️store/🦀️.rs:283-286`), which only becomes true once the Store's **one-slot-per-step cleanup
cursor** happens to land on that exact slot: `try_take_one_returned` (`🏪️store/🦀️.rs:206-232`)
probes exactly ONE of `SNAPSHOT_READ_LEASE_CAPACITY = 1_024` slots per call and advances
`cleanup_cursor` by one — a deliberate law, asserted by
`🏪️store/🧪️tests/🔬️unit/🦀️.rs:366` `dropped_snapshot_read_remains_observable_until_one_slot_per_step_cleanup_takes_its_guard`.

Those probes were driven **only** from the query's own driver:
`VcsArtifactApp::advance_local_interaction_query_one` rotated four stages (query unit, document
pump, config pump, interaction pump), and `advance_typed_operation_publication` called it on every
*other* turn — so **one cursor probe per store per 8 host continuations**. Reclaiming one lease
whose slot index is `k` ahead of the cursor therefore costs up to `8 × 1024 = 8192` continuations,
for each of three stores. The host's settle drain gives up at 4096 continuations, and every one of
those turns leaves the debug state string **byte-identical** — exactly what the browser printed at
turns 1, 2, 4, … 4096.

On a freshly booted native fixture the cursor and the lease indices are in lockstep (index 0,1,2 /
cursor 0), which is why the native reproduction terminated at 127 turns and the browser did not: a
boot shell issues and drops many document snapshot reads before the first `readLocalInteraction`,
so the lease indices march ahead of the rarely-driven cleanup cursor.

Two more defects in the same family made it worse and were fixed with it:

- `publish_local_interaction_query_reply` / `take_local_interaction_query_reply` **withheld the
  query's terminal `Closed` reply** while *any* of the three pumps still owned a disposer — pumps
  that unrelated maintenance fills, and that are not part of the query's own ownership witness
  (`owners_are_empty()` already is).
- `document_snapshot_read_returns` could be **filled by `maintenance_step`'s idle fast path**
  (`🔌️plugin/🦀️.rs`, the `pump.drive(...)` early return) while the only thing that could ever
  **drain** it was the query driver — so with no live query the disposer was stranded, and
  `retained_fields_terminal_is_empty` did not even check that pump.

## 3. Changes

### Query ownership stops at registry acceptance

(Line numbers are as of this write-up; the files are under concurrent peer edit.)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🔐️authority/📖️inputs/🦀️.rs:7-55` —
  `InputReadState` (`:7`) no longer retains `[Option<SnapshotReadReturn>; 2]`; `close_step` (`:31`)
  hands each read back with `SnapshotRead::return_to_registry()` and reports `Complete`;
  `terminal_is_empty()` (`:53`) is `closing && document.is_none() && config.is_none()`. Registry
  **acceptance** is the witness; reclaiming the accepted slot is Store maintenance.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs:101-135` —
  `CapturedRootRetirementState` (`:101`) drops its `returned` witness; `close_step` (`:115`) returns
  the lease and completes; `terminal_is_empty()` (`:133`) is `root.is_none()`. The `Arc::into_inner`
  miss path is unchanged (another alias then owns the `SnapshotRead`'s own `Drop` return).

### The returned-read pumps become cooperative maintenance, not query work

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18522` —
  `advance_local_interaction_query_one` is now only the live-query unit; new
  `advance_snapshot_read_returns_one` (`:18542`) owns the three-store rotation and new
  `snapshot_read_returns_terminal_is_empty` (`:18553`) is their joint witness. Field
  `local_interaction_query_stage` renamed to `snapshot_read_return_stage` (it is the pump rotation
  cursor now) and is no longer reset by `begin_local_interaction_query`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23584` `maintenance_step` — the idle fast
  path now goes through `advance_snapshot_read_returns_one`, which both **fills** an empty pump (one
  registry probe) and **drains** a non-empty one, and rotates over **all three** capture stores
  instead of only the document store; its `document_snapshot_read_returns.terminal_is_empty()`
  precondition is therefore gone. Deliberately NOT a top-of-function preemption: draining a whole
  document snapshot ahead of the 23-stage round robin would starve every other maintenance stage,
  and the idle path already both fills and drains.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23022` `close_step` — drains the pumps
  after the query slot is gone, so app close is exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22959` `retained_fields_terminal_is_empty`
  — uses `snapshot_read_returns_terminal_is_empty()`, which adds the previously unchecked document
  pump; `close_terminal_is_empty`'s now-redundant `document_snapshot_read_returns` clause is gone.

### The terminal reply is no longer hostage to unrelated pumps

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24144` `take_local_interaction_query_reply`
  and `:24153` `publish_local_interaction_query_reply` — the three-pump gate is deleted.
  `LocalInteractionLiveQuery::owners_are_empty()` is the exact witness that all three captured roots
  were handed back, and `take_reply_admitted` already requires it.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🦀️.rs` — `is_closing()`
  removed; it existed only for that gate.

### Same coupling elsewhere (NOT touched by this wave)

`SnapshotReadReturn` / `return_to_registry_witness` stay in the Store API and are still used by
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🧵️session/🦀️.rs:2833,3033`,
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/…/✏️editor/🧵️session/🦀️.rs:907,1760` and
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs:350,367`, each of which blocks its own close on
the same capacity-bounded cursor. Those are other plugins' lanes and were left alone; they carry the
same latent liveness bound.

`has_pending_work()` was left as it is: with a page published and awaiting ACK it is already
`false` (`page_sent=true` and the inner query's `ready=true`), which is the "only waiting on the
host" case; while closing there is real retirement work, and after the fix that work is O(1) in
turns instead of O(registry capacity).

## 4. Tests

### puzzle3d — the host loop, end to end

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:7-140`

New `read_local_interaction` / `read_local_interaction_while_rendering` play the HOST half exactly
as `plugin_continue_typed_operations` → `advance_typed_operation_output` does — one
`advance_typed_operation_publication` unit, then `publish_local_interaction_query_reply` into the
same four-frame admission, decoding each frame with `protocol::decode_app_frame` — and deliver a
page acknowledgement **only** at the `has_runnable_typed_operations() == false` quiescence point the
shell's `settlePluginTurn(drainOperations=true)` can actually reach.

- `local_interaction_read_of_an_unselected_document_terminates` — the empty-selection capture.
- `local_interaction_read_of_a_selected_document_terminates` — a live `object` selection.
- `local_interaction_read_terminates_under_concurrent_render_pressure` — renders every turn.
- `repeated_local_interaction_reads_of_one_instance_terminate` — three successive reads.
- `local_interaction_query_return_does_not_fault_the_next_maintenance_step` — rewritten onto the
  same driver. **It was broken before this wave**: it never acknowledged non-terminal pages, so it
  stalled at page 0 and failed with "local interaction query never produced a terminal page".

### framework — the state machine, with no Store reclamation at all

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🧪️tests/📡️live/🦀️.rs`

- `local_interaction_live_terminates_without_any_store_reclamation` (new) — drives the whole read
  with the pumps **never** run, ACKing only at `has_pending_work() == false`, and bounds the
  post-terminal-ACK closing cost at `CLOSE_TURN_BUDGET = 32` runnable turns regardless of the byte
  grant. This is the regression law for the defect.
- `local_interaction_live_empty_capture_reaches_its_terminal_closed_reply` (new) — Started, ONE
  terminal page of length 0, its exact ACK, retirement, `Closed`, terminal emptiness.
- `finish_close` no longer pumps the stores during the close loop, and the
  `snapshot_read_leases_terminal_is_empty` assertion moved into a new `reclaim_returned_leases`
  helper that runs **after** the terminal reply — the old placement encoded the very coupling this
  wave removes.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs`

- `EmptyCapture` / `empty_capture_for_live_law()` (new, `//#region 🫙️EmptyCapture`) — a capture that
  completes with zero bytes.
- `local_interaction_query_empty_capture_publishes_its_terminal_page_and_retires` (new) — at byte
  grants 1/64/4096 the fixed page authority publishes the empty terminal page, answers a second
  `advance` with `PageReady`, accepts exactly one ACK, retires, and reaches terminal emptiness with
  `completed_bytes() == retired_bytes()`.

Note on the `length=0` in the browser trace: the REAL `LocalInteractionCaptureCursor` never emits a
zero-length terminal page — `ArtifactCanonicalEditEncoder::next_byte`
(`🏪️store/🧵️canonical-edit/🧵️borrowed/🦀️.rs:214-219`) decrements `depth` **before** returning the
root object's closing `}`, so `is_complete()` is true on the page that carries that byte. The
zero-length terminal page is nevertheless a state the fixed page authority must answer, which is
what these two laws pin down.

## 5. Commands and results

Environment for every command: `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`,
run in the foreground.

### Reproduction, BEFORE the fix

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 local_interaction -- --test-threads=1`

```
running 1 test
test editor::puzzle3d::component::tests::local_interaction_query_return_does_not_fault_the_next_maintenance_step ... FAILED
[DEBUG] typed-operation publication turn=1024 operations=[] latest_wins_empty=true effects=0 events=0 ui=0
        query=started=true page_sent=true closing=false ... inner=ready=true closing=false retiring=false length=256 terminal_page=false
panicked at …🧪️tests/🔬️unit/🦀️.rs:37:32: local interaction query never produced a terminal page
```

(the pre-existing test's own defect — it never acknowledged intermediate pages)

With the new host-faithful driver, before the fix all reads still terminated natively but cost
**127 turns** each, and the trace showed the browser's exact stuck state string transiently:

```
[DEBUG] typed-operation publication turn=64 … query=started=true page_sent=false closing=true cancelled=false
        failed=false terminal_sent=false inner=ready=false closing=true retiring=true length=0 terminal_page=true
[DEBUG] local interaction read 1 turns=127 bytes=363
```

### AFTER the fix

`cargo check -p semio-framework-plugin` → clean (no warnings, no errors).

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`
→ `Finished \`dev\` profile [unoptimized] target(s) in 16.24s`, with one pre-existing peer warning
(`✏️editor/🦀️.rs:1375: function \`restored_precompute_session\` is never used`).

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 local_interaction -- --test-threads=1 --nocapture`

```
[DEBUG] local interaction read 1 turns=27 bytes=363
[DEBUG] local interaction read 2 turns=27 bytes=483
[DEBUG] local interaction read 4 turns=27 bytes=363
[DEBUG] local interaction read 1 turns=27 bytes=363
[DEBUG] local interaction read 2 turns=26 bytes=363
[DEBUG] local interaction read 3 turns=26 bytes=363
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 601 filtered out; finished in 1.72s
```

127 → 27 turns, and the close no longer contains any Store-reclamation wait at all.

`cargo test -p semio-framework-plugin --lib local_interaction -- --test-threads=1`

```
test result: FAILED. 34 passed; 1 failed; 0 ignored; 0 measured; 565 filtered out; finished in 8.68s
```

The 34 include the two new empty-capture laws and
`local_interaction_live_terminates_without_any_store_reclamation`. The one failure is
`plugin_builder_contract_tests::local_interaction_dispatch::local_interaction_registered_query_channel_continuation_ack_and_close`
— see §6.

`cargo test -p semio-framework-os-kernel --lib snapshot_read -- --test-threads=1`
→ `test result: ok. 8 passed; 0 failed; …` (the registry's one-slot-per-step law is untouched).

Both test commands were re-executed once more at the wave's FINAL source state (after the
`maintenance_step` preemption removal and the docstring corrections) with identical results —
puzzle3d `5 passed`, plugin `34 passed; 1 failed`. Between those two runs the puzzle3d test target
was transiently uncompilable for a few minutes on a peer's fill lane (`E0063: missing fields
\`fill_fault_notice\` and \`fill_faulted\` in initializer of \`Puzzle3dPrecomputeSession\`, at
`✏️editor/⏳️precompute/🦀️.rs:2173` and `✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:479`); the
peer landed the missing initializers and the rerun above is from after that.

## 6. Blockers, peer breakage, and not-verified

- **`local_interaction_registered_query_channel_continuation_ack_and_close` fails —
  NOT wave I.** `📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:466` asserts `presence.is_empty()` on
  every `AppFrame::Ephemeral`, and the FIRST ephemeral frame of the FIRST exchange now carries
  `presence = {"revision":0}` (probed by temporarily printing the payload, then reverting the
  probe). Evidence it is not this wave's: (a) it fails on the first exchange, before any changed
  path runs; (b) removing this wave's `maintenance_step` early return and rerunning reproduces it
  unchanged; (c) `git diff` over `🔌️plugin/` contains no `presence` hunk at all — the presence
  publication is at HEAD.
- **`cargo test -p semio-framework-plugin --lib` (whole suite) aborts** — unrelated peer breakage.
  `component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper` panics with
  `interactive-job.catalog-authority … generated_migrated=false` for tool `increment`, then
  double-panics in `🏪️store/🦀️.rs:2117` ("artifact store cursor disposer reached Drop before
  terminal-empty ownership"), which kills the binary and marks everything after it FAILED. Targeted
  filters (`local_interaction`, `snapshot_read`) are therefore the usable evidence.
- Earlier in the session the plugin **test target did not compile at all** — peer refactor of
  `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` (first `E0616 field window_transient_store is private`
  ×12 plus `E0624 method insert_admitted is private`, later `E0284 cannot infer the value of const
  parameter HAS_CHILD declared on the struct ComposedParentApp` ×10 at lines 189/192/195). The peer
  landed a fix mid-session and every framework law above was then executed.
- The browser stall itself was **not** reproduced in a browser (no wasm rebuild in this wave's
  scope). The native reproduction reproduces the state string and the mechanism; the
  4096-continuation timeout follows from the capacity-bounded cursor arithmetic in §2, not from a
  measured browser run after the fix.
- `interactionSelect` faulted with `interactive-job.worker-pump` ("framework reserved mounted worker
  transition was rejected") at one point during this wave — reproduced on the pre-existing
  `selected_object_inspector_renders_that_object_field_group` too, so not wave I's — and was fixed
  by a peer mid-session; the selected-document law passes now.
- `dispatch(app, "setActiveExample", …)` never quiesces through the testkit's `settle`: it loops to
  the 1 048 576-turn guard with `operations=[] latest_wins_empty=true effects=0 events=0 ui=0
  query=none`, i.e. the typed **completion** outbox is never drained by `settle`
  (`advance_typed_operation_output` drains it via `take_typed_operation_completion`, the testkit
  helper does not). That is W-A's completion-frame lane; a flagship-document variant of the read law
  is therefore not included.
## 7. Whole-crate puzzle3d suite

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 -- --test-threads=1`

```
test result: FAILED. 527 passed; 79 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1845.44s
```

Attribution of the 79 (none in wave I's lane; all five local-interaction laws pass):

- `📓️2026-09-09-remaining-test-failures-audit.md` records a **pre-existing 44-failure baseline in
  `editor::puzzle3d::component` alone**, before this wave, plus 5 tests that flip between runs from
  peer churn. That audit's scope excluded `editor::puzzle3d::precompute::component` and
  `retained_command`, which the whole-crate run adds.
- The `editor::puzzle3d::precompute::component::tests::fill_worker_*` block (the largest group) is a
  **poisoning cascade of one failure**: the first test panics at
  `✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:752`, and every later one dies at `:502:49`
  (`GUARD.…lock().expect("fill envelope test guard")`) on the poisoned static mutex. That is W-D's
  fill/precompute lane.
- The remaining named failures (`…german_reuse_section_labels`, `brush_placement_picker_…`,
  `fill_build_tick_…`, `accept_suggestion_…`) are the audit's own buckets (a)/(b) in the puzzle3d
  editor, i.e. W-D3/W-M2/W-D4 lanes.

Note on the run above: it was taken with an earlier shape of the `maintenance_step` change that
drained a non-empty returned-read pump at the TOP of the function, ahead of the 23-stage round
robin. That preemption was removed afterwards precisely because it can starve the round robin (a
whole document snapshot's retirement is many units); the idle fast path already both fills and
drains. Every targeted run in §5 was re-executed after the removal and is unchanged.

## 8. Post-fix `editor::puzzle3d::component` rerun

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4
editor::puzzle3d::component -- --test-threads=1`, run at this wave's final source state:

```
test result: FAILED. 87 passed; 57 failed; 0 ignored; 0 measured; 462 filtered out; finished in 1123.95s
```

`📓️2026-09-09-remaining-test-failures-audit.md`'s baseline was **89 passed / 44 failed of 133**;
the module is now **144 tests** (11 added since, 5 of them this wave's).

### Wave I's own laws in that run

`local_interaction_read_of_an_unselected_document_terminates`,
`local_interaction_read_terminates_under_concurrent_render_pressure`,
`repeated_local_interaction_reads_of_one_instance_terminate` and
`local_interaction_query_return_does_not_fault_the_next_maintenance_step` — **ok**.

`local_interaction_read_of_a_selected_document_terminates` — **FAILED on its precondition, not on
its subject**. The read never ran; `select_id` faulted first at `🧪️tests/🔬️unit/🦀️.rs:120`:

```
interactionSelect: Fault { origin: Framework, code: FaultCode("interactive-job.admission-capacity"),
  message: "framework reserved worker session capacity is exhausted" }
```

That is a process-wide fixed worker-session registry exhausted by the ~100 tests that ran before it
under `--test-threads=1`, and it takes down **every** selection-dependent test in the module with the
identical fault — `selected_object_inspector_renders_that_object_field_group`,
`selected_vortex_inspector_renders_the_vortex_field_group`, `world_pick_*` (4),
`world_select_emits_no_artifact_mutations`, `world_vortex_*` (2), `select_same_kind_*` (2),
`inspector_field_actions_resolve_selection_without_embedding_ids`, `patch_inspector_origin_axis_*`
(2), `gumball_*` (4), `hover_suggestion_…`. Run in isolation the same law passes
(§5: `5 passed`, twice, at the final source state). This is the same reserved-worker lane that
earlier in the session produced `interactive-job.worker-pump` on these tests — a peer's, not wave I's.

### The 44 → 57 delta

Not attributable to wave I: no failure in the list carries a local-interaction, snapshot-read-return,
maintenance-ordering or close assertion. The delta is dominated by the selection family above flipping
wholesale on this run (the audit's own §0 records ±5 tests flipping between two runs five minutes
apart, and its bucket (a) item 2 already counted 17 of its 44 in this family), plus the module's own
growth. The audit's `open_vortex_suggestions_*` / `window_options_*` / `set_camera_*` hang-class items
are all still present verbatim.

### A concrete lead for W-A, seen in the same log

`nakagin_example_loads_via_operations` spins to turn **9 519 104** with

```
[DEBUG] typed-operation publication turn=9519104 operations=["608:Publishing:true:true"]
        latest_wins_empty=true effects=0 events=0 ui=0 query=none
```

i.e. one typed operation parked in `Publishing` with `has_runnable_work() == true` **and**
`result_page_presented == true` — a presented page that keeps reporting runnable while the only
remaining step belongs to the host. That is exactly the failure shape wave I removed from the
local-interaction path, in the typed-operation publication path instead.
