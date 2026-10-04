# Documentation audit, quiz challenge levels (2026-10-03)

Read-only audit. The code decides; every finding names the doc line, the code line and a replacement. Paths are relative
to `C:\git\semio`. `Q` = `🧰️framework/🛍️products/❓️quiz`, `P` = `🎓️teaching/🛂️proctor`, `A` = `🎓️teaching/🏛️architecture/❓️quiz`,
`D` = this ticket's `📓️design.md`.

Method: read both challenge twins, both lifecycle twins (TS and Rust), validation (TS, Rust `command_rejection`), sheet,
scoring, badges, views and crowd (TS), the proctor's manifest, presence admission, storage and projector constants, the
React session, deputy, outbox, crowd gate and i18n; then compared sentence by sentence. Nothing was built or run.

## Summary

- 1 probable code defect surfaced by a doc sentence (F1).
- 9 real drifts in the docs (F2 to F10), 7 of them stale or false statements, 2 imprecise.
- 12 smaller drifts (L1 to L12).
- All function names, constants, the rule table, the time rule (floor only), `already-opened`, `start-run.at`, the caps,
  the lifecycle table order, `FORMAT_VERSION` 3, `STATE_FORMAT` 2, `PROJECTOR_REVISION` 6, the Rust name list and the
  catalog badge rules match the code.

## Findings that matter

### F1. Proctor refuses every typed matching guess in the thinking room (code vs doc, probably a code defect)

- Doc: `P/README.md:487` "for matching only values a card of that dimension shows". `D:144` and `Q/README.md:477-479`
  and schema `ThinkingMatchingAnswer` (L1226) say the draft carries the guessed numbers.
- Code: `Q/🔨️modules/👥️presence/🟦️.ts:125-131` `thinkingAnswer` publishes `values: answer.guesses` for a hidden-keys
  matching (the React layer calls it for every open run, `🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx:473`).
  `P/🔨️modules/👥️presence/🦀️.rs:158-167` admits a matching value only if it is an authored value of the dimension
  (`values.contains(value)`), otherwise `value-unknown`. A guess is practically never an authored value, so on hard and
  expert the draft is `refused` and not shared.
- Effect is small (the crowd is not offered on those runs), but the contract and the proctor disagree, and the client
  sends a state the proctor refuses on every guess.
- Proposal: fix the code (admit any finite number, positive on a logarithmic dimension, plus card values) and then the
  doc sentence reads: "for matching only finite numbers (positive on a logarithmic scale): the value of a card, or the
  learner's guess where the keys are hidden". If the code stays, `D:144`, `Q/README.md` thinking text and the schema
  must say that guesses are not shared.

### F2. `Q/README.md:17` "the bounded instant of an action"

- Code: `Q/🔨️modules/⛰️challenge/🟦️.ts:93-96` and `🦀️.rs:139-143`: `acted(at, floor) = max(at, floor)`; nothing bounds the
  instant from above (revised 2026-10-03).
- Replace: "the instant a learner acted, raised to its floor".
- Same stale phrase: `D:216` "bounded instants" (see G7) and `Q/🔮️oracles/🔣️.json:75` (see O1).

### F3. `Q/README.md:261-264` the six-decade claim is false

- Text: "among values that span six decades or fewer no key can be that far off, and a fixed factor would never fire".
- Code: `Q/🔨️modules/⛰️challenge/🟦️.ts:62-73`. Keys and truths both lie in `[lo, hi]`, so a key can be off by up to
  `hi / lo`. A fixed 1000 can therefore fire as soon as the set spans more than three decades; six decades is only where
  `sqrt(hi / lo)` stops being below the cap of 1000.
- Replace: "being off by more than a factor of 1000 is far off on a ladder that spans many decades, but among values that
  span three decades or fewer no key can be that far off, and up to six decades a fixed factor would be too lenient;
  half of what the set spans in decades (the square root of the ratio) carries the same idea to every set."
- Same arithmetic error in `D:34`, right-hand cell (G1).

### F4. `Q/README.md:526-528` the quiz page shows the others "as after any run"

- Text: "…the others are not offered at all … on the results and on the quiz's page they show as after any run."
- Code: `Q/🎯️targets/⚛️react/🔨️modules/🗳️crowd/🟦️.tsx:44-52` (`hidden` turns every place but `results` off) and
  `…/📖️quiz-page/🟦️.tsx:171` (`hidden: opened !== undefined && !challengeRules(opened).keys`): while a run of that quiz
  that hides the keys is open, the quiz page does not offer them either.
- Replace: "…whatever the preference — because the crowd would show the hidden keys; the same holds on the quiz's page
  while such a run of the quiz is open; on the results, and on the quiz's page otherwise, they show as after any run."

### F5. `Q/README.md:636-637` and `D:201` the results line

