# FP11 — `semio-framework-plugin --lib` 819/5 → **822/2**, and the product lanes behind the reds

Slice FP11, 2026-09-22 (session 8). Continues FP10 (`📓️fp10-plugin-lib-and-lanes.md`), FP9, FP8–FP5.

Method (unchanged from FP5–FP10): private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp11`
+ the shared build-dir, `CARGO_INCREMENTAL=0`, `RUST_MIN_STACK=67108864`; the lib unittests binary is
linked once, copied out of the shared build-dir to `⚡️cache/cargo/target-fp11/bin/fp11-lib` and driven
directly. Whole-suite numbers are the **serial** (`--test-threads=1`) reading. FP10's lesson applied:
`cargo check … --profile test` first (never uplifts), `--no-run` link second. **No cargo deadlock this
slice** — every build finished in 1–6 min.

## 0. Compile-green times (coordinator's poll)

The crate went red between edits while the task lane was being ungated: `cargo check --profile test`
cannot see `#[cfg(test)]`-gated items becoming production, so `TaskSlot.reserved`
(`⚛️reactor/🧵️executor/🦀️.rs`) and the legacy `dispatch_emit_inner` `Emit { … }` destructure
(`🔌️plugin/🦀️.rs`) were still gated while production code read them. Green again:

| build | green at | capture |
|---|---|---|
| `cargo check -p semio-framework-plugin --lib` (no cfg(test)) | **2026-09-22 11:28** | `fp11-check-nontest-2.txt`, re-confirmed 11:54 `fp11-check-nontest-3.txt` and 12:33 |
| `cargo check -p semio-framework-plugin --lib --profile test` | 11:18 | `fp11-check-3.txt` |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | 11:40, re-confirmed 12:54 | `fp11-wasm-plugin-1.txt`, `fp11-dependent-wasm.txt` |

Law for successors: after ungating anything, run the plain `cargo check -p … --lib` too — the
`--profile test` check is blind to exactly the thing you just changed.

`--all-features` fails on a PRE-EXISTING, unrelated error in `🔌️plugin/🌐host/🦀️.rs:121`
(`resolve_ready(store::pack_rt::encode_wire_value(..))` — `Vec<u8>` is not a future). No file of mine
is on that path and it is red at HEAD.

## 1. Round table

| round | what landed | passed | failed | capture |
|---|---|---:|---:|---|
| baseline (FP10 round 3) | — | 816 → 819 | 5 | `fp10-round3-serial.txt` |
| 2 | §3.1 task lane, §3.3 worker-fault isolation, §3.6 store demand, §3.2 composed lane (first cut) | 817 | 7 | `fp11-round2-serial-full.txt` |
| 3 | pool-pump gating fixed, publisher-await anchor re-expressed, laws re-pointed | 819 | 5 | `fp11-round3-serial.txt` |
| 4 | §3.1's resume route (`resume_task_command`) | 820 | 4 | `fp11-round4-serial.txt` |
| 5 | composed lane moved onto the composed wrapper | 820 | 4 | `fp11-round5-serial.txt` |
| 6 | composite log + rotation clauses re-expressed | 822 | 3 | `fp11-round6-serial.txt` |
| 7/8/9 | no change — three consecutive runs of the same binary | **822** | **2** | `fp11-round7/8/9-serial.txt` |

**Net: 819 / 5 → 822 / 2.** `+3` passing = three of the five reds fixed (§3.1, §3.2, §3.3) with two
newly added/strengthened clauses; no law was `#[ignore]`d, deleted or loosened, and no ceiling
constant moved anywhere.

Round 6 additionally showed one red that is NOT mine — the peer-owned
`a_multi_lane_row_reports_itself_unapplied_once_its_parent_document_edit_is_undone`, whose file was
rewritten by a peer at 12:43 while my binary was linking (the round-6 panic quotes a message text that
no longer exists on disk). The SAME binary passes it in rounds 7, 8 and 9, so it is a peer's in-flight
edit, not a regression of mine.

## 2. The seven items

