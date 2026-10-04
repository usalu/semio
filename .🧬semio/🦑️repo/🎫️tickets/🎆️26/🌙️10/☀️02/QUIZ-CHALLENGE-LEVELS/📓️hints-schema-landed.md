Schema landed (2026-10-04): `🧬️schema/🔣️.json` + `🟦️.ts` carry `Quantity.additive` (required, after `prefixed`), `CompareHint` {kind "compare", item, other, dimension?, factor? (> 0), difference?, under; JSON `oneOf` requires exactly one of factor/difference}, `ProfileHint` {kind "profile", item, category, axis}, `GroupHint` {kind "group", item, other, together}, `CategoryHint` {kind "category", item, category}, `Hint` = oneOf those four (Magnitude/Misplaced removed; the reach/miss definition moved into the `Challenge` description), `WireVersion` 3 / `WIRE_VERSION = 3` in TS.

TS `hintsOf` readings of §8.3 where the text leaves room (twins please match; details in `📓️report-hints-core-typescript.md`):
- Sorting: keyed items in **sheet order**, key = `keys[answer.order.indexOf(item)]` (first index; skipped when absent or ≥ keys.length); hints emitted in `answer.order`. Pool order (for ties) = sheet order.
- Compare: error log `ρ > τ ? ρ/τ : τ/ρ`, linear `|δ − Δ|`; strict `>` keeps the first on ties; `under = claim >= pivot ? truth > claim : truth < claim` (pivot 1 log, 0 linear). Key order of the object: kind, item, other, dimension?, factor|difference, under.
- Classification: an assignment to a category id not in the task gives no hint. Own and assigned category both with a profile → profile rule only (no group/category fallback, also when it yields nothing). Profile reach per axis over the task profiles of the sheet's categories that carry that axis: `(max − min) / 2`; axes with reach not > 0 skipped; hinted iff some axis has `gap > reach × (1 + 1e-9)`; axis = largest `gap / reach` (strict `>`, task axis order).
- Group `together: false`: first `R` (sheet order, R ≠ X, assigned) with `R.category == X.category` and `assigned(R) != assigned(X)` ("put elsewhere" = elsewhere than X).

## Round 2 (2026-10-04, design §8.4a; schema JSON + TS twin landed)

Contract (`🧬️schema/🔣️.json`, `🟦️.ts`; the Rust twin must mirror it):
- `WireVersion` **4** (`WIRE_VERSION = 4`).
- New `$defs/ShortText` (annotated like `Text`): object `{en, de}`, both required strings, `minLength 1`, `maxLength 40` (code points), `additionalProperties: false`. TS `export type ShortText = Text;` plus `SHORT_LENGTH = 40`. Rust: `pub type ShortText = Text;` (and a `SHORT_LENGTH` const if you like the symmetry).
- Optional `short: ShortText` — always the property **right after `label`** — on `Quantity`, `Axis`, `Category`, `ClassificationItem`, `SortingItem`, `MatchingItem`, `SheetItem`, `SheetAxis`. Sheet categories and quantities are the task's own `Category`/`Quantity`, so they carry it already.
- Optional `familiar: boolean` on `SortingItem` (after `value`) and `MatchingItem` (after `values`), before `explanation`. **Not** on any sheet type: the sheet never carries it (hints are core-side).
- New `$defs/Verdict` enum `["under", "over", "reversed"]`. TS `VERDICTS` const + `Verdict` type. Rust: enum, serde lowercase.
- `CompareHint`: `under: boolean` → **`verdict: Verdict`** (required). Emitted member order: `kind, item, other, dimension?, factor|difference, verdict`.
- `ProfileHint`: `{kind, item, category, axis, other?: Slug, above?: boolean}`; JSON `dependencies: {other: [above], above: [other]}` (both or neither). Emitted member order: `kind, item, category, axis, other?, above?`.

Sheet (`sheetOf`): `short` is copied wherever present, right after `label`:
- items → `{id, label, short?, icon?}`;
- hidden-key axes → `{id, label, short?}` (shown-key axes are the whole task axis);
- hidden-key categories → `{id, label, short?, icon?, profile?}` (shown-key categories are the whole task category).

`hintsOf` readings (TS landed in `⛰️challenge/🟦️.ts`; twins and the Python reference please match). Pivot `p` = 1 (log, claim ρ = kX/kR, truth τ = vX/vR) or 0 (linear, claim δ = kX − kR, truth Δ = vX − vR).
1. **Verdict** (only comparisons, no arithmetic):
   `reversed = claim > p ? truth <= p : claim < p ? truth >= p : false`;
   otherwise `under = claim >= p ? truth > claim : truth < claim` (unchanged from round 1); otherwise `over`.
   A claim at the pivot (equal keys) is never reversed. A truth at the pivot (equal true values) with a claim off it **is** reversed on both scales. This is the linear "sign δ ≠ sign Δ, δ ≠ 0" rule of §8.4a; the log formula of §8.4a (`(ρ ≥ 1) ≠ (τ ≥ 1)`) differs from it only at τ = 1 with ρ > 1. We take the symmetric sign rule on both scales.
2. **Reference choice with `familiar`.** Pool and errors are unchanged. Tie window: `e × (1 + REACH_SLACK) ≥ max e`. Inside the window the candidate wins in this order:
   - a familiar item (`item.familiar === true`, absent = false);
   - then the smallest oriented claim (`max(ρ, 1/ρ)` / `|δ|`);
   - then the first in sheet order.
   Iterating in sheet order, a candidate replaces the current pick iff `(cand.familiar && !pick.familiar) || (cand.familiar === pick.familiar && cand.oriented < pick.oriented)`. `familiar` never widens the pool: a familiar item that misses is no anchor.
