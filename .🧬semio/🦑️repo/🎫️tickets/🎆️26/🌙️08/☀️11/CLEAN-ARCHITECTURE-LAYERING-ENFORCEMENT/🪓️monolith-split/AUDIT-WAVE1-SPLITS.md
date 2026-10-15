# Wave 1 Monolith Split Audit (read-only)

Date: 2026-10-15. Scope: the seven RESULT-*-SPLIT.md units in this folder (renderer, scenes, shell, infinite world/board-normal/board-dag, manifest, plugin instance, plugin host, hub bootstrap). Read-only: no source edits, no git writes, no cargo/nx builds.

Method: each split was re-derived from the saved originals (`🗑️generated/` files; manifest original = `git show HEAD:`, saved to `🗑️generated/audit-wave1/manifest-HEAD.rs`). The parent plus all `#[path]`-reachable descendants were read from the current tree. Losslessness is a multiset comparison of normalized non-blank lines (per the brief). Scripts and raw output: `🗑️generated/audit-wave1/audit.py`, `audit-output.txt`, `buckets.json`, `violations.json`, `emoji-check.ts`.

Limits: no compiler run (the compiler-ingress gate still blocks cargo, per the RESULT reports). The visibility check is a name-level static scan of top-level items only (no methods, fields or macros), so it reports candidates; each candidate was then read in the source. Expected rewrites (path depth, wrapper removal, tuple-field widening, `impl` bands) are listed as expected, not as failures.

## Summary

| Split | Tree files | Lines missing / extra (multiset) | Path literals | Folder emoji | Child headers | Largest child (active lines) | DEBUG / scratch | Visibility scan | Verdict |
|---|---|---|---|---|---|---|---|---|---|
| renderer (`🧊️renderer/`) | 17 | 35 / 30 | 90, 0 unresolved | 16/16 | 16/16 | 2286 (`🦴️glb-decode`) | 0 / 0 | 2 candidates, both false positives | PASS (static) |
| scenes (`🎯️targets/🧊️wgpu/`) | 16 | 1 / 1 | 31, 0 unresolved | 15/15 | 15/15 | 1923 (`✒️ink-canvas-model-state`) | 0 / 0 | 0 | PASS (static) |
| shell (`🧊️wgpu/`) | 21 | 3 / 25 | 80, 0 unresolved | 20/20 | 20/20 | 1976 (`📨️command-dispatch`) | 0 / 0 | 0 | PASS (static) |
| infinite A world (`🌍️world/`) | 14 | 0 / 0 | 20, 0 unresolved | 13/13 | 13/13 | 2129 (parent) | 0 / 0 | 118 private items used outside their module | FAIL |
| infinite B normal (`➕️normal/`) | 7 | 3 / 7 | 7, 0 unresolved | 6/6 | 6/6 | 2348 (parent) | 0 / 0 | 26 private items used outside their module | FAIL |
| infinite C dag (`🕸️dag/`) | 6 | 1 / 5 | 10, 0 unresolved | 5/5 | 5/5 | 1333 (parent) | 0 / 0 | 0 | PASS (static) |
| manifest (`🛂️manifest/`) | 8 | 2 / 39 | 33, 0 unresolved | 7/7 (incl. 1 peer child) | 7/7 | 1637 (`🧮️arg-schema`) | 0 / 0 | 0 | PASS with WARN (peer edits) |
| plugin instance (`🧵️instance/`) | 7 | 7 / 15 | 19, 0 unresolved | 6/6 | 6/6 | 1973 (`🚪️ingress`) | 0 / 0 | 0 | FAIL (7 undocumented rewritten lines) |
| plugin host (`🖥️host/`) | 4 | 1 / 1 | 40, 0 unresolved | 3/3 | 3/3 | 2106 (`🔀️post-turn-relay`) | 0 / 0 | 0 | PASS (static) |
| hub bootstrap (`🏗️bootstrap/`) | 23 | 95 / 588 | 28, 0 unresolved | 22/22 | 22/22 | 1162 (`🛠️admin`) | 0 / 0 | 0 | FAIL (36 region markers dropped) |

Folder emoji: all 113 new child folder names start with an RGI emoji followed by U+FE0F (bun `/\p{RGI_Emoji}/v`). All 113 child files start with `//!` followed by an RGI emoji. No duplicate emoji among siblings created by a split. Pre-existing sibling duplicates exist and are noted below. No file exceeds 2,500 active lines (active = non-blank, non-`//`). `[DEBUG]` hits: 0 in every tree and every original.

Verdicts: PASS (static) means no lossless or visibility defect was found by static analysis, but nothing was compiled. FAIL means a defect that a compiler will reject (infinite A, infinite B) or an undocumented change that the brief does not allow (plugin instance, hub).

## Per-split findings

### renderer: PASS (static)

Missing 35 original lines, all expected: 20 `///` module-doc lines converted to `//!` (orig 4409-4420 and 9947-9956; the text is present in `📦️kernel-runtime/🦀️.rs` and `📊️scale-bench/🦀️.rs`); wrappers `mod kernel_runtime {` (orig 4423) and `pub mod scale_bench {` (orig 9958) plus 2 closing braces (orig 274, 279); `#[path = "."]` (ignored by the normalizer); 4 include paths re-based (orig 9822 to `📦️kernel-runtime/🏊️kernel-pool/🦀️.rs:2152`, orig 9922 to `🏊️kernel-pool/🦀️.rs:2252`, orig 10099 to `📊️scale-bench/🦀️.rs:153`, orig 19015 to `🧠️app-runtime/🦀️.rs:1370`); 5 tuple fields widened to `pub(crate)` (orig 12347, 12421, 12992, 19111, 19190; now `🎨️presentation-raster/🦀️.rs:24, 98, 669` and `🔬️native-socket-probe/🦀️.rs:6, 85`).

Extra 30: the new `#[cfg(test)] const RENDERER_SOURCE` block at `🦀️.rs:265-285` (concat of 17 `include_str!`), the 5 tuple-field widenings, and the re-based include lines. No peer edits detected.

