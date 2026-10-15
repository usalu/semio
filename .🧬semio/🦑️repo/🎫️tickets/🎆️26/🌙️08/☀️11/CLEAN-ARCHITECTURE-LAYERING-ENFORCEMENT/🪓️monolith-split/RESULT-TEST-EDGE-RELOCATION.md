# Result: Test Edge Relocation (work packages 2 and 6)

Executor: Sonnet, foreground, no sub-agents. Date of run: 2026-10-15. No modifying git command was used (files were created, edited, moved with `mv` and removed with `rm`; the index and HEAD were not touched by me).

Path aliases used below (all repo-relative):

- `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `G3` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `TXT` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot`
- `DRAW` = `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `ARCH` = `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any`

## Outcome in one table

| # | Edge source (framework side) | Decision | Result |
|---|---|---|---|
| 1 | `OS/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🟦️.ts` (2 edges, 1 test case) | (1) move the test case into the owning plugin | done |
| 2 | `OS/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx` (5 edges, 3 test blocks) | (2) framework-owned neutral fixture, plugin-behaviour assertions dropped (already covered by the plugin) | done |
| 3 | `OS/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (4 edges, 1 test case) | (1) the plugin already owns the same test; delete the duplicate, port its one missing assertion | done |
| 4 | `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx` (3 edges, 1 test case) | (2) framework-owned neutral fixture | done |
| 5 | `OS/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts` (2 edges, 2 test cases) | (1) move both TXT-specific cases and their TXT data into the TXT plugin | done |
| 6 | `OS/🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx` (1 edge) | (3) move the harness page into the `✏️s/🧑‍💻dev` integration root that already owns `composeSpecificOsCatalogV1` | done, harness boot not runnable here (see notes) |
| 7 (WP6) | `✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🟦️.ts` -> energy EPW extension test | drop the redundant case; the EPW check is already a twin of the energy-model package script | clear in the tree (a peer removed the import first, see notes) |
| 8 (WP6) | `✏️s/🔌️plugins/🏛️architect/🧪️tests/🧪️graph-tool/🟦️.ts` -> architect artifact fixture | move the test next to its fixture, inside the artifact | done |

Dependency-cruiser (`--config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err`):

- The 6 framework test files had 94 error edges into `✏️s` before (17 `framework-no-s`, 17 `framework-no-implementation`, 16 `framework-no-plugin-packages`, 17 `no-cross-technology-🧰️framework-to-✏️s`, 7 `no-cross-package-relative-@semio-tech/draw-drawing-rs`, 7 `…/procedural-generation3d`, 7 `…/procedural-generation3d-rs`, 2 `…/stdio-txt`, 1 `…/semio-s-workspace`, 3 `renderer-hosts-only-ui` that targeted `✏️s`). After: **0**. Total errors originating in those 6 files: 206 before, 112 after; the remaining 112 are unrelated framework-internal relative-import and unlisted-dependency categories that this package does not own.
- R3: `plugin-no-extension-or-artifact-🏛️architect` (graph-tool) 1 -> 0; fixture-placement -> energy EPW (`plugin-no-extension-or-artifact-🧪️tests`, `no-cross-plugin-🧪️tests-to-🔋️energy`, `no-cross-package-relative-@semio-tech/energy-js`) 3 -> 0.
- New edges I introduced (all `no-cross-technology-✏️s-to-🧰️framework`, caused by package-name imports that the cruiser resolves to the workspace symlink target): multi-shell harness 3 (`@semio-tech/framework-os-dev`, `@semio-tech/framework-renderer-react`, `@semio-tech/framework`), TXT typed-file test 1 (`@semio-tech/framework`). See "Finding" in the notes. The graph-tool test keeps its three pre-existing framework edges (same imports, new location); the R3 rule is the one that disappeared.

Raw logs (kept, prefix `ter-`): `🪓️monolith-split/🗑️generated/ter-base-*.log`, `ter-after-*.log`, `ter-after-cruiser.txt`.

## Per edge

### 1. evaluation-source read (WP2)

