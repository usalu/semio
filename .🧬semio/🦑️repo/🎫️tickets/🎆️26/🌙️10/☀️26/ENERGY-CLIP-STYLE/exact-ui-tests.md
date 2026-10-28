# Exact UI Validation

All final checks passed against the frozen production browser renderer.

| Final Check | Result | Runtime |
| --- | --- | --- |
| Exact UI Suite | 9 tests passed | 8.832 s |
| Registered Browser Lifecycle Bridge | 1 Python test; 8 JS cases passed | 0.507 s |
| Five Encoded Lessons and Deterministic Overview | 2 tests passed; all five lessons verified | 115.078 s |
| Rendered Pointer, Navbar, Footer, Tree and Popup Paint | 3 tests passed | 65.507 s |

The five encoded lessons each contain24 frames at320×180 and4fps. Every lesson has byte-identical decoded first and last RGB frames, and all five lessons share exactly the same boundary frame. Every original content frame appears in order inside the new focused viewport and satisfies the independent Pillow RGB error bound. Audio lasts6.000 seconds, with silent leading/trailing padding, middle RMS0.08832 and a440Hz Fourier peak.

## Independent Oracle

The retained exact-browser-oracle.mjs imports the real WindowChrome, WindowSilhouette, Navbar and Footer implementations, compiles the actual Tailwind UI stylesheet, loads the canonical Anta WOFF2, and renders through the repository-managed pinned Chromium adapter. Its composition is independent from the production browser component. It raises on missing fonts or browser errors. No imitation stylesheet supplies the expected geometry.

Portable cases and permanent tests live at presentation/📐️geometry/🧪️tests/{🔣️.cases.json,🐍️.py}. The isolated runner is run-exact-ui-tests.py. The production Python render_batch test checks actual DOM metadata and strict pixels against the independent oracle.

At1600×900, a400×300 window at(50,50), labeled1.1 Luft, has title width41.53125, cap height22.390625 and a22.390625-square control atx427.609375. Its11.2px Anta label uses11.2px line height and3.2px padding. The SVG border has a1px inset and the same U-shaped outline as the owning UI. Both insets are checked across five actual TypeScript cases, including oversized titles, a192px clamp, merged sub-epsilon gaps and narrow windows. The complete3+2 window grid fills the canvas with3.2px gaps and insets.

The nominal22.4px dimensions quantize to Chromium's1/64px layout grid, so portable native geometry uses0.04px tolerance. The production browser metadata fixture matches its independent DOM measurements directly. Pinned Chromium window glass isRGB(5,25,30); the transparent U-gap isRGB(0,17,23). Production pixel assertions have no tolerance.

## Coverage and Limits

Twelve portable font-shaping cases match independent Canvas advances within0.04px. Native authored glyph edges also match the browser baseline and inline origin within1.5px to accommodate hinted raster bounds versus continuous native outlines. A native camera stroke has normalized raster coverage1px within0.02px. These native authoring checks do not substitute for production browser pixel checks.

All six surface levels and an explicit opaque backdrop have exact computed CSS values. The analytical native surface port allows one channel unit against pinned Chromium raster quantization. The retained true nested-window observation is separate: browser BackdropRoot layers cannot be reproduced by flattening their parents to RGB. Production renders those layers with the actual browser UI. No empirical color offsets were introduced.

The cursor regression fixtures use the current navbar/footer,22.4px cap/control,16px icon and a fullscreen control parented directly by its window. Their Pillow oracle intersects polygons with half-open rectangles. Eleven interaction tests, three interval tests, four authored helper tests and eight presentation contract tests passed in bounded runs. Source closure checks include geometry and typography owners. The authored helper regression checks camera center(40,-15), cursor scale8/807.2, event order, cancellation, entry/exit intervals and the explicit parent border handle.

Normal window hover and press feedback remain covered by actual rendered pixel and ancestor checks. The independent actual UI oracle confirms that hovering the body of an already active window leaves its enclosing shell byte-identical. The movie regression now asserts that measured behavior instead of relying on an invented root/canvas border. Root/canvas edge pixels are explicitly checked.

## Red-to-Green Evidence

The original typography helper failed all12 CSS font-size cases before its replacement. A pressed-and-hovered control failed because the pure projection returned primary#ff344f while the real source CSS paints active-hover#fd5a6d. The one-line projection correction and portable case pass; an unhovered press remains primary.

The first actual-browser encodes correctly failed strict boundary identity for all five lessons, despite deterministic fresh overviews. The retained-layer history changed thumbnail rasterization. Production now remounts the canonical overview UI and waits for current silhouette measurements, while keeping canonical1600×900 CSS coordinates and using device pixel ratio for output resolution. Direct raw-PNG boundary checks and all five final decoded movie checks now pass with no copied boundary pixels and no relaxed equality assertion.

