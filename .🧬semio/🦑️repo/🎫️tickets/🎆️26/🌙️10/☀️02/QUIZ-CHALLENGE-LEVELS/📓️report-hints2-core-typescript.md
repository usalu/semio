# Report: hint revision round 2, schema and TypeScript core (design §8.4a)

Scope: §8.4a items 1 (verdict), 4 (short labels in the contract and the sheet), 5 (familiar references), 6 (profile hints relative to items), 8 (cap) and 9 (wire version).

The schema and the TS twin were landed first. The readings everyone shares are in `📓️hints-schema-landed.md`, section "Round 2".

Abbreviation: `Q` = `🧰️framework/🛍️products/❓️quiz`.

## Changed files

### Updated

**`Q/🧬️schema/🔣️.json`**
- `WireVersion` const 4.
- New `$defs/ShortText` (`{en, de}`, each 1–40 code points).
- Optional `short` after `label` on `Quantity`, `Axis`, `Category`, `ClassificationItem`, `SortingItem`, `MatchingItem`, `SheetItem` and `SheetAxis`.
- Optional `familiar: boolean` on `SortingItem` and `MatchingItem` only.
- New `$defs/Verdict` enum `under | over | reversed`.
- `CompareHint.under` is replaced by the required `verdict`.
- `ProfileHint` gains `other?` and `above?`, with `dependencies` so that both are present or neither.
- The `Hint` description now covers the cap and the short labels. `SheetItem` gains a description.

**`Q/🧬️schema/🟦️.ts`**
- `WireVersion`/`WIRE_VERSION` set to 4.
- New `ShortText` and `SHORT_LENGTH`.
- `short?` on the eight types above; `familiar?` on sorting and matching items.
- New `VERDICTS` and `Verdict`.
- `CompareHint.verdict`; `ProfileHint.other?` and `above?`.

**`Q/🔨️modules/⛰️challenge/🟦️.ts`**
- New exports `HINTS_PER_TASK = 3` and `verdictOf(claim, truth, pivot)`.
- `compareHint` prefers a familiar candidate inside the tie window.
- New private `profileOther` and `capped`.
- `hintsOf` = `capped(weightedHints(…))`.

**`Q/🔨️modules/🃏️sheet/🟦️.ts`**
- New `shortOf`.
- Items, hidden-key axes and hidden-key categories carry `short` right after `label`.
- `familiar` is never projected.

**`Q/🔨️modules/✅️validation/🟦️.ts`**
- `text(…, max)` and a new `short()` check (≤ `SHORT_LENGTH` per language).
- New `familiar()` check, for sorting and matching items only.
- The optional property lists are extended.

**`Q/README.md`**
- Quantity section: new paragraph on `short` and `familiar`.
- Core "Hints" bullet: verdict, familiar, profile `other`/`above`, the cap.
- The Rust name list gains `HINTS_PER_TASK` and `verdict_of`.

**`Q/🧫️fixtures/🤝️wire-version/🔣️.json`**
- Recommitted from its `🐍️.py` with `PYTHONIOENCODING=utf-8` and CR stripped: `wireVersion` 4, fingerprint `e683a4482e9316af`.

**Tests**
- `Q/🧪️tests/🧗️challenge-ladder/🟦️.ts`:
  - The mathjs oracle gains `oracleVerdict` (a sign-based route), the familiar tie-break, `oracleCapped`, and profile `other`/`above` with weights.
  - New cases:
    - every verdict branch, including the truth at the pivot and the claim at the pivot;
    - a verdict sign sweep with 338 combinations;
    - the familiar order (inside and outside the window, a missed familiar item is no anchor, sorting and matching);
    - the cap for sorting, matching and classification, including ties and order kept;
    - profile `other`/`above` (both sides, strictness, farthest wins, sheet-order ties, an anchor without the axis, member order).
  - The random sweeps now include familiar items and the cap, and assert that every verdict and every profile form occurs.
- `Q/🧪️tests/🎴️sheet-randomization/🟦️.ts`: `short` is carried at every challenge with its member order, and `"familiar"` never appears in a sheet.
- `Q/🧪️tests/🩺️document-validation/🟦️.ts`:
  - 9 structural cases for `short` and `familiar`, each checked against ajv;
  - one accept case (40 code points, including emoji);
  - checks of the `Hint`, `Verdict`, `SheetItem`, `SheetAxis` and `ShortText` definitions.
- `Q/🧪️tests/👁️read-views/🟦️.ts`: `verdict` replaces `under`.

**Ticket**
- `📓️hints-schema-landed.md`: "Round 2" appended.

### Created
- This report.

### Removed
- Nothing. My tool output folder `🗑️generated/hints2-core` is deleted.

## Gates

| Command | Result |
|---|---|
| `bun node_modules/typescript/bin/tsc --noEmit -p TICKET/tsconfig.core.json` (repo root) | exit 0 |
| `bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts document-validation sheet-randomization read-views challenge-ladder` (in `Q/📦️packages/🟦️typescript`) | 4 files, 196/196 passed (challenge-ladder alone: 53/53) |
| `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` (same directory) | 11 files: 448 passed, **1 failed** (see Red) |
| `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe Q/🧪️tests/🤝️wire-version/🐍️.py` | `wireVersion` 4, `e683a4482e9316af`, written LF |

