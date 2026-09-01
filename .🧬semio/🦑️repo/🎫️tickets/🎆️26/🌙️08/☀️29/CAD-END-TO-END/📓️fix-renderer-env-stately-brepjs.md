# Fix: renderer jsdom env, brepjs fixture count, stately root-cause (handoff)

## Scope (exclusive ownership, as assigned)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️component.ts` (investigated only, not edited — see below)
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts`

## Before / after (full `test-long` run)
- Before: `Test Files 5 failed | 4 passed (9)` — `Tests 63 failed | 258 passed (321)`
- After: `Test Files 3 failed | 6 passed (9)` — `Tests 60 failed | 261 passed (321)`
- Net: **3 tests fixed** (2 renderer + 1 brepjs). The 1 remaining stately failure is root-caused below to files outside my slice (model-definition JSON assets + `📄️artifact/🟦️component.ts`), not fixed here per instructions.

Renderer file alone: `69 passed (69)`, 0 failures. Brepjs file alone: `30 passed (30)`, 0 failures.

## 1. Renderer `ReferenceError: document is not defined` — FIXED

Root cause: `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts` set `environmentMatchGlobs: [[renderer-file, "jsdom"]]` to give only the renderer suite a DOM while the rest of `@semio-tech/cad-js` runs in `node`. **`environmentMatchGlobs` does not exist in vitest 4** (confirmed: absent from `node_modules/vitest/dist/config.d.ts` and `chunks/reporters.d.*.d.ts`, which only expose `environment`/`environmentOptions`; it was a vitest 1/2 option). It's not a TS type error because `vitest.config.ts` isn't included in the CAD tsconfig, so it silently no-opped — the whole project ran in `node`, and the two renderer tests that build real DOM fixtures (`document.body`, `document.createElement`) threw.

The functions under test (`replIsQueryTypingTarget`, `replShouldRepeatInteractionOnSpace`) were already pure — they take `EventTarget | null` as a parameter and delegate to `isUiTypingTarget` from `@semio-tech/ui-react`. There was nothing left to refactor; only the test fixtures touch `document` directly. So per the ticket's decision tree, environment configuration was the right fix, not an API redesign.

Fix (the vitest-4-native equivalent of per-file environment override — confirmed working, not guessed: traced the pragma regex `@(?:vitest|jest)-environment\s+([\w-]+)` in `node_modules/vitest/dist/chunks/cli-api.BK8pd4xc.js`):
- Removed the dead `environmentMatchGlobs` line from `🧪️vitest.config.ts` and updated its docstring.
- Added `// @vitest-environment jsdom` near the top of `📺️renderer/🟦️component.tsx` (after the `/// <reference>` directives). This only affects vitest's transform for that in-source spec; it has zero effect on the file's real production bundling.
- `jsdom` is already a repo devDependency and already used this way in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts` (`environment: "jsdom"`), so no new runtime/devDependency was added.

Verified: both named tests pass in isolation, and the full renderer file (69 tests) passes with no regressions from switching its transform environment.

## 2. Stately `TypeError: null is not an object (evaluating 'rd.data')` — ROOT-CAUSED, NOT FIXED (out of slice)

This is **the same root cause as the sibling's `res.ok`-is-null failures in `📄️artifact/🟦️component.ts`** (measure tests + most of the interaction e2e-fixture suite there). Confirmed by adding no code, only reading the existing `[DEBUG]` traces already in `runCommit`/`send` and running the failing test in isolation:

```
[DEBUG] result {"r":{"ok":true,"transient":false},"afterState":"ready"} ctxKeys [] hostKind undefined
```

After both `selection.changed` events, the machine correctly reaches state `"ready"` — but `ctxKeys` is `[]`. The `measure.distance` interaction's `selectA`/`selectB` transitions are supposed to `assign` `vertexA`/`vertexB` into context via each transition's `effects`. Those assigns never land, so the `hasBothVertices` guard fails, `canCommitFromState` returns false, `runCommit` never runs, and `lastResponse` stays `null`.

**Exact defect, independently verified from two directions:**
- The canonical `EffectSpec` type (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts:558`) discriminates on a field named **`operation`**: `{ operation: "assign"; target; value } | { operation: "clear"; ... } | ...`. `applyEffectAsync` (`⚙️engine/🎬️actions/🟦️component.ts:1139`) branches on `a.operation === "assign"` etc. — code and type agree.
- Every model-definition interaction JSON asset instead writes the discriminant as **`"mutation"`**, e.g. `🔣️length.json`: `{"mutation": "assign", "target": {...}, "value": {...}}`. Checked repo-wide: `grep -c "\"mutation\"" .../modelDefinitions -r` → **481 occurrences**, and **zero** occurrences of `"operation": "assign"` (or any other operation kind) anywhere under `modelDefinitions`.
- `parseInteractionSpec` (`📐️geometry/🟦️component.ts:832`) does not validate the shape of individual `effects[]` entries (only that `effects` is an array), so the JSON's `{mutation: "assign", ...}` objects pass straight through as `InteractionSpec["machine"]["states"][].on[].transitions[].effects[]`. At runtime `effect.operation` is `undefined` for all of them, so every `if (a.operation === "assign") ...` / `"clear"` / `"append"` / `"kernel.query"` / `"action"` branch in `applyEffectAsync` (and the analogous branches in `staticInitialContext`) silently no-ops. This affects every interaction that relies on transition effects — which is why the sibling's failures span measure, box, transform, curve, and most of the e2e-fixture suite, not just measure.

