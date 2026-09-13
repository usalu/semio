# Audit — puzzle3d fill after phase 3 (compile/test/current-flow/gap)

Read-only. No source touched. Editor root `E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
schema root `A` = `…/✳️any`, host `H` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.
Inputs read: `📋️master-plan.md`, `📓️status.md`, `📓️wave-{A,B1,B2,C,D,G}.md`, `📓️audit-fill-pipeline.md`,
`📓️audit-tests-and-deploy.md`.

## 1. Compile

`cd /Users/ueli/Documents/semio && cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --message-format short` →
`🗑️generated/p4-fill-check.txt`. **0 errors, 96 warnings**, `Finished dev profile … in 6m 01s`. Matches the state every
wave report (A/B1/B2/C/D/G) already claimed — waves A–G landed and the tree is green. The 96 warnings are the same
pre-existing `unnecessary qualification` / `never used` noise the waves called out (baseline was 97; B2's cleanup
dropped one). `integration-check-2.txt` already sitting in the generated folder (14:18, stale — before the 20:45
editor touch) agreed with this; I re-ran fresh rather than trust it.

## 2. Tests

Command (`📓️audit-tests-and-deploy.md` §3): `bun ./📜️script.ts test -- fill` from
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`. The nextest-wrapped runner has a 15 s watchdog on the
`fundamental` profile and cancel-on-first-failure; the moment the first fill test failed it killed the whole batch
(`🗑️generated/p4-fill-tests.txt`) — not useful for a full count, so I fell back to plain `cargo test -p
semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill --test-threads=4`
(`🗑️generated/p4-fill-tests-cargo.txt`, 1012.78 s) and re-ran the initially-failing subset with
`--test-threads=1` to separate real regressions from test-isolation artifacts (`🗑️generated/p4-fill-tests-isolated.txt`).

**Result: 95 passed, 5 failed** (678 non-fill tests filtered out). Under `--test-threads=1`, one of the five
(`fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks`) **passed** — a parallel-run
artifact (plausibly the new process-global live-session registry wave B2 introduced for `FillLock`, shared across
concurrently-running test threads). The other **four are real, deterministic failures**, reproduced in isolation:

| Test | `🦀️.rs` panic site | Message |
|---|---|---|
| `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` | `E/🧪️tests/🔬️unit/🦀️.rs:6187` | `fillBuildTick worst turn 5.093459ms over 771 turns exceeds this artifact's own unoptimized budget 2ms` |
| `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run` | `E/🧪️tests/🔬️unit/🦀️.rs:4240` | `the Cancel fill affordance publishes the live job identity once planning has produced readiness` (panics on `.expect()` — identity was `None`) |
| `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one` | `E/🧪️tests/🔬️unit/🦀️.rs:4372` | `an armed, planning fill run publishes its cancel identity` (same — `None`) |
| `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count` | `E/🧪️tests/🔬️unit/🦀️.rs:2005` | `assertion left == right failed: fill count starts at zero before any request` — `left: Some(100.0), right: Some(0.0)` |

The 4th is test staleness, not a product bug: the test's own literal (`0`) was never updated for wave B2/D's product
decision 4 (default `fill_count` = 100) — it currently asserts the pre-phase-3 default. Trivial fix, flagged for
whoever owns the next test pass, not analysed further here.

The **first three are load-bearing for this ticket's gap analysis**:

1. **`fillBuildTick` already violates its own interactive budget** (5.09 ms vs 2 ms, on the very test wave B2's own
   report (§4.3/§6.4) flagged as "re-run once the tree compiles"). Confirms the audit-fill-pipeline prediction: making
   `fillBuildTick` a document-intent action that diffs the whole fixture every 120 ms tick (§2.3 below) is measurably
   too expensive once a plan runs long — this is the SAME mechanism requirement (ii) asks to replace with a
   provisional/transaction layer, so the performance regression and the product gap are the same code path.
