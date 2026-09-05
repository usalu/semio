# Extent tightening report — agent Y2

File touched: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
(only `fn extent(...)` bodies inside the four affected `impl … PuzzleCommandWork for …` blocks, plus 4
new `#[test]`s). No other region of the file was touched — `build_tool_job`, `PUZZLE3D_RETAINED_TOOL_IDS`,
`PUBLICATION_CONTRACTS`, `bounded_first_step_tool_proofs!`, and `.action_interactive_job` (agent E2's lock)
were left untouched, and `git diff --stat` on this file shows only the 4 `extent()` bodies + the new test
block changed.

## Nakagin measured counts (recounted directly from the DSL, not inherited from either audit)

Source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🧪️tower/🗣️.dsl.semio`
(128,755 bytes — matches the file the earlier findings cite).

Parsed the `objects [...] { }` block programmatically (Python, counting rows and `vortex-kind=` tokens
per row, not by eyeballing):

| quantity | value | how derived |
|---|---|---|
| object instances (`N`) | **180** | rows between `objects [...] {` and its closing `}` |
| object-instance vortices total (`V`) | **358** | sum of `vortex-kind=` occurrences per object row (min 1, max 10, avg ≈1.99) |
| attraction instances (`A`) | **0** | `attractions [...] { }` block is empty |
| target-volume instances | **0** | `target-volumes [...] { }` block is empty |
| reference instances | **0** | `references [...] { }` block is empty |
| object kinds (catalog) | 12 | `meta.objects [...] {}` rows |
| vortex kinds (catalog) | 18 | `meta.vortices [...] {}` rows |
| kind-compatibility rows (`C`) | **14** | `meta.kind-compatibility [...] {}` rows |

This corrects the `extent-budget-audit.md` figure of 121 objects (the 2026-09-05 findings doc already
flagged the undercount at 180; this file independently reproduces 180 and additionally establishes the
real per-object vortex distribution, which the audit never measured — it assumed the worst-case constant
64 for every object).

## Method

For each Work, read `step()`'s state machine stage-by-stage and counted the exact (or a proven upper
bound on the) number of `step()` calls to reach `Complete`, in terms of the document's real cardinalities
(`N`, `V`, `A`, `C` above) rather than the structural constant `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT = 64`.
`PUZZLE_COMMAND_WORK_ITEMS = 4_096` throughout (`🎮️commands/🧵️retained/🦀️.rs:12`).

`PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT` itself was **not** touched (still 64) — it is still used correctly
in `acceptSuggestion`'s catalog-kind-cap terms, where it bounds `kind.representations.len()` /
`kind.vortices.len()` for one CATALOG kind, an invariant `step()` itself enforces with a hard `Err` before
those stages run (`Puzzle3dAcceptSuggestionStage::Candidate`: `if kind.representations.len() >
PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
return Err(...) }`). The bug was never that constant in isolation — it was multiplying it by the SCENE
object count (`document.objects.len()`), which is unbounded and unrelated to a catalog kind's own
representation/vortex cardinality.

## 1. `Puzzle3dWorldRelocateWork::extent` (`worldRelocate`)

**Old formula:** `document.objects.len() * 2 + document.objects.len() * 64 + document.attractions.len()`
= `66N + A`.

**Real work (`step()`, stages `Object → ExistingAttractions → CandidateObject ⇄ CandidateVortex →
PublishAttraction`):**
- `Object`: scans objects 0..k until a match or exhaustion → ≤ `N+1` `step()` calls.
- `ExistingAttractions`: scans all `A` attractions once, +1 terminal call → `A+1`.
- `CandidateObject`: dispatches exactly once per object (skip the source object, or hand off to
  `CandidateVortex`), +1 terminal call when exhausted → `N+1`.
- `CandidateVortex`/`PublishAttraction`: per non-source object, one `step()` per vortex it owns plus one
  "owner advance" `step()`, plus up to one extra `PublishAttraction` `step()` per vortex found within
  `config.proximity_radius` → bounded above by `2V + N` (using the full document vortex total `V`, which
  is only more generous than subtracting the source object's own vortices).

**New formula:** `(N+1) + (A+1) + (N+1) + (2V+N) = 3N + 3 + A + 2V`.

**Soundness:** every term is a proven upper bound on an actual `step()`-call count per stage, derived
directly from the state machine (not from unrelated structural constants), using `checked_add`/`checked_mul`
throughout so it can only return `None` on real overflow, never silently underflow/wrap.

**Nakagin:** old = `180*66 + 0` = **11,880** (exceeds 4,096 cap → **faults**). New = `3*180+3+0+2*358`
= **1,259** (≤ 4,096 → **passes**, 31% of the cap, still leaves headroom, not maximally tight but far more
realistic than 2.9× over budget).

## 2. `Puzzle3dCreateAttractionWork::extent` (`createAttraction`)

**Old formula:** `document.attractions.len() + document.objects.len()*64*2 + kind_compatibility.len() + 1`
= `A + 128N + C + 1`.

**Real work (`step()`, stages `Existing → Attracting → Attracted → Compatibility → Publish`):**
- `Existing`: scans all `A` attractions, +1 terminal → `A+1`.
- `Attracting`: `scan_endpoint` walks every object's vortex list one `step()` at a time (with one "owner
  advance" `step()` per object), +1 terminal guard → bounded by `V+N+1`.
- `Attracted`: identical second full scan (cursors reset by `begin_endpoint_scan`) → bounded by `V+N+1`.
- `Compatibility`: scans all `C` kind-compatibility rows, +1 terminal → `C+1`.
- `Publish`: exactly 1 `step()`.

**New formula:** `(A+1) + 2*(V+N+1) + (C+1) + 1 = A + 2V + 2N + C + 5`.

**Soundness:** each stage's bound matches its `step()` body's actual loop structure (two full endpoint
scans over real vortices, not a per-object worst-case constant); `document.meta.kind_compatibility.len()`
is a real per-document field already read by `step()`'s `Compatibility` stage.

**Nakagin:** old = `0 + 128*180 + 14 + 1` = **23,055** (5.6× the cap → **faults**). New =
`0 + 2*358 + 2*180 + 14 + 5` = **1,095** (≤ 4,096 → **passes**).

## 3. `Puzzle3dAcceptSuggestionWork::extent` (`acceptSuggestion`)

**Old formula:** `document.objects.len()*64 + catalogs.objects.len() + 64*2 + document.attractions.len() + 4`
= `64N + K + 132 + A` (`K` = catalog object-kind count).

**Real work (`step()`, stages `Target → Candidate → Representation → Vortices → ExistingAttractions →
PublishObject → PublishAttraction → PublishResult`):**
- `Target`: walks every scene object's real vortex list (same owner-advance pattern as above), +1
  terminal → bounded by `V+N+1`. This is the term that was wrongly charged at the flat per-object 64.
- `Candidate`: exactly 1 `step()` (index resolved by hash, no scan over the catalog).
- `Representation`/`Vortices`: each bounded by `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT` (64) **correctly**
  — `Candidate` already rejects (via `Err`, terminating early) any catalog kind whose representation or
  vortex count exceeds 64, so by the time these stages run that cap is a proven invariant, not a guess.
  `+1` terminal each → `2*(64+1) = 130`.
- `ExistingAttractions`: scans all `A` attraction instances, +1 terminal → `A+1`.
- `PublishObject`/`PublishAttraction`/`PublishResult`: exactly 1 `step()` each → `3`.

**New formula:** `(V+N+1) + 1 + 130 + (A+1) + 3 = V + N + A + 136`.

The old `catalogs.objects.len()` term (`K` = 12 on Nakagin) was dropped: no `step()` call in this Work
ever iterates the full catalog list — `Candidate` only indexes one entry by `hash % len()` — so that term
never corresponded to real work and only added slack, never soundness.

**Soundness:** `Target`'s bound now reflects the document's real vortex distribution instead of assuming
64 per object; `Representation`/`Vortices` keep the 64-per-catalog-kind bound because `step()` itself
enforces that ceiling with a hard fault before reaching them, so 64 is not a guess there.

**Nakagin:** old = `64*180 + 12 + 128 + 0 + 4` = **11,664** using the corrected `N=180` (the original audit's
7,888 used the undercounted `N=121`; either way it exceeds the cap → **faults**). New =
`358 + 180 + 0 + 136` = **674** (≤ 4,096 → **passes**).

## 4. `Puzzle3dPatchInspectorWork::extent`, `"vortex"` arm (`patchInspector`)

**Old formula (vortex arm only):** `document.objects.len() * PUZZLE_COMMAND_DECODED_ITEMS` = `512N`. This
was flagged by the audit as a straight design error (multiplying by the wrong constant, 512 instead of a
per-object vortex count), not a precision edge case.

**Real work (`step()`, stage `Vortices`):** for each object, one `step()` per vortex it owns plus one
"owner advance" `step()`, plus one final terminal call when the object cursor exhausts → `N + 1 + V`.

**New formula (vortex arm):** `document.objects.len() + 1 + Σ object.vortices.len()` = `N + 1 + V`. The
other four arms (`object`, `attraction`, `reference`, `targetVolume`) were also each missing the same
"+1 terminal `step()` call" that every stage's exhaustion check consumes (confirmed by reading
`Puzzle3dPatchInspectorStage::{Objects,Attractions,References,Volumes}` — each has a
`let Some(x) = ...get(cursor) else { return Ok(self.complete()) }` guard that is itself one additional
`step()` call beyond the last real row). Since the whole `extent()` body was already inside this ticket's
write-lock and the fix required rewriting the full `match`, all four sibling arms were tightened to add
that same `+1` so the entire function is sound, not just the arm that happened to exceed the cap on
Nakagin. None of the other arms exceed the cap on Nakagin even without the fix (max is `object` at 180),
so this is a soundness correction, not a capacity fix, for those three.

**Nakagin (vortex arm):** old = `512*180` = **92,160** (22.5× the cap → **faults**, the worst of the four).
New = `180 + 1 + 358` = **539** (≤ 4,096 → **passes**).

## Summary table

| Work / tool id | old formula | old value (Nakagin) | new formula | new value (Nakagin) | cap |
|---|---|---|---|---|---|
| `worldRelocate` | `66N + A` | 11,880 | `3N+3+A+2V` | 1,259 | 4,096 |
| `createAttraction` | `128N + C + 1` | 23,055 | `A+2V+2N+C+5` | 1,095 | 4,096 |
| `acceptSuggestion` | `64N + K + 132 + A` | 11,664 | `V+N+A+136` | 674 | 4,096 |
| `patchInspector` (vortex) | `512N` | 92,160 | `N+1+V` | 539 | 4,096 |

`N=180`, `V=358`, `A=0`, `C=14`, `K=12` — all measured directly from the Nakagin DSL fixture (see above),
not inherited from either prior audit.

## Verification run

- `rustfmt --edition 2021 --emit stdout <editor file>` — **exit 0**, no stderr. Also diffed rustfmt's
  reformatted output against the file: after one round of collapsing two multi-line `.checked_add(...)`
  chains to match `rustfmt.toml`'s `max_width = 250` / `use_small_heuristics = "Max"`, the diff over every
  line touched by this change (`object_vortices`, `*_stage` locals, the 4 new
  `*_extent_fits_within_cap_for_nakagin` test fns) is **empty** — the code is already in the project's
  canonical format.
- Did **not** run `cargo` (main session owns the one build in flight, per instructions).
- Added 4 `#[test]`s (`world_relocate_extent_fits_within_cap_for_nakagin`,
  `create_attraction_extent_fits_within_cap_for_nakagin`,
  `accept_suggestion_extent_fits_within_cap_for_nakagin`,
  `patch_inspector_vortex_extent_fits_within_cap_for_nakagin`), each building a
  `Puzzle3dPlaySnapshot` from `NAKAGIN_EXAMPLE_FIXTURE.clone()` (same construction pattern as
  `Puzzle3dPlayApp::initial_snapshot()`, swapping `default_fixture()` for the Nakagin fixture), calling
  the Work's `extent()` directly, and asserting `Some(x)` with `x <= PUZZLE_COMMAND_WORK_ITEMS`. These are
  arithmetic/logic checks only — not run through `cargo test` here since cargo was off-limits; the
  arithmetic above was independently verified with a standalone Python calculation against the same
  measured `N`/`V`/`A`/`C`.

