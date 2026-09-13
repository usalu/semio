# 🖥️ Wave C — viewport / UI

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`, master plan §1 decisions 3/4/5 and §2.5.
`E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
`H` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

---

## 1. Changes

### 1.1 Rust — terminology (`E/🗣️terminology/🦀️.rs`)

| Where | What |
|---|---|
| `🦀️.rs:28-42` | 15 new `app_labels!` cells, all four locale×terminology axes each: `fill_stage_preparing`, `fill_stage_selecting_target`, `fill_stage_selecting_candidate`, `fill_stage_testing_collision`, `fill_stage_locking`, `fill_stage_done`, `fill_stage_stalled`, `fill_stall_no_open_vortex`, `fill_stall_document_capacity`, `fill_stall_no_compatible_kind`, `fill_tested`, `fill_locked`, `fill_rejected`, `fill_collision`, `fill_requested`. |
| `🦀️.rs:172-196` `puzzle3d_fill_stage_label(labels, stage, stall_reason) -> String` | Folds the planner's 17 machine stage strings onto the seven phases a person can act on. `prepare-*` and `discard-tail` → *Preparing* (the `_` arm, so the mapping is total); `select-target` → *Selecting vortex*; `prepare-candidates`/`select-candidate` → *Selecting object*; `construct-preview`/`query-broad-phase`/`test-collision` → *Testing collision*; `accept-candidate` → *Locking*; `complete` → *Done*. A stall short-circuits the phase and reads `Stalled — <reason>`. |
| `🦀️.rs:201-209` `puzzle3d_fill_stall_reason_label` | `no-open-vortex` / `document-capacity` / `no-compatible-kind` localized; any other reason is surfaced verbatim (a visible machine token beats a silently wrong sentence). |

EN/DE, reuse-terminology aware: e.g. `fill_stall_no_open_vortex` reads "no open vortex" (native EN) /
"kein offener Vortex" (native DE) / "no open connection point" (reuse EN) / "kein offener Verbindungspunkt" (reuse DE).
There is no default locale: `puzzle3d_label_axes` still refuses any unauthored axis.

### 1.2 Rust — main window (`E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`)

| Line | What |
|---|---|
| `:4` | Module docstring: "reveal cutoffs" dropped from the interaction-channel list. |
| `:16` | Imports `puzzle3d_fill_stage_label`. |
| `:767-782` | `fillBuild` block is now `{count, appliedCount, requestedCount, tested, rejected, collisions, done, stage, stallReason}` (§2.5). `requestedCount` is `FillProgressSummary::max_count` (wave A redefined it as the requested count). **`stage`/`stallReason` travel as the planner's own machine identities, not localized text** — the localized sentence for the same state rides the ghost's `statusLabel`, so a probe reads identities on one channel and a person reads words on the other. This is why `world_interaction_json`'s signature did NOT have to grow a `labels` parameter (no churn for wave B2's callers/tests). |
| `:783` (was `:791`) | `"revealCutoffs": { "puzzle3d-fill": runtime.fill_count }` **removed**, with its comment. |
| `:858-871` `world_fill_preview_json` | Now derives the status label from the live summary: `puzzle3d_fill_stage_label(labels, progress.stage.as_str(), progress.stall_reason.as_deref())` instead of the fixed `labels.fill_progress`. |

Instances are untouched: `instance_record_json` (`:213-235`) still emits every `Puzzle3dObject` with
`disabled: object.locked`, so a locked piece renders as an ordinary (muted) document instance the moment
wave B2's `create_object` lands. Nothing else changed there.

### 1.3 Rust — fill tool measures (`E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, rewritten)

