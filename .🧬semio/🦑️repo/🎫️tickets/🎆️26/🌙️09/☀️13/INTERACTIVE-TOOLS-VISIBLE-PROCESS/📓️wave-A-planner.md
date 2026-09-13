# 🪣️ Wave A — fill planner (requested-count planning, verdict stream, wire)

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`. Owner files only:
`E/⏳️precompute/🪣️fill/{🦀️.rs, 🧫️fixtures/🔣️.json, 🧪️tests/🔬️unit/🦀️.rs}` and the fill types in
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️.rs, 🔣️.json}`.
`E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 1. Schema (`🧬️schema/🦀️.rs`, `🧬️schema/🔣️.json`)

| What | Where |
|---|---|
| `pub const FILL_TRIED_RING: usize = 12` | `🧬️schema/🦀️.rs:684` |
| `pub enum FillCandidateVerdict { Testing, Free, Collision, Rejected, Accepted }` (`Default` = `Testing`, camelCase `value`/serde rename) | `🧬️schema/🦀️.rs:693` |
| `FillCandidateVerdict::wire() -> &'static str` — the single wire spelling both the hand-written encoder and every test read | `🧬️schema/🦀️.rs:705` |
| `pub struct FillTriedCandidate { sequence: u64, verdict, reason: Option<String>, ghost: BrushPreviewState }` | `🧬️schema/🦀️.rs:722` |
| `FillBuildPreview`: `total_count` **removed**, `requested_count: usize` added in its place | `🧬️schema/🦀️.rs:771` |
| `FillBuildPreview`: `verdict`, `tried: [Option<FillTriedCandidate>; FILL_TRIED_RING]`, `tested_count: u64`, `stall_reason: Option<String>` | `🧬️schema/🦀️.rs:781-793` |
| `FillProgressSummary`: `tested: u64`, `rejected: u64`, `collisions: u64`, `stage: String`, `stall_reason: Option<String>` added; `max_count` now means *requested* | `🧬️schema/🦀️.rs:815-825` |
| JSON schema: `Puzzle3dFillPreviewVerdict`, `Puzzle3dFillPreviewTried`, diagnostic `requestedCount`/`verdict`/`testedCount`/`stallReason`/`tried` (min/maxItems 12), root `verdict`, `x-semio-maxEncodedUtf8Bytes` 4096 → 16384 | `🧬️schema/🔣️.json` |

`FillBuildProgress` was left alone — it already carries the whole `preview`.
No other schema facet (`🟦️.ts`, `🔗️.graphql`, `🛰️.proto`) mirrors the fill preview types, so nothing else needed touching.

## 2. Builder (`E/⏳️precompute/🪣️fill/🦀️.rs`)

- `begin_preparation(roots, operation, requested_count)` — `🪣️fill/🦀️.rs:3205`. `max_count` and
  `preview.requested_count` are the requested value; `FILL_COUNT_MAX` is no longer imported or
  mentioned anywhere in this file (the doc comment on `preparation_capacity_refusal` was reworded).
- `requested_count(&self)` — `:3516`; `set_requested_count(&mut self, requested)` — `:3525`.
  - **Raising**: lifts `max_count`, and if the builder sits at `Complete` below the new target it
    clears `stalled`/`stall_reason` and rewinds the *stage* only, to `PrepareTargets`. The RNG state
    is untouched, so the longer plan keeps the shorter one as its exact prefix.
  - **Lowering**: sets `max_count` and, when `sequence.len() > max_count`, enters the existing
    `FillJobStage::DiscardTail` walk with no weight-replan semantics. A new field
    `tail_floor` (`:1211`) is what `discard_tail_one` stops at (`:3566`) — `begin_soft_replan`
    sets it to `applied_count` (`:3507`), a lowered ask to `max(requested, applied_count)` (`:3544`).
  - `discard_tail_one`'s finish branch now goes to `Complete` when the remaining plan already covers
    `max_count` (it previously always rewound to `PrepareTargets`, which would have kept planning
    past a lowered ask).