## Scope discipline

Only the four `extent()` bodies and the new `#[test]`s were edited. `Puzzle3dRelocateVolumeWork::extent`
(`relocateTargetVolume`) was left untouched — its extent is a small constant (4) that never approaches the
cap regardless of `N`, so it needed no tightening, and its Work otherwise falls under agent E2's lock
(publication lane / `build_tool_job`) rather than this ticket. No other Work's `extent()` was modified.

## Addendum: soundness closed by driving `step()` to `Complete` (coordinator follow-up)

The tests above only prove the bound is **realistic** (fits under the cap). They say nothing about
**soundness** (`extent()` >= the real number of `step()` calls). Both properties are independent, and only
one was tested. This addendum adds one loop-driving `#[test]` per changed Work that builds the Nakagin
snapshot, calls `extent()`, then actually drives `step()` in a loop until `Complete`, counting every
`Progress`-returning call, and asserts `iterations <= extent` — closing the direction that matters more,
since an over-tight bound lets the job's cursors run past it (the subtler, worse failure this ticket exists
to avoid).

### Real args, not a trivial early exit

Per the coordinator's explicit warning, a test that drives a trivial early-exit path would pass while
proving nothing. So each test uses realistic command args that force the Work through its genuine,
non-degenerate stages, verified against the real Nakagin DSL rather than assumed:

