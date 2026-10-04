# Explore: the quiz React client, as built, for the challenge levels

Read-only survey, 2026-10-02, of `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/` as it is on disk now (other sessions
are editing `🎨️.css`, `▶️run`, `🏆️leaderboard`, `🪟️chrome` — see section 7). Everything below was read in the
files unless it is tagged **[inferred]**. `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `Q` =
`🧰️framework/🛍️products/❓️quiz`, `M` = `R/🔨️modules`. Line numbers are of the files as read today.

## 0. The ten facts that shape the design

1. **The sheet is solution-free.** `SheetSortingTask.items` are `{id,label,icon}` only (`Q/🧬️schema/🟦️.ts:156,171-179`):
   the client never holds a sorting value before the results. "Easy: the keys are visible" is impossible with today's sheet; the
   value must either enter the sheet (then `sheetOf(quiz, seed)` is no longer `f(quiz, seed)` and the deputy's staleness
   check breaks, see 3.3) or be disclosed per level by some other view. Matching already shows its values (cards, `SheetDimension.cards`, `:182`);
   classification shows category profiles only where the quiz authored axes.
2. **Sorting guesses already exist and are the "hard" mechanic** — optional, always visible, SI-prefix-aware, but **never scored**
   (`Q/🔨️modules/📏️scoring/🟦️.ts:71-87` ignores `answer.guesses`; validation only checks they agree with the order,
   `Q/🔨️modules/✅️validation/🟦️.ts:638-646`). They only reorder the list.
3. **There is no timer, clock, tick or countdown anywhere in the client.** The only intervals are the presence overlay
   placement (`M/👥️presence/🟦️.tsx:992`), the presence/leaderboard pollers and retry timeouts. `RunView` knows only
   `startedAt`/`submittedAt` (proctor clock, `Q/🧬️schema/🟦️.ts:340-350`); there is no per-task "opened at".
4. **Preferences are device-level and outside the session.** They live in React state of `QuizApp` + store slice
   `preferences` (`R/🟦️.tsx:508,523`, `M/🎛️preferences/🟦️.tsx:72`), are not part of any command and not of `QuizState`.
   A challenge level is per run, so it must travel in `start-run` and `RunView`, not in preferences (a preferences-held
   default is fine on top).
5. **`start` is a single call:** `session.startRun(quiz, signal)` (`M/🧭️session/🟦️.ts:799`) building
   `{type:"start-run", id, learner, run, quiz}` (`Q/🧬️schema/🟦️.ts:264`). It is called from one place
   (`actionsOf`, `M/📖️quiz-page/🟦️.tsx:41`, shared by the compact overview card and the quiz page); `resumeRun` from there and from the
   profile table (`M/📇️profile/🟦️.tsx:149`).
6. **A score is a percent everywhere except the leaderboard "points".** `formatScore` (`M/📏️quantity/🟦️.ts:159`) for
   run/task/quiz-best/crowd; `formatPoints` (`:164`) only for the learner total and leaderboard `Total`. Points are computed
   in the core (`Q/🔨️modules/👁️views/🟦️.ts:85`): sum over catalog quizzes of **best score × 100**. A per-run points number is
   shown nowhere, and "best" is per score, not per points.
7. **Translation tests are strict** (`Q/🧪️tests/🗣️translation-completeness/🟦️.tsx`): every key exists in EN and DE with identical
   placeholders, **every key must be used in client sources and every used `"quiz.x.y"` literal must exist** (regex
   `"(quiz\.[a-zA-Z]+\.[a-zA-Z]+)"`: exactly three letter-only segments), no `{{n}} runs/points/answers` plural patterns (counts are "Label: n"),
   informal German ("du"), one German word per concept.
8. **Navigation/address cannot carry a level**: the address names only home pages (`#board`, `#<quiz>`); run and results are never
   addressable (`M/🚏️navigation/🟦️.tsx:97-100`). The level belongs to the run (`RunView`), not to the URL.
9. **Two leaks undermine hard/expert unless handled:** the in-run `CrowdDoor` ("Show it now", `M/▶️run/🟦️.tsx:200`, or preference `others: always`)
   shows other learners' sorting places/matching values, and the thinking room shares the learner's draft answers incl. sorting
   guesses with peers (`M/👥️presence/🟦️.tsx:469-477`, `presenceDrafts`).
10. **The deputy (offline decider) derives its state from `RunView`/`RunSummary` only** (`M/🫡️deputy/🟦️.ts:105-167`): a level must be a field of
    those views (and `RunState`) or the device cannot decide, score or even recognise a run, and a level-dependent sheet must be
    produced by the same `sheetOf` the deputy compares against (`:151`).

## 1. Module inventory

Package glue: `R/📦️packages/🟦️typescript/🟦️.tsx:2` = `export * from "../../🟦️.tsx"` (package `@semio-tech/quiz-react`,
`package.json`: exports `.` and `./🎨️.css`; deps framework, framework-server, pets, pets-react, quiz, ui-react, react 19). No runtime dependency outside
the repo. `📋️project.json`: targets `test` / `test-quick` / `test-long` / `test-exhaustive` / `typecheck` (all `bun ./📜️script.ts …`;
`📜️script.ts` runs vitest with `../../🧪️tests/🎚️config/🟦️.ts`, typecheck = `tsc --noEmit -p tsconfig.json`). Launch entries (`.vscode/launch.json`):
`🧪️test❓️quiz⚛️react` (`bun nx run @semio-tech/quiz-react:test`), `🛠️dev❓️quiz⚛️react🪁️typecheck`, dev site `🛠️dev🎓️teaching🏛️architecture❓️quiz` (+ `🌐️site`).
The only consumer is the teaching site `🎓️teaching/🏛️architecture/❓️quiz/🟦️.ts` (`mountQuiz`).

