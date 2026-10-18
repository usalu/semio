# Result c6fin2 (census 6 packages P12, P17, P23, P52, P61, P66, P67, P68, P69)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo, no `bun repo` build (only `bun repo --help` as the baseline boot probe). `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c6fin2-tools/` (Rust: copies of the c5b tools, TS: `F/c6fin2-tools/ts/`, specs in `specs/`, snapshots in `orig/`). Scratch (outside the repo): `/Users/ueli/.cache/c6fin2/` (sandboxes, staged trees, a scratch `nm/node_modules` + `tsconfig.override.json` for `bun test`, copied from the pai cache plus a few bun-cache packages).

## 0. Verdict

**11 of 14 files split, 3 not split (reported).** All Rust children are at most 741 lines (one trait impl), all TS children at most 713 lines.

| tag | file | before | root after | children | largest child | status |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| pj (P17) | `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` | 2,215 | 177 | 9 | 421 | DONE |
| pjr (P17) | `…/🎒️pack/🔤️json/📥️retained/🧬️schema/🦀️.rs` | 1,368 | 35 | 4 | 478 | DONE |
| hud (P23) | `🌎️hub/🧪️tests/🔬️bin-unit/📇️directory/🔬️unit/🦀️.rs` | 2,096 | 54 | 8 `include!` fragments | 434 | DONE |
| p2h (P52) | `✏️s/…/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` | 3,070 | 85 | 10 | 458 | DONE |
| p3e (P52) | `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` | 3,016 | 305 | 8 | 741 (`🪟️artifact-editor-impl`, one `impl ArtifactEditor for Generation3dPlayApp`, cannot be cut) | DONE |
| p3v (P52) | `…/🧊️generation3d/…/👁️viewer/🦀️.rs` | 1,958 | 129 | 7 | 421 | DONE |
| p3s (P52) | `…/🧊️generation3d/…/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 1,114 | 123 | 3 | 416 | DONE |
| mft (P61) | `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | 2,311 | 42 | 8 | 713 (`📥️mutation-input-schema`: helpers and the 551-line reader are mutually recursive) | DONE |
| sps (P66) | `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` | 1,308 | 172 | 7 | 263 | DONE |
| tcf (P67) | `🌎️hub/🧪️tests/🏛️trusted-catalog-fixtures/🟦️.ts` | 1,074 | 151 | 3 | 379 | DONE |
| w3s (P69) | `…/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts` | 1,891 | 74 | 7 | 559 | DONE |
| tjc (P68) | `…/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` | 1,858 | 1,858 | 0 | | NOT SPLIT: one function `toolJobCoverageSelfTests` of 1,836 lines (shared locals, returns a sum of counts); the brief forbids rewriting a function over 1,000 lines |
| srs (P68) | `…/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` | 3,692 | 3,692 | 0 | | NOT SPLIT: one function `registerTests1` of 3,672 lines (shared destructured dependencies, 40+ `describe` blocks over one closure, one `describe` of 1,050 lines); needs a kit-factory redesign, and vitest is not available here to run it |
| jsi (P12) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` | 2,120 (2,115 at the last read; a peer was editing it) | 2,120 | 0 | | NOT SPLIT, see section 5 |

## 1. Rust method and oracles

- `modsplit.py` (path-module children, `pub use child::*`, widening of cross-child privates, impl re-opening where needed) for pj, pjr, p2h, p3e, p3v, p3s; `frag.py` (`include!` fragments, module identity unchanged) for hud. I fixed one tool defect in `rsitems.py`: a `type X<T> = Y<T, {CONST}>;` was cut at the const-generic brace (pj line 886).
- Impl re-opening (declared pre-edits, `expect_extra` `}` only): p2h one cut in `impl Generation2dRetainedMutationOwner` (before `accept`), p3s one cut in `impl Generation3dMountedTypedSnapshotOwner` (before `assign_json`).
- Visibility facts: `number` (pj) and `retained-owner-catalog` (p2h) use no name of their root, so they carry no `use super::*` (unused-import warning removed after the first oracle run); the hostile-lease items (`cfg(any(test, feature="testkit"))`, `pub`) sit in the same child as the other hostile items so no child is a `pub use` of only private items.
- Mount facts: the `#[path]` mounts, `use` blocks, `extern crate` lines, macros (`json!`), inline `testkit`/`native_codec` modules and the `#![allow]` attribute stay in the roots. `float_format` (pj) stays in the same child as its only user.

