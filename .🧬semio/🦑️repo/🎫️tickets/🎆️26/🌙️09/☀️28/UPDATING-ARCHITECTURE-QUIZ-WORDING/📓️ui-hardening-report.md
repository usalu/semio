# 📓️ UI hardening — defects, languages and accessibility

Work package "UI defects, languages and accessibility" of the quiz website's React client (agent "ui", 2026-10-02).
Sources: `📓️audit-final-i18n-a11y.md` (D1–D7, I1–I25, N1–N14) and `📓️audit-final-requirements.md` (D1, D2, D3, D5; the client
part of D4). Paths: `RX` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `RX/🔨️modules`, `T` = `🧰️framework/🛍️products/❓️quiz/🧪️tests`,
`F` = `🧰️framework/🛍️products/❓️quiz/🧫️fixtures`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `DS` = `🧰️framework/🔨️modules/🖱️ui`.

## 1. Result in numbers

| Class | Fixed | Rejected | Open |
|---|---|---|---|
| DEFECT, i18n/a11y audit (7) | 7 | 0 | 0 |
| DEFECT, requirements audit (D1, D2, D3, D5) | 4 | 0 | 0 |
| IMPROVE, i18n/a11y audit (25) | 24 and I20 (a)–(c) | I20 (d) | 0 (I9, I10, I25 fixed but not measured in a browser, see §6) |
| NOTE, i18n/a11y audit (14) | N1, N3, N9, N10, N12 | — | N2, N4, N5, N6, N7, N8, N11 (N13, N14 need no change) |
| New deliverables | wording (identity, for fun, privacy notice, footer), leaderboard page on the new contract, rate limits, CSP check | — | — |

## 2. Gates (commands as run, results as printed)

