# 📓️ W1-B Report — `⏪️time-travel` Module

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W1-B, 2026-09-30. Status: **DONE, VERIFIED** (every command below was run and its output seen).

## 1. What was built

New framework module `🧰️framework/🔨️modules/⏪️time-travel` (crate `semio-framework-time-travel`, lib `semio_framework_time_travel`, nx project `@semio-tech/framework-time-travel-rs`). It is the pure, target-neutral time-travel session reducer of design §4.

| File | Role |
|---|---|
| `⏪️time-travel/🦀️.rs` | Rust owner. Contains `TimeTravelBase`, `TimeTravelTarget`, `TimeTravelStage` (with `ToValue`/`FromValue`, bare camelCase string), `TimeTravelDraft`, `TimeTravelPending`, `TimeTravelProgress`, `TimeTravelSession`, `TimeTravelChoice`, `TimeTravelEvent`, `TimeTravelEventKey`, `TimeTravelEffect`, `TimeTravelEffectKind`, `TimeTravelRefusal` (`timeTravel.illegal`, `.stale`, `.blocked`, `.empty`), `TIME_TRAVEL_FROZEN_CODE` and `TimeTravelLabel` (18 EN/DE rows). The reducer is `TimeTravelSession::apply(&mut self, event) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal>`; it mutates in place and a refusal leaves the session untouched. Helpers: `new`, `accepted_draft`, `inputs`, `start_of`, `unchanged`, `finalize_refusal`, `invariant_violation`. |
| `⏪️time-travel/🟦️.ts` | TS twin: `applyTimeTravel` (pure; never mutates its input), the same helpers, JSON converters and `TIME_TRAVEL_LABELS`. `InputReplacement`, `SupersededInput`, `ReplayReport` and `replayReportBlocksFinalize` are imported from the replication twin (`../📡️replication/🟦️.ts`), so each type has one source. |
| `🧬️schema/🔣️.json` | JSON Schema of record (draft-07, `$id` `…/framework/time-travel/schema.json`). `ReplayReport` is a `$ref` to replication's `…/replication/replay-report.json#/definitions/Report`. Replacement payloads use the protocol `ToValue` shape (a byte array). |
| `🧫️fixtures/🧫️lifecycle-law/🔣️.json` | Language-agnostic law fixture: 20 named contexts, the 17 canonical events, a **119-row matrix** (every one of the 6 × 17 (stage, event) pairs, split into guard branches from a closed set of 18 guards), 30 cases with full expected sessions and effects, 8 scenarios with a final session, 10 invariants (each with a counterexample), and 18 labels. |
| `🧪️tests/🔬️unit/🦀️.rs` | Rust unit tests (10), mounted via `#[cfg(test)] #[path]`. They check every matrix row, stale generations in every context, every case, every scenario, the invariants, the labels and the `Stage` value round trip. |
| `🧪️tests/🧪️conformance/🟦️.ts` | bun:test (15 tests) with three oracles. **ajv** (strict) validates the fixture and every reducer output, and rejects hostile mutations. **xstate** builds a machine from the matrix plus an independent context model; its guards and assignments are written separately from the reducer. **fast-check** runs random sequences in which the reducer and xstate must agree on acceptance, stage, full model, effect kinds and `startReplay` order; a second run with a pinned seed asserts that all 34 legal law rows are visited. |
| `📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts,package.json,🦀️.rs}` | Scaffolding copied from `⏯️tool-run`. nx targets: `test`, `test-quick`, `test-long`, `test-exhaustive`, `check`. |

Registrations:
- Root `Cargo.toml`: `members` line and `[workspace.dependencies]` entry, both placed after `⏯️tool-run`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `members-of-modules` gets `"⏪️time-travel"`.
- No root `package.json` change, because there is no TS package (tool-run pattern).

Dependencies:
- `semio-framework-replication` (`protocol`).
- `semio-framework-value-derive`, used through `#[value(crate = "::protocol::value")]`.
- Dev-dependency: `serde_json`.
- **No `os-kernel` or `ui`.** The crate compiles while other crates are broken.

