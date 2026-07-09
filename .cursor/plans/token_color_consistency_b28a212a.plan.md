---
name: Token Color Consistency
overview: Introduce one shared, tokens.json-derived color resolver in @semio-tech/ui-styling and migrate every renderer/fixture/catalog across the monorepo from raw hex to design-token references, eliminating off-palette colors (e.g. WIRES) and duplicated per-file resolvers.
todos:
 - id: ticket
   content: Read repo://goals and open/reopen a repo MCP ticket for the UI token-color consistency task
   status: completed
 - id: tokenmap
   content: Extend ui/styling/script.ts generate to emit a TS token map (STYLING_TOKENS) and register generate in launch.json
   status: completed
 - id: resolver
   content: Add shared resolveColorHex/resolveColorRgba/tokenVar resolver in ui/styling/js (DOM probe + headless token-map fallback, hex normalization, cache)
   status: completed
 - id: puzzle2d
   content: Route puzzle/2d/react chrome, vello theme, headless fallback, and serializeKindCatalogBundle (resolve catalog colors to hex) + DEFAULT_HANDLE_KIND_CATALOG through shared resolver
   status: completed
 - id: three
   content: Replace puzzle/3d + infinite/world/r3f local resolvers and inline hex (incl. rubber band, grids) with shared resolver + token refs
   status: completed
 - id: cad-gizmo-5d
   content: Migrate cad SPATIAL_SCENE_COLOR_FALLBACK, ui/react getComputedColor + gumball/gltf colors, and puzzle/5d grip default to shared resolver + token refs
   status: completed
 - id: fixtures
   content: Migrate WIRES metabolism fixture, sketchpad kit WIRES catalogs, and mindmap react test data to token references
   status: completed
 - id: tests
   content: Extend existing vitest files (styling, puzzle 2d/3d, wires play) to cover resolver + token-hex catalog serialization
   status: completed
 - id: regen-verify
   content: Run script.ts generate, rebuild puzzle/2d/rs, run affected vitest, verify runtime colors, then close ticket with summary
   status: completed
isProject: false
---

# Token Color Consistency Across Monorepo

## Problem

`ui/styling/tokens.json` is the single source of truth (40 `--color-*` tokens + semantic vars in `js/ui.css`). But many places bypass it:

- WIRES fixtures/catalogs use Tailwind hex (`#64748b`, `#0ea5e9`, `#a855f7`, `#22c55e`) in [reasoning/mindmap/wires/fixture/metabolism.wires.json](reasoning/mindmap/wires/fixture/metabolism.wires.json) and [compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts) (`SKETCHPAD_KIT_WIRES_KIND_CATALOGS`).
- Each renderer reimplements its own CSS-var resolver + inline hex fallback table: `puzzle2dProbeCssComputed` ([puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx)), `probeCssComputed`/`cssColorForThree` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) and [infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx)), `readSpatialCssColor` ([cad/js/renderer/index.tsx](cad/js/renderer/index.tsx)), `getComputedColor`/`getComputedColorForGltf` ([ui/react/index.tsx](ui/react/index.tsx)).
- Off-palette hex in gizmo (`GUMBALL_AXIS_COLORS` `#ef4444/#22c55e/#3b82f6`), CAD (`SPATIAL_SCENE_COLOR_FALLBACK` gold/gray), 3D rubber band `0xf472b6`, grids `0xb8c4d0/0x6a7a8a`, puzzle default handle `#94a3b8`.

Constraint: the Vello WASM parses node/edge kind `color` as **hex only** and Three.js needs hex/number, so catalog/fixture token references must be resolved to concrete hex by the TS host before reaching those layers.

## Approach

One shared resolver in `@semio-tech/ui-styling/js`, fed by a generated TS token map, normalizing any token reference / `var()` / `color-mix()` to hex (and an RGBA variant). All renderers import it; all fixtures/catalogs store token references instead of raw hex.

### 1. Token map generation

- Extend [ui/styling/script.ts](ui/styling/script.ts) `generateStylingArtifacts()` to also emit a generated TS module (e.g. `ui/styling/js/tokens.generated.ts`) exporting `STYLING_TOKENS: Record<string,string>` (hex by key, from `tokens.json`) alongside existing CSS/C# outputs. Keep regeneration via `bun ./script.ts generate`.
- Register the styling generate command in [.vscode/launch.json](.vscode/launch.json) following existing grouping (currently missing).

