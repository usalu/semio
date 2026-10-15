# Result: region markers restored for the monolith splits (audit rows 1, 2, 4, 7, 9, 10, 23, 25, 26, 27, 30, 32 to 34, 36, 39, 42, 43, 45)

Executor: Sonnet 5.5, foreground, no sub-agents. Nothing was committed, staged, stashed or checked out. Only marker lines were inserted or moved, with one deliberate exception (the CAD test import, section 3.2). Originals are read from `git show 793f635:<path>` (the HEAD of this session). The list of files that existed at that commit is in `🗑️generated/regrestore-headfiles.txt`; per-split snapshots (hash of every tree file, before the first write) are in `🗑️generated/regrestore-snap/*.json`.

Tool: `regrestore-tools/` (`run.py <analyze|dry|apply|verify|balance|check|preview> <split>`, `specs.py`, `regrestore.py`, `bim_leaves.py`, `depthfix.py`, `depthscan.py`, `groupscan.py`, `ctxscan.py`, `touched.py`, `report.py`). Logs: `🗑️generated/regrestore-tsc-{os,cad}-{before,after1,after2}.log`.

## 1. Method

1. Parse the original into region instances (name, nesting, enclosed lines). Names are compared after cutting a trailing ` — description` (the originals open `🧭️DockAssembly — default ...` and close `🧭️DockAssembly`).
2. Map every non-trivial line of each tree file to its original line by unique-line anchors (runs of equal delta, then extended over equal neighbours). Each run is cut where the original region stack changes. Result: for each child, the original region stack of each contiguous piece.
3. Turn the pieces into marker pairs, in the original marker text. A region that spans several files is repeated, balanced, in each file; nesting follows the original; names a file already carries are not duplicated (existing partial markers are completed instead: a stray `#region` gets its `#endregion` after the last member, a stray `#endregion` gets its `#region` before the first member).
4. Guards: a marker never lands inside a bracketed group or an `import`/`use` list (the span is widened to the whole statement); never between an attribute or doc comment and its item; a region must start and end at the same bracket depth (spans are widened or shrunk to a balanced block); a lexer for Rust and TS (strings, raw strings, templates, regex literals, comments) is used for this.
5. Writes re-read the file and assert `text == text read at plan time` and `strip_markers(new) == strip_markers(old)` before writing.

Policies for broken originals (all listed in the per-split notes):
- Regions that were already empty at HEAD (the members had left the file before the split) are restored as empty pairs at their original position (p2d: 8, p5d: 1).
- The two mojibake markers of the CAD original (`­ƒ¬®Repl` closing `🪩️Repl`, `­ƒº¬Tests`) are read as `🪩️Repl` and `🧪️Tests`.
- A region left open to the end of the original file (`Gumball`, `MeshHelpers`, `🔖️plugin-runtime`, `🔖️DependencyOrderedBoot`, `🔖️json`, `🧪️Tests` of CAD) is closed after its last contiguous child region, or before the next sibling that opens at the same level, or at the end of the file.

## 2. Verification table

`names` counts distinct canonical region names (one count per original name; the original names are the open-marker texts). Balanced means: every `#region` has its `#endregion`, properly nested, `#endregion` without a name closes the top. "Code differs" counts tree files whose marker-stripped text is not identical to the snapshot taken before my first write; see the notes under the table.

