# Audit — React client state, wire and pages of the challenge levels

Auditor: read-only (no repo file touched except this report). `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`,
`M` = `R/🔨️modules`, `S` = `M/🧭️session/🟦️.ts`, `L` = `🧰️framework/🛍️products/❓️quiz/🔨️modules/🧾️lifecycle/🟦️.ts`.
Normative: `📓️design.md` §1, §2, §3.5, §5 with the 2026-10-03 revisions (`acted(at, floor)`, `reach` as a factor).

## Gates run (once each)

- `bun ./📜️script.ts test` in `R/📦️packages/🟦️typescript`: 22 files, **749 passed, 0 failed**.
- `bun ./📜️script.ts typecheck`: 14 errors, **all** in `🐾️pets/🎯️targets/⚛️react/🔨️modules/🤏️grasp/🟦️.ts` (pets session); 0 in quiz files.
- Reproductions: a vitest spec outside the repo (scratchpad `repro.spec.ts` + `vitest.audit.config.ts`, reusing the journey
  suite's `FakeProctor`), run with `bunx vitest run --config <scratchpad>/vitest.audit.config.ts --reporter=verbose
  --disable-console-intercept` from `R/📦️packages/🟦️typescript`. R1–R6 below are those cases; "reasoned" marks findings
  derived from code only.

## Findings, ranked

### Bugs

**B1. Stale hints come back after a re-read while a newer answer is pending (confirmed, R2).**
`S:958-960` (`withPending`) overlays the outbox's pending answers on a freshly read view but keeps the proctor's `hints`,
which belong to the older answers. With a deputy nothing repairs it: `answerSettled` re-reads only `deputy === undefined`
(`S:1268`), and `answer()` recomputes hints only on the next answer (`S:892-894`). Repro: deputised easy run, sorting
answered wrong (delivered, hints shown), `outage` on, answered right (hints cleared, `[AUDIT-R2]`), `session.open(run)` →
`loadRun` → answer is the correct one **and** the hints of the wrong one are shown beside it, until the next answer. Same path
in `restored()` (`S:1043`), `overlayPending` (`S:962-968`) and `adoptRun` (`S:1023-1030`, other tab's stored hints with
unioned answers). Fix: whenever `withPending` changes an answer of a hinting run, recompute the hints (deputy) or drop them.

**B2. A run re-read that started before a newer answer was delivered overwrites that answer (confirmed, R1).** New
re-read-after-every-delivered-answer on easy (`S:1268`) widens an old race in `loadRun` (`S:943-956`): the read's snapshot is
older than an answer given and delivered while it flies, and `withPending` no longer has it (it left the outbox). Repro with
the run query held: A1 delivered → read#1 starts; A2 given and delivered → read#2 starts; releasing read#1 reverts
`answers[task]` to A1 with A1's hints (`[AUDIT-R1]`), read#2 restores A2. Visible as flicker and, worse, a lost update when
the learner acts on the reverted order in between. If the proctor's projection lags (the double's `PROJECTION_LAG` models it
for fresh views) read#2 can be stale too and nothing re-reads again. Fix: a generation counter bumped by `answer()`/delivery;
ignore a read that began before the latest bump, or merge answers by `at`.

**B3. A stored learner view of the old shape is restored and breaks rendering (confirmed shape, R6; crash reasoned).**
`restoredLearnerView` (`S:425-427`) checks only `learner`, `runs`, `badges`. `best: {quiz: 0.9}` and runs without `challenge`
survive `restoreQuizState` (`[AUDIT-R6]`). `quiz-page/🟦️.tsx:68` (`challengeRules(best.challenge).par`), `📇️profile:138,144,157`
and `CHALLENGE_LABELS[run.challenge]` then dereference `undefined` (TypeError on home). §5 says stored runs without
challenge are not restored; the learner view needs the same rule (every run has `challenge`, every `best` entry has
`challenge/score/points`), or the whole view must be dropped.

### Offline/online divergence

**D1. A timed run started *and* opened on the device regains its time after sync (confirmed, R3).** The deputy starts the run
at device time t0; the proctor's `run-started.at` is its arrival time (`L:121`), and it is the floor of `open-task`
(`L:134`). So the proctor's `opened[task]` is the delivery time, not the device's. The re-read replaces the device's
`opened` (`S:1252`, `run-loaded`), and the run screen derives its deadline from `view.opened` (`▶️run:205-206`). Observed
(`[AUDIT-R3]`): started 1760000000000, opened +5 s, 40 s allowed, 20 s after the deadline the device says "time is up"; after
delivery `opened` = proctor start (+73 s) and 45 s are left again, so the learner may answer a task the device had closed.
The verdict itself is never stricter at the proctor (README "same floors" holds for the answer's instant, not for `opened`),
but the README claim at 745-749 and design §1 "the device keeps the time" are not true for the view. The shipped test
(`🚶️learner-journey` "opens the tasks ... while the proctor is away") starts the run **online**, which hides this. Fix: on
adopting a proctor view of an open timed run keep `min(held, loaded)` per task in `opened`, or carry the start instant in
`start-run`.

**D2. `mergeRunViews` drops the older tab's `opened` (confirmed, R5; consequence reasoned).** `S:408-412` returns
`{...theirs, answers: union}`; `[AUDIT-R5]`: merging `{a}` with `{b}` gives `{b}`. Two tabs opening different tasks at the
same moment each lose the other's opening; the tab then shows "Start the clock" for a task it opened, and `openTask`
(`S:866-868`) re-sends; the deputy's `decide` sees no `opened[task]` and emits a second `task-opened` whose later `at`
overwrites the first in `evolveLearner` (`L:213-214`), granting extra time on the device. Fix: union `opened` keeping the
earliest per task.

**D3. A rejected answer is not re-read while the device is ahead for another run (reasoned).** `answerSettled` calls
`loadRun(run)` (`S:1275`), which returns the held view when `ahead()` (`S:945`); only runs of delivered *decisions* enter
`unconfirmed` (`S:1235`), so the run whose answer got `time-up`/`task-unopened` keeps showing the rejected answer until it is
reopened. Fix: add the run to `unconfirmed` in `answerSettled`.

**D4. `open-task` rejected `quiz-revised` at delivery does not void the run (reasoned).** `decisionSettled` only raises a
notice (`S:1249`), whereas `answerSettled` voids (`S:1272`) and interactive `openTask` voids (`S:878`).

**D5. Coalescing keeps the last edit's `at` (reasoned).** Offline, an in-time answer superseded by a later edit that is past
the deadline loses the in-time one at the proctor (`time-up`), online both would be delivered one by one and the in-time one
survives. Not reachable through the run screen (`▶️run:210` guards by `session.now() < deadline`), but `S:886-895` has no
guard of its own for an unopened or timed-out task; the guard belongs in `answer()` next to the stamp.

Checked and fine: switching challenge voids the old run and drops its queue on both paths (`S:841-845`, `S:1239-1243`;
journey test + code); `open-task` before its answers in the outbox (queued-at order; R4 and the shipped test); idempotence of
`openTask` (`S:866-868`, proctor `events: []` → re-read `S:874`); reload between a deputised `open-task` and its first
answer (R4: `opened` and the queued command survive, the answer is delivered, no notice); `RunState` rebuild
(`🫡️deputy:160-184`: challenge from view else summary, `opened` from view, `sheetOf(quiz, seed, challenge)` comparison via
`alike`, no `undefined`-valued keys so the JSON round trip keeps it equal); time verdicts never consult the decider's clock
(`L:146-149`); no re-read loop (a read triggers no command).

### Accessibility

No defect found in the chooser: fieldset + legend, four native radios of one name (arrow keys), label = challenge name, par
points in the `aria-describedby` line (`⛰️challenge:27-52`). Discard dialog is the shared `alertdialog` (focus on "Keep",
Escape closes, focus returns, background inert). Minor: `ChallengeChooser` has no `disabled` while the start is busy, though
`client-interface.md` §4 lists `disabled?`; choosing a radio changes the primary action label without any announcement.

### Design deviations

- `RunView.hints`/`opened` linger after `run-submitted`/`run-voided` in the held view (`S:341-349`), §2 says hints exist only
  while open; the results screen ignores them, the next read clears them. Low.
- Crowd own-mark for a guess (`🗳️crowd:211-237`) re-implements the core's `nearestValue` over the *figure's keys* (cards ∪
  crowd keys ∪ thinking keys), not over all authored values (§3.6). It agrees with the tally once the learner's own run is in
  the crowd; before that refresh it can mark another column. The nearest-value rule should come from the core (export it).
- The results page keeps the crowd open (`🏁️results:272`, `hidden: false`) while a hard/expert run of the same quiz is open;
  §20 speaks of runs and the quiz page only, so this matches the text but shows authored values of the quiz. Judgement call.
- `isTimestamp` (`💾️persistence:196`) repeats the core's validation predicate (`✅️validation:495`); acceptable, but should be
  exported once from the core.

### Coverage gaps

No test for: a run started offline and opened offline (D1); a re-read with a pending newer answer and hints (B1); overlapping
reads (B2); `mergeRunViews` with `opened`/`hints` (D2); a stored learner view of the old shape (B3); an answer rejected while
ahead (D3); `open-task` rejected at delivery (D4). Persistence of `preferences.challenge` default/validation (`🎛️preferences:90`)
is correct by reading; the journey suite covers restore of runs only.

### i18n and repo rules

EN/DE complete for every new key (diff of `🌐️i18n/🟦️.ts` read; German uses *du*, no plural forms: "Punkte: n von m",
"Elemente in der falschen Kategorie: n"); no hard-coded learner text found in the files read; no comments inside
definitions; no logic duplicated from the core except the two items above. Style nits: new docstrings repeat an emoji
already used in the same file — `S:1261` (📬️, also `S:1229`), `📖️quiz-page:36` (🎬️, also `:22`), `🎛️preferences` (🔁️ twice,
`:100`, `:106`); many older repeats exist, so the rule is not enforced file-wide. `🗳️crowd:332` reads `marked =(mark` (from the
adaptive-layout session, not this change).
