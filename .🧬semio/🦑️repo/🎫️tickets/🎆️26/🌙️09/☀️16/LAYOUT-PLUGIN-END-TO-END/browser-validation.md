# Browser Validation

2026-10-28. The current launcher probe route now runs browser-journey.mjs. Existing older probe inputs are retained; their earlier checks are superseded by this journey and the version-2 browser-journey.json contract.

## Current UI Witnesses

The harness resolves exactly one visible Artifact panel using its declared panel/active-tab attributes. Every frame/page/selection snapshot comes from that panel alone. Active owner and all preimport window IDs come from actual mode dock tabs, independently of the focused-program panel.

Boot requires the selected layout ready beacon, no shell error, no visible window/shell/history fault attributes, healthy surface phase/fault payloads, a real visible canvas, and an independently decoded nonuniform canvas screenshot. The Artifact baseline then requires the exact two demo page IDs and the expected demo frames. The first Inspector edit additionally requires the initial story value to equal Hello layout.

Baseline SVG/PNG downloads precede document edits. The addFrame form is expanded and populated through its actual scoped argument rows, with kind selected via its actual custom Select trigger, aria-controls portal listbox, and option data-value and x=251/y=337 entered in the numeric inputs. Actions and Artifact are open before the measured baseline. Execute receives keyboard focus and a trusted Enter; no forced click or synthetic private dispatch is used. The same mounted Artifact DOM must then gain exactly one stable frame ID with no removed frames/pages. Undo follows immediately, with no panel/selection or other gestures; DOM polling must recover the exact pre-create arrays. Redo follows immediately and must recover the exact post-create arrays. The evidence record includes all three snapshots.

The story marker LAYOUT-EDIT-731 is authored through the real Inspector textarea under layout-play-inspector.global.story.story-1.content and committed by actual Tab blur. The textarea locator is resolved again after the fingerprinted chunk rerenders. Final SVG, PNG, and PDF downloads happen only after this edit. jsdom validates SVG structure; Sharp independently rasterizes SVG and decodes PNG at exact 400-by-500 dimensions. Both edited outputs must contain the expected rect fill throughout the 12-by-12 interior patch, differ from their respective baseline at at least 95% of those pixels, and agree with each other. pdfjs requires the actual authored marker and exact observed document page count; stock Hello layout text cannot satisfy the edited PDF witness.

The archive is downloaded after all edits and its exact frame/page IDs and marker are saved. The original then gains a page and a different original-only story marker. Import uses Playwright's ordinary browser file chooser rather than a nonexistent attached file input. The harness awaits a newly observed Blueprint owner absent from the complete preimport window ID set, focuses that owner's normal dock tab, reopens and resolves Artifact, and requires exact saved frame/page arrays. Inspector in the focused new owner must recover the saved marker and exclude the later original-only marker; the new Artifact must exclude the original-only page.

All recorded steps check visible surface and host status fields and console/page errors. Missing, duplicate, hidden, or disabled action argument controls fail the journey. Zero-argument actions execute on their row and are not falsely required to expose Execute. All output paths remain beneath the ticket generated folder. The overall deadline and process signals close the browser and retain failure reports/screenshots.

## Executed Checks

All checks used the registered repo tool interface with explicit workspace cwd.

- Syntax: repo --service=off tools exec --light node -- --check <ticket>/browser-journey.mjs passed.
- Launcher parser: repo --service=off tools exec --light python -- <ticket>/launch-layout.py --help passed.
- Independent media/contract laws: repo --service=off tools exec --light node -- --test <ticket>/media-oracles.test.mjs passed 3 tests, zero failures, about 1.92 seconds.
- The tests validate the language-neutral journey contract, rasterized-SVG/PNG patch agreement, actual before/after changes, exact dimensions, nonuniform canvas content, authored PDF marker/page count, and refusal of wrong fill, unchanged geometry, blank canvas, wrong dimensions, stock PDF text, wrong page count and unsupported formats.
- Existing third-party libraries are jsdom, Sharp/libvips, pdfjs-dist and the existing native canvas package. They remain test-only dependencies. pdfjs standard-font data resolves from its installed package without network fetches.

## Live Prerequisite

No live browser pass is claimed. The current 6079 server is unavailable until the root task finishes component activation/staging. No attempt to run the browser against a stale or refused stage was made. Coordinate with the root task before the first live probe; it owns the activation retry and full log. The next live command is the retained launcher probe route, which now selects the current journey.

## Files

- browser-journey.mjs: current public-UI journey.
- browser-journey.json: portable expected state/media contract.
- media-oracles.mjs: independent actual downloaded-media and canvas readers.
- media-oracles.test.mjs: executed positive/refusal laws using the same contract.
- launch-layout.py: probe route updated to the current journey.
- This report.

No runtime UI API, private command hook, product source, or AGENTS.md changed in this browser work.

## Version-2 Review Corrections

The current Select implementation has no native select element. The helper now activates the scoped combobox trigger, follows aria-controls to its exact visible portal listbox, clicks the rect option by data-value, and verifies the retained aria-selected value in a reopened popup before closing it with Escape. All those gestures finish before the measured create/undo/redo baseline. No selectOption, force-click, synthetic event dispatch, or private hook is used.

Surface checks now consume only actual published status attributes: generic/world data-status-json and Board data-board-status-json. Present malformed/non-object JSON fails; Board snapshotParsed=false or nonempty refusalReason fails. Missing Canvas2d/WasmCanvas status remains valid because those hosts publish none. Existing shell/window/history fault attributes, console/page errors, and owner-scoped canvas captures remain independent witnesses. Canvas captures now require both contrast greater than16 and channel standard deviation greater than2, rather than accepting a single changed pixel; the actual measured statistics are retained in the boot report.

The600second deadline is installed before Chromium launch and covers browser work and Node-side media readers. Deadline/SIGINT/SIGTERM is recorded distinctly, closes Chromium, and schedules a5second hard-exit grace if work does not return. Final browser close is bounded to2seconds; a failed close records failure and exits after an additional bounded grace. These lifecycle paths were source-reviewed, not exercised against a live stage.

After correction, browser-journey.mjs syntax validation passed and the same registered independent media suite passed4tests, zero failures, about4.57seconds. The new fourth test uses the portable JSON status cases plus jsdom to cover absent Canvas status, valid/pending Board status, malformed generic/Board JSON, primitive JSON, refused snapshots, refusal reasons, fault payloads and failed phases. No live browser success is claimed; activation/staging remains the root-owned prerequisite. Added browser-health.mjs contains only test-harness status interpretation.
