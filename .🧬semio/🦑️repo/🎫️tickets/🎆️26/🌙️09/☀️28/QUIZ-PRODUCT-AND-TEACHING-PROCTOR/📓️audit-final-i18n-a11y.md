# Quiz website — final audit of languages and accessibility

Read-only audit. Nothing was built, run or edited; every statement below comes from reading the source, from mechanical key/number comparisons done with one-off read-only scripts, and from contrast arithmetic on the design-system tokens. Items that need a renderer or a screen reader to confirm are marked "not measured" and listed again at the end.

Yardstick: WCAG 2.2 AA. Repo rules: no default language, English first and German second, German informal "du".

Path legend (all paths are relative to `C:\git\semio`):

- `R/` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`
- `M/` = `R/🔨️modules` (one folder per module, file `🟦️.ts` or `🟦️.tsx`; written as `M/<module>:<line>`)
- `S/` = `🎓️teaching/🏛️architecture/❓️quiz` (site shell)
- `C/` = `🎓️teaching/🏛️architecture/⚡️energy` (quiz content, `C/<topic>/❓️quiz/🔣️.json`)
- `DS/` = `🧰️framework/🔨️modules/🖱️ui` (design system, only cited where a finding lives there)

## Summary

| Class | Count |
|---|---|
| DEFECT | 7 |
| IMPROVE | 25 |
| NOTE | 14 |

DEFECTs: D1 document title/lang, D2 matching select steals cards, D3 "Keep this order" loses focus, D4 connection status chatters, D5 peer label contrast, D6 forced-colors state loss, D7 wrong table header in matching.

---

## A. Languages

### A1. Mechanical comparison of the i18n module (`M/🌐️i18n:50-319` EN, `:322-591` DE)

Method: parsed both bundles by nesting, compared key paths, placeholders, and grepped every `"quiz.x.y"` literal in the 28 source files of the React target.

- Keys: EN 224, DE 224, same set, same order. Missing in DE: none. Missing in EN: none.
- `{{placeholder}}` sets per key: identical in both languages for all 224 keys.
- Used in a component but not defined: none (also none through `REJECTION_LABELS` and `TASK_KIND_LABELS`, `M/🌐️i18n:599-613`; the only dynamic keys are `quiz.${command.type}` / `quiz.${query.type}` in the proctor envelope and are not UI strings).
- Defined but never used: none. All 13 keys not found as literals in components (`quiz.task.*` and `quiz.rejection.*` except `refused`) are reached through the two maps above. `quiz.rejection.handleInvalid` and `quiz.identity.handleInvalid` carry the same text, which is harmless duplication.
- Identical EN/DE strings (legitimate cognates): `textNormal`, `english`, `german`, `identity.pseudonym`, `identity.name`, `learner.quiz`, `learner.status`, `radar.minimum`, `radar.maximum`, `results.dimension`, `crowd.choice`, `presence.online`, `presence.onlineMark`.
- Content (`C/*/❓️quiz/🔣️.json`, `S/🔣️.json`): 288 `{en, de}` texts, all non-empty (the schema requires both, `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json` `Text`, `required: ["en","de"]`), no formal address ("Sie/Ihr/Ihnen") anywhere in the 288 texts or the 224 UI strings.

### A2. Hard-coded strings and locale-blind formatting

Every visible string of every component goes through `text()`/`localized()`. What remains outside:

- Raw English technical detail inside a localized sentence: `quiz.rejection.refused` = "…refused the request ({{detail}})" is filled with `error.message` / `HTTP 500` / `revisionConflict: …` — see I14.
- Decorative glyphs with `aria-hidden` only (`⠿ ▲ ▼ ⋯ ⚠ ↻ ✓ 👥`) — fine.
- Placeholders `"–"` without text for assistive technology — see N9.
- Numbers: every quantity, score, point total, date and clock time uses `Intl` with the active locale (`M/📏️quantity:45-132`), values and units are joined with a no-break space (`withUnit`, `:85-88`), names sort with `Intl.Collator(locale)` (`M/🏆️leaderboard:54`). No component formats a number or date with `toFixed`, `toLocaleString()` without locale, or template strings. Plain integers (ranks, counts, task numbers) are not grouped; harmless below 1 000.

### A3. Language assumptions

- **D1** (below): `<html lang="en">`, German-only `<title>` and loading text; `document.title` is never set.
- `M/🌐️i18n:26-33` `preferredLocale` ends in `return "en";`. This is the one remaining fallback; it is documented in the module header ("English winning when neither English nor German is listed") and softened by the always-visible language switch (`🟦️.tsx:382`) and the first-visit preferences card (`🟦️.tsx:250`). See N1.
- `{ en: id, de: id }` fallbacks for a missing item/category (`sorting:35`, `classification:28`, `matching:36`, `results:27`, `quiz-page:105`) are language-neutral (the id). Fine.
- `DS/🎯️targets/⚛️react/🌐️i18n/🟦️.ts:2154,2229` `fallbackLng: "en"` and `normalizeUiLocale` (`:2176-2178`) default the design system's own chrome to English; the quiz bypasses it with `lng: locale` (`M/🌐️i18n:622`), so no quiz string can fall back. The shared port's language is applied in an effect (`🟦️.tsx:347-349`), after the first paint.
- Catalog texts with a single language: none (schema).

### A4. German quality (UI and content)

Quoted string → correction. Line numbers are in `M/🌐️i18n` unless a JSON path is given.

UI chrome (DE):

| Line | Current | Correction | Why |
|---|---|---|---|
| 357 | `textLargest: "Am größten"` | `"Maximal"` (or `"Sehr groß"`) | Awkward as the last of Normal / Groß / Größer. |
| 362 | `answers: "Zeigen, was die anderen denken"` | `"Anzeigen, was die anderen denken"` | An imperative "Zeigen" is no checkbox label; line 360 already says "anzeigen". |
| 397 vs 423 vs 421 | `home.open: "Begonnen"`, `learner.statusOpen: "Läuft"`, `learner.started: "Begonnen"` | `home.open` and `statusOpen`: `"In Bearbeitung"`; `learner.started`: `"Gestartet"` | Same EN text "In progress" has two German words, and "Begonnen" is also the "Started" column. |
| 433 | `"Jeder Durchgang zieht und mischt die Elemente neu."` | `"Jeder Durchgang wählt die Elemente neu aus und mischt sie."` | "zieht" is a calque of "draws". Same sentence in `S/🔣️.json` introduction paragraph 2. |
| 466 | `binEmpty: "Noch keine Elemente – Elemente hier ablegen."` | `"Noch keine Elemente."` | Repeated word; "ablegen" is also pointer-only wording for a bin that keyboard users fill with the select. |
| 470, 495 | `"{{item}} zugeordnet zu {{category}}"`, `"{{value}} zugeordnet zu {{item}}"` | `"{{item}} wurde {{category}} zugeordnet"`, `"{{value}} wurde {{item}} zugeordnet"` | "zugeordnet zu" is an English calque ("assigned to"). |
| 513 | `credit: "Anrechnung"` | `"Punkte"` | "Anrechnung" is not a score word. |
| 530 | `"…nach Gesamtpunkten gereiht."` | `"…nach Gesamtpunkten geordnet."` | "gereiht" is Austrian usage. |
| 532 vs 530, 542 | `learner: "Person"` while caption says "Lernenden" | `"Lernende"` | One word for the one concept. |
| 543 | `full: "Ganze Rangliste"` | `"Vollständige Rangliste"` | Idiom. |
| 548 | `thinking: "Denken gerade mit: {{count}}"` | `"Arbeiten gerade mit: {{count}}"` | "mitdenken" cannot be counted like this; EN twin is also a calque (I19). |
| 460 | `voided: "…daher wurde dein Durchgang verworfen. Bitte starte neu."` | `"…daher wurde dein Durchgang verworfen. Bitte starte einen neuen Durchgang."` | "starte neu" lacks an object; "verworfen" vs status "Ungültig" (423) — pick one word. |
| 361 | `summary: "Sprache, Farbschema, Textgröße und die Cursor der anderen."` | add "und was die anderen denken" | Stale since the fifth setting `answers` exists (EN line 89 has the same gap). See I15. |

Catalog (`S/🔣️.json`):

| Line | Current | Correction |
|---|---|---|
| 5 | title DE `"Quizze Architektur und Technologie"` | `"Quizze zu Architektur und Technologie"` (it is the site title in the navbar on every screen) |
| 15 | `"Jeder Durchgang zieht und mischt die Elemente neu, daher gleicht kein Durchgang dem anderen."` | `"Jeder Durchgang wählt die Elemente neu aus und mischt sie, daher gleicht kein Durchgang dem anderen."` |
| 19 | `"…gemessen auf einer logarithmischen Skala, wo die Werte Größenordnungen umfassen…"` | `"…gemessen auf einer logarithmischen Skala, wenn die Werte mehrere Größenordnungen umfassen…"`; `"Bei Klassifikationen mit Netzdiagrammen"` → `"Bei Klassifizierungen mit Netzdiagrammen"` (UI says "Klassifizierung", `M/🌐️i18n:436`) |
| 27 | `"…und dafür, jedes Quiz abgeschlossen zu haben. Die Bestenliste ordnet alle nach der Summe ihrer besten Quiz-Ergebnisse."` | `"…und dafür, alle Quizze abgeschlossen zu haben. Die Rangliste ordnet alle nach der Summe ihrer besten Quiz-Wertungen."` ("Bestenliste" vs UI "Rangliste"; "Ergebnisse" vs UI "Wertungen") |
| 77 | badge `pattern-seer` "Jede Klassifikationsaufgabe des Katalogs ganz ohne Fehler gelöst." | `"Klassifizierungsaufgabe"` |

Quizzes (DE):

- `C/🔥️heating/❓️quiz/🔣️.json:9` `"…von der Einfachverglasung bis zum Passivhausdach sowie Heizlasten und Heizwärmebedarfe von Gebäuden von den 1960ern bis zum Passivhaus."` → `"…Gebäuden aus den 1960er-Jahren bis zum Passivhaus."` (three "von", and "den 1960ern" is colloquial).
- "…der 1960er" / "…der 1970er" without "-Jahre" at nine places: `C/📊️demand:85,165,201,228`, `C/🧲️physics:75,138,237,403`, `C/❄️cooling:249`. Correct form: "der 1960er-Jahre" ("Einfamilienhaus der 1960er-Jahre", "Bürohochhaus der 1970er-Jahre").
- `C/🧲️physics/❓️quiz/🔣️.json:18` `"(eine Rate: Energie pro Zeit, gemessen in Watt)"`, `:24` `"Eine Rate: wie schnell Energie umgesetzt wird (1 W = 1 J/s)."`, `:102` `"die Rate, mit der der Autoakku gefüllt wird"` → "Rate" is the English word; German: `"(Energie pro Zeit, gemessen in Watt)"`, `"Energie pro Zeit: wie schnell Energie umgesetzt wird (1 W = 1 J/s)."`, `"die Geschwindigkeit, mit der der Autoakku geladen wird"`.
- `C/🧲️physics:120` `"dauert das Füllen 60 ÷ 11 ≈ 5,5 Stunden"` → `"dauert das Laden …"`.
- `C/🧲️physics:389` `"ein Motor macht nur etwa ein Viertel davon zu Bewegung"` → `"ein Motor setzt nur etwa ein Viertel davon in Bewegung um"`.
- Terminology that is consistent and correct (verified, see list at the end): Heizlast / Heizwärmebedarf / Kühllast / Kühlbedarf / Endenergiebedarf / Primärenergiebedarf, U-Wert, Luftwechselrate, GEG / EnEV / WSchVO / KfW-Effizienzhaus / Passivhaus (Institut). "Heizenergiebedarf" and "Heizbedarf" do not occur.

English quality (where clearly off):

- `M/🌐️i18n:276` `"Thinking along: {{count}}"` — calque of "mitdenken". → `"Also working on it: {{count}}"`.
- `M/🌐️i18n:283` `"⌀ place {{position}}"` — "⌀" is the German "Durchschnitt" sign, in English it is "diameter". → `"avg. place {{position}}"` (visual only; the sr-only sentence is fine).
- `S/🔣️.json:18` `"every pair in the wrong order costs the more, the further apart the true values are"` — German "umso mehr, je weiter". → `"every pair in the wrong order costs more the further apart the true values are"`.
- `S/🔣️.json:22` `"so choose something distinctive if your results should stay yours"` → `"if you want your results to stay yours"`.
- `C/❄️cooling/❓️quiz/🔣️.json:50,68,77,122` `"DIN 1946-6 asks …"`, `"EN 16798-1 asks …"`, `"ask 6 m³/h"`, `"DIN 1946-4 asks …"` — "verlangt" is "requires". 
- `M/🌐️i18n:161` `"draws and shuffles the items anew"` (and `S/🔣️.json:14`) — fine but stiff: `"picks and shuffles the items afresh"`.
- `100 %` with a space appears 14 times in EN content, while the EN UI formats scores as "99.7%" (`Intl`, `formatScore`). EN: `100%`; DE keeps `100 %`.

### A5. Content texts

- Every item of the four quizzes has `explanation` in both languages (checked: 0 missing among 4 quizzes, all tasks); the results tables show it in the last column (`M/🏁️results:94-96,122-124,167-169`).
- EN/DE number sets per text are equal (only difference: "windows before 1979" / "Fenster bis 1978", which is the same statement).
- Decimal comma in every DE text, decimal point in every EN text (the only DE "point" numbers are the date `04.12.2020`). Units are spelled the same in both languages and across all files: `kWh/(m²·a)` 104×, `W/m²` 56×, `W/(m²·K)` 47×, `m³/h`, `m³/(h·m²)`, `MJ/kg`, `1/h`.
- Math spot-checks (all consistent): 33 000 kWh = 110 m² × 303; 60 GW = 527 TWh / 8 784 h; 164 400 TWh = 592 EJ; the final-energy sums of the "demand" matching task; full-load hours × W/m² = kWh/(m²·a) in the cooling task.
- Typography issues → I20.

---

## B. Accessibility

### B1. Traces by keyboard and screen reader

**Sorting** (`M/↕️sorting`). Tab order: "Keep this order" (while no answer) → per item "Move X up", "Move X down" (`:75-80`, buttons with `aria-label`, `aria-disabled` at the ends so they stay focusable). Enter/Space moves, focus is restored to the same button after the DOM moved (`useFocusAfterRender`, `M/🧩️task:44-56`), a polite atomic live region says "X is now at position 3 of 14". Reordering without drag and drop: ok. Findings: D3, I23, N7.

**Classification** (`M/🗂️classification`). One `<select aria-label="Category for X">` per item (`:53-66`), focus is restored to the select after the item re-mounts in another list (`:57`), polite announcement. Findings: I1.

**Matching** (`M/🃏️matching`). One select per item and quantity listing all cards with "– used by X", ✕ button to unassign, card pool as list, polite announcement. Findings: D2, D7, I8.

**Submit dialog** (`M/▶️run:216-258`). `role="alertdialog"` while confirming, `role="dialog"` while working, `aria-modal`, `aria-labelledby`/`aria-describedby`, initial focus on `[data-autofocus]` (`:68-70`), Tab trap (`:108-126`), Escape closes or cancels, focus returns to "Submit quiz" (`:88-91`). Findings: I3, I21.

**Layered home** (`M/🏠️home`, `DS/🧱️elements/🥞️LayeredOverview/🟦️.tsx`). Cards are reached in DOM order (learner, quiz, how it works, …). Keyboard focus on any control of a card reveals its page (`:441` `onFocus` with `:focus-visible`), blur conceals it (`:442`); mouse hover reveals only for `pointerType === "mouse"` (`:439-440`). The heading link `href="#<page>"` or the "Read more/Open" button opens the page; the page becomes a labelled `role="region"` that receives focus (`:395-398,727`), Escape closes it (`:751-755`), focus returns to the card's first focusable (`:735`). Backdrop pages are `aria-hidden` and `inert` (`:397,399`). Findings: I4, I5, I10, N8.

**Leaderboard** (`M/🏆️leaderboard:239-283`). Native buttons inside `<th scope="col" aria-sort>`, chevron icons show direction by shape, the scroll container is a focusable named region (`tabIndex={0}`, `role="region"`), no live region on polling. Ok. Findings: N9 (placeholder "–").

**Preferences** (`M/🎛️preferences`). Segmented buttons with `aria-pressed` in named groups, two native checkboxes in labels, language buttons carry `lang`. Ok. Findings: D6 (pressed state in forced colors), I15.

### B2. Names, roles, states, live regions, inertness

Findings D4, D6, D7, I3, I6, I7, I8, I11, I12, I21. What was checked and is fine is in the verified list.

### B3. Colour, contrast, forced colors, text size, motion

Token values (light / dark): base `#f7f3e3` / `#001117`; foreground 17.3:1 / similar; `--muted-foreground` `#3e494a` 8.4:1 / `#91968f` 6.4:1; `--border-normal-color` `#7b827d` 3.5:1 / 4.9:1; `--active-base` `#ff344f` 3.2:1 / 5.4:1; dark text on active-base 5.4:1; presence slots against the page ≥ 3.3:1 in both themes.

Findings: D5, D6, I9. Text size: `:root { font-size: calc(100% * var(--quiz-text-scale, 1)) }` (`R/🎨️.css:7-9`, set in `🟦️.tsx:288-296`) scales every rem-based size, the spider diagram measures its own em (`M/🕸️radar:389-423`), so labels follow. The media queries that choose the home layout are px-based and do not follow the text size (I10).

### B4. Touch and mobile

- Viewport meta `width=device-width, initial-scale=1.0`, no zoom lock (`S/🌐️.html:5`, built `dist/index.html:5`). Ok.
- Target sizes: every control carries `.quiz-target` (`R/🎨️.css:40-42`, min 24 px) or the chrome's `min-h-medium`; ICON buttons are at least 1.75 em wide. Ok. The radio/checkbox boxes are 1.25 em but sit inside 24 px labels (equivalent target). Ok.
- Dragging (WCAG 2.5.7): drag exists only as an addition on the grip (`M/🧩️task:64-76`, `touch-none`), every action has the select/button path. Ok.
- Hover-only: "hover shows the page clear" is hover (mouse only) or keyboard focus; the touch equivalent is a tap on the card, which opens the page (`M/🪟️chrome:69-77`). So no function is hover-only; touch users cannot "peek" without opening (N8).
- Reduced motion: `useReducedMotion` snaps glide and zoom (`LayeredOverview:169-180,578,594`), card lift is `motion-reduce:transition-none` (`M/🪟️chrome:60`), cursor glide is cut by `R/🎨️.css:272-278`. Ok.
- Findings: I9, I10.

### B5. Charts (spider diagrams)

`M/🕸️radar:456-517`: one `<svg role="img" aria-labelledby>` with `<title>`, a `<details>` with a real table (caption, `th scope`, value/min/max with units) as the text alternative, labels measured so none is clipped, legend with numbers when labels do not fit. One polygon, no colour-only meaning. Finding I12.

### B6. Language of parts

`<div class="quiz-app" lang={locale}>` (`🟦️.tsx:386`), `document.documentElement.lang` set by `setUiLocale` (`DS/🎯️targets/⚛️react/🌐️i18n/🟦️.ts:2260-2263`) in an effect, language buttons carry their own `lang` (`M/🎛️preferences:75`, `M/🪟️chrome:150`). German proper names inside English content ("Gründerzeit", "Stromspiegel für Deutschland", `Wikipedia “Teelicht”`) are names; no markup needed. Finding D1 (before JS and the title).

---

## Findings

### D — DEFECT (7)

**D1. Document language and title are German-only, `lang` is fixed to English; the app never sets `document.title`.**
- Where: `S/🌐️.html:2` `<html lang="en">`, `:6` `<title>Quizze · Architektur und Technologie</title>`; `S/🏗️builder/🌐️vite/🟦️.ts:33,35` `title: "Quizze · Architektur und Technologie"`, `loading: { title: "Quizze · Architektur und Technologie" }`; the shared host template `DS/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:737` `<html lang="en">`, `:741` `<title>${spec.title}</title>`, `:723` the loading `<div>`; the built `S/📦️packages/🟦️typescript/dist/index.html:2,6,10` and `404.html` are the same. `document.title` is written nowhere in `R/` (grep). `applyLocale` runs in `useEffect` (`🟦️.tsx:347-349`).
- Effect: an English reader sees a German tab title forever; the German loading text is spoken with an English voice; the document language is wrong until the first effect has run; there is no screen-specific title.
- Fix: (1) in `QuizApp` set `document.title = "<h1 of the screen> · <localized catalog title>"` in an effect keyed on `locale` and `stepKey`, with a pair fallback `"Quizze · Quizzes"` until the catalog arrived; (2) make the loading text bilingual with parts marked (`<span lang="de">Quizze</span> · <span lang="en">Quizzes</span>`) and set `<html lang>` from an inline script before first paint (stored quiz preference in the tenant store, else `navigator.languages`), leaving no fixed `lang="en"`; do it in the host template so every site benefits; (3) add a bilingual `<noscript>` (today `html:not([data-semio-styled]) body{visibility:hidden}` keeps the page blank without JS, N3).

**D2. Matching: browsing the select with the arrow keys takes cards away from other items.**
- Where: `M/🃏️matching:25-28` `const next = Object.fromEntries(Object.entries(assignments).filter(([holder, index]) => holder !== item && index !== card));` and `:112` `onChange={(event) => set(dimension, item, event.target.value === "" ? undefined : Number(event.target.value))}` (lines 108-123).
- Effect: on Windows/Linux (Chrome, Edge, Firefox; NVDA/JAWS in forms mode) ArrowDown on a closed select changes the value and fires `change` immediately. Every intermediate option that is "– used by Y" is committed, which deletes Y's assignment permanently (the previous holder is not restored when the user moves on). A keyboard or screen-reader user who only wants to hear the options destroys other answers. Not measured on a device; follows from the handler and the platform behaviour.
- Fix: do not commit a take-over from a select change. Either render options held by another item as `disabled` (keep the "– used by Y" text) and let the user free the card with Y's ✕ first, or hold the choice in local state and commit on `blur`/Enter. Keep the pointer drag take-over. Add a keyboard test that presses `{ArrowDown}` twice and asserts that no other item lost its card.

**D3. Sorting: "Keep this order" unmounts under focus and says nothing.**
- Where: `M/↕️sorting:51-56` `{answer === undefined ? (<div …><p>{text("quiz.sorting.keepHint")}</p><BodyButton onClick={() => onAnswer({ kind: "sorting", order })}>{text("quiz.sorting.keep")}</BodyButton></div>) : null}`.
- Effect: pressing the button sets the answer, the block disappears, focus falls to `<body>`; no announcement. Chrome/Firefox keep a sequential start point for sighted keyboard users, screen-reader users get silence and are put back at the top. The existing test (`🧰️framework/🛍️products/❓️quiz/🧪️tests/⌨️task-keyboard/🟦️.tsx:153-158`) passes because jsdom falls back to the first tabbable element.
- Fix: in the handler call `announce(text("quiz.sorting.kept"))` (new key, EN "Order kept", DE "Reihenfolge übernommen") and `focusAfterRender(elementId(scope, order[0], "down"))`; or keep the element and turn it into the status text. Add the key to both bundles (the completeness test will enforce it).

**D4. The connection indicator is a polite live region and flips on every saved answer.**
- Where: `🟦️.tsx:298-307` `<p role="status" aria-live="polite" data-tone={status.tone} …>` with `connectionMessage` (`:173-180`): `pending > 0` → "Saving answers (N waiting)", then "All answers saved".
- Effect: each answer or move makes `pending` go 0 → 1 → 0, so a screen reader is likely to announce "Saving answers (1 waiting)" and "All answers saved" for every action, queued against the task's own "X is now at position …" message (`M/🧩️task:36-42`). Not measured with a screen reader.
- Fix: keep the text visible but not live for the `calm` and `busy` tones; mirror only transitions into the `alert` tone (offline, reconnecting) and the recovery from it into a live region, or debounce so "saving" is announced only after 2–3 s of pending. The mobile layout already shows glyph + hidden text (`max-md:sr-only`), keep that.

**D5. Others' name labels fail 4.5:1 for 5 palette slots in the light theme and 1 in the dark theme.**
- Where: `R/🎨️.css:225-235` `.quiz-peer-label { background: var(--quiz-peer); color: var(--base); font-size: 0.75rem; }`, used by `M/👥️presence:883,892`; slots `DS/🎨️styling/🎨️palette/🎨️.css:228-251`.
- Contrast (text `--base` on slot colour): light theme slot 2 `#1a891a` 4.07, slot 5 `#1a8989` 3.79, slot 7 `#89891a` 3.34, slot 9 `#1a8952` 3.98, slot 11 `#52891a` 3.81; dark theme slot 8 `#5858e4` 3.60. From slot 12 on the light theme gets lighter (`presenceColor`, +0.14 lightness) and is worse. These are names of other people, rendered text, 12 px: SC 1.4.3. The cursors can be switched off in the settings, the labels are `aria-hidden`.
- Fix: derive the ink per slot (`--quiz-peer-ink: black|white` chosen by relative luminance in `paintStyle`, `M/👥️presence:840-842`) and use it for `.quiz-peer-label`; or lower lightness of the hues 60–180 in the light palette source (the generated CSS comes from `DS/🎨️styling/🔣️.json`).

**D6. Windows high contrast / forced colors: every state shown by a background is lost.**
- Where: `R/🎨️.css` has no `forced-colors` rule. State carriers that rely on `background`/`box-shadow`: pressed segments (`M/🪟️chrome:153` `bg-active-base text-active-foreground`), the current task button (`M/▶️run:160-161`), the own leaderboard row (`R/🎨️.css:90-93` `.quiz-me` inset shadow + tint), the drop highlight (`:155-159`), online marks (`:262-270`), crowd markers (`:252-260`), the spider diagram area (`:189-193`, fill and stroke are forced to system colours, the polygon may cover rings and spokes).
- Effect: in forced colors the language, theme and text-size controls show no visible pressed state, the current task is not marked, own row marker, online dots and crowd markers vanish. Meaning stays available in text ("(you)", `aria-pressed`, sr-only "online"), but sighted high-contrast users lose it.
- Fix (append to `R/🎨️.css`):
  ```css
  @media (forced-colors: active) {
    .quiz-app [aria-pressed="true"], .quiz-app [aria-current="step"] { forced-color-adjust: none; background: Highlight; color: HighlightText; }
    .quiz-me { outline: 2px solid Highlight; outline-offset: -2px; }
    .quiz-drop-active { outline: 2px dashed Highlight; }
    .quiz-online, .quiz-crowd-marker, .quiz-peer-label { forced-color-adjust: none; background: CanvasText; color: Canvas; }
    .quiz-radar-area { fill: none; stroke: Highlight; stroke-width: 3; }
  }
  ```

**D7. Matching table: the header of the unassign column is the text "Not assigned".**
- Where: `M/🃏️matching:93-95` `<th scope="col" className={HEAD}><span className="sr-only">{text("quiz.matching.none")}</span></th>` (EN "Not assigned", DE "Nicht zugeordnet", the text of the empty select option).
- Effect: every ✕ button cell is announced under the column "Not assigned", the opposite of what the button does.
- Fix: new key `quiz.matching.actions` (EN "Remove", DE "Entfernen") for the header text.

### I — IMPROVE (25)

**I1. Classification select commits on every arrow press** (`M/🗂️classification:53-58`, `onChange={(event) => assign(item, …, true)}`). On Windows/Linux a closed select fires `change` per ArrowDown, so a keyboard user passing three categories moves the item and sends three answers/presence frames and three announcements. Non-destructive, but noisy. Fix: commit on `blur` or Enter, or accept and debounce the announcement (400 ms).

**I2. "Switch identity" forgets the learner at once** (`M/📇️profile:62,113` `onClick={() => session.forgetLearner()}`; `M/🧭️session:443-445`). For an anonymous learner the id is the only credential ("Your progress stays tied to this device", `M/🌐️i18n:100`), so this is irreversible data loss without confirmation (SC 3.3.4). Fix: confirm with the dialog pattern of the run screen, at least for `anonymous`.

**I3. Submit dialog details** (`M/▶️run`): `:223` the title stays "Submit this quiz?" while the body shows "Saving answers (3 of 12)" → switch the title to `quiz.run.submitting` in the working stage; `:218` the role flips `alertdialog` → `dialog` on the same node, keep one role; `:108-126` the trap lives on the dialog node, so after a click on the backdrop (focus on `<body>`) Escape and Tab no longer work and the page behind is reachable → add `inert` to `header`/`main` while open, or listen on `document`; `:246` the progress `role="status"` announces every step, give it `aria-live="off"` and announce only start and end.

**I4. Opened page: the "Overview" button is last in tab order and Escape is undiscoverable** (`DS/🧱️elements/🥞️LayeredOverview/🟦️.tsx:810-822` rendered after the strip, `:751-755` Escape listener). Fix: render the button before the strip (it is `absolute`, position stays) or add a visible "Esc" hint and `aria-keyshortcuts="Escape"`.

**I5. Focus indicator of the card heading link is only an underline** (`M/🪟️chrome:53` `className="text-inherit no-underline outline-none hover:underline focus-visible:underline"`). The card lift and the page reveal add a cue, but the link itself has a 1 px change. Fix: `focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--active-base)]` (3.2:1 on light, 5.4:1 on dark).

**I6. Status glyphs are read as words by screen readers.** `M/▶️run:167-169` `` `✓ ${text("quiz.run.complete")}` `` / `` `○ ${…}` `` inside the task button name; `M/🏁️results:22-23` `✓`/`◐`/`✗`, `:117`, `:161`. Fix: wrap each glyph in `<span aria-hidden="true">`; the word carries the meaning.

**I7. `list-none` removes list semantics in Safari/VoiceOver.** Add `role="list"` to: `M/↕️sorting:60`, `M/▶️run:152`, `M/🃏️matching:67`, `M/🏁️results:236`, `M/🏅️badges:48,79`, `M/👥️presence:999`, `M/📇️profile:157`, `M/📖️quiz-page:192`, `M/🗂️classification:72` (the `chips` class), `M/🪟️chrome:126` (`Facts`). In sorting the item numbers are `aria-hidden` (`:71`), so in Safari the order is not conveyed at all.

**I8. Data tables without a name or with an unnamed header cell.** `M/🃏️matching:84` `<table>` and `M/🏁️results:34-51` `ResultTable` have no caption/label (the card heading names the region, not the table) → `aria-labelledby` the task/dimension heading; `M/📇️profile:141` `<td className={head} />` in `<thead>` → `<th scope="col"><span className="sr-only">{text("quiz.learner.actions")}</span></th>`. In matching the row header cell also contains the live crowd sentence (`:105`), which makes the row name change while others type; move it into a separate cell or `aria-describedby`.

**I9. Chips may overflow on German labels** (not measured). The chip class is `whitespace-nowrap` + `max-w-[12rem]` + `overflow-visible` + `leading-none` (`DS/🔨️modules/🎛️chrome-control-presentation/🟦️.ts:15-21,35`), used by `BodyButton` (`M/🪟️chrome:163-178`), `Segments`, footer actions. Longest DE labels: "Diese Reihenfolge übernehmen" (28 chars, `M/🌐️i18n:482`), "Letztes Ergebnis ansehen" (24), "Jetzt erneut versuchen" (22), "Zurück zu den Quizzen" (21). In a 12 px display face the first two probably exceed 192 px. Fix: measure; allow wrapping for body buttons (`whitespace-normal max-w-none h-auto`) or shorten ("Reihenfolge übernehmen", "Letztes Ergebnis").

**I10. Overview cards are clipped when the viewport is short** (not measured). `DS/🧱️elements/🥞️LayeredOverview/🟦️.tsx:829` `<section … overflow-hidden>`, `:836` `<div className="pointer-events-none absolute inset-0 z-[31] flex items-center justify-center px-double pb-[5.5rem]">` (list mode; the card layer cannot scroll), `:850` strip `overflow-clip`; the layout switch is width-only (`DS/📱️device/🟦️.ts:34,39`). 200 % zoom on a 1280×720 window gives 640×360 (list mode, ~224 px for a ~260 px card), landscape phones and text size "Largest" (150 %, also `5.5rem` grows) behave the same. Content stays reachable through "tap opens the full page" and keyboard focus scrolling, but part of the card is not readable. Fix: make the list-mode card layer `overflow-y: auto` with safe centring, express the bottom reserve in px, and choose the list layout also by height or text scale (`(max-height: …)` or `quiz-text-scale > 1.25`).

**I11. Brand button can have an empty name; the logo is not hidden from AT.** `🟦️.tsx:360` `title = state.catalog === undefined ? "" : …`, `:371-373` the button contains only the logo and an empty span until the catalog loads; `:363` `<ShellBrandLogo …>` → `DS/🧱️elements/🔝️Navbar/🟦️.tsx:254-256` renders raw SVG without `aria-hidden`. Fix: `aria-label={text("quiz.nav.home")}` on the button while `title === ""` and wrap the logo in `<span aria-hidden="true">`.

**I12. Spider diagram: the name does not say that a table exists, and the values are collapsed.** `M/🕸️radar:459-460` name is `"Spider diagram of {{name}}"` only, the table sits in a closed `<details>` (`:481-482`). Fix: add `aria-describedby` pointing to the summary text "Values as a table" or put a short value list into `<desc>` (axis: value unit, in axis order). Not a failure of SC 1.1.1 because the table is adjacent and discoverable.

**I13. Plurals.** `quiz.home.points` `"{{points}} points"` / `"{{points}} Punkte"` (`M/🌐️i18n:118,390`), `reconnectingWaiting` `"({{waiting}} answers kept…)"` / `"({{waiting}} Antworten … gesichert)"` (`:71,343`; waiting = 1 is the common case at the start of an outage: "1 answers", "1 Antworten"). Fix without plural machinery: `"Points: {{points}}"`, `"Answers kept on this device: {{waiting}}"`, DE `"Punkte: {{points}}"`, `"Auf diesem Gerät gesicherte Antworten: {{waiting}}"`.

**I14. Technical English detail in German sentences.** `M/🌐️i18n:316,588` `refused` + `{{detail}}` is filled with `error.message`, `HTTP 500`, `revisionConflict: …` (`M/🛂️proctor:79,233`, `M/🧭️session:283`, `M/🪪️identity:22,27`, `M/📮️outbox:296`). Fix: show only a localized sentence for the known kinds (offline, 5xx, conflict) and keep the detail in a collapsed `lang="en"` span or the console.

**I15. Settings summary is stale.** `M/🌐️i18n:89` / `:361` list four settings; the panel has five (`showAnswers`, `M/🎛️preferences:107-110`). Add "and what others think" / "und was die anderen denken".

**I16. German UI wording** — the table in A4 (UI chrome), 13 rows.

**I17. German catalog/content wording** — the lists in A4 (catalog and quizzes): "Quizze Architektur und Technologie", "zieht und mischt", "wo die Werte…", "Rate" ×3, "das Füllen", "macht … zu Bewegung", "der 1960er" ×9, "von den 1960ern".

**I18. One word per concept (DE).** "Rangliste" (UI) vs "Bestenliste" (`S/🔣️.json:27`); "Wertung" (`quiz.home.best`, `learner.score`, `results.score`) vs "Ergebnis" (`results.title`, catalog "Quiz-Ergebnisse") vs "Punkte" (`home.points`) — pick Wertung for the percentage and Punkte for the sum; "Klassifizierung" (UI) vs "Klassifikation(en)" (`S/🔣️.json:19,77`); "Lernende" vs "Person" (A4).

**I19. English wording** — the list in A4 (English quality), 7 items: "Thinking along", "⌀ place", "costs the more", "should stay yours", "asks" ×4, "draws … anew", "100 %".

**I20. Number and unit typography in content.** (a) `C/🧲️physics/❓️quiz/🔣️.json:138,425,434` write `33 000`, `10 529`, `164 000` with a normal space — it can break across lines; use U+202F (narrow no-break space) and U+00A0 between value and unit everywhere in content (today plain spaces: "2 kW", "110 m² × 160 W/m²"), as `withUnit` already does in the UI. (b) DE content writes `8000`, `2500`, `1000`, the UI shows `8.000 kWh/(m²·a)` (`Intl` "de"); match one convention, e.g. thousands dots in DE content or `useGrouping: "min2"` in the UI. (c) Energy sorting shows prefixed units "164,4 PWh" / "2,925 PWh" on the cards while the explanations say "164 000 TWh" / "2925 TWh" (`C/🧲️physics/❓️quiz/🔣️.json:424-425,433-434`, energies sorting task); use the card's prefix in the explanation or cap `prefixed` at T. (d) `−` (U+2212) in content vs hyphen-minus produced by `Intl` for "-5 €/(m²·a)" in the radar table (`M/🕸️radar:508-510`); minor.

**I21. Submit button is `disabled`, so its explanation cannot be reached from the button.** `M/▶️run:141` `disabled={!ready} aria-describedby={ready ? undefined : …}`. A disabled button is not focusable, the `aria-describedby` never speaks. The hint is visible and read in sequence, so this is a polish item: use `aria-disabled` + ignore click + keep focus, as the move buttons do.

**I22. German abbreviations are unexplained for the English audience.** EN texts say "EnEV 2014", "KfW Efficiency House 40", "WSchVO 1995", "BAnz AT 04.12.2020 B1", "IWU type EFH_E" (`C/📊️demand`, `C/🔥️heating`); only GEG and one WSchVO label are expanded. Expand at first use per quiz ("Energy Saving Ordinance (EnEV) 2014", "KfW (German development bank) Efficiency House").

**I23. Sorting announcements.** `M/↕️sorting:44` the same sentence twice (pressing "up" at the top twice) is not announced again by a polite region; the module header claims buttons "announce why nothing moved" (`:3`) but the text only repeats the position. Fix: say "already first"/"already last" (new keys) or clear the region before re-announcing.

**I24. `lang` of the first paint and detection.** `🟦️.tsx:347-349` applies the shared port's language in an effect (see A3); call `setUiLocale` once synchronously in `mountQuiz` before `createRoot` (it only touches `documentElement.lang` and the i18next port). Same place as the D1 fix.

**I25. Others' cursor labels can cover the focused control** (SC 2.4.11 Focus Not Obscured, not measured). `R/🎨️.css:225-235` `.quiz-peer-label` (name pill at `top: 1rem; left: .75rem` of the pointer) inside `fixed inset-0 z-40` (`M/👥️presence:939`) is above the page and above the focus ring. The setting "Show others' cursors" exists (`M/🎛️preferences:103-106`). Fix: lower the overlay's z-index below the sticky/focused control, or hide peer labels while the pointer of a peer is within 24 px of `document.activeElement`.

### N — NOTE (14)

- **N1.** `M/🌐️i18n:26-33` `preferredLocale` falls back to `"en"` when neither English nor German is in `navigator.languages` (e.g. `fr`, `it`). Documented and softened by the always visible `LanguageSwitch`; the strict reading of "no default language" would show a neutral two-language chooser instead.
- **N2.** `Intl` with `"en"` gives US date style ("Oct 1, 2026, 3:00 PM", `M/📏️quantity:119-132`); for European English readers `"en-GB"` reads better. Same decision for all `Intl` calls.
- **N3.** No `<noscript>`; without JS the page stays hidden (`html:not([data-semio-styled]) body{visibility:hidden}` in the built `index.html`).
- **N4.** The logo asset `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg` contains a stray `--&gt;` text node right after `<svg …>` and has no `<title>`; harmless to AT, a glitch in the asset.
- **N5.** Body font is the display face Anta for all text; there is no font or line-spacing preference. Legibility for dyslexic readers is a design-system topic.
- **N6.** Landmark density: the classification view makes every bin, the pool and the group a named `<section>` (`M/🗂️classification:75,81,90`), up to 8 regions per task; headings alone would be calmer.
- **N7.** Sorting has two tab stops per item (14 items → 28 stops). Fine for correctness; a roving grid or Alt+Arrow on the item would shorten it.
- **N8.** Touch users cannot peek at a page without opening it (the tap opens the page, `M/🪟️chrome:69-77`); and opening by link adds a history entry while opening by pointer does not (`LayeredOverview:196-199` `replaceState`), so Back closes in one case and leaves the site in the other.
- **N9.** Placeholders "–" have no text for assistive technology: `M/🏆️leaderboard:270` (no best score), `M/📇️profile:83,85` (no score, not submitted). The results table does it right (`M/🏁️results:61-62`).
- **N10.** `S/🔣️.json:26` / `:84`: "Flawless runs earn badges: … and for completing every quiz" — the badge "Completionist" needs only one submitted run of every quiz, not flawless ones.
- **N11.** `C/🔥️heating` prompt says "both per m² of heated floor area" while the explanations speak of "Wohnfläche / living area" and "Gebäudenutzfläche A_N" (`:189`, `:226`, `:253`); say which reference area the numbers use.
- **N12.** EN content uses an em dash with spaces ("—"), EN UI strings use an en dash with spaces ("–", e.g. `M/🌐️i18n:56`); pick one.
- **N13.** The one coloured identity cue the learner has for others is the online mark colour (`.quiz-online`, 0.5 rem square); it passes 3:1 against the page in both themes (3.3–14:1) and the text name is always next to it, so this is only a reminder that colour never carries the meaning alone.
- **N14.** `Brennwert` is used for food energy (`C/🧲️physics:143`, correct on German nutrition labels) while the building context uses `Heizwert` for fuels; fine, but the word has a second meaning for boilers (Gas-Brennwertkessel, same quiz family).

---

## Verified OK

- EN/DE key parity (224/224), identical placeholders, no unused or undefined keys, no formal address; the completeness test `🧰️framework/🛍️products/❓️quiz/🧪️tests/🗣️translation-completeness/🟦️.tsx` enforces it.
- Schema requires `en` and `de` on every learner text; no text with one language.
- All numbers, scores, points, dates and times formatted with `Intl` in the active locale; no-break space before units; locale collator for sorting; DE decimal comma and EN decimal point in all content.
- Terminology consistent across the 4 quizzes: Heizlast, Heizwärmebedarf, Kühllast, Kühlbedarf, Endenergiebedarf, Luftwechselrate, U-Wert / U-value, GEG, EnEV, WSchVO, KfW-Effizienzhaus, Passivhaus (Institut).
- Language of parts: `lang` on `.quiz-app`, on language buttons, `documentElement.lang` follows the choice.
- Viewport meta without zoom lock; skip link; one `h1` per screen; focus moves to the `h1` on every screen change (`🟦️.tsx:347-354`); landmarks `header`/`nav`/`main`.
- Identity form: labelled radios in a fieldset with legend, hint via `aria-describedby`, `aria-invalid`, error `role="alert"`, `autocomplete` (`name`, `nickname`), cancel button during sign-in.
- Drag has an alternative everywhere (selects, move buttons, ✕), the grip is hidden from AT; drag ghost is `aria-hidden`; Escape abandons a drag.
- Presence, crowd and leaderboard updates are not live regions (verified in `M/🗳️crowd`, `M/👥️presence`, `M/🏆️leaderboard`); the leaderboard fetching message is a status only while the page is opened (`M/🏆️leaderboard:229`); decorative cursor layers are `aria-hidden` and `pointer-events-none`; presence in the navbar and roster is text.
- Layered home: backdrop panes `inert` + `aria-hidden`, opened page is a labelled region that takes focus, Escape closes, focus returns into the card, hover reveal only for mouse and keyboard-focus reveal for `:focus-visible`, tap opens on touch, `prefers-reduced-motion` snaps glide and zoom.
- Leaderboard table: `aria-sort`, buttons in header cells, chevrons as shapes, named scrollable region (`tabIndex={0}`), caption for the excerpt table, own row marked by text, `aria-current` and a tint.
- Results: verdicts as word + symbol, never by colour; tables with `scope`; explanation column present.
- Spider diagram: `role="img"` + `<title>`, table with caption/units/min/max, labels wrapped or numbered legend, no colour-only series.
- Target size: all controls ≥ 24 px (`.quiz-target`, chrome `min-h-medium`).
- Contrast of the main tokens: foreground 17.3:1 (light), muted-foreground 8.4:1 (light) / 6.4:1 (dark), normal border 3.5:1 / 4.9:1, active colour 3.2:1 / 5.4:1, dark ink on active 5.4:1; presence colours ≥ 3.3:1 against the page.
- Reduced motion: card lift `motion-reduce`, peer glide cut, pulse placeholders `motion-reduce:animate-none`.
- Text size preference: root font size scale, all rem-based; the radar measures its own em.

## Not measurable from the source (to confirm with a browser or screen reader)

1. D4 chatter with NVDA/VoiceOver during a sorting task.
2. D2 on Windows Chrome/Firefox with the arrow keys (and that macOS opens the popup instead).
3. I9 chip overflow for the long German labels at 100 % and "Largest".
4. I10 clipping at 200 % zoom, 400 % zoom (320 px), landscape phone, text size "Largest".
5. D6 how the chosen browser forces SVG `fill`/`stroke`.
6. Contrast of muted text on the translucent glass cards over the blurred pages (tokens pass on the flat page colour).
