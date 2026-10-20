# Result exports-stage: staged manifest batch (export keys, dependency declarations) behind the HARD FREEZE

Executor `exports-stage`, 2026-10-20 01:30 to 02:45. C = this ticket folder. **Nothing frozen was touched**: no `package.json`, no `bun.lock`, nothing under `📚️library/`, no root `📜️script.ts`, no `🔣️taxonomy.json`, no glue file. No sub-agent, no modifying git command, no cargo, no dependency sync/lock/show. All output is in `C/exports-stage-tools/` (kept inputs) and `C/🗑️generated/exports-stage` (deleted at the end).

## 0. Verdict

| item | result |
| --- | ---: |
| batch file | `C/exports-stage-tools/batch.json` (634 entries; 506 apply by default) |
| anchored apply | `C/exports-stage-tools/apply.ts` (`check` default, `apply --freeze-lifted`) |
| manifests touched by the default apply | 34 |
| new export keys | **408** in 26 packages (framework 154, repo-lib 134, framework-os 25, value-ts 19, framework-plugin 12, s-fem-engine-ts 10, framework-actor 8, rest 43) |
| export keys removed | **20** (19 moved off misplaced glue + the dead `nx-bootstrap-tools ./script`) |
| glue files created / extended / deleted | 379 / 21 / 19 (glue is source, not manifest; staged in the same batch, written by `apply.ts`) |
| importer rewrites (moved keys only) | 25 specifiers in 23 files |
| dependency declarations applied by default | 81 = 27 third-party + 34 `@types/*` + 3 requested workspace + 17 tst6 |
| dependency declarations optional / blocked | 119 hygiene + 1 physical (opt-in `--include=`), 5 refused for cycles |
| blocked export entries | 3 (below) |
| `no-cross-package-relative` | **1,062 violation edges now, 22 after batch + importer rewrite** (cruise-style 1,543 to 25) |

Everything below was measured on the live tree at 02:25; peers keep editing, so `apply.ts` re-checks baselines (section 5).

## 1. Method (why not the earlier JSON lists)

The five input lists (lay13, lay14, lay15, tax2b, glue) are 4 to 9 hours old, and tax2b/lay15 rewrote hundreds of importers since. So the batch is **recomputed from the live tree** and the inputs are used as provenance and as a cross-check:

1. `scan.ts`: 16,322 source files, 39,921 relative edges, 12,720 bare workspace imports, 7,396 bare third-party imports.
2. `ncpr-now.ts`: each edge evaluated with the **exact live policy** (`policy-dump.cjs` dumps the dependency-cruiser config; the 303 `no-cross-package-relative-*` rules become 247 owner selectors). Calibration against the real cruise (lay13 `cruise-area.sh`) on hub (4 of 4 pairs identical) and on course + print + mit-bestand (766 modules, 8 of 8 identical): 12 edges, 0 differences.
3. `moves.ts`, `groups.ts`, `build-candidates.ts`: group edges by (owner, target), drop edges that an existing entry already exposes (deep-origin identity per imported name, the `ncpr2.ts` logic; repo-lib barrels do not count), derive key, value and glue exactly like the existing keys (`./hash`, `./modules/...json`, repo-lib narrow keys equal lay15's 20 of 21), check the alias edge against every other rule, so the batch also says which rules still fire.
4. `deps.ts`, `health.ts`, `import-names.ts`, `missing-keys.ts`: dependency gaps, dead export targets, names every bare import asks for vs what the entry exports, bare imports of keys that do not exist.
5. `assemble.ts`, `validate.ts`, `simulate.ts`, `reconcile.ts`, `stats.ts`; `stage.ts` runs the whole chain (about 2.5 minutes, `--refresh` drops the caches).

## 2. Batch content

### 2.1 Export keys (tier `export`, 393 apply + 3 blocked; tier `move`, 31)

Every entry: package.json path, field, key, value, justification (importer files and edges, target file, imported names, which input proposed it, which other rules the alias still trips). Examples are in `batch.json`; the first is `@semio-tech/assets ./builder/publication` (1 edge).

- **Naming** follows the package's existing convention (framework strips the leading `🔨️modules/`, JSON documents get `.json` and a `🔣️json/<stem>` wrapper, `🧪️tests` becomes `🧪️testing`, repo-lib uses the narrow domain keys of lay15 so zero-touch closure files never go through a barrel). The earlier proposals differ on `.json` suffixes and `modules` segments for 80+ keys; the live convention won, the old name is listed in `justification.proposedByInputs`.
- **No alias duplicates**: validation compares every key of every package after the overlay: 0 introduced (3 pre-existing duplicates elsewhere are untouched).
- **Moves (19 misplaced glue files)**: framework carried glue that re-exports modules of other owners (`@semio-tech/framework/os/modules/...`, `.../tool-run/testing/fixtures`, `.../actor/wire-turn`). The batch removes the old key, deletes the old glue, adds (or reuses) the key in the owning package, extends the destination glue with the names the old glue exported, and rewrites the 25 importer specifiers. `apply.ts` refuses to remove a key that anything else still imports (`git grep`). Three framework-os `store/*/schema` entries are not pure glue (renamed exports plus bare re-exports), so they stay normal importers and get ordinary schema-ts keys.
- **Removal**: `nx-bootstrap-tools ./script` points at a deleted file. It is removed and `.` is added (the module body is `🟦️.ts`). The last importer is the build-mode test that **tsc-stage patch 8** re-points; `apply.ts` refuses until that patch is in (apply tsc-stage first).
- **Blocked (3)**: `energy-js ./extensions/epw/testing/unit` (the alias glue itself violates `plugin-no-extension-or-artifact-energy`), `framework ./products/repo/modules/dashboard/testing/coverage/oracle` (owner is the framework package but the file is repo-product code: `framework-no-products`; the repo product needs its own owner), `repo-lib ./build/script` (target is one of the 43 `📜️script.ts` that tax-stage restructures; re-stage afterwards).

### 2.2 Dependencies (field `dependencies`/`devDependencies`)

| tier | entries | default | what |
| --- | ---: | --- | --- |
| requested | 3 | yes | hub: `@semio-tech/framework-renderer-wgpu`, `@semio-tech/puzzle-js`; mit-bestand: `@semio-tech/print` (tax2b). Both roots list the packages as workspace members, so they resolve today; this is declaration only |
| third-party | 28 | yes | physically unresolvable from the importer's binding manifest (isolated linker: the nearest ancestor with a `node_modules`), declared elsewhere with this version. Root `package.json` 22: print d3-shape/-interpolate/-time/-chord/-contour/-hexbin/-delaunay/-format/-time-format/-sankey/-random/-regression/-quadtree, dagre, suncalc, fft.js, `@asamuzakjp/css-color`, unified, remark-gfm/-parse/-rehype, rehype-stringify, semver, `@vscode/test-cli`, `@vscode/vsce`; `✏️s/package.json`: dagre; framework package: d3-shape; others for the print viz-inference subtree |
| types | 34 | yes | `@types/*` for untyped packages the importer cannot get types for: d3-path, d3-polygon, d3-geo (3 manifests), d3-force (mit-bestand), `@types/reveal.js` (root, mit-bestand) and the types of the packages above |
| hygiene | 119 (+4 refused) | no (`--include=hygiene`) | workspace packages a non-root binding manifest does not declare although they resolve through auto-linked members (✏️s 65, hub 14, nine small plugin/package manifests). The 4 refused would close a cycle: framework to value-rs/framework-os/renderer-wgpu, nx-bootstrap-tools to repo-lib |
| tst6 | 19 (+1 refused) | yes | `result-tst6.md` / `tst6-tools/gaps-*.json`, declared at the package that owns the importer and, where the importer sits outside every package directory, at the root binding as well (details in 2.3) |
| physical | 1 | no (`--include=physical`) | `✏️s/package.json` to `@semio-tech/puzzle-wasm` (a hub package; only the ✏️s tsconfig path resolves it) |

Not produced: `@types/lines-and-columns` (no unresolved importer left), `@semio-tech/nx-bootstrap-tools` (it was the dead `./script` key). Versions come from the declaring manifest, or `bun.lock`; `bun repo dependencies lock` must run after the apply.

### 2.3 tst6 gaps (added on the coordinator's request, 02:50)

Source: `result-tst6.md`, `tst6-tools/gaps-unresolved.json` (19,720 files scanned), `gaps-undeclared.json` (311 rows: 50 `undeclared-unresolved`, 261 `undeclared-resolves`). `tst6-gaps.ts` diffs them against the staged entries; `scan.ts` now also reads `createRequire(import.meta.url)("pkg")`, which hid tar-stream, cross-spawn, safe-regex, lru-cache, `@iarna/toml`, scheduler from the first scan.

| tst6 item | batch entries |
| --- | --- |
| root d3-*, dagre, fft.js, suncalc, remark-*, unified, rehype-stringify, `@asamuzakjp/css-color`, `@vscode/*` | already staged (tier third-party / types), see 2.2. `@asamuzakjp/css-color` is pinned to the 3.2.0 that bun.lock resolves (jsdom/cssstyle); npm latest is 7.1.7 and would change the install, not pinned. `@vscode/test-cli` ^0.0.10 and `@vscode/vsce` ^3.5.0 are the pins the vscode package already declares (npm registry reachable, read-only, nothing downloaded) |
| `@bytecodealliance/jco-transpile` | 0.20.0 at the root and at framework-plugin (npm: latest 0.20.0, which is what the declared jco 1.39.0 depends on; already in bun.lock). tst6 wrote `jco-transpile/component`: same package |
| wgpu to `@semio-tech/plugin-registry` | entry exists but **refused**: `plugin-registry` already depends on `framework-renderer-wgpu`, the declaration closes a cycle. 12 wgpu source files import the registry; that cycle needs an owner decision (move the imported registry entries or invert the dependency) |
| print inferences to `d3-regression` | `devDependencies` 1.3.10 in the print viz-inference manifest (the probes sit inside its package directory) |
| teaching to `tar-stream` | `🎓️teaching/package.json` 2.2.0 (the version already declared in the repo; bun.lock also holds 3.2.2, npm latest) |
| emulation-only packages | cross-spawn 7.0.6 at `@semio-tech/process` and the root; lru-cache 11.5.3 at the framework package and the root; semver 7.8.5 at framework-kernel, repo-lib and the root; safe-regex 2.1.1 at repo-lib and the root. Versions are the bun.lock entries, equal to npm latest except dotenv (lock 16.4.7, npm 18.0.8) |
| also from the tst6 rows | `@iarna/toml` 2.2.5 (repo-lib, root, nx-bootstrap-tools), `iconv-lite` 0.7.3 (nx-bootstrap-tools), `scheduler` 0.28.0 (draw package) |
| not staged | `color` (tst6 names it for the root, but no source file imports it: the earlier tst2 note says the same; nothing to declare), `retry`/`toml`/`probe`/`x`/`succeeds`/`exit`/`now`/`poll`/`external`/`dep`/`vscode` (strings, WASI import names or the `vscode` host module, not packages), `@semio/process` (string in a fixture), the 261 `undeclared-resolves` rows (vitest, react, sharp, fast-json-patch, ... resolve through ancestor bindings; hygiene, not staged) |

Root declarations exist because the importing files sit outside every package directory (isolated linker: the nearest binding is the root); the owner-package declarations keep the project graph honest.

## 3. Input reconciliation

| input | entries | in batch | already exposed by an existing entry | glue extension only | handled by a move | stale (no current violation) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| lay13 `exports-needed.json` | 374 targets | 291 | 17 | 10 | 19 | 37 |
| lay14 `exports-needed.json` | 25 | 9 | 2 | 0 | 0 | 14 |
| lay15 `narrow-keys.json` | 21 | 20 | 0 | 0 | 0 | 1 (`./artifacts/root`) |
| tax2b queue (`result-tax2b.md` section 3) | 27 targets, 35 edges | 24 | 1 | 0 | 0 | 2 |
| glue 3.1 (7 dead entries), 3.2 (`config/schema/mutations`) | 8 | 0 | | | | all done by the deps owner / no importer left |
| tsc-stage `./script` | 1 | removal (needs tsc-stage patch 8) | | | | |

Plus 97 groups the inputs never listed (found by the fresh scan). lay13 `notExportable` (315): 15 query imports (14 still violating), 250 "reverted" (100 still violating, all now in the batch), 24 frozen (24), 22 "no import node" and 4 "package unknown" (none left). Not manifest-fixable, 15 edges: `?raw`/`?url` reads of a source or asset file (OS renderer wgpu boot sources, ui-react i18n, course contrast CSS, puzzle `.dsl.semio` stories, one logo SVG). A package export cannot point outside its directory; the file must move into a package or the reader must use an in-package copy.

## 4. Expected reduction of `no-cross-package-relative`

`simulate.ts` overlays the batch on the live manifests (post-batch authored targets) and classifies every current edge. The manifests alone do not change the cruise count: relative importers must still be re-pointed (`ncpr2-edges.ts` writes the cruise-like edge file for `lay15-tools/ncpr2.ts ... apply`, the 25 move rewrites are in the batch).

| class | edges |
| --- | ---: |
| current violation edges (unique importer/target pairs) | 1,062 |
| enabled by this batch (importer can use the new key or the glue) | 739 |
| already enabled by an existing entry (rewrite pending) | 282 |
| removed with the deleted misplaced glue | 19 |
| not manifest-fixable: query imports | 15 |
| blocked entries (3 keys) | 7 |
| **left after batch + rewrite** | **22** (cruise-style 25, from 1,543) |
| enabled edges that still violate when pointed at the entry file | 0 |

Per area, now / existing entry / batch / moved / query / blocked: repo 370/181/187/0/0/2, fwmodules 327/26/301/0/0/0, teaching 71/33/37/0/1/0, root script 68/0/65/0/0/3, splug4 50/0/50, osrenderer 48/21/23/0/4/0, fwrest 30/1/7/19/3/0, osrest 29/0/29, play 20/13/7, splug1 16/0/14/0/0/2, smod 14/0/7/0/7/0, other 11/6/5, hub 4/0/4, splug3 2, mit 1/1/0, splug2 1.

Honest limits: (a) the count in the table is "can be rewritten", the cruise number only drops after the rewrite run; (b) 258 of those edges are ui-react importing framework ui sources, which the alias trades for another rule (below).

## 5. Rules and cycles the batch does not fix (decisions, nothing applied)

- `ui-no-framework-packages` would fire for **136 keys / 267 edges** once importers use the alias: the policy allow-list has the source form of the ui module but not the glue form `🧰️framework/📦️packages/🟦️typescript/🖱️ui/` (it has it for transient, interaction, manifest generated). One library line (frozen, queue): add the glue forms of `🖱️ui`. The keys are harmless until then.
- Rules that fire before and after the alias (unchanged by the batch): `plugins-framework-sdk-only` 22 edges, `plugin-no-extension-or-artifact-fem` 13 and `-energy` 5, `renderer-hosts-only-ui` 7 (needs the lay14 ruling: OS glue only under `./modules/renderer/engine/elements/`), `framework-no-products` 3.
- Declared-dependency cycles that adopting the keys would close (importer package to entry package): ui-react to framework (258 edges, framework already depends on ui-react), nx-bootstrap-tools to repo-lib (95; those importers must reference the glue relatively, as lay15 did), repo-coordinator to repo-lib 10, fem-js to fem-2d/fem-3d 12, framework-renderer-react to framework-os 3, framework-kernel to framework 4, course to course-react 2, repo-lib to plugin-registry 2, repo-test to repo-lib 1. The batch itself adds no workspace dependency edge that closes a cycle (5 refused, including wgpu to plugin-registry from tst6: plugin-registry already depends on wgpu).
- Names: `import-names.ts` found 3 bare imports asking for names the entry does not export: `repo-lib/laws` (three names, tsc-stage patch 2), `print-viz-inference/script` `verifyVizChartInferences` (the function was lifted to `🧪️tests/🔬️typescript-package-commands`; needs a key for that module and an importer change, not in this batch), `plugin-registry/hub-source/service-worker` `default` (a `?worker&url` virtual import, fine).
- Unresolvable packages in code: `framework-surface-{node-graph,paint,tiled-map,terrain}-rs` and `flow-core` (stale `.storybook/preview.tsx`, flow wasm bindings), `captured-fixture`, `fixture-wasm` (fixtures).

## 6. Apply procedure

1. Wait until the freeze is lifted and `bun repo dependencies sync` has exited. Apply the library patches first: tsc-stage (patch 8 is a hard prerequisite of the `./script` removal; patch 2 adds the laws names), tax-stage. If tax-stage or anything else moved targets of the new glue, run `bun C/exports-stage-tools/stage.ts` again (the only target-dependent entry, `./build/script`, is already blocked).
2. `bun C/exports-stage-tools/apply.ts check` must say `plan: 34 manifests, 379 glue files to create, 21 to extend, 19 to delete, 23 importer files to rewrite`.
3. `bun C/exports-stage-tools/apply.ts apply --freeze-lifted`. Optional tiers: `--include=hygiene,physical`.
4. `bun repo dependencies lock`, then sync; then `ncpr2-edges.ts` and `lay15-tools/ncpr2.ts ... apply`; then the per-area cruise.

`apply.ts` refuses when: a target manifest, an extended glue or a deleted glue differs from staging (sha256); a `dependencies sync|lock`, `deps4-*.sh` or `chain*.sh` process runs; a glue file to create exists with other content; a glue re-export target vanished or no longer mentions the exported name; a removed key is still imported anywhere (`git grep`, minus the staged rewrites); a manifest does not round-trip through `JSON.stringify(..., 2)`. It writes originals to `C/🗑️generated/exports-stage/applied-<ts>/` (hash-named) and verifies afterwards that every added value exists and every written glue parses. It never runs lock/sync and never commits.

## 7. Verification (exact commands)

- `bun C/exports-stage-tools/stage.ts --refresh`: all 12 steps rc 0 (2.5 minutes; the tst6 additions were re-run step by step afterwards); `missing-keys` and `import-names` run standalone (`missing-keys`: 1 hit, a string fixture; `import-names`: 10,799 imports checked, 3 misses above).
- `bun .../validate.ts` (virtual overlay of the 506 selected entries): `selectedEntries 506, exportAdds 408, exportRemoves 20, dependencyAdds 81, glueCreate 379, glueExtend 21, glueDelete 19, rewrites 25, aliasDuplicatesIntroduced 0`, **problems: {}** (every export target exists after the overlay, every glue re-export resolves and names a real export with the right type/value kind, rewritten importers' names exist in the destination, no deleted glue still imported or exported, no cycle or already-declared dependency, no path over 256 characters or a component over 255 bytes, no banned path stem).
- `bun .../apply.ts check --ignore-sync --ignore-removed`: plan line above, nothing written. Without the overrides it refuses today, as intended: `dependency sync/lock is running` (deps4 loop pid 38014, chain4) and the `./script` importer.
- Scratch end-to-end: `bun .../scratch.ts <dir>` mirrors the 535 files the batch reads or writes, `bun .../apply.ts apply --root=<dir> --freeze-lifted`: applied, **0 verification problems**, the diff of the root manifest is about 55 appended lines, the value manifest 20 appended keys, the tools manifest 1 removed and 5 added; a second `check` against the mirror refuses on the changed baselines.
- Calibration cruise: `lay13-tools/cruise-area.sh` for `🌎️hub` (3:41) and `📚️course`, `📓️print`, `♻️mit-bestand` (766 modules): ncpr pairs equal to the evaluator.
- Not run: the full per-area cruise on the overlay (the cruise reads the live manifests; the evaluator above is its stand-in), any bun/vitest suite, tsc over the new glue (names and kinds are checked against the targets' exports, the Bun transpiler parses every glue file), `bun repo dependencies lock`, any cargo.

## 8. Files

Kept inputs in `C/exports-stage-tools/`: `batch.json`, `apply.ts`, `stage.ts`, `scratch.ts`, `tst6-gaps.ts`, `scan.ts`, `rules.ts`, `ncpr-now.ts`, `moves.ts`, `groups.ts`, `build-candidates.ts`, `deps.ts`, `health.ts`, `assemble.ts`, `validate.ts`, `simulate.ts`, `reconcile.ts`, `stats.ts`, `import-names.ts`, `missing-keys.ts`, `ncpr2-edges.ts`, `glue-ops.ts`, `inputs.ts`, `cycles.ts`, `model.ts`, `lib.ts`, `policy-dump.cjs`. Generated output (`C/🗑️generated/exports-stage`, scratch mirror, calibration cruise JSON, the `[DEBUG]` stage log) deleted. The apply writes its backups to `C/🗑️generated/exports-stage/` again.

## Corrections 2026-10-20 (stage-fix)

- Counts recomputed from `batch.json`: 634 entries, 506 apply, 408 export adds, 20 export removes, 81 dependency entries, 379 glue creates, 21 extends, 19 deletes, 25 rewrites, 34 manifests (the earlier figures 636/508/405/376 were stale).
- Twelve export keys ended in `/.json` (an emoji-only `🔣️.json` stem was dropped by the ASCII filter). They now use the directory stem (`./artifacts/storage/schema.json`); the ten repo-lib glue wrappers moved from `…/🔣️.json/🟦️.ts` to `…/🔣️json/🟦️.ts` (the `🔣️json/<stem>` wrapper rule). `build-candidates.ts` derives the same keys on a re-stage.
- The `dotenv` devDependency entries (repo-lib and root) are dropped: tsc-stage patch 14 removes the import.
- The tax-stage package.json overlap is gone: tax-stage no longer renames the tools bazel script, so only the already-staged removal of `./script` remains in this batch.