| Check (all run on the live tree, `F/c6fin2-tools/`) | Result |
| --- | --- |
| `modsplit.py lossless <tag>` | `LOSSLESS OK` for pj, pjr, p2h, p3e, p3v, p3s (missing 0; extras only docstrings, wiring, markers, declared `}`) |
| `chk.py live hud` | OK: 40 tests to 40, gone=0, extra(non-marker)=0 |
| `chkmod.py` / `vmod.py live` / `modclash.py` / `defscan.py` / `ambig.py live` | OK for all six module splits (0 cross-file private leftovers, 0 glob ambiguity candidates, rustfmt parses every file, path lengths below 240 bytes) |
| `oracle5.py <tag>` (patched resolution differential, original restored in one sandbox vs live tree, lib and `--test`) | identical error and warning multisets for pj (277/277), pjr (29/29 lib, 107/107 test), p2h (452/452, 567/567), p3e (837/837, 865/865), p3v (403/403, 415/415), p3s (109/109, 20/20); hud is an `include!` split (module identity unchanged) |
| `bun rgi.mjs` / `bun tax.ts` (repaired for `loadCatalogTaxonomy`) | 49 Rust folder names RGI, 49 of 49 resolve in the live taxonomy; the 25 TS folders also resolve |
| `bun census-own.ts` (canonical `readRustSourceClosureV1`) | inside the gate: pj 18 files, 3,523 lines, 231 KB (0.882); pjr 6 (lib) / 11 (test) files; hub route laws 13 files (test); p2h 11 / 12; p3e 12 / 14 files, test closure 253,483 bytes (0.967, the tightest ratio, was about 0.95); p3v 8 / 11; `semio-s-artifact-procedural-3d-io-binary` 12 / 15 |

Not run: the typeck-level `pos` oracle (union sandbox) and any cargo build. The patched oracle cannot see type-level privacy (E0616/E0624) when external crates are unresolved; the widening is conservative by name (`widen2.py`, 0 leftovers in `defscan.py`).

## 2. TS and JS method

- Splitter: `ts/gsplit.cjs` (statement-level split with import re-pointing of importers by TypeScript AST). Changes to the copy: shorthand-property references are tracked (a latent miss of imports), `export { local }` lists keep their import, `import("…")` type arguments of importers are re-pointed to the child, relative `import()` specifiers inside moved bodies are rebased (a miss found in sps: four dynamic imports), one gsplit cross-check `typeof import(parent)` fixed by hand.
- Lossless: `ts/lossless.cjs` for mft, sps, tcf, w3s: `lost=0`; added lines are only child docstrings (mft also the moved UiContract doc; the orphan `PluginClient` JSDoc that the tool dropped was re-inserted in the mft root; w3s adds the duplicated ambient `declare const` block and four `@webgpu/types` reference directives).
- `ts/newcheck.py`: 25 TS folders, 0 problems (folder shape, docstring emoji = folder emoji, taxonomy prefix, unique among siblings, longest path 205 bytes).
- Type check (`ts/tscfiles.py`, repo tsconfig, cached TypeScript 5.9.3): no TS2305/TS2304/TS2724 or unresolved relative module in any split file or rewritten importer; the only diagnostics are the environment ones (missing `node_modules`, `@types/node`, `@types/bun`, `@webgpu/types`, `vitest/importMeta`).

### TS importer, registry and reader changes (all edited with exact-count assertions)