| Command | Result |
|---|---|
| `bun nx run-many -t test -p @semio-tech/quiz-react @teaching/architecture-quiz --skip-nx-cache` | `Successfully ran target test for 2 projects` (exit 0) |
| `bun ./📜️script.ts test` in `RX/📦️packages/🟦️typescript` | `Test Files 16 passed (16)`, `Tests 364 passed (364)` (baseline before this work: 230 tests) |
| `bun ./📜️script.ts test` in `S/📦️packages/🟦️typescript` | `Test Files 4 passed (4)`, `Tests 33 passed (33)` |
| `bun nx run @semio-tech/quiz-react:typecheck --skip-nx-cache` | **exit 1**: 133 errors, **0** in the quiz target, its tests, the quiz core or the files I changed in `DS`. All 133 are in `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` (83), `🖱️ui/🧬️contract/🧵️retained/**` (46) and `🎭️actor/🤖️generated/🎭️actor/🟦️.ts` (4): git-ignored `🤖️generated` bindings on this machine are older (2026-09-14) than their sources (2026-09-30). Predates this work (`📓️react-report.md`: 47 errors of the same kind on 09-29). Regenerating is `bun nx run workspace:schema-generate`, a workspace-wide step outside this package; not run while four other agents build |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `clean=true errors=0 warnings=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `clean=false errors=1`: `directory-kind-unresolved 🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity` — the edge agent's new directory, not registered yet; nothing of mine |
| `bun nx run @semio-tech/ui-react:test-exhaustive --skip-nx-cache` (I touched `DS`) | `2 failed | 33 passed (35)` files, `17 failed | 1027 passed (1044)` tests. Baseline before my change: `20 failed | 1024 passed`. The 17 are a subset of the 20 (Shell/Mode/Window/Tree tests of `🟦️.tsx`, `ShellScope` tokens); none concerns `LayeredOverview`. The three that pass now (Tree text-flow, Layout overlay, iconCodec) are others' work |
| `DS/🎨️styling` suite (I touched the host template) | 64 pass / 4 fail before and after; in-source `-t "semioHostHtml"`: `6 passed | 75 skipped` |
| harness `contract --owner "🧰️framework/🛍️products/❓️quiz"` | exit 1 with `5865 high-priority breach(es)` across the whole repository (the command reports every owner: plugins, `temp/…`); **0** lines for `🧰️framework/🛍️products/❓️quiz`, `🎓️teaching` or the `DS` files I touched. The first run had 5867: the two extra ones were my test directory `📄️host-document` (generic emoji), renamed to `📰️host-document` (decision 9) |

Browser check (dev stack on 6071/8801 with its own data folder, stopped afterwards; Chromium, English and German): first
paint title pair and no `lang`; `lang`/title follow language and screen; privacy dialog and switch-identity dialog
(modal, inert siblings, Escape, focus back); refused pseudonym explained at the field; anonymous registration (262 ms),
pseudonym registration, recall by query only (166 ms); 12 classification choices in 1.4 s → one announcement, connection
announcer silent while the indicator flips; "Keep this order" → focus on the list, "Order kept"; `aria-disabled` submit
with hint; submit dialog → results; leaderboard page ("Learners in total: 1", own row not doubled); badges page (lock icon,
"Noch nicht erhalten", dashed border, opacity 1); runs table header cells; matching select with a disabled used card.
Not checkable there: forced colors, contrast, the neutral language chooser (the pane's browser list is `en-GB`) and a
`429` (all four are pinned by tests). Screenshots were not available (hidden pane), so nothing was judged by eye.

## 3. Findings → status → test → files

### DEFECT — i18n/a11y audit

| Id | Status | Test | Files |
|---|---|---|---|
| D1 document language and title | fixed: no `lang` and a title in every language before the app runs; `lang` and `document.title` follow the chosen language and the screen; loading text and `<noscript>` per language; locale applied before the first paint (also I24) | `T/🌍️language-choice` (5 tests, i18next as oracle for the negotiation, fixture `F/🌍️language-choice`), `S/🧪️tests/📰️host-document` (5), `DS/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin` (2 new) | `RX/🟦️.tsx`, `M/🌐️i18n`, `M/🎛️preferences`, `DS/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, `S/🏗️builder/🌐️vite/🟦️.ts`, `S/🌐️.html` |
| D2 matching: arrow keys take cards away | fixed: a card another item uses is a `disabled` option ("… – used by …"); nothing changes elsewhere until the learner removes it there or drags it | `T/⌨️task-keyboard` (arrow-down model over every option) | `M/🃏️matching` |
| D3 sorting "Keep this order" | fixed: focus moves to the list, live region says "Order kept" | `T/⌨️task-keyboard` | `M/↕️sorting`, `M/🧩️task` |
| D4 connection indicator chatters | fixed: the indicator is no live region; a separate polite status speaks once on entering an alert state and once on recovery | `T/📢️live-regions` (aria-query as oracle for which roles are live, fixture `F/📢️live-regions`) | `RX/🟦️.tsx` (`ConnectionStatus`) |
| D5 peer label contrast | fixed: each palette slot gets black or white ink by WCAG luminance, per appearance; ≥ 4.5:1 for every slot in both themes | `T/🌗️contrast-states` (colord as oracle, fixture `F/🌗️contrast-states`) | `M/👥️presence` (`peerInk`, `paintStyle`), `RX/🎨️.css` |
| D6 forced colors | fixed: `@media (forced-colors: active)` rules for pressed/current, own row, drop target, online mark, crowd marker, peer label, radar area, disabled, focus | `T/🌗️contrast-states` (the stylesheet parsed with lightningcss; every stateful selector of the fixture must have a rule) | `RX/🎨️.css` |
| D7 matching unassign column header | fixed: header "Remove" / "Entfernen"; table named by its dimension heading; row header is the item label only (also I8) | `T/⌨️task-keyboard` | `M/🃏️matching`, `M/🌐️i18n` |

### DEFECT — requirements audit

