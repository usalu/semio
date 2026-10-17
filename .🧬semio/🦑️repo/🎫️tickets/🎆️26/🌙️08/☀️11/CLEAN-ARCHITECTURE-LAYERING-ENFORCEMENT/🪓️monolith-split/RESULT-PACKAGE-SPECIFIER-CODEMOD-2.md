# Result: Package Specifier Codemod 2 (slice PACKAGE-SPECIFIER-CODEMOD-2, prefix `psc2`)

Status: DONE (2026-10-16 16:21 to 20:10). Tools: `psc2-tools/` (this folder). Logs and raw outputs: `🗑️generated/psc2-*` (kept). Folder `F` = this folder.

## 1. Verdict

- The generated family `no-cross-package-relative-*` (exact cruiser command, section 4) fell from **8,429 to 4,877 error lines** (outside `🧪️tests`: **4,841 to 2,848**) and from **6,795 to 3,988 distinct edges** (outside tests: **4,098 to 2,403**). As distinct (rule, importer, target) lines, 3,733 were removed and 181 added (new files of peers, and edges that the new package regions now flag in skipped areas).
- 1,875 importer files were rewritten to package specifiers (about 3,200 specifier edits), 549 facades were created and 69 extended, 30 TypeScript packages were created, 555 explicit `exports` entries were added, 125 manifests got `workspace:*` dependencies, and all six Bun locks and `workspaces` lists are consistent (section 7).
- `not-to-unlisted` did not rise: 890/140 to 405/0 (the drop is the `hr2` slice; none of the remaining lines involve a dependency I touched). `no-circular` did not rise: 574/471 to 574/470. The cycle lines the cruiser reports as "added" are the same cycles with another member named: `cycle.ts` finds the same cycle in the pre-edit tree through snapshot overlay (section 4).
- What is left in the family is, in production lines: the skipped areas (library, `pi1`, `rhu2`, `sf3`: 2,274 production edges by AST scan, 100 percent prepared for hand-off, section 8), the UI internals (145 edges, decision needed, section 9), 4 cross-scope test edges and about 20 edges to assets and JavaScript targets that cannot be expressed as TypeScript facades (section 9).
- Verification (section 5): `tsc` over the whole repository (13k files) with the tree as it was before my edits (a compiler host overlay of every snapshot) against the final tree: 0 new errors in any touched file or facade (6 existing "no exported member" lines changed only the printed module path); tests: 905 test files before versus after, 895 identical, 10 differ for peer or environment reasons that I checked one by one; 3 source-text law tests needed a fix and got it; every new export loads in a fresh bun process (or needs the test runner or Vite, listed) and resolves through a real Vite plugin container; 6,867 of 6,936 specifier imports in the tree resolve exactly to their export target (the 69 others are pre-existing built-output entries).
- Not run: `bun repo run …:workspaces-check`, `verify taxonomy`, `verify layering` (the launcher is unreliable under load); nothing was installed (`bun install` was not run; the `node_modules` links were made by hand, section 7).

## 2. Method and tools

Generalised from `RESULT-PACKAGE-SPECIFIER-CODEMOD.md`. The psc tools were fixed to three owners; the psc2 tools take every workspace package from the live policy snapshot.