- Used from the plugin: `G3/🧵️preview-eval/📥️tessellation/🧬️schema/🔣️.json` and the input rows, only by the test case "original command reply prevalidation retains malformed input and native fault before routing". That case validates plugin data against the plugin's own schema with Ajv and JSON Patch; it has no framework behaviour in it.
- Changed:
  - removed that case from `OS/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🟦️.ts` (-7 lines);
  - new `G3/🧵️preview-eval/📥️tessellation/🧪️tests/🟦️.ts` (the same case, static imports);
  - fixture moved `G3/🧵️preview-eval/📥️tessellation/🧫️fixtures/📥️input.json` -> `G3/🧵️preview-eval/📥️tessellation/🧪️tests/🧫️fixtures/📥️input.json`, because the fixture-placement law requires fixtures under a `🧪️tests` owner (this also cleared two fixture-placement failures, see edge 7);
  - Rust include re-pointed in `G3/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (one `include_str!` path, existence of the target verified, not compiled);
  - registered the new suite in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🟦️typescript/📜️script.ts` (`suites`, which are type-checked strictly and run with `bun test`).
- Tests: `bun test` on the framework file 12 pass -> 11 pass; new plugin file 1 pass (36 expects); strict `tsc` (the suite flags) on the new file clean.
- Cruiser: 2 edges per rule (framework-no-s, -implementation, -plugin-packages, no-cross-technology) -> 0; new file has 0 errors.

### 2. editable-controls (WP2)

- Wrong edge: the three blocks at the end of the file (scalar commits, ordered collection commands, metadata and status) drove the real `UiNodeView` from the Generation3d inspector fixtures and then asserted the plugin's `editInputValue`, `editCollectionValue` and `editWidgetFacet`.
- Why not move: the plugin cannot import the renderer (`plugins-framework-sdk-only`), and `UiNodeView` is not on any public package export, so a cross-layer test has no legal home without widening two public APIs. The plugin-behaviour half is already tested by `G3/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🟦️.ts` (Ajv + JSDOM + the same edit functions; re-run: `generation3dWidgetInputSelfTests` via `…/🎯️selection/🧪️tests/🔬️unit/🟦️.ts`, 1 pass).
- Changed:
  - `OS/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx`: removed the 5 plugin imports and `Ajv`, replaced the three blocks by three framework-owned blocks driven by the new fixture and a shared `mountCommitNode` helper (8 tests, same count as before: 2 locales x 2 gestures, 2, 2). The plugin edit-function assertions are replaced by the expected `intents[0].input`.
  - new `OS/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/♿️editable-controls/🎛️commit-gestures.json` (neutral controls: number, long text, text, toggle, collection set/add/remove/move, text facet, select facet, three status diagnostics, English and German labels).
  - dead data removed from `G3/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️.json`: `reactCommits` and `reactMetadata` (only consumer was the removed blocks; a repo-wide `git grep` finds no other reader).
  - `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: entry `⚖️test-generation3d-inspector-controls♿️react🟦️` renamed `⚖️test-inspector-commit-controls♿️react🟦️` (same position, same group) and its `--testNamePattern` updated to `retains committed scalar control|retains ordered collection commands`.
- Tests (vitest through the renderer-react config, `SEMIO_TEST_LEVEL=long`): before **the suite did not load** (0 tests; Vite: `Failed to resolve import "@semio-tech/framework/mesh/polygon"` from the plugin file `G3/✏️editor/🎮️commands/🎚️set-widget-input/🟦️.ts`, i.e. the cross-layer import was already breaking the framework suite); after **112 pass** (the file's 8 inspector tests plus the rest of the file). `tsc` on the file with the renderer-react compiler options: 0 errors from my code; 1 pre-existing `TS7016` for `dom-accessibility-api` on line 4.
- Cruiser: 5 edges per rule -> 0.
- Coverage note: the end-to-end "real plugin data through the real interpreter" check is gone; the plugin keeps its own JSDOM twin and the framework keeps the interpreter law on neutral data.

### 3. engine-contract (WP2)

- Edge: four drawing fixtures/schemas imported for the single case "Draw utility action and interruption fixtures conform to their closed contracts".
- That case is an Ajv-only validation of Draw data against Draw schemas. `DRAW/✏️editor/🪛️utilities/🎬️actions/🧪️tests/🟦️.ts` already tests the same fixtures with the same assertions (and adds a SQLite count oracle).
- Changed:
  - `OS/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`: removed the 4 imports and the case (-23 lines);
  - `DRAW/✏️editor/🪛️utilities/🎬️actions/🧪️tests/🟦️.ts`: added the only assertion the duplicate had and the plugin test lacked (an unknown utility id is not in the policy);
  - `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: entry `🧪️test🖍️draw🧰️actions⚛️react` pattern no longer names the removed case (`neutral utility declarations keep mounted palette commands executable` stays). The plugin test already has the launch entry `▶️utility-action-policy🖍️draw🟦️`.