| Line | What |
|---|---|
| `:14-18` | `TOOL_ID` kept; new `FILL_PROGRESS_STEP_PAGE: usize = 4`. |
| `:32-46` `count_measure` | `WindowMeasure::Slider` → **`WindowMeasure::Number`**: `min: Some(0.0)`, `max: None`, `step: Some(1.0)`, `value: runtime.fill_count as f64` (no `.min(MAX)` clamp), `ready: Some(progress.applied_count as f64)` (the LOCKED count, so the soft extent never promises a piece the user cannot select), `loading: !done`, `on_change: setFillCount`. The `reveal: Some("puzzle3d-fill")` key is gone. `PUZZLE3D_FILL_COUNT_MAX` is no longer imported here. |
| `:52-65` `progress_steps` | Builds up to 4 `MeasureProgressStep`s from the summary counters: `locked · N` (Success), `collision · N` (Danger), `rejected · N` (Warning), `tested · N` (Info); a zero counter contributes no line. Words come from the terminology table (EN+DE). |
| `:73-89` `progress_measure` (was `cancel_measure`) | `WindowMeasure::Toggle` → **`WindowMeasure::Progress`**: `stage` = the localized `puzzle3d_fill_stage_label`, `completed` = `applied_count`, `total: Some(max_count)` (= requested), `steps` = the above, `cancel: Some(cancelFillBuild {job, operation, generation})`, `loading: Some(true)`. Measure id kept as `{PUZZLE3D_PLAY_CONTROLLER_ID}-fill-cancel` so existing assertions keep their handle. Still gated on a live `fill_job_identity()` and `!done`. |
| `:93-98` `measures` | Unchanged shape: count entry, progress row, distribution group. |

**Deviation from the brief, called out:** the brief's example step lines were `"collision · <kind>"` /
`"locked · <kind>"`. `FillProgressSummary` carries no per-candidate kind, and the brief also says the steps
are "built from the summary counters" — so the lines are `word · count`. A kind-per-line version needs the
tried ring plumbed into the session summary (wave A/B1 own that); it is a one-line change here once it is.

### 1.4 TS — `H/🌐️World3dHost/🟦️.tsx`

