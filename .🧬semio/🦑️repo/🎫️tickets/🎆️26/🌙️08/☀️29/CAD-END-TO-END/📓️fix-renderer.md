# Fix Renderer — Report

## Scope
Owned files:
1. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx`
2. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`

## Result
- Renderer file: 16 → 0 errors.
- Framework `ui-react` file: 3 → 0 errors.
- Repo-wide total: 86 → 45 errors, all pre-existing and outside my slice (sibling agents' `spatial-kernel` / `actions`+`stately` / `artifact`+`inferences` files, plus the known `@semio-tech/s-3d-js` package-exports blocker). No new errors introduced anywhere.

## Fixes in the renderer (`📺️renderer/🟦️component.tsx`)
- **`WINDOW_SEARCH_USER` (46) / `SearchInput.placeholder` (new, surfaced after adding the export)**: wrapped the constant's plain-string values with `uiDataLabel(...)` at the `buildInteractionReplSearch` call site so the value satisfies `SearchInput.placeholder: UiLabel | undefined`.
- **`createTemplatePool`/three.js (3291/3296/3339)**: `cadMeshGeometryPool` was instantiated as `createTemplatePool<string>()`, whose factory/value type was hardcoded to `Object3D` — but the pool actually stores `BufferGeometry` (`geometry.dispose()`, `<mesh geometry={geometry}>`). Made `createTemplatePool` generic over the pooled value (`<TKey extends string, TValue = Object3D>`, default preserved) in the framework file below, then instantiated `createTemplatePool<string, ThreeBufferGeometry>()` here. `createTemplatePool` had exactly one other reference (its own declaration) repo-wide, so this was a safe, non-breaking generalization rather than a workaround.
- **`activeModelDefinitionId` (4938)**: `useHostState`'s three arguments disagreed on nullability (`string | null | undefined` controlled prop vs. `(value: string) => void` onChange vs. `() => string | null` initial). Normalized both the controlled prop (`?? undefined`) and the initial-value thunk (`?? defaultModelDefinitionId()`, the same fallback already baked into `defaultInteractionReplChromeState()`) to `string`, matching what the value always is at runtime.
- **Button (6134/6158/6503)**: `ButtonProps.icon` is a genuinely required field (framework contract, confirmed against every other call site repo-wide — all supply one) and there is no `size` variant on `buttonGroupItemVariants` at all (only `variant: default|ghost|outline`). Removed the invalid `size="sm"` and added a real icon per button's actual action: `icon="arrow-right"` (run transition), `icon={interactionMenuOpen ? "chevron-up" : "chevron-down"}` (suggestions toggle), `icon="x"` (clear field) — same convention used elsewhere (`icon="x" text={clearLabel}`).
- **Input `id` (6140/6552/6576)**: `InputProps` requires `id` (via `ElementProps`) for accessibility. Added stable ids: `cad.replCommand` for the REPL command input, `cad.attribute.${defn.id}` for the number/text attribute editors — matching the id already used on the sibling enum `<Select id={`cad.attribute.${defn.id}`}>` in the same function.
- **`UiLabel` (6931)**: a test fixture passed a bare string as `EngagementControl.label`. Replaced with `uiDataLabel("Height")`, the repo's sanctioned literal-data escape hatch (used identically elsewhere in this file and in framework tests).
- **`../🟦️index.ts` (7181)**: dangling import left by the concurrent module-consolidation refactor. Retargeted to the real, already-used-elsewhere-in-this-file location of `ModelSpace`: `.../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`.
- **`.sort()` on readonly array (7327/7328)**: copied before sorting (`[...modelDefinitionPickTargetKinds(...)].sort()`) instead of mutating the readonly array.
- **`"solid"` / `TypologyStylePatternKind` (7407 + a `Required<Pick<...>>` fallout it exposed)**: the real union (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`) is `"none" | "hatch" | "crosshatch" | "dots"` — no `"solid"` member; `"none"` is the correct value for "no pattern / solid fill". Fixing the `kind` then surfaced that `ResolvedTypologyStyle.pattern` requires all of `direction`/`spacing`/`lineWidth`/`color`; filled them with the same defaults the kernel's own `resolveTypologyStyle` uses for the "none" pattern.

## Fixes in the framework file (`⚛️react/📦️index.tsx`)
Minimal, exactly what was needed for its 3 original errors plus one export needed by the renderer's `WINDOW_SEARCH_USER` cluster:
1. **Tutorial camera tuple (5869 ×3, TS4104)**: `tutorialLerp3`'s return type was annotated `readonly [number, number, number]`, but it's assigned into `TutorialCameraState`'s `position`/`target`/`up`, which the generated schema types as a mutable `[number, number, number]`. The function always builds a fresh array literal (never aliases its inputs), so there was no reason for the readonly annotation — changed the return type to the mutable tuple. One-line change, no cast, no `readonly` dropped from any actual data.
2. **`WINDOW_SEARCH_USER` export (new, for the renderer's cluster)**: added `export const WINDOW_SEARCH_USER = { actionPlaceholder, actionPlaceholderActive, suggestionsAria, noMatches }` next to the existing `UI_WINDOW_SEARCH` translation-key map and `ENGAGEMENT_USER` sibling pattern, using the same English copy already present in the `en`/`de` chrome translation bundles (`ui.windowSearch.action` = "Action", `.actionActive` = "Action or value", `.suggestions` = "Suggestions", `.noMatches` = "No matches"). This is the sanctioned default-copy pattern for standalone surfaces that build a `SearchSpec` without going through `<Search>`'s own `useLabel` hooks.

## One out-of-slice edit (flagged explicitly, not one of the two owned files)
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️component.tsx`: made `createTemplatePool<TKey extends string>()` generic as `createTemplatePool<TKey extends string, TValue = Object3D>()` (see above). This file isn't owned by any of the three sibling agents (spatial-kernel / actions+stately / artifact+inferences); `createTemplatePool` had no other caller repo-wide besides its own declaration and the CAD renderer, so the change is behavior-preserving for every existing use and was required to remove the cast-free type mismatch at the renderer's geometry pool.

## Not touched
All 45 remaining repo-wide errors are outside this slice — the `@semio-tech/s-3d-js` blocker (~19 direct + cascading `any`s) and the sibling agents' in-progress files (`🎬️actions`, `🎰️stately`, `📄️artifact`, `📔️registry`, `🧬️typology`, `🧬️schema/💡️inferences`, and `🌐️spatial-kernel`'s three engine files). None were modified.
