# Applied Ledger Ceiling Audit — read-only, file:line evidence only

Scope: whether `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` is a per-session lifetime cap or has
in-use compaction, who introduced it and why, whether a multi-mutation single-edit path already
exists, undo/redo semantics, and a recommendation with exact touch points. No cargo run, no
source edits, no git state changes.

## 1. Is 64 a lifetime cap, or is there in-use compaction?

**It is a lifetime cap on the live `ArtifactStore` instance.** Every edit ever *created* while the
store is open (never mind undo) permanently consumes one of the 64 slots in
`envelope.vcs.edits: ArtifactHistoryLedger<Edit<Mutation>>`. Nothing in the normal command surface
(`Apply`, `AmendLast`, `Undo`, `Redo`, `CommitCheckpoint`, the one-item tool path) ever frees a
slot from that ledger. The only code that ever pops a slot from `vcs.edits` is the store's
close/dispose teardown.

Evidence:

- The generic free-list primitive exists and genuinely reclaims slots — `ArtifactHistoryLedger::remove_key`
  (`🌿️vcs/🦀️.rs:517-551`) pushes the freed index onto `self.free_head` (`:548`), and
  `reserve_slot` (`:335-351`) prefers `free_head` over growing `slots` (`:339-346`) — so the
  ledger type itself is *capable* of compaction/reuse if something calls `remove_key`/`pop` on it
  mid-session.
- Nothing does, in normal operation. Every call site of `.vcs.edits.pop()` /
  `envelope.vcs.edits.pop()` is inside store-close/teardown machinery, not command dispatch:
  - `🏪️store/🦀️.rs:14850-14867` `close_take_last_history_edit_retirement` — pops one edit,
    invoked only from `ArtifactStoreCursorDisposerPhase::HistoryEdits` (`🏪️store/🦀️.rs:1918`),
    itself only reached from the store's cursor-disposer state machine used at store close.
  - `🏪️store/🦀️.rs:1202` and `:1365` — inside `ArtifactVcsRetirement`/displaced-envelope
    retirement Drop-safe teardown (`SnapshotRetirementStep` state machines), not command dispatch.
  - `grep -na "vcs\.edits\.pop\|vcs\.edits\.remove_key"` across `🏪️store/🦀️.rs` returns **only**
    these teardown sites — no hits inside `apply_command`, `amend_command`, `begin_apply_one`,
    `advance_apply_one`, `undo_lane_position`, `redo_lane_position`, or `CommitCheckpoint`.
