# 🧾 T3: test debt of wave 1122-dist (2026-11-24, 17:20-19:20)

Every file ran with `repo test-file`, one at a time, host load 40-73. No sub-agents, no restarts. Landings are in `T/landings.md` (four "T3" entries, with sha256 before and after). Logs are in `wave-1122-dist/🗑️generated/T3/`, scripts in `wave-1122-dist/T3-scripts/`. All locks are released.

## Results

| file | before | after (runs) | cause and fix |
|---|---|---|---|
| `B🌐️remote-execution/🚀️offload/🧪️tests/🌐️cluster` | 13/2 (kill, cancel) | **15/0** (run 4, 403 s, load 50-70) | **Cause:** the tests found the remote compiler by polling `ps -ax` for `.semio-offload/o`, with fixed 60 s and 90 s waits. Under load the poll missed it. BSD `ps` cuts the command column when there is no terminal.<br>**Fix (test only):** the workers get a clone of the test toolchain's sysroot. Its `bin/rustc` writes `compiler-<crate>.pid` into the action root, then `exec`s the real rustc. `--print sysroot` names the clone, so the worker offers exactly the same properties as before (checked). Kill and cancel now wait on that pid file with `stretchedBudgetMillisecondsV1`. Cancel asserts that the recorded pid dies. `reap` only kills a marker pid that is still `rustc`. When `fleetReady` fails, its message now shows the workers' last lines and the scheduler's last events.<br>**History:** runs 1-3 used *hard links*, and dyld sometimes resolved the binary to its `~/.rustup` path, which the sandbox hides. That gave `exit 134` ("Library not loaded"); I diagnosed it with a temporary `[DEBUG]` line, which I removed (byte-identical). APFS clones have their own inodes and fixed it. |
| unit-cache `🛰️fleet/🧪️tests` (main), "a refusal is re-checked as soon as the credential changes" | 2 of 5 failed ("uploads did not settle", K7) | **18/0, 18/0** (load 73, 63) | **Product bug:** a job put off while the credential was denied kept a random 7.5-22.5 s `notBeforeMs`, even for the successor drainer that had the right credential. Under load, that plus the takeover went past 60 s.<br>**Fix (schema-first):** the deferral records `SpoolJob.deferredBy` (the fleet identity). A drainer with another identity takes the job at once. A retry drops the field.<br>**Second bug, found by Ajv in the new test:** the deferral wrote a float `notBeforeMs`, which the closed schema refuses (`integer`). It is now rounded.<br>The test's 60 s settle waits are now stretched. |
| unit-cache `🛰️fleet/📤️uploader/🧪️tests` | 8/0 | **9/0** | New test: a job deferred under a denied credential is left alone by the same credential and uploaded at once by another. The job validates against the schema. |
| job-slots `🧵️job-slots/🧪️tests/🏁️finals` | 1 failure in 5 (O24) | **2/0** ×3 | **Cause:** `reservedMaximum` added up every extra local start and never released one, so it counted how many passes happened to land on extra slots (timing), not the concurrent reservation. It is now a set of the passes that hold a reservation.<br>The test also called `writeUnitPeakV1` without O18's `threads` argument, so its 6000 MB peaks were silently dropped. It now passes `offloadDeclaredThreadsV1(true)`. |
| offload `🚀️offload/🧪️tests/⏱️timeouts` | one load flake (9.4 s vs 4 s at load 73, O25) | **15/0** | The ride-through bound is now stretched. It is at most 8× = 32 s, still below the 60 s ride-through it has to tell apart. |
| `B🗄️cas/🧪️tests` | one load flake (the 40 ms admission test, O25) | **62/0** | The test now asserts that the refusal arrives before the hanging 500 ms collection ends (timer order) instead of `< 400 ms` wall time. I also removed three byte-identical duplicated tests: transient admission, released lease, and lease/507 (65 → 62). |
| offload `🚀️offload/🧪️tests/🏁️finals` (O20 "not run") | **3/2** | **5/0** ×2 | **Caused by this wave (O18):** the `.peak` record is now kept per thread count, and the test still called `writeUnitPeakV1` with 3 arguments. Nothing was written, so it read the 2048 MB default. The test now passes the declared threads. |
| driver `🏗️native-build/🧩️component-pass/🧪️tests` | O26: "cannot finish, SIGTERM reaches the runner"; 46/24 | **finished** in 911 s (load 50-65) | **No SIGTERM this time and no file split needed:** all 3 suites ran inside their stretched budgets. Results: reclaim 0/2, queued-cancellation 2/5, native-artifacts 49/24.<br>**None of the failures traces to O10-O27, with this evidence:** the tests and the failing code paths are byte-identical to commit `b4927d2c311` (11-21, before the wave), or the failure is a cascade.<br>The wave's own tests all pass: fleet-sized jobs, component jobs, fleet/local component jobs, p95 batches, and the hard-reserve refusal (which could not be isolated before). |
| O20/O21/O8/K4 "not run" files | not run | all pass | closure 3/0, eligible 4/0, failure 4/0, linker 3/0, gate 4/0, toolchain 4/0, determinism 7/0, `⌨️command/🧪️tests` 7/0, unit-cache `🔭️interface` 30/0, `🔑️key-stability` 5/0. |

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`

## Driver file: every failing test, one line each

**Reclaim:**
- `reclaim preserves Cargo's hard failure after 1 pass(es)` and `after 2 pass(es)`, each counted twice: once by the suite and once as `(unnamed)` from the isolated child.
  - Symptom: "reclaimed 0 file(s)".
  - Code: `🧩️component-pass/🟦️.ts:1117` puts every `passFiles` entry into `failedFiles`, and reclaim (`:868`) skips `failedFiles`.
  - Not this wave: these lines and the test are identical in `b4927d2c311`.

