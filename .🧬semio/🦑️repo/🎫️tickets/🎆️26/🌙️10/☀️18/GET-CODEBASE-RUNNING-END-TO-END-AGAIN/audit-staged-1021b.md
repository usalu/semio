# 🔎️ Staged Wave 1021b Audit (read-only, auditor audit-staged-1021b)

📅️ Snapshot: 2026-10-21, about 16:30 local. Live storage file last changed 16:18. `graph-parallel-tools/staged/round2.json` is 16:16, newer than `result-graph-parallel.md` (15:15), which does not cover it.

📜️ Rules checked (AGENTS.md): emoji-first docstrings; no comments inside definitions; no fallback, compat, legacy or deprecation; no runtime external dependency; no default language; EN and DE; custody, lease and quota; paths at most 256 chars; tool-independent interfaces; no `[DEBUG]`.

🧪️ Method: scripted checks over every staged file (docstring leads, in-body comments, imports, hard-coded paths, `?? "en"`, fallback words, raw spawns). In-memory simulation of every anchored edit against the live files, with nothing written. Manual reads of the custody, lock, quota and apply paths. No builds, no `bun`, no cargo, no bazel, no git-modifying commands. Abbreviations: `GP` = `graph-parallel-tools/staged/new2/` (plus `staged/ops.json`, `staged/round2.json`), `GC` = `cache-gc-tools/`, `PM` = `pm-seam-tools/staged3/`, `BC` = `backend-contract-tools/staged/`. All paths are relative to this ticket folder unless absolute.

## 🚦️ Verdict

NOT READY. Three BLOCKER items below. Graph-parallel round 2 is the largest risk: it moves the custody model its own result document forbids, and nothing records a test run for it.

Severity: **BLOCKER** (must fix before apply), **HIGH** (AGENTS MUST violated or custody and apply risk), **MEDIUM**, **LOW**.

## 🗂️ Per-set findings

### graph-parallel (ops.json 33 edits and 20 new files; round2.json 38 edits, 8 replaced, 8 new, 1 deleted)

Simulation: all 33 ops anchors are unique in the live files; round 2 applies after ops with 0 failures. Longest target 155 characters (215 bytes absolute); ticket copies at most 195 bytes. Pass: no in-body comments, EN and DE messages complete, no default language, runtime imports only `node:`/`bun:`, Ajv and python3 only in tests.

