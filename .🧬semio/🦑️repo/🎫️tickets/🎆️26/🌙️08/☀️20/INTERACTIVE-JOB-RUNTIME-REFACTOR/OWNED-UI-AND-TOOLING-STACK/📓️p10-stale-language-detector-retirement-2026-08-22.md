# Stale Browser Language Detector Retirement

Date: 2026-08-22
Source scout: `📓️terra-next-stale-language-detector-scout-2026-08-22.md`
Verdict: **implementation gates verified; dependency count is provisionally 138 = 75 JavaScript + 63 Rust pending the independent Terra acceptance audit.**

<!-- #region Implementation -->

## Implementation

- Removed the sole `i18next-browser-languagedetector` import and plugin registration from the UI React barrel.
- Removed the ignored `detection` configuration while preserving `resolveRequestedUiLocale`, the closed `de* → de` / fallback `en` policy, explicit `lng`, synchronous initialization, and the public i18n port.
- Removed the sole direct manifest edge and reconciled `bun.lock` with Bun. The direct `8.2.1` resolution is gone; `@babel/runtime` remains independently reachable.
- Added permanent in-source regressions for locale normalization, explicit persisted/document/i18n initialization, German chrome resolution, and absence of the retired import, binding, and configuration.
- Added a Vite-served browser harness that seeds locale state before dynamically importing the actual production barrel, mounts the actual public `NotFound`, and records first-paint locale and label evidence.

### Files

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `bun.lock`
- `🧪️p10-stale-language-detector-browser-gate.html`
- this report

<!-- #endregion Implementation -->

<!-- #region Validation -->

## Validation

### Focused and full UI

| Command                                                                                               | Result                                                |
| ----------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- -t 'owned locale detector retirement'` | PASS — 3 focused tests; 720 skipped; 723 total.       |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                          | PASS — 20 files; 723 tests.                           |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                           | PASS.                                                 |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                | PASS.                                                 |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                                 | PASS — no violations; two existing allowlisted files. |

### Manifest, lock, and census

| Command                                                                                                                                             | Result                                                                                                                                            |
| --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary`                                                                           | PASS — lockfile saved by Bun.                                                                                                                     |
| `bun install --lockfile-only --frozen-lockfile --ignore-scripts --no-progress --no-summary`                                                         | PASS.                                                                                                                                             |
| `bun ./📜️script.ts verify dependencies`                                                                                                             | PASS — baseline 238, current 138, 100 removed, no new third-party dependencies.                                                                   |
| `bun ./📜️script.ts verify dependencies list js --format json`                                                                                       | PASS — detector absent.                                                                                                                           |
| `bun ./📜️script.ts verify dependencies list rust --format json`                                                                                     | PASS.                                                                                                                                             |
| `bun ./📜️script.ts verify dependencies list js --format json \| bun -e 'const rows=JSON.parse(await Bun.stdin.text()); console.log(rows.length)'`   | PASS — `75`.                                                                                                                                      |
| `bun ./📜️script.ts verify dependencies list rust --format json \| bun -e 'const rows=JSON.parse(await Bun.stdin.text()); console.log(rows.length)'` | PASS — `63`.                                                                                                                                      |
| `bun ./📜️script.ts verify dependencies parity js`                                                                                                   | PASS — 83 manifests, 260 external rows, 110 evidenced, 150 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 lock fixtures. |

Ratchet evidence is exactly **138 = 75 JavaScript + 63 Rust**.

### Absence and API scans

- `rg -n --hidden --glob '!**/node_modules/**' --glob '!**/dist/**' --glob '!**/target/**' --glob '!.🧬semio/**' --glob '!.cursor/**' 'i18next-browser-languagedetector' package.json bun.lock 🧰️framework ✏️s 🌎️hub ♻️mit-bestand` — PASS, no matches.
- `rg -n 'LanguageDetector|\.use\([^)]*LanguageDetector|detection\s*:' <UI React barrel>` — PASS, no matches.
- Fixed-literal scans of the UI barrel, its manifest, and `bun.lock`, plus dynamic `import()` / `require()` and downstream-binding scans — PASS, no current detector identity, loading form, binding, or exported API.

The repeat broad scan found only non-current evidence plus one explicitly out-of-scope Compose build stub: historical `🔒️dependencies.json`, `.cursor/plans`, ticket records/artifacts, and `compose/client/bin/engine/js/vite.mcp-app.config.ts`. No source change was made outside the authorized scope.

### Formatting and diff hygiene

| Command                                                                                                     | Result                                            |
| ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `bunx prettier --write <UI React barrel> <UI React package.json> <browser harness>`                         | PASS — formatted.                                 |
| `bunx prettier --check <UI React barrel> <UI React package.json> <browser harness> <implementation report>` | PASS — all matched files use Prettier code style. |
| `git diff --check -- bun.lock <UI React barrel> <UI React package.json>`                                    | PASS — clean.                                     |

<!-- #endregion Validation -->

<!-- #region Browser -->

## Actual Browser Gate

The `browser:control-in-app-browser` skill required the production module to be exercised through the in-app Browser, so the fixture dynamically imports the Vite-transformed production `@semio-tech/ui-react` barrel only after setting bootstrap inputs and mounts the actual public `NotFound` component. This prevented duplicated parser or mock-component evidence from being treated as runtime proof.

Vite command:

`bunx vite --host 127.0.0.1 --port 4180 --strictPort`

Vite became ready in 199 ms; a direct HTTP HEAD request to the literal stored-case fixture URL returned `HTTP/1.1 200 OK`.

This worker's repository-root Vite log also reported two non-product diagnostics: its global dependency scan could not resolve the explicitly out-of-scope `@semio-tech/compose-sketchpad/boot` import from `compose/client/lib/sketchpad/play/index.html`, and the unavailable worker-browser attempts reached the emoji path in percent-encoded form that Vite rejected with `No matching HTML proxy module found`. Neither diagnostic was treated as runtime proof or repaired in this packet; the coordinator's independent production-barrel run below supplied the accepted real-browser evidence.

Cases:

- `...?case=stored`: PASS — storage was set to `ui.chrome.locale=de` before import; navigator override was `unused`; resolved locale was `de`; resolved and rendered labels were `Zurück`; heading was `Fehlende Seite`; `first-paint-ready` was `true`; gate error was empty; console warnings/errors were zero.
- `...?case=navigator`: PASS — storage was absent before import; navigator override was `de-AT`; resolved locale was `de`; resolved and rendered labels were `Zurück`; heading was `Fehlende Seite`; `first-paint-ready` was `true`; gate error was empty; console warnings/errors were zero.

The independently recorded evidence is `📓️coordinator-owned-locale-real-browser-gate-2026-08-22.md`. This worker's own binding returned `No browser is available`; after reading the required `bootstrap-troubleshooting` documentation, the one permitted `agent.browsers.list()` call returned `[]`. The live Vite URL and exact assertions were therefore handed to the browser-capable coordinator. No unrelated browser driver or source-only substitute was used.

<!-- #endregion Browser -->

<!-- #region Residuals -->

## Residuals

- No product-code residual is known for this dependency retirement.
- The actual-browser production-barrel gate is recorded and passing; the overall 138-identity state remains provisional only for the requested independent Terra acceptance audit.
- The broad historical and Compose-stub literals are outside this serialized packet and do not declare, resolve, import, register, or export the retired package from the current UI package.
- The worker-owned Vite server was stopped after the coordinator recorded the independent browser gate.
- No Cargo command, Compose edit, P3/P8 edit, ticket lifecycle call, or modifying Git command was used.

<!-- #endregion Residuals -->