- mft: 39 importers re-pointed to the children by gsplit (`F/c6fin2-tools/orig/mft-importers/` keeps the originals); `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` keeps its blanket `export * from` the manifest root and gains one `export * from` per child (the package barrel is the public surface); `🧪️tests/🧪️hostresolvedargs` `typeof import` re-pointed; readers re-pointed: `🎫️shell-holds` (reads the `🔘️definitions-action-ids-and-shell-brand` child), `🧬️schema-owner` (two import-edge assertions), `🖱️ui/🧪️tests/🎚️axes` unchanged (the generated-axes import stays in the root). Registries: `🔣️taxonomy.json` `generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile.sourceModulePaths` and `inputPatterns` gain the 8 children (byte-sorted; `closure-delta.cjs` shows all 8 are value-reachable from the frame worker), `🧪️test/📋️project.json` `test-schema` inputs gain `🛂️manifest/*/🟦️.ts`.
- sps: the identity-bearing code (`import.meta.dir`/`filename`, the router, the directory-wiring scripts) stays in the script root; constants moved to `🧮️component-constants`. Reader: `🏗️compiler/♻️capture/🧪️tests/🟦️.ts` now reads the module tree of the script plus the `🧾️catalog-artifact-io` child (that child is imported by siblings only; the tree walker does not follow sibling edges).
- tcf: three importers/entry points: `proveTrustedCatalogIdentityRolesFixture` and the two script classes stay in the root (their importers and the hub `📜️script.ts` lazy imports are untouched); two importers re-pointed by gsplit. Registry lists for the three new hub test folders: `🌎️hub/📦️packages/🦀️rust/📋️project.json` (3 brace lists), library `📦️packages/🟦️typescript/📋️project.json` (5 brace lists), `🧱️foundation-source` and `🧱️socket-grant-command-source` fixtures (brace list), `🧪️testing/📜️script-sources/🟦️.ts` (3 directory rows), `🔧️build-surfaces` fixture (3 file rows), `plugin-publication-source-ownership` fixture (the consumer row moved to the child that imports the two owners), `📓️print/🔮️oracles/🔣️.json` (`reachableFrom` row moved to the child that holds the dynamic `import("ajv/dist/2020.js")`).
- hud: `🧱️socket-grant-command-source` fixture and `🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts` point the two socket directory laws at the fragment `🔌️socket-delivery-visibility-and-public-projection` (the TS plan now builds `source` as `…/🔬️unit/<child>/🦀️.rs`, no empty-child case left).
- pj/pjr: `🔬️unit/📏️retirement/🟦️.ts` (line 1226) reads the `🎛️controlled-and-borrowed-bridge` child; `🏪️store/…/📨️emission/📦️owned/🧪️tests/🟦️.ts` (line 322) and `🎒️pack/🔤️json/🧪️tests/🧱️ownership/🟦️.ts` read the module tree (`readModuleTreeSource`, with `{crates:false}` for the store test). The needles of all edited readers were re-checked against the new trees with `needles.py`.
- w3s: no importer, no list entry (children are not tests).

### TS tests run here (scratch resolution via `ts/bt.sh`; many packages are missing in this checkout)

| Test | Result |
| --- | --- |
| `🛂️manifest/🧪️tests/**` and `🪟️view-context/🧪️tests/**` (9 files) | 119 pass |
| `🌎️hub/…/🏗️compiler/♻️capture/🧪️tests/🟦️.ts` (stdio script text reader) | 3 pass |
| `📚️library/🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts` | 7 pass |
| `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts` | 6 pass, 1 fails only on the missing `@types/node` diagnostics (the earlier unintended-comparison diagnostic from the plan change was fixed) |
| renderer tests of mft importers, `🧱️foundation-source`, `🔮️oracle-governance` | cannot run here (missing react, three, jsdom, d3-scale, process envelope); not claimed |