| Location | Rule | Sev | Finding and one-line fix |
|---|---|---|---|
| round2 `storage/🟨️.mjs` (`withBuildGraphWorkerWriterV1`), `GP/leaves/leaves.mjs:77-100`, `GP/project-farm/service.mjs:38`, observation round2 `writeSealedBlocksV1` | custody | BLOCKER | Workers write optional leaves and sealed blocks straight into the shared graph directory under an 8 MB per-job grant only; the quota and ledger see them after the parent adopts (`GP/project-farm/glue.mjs:83`). `result-graph-parallel.md` §2 and §6 F1 say this needs storage-owner sign-off, which is not on record. Fix: revert to staged files and parent adoption, or get the sign-off and reserve quota before each write. |
| `GP/leaves/leaves.mjs:59`; `GP/project-farm/glue.mjs:44-46,75-76`; observation round2 `persistSealedBlocksV1` (`presence.has`) | race, dangling reference | BLOCKER | Two workers can write the same content-addressed leaf (closure inventories and blocks are shared between projects). `adopt` accepts the first and calls `rmSync` on the second if quota refuses it, while `presence` and `knownBlocks` still say the file exists. A record can then reference a deleted block. No test covers duplicate names. Fix: dedupe accepted names per writer before reserving; an existing name is "already present" and is never removed; add a two-worker same-name law test. |
| `GP/project-farm/glue.mjs:48-51` vs `:83` | quota | HIGH | The byte grant is released in `finally` after `infer`, before `replay` calls `adopt` for the same bytes, so quota is overcommitted in between. Fix: release the grant after adoption. |
| `GP/farm/engine.mjs:145,153-154,182,260`; `GP/project-farm/glue.mjs:41`; `GP/pool/pool.mjs:19,30`; `farm/schema.json` "fallback" text (EN/DE) | fallback | HIGH | Every crash, timeout, failure and refusal silently becomes an in-process inference, counted in `fallbacks` and reported as "inferred in this process because a worker was unavailable". Fix: a failed job rethrows the project's own error; the farm succeeds or fails per build. |
| `GP/pool/pool.mjs:21` | message | MEDIUM | `say("fallback", { count: 0, … })` prints "0 projects were inferred in this process". Fix: pass the number actually inferred in the parent. |
| `GP/project-farm/glue.mjs:35-36`; `GP/project-farm/service.mjs:21`; `GP/farm/engine.mjs:253`; `GP/farm/worker.mjs:18` | unbounded memory | HIGH | `exportSourceAnalysisRecordsV1` returns every parser record the parent holds. It is serialized whole into a session file per open, deserialized whole by each worker, and kept in `seedBytes`. The 256 MB cap applies only after deserialization. Fix: select the seed by the worker's project set and cap it before serializing. |
| `GP/project-farm/glue.mjs:42,75`; `GP/project-farm/service.mjs:9` vs `GP/farm/engine.mjs:44-47` | unbounded memory | HIGH | Record bytes are capped per worker (256 MB each, up to 12 workers, about 3 GB) in decoder tables kept for the whole phase. The planner reserves 2 GB for the parent and never counts this. Fix: one parent-wide byte cap read from the policy and counted in `planGraphFarmV1`. |
| `GP/farm/worker.mjs:6` | swallowed error | MEDIUM | `send` swallows the `process.send` exception, so a result that cannot be serialized leaves its job waiting for `jobTimeoutMilliseconds` (30 min). Fix: send a `failure` message from the catch. |
| `GP/pool/pool.mjs:27` | lease | MEDIUM | `leaseSafetyMilliseconds` (1 h) releases the host capacity grant and the pool lease while a long build still runs. The farm then degrades to serial silently and the host budget is oversubscribed. Fix: renew while jobs are in flight, or remove the timer. |
| `GP/leaves/leaves.mjs:16,27`; `GP/farm/engine.mjs:253`; `GP/project-farm/glue.mjs:44` | blocking | LOW | `Atomics.wait` blocks the event loop up to 200 ms per flush; synchronous file I/O per reply. Fix: async flush and adoption. |
| `GP/module-farm/glue.mjs:10` vs `GP/farm/engine.mjs:43` | threshold | LOW | Auto mode passes files/250 as the project count, so auto mode engages only from 6,000 files. The 3,534-file measurement was forced mode. Fix: state the threshold or measure in auto mode. |
| `GP/farm/tests/gen_fixture.py:2` (applied by ops) | docstring | HIGH | Module docstring "Writes the farm law fixture…" has no emoji lead. Fix: prefix an emoji. |
| `GP/project-farm/service.mjs:45` | docs | LOW | The docstring says a worker keeps no private files, which is the opposite of the staged design in the result doc. Fix: update with round 2 or the result doc. |
| round2.json as a whole | verification | HIGH | round2 (16:16) postdates the result doc (15:15), changes custody, the storage file and two new modules, and has no recorded test or overlay run. Fix: run the laws on a round-2 overlay before GO. |

### cache-gc staged3, staged4, apply3.py, apply4.py, recreate.py

| Location | Rule | Sev | Finding and one-line fix |
|---|---|---|---|
| `patches.py:370` (GRAPH_STORAGE4 row 1) vs graph-parallel round2 storage publish | cross-set anchor | HIGH | Simulated: after graph-parallel's storage edits the anchor count is 0, so `apply4` refuses on drift. Refusal is safe, but neither set applies as staged. Fix: re-anchor the member-index hook in the parent branch of the rewritten publish function. |
| `GC/staged3/gc-launch.ts:38,50` | swallowed error | MEDIUM | `child.on("error",()=>{})`, `stdio:"ignore"` and a `catch{}` around the launch. A collector that never starts leaves no trace, but the 5-minute stamp is written anyway. Fix: write the failure to the collector status and return it to the writer. |
| `patches.py:199` (GRAPH_STORAGE3) | fire-and-forget | MEDIUM | `void import(…).then(…).catch(()=>{})` spawns the collector from the writer path with no error surfaced. Fix: await the launch decision and report its failure. |
| `GC/staged3/gc-runner.ts:63` | fail-open | MEDIUM | An `lstat` failure on a subdirectory is swallowed and its mtime ignored, so the unit can look older than it is. Fix: treat a failed stat as "now" and keep the unit. |
| `GC/staged3/gc-runner.ts:213` | swallowed error | MEDIUM | `chmod(...).catch(()=>{})` before `unlink`. On Windows a read-only file fails the unlink. Fix: surface the chmod error; chmod only on EPERM. |
| `GC/staged4/members-engine.mjs:114-119` (called from the storage hook) | blocking, custody | MEDIUM | Every publication reads the whole leaf synchronously (up to `maximumFileBytes`) and hashes it under the writer's custody. Fix: hash the buffer the writer already produced, or defer hashing to the collector. |
| `GC/staged4/members-engine.mjs` `memberStampV1` and `proveGroupV1` | proof | MEDIUM | A leaf is vouched by kind, size and `ino:size:mtimeNs` alone, so an in-place edit that keeps size and mtime (tar restore, `touch -r`) is trusted with its indexed hash and links. Fix: include ctime in the stamp, or hash-sample on verify. |
| `GC/apply.py:79-82` | drift | HIGH | When a file has no recorded base the drift check is skipped and the file is overwritten; `--tolerate-drift` widens this. Fix: refuse unrecorded bases unless `--record` ran in this round. |
| `GC/apply.py:95-110` | restore | HIGH | `--restore` writes the backup over current files without a hash check, clobbering peers' edits made after apply. Fix: restore only when the current hash equals the manifest's post-write hash. |
| `GC/apply3.py:59-62` | all-or-nothing | HIGH | Apply writes first, then `recreate.run` removes the graph image. A failure after the write leaves a half-applied state. Fix: run the recreate preflight and take the lock before any write; make the removal the last step with a rollback manifest. |
| `GC/recreate.py:48-59,90` | TOCTOU | HIGH | The busy and open-file checks are advisory: a `repo` process can start between the check and `shutil.rmtree` (line 90). Fix: take the collector single-flight lease (or an exclusive lock file) before the check and hold it through the removal. |
| `GC/staged3/gc-test.ts:102`; `GC/staged4/members-test.ts:33`; `members-resolve-test.ts`; `members-child-test.ts:22` | cross-platform | MEDIUM | Tests hard-code `true`, `python3` and `node`. They fail on native Windows and in a Bun-only devcontainer. Fix: `process.execPath` plus one shared Python resolver. |
| `GC/staged3/ledger-policy.json:704` (`cargo-legacy`) | legacy naming | LOW | Rename the id. |
| `GC/paths.py:15` | zero-touch | LOW | Ticket tooling falls back to a hard-coded `/Users/ueli/Documents/semio` root. |
| `GC/apply3.py:2`, `apply4.py:2`, `recreate.py:2` | docstring | LOW | Ticket tooling docstrings have no emoji lead. |