3. **Profile `other`/`above`.**
   - The axis `a` is chosen as in round 1. `P` = the assigned category's value on `a`, `Q` = X's own category's value.
   - Candidates `R`: presented, assigned items in sheet order, `R ≠ X`, placed in their own category (`assigned(R) == R.category`), and that category's task profile has `a`. Call that value `r`.
   - `R` qualifies when `P > r && Q < r` (`above: true`) or `P < r && Q > r` (`above: false`). All comparisons are strict.
   - Among qualifying candidates, the one with the **largest `|Q − r|`** wins: X's true distance from `R`, i.e. the most absurd claimed side. Strict `>`, so the first in sheet order wins ties.
   - With no qualifying `R` the hint is `{kind, item, category, axis}` as before.
4. **Cap `HINTS_PER_TASK = 3`** (exported from `⛰️challenge`).
   - Each emitted hint gets a weight: compare → the error `e` of its **chosen** reference; profile → `gap / reach` of its chosen axis (the same `ratio` value the axis choice compares); group and category → no weight.
   - Ranking: weighted hints first by weight descending (ties: the earlier in emission order), then unweighted hints in emission order. The first 3 are kept.
   - The kept hints are returned **in emission order**: sorting = learner order, matching = dimensions then items in sheet order (one cap over all dimensions of the task), classification = sheet order.
   - Implementation: a stable sort of indices with the comparator `(class) || (weight desc via > / <) || index`. Only comparisons, so it is twin-safe.
   - Weights are raw: a matching that mixes a logarithmic and a linear dimension compares ratio errors with distances as plain numbers (there is no twin-safe common unit without `log`); every live matching is logarithmic only.

New TS exports from `⛰️challenge`: `HINTS_PER_TASK = 3`, `verdictOf(claim, truth, pivot): Verdict`. Rust names: `HINTS_PER_TASK`, `verdict_of`. From the schema twin: `ShortText`, `SHORT_LENGTH`, `VERDICTS`, `Verdict`.

Validation (`quizIssues`):
- `short` is allowed on quantity, axis, category and all three item kinds. Each language must be 1–40 code points (`length-invalid`); missing languages give `required`; extra keys give `property-unknown`; a non-object gives `type-invalid`.
- `familiar` is allowed on sorting and matching items only, and must be a boolean (`type-invalid`). On classification items it gives `property-unknown`.
- Tasks carry no `short`.

Wire fixture: `🧫️fixtures/🤝️wire-version/🔣️.json` was recommitted, `wireVersion: 4`, fingerprint `e683a4482e9316af`. Any further schema change beyond prose changes the fingerprint.

Committed vectors that are now stale (owned by the vectors agent):
- `🧾️learner-lifecycle` `hints-on-easy/…/energy-carriers`: committed 4 hints, the core now gives 3 because of the cap.
- `⛰️challenge-rules`: still carries `under` (74×).

## Round 3 (2026-10-04, design §8.4b item 3; TS, Rust, Python reference + numpy oracle landed together)

No schema or wire change (`WireVersion` stays 4). Reference diversity inside one task:
1. **Order of decision.** Weigh, cap, then choose references.
   - Each compare hint is first made without a reference: its pool (anchors, else every other keyed item), the error per candidate, and the **tie window** (`e × (1 + REACH_SLACK) ≥ max e`), kept in sheet order.
   - Its cap weight is now the **largest error of the pool** (`max e`), no longer the error of the chosen reference. The two differ by at most the relative slack, and the weight no longer depends on the choice that follows. Profile weights and the cap itself are unchanged (round 2, item 4).
   - The cap keeps the same hints as before; they stay in emission order: sorting = learner's order, matching = dimensions in sheet order, then items in sheet order.
   - Only then are the references of the **kept** compare hints chosen, in that order. A hint the cap drops therefore uses up no reference.
2. **Choice inside the tie window.** Candidates in sheet order; a candidate replaces the current pick iff
   `fresh(c) ≠ fresh(p) ? fresh(c) : c.familiar ≠ p.familiar ? c.familiar : c.oriented < p.oriented`.
   - `fresh` = not yet the `other` of an earlier kept compare hint of the same task. For a matching this is per dimension: a reference named in another dimension is still fresh.
   - So the order is: not yet named, then familiar, then the smallest oriented claim (`max(ρ, 1/ρ)` / `|δ|`), then the first in sheet order.
   - Diversity never widens the window. When it holds only named references, the same rule picks among them, and a reference is named again.
   - Only `other` counts: an item that is the `item` of an earlier hint is not "used".
3. **Names.**
   - TS: `Pending`/`Related` types, `referenced(pending, used: Map<Slug | undefined, Set<Slug>>)`; `capped` now returns the kept weighted entries.
   - Rust: `Related`, `Pending`, `enum Drafted { Made(Hint), Compare(Pending) }`, `referenced(&Pending, &mut Vec<(Option<&Slug>, &Slug)>)`; `capped<T>` is generic.
   - Python: `compare_hints` returns `(largest, {item, dimension, tied})` and `capped` calls `referenced`. numpy: `numpy_referenced`, which builds the used mask by `numpy.isin`. The lifecycle reference is kept identical.
