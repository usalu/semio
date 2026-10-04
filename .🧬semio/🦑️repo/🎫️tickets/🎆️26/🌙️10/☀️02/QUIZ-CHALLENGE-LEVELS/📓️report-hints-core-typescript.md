# Report — specific hints: schema, TypeScript core, quiz content (2026-10-04)

Scope: design §8 "Revision 2026-10-04 — specific hints". Schema + TS twin landed first (`📓️hints-schema-landed.md`).

## Changed files

Updated (Q = `🧰️framework/🛍️products/❓️quiz`):
- `Q/🧬️schema/🔣️.json` — `WireVersion` const 3; `Quantity.additive` (required, after `prefixed`); `CompareHint`, `ProfileHint`, `GroupHint`, `CategoryHint`; `Hint` oneOf those four; `MagnitudeHint`/`MisplacedHint` removed; the reach/miss definition moved into the `Challenge` description and the `SortingAnswer.guesses`, `SortingItemResult.miss`, `MatchingItemResult.miss` descriptions now point to `Challenge`.
- `Q/🧬️schema/🟦️.ts` — `WireVersion = 3`, `WIRE_VERSION = 3`, `Quantity.additive`, the four hint types, `Hint` union.
- `Q/🔨️modules/⛰️challenge/🟦️.ts` — `hintsOf` per §8.3 (private `Keyed`, `compareHint`, `profileAxis`).
- `Q/🔨️modules/✅️validation/🟦️.ts` — `quantity()` requires and type-checks `additive`.
- `Q/README.md` — quantity sentence (`additive`), the core "Hints" bullet rewritten, client "Hints" bullet ("?" symbol, no arrow), "easy names it in every hint" corrected.
- Quiz content: `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json` (`powers`, `energies`: `additive: true`), `🔥️heating`, `❄️cooling`, `📊️demand` (every quantity `additive: false`).
- TS test literals (`additive` added): `Q/🧪️tests/{⚖️partial-credit-scoring,🎖️badge-awards,🎴️sheet-randomization,👁️read-views,🔁️run-lifecycle,🗳️crowd-answers,🩺️document-validation,🧗️challenge-ladder}/🟦️.ts`, `Q/🧪️tests/{⌨️task-keyboard,📐️adaptive-layout,🚶️learner-journey,🪜️challenge-views}/🟦️.tsx` (only the `additive` member; their hint rendering belongs to the React agent).
- Tests rewritten/extended: `🧗️challenge-ladder` (hintsOf: 25 cases incl. three mathjs-oracle sweeps), `👁️read-views` (compare/category/group hints in `runView`), `🩺️document-validation` (schema admits/refuses every new hint shape, `additive` required/typed).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/{🔣️schema-catalog.json,📓️schema-catalog.md}` — regenerated (`schema generate`, `schema docs`); the drift probe showed only `framework.product.quiz` drifting.

Created: `📓️hints-schema-landed.md`, this report. Removed: nothing.

## Gates run

| Command | Result |
|---|---|
| `bunx tsc -p TICKET/tsconfig.core.json --noEmit` | exit 0 |
| `bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts challenge-ladder` (in `Q/📦️packages/🟦️typescript`) | 46/46 passed |
| `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` (same dir) | 426 passed, 5 failed — all in `🗃️shared-vectors` (see Red) |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | 5 files, 235/235 passed |
| `bun ./📜️script.ts schema generate && schema docs && schema generate --check` | current, 3734 scopes |

## Red / not run (vector-driven, for the vectors agent)

`🗃️shared-vectors` (5 tests): committed fixtures still carry quantities without `additive` (`🃏️sheet-assembly`, `📏️sorting-concordance`, `🔀️matching-concordance`, `🧾️learner-lifecycle` quizzes) and `⛰️challenge-rules` vectors still carry `magnitude` hints. Also pending elsewhere: `🧫️fixtures/🤝️wire-version` recommit (version 3), the feature cases' TS bindings (`⛰️challenge-rules`, `🧬️schema-conformance` feature lists `MagnitudeHint`/`MisplacedHint`), the proctor fixture `🎓️teaching/🛂️proctor/🧫️fixtures/⚡️power/🔣️.json` (3 quantities without `additive`), the site e2e spec `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/⛰️challenge-levels/🟦️.ts` (old hint wording), React modules/tests rendering `magnitude`/`misplaced`. Not run: React suite/typecheck, Rust, proctor (other agents' areas). Whoever lands `🦀️.rs` last must re-run `bun ./📜️script.ts schema generate` + `schema docs` (the catalog pins the twins' hashes).

## Deviations and decisions

- `CompareHint`: JSON `oneOf: [{required:[factor]},{required:[difference]}]` enforces "exactly one"; `factor` has `exclusiveMinimum: 0`.
- Sorting: keyed items are taken in **sheet order** (key = `keys[answer.order.indexOf(id)]`, first index; absent or ≥ `keys.length` → not keyed); hints are emitted in the learner's order. Unknown ids in `order` still shift positions (as before).
- Classification: an assignment to a category id not in the task gives no hint. When own and assigned category both carry profiles, only the profile rule applies (a near miss, or no comparable axis, gives no hint — no group/category fallback), per "not covered by a profile rule because a profile is missing".
- `GroupHint together:false`: "assigned elsewhere" read as `assigned(R) ≠ assigned(X)` (R has X's own category); R ranges over presented, assigned items ≠ X.
- Profile reach: over the task profiles of the **sheet's** categories that carry the axis; axes with reach not > 0 skipped; hinted iff some axis has `gap > reach × (1 + REACH_SLACK)`; axis = largest `gap / reach` with strict `>` (first in task axis order). Uses task (true) profiles, so hints are the same at every challenge.
- Ties everywhere: strict `>` while iterating in sheet (axis) order keeps the first.
- `additive` of test fixtures: W/Wh/g quantities `true`, everything else `false`.

## Notes for the next agent (signatures)

```ts
type Quantity = { label: Text; unit: string; scale: Scale; prefixed: boolean; additive: boolean };
type CompareHint = { kind: "compare"; item: Slug; other: Slug; dimension?: Slug; factor?: number; difference?: number; under: boolean };
type ProfileHint = { kind: "profile"; item: Slug; category: Slug; axis: Slug };
type GroupHint = { kind: "group"; item: Slug; other: Slug; together: boolean };
type CategoryHint = { kind: "category"; item: Slug; category: Slug };
type Hint = CompareHint | ProfileHint | GroupHint | CategoryHint;
const WIRE_VERSION: 3;
function hintsOf(task: Task, sheetTask: SheetTask, answer?: Answer): Hint[]; // unchanged signature
```

Object member order as emitted: `kind, item, other, dimension?, factor|difference, under`; `kind, item, category, axis`; `kind, item, other, together`; `kind, item, category`.
Compare core: `claim = linear ? kX − kR : kX / kR`, `truth` likewise with values, `error = linear ? |claim − truth| : (claim > truth ? claim / truth : truth / claim)`, `under = claim >= (linear ? 0 : 1) ? truth > claim : truth < claim`. Pool = other keyed items whose key does not miss, else all other keyed items; empty pool → no hint.
