# Audit — React run screen, task views, results and styles of the challenge levels

Read-only audit against `📓️design.md` §1, §3.1, §3.3, §5, and WCAG 2.2 AA. `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`,
`M` = `R/🔨️modules`. Code read first, the agent's report (`📓️report-client-run.md`) last.

Gates run once: React suite `bun ./📜️script.ts test` — **749 passed (749)**. React typecheck — **15 errors, all in
`🐾️pets/📦️packages/🟦️typescript/🟦️.ts` (duplicate re-exports `Canopy`, `Grip`, `Tier`, …)**, none in the quiz client.
A probe (`parseQuantity`, `formatCountdown`, `formatFactor`) was run from the scratchpad; no repo file was touched.

## Verdict

The design is implemented faithfully and tested. Nothing breaks the contract of §5. The clock, the gate, the guess flow, the
hints and the results behave as specified; timers are cleaned up; nothing in the clock depends on reduced motion; focus is never
moved at time up. The findings below are the real gaps.

## Findings (ranked)

### Bugs

1. **Typed guesses with common SI prefixes are rejected; a lone `M` on a metre quantity reads as metres** —
   `M/📏️quantity/🟦️.ts:98-105` (prefix table), `:134-140` (`typedExponent`). Hard and expert make a guess the only way to
   complete a sorting or matching task, so this now blocks the answer path.
   Reproduction (probe, `{unit:"m", prefixed:true, scale:"logarithmic"}`, locale `en`): `"30 cm"` → `undefined` (alert "Enter a
   positive number, optionally with an SI prefix…"); `{unit:"Pa"}` `"100 hPa"` → `undefined`; `{unit:"m"}` `"5 M"` → `5`
   (`"m".endsWith("m")` after lower-casing, so the prefix is dropped) while `"5 Mm"` → `5000000`. `da`, `h`, `d`, `c` are valid SI
   prefixes but missing because the table only holds multiples of three. Fix: add `h`, `da`, `d`, `c`; compare the unit case-sensitively
   (a lone prefix letter is a prefix when it differs in case from the unit); keep the formatter on multiples of three.
2. **A hint is shown for an answer that is no longer the answer, when no deputy decides** — `M/🧭️session/🟦️.ts:332` (`answer-given`
   keeps `view.hints`) with `M/🧭️session/🟦️.ts:886-895` (hints refresh only through a deputy or after delivery). Reproduction: easy
   run, no deputy; move Horse to its correct place — the "Far too low" note stays beside Horse until the proctor's re-read arrives
   (and a hint that should appear for another item shifted by the move is missing meanwhile). Also `M/🧩️task/🟦️.tsx:239-254`
   (`useHintAnnouncement`) then announces the stale-then-fresh pair as two events. With a deputy both dispatches batch, so no flash.
   Fix: clear `hints` of the task in `answer-given` (a hint is a function of the recorded answer) and let the next view refill them.
3. **The factor named in a hint can be rounded up past the true reach** — `M/📏️quantity/🟦️.ts:170-172` (`formatFactor`, two
   significant digits, half away) used by `M/🧩️task/🟦️.tsx` `magnitudeText`. "More than ×N off" must round down. Reproduction: a
   set with `hi/lo = 3.84` has reach `1.96`; the note says "more than ×2 off" while an item off by ×1.97 is flagged; a reach of
   `1.04` reads "more than ×1 off". The test (`🪜️challenge-views`) uses the same `toPrecision(2)` rounding as the code, so it is not
   an independent check of this direction. Only matters for narrow sets; the physics sets have reach 1000 or large.

### Accessibility

4. **Remaining time is not available on demand to a screen-reader user who is inside a control** — `M/▶️run/🟦️.tsx:150-165`. The
   `m:ss` text is a plain paragraph with `aria-live="off"` and no role; it is spoken at start, 30 s, 10 s and time up only. A user
   focused on a select or guess field cannot ask for the time without leaving it for browse mode. `m:ss` is also read as "0:48"
   with no unit. Whether the expert timer is acceptable under 2.2.1: **yes, as the essential exception** — the learner picks the
   challenge, its clock is described before the run (`expertHint`), it starts only on "Start the clock" after the allowance is
   shown, an easier challenge on the same quiz is one click away, and warnings (30 s, 10 s) are given. Recommend `role="timer"`
   on the paragraph (implicit `aria-live=off`, found by role) and a "Time left?" button that announces `clockLeft` politely; speak
   spoken times in words ("48 seconds", "1 minute 5 seconds") instead of `m:ss`.
5. **Other tasks' running clocks are invisible; an opened task can expire unnoticed** — `M/▶️run/🟦️.tsx:268-276` (step buttons show
   only Complete/Incomplete), `:120-141` (the status is mounted per displayed task). Reproduction: expert run; start the clock of
   task 1 (about 1 min), press "Next task", read task 2 for a minute; task 1 is read-only with no notice. Nothing is spoken for it
   (its `TaskClock` is unmounted) and its step still says "Incomplete". Fix: a third step state ("Clock running", "Time is up")
   with a word and symbol, and a polite status for any clock reaching 30 s, 10 s, 0 whichever task is shown.
