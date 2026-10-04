# Report — contract revisions of the challenge levels (2026-10-03)

Agent: contract revisions. `Q` = `🧰️framework/🛍️products/❓️quiz`, `P` = `🎓️teaching/🛂️proctor`, `R` = `Q/🎯️targets/⚛️react`,
`T` = this ticket folder, `G` = `.🧬semio/…/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`.

Five rules, end to end through schema, both cores, the Python references and vectors, the proctor, the React client and
the READMEs:

1. **`start-run` carries `at`** (audit D1). The command is `{ id, learner, run, quiz, challenge, at }`, and `at` is required.
   - `run-started.at = command.at`: the run starts at the device's claim.
   - `run-voided` keeps the decider's `now`.
   - `open-task` and untimed answers keep `acted(at, run.startedAt)`, so their floor is now the device's start.
2. **`REACH_SLACK = 1e-9`.** `misses` compares against `reach × (1 + REACH_SLACK)` on both scales.
   - `0.018` against `18` is now within reach.
   - Exactly the widened reach does not miss; the next double above it does.
3. **Caps count every started run** (proctor audit S1). `runs` and `runsPerQuiz` count `state.runs` whatever their status (open, submitted or voided). Switching the challenge back and forth now ends in `runs-exhausted`.
4. **`already-opened`** (proctor audit S2). This is a new rejection. A repeated `open-task` is refused, so no accepted empty turn stores a receipt.
5. **`at` is a safe integer** (proctor audit S3).
   - Schema: `Timestamp.maximum = 2^53 − 1`.
   - TypeScript and Rust both export `MAX_TIMESTAMP`.
   - Both cores refuse an `at` above it on `start-run`, `open-task` and `record-answer` in `commandRejection`, with `id-invalid`.
   - Rust still decodes any `u64`; negative, fractional and missing values stay `command-malformed`.

## Changed files (all updated; none created in the product; none removed)

**Schema and cores**

- Schema:
  - `Q/🧬️schema/🔣️.json`: `Timestamp` maximum; `start-run` gains `at`; `already-opened`; texts of `Command`, `Rejection` and `Limits`.
  - `Q/🧬️schema/🟦️.ts`: `MAX_TIMESTAMP`, `StartRunCommand.at`, `REJECTIONS`.
  - `Q/🧬️schema/🦀️.rs`: `MAX_TIMESTAMP`, `Command::StartRun.at`, `Rejection::AlreadyOpened` and its `as_str`, docs.
- `Q/🔨️modules/⛰️challenge/{🟦️.ts,🦀️.rs}`: `REACH_SLACK` and `misses`.
- `Q/🔨️modules/✅️validation/🟦️.ts`: new exported `isTimestamp`; `commandRejection` checks the `at` of all three commands.
- `Q/🔨️modules/✅️validation/🦀️.rs`: `command_rejection` bounds `at`.
- `Q/🔨️modules/🧾️lifecycle/{🟦️.ts,🦀️.rs}`: `startRun` (caps over all runs, `at: command.at`) and `openTask` (`already-opened`).

**Unit tests**

- TypeScript (`Q/🧪️tests/`):
  - `🔁️run-lifecycle`: `start(at, …)`; new tests for "counts every started run…" and "starts a run at the instant the device claims…"; `already-opened`; the timestamp bounds.
  - `🧗️challenge-ladder`: the mathjs oracle with the slack; new test "widens the reach by exactly the slack…".
  - `🩺️document-validation`.
- Rust:
  - `Q/🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs`: helpers `command_start(learner, run, at)` and `command_start_at(…, challenge, at)`; new tests `switching_the_challenge_counts_every_started_run_against_the_caps` and `a_run_starts_at_the_instant_the_device_claims_whenever_it_is_decided`.
  - `⛰️challenge` unit: new test `the_slack_widens_the_reach_by_a_billionth_and_not_one_double_more`.
  - `✅️validation` unit, `👁️views` unit and `🧬️schema` unit.

**Python references, cases and vectors**

- References `🐍️.py`:
  - `⛰️challenge-rules`, `📏️sorting-concordance`, `🔀️matching-concordance`, `🧾️learner-lifecycle`: slack. The numpy corroboration uses `multiply`/`add`.
  - `🧾️learner-lifecycle` also: `start-run.at`, caps, `already-opened`, `is_timestamp`/`MAX_TIMESTAMP` in `command_rejection`. Its learner view now lists runs in reverse start order, as the twins do, instead of by `startedAt`.
  - `🪪️identity-shapes`: `at` counts as an id-shaped member.
- Features `🥒️.feature` of the same five cases.
- Bindings: `🧾️learner-lifecycle/{🟦️.ts,🦀️.rs}` (the site play stamps `at`).
- Generator `G` (small hunks):
  - `Learner.start(…, at=None)`;
  - new helper `slack(bound)`;
  - renamed and new boundary vectors;
  - new learner sequences;
  - the `start-run` instances carry `at`;
  - `write` falls back to an in-place write when another process maps the fixture.
