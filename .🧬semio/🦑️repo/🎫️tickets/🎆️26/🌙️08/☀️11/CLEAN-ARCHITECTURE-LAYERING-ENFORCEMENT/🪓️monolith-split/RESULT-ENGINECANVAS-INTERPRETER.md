# Result: EngineCanvas and Interpreter wgpu split, with the neutral contracts module

Status: split applied to the working tree, name-resolution differential identical to the original in all four configurations, source-text readers re-pointed. **Not compiled with cargo**: the compiler-ingress source-boundary gate refuses the crate (see section 8). Nothing below claims more than what was run.

`E/` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`. A = `E/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/`, B = `E/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/`. Tools, logs and the pre-edit snapshots are in `🗑️generated/ecint/` (`tools/`, `oracle/`, `ts-before/`, `ts-after/`, `A-original*.rs`, `B-original*.rs`, `scenes-pre/`).

Note on the repo MCP: `ticket_open` and `ticket_close` were not used (the repo MCP is not connected in this session; the sub-track is recorded in `📓️FLEET.md`).

## 1. Step 0: neutral contracts module

New module `crate::element_contracts` at `E/🎯️targets/🧊️wgpu/🧬️element-contracts/🦀️.rs` (159 lines, `pub mod element_contracts`, mounted in `E/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` next to `media_slots`-style `#[path = "../…"]` mounts, before `engine_canvas`). The folder emoji reuses the repo's existing `🧬️contract` concept. Definitions were moved verbatim, no alias or re-export is left in `interpreter` or `scenes`:

| Contract | Came from |
|---|---|
| `WorkerCell<T>`, `TEST_WORKER_CELLS`, `test_worker_cell` (cfg test) | B head |
| `FrameworkWidgetContext` | B head |
| `ScenePointerTarget` (+ impls, `From<UiRetiredComponentScene>`), `SCENE_POINTER_OWNER_CAPACITY`, `fixture_scene_pointer_target` (cfg test) | B region RetainedEngineCutover |
| `RasterUploadRefusal` | Scenes `🪣️pending-raster` (B imported it from `crate::scenes`) |

Use sites updated (exact replacements, recorded in `🗑️generated/ecint/tools/edits.jsonl`, 31 records in 23 files): Scenes parent and children (`scene-state`, `scene-camera-dispatch`, `node-graph-and-tiled-map`, `pending-raster`), Scenes tests (`wgpu-canvas2d`, `wgpu-standalone`), Shell parent and 5 children (`pointer-input`, `chrome-frame-and-main-window`, `storage-prefs-and-i18n`, `theme-file-import-and-accessibility`, `space-admin-and-settle`), `🏠️os-host`, `📮️mailbox-impl`, `🧠️app-runtime`, the `🧪️wgpu-os-host-standalone` and A `🧩️wgpu-engine-surfaces` tests. Scenes edits are import or path text only, plus removal of the enum.

Result of the cycle work (grep over the live tree): A to Interpreter references 0 (was 2), A to Scenes 0. A is now a leaf of the three. **Residual cycle, not fixed:** B and Scenes still call each other by function (B to Scenes 159 references; Scenes to Interpreter 8 distinct names: `component_scene_is_presented`, `current_ui_image_key`, `note_world3d_live_camera`, `take_world_camera_diagnostic`, `ui_document_close_pending_for`, plus test-only `framework_widget_context`, `canvas_catalogue_drop`, `INTERPRETER_SOURCE`). These are retained-engine state queries, not contracts; moving them means moving `UI_ENGINE` state, which is out of this slice. Scenes also keeps its own private, different `WorkerCell` (RefCell inside the mutex); it was not unified because that changes borrow semantics. Both are recorded, not fixed.

## 2. Step 1: A split (9 children)

Parent `🦀️.rs` 7,651 to 205 lines. Children (folder under A, lines): `🗂️engine-surface-registry` 1138, `📦️prepared-engine-canvas` 769, `🎓️tutorial-surface-geometry` 175, `🧩️engine-surface-attach` 1148, `🕸️node-graph` 1341, `🗺️tiled-map` 309, `🎲️board2d` 889, `🖌️paint2d` 609, `✍️text-editor` 1382. Ranges follow the seam map (A.3/A.4), located by content on the audited numbering and verified with an automatic line remap against the live file.