| Id | Status | Test | Files |
|---|---|---|---|
| D1 switch identity without confirmation | fixed: `alertdialog`; anonymous: "nobody – you included – can ever continue them", confirm "Switch and give up this progress"; named: how to come back; initial focus on "Keep this identity" (also I2) | `T/📇️learner-pages` ("switching identity", 2), `T/🚶️learner-journey` | `M/📇️profile`, `M/🪟️chrome` (`Dialog`), `M/🌐️i18n` |
| D2 baked default language, German-only title | fixed, see D1 above; `preferredLocale` returns none instead of English; the shared port's `fallbackLng` is never reached because every key exists in both bundles (`T/🗣️translation-completeness`) | as D1 | as D1 |
| D3 empty header cell in the runs table | fixed: every header cell is a `th`, the last one "Actions" for assistive technology (also I8) | `T/📇️learner-pages` ("the runs table") | `M/📇️profile` |
| D5 badges told apart by opacity only | fixed: check/lock icon, visible words, solid/dashed border, no opacity; same on card and page | `T/📇️learner-pages` ("badges", 2) | `M/🏅️badges` |
| D4 (client part) | done: page for `Leaderboard { rows, learners, own? }`, state identity kept when the answer is unchanged, jittered self-scheduling poll with backoff | `T/🏠️home-grid` (page vectors of `F/🏠️home-grid`: 250 learners; own rank 100 inside the sent rows, own rank 180 apart, not ranked), `T/🚦️rate-limits` ("polling") | `M/🏆️leaderboard`, `M/🧭️session`, `M/🛂️proctor`, `M/🏠️home`, `M/📇️profile` |

### IMPROVE — i18n/a11y audit

