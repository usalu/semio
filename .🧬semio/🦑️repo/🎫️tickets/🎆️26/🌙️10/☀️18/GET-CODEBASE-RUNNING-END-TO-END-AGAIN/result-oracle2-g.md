# result-oracle2-g (executor oracle2-g, wave 1026g)

Tools `C/oracle2-g-tools/` (`src/` = the staged module, `staged/` + `stage.py check|apply|revert` = the batch, `run-laws.sh` = graph-free laws in an overlay, `gen-corpus.py` = corpus generator, `target-run.mjs` = oracle run of one target), output `C/🗑️generated/oracle2-g/`.

## Status (newest first)
- 21:05 REAL ORACLE RUNS DONE (after 19:35; leases waiting 0 throughout; no new heavy run after the gate-priority addendum 20:56). Batch re-staged (policy memory/time fix below, 14 files), laws 24/24, tsc strict on the module 0 diagnostics (module-only tsconfig in the overlay; the one finding, a too wide parameter type of `awaitLogFlushV1` in my own test, is fixed).
- 19:12 DONE (batch staged, laws 24/24). - 19:05 RESUMED after the restart (none of my pids survived). Fixed two defects found by the live corpus under host load and staged the batch (`oracle2-g-tools/staged`, 14 files incl. the README paragraph EN+DE). Real oracle runs of the huge keys are HELD until 19:35 (quiet window); the plan sizes below were measured without running any command.

## What the oracle could not do before, and what the batch changes
1. Seatbelt refuses profiles over 65,535 bytes, so a key of 12k to 26k files could not be proven at all. Now the plan is built with directory collapse (an allowed subtree plus `deny` exceptions, `alternatives`), a profile holds at most `capacity` rules (600, adapts when the kernel refuses), a larger plan is split into runs that each grant everything declared and refuse a share of the undeclared files (the union is as strict as the whole plan), and `--max-profiles` bounds the runs; a plan that needs more hands over to the mirror and says so on the progress channel. Directories stay listable through ONE rule.
2. NEW defects fixed here (both only show on a loaded or non-local-time host):
   - the unified log was read after a fixed 2.5 s pause: under load average 40 a denial was missed and the run reported `clean` (row `silent-tolerated-read-is-invisible-to-the-mirror`, expected not clean). Now `awaitLogFlushV1` reads a marker file under a profile that refuses it and polls the log until that denial shows (limit 60 s; note `log:unflushed` if it never does). The law `the unified log is awaited until a marker denial shows` covers both outcomes.
   - `log show --start "<local stamp>"` was formatted with the process time zone: under `bun test` (TZ=UTC) the window started an hour early (slow) and west of UTC it would start in the future (denials missed). Now `--last <n>s`, no time zone.
3. Aborted signal: a command started with an already aborted signal stops at once (law added).
4. Corpus: 12 policy rows (own tree, refusals on/off), split run `split-profiles-learn-undeclared` (capacity 2, 6 allowed + 2 learned files), fallback law; the plan rules are frozen from the staged module and checked against the independent Python evaluator of the rendered Seatbelt text.

## Plan sizes of the real huge keys (declared list from the sealed generation, workspace listing live; no command run)
| target | declared files | rules | allow / deny | profiles at 600 rules |
| --- | --- | --- | --- | --- |
| dsl-derive-rs:generate-schema-index | 12,246 | 9,828 | 8,279 / 1,549 | 59 |
| framework:test-source-text (the 9 framework keys are identical) | 18,965 | 5,091 | 2,478 / 2,613 | 15 |
| print:test-viz (and test-viz-fixtures) | 26,058 | 2,410 | 522 / 1,888 | 6 |
With the default `--max-profiles 4` all three hand over to the mirror (output equivalence, no read trace); `--max-profiles 8` proves print:test-viz with the kernel. Planning costs 6 to 16 s of CPU.

## Laws
Graph-free (overlay with the live framework modules, staged module copied in; `sh oracle2-g-tools/run-laws.sh`): 24/24 green in 37 s on a loaded host (23 tests of the module + the Windows plan file; the live corpus runs on the seatbelt and mirror backends of this macOS machine, landlock reports why it is unavailable). The same live corpus failed twice before the two fixes of point 2 (missed denial, ENFILE from peers is host noise). Not run: tsc strict on the module and the official `bun repo` test (quiet window); real oracle runs of the three huge keys (19:35).

## Apply
`python3 -I C/oracle2-g-tools/stage.py check` then `apply` (three-way merge against peer edits, backup under `C/🗑️generated/oracle2-g/backup`, `revert` undoes). Laws after apply: `bun repo test` of the read-oracle module (official runs only after the quiet window).

## Real oracle runs of the three huge keys (declared list = sealed-generation key + root files; `target-run.mjs`)
BEFORE (live module, print:test-viz, 26,060 literal rules in ONE profile): after 319 s `sandbox-exec` aborts with `Assertion failed: (diff <= INSTR_JUMP_NE_MAX_LENGTH), serialize.c` (exit 128); nothing is proven, and the old code did not even recognise the abort as a profile refusal (the staged `seatbeltRefusalV1` does).

AFTER (staged module through its CLI):
| key | backend | result |
| --- | --- | --- |
| print:test-viz (`--max-profiles 8`) | seatbelt, kernel-enforced, 6 profiles of at most 600 rules | runs: 8 rounds, 21 to 363 s each (host load), 30 undeclared files learned, not converged: the task reads every workspace `package.json` (24 of the 30; plus root `Cargo.toml` of the framework, `🖼️assets/README.md`, `workspace-taxonomy-locator/⚡️cache/🟦️.ts`). Each round names only the first 2 to 13 because the command stops at the first refusal. |
| framework:test-source-text (default `--max-profiles 4`) | plan needs 15 profiles: the CLI says so and continues with the mirror (output equivalence) | 2 rounds, 1 s each: the mirror needs the root `📋️project.json` (without it `repo` finds no repository: `--extra`) and then names `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`; output still differs |
| dsl-derive-rs:generate-schema-index (default) | plan needs 58 profiles: mirror | 1 round, 3 s: `Cargo package has no workspace` (the workspace `Cargo.toml` is missing from the key; the kernel run of print learned `🧰️framework/Cargo.toml` too) |
The mirror proves non-equivalence only; it cannot name a missing file (`learned` is 0 or 1 where the kernel names 3 to 13 per round). Plan cost after the memory fix: 6 to 8 s and 170 to 190 MB per plan (before: 20 s and 340 MB, which tripped the 500 MB guard of the oracle in round 5).

## Policy memory/time fix (found by the real run)
`coverV1` kept one entry object per directory entry for each of the 6 cover weights and listed every directory 6 times: 26k-file keys exceeded the oracle's 500 MB guard after a few rounds. Now `layoutV1` lists the declared directories once (names plus summed weights), `coverLayoutV1` solves it per weight; rules are identical (corpus rows frozen from the old staged module still pass; counts for the three keys unchanged).

## Recipe for gate-g after apply
1. `python3 -I C/oracle2-g-tools/stage.py check` then `apply`.
2. `bun repo test` for the module `🏗️build/📥️inputs/🔭️read-oracle` (all 24 tests; the live corpus needs `sandbox-exec` and ~40 s).
3. Oracle leg of a huge key (one at a time, right after a gate verdict, lease waiting below 4): `ORACLE2_CLI=<live read-oracle>/🚪️cli/🟦️.ts bun C/oracle2-g-tools/target-run.mjs <tag> <closure.jsonl> <task id> <declared list> --max-profiles 8 --rounds N [--extra <root files list>]`.
