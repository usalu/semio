# 🎚️ Wave B1 — session / job bridge

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`, master plan §1–2.3.
Editor root `E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Files owned and changed: `E/⏳️precompute/🦀️.rs`, `E/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`,
`E/⏳️precompute/📐️geometry/🦀️.rs`, `E/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs`.

## 1. Public API (exact signatures)

```rust
// E/⏳️precompute/🦀️.rs — impl Puzzle3dPrecomputeSession
pub fn fill_requested_count(&self) -> u32;
pub fn set_fill_requested_count(&mut self, count: u32);
pub(crate) fn take_fill_locked_chunk(&mut self, max_delta: usize) -> Option<FillApplyChunk>;
pub fn fill_progress_summary(&self) -> FillProgressSummary;   // extended, see §3
pub(crate) fn set_fill_applied_count(&mut self, count: u32);  // now syncs the builder too
pub(crate) fn apply_fill_count_chunk(&mut self, requested: u32, max_delta: usize) -> Option<FillApplyChunk>; // KEPT for B2

// E/⏳️precompute/🦀️.rs — constants
pub(crate) const FILL_LOCK_PLACEMENTS_PER_TICK: usize = 8;
pub(crate) const FILL_REQUESTED_COUNT_DEFAULT: usize = 100;   // replaces FILL_COUNT_MAX as a SEED, not a ceiling

// E/⏳️precompute/📐️geometry/🦀️.rs — constants
pub(crate) const NAKAGIN_DOCUMENT_OBJECTS: usize = 180;
pub(crate) const DOCUMENT_FILL_HEADROOM_SLOTS: usize = 1024;
pub(crate) const DOCUMENT_OBJECT_SLOTS: usize = (NAKAGIN_DOCUMENT_OBJECTS + DOCUMENT_FILL_HEADROOM_SLOTS).next_power_of_two(); // 2048, unchanged
```

`FILL_COUNT_MAX` is **deleted**. No constant bounds a plan any more.

## 2. How the requested count reaches the worker

`SceneConfig` carries no `fill_count` (checked: `🧬️schema/🦀️.rs:553`), so `runtime.fill_count` reaches the
planner through the session, not through the scene:

1. The app calls `precompute.set_fill_requested_count(runtime.fill_count)` — **wave B2 must add this call**
   next to the existing `precompute.set_fill_applied_count(runtime.fill_count)` sites
   (`E/🦀️.rs:6996`, `:8235`, `:8295`, `:8335`).
2. `Puzzle3dCollision::fill_requested_count` (engine field) records it, and
   `start_fill_preparation` seeds every fresh builder with it —
   `FillBuilder::begin_preparation(roots, operation, self.fill_requested_count)` — and arms the local lane
   with `fill_steps_remaining = requested` (was `FILL_COUNT_MAX`).
3. It travels across dispatches on `Puzzle3dFillSession::fill_requested_count` (take/install), exactly like
   the cancel token, and is restored from the envelope by `restore_persisted_fill`.
4. `Puzzle3dPrecomputeSession::publish_fill_requested_count` writes it to **`FillEnvelopeAuthority::requested_count`**
   (new field) and, when readable, straight into the builder.
5. `drive_fill_envelope` — the isolated worker's own step path (`Puzzle3dFillBoundedJob::step` →
   `drive_fill_envelope`) — reads `authority.requested_count` **before every builder step** and forwards it
   through `FillBuilder::set_requested_count`.
6. It is re-published on every `set_fill_applied_count` and every `take_fill_locked_chunk`, so a contended
   registry can only delay a target, never lose it (idempotent by construction).

**Lowering floor.** `publish_fill_requested_count` hands the builder `requested.max(fill_applied_count)`:
the builder may only discard plan rows the document no longer shows. `take_fill_locked_chunk` deletes the
document tail 8-at-a-time and calls `set_fill_applied_count`, which re-publishes — so the builder's target
walks down one locked chunk behind the document, and the discard only reaches the new count once the
document is already there. `FillProgressSummary::max_count` reports the SESSION's requested count
throughout, never the intermediate floor.

**Raising.** A live builder simply gets the higher target (wave A's `set_requested_count` un-stalls a
`Complete` planner). A run that already terminalized is resumed in place by the new
`Puzzle3dPrecomputeSession::resume_completed_fill_job`, called from `enqueue_fill_job_spending`: a
`Terminal(Complete)` envelope is deliberately never reaped (`take_finished` excludes `Complete`), so the
builder, its spatial index and the whole locked prefix are still standing — the authority goes back to
`Admitted`, `worker_terminal` is cleared, and the SAME `(job, token)` pair is handed back for a fresh
`Effect::SpawnJob`. The reactor drops a bounded job's slot the turn it reports terminal
(`⚛️reactor/💼️jobs/🦀️.rs:441`), so re-spawning that id binds this very envelope again and the deterministic
sequence continues instead of replanning from zero. `poll_fill_job` reports the flip because the resume
writes the authority's observation and leaves the session's stale.

## 3. Extended readouts