**Queued exclusive compiler cancellation:**
- Fails with "Live Original resource control issuance required". The test's control is not issued from its Original. Test-side, already in TI2.

**Unit graph compilation cancellation** (the root of the queued-cancellation suite):
- The abort reason "metadata cancellation" escapes and closes the process-wide Original.
- The test is unchanged since 11-21.
- Not attributed. Next step: find which driver promise rejects with `signal.reason` without a handler. Look first at the cancel path around `🧩️component-pass/🟦️.ts:780` and the new fleet/local admission calls at lines 534-537 and 1209-1222.
- Cascade from it: `original finite caller budget`, `unlaned same-profile Cargo jobs`, `(unnamed)`. They all fail with "Original process admission closed - first failure: metadata cancellation".

**Native-artifacts suite:**
- `a real build over wasm32-wasip2 components`: "Live Original resource control issuance required". Test-side, TI2.
- `jobs start as soon as their completions happen…`: `after:["job:tool","components:check"]` violates the `📋️prerequisites` schema `^job:` (dated 10-21). The test is stale against the contract, from before the wave. It also starts the cascade below.
- Cascade, all failing with "Finite job-only prerequisite graph required" / "admission closed":
  - `reserved nested compiler processes`
  - `exclusive compiler jobs`
  - `a component Cargo did not link`
  - `a lane job runs inside`
  - `a finalizer runs once`
  - `a blocked browser release`
  - `a job failing at once`
  - `shared restart status`
  - `queued exclusive` (second copy)
  - `unit graph cancellation` (second copy)
  - `pending stages and finalizers`
  - `original finite caller budget` (second copy)
  - `unlaned same-profile` (second copy)
  - the 4 `closure-scoped partial Cargo provenance` rows
- TI2's isolated runs add one real failure: `reserved nested…` expects `SEMIO_COMPILER_CAPACITY_HELD`, but the driver deletes it from job environments at `🧩️component-pass/🟦️.ts:677`. That code is unchanged since `b4927d2c311`.
- `refuses a repeated missing source witness before staging`: the isolated test did not end with its declared rejection. Not traced.
- `a failed cargo exit with no failed unit surfaces cargo's own stderr…`: it expects `cargo exit 101…` but gets "Native build failed: comp-a". With the stub in `stderr-exit` mode, comp-a is unlinked, so `missing` is not empty and the failure names the package. The failure rule (`:1140`) is the same as in `b4927d2c311` (`:1016`). Not this wave.

**Recommendation:** fix the two test-side roots: `jobs start…` against the `^job:` contract, and the control used by `a real build over wasm32`. Then fix the unit-graph cancellation leak. Together these should clear about 20 cascade failures. All of these are outside this wave's changes.

## Not done or open
- Item 5 did not cover W2's closures test (W-lanes hold `lock.worker-ready`), O15's Rust `cargo test` (it is not a `test-file`), or T1/T2b's roughly 175-225 never-run files.
- The cluster file needed 4 runs instead of 3: runs 1-3 diagnosed the hard-link fault, and run 4 is the only one on the final code.