| Tool | What it does |
| --- | --- |
| `dump-policy.cjs` | loads `loadDependencyDirectionPolicy` and dumps all workspace packages and the exact regexes of every `no-cross-package-relative-*` rule (302 packages and rules at the end) |
| `model.ts`, `scan.ts` | TypeScript-AST scan of about 13.5k source files; an edge is flagged when the importer is outside the owner's owned region (`from.pathNot`) and the resolved target matches the rule's `to.path` and not its `to.pathNot` |
| `skip.ts`, `skipsumm.ts`, `skipped-tsv.ts` | skip rules (importer or target in a running slice), summaries, and the full list of remaining edges (`🗑️generated/psc2-skipped-edges.tsv`) |
| `plan.ts` | per target file picks the public entry (below) and writes the plan JSON |
| `apply.ts` | writes facades (merging into existing ones), explicit `exports`, new package manifests; snapshots every file it changes |
| `rewrite.ts` | re-scans each importer right before editing and patches only the module-specifier literal (JSON default imports become named imports; three dynamic JSON import shapes are handled); adds `workspace:*` to the importer's owning package (`dependencies` for production, `devDependencies` for test-only); bootstrap entries (`📜️script.ts`, Vite builders, vitest configs) get a relative path to the facade |
| `link.ts`, `workspaces.ts`, `lockgen.sh`, `lock.ts`, `lockcheck.ts`, `depcheck.ts` | `node_modules` links, the six `workspaces` lists, lock derivation and transplant, and a check that every added dependency resolves in each Bun scope that installs the depending package |
| `mktsc.ts`, `tsc.sh`, `overlay-tsc.mjs`, `tscattr.py` | whole-repository type-check, the same check on the pre-edit tree without touching the disk, and attribution of the difference |
| `picktests.ts`, `runtests.ts`, `testdiff.ts` | test selection, per-file `bun test` runs, comparison by failing test name |
| `facade-check.ts`, `facade-load.ts`, `vite-check.mjs`, `spec-check.ts`, `unused.ts`, `unused2.ts` | facade statements exist, fresh-process load, Vite resolution, exact-target resolution of every specifier import, unused exports |
| `cruise.sh`, `ruletab.ts`, `errdiff.ts`, `attrib.py`, `cycle.ts`, `whatif.ts` | the cruiser runs, per-rule tables, set differences (ASCII keys, because the cruiser output has mojibake in emoji), attribution to my files, cycle finder with snapshot overlay, carve simulation |

Public-entry decision per target file (`plan.ts`):
1. An existing exported facade that already covers every needed name (checked with the TypeScript checker by symbol identity) is reused, but only if it re-exports that single module or sits on the target's own mirror path. The package root barrel is never used: it would drag in the whole package and create cycles.
2. Otherwise the missing names are added to that facade.
3. A real module that already lies inside the TypeScript package directory is exported directly.
4. Otherwise a new facade is created on the mirrored path inside the TypeScript package directory (the SDK mirror drops `🔨️modules/`; `🧪️tests` becomes `🧪️testing` so test discovery cannot collect a facade; a path over 236 bytes drops leading segments; a key already used by a built `dist` entry gets a `📥️source` prefix). Names are the union of what importers use, value versus type from the checker, `export *` and namespace imports with unknown use export everything; default imports are re-exported as `default`.

Owners without a TypeScript package get one (section 3): `@semio-tech/schema-ts` style manifests with explicit `exports`, registered in the six `workspaces` lists.

## 3. Batches

| Batch | Scope | Importer files / edits | Facades new / extended | Manifests with new `dependencies` | Verification |
| --- | --- | ---: | ---: | ---: | --- |
| b1 | norm contract (`@semio-tech/norm-js`, 690 artifact twins) | 691 / 691 | 2 / 0 | 15 | tsc norm closure 3 to 3 errors; tests 54 of 55 identical (1: taxonomy load error of a peer) |
| b2 | process and the SDK (`@semio-tech/process`, `@semio-tech/framework`) | 579 / 975 | 69 / 41 (+4 direct) | 26 | tsc 0 added in touched files; tests 373 of 379 identical |
| b3 | every other owner with a TypeScript package (os, os-dev, os-mcp, print, 2d, kernel, replication, actor, assets, async, repo-test, pets, stdio artifacts, block, fem, gis, trinity, hub) | 481 / 852 | 273 / 8 | 72 | tests 374 of 377 identical, 3 explained |
| b4 | 23 new TypeScript packages for Rust-only owners | 178 / 299 | 104 / 0 | 38 | tests 159 of 162 identical, 3 explained |
| b5 | carve of `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine` into `@semio-tech/s-spatial-kernel-ts` | 55 / 251 | 27 / 2 | 12 | tests 61 of 63 identical, 2 explained |
| b6 | carves `cad-engine-ts`, `s-dev-catalog-ts`, `s-dev-block-ts`, `stdio-inventory-ts`, `s-fem-engine-ts` | 43 / 64 | 22 / 2 | 9 | tests 35 of 35 identical |
| b7 | carve `framework-ui-viewport-ts`, ui-react and ui-styling entries | 38 / 62 | 11 / 1 | 8 | tests 49 of 49 identical (after a fix, section 6) |
| h1 | hand-off facades for skipped importers (no importer edited; one test edit was reverted, section 6) | 0 | 41 / 25 | 0 | load and Vite resolution |
| fix1 | `export * as core` of the CAD entry needed every kernel name | 0 | 0 / 1 | 0 | CAD layering test |

