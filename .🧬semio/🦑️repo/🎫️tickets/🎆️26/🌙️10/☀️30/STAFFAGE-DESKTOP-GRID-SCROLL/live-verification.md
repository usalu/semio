# Live Verification

Date: 2026-10-30. Goal `🎯updatedtutorials`. Ticket `26/10/30/STAFFAGE-DESKTOP-GRID-SCROLL` reopened for the remaining catalog findings.

## Cause

`CatalogStore.#issues` passed only rows whose `asset` was set. A row that failed `validateAsset` keeps `asset` unset, so `validateManifest` reported `manifest-asset-missing` (`Manifest entry "…" has no asset document.`) for a document that was loaded. Fifteen invalid component documents plus the components `manifest-count` finding were the sixteen findings behind “The manifest check reports 16 findings”.

`Loaded` now keeps `allAssets` for every loaded document. The manifest check reads that map. A failed `validateAsset` still leaves the row `invalid` and off the drawing path.

The current 107 catalog documents already pass `validateAsset` and `validateManifest`. The false missing-document finding was the store, and the dev server that showed it had stopped.

## Tests

`repo tools exec bun -- x vitest run` on `🎯️targets/⚛️react/🧪️tests/📚️catalog-loading/🟦️.tsx`:

- 7 passed, 0 failed (2026-10-30 23:36).
- `loads the authored archive as one hundred seven ready rows with an empty manifest check` passed (1696 ms): 107 rows, every row `ready`, `manifestIssues` `[]`, including `cantilever-balcony`, `glulam-beam`, and `steel-column`.
- `still shows an invalid asset document to the manifest check` passed: only the unauthored fixture entry is `manifest-asset-missing`.

## Bundles and servers

Both Vite builds finished with exit 0 and 108 JavaScript chunks (the app chunk plus one chunk per asset):

- `SEMIO_BUILD_MODE=prototype` cache `/home/deck/semio/.cache/b/p/m/vite/staffage-prototype` → `🌐️site/📦️packages/🟦️typescript/dist/prototype` (2.97 s).
- `SEMIO_BUILD_MODE=release` cache `/home/deck/semio/.cache/b/p/m/vite/staffage-release` → `dist/release` (2.41 s).

Nothing was listening on 6071 or 6072 before the restart. A new Vite dev server is on `http://127.0.0.1:6071/` (`--mode prototype`, `--strictPort`, cache `/home/deck/semio/.cache/b/p/m/vite/staffage-dev` created empty). Port 6072 had no static server to restart.

## Headless Chromium

Playwright 1.64.1, Chromium headless shell 156.0.8078.4, `http://127.0.0.1:6071/`, locale `en-US`. Script: `live-browser.mjs`. Every route waited until “Loading assets” and “Loading…” were gone.

Routes at 1440×900: `#/`, `#/all`, `#/components`, `#/components/cantilever-balcony`, `#/components/glulam-beam`, `#/components/steel-column`, `#/humans`, `#/animals`, `#/vegetation`, `#/furniture`, `#/objects`, `#/vehicles`, `#/machinery`, `#/compare`, `#/playground`, `#/collections`.

Strip routes again at 1280×640: `#/`, `#/components`, `#/humans`.

Results, all 19 visits:

- Console errors: 0. Page errors: 0.
- Body text never contained “manifest check reports”, “has no asset document”, “failed validation”, or “Not authored yet”.
- `#/` showed `107 authored · 0 planned` at both viewports.
- `#/all` showed `Shown: 107 of 107`.
- Cantilever Balcony, Glulam Beam, and Steel Column were on their pages.
- Every `[data-layered-overview]` was `data-mode="strip"` (one on `#/` and `#/all`, two on `#/components`, three on `#/humans`).
- The strip overlay is `overflow-y: auto`. At 1440×900 the components and humans overlays scroll (760 px content, 737 px view). At 1280×640 `#/` scrolls (613 / 588) and components and humans scroll (760 / 477).

No `🗑️generated` files were left in this ticket.