- Text: "Hard · 87 % · 261 of 300 points".
- Code: `Q/🎯️targets/⚛️react/🔨️modules/🏁️results/🟦️.tsx:297-298` shows the score on its own line
  (`quiz.results.score`) and `challengeScored` below it; i18n `🌐️i18n/🟦️.ts:233` "{{challenge}} · Points: {{points}} of
  {{par}}" (DE L674 "Punkte: … von …").
- Replace: "The results say the score ("87 %") and, below it, challenge and points ("Hard · Points: 261 of 300")".

### F6. "no value at all" on hidden-keys sheets (four places)

- Text: `Q/README.md:121` "where the keys are hidden it carries no value at all"; schema `Sheet` description
  (`Q/🧬️schema/🔣️.json:421`); `P/README.md:422-423` "`hard` and `expert` sheets carry no number at all" (and in the same
  breath expert carries `seconds`); `Q/🔨️modules/🃏️sheet/🟦️.ts:6-7` ("carries no value at all and profiles become shares").
- Code: `sheet/🟦️.ts:43-56` keeps `profile` on every hidden classification category as shares `(v − min) / (max − min)`;
  `seconds` is a number on expert sheets.
- Replace (README): "…a sheet never says which item a number belongs to; where the keys are hidden it carries none of
  them: no sorting `keys`, no `cards`, no descriptions, no axis numbers, and profiles only as shares of their axis range."
  Replace (proctor): "`hard` and `expert` sheets carry no keys (no sorting `keys`, no `cards`, no descriptions, no axis
  numbers; profiles are shares of the axis range), and every task of an `expert` sheet carries its `seconds`." Schema
  `Sheet`: "…where it hides them the sheet carries none of those and a profile is only a share of its axis range."

### F7. Schema never defines the reach or the miss it uses

- `MagnitudeHint` (L905) "farther … than the reach of the presented values", `SortingItemResult.miss` (L526),
  `MatchingItemResult.miss` (L539) and `Challenge` (L55) use "reach" without stating it. The README and both docstrings do.
- Code: `challenge/🟦️.ts:62-83`.
- Add to `MagnitudeHint.description` (and refer to it from the two `miss` members): "The reach of a set of presented true
  values: on a logarithmic scale the factor `min(1000, sqrt(max / min))` (1000 where they do not spread), on a linear scale
  half their spread (unbounded where they do not spread). A key misses when `max(key, value) / min(key, value)`, or
  `|key − value|`, exceeds the reach by more than a relative 1e-9."

### F8. Schema `SortingAnswer.guesses` / `MatchingAnswer` say "exactly when"

- Text: L458 "Present exactly when the sheet task hides the keys"; L469 "`guesses` exactly when it hides them" (and
  `D:103` "Present exactly when").
- Code: `Q/🔨️modules/✅️validation/🟦️.ts:693-720`, `726-740`: on a hidden-keys sheet task `guesses` may be absent (partial
  answer, valid while the run is open; `answerRejection` accepts `guesses === undefined`); only `answerComplete` demands
  a guess for every item. (`assignments` on a keys-shown matching really is required: `assignmentsFit` rejects absence.)
- Replace (`SortingAnswer.guesses`): "Only where the sheet task hides the keys (a sorting with `keys` refuses them). There
  they are the answer: the answer is complete once every item has a guess; items with a guess stand in `order` in ascending
  guess order; an item without a guess counts as a miss." Replace the `MatchingAnswer` sentence likewise: "`assignments`
  where the sheet task shows the keys (its dimensions carry cards), `guesses` only where it hides them".

### F9. Schema `Rejection` omits what the proctor really answers

- `P/README.md:366-368` and `Q/🧬️schema/🔣️.json:731`.
- Code: `Q/🔨️modules/✅️validation/🦀️.rs:258-270` (`at > MAX_TIMESTAMP` is `id-invalid` in Rust) and `🟦️.ts:494-501` (TS also
  refuses a non-timestamp `at` and a challenge outside the four as `id-invalid`).
- Schema `id-invalid` currently lists ids and slugs only. Add: "…or an `at` beyond 2^53 − 1". `time-up`: replace "the answer
  was given after the task's seconds had passed" with "the instant of the answer, raised to the task's opening, lies more
  than the task's `seconds` after that opening". Add `run-open`: "a run of the quiz is open at the same challenge (the client
  resumes it)" (the code is not new, but its challenge-dependent meaning is).
- `P/README.md:367-368`: "an `at` beyond 2^53 − 1 … is `id-invalid`" is exact only up to 2^64 − 1; a larger number does not
  decode (u64) and is `command-malformed`. Replace: "an `at` from 2^53 up to 2^64 − 1 is `id-invalid`".

### F10. Crowd sentence "a task a timed run left without any answer adds its score bin only"