The initial system Edge fallback exposed1-bit compositor differences; final oracle and production tests both use the pinned repository browser. All commands ran through repo. Generated evidence is confined to the ticket generated directory and is disposable after review; the input scripts and this report are retained.

## Final Popup and Source-Membership Audit

A final visual audit found a real composition error that the former tree test's projected styles did not detect: the navigation Tree was nested under a Navbar button, so ancestor hover selectors could paint the entire popup gray and brighten every sibling. The production composition now uses the actual Popover, PopoverTrigger and portaled PopoverContent with Tree as its direct child.

The independent oracle now renders that same owning UI pattern separately and records portable paint goldens. Normal labels are oklab(0.70831 -0.0076355 0.00975207); only the hovered leaf and its real owner become rgb(247,243,227). Row and label backgrounds remain transparent, menu glass retains its computed color, and the popup remains outside Navbar DOM ancestry. The new production test asserts these actual DOM values, plus byte-identical unhovered-sibling background pixels and absence of the flat gray fill. It then moves to the next leaf in one browser batch, proving the prior leaf returns to normal while its sibling becomes emphasized. The targeted stateful popup check passed in7.774 seconds. All three rendered-frame tests passed in65.507 seconds.

Source-cache membership received a separate red-first test with an isolated temporary candidate tree and a mocked compiler frontend. The former known-input-only cache failed to discover a newly added nested TSX file. The fixed cache passes warm reuse, new candidate addition, ignored Markdown, candidate removal and a candidate arriving during compiler discovery. No actual UI source was modified by this test. The red run failed in0.184 seconds; the final extended case passed in0.207 seconds. Authoritative scan_sources metadata supplies scanner roots and glob patterns.

The final nine-test exact UI suite passed in7.383 seconds. The five closed-menu movie checks remain the historical115.078-second all-pass validation; they were not repeated for the popup-only composition and cache-membership changes.

## Native Fused-Cap Border Ownership

A bounded native helper regression failed before the fix because a fused cap control declared border width0 but the interaction painter reapplied width1. The portable borderOwnership fixture now checks normal, hover, press and release states. The native button factory must retain the width marker, and the cursor helper must preserve it through the actual activation callback and leave phase. Standalone controls retain width1. The red assertion was1 != 0; all five native helper tests now pass in3.280 seconds. Browser UI and encoded movies were unaffected and were not repeated.

## Zero-Touch Browser Provisioning

Eight portable lifecycle cases validate healthy launch, the single missing-executable provision/retry path, unrelated errors, misleading missing-file text, provision failure, retry failure, cancellation before launch and cancellation during provisioning. The cases failed before the private helper existed and now all pass. Errors preserve their original identity, unrelated errors never install a browser, and cancellation prevents subsequent work.

The retained run-browser-cold-smoke.mjs also triggers a real Playwright default missing-executable exception using an isolated owned cache path, then retries with the existing provisioned browser. The mock-provision smoke passed in4.10 seconds without a download or mutation of the installed browser cache. A second smoke invoked the actual registered playwrightV1 install chromium adapter with inherited capacity, cwd, AbortSignal, stderr output and progress reporting. It passed in3.98 seconds; the already installed artifacts required no download. The recovered browser loaded a real page successfully and was closed. Progress emitted running/finished DEBUG events on stderr.

These lifecycle checks changed no UI composition or movie frames. The private helper remains behind opaque launch/provision ports and is included in the browser dependency closure before certification. New retained inputs are browser/🧪️tests/{🔣️.cases.json,🟨️.mjs} and the ticket cold-smoke script; generated smoke evidence remains disposable under generated/browser-cold.

## Standard Test Registration

The authoring commands.test list now includes the geometry test module and a Python adapter for the co-located JavaScript browser lifecycle cases. Static validation first failed because the required registration was missing, then passed after insertion. Every one of the16 existing module registrations observed in the file was preserved in order, with18 total and no duplicates. TypeScript syntax validation passed.

The Python lifecycle adapter invokes the existing runtime.run_command interface with repo tools exec --light node. It confines its log to an owned temporary folder under SEMIO_TEACHING_OUTPUT_ROOT, confirms every portable case ran, removes the folder on success and retains failure diagnostics. The bounded adapter check passed in0.507 seconds, and the geometry suite passed all nine tests in8.832 seconds. Together these are10 registered Python tests, with the lifecycle bridge executing eight JavaScript cases. The unrelated course modules were not run.

Registration changes are confined to authoring/📜️script.ts and browser/🧪️tests/🐍️.py, plus the ticket runner's optional --browser mode and this report. No UI pixels or movie composition changed.