## 3. Crates the coordinator must compile

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests` for:

1. `semio-framework-pack-json` (also `--features testkit`, and its companion `semio-framework-pack-json-tests`)
2. `semio-framework-pack-json-retained-schema`
3. `semio-hub-directory-route-laws` (`--features testkit`; hud fragments)
4. `semio-s-artifact-procedural-2d-host`
5. `semio-s-artifact-procedural-3d-editor` (editor has a test closure at 0.967 of the byte limit)
6. `semio-s-artifact-procedural-3d-viewer`
7. `semio-s-artifact-procedural-3d-io-binary` (root `💾️binary/🦀️.rs` mounts the split snapshot module)

Dependents worth a `check` after the first six: the crates that depend on pack-json (`semio-framework-pack-json` is a foundation crate).

## 4. TS tests the coordinator should run (full `node_modules`)

- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/**`, `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts`, the renderer tests under `🧑‍🎨engine/🧪️tests/` that import the manifest (`♿️native-accessibility`, `🏭️registry-utilities`, `📋️renderer-types-and-forms`, `🔁️refresh-coalescing`, `🔬️wgpu-extension-dispatch`, `🗣️dialog-origin`, `🪟️spawned-program-session`, `🪪️window-action-panel`, `🎫️shell-holds`, the `🧱️elements` ones), plugin `🧵️retained-command/🧪️tests/🧬️schema-owner`.
- `📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts` (`testWgpuGeneratorOwnership`, `testBrowserInputClosure`: they `Bun.build` the frame worker and compare the loaded modules with the 8 new `sourceModulePaths` rows).
- `🧪️test/🧪️tests/🔮️oracle-governance/🟦️.ts` (the `reachableFrom` row of the trusted-catalog ajv import moved to the browser-actor child).
- Hub: `🧱️foundation-source`, `🧱️socket-grant-command-source`, `trusted-stdio-gis-bundle-check`, `native-catalog-checks`, `inference-fixtures`, and `bun test` of `🏛️trusted-catalog-fixtures` consumers; `📚️library/🧪️tests/📣️plugin-publication-source-ownership`.
- `🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts` and `…/📨️emission/📦️owned/🧪️tests/🟦️.ts` (readers of pj/pjr).

## 5. Open risks and the three files that are not split

- **jsi (P12) `📥️inference/🟨️.mjs`**: analysed, not changed. `bun repo --help` boots (run before and after all my edits, exit 0, `complete` stage) because the file is unchanged. A lossless split is blocked by the module's own revision and hot reload design, not by size: (1) `importRevision(import.meta.url, revision)` re-imports the file by URL query; static children keep their cached instance (I proved this with a toy reload: without evicting the child from `require.cache` the child stays stale, with eviction it reloads), so every child would have to be evicted and listed in `implementationRevision()`; (2) `TAXONOMY_PROJECTION`, `sourceAnalysisOwnerIdentityV1(import.meta.url, IMPLEMENTATION_REVISION)` and `registerSourceAnalysisV1` register analyses whose ids derive from the revision-bearing URL; a child with a constant URL would re-register the same id with a new `analyze` closure and throw `Ambiguous source analysis revision registration`; (3) eleven `let` bindings are assigned by `libraryBootstrap` and read by about 25 functions, so bootstrap and state must live in one module with live bindings; (4) 15 `import.meta` sites mix package resolution (safe), relative requires such as `../🚪️admission/🟦️.ts` (need an extra `../`) and identity (must stay in the parent); (5) `🧪️test/🟨️.mjs` hashes this one file for `libraryRevision`; (6) a peer is rewriting its imports (the file changed twice during this slice) and 61 files import it. The mechanical splitter was extended for `.mjs` (`ts/gsplit-mjs.cjs`, `ts/refmap.cjs` prints the full reference map). Recommended: a dedicated ticket that first makes children revision-aware (an explicit module list used by `implementationRevision()` and `importRevision()` eviction, an identity passed by the parent), then moves the pure groups (bun lock graph, rust token scanner, generator output inputs, native/go workspace, playground targets, import edges) and the bootstrap state; verify with the real graph discovery, not with the cache tests (only 17 of 79 of the jsi consumer tests pass here, for environment reasons).
- **tjc and srs (P68)**: single functions over 1,000 lines, reported as instructed.
- p3e keeps one 741-line child (single trait impl); mft keeps a 713-line child (mutually recursive input helpers and reader).
- The `ajv` production-debt row (`reachableFrom`) and the `wgpu-frame-worker` closure lists are lists maintained by hand; they were updated by reasoning (closure-delta, the dynamic import location) and cannot be re-proved without the full `node_modules`.
- Concurrent edits: the hub files `📜️script.ts` (stdio) and `🏛️trusted-catalog-fixtures/🟦️.ts` and the library import lines were being rewritten by a peer (library un-barreling) while I split them; I refreshed the snapshots right before each apply and the splitter refuses to write when its input changed. A later peer sweep over those two files must also cover their new child folders.
- The `.vscode/launch.json` and `build-owners` files needed no change (no crate, route or owner row changed). Nothing was committed or staged.