6. **A key reads as the value of the item beside it** — `M/↕️sorting/🟦️.tsx:156` (`.quiz-sort-key` without a name), place number
   `aria-hidden` (`:143`). Screen reader: "Horse, 20 g, Move Horse up" — it is the value of *place 1*, not Horse. Sighted users get
   the same adjacency. Fix: name it ("Value of place 1: 20 g" visually hidden plus the place number shown) or tie it to the list
   with `aria-label` on the place.
7. **"Far too low / Far too high" is ambiguous in an ascending list** — `M/🌐️i18n/🟦️.ts:236-237,669-670`,
   `M/↕️sorting/🟦️.tsx:150`. The smallest place is at the top, so "Horse: far too low" can be read as "the item sits too low in the
   list; move it up" — the opposite of what is needed. The note should name the key: "The value of this place is far too low for
   Horse" (DE "Der Wert dieses Platzes ist viel zu niedrig für Pferd"). Matching is clear (the card named in the note's row).
8. **Polite chatter on an easy sorting** — `M/🧩️task/🟦️.tsx:239-254`. One move in a ten-item set shifts nine keys; every item whose
   hint appears is joined into one utterance ("Hint for A: …, more than ×160 off. Hint for B: …"), after the move announcement.
   Suggest announcing the count of new hints ("3 new hints") and leaving the words to the notes beside the items, as the
   classification count does.
9. **A guess typed but not committed when time runs out is dropped silently** — `M/🧩️task/🟦️.tsx:131-139` (`typed` falls back to
   `shown` once `readOnly`; `commit` returns on `readOnly`). The user sees their text vanish; the only notice is the status of the
   clock. Acceptable by design (the answer instant is the commit), but say it ("Your last entry was not saved") or keep the draft
   visible, marked.
10. **Hidden-key sorting moves a field under the user's tab sequence** — `M/↕️sorting/🟦️.tsx:97-105`. Tab from A commits A, the
    list reorders, focus follows the field the user tabbed to (correct), but the next Tab goes to its new neighbour, so entering
    guesses top to bottom skips or repeats fields. Advised in advance (`guessHint`) and announced, so 3.2.2 holds; consider
    reordering on Enter or on leaving the task instead of on every blur.
11. Locked selects (`aria-disabled`, not `disabled`) revert silently when the user arrows through them; only the sorting buttons say
    "Time is up" (`M/🗂️classification/🟦️.tsx:51`, `M/🃏️matching/🟦️.tsx:57`). Add the same announcement as `move()` makes.

Passes: hint and miss are words plus a symbol, never colour alone (`Mark` is `aria-hidden`); hints describe the controls they are
about (`aria-describedby`); guess fields have a name containing the visible label (2.5.3), an error with `role="alert"`, a preview
by `aria-describedby`; target size 24 px (`quiz-target`); forced colours have a rule and a fixture entry for every painted state
(`🧫️fixtures/🌗️contrast-states/🔣️.json:22-27`); the clock bar is `aria-hidden` and decorative; focus moves to the task heading
after the start button leaves, never at time up; a locked task keeps focus and states why.

### Design deviations

12. **Results show score and points on two lines, not "Hard · 87 % · 261 of 300 points"** — `M/🏁️results/🟦️.tsx:289-290`. Same
    information; harmless. The labels (`Points: x of y`) are clearer for German ("Punkte: 261 von 300").
13. **Sorting result of a timed task never answered** still lists "Your position 1…n" with a verdict "Not correct" for each item
    (`M/🏁️results/🟦️.tsx:136-152`): the positions are the sheet order, not the learner's. Show "Not answered" in the position
    cell when every item has `miss` and no `guess`.
14. **Non-miss guesses read "Within reach"** (`quiz.results.near`): an item off by ×900 reads "≈ Within reach". Accurate against the
    rule, but "Within reach" sounds like "close". Consider "Not far off" / "Nicht weit daneben".
15. The confirmation lists "open or unanswered" tasks only by title (`M/▶️run/🟦️.tsx:339-353`); it does not tell a task whose clock is
    still running from one never started. Small.

### Edge cases checked

- **Easy hint beyond the keys** — unreachable: `keys.length` equals the item count (sheet contract); a missing key renders blank
  and `reach` is computed from `keys` only, no crash.
- **Zero-spread sets** — logarithmic reach 1000, hint text "×1,000" (`formatFactor(1000)`; DE "×1.000"); linear reach `∞`,
  `Number.isFinite` guard drops the amount, and the core never fires a hint (`+∞` never misses).