This is a data/schema mismatch in the **model-definition JSON assets**, explicitly called out in the ticket as the sibling agent's territory (along with `📄️artifact/🟦️component.ts`, which only *consumes* the broken specs). I did not touch either — per the ticket's own guidance ("if the true fix is in the artifact engine or in the measure action assets, hand it off rather than patching around it in stately"), and per my own instructions not to edit those files. The fix is a rename of the discriminant field from `mutation` to `operation` across the ~46 affected model-definition JSON files (all effect kinds, not just `assign` — `kernel.query`, `clear`, `append`, `action`, etc. use the same field), matching the canonical `EffectSpec` schema. No `stately`-side workaround exists: `stately/🟦️component.ts` only calls `createInteractionRuntime`, which lives in the sibling's file and consumes the sibling's assets.

## 3. Brepjs `expected [ …(11) ] to have a length of 12 but got 11` — FIXED

Investigated by reading the fixture directly (not through the code under test), independent of the round-trip path being tested:

```
$ python3 -c 'json.load(...)' on 🔣️hexagonal-cut-concrete-forest-left.model.json
spatial.shape                    1 object
aec.building                    11 objects: bim-1 .. bim-11 (sequential, no gaps, no bim-12)
aec.building.energy               1 object
aec.building.structure.classic  11 objects: classic-structure-1 .. classic-structure-11 (sequential, no gaps)
```

The fixture genuinely has 11 `aec.building` objects, not 12 — verified straight from the on-disk JSON, independent of `ModelSpace.fromJSON`. The test's own next assertion, two lines down, already expects 11 for the sibling `aec.building.structure.classic` model built from the same 11 physical elements — so a `building` count of 12 was never internally consistent with the rest of the same test. Running the (pre-fix) test confirms the runtime round-trip reproduces exactly 11 keys with no extra transformation loss (`AssertionError: expected [ …(11) ] to have a length of 12 but got 11`), i.e. nothing is being dropped by `ModelSpace.fromJSON` — the fixture and the round-trip agree at 11; only the test's hardcoded `12` was wrong.

Fixed `🧱️brepjs/🟦️component.ts:3300`: `expect(Object.keys(building.objects)).toHaveLength(12)` → `toHaveLength(11)`.

Verified: the full brepjs file now passes 30/30, including this test.

## Typecheck
```
npx tsc -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit 2>&1 | grep "error TS"
```
Unchanged before/after my edits — 3 pre-existing errors, all outside my slice (`🧰️framework/🔨️modules/🧊️3d/🟦️.ts` ×2, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` ×1). No new errors introduced.

## Files touched
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx` — added `// @vitest-environment jsdom` pragma.
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts` — removed dead `environmentMatchGlobs`, updated docstring.
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts` — fixed fixture assertion `12` → `11` at line ~3300.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️component.ts` — not edited (root cause is upstream, see §2).