- Verdict stream, one writer each:
  - `push_tried` (`:4393`) — claims a ring slot for a constructed pose, `Testing`, bumps `tested_count`.
    Called from `construct_preview` right after `candidate_ghost` is published.
  - `record_verdict` (`:4406`) — the only rewriter; also counts `collisions`.
  - `verdict_for` (`:4419`) — `"solid-overlap"` ⇒ `Collision`, any other reason ⇒ `Rejected`.
    `reject_candidate`/`reject_target` route through it, so `test_collision`'s overlap arm produces
    `Collision` with reason `solid-overlap` and every other refusal produces `Rejected(reason)`
    without a second call site that could drift.
  - `test_collision`'s "no pair left" arm records `Free`; `AcceptCandidate::Commit` records `Accepted`.
  - `publish_preview` stamps the slot written this turn with the sequence it is actually published
    under (`tried_dirty`).
- Stalls (`stall()`, `:4429`; constants `:27-33`):
  - `no-open-vortex` — target preparation enumerated nothing (`:3909`) or the round had no targets.
  - `no-compatible-kind` — targets existed but no pose was ever built this round.
  - `no-free-placement` — poses were built and every one of them was refused.
    (**New, beyond the three named in the master plan** — see §6.)
  - `document-capacity` — a fixed document page refused an owner.
- **Capacity is now a visible stall, not a fault**: `capacity_diagnostic` (`:4553`) +
  `stall_on_capacity` (`:4562`) replace both `StepOutcome::Fault` arms
  (`fill-preparation-capacity`, `fill-fixed-collection-capacity`). The refusal still publishes one
  ghost-less diagnostic under its own preview sequence, now carrying
  `stall_reason = "document-capacity"`, then completes. `PreparationCapacityRefusal.diagnostic_published`
  was replaced by the builder-level `capacity_stall_published`. Faults remain for the genuine
  invariant break (`stale-fill-operation`) and for a failed preview-sequence allocation.
