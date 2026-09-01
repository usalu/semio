# Fix: editor/engine actions, stately, runtime, registry

## Scope
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/`:
`🎬️actions/🟦️component.ts`, `🎰️stately/🟦️component.ts`, `🏃️runtime/🟦️component.ts`, `📔️registry/🟦️component.ts`.
`🧬️typology/🟦️component.ts` was inspected only (0 errors before and after — untouched).

## Before / after (this slice)
| file | before | after |
|---|---|---|
| `🎬️actions/🟦️component.ts` | 23 | 5 |
| `🎰️stately/🟦️component.ts` | 8 | 4 |
| `🏃️runtime/🟦️component.ts` | 5 | 0 |
| `📔️registry/🟦️component.ts` | 1 | 0 |
| `🧬️typology/🟦️component.ts` | 0 | 0 |
| **slice total** | **37** | **9** |

Repo-wide `tsc` total: 371 (baseline) → 149 (current run, includes concurrent sibling progress on renderer/spatial/geometry/inferences/artifact — not attributable to this slice alone).

## Fixes applied
- **`📔️registry/🟦️component.ts`**: `modelDefinitionActionCatalog()` was already correctly implemented but not `export`ed — added `export`. Removed the dead `typologyStyleCache` box (and its reset line): it referenced a `ResolvedTypologyStyle` type that exists nowhere in the repo, and the cache itself was never populated or read anywhere — orphaned leftover from a module split, not a real feature. (1 error fixed.)
- **`🏃️runtime/🟦️component.ts`**: `shippedCadComputerContributionsJson`/`syncCadComputerContributions` built/read a legacy `{ kind, appId, moduleId, label, iconId, computersJson }` nested `contribution` object that predates the open-contribution-mechanism migration. Real `ProgramContributionEntry` (`🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`) is `{ pluginId, topicContribution?: { topic, payload } }`. Rewrote both functions against the Rust-authoritative payload shape `CadComputerTopicPayload { appId, moduleId, computersJson }` (`✏️s/…/💡️inferences/🦀️component.rs` `CAD_COMPUTER_TOPIC = "cad.computer"`); dropped the unused `label`/`iconId` fields (not part of that struct, not read anywhere in TS either). (5 errors fixed.)
- **`🎬️actions/🟦️component.ts`**:
  - Added `AnchorAttachment`, `AnchorRecord`, `AnchorRef`, `VertexRecord`, `FaceRecord` to the existing import from spatial-kernel `📐️geometry/🟦️component.ts` (all already exported there). (6 errors fixed.)
  - Imported `modelDefinitionActionCatalog` from `../📔️registry/🟦️component.ts` (now exported, see above). (1 error fixed.)
  - Added `import type { InteractionRuntime } from "../📄️artifact/🟦️component.ts"` for `abortActiveInteractionSession`'s parameter type (type-only import avoids the runtime circular dependency — `artifact/component.ts` already imports plain values back from `actions/component.ts`). (1 error fixed.)
  - Fixed `command.assignDirectionFromPoint`'s `event?.point ?? origin`: `InteractionEvent`'s index signature types every field `unknown`, which collapsed the union to `{}` against `vec3Param`'s `Vec3` fallback param. Cast `event` to `{ readonly point?: Vec3 }` locally, matching the existing cast style used elsewhere in this file (`event as SelectionEvent`, etc.). (1 error fixed.)
  - Added a local `InteractionEventModifiers` interface (`shift?/ctrl?/alt?/meta?: boolean`) and retyped `selectionTargetsWithMode`'s `modifiers` parameter and its one call site to use it instead of `InteractionEvent["modifiers"]` (which is `unknown`). (4 errors fixed.)

## Left unfixed — blocked on files outside my slice
- **`🎬️actions/🟦️component.ts` (5 errors)** and **2 cascading `TS7006` implicit-any errors** (`h` in two `.find` callbacks) all stem from the same root cause: `findState`, `listFinalInteractionStates`, and `MODEL_ENTITY_KINDS` are real, correct implementations that already exist in `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` (lines 22, 761, 770) but are module-private (no `export`). Per the brief, spatial-kernel/geometry is the kernel sibling agent's file — I added the correct import names on my side and left them as a `export`-needed dependency rather than duplicating the logic or editing that file myself.
- **`🎰️stately/🟦️component.ts` (4 errors)**: test-only `StubKernel`/`MeasureParityKernel` subclass `BrepjsKernel` (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts` lines 2571/2574, not my slice). `BrepjsKernel.id`/`operations` are declared with inferred literal types (`"brepjs-opencascade"` / a specific `as const` tuple) instead of the `SpatialKernel` interface's `string` / `readonly string[]`, so no subclass can declare a different id/operations list — this is a base-class typing defect, not something fixable from the subclass side without `any`/casts. The `inferences` sibling slice hit the identical issue independently (see `📓️fix-inferences.md`) and flagged the same fix: widen `readonly id: string = "brepjs-opencascade"` and `readonly operations: readonly string[] = [...]` in `🧱️brepjs/🟦️component.ts`.

## Verification
Ran `npx tsc -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit 2>&1 | grep "error TS"` after edits (repo-wide, ~2–5 min). Confirmed via per-file grep that `🏃️runtime` and `📔️registry` are now error-free, `🧬️typology` remains error-free (untouched), and no new errors appeared in files outside this slice as a result of my changes (spot-checked the 2 unrelated `🧊️3d` `index.ts` errors — pre-existing, about a `flow_core` wasm pkg, unrelated to anything I touched).

No `any`, `as unknown as`, `@ts-ignore`/`@ts-expect-error`, or `readonly` drops were used anywhere in this pass.
