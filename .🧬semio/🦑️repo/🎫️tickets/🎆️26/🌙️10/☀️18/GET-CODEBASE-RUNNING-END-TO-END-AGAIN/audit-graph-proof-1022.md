# Audit graph-proof 1022

Auditor `graph-proof`. Scope: prove today's live fixes end to end (dependency receipt, graph reuse, reader leases, stale reasons, lock order). No repo file was edited except the temporary trailing newline on one file, which was removed again (see step 2).

Run logs and captured output live outside the repo in the scratchpad: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad/` (`run1.json`, `run1.err`, `run1.timing`, `run2.json`, `run2.err`, `run2.timing`, `leases-pre.txt`, `leases-mid-run.txt`, `leases-post.txt`).

## Step 1: lease wait before the runs

- `bun repo leases status` at 20:07 showed one exclusive `repository-graph-facts:/Users/ueli/Documents/semio:shard:7` holder, process 8247 (`bun repo run ... exec`, session a1op65zrdbr, 105 s old at the first check).
- It ended on its own after about 4 minutes (gone at 20:13:47). No exclusive graph-facts holder was present before my runs started.

## Step 2: target runs

Edit: the file `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🏭️generator-outputs/🟨️.mjs` had ended in `}\n`. A `\n` was appended (6044 to 6045 bytes) at 20:14:17, the same second both runs started. Its sha256 was `33ce24a4...b029a0` before. The file is untracked, so `git diff` cannot verify it; the sha was used instead.

Restore: the last byte was removed again. Result: 6044 bytes, sha256 `33ce24a402abb44b1ecec83e850e4a450cbca3058939983b17cfa36f63b029a0`, byte-identical to the backup taken before the edit.

Both runs were started detached with `nice -n 10 bun repo show targets --json`, with stdout to `run*.json` and stderr to `run*.err`.

| Run | Start | End | Wall | Exit | Result |
|---|---|---|---|---|---|
| run1 | 20:14:17 | 21:13:03 | 3526 s | 0 | 6569 targets, valid JSON |
| run2 | 20:14:18 | 21:39:48 | 5130 s | 1 | failed, partial output (7684 bytes) |

- Both runs queued at machine admission at start. run2 logged 143 capacity-wait lines (9 ahead, spare memory negative).
- Both runs took the exclusive `repository-graph-facts:/Users/ueli/Documents/semio:shard:7` lease. run1 also held `build-graph-state:selected:7` exclusively (holder pid 43649, 1701 s at 21:09). run2 queued behind run1 and then acquired the same lease at its own runtime (`run2.err` lines 1169 to 1173).
- Inference: run1 logged `Inferring projects: 0 of 3750` and ran to `3750 of 3750`. run2 logged `0 of 3749` and reached `3748 of 3749`. In both cases every project was re-inferred. No graph-record reuse occurred.
- The only "reused" lines are TypeScript program file counts (`[input-precision] ... 10517 reused`, 5 lines in run1, 6 in run2). These are not graph records.
- Stale lines (EN, as printed):
  - Dependency generation receipt, run1 line 200: `The dependency generation receipt of scope . is stale: An installer input changed its content: w:...🩹️patches/nx@23.2.0.patch` (class `input-changed`, detail `bytes`). run1 then published generation epoch 9 for `.`, `mit-bestand`, `s`, `hub`, `teaching` and `semio-tech` (lines 12926 to 12931). run2 also logged the same receipt line.
  - Store dependency sync, run1: `reason: obsolete-generations`.
  - Graph, run1: `@leutwiler/realparts-of-powers-z-n is stale: the source analysis of <file> changed` (about 7 files under `framework/.../leases/...`, `schema/validator`, `process/routing`), `inference protocol of target-policy changed from ...target-policy/2 to .../3`, `declared data file ...caching/policy.json changed`, plus `62340 more stale inputs are not listed`.
  - Graph, run2: same shape, now including `process/leases/order/registry`, plus `70824 more stale inputs are not listed`.
  - The edited `generator-outputs/🟨️.mjs` is not named in either log (0 hits for `generator-outputs`).
- Stale-impl-guard re-exec: zero occurrences of `format-stamp`, `reexec` or `re-exec` in either log. No re-exec occurred.
- run2 failure (exit 1, `OwnedCommandFailure: repository:command failed (1)`): `Uncaptured implementation schema read refused: 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧬️schema/📏️physical-budget.json (its owning module is captured but the schema is not)`. That schema file was created by a peer at 21:07:24, during run2. This is a live-code blocker for `show targets`, not a lease problem.

## Step 3: lock order after the runs

`bun repo leases status` (`leases-post.txt`) reports `34 held` before, `53` to `59` during, `41` after, with `0 deadlock cycles` and `10 recent order violations` in every snapshot. The 10 ORDER rows are a rolling window. Every one is requested by `repo:compiler-writer`, mostly cargo or compiler-capacity leases while the image lease is held shared. None comes from `show targets`. The violations pre-date these runs and were not caused by them.

## PASS/FAIL per claim

| Claim | Verdict | Evidence |
|---|---|---|
| Dependency receipt survives tool-source edit (`.mjs` edit) | PASS (narrow, confounded) | The edited file was never named in a stale line in either run. The only dependency-receipt reason was the installer input `nx@23.2.0.patch` (bytes). Caveat: the receipt was invalid for the patch anyway, so the edit's effect was not isolated. |
| Graph records reused | FAIL | Both runs re-inferred every project (`0 of 3750`, `0 of 3749`). The only "reused" lines are TypeScript program file counts. The graph was stale (62340 and 70824 stale inputs). |
| No exclusive lease for readers | FAIL | Both `show targets` runs took exclusive `repository-graph-facts:shard:7`, run1 also `build-graph-state:selected:7`. The graph was stale, so the rebuild path was taken. A warm-graph reader was not measured in this window. |
| Stale reasons named (EN/DE) | PASS (EN), DE not exercised | Each stale line names the reason and path. The DE texts exist in the schemas: the dependency-receipt schema has `Eine Installer-Eingabe hat ihren Inhalt geändert`, and the graph schema has `reasonAnalysis: "die Quellanalyse von {path} ..."`. The runs printed EN only. |
| Stale-impl-guard re-exec | None observed | Zero hits for `format-stamp`, `reexec` or `re-exec` in both logs. |
| Lock order clean | FAIL | 10 ORDER rows before, during and after. All are requested by `repo:compiler-writer`. The violations pre-date these runs and are not from `show targets`. |

## Open issues

1. `show targets --json` run2 fails with `Uncaptured implementation schema read refused` for the peer-created `physical-budget.json` (21:07:24). Run1 succeeded before that file existed in its window, so the failure is timing-dependent and needs a live fix or a capture-rule change.
2. Readers rebuild the graph exclusively whenever it is stale, so the reader-lease claim can only pass on a warm graph. Re-measure after the peers' live edits settle.
3. Two concurrent runs serialise on `repository-graph-facts` and wait on memory admission. Two runs cost about 59 and 86 minutes of wall time here.
4. The ORDER rows are pre-existing `compiler-writer` violations, not from these runs.

No child processes of mine remain (checked for pids 27408, 27412, 27414, 27415, 27419, 27420, 43649, 45683). No background waits are pending.