### 2. Shared resolver (new region in [ui/styling/js/index.ts](ui/styling/js/index.ts))

- `tokenVar(key)` -> `var(--color-<key>)`.
- `resolveColorHex(ref, fallbackKey)`: DOM path probes computed style (hidden element, dark-class aware) and converts oklab/`color-mix`/`rgb()` to `#rrggbb[aa]` via canvas (port `cssColorForThree`); headless path resolves `var(--color-<key>)` against `STYLING_TOKENS`, else uses `STYLING_TOKENS[fallbackKey]`. No raw hex in callers.
- `resolveColorRgba(ref, fallbackKey)` for Vello theme JSON.
- Cache results (port `_elementsComputedColorCache`).

### 3. Replace per-renderer resolvers with the shared one

- puzzle/2d: route `puzzle2dDefaultStylesFromElementsUiTokens`, `serializePuzzle2dVelloThemeJson`, and `PUZZLE_2D_STYLES_HEADLESS_FALLBACK` through the shared resolver/token map; make `serializeKindCatalogBundle` resolve handle/node/edge `color` (token ref -> hex) before WASM; `DEFAULT_HANDLE_KIND_CATALOG.color` -> token ref.
- puzzle/3d + infinite/world/r3f: drop local `probeCssComputed`/`cssColorForThree`/`MESH_STYLE_HEADLESS`/`WORLD_MESH_BORDER_HEADLESS`; use shared resolver; replace inline `#cbd5e1/#38bdf8/#64748b`, rubber band `0xf472b6`, grids `0xb8c4d0/0x6a7a8a` with token refs.
- cad: replace `SPATIAL_SCENE_COLOR_FALLBACK` table with token refs via shared resolver (upgrade to probe so `color-mix` resolves); keep `spatialSceneColors()` shape.
- ui/react: replace `getComputedColor`/`getComputedColorForGltf` with shared resolver; `GUMBALL_AXIS_COLORS`, plane handles `#94a3b8`, scale `#f8fafc` -> token refs.
- puzzle/5d: grip default `#94a3b8` -> token ref.

### 4. Migrate fixtures/catalogs to token references

- [reasoning/mindmap/wires/fixture/metabolism.wires.json](reasoning/mindmap/wires/fixture/metabolism.wires.json): relationship kinds owns->`var(--color-muted-foreground)`, is->`var(--color-secondary)`, references->`var(--color-tertiary)`, has->`var(--color-success)`; identity ramp -> grayscale token vars (`gray-800..gray`).
- [compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts) `SKETCHPAD_KIT_WIRES_KIND_CATALOGS`: map all 13 identity kinds + 4 relationship kinds to existing token vars (accents + grayscale ramp). Mapping chosen to keep the 4 relationship kinds visually distinct; identity kinds spread across the ramp.
- [reasoning/mindmap/react/index.tsx](reasoning/mindmap/react/index.tsx) `#fff` test data -> token ref.

Note: no new tokens are added (per chosen "resolve" mechanism); `references` (was purple) maps to `tertiary` since the palette has no purple. Distinctness for catalogs that exceed the palette's distinct hues is handled by reusing the grayscale ramp plus shape/pattern differentiation already present.

### 5. Tests (extend existing files only)

- Add resolver unit tests in [ui/styling](ui/styling) `script.ts`/styling vitest (headless `var(--color-*)` -> hex; fallbackKey path).
- Extend puzzle/2d vitest: kind-catalog token refs serialize to hex for WASM; defaults resolve to token hex.
- Extend puzzle/3d and wires play vitest: headless fallbacks equal token hex.

### 6. Regenerate + verify

- Run `bun ./script.ts generate` in `ui/styling`; rebuild puzzle/2d/rs (build.rs re-reads tokens.json).
- Run affected vitest suites and confirm runtime (boot wires play, check edge/node colors via `[DEBUG]` logs) before declaring done.

## Repo workflow

- Read `repo://goals`, then reopen/open a ticket via repo MCP for this UI styling-consistency task; keep any temp logs/scripts inside the ticket folder; close with summary + file list when done.

## Out of scope

- SVG asset fills (cursors, IFC/HDR geometry), `.repo/` archives, and storybook story-only tokens (mechanical/non-theme).
