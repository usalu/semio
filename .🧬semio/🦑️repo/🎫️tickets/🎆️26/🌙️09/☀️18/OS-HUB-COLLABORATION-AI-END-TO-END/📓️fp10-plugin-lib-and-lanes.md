# FP10 — `semio-framework-plugin --lib` 816/6 → **819/5**, plus the four product lanes

Slice FP10, 2026-09-22 (session 14). Continues FP9 (`📓️fp9-plugin-lib-zero-red.md`), FP8–FP5.

Method (unchanged from FP5–FP9): the lib unittests binary is built once with the private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp10` + the shared build-dir, copied out of the
shared build-dir to `⚡️cache/cargo/target-fp10/bin/fp10-lib`, and driven directly with
`RUST_MIN_STACK=67108864 CARGO_INCREMENTAL=0`. Whole-suite numbers are the **serial**
(`--test-threads=1`) reading.

> **cargo deadlock at ~15:00, ~22:00 and ~14:20 (three times).** Every `cargo test --no-run` on
> `semio-framework-plugin` and on the flow crate parked at 0 % CPU in `prebuild_lock_exclusive →
> flock` with NO child process, while 8–42 unrelated rustc ran — so rule 23(a)'s "no rustc anywhere"
> test would have missed all three; rule 27(b)'s corrected test caught them. Killed MY pid each time
> (rule 33: never a peer's) and rerun once. The cure that finally worked is rule 25's own note:
> **`cargo check -p semio-framework-plugin --lib --profile test` never uplifts and finished in 32 s**
> while the equivalent `--no-run` had been parked 22 min. Successors: check first, link second.

## 1. Measured

| suite | before | after | capture |
|---|---|---|---|
| `semio-framework-plugin --lib` (serial) | 816 / 6 | **819 / 5** | `fp10-round0-serial.txt` → `fp10-round3-serial.txt` |
| `semio-framework-artifact-infinite-dag --lib` | 57 / 1 | **58 / 0** | `fp10-dag-round1.txt` |
| `semio-framework-artifact-flow-flow --lib retained` | 8 / 6 | 8 / 6 (attempt reverted, §3) | `fp10-flow-retained-round0/1/revert.txt` |

Round 0 reproduced FP9's end state exactly, name for name (816/6, 30.81 s). Both wall-clock ceilings
drew green in every round of this slice, so all six survivors have a source cause.
`+3` passing = the one fixed red plus two NEW laws (§2.1).

## 2. The six reds

| # | law | class | FP10 |
|---|---|---|---|
| 1 | `editor_fixture_still_mutates_normally` | fixture proved what the runtime FORBIDS | **FIXED** §2.1 |
| 2 | `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta` | product: the async-task lane is test-only | diagnosed, not landed §2.2 |
| 3 | `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles` | product: the composed settle path BUILDS then DISCARDS the child lane | root cause located to one line §2.3 |
| 4 | `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | fixture: `TestSnapshot` declares no child refs | not landed §2.4 |
| 5 | `child_root_maintenance_requires_terminal_empty_before_reclaim` | law pinned a stage the rotation no longer honours | re-expressed; now red on a SHARPER cause §2.5 |
| 6 | `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | product: a per-operation Worker fault aborts the WHOLE turn | diagnosed, not landed §2.6 |

### 2.1 FIXED — the editor fixture now has a real OWNED factory (FP9 §4.3)

`SurfaceEditorFixture` had no `command_id` override, no manifest and no `ToolJobFactory`, so the law
dispatched the generic `"typed-command"` verb into a registry-less `new_app` wrapper and died at
`require_ui_safe_declaration` with `interactive-job.unknown-key`. It proved the fail-closed refusal,
not that an editor mutates.

Landed in `🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`, new region
`🧪️SurfaceRegisteredFactory` — the exact five parts FP9 §4.3 specified:

- **`SurfaceFixtureJob`** — a real `semio_framework_job::InteractiveJob`: owns its typed command, its
  retained wire pages and its `ArtifactToolCompletion<EditorApp<SurfaceEditorFixture>>`; answers
  cancellation first, yields once per admitted page, hands back ONE exact completion, and closes one
  item per turn to terminal-empty.
- **`SurfaceFixtureFactory`** — `ToolJobFactory` (`Migrated`, `ToolExecutionContract::resumable(4_096,
  1, 1, 4_096, 500, 1, 1)`, wire-page constructor that refuses a checkpoint resume) plus
  `ArtifactOwnedToolJobFactory<Owner = EditorApp<SurfaceEditorFixture>>` on the `Artifact` lane.
- **`surface_manifest()`** — `App::builder(SURFACE_CONTROLLER_ID, …).document(["state"])
  .mode("edit", …).window_kind("main", …).app_command("increment", …, ActionKind::Mutation)
  .interactive_jobs(Migrated)`.
- **`OpBinary::TOOL_JOB_IDS = &["increment"]`**, `ArtifactEditor::command_id` overridden to
  `"increment"` (the generic `"typed-command"` is excluded from `expected` BY NAME in
  `validate_tool_job_rows`, `🔌️plugin/🦀️.rs:13255`, so a proof row for it can never be
  authoritative), `bounded_first_step_tool_proofs!` with
  `controller: "testkit.surface@1/*#editor"` — the DERIVED surface id, not `EditorApp::APP_ID`'s
  `"surface"` placeholder — `register_tool_job_factories` forwarding the factory, `build_tool_job`
  minting it, and `build_artifact_store_one_item_preparation_factory` for the Artifact lane.
- **The law settles.** A migrated verb's `dispatch_typed` only ADMITS; the document advances when the
  worker's emit walks the bounded publication ladder. The law now calls
  `settle_registered_typed_operation` and asserts the receipt carries the `Artifact` lane the factory
  declares, THEN asserts `count == 1`. (Measured: without the settle the count is 0 — asserting
  before settling would have asserted that a migrated editor does not mutate.)

Two NEW laws, both green, guard the parts that could silently drift:
- `surface_editor_controller_is_the_derived_id` — joins the controller literal to
  `semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), AppRole::Editor)` AND to the live
  `app.app_id().await`, so a drift fails here by name instead of as
  `interactive-job.catalog-controller` from inside an unrelated dispatch.
- `editor_fixture_without_a_manifest_declaration_fails_closed` — keeps the OLD law's real content:
  the registry-less wrapper still refuses `interactive-job.unknown-key`, the refusal now NAMES the
  editor's own verb (`increment`, not the placeholder), and the document is untouched. Nothing the
  old law asserted was dropped; it moved to the law whose subject it actually is.

### 2.2 DIAGNOSED, NOT LANDED — the async-task lane

FP9 §5.1 confirmed and sharpened. `Emit::tasks` (`🔌️plugin/🦀️.rs:10944`), `AsyncTask` (`:11015`),
`TaskCtx`, `TaskResolution`, `reactor::spawn_task`, `TASK_RECORDS`, `TASK_RESUMES` and
`TEST_FUTURE_EXECUTOR` are ALL `#[cfg(test)]`. `spawn_task`'s own docstring says it is "Called from
`🔌️plugin/🦀️.rs`'s `dispatch_emit`, right after a gesture's mutation lanes land" — it has no caller
outside tests. The migrated publication ladder answers the lane BY NAME at `🔌️plugin/🦀️.rs:28615`:

