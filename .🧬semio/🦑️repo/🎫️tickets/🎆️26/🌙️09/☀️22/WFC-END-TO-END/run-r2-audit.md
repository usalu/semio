# WFC Round Two Audit

Read-only review on 2026-11-04. Only this report is authored. Read AGENTS.md, previous audit, run-2026-11-04-state.md, run-2026-11-04-plan.md and browser inputs. No native/browser pass is asserted.

## Concrete Browser Input Blockers

1. wfc-browser-scenario.ts forms (70-86) never selects variant.authorWindow. That field is used only as bootWindowPrefix. unfoldActionsRail unfolds engagement chips; it does not activate the intended window. Select the author mode-dock-tab before scanning its action pane.
2. Bitmap keep (145-154) never selects output. PinPreview requires an addressed wfc-bitmap-output window (pin-solution require_preview_window,75-82). The global selector can miss this output-only action or address the wrong pane. Select output before unfolding/clicking keep. Existing public tab identity is data-slot=mode-dock-tab plus data-window-id.
3. Grid3d cases create scenario-tile with RGBA254,253,252,255 then changeTileColor to the identical values. forms requires a new edit for both. Make the second colour different to avoid testing a semantic no-op as a mutation.

## Verified Correspondence

All five browser preview strings match native surface IDs: wfc.bitmap.output, wfc.grid2d.surface.preview, wfc.wfc2d.preview, wfc.grid3d.preview, wfc.3d.preview. Public ready English/German status strings match framework actions-labels, as does toolRunStart and framework.toolRun.ready.toolRunStart. No mismatch found in these selectors.

Component Cargo metadata declares five distinct app IDs and React6041-6045/WGPU6141-6145. Prior audit explains fresh dev targets and framework-owned example selection; no conflicting route observed. Five ArtifactEditor command_from_action adapters use current mutable args/original_fault parameters. Parent already repaired graph2D/3D projection signatures.

Bitmap render uses doc.render_operation_ref canonical revision. Live output filters against run.identity.base_revision; finished cache requires source_revision plus dimensions. Complete authored pins reconstruct output after dismissal/reload. Retained PinPreview validates addressed output kind and exact source revision before cursor admission. The earlier dimensions-only freshness hole is closed in inspected source.

## Ownership And Remaining Proof

Cursor close/post laws remain with bitmap_finish_r2. Current route emits one atomic PinSolution, avoiding the earlier repeated per-pin cold snapshot publication. Maximum512x512 receipt, cancellation, inverse and cleanup laws require actual native execution.

A concurrent read briefly observed trailing NUL padding in retained-commands; stable reread found17549 bytes and no NUL. Not a remaining defect.

Required acceptance: fresh boots for five editors, localized author forms producing real edits, solve/finalize/dismiss, addressed bitmap keep, undo/redo and folder reload. Tiny examples may finish before pause/abort; documented race notes do not prove cancellation. A larger admitted problem is needed. Syntax/import checks and oracle payload checks do not establish end-to-end success.
