# 🏁️ Wave W1-H: Fill Silent Stall

Lane W1-H. It follows up `📓️wave-W1-G.md` §7 item 1.

**Status: landed.** The run over the 1.5× own-mesh Nakagin variant now plans: it tests 3 344 candidates, places 1, and ends with a visible `warning` / `no-free-placement` step. A completed plan now carries its end in its stage, so the planner cannot complete without a reason. Verification results are in §3.

Paths:
- `P` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute`
- `T` = this ticket folder

Logs are in `T/🗑️generated/W1-H/`.

## 1. Root cause

W1-G suspected a stale spatial-index step. That was not the cause. There were two defects, and both were needed to produce the silent run.

### 1.1 A spatial cell refused dense documents

- **Symptom.** A spatial cell's member bucket was `FixedOwnerSet<String>`, which is 32 bookkeeping slots wide.
- **Measurement** (8-unit cells, all 180 Nakagin bodies). The fullest cell holds:

  | Own-mesh scale | Members in the fullest cell |
  |---|---|
  | 0.5 | 24 |
  | 1.0 | 30 |
  | 1.25 | 31 |
  | 1.5 | 43 |

- **Effect.** During `PrepareSpatial`, the replacement for body 129 was refused as `Rejected`, which set `collection_over_capacity`. Debug state at the end: `entries=128`, `placed=180`, `fixed=spatial`.
- **Why this is a defect.** The documentation of `FIXED_OWNER_SLOTS` says it bounds close-step work and never document size. This bucket used it as a density limit anyway.
- **A later risk.** Scale 1.0 was already at 30 of 32, so fill placements next to the tower would have hit the same limit mid-plan.

### 1.2 The run job returned a bare `Complete`

- **The path.** `FillRunJob::step` passed the `Complete` from `FillBuilder::capacity_outcome` straight through for the `document-capacity` stall.
- **What was skipped.** It never ran `observe`, `settle` or `flush`, so there was no warning step and no final progress.
- **The result.** The planner held `stalled = true` and the ledger saw nothing: counters `[0,0,0,0]`, no steps.

## 2. What changed

### 2.1 Geometry (`P/📐️geometry/🦀️.rs`)

- **New constant `DOCUMENT_CELL_MEMBER_SLOTS = DOCUMENT_OBJECT_SLOTS` (2048)** (`:65-69`). A cell can never refuse a body that the entry map admits.
- **Sub-page ceiling.** New `OWNER_SUB_PAGE_UNBOUNDED` (`:71`) and `owner_sub_page_slots_within` (`:104`). `FixedOwnerMap` and `FixedOwnerSet` take a fourth const parameter `S`, the sub-page ceiling, defaulting to unbounded (`:353`, `:365`, `:588`, `:598`). Every existing owner keeps its sizing.
- **`pub(crate) type CollisionCellMembers = FixedOwnerSet<String, DOCUMENT_CELL_MEMBER_SLOTS, FIXED_OWNER_SLOTS>`** (`:1102`). It is used for `cells` and `retiring_bucket`. A sparse cell still claims one 32-slot page and grows one page at a time.
- **Doc comments.** Updated on `FIXED_OWNER_SLOTS` and `DOCUMENT_CELL_SLOTS`.

### 2.2 Fill (`P/🪣️fill/🦀️.rs`): type-level terminal

- **New stall type** (`:30`):
  ```rust
  pub(crate) enum FillStall { NoOpenVortex, NoCompatibleKind, NoFreePlacement, DocumentCapacity }
  // ALL, const fn reason(self) -> FillRunReason
  ```
  It replaces the four `FILL_STALL_*` strings and the `of_id(..).unwrap_or(Rejected)` fallback.
- **New end type** (`:58`), carried by the stage:
  ```rust
  pub(crate) enum FillPlanEnd { Reached, Stalled(FillStall) }
  FillJobStage::Complete(FillPlanEnd)   // :191
  ```
  The `stalled: bool` field is gone.
- **New accessors.** `pub(crate) fn end(&self) -> Option<FillPlanEnd>` (`:1301`). `fn next_round_stage` (`:1310`) is the only place that builds `Complete(Reached)`, and it does so only when `sequence.len() >= max_count`.
- **`stall(FillStall)`** (`:2164`). A candidate still under test gets a `Refused(stall.reason())` verdict first. `FillRunEvent::Stalled` was removed.
- **Capacity check split in two:**
  - `pub(crate) fn capacity_refusal<C: FillStepContext>(&mut self, context: &mut C, writer: Option<&mut ToolRunTickWriter>) -> Option<StepOutcome>` (`:2287`) handles the preflight refusal: a danger step, then a fault.
  - `pub(crate) fn stall_on_capacity(&mut self) -> bool` (`:2301`) handles a page that fills mid-plan.
- **Stale spatial index.** `discard_tail_one`, `prepare_one` and `prepare_spatial_one` now return `Result<(), StaleSpatialIndex>`. `advance` turns a stale result into `Fault("stale-spatial-index")` (`:2364`, `:2370`).
  - Before, `Stale` set `stalled` without changing stage. That was a livelock, not a visible end.
  - It is unreachable today, because the builder's owner never changes.

### 2.3 Fill: run-job assertion

- **`FillRunJob::settle(&mut self) -> Result<(), &'static [u8]>`** (`:2894`) derives the one terminal step from `builder.end()`:

  | Plan end | Terminal step |
  |---|---|
  | `Stalled(s)` | `warning` with `s.reason()`, args `[placements]` |
  | `Reached` with provisional cap | nothing more (the cap `warning` was already published) |
  | `Reached` with placements ≥ requested | `success` `requested-reached` |
  | anything else | the run faults with `fill-run-end-undeclared` |

