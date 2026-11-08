# 1107 Result: lane behaviours

STATUS: baseline stale items fixed, 8 pane cycle cases x en/de WRITTEN, tsc GREEN, listing GREEN, NOT RUN in a browser (no activation yet: "written, not run").

## 1. Audit of baseline 25 x en/de and `1105b-tools/e2eready` (static)
Already fixed by other lanes before I looked (verified in the working tree): e2eready `expect.json` koordinator `rowText` is per language (F1), probe accepts `Play / Pause` via `pausedTokens` (F4), energy status regex is `\b` in `🧬️simulation/🧫️cases.json`, catalog `demonstrator.landing.title` en is "Designing with Existing Stock · Demonstrator" (F3), `🌐️.html` title and `locale/⚛️.ts:23` `document.title` follow it, footer is now an `<a>` (no test selects it as a button), picker heading is two block spans and the picker is a `role=group` (tests only use `[data-slot=demonstrator-locale-picker]`, `button[lang]`, `button[data-locale]`: still valid), `h1[data-slot=app-name]` kept.

Stale test side, fixed by me (`D` = `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance`):
- `D/🟦️.ts` koordinator undo-redo asserted `/Box \d+/` in both languages; de row reads `Quader N` (cad terminology `typology_box`). Now `scenario.rowText[language]`; schema `D/🧬️edits/🔣️.json` gained required `rowText {en,de}`, data in `D/🧬️edits/🧫️cases.json`.
- `D/🟦️.ts` statik undo-redo asserted the transport label exactly `Play`/`Abspielen`; the non-live results window renders `Play / Pause` (fem results panel). Now `pausedTransport()` accepts the optional ` / <word>` suffix (same rule as the probe).
Verified correct, left alone: baseline `🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json` still uses `^Finalized ·` / `^Aborted, nothing was changed ·`. Producer `🔌️plugin/⏯️tool-run/🪧️panel-groups/🦀️.rs:64` formats `"{state} · {stage} (n/m)"`, so ` ·` is real; the `\b` in the simulation cases JSON is looser but also true. F2 (acting only in en) is fixed in the probe (`runPane(..., true)`), no env switch needed. Port: probe takes the URL as argument, default run in the brief is 6029, nothing hard-coded stale found.

Product-side defects / notes (not edited):
- P1 Koordinator copy "connects ... in shared views" vs behaviour (see 1107-result-e2ecases.md): copy loose, no propagation between the four models.
- P2 The fem results transport button is the non-live toggle `Play / Pause` whenever the window is not live; a screen-reader user hears a toggle without state. Product decision whether that is intended.
- P3 `ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND.windowTitle` (and all pane brands) is now `DEMONSTRATOR_NEUTRAL_TITLE` = "Designing with Existing Stock / Entwerfen mit Bestand" (language-neutral, joined with " / ") while `<title>` in `🌐️.html` joins with " · ". Two separators for one purpose; harmless for tests, a copy inconsistency.
- Unverified runtime items remain as in 1107-audit-e2e.md (class values, `Quader N` index format, `action.addFeature` id, live vs non-live transport).

## 2. New `@behaviour @cycle` cases (written, not run)
Files: `D/🧭️behaviours/🎭️cycle.ts` (8 cases), `D/🧭️behaviours/🧫️cases.json` (added texts `execute`, `renameZone` and `cycle.energie`), `D/🧭️behaviours/🎚️config/🟦️.ts` (testMatch now lists both specs). Tools: `C/1107-tools/behaviours/{tsconfig.json,list.config.mjs}`. Logs: `C/🗑️generated/1107/behaviours-{tsc,list}.txt`.
Every case: boot to `ready` with the explicit `lang`, authored fixture asserted, one act, undo, redo, all via a single `actUndoRedo(fingerprint, act, history)` law (changed, back to before, back to after). Accessible locators where the producers expose roles: `spinbutton`, `button` by localized name (Enabled/Aktiviert, Remove/Entfernen, Execute/Ausführen), `tree`; ids only where the baseline already proved no role exists (tree labels, inspector inputs, engagement toggle).
| Pane | Fixture | Act | Undo/redo observable |
|---|---|---|---|
| generator | 7 widget ids, height 6, preview settled | inspector height 6 -> 6.5 | graph snapshot + preview geometry |
| koordinator | 4 cad windows settled | catalogue Box/Quader row | shape window projection; other 3 unchanged |
| aggregator | seed-left-001 in both puzzle3d windows | catalogue kind click | instance ids of both windows |
| energie | model settled, zone row listed | `rename-zone` window action on the zones table | zone row texts (NEW: the baseline energie case is run/cancel only, which writes no document) |
| aussuchen | Glulam row, max 24, curated empty | spinbutton ArrowUp | count + curated rows + grid ids |
| bearbeiten | 7 step rows, workpiece settled | row `Enabled` button | workpiece projection |
| verfolgen | map canvas, integer positions | addFeature action | positions count, routes/regions unchanged |
| statik | model + results settled, `Case: dead` | deformation scale 300 -> 600 | scale + results projection |
Titles are at most 24 chars. Selection `--grep @cycle` = 16 executions; `@behaviour` = 40 (24 + 16); baseline `--grep-invert @behaviour` still 50.

Proof: `tsc -p C/1107-tools/behaviours/tsconfig.json` rc 0 over all behaviour files plus the edited baseline spec; `playwright test --list` (repo playwright 1.64.1) as above; `e2eready/run.sh --check` GREEN 8/8. Not proven: any selector or timing against a served page.

Risks on first run (all UNVERIFIED): energie `rename-zone` action id/row `action.rename-zone` and `arg.zone`/`arg.newName` inputs, execute button found by role name inside the shell (`.first()`), undo from the model window canvas; koordinator document row found by tree-label text `Box N`/`Quader N`; others as listed in 1107-result-e2ecases.md.

## Question for the coordinator
A recent lane introduced the English brand name "Designing with Existing Stock" (abbreviation "DwES", neutral title "Designing with Existing Stock / Entwerfen mit Bestand"). I did not touch it; tests read it from the catalog so they follow either way. Is that the intended final English name, and should the funding/partner copy and the repo-level name (`mit-bestand`) follow?