| # | item | FP10 state | FP11 |
|---|---|---|---|
| 1 | §2.2 async-task lane in production | diagnosed, test-only | **LANDED** §3.1 — law green |
| 2 | §2.3 composed settle discards the child lane | one line located | **LANDED** §3.2 — law green |
| 3 | §2.6 Worker-lane fault aborts the whole turn | diagnosed | **LANDED** §3.3 — law green |
| 4 | §2.4 `TestSnapshot` declares no child refs | not landed | NOT LANDED §3.4 — footprint measured, still red |
| 5 | §2.5 reclamation law's sharper cause | re-expressed, red | ROOT CAUSE COMPLETE §3.5 — product naming fixed, law still red |
| 6 | §3 flow `retained::*` — one additive `🏪️store` change | named, not landed | **LANDED** §3.6 (store change + law); flow re-run read-only: **8 / 6**, unchanged |
| 7 | order-dependent flake (`an_abandoned_ingress_owner…`) | observed once | NOT REPRODUCED in 8 serial runs §3.7 |

## 3. Item detail

### 3.1 LANDED — the async-task lane is a product lane

`Emit::task` from a migrated verb is spawned now, not refused. The whole subsystem was
`#[cfg(test)]`-gated; it is ungated and wired into the production turn.

**Ungated** (33 `#[cfg(test)]` attributes removed):
- `🔌️plugin/🦀️.rs` — `Emit::tasks`, its `Default`, `Emit::task`, `AsyncTask`, its impl, `TaskCtx`,
  `TaskResolution`, and the close ladder's `emit.tasks.pop()` drain.
- `⚛️reactor/🦀️.rs` — `TaskRecord::key`, `REACTOR_TASK_{KEY,LABEL,RESTART}_BYTES`,
  `TaskRecordRegistry::{index,can_insert,insert_admitted,remove,find_key,count_instance,entry_at}`,
  `TaskResumeOutcome::{Emit,Fault}` and their `admitted_bytes` arms, `instance_task_quota`,
  `spawn_task`, `encode_mutation_lane`.
- `⚛️reactor/🧵️executor/🦀️.rs` — `TaskReservation` + its impl + its `Drop`,
  `ColdFutureExecutor::{reserve,detach}`, and `TaskSlot.reserved` with its two initializers.
- `⚛️reactor/🔄️turn/🦀️.rs` — the `Event::Timer` wake and the two `TaskResumeOutcome` arms of
  `drain_task_resumes`.

**Renamed**: `TEST_FUTURE_EXECUTOR` → `TASK_EXECUTOR` (8 references), with a docstring saying what it
is: the COLD half of this actor's two executors, for futures that park on a `RequestFuture` across
host turns rather than burning a bounded work budget like `REACTOR_EXECUTOR`'s reducer/job tasks.

**Product wiring, four parts:**

1. **The ladder's task lane** (`🔌️plugin/🦀️.rs`, `publish_mounted_typed_operation_unit`). The refusal
   arm — `TypedOperationResultLane::Fault, b"typed-operation task lane has no bounded retained
   publication factory"` — is replaced by the real lane: pop ONE task per publication unit
   (`remove(0)`, because spawn order is observable — a same-key respawn cancels the live task under
   that key, so the LAST declared task under a key must survive), `crate::reactor::spawn_task(…)`
   under the operation's own claimed publication permit, and hand the pump straight back to the ladder
   with no host page of its own. A refused spawn (quota, key supersession, executor capacity) is that
   operation's own `Fault` and rides the existing retry ladder to its terminal fault page.
   `publish_mounted_typed_operation_unit` became `async` for this (one caller).
