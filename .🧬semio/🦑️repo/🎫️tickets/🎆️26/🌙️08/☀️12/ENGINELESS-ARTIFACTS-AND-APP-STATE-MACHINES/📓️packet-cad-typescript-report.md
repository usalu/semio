# CAD TypeScript engine dissolution — final 3 files (#2553)

No prior `📓️packet-cad-typescript-report.md` existed. The prior agent's own report file was never written (only inferred from `📓️results.md` scoreboard line `📕️norm ×2, 📐️cad, 💡️reasoning remain` and the doctrine notes in `📌️important.md`). What the prior agent had already landed, confirmed by directory inspection before starting:

- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚙️engine/` — `🎬️actions`, `🎰️stately`, `🏃️runtime`, `📄️artifact`, `📔️registry`, `📺️renderer`, `🕹️interaction`, `🧬️typology` (app engine, D5).
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/` — `📐️geometry`, `🗺️spatial`, `🧱️brepjs` (shared module engine, D6a).
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️component.ts` already existed (16 lines: `CadBounds`/`CadInference`).

Only 3 files remained under the artifact `⚙️engine`: `🟦️index.ts` (barrel), `🔍️query/🟦️component.ts` (1689 lines), `🧪️tests/🟦️component.ts` (3025 lines).

## Destinations

| File | Destination | Classification |
|---|---|---|
| `🔍️query/🟦️component.ts` | `🧬️schema/💡️inferences/🟦️component.ts` (merged, appended under `// #region 🔍️ConstructQueryLanguage`) | D4 — derived compute over a `Model` snapshot, no mutable state |
| `🟦️index.ts` | kept in place, exports repointed to the 5 new homes | barrel — 2 real external consumers found (`📺️renderer/component.tsx`, `📜️script.ts`), so deletion was not zero-risk; **see "index.ts had to go" below — this changed mid-task** |
| `🧪️tests/🟦️component.ts` | split by describe-block across 4 destination files | see split table below |

### `🔍️query` → `💡️inferences` merge

Byte-identical append (via `cat`, not retyped) after the existing `CadBounds`/`CadInference` interfaces, wrapped in `// #region 🔍️ConstructQueryLanguage` / `// #endregion`. All of `🔍️query`'s original relative imports (`../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/{📐️geometry,🗺️spatial}` and `../../../../../../../../🎛️apps/📐️cad/⚙️engine/🎬️actions`) resolve **unchanged** — `⚙️engine/🔍️query/` and `🧬️schema/💡️inferences/` sit at the same depth under `✳️any/`, so no path rewriting was needed, only a directory-level cross-check.

### `🧪️tests/component.ts` split (3025 lines, verified line-exact)

Baseline (grep on the original file before any edits): **127 `it(` + 4 `it.each` + 500 `expect(` + 25 `describe(`**.

Boundaries were first computed with a "next describe start − 2" heuristic (assuming a blank separator line between blocks) — **this was wrong twice** (two block pairs are adjacent with zero blank line: `action and interaction registries`→`model diff` at 1609/1610, and `selection filter`→`interaction box` at 1689/1690). Caught by grepping `^  });$` (exactly 25 hits, one per `describe`) and cross-checking that the 25 ranges tile lines 40–3021 with zero overlap and zero gap beyond the 22 real blank separators. All 4 destination files' final `it`/`it.each`/`expect`/`describe` counts were re-grepped after the split and sum to **exactly 127/4/500/25** — no assertion lost, none duplicated.

| Destination | describe blocks | it | it.each | expect |
|---|---|---|---|---|
| `🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` | vec, model definition catalogs, model space and hashing, transformations, attribute validation, edge and solid geometry, expr, model json, metadata, selection filter | 44 | 0 | 170 |
| `🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts` | model commit mesh, model diff | 5 | 0 | 15 |
| `🎛️apps/📐️cad/⚙️engine/🎬️actions/🟦️component.ts` | box display committed | 1 | 0 | 3 |
| `🎛️apps/📐️cad/⚙️engine/📄️artifact/🟦️component.ts` | interactions, action and interaction registries, interaction box, interaction length entry, stateEngine option, measure distance, measure area, document history, measure distance history, interaction session undo redo, undo routing, interaction e2e fixtures | 77 | 4 | 312 |
| **total** | 25 | **127** | **4** | **500** |

Classification rule used: each `describe` was assigned to whichever of the 5 original import groups (`../📐️geometry`, `../🧬️typology`, `../🗺️spatial`, `../🎬️actions`, `../📄️artifact`) it depended on most heavily, verified by grepping the block body against each domain's exported-symbol list, not by guessing from the title. Two blocks were reclassified after grep contradicted the initial title-based guess: `selection filter` (title suggests `actions`, but its only calls — `selectionEventMatches`, `expandSelectionTargetsForAccept` — are geometry exports) and `stateEngine option`/`box display committed` (both call `resolveDisplay`/`pureTsStateEngineProvider` from `actions`, but the former also drives a full `createInteractionRuntime` session so it went to `artifact`, the latter is a pure `resolveDisplay()` call with no runtime so it stayed in `actions`). `interactions` and `action and interaction registries` are the two largest blocks (225 and 753 lines) and are irreducibly cross-cutting (typology + actions + geometry all exercised through `createInteractionRuntime`); both went to `artifact` since that's the file `InteractionRuntime`/`DocumentHistory` actually live in.

