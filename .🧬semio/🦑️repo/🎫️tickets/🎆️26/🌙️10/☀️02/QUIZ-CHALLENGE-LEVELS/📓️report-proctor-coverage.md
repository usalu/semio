# Report — proctor coverage gaps and doc drift (2026-10-03)

Agent: proctor coverage. `P` = `🎓️teaching/🛂️proctor`, `T` = this ticket folder. Input: `📓️audit-proctor.md` (C1–C7, D1, D2),
`📓️design.md` with the 2026-10-03 revisions, `📓️report-proctor.md`, `📓️report-contract-revisions.md`.

## Outcome

Every gap the brief named is covered end to end, the README matches the tests, and the capacity gate now plays every
challenge. Writing the tests turned up one **proctor bug**. It was not in the core, so the core is unchanged.

- **The bug:** the thinking room refused every guessed matching value with `value-unknown`. A learner on hard or expert
  (the React client publishes `thinkingAnswer`, which carries guesses as the schema says) had each draft refused.
- **The fix** is in `P/🔨️modules/👥️presence`.

## Changed files (all updated; nothing created or removed in the repo)

- `P/🔨️modules/👥️presence/🦀️.rs`: new admission rule for drafts.
  - `Shape` keeps the scale of the sorting and of each matching dimension, no longer the card values.
  - A matching draft value, or a sorting guess, is refused only when no answer can carry it: `value-invalid` when it
    is not positive on a logarithmic scale.
  - A sorting guess must name an item of its task (`id-unknown …/guesses/<item>`).
  - `VALUE_UNKNOWN` is replaced by `VALUE_INVALID`; new helper `carried(scale, value)`.
- `P/🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs`: in the existing test, guessed sorting and matching drafts are
  admitted (any value on the linear hours scale). Refused: a zero capacity, a negative sorting guess, and a guess on an
  unknown item.
- `P/🧪️tests/🌐️end-to-end/🦀️.rs`: helpers `receipts(data, learner)` (read-only `Database::erasure`), `concordance`,
  `scored`, type `Scored`. The thinking test now expects `value-invalid` and checks that a guessed draft reaches a peer.
  Three new tests:
  - **`expert_and_hard_runs_clock_guess_and_score_sortings_and_matchings_from_the_instants_the_device_claims`**
    - The expert power run has `seconds` 78 and 102, and its keys are hidden.
    - Malformed `at` on `open-task` gives `command-malformed`: −1, 2.5, "soon", null, or missing.
    - An `at` of 2^53 or `u64::MAX` gives `id-invalid`.
    - A malformed `open-task` is refused: task `Appliances` gives `id-invalid`, a missing task `command-malformed`.
    - Malformed `at` on `record-answer` is refused the same way.
    - None of these refusals adds a receipt.
    - An opening adds exactly 1 receipt. Five repeats give `already-opened` and no further receipt.
    - A partial sorting (2 of 4 guesses) at the limit is accepted; the same task 1 ms later gives `time-up`.
    - The capacities guessed in time are accepted; the hours after 102 s give `time-up`.
    - `opened` lists both tasks.
    - The result: unguessed items `miss: true` with no guess; the hours dimension misses everywhere.
    - The score equals an independently computed concordance; points = score × 400.
    - `open-task` after submission gives `run-closed`.
    - Hard run: `run-untimed`. A capacity guessed ×1000 shows `miss: true` with `assigned` 1e7, and score and points
      match the independent formula (× 300).
  - **`an_easy_run_hints_at_far_off_cards_and_misplaced_items_until_it_closes`**
    - Swapped capacity cards give hints with a `dimension`: nuclear-plant `low`, rooftop-pv `high`.
    - Hours swapped by 1050 lie within reach and give no hint.
    - The sorting gets no hint.
    - After submission there are no hints, and no miss marks where the keys show.
    - Classification hints `misplaced` with count 2, then 1.
    - The voided run has no hints.
  - **`switching_the_challenge_back_and_forth_ends_in_runs_exhausted`**
    - 200 alternating starts are all accepted, the first without a void.
    - Start 201 gives `runs-exhausted`. The same challenge still gives `run-open`.
    - The learner view holds 200 runs, 199 of them voided.
    - The other quiz still starts.