Input script kept: `🧪️w1-b-generate-lifecycle-law.py` in the ticket root. It is an independent Python transcription of the law and regenerates the fixture: `python3 <ticket>/🧪️w1-b-generate-lifecycle-law.py`.

## 2. Approved interpretations of §4 (coordinator approved all ten, 2026-09-30)

1. `Begin { target: TimeTravelTarget { mutation, position }, original }`. `position` is the op's applied position at the session base. `accepted` is kept in history order `(position, mutation id by UTF-8 bytes)`. `StartReplay.from` is the earliest accepted target (a `MutationId`).
2. `BaseMoved { base, positions: Vec<TimeTravelTarget> }` carries the re-resolved positions of the session targets. It is legal in every stage, including `Inactive`, where it only records the base; `Begin` therefore carries no base. A `BaseMoved` whose base equals the session base returns `timeTravel.stale`.
3. Only `Begin`, `BaseMoved` and `Exit` carry no generation. Every other event carries `generation`, and a mismatch returns `timeTravel.stale` before any stage check. `generation` never resets (`wrapping_add`). It increments on every transition that starts or discards driver work: `Begin`, `StartReplay`, `Commit*` and `Close`.
4. `Accept` behaves as follows:
   - an unchanged draft (equal to its start value, i.e. the accepted draft or else the original) behaves like `Discard`;
   - a draft equal to the original removes that target's accepted draft;
   - if `accepted` is empty afterwards, the session goes to `Reviewing` with no report and no replay starts.
5. `Discard` or an unchanged `Accept` returning to `Reviewing` restarts the replay when the kept report was invalidated, i.e. `BaseMoved` during `Editing` drops the report.
6. `FinalizeFaulted{code}` goes to `Replaying` (a `StartReplay` that revalidates on the current base) and sets `fault = code`.
7. `Exit`:
   - illegal from `Inactive`;
   - `timeTravel.blocked` in `Finalizing` (the commit is in flight);
   - emits `[CancelReplay, Close]` from `Replaying`;
   - emits `[Close]` from every other stage.
8. `RequestFinalize` (only in `Reviewing`) checks in this order: `timeTravel.empty`, then `timeTravel.blocked` (no report, or a report with Error or Fatal), then goes to `Choosing` with `OpenFinalizePrompt`.
9. `Begin` in `Editing` with a changed pending draft returns `timeTravel.blocked`. With an unchanged draft it switches target and inherits the return stage.
10. Labels:
    - The crate has no `Locale` type. `TimeTravelLabel::{key,en,de,localized}` exists, and hosts call `label.localized(LocalizedLabel::native)`.
    - Every label has both locales and there is no default.
    - Beyond stages and refusals, the labels include `frozen`, `choiceOverwrite(+Description)`, `choiceAlternative(+Description)`, `alternativeNameDefault`, `noChanges` and `replayProgressValueText` (`{done}`/`{total}`; format it with `tool_run_format`).

The fault lifecycle follows from the above: `fault` is set by `ReplayFaulted` or `FinalizeFaulted`. It is cleared by any other transition that starts a replay, and by `Close`.

**Driver contract for W2-A (plugin runtime host):**
- Resolve `position` at the session base on `Begin`.
- Send `BaseMoved` with the re-resolved positions of every session target on every store generation change, in every stage.
- Carry the session `generation` on every other event.
- Treat `Reviewing` without a report as "no changes; the preview is the committed head" (label `noChanges`).
- `StartReplay` is latest-wins: it replaces any running replay.
- Return `timeTravel.frozen` for artifact-lane commands while the stage is not `Inactive`.

## 3. Verification (exact commands, exact results)

