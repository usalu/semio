STATUS: EDITS DONE; the demonstrator `test` target did NOT run: `repo --tool=bazel run` fails twice in the peer-owned `repo:generator-inputs:prototype` step (`Registry implementation import is not schema-owned: @semio-tech/framework-pack-json-ts/members in 🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🟦️.ts`), logs `🗑️generated/1101/uifix-test1.txt` and `uifix-test2.txt`. Verified instead: Python twin 12 tests OK (1 skipped, no jsonschema), bun runtime check of the resolver table, pane views in both locales and transpile of all edited TS/TSX; vitest and tsc UNVERIFIED.

Root `D` = `♻️mit-bestand/🧺️demonstrator`.

## What changed
- German lock removed: no `lockedLocale` in `D/🔨️modules/🌐️locale/🔣️.json` or its schema, no `DEMONSTRATOR_LOCKED_LOCALE`/`DEMONSTRATOR_LOCALE`, no `locks.locale` on the 8 brands, no `initUiLocaleSync("de")` at boot, no `"de"` defaults in the footer (`fundedByZukunftBauFooterItem(key, locale, iconOnly)` and `aProjectOfLuhUdkFooterItem` take all three explicitly).
- Language resolution (`🌐️locale/🟦️.ts` `resolveDemonstratorLocale`, `⚛️.ts` `readDemonstratorLocale`/`commitDemonstratorLocale`): stored choice, else the browser's preferred languages (first supported tag), else a picker (`⚛️locale-picker.tsx`, English and Deutsch each in its own `lang`, nothing preselected). A navbar switcher (`aria-pressed`, localized group label) changes language at any time; the committed choice is persisted, sets `<html lang>` and `document.title`. `🌐️.html` has no static `lang` and a neutral title.
- Panes are locale functions: `DemonstratorPaneSpec.text(locale)`, `demonstratorPaneViews(locale)` (label, tagline, description, windowTitle, shortWindowTitle from the catalog; localized brand with footer credits). Static brands keep `windowTitle` = project name (proper noun) and have no footer items or locale; `demonstratorFooterItems(locale)` and `demonstratorGeneralIntroduction(locale)` replace the former constants (logo alt texts are localized now). Shells get `initialLocale={locale}` (no lock); switching language remounts the overview.
- 16 new brand keys `demonstrator.brand.pane.<id>.{windowTitle,shortWindowTitle}`, 4 landing/locale keys (146 keys total, all en+de).
- Footer mobile: the landing passes `iconOnly = viewportMobile`; logo URLs derive from `DEMONSTRATOR_ASSETS_DIR`; component exports removed.
- Tablet: `device = elementsSurfaceDeviceForMatches({mobile: UI_MOBILE_MEDIA_QUERY, tablet: UI_TABLET_MEDIA_QUERY})`, no storage override.
- Keyboard parity: focus-visible emphasis in `🎨️globals.css`.

## Tests
- Acceptance spec `🧪️tests/🎭️acceptance/🟦️.ts` (re-read, edited by fail-closed replace): explicit `locale` per case (`en-US` default, `de-DE`, `fr-FR` for the picker), all expected wording read from the catalog: `overview chrome en|de`, `funding logos en|de`, `language picker`, `language switch`. Titles stay within 24 chars. NOT run (no serve/test-e2e per brief).
- `🧪️demonstratorlocale` (TS + Py, shared `🧫️cases.json`): language-agnostic `localeResolution` table (7 rows), no-lock/no-default-language source scan, html without static `lang`, window titles never literal, identical-cell allowlist extended by 4.
- `🧪️demonstratorpanedescription`: two paragraphs and distinct catalog window titles in both locales. `🏛️shell-brand`: localized titles/footers in both locales.

## Open / not done
- Standalone dev shells (`🐚️shell`, vite config) use the static brands: window title is the project name only and no partner credits, because a static brand cannot name a language.
- M5 (landing `<h1>`/landmark), L1 (small text), L2 (overflow clip), M2 partner-table dedupe between `footer.tsx` and `brand.ts`: untouched.
- Rendered behaviour (switcher clickable inside the pointer-events-none navbar wrapper, remount on switch) unverified in a browser.
