# Result stage-fix (executor `stage-fix`, 2026-10-20)

`C` = this ticket folder, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `M` = `C/staged-merge-tools`. Only staged copies and apply tooling under `C/*-tools/` were edited, plus the four live (non-library, non-frozen) edits of section 4 (items 5 and 6 of the second coordinator message). Nothing was applied, committed, stashed or reverted.

## 0. Per-set verdict

Order planned by `M/plan.ts` (replay in a sandbox, `replay ok`, 680 changed files, 656 changed sources and JSON parsed with 0 syntax errors, 2,341 relative imports resolved, 0 regressions, 0 undeclared files):
`lease-fair -> tax-stage -> tsc-stage -> walkperf -> bzl-stage -> cargo-pressure -> fixups6 -> env-pass2 -> own-crash2 -> exports-stage`.

| set | verdict | what is left |
|---|---|---|
| lease-fair | READY | CPU-time bounds of the fairness test are load-sensitive: run it on a quiet machine. |
| tax-stage | READY | 101 ops (67 anchored patch, 30 write, 4 delete), no `package.json` op, no `📜️script.ts` renamed or deleted. 15 scripts cannot be lifted without changing their argv (section 3) and stay in place as router-law backlog. |
| tsc-stage | READY after tax-stage | base sha fence on the 23 entries; restage after any peer edit of those files (section 5). |
| walkperf | READY after tsc-stage | `generation receipt` suite now gates (`run-tests.sh real gate`). |
| bzl-stage | READY (mechanics), provisional | no `result-bzl-stage.md`: drive skips it unless `--include-provisional`. |
| cargo-pressure | READY, owner sign-off pending | `recheck` is required now (fence 1 closed). The 15 s throttle (fences 2 and 3) is a design decision, only the wording of the feature file was made consistent. |
| fixups6 | READY after tax-stage and tsc-stage, fragile | the graph test is a hot file (changed 5 times in 80 minutes); every change needs `bun M/plan.ts restage fixups6`. |
| env-pass2 | READY (mechanics), provisional | its own executor is still adding patches (43 files at last count); the duplicate of tsc-stage patch 1 was removed from `T.json`. |
| own-crash | APPLIED (planner: already-applied) | normalisation follow-up is `own-crash2`. |
| own-crash2 | READY | item 4 of the second message (section 6). |
| exports-stage | READY after the freeze, deps-owner gated | 634 entries / 506 apply; item 3 fixed (section 7); `confirm` = `--freeze-lifted`; then `bun repo dependencies lock` and `sync`. |
| bzl-real | NOT READY (harness, not a repo change) | absolute path and stale docstring fixed (section 8); evidence rows of its report are not corrected. |
| walkperf2, law-sweep2, inputs-closure | NOT READY | no staged content / not started. |

## 1. Safety: dry-run contract

Contract of every apply tool: `check` and `apply --dry-run` write nothing; `--ignore-sync` only skips the sync guard; `--rebase` exists only where a set supports it (tsc-stage, walkperf, cargo-pressure, bzl-stage) and is never passed by the planner on its own; any other flag exits 64; exit 2 = a `dependencies sync|lock` or `deps4-*.sh` loop runs, 3 = drift or refusal.

- env-pass2: `apply.py` rewritten (mode first, flags anywhere, atomic writes, idempotent count-checked anchors, sync guard, `--dry-run` honoured). Its executor edited the file afterwards (span edits); my guard and flag handling were re-applied on top and span edits that no longer match their offset fall back to the same count-checked anchors (exact for the uniform call rewrites this generator emits).
- fixups6 and own-crash: `apply.py` rewritten, root found by the `bunfig.toml` + `bun.lock` walk (no `APPLY_ROOT`, no live default), `--dry-run` honoured, sync guard added.
- lease-fair, cargo-pressure, tsc-stage, walkperf, bzl-stage, tax-stage: sync guard now also matches `dependencies lock` and `deps4-loop2.sh`; unknown flags refused.
- exports-stage `apply.ts`: `--dry-run` (plan only, even with `--freeze-lifted`), unknown flags exit 64, exit 2 only for the sync guard (was 2 for everything), root found by the marker walk (was a hard-coded depth, wrong inside a sandbox), `--ignore-sync` honoured in every mode.
- Proof: `C/stage-fix-tools/purity.py M/plan.json` runs `check`, `apply --dry-run` and `apply --dry-run --ignore-sync` of all 13 sets on the live tree and compares a digest of 1,097 target paths before and after: unchanged (`targets unchanged: True`). Without `--ignore-sync` all sets answer rc 2 while the sync runs.
- The planner now also refuses a set whose `--dry-run` changes its sandbox (`its --dry-run wrote files`) and `drive` stops with code 7 when a dry-run changes the live fingerprint of the planned targets.

## 2. Planner (`M/plan.ts`, `schema.json`, `sets.json`)

