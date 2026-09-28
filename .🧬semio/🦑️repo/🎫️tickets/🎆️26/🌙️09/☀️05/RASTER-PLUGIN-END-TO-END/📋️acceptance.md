# Raster End-User Acceptance

The requested outcome remains a complete usable raster editor, not a collection of independently passing algorithms. This checklist consolidates the original acceptance in `📓️complete-editing.md`; it does not replace it or narrow the goal. A row is complete only when its end-user behavior and required implementations are verified together.

| Requirement | Current evidence | Remaining proof or implementation |
| --- | --- | --- |
| Paint/erase real layer pixels with color, size, opacity, hardness, transforms and selections | Shared Rust/TypeScript pixel operations; authoritative editPixels command; earlier live publication/history recorded in the main notes | Revalidate after incremental source preparation. The completion promise is now preserved and 17 mounted tests verify duplicate-dispatch exclusion and applied/refused/superseded feedback; live rapid strokes and native surface parity remain pending. Layer protection is implemented in source with 75 Raster TypeScript and 31 mounted React checks passing; final-source native verification and live protection workflows remain pending. |
| Usable layer management | Add, duplicate, delete, visibility, patch, move commands; corrected sibling-relative ordering and Inspector described in main notes | Flatten Layers is implemented with a 49-test Raster TypeScript pass; native/history proof now passed in the 275-test suite; live proof is pending. Merge Down is now authored for adjacent normal-blend pixel/group siblings, with 62 passing Raster TypeScript tests; native/history/Inspector coverage passed in the 281-test suite; live proof is pending. Layer locking and duplicate placement have earlier native evidence (297/299-test checkpoints). Full matrix scale/rotation/shear controls are now implemented for pixel/group layers with 140 TypeScript tests; current native retained history is pending. Broader selective merge behavior and a coherent live nested-layer workflow, including undo/redo and reload, remain open. Export flattening is not a user editing command. |
| Pixel and group masks | Selection masks, enable/invert and placement previously verified live. New editMask coverage brush, imported luminance/alpha normalization, expected descriptor guard, selection and linked/unlinked transforms; mounted pixel/group mask cases pass | Retained mask publication/history tests now passed in the 275-test native suite. Live erase/reveal, cancellation, undo/redo, both canvas views and saved document round-trip. Mask fill now has neutral/oracle and mounted pixel/group evidence; native publication and live fill remain pending. Inspector mask link/unlink and shear controls are now implemented with complete affine persistence. Mounted affine editing passed 35 tests and native paint passed 66 selected tests; current full Raster linkage/history and live mask authoring remain open. |
| Rectangle/ellipse/lasso/contiguous-color selection with replace/add/subtract/intersect/invert | Shared algorithm fixtures and Sharp comparisons; mounted focus-preservation and cancellation tests | Full live selection workflow on transformed image/mask targets, keyboard-only operation, native UI parity and persisted-versus-ephemeral boundaries. |
| Adjustments, filters, fill, crop, resize, rotation and flips | Shared core has these operations; tests cover oracle pixels and bounded progress. React controls dispatch authoritative commands | Live parameter validation and undo/redo for each family. Non-destructive brightness/contrast parameters passed current native tests; activation/live proof remains pending. Extend non-destructive algorithms beyond brightness/contrast where required by the editing workflow. |
| Import, editable save/load and flattened export | Current editor exports artifact packs and composites through image:out; image:in owns a retained importer. Existing tests cover parts of codecs/compositing | Exercise file UI end to end with image import, editing, save, reload, and independent decoding of exported pixels. Verify layers/masks/adjustment parameters survive editable saves. Confirm large export cancellation and source preparation responsiveness. |
| Accessible, localized and customizable interaction | English/German controls, semantic toolbar labels, focus tests and existing host customization | End-user keyboard audit, visible focus and understandable errors/progress, narrow-screen layout, no unresolved locale fallback, customization retention, and native control parity. |
| Local-first multi-user history and cancellation | Canonical semantic mutations, retained work, immutable source-revision checks, history and asset ownership tests | Two-client current-source acceptance, conflict receipt and short disconnect recovery. Verify cancellation during preparation/compute/encoding and ensure no unpublished output enters history. |
| Performance and bounded work | Pixel grants, selection scans and PNG encoder stages exist; latest native source preparation and output initialization are incremental | Shared pixels native passed 38 tests (zero skipped), including affine controls. Retained flatten and staged export have earlier native evidence; source preparation, composition, encoding and delivery use bounded stages. Current native export/cancellation follow-ups, browser source/result copy review and realistic large-image responsiveness checks remain open. |
| Packaging and verification | Bun/Nx and existing launch targets; no new runtime library; no modifying Git operation or worktree | Successful current native/React builds, final runtime logs and cleanup of ticket-generated outputs. Close the repo ticket only when all required work and evidence are complete. |

