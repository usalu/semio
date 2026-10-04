# Report — teaching site (`🎓️teaching/🏛️architecture/❓️quiz`) at the challenge levels

Agent: site. `SITE` = `🎓️teaching/🏛️architecture/❓️quiz`. Ticket folder `T`.

## What changed

Created
- `SITE/🧪️tests/⛰️challenge-levels/🟦️.ts` — new spec in the `desktop` project, four tests on physics: easy (keys on the
  ladder, the largest drawn power moved onto the smallest key → `data-hint="low"` beside it, fixed → no hint, perfect →
  100 points, no badge, best "easy 100 of 100"), medium (keys, no hints, perfect → 200 points and `badgesFor(["physics"],
  "medium")` = physics-expert + numerical-brain), hard (no keys, guess fields, true guesses → 300 points + badges; a second
  run with one guess ×10⁴ → score < 100, exactly that row marked `data-miss`, best stays 300), expert (`page.clock.install()`
  before arrival; closed clock shows no items; task 1 answered after starting its clock; task 2 started, one guess, then
  `page.clock.fastForward(allowed + 2 s)` → `data-clock="up"`, every guess field `readonly`; task 3 never opened;
  confirmation names tasks 2 and 3; result expert, par 400, task scores 100/0/0, score 33.3 %, 133.3 points).

Updated
- `SITE/🔣️.json` — introduction: new paragraph on the four challenges (keys, hints with the reach in words, typed
  estimates, clock, early submission on expert, 100/200/300/400 × score, best run per quiz = most points), scoring
  paragraph + "an estimate far off costs every pair it belongs to", badge paragraph "on medium or harder" and "sum of the
  points of their best run of every quiz" (EN + DE, *du*). Badges `*-expert`, `numerical-brain`, `pattern-seer`:
  `"challenge": "medium"` and descriptions "… on medium or harder" / "auf Mittel oder schwerer". Quiz files untouched.
- `SITE/🧪️tests/🧪️catalog/🟦️.ts` — content-policy case: every badge but `completed-quizzes` names a least challenge that
  meets `medium` (`challengeMeets`).
- `SITE/🎭️e2e/🚶️learner/🟦️.ts` — `Challenge`, `CHALLENGES`, `PAR`, `KEYS_SHOWN`, `CHALLENGE_NAMES`; `SourceBadge.rule.challenge`;
  `badgesFor(perfect, challenge = "medium")`; `rememberedChallenge`, `chooseChallenge`; `playQuiz(device, quiz, challenge =
  "medium")` (card action at the remembered challenge, else the quiz page's radio + its action, confirming the discard
  dialog); `startClock`, `clockOf`, `typedValue` (`3828e23 W`: whole mantissa, exponent, base unit — locale-proof),
  `guessSorting`, `guessMatching`, `keysHidden`, `shownHints`, `sortedItems`, `FAR_OFF`; `sortInto`/`moveDown` use
  `.quiz-sort-up`/`.quiz-sort-down` (the ladder has a grip) and press "Keep this order" when the dealt order is already
  right; `answerTask` starts a clock first and answers hidden keys by typed true values (flawed: one guess ×10⁴);
  `shownResults` returns `challenge`, `points`, `par` and per row `miss`, row labels without their icon; `amounts`,
  `namedChallenge`, `scoredPoints`, `shownBest`; `expectFeedback(…, challenge = "medium")` checks challenge, par,
  points ≈ score × par and par on perfect; `taskOf`, `itemOf` exported.
- Specs kept at their intent, adapted to points: `🎯️quiz-runs`, `📱️phone`, `📴️proctor-away` (card best via `shownBest`,
  `PAR.medium` instead of 100 %), `🏆️live-leaderboard` (totals in points: `PAR.medium`, partial points).
- `SITE/🎭️e2e/🎚️config/🟦️.ts` — `⛰️challenge-levels/🟦️.ts` in `desktop`.
- `SITE/README.md` — `mountQuiz` row (logo, legal, pets), catalog-test row, new section "Challenges" (table + points +
  badges), spec row `⛰️challenge-levels`.
- `.claude/launch.json` — re-added `architektur-und-technologie-quizze-steady` (Nx `dev`, 6061/8791,
  `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`) and `architektur-und-technologie-quizze-beside` (`bun <site>/📜️script.ts dev`, site
  6063, proctor 8793, `PROCTOR_DATA=.🧬semio/🎓️teaching/proctor-beside`), after `architektur-und-technologie-quizze`.