- Fixtures regenerated: `⛰️challenge-rules`, `📏️sorting-concordance`, `🔀️matching-concordance`, `🧾️learner-lifecycle`, `🧬️schema-conformance`, `🪪️identity-shapes`.

**Proctor**

- Code and docs:
  - `P/README.md`: caps table, `PROCTOR_MAX_RUNS`, the wire table rows for start-run and open-task, the `at` paragraph, the rejections. The README no longer points at a ticket document.
  - `P/🔨️modules/⌨️cli/🦀️.rs`: log text "started runs".
  - `P/🔨️modules/🎚️config/🦀️.rs`: docs.
- Unit tests:
  - `🎭️actors`: `at` on every start; repeated opening gives `already-opened`; switching until `runs-exhausted`; test renamed per audit D1.
  - `🔭️projections`, `⌨️cli`, `🧩️instance`: `at`.
- `P/🧪️tests/🌐️end-to-end/🦀️.rs`:
  - `start` stamps `now()`;
  - a start without `at`, or with `at` of −1 or 1.5, gives `command-malformed`; `at` of `u64::MAX` or 2^53 gives `id-invalid`;
  - a repeated `open-task` gives `already-opened`;
  - new: a run started 10 min earlier keeps its device start, and its opening is raised to it.
- `P/🧪️tests/🏋️capacity/🟦️.ts`: `at: Date.now()` on both start-run sites.

**React** (small edits; re-read before each)

- `R/🔨️modules/🧭️session/🟦️.ts`:
  - `startRun` stamps `at: this.now()`;
  - `openTask` treats `already-opened` as success and re-reads when the held view lacks the opening;
  - `decisionSettled` raises no notice for a delivered `open-task` refused `already-opened`.
- `R/🔨️modules/📮️outbox/🟦️.ts`: `restoredCommand` restores a `start-run` only with an `at` that passes `isTimestamp`.
- `R/🔨️modules/💾️persistence/🟦️.ts`: `isTimestamp` is now re-exported from the core, which removes the duplicate the client audit noted.
- `R/🔨️modules/🌐️i18n/🟦️.ts`: `quiz.rejection.alreadyOpened`:
  - EN "The clock of this task is already running.";
  - DE "Die Uhr dieser Aufgabe läuft bereits.";
  - an entry in `REJECTION_LABELS`.
- Tests:
  - `Q/🧪️tests/🫡️deputy-decisions`: `at`; a repeat opening gives `already-opened`.
  - `Q/🧪️tests/📬️outbox-delivery`: `at`; undated and 2^53 starts are not restored.
  - `Q/🧪️tests/🚶️learner-journey`: `at: proctor.now` on seeded starts.

**Other**

