# R151: why a read restarts inside itself, and why it never hands over (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R151/` (`patches.py` + `patches2.py` the anchored edits, `land.py`/`land2.py`/`window.sh`/`window2.sh` the landings with a tested restore, `battery.sh`/`tests.list`/`compare.py` the battery, `probe.ts` the in-process measurement, `stage.py`/`stagetree.py` the staging). Landing entries: `landings.md`, "R151 landing 1" (21:24:38 to 21:25:19 UTC) and "R151 landing 2" (21:27:09 to 21:27:30 UTC). Scratch: `scratchpad/R151/` (staged copies, probe output).

## 1. Mechanism (what the evidence shows)

**The count is the progress of ONE invocation of the owner loop.** `taken N of M` is `finished` of `inferProjectNodes` (`📦️project-nodes/🟨️.mjs`: `taken` Set, take ledger, `reportedProgress = max(...)`): inside one invocation it never goes down. So "backwards inside the same read" means the loop was entered again in the same process. The read is one process: `slice-memory.json` of g1-m07 says pid 48765, uptime 14063 s, phase `taken 2033 of 4455`, while `owners.resolvedProjects` says **24777 projects finished** in this process (5.6 times 4455), `lookup.count 24782`, cpu 6405 s user + 2483 s system, `lookup.reusable` 10.4 million ms. That is five to six full passes inside one process.

**Three things re-enter the loop in one process**, all of them whole-closure repeats:

| # | where | trigger | what restarts |
|---|---|---|---|
| 1 | `loadBuildGraphUnpinnedV1` attempt loop (up to 9 attempts) | `loadBuildGraphAttemptV1` ends in `invalidated("observed-facts" / "producer-error" / "membership" / "admission")`: ANY of the roughly 200000 observation rows of the attempt (every file read by every owner, tooling files included) is stale at the end | admission closure (memo invalid because it holds the stale row), the whole owner discovery, final admission |
| 2 | `settleObservedComputationV1` rounds (up to 64) | a failing or inconsistent round while any observed fact changed during it | the whole compute of the round |
| 3 | pool hand-off loads (R140 wall 19) | owners phase ended with a refusal | the load, from the start |

Real evidence for 1 (service logs of the slice lab, copied before they vanish):

- Lab run of R143 (`/tmp/sl-c1091`, 16 GB / 8 cores, slice `repo-mcp`, 53 owners): all 53 owners done at 270 s, then at about 340 s `[graph-retry] {"reason":"observed-facts","before":"b6d233...","after":"b6d233...","facts":[{"kind":"file","path":"...native-build/💾️storage/🧱️fixed/🟦️.ts","before":"e269b4...","after":"e0b45c..."}]}`: membership unchanged, **one peer edit of one tooling file** invalidated the finished attempt. The second attempt logged `admission-memo-write 56736 ms` (admission derived again, the memo holds the stale row) and `graph-original-inference 5304 ms` (records reused: the inference itself was cheap, the admission was the cost).
- R143 run 3 (`m1-host-1855acb16923.log`): two `[graph-retry] reason producer-error` lines, the second one naming `📦️project-nodes/🟨️.mjs` (`ebaae770a6d0` -> `17b9c97be5ac...`, the R141 landing) as the changed fact.
- R140 run 5 (`/tmp/sl-cb6c1`): `[graph-settle] {"round":0,"failure":"unserved build.graph.owner-too-heavy ..."}` then a full second round (the "second pass starting at 0" of the brief; it is mechanism 2 after a genuine failure while facts moved during the long round).

With 4455 owners one pass costs, measured on a worker, about 7 owners/s of pure record validation at load 50 (10 minutes), plus the admission closure (285 to 538 s). With peers landing every few minutes a 25-minute pass is invalidated by a large share of edits, which is why the pass restarts again and again (g1-m07: 4382 of 4455, then 1657, then 4407 at 14364 s).