- Tests: framework file `-t "neutral utility declarations…|Draw utility action…"` before **3 pass** (copy of HEAD run beside it, then deleted), after **2 pass**; plugin `bun test` 1 pass (22 expects) -> 1 pass (23 expects).
- Cruiser: 4 edges per rule -> 0.

### 4. staged-arg-controls (WP2)

- Edge: the case "renders drawing mutation layer choices in en/de and refuses unsupported values" ran the framework's `mutationInputDefs`, `argControl` and `renderStagedArgControl` on the Draw create-layer schema.
- Changed:
  - `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx`: 3 plugin imports replaced by one fixture import; case renamed "renders staged mutation choices in {locale} and refuses unsupported values"; same assertions (select kind, options, accessible name, valid values accepted, invalid refused). I did not touch `🛠️ShellHelpers/🟦️.tsx`.
  - new `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🎛️staged-arg-choices/🔣️.json`: a neutral document schema, a neutral `createItem` mutation schema with two enum/select fields (`kind`, `mode`) carrying `x-semio-ui` option labels in English and German, and the law rows.
- Tests (vitest, long level): before **18 pass, 2 fail**. The two failures were the Draw case itself (the Draw document schema now requires `fontFamily` for kind `text`, so the case rejected a value it expected to accept; the German run then failed on a leaked combobox from the English failure). After **20 pass, 0 fail**. `tsc` on the file: 0 errors.
- Cruiser: 3 edges per rule -> 0.

### 5. typed-snapshot (WP2)

