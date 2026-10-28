# result-audit9 (lane audit9, prefix a9, wave 1027b): pre-landing adversarial review of the staged G46 queue

STATUS 18:03 FINAL. 13 batches reviewed base→new against the live code. Verdicts: 9 LAND (2 of them with a landing order), 2 FIX FIRST with a corrected audit9 copy (`multiwriter`, `jloss`), 1 FIX FIRST that needs a lane change (`hold`), 1 HOLD because it is stacked on `hold` (`lastrun`). Both corrected copies are verified on mirror `a9m`, which is now dropped. Nothing live was touched. No official run, no ticket tool, and no process of mine is still running.

## Verdicts

| batch | verdict | one-line reason |
|---|---|---|
| builderlock `multiwriter` | **FIX FIRST → audit9 copy** | The merge validation is asymmetric. It refuses a peer's stub change, but merges this writer's own stub changes (an added, removed or re-targeted project) onto a head whose peer fragments were computed against the old stubs. The merged graph matches no serial order and is never re-detected (§multiwriter). |
| jloss `jloss` | **FIX FIRST → audit9 copy** | The in-process replay after `UserDropped` does not check that the volume keeps a persistent FSEvents history (device UUID non-empty and unchanged). The supervisor's `--since` restart already requires that. Without it, a no-history volume turns a declared loss into a silent miss (§jloss). |
| quiet16 `hold` | **FIX FIRST (lane change)** | A quiet grant now runs at once with the FULL request's jobs and only the quiet reservation booked. Its growth never blocks anything. Busy runs that were forecast quiet therefore over-commit memory whenever the growth has to wait. Base blocked before the body (§hold). |
| quiet16 `lastrun` | HOLD (stacked on `hold`) | Sound on its own. With `hold` it makes nearly every run of a class quiet, which widens the `hold` over-commit to the common agent edit loop. |
| farmid `authority` | LAND, after `farmfree` | The worker adopts the parent's sealed load rows. Every shared fact is conflict-checked, and the farm opens only after the parent has inferred `minimumProjects` itself, so the parent's authority is already sealed. LOW follow-up in §authority. |
| td-graph `demand` | LAND | Test-only, 4 files. The fixture text `Ordinary command Source required` exists nowhere in live code. The new text is the test's own gate-port stub, so the law now proves that the refused read was reached. The `✏️s/🧑‍💻dev` rows point at an existing project. |
| agents16 `allrun` | LAND, order 1 | audit8's required change is in: a reduced upgrade sets aside the minimum cores of every later fresh or quiet waiter. The first-job exemption is capped at `own < ceiling + cores`, and `elastic` never makes another request wait. |
| agents16 `farmfree` | LAND, order 2 (before farmid `authority`) | Only the law changes now (`contracts:{}`). Its manifest row for `🏊️project-farm/🟨️.mjs` is base = new = live f1f2918e. If `authority` lands first, `farmfree` reports `drifted` on that row. |
| agents16 `behind` | LAND, order 3 | Labels use `^[^\p{Cc}]{1,120}$` and the door strips control characters (audit8 §behind fixed). |
| edit30 `freshmtime` | LAND | It cannot hide a real change. Cargo checks sources against the dep-info mtime, which Cargo pins to `invoked.timestamp`, and a restore never touches that. Stamping outputs at restore time only makes dependents compare as a real compile would (§freshmtime). |
| playdemo `vitestext` | LAND | Two test configs externalise library modules. The pattern matches Vite's forward-slash ids on every platform. Both suites were run by the lane. |
| playdemo `wasmlock` | LAND (+ LOW) | The lock now also carries the workspace members that the stale lock missed (`--locked` needs them). The pre-compile refusal is correct. LOW: a missing lockfile throws a raw ENOENT instead of the bilingual refusal. |
| gofix `goyaml` | LAND, own group | Mirrors its siblings: explicit `GOWORK`, and the module is listed in `go.work`. Its registry row is the library's own output. A membership change, so keep it out of graph groups (as the lane says). |