- `undo_lane_position` (`🏪️store/🦀️.rs:15830-15847`) and `redo_lane_position`
  (`:15854-15869`) only move a `String` id between `self.applied_edit_ids` and
  `self.redo_edit_ids` (both plain `Vec<String>`, pre-reserved at capacity 64 —
  `applied_edit_ids: std::mem::ManuallyDrop<Vec<String>>` at `:13581`, filled from
  `ArtifactStoreInitializationOwnerCatalog::try_new()`'s `try_reserve_exact(capacity)` at
  `:12411`). **The underlying `Edit` stays in `vcs.edits` the whole time** — undo does not call
  `vcs.edits.remove_key`/`pop` anywhere. So undoing edits shrinks `applied_edit_ids.len()` (which
  gates the one-item path's own capacity check, see §3) but does **not** free a `vcs.edits` ledger
  slot, and therefore does not restore the ability to *create new* edits once `vcs.edits` itself
  is full.
- `CommitCheckpoint` (`🏪️store/🦀️.rs:15699-15742`) bundles pending applied edit ids into a
  `Change` (`vcs.changes` ledger, its own separate 64-slot ledger) and a `Checkpoint`
  (`vcs.checkpoints` ledger, also separate/64), but never touches `vcs.edits` or
  `applied_edit_ids`/`redo_edit_ids`. Checkpointing is not compaction of the edits ledger.
- No "compact"/"retire"/"snapshot retirement" identifier in `🌿️vcs/🦀️.rs` or `🏪️store/🦀️.rs`
  refers to shrinking a *live* `vcs.edits` ledger. `grep -na "compact"` in both files returns
  nothing; `grep -na "retire\|retirement"` returns only the Drop-safe teardown machinery discussed
  above (`ArtifactStoreEditRetirement`, `ArtifactVcsRetirement`, `retire_document_envelope`, the
  `ArtifactStoreCursorDisposerPhase` state machine) — all reachable only when the *whole store or
  envelope* is being disposed, never mid-session.

**Conclusion:** within one live `ArtifactStore<P, Mutation>` instance, once 64 distinct `Edit`
values have ever been created (`apply_command`'s fresh path, `amend_command`'s fresh-edit branch,
or the one-item `begin_apply_one`/`advance_apply_one` publish path — see §3), `reserve_edit_history_slot`
(`🏪️store/🦀️.rs:14647-14659`, backed by `vcs.edits.reserve_one()`) returns
`Err(VcsError::ValidationFailed("edit history ledger is saturated"))` forever, regardless of how
many of those edits have since been undone. The only way to get a fresh 64-slot budget is to close
and reopen the store (fresh `ArtifactStore::new`/hydration → fresh, empty `ArtifactHistoryLedger`).
Amending into an already-coalescing edit (`amend_command`'s "absorb" branch, `🏪️store/🦀️.rs:15926-15942`)
does *not* consume a new slot — but that only helps for a single continuous drag/coalesce gesture
sharing one `coalesce_key`, not for N independently-created edits.

## 2. Designed behavior / who introduced 64, and why

- **True first introduction:** commit `9d7cabfd9c` (`git log -S "ARTIFACT_HISTORY_LEDGER_CAPACITY" --date=iso --all --reverse`), authored by Ueli Saluz, **2026-08-23 10:55:52 +0200** (never trust the frozen `🎆️26🌙️06☀️04` template string in the auto-commit body — confirmed via `--date=iso`). Commit subject lines: *"📚️VCS artifact history ledger with generation-keyed fixed-capacity slots"* and *"🏪️Store sync retained actor turn with bounded artifact mailbox and snapshot-retirement lanes across store authority"*. The diff (`git show 9d7cabfd9c`) introduces the entire `ArtifactHistoryLedger<T>` type wholesale (generation-keyed fixed-capacity slots, `ArtifactHistoryKey`, `remove_key`, `try_push`) at the same time as `ARTIFACT_HISTORY_LEDGER_CAPACITY: usize = 64` — i.e. 64 was chosen together with the fixed-capacity-slot *design*, as part of a "bounded artifact mailbox / snapshot-retirement" pass across the whole store, not tuned per-app later.
  - The touched ticket doc referenced in that commit's tree,
    `.../EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📓️p8yt-full-operation-store-and-roster-foundation.md`,
    no longer exists on disk (ticket-close cleanup elsewhere) — its rationale text could not be
    read; only the commit diff and subject lines are available as evidence for intent.
- **File relocated (not redesigned) 2026-09-02:** commit `21fbcd3538`, 2026-09-02 12:19:02 +0200,
  a repo-wide "kind-only filenames"/taxonomy migration (`git show 21fbcd3538` diff for this file is
  a pure re-add of the same lines at the same relative offsets under the new path) — this is the
  commit `git log -S ... -- 🌿️vcs/🦀️.rs` (path-scoped) finds as "first", but it is a mechanical
  move, not the origin.
- **2026-09-08** commit `025ec86a42` only removed a *test* (`Vec::with_capacity(ARTIFACT_HISTORY_LEDGER_CAPACITY)` /
  `assert_eq!(drained, ARTIFACT_HISTORY_LEDGER_CAPACITY)`, per `git show 025ec86a42` diff) — no
  production-code change to the constant.
- **Present-day design docstring** (`🌿️vcs/🦀️.rs:285-286`): *"Fixed-capacity generation-keyed
  history authority. Live entries form one stable linked order; removed slots are tombstoned and
  reused only after their generation advances."* — describes the general-purpose reusable-slot
  design; it does not claim anything reuses slots for `vcs.edits` specifically during live editing
  (and per §1, nothing does).
- **Already-flagged as a live production defect in THIS ticket, prior to this audit:**
  - `📓️2026-09-09-runtime-verification.md:50-54` (already in the ticket tree) names this exact
    ceiling ("**Applied-edit ledger ceiling**... `push_applied` refuses at 64... retained tool
    operations that emit N mutations for one Emit create N one-item edits, so any example load or
    fill > 64 placements hit the ceiling. Audit `📓️2026-09-09-applied-ledger-ceiling-audit.md` in
    flight; fix wave to follow (one Edit per Emit vs compaction vs capacity)") — i.e. this file
    *is* that referenced audit.
  - `📓️2026-09-09-wave-D-production-defects.md:252-271` ("P4 the document one-item publication
    has no preinstalled capacity — FRAMEWORK CEILING") independently reaches the same structural
    conclusion with measured evidence ("temporary instrumentation in `🏪️store/🦀️.rs`, since
    reverted"): fault text `one-item publication requires preinstalled fixed applied and revision
    capacity / applied=64/64 revision=64/64 cursor=Some((64, 64))`; states "64 is
    `os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY`... a framework constant with no per-app knob"; names
    the same two remediation options this audit's §5 evaluates ("raise
    `ARTIFACT_HISTORY_LEDGER_CAPACITY`... or give the document vocabulary a whole-document
    replacement leaf so an example load is one edit"); and states plainly "**No puzzle3d-side
    change can preinstall past it.**"
  - `📓️2026-09-09-wave-X-test-suite.md:305-312` records the same fault from
    `nakagin_example_loads_via_operations`.

## 3. Does an existing multi-mutation single-edit path already exist?

**Yes — two structurally different publication paths coexist in `🔌️plugin/🦀️.rs`, and only one
of them pays one ledger slot per mutation:**

**(A) Batched — `ArtifactCommand::Apply`/`AmendLast` → `apply_command`/`amend_command` (ONE Edit, N forwards, ONE ledger slot).**
- `dispatch_emit_group` (`🔌️plugin/🦀️.rs:18422,18457-18458`) routes a whole `emit.artifact_mutations: Vec<Mutation>`
  as `ArtifactCommand::Apply { mutations: artifact_mutations, description }` (or `AmendLast` when a
  coalesce key is present).
- `apply_command` (`🏪️store/🦀️.rs:15874-15913`) calls `self.replay_mutations(&pre_snapshot, mutations)`
  **once**, over the **whole** input `Vec<Mutation>` (`:15882`), producing one `forwards: Vec<Mutation>` /
  `inverse: Vec<Mutation>` pair, builds exactly one `Edit` (`:15888-15899`), and calls
  `self.reserve_edit_history_slot()` (`:15887`) **once** regardless of how many mutations were
  batched in. `self.applied_edit_ids.push(edit_id)` (`:15908`) also happens once. This is already
  "N forwards, one ledger slot" — the mechanism the question's option (a) asks for.

**(B) One-item — `begin_apply_one`/`advance_apply_one` via `publish_mounted_typed_operation_unit` (ONE mutation per Edit, ONE ledger slot per mutation, one mutation drained per call).**
- `publish_mounted_typed_operation_unit` (`🔌️plugin/🦀️.rs:20187` def, relevant branch
  `:20409-20445`) pops **exactly one** mutation off `emit.artifact_mutations` (`emit.artifact_mutations.pop()`,
  `:20423`) and calls `self.store.begin_apply_one(...)` (`:20424-20433`) for that single mutation,
  storing the in-flight `ArtifactStoreOneItemPublication` on `mounted.pending_artifact_publication`
  (`:20435`). It is invoked once per bounded work unit (`self.publish_mounted_typed_operation_unit(&mut mounted)` at `:19849`,
  inside the mounted-operation step loop) — i.e. across many turns, each turn drains one more
  mutation from the same `emit.artifact_mutations` Vec that the retained `Work` produced in full at
  `PuzzleCommandWorkStep::Complete`.
- The one-item commit contract *forbids* batching inside a single `begin_apply_one`/`advance_apply_one`
  cycle: `advance_apply_one`'s `PreflightingCommit` phase (`🏪️store/🦀️.rs:15512-15538`) hard-requires
  `prepared.edit.forwards.len() != 1` to be false (`:15519`, part of the disjunction that faults with
  *"one-item prepared candidate failed its exact fixed commit contract"*, `:15531`) — i.e. exactly
  one forward mutation per one-item `Edit`, by construction.
- The capacity fault itself lives in the same phase: `:15533-15537`
  (`self.applied_edit_ids.len() == self.applied_edit_ids.capacity() || self.revision_accumulator.applied.len() == ... .capacity() || ...cursor...`)
  → `Err(VcsError::ValidationFailed("one-item publication requires preinstalled fixed applied and
  revision capacity"))` — the exact fault text quoted in `📓️2026-09-09-wave-D-production-defects.md:256-257`
  and `📓️2026-09-09-wave-X-test-suite.md:308-309`.

**Puzzle 3D's `setActiveExample`/`setFillCount` use path (B).**
`Puzzle3dSetActiveExampleWork`/`Puzzle3dPrecomputeCommandWork` (`✏️editor/🦀️.rs:6348,6356`) are
retained `Work` types (used because, per `📓️interactive-job-migration-recipe.md:112`,
"`setActiveExample`, `createAttraction`, fill/brush operations are genuinely unbounded in item
count and need real multi-stage chunking to stay under the 8ms step budget"). The fill/example
completion step returns `Emit { artifact_mutations: std::mem::take(&mut self.fill_mutations), ... }`
(`✏️editor/🦀️.rs:5663`) — a single `Emit` carrying every accumulated mutation — which is then
dispatched as a **typed/mounted tool operation**, so it goes through path (B) above, one mutation →
one `begin_apply_one` → one ledger slot, drained one mutation per turn.
`📓️2026-09-09-wave-D-production-defects.md:264-266` states the resulting count directly: "A
retained tool operation publishes `emit.artifact_mutations` ONE PER TURN through
`store.begin_apply_one`, so `setActiveExample nakagin` (180 objects + domain + catalogs ⇒ ~182
one-item edits) saturates the ledger at edit 64."

**Other apps.** `📓️interactive-job-migration-recipe.md:110` names Writer as the one app whose
document lane has "a natural one-item bounded preparation shape" — its
`bounded_first_step_tool_proofs!` (that doc's line 1134-1152, cited in the recipe) lists all 18
tool ids under one shared factory with one shared `resumable(...)` contract, described as "no
per-action tuning needed" **because Writer's document commands are single-document-replace-per-command**
— i.e. Writer's typed operations don't multiply mutations into many one-item edits the way
puzzle3d's `setActiveExample`/fill do; the recipe doc explicitly contrasts this with puzzle3d
("puzzle3d instead needs bespoke per-action `Work` types... because several of its actions...are
genuinely unbounded in item count", same line). Puzzle 5D is named
(`📓️interactive-job-migration-recipe.md:181`) as the one other puzzle-family app with an
Artifact-lane one-item factory (`Puzzle5dStorePreparationFactory`,
`🗿️artifacts/🖐️5d/.../✏️editor/🦀️.rs:7995-8155`), i.e. it uses the same one-mutation-per-edit
mechanism (B) as puzzle3d — this audit did not independently confirm whether Puzzle5D's own
bulk-load tool operations (its own example loads, if any) emit enough mutations per Emit to hit
the same 64-edit ceiling; that would need a separate, app-specific check of Puzzle5D's `Work`
completion sites, out of this audit's file:line-only scope. CAD was not found using either
`begin_apply_one` in a bulk-mutation loop or `ArtifactCommand::Apply` for large batches in the
files this audit read; no evidence gathered either way — not claimed.

## 4. Undo/redo semantics today

- **One applied edit id = one undo step**, unconditionally, for both paths (A) and (B): `undo_lane_position`
  (`🏪️store/🦀️.rs:15830-15847`) removes exactly one id from `applied_edit_ids` and pushes it to
  `redo_edit_ids`; the *entire* `Edit.forwards`/`Edit.inverse` for that one ledger entry is
  reverted/replayed together (`redo_lane_position:15854-15869`, `for operation in &edit.forwards { folded = apply_mutation(...) }`,
  `:15859-15861` — all forwards of one Edit apply/undo atomically as a unit).
- Consequence for path (A) (batched `ArtifactCommand::Apply`): if N mutations are bundled into one
  `Edit.forwards`, one `Ctrl+Z` undoes **all N together** — coarser undo granularity, by
  construction of the existing batched path.
- Consequence for path (B) (one-item, current puzzle3d bulk-load behavior): each of the ~182
  mutations from `setActiveExample nakagin` is its own `Edit` and its own undo step today (when it
  doesn't fault on ceiling) — i.e. undoing "load Nakagin" currently would take up to 182 separate
  undos, not one. Neither current path gives "one coherent undo step for one user-visible gesture"
  cleanly for a bulk load: (A) already collapses to one step (correct for this use case) but isn't
  wired to the bounded/progressive typed-operation path; (B) is wired to that path but produces
  many undo steps and burns ledger capacity per mutation.
- **Shell History panel.** The wire/decode layer the shell's history view is built from is
  `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs`: `HistoryCursor` (`:142-146`,
  fields `applied_edit_ids: Vec<String>`, `redo_edit_ids: Vec<String>`) and `HistoryEdit` (`:103`)
  mirror `store::ArtifactVcs`'s `applied_edit_ids`/`vcs.edits` 1:1 — `encode_cursor`/`decode_cursor`
  (`:1160-1205`) round-trip the full `applied_edit_ids`/`redo_edit_ids` lists id-for-id (comment at
  `:135-141`: "carries the FULL applied-edit list... because undo-then-apply interleavings are not
  representable by a single marker"). This audit did not locate the shell UI component that renders
  this into a visible History panel (out of scope for the framework/store/plugin files this audit
  was pointed at); the wire contract it would read from is this 1:1 mirror of the applied-edit
  ledger, so **whatever currently renders one row per `HistoryEdit`/`applied_edit_ids` entry would
  render one row per one-item mutation today** (182 rows for a Nakagin load, once/if it didn't
  fault), and would render one row per batched Edit if path (A) were used instead.
- **What batching (option a) would change here:** collapsing N one-item edits into one batched Edit
  moves the undo granularity from "one undo per mutation" to "one undo per gesture" and the History
  panel from "N rows" to "1 row" for that gesture — a real, user-visible behavior change, not just
  an internal optimization. This is a product decision, not just an engineering one.

## 5. Recommendation

**Recommended: (a) one Edit per Emit, with paged/progressive preparation across turns — extend
path (B)'s existing phased state machine to accumulate multiple forward mutations into ONE staged
`Edit` before committing ONE ledger slot, instead of committing one ledger slot per mutation.**

Rationale, weighed against (b) and (c):

- **(c) larger capacity** is the one-line fix but does not solve the problem, it only moves the
  wall: `setActiveExample nakagin` already needs ~182 slots
  (`📓️2026-09-09-wave-D-production-defects.md:266`); any fixed constant is one larger fixture away
  from faulting again, and CLAUDE.md's "no pragmatic path, aim for the clean long-term solution;
  don't care about implementation effort" rules out treating the ceiling number itself as the fix.
  **Memory cost per slot, from the actual types** (all `ARTIFACT_HISTORY_LEDGER_CAPACITY`-sized
  reservations, `🏪️store/🦀️.rs:12420-12433` `admitted_items`/`admitted_bytes` computes this
  exact sum for the parallel `applied`/`redo`/cursor-mirror/revision catalogs):
  - `applied_edit_ids`/`redo_edit_ids`/`cursor_applied_edit_ids`/`cursor_redo_edit_ids`: four
    `Vec<String>` reserved at capacity → `4 × capacity × size_of::<String>()` shell bytes
    (`String` is 24 bytes on a 64-bit target: ptr+len+cap) — **excludes** each string's own heap
    bytes (edit ids are content-addressed hex ids, `content_addressed_entity_id`
    `🌿️vcs/🦀️.rs:17-25`, `"edit-" + 16 hex chars` = 21 bytes each, heap-allocated per id).
  - `applied_revision`/`redo_revision`: two `Vec<CursorRevisionRecord>` reserved at capacity, each
    record `{id_digest: [u8;32], edit_digest: [u8;32], prefix_digest: [u8;32]}` = 96 bytes
    (`🏪️store/🦀️.rs:12280-12284`) → `2 × capacity × 96` bytes.
  - `vcs.edits: ArtifactHistoryLedger<Edit<Mutation>>` itself: `capacity` slots of
    `ArtifactHistorySlot<Edit<Mutation>>` (`🌿️vcs/🦀️.rs:277-283`, `generation: u32` + two
    `Option<u16>` + `free_next: Option<u16>` + `value: Option<Edit<Mutation>>` ≈ 16 bytes overhead
    + `size_of::<Edit<Mutation>>()`). `Edit<Op>`'s own shell
    (`🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1442-1453`: `id: String`, `actor: Option<String>`,
    `forwards: Vec<Op>`, `inverse: Vec<Op>`, `mutation_meta: Vec<MutationMeta>`,
    `description: Option<String>`, `coalesce_key: Option<String>`, `sequence_number: i32`,
    `started_at: String`, `finished_at: Option<String>`) is roughly nine 24-byte
    String/Option<String>/Vec fields plus a padded `i32` ≈ **~220 bytes of shell per Edit**,
    *excluding* the heap payload of `forwards`/`inverse`/`mutation_meta` (unbounded by this
    constant — each puzzle3d mutation's own payload size is separate from the ledger-capacity
    question entirely). At capacity 64 this is a small, bounded amount of shell memory either way
    (tens of KB) — the actual cost driver for a "just raise the constant" fix is not bytes, it's
    that raising it doesn't fix the unboundedness of "how many mutations can one bulk gesture
    produce," which is unbounded by nature (fill counts, example sizes).
- **(b) ledger compaction/checkpoint-folding** is semantically real (git-style "shallow history"),
  but non-trivial and cross-cutting: `CursorRevisionAccumulator`'s hash chain
  (`🏪️store/🦀️.rs:12277-12388`, `revision()` at `:12382-12387`) is an append-only prefix-hash
  over the *entire* applied/redo sequence from `identity_digest` — folding old edits into a
  checkpoint would require re-anchoring `identity_digest` at the checkpoint (a "shallow" identity),
  touching every `revision`/`reconcile` call site (`reconcile` at `:16859`, `known_hlc`,
  `content_addressed_checkpoint_id_with_pending_change` and everywhere a caller resolves an
  `applied_edit_ids` entry against `vcs.edits` by id: `edit_is_local`, coalesce-target lookup at
  `:15924-15925`, `known_hlc` at `:16279`). It also means undo depth becomes bounded by the last
  compaction point — a real semantics change (old edits become permanently non-undoable), which is
  the same tradeoff CLAUDE.md already accepts for "no legacy support" but is a bigger design
  surface than this ticket's remit.
- **(a)** reuses machinery that already exists on both sides: `apply_command` already proves "N
  forwards, one ledger slot, one `reserve_edit_history_slot()` call" works
  (`🏪️store/🦀️.rs:15874-15913`), and `begin_apply_one`/`advance_apply_one`'s phased
  `Preparing → PreparingCursor → PreflightingCommit → Publishing` state machine
  (`ArtifactStoreOneItemPublicationPhase`, `🏪️store/🦀️.rs:15462-15563`) already proves
  "bounded-step progress across many turns, resumable, cancellable" works for a single item. The
  fix is a `begin_apply_many`/paged variant of that same state machine: keep draining
  `emit.artifact_mutations` one bounded 8ms step at a time
  (`INTERACTIVE_STEP_CEILING_US = 8_000`, `🔨️modules/⏱️trace/🦀️.rs:96`) exactly as today, but
  accumulate each drained mutation into a **staged, not-yet-committed** `forwards`/`inverse` buffer
  (bounded by `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES = 1_048_576`,
  `🏪️store/🦀️.rs:13078`/`ARTIFACT_STORE_ONE_ITEM_ID_BYTES = 256`, `:13080`, the existing
  per-item footprint ceiling — reinterpreted as a per-*edit* ceiling for the batch) instead of
  calling `reserve_edit_history_slot`/`insert_reserved_edit_history` after every single mutation,
  and only commit the ONE accumulated `Edit` (one `reserve_edit_history_slot()`,
  `🏪️store/🦀️.rs:14647`) once the whole `emit.artifact_mutations` Vec is drained. Exact touch
  points:
  - `🏪️store/🦀️.rs:15369-15460` `begin_apply_one_owned` — the `Input`-generic preparation
    entry point; add a parallel `begin_apply_many_owned` (or generalize this one) that accepts
    `Vec<Input>` and stages instead of preparing-and-committing per call.
  - `🏪️store/🦀️.rs:15464-15619` `advance_apply_one` — the phase state machine to extend with a
    "keep draining, don't commit yet" loop condition before `PreflightingCommit`; the existing
    `PreflightingCommit` capacity check (`:15533-15538`) then only ever fires once per *gesture*,
    not once per mutation.
  - `🔌️plugin/🦀️.rs:20409-20445` `publish_mounted_typed_operation_unit`'s
    `ArtifactToolCompletionValue::Emit` branch — change `emit.artifact_mutations.pop()` (single
    mutation, `:20423`) to feed the batched preparation instead, still respecting
    `grant.permits_one()`/the bounded-step budget per call (`🏪️store/🦀️.rs:15485-15487`,
    `ArtifactStoreOneItemGrant`).
  - Undo/redo semantics (§4) become "one undo per gesture" for these bulk paths, matching (A)'s
    existing behavior — a coordinator/product decision to confirm explicitly (this audit only
    surfaces that the change is real, not which is preferred).

## Files read for this audit (no source file was edited)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` (full read, 1-1070 + tail sections)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (targeted reads: 561-1070, 12275-12530,
  14380-14930, 15330-15970, 15680-15760)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (targeted reads: 4541-4560, 8950-9310,
  14650-14780, 18280-18460, 19640-19880, 20180-20500)
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1430-1490` (`Edit<Op>`)
- `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:96` (`INTERACTIVE_STEP_CEILING_US`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs` (targeted reads: 1-1210)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  (targeted reads around 5460-5770, 6340-6360, 6820-7060)
- Ticket docs (already in tree, not authored by this audit): `📓️2026-09-09-wave-D-production-defects.md`,
  `📓️2026-09-09-wave-X-test-suite.md`, `📓️2026-09-09-runtime-verification.md`,
  `.../INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📓️interactive-job-migration-recipe.md`
  (path under `🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️interactive-job-migration-recipe.md`)
- `git show 9d7cabfd9c`, `git show 21fbcd3538`, `git show 025ec86a42`,
  `git log -S "ARTIFACT_HISTORY_LEDGER_CAPACITY" --date=iso --all --reverse`