- Edge: two TXT-specific cases. "complete typed TXT snapshots preserve all legal fields in independently readable SQLite files" used the TXT plugin's converters and schema. "registered full TXT file laws borrow genuine Store reads and retire accepted complete output" scanned the plugin's Rust test file. The fixture also carried TXT-only data (`dialect` `s.stdio.txt`, `encoding`, `cases`, `collisions`).
- Changed:
  - `OS/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts`: both cases and the now-unused imports/consts removed (-29 lines); added `expect(admit(fixture.closeGrant))` to the first grant-refusal case so the close-grant schema admission, which only the moved case checked, stays covered in the framework;
  - `OS/🚪️io/📸️typed-snapshot/🧪️tests/🧫️fixtures/🔣️.json`: `dialect`, `encoding`, `cases`, `collisions` removed (verified unused by the framework's TS and Rust tests; every remaining key is still read);
  - new `TXT/🧪️tests/🗂️typed-file/🟦️.ts` (both cases; Ajv instead of the validator oracle, so the only framework import is `@semio-tech/framework`, which already exports `SqliteOperation`, `exportSqliteDatabase`, `importSqliteDatabase`) and new `TXT/🧪️tests/🧫️fixtures/🗂️typed-file.json` (the four moved keys);
  - `TXT/🧪️tests/🦀️.rs`: `registered_full_fixture` now includes the plugin fixture instead of reaching into the framework fixture (it reads only `cases` and `dialect`; path existence verified, **not compiled**);
  - registered in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/📜️script.ts` (`snapshotSqliteTests`, which `test-snapshot-sqlite source` runs with `bun test`).
- Tests: framework file before **10 pass** (234 expects) -> after **8 pass** (118); new plugin file **2 pass** (115); total 10 tests, 233 expects (two plugin grant-admit expects dropped, one framework close-grant expect added). Plugin group (`🧪️tests/🟦️.ts` + new file) 15 pass, 0 fail. Strict `tsc` (the `test-typed-snapshot-file-source` flags) on both files: clean.
- Cruiser: 2 edges per rule -> 0 (`no-cross-package-relative-@semio-tech/stdio-txt` 2 -> 0).

### 6. multi-shell-harness (WP2)

- This is a dev entry page, not an assertion test: it mounts several `FrameworkOsShell` instances from the specific OS catalog (`composeSpecificOsCatalogV1`). It belongs with the dev entries in `✏️s/🧑‍💻dev`.
- Changed:
  - moved `OS/🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx` -> `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧪️multi-shell-harness/🟦️.tsx`; the relative CSS import became `import "@semio-tech/framework-os-dev";` (that package root imports the dev CSS and has no other import-time side effect), the catalog import became `../../🟦️.ts`;
  - moved `OS/🧑‍💻dev/🧪️tests/🧫️fixtures/🧪️multi-shell-harness/🌐️.html` -> `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧫️fixtures/🧪️multi-shell-harness/🌐️.html` and fixed its script `src` (it pointed at `./🟦️.tsx` next to itself, which never existed);
  - `OS/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts`: removed the dead export `FRAMEWORK_OS_MULTI_HARNESS_PORT` and its docstring, which named the harness path (no reader anywhere; `dev multi` is not registered in `launch.json`);
  - `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧫️fixtures/🌐️favicon-delivery.json`: dropped the harness HTML from `documents` (it listed a path that did not exist, so that case could not read it; a framework UI fixture should not name an `✏️s` page).
- Tests: the harness has no runner (no launch entry, no script, orphaned port constant), so no before/after count; **its boot was not exercised**. The favicon case that read the stale path: before it pointed at a file absent from HEAD (`git cat-file -e` fails), after `bun test … -t "favicon delivery"` is **1 pass** (70 expects).
- Cruiser: 1 edge per rule -> 0 in the framework tree.

### 7. fixture-placement -> EPW (WP6, R3)

- Wrong edge: the meta-test "plugin fixture placement" imported `runWeatherChecks` from the energy EPW extension's test and re-ran it. That check is already executed as the `epw-weather` twin by `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📜️script.ts`; I re-ran it directly: `runWeatherChecks()` returns 7 checks.
- When I came to edit, the import and the case were already gone from the working tree (a peer removed them, together with the `runOriginalCopyCloseChecks` stdio case, between my reads; HEAD still has both). I did not edit this file; the decision (drop the redundant case) matches the tree.
- Related fix I did make for this meta-test: relocating the tessellation fixture under `🧪️tests` (edge 1) removed two of its failures that my interim state had triggered.
- Tests (`bun test`, HEAD copy next to the file for "before", deleted afterwards): before 41 tests, 36 pass, 5 fail; after 39 tests, 35 pass, 4 fail. The 4 remaining failures are pre-existing placement violations in the trinity jack query test (fixture under a `🧬️schema` path) and the puzzle transform-tool Cargo manifest reader, none touch my files. (The "before" run already contained my tessellation move, so two of its five failures were caused by that interim state, not pre-existing.)
- Cruiser (from the audit log `gate-tsforbidden-depcruise.txt` for "before"): 3 R3-related hits -> 0; the file keeps one `not-to-unlisted` (`web-tree-sitter`), unchanged.

### 8. architect graph-tool -> artifact fixture (WP6, R3)

- Wrong edge: a plugin-level test read the fixture of the plugin's own program artifact.
- Changed: moved `✏️s/🔌️plugins/🏛️architect/🧪️tests/🧪️graph-tool/🟦️.ts` -> `ARCH/🧪️tests/🧪️graph-tool/🟦️.ts` (beside `🧫️fixtures/🛠️graph-tool/🔣️.json`; the imports were adjusted: fixture is now `../🧫️fixtures/🛠️graph-tool/🔣️.json`, the framework schema path is eleven levels up); the empty plugin-level `🧪️tests` folder removed; path updated in `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts` (the package's `test` list).
- Tests: `bun test` 9 pass (15 expects) before and after; the package's three-file group 11 pass, 0 fail.
- Cruiser: `plugin-no-extension-or-artifact-🏛️architect` 1 -> 0. Its three framework-schema edges (`plugins-framework-sdk-only`, `no-cross-technology-✏️s-to-🧰️framework`, `no-cross-package-relative-@semio-tech/framework-tool-machine-rs`) are unchanged: they belong to WP7/8 (SDK barrel).

## Files

Created: `G3/🧵️preview-eval/📥️tessellation/🧪️tests/🟦️.ts`; `G3/🧵️preview-eval/📥️tessellation/🧪️tests/🧫️fixtures/📥️input.json` (moved); `TXT/🧪️tests/🗂️typed-file/🟦️.ts`; `TXT/🧪️tests/🧫️fixtures/🗂️typed-file.json`; `OS/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/♿️editable-controls/🎛️commit-gestures.json`; `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🎛️staged-arg-choices/🔣️.json`; `ARCH/🧪️tests/🧪️graph-tool/🟦️.ts` (moved); `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧪️multi-shell-harness/🟦️.tsx` (moved); `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧫️fixtures/🧪️multi-shell-harness/🌐️.html` (moved).

Edited: `OS/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🟦️.ts`; `OS/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx`; `OS/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`; `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx`; `OS/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts`; `OS/🚪️io/📸️typed-snapshot/🧪️tests/🧫️fixtures/🔣️.json`; `OS/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts`; `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧫️fixtures/🌐️favicon-delivery.json`; `G3/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️.json`; `G3/✏️editor/🧪️tests/🔬️unit/🦀️.rs`; `DRAW/✏️editor/🪛️utilities/🎬️actions/🧪️tests/🟦️.ts`; `TXT/🧪️tests/🦀️.rs`; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/📜️script.ts`; `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🟦️typescript/📜️script.ts`; `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts`; `.vscode/launch.json`; `.vscode/🧩️launch.seed.jsonc`.

Removed (moved away): `OS/🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx`, `OS/🧑‍💻dev/🧪️tests/🧫️fixtures/🧪️multi-shell-harness/🌐️.html`, `G3/🧵️preview-eval/📥️tessellation/🧫️fixtures/📥️input.json`, `✏️s/🔌️plugins/🏛️architect/🧪️tests/🧪️graph-tool/🟦️.ts` (and the empty `🧪️tests` folder).

Not touched: `🛠️ShellHelpers/🟦️.tsx`, `🗑️generated` folders (my `ter-*` logs stay in `🪓️monolith-split/🗑️generated`).

## Commands run

- Cruiser: before `bun`-free `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs` on the 6 framework files (28 s), after on the 11 touched paths (about 12 min under load average 40); both with the policy config above and `--output-type err`.
- `bun test ./<file>` (needs the `./` prefix with emoji paths) for the read, typed-snapshot, draw actions, graph-tool, tessellation, TXT, architect, fixture-placement, favicon and selection-unit files.
- vitest through a thin config in the session scratchpad that imports the renderer-react config and only overrides `include`; run with `SEMIO_VITEST_POLICY` and `SEMIO_TEST_LEVEL=long`, from the renderer-react package directory.
- `tsc` with the exact flags of the owning runners (artifact `suites` flags; `test-typed-snapshot-file-source` flags; renderer-react compiler options through a temporary tsconfig that was deleted).
- `bun test` for `🔏️path-emoji-statutes` and `🔤️taxonomy-leading-grapheme` (taxonomy sanity): 38 pass, 9 fail, none of the failures mention a path I created or moved (peer in-flight state).

## Notes, findings and what is not verified

- **Not compiled:** the two Rust `include_str!` path edits (`TXT/🧪️tests/🦀️.rs`, `G3/✏️editor/🧪️tests/🔬️unit/🦀️.rs`). I checked that both targets exist and that the keys the Rust code reads (`cases`, `dialect`) are present in the new plugin fixture; no cargo check was run (load average about 40, build budgets).
- **Not run:** the multi-shell harness page (no runner exists), the full `bun repo run …:test` target of the renderer package (I ran vitest directly on the changed files).
- **Finding (rule inconsistency for the parent ticket):** `no-cross-technology-✏️s-to-🧰️framework` fires for package-name imports of the sanctioned SDK (`@semio-tech/framework`) because the cruiser resolves workspace symlinks to `🧰️framework/…` and classifies them as local. The audit log has 1758 such edges, 87 of them to the SDK barrel. Any s-side file that imports the SDK is flagged, so the rule cannot be met by the route `plugins-framework-sdk-only` prescribes. This is why my moves add 4 such edges (harness 3, TXT test 1) while removing 94 framework-to-s edges. Fixing the rule (treat resolved workspace-package imports through authored export targets as non-local) should precede work package 7.
- **Finding:** two framework tests were already red in the tree because of the cross-layer import (editable-controls did not load; the staged-arg Draw case failed on a Draw schema change). Neither would have been caught by a layering gate; both are green now.
- Peers touched overlapping files while I worked: `.vscode/launch.json` (my edits are three single-line replacements and one rename), the fixture-placement test (see edge 7) and the renderer's `🧬️contract/🧵️retained` tables folder (a transient Vite resolve failure during one run; retried and green).
- Moved test bodies keep their existing `[DEBUG]` console lines; I added none.