- Owner discipline: the ring is a real owner, so it is walked by the admission census
  (`preview_unit` section 3, `:2331`, with a new `FillDslOwnerRoot::TriedGhost(index)` for each
  entry's `scale` DSL) and retired one owner per call by `retire_fill_preview` (`:2547`).
  `stall_reason` is credited and retired alongside `rejection_reason`.

## 3. Wire (final shape)

`FILL_PREVIEW_JSON_MAX_BYTES` 4 KiB → **16 KiB** (`🪣️fill/🦀️.rs:36`); fixture `maximumBytes` 16384;
`x-semio-maxEncodedUtf8Bytes` 16384. One field per grant and the fuel/deadline/clock discipline are
unchanged — `fuel.checked_sub(1)`, the `FillPreviewJsonPass` field cursor and the census/reserve/
encode/validate phases are all still exactly as the policy tests read them.

```
{ "targetVortexFullId", "objectKindId", "sourceVortexIndex", "meshUrl", "origin", "orientation",
  "color", "opacity":0.35, "verdict":"testing|free|collision|rejected|accepted",
  "fillBuildPreview": {
    "operation","baseRevision","registryGeneration","sequence","generation","stage","statusLabel",
    "targetVortexFullId","candidateObjectKindId","candidateGhost","currentPairObjectId",
    "collisionCount","sampleCursor","insideBoth","lastSample","candidatePage","truncated",
    "rejectionReason","targetCursor","candidateCursor","acceptedCount",
    "requestedCount","searchCount","rejectedCount",
    "verdict","testedCount","stallReason",
    "tried":[ null | {"sequence":N,"verdict":"…","reason":null|"…","ghost":{…6 ghost fields…}}, … ]
  } }
```

Two decisions worth reading before wave C codes against it:
1. **`tried` is always exactly 12 elements**, empty slots publish `null` — same idiom as
   `candidatePage`'s fixed 8. It is a ring in *slot* order, not chronological order; order by
   `sequence` (each entry carries the preview sequence its verdict was published under).
2. **`reason` is always present**, `null` when there is none (the master plan wrote `"reason"?`).
   An always-present key is what an exact-key census on the TS side wants.
The root `verdict` is emitted only when a `candidateGhost` exists (same gate as the other root
ghost fields), so `World3dHost` can branch `danger` vs `highlighted` straight off the root object.

## 4. Tests (`🪣️fill/🧪️tests/🔬️unit/🦀️.rs`, `🪣️fill/🧫️fixtures/🔣️.json`)

The language-neutral fixture now carries: `limits.maximumBytes` 16384 and `limits.triedItems` 12;
`documentCapacities.fillCountMax` **removed**; `diagnosticNumericFields` `totalCount` → `requestedCount`
plus `testedCount` (15 entries); `boundaryLaws.verdicts`, `boundaryLaws.triedRing` (12 admitted / 13
refused), `boundaryLaws.requestedCount` (1 000 000 and 2^53−1, both admitted);
`boundaryLaws.aggregateSourceStrings` extended with `stallReason` and the four `tried[0].*` strings;
`boundaryLaws.fullWire` at 16384 / 16385. The fixture `preview` now holds a **full ring of 12**
entries covering all five verdicts, and both `locales.{en,de}.expected` were regenerated for the new
wire.

Updated (existing laws, same names unless noted):
- `retained_preview_json_matches_language_neutral_fixture_and_test_only_serde_oracle` — now also
  pins `Puzzle3dFillPreviewTried`/`Puzzle3dFillPreviewVerdict`, the 12-wide `tried` min/maxItems,
  and asserts the schema's verdict `enum` equals the owned `FillCandidateVerdict::wire()` spellings.
  Byte-for-byte parity against the serde oracle now covers a preview holding the full ring.
- `retained_preview_json_all_diagnostic_numeric_boundaries_are_preflighted` — `testedCount` and
  `requestedCount` boundaries added.
- `retained_preview_json_all_native_string_sources_enforce_wire_cap_before_mutation` — 9 → 14 string
  sources (`stallReason`, `tried[0].reason`, `tried[0].ghost.{targetVortexFullId,objectKindId,meshUrl}`).
- `constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently` and
  `capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_stalling` (renamed from
  `…_before_fault`) — the second step is now `StepOutcome::Complete` with
  `stall_reason == "document-capacity"`, not a fault.
- `document_capacities_match_the_language_neutral_capacity_law` — the `FILL_COUNT_MAX` clauses became
  "the flagship fixture leaves the object/attraction capacity room to plan into".
- `drive_preview_json`'s terminal-observation bound is now `4 * FILL_PREVIEW_JSON_MAX_BYTES`
  (census and encode are each one byte per grant, so a full-cap page needs > 32 k grants).
- `empty_builder()` and the four other `begin_preparation` call sites pass `TEST_REQUESTED_COUNT = 100`.

New:
- `raising_the_requested_count_continues_the_plan_as_an_exact_prefix` — plan(100) on the
  Nakagin-scale roots, `set_requested_count(150)`, assert the builder wakes at `PrepareTargets` with
  an untouched `rng_state`, drive to 150, and assert the resumed plan equals a fresh plan(150) and
  that plan(100) is its exact prefix.
- `lowering_the_requested_count_discards_the_planned_tail_and_raising_continues` — plan(100), lower
  to 60 ⇒ `sequence`/`appended_objects`/`appended_attractions` all 60, `placed_lookup` shrank by
  exactly 40 (every discarded placement withdrew its own spatial owner), stage `Complete`; then
  raise to 80 and keep planning.
- `every_tried_candidate_reaches_the_ring_with_its_own_verdict` — drives past the ring width and
  asserts every slot's verdict/reason pairing (`Collision` ⇒ `solid-overlap`, `Rejected` ⇒ some other
  reason, `Free`/`Accepted` ⇒ no reason, at most one `Testing` and it is the live ghost), that each
  entry carries a published sequence, and that accepted placements show up as `Accepted`.