Totals from the snapshots: 1,875 importer files edited (3,194 specifier edits), 549 facade files created, 69 extended, 87 manifests with new or changed `exports`, 125 manifests with new dependencies, 30 new packages.

New TypeScript packages (30): `@semio-tech/layout-layout-ts`, `cad-cad-ts`, `puzzle-3d-ts`, `framework-time-travel-ts`, `framework-tool-run-ts`, `framework-diagnostic-ts`, `framework-pack-error-ts`, `framework-pack-record-ts`, `framework-pack-ts`, `framework-pack-json-ts`, `framework-mesh-engine-ts`, `framework-compiler-ts`, `framework-pixels-ts`, `framework-canvas-ts`, `framework-deflate-ts`, `framework-dsl-ts`, `framework-dsl-record-ts`, `framework-surface-ts`, `framework-io-base64-ts`, `framework-io-schema-ts`, `framework-sqlite-snapshot-ts`, `framework-tool-machine-ts`, `neural-engine-ts` (all b4, siblings of the Rust packages), and the carves `s-spatial-kernel-ts`, `cad-engine-ts`, `s-dev-catalog-ts`, `s-dev-block-ts`, `stdio-inventory-ts`, `s-fem-engine-ts`, `framework-ui-viewport-ts`. Each lives at `<owner>/📦️packages/🟦️typescript` with `private`, `type: module` and explicit `exports` only.

Target packages by production error lines (cruiser, all sibling rules counted, before to after): `norm-js` 691 to 0, `process` 693 to 504 (the rest is the library), `framework` and `framework-rs` 376 to 223, `framework-os` 187 to 87, `semio-s-workspace` 135 to 0, `print` 40 to 1, `framework-os-dev` 37 to 13, `block-js` 29 to 0, `stdio-xml` 26 to 0, `framework-2d-js` 23 to 0, `framework-sqlite-snapshot-rs` 20 to 2, `print-viz-inference` 12 to 0, `framework-pixels` 10 to 0, `cad-js` 9 to 0. Unchanged because the importers or targets are in a skipped area: `repo-lib` 1,018 to 1,025, `schema-validator` 102 to 104, `plugin-registry` and `framework-plugin` about 102.

## 4. Cruiser (exact command)

