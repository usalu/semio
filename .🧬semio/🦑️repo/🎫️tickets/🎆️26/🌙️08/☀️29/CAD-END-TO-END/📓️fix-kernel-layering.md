# Fix kernel layering (geometry.ts ↔ actions/artifact/typology circularity)

## Root cause confirmed
The 11 "genuinely blocked" errors from `📓️fix-spatial-kernel.md` were not a real architectural
circularity in the sense of needing a provider/registration hook — every one of the six symbols
`geometry.ts` called into (`parseActionSpec`, `shippedSpatialInteractionCatalog`,
`loadSpatialInteraction`, `typologyConstructKitByInteraction`, `SelectionOperationInteractionDef`,
`selectionOperationsForModelDefinitionFromActions`) turned out, on inspection of their real bodies in
`actions.ts` / `artifact.ts` / `typology.ts`, to have **zero dependency on cad-runtime machinery**
(no `ActionRegistry`, no `StateEngine`, no execution/interpretation of `kernel.call` steps — only
parsing, string-based id derivation, and reads of the same shared `modelDefinitionAssetModules`
singleton geometry.ts already holds a duplicate reference to, per the prior agent's precedent).

This matches `✏️s/🔌️plugins/📐️cad/AGENTS.md`'s layering intent directly: parsing a declarative
`spatial.action/v1` / `spatial.interaction/v1` document is a kernel concern (geometry.ts already
owns `parseInteractionSpec`, `parseTypologySpec`, `parseAttributeDefinitionSpec`,
`parsePropertyDefinitionSpec` — `parseActionSpec` was simply missing from that list, an omission
from the same file-split that caused the rest of the ticket). Selection commands
(`selection.selectAll` etc.) are generic spatial-kernel primitives, not cad-specific interpretation.
Only actual *execution* (running an action's `kernel.call` steps, driving a `StateEngine`) is a
cad-plugin concern — and geometry.ts never called into that.

`🔒️layering.json` only encodes the framework-vs-implementation direction (both spatial-kernel and
the cad plugin are `"implementation"`), so it doesn't directly adjudicate this intra-implementation
boundary; the applicable rule is `cad/AGENTS.md`'s own description of the kernel as the
domain-neutral geometry/topology layer beneath cad-plugin actions/interactions.

## Fix — moved (duplicated, per established registry.ts precedent), not hooked
No provider/registration hook was needed. Following the exact idiom the prior agent already
established for `registry.ts`'s catalog caches (same `ephemeralBox`/`ephemeralMap` string keys ⇒
shared runtime state, not duplicated state), I added natively to `geometry.ts`:

1. **`ActionParameterSpec` / `ActionStepSpec` / `ActionSpec` + `parseActionSpec`** — moved next to
   the other `parse*Spec` functions in the `📜️Spec` region. `actions.ts` keeps its own identical
   copy (I did not touch actions.ts, sibling-owned); recommend it later imports these from
   geometry.ts instead.
2. **`modelDefinitionInteractionCatalog()`, `ModelDefinitionInteractionFixture`,
   `shippedInteractionJsons()`, `interactionFixtureRow()`, `shippedSpatialInteractionCatalog()`,
   `COMPILED_INTERACTION_BY_ID`, `loadSpatialInteraction()`** — reconstructed verbatim from
   `artifact.ts`, reading the same `modelDefinitionAssetModules` singleton geometry.ts already
   holds, and the same `COMPILED_INTERACTION_BY_ID` ephemeralMap key `artifact.ts` uses (so
   `registry.ts`'s existing `interactionCompileCacheClear` hook — which artifact.ts wires to clear
   this exact map — already invalidates geometry.ts's copy for free; no new invalidation plumbing
   needed).
3. **`typologyObjectPascalFromLabel`, `TypologyConstructKit`, `typologyConstructAssetIds`,
   `typologyConstructKitByInteractionCache`, `typologyConstructKitByInteraction()`** — reconstructed
   verbatim from `typology.ts`, depending only on `listModelDefinitionTypologies()` (already native
   to geometry.ts).
4. **`SelectionApplyOperation`, `SelectionOperationInteractionDef`, `SELECTION_INTERACTION_KEYS`,
   `SELECTION_ACTION_META`, `selectionOperationDefForActionId()`** — reconstructed verbatim from
   `artifact.ts`, and `listSelectionOperationsForModelDefinition` now builds the list directly
   (inlining what `selectionOperationsForModelDefinitionFromActions` did) instead of calling out —
   it only ever needed `listActionsForModelDefinition`, already native to geometry.ts.

No sibling-owned file was edited. `artifact.ts`'s/`actions.ts`'s/`typology.ts`'s own copies of these
six symbols are now dead weight from geometry.ts's point of view (still fully functional for their
own callers) — **recommend whichever agent picks up `actions.ts`/`artifact.ts`/`typology.ts` next
deletes their private copies and imports `ActionSpec`/`parseActionSpec`/
`SelectionOperationInteractionDef`/`TypologyConstructKit`/`typologyConstructKitByInteraction`/
`loadSpatialInteraction`/`shippedSpatialInteractionCatalog` from geometry.ts instead**, the same way
`registry.ts`'s catalog getters should eventually be replaced by geometry.ts's. Out of my slice —
not touched.

## Nothing left blocked
All 11 previously-blocked errors are resolved with real implementations, not stubs/casts/hooks. No
`any`, no casts, no `@ts-ignore`, no provider/hook indirection was needed or used.

## Verification
Ran the real `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json` (`npx tsc --noEmit`, ~3 min)
and, separately, the same config with a scratch `paths` override pointing `@semio-tech/s-3d-js`
straight at its real file (`🧰️framework/🔨️modules/🧊️3d/🟦️.ts`) to see through the still-unfixed
`ERR_INVALID_PACKAGE_TARGET` blocker (its `package.json` `exports` still reads
`"." : "../../🟦️.ts"`, escaping the package dir — untouched, not my file, flagged again below).

**My 3 files, real tsc (s-3d-js cascade included):**
| File | Prior report ("genuinely blocked") | After this fix |
|---|---|---|
| `📐️geometry/🟦️component.ts` | 11 genuine + s-3d-js cascade | **5**, all s-3d-js cascade (2× TS2307 + 3× TS7006) — **0 genuine** |
| `🗺️spatial/🟦️component.ts` | 0 genuine | 2, s-3d-js cascade only (unchanged) |
| `🧱️brepjs/🟦️component.ts` | 0 genuine | 21, s-3d-js cascade only (unchanged) |

**With the s-3d-js `paths` workaround (isolates genuine errors):** geometry.ts **0**, spatial.ts
**0**, brepjs.ts **0** — my whole slice is clean.

**Repo-wide** (real tsc, same command as the ticket's typecheck instructions): **61** `error TS`
lines total (down from the prior report's 103; other sibling agents are also still active on
`renderer.tsx` 16, `stately.ts` 6, `inferences.ts` 3 — the remainder besides my slice's 28 s-3d-js-
cascade lines is `actions.ts`/`artifact.ts`/`registry.ts`/`typology.ts` each only 2, all s-3d-js
TS2307, not new). I introduced no errors anywhere outside my slice — verified every error in
`actions.ts`, `artifact.ts`, `registry.ts`, `typology.ts` is the pre-existing s-3d-js TS2307
cascade, none reference any name I added.

## Unrelated blocker, still present, not fixed (out of scope, flagged before)
`🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/package.json`'s `exports` still escapes its
package directory. Untouched per instructions; this is the same blocker the prior agent flagged via
`spawn_task` (task_e28b586e).