| Line | What |
|---|---|
| `:249-263` | `WorldFillBuildRecord` → `{count, appliedCount, requestedCount, tested, rejected, collisions, done, stage, stallReason}` (mirror of §2.5). |
| `:264-274` | `WorldInteractionRecord.revealCutoffs` **removed**. |
| `:326-334` | New `WorldBrushVerdict = "testing" \| "free" \| "collision" \| "rejected" \| "accepted"` + `WORLD_BRUSH_VERDICTS` set. |
| `:336-348` | `WorldBrushPreviewRecord` gains `verdict?: WorldBrushVerdict`. |
| `:350-357` | New `WorldFillTriedRecord = {sequence, verdict, reason?, ghost}`. |
| `:359-388` | `WorldFillDiagnosticRecord` gains `verdict`, `tried`, `testedCount`, `requestedCount`, `stallReason`; **`totalCount` removed** (contract §2.1 retires it in favour of `requestedCount`). |
| `:394-400` | `WORLD_FILL_PREVIEW_JSON_MAX_BYTES` **4 KiB → 16 KiB**; new `WORLD_FILL_TRIED_MAX = 12`. |
| `:401` | `WORLD_FILL_ROOT_KEYS` + `"verdict"`. |
| `:402-431` | `WORLD_FILL_DIAGNOSTIC_KEYS` + `verdict`/`tried`/`testedCount`/`requestedCount`/`stallReason`, − `totalCount`. |
| `:432-433` | `WORLD_FILL_GHOST_KEYS` + `"verdict"`; new `WORLD_FILL_TRIED_KEYS`. |
| `:476` | `MeshStyleKind` gains `"danger"`, second in priority (after `disabled`). |
| `:491` | `MESH_STYLE_PAINT.danger = { fill: tokenVar("danger"), line: tokenVar("danger"), emissiveIntensity: 0.35, opacity: 0.72 }`. |
| `:535-551` | `resolveMeshStyle` takes `danger?` and orders `disabled → danger → celebrated → selected → highlighted → hovered → neutral`. |
| `:1722-1755` | `parseWorldBrushPreview`: root `verdict` validated against the enum; a reusable `ghostPose()` predicate (own-key census against `GHOST_KEYS`, the six required pose fields, optional `verdict`) now validates BOTH `candidateGhost` and every `tried[].ghost`; `tried` must be an array of at most `WORLD_FILL_TRIED_MAX` entries, each `{sequence≥0, verdict ∈ enum, reason?: string\|null, ghost}` with no foreign keys; `verdict`, `testedCount`, `requestedCount`, `stallReason` validated; `totalCount` check dropped. |
| `:3693-3700` | New `brushGhostPaint(preview, palette)` — `collision` ⇒ danger style **and** the danger token as mesh colour (the kind hue is deliberately overridden: a refusal a viewer could read as "a piece of that kind goes here" is worse than no ghost); everything else keeps today's highlighted paint tinted by `preview.color`. |
| `:3702-3738` | `BrushPreviewGhost` takes an `opacityScale` (default 1), paints via `brushGhostPaint`, and passes `pickEnabled={false}` (that prop had become required and this call site was missing it — a pre-existing error in this file, fixed while here). |
| `:178` | `WorldInstanceRecord.revealIndex` **removed** — mid-session wave G stopped emitting it from `instance_record_json` and dropped it from both object fingerprints, so the wire no longer carries it and nothing on the host reads it. |
| `:3753-3769` | New **`FillTriedGhosts`** layer: renders the whole `tried` ring as ordinary ghosts — `collision` in danger red, `accepted`/`free` highlighted, everything else muted (`0.4×` vs `0.6×`) — each faded by its `sequence` distance from the newest (down to `WORLD_FILL_TRIED_FAINTEST = 0.25`). The ENTRY's `verdict` is spread onto the pose it renders (`{...entry.ghost, verdict: entry.verdict}`), so a producer that leaves the nested ghost unmarked still paints the right colour. |
| `:3765-3812` | `FillDiagnosticOverlay` reads `tested · locked / requested`, the producer's localized `statusLabel`, and `stallReason` when set (falling back to the rejection reason / collision count otherwise). New probe attributes (below); all previous `data-fill-*` kept. |
| `:3138-3178` (deleted) | The imperative reveal-cutoff effect and the committed-cutoff reconciliation in `WorldInstancesLayer`, plus the `revealCutoffs` prop and its type. |
| `:4196 / :4266 / :4382` | Three `isRevealCutoffHidden(...)` guards removed (marquee hit testing and the frame-visible-instances filter now see every instance). |
| `:7106` | `revealCutoffs={interaction.revealCutoffs}` prop removed from the `WorldInstancesLayer` call. |
| `:7124` | `FillTriedGhosts` mounted beside `BrushPreviewGhost`, gated on `fillMode && fillDiagnostic`. |

**Re-render cost:** everything new is read off `fillDiagnostic`, which is already derived from
`brushPreviewJson` inside the existing `puzzle3d_fill_build_scope()` dirty scope and already re-renders once
per 120 ms tick. `FillTriedGhosts` is a sibling of the ghost that tick already re-renders; no new state, no
new effect, no new subscription. Removing the reveal layer *removes* one `useLayoutEffect` + one store
subscription per instances change.

### 1.5 TS — reveal-cutoff removal outside World3dHost

