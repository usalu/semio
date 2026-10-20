# result-x-ingress: why every `repo run` pays minutes and takes an exclusive dependencies lease

`T` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `C` = ticket 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN. Tools: `T/x-ingress-tools/`. Status: ANALYSIS DONE, ROOT CAUSE FIXED LIVE (one token), FIX STAGED AND TESTED IN A MIRROR (sections 4-5), LIVE TIMING OF THE WARM PATH PENDING (blocked at 15:55 by nlink=2 on package.json/bun.lock, section 3).

## 1. Code path (file:line)

Every `repo` command except `tools`, `reset`, `cache` (and `cargo`, which returns earlier) runs the build-environment bootstrap, `--dry-run` included:

1. `L/🏗️build/📜️script.ts:132` calls `ensureBuildEnvironmentV1(root,invocation,phaseControl)`.
2. `L/🏗️build/🚀️bootstrap/🟦️.ts:47` `ensureBuildEnvironmentV1`. Purpose: prove that the pinned parser material (typescript, @iarna/toml) in the dependencies partition is the generation the lock demands, before build inference may import anything.
3. `…🟦️.ts:84-85` `inspect(false)` = `storage(false, inspectIn(false,false))` = `withRepositoryCacheStoreV1(workspace,"dependencies",…)`: a SHARED image lease on `native-fixed-image:.cache/b/d`.
4. `…🟦️.ts:61-63` `inspectIn`: first `generation.verifyJavascriptDependencyGenerationReceiptV1` (the "fast" receipt, `L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts:97`). On `status!=="match"` it falls to `currentOwner()` (`…🟦️.ts:63`) = `withCurrentBunWorkspaceOwnershipV1(all)` (`L/🗂️workspaces/🟦️bun/🟦️.ts`) plus `readJavascriptDependencySourceOwnerScopeReadV1` (`L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:383`). These are exactly the progress lines "Inspecting source metadata / Listing source entries / Rechecking source evidence / Reading source text / Checking source membership" (the `build:parser-source:*` resources are published by `parserSourceProgressV1`, `…🟦️.ts:38-45`) and "Checking declared package providers: 1104 edges" (resource `javascript-source-provider:<n>`, `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:354`, the provider-resolution walk). They are the walk of the WHOLE workspace (`{kind:"all"}`: 75k directories, 312 manifests, 1,104 provider edges, 375,552 operations), not of the selected task closure.
5. `…🟦️.ts:80`: `if(!publishReceipt&&walk){candidate=…;return "receipt-required";}`. A shared pass that had to walk NEVER decides readiness; it always returns `receipt-required` so that the exclusive pass publishes the generation receipt.
6. `…🟦️.ts:86-88` then runs `inspect(true)` = `withJavascriptDependencyStorageV1` (`L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:73`): `withRepositoryCacheStoreQueuedMaintenanceV1` with `maximumQueueWaitMilliseconds` 21,600,000 (6 h, `SourceOwnerStoreSyncProtocol` in `…/🧬️schema/📥️install.json:4700`) = the EXCLUSIVE `native-fixed-image:.cache/b/d` lease, plus an exclusive `javascript-dependency-bindings:<ws>` gate. This is the "exclusive dependencies lease" in the log (`dry-test.log` line 120).
7. Inside the exclusive hold (`…🟦️.ts:79`): `publishJavascriptDependencyGenerationReceiptV1` (`…/🧾️generation/💾️storage/🟦️.ts:125`). When it does not return `published`/`present` (`deferred`, `stale`, `failed`: `adoptedCandidateV1`, `…🟦️.ts:34`) the code discards the shared walk and runs `currentOwner()` AGAIN under the exclusive hold (`…🟦️.ts:79`), then a THIRD walk as confirmation (`…🟦️.ts:80`, `walk&&!cached?await currentOwner()`), writes the parser receipt, and tries to publish once more at the end (`…🟦️.ts:82`, last statement). Net: 3 full workspace walks per run, 2 of them while holding the exclusive image lease.

### Why a warm tree still pays (the receipt exists in code, never in the store)

