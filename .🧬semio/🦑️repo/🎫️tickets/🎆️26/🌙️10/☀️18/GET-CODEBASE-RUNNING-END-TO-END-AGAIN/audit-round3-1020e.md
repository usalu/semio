# Audit round 3 (aud-round3-1020e)

Ticket: `GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Subject: ROUND 3 section of `result-integrate2.md` (applied 22:30-22:47), plus fix2 and fix3 (23:01, 23:41, not in the ledger).
Method: read-only. Ledger `🗑️generated/round3-apply/ledger.json` compared with live sha256 (last-writer attribution per file). Check modes run only without `--write`/`--apply`/`--record`/`--undo` (no tool wrote). Static greps limited to ledger-listed files. Named tests: 6 `bun test` runs in total, see section 5. Scratch output (test logs) in the session scratchpad, nothing written to the ticket except this file.

## 1. Summary

- Content of most round-3 groups is live (ledger after-hashes match, check modes report 0 pending for mem-gov, cache-gc, conc-cargo, conc-bazel, conc-nx-2d, conc-graph, graph-speed-followup, fixup-bazel-tests, fixup-loop-hunt-test, fix3).
- Round-3 result claims that FAIL today: conc-leases processes suite (15/17 pass under the harness env, 2 fail + 2 errors), conc-nx put-slots suite (9/13, 4 fail, default env), gran-keys dry run (now 1 operation + 2 missing anchors, result says 0), conc-deps check (REFUSED on the applied tree), bzl-default setup-1 (REFUSED), conc-harness check (REFUSED; apply-all hard-codes "applied").
- 20 live files differ from their last ledger after-hash with no later ledger writer (section 3). fix2 and fix3 changed 9 files that the ledger does not record.
- Tooling defects: `fixups --op nx-tests` now reports pending and would undo fix2 (do not re-run with --write); apply-all classify for conc-harness and gran-keys is hard-coded "applied"; `check` for conc-leases, conc-nx-1, conc-nx-2p, conc-harness, loop-hunt is refused by design once later sets rewrite the same files, so "check applied/0 pending" is not reachable for them.
- Real code defect (not env-only): `databasePath` in `🔒️leases/🟦️.ts` walks every ancestor and rejects symlinked ones (`Invalid lease store directory: /var` on macOS, where /var and /tmp are symlinks). Present in the merged copy too.

## 2. Per-set claims (3 each: symbol, check mode, named test)

Status: PASS / FAIL / NOT RUN (budget or heavy) / WEAK (token-only).

| Set | Symbol (live) | Check mode (live, no write) | Named test (live) |
| --- | --- | --- | --- |
| mem-gov | WEAK: `bazelResourceSetsV1` in platform suite (1 hit), not in mem-gov's own 8 files | PASS: `dry-run: 0 file(s) would be written` | NOT RUN (host-capacity 16, 17) |
| bzl-default probe+setup | PASS: `requireBazelSetupStorageV1`, `bazelCacheFlagsV1` in import of `🔧️tools/🏗️bazel/📜️script.ts`; probe suite file present | FAIL: REFUSED rc=2, setup-1 anchor count old=0 new=0 (conc-bazel at 22:40 reshaped the same import line after this group at 22:30) | NOT RUN (storage law 8, probe 6) |
| conc-leases (merged copy) | PASS: `withLeaseTaskV1` exported from order module and re-exported by leases module; `currentLeaseLineageV1`, `checkSelfDeadlockV1`, `isLeaseMediumV1` present | FAIL: REFUSED rc=1, 9 targets `drifted` (7 leases files changed by fix2/fix3, catalog.json and script.ts changed by later sets) | FAIL: processes suite 15 pass / 2 fail / 2 errors (harness env); 0 pass / 17 fail under default env (`Invalid lease store directory: /var`) |
| conc-leases fix2 | PASS: symbols above and `withNxPutSlotV1` wrapping `withLeaseTaskV1` in put-slots module | FAIL (not idempotent): dry run raises `Drift: test file changed since staging: 🟦️.ts` because fix3 changed the same test later | same as conc-leases; live processes test is now the ORIGINAL staged version (fix2 restored it), not the merged one that the result tested |
| conc-leases fix3 | PASS: `isLeaseMediumV1`, `media` schema and policy rows | PASS: `already applied` | NOT RUN (order suite 46) |
| conc-cargo (set 1 and group 2) | WEAK: `lane` token in 17 of 34 files; new `unit-cache/🧹️gc` module exists | PASS: set 1 `0 write(s), 0 error(s)`; group 2 `dry run: nothing written`, 0 pending | NOT RUN (admission, memo, fetched) |
| conc-nx 1 | PASS: `pool-order` suite files present | FAIL (by design): DRIFT on `pool-order/🟦️.ts`, changed by fixup-nx-tests (later group); `ops 14 (pending 0)` | FAIL: see put-slots (below) for the same set; pool-order not run |
| conc-nx 2-daemon | PASS: daemon files present | PASS: `ops 8 (pending 0)` | NOT RUN |
| conc-nx 2-put | PASS: `withNxPutSlotV1`, `nxPutSlotCountV1` in put-slots module | FAIL (by design + fix2): DRIFT on put-slots `🟦️.ts` (fix2) and put-slots test (fixup, then fix2); `ops 3 (pending 0)` | FAIL: put-slots suite 9 pass / 4 fail of 13 (default env, one visible fail: "a cancelled waiter leaves without a slot and without blocking the next one"); result claims 13 green. Not re-run under harness env |
| conc-deps | PASS: `🚦️concurrent/🧪️tests/🟦️.ts` exists, 0 drift against ledger, `generation` in 12 of 14 files | FAIL: REFUSED rc=2, `anchor 's03-frame-type' must occur once ... found 0`. The applied file no longer has the anchor; the check does not recognise its own applied state | NOT RUN (concurrent 14, platform 1) |
| conc-bazel | WEAK: `slot` in 14 of 16 files | PASS: `summary {superseded 1, same 8, applied 8} problems 0`, `dry run ok: 0 file(s) would be written` | NOT RUN (operations 10, storage 8, sessions 21, platform 4) |
| fixup-bazel-tests | PASS: `bazelResourceSetsV1` in platform suite | PASS: `applied` | NOT RUN |
| cache-gc | PASS: `"cache"` in the catalog `--json` operations list (live line 172) | PASS: `dry-run: 0 file(s) would be written` | NOT RUN (gc 15, ledger, providers). Law test is red per result (foreign files), not re-checked |
| conc-harness (ADOPTED) | PASS: `traceLeaseV1` in leases module (3 hits) | FAIL: REFUSED rc=2 (catalog `--json` operations list, help fixture en/root, lease `acquireResourceLease` anchor all superseded). apply-all classify is hard-coded `applied` for this group, so the status line is false | NOT RUN |
| gran-keys (ADOPTED) | WEAK: `gran` token in 4 files | FAIL: REFUSED rc=1, `set: 1 file operation(s)`, anchors `inputs-group` (in `📥️inputs/🟨️.mjs`, 0 occurrences) and `proctor` (in course-B's proctor `📋️project.json`, 0 occurrences). Result says "dry run 0 operations left". apply-all classify hard-coded `applied` | NOT RUN |
| conc-graph | PASS: `🕰️generation-protocol` test dir new; `generation` in 15 of 43 files | PASS: `ops 47 (pending 0)`, 26 new files | NOT RUN (record-header 2, closure-facts 19, environment 20, load 26, generations 23, protocol 33) |
| loop-hunt | WEAK: `graph-settle` in 1 of 8 files | FAIL (by design): `0 file(s) pending, 1 drift` (settle-repeat test is a fixup-created file that now differs from staged) | PASS: settle-repeat suite `1 pass 0 fail` (run 3) |
| fixup-loop-hunt-test | PASS: file present | PASS: `applied` | (covered by loop-hunt test) |
| graph-speed followup | WEAK: `progress.json` in 2 of 6 files | PASS: `ops 3 (pending 0)` | NOT RUN (presentation-resources 3) |
| course-B | NOT RUN: symbol token not chosen; ledger has 4 external drifted files | NOT RUN: `apply-course.py --check` runs the whole x-integrate overlay series, skipped as heavy | NOT RUN. Ledger verify_red: deploy suite (result: 3 red of 38 by design; not re-run) |
| fixups nx-tests | PASS on content (put-slots test already carries the fix) | FAIL: dry run `pending: 1 file(s)`. Re-applying would re-insert `SEMIO_LEASE_ORDER??=observe`, which fix2 removed on purpose | n/a |

## 3. Drift since apply (live sha != last ledger after-hash)

Attribution: `ledger.json` lists 405 tracked files; 385 match the after-hash of their last ledger writer; 20 do not. Ledger-explained drift (a later ledger group rewrote the file and the live file matches that group) is not listed here. Files that changed with no later ledger writer:

- conc-leases, changed by fix2 (23:01) or fix3 (23:41), not in ledger (7): `🔒️leases/🟦️.ts`, `🧭️order/🔣️policy.json`, `🧭️order/🟦️.ts`, `🧭️order/🧪️tests/📥️processes/🟦️.ts`, `🧭️order/🧪️tests/🟦️.ts`, `🧭️order/🧪️tests/🧫️fixtures/🔣️.json`, `🧭️order/🧬️schema/🔣️.json` (all under `🧰️framework/🔨️modules/🏃️process/`).
- conc-nx-2p, changed by fix2 (not in ledger) (1): put-slots `🟦️.ts`. Also put-slots test, changed by fix2 after fixup-nx-tests (1).
- gran-keys, external (5): `🏗️build/📥️inputs/🦀️resolution/🔣️.json`, `🏗️build/📥️inputs/🟨️.mjs`, `🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json`, `🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`, `🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`.
- course-B, external (4): `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`, `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🕸️site-graph/🟦️.ts`, `.../🧪️deploy/🕸️site-graph/🧬️schema/🔣️.json`, `.../📚️course/🧫️fixtures/🕸️site-graph/🔣️.json`.
- loop-hunt, external (1): `🏗️build/👁️observation/🟨️.mjs`.
- conc-graph, external (1): `🏗️build/👁️observation/🧬️schema/🔣️.json`.
- Explained by later groups (checked, not external): mem-gov `policy.json`, bzl-default (2 files), conc-harness and cache-gc catalog.json, conc-bazel and conc-graph script.ts, graph-speed-followup `script-inputs/🟨️.mjs`, conc-leases catalog and script.ts.

Ledger hygiene: `verify_red` entries are never cleared, so the ledger still lists conc-nx-1 pool-order and conc-nx-2p put-slots as red after their fixups.

## 4. Partial applies and superseded groups

1. bzl-default probe+setup: applied at 22:30, then conc-bazel (22:40) reshaped the import line the setup-1 edit anchors on. The required symbols are present; the check now refuses. Treat as applied but unverifiable by check.
2. conc-leases: merged copy applied at 22:31. fix2 (23:01) then restored the ORIGINAL processes test and fixtures (live processes test equals `conc-leases-tools/staged` byte for byte), so the result's "processes 14 green" was measured on a file that is no longer live.
3. conc-harness and gran-keys: applied at 22:10 by accident (overlay simulation against the live tree). conc-harness has NO backup of its 6 edited originals; `--undo conc-harness` only removes its new files.
4. cache-gc: the first apply failed after writing files (help regeneration); fixed and re-applied from the ledger-reconstructed state (result). Not re-verified here beyond check mode.
5. conc-deps: applied fully (14 files, 0 drift), but its check is not idempotent (section 2).
6. conc-cargo group 2: no-op, set 1 already carried the final files (check says 0 writes).

## 5. Named tests run (6 of 6 budget)

1. put-slots suite (default env): 9 pass, 4 fail of 13. Visible fail: "a cancelled waiter leaves without a slot and without blocking the next one".
2. processes suite, default env: 0 pass, 17 fail, 6 errors; 8x `Invalid lease store directory: /var` (macOS symlinked tmpdir; `databasePath` in `🔒️leases/🟦️.ts` line 151 rejects symlinked ancestors).
3. settle-repeat suite (loop-hunt): 1 pass, 0 fail.
4. processes suite, default env, run again for the fail list (discarded).
5. processes suite, default env, grep of error lines.
6. processes suite with the harness env (SEMIO_WORKSPACE_ROOT, SEMIO_REPO_ROOT, REPO_ROOT, SEMIO_TICKET_DIR, SEMIO_BUILD_MODE=prototype, SEMIO_TEST_ARTIFACT_DIR set to a scratch dir): 15 pass, 2 fail, 2 errors of 17. Failing: "a waiter does not hold the directory admission gate while it waits..." (expected "timed out", got "acquired"), and "fair readers: shared requests through withResourceLeases no longer starve an exclusive request".

Not run: put-slots under harness env, conc-deps concurrent, cache-gc gc, bzl-default storage, course-B deploy, all other suites in the table.

## 6. Merged-copy divergence from the original staged set

Originals: `conc-leases-tools/staged/` (+ `staged-optional/`) and `cache-gc-tools/staged/`. Merged copies: `round3-tools/staged/conc-leases/` and `round3-tools/staged/cache-gc/`, generated by `make-leases.py` and `make-cachegc.py`.

conc-leases (merged vs original, staged payload, 7 files differ; top-level `stage.py` also differs by 31 lines):
- `🔒️leases/🟦️.ts` (16 lines): acquire wrappers go through `traceLeaseV1` (intended, conc-harness).
- `🧭️order/🧪️tests/🟦️.ts` (8 lines): the merged copy drops the `checkSelfDeadlockV1` import and the whole fixture-driven `self-deadlock` test loop. Not documented in the result.
- `🧭️order/🧪️tests/📥️processes/🟦️.ts` (48 lines): drops the `withLeaseTaskV1` import and the "concurrent sibling tasks" test; changes "refused after the grace period" into an immediate refusal (no 900 ms check). Not documented. Superseded in live by fix2 (ORIGINAL restored).
- `🧭️order/🧪️tests/🧫️fixtures/🔣️.json` (118 lines): top-level `selfDeadlock` key removed; the `order` list gets 5 extra rows (self-deadlock-exclusive-then-shared, self-deadlock-shared-then-exclusive, reentrant-shared-is-fine, same-resource-other-directory-is-other-lease, bounded-self-conflict-is-left-to-expire).
- `⚙️operations/🔣️catalog.json` (57 lines): adds conc-harness' `concurrency` operation to `--json` and `--area` lists, adds `--workloads`, `--sessions`, `--criteria`, rewrites incremental flag descriptions (en and de).
- `⚙️operations/🧬️schema/🔣️.json` (2 lines): adds `concurrency` to the command enum.
- `📜️script.ts` (1 line): adds the `concurrency` dispatch line.
- Result: the merged copy couples conc-leases to conc-harness' catalog and removes conc-leases' own self-deadlock tests. Live now matches neither copy for the order tests and fixtures (fix3 changed them).

cache-gc (merged vs original): staged payload is byte-identical; only `patches.py` differs. The merged copy replaces the anchored `--json` catalogue pair with `_json_flag()`: appends `"cache"` to whichever operations list the `--json` flag has (idempotent) and inserts the `--area` block from the original catalogue if missing. Semantic consequence: the merged copy works for any list, but `--tasks` presence is only guaranteed when it sits inside the copied `--area` block (not verified).

## 7. Not verified

- All claims marked NOT RUN, and course-B check mode (heavy overlay series).
- Whether the fix2/fix3 edits are complete beyond symbol presence (fix2 dry run cannot prove idempotence).
- The cause of the 4 put-slots fails (not re-run under harness env).