- `Q/README.md:500-501`, `D:183`.
- Code: `Q/🔨️modules/👁️views/🟦️.ts:326-340`: a sorting result is left out of the places and of the mean position when
  `every item has miss === true and guess === undefined` — that is a task without an answer and also an answer that
  holds an order but no guess. `P/README.md:420` has the exact wording.
- Replace: "a sorting in which no item was guessed (a timed run left it unanswered) adds its score bin only".

## Smaller drifts

- **L1.** `Q/README.md:127-131` "An answer carries guesses exactly there" → "An answer carries guesses only there"
  (partial answers may omit them, F8).
- **L2.** `Q/README.md:356-361` "commandRejection refuses it the same way" is ambiguous: say "refuses it as `id-invalid`"
  (the proctor itself answers `command-malformed`, `P/🔨️modules/🎭️actors/🦀️.rs:207`).
- **L3.** `P/README.md:177` "4186 bytes … 3.9 times below": the figure replaced 2952 in this change but no code derives it
  (`P/🧪️tests/🌐️end-to-end/🦀️.rs:1251-1256` asserts only `largest > 1000 && largest * 3 <= limit`). Not re-measured; treat as
  unverified.
- **L4.** `A/🔣️.json` introduction paragraph 2 (EN/DE): "A run only counts as a whole: answer every task, then submit." and
  "your best run counts" are overruled by paragraph 3 (expert may submit early; the run with the most points counts).
  EN: "…answer the tasks, then submit; on expert you may submit before every task is answered. … the run with the most
  points counts." DE: "Beantworte die Aufgaben und gib dann ab; auf Experte darfst du abgeben, bevor alle beantwortet sind. …
  dein Durchgang mit den meisten Punkten zählt."
- **L5.** `A/🔣️.json` paragraph 3 and `A/README.md:33`: "by more than a factor of 1000 or, where the values of a task span less,
  by more than half their span" does not say that on a logarithmic scale the span is counted in orders of magnitude (the
  reach is a factor of `sqrt(max/min)`) and on a linear scale it is half the range, and that a linear set has no 1000. Proposal
  (README): "a value placed farther off than its set's reach — on a logarithmic scale a factor of 1000, or half the orders of
  magnitude the values span where that is less; on a linear scale half their range — is called far too high or too low".
  Paragraph 3 EN: "…by more than a factor of 1000 or, where the values of a task span fewer than six orders of magnitude,
  by more than half of them (on a linear scale: by more than half their range)".
- **L6.** `Q/🔨️modules/⛰️challenge/🟦️.ts:58` and `🦀️.rs:67` `REACH_FACTOR` docstring "The largest factor a value may lie
  off the truth … without missing": it is the cap of the logarithmic reach (and the reach where the values do not spread),
  widened by `REACH_SLACK`. Replace: "The cap of the reach on a logarithmic scale, a factor".
- **L7.** `Q/🧬️schema/🔣️.json:1036-1042` `LeaderboardRow.total` and `best` have no description; add "Sum of the points of the best
  runs in scope" and "The best run per quiz id in scope: the one with the most points".
- **L8.** `D:34` row: the sentence says a value misses when the ratio "exceeds" the reach; the normative text
  (`D:124`, code) is "exceeds the reach widened by `REACH_SLACK`". Add "(by more than the relative `REACH_SLACK`, §3.1)".
- **L9.** `D:38` first sentence: "`open-task` and `record-answer` carry `at`" → "`start-run`, `open-task` and `record-answer`
  carry `at`" (the parenthetical later says so).
- **L10.** `D:124` operations list "Only `/`, `sqrt`, `−`, `abs`, `min`, `max` and comparisons" omits `*` and `+`, which
  `reach × (1 + REACH_SLACK)` introduced; the README (L258-259) and both docstrings include them.
- **L11.** `Q/README.md:723-725`, `D:195-196` and the deputy docstring agree; but `Q/README.md:754-760` never states that the
  deputy does not decide `record-answer` (the session records it and refuses a late one by `overdue`,
  `🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts:295-302`, `952-955`; deputy docstring L13-15). Add to the "challenge and the
  time" bullet: "An answer is never the deputy's to decide: the session records it, refuses it with `time-up` on the device
  when the device's clock says the task is over, and the proctor decides it at delivery."
- **L12.** Rust `🧾️lifecycle/🦀️.rs:12` "Every other `at` is the decision time" is true for the other events; no change needed
  (checked against `run-voided`, `run-submitted`, `badge-awarded`, `learner-registered`).

## Design document, internal contradictions after the revisions

