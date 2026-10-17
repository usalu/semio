# Result SHELLHELPERS-UNBARREL (prefix `shu`): remove the `ShellHelpers/🟦️.tsx` re-export barrel

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`. Tools in `F/shu-tools/`, raw outputs in `F/🗑️generated/shu-*`, pre-edit snapshots in `F/🗑️generated/shu-snap/` and `shu-snap2/` (`SHA.txt`, `barrel.orig`).

## 0. Verdict

**DONE.** `E/🛠️ShellHelpers/🟦️.tsx` (427 lines, 349 re-exported names, declared nothing itself) is deleted. 106 importer files now import every name from the module that owns it; no compatibility re-export remains. The type-check has no new diagnostic (24 before, 9 after; the 15 that went away were fixed by peers), and the renderer-react vitest package (long level) shows the baseline failure set (38 failed tests in 22 files before; 38 in 23 files in the final run, the three differing names are load-flaky and pass solo, see section 4).

| Item | Value |
|---|---|
| Barrel | deleted; the `🛠️ShellHelpers` folder stays (18 module folders), as `⚙️EngineCanvas`, `🌉️ProgramBridge`, `🎞️Scenes`, `🛰️Dock` also have no root file; the `members-of-elements` taxonomy row names the folder and is unchanged |
| Names re-pointed | 307 of the 349 barrel names (295 in the 89 importers that name `ShellHelpers`, 12 more in the 17 importers inside the folder). The other 42 names had no importer and are gone (listed in 3.4) |
| Importer files changed | 106 (87 outside the folder plus 19 inside; 88 by the first codemod pass, 3 partly by hand, 17 by the second pass), counting only `import`, `export … from`, dynamic `import()` and `import()` type sites |
| Owner modules | 23 (one per child module of the folder, plus `📤️SegmentedDownload` and `@semio-tech/framework`) |
| Readers and docs re-pointed | 1 Rust census test, 1 hub oracle, 1 fixture, 1 history test, 1 policy list, 13 docstrings, 6 fixture notes (section 3.3) |

## 1. Found state

- Fresh slice, started 2026-10-17 17:41. No earlier `shu` folder or log.
- The barrel held only `export { … } from` lists (24 lists: 22 relative, 2 for `@semio-tech/framework`, one `export type`) plus a `ChunkedImport` docstring region. It was modified in the working tree at 15:31 by a peer; I removed the text as found (unchanged since my snapshot at deletion time, `cmp` clean).
- No other file under `ShellHelpers/**` re-exports (`F/shu-tools/exportfrom.cjs`): the 18 child modules own their names. `🔩️plumbing/🟦️.ts` is an owner (16 names), not a barrel.
- Concurrent edits seen: `E/../🧪️tests/🪟️spawned-program-session` was edited by a peer (tt5) between my apply and my check; both edits are in the file. Peer type-fixes made 15 baseline diagnostics disappear.

## 2. Method

1. **Importer discovery** (`shu-tools/scan.cjs`, TS AST): every candidate file (git-tracked and untracked text `ShellHelpers`, plus every `.ts`/`.tsx` below the folder, because files inside the folder import `../../🟦️.tsx` without naming the folder) is parsed. Sites: static `import`, `export … from`, dynamic `import()`, `import()` types, `require`, `vi.mock`/`vi.doMock`/`vi.importActual`, `?raw` suffixes. Result: 94 sites in 89 outside files (90 imports, 1 `export … from`, 2 dynamic imports, 1 `import()` type) plus 17 files inside the folder; no `vi.mock`, no `?raw`, no default or namespace use.
2. **Owner map** (`shu-tools/ownermap.cjs`): a TypeScript program rooted at the barrel; for each export, `getAliasedSymbol` follows the chain to the declaring file, then the owner module's exported name is looked up (no renames occurred, 0 problems). Names that the barrel took from `@semio-tech/framework` keep that specifier (4 names). Two names (`SEGMENTED_DOWNLOAD_CONTRACT`, `SEGMENTED_DOWNLOAD_REFUSAL`) resolve through `📤️SegmentedDownload` into `🔨️modules/🎭️actor/📮️shard-client`, outside the renderer package; the codemod stops at the `SegmentedDownload` element for names whose owner lies outside `📺️renderer` (no importer used them anyway).
3. **Codemod** (`shu-tools/rewrite.cjs`, `run.sh`): per file, each named import or named re-export element is moved to the declaration for its owner module. Type-only is preserved per element and per declaration, aliases are kept, an existing named, non-default, same-type-ness import of the same module is extended instead of adding a second one (verified with `shu-tools/dupes.cjs`: 0 files gained a duplicate declaration), declarations are formatted in the file's existing style (single line up to 140 characters, otherwise one name per line, trailing comma kept). Anything it cannot rewrite (dynamic `import()`, `import()` types, namespace/default/side-effect use) is reported and refused, not guessed. It is idempotent: runs `r3`, `r4`, `r5`, `r8`, `r9` over a fresh scan found 0 sites.
4. **Lossless check** (`shu-tools/lossless.cjs`): for all 106 files, the text outside import and `export … from` declarations and the set of imported `(local name, type-only, imported name)` are compared with the pre-edit snapshot. Result: identical in all but the files I edited by hand for other reasons (3 dynamic/type sites, one comment, and the peer's `spawned-program-session` edit).

## 3. What changed

### 3.1 Importers (106 files)

- **Renderer tests** (`E/../🧪️tests/*`, 28 files), **elements** (`ShellHost` 29 files incl. children and its `📎️local-folders` test, `ChromePanels` 7, `Shell` 6, `World3dHost` 5, `Interpreter` 3, `UtilityTree` 2, and one each in `TextEditor`, `Board2dHost`, `Canvas2dHost` test, `ShellSearch` test, `IconRenderHost` test), the target entry `🎯️targets/⚛️react/🟦️.tsx` (its `export type { … } from` list became two lists, its import list was split into owner imports; it also merged `IMPORT_CHUNK_BYTES` and `importPayloadChunks` into its existing `@semio-tech/framework` import), the puzzle plugin test `✏️s/🔌️plugins/🧩️puzzle/…/🧪️tests/🥽️brush-mesh-upload/🟦️.ts` (one owner: `🛑️extension-io`), and 19 files inside the folder (2 time-travel tests in the first pass, 17 in the second: `⏪️time-travel/🟦️.tsx` and its 7 tests, `⏯️tool-run-panel` test, `🧪️tests/{🌐️chrome-history-locale, 🌐️instance-title, 🎭️browser-actor-panels, 🧩️component, 🧪️command-rejection, 🧪️fault-notices, 🧪️staged-arg-controls, 🪟️tree-windows}`).
- By hand (the codemod reported them): `🧪️tests/🎫️shell-holds` (`await import` of `🔩️plumbing`), `🧪️tests/🔁️refresh-coalescing` (`await import` of `🔩️plumbing/📥️downloads-and-file-requests`, which owns `requestBackboneFilePath` and `requestBackboneFolderPath`, the two names the fixture `🔗️sync-backbone-path-picker` lists), `🧪️tests/🗃️shell-store-reducer` (`import()` type of `TutorialUiBridgeContext` now from `🧾️transfer-checkin-tutorial`).

### 3.2 Names per owner module (307 re-pointed)

`🛣️route-and-chrome` 50, `🧾️transfer-checkin-tutorial` 35, `🏷️labels` 29, `🛑️extension-io` 25, `🎛️command-registry` 23, `🔄️ui-refresh` 18, `🔩️plumbing/🟦️.ts` 14, `🔩️plumbing/📥️downloads-and-file-requests` 13, `🌳️tree-windows` 11 (plus the 12 added by the second pass, spread over the same modules), `🔩️plumbing/🆔️action-ids` 9, `🎞️media-import` 8, `🔩️plumbing/👥️presence-identity` 8, `🔩️plumbing/🔀️example-switching` 8, `🪟️window-panes` and its five children 29, `📌️panel` 5, `🔩️plumbing/🎮️leftover-views` 3, `🔩️plumbing/🚀️effect-dispatch` 3, `📤️SegmentedDownload`, `@semio-tech/framework` 2.

### 3.3 Source-text readers and documentation

| File | Change |
|---|---|
| `E/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs` | `react_declared_storage_keys` read the barrel as one of React's storage-key sources; now reads `🛠️ShellHelpers/🔩️plumbing/👥️presence-identity/🟦️.ts`, which declares `PRESENCE_CLIENT_STORAGE_KEY` (the census lists `semio.presence.client`; the barrel never declared it). Not run (Rust, no compile admitted); one line, path verified to exist |
| `🌎️hub/🧪️tests/🛂️space-administration-checks/🟦️.ts` | `presence-lease-oracle` reads the presence module for `PRESENCE_HEARTBEAT_INTERVAL_MS = 5000`; verified the needle with `shu-tools/readers-check.ts` (barrel and new path both true) |
| `E/../🧪️tests/🧫️fixtures/🌳️wgpu-document-reconcile/🔣️.json` | `reactShellHelpersSource` now `🌳️tree-windows/🟦️.tsx`; both needles of the wgpu-document-reconcile test verified present there with `readModuleTreeSource` |
| `E/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🕰️history-body/🟦️.tsx` | `readFileSync(<barrel>)` replaced by `shellHelpersSource(shellHelpers)` (all non-test sources below the folder), so the "no host-built history tab" law now reads the whole shell surface instead of an export list; 5/5 pass |
| `📚️library/🔍️discovery/📖️source-access/🟦️.ts` | removed the `ShellHelpers/🟦️.tsx` entry from `POLICY_SPLIT_SOURCE_ROOTS` (83 roots remain, all read, 0 bad: `shu-tools/roots-check.ts`) |
| 13 docstrings and comments naming `ShellHelpers/🟦️.tsx` | now name the owner module (`generate-interactions`, `interaction-scope`, `dev-collaboration`, `shard-client`, `manifest`, `retained-control-commit`, `settle-pump`, `shell-uri-documents`, `channel-adapter`, `load-progress`, `measure-controls`, `panel`) |
| 5 fixture notes | `ui-dirty-scope`, `retained-control-commit`, `app-mode-layouts`, `window-actions-search-panes`, `window-pane-chrome` (prose only; JSON parses) |

### 3.4 The 42 barrel names nobody imported (dropped with the barrel)

`CHORD_KEY_NAME_ALIASES DOCUMENT_TRANSFER_NOTICE_LABELS_V1 FRAMEWORK_RESERVED_ACTION_IDS HistoryRefusalCodeV1 ImportChunk KeyboardEventLike LeftoverInteractionViewV1 MediaFramesRunV1 NaturalFileFormatV1 OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID SEGMENTED_DOWNLOAD_CONTRACT SEGMENTED_DOWNLOAD_REFUSAL SHELL_HISTORY_DOM_LABELS SegmentedDownloadEncoding SegmentedDownloadSink SegmentedDownloadSinkFactory ShellNoticeV1 TREE_WINDOW_REPORT_DEBOUNCE_MS TreeWindowSchedulerOptionsV1 UndeclaredActionDiagnostic WINDOW_CONFIG_RAIL_ACTION_IDS WindowKindActionDeclaration beginIsolatedJobDrive canonicalChordKeyToken chordKeyTokenIsCanonical contextMenuGroupLabel createBufferedDownloadSink createSegmentedDownloadSink currentShellLocaleV1 endIsolatedJobDrive importChunkArguments isSpacePanelState isolatedJobDriveIsActive isolatedJobDriveSnapshot parseSegmentedDownloadMarker parseSegmentedDownloadOperationId pluginLoadRemainingMs requestIsolatedJobUiPoll ribbonParentLabel subscribeIsolatedJobDrive takeIsolatedJobUiPoll`

## 4. Verification actually run

| Check | Command | Before | After |
|---|---|---|---|
| Type-check, whole renderer | `F/shu-tools/tsc.sh <label>` (tsc `--noEmit -p F/🗑️generated/shu-tsconfig.json`: `💻️os/tsconfig.json` plus `📺️renderer/**`, normalised by `tscnorm.py`) | `shu-tsc-before.tsv`: exit 2, 24 diagnostics (frame-worker 3, row-action-admission 3, wgpu-browser-media 1, package-integration 8, wgpu-module-routes 1, embedded-mount 2, spawned-program-session 3, echo-pack 1, lifecycle 1, paint 1) | `shu-tsc-a3.tsv`: exit 2, 9 diagnostics, all in the baseline set (the only textual difference is the message of one `♻️lifecycle` diagnostic that a peer changed). A first run after the barrel deletion gave 60 (17 `TS2307` for the files inside the folder that the first scan had missed); the second codemod pass fixed all of them |
| Unit tests, renderer-react package, long | `F/shu-tools/vitest.sh <label>` (the package config `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `SEMIO_TEST_LEVEL=long`; sets via `vtsum.py`) | `shu-vt-before`: 222 files (200 passed, 22 failed), 3,206 tests (3,168 passed, 38 failed) | `shu-vt-a3`: 222 files (199 / 23), 3,205 tests (3,167 / 38). The failed-test list differs in three names: `package-integration … uncorrelated progress frame` (passes now, a peer fixed it), `embedded-mount … renders two idle real ShellHosts` (the known load-flaky `vi.waitFor` test: fails in `shu-vt-a3`, passes in `shu-vt-a2` and in the second solo run `shu-vt-solo2`), and `Paint2dHost selection-focus` (fails on a different case each loaded run, 'Apply' before, 'Mask from selection' in `a3`, passes in `a2` and both solo runs). Run `a2` (before the history-body fix) had 37 failed with one extra failure (`history-body`, an `ENOENT` on the deleted barrel) which the fix removed (`shu-vt-hb`: 5/5 passed) |
| Puzzle plugin renderer-contract (the `brush-mesh-upload` importer) | `F/shu-tools/puzzle-vt.sh <label>` | 3 files, 20 tests passed | 3 files, 20 tests passed |
| Reader needles | `bun F/shu-tools/readers-check.ts barrel|owners` | all true | all true |
| Policy roots | `bun F/shu-tools/roots-check.ts` | n/a | 83 roots, 0 bad, no ShellHelpers entry |
| Remaining references | `scan.cjs` over the fresh candidate list (`run.sh r9`, after all edits): 0 sites; no `ShellHelpers/🟦️.tsx` string remains in a `.ts`/`.tsx`/`.json` file except the Rust comments below | 94 + 17 | 0 |

## 5. Hand-offs and limits

1. **Rust comments**: 66 Rust files carry doc comments in `✏️s/**`, `🌎️hub`, `🧰️framework/🔨️modules/{🎠️kernel,🖱️ui,🛂️manifest}` and `E/🐚️Shell/🎯️targets/🧊️wgpu/**` still say "`🛠️ShellHelpers/🟦️.tsx`" (several with long-stale line numbers such as `:4169`). None is code; they were left because Rust slices are running and the names have 23 different owners. A later sweep can map each backticked function name to the owner table in `F/🗑️generated/shu-owners0.json`.
2. **`📤️SegmentedDownload/🟦️.ts`** re-exports `SEGMENTED_DOWNLOAD_CONTRACT` and `SEGMENTED_DOWNLOAD_REFUSAL` from `🔨️modules/🎭️actor/📮️shard-client/📤️segmented-download` (a compatibility-style re-export owned by another slice); I did not touch it.
3. **Not run**: the Rust test `browser-prefs-persistence` (no compile admission; the edited line points at an existing file that declares the key), ESLint, a Storybook build, the hub `presence-lease-check` through the launcher (a bare `bun ./📜️script.ts` is refused with "Original parent Script process invocation required"; the needle it checks was verified separately).
4. **Tooling lesson recorded**: a candidate scan that greps for the folder name misses importers inside the folder (they import `../../🟦️.tsx`); include every source file below the barrel's directory.
5. Files not to delete: `F/shu-tools/`, `F/🗑️generated/shu-*` (snapshots, owner map `shu-owners0.json`, scans, tsc/vitest outputs).

## 6. Files touched

- Deleted: `E/🛠️ShellHelpers/🟦️.tsx`.
- Edited: the 106 importer files (`F/🗑️generated/shu-snap/SHA.txt` and `shu-snap2/SHA.txt` list them), plus the readers and docs in section 3.3.
- Created: `F/shu-tools/*`, this file.