`.cache/b/d/m/generation/` holds four 0-byte `*.frontier.ndjson` files (05:44, 06:00, 06:37, 14:51), no `*.receipt.json`, no `*.execution.ndjson`, no `*.closure.ndjson`. No receipt has ever been sealed on this machine. Reasons seen:
- `[DATA] Store dependency generation receipt deferred … "reason":"implementation-unsettled" "path":"…/💾️storage/🍎️image/📡️roster/🟦️.ts"` (our own dry run, 15:28): capture refuses to seal while any file of the sealed runtime closure (176 files, 130 of them `🔨️modules/🏃️process`, i.e. other fleets' code) has mtime/ctime younger than `process start - 5 s` (`captureGenerationReceiptV1`, `…/🧾️generation/🟦️.ts` rule `implementation-unsettled`). `x-ingress-tools/closure-churn.py`: 11 of 176 closure files changed within 1 h, 30 within 6 h, 94 within 24 h, 8 of them at 15:12:32 (the peer's ingress-perf apply) and the bootstrap itself at 15:20:22. With the fleet editing process modules continuously, no process ever starts "after the last edit".
- earlier: `provider-observation-unsettled` at the end of every `dependencies sync` (`C/🗑️generated/chain*/…`): the sync's own link writes are younger than 5 s.
- ROOT CAUSE OF THE NEVER-SEALED RECEIPT (proven, `x-ingress-tools/repro-write-limit.ts`): `writeJavascriptDependencyGenerationReceiptV1` (`L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts:144`) wrote evidence in 262,144-byte chunks, but `withArtifactRelativeDirectoryV1` accepts at most `RelativeCreateProtocol.maximumWriteBytes` = 65,536 per write (`🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json:463`; check in `…/🟦️.ts`: `length>p.maximumWriteBytes` throws "Bounded closed relative write bytes required"). The first evidence file (`*.frontier.ndjson`, 20 MB) is created, its first write throws, the error is swallowed as `[DATA] … receipt failed`, and a 0-byte file stays behind. That is every 0-byte frontier file in `generation/` (05:44, 06:00, 06:37, 14:51, 15:31, 15:33, 15:34: each EXCLUSIVE holder that got past the deferral rules produced one). The peer's glue test never saw it because it writes the fixture evidence with `writeFileSync`, not through the real publisher. This is the only reason the fast tier never hit.
- the 5-second settle rules (`implementation-unsettled`, `provider-observation-unsettled`) additionally defer publication whenever a closure file or a provider entry is younger than the walk start (above). They are correct but, with the fleet editing closure files, they defer most attempts.
Consequence: the "fast" tier (`verify`, ~3-7 s loaded) can never hit, every run walks, and every run requests the exclusive lease.

### Why the exclusive request stalls everybody (not only the requester)

The lease queue is FIFO. An exclusive request queued behind a shared holder blocks every later SHARED request: our own measurement run waits as a SHARED requester, `position 7 of 7, behind the request of process 93898 (exclusive …)` (`T/🗑️generated/x-ingress/bazel-dry-1.log`). The original observation (`dry-test.log`): exclusive request behind a shared holder (pid 55035, `repo run @semio-tech/framework-os-mcp-rs:build`), which itself was CPU-bound in its own shared walk for 22 minutes. With 6 course-target processes started in parallel by the fleet, each does walk (shared, minutes at load 20) → exclusive (queued behind the others) → 2 more walks under the exclusive hold; the queue drains at one process per 2-3 walks.

## 2. Receipts and peer sets: what exists, what covers this path

| Set | Where | Applied live? | Covers `repo run` ingress? |
|---|---|---|---|
| walkperf / walkperf2 (generation receipt: `🧾️generation/{🟦️.ts,💾️storage/🟦️.ts,🧬️schema,🧪️tests}`, bootstrap hooks) | `C/walkperf-tools/`, `C/result-walkperf.md` | YES (core, glue, schema, tests present; bootstrap calls `verifyJavascriptDependencyGenerationReceiptV1`) | This IS the mechanism. It is sound and fast on a hit (6-12 s measured by the peer at load 45-107, 1-2 s estimated idle) but the publish path defeats it (above). |
| ingress-perf (A journal revision, B roster, B2 recheck window, C fetched-registry receipt, D scheduler receipt, E custody session, F lease-image sweep stamp) | `C/ingress-perf-tools/`, `C/result-ingress-perf.md` | YES since 15:12:32 (`📡️roster`, `📜️journal/🔖️revision`, opening `🟦️.ts` have that mtime); its own report still says "STAGED, NOT APPLIED" (stale) | NO for the walk. It targets the cargo ingress (`bun repo cargo`): storage reservation recheck, hdiutil roster, `cargo fetch`, scheduler. Its journal-revision cache and roster single-flight reduce `store.recheck()` cost, which this path also calls, but nothing in it touches `ensureBuildEnvironmentV1`, the provider walk or the exclusive publish. Its applied files are in the sealed closure, so applying it invalidated (and deferred) the receipt once more. |
| show-perf | `C/result-show-perf.md` | not applied | NO. It is `bun repo show projects` inference (GC bound under memory starvation). |
| graph-perf | `C/result-graph-perf.md` | not applied | NO. Inference graph persistence phases of `show projects`. |
| gperf / walkperf (first report) | `C/result-gperf.md` | n/a | gperf is the graph loader; not this path. |

