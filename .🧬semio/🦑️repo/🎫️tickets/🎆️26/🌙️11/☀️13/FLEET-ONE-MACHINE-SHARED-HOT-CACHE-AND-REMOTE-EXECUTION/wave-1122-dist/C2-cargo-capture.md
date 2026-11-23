# C2: why `repo cargo` sat in `source.capture` (diagnosis + fix)

## Verdict
`source.capture` is NOT the cargo workspace. It captures the **build tool's own source image** (the repo CLI's TypeScript import closure, `captureCompilerBuildSourceClosureV1` -> `captureNxAuthorizerSourcesV1`, entry = the build script). It is identical for every command and never looks at the invoked manifest, a lease, a lock or the graph. "Scope to the invoked manifest" does not apply; nothing in it waits on another holder (no lease/lock call exists in the walk).

What it waits on: a **serial full walk** of ~1.8k files (reads, ancestry, import scan), needed whenever the per-entry source memo (`~/Library/Caches/semio-repo/source-memo/<root>/<entry>.json`) is stale. Any peer edit inside the tool closure invalidates it (memos were rewritten 00:07, 01:46, 02:14, 02:25 today), so on this live tree most invocations pay the full walk, and under host load 35-70 each awaited syscall costs milliseconds: 136 s measured, far more under load 70 (P1: >= 12 min, killed; its "0 % CPU" is the idle wait of a latency-bound serial walk).

## Reproduction (foreground, `taskpolicy -b`, load 33-55)
`SEMIO_REPO_PROFILE=1 repo cargo --store=shared --configuration=prototype -- metadata --no-deps --format-version 1` in `T/wave-1122-dist/proof` (output in the stderr profile, scratch in `🗑️generated/C2-run1.*`): total 760 s.
| phase | start s | duration s |
|---|---|---|
| boot + door.plan/load | 0 | 36 |
| door.capabilities | 37 | 13 |
| door.script until capture | 50 | 42 |
| **source.capture (served: capture = memo stale, full walk)** | 93 | **136** |
| command stage before cargo (leases.open 2 s, store open/reservation 6 s, rustup verify 9 s, generation scope 286 s of which compiler native-body self 47 s) | 217 | ~250 |
| cargo workspace-body (`metadata --no-deps` itself, self time) | 177 (inner) | **213** |
No lease or fleet exclusive holder was involved in capture (`repo leases status` showed none awaited; the only 125-wide `lease-sweep`/`boot-recovery` rows are <1 s).

## Changes (LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`)
1. `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts`: the walk reads ahead. Every discovered import/companion/data path starts its (ancestry-ledgered, bytes-identical) read immediately through a counting semaphore (`SourceReadAhead.concurrency` = 24 open files, schema-declared); the single-threaded import scan consumes the results in the same DFS order, so manifest, digest and error lineage are unchanged. New `report` option: "memo absent or stale, walking the tool closure" and "walked N files, now <path>" every 16 files.
2. `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json`: `SourceReadAhead` constant (concurrency 24, reportFiles 16).
3. `LIB/🏗️build/📥️source/🔏️closure/🏗️compiler/🟦️.ts`: passes `report: profileProgressV1`.
4. `LIB/🏗️build/⏱️profile/🟨️.mjs` + `.d.mts`: `profileProgressV1(text)`; the front door heartbeat now prints `last phase started: source.capture (walked 640 files, now <path>)` (cleared by the next phase/mark; never part of profile rows).

## Tests
- `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️ancestry/🟦️.ts`: new law: a 61-file closure beside an unrelated 60-file directory with mode 000 (any touch would throw) is captured in < 10 s, manifest equals exactly the closure, sha256 of every source equals an independent `crypto` hash, progress reports present.
- `LIB/🏗️build/⏱️profile/🧪️tests/🟦️.ts`: progress text appears in `profileCurrentPhaseV1` and is cleared by the next marker.
(Results below.)

## Open
- A stale memo still means a full walk; an incremental memo (reuse digests of unchanged files by stat stamp) would make a one-file edit cost one file. Not done: needs a memo schema change in `🧊️memo`.
- Independent of capture: the command stage costs ~250 s before cargo starts (rustup generation scope 286 s, compiler native-body 47 s self) and `cargo metadata --no-deps` on the 13-package workspace has 213 s self time in `cargo.compiler.workspace-body` under load 35-55; not analysed here (a shared CARGO_HOME package-cache or build-dir wait is the first suspect; look for `Blocking waiting for file lock` with `lsof` during a run).

## Coordinator lead: orphaned wrapper pid 72808 (build-script RUN of quote, ignored SIGTERM)
It was mine: child of my `repo cargo` repro (bun 45878, sealed gen 464db2e13515a20d), orphaned when the repro was stopped. Cause, `unit-cache/🏗️buildscript/🟦️.ts` `runOffloadedV1`: SIGINT/SIGTERM were hooked with `process.once(abort)` only around the light-slot wait + offload; an abort during the light-slot wait threw, `catch{}` swallowed it, the once-handler was already consumed, and the code fell through to `acquireSlotV1(root,slots,{onWait})` (no signal, no deadline: the compile-slot queue is FIFO and waits forever by design) while holding the per-run claim `.lock`. So at host load 60-87 with all slots busy it sat 11 min with no child and one SIGTERM did nothing.
Fix (landed): handlers stay installed for the whole function and record the exit status (130/143); an abort skips the local slot wait and the script run and returns that status; the local slot wait receives the abort signal. The claim is released by the existing `finally`. Law (`🔓️slot-held`): with the compile slot and the light slot held, a SIGTERM to the run exits 143 within 15 s without running the script.
Not changed: the local compile-slot wait itself has no time bound (FIFO ticket queue, documented "a unit never fails for want of one"); a bound would be a policy decision (`policy.slot.maximumWaitMilliseconds` then run unslotted) for the O8 owner. `offloadBuildScriptRunV1` already carries the command-offload deadlines (O8).

## Test results
authorizers 🧭️ancestry 6 pass / 0 fail (new law incl.); profile 10 / 0; buildscript 🔓️slot-held 3 / 0 (new SIGTERM law). Not measured: wall-time gain of read-ahead on a stale memo (host load 55-87 made a before/after pair meaningless); first run of the ancestry file earlier failed only with 'Original Script deadline refused' (no SEMIO_LANGUAGE/load), passed on re-run.