## 3. Step 2: B split (10 children)

Parent 6,861 to 157 lines. Children (folder under B): `🧮️render-plan-validator` 224, `🧩️retained-ui-engine` 1686, `✍️text-and-table-focus` 588, `🖋️ink-and-vfs-focus` 782, `🖱️scene-interaction` 496, `📄️retained-document-consumer` 359, `🖼️ui-image-loading` 383, `🔬️introspection` 1411, `🧱️mesh-and-board2d-stats` 229, `🎯️chrome-ledger` 782. Largest child 1,501 active lines. Deviations from the map: the `//#endregion RetainedEngineCutover` line (4133) travels with the test mounts into `🧩️retained-ui-engine`, and `//#endregion 🔬️IntrospectionBuilders` (6006) travels into `🔬️introspection`, so region markers are balanced in every file (checked by script). A peer changed B at 04:45 (`semio_framework_job_worker` paths in the clipboard io code, +1 line); the split was rebuilt against that live text.

## 4. How the children are wired

- Parents keep: the original docs, only the `use` bindings their own kept tests need (cfg test), `#[path] mod` per child, `pub use` for every `pub` item (public paths unchanged), `pub(crate) use` for `pub(crate)` items that the rest of the crate names, `#[cfg(test)] use` for names the parent's kept tests use, and a `#[cfg(test)] pub(crate) const ENGINE_CANVAS_SOURCE` / `INTERPRETER_SOURCE` (`concat!(include_str!())` of parent and children, same pattern as `SHELL_SOURCE`).
- No globs. Each child has explicit `use` lines: the original header bindings it uses (trimmed from the monolith header, with the original cfg), `use super::<sibling>::{…}` for sibling items, `use super::{…}` for items of the parent-included test file. Import cfg gates are derived from the definition gate plus the gate common to all uses (test-only uses are `#[cfg(test)]`).
- Test mounts: `engine_surface_attach_tests`, `node_graph_attach_tests`, `node_graph_gesture_tests` and `paint2d_engine_tests` stay in the A parent (they address each other as `super::…_tests`, and the included `🧊️wgpu-standalone` file in the parent does too), paths unchanged. `metrics_invalidation_drain_tests`, `catalogue_workflow_drop_tests`, `board2d_engine_tests` moved with their child (`../` added). In B, `render_plan_validator_tests`, `ui_command_wiring_tests`, `app_catalogue_tests`, `introspection_tests` moved with their child, `worker_cell_tests` stays in the parent.
- Path literals: 7 `#[path]` mounts gained `../` (checked by script, 52 path and include literals across the new files all resolve).

## 5. Visibility widening (to `pub(super)`, nothing wider)

| | Items | Impl methods | Struct fields (incl. one tuple field) | `pub(super)` tokens |
|---|---|---|---|---|
| A | 55 | 53 | 240 | 348 |
| B | 103 | 8 | 224 + 1 | 336 |

Items are exact (token level, first path segment, test files included): the seam map's 31 (A) and 71 (B) names are all widened; the extras are items used only by test files (23 in A kept in the parent tests, plus `sync_board_engine`; the `thread_local!` statics `FOCUSED_*`, `INK_CLIPBOARD_STREAMS`, … that the map did not track in B). Methods and fields are **name-level evidence** (a field when its owner and name both occur in another module, or `.name` occurs there; a method when `.name` or `::name` occurs): this over-approximates for generic names such as `len`, `get`, `new`, `map` and can be tightened once a compiler can type-check. Tuple struct `UiEngineCell` got `pub(super)` on its field because the worker-cell test constructs it from the parent. Free `struct` and `enum` definitions moved into children were checked for widening (the splitter blind spot): every cross-module struct and enum carries `pub(super)` or a wider visibility.

## 6. Readers