- `P/README.md`:
  - The caps table gains a row "Openings of one task … `already-opened`", plus a receipts paragraph, so "no table grows
    without bound" is true again.
  - The body limit reads "4186 bytes (re-measured 2026-10-03) … more than three times below, the headroom the
    end-to-end test holds".
  - The capacity paragraph describes the challenge mix.
  - A note says the measured needs come from medium-only play.
  - The presence states rule covers guesses and `value-invalid`.
  - The layout row lists the new end-to-end coverage.
  - Already correct, left unchanged: the time rule, caps counting every started run, the `already-opened` rejection.
- `P/🧪️tests/🏋️capacity/🟦️.ts`: each learner plays challenge `CHALLENGES[(index + play) % 4]`.
  - `answerOf` assigns cards where they show and guesses where they are hidden (sorting ordered by its guesses), with the
    draft taken from the core's `thinkingAnswer`.
  - Expert: `open-task` before the answers, and no answer within `CLOCK_MARGIN_MS` (1 s) of the deadline.
  - Easy: the run is read again after each answer.
  - The module doc is updated.

D1 (stale unit test name) had already been renamed by the contract-revisions agent
(`the_clock_of_a_timed_run_runs_from_the_instants_the_device_claims_and_the_proctors_clock_is_no_verdict`); I checked it.

## Gates (exact commands, observed)

| Gate | Command | Result |
|---|---|---|
| Selected e2e while developing | in `🎓️teaching`: `RUSTC_WRAPPER="" cargo test -p teaching-proctor --test end_to_end -- --nocapture expert_and_hard an_easy_run switching_the learners_see_what a_body_past` | 5 passed; a temporary `[DEBUG]` line measured 4186 (architecture) and 2162 (fixture) bytes; removed |
| Proctor suite | `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` | **90 unit + 15 conformance + 21 end-to-end passed**, 0 failed (before: 90 + 15 + 18) |
| Clippy | in `🎓️teaching`: `RUSTC_WRAPPER="" cargo clippy -p teaching-proctor --all-targets -- -D warnings` | clean (exit 0; the only warnings are the replication crate's own, not denied) |
| Capacity typecheck | `bun node_modules/typescript/bin/tsc --noEmit -p T/tsconfig.proctor.json` | 0 errors |
| Capacity gate | not run | needs a quiet machine (about 5 min with the release build); the README figures await that run |

## Deviations and decisions

1. **Presence fix in the proctor.** The schema says a matching draft carries the guessed number, and the client does
   publish it. The room cannot know the publisher's challenge, so it cannot hold a value to the cards. The rule became
   "a number an answer can carry", the answer rules applied to drafts: positive on a logarithmic scale, otherwise any
   finite number (the core checks finiteness). `value-invalid` matches the core's own issue-code family.
2. **Receipts counted from the database.** The end-to-end test counts receipts through a read-only handle beside the
   serving proctor (`Database::open_read_only(…).erasure`), the way `backup` reads while serving. No test-only API was
   added.
3. **Body limit.** Re-measured, not assumed. The README now states the 3× bound that the test guards; 4186 is the
   measured figure.
4. **Capacity margin.** Expert learners stop answering one second before the device deadline. The `at` they send is the
   device instant, so delivery delay never turns into `time-up` errors of the hall.

## Notes for the next agent

- Run `bun nx run @teaching/proctor:capacity` on a quiet machine, then refresh the README figures: the commands-per-second
  and reads-per-second column, and the measured paragraph.
- New presence code: `value-invalid` (was `value-unknown`). No client or test outside `P` named the old code.
- Not covered yet (from the audit): crowd e2e with unanswered items (C4) and a projections unit test of `hints` (C8). Both
  are proven elsewhere: by the unit and fold tests, and through the API.