Visibility: 2 raw candidates `🦴️glb-decode/🦀️.rs:1454` and `:1456` (`row`). These are a local loop variable (`for row in 0..4`) shadowing `fn row` at `📊️scale-bench/🦀️.rs:116`. False positive, not a defect.

### scenes: PASS (static)

Missing orig:2063 `include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs")` re-based to `📷️scene-camera-dispatch/🦀️.rs:203` (`../../../`). Extra 1 is that same line. Visibility: 0. Largest child 1923 active lines. Parent 398 lines total (338 active).

### shell: PASS (static)

Missing 3, all expected: orig:813 include_str re-based to `🔖️identity-and-types/🦀️.rs:679`; orig:34149 `SHELL_THEME_DOCUMENT_JSON` re-based to `🎨️theme-file-import-and-accessibility/🦀️.rs:688` (10 `../`); orig:3598 `struct PresentedInputCandidateWitness(u64);` now `pub(crate) u64` at `🏛️space-admin-and-settle/🦀️.rs:942`.

Extra 25: 5 new `impl ShellState {` bands (`⚓️dock-model-and-measures/🦀️.rs:950`, `🧩️session-and-topology/🦀️.rs:11`, `⚙️settings-and-dock-persistence/🦀️.rs:12`, `🎬️shell-actions/🦀️.rs:32`, `📨️command-dispatch/🦀️.rs:13`) with their closing braces; 12 `//#region` markers added in the children; 2 re-based includes; 1 widened tuple field. No peer edits detected in the split text.

### infinite A world: FAIL (visibility)

Losslessness: 0 missing, 0 extra. Structure is clean.

Visibility: 118 candidates, all `private` items used from a module outside their own subtree. Examples (verified by reading source): `🦀️.rs:1236` `interaction_id: WorldInteractionId` (defined `🧵️interaction/📇️registry/🦀️.rs:16`, `struct WorldInteractionId {` with no `pub`); `🦀️.rs:2023` uses `WorldBrushMeshRun` (`📡️assets/🦀️.rs:737`); `🦀️.rs:2095` uses `WorldFaceOverlayMeshCursor` (`🖌️overlay/🦀️.rs:616`); sibling `🧵️interaction/🔍️marquee/🦀️.rs:19` uses `WorldInteractionObjectToken` (`🧵️interaction/📇️registry/🦀️.rs:189`). RESULT-INFINITE-SPLIT.md says items used outside a child get `pub(super)` or `pub(in crate::world)`. These items are still private, so the widening was not applied to them. Full list: Appendix A.

### infinite B normal: FAIL (visibility)

Losslessness: 3 missing (orig:3 `pub mod board_host {` wrapper, orig:3066 include re-based to `🧭️host/🦀️.rs:419`, one preamble line) and 7 extra: 3 `impl BoardHost {` bands (`🎨️style/🦀️.rs:96, 568`, `🪣️fill-job/🦀️.rs:5`), 2 closing braces, the re-based include at `🧭️host/🦀️.rs:419`, and one preamble line. All expected.

Visibility: 26 candidates, all `private`. Examples: `🦀️.rs:229` `node_kind_id: BoardPointerSpan`, where `struct BoardPointerSpan {` at `➕️normal/🧭️host/🦀️.rs:116` is private (its `pub(super)` fields at lines 123 and 139 do not widen the type); `🎨️style/🦀️.rs:327` uses `BoardEntityRetirement` (`🧭️host/🦀️.rs:231`, private); `🦀️.rs:2638` uses `BoardDeletePlanningOperation` (`🧭️host/🦀️.rs:178`). Full list: Appendix B.

### infinite C dag: PASS (static)

Missing 1: orig:5222 `mod wasm_session {` wrapper (expected). Extra 5: 3 `impl DagHost {` bands at `🧭️host-selection/🦀️.rs:5`, `🧱️frame-minimap-align/🦀️.rs:5`, `🛠️mutate/🦀️.rs:5`, and closing braces. Visibility: 0.

### manifest: PASS with WARN (peer edits)

Missing 2 expected include re-bases: orig:1237 to `🧮️arg-schema/🦀️.rs:1128` (`../🔣️input-labels.json`), orig:1349 to `🧮️arg-schema/🦀️.rs:1240` (`../../🌱️value/🧬️schema/🔣️.json`). Original = `git show HEAD:` (8138 lines, matching the RESULT report).

Extra 39: 15 lines attributable to `📇️space-index/🦀️.rs` (27 lines, untracked, peer-created, not part of this split; RESULT-MANIFEST-SPLIT.md mentions the peer mount), 18 region-marker lines, 4 `impl` or closing-brace lines, 2 re-based includes, and 4 lines typed as other. The peer child starts with `//!` and an emoji and passes the header check.

Notes (not defects): sibling emoji duplicates `🔖️action-args` and `🔖️channel-version` under `🛂️manifest/` are pre-existing, not created by the split. `🟦️.ts` at the parent level is tracked with 22 uncommitted insertions and is not split scratch. `🔣️input-labels.json` is a tracked asset required by `include_str!`.

### plugin instance: FAIL (undocumented rewritten lines)

Losslessness: 7 original lines were rewritten. `store::ChildRestoreProjection::from_snapshot(...)` (orig:2033, 2073, 2641, 2686, 2909, 3430, 4857) is now `semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(...)` at `🚪️ingress/🦀️.rs:834, 874, 1442, 1487, 1710, 2231` and `📨️dispatch/🦀️.rs:1319`. The RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md report does not mention this change. The crate is declared in `🔌️plugin/📦️packages/🦀️rust/Cargo.toml`, so the path may resolve. Equivalence is not demonstrated (the original resolved through the parent `use store::{…}` at `🦀️.rs:68`). Needs a compile or a documented justification.

Other: path check passed for all 19 literals. Extra: 4 `impl` bands (`🦀️.rs:938`, `🚪️ingress/🦀️.rs:5`, `📨️dispatch/🦀️.rs:5`, `🖼️render/🦀️.rs:5`) and closing braces. Visibility: 0.

