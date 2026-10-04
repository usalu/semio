# Audit — teaching proctor on the challenge levels (read-only, 2026-10-03)

Scope: `git diff HEAD -- 🎓️teaching/🛂️proctor` (17 files), read against `📓️design.md` (§1, §2, §3.5, §4, with the 2026-10-03
revisions) and `📓️explore-proctor.md`; `📓️report-proctor.md` read after forming the view. `P` = `🎓️teaching/🛂️proctor`,
`Q` = `🧰️framework/🛍️products/❓️quiz`. No cargo, no test run; every claim below is by reading code. Lines with `[V]` were
verified at the cited line, `[I]` is inference.

## Verdict

No wrong result, no format disagreement and no wire hole was found in the proctor's own code. Two **resource-bound
regressions** come from the new lifecycle (one in `start-run`, one in `open-task`); both break the README promise "No
stream and no table grows without bound" (`P/README.md:128`). Formats, admission, projections and crowd tally are
consistent. The rest is coverage and doc drift.

## 1. Findings, ranked

### S1 (security / resource bound, high) — switching the challenge voids runs without any cap

`Q/🔨️modules/🧾️lifecycle/🦀️.rs:199-211` [V]: an open run of the quiz at another challenge is voided and a new run started.
The caps at line 204 count only `Submitted` runs (`limits.runs`, `limits.runs_per_quiz`); `Voided` runs are never counted.
Before this change `run-open` stopped a second open run of the same revision, so a new run needed a submission (capped at
200 per quiz) or a quiz revision.

Scenario: one registered anonymous learner (one sign-up) sends `start-run easy`, `start-run medium`, `start-run easy`, …
with fresh run and command ids. Every command is accepted (`[run-voided, run-started]`, 2 events + a receipt row + 2 outbox
rows) and `LearnerState.runs` grows by one `RunState` each time. At the documented 300 commands per second per address
that is about a million voided runs per hour from one address. Consequences [I from reading]:
- the learner's whole `Folded` state (STATES) is re-serialised on every batch that touches it (`P/…/🔭️projections/🦀️.rs`
  `written`), and the learner view lists every run (`learner_view` maps all `state.runs`), so state, view and fold cost
  are O(runs): O(n²) over the attack, and `quiz.learner` reads grow without limit for everybody polling that learner;
- each voided run may also keep up to 2000 recorded answers of up to 16 KiB each in the state;
- the decider rehydrates the stream from the log on each activation.

Fix (core + README): cap all runs of a learner (`runs-exhausted` counting open and voided, e.g. a fixed
`runs_per_quiz × something`), or refuse a switch away from an open run that has no answers more than N times, or drop voided
runs from the state (an event-sourced projection can keep a count). Update README caps table row 135 and add a test that
alternating `start-run` stops with a rejection.

### S2 (security / resource bound, medium) — a repeated `open-task` is an accepted no-op that still stores a receipt

`Q/…/🧾️lifecycle/🦀️.rs:236` [V] answers `Ok(Vec::new())` for an opened task. The authority treats that as an accepted turn
with zero events and still commits its receipt: `Turn.receipt` is built unconditionally (`🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs`
`decide`, "Stage"), `commit` binds `staged.key` (the command id) and `P/🔨️modules/🗄️storage/🦀️.rs` `commit` →
`bind_receipt` inserts a `proctor_receipt` row for every turn that carries a key. Rejections store no receipt; this accepted
no-op does.

Scenario: a learner with one open expert run and one opened task sends `open-task` with a new command id 300 times a
second from one address. Each answer is `accepted` with `events: []`, each adds one `proctor_receipt` row (a text key of 32
characters plus six more columns, about 150 bytes with its index): roughly 160 MB per hour from one address, unbounded
by `answers_per_run` (which counts only `answer-recorded`), by the 2000-answer cap or by the sign-up allowance
(a registered learner is enough). This is a new unbounded table growth that did not exist before `open-task`.

Fix: answer a repeat with a rejection (for example `task-opened`, or reuse nothing and reject `run-closed`-style) so no receipt
is stored, or count openings against `answers_per_run`. README row 358 ("a repeat emits nothing") must follow. Add a test
that 1000 repeated `open-task` commands leave `proctor_receipt` at its size after the first.

### S3 (security, low) — `at` is unbounded in both directions; no overflow, but two side effects

`acted` is `max(at, floor)` (`Q/…/⛰️challenge/🦀️.rs:134` [V]); `record_answer` uses `at.saturating_sub(opened)` and
`seconds.saturating_mul(1000)` (`lifecycle:252` [V]); `task_seconds` saturates (`challenge:129` [V]). So no `u64` overflow or
underflow is reachable: a past `at` is raised to the floor, a future `at` simply moves the clock. Side effects:
- A client that opens a task with `at = u64::MAX` (serde accepts any `u64`; `Timestamp` has no maximum in the schema) gets a
  `task-opened` and a `RunView.opened` holding 1.8e19, which a TypeScript reader parses with lost precision (above 2^53);
  only that learner's own run view is affected [I].