## Active Verification

Current passing independent suites: Raster TypeScript **140 tests**, shared pixels/host TypeScript **150 tests**, shared pixels native **38 tests, zero skipped**. Persisted full affine transforms and Inspector link/shear controls are now implemented in source across schemas, codecs, mutations, renderers and pointer mapping. Native integration and end-user behavior remain unverified. See [mask affine work](📐️mask-affine.md), [asset replacement](🖼️asset-replacement.md) and [layer transform controls](📐️layer-transform.md).

Current full native layer-transform run 2 (**75362**) passed **321 tests, zero skipped**. It verifies layer transforms, retained pixel/group undo/redo, mask linkage, asset replacement at capacity, selection pruning, canonical editable archive reload/history, retirement coverage and retained export/cancellation. The preceding run's two export faults did not reproduce; their cause remains unestablished. Temporary timing instrumentation is removed, and a focused uninstrumented export verification is active.

Native paint green run (**93882**) passed **68 tests, 196 filtered**, including lock-aware gesture admission and target revision invalidation. Mounted revision run (**75604**) passed **43 tests**; follow-up **44843** verifies four additional session cases (expected total 47). Full-host compositor completion/cancellation passed **2 selected tests, 703 filtered** (40038). See [verification](🏗️native-verification.md), [native interaction](🖱️native-editing-parity.md), [transforms](📐️layer-transform.md) and [export](📤️interactive-export.md).

Activation **13 (session 18283)** is live, sampled waiting on Cargo's prebuild lock. It predates the latest source. Prior preview sessions do not prove current-source editing. Browser navigation was rejected by browser security policy; no alternate surface, URL or transport is used to bypass it. Live UI, native parity, broad current-source mounted verification, real save/reload, two-client and performance acceptance remain open. The chronological checkpoints below are historical evidence, not current job state.

Native source audit confirmed missing lock-aware gesture admission, mask-command routing and pixel-selection carriage. The native lock/revision red run (79604) reproduced both defects. Lock-aware admission and captured-target invalidation are now implemented; native green run 93882 passed 68 tests and mounted run 75604 passed 43. The expanded mounted follow-up remains active. Shared model tests passed 150 cases. See [native editing parity](🖱️native-editing-parity.md).

## Next Sequence

1. Resolve the current native/build results and finish live mask and adjustment acceptance.
2. Complete layer protection and merge/flatten editing through semantic events and retained jobs.
3. Exercise the complete import/edit/save/reload/export workflow and correct any integration gaps.
4. Complete native controls and interaction parity, then run accessibility, multi-user and large-image acceptance.
5. Audit all rows against current source and runtime evidence, clean generated outputs and close the ticket only when the full requested outcome is achieved.

Latest interaction verification: mask-fill run 45870 passed all eight mounted tests. The new alphaFill implementation passed shared-pixel (106) and Raster (50) suites after a three-case red run. The 6061 preview loaded the Raster editor, but HMR retired its instance and rejected selection; a full reload is pending. Do not count this as live command acceptance. The still-running native jobs have not yet established coverage for this source revision.

Mask link/unlink control is an additional open layer-mask requirement: current renderers support both modes, but Inspector has no action. Lossless frame conversion can require shear, beyond the current translate/rotate/scale schema. Carry full affine support through schema, native/TypeScript rendering, pointer mapping and semantic undo before claiming position-preserving link/unlink.

Pixels native session 40901 is now terminal, exit 0: **33 passed, 0 skipped**. Its private target directory resolved the observed verification delay without duplicating the intermediate cache. Other native/activation/preview sessions remain pending.

The current frontend completion matrix passed 17 mounted cases. The full-host forwarding test and idle regression suite passed all three cases. Full Raster native 62344 ended on a shared UI token compile failure; the current token definition exists, and 91306 is the fresh isolated-output retry. Browser reload was blocked by browser URL policy, so no subsequent browser navigation was attempted and live acceptance remains open.

## Selection Controls Verification

Select All and Invert Selection now use cancellable 32,768-pixel grants with progress and yield points. Cancel preserves the previous selection; Deselect cancels preparation before clearing it. Shared pixels/host-helper validation passed 111 tests, and the initial mounted matrix passed 21 tests. The follow-up mounted run passed all 23 tests, including keyboard Ctrl/Cmd cancellation. This closes the synchronous fill/map loop for these controls only; browser image copies and selection merging still require review. Native 91306 and activation 69010 remain pending, and the recorded browser policy block remains in force.