### plugin host: PASS (static)

Missing 1: orig:5164 include re-based to `🔀️post-turn-relay/🦀️.rs:2367` (`../⚡️effects/...`). Extra 1 is the same line. Visibility: 0. Pre-existing siblings under `🖥️host/` share emoji (`📥️` x3, `🧪️` x2, `🧬️` x2); the three new folders `🎭️guest-runtime`, `🔀️post-turn-relay`, `🧭️routers` are unique.

### hub bootstrap: FAIL (region markers dropped)

Region markers: all 36 `//#region` / `//#endregion` lines of the original (for example orig:112, 199, 750, 2595, 11963) are absent from the entire `🌎️hub/🏗️bootstrap/` tree (grep count 0). The RESULT-HUB-BOOTSTRAP-SPLIT.md report does not mention this. Comment-only, but the brief requires every non-blank line to be accounted for.

Other findings, documented or expected: 44 `use` preamble lines redistributed to the 22 children; include re-based at `🔑️access-grants/🦀️.rs:635` (orig:3956); 4 tuple fields widened (`🧰️http-support/🦀️.rs:17`, `🚀️startup/🦀️.rs:188`, `🧭️open-plan/🦀️.rs:27`, `📇️directory-commands/🦀️.rs:80`). Two `#[cfg(feature = "integration-fixtures")]` lines (orig:84, 86) were reformatted to `#[cfg(feature="…")]` at `🦀️.rs:31` and `🦀️.rs:100` (whitespace only).

Not in the report: 55 root preamble `use` lines are now `#[cfg(test)]`-gated (for example `🦀️.rs:14, 42, 44, 46`), and `🔑️access-grants/🦀️.rs:330` gets `#[cfg(not(test))]`. A static scan found no non-test use of the test-only names in the root; the one hit (`🦀️.rs:443`) is the qualified `tokio::sync::Mutex`. Compiling the non-test bin is still needed to confirm.

## Cross-child reference spot checks (10 per split)

Seeded random sample (seed 20261015) over all cross-file references to a uniquely defined item. Verdict per row: a `pub`, `pub(crate)` or `pub(super)` item is OK when reachable from the referencing file; a `private` item is OK only when the referencing file is inside its module.

### renderer

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `📦️kernel-runtime/🏊️kernel-pool/🦀️.rs:1741: reserve_seq` | 📦️kernel-runtime/🧮️kernel-core/🦀️.rs:71 | `📦️kernel-runtime/🧮️kernel-core/🦀️.rs:71` | pub(super) | OK |
| `📮️mailbox-impl/🦀️.rs:527: RendererAssetDecodeTurnKind` | 🔁️asset-decode-jobs/🦀️.rs:256 | `🔁️asset-decode-jobs/🦀️.rs:256` | pub(crate) | OK |
| `📮️mailbox-impl/🦀️.rs:495: RendererAssetDecodeTurnKind` | 🔁️asset-decode-jobs/🦀️.rs:256 | `🔁️asset-decode-jobs/🦀️.rs:256` | pub(crate) | OK |
| `🖥️frame-presentation/🦀️.rs:147: frame_receive_close` | 🧾️frame-build/🦀️.rs:590 | `🧾️frame-build/🦀️.rs:590` | pub(crate) | OK |
| `🖥️frame-presentation/🦀️.rs:457: frame_inline_gate` | 🧾️frame-build/🦀️.rs:568 | `🧾️frame-build/🦀️.rs:568` | pub(crate) | OK |
| `🦴️glb-decode/🦀️.rs:1678: RendererAssetFetchOwner` | 🗂️asset-authority/🦀️.rs:75 | `🗂️asset-authority/🦀️.rs:75` | pub(crate) | OK |
| `📦️kernel-runtime/🏊️kernel-pool/🦀️.rs:124: mounted_product_replay_worker_slot` | 📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs:931 | `📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs:931` | pub(super) | OK |
| `📦️kernel-runtime/🏊️kernel-pool/🦀️.rs:222: MountedProductReplayCheckpoint` | 📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs:727 | `📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs:727` | pub(super) | OK |
| `🔁️asset-decode-jobs/🦀️.rs:178: RendererAssetFormatCursor` | 🗂️asset-authority/🦀️.rs:259 | `🗂️asset-authority/🦀️.rs:259` | pub(crate) | OK |
| `🔁️asset-decode-jobs/🦀️.rs:23: GlbMaterializeCursor` | 🦴️glb-decode/🦀️.rs:1642 | `🦴️glb-decode/🦀️.rs:1642` | pub(crate) | OK |

### scenes

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `⌨️scene-input/🦀️.rs:1061: hit_double_click_target` | 🧩️board2d-and-vfs/🦀️.rs:569 | `🧩️board2d-and-vfs/🦀️.rs:569` | pub(crate) | OK |
| `⌨️scene-input/🦀️.rs:979: write_scene_action` | 📷️scene-camera-dispatch/🦀️.rs:288 | `📷️scene-camera-dispatch/🦀️.rs:288` | pub(crate) | OK |
| `📰️event-feed-and-timeline/🦀️.rs:965: scene_action` | 📷️scene-camera-dispatch/🦀️.rs:238 | `📷️scene-camera-dispatch/🦀️.rs:238` | pub(crate) | OK |
| `🖌️canvas2d/🦀️.rs:990: RasterUploadRefusal` | 🪣️pending-raster/🦀️.rs:14 | `🪣️pending-raster/🦀️.rs:14` | pub(crate) | OK |
| `🦀️.rs:129: AdmittedSurfaceMap` | 🗃️scene-state/🦀️.rs:42 | `🗃️scene-state/🦀️.rs:42` | pub | OK |
| `🗃️scene-state/🦀️.rs:688: PENDING_RASTER_STATE` | 🦀️.rs:137 | `🦀️.rs:137` | private | OK |
| `🦀️.rs:386: SceneSurfaceState` | 🗃️scene-state/🦀️.rs:564 | `🗃️scene-state/🦀️.rs:564` | pub(crate) | OK |
| `🖋️ink-canvas-render/🦀️.rs:55: InkBoundsF` | ✒️ink-canvas-model-state/🦀️.rs:89 | `✒️ink-canvas-model-state/🦀️.rs:89` | pub(crate) | OK |
| `⌨️scene-input/🦀️.rs:955: mutate_scene_state` | 🦀️.rs:389 | `🦀️.rs:389` | private | OK |
| `✒️ink-canvas-model-state/🦀️.rs:978: InkEditKind` | 🗃️scene-state/🦀️.rs:348 | `🗃️scene-state/🦀️.rs:348` | pub(crate) | OK |