- A future `at` on `open-task` gives the learner unlimited time for that task. This is the documented trust model (the
  device holds every solution), so it is not a finding by itself; the README says it ("the proctor's own clock never enters
  a time verdict").
Suggestion: reject or clamp an `at` above `Number.MAX_SAFE_INTEGER` at admission (`command_rejection`) so a view never
carries an unsafe integer. Not covered by any test (see C5).

### D1 (doc drift) — stale test name

`P/🔨️modules/🎭️actors/🧪️tests/🔬️unit/🦀️.rs:270`: `the_clock_of_a_timed_run_runs_from_the_opening_the_device_claims_bounded_by_the_proctors_own`.
After the floor-only revision there is no bound by the proctor's own clock; the assertion message at line 300 says the
opposite ("the proctor's own clock is no verdict"). Rename (for example `…_the_device_claims_and_the_proctors_clock_is_no_verdict`).

### D2 (doc drift) — README, small

- `P/README.md:128` ("No stream and no table grows without bound") and the caps table row 135 are false as of S1/S2.
- `P/README.md` Wire section opens the new command table with "(design §8 of the challenge levels)": a README should not
  point at a ticket document; name the rule instead.
- The "largest real command is 4186 bytes … 3.9 times below" figure comes from the new assertion's own measurement; the
  assertion only asks for 3× (`P/🧪️tests/🌐️end-to-end/🦀️.rs` `largest * 3 <= limit`), so the README claims more than the
  test guards. The 4186 was not re-measured in this audit.
- The limits table "What a class of 300 needs" still counts commands per second from medium-only play; expert runs add one
  `open-task` per task (about 2 to 4 more commands per quiz).

### C-coverage gaps (all are about what the tests do not prove)

| # | Gap | Where |
|---|---|---|
| C1 | End to end, expert is played only for the classification task of `homes`. No expert sorting or matching (the `dimensions` multiplier of `seconds`, guesses under a clock, a partial sorting or matching submission), although the unit helper `answer` plays them all through the bus. | e2e `every_challenge_deals_its_own_sheet_and_earns_its_own_points` |
| C2 | Hints end to end: only a sorting magnitude hint. No matching hint (with `dimension`), no classification `misplaced` count, no check that the hints disappear on submit or when the run is voided (the run view is rewritten on those events: `fold.runs`, `run_of`, so this holds by reading, not by a test). | same test |
| C3 | Hard matching: only a perfect guess; no matching miss, no unguessed item in an open hard run. The crowd e2e uses hard but multiplies every guess by 1.5, which never misses. | same, crowd test |
| C4 | Crowd equality with `quiz::crowd_view` in e2e covers complete runs only; unanswered items and a sorting nobody guessed in are covered by the unit test `a_guess_counts_under_the_nearest_authored_value…` and the random fold test, not over HTTP. Both oracles call the same `crowd_value`, so e2e proves incremental = batch, not the nearest-value semantics (that sits in the shared vectors). | e2e crowd test |
| C5 | Wire edge cases: no test for a malformed `at` (missing, negative, fractional, above `u64`, above 2^53) or a malformed `open-task`; `command-malformed` is asserted for `challenge` only. No test of `open-task` on a closed run (`run-closed`) or a revised quiz (`quiz-revised`) through the proctor. | e2e, actors unit |
| C6 | No test for S1 (alternating `start-run`) or S2 (repeated `open-task`). | missing |
| C7 | Capacity gate plays `medium` only and was not run (`report-proctor.md`): `open-task`, expert, hints recomputation per answer, and the voiding path are unmeasured. | `P/🧪️tests/🏋️capacity/🟦️.ts` |
| C8 | Projections: `RunView` after `answer-recorded` with hints is covered through the API in e2e (not in `projections` unit tests); a projections unit test of the hints would pin the "run view is rewritten, learner view is not" rule directly (the new test pins it for `opened` only). | `🔭️projections` unit |

Time assertions are deterministic: every time verdict in e2e and unit tests uses explicit `at` values (`opened_at = now() +
3_600_000` is only an arbitrary base; nothing waits, nothing depends on delivery time). The one wall-clock reading that
matters, `play()` on a timed run in e2e, is never called with `expert` (only medium), so there is no latent 30-second flake.

## 2. Checked and found consistent

1. **Wire and admission.** `quiz.open-task` is in the manifest (`P/🔨️modules/🧩️instance/🦀️.rs:208`, learner stream, `Optimistic`)
   and in the learner policy template (line 236); `command_target` falls through to the learner for every command but a
   handle claim (`P/…/🎭️actors/🦀️.rs:103-108`) so no change was needed; `quiz::command_rejection` checks `open-task` ids and
   the task slug (`Q/…/✅️validation/🦀️.rs:264`); the kind check `envelope.kind == quiz.<type>` makes `open-task` mismatches
   fail with `envelope-mismatch`. Tests pin command list, actor kinds, offline policies, grants and a negative grant on a
   handle stream, and the sign-up allowance excludes it. The TypeScript `commandTarget`/`commandEnvelope` build the kind from
   `command.type`, so no list there needs `open-task`. README table, rejections (`run-untimed`, `task-unopened`, `time-up`),
   "five command kinds" docstring: consistent. Idempotency: the same command id replays the stored receipt (events `[]`,
   as for every command); a repeat with another id is the S2 case.
2. **Formats.** `FORMAT_VERSION` 3 in `P/🔨️modules/🗄️storage/🦀️.rs:68`, `P/🏗️bootstrap/🟦️.ts:70`, the storage unit test (refuses v2,
   asserts `FORMAT_VERSION == 3`), README (v3 and the sample message), and the site's local-stack test reads the Rust constant
   by regex, so a drift would fail there. `STATE_FORMAT` 2 (`actors:79`, both deciders use it, unit-tested through
   `state_format()`), `PROJECTOR_REVISION` 6 (in the stamp; the unit test resets from `REVISION - 1`). `serve` refuses any
   other version in `check_format` (`stamp_format`/`verify_format`); the dev launcher `settleDevelopmentData` moves the
   launcher's own folder aside on any difference of schema or version (older or newer) and refuses a named `PROCTOR_DATA`.
   No other place in the repo outside tickets names the proctor format.
3. **Projections.** `task-opened` is in `run_of` and in the fold match, so the run view is rewritten (`fold.runs`);
   `viewed` (learner view) and `ranked` (transcript) skip it and `answer-recorded`; that is correct because `learner_view`
   reads only identity, runs' challenge/status/score/points, badges and bests, none of which `opened` or answers touch. The
   run view is recomputed from state on every rewrite (`run_view`), so `hints` are fresh after each `answer-recorded`
   and absent after `run-voided`/`run-submitted` (both rewrite it; hints need `status == Open`). Transcript and board carry
   `challenge`/`points`; best compares points with strict `>`.
4. **Crowd.** `CrowdTally::fold(quiz, result)` mirrors `crowd_view`: classification skips unassigned items, sorting adds
   places only when `crowd_orders(items)` and still bins the score, matching uses `crowd_value` (card value, nearest authored
   value for a guess, nothing when unanswered), dimension scores are binned before an unknown task/dimension is skipped.
   The tally depends now on the catalog's quiz, and the projector fold is skipped for a quiz outside the catalog (its view
   was never written); a changed catalog changes the fingerprint and refolds. The only way to diverge from `crowd_view` that
   I could construct is a result whose quiz is missing from the catalog: no view exists for it, so no mismatch is visible.