2. **The cancel/abort affordance is not reliably present while a run has committed progress.** Both failing tests
   wait for `fill_ready() > 0` (pieces already locked into the document — `count_measure`'s `ready`, §3.1 below)
   — one also waits for a spawned background job (`live` non-empty) — and then find
   `precompute.fill_job_identity()` is `None`, so `progress_measure` (§3.1) returns `None` and no "Cancel fill" /
   progress row exists at all. **A user can already have real, committed document objects on screen with no cancel
   button showing.** This is not a flake: reproduced deterministically in isolation on both tests, at two different
   panic sites in two different code paths (one via `fillBuildTick`+`cancelFillBuild`, one via `engagementAbort`).

## 3. Current end-to-end flow (file:line)

### 3.1 Arming and the count control

`E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` — the fill tool (`TOOL_ID = "fill"`, `:15`) is armed via the ordinary
`setActiveTool` action; its measures (`:93-98 measures()`) are: `count_measure` (`:33-48`, a
`WindowMeasure::Number { min: 0, max: None, value: runtime.fill_count, ready: progress.applied_count, on_change:
"setFillCount" }` — `ready` is the LOCKED count, not a plan extent), `progress_measure` (`:73-89`, a
`WindowMeasure::Progress` gated on `precompute.fill_job_identity()` being `Some` AND `!progress.done` — returns
`None` otherwise, §2 above), and the distribution tree.

### 3.2 Cancellation

`E/🎮️commands/🪣️fill-build-tick/🦀️.rs` `cancel_fill_build` (whole file, 52 lines) — reads `(job, operation,
generation)` out of the action args (supplied by `progress_measure`'s `cancel` field) and calls
`precompute.cancel_fill_job_for(job, operation, generation)` (`E/⏳️precompute/🦀️.rs:3717`), an identity-gated no-op
if the triple is stale. There is a second abort path, `engagementAbort` (`E/🦀️.rs:3566` dispatch,
`8465` action definition, bound to `Escape` at `:8396`), whose publication contract
(`E/🦀️.rs:7134`) is `ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes:
&[ArtifactToolPublicationLane::WindowTransient] }` — **`WindowTransient` only, no `Artifact` lane.** This is
structural, not incidental: `engagementAbort` is declared incapable of emitting a document mutation at all. It can
cancel the background job and clear armed/interaction state; it cannot and does not undo any object that a prior
`fillBuildTick` already locked into the document.

### 3.3 Where accepted pieces become document mutations (already immediate — decision 2 already landed)

`E/🎮️commands/🪣️fill-build-tick/🦀️.rs` `fill_build_tick` (top of file) — every ~120 ms tick: `poll_fill_job()` →
`set_fill_count::take_locked_into_fixture(&mut precompute, &mut ctx.scene.fixture)` (mutates `ctx.scene.fixture`
directly) → `enqueue_fill_job()` if none is live. `take_locked_into_fixture`
(`E/🎮️commands/🧮️set-fill-count/🦀️.rs:61-77`) calls `precompute.take_fill_locked_chunk(FILL_LOCK_PLACEMENTS_PER_TICK)`
(`E/⏳️precompute/🦀️.rs:3331`, chunk ≤ 8 per tick) and splices `chunk.added_objects`/`added_attractions` straight
into `fixture.objects`/`fixture.attractions` (removed ids first, for a lowering run). Because `fillBuildTick`'s
epilogue treats this as a document-intent action (`puzzle3d_action_document_intent`, wave B2 report §3), the ctx
diff (`puzzle3d_operations_from_fixture_change`) turns that fixture edit into a real `create_object` +
`connect_vortices` (or `delete_object`) **on every tick**, coalesced under history key `"fill-count"`
(`E/🦀️.rs:3426`, `AmendLast`). `setFillCount`'s own `FillLock` stage (`E/🦀️.rs:6937-6949`) does the identical thing
synchronously in a loop until a chunk is empty, sharing the same `take_locked_mutations`
(`E/🎮️commands/🧮️set-fill-count/🦀️.rs:45-54`) and the same `"fill-count"` coalesce key (`E/🦀️.rs:6983`).

**So: yes, locked pieces are durable document history immediately, every tick, with no provisional state anywhere**
— confirming `status.md`'s note that "B2's tick commits locked pieces straight into the document contradicts
requirement 2." There is no run/operation id stamped on a locked `FixtureObject`, no separate "provisional" owner
set, nothing that distinguishes "placed by the run in progress" from "placed by a previous run or a manual edit" —
`FillApplyChunk` (`E/⏳️precompute/🦀️.rs:3003-3008`) carries only `applied_count`/`added_objects`/`added_attractions`/
`removed_object_ids`, no identity. The only thing that CAN revert an in-progress run today is the generic document
**Undo**, which reverts the whole coalesced `"fill-count"` history entry — not scoped to "since I pressed start",
and unusable the moment anything else coalesces under or after it.

### 3.4 What the viewport receives for tried candidates

`FillBuildPreview` (`A/🧬️schema/🦀️.rs:771-793`, wave A): `verdict` (of `candidate_ghost`), `tried: [Option<
FillTriedCandidate>; FILL_TRIED_RING]` with `FILL_TRIED_RING = 12` (`A/🧬️schema/🦀️.rs:684`), `tested_count`,
`requested_count`, `stall_reason`. Each `FillTriedCandidate` (`:722`) carries `sequence`, `verdict`, `reason`, and a
full `ghost: BrushPreviewState` — 6 pose fields (`A/🧬️schema/🦀️.rs:586-596`: `target_vortex_full_id: String`,
`object_kind_id: String`, `source_vortex_index: usize`, `mesh_url: String`, `origin: Vec3`, `orientation: Quat`,
optional `scale`). The ring is written by `push_tried`/`record_verdict`/`verdict_for`
(`E/⏳️precompute/🪣️fill/🦀️.rs:4393-4429`, wave A) and walked by `retire_fill_preview` as a real owner in the
admission census.

`H/🌐️World3dHost/🟦️.tsx`: `MeshStyleKind` (`:493`) = `"disabled" | "danger" | "celebrated" | "selected" |
"highlighted" | "hovered" | "neutral"` — **no `"success"` variant.** `MESH_STYLE_PAINT.danger` (`:513`) is
`{ fill: tokenVar("danger"), … opacity: 0.72 }`; `brushGhostPaint` (`:3725-3729`) is `collision → danger, else →
highlighted` (`highlighted` = `tokenVar("secondary")`, `:508` — a UI accent blue/teal, not a distinct "this
fits" green). `FillTriedGhosts` (`:3776-3798`) renders the WHOLE `tried` prop (≤ 12 entries) every re-render, each
faded by `sequence` distance from the newest down to `WORLD_FILL_TRIED_FAINTEST = 0.25` (`:3768`), collision in
danger red, `accepted`/`free` in `highlighted`, everything else muted. `FillDiagnosticOverlay` (`:3799+`) is the
text HUD (`tested · locked / requested`). There is no wgpu-native (non-React) render path for this — the wave-C
report lists only `H/🌐️World3dHost/🟦️.tsx`; the wgpu targets folder (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu`)
is where the FRAMEWORK's own `WindowMeasure::Progress`/`Number` widgets live (wave F), not fill's 3d ghost painting,
which is React/r3f-only.

### 3.5 Byte caps on the preview wire, and whether "all tested candidates" fits

`FILL_PREVIEW_JSON_MAX_BYTES` (`E/⏳️precompute/🪣️fill/🦀️.rs:36`) and `WORLD_FILL_PREVIEW_JSON_MAX_BYTES`
(`H/🌐️World3dHost/🟦️.tsx:403`) are both `16 * 1024` (raised from 4 KiB in wave A/C), a **hard refusal** past the
cap (one byte over ⇒ the whole preview parses to `null` client-side, per wave C §2). `WORLD_FILL_TRIED_MAX = 12`
(`:405`). Each `FillTriedCandidate.ghost` carries two full strings (`targetVortexFullId`, typically
`"<objectId>:<vortexIndex>"`, and `meshUrl`, typically a longer catalog path) plus `objectKindId`, 7 floats
(origin+orientation), verdict, optional reason — realistically 150–300 bytes of JSON per entry once escaped. 12
entries × ~250 bytes ≈ 3 KB, comfortably inside 16 KiB alongside the rest of the diagnostic (`candidatePage[8]`,
identity fields, etc.) — which is exactly why 12 was chosen and why it fits **today**.

**It does not scale to "all tested".** A fill run to the new default of 100, or a user-typed 5 000 (the count has
no ceiling per decision 1), tests many more candidates than it accepts — `preview.tested_count` in the wave-A
tests already exercises far past the ring width. At ~250 bytes/candidate, even 65 tested candidates alone would
blow the 16 KiB cap if sent as full `BrushPreviewState` records; a few hundred to a few thousand tested candidates
(the realistic range once the ceiling is gone) is one to two orders of magnitude over budget. And the wire is not
additive across ticks — each `fillBuildPreview` publish is a **full snapshot** of the current ring, not a delta;
whatever scrolled out of the 12-slot ring is gone from the wire forever, independent of the byte question. Two
separate problems, both blocking requirement (i) as currently wired: (a) per-candidate payload is too fat to repeat
hundreds/thousands of times per snapshot, (b) the transport model (full ring snapshot, not accumulate-on-client) throws
away history even where it *would* fit.

### 3.6 "Success" design token

Confirmed present and unused for this purpose: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🔣️.json:11` declares
`"success": "#737373"` (light) and the generated TS token table
(`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️tokens/🟦️.ts:10,1494`) carries `"success": "#7eb77f"` (a real
green, distinct from `danger`/`highlighted`/`secondary`). `tokenVar("success")` is therefore a valid, ready-to-use
CSS custom property — it is simply never referenced by `World3dHost/🟦️.tsx`'s `MeshStyleKind`/`MESH_STYLE_PAINT`
(`:493-517`), which has no `"success"` arm. Task 4(i)'s "fitting = success" is a genuinely new mapping to add, not
a rename of something that already exists under another name.

## 4. Requirement gap for the phase-4 asks

### (i) Show ALL tested meshes, colored by verdict (collision = danger, fitting = success), not a ring of 12

**What must change:**
- **Schema** (`A/🧬️schema/🦀️.rs`): the fixed `[Option<FillTriedCandidate>; 12]` ring (`:684`, `:785`) cannot carry
  "all" — replace the wire shape with a small, append-only **delta record** per candidate: not a full
  `BrushPreviewState` (7 fields incl. two strings), but `{ sequence: u64, kind_index: u16 (or a short interned id),
  verdict: u8, transform: compact pose }`. A `kind_index` into a small per-run kind catalog (already known
  client-side — `meshes: readonly WorldMeshRecord[]` is already threaded into `FillTriedGhosts`, `:3776`) removes
  the `objectKindId`/`meshUrl` strings entirely; `targetVortexFullId` is not needed for rendering (only for
  debugging) and can be dropped from the per-candidate record or moved behind a lower-priority/omit-when-tight
  budget rule like `truncated` already does for `candidatePage`.
- **Transport model**: stop publishing a full-ring snapshot every tick; publish only candidates **new since the
  client's last acknowledged `sequence`** (mirrors `candidatePage`'s fixed-8-per-page idiom, `E/⏳️precompute/🪣️fill/🦀️.rs`
  §"QueryBroadPhase"): the client sends (or the preview cursor tracks per-client-generation) a `sinceSequence`
  cursor, the server pages out up to N (8–16) new entries per tick, budget-checked the same way
  `FillPreviewJsonCursor`/`preview_json_step` already budget-check every other field (fuel/deadline discipline
  intact, no new control-flow shape). The **client** becomes the accumulator of "all tested" — it already has to
  hold state for `WorldInstancesLayer`; a parallel `Map<sequence, TriedRecord>` (or a flat typed array keyed by
  sequence, capped only by realistic run size) is cheap.
- **Rendering** (`H/🌐️World3dHost/🟦️.tsx`): replace `FillTriedGhosts`' one-`<BrushPreviewGhost>`-per-entry approach
  (fine at 12, wrong at hundreds/thousands — that component mounts a full r3f mesh per ghost) with **GPU
  instancing grouped by `(kind, verdict)`** — 2–3 instanced meshes per object kind (collision/free/testing), one
  draw call each, transforms uploaded as an instance-attribute buffer updated incrementally as new records arrive.
  This is the same pattern `WorldInstancesLayer` already uses for real locked objects; a `FillTriedGhosts`
  rewrite onto instancing is the natural sibling, not a new technique for this codebase.
- **Color mapping**: add `"success"` to `MeshStyleKind` (`H/🌐️World3dHost/🟦️.tsx:493`) and `MESH_STYLE_PAINT`
  (`:505-517`), e.g. `success: { fill: tokenVar("success"), line: tokenVar("success"), emissiveIntensity: 0.25,
  opacity: 0.55 }` (dimmer than `danger`'s 0.72 — a tried-but-not-selected free candidate is background
  information, not a call to action); update `brushGhostPaint`/the tried-ghost verdict switch
  (`:3725-3729`, `:3776-3798`) from the current binary `collision→danger, else→highlighted` to `collision→danger,
  free|accepted→success, testing→highlighted` (keep `highlighted` only for the single live in-progress candidate,
  which is a different signal — "being tested right now" — from "was free").

### (ii) Transaction: provisional until finalize, abort removes everything with no durable history, finalize only when complete

**What must change — this is the big one, and it reverses §3.3's current "lock = commit" architecture:**
- **A run needs an identity.** Nothing today marks "this object was placed by the run currently in progress."
  Add a run/operation id (the existing `(job, operation, generation)` triple already identifies the background job
  uniquely — reuse it, or mint a per-armed-tool "transaction id") and stamp it on every placement the run makes
  while provisional.
- **Provisional placements must not be `create_object`/`connect_vortices` document mutations.** The cleanest fit
  with "no hidden precompute, but no durable history either" is closer to the phase-3-deleted ghost-tail mechanism
  than to the current immediate-commit one — but keyed to "provisional run state," not "reveal cutoff": keep
  accepted candidates in `FillBuilder.appended_objects`/`sequence` (`E/⏳️precompute/🪣️fill/🦀️.rs`, `AcceptCandidate::Commit`)
  exactly as today (this part is fine and already resumable/RNG-prefix-safe per `audit-fill-pipeline.md` §6), but
  **stop** `take_locked_into_fixture`/`take_locked_mutations` from ever running during the "running" lifecycle
  phase. Render them the way `H/🌐️World3dHost/🟦️.tsx`'s (i)-instanced tried-ghosts do — as an ephemeral, richly
  colored (per (i)) overlay, not as real `WorldInstanceRecord`s — so they are visible, but nothing round-trips to
  document history while running.
- **`start`**: the existing `setActiveTool("fill")` arm is fine as the entry point; it should additionally open the
  transaction (mint the run id, reset the provisional accumulator).
- **`abort`**: today's `engagementAbort` (`E/🦀️.rs:7134`, `WindowTransient`-only lane) already has exactly the right
  shape for this — cancel the job, clear armed state — **and does not need to grow an `Artifact` lane at all** if
  provisional placements were never document mutations to begin with (fixing this gap the "no round-trip" way
  makes today's abort's inability to emit `Artifact` mutations a non-issue instead of the blocker it is today with
  immediate-commit). This also directly fixes test failures #2/#3 (§2): if commit no longer happens per-tick,
  "ready > 0 but no cancel identity" stops being observable because the provisional-vs-committed distinction is
  structural, not a race on `fill_job_identity()`.
- **`finalize`**: a NEW action (not `setFillCount`, which today does double duty as both "change the target" and
  "apply what's ready") that is enabled only when the run's own completion condition holds — `FillProgressSummary
  .done` (`applied`/`accepted` reaching `requested_count`, or a stall the user accepts) — and on invocation turns
  the WHOLE provisional accumulator into document mutations in one shot (or one bounded multi-tick drain, same
  `FILL_LOCK_PLACEMENTS_PER_TICK`-style chunking as today, but gated on "finalizing," not "every tick unconditionally").
  This is what recovers the interactive-budget regression (§2 test #1) for free too: `puzzle3d_action_document_intent`
  only needs to fire on ticks that are actually finalizing, not on every 120 ms tick of a run that might run for
  minutes.
- **UI gating**: `count_measure`'s `ready` (`E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:42`, currently "LOCKED = document
  count") needs to become "PROVISIONAL count" while running and only mean "document count" once finalized — and a
  `WindowMeasure` for `finalize` (disabled until `done`) needs to be added next to today's `progress_measure`'s
  `cancel`.

### (iii) start / abort / finalize controls

Given (ii)'s redesign, the pieces are:
- **start** — reuses `setActiveTool("fill")` (already exists).
- **abort** — reuses `engagementAbort` (already exists, already `Escape`-bound at `E/🦀️.rs:8396`); once (ii) lands,
  its existing `WindowTransient`-only contract becomes sufficient rather than a blocker.
- **finalize** — genuinely new action + measure (§ii). Needs: an `ActionDefinition` (pattern-match
  `E/🦀️.rs:8465`'s `engagementAbort` entry), a dispatch arm (pattern-match `:3566`), an
  `ArtifactToolPublicationContract` with the `Artifact` lane THIS time (it is the one action allowed to write the
  document), and a `WindowMeasure` (a plain enabled/disabled button-shaped measure — `WindowMeasure::Toggle` or a
  dedicated variant, `disabled: !progress.done`) surfaced next to `progress_measure` in
  `E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:93-98`.
- Puzzle 2d/5d (wave D's parity layer, `E2`/`E5`) will need the same three-action shape once 3d's is settled — out
  of scope for this fill-state audit, flagged for the sibling `📓️audit-p4-tool-inventory.md`.

## 5. Summary of concrete files touched by a phase-4 implementation (fill only)

| File | Change |
|---|---|
| `A/🧬️schema/🦀️.rs` | Replace/augment the 12-wide `tried` ring with a compact append/delta candidate record + run id |
| `E/⏳️precompute/🪣️fill/🦀️.rs` | Stop the ring-only publish; add cursor-paged "new since sequence" emission; add run/transaction id |
| `E/⏳️precompute/🦀️.rs` | `take_fill_locked_chunk` gated on "finalizing," not every tick; run-id plumbing; fix `fill_job_identity()` race (test #2/#3 root cause) |
| `E/🎮️commands/🪣️fill-build-tick/🦀️.rs` | Stop calling `take_locked_into_fixture` unconditionally; only on finalize |
| `E/🎮️commands/🧮️set-fill-count/🦀️.rs` | Keep as "change target," decouple from "apply" |
| new `E/🎮️commands/…/finalize-fill/🦀️.rs` (or similar) | The one action allowed to emit `create_object`/`connect_vortices` for a run |
| `E/🦀️.rs` | New action definition/dispatch/publication contract for finalize; `FillLock` stage semantics revisited |
| `E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` | `count_measure.ready` meaning split running-vs-finalized; new finalize measure |
| `H/🌐️World3dHost/🟦️.tsx` | `success` mesh-style token; instanced (not per-ghost-component) tried rendering; cursor-paged wire parsing instead of full-ring parsing |

Compile output: `🗑️generated/p4-fill-check.txt`. Test outputs: `🗑️generated/p4-fill-tests.txt` (killed early by the
15 s nextest watchdog), `🗑️generated/p4-fill-tests-cargo.txt` (full run, 95/100 pass, 5 fail),
`🗑️generated/p4-fill-tests-isolated.txt` (single-threaded re-run isolating the 3 real failures from the 1
parallelism artifact). All left in place per instructions.