```rust
#[cfg(test)]
() if !emit.tasks.is_empty() => TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, b"typed-operation task lane has no bounded retained publication factory")?,
```

Making this a production lane is not a fixture fix and not a one-line wiring: it means ungating the
whole reactor async subsystem (`AsyncTask` + `TaskCtx` + the awaitable `host::*` surface +
`RequestRegistry`/`RequestFuture` + the actor-local `LocalExecutor` + the erased `TASK_RESUMES`
queue), giving the ladder an explicit bounded state machine that spawns at most one task per
publication unit under the operation's own cancellation lease (`publish_mounted_typed_operation_unit`
is SYNC, so the spawn needs the async twin `publish_mounted_typed_child_operation_unit` already has),
and validating it on `--target wasm32-wasip2` — that code is the wasm-gated half. Deliberately not
attempted half-way: a partial ungating that fails to compile on wasm32 breaks every plugin build on
the machine. The law was NOT re-expressed to call `spawn_task` directly — that tests a copy of the
reducer's task body instead of the product's route (FP9's judgement, still right).

### 2.3 ROOT CAUSE LOCATED TO ONE LINE — the composed settle path discards what it built

FP9 §5.4 said `result_from_last_edit` "has no composed variant". True
(`🔌️plugin/🦀️.rs:25170`–`25235`, `member_edits: Vec::new()` hard-coded) but NOT the root. The root is
in `publish_mounted_typed_child_operation_unit` (`🔌️plugin/🦀️.rs:~28195`): the migrated composed
publication calls `self.dispatch_emit_group(…)`, and `dispatch_emit_group`
(`:25614`–`:25836`) builds the COMPLETE composed `InvocationResult` — parent `KernelMutation`s under
`ArtifactHandle(meta.instance_id)`, one child `KernelMutation` per touched child under the
content-addressed `artifact_handle_of(child_id)`, and a full `member_edits: Vec<EditRef>` — exactly
what the law demands. The ladder then throws all of it away and keeps three scalars:

```rust
let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0, committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_content_generation };
```

So the child lane is not missing — it is BUILT and DROPPED. (The sibling law
`a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them` passes, which proves the child
document really is edited on this route; only the REPORTING is lost.) Landing it means giving the
mounted operation a bounded, explicitly-retired slot carrying those mutations and that `UndoGroup`
back through the settle receipt, and draining it in the operation's close ladder
(`🔌️plugin/🦀️.rs:18620`–`18700`) and in `terminal_is_empty`. That is new retained state in the
strictest close-accounting region of the file and was not attempted with the build queue at 15–22 min
per round. FP9's shim guard stays: it fills only what the composed pipeline left empty, so the day
the slot lands the law goes green without touching the shim.

### 2.4 NOT LANDED — `TestSnapshot` declares no child refs

FP9 §5.5 confirmed, unchanged. `validate_parent_child_restore` (`🔌️plugin/🦀️.rs:24401`) admits a
restore only if the loaded PARENT snapshot declares that `(slot, child)` through
`ArtifactCompositionFields::child_projection`, and `TestSnapshot::visit_child_refs`
(`🧪️tests/🖥️test-app-mutations-document/🦀️.rs:30`) is `Ok(())` — it declares nothing, ever. Giving it
a real child-ref lane changes its pack, DSL and serde shape and every oracle pinned to it.

### 2.5 RE-EXPRESSED — and now red on a much sharper cause

`child_root_maintenance_requires_terminal_empty_before_reclaim` set `maintenance_stage = 4` before
every call and asserted the answer was `Pending`. `PluginApp::maintenance_step` ROTATES
(`🔌️plugin/🦀️.rs:30716`): one call walks all 26 `MAINTENANCE_STAGES`, continuing past any stage that
released nothing, so a later idle stage answers `Complete` and that is what the call returns — the
law failed on a stage the lying owner never reached. Its sibling
`child_snapshot_retirement_rejection_preserves_exact_erased_owner` already had the rotation-aware
form (`assert_maintenance_reports_block`, `:1957`).

Re-expressed to the PROPERTY, not its proxy: every turn until the fault arrives must leave
`app.child_content_retirements` POPULATED — the lying owner's authority must never be reclaimed
without its terminal witness — whatever step word the rotation returns; the loop now spans
`64 * MAINTENANCE_STAGES` turns; the fault clause
(`interactive-job.child-snapshot-terminal-not-empty`) and the final retention clause are unchanged.
This states MORE than "never `Complete`": a `Complete` that DID reclaim now fails, and a `Complete`
from an idle sibling stage correctly does not.

**What it measures now** (`fp10-round3-serial.txt`, and alone in 0.02 s):

```
the lying owner's authority must never be reclaimed without its terminal witness:
turn 2 answered Pending { released_items: 1, released_bytes: 0 } from stage 4 and left the registry empty
```

So stage 4's OWN unit reclaims the registry entry on turn 2 and the terminal witness at
`🔌️plugin/🦀️.rs:9251` never fires. The reason is visible in the store path: the lying
`TestLyingOwnedValueRetirementFactory` is only ever consulted by
`ReturnedSnapshotReadRetirement::close_step` (`🏪️store/🦀️.rs:1652`) AFTER
`Arc::into_inner(alias)` succeeds; while the child-content view still aliases that snapshot the
retirement answers `Pending`/`Complete` without ever building the lying owner, and
`ChildContentRetirement::close_step`'s `ChildContentTake::Complete` arm (`:9280`) then reclaims.
**The fixture's lie no longer reaches the path it was written for.** That is a strictly better
diagnosis than FP9 §5.6's ("the law pins one stage") and it is where the next slice starts: either
the lease must be dropped before the disposer runs, or the witness must also guard the
`ChildContentTake::Complete` reclaim.

### 2.6 DIAGNOSED, NOT LANDED — a Worker-lane fault aborts the whole publication turn

The fixture (`🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs:~845`–`900`) parks operation 1 at
`MountedTypedCommandFullOperationStage::Worker` with `session: None, terminal_seen: true` to model a
"retained first worker" the ready publisher must step past. `drive_worker_step`
(`🔌️plugin/🦀️.rs:18471`) refuses that state by name, and the refusal is CORRECT — `Worker` with no
session is unreachable in production (`:18475` is the only place `session` is cleared, and it moves
the stage to `Publishing` in the same breath).

The PRODUCT defect the law exposes is one level up: `drive_typed_operation_worker`
(`🔌️plugin/🦀️.rs:27535`) propagates that `Err` with `?`, so ONE operation's structural fault aborts
`advance_typed_operation_publication_one` for the WHOLE actor — every other mounted operation stops
advancing, permanently. The Publishing lane already has the right shape for exactly this
(`advance_typed_operation_publication_unit`, `:27726`–`:27742`: count the attempt, and past
`TYPED_OPERATION_MAXIMUM_RETRIES` mint the operation's own Fault result page), and
`fault_stalled_typed_operation_publication` (`:27681`) writes the doctrine down: a per-operation
failure becomes that operation's fault page, never the turn's fault. The Worker lane is the one lane
that does not honour it.

Landing it: catch `drive_worker_step`'s `Err`, cancel that operation's lease, mint its terminal fault
page, move it to `Publishing` so it retires, and let the turn continue. That changes what the law's
own operation 1 becomes, so its `stage == Worker` clause and its two `take_typed_operation_result_page(7)`
clauses must be re-expressed onto the stronger statement (the stuck operation never poisons the turn;
the ready publisher still publishes; the stuck one is terminated by name). Not attempted with one
build round left — a wrong guess costs 15–22 min.

## 3. Flow `retained::*` — diagnosed correctly, fix ATTEMPTED AND REVERTED

**Correction to the brief and to FP9 §5:** these six reds are NOT in `✏️s/🔌️plugins/🌊️flow/**` (the
peer session's flow topic) and the crate is not `semio-s-artifact-flow-flow`. They are in the
FRAMEWORK flow artifact crate **`semio-framework-artifact-flow-flow`**,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/**`. **No file under
`✏️s/🔌️plugins/🌊️flow/**` was touched**, so there is no overlap with that peer at all.

**FP9's second diagnosis is wrong and is corrected here.** `FlowRetirement::allocated_bytes` is
CORRECT: `exact_direct_backing`'s line 68 (`allocated_bytes() == expected_bytes`) PASSES. The law dies
one line later, at line 70, on `next_close_byte_demand()` — `owner_release_demand`
(`🧵️retained/🦀️.rs:76`) answered a flat `Ok(1)` for every direct-backing owner
(`left: 1 / right: 8193`). The multi-root law's `left: 2 / right: 31218` is the same cause plus the
frontier page, whose `released_allocation_bytes` the close step discards.

**FP9's first diagnosis (`CopyCursor` propagating a bare `Blocked`) is right, and its root is one
level deeper**: `FlowRetirement`'s `ErasedSnapshotRetirement::close_step` answers `Blocked` while it
still owes itself an allocation, and the erased view is the ONLY entry a
`Box<dyn ErasedSnapshotRetirement>` driver has — the driver cannot reserve on its behalf, so it spins
(`positive copy close grant blocked` in all four `CopyCursor` laws). `close_page` is the inherent
method that pays first, but it is not reachable through the trait.

A five-part fix was written and measured (`fp10-flow-retained-round1.txt`): `owner_release_demand`
publishing the physical demand, an atomic grant-gated `release_root_backing`
(`RootBackingRelease::{NotApplicable, Blocked, Released}`, `root_backing_credit` removed), the
frontier page reporting its bytes, the erased `close_step` paying its own allocations with
`close_page` delegating to it, and a demand-aware `CopyCursor`. Round 1 turned **both**
`flow_physical_retirement_*` laws GREEN and moved the failure set to four `CopyCursor` laws plus two
consequential ones.

**It was then REVERTED, and both files are byte-identical to HEAD again** (`git diff --stat` on
`🌊️flow/` is empty; `fp10-flow-retained-revert.txt` re-measures the exact baseline 8/6). Reason: the
last iteration made `flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages`
spin at 100 % CPU for 21 minutes instead of failing fast, and there was no build round left to find
it. **Leaving a live-locking test in the tree is worse than leaving the baseline red**, so the tree
is back at baseline.

**The real obstacle, for the successor** — the two laws are in genuine doctrinal conflict and no
change inside flow alone satisfies both:
- `flow_physical_retirement_every_direct_string_and_vec_releases_actual_capacity_once` demands the
  physical release be ATOMIC and GRANT-GATED: `close_step(1, expected-1)` must answer `Blocked` and
  `close_step(1, expected)` must report exactly `expected` bytes. A heap allocation cannot be freed
  in pieces, so this is the honest accounting, and `next_close_byte_demand` exists precisely so a
  driver can ask before it grants (`drain`, `retire_cold` and
  `flow_physical_retirement_frontier_requires_exact_admission_and_releases_metadata_last` all do).
- the four `CopyCursor` laws drive with `close(cursor, grant)` at **grant = 1** and assert
  `released_bytes <= grant`. The cursor's `active_root_retirement` is a FOREIGN
  `Box<dyn ErasedSnapshotRetirement>` that publishes no demand through the trait, so at grant 1 it
  can neither be granted enough (Blocked forever) nor report truthfully (overgrant).

The clean resolution is a **defaulted `fn next_close_byte_demand(&self) -> usize { 1 }` on
`ErasedSnapshotRetirement` (`🏪️store/🦀️.rs:1622`)**, so a retirement that needs a minimum grant can
say so and a nested driver can pay it out of its own allocation admission rather than the caller's
payload page — which is literally what
`flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages` is named
after. That trait is KN3's file this session, so it was deliberately not touched; the doubling-offer
workaround written instead is what live-locked.

## 4. `♾️infinite/🕸️dag` — FIXED

`vcs::dag_vcs_tests::dag_demo_ownership_matches_neutral_graph_identity` failed on
`String("dag.fixture")` vs `String("dag.hostDocument")`. `DagHostSnapshot::default()`
(`🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:626`) parses the bundled demo DSL, and that asset
(`🕸️dag/🖼️assets/🎬️demo/🗣️.dsl.semio:2`) declared `schema=dag.fixture` while the DAG's OWN JSON
Schema (`🕸️dag/🧬️schema/🔣️.json:32`) declares `"schema": { "const": "dag.hostDocument" }`, and the
package-contract fixture (`🧫️fixtures/📦️package-contract/📜️cases.json:6`) and the neutral identity
fixture both say `dag.hostDocument`. The default document was schema-INVALID against its own
contract; the asset was the single outlier and is corrected. `bundled_demo_fixture_is_canonical`
compares `DAG_DEMO_TEXT` to `print_dsl(default_dag_document())` and the default is parsed FROM the
asset, so it stays consistent. Grepped: no `"dag.fixture"` string consumer remains anywhere in the
tree. **Measured: `cargo test -p semio-framework-artifact-infinite-dag --lib` → 58 passed, 0 failed**
(`fp10-dag-round1.txt`; was 57/1).

## 5. Files changed

Product code (1 file):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🖼️assets/🎬️demo/🗣️.dsl.semio` —
  `schema=dag.hostDocument` (§4)

Laws / fixtures (2 files):
- `🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs` — new region `🧪️SurfaceRegisteredFactory`
  (`SURFACE_CONTROLLER_ID`/`SURFACE_TOOL_ID`/`SURFACE_PAYLOAD_SCHEMA`/`SURFACE_TOOL_CONTRACT`,
  `SurfaceFixtureJob`, `SurfaceFixtureFactory`, `surface_manifest`), `OpBinary::TOOL_JOB_IDS`,
  `ArtifactEditor::command_id`, `bounded_first_step_tool_proofs!`, `register_tool_job_factories`,
  `build_tool_job`, `build_artifact_store_one_item_preparation_factory`, the law moved onto the
  registered app with an explicit settle, and two new laws (§2.1)
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` —
  `child_root_maintenance_requires_terminal_empty_before_reclaim` re-expressed onto reclamation, with
  a turn/stage diagnostic (§2.5)

Reverted to HEAD after measurement (§3): `🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` and
`🧵️retained/📑️copy/🦀️.rs`.

Captures (`🗑️generated/`): `fp10-round0-serial.txt`, `fp10-round1-check.txt`, `fp10-round1-build.txt`,
`fp10-round1-serial.txt`, `fp10-round2-build.txt`, `fp10-round2-serial.txt`, `fp10-round3-build.txt`,
`fp10-round3-serial.txt`, `fp10-bin-path.txt`, `fp10-dag-round1.txt`,
`fp10-flow-retained-round0.txt`, `fp10-flow-retained-round1.txt`, `fp10-flow-retained-round2.txt`,
`fp10-flow-retained-revert.txt`.

## 6. Honest gaps

- **The gate is NOT green: 5 reds remain** (819 / 5). One of the six is fixed; three of the remaining
  five (§2.2, §2.3, §2.6) are PRODUCT features/defects with the exact code site named, one (§2.4) is
  a fixture whose pack/DSL/serde shape would have to change, and one (§2.5) is re-expressed and now
  fails on a sharper, newly-named cause.
- **Of the four lane items the slice was given, one landed** (the editor fixture's real owned
  factory). The task lane (§2.2) and the composed settle lane (§2.3) are diagnosed to the line but
  NOT implemented — both need retained-state or wasm-gated changes that cannot be validated in one
  build round. The flow lane was implemented, measured, and reverted (§3). The DAG red is fixed (§4).
- **No law was `#[ignore]`d, deleted or loosened, and no ceiling constant moved anywhere.** One law
  was re-expressed (§2.5) onto a strictly stronger statement; the old
  `editor_fixture_still_mutates_normally`'s real content (the fail-closed refusal) was preserved as
  its own new law rather than dropped (§2.1).
- **`🏪️store/🦀️.rs` was NOT touched** (KN3 owns it this session) — §3 names the one additive change
  there that would unblock the flow lane.
- **No file under `✏️s/🔌️plugins/🌊️flow/**` was touched** (the peer session's flow topic).
- **One order-dependent flake observed, not caused by this slice:**
  `an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving` failed once in the
  round-2 serial run (`instance busy or poisoned: 4024`) and PASSES alone with `--exact`
  (0.19 s); it did not recur in round 3. It is process-wide instance state shared across the serial
  suite, the same class as FP9 §3.9's thread-local patch authority.
- **`semio-framework-plugin` gained no product behaviour this slice**, so no dependent plugin crate
  needed a `--features component-app-assembly` re-check. The DAG asset change is data, verified by
  that crate's own suite.
- Every number in this report is read from a capture in `🗑️generated/`. Nothing is claimed that was
  not executed.