- **Never-opened timed task** — its clock never started, so it cannot run out; body hidden, `answer` impossible, `task-unopened`
  cannot be provoked from the UI; scored a miss at submission and shown "Not answered".
- **Switching tasks while the clock runs** — `TaskBody` is keyed by task, remounts, deadline comes from `view.opened`, no spurious
  announcement on return (tested). See finding 5 for what the user can no longer see.
- **Reload** — deadline `opened + seconds × 1000` from the held view (the deputy rebuilds `opened` from `view.opened`,
  `🫡️deputy/🟦️.ts:179`); `previous` is undefined on mount so a clock already in its last 30 s is not announced again (text only).
- **Hidden tab** — a `visibilitychange` listener re-renders; `answer` is also guarded by `session.now() < deadline`, so the render
  lag cannot let a late answer through; timers cleared in the effect cleanup, listener removed.
- **Deadline formula** — `opened + seconds × 1000` minus `session.now()`, `Math.max(0, …)`; the client locks at `left === 0`, the
  core only after `> seconds × 1000`, so the client is never more lenient than the decider.
- **Very long labels** — `.quiz-app` `overflow-wrap: break-word`, `li` `anywhere`, grid items `min-w-0`; the nowrap results cells fold
  into records below 52/58 rem. Not seen in a browser.
- **Crowd door** — `keysHidden` (open run, `!challengeRules(...).keys`) turns the gate `off` on the run and on the quiz page
  (`M/📖️quiz-page/🟦️.tsx:171`); results stay open. Tested for hard and expert with `others="always"`.
- **Classification keys hidden** — no descriptions, axes without numbers; the text alternative says shares as percentages (same
  information as the shape).

### i18n

- EN/DE complete, all keys literals, no plural forms, *du* throughout; `translation-completeness` passes. Hard-coded: `×` in
  `magnitudeText` (`M/🧩️task/🟦️.tsx`) and the symbols (`≫ ≪ ≠ ≉ ≈ ⏱ ⏳ ⚠ ⌛`), all `aria-hidden` and language-neutral.
- **One German word per concept broken**: `quiz.results.credit` = "Punkte" (per-item credit, `M/🌐️i18n/🟦️.ts:696`) and
  `quiz.challenge.scored` / `quiz.results` points = "Punkte" (`:666`, `:736`). Rename the credit column ("Teilpunkte" is still
  points; use "Anteil" or "Wertung der Antwort").
- "Experte" is a noun among adjectives ("Leicht, Mittel, Schwer"): "Profi" or "Experte" is fine, but "Expertenstufe" would match.
- `clockIntro` EN: "It shows once you start the clock." — "It" is unclear; "The task shows once you start the clock."
- DE "Offen" (incomplete task) and "offener Durchgang" (open run) share a word; harmless in context.

### Coverage gaps

- `session.now() < deadline` guard in `TaskBody.answer` (answer fired between deadline and the last tick) is untested.
- No test of unmount while the clock runs (timer and listener cleanup), of `visibilitychange`, of mounting in the 30 s / 10 s /
  up stage (no announcement), or of reload with an already-opened task.
- No test of stale or cleared hints after a local answer (finding 2) nor of chatter with several simultaneous hints.
- No test for SI prefixes beyond `k`/`M` on prefixed quantities (finding 1) nor for `5 M` on a metre unit.
- No German render of a clock or a hint; completeness covers keys only.
- `🌗️contrast-states` checks that a forced-colours rule exists, not the contrast of `--color-warning` against the 12 % tint.

### Style / code quality

- `M/▶️run/🟦️.tsx:251` `timed = tasks.some(seconds !== undefined)` duplicates `challengeRules(view.sheet.challenge).timed`; use the
  core's rule as `keysHidden` does.
- `GuessField.describedBy` (`M/🧩️task/🟦️.tsx:130,143`) has no caller: dead prop; remove or use it (easy has no guess fields).
- Whole `TaskView` re-renders every second (`useRemaining` lives in `TaskBody`), including radar SVGs and ten `GuessField`s. Move the
  countdown into `TaskClock` and lift only the `locked` transition (one render at 0).
- Duplicated leading docstring emoji in a file (repo convention is unique): `M/▶️run/🟦️.tsx` `🔣️` (TASK_KIND_ICONS, CLOCK_SYMBOLS) and
  `🚦️` (ClockStage, clockStage); `M/📏️quantity/🟦️.ts` `⏱️` shared with the session's `openTask`.
- Stale docs: `M/📏️quantity/🟦️.ts:163` `formatPoints` ("score × 100"), `M/▶️run/🟦️.tsx` header ("a submission that is only possible once
  every task is complete").
- No comments inside definitions, docstrings start with an emoji and carry `@see`; no logic the core already provides is
  re-implemented (`reach`, `answerComplete`, `challengeRules`, `thinkingScope` are imported).