Grep confirmed puzzle3d's fill slider was `RevealCutoffStore`'s **only** client (`PUZZLE3D_FILL_REVEAL_GROUP_ID`
was the only group id in the repo, and the only writer was `WindowMeasureSlider`'s `measure.reveal` branch),
so the whole mechanism is gone rather than left dangling:

- `H/🛠️ShellHelpers/🟦️.tsx` — the entire `//#region RevealCutoffStore` block deleted (`RevealCutoffStore`,
  `createRevealCutoffStore`, `worldRevealCutoffStore`, `PUZZLE3D_FILL_REVEAL_GROUP_ID`,
  `reconcileCommittedRevealCutoffs`, `isRevealCutoffHidden`). `world3dMarqueeOverlayShape`, which happened to
  live inside that region, is kept in place.
- `H/🛠️ShellHelpers/🟦️.tsx` `WindowMeasureSlider` — the `revealGroupId` branch removed: no `clampToReady`,
  no `onValueCommit`, no `onPointerCancel`; `onValueChange` dispatches directly again.
- `🧰️framework/…/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` — the six reveal symbols dropped from the import list and
  from the `export { … }` re-export line.

### 1.6 Files touched

```
✏️s/…/✏️editor/🗣️terminology/🦀️.rs
✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs
✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs
✏️s/…/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs
🧰️framework/…/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx
🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx
🧰️framework/…/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx
🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts
```

`E/🎭️modes/✏️edit/🪟️windows/🧊️main/🟦️.ts` needed no change: it mirrors the `render()` inputs
(instances/meshes/utility/options) and never carried the interaction block.

---

## 2. TS record shapes (the wire this host admits)

```ts
type WorldBrushVerdict = "testing" | "free" | "collision" | "rejected" | "accepted";

type WorldBrushPreviewRecord = {
  targetVortexFullId?: string; objectKindId?: string; sourceVortexIndex?: number; meshUrl?: string;
  origin?: readonly [number, number, number]; orientation?: readonly [number, number, number, number];
  scale?: readonly [number, number, number] | number; color?: string; opacity?: number;
  verdict?: WorldBrushVerdict;                    // absent ⇒ treated as "testing" / highlighted paint
  fillBuildPreview?: WorldFillDiagnosticRecord;
};

type WorldFillTriedRecord = { sequence: number; verdict: WorldBrushVerdict; reason?: string | null; ghost: WorldBrushPreviewRecord };

type WorldFillDiagnosticRecord = {
  operation; baseRevision; registryGeneration; sequence; generation;      // identity (unchanged)
  stage: string; statusLabel: string;                                     // machine phase + localized sentence
  targetVortexFullId: string | null; candidateObjectKindId: string | null;
  verdict: WorldBrushVerdict;                                             // NEW — of candidateGhost
  candidateGhost: WorldBrushPreviewRecord | null;
  tried: readonly WorldFillTriedRecord[];                                 // NEW — ≤ 12, compact (no nulls)
  testedCount: number; requestedCount: number;                            // NEW — totalCount removed
  stallReason: string | null;                                             // NEW
  currentPairObjectId; collisionCount; sampleCursor; insideBoth; lastSample;
  candidatePage: readonly (string|null)[] /* exactly 8 */; truncated; rejectionReason;
  targetCursor; candidateCursor; acceptedCount; searchCount; rejectedCount;
};

type WorldFillBuildRecord = {   // interactionJson.fillBuild
  count; appliedCount; requestedCount; tested; rejected; collisions; done: boolean;
  stage: string; stallReason: string | null;
};
```

**Producer contract (wave A):** `tried` must be a **compact** JSON array of entry objects — length 0…12,
no `null` holes (unlike `candidatePage`, which stays a fixed 8 with nulls). An array of 13, a `null` entry,
an unknown key, an unknown verdict string, or a non-`string|null` `reason` all make the whole preview parse
to `null` (i.e. the ghost and HUD vanish), so this is a hard refusal, not a lenient read.

Byte cap: `WORLD_FILL_PREVIEW_JSON_MAX_BYTES = 16384`, still a hard refusal one byte past it
(the contract test pins 16384 admitted / 16385 refused).

---

## 3. Probe surface — `data-fill-*` on `[role="status"]` (the fill HUD)

Existing, unchanged: `data-fill-operation`, `-base-revision`, `-registry-generation`, `-generation`,
`-sequence`, `-stage`, `-target-cursor`, `-candidate-cursor`, `-search-count`, `-rejected-count`,
`-sample-cursor`, `-inside-both`, `-current-pair`, `-has-ghost`, `-truncated`.

**New:**

| Attribute | Source | Use |
|---|---|---|
| `data-fill-tested` | `diagnostic.testedCount` | monotone growth proves the search is running |
| `data-fill-locked` | `diagnostic.acceptedCount` | monotone growth proves pieces are being committed |
| `data-fill-requested` | `diagnostic.requestedCount` | proves typing 250 / 40 reached the planner |
| `data-fill-verdict` | `diagnostic.verdict` | observing both `collision` and `free`/`accepted` is gate §4.6 |
| `data-fill-tried-count` | `diagnostic.tried.length` | proves the tried ring is populated (≤ 12) |
| `data-fill-stall-reason` | `diagnostic.stallReason` (omitted when null) | proves a capacity limit reads as a stall, never a fault |

Visible text is `statusLabel`, then `tested · locked / requested`, then the stall reason (or the rejection
reason / collision count); `aria-label` carries the same in order, so visible/ARIA parity holds.

---

## 4. Tests

### 4.1 Rust — `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs`

- `world_interaction_json_publishes_fill_counters_and_no_reveal_cutoffs` — asserts the `fillBuild` own-key
  set is exactly the nine §2.5 keys (sorted compare, so an extra or missing key fails), that `stage` is a
  string, `stallReason` is string-or-null, the three counters are `u64`, and that `revealCutoffs` is absent.
- `fill_stage_labels_answer_in_english_and_german_with_no_default_locale` — for six representative stages,
  both locales answer non-empty AND differ (an English fallback in the German cell fails the test);
  `discard-tail` and `prepare-fixture` fold to the same phase; the German stall sentence contains
  "kein offener Vortex"; the English one "document capacity reached"; an unlabelled reason is echoed verbatim.

Command:
```
cd ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust && \
  RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- <filter>
```

RESULT_RUST

### 4.2 TS — `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

Command (the nx target `@semio-tech/framework-renderer-react:test` shells out to the same thing; the nx
daemon was timing out on `REQUEST_PROJECT_GRAPH` in this session, so the package script was run directly):

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && \
  SEMIO_TEST_LEVEL=long bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" \
    "../../../../🧪️tests/🔬️engine-contract/🟦️.ts" --testNamePattern="<pattern>" --reporter=verbose
```

(`bun ./📜️script.ts test <file>` alone runs at the default `fundamental`/quick level, whose include list is
only `⚡️quick/🟦️.ts` — it finds no test files. `SEMIO_TEST_LEVEL=long` is what admits the engine-contract suite.)

Updated:
- `admits only the fixed localized fill diagnostic schema` — fixture grew `verdict`/`tried`/`testedCount`/
  `requestedCount`/`stallReason` and lost `totalCount`; the safe-integer loop covers the two new counters;
  the exact-byte-cap arithmetic moved 4096/4097 → **16384 admitted / 16385 refused**; the "full ghost root"
  key census 9 → **10** (root `verdict`), and an out-of-enum root verdict must parse to `null`.
- `renders the %s fill label with visible and ARIA parity` — fixture updated; the ARIA prefix assertion is
  now `"<label>; census; 0 · 0 / 1; testing;"`.

Added:
- `parses the tried-candidate ring and the per-candidate verdicts, and refuses an unbounded one` — a 12-entry
  ring parses (verdict, reason, per-entry ghost pose all readable); 13 entries, a `null` entry, a foreign key,
  an unknown verdict, a negative sequence, a malformed ghost pose, a numeric `reason`, an out-of-enum
  top-level `verdict` and a numeric `stallReason` each parse to `null`; a valid `stallReason` round-trips;
  a verdict on a tried ghost round-trips.
- `reads tested · locked / requested, the verdict and the stall reason off the fill overlay probe attributes`
  — renders `World3dHost` to static markup and asserts `data-fill-tested="37"`, `data-fill-locked="12"`,
  `data-fill-requested="250"`, `data-fill-verdict="rejected"`, `data-fill-tried-count="1"`,
  `data-fill-stall-reason="no-open-vortex"`, the German status sentence in the visible span, and `37 · 12 / 250`.

Removed: the `describe("reveal cutoff store", …)` block (4 tests) and its five imports — the functions it
covered no longer exist.

Whole-suite run (no name filter), to prove nothing else regressed:

```
SEMIO_TEST_LEVEL=long bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../../../🧪️tests/🔬️engine-contract/🟦️.ts"
  → Test Files 1 failed (1) · Tests 3 failed | 602 passed (605) · 37.42s
```

The 3 failures are pre-existing/peer-owned and each has a matching pre-existing typecheck error (§5.2):
`packValueFromBase64: expected pk: prefix` (`:7874`), `ReferenceError: panelTreePanelHost is not defined`
(`:8419`), and `transformMode "move" vs "transform"` (`:8945`). None touches fill, ghosts, verdicts or the
reveal cutoff.

```
--testNamePattern="fill"                 →  Test Files 1 passed (1) · Tests 15 passed | 590 skipped (605)
   ✓ admits only the fixed localized fill diagnostic schema                                          6ms
   ✓ reads tested · locked / requested, the verdict and the stall reason off the fill overlay probe attributes  25ms
   ✓ renders the Fill progress fill label with visible and ARIA parity                               4ms
   ✓ renders the Füllfortschritt fill label with visible and ARIA parity                             5ms
   (plus 11 pre-existing fill-named tests, all green)

--testNamePattern="tried-candidate ring" →  Tests 1 passed | 604 skipped (605)
   ✓ parses the tried-candidate ring and the per-candidate verdicts, and refuses an unbounded one     4ms
```

---

## 5. Compile / typecheck gates

### 5.1 Rust — gate §4.1: **PASS, 0 errors**

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --message-format short
  → warning: `semio-s-artifact-puzzle-3d` (lib) generated 96 warnings
  → Finished `dev` profile [unoptimized] target(s) in 8.03s
  → grep -c error = 0
```

96 warnings (baseline 97), all pre-existing `unnecessary qualification` / `never used` noise from peer waves.
Wave A's schema (`FillCandidateVerdict`, `FillTriedCandidate`, the extended `FillBuildPreview`, and
`FillProgressSummary { …, tested, rejected, collisions, stage, stall_reason }`) and wave F's
`WindowMeasure::{Number, Progress}` + `MeasureProgressStep{,Kind}` had both landed by the time this
compiled, so nothing here is coded against an absent contract.