FOR coordinator: land the audit9 copies INSTEAD of the lane copies. Same base hashes; `check` = `base` on live 18:03; the lane's mode-keeping stage.py is kept (`test_stage_mode.py` OK).
- `python3 -I C/audit9-tools/staged/multiwriter/stage.py apply --root <root>` · verify `sh C/audit9-tools/staged/multiwriter/verify.sh <root>` (mirror a9m: 64341 B < 65536, scan 12/0 bad, 21 + 174 + 1 + 2 pass, `VERIFY OK`) · revert `… stage.py revert --root <root>`.
- `python3 -I C/audit9-tools/staged/jloss/stage.py apply --root <root>` · verify `sh C/audit9-tools/staged/jloss/verify.sh <mirror root>` · revert `… stage.py revert --root <root>`.
  - On a8m-style sealed mirrors, the QoS law and the journal "detached process" law fail only because `ps` gets EPERM under `sandbox-exec`. Rerun those two files with `mirror.py run <m> --no-seal`: a9m gave 11/0, guard clean.
  - Everything else PASS: unit tests incl. the new one, 6 warning-free targets, replay law, the 3 negative controls.
  - Same NEEDS A JOURNAL RESTART line as the lane (TERM the journal pid after apply).
- Rebuild both copies from the lanes' current staged bytes (anchored; exit 1 = anchor lost): `python3 -I C/audit9-tools/make_fixes.py`.

FOR coordinator (order):
1. agents16 `allrun` → `farmfree` → `behind`.
2. `farmfree` before farmid `authority` (shared row `🏊️project-farm/🟨️.mjs`).
3. audit9 `multiwriter` and quiet16 `hold` both edit `🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts` (base fd11be01). `hold` is held anyway, so its re-cut must start from multiwriter's version.
4. `wasmlock` together with `authority` (one re-inference, as playdemo says). Re-derive `✏️s/Cargo.lock` with `C/playdemo-tools/lock.py` if any `✏️s` manifest moved.

FOR quiet16 / coordinator: HOLD `hold` + `lastrun` until the §hold change lands.

## Findings with scenarios and fixes

### multiwriter (MEDIUM-HIGH, silent stale graph): the stub rule must be symmetric
`mergeGenerationV1` refuses `structure` when the HEAD changed a stub (`stubOf(head)≠stubOf(base)`), because "every computation reads all stubs". That is the lane's own premise: `compute` gets `spec.stubs` of all owners, and `shapeComputeResultV1` resolves edges against them. The writer's own stub changes are never compared with base. These include an added owner (base and head lack it), a removed owner, and a changed `targets`/`kind`/`config`/`nativeRoot`/`tag`. They are merged onto a head whose THEIR fragments were computed against the OLD stubs.

Scenario: writer W holds a custody claim on {X}, reads base B, then plans (tens of seconds).
- Meanwhile peer P holds the disjoint {Y}, rewrites Y so that it imports package `n`, publishes H and releases.
- A config for the new project N appears after P's membership walk, so P was not structural.
- W's plan finds N added, so W is structural. `extend` is granted, because no claim is HELD any more.
- W computes on B. Its re-edge set comes from base-Y, which did not import `n`, so W does not touch Y. W loses the swap and merges.
- Merge result: N exists, and P's Y still resolves `n` as external.
- In either serial order Y→N would be an edge: if P runs after W it sees N; if W runs after P, its added-project re-edge covers new-Y.
- No later verification brings it back. Y's and N's facts hold, and the membership is unchanged, so the edge stays wrong until Y or N is touched.
- Same class of bug: a removal (P's new edge into R points at nothing) and `kind→unavailable` (P's edges into X computed while X was a project).

Fix (`C/audit9-tools/staged/multiwriter`): after OURS is known, `if(theirs.size)for(const name of sorted(ours))if(stubOf(mine.owners[name])!==stubOf(base.owners[name]))return{conflict:"structure",owner:name};`
- The same rule is in the Python oracle, plus a sentence each in the docstrings and the schema description.
- With no peer owner changed, the merge is still allowed (a metadata-only head cannot be stale against the new stubs).
- New law: added, removed and re-targeted projects each refuse naming the owner, and merge onto a head that only moved `@env` rows.
  - RED with the lane's merge + oracle (`"conflict": null` received).
  - GREEN with the copy: merge 6/0, custody 9/0.
  - 600 seeded worlds equal the oracle, and every outcome kind still occurs.