| Id | Status | Test | Files |
|---|---|---|---|
| I1 select commits per arrow press | fixed the audit's second way: accepted, announcement debounced 400 ms (latest wins) | `T/⌨️task-keyboard` | `M/🧩️task`, `M/🗂️classification`, `M/🃏️matching` |
| I2 switch identity | fixed (requirements D1) | `T/📇️learner-pages` | `M/📇️profile` |
| I3 submit dialog | fixed: one role, title follows the stage, progress not live + one spoken status, trap/Escape at document level, portal + `inert` | `T/🚶️learner-journey` | `M/▶️run`, `M/🪟️chrome` |
| I4 Overview button last, Escape undiscoverable | fixed: first in the DOM, `aria-keyshortcuts="Escape"` | `T/🏠️home-grid` | `DS/🧱️elements/🥞️LayeredOverview/🟦️.tsx` |
| I5 focus indicator of card links | fixed: outline | — (style) | `M/🪟️chrome`, `RX/🎨️.css` |
| I6 status glyphs read as words | fixed: `Mark` (`aria-hidden`) + word | `T/🗣️translation-completeness` (source scan), `T/🚶️learner-journey` | `M/🪟️chrome`, `M/▶️run`, `M/🏁️results` |
| I7 `list-none` lists | fixed: `role="list"` on every one | `T/🗣️translation-completeness` (source scan) | 10 modules |
| I8 tables without a name | fixed | `T/⌨️task-keyboard`, `T/📇️learner-pages` | `M/🃏️matching`, `M/🏁️results`, `M/📇️profile` |
| I9 chips overflow on German labels | fixed by both proposed means (body buttons wrap; labels shortened: "Reihenfolge übernehmen", "Letztes Ergebnis"); **not measured** | — | `M/🪟️chrome`, `M/🌐️i18n` |
| I10 cards clipped on short viewports | fixed: list layer scrolls with safe centring, reserve in px, list mode below 480 px × text scale, breakpoints scale with the text size; **not measured** | `T/🏠️home-grid` (layout by text scale and height) | `DS/🧱️elements/🥞️LayeredOverview/🟦️.tsx`, `M/🏠️home` |
| I11 brand button name, logo hidden | fixed | `T/🚶️learner-journey` | `RX/🟦️.tsx` |
| I12 spider diagram description | fixed: `<desc>` with the axis values, `aria-describedby`, "The values follow as a table." | `T/⌨️task-keyboard` | `M/🕸️radar` |
| I13 plurals | fixed: counts as "Label: n" | `T/🗣️translation-completeness` | `M/🌐️i18n` |
| I14 technical English in German sentences | fixed: localized sentence; the technical text in a collapsed "Technical details" marked `lang="en"` | `T/🗣️translation-completeness` (no sentence carries `{{detail}}`), `T/🎭️identity-step` (what the proctor refuses) | `M/🪟️chrome` (`ProblemNote`), `M/🪪️identity`, `M/🏠️home`, `M/▶️run` |
| I15 settings summary | fixed | `T/🗣️translation-completeness` | `M/🌐️i18n` |
| I16 German UI wording (13 rows) | fixed | `T/🗣️translation-completeness` ("one German word per concept", informal "du") | `M/🌐️i18n` |
| I17 German catalog/content wording | fixed (15 strings + catalog) | `S/🧪️tests/🧪️catalog`, `ui_content_structure_check.py` | `S/🔣️.json`, four quiz files (`📓️ui-content-wording-report.md`) |
| I18 one word per concept | fixed: Rangliste, Wertung (percentage) / Punkte (sum), Klassifizierung | `T/🗣️translation-completeness`, `S/🧪️tests/📰️host-document` | `M/🌐️i18n`, `S/🔣️.json` |
| I19 English wording (7) | fixed | — | `M/🌐️i18n`, `S/🔣️.json`, quiz files |
| I20 number and unit typography | (a), (b), (c) fixed in content (980 no-break spaces, 34 German thousands separators, card value first); **(d) rejected**, §5 | `ui_content_typography_check.py` | four quiz files |
| I21 submit `disabled` | fixed: `aria-disabled` + reachable hint | `T/🚶️learner-journey` | `M/▶️run` |
| I22 German abbreviations for English readers | fixed (14 expansions) | — | quiz files |
| I23 sorting announcements | fixed: "already first / last" | `T/⌨️task-keyboard` | `M/↕️sorting`, `M/🌐️i18n` |
| I24 `lang` at first paint | fixed with D1 | `T/🌍️language-choice` | `RX/🟦️.tsx` |
| I25 labels over the focused control | fixed: a label within 24 px of the focused control is hidden until focus moves; **not measured** | `T/📡️presence-client` | `M/👥️presence`, `RX/🎨️.css` |

### NOTE — i18n/a11y audit

| Id | Status |
|---|---|
| N1 fallback to English | fixed: neutral chooser (decision 1) |
| N3 no `<noscript>` | fixed: one paragraph per language, visible although the boot style hides the body |
| N9 "–" without text | fixed: `Missing` (dash + hidden words) in leaderboard, profile, results, radar |
| N10 badge sentence wrong | fixed in the catalog introduction |
| N12 em dash vs en dash | fixed: en dash with spaces everywhere a learner reads |
| N2 `en` date style | open: `Intl` with `en` stays; switching to `en-GB` is a content decision for the owner |
| N4 logo asset stray text node | open: asset of `🧰️framework/🔨️modules/🖼️assets`, hidden from assistive technology now (I11) |
| N5 font, N6 landmark density, N7 tab stops in sorting, N8 touch peek and history | open: design topics, no defect |
| N11 reference area in the heating task | open: needs the author's decision which area the numbers use |
| N13, N14 | no change needed |

### New deliverables