**Why the reads at 13000 s kept going on the 4455-owner admission after R141.** Three independent reasons, all confirmed in code:
1. A running process never reloads its modules. R141's smaller closure is in `closeBuildOwnerAdmissionV1`; a process that loaded the old admission code derives the old closure in every attempt for ever.
2. R139's hand-over (`SEMIO_STALE_IMPLEMENTATION`) needs a WAITER on the same door; a lone worker has none. Its tree identity is `hostImplementationV1`, the digest of 18 SERVICE files only (`HostPolicy.serviceRevisionFiles`): a landing in the inference library never changes it.
3. The readiness supervisor adopts the running stage ("Adopted stage workspace, which was already running; not restarted", `ready.json`) and restarts for new code ONLY while the stage waits for a NAMED HOLDER for 10 minutes (`readyHolderRestartV1`); a stage that prints progress, even backwards, is never restarted, and the `codeRoots` of that check (`process` module, `⚡️caching`) did not contain the build library. The only end was the stage cap: g1-m07's read ended at 14364 s with `stage-timeout`, 4407 of 4455 taken.

**How wide is the validity today.** Narrow for owner RECORDS (the stamp protocol: 18 inferrer groups, `none` for infrastructure; an edit below a declared fragment does not change a record identity, only the stamp does, and the unclassified-code law is green), wide for the ATTEMPT: its authority receipt holds the content fact of every file any owner read, so an edit of any tooling file that an owner's command closure imports (the cargo script closure, `nativeCommandInputs`, whose key is also part of the record key of every native owner) stales the attempt even when no inference result changes. The admission memo (R140 wall 22) behaves the same: one stale row of 444157 dropped it. What a content edit of a file in the command closure really does to the graph is semantic (the digests of the command inputs are part of the node), so the record-key part stays as it is (open, section 6); the restart cost of such an edit is what this package removes.

## 2. What was built (all red first, all landed under the lock)

| change | where | law (red before, green after) |
|---|---|---|
| per owner admission memo: the facts of every owner observed in this process with the rows that observation consumed; a repeated attempt gets the answer while its rows hold, reused rows are retained by the capture (`retainObservedRowsV1`, existing) | `📥️load/🟦️.ts` (`observeOwnerFactsMemoizedV1`, `rememberOwnerFactsV1`), limits `ownerFactsMemoEntries` 16384 | `🧪️tests/🕸️graph/🩹️repair-rounds` (9): one edit re-observes the owners whose rows it touched and no others; eviction keeps the newest entries |
| repair in place: when the owner phase's facts are stale but membership and admission are unchanged, only the inference is repeated (`discoverOnce`), at most `repairRounds` 4 times, the admission rows of the hold check are those of the FINAL inventory | `loadBuildGraphAttemptV1` | same file: edit of an unrelated file (no effect), of an owner's inference source (inference repeated once, no owner re-observed), of an owner's admission file (that owner only), of a tooling file read by every owner (each owner once), bounded rounds then restart, changed membership is not repaired |
| stage on stale code hands over: a stage that shows progress is stopped with its tree and restarted at once when the code changed after its start and it has run `staleStageMilliseconds` (30 min, times the machine speed factor); `codeRoots` now include the build library; new refusal `stage-code-stale` (EN/DE), reason `code-changed`, not a failure or a stall | `🚦️ready` runner, process-tree policy | `🚦️ready/🧪️tests/🕰️stale-stage` (5, real runner processes: a progressing stage is stopped after the stale age on a changed file, a young stage and an unchanged stage are not), ready main test (handler: no back-off, no attempt, both languages), process-tree scaled policy test |
| never silent: `[graph-retry]` (reason + stale paths) on every invalidated attempt (before only with `SEMIO_BUILD_TRACE_AUTHORITY=true`), `[graph-repair]` per repair round | `📥️load/🟦️.ts` | repair-rounds test "said with the sources that caused it" |

