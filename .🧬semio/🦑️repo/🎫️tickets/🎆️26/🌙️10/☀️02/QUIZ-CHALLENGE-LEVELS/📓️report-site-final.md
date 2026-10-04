# Report — site gates of the challenge levels: script budget, rehearsal topology, e2e breadth

Agent: site final. `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`,
`M` = `R/🔨️modules`, `T` = this ticket folder. The acceptance-fix report (`📓️report-acceptance-fixes.md`) existed before
any e2e run of mine started; the budget work was done before it.

## 1. Script size budget

### What the budget measures

`siteArtifactProblems` (`S/🚀️deploy/🟦️.ts:347`) sums the gzip size of every script and stylesheet that `index.html`
**links** (`<script src>` / `<link href>` under `/assets/`): the entry script and the entry stylesheet. Lazy chunks
(dynamic `import()`, not modulepreloaded) are not counted. `QUIZ_SITE_BUDGET = { scriptGzipBytes: 260 000,
styleGzipBytes: 60 000, totalBytes: 4 000 000 }`. So the budget is about the first load.

### What grew (measured, release build for `http://127.0.0.1:8892`)

| When | Entry script gzip | Source |
|---|---|---|
| pets round 1 (2026-10-02, ~13:00) | 216 519 | `QUIZ-PETS/📓️report-wp-p.md` |
| 2026-10-02 14:00 → 14:24 → 15:02 | 227 880 → 250 418 → 250 475 | `QUIZ-PETS/📓️report-wp-r.md` (other tickets' work) |
| `HEAD` (commit 2026-10-02 22:24; challenge levels not yet begun) | **254 965** | `T/head_variant.ts` + `head_variant.vite.ts`: the 39 entry sources changed since `HEAD` loaded at their `HEAD` text |
| pets round 2 C3 (2026-10-03 13:47) | 268 525 | `QUIZ-PETS/📓️report2-c3.md` (C3 alone: +1 239) |
| integration report | 269 009 | |
| start of this work | **269 098** (9 098 over) | `T/measure_site_budget.ts` |

Growth `HEAD` → working tree: +14 133 B gzip (+51 k minified). Per module (`T/diff_chunk_modules.ts head before`,
unminified bytes, each module gzipped alone): `▶️run` +13.1 k (clock, timed tasks, guesses — challenge levels and
adaptive layout), `🧭️session` +11.1 k, quiz `🌐️i18n` +9.0 k (EN+DE texts of challenges, clock, hints, pets),
`🧩️task` +5.6 k (guess field, hints), `🎛️preferences` +4.8 k (pets round 2 + per-quiz challenge), `🏁️results` +4.8 k,
core `⛰️challenge` +4.3 k (new), catalog JSON +3.1 k, `📖️quiz-page` +2.4 k, `🃏️matching` +2.4 k, React `⛰️challenge` +2.1 k
(new), `🐾️pets` glue +1.4 k, the rest under 2 k each. About two thirds is this ticket's code; most of it only a run shows.
Already at `HEAD` the entry carried everything a run needs; the large framework parts (react-dom ~181 k minified, the
UI i18n bundle ~89 k, the icon registry and shortcodes ~117 k, the catalog material ~80 k) are unchanged since `HEAD`.

### Decision: split, do not raise

The run and its results are not needed for the first screen, so they now come in lazy chunks (`R/🟦️.tsx`):

- `useRunScreens()` (in `Screen`) fetches `RunScreen` and `ResultsScreen` by dynamic `import()` as soon as the client shows
  its first screen, and keeps retrying a failed fetch with `retryWithJitteredBackoff` (`RETRY_TIMING`) so a short
  connection shortage only delays it; a run or its results wait on the existing loading screen while the chunk is still
  missing. No `React.lazy`/`Suspense`: once fetched, the screens render synchronously (no fallback flash, no throttled
  reveal), and the React suite needed no change.
- Shared pieces moved out of run-only modules so the entry does not pin them: `TaskGlyph`, `TASK_KIND_ICONS` (from
  `▶️run` → `🪟️chrome`, used by the quiz page and the results) and `Announcement`, `useAnnouncement`, `LiveRegion` (from
  `🧩️task` → `🪟️chrome`, used by the preferences). All importers and the root re-exports changed at once.
- Module-level expressions that Rollup must assume to have side effects kept the run modules in the entry through the
  package's static re-exports: `results` built its class constants with `cn(…)` (now plain strings, the same classes
  `cn` produced), `run` has `memo(TaskView)` and `radar` five `new Set("…")` (now `/* @__PURE__ */`-annotated).

