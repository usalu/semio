# Fix inferences component.ts (💡️inferences/🟦️component.ts)

## Result
- File errors: 37 → 2 (both cross-slice blockers, see below).
- Repo/tsconfig-wide total at time of this run: 243 `error TS` (was 371 baseline; other sibling agents are editing concurrently so this number moves independently of this slice).
- No new errors introduced in any other file by this change (verified: grepped the post-fix tsc output for every new symbol this file now imports — `TypologyRef`, `SelectionTarget`, `CstElement`, `WireRef`, `EdgeRef`, `VertexRef`, `emptyMeshTransfer` — all remaining hits are pre-existing errors in sibling files, unrelated to this file's imports).

## Root causes fixed (all within this file)
1. **Chevrotain `CstElement` union not narrowed to `IToken`/`CstNode`.** `CstChildrenDictionary[key]` is typed `(IToken | CstNode)[]`, so any `n.children.SomeTerminal?.[0]` is `CstElement | undefined`, not `IToken`. Added two runtime type guards (`asToken`, `asNode`, discriminating on `"tokenType" in e` / `"children" in e`) and used them everywhere a terminal/subrule was accessed without narrowing (`cstToExpr`'s `primaryExpr` branch, `cstToPropMap`, `cstValueLiteralToValue`, the `actionId` lookup in `cstToAst`, and the `LIMIT` integer literal in `cstToAst`). This was the majority of the 37 errors (9).
2. **Missing branded-ref casts in `iterateContainsInverse`.** The `face`/`from.id as FaceRef` cast already existed for the "face" branch; the sibling `shell`/`wire`/`edge`/`vertex` branches were missing the equivalent `as ShellRef`/`as WireRef`/`as EdgeRef`/`as VertexRef` casts. Added the three missing type aliases (`WireRef`, `EdgeRef`, `VertexRef` from `kernelGeometry`) and the casts (4 errors).
3. **Untyped generator IIFEs in `traverseRel` widened `kind` to `string`.** The `ADJACENT_TO` and `HAS_VERTEX` inline `function*` branches yielded object literals without a contextual type, so `kind: "solid"` widened to `string`, making the union incompatible with `ModelEntityRef`. Added explicit `Generator<EntityHandle>` return-type annotations to both IIFEs and the empty fallback (3 errors).
4. **`ctx.preview` typo** — `ConstructQueryContext` (owned by the actions slice) only has `kernel`, not `preview`. `SpatialKernel extends SpatialPreviewKernel`, so `ctx.kernel` is directly assignable where a preview kernel is expected. Fixed the one call site in `runTransformationCall` (1 error).
5. **Mutating a `Readonly<Record<...>>`.** `ConstructQueryRow` is `Readonly<Record<string, unknown>>`; the return-projection loop built it in place. Built it in a local mutable `Record<string, unknown>` instead and yielded that (still structurally assignable to the readonly type) — no `readonly` was dropped from the public type (1 error).
6. **Test-only `QueryTestKernel` stub mismatches:**
   - `createBoxFromCorners()` was overridden with zero params while the base takes one, and the stub's own `createBoxFromCornersDiff` called it with an argument. Restored the parameter (unused) on the override (1 error).
   - `tessellate()` returned an ad hoc `{ positions, indices }` shape instead of `MeshTransfer`. Switched to the existing `emptyMeshTransfer()` helper from `@semio-tech/s-3d-js` (1 error).
   - `selectionTargets` test `seed` array literal widened `kind: "solid"` to `string`. Annotated it `readonly SelectionTarget[]` (imported the type) (1 error).
   - Two vitest cases built `model.objects[...]` with a raw string `typology` (needs `TypologyRef`) and read a non-existent `ModelDiff.solid` (the diff has no such field — the solid id was already known from the `solidRef("box")` passed into `boxModelDiff`). Restructured both tests to keep the `SolidRef` in a local `solid` variable and cast the typology literal `as TypologyRef` (4 errors).
   - Fixing `createBoxFromCorners`/`tessellate` also transitively cleared all 9 "`QueryTestKernel` is not assignable to `SpatialKernel`" errors at the various `runConstruct(...)` call sites in the test block — those were downstream of the two stub-shape mismatches above, not of the `id`/`operations` issue below.

## Cross-slice blocker (left in place, 2 errors)
```
component.ts(1448,23): error TS2416: Property 'id' in type 'QueryTestKernel' is not assignable to the same property in base type 'BrepjsKernel'.
component.ts(1449,23): error TS2416: Property 'operations' in type 'QueryTestKernel' is not assignable to the same property in base type 'BrepjsKernel'.
```
`BrepjsKernel` (in `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`, lines 2571 & 2574 — not my slice) declares:
```ts
readonly id = "brepjs-opencascade";
readonly operations = ["solid.createBox", ...] as const;
```
With no explicit type annotation, TypeScript infers the **literal** type `"brepjs-opencascade"` for `id` and a fixed 7-tuple literal type for `operations` (both behave like `const`-inference on `readonly` class fields). The `SpatialKernel` interface itself only requires `id: string` / `operations: readonly string[]`, so the class body is stricter than the interface it implements. Any subclass (here, the vitest `QueryTestKernel` stub) that legitimately needs a *different* `id`/`operations` value — which any pure-string/array is, since a different literal is never assignable to the base's narrower literal type — cannot override them, no matter how it's typed on the subclass side (verified: widening the override to `readonly id: string` still fails, since override types must be assignable *to*, not from, the base's inferred type).
- **Fix needed in the sibling file:** annotate `readonly id: string = "brepjs-opencascade";` and `readonly operations: readonly string[] = [...] as const;` (or drop the redundant `as const`) in `🧱️brepjs/🟦️component.ts`. This is outside my owned file — flagging for whichever slice owns `⚙️engine/🧱️brepjs/🟦️component.ts` (adjacent to but not explicitly listed among the five sibling slices in my brief; likely bundled with "spatial-kernel (geometry/spatial)").
- No workaround was attempted inside my file since every option either drops type safety (forbidden) or requires editing the base class (outside my slice).

## Files touched
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️component.ts` (only file edited)