| ID | Line | Text | Problem | Proposal |
|---|---|---|---|---|
| G1 | 34 | "seven of nine numeric sets span under 4 decades, where a fixed 1000 would never fire" | A fixed 1000 can fire from three decades up | "…span under 4 decades, where a fixed 1000 would hardly ever fire (never under 3)" |
| G2 | 34 | "a value misses when `max/min` exceeds it" | Slack missing, §3.1 L124 has it | add "(widened by `REACH_SLACK`, §3.1)" |
| G3 | 38 | "`open-task` and `record-answer` carry `at`" | `start-run` also, per the parenthetical | see L9 |
| G4 | 124 | operations list | `*`, `+` missing | see L10 |
| G5 | 147 | "an `at` that is no Timestamp is a malformed command, not a rejection … No rejection code is added" | Code: TS `commandRejection` and Rust (`at` beyond MAX) answer `id-invalid` (`validation/🟦️.ts:494-501`, `🦀️.rs:258-270`); README L356-361 says so | "…Rust cannot decode a challenge outside the four or a non-u64 `at` (`command-malformed`); an `at` beyond 2^53 − 1 decodes and both cores refuse it `id-invalid`; TypeScript refuses all of them `id-invalid`. No new rejection code." |
| G6 | 183 | "a task without any answer adds its score bin only" | see F10 | see F10 |
| G7 | 216 | "`bounded instants`" | no bound exists | "instants raised to their floors" |
| G8 | 201 | "Hard · 87 % · 261 of 300 points" | see F5 | see F5 |
| G9 | 103 | "`SortingAnswer.guesses`, `MatchingAnswer.guesses` … Present exactly when" | see F8 | "only when the sheet task hides the keys (complete when every item has one)" |
| G10 | 144 | "`ThinkingMatchingAnswer.values` carries guessed numbers" | proctor presence admission refuses them, F1 | resolve with F1 |

Searched the design for `[floor, now]`, `REACH_DECADES`, "Already opened → no events" and "caps as before": none remain.
`D:170` says `already-opened` with the receipt remark, `D:169` says caps count every run; both match the code.

## Other drift found on the way (outside the requested list)

- **O1.** `Q/🔮️oracles/🔣️.json:75` (rationale): "the reach of a set of values (half its spread on the quantity's scale, capped
  at three decades on a logarithmic one)" is the old log10 formulation (equivalent in value, but the reference now computes
  `sqrt(amax / amin)` with `minimum`, `🧪️tests/⛰️challenge-rules/🐍️.py:152`); "the bounded instant a learner acted" and
  "numpy.clip for the instants" are stale (the reference uses `numpy.maximum`, `🐍️.py:23-24`, `99-100`); "numpy.ptp over
  numpy.log10 for the spread" no longer describes the logarithmic branch. The feature file
  (`🧪️tests/⛰️challenge-rules/🥒️.feature:15-22`) is current.

## Verified without drift

- `Q/README.md` Challenge levels table, `CHALLENGE_RULES`, `challengeRank`, `challengeMeets`, `points`, `REACH_FACTOR`,
  `REACH_SLACK`, `reach`/`misses` formulas (L251-260), `TASK_SECONDS` 30/8/12/12, hint rules, time rule and floors
  (L275-290), `already-opened`, `start-run.at`, the Rust name list (L300-301).
- Lifecycle table (L308-342) in code order (TS `🧾️lifecycle/🟦️.ts:112-153`, Rust `193-260`), caps (L363-367, `DEFAULT_LIMITS`
  in `🧬️schema/🟦️.ts:281`), `Timestamp`/`MAX_TIMESTAMP`.
- Scoring and answer validation sections (guesses, misses, partial answers, `answerComplete`), badges and
  `challengeMeets`, leaderboard/learner view (`Best`, strictly-more rule, `reachedAt`), crowd nearest-value rule, state
  classes, deputy and outbox statements, `preferences.challenge` default `medium`, the chooser, hints announcement,
  clock stages.
- Proctor README: wire table (offline policies match `🧩️instance/🦀️.rs:204-210`), floors and `at` rule, `FORMAT_VERSION` 3
  (`🗄️storage/🦀️.rs:68`), `STATE_FORMAT` 2, `PROJECTOR_REVISION` 6, revision-6 text, caps table, dev launcher moving the
  disposable directory aside (`🏗️bootstrap/🟦️.ts:128-149`), end-to-end coverage list.
- `A/README.md` Challenges section and spec row, the catalog badge rules (`"challenge": "medium"` on six badges), EN/DE
  challenge names, points 100/200/300/400, "most points" and badges-need-medium statements.
- Schema: `Challenge`, `SheetAxis`, `Sheet*Task` members, `seconds`, `opened`, `hints`, `Best`, `RunSummary.points`,
  `Command` and `start-run.at` descriptions, `Limits`, `BadgeRule`, item results.
- `challenge` module docstrings (TS and Rust) and the lifecycle decider docstrings (TS and Rust).
