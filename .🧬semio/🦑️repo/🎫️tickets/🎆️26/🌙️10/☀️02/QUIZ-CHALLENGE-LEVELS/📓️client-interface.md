# Client interface — state, wire and page layer → run screen and task views

Written by the client-state agent before implementing, so the run/task agent can rely on it. `R` =
`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`. Everything below is reexported from `R/🟦️.tsx`
(package `@semio-tech/quiz-react`). If something here changes while implementing, this file is updated in place.

## 1. Session (`M/🧭️session/🟦️.ts`, class `QuizSession`)

```ts
startRun(quiz: Slug, challenge: Challenge, signal: AbortSignal): Promise<SessionFailure | undefined>
openTask(run: Id, task: Slug, signal: AbortSignal): Promise<SessionFailure | undefined>
answer(run: Id, task: Slug, answer: Answer): void          // stamps `at` = this.now()
now(): number                                              // the session clock (QuizSessionOptions.now, default Date.now)
```

- `startRun`: an open run of the quiz at the same challenge is resumed (`run-open`); at another challenge the core voids it
  (no notice is raised for that switch) and the new run opens. The `act` wrapper of home shows progress and cancel.
- `openTask`: decided like `start-run` — by the proctor, or by the deputy once the proctor is known to be away or the
  learner's patience (`DEPUTY_PATIENCE_MS`) is spent. A task already opened in the held view resolves `undefined` at once
  without a command. On success `state.runs[run].opened[task]` is set before the promise resolves (from the proctor's
  `task-opened` event, or from the deputy's view). Failures: the same `SessionFailure` as every other action
  (`rejected` with `run-untimed`, `unknown-task`, `run-closed`, `quiz-revised` …, `refused`, `waiting`); `quiz-revised`
  also voids the run (home + notice, like an answer), `run-closed`/`unknown-run` reload the run. Cancel with the signal.
- `answer`: applies at once, queues `record-answer {…, at: now()}`. On a timed run send answers only for opened tasks whose
  time is not up (the proctor/deputy rejects otherwise: `task-unopened`, `time-up` → a notice through the existing notice
  path, texts in `REJECTION_LABELS`).
- `now()`: use it for every countdown: `remaining = opened[task] + sheetTask.seconds * 1000 - session.now()`. Tests drive it
  through `new QuizSession({ …, now })`.

## 2. State (`state.runs[run]`, a `RunView`)

- `sheet.challenge` always present; `challengeRules(view.sheet.challenge)` (core, `@semio-tech/quiz`) says `keys`, `hints`,
  `timed`, `par`.
- `opened?: {task: Timestamp}` — present on timed runs (also `{}`), kept current after every `openTask`.
- `hints?: {task: Hint[]}` — present only while the run is open on easy and some hint exists; kept current after every
  answer: with a deputy (site material present) the session recomputes the hints through the deputy right after the
  answer; without one it re-reads the run view once the answers of an easy run are delivered (when no answer of that run
  waits any more). A task without hints has no entry. Hints of a run whose held sheet is not the material's are not
  recomputed (the proctor's view brings them).
- Stored runs without `sheet.challenge` are not restored.

## 3. Crowd gate (`M/🗳️crowd/🟦️.tsx`)

```ts
crowdGate(others: OthersChoice, place: CrowdPlace, facts: { asked: boolean; submitted: boolean; hidden: boolean }): CrowdGate
keysHidden(view: RunView): boolean   // view.status === "open" && !challengeRules(view.sheet.challenge).keys
```

`hidden` makes the gate `off` at `run` and `quiz` (never at `results`). The run screen must pass
`hidden: keysHidden(view)`:
`crowdGate(props.others ?? "submitted", "run", { asked: state.asked.includes(view.quiz), submitted: false, hidden: keysHidden(view) })`;
the results pass `hidden: false`. The quiz page passes whether the quiz has an open run that hides the keys.

## 4. Challenge texts (`M/⛰️challenge/🟦️.tsx`, new React module)

```ts
CHALLENGE_LABELS: { readonly [C in Challenge]: QuizLabelKey }   // quiz.challenge.easy|medium|hard|expert
CHALLENGE_HINTS:  { readonly [C in Challenge]: QuizLabelKey }   // quiz.challenge.easyHint … (placeholder {{par}})
challengeScored(challenge: Challenge, points: number, text: QuizText, locale: QuizLocale): string
  // "Hard · Points: 261 of 300" / "Schwer · Punkte: 261 von 300" (phrase quiz.challenge.scored, formatPoints)
ChallengeChooser(props: { challenge: Challenge; onChange(challenge): void; text: QuizText; disabled?: … })  // the page's fieldset
```

EN names: Easy, Medium, Hard, Expert; DE: Leicht, Mittel, Schwer, Experte. The no-plural rule of
`🗣️translation-completeness` forbids "{{par}} points", so points always read "Points: 261 of 300". For the results
summary use `challengeScored(result.challenge, result.points, text, locale)` beside `formatScore(result.score)`.

## 5. Rejections

`REJECTION_LABELS` (`M/🌐️i18n/🟦️.ts`) covers `run-untimed`, `task-unopened`, `time-up` (keys `quiz.rejection.runUntimed`,
`taskUnopened`, `timeUp`).

## 5a. Requests to the run/task agent (your files)

- `R/🎨️.css`: the chooser is `<fieldset class="quiz-kinds" data-options="4">`; `.quiz-kinds` has 3 columns from 44rem,
  which leaves 3 + 1. Please add e.g. `@container quiz-card (min-width: 44rem) { .quiz-kinds[data-options="4"] {
  grid-template-columns: repeat(2, minmax(0, 1fr)); } }` and 4 columns from ~64rem.
- After a `time-up` or `task-unopened` rejection of a delivered answer the session raises the notice and re-reads the
  run, so the rejected answer disappears from `state.runs[run].answers`.
- The deputy never opens a task of / submits a run it knows only from the learner view: `openTask` loads the run view
  first, and such a command waits for the proctor (cancellable).

## 6. Labels other layers see (site e2e)

Primary actions of a quiz card/page name the challenge: "Start (Medium)", "Again (Medium)", "Resume (Hard)"; DE
"Starten (Mittel)", "Nochmal (Mittel)", "Fortsetzen (Schwer)". The best fact reads "Best: 200 of 200 (Medium)" /
"Bestwert: 200 von 200 (Mittel)"; the learner total stays "Points: n"; the navbar names a run "Quiz (Medium)". The quiz page's chooser is a radio group named
"Challenge" / "Herausforderung" with radios named by the challenge names. Choosing another challenge than the open run's
and pressing the start action opens an alert dialog "Discard the open run?" with "Discard and start" / "Keep the open run".