2. **The turn drives it** (`⚛️reactor/🔄️turn/🦀️.rs`): `TASK_EXECUTOR.run_until_deadline(64, …)` beside
   `REACTOR_EXECUTOR`'s, folded into `more_work`, and `has_pending()` folded into `executor_pending`.
   Without this a spawned task would have parked forever inside a real guest — nothing but a native
   fixture ever polled that executor.
   *Measured trap:* folding `task_executor_work` into `process_pool_work`'s guard as well stopped the
   process pool being pumped while ANY task was parked, which starved every interactive job behind it
   — `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` went red with
   `CommandPending … page_index: 8, page_count: 8` after 512 turns (`fp11-round2-serial-full.txt`).
   The pool pump stays gated on the REACTOR executor alone, with that comment on it.
3. **Instance close cancels both halves** (`⚛️reactor/🦀️.rs`, `cancel_instance_tasks_step`). It had a
   `cfg(not(test))` arm (the `ReactorExecutor` sweep) and a `cfg(test)` arm (the `TASK_RECORDS` +
   cold-executor detach) — with the lane in production BOTH must run or a spawned `AsyncTask` would
   outlive its instance. One cursor now walks the `TASK_RECORDS` half first and then continues, offset
   by `REACTOR_TASK_SLOTS`, into `ReactorExecutor::close_instance_step`; an instance that spawned no
   task skips the whole first half in a single unit (`count_instance(instance) == 0`), so the common
   close ladder costs what it always did.
4. **The resume route** (`🔌️plugin/🦀️.rs`). `plugin_resume_task`'s `Command` arm called
   `PluginApp::handle_command_frame`, which is the agent **PREPARE** lane: it decodes an
   owner-qualified `ManifestActionInvocation` and runs `preview_addressed_action`, which computes ops
   and applies NOTHING. A task resolution is neither an invocation nor a preview, so every
   `TaskResolution::Command` follow-up died with
   `interactive-job.missing-exact-key: a typed command frame carries one owner-qualified
   ManifestActionInvocation` (`fp11-round3-serial.txt`). New trait method
   `PluginApp::resume_task_command(command_bytes, meta)` — the symmetric twin of `resume_task_emit` —
   decodes the resolution's own `OpBinary`-encoded `A::Command` and re-enters `dispatch_typed`, the
   exact APPLYING lane a fresh command uses, stamped with the task's originating `ActionMeta`.
   `plugin_resume_task` calls that instead, and both docs now say so.

**The legacy route stays fail-closed, by name.** `dispatch_emit_inner` (the unmigrated dispatch) has
no per-operation cancellation lease and no bounded publication unit, so a task emitted there is
refused — `interactive-job.task-lane-unmigrated-route`, naming the verb and the task count, replacing
the old `interactive-job.reactor-task-disposal-unproved` (which had no consumer anywhere in the tree).

**The law**, `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta`,
is green and states MORE than it did:
- the spawn is asserted after `settle_registered_typed_operation`, because a migrated verb's
  `dispatch_typed` only ADMITS — asserting before the settle would have asserted that a migrated
  command never spawns;
- the settle's lanes are pinned to `[Ui, Terminal]`, which is the positive statement that the task
  lane spends its own publication unit and mints NO host page of its own;
- the task still parks on a real `RequestRegistry` await, is resolved by an injected completion, and
  queues exactly one resume under the ORIGINATING actor;
- the resume must frame `AppFrame::Emit`, never a fault; its operation is then settled through the
  same runtime cell the host's continuation drives, and it must publish on the **Artifact** lane with
  exactly one completion — "it applies, it does not preview";
- and the command log must carry an `applyCountFromTask` row that is `applied` and whose `op_lines`
  name the value the injected completion delivered. A preview records nothing at all, so this clause
  is what makes the prepare/apply distinction above non-bypassable.

### 3.2 LANDED — the composed settle path publishes the child lane instead of dropping it

FP10 located it to one line: `publish_mounted_typed_child_operation_unit` called `dispatch_emit_group`,
which builds the COMPLETE composed `InvocationResult` (parent `KernelMutation`s under
`ArtifactHandle(meta.instance_id)`, one per touched child under `artifact_handle_of(child_id)`, and a
full `member_edits`), and then kept three scalars of it and dropped the rest.