5. **Body limit 4× → 3×.** Justified: the largest answer now includes guesses written as the longest JSON number
   (`-1.2345678901234567e-300`, an adversarial rather than a real client string; real `JSON.stringify` of a double is at
   most 24 characters, the same bound), 4186 > 16384/4. The limit itself is unchanged; an answer is capped per task by the
   sheet (guess keys must be sheet items) so a client cannot inflate one command beyond the measured shape. See D2 for the
   README/test mismatch. `open-task` is small (about 400 bytes).
6. **Repo rules.** Changes are concise and carry no comments inside definitions; new and changed docstrings start with an
   emoji (`🕰️ ✍️ 🎬️ ⏱️ 📤️ 🧩️` in the e2e file are not repeated in the file; the repeats `📨️ 🔌️ 📦️ 🔚️`/`🎽️ 🆕️` predate this
   change). No compatibility shim, no legacy path, nothing migrated. The two clippy fixes (`is_multiple_of`) are in test
   helpers only. `P/🔨️modules/👪️crowd/🦀️.rs:97-110` has two long chained lines that rustfmt-style readers will find dense, but
   they follow the file's existing style.

## 3. Recommended order of work

1. S1: cap runs including voided ones in the quiz core (`start_run`), mirror in the TypeScript twin, vectors, README row 135.
2. S2: make a repeated `open-task` a rejection (or a counted event) in the core, twin, vectors, README row 358.
3. S3: bound `at` at admission (both twins).
4. C1-C3, C5, C6 as end-to-end/unit tests; C7 once the machine is quiet.
5. D1, D2.
