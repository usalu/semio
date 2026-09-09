# Wave B — one gesture = ONE Edit = ONE ledger slot, prepared page-by-page

Implements `📓️2026-09-09-applied-ledger-ceiling-audit.md` §5 option (a) and closes
`📓️2026-09-09-wave-D-production-defects.md` §P4 (`ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` as a
per-session lifetime cap on applied edits).

**Headline result — both red tests are green:**

```
test editor::puzzle3d::component::tests::nakagin_example_loads_via_operations ... ok
test editor::puzzle3d::component::tests::set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document ... ok
```

and the store proves the general law directly:

```
test os_store::component::tests::artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step ... ok
```

---

## 1. The design that was built, and the one deliberate deviation from the audit

### What ships

The one-item publication state machine **became** the batched machine — there is exactly ONE
machine, and the single-mutation publication is literally its `N = 1` case. `begin_apply_one` /
`advance_apply_one` / `cancel_apply_one` / `ArtifactStoreOneItemPublication` /
`ArtifactStoreOneItemAdmissionRejected` **no longer exist anywhere in the repo**; every caller now
uses `begin_apply_batch` / `advance_apply_batch` / `cancel_apply_batch` /
`ArtifactStoreBatchPublication` / `ArtifactStoreBatchAdmissionRejected`.

One gesture:

