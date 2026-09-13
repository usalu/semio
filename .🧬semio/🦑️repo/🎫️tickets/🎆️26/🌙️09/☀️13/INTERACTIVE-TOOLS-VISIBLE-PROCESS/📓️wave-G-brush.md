# 🖌️ Wave G — brush suggestions as a visible process

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`.
`E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
`A` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. What was wrong

Hovering a vortex ran up to eight synchronous `refresh_brush_candidates` slices per 120 ms tick,
stopped the moment ONE collision-free candidate existed, and published only the finished `Select`
list. Everything in between — how many candidates exist, how many were tested, which one was refused
and why — existed exclusively as `[DEBUG] eprintln!` lines (`📓️audit-tool-inventory.md` §2 row
"3d brush", class C — hidden precompute).

## 2. The readout

### 2.1 Schema (`A/🧬️schema/🦀️.rs`)

Added in its own `//#region 🔖️BrushSearchProgress`, directly after `BrushCollisionFreeResult`:

```rust
pub struct BrushSearchProgress {
    pub target_vortex_full_id: String,
    pub tested: usize,          // free + blocked — candidates that reached a verdict
    pub free: usize,            // cumulative, deduplicated, survives a resumed pass
    pub blocked: usize,         // refused on overlap in this pass
    pub total_candidates: usize,// the whole compatible list
    pub done: bool,             // the pass exhausted the list with nothing owed
    pub current_candidate_kind: Option<String>,
    pub current_verdict: FillCandidateVerdict,   // testing | free | collision
    pub current_ghost: Option<BrushPreviewState>,
}
impl BrushSearchProgress { pub fn begin(target_vortex_full_id: &str) -> Self }
```

The verdict type is wave A's `FillCandidateVerdict` **on purpose** — one `"verdict"` wire spelling for
every ghost the viewport paints, so `testing`/`free`/`collision` mean the same thing whether the brush
or the fill lane produced them (master plan §2.1). The brush never reaches `Rejected`/`Accepted`:
accepting a candidate is a document mutation, not a search verdict.

`current_*` **survives completion** — a vortex where everything collided keeps publishing the refused
ghost instead of falling silently back to nothing.

### 2.2 Engine wire (`E/⏳️precompute/🦀️.rs`, brush-lane functions only)

| Where | What |
|---|---|
| `Puzzle3dCollision.brush_progress: HashMap<String, BrushSearchProgress>` | per-target readout, cleared wherever `brush_cache` is cleared or a target is invalidated |
| `Puzzle3dCollision::publish_brush_progress` | the ONE latest-wins publication point |
| `Puzzle3dCollision::brush_search_progress` | reader; an untouched vortex reads "not done", never "nothing here" |
| `Puzzle3dCollision::brush_collision_free_until` | updates the readout per slice: verdict set to `Testing` before the narrow phase, to `Collision`/`Free` after; also **deduplicates** `free` pushes, so a pass that restarts at 0 (a mesh arrived late) refines the same list instead of duplicating it |
| `Puzzle3dPrecomputeSession::brush_search_progress` | public reader for the window layer |
| `Puzzle3dPrecomputeSession::advance_brush_search` | the bounded per-tick driver (see §4) |
| `BRUSH_SEARCH_SLICES_PER_TICK = 8`, `BRUSH_SEARCH_WALL_BUDGET_US = 2_000` | the two bounds |

## 3. Streaming the result

- `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs` — new `brush_search_measures`
  pushes a `WindowMeasure::Progress` (framework contract §2.4) with `completed = tested`,
  `total = total_candidates`, a localized `stage` caption, `loading = !done` and **no cancel action**
  (leaving the vortex / closing the popup is the cancel). The placement `Select` appears with the
  FIRST free candidate, and its label repeats the count: `Placement — 3 free · 7 / 12 tested`.
  `brush_search_summary` / `brush_search_stage` are the two caption builders.
- `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` — `suggestionMenu` gained a `progress` block
  (`tested`, `free`, `blocked`, `totalCandidates`, `done`, `currentCandidateKind`, `currentVerdict`),
  machine identities only; the localized sentence for the same state is the measure's stage caption,
  matching the `fillBuild` convention wave C documented one block below it.
- `world_brush_preview_json` now publishes a `"verdict"` key on the ghost and picks WHICH ghost:
  1. while the search is running — the candidate under test (if its mesh is resident),
  2. else the settled pick (`free`),
  3. else the last tried candidate, so an all-collision vortex shows a `collision` ghost rather than
     nothing.

## 4. Budget

`advance_brush_search` replaces both hand-rolled eight-slice loops. It runs at most
`BRUSH_SEARCH_SLICES_PER_TICK` (8) slices, checks the wall clock between slices and stops past
`BRUSH_SEARCH_WALL_BUDGET_US` (2 000 µs — the audited user-visible lane wall,
`📓️audit-progress-primitives.md` §3.1; the hard interactive ceiling is 8 000 µs), and breaks the
moment the search is `done`, so a resolved target costs nothing per tick. Each slice keeps its own
500 µs `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US` deadline, so worst case is ~2.5 ms of a 120 ms tick.
It no longer stops at the first free candidate — watching the list fill in IS the feature.

