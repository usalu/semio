# Audit of the challenge-level cores (read-only)

Scope: design §2 and §3 against `🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}`, `🔨️modules/{⛰️challenge,🃏️sheet,✅️validation,📏️scoring,🧾️lifecycle,👁️views,🏅️badges,👥️presence}/{🟦️.ts,🦀️.rs}`, the Python references under `🧪️tests/*/🐍️.py` and `generate_quiz_vectors.py`. Read independently of the two core reports.
Runs: the TypeScript core suite once (11 files, 406 tests, all passing). Two throw-away probes in the scratchpad (bun, python, and one single-file `rustc`, no cargo) for the findings marked "reproduced". No repo file was touched except this one.

`Q` = `🧰️framework/🛍️products/❓️quiz`.

## Verdict in one paragraph

Both cores follow the design rule by rule (table in §3); I found no deviation from §2/§3 in the code and the twins agree in every place I compared line by line. The two real defects are not deviations from the design but properties of it that the shared vectors cannot see: (B1) the miss test is unstable at exactly the "factor 1000" boundary because it compares `log10` differences in floating point, and (B2) the time limit is judged on instants taken from two different clocks and bounded asymmetrically, so the deputy and the proctor can give different verdicts for the same command. Everything else is parity risk, missing coverage or style.

## Findings, ranked

### B1 — wrong hints and scores for guesses exactly at the reach (bug; reproduced; medium)

`Q/🔨️modules/⛰️challenge/🟦️.ts:75-77` and `…/🦀️.rs:98-100` (and the Python references `⛰️challenge-rules/🐍️.py` `misses`, lifecycle `misses`, sorting `misses`) test `|s(value) − s(truth)| > reach` with `s = log10`. The design says "more than" (strict), so a guess of exactly 1000x (or exactly the half spread) must not miss. In floating point it does for a quarter of all inputs.