| Deliverable | Test | Files |
|---|---|---|
| Honest identity wording: public, no passwords, pseudonym recommended, anonymous progress on this device and lost on switch/clear; "for fun" on identity step, leaderboard and introduction | `T/🎭️identity-step` | `M/🪪️identity`, `M/🌐️i18n`, `M/🏆️leaderboard`, `S/🔣️.json` |
| Identification on the domain contract (register by command, recall by query, `handle-claimed` → ask again, handle policy explained per character) | `T/🎭️identity-step` (fault vectors of `F/🎭️identity-step` in both languages, then 7 flow tests), `T/📬️outbox-delivery`, `T/🚶️learner-journey` (proctor double on `decideHandle`/`decideLearner`) | `M/🧭️session`, `M/🛂️proctor`, `M/🪪️identity` |
| Footer with imprint/privacy links from `site.legal`, built-in notice "What is stored" (what the server stores, what the browser stores, what is live only; no legal claim) | `T/🔏️privacy-notice` (3), `S/🧪️tests/📰️host-document` | `M/⚖️legal` (new), `RX/🟦️.tsx`, `S/🟦️.ts`, `S/🚀️deploy/🔣️.json` |
| `429` / `503` + `Retry-After` as a transient shortage: outbox holds, loads and polls wait what was asked plus jittered backoff, presence rejoins back off when a socket flaps; the indicator says "Server busy – retrying shortly" (tone busy, no alert, no retry button) | `T/🚦️rate-limits` (9, fixture `F/🚦️rate-limits`) | `M/🛂️proctor`, `M/📮️outbox`, `M/🧭️session`, `M/🏆️leaderboard`, `M/👥️presence`, `RX/🟦️.tsx` |
| CSP | read against the deploy agent's policy: nothing to widen (`🗒️ui-notes-for-deploy.md` §1); the host document gained no inline block (`S/🧪️tests/📰️host-document`) | — |

## 4. Decisions

1. **No default language.** Order of offering: English, then German. A stored choice wins. Otherwise the browser's list
   only preselects among the offered languages (first offered entry). When it names none, nothing is assumed: `<html>` has
   no `lang`, title/loading/noscript are bilingual with `lang` on each part, and the client asks in both languages at once.
   Recorded in `📓️design.md` §10 and in the docstring of `M/🌐️i18n`.
2. **Matching.** A used card is a disabled option rather than a confirm step: browsing can never change another item, and
   the option says who uses it. Pointer dragging keeps taking a card over (one deliberate gesture, announced).
3. **Connection status.** Visible text is never live; one polite status speaks on entering an alert state and on recovery.
   A throttled proctor is "busy", not an alert.
4. **"Updated …" on the leaderboard** is when the standings last changed in this view (the state keeps its identity when
   a poll returns the same answer, so nothing re-renders).
5. **Dialogs** are portaled into `.quiz-app` and make their siblings `inert` (theme and text-size variables still apply).
6. **Destructive default.** In "Switch identity?" the safe button has the initial focus.
7. **`<noscript>`** uses `style` attributes, which the policy already admits, instead of a new `<style>` block that would
   change the hashed set of every semio host.
8. **Test oracles** (dev dependencies of the react package only, registered in `🔮️oracles/🔣️.json`): colord (contrast),
   aria-query (live roles), i18next (language negotiation), next to lightningcss and lodash that were there.
9. **Test directory name.** `📄️` is a generic emoji the test harness refuses for a case name, so the site's new test is
   `📰️host-document`.

## 5. Rejected

- **I20 (d)** — replace the hyphen-minus that `Intl` writes for negative numbers in the UI by U+2212. `Intl.NumberFormat`
  is the formatting authority and the test oracle of the quantity formatting in both languages (CLDR writes hyphen-minus
  for `en` and `de`); post-processing its output would make the UI differ from its oracle and from what a learner's own
  locale tools show. Content prose keeps U+2212 (0 changes needed there).

## 6. Open issues