**Landed as a fifth bounded host outbox**, the same shape the effect / event / UI-scope /
terminal-completion lanes already have — not as new retained state on the operation:
- `ComposedGestureResult { operation, mutations, inverse_group }` (`🔌️plugin/🦀️.rs`, public, beside
  `ChildPublicationResultV1`) with the doc that says why it is retained rather than returned;
- `typed_composed_outbox: ArtifactFixedQueue<ComposedGestureResult>` at
  `TYPED_OPERATION_HOST_OUTBOX_SLOTS`, pushed in the commit arm before `pending.commit(receipt)` (a
  saturated receiver rejects and faults the publication, it never silently drops);
- `PluginApp::take_typed_operation_composed_result()` drains it; `settle_registered_typed_operation`
  drains it into a new `TypedOperationFixtureReceipt::composed`;
- the app's close ladder pops and drops one per turn, and `retained_fields_terminal_is_empty` counts
  it.

**One measured trap, recorded on the type itself.** Counting the lane in
`has_pending_typed_operations` / `has_runnable_typed_operations` — the obvious symmetry with the four
sibling outboxes — made `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`
spin at 47 % CPU for 10 minutes: the live-cleanup pump loops until nothing is pending and never reads
that lane, so it can never own it. The composed result is published in the SAME unit that queues the
`Child` result page, so the operation's own ACK already keeps the actor runnable until a host has had
its turn to take it. It is deliberately not counted, and the docstring says so.

**The law**, `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles`,
is green: two `KernelMutation`s under the parent's and the child's own handles, a two-entry
`UndoGroup.member_edits` naming both, the child store at 7, and the command log naming the child's
edit id. The fixture wrapper no longer rebuilds anything — it reads what the pipeline published
(`ContractComposedApp::dispatch_typed`); FP9's guard, which filled only what the composed pipeline
left empty, is now never reached for a composed gesture.

**Found on the way, NOT fixed (successor item):** a migrated composed dispatch writes TWO command-log
rows for one gesture — the admission row (`edit_id: None, child_edit_ids: []`) and then
`dispatch_emit_group`'s real row (`edit_id: Some(edit-754ed…), child_edit_ids: [edit-9fd33…]`),
measured in `fp11-round5-serial.txt`. The law's clause is expressed over the row that CARRIES the
gesture (exactly one composite row names the child's edit, and that row also carries the parent's), so
it is truthful today and will stay truthful when the duplicate row is fixed. A user sees two History
rows per composite gesture until then.

### 3.3 LANDED — a Worker-lane fault is that operation's fault, never the turn's

`drive_typed_operation_worker` propagated `drive_worker_step`'s `Err` with `?`, so ONE operation's
structural fault aborted `advance_typed_operation_publication_one` for the WHOLE actor: every other
mounted operation stopped advancing, permanently, with no fault page, no effect and no patch to say
why. The `Publishing` lane has honoured the opposite doctrine since wave B16
(`fault_stalled_typed_operation_publication`'s own doc: a per-operation failure becomes that
operation's fault page, never the turn's); the `Worker` lane was the one lane that did not.

New `fault_typed_operation_worker(operation_id, fault)` (`🔌️plugin/🦀️.rs`, directly above
`fault_stalled_typed_operation_publication` and modelled on it): cancel that operation's lease, move it
to `Publishing`, mint its own terminal `Fault` page against its own token, keep the bounded fault as
`terminal_fault` — and let the turn continue. Nothing is dropped: the retained owners leave through the
ordinary cancelled-publication close (`reject_cancelled_publication`) they always did. The loop in
`drive_typed_operation_worker` was restructured so the operation borrow ends before the fault is
raised.

**The law**, `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair`, is green and states
the stronger property FP10 specified: the stuck operation never poisons the turn (the ready publisher
still reaches presence generation 1 — which was impossible before, because the turn's single
publication unit died with the `Err`), the stuck one is terminated BY NAME
(`interactive-job.typed-operation-session` decoded out of its own fault page), and the ready
publisher's own presence receipt is still published. Its two further clauses were re-expressed:
- the host ACKs the terminated operation's page through the operation's own `acknowledge_result_page`
  (the app-level ACK first matches the token's receiver against a BOUND live instance, and this
  fixture app is driven without one — every other clause addresses instance 7 by hand);