Native run 91306 completed with 270 passing and four failing tests out of 274. Mask/adjustment work compiled and its tests passed; flattening exposed cleanup and duplicate-identity faults. Those repairs and the two test-catalog corrections are recorded in the main notes and are awaiting final-source native validation.

The native repair rerun completed successfully: `raster-native-isolated-target-2.log`, 275 passed, zero skipped, exit 0. This includes repeated layer creation, flatten compositing/inverse and retained undo/redo, mask editing and adjustment parameters. Live activation and end-user workflow acceptance remain outstanding.

Final focused mounted verification after correcting the mask test readiness wait passed all 23 tests (`raster-selection-edit-react-3.log`, exit 0). The separate full-renderer census's reported stack-overflow failures remain tracked pending its final diagnostics or rerun.

## Merge Down Verification In Progress

The new command preserves the selected upper identity, parent/index, surrounding layers and shared assets, and uses the same cancellable baking pipeline as Flatten Image. Neutral/Sharp cases include nested layers and a masked translucent group (62 Raster TypeScript tests passed). Native run 39307 failed on a missing command-module import, now fixed. Native rerun 5119 and live acceptance remain outstanding. Arbitrary selected ranges, backdrop-dependent blends/adjustments, and arbitrary transformed-parent appearance still need broader coverage before the selective-merge requirement is complete.

Merge Down native rerun 5119 compiled: 278/279 tests passed. Pixel/asset/history cases passed; the Inspector assertion read a field omitted by the fixture projection helper and has been corrected to inspect the actual node. Run 94651 includes that correction and a new 64-asset-capacity regression. Final-source native and live acceptance remain pending.

## Asset-Capacity Regression

Both neutral fixtures now exercise full 64-entry asset pools. The initial native run failed for both Merge Down and Flatten Image because their new asset was inserted before obsolete source assets were removed. Publication order is corrected inside the atomic edit, with shared and unrelated assets preserved. Current native verification is pending as described above. A loaded-document regression additionally requires exact undo/redo for both commands at capacity. This is not a claim that live editing, save/reload or collaboration acceptance is complete.

Final capacity verification: **281 native tests passed**, zero skipped, exit 0, `raster-merge-capacity-native-5.log`. All capacity cases and current Merge Down Inspector/history cases are now verified natively. The feature rows remain open where end-user runtime, native surface controls, collaboration or additional implementation are still required.

## Selection Combination Verification

The final selection-combination pass now uses cancellable grants in TypeScript and Rust. Schema/neutral vectors cover all four soft-coverage modes and absent current coverage; Sharp independently matches their results. Validation: 120 TypeScript tests, 34 native tests and 27 mounted canvas tests passed. The mounted suite cancels after combination has begun through Cancel, Deselect, local Pan and an external host tool switch. Creation and combination share one progress bar without resetting it. Browser source/result copying, realistic large-image responsiveness and live/native-surface acceptance remain open.


## Layer Protection Checkpoint

Persisted model/mutation, command guards and localized lock controls are implemented in source; [protection evidence](🔒️layer-protection.md) records 75 Raster TypeScript tests and 31 mounted editor tests passing. Native compile/publication/history validation is pending behind changing framework ownership compile failures; live acceptance, editable save/reload and true multi-client lock-versus-edit arbitration remain open. Keep the full acceptance scope above active.


## Latest Native Protection Result

Native protection run 6 passed all 297 tests with zero skips. TypeScript duplication planner run 1 passed 84 tests, before the additional mask fixture metadata was authored. [Duplicate placement work](📋️duplicate-placement.md) adds shared regression cases and Inspector action coverage; native behavior red/green remains in progress. Live workflow acceptance and the broader goal remain open.


Duplicate final native run green-1 completed successfully: 299 tests passed, zero skipped (`raster-duplicate-native-green-1.log`). This verifies same-parent duplication, metadata/inverse assertions and both-language Inspector action bindings/protection. Live browser acceptance remains open.

## 2026-09-28 Export Checkpoint

[Interactive export notes](📤️interactive-export.md) track retained media export, PNG download, private output ownership and localized progress/Cancel controls. The current TypeScript suite passed 120 tests. Native run 5 executed 310 tests, with 307 passing: the command download and rendered progress/cancel workflow passed; two media-export admission tests and one editable archive fixture failed. Repairs are under native run 6. The focused framework progress suite passed all three selected tests. Checkpoint counters now reach the live Layers view, and explicit user cancellation has a graceful terminal result while worker failures remain visible. Browser acceptance remains unverified under the previously recorded policy block. [Editable archive verification](💾️editable-round-trip.md) now includes nested layers, masks, protection, adjustment parameters and undo/redo, with its first runtime result pending after a fixture cleanup correction.

## Selection After Layer Creation