Each destination's `import.meta.vitest` block follows the existing repo convention (seen already in `🔍️query`, `🎰️stately`, `🏃️runtime`, `🧱️brepjs`): a dynamic `await import(...)` of the runtime (`bootstrapCadModules`) and brepjs test kernel, gated behind `import.meta.vitest`, followed by an `if (import.meta.vitest) { … }` block with the `describe`s. New static imports added per destination were computed as the *precise* symbol difference between what each assigned block's body actually references and what that destination file already imports for its own implementation (not a blanket re-import of every original domain symbol) — e.g. `geometry` only gained `SpatialKernel, SpatialPreviewKernel, applyModelDiff` from `spatial` and 5 symbols from `actions`, not all ~70 that were in the original combined import line. The two JSON fixture string constants (`CAD_E2E_ROUTES_MODEL_SPACE_JSON`, plus loom/building for `artifact`) were copied byte-for-byte via `sed` extraction, never retyped, to eliminate transcription risk in ~3–10KB literal blobs.

## `🟦️index.ts` had to go — mid-task correction

Original plan: keep the barrel, repoint its 5 `export *` lines to the new homes (done first). Then, per the definition-of-done (`find … -name ⚙️engine -type d` must return 0 under `🗿️artifacts`), the *entire* `⚙️engine` directory — including `🟦️index.ts` itself — had to stop existing. There is no taxonomy-legal home for a bare barrel file outside an `⚙️engine`/`⚙️engine`-shaped leaf that I could place it in without risking a `subsetChildDirs` violation I can't verify (can't edit `🔣️taxonomy.json`). So the barrel was deleted outright and its 2 real external consumers were fixed to import directly from the underlying files:

- **`📺️renderer/🟦️component.tsx`** (fixed): its ~100-symbol barrel import (lines 102–203) was classified symbol-by-symbol against the 5 destination files' actual `export` statements (regex-verified, not guessed) and split into 5 direct imports (`📔️registry` for the `kernel-3d-js` re-exports `emptyMeshTransfer`/`kernelGeometry`/`solidRef`/`FaceGroup`/`FaceInfo`/`MeshTransfer`/`Vec3`, `📐️geometry` for 50 symbols, `🧬️typology` for 2, `🗺️spatial` for 6, `🎬️actions` for 21, `📄️artifact` for 13). Verified zero symbol-name collisions across the 5 new import statements. One symbol (`ShellRecord`) was dropped from the new imports entirely — turned out the file already independently declares `type ShellRecord = kernelGeometry.ShellRecord;` in its own body (line ~213), so the barrel import of it was already dead/shadowing before I touched anything. Also fixed the file's separate direct `🧱️brepjs` import (line 220), which pointed at the old artifacts-engine path — this one was already broken by the prior agent's earlier move, not by me.
- **`📜️script.ts`** (**not fixed — forbidden to edit**): line 34 (`defaultModelDefinitionId` dynamic import from the barrel) is now broken by my directory deletion. Lines 33 and 35 (`bootstrapCadModules` from the old `🏃️runtime` path, `buildSpatialStatelyMachineCatalogView` from the old `🎰️stately` path) were **already** broken before I started — the prior agent had already relocated both to `🎛️apps/📐️cad/⚙️engine/` without updating this consumer. `GenerateScript.run()` was therefore already 2/3 non-functional; my change completes the breakage of the third dynamic import. The dependency-boundary lint at line 13 (`dependencyBoundaryBreachesForBundleDir(repoRoot, ".../⚙️engine")`) now points at a directory that doesn't exist — likely degrades to "no breaches found" rather than crashing, but unverified since I can't run it without editing the file that calls it. **Flagging for the ticket owner to fix `script.ts` directly** since I'm barred from touching it.

## Other files fixed (not `📜️script.ts`, in scope)

- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `DOMAIN_FILES` updated: dropped the deleted barrel and `🔍️query` entries, added the 4 new `import.meta.vitest`-bearing files (`geometry`, `spatial`, `actions`, `artifact`) plus `💡️inferences`; doc comment updated from "6 folded domain files" to "9".
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json` — `include` list updated the same way (this is also the closest thing to a typecheck target this plugin has — see below).
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📋️project.json` — `namedInputs.default` glob updated from the deleted `🗿️artifacts/…/⚙️engine/**/*.ts` to `🎛️apps/📐️cad/⚙️engine/**/*.ts` + `🔨️modules/🌐️spatial-kernel/⚙️engine/**/*.ts` + the `💡️inferences` file, so nx cache invalidation still tracks the right sources.

