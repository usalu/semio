# Fix: `⚙️engine/📄️artifact/🟦️component.ts`

## Scope

Owned file: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📄️artifact/🟦️component.ts` (3304 lines, `InteractionRuntime` + interaction-registry + e2e test suite).

## Result

- File errors: **39 → 1**.
- Repo-wide `tsc` errors: **371 → 154** (other concurrent slices also progressed).
- No new errors introduced in any other file (verified by diffing the error file list before/after).

## What was wrong and how it was fixed

1. **`ExprEnv.preview` missing** (3 call sites: `seedChildContext`, and the two `writePathTarget`/`readPathTarget` calls in `runCommit`). Fixed by routing through the existing `this.exprEnv(extra)` helper instead of hand-building the env object, so `preview` is always populated.

2. **Dangling pointer/length/scalar-entry helpers** (`CURSOR_RAW_CTX`, `LENGTH_LOCK_CTX`, `HEIGHT_LOCK_CTX`, `SCALAR_AXIS_T_CTX`, `clearInteractionLengthEntryFields`, `positiveLengthLock`, `positiveHeightLock`, `scalarEntryAxis`, `scalarHeightFromAxisT`, `lengthEntryRawPoint`, `applyLengthEntryToContext`, `lookupGuard`, `findState`, `applyActionPatchToContext`). These bodies already exist, unexported, in the sibling `⚙️engine/🎬️actions/🟦️component.ts` (and `findState` in spatial-kernel `📐️geometry`). Rather than depend on those siblings adding exports, I followed the pattern the codebase already uses for this exact situation — `📐️geometry` has its own private `findState` and `🎬️actions` has its own private `lookupGuard`, i.e. tiny pure `(spec, ctx) → value` helpers are duplicated per file, kept close to their call sites (per repo convention: "if code is repeated, it must be close to each other"). I reinstated local, unexported copies of all of the above in a new `#region 📏️PointerContext` right before the `InteractionRuntime` class. This is self-contained and does not depend on any other agent's timing.

3. **`InteractionRegistry`/`ActionRegistry` `.register(...)` calls** — both registries are `ReadonlyMap` type aliases (immutable-update pattern); there never was a `.register` method. Fixed the 3 test call sites to use the existing `registerInteractionSpec`/`registerActionDef` functions with reassignment (`let` + `x = registerXxx(x, ...)`).

4. **`SelectionTarget` missing `editable`** at two `selectionTargetsPointTransformDiff(...)` test call sites — added `editable: true`.

5. **`ModelEntityKind` comparisons with `"surface"`/`"part"`** — neither literal is a member of `ModelEntityKind` (`anchor|vertex|edge|wire|face|shell|solid|object|geometry|attribute`); these were dead legacy comparisons that could never be true. Removed the now-meaningless `sel(...)` special case (`editable: kind === "surface" || kind === "part" ? false : editable` → just `editable`) and deleted the one now-untestable assertion `allTargets.every((t) => t.kind !== "surface")`.

6. **`heightSeg.params.to` (possibly-undefined / unknown)** and **`ev.point` (unknown)** — `DisplayItem.params` and `InteractionEvent`'s index-signature fields are intentionally untyped (`Record<string, unknown>` / index signature) at the geometry/actions layer, not discriminated per `kind`. Narrowed locally with `Array.isArray` + `typeof` checks instead of casting.

7. **`InteractionResponse` missing `errors/warnings/infos/data`** in `runSelectionOperationInteraction` — filled in the omitted fields (`errors: []`, `warnings: []`, `infos: []`, `data: result.data ?? null`).

8. **`string` not assignable to `TypologyRef`** — `TypologyRef` is a branded string (`string & { __brand }`); used the same `as TypologyRef` idiom already used pervasively across the codebase for this exact type, and added the `TypologyRef` type import.

9. Import list updates: added `InteractionLengthEntrySpec`, `InteractionScalarEntrySpec`, `StateDefSpec` (geometry), `ActionContextPatch`, `writeInteractionContextVec3` (actions), `TypologyRef` (geometry, test-region import), `registerActionDef` (actions, test-region import).

## Cross-slice dependency (not fixed, out of my slice)

- `shippedInteractionJsons()` (line ~894 area) calls `modelDefinitionInteractionCatalog()`, which exists in `⚙️engine/📔️registry/🟦️component.ts` (owned by the actions/stately/runtime/registry slice) but is not exported (`function modelDefinitionInteractionCatalog` — no `export`). I added it to my import list from `../📔️registry/🟦️component.ts` on the assumption it will be exported. As of my last typecheck run it is still private, producing the one remaining error in my file:
  ```
  ⚙️engine/📄️artifact/🟦️component.ts(11,40): error TS2724: '"../📔️registry/🟦️component.ts"' has no exported member named 'modelDefinitionInteractionCatalog'.
  ```
  Once the registry owner exports it, this resolves with no further changes on my side. I did not edit `📔️registry/🟦️component.ts`.

## Files touched

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📄️artifact/🟦️component.ts` (only file edited)