`FillProgressSummary` (wave A's schema) is filled by two helpers in `E/⏳️precompute/🦀️.rs`:

- `fill_progress_summary_of(fill, requested, applied)` — builder readable locally: `tested` ←
  `FillBuilder::tested_count`, `rejected` ← `preview.rejected_count`, `collisions` ←
  `FillBuilder::collisions`, `stage` ← `preview.stage`, `stall_reason` ← `preview.stall_reason`.
- `fill_progress_summary_of_observation(observation, requested, applied)` — isolated plan: same fields off
  the extended `FillObservation`.

`FillObservation` grew `tested: u64, rejected: u64, collisions: u64, stage: u8, stall: u8` and stays a
fixed `Copy` struct of scalars (one per envelope slot, diffed on every 120 ms poll). The stage and stall
reason travel as codes against `FILL_STAGE_LABELS` / `FILL_STALL_LABELS`; an undeclared stall reason
travels as `FILL_STALL_OTHER` and reads back as `"stalled"` — a stall is never invisible.

## 4. Capacity is a stall, not a fault

`FillBuilder::step` still faults with `b"fill-preparation-capacity"` / `b"fill-fixed-collection-capacity"`
when a fixed document page refuses a row (wave A's file). `drive_fill_envelope` now inspects the fault
payload: a capacity payload terminalizes the envelope as `Complete` with
`observation.stall = "document-capacity"` instead of `Fault`. Consequences:

- `observe_fill_terminal_reason` never latches `fill_faulted` / `fill_fault_notice` for it, so
  `take_fill_fault_notice` stays for genuine faults and no `fill_failed` notice is raised.
- `fill_progress_summary().stall_reason == Some("document-capacity")` — visible in the HUD (wave C).
- The latched stall survives every later observation refresh in the same drive.

Geometry: the `DOCUMENT_*_SLOTS` doc comments and the capacity law no longer mention a plan ceiling.
`DOCUMENT_OBJECT_SLOTS` is now literally derived as flagship fixture + declared fill headroom, rounded to
the next power of two (still 2048).

## 5. Tests

(results below)

## 6. Open issues for B2 / C / A

- **B2 must call `precompute.set_fill_requested_count(runtime.fill_count)`** wherever it already calls
  `set_fill_applied_count(runtime.fill_count)`. Until it does, a session plans toward
  `FILL_REQUESTED_COUNT_DEFAULT` (100) — deliberately the same number as the new `Puzzle3dConfig::fill_count`
  default, so the seam is invisible if B2 lands the default too.
- `apply_fill_count_chunk` is **deleted** (see the follow-up below) — `take_fill_locked_chunk` is the one
  document-side delta, and `setFillCount` reaches it through B2's `set_fill_count::take_locked_mutations`.
- **B2/C**: `fill_progress_summary()` now carries `tested/rejected/collisions/stage/stall_reason` — feed the
  `fillBuild` interaction block (§2.5) and the HUD from it, and show `stall_reason` instead of a notice.
- **Wave A**: `🪣️fill/🧪️tests/🔬️unit/🦀️.rs:1113,1119,1120` still reference `FILL_COUNT_MAX` and the
  `fillCountMax` fixture key; `DOCUMENT_FILL_HEADROOM_SLOTS` / `NAKAGIN_DOCUMENT_OBJECTS` are exported for
  exactly those laws.
- **Wave H**: the resume path re-spawns the SAME job id against the same envelope token. The unit law drives
  `drive_fill_envelope` directly, so the reactor's own re-spawn of a retired job id is only covered by the
  browser probe — verify that typing a larger count on a finished plan continues it without a restart.

## 7. Follow-up (coordinator request)

### 7.1 `apply_fill_count_chunk` deleted

Grepped `✏️s/🔌️plugins/🧩️puzzle` (3d AND 5d, `*.rs` / `*.ts` / `*.tsx`): after B2 rewrote
`set-fill-count` onto `take_locked_mutations` → `take_fill_locked_chunk`, the only remaining occurrence of
`apply_fill_count_chunk` was its own definition. **Deleted** from `E/⏳️precompute/🦀️.rs`. Nothing else used
it; `FillApplyChunk` stays (it is `take_fill_locked_chunk`'s return type) and `set_fill_applied_count`
stays — `take_fill_locked_chunk` calls it to commit the new locked cursor, sync the builder's
`applied_count` and re-publish the requested count, and the app still calls it directly.

### 7.2 `compose_fill_display` / `reveal_index`

- `Puzzle3dPrecomputeSession::compose_fill_display(count: u32)` had **no caller left** (the
  `Puzzle3dEngineCommand::ComposeFillDisplay` dispatch arm goes straight to `compose_fill_projection`).
  **Deleted.**
- `Puzzle3dCollision::compose_fill_display` is `#[cfg(test)]` and still carries
  `compose_fill_display_is_read_only_and_matches_apply_prefix` — kept.
- **For wave A**: `FixtureObject::reveal_index` (`🧬️schema/🦀️.rs:476-478`) is now written in exactly one
  place, `🪣️fill/🦀️.rs:4375` (`placed_object.reveal_index = Some(self.appended_objects.len())`), and read
  by nobody — B2 removed the editor-side `Puzzle3dObject.reveal_index` and wave C removes the client reveal
  cutoff. When wave A drops the field, the two strip sites in my file go with it:
  `⏳️precompute/🦀️.rs` `apply_fill_count` (`#[cfg(test)]`, ~:2786) and `compose_fill_projection`'s
  `persisted` flag — at that point `compose_fill_projection` loses the flag and becomes a plain prefix
  query. **Also for wave A**: `Puzzle3dEngineCommand::ComposeFillDisplay` itself has no production caller
  left once the ghost tail is gone.

### 7.3 Test-build fix in my own files

`E/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` — `mount_fill_worker(...).expect("mounted worker")` needed
`Debug` on `WorkerJobSessionAdmissionRejected<SharedFillWorkerJob>` (the rejected admission is an owner
with an exact incremental close, deliberately not `Debug`). Rewritten as a `let Ok(mounted) = … else
{ panic!(…) }`, which needs no bound on the error owner.