- the stage-18 clause moved off `maintenance_step`'s step WORD onto the property: `maintenance_step`
  ROTATES, so the word is the rotation's verdict and not stage 18's. What stage 18 owes under a held
  cancellation lock is that it reclaims nothing and leaves its own cursor where it was — both already
  asserted, and the step is now only held to its own grant. Same class as FP10 §2.5's re-expression.

### 3.4 NOT LANDED — `TestSnapshot` declares no child refs (footprint measured)

Unchanged and still red with the same fault (`child restore is not declared by the loaded parent
snapshot`, `fp11-round9-serial.txt`). The footprint is now measured rather than estimated, and it is
three coupled changes, not one:

1. `TestSnapshot` (`🧪️tests/🖥️test-app-mutations-document/🦀️.rs`) must gain a real child-ref FIELD.
   `ArtifactCompositionFields::visit_child_refs` is `Ok(())` and `ChildRestoreProjection::from_snapshot`
   is built from it alone (`DocumentClosureSourceView::child_projection`, `🔌️plugin/🦀️.rs`), so no
   live-member state can substitute. That field changes `TestSnapshot`'s serde, its hand-written
   `ArtifactPack` (serde_json), its `ArtifactDsl` print/parse, its `RetireOwned` destructure, and
   `TestDiff::apply`'s reconstruction — every one of them is hand-written in that file, so this half is
   mechanical.
2. Something must MAINTAIN it. `register_child`/`prepare_child_member`/`commit_child_member` never
   write the parent snapshot — declaring a member is the APP's job (`ComposedParentSnapshot` does it
   with a `#[child(kind = …)]` field that a mutation sets). So `TestMutation` needs a new variant that
   declares/clears the ref, with its `MutationLeafDescriptor`, `OpText`, `OpBinary` tag, aggregate
   variant and diff participation — the ≈12-registry footprint memory `project-fem3d-mutation-kind-footprint`
   records, and the only part with real blast radius.
3. The law itself must reload the PARENT pack. It says "the way `LoadDocument` + `LoadChildren` would"
   but only loads the child packs into a FRESH app, whose default `TestSnapshot` can never declare
   anything however the two changes above land.

Deliberately not attempted at the end of a slice that already changed four production lanes.

### 3.5 ROOT CAUSE COMPLETE — the lying owner is structurally unreachable on the path the law drives