All commands were run from `/Users/ueli/Documents/semio` unless noted. The build gate (`pgrep -x rustc`) read 0 before every cargo call.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-time-travel --message-format=short` | Finished, 0 warnings |
| `cargo check -p semio-framework-time-travel --target wasm32-wasip2 --message-format=short` | Finished, 0 warnings |
| `cargo check -p semio-framework-time-travel --target wasm32-unknown-unknown --message-format=short` | Finished, 0 warnings |
| `cargo clippy -p semio-framework-time-travel --all-targets --message-format=short` | 0 warnings in this crate (warnings shown come from `value-derive` and `replication` and pre-date this WP) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-w1-b cargo test -p semio-framework-time-travel --message-format=short` | `test result: ok. 10 passed; 0 failed` |
| `bun test "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts"` | `15 pass, 0 fail, 2110 expect() calls` (three consecutive runs, all green) |
| `cd …/⏪️time-travel/📦️packages/🦀️rust && bun ./📜️script.ts check` | exit 0 (native and wasip2) |
| `cd …/📦️packages/🦀️rust && CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1-b bun ./📜️script.ts test` | bun 15 pass; nextest (profile fundamental) `10 tests run: 10 passed` |
| `bun nx show project @semio-tech/framework-time-travel-rs --json` | targets `build` (inferred), `check`, `test`, `test-exhaustive`, `test-long`, `test-quick` |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🔨️modules/⏪️time-travel` | `clean=true errors=0 warnings=0` |
| `node_modules/.bin/tsc -p <scratch strict tsconfig over the twin and the conformance test>` | 0 errors in time-travel. 1 error elsewhere: `🛂️manifest/🟦️.ts(117,3)` has no exported member `DialogChoice` (W1-E, in flight) |
| `bun ./📜️script.ts verify dependencies literal-external` | exit 1, red repo-wide from before this WP (literal-external 258; oracle conflicts `rust:image` and `rust:serde_json` in other crates). This module appears nowhere in the report |
| `bun ./📜️script.ts verify docstrings emoji-first` | exit 1, red repo-wide from before this WP (5107 findings). None in this module; docstring emojis are unique per file (checked by script) |

Oracle strength was checked by deliberate mutation, then restored:
- A TS `resume` bug made 6 of 15 tests fail, including the xstate and fast-check tests.
- A Rust `ReplayCancelled` bug that kept progress made `every_matrix_row_holds` fail (invariant `progress-only-while-replaying`).
- The fast-check coverage gap was found and fixed. fast-check's default `size: "small"` visited only 22 to 26 of 34 legal rows. The test now uses `size: "max"`, biased picks and a pinned-seed coverage test that visits 34 of 34.

## 4. Open items and notes

- **Launch rows** were not generated: per the plan, the coordinator regenerates them centrally. The nx project exists.
- **Reusable-module rule** (two production consumers): the only consumer planned so far is W2-A (`🔌️plugin` `TimeTravelLedger`). Hosts see the session only through W2-A's patch.
- **Bun pitfall**: `bun test 🧰️framework/…/🟦️.ts` (a relative path without `./`) is treated as a filter. Bun then scans the whole repo and segfaults (bun 1.3.14, RSS 2 GB). The same happens with tool-run's suite. Pass an absolute path or a `./`-prefixed one; `📜️script.ts` already passes an absolute path.
- The fixture's `ReplayReport` validation depends on replication's `⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json` `$id`. If W1-A renames that schema, update the `$ref` in `🧬️schema/🔣️.json` and the import in the conformance test.
- Scratch left in `🗑️generated/w1-b/`: `taxonomy-time-travel.log`, `dependencies.log`, `docstrings.log`. Private uplift target dir: `.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-w1-b`.

## 5. Follow-up (audit `📓️audit-modules-ui.md`: M-3, N-2, N-5, N-7), 2026-09-30

Status: **DONE, VERIFIED**. Only files of this module, the fixture generator in the ticket root, and this report changed. Rust sources are unchanged since the last Rust run.

### 5.1 M-3: "no changes" versus "needs replay"

Before this fix, the module docs said "`Reviewing` without a report means no changes", which was wrong. After a cancelled or faulted replay, drafts exist but there is no report. W2-A built its panel on that rule and showed "No changes".

- **The four cases are now explicit.** `TimeTravelReview { NoChanges, NeedsReplay, Blocked, Ready }` (Rust, with `ToValue`/`FromValue` as a bare camelCase string; TS `TIME_TRAVEL_REVIEWS`). `TimeTravelSession::review() -> Option<TimeTravelReview>` (TS `timeTravelReview`) returns `Some` only in `Reviewing`:
  - `NoChanges` iff `accepted` is empty; the preview is the committed head;
  - `NeedsReplay` iff drafts exist and there is no report;
  - otherwise `Blocked` or `Ready` from `ReplayReport::blocks_finalize`.
  - `TimeTravelReview::label()` (TS `timeTravelReviewLabel`) maps to the labels `noChanges`, `needsReplay`, `reportBlocking`, `readyToFinalize`.
- **New event `Rerun { generation }`** (TS `{ type: "rerun", generation }`; key `rerun`, placed after `replayFaulted`). It is legal in `Reviewing` when drafts exist and there is either no report or a fault. It then behaves like any replay start: stage `Replaying`, effect `StartReplay { drafts, from }`, generation + 1, fault cleared.
  - Refusals: `timeTravel.empty` when there are no drafts; `timeTravel.illegal` when a current report exists without a fault, or in any other stage.
  - `TimeTravelSession::rerun_refusal()` (TS `timeTravelRerunRefusal`) lets hosts enable the button. It is the single source the reducer itself uses.
- **`ReplayCancelled`** now records `fault = "timeTravel.cancelled"` (`TIME_TRAVEL_CANCELLED_CODE`). `TimeTravelLabel::for_fault(code)` (TS `timeTravelFaultLabel`) maps it to `TimeTravelLabel::ReplayCancelled`.
- **New labels** (EN/DE): `needsReplay`, `reportBlocking`, `readyToFinalize`, `replayCancelled`, `actionRerun` ("Replay again" / "Erneut anwenden"). There are now 23 rows.
- **Driver contract** is updated in the Rust docs, the TS docs and the schema description.

### 5.2 N-2: state hygiene and input validation

- **(a) Stale fault.** `settle` with no drafts left now also clears `fault`. So reverting the only draft after a faulted replay leaves no stale fault. New invariant `fault-needs-drafts`: a fault exists only while drafts are accepted.
- **(b), (c) Validation.** `TIME_TRAVEL_TEXT_MAX_BYTES = 256` and two validators:
  - `is_time_travel_fault_code` (TS `isTimeTravelFaultCode`): non-empty, no Unicode `White_Space`, at most 256 UTF-8 bytes.
  - `is_time_travel_alternative_name` (TS `isTimeTravelAlternativeName`): at least one character outside `White_Space`, at most 256 UTF-8 bytes, committed verbatim with no trimming.
  - `TimeTravelEvent::is_well_formed()` (TS `timeTravelEventWellFormed`) runs right after the generation fence. A malformed `ReplayFaulted`, `FinalizeFaulted` or `Choose{Alternative}` returns `timeTravel.illegal` in every stage.
  - The twins use the same definition of whitespace: Rust `char::is_whitespace` and TS `\p{White_Space}`. The oracle checks it against an explicit 25-code-point list.
- **Schema.**
  - New `FaultCode` and `AlternativeName` definitions: `minLength`/`maxLength` 256 and an explicit `White_Space` character class, written as regex escapes. They are used by the session's `fault`, the strict `TimeTravelEvent` and `commitAlternative.name`.
  - New lax `TimeTravelEventInput`: what a host may dispatch, including malformed text. Fixture cases, scenario steps and matrix `input` rows use it.
  - Test law: an event that fails the strict schema is never well formed. The schema cannot express the byte ceiling (it bounds code points), so a 129 × "ä" name is schema-valid but not well formed; the test pins that.

### 5.3 N-5 and N-7

- **N-5:** docstring emojis are unique per file in all module files, checked by `🗑️generated/w1-b/docstring-emoji.py`. The `🕰️` duplicate is gone, as are four others introduced by the follow-up.
- **N-7:** the TS twin and the conformance test import `@semio-tech/framework-replication` by package name, like `🛠️tool-machine`. The replay-report schema stays a relative import, because the package exports no schema.

### 5.4 Fixture (`🧪️w1-b-generate-lifecycle-law.py` regenerated it)

| Section | Now |
|---|---|
| Contexts | 22 (added `reviewing.cancelled`, `reviewing.faultAfterReport`) |
| Canonical events | 18 |
| Matrix | 130 rows (6 × 18 pairs), 35 legal |
| Guards | 24 (added `needsReplay`, `reportCurrent`, `validCode`, `invalidCode`, `validName`, `invalidName`); rows may carry their own `input` |
| Cases | 45 (15 new: cancel marker, rerun after cancel, rerun after a finalize fault with a report, rerun while replaying, stale-fault clearing, empty/whitespace/over-long codes, codes at exactly 256 bytes, malformed input outside its stage, empty/whitespace/over-long names, names at exactly 256 bytes, padded names) |
| Scenarios | 10 (added "a cancelled replay is rerun and finalized" and "a faulted replay is rerun once, then a current report makes rerun illegal") |
| Invariants | 11 (added `fault-needs-drafts`) |
| Labels | 23 |
| `reviews` | the review of every context |
| New codes and limits | `reviewKinds`, `cancelledCode`, `limits.textMaxBytes` |

### 5.5 Oracle changes

- The xstate model gains `rerun`, the cancel marker, the fault clearing, and independent `needsReplay`, `validCode` and `validName` guards.
- fast-check now draws valid and malformed codes and names, and starts each sequence either fresh or from any fixture context (1 : 2 weighting).
- **Budget fix.** At load ~45 the old 3000-run properties took ~10 s each, over the 15 s `fundamental` budget of `runTestBudgeted`; one router run was killed and exited 1. The run count now follows `SEMIO_TEST_LEVEL`: 400 at `fundamental`, 3000 at `quick`, `long` and `exhaustive`. The pinned seed still visits **all 35 legal rows at 400 runs**, thanks to the context starts.

### 5.6 Verification (exact results)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-time-travel` | Finished, 0 warnings |
| `cargo check -p semio-framework-time-travel --target wasm32-wasip2` | Finished, 0 warnings |
| `cargo clippy -p semio-framework-time-travel --all-targets` | 0 warnings in this crate |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1-b cargo test -p semio-framework-time-travel` | `12 passed; 0 failed` |
| `bun test <absolute path of 🧪️tests/🧪️conformance/🟦️.ts>` | `18 pass, 0 fail, 2736 expect() calls` (4.4 to 13 s depending on load) |
| same with `SEMIO_TEST_LEVEL=quick` | `18 pass, 0 fail` (26 to 34 s) |
| `bun ./📜️script.ts test` (package router), twice | exit 0 both times: bun 18 pass; nextest `12 tests run: 12 passed` |
| `bun ./📜️script.ts test quick` | exit 0: bun 18 pass; nextest 12 passed |
| strict `tsc` over the twin and the conformance test | 0 errors |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🔨️modules/⏪️time-travel` | `clean=true errors=0 warnings=0` |

