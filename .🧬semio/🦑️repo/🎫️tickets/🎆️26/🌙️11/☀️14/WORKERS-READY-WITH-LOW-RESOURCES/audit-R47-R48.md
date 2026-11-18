# Audit: R47 (former-root rebase, restart counting) and R48 (cut-and-continue)

Read-only audit. No build, no test run, no git write. Nothing was executed except read-only shell commands (`cat`, `grep`, `sed`, `shasum`). Every finding below comes from reading the code; "not run" marks what would need a run to confirm.

B = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`
T = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`

Severity: must-fix = wrong result (fresh/unchanged/done when not) or a broken claim the report relies on; should-fix = real defect or unbounded/unsafe-in-practice behaviour without a wrong verdict; note = minor.

## Summary

Must-fix:
1. A1: the synchronous fact checks used for project-record reuse and record writes are not rebased (`B/👁️observation/🟨️.mjs:132-141, 842-844`). A record written before a move is validated against the old absolute path. If the old tree still exists with unchanged files, a record can be reused for a live tree that was edited after the move.
2. A2: the former-root table is process-wide and not per workspace (`B/👁️observation/🏚️relocation/🟨️.mjs:9-12`). A generation recorded under the root of another live workspace in the same process remaps that workspace's own rows to the other tree.

Should-fix: A3 (chained/nested former roots depend on insertion order), A4 (JSON-escaped roots on Windows never match), B1 (no progress criterion in the moving-tree follow), B2 (progress reset makes the 6-restart cap unbounded for reads over 10 min), C1 (cut restarts before the killed worker is gone), C5/C6 (start level is a constant, learned level is not stored on refusal), D1 (schema text says 9966 MB, value is 10240), D2 (landing originals missing under `🗑️generated`).

## Part A: R47 former-root rebase

### A.1 Path boundary and matching (answer: sound for the boundary, one sound-but-missed case)

`B/👁️observation/🏚️relocation/🟨️.mjs:2`:
`FOLLOW="(?=[/\"\\\\\\s,\\]\\}:]|$)",PRECEDE="(?<![\\p{L}\\p{N}._@+\\-/])"`