### pm-seam staged3 (plan.json, 14 files, 3 rewrites)

| Location | Rule | Sev | Finding and one-line fix |
|---|---|---|---|
| `PM/plan.json:7` (`requires`) | ordering | HIGH | `requires` says "rounds 1 and 2". Round-3 anchors `round3-launch-manager` and `round3-launch-keys` exist only in `GC/staged3/gc-launch.ts`, which is not live. The `round3-script-*` anchors match 0 times in the live `📜️script.ts`. The result doc validated the set on `ckpt-hot-cache`, not on the live tree. Fix: list cache-gc staged3 and the round 1 and 2 order in `requires`; `apply.py check` must pass before any write. |
| `PM/apply.py` busy guard | busy | LOW | Refuses only on `dependencies ensure|sync|lock` and `show projects`; graph-farm and `cache gc` runs are not covered. Fix: add `script.ts repo` and `cache gc`. |
| PM round-3 new strings (24 edits) | rules | — | No docstring, comment, fallback or hard-path hits. |

### backend-contract READY3 (manifest.json: 27 files; all 27 sha256 match the staged files)

| Location | Rule | Sev | Finding and one-line fix |
|---|---|---|---|
| `BC/semio_store.ts:147-161` | custody, lease | BLOCKER | The lock heartbeat swallows `utimes` errors (147), so a live holder can look stale past `lockStaleMilliseconds` and be taken over (161). Its `finally { rm(lock) }` (148) then deletes the successor's lock, so two writers can run. Fix: abort the body when a touch fails, and delete the lock only when it still carries our random owner token. Check whether the live store already has this code. |
| `BC/semio_scheduler.ts:23` | fallback, cache keys | HIGH | `gitV1(...).then(…, () => undefined)` turns any git failure (timeout, corrupt index) into a filesystem walk that ignores `.gitignore`. The inventory and the cache keys change silently. Fix: walk only when git is absent (ENOENT); surface other failures. |
| `BC/taskmodel.ts:65,68` (also `conformance.ts:105`, `semio.ts:57,59`, `semio_store.ts:104`, `stub.ts:28`) | tool-independent interface | HIGH | `new Bun.Glob(…)` in backend-neutral code. Fix: route through the in-house glob of the runtime-seam set, behind the tool interface. |
| `BC/semio_scheduler.ts:4` | cross-set | MEDIUM | Imports `gitV1` from `🌿️git`, which only the tool-iface set creates, and it is not live yet. Fix: order tool-iface first in `requires`. |
| `BC/semio_store.ts:58,68` | swallowed error | MEDIUM | `catch{}` on touch and on listing drops entries from usage accounting, so quota is under-counted. Fix: count and report unreadable entries. |
| `BC/conformance_test.ts:26`; `BC/semio_test.ts:27` (`testPythonV1()??"python3"`) | cross-platform | LOW | Fallback to `python3` in tests. Fix: the resolver tries `python` on Windows. |