`node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (alarm 7200 s, via `psc2-tools/cruise.sh`).

| Run | Window | Exit | Error lines | Modules / dependencies | Raw |
| --- | --- | ---: | ---: | --- | --- |
| before | 17:19 to 17:37 (1,073 s), tree untouched by me | 63 | 13,375 | 17,804 / 50,168 | `psc2-before-err.txt` |
| after1 | 19:06 to 19:16 (605 s), after b1 to b6 | 159 | 8,863 | 18,338 / 51,341 | `psc2-after1-err.txt` |
| after2 (final) | 19:31 to 19:47 (967 s), after b7 and the last edit that affects the graph | 127 | 8,831 | 18,419 / 51,519 | `psc2-after2-err.txt` |

The first two attempts of the baseline failed for operational reasons: the first ran at nice +5 (zsh `BG_NICE`) and was starved, the second was killed with SIGTERM after 30 minutes by something outside this slice. `cruise.sh` now retries on SIGTERM.

| Rule (all / outside `🧪️tests`) | before | after1 | after2 |
| --- | ---: | ---: | ---: |
| `no-cross-package-relative-*` lines | 8,429 / 4,841 | 4,911 / 2,878 | 4,877 / 2,848 |
| `no-cross-package-relative-*` distinct edges | 6,795 / 4,098 | 4,007 / 2,421 | 3,988 / 2,403 |
| `not-to-unlisted` | 890 / 140 | 405 / 0 | 405 / 0 |
| `no-circular` | 574 / 471 | 574 / 470 | 574 / 470 |
| `ui-no-framework-packages` | 271 / 215 | 297 / 232 | 297 / 232 |
| `framework-no-products` | 159 / 106 | 174 / 116 | 176 / 117 |
| `plugins-framework-sdk-only` | 626 / 558 | 626 / 558 | 627 / 558 |
| `renderer-hosts-only-ui` | 391 / 217 | 391 / 217 | 391 / 217 |
| `no-core-path` | 998 / 628 | 898 / 550 | 901 / 553 |
| total error lines | 13,375 | 8,863 | 8,831 |

(The "outside tests" column uses an ASCII-projected test path filter, because the cruiser output has mojibake in emoji. The `psc` figures of the previous slice used a different filter, so the 4,841 differs by a few lines from the audit's 4,849.)

Attribution of the rules that moved up (`attrib.py`, ASCII keys, importer touched by a rewrite or target created by an apply):
- `ui-no-framework-packages` +16 production lines: the ui files I touched keep their old edge and gain the edge to the facade (equal count), plus 28 new edges of a Python builder tree that peers created after the baseline (`🎨️styling/🏗️builder/🐍️python/**`, files from 18:33 to 19:06).
- `framework-no-products` +11 and `no-core-path` +3: edges of the same new peer files, and test edges that moved from the module to a facade.
- `not-to-unlisted`: the 12 lines that appeared between after1 and after2 are test files of peers importing third-party packages; the production list is empty.
- `no-circular`: of 21 added lines 2 had an importer I touched (`kernel/🏷️notice-labels`, `kernel/📦️package-assets`). `cycle.ts` with `OVERLAY` (the pre-edit text of every touched file) finds the same two cycles, so they pre-exist; the cruiser prints one arbitrary member per cycle and the member changed.
- `plugins-framework-sdk-only` removed 130, added 131 and `no-state-outside-os` 82 and 82: the same edges, re-rendered with the facade as target.

## 5. Verification

| Check | Result |
| --- | --- |
| `tsc --noEmit` over the whole repository (12,955 files), tree before my edits (compiler host overlay of all 22 snapshot files, `overlay-tsc.mjs`) versus final tree (`psc2-tsc-all-before-overlay.txt` 1,975 diagnostics, `psc2-tsc-all-after.txt` 1,946) | 40 diagnostics removed, 11 added; of the added, 6 are existing `TS2305` "no exported member ValueRefusalKind, PagedRefusalMetadata, PagedAllocationMetadata" lines in two `🎒️pack` producer files whose printed module changed from the relative path to the specifier (the same 6 are in the removed list), 5 are in peer files (library tests, process cleanup test, CAD story). 0 new errors in touched files or facades. b1 was also checked with a real before-run (norm closure, identical), b2 to b4 per batch with the same overlay method (0 added in touched files each time). |
| `bun test` per file: 905 files that were run before each batch and after the last edit (`psc2-tests-final-before.json`, `psc2-tests-final-after.json`) | 895 identical (pass/fail counts, exit, failing names), 10 differ: 3 improved, 7 worse, all checked (below). Totals: 5,851 pass / 1,088 fail before, 5,872 / 1,099 after. |
| New entries in a fresh bun process (`facade-load.ts`) | b1 2/2, b2 114 with 0 failures, b3 283 with 3 (a `?url` import of a wasm path, a `?worker&url` import, a module that calls `test()` and needs the test runner), b4 104, b5 29, b6 24 with 1 (`brepjs_single.wasm?url`), b7 12, h1 68 with 3 (modules that call `test()`). Every failure needs Vite or `bun test`; none is a missing name. |
| Real Vite plugin container resolves every planned specifier to its target (`vite-check.mjs`) | 2, 114, 283, 104, 29, 24, 12, 68: 0 failures |
| `spec-check.ts`: every import of a workspace package specifier resolves exactly to its authored export target | 6,867 of 6,936 ok; the 69 bad ones are pre-existing entries whose target is built output (`dist`, `pkg`) or a package without a root link; 0 are on a specifier of an export this slice created or extended |
| `facade-check.ts`: every facade statement targets an existing file and every export key targets an existing file | b1 to b7, h1, fix1: 0 bad (after the last edit) |
| Unused exports | 555 created; 89 had no specifier importer: 41 are the hand-off set (h1, no importer yet), about 40 are reached by bootstrap entries through a relative path to the facade, the 4 hub exports whose only importer was reverted were deleted, and 4 (`framework-replication/apply-refusal/**`, `framework-surface-ts/node-graph/scene/testing/fixtures**`) lost their importer to peer work in flight (`unused.ts`, `unused2.ts`) |
| `depcheck.ts`: every added `workspace:*` dependency resolves in every Bun scope that installs the depending package | 371 additions, 0 unresolvable (one found and fixed, section 6) |

The 10 test files that differ (all re-run by hand):
- `norm/🧪️tests/🔮️oracle-source-ownership`: better (a peer fixed the taxonomy loader).
- `replication/⚠️error/🧪️tests`, `os/🔨️modules/🛢️db/🧪️tests/📦️crate-boundaries`: better.
- `process/…/🧾️receipt/📦️retention/🧪️tests`: 3 more failures that read the text of `🧩️component-pass/🟦️.ts`, which a peer edited at 19:03 (the stall watchdog, not an import).
- `process/…/💾️storage/📤️publication/🧪️tests` (two files): the same peer edit family, `Original parent Script process invocation required` environment errors.
- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🏃️command-ownership`: `phase.leaseControl.signal` undefined because a peer changed `stdioCargoMetadata` to use `phase.leaseControl` while the test's fake ports still lack it.
- `ui/🎨️styling/🏗️builder/🐍️python/🧪️tests`: a Python environment test of a peer's new tree.
- `…/🧪️test/🧪️tests/⚖️comparison-profiles` and `🔍️discovery-contract`: `exit null` (killed by the 240 s timeout under load) before, `Original finite test artifact callback required` (launcher environment) after.

## 6. Decisions, fixes and non-exports (each case recorded)

Public API versus internals. I created facades only where an importer outside the package uses the module. The cases where the module is arguably an internal implementation detail that other packages reach through the module boundary, and that I still exposed because the importer is a production sibling of the same domain, are: the norm wire contract (`norm-js/registry/contract`, 690 twins), the process native-build storage and source modules, the pack and io schema modules. The hub admin elements import 4 hub entries (`os-hub-ts/observability`, `testing/fixtures/observability-v1`, …). Not exported, recorded:
- `🧰️framework/🔨️modules/🖱️ui/**` internals imported by `🖱️ui/🎯️targets/⚛️react` and others (145 production, 9 test edges, section 9): a facade through `@semio-tech/framework` would add a `ui-no-framework-packages` violation per edge, and a `@semio-tech/ui-*` package would publish about 70 element, hook and style modules of the ui as a package surface. That is the owner's decision (P3).
- Non-TypeScript targets: 8 `.dsl.semio` examples, fonts (`.ttf`), `.glb`, `.svg`, `.css`, `.jpg`, `.png`, and `.js`/`.mjs` modules (15 production, 16 test edges): a TypeScript facade cannot re-export these URL or text imports. They need a direct export of the asset file from a package that contains it, or a move of the asset.
- Four JSON dynamic imports in `.js` flow wasm tests (`dynamic-json-shape`).

Fixes made while verifying:
- Source-text law tests that required relative imports only: `🧬️schema/📄️source/🔗️closure/🧪️tests` and `🎒️pack/🔤️json/📥️decode/🧪️tests` now accept `@semio-tech/` specifiers (they still forbid products, the hub and `🖥️s`); `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔬️layering` bundled with `packages: "external"`, which externalised the kernel package, so it now externalises only third-party packages with a small esbuild resolve plugin.
- `export * as core` of the CAD package entry lost kernel names behind an explicit facade: the plan now exports every name for a namespace re-export, and the `cad-engine-ts/kernel` facade was extended (fix1).
- JSON exports whose name starts with a digit (`2dSchema`, `3dSchema` in the viewport package) were invalid identifiers: renamed `viewport2dSchema`, `viewport3dSchema` with their 6 users, and `plan.ts` now prefixes the module name.
- Empty side-effect facades (a dynamic import of a module that only registers tests) were not modules (`TS2306`): they now carry `import "…"`.
- Cross-scope: a `✏️s` test (`🎭️variants/🌍️gis/💡️services/🧪️tests/🤝️hub-edit-durability`) reaches into the hub by a relative path. A specifier `os-hub-ts/…` made `bun install` of the `✏️s` scope fail (`Workspace dependency "os-hub-ts" not found`), because that scope does not list the hub packages. I reverted its 4 lines and the dependency, deleted the 4 hub facades that served only it, and `skip.ts` now skips `✏️s` and `🧰️framework` importers of `🌎️hub` (4 test edges remain, section 9).
- A peer (`tfe`) already uses my new packages (`framework-pack-record-ts`, `framework-pack-error-ts`) from its fixtures aggregator; I added one name to `framework-pack-ts/testing/fixtures`, which its `os` test needed.
- The norm twin generator `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-norm-ts-twins.ts` writes the relative contract import; its one line now writes `@semio-tech/norm-js/registry/contract` for twins under `🗿️artifacts/` (the script already fails with `ENOENT` on a schema path of a renamed twin, a pre-existing defect I did not touch).

## 7. Lockfiles, workspaces, links

- 30 new packages are listed in the `workspaces` of the scopes that list a sibling (root and `🎓️teaching` for framework packages; `✏️s`, hub, `♻️mit-bestand`, `🏢️semio-tech` for all); each list stays byte-sorted. Command for the compile window: `bun repo run @semio-tech/repo-lib:workspaces-check --skip-cache` (not run).
- Locks: `bun install --lockfile-only --offline` in each of the six scopes (`lockgen.sh`; the generated lock is moved aside, the checked-in one restored at once), then `lock.ts` transplanted only the semantic differences (changed workspace dependency lists, new workspaces, new package rows) into the repo's `JSON.stringify` format. `lockcheck.ts`: for all six scopes `workspaces` deep-equal what bun derives, no derived package row missing; 1 third-party row (`is-promise` or `readable-stream`) differs in the root and `✏️s` locks, unrelated to this slice. Backups: `🗑️generated/psc2-lock-backup/`. Run once after b6 and again after b7, h1 and the cross-scope revert.
- `node_modules/@semio-tech/<name>` and `node_modules/os-hub-ts`: about 63 absolute symlinks to the package directories for every package an importer now reaches by specifier (as `bun install` would make them; untracked). A fresh checkout gets them from `bun install`.

## 8. Skipped areas and hand-off

Skip rule: an edge is skipped when its importer or its target lies in `rhu2` (renderer TS, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/`), `pi1` (`…/🔨️modules/🔌️plugin/`), `sf3` (`✏️s/🔌️plugins/🌀️procedural/`) or the parent-held `📚️library`. The hub directory (`🌎️hub/📇️directory`) is finished (`hdc`, FLEET 15:5x), had two test edges and is included. `orc` (os-pack) had no TypeScript edge.

Remaining flagged edges after the last edit (AST scan `psc2-scan-13.json`, list with importer, line and target in `🗑️generated/psc2-skipped-edges.tsv`):

| Reason | Production | Test |
| --- | ---: | ---: |
| parent (`📚️library`) | 1,571 | 796 |
| `pi1` (plugin) | 395 | 354 |
| `rhu2` (renderer TS) | 290 | 838 |
| `sf3` (procedural) | 18 | 11 |
| cross-scope (`✏️s` or framework importer of a hub package) | 0 | 4 |
| open: ui internals | 145 | 9 |
| open: assets and JavaScript targets | 15 | 16 |
| open: other | 5 | 6 |

Hand-off, no facade work is left for the skipped importers: plans `h1` and `h2` (`WITH_SKIPPED=1 psc2-tools/prep.sh <label> <batch> --all-ts`, which plans every edge whose target is outside a skipped area, whoever the importer is) added the 46 entries and 27 extended facades their names need, with a fresh load and Vite check. The owner of a skipped area rewrites its own files with `bun psc2-tools/rewrite.ts <plan> <scan> <policy> --area <path prefix> --ignore-skip --write --snapshot <file>`, plan = a fresh `WITH_SKIPPED=1` plan (scan and policy from `psc2-tools/fresh.sh <label>`; `psc2-plan-h2.json` and `psc2-scan-15.json` are current). The dry run lists 367 skipped importer edges whose facades now exist (apart from the two peer files below). The library needs `@semio-tech/process`, `@semio-tech/schema-validator-ts` and `@semio-tech/framework`; its bootstrap files (`⚡️caching/🚀️bootstrap/🛠️tools`, package `nx-bootstrap-tools`) must keep relative paths to the facades.

`psc2-plan-h2.json` would also re-point 2 files that a peer (`tfe`) was creating while I worked: `🧰️framework/📦️packages/🟦️typescript/🌱️value/♻️retirement/🧪️testing/🧫️fixtures/🟦️.ts` and `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🧫️fixtures/🟦️.ts` (3 edits); I left them alone.

The open edges that are not ui or assets: the Rust-package `🧠️semio/🌊️session` bindings of the spatial kernel (3 production edges plus 1 test; the package is `🕸️bindings` and has no TypeScript package directory), and the `compute-ownership-runner` consumer edges in `◻️2d/🧮️compute` (3 plus 1 test and 4 test edges; the package directory is the package, no `📦️packages/🟦️typescript`).

## 9. Open items and decisions for the parent

1. UI internals (145 production, 9 test edges): decide between a ui TypeScript package with a deliberate public surface (what-if: 50 ui files would then be flagged against the SDK, which keeps `ui-no-framework-packages` equal) and moving the elements into `🎯️targets/⚛️react`.
2. Assets and JavaScript (31 edges): direct exports of asset files, or move.
3. `✏️s` to hub test (4 test edges): the test belongs to the hub or needs the hub packages in the `✏️s` Bun scope.
4. Compile window: `workspaces-check`, `verify taxonomy` for the 30 new `📦️packages/🟦️typescript` folders, the offline lock refresh, `bun install` (it creates the links I made by hand).
5. The two `ui` source-text laws I fixed accept any `@semio-tech/` specifier; tightening them to an allowlist is the owner's call.

## 10. Commands run (real results)

| # | Command | Result |
| --- | --- | --- |
| 1 | `psc2-tools/cruise.sh before full` (third attempt) | exit 63, 13,375 errors, 1,073 s |
| 2 | `node psc2-tools/dump-policy.cjs` (14 times) and `psc2-tools/scan.ts` (13 times) | 272 to 302 packages and rules; 4,183 production edges at 16:24, 2,439 at 19:40 |
| 3 | per batch: `plan.ts`, `apply.ts --write`, `rewrite.ts --write`, `link.ts --write`, `verify.ts`, `facade-check.ts` | section 3; `verify.ts` reports only peer edits after my write (line counts and non-import lines in 16 files, listed in `psc2-snap/`) |
| 4 | `bun psc2-tools/runtests.ts` per batch, before and after, and the 905-file final run | section 5 |
| 5 | `psc2-tools/tsc.sh` b1, b4 (real before and after), `overlay-tsc.mjs` b2 to all | section 5; the b2 real "before" run failed (a peer renamed SDK folders during it), which is why the overlay host was written |
| 6 | `psc2-tools/lockgen.sh` six scopes, three rounds; `lock.ts --write`; `lockcheck.ts` | exit 0 except two rounds that failed on the unresolvable `os-hub-ts` dependency (fixed, section 6); final lockcheck clean |
| 7 | `bun psc2-tools/depcheck.ts psc2-snap` | 371 additions, 0 unresolvable |
| 8 | `psc2-tools/cruise.sh after1 full`, `after2 full` | exit 159 (8,863) and 127 (8,831) |
| 9 | `bun psc2-tools/spec-check.ts`, `unused.ts`, `unused2.ts`, `facade-load.ts`, `vite-check.mjs`, plan `h2` apply | section 5; h2: 5 facades created, 2 extended, 16 entries loaded (2 need the test runner) and resolved |
| 10 | not run: `bun repo run @semio-tech/repo-lib:workspaces-check`, `verify taxonomy`, `verify layering`, `bun install` | see section 1 |

## 11. Files changed

- 1,875 importer source files (keys of the `*-rewrite.json` snapshots in `🗑️generated/psc2-snap/`), 549 new facade files and 69 extended ones (`*-apply*.json`), 30 new `package.json`, 87 manifests with new `exports`, 125 manifests with new dependencies, six `bun.lock` files, the `workspaces` list of the root and five scope `package.json` files.
- Hand-edited: the three law tests of section 6, the facade `🎒️pack/📦️packages/🟦️typescript/🧪️testing/🧫️fixtures/🟦️.ts` (one name), the cross-scope revert in `hub-edit-durability`, the norm twin generator script (one line).
- Untracked environment: `node_modules/@semio-tech/*` links.
- Not touched: `AGENTS.md`, `.vscode`, the policy or taxonomy files, the rule generator, any ticket tool, any git state. No `[DEBUG]` log was written to the repository.