| split | audit row | orig names | tree names before | tree names after | tree files | files touched | unbalanced files (before / after) | code differs from snapshot | rustfmt parse | tsc |
|---|---|---|---|---|---|---|---|---|---|---|
| bim | 4 BIM model rehome | 2 | 1 | 2 | 223 | 216 | 0 / 0 | 0 | 216 files + the mutations parent, 0 failures | n/a |
| cad | 7 CAD renderer | 22 | 2 | 22 | 12 | 11 | 0 / 0 | 1 (intended, 3.2) | n/a | no change |
| p2d | 9 puzzle 2d editor | 16 | 8 | 16 | 5 | 4 | 0 / 0 | 0 | 0 failures | n/a |
| p5d | 10 puzzle 5d editor | 19 | 18 | 19 | 9 | 4 | 0 / 0 | 0 | 0 failures | n/a |
| md | 1 markdown-html-compiler | 7 | 6 | 7 | 1 | 1 | 0 / 0 | 0 | n/a | no change |
| pres | 23 presentation react | 20 | 2 | 20 | 17 | 15 | 0 / 0 | 0 | n/a | no change |
| pres2 | 2 animate react copy | 20 | 2 | 20 | 17 | (same tree as 23) | 0 / 0 | 0 | n/a | no change |
| world | 25 infinite world | 27 | 27 | 27 | 14 | 7 | 3 / 0 | 0 | 0 failures | n/a |
| normal | 26 directed normal | 7 | 7 | 7 | 7 | 6 | 2 / 0 | 0 | 0 failures | n/a |
| dag | 27 directed dag | 16 | 16 | 16 | 6 | 6 | 1 / 0 | 0 | 0 failures | n/a |
| wgpurenderer | 30 wgpu renderer | 37 | 36 | 37 | 17 | 15 | 8 / 0 | 0 | 0 failures | n/a |
| world3d | 32 World3dHost | 22 | 1 | 22 | 10 | 10 | 0 / 0 | 5 (peer import edits) | n/a | no change |
| scenes | 33 Scenes | 40 | 40 | 40 | 16 | 8 | 5 / 0 | 0 | 0 failures | n/a |
| shellhost | 34 ShellHost | 37 | 25 | 37 | 31 | 31 | 0 / 0 | 4 (peer import edits) | n/a | no change |
| pluginruntime | 36 PluginRuntime | 15 | 1 | 15 | 19 | 19 | 0 / 0 | 0 | n/a | no change |
| shellhelpers | 39 ShellHelpers | 21 | 1 | 21 | 11 | 11 | 0 / 0 | 0 | n/a | no change |
| oshost | 42 os host | 34 | 34 | 34 | 9 | 0 | 0 / 0 | 0 | n/a | n/a |
| osts | 43 os.ts | 40 | 40 | 40 | 6 | 1 | 1 / 1 (kept on purpose) | 0 | n/a | no change |
| graphql | 45 repo GraphQL | 37 | 37 | 37 | 10 | 0 | 0 / 0 | 0 | n/a | n/a |

Checks that apply to the whole table:
- Names: for every split, the set of distinct names in the tree contains the original set (`run.py verify` prints `missing=[]` for all 19 rows).
- Member files without marker (a tree file holds lines of a region but carries no marker of that name; segments of fewer than 4 non-blank lines ignored): 0 for all 19 rows.
- Code identity: asserted in the writing script for every write (`strip_markers(new) == strip_markers(old)`), and again by `run.py verify` against the pre-write snapshot. The 10 files in the "code differs" column are explained below.
- Brackets: `depthscan.py` (open and close of every pair sit at the same bracket depth) reports 2 pairs over all touched files, both `🧵️FillJob` in the normal split, which the original itself has misaligned (FillJob opens inside `impl BoardHost {` and its last line is a second `impl BoardHost {`). `groupscan.py` (a marker inside an import list, call or array) reports 0. `ctxscan.py` flags 14 markers, all trailing `#endregion`s after a comment line or the existing `Header` regions after the file docstring; no marker sits between an attribute or doc comment and its item.
- rustfmt: `rustfmt --edition 2021 --emit stdout --config-path rustfmt.toml < file` on stdin for every touched Rust file (parse check, exit 0 or 1 accepted, 2 would be a parse error): bim 216, p2d 4, p5d 4, world 7, normal 6, dag 6, wgpurenderer 15, scenes 8, plus the bim mutations parent: 0 parse failures. Raw cargo is gated, so no cargo check was run; only marker comment lines were added, which is what the identity check proves.
- tsc: `node_modules/.bin/tsc --noEmit -p 🗑️generated/regrestore-tsconfig-os.json` (extends `💻️os/tsconfig.json`; files: ShellHost, ShellHelpers, World3dHost, PluginRuntime parents, the os root, the presentation react parent; follows the imports into all children) and `-p regrestore-tsconfig-cad.json` (renderer parent, both test suites, the story). os: 34 `error TS` lines before and after (27 in `UiDocumentStore/🧪️tests/🧪️typedwire`, 5 in five PluginRuntime children (the one pre-existing `Lane` import error repeated per child, see `RESULT-PLUGIN-RUNTIME-TSX.md`), 2 in `WasmSessionLoader`); cad: 15 before and after (missing `brepjs` / spatial-kernel modules and the 3d-js suite); none of these errors is on a line of a region marker. The sorted error lists with `(line,col)` stripped are identical before, after1 (after all TS inserts) and after2 (after the folder rename and all fixes): `diff` empty. The baseline is not a silent zero: both logs contain real errors that are reproduced exactly.