Minimal input: `misses(18000, 18, "logarithmic", 3)` is `true`. `Math.log10(18000) − Math.log10(18) = 3.0000000000000004`. Counting integer truths 1…2000 with an exact 1000x guess: 529 miss (26 %), TypeScript and Rust agree (529 and 9 for the /1000 direction, Rust checked with a one-file `rustc`), Python agrees on 20 000 random pairs. At reach 1 (a 2-decade set) an exact 10x guess misses for 20 of 500 truths. Consequences: on easy a key that is exactly 1000x off is flagged "far too high" for some items and not for others (the owner's request is literally "off by a factor more than 1000"); on hard/expert the guess `2200 W` for `2.2 W` costs every pair the item touches for some truths and not for others. The unit slip W/kW is exactly the exact-1000x error the feature is about.
Why the vectors miss it: every boundary vector uses powers of ten (`1000`, `100`, `10`, `[1, 1e9]`), where `log10` is exact.
Fix (all three languages): compare with a tolerance, e.g. `> reach + 1e-9`, or equivalently `|log10(value/truth)|`-free ratio test; add vectors with truths 18, 22, 24, 29 at x1000, /1000 and at the half spread, and linear boundaries with non-dyadic decimals.

### B2 — the same command can be `time-up` or accepted depending on when it is delivered (bug in the design's claim "deputy and proctor decide alike"; reproduced; medium)

`Q/🔨️modules/🧾️lifecycle/🟦️.ts:133,145-148` and `🦀️.rs:226-258`. `open-task` and `record-answer` carry the device's `at`; `task-opened.at = acted(at, startedAt, now)` clamps down to the decider's `now` when the device clock runs ahead, `answer-recorded` is clamped the same way, but a command delivered late (outbox) is no longer clamped. The limit is then `bounded(at_answer) − opened`, with `opened` already shifted back to the proctor's clock and the answer still on the device's clock, so the skew `Δ` leaks into the elapsed time.

Reproduced (`decideLearner` with a one-task expert run, `seconds = 54`): device clock 20 s ahead, open delivered at once (`opened = now`), the learner answers 40 s later (device `at = opened + 40 s + Δ`). Delivered at once: `answer-recorded`. Delivered 2.5 minutes later: `{rejection: "time-up"}`. An honest learner who used 40 s of 54 s loses the answer silently on delivery after the deputy accepted it. The opposite skew (device behind) is lenient, never strict, so the harm is one-sided but real; the header of the lifecycle module ("reach the same verdict") is false under skew.
Also: `run-started.at` is the decider's `now`, so a `start-run` delivered late (arrival time of the proctor) raises the floor of `open-task` above the device's instants; every open then clamps to the start, again only lenient.
Fix options: judge the limit on device-relative durations (keep the device's `at` of the opening in `task-opened`, `time-up` iff `at_answer − at_open > seconds × 1000`, bound each only against absurd values), or clamp the answer by the same skew that the opening revealed (`task-opened` stores `now − at` as offset).

### P1 — parity risks (no divergence found, none excluded)

1. `log10` is the only platform-dependent primitive and sits on a binary decision (B1). TypeScript (V8 fdlibm), Python (UCRT) and Rust (MSVC libm) agree on Windows for 20 000 pairs; glibc (the devcontainer) was not run. If any pair disagrees there the twins disagree on a miss, i.e. on points. The vectors only assert agreement within `1e-12`, and booleans are exact, so one flip fails; but they never test a non-power-of-ten boundary (B1).
2. JSON text of numbers: Rust serialises `300.0`, `0.0`, `1.0` for `f64` (`points`, `Best.points`, `total`, `par`), TypeScript `300`, `0`, `1`. The host compares structurally (`ordered-json-v1`, key order never significant) so the vectors pass; any text comparison, hash or digest of events/views between the two would differ.
3. Map key order: TypeScript `sortedRecord` (`👁️views/🟦️.ts:51`, used for `hints`, `opened`, `best`, `answers`) builds an `Object`; keys that look like array indices (a slug such as `10` or `2024` is valid) enumerate first in numeric order, so the JSON text order differs from Rust's `BTreeMap` byte order (`"10"` before `"9"`). Structural comparison hides it. The design's "member order" paragraph (§2) is untested everywhere (key order is explicitly insignificant in the host).
4. `Math.max/min` vs `f64::max/min`: `scoring/🟦️.ts:186` `Math.max(farthest, d)` vs `🦀️.rs:185` `largest.max(d)` differ on NaN (TypeScript poisons `farthest`, Rust ignores the NaN); `reach` `Math.min(cap, half)` vs the explicit `if` at `⛰️challenge/🦀️.rs:84-93` likewise. Unreachable today: axis ranges (`max > min`) and profile values (in range) are validated, true values on a logarithmic scale must be positive. Worth a one-line note because a quiz that bypasses `quizIssues` would score differently.
5. Optional members given as `null`: Rust `Option` fields with `#[serde(default)]` accept `"guesses": null`, `"assignments": null` as absent; TypeScript `answerRejection` rejects them (`isObject(null)` false). The deputy is therefore the stricter side (harmless), but the validators are not twins on that input.
6. Timestamps: Rust `Timestamp` is `u64` and refuses `12.5`, `-1`, `1e999` (command-malformed); TypeScript `commandRejection` checks neither `at` nor `challenge`, `acted(NaN, …)` is `NaN` and an `answer-recorded` with `at: NaN` would serialise as `null`. The client stamps `Date.now()` (integer) so it cannot happen today, but the deputy is the lenient side here; the design says the types keep it out. No test covers it ("no shared vector covers it" by design).
7. Sort stability and ties: `ascendingItems` (stable, tie by definition index) and Rust `sort_by(... .then(index))`, `submittedRuns` (stable by submittedAt) and Rust `sort_by_key` (stable), standings (`total`, badges, `reachedAt`, learner id): equal. Totals are sums in catalog order in both (`views` TS 96-98, Rust `total`), so floating tie-breaks agree.
8. Integer overflow: Rust `task_seconds` saturates, `time-up` uses `saturating_sub`/`saturating_mul`, `acted` is `max/min` on `u64`; TypeScript doubles are exact below 2^53. No divergence.
9. `{}` vs absent: `RunView.opened` is `{}` on a timed run nothing opened (both), absent on untimed (both); `hints` absent when empty (both); `SheetAxis` `unit/min/max` absent together (both); `Category.profile` `{}` after sheet shares (both; the schema's `Profile.minProperties: 1` would reject it, but it needs an invalid quiz).

### Design conformance, rule by rule (OK unless noted)

| Rule | Result |
|---|---|
| `CHALLENGE_RULES`, `challengeRank`, `challengeMeets`, `points` (§3.1) | OK, TS `⛰️challenge/🟦️.ts:21-55`, Rust `🦀️.rs:33-63` |
| `reach`: cap 3 log, half spread, zero spread = cap, linear = half spread | OK (`🟦️.ts:61-72`, `🦀️.rs:71-94`); strictness broken numerically, see B1 |
| `misses` strict `>`; `+∞` never misses | OK in form; B1 in floating point |
| `taskSeconds` (matching x dimensions) | OK; Rust saturates |
| `acted = min(max(at, floor), now)` | OK; the floor-after-now case returns `now` (< floor) in both, corroborated against `numpy.clip` |
| `hintsOf` sorting: per position of `answer.order`, ladder key vs item value, reach over `keys` | OK; entries not resolvable give none (both) |
| `hintsOf` matching: sheet order of dimensions and items, reach over presented values per dimension, card index range-checked | OK (TS does not check integrality, Rust cannot hold a fraction; validated before it is recorded) |
| `hintsOf` classification: count of assigned items outside their category | OK |
| Sheet: identical draws at all challenges (`items`, categories shuffle, card shuffle drawn even when hidden), keys ascending computed before rotation, cards absent, descriptions dropped, axes `{id,label}`, shares `(v-min)/(max-min)` with ids of no axis dropped, seconds only when timed, member order | OK (`🃏️sheet/🟦️.ts:64-101`, `🦀️.rs:46-125`) |
| Validation: sorting with keys rejects guesses; hidden requires finite (positive on log) guesses of presented items in non-decreasing order along `order`; matching with cards rejects guesses, hidden rejects assignments; complete = recorded order / every item guessed / every item in every dimension | OK in both (`✅️validation/🟦️.ts:640-734`, `🦀️.rs:192-241`). Sorting `order` must always be a full permutation (also on timed sheets), so `reach` over `order` equals `reach` over the sheet |
| `commandRejection` for `open-task` (ids and task slug) | OK; `challenge`/`at` unchecked by design (P1.6) |
| Scoring: keys-hidden sorting (misses, no guess, `total = 0` case), keys-hidden matching (miss pair, opposite order, tie = half), unanswered classification, `guess`/`miss`/`assigned` presence, partial answers only on timed sheets | OK (`📏️scoring/🟦️.ts:84-224`, `🦀️.rs:36-208`). `score` stays in [0, 1] (discordant is a monotone sub-sum of total) |
| Lifecycle `start-run`: same challenge -> `run-open`, other -> `[run-voided, run-started]`, stale -> void, cap before void | OK (TS 111-122, Rust 190-224), same order of checks |
| `open-task`: `unknown-run`, `run-closed`, `quiz-revised`, `run-untimed`, `unknown-task`, already opened -> `[]` | OK (TS 125-134, Rust 226-239) |
| `record-answer`: through `unknown-task`, `task-unopened`, bounded instant, `time-up` iff `t - opened > seconds*1000`, then `answer-invalid`, `answer-recorded.at = t`; untimed bounded to `[startedAt, now]` | OK (TS 137-152, Rust 241-258); semantics under skew: B2 |
| `submit-run`: untimed `run-incomplete`, timed never (the `!result` fallback is unreachable) | OK |
| Views: `opened` on timed runs (`{}` when none), `hints` only open + hinting + non-empty, tasks in code point order | OK (TS 129-150, Rust 79-106) |
| `learnerView`/transcripts: `best` by points with strict replacement, `total` over catalog quizzes in catalog order, `RunSummary.points` with `score`, `reachedAt` = last raise | OK (TS 84-114, Rust `bests`, `total`, `learner_view`) |
| Badges: least challenge on `perfect-quiz`/`perfect-tasks`, none on `completed-quizzes` | OK (TS/Rust 1:1) |
| Crowd: score bins from accuracy at every challenge, guess -> nearest authored value of the whole task, tie -> smaller, unanswered -> nowhere, task without any answer -> score bin only | OK, one nuance below |

Nuance (style): the crowd excludes a sorting result from places when every item is a miss without a guess (`👁️views/🟦️.ts:325`, Rust `crowd_orders`). That also drops an expert result that carries a recorded `order` but not a single guess, wider than "a task without any answer" in design §3.6. Harmless (the order is meaningless without guesses), but document it in the design.

### Python independence (Q3)

- `⛰️challenge-rules`, `🃏️sheet-assembly`, `📏️sorting-concordance`, `🔀️matching-concordance`: genuine second implementations written to the design text, each corroborated by a third-party library on the new formulas: numpy `ptp`/`minimum`/`log10` for `reach`, vectorised comparison and `sign` for misses and hint directions, `clip` for `acted` (also covering floor > clock), `prod` for seconds, `sort` for the ladder, `interp` for profile shares, `numpy.random.MT19937` for the stream, scipy Kendall/Spearman for concordance direction, `Fraction` for points. Strong.
- `🧾️learner-lifecycle` (and `✅️`, `📊️`, `🏆️`): plain re-implementations that copy the helper functions of the challenge module (`acted`, `misses`, `hints_of`) and follow the TS structure statement by statement. No third-party oracle for the new lifecycle rules (rejection order, `time-up`, bounded instants, switching); correctness there rests on one reading of the design table. `jsonschema` judges the schema from both sides (strong: ~110 conforming/breaking instances).
- Vectors are generated by the Python adapters (`generate_quiz_vectors.py`), so Python = expected by construction; the twins are judged against it.
- Boundaries covered: reach exactly at/just beyond (linear, log cap, log half spread, zero spread), time limit exactly at `limit` (accepted) and `limit + 1` (time-up) for a classification task, instants before the floor, after the clock, floor after the clock, `start - 1`, `start`, `now + 10^7`, switching with stale open runs, cap before the switch, unanswered/partly answered on every kind.
- Boundaries not covered: guesses at the reach with non-power-of-ten values (B1); the time limit for sorting and matching tasks (only `heating-systems` is cut at the limit); a skewed device clock with late delivery (B2); `null` optional members; text-level number formatting; Linux libm.
- Nit: the Python lifecycle `learner_view` orders runs with `sorted(..., key=startedAt, reverse=True)` (stable, keeps start order for equal `startedAt`), the twins reverse start order; equal `startedAt` never occurs in the vectors (clock ticks 60 s).

### Schema (Q4)

`🔣️.json` expresses §2 exactly: required/optional members, `dependencies` for `SheetAxis` (`unit/min/max` together), `seconds >= 1`, `MisplacedHint.count >= 1`, `RunView.hints` `minProperties: 1` and per-task `minItems: 1`, `Best` `points >= 0`, `x-semio-formats` on every new object definition (not on enums/oneOf aliases, as for `Scale`, `TaskKind`, `Answer`), `Rejection` enum with the three new codes, `Timestamp` integer >= 0. Rust and TypeScript twins match field for field (names, optionality, order). Notes:
- `SheetAxis` `unit/min/max` "travel together" is not expressible in the TypeScript type or the Rust struct (each `Option`), only in the schema; a document with `unit` alone decodes in Rust.
- `Profile.minProperties: 1` vs a sheet category whose shares drop every axis would be `{}` (only for an invalid quiz).
- In draft-07, a `description` next to `$ref` is ignored by validators (`ClassificationItemResult.assigned`, the command `at` members, `learner`); existing style, no effect.
- `MatchingAnswer` accepts `{kind: "matching"}` at schema level; both validators reject it on a keys-shown sheet (needs `assignments`) and accept it on a keys-hidden one. Consistent with §2's "exactly when".

### Things that will bite the client or the proctor (Q5)

1. B2 (late delivery under clock skew drops answers); also `time-up` can fire on the proctor after the deputy accepted.
2. Deputy `recorded` is rebuilt as `Object.keys(answers).length` (`🎯️targets/⚛️react/🔨️modules/🫡️deputy/🟦️.ts:175`), the proctor counts every recorded answer; with guess fields recorded per edit the 2 000 `answersPerRun` cap (default) is reached later offline than online, and the proctor rejects `answers-exhausted` at delivery what the deputy accepted. Pre-existing, made likelier by guesses.
3. The deputy rebuilds `opened` from `view.opened`; with only a `RunSummary` (no view) it has `{}` and answers `task-unopened` where the proctor knows the opening. The summary has no `opened`.
4. The proctor decodes commands (command-malformed for fractional/negative `at`, unknown challenge, `null` handled as absent), the deputy does not: online and offline verdicts differ on malformed input (P1.5, P1.6).
5. `open-task` on an opened task decides `[]` in both cores (an accepted command with no events, precedent: the identify relay), fine, but the client must treat "accepted with no events" as settled.
6. `RunState.revision` is not in `RunView`; the deputy derives it by comparing the held sheet to the material's (`deputy/🟦️.ts:163`); correct, but a revised quiz that keeps sheet equality at the held challenge hides the revision.

## Suggested actions (in order)

1. B1: tolerance in `misses` (three languages) + vectors at non-power-of-ten boundaries; document the tolerance in the design.
2. B2: decide on device-relative durations or an explicit skew offset; add a lifecycle vector `late-delivery-with-a-fast-clock` (open delivered, answer delivered after the limit by the proctor clock but within it by the device clock).
3. Add vectors: time limit at/after for sorting and matching, `null` optional members, a Linux/CI run of the shared vectors for `log10`.
4. Mention the crowd nuance and the `SheetAxis` pairing limitation in the design/README.
