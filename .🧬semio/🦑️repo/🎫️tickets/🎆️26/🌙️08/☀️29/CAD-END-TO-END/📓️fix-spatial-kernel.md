# Fix spatial-kernel (geometry / spatial / brepjs)

## Scope
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`

## Root cause
Confirmed the ticket's hypothesis: these files were split out of one larger module and cross-file
references were never repaired. Two distinct patterns:

1. **Missing `export` keywords.** `geometry/🟦️component.ts` defined `AnchorRef, AnchorRecord,
   AnchorAttachment, VertexRecord, EdgeRecord, WireRecord, FaceSurface, FaceRecord, ShellRecord,
   SolidPrimitive, SolidRecord, KernelGeometryJson` as **non-exported** local type aliases
   (`type X = kernelGeometry.X`) even though `spatial/🟦️component.ts` (and downstream
   actions/artifact/renderer files) already imported them. Fix: added `export` to each alias.
   `spatial/🟦️component.ts`'s own import line only pulled 7 of the ~16 names it actually uses —
   fixed to import everything it needs.

2. **Orphaned shared-singleton state.** `geometry/🟦️component.ts` calls a dozen module-definition
   catalog/cache functions (`modelDefinitionAssetModules`, `defaultModelDefinitionIdCache`,
   `modelDefinitionManifestCatalog`, `modelDefinitionTypologyCatalog`,
   `modelDefinitionAttributeCatalog`, `modelDefinitionPropertyCatalog`, `modelDefinitionStatCatalog`,
   `modelDefinitionTransformationModules`, `modelDefinitionFolderIdMapCache`,
   `typologyOwnerByIdCache`, `actionOwnerByIdCache`, `interactionOwnerByIdCache`,
   `attributeOwnerByIdCache`, `propertyOwnerByIdCache`, `statOwnerByIdCache`, `typologyStyleCache`)
   that no longer existed in the file. The canonical implementation is still alive, verbatim, in the
   sibling (not-mine) file `…/⚙️engine/📔️registry/🟦️component.ts`, module-private (no `export`).
   `registry/🟦️component.ts` already imports `TypologyRef` (type-only) from `geometry`, so importing
   the other direction would be circular. Fix: reconstructed the identical block (interface
   `ModelDefinitionAssetModules`, `emptyModelDefinitionAssetModules`, the `ephemeralBox(...)` cache
   declarations, and the catalog getter functions) locally inside `geometry/🟦️component.ts`, using
   the **exact same `ephemeralBox` key strings** as `registry.ts`. `ephemeralBox` is a global
   string-keyed singleton (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`), so this is not duplicated state —
   both files hold independent references to the identical runtime box/map, exactly like the
   pre-existing `COMPILED_INITIAL_CONTEXTS` / `interactionCompileCacheClear` pattern already used
   across these files. One divergence from `registry.ts`'s copy: `typologyStyleCache` had to be
   typed `Map<...> | null` instead of `ReadonlyMap<...> | null` because `geometry.ts`'s
   `resolveTypologyStyle` actually **writes** to it (`registry.ts`'s copy is read-only/reset-only, so
   its `ReadonlyMap` typing was accidentally fine there but wrong for this file).

## Other fixes in this slice
- `ExprEnv.preview` was `readonly preview: SpatialPreviewKernel` (required) but two call sites
  (`mergeInteractionCallOutputs`, `staticInitialContext`) construct envs without one, and a test
  constructs `evalGuard(g, { context: {...} })`. Made `preview` optional and added `?.` at the three
  `evalExpr` call sites that dereference it (`abs`, `distance`, `fold`), matching the existing
  optional-preview pattern already used at `readModelEntityProperty`. Verified no downstream file
  (`actions.ts` etc.) reads `env.preview` without going through `SpatialKernel`'s separate, still-
  required `ctx.preview` — so this widening can't regress anyone else.
- `hashVertexPosition`: replaced an invalid `number[] as Vec3` cast with an explicit 3-tuple literal.
- `parseModelJson`: replaced the invalid `Record<string, unknown> as KernelGeometryJson` whole-object
  cast with a per-field cast (`geometry.anchors as AnchorRecord[]`, etc.), same pattern already used
  one line above for `r.objects as SpatialObjectRecord[]`.
- `lookupGuard` was called but never defined; reconstructed from usage + the existing `guardNames`/
  `NamedGuard` (`{name, expr}`) shape: `spec.guards?.find((g) => g.name === name)?.expr ?? null`.
- Test-code `TypologyRef` string-literal assignments (`typology: "energy.energy.hull"`, etc.) were
  missing the `as TypologyRef` cast that's already the established pattern everywhere else in the
  same file/tests — added it (4 sites across geometry.ts + spatial.ts).
- One test used `kind: "surface"` for a `SelectionTarget`, but `"surface"` is not a `ModelEntityKind`
  (kinds are anchor/vertex/edge/wire/face/shell/solid/object/geometry/attribute) — the test's intent
  ("a kind outside `accept: ["face"]`") is unaffected by swapping it for `"edge"`.
- `spatial/🟦️component.ts`'s `EntityDiff.{added,modified,removed}` are `readonly` (correct, for the
  public `ModelDiff` contract) but `applyEntityDiff`/`applyModelDiff` mutate them while building the
  inverse patch. Added `MutableEntityDiff`/`MutableModelDiff` (identical shape, no `readonly`) used
  only for the in-progress accumulator locals; the function signatures/return types are unchanged
  (`ModelDiff` stays fully readonly for every consumer).
