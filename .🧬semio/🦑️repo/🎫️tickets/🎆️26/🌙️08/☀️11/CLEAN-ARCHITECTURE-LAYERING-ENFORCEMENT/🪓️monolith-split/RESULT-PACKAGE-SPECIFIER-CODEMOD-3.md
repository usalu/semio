# Result: Package Specifier Codemod 3 (slice PACKAGE-SPECIFIER-CODEMOD-3, prefix `psc3`)

Status: DONE (2026-10-16 20:04 to 20:57). Tools: `psc3-tools/` (this folder; a copy of `psc2-tools/` with the `sf3` area removed from `skip.ts`, plus `opens.ts` and `resolve-check.ts`, and an arrow fix in `attrib.py`). Logs and raw outputs: `🗑️generated/psc3-*` (kept). Folder `F` = this folder.

## 1. Verdict

- The `sf3` area (procedural and generation) is rewritten to package specifiers. In the exact cruiser run, the rules of the packages I touched went to 0: `procedural-js` 16 to 0, `procedural-generation3d` 4 to 0 (and the `-rs` sibling rule 4 to 0), `s-spatial-kernel-semio-session` 3 to 0, `2d-compute` 6 to 0, `compute-ownership-runner` 6 to 0, `os-hub-ts` 5 to 1 (the one left is a library test, parent-held), `no-cross-technology-✏️s-to-🌎️hub` 4 to 0.
- My net effect on the family `no-cross-package-relative-*` is 43 lines removed and 1 added (the moved gate's runner imports the library by a relative path, as the old runner in the services script did; the parent's library pass fixes both). Whole-run family totals went up (4,893 to 4,923 lines), because peers created files after the baseline (88 added lines, section 5). `not-to-unlisted` is unchanged (405 lines, 0 production, no set difference). `no-circular` fell from 584 to 568 lines; the set difference is 14 removed (renderer tests, `rhu3`) and 2 members added in a peer's process module (section 4). Neither involves a file of mine.
- The 4 `✏️s` to hub test edges are solved by moving the gate into the hub composition test root (section 3, batch c3). The old `no-cross-package-relative-os-hub-ts` lines (4) and `no-cross-technology-✏️s-to-🌎️hub` lines (4) of that file are gone.
- The 145 UI-internal edges and the 31 asset and JavaScript edges are listed with concrete proposals in section 8, for the owner. I did not decide them and did not touch them.
- Verification (section 5): whole-repository `tsc` with the tree as it was before my edits (compiler host overlay) against the final tree: 1,925 diagnostics both, 0 in any touched file, 4 removed and 4 added, all in peer files. Test sets: 16 files identical or smaller (one file moved, same failure at the new path), the MCP session test file 7 pass / 1 fail before and after. New exports: 15 load in a fresh bun process and resolve through a real Vite plugin container, 0 failures. Workspace membership check and lock derivation are clean.
- Not run: `bun repo run …:workspaces-check` through the launcher (I called its function `publishWorkspaceMembership(root, "check")` directly instead), `verify taxonomy`, `verify layering`, `bun install`. The moved gate itself (`hub-edit-durability`) needs the launcher environment and a hub, so it was not executed.

## 2. Scope decision

The skipped areas of `psc2` and their state according to `📓️FLEET.md` at 20:04:
- `sf3` (procedural, generation): finished 19:1x. Free, handled here.
- `📚️library` (parent), `🔌️plugin/**` (`pi2` running), renderer TS (`rhu3` running): still skipped, as ordered.
- No other slice that owned a skipped area had finished. `hdc` and `orc` were already included by `psc2`.

What the fresh scan (`psc3-scan-1.json`, 20:07, 13,565 files, 4,488 edges) found in the `sf3` area: 21 edges to rewrite (11 production, 10 test): the generation2d and generation3d sqlite and schema snapshot modules and their tests, the generation3d bootstrap script's process capacity import, and 2 test files of `✏️s/🧪️tests/🏘️plugin-composition/🌀️procedural-formats`. The 8 other edges that touch the area stay with their owners: 7 production edges from `script.ts` bootstrap files of the area into the library (parent), 1 test edge into renderer `ShellHost` (`rhu3`).

Also done, because they were open and had a mechanical fix: the spatial-kernel session edges and the compute consumer edges (batches c2 and c2m). Left alone: the `💻️client/🧩️vscode/🏗️builder` files, which a peer was creating while I worked (untracked, created 20:24 to 20:26, 33 new family lines), and the retirement fixtures edges (`rt2`/`tfe` territory).

## 3. Batches