### 5.2 TypeScript — `@semio-tech/framework-renderer-react:typecheck`

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
bun ./📜️script.ts typecheck        # == bunx tsc --noEmit -p tsconfig.json
```

850 errors repo-wide at the time of writing — **the tree is broadly red from concurrent peer work and
pre-existing debt, not from this wave.** Errors attributable to wave C: **0**. Breakdown of the files this
wave owns:

| File | Errors | Verdict |
|---|---|---|
| `🌐️World3dHost/🟦️.tsx` | 4 | all pre-existing and untouched by this wave: `:1475` `leftoverSelectIdsMustNameHoverPickV1` nullable arg, `:5144`/`:6865`/`:6870` `window.addEventListener` overloads. A 5th (`:3734` missing `pickEnabled` on `GlbInstanceMesh`, confirmed present in `git show HEAD:…`) **was fixed** by this wave. |
| `🛠️ShellHelpers/🟦️.tsx` | 1 | `:632` pre-existing `Uint8Array`/`BlobPart`. (A first pass showed 16 more at `:3342-3401` — the TS `WindowMeasure` union had no `"number"`/`"progress"` arm yet, so wave F's new render branches narrowed to `never`. Wave F regenerated the manifest mid-session and they cleared; `🎚️measure-controls/🟦️.tsx`'s 40 cleared with them.) |
| `🔬️engine-contract/🟦️.ts` | 14 | all pre-existing or peer-owned, at lines 535/5272/5273/5408/5420/5431/5433/7530/7637/8419/8870/10102(×2)/10831 — none inside this wave's added or edited blocks. `:10102` (`Property 'onChange' does not exist on type 'WindowMeasure'`) is new from wave F's union growing a variant without `onChange`; it is wave F's to narrow. |

---

## 6. Open issues / handoffs

1. **Wave F: manifest regenerated mid-session — one leftover.** The `"number"`/`"progress"` arms reached the
   TS `WindowMeasure` union while this wave was verifying, clearing 56 errors in `🛠️ShellHelpers`. What remains
   is `🔬️engine-contract/🟦️.ts:10102`, which reads `.onChange` off an unnarrowed `WindowMeasure` — the new
   `Progress` variant has no `onChange`. Wave F to narrow that call site.
2. **Wave F should retire `WindowMeasure::Slider.reveal`.** Its last consumer is gone (§1.5). The Rust field
   and its doc comment (which still points at the deleted `RevealCutoffStore` and `revealCutoffs`) remain.
3. **`reveal_index` — resolved mid-session, one remnant.** Wave G (editing the same window file concurrently)
   removed `revealIndex` from `instance_record_json` and from both object fingerprints; this wave then removed
   `WorldInstanceRecord.revealIndex` on the host, so the whole reveal channel is gone end to end. The only
   remnant is the `Puzzle3dObject::reveal_index` FIELD itself
   (`…/✳️any/🧬️schema/🦀️.rs:476-478`, still assigned in `⏳️precompute/🦀️.rs`) — waves A/B2 to retire with the
   ghost tail.
4. **Wave B2: the fill tool's own assertions.** `E/🧪️tests/🔬️unit/🦀️.rs:4481` and `:4649` still assert
   `find_measure_slider_max(tool_measures, "puzzle3d-fill-count") == PUZZLE3D_FILL_COUNT_MAX`. The measure is
   no longer a slider and has no max; those two assertions (and the `PUZZLE3D_FILL_COUNT_MAX` uses at
   `:4113`, `:4533`, `:4536`, and in `📨️engagement-submit` / `🔂️engagement-repeat-last`) are B2's to retire.
   `cancel_measure` was renamed to `progress_measure`; it had no callers outside this file.
5. **Wave G is publishing `suggestionMenu.progress`** (seen landing in `world_interaction_json` while this
   wave was editing the same file) and a `"verdict"` on the BRUSH ghost. The verdict is already admitted here
   — `WORLD_FILL_ROOT_KEYS` carries `"verdict"` and the parser validates it against the shared enum, so the
   brush ghost paints danger/highlight through the same `brushGhostPaint` — but `WorldSuggestionMenuRecord`
   does not yet declare `progress`. It is an ignored extra key today (that record has no own-key census);
   wave G should declare it if the popup is to read it.
6. **Step lines carry counts, not kinds** (§1.3) — needs a per-candidate kind on the session summary to match
   the brief's literal `"collision · <kind>"` example.
7. **Environment: the disk was full.** `/` had 377 MiB free at the start of verification (`ENOSPC` killed a
   background task output). `bun …/⚡️caching/📜️script.ts cache-prune` reported
   `cargo: 269.34 GiB over budget but touched within the last 48h; left in place` and freed 2 MiB — the 48 h
   guard is useless under a live fleet (`.🧬semio/🦑️repo/⚡️cache/cargo/build` is 349 GiB, `debug/build` alone
   187 GiB, `semio-s-artifact-stdio-semio` 53 GiB). Space was recovered by deleting
   `~/Library/Caches/Mozilla.sccache` (10 GiB, a stale sccache store this repo never uses — CLAUDE.md/memory
   forbid `RUSTC_WRAPPER`/sccache here), which brought `/` back to 101 GiB free. **No repo cache and no peer's
   build artifacts were touched.** The shared cache will hit the wall again; the prune policy's guard window
   needs a ticket.
8. **nx daemon.** `bun nx run @semio-tech/framework-renderer-react:typecheck` died with
   `NX The daemon timed out while processing REQUEST_PROJECT_GRAPH`. Every command in §4/§5 was therefore run
   through the package's own `📜️script.ts` / `bunx` directly, which is the same executable the nx target wraps.