- `:typecheck` exits 1 because of the stale generated framework bindings (§2). The quiz target itself has 0 errors.
- `🎓️teaching` taxonomy: one error in the edge agent's `🏋️capacity` directory (§2).
- I9, I10 and I25 are implemented and unit-tested but not measured at 200 % / 400 % zoom, "Largest" text or with long German
  labels in a browser; forced colors and contrast were not looked at in a real high-contrast mode (the audit's list "not
  measurable from the source", items 3–6, stays open for a person with a screen).
- The site has no typecheck target; its entry and vite config are covered by the node test and by running the dev server.
- The dev proctor refuses the existing dev data folder `.🧬semio/🎓️teaching/proctor-dev` (storage format v1, the proctor
  reads v2): `bun ./📜️script.ts dev` fails on this machine until that folder is removed. Not mine to delete; told deploy
  in `🗒️ui-notes-for-deploy.md`.
- `site.legal.imprint` and `site.legal.privacy` are unset: the owner's step before the public launch.
- N2, N4–N8, N11 (§3).
- Repo MCP servers (`repo`, `semio`) did not connect in this session: the ticket was neither reopened nor closed through
  them; all files are in the existing ticket folder.

## 7. Notes for the other agents

`🗒️ui-notes-for-e2e.md` (flows, hooks, every text old → new in both languages), `🗒️ui-notes-for-deploy.md` (CSP, host
document, `site.legal`, edge headers), `🗒️ui-notes-for-domain.md` (what the client relies on).

## 8. Files

Created
- `M/⚖️legal/🟦️.tsx`
- `T/{🌍️language-choice,📢️live-regions,🌗️contrast-states,📇️learner-pages,🚦️rate-limits,🎭️identity-step,🔏️privacy-notice}/🟦️.tsx`
- `F/{🌍️language-choice,📢️live-regions,🌗️contrast-states,🚦️rate-limits,🎭️identity-step}/🔣️.json`
- `S/🧪️tests/📰️host-document/🟦️.ts`
- ticket: `📓️ui-hardening-report.md`, `📓️ui-content-wording-report.md`, `🗒️ui-notes-for-e2e.md`, `🗒️ui-notes-for-deploy.md`,
  `🗒️ui-notes-for-domain.md`, `ui_text_changes.ts`, `ui_content_typography_check.py`, `ui_content_structure_check.py`

Updated
- `RX/🟦️.tsx`, `RX/🎨️.css`
- `M/{🌐️i18n,🛂️proctor,📮️outbox,🧭️session}/🟦️.ts`
- `M/{🪟️chrome,🧩️task,↕️sorting,🃏️matching,🗂️classification,🪪️identity,🏆️leaderboard,📇️profile,🏅️badges,▶️run,🏁️results,🕸️radar,🎛️preferences,👥️presence,🏠️home,📖️quiz-page}/🟦️.tsx`
- `RX/📦️packages/🟦️typescript/{package.json,tsconfig.json,📋️project.json}`, `RX/🧪️tests/🎚️config/🟦️.ts`, `bun.lock`
- `T/{⌨️task-keyboard,🚶️learner-journey,🏠️home-grid,📬️outbox-delivery,🗣️translation-completeness,💭️crowd-client,📡️presence-client}/🟦️.tsx`
- `F/🏠️home-grid/🔣️.json`, `🧰️framework/🛍️products/❓️quiz/🔮️oracles/🔣️.json`
- `DS/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, `DS/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts`, `DS/🧱️elements/🥞️LayeredOverview/🟦️.tsx`
- `S/🏗️builder/🌐️vite/🟦️.ts` (shared with deploy), `S/🌐️.html`, `S/🟦️.ts`, `S/🔣️.json`, `S/🚀️deploy/🔣️.json` (`site.legal`), `S/🧪️tests/🎚️config/🟦️.ts`
- `🎓️teaching/🏛️architecture/README.md`, `🎓️teaching/🏛️architecture/⚡️energy/{🧲️physics,🔥️heating,❄️cooling,📊️demand}/❓️quiz/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (module `⚖️legal`, five fixture and eight test directories)
- ticket: `📓️design.md` (§10 language decision)

Removed
- nothing in the repository; `🗑️generated/ui/` of the ticket (my tool output) is deleted. No `[DEBUG]` log was added.