- Full verify: 21 + 174 + 1 + 2, `VERIFY OK`.

Reviewed and sound:
- OCC re-validation is cumulative from the base against each newer head.
- Row keys include the answer (`[kind,path,fact]`), so the OUR-row check is exact.
- `@env` and derived supports are united, and a stale extra support only re-asks.
- The stamp `min(atMs)` is conservative, because `coverageOfV1` serves only requests older than `atMs`.
- Retirement:
  - It is gated by `alone()`, which makes the claim structural so newcomers wait.
  - Both production `withBuilder` paths take the custody; no builder takes the lease path.
  - The 10-minute grace stays.
- Custody:
  - A held claim never waits, so there is no deadlock.
  - Dead claims are found by lease probe.
  - Re-edged owners outside a claim are still caught by OURS∩THEIRS in the merge.

Not blocking:
1. `global` is the delta's `other` edges (`shapeComputeResultV1`), so it depends on the slice. Expect frequent `global` refusals: sound, no gain. This is the lane's Open 1, which proof run (2) decides.
2. `extend` checks only HELD claims, so a widening can overtake an earlier waiting claim. That is a fairness issue, not a soundness issue.
3. An unreadable claim document (for example another implementation's schema) counts as no claim. `requireCurrentGraphModuleV1` covers the landing transition.

### jloss (MEDIUM, a declared loss can become a silent miss)
`resync()` replays `since` the oldest of the trailing cursors on every `UserDropped` that comes without `KernelDropped`. FSEvents can replay only from a persistent per-volume history. `FSEventsCopyUUIDForDevice` returns NULL on read-only volumes and on volumes with `.fseventsd/no_log`, and the UUID changes when the history is reset.
- The supervisor path already gates `--since` on `expected == uuid && !uuid.is_empty()`.
- The in-process replay does not.
- So on a checkout on such a volume (external drive, `no_log`), a drop is answered by a replay that cannot hold the dropped events. Base said `lost`; the batch says nothing, and the memo/vouch/plan cursors serve stale.

Fix (`C/audit9-tools/staged/jloss`):
- `State.uuid` is the UUID at stream start.
- New `fsevent::replayable(started, now)` = non-empty and equal.
- `resync` takes the existing loss branch (`lost("user-dropped","")` + continue from now) when it is false.
- The pin is recomputed (sha256 bcd550c2…, 53901 B).
- New Rust unit `drops_replay_only_from_the_persistent_history_the_stream_started_on`.
- On a9m: unit tests PASS, 6 warning-free targets PASS, the stalled replay law PASS (the live volume has a history, so replay still happens), all 3 negative controls bite, and supervisor, journal and provision laws PASS. The two `ps` laws are 11/0 unsealed.
- Not demonstrated on a real `no_log` volume (that would need a scratch disk image). The evidence is the API contract plus the batch's own supervisor rule.

Hardening, not blocking: the replay anchor is "oldest of the last 8 callbacks". If fseventsd ever delivered events newer than the dropped ones across more than 8 callbacks before the marker, the replay would start too late. A time anchor (`FSEventsGetLastEventIdForDeviceBeforeTime(dev, lastCleanCallbackTime − margin)`) would bound that independently of the callback rate.

### hold (HIGH on a declared 16 GB host, MEDIUM on the gate host): a quiet forecast no longer bounds what a busy run uses
- Base (`withHostCapacityV1`, inherited branch): a quiet grant called `requestHostCapacityUpgradeV1` and WAITED in the fair queue before the body ran. A run that does work was therefore admitted at full size, or waited.
- `hold`: the body runs at once with `plan.jobs` = the full request's jobs, booking only the quiet reservation. Growth starts only when the heartbeat sees the tree outgrow it. "The run goes on (nothing pauses)", and Bazel actions under a reused plan have no executor point (lane §Risks).
- `lastrun` then forecasts quiet whenever the last run of the class executed nothing, or executed work within 30 min. That covers almost every agent edit-build loop.

Scenario (h16, 9 GB foreign, ~5 GB budget; `maximumQuiet` 4):
- Three agents each run a no-op, then edit and build print/course/dash. All three are quiet-admitted at ~0.5 GB, because the trees have not grown yet when the next one is admitted.
- All three run at full width, with real footprints of 2.9 + 3.6 + 3.7 GB.
- Their growth requests wait for memory, but nothing waits on them. The host is about 5 GB over budget for the whole compile, which is a swap storm. Base would have serialised them at the growth.
- The lane's simulation reports "real above committed ≤ 526 MB". That only holds while every growth is granted at once. The table has no scenario in which two outgrown runs compete for the same memory.

Required (lane), one of these, with a law for each:
- (a) Wire the executor point the guide names ("the first Bazel action that really runs"). It must block on the growth like the graph rebuild point, so that a quiet run that starts work waits there with its original arrival.
- (b) Have the door carry, for a quiet admission, the jobs whose memory fits the free memory now (not booked), so that a busy-after-quiet run's width is bounded by what the machine had when it started.

Law: on h16, two quiet-forecast runs that both turn busy at once never hold more than budget + one job beyond what is committed (sim scenario `hot-busy2`). Keep audit8's `followup-150` behaviour.

### freshmtime (no finding)
Cargo has two freshness checks:
- `CheckDepInfo`: every source in the dep-info compared with the mtime of the fingerprint dep-info file. Cargo sets that file's mtime to `invoked.timestamp`, which is written before rustc (the wrapper) starts.
- Dependency check: the max mtime of a dependency's outputs compared with the max mtime of this unit's outputs.

What the stamp does:
- Restore stamps only non-template outputs (rlib/rmeta/executables) with the restore time. That is ≥ `invoked.timestamp` of the unit and ≤ the start of every dependent.
- A source edit is always newer than `invoked.timestamp` of the unit that read it, so it is detected as before. No reference time moves later.
- The `.d` template was already written at restore time in base.
- Build-script runs are not rustc, so the wrapper never restores their `output` file.
- `utimesSync` can throw, but `serve()` failures fall back to a compile (`try{if(await serve())…}catch{}`).

### authority (follow-up, LOW)
`adoptInferenceAuthorityV1` clears the worker's own load rows and adopts the parent's. A worker-only load row (a fact the worker read at load that the parent never sealed) is dropped silently, so records the worker produces would not depend on that fact. Today the worker reads its session at `open` inside `observeInferenceSnapshotV1`, which has sealed already. Its load rows are bootstrap reads that the parent shares, so the exposure is theoretical. Suggested law: a worker with an extra load row reports it among `conflicts`.

### wasmlock (LOW)
- `refuseMismatches` reads the resolved lockfile without an existence check. An owner whose workspace has no `Cargo.lock` yet throws a raw ENOENT before the compile, where wasm-pack would previously have created the lock. Guard it with `existsSync`, or refuse bilingually.
- The lock also gains `semio-framework-graph-algorithms`, `os-store-engine-close/member-group` and `syn 3.0.9`. These follow from the current manifests (the live lock is stale for `--locked`), so they are not unrelated drift. Re-derive the lock if a manifest moves before landing.

## Method
- Diffs: `C/audit9-tools/diffs.py` (base→new per batch, plus live status base/new/drift; output `C/🗑️generated/audit9/*.diff`).
- Overlap cross-check of all 13 manifests: four shared rows (§order above). The stacked agents16/quiet16 rows match each other's new→base hashes.
- Mirror `a9m` (default subtrees, created 17:51, dropped at the end):
  - corrected multiwriter: RED proof with the lane's merge/oracle, then the full verify;
  - corrected jloss: verify.sh sealed, plus the two `ps` laws unsealed.
- Logs (sweepable): `C/🗑️generated/audit9/`.
- One live-tree GUARD line during the jloss verify: `🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts` changed in live while the mirror stayed untouched. That was a peer or gate edit, not mine (my run was sealed).

## Files
- `C/audit9-tools/staged/{multiwriter,jloss}/` (manifest, files, base, stage.py, verify.sh; lane copies plus the anchored edits).
- `C/audit9-tools/make_fixes.py`, `C/audit9-tools/diffs.py`.