Add, Drop and Duplicate now request selection of their new layer through the framework's post-publication interaction lane. The neutral request tests bring the TypeScript total to **124 passed**. Native creation/duplication selection, deletion pruning and history assertions are authored but pending; [duplication notes](📋️duplicate-placement.md) record the scope. Native run 6 remains live, and its actual test census must be checked because these tests were added while dependencies were building. The current full Raster census is 313.

## Coordinate Frames and Native Workflow Follow-up

Shared affine coordinate-frame conversion is now schema-first and implemented in Rust/TypeScript, with five neutral placement vectors, invalid geometry refusal, world-point invariance, reverse conversion and a Three.js oracle. Existing pixel targets passed 126 TypeScript tests and 36 native tests. Persisted full-affine Raster transforms and end-user mask link/unlink integration remain open.

Raster TypeScript remains 124/124. Native run 7 was 310/313; remaining failures led to actual layer topology membership for stale-selection pruning, decimal fixture correction and cancellation test contract correction. Run 8 was blocked before tests by shared kernel compilation. Run 9 is pending after a narrow borrowed Option projection fix. See interactive-export, duplicate-placement, editable-round-trip and native-verification notes for exact scope. The overall editor goal and this ticket remain open, including live/browser acceptance and the previously listed feature requirements.

Latest verification: native runs 8–10 stopped in changing shared dependencies before Raster tests; do not report the corrected native workflows as passing. Last Raster execution remains run 7 (310/313). Shared frames are 126 TS / 36 native green, Raster TS 124 green. Activation is awaiting a Cargo prebuild lock. Ticket and overall goal remain active; full mask affine persistence/control integration and all prior open end-user acceptance items remain required.

## Current Source Audit: Remaining Editing Breadth

Full affine persistence does not yet give end users full layer-transform controls. Inspector currently offers layer translation and pixel extents, while scale/rotation/shear controls are currently mask-only. A complete layer transform action must support pixel and group layers through semantic events and exact history; do not misuse pixel replacement to transform a group.

The shared stack and compositor currently support only brightness/contrast as non-destructive adjustment content. Destructive pixel algorithms exist independently, but they do not prove non-destructive adjustment-layer breadth. Inspector has brightness/contrast parameter controls only. These remain source-confirmed implementation gaps within the original complete-editor objective, alongside the previously listed live/native parity and collaboration requirements.

The newly implemented capacity-safe asset replacement is pending full native publication/history verification; TypeScript planner and independent RFC 6902 checks passed 134 tests. It addresses pixel/mask edits reusing an exclusively owned slot, while shared assets at a full pool still produce a capacity refusal without mutation. The overall goal is not complete.

## Native Mask Painting Checkpoint

Implemented shared paintTarget/maskValue configuration, semantic commands, shared scene transport, native/React controls, native pixel/group-mask strokes and exact editMask revision publication. New neutral mask placement and revision fixtures have independent Three.js and fast-json-patch checks. Latest pure suites: 171 shared pixels/model tests and 145 Raster TS tests passed. Native mask/config and mounted controlled-setting verification are still pending; do not mark this acceptance item complete. Previous full native 321, paint 68 and mounted revision 47 remain historical evidence for the pre-mask-setting checkpoint.

Active verification sessions: native paint 88342; full Raster 63331; native WGPU publication 20371; mounted controls 64055. Uninstrumented export 69035 failed before tests on a changed SVG serializer return type, now repaired in Raster's caller; full Raster rerun includes export. Mounted controls 88851 failed 11 old/harness expectations (36 passed), corrected for explicit config-command publication and rerun as 64055. Details and scope limits: 🖱️native-editing-parity.md.

Mounted shared mask-control verification now passed 47/47 (48839). Full Raster mask-config verification 63331 failed before tests on missing schema-field state annotations; repaired and rerunning. Shared selection core now has an owned native row-granted selection job under verification. Its job progress/cancellation fixture and SVG oracle passed in TS (172), followed by the enclosing-polygon probe (173). Native end-user pixel selection transport, controls and publication remain open.


### Completed Pixel Selection State Checkpoint

- Shared intrinsic coverage descriptor/config mutation and guarded `setPixelSelection` command authored; Rust command registration/census updated to 26 commands, 25 retained config/document routes, 26 proofs including export.
- Pixel/mask edits and mask-from-selection now use one bounded native RLE parser.
- Optional coverage scene lane authored in Rust/TypeScript with neutral fixture and 40,000-byte native spine test.
- Raster TypeScript suite: 150/150 pass with Sharp coverage round trip and direct scene lane reconstruction.
- Native config/commands/scene proof pending. Mounted scene collection blocked by unrelated media import; no native/live UI acceptance claimed.
- React shared selection publication/restoration and native selection gesture/filter UI remain open. See `🎯️native-selection.md`.