### shell

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🖱️pointer-input/🦀️.rs:1516: ShellPaletteKind` | 🔖️identity-and-types/🦀️.rs:1351 | `🔖️identity-and-types/🦀️.rs:1351` | pub(crate) | OK |
| `🖱️pointer-input/🦀️.rs:417: panel_resize_anchor` | 🧱️chrome-paint-and-navbar/🦀️.rs:315 | `🧱️chrome-paint-and-navbar/🦀️.rs:315` | pub(crate) | OK |
| `💬️chrome-dialog-and-tooltips/🦀️.rs:267: AgentApprovalPaintOp` | 🗨️chrome-overlays-and-tour/🦀️.rs:382 | `🗨️chrome-overlays-and-tour/🦀️.rs:382` | pub(crate) | OK |
| `💬️chrome-dialog-and-tooltips/🦀️.rs:932: UI_INTRODUCTION_VEIL_CONTROL_ID` | 🔑️storage-prefs-and-i18n/🦀️.rs:41 | `🔑️storage-prefs-and-i18n/🦀️.rs:41` | pub(crate) | OK |
| `🦀️.rs:927: TutorialRuntime` | 🎓️tutorial-runtime/🦀️.rs:44 | `🎓️tutorial-runtime/🦀️.rs:44` | pub | OK |
| `🧩️session-and-topology/🦀️.rs:218: ShellDocumentRetirementRegistry` | 🧵️shell-async-runners/🦀️.rs:706 | `🧵️shell-async-runners/🦀️.rs:706` | pub(crate) | OK |
| `🧾️panel-records/🦀️.rs:390: measure_bindings` | ⚓️dock-model-and-measures/🦀️.rs:944 | `⚓️dock-model-and-measures/🦀️.rs:944` | pub(crate) | OK |
| `🖱️pointer-input/🦀️.rs:49: window_pane_owner` | ⚓️dock-model-and-measures/🦀️.rs:715 | `⚓️dock-model-and-measures/🦀️.rs:715` | pub(crate) | OK |
| `🧭️navbar-footer-and-overlays/🦀️.rs:1203: chrome_icon` | 🧱️chrome-paint-and-navbar/🦀️.rs:79 | `🧱️chrome-paint-and-navbar/🦀️.rs:79` | pub(crate) | OK |
| `🧭️navbar-footer-and-overlays/🦀️.rs:374: surface_status_pills_for` | 🧱️chrome-paint-and-navbar/🦀️.rs:1543 | `🧱️chrome-paint-and-navbar/🦀️.rs:1543` | pub(crate) | OK |

### infinite A

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🖌️overlay/🦀️.rs:1594: World3dState` | 🦀️.rs:1898 | `🦀️.rs:1898` | pub | OK |
| `🖌️overlay/🦀️.rs:1527: WorldNativeMeshFailure` | 🦀️.rs:1242 | `🦀️.rs:1242` | pub | OK |
| `🧵️interaction/🔍️marquee/🦀️.rs:19: WorldInteractionObjectToken` | 🧵️interaction/📇️registry/🦀️.rs:189 | `🧵️interaction/📇️registry/🦀️.rs:189` | private | FAIL (private item used outside its module) |
| `🧵️interaction/🔍️marquee/🦀️.rs:638: INTERACTION_TARGETS_OPEN` | 🧵️interaction/⚙️authority/🦀️.rs:1340 | `🧵️interaction/⚙️authority/🦀️.rs:1340` | pub(super) | OK |
| `🦀️.rs:2095: WorldFaceOverlayMeshCursor` | 🖌️overlay/🦀️.rs:616 | `🖌️overlay/🦀️.rs:616` | private | FAIL (private item used outside its module) |
| `🧵️interaction/⚙️authority/🦀️.rs:1011: WorldInteractionActive` | 🧵️interaction/📇️registry/🦀️.rs:716 | `🧵️interaction/📇️registry/🦀️.rs:716` | pub(in crate::world) | OK |
| `🧵️interaction/⚙️authority/🦀️.rs:1030: WorldInteractionStep` | 🧵️interaction/🧲️picking/🦀️.rs:89 | `🧵️interaction/🧲️picking/🦀️.rs:89` | pub | OK |
| `🧹️lifecycle/🦀️.rs:427: WorldDrawRebuildStep` | 🦀️.rs:1518 | `🦀️.rs:1518` | pub | OK |
| `🧹️lifecycle/🦀️.rs:378: World3dState` | 🦀️.rs:1898 | `🦀️.rs:1898` | pub | OK |
| `🧵️interaction/🕹️gumball/🦀️.rs:462: WorldCancelReason` | 🧵️interaction/📇️registry/🦀️.rs:498 | `🧵️interaction/📇️registry/🦀️.rs:498` | pub | OK |

