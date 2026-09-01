# 🫀️ Fix `core` Namespace + Extensions

## Scope
`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📦️index.ts`, new `🫀️core.ts` beside it, the four
`✏️s/🔌️plugins/📐️cad/🧩️extensions/*/🟦️component.ts` files, and
`🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📦️index.ts`.

## Root cause
`core` was `export const core = { ...geometry, ...spatial, ...registry }` — a runtime value only.
Extensions used `type X = core.Y`, which needs `core` to also exist as a **type namespace**, hence
`TS2503: Cannot find namespace 'core'` (20 occurrences) plus knock-on `TS18046`/`TS2345` errors in
`aec-building-structure` and `spatial-shape`/`aec-building-energy` caused by `TypologyRef`/`row`
falling back to an error type once `core.*` type lookups failed.

## Fix
1. New file `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🫀️core.ts` — `export * from` the three
   engine components (geometry, spatial, registry). Naming follows the repo's existing "🫀️ = core"
   convention (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/`).
2. `📦️index.ts`: replaced the three `import * as` + `export const core = {...}` lines with
   `export * as core from "./🫀️core.ts";`. `export * as` produces a binding usable both as a value
   and as a type namespace, which is exactly what `import { core } from "@semio-tech/cad-js"` +
   `type X = core.Y` needs.
3. Not added to `tsconfig.json` `include` — `🫀️core.ts` is reached transitively via `📦️index.ts`'s
   import graph, so `tsc` already type-checks it.

## Collision audit (requested in ticket)
Extracted every top-level `export function/class/interface/type/const` and `export {..}` /
`export type {..}` name from `geometry`, `spatial`, and `registry`'s `🟦️component.ts` — **zero
name collisions** among the three (spatial and registry only *import* from geometry, they never
re-export its names, so `export *` has nothing ambiguous to drop). Cross-checked the full set of
symbols the four extensions destructure from `core` (`Model`, `applyModelDiff`,
`applyTransformation`, `computeStat`, `loadStatDefinition`, `loadTransformation`,
`objectsForStatCompute`, `bboxSizesFromPositions`, `cloneModelGeometryShell`,
`collectSolidRefsForObjects`, `collectVertexPositionsForObjects`, `collectFaceRefsForObjects`,
`qualifiedTransformationId`, `registerImportProfile`, `registerPropertyComputer`,
`registerStatComputer`, `registerTransformationApplier`, `resolvePrimitiveRefKind`, `solidRef`,
`transformationObjectId`, `typologyFromStepLayer`, `defaultModelDefinitionId`,
`derivePropertyValue`, `loadPropertyDefinition`) and every `core.X` type reference (`Model`,
`StatComputeContext`, `TransformationSpec`, `TypologyRef`, `ObjectRef`, `SpatialKernel`, `FaceRef`,
`PropertyComputeContext`, `SolidRef`) against the merged export set — **all present, none
missing**. Confirmed at runtime-shape level too (`tsc` now resolves every one with no `any`
fallback).

## Newly-surfaced errors (not pre-existing-masked, fixed)
Once `core.TypologyRef` resolved to its real branded type (`string & { __brand: "TypologyRef" }`)
instead of an error-type, 42 `TS2322: Type 'string' is not assignable to type 'TypologyRef'`
errors surfaced in the STEP-layer→typology lookup tables (`BUILDING_LAYER_TYPOLOGY`,
`STRUCTURE_LAYER_TYPOLOGY`, `BUILDING_TO_STRUCTURE_TYPOLOGY`, `ENERGY_LAYER_TYPOLOGY`) across
`aec-building`, `aec-building-structure`, `aec-building-energy`. These were always latent — the
namespace failure had degraded `TypologyRef` to `any`, silently hiding them. Fixed with the
established codebase idiom already used elsewhere in these same files for single values
(`"..." as TypologyRef`), applied per map entry. (Tried a single `as Readonly<Record<string,
TypologyRef>>` cast on the whole literal first — TS rejects it with TS2352 "neither type
sufficiently overlaps", a mapped-type comparability quirk, so per-entry casts are also the
*correct* fix here, not just the concise one.)

The `row is of type 'unknown'` (4×, `aec-building-structure`) and `Argument of type '{}' is not
assignable to parameter of type 'string'` (2×, `spatial-shape` + `aec-building-energy`) errors
listed in the ticket as secondary items were themselves downstream of the same `core` namespace
failure and disappeared once the barrel fix landed — no separate change was needed for those.

## `🧊️3d` module — flow_core `tessellate`/`dispose` (left unfixed, reported)
`ensureBrepWasmLoaded()` in `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📦️index.ts:338`
destructures `{ tessellate, dispose }` from the dynamically-imported
`.../🌊️flow/🫀️core/pkg/flow_core.js`. Investigated whether the generated bindings are simply
stale:

- `pkg/flow_core.d.ts` only declares `DagSession`, `DagSnapshotVcs`, `KernelHost`, `initSync`,
  default init — no `tessellate`, `dispose`, or any brep-related free function.
- Traced the wasm crate's actual root (`[lib] path = "📦️glue.rs"` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`). `glue.rs` does
  `pub use brep_geometry::{dispose_geometry, export_solid_json, import_solid_json,
  retain_geometry_handles, tessellate_geometry};` — a plain `pub use`, not a `#[wasm_bindgen]`
  export.
