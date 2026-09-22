# Select Popup Wheel Parity

The retained and immediate WGPU Select popups did not own ordinary wheel events. Chevron scrolling was already wired. The retained regression uses a popup escaping a clipped parent and the existing neutral overlay fixture; UI30 failed at the actual scroll assertion (`0` instead of `34`) before implementation.

The repair routes retained wheel input through the topmost open Select before scene or ancestor scrolling. Its accepted popup geometry supplies the maximum offset, so the router needs no independent theme estimate. Non-finite deltas cannot poison scroll state.

The immediate path records one bounded popup scope in the existing staged interaction maps. The scope includes the owner, menu rectangle, maximum offset and the end of its registered hits. These maps publish with the matching hit registry. A later hit blocks the popup, while an underlying hit or empty gutter does not. Owner IDs and option values may contain `.item.` without affecting wheel ownership. The live open-select map cannot revoke the last presented frame before replacement acknowledgement.

The neutral fixture also supplies row height and viewport padding. Taffy independently computes content height for the maximum-scroll assertion. Native146 first encountered a test-only DrawList alias compilation error. Its corrected rerun passed all three component-close handoff tests and failed the immediate Select wheel assertion because the shell did not consume the event. The shell binding and bounded staging cleanup were added after that observed failure. Green reruns and browser acceptance remain pending.

UI29, before this wheel change, passed all 670 tests. Native145 passed 1,362 of 1,368 renderer tests; the six failures are assigned to the two Sol agents. The React General Settings grid suite passed 10 of 10 after its neutral fixture was aligned with the scoped Tree value-column variable.

UI31 initially found a missing `E: Clone` bound on the immediate wheel method. After correcting that compile error, its full rerun passed all 672 tests in 10.906 seconds, including the retained wheel, Taffy extent, candidate/accepted scope, gutter, occlusion, finite delta and clamping assertions. The exact fixed heap accounting tests remained green. Native147 is rebuilding after the same bound correction; WASM21 is compiling for browser verification.

Native147's corrected full run passed 1,369 of 1,370 tests in 41.829 seconds. The immediate Select wheel law passed. The remaining Dock silhouette law could not find its expected clipped fill and is assigned to SolTree for runtime diagnosis. The rapid popup-dismiss/close law was added after the likely compile snapshot and must be selected explicitly next time.

The fresh React browser oracle at 1600×300 used the seven-option Orthographic Select in Top Window Options. A real wheel delta of 34 moved `scrollTop` from 29 to its maximum of 42, with no console error. Its DOM and screenshot are saved under `🗑️generated/astra-runtime/checkpoint21-iab`. At 1600×1000, the same menu measures 128×232.0625 against a 104-pixel trigger: 1-pixel border, two 22.375-pixel chevron bands and a 185.3125-pixel viewport. WGPU still uses trigger width and rows-plus-padding for natural height. These additional source/DOM mismatches are assigned to SolLocale; the wheel repair alone is not full popup geometry acceptance.

The P5d interactivity verification passed after the Select wheel and close changes (`p5d-select-close-final/run.log`, Nx duration 3.0 seconds).
