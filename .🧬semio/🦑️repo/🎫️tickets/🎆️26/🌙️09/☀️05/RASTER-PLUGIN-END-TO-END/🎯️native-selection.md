# Native Pixel Selection Integration

The preceding turn was progress: shared mask settings, scene wiring, native intent/publication, controls and verified TypeScript suites changed authoritative state. Four previous verification sessions were confirmed live at this turn's start.

## Current Gap

Native surface marquee selects layer bounds. Native stroke publication still has selection:null. Shared Rust editing currently exposes only synchronous selection_mask, performing a pixel-by-polygon-edge scan without progress/cancellation; TypeScript already exposes a scanline PixelSelectionJob with row grants. Native interactive selection must use the same bounded job semantics before connecting rectangle, ellipse and lasso tools.

## Contract and Test First

Added SelectionShape and SelectionRasterization definitions to the shared editing schema. New neutral fixture pins a concave polygon, row grants/progress, invalid grants and cancellation before, during and after completion. TypeScript tests compare the exact rasterized result with independent libvips SVG alpha coverage. Native tests target a new owned PixelSelectionJob with the same grant/result contract. No native selection job implementation has been authored yet. Native scene selection transport and controls remain required; this job alone does not establish end-user selection completion.

## Native Job Implementation

Native red run 22182 failed with three missing PixelSelectionJob compiler errors. TypeScript fixture run 12113 passed 172 tests, including exact independent libvips SVG coverage. Native PixelSelectionJob now owns its shape and candidate, advances 1–64 scanlines, reports row progress, refuses partial result access and discards on cancellation. Output initialization follows processed rows. Synchronous selection_mask delegates to that single implementation. Native green run 8631 is active.

Source review found that the prior intersection formula can overflow for finite polygon coordinates; a shared enclosing-polygon fixture was added before changing arithmetic, with a focused TS red run pending. The intended repair uses scaled vertical interpolation and a weighted horizontal interpolation, avoiding overflowing edge differences/products.

The finite enclosing-polygon probe passed unchanged in TS run 59121 (173 tests). It therefore does not establish an arithmetic defect or justify the proposed interpolation rewrite; that rewrite was not applied. Native scanline filling guards NaN intersections before slicing, matching TypeScript's non-filling behavior and avoiding an invalid slice range. Keep numerical precision claims limited to the tested shapes. The cancellation/progress job remains the substantive selection work in this checkpoint.

## Current Verification Handles

Native selection green 8631 (`raster-selection-job-native-green-1.log`) was confirmed live after the implementation and guard changes. Shared TypeScript selection/model run 59121 passed 173 tests; it is terminal, not a pending red run. Native mask paint 88342 and WGPU publication 20371 were confirmed live; the latter reports the shared artifact directory lock. Full Raster mask/config rerun 10202 was confirmed live after adding missing schema-field state annotations. Do not restart any live job solely on elapsed time.

## Next Integration Requirement

Carry compact pixel-selection coverage and its target revision through the local editor config/scene contract, separate from framework layer selection. Native rectangle/ellipse/lasso gestures must drive the owned bounded job and publish coverage only on completion; cancellation must preserve the previous valid selection, while target revision changes invalidate mismatching coverage and discard candidates. Painting and algorithm commands must carry the resulting selection spans instead of selection:null. Add/subtract/intersect and wand selection need the shared combination/flood algorithms, progress and cancellation. This remains open; the new core job is necessary infrastructure rather than end-user parity evidence.

Activation session 18283 was also confirmed live in this turn; it predates current mask/selection source and is not current UI acceptance evidence. No activation process or lock was altered.


## Completed Selection Configuration and Transport Checkpoint

Authored a shared completed-selection descriptor (`layerId`, `target`, intrinsic `width`/`height`, bounded RLE `spans`) in Rust, TypeScript, JSON Schema, GraphQL and Proto. An absent selection is unrestricted; `[]` is an empty selection. Config mutation validates the descriptor, preserves it through brush settings and unchanged paint target, and clears it when switching between pixels and mask. Content image keys deliberately remain outside persisted coverage.

Added `setPixelSelection` as a config-only retained command, with exact image-key and intrinsic-extent guards. Hidden/missing targets refuse; locked layers can still be selected. Reused the same RLE parser for edit pixels, edit mask and mask from selection. Added a third optional paint scene lane for the completed coverage, including a 40,000-byte native header-size test, so coverage does not overflow the 32 KiB scene header.