### infinite B

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🪣️fill-job/🦀️.rs:519: BoardFillIngressHandleText` | 🦀️.rs:723 | `🦀️.rs:723` | pub | OK |
| `🪣️fill-job/🦀️.rs:458: BoardFillText` | 🦀️.rs:345 | `🦀️.rs:345` | pub | OK |
| `🎛️selection-events/🦀️.rs:2044: BOARD_POINTER_ITEM_CAPACITY` | 🧭️host/🦀️.rs:79 | `🧭️host/🦀️.rs:79` | pub | OK |
| `🧹️scene-delete-links/🦀️.rs:760: BoardDrawLod` | 🦀️.rs:151 | `🦀️.rs:151` | private | OK |
| `🦀️.rs:2642: BoardPointerPlanRetirement` | 🧭️host/🦀️.rs:810 | `🧭️host/🦀️.rs:810` | pub | OK |
| `🎨️style/🦀️.rs:321: BoardHostClosePhase` | 🧭️host/🦀️.rs:40 | `🧭️host/🦀️.rs:40` | pub(super) | OK |
| `🧭️host/🦀️.rs:1004: BOARD_EVENT_KEY_BYTE_CAPACITY` | 🦀️.rs:1609 | `🦀️.rs:1609` | pub | OK |
| `🧹️scene-delete-links/🦀️.rs:2060: BoardOwnedEvent` | 🦀️.rs:1722 | `🦀️.rs:1722` | pub | OK |
| `🪣️fill-job/🦀️.rs:335: BoardFillCapturePhase` | 🦀️.rs:688 | `🦀️.rs:688` | private | OK |
| `🧹️scene-delete-links/🦀️.rs:1334: admitted_board_pointer_id` | 🧭️host/🦀️.rs:996 | `🧭️host/🦀️.rs:996` | pub(super) | OK |

### infinite C

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🛠️mutate/🦀️.rs:224: dag_draw_lod` | 🦀️.rs:1076 | `🦀️.rs:1076` | pub | OK |
| `🛠️mutate/🦀️.rs:168: point_in_rect` | 🦀️.rs:539 | `🦀️.rs:539` | private | OK |
| `🖌️paint/🦀️.rs:466: dag_handle_body_fill` | 🦀️.rs:1008 | `🦀️.rs:1008` | pub(crate) | OK |
| `🖌️paint/🦀️.rs:590: preview_content_bounds` | 🦀️.rs:592 | `🦀️.rs:592` | private | OK |
| `🧭️host-selection/🦀️.rs:182: DagInteractionPlanFault` | 🦀️.rs:1131 | `🦀️.rs:1131` | pub | OK |
| `🧭️host-selection/🦀️.rs:209: DagInteractionPlanFault` | 🦀️.rs:1131 | `🦀️.rs:1131` | pub | OK |
| `🧭️host-selection/🦀️.rs:207: DagPointerIntent` | 🦀️.rs:1146 | `🦀️.rs:1146` | pub | OK |
| `🛠️mutate/🦀️.rs:163: DAG_COMPUTATION_HEADER_ROWS` | 🦀️.rs:99 | `🦀️.rs:99` | private | OK |
| `🖌️paint/🦀️.rs:821: dag_node_label_fill` | 🦀️.rs:985 | `🦀️.rs:985` | pub(crate) | OK |
| `🖌️paint/🦀️.rs:371: slider_track_bounds` | 🦀️.rs:573 | `🦀️.rs:573` | private | OK |

### manifest

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🖼️view-model/🦀️.rs:604: TopicContribution` | 🧾️plugin-manifest/🦀️.rs:15 | `🧾️plugin-manifest/🦀️.rs:15` | pub | OK |
| `🕹️actions/🦀️.rs:522: ActionKind` | 🧳️product-vocabulary/🦀️.rs:15 | `🧳️product-vocabulary/🦀️.rs:15` | pub | OK |
| `🕹️actions/🦀️.rs:510: ActionKind` | 🧳️product-vocabulary/🦀️.rs:15 | `🧳️product-vocabulary/🦀️.rs:15` | pub | OK |
| `🕹️actions/🦀️.rs:880: ActionKind` | 🧳️product-vocabulary/🦀️.rs:15 | `🧳️product-vocabulary/🦀️.rs:15` | pub | OK |
| `🧳️product-vocabulary/🦀️.rs:149: ActionDefinition` | 🕹️actions/🦀️.rs:344 | `🕹️actions/🦀️.rs:344` | pub | OK |
| `🦀️.rs:287: ActionSemantics` | 🕹️actions/🦀️.rs:245 | `🕹️actions/🦀️.rs:245` | pub | OK |
| `🧱️app-surface/🦀️.rs:897: ActionDefinition` | 🕹️actions/🦀️.rs:344 | `🕹️actions/🦀️.rs:344` | pub | OK |
| `🧱️app-surface/🦀️.rs:898: ActionKind` | 🧳️product-vocabulary/🦀️.rs:15 | `🧳️product-vocabulary/🦀️.rs:15` | pub | OK |
| `🦀️.rs:327: ApprovalMode` | 🕹️actions/🦀️.rs:77 | `🕹️actions/🦀️.rs:77` | pub | OK |
| `🦀️.rs:302: ActionKind` | 🧳️product-vocabulary/🦀️.rs:15 | `🧳️product-vocabulary/🦀️.rs:15` | pub | OK |

### plugin instance

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🧷️command-routes/🦀️.rs:74: time_travel_freezes_reserved_action` | 🦀️.rs:1112 | `🦀️.rs:1112` | private | OK |
| `🖼️render/🦀️.rs:915: MediaExportPreparation` | 🦀️.rs:521 | `🦀️.rs:521` | private | OK |
| `🔌️plugin-app-impl/🦀️.rs:381: VcsArtifactApp` | 🦀️.rs:573 | `🦀️.rs:573` | pub | OK |
| `🔌️plugin-app-impl/🦀️.rs:1376: VcsArtifactApp` | 🦀️.rs:573 | `🦀️.rs:573` | pub | OK |
| `🚪️ingress/🦀️.rs:5: VcsArtifactApp` | 🦀️.rs:573 | `🦀️.rs:573` | pub | OK |
| `🔌️plugin-app-impl/🦀️.rs:87: VcsArtifactApp` | 🦀️.rs:573 | `🦀️.rs:573` | pub | OK |
| `🔌️plugin-app-impl/🦀️.rs:1472: VcsArtifactApp` | 🦀️.rs:573 | `🦀️.rs:573` | pub | OK |
| `🚪️ingress/🦀️.rs:299: time_travel_frozen_fault` | 🦀️.rs:1117 | `🦀️.rs:1117` | private | OK |
| `🚪️ingress/🦀️.rs:298: time_travel_freezes_reserved_action` | 🦀️.rs:1112 | `🦀️.rs:1112` | private | OK |
| `🧾️typed-operations/🦀️.rs:5: VcsArtifactApp` | 🦀️.rs:573 | `🦀️.rs:573` | pub | OK |

