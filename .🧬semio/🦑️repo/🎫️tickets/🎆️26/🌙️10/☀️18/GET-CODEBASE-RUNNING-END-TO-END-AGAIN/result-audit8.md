# result-audit8 (lane audit8, prefix a8, wave 1027b): pre-landing adversarial review of the staged G45 queue

STATUS 16:50 FINAL. 24 batches reviewed against their base→new diffs and the live code. Verdicts: 17 LAND, 4 LAND with ordering or follow-up, 3 FIX FIRST. The 3 FIX FIRST batches have a corrected copy each, verified on mirror `a8m` (dropped). A 4th, `allrun`, needs a precise change in its lane. Nothing live was touched. No official run, no ticket tool, no process of mine still running.

FOR coordinator: land the audit8 copies INSTEAD of the lane copies (same base hashes, `check` = `base` on live 16:40, lane stage.py kept, mode law OK):
- `python3 -I C/audit8-tools/staged/psgate/stage.py apply --root <root>` · verify `sh C/audit8-tools/staged/psgate/verify.sh <root>` (25/0 bare, 25/0 under `sandbox-exec`) · revert `… revert --root <root>`
- `python3 -I C/audit8-tools/staged/windows/stage.py apply --root <root>` · verify on a mirror: `sh C/audit8-tools/staged/windows/verify.sh <mirror root>` (official envelope via td-misc envrun.sh, 3/0 twice at load 64) · revert `… revert --root <root>`
- `python3 -I C/audit8-tools/staged/consumers/stage.py apply --root <root>` · verify `sh C/audit8-tools/staged/consumers/verify.sh <root>` (peeredit2's 19-suite runner, same allowed.txt) · revert `… revert --root <root>`
- Rebuild of all three from the lanes' current staged bytes (anchored, exit 1 = anchor lost): `python3 -I C/audit8-tools/make_fixes.py`
FOR agents16 / coordinator: HOLD `allrun` and the `behind` batch stacked on it until the reduced-upgrade change in §allrun below lands (evidence: your own stored vector `followup-150`).
FOR coordinator (landing order, shared base hashes; whichever lands second reports `drifted`):
1. `td-leases/startup` and `agents16/allrun` (+`behind`) both edit `📚️library/🏗️build/🌡️host-capacity/🟦️.ts` (base 398bc8c3). Land `startup` first, then re-cut with `python3 -I C/agents16-tools/make.py allrun` and `… behind`.
2. `agents16/farmfree` and `audit7/contracts` both edit `🏊️project-farm/🟨️.mjs` (base f3a178d2). `reclaim` is stacked on `contracts`. Land `contracts` → `reclaim`, then re-cut with `python3 -I C/agents16-tools/make.py farmfree`.
3. `td-capacity/envelope` needs `td-leases/envelope` + `td-capacity/launcherns` + `td-capacity/custodian` live first.

## Verdicts

| batch | verdict | one-line reason |
|---|---|---|
| gatefast `scopedcut` | LAND | Sound (§scopedcut). A deferred question keeps its stale support. Dirtiness was already filtered by `only`. Coverage stamps cover only `only`. |
| td-ucache `scratch` | LAND | Test-only. No assertion changed (0 `expect` edits). Cargo's own home config is correctly excluded. Cargo and rustup are the oracle. |
| td-misc `texts` | LAND | Uncatalogued English throw → catalogued refusal. |
| td-misc `artdir` | LAND | `?? tmpdir()` house idiom. It drops a `toBeTruthy` precondition, not a contract law. |
| td-misc `resident` | LAND + follow-up | Strictly safer than base. Still incomplete: it checks trust at query time, not over the interval (§resident). |
| td-misc `windows` | FIX FIRST → audit8 copy | The `leaf-transient` op count was weakened from exact to `>= single`. That neither proves a retry happened nor bounds it (§windows). |
| td-misc `consumption` | LAND | Hard-coded ticket sink → artifact dir / tmpdir. |
| td-leases `envelope` | LAND (notes) | Opt-in. The in-process birth is used only without a parent. Under a parent, capabilities must be equal (§envelope). |
| td-leases `startup` | LAND, order 1 | Re-exports keep every importer. Same JSON object (`cargoExecutionSchemaV1` was already the raw JSON). Budgets re-measured: 2 tightened, 3 raised with an import-trace argument. |
| td-capacity `timeouts` | LAND | Explicit 180 s on whole-framework scans. |
| td-capacity `launcherns` | LAND | The regex accepts only `store` or `options` as the single owner of both arguments, as the C1 code reads them. |
| td-capacity `psgate` | FIX FIRST → audit8 copy | The expectation came from the production function itself, so the law masked a broken process-table reader (§psgate, demonstrated). |
| td-capacity `custodian` | LAND | The fixture refuses an inherited Original, so a raw spawn without the envelope is the correct independent child. The deadline is a fixture value. |
| td-capacity `envelope` | LAND, order 3 | `capabilities:{}` matches the suites' rows (no `artifacts` key). Marker race fixed. |
| agents16 `allrun` | FIX FIRST (lane change) | A reduced upgrade takes every core left, so a fresh run without a session that used to be admitted now waits for the whole long run (§allrun). |
| agents16 `farmfree` | LAND, order 2 | Own-pool release is a no-op and `capacity.release` is idempotent. LOW: the grant goes back up to `shutdownGraceMilliseconds` before the workers exit. |
| agents16 `behind` | LAND after `allrun` | Stacked on allrun. No cross-user leak (directory per user). Labels are argv parts ≤ 120 chars, never empty. LOW: allow-list for control characters (§behind). |
| td-products `dashjourneys` | LAND + follow-up | The dirty-label wait fix is correct. The automatic scaling of the SLO by load can hide a 3-5x responsiveness regression on the gate host (§dashjourneys). |
| td-products `fixable` | LAND | Restores the intentional fixture defect. Re-sweep risk noted by the lane. |
| td-products `coursevt` | LAND | Two real Vitest suites that were in no include list. |
| audit7 `contracts` | LAND, order 2 | The worker uses the parent's contracts. Session v3. Laws are red on live. |
| audit7 `reclaim` | LAND, order 2 | Deletes only on ESRCH, so PID reuse or EPERM keeps the directory. Another user's dir in sticky `/tmp` → rm fails, caught. |
| peeredit2 `consumers` | FIX FIRST → audit8 copy | The fix is correct for today's reference shape but fails open on any other closure object (§consumers). |

## Findings with scenarios and fixes

### psgate (MEDIUM, masking): the law takes its expectation from the code under test
Staged: `const readable=liveChildPidsV1()!==undefined; expect(lent).toBe(readable)`. `liveChildPidsV1` runs `spawnSync("ps",…)` through PATH and answers `undefined` on any failure.
- Scenario: a production regression makes the reader fail on a normal host. Examples: PATH without `/bin` in an allow-listed action env, a parse change, a 10 s timeout under load. Lending then silently never happens anywhere, and the only real-process lending law turns green with the message "never lent".
- Demonstrated on `a8m` with `env PATH=/nonexistent bun test -t "In a real process"`: the td-capacity copy gives 1 pass / 0 fail (masked). The audit8 copy gives 0 pass / 1 fail: `the production process-table reader answers exactly where an independent ps call can read the table, Expected true, Received false`.
- Fix (`C/audit8-tools/staged/psgate`): the expectation comes from an independent probe, `/bin/ps` or `/usr/bin/ps` `-o pid= -p <pid>` (win32: readable). The law asserts that the production reader agrees with that probe, then asserts `lent === readable`. Under the mirror seal both fail, so the law still passes there: 25/0 sandboxed, 25/0 bare. One bare run had 24/1 on an unrelated timing test at load 74 and was 25/0 on rerun.
- Cosmetic: the lane's stage.py docstring says "print's Bazel plan becomes reusable …" (copied template text).

### windows (LOW-MEDIUM, weakened assertion)
The old expectation (rejected) is proven wrong by code: `observe()` wraps `observeOnce` in `retryOnConcurrentEditV1("read", …)`, and the sibling law asserts that wrapper. The replacement `readOperations >= single` is weaker than needed:
- A single unretried pass satisfies it, so the law no longer proves the race was hit and healed.
- It is unbounded above, so a retry storm passes.

Fix (`C/audit8-tools/staged/windows`): `single < readOperations <= single × profiles.read.maximumAttempts` (6, read from the churn schema). The digest still equals the oracle. Envelope 3/0 twice. The lane's evidence (351 vs 297 ops, 3/3 runs) is inside the bound.
- Note: `fdStable:true` for `leaf-transient` is only a fixture field that feeds the SQLite formula; it is never observed. It now means "the descriptor of the accepted pass was stable". A schema description saying so would prevent a misreading. Not blocking.

### consumers (MEDIUM, fail-open in owner admission)
The staged branch is `else if(typeof input?.input==="string"&&Array.isArray(input.projects)) addInput(...)`. Any other non-string closure item is silently dropped from owner admission. That covers a future tool or reference shape, `{input, projects, dependencies}`, and an empty `projects`.
- Scenario: a provider or tool reference shape grows by one key. Its group's files leave the owner's admitted inputs, and a change there no longer dirties the owner (stale serve).
- The house rule two branches above, for named inputs, refuses unknown shapes ("Closed selected source input required").
- Fix (`C/audit8-tools/staged/consumers`): accept exactly `{input:string, projects:[≥1]}` (2 keys, the shape `sharedReferenceOfV1` reads); anything else throws `Canonical command closure reference required: <item>`.
- Checked on `a8m` under the runner envelope (red-g `mrun.sh chain`, realpath TMPDIR):
  - admission 16/4, source-demand 22/8: every failure is in the lane's `allowed.txt`.
  - source-discovery 5/1: the identical failure with peeredit2's original copy ("Live Original process issuance required").
  - The new message fired 0 times.
- No new law for the refusal branch (it would need a hand-built owner-admission world). Worth adding in peeredit2's next batch.
- Correctness of the rest, reviewed:
  - A `{input, projects}` reference → `addInput(project, input)`. That is the same treatment as declared named inputs (no dependency edge), and `addInput` checks that the owner exists.
  - Excluding references from `commandResources` only affects `.ts` resources inside a provider group of a library owner's own compile set. That makes the owner more conservative, never less.
  - `expandedCommandInputsV1` is the old inline `expanded` moved out unchanged.

### allrun (MEDIUM, starvation moved to another class): the reduced upgrade takes every remaining core
New line: `if(pause===undefined&&memoryFits&&replaced!==undefined){cores=min(preferred,floor((ceiling-own+replaced.cpu)/cpuPerJob)); if(cores>=1||first) admit(max(1,cores),"reduced",need(minimumJobs))}`.

Upgrades are ordered before fresh requests, so a running build's upgrade takes every core left in the same evaluation.
- Evidence: the lane's own changed vector `followup-150`.
  - Before: upgrade `l34n…` waits `load`, fresh quiet request `blg4…` (1 job, 900 MB, no session) is admitted.
  - After: `l34n…` is admitted `reduced` with 6 jobs × 2 cpu, and `blg4…` waits `load`.
- Scenario: a 10 s no-op without an agent session (a human terminal, CI, any request whose waiter row carries no `session`) waits until a 400 s run ends. That is the mixed-long starvation the batch set out to fix, moved to the session-less class. The first-job-per-session rule rescues only requests that carry a session.
- Required change (decision `🟦️.ts`, its Python port `🔼️upgrade/🐍️oracle`, and a regeneration of the stored vectors):
  - Iterate `const ordered=orderHostWaitersV1(aged,active)` with its index.
  - In the reduced line, subtract `ordered.slice(at+1).filter(w=>w.upgrade!==true&&w.grow!==true&&w.request.elastic!==true).reduce((s,w)=>s+w.request.minimumJobs*w.request.cpuPerJob,0)` from the free cores, and keep `cores>=1||first`.
  - Law: `followup-150` must again admit `blg4…`, and the mixed-long replay must stay starvation-free. `sim/table.py --law` must stay PASS.
- Over-commit, checked and bounded:
  - Memory is never over-committed: `memoryFits` / `fits` run before every new admission, and reduced keeps `need(minimumJobs)` (the whole request's memory).
  - CPU over-commit is at most one job (`cpuPerJob` cores) per agent session that holds no other grant, because fresh requests have `minimumJobs:1` and upgrades take the reduced path with `max(1,cores)`.
  - With ~20 agent sessions on this machine, that is up to +20 jobs above `2 × cores`. Optional cap: allow the session exemption only while `own < ceiling + host.cores`.
- The `withHostCapacityV1` inherited-quiet re-read after growth is correct (same id check). Making the farm `elastic` with `observe:0` is accounted conservatively (its unrealized reservation plus the real `available`).

### resident (follow-up, not blocking): trust must hold over the interval, not at the moment of the query
`since()` now refuses while `trust!=="exact"`, but a cursor taken while trust was `verify` is answered `complete:true` once trust is back to `exact`.
- Example: a canary fence late under load gives `setTrust("verify","canary")` with no `lost()` entry. Then `recheck` ok → `exact`. Any changes the watcher dropped in that window are missing from a "complete" answer, and the graph membership is served stale.
- Required change (`🏡️resident/🗃️state/🟨️.mjs`): `let exactFrom=0;` and `setTrust:value=>{if(value==="exact"&&trust!=="exact")exactFrom=++seq;trust=value;}`, then in `since`: `if(trust!=="exact"||from.seq<exactFrom)return{complete:false,reason:"trust"}`.
- Also consider starting `trust` as `"none"` until the service's first `setTrust`. Today the gate opens (`gateOpen()`) before the watcher's trust is applied, with the default `"exact"`.
- Law: in verify mode, a cursor taken mid-window, then `setTrust("exact")`, must still give `since(cursor)` incomplete.

### scopedcut (no finding)
- `suspectsWithinV1` skips only pseudo owners that no project in the scope holds. The scope includes `@env` unless `skipEnvironment`, i.e. coverage-served.
- Such a question:
  - can dirty no in-scope project;
  - keeps its stale support (no `refresh` row), so the next verification that selects one of its holders asks it;
  - is never stamped, because `mergeVerifiedStampV1` stamps only `only`.
- Base already filtered `dirty` to `only`, so a changed shared question was deferred before too. Only the held ones were re-supported early.
- `refreshClosure` widens `only` to the dependency closure until a fixpoint. A project outside the slice whose targets an in-slice project consumes therefore joins the scope.
- Questions held only by `@env` under `skipEnvironment` changed nothing in base either: `changedHolders` ignores `@env` because it is not in `manifest.owners`.

### envelope (td-leases, notes, no change required)
- Masking surface:
  - A declared file born in-process passes even if the official runner stopped handing `SEMIO_SCRIPT_PROCESS_INVOCATION` to children.
  - Exposure is limited: ~11 declared files; the ~250 undeclared ones still fail loudly.
  - The equality check under a parent compares only `artifacts`/`command`, not `owner` or budget.
- Schema-first: the declarations duplicate the suite rows (owner, 300000, capabilities). A helper reading the row from `🗂️suites/🔣️.json` by id would remove the drift.
- Gate tooling: `mrun.sh env` (proc-run grants `artifacts`) will refuse the 6 td-capacity files that declare `{}`. They now pass bare, which is the mode difflaw tries first, so this is not a regression.

### behind (LOW)
- Labels are argv parts. `clean` removes whitespace only, and the schema pattern `^[^\n\r\t]{1,120}$` admits ESC and other C0 characters into other agents' progress lines. Use `^[^\p{Cc}]{1,120}$`.
- The same-user directory (`~/Library/Caches/semio/host-capacity`) means no cross-user exposure. The detail matches what `leases status` already shows.

### dashjourneys (follow-up)
- `scaled()` multiplies the 1000 ms interaction SLO and the 5000 ms load SLO by `load/cores` (clamped to 1..16).
- On the gate host (load 35-75 on 10 cores, scale 3.5-7.5) a 3-7x responsiveness regression passes. The scale also counts the load of the test's own cargo build.
- Minimum: print the applied scale, load and cores in the `wait_for` timeout text and in a passing run's log.
- Better: an explicit `SEMIO_TEST_TIME_SCALE` from the runner, or a CPU reference loop calibrated against a quiet-host baseline, instead of the load average.

## Method
- Diffs: `C/audit8-tools/diffs.py` (base→new per batch, plus live status base/new/drift; output `C/🗑️generated/audit8/*.diff`).
- Overlaps: a manifest cross-check found the 3 shared-base conflicts above. All other batches are `base` on live; the stacked ones are `drifted` by design.
- Mirror `a8m` (default subtrees, created 16:39, dropped 16:48):
  - corrected copies applied; parse scan 7 files, 0 bad;
  - psgate verify plus the PATH negative control;
  - windows twice through the envelope;
  - consumers suites under the chain envelope, compared with peeredit2's copy.
- Not run: product builds (none needed for the 3 copies: test files, plus one fail-closed branch in owner discovery that never fires on today's shapes).

## Files
- `C/audit8-tools/staged/{psgate,windows,consumers}/` (manifest, files, base, stage.py, verify.sh; windows' verify.sh points to `td-misc-tools/envrun.sh`).
- `C/audit8-tools/make_fixes.py`, `C/audit8-tools/diffs.py`.
- Evidence (sweepable): `C/🗑️generated/audit8/`.