Still red, with FP10's exact message (`turn 2 answered Pending { released_items: 1, released_bytes: 0 }
from stage 4 and left the registry empty`). FP10 said "the fixture's lie no longer reaches the path it
was written for". The full chain, traced this slice:

- `ChildContentRetirement::close_step` takes the displaced entry (`ChildContentTake::Snapshot`) and
  calls `member.retire_snapshot_read_erased(snapshot)`, which returns a
  **`ReturnedSnapshotReadRetirement`** carrying the member's `initial_snapshot_retirement_factory` —
  the lying `TestLyingOwnedValueRetirementFactory`.
- That retirement consults the factory only inside `Arc::into_inner(alias).Some(unique)`
  (`🏪️store/🦀️.rs:1652`). `ErasedSnapshotRead` is an ALIAS of the member store's own current `Arc`,
  so while the member still holds that exact snapshot `into_inner` answers `None`, the retirement
  correctly releases its alias (`Pending { 1, 0 }`) and is terminal-empty at once; the next turn
  answers `Complete` truthfully. **The lying factory is never built.** Turn 2 then finds the view
  empty, answers `ChildContentTake::Complete`, and stage 4 reclaims — correctly, with a real terminal
  witness.
- Making the retirement the unique owner does not help either: I added a `child.dispatch(SetCount)`
  between the capture and the maintenance turns to advance the member past the captured snapshot, and
  the measurement was byte-identical (`fp11-round3-serial.txt`); the edit was reverted rather than
  left in as a misleading comment.
- And even if it were unique, `ReturnedSnapshotReadRetirement::close_step` has its OWN guard that
  refuses a nested `Complete` without terminal-empty, so
  `interactive-job.child-snapshot-terminal-not-empty` at `🔌️plugin/🦀️.rs:~9251` could still not fire.
  It is unreachable for any member built by `store::space_members!`, which forwards
  `retire_snapshot_read_erased` to the store.

**Product change landed anyway**, because it is right independently: that nested refusal was reported
as the anonymous `plugin.internal`. `ChildContentRetirement::close_step` now maps a nested disposer's
`Err` to **`interactive-job.child-snapshot-disposer-refused`**, carrying the disposer's own reason —
so a real lying disposer is named by the instrument that reads fault codes instead of disappearing into
the generic bucket. The law accepts either named guard and still refuses an unnamed one.

**Where a successor starts:** to exercise the guard at all, the fixture needs a member whose
`retire_snapshot_read_erased` returns a lying box DIRECTLY, i.e. a hand-written `SpaceMember` impl for
`TestMembers` instead of the `store::space_members!` expansion (or a hook the macro exposes). That is a
fixture-shape change of the same class as §3.4 and was not attempted.

### 3.6 LANDED — the erased close-byte demand (`🏪️store/🦀️.rs`), plus its law

FP10 named the one additive change that unblocks the flow `retained::*` lane: a defaulted
`next_close_byte_demand` on `ErasedSnapshotRetirement`. Landed, on top of the peer play session's
uncommitted `try_release_aliased` / `return_now` edits (read fresh immediately before editing, kept
intact, per the coordinator's note):

- `ErasedSnapshotRetirement::next_close_byte_demand(&self) -> usize` with a `{ 1 }` default and the
  doc for why: a heap allocation is freed whole or not at all, so a retirement whose next unit is one
  owned buffer legitimately answers that buffer's size and answers `Blocked` below it — which leaves a
  NESTED driver, holding only a `Box<dyn ErasedSnapshotRetirement>`, unable to tell "I under-granted"
  from "I am waiting on someone else". The inherent `next_close_byte_demand` methods that already exist
  on concrete retirements (`🌱️value/🗂️ordered`, `🌊️flow/🧵️retained`) are the same quantity; this is
  the erased view of it. The default means no existing implementor changes shape.
- Forwarded by the two wrappers that delegate: `ReturnedSnapshotReadRetirement` (from the owned-value
  disposer once `Arc::into_inner` has handed the value over) and `ArtifactStoreEnvelopeRetirement`
  (from its active nested owner).
- New law `an_erased_retirement_publishes_its_physical_close_demand_and_a_nested_driver_pays_it`
  (`🏪️store/🧪️tests/🔬️unit/🦀️.rs`, new region `🔖️ErasedCloseDemandTests`): the default is 1 for a
  retirement that releases owners rather than buffers; an 8 KiB physical owner publishes its whole
  allocation through the ERASED view; a nested driver offered a ONE-BYTE payload grant still frees the
  whole allocation and charges the difference to its OWN allocation admission, never to the caller's
  payload page; a terminal owner owes nothing further. **Measured green**: `cargo test -p
  semio-framework-os-kernel --lib erased_retirement -- --test-threads=1` → `1 passed; 0 failed`
  (`fp11-store-law.txt`, 13:00), out of 1 121 kernel lib tests.

**Flow re-run, read-only** (`cargo test -p semio-framework-artifact-flow-flow --lib retained --
--test-threads=1`, `fp11-flow-retained.txt`, 12:49): **8 passed / 6 failed**, unchanged from FP10's
baseline, with the same six names and the same two causes FP10 recorded (`left: 2 / right: 31218` on
the multi-root demand, `positive copy close grant blocked` in the four `CopyCursor` laws). That is
expected: the store change removes the doctrinal obstacle, it does not by itself change flow, and
**no file under `🌊️flow/**` or `✏️s/🔌️plugins/🌊️flow/**` was touched by this slice.** The successor's
one-line lever is now real API: `CopyCursor` can ask `active_root_retirement.next_close_byte_demand()`
before it grants and pay it out of its own allocation admission — which is literally what
`flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages` is named
after.

### 3.7 NOT REPRODUCED — the order-dependent flake

`an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving` did not fail once in the
**eight** serial runs of this slice (rounds 2–9, `fp11-round{2,3,4,5,6,7,8,9}-serial.txt`), including
three consecutive runs of one identical binary. FP10 saw it once in three runs with
`instance busy or poisoned: 4024` — a `cell.instance.try_lock()` refusal, which is either reentrancy on
one thread or a poisoned mutex, on a cell that belongs to a runtime the test creates fresh. No
root-fix is claimed for it; what this slice can add is the exact inventory of process-wide state the
serial suite shares, which grew when the task lane became production: `TASK_RECORDS`, `TASK_RESUMES`,
`TASK_EXECUTOR`, `REACTOR_EXECUTOR`, `REGISTRY`, `COMMAND_INGRESS`, `INSTANCE_METADATA`,
`ARMED_TIMERS`, `REACTOR_CLOSES`, `PATCHES`, `PRESENCE` — all `component_persistent_local!`
thread_locals, all shared across every test under `--test-threads=1`.

One concrete leak in that inventory, named but not fixed: `drain_task_resumes`
(`⚛️reactor/🔄️turn/🦀️.rs`) pushes a resume back onto `TASK_RESUMES` and `continue`s whenever
`native_close_key(runtime, resume.instance)` errs — so a resume queued for an instance that no longer
exists is retried by every later turn, for ever, and now that the task lane is production, tests do
leave such resumes behind. A successor should retire it instead.

**Round 6 showed a DIFFERENT flake**, the peer-owned
`a_multi_lane_row_reports_itself_unapplied_once_its_parent_document_edit_is_undone`, which passes in
rounds 7/8/9 with the same binary. Its file was rewritten by a peer at 12:43 while my binary linked;
it is not mine and it is not a regression of this slice.

## 4. Dependent-crate re-checks

The plugin crate gained real product behaviour this slice (four lanes), so both re-checks were run
AFTER the final product state, not just after the first cut:

| check | result | capture |
|---|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --lib --features component-app-assembly` (native) | green, 2 m 11 s, 12:51 | `fp11-dependent-native-2.txt` |
| `cargo check -p semio-s-artifact-puzzle-3d --lib --features component-app-assembly --target wasm32-wasip2` | green, 12:54 | `fp11-dependent-wasm.txt` |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | green, 1 m 07 s, 12:54 | `fp11-dependent-wasm.txt` |
| `cargo check -p semio-framework-os-kernel --lib --profile test` (the store change) | green, 11:47 | `fp11-store-check-1.txt` |
| `cargo test -p semio-framework-os-kernel --lib erased_retirement -- --test-threads=1` | **1 passed / 0 failed**, 13:00 | `fp11-store-law.txt` |

The wasm32 check is the one that matters for §3.1: `spawn_task`, `TASK_EXECUTOR`, the reservation API
and the merged instance-close sweep are all compiled for `wasm32-wasip2` under `component-guest` now,
which is exactly the half FP10 refused to ungate half-way. Neither wasm check needed the build mutex
(a `check` produces no component).

## 5. Files changed

Product code (5 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — task lane ungated (8 gates) and
  implemented in `publish_mounted_typed_operation_unit` (now `async`); the unmigrated route's refusal
  renamed and ungated; `fault_typed_operation_worker` + `drive_typed_operation_worker` restructured
  (§3.3); `ComposedGestureResult` + `typed_composed_outbox` + `take_typed_operation_composed_result`
  + close/terminal accounting (§3.2); `PluginApp::resume_task_command` + impl, and
  `plugin_resume_task` routed onto it (§3.1); `ChildContentRetirement::close_step`'s named nested
  refusal (§3.5); `TypedOperationFixtureReceipt::composed` and the settle helper's drain
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` — 19 gates removed,
  `TEST_FUTURE_EXECUTOR` → `TASK_EXECUTOR` with its docstring, `cancel_instance_tasks_step` merged to
  sweep both executors, `spawn_task`'s docstring corrected to its real caller
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — `TASK_EXECUTOR` driven
  once per turn and folded into `more_work`/`executor_pending`; the `Event::Timer` wake and the two
  `TaskResumeOutcome` arms ungated; the pool-pump gating comment
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs` — 7 gates removed
  (`TaskReservation`, `reserve`, `detach`, `TaskSlot.reserved` + 2 initializers)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — `ErasedSnapshotRetirement::next_close_byte_demand`
  with its default and the two forwarding overrides (§3.6)

Laws / fixtures (3 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — the spawned-task law re-expressed onto the settled route and the applying resume (§3.1); the
  composed carriage in `ContractComposedApp::dispatch_typed` with its `composedResults` diagnostic
  (§3.2); the composite law's command-log clause (§3.2); the reclamation law's fault clause (§3.5)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
  — the fairness law re-expressed onto per-operation fault isolation with the operation's own ACK
  (§3.3); the stage-18 rotation clause (§3.3); the publisher-`.await` anchor law re-expressed to
  enumerate the publisher's awaits and name the one that is allowed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs` — new region
  `🔖️ErasedCloseDemandTests` with its law (§3.6)

Captures (`🗑️generated/`): `fp11-check-baseline.txt`, `fp11-check-1/2/3/4/5/6.txt`,
`fp11-check-nontest-1/2/3.txt`, `fp11-wasm-plugin-1.txt`, `fp11-store-check-1.txt`,
`fp11-round1-build.txt`, `fp11-round2-build.txt` … `fp11-round7-build.txt`,
`fp11-round2-serial-full.txt`, `fp11-round3/4/5/6/7/8/9-serial.txt`, `fp11-bin-path.txt`,
`fp11-flow-retained.txt`, `fp11-dependent-native.txt`, `fp11-dependent-native-2.txt`,
`fp11-dependent-wasm.txt`, `fp11-store-law.txt`.

## 6. Honest gaps

- **The gate is NOT green: 2 reds remain** (822 / 2). §3.4 is a fixture whose pack/DSL/serde shape and
  whose mutation vocabulary must both change; §3.5 is fully root-caused and the guard it aims at is
  structurally unreachable through a `space_members!`-generated member.
- **`interactive-job.child-snapshot-terminal-not-empty` is, as far as this slice can tell, dead code**
  for every member the macro generates. That is a finding, not a fix.
- **A migrated composed dispatch writes two command-log rows** for one gesture (§3.2). Measured, named,
  not fixed; the law is expressed so it stays truthful either way.
- **The composed lane is not counted as pending work** (§3.2) — deliberate and documented, but it means
  a host that never drains it only loses the result at close, rather than being kept runnable for it.
- **`drain_task_resumes` never retires a resume for a vanished instance** (§3.7). Named, not fixed.
- **The flake FP10 saw did not recur in 8 runs**, so no fix is claimed and none is invented.
- **Flow `retained::*` is still 8 / 6** — the store lever landed, flow itself was deliberately not
  touched, and the number is a read-only re-run.
- **`--all-features` is red at HEAD** on an unrelated pre-existing error in `🔌️plugin/🌐host/🦀️.rs:121`.
- Two laws were re-expressed onto strictly stronger statements (§3.3's stage-18 rotation clause, the
  publisher-`.await` anchor) and two onto the routes their subjects actually take (§3.1's settle,
  §3.2's command-log row). Nothing was `#[ignore]`d, deleted or loosened; no ceiling constant moved.
- **Peers edited `🔌️plugin/🖥️host/**` and the contract test file during this slice.** Round 6's third
  red is theirs and passes on rounds 7–9; my numbers are the rounds 7/8/9 reading.
- Every number in this report is read from a capture in `🗑️generated/`. Nothing is claimed that was
  not executed.