- **The run job's own `stall` field was removed.**
- **Mid-plan capacity** (`:2999`). `step` calls `stall_on_capacity()`, then `observe`, then the normal settle, flush and `Complete` path.
- **Page-rejection faults are named.** They were empty before: `fill-run-tick-page` (`:2921`) and `fill-revalidate-tick-page` (`:3220`).

## 3. Tests

### 3.1 New or changed laws

- **Fixture law `laws.visibleEnd`** (`P/🪣️fill/🧫️fixtures/🎞️fill-run.json`). Nakagin, seed 1, requested 12, own-mesh scale 1.5. Expected: tested 3344, locked 1, collisions 3343, rejected 0, stall `no-free-placement`.
- **Fixture `documentCapacities.cellMemberSlots: 2048`.**
- **`fill_run_ends_visibly_with_a_declared_reason_for_every_case_and_own_mesh_variant`** (`P/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:1570`). The helper `fill_run_visible_end` (`:1550`) checks, for every shipped case and every `visibleEnd` case:
  - the last progress is `Complete`;
  - the last step carries `[locked]`;
  - the last step is either `success:requested-reached` with the request met, or a `warning` whose reason is in `FillStall::ALL`;
  - a run with zero verdicts always names a stall.
- **`fill_run_job_mid_plan_capacity_stall_ends_with_a_visible_warning_step`** (`:1604`). Concrete Forest seed 7: after the first placement it forces `collection_over_capacity`. Expected: a `document-capacity` warning with `[1]`, and every tested candidate reached a verdict.
- **`spatial_capacity_plus_one_refusal_preserves_exact_old_state`** (geometry tests; name kept because the P4e policy requires it). One cell now admits all 2048 entries. Entry 2049 is refused by the entry capacity and leaves the state unchanged.
- **`document_scale_capacities_…` and `every_fixed_owner_sub_page_…`.** A bucket is `DOCUMENT_CELL_MEMBER_SLOTS` wide, grows one bookkeeping page at a time, and every cell occupied by one page stays ≤ 8 MiB.
- **Existing tests.** Updated for `Complete(_)` and `end()` in place of `stalled`.

### 3.2 Red proof (before any source change)

Log: `test-red.txt`. All three new or changed laws failed:
- the geometry law failed at entry 32;
- the mid-plan law ended with last progress `Running`;
- the own-mesh Nakagin variant ended with no progress and no step.

