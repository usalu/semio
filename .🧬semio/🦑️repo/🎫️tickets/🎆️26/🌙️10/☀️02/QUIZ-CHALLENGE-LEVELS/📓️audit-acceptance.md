# Acceptance audit: quiz challenge levels (2026-10-03)

Read-only audit of the owner's request against the code on disk. Nothing was built or run; every statement below comes from
reading the files at the lines cited. A test "proves" a requirement only where I read its assertions. The integration agent's
e2e gate was running at the time and is not part of this evidence.

Abbreviations. `Q` = `🧰️framework/🛍️products/❓️quiz`; `CM` = `Q/🔨️modules`; `QT` = `Q/🧪️tests`; `RM` = `Q/🎯️targets/⚛️react/🔨️modules`;
`P` = `🎓️teaching/🛂️proctor`; `S` = `🎓️teaching/🏛️architecture/❓️quiz`. TS = `🟦️.ts(x)`, RS = `🦀️.rs`. "Vectors" = the shared fixtures under
`Q/🧫️fixtures/<case>/🔣️.json`, driven by the `QT/<case>/🥒️.feature` scenarios through the TS adapter (`🟦️.ts`), the Rust adapter
(`🦀️.rs`) and the Python reference (`🐍️.py`, numpy/mathjs/jStat as oracle).

## 1. Verdict

Every one of the nine requirements has an implementation in both cores, the proctor and the client, and a test at the unit or
shared-vector tier. I found no requirement without an implementation. The gaps are in (a) evidence at the site tier, (b) what a
learner is told on hard and expert, (c) one trust assumption of expert, and (d) a few promises that live only in prose.

## 2. Gaps, ranked by importance