### plugin host

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🎭️guest-runtime/🦀️.rs:1064: EpochDeadlines` | 🦀️.rs:270 | `🦀️.rs:270` | pub(crate) | OK |
| `🎭️guest-runtime/🦀️.rs:1064: plugin_host_worker_pool` | 🦀️.rs:202 | `🦀️.rs:202` | pub(crate) | OK |
| `🎭️guest-runtime/🦀️.rs:1400: store_compiled_component` | 🦀️.rs:522 | `🦀️.rs:522` | private | OK |
| `🎭️guest-runtime/🦀️.rs:1435: EpochDeadlineCell` | 🦀️.rs:294 | `🦀️.rs:294` | pub(crate) | OK |
| `🦀️.rs:832: ActorHostState` | 🎭️guest-runtime/🦀️.rs:711 | `🎭️guest-runtime/🦀️.rs:711` | pub(super) | OK |
| `🦀️.rs:834: ActorHostState` | 🎭️guest-runtime/🦀️.rs:711 | `🎭️guest-runtime/🦀️.rs:711` | pub(super) | OK |
| `🦀️.rs:833: ActorHostState` | 🎭️guest-runtime/🦀️.rs:711 | `🎭️guest-runtime/🦀️.rs:711` | pub(super) | OK |
| `🔀️post-turn-relay/🦀️.rs:672: GuestRuntimes` | 🎭️guest-runtime/🦀️.rs:1675 | `🎭️guest-runtime/🦀️.rs:1675` | pub | OK |
| `🧭️routers/🦀️.rs:1119: PluginInstanceHandle` | 🔀️post-turn-relay/🦀️.rs:2202 | `🔀️post-turn-relay/🦀️.rs:2202` | pub | OK |
| `🔀️post-turn-relay/🦀️.rs:376: GuestInstance` | 🎭️guest-runtime/🦀️.rs:54 | `🎭️guest-runtime/🦀️.rs:54` | pub | OK |

### hub bootstrap

| Referencing file:line | Item | Defined at | Visibility | Verdict |
|---|---|---|---|---|
| `🪞️pair-blobs/🦀️.rs:25: authorized_for_blob` | 🛡️guard/🦀️.rs:102 | `🛡️guard/🦀️.rs:102` | pub(crate) | OK |
| `🔑️access-grants/🦀️.rs:512: SocketBindingValidityV1` | 🎟️socket-grants/🦀️.rs:148 | `🎟️socket-grants/🦀️.rs:148` | pub(crate) | OK |
| `📣️directory-events/🦀️.rs:704: SocketBindingValidityV1` | 🎟️socket-grants/🦀️.rs:148 | `🎟️socket-grants/🦀️.rs:148` | pub(crate) | OK |
| `🪪️session-agent/🦀️.rs:116: SocketBindingKeyV1` | 🎟️socket-grants/🦀️.rs:33 | `🎟️socket-grants/🦀️.rs:33` | pub(crate) | OK |
| `🦀️.rs:274: HubMode` | 🩺️readiness/🦀️.rs:26 | `🩺️readiness/🦀️.rs:26` | pub(crate) | OK |
| `🚀️startup/🦀️.rs:256: now_ms` | 🧰️http-support/🦀️.rs:32 | `🧰️http-support/🦀️.rs:32` | pub(crate) | OK |
| `🦀️.rs:458: verify_pending_catalog_guests` | 🚀️startup/🦀️.rs:169 | `🚀️startup/🦀️.rs:169` | pub(crate) | OK |
| `🛣️routes/🦀️.rs:22: get_space_artifact_creation_catalog` | 🌱️artifact-creation/🦀️.rs:499 | `🌱️artifact-creation/🦀️.rs:499` | pub(crate) | OK |
| `🔑️access-grants/🦀️.rs:440: SocketAudienceV1` | 🎟️socket-grants/🦀️.rs:42 | `🎟️socket-grants/🦀️.rs:42` | pub(crate) | OK |
| `🛠️admin/🦀️.rs:685: AdminPrincipalV1` | 🛡️guard/🦀️.rs:121 | `🛡️guard/🦀️.rs:121` | pub(crate) | OK |

## Appendix A: infinite A visibility candidates (118)

Module base: `🌍️world/`. Format: referencing file:line, item, defining file:line, visibility (all `private`).

- 🦀️.rs:1236: WorldInteractionId defined 🧵️interaction/📇️registry/🦀️.rs:16 vis=private
- 🦀️.rs:1963: World3dSceneGumballConfigRecord defined 🌉️bridge/🦀️.rs:573 vis=private
- 🦀️.rs:2023: WorldBrushMeshRun defined 📡️assets/🦀️.rs:737 vis=private
- 🦀️.rs:2039: WorldInteractionAuthority defined 🧵️interaction/📇️registry/🦀️.rs:747 vis=private
- 🦀️.rs:2040: WorldInteractionMeshRegistry defined 🧵️interaction/📇️registry/🦀️.rs:53 vis=private
- 🦀️.rs:2041: WorldInteractionObjectRegistry defined 🧵️interaction/📇️registry/🦀️.rs:207 vis=private
- 🦀️.rs:2043: World3dSnapshotApplyCursor defined 🖌️overlay/🦀️.rs:1731 vis=private
- 🦀️.rs:2048: World3dSceneBridgeCursor defined 🌉️bridge/🦀️.rs:706 vis=private
- 🦀️.rs:2049: World3dSceneBridgeCursor defined 🌉️bridge/🦀️.rs:706 vis=private
- 🦀️.rs:2057: World3dCameraFitRequest defined 🌉️bridge/🦀️.rs:288 vis=private
- 🦀️.rs:2058: World3dCameraFitOwner defined 🌉️bridge/🦀️.rs:297 vis=private
- 🦀️.rs:2060: World3dCameraBoundsCursor defined 🌉️bridge/🦀️.rs:303 vis=private
- 🦀️.rs:2080: WorldCameraSync defined 🧹️lifecycle/🦀️.rs:38 vis=private
- 🦀️.rs:2084: World3dPreparedStatus defined 🖌️overlay/🦀️.rs:1708 vis=private
- 🦀️.rs:2090: WorldPlaceholderMeshCursor defined 🧱️scene/🦀️.rs:1397 vis=private
- 🦀️.rs:2091: WorldTerrainMeshCursor defined 🧱️scene/🦀️.rs:753 vis=private
- 🦀️.rs:2092: TerrainTileMeshPayload defined 🧱️scene/🦀️.rs:656 vis=private
- 🦀️.rs:2095: WorldFaceOverlayMeshCursor defined 🖌️overlay/🦀️.rs:616 vis=private
- 🦀️.rs:2110: World3dDynamicRetirement defined 🧹️lifecycle/🦀️.rs:114 vis=private
- 🦀️.rs:2377: WorldInteractionAuthority defined 🧵️interaction/📇️registry/🦀️.rs:747 vis=private
- 🦀️.rs:2378: WorldInteractionMeshRegistry defined 🧵️interaction/📇️registry/🦀️.rs:53 vis=private
- 🦀️.rs:2379: WorldInteractionObjectRegistry defined 🧵️interaction/📇️registry/🦀️.rs:207 vis=private
- 🦀️.rs:2399: WorldCameraSync defined 🧹️lifecycle/🦀️.rs:38 vis=private
- 🖌️overlay/🦀️.rs:1201: WorldGumballGesture defined 🧵️interaction/🕹️gumball/🦀️.rs:147 vis=private
- 🖌️overlay/🦀️.rs:1498: WorldInteractionId defined 🧵️interaction/📇️registry/🦀️.rs:16 vis=private
- 🌉️bridge/🦀️.rs:949: WorldPlaceholderMeshCursor defined 🧱️scene/🦀️.rs:1397 vis=private
- 🎛️frame/🦀️.rs:42: World3dSnapshotApplyCursor defined 🖌️overlay/🦀️.rs:1731 vis=private
- 🖱️pointer/🦀️.rs:551: WorldComponentAddress defined 🧵️interaction/🕹️gumball/🦀️.rs:81 vis=private
- 🖱️pointer/🦀️.rs:1436: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:719: WorldObjectPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:506 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:720: WorldComponentPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:666 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:721: WorldMarqueePickCursor defined 🧵️interaction/🔍️marquee/🦀️.rs:80 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:722: WorldMarqueePublishJob defined 🧵️interaction/🔍️marquee/🦀️.rs:579 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:723: WorldComponentMarqueePublishJob defined 🧵️interaction/🔍️marquee/🦀️.rs:814 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:724: WorldGumballPickCursor defined 🧵️interaction/🕹️gumball/🦀️.rs:122 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:725: WorldGumballCommitJob defined 🧵️interaction/🕹️gumball/🦀️.rs:639 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:726: WorldBrushCommitJob defined 🧵️interaction/🕹️gumball/🦀️.rs:658 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:754: WorldMarqueeGesture defined 🧵️interaction/🔍️marquee/🦀️.rs:5 vis=private
- 🧵️interaction/📇️registry/🦀️.rs:755: WorldGumballGesture defined 🧵️interaction/🕹️gumball/🦀️.rs:147 vis=private
- 🧵️interaction/🔍️marquee/🦀️.rs:19: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🔍️marquee/🦀️.rs:20: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🔍️marquee/🦀️.rs:91: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🔍️marquee/🦀️.rs:849: WorldComponentTarget defined 🧵️interaction/🕹️gumball/🦀️.rs:7 vis=private
- 🧵️interaction/🔍️marquee/🦀️.rs:850: WorldInteractionText defined 🧵️interaction/⚙️authority/🦀️.rs:1350 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:20: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:32: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:35: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:53: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:56: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:67: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:106: WorldInteractionMeshToken defined 🧵️interaction/📇️registry/🦀️.rs:37 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:131: WorldInteractionMeshToken defined 🧵️interaction/📇️registry/🦀️.rs:37 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:230: WorldInteractionMeshRegistry defined 🧵️interaction/📇️registry/🦀️.rs:53 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:233: WorldInteractionMeshToken defined 🧵️interaction/📇️registry/🦀️.rs:37 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:383: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:392: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:417: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:419: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:429: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:438: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:471: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:477: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:514: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:567: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:591: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:599: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:611: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:626: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:660: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:679: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:724: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:835: WorldComponentTarget defined 🧵️interaction/🕹️gumball/🦀️.rs:7 vis=private
- 🧵️interaction/🧲️picking/🦀️.rs:839: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:20: WorldInteractionObjectSlot defined 🧵️interaction/📇️registry/🦀️.rs:196 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:51: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:58: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:116: WorldInteractionObjectSlot defined 🧵️interaction/📇️registry/🦀️.rs:196 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:260: WorldInteractionObjectRegistry defined 🧵️interaction/📇️registry/🦀️.rs:207 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:264: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:280: WorldInteractionObjectToken defined 🧵️interaction/📇️registry/🦀️.rs:189 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:662: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:663: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:710: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:711: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/🕹️gumball/🦀️.rs:727: WorldInteractionSpan defined 🧵️interaction/📇️registry/🦀️.rs:692 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:120: WorldInteractionAuthority defined 🧵️interaction/📇️registry/🦀️.rs:747 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:123: WorldCameraSync defined 🧹️lifecycle/🦀️.rs:38 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:225: WorldInteractionAuthority defined 🧵️interaction/📇️registry/🦀️.rs:747 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:311: WorldInteractionRegistryBuildCursor defined 🧵️interaction/📇️registry/🦀️.rs:333 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:322: WorldInteractionRegistryBuildCursor defined 🧵️interaction/📇️registry/🦀️.rs:333 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:383: WorldGumballGesture defined 🧵️interaction/🕹️gumball/🦀️.rs:147 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:383: WorldGumballCommitJob defined 🧵️interaction/🕹️gumball/🦀️.rs:639 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:452: WorldGumballPickCursor defined 🧵️interaction/🕹️gumball/🦀️.rs:122 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:480: WorldGumballGesture defined 🧵️interaction/🕹️gumball/🦀️.rs:147 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:480: WorldGumballCommitJob defined 🧵️interaction/🕹️gumball/🦀️.rs:639 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:486: WorldGumballCommitJob defined 🧵️interaction/🕹️gumball/🦀️.rs:639 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:495: WorldBrushCommitJob defined 🧵️interaction/🕹️gumball/🦀️.rs:658 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:518: WorldMarqueeGesture defined 🧵️interaction/🔍️marquee/🦀️.rs:5 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:544: WorldMarqueePickCursor defined 🧵️interaction/🔍️marquee/🦀️.rs:80 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:588: WorldObjectPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:506 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:590: WorldComponentPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:666 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:631: WorldObjectPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:506 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:636: WorldComponentPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:666 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:746: WorldObjectPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:506 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:751: WorldObjectPickCursor defined 🧵️interaction/🧲️picking/🦀️.rs:506 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:896: WorldComponentMarqueePublishJob defined 🧵️interaction/🔍️marquee/🦀️.rs:814 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:898: WorldMarqueePublishJob defined 🧵️interaction/🔍️marquee/🦀️.rs:579 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1025: WorldMarqueeGesture defined 🧵️interaction/🔍️marquee/🦀️.rs:5 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1136: WorldCameraSync defined 🧹️lifecycle/🦀️.rs:38 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1177: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1182: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1189: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1207: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1212: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1227: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1282: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1299: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private
- 🧵️interaction/⚙️authority/🦀️.rs:1319: WorldFlatAction defined 🧵️interaction/📇️registry/🦀️.rs:698 vis=private

## Appendix B: infinite B visibility candidates (26)

Module base: `➕️normal/`.

- 🦀️.rs:229: BoardPointerSpan defined 🧭️host/🦀️.rs:116 vis=private
- 🦀️.rs:267: BoardPointerSpan defined 🧭️host/🦀️.rs:116 vis=private
- 🦀️.rs:272: BoardPointerSpan defined 🧭️host/🦀️.rs:116 vis=private
- 🦀️.rs:2638: BoardDeletePlanningOperation defined 🧭️host/🦀️.rs:178 vis=private
- 🦀️.rs:2639: BoardDeleteOperation defined 🧭️host/🦀️.rs:152 vis=private
- 🦀️.rs:2640: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private
- 🦀️.rs:2645: BoardEntityRetirement defined 🧭️host/🦀️.rs:231 vis=private
- 🎨️style/🦀️.rs:327: BoardEntityRetirement defined 🧭️host/🦀️.rs:231 vis=private
- 🎨️style/🦀️.rs:335: BoardEntityRetirement defined 🧭️host/🦀️.rs:231 vis=private
- 🎨️style/🦀️.rs:343: BoardEntityRetirement defined 🧭️host/🦀️.rs:231 vis=private
- 🎨️style/🦀️.rs:351: BoardEntityRetirement defined 🧭️host/🦀️.rs:231 vis=private
- 🎛️selection-events/🦀️.rs:777: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private
- 🎛️selection-events/🦀️.rs:924: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private
- 🎛️selection-events/🦀️.rs:1055: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private
- 🎛️selection-events/🦀️.rs:1153: BoardPointerSpan defined 🧭️host/🦀️.rs:116 vis=private
- 🎛️selection-events/🦀️.rs:1153: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private
- 🧹️scene-delete-links/🦀️.rs:1104: BoardDeletePlanningOperation defined 🧭️host/🦀️.rs:178 vis=private
- 🧹️scene-delete-links/🦀️.rs:1122: BoardPropertyAudit defined 🧭️host/🦀️.rs:200 vis=private
- 🧹️scene-delete-links/🦀️.rs:1365: BoardDeleteOperation defined 🧭️host/🦀️.rs:152 vis=private
- 🧹️scene-delete-links/🦀️.rs:1384: BoardDeletePlan defined 🧭️host/🦀️.rs:142 vis=private
- 🧹️scene-delete-links/🦀️.rs:1385: BoardDeletePlan defined 🧭️host/🦀️.rs:142 vis=private
- 🧹️scene-delete-links/🦀️.rs:1476: BoardEntityRetirement defined 🧭️host/🦀️.rs:231 vis=private
- 🧹️scene-delete-links/🦀️.rs:1526: BoardDeletePlanningOperation defined 🧭️host/🦀️.rs:178 vis=private
- 🧹️scene-delete-links/🦀️.rs:1527: BoardDeletePlan defined 🧭️host/🦀️.rs:142 vis=private
- 🧹️scene-delete-links/🦀️.rs:2152: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private
- 🧹️scene-delete-links/🦀️.rs:2213: BoardPointerCommitOperation defined 🧭️host/🦀️.rs:243 vis=private

## Appendix C: renderer candidates (false positives)

- 🦴️glb-decode/🦀️.rs:1454: row defined 📊️scale-bench/🦀️.rs:116 vis=private (local `for row in 0..4`, not the item)
- 🦴️glb-decode/🦀️.rs:1456: row defined 📊️scale-bench/🦀️.rs:116 vis=private (local `for row in 0..4`, not the item)

## Out of scope and notes

- RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md and RESULT-FRAMEWORK-S-IDS.md are not in the brief and were not audited.
- Peer activity observed during the audit: `📇️space-index/` (manifest, untracked), and a shell parent whose saved original already includes 11 lines a peer added at 02:40. These were not attributed to the executors.
- Pre-existing duplicate sibling emoji (not created by any split): manifest `🔖️action-args` / `🔖️channel-version`; plugin host `📥️` x3, `🧪️` x2, `🧬️` x2.
- The RESULT reports say the compiler gate blocked every attempt. This audit did not change that status; none of the PASS verdicts is a compile result.