| Batch | Scope | Importer files / edits | Facades new | Exports new | Manifests with new dependencies | Verification |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| c1 | `sf3`: `@semio-tech/procedural-js` (foundation generation sqlite and schema snapshot, sqlite test support and fixture), `@semio-tech/procedural-generation3d` (snapshot schema, two mutation diffs), `@semio-tech/process` (`compiler-capacity`, an existing prepared facade, used by the generation3d bootstrap script by relative path to the facade) | 11 / 21 | 8 | 8 | 3 (`generation2d` ts, `generation3d` ts, `✏️s` root) | 9 test files identical (38 pass / 1 fail before and after) |
| c2 | new TS package `@semio-tech/s-spatial-kernel-semio-session-ts` at `🌊️session/📦️packages/🟦️typescript` (sibling of the Rust bindings), 1 facade for `🌊️session/🟦️.ts` | 3 / 3 | 1 | 1 | 2 (`s-spatial-kernel-ts`, `cad-ts`) | 10 test files identical (16 pass / 43 fail before and after) |
| c2m | compute consumer: `exports` added to the existing packages `@semio-tech/2d-compute` (`./testing/consumer`, `./testing/consumer/host`) and `@semio-tech/compute-ownership-runner` (`.`); the hub compute test now imports the two existing SDK entries `@semio-tech/framework/2d/compute/testing/consumer[/host]` | 1 / 2 | 0 | 3 | 1 (hub compute ownership) | same 10 test files; 6 entries load and resolve |
| c3 | the cross-scope gate (section 4) | 1 moved | 0 | 0 | 6 (hub root) | see section 5 |

