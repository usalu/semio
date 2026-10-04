# Report — teaching proctor on the challenge levels

Agent: proctor (design §4, the map in `📓️explore-proctor.md`). `P` = `🎓️teaching/🛂️proctor`. Built against the quiz crate
after the 2026-10-03 revision (`acted(at, floor) = max(at, floor)`: the proctor's own clock never enters a time verdict).

## Changed files

Created: `TICKET/tsconfig.proctor.json` (typecheck input for the capacity gate and bootstrap). Updated in the repo (nothing created or removed there):
- `P/🔨️modules/🧩️instance/🦀️.rs` — `quiz.open-task` in the manifest (learner stream, `OfflinePolicy::Optimistic`, between
  `start-run` and `record-answer`) and in the learner policy template's action list.
- `P/🔨️modules/🎭️actors/🦀️.rs` — `STATE_FORMAT` 1 → 2. Admission and target need no change (`open-task` targets the
  learner like every command but a handle claim; `command_rejection` of the core checks its ids and slug).
- `P/🔨️modules/🔭️projections/🦀️.rs` — `PROJECTOR_REVISION` 5 → 6; `task-opened` rewrites the learner state and the
  run view only (like an answer: no learner view, no transcript); `run_of` knows it; the crowd fold passes the quiz of
  the current catalog (a result of a quiz the catalog no longer lists is not tallied — its view is never written).
- `P/🔨️modules/👪️crowd/🦀️.rs` — `CrowdTally::fold(&mut self, quiz: &Quiz, result: &RunResult)`: classification counts
  only assigned items, sorting places items only when `quiz::crowd_orders(items)`, matching counts
  `quiz::crowd_value(definition, dimension, item)` (assigned card value, guess → nearest authored value, unanswered →
  nowhere). Module doc rewritten (the tally now takes the nearest value from the definition it is folded with).
- `P/🔨️modules/🗄️storage/🦀️.rs` — `FORMAT_VERSION` 2 → 3 with its doc; storage unit test now refuses a v2 file.
- `P/🏗️bootstrap/🟦️.ts` — `PROCTOR_STORAGE_FORMAT.version` 3 (the site's local-stack test compares it to the Rust source).
- `P/🧫️fixtures/📚️catalog/🔣️.json` — badge `perfect-power` asks for `"challenge": "medium"`, so every proctor suite
  loads and decides a badge rule with a least challenge.
- Unit tests: `🎭️actors` (shared `perfect` answers guesses where the sheet hides the keys; new test of the clock of a
  timed run through the framework port: `task-unopened`, `unknown-task`, floor of an opening and of an answer, once per
  task, an answer at the limit accepted however late it is decided, `time-up` one millisecond later however early it
  is decided, a device clock ahead taken as claimed, `run-untimed`, another challenge voids the open run),
  `🧩️instance` (command list, actor kinds, offline policies pinned; policy grants `open-task`), `🔭️projections`
  (`answer`/`play` take a challenge and open timed tasks; new test: best by points across challenges, voided run on a
  switch, an expert run view with `opened: {}`, board = core leaderboard; an expert run's `task-opened` leaves learner
  view, transcripts and board alone; crowd folded from a medium and a hard (guessed) run equals `crowd_view`), `👪️crowd`
  (random results of every challenge with guesses, unassigned items, unguessed sortings; new deterministic test of the
  nearest value, the tie to the smaller value, unanswered items), `❓️queries`, `⌨️cli`, `🗄️storage`.
- `P/🧪️tests/🌐️end-to-end/🦀️.rs` — helpers `start`, `open_task`, `submit`, `now`; `record` stamps `at`; `perfect` guesses
  where keys are hidden; `play` takes a challenge. Main test: `run-started.challenge`, `command-malformed` for a missing
  or unknown challenge, points and `best` as `Best` on learner view and board (200 per perfect medium run). Crowd test:
  the second learner plays **hard** with reversed, guessed sortings and guesses ×1.5 — crowd still byte-equal to
  `crowd_view`. New test `every_challenge_deals_its_own_sheet_and_earns_its_own_points`: easy (keys, cards, hints on a
  reversed order, none after the right one, only `sorter` earned), medium (keys, guesses refused, no hints, 0.5 → 100
  points), hard (no `value`/`keys`/`cards`/`seconds`, assignments refused, a guess four decades off misses, points =
  score × 300), expert (`run-untimed` on a medium run, switching voids it, `run-open` at the same challenge, `seconds`
  62 and `opened: {}`, axes `{id, label}` only, `task-unopened`, an opening dated an hour ahead taken as claimed, an
  answer dated 0 raised to the opening, an answer at the limit accepted, one ms later `time-up`, partial submission
  with two unanswered items, points = score × 400), learner runs/best/total and board row, all views equal after a
  restart. `largest_answer` also measures guessed answers with the longest JSON number.
- `P/🧪️tests/🏋️capacity/🟦️.ts` — `start-run` (hall and script) carry `challenge: "medium"`, `record-answer` carries
  `at: Date.now()`, `answerOf` reads the now optional `cards`. Not run (needs a quiet machine, ~5 min).
- `P/README.md` — boot step 3 (revision 6), caps table (`run-open` vs voiding), Wire: command table with payloads,
  offline policies and `at` semantics, new rejections, leaderboard rows by points, crowd folding, `quiz.run` per
  challenge, body limit figure, Data (format v3), Layout (end-to-end coverage).

## Gates

All run from the repo root after the quiz crate's floor-only revision landed.
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` (fundamental): **90 unit + 15
  conformance + 18 end-to-end passed, 0 failed** (before: 87 + 15 + 17). Run twice; the second after the clippy fix.
- `cargo clippy -p teaching-proctor --all-targets -- -D warnings` (in `🎓️teaching`): clean, after replacing two
  pre-existing `% 3 == 0` in test helpers by `is_multiple_of(3)` (`🔭️projections`, `❓️queries` unit tests).
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:check --skip-nx-cache` (release build, no Docker):
  succeeded; the architecture catalog already carries `"challenge": "medium"` on the four `*-expert` badges and both
  `perfect-tasks` rules, and the proctor accepts it.
- `tsc -p TICKET/tsconfig.proctor.json` (capacity gate + bootstrap): 0 errors.
- Not run: the capacity gate `@teaching/proctor:capacity` (needs a quiet machine, ~5 minutes with the release build).
- An earlier run against the crate before its revision failed exactly the floor-only assertions (expected).

## Deviations and decisions

1. **Body-limit headroom 4× → 3×.** With guesses, the largest real `record-answer` of the architecture catalog is
   4186 bytes when every guess is written as the longest JSON number (`-1.2345678901234567e-300`), 3.9× below the
   16384-byte limit; the end-to-end assertion now asks for 3× and the README states 4186 bytes. The limit is unchanged.
2. **No waiting test for `time-up`.** With the floor-only rule the clock is decided on claimed instants, so the end-to-end
   test reaches `time-up` deterministically with explicit `at` values; nothing waits for the wall clock.
3. **Crowd of a quiz outside the catalog.** The projector tallies a result only when its quiz is in the current catalog
   (it needs the definition for the nearest value). Such a tally was never viewed before either; a changed catalog
   refolds everything anyway (fingerprint stamp).
4. **Fixture badge with a least challenge** instead of a separate catalog unit test: every suite then exercises it.

## Notes for the next agent

- `CrowdTally::fold(quiz, result)` is the one signature change; everything else of the proctor's API is unchanged.
- The fixture's `perfect-power` now needs medium or harder: an easy perfect power run earns only `sorter`.
- Clients must send `quiz.open-task` with `at` before the answers of an expert task; the proctor answers `run-untimed`,
  `task-unopened`, `time-up` (the TypeScript `REJECTIONS` must list them).