TypeScript red run `raster-selection-config-red-1.log` failed on the deliberately missing parser export (136 passed, 1 failed). Green run `raster-selection-config-green-1.log` passed all 149 tests / 637 assertions, including Sharp PNG coverage round trips. Native tests for config/codecs, revision/extent/visibility, and command registration are authored but not yet confirmed. Current live full Raster session 10202 was launched before this checkpoint and may compile these edits; do not treat its eventual result as current without checking its captured diagnostics/test roster.

Mounted scene attempt 1 selected a registration helper rather than an executable test file and found no tests. Attempt 2 correctly selected Interpreter but could not collect tests because MediaTransportHost imports a nonexistent relative plugin media module. No unrelated media source was changed. A direct Bun scene transport contract test was added to the existing Raster test suite. Native UI paint scene run 8363 is pending (`raster-selection-scene-native-1.log`).

Still required: make React selection operations publish this config and restore it from the optional lane; bind native selection gestures/jobs and selected paint edits; finish native filter controls and end-user runtime acceptance. This checkpoint is infrastructure progress, not a completed selection experience.


Direct scene/config verification `raster-selection-config-green-2.log`: **150 passed, 0 failed, 641 assertions**, including reconstruction of the optional coverage lane and omission when no selection is present. This proves the shared TypeScript carrier contract, not mounted/native end-user behavior. The mounted Interpreter suite remains uncollected due to its unrelated MediaTransportHost import error.


## React Shared Selection Integration In Progress

The previous continuation was progress: it authored the config descriptor, retained command, and optional scene lane. This continuation revalidated live native handles 88342, 20371, 8631, 10202 and 8363 before continuing.

React selection tools now publish `setPixelSelection` after bounded shape/combination/span preparation, and only render/use coverage restored from the shared scene. Paint2dHost forwards the optional selection carrier. Empty coverage stays distinct from unrestricted editing; matching intrinsic coverage survives image-content changes. Refused publication leaves authoritative coverage unchanged. Restoration gates editing until complete and exposes progress/cancellation; clearing dispatches config state instead of only changing local component state. Escape cancels an in-flight candidate before clearing a completed selection on a subsequent Escape.

Added `restoreSelection` to the paint editing model, validating ordered bounded RLE and restoring in 32,768-pixel grants. The red shared-source run failed on the deliberately missing export. Green `raster-selection-restore-green-1.log` completed **177 passed, 0 failed**, including shared neutral coverage, invalid spans, progress/cancellation and existing Sharp oracles. Nx labeled the target flaky after red-to-green source changes; this is not evidence of an observed nondeterministic test failure.

Mounted tests now simulate the actual config command/scene feedback, distinguish config commands from document edits, and add explicit tests for shared empty/partial coverage after content updates and refused selection publication. Sessions 44891 (red baseline) and 68480 (first integration run) are confirmed live; their results remain pending. The test harness received small typing corrections after run launch, so rerun only after a terminal result if necessary.


Mounted-run diagnostic: sampling the confirmed worker of session 68480 (PID 64904) recorded a 17.3 GB footprint, peak 18.8 GB (`raster-selection-mounted-sample-1.txt`). Both my red and first integration mounted runs were explicitly terminated (worker/parent pairs 63658/63635 and 64904/64889) after revalidating their exact selection-focus commands. This was cancellation for observed excessive memory, not a timeout inference. Their test verdicts are unproven. Native jobs and other agents' processes were untouched. Isolating the new refusal test with an explicit 20-second per-test timeout in `raster-selection-shared-refusal-1.log` (37545).


The first isolated run (37545) terminated before tests because the script already supplies `--testTimeout`; Vitest rejects a second value (900000 and 20000). The replacement run (96538, `raster-selection-shared-refusal-2.log`) uses the supported per-test timeout option and one test-name filter. It is a replacement for a terminal CLI-argument failure, not a duplicate live run.


Isolated mounted results now verified: refusal test **1 passed / 49 skipped** (`raster-selection-shared-refusal-2.log`, 96538 exit 0), shared empty and partial selection restoration across image-key changes **2 passed / 48 skipped** (`raster-selection-shared-restore-mounted-1.log`, 20118 exit 0). These directly exercise the mounted controls and submitted edit payloads. A complete 50-case mounted run with verbose per-case reporting is now launched as `raster-selection-shared-mounted-green-2.log`; its result remains pending.