- `modelDiffSyncNurbsThroughEdgesForMovedVertices`: called by `spatial/🟦️component.ts` but not
  defined there. Found its real (but dead/unused, non-exported) implementation sitting in
  `…/⚙️engine/🎬️actions/🟦️component.ts` — actions.ts is downstream of spatial.ts, never actually
  calls it, and only imports `isEmptyModelDiff` etc. from spatial.ts, so this was clearly misplaced
  during the split rather than a real cross-slice dependency. Moved the implementation verbatim into
  `spatial/🟦️component.ts` (its only real call site). Recommend the dead copy in `actions.ts` be
  deleted by whoever owns that file — out of my slice, not touched.
- `ActionResult` / `ActionContextPatch`: `SpatialKernel.executeAction` (in `spatial.ts`) returns
  `ActionResult`, whose only real definition lives in `…/⚙️engine/🎬️actions/🟦️component.ts`
  (downstream of spatial.ts — importing it back would be circular). Reconstructed both interfaces
  verbatim in `spatial/🟦️component.ts` from the `actions.ts` source (dropped the unused
  `<TData = unknown>` generic default usage note: kept the generic for fidelity). Recommend
  `actions.ts` switch to importing `ActionResult`/`ActionContextPatch` from `spatial.ts` instead of
  keeping its own copy — out of my slice.
- `inlineModelSpaceFixtureJson` (brepjs.ts) was typed to return `ModelSpaceJson`, but it deliberately
  emits the *inline/pre-normalization* row format that `materializeInlineObjectPrimitives` in
  geometry.ts is built to consume (`primitives` as an array of `{kind,id,...}` topology rows, not the
  normalized `Record<string,string>` slot map `SpatialObjectRecord.primitives` requires). Introduced
  a small accurate local type `InlineModelSpaceFixtureJson` instead of force-casting through
  `unknown`. Not used by any other file in-repo today, so this is a pure type-accuracy fix with zero
  call-site impact.

## Genuinely blocked (left as errors, needs a decision outside my slice)
11 errors remain in `geometry/🟦️component.ts`, all one cluster: it calls into functions whose real,
non-trivial implementations live only in downstream files that themselves import heavily from
`geometry.ts`/`spatial.ts` — a genuine circular dependency, not a missing-export bug:
- `parseActionSpec` (×2, lines 2832, 3000) — real impl + `ActionSpec`/`ActionStepSpec` parsing in
  `…/⚙️engine/🎬️actions/🟦️component.ts`.
- `shippedSpatialInteractionCatalog`, `loadSpatialInteraction` (×3, lines 2885, 2891, 2982, 3004) —
  real impl in `…/⚙️engine/📄️artifact/🟦️component.ts`.
- `typologyConstructKitByInteraction` (line 2963) — real impl in `…/⚙️engine/🧬️typology/🟦️component.ts`.
- `SelectionOperationInteractionDef` (×2, lines 3024, 3075), `selectionOperationsForModelDefinitionFromActions`
  (line 3025) — real impl in `…/⚙️engine/📄️artifact/🟦️component.ts`, itself pulling from `actions.ts`'s
  `ActionRegistry`/`modelDefinitionActionRegistry`.

I did not duplicate these (unlike the registry.ts caches, these are non-trivial parsers/registries,
not simple shared-singleton lookups — duplicating them would drift from the canonical logic).
Resolving this needs an architectural call: either move these functions down into `geometry.ts`
(their only caller chain that needs them at this layer), or move `geometry.ts`'s calling functions
(`listActionsForModelDefinition`, `actionOwnerById`, `listSpatialInteractionsForModelDefinition`,
`actionIdsReferencedByInteractionSpec`, `listSelectionOperationsForModelDefinition`, etc.) further
downstream. Left untouched, precisely as forbidden-fix rules require.

## Unrelated blocker flagged, not fixed (out of scope)
A concurrent session's file rename left `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/package.json`
with `"exports": {".": "../../🟦️.ts"}` — an path escaping the package directory, which Node/TS
module resolution rejects (`ERR_INVALID_PACKAGE_TARGET`). This breaks `@semio-tech/s-3d-js`
resolution repo-wide (TS2307 everywhere it's imported, i.e. most of the cad plugin) independent of
anything in this ticket. Flagged via `spawn_task` (task_e28b586e) rather than fixed directly — not
my slice. Verified my 3 files are unaffected by it (used a scratch tsconfig with a `paths` override
pointing straight at the real file to typecheck around the broken package.json).

## Results (errors in my 3 files only)
| File | Baseline | After fix (real tsc, s-3d-js bug included) | After fix (s-3d-js bug worked around) |
|---|---|---|---|
| `📐️geometry/🟦️component.ts` | 79 | 16 | **11** (all in the "genuinely blocked" cluster above) |
| `🗺️spatial/🟦️component.ts` | 59 | 2 | **0** |
| `🧱️brepjs/🟦️component.ts` | 2 | 21 | **0** |
| **Total (my slice)** | **140** | 39 | **11** |

All of the "after fix (real tsc)" counts above beyond the 11 genuinely-blocked ones are the
`@semio-tech/s-3d-js` module-resolution cascade (TS2307 plus the "implicitly any" errors it
triggers downstream), not real bugs in my files — see the flagged blocker above.

## Repo-wide total
Baseline was 371 `error TS` lines. With the `paths` workaround for the unrelated s-3d-js blocker,
repo-wide is now 103 (down from 371), spread across sibling in-progress files I do not own
(`renderer.tsx` 48, `stately.ts` 6, `actions.ts` 5, `inferences.ts` 2, `artifact.ts` 1) plus a few
unrelated `framework/ui` files (26+1+1+1) outside the cad plugin entirely. None of these regressed
versus baseline — every sibling file's count went down or stayed flat after my export fixes
(e.g. `artifact.ts` 39→1, `inferences.ts` 37→2, `actions.ts` 23→5), consistent with them consuming
exports I restored. I did not edit any file outside my slice.
