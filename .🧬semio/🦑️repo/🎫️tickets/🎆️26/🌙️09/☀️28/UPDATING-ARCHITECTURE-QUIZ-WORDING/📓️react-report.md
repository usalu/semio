# ⚛️ `@semio-tech/quiz-react` — web renderer and proctor client (report)

Work package: design §10 client, §12 renderer entry, §9a wire mapping. Layout per the coordinator's correction
(🖱️ui pattern): implementation at `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/`, package glue only under
`…/🎯️targets/⚛️react/📦️packages/🟦️typescript/`. Domain-neutral: nothing knows a concrete catalog. The audit fixes
are in [Fixes](#fixes).

## Status

- **Tests: 107/107 green** (6 files), stable over repeated runs, and green through `bun ./📜️script.ts test` and
  `bun nx run @semio-tech/quiz-react:test`. The site's own `@teaching/architecture-quiz:test` passes too (11/11).
- **Type check: zero errors in quiz-react, the quiz core and the new ui-react i18n module.** `bun ./📜️script.ts
  typecheck` still exits 1 because of 47 errors in `🧰️framework/🔨️modules/…`, not in quiz-react. The framework's own
  typecheck reports the same errors in the same files, so they predate this work. `--noUnusedLocals
  --noUnusedParameters` finds nothing unused in quiz-react.
- **Runtime:** two checks in the site dev server at 375 px, with the dev proctor down and then up (item 7 in
  [Fixes](#fixes)), one against the coordinator's walk-through learner for items 9 and 10, and the spider diagrams of
  the Energy Demand quiz at 375, 596, 768 and 1280 px (item 11). I did not create
  learners or submit forms against the dev proctor. The full flow,
  several tabs, cancellation and cross-device staleness are verified in jsdom against a proctor double.

## Files

Implementation (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/`):

| Path | Purpose |
|---|---|
| `🟦️.tsx` | Entry: `mountQuiz`, `QuizApp`, app shell (skip link, header nav, connection status, language switch, settings, notices, one connection-aware waiting panel, focus moves to each screen's `h1`), explicit re-exports of every module's public API |
| `🎨️.css` | All styling: `--quiz-*` tokens derived from the semio palette `--color-*`/`--font-*` (with the palette values as fallbacks), light/dark via `data-theme`, text size via `--quiz-text-scale`, desktop first plus breakpoints at 60em and 40em, wrapping and hyphenation for long text, targets at least 2.75em, WCAG-checked contrasts |
| `🔨️modules/🛂️proctor/🟦️.ts` | `ProctorClient` over `ServerClient`: commands and queries per §9a, `newId` (`crypto.getRandomValues`), `proctorTransport` (on `fetchWithTimeout`, abortable, non-JSON error pages become typed errors), `isTransient` / `isNotFound`, `retryTransient` (on `retryWithJitteredBackoff`), `quizRejection` (reads `Rejection::Invalid{detail}` the way the proctor encodes it), and reachability tracking |
| `🔨️modules/📮️outbox/🟦️.ts` | `Outbox` (persisted local-only, **one record per command**, shared by every tab). Holds the union of all tabs' queues; per (run, task) only the latest stays, never touching the command on the wire. Delivers in order with jittered backoff under the original command id and skips a record another tab already delivered or superseded. Status for the UI, `settled(run, signal, progress)`, `wake`, `discard`, `forget`, `coalesce` |
| `🔨️modules/💾️persistence/🟦️.ts` | Owned `StorageArea` seam (`get`, `set`, `remove`, `keys`, and `watch` for other tabs' changes) with `browserStorageArea()` over `localStorage` plus its `storage` event, and `memoryStorageOrigin()` for tests. `LocalStore` has single-valued slices `semio.quiz.<tenant>.<slice>` and **per-record collections** `semio.quiz.<tenant>.<collection>/<id>` (`outbox`, `runs`). Degrades to "nothing stored" when storage is unavailable, full or corrupt |
| `🔨️modules/🧭️session/🟦️.ts` | Event-driven client state: pure `evolveQuizState` over `QuizClientEvent`s, `restoreQuizState`, `mergeRunViews`, and the `QuizSession` controller (external store for `useSyncExternalStore`). Handles identify, start/resume, answer, submit (with phases, cancel and post-cancel reconciliation), `reconnect`, `latestWins` refreshes, projection-lag tolerance, forgetting an unknown learner, and adopting other tabs' stored changes without writing them back |
| `🔨️modules/🌐️i18n/🟦️.ts` | `QUIZ_BUNDLE_EN` / `QUIZ_BUNDLE_DE` (169 keys each; German uses informal *du*), registered through **`@semio-tech/ui-react/i18n`**. `quizText(locale)` works through `uiI18n.t(…, { lng })` and `resolveUiLabel`. Also `preferredLocale`, `applyLocale` (`setUiLocale`), `REJECTION_LABELS`, `TASK_KIND_LABELS` |
| `🔨️modules/📏️quantity/🟦️.ts` | `formatQuantity` with SI prefixes q…Q and a four-significant-digit mantissa in [1, 1000), rounding the shortest round-trip decimal half-expand exactly like ICU. `formatScore` / `formatPoints` show whole values whole and everything else with one decimal through `oneDecimal`, so only a perfect score reads 100 % and only zero reads 0 %. Also `formatNumber`, `formatInstant`, `formatDate`, `withUnit` (no-break space; none before °, ′, ″) |
| `🔨️modules/🕸️radar/🟦️.tsx` | Geometry (`radarFraction` clamped, `radarAngle` with 12 o'clock clockwise, `radarPoint`, `radarPolygon`). Label layout: `radarLayout` (wrapped labels, circle sized and shifted for them, numbered legend when cramped), `wrapLabel`, `textMeasure` (canvas), `estimateTextWidth`, `RADAR_METRICS`, `RADAR_LABEL_EM`. `RadarChart`: one `role="img"` SVG named by `<title>`, laid out for its measured width at one user unit per pixel, plus a `<details>` value table with value, min and max |
| `🔨️modules/🤏️drag/🟦️.ts` | Pointer drag and drop through the Pointer Events API (mouse, pen, touch). Uses `data-quiz-drop` zones, an inert ghost, highlighting and Escape to cancel |
| `🔨️modules/🧩️task/🟦️.tsx` | Shared task pieces: `TaskViewProps`, polite `LiveRegion`, `useFocusAfterRender`, the `DragGrip` hidden from assistive technology |
| `🔨️modules/🗂️classification/🟦️.tsx` | Pool and category bins with a category `<select>` per item (focus follows the moved item), drag onto bins or the pool, and spider diagrams for categories that have profiles |
| `🔨️modules/↕️sorting/🟦️.tsx` | Ordered list with move up/down buttons (`aria-disabled` at the ends so they stay focusable), focus follows the item, drag onto an item, and an explicit "keep this order" |
| `🔨️modules/🃏️matching/🟦️.tsx` | Per dimension: a card pool showing which card is used by which item, a `<select>` per item (choosing a used card takes it over, so each card is used once), an unassign button, and drag from card to row |
| `🔨️modules/👋️introduction`, `🪪️identity`, `🏠️home`, `▶️run`, `🏁️results`, `🏆️leaderboard`, `🎛️preferences` | The screens of §10 (details below) and the display preferences |

Package glue (`…/🎯️targets/⚛️react/📦️packages/🟦️typescript/`):
- `package.json`: `@semio-tech/quiz-react`, `bundleKind: "ui"`, exports `.` → `./🟦️.tsx` and `./🎨️.css` → `../../🎨️.css`.
- `📋️project.json`: targets `test`, `test-quick`, `test-long`, `test-exhaustive` and `typecheck`.
- `📜️script.ts`: `test` runs vitest with `quick` as the minimum level (the fundamental 15 s budget is too short for the jsdom cold start). `typecheck` runs `bun …/typescript/bin/tsc`, because `runBunx(["tsc"])` fails on native Windows (bun tries to run `.bin/tsc` as a shell script).
- `tsconfig.json`: extends the root config and maps the workspace packages, including `@semio-tech/ui-react/i18n`, to their sources.
- `🟦️.tsx`: `export * from "../../🟦️.tsx"`.

Tests and fixtures:
- `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (jsdom, aliases, processes the quiz stylesheet so tests can read it, reuses `🖱️ui/🧪️tests/🧹️react-environment` as its setup file).
- `🧰️framework/🛍️products/❓️quiz/🧪️tests/{📐️quantity-formatting,🕷️radar-geometry,📬️outbox-delivery,⌨️task-keyboard,🚶️learner-journey,🗣️translation-completeness}/🟦️.tsx`.
- `🧰️framework/🛍️products/❓️quiz/🧫️fixtures/{📐️quantity-formatting,🕷️radar-geometry}/🔣️.json` (language-agnostic vectors).

Shared files edited:
- The root `package.json` workspaces now include `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript`. `bun install` exited 0.
- For the bundle fix: ui-react's barrel, its `package.json` exports, `🧱️elements/📚️I18n` and `🧱️elements/🏷️Label`, plus one alias line in the site's `🏗️builder/🌐️vite/🟦️.ts`. See Fixes item 5.

## Public API (entry)

- `mountQuiz(root: HTMLElement, options: QuizOptions): () => void` renders `<StrictMode><QuizApp/></StrictMode>` and
  returns the unmount function.
- `QuizApp(options: QuizOptions)`. `QuizOptions = { proctor: string; tenant: string; transport?: ProctorConnect;
  storage?: StorageArea; languages?: readonly string[]; timing?: RetryTiming }`. `proctor: ""` means same origin. The
  optional fields are test seams.
- Everything else is re-exported by name from the entry: formatting, radar, persistence, proctor, outbox, session,
  preferences, drag, task views, screens, i18n bundles, and `connectionMessage` / `waitingMessage`. No external type
  leaks.

## Behaviour by requirement

- **Steps:**
  - Introduction on the first visit, showing `catalog.introduction`, and later reachable from the nav.
  - Identity: anonymous, pseudonym or name, with a hint for each. It states that no password is needed, that a known
    handle loads the existing progress on any device, and warns that anyone typing the same handle continues it.
    Handles are NFC-normalised and checked with the core `normalizeHandle`.
  - Home: each quiz with its task count, best score, and start / resume / start again / last result, plus all badges
    shown as earned (with date) or not yet earned (grayscale, text state), each with its description.
  - Run: task nav in sheet order with per-task ✓/○ completeness (core `answerComplete`), a progress bar and text, and
    prev/next. Submit is `disabled` until every task is complete. It opens a confirmation alertdialog (focus trap,
    Escape); the submission then shows phase progress (saving answers k/n → submitting → loading results) with Cancel.
    A cancelled submission is reconciled with the proctor.
  - Results: only once `status === "submitted"`. The header shows the percentage only (points belong to the totals).
    Shows the per-task score and a table of the learner's answer against
    the solution, including true values with units, correct categories and positions, the correct order, explanations,
    newly earned badges, and verdicts in words and symbols.
  - Leaderboard: a large table sortable on every column with `aria-sort`. The description sits outside the horizontal
    scroller, so it wraps. Anonymous learners appear as "Anonymous #<tag>" / "Anonym #<tag>". The own row (matched by
    `learnerTag(own id)`) has `aria-current` and "(you)". It polls every 10 s while visible, refreshes when the page
    becomes visible again, and uses `latestWins`.
- **Local-first slices:**

  | Slice | Contents |
  |---|---|
  | Persisted local-only, shared by the site's tabs | `introduced`, `learner`, `catalog`, `learner-view`, `preferences` (single values); `runs/<run id>`, `outbox/<command id>` (one record each) |
  | Persisted shared | Proctor events, only through commands |
  | Ephemeral shared | Leaderboard |
  | Ephemeral local-only | Step, notices, awards, drag, focus |

  Answers apply locally at once. A loaded run view comes from the proctor except for answers still in the outbox. The
  learner view's run summaries close cached runs (submitted or voided) and drop their undelivered answers, on restore
  and on every learner-view refresh.
- **Connection:** the header shows "Connecting to the quiz server…" until the first answer, then "All answers saved",
  "Saving answers (n waiting)", "Connection lost – retrying (n answers kept on this device)" or "Offline – …". Screens
  whose data has not arrived show one waiting panel with the same state and a "Try again now" when unreachable or
  refused.
- **i18n:** no default. An explicit choice is persisted; otherwise the first `en`/`de` entry in `navigator.languages`,
  and English when neither is listed. `lang` is set on the root and on the document. German addresses the learner as
  *du*.
- **Customization:** theme (system, light, dark) and text size (1, 1.125, 1.25, 1.5×), persisted locally and shared
  across tabs.

## Tests (what each proves, and the oracles)

| Case | Covers |
|---|---|
| 📐️quantity-formatting | 15 shared quantity vectors, 7 score and 3 points vectors in en and de. **Oracle:** `Intl.NumberFormat` engineering notation's `formatToParts` over 2,808 values (54 decades × 13 mantissas × ±, in en and de). Unprefixed, percent (`maximumFractionDigits: 1`) and points formatting against `Intl.NumberFormat`. A sweep over 0.0001…0.9999 in both locales: never "100 %", never "0 %" |
| 🕷️radar-geometry | Shared vectors (inside and clamped). **Oracle:** `d3-scale` `scaleLinear().clamp(true)` for 3 to 9 axes (252 points). Angle convention. Rendering checks the named `img` and the value table in German. **Labels:** 13 shared wrap vectors and 10 shared layout cases. A sweep over 101 widths × 7 label sets × 2 text sizes × 2 measures (2,828 layouts) checks that every label box lies inside the view box, no line is wider than its box, labels clear the circle and each other, and the font size never scales. Also 3 to 12 spokes, the legend fallback, and rendering at a measured width in both modes. **Oracle:** the brand font Anta's real advance widths and GPOS kerning read with `opentype.js` |
| 📬️outbox-delivery | Coalescing without touching the command on the wire. Latest-only while queued. Retry through 503 under one id. Exactly-once with a deduplicating proctor double that loses a response. Persist and reload. Verdicts, `settled` progress and abort. **Two tabs** each keep their own record and survive a reload. **Cross-tab coalescing:** the newest answer wins and a delivered command is never sent again. **Discarding a closed run** retires the answer in flight (never retried nor restored) and drops the queued ones, keeping other runs'. §9a envelope mapping, `quizRejection`, id format |
| ⌨️task-keyboard | For each kind: Tab order, select, Enter and Space, focus kept on the moved item, live-region text, `answerComplete` transitions. Pointer drag and drop for all three kinds |
| 🚶️learner-journey | The full journey against the core-decider proctor double (projection lag, outage, confirm, results, tag-based leaderboard, reload). Also: the unknown-learner fallback, **two tabs** of one learner keeping both tabs' unsynced answers, **cancel race** reconciliation, **proctor wins over delivered answers** with only undelivered ones on top, **`open()` refreshes cached runs**, **another device's submission closes the cached run on home** (learner-view refresh and restore, stale answers dropped, card shows "Start again" / "View last result"), the **anonymous tag name**, **first paint** while unreachable (state plus retry) and before the first answer ("Connecting", never "Connected"), and the **phone wrapping** CSS |
| 🗣️translation-completeness | Identical key sets in en and de, non-empty labels, identical placeholders, every key resolved through the shared port in both locales, coverage of the core `REJECTIONS` and `TASK_KINDS`, every key used by the sources, **informal German**, and the **`@semio-tech/ui-react/i18n`-only import** guard with `resolveUiLabel` semantics |

## Commands run and results

| Command | Result |
|---|---|
| `bun install` | exit 0 |
| `bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts` | 107 passed, repeated runs green |
| `bun ./📜️script.ts test` | 6 files, 107 tests passed |
| `bun nx run @semio-tech/quiz-react:test --skip-nx-cache` | 107 passed |
| `bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | 11 passed |
| `bun ./📜️script.ts typecheck` | 0 errors in quiz-react, the quiz core and the ui-react i18n module; 47 pre-existing framework errors |
| `tsc -p` ui-react `tsconfig.json` | 0 errors in the ui-react barrel, `🌐️i18n`, `📚️I18n` and `🏷️Label`; 49 pre-existing errors elsewhere |
| `bun ./📜️script.ts test quick` (ui-react package) | 805 passed, 42 failed (see Fixes item 5) |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🛍️products/❓️quiz` | `clean=true` |
| `bun ./📜️script.ts verify taxonomy report --scope …/🖱️ui/🎯️targets/⚛️react` | The new `🌐️i18n` resolves; 2 pre-existing errors (`📜️script.ts` disposition, `🧹️lint` directory) |
| `bun nx run @teaching/architecture-quiz:build --skip-nx-cache` | Before 1,691 kB JS; after 414 kB JS (see Fixes item 5) |

## Fixes

Audit round 1 (coordinator, items 1–4), round 2 (`📓️site-infra-report.md` §3, items 5–8) and round 3 (the
coordinator's browser walk, items 9–11). Each has tests; all 107 pass.

1. **CRITICAL: multi-tab data loss.**
   - *Cause:* the outbox and the run cache were persisted as whole-slice values, so each tab overwrote the others'
     unsynced answers.
   - *Fix, storage:* a new owned `StorageArea` seam with key enumeration and a `watch` for other tabs' changes (the
     `storage` event). The collections are stored one key per record: `outbox/<command id>` and `runs/<run id>`. A
     tab's write never replaces another tab's record.
   - *Fix, outbox:* every tab holds the union of all records in queued-at order. Each tab's queued-at stamps are
     strictly increasing and later than every command it knows of. Per (run, task) only the latest stays; superseded
     records are dropped, never the command on the wire. Before each attempt a tab checks that the record still exists,
     so a command another tab delivered or superseded is not sent again. The same command sent from two tabs is safe
     because it is idempotent by command id.
   - *Fix, session:* other tabs' learner, catalog, learner-view and run changes are adopted without writing them back,
     which rules out ping-pong between tabs. Run records merge through `mergeRunViews`: a closed status wins, and open
     runs take the union of answers. Undelivered answers from the union outbox are overlaid again whenever the outbox
     changes and when a tab restores.
   - *Tests:* the outbox "two tabs … never overwrite each other" and "cross-tab coalescing" cases, and the journey
     "keeps both tabs' unsynced answers…" case (two sessions, outage, reload, both delivered).
2. **HIGH: cancel race.** `submit` now reconciles every cancelled submission: it reads the run view again and adopts
   `submitted` and its result if the proctor had committed. A run that closes while the learner is on it is followed
   to its results (submitted) or home with a notice (voided); this also covers another tab submitting. *Test:* the
   proctor double commits the submission and holds back its answer, the learner cancels, and results appear.
3. **MEDIUM: cross-device staleness.** `loadRun` lets the proctor's view win and overlays only answers still waiting in
   the outbox. `open()` refreshes a cached run or results screen from the proctor every time, and the cached view
   still shows at once, also offline. *Tests:* another device's answer replaces a delivered local one while an
   undelivered local one stays on top, and opening a cached run queries the proctor and adopts its answers.
4. **Leaderboard tag (confirmed schema change).** Rows are keyed by `tag`. The own row is `row.tag ===
   learnerTag(own id)` (core export). Anonymous learners are named "Anonymous #<tag>" / "Anonym #<tag>", also in the
   header and on home, so no part of an id (a credential) is ever displayed. *Tests:* the journey leaderboard and the
   tag-name case.
5. **Bundle weight.** The i18n port region of ui-react (locale storage, the chrome bundles, `uiI18n`, registration,
   shell instances and locale resolution) moved out of the 11,800-line barrel into
   `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`.
   - *How it was moved:* by the script `extract_ui_i18n_port.mjs` in this folder, with exact marker lines and a
     re-check that the file was unchanged before writing. The barrel imports and re-exports everything unchanged.
     `resolveTranslationLabel`'s pure core became `resolveUiLabel(value, tier)` in `🧱️elements/📚️I18n`, and `Label`
     delegates to it.
   - *New subpath:* `@semio-tech/ui-react/i18n` (`package.json` export), with aliases in quiz-react's test config and
     tsconfig and in the site's Vite config. quiz-react now imports only that subpath; a test guards it.
   - *Sizes* (`bun nx run @teaching/architecture-quiz:build`):

     | | Before | After |
     |---|---|---|
     | Modules | 760 | 160 |
     | Main JS | 1,691.39 kB (gzip ≈ 370 kB) | 413.97 kB (gzip ≈ 122 kB) |
     | CSS | 36.52 kB | 20.92 kB |
     | `pdf.worker` | 1,232.30 kB emitted | gone |
     | `react-dom/server` (`renderToStaticMarkup`) | inside the chunk | gone (only `react-dom/client` remains) |

   - *ui-react tests* (`bun ./📜️script.ts test quick` in its package): 805 passed, 42 failed. None of the failures
     involve i18n; they are Tree windowing, shell layout (`NaNpx`, folded search pane), CSS tokens missing (`no :root
     block containing --base`) and Playwright missing its browser. The i18n tests in that suite pass, and rendered
     chrome resolves its translations (e.g. "Window tabs"). The plain `test` run at fundamental level is additionally
     killed by its 15 s budget. The ui-react type check has no errors in any file I touched.
6. **German register.** Every German string addresses the learner with *du* (26 phrases changed; e.g. "Wie möchtest du
   erscheinen?", "Deine Antwort", "Bitte gib 1 bis 64 Zeichen ein."). *Test:* no *Sie/Ihr/Ihnen* form in any German
   label.
7. **Phones and first paint.**
   - *Phone wrapping:* at 375 px long texts wrap: `overflow-wrap` and `hyphens: auto` on the app,
     `overflow-wrap: anywhere` on text elements, `min-width: 0` for flex children. The leaderboard description is now a
     paragraph outside the horizontal table scroller; the table is named by the heading and described by that
     paragraph. *Tests:* the stylesheet rules, and a DOM check that the description is outside the scroller.
   - *First paint:* reachability "unknown" now reads "Connecting to the quiz server…", never "Connected". Screens
     without their data show one waiting panel that says connecting, "cannot be reached – trying again" with "Try again
     now" (`session.reconnect()` restarts the loads at once), refused, or loading. The static placeholder and the
     "Waiting…" + "✓ Connected" flicker are gone. *Tests:* the journey "shows the connection state on a first visit…"
     and "says it is connecting — never connected…" cases.
   - *Runtime, proctor down:* the site at 375 px shows "Loading… / The quiz server cannot be reached right now – trying
     again. / Try again now" with "⚠ Connection lost – retrying" in the header, in brand fonts.
   - *Runtime, proctor up:* home and the leaderboard render at 375 px from the real proctor. The leaderboard
     description wraps, and the placeholder learner shows as "Anonymous #<tag>". The unknown placeholder learner is
     forgotten after the grace period with the informal German notice. The German identity texts wrap. No console
     errors.
8. **Ties.** Unchanged, as asked: the strict ordering (total, badges, reachedAt, id) comes from the proctor.
9. **Score display rounded a non-perfect score up.**
   - *Cause:* `formatScore` used `Intl.NumberFormat` percent with no fraction digits, so 0.997 read "100 %". The
     results header also showed the same number twice, as a percentage and as points.
   - *Fix:* `oneDecimal(value)` keeps whole values whole (float noise below 10⁻⁹ counts as whole) and rounds the rest
     to one decimal. When that rounding lands on a whole number, it steps back by 0.1 towards the true value: 99.96
     becomes 99.9, never 100, and 0.04 becomes 0.1, never 0. `formatScore` and `formatPoints` format that value with
     `maximumFractionDigits: 1`, so the output is locale-aware: "99.7%" / "99,7 %", "12.5%" / "12,5 %", "100%" only for
     1. The results header now shows only "Your score: …"; points stay in the home and leaderboard totals. The key
     `quiz.results.points` is removed (169 keys per locale).
   - *Tests:* the fixture `🧫️fixtures/📐️quantity-formatting/🔣️.json` gains `scores` (perfect, 0.997, 0.9996 → 99.9,
     0.29 → 29 despite float noise, 0.125, 0.0004 → 0.1, 0) and `points` (300, 299.7, 399.96 → 399.9) in en and de. A
     sweep over every 0.0001 step from 0.0001 to 0.9999 in both locales asserts neither "100 %" nor "0 %". The Intl
     oracle uses `maximumFractionDigits: 1`, and the journey asserts that the header reads exactly "Your score: 100%".
   - *Runtime:* the coordinator's walk-through learner in the dev site (German) shows "Beste Wertung: 99,7 %" on home,
     "Gesamt: 99,7 Punkte", and on the results screen only "Deine Wertung: 99,7 %", with task scores "99,5 %" and
     "99,3 %".
10. **Cross-device staleness on home.**
    - *Cause:* cached `runs/<id>` records were only reconciled when a run or results screen was opened. So home kept
      offering "Resume" for a run another device had submitted, and its undelivered answers stayed queued.
    - *Fix:* the `learner-loaded` event now reconciles in `evolveQuizState`. Every cached run that is open locally but
      submitted or voided in the learner view's run summaries takes that status (and `submittedAt`). `adoptLearnerView`
      (learner refresh, run start, another tab's stored learner view) then follows each closure with `followClosure`:
      it drops the run's outbox entries, moves off a run screen that closed, and raises the voided notice.
    - *Fix, restore:* `restored()` does the same against the cached learner view, so the card is right at first paint
      and offline. It persists the reconciled run records and drops the closed runs' outbox records.
    - *Fix, outbox:* `discard` / `forget` now also retire an answer that is already in flight. Its record is dropped
      at once, and a failed attempt is not retried; it finishes only if that attempt succeeds.
    - *Tests:* the journey "closes a cached run another device submitted…" case:
      - Device A queues an answer during an outage and closes. Device B recalls the same handle, answers, submits.
      - Device A reloads while the outage continues. The run becomes `submitted` through the learner-view refresh,
        `openRunOf` is gone, `lastSubmittedRunOf` is the run, and the outbox is empty.
      - A session restored from a store whose run record says open and whose outbox holds a stale answer, while the
        cached learner view says submitted, is correct synchronously. Its outbox is empty, and the stored record is
        persisted as submitted.
      - `QuizApp` on that device shows "Start again" / "View last result" and neither "Resume quiz" nor "In progress".
      - The outbox case "drops a closed run's answers, retiring the one in flight…".
    - *Mutation checks* (temporarily, then restored byte-identical):
      - Without the evolve reconciliation, the test fails with `'open'` instead of `'submitted'`.
      - Without the discard in `restored()`, it fails with a stale answer left in the outbox.
      - Without the persist, it fails with the stored record still `'open'`.
    - *Runtime:* in the dev site I set the walk-through learner's cached run record and cached learner-view summary
      back to `open`, then reloaded. A `[DEBUG]` log showed `d7e0e7 open → submitted`, outbox 0. The Heizen card showed
      "Erneut starten" / "Letztes Ergebnis ansehen" with no click. The original records were then put back and the
      debug log removed.
11. **Spider diagram axis labels clipped** (Energy Demand quiz, German, ~600 px).
    - *Cause:* a fixed 240 × 240 view box with single-line `<text>` labels anchored at the spoke ends. Content labels
      of up to 82 characters ran past the view box, and the SVG scaled with its container, so the text shrank too.
    - *Fix, layout:* `radarLayout(labels, width, fontSize, measure)` lays out a diagram for the width it gets.
      - Proportions are in label font sizes (`RADAR_METRICS`).
      - The circle's radius stays within 3 to 7.5 font sizes. It shrinks to 2 when that keeps a side label's longest
        word whole.
      - The circle shifts sideways so the side with the longer labels gets the room it needs.
      - Labels wrap into the room on their side (twelve and six o'clock use the whole width), stack outward from the
        spoke end, stay clear of the circle, and are pushed apart vertically, away from the centre, if they overlap.
      - The height grows to hold everything.
    - *Fix, wrapping* (`wrapLabel`):
      - Lines break at white space, then after hyphens.
      - A word is split only when it alone is too wide, with at least 3 characters carried over. The split moves back
        up to 4 characters to just before a consonant that starts a syllable ("Lüftungswärmever-/lust",
        "Wärmerückgewin-/nung").
    - *Fix, fallback:* when even a small circle leaves a side label too little room for its longest word (large text
      on a phone), the spokes carry numbers and the labels form a numbered legend below the diagram, across the whole
      width.
    - *Fix, rendering:*
      - `RadarChart` measures its figure with a `ResizeObserver`. A hidden 10 em ruler catches text-size changes, and
        `document.fonts` loads trigger a new layout.
      - Text is measured with an `OffscreenCanvas` in the computed font. Where no canvas exists (tests, the first
        pass), a conservative per-character estimate is used; jsdom falls back to 18 em.
      - The SVG is one user unit per CSS pixel, so labels stay at 0.85 em of the surrounding text, with one `<tspan>`
        per line.
      - The `<title>` name and the value table (the text alternative) are unchanged.
    - *Tests* (🕷️radar-geometry, now 37 cases), with new sections in `🧫️fixtures/🕷️radar-geometry/🔣️.json`:
      - `wraps`: 13 language-agnostic vectors under a monospace measure (hyphen breaks, splits, the 3-character tail,
        syllables, astral characters, white space, no vowels).
      - `layouts`: 10 cases, including the demand labels in de and en at 287, 309 and 512 px, large text, 5 and 9
        spokes, long compounds, empty and short labels.
      - A property checker ensures that every box (spoke labels, legend keys and texts) lies inside the view box, no
        line is wider than its box, spoke labels clear the circle by half a gap, no two boxes overlap, the anchors are
        consistent, the radius stays within bounds and the font size never scales. It runs on every shared case,
        on 3 to 12 spokes, and on a sweep of 101 widths (200–1400 px) × 7 label sets × 13.6 and 20.4 px × the estimate
        and the brand font — 2,828 layouts.
      - **Oracle:** the brand font Anta (`🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/*/🔤️outline.ttf`, imported
        with `?inline`) read with `opentype.js`, including GPOS kerning and subset fallback. The estimate is never
        narrower than Anta for any printable Latin-1 character or any fixture text, and at most a third wider for texts
        over 8 characters. Lines
        laid out from the estimate measure no wider in Anta than their boxes. With Anta the demand labels keep every
        word whole at 287, 309 and 512 px.
      - Rendering, with the element sizes stubbed: the view box equals the measured width, the font size follows the
        measured em, and every `<tspan>` lies inside, both with labels at the spokes and in the legend.
    - *Mutation checks* (each temporary; restored byte-identical). Removing the circle clearance, the separation,
      the wrapping, the sideways shift, the word-keeping radius, the 3-character tail or the legend fallback each
      fails at least one test.
    - *Two bugs the tests found along the way:*
      - Floating-point rounding split "Kühlbedarf" when its room equalled its width. It is now compared with a 10⁻⁶
        tolerance.
      - The estimate undershot "¾", "Æ", "æ" and "ð" in Anta. The calibration script `quiz_react_anta_metrics.ts` in
        this folder now reports that no character is underestimated.
    - *Runtime* (dev site, real stylesheet and fonts):
      - The real `RadarChart` was mounted with the six Energy Demand profiles inside the task layout's own classes
        (`quiz-run-layout` › `quiz-task` › `quiz-bins` › `quiz-bin`), without starting a run on the dev proctor.
      - For every label, `getBBox()` was measured against the view box. At 375 (mobile), 596 (the pane), 768 (tablet)
        and 1280 px, all 6 charts have every label inside, no overlaps, a scale of 1.000–1.002 and 13.6 px labels.
      - At 596 px (radius 102) the long left label takes 6 lines and the bottom label 2. At 375 px (radius 39) the
        left label takes 8 lines with "Energiekosten" whole.
      - At 375 px with 1.5× text, the numbered legend takes over (radius 91, 20.4 px text).
      - No console errors. The viewport was reset to desktop and the page reloaded.
    - *Glue:* `opentype.js@1.3.4` and `@types/opentype.js@1.3.9` are dev dependencies of quiz-react (test oracle
      only). `bun install` exited 0.

Temporary ticket scripts kept as inputs: `extract_ui_i18n_port.mjs`, `quiz_react_german_du.mjs`,
`quiz_react_anta_metrics.ts` and `quiz_react_nbsp_escapes.mjs`. The last one restored visible `\u00a0` escapes that my file-writing tool had turned into
invisible literal no-break spaces.

## Open issues and handoffs

1. **Framework type errors** (47, in `🖱️ui/🧬️contract/🧵️retained/…`, `🛂️manifest`, `🎭️actor/🤖️generated`, the theme)
   make `:typecheck` exit 1 until their owners fix them. The target will then pass with no changes here.
2. **ui-react suite:** its owners should look at the 42 unrelated failures and the fundamental-level budget (Fixes
   item 5).
3. **`./🎨️.css` export** points outside the package dir, like ui-react's subpath exports (and now `./i18n`). The entry
   already imports the stylesheet.
4. **Registrations owned by others:**
   - Taxonomy is done for quiz-react (`members-of-modules`, `members-of-tests`, `members-of-fixtures`). The new
     `🖱️ui/🎯️targets/⚛️react/🌐️i18n` resolves without a new entry.
   - Oracle registry (conformance): `d3-scale@4.0.2` and `opentype.js@1.3.4` (with `@types/opentype.js@1.3.9`) are
     test-only oracles in `🧪️tests/🕷️radar-geometry`.
   - Launch rows (site-infra): quiz-react `test` and `typecheck`.
5. **Browser pass:** the site agent should still drive identify → run → submit → leaderboard with the proctor, across
   two tabs, and at 768 and 1280 px, including a screen reader smoke test of the live regions and the dialog.

## 2026-09-29 — card grid in the play/demonstrator language (appended by the coordinator; the agent could not write .md)

- **Shared card** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx`: `WindowChrome` at `level="dialog"`
  (never active), full-width title cap chip, action chips as real buttons ≥ 24 px (`OverviewCardAction`),
  `<section aria-labelledby>` or whole-card button; story + component test (3 cases); registered in the ui-react barrel
  and test config. `🏢️semio-tech/🎡️play/⚛️play-card.tsx` and `♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx` are
  thin wrappers (slots and data attributes unchanged, duplicate CSS removed).
- **Slim subpath** `@semio-tech/ui-react/chrome` (`🎯️targets/⚛️react/🪟️chrome/🟦️.ts`): `WindowChrome`, appearance,
  dom-event binding, `uiSpacingLen`, form-control defaults moved out of the barrel and re-exported unchanged; ports
  `setThemedIconCatalog` and `setNavbarTrailingChrome` decouple the barrel; `Navbar` gained `label`.
- **Quiz look and feel**: every screen built from `QuizCard`s under `Navbar` + `ShellBrandLogo`, appearance via
  `useElementsSurfaceChrome`, text size → root font size, site CSS = ui styles chain + `@source` of the quiz target.
- **Home grid**: learner, quiz 1, how it works, quiz 2, leaderboard, quiz 3, badges, quiz 4, preferences; 3 × 3 from
  1024 px (larger centre), 2 columns from 768 px (leaderboard spans), 1 column below; quiz cards with emoji (text
  presentation), task count, best, run state, Start/Resume/Start again + Last result; leaderboard card top 5 + gap +
  own row (`aria-current`). Header cells and numbers `quiz-nowrap`; tables scroll inside cards.
- **Checks**: quiz tests 133/133 before presence; `🏠️home-grid` case (order/excerpt vectors, nine sections with the
  leaderboard fifth, actions per state, `aria-current`, nowrap, breakpoints, column counts 320–1440 px via lightningcss
  oracle `quiz-react-lightningcss`); OverviewCard 3/3; mutation checks 10/10 killed; ui-react 835 pass / 22 fail
  (pre-existing: tab tokens, Tree gutter, Window engagement, assets `shortcodeEmoji`, ShellScope, Panel, Layout);
  play tests 72 pass / 3 fail (unrelated); demonstrator 20 pass (branding test needs `@semio-tech/flow-core` wasm);
  play nx build blocked by rustc `STATUS_NO_MEMORY` while compiling wasm prerequisites. Browser walk at
  1440/1280/1024/768/375 light and 1440/768/375 dark: home columns 3/3/3/2/1, 0 overflow, 0 console errors.
- **Bundle** (`@teaching/architecture-quiz:build`): before 421.12 kB JS / 21.02 kB CSS; card grid 592.25 kB / 246.33 kB;
  card grid + presence 613.75 kB (gzip 166,639 B) / 247.26 kB (gzip 36,572 B). Largest parts: icon table ~86 kB,
  tokens 28 kB, WindowChrome 24 kB, appearance 13.6 kB, silhouette 11.6 kB, `cn` 10.8 kB. Remaining lever: per-icon
  modules in `@semio-tech/assets`.

## 2026-09-29 — shared presence and cursors (appended by the coordinator)

- **`🔨️modules/👥️presence/🟦️.tsx`**: `PresenceRoom` (one `semio.presence.v1` socket; admits member states through the
  core's `presenceProblem`/`cursorProblem`; keeps refusals without closing; sends the latest state at most every 67 ms,
  never repeating the last sent; after loss waits `random × minMs`, rejoins with `retryWithJitteredBackoff`, resends
  the latest state); `QuizPresence` (roster room `<catalog>` + room of the current place; `presencePlace` maps
  run/results to `<catalog>/quiz/<quiz>` with the run's task; identity and badges have no room, badges counts as home);
  React glue `usePresencePointer` (relative to the nearest `data-presence-anchor`, 10⁻⁴ precision, nothing while a
  button is pressed, focus only when moved by keyboard), `PresenceOverlay` (`aria-hidden`, 120 ms glide unless reduced
  motion), one colour per learner via `presencePaint`.
- **Anchors are cards only**: `home:*`, `leaderboard`, `introduction`, `run`, `task:<id>`, `results`, `result:<task>`;
  items, cards and drags are never shared.
- **Text for everyone**: navbar "Online: N", learner card "Who is where" disclosure, quiz cards "Learning now: N",
  leaderboard online marks; preference "Show others' cursors" (default on, local only); en/de (du).
- **Checks**: quiz tests 165/165; `👥️presence-client` 32 cases (throttle vectors vs lodash `throttle`, oracle
  `quiz-react-lodash-throttle`); journey case (no socket before identification, roster + home room after, online
  1 → 2, learning now, peer cursor, preference, leave, quiz room with task); `👥️presence-presentation` 5 cases (palette
  vs d3-color); mutation checks 12/12 killed. Two-device run (localhost 1440 light vs 127.0.0.1 768 dark, Playwright
  Chrome against the dev proctor): A's pointer at 0.3/0.6 of the Heating card appears at 0.300/0.600 of B's card in A's
  colour; B's keyboard focus framed on A; "Learning now: 1" and "working on Energy Demand"; A leaves → B "Online: 1";
  no console errors (evidence `🗑️generated/ui-final/presence-*.png`, `presence-report.json`).
- **Handoffs**: the core `SCREENS` has no badges screen (badges counts as home); production needs the site origin in
  `PROCTOR_ALLOWED_ORIGINS` (baked into the image) and `connect-src` for the proctor if a CSP is added.

## 2026-09-29 — layered home like semio-tech play (appended by the coordinator)

- **Overview**: home is the shared `LayeredOverview` (`@semio-tech/ui-react/chrome`) with nine panes in reading order
  `learner, physics, intro, heating, board, cooling, badges, demand, prefs`; cells from `homeCells` (3 × 3 desktop with
  the board centre, 2 × 5 tablet, list mode below 768 px); hash routing by the element, the opened page controlled
  from the session step `{ screen: "home", page }`; `Navbar` in flow above.
- **Cards**: compact, centred in their cells (`.quiz-home-grid` `align-items: center`), heading link to `#page`, pointer
  click opens (not on controls), `revealed` lifts the card.
- **Pages** (pure views over session state, `PageFrame`): learner profile (`📇️profile`: totals, rank, run history with
  View result/Resume, badges), quiz page (`📖️quiz-page`: description, facts, tasks with kinds, Start/Resume/Start again,
  Last result), introduction, leaderboard (`LeaderboardPage`, polling only while opened or revealed), badges
  (`BadgesPage`), preferences (`PreferencesPage`). Hover never starts a run. The old leaderboard/badges screens and
  `HomeCard` were removed.
- **Presence**: places intro → introduction, board → leaderboard, badges → badges, quiz page → quiz (rooms), learner and
  prefs without room; anchors `quiz:<id>`, `quiz:<id>:tasks`, `badges`; peer cursors only on anchors outside inert panes.
- **Checks**: quiz-react 182/182; site test 17/17; mutation checks 11/11 killed; `🏠️home-grid` cases (order and cell
  vectors, nine sections, board fifth, heading links, inert/aria-hidden panes, `#heating` deep link runs nothing, Escape
  returns focus to the card link, hover/focus reveal never starts a run, polling gating with fake timers, column counts
  via lightningcss); browser walk (Playwright Chrome, synthetic learners, every command refused) at 1440/768/375 light
  and dark: rest veil visible, hover each card → veil hidden + strip glides to the card's cell, open `#heating` → page
  region focused + Overview button, Escape → focus on the card link, list mode at 375, 0 overflow, 0 console errors;
  two-device presence re-run green. Screenshots `🗑️generated/layered-final/quiz-*.png`, `presence-*.png`.
- **Bundle**: main JS 642,365 B (gzip 175,347), CSS 248,730 B (gzip 36,848).
- **Handoffs**: pages are short, so mostly their top part shows behind the cards; the tablet card layer scrolls and
  takes the pointer, so there is no pointer pan on tablets.

## 2026-09-29 (night) — live grid home and what the others think (appended by the coordinator)

- **Live grid home**: `rest="grid"` with `HOME_GRID_TRACKS` (1 : 1.5 : 1 × 1 : 1.4 : 1); the card layer uses
  `var(--layered-columns)`/`var(--layered-rows)` from 768 px without gap or outer padding (spacing inside
  `.quiz-home-cell`); budget = number of pages, suspend times infinite; all nine panes mounted and inert at rest, no
  placeholders; measured tracks 411/617/411 × 256/359/256 px at 1440 × 900, each page top-centred on its card's cell.
  Home polls the leaderboard every 10 s and the crowd of every quiz; in the walk a peer's total rose 170 → 200 in the card
  and in the backdrop page after one poll.
- **Presence inside pages**: one room socket per room kind; home's room watches introduction, leaderboard, badges, every
  quiz page and every thinking room (≤ 16) at 4 Hz and re-sends the watch after a rejoin; each page draws the others on
  its own anchors scaled with the page (peer pointer measured inside the leaderboard pane at (0.35, 0.4)); decorative
  (`aria-hidden`).
- **In the run**: pointer and focus on `item:<id>`/`category:<id>`, drag shown as "Ben ▸ …"; drafts published ≤ 2 Hz
  (matching converted with `thinkingAnswer`); live crowd per item (classification counts per category, sorting markers
  and mean place, matching values by count then numerically); submitted crowd when nobody else is online; preference
  "Show what others think" / "Zeigen, was die anderen denken".
- **Results and quiz page**: an "Everyone" column per result item; the quiz page lists what everyone answered item by
  item. Wording: submitted figures include the viewer's own runs ("What everyone answered"), live figures never do
  ("What others think now").
- **Shared fix**: `measureWindowSilhouetteMetrics` in `🧱️elements/🗂️WindowChrome` measured with
  `getBoundingClientRect`, so silhouettes inside scaled pages were drawn at the scaled size and opened quiz pages showed
  cut-off cards; it now measures in the element's own pixels (assertion added to the silhouette test).
- **Renames/registrations**: test and fixture `👥️presence-client` → `📡️presence-client` (sibling emoji collision);
  `🔨️modules/🗳️crowd` and `💭️crowd-client` registered.
- **Checks**: quiz-react 229/229; `💭️crowd-client` suite with lodash as oracle; 43/43 mutations killed; typecheck without
  quiz errors; oracle registry valid (ajv); two-device walk against the dev proctor (screenshots
  `🗑️generated/crowd-final/`): live grid, peer cursors inside backdrop pages, drafts reflected as "what others think",
  crowd on results and quiz pages. Bundle JS 666.2 kB (gzip 182.1 kB), CSS 248.9 kB (gzip 36.9 kB).
- **Handoffs**: the crowd includes the viewer's own submitted runs (excluding them would need the learner in the query);
  a revealed page fills the view while the other cards stay on top (element behaviour).
