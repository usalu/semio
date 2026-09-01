# 📐️ CAD End to End — Status

Start commit `bb06c41f73` · goal `🎯r2603` · ticket `2026/08/29/CAD-END-TO-END`

## Result

| Metric | Start | Now |
| --- | ---: | ---: |
| CAD `tsc` errors | 371 | **0** |
| CAD TS test files passing | 0 of 9 | **9 of 9** |
| CAD TS tests running | 1 | **321** |
| CAD TS tests passing | 0 | **321** |

Verified directly, twice:
`bun nx run @semio-tech/cad-js:test-long` → `Test Files 9 passed (9) · Tests 321 passed (321)`, exit 0.
`npx tsc -p "…/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit` → 0 errors inside CAD (3 remain repo-wide, all outside CAD — see Blocked).

Model-definition assets now actually load at runtime: **9** model definitions, **38** typologies, **97** actions, **60** interactions, **10** transformations.

## The two bugs that were actually stopping CAD

**1. Asset discovery was silently dead.** The 12 `import.meta.glob` patterns in `…/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts` were off by **five directory levels** — `../../../../` landed on `🪆️subsets`, but reaching the plugin root needs nine. Every glob returned `{}`, so every model definition, typology, action, interaction and transformation on disk was **invisible to the runtime**, and `registerModelDefinitionAssets()` registered nothing. Nothing errored; discovery just returned zero. Fixing the 12 globs took the suite from "8 of 9 files fail to load, 1 test runs" to 321 tests running.

**2. Every interaction effect silently no-opped.** The authoritative Rust schema (`🎬️interaction-spec/🦀️component.rs:234`) tags the effect enum `#[serde(tag = "mutation")]`, and all 49 shipped interaction assets emit `{"mutation": "assign", …}` — but the TypeScript `EffectSpec` union and all its consumers discriminated on `.operation`, which is **never present in the data**. So `assign` / `clear` / `append` / `kernel.query` / `action` / `interaction.call` all did nothing: interaction context stayed `{}`, commit guards failed, and interactions never reached `committed`. Renamed the discriminator to `mutation` (schema-first) and repaired the 13 consumer sites `tsc` then flagged. **45 tests went green at once.**
`"operation"` is still correct for **action steps** — the 88 action JSONs genuinely use it. Only the effect discriminator was wrong.

## Also fixed

3. **`@semio-tech/s-3d-js` was unresolvable repo-wide.** A concurrent session's module consolidation (`📦️index.ts` → `../../🟦️.ts`) left the package `exports` target pointing outside its own package directory. 19 `TS2307` + ~20 cascading implicit-`any`, and at runtime `PRIMITIVE_MODEL_ENTITY_KINDS` was `undefined`, so 8 of 9 suites could not even load. Restored a valid in-package `📦️index.ts` re-exporting their consolidated file (their layout kept intact).
4. **Import-cycle TDZ**: `MODEL_ENTITY_KINDS` was built at module scope from a circular import, so it was `undefined` at evaluation — made lazy.
5. **Three wrong relative paths** left by the same consolidation: two test fixtures (`ENOENT` on the concrete-forest play model) and the `flow_core` import in the 3d module.
6. **`core` was a value, not a namespace** — extensions wrote `type X = core.X`, which needs a namespace. Added a barrel and `export * as core`.
7. **Renderer tests never had a DOM**: `environmentMatchGlobs` does not exist in vitest 4, so it silently no-opped and the whole project ran in `node`. Replaced with a per-file `@vitest-environment jsdom` pragma.
8. **13 kernel geometry operations implemented** in `BrepjsWasmEngine.executeCommandDiff`, which previously had none: `solid.boolean{Union,Difference,Intersection}` (real OpenCascade booleans), `surface.{plane,loft,sweep1,sweep2,networkSrf}`, and `edit.{join,explode,chamfer,fillet,split,trim}`. Three new facade primitives (`cut`, `intersect`, `sweep`) added behind the existing opaque-handle interface — no brepjs type leaks out.
9. **`command.addSelection` ignored its `key` parameter**, which `chamfer.json`/`fillet.json` rely on to keep `firstCurve`/`secondCurve` selections apart.
10. **Construct kits authored for the 11 `aec.building.*` typologies** (9 action + 11 interaction assets), after confirming no transformation targets `aec.building` — it is only ever a transformation *source*, so those typologies were genuinely missing their constructors rather than being transformation-derived.
11. ~330 further type errors across kernel, actions, artifact, renderer, inferences and extensions — see the `📓️fix-*.md` reports in this folder.
12. **Retired mutation vocabularies replaced**: `🧬️mutations/💾️binary/📡️component.protocol.semio` and `📝️text/📖️component.grammar.semio` still named a 14-verb pre-migration vocabulary; regenerated against the current 20 `CadMutation::KINDS` (gate went from 13 stale kinds to 0).

## Blocked — by other sessions, not by CAD

- **CAD's Rust crate cannot be built or tested.** `semio-s-plugin-cad` depends on `semio-s-plugin-stdio`, which a concurrent session is mid-refactor on (~1060 modified paths, 329 compile errors, down from 398). **Zero errors originate in CAD's own Rust sources.** Verified repeatedly with `cargo build -p semio-s-plugin-cad --keep-going`; the only failing crate is `semio-s-plugin-stdio`. Not touched — editing 1060 in-flight files would collide destructively.
- **Live-app runtime proof not obtained.** The machine is saturated by other sessions (load average 38, 30–35 concurrent `cargo`/`rustc` holding the shared `target/` lock). The CAD dev server never bound port 6020; the dev tool's own budget guard killed the wasm build at 20 minutes with "Likely shared cargo target-dir lock contention from another concurrent session". All CAD evidence here is from the test suite and typechecker, **not** from the running app. This remains genuinely unverified.
- **`flow_core` wasm bindings not regenerated.** `flow_core` never exported `tessellate`/`dispose` via `#[wasm_bindgen]` — a real gap that would break CAD's brep tessellation WASM path at runtime. The Rust fix is in place (`#[cfg(target_arch = "wasm32")] mod wasm_bridge` in `🌊️flow/📐️brep-geometry/🦀️component.rs`, wrapping the existing `tessellate_geometry_json_for_wasm` / `dispose_geometry`), but `semio-framework-os-flow-core:wasm` is queued behind the same cargo lock, so `pkg/flow_core.{js,d.ts}` still lack the exports. These are the 2 remaining repo-wide `tsc` errors. The TS side already fails loudly and correctly ("rebuild flow/core wasm") rather than silently.
- The 3rd repo-wide `tsc` error (`…/🦑️repo/…/📦️index.ts:6083`) is in the repo library, untouched by this ticket.

## Independent validation

`mutate-cad-1` — 20 typed mutation kinds validated against a **separate Python oracle**. The oracle phase runs without cargo and **passed 41/41** (20 mutate + 20 inverse + 1 identity round-trip). No drift between the Rust enum, the oracle, the feature file and the registry. The `subject` and `parity` phases need the Rust build and are blocked as above.

## Known gap, not a regression

`ModelSpace` carries the hash primitives (`vertexHashesByModel`, `geometryHashesByModel`) that `AGENTS.md`'s linked-editing semantics require — "when a primitive is edited inside a model space, all primitives with the same hash are also edited", plus the warning when models can no longer be linked — but **nothing consumes them yet**. No test covers it. This is a missing feature rather than something broken, and deserves its own ticket.