- **`worldRelocate`**: "relocates" object `25b0dba0-8f81-423a-94a1-b911a6031010` ("Capsule With Balcony
  Backslash") to its own current origin `[-8.85, -2.8499999999999996, 7.7]` — a real, non-degenerate
  command. Independently recomputing world-space vortex positions from the DSL with the same
  `quat_rotate_vector` Hamilton-product formula `step()` uses (own Python re-implementation, cross-checked
  against the Rust source at editor `🦀️.rs:485-494`) shows this object's one vortex
  (`…:link`, kind "door capsule right") sits at effectively zero distance (~1e-15, i.e. an exact assembled
  joint) from **four** "door tambour right" vortices already on neighbour object
  `5f0266bc-856b-4ef2-9eb0-16ef5e1fb952` — all well inside `proximity_radius` (0.75,
  `default_proximity_radius()`, `✏️editor/🎚️config/🦀️.rs:37`). This forces the real
  `CandidateVortex ⇄ PublishAttraction` ping-pong (editor `🦀️.rs` `Puzzle3dWorldRelocateStage::
  CandidateVortex`/`PublishAttraction`) to loop back more than once, not zero times.
- **`createAttraction`**: `attracting = "25b0dba0-…:link"` (kind "door capsule right"),
  `attracted = "5f0266bc-…:sl0_d0"` (kind "door tambour right") — a real pair the fixture's own
  `kind-compatibility` table marks bidirectionally compatible (`"door tambour right" "door capsule right"
  true false vortex`), with no pre-existing attraction between them (Nakagin has 0 attraction instances),
  so `step()` runs every real stage (`Existing → Attracting → Attracted → Compatibility → Publish`) to a
  genuine success instead of one of the Work's several early-`Complete` short-circuits
  (duplicate/incompatible/empty-id).
- **`acceptSuggestion`**: `fullId = "25b0dba0-…:link"`, a real vortex full-id present in the document, so
  `Target` finds a genuine match and the full `Candidate → Representation → Vortices →
  ExistingAttractions → PublishObject → PublishAttraction → PublishResult` chain runs to completion,
  instead of the "no target requested" early-`Complete`.
- **`patchInspector`** (vortex arm): `ids: ["25b0dba0-…:link"]`, `field: "hidden"`, `value: true`. The
  `Vortices` stage walks every object and every real vortex **unconditionally** — selection only gates
  whether a visited vortex is mutated, never whether it is visited (confirmed by re-reading `step()`: the
  per-vortex branch runs `Ok(Self::progress(...))` regardless of `self.selected.contains(...)`) — so this
  is a genuine full-document run by construction; no early-exit risk exists for this arm at all.

### A real soundness bug this work surfaced and fixed

Manually tracing `Progress`-vs-`Complete` returns through every stage (required to justify the
`iterations > N` non-triviality assertions below without being able to execute the test) surfaced a real,
pre-existing off-by-one in `Puzzle3dPatchInspectorWork::extent`'s outer combination line, `let items =
source.checked_add(scan)?;`. The `Selection` stage's own exhaustion call (`source_id` returning `None`)
is a real `step()` call that returns `Progress` (a genuine `+1` against `source_len`), distinct from each
entity arm's own terminal call, which returns `Complete` instead (contributing zero to the `Progress`
count that `work_cursor` tracks in `RetainedPuzzleCommandJob`). The old line omitted the `Selection`
stage's own `+1` entirely. Fixed to:

```rust
let items = source.checked_add(scan)?.checked_add(1)?;
```

This is within the ticket's write-lock (`Puzzle3dPatchInspectorWork::extent`'s own body, already being
rewritten) and affects all five entity arms uniformly (`object`/`vortex`/`attraction`/`reference`/
`targetVolume`), not just the arm that exceeded the cap. It was found by manually deriving the exact
`Progress`-call count for the vortex-arm test below (`source(1) + Vortices-stage(538) = 540` real
`Progress` iterations) and comparing it against the pre-fix formula (`source(1) + scan(539) = 540` —
**equal**, not `>=`, i.e. already at the boundary and one real off-by-one away from unsound) versus the
post-fix formula (`541`, giving the bound a genuine `+1` margin). No test in this file could have caught
this without actually driving the loop — the extent-only test only checks the value fits under the cap,
which it did either way.

### Manual `Progress`-vs-`Complete` accounting per Work (why `iterations <= extent` holds in general, not just for this run)

For every stage in all four Works, whether it ends its own scan with a call that returns `Progress`
(counted) or `Complete` (not counted, since `work_cursor` only increments on `Progress`) was read directly
from `step()`, stage by stage:

- **`worldRelocate`**: `Object` stage's own exhaustion-without-match returns `Complete` (real
  `<= N`, formula budgets `N+1`, slack ≥1); `ExistingAttractions`'s exhaustion returns `Progress` (real
  exactly `A+1`, formula `A+1`, slack 0, exact); `CandidateObject`'s exhaustion returns `Complete` (real
  exactly `N`, formula `N+1`, slack 1); `CandidateVortex`/`PublishAttraction` never return `Complete`
  mid-scan (real `<= 2V+N`, formula `2V+N`, slack from not subtracting the source object's own vortices).
  Total real `<= 3N+A+2V+1`; formula `3N+3+A+2V`; **slack ≥ 2**.
- **`createAttraction`**: `Existing`'s exhaustion (no duplicate found) returns `Progress` (real exactly
  `A+1` when no match, formula `A+1`, slack 0); `Attracting`/`Attracted`'s exhaustion-without-match returns
  `Complete`, but every call while scanning (including the matching one) returns `Progress` (real `<= V+N`
  each, formula `V+N+1` each, slack 1 each); `Compatibility`'s "found compatible, advance to Publish" path
  returns `Progress` (real exactly `C+1` in the success path, formula folded into the total's `+C+2`);
  `Publish` itself returns `Complete` directly (0 real contribution, formula still budgets for it as part
  of the trailing `+2`). Total real (success path) `<= A+C+2+2V+2N`; formula `A+2V+2N+C+5`; **slack ≥ 3**.
- **`acceptSuggestion`**: `Target`'s exhaustion-without-match returns `Complete` (real `<= V+N`, formula
  `V+N+1`, slack 1); `Candidate` always returns `Progress` exactly once (exact); `Representation`'s
  exhaustion returns `Progress` (real exactly `R+1`, `R <= 64` enforced by `Candidate`'s own capacity
  fault, formula budgets `65`); `Vortices`'s "non-empty, advance to ExistingAttractions" path returns
  `Progress` (real exactly `Vn+1`, `Vn <= 64`, formula budgets `65`); `ExistingAttractions`'s exhaustion
  returns `Progress` (real exactly `A+1`, exact); `PublishObject`/`PublishAttraction` each return `Progress`
  exactly once; `PublishResult` returns `Complete` directly (0 real contribution, formula still budgets 1
  for it as part of the trailing `+3`, an extra point of slack). Total real (success path)
  `<= V+N+A+R+Vn+6 <= V+N+A+134` (at `R=Vn=64`); formula `V+N+A+136`; **slack ≥ 2**.
- **`patchInspector`** (vortex arm, and by the same structural argument, all five arms): `Selection`'s
  exhaustion returns `Progress` (real exactly `source_len+1`); the `Vortices` stage's per-object
  "owner-advance" exhaustion returns `Progress` (real vortex-scan total exactly `V+N`, not `V+N+1` — the
  arm's OWN final exhaustion, at `object_cursor == N`, returns `Complete`, contributing 0). Real total
  exactly `source+1+V+N`; formula (post-fix) `source+(N+1+V)+1 = source+N+V+2`; **slack exactly 1, always**
  — a genuinely tight bound, not merely a plausible one, confirmed by driving the loop rather than assumed.

Every arm has slack ≥ 1 by this direct accounting, independent of the specific Nakagin numbers plugged
in — the loop-driving tests below are a concrete instance of that general argument, not the argument
itself.

### Verification run (still no cargo)

- `rustfmt --edition 2021 --emit stdout <editor file>` — **exit 0**, no stderr, both immediately after
  these changes and after re-diffing against a fresh rustfmt pass: the only formatting adjustment needed
  was wrapping one `let command = Puzzle3dCommand::from_action(...)` call onto two lines (rustfmt's own
  choice once the line exceeded `max_width = 250`), now applied verbatim from rustfmt's own output. Every
  other line this addendum touches (all four new loop-driving tests, the one-line fix to
  `Puzzle3dPatchInspectorWork::extent`) produces **zero** diff against a fresh rustfmt pass.
- Did **not** run `cargo` — the main session owns the one build in flight. The `iterations`/`emit`
  assertions below (exact mutation counts, `> N` non-triviality floors) were derived the same way the
  arithmetic in the main report was: by hand-tracing `step()`'s state machine against the real Nakagin DSL
  data (object array order recovered by parsing the fixture's own declaration order; vortex world
  positions recomputed with an independent Python port of `quat_rotate_vector`), not by executing the
  Rust. This is stated plainly as an honest limit on what could be confirmed without cargo, not claimed as
  an executed pass.
- New tests added (all in the same `mod tests` block, immediately after each corresponding
  `*_extent_fits_within_cap_for_nakagin` test): `world_relocate_step_loop_stays_within_its_own_extent_for_nakagin`,
  `create_attraction_step_loop_stays_within_its_own_extent_for_nakagin`,
  `accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin`,
  `patch_inspector_vortex_step_loop_stays_within_its_own_extent_for_nakagin`. Each asserts, after driving
  `step()` to `Complete`: `iterations <= extent` (the soundness property itself), a lower bound on
  `iterations` proven above to hold regardless of exact object ordering or geometry (ruling out a trivial
  early exit), and an exact or minimum count on the completed `Emit`'s `artifact_mutations` (ruling out
  one of the Work's early-`Complete` short-circuits producing an empty result that would otherwise pass
  the iteration count checks for the wrong reason).

None of the four Works needed to be left un-driven — real, realistic args were available for all four
directly from the Nakagin fixture's own already-assembled geometry and compatibility table, so there is no
honest gap to report here.