- `grep -rn "wasm_bindgen"` across the entire `🌊️flow` module tree (not just the wasm bridge file)
  hits **zero** Rust source files — only the already-generated `pkg/*.js`/`*.d.ts` mention it.
  `brep-geometry/🦀️component.rs`'s `tessellate_geometry`/`dispose_geometry`/
  `tessellate_geometry_json_for_wasm` (the latter looks purpose-built as the wasm-facing JSON
  variant, returning `String`) carry no `#[wasm_bindgen]` attribute at all.

**Finding: this is not stale/regenerable bindings — rebuilding today's Rust source would produce
the exact same `.d.ts`.** The Rust side never wires `tessellate_geometry_json_for_wasm`/
`dispose_geometry` to `#[wasm_bindgen]`, so no `tessellate`/`dispose` (or any name) can appear in
`flow_core`'s generated bindings until someone adds that annotation on the Rust side. That's a
Rust change to a module outside this TS-only slice and outside this ticket's stated ownership, so
per "if an error needs a decision outside your slice, leave it and report it" — **left both
`TS2339` errors unfixed**. Minimal correct action here is no code change to `📦️index.ts`, since
any type-level patch (widening, `any`, `as unknown as`) is explicitly forbidden and would paper
over a real missing capability rather than fix it.

## Before / after (this slice's files only)
| File | baseline | after |
|---|---|---|
| `cad 📦️index.ts` | 0 | 0 |
| `🏛️aec-building-structure/🟦️component.ts` | 10 | 0 |
| `🏢️aec-building/🟦️component.ts` | 1 | 0 |
| `📐️spatial-shape/🟦️component.ts` | 8 | 0 |
| `🔥️aec-building-energy/🟦️component.ts` | 8 | 0 |
| `🧊️3d 📦️index.ts` | 2 | 2 (reported above, not fixed) |
| **slice total** | **29** | **2** |

Repo-wide (`grep "error TS" | wc -l`): baseline 371 → now 344 (−27, matching this slice's fixes
exactly). Confirmed via file-list diff that **no file outside baseline's error set gained new
errors** — no regressions introduced elsewhere. Repo-wide total is a moving target since five
sibling agents are editing concurrently in the same tree; the 344 figure is a snapshot, not
attributable solely to this slice.

## Confirmation: extensions resolve every `core.*` symbol
All four extensions' destructured values and `core.X` type references were symbol-matched against
the merged geometry+spatial+registry export set before editing (see collision audit above), and
`tsc` now reports zero errors in all four `🟦️component.ts` files and in `cad`'s own `📦️index.ts` —
confirmed by actually running the typecheck, not assumed.

## Files touched
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🫀️core.ts` (new)
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📦️index.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🟦️component.ts`
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📦️index.ts` — investigated only, no edit.