- `🔣️taxonomy.json` — `⛰️challenge-levels` in `members-of-tests`.
- Fixture `🧫️fixtures/🧾️learner-lifecycle/🔣️.json` regenerated (site plays read the live catalog).

Removed: nothing. My e2e run directories and `T/🗑️generated/site/` were deleted.

## Gates

| Gate | Command | Result |
|---|---|---|
| Lifecycle vectors | `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py lifecycle` (twice) | wrote, then `unchanged` |
| Parity | `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=14 executed=129 passed=129, parity=129/129 |
| Site node tests | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | 5 files, 138 passed (with the new catalog case and the edited catalog) |
| Typecheck | `bun nx run @teaching/architecture-quiz:typecheck` | exit 0 (before and after the driver/spec work) |
| Taxonomy | `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching/🏛️architecture/❓️quiz"` | clean, 0 errors |
| E2E | `bun ./📜️script.ts test-e2e dev --project=desktop [--no-deps] [specs]` in the site package (ports 6161/8891 free) | see below |

E2E history (dev topology only; rehearsal not run):
1. With `boot`: boot failed on hundreds of console errors from `🐾️pets/…/🖌️depiction` (`<path> d … NaN`, pets session
   mid-change), so nothing ran after it.
2. `--no-deps challenge-levels` (run 3/4): **medium, hard and expert passed every step of their bodies**; each test then
   failed only at the device teardown on those pets console errors. Easy failed at the hint step whenever the Sun was
   the largest drawn item (3 of 3), while `sunlight-on-earth` got its hint (with an unresolved label
   `quiz.task.farLow`) — see open problems.
3. Desktop project (`--no-deps`, 20 tests): first run a page error `this.rehintAll is not a function` (session mid-edit);
   last runs **8/8 and 14/20 failed at `playQuiz`**: the proctor answers start-run with
   `{"status":"rejected","reason":{"kind":"invalid","detail":"command-malformed: missing field `at`"}}` (read from the
   Playwright trace) — the Rust command now wants `at` on `start-run`, the client does not send it yet. 6 desktop tests
   that start no run passed.

Not run: `deploy-check` (needs Docker), rehearsal topology, the other projects (phone, layout, presence, shortage, away,
pets) — the stack refuses every run start until client and proctor agree on `start-run`.

## Open problems (other layers)

1. **start-run `at`** — proctor/core expects `at` on `start-run`, the client sends none (`command-malformed`). Every e2e
   that plays a quiz fails until one side changes.
2. **Easy hint on the Sun** — moved onto the smallest key (35 W) the Sun gets no hint in the browser (3/3), although
   `hintsOf` should give `low`; with another largest item the hint shows. Suspect the session/deputy path (deputy
   `hints`, sheet comparison `alike` with `3.828e26`, or `loadRun` replacing the view's hints). Someone has since changed
   my spec's expected text to "Key far too small" and added `rehintAll` in the session — likely the fix in progress.
3. Pets depiction NaN console errors make every spec fail at teardown while they last.

## Deviations and decisions

1. Existing specs that asserted "100 %" on the card and totals of 100 now assert points (`PAR.medium`): the card no
   longer shows a percentage, and totals sum points. Intent unchanged.
2. Hard/expert answers are typed as whole-mantissa exponent numbers in the base unit (`typedValue`), so a German `.` group
   mark can never turn `3.828` into 3828.
3. `-beside` row runs `📜️script.ts dev` directly (the Nx target pins 6061); no own Vite cache because a relative
   `TEACHING_ARCHITECTURE_QUIZ_CACHE` resolves against the site root. Rows only in `.claude/launch.json`, as before.
4. Leaderboard totals are still parsed with `,` → `.`; totals ≥ 1000 (possible now) would misparse — no desktop learner
   reaches 1000 today (most is 800).

## Notes for the next agent

- `playQuiz(device, quiz, challenge = "medium")`, `chooseChallenge(device, quiz, challenge)`, `startClock(device)`,
  `guessSorting(device, item, value, quantity)`, `guessMatching(device, dimensionIndex, item, value, quantity)`,
  `shownHints(device): {item: "high"|"low"|"misplaced"}`, `shownResults(device)` → `{score, challenge, points, par, tasks[{task,
  score, rows[{label, text, miss}]}]}`, `shownBest(device, quiz)` → `{challenge, points, par}`, `badgesFor(perfect, challenge)`.
- Run the spec alone: `bun ./📜️script.ts test-e2e dev --project=desktop challenge-levels` in the site package (through
  Nx put `--` before the gate's arguments, or Nx eats `--project`).