Receipt types in the store relevant here: generation receipt (`generation/<scopeKey>.<sha>.receipt.json` + `.frontier/.execution/.closure.ndjson`) and the parser receipt (`bootstrap/<sha256(key,generation,bindingDigest)>.json`, 548 bytes; four exist, newest 14:27, i.e. the parser receipt is already current and is NOT the blocker).

## 3. Measurements

Harness `T/x-ingress-tools/timed.ts` (prefixes every line with elapsed ms, prints a per-phase table). Real runs were possible only while the fleet allowed it: my first run (`bazel-dry-1.log`, started 15:31:13 behind 6 other course-target processes) was killed at 648 s on the coordinator's request (storage blocker), a second attempt (`bazel-dry-2.log`, 15:53) died in 2 min at boot with `Bounded regular bootstrap input required` (`🏗️build/🚀️bootstrap/🟦️.ts:57`, `package.json` and `bun.lock` had nlink=2 from x-inputs' hardlink mirror; coordinator is removing it). Numbers below are from `bazel-dry-1.log` (timestamps) and the coordinator's `dry-test.log` / `run-test-1.log` (line counts only, no timestamps). Load average 19-25 on 10 cores throughout.

`bun repo --tool=bazel run @teaching/architecture-course:test --dry-run`, receipt absent (state of the machine, no receipt ever sealed):

| Phase (run 1, elapsed) | Duration | Note |
|---|---|---|
| wait SHARED `native-fixed-image:.cache/b/d` | 0 -> 183 s (181.7 s) | position 7 of 7 behind an EXCLUSIVE holder (course:build) |
| walk 1, shared: metadata 5.0 s, listing entries 14.1 s, evidence 2.0 s, declared providers (1,104 edges) 11.7 s, evidence/text/evidence 1-2 s | 198 -> 248 s (about 50 s, 375k operations) | what the progress lines are |
| `receipt-required`: second wait SHARED | 251 -> 554 s (303 s) | queued behind the earlier EXCLUSIVE requests of the other processes |
| wait EXCLUSIVE `.cache/b/d` | 557 -> 648 s (91 s, killed) | position 2 of 4 |
| Exclusive hold of a predecessor (from `dry-test.log` order) | walk 2 + walk 3 + deferred publish, 2-4 min each (observed holders: 93816 at 159 s, 93180 at 250 s, 95688 queued 84-99 s then holding) | all other processes blocked meanwhile |
| Time to reach the first line of Bazel | never within 10.8 min; coordinator's runs: 16.4 min (`dry-test.log`: 16:24 total, rc=1 at `readNativeStorageMountedChildV1`) and 12 min (`run-test-1.log`) | |

Warm-tree reference (walkperf, not re-measured): verify of a sealed receipt 3.6-7.2 s at load 56-65 (frontier 1.9-5.0 s, execution 2.8-3.5 s, everything else < 0.3 s); idle estimate 1-2 s. The Nx comparison (`bun repo run @teaching/architecture-course:test --dry-run`) goes through the same `script.ts:132` ingress and was not run separately: it would repeat the same queue (fleet of 6 course processes already in it).

Expected after Defect 1 (live) + staged set, warm: first run = 1 walk (about 50 s at this load) + replay (about 5-10 s) + best-effort publish (about 5 s, no queue); second run = verify only (3-7 s at this load), no exclusive request. NOT yet measured: re-run `timed.ts` after the nlink issue is gone (command in the handoff).

## Backend edits (live, minimal, by x-ingress)

