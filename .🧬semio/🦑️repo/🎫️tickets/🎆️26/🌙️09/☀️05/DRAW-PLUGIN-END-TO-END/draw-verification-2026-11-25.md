# Draw end-to-end verification, 25 November 2026

The live React playground on port 6064 accepted the published Draw WebAssembly component. Its English and German editor journeys reproduced a failure after switching from Demo to Blank Drawing and back. The empty drawing reported one selected layer. In the instrumented English run, copy settled without a clipboard effect and the queued paste received no fragment. A separate run completed copy and paste, then cut coincided with an asynchronous camera update and the guest rejected a stale document root. Later input to that instance failed with the same error.

The keyboard shortcut now resolves a retained paste fragment at dispatch time so a preceding copy can replace an older fragment. The viewport regression test first failed with two identical camera publications. After the renderer change, all 17 framing tests passed with 145 assertions, including independent Three.js projection comparisons.

The editor journey now checks that each example load clears selection. It keeps the status panel open while observing progress. The ticket journey launcher propagates a failure in either language through its exit status.

The registry and publication evidence is in [registry-25.md](registry-25.md). A successful live journey requires all 13 checkpoints, two document loads, the expected PDF raster differences, and no reported browser or guest faults.

The live Viewer passed in both languages, including rendering, zoom and returning to Editor. See [Viewer verification](viewer-25.md). The mounted canvas input suite passed all 27 tests through the React Vitest owner. The canvas/SVG agreement suite passed 47 tests with 164,027 assertions.

The descriptor emitter executed the initially served 143,710,776-byte component. Its JSON and packed descriptor agreed with each other and with the model receipt. The committed pair differed in the two applications’ action lists and examples. Final synchronization requires the rebuilt component.

The full Draw TypeScript owner passed 715 tests. The renderer’s mounted window action panel passed all 22 cases. The example command now clears object and point selections when loading a replacement example. The stale-publication fix confines a superseded operation to its own failure page and preserves the mounted actor so that later edits can run.

The initial local WASI build compiled the component crate and all its dependencies. Its final linker reached 7,182 MB, and the host stopped it at the 4,096 MB final-unit ceiling. The native retry requests one linker thread and stripped symbols. The regular component driver also rebuilt its descriptor emitter and started its own build with five local compiler slots. The first native build's partial output is unsuitable for verification.

The archive-load law passed in the native editor. The end-to-end PDF command law failed when the Drawing fixture polled for pending geometry using a newly minted context with no parent source supplier. Both Drawing geometry tests now poll through the mounted app's own context.

A separate native test drove the export read back to its issuing store with exact copy, capacity and release grants, then failed to close the store. The read registry quoted only releases. Its final step needs to move the retired registry alias out of the owner slot and requires a copy grant. The owner now delegates to the same demand calculation used by the closing operation. Both the store and initialization replay retain that copy quote. An independent captured-snapshot test checks that withholding one copy byte leaves the registry entry in place, and that granting the full quote retires it.

The managed prototype component completed all 11 stages with 676 compilations and 276 cache hits. The 143,779,747-byte WASI component has SHA-256 fcc8e11c2267be890c236bf25f06d9e378b87b4becc78bebb74d95a6bcfb10e2. The component, model receipt, browser descriptor, and restored descriptor pair passed the receipt verifier. All three Wasm cores served by the normal Draw playground on port 6064 matched the published model.

The first English journey against this component passed through PDF export. It exercised starting Demo, switching examples, keyboard copy/paste/cut, attaching a fresh folder, drawing a rectangle, undo/redo and saving. The rectangle changed 11,250 rasterized PDF pixels at 200 × 200. Reloading and reconnecting then exposed an older window configuration revision that faulted the actor. The first German journey on the same component reached copy and timed out awaiting the pasted layer. These runs did not establish full acceptance.
