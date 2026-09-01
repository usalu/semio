# Fix: editor/engine actions, stately (round 2)

## Scope
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/`:
`🎬️actions/🟦️component.ts`, `🎰️stately/🟦️component.ts`, `📔️registry/🟦️component.ts`, `🧬️typology/🟦️component.ts`.

## Before / after (genuine errors, excluding the `@semio-tech/s-3d-js` TS2307 cascade)
| file | before | after |
|---|---|---|
| `🎬️actions/🟦️component.ts` | 8 | 0 |
| `🎰️stately/🟦️component.ts` | 4 | 4 (handoff, see below) |
| `📔️registry/🟦️component.ts` | 0 | 0 |
| `🧬️typology/🟦️component.ts` | 0 | 0 |

Repo-wide `tsc` total: 86 (baseline this run) → 61 (final). The `@semio-tech/s-3d-js` `exports` blocker is still open (2 errors each in `🎬️actions`, `🎰️stately`, `📔️registry`, `🧬️typology` = 8 total); untouched, per instructions.

## Fixes applied
- **`🎬️actions/🟦️component.ts`**:
  - `MODEL_ENTITY_KINDS`, `findState`, `listFinalInteractionStates` are not exported from the kernel-owned `spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` (confirmed still private there; that file has live uncommitted changes from the kernel sibling agent). Per the established idiom (`geometry.ts`'s own private `findState`, `artifact.ts`'s own `lookupGuard`), added private per-file copies instead of depending on the kernel's exports:
    - `findState`/`listFinalInteractionStates` (identical bodies to the kernel's, using the already-exported `InteractionSpec`/`StateDefSpec` types) placed inside the `🎬️Statechart` region.
    - `MODEL_ENTITY_KINDS` rebuilt as `new Set<string>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object", "geometry", "attribute"])`, reusing the kernel's already-exported `PRIMITIVE_MODEL_ENTITY_KINDS` — this is the exact same construction the kernel file itself uses internally (`modelDefinitionSelectionEntityKinds`), so no literal-list duplication/drift risk. (Fixed the 3 direct errors plus 2 cascading `TS7006` "`h` implicitly any" errors that were a symptom of `findState`'s import failing.)
  - `modelDiffTransformNurbsPolesOnEdges`'s `pole`/`index` (2 lines, 3 errors) — `curve.poles.map(...)`/`.every(...)` inferred `any` because `EdgeRecord = kernelGeometry.EdgeRecord` and `kernelGeometry` comes from the still-broken `@semio-tech/s-3d-js` import (confirmed with an isolated repro: mapping over an `any`-typed array still trips `noImplicitAny` on the callback params — a real TS quirk, not user error). Annotated the callback params explicitly with the file's own already-imported `Vec3`/`number` types (matching the sibling `mapPoint`/`vec3Eq` signatures and the identical pattern already used two functions below at the explicitly-typed `let poles: Vec3[] | null`). This is a forward-correct annotation, not a widening — once the kernel sibling's `s-3d-js` blocker resolves, `Vec3` becomes its real type and the annotation is exactly right.
- **`📔️registry/🟦️component.ts`**: found `modelDefinitionInteractionCatalog()` — correctly implemented, same shape as `modelDefinitionActionCatalog` (which a prior pass already exported) — but not `export`ed. `../📄️artifact/🟦️component.ts` (owned by the artifact/inferences sibling agent, outside my slice) was importing it and failing with `TS2724`. Added `export`; did not touch the artifact file. Verified no other private catalog helper (`modelDefinitionTypologyCatalog`, `…ManifestCatalog`, `…AttributeCatalog`, `…PropertyCatalog`, `…StatCatalog`, `…TransformationModules`) is referenced outside this file, so left those private.

## Left unfixed — handoff
- **`🎰️stately/🟦️component.ts` (4 errors, unchanged from prior pass)**: `StubKernel`/`MeasureParityKernel` (test-only, `import.meta.vitest`-gated) extend `BrepjsKernel` and correctly, honestly declare real operation lists for what they stub. `BrepjsKernel` (`🧱️brepjs/🟦️component.ts`, kernel sibling's file, confirmed live-modified right now via `git status`) still declares `readonly id = "brepjs-opencascade"` and `readonly operations = [...] as const` with no `string`/`readonly string[]` annotation, so subclasses are structurally forbidden from declaring a different id/operations — TS2416 is emitted on the subclass regardless of what it does (verified: overriding a narrower-inferred base property with a wider explicit type is not assignable, by TS's covariance rule — no non-`any`/non-cast fix exists on the subclass side). This is the identical diagnosis the previous agent and the inferences sibling independently reached. Reporting as handoff rather than editing `🧱️brepjs/🟦️component.ts` (not my slice, and being actively edited by another agent right now).

## Verification
Ran `npx tsc -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit 2>&1 | grep "error TS"` before and after. Grepped for all four slice files plus a broader diff of the full error list to confirm no new errors appeared outside my slice. No `any`, `as unknown as`, `@ts-ignore`/`@ts-expect-error`, `unknown`/`object` widening, or `readonly` drops were used.