Battery before/after (16 test directories, compared): nothing worse; pre-existing reds unchanged (`🔄️candidate` 2, `🎟️phase` 10: not mine, red on the unchanged tree; the candidate model test lacks the `readAdmissionMemoV1` binding).

## 3. Edit kind x what was re-done (before / after)

Law numbers are exact (`🧪️tests/🕸️graph/🩹️repair-rounds`, the real `loadBuildGraphAttemptV1` and the real memo function extracted from the source with scripted rows, 6 owners). Wall numbers are real production code on the live tree, in process (`probe.ts`, `mini.ts`), machine load 45 to 68, so every wall is 2 to 5 times an idle machine's.

| edit during the read | before (old code) | after |
|---|---|---|
| (a) a file no row of any owner contains (an unrelated owner's source, any file outside the closure) | nothing stale, nothing redone (but every attempt derived the admission twice: initial + final inventory) | nothing redone; admission observed once per owner (law: 1 observation each) |
| (b) a source file one owner's inference read (a stale row of the discovery only) | `invalidated("observed-facts")`: new attempt = admission closure again (lab: `admission-memo-write` 56.7 s for 53 owners, 57 owners 61 s old retry vs 22.7 s with the memo) + the whole discovery (5.3 s with all records present, 7 owners/s at 4455 owners) | the inference is repeated once in place (`graph-repair`), the owner re-infers, every other record is taken, 0 owners re-observed (law: `discovered` 2, each owner observed once) |
| (b2) a file one owner's ADMISSION read | restart, all owners observed again | exactly that owner observed again (law: `[1,1,2,1,1,1]`; real rows of `repo-mcp`, `mini.ts`: edit of its `Cargo.toml` -> 1 observation, edit of an unread file -> 0) |
| (c) a tooling file in the engine closure that every owner's admission read | restart per attempt: all owners observed again each time (57 of 57, 54 to 68 CPU s for the 3 probes) | all owners observed again ONCE per edit and not per attempt (law `[2,2,2,2,2,2]`); the repair does not restart the attempt |
| (d) a tooling file no owner read | no effect | no effect |
| many edits in a row (a tree that changes every round) | 9 attempts, each from admission, then `Build source authority keeps changing during inference` | 4 repair rounds per attempt (inference only), then the old restart; bounded (law) |
| membership or admission changes | restart | restart (law: not repaired in place) |

Measured admission cost of a retry (in process, no edit), 3 probes = 57 owners / 204146 rows: cold 82 to 107 s wall (54 to 68 CPU s); retry with the memo 22.7 to 27 s wall (the hold check of 204k rows, 12 to 18 CPU s); retry without the memo (old behaviour) 46 to 61 s wall (32 to 41 CPU s). `repo-mcp` alone, 53 owners / 132536 rows: cold 55 to 60 s, retry with memo 7 to 15 s, old retry 53 s. Not reliable and not used: my probe's simulated edits of (b) and (c) did not trigger in two of three closure runs (the real hold check rebases the rows in place, so a later path comparison of mine misses them); the production function itself was then checked on real rows in `mini.ts` (the line above) and by the laws.

## 4. Watching the workers after the landing (store `/Users/Shared/semio-store/v1/origins/<m>`, samples every 20 s from 21:32 UTC, `scratchpad/R151/watch.txt`)

Facts read from `ready-timings.json`, `ready.json`, the ready logs and `slice-memory.json` of each machine (the service logs with `[graph-retry]` / `[graph-repair]` stay on the workers and are not in the store):

- **g1-m08** (hand-over proven on a real worker): its previous run `workspace` ran 20:02:29 to 21:26:41 (5052 s) with steady progress (`taken 3675 of 4455` in the last lines) and ended with outcome `stalled` 80 seconds after landing 1; no holder wait was in its log. That is the new stale rule (the old rule needs a holder wait of 10 minutes). The new run started at once on the current code (admission 590 owners instead of 4455).
- **g1-m01**: its read stood at `taken 307 of 590` for 11 minutes and was stopped at 21:43 with reason `code-stale` (the new reason, landing 3) after 30 minutes, and the new read was at `taken 588 of 590` after 415 s. Before the landing the same machine showed the old shape: `588 of 590` at 916 s, then `307 of 590` at 1130 s inside one read (attempt invalidated at the very end of the first pass).
- **g1-m07**: the 3.99-hour read ended only at the stage cap (`stage-timeout`, 14364 s, `taken 4407 of 4455`); the following run (21:26:25, after landing 1) failed after 366 s with `Original project inference observations required` (service reference 83ec1075, `observeProjectInferenceV1`, owners phase): the first occurrence of that text in the store, not in the admission or repair path; its cause is NOT established here (a record that was not published, for example after a racy read) and it is the next thing to look at on that machine.
- **g1-m10**: `stage-timeout` at 21:15 (4 h), then new reads: `1195 of 4455` (old admission), then `590`-owner reads, `taken 581 of 590` at 897 s.
- **g1-m02**: the client's read timer was still counting (`16529 s`, reason `adopted`, `taken 567 of 4455`) at 21:48, but `slice-memory.json` shows a slice worker process that is only 1824 s old at 21:51, so the worker process was replaced inside that client read around 21:21, BEFORE landing 1 (not credited to the stale rule; why it was replaced is not established). Note for every number above: `(N s)` in the log is the client's wait, the process age is `uptimeSeconds` of the slice report; they agree only for a process that was never replaced (g1-m07 before 21:05: 13973 s both).
- **Counts that still fall inside one read on the new code** (g1-m08 `251 -> 113` at 21:44:15 at read second 1020, g1-m10 `289 -> 173` at 21:45:38): a repair round enters the owner loop again and starts from the stored records, so a fall is expected after a repair; it can also be a settle round. They cannot be told apart from the store; landing 4 (21:50) adds `owners.pass` to `slice-memory.json`, which counts the entries of the owner loop per process, so the next reading of the store tells them apart (the new field was not yet present in any worker at the end of this package).
- **No worker reached `ready: true` during the watch.** The fix removes whole-closure restarts and stale reads; the tail of the take pass (the owners a 16 GB host refuses, R142/R145) is not touched.




## 5. Not verified

- No slice-lab run: two labs of other agents were running during the whole package (rule: one lab at a time), so `[graph-repair]` was never seen in a real service log; the edit-during-read table rests on the model laws with the real functions and on the in-process measurement.
- The simulated edits of my closure probe (b) and (c) did not trigger (see section 3); the real function was checked on real rows instead (`mini.ts`).
- `staleStageMilliseconds` 30 minutes is a declared number, not measured against a real landing rate; a worker on a busy tree is restarted at most every 30 minutes (times its speed factor), records persist.

## 6. Not done, open (honest list)

- **Settle rounds (mechanism 2) and the owner-level carry.** A repair round still re-runs the whole discovery: unchanged owners are looked up again (about 7 owners/s at load 50 on a 4455-owner closure). A carry of the previous round's owner results (skip the owners no stale row touches) is the next step; the 57-owner closure that R141 gave the readiness probes makes it cheap enough today.
- **Admission re-check at a derived interval inside the owners phase** (the brief's item 3, second half): done only at the repair boundary (after each repair round `inventory()` runs again and a changed admission ends the attempt, so the next attempt works on the shrunk closure); not inside a single long pass.
- **Record validity for edits of files in the command-source closure** (case c) is still the digest of that closure (`commandRevision` in the record key of every native owner). Narrowing it is a semantic change (command input digests are in the node), not a hash change; the lead is to keep the digest of the command closure out of the node until a task runs.
- The staged-code hand-over of a process that has no supervisor (a plain `repo` read in a terminal) is still R139's waiter rule only.