- `retained_preview_json_tried_ring_and_unbounded_request_match_the_language_neutral_law` — ring of
  12 admitted / 13 refused by the schema itself, every verdict value round-trips byte-for-byte
  against the serde oracle, and an arbitrarily large `requestedCount` still encodes.

## 5. Verification

## 6. Open issues for other waves

**Wave B1 (session)** — already landed against this API while wave A was writing it
(`E/⏳️precompute/🦀️.rs` calls `begin_preparation(.., requested)`, `requested_count()`,
`set_requested_count()`), so no adaptation is owed. Two things to double-check on their side:
1. `FillProgressSummary` gained `tested`/`rejected`/`collisions`/`stage`/`stall_reason`. Fill them
   from `fill.preview.tested_count`, `fill.preview.rejected_count`, `fill.collisions` (a new
   `pub(crate)` counter on the builder), `fill.stage_label()` (now `pub(crate)`) and
   `fill.preview.stall_reason.clone()`. The observation-only fallback has no builder to read, so it
   should report zeroes plus whatever stage the envelope knows.
2. A capacity refusal no longer arrives as `StepOutcome::Fault` — it is one `PreviewReady` carrying
   `stall_reason = "document-capacity"` followed by `StepOutcome::Complete`. Any envelope path that
   latched a fault notice for `fill-preparation-capacity` / `fill-fixed-collection-capacity` now sees
   a normal completion instead, and `a_faulted_fill_envelope_latches_one_notice_and_never_silently_retries`
   needs a different fault to exercise (`stale-fill-operation` is the remaining genuine one).
   Note the admission census still **rejects** a builder whose fixed page overflowed, unchanged.

**Wave C (viewport/UI)** — the wire is in §3. Beyond `WORLD_FILL_PREVIEW_JSON_MAX_BYTES` 4096 → 16384:
1. `tried` is always exactly 12 entries with `null` holes, in ring-slot order — sort by `sequence`
   before rendering a feed.
2. `reason` is always present and may be `null` (the plan wrote `"reason"?`); an exact-key census
   should expect the key.
3. The **root** object carries `verdict` (only when `candidateGhost` exists), so the danger/highlight
   branch reads `parsed.verdict`, not just the diagnostic's.
4. `totalCount` is gone; the HUD denominator is `requestedCount`.
5. **Four** stall reasons ship, not three: `no-open-vortex`, `no-compatible-kind`,
   `no-free-placement`, `document-capacity`. `no-free-placement` is the common real case (poses were
   built, every one of them was refused) and the master plan's list was an "e.g." — it needs an
   EN+DE label like the others.

**Wave E (policy tests)** — three literals now disagree with production and must be retargeted:
- `📜️script.ts:9700` and `📜️script.ts:9766`:
  `"pub(crate) const FILL_PREVIEW_JSON_MAX_BYTES: usize = 4 * 1024"` → `16 * 1024`.
- `📜️script.ts:9852`: `fixture.limits?.maximumBytes !== 4096` → `!== 16384`.
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts:18` mutation
  `output-cap` and `:43` mutation `fixture-cap` carry the same two literals.
The guarded patterns those suites read (`fuel.checked_sub(1)`, the `now_us >= deadline` /
`checked_add(2_000)` clock arithmetic, `pub(crate) struct FillPreviewJsonCursor`, the
`candidate_page`/`truncated` publication clauses and the oracle test's name) were all left intact.
`📜️script.ts` never read `documentCapacities.fillCountMax`, so removing it breaks nothing there.

**Wave B1 / geometry** — `📐️geometry/🧪️tests/🔬️unit/🦀️.rs:506` still asserts
`DOCUMENT_OBJECT_SLOTS >= FILL_COUNT_MAX + 1024` and `📐️geometry/🦀️.rs:32` still documents the
capacity in terms of `FILL_COUNT_MAX`; both are wave B1's files and need the same rewording applied
here (the plan ceiling is now the document page, and overrunning it is a `document-capacity` stall).
