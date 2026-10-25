# course-c (wave 1023c) result

Status 14:50 (10/24): FULL e2e with the final tests (orch-course17e9, 13:21-14:28, load 30-40): 71 passed, 5 failed, all in the pets project; presence incl. `cooperative roster and answers` PASSED (no retries; deterministic redraw loop + law `🤝️sheet-overlap`). Pets: ladder + parachute specs timed out under load 39 (120 s / 6 s predicates; environmental), and the 3 'pet deeds ... 200% text' specs still failed: ROOT CAUSE (source reading, `🐾️pets/🟦️.tsx`): `PetsPlay` renders nothing while nobody is on stage, which is the specified behaviour (test `is there only while ... somebody is on stage`), and a text-size change makes the cast re-form, so the row of deed buttons is mounted anew and the focus is lost = tests must tolerate re-formation. Fix: the walk now marks the row, detects remounts, re-prepares (label + checks) and restarts the direction (<= 8 times), and uses sub-pixel tolerances (ratio 0.98, 0.5 px) for the 0.9967 viewport ratio. Verification: focused run focus6 (`--grep "with 200% text"`, log `🗑️generated/course-c/focus6.log`). Then warm with settle.

## Changes (e2e-orch-tools, selftest 109 assertions 0 failures)
- `orch.py`: new `QueueClock`; step timeouts now count from admission (deadline = start + timeoutSeconds + queued seconds, where queued = time between a wait line [capacity / lease position / Bazel slot] and the next non-wait output; heartbeats are neutral, a silent queue expires after `queueGraceSeconds` 60). Preflight target-verification retry loops stop on cancel.
- `plan.json`: `queueGraceSeconds`; `plan.schema.json`: thresholds declared as `properties` and `then` clauses declare their required keys (the installed Ajv 8.20 strict mode rejected the old schema, so the Ajv oracle never ran).
- `vectors.json` `queue` vectors (language-agnostic) + `selftest.py` queue tests (fake step queued past its timeout passes, working past it fails).

- Later driver changes (selftest 114 assertions, 0 failures): `forget_unfinished` (continuing a run id retries failed/cancelled/skipped steps instead of skipping them), `execute_retrying` also settles a warm step that passed but did work (`warmSettleRuns` 1: inputs edited by peers between cold and warm are rebuilt once), per-execution log parsing (appended logs no longer double-count spawns), queue clock starts at the log offset of the execution.

- 15:15 `bun repo dependencies lock --scope ♻️mit-bestand` (exit 0, bun.lock 15:16): jack-c had added the two jack-lsp workspace members to `♻️mit-bestand/package.json` (10-23 22:39) without regenerating the lock, so the frozen installer failed (`lockfile had changes, but lockfile is frozen`) for every run that syncs dependencies (focus6 died on it). jack-c's OPEN 3 also lists the other 3 lock files.

## Log
- 22:10 course11 launched without --no-verify-targets: preflight `show projects --json` queued 12 min at the capacity gate, did not honour SIGTERM (retry loop); killed my pids 84605/86886/9481, relaunched with --no-verify-targets.

- 23:34 course11 FAILED (exit 75 outdated implementation); driver now retries a step that exits 75 with that text (`execute_retrying`, `staleRetries` 2; selftest 111 assertions 0 failures); course12 launched with the OLD driver code (loaded before the edit).

- 01:14 course13 typecheck FAIL = peer syntax error in process/script.ts (see status), fixed; queue credit worked (37 min typecheck incl. 25 min waits not counted).

- 01:47 course14: proctor-build PASS (21 min), typecheck FAIL TS7016 (see status).

- 02:12 course15 check FAIL (see status). 02:25 fix written, course16 launched; proof pending.