Why c2m needs no importer rewrites: the three bootstrap scripts (`📜️script.ts`) of the `os`, `flow` and `hub` compute ownership packages import `execution/🟦️.ts` by relative path, and so do the plugin-area scripts and the SDK facades (`framework/📦️packages/🟦️typescript/◻️2d/🧮️compute/…`). Once the module is an authored export target, the cruiser accepts the relative path (the rule's `pathNot` lists export targets). This removed 12 edges (4 production, 8 test; 4 of them, 1 production and 3 test, in the `pi1` area) with one manifest line per package; the only importer edit is the hub compute test (2 lines, to the existing SDK entries).

Names and keys follow `psc2`: the subpath mirrors the target path without `🔨️modules/`, `🧪️tests` becomes `🧪️testing`, and a path over 236 bytes drops leading segments, which is why the two generation3d diff facades have different key prefixes (`standards/1/…` versus `1/…`). The longest new path is 234 bytes. The 30 new directories all resolve in the taxonomy (`frs-tools/taxcheck.ts`: checked 30, unresolved 0).

## 4. The cross-scope edges (item 2)

The test `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤝️hub-edit-durability/🟦️.ts` imported four hub modules by relative path (`🚀️local-bootstrap` execution and authentication, `🏗️bootstrap` publication and selection). The `psc2` finding stands: a specifier `os-hub-ts/…` cannot work, because the `✏️s` Bun scope does not list the hub packages, and it must not list them (the hub scope lists `../✏️s` as a workspace, so the reverse list would be a cycle between scope roots). A hub "public test-support entry" would have the same scope problem. So the test belongs where both worlds are composed: the hub composition root of the same plugin.

What I did (greenfield, no alias left behind):
- Moved the test to `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🤝️hub-edit-durability/🟦️.ts` and its fixture to `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧫️fixtures/🤝️hub-edit-durability/🔣️.json`. Inside the hub the four hub modules are intra-package relative imports (allowed); `issueLocalCredential` is now a static import (it was a dynamic import through a directory search for the hub root); the process routing import is `@semio-tech/process/routing`. The fixture path in the old file (`../../🧫️fixtures/…`) pointed at an empty directory since the taxonomy sweep moved fixtures under `🧪️tests/🧫️fixtures`; the moved file reads `../🧫️fixtures/…`, which exists.
- Moved the runnable target with it: `hub-edit-durability-check` is removed from the project `@semio-tech/s-gis-services-native` (`📋️project.json`, `📜️script.ts`) and added to `@semio-tech/gis-plugin` (`🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📋️project.json`, `📜️script.ts`, class `HubEditDurabilityCheckScript`, same policy: 1,800,000 ms, 128 MiB, 65,536 lines, same `dependsOn` on `@semio-tech/framework-os-mcp-rs:build`). The launch route keeps its name and now runs `bun repo run @semio-tech/gis-plugin:hub-edit-durability-check` in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`.
- Updated the three readers of the old path: the `mcp-composition` fixture (9 cases to 8) and its law (`cases.length` 9 to 8; the law also read its fixtures from `../..`, a stale path, now `..`), and the source-text law in `🌉️mcp/📡️stdio/🎫️session/🧪️tests` (two path lists).
- The test's workspace imports are listed as `workspace:*` devDependencies of `🌎️hub/package.json` (`framework`, `framework-os`, `framework-os-mcp`, `framework-replication`, `repo-test`; `process` was there), so `not-to-unlisted` does not rise.

Cruiser effect: the 4 `os-hub-ts` lines, the 4 `no-cross-technology-✏️s-to-🌎️hub` lines and the 1 `no-cross-technology-✏️s-to-🧰️framework` line of that file are gone. One line is added: the moved runner class imports `runRepositoryCommand` from the library by relative path (`plugins-framework-sdk-only` and `no-cross-package-relative-@semio-tech/repo-lib`), exactly as the services script did for its other gates; it joins the 1,596 library edges the parent rewrites with the prepared facades.

## 5. Verification

Cruiser, exact command (alarm 7200 s, `psc3-tools/cruise.sh`): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err`.

| Run | Window | Exit | Error lines | Modules / dependencies | Raw |
| --- | --- | ---: | ---: | --- | --- |
| before | 20:05 to 20:23 (18 min), one attempt | 154 | 8,858 | 18,428 / 51,600 | `psc3-before-err.txt` |
| after | 20:34 to 20:50 (16 min), one attempt | 147 | 8,851 | 18,459 / 51,753 | `psc3-after-err.txt` |

The baseline ran while I wrote batch c1 (20:09 to 20:10) and c2 (20:16). It still shows every edge I removed (all 16 `procedural-js` lines, the 3 session lines, the 12 compute lines, the 9 `hub-edit-durability` lines, the generation3d capacity edge), so the cruiser had read the pre-edit files. I checked this edge by edge.

| Rule (all / outside `🧪️tests`) | before | after |
| --- | ---: | ---: |
| `no-cross-package-relative-*` lines | 4,893 / 2,855 | 4,923 / 2,898 |
| `no-cross-package-relative-*` distinct edges | 3,978 / 2,404 | 4,019 / 2,443 |
| `not-to-unlisted` | 405 / 0 | 405 / 0 |
| `no-circular` | 584 / 480 | 568 / 487 |
| `ui-no-framework-packages` | 297 / 232 | 297 / 232 |
| `framework-no-products` | 176 / 117 | 176 / 117 |
| `plugins-framework-sdk-only` | 627 / 558 | 627 / 558 |
| `renderer-hosts-only-ui` | 391 / 217 | 371 / 198 |
| `no-core-path` | 902 / 554 | 902 / 554 |
| `no-cross-technology-✏️s-to-🌎️hub` | 4 / 0 | 0 / 0 |
| total error lines | 8,858 | 8,851 |

Per-package lines of the rules I touched (before to after): `procedural-js` 16 to 0, `procedural-generation3d` 4 to 0, `procedural-generation3d-rs` 4 to 0, `s-spatial-kernel-semio-session` 3 to 0, `2d-compute` 6 to 0, `compute-ownership-runner` 6 to 0, `os-hub-ts` 5 to 1.

Attribution (`errdiff.ts`, ASCII keys because of cruiser mojibake; `attrib.py` with `psc3-snap/attr/`):
- Removed lines whose importer is a file of mine: 42 (family 13 production and 21 test, `s-to-hub` 4, `hub-to-framework` 2, `s-to-framework` 1, `plugins-framework-sdk-only` 1). Another 9 family lines went because their target became an export (plugin-area bootstrap scripts of the compute packages).
- Added lines caused by me: 2, both the same edge (moved runner to the library, family and `plugins-framework-sdk-only`).
- Added family lines that are not mine: 88. Importers: 33 in `💻️client/🧩️vscode/🏗️builder` and its tests (untracked files created 20:24 to 20:26), 17 in renderer targets and tests (`rhu3`), 10 in `🧑‍💻dev/🪝️host-ports`, and the remaining 29 in plugin registry, shell schema and `🗄️plugin-module-store` files (`pi2` and peers).
- `no-circular`: 14 removed (the renderer test files `…/🧪️tests/<name>/🟦️.tsx`, `rhu3`), 2 members added: `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` (+5 production lines) and `…/🌱️birth/🧾️claim/🟦️.ts` (+2). Both files were modified by a peer at 20:21, during the run. The "outside tests" count rose by these 7 lines; none involves a file of mine. `cycle.ts` was not needed.
- `not-to-unlisted`: no set difference at all.

| Check | Result |
| --- | --- |
| `tsc --noEmit` over the whole repository (13,527 files). Before = the tree before my edits through the compiler host overlay of every snapshot (`overlay-tsc.mjs`, snapshots c1, c2, c2m and the c3 files); after = `tsc -p` on the final tree (`psc3-tsc-final-before.txt`, `psc3-tsc-final-after.txt`) | 1,925 diagnostics before, 1,925 after (`tscdiff.ts`: 4 removed, 4 added). The 8 are in peer files (renderer `🪆️embedded-mount` test, library `🏗️web-build` test, `🔗️AgentBridge` test, `🧑‍💻dev/🪝️host-ports`, vscode `🏗️custody` test). 0 diagnostics in any of the 21 touched or created TypeScript files, before and after; 0 mentioning `hub-edit-durability`. |
| `bun test` per file, 16 files (`runtests.ts`, `testdiff.ts`) | 15 identical (47 pass; fail set 43 identical). The moved gate: `bun test` fails with `Original parent Script process invocation required` at the old path before and at the new path after, because it needs the launcher environment; the failure comes after the module graph loaded, which proves all its imports resolve. |
| `bun test` of `🌉️mcp/📡️stdio/🎫️session/🧪️tests` (reads the moved paths) | 7 pass / 1 fail before (old file restored for a few seconds; hash of the test file verified after) and 7 pass / 1 fail after, the same failing test. A first after-run at 20:24 showed 5 pass / 3 fail while a peer's process module was mid-edit (missing export `requireAcceptedPhysicalCleanupWriterProofV1`); that peer state is gone. |
| `mcp-composition` law (`serviceMcpCompositionOwnership`) | not runnable at `HEAD`: it read `../../🧫️fixtures` (ENOENT). With the path fixed all 8 cases pass (source exists, removed source absent, project options, registration, `Bun.build`), then it stops at `recipe.lawPath` of the `🧷️untrusted-content` fixture, which names `🌉️mcp/🗿️artifact/🧫️fixtures/🧷️untrusted-content-law.json`, a file that does not exist (pre-existing, not mine). |
| New entries in a fresh bun process (`facade-load.ts`) | c1 8 of 8 (names checked), c2 1 of 1, c2m 6 of 6 (the six entries: 2d-compute x2, compute-ownership-runner, framework SDK x2, process routing); 0 failures |
| Real Vite plugin container (`vite-check.mjs`) | c1 8, c2 1, c2m 6: 0 failures |
| `resolve-check.ts` (bun resolution of every static import of the moved gate, the hub script and the hub compute test) | 43 of 43 |
| `spec-check.ts`: every specifier import of the touched packages resolves exactly to its export target | `procedural-js` 16 of 16, `procedural-generation3d` 4 of 4, `s-spatial-kernel-semio-session-ts` 3 of 3 |
| `facade-check.ts`, `unused.ts` | 9 facade statements, 0 bad; 9 created exports, 0 without an importer |
| `verify.ts` on the codemod snapshots | only the intended `workspaces` list change of `✏️s/package.json`; the c3 edits are hand edits with deliberate non-import changes |
| `depcheck.ts`: every added `workspace:*` dependency resolves in each Bun scope that installs the depending package | 12 additions, 0 unresolvable |
| `publishWorkspaceMembership(root, "check")` (the function behind `workspaces --check`, called directly) | "Bun source membership is current (6 selected scopes, 303 physical packages)", `missing: []`, `stale: []`, `orderChanged: false` |
| `lockcheck.ts` (lock bun derives offline from the manifests versus the checked-in lock) | all six scopes: `workspacesDiffer` 0, `packagesMissing` 0; 1 third-party row (`is-promise` in root, `readable-stream` in `✏️s`) differs, as in `psc2`, unrelated |

## 6. Locks, workspaces, links

- New package `@semio-tech/s-spatial-kernel-semio-session-ts` is listed in the `workspaces` of `✏️s`, `🌎️hub`, `♻️mit-bestand` and `🏢️semio-tech` (byte-sorted, `workspaces.ts`; the Rust sibling lives one level deeper in `🕸️bindings`, so the plan carried `noSibling`). Root and `🎓️teaching` do not install the depending packages.
- Locks: `lockgen.sh` for the six scopes (`bun install --lockfile-only --offline`, checked-in lock restored at once), then `lock.ts --write` for `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech` (5, 7, 5, 5 changed workspace entries, one new workspace and one new package row each). Root and `🎓️teaching` needed nothing. Backups: `🗑️generated/psc3-lock-backup/`.
- Links: `node_modules/@semio-tech/{procedural-js,procedural-generation3d,s-spatial-kernel-semio-session-ts}` (hand-made symlinks, untracked; `bun install` makes them).

## 7. Skipped areas and hand-off after this slice (AST scan `psc3-scan-2.json`, list `🗑️generated/psc3-skipped-edges-after.tsv`)

| Reason | Production | Test |
| --- | ---: | ---: |
| parent (`📚️library`) | 1,596 | 802 |
| `pi1` area (`pi2` running) | 396 | 351 |
| `rhu2` area (`rhu3` running) | 281 | 839 |
| open: UI internals | 145 | 9 |
| open: asset and JavaScript targets | 15 | 16 |
| open: other | 17 | 4 |

The 17 + 4 "other" edges: all 17 production lines are the new vscode builder files of a peer (they import `🏃️process` internals by relative path; `@semio-tech/process` already has facades for those modules from `psc2`); the 4 test lines are 3 retirement fixture edges (`rt2`/`tfe`, one of them in the `🧪️test` source-text law) and 1 `🗄️plugin-module-store` test (peer). The `sf3` hand-off that remains is 7 production edges of the area's `script.ts` files into the library, to be rewritten by the parent with `bun psc3-tools/rewrite.ts <plan> <scan> <policy> --area ✏️s/🔌️plugins/🌀️procedural/ --ignore-skip --write --snapshot <file>` and a fresh `WITH_SKIPPED=1` plan (`psc3-tools/fresh.sh <label>` then `prep.sh`). The facades they need already exist.

## 8. Owner decisions (not decided, not touched)

### 8.1 UI internals: 145 production and 9 test edges

Full list with line numbers: `🗑️generated/psc3-ui-edges.tsv` (154 rows). Importer and target modules, grouped (appendix below).

Cause: `🧰️framework/🔨️modules/🖱️ui` is carved into nested packages (`ui-react` at `🎯️targets/⚛️react`, `ui-styling`, `ui-locale`, `ui-viewport`). The ui core (`🧱️elements`, `🔨️modules`, `📱️device`, `🧬️contract`, `🎚️axes`, test fixtures) stays in the owned region of `@semio-tech/framework`, so everything that is outside the core imports it by relative path. A facade through `@semio-tech/framework` would add one `ui-no-framework-packages` violation per edge, so the codemod did not create one.

| Group | Edges | Importers | Targets |
| --- | ---: | --- | --- |
| A | 120 prod | `ui-react` barrel `🎯️targets/⚛️react/🟦️.tsx` | 68 element modules (`🧱️elements/*`, 76 edges), 31 module libraries (`🔨️modules/*`, 41 edges), `📱️device` (3) |
| B | 18 prod | `ui-react` runtime: `🪟️chrome` 10, `🌓️appearance` 6, `🌐️i18n` 2 | elements 9, modules 7, `📱️device` 2 |
| C | 1 prod, 1 test | `ui-locale` (`🌐️locale/🟦️.ts` and its test) | `🎚️axes/🔣️.json` (also read by the Rust crate and by the manifest module) |
| D | 6 prod, 2 test | `🎠️kernel/📤️return` (3), `🎭️actor/📮️shard-client` (3), 2 kernel tests | `🧬️contract/🧵️retained/*` (5 modules), test fixtures entry |
| E | 6 test | quiz tests (3 css), flow editor test (builder retirement fixture), node-graph test and flow host test (ui fixtures entry) | `🎨️styling/*.css` 3, ui fixtures 3 |

Proposal (recommended, in order; each step is mechanical with `psc3-tools`: carve plan as in `psc2` batches b5 to b7):
1. **U1 carve `🧬️contract` into `@semio-tech/ui-contract-ts`** (`🖱️ui/🧬️contract/📦️packages/🟦️typescript`, explicit exports for the 5 retained modules, the builder retirement fixture and the contract part of the fixtures). Measured with `whatif.ts` (`🗑️generated/psc3-whatif-ui-contract.txt`): 99 edges from outside would use the new specifiers (this is where group D, kernel and actor, get a small dependency instead of the whole ui), 16 edges from the contract to the framework root become flagged, and those 16 are edges that violate `ui-no-framework-packages` today. This fixes D's 6 production edges and E's builder retirement fixture edge (1 test).
2. **U2 carve the ui core into `@semio-tech/ui-core`** (exports for `🧱️elements`, `🔨️modules`, `📱️device`, `🎚️axes`, the ui fixtures entry; about 100 export keys, the surface the barrel already exposes). Measured (`🗑️generated/psc3-whatif-ui.txt`): 26 edges from outside the ui use the new specifiers, 50 core edges to the framework root become flagged (same set the `ui-no-framework-packages` rule counts today, so that rule stays equal). This fixes groups A, B and C (139 production and 1 test edge) and the 4 test edges that go through the ui fixtures entry (2 kernel tests, the node-graph test, the flow host test). The barrel then reads `export { Button } from "@semio-tech/ui-core/elements/button"`; the package root barrel is not to be used (it would pull in the whole package and create cycles).
3. Rejected alternatives: (a) move the React elements under `🎯️targets/⚛️react`: 86 files of the ui core import elements and modules by relative path, they would be the new cross-package edges; (b) dissolve `ui-react` into the framework region: removes the edges but removes the package and its consumers' specifier; (c) route through `@semio-tech/framework`: one new `ui-no-framework-packages` violation per edge.
4. CSS (E, 3 test edges): `@semio-tech/ui-styling` already exports `./🎨️palette.css` and `./🖌️ui.css`; re-point the three imports to those keys (see 8.2, same mechanism).

The decision needed is U1 and U2 as stated (two packages versus one, and the package names). The tool support is there; the cost is the export surface of about 105 keys.

### 8.2 Assets and JavaScript targets: 15 production and 16 test edges

A TypeScript facade cannot re-export `?raw`, `?url`, `?inline`, fonts, images or JavaScript worker hosts. The owner has two choices per row: an explicit `exports` entry for the file in the package that owns it (the importer then uses `@semio-tech/<package>/<key>` with the same query suffix; `ui-styling` and `quiz-react` already do this for CSS), or moving the file next to its only user. Proposals:

| Group | Edges | Importer to target | Proposal |
| --- | ---: | --- | --- |
| DSL examples | 8 prod | `✏️s/🧑‍💻dev/🏗️fem` and `…/🧱️block/📖️stories/🧭️coordination` to `…/🖼️assets/…/🗣️.dsl.semio?raw` (fem 2d and 3d demo; block 2d left and right, 3d left and nakagin, 5d left and nakagin) | add `exports` such as `"./examples/<name>/dsl": "./…/🗣️.dsl.semio"` to the fem and block artifact TS packages; import `@semio-tech/<pkg>/examples/<name>/dsl?raw`. Check the query suffix with `vite-check.mjs` first (one run). |
| Images and model | 3 prod | puzzle story `🎭️3d-world` to two floor-plan images of `♾️infinite/🖼️assets` (`.jpg`, `.png`, `?url`); block story to `capsule_J.glb?url` in `🖼️assets` | `@semio-tech/assets` (9 exports) gets `./metabolism/capsule-j.glb`; the two floor plans: export from the infinite package, or move them next to the story if it is their only user |
| Fonts and cursor | 5 test | quiz `🕷️radar-geometry` to four `anta` `outline.ttf?inline`; quiz `📡️presence-client` to `☀️light.svg?raw` | `@semio-tech/assets` exports `./fonts/anta/<subset>.ttf` (4) and `./cursor/default/light.svg` |
| CSS | 7 test | quiz tests to `🎯️targets/⚛️react/🎨️.css?raw` | already exported as `@semio-tech/quiz-react/🎨️.css`; re-point the 7 imports (mechanical) |
| JavaScript worker hosts | 4 prod | flow `🫀️core/📦️packages/🦀️rust/📜️script.ts` dynamic imports of `🕸️wasm/🧪️tests/{🖥️host,🎬️draw-list,⏱️consumed-browser-clock,🧬️schema-oracle}/🟨️.js` | short term: `exports` entries on the flow wasm package for the four files; long term: convert the hosts to `🟦️.ts` so they get ordinary facades |
| `.mjs` helpers | 2 test | `🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs` (puzzle browser-contribution test), `🧪️test/🕸️dependencies/🟨️.mjs` (hub stdio oracle test) | `exports` on the owning packages (dev engine, `repo-test`) |
| JSON | 2 test | stdio ifc and gltf tests to `📇️registry/…/🔣️.json` (cohort fixture, registry schema) | the owner `✏️s/🔌️plugins/🗄️stdio/📇️registry` has no TS package; tool-supported (JSON default imports become named imports) once a package `stdio-registry-ts` is accepted |

Full rows with line numbers: `🗑️generated/psc3-skipped-edges-after.tsv` (reason "open: unsupported asset or JavaScript target"); locale axes JSON is in 8.1 group C.

### Appendix: the 154 UI edges by importer (kind, importer, edge count, target modules)

- `prod` `UI/🎯️targets/⚛️react/🟦️.tsx` (120): `🗄️storage-port`, `🐳️dock-skeleton`, `🔎️fuzzy-ranking`, `📱️device/🟦️.ts`, `🔌️Ports`, `🔌️Ports`, `🐚️ShellScope`, `🏷️class-name-composition`, `🌀️status-border-presentation`, `🖱️interaction-presentation`, `📝️form-control-presentation`, `📏️border-presentation`, `🌈️surface-presentation`, `📋️menu-item-presentation`, `🏠️shell-floor-presentation`, `🎛️chrome-control-presentation`, `🆔️element-identity`, `🐳️dock-skeleton`, `🔣️Icons`, `🖱️ContextMenu`, `🚗️UiDriver`, `🔤️keybinding-text-interpretation`, `💡️control-tooltip-presentation`, `🕹️control-keybinding-context`, `⌨️control-hotkey-presentation`, `💾️keybinding-persistence`, `📱️device/🟦️.ts`, `📱️device/🟦️.ts`, `🔲️WindowSilhouette`, `💡️ChromeControlHint`, `🎗️UiLabel`, `📚️I18n`, `📨️UIDialog`, `🌈️Surface`, `📋️MenuItem`, `🧱️DragHandle`, `🧭️PanelTabBar`, `🧭️flow-direction-context`, `🗂️WindowChrome`, `🃏️OverviewCard`, `🥞️LayeredOverview`, `🥞️layered-overview-geometry`, `🚧️WindowContentDeadLine`, `🆔️element-identity`, `🆔️ElementId`, `⌨️Command`, `🔚️Footer`, `📐️Layout`, `🗨️Popover`, `🏷️Label`, `🏷️Label`, `📻️TableAvatar`, `👥️PresenceBar`, `👥️presence-presentation`, `🦴️Skeletons`, `📝️Field`, `⚡️ActionGroup`, `🔳️ButtonGroup`, `🔘️Button`, `🧾️Form`, `☑️Checkbox`, `✏️Input`, `🔽️Select`, `🎚️Slider`, `🪜️Stepper`, `🔤️Textarea`, `🔀️Toggle`, `🎛️ToggleGroup`, `⭕️Ring`, `↕️Collapsible`, `💬️Dialog`, `↔️Resizable`, `📜️Scrollable`, `🔝️Navbar`, `🧪️NavbarExampleSelect`, `📑️Tabs`, `🎴️IconSelector`, `🌳️Tree`, `🌳️Tree/🧭️hover-path`, `🌳️Tree/📏️row-alignment`, `🖼️Panel`, `🖼️Panel/🌲️tree-source`, `🖼️Panel`, `🎀️Ribbon`, `🪟️Window`, `🕸️Diagram`, `🎬️Scene`, `📊️Table`, `🕰️HistoryTable`, `⚙️VirtualFileSystem`, `🎨️Canvas`, `👻️PanelGhost`, `🗣️ui-terminology`, `📱️ui-mobile-context`, `🎨️chrome-class-names`, `⚓️panel-anchor`, `🧲️panel-dock`, `🛟️chrome-safe-area`, `📐️window-measures`, `🖱️ContextMenu/🥣️chrome`, `🔍️WindowSearch`, `🗺️window-layout`, `🎮️Engagement`, `📌️WindowPane`, `📌️WindowPane`, `🎮️Engagement`, `🗺️window-layout`, `🔍️WindowSearch`, `🖱️ContextMenu/🥣️chrome`, `📐️window-measures`, `🛟️chrome-safe-area`, `🧲️panel-dock`, `⚓️panel-anchor`, `🎨️chrome-class-names`, `🎬️Scene/📐️geometry`, `📱️ui-mobile-context`, `🗣️ui-terminology`, `👻️PanelGhost`, `🧬️style-variants`, `🧬️style-variants`
- `prod` `UI/🎯️targets/⚛️react/🪟️chrome/🟦️.ts` (10): `🏷️class-name-composition`, `🗂️WindowChrome`, `🃏️OverviewCard`, `🥞️LayeredOverview`, `🥞️layered-overview-geometry`, `🔝️Navbar`, `🔣️Icons`, `🚗️UiDriver`, `📱️device/🟦️.ts`, `👥️presence-presentation`
- `prod` `UI/🎯️targets/⚛️react/🌓️appearance/🟦️.ts` (6): `🗄️storage-port`, `📱️device/🟦️.ts`, `🔌️Ports`, `🚗️UiDriver`, `👂️dom-event-binding`, `📝️form-control-presentation`
- `prod` `F/M/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts` (3): `🧬️contract/🧵️retained/🏘️instance`, `🧬️contract/🧵️retained/🩹️operations`, `🧬️contract/🧵️retained/💾️resident`
- `prod` `F/M/🎭️actor/📮️shard-client/🟦️.ts` (3): `🧬️contract/🧵️retained/🏘️instance`, `🧬️contract/🧵️retained/💾️resident`, `🧬️contract/🧵️retained/💾️resident`
- `prod` `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts` (2): `🗄️storage-port`, `📚️I18n`
- `prod` `UI/🌐️locale/🟦️.ts` (1): `🎚️axes/🔣️.json`
- `test` `F/P/❓️quiz/🧪️tests/🌗️contrast-states/🟦️.tsx` (2): `🎨️styling/🎨️palette/🎨️.css`, `🎨️styling/🖌️ui/🎨️.css`
- `test` `F/M/🎠️kernel/🧪️tests/🏷️localized-label-fixture/🟦️.ts` (1): `🧪️tests/🧫️fixtures/🟦️.ts`
- `test` `F/M/🎠️kernel/🧪️tests/📦️package-assets/🟦️.ts` (1): `🧪️tests/🧫️fixtures/🟦️.ts`
- `test` `F/M/🗺️surface/🕸️node-graph/📡️scene/🧪️tests/🟦️.ts` (1): `🧪️tests/🧫️fixtures/🟦️.ts`
- `test` `F/P/❓️quiz/🧪️tests/💭️crowd-client/🟦️.tsx` (1): `🎨️styling/🎨️palette/🎨️.css`
- `test` `F/P/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🟦️.ts` (1): `🧪️tests/🧫️fixtures/🟦️.ts`
- `test` `UI/🌐️locale/🧪️tests/🟦️.ts` (1): `🎚️axes/🔣️.json`
- `test` `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts` (1): `🧬️contract/🏗️builder/♻️retirement`

## 9. Foreign findings (not mine, not fixed unless stated)

- `🌉️mcp-composition` law: stale path fixed (section 5); it still stops at the missing `untrusted-content-law.json` named by the recipe fixture.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🎚️config/🔣️.json` names the project `@semio-tech/framework-os-mcp-rs` for all 12 MCP checks including `hub-edit-durability-check`, but those targets live in `@semio-tech/s-gis-services-native` (and now `@semio-tech/gis-plugin`). Pre-existing; the acceptance owner decides the project names.
- A peer's process module was transiently broken at 20:24 (`requireAcceptedPhysicalCleanupWriterProofV1` not exported) and was fine again at 20:55. Two peer files at 20:21 added the process cycle members of section 5.
- Empty leftover directories from the taxonomy sweep: `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧫️fixtures/{🌉️mcp-composition,🔌️service-composition,🤝️hub-edit-durability,🧷️untrusted-content}` (untracked, not touched).
- `.vscode/launch.json` shows a very large diff against `HEAD` from peers; my edit is one line in each of the two launch files.

## 10. Commands run (real results)

| # | Command | Result |
| --- | --- | --- |
| 1 | `psc3-tools/cruise.sh before full` | exit 154, 8,858 error lines, 18 min |
| 2 | `psc3-tools/fresh.sh 1` and `fresh.sh 2` (`dump-policy.cjs`, `scan.ts`) | 302 and 303 packages; 4,488 and 4,471 edges (2,445 and 2,450 production) |
| 3 | `prep.sh 1 c1 <3 owners>`, `apply.ts --write`, `rewrite.ts --write`, `link.ts --write`, `verify.ts`, `facade-check.ts`, `facade-load.ts`, `vite-check.mjs`, `depcheck.ts` | c1: 8 facades, 21 edits in 11 files; all checks 0 bad |
| 4 | `plan.ts … --ts-package`, `apply.ts`, `rewrite.ts`, `workspaces.ts --write` for c2 | 1 package, 1 facade, 3 edits; 4 workspace lists |
| 5 | hand edits c2m, c3 with re-read before each edit (`python3` with `assert count == 1`) | section 3 and 4 |
| 6 | `runtests.ts` before and after, 16 files; `bun test` of the session test file before (old file restored) and after | section 5 |
| 7 | `tsc.sh final-after` and `overlay-tsc.mjs` (final-before), `tscdiff.ts` | 1,925 and 1,925 diagnostics, 4 removed, 4 added, 0 in touched files |
| 8 | `psc3-tools/cruise.sh after full` | exit 147, 8,851 error lines, 16 min |
| 9 | `ruletab.ts`, `errdiff.ts`, `attrib.py`, the ASCII-set scripts for `no-circular` and `not-to-unlisted` | section 5 |
| 10 | `lockgen.sh` six scopes, `lock.ts --write` four scopes, `lockcheck.ts` | section 6 |
| 11 | `publishWorkspaceMembership(root, "check")` through `bun -e` | current, 303 packages |
| 12 | `frs-tools/taxcheck.ts` on the 30 new directories; byte-length scan of new paths | 0 unresolved; longest 234 bytes |
| 13 | `whatif.ts` for `🖱️ui` and `🖱️ui/🧬️contract` | 50/26 and 16/99 |
| 14 | not run: `bun repo run …:workspaces-check` (launcher), `verify taxonomy`, `verify layering`, `bun install`, the moved gate itself | see section 1 |

## 11. Files changed

- Created: 8 facade files and 5+3 `exports` keys in `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/**` and `…/🧊️generation3d/📦️packages/🟦️typescript/**`; the package `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🟦️typescript/{package.json,🟦️.ts}`; `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🤝️hub-edit-durability/🟦️.ts` and `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧫️fixtures/🤝️hub-edit-durability/🔣️.json`.
- Importers rewritten (14 lines of specifier edits in c1 files and 3 in c2 files): the 11 c1 files listed in `psc3-snap/c1-rewrite.json`, the 3 c2 files in `psc3-snap/c2-rewrite.json`, `🌎️hub/🧩️compositions/🪐️space/🏛️ownership/🧮️compute/🧪️tests/🟦️.ts`.
- Manifests: `exports` on `procedural-js`, `procedural-generation3d`, `2d-compute`, `compute-ownership-runner`; dependencies on `generation2d` ts, `generation3d` ts, `✏️s/package.json`, `s-spatial-kernel-ts`, `cad-ts`, `🌎️hub/🧩️compositions/🪐️space/🏛️ownership/🧮️compute/package.json`, `🌎️hub/package.json`; `workspaces` of `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`; four `bun.lock` files.
- Moved and edited for c3: the files of section 4 (`🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/{📋️project.json,📜️script.ts}`, `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust/{📋️project.json,📜️script.ts}`, `…/💡️services/🧪️tests/🌉️mcp-composition/🟦️.ts`, `…/🧫️fixtures/🌉️mcp-composition/🔣️.json`, `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🧪️tests/🟦️.ts`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`); deleted: `…/💡️services/🧪️tests/🤝️hub-edit-durability/🟦️.ts` and `…/💡️services/🧪️tests/🧫️fixtures/🤝️hub-edit-durability/🔣️.json` (original saved in `🗑️generated/psc3-snap/c3-orig/`).
- Untracked environment: the three `node_modules/@semio-tech/*` links.
- Not touched: `AGENTS.md`, the policy, taxonomy and rule generator files, any ticket tool, any git state. No `[DEBUG]` log was written to the repository.
