# Staffage Phone Usability and Performance

The phone overview and catalog use native vertical lists. Every archive destination has a direct link, and category and all-assets pages can render their full index from the manifest. Large screens keep the existing layered grid. Short coarse-pointer landscape viewports use the phone layout.

Opening a phone asset route requests only its selected catalog document. The detail lets the reader cancel while loading and retry after cancellation or failure. Advanced search and sorting still load authored category data when needed. Back and Up restore the previous list scroll position. Viewer stages start paused on phones and keep vertical page scrolling, with touch-sized controls and a scrollable language gate.

## Verification

- The original source failed both new phone-navigation regressions before implementation. The final run passed 62 React tests across the grid, search and filter, language selection, detail, viewpoint and model-viewer scenarios. The Staffage React TypeScript check also passed.
- Playwright Chromium at 320 × 640 exercised all 13 overview links, vertical scrolling without horizontal overflow, 44-pixel link and button targets, a 16-pixel search input, explicit clip playback, browser Back scroll restoration and the all-assets filters. Navigation to the overview, category and all-assets index triggered zero catalog document requests. Opening Walking Commuter triggered one request for that asset.
- Playwright Chromium at 844 × 260 with French browser preferences and 200% text confirmed the language gate scrolls. Choosing German entered the phone layout and set the document language to German.
- The full Staffage React run under the normal sandbox reported separate failures in the authored archive invariant and the playground geometry oracle. The catalog invariant found 136 ready and 16 invalid records in the shared checkout. Examples include Multi-stem Birch solids below the 75 mm minimum for 1:500 and missing body solids for Ivy on Trellis. The catalog asset files and their validators were outside this change. The geometry oracle could not access its existing uv cache from the sandbox.

The browser checks are reproducible with [staffage-phone-browser.mjs](./staffage-phone-browser.mjs). They use full Chromium in headless mode. The separate Chromium headless shell supplied zero animation frames on this host, which prevented Playwright from declaring tap targets stable. Verification used Chromium emulation.