| # | Gap | Kind | Evidence |
|---|---|---|---|
| G1 | **The tolerance of hard and expert is invisible and, on most live sets, far below the owner's x1000.** `reach = min(1000, sqrt(max/min))` (`CM/⛰️challenge/🟦️.ts:62-73`). Computed from the shipped quizzes: 7 of the 9 numeric sets have a reach of x4 to x64 (heating-load 4, final-energy 5.4, u-value 7.6, cooling-load 12.9, cooling-demand 63, air-change-rate 64); only the two physics sortings reach 1000. So on easy a hint fires at x4 in the heating quiz, and on hard a guess 5x off the heating load is a "miss" that costs every pair it touches. The learner is told the amount only inside an easy hint (`RM/🧩️task/🟦️.tsx:252-255`). The hard/expert task text says "Type your guess" (`RM/🌐️i18n/🟦️.ts:332,351`) and the result says "Far off" with no amount (`:377`, `RM/🏁️results/🟦️.tsx:49-57`); the rule is explained only in the catalog introduction (`S/🔣️.json`, paragraph 3). The deviation from "factor more than 1000" is a documented design choice (design §1, README "Reach and miss"), not a bug, but the owner never saw it as a decision. | confusing + owner-literal | CM challenge TS 62-73; i18n 332/351/377; the computation above |
| G2 | **Expert's time verdict rests wholly on the device's claim, and nothing bounds it.** `acted(at, floor) = max(at, floor)` has no upper bound (`CM/⛰️challenge/🟦️.ts:94`, RS `:142`); the proctor e2e asserts an opening dated an hour ahead "counts as it claims" (`P/🧪️tests/🌐️end-to-end/🦀️.rs:861`) and an answer dated before its opening counts from the opening (`:868`). A modified client can therefore claim `at` = opening for every answer, take unlimited time, and still earn 400 par points and top the board. Design §1 and README accept this ("trusted no further than the device already is"), so it is a promise in prose, with no mitigation (for example a lower bound on elapsed time from the proctor's arrival clock) and no test of the exposure. | trust / prose-only | design §1 "Who keeps the time"; README "The clock"; e2e 861, 868 |
| G3 | **Site e2e evidence is narrow.** `S/🧪️tests/⛰️challenge-levels/🟦️.ts` plays the physics quiz only, in English, in the `desktop` project (`S/🎭️e2e/🎚️config/🟦️.ts:44`). Not proven in a browser: matching hints and classification hint on easy; classification shape-only on hard/expert (the demand quiz is the only one with axes); a fully answered expert sorting and matching (the spec answers task 0, half of task 1, none of task 2: `:140-166`); the offline deputy at any challenge but medium (`📴️proctor-away`, `🔌️connection-shortage` are untouched by this ticket); layout at phone width for guess fields, hints and the clock (`📱️phone` and `📐️adaptive-layout` play medium only: `S/🧪️tests/📱️phone/🟦️.ts:65`, `📐️adaptive-layout/🟦️.ts:4`); German text of any new element (`🗣️both-languages` is untouched); the discard dialog (the driver clicks it only `if` visible: `S/🎭️e2e/🚶️learner/🟦️.ts:375-377`); the remembered challenge after a reload (written and polled, never reloaded). The owner prioritises desktop, then mobile: mobile has no real-browser evidence for the new UI. | evidence | files cited |
| G4 | **German rendering is proven only for parts.** `🗣️translation-completeness` proves key parity and placeholders (`QT/🗣️translation-completeness/🟦️.tsx:34-60`). Rendered in German and asserted: the sorting hint (`QT/🪜️challenge-views/🟦️.tsx:210-212`), the chooser (`QT/🏠️home-grid/🟦️.tsx:810-815`), points words (`challenge-views:744-748`), durations and factors (`QT/📐️quantity-formatting/🟦️.tsx:182-215`), the learner table (`QT/📇️learner-pages/🟦️.tsx:140`). Not rendered in German: the clock and its spoken stages, matching and classification hints, guess fields, miss marks, the locked note, the discard dialog. All other `challenge-views` tests pass `locale="en"`. | evidence | cited lines |
| G5 | **The quiz card cannot choose a challenge.** The request says the learner chooses on "the quiz page/card"; the chooser exists on the page only (`RM/⛰️challenge/🟦️.tsx:27`, `RM/📖️quiz-page/🟦️.tsx:182`) and a test asserts the card has no fieldset or input (`QT/🏠️home-grid/🟦️.tsx:763-776`). The card starts the one remembered challenge, which is global across quizzes (`RM/🎛️preferences/🟦️.tsx:106`): a learner who picked Expert on physics gets "Start (Expert)" on every card. The action names the challenge, so it is not hidden, but it is one click from an expert run. | design-intended, confusing | cited lines |
| G6 | **Capacity figures are not refreshed.** `P/README.md:276-277` says the hall figures were "measured while the hall played medium runs only" and are refreshed "by the next run of the gate"; the gate now plays all four challenges (`P/🧪️tests/🏋️capacity/🟦️.ts:479`) but was not run (`📓️report-proctor-coverage.md`, gates table). Also open from that report: no e2e for the crowd with unanswered items (C4) and no projections unit test of easy hints (C8: `P/🔨️modules/🔭️projections/🧪️tests/🔬️unit/🦀️.rs:185` asserts only that an expert run has no hints). | prose-only / evidence | cited lines |
| G7 | **Classification differs by challenge only through what can be hidden.** Hard changes classification by removing category descriptions and axis numbers; scoring has no tolerance and does not change (`CM/📏️scoring/🟦️.ts:174-202`). Both shipped classifications have something to hide (physics `power-or-energy`: descriptions on both categories, no axes; demand `standard-profiles`: four axes, no descriptions), so no learner meets an identical challenge today. But a classification with neither is authorable and `quizIssues` does not warn: the vectors themselves hold it (`cooling-1-hard` and `icons-1-hard` in `🃏️sheet-assembly`, task has no axes and no descriptions), where hard and expert-minus-clock equal medium yet earn 300 against 200. The challenge line "The values are hidden: you guess them" (`RM/🌐️i18n/🟦️.ts:243`) is untrue for a classification. | stated reason exists (README 248-250), still a seam | cited lines |
| G8 | **No automated contrast or axe check for the new elements.** Forced-colours rules are fixture-checked for hint, the three clock stages, the clock bar and the miss mark (`Q/🧫️fixtures/🌗️contrast-states/🔣️.json:22-27`, `QT/🌗️contrast-states/🟦️.tsx:111-117`), but the contrast of hint, miss and clock text is not computed (the test computes only peer-label ink: `:87-97`). Accessibility of the new UI is otherwise asserted by role, name and `aria-*` in React tests. | evidence | cited lines |
| G9 | **Linear-scale reach is exercised by vectors only.** All shipped numeric sets are logarithmic, so the linear branch of `reach`/`misses` and its hint text ("half the spread", `QT/🧗️challenge-ladder/🟦️.ts:149-155`, `challenge-views:215`) never occurs in a real run. | low | data computation above |
| G10 | **Interpretation to confirm with the owner.** The owner's expert sentence mentions only the timer; the design makes expert = hard + clock (design §1, "each is the one before plus one step"). Also, the owner's easy rule is a flat x1000 (see G1). | decision | design §1 |

## 3. Requirement trace

### R1. Four challenges selectable per run; chosen on page/card; remembered; switching an open run

| Part | Implementation | Test |
|---|---|---|
| Four challenges, one rule table | TS `CM/⛰️challenge/🟦️.ts:22-27` (`CHALLENGE_RULES`), `:35` rank, `:49` meets; RS `CM/⛰️challenge/🦀️.rs:35,48,58`; schema `Challenge` enum | shared vectors `QT/⛰️challenge-rules/🥒️.feature:56` (rules scenario), adapters TS `:32`, RS `:58-126`, Python `🐍️.py`; TS unit `QT/🧗️challenge-ladder/🟦️.ts:89-120`; RS unit `CM/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs` |
| Chosen at start, fixed for the run | TS `CM/🧾️lifecycle/🟦️.ts:113-123` (`startRun`), `:210` evolve; RS `CM/🧾️lifecycle/🦀️.rs:193-212` | vectors `QT/🧾️learner-lifecycle/🥒️.feature:88` (`points-by-challenge`, `switching-the-challenge`, `started-offline-*`); `QT/🔁️run-lifecycle/🟦️.ts:539-650`; proctor e2e `P/🧪️tests/🌐️end-to-end/🦀️.rs:782` |
| Chooser on the quiz page | `RM/⛰️challenge/🟦️.tsx:27-55` (fieldset of four radios, each `aria-describedby` its line with the par); `RM/📖️quiz-page/🟦️.tsx:161-213` | `QT/🏠️home-grid/🟦️.tsx:779-801` (four radios, names, par lines, keyboard, remembered), `:805-815` German; site `S/🧪️tests/⛰️challenge-levels/🟦️.ts:54,80,101,133` via `playQuiz` (`S/🎭️e2e/🚶️learner/🟦️.ts:357-377`) |
| Card starts the remembered one, names it | `RM/📖️quiz-page/🟦️.tsx:42-58`, `RM/🏠️home/🟦️.tsx:217` | `QT/🏠️home-grid/🟦️.tsx:743-761` (actions per card), `:763-776` (no chooser on the card, by design: G5) |
| Remembered on the device | `RM/🎛️preferences/🟦️.tsx:82,106` (default `medium`, invalid values fall back), written by the host through `writePreferences` (`:130`) | `QT/📡️presence-client/🟦️.tsx:846-852`; site `rememberedChallenge` polled after choosing (`S/🎭️e2e/🚶️learner/🟦️.ts:352-364`); no reload test (G3) |
| Switching an open run | core: another challenge voids the open run, the same one answers `run-open` (TS `CM/🧾️lifecycle/🟦️.ts:118-122`; RS `:193-212`); client asks first: `RM/📖️quiz-page/🟦️.tsx:183-213`; caps count voided runs too | vectors `switching-the-challenge`, `switching-until-the-cap`, `the-cap-before-the-switch`; proctor e2e `:849` (voids), `:1083-1100` (200 switches then `runs-exhausted`); React `QT/🏠️home-grid/🟦️.tsx:818-844` (keep, Escape, discard); `QT/🚶️learner-journey/🟦️.tsx:1626-1629`; site: dialog clicked only conditionally (G3) |

### R2. Easy: keys visible and assigned; hints beyond the reach; hint for classification

| Part | Implementation | Test |
|---|---|---|
| Keys shown (sorting ladder, matching cards, classification descriptions and axes) | TS `CM/🃏️sheet/🟦️.ts:67-91` (rules.keys); RS `CM/🃏️sheet/🦀️.rs:47-100` | vectors `QT/🃏️sheet-assembly/🥒️.feature:47` (`*-easy` sheets), proctor e2e `:797-799`; React `QT/🪜️challenge-views/🟦️.tsx:138-161`; site `⛰️challenge-levels:61-62` |
| Reach and miss (x1000, sqrt rule for narrow sets, linear half-spread, slack) | TS `CM/⛰️challenge/🟦️.ts:59-83`; RS `:69-120` | vectors `QT/⛰️challenge-rules/🥒️.feature:64` incl. the exact-x1000 and Sun cases; unit `QT/🧗️challenge-ladder/🟦️.ts:138-263` (incl. mathjs) |
| Hint rule, sorting | TS `CM/⛰️challenge/🟦️.ts:106-117`; RS `:162-175` | vectors `:81` scenario; unit `challenge-ladder:308-383`; proctor e2e `:803-812` and `P` unit n/a; React `challenge-views:197-262` (words and symbol, factor cut down, polite once); site `⛰️challenge-levels:58-69` |
| Hint rule, matching | TS `:118-139`; RS `:176-193` | vectors; `challenge-ladder:386-445`; proctor e2e `:1054-1060`; React `challenge-views:280-290`; site: none (G3) |
| Hint, classification (count, never which) | TS `:140-146`; RS `:194-203` | vectors; `challenge-ladder:447-470`; proctor e2e `:1064-1074`; React `challenge-views:292-300`; site: none (G3) |
| Hints on the view only while open, easy only | TS `CM/👁️views/🟦️.ts:117-136`; RS `CM/👁️views/🦀️.rs:74-93` | vectors `hints-on-easy`; proctor e2e `:785-815,1076-1078` (none after submit or void); deputy `QT/🫡️deputy-decisions/🟦️.tsx:200-224`; session `RM/🧭️session/🟦️.ts:1121-1129,1371-1380`, `QT/🚶️learner-journey/🟦️.tsx` (re-read without a deputy) |
| Reach shown to the learner | `RM/🧩️task/🟦️.tsx:252-255` ("more than x...") | `challenge-views:220-230` (never larger than the rule's) |

### R3. Medium: keys visible, assign, no hints

Same sheet rule as easy, `hints: false` in the table (`CM/⛰️challenge/🟦️.ts:24`), `runHints` skipped (`CM/👁️views/🟦️.ts:136`). Proof: vectors `*-medium` sheets and `hints-on-easy` (none for other challenges); proctor e2e `:813-825` (keys on a medium sheet, guesses refused with `answer-invalid`, no hints); deputy `QT/🫡️deputy-decisions/🟦️.tsx:200-224` ("none for a run of another challenge"); site `⛰️challenge-levels:80-99`; scoring `QT/⚖️partial-credit-scoring/🟦️.ts:111-127` (order alone, same at easy and medium, guesses refused).

### R4. Hard: keys hidden; learner guesses; classification shape-only; scoring of guesses with misses

| Part | Implementation | Test |
|---|---|---|
| Sheet without keys, cards, descriptions, axis numbers; profiles as shares | TS `CM/🃏️sheet/🟦️.ts:40-57,67-91`; RS `CM/🃏️sheet/🦀️.rs:103-113` | vectors `*-hard` (`energy-5489-hard`: `axes=id-label`, no descriptions); proctor e2e `:829-831` (no `value`, `keys`, `cards`, `seconds` anywhere), `:854` (axes are id and label only) |
| Answer shape: sorting `guesses`, matching `guesses` (no assignments), validity and completeness | TS `CM/✅️validation/🟦️.ts:651-661,682-692,694-728,729-741`; RS `CM/✅️validation/🦀️.rs:192-260` | vectors `QT/✅️answer-validation/🥒️.feature:43`; proctor e2e `:833-834` (assignments refused where hidden) |
| Scoring: miss costs every pair; perfect stays reachable; absent guess is a miss | TS `CM/📏️scoring/🟦️.ts:84-118` (sorting), `:121-161` (matching); RS `CM/📏️scoring/🦀️.rs:86-170` | vectors `QT/📏️sorting-concordance/🥒️.feature:83`, `QT/🔀️matching-concordance/🥒️.feature:70`; unit `partial-credit-scoring:398-545` (sorting), `:548-677` (matching) incl. jStat and mathjs oracles; proctor e2e `:838-842` (guess marks and score), `:1022-1026` (x1000 on a capacity is a miss) |
| Classification: descriptions and axis numbers hidden, credit unchanged | TS `CM/🃏️sheet/🟦️.ts:44-57`; scoring `CM/📏️scoring/🟦️.ts:174-202` | vectors `🕸️profile-similarity` (sheet side in `🃏️sheet-assembly`); React `challenge-views:302-309`; proctor e2e `:854` (expert); site: none (G3); seam G7 |
| Client: guess fields, items order by guesses | `RM/↕️sorting/🟦️.tsx:110-120,176-188`; `RM/🃏️matching/🟦️.tsx:79-110`; shared `GuessField` in `RM/🧩️task/🟦️.tsx` | `challenge-views:163-195` (complete once every item has a guess), `QT/⌨️task-keyboard/🟦️.tsx`, `QT/📐️adaptive-layout/🟦️.tsx:225-232`; site `⛰️challenge-levels:101-131` (sorting) |
| Results: "Your guess", miss mark in words and symbol | `RM/🏁️results/🟦️.tsx:49-57,133-140` | `challenge-views:687-733,751`; site `:122-128` |
| Crowd withheld while keys are hidden | `RM/🗳️crowd/🟦️.tsx:49-57`, `RM/▶️run/🟦️.tsx:386` | `challenge-views:666-677`, `QT/🏠️home-grid/🟦️.tsx:846-860`, `QT/💭️crowd-client/🟦️.tsx:324` |

### R5. Expert: timer per question; opening; time-up; partial submission; offline/online agreement

| Part | Implementation | Test |
|---|---|---|
| Seconds per task on the sheet | TS `CM/⛰️challenge/🟦️.ts:86-91`, `CM/🃏️sheet/🟦️.ts:60-62`; RS `CM/⛰️challenge/🦀️.rs:127-138` | vectors `⛰️challenge-rules` (clock scenario `:73`), `🃏️sheet-assembly` (`*-expert`); proctor e2e `:853`, `:947` (30 + 12 x items, matching per item and dimension) |
| `open-task`: untimed refused, once only, unknown task, closed run, revised quiz | TS `CM/🧾️lifecycle/🟦️.ts:126-135`; RS `CM/🧾️lifecycle/🦀️.rs:228-241`; admission and policy `P/🔨️modules/🧩️instance/🦀️.rs:208,236` | vectors `the-clock`, `the-limit-of-a-sorting-and-a-matching`; `QT/🔁️run-lifecycle/🟦️.ts:539-542`; proctor unit `P/🔨️modules/🧩️instance/🧪️tests/🔬️unit/🦀️.rs:15,30,132`, `P/🔨️modules/🎭️actors/🧪️tests/🔬️unit/🦀️.rs:280-315`; e2e `:847-862,951-978,1010,1016` (also: refused repeat stores no receipt) |
| `task-unopened`, `time-up` at the millisecond | TS `CM/🧾️lifecycle/🟦️.ts:138-153`; RS `:243-260` | `run-lifecycle:569-599,621,639,650`; e2e `:868-871,981-989,997-999` |
| Partial submission; missing counts as a miss | TS `CM/🧾️lifecycle/🟦️.ts:156-165`, `CM/📏️scoring/🟦️.ts:204-224`; RS `:262-281` | `partial-credit-scoring:507-533,635-647,679-717,763-780`; vectors `sorting-concordance:83`, `matching-concordance:70`, `profile-similarity:56`; e2e `:876,1002-1009`; React `challenge-views:503-521,687-742`; site `:158-173` |
| Client clock: closed until started, text countdown (not a live region), spoken at start, 30 s, 10 s, up; read-only at zero; submit names open tasks; hidden tab loses nothing | `RM/▶️run/🟦️.tsx:83-130,150-230,232-256,258-300,309-334`; session `RM/🧭️session/🟦️.ts:919-941,949-957` | `challenge-views:311-376,377-657`; site `:133-173` with the page clock; reduced motion covered at `challenge-views:437` |
| Offline and online agree on the verdict | verdict from the device's instants only: TS `CM/⛰️challenge/🟦️.ts:94`, session `RM/🧭️session/🟦️.ts:298-302` reuses `acted`; start carries `at` too (`CM/🧾️lifecycle/🟦️.ts:121`) | vectors `clock-ahead-*`, `clock-behind-*`, `clock-far-behind-*`, `started-offline-*`, `the-last-instant`; `run-lifecycle:600-650`; deputy `QT/🫡️deputy-decisions/🟦️.tsx:148-198`; `QT/🚶️learner-journey/🟦️.tsx:1429-1460,1490-1550,1881-1905`; e2e `:889-891`; limit of the proof: G2 |

### R6. More points for harder challenges: results, learner view, leaderboard; best by points; badges need medium

| Part | Implementation | Test |
|---|---|---|
| `points = score x par` in the result | TS `CM/⛰️challenge/🟦️.ts:54`, `CM/📏️scoring/🟦️.ts:223`; RS `:63`, scoring `:55` | vectors rules scenario `:56`; `partial-credit-scoring:747-761`; e2e `:810,842` |
| Results screen shows score and "Hard . Points: 261 of 300" | `RM/⛰️challenge/🟦️.tsx:22-24`, `RM/🏁️results/🟦️.tsx` | `challenge-views:687-733`; site `expectFeedback` (`S/🎭️e2e/🚶️learner/🟦️.ts:718`) in all four challenge tests |
| Learner view: runs with challenge and points, best as `Best`, total of points | TS `CM/👁️views/🟦️.ts:84-114`; RS `CM/👁️views/🦀️.rs:55,492`; client `RM/📇️profile/🟦️.tsx:31-42`, `RM/📖️quiz-page/🟦️.tsx:61-69` | vectors `QT/🏆️leaderboard/🥒️.feature:83`; e2e `:878-883` (an easy 100 loses to a hard 0.5..1.0 x 300); `QT/📇️learner-pages/🟦️.tsx:140`; site `shownBest` (`⛰️challenge-levels:77,98,130`) |
| Leaderboard ranks by points, row shows challenge | TS `CM/👁️views/🟦️.ts:219-250`; client `RM/🏆️leaderboard/🟦️.tsx:37-71,152` | vectors `🏆️leaderboard:99`; proctor unit `P/🔨️modules/🔭️projections/🧪️tests/🔬️unit/🦀️.rs:166`; e2e `:884-885`; `QT/🏠️home-grid/🟦️.tsx:861,984`; site `S/🧪️tests/🏆️live-leaderboard/🟦️.ts` (totals in `PAR.medium`) |
| Best run = most points (a later run replaces only with strictly more) | TS `CM/👁️views/🟦️.ts:84-93`; RS `:492` | vectors `🏆️leaderboard:83,99`; e2e `:880-881`; site `⛰️challenge-levels:130` (the perfect hard run stays after a flawed one) |
| Badges ask a least challenge; catalog sets medium | TS `CM/🏅️badges/🟦️.ts:7-11`, validation `CM/✅️validation/🟦️.ts:365-375`; RS `CM/🏅️badges/🦀️.rs`; catalog `S/🔣️.json` (badges) | vectors `🏅️badge-rules:31`; `QT/🎖️badge-awards/🟦️.ts`; catalog test `S/🧪️tests/🧪️catalog/🟦️.ts:40-45`; e2e `:811` (perfect easy earns only `sorter`); site `⛰️challenge-levels:74-75,94-96` |

### R7. Every kind of question

| Task kind | easy | medium | hard | expert |
|---|---|---|---|---|
| Sorting | ladder of keys, far-key hint per item | ladder, no hint | typed guess per item, miss marks, score with misses | hard + clock |
| Matching | cards, far-card hint per item and dimension | cards | typed guess per item and dimension, miss marks | hard + clock |
| Classification (incl. spider profiles) | descriptions and axis numbers, count of misplaced items | same, no count | descriptions removed, diagrams as shape (shares), no numbers; credit unchanged | hard + clock; unanswered items earn 0 |

Implementation per cell: core `CM/⛰️challenge/🟦️.ts:104-147`, `CM/🃏️sheet/🟦️.ts:65-93`, `CM/📏️scoring/🟦️.ts:84-202` (RS twins above); client `RM/↕️sorting/🟦️.tsx`, `RM/🃏️matching/🟦️.tsx`, `RM/🗂️classification/🟦️.tsx:42-45,108-114,126-127`, `RM/🕸️radar/🟦️.tsx` (shape mode). Tests: `challenge-views:138-161` iterates every kind at every challenge; proctor e2e exercises sorting and matching at all four challenges and classification at easy (hint), medium (untimed) and expert (clock, shape, partial). No cell is identical to its neighbour on shipped data. The one seam is G7 (a classification with neither descriptions nor axes), and the stated reason for "no guess in classification" is README 248-250 and design §1, not the learner-facing challenge text.

### R8. Accessibility and both languages for every new UI element

| Element | Accessibility | Tests (en / de) |
|---|---|---|
| Chooser | fieldset and legend, native radios, each described by its line | `home-grid:779-801` / `:805-815` |
| Open-run line and discard dialog | alertdialog, focus on the safe button, Escape keeps the run | `home-grid:818-844` / not in German (G4) |
| Hints (sorting, matching, classification) | words plus symbol, never colour alone, describe their controls, polite once, several by count | `challenge-views:197-309` / sorting only in German |
| Guess fields | named per item, example in the unit, invalid messages | `challenge-views:163-195`, `QT/⌨️task-keyboard/🟦️.tsx` / not in German |
| Clock | `role="timer"` text, separate polite status, spoken stages, bar decorative, lock described by why, focus unmoved | `challenge-views:377-657` / durations and countdown in German via `quantity-formatting:182-215`; the component not rendered in German |
| Miss mark and "Not answered" | symbol plus words, forced-colours rule | `challenge-views:687-742`; fixture `🌗️contrast-states` / not in German |
| Points and challenge in results, tables, board | text with number formats per locale | `challenge-views:744-748`, `learner-pages:140`, `quantity-formatting:118,165` |
| Key parity | `translation-completeness` (same keys, same placeholders) | `QT/🗣️translation-completeness/🟦️.tsx:34-60` |

### R9. Offline deputy and proctor decide alike for every new command

| Command | Deputy decides | Proctor decides | Alike by |
|---|---|---|---|
| `start-run` (challenge, `at`) | `RM/🫡️deputy/🟦️.ts:124-128` via `decideLearner`; session `RM/🧭️session/🟦️.ts:885-910` | same core, `P/🔨️modules/🎭️actors/🦀️.rs` | vectors (TS and RS run the same sequences), `deputy-decisions:75-103,148-198` |
| `open-task` | same path; session `:919-941` | same | vectors `the-clock`; `learner-journey:1429-1550` |
| `record-answer` (`at`) | never the deputy's (header comment `:12-15`); the session refuses a late answer itself with the core's `acted` (`RM/🧭️session/🟦️.ts:298-302,955`) and the proctor judges at delivery from the same `at` | same core | vectors `clock-*`; `deputy-decisions:167-172`; `learner-journey:1429-1460` |
| `submit-run` (timed, partial) | `decidable` guard: only with a run view held (`:1190-1193`) | same | vectors; `deputy-decisions:130-147,180-198` |
| Outbox restores | `start-run` only with a challenge and `at`, `open-task`/`record-answer` only with `at`, open-task queued before the answers of its task | n/a | `QT/📬️outbox-delivery/🟦️.tsx:474-496`; `learner-journey:1499-1509` |

A residual asymmetry: the client locks an expert task at `now >= deadline` while the core still accepts an answer made at exactly
the deadline (`CM/🧾️lifecycle/🟦️.ts:149`, strict `>`). The client is one millisecond stricter, never more lenient; harmless, and
`challenge-views:565-584` fixes the client side.

## 4. Behaviour promised only in prose

- Expert's integrity against a modified client (G2): design §1 and README say it is accepted; there is no test of the exposure and no mitigation.
- The reach rule as the owner would read it (G1): in the catalog introduction and README, not at the point of play.
- Capacity with a challenge mix: README `P/README.md:276-277` promises a refresh that has not happened (G6).
- Proctor C4 and C8 are "proven elsewhere" per `📓️report-proctor-coverage.md`, not by a test of their own.
- "Hidden tabs change nothing": covered by `challenge-views:602-617`, with jsdom timers; the site spec uses the page clock but never hides the page.

## 5. What a learner would find confusing

1. The tolerance (G1): why is "5x off" far off here but not in physics, and what is the limit? Nothing says so on hard.
2. Hard on a classification: the challenge line promises "you guess them", but only descriptions and diagram numbers disappear (G7).
3. The remembered challenge is global and one click from an expert run on any card (G5).
4. Easy and medium differ for sorting only by hints that appear after the first move; before it the two screens are identical. The line under the radios says so ("answers that are far off get a hint"), which is enough, but a learner reading the two lines side by side does not learn that classification hints are counts.
5. A perfect easy run (100) loses to a weak hard run: intended ("best = most points"), told in the introduction and the page hint (`quizPage.hint`), not on the results screen.

## 6. Limits of this audit

No test was run and no build was made. Line numbers are those on disk at the time of reading and will drift while others work in
the same files. The Python reference and the Gherkin files were read for coverage claims, not re-executed.