### Root `R/🟦️.tsx` (559 lines)
- Reexports (all modules' public API) `:55-183` — **every new export a test needs must be added here**.
- `interface QuizOptions {proctor, tenant, material?, logo?, legal?, pets?, transport?, presence?, storage?, languages?, timing?}` `:191`.
- `connectionState(connection)` `:222`, `connectionMessage` `:233`, `waitingMessage` `:240`, `Waiting` `:249`, `stepKey` `:264`,
  `documentTitle(state, locale)` `:279`, `Page({aside?, wide?})` `:289` (`max-w-[100rem]` when `wide`), `Screen` `:300`
  (switch on `step.screen`; run `:335`, results `:343`), `useRootTextScale` `:354`, `ConnectionStatus` `:369`, `Client` `:403`
  (navbar items `:445-452`, `<main id="quiz-main" tabIndex={-1}>` `:469`), `useSetup` `:487`, `QuizApp(options)` `:500`, `mountQuiz(root, options)` `:547`.

### `M/` (all `🟦️.ts(x)`)
| module | purpose | exports (line) |
|---|---|---|
| `🌐️i18n` (858) | EN+DE chrome bundles on the shared ui-i18n port | `QuizLocale` `:18`, `QUIZ_LOCALES` `:21`, `isQuizLocale` `:24`, `preferredLocale` `:30`, `localized(text, locale)` `:39`, `applyLocale` `:45`, `QUIZ_BUNDLE_EN` `:56`, `QUIZ_BUNDLE_DE` `:439`, `QuizLabelKey` `:822`, `REJECTION_LABELS` `:827`, `TASK_KIND_LABELS` `:847`, `QuizText` `:852`, `quizText(locale)` `:855` |
| `📏️quantity` (191) | numbers, units, percents, points, dates | `SIGNIFICANT_DIGITS`, `SI_PREFIXES`, `engineering` `:69`, `formatNumber` `:78`, `withUnit` `:85`, `formatQuantity(value, quantity, locale)` `:91`, `parseQuantity(text, quantity, locale)` `:146`, `formatScore(score, locale)` `:159`, `formatPoints(points, locale)` `:164`, `oneDecimal` `:170`, `formatInstant/Clock/Date` `:179/184/189` |
| `🧩️task` (103) | shared task plumbing | `TaskViewProps<T,A>` `:13`, `DROP_ZONE_CLASS` `:22`, `SELECT_CLASS` `:25`, `ICON_BUTTON_CLASS` `:28`, `SELECT_ANNOUNCEMENT_DELAY_MS=400` `:33`, `Announcement` `:36`, `useAnnouncement(delayMs=0)` `:43`, `LiveRegion` `:61`, `useFocusAfterRender` `:71`, `elementId(scope, ...parts)` `:85`, `DragGrip` `:90` |
| `🗂️classification` (114) | classification task UI | `ClassificationTaskView` `:26` |
| `↕️sorting` (230) | sorting task UI + numeric guesses | `reordered` `:29`, `ordered` `:39`, `SortingTaskView` `:150` (internal `GuessField` `:65`) |
| `🃏️matching` (127) | matching task UI | `assignCard` `:28`, `MatchingTaskView` `:34` |
| `▶️run` (255) | run player | `TASK_KIND_ICONS` `:29`, `TaskGlyph` `:32`, `TaskView` `:38`, `RunScreen` `:67` |
| `🏁️results` (294) | results of a submitted run | `TaskResultView` `:200`, `ResultsScreen` `:215` (internal `Verdict` `:29`, `ResultTable` `:59`, `ClassificationResult` `:78`, `SortingResult` `:105`, `MatchingResult` `:151`, `ResultTables` `:189`) |
| `🏠️home` (266) | layered overview, pages + cards | `HomeLayout`, `homePages` `:46`, `homeCells` `:53`, `HOME_GRID_TRACKS` `:75`, `HOME_GRID_ROW_HEIGHT_PX=160` `:85`, `HOME_CHROME_HEIGHT_PX=56`, `homeGridMinHeight` `:93`, `homeLayoutQueries` `:102`, `pageLabel` `:125`, `HomeScreen` `:145` |
| `📖️quiz-page` (183) | a quiz's card + page, start/resume action | `Act` `:22`, `QuizCardView` `:76`, `sheetOfQuiz` (internal) `:102`, `QuizPage` `:147` |
| `📇️profile` (222) | learner card/page, runs table, switch identity | `SwitchIdentity` `:51`, `LearnerCard` `:98`, `LearnerPage` `:159` |
| `🏆️leaderboard` (459) | period/category choice, sortable tables, polling | `LEADERBOARD_POLL_MS` `:28`, `POLL_BACKOFF_MAX_MS`, `BOARD_EXCERPT_SIZE`, `LeaderboardKey/Sort`, `RANK_ORDER`, `nextSort` `:50`, `sortLeaderboard` `:75`, `ownRow` `:87`, `boardExcerpt` `:93`, `pollDelay` `:102`, `usePolling` `:110`, `leaderboardColumns` `:154`, `LeaderboardCard` `:259`, `LeaderboardPage` `:377` (internal `BoardChoices` `:180`, `SortHeading` `:194`, `BoardRow` `:234`, `PageRow` `:332`) |
| `🏅️badges` (122) | badge card/page | `awardOf` `:21`, `badgesEarnedIn` `:27`, `BadgesCard`, `BadgesPage` |
| `🗳️crowd` (463) | when/how the others' answers show | `OTHERS_CHOICES` `:32`, `crowdGate` `:46`, `crowdShown` `:53`, `CrowdDoor` `:60`, `answerFigure` `:227`, `scoreFigure` `:263`, `AnswerFigure` `:310`, `ScoreFigure` `:401`, `TaskFigures` `:445` |
| `📊️plot` (43) | column mark | `Column`, `columnShare`, `peakOf`, `formatShare` |
| `🕸️radar` (524) | spider diagram (classification profiles) | `RadarChart` `:445`, layout helpers |
| `🧭️session` (1256) | state, events, controller | see 3.1 |
| `🫡️deputy` (168) | on-device decider | see 3.3 |
| `📮️outbox` (366) | persisted command queue | `Outbox` `:105`, `coalescingKey` `:56`, `commandRun` `:61`, `coalesce` `:97` |
| `💾️persistence` (187) | tenant-scoped local store | `StorageArea`, `LocalSlice` `:110`, `LocalCollection`, `localStore` , `isRecord` |
| `🛂️proctor` (441) | wire client, retry, throttling | `ProctorClient` `:353`, `newId` `:37`, `commandEnvelope` `:262` (payload = JSON of the command), `commandTarget` `:251` |
| `🚏️navigation` (134) | navbar ways + address | `placeName` `:22`, `NAVIGATION_WAYS`, `NavigationControls` `:57`, `stepAddress` `:98`, `useAddress` `:106` |
| `🎛️preferences` (272) | persisted display preferences | see 3.4 |
| `🪟️chrome` (379) | card language and small parts | `QuizCard` `:33`, `PageFrame` `:105`, `CardIcon` `:117`, `Glyph` `:129`, `IconLabel` `:140`, `Mark` `:151`, `Missing` `:156`, `Facts` `:166`, `TABLE` `:180`, `Records` `:191`, `Segment`/`Segments` `:202/211`, `BodyButton` `:242`, `Problem`/`ProblemNote` `:262/269`, `Dialog` `:298`, `CardAction` `:25`, `cn` |
| `🪪️identity` (240) | identity step, problem formatting | `learnerName` `:21`, `failureProblem` `:28`, `thrownProblem` `:40`, `noticeProblem` `:45`, `handleFault*`, `IdentityScreen` `:118` |
| `👥️presence` (1091) | rooms, cursors, thinking drafts | `PRESENCE_ANCHORS` `:725`, `presencePlace` `:436`, `presenceDrafts` `:469`, `sheetItemLabels` `:480`, `usePresenceTask` `:762`, `useDocumentVisible` `:771`, `usePresenceView` `:757`, `PresenceOverlay`, `PanePeers`, `OnlineMark`, `PresenceStatus`, `PresenceList` |
| `🐾️pets` (262) | glue to the pets product | `effectivePetMode` `:43`, `QuizPetsProvider` `:160`, `usePetsReduced` `:208`, `usePetsForced`, `PetsSwitch` `:223`, `QuizPets` `:252`, `QUIZ_PET_KEEPOUTS` `:87` (`[data-quiz-item], [data-quiz-drop], .quiz-app > header`) |
| `👋️introduction`, `⚖️legal`, `🤏️drag` | intro screen/card/page; footer + privacy dialog; pointer drag (`startPointerDrag` `:46`, `dropZoneAt` `:13`) | |

## 2. The run flow as built

### 2.1 Home card → quiz page → start → run
1. `HomeScreen` (`M/🏠️home/🟦️.tsx:145`) builds one pane per page; for a quiz id it renders the card `QuizCardView` (`cardOf`, `:213-216`) and the page `QuizPage` (`:184`). It owns the
   command runner `act` (`:166-175`): `const act: Act = (run) => {…controller = new AbortController(); setPending(controller); run(controller.signal).then(failure => setProblem(failureProblem(failure,text)))…}` and shows
   `role="status"` with an indeterminate `<progress>` "Preparing your quiz…" and a **Cancel** button while pending (`:238-244`). Start is cancellable already.
2. `actionsOf(props)` (`M/📖️quiz-page/🟦️.tsx:34-50`) is the one place that builds the primary action:
   ```tsx
   open === undefined
     ? <CardAction primary disabled={busy} onClick={() => act((signal) => session.startRun(quiz.id, signal))}>{text(best === undefined ? "quiz.home.start" : "quiz.home.again")}</CardAction>
     : <CardAction primary disabled={busy} onClick={() => act((signal) => session.resumeRun(open, signal))}>{text("quiz.home.resume")}</CardAction>
   ```
   plus `last` = "View last result" → `session.open({screen:"results", run:last})`. Both `QuizCardView` (`:76`, footerLeft `last`, footerRight `primary`) and `QuizPage` (`:147`, same footer on the first card) use it.
   `facts()` `:55` lists tasks, best score, "In progress", learning-now. `Earned` `:68`. `QuizPage` also loads the newest run's sheet (`loadRun(latest)`, `:156`) for the crowd labels.
3. `QuizSession.startRun` (`M/🧭️session/🟦️.ts:799-816`): needs `state.learner`; `command = {type:"start-run", id:newId(), learner, run:newId(), quiz}`; `decided(command, signal)`
   (proctor, or the deputy after `DEPUTY_PATIENCE_MS=3000`, see 3.3); a `run-open` rejection resumes the open run; accepted: `run-voided` events discard stale open runs; then `resumeRun(command.run, signal)` (`:819`):
   loads the run view if unseen, dispatches `{type:"step-opened", step:{screen:"run", run}}`. **A new run of a quiz with an open run resumes the open one** — a level chooser must not offer a level for a resumed run.
4. `Screen` (`R/🟦️.tsx:335-342`): `state.runs[step.run] === undefined ? waiting : <Page wide><RunScreen key={step.run} session state run text locale others={preferences.others} /></Page>`.

### 2.2 `RunScreen` (`M/▶️run/🟦️.tsx:67`)
Props: `{session, state, run: Id, text, locale, others?: OthersChoice}`. Local state: `current` task index, `submission` (`undefined | {stage:"confirming"} | {stage:"working", controller, phase}`), `problem`.
- Derived: `view = state.runs[run]` (`RunView`), `tasks = view.sheet.tasks`, `complete = tasks.map(t => answerComplete(t, view.answers[t.id]))`, `ready = done === tasks.length && view.status === "open"` (`:88-90`).
- Cards: (1) the **run card** (`card="run"`, `:118-172`): title = `localized(view.sheet.title)`, footerRight = Submit `CardAction` with `aria-disabled={!ready}` + `aria-describedby` on the hint (stays focusable), description `<p class="quiz-prose …">`, `.quiz-run-head` containing the task step chips `<ol class="quiz-steps">` (buttons with `aria-current="step"`, ✓/○ + Complete/Incomplete, `:134-160`) and `.quiz-run-progress` (`<progress>` + `quiz.run.progress`, `:161-165`), the submit hint, `ProblemNote`.
  (2) the **task card** (`card="task"`, `:173-205`): heading `ref={taskHeading}`, footer Previous/Next, a muted line `Task i of n · <kind> · Complete|Incomplete` (`:193-195`), the prompt `<p class="quiz-prose m-0 text-sm …">`, then
  ```tsx
  <div className="quiz-task">
    <TaskView key={task.id} task={task} answer={view.answers[task.id]} onAnswer={(answer) => session.answer(run, task.id, answer)} text={text} locale={locale} />
  </div>
  <CrowdDoor gate={gate} onAsk={…} onUnask={…} text={text} />
  {crowdShown(gate) ? <TaskFigures … /> : null}
  ```
  (`.quiz-task` is a `container-type: inline-size` query container, CSS `:270`.) `key={task.id}` remounts the task view when the task changes, so local field state (e.g. a guess draft) is dropped on navigation.
  (3) the **dialog** (`:206-252`): `Dialog role="alertdialog"` "Submit this quiz?" (confirming) → "Submitting…" with `<progress>` (determinate while `saving`, indeterminate after), Cancel aborts, a separate `<p role="status" class="sr-only">` speaks the phase name (`PHASE_SPOKEN` `:62`) while the visible body is `aria-live="off"`.
- Task navigation is **free** (`go(target)` `:93`, any order, step chips); focus moves to the task heading after `go` (`moved` ref, `:77-80`).
- `submit()` `:98-113`: `session.submit(run, controller.signal, onPhase)`; failures → `failureProblem`.
- Presence: `usePresenceTask(taskId)` `:83` (sets the learner's place incl. current task), `anchor={PRESENCE_ANCHORS.task(task.id)}`.

`TaskView` (`:38-48`) is the one dispatch on `task.kind`:
```tsx
export function TaskView(props: { task: SheetTask; answer: Answer | undefined; onAnswer: (answer: Answer) => void; text: QuizText; locale: QuizLocale }): ReactElement
  case "classification": <ClassificationTaskView task answer={answer?.kind==="classification"?answer:undefined} onAnswer text locale />
  case "sorting":        <SortingTaskView …/>      case "matching": <MatchingTaskView …/>
```
Every kind gets exactly `TaskViewProps<T extends SheetTask, A extends Answer>` (`M/🧩️task/🟦️.tsx:13-19`): `{task: T; answer: A|undefined; onAnswer: (answer:A)=>void; text: QuizText; locale: QuizLocale}`.
**There is no `disabled`/`readOnly`/`results`/`level` prop today** — task views are not reused read-only (results re-render separate `…Result` tables). Adding a `level`/`hints`/`locked` prop means extending `TaskViewProps` and the one `TaskView` switch.

### 2.3 Task kinds
- **Classification** (`M/🗂️classification/🟦️.tsx:26`): items as chips (`quiz-chip`: grip, label with `IconLabel`, a `<select className={SELECT_CLASS}>` listing categories, `:53-76`); pool section + category bins (drop zones `data-quiz-drop="pool|category:<id>"`), category `description`, optional `RadarChart` of `category.profile` over `task.axes` (`:104`). `assign()` `:36` emits `{kind:"classification", assignments}`, announces via `useAnnouncement(SELECT_ANNOUNCEMENT_DELAY_MS)`, refocuses the select.
- **Sorting** (`M/↕️sorting/🟦️.tsx:150`): below.
- **Matching** (`M/🃏️matching/🟦️.tsx:34`): per dimension a card pool (`ul.quiz-cards`, each card `cardText(dimension, index) = formatQuantity(dimension.cards[index], dimension.quantity, locale)`, shows free/used-by, `:68-86`) and one row per item with a `<select>` over the cards (taken cards are `disabled` options `:104-112`) + remove button. Answer: `{kind:"matching", assignments:{[dimension]:{[item]: cardIndex}}}`. **The values are visible by construction** (cards); the answer is a card *index*, so a "hidden keys, learner types numbers" variant needs a different answer shape (a scored index cannot hold a typed number).
- Task kind icons/labels: `TASK_KIND_ICONS` `M/▶️run/🟦️.tsx:29`; `TASK_KIND_LABELS` `M/🌐️i18n/🟦️.ts:847`.

### 2.4 The sorting guess field in detail (`M/↕️sorting/🟦️.tsx`)
- **Answer shape** `SortingAnswer = {kind:"sorting", order: Slug[], guesses?: Record<Slug, number>}` (`Q/🧬️schema/🟦️.ts:207`); guesses are in the quantity's **base unit** (never prefixed). `answerOf(order, guesses)` `:49` omits `guesses` when empty.
- **Default state:** `order = answer?.order ?? task.items.map(i => i.id)` (presented order), `guesses = answer?.guesses ?? {}` (`:155-156`). With `answer === undefined` a "Keep this order" block shows (`:193-198`); the order only counts as an answer after a move, a guess or `keep()` (`:183`), which also moves focus to the list.
- **`GuessField`** (`:65-147`) props `{id, label, quantity, value: number|undefined, onCommit(value|undefined, focus: string|undefined), text, locale}`. State: `draft: string|undefined`, `rejected: boolean`.
  `shown = value === undefined ? "" : formatQuantity(value, quantity, locale)`; `typed = draft ?? shown`; `parsed = draft===undefined||blank ? undefined : parseQuantity(draft, quantity, locale)`. Input:
  ```tsx
  <input id={id} type="text" value={typed} autoComplete="off" spellCheck={false}
    placeholder={text("quiz.sorting.guessPlaceholder", { example })} aria-label={text("quiz.sorting.guess", { item: label })}
    aria-invalid={rejected} aria-describedby={rejected ? errorId : parsed === undefined ? undefined : previewId}
    className="quiz-input quiz-target min-w-0 text-sm tabular-nums" onChange=… onBlur={leave} onKeyDown={press} />
  ```
  Preview `<span id=previewId>= {formatQuantity(parsed,…)}</span>` while typing (`:135-139`); error `<p role="alert">` with `quiz.sorting.guessInvalid[Positive]` (`:140-144`, "positive" on logarithmic scales). Commit on **Enter** (`commit(id)`) and on **blur** (`leave` passes the next focus target id); **Escape** reverts the draft; commit is skipped when `draft === shown` (so a rounded display never overwrites an exact guess) and when `parsed === value`. Empty text removes the guess. An unreadable text stays flagged until fixed or emptied.
  `exampleGuess(quantity)` = `withUnit("2", quantity.prefixed ? "k"+unit : unit)` (`:59`), the unit always shows through the placeholder.
- **Parsing** `parseQuantity` (`M/📏️quantity/🟦️.ts:146`): `^([+-])?(digits with grouping)(e±n)?\s*(unit)$`; locale-aware decimal/group marks (`plainMantissa` `:117`), prefix table `SI_PREFIXES` (+ `u`/`μ`→µ, `K`→k; prefix case-sensitive, unit case-insensitive, `:98-105`, `typedExponent` `:135`), value assembled as `Number("<digits>e<exp>")`; `undefined` if not positive on a logarithmic scale.
- **Reorder on commit:** `guess()` `:175` → `ordered(order, next)` (`:39`: guessed items sort ascending among the places they occupy, ties stable, unguessed items stay) → `onAnswer(answerOf(sorted, next))`, announces `quiz.sorting.guessed` ("{{item}}: guess {{value}}, now at position …") and refocuses (`focusAfterRender(focus)`) when the order changed. **Moving by hand** (`move` `:162`, buttons ↑/↓ with `aria-disabled` at the ends, or drag via `DragGrip` onto another item's `data-quiz-drop="item:<id>"`) calls `without(guesses, id)` — the guess is dropped (`quiz.sorting.moveClears`).
- **Layout:** each `<li class="quiz-sort …">` = grip, place number (`aria-hidden`), label (`IconLabel`), guess, up, down (`:204-221`); CSS grid areas `grip place label guess up down`, container queries at 36rem/56rem (`R/🎨️.css:423-493`); at 56rem the guess is `12rem minmax(0,1fr)` leaving a column for the preview.
- **Texts:** `quiz.sorting.hint/list/smallest/largest/up/down/drag/moved/first/last/keep/kept/keepHint/moveClears/guessHint/guess/guessPlaceholder/guessPreview/guessInvalid/guessInvalidPositive/guessed/guessCleared` (`M/🌐️i18n/🟦️.ts:257-280`).
- **Results:** `SortingResult` (`M/🏁️results/🟦️.tsx:105`) adds a **"Your guess" column only if any guess exists** (`guessed`, `:108,111,120-124`; `<Missing label="No guess"/>` for unguessed rows), then the true `value`, rank + `Verdict`, explanation, and an ordered list "Correct order" with values (`:137-146`). So *true values are first revealed in results*.
- **What a level can reuse:** `GuessField` is the hard-level input as is; for "easy: keys visible" show `formatQuantity(trueValue)` per row (needs the value in the sheet), for the ">×1000 hint" compare `parsed` against the value with `Math.abs(Math.log10(parsed/value)) > 3` (logarithmic scales only; **[inferred]** the quantity of the existing sorting tasks is logarithmic + prefixed: `powers`, `energies`).

## 3. Session and state

### 3.1 `QuizState` (`M/🧭️session/🟦️.ts:138-153`) — full shape
```ts
interface QuizState {
  step: QuizStep;                       // {screen:"introduction"}|{screen:"identity"}|{screen:"home";page?}|{screen:"run";run}|{screen:"results";run}
  trail: QuizTrail;                     // {back: QuizStep[]; forward: QuizStep[]}
  introduced: boolean;
  learner?: QuizLearner;                // {id, identity?}
  catalog?: CatalogView;                // {id,title,introduction,quizzes: CatalogQuizView[],badges}
  learnerView?: LearnerView;            // {learner,identity,runs: RunSummary[],badges,best: Record<quiz,Score>,total}
  runs: Record<Id, RunView>;            // {run,learner,quiz,status,sheet,answers,result?,startedAt,submittedAt?}
  awards: Record<Id, Slug[]>;           // badges announced by a submission
  board: BoardChoice;                   // {period, quiz?}
  leaderboards: Record<string, HeldLeaderboard>;
  submissions?: number;
  crowds: Record<Slug, CrowdView>;
  asked: Slug[];                        // quizzes whose crowd the learner asked to see before submitting
  notice?: QuizNotice;
}
```
Events (`QuizClientEvent` `:159-178`): `introduction-read, step-opened{step,instead?}, step-retraced, catalog-loaded, learner-identified, learner-recalled, learner-forgotten, learner-loaded, run-loaded{view}, answer-given{run,task,answer}, run-submitted{run,result,badges,at}, run-voided, board-chosen, leaderboard-loaded, crowd-loaded, crowd-asked, crowd-unasked, notice-raised, notice-dismissed`. Pure fold `evolveQuizState` `:280`; persistence `persistQuizState` `:431` (slices `introduced, learner, catalog, learner-view`; collection `runs` one record per run, restored by `restoredRun` `:405` which only checks `run, learner, sheet, answers, status` — an extra `level` field would survive untyped).
Commands (`Q/🧬️schema/🟦️.ts:261-273`): `identify-learner`, `start-run {id,learner,run,quiz}`, `record-answer {…,run,task,answer}`, `submit-run {…,run}`.
Selectors: `openRunOf` `:366`, `lastSubmittedRunOf` `:373`, `runAwards` `:359`, `mergeRunViews` `:384` (tab merge: closed beats open; open answers union).
Controller `QuizSession` (`:522`): `start/stop/reconnect/open/back/forward/up/readIntroduction/dismissNotice/forgetLearner/identify/startRun/resumeRun/answer/submit/loadRun/askCrowd/unaskCrowd/chooseBoard/refreshCatalog/refreshLearner/refreshCrowd/refreshLeaderboard`. `answer(run, task, answer)` (`:828`) dispatches `answer-given` locally and `outbox.enqueue({type:"record-answer",…})`; `submit(run, signal, onPhase)` (`:839`) → saving (outbox settled) → submitting → results, `SubmissionPhase = saving{done,total}|submitting|results`.
How `start` carries data: only `{quiz}` today. **[inferred]** carrying a level = extend `StartRunCommand` (schema, TS twin, Rust twin), `startRun(quiz, level, signal)` (`:799`), the `restoredCommand` start-run branch (`M/📮️outbox/🟦️.ts:69`, the `start-run` case validates only `run`/`quiz`), `RunView`+`RunSummary` and a `run-started` event field.

### 3.2 Outbox (`M/📮️outbox/🟦️.ts`)
Persisted per-record queue (`semio.quiz.<tenant>.outbox/<id>` via `localStore`), answers coalesced per `run/task` (`coalescingKey` `:56`), every other command kept in order; `decision(command)` (session `:492`) = every command but `record-answer`. A deputy-decided `start-run` waits here and is replayed to the proctor — **the level in the command is therefore also what the proctor will re-decide with**; a proctor that disagrees (e.g. a level not offered) would void the device's run (`decisionSettled` `:1164-1180`).

### 3.3 Deputy (`M/🫡️deputy/🟦️.ts`)
`new Deputy(material: QuizMaterial = {catalog, quizzes})` (`:95`, `R/🟦️.tsx:492`). It keeps nothing: `state(held: HeldLearner)` (`:105`) rebuilds a `LearnerState` (`{learner, identity?, runs: RunState[], badges}`) from `learnerView.runs` (`RunSummary`) and the held `RunView`s; `run()` (`:146-167`) builds each `RunState {run, quiz, revision, seed, status, answers, recorded, result?, startedAt, submittedAt?}` where
`revision = current ? loaded.revision : REVISED` and `current = loaded !== undefined && (view === undefined || status !== "open" || alike(view.sheet, sheetOf(loaded.quiz, seed)))` — **the held sheet must equal `sheetOf(quiz, seed)`**. `decide()` (`:118`) → core `decideLearner(startRun|recordAnswer|submitRun)` (`Q/🔨️modules/🧾️lifecycle/🟦️.ts:81,139,155`); `runView(state, run)` → core `runView(state, run, quizzes)`; `learnerView`, `leaderboard` (device-only standing).
Consequences for a level **[inferred]**: (a) `RunState`, `RunView`, `RunSummary`, `run-started`, `StartRunCommand` all need the level so `state()` can recover it; (b) `sheetOf` must accept the level if the sheet differs per level, or `alike(...)` flags the run as `REVISED` and it is voided; (c) scoring with a multiplier lives in core `submitRun`/`scoreRun` (`Q/🔨️modules/📏️scoring`), which the deputy calls unchanged — good, one implementation; (d) a timer limit cannot be enforced by a deputy that has the device clock only — a run "expired" verdict must be derivable from event `at` timestamps the deputy itself writes (`this.now()`, `Date.now` default `:621`).
The session flows: `deputise(command)` `:1041` → `decide`, `outbox.enqueue(command)`, `fold(after, events)` `:1058` (dispatch `run-loaded` with `sheet: this.state.runs[run]?.sheet ?? view.sheet`) and `learner-loaded`.

### 3.4 Preferences — the precedent for a chooser (`M/🎛️preferences/🟦️.tsx`)
- Type `QuizPreferences {locale?, theme, textSize, showCursors, others, animateIcons, iconsChosen, pets, petsLiveliness, petsChosen}` `:56-67`. Stored whole (`writePreferences(store, prefs)` `:110` → `store.write("preferences", prefs)`), read with per-field validation and defaults (`readPreferences` `:72-88`: unknown values fall back, e.g. `others` default `"submitted"`, `animateIcons: record.animateIcons !== false`). `QuizApp` keeps it in `useState(() => readPreferences(store))`, re-reads on another tab's change (`R/🟦️.tsx:521`) and passes `preferences` / `onPreferences` down (`Client`, `HomeScreen`, `RunScreen` gets only `preferences.others`).
- A "chosen" fact next to the value is the pattern when a device default (reduced motion) must not override an explicit choice: `iconsChosen`/`petsChosen`, `effectiveIconMotion(animate, chosen, reducedMotion)` `:92`, `withIcons` `:98`, `withPets` `:105`, `effectivePetMode` (`M/🐾️pets/🟦️.tsx:43`).
- UI: `PreferencesPanel` `:170-227` — each setting is `<div className="quiz-setting"><span className="quiz-setting-name …" aria-hidden>name</span><Segments label options value onChange/></div>`, e.g. the `others` chooser:
  ```tsx
  <Segments label={text("quiz.preferences.others")} options={OTHERS_CHOICES.map((choice) => ({ value: choice, label: text(OTHERS_CHOICE_LABELS[choice]) }))} value={preferences.others} onChange={(others) => onChange({ ...preferences, others })} />
  ```
  with `OTHERS_CHOICE_LABELS: {[C in OthersChoice]: QuizLabelKey}` `:53`. The label maps use exhaustive `{[C in X]: QuizLabelKey}` records; checkboxes use `className="quiz-check"`. Panel appears in `PreferencesPanelCard` (first visit beside the identity/introduction, and `PreferencesPage`).
- `Segments` (`M/🪟️chrome/🟦️.tsx:211`): `role="group"` + `aria-label`, one `<button aria-pressed>` per option (not a radio group), `Segment {value,label,short?,lang?}`, **no per-option description, no disabled state**; used by language (navbar, `compact` with `short` EN/DE), theme, text size, pets, others, leaderboard period and category (`M/🏆️leaderboard/🟦️.tsx:180-191`). For a chooser that must **explain each choice**, the identity step is the other precedent: a `fieldset.quiz-kinds` of real radios with `aria-describedby` hints (`M/🪪️identity/🟦️.tsx:107` `KINDS`, `:176-201` the fieldset, CSS `.quiz-kinds` 3 columns from `44rem`).

### 3.5 Navigation, trail, address (`M/🚏️navigation/🟦️.tsx`)
`QuizStep` run/results carry only `run: Id`; `placeName(step, state, locale, text)` `:22` names a run by its quiz title and results as `quiz.results.title`. `NavigationControls` (overview/back/forward/up) uses `aria-disabled` buttons that stay focusable. `stepAddress(step)` `:98` = `#<page>` only for home pages, `""` otherwise; `useAddress` `:106` replaces (never pushes) the hash. `stepAbove` (session `:253`) = quiz page above run/results. A level in a run could appear in `placeName` (e.g. "Quiz · Hard") but is **not** part of any address **[inferred: it needs no address]**.

## 4. i18n

- **Texts:** `QUIZ_BUNDLE_EN` (`M/🌐️i18n/🟦️.ts:56-436`) is a nested object, leaves `phrase("normal text")` = `{label:{normal, beginner}}` (`:53`, beginner defaults to normal); `QUIZ_BUNDLE_DE: typeof QUIZ_BUNDLE_EN` (`:439-819`) must have the same shape (compile error otherwise). Interpolation `{{name}}`; keys are `quiz.<group>.<name>` e.g. `quiz.run.submit`; groups in order: `app, nav, connection, preferences, introduction, identity, home, learner, quizPage, task, run, classification, sorting, matching, radar, results, leaderboard, crowd, presence, legal, rejection`.
- **No default language:** `preferences.locale` is `undefined` until chosen or preselected by `preferredLocale(browserLanguages)` (`:30`); `Client` is only rendered with a locale (`R/🟦️.tsx:527`); otherwise `LanguageChoice` speaks all languages. English first, German second (`QUIZ_LOCALES = LANGUAGES = ["en","de"]`, `Q/🧬️schema/🟦️.ts:31`). Learner content is `Text = {en, de}` via `localized(text, locale)`; chrome via `quizText(locale)` → `QuizText = (key: QuizLabelKey, values?) => string`.
- **Plural/number formatting:** there are no plural forms at all — counts are "Label: n" (test enforces). Numbers via `Intl.NumberFormat(locale)` (`formatNumber`, `formatScore` percent with `oneDecimal` so 99.96 never reads 100, `formatPoints` one decimal), dates via `Intl.DateTimeFormat`. German informal ("du"); one German word per concept (banned list in the test, "Rangliste" for leaderboard, "Lernende" for learners, "Durchgang/Durchgänge" in `rejection` texts).
- **Adding keys:** add to **both** bundles at the same position; use via `text("quiz.group.name", values)` as a *string literal* in a client source file (the test greps `"quiz.a.b"` literals; a computed key must be a literal in a map such as `OTHERS_CHOICE_LABELS`/`TASK_KIND_LABELS`/`PHASE_SPOKEN`/`PERIOD_TEXT`); a key that is unused or an unregistered literal fails the suite. Label records for enums are `{[K in Enum]: QuizLabelKey}` (exhaustive). `REJECTION_LABELS` is exhaustive over `REJECTIONS` — a new rejection (e.g. `level-unavailable`) needs a label.

## 5. Where scores are shown (every place)

| place | what | code |
|---|---|---|
| Results summary | run score as percent: `quiz.results.score` "Your score: {{score}}" | `M/🏁️results/🟦️.tsx:257` (`formatScore(result.score, locale)`) |
| Results, how everyone scored | `ScoreFigure` histogram, 10 bins `bin / n` as percent edges, own bin marked, caption `quiz.crowd.ownScore` | `:258`; `M/🗳️crowd/🟦️.tsx:401,404,418,428,436` |
| Results per task | `quiz.results.taskScore` "Task score: {{score}}" | `:286` |
| Results, classification | per item credit `({formatScore(credit)})` + `Verdict` | `:94` |
| Results, matching | per dimension `quiz.results.dimension` "{{quantity}}: {{score}}" | `:162` |
| Results badges | new badges list (`runAwards`) | `:239,260-277` |
| Quiz card/page facts | best score `quiz.home.best` "Best score: {{score}}" or `quiz.home.notYet` | `M/📖️quiz-page/🟦️.tsx:60` |
| Quiz page crowd | `ScoreFigure` with `own={learnerView.best[quiz]}` | `:129` |
| Learner card/page | total `quiz.home.points` "Points: {{points}}" (`learnerView.total`), facts played/badges/rank | `M/📇️profile/🟦️.tsx:41` |
| Learner runs table | per run `Score` column `formatScore(run.score)`, `Missing` when none | `:137` |
| Leaderboard card (centre of home) | `shown` columns rank, learner, `total` (label `quiz.leaderboard.total`, or `quiz.leaderboard.points` for a one-quiz board), badges; `BoardRow` `formatPoints(row.total)` | `M/🏆️leaderboard/🟦️.tsx:269-274,247` |
| Leaderboard page | columns from `leaderboardColumns` `:154` (rank, learner, total, **best score per quiz** as `formatScore(best)` `:355`, badges, runs, last submission); `PageRow` `formatPoints(row.total)` `:349` | `:332-370` |
| Sorting of tables | `SortHeading` `:194` buttons with `aria-sort`, `nextSort` `:50`, `sortValue` `:55` (`best:<quiz>`) | |
| Per-run points | **shown nowhere** (a run has a score only) | — |
| Crowd answer figures | counts/shares, not scores; the task score bins `scoreFigure` | `M/🗳️crowd/🟦️.tsx:263,445` |

Semantics today (core): `LearnerView.best[quiz]` = best score in [0,1]; `total` = Σ over catalog quizzes of `best × 100` (`Q/🔨️modules/👁️views/🟦️.ts:85`); `LeaderboardRow {rank, tag, identity, total, reachedAt, best, badges, runs, lastActivity}` (`Q/🧬️schema/🟦️.ts:369`); `Leaderboard.period ∈ daily|weekly|monthly|all-time`, optional `quiz`. **[inferred]** "points with a level multiplier" can reuse `formatPoints`; the run result needs a points field (percent stays `formatScore`); `best` per quiz would have to become "best points" (or best per level) to be consistent; the `crowd` score bins are computed over 0–1 scores and a level-mixed crowd would need a per-level crowd or normalised score.

## 6. Accessibility conventions and what a countdown must respect

- **Live regions:** `LiveRegion` (`M/🧩️task/🟦️.tsx:61`, `<p class="sr-only" role="status" aria-live="polite" aria-atomic="true">`, each announcement a new keyed `<span>` so repeats are spoken) fed by `useAnnouncement(delayMs)` (`:43`, debounced: select-driven tasks use `SELECT_ANNOUNCEMENT_DELAY_MS=400` so only where the item ended up is spoken). The **connection indicator** is the model for a *visible, constantly changing, never-live* value: visible text `<p data-tone>` not a live region (alternating saving/saved would chatter) **plus one separate polite `role="status"` that speaks only on entering an alert state and on recovery** (`R/🟦️.tsx:369-387`, test `Q/🧪️tests/📢️live-regions`). The submit dialog does the same (`aria-live="off"` visible body, separate `role="status" class="sr-only"` for phase starts, `M/▶️run/🟦️.tsx:238,246`). Errors: `ProblemNote role="alert"` (`M/🪟️chrome/🟦️.tsx:272`), guess error `role="alert"`.
- **Keyboard:** every task is operable with Tab/select/Enter/Space (test `Q/🧪️tests/⌨️task-keyboard`, 15 cases: classification select commits per arrow key → 400 ms debounced announce and focus follows the moved item; sorting buttons stay focusable at the ends via `aria-disabled` and announce why nothing moved; matching refuses to take a card from another item while browsing a select; pointer drag as an extra path). Unavailable controls use `aria-disabled` and stay in the tab order (nav ways, Submit, ↑/↓). Dialog traps Tab, Escape calls `onEscape`, returns focus to the opener, inerts siblings (`M/🪟️chrome/🟦️.tsx:298-379`). Skip link `R/🟦️.tsx:456`; the `<h1>` of each new step is focused (`:425-428`); run focus moves to the task heading on task change (`M/▶️run/🟦️.tsx:77-80`). `aria-keyshortcuts="Escape"` on the overview way.
- **Target size:** `.quiz-target` ≥ 24 px (`R/🎨️.css:178`), 2.5rem on coarse pointers inside `.quiz-task` (`:283-295`).
- **Reduced motion:** icons animate only when `data-icon-motion="on"` on `.quiz-app` (`effectiveIconMotion`: the learner's explicit choice wins, else on unless the device asks for reduced motion; `R/🟦️.tsx:409,455`); pets: same "device decides the default only" rule; `@media (prefers-reduced-motion: no-preference)` gates the column rise (`R/🎨️.css:993-998`); a blanket `@media (prefers-reduced-motion: reduce)` sets `transition-duration: 0.01ms !important` (`:1183-1189`); `QuizCard` uses `motion-reduce:` utilities. Memory note (verified in memory file, not in code): the owner's workstation is a Remote Desktop session, so browsers report reduced motion — anything *gated on* `prefers-reduced-motion` looks frozen there. **A countdown's ticking digits are not motion and must tick regardless; only decorative animation (a shrinking bar's transition, pulses) may follow the reduced-motion/`data-icon-motion` rules, and it must remain correct when frozen (text first).**
- **Forced colours:** `@media (forced-colors: active)` block `R/🎨️.css:1193-1244` (selected `aria-pressed`/`aria-current="step"` → `Highlight`/`HighlightText`, `.quiz-me`, `.quiz-drop-active`, `.quiz-column-bar`, peer labels, radar area, disabled → `GrayText`, `:focus-visible`). The test `Q/🧪️tests/🌗️contrast-states` reads the stylesheet with `lightningcss` and requires a forced-colours rule for **every state in the fixture `Q/🧫️fixtures/🌗️contrast-states/🔣️.json`** (`forcedColors[] {state, selector, properties}`) — a new background/shadow-painted state (level badge, low-time warning) needs both a rule and a fixture entry. Meaning is never colour-only (✓/○/◐/✗ `Mark` + word, lock/check icon, dashed vs solid frames).
- **Existing timers/clocks:** none for tasks. `useDocumentVisible()` (`M/👥️presence/🟦️.tsx:771`) marks a hidden tab as away (relevant: a timer must decide what hidden tabs do — **[inferred]** the proctor/deputy clock, not the visible tick, is the authority). The presence overlay uses a 500 ms `setInterval` + `requestAnimationFrame` (`:985-992`); pollers use `setTimeout` with jitter (`usePolling` `M/🏆️leaderboard/🟦️.tsx:110`, hidden tabs pause polling and resume on `visibilitychange`). The pets stage clock lives in `@semio-tech/pets-react` (`petsTempo()`, attribute `data-pets-tempo`, `M/🐾️pets/🟦️.tsx:93-96`) and is decoration. The session takes `now?: () => number` (default `Date.now`, `:621`) and the outbox `now` — **the injectable `now` seam is the precedent for testable time**; vitest tests also use fake timers in places (`⌨️task-keyboard` uses `vi`).
- **WCAG 2.2.1 Timing Adjustable** (Level A) is the standard a timed level meets by being an explicit, learner-chosen level (expert) **[inferred]**; consider also: a visible text remaining time, polite announcements at thresholds (not each second), no auto-focus steal at expiry, Submit confirm bypass at expiry.

## 7. CSS conventions (`R/🎨️.css`, 1243 lines)

- Plain CSS, **no `@layer`**; the design system's Tailwind-style utilities (`text-sm`, `gap-double`, `border-normal`, `bg-active-base`, `text-muted-foreground`, `p-double`, `size-workbench`, `max-md:sr-only`, …) carry colour/type/spacing; this sheet holds only what utilities cannot say (header comment `:1-5`).
- Class naming `quiz-*`: layout parts `quiz-app, quiz-page, quiz-pair, quiz-home-grid, quiz-home-cell, quiz-run-head, quiz-steps, quiz-step(-state), quiz-run-progress, quiz-task, quiz-rows, quiz-classify, quiz-bins, quiz-chip, quiz-sort(-place|label|up|down), quiz-guess, quiz-match, quiz-cards, quiz-value(-state), quiz-slot(s), quiz-records, quiz-fold, quiz-summary, quiz-result, quiz-settings, quiz-setting(-name), quiz-kinds, quiz-segment, quiz-figures, quiz-plot(-*), quiz-histogram, quiz-column(-cap|bar), quiz-radar(-*)`; atoms `quiz-target, quiz-prose, quiz-nowrap, quiz-name, quiz-glyph, quiz-label-icon, quiz-input, quiz-check, quiz-radio, quiz-progress, quiz-alert, quiz-note, quiz-me, quiz-online, quiz-footer, quiz-backdrop, quiz-drag-ghost, quiz-dragging, quiz-drop-active, quiz-peer*`.
- **Data attributes** are the state vocabulary: `data-card="<name>"` (QuizCard, e.g. `run`, `task`, `results`, `task-result`, `quiz:<id>`, `board`, `dialog`), `data-presence-anchor`, `data-icon-motion="on|off"` and `data-pets` on `.quiz-app`, `data-motion` on glyphs, `data-quiz-drag|drop|item|grip`, `data-used`, `data-earned`, `data-state="earned|locked"`, `data-complete`, `data-crowd`, `data-crowd-gate`, `data-tone` (calm|busy|alert), `data-board-*`, `data-head="sort"`, `data-cell="lead|name|note"`, `data-label`, `data-filled`/`data-emphasis`, `data-learning`.
- **Container queries over media queries:** query containers `quiz-card` (card content, `:266`), `quiz-task` (`.quiz-task`, `:270`), `quiz-rows` (`.quiz-rows`), `quiz-records` (`.quiz-records`, font-size trick `--quiz-fold/36` rem so `max-width: 36em` folds a table at its own width, `:604-611`); media queries only for the overview grid (768/1024), `pointer: coarse`, `prefers-reduced-motion`, `forced-colors`. Widths in rem so the learner's text size changes the layout.
- **Custom properties** used/defined: `--quiz-text-scale` (set on `:root` by `useRootTextScale`, `R/🟦️.tsx:354`), `--quiz-icon-order`, `--quiz-fold`, `--quiz-home-columns/rows`, `--quiz-plot-columns`, `--quiz-peer(-ink-light|dark)`; design tokens `--spacing-single|double`, `--ui-spacing`, `--active-base`, `--base`, `--foreground`, `--muted-foreground`, `--border-normal-color`, `--stroke-hairline|default`, `--color-warning`, `--color-secondary`, `--size-workbench`.
- Hint/notice boxes: `<div className="flex flex-col gap-single border-l-2 border-normal ps-double">` (sorting keep hint, `M/↕️sorting/🟦️.tsx:194`) and `quiz-note quiz-prose … border-l-2 px-double py-single text-xs` (identity, `.quiz-note` `:802`); errors `quiz-alert` (warning bar + tint).
- **Adaptive-layout ticket** (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-ADAPTIVE-LAYOUT/`): open; description "make the quiz client far more adaptive to width and device … an element that fits on one line uses left and right and breaks only where there is no room" (`🎫️ticket.json`; the ticket's `llm` is `opus-5-5`). Files: `layout_survey.ts` (a Playwright survey: `bun layout_survey.ts [origin] [case prefix] [label]` over 11 viewport cases phone-320 … desktop-1920 + `largest` text, walking first visit, identity, overview, each task kind open/answered, results and every home page; writes `🗑️generated/<label>/report.json` with wider-than-page, sideways scroll, cut text, small targets, task-row line counts), `report_digest.py`, `🗑️generated/{before,after1,after2}`. **What it is changing right now (git working tree vs index, read-only):**
  - `R/🎨️.css`: `@media (pointer: coarse)` extra rule for `.quiz-task .quiz-value > [data-quiz-grip]` (`:292`); the `.quiz-fold` thead hiding no longer applies to `.quiz-plot`; `min-inline-size: 5.5rem` for `td[data-label]` in folded records (`:676`); `.quiz-plot` folding block moved out of `.quiz-fold`'s `@container` into its own `@container quiz-records (max-width: 36em)` (`~:1079`).
  - `M/▶️run/🟦️.tsx:163`: the progress `<span>` loses `className="quiz-nowrap"`.
  - `M/🏆️leaderboard/🟦️.tsx:234-326`: `BoardRow` and the card table get `TABLE.*` roles, `data-cell`/`data-label`, and the card table is wrapped in `<Records fold={19} className="min-h-0 flex-1">` with `data-head="sort"`.
  - `M/🪟️chrome/🟦️.tsx:191`: `Records` gains a `className` prop.
  - Already staged in the index (earlier part of the same effort, listed `M ` in `git status`): `↕️sorting`, `🃏️matching`, `🗂️classification`, `🏁️results`, `🌐️i18n`, `🎛️preferences`, `📇️profile`, `🗳️crowd`, `🕸️radar`, `🤏️drag`, `🪪️identity`, `R/🟦️.tsx`, and `Q/README.md`; the sorting grid (`.quiz-sort`, `.quiz-guess`, container queries 36rem/56rem, `:419-493`) is the product of that work.
  - **Collision zones for the levels work:** `.quiz-sort`/`.quiz-guess` grid areas (adding a "key value" cell or a hint to a sorting row changes `grid-template-areas` that the adaptive session tuned and surveys), `.quiz-run-head`/`.quiz-run-progress` (where a level badge or timer next to the progress would go), `Records`/`TABLE` tables in results and leaderboard (adding a `Level` or `Points` column changes `fold` widths: `ResultTable fold={52}`, `Records fold` 19/44/46/68), `chrome/🟦️.tsx`, and the same `🎨️.css`. Edit these with the Edit tool on exact strings, never re-write the file, and rerun `layout_survey.ts` as the layout check.

## 8. Tests (names + what they cover)

`R/🧪️tests/🎚️config/🟦️.ts` (the vitest config, jsdom, aliases to the workspace packages) is the only thing inside the react target; **the cases live in `Q/🧪️tests/<name>/🟦️.ts(x)`** and the config's explicit `include` list (`R/🧪️tests/🎚️config/🟦️.ts:34-55`) names the React ones — **a new React test directory must be added there**; core ones are listed in `Q/🧪️tests/🎚️config/🟦️.ts`.
React (jsdom) suites registered: `📐️quantity-formatting` (shared vectors, `Intl.NumberFormat` oracle, parse round trips), `🕷️radar-geometry` (d3-scale/opentype oracles), `📬️outbox-delivery` (coalescing, retries, exactly-once, envelope), `🫡️deputy-decisions` (replays lifecycle vectors through core deciders and the Deputy; same verdicts/events/views), `⌨️task-keyboard` (15 cases, see section 6; uses a `Harness` around `ClassificationTaskView|SortingTaskView|MatchingTaskView`), `🚶️learner-journey` (22 cases: whole journey against a proctor double on the framework wire, several tabs, cancellation, deputy while the proctor is away, presence through the app), `🗣️translation-completeness` (section 4), `🏠️home-grid` (40 cases: pages order/cells, cards, polling, quiz card actions by run state, grid tracks), `🖼️task-icons` (glyphs aria-hidden, motions, `data-icon-motion`), `📡️presence-client` (23), `💭️crowd-client` (37: when the others show, figures, ask/hide), `🌍️language-choice` (7), `📢️live-regions` (connection indicator is not live; one polite status), `🌗️contrast-states` (peer ink contrast via colord; forced-colours rules from the fixture), `📇️learner-pages` (switch identity dialog, runs table headers, badge state), `🚦️rate-limits` (14: 429/Retry-After), `🎭️identity-step` (12), `🔏️privacy-notice` (4), `🐾️pet-companions` (33), `🚏️navigation` (16: trail vs jsdom history oracle, address, navbar).
Core suites (node): mt19937, sheet-randomization, document-validation, partial-credit-scoring, badge-awards, run-lifecycle, read-views, shared-vectors, presence-roster, crowd-answers; plus language-agnostic cases with `🥒️.feature` + `🐍️.py` + `🦀️.rs` (answer-validation, sheet-assembly, seeded-randomness, badge-rules, leaderboard, shared-presence, crowd-view, sorting-concordance, matching-concordance, profile-similarity, schema-conformance, learner-lifecycle, identity-shapes) with fixtures in `Q/🧫️fixtures/<case>/🔣️.json`. Any new feature in the AGENTS.md sense needs a language-agnostic case plus a third-party oracle.
Run: `bun nx run @semio-tech/quiz-react:test` (launch entry `🧪️test❓️quiz⚛️react`), `bun nx run @semio-tech/quiz:test`, `bun nx run @semio-tech/quiz-rs:test`, `bun nx run @semio-tech/quiz-react:typecheck`.

## 9. Extension points (concrete)

All entries **[inferred]** from the code above; each names the exact place.

**A. Level chooser before starting a run**
- The decision point is the click that calls `session.startRun` — only in `actionsOf` (`M/📖️quiz-page/🟦️.tsx:41`). Two placements: (1) the **quiz page** (`QuizPage`, `:147`; the tasks card at `:168` or a new `QuizCard` between `quiz` and `quiz-tasks`) hosts a chooser with room for an explanation per level; (2) the **overview card** (`QuizCardView` `:76`) is size-limited (`HOME_GRID_ROW_HEIGHT_PX = 160`, tests assert "the tallest card fits its cell" in both languages, `M/🏠️home/🟦️.tsx:83-97`, `Q/🧪️tests/🏠️home-grid`), so it should not grow a four-option control; it can start with the learner's remembered level or open the page.
- Component: `Segments` (`M/🪟️chrome/🟦️.tsx:211`) gives a 4-button toggle group but no per-option text/disabled and not a radio group; for explained levels use the radio-fieldset pattern of the identity step (`M/🪪️identity/🟦️.tsx:176-201`, `.quiz-kinds`/`.quiz-radio`) or extend `Segment` with `description?`/`disabled?`. Place the chosen level in `QuizProps` (`M/📖️quiz-page/🟦️.tsx:24`) / `actionsOf` and into `session.startRun(quiz, level, signal)` (`M/🧭️session/🟦️.ts:799`).
- A remembered default belongs to preferences: add `level: ChallengeLevel` to `QuizPreferences` (`M/🎛️preferences/🟦️.tsx:56`), validated in `readPreferences` (`:72`), a `LEVEL_CHOICES` array + `LEVEL_CHOICE_LABELS: {[L in ChallengeLevel]: QuizLabelKey}` like `OTHERS_CHOICES` (`M/🗳️crowd/🟦️.tsx:32`, `M/🎛️preferences/🟦️.tsx:53`), a `quiz-setting` row in `PreferencesPanel` (`:170`), `QuizPreferences` must reach `HomeScreen` → `QuizPage` (`preferences` already does, `M/🏠️home/🟦️.tsx:184`); the card's `QuizCardView` currently does not receive `preferences` (only `QuizPage` gets `others`).
- A resumed run (`openRunOf`) must show its fixed level instead of a chooser (`actionsOf` branch `open !== undefined`, `:44-48`). Add the run's level to `facts()` (`:55`).
- Cancellation/progress of start already exist (`act`, `HomeScreen :166-244`).

**B. Level badge during the run, in results, in tables**
- Run: the muted line of the task card `Task i of n · kind · state` (`M/▶️run/🟦️.tsx:193-195`) or the run card head (`.quiz-run-head`, `:133`) — next to the `<progress>`; both are in the adaptive session's zone. Data: `view.level` (new field of `RunView`, `Q/🧬️schema/🟦️.ts:340`).
- Results: summary (`M/🏁️results/🟦️.tsx:255-259`, `.quiz-summary`, first child `flex: 0 1 20rem`) — add level + points beside `quiz.results.score`; the title uses `placeName`/`quiz.results.title` (`M/🚏️navigation/🟦️.tsx:22-29`) for the navbar way names.
- Tables: learner runs table (`M/📇️profile/🟦️.tsx:RunRow`, add a `Level` column and keep `<Records fold={46}>` in sync), leaderboard (`PageRow` `:332`, `leaderboardColumns` `:154`, `LeaderboardKey`/`sortValue` `:37,55`, `BoardRow` `:234` — add a column key or fold the level into the points), `RunSummary` rows, quiz-page facts. Text by words and a symbol (`Mark`), never colour alone; if a coloured chip is used add it to the forced-colours rules + fixture (section 6).

**C. Easy-level hints (magnitude off by more than ×1000)**
- Sorting: `GuessField` has the parse result (`parsed`, `:79`) and `quantity` (`:65`); the comparison value must be available client-side (see fact 1) — render the hint through the same pattern as the existing preview/error: an element with an id referenced from the input's `aria-describedby` (`previewId`/`errorId`, `:81-82,126`), `role="alert"` only for an error, plain `text-xs text-muted-foreground` for a hint, plus an `announce(...)` through `useAnnouncement` for screen readers (`M/↕️sorting/🟦️.tsx:153,179`). New keys `quiz.sorting.hintOff…` in both bundles.
- Matching/classification hints: `MatchingTaskView.set` (`:42`) and `ClassificationTaskView.assign` (`:36`) are the single choke points where an answer changes and an announcement is made.
- All three views need a `level`/`hints` input: extend `TaskViewProps` (`M/🧩️task/🟦️.tsx:13`) and pass it from `RunScreen` (`:198`).

**D. Hidden keys / "learner guesses" input per kind (hard)**
- Sorting: already hidden (no values in the sheet); promote `GuessField` to required/prominent — the "Keep this order" block (`:193-198`) and `keepHint` text say a guess is optional, the place number/`quiz.sorting.guessHint` text would change per level. Guess is unscored today; a "hard" reward needs a scoring rule (core).
- Matching: hiding the card values needs a new input type — today answers are card indices (`MatchingAnswer`), and `cardText` (`:40`) shows values in 3 places (card list `:75`, option text `:109`, announcements `:46`). A hidden-key variant would show only neutral card labels or a numeric field per item and map a typed number to the nearest card or change the answer shape.
- Classification: the only "keys" are the category profiles (`RadarChart` on `category.profile`, `:104`) and descriptions (`:103`); hiding them is a render switch there.
- Results: `SortingResult` already adds the guess column; add per-row magnitude error (`|log10(guess/value)|`) there for hard/expert.

**E. Countdown for expert**
- Container: the task card (`M/▶️run/🟦️.tsx:174-205`) top line or the run card head; per-task key `key={task.id}` already remounts per task. A `useCountdown(deadlineMs, now)` hook should follow the `useAnnouncement`/`usePolling` pattern (own file section in `🧩️task`/`▶️run`, `setTimeout`/`setInterval` with cleanup, injectable `now` like `QuizSessionOptions.now` `:478`, pause nothing on `visibilitychange` — the deadline is an absolute instant).
- Authority: the deadline must be derivable from the run/task start recorded by the proctor/deputy (`startedAt`, or a new "task opened" event) — `RunView` today has only `startedAt`. Expiry must lock the task (`disabled`/`readOnly` prop, none exists today) and, if per-run, auto-submit through `submit()` (`:98`) bypassing the confirm dialog; `session.submit` already handles phases/cancel.
- A11y: visible `mm:ss` text (not a live region, like `ConnectionStatus`), a separate `role="status"` sr-only that speaks at thresholds and at expiry, a `<progress>`/`quiz-progress` for the visual only (decoration, not motion-gated for correctness), forced-colours rule for a "low time" state, `tabular-nums` (`.quiz-input` family already uses it), text-size and width via the `quiz-task` container, no focus steal.
- Presence: the thinking room shares drafts (`presenceDrafts`) — decide whether expert drafts are shared at all; the `others` gate (`M/🗳️crowd/🟦️.tsx:46`, `crowdGate("run")`) must return `off`/`locked` and `CrowdDoor` (`M/▶️run/🟦️.tsx:200`) not offered for hard/expert.

**F. Cross-cutting edits a level needs (checklist)**
1. Schema `Q/🧬️schema/🔣️.json` + TS twin (`StartRunCommand`, `RunStartedEvent`, `RunView`, `RunSummary`, `RunResult`/points, `LeaderboardRow`/`LearnerView`, `Limits`?), Rust twin; `REJECTIONS` if a level can be refused.
2. Core: `startRun` decider (`Q/🔨️modules/🧾️lifecycle/🟦️.ts:81`), `RunState`, `evolveLearner` `run-started` (`:178`), `sheetOf(quiz, seed[, level])`, `scoreRun` multiplier, `earnedBadges` (badges "perfect-quiz": per level?), `views` (`best`, `total`, `leaderboard`, crowd).
3. React: `session.startRun`, `outbox.restoredCommand` start-run branch, `deputy.run()` (level + sheet check), `restoredRun`, `evolveQuizState` untouched (views carry the level), `TaskViewProps`, `RunScreen`, `ResultsScreen`, tables, `QuizPage`/`QuizCardView`, preferences, i18n (both bundles), CSS + forced-colours + fixture, `R/🟦️.tsx` reexports, test dirs registered in the vitest `include`.
4. Tests to touch: `deputy-decisions` (shared lifecycle vectors with levels), `learner-journey`, `home-grid` (card actions by run state), `task-keyboard` (new hint/field behaviours), `translation-completeness` (automatic), `contrast-states` fixture, `quantity-formatting` (if points formatting changes), core lifecycle/scoring/leaderboard + Python reference vectors.

## 10. Verified vs inferred

- **Verified by reading** (file and line cited): sections 1–8 and the quoted fragments; the git working-tree vs index diff of `▶️run`, `🏆️leaderboard`, `🪟️chrome`, `🎨️.css`; the ticket files of QUIZ-ADAPTIVE-LAYOUT, QUIZ-CHALLENGE-LEVELS (ticket.json only) and QUIZ-SORTING-NUMERIC-GUESSES (`📓️design.md`, the contract of the guess field).
- **Inferred** (marked **[inferred]**): everything in section 9 and the remarks on the deputy's needs (3.3), points semantics (section 5), the logarithmic nature of the existing sorting quantities (taken from the ticket `QUIZ-SORTING-NUMERIC-GUESSES/📓️design.md`: "today: `powers` and `energies` … both logarithmic and SI-prefixed", not from the quiz JSON), and WCAG 2.2.1 applicability.
- **Not examined:** the Rust twin, the proctor service, the teaching site's catalog JSONs, the `ui-react` chrome internals (`Navbar`, `OverviewCard`), the pets stage clock source, the e2e tests under `🎓️teaching/…/🎭️e2e`. I did not run any test or the dev site; no runtime behaviour was confirmed.
