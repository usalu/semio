# Checkpoint 15 Runtime

Canonical WGPU activation15 passed all20 tasks in17m15s; React activation8 passed all14 tasks in35s. Five measured WGPU/browser artifacts were sealed before the paired19-step journey. The paired journey completed:19steps,14 action-ledger differences, and exactly2 physical failures. Both failures are WGPU selected-value confirmations for Appearance and Language. No worker crash occurred. Allfive artifact hashes remained unchanged after the run.

## Camera result

After dismiss-tour, Top live camera position/target agree within0.00001; Perspective position within0.00005 and target within0.00001. Viewport size differs by0.06562 logical pixels. The corrected orthographic aspect behavior is therefore present in the fresh application. Boot cameras still settle later than the initial boot snapshot.

## General result

Both renderers open General and both Select popups. The WGPU option-routing repair closes the Appearance popup and permits Language to open, but selected-value confirmation still times out for Appearance and Language. The Appearance activation step records no action; Language records `setLocale` with `{value:de,windowId:framework.settings.general}` and controller `s.puzzle.puzzle3d@1/*#editor`, yet the control stays English. React commits both values. This is an active application failure assigned to SolWindow, separate from the preflight latch repair that landed after this artifact boundary.

The first General screenshot still has full-height glass. Opening Select shrinks the glass; native25 exposed the accepted-layout phase race and its latch fix is undergoing native27. Width/height also remain120×22.4 vs React160×16; see astra-general-control-geometry.md.

## Reference visual review

Paired step05 screenshots are both Light and have matching settled cameras. Both display loaded GLB models. WGPU reference remains too opaque/high-contrast and the Top plan appears square, whereas React’s visible aspect matches2275×2560. Its placement differs as well. S3 is checking natural-dimension propagation, origin semantics, and reference material behavior; the source-bounded pixel oracle alone cannot close this application visual gap.

General labels, tree row icons/chevrons, closed-control typography, window cap/chip affordances, and active window identity also remain visibly different. These are open parity items. Later dark-vs-light screenshots cannot serve as equal-appearance pixel comparisons because the WGPU appearance action failed.

Evidence: `🗑️generated/astra-runtime/paired-checkpoint-15/{steps.json,cameras.md,geometry.md,latency.md}` and paired screenshots. Physical postconditions passed for Settings closure, closing all windows, creating and interacting with a new World3d window, orbit/pan/zoom, and fullscreen entry plus exit. Both trusted fullscreen chords stayed on the owned DIV; the root fullscreen repair is now confirmed in the full application. Ledger differences remain separate from these physical outcomes.

## Full journey interim findings

The full50-step run uses the same sealed artifacts. After the earlier panel sequence, Appearance activation leaves its popup open; Language open and activation then fail behind it. The shorter19-step run closed that popup but failed selected-value confirmation. The differing history makes retained owner publication and down/up frame interleaving part of the required regression. SolWindow is tracing this path.

Step38 exposes a separate real command-palette gap: WGPU paints a large blank glass panel with only a Search title. Its snapshot gains no controls or dialog surface and focus remains on DIV. React gains the dialog, focused `ui.search.input`, and command rows. Terra is tracing normal producer/publication/layout/paint/focus; the screenshot itself already rules out treating this step as an accepted search feature.

The probe’s cap-focus, individual cap-close, pane-chip, and gutter helpers currently verify target clicks but do not all assert the resulting active/focused/layout/pane state. Their lack of runtime exceptions is not complete feature acceptance. Close-all/reopen, visible World body, selected value, and fullscreen steps do have outcome assertions. Stronger per-control postconditions remain required.

## Full journey completion

The50-step run completed with27 action-ledger differences and3 explicit runtime failures:Appearance popup retirement,Language popup opening,and Language activation. These are the same General path; the empty command palette is an additional screenshot/structure failure that the current chord helper does not yet assert. No worker crash occurred. Allfive measured artifact hashes remained unchanged after the full journey. The complete run is diagnostic coverage,not full renderer acceptance.

## Worker attribution

A separate WGPU-only boot,dismiss-tour,settings-open profile completed three steps with zero errors on the same unchanged artifacts. The bitmap retirement repair is visibly present in CPU attribution: `ShellDocumentRetirementRegistry::close_one` self samples are4.062ms, versus1043.523ms in checkpoint14’s earlier profile; its new occupied-index walk contributes16.278ms. These runs differ in sample count/duration and include profiler overhead, so this is attribution evidence rather than a controlled speedup benchmark.

`RasterContentIdentity::mix_bytes` remains the largest named self sample at342.695ms. The ongoing pool/prepared handoff must remove per-frame whole-image rehashing. Aggregate paired15 workerTick max is75.5ms; shellRefresh spans up to806ms and dispatchApply up to383.3ms. Different authority spans are not interchangeable with one synchronous frame, and frame-budget acceptance remains open.