Notes on the "code differs" column:
- cad: 1 file, the parent `🟦️.tsx`: the import path of the renamed test folder changed (3.2). Intended.
- world3d (5 files) and shellhost (4 files): another fleet agent re-points imports in `🧱️elements/**` while I worked (coordinator message). Those files changed after my snapshot; each of my writes was asserted marker-only against the file content read immediately before it. I anchored on member declarations, and the only markers near imports are the `🔌️Adapters` pairs, widened to whole `import` statements. After the peer's edits, 2 `Adapters` closes (`identity-document-ports`, `space-artifact-creation`) had ended up inside an import list; I moved them to after the statement (`depthfix.py`, marker lines only).
- bim: the mutations parent (`🧬️mutations/🦀️.rs`) was not in the first snapshot; its write was asserted marker-only at write time.

## 3. Coordinator follow-ups

### 3.1 (a) the "4 os.ts children" without a leading emoji doc comment

The four files are `🏪️store/📦️blob/🧬️schema/{📦️packages/🦀️rust/📜️script.ts,🧪️tests/🟦️.ts}` and `🏪️store/🔗️link/🧬️schema/{📦️packages/🦀️rust/📜️script.ts,🧪️tests/🟦️.ts}` (below `💻️os/🔨️modules/`). They are not children of the os.ts split: that split created exactly five children (`🎁️packs`, `📞️directory-client`, `🧵️backbone`, `🧾️app-channel`, `🪢️wire-contracts`), all of which start with a `/** ... */` docstring. The four files are new (index `A`, written 2026-10-15 04:20 to 04:30), belong to a separate schema-producer packaging of the store blob and link records (`📦️packages/🦀️rust/Cargo.toml`, `🦀️.rs` codec), and no RESULT file mentions them. I added the requested doc comments anyway (the files did not change since I read them; the shebang stays on line 1):
- `📦️blob/.../📜️script.ts`: `/** 🪪️ Blob identity package script: native identity law tests and the schema contract check. */`
- `🔗️link/.../📜️script.ts`: `/** 🔗️ Durable link package script: native codec tests and the schema contract check. */`
- `📦️blob/.../🧪️tests/🟦️.ts`: `/** 🧪️ Contract test: ... */`
- `🔗️link/.../🧪️tests/🟦️.ts`: `/** 🔬️ Contract test: ... */`

These files print `[DEBUG]` log lines of their author; I left them.

### 3.2 (b) sibling emoji duplicates

- CAD renderer tests (row 7): `🧪️repluserfacingsuggestiondetail` is tracked at HEAD, `🧪️hookorder` was created by the CAD split (index `A`, 04:02). Renamed `🧪️tests/🧪️hookorder` to `🧪️tests/🪝️hookorder` (🪝, RGI, already used as a folder prefix in the repo, e.g. `🪝️wire-codec`; not used by any sibling). References: the only one is the dynamic import in the renderer parent (`await import("./🧪️tests/🪝️hookorder/🟦️.tsx")`, the single intended code change). The test file's own relative imports are unchanged (same depth) and it already opened with `/** 🪝️ ... */`. `git grep hookorder` finds no taxonomy, inventory, `project.json` or tsconfig entry (the only other hits are `RESULT-CAD-RENDERER.md`, where I updated the three mentions, and the audit). tsc for the cad config, which lists the test file, is unchanged (15 errors before and after).
- os.ts (row 43): `🔗️link/🧬️schema/` has `📦️codec` and `📦️packages`. Both are new, but neither was created by the os.ts split (see 3.1); no rename done, as instructed ("only if our split created it"). If the owner of that work wants it: one of the two folders needs a different emoji, and the only importers are `🦀️.rs` (`#[path]` of the codec), `📦️packages/🦀️rust/Cargo.toml` and the test's `join(root,"📦️codec/🦀️.rs")`.

## 4. Per split