## Red, and what was not run

**Red: `🗃️shared-vectors` › `🧾️learner-lifecycle`.**
- `hints-on-easy/a1e7a8fabefe2d66ae3c00c73ebccd9f/hints/energy-carriers: produced 3 entries, committed 4`.
- This is the cap. The committed vectors predate round 2.
- `⛰️challenge-rules` also still carries `under` (74 times); it is checked by the case harness, not by this suite.
- Both belong to the vectors agent.

**Not run** (other agents' areas):
- Rust and the proctor;
- the React suite and typecheck: `🎯️targets/⚛️react/🔨️modules/🧩️task/🟦️.tsx` and its tests still read `under`;
- the site;
- `schema generate`: the catalog pins the twins' hashes, so whoever lands `🦀️.rs` last must rerun `bun ./📜️script.ts schema generate` and `schema docs`.

## Deviations and decisions

1. **Reversed.** Rule: `claim > p ? truth <= p : claim < p && truth >= p`, on both scales.
   - This is §8.4a's linear sign rule (with `δ ≠ 0`, since equal keys claim no order).
   - §8.4a's log formula `(ρ ≥ 1) ≠ (τ ≥ 1)` differs only at `τ = 1, ρ > 1`, where it would say `over`. I chose the symmetric rule on both scales: when the true values are equal, a claimed order is wrong, and asking for the order shows no number.
   - Not reversed: `under`/`over` as in round 1, so a claim at the pivot is `under` exactly when the truth lies above it.
2. **Profile `other` choice.** "Largest gap" is read as the largest `|Q − r|`: X's true distance from the anchor, which gives the most absurd claimed side.
   - Since `r` lies strictly between `P` and `Q`, this equals the smallest `|P − r|` up to rounding. I compute `|Q − r|` directly.
   - Anchors are presented, assigned items placed in their own category whose category profile has the axis. All comparisons are strict.
3. **Cap weights and order.**
   - A compare hint is weighted by the error of its *chosen* reference; a profile hint by `gap / reach` of its chosen axis.
   - Group and category hints are unweighted and come after the weighted ones in emission order.
   - Equal weights keep the earlier hint. The kept hints are returned in emission order, not in weight order, so the order of `RunView.hints` stays as documented.
   - Matching is capped once over all its dimensions.
   - Weights are raw numbers. A matching that mixes scales would compare ratio errors with distances; there is no twin-safe common unit without `log`. All four live quizzes are logarithmic only.
4. **Short labels.** I added a new `$defs/ShortText` instead of repeating the constraint at each use. Its twins are `type ShortText = Text`, plus `SHORT_LENGTH = 40` in TS.
   - `short` sits right after `label` in every type, and in the sheet projections too.
   - Tasks carry no `short`.
5. **`familiar`.**
   - It is inlined as a boolean property with a description, with no `$def`.
   - It is never part of a sheet: `SheetItem` refuses it, and the sheet test asserts it never appears.
   - Validation reports it as `property-unknown` on classification items.
6. **`Verdict`.** A `$def` enum, with `VERDICTS`/`Verdict` in TS, following the `Scale`/`SCALES` pattern.

## Notes for the next agent

```ts
export const HINTS_PER_TASK = 3;
export function verdictOf(claim: number, truth: number, pivot: number): Verdict; // pivot 1 log, 0 linear
type Verdict = "under" | "over" | "reversed";  // VERDICTS
type CompareHint = { kind: "compare"; item; other; dimension?; factor? | difference?; verdict: Verdict };
type ProfileHint = { kind: "profile"; item; category; axis; other?: Slug; above?: boolean };
type ShortText = Text; // ≤ SHORT_LENGTH (40) code points per language
```

Member order:
- `CompareHint`: `kind, item, other, dimension?, factor|difference, verdict`;
- `ProfileHint`: `kind, item, category, axis, other?, above?`;
- sheet item: `id, label, short?, icon?`;
- hidden-key axis: `id, label, short?`;
- hidden-key category: `id, label, short?, icon?, profile?`.

**Rust twin.** Mirror the following:
- `ShortText`, `Verdict` (serde lowercase), `short` and `familiar` (skip when absent);
- `verdict_of`, `HINTS_PER_TASK`, the familiar tie-break, `profile_other` and the cap;
- `WIRE_VERSION = 4`;
- the validation of `short` (≤ 40 chars per language) and `familiar`.

**Python reference.** Implement the same readings (see the landed note), regenerate every fixture, and add the 2 new `$defs` to the schema-conformance mapping (`Verdict`, `ShortText`).

**React.** Render the following:
- `verdict` (reversed = the order question without a number);
- `short ?? label` for items, categories, axes and quantities: the sheet carries `short`;
- profile `other`/`above`.
