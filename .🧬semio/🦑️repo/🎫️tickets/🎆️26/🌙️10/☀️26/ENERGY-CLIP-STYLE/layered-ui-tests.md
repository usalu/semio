# Layered UI Tests

Final verification is GREEN across the current 25 scoped checks: 14 hierarchy/browser tests, 9 contract/publication tests and 2 native/media tests. The combined run completed 23 of 25 successfully in 950.891 seconds. Its two contract failures came from the already-loaded former manifest assertion and a concurrent `render.mjs` edit; the separate current contract run passed all 9 tests in 56.562 seconds and supersedes those two results. This is verification across those runs, not a claim that the initial combined invocation had no failures.

The portable hierarchy fixture covers six topics in the requested order, English and German titles, and parent ownership of authored videos. All five energy videos and the six BIM videos added concurrently by another authorized agent are preserved. Topics without authored videos retain empty lists. The first run failed all three tests because the hierarchy implementation did not exist, establishing RED before production changes. A NumPy oracle independently checks the row-major cells.

The independent browser oracle imports the real shared `LayeredOverview`, `OverviewCard`, `OverviewCardOpenChip`, geometry module and compiled design-system CSS. It uses Chromium through the repository Playwright adapter. It checks the same interaction semantics as the course's existing layered-home tests: pointer motion changes the strip, the glass has blur, every card has a bottom-left localized view chip, chip hover reveals an inert pane at the exact overview bounds, clicking opens it, and Escape returns. Both six-topic and five-video scopes pass in English and German under a reduced-motion environment.

Verified command: `repo capacity run --memory=2048 --cpu=1 -- repo tools exec --light node -- <browser-tests>/🥞️layered/🟨️.mjs <ticket>/🗑️generated/layered-tests`.

Result: four browser scenarios passed. Generated measurements and images are temporary evidence under the ticket's generated directory. The oracle is registered in the regular browser Python test module.

The production comparison passes fourteen actual rendered frames across English and German. It checks topic and energy video reading order, moving root and inactive nested preview strips, moving active energy strips, localized view chips at the bottom left of every card, the exact shared-component glass colors and backdrop filters, exact full-overview hover placement, inert revealed previews, and interactive opened video panes. It reads the renderer's actual DOM metadata and compares glass against the separately bundled shared-component oracle.

The initial six targeted tests passed in 19.745 seconds. The hierarchy and browser tests are registered in the regular authoring script. The fixture uses authored energy video IDs (`physical_fundamentals`, `ventilation`, `heating`, `cooling`, `energy_balance`); course quiz routes use a different taxonomy and are not substituted for video IDs.

Verified targeted runner: `repo capacity run --memory=4096 --cpu=1 -- <existing-teaching-python> -X utf8 -B <ticket>/run-layered-tests.py`. This reads the existing teaching environment without provisioning or changing it.

## Current Regression Results

The flat window fixture was replaced with topic-owned videos and independent three-second focus and restore timing contracts. Existing geometry, typography, surface, source-closure, publication failure, cursor interval, scene-helper and media-preservation assertions remain covered. The obsolete native-camera shell test was removed with its unused production shell implementation; actual Chromium silhouette, border and full-screen chrome checks remain.

The new production pointer test observes captured native `pointerdown`, `pointerup` and `click` events. At both topic and video levels, hover remains inert, press does not open, same-target release opens, and outside release cancels. The four-fps production choreography test captures all four real clicks: Energy, the selected video, its fullscreen restore control, and the parent-navigation footer. It also verifies that the final restore hold is already at the parked root state before the canonical endpoint, with less than half a CSS pixel of pane displacement.

The independent NumPy phase test checks both focus and restore commit boundaries just before and after the complete press duration. A reversed-JSON-key-order test exposed order-dependent choreography; it now reads the eight named stages explicitly. The dead flat `cursor_timeline` function, its seven timing fields, its phase list, schema entries and obsolete fixtures/tests were removed. The generic immutable pointer state machine remains covered by its Pillow/NumPy mask and gesture cases.

Concurrent catalog editing is supported through an explicit finite capture context and required catalog payload. Snapshot tests established RED before the context existed and now pass defensive input/read ownership, nested inheritance, restoration after exceptions and independent asynchronous contexts. No production data or disk content is monkeypatched. Both encoded tests capture their catalog once, so other agents can keep authoring topics without changing the model partway through an export.

Publication certificates include the actual captured catalog payload. The payload replaces only the mutable catalog file hash; every other code, asset and nested content guard remains. The permanent publication test verifies that changing the captured model changes the certificate while source hashes remain identical, and that modifying the returned certificate cannot mutate its input model. Source paths are normalized before checking catalog exclusion.

The shared handoff is a versioned image captured from the actual UI. Fresh DOM capture and metadata remain mandatory on every call; only the common root image bytes are shared. Portable cases cover eligibility and exclusion of media, menu, pressed, revealed and opened states. Tests verify invalidation by catalog, locale, viewport, tracked UI code contents, source membership, camera and cursor; equivalent JSON object order still reuses the same image. A source change during capture rejects the asset. These three tests pass in 0.910 seconds; the seven hierarchy/capture tests pass in 1.346 seconds.

Latest passing groups:

- Registered hierarchy and browser suite: all 14 tests passed in the final combined run, including required catalog packets, capture contexts, handoff identity/invalidation, the independently bundled shared-component oracle and authoritative native-event assertions.
- Generic interaction plus hierarchy tests after dead-code removal: 13 tests, 12.193 seconds.
- Geometry: all 8 tests passed; cursor intervals: all 3 tests passed; authored scene helpers: all 5 tests passed.
- Presentation contract/publication: all 9 current tests passed in a separate durable run, 56.562 seconds, with no failures or errors. The combined run had loaded an earlier energy-manifest selection assertion and encountered a real `render.mjs` edit at 23:25:18 UTC after that run began; the corrected energy selection and unchanged current-source hash assertion both pass in the separate run. The fixture places cached chapters under `de/speaker`, matching the current runtime locale isolation.
- Real popup hover: passed in 16.917 seconds after explicitly allowing CSS transitions to settle. Sibling ink and transparent glass styles remain unchanged while the backdrop pans beneath them; static backdrop pixels are no longer incorrectly expected to remain unchanged.
- Native topic/video focus, restore boundary and navbar/footer/tree hover checks passed. The current restore-timing four-fps event and pre-boundary hold checks passed.
- Current native/media verification: both tests passed in the final combined run. All five energy routes preserve exactly identical decoded first/last frames and common boundaries across lessons; every source frame appears in order, the fitted source image stays within its original viewport, and the audio retains its 440 Hz content with silent three-second padding on each side. Repeated standalone overview captures are pixel-identical. Each route wraps a two-second testsrc2/audio fixture into 32 frames at 4 fps; these checks do not claim a full authored-lesson rerender.
- Earlier attempts identified concurrent BIM/Optimization catalog edits, raw-browser raster variation and an insufficient 60-second per-lesson test budget under the expanded preview load. Explicit catalog capture, versioned actual-UI handoff images and a bounded 180-second per-lesson timeout resolved those issues. Strict pixel, frame-order and audio assertions were retained.

## Test-Owned Files

All paths below are relative to the workspace root. Temporary measurements, images, videos and logs are under this ticket's `🗑️generated` folder and must be removed when the ticket closes.

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🔣️.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧬️schema.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🥞️layered/🟨️.mjs`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🥞️layered/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🏁️handoff/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🔣️.cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE/run-layered-tests.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE/run-layered-contract-tests.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE/layered-ui-tests.md`