## 5. Files changed

| File | Change |
|---|---|
| `A/🧬️schema/🦀️.rs:640` | `//#region 🔖️BrushSearchProgress` — `BrushSearchProgress` + `begin` |
| `E/⏳️precompute/🦀️.rs:28` | `BRUSH_SEARCH_SLICES_PER_TICK`, `BRUSH_SEARCH_WALL_BUDGET_US` |
| `E/⏳️precompute/🦀️.rs:~1727` | `brush_progress` field + init |
| `E/⏳️precompute/🦀️.rs:~2047/2140/2344` | `brush_progress.clear()` beside every `brush_cache.clear()` |
| `E/⏳️precompute/🦀️.rs:~2218` | per-object invalidation drops the readout too |
| `E/⏳️precompute/🦀️.rs:~2340` | `publish_brush_progress`, `brush_search_progress`, rewritten `brush_collision_free_until` |
| `E/⏳️precompute/🦀️.rs:~3137` | session `refresh_brush_candidates` (eprintln removed), `brush_search_progress`, `advance_brush_search`, `brush_preview` (eprintln removed) |
| `E/🎮️commands/⏱️suggestions-tick/🦀️.rs` | one `advance_brush_search` call; 4 `[DEBUG]` eprintlns removed |
| `E/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs` | invalidate + `advance_brush_search`; 4 `[DEBUG]` eprintlns removed |
| `E/🎮️commands/🔒️close-vortex-suggestions/🦀️.rs` | closing IS the cancel: drops the latched live target when brush is not armed |
| `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs` | `brush_search_measures`, `brush_search_summary`, `brush_search_stage` |
| `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` | `suggestionMenu.progress`, ghost `verdict` + ghost selection chain; 6 `[DEBUG]` eprintlns removed |
| `E/🗣️terminology/🦀️.rs` | `brush_search`, `brush_search_free`, `brush_search_tested`, `brush_search_blocked`, `brush_search_done` (EN + DE, native + reuse) |
| `E/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs` | `//#region 🔎️BrushSearchVisibleProcess` — 5 tests + fixtures |

## 6. Tests

`//#region 🔎️BrushSearchVisibleProcess` in `E/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs`.
Fixture: one host vortex, two compatible kinds; `blocker` parks a registered body exactly where both
candidates dock; `late_mesh` decides whether the second kind's geometry has arrived, which is how a
search is held open mid-list.

| Test | Asserts |
|---|---|
| `brush_search_progress_counts_every_free_candidate_and_flips_done_when_the_list_is_exhausted` | counters never regress across slices, `done` flips, `total_candidates == 2`, `free == 2`, `blocked == 0`, `tested == free + blocked`, verdict `Free` |
| `brush_search_progress_counts_blocked_candidates_as_a_verdict_not_a_silence` | `free == 0`, `blocked == 2`, verdict `Collision`, the refused ghost stays paintable |
| `brush_search_publishes_free_candidates_before_the_search_is_done` | with one mesh missing: `!done` yet `free == 1` and the picker's own cache list already holds it; after the mesh arrives the list GROWS to 2 instead of starting over |
| `brush_ghost_json_carries_the_verdict_of_the_candidate_it_shows` | `world_brush_preview_json` emits `"verdict": "free"` on a clear vortex and `"verdict": "collision"` on an all-blocked one |
| `brush_search_captions_answer_in_english_and_german_with_no_default_locale` | `3 free · 7 / 12 tested` / `3 frei · 7 / 12 getestet`, blocked segment, completion caption, EN ≠ DE |

RESULTS: see §8.

## 7. Open issues

1. **A permanently unresolvable target re-passes every tick.** When a candidate's mesh never arrives,
   every tick spends its whole 2 ms budget re-testing the same list (the `free` dedupe keeps the
   result correct, only the work repeats). The old code had the same shape; a "no forward motion
   twice in a row" stall guard was considered and dropped because the FIRST slice of a cold target
   legitimately makes no candidate progress (it reconciles the broad-phase index), so the guard would
   have cut warm-up short. Revisit if a profile shows it.
2. **`blocked` is per-pass, `free` is cumulative.** A pass that restarts at index 0 resets `blocked`
   while `free` carries. Counters are therefore monotonic WITHIN a pass, which is what the tests pin;
   a restart is honest ("re-testing"), but the HUD can show `blocked` dipping once if a late mesh
   forces a fresh pass.
3. **Host renderer.** The ghost's `"verdict"` key is published; painting `collision` with the danger
   token and `free`/`testing` highlighted is wave C's `H/🌐️World3dHost/🟦️.tsx` work.
4. **`suggestionMenu.progress` consumer.** The popup's React side has to read the new block; nothing
   breaks without it (additive key).

## 8. Verification

RESULTS PLACEHOLDER
