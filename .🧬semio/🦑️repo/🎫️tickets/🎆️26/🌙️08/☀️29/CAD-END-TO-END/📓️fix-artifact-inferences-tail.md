# Fix: artifact + inferences tail errors, and test-suite verification

## Scope
Owned files:
1. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📄️artifact/🟦️component.ts`
2. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️component.ts`

Read `📓️fix-artifact-engine.md` and `📓️fix-inferences.md` first, per brief; both files were previously reduced from 39/37 errors to 1/2 (both cross-slice blockers, not this slice's to fix).

## What I found on arrival
Re-running `tsc` showed the two documented cross-slice items still open (verified directly, not assumed):
- `modelDefinitionInteractionCatalog` — still `function` (not `export function`) in `⚙️engine/📔️registry/🟦️component.ts:98`.
- `BrepjsKernel.id` / `.operations` in `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts:2577,2581` — still no explicit type annotation, still narrows to literal types.

Both are outside my slice; I did not touch either file, per the brief.

But `tsc` also surfaced **2 new genuine errors** in my owned file `📄️artifact/🟦️component.ts` that postdate the prior fix report (39→1 became 39→1+2 new = 3 total before my pass):
```
📄️artifact/🟦️component.ts(1537,75): error TS7006: Parameter 'pole' implicitly has an 'any' type.
📄️artifact/🟦️component.ts(1545,57): error TS7006: Parameter 'pole' implicitly has an 'any' type.
```

## Root cause and fix
Both are in the same test (`selectionTargetsPointTransformDiff moves all nurbs poles when an edge is selected`). `edge.curve.poles`'s static type flows from `EdgeCurve`, imported from `@semio-tech/s-3d-js` — the package under the KNOWN EXTERNAL BLOCKER (broken `exports` map, `TS2307` repo-wide). With that import unresolved, `EdgeCurve` (and thus `curve.poles`) resolves to `any`, so calling `.map(callback)` on it makes TypeScript unable to infer the callback parameter type from an `any` receiver — a callback-on-`any` implicit-any, distinct from (but caused by) the blocker's `TS2307`.

Fix: added explicit `Vec3` parameter annotations (the type the array *should* narrow to once the blocker resolves) instead of relying on inference:
```ts
const before: readonly Vec3[] = edge.curve?.kind === "nurbs" ? edge.curve.poles.map((pole: Vec3) => [...pole] as Vec3) : [];
...
expect(updated.curve.poles).toEqual(before.map((pole: Vec3) => [pole[0] + 1, pole[1], pole[2]]));
```
`Vec3` was already imported in this file. No `any`/`as unknown as`/`@ts-ignore` used; the annotations are correct regardless of the blocker's state and remain correct once it resolves.

## Result — tsc
Re-ran `npx tsc -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit` after the fix. Both files now show **only**:
- `📄️artifact/🟦️component.ts`: 2× `TS2307` (blocker, not touched) + 1× `TS2724 modelDefinitionInteractionCatalog` (cross-slice handoff, unchanged, still not exported as of this run).
- `💡️inferences/🟦️component.ts`: 1× `TS2307` (blocker, not touched) + 2× `TS2416` `BrepjsKernel.id`/`.operations` (cross-slice handoff, unchanged, still not annotated as of this run).

No genuine errors remain in either owned file. Repo-wide `tsc` error count at this run: 76 (was 86 before this session's fix; other sibling slices are moving concurrently so this total is not attributable to this fix alone).

Only file touched: `📄️artifact/🟦️component.ts` (2-line edit, both inside the same test). `💡️inferences/🟦️component.ts` was not edited — its 2 remaining errors are exactly the previously-documented `BrepjsKernel` cross-slice blocker, unchanged.

## Test-suite verification
Ran the exact instructed command first:
```
bun nx run @semio-tech/cad-js:test
```
This resolves to the **fundamental** test level (15 000 ms wall-clock budget, no `test` alias in `📋️project.json`'s target defaults to level `fundamental` via `resolveTestLevel`). It was killed by the repo's own budget watchdog before any suite finished:
```
[budget] .../vitest.mjs run --config 🧪️vitest.config.ts ... exceeded 15000ms — killed. Trim it, or assign it to a higher level (quick/long/exhaustive).
```
This is not a pass/fail signal — the whole run is aborted before tests execute. To get an actual signal I additionally ran the `long` level (300 000 ms budget), which is the smallest level that let the suite actually execute:
```
bun nx run @semio-tech/cad-js:test-long
```

### Actual result (`test-long`), observed directly — 9 test files, 1 test executed, 1 failed, 8 suites failed to load
```
Test Files  9 failed (9)
     Tests  1 failed (1)
  Duration  30.14s (transform 107.48s, setup 0ms, import 15.42s, tests 22ms, environment 2ms)
```

**8 suites fail at collection (0 tests each), all from the same root cause — the KNOWN EXTERNAL BLOCKER, confirmed at runtime, not just typecheck:**
```
TypeError: Spread syntax requires ...iterable not be null or undefined
 ❯ 🎬️actions/🟦️component.ts:884:48
   const MODEL_ENTITY_KINDS = new Set<string>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object", "geometry", "attribute"]);
```
`PRIMITIVE_MODEL_ENTITY_KINDS` is imported (as a *value*, not type-only) from `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`, which itself has a value import `import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";`. With that package's `exports` map broken (per the ticket's KNOWN EXTERNAL BLOCKER), the module fails to resolve at runtime under vitest, so `geometry/🟦️component.ts`'s exports — including `PRIMITIVE_MODEL_ENTITY_KINDS` — come back `undefined`, and the spread throws. This cascades into every suite whose module graph pulls in `🎬️actions/🟦️component.ts` or `📐️geometry/🟦️component.ts`:
- `../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts`
- `⚙️engine/🎰️stately/🟦️component.ts`
- `⚙️engine/📄️artifact/🟦️component.ts` (my file)
- `⚙️engine/🏃️runtime/🟦️component.ts`
- `../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`
- `../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`
- `⚙️engine/📺️renderer/🟦️component.tsx`
- `🧬️schema/💡️inferences/🟦️component.ts` (my file)

**Consequence for my owned files:** I cannot observe pass/fail of any individual test *assertion* inside `📄️artifact/🟦️component.ts` or `💡️inferences/🟦️component.ts` right now — both suites fail to even load, for a reason entirely outside my slice (the s-3d-js blocker another session is actively fixing). I am not fixing this, per the ticket's explicit instruction, and I did not work around it (no stub, no package.json edit).

**1 test actually ran and failed** (in `🎬️actions/🟦️component.ts`, which loaded without hitting the spread crash in this particular file's isolated module graph):
```
FAIL |@semio-tech/cad-js| ...🎬️actions/🟦️component.ts > @semio-tech/cad-js/core box display committed > keeps box-preview visible for committed state
TypeError: modelDefinitionInteractionCatalog is not a function. (In 'modelDefinitionInteractionCatalog()', 'modelDefinitionInteractionCatalog' is undefined)
 ❯ shippedInteractionJsons 📄️artifact/🟦️component.ts:963
 ❯ loadSpatialInteraction 📄️artifact/🟦️component.ts:1020
 ❯ requireSpatialInteraction 📄️artifact/🟦️component.ts:1029
```
This is a **live confirmation at runtime** of the already-documented cross-slice handoff: `modelDefinitionInteractionCatalog` is still not exported from `⚙️engine/📔️registry/🟦️component.ts` (owned by the actions/stately/registry slice). Not fixed here — not my file, and the ticket brief explicitly says wait for that owner.

## Honest summary
- My 2 owned files: 0 genuine `tsc` errors remaining (2 new implicit-any errors found and fixed this session; the pre-existing cross-slice blockers are unchanged and confirmed still open).
- Test suite: **cannot be verified end-to-end right now.** 8 of 9 suites fail to load due to the KNOWN EXTERNAL s-3d-js BLOCKER (confirmed at runtime, exact error above) and the 1 suite that does load has 1 failing test caused by the also-already-documented `modelDefinitionInteractionCatalog` export gap. No test failure observed is attributable to either of my two owned files' own logic — both are blocked from even loading.
- I did not fix, stub, or work around either blocker. Do not treat this as "tests pass" or "tests fail" for my slice — they are **unobservable** until the s-3d-js blocker and the registry export land.

## Files touched
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📄️artifact/🟦️component.ts` (2-line fix, both inside one test)
- `💡️inferences/🟦️component.ts` — not touched (no genuine errors found there this pass)