Build-gate note: the first follow-up `cargo test` and the `test quick` router run started while `pgrep -x rustc` read 14 and 17, above the rule-12 threshold of 8. Both finished normally. All other cargo calls ran below 8.

### 5.7 For W2-A, and still open

- **W2-A must adopt** the names above (sent to main):
  - replace `no_changes: stage == Reviewing && report.is_none()` with `session.review()` and `review.label()`;
  - add a reserved rerun action that dispatches `Rerun { generation }`, labelled `ActionRerun` and enabled when `rerun_refusal().is_none()`;
  - show `TimeTravelLabel::for_fault` for `timeTravel.cancelled`;
  - pre-validate the finalize dialog's name with `is_time_travel_alternative_name`.

  Nothing in W2-A breaks at compile time: it matches `TimeTravelEffect` exhaustively (unchanged) and only constructs events. Its fault codes are valid.
- **Audit items not in this follow-up's scope**, left open for the coordinator:
  - N-1: no `CloseFinalizePrompt` effect; hosts infer the prompt from the stage.
  - N-3: no `TargetsLost` event.
  - N-4: the Finalizing exit contract note for W2-A.
  - N-6: the fuzzer compares effect kinds and `startReplay` order, not the `showPreview` or `commit*` payloads.
  - N-8: the choice and frozen labels are duplicated in `🛂️manifest` and `🔌️plugin`.