Passed: EN and DE through `BackendTextV1`; the git raw spawn from the previous audit is gone; the Python oracle is stdlib only.

### storage-seam §8 (live)

| Location | Rule | Sev | Finding and one-line fix |
|---|---|---|---|
| `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:30` (identity-only `recheck`) | custody | MEDIUM | Between boundaries the store no longer re-measures capacity or re-reads the lease journal. The result claims that every borrower operation verifies lease custody; I could not confirm that from the reads. Fix: add a law test that another process's lease change is caught at the next boundary, or keep a cheap journal stat in `recheck`. |
| `L/🏗️build/🕸️graph/🗂️generation-files/🟦️.ts:9-11` | race | LOW | The single in-flight recheck and shared refusals are correct. Write and remove are windowed like reads, as before. |

### audit-fix (live)

| Location | Sev | Finding |
|---|---|---|
| `L/🔍️discovery/✅️owned-file-approvals/🟦️.ts:322-331` | LOW | Causes are kept. The new messages are English-only, consistent with the module. |
| `L/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:26,329-343` | PASS | The memo is created inside `projectWithDefaults` (line 329), so it is per project and has no cross-project key collision. |
| `L/🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs:39` | PASS | An unlinked main or gate database now fails loudly. |
| `L/🏗️build/📥️inputs/💾️storage/🟨️.mjs:84,217` | PASS | Duplicate docstring emojis fixed. |
| `L/🏗️build/📥️inputs/💾️storage/🟨️.mjs:106` | LOW | Production keeps the `observeOptionalPublication` hook. Confirm no fixture counter is reachable outside tests. |

## 🔀 Cross-set conflicts

| Sets | Where | Sev | Result |
|---|---|---|---|
| graph-parallel round2 vs cache-gc staged4 | storage publish function | HIGH | Anchor count 0 after graph-parallel; apply4 refuses (safe, but not applicable as staged). |
| graph-parallel worker leaves vs cache-gc member index | parent-only hook | MEDIUM | Worker-written leaves are never indexed, so the proof treats them as unproven. Fix: call `recordMemberLeafV1` from the parent's adoption. |
| pm-seam round3 vs cache-gc staged3 | `gc-launch.ts` anchors | HIGH | Undocumented apply dependency (see PM table). |
| pm-seam round3 vs cache-gc build-script (`📜️script.ts`) | BUILD_SCRIPT and BUILD_SCRIPT4 | LOW | No anchor overlap found; neither applies until pm rounds 1 and 2 land. |
| backend READY3 vs tool-iface | `semio_scheduler.ts:4` | MEDIUM | Compile-time dependency on the `🌿️git` adapter. |
| graph-parallel vs storage-seam §8 and audit-fix (live) | storage, project-assembly | none | All ops and round2 anchors still unique and applicable (simulated). |
| graph-parallel vs cache-gc staged3 GRAPH_STORAGE3 | storage | none | Simulated: unique. |

## 🧱️ Blockers before apply

1. graph-parallel round 2 custody: revert worker writes to staged adoption, or obtain storage-owner sign-off (result §6 F1).
2. graph-parallel adoption: dedupe leaf names and never remove an accepted name (`GP/leaves/leaves.mjs:59`).
3. backend READY3 lock: a failed heartbeat aborts the body, and release deletes only our own owner token (`BC/semio_store.ts:147-161`).

## 📝️ Non-blocking

- HIGH items above still need fixes before apply: graph-parallel fallback, grant release order, record and seed caps; cache-gc apply restore, drift, and recreate TOCTOU; backend `Bun.Glob` and git fallback; pm-seam ordering.
- Ticket tooling docstrings and paths (cache-gc `apply3.py`, `apply4.py`, `recreate.py`, `paths.py`) are LOW.

## ✅ Checked, no finding

- No `[DEBUG]` leftovers in any set. No comments inside definitions in GP, GC or BC.
- Path lengths: GP targets at most 155 characters (215 bytes absolute); GP ticket copies at most 195 bytes; BC destinations at most 172 bytes.
- BC manifest hashes match all 27 staged files.
- The audit-fix edits keep their named anchors; no conflict with the graph-parallel anchors.

## 🔁️ Caveats

- No test or build run. All findings come from text reads, the in-memory anchor simulation, and scripted checks.
- Round 2 of graph-parallel has no recorded verification, and `result-graph-parallel.md` predates it.
- `overlap-check.py` (run at 15:15) was not re-run.
- One recursive grep was started by mistake under the framework tree and moved to the background. It was read-only and may still be running.