Done (each re-pointed to `readModuleTreeSource`, or `policyReadModuleTreeSource` in the root script, which wraps the same helper through the symlink-refusing policy read; log in `tools/reader-edits.jsonl`, 20 records):
- `POLICY_SPLIT_SOURCE_ROOTS` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` now lists the A and B parents.
- Root `📜️script.ts`: `INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE` (p3mn gate, was `policyReadFileSafe`) and the `mountedInterpreter` read (now using `INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE`) read the tree. The other root reads of these files go through `policyReadRustPolicySource`, which is registry driven and is now tree-aware for A and B.
- `block_on` allowlist: **no entry exists for A or B** (neither file contains `block_on` or `run_blocking`; `/usr/bin/grep` verified), so nothing was re-keyed. The entry in the seam map belongs to the OS host split.
- TS: `🛑️scene-pointer-cancellation` (two `?raw` imports), `🫳️node-graph-gestures`, `🧪️node-graph-edit-rows`, `🖼️wgpu-raster-residency`, `🔗️node-graph-wire-edit` (2), flow `🧪️source-contract`, `🧲️engine-surface-retention` (also needed the import), `⏱️frame-latency`, UI `♻️physical-job-close` (3 reads), `🧵️worker-cell` test (tree plus the contracts file, and its `UiEngineCell(WorkerCell<…>)` needle now tolerates the widened tuple field). Already tree-aware from the earlier readers task and unchanged: `🔬️targets-wgpu-mounted-layout-unit`, the fixture-keyed owner-move, raster-witness and reconcile tests, `🔬️interactivity-mounted-layout-text`, `🔬️interactivity-mounted-frame-transaction` (registry).
- Rust: `🎞️Scenes/🧪️tests/🧊️wgpu-standalone` and `🧪️tests/🔬️wgpu-renderer-async-boundary` use `crate::interpreter::INTERPRETER_SOURCE` and `crate::engine_canvas::ENGINE_CANVAS_SOURCE`.
- Doc references that named a file and line were updated: paint2d test doc (`2801`), the frame-worker doc, the Cargo.toml comment.

Not done: the generated distribution manifest (`🧑‍💻dev/📤️distribution/🧾️manifest.json`, entries for A and B) still lists the old file bytes and hashes, regenerate it with its generator. Comment-only references in `time-travel`, `ShellHost/🎯️input-ledger`, `TaskManager` and the puzzle plugin engine doc still name the old files (no code effect).

## 7. Verification actually run

| Check | Command or method | Result |
|---|---|---|
| Baseline | the sanctioned launcher, see section 8 | refused by the source-boundary gate |
| Full-resolution differential | `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null` from the crate lib root `📦️packages/🦀️rust/📚️library/🦀️.rs`; baseline sandbox = live tree with the original A and B, the new folders removed and every recorded edit reversed; current = the live tree; sandbox patches applied to both (external glob imports neutralised, missing `extern crate` removed, 21 missing `include_str!` fixtures stubbed in both so expansion completes); error multisets compared by code, message, span text and label, file and line stripped | native 441, `--test` 464, wasm32-unknown-unknown 615, `--test` wasm32 670 distinct errors, **only in baseline 0 and only in current 0 in all four**. Unresolved-external-crate errors (E0432/E0433) are compared as sets, because the same `use` now sits in several files (that is not a regression) |
| Oracle sensitivity | in the sandbox: removed `pub(super)` from `ENGINE_SURFACES`, dropped one name from a sibling import | native and test configs report new E0603 and E0425 as expected |
| Oracle limit | in the `--test` configurations rustc stops after import resolution (the sandbox has parse errors from a peer in `🧭️wgpu-navbar-footer-parity` and missing crates), so unresolved names inside mounted test bodies are not reported; a probe with an undefined function in a mounted test was not flagged | compensated by `tools/selfcheck.py` (independent re-lex of the written files: every name used by a child, its mounted tests or the included test file that is defined in the target tree or was a header binding must be imported or defined locally): 0 unimported names in A and B; mutation check (removing `focus_ink_editor` from a test import) is caught |
| Definition scan | `tools/defscan2.py`: every private item, method or field with a same-named identifier in another module | 0 for B; 1 for A (`tutorial_surface_geometry`, a fn whose name equals the module name; different namespaces, fine) |
| Losslessness | `tools/verify.py`: multiset of trimmed non-blank lines, `pub(super) ` and `../` normalised | A: only the 12 header import lines differ (re-expressed as trimmed per-child imports); B: only 5 header import lines. No code line lost |
| Syntax | `rustfmt --edition 2021 --emit stdout` on all 22 new or rewritten files and on the 21 edited Rust files | all exit 0 |
| Paths | script over `#[path]`, `include!`, `include_str!` literals | 52 checked, 0 missing |
| Length | longest repo-relative path | 198 bytes, limit 240 |
| Emoji | `bun tools/em.ts` with `/^\p{RGI_Emoji}$/v`, shape (emoji, one U+FE0F, kebab) and sibling uniqueness | 20 folders all RGI, unique among siblings |
| Region markers | script | balanced in every new file |
| Cargo | `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short` | refused: "Rust crate Source boundary exceeded before compiler byte allocation" (also before the edit); `--tests` and wasm variants not run |
| TS readers | `bun test ./<file>` before and after (logs in `ts-before/`, `ts-after/`) | pass/fail identical for all 12 files with identical failing-test sets (pre-existing failures unchanged: scene-pointer-cancellation 4/1, gestures 11/1, raster-residency 4/1, source-contract 0/1, engine-surface-retention 11/1, frame-latency 2/1, owner-move 7/4, raster-witness 4/7); wire-edit 10/0, mounted-layout-unit 5/0, physical-job-close 9/0, worker-cell 5/0 (the after-run first showed physical-job-close 7/2 and worker-cell 4/1, caused by two further single-file reads and the widened tuple field; fixed and re-run green) |
| Registry guard | `bun test` of `🔬️rust-policy-source-evidence` | 6 pass, 0 fail |