## Definition of done

```
find ✏️s/🔌️plugins/📐️cad -path "*🗿️artifacts*" -name "⚙️engine" -type d
```
→ **0 results.** Confirmed.

## Typecheck

No `typecheck` nx target exists for `@semio-tech/cad-js` (`📋️project.json` only defines `test`/`test-quick`/`test-long`/`test-exhaustive`/`generate`/`fixture`, all of which shell out to `bun ./📜️script.ts test…` → vitest). `tsconfig.json`'s `include` list is the closest equivalent, so I ran `tsc --noEmit -p tsconfig.json` directly against it as a best-effort check (not a registered target, discovered not assumed).

**Result: 634 errors.** Root-caused every distinct category before accepting them as pre-existing:

1. **18 errors** are literally `Cannot find module '@semio-tech/kernel-3d-js'` — the ticket's documented known-broken dependency (no package.json entry, no node_modules symlink, tracked in `26/08/06/CAD-PLUGIN-RESIDUAL-MOP-UP-TS-MODULES-EXTENSIONS`). Per instruction, left alone.
2. The large majority of the remaining errors are **downstream cascades of #1** — `Cannot find name 'AnchorRecord'` etc. because the type that would have defined it never resolved.
3. A smaller cluster of **genuine pre-existing type mismatches independent of kernel-3d-js**: `TypologyRef` branded-type vs. plain `string` literals (in both original production code and the verbatim-moved test code), `ExprEnv` missing a required `preview` field at several call sites, `InteractionRegistry`/`ActionRegistry` missing a `.register()` method the code calls, and chevrotain `CstElement`/`IToken` mismatches inside the moved query code. I checked every error whose line number fell inside a region I actually wrote or edited (the 4 new `Tests` regions, the repointed imports) against the original file content — all of them trace to **code copied byte-for-byte from the original `🔍️query`/`🧪️tests` files**, not to anything I changed. None are new regressions from the split.
4. Two **newly-discovered pre-existing bugs**, unrelated to my 3 files, inside `📺️renderer/🟦️component.tsx`'s own untouched `import.meta.vitest` block (lines ~6709, ~7155): a dynamic `import("../📐️brepjs/…")` (wrong emoji, should be `🧱️brepjs`) and a dynamic `import("../🟦️index.ts")` pointing at a file that has never existed at that path (one directory too shallow). Left untouched — out of this ticket's scope, flagging for a follow-up.
5. A handful of `Cannot find module` errors in `📦️index.ts` (the *package*-level public barrel, a different file from the *artifact-engine* barrel I dissolved) reference `🧬️schema`/`🚪️io`/`🪓️decomposer` paths under `artifacts/📐️cad` directly — unrelated pre-existing gaps from a different decomposition effort, not touched.

I did not run the `test`/`test-quick` nx targets themselves — vitest's `includeSource` in-source-test mechanism requires the same module graph to resolve, and it will hit the identical `kernel-3d-js` wall before a single assertion executes, so a run would not have produced signal beyond what `tsc` already showed. Stated honestly rather than claimed as a pass.

## Files touched

- Edited: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` (appended Tests region, +608 lines)
- Edited: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts` (appended Tests region, +85 lines)
- Edited: `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚙️engine/🎬️actions/🟦️component.ts` (appended Tests region, +30 lines)
- Edited: `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚙️engine/📄️artifact/🟦️component.ts` (appended Tests region, +2312 lines)
- Edited: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️component.ts` (merged `🔍️query`, 16 → 1708 lines)
- Edited: `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚙️engine/📺️renderer/🟦️component.tsx` (barrel import split into 5 direct imports + one direct `🧱️brepjs` path fix)
- Edited: `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts`, `tsconfig.json`, `📋️project.json`
- Deleted: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (entire directory: `🟦️index.ts`, `🔍️query/`, `🧪️tests/`)
- Not touched (forbidden): `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts` (lines 13, 33, 34, 35 now reference dead paths — 33/35 pre-existing, 13/34 newly broken by the required directory deletion)
- Not touched (out of scope, discovered in passing): `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (stale doc-comment reference to a `.rs` file, not code, and forbidden anyway); `📺️renderer/🟦️component.tsx` lines ~6709/~7155 (two pre-existing bugs in its own test block, unrelated to the 3 files in scope)

## Deviations from the ticket brief

1. Barrel ended up deleted, not kept-with-repointed-exports, once the "no `⚙️engine` dir may exist" requirement was reconciled against "barrel physically lives inside an `⚙️engine` dir." The ticket explicitly offered this as a valid option ("delete the barrel if nothing needs it — check for external consumers first"); I checked, found 2 consumers, fixed the one I'm allowed to touch, and documented the one I'm not.
2. `script.ts` (cad plugin) now has one more dead reference than before (line 34) as an unavoidable consequence of point 1, given I cannot edit that file.