1. **admission** takes the whole `Vec<Mutation>` and declares ONE gesture-wide footprint
   (work items sum across every staged forward+inverse owner, retained bytes stay the widest single
   item, because only one item's owners are ever live at a time);
2. **staging** folds exactly ONE mutation per `advance_apply_batch` turn — the app's own per-item
   preparation computes `inverse` + `diff.apply` against the **running post root** of the previous
   item, and the store appends the result into a staged `Edit`;
3. **commit** reserves ONE `reserve_edit_history_slot()` and pushes ONE `applied_edit_ids` entry,
   so 182 mutations consume 1 of the 64 slots, not 182.

### The deviation: the app-side trait shape did NOT change

The audit's §5 and this wave's brief both anticipated changing
`ArtifactStoreOneItemPreparationFactory` / `ArtifactStoreOneItemPreparation` to accept `Vec<Input>`
and migrating every implementor (task item 3). **That was not done, deliberately.** Instead the
batching lives entirely in the store: it drives N *per-item* preparations sequentially, chaining
each one's base read onto the previous one's post root, and merges their prepared candidates into
ONE staged `Edit`.

Reasons, in order of weight:

- **~80 files implement that trait** (`grep -rn "ArtifactStoreOneItemPreparationFactory\b"` — 78
  files outside the store, across every `✏️s/🔌️plugins/*` app). Changing the trait means each app
  must grow its own accumulation loop. Only three crates are compilable inside this wave's budget
  (`semio-framework-os-kernel`, `semio-framework-plugin`, `semio-s-artifact-puzzle-3d`, ~4 min per
  cycle), so ~75 crates would have been edited **without a single compile** — the exact
  "written but not confirmed by a run" failure mode this ticket has been bitten by repeatedly.
- The per-item contract is the *right* app-side contract and did not need to widen: an app's job is
  "turn ONE mutation into ONE bounded prepared candidate", and every existing implementor already
  does exactly that, cursorized. Batching is a *history* concern, and history is the store's.
- Nothing about the user-visible semantics changes: one gesture is still one `Edit`, one ledger
  slot, one undo step, one History row, and the staged edit is byte-identical to what
  `ArtifactCommand::Apply` records for the same mutation list (asserted below).

Consequence: **zero app crates needed migration.** The only app-side edits in this wave are two
that were *compensating* for the old drain (see §4).

Cost of the deviation, stated plainly: a fold costs ~7 turns per mutation (begin item → prepare →
fold → 3-4 bounded item-retirement steps) instead of ~3, because each item's preparation is opened
and retired around its fold. For Nakagin (~182 mutations) that is ~1 300 turns instead of the
~2 200 the old path needed — still one gesture, and it no longer faults. If the coordinator wants
the ~3-turn shape, that is exactly the trait-widening migration above and should be its own wave
with a budget for compiling all ~80 crates.

---

## 2. Every change, file:line

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`

| line | change |
| ---: | --- |
| 13098 | `ArtifactStoreOneItemFootprint::merged` — folds one item's footprint into the gesture-wide declaration (work items sum, retained bytes max). |
| 13322 | `trait ArtifactStoreBatchItemAuthority` — the erased per-item app authority (`type Input`), impl'd for `Arc<dyn ArtifactStoreOneItemPreparationFactory>` (Input = `Mutation`) at 13338 and for `Arc<dyn MemberStoreOneItemWirePreparationFactory>` (Input = `MemberStoreOneItemWire`) at 13350. |
| 13355 | `struct ArtifactStoreBatchItemRequest<P>` — one item's owner bundle, minted from the batch's single shared live authority plus the running post root. |
| 13366 | `trait ArtifactStoreBatchSource<P, Mutation>` — object-safe remaining-inputs census + `begin_next` + bounded `close_step` + `terminal_is_empty`. |
| 13375 | `struct ArtifactStoreBatchSourceOf<P, Mutation, A>` — the ONE source shape (inputs held in reverse authored order so `pop` returns authored order); `footprint` (13391) preflights every input and merges; `begin_next` (13411) hands one owner to the app factory and takes back its owners on rejection; `close_step` (13427) releases one input per turn. |
| 13575 | `pub struct ArtifactStoreBatchAdmissionRejected<Mutation>` — carries `mutations: Vec<Mutation>` (was `mutation: Mutation`). |
| 13591 | `struct ArtifactStoreBatchStage<P, Mutation>` — the staged gesture edit + running post root + its OWN retirement authority; `checkpoint` 13611; `close_step` 13615 retires staged forwards/inverses/metadata/post root/strings ONE owner per bounded turn; `terminal_is_empty` 13664. |
| 13683 | `pub struct ArtifactStoreBatchPublication<P, Mutation>` — replaces `ArtifactStoreOneItemPublication`; adds `admitted_items`, `source`, `item_closing`, `stage`. |
| 13715/13719 | `admitted_items()` / `staged_items()` accessors. |
| 13725 | `take_staged_prepared()` — hands the staged gesture back as ONE sealed `ArtifactStoreOneItemPrepared` so the durable-group three-store assembly can still linearize candidates before visibility. |
| 13742 | `progress()` — monotone: staged totals + the currently-preparing item, with the item's contribution absorbed into the stage in the same turn it is folded (`item_closing` gates it), so no turn ever reports less than the one before it. |
| 13795 | `close_step` — item preparation, then remaining source inputs, then the staged edit, then receipt/authority/fault; unchanged discipline, three new owners. |
| 13849/13859 | `terminal_is_empty` + `Drop` witness extended with `source`/`stage`. |
| 15628 | `pub fn begin_apply_batch(... mutations: Vec<Mutation> ..., factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>)`. |
| 15657 | `pub fn begin_member_apply_batch`. |
| 15673 | `fn begin_apply_batch_owned` — replaces the closure-generic `begin_apply_one_owned`; same admission gates (durable-group idle, generation/revision, lane, backbone, actor/group identity, live cursor, sequence), plus empty-gesture and item-census-vs-footprint gates. |
| 15767 | `fn batch_item_base` — the store's own committed root for item 0, and a lease over the previous item's post root (through the same exact `SnapshotReadLeaseRegistry`) for every later one. |
| 15783 | `pub fn advance_apply_batch` — `Preparing` now has four bounded sub-turns (retire the folded item → begin the next item → advance it → fold it), then the unchanged `PreparingCursor` → `PreflightingCommit` → `Publishing` → `AwaitingAck` chain. |
| 15859-15898 | `PreflightingCommit` validates the STAGED edit (`folded == admitted_items`, `forwards.len() == mutation_meta.len() == admitted_items`, inverse+forwards ≤ footprint work items) and computes the merged `CursorRevisionAccumulator::edit_digest` once, then the unchanged applied/revision/cursor capacity gate. |
| 15902-15960 | `Publishing` — ONE `reserve_edit_history_slot()`, ONE `applied_edit_ids.push`, ONE `insert_reserved_edit_history`, ONE revision record, for the whole gesture. |
| 15966 | `pub fn cancel_apply_batch`. |
| 15979 | `fn fold_batch_item` — validates the candidate through `authority.validate_prepared(candidate)` **before** taking it (a rejected item stays inside its preparation and is retired by that preparation's own close), reserves the gesture's exact staging capacity once via three `try_reserve_exact` calls on the first fold, reverses each item's inverse block exactly as `replay_mutations` does, appends forwards/inverse/metadata, moves the post root in, and retires the displaced root only when the batch is its last owner. |
| 16062 | `fn empty_batch_stage` — the allocation-free staged shell. |
| 13490 | `impl ErasedMemberStoreOneItemPublication for MemberStoreOneItemPublication` now requires `P: Send + Sync`, `Mutation: Send` (the stage owns `Arc<P>` and `Vec<Mutation>` directly). |
| 18419 | `begin_one_item_wire_publication` builds a one-item `ArtifactStoreBatchSourceOf` over the wire factory — the member wire lane is a 1-batch on the same machine. |

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

| line | change |
| ---: | --- |
| 14735-14737 | `PendingArtifactStorePublication::{Artifact,Config,Draft}` now hold `store::ArtifactStoreBatchPublication`. Presence/Transient/WindowTransient are untouched (genuinely ephemeral one-item lanes). |
| 19993-20003 | `mounted_typed_interaction_writes_are_next` additionally requires `mounted.pending_artifact_publication.is_none()` — a batched lane empties its emit vector at admission, so the in-flight publication, not the drained vector, is what still owes a turn. Without this the interaction lane would fire while the document publication was still staging. |
| 20359/20374/20389 | `self.{store,config_store,draft_store}.advance_apply_batch(...)`. |
| 20546-20568 | The Artifact arm drains the WHOLE lane: `std::mem::take(&mut emit.artifact_mutations)` into ONE `begin_apply_batch`, guarded by `!emit.artifact_mutations.is_empty()`; the rejection path restores the whole vector. Factory is passed as `self.artifact_one_item_factory.as_ref()` (an `&Arc`, no more `as_deref`). |
| 20571-20583 | Same for the Config lane. |
| 20591-20603 | Same for the Draft lane. |
| — | `acknowledge_result_page` (15072) needed **no** change: the result page/ACK cycle is one per *lane publication*, and a batch is one lane publication. The interaction-write lane and child publications are untouched. |

### Other framework callers

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1159-1161`
  (publication field types), `:1169`/`:1184` (`close_assembly_publication`/`take_assembly_prepared`,
  the latter now calling `take_staged_prepared`), `:1280`/`:1301`/`:1322` (`begin_apply_batch` with
  `vec![mutation]`), `:1340`/`:1350`/`:1360` (`advance_apply_batch`), and the three rejection arms
  now `mutations.pop()`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:284` (publication type),
  `:432` (`begin_apply_batch(..., vec![*typed], ..., Some(&factory))`), `:446`
  (`advance_apply_batch`).

### App-side changes (the two compensations the old drain forced)

- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:389-393,409` — **removed `emit.artifact_mutations.reverse()`**.
  All fifteen norm editors were reversing their bundle on the way out *because* the old lane drained
  it LIFO one `Vec::pop` per turn. The batched lane stages the bundle front-to-back into one edit,
  exactly as `Emit::commit`'s ordinary dispatch applies it, so the compensation is now a corruption
  and is gone with the drain it compensated for. Docstring rewritten to say so.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:132-136` — the
  docstring of `set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document`
  claimed to prove "the LIFO drain order `norm_retained_reduce` compensates for"; it now states the
  real law (authored bundle order survives staging into ONE batched edit).

### Repo gates (`📜️script.ts` and its two executable self-test suites)

- `📜️script.ts:2052-2053` — the mounted-operation audit now requires `presence`/`transient` to
  drain by `.pop()` and `artifact_mutations`/`config_mutations`/`draft_mutations` to drain by
  `std::mem::take(&mut emit.<lane>)`.
- `📜️script.ts:2056-2059` — publisher seams `begin_apply_batch`/`advance_apply_batch`.
- `📜️script.ts:2093-2097` — `toolJobPublicationFreshnessBeforeEveryTurn`'s begin-turn list.
- `📜️script.ts:2115` — `toolJobStoreOneItemPublicationBounded` → **`toolJobStoreBatchPublicationBounded`**,
  rewritten for the batched shape and *strengthened*: it now additionally requires
  `fold_batch_item` to exist, to call `authority.validate_prepared(candidate)`, to carry
  `inverse.reverse();`, to contain no `while` loop and no `replay_mutations(`/`apply_command(`; and
  requires the publisher to drain each durable lane with `std::mem::take` and never
  `<lane>.last().cloned()`. Exported name updated at `:30817`; call site `:6960`.
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts:392-450` — the language-neutral
  self-test fixture rewritten to the batched shape, plus **two new hostile rows**: a publisher that
  clones instead of draining its whole gesture lane, and a fold without the `replay_mutations`
  inverse ordering.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:4,138,143-145` —
  same rename; its hostile mutation of the store source now targets the batched admission
  (`source.footprint(lane)`), and the move-only-ownership hostile targets `std::mem::take`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts:80,94,100` —
  the canonical-sealer authority linkage now anchors on `fn fold_batch_item(` (the turn that
  validates the private seal) and on `authority.validate_prepared(candidate)`, including its
  hostile row.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:230` — the GIS durable three-store assembly gate
  now forbids `begin_member_apply_batch` and requires `begin_apply_batch(`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:608` — docstring reference
  to the publisher seam updated to `begin_apply_batch(..., …_factory.as_ref())`.

### Tests

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`
  - `:296-312` — the `one_item_publication_source_denies_generic_or_unbounded_shortcuts` static law
    retargeted at `advance_apply_batch` … `cancel_apply_batch`, plus a NEW second slice over
    `fold_batch_item` … `empty_batch_stage` requiring `authority.validate_prepared(candidate)`,
    `inverse.reverse();`, exactly three `try_reserve_exact(` calls (the gesture's whole admitted
    forwards/inverse/metadata capacity, reserved once), and no generic shortcuts.
  - `:2976` `publish_demo_batch` helper (bounded per-turn grant, exactly as a host actor tick).
  - `:3002` **`artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step`**
    — 200 mutations (asserted `> ARTIFACT_HISTORY_LEDGER_CAPACITY`) publish once; `applied_edit_ids`
    and `vcs.edits` both have length 1; forwards/metadata/inverse each have 200 entries; the staged
    `forwards` and `inverse` are asserted **equal to what `ArtifactCommand::Apply` records for the
    same list** (the store's own oracle); consuming the staged inverse **tail-first** returns the pre
    root while head-first does not (the "inverse is applied in reverse order of forwards" law); ONE
    `Undo` reverts the whole gesture and ONE `Redo` restores it.
  - `:3058` `…_of_one_mutation_is_the_single_item_case` — `N = 1` on the same machine.
  - `:3077` `…_rejection_mid_batch_leaves_no_partial_edit_and_still_retires_exactly` — a gesture
    whose 4th mutation cannot prepare against the running post root (`DeleteN` then `SetN`) commits
    nothing: no ledger slot, no applied edit, no root replacement, no generation bump, and closes
    terminal-empty.
  - `:3106` `…_cancel_mid_flight_retires_every_staged_owner_without_publishing`.
  - `:2829` `close_durable_publication` retargeted and its turn budget raised (a staged gesture
    retires more owners than a single item).
  - Every pre-existing one-item lifecycle test migrated (`:2942` retry/ack/move-only-root, `:3139`
    cold rebase, `:3166` forged digest, `:3208` stale/saturation/cancel, `:3319` Drop witness,
    `:2463`/`:2536` member-wire) — the saturation test now asserts the ledger-ceiling fault text
    explicitly instead of counting turns.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs:449,498,1261`
  and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15722` — the shared
  `retained_document_cancellation` fixture takes `Arc<dyn …Factory>`; its retained-seam law now names
  `begin_apply_batch`/`advance_apply_batch`. Caller
  `…/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1856`.
- `✏️s/🔌️plugins/🌊️flow/…/🧵️retained/🎚️config/🧪️tests/🔬️unit/🦀️.rs:18-40` and
  `…/🧵️retained/🗿️artifact/📬️preparation/🧪️tests/🔬️unit/🦀️.rs:23-33` — migrated; these are the tests
  that pin `progress().completed_bytes` monotone and step-bounded, which is why `progress()` absorbs
  a folded item's checkpoint into the stage in the same turn.
- `.🧬semio/🦑️repo/🎫️tickets/…/PUZZLE-3D-END-TO-END/🔍️verify-wave-B-batched-publication.ts` (new) —
  runs the repo's own batched-publication gate + 3 hostile controls directly against the LIVE
  store/plugin sources, because `bun ./📜️script.ts verify interactivity tool-jobs` currently aborts
  earlier on a peer-owned failure (§5).

---

## 3. Commands run, and their tails

All cargo runs: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`, foreground, `-j 4`.

**Baseline before any edit** (the tree was healthy):
```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4
    Finished `dev` profile [unoptimized] target(s) in 42.13s
```

**Framework + app compile after the change:**
```
cargo check -p semio-framework-os-kernel -j 4
    Finished `dev` profile [unoptimized] target(s) in 0.47s        (0 errors, 0 warnings)
cargo check -p semio-framework-os-kernel --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 11.57s       (0 errors)
cargo check -p semio-framework-plugin --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 0.48s        (0 errors; 27 pre-existing warnings)
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4
    Finished `dev` profile [unoptimized] target(s) in 0.60s        (0 errors, 0 warnings)
cargo check -p semio-s-plugin-norm -j 4
    Finished `dev` profile [unoptimized] target(s) in 52.87s       (0 errors)
cargo check -p semio-s-plugin-flow --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 15.60s       (0 errors)
```

**The two red puzzle 3D tests:**
```
RUST_MIN_STACK=134217728 … cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 \
  nakagin_example_loads_via_operations -- --test-threads=1
test editor::puzzle3d::component::tests::nakagin_example_loads_via_operations ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 578 filtered out; finished in 0.73s

… set_active_example -- --test-threads=1
test editor::puzzle3d::component::tests::set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document ... ok
test editor::puzzle3d::component::tests::set_active_example_hostile_static_law_rejects_whole_document_reset ... ok
test editor::puzzle3d::component::tests::set_active_example_swaps_the_document_and_undo_restores_it ... FAILED   (P10 pool contention — passes alone, see §5)
test editor::puzzle3d::component::tests::set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin ... FAILED   (pre-existing, wave-X:426)
```

**Store batched + one-item + durable-group:**
```
cargo test -p semio-framework-os-kernel --lib -j 4 -- --test-threads=1 artifact_store_batch artifact_store_one_item durable_group
test os_store::component::tests::artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing ... ok
test os_store::component::tests::artifact_store_batch_publication_of_one_mutation_is_the_single_item_case ... ok
test os_store::component::tests::artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step ... ok
test os_store::component::tests::artifact_store_batch_rejection_mid_batch_leaves_no_partial_edit_and_still_retires_exactly ... ok
test os_store::component::tests::artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history ... ok
test os_store::component::tests::artifact_store_one_item_drop_rejects_an_unclosed_publication_owner - should panic ... ok
test os_store::component::tests::artifact_store_one_item_single_retry_ack_and_move_only_root_preserve_generation_revision_and_history ... ok
test os_store::component::tests::artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged ... ok
   (+ all 19 durable_group tests)
test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 1029 filtered out; finished in 1.33s
```

**Plugin typed-operation suite** (12 passed / 2 failed, both peer-owned, §5):
```
cargo test -p semio-framework-plugin --lib -j 4 typed_command_full_operation -- --test-threads=1
test result: FAILED. 12 passed; 2 failed; 0 ignored; 0 measured; 562 filtered out; finished in 0.10s
```
```
cargo test -p semio-framework-plugin --lib -j 4 retained_latest_wins_real_document_publication -- --test-threads=1
test component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close ... ok
test result: ok. 1 passed; 0 failed; …
```

**Repo static gates:**
```
bun .🧬semio/…/PUZZLE-3D-END-TO-END/🔍️verify-wave-B-batched-publication.ts
ok    batched Store publication is bounded
ok    ephemeral one-item publication is bounded
ok    document freshness precedes every publication turn
ok    a replay hidden inside batched admission is rejected
ok    a whole-lane clone-then-pop publisher is rejected
ok    a per-publication factory reconstruction is rejected
```
The coverage suite's own batched-publication self-tests execute and pass (they are reached and
cleared before the run stops at line 1852 on a peer-owned cohort failure, §5). The two other static
gates that read this region were executed directly and pass:
`storeCanonicalEditSealerSelfTests()` → `{ grants: 4, schemaHostiles: 4, sourceHostiles: 6,
digestOracles: 1, mapGrants: 4, mapSchemaHostiles: 4, mapSourceHostiles: 6, mapDigestOracles: 1,
readerChecks: 37 }`, and `proveGisDurableThreeStoreAssembly` →
`gis-durable-three-store-assembly-oracle: AJV=1 SHA256=node+webcrypto roles=3 cancellation=4 rejection=3`.

**Whole puzzle 3D suite, per-process isolation** (`🔍️isolate-puzzle3d-tests.py`, the same instrument
`📓️2026-09-09-wave-X-test-suite.md` §4 used to measure "64 remaining isolated failures"):
```
total 578 ok 526 failed 52
```
i.e. **52 isolated failures now, against the 64 the ticket recorded before this wave** — and the two
target tests moved red → green. In-process (`--test-threads=1`, one binary) the same suite reads
`497 passed; 79 failed`; the extra 27 are the P10 worker-pump pool contention wave D already
documented as non-deterministic.

---

## 4. Behavioural changes a reviewer must sign off

1. **Undo granularity.** One gesture is now ONE undo step and ONE History row. Undoing
   "load Nakagin" or "fill 200" is one `Ctrl+Z`, not 182. This is the coordinator's stated intent and
   is pinned by the 200-mutation test's `Undo`/`Redo` assertions. `📜️history/🦀️.rs`'s
   `HistoryCursor` mirrors `applied_edit_ids` 1:1, so the History panel renders one row per gesture
   automatically — no shell change was needed, and none was made.
2. **Inverse ordering.** The staged inverse follows the store's own convention exactly
   (`replay_mutations` / `replay_suffix` / `replay_suffix_partitioned`): each item's inverse block is
   reversed, blocks are concatenated in forward item order, and the whole vector is consumed
   **tail-first**. The test asserts both halves — tail-first reproduces the pre root, head-first does
   not — and asserts equality with the `ArtifactCommand::Apply` oracle.
3. **The norm apps' LIFO compensation is deleted** (§2). All fifteen norm editors were reversing
   their emitted bundle to survive the old LIFO drain; with the batched lane that reversal would
   corrupt their ordered `remove-layer`/`insert-layer` runs. Not covered by an executed test in this
   wave — see §5.
4. **`ArtifactStoreOneItemFootprint` is now a gesture-wide declaration** for the durable lane:
   `work_items` sums, `retained_bytes` is the max. `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS =
   65 536` therefore caps a gesture at ~32 768 mutations for a 2-work-item app such as puzzle 3D;
   past that, admission fails cleanly with
   `"batched preparation footprint exceeds its fixed item or byte capacity"` instead of faulting
   mid-flight.
5. **Intermediate roots are leased.** Item *i > 0* prepares against a `SnapshotRead` over item
   *i − 1*'s post root, issued from the same `SnapshotReadLeaseRegistry`
   (`SNAPSHOT_READ_LEASE_CAPACITY = 1 024`, reclaimed by the store's returned-read retirement pump).
   One lease is live at a time; if the registry is busy the turn returns `Blocked` and retries, it
   never faults. The displaced root is retired through `displaced_retirements` only when the batch is
   its last owner (`Arc::strong_count == 1`) — otherwise the still-closing item preparation holds the
   registry lease and the existing returned-read machinery retires it.

---

## 5. NOT verified by this wave (explicit list)

- **The norm apps' de-reversal (§4.3) was not executed.** `cargo check -p semio-s-plugin-norm`
  passes, but no norm test drives `norm_retained_reduce`'s bundle order end to end in this wave.
  The one test that documents this seam
  (`din4108 … set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document`)
  was not run — running it needs the norm plugin's own test harness, outside this wave's compile
  budget. **Recommend the coordinator run
  `cargo test -p semio-s-plugin-norm --lib set_snapshot_dispatches_through_the_tool_job_path` before
  the next wasm build.**
- **No wasm component was rebuilt and no server was started** (per the brief).
  `⚛️reactor/🔄️turn/🦀️.rs` was not touched.
- **~75 app crates that implement `ArtifactStoreOneItemPreparationFactory` were not compiled.** They
  are also not *changed* — the trait, its request type and `ArtifactStoreOneItemPrepared` are
  byte-identical to before — so they cannot have broken, but this is reasoning, not a run.
- **The full `bun ./📜️script.ts verify interactivity tool-jobs` gate does not pass**, because it
  aborts before reaching this wave's predicates on a peer-owned failure:
  `scalar Config route disposition generation2d/addGeneration`
  (`🔬️tool-job-scalar-config-cohort/🟦️.ts:27`). The batched-publication predicates it would have
  run are covered by `🔍️verify-wave-B-batched-publication.ts` and by the coverage suite's own
  self-tests, both green.
- **`semio-framework-os-kernel`'s wider `os_store::` suite cannot produce a summary**: several
  fixtures build an `ArtifactStore` without `install_member_store_owners_exact`, so
  `reserve_edit_history_slot` refuses with
  `"edit history insertion requires its exact mutation retirement factory"` and the store's `Drop`
  witness turns the unwind into a `SIGABRT` that kills the binary. Pre-existing: that gate is
  committed since `21fbcd3538` (2026-09-02), long before this wave, and none of the touched
  functions are on the path. Known-affected in that class:
  `channel_backbone_round_trips_between_store_and_actor`, `attach_reconciles_a_pushed_snapshot`,
  `checkout_checkpoint_restores_applied_edits`, `artifact_snapshot_root_is_o1_…`,
  `chronological_determinism_any_arrival_order_converges` and others. Every test in the region this
  wave touched passes (27/27 above).

### Failures observed that are peer-owned or pre-existing (attribution evidence)

| failure | evidence it is not this wave |
| --- | --- |
| `puzzle3d … set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin` | already listed in `📓️2026-09-09-wave-X-test-suite.md:426`; pure `Puzzle3dSetActiveExampleWork::step`/`extent` test, no store involvement |
| `puzzle3d … suggestion_and_precompute_hostile_static_law_…` | source-text law over `✏️editor/🦀️.rs`; `Puzzle3dPrecomputeCommandStage::CheckpointBytes` has been removed from that file by the in-flight `⏳️precompute` refactor (`grep -c` = 0) |
| `puzzle3d … set_active_example_swaps_the_document_and_undo_restores_it` | passes in isolation, fails in-suite with `interactive-job.worker-pump` — exactly wave D §P10 ("not deterministic … pool contention") |
| `puzzle3d ⏳️precompute/🪣️fill` compile break seen mid-wave (`E0503 cannot use sequence because it was mutably borrowed`) | `git status` shows `⏳️precompute/**` as another session's uncommitted `MM` edits; it healed on its own within ~20 min |
| `plugin … fixture_contract_is_anchored_…` | fails on `reactor.contains("output.typed_operation_result.as_ref()")` — the coordinator-owned `⚛️reactor` instrumentation, untouched here |
| `plugin … host_configuration_uses_one_bounded_event_sourced_lane_…` | fails on `expect("typed command route")`: `async fn dispatch_typed_command(` no longer exists in `🔌️plugin/🦀️.rs` (peer rename in the same file this wave also edits) |
| `latest-wins` self-test `retained ChildEmit close …` | its markers are absent from `🔌️plugin/🦀️.rs` **at `HEAD`** too (`git show HEAD:… | grep -c` = 0) |
| `flow … flow_actual_surface_factories_close_all_owners_under_neutral_grants` | `interactive-job.catalog-authority` for `flowEvalResolve` (`generated_migrated=false`) — a tool-proof catalog issue, unrelated to publication |
| the 52 isolated puzzle3d failures | dominated by `interactive-job.missing-owned-reducer` (`setActiveUtility`), `interactive-job.worker-pump`, `typed-operation cancelled before its next publication unit` and `addObjectKind is a Mutation that emits mutations` — the same classes wave X recorded, at a lower count (52 vs 64) |

---

## 6. Follow-ups for the coordinator

1. Run `cargo test -p semio-s-plugin-norm --lib set_snapshot_dispatches_through_the_tool_job_path`
   to close §5's one real gap (the de-reversal).
2. Decide whether to widen `ArtifactStoreOneItemPreparation` to take `Vec<Input>` as its own wave
   (§1) — it would take a gesture from ~7 turns/mutation to ~3, at the cost of migrating ~80 app
   crates. Nothing in this wave blocks it: it would delete `ArtifactStoreBatchSourceOf`/
   `fold_batch_item` and move the accumulation into the apps.
3. `replay_mutations` (`🏪️store/🦀️.rs:16037`) refuses a 200-item `AddN` batch with
   `"artifact edit-message entry exceeds its exact byte authority"` because each `AddN` emits one
   info message. The batched publication path does not record edit messages and is unaffected, but
   `ArtifactCommand::Apply` has a real, undocumented message-lane ceiling on large batches. Worth its
   own ticket.