- **bim (4)**: the original region `🔖️Leaves` (HEAD lines 389 to 7288) wrapped the inline mount tree of all mutation leaves; the rehome (`RESULT-BIM-MODEL-REHOME.md`) moved the wrappers into `pub mod <leaf>;` mounts in `🧬️mutations/🦀️.rs` (inside its new `//#region 🧩️Modules`) and each leaf's own mounts into 216 new `<leaf>/🦀️.rs` files. `//#region 🔖️Leaves` now wraps the leaf mounts in the mutations file (from `create_site` to `set_type_thermal_data`) and the mount block of each of the 216 leaf files. Placed by `bim_leaves.py` (the paths were rewritten, so the line anchors do not apply). `🔖️Declaration` was already in the parent.
- **cad (7)**: 20 names added over 10 files (`Adapters`, `PortWiring`, `R3FPreviewKernel`, `WorkerClient`, `ArchivedFootprints`, `Layout`, `GeometryTargets`, `DisplayPrimitives` with its nested `CircleArcPreview`/`HostCustomization`/`TransformGumball`/`SpatialSceneColors`, `Interaction`, `GeometryInteraction` with `SpatialPickGeometryLayer`, `TypologyPatternMaterial`, `CommittedMesh`, `Hooks`, `Canvas`, `Repl`); `Header` and `Tests` were already in the parent. The original has 22 opens and 20 closes: two mojibake markers (see section 1) and one unclosed `Tests` (the parent already wraps its in-source test block).
- **p2d (9)**: the eight regions `Constants`, `Scene`, `BoardReads` (with `TargetRegions`), `Clipboard`, `BoardHostSync`, `UiScopes`, `Puzzle2dCommand` were already empty at HEAD (only blank lines between the markers); restored as empty pairs in the parent where they stood (after the last `use`). `PlayApp`, `RetainedCommands` and `ReservedJobs` were added to the children `🖌️session`, `🧰️kit` and `🧵️retained` (the original left `ReservedJobs` unclosed and closed `PlayApp` over it; the stray `📜️ToolProofs` close at HEAD line 3256 is the original's).
- **p5d (10)**: `🙈️SelectionFlags` (empty at HEAD) restored in `🗂️catalog` inside `🧬️InferredKinds`; `PlayApp` and `RetainedCommands` added to `👆️gestures`, `🧰️kit`, `🧵️retained`.
- **md (1)**: 6 of 7 names were carried; `🧪️Tests` (the in-source `import.meta.vitest` block, HEAD lines 361 to 366) was not carried because the deduplication deleted the animate test suite it dispatches to. Restored as an empty pair after `🔌️Compiler`; the block itself is not brought back (that would be code).
- **pres (23) and pres2 (2)**: the 14 modules under `🎤️presentation/🎯️targets/⚛️react/🔨️modules/` carried no markers. 18 names added over 15 files (`ArrangementSettled`, `HiddenPreflight`, `MorphView`, `Markdown`, `MountOptions`, `RevealChrome`, `PresentationDeck`, `Mount`, `SlideEpoch`, `PdfCanvasPort`, `RevealMorph`, `FigureScroll`, `ArrangementSection`, `PresentationInteractionProvider`, `Interaction`, `json`, `Renderer`, and the pair `Adapters`); the parent's `🔖️json` and `🧪️Tests` wrap its in-source test block. The animate copy (row 2) has the same 20 names and the same tree, so it is covered by the same markers. The unclosed `🔖️json` of the original (to the end of the file) is closed after its last children. The parent still starts with `// #region 🧲️Header`, not a doc comment (docstring criterion D of the audit; not part of this task).
- **world (25)**: 3 unbalanced files fixed: `🧱️scene` (the original leaves `Gumball` and `MeshHelpers` unclosed: closed after their members, `MeshHelpers` after `PlaceholderMesh`), `🧵️interaction/📇️registry` (stray open of `🧵️WorldInteractionTransaction`: close added) and `🧵️interaction/🦀️.rs` (stray close: open added before the `#[path]` mounts).
- **normal (26)**: `board_host` (the original region opens inside `impl BoardHost {` and spans several `impl` blocks) now wraps the content of each child, `🧵️FillJob` got its missing partners in `🎛️selection-events` and `🪣️fill-job`. I first applied an older version of the tool here, found `board_host` markers cutting `}` lines, removed exactly my 15 marker lines and applied again. Remaining oddity (the original's): `FillJob` ends right after a second `impl BoardHost {` line.
- **dag (27)**: `🔖️IoNode` stray close got its open (parent, before the `#[path]` mounts); `DagHost` added to `🖌️paint`, `🛠️mutate`, `🧭️host-selection`, `🧱️frame-minimap-align`, `WasmSession` to `🌐️wasm-session`.
- **wgpurenderer (30)**: 8 unbalanced files fixed (bare `//#endregion` closers of `Requests/Outcomes`, `KernelFuture`, `KernelClient`, `KernelPoolState`; `RendererAssetAuthority`, `RuntimeMailbox`, `AppInteractionState`); `KernelRuntime`, `RuntimeMailbox` and `ScaleBench` added to the kernel, mailbox, raster and scale-bench children. 36 of 37 names were present before; the original leaves `RuntimeMailbox` open (closed at the end of the last child).
- **world3d (32)**: 21 names added over 10 files (outer `🔖️World3dHost` and `WorldSceneParsing`, then `WorldMeshPaint`, `GlbMeshBounds`, `GlbMeshStyling`, `WorldInteractionDomain`, `CatalogueDrop`, ...).
- **scenes (33)**: 5 unbalanced files fixed (`TextEditor`/`State`, `InkCanvas`/`InkCanvasState`, `SceneRuntime`); the rest of the 40 names were present.
- **shellhost (34)**: 12 names added (`FrameworkOsShell` around each hook body, `ShellMount`, `CheckIn`, `SyncLeaf`, `FooterUtilityLeaves`, `DockAssembly`, `TutorialOverlayHosts`, ...); the long-named opens of the original (`🔖️CheckIn — ticket §C5 ...`) pair with their short closes. `Adapters` pairs wrap the import blocks. Two lexer problems found and fixed on the way (regex literal with `//` inside `check-in-sync`, `shell-uri-documents`).
- **pluginruntime (36)**: 14 names added over 18 children (`🔖️plugin-runtime` and `ActorAdapter`/`ChannelAdapter` nests, `GenericThunkQueue`, `PluginTurnScheduler`, `Transaction`, `Merge`, `DependencyOrderedBoot`, ...), the parent wraps its re-exports and the in-source test dependencies.
- **shellhelpers (39)**: 20 names added over 11 files (`ShellHelpers` outer region per child, `UtilityRegistry`, `TreeWindows`, `CommandRegistry`, `ToolRegistry`, `RequestMediaFrames` with `Bmff`/`Tier1`/`Tier2`, ...).
- **oshost (42)**: nothing to do. All 34 names are present, the nine files are balanced (the three unbalanced files of the audit were fixed by a peer in the meantime; HEAD itself has 39 opens and 39 closes).
- **osts (43)**: the five children and the parent are covered; one region added: `🔖️BackboneWorkerProtocol` around the re-exports in the parent. `🧵️backbone/🟦️.ts` keeps its single unpaired `//#endregion 🔖️EnvelopeCodec` (8 opens, 9 closes) on purpose: the original (40 opens, 41 closes) has exactly this stray close (HEAD line 1511) and no open for it anywhere. Tree total: 58 opens and 59 closes, the same one stray. To balance it, the open would go before `export function` of the envelope codec in `🧵️backbone`; I did not invent it.
- **graphql (45)**: nothing to do. All 37 names exist in the facade and its children (`🏗️repo-schema`, `🗄️fs-context`, `🔤️grammar`, `⚡️execution`, `🖼️sources`, `🎥️context-port`, `🩻️recording-context`); 10 files balanced; no member file without marker.

## 5. Spot-check of the splits that were not part of the work

Read-only (`run.py check <split>`), no file changed: names present and files balanced everywhere; member files without a marker of the region they hold (strict check, segments of 4 or more lines) exist in some:

| split | orig names | in tree | unbalanced files | member files without marker |
|---|---|---|---|---|
| hub bootstrap (`hubboot`) | 18 | 18 | 0 | 0 |
| hub inference runtime (`hubinf`) | 1 | 1 | 0 | 1 (`🌍️gis-map-binding`: `🛣️Routes`) |
| hub directory (`hubdir`) | 21 | 21 | 0 | 0 |
| tiled map (`tiled`) | 13 | 13 | 0 | 9 (`MapContent` in 4 files, `MapLod`, `VectorTiles`, ...) |
| manifest (`manifest`) | 42 | 42 | 0 | 18 (`🛂️Manifest`/`🔖️Manifest` in `🕹️actions`, `🖼️view-model`, `🧮️arg-schema`, ...) |
| Shell wgpu (`shell`) | 87 | 87 | 0 | 9 (`ShellChrome`, `ShellTypes`, `ShellInput`, `ShellLifecycle` in 6 children) |
| plugin host (`plughost`) | 25 | 25 | 0 | 0 |
| repo cli (`cli`) | 13 | 13 | 0 | 7 (`🧭️RepoCli` in 5 children, `🔌️McpVerb`, ...) |
| quiz (compared by name over the whole `❓️quiz` tree) | 9 | 9 | 0 | n/a |

These meet the audit criterion (names present, files balanced). The member-file gaps are real but were left as they are, per the instruction; `python3 run.py apply tiled manifest shell cli hubinf` from `regrestore-tools/` closes them in the same way as the splits above.

## 6. Limits

- Anchoring is by unique lines, so a child item whose every line was reworded in the split can be missing from a region (the unclaimed-line counts are printed by `run.py analyze`; e.g. 88 of 5,180 non-blank original lines for pres, 472 of 13,196 for shellhost; they are mostly imports and signature lines).
- Regions that cross files are repeated, not collapsed: the tree has more marker lines than the original for every split.
- Nesting differs from the original where a file already had the inner region and the outer one is added (the outer is then placed around the inner pair when possible; where an inner region stood alone as a stray, it stays inside).
- Not compiled with cargo (gated); the Rust checks are parse and marker-only identity.