The shipped cases passed through the visible-end helper.

### 3.3 Green and verification

<!-- VERIFY -->

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill_run_ends_visibly fill_run_job_mid_plan_capacity spatial_capacity_plus_one`

## 5. Deviations

1. **The root fix is in the spatial index, not a new stall reason.** The index now admits the dense document, so the variant produces verdicts. The silent path is closed separately (§1.2), so if a real capacity limit is ever hit, it shows as `document-capacity`.
2. **The "zero verdicts ⇒ stall reason" invariant has one exception: requested count 0.** In that case the run ends with `success:requested-reached [0]`, which is still a declared terminal reason. A dedicated reason would need a schema and terminology change owned by other lanes.
3. **The parry3d oracle's Nakagin variant stays at scale 0.5 (W1-G's value).**
   - I switched it to 1.5, and the oracle alone ran for more than 75 minutes at load average 60–80 and did not finish. At 1.5, far more body pairs overlap, and each one needs a 16³ parry containment grid in a debug build.
   - I stopped that run (my own test process, pid 50636 only) and restored 0.5. That aborted log is `test-fill-t4-aborted-oracle-1.5.txt`.
   - The 1.5 variant is covered by the `visibleEnd` counters instead.
4. **`StaleSpatialIndex` faults instead of stalling.** A stale index is an invariant break, like `stale-fill-operation`. It is not a document condition the user can act on.

## 6. Foreign edits

- **`📜️script.ts:9674`.** The P4e literal `cells: FixedOwnerMap<(i32, i32, i32), FixedOwnerSet<String>, DOCUMENT_CELL_SLOTS>` became `… CollisionCellMembers, DOCUMENT_CELL_SLOTS>`.
- **`📜️script.ts:9615`.** `fill.includes("self.prepare_one();")` became `"self.prepare_one()"`, because `prepare_one` now returns a `Result`. The mutation `missing-cooperative-unit` still goes red.
- **`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts:134,212`.** The same cell literal, in the clean miniature and in the `dynamic-bucket` mutation.
- **Policy check.** `bun T/🐍️w1d-policy-probe.ts` reports self-tests `tool-run=38 run-job=32 trace=13 p4e=27`, with no p4e, run-job or trace findings (`policy-probe-2.txt`).
- **The run job region in `P/🪣️fill/🦀️.rs`, which W0-I may adopt.** I re-read the file before editing (it was unchanged since 11:38). Every edit was an exact-string replacement and no foreign change was reverted.

## 7. Other paths that end silently (step 4)

| Path | Status |
|---|---|
| Mid-plan `document-capacity` returned a bare `Complete` from the run job | **Fixed** (§1.2, §2.3) |
| `Stale` in `prepare_spatial_one` / `discard_tail_one` set `stalled` without changing stage (livelock) | **Fixed**: now a named fault |
| A completed plan with no reason, or `Reached` below the request | **Structurally impossible**: `Complete` carries a `FillPlanEnd`, and `settle` faults with `fill-run-end-undeclared` otherwise |
| Tick page rejection faulted with an empty detail (run and revalidate) | **Fixed**: named details |
| `FillRunJob::settle_owed` checkpoint page rejection returns `Yield` and drops that checkpoint | **Open** — not terminal; the next placement checkpoints again |
| `BrushSuggestionsRunJob::flush` (`P/🖌️brush/🦀️.rs:990,996`) faults with an empty detail on encode or page rejection | **Open** — brush lane, not changed here |
| `FillRevalidateJob` with no conflicts ends with a tick that has no step | By design — the ledger finalizes, and it is not the run's terminal |

## 8. Open items

1. **The parry3d oracle at own-mesh scale 1.5** needs a cheaper overlap estimate or a release-profile lane before it can join the suite (§5.3).
2. **The brush suggestions fault details** (§7).
3. **`FillRunReason::of_id`** may now be unused outside tests. The schema owner should check its dead-code status.
