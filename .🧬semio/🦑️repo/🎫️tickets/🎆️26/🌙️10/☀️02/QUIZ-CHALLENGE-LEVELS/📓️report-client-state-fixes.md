# Report — fixes of the client-state audit

Agent: client-state fixes (`📓️audit-client-state.md` B1–B3, D2–D5, design deviations, style; `📓️audit-client-run.md` #2).
`R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`, `T` = `🧰️framework/🛍️products/❓️quiz/🧪️tests`.

## Changed files (all updated, none created or removed)

- `M/🧭️session/🟦️.ts` — the fixes below; every docstring emoji in the file unique now.
- `M/🗳️crowd/🟦️.tsx` — own mark and correct mark of a matching through the core's `nearestOf`.
- `M/📖️quiz-page/🟦️.tsx`, `M/🎛️preferences/🟦️.tsx` — docstring emojis unique (no code change).
- Core, small hunks: `🔨️modules/👁️views/🟦️.ts` and twin `🦀️.rs` — new exported `nearestOf(values, scale, guess)` /
  `pub fn nearest_of`; `nearestValue`/`nearest_value` delegate to it (TS `nearestValue` is exported now, as the Rust one was).
- Tests: `T/🚶️learner-journey/🟦️.tsx` (FakeProctor takes material and can `revise` a quiz; 10 new cases, one case
  rewritten), `T/💭️crowd-client/🟦️.tsx` (1 new case), `T/🫡️deputy-decisions/🟦️.tsx` (followed-sequence check made an
  `arrayContaining`, since the contract agent keeps adding lifecycle sequences).

## What was fixed

| Finding | Fix (session unless named) | Regression test (journey unless named) |
|---|---|---|
| B1 / run-audit 2 | `answer-given` drops the hints of its task; `withPending` drops the hints of every task whose answer it overlays; with a deputy `rehint(run)` recomputes after every change (answer, overlay, read, other tab, restore), without one the hints wait for the proctor's view | "drops the hints of a task the moment its answer changes … (no deputy)", "has the deputy tell the hints of the answers shown …" |
| B2 | `loadRun` notes, when asked, the tasks with a waiting answer and an edit counter; on return those tasks and every task answered since keep the held answer and hints (`keptFresh`); a read overtaken by a later read of the same run is dropped (ticket per read) | "never lets a read … bring back the older answer or its hints, and drops a read that a later one overtook" (held transport) |
| B3 | `restoredLearnerView` requires `best` to be a record of `Best` and every run summary to carry a `Challenge`, else the whole view is dropped | "restores a stored learner view only in the shape of the challenges …" |
| D2 | `mergeRunViews`: `opened` = union, earliest per task; hints per task from the copy whose answer is kept | "merges another tab's copy of an open run …" (pure) |
| D3 | `loadRun` asks `behind(run)` (a waiting deputy decision about this run, a start of a run of its quiz, or a registration) instead of the global `ahead()`; after `time-up`/`task-unopened` the run is read at once, or queued in `unconfirmed` while behind | "reads a run again at once after the proctor refused one of its answers, while the deputy's start of a run of another quiz still waits" (two quizzes) |
| D4 | `decisionSettled`: `open-task` rejected `quiz-revised` → `unconfirmed.delete` + `voided(run)`, the path of `start-run` | "voids a run on the device when the proctor refuses an opening the deputy decided because the quiz was revised" |
| D5 | `answer()` stamps `at` once and refuses (no state change, nothing queued, `time-up` notice) when `acted(at, opened) − opened > seconds·1000`, exactly the deciders' rule; unopened tasks still go to the deciders | rewritten "keeps the clock of an expert run …" (answer at the deadline accepted, one after it refused without an envelope); "delivers an answer replaced while it waited with the instant of its own last edit" (verification: the outbox already keeps the last command whole) |
| Lingering hints | `run-submitted`, `run-voided` and `closedRun` (learner listing) drop `hints`; `opened` stays — the core's `runView` keeps `opened` on every timed run and gives hints only while open (checked, `👁️views/🟦️.ts:136,147`) | "keeps no hints on a run once it is submitted, voided or closed by its listing …" (pure) |
| Crowd own mark | candidates = cards ∪ true values of the result ∪ submitted crowd keys (all authored values); own and correct = `nearestOf(candidates)`; true values are columns now, so ✓ never lands on another value | crowd-client "marks a guess of a result where the crowd will tally it …" (oracle: core `crowdView` tally + lodash `minBy`) |
| Emojis | all repeats in `🧭️session`, `📖️quiz-page`, `🎛️preferences` replaced (probe `docstring_emoji_probe.ts`: none left, also `🗳️crowd`) | — |

## Gates (in `R/📦️packages/🟦️typescript` unless named)

- Baseline before any change: `bun ./📜️script.ts test` — 22 files, **761 passed, 8 failed** (run-layer i18n mid-change,
  deputy vector list).
- Core contract state while I worked: the core already requires `StartRunCommand.at` (`🧬️schema/🟦️.ts:287`) and has a new
  `already-opened` rejection. Per brief I did not implement that. To verify my fixes at runtime I added `at` **temporarily**
  (session `startRun`, 3 journey seeds, 3 deputy-test literals), ran, and removed it again:
  - `bun ./📜️script.ts test`: **792 passed, 6 failed** (798) — 5 deputy-decisions (`id-invalid` in vectors/literals,
    `already-opened`, new vector ids), 1 translation-completeness (`already-opened` has no label). All of journey (incl. the
    10 new cases) and crowd-client green.
  - Mutation check (old behaviour restored temporarily in session + crowd, then the fixed files copied back, `cmp`-verified):
    `test learner-journey crowd-client` → **10 failed / 176 passed**: exactly the 10 new/rewritten regression cases; the
    coalescing case passes before and after (verification only).
- Final tree (no temporary patch): `bun ./📜️script.ts test` — 22 files, **767 passed, 34 failed** (801): 30 journey + 2
  deputy-decisions (start-run without `at` → `id-invalid`), 1 translation-completeness (`already-opened`), 1 deputy vector
  list (fixed afterwards; deputy file rerun: 24 passed, 2 failed, both `id-invalid`). All red is the pending contract change.
- `bun ./📜️script.ts typecheck`: **25 errors** — 9 in `🐾️pets` (`🎥️projection`, `🎪️stage`, `🎯️choice`, `👥️population`,
  `🕰️clock`; pets session, not touched); 16 from the contract change: `🧭️session:886` and 14 test literals
  (`📬️outbox-delivery` 11, `🚶️learner-journey` 3 seeds) lack `start-run.at`, `🌐️i18n:941` lacks `already-opened`. None from
  my code.
- Rust: `bun ./📜️script.ts test crowd` in `🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust` — compiled, **4 passed**
  (views crowd tests, incl. the `nearest_value` unit cases).

## Not done / open

- **D1 (offline start time)** — left to the contract change (`start-run.at`, design §2 rev. 2026-10-03): a run started and
  opened on the device takes the proctor's arrival time as start/floor, so after sync the proctor's `opened` replaces the
  device's and the run screen's deadline moves later. Once `run-started.at` is the device's claim, the deputy's and the
  proctor's `opened` agree; the journey test "opens the tasks … while the proctor is away" should then also start the run
  offline (R3 of the audit is the reproduction).
- The contract agent must: add `at: this.now()` in `QuizSession.startRun`, require `isTimestamp(at)` for `start-run` in
  `📮️outbox` `restoredCommand`, add `at` to the start-run literals in `📬️outbox-delivery`, `🫡️deputy-decisions`, the journey
  seeds, handle `already-opened` (label in `REJECTION_LABELS`; `openTask` should treat it like an opening already held:
  read the run, resolve `undefined`).

## Deviations and decisions

1. Unopened timed tasks are not refused in `answer()`: another device may have opened the task, which the held view may not
   know yet; the deciders judge it (`task-unopened` → notice + re-read). Only the deadline is refused locally, and only when
   the held opening says so (never more lenient than the deciders: a held opening is never earlier than the proctor's).
2. B2 merges instead of discarding a read: a stale read may still carry a closed status or another device's answer to
   another task; only answers the read cannot hold are kept local.
3. `behind(run)` treats any waiting `start-run` of a run whose quiz is unknown as concerning it (conservative).
4. Crowd: matching columns now include the result's true values (on hard/expert they had none), so the ✓ is on its own value;
   a guess is marked under the nearest authored value the client knows (cards, true values, tallied keys). Before the
   learner's own run is tallied this equals the tally whenever the nearest authored value is among them (always, for
   presented items).
5. `nearestOf` added to the core (both twins) rather than a client copy, so the rule exists once.

## Notes for the next agent

- New session internals: `unhinted(view, dropped?)`, `keptFresh(loaded, held, fresh)`, `overdue(view, task, at)` (module),
  `QuizSession.rehint(run, persist?)`, `rehintAll()`, `behind(run)`; fields `edits/edited/reads/adopted`.
- Core: `nearestOf(values: Iterable<number>, scale, guess): number | undefined`, `nearestValue` exported (TS);
  `nearest_of(impl IntoIterator<Item = f64>, Scale, f64) -> Option<f64>` (Rust).
- Journey `FakeProctor(material?)`, `proctor.revise(quiz)`, `heldTransport(proctor)`, `TWO_QUIZZES`/`KITCHEN`, `heldRun()`.
- Tool output of this agent was under `🗑️generated/state-fixes` and is deleted.