1. `L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts:144` (15:36): `Math.min(262144,file.bytes.length-offset)` -> `Math.min(65536,file.bytes.length-offset)`. One token. Unblocks sealing of generation receipts. Effect on receipts: the file is part of the sealed closure, so any process that started before 15:36 defers once (`implementation-unsettled`); processes started after 15:36:05 can publish. Not covered by a test in the live tree yet; the staged suite below pins it (regression law with a 300 KB evidence through the real relative-create API).

## 4. Fix design

Goal: a warm unchanged tree reaches Bazel in seconds, a miss costs one walk, nothing on the ordinary path ever queues for an exclusive image lease, progress and cancellation stay.

Layers (all exist; the design only repairs and re-routes):
1. Receipt hit (fast tier, `verifyJavascriptDependencyGenerationReceiptV1`): SHARED lease, stamps and bounded reads, no walk. Needs a sealed receipt: Defect 1 (chunk ceiling) fixed live.
2. Receipt miss: one complete shared walk (`currentOwner`, progress `build:parser-source:*`, cancellable). Then readiness is decided in the same shared pass: `confirmGenerationWalkV1` replays the walk's own recorded observations in memory (capture with the sentinel implementation `in-process-replay`, verify with the same tiers, require `proof.sha256 === walk.sha256`; a deferral or any mismatch falls back to one more complete walk, i.e. today's behaviour). The old early `return "receipt-required"` after a walk is gone (`🏗️build/🚀️bootstrap/🟦️.ts:80`).
3. Publication of the generation receipt is best effort: SHARED image custody + writer lease `javascript-dependency-generation-writer:<ws>` acquired with `timeoutMs:0` (busy = skip), `store.recheck()` before and after (precedent `🏗️build/📊️reporting`). Failures are reported on stderr (`busy`, `error`; the writer already reports `deferred`, `stale`, `failed`) and never decide readiness. Only a cancelled run throws. Superseded receipts/evidence are retired with a 10 min grace so concurrent verifiers keep their evidence.
4. Exclusive custody remains only where it is a law: dependency install/sync and the parser receipt write after a dependency change (`inspect(true)`). That pass now adopts the shared walk by replay instead of re-walking.
5. Racy timestamps and closure churn are unchanged and sound: the 5 s settle rule defers sealing, the sealed 176-file closure invalidates the receipt on any edit; both now cost one walk and nothing else (before: 3 walks and an exclusive hold).
6. Revision-based skip of the remaining verify cost (frontier 75k lstat, execution 20k lstat): a per-store "verified at" stamp keyed by (receipt sha, boot id) valid for 2 s would let N parallel commands share one verification; not staged (needs a measurement of how often the fleet starts commands within 2 s of each other).
7. Scoping to the task closure: the bootstrap scope is always `.` (parser material), independent of the task, and `.` owns the whole workspace, so the check cannot be narrowed by task; the lever is the receipt (O(stamps)) not the walk. The per-task consumer path (`withJavascriptDependencyReadStorageV1`) is already scoped to the owner scope of the task and uses the same receipt.
8. Cross-process single flight of the walk on a miss (N parallel commands, one walk): needs a walk lease held across walk, replay and publish so followers re-verify after the leader published. With the shared best-effort publish this is now possible (the leader needs no exclusive image lease). Designed, not staged.

## 5. Staged files, tests, apply order

Handoff for the peer: `T/handoff-ingress.md` (apply order, relation to ingress-perf/walkperf/show-perf/graph-perf, open items).

Staged under `T/x-ingress-tools/` (nothing under the frozen layers was edited except the one-token Backend edit):
- `apply.py` (`check`, `apply [--dry-run] [--root R]`, `mirror DIR [--bare]`; anchors must occur exactly once, idempotent; `check` on the live tree: 24 operations pending, all anchors found), `check.sh` (mirror + tests + tsc), `mirror-test.py` (mirror and iterate until the module closure loads).
- `staged/core.append.ts` (pure policy appended to `G/🟦️.ts`: `confirmGenerationWalkV1`, `publishGenerationBestEffortV1`, `runGenerationWriterV1`, types), `staged/glue.insert.ts` (into `G/💾️storage/🟦️.ts`: `writeJavascriptDependencyGenerationFilesV1`, `replayJavascriptDependencyGenerationWalkV1`, `publishJavascriptDependencyGenerationReceiptSharedV1`, `reportJavascriptDependencyGenerationPublishV1`) plus 12 anchored edits (imports, capture accepts an implementation, publish/write/retire grace, writer extraction), 5 anchored edits in `🏗️build/🚀️bootstrap/🟦️.ts`.
- Tests (new member `G/🧪️tests/🚪️ingress/`): `staged/ingress.test.ts` (10 tests, 81 expects), `staged/ingress.fixture.json` (language-agnostic scenarios), `staged/ingress.schema.json` (fixture schema), `staged/ingress.oracle.py`.
- Tools: `timed.ts`, `repro-write-limit.ts`, `closure-churn.py`.