| Build | Entry script gzip | Lazy run chunk | Lazy results chunk |
|---|---|---|---|
| before | 269 098 | — | — |
| run/results lazy, `cn` + `memo` pure | 259 932 | 8 240 | 3 232 |
| + radar pure | 256 228 | 12 022 | 3 226 |
| + task/drag lazy (announcement in chrome) | 254 305 | 13 868 | 3 216 |
| after the acceptance fixes landed (final) | **254 301** (5 699 headroom) | 13 869 | 3 218 |

Stylesheet 40 764 B gzip (unchanged), no budget problem (`siteArtifactProblems` → `[]`); the rehearsal gate built and
accepted the artifact in every run below. The budget stays 260 000; `S/README.md` (deploy-check step 4) now says what it
counts, that run/results/pets are lazy, and the measured weights. The deploy test was not touched (number unchanged).

## 2. Gates

All e2e in the site package (`S/📦️packages/🟦️typescript`) unless noted; `NX_PLUGIN_NO_TIMEOUTS=true` always.

| Gate | Command | Result |
|---|---|---|
| React suite (after my split) | `bun ./📜️script.ts test` in `R/📦️packages/🟦️typescript` | 22 files, **831 passed**, no `act` warnings |
| React typecheck | `tsc --noEmit -p tsconfig.json` there | my files clean; at that moment only the pets session's in-flight core errors (`🐾️pets/🔨️modules/…`) |
| Site typecheck | `bun nx run @teaching/architecture-quiz:typecheck --skip-nx-cache` | **success** |
| Site node tests | `bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | 5 files, **228 passed** |
| Full gate | `bun nx run @teaching/architecture-quiz:test-e2e dev rehearsal` | dev 31 passed / 4 failed / 20 not run (37 min); rehearsal 30 / 5 / 20 (28 min) — see below |
| Reruns of the red ones | `bun ./📜️script.ts test-e2e rehearsal --project=desktop --no-deps challenge-levels quiz-runs` | **15/15** |
| | `… dev --project=desktop --no-deps challenge-levels both-languages` | 12 passed, 3 failed (expert hung once; both-languages 2, see below) |
| | `… dev --project=desktop --no-deps challenge-levels -g expert --repeat-each=3` | **9/9** |
| phone + layout | `… dev rehearsal --project=phone --project=layout --no-deps` | dev **6/6**, rehearsal **6/6** |
| presence, shortage, away, pets | `… dev rehearsal --project=<p> --no-deps`, one after the other | see §2.2 |
| earlier, alone | `… dev --project=desktop --no-deps challenge-levels` (12 tests, before the full gate) | **12/12**; `… dev --project=phone --no-deps` **2/2** |

### 2.1 Desktop project, by spec (best evidence per topology)

| Spec | dev | rehearsal |
|---|---|---|
| `⛰️challenge-levels` (12) | 12/12 (alone), expert 9/9 repeated | 12/12 (rerun) |
| `🎯️quiz-runs` (3) | 3/3 (full gate) | 3/3 (rerun); full gate 2/3 |
| `🪪️first-visit`, `🏆️live-leaderboard` | passed (full gate) | passed (full gate) |
| `🗣️both-languages` (3) | 3/3 (last rerun); full gate 2/3, first rerun 1/3 | passed (full gate) |
| `🥞️layered-home` (5) | 3/5 | 3/5 |

### Red, with evidence (not fixed: not mine or the host)

- **`🥞️layered-home`** "home is a grid of nine cards…" (`intro lies in the cell of column 2, row 0`) and "between the
  cards the mouse pans…" (`the pointer at 0.5, 1 pans to demand`, expected < 1.5): both topologies, every run — the
  adaptive-layout session's home-grid sizing, as in `📓️report-integration.md`.
- **`🗣️both-languages`** (dev only): once a click on the results' action timed out "waiting for scheduled navigations",
  once the `de` test counted 23 controls in one language and 47 in the other on the introduction, and once the `en`
  test failed on `POST /queries net::ERR_NO_BUFFER_SPACE` — Windows ran out of socket buffers. It passed in rehearsal
  in the full gate and 3/3 in dev once the host calmed down. Not touched by this work; the 23-vs-47 control count was
  not reproduced and is worth watching (pets controls appearing in one language only would explain it).
- **Load failures in the full gate**, all green on rerun: `⛰️challenge-levels` "hard on a matching" (dev: the identity
  step's button not clickable within 20 s), "hard on a classification" (rehearsal: `press Enter` on a guess field
  timed out), "expert …" (rehearsal: the remembered challenge poll timed out; dev rerun: the page stopped answering —
  no screenshot could be taken — right after the demand results), `🎯️quiz-runs` demand drag (rehearsal: the bin never
  lit within the 20 s retry; the integration report saw the same under load). The host was saturated (the full gate
  took 37 min; `ERR_NO_BUFFER_SPACE`).

### 2.2 The other projects (`--no-deps`, one project per invocation, both topologies per invocation)

| Project | dev | rehearsal |
|---|---|---|
| boot | 1/1 (full gate) | 1/1 (full gate) |
| phone (incl. the new hard test) | 2/2 | 2/2 |
| layout | 4/4 | 4/4 |
| presence | 1/1 | 1/1 |
| shortage | 2/2 | 2/2 |
| away | 2/2 | 2/2 |
| pets | 13/13 | 13/13 |

Totals by best evidence: dev 54 of 56 tests green, rehearsal 54 of 56; the two red in each are the `🥞️layered-home`
ones of the adaptive-layout session. A final `… dev --project=desktop --no-deps both-languages` on the calmer host:
**3/3**.

## 3. E2E breadth (audit gap 3)

`S/🧪️tests/⛰️challenge-levels/🟦️.ts` (desktop project) — eight new tests, each on its own pseudonymous learner:

1. **easy matching** (heating, `u-values`): every true card → no hint; the cards of the smallest and largest exchanged
   → `{smallest: high, largest: low}`, "Card far too large/small" beside the items; the true cards back → no hint;
   perfect run → 100 points.
2. **hard matching** (heating): no selects, one guess field per item and dimension; perfect answers, then one guess
   ×10⁴ → exactly that row a miss with "Far off", `[data-tolerance]` reads "A guess counts within ×…/±… of the true
   value.", the other task scores 100.
3. **easy classification** (demand, spider profiles): all true → no hint; one item into another category → the task's
   own hint only (`{"": "misplaced"}`, "Items in the wrong category: 1", none on an item); back → none; perfect → 100.
4. **hard classification** (demand): on medium every diagram's table has 4 columns and names `kWh/(m²·a)`; on hard
   2 columns (axis, share), shares in %, no `kWh`, `€`, `m²` anywhere in a figure, no category description; perfect
   run → 300 points.
5. **expert full** (with `page.clock.install()`): demand (classification + matching) answered in time → 400 points;
   then physics: classification and `powers` in time, `energies` started and fast-forwarded past its limit →
   confirmation names it; scores 100/100/0, 66.7 %, 266.7 of 400. (The time-out comes last: after a fast-forward the
   page is busy catching up, so nothing navigates after it.)
6. **German** (`device("de")`): chooser group "Herausforderung" with 4 radios, "Leicht/Mittel/Schwer/Experte",
   "Höchstpunktzahl: 100…400", the expert line; hint "Wert viel zu klein"; results "Leicht · Punkte: 100 von 100"; no
   unresolved label on run and results.
7. **discard dialog**: open medium run on heating with one task answered; page chooses hard → start → alertdialog
   "Discard the open run?" / "Your run on Medium is still open. Starting on Hard discards it with its answers." →
   confirm → a hard run with no task complete and hidden keys; remembered `hard` for heating, still `medium` for physics.
8. **reload on expert** (real time): `powers` clock started, one guess, ≥ 3 s pass; reload → back to the run (card
   action if the overview shows) → the clock is running (not closed, not restarted), shows no more than before the
   reload, the guess is still in its field.

`S/🧪️tests/📱️phone/🟦️.ts` (phone project, 375 × 812): **hard on a phone** — physics on hard; on every task each guess
field lies within 0…375 px and is ≥ 24 px tall, physics category descriptions absent, every task answered by typing,
no page or box scrolls sideways before and after submitting; 300 points; no unresolved label.

Driver (`S/🎭️e2e/🚶️learner/🟦️.ts`): `rememberedChallenge(device, quiz)` already read `challenges[quiz]` (acceptance
agent); I added `swapExtremes(device, task, dimension, items)` (the card exchange `answerTask` "flawed" used inline,
now shared with the easy-matching test) and `description?` on `SourceTask.categories`.

## 4. Files

Updated
- `R/🟦️.tsx` — lazy run/results screens (`RunScreens`, `runScreens`, `useRunScreens`), re-exports, module docstring.
- `M/🪟️chrome/🟦️.tsx` — `TASK_KIND_ICONS`, `TaskGlyph`, `Announcement`, `useAnnouncement`, `LiveRegion` (moved in), docstring.
- `M/▶️run/🟦️.tsx` — moved out `TaskGlyph`/`TASK_KIND_ICONS`, imports from chrome, `memo` pure.
- `M/🏁️results/🟦️.tsx` — class constants as strings, `TaskGlyph` from chrome.
- `M/🧩️task/🟦️.tsx` — moved out the announcement pieces.
- `M/📖️quiz-page/🟦️.tsx`, `M/↕️sorting/🟦️.tsx`, `M/🃏️matching/🟦️.tsx`, `M/🗂️classification/🟦️.tsx`, `M/🎛️preferences/🟦️.tsx` — imports.
- `M/🕸️radar/🟦️.tsx` — pure `Set`s.
- `S/README.md` — what the budget counts and the weights.
- `S/🎭️e2e/🚶️learner/🟦️.ts`, `S/🧪️tests/⛰️challenge-levels/🟦️.ts`, `S/🧪️tests/📱️phone/🟦️.ts`.

Created (ticket inputs): `T/measure_site_budget.ts`, `T/site_chunk_modules.vite.ts`, `T/head_variant.ts`,
`T/head_variant.vite.ts`, `T/lazy_variant.ts` (the experiment before touching sources), `T/diff_chunk_modules.ts`,
`T/attribute_site_chunks.ts` (source-map attribution; misleading for JSON, which has no mappings — superseded by
`site_chunk_modules.vite.ts`), `T/quiz_shape_probe.ts`.

Removed: `T/🗑️generated/site-final/` and my e2e run directories.

## 5. Deviations and decisions

1. **Split, not raise.** The growth is legitimate, but only a run shows it, so splitting helps the first paint; the
   budget number stays.
2. **State-based loading, not `React.lazy`.** No Suspense fallback once the chunk is there, deterministic tests, and a
   retry with jittered backoff on a failed fetch instead of an error boundary.
3. **`/* @__PURE__ */`** on `memo(TaskView)` and the five radar `Set`s: bundler annotations at module scope, not prose
   comments; there is no package-level `sideEffects` declaration available (the React target's modules lie outside its
   package directory, whose nearest `package.json` is the repo root). Without them the package's static re-exports pin
   the run modules in the entry.
4. **Announcement pieces in `🪟️chrome`**: they are generic polite-live-region parts shared by tasks, the run and the
   preferences; leaving them in `🧩️task` would keep the whole task module (guess field, grip, drag) in the entry.
5. **Phone hard run in `📱️phone`**, not in `⛰️challenge-levels`: the device fixture takes its viewport from the project,
   and the phone project is the one with the phone viewport, touch and `isMobile`; nothing new registered.
6. **Reload test in real time** (no `page.clock`): what matters is that the clock is measured from the stored opening
   instant across a new document; a fake clock would not carry over a reload faithfully.
7. **No new React unit test for the chunk loading**: the import is internal; the 831 React tests drive every screen
   through it, and both e2e topologies (dev server modules and the release chunks) exercise it for real.

## 6. Notes for the next agent

- New exports from `@semio-tech/quiz-react` locations: `TaskGlyph`, `TASK_KIND_ICONS`, `Announcement`,
  `useAnnouncement`, `LiveRegion` now come from `M/🪟️chrome` (the package API is unchanged).
- Keep run-only code out of entry modules: anything imported by home, the quiz page, preferences, navigation or the root
  is charged to the entry. A module-level call in a run-only module (e.g. `cn(...)`, `memo(...)`, `new Set("...")`)
  pulls it back into the entry through the package's static re-exports — annotate it pure or make it a literal.
  `T/measure_site_budget.ts <dist>` and `T/site_chunk_modules.vite.ts` show where bytes go.
- E2E driver: `swapExtremes(device, task, dimension, items)`.