Not run: `bun ./📜️script.ts verify interactivity …` gates (they fail earlier on peer state, per the earlier readers result), native and wasm cargo builds, the Rust tests themselves.

## 8. Not done and known risks

- No compile. Privacy of struct fields and methods (E0616, E0624) and trait-method scoping are not covered by the oracle (typeck never runs here); they are covered by the name-level widening and the scans above. No in-crate trait is defined in A or B and no header import is a trait, so trait scoping cannot differ.
- Unused-import warnings are possible: gating uses the common cfg of all uses, so a name used in two differently gated places is imported ungated.
- The 24 test-only widenings in A could be avoided by mounting the four cross-referencing tests in children; they were kept in the parent because they address each other as `super::…_tests`.
- `EngineSurface` and similar hub types have wide `pub(super)` field surfaces; narrowing is a later pass with a compiler.

## 9. Wrong-direction items (recorded, not fixed)

A (framework element naming concrete plugin or product code):
- Plugin crate imports: `flow::{dag::{dag_screen_to_world, dag_world_to_screen}, FlowHost}` (children `🗂️engine-surface-registry`, `🧩️engine-surface-attach`, `🕸️node-graph`, `🎓️tutorial-surface-geometry`), `framework_editor::EditorHost` (`✍️text-editor`, registry, attach), `semio_framework_os_node_graph::GraphHost` (`🕸️node-graph`, registry, attach), `infinite_world::world::tool_run_trace` (registry, attach).
- Plugin and note state names: `puzzle_board_*` (`🎲️board2d`), `puzzle3d_catalogue_drag_payload*`, `FLOW_WIDGET_DRAG_MIME` (`🕸️node-graph`), `last_note_click` and `editing_note_id` (`🕸️node-graph`, registry).
- The wasm export `semioWgpuHostPageHidden` (`✍️text-editor`) is host plumbing inside an element.

B:
- `infinite_world` imports in `🖱️scene-interaction`, `🖼️ui-image-loading`, `🧮️render-plan-validator`; `gisMap.*` labels in the validator; `world3d` names in five children; `usvg`/`resvg` named directly in `🖼️ui-image-loading`.
- Diagnostics compiled into the production element: the six `#[wasm_bindgen]` dump exports (`dumpStructure`, `dumpFrameStats`, `dumpMeshStats`, `dumpBoard2d`, `dumpChrome`, `dumpAccessibility`) and all accessibility and ledger builders in `🔬️introspection`, `🧱️mesh-and-board2d-stats`, `🎯️chrome-ledger`.

Cross-element: the B and Scenes function-level cycle described in section 1.
