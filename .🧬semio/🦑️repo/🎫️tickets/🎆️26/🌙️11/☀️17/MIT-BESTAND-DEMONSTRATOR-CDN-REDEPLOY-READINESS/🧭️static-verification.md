# 🧭️ Static Verification

How every app is checked against the built release bundle, outside the repo graph.

- Serve the build output folder at the root of a local static server: `python3 -m http.server <port> --bind 127.0.0.1 --directory <output>`.
- The eight apps are hash routes: `/#generator`, `/#koordinator`, `/#aggregator`, `/#energie`, `/#aussuchen`, `/#bearbeiten`, `/#verfolgen`, `/#statik` (the names of `🪧️brand.ts`; `🔨️modules/🧩️runtime/🔣️.json` still says `energy` and `fem3d`).
- Playwright: `PLAYWRIGHT_BASE_URL=http://127.0.0.1:<port>/ PLAYWRIGHT_BROWSERS_PATH=".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"` with the config `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts` and `--grep " boot$"` (16 runs, `en` and `de`). Chromium 1248 is installed; `playwright-core` resolves only while the dependencies image is mounted.
- `verfolgen boot` needs painted map tiles; the release bundle embeds the prefetched Switzerland z0–10 tiles under `/osm` and `/vt`.