Tests and third-party oracles:
| Test | Independent check |
|---|---|
| fixture contract | Ajv (strict) validates fixture against its schema; ceiling equals the descriptor module's `RelativeCreateProtocol.maximumWriteBytes` read from its schema |
| confirm / publish / writer tables | Python oracle derives the expected result of every fixture row |
| writer with the real lease | Python `sqlite3` holds `BEGIN EXCLUSIVE` on the real lease database: writer returns `busy` in < 2 s, runs after release |
| files writer | evidence of 0, 1, 65535, 65536, 65537, 300000, 5000000 bytes through the real relative-create API; sha256 equals Python `hashlib` of the same deterministic pattern; failing write leaves no file; different-size existing file refused |
| retire grace | young unreferenced evidence stays under grace, goes without |
| replay | real `withCurrentBunWorkspaceOwnershipV1` topology + real read custody: unchanged tree -> match with the walk's sha; unsettled -> deferred; edited `package.json` -> `input-changed`; restored -> match |
| bootstrap source laws | no `return "receipt-required"` after a walk, one `currentOwner()` left, best-effort publish present and free of `withJavascriptDependencyStorageV1`, the `!publishReceipt` branch byte-identical, exactly 2 catch-with-return |

Verified in a scratch mirror outside the repo (`sh check.sh <dir>`): new suite 10 pass 0 fail; mutation (`relativeWriteCeilingBytes` back to 262144) fails 2 tests; existing suites in patched vs unpatched mirror have identical failing sets (parsers: 2 pass 2 fail both; generation: 11 pass 1 fail both; the failures need the real envelope/tree); `tsc --noEmit` over bootstrap, core, glue and the new test: no diagnostic in the touched files; `apply.py apply` twice: second run 0 pending.

Apply order: see handoff (other closure edits first, then `apply.py apply`, `check.sh`, existing suites, live validation with two consecutive dry-runs under `timed.ts`).

Not done: live application and live timing of the staged set (frozen); the second real measurement (blocked by nlink=2 on `package.json`/`bun.lock` at 15:55); single flight; verify stamp; taxonomy admission of the new test member.

## 6. Follow-up after the review (16:30-17:00)

State on resume: the machine rebooted at 16:03; the peer applied `x-ingress-tools/apply.py` live (24 operations, 10/10 tests) and sealed receipts exist since 16:25 (`.cache/b/d/m/generation/*.receipt.json`, 20 MB frontier), which confirms Defect 1 and its fix. `review-release-ingress.md` raised X-I1, X-I2, C1, X-I3, X-I6 (plus X-I4, X-I7).

Staged under `T/x-ingress-tools/followup/` (no live edit; the tree is quiet): `apply.py --check|--live|mirror` (two-phase, anchored, writes nothing on any refusal), `check.sh`, `test_apply.py` (6 tests), `staged/` (core append `guardGenerationReplayV1` + `generationWalkOriginV1`, guarded replay wrapper, files writer with fresh-scope leaf removal, new test member `🧪️tests/🛡️followup` with JSON fixture, schema, Python oracle, 7 tests). Handoff for the peer: `T/handoff-ingress-followup.md`.

Findings while fixing: (1) the descriptor protocol makes in-scope removal of a failed leaf impossible (the failed child poisons the directory frame, "Live relative child admission required"), so X-I3 is implemented as removal in a fresh descriptor scope, not in the failing one; (2) containment of the replay (X-I1) also covers the new C1 cap, because a capture above 64 MiB now throws early and becomes a deferral; (3) the reviewer's "three receipts for one scope" is the 10 minute retire grace of the shared publisher, not a retirement defect.

Verification: follow-up suite 7/7, x-ingress suite 10/10, apply tests 6/6, tsc clean in the touched files, three mutations detected; the single failing peer generation law (`projectListing(` count) fails identically without the follow-up. Remaining: live apply and the two-run warm timing (needs the peer's `show projects` to finish and `package.json`/`bun.lock` nlink back to 1).