- `after` edges are hard: a set whose predecessor is excluded is excluded (`predecessor-excluded`), a cycle in the registry is an error, pair runs apply each set after its own predecessors (so three-way chains such as tax-stage -> tsc-stage -> fixups6 are judged correctly), `drive --only/--from` refuses a step whose predecessor is neither selected earlier nor applied, `--from <unknown id>` is refused.
- No automatic `--rebase`: `attempt()` runs once. `plan --rebase-set a,b` is the only way, recorded in `plan.json` (`rebase`). `sets.json` has no `rebase` field any more.
- A pair that commutes by digest but conflicts under `diff3` is downgraded to `conflict`.
- New: `materialize <id> --out dir` (replays the predecessors in a sandbox and writes every file they changed, plus `.materialized.json` with deletions), `restage <id>` (materialize, then the set's own `restage` argv from the registry), `confirm` (extra flag for the real run only, `--freeze-lifted` for exports-stage), adapters `edits-json` (env-pass2) and `batch-json` (exports-stage), a syntax pass over every changed file (`replay.syntax`).
- The dry-run rehearsal of `drive` also replays the plan order in a sandbox.
- Registry: reordered to the audit order, `own-crash`, `own-crash2`, `fixups6` registered, `walkperf` verify split into a gating and an informational item.
- Test: `M/plan.test.ts` + `fixtures/merge.json` extended (predecessor exclusion, leaky dry-run, explicit rebase only, `--from` unknown, hard `--only`): 5 pass (`bun test ./C/staged-merge-tools/plan.test.ts`).
- Apply window protocol (sync stopped, freeze lifted): `sh C/stage-fix-tools/restage-all.sh` (about 5 min), `bun M/plan.ts plan` (about 9 min), `sh M/apply-all.sh --dry-run --ignore-sync`, then `sh M/apply-all.sh --yes [--include-provisional]`. `check` says whether the plan is current.

## 3. tax-stage

- Ruling applied: no `📜️script.ts` is renamed or deleted. `plans/children.ts`, `plans/modules.ts`, `t-children.txt` removed; the 18 test-spawned child programs and the 2 bazel module scripts go through the same lift as the other scripts (`📜️script.ts` stays the entry and router, the body moves to a `🟦️.ts` owner module). Of the 39 candidates 24 lift; 15 do not and stay untouched (`C/stage-fix-tools/lift-report.ts` lists them): 8 programs read `process.argv` and `defaultCommand` only applies to an empty argv (a lift would turn their first positional argument into a command name; spawn sites would need a rewrite), 5 use module-level program state, 2 are the bazel module scripts (no command to route). Router-law backlog is therefore 3 (not liftable by grammar) + 15 = 18 library scripts, not 3.
- Compat re-exports: the digest and lock modules no longer re-export `sourceClosureDigest` / `selectedCargoLockDigest` (their one importer, `⚡️caching/🧪️tests/🎚️build-mode`, imports from the `🟨️.mjs` owner). `prepareBazel` and the `📦️artifacts` barrel were pre-existing exports of untouched files (the barrel is the public `@semio-tech/repo-lib/artifacts` entry); tax-stage no longer touches them.
- package.json: no op. The ghost fix skips `package.json`; the dead `./script` export is removed by exports-stage E0857.
- Hot files: edit-only and small rewrites are derived as unique-anchor line patches (`lineAnchoredEdits`, replay verified at staging), so peer edits elsewhere in the graph test or its fixture no longer conflict.

## 4. Other per-set fixes

- lease-fair: three `[DEBUG]` logs removed (staged and tool sources); sync guard.
- cargo-pressure: `recheck` required in `CargoSourceAdmissionPortV1`, calls `port.recheck(...)`, five test ports in the existing admission suite get `recheck`, anchor-local idempotency, feature file wording, sync guard. Not run: the admission suites (they need the mirror of the original executor); syntax and imports are covered by the planner replay.
- tsc-stage: base sha fence, no silent retarget (`restage` picks and logs the target: script, `📦️child/🟦️.ts`, sibling or `🔬️script-commands`), anchor-local idempotency, `restage TREE` with `diff3` rederivation and a hand-resolution slot (`resolved/`).
- walkperf: `bases-from TREE` (lineage on tsc-stage), anchor-local idempotency, gating `generation receipt` suite, `--rebase` only by flag.
- bzl-stage: zero-fuzz: the fuzzy `patch -s` is gone; ops are exact anchors derived from base -> staged, applied idempotently behind the sha fence; `restage TREE` replays them on the predecessor result.
- fixups6: `restage.sh TREE` splits and checks the predecessor result (`SPLIT_TREE`), 4 new peer test titles added to `groups.json`, oracle lossless (871 lines, 107 tests, 0 problems).

## 5. Verification run

- `bun test ./C/staged-merge-tools/plan.test.ts`: 5 pass.
- `python3 C/stage-fix-tools/purity.py`: unchanged targets (section 1).
- Whole-plan sandbox replay: section 0. `sh M/apply-all.sh --dry-run --ignore-sync --include-provisional` (live dry-run of every set, sandbox replay of the 10-step order, verify commands listed): rc 0, `verification failures: 0`; digest of the 1,097 target paths identical before and after; `plan.ts check`: `inventory current, live current`. Without `--include-provisional` bzl-stage and env-pass2 are skipped (no result report). Sets that need a predecessor print `defer` in the live dry-run and pass in the sandbox replay.
- `bun test ./C/bzl-real-tools/staged/store-receipt.test.ts`: 6 pass. Admission law on a physical overlay with the own-crash2 patch: 21 pass (negative control on the unpatched reader: 2 fail).
- Not run: any cargo build or test, `bun repo run`, the library suites of the sets, anything that needs the sync to be stopped.

## 6. own-crash2 (follow-up of own-crash, item 4)

`C/own-crash2-tools` (anchored edits, composes with tsc-stage): `readTaxonomyUnchecked` normalises an absent `implementationAdmission` to `null` (schema contract: absent = none), `packageImplementationAdmitted` takes `PackageImplementationAdmission | null` again, the `omitted-admission-never-admits` fixture row is dropped (an absent key cannot reach the consumer), the test asserts strict `null` and that the reader normalised every rule the document leaves without one.

## 7. exports-stage (item 3)

12 keys ending in `/.json` now use the directory stem (`./artifacts/storage/schema.json`), 10 repo-lib glue wrappers moved to `…/🔣️json/🟦️.ts`; `build-candidates.ts` derives the same on a re-stage. `dotenv` devDependency entries (repo-lib, root) dropped. Counts recomputed (634 entries, 506 apply, 408 export adds, 20 removes, 81 dependency entries, 379 glue creates) and corrected in `result-exports-stage.md`. Not verified (as before): that the seven runtime-looking third-party devDependencies are only imported by tests and scripts.

## 8. bzl-real (item 2)

`stage.ts` finds the root by the marker walk (no `/Users/ueli`), the staged `store-receipt.mjs` reads its schema through a relative `new URL(…, import.meta.url)`, the adapter docstring now states that no `.bazelignore` is written. tax-stage no longer renames the patched file, so the order problem is gone. Open: two evidence rows of `result-bzl-real.md` (Q3, F1) do not add up and need a re-run.

## 9. Live edits outside the staged sets

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/🦀️.rs`: `from_returned` and `begin_returned_close` stay `pub` (the durable-group testing helpers of another crate call them, so a revert breaks that crate); they are `#[cfg(feature = "durable-group-testing")]` (the crate's test-only feature, enabled by the wire crate's own `durable-group-testing`), now `#[doc(hidden)]` and documented. Not compiled.
- `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts`: the exporter no longer passes `--features typegen` (the actor package defines no features; the `[[test]] typegen` target needs none). Not run (no cargo).
- Incident: while building a symlink-overlay for a test I deleted `L/🔍️discovery/🔐️taxonomy-authority/🟦️.ts` through a symlinked directory. It was clean in `git status` at 03:12 and is restored byte-identical from `HEAD` (`git hash-object` = index blob `78be7a7f…`, `git status` clean). No other file was touched by that command.

## 10. Files

Edited: `C/staged-merge-tools/{plan.ts,plan.test.ts,schema.json,sets.json,fixtures/merge.json,plan.json}`; `C/lease-fair-tools/{apply.py,fairness-test.ts,patch_progress.py,staged/**}`; `C/tax-stage-tools/{apply.py,apply.sh,stage.ts,vfs.ts,plans/*.ts,staged/**}` (removed `plans/children.ts`, `plans/modules.ts`, `t-children.txt`); `C/tsc-stage-tools/{stage.py,patches.json,staged/**}`; `C/walkperf-tools/{walkperf.py,run-tests.sh,bases.json}`; `C/bzl-stage-tools/{stage.py,bases.json,staged/**}`; `C/cargo-pressure-tools/{apply.py,bases.json,staged/*.txt}`; `C/fixups6-tools/{apply.py,split.ts,oracle.ts,restage.sh,groups.json,manifest.tsv,staged/**}`; `C/env-pass2-tools/{apply.py,staged/patches/T.json}`; `C/own-crash-tools/apply.py`; `C/exports-stage-tools/{apply.ts,assemble.ts,build-candidates.ts,batch.json}`; `C/bzl-real-tools/{stage.ts,staged/**}`; `C/result-exports-stage.md` (corrections appended). New: `C/own-crash2-tools/`, `C/stage-fix-tools/{restage-all.sh,lift-report.ts,purity.py}`.
Live (non-library): the two files of section 9.
