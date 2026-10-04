# Quiz challenge levels — closing summary (2026-10-03)

Request: four challenges for every quiz — easy, medium, hard, expert — that change how every kind of question behaves and
give more points on harder challenges. Normative design: `📓️design.md` (with its 2026-10-03 revision notes).

## What a learner gets

| Challenge | Sorting | Matching | Classification | Extra | Most points per quiz |
|---|---|---|---|---|---|
| easy | a ladder of the true values; move each item to its value | value cards as before | descriptions and axis numbers | hints: "Key far too large/small (more than ×N)"; classification: count of misplaced items | 100 |
| medium | ladder, no hints | cards, no hints | as easy, no hints | — | 200 |
| hard | no values; type a guess per item, items order themselves | no cards; type a guess per item and quantity | no descriptions; spider diagrams show shape only | results show the tolerance per task and how far each guess was | 300 |
| expert | as hard | as hard | as hard | every task runs against its own clock ("Start the clock", `m:ss`, time up locks it; submit any time) | 400 |

- "Off by a factor of more than 1000" is generalised as the **reach**: ×1000, or the square root of the presented values'
  max/min ratio where that is less (seven of nine live sets span under four decades). On hard and expert a guess beyond
  the reach (or missing) loses every pair it is part of in the magnitude-weighted pair concordance.
- Points = score × par; the run with the most points per quiz counts; the leaderboard sums those points; the six
  perfection badges of the site need medium or harder.
- The challenge is chosen on the quiz page (four explained radios), remembered per quiz, named on the card's action;
  switching with an open run asks first and voids it.
- Time is kept by the device and bounded only by floors and a five-minute lead cap, so the offline deputy and the proctor
  decide alike; the proctor is never stricter than the device.

## Where it lives

- Contract and cores: `🧰️framework/🛍️products/❓️quiz/🧬️schema`, new module `🔨️modules/⛰️challenge` (TS + Rust), sheet,
  validation, scoring, lifecycle (`open-task`, `start-run`/`record-answer` with `at`, rejections `run-untimed`,
  `task-unopened`, `time-up`, `already-opened`), views, badges, presence; Python references, 12 regenerated fixtures and
  the new language-agnostic case `🧪️tests/⛰️challenge-rules`; README chapters "Challenge levels" and "The challenge on
  screen".
- Proctor `🎓️teaching/🛂️proctor`: `quiz.open-task`, storage format 3, state format 2, projector revision 6, crowd tally,
  presence accepts guesses, `float_roundtrip` JSON parsing (the root cause of a missing hint on the Sun), end-to-end tests
  for every challenge, capacity script plays all four.
- Client `🎯️targets/⚛️react`: session/outbox/deputy, chooser module `⛰️challenge`, per-challenge task views, shared guess
  field, hints, clock, results with points and tolerance, lazy run/results chunks; EN + DE.
- Site `🎓️teaching/🏛️architecture/❓️quiz`: catalog introduction and badge rules, catalog content rules, driver and the
  spec `🧪️tests/⛰️challenge-levels` (12 tests incl. German, phone, discard, reload), launch rows `-steady`/`-beside`.

## Verification (final tree, 2026-10-03)

| Gate | Result |
|---|---|
| Quiz TypeScript suite | 415/415 (coordinator rerun 15:47) |
| Quiz Rust crate | 180 passed, clippy clean |
| Parity exhaustive, quiz owner | 129/129 (coordinator rerun) |
| React suite / typecheck | 831/831 (coordinator rerun) / 0 errors |
| Proctor | 90 unit + 15 conformance + 21 end-to-end, clippy clean |
| Site node tests / typecheck | 228/228 / 0 errors |
| Site e2e, dev and rehearsal | per project 54/56 in each topology; the two red tests are `🥞️layered-home` (home grid of the adaptive-layout session) |
| Taxonomy (quiz scope) | clean |
| Site entry budget | 254 301 of 260 000 gzip bytes |

Not run: the proctor capacity gate (needs a quiet machine) and `deploy-check` (needs Docker). Staged deploy bundles in
`.🧬semio/🎓️teaching/architecture-quiz-poc` predate this ticket and must be rebuilt before a deploy.

## Reopened 2026-10-04 — specific hints (design §8, §8.4a, §8.4b)

The owner rejected generic easy hints ("≪ Wert viel zu klein (mehr als ×1.000)") and asked for specific comparisons
("Bist du sicher, dass 1000 XX zusammen nur eine YY ergeben?"). Every easy hint now questions one relation the learner's
own answer claims between two items of the task, never the truth itself:

- sorting/matching (`CompareHint`, verdict under/over/reversed): „Bist du sicher, dass 1,7 Billionen × „Automotor unter
  Volllast“ in puncto Leistung zusammen nur 1 × „Sonne“ ergeben?“; non-additive quantities "…höher/…-mal so hoch…";
  a reversed pair asks only for the order; the reference is the learner's most contradicted correctly placed item
  (unused in this answer first, then familiar, then the most absurd claim); numbers as plain digits, number words, or
  mantissa × 10ⁿ;
- spider profiles (`ProfileHint`): relative to a correctly placed item on the axis with the largest gap, else the value;
- plain classification (`GroupHint`/`CategoryHint`): "…dass „X“ und „R“ in dieselbe Kategorie gehören?";
- at most 3 hints per task; authored `short` labels (91, polished 19), `familiar` items (27), `Quantity.additive`;
  wire version 4.

Two hint audits (`📓️audit-hints.md`, `📓️audit-hints-2.md`) drove rounds 2 and 3. Final tree (2026-10-04, coordinator
rerun): quiz TS 450/450, React 920/920, parity 138/138; agents: Rust 191, proctor 90+15+21, site 243–247 node tests,
e2e rehearsal desktop 30/30 (13/13 challenge), dev challenge 12/13 (one load timeout clicking a home card, passes alone).
Screenshots of the new hints: `📸️shots/*-easy-hint*.png` (EN/DE, desktop/phone, sorting, matching, profile).

## For the owner to confirm

- Expert is read as hard plus a clock (keys hidden); the request only names the timer.
- The ×1000 threshold is the cap of the reach; narrower sets use a smaller tolerance.
- Points 100/200/300/400 per quiz at most; badges need medium.

Screenshots: `📸️shots/` (desktop and phone, every challenge). Audits: `📓️audit-*.md`; layer reports: `📓️report-*.md`.
