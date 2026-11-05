# Browser Preflight

Date: 2026-11-04

The repository browser is ready. Executed `repo tools verify chromium --lang=en`: **exit 0**, Chromium pin **1248**, found **1248**, status **ok**. The owned hook checks Chromium and the matching headless shell for executable access and host requirements.

Raw Playwright imports under Node and Bun had no `PLAYWRIGHT_BROWSERS_PATH`, so their missing-executable result referred to the user-global fallback. The first-party `loadPlaywrightV1` adapter selects the declared repository browser home before loading the library. Its matching executable exists at `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-1248`.

The retained `browser-export-probe.mjs` now uses `loadPlaywrightV1` and the shared `browserLaunchOptionsV1` policy. No installer, dependency synchronization, provider edit, or cache policy/partition change was needed.

Relevant source owners:

- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🎭️playwright/🟦️.ts`: repository browser environment adapter.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts`: portable launch policy and executable/host probe.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/🔣️.json`: pinned Chromium row.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⌨️command/🟦️.ts`: owned verification and provisioning dispatch.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🌐️system/🟦️.ts`: controlled browser setup through the existing native dependency phase.

If provisioning becomes necessary, the tool table's Chromium row delegates to its owned native-dependencies `sync browsers` recipe. No raw installer is required. The serve remains stopped pending native acceptance; browser application acceptance has not yet run.

## Goal Continuation Revalidation

Re-read the current retained probe. It still uses the repository Playwright environment and launch policy, explicit English browser locale, complete initial/created/undone document downloads, exact 256×256 PNG/SVG dimensions, decoded foreground evidence, and the actual viewer surface/fault checks.

The probe passed a fresh Node syntax check through `repo tools exec --light node -- --check` (exit 0). A fresh `repo tools verify chromium --lang=en` again passed (exit 0, pin 1248/found 1248/status ok). No installation or source changes were required. No application/browser startup was attempted; the serve remains stopped for the agreed native-first sequence.

The existing Sharp image oracle also loads successfully under the owned Bun route (libvips 8.17.3); no dependency setup was required. This is prerequisite validation, not image-export acceptance.

## Visible Notice Contract

The current transient-notice owner is `ShellHost/🍞️transient-notice/🟦️.tsx`, rendering `[data-semio-transient-notice]` with `data-notice-code`, alert/status role and assertive/polite live semantics. The retained probe now observes that exact UI owner passively and retains code/text through a Playwright binding across navigation. It requires `icon-render-export.done`, rejects visible failed code/message before and after Viewer startup, and retains `notices.json` even on failure. It does not import private ledger state or inject application/native authority. The updated source passed its owned syntax check; no browser or server was launched for this preparation.
