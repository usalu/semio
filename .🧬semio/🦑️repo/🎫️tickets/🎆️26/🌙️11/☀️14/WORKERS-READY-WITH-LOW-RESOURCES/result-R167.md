# R167: "Build source authority keeps changing during inference"

Scripts: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R167/` (`patches.py`, `scale.py`, `land.sh`, `preload.ts`). Scratch: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R167/`.

## 1. The wall, counted

- Origins searched: g1-m01 to g1-m10 and g1-server under `/Users/Shared/semio-store/v1/origins/`. The text occurs on **one machine, once**: g1-m03, request `f0be4e1a`, 2026-11-18 00:27:06 UTC (files `service-failure*.txt`, `ready.json`, `ready-log-workspace*.txt` are copies of that one event). Since 22:30 UTC: **1 occurrence** (no other origin; "Generation keeps changing", "fragments keep disappearing", "closure.moving", "removed during discovery": 0 each).
- That read had run 11,381 s (3.1 h, started about 21:18 UTC) in a process that loaded the code of before R154/R156 (R154 landed 22:12, R156 22:22). The stack says `📥️load/🟦️.ts:217:9`; HEAD has the throw at 210, the live file today at 256, so 217 is an intermediate version, not today's code. The progress line was frozen at "taken 229 of 590" for the last 30 minutes.
- What the throw site is: `loadBuildGraphUnpinnedV1`, the outer loop `for(attempt=0;attempt<=8)` around `loadBuildGraphAttemptV1`. It repeats while an attempt returns "invalidated" (or `undefined`). In HEAD, an attempt returned that for 7 reasons (`admission`, 3x `membership`, 3x `owner-registry`, `producer-error`, `publication-facts`). So 9 attempts, each invalidated by a tree that moves every few seconds, ended the read: "retry until nothing changed, give up after nine". "Source authority" here is the observed facts rows (files and digests read by the inference: manifests, scripts, shared tooling) plus the membership digest plus the owner registry file `📋️build-owners.json`; it is not the implementation (that is pinned by `withImplementationPinV1`, which only names the wrapper in the stack).

### Classification (R156 table, extended)

| site | what changed | before | fate |
|---|---|---|---|
| attempt: stale observed row | a source read by the inference | restart (outer loop) | R156: repair round on the fresh inventory, seal after 4 rounds |
| attempt: membership moved | file set of the owners | restart | R156: repair round, seal |
| attempt: admission moved | owner closure | restart | R156: repair round, seal |
| attempt: producer error with stale facts | source changed under an inference | restart | R156: repair round, then typed `code-stale` (R154) |
| attempt: publication facts | observed rows after the write | restart | R156: seal records under the facts it read |
| attempt: owner registry (3 sites: `inventory`, `discoverOnce`, after produce) | `📋️build-owners.json` or the selection scope key moved | restart, **9 attempts then throw** | **R167: adopted, no count** (below) |
| implementation files | any loaded Source edited | `Stale loaded Source revision`, restart | already pinned (`implementationPinnedV1`: the read completes on what it loaded; R151 `code-stale` for the next read) |

## 2. The fix (R156 model)