- `/Users/Shared/semio-store`, `semio_old`, `semio.bak`, `semio~x`, `semio@2`, `semioé` are not matched (`-`, `_`, `.`, `~`, `@`, letters are not in FOLLOW). Good.
- `/home/u/Users/Shared/semio` is not matched (the preceding letter `u` is in PRECEDE's excluded set). Good.
- Case: a former root that differs only in case (APFS is case-insensitive by default) is not matched. Safe miss: the rows stay stale and are re-verified. Note.
- NFC/NFD: a former root recorded in a different normal form is not matched. Safe miss. Note.
- Symlinks: rebase is text-only. The stat that follows is on the live path, so symlink targets resolve as the filesystem does. Not a wrong-fresh source. Note.
- Path segments: `..` is not normalized. A recorded `/Users/Shared/semio/../x` (a file outside the tree) becomes `/live/../x`, which points at a different outside file. Only happens if a recorded path is not normalized; no evidence that one is. Note.

### A.2 Where the rebase is applied (answer: most comparisons yes, the synchronous ones no)

Applied (fine): `observationsHold` rebases its rows first (`B/👁️observation/🟨️.mjs:1190-1191`: `observations = rebaseRowsV1(observations);`), so `check`, `consistentObservationsV1` and `settleClosureRowsV1` see live paths. Journal path mapper (`B/🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs:9-12`, `rebaseTextV1` before `startsWith(base+"/")`). Directory-stamp doubt (`B/🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs:81, 91, 100`). Closure settler (`B/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs:84` `relative`, `:134` the question handed to the deriver). Digests are not touched: `rebaseValueV1` changes only strings that contain a former root literal, and sha256 strings never do.

Not applied: see finding A1.

### A.3 FINDING A1 (must-fix): synchronous fact checks read recorded old-root paths

Code:

`B/👁️observation/🟨️.mjs:139` (inside `projectObservationHoldsV1`, line 132):
`const holds=row.kind==="file"?fileFactHolds(row.path,row.fact)&&JSON.stringify(facts.file(row.path)?.sha256??null)===...:current([row]);`

`B/👁️observation/🟨️.mjs:842-843` (`current`):
`function current(observations) { return consistentObservationsV1(observations)&&observations.every(({ kind, path, fact }) => kind === "file" ? fileFactHolds(path, fact) : ...`

These are used with the recorded rows unchanged, by:
- `:170-172` (in-process reuse of a previous record: `previous.observations.every(row=>projectObservationHoldsV1(row,parent))`, and `current(previous.observations)` when there is no scope),
- `:239-241`, `:399-401`, `:530` (memo and shared checks),
- `:954-955` (disk record reuse, `current(candidate.header.observations)`),
- `:982` (record write, `rows.every(row=>projectObservationHoldsV1(row,parent))` / `current(rows)`),
- `:1018` (receipt write, `!current(rows)`).

Failing scenario 1 (cost, certain from code): the tree moves from `/Users/Shared/semio` to a new root. A project record written before the move has `row.path` under the old root. `fileFactHolds` stats the old path, gets ENOENT, returns false. The record is not reused and the project is re-inferred. This is one recomputation per owner per relocation. It is not a wrong result, but R47 section 1 says rebasing covers the comparisons, and it does not cover this one.

Failing scenario 2 (wrong fresh, conditional): the old tree still exists on the machine (R47's own "old root exists again" question), and its files are unchanged. The live tree was edited after the move. `fileFactHolds` checks the old files, which still match. The record is reused for the live owner. The record key does not catch it: `B/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:145` builds `projectRecordKeyV1 = (head, root, json, ...) => [...head, root, json, ...]`, where `root` is the owner's root (appears to be workspace-relative; not checked further), so the absolute old root is not part of the key. Result: a stale project inference reported as valid for the live tree.

Fix direction: apply `rebaseRowV1` inside `projectObservationHoldsV1`, `current`, and `fileFactHolds` callers, or rebase rows once at the record-reading boundary (`readInferenceRecordOrMissV1`, `:954`).

Not run: scenario 2 needs a record written under the old root, the old root still present, and a live edit. Deduced from the code paths.

### A.4 FINDING A2 (must-fix): the former-root table is process-wide, not per workspace

`B/👁️observation/🏚️relocation/🟨️.mjs:1, 9-12`:
`const table=new Map();` ... `table.set(former,{live,pattern:...});return true;`

- `noteFormerRootV1` refuses only a root nested inside the current `live` (`:10`: `nested(former,live)`). It does not refuse a former root that equals the live root of another workspace.
- `table.set` overwrites: the same former root taught with a different live replaces the earlier mapping for all callers.
- `adoptRecordedRootsV1` is called on every index open, for each workspace (`B/🕸️graph/🕰️generations/🟨️.mjs:242, 249`), and the closure settlers keep up to 16 workspaces at once (`B/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs:64`, `WORKSPACE_SETTLERS_V1=16`).

Failing scenario: one service process serves W1 at `/Users/Shared/semio` and W2 at `/Users/ueli/Documents/semio`. A generation for W2 was made on another machine under `/Users/Shared/semio` and is adopted. The table gets `/Users/Shared/semio -> W2`. Now W1 is read. Its `adoptRecordedRootsV1` call is a no-op for its own root (equal to live, not inserted), but the table still maps every W1 row under `/Users/Shared/semio` to W2. A file edited in W1 is compared with the unchanged W2 file, holds, and a stale fact is reported as fresh. Not run; needs two workspaces in one process.

Fix direction: scope the table by workspace (key the mapping by `(workspaceRoot, former)`), or refuse a former root equal to any live root the process serves, and refuse an overwrite with a different live.

### A.5 FINDING A3 (should-fix): nested and chained former roots depend on insertion order

`B/👁️observation/🏚️relocation/🟨️.mjs:22-26`:
`for(const[former,{live,pattern}]of table)if(out.includes(former))out=out.replace(pattern,()=>live);`

One pass in insertion order, no fixpoint. `noteFormerRootV1` (`:10`) does not refuse former-vs-former nesting or chaining.

Example (reasoned from the code, not run): table `[B->C, A->B]`, text `/A/x`. The B pattern does not match. The A pattern gives `/B/x`, which is not rebased again. Result `/B/x` (the intermediate root). The reverse insertion order gives `/C/x`. Nested former roots (`A` and `A/sub` both taught) behave the same way: `A/sub/x` maps to `live/sub/x` or `live/x` depending on order. If the intermediate root still exists, the row is compared with a wrong file.

Fix direction: refuse former roots that nest or chain (an intermediate root that is also a former or live root), or sort by length descending and rebase longest-first.

### A.6 FINDING A4 (should-fix on Windows): JSON-escaped roots never match the row text

`B/👁️observation/🏚️relocation/🟨️.mjs:56, 59`:
`const hit=FIRST.exec(row.path);` ... `const root=JSON.parse('"'+text+'"');`

The root is decoded from JSON (so `C:\\Users\\x` becomes `C:\Users\x`), but `row.path` for closure and registry rows is the raw JSON array text, in which the same backslash is written twice. `rebaseTextV1` (`:24`) searches the raw text for the decoded root, so it never matches. `rebaseRowV1` (`:39`, `rebaseTextV1(row.path)`) therefore leaves closure and registry question paths on the old root. Those questions are handed to the deriver through `B/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs:134` (`derive(rebaseRowV1(row),...)`). The fact side (`rebaseValueV1` on the parsed `fact`) works, because parsed strings are unescaped. Consequence on native Windows: closure questions keep the old root (failed or re-derived), silently. Not run; cross-platform rule (CLAUDE.md) applies.

Fix direction: match against the JSON-escaped form of the former root (`JSON.stringify(root).slice(1,-1)`), or decode the row path before matching.

### A.7 Other answers

- "Path legitimately outside the tree": only prefixes with a separator boundary are rebased (A.1). Outside paths that do not start with a former root are untouched.
- "Old root exists again": path-based rows are rebased regardless of whether the old root exists, so they check the live tree. The old root matters only for the un-rebased synchronous checks in A1, which is where the wrong-fresh risk is.
- Journal mapper (`B/🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs:9-12, 93-96`): an unresolved or symlinked root prefix gives `undefined`, and `pathLevel` (`:93`, `path===undefined?1`) treats it as doubtful. Safe.
- `stableDirectories` (`B/🕸️graph/🕰️host/🟦️.ts:133-134`) are built from live paths (`join(root, ...)`), and `observationsHold` compares them with rebased paths. Consistent.
- Environment rows (`holdsByStat`, `B/🕸️graph/🕰️host/🟦️.ts:63-66`) are workspace-relative and do not go through the rebase. Consistent.
- Directory stamps saved under live paths: consistent (`classify` input is rebased, `B/🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs:81, 84`).

## Part B: R47 restart counting for moved stamps

Code: `B/🧷️format-stamps/🔄️reexec/🟨️.mjs:38` (`followed`, `attempt`) and `:47` (environment carried to the next process).

Schema: `B/🧷️format-stamps/🧬️schema/🔣️.json:73-74` (`maximumReexecutions: 6`, `progressMilliseconds: 600000`) and `:103` (`movedPatienceMilliseconds: 1800000`).

### B.1 FINDING B1 (should-fix): a moving tree can loop for the full 30 minutes with no progress test

`:38`: `followed = signature!=="" && LIVE defined && signature!==LIVE && now()-since < movedPatienceMilliseconds`.

Only the immediately previous restart's signature is compared. A tree that keeps moving (the report's own premise, "A live tree always moves") gives a new signature on every restart, so each restart is followed and not counted. Nothing checks that a read got further than the last one. Two cases:
- Monotone moves: about 100 restarts of about 17 s each in 30 minutes, every read interrupted before it ends.
- Flapping A->B->A->B: each signature differs from the previous one, so the cap never counts. Same result.

The wall-time cap does hold. `since` is carried in the environment (`:47`) and checked at each restart (`:38`). It is checked only at restart boundaries, so a running read is not stopped by it. Patience answer: yes, wall time, anchored at the first restart of the chain (not the first moved restart; note).

Fix direction: count a follow when the read did not complete a stage (for example, no new published generation or owner), or cap the number of followed restarts too.

### B.2 FINDING B2 (should-fix): the progress reset makes the 6-restart cap unbounded for reads over 10 minutes

`:38`: `previous = ranMilliseconds>=protocol.progressMilliseconds ? 0 : parseInt(env REEXEC)`, with `progressMilliseconds: 600000` (`B/🧷️format-stamps/🧬️schema/🔣️.json:74`). The restarted process's `ranMilliseconds` is its uptime (`:32`).

Failing scenario: a read that runs 11 minutes and yields with the same moved values each time. `previous` is 0 on every restart, `attempt` stays 0, and the cap of 6 never triggers. Same-value restarts are not followed, so patience does not apply either. The loop has no wall-time bound.

This matters because R47 section 5 measured reads of 835 s and 703 s, which are above the 600 s threshold. The report's claim "a real loop is still bounded at 6" is true only for reads under 10 minutes. Not run; the trigger needs an identical signature on a long read.

Fix direction: do not reset the count on progress for restarts whose signature equals the previous one, or bound the total wall time of the chain.

### B.3 Note: `since` anchoring

`:38` and `:47`: `since` is the first restart of any origin (edit or load), not the first moved restart. Harmless (the cap is then slightly stricter), but the report says "patience counts from the first moved stamps".

## Part C: R48 cut-and-continue

### C.1 Owner records after an interrupted attempt (answer: no half-written record is trusted)

- Writes are atomic: `B/👁️observation/🟨️.mjs:927-929` (`importProjectRecordV1`) and `:985-986` (record write) write `${cachePath}.${pid}.${started}.tmp` with `flag:"wx"`, then `renameOverBusyFileSyncV1(temporary, cachePath)`. A killed writer leaves only a `.tmp` file, never a torn file at `cachePath`.
- Reuse is validated, not trusted by name: `:170-172`, `:954-955` check every recorded observation against the tree (see A1 for the un-rebased caveat). The key includes the project json and head digests (`B/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:145`).
- Racy-timestamp guard: `:982` refuses to publish a record whose source mtime/ctime is within `RACY_MS` of its start.
- Note: orphan `*.tmp` files from killed writers accumulate. Nothing sweeps them.
- Note: the version marker (`B/🪪️graph-state/📦️project-records/🟦️.ts:78`) is written non-atomically, but it is read back and compared with the full version string (`:115`). A torn marker never equals the full string, so it fails safe.

### C.2 Kept records keyed by input digest (answer: yes, through the observations)

A record is reused only if its key matches (`projectRecordKeyV1`, json and head included) and its observations hold. An edit after the record was written changes the observed file (size, mtime, ctime, inode, or sha256), so the record is not reused. Subject to A1 for the sync path.

### C.3 FINDING C1 (should-fix): the next worker starts before the killed one is gone

`B/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs:40`: `finish()` calls `worker.terminate?.()` and does not wait for it. The last statement, `done(value)`, rejects the attempt immediately. The run loop then starts the next attempt (`:85` `runWorker`, `:90` `continue`), which spawns a new worker.

`B/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs:38-47`: `terminate()` sends SIGINT, SIGKILL after `SLICE_GRACE_MILLISECONDS_V1=2000` (`🧹️reap/🟨️.mjs:7`), and resolves only when the process is gone. The old process can stay alive for the grace period and longer if it is blocked in a synchronous syscall (R47 section 5 measured 1.5 to 8.5 s blocks).

The pool's own comment (`:1`) says "loads of one workspace root run one after the other (they share one fact store)". The cut path breaks that invariant during the overlap. Record files are safe (C.1). Other shared writes (the node store, journal, stamp files) were not audited here. Not run.

Fix direction: await the `gone` promise before starting the next attempt (the reap already exists: `reaped(load)` in `finish`), at the cost of up to the grace period per cut.

### C.4 Step 6 (answer: no infinite loop; count-bounded, no wall-time bound)

- Plan cut: `graphCutAfterV1` (`B/🧠️memory/✂️cut/🟨️.mjs:14-16`) returns only `next>level`, and `:90` sets `level=error.cutTo` with `restarts--`. Strictly increasing, at most 6 cuts.
- Last level: `graphCutAfterV1` returns `undefined`, so the judge fails with `plan-memory` (`:52`), and the run loop rethrows it (`:94`, no branch matches).
- Owner hand-off: at most `heavyHandoffs: 3` (`:91`).
- Exit restarts: bounded by `sliceRestartMaximum` (`:95`).
- Not bounded: total wall time over all cuts. Each cut repeats the generation read (about a minute per R48). Note.

### C.5 FINDING C2 (should-fix): the learned level is not stored on refusal, so every read re-climbs

`:75`: `let level = learnedLevel ?? min(levels, graphCutStartLevelV1(...))`. `:86`: `learnedLevel = graphCutLearnedV1(...)` runs only on success. A load that ends in `plan-memory` at level 6 leaves `learnedLevel` as it was, so the next read starts at the start level again and repeats up to five cut restarts, each with partial work. Also, `graphCutLearnedV1` (`✂️cut/🟨️.mjs:37-39`) relaxes one level after any load under half the limit, so the level oscillates between reads (a cut, then an uncut run that overshoots). Note-level cost, not a correctness issue.

### C.6 FINDING C3 (should-fix): the peak decision is measured in flight, but the start level is a constant

In flight (fine): `judge` uses the measured tree (`:44-53`, `observe(worker.pid)` at `:54`); the verdict is `planGuardVerdictV1` (`B/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs:54-61`).

Start level (a guess): `B/🧠️memory/✂️cut/🟨️.mjs:22` `Math.ceil(Math.log2(policy.unboundedMegabytes/limitMegabytes))` with `unboundedMegabytes: 10240` (`B/🧠️memory/✂️cut/🧬️schema/🔣️.json:13`), a single figure taken from one 16 GB Mac. Not measured on the current host or load. Note the description in the same schema (`:7`) says "9966 MB" for the same field (see D1).

Learned peak mixes sources: `:54` `peak=(entry.peakMegabytes=Math.max(entry.peakMegabytes??0,grown))`, where `grown` is the headroom's growth (`headroom().currentBytes - base`), not the tree. The learned peak can be inflated by unrelated memory growth in the service. Conservative (relaxes less), but it is not the "measured footprint" the report claims.

Refusal when availability is unknown: `planGuardVerdictV1` returns `refuse, "unknown"` when `availableMegabytes===undefined` (`🧱️limit/🟨️.mjs:59`). A host that cannot report availability cuts every load over the limit. Conservative. Note.

### C.7 Other R48 checks

- Restart counting in cut: `restarts--` plus the `for` increment nets to zero (`:90-93`), as intended.
- The cut environment is set only in the child environment (`:38`, `graphCutEnvironmentV1`), not in `process.env`, so a later load does not inherit it. Fine.
- Worker wave: `nextOwnerWaveV1` (`B/🕸️graph/🕰️generations/🟨️.mjs:46-48`) and the in-process hint (`:346`, `:366`) respect `SEMIO_SLICE_WAVE_BASIS_POINTS`. The hint is in-process only, so it cannot carry an uncut size into a cut process. Fine.
- Plan-cut dispatch from the worker (`:65`) uses `error.values?.megabytes`; a missing value gives NaN and one level per restart. Bounded. Fine.
- Farm and concurrency (`B/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:97-99`, `B/🧠️memory/🪜️plan-stage/🟨️.mjs:44-48`): the cap is applied in both places, and `planFarmWorkersV1` returns the cut when there is no plan limit. Fine.

### C.8 FINDING D1 (should-fix): schema documents a different value than the constant

`B/🧠️memory/✂️cut/🧬️schema/🔣️.json:7` (description): "unboundedMegabytes ... (9966 MB, ticket ...)". `:13` value: `"unboundedMegabytes": 10240`. The schema is the single source (`✂️cut/🟨️.mjs:3-5`), so the documented measurement and the used constant disagree. Fix the value or the text, and say which measurement it comes from.

## Part D: repo rules that matter for correctness

- Comments inside definitions: none found in the changed files (`✂️cut/🟨️.mjs`, `👁️observation/🏚️relocation/🟨️.mjs`, `slice-pool/🟨️.mjs`, `🧷️format-stamps/🔄️reexec/🟨️.mjs`). Fine.
- Docstrings start with an emoji: all `/** ... */` docstrings in the four files above start with an emoji. Fine.
- Texts in English and German: `host-slice-owners-cut` and `host-slice-owners-handoff` exist in both languages (`B/🏡️resident/🧬️schema/🔣️.json:1970-1971` en, `:2052-2053` de). Fine.
- English-only internal errors: `new Error("Slice load is cut smaller")` and `"Slice plan is cut smaller"` (`slice-pool/🟨️.mjs:52, 65`) are not rendered to users (the run loop catches them). Note.
- Compatibility or migration code: none added. `plan-memory` stays as the last-level refusal, which is the intended end of the ladder. Fine.
- FINDING D2 (should-fix, process): the saved originals the request points to, `T/🗑️generated/R47-landing-*` and `T/🗑️generated/R48-landing-*`, do not exist. `T/🗑️generated/` has no R47 or R48 directory. `T/landings.md` (lines 3070-3200) lists the landed files with digests, and `T/🧰️scripts/R47/land.py` and `T/🧰️scripts/R48/land.py`, `land2.py` exist. Without originals, `land.py --rollback` (R48 section 5) cannot restore the previous files.

### Landed-versus-live digest check (done)

Live file, first 12 hex of sha256, compared with the "after" digest in `T/landings.md`:

| file (under B) | live | landings.md | match |
|---|---|---|---|
| `👁️observation/🏚️relocation/🟨️.mjs` | 71727d3d8a2a | 71727d3d8a2a (R47 landing and landing 2) | yes |
| `👁️observation/🟨️.mjs` | f14b5a369bcc | f14b5a369bcc (R47 landing 2) | yes |
| `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` | 2ae5856b0caf | 2ae5856b0caf (R48 follow-up) | yes |
| `🧠️memory/✂️cut/🟨️.mjs` | be23720e029e | be23720e029e (R48 landing) | yes |
| `🧠️memory/✂️cut/🧬️schema/🔣️.json` | d18ad8229077 | d18ad8229077 (R48 follow-up) | yes |
| `🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs` | f93f43c12428 | f93f43c12428 | yes |
| `🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs` | cae36217dbec | cae36217dbec (R47 landing 2) | yes |
| `🧷️format-stamps/🔄️reexec/🟨️.mjs` | 6673a4a2044f | 6673a4a2044f | yes |
| `🧷️format-stamps/🧬️schema/🔣️.json` | 909e1be7bffe | 909e1be7bffe | yes |
| `🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs` | 6fa7807d0339 | 6fa7807d0339 | yes |
| `🕸️graph/🕰️generations/🟨️.mjs` | e75cbb2c84f8 | e75cbb2c84f8 | yes |

The R48 hunk entries (`plan-stage`, `project-nodes`, the tests) are listed by hunk id, not file digest, so they cannot be compared this way. Result: no file was changed after its last landing, but the originals needed for a rollback do not exist (D2).

## Not verified

- Nothing was run. A1, A2, A4, B2 and C1 are from code reading and need a targeted run to confirm.
- Windows and Linux were not exercised.
- The owner-pool (`project-nodes/🌐️owner-pool`) and the fleet hand-off (`fleetReadV1`) were not audited; R48 states the hand-off is unverified end to end.
- Records written by `importProjectRecordV1` from the fleet were checked only for atomicity.
