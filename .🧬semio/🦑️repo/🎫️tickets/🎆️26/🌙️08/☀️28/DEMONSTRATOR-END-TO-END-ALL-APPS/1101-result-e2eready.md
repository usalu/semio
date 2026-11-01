STATUS: READY, not run against a server (none allowed). Dry-load GREEN (`run.sh --check` 8/8); `tsc` rc 0 on the probe (`🗑️generated/1101/e2eready-tsc.txt`); a no-server run printed the verdict table and exited 1 as designed (`🗑️generated/1101/e2eready-dry/`). Every action path is UNVERIFIED in a real browser.

C = this ticket folder. D = `♻️mit-bestand/🧺️demonstrator`.

## Run it
`C/1101-tools/e2eready/run.sh <base-url> [--panes a,b] [--languages en,de] [--no-landing]` (serve = http://127.0.0.1:6029/). `run.sh --check` is the server-free dry-load. Output: table on stdout, plus `C/🗑️generated/1101/e2eready/{e2eready.txt,e2eready.json,run.log,run.rc,<unit>.png on failure}`; exit 1 on any FAIL. Env: `E2E_ACTIONS_ALL_LANGUAGES=1`, `DEMONSTRATOR_E2E_GPU=swiftshader` (verfolgen map cannot paint there), `E2E_OUT`, `BUN`. Browsers: `PLAYWRIGHT_BROWSERS_PATH` defaults to the repo cache (chromium-1248 present).

Table columns per unit: BOOT (shell reaches `data-shell-ready`, intro dismissed), CONTENT (every window of the pane carries real content by its surface kind), ACTION (one real action, assertion on visible state, undo/redo where the pane has it), CONSOLE (no pageerror, no console error except the exact 404 resource line, same policy as the spec).

Units: `landing@en`, `landing@de` (html lang, title, 8 cards, tagline per card from the catalog), `picker` (fr-FR browser: picker shown, nothing preselected, no cards; choose de; lang/title/cards; choice kept across reload), `switch` (navbar switcher de then en: aria-pressed, lang, title, tagline), then each pane in every language (`generator@en`, `generator@de`, ...). Actions run in the first language by default; boot, content and console run in all.

## Language-agnostic expectation file
`C/1101-tools/e2eready/expect.json` (schema `demonstrator-e2e-ready/1`): languages by code plus browser locale (no default), panes, windows with surface kind and content flag, one action per pane with element ids and per-language wording keyed by language code. Constants are not copied: action parameters are read from the spec's own case files (`🧬️edits`, `🧬️simulation`, `🧬️curation` under D/🧪️tests/🎭️acceptance), landing wording from the demonstrator catalog, pane ids from `🪧️brand.ts`.

Actions (all assert visible state and then undo/redo where applicable):
- generator `graphValueEdit`: select height widget, edit in inspector, graph value and preview geometry change, undo and redo restore.
- koordinator `catalogueAdd`: add box from catalogue, new instance in the shape window only, document row, undo/redo.
- aggregator `catalogueAdd`: add kind, new instance in both views, row, undo/redo.
- energie `toolRun`: run, finalized status, final step, accepted result; restart and abort leaves no result.
- aussuchen `rowSelectPreview`: placeholder, select stock row, preview shows exactly that object.
- bearbeiten `rowToggle`: toggle step, inspector true to false, geometry changes, undo/redo.
- verfolgen `featureAdd`: add feature, position count +1 and only that, undo/redo.
- statik `paramScale`: phase and deformation scale, geometry changes, undo restores.

## Dry-load proof (`run.sh --check`, no server)
Pane ids equal the brand roster; window kinds equal the acceptance spec's `PANE_CASES`; every pane has a known action and resolvable cases; both languages complete for all wording used; no default language in the file; headless chromium reads a fixture DOM with the production attributes (world, graph, table, tree, tool-run id, alias selector, window id folding); table renderer. If the spec, brand or catalog drifts, the check goes red before any browser run.

## Spec and probe updates for language selection
- Acceptance spec D/🧪️tests/🎭️acceptance/🟦️.ts was already moved to explicit locales by lane `uifix` (overview en|de, language picker, language switch, funding en|de, wording from the catalog). Here: only the stale energie note ("locked German locale") corrected. The per-pane tests (generator, koordinator, aggregator, bearbeiten, verfolgen undo-redo; aussuchen preview and undo-redo; energie run-cancel; statik undo-redo) already prove one real action per pane with console policy from the auto fixture, and their wording regexes accept both languages.
- `C/🔍️landing-eight-pane-probe.ts`: no German skip text; locale via `PROBE_LOCALE` (default en-US); skip by element id; intro matches accept both languages.

## Known gaps
- statik caption `Case: dead` and fem phase wording come from the fem plugin and are English-only literals in the guest (`📊️results/🦀️.rs`); the expectation keeps `simulation.fem.caption` from the case file, so the German run of statik will fail on a missing translation if the guest is not localized. Run statik with `--languages en` to separate that from other faults.
- Selectors are the production ones the spec uses (`data-slot="demonstrator-locale-picker|switcher"`, tab ids, `framework.panel.*`); a rename in `⚛️locale-picker.tsx` shows up as a boot FAIL in `picker`/`switch`.
- Official gate stays `test-e2e` (Playwright spec). This runner is the fast per-pane triage in front of it; `tsc` ran with zero diagnostics (a deliberate-error control was not run).