Finding on the live code: since R156 the only invalidation an attempt can still return is `owner-registry`; under the implementation pin the registry file is read once per read (`pinnedAuthorityReadV1`, cleared at the start of `loadBuildGraphOnceV1`), so within a pinned read the scope key cannot differ from the one the attempt started with: the pinned read is a single attempt. The nine-attempt loop was therefore dead weight in production and a wall in every unpinned or older-code case (and in m03's process).

Edit (`🕸️graph/📥️load/🟦️.ts`, anchored in `patches.py`): `for(attempt<=8)` becomes `for(;;)`; the final `throw Error("Build source authority keeps changing during inference")` is removed; on a moved owner registry the loop calls `clearPinnedAuthorityV1()` (adopts the new registry: the next attempt reads it once, pinned again, plans the scope key of the new registry, owners are inferred or served from their own records) and prints `[graph-repair] {"registry":"adopted"}`. Cancellation (`checkScriptInvocation`) stays at the top of every iteration; every iteration is a full attempt that holds or returns data, so there is no count to hit. The result is published under the registry it was computed with (the scope key is a hash of selection and registry), a later registry change makes the NEXT read plan a new scope key (stale for the next read only).

Not built (R156 item 3, the semantics revision for per-owner acceptance across machines): the sealed result carries the digest per owner (`seal.owners`) and the implementation identity (`module` of the authority receipt), but no semantics revision that lets another machine accept or reject per owner. I did not touch it; R164 owns acceptance across machines. Note for R163: `ownerFactsMemoV1` keys include the `scope` string, so a moved registry (new scope key) loses every memo entry; if registry moves are frequent, key the admission memo by owner and selected targets without the registry digest, or the adopt step re-observes every owner.

### Bounded retry loops in `🕸️graph/` and `📥️source/` (fate)

| loop | bound | fate |
|---|---|---|
| `📥️load` outer (`loadBuildGraphUnpinnedV1`) | 9 | **removed** (this report) |
| `📥️load` `settledInventory` (`BUILD_MEMBERSHIP_CHANGED`, 4 tries) | `repairRounds` | kept: the error is "Build directory/member removed during discovery" (a file vanishing inside one short walk), a different text; after production R156 already returns `undefined` and seals; the very first inventory still throws after 4. Residual risk: named for follow-up, 0 occurrences in logs |
| `📥️load` repair rounds (2 sites) | `repairRounds` 4 | R156: seal after the bound (not a give-up) |
| `🕰️generations` `read` pointer/manifest (151) | 8 | kept: a manifest retired between two reads, sleeps 5 ms x n; not source churn; 0 occurrences |
| `🕰️generations` build resume (439) "Generation keeps changing while it is built" | 8 | kept: counts resumes of a crashed builder's journal, each iteration carries computed owners; not source churn; 0 occurrences |
| `🕰️generations` snapshot (513) "fragments keep disappearing" | 8 | kept: a retired fragment, not a source change; 0 occurrences |
| `🕰️generations` refreshClosure rounds (529) | 8 | kept: widens the verified closure, ends with the result (returns, never throws) |
| `🧩️claims` `tryHold` | 3 | kept: lock-file takeover, no source relation |
| `🚪️admission` pass loop | owners + 65536 | kept: a fixpoint over a finite closure |
| `📦️project-nodes` owner-pool / farm (4, 2, EMPTY) | 4 / 2 | kept: lost-process re-dispatch, R163/R164 own it |
| `🧭️script-inputs/💾️entries` | 2 | kept: cache entry re-read |
| `📥️source/🔏️closure/🛂️readmission` | 6 attempts, growing pause, then refusal `closure.moving` (exit 75) | kept (named): it repeats only while ONE capture tears (a file changes between hash and load), a typed refusal not an internal error; 0 occurrences in logs; if it shows up, the same R156 treatment (seal the capture) applies |

## 3. Laws

Red first, then green against the staged files (a bun preload `preload.ts` substitutes staged copies; nothing live edited for tests before the window):

- `🧪️tests/🕸️graph/🔄️candidate` (existing law, changed): the case "all nine invalidated" is now "the registry moves nine times and the tenth read holds"; new cases "sixty-four times and no count gives up" and "ALL: registry and source authority move forty times"; the "keeps changing" branch is gone; schema bound 9 to 128. Staged test on OLD code: 0 pass 3 fail; on staged code: the first test passes (1 pass; the two other fails are baseline red on live, the inventory model of R154/R156: same 1 pass 2 fail before my change).
- New law `🧪️tests/🕸️graph/🧲️adopt-registry` (3 tests): (a) 200 seeded worlds where registry and implementation versions move on a logical clock (periods 1 to 400, work 5 to 250 ms), pinned read: exactly 1 attempt, no clear, completes on the registry it loaded; (b) 200 worlds, unpinned, registry moves up to 119 times: attempts = moves + 1 at most, every move adopted (`clears = attempts - 1`), result carries the last registry, and at least one world needs more than the old bound of nine; red on old code ("keeps changing"), green on new; (c) a seal of 4096 stale owners names all of them (was cut at 64) and the authority receipt schema admits the list.
- R156's churn law `🌊️seal-under-churn` 3/3 and `🩹️repair-rounds` 11/11, `🧊️stale-facts` 2/2, `🔁️settle-repeat`, `🔄️settlement`, retry projection 4/4: identical old vs new. Baseline reds (unchanged, not mine): `🛟️records-first`, `🔌️nx-engine`, `🛂️admission`, `📌️pinned-publication`, `🔭️observed-facts`, observation 51/52, `📥️birth` 7/15.

## 4. Extra item: `SealPolicy.maximumOwners = 64`

What it bounds: only the diagnostic list `seal.laterChangedOwners` (the owners of the stale rows a sealed read names; `sealVerdictV1` slices to it) and its mirror `maxItems: 64` in the retry `Seal` schema and in the authority receipt schema `🧬️schema/📥️authority.json` `properties/seal/properties/laterChangedOwners`. It does not bound reads or work (`seal.owners`, the per-owner digests, is unbounded and `sealedWithLaterChanges` is the exact count). It is a workspace-scale collection (owners), so it is derived: all three values are 9548 (4774 owners x 2) and are three members of the workspace-scale family (`scale: owners`) in one `equal` group, in `scale.py`. The list is still cut by `maximumStalePaths` (4096 stale paths kept, a diagnostic count, not named for a workspace collection; the count stays exact). Same edit removes two now-stale exempt rows in the family (the `tasks` explicit-selection bounds of `🏗️build/🧬️schema/🔣️.json` and `🚪️selection.json` are 65536, above the bar; an exempt must stay below it, so the growth law failed on them since R160's landing). Scratch mirror of the two schema trees: the three scale laws 2/3 before, 3/3 after (the red cause before = the two exempts plus `maximumOwners`).

## 5. Landing and watch

Landed 2026-11-18 01:02:59 to 01:03:20 UTC under `landing.lock` (hold 21 s; the 01:00:00 try found R166 holding it and changed nothing). Under the lock: candidate 1 pass 2 fail (the 2 are baseline red, unchanged), adopt-registry 3/3, seal-under-churn 3/3, repair-rounds 11/11, growth `ceiling` filter 5/5 (4/5 before), retry projection 4/4. Entry appended to `landings.md`.

Watch (01:03 to 01:10 UTC, only the first minutes after the landing, workers were still restarting): occurrences of the old text in any origin since the landing: **0** (the only files holding it are the three g1-m03 copies of the 00:27 event). Walls seen at that time on other machines, none of this loop: g1-m01 `Partition / is 97.1% used (limit 97%)` (disk, not source authority); g1-m07 and g1-m08 `SDK derivative publication byte ceiling exceeded` (R166); g1-m02 an acquireWaitingOutsideGate lease wait with `taken 0 of 4455`; g1-m08 reason `code-stale` (R151 hand-over working). The next wall of g1-m03 is not known yet: its process was restarted (`running`, workspace stage) and has not finished a read; check `origins/g1-m03/ready-log-workspace*.txt` after about 15 quiet minutes.

## 6. Not verified

- No live read of the whole workspace was run (rule): the outer loop is proven on the real extracted function over a model, not on a 590-owner read.
- The claim that m03's process ran pre-R154 code rests on the stack line number (217 vs HEAD 210 vs live 256) and the 3.1 h duration, not on a copy of that process's source (no copy of it exists in the bootstrap store).
- The semantics revision for per-owner acceptance across machines (R156 item 3) is not built (R164).
- `settledInventory`'s first inventory (4 tries on `BUILD_MEMBERSHIP_CHANGED`) and the `closure.moving` re-admission (6 attempts) are the remaining bounded repeat-until-stable loops; neither has occurred in the logs.
