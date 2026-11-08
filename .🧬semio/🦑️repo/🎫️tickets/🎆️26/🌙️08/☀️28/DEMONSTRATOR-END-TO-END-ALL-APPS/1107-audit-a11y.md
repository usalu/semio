# 1107 Accessibility and Language Audit (lane aud-a11y)

Read-only audit. No repo runs, no sub-agents, no edits outside this ticket folder.

Path bases used below:
- DEMO = `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/`
- PLG = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/`
- FW = `/Users/ueli/Documents/semio/🧰️framework/`

Severity: `blocks e2e` (would fail the e2e or the run.sh a11y gate), `user-visible` (wrong or missing text, focus or name a real user hits), `polish`.

## Correction to the brief

The brief says the en landing title at `DEMO🔨️modules/🌐️locale/🔣️.json:21` is German. It is not. Line 21 is:
- en: `"Designing with Existing Stock · Demonstrator"` (English, correct)
- de: `"Entwerfen mit Bestand · Demonstrator"` (German, correct)

The German brand leaks into en elsewhere (findings 2, 7, 8, 10, 11 below). Those are the real wrong-language cells.

## Findings

### A. Locale catalogue (`DEMO🔨️modules/🌐️locale/🔣️.json`)

1. **user-visible** `:135`, `:139` (en cells of `demonstrator.brand.pane.generator.windowTitle` and `...aggregator.windowTitle`). The en value equals the de value, `"Entwerfen mit Bestand · Generator"` / `"· Aggregator"`. Wrong language in en.
   Fix: add key `demonstrator.brand.name` with en `"Designing with Existing Stock"` and de `"Entwerfen mit Bestand"`. Rewrite every `windowTitle` en cell (`:135`, `:137`, `:139`, `:141`, `:143`, `:145`, `:147`, `:149`) as `"<brand.name en> · <Pane en>"`.

2. **user-visible** `:136`, `:138`, `:140`, `:142`, `:144`, `:146`, `:148`, `:150` (en `shortWindowTitle`). The en cell is `"EmB · Gen"`, `"EmB · Coord"`, etc. `EmB` is a German acronym. The short label is also taken from the last segment (`🟦️.tsx:101`), so `Gen`, `Coord`, `Sel`, `Edit`, `Trace`, `Statics` appear in en while `Verf`, `Bearb`, `Aus` appear in de.
   Fix: en short titles as `"DwES · Gen"` style, with the abbreviation derived from the new brand key, and abbreviations per pane that are real words in each language (no truncation like `Verf`, `Bearb`).

3. **user-visible (touch only)** `:14`, `:16`, `:18`, `:20` (beginner `neighbour*` labels). They read "Swipe down to reach {{label}} above". These strings are the accessible names of the neighbour hint buttons (`LayeredOverview/💡️hints/🟦️.tsx:62`). A screen reader user hears a gesture instead of an action.
   Fix: beginner wording without the gesture for the name, e.g. en `"Show {{label}} above"`, de `"{{label}} oben anzeigen"`. Keep the swipe wording only in the visible hint text.

4. **polish** `:6` vs `:5`, `:10` vs `:9`, `:12` vs `:11`. Beginner and normal cells are identical. Either keep identical on purpose or write real beginner wording (`:6` "Every demonstrator" is the same as normal).

5. Checked, no finding: `:37` "Zukunft Bau" (proper noun, identical in both); `:12`, `:24`, `:38` and all `footer`, `card`, `locale.choose` keys have both cells, non-empty, correct language. Scan of all 146 keys: no missing cell, no empty cell.

### B. Brand constants and page HTML

6. **user-visible** `DEMO🪧️brand.ts:52` `export const ENTWERFEN_MIT_BESTAND_NAME = "Entwerfen mit Bestand";`. It is the German brand used as visible text on the landing in en: `DEMO🟦️.tsx:179` (Navbar `label`) and `DEMO🟦️.tsx:188` (`<h1 data-slot="app-name">`). An en visitor sees the German name in the page heading.
   Fix: delete the constant. Resolve the name with `demonstratorText("demonstrator.brand.name", locale)` at `🟦️.tsx:179` and `:188` (needs the new key from finding 1).

7. **user-visible** `DEMO🪧️brand.ts:55` `ENTWERFEN_MIT_BESTAND_LOGO_SVG` contains `role="img" aria-label="Entwerfen mit Bestand"`. The accessible name of the logo is German in both locales.
   Fix: give `ShellBrandLogo` a locale-resolved label, or mark the svg `aria-hidden` (the h1 names the app; the logo is decorative).

8. **user-visible** `DEMO🪧️brand.ts:356, 514, 563, 616, 659, 708, 774, 823` (`windowTitle: ENTWERFEN_MIT_BESTAND_NAME`) and `:979` (`DEMONSTRATOR_SHELL_BRANDS`). The base brands keep the German title. It is overwritten per locale only in `demonstratorPaneViews` (`:893-897`). The owner catalog (`DEMONSTRATOR_SHELL_BRANDS`, used by `🐚️shell/🟦️.ts:11` and `🏗️builder/🌐️vite/🐚️shell/🟦️.ts:8`) still carries the German title.
   Fix: make `windowTitle` on the shell brand locale-free (a neutral key, or remove it from the owner catalog and resolve per locale in `resolveShellDefaults`).

9. **user-visible** `DEMO🌐️.html:6` `<title>Entwerfen mit Bestand</title>`. This is the document title before any language is chosen, so the picker page is German with no choice made. `DEMO🌐️.html:2` `<html>` has no `lang` attribute. `initUiLocaleSync` (`FW🔨️modules/🖱️ui/🌐️i18n/🟦️.ts:221`) sets `document.documentElement.lang` only after a choice, so the picker state has no page language (WCAG 3.1.1).
   Fix: use a neutral bilingual title (`"Designing with Existing Stock / Entwerfen mit Bestand"`) and `lang="mul"` on `<html>` until a choice is made. Do not pick en or de as the default.

10. **user-visible** `DEMO🐚️shell/🟦️.ts:15`. `document.title = ... || ENTWERFEN_MIT_BESTAND_NAME`. This is a silent fallback to the German name when the locale is unknown (`readDemonstratorLocale()` returns undefined) or the view is not found. It is the default-language fallback the brief forbids.
   Fix: when no locale is resolved, use the neutral bilingual title from finding 9. Throw or log `[DEBUG]` on a missing view instead of falling back.

### C. Landing page TSX (`DEMO🟦️.tsx`, `DEMO⚛️locale-picker.tsx`, `DEMO⚛️footer.tsx`, `DEMO⚛️demonstrator-card.tsx`)

11. **user-visible (keyboard)** `DEMO🟦️.tsx:116-121` with `DEMO⚛️locale-picker.tsx:23-27`. Choosing a language in the picker unmounts the focused button and the landing replaces it. Focus falls to `<body>` and the next Tab starts from the top (WCAG 2.4.3).
   Fix: after `chooseLocale`, move focus to the landing heading. Add `tabIndex={-1}` to `h1[data-slot="app-name"]` (`🟦️.tsx:187`) and focus it in an effect when `locale` changes from undefined to a value.

12. **polish** `DEMO⚛️locale-picker.tsx:14-21`. The `<h1>` joins two languages with a `" / "` text separator; screen readers read "slash". Also the picker buttons (`:23-27`) have no group name.
    Fix: one `<p lang>` per language, no literal separator, and wrap the buttons in `role="group"` with `aria-labelledby` pointing at the heading.

13. **polish** `DEMO⚛️footer.tsx:52-63`. The funded-by credit is a `<button>` that calls `window.open`. It has no `href`, so middle-click and Ctrl-click do not open the partner page, and it is announced as a button, not as a link to a new tab.
    Fix: render an `<a href target="_blank" rel="noopener noreferrer">` with the same styling and the existing `aria-label` (`:54`).

14. Checked, no finding: `DEMO⚛️demonstrator-card.tsx:22, 30` (accessible name "Open demonstrator {{label}}", button element, keyboard works). `DEMO⚛️footer.tsx:27, 82` (partner links have localized `aria-label`, images `alt=""`, group labelled). `DEMO⚛️locale-picker.tsx:36, 45` (switcher is a labelled group with `aria-pressed`). `DEMO🟦️.tsx` contains no literal UI text other than the brand name (findings 6, 7).

### D. Shared pane chrome (`FW🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/`), affects all 8 panes

15. **verify (not run)** Focus handling across switches is implemented in `🟠️pane-styles/🟦️.tsx:40-48` (region focus on open, card focus restored on close) and Escape handling at `:99`. The overview button has `aria-keyshortcuts="Escape"` (`🟢️overview-button/🟦️.tsx:14`). The component tests in `🧪️tests/🧩️component/🟦️.tsx:520-535` cover it. Not verified in a browser: add an e2e step "open pane, Escape, focus lands on its card" in en and de.

### E. Pane plugins (PLG)

16. **user-visible (en and de both English)** CAD REPL, hard-coded English, no locale cells. PLG📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/
    - `📋️repl-panes/🟦️.tsx`: `:77-78` "Clear" (button), `:87` "Attributes", `:144` "Enabled", `:220` "Stats", `:226` "Model", `:290` "Properties".
    - `🎛️repl/🟦️.tsx`: `:146` "Editor", `:236` "Model definition", `:272` "Transfer from", `:297` "Transfer to", `:331` `aria-label="Show primitives"`, `:365` `aria-label="Filter primitives"`, `:388` `aria-label="Show typologies"`, `:425` `aria-label="Selection typologies"`, `:460` "Last response".
    Fix: one table of `{en, de}` cells per label (same pattern as `WINDOW_SEARCH_USER`) and resolve from the pane locale. The `aria-label`s use the same table. "Clear" needs a field-specific name (for example "Clear Model definition").

17. **polish** PLG🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧱️editor-vocabulary/🦀️.rs:20: `label: "Catalog".into()` on a `MediaPortSpec`. Plain English, no German.
    Fix: `LocalizedLabel::native("Catalog", "Katalog")` if the field type allows, else change the field type to a localized label.

18. **polish** PLG🌀️procedural / other plugins, `DEMO`-equivalent: the fem manifest `PLG🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📋️manifest/🦀️.rs:30` passes `["Entwerfen mit Bestand", "Statik"]` as terminology data. German brand text in data for all locales.
    Fix: take the brand name from the same `brand.name` key as finding 1.

19. **polish** PLG📐️cad/…/📚️examples/…/🕹️interactions/🗺️constructSurface.json: transition labels `"Sweep 1"` / `"Sweep 2"` have identical en and de cells (`.machine.states[0].on[4|5].transitions[0].label`). Example data only; a German cell with the same text is acceptable, flagged for consistency.

20. Checked, no finding: `PLG🌍️gis/.../💡️inference/🪟️presentation/🟦️.tsx:38-59` (labelled section, live region, progress, buttons all from `control.*[locale]`). `PLG🌀️procedural/.../🧊️generation3d/🦀️.rs:172-174` (`LocalizedLabel::native` with en and de). `PLG🧩️puzzle/.../🗣️terminology/🦀️.rs:98-105` (native_en and native_de).

21. **Locale JSON sweep of the eight pane plugins** (script in scratchpad, all `{en, de}` cells in every JSON file, excluding tests and fixtures): about 9,500 cells across procedural, cad, puzzle, energy, sourcing, process, gis, fem. Result: 0 empty cells, 0 cells with only one language, 0 en cells that look German, 0 de cells that look English. The only en==de cells are cognates (Zoom, Radius, Minimum, Interpolation, Schema) and finding 19.

## Not covered (limits)

- No browser run. Focus, roles and keyboard findings are from reading the code. Finding 15 needs a browser check.
- The eight pane plugins have almost no TSX. Their UI comes from the framework shell (`FW🛍️products/💻️os/...`), which was not audited beyond `ShellHost` locale sync (`ShellHost/🟦️.tsx:174-183`, correct: requested locale is applied to mounted shells).
- Rust production labels in the process (`PLG🏭️process`), energy, fem and procedural plugins were sampled by grep only. Many hits were test fixtures (`🧪️tests`). `PLG🏭️process/.../🧬️mutations/🦀️.rs:722, 734` ("Deep Capability", "Deep Step") were not confirmed as test-only.
- `ShellLocalePicker` in `FW…ShellHost` (picker for pane shells without locale) was not reviewed.

## Summary by severity

- blocks e2e: none established by reading. Findings 6 and 9 would fail any en heading or title assertion.
- user-visible: 1, 2, 3, 6, 7, 8, 9, 10, 11, 16
- polish: 4, 12, 13, 17, 18, 19
- verify: 15