- `Q/README.md`: reach and miss, the clock, the lifecycle table rows, the `at`/timestamp paragraph, the caps, the outbox and deputy paragraphs, the Rust names.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`: regenerated (see Red).
- Ticket inputs kept (in `T/`): `stamp_start_at.py`, `stamp_start_at_typescript.py`, `stamp_proctor_start_at.py`, `stamp_react_start_at.py`.

## New vectors

- **`⛰️challenge-rules` misses (34 → 46; `📏️ guessed` now 27, `🔀️ guessed` 22, learner sequences 23, shapes 57):**
  - Within: `log-typed-thousandth-of-eighteen-within` (0.018 vs 18), one double beyond ×1000 / ÷1000 / the root (renamed `…-within-the-slack`).
  - Beyond: `log-thousandfold-of-eighteen-beyond-the-slack` and `log-typed-guess-below-a-thousandth-of-eighteen-misses`.
  - At the slack boundary on the log cap, from both sides: exactly `slack(1000)` does not miss, and `nextafter` does.
  - The same at the root of the ratio, and at the linear reach from both sides.
  - The generator asserts the exact list of misses.
- **`📏️ guessed` (+2):**
  - one double beyond ×1000 of the largest is within the slack (score 1);
  - ×1000 × (1 + 2e-9) is beyond;
  - a typed thousandth of the smallest is within;
  - a typed ten-thousandth misses.
- **`🔀️ guessed` (+1):** one double beyond is within the slack; 2500.000005 is beyond.
- **`🧾️` learners (+4):**
  - `switching-until-the-cap`: easy/medium/easy alternation leads to `runs-exhausted` per quiz and in total.
  - `started-offline-decided-on-the-device` and `started-offline-delivered-late`: the audit's D1 scenario, delivered 73 s late. The generator asserts both give identical verdicts and instants relative to the device start: an opening at +5 s, an opening dated 0 raised to the start, `time-up` at limit+1 and +20 s, accepted at the limit.
  - `the-last-instant`: a start at 2^53 − 1, and a repeat opening gives `already-opened`.
  - `the-clock` now also expects `already-opened`.
- **`🧾️` malformed:** a start, an opening and an answer beyond 2^53 give `id-invalid`.
- **`🪪️ shapes` (+5):** starts at 0 and at 2^53 − 1 accepted; 2^53 (start, open) and 2^64 − 1 (answer) give `id-invalid`.
- **`🧬️` instances:** a start at 2^53 − 1 conforms, plus `rejection-already-opened`. These are refused: a start without `at`, a start beyond a safe integer, a `run-started` beyond a safe integer.
- The deputy suite replays every lifecycle sequence, so the D1 scenario is decided on the device by the same core.

## Gates (exact commands, observed)

| Gate | Command | Result |
|---|---|---|
| Fixtures | `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe T/regenerate_challenge_vectors.py` (repo root, second full run) | 12 of 12 `unchanged` |
| TS core | in `Q/📦️packages/🟦️typescript`: `SEMIO_TEST_BUDGET_MS=300000 bun ./📜️script.ts test` | 11 files, **413 passed**, 0 failed |
| Core typecheck | `bun node_modules/typescript/bin/tsc --noEmit -p T/tsconfig.core.json` | **0 errors** |
| Rust core | `RUSTC_WRAPPER="" cargo test -p semio-framework-quiz` | **179 passed**, 0 failed |
| Clippy | `cargo clippy -p semio-framework-quiz --lib --tests -- -D warnings` | clean |
| Parity | in `…/🦑️repo/🔨️modules/🧪️test`: `RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | cases=14 executed=129 passed=129 **parity=129/129** (no new scenarios; the new vectors sit in existing ones) |
| Proctor | `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/proctor:test --skip-nx-cache` | **90 unit + 15 conformance + 18 end-to-end passed** |
| Proctor clippy | `cargo clippy -p teaching-proctor --all-targets -- -D warnings` (in `🎓️teaching`) | clean |
| Proctor TS | `tsc --noEmit -p T/tsconfig.proctor.json` (capacity + bootstrap) | 0 errors |
| React | in `R/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | 22 files, **801 passed**, 0 failed |
| React typecheck | same folder: `bun ./📜️script.ts typecheck` | 6 errors, all in `🐾️pets/🔨️modules/{💞️sociability,🚶️locomotion}` (pets session); 0 in quiz files |
| Taxonomy | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | clean=true errors=0 warnings=0 |

## Red or not run

- **Proctor budget:** the first proctor run without `SEMIO_TEST_BUDGET_MS` was killed by the 15 s fundamental budget while the end-to-end binary ran, because the host was busy. With the larger budget the same suite passed.
- **Schema catalog:** `bun ./📜️script.ts schema generate --check` was stale.
  - I regenerated it (3734 scopes). The new entries are pets `$defs` from the pets session.
  - It went stale again right after, because the pets session keeps changing its schema.
  - My quiz changes add no `$defs`.
- **Not run:**
  - the capacity gate (needs a quiet machine);
  - the site and end-to-end Playwright suites (the site agent's area). The site's play driver goes through the React session, so it gets `at` from `startRun`.

## Deviations and decisions

1. **The slack applies to both scales**, as the brief says. On a linear scale a value at the reach is widened by 1e-9 of the reach. No linear vector other than the new boundary ones changed its verdict.
2. **One double beyond an exact boundary is now within the reach.** The former "one double beyond" vectors were renamed `…-within-the-slack`. New "beyond the slack" values were added for each, so every former bound still has a missing neighbour.
3. **Caps:** `state.runs.length` is used for `runs`, and runs of the quiz for `runsPerQuiz`. The cap check stays before the void, as before.
   - Vector `the-cap-before-the-switch` needed `runs: 2` instead of 1, because its open easy run now counts.
   - Unit tests that relied on a submitted-only count were adjusted.
4. **`already-opened` comes after `unknown-task`** in the decision order. The order is open → stale → untimed → unknown task → already opened.
5. **`at` above `MAX_TIMESTAMP` is `id-invalid` in Rust too**, in `command_rejection`, the same path as TypeScript.
   - It is checked before any other rule, so the proctor never folds such an instant.
   - The schema now has `maximum: 2^53 − 1`, so Python and jsonschema refuse such documents as non-conforming. Lifecycle vectors with such instants therefore live only under `malformed`; I moved one there after the schema-conformance check caught it.
6. **Python learner view order:** it now uses the reverse of start order instead of sorting by `startedAt`. Device-claimed starts can now be out of order with the fold, and the twins already reversed fold order (the core-fix report's "Python nit").
7. **Client:** `openTask` treats `already-opened` as success and reloads the run when the held view lacks the opening. A delivered `open-task` refused `already-opened` raises no notice, and the run is already in `unconfirmed`, so it is re-read.
8. **Proctor e2e:** `start` stamps `now()`, the same as `record`.

## Notes for the next agent

- New APIs:
  - TS: `MAX_TIMESTAMP`, `isTimestamp(value)` (core, re-exported by the React persistence module), `REACH_SLACK`, rejection `"already-opened"`.
  - Rust: `quiz::MAX_TIMESTAMP`, `quiz::REACH_SLACK`, `Rejection::AlreadyOpened`, `Command::StartRun { …, challenge, at }`.
- Every builder of `start-run` must stamp `at` from the device clock (React: `session.now()`; proctor tests: the decision time).
- The design owner (coordinator) should check that §2 lists `already-opened` (it does) and that §3.5 states the caps and the safe-integer rule (it does).
