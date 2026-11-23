# Trinity and Jack runtime end to end (interim)

## How the s app hosts Trinity

- The browser "s app" for a plugin is a playground variant. Trinity declares two rows in `🌎️hub/🧩️compositions/🔱️trinity/🧩️component/📦️packages/🦀️rust/Cargo.toml`: `trinity-jack` (app `s.trinity.jack@1/*#editor`, react port 6054) and `trinity-rewriting` (app `s.trinity.rewriting@1/*#editor`, react port 6056). Both rows are in the generated catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` (lines 5449 and 5469) and the plugin is registered in `🔌️plugins.json`.
- Canonical launch: `repo run @semio-tech/framework-os-dev:dev-trinity-jack-react-prototype` (and `...dev-trinity-rewriting-react-prototype`). The targets are inferred at graph time from the playground rows, not committed in a `project.json`. Chain: `session-<variant>` then `<project>:component` wasm build, `prepare-`, `activate-`, `serve-` (Vite, `S_OS_PORT`).
- Verification markers: `data-semio-os-ready=<variant>` / `data-semio-os-error=<variant>` (FEM convention).

## Findings so far

1. `repo run` on the dev/prepare targets waits indefinitely on graph loading ("Reading the project graph slice", 21 of 600 after 4 minutes, behind another load). Native fallback used instead: `launch-native.ts` (copied from the FEM 2D ticket, generalized with `TRINITY_VARIANT`) issues the script process envelope and runs the owning script directly.
2. No Trinity component was ever published: `.cache/b/p/m/browser/prototype/modules/` held only `🏗️fem`.
3. `plugin trinity` (native `buildComponentClosureV1`) built `semio-native-transpile` (93 units, about 9 minutes under load) and then failed at the descriptor emitter capability check: `semio-plugin-describe: unknown command "capabilities"`. Root cause: `ensureDescriptorEmitterV1` only builds the emitter when its files are missing; the published tool `.cache/b/p/m/browser/prototype/tools/descriptor-emitter/win32` dates from 2026-11-18 and predates the `capabilities` command in `🛂️descriptor-emission/🦀️.rs`, and nothing invalidates it when its source changes. All three cached copies of `semio-plugin-describe.exe` are that stale build.
4. Attempted workaround: the stale directory was renamed aside and the emitter rebuilt through `semio-plugin-describe` `build --host-platform=win32` via the same launcher. It was restored to its original name afterwards because the rebuild could not finish (see Blocker). The plugin build path reads the emitter manifest before any ensure, so a missing directory fails with ENOENT instead of rebuilding.
5. Concurrent edits by other agents repeatedly broke script loading for a few minutes at a time (`Export named 'concurrentEditErrorOfV1' not found`, `Cannot find module './🔣️.json'`, `Cannot find module './🗄️analysis-store/🟨️.mjs'`); retrying after a short wait cleared each.

## State after the second session (disk freed to about 35 to 42 GB)

- The emitter rebuild through the new `ensureDescriptorEmitterV1` path ran: lease wait about 15 minutes, `synchronize` about 20 minutes, cargo compile of 578 artifacts about 25 minutes, then the cargo-provenance verification of about 2.9 GB of build records, which was still reading (437 provenance steps logged) when I stopped polling after roughly 2 hours. The emitter publication directory is therefore still empty and the trinity component has not been built, served or driven.
- The build is still running in the background (`launch-native.ts` bun process, log `🗑️generated/plugin-build7.log` in this ticket folder, started 04:42). If it finishes, `.cache/b/p/m/browser/prototype/tools/descriptor-emitter/win32` and `win32.source-digest` appear. Until then the shared emitter path is empty, so another agent's component build that needs the emitter will queue behind or fail on it.
- Not done: trinity component build, `activate` and `serve` on 6054 and 6056, and every browser check (Jack editor and viewer, MATCH/CREATE on a small fixture, RETURN results, wheel zoom, query-to-canvas sync, undo, Rewriting editor and viewer). No Playwright script or screenshots exist. The `manifestId` snapshot field was not exercised.
- Next command once the emitter publication exists, from `C:\semio` with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=C:\cargo-nr CARGO_BUILD_JOBS=8`: re-run the `plugin trinity` launcher line from the continuation section below (it now skips the emitter build because the digest matches).

## Restart after the Cargo.lock retry (not a repo bug; launcher bug)

- Both `plugin-build7` and `plugin-build10` ended right after the line "Another process kept editing C:\semio\Cargo.lock ... starting the command again with a new snapshot (1 of 6)". That line comes from the restart wrapper `runBuildWithRestartV1` (`🏗️build/🚪️door/🔄️restart/🟦️.ts`) and `runWithStaleReexecV1` (`🏗️build/🧷️format-stamps/🔄️reexec/🟨️.mjs`).
- The wrapper treats a command as the outermost one only when `SEMIO_SCRIPT_PROCESS_INVOCATION` is unset. My `launch-native.ts` injects that envelope, so the script is a nested command. A nested command that needs a restart does not start a replacement: it prints the message and exits with the declared restart code 75 (`protocol.exitCode`, `EX_TEMPFAIL`) so that its outer caller restarts it. My launcher took that code as final and exited without restarting, and my `nohup` log recorded no exit code, so it looked silent.
- So the repository behaves as specified: a retried command either completes or exits with non-zero code 75, and the existing law in `🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts` (line 203, nested stale run yields `{status:"yielded",code:75}`) already covers it. No repo change and no new test.
- Fix applied to the launcher only (`launch-native.ts` in this folder): it now restarts the `repo` child on exit code 75 up to 6 times and logs `[DEBUG] repo exited code=... signal=... attempt=...` for every exit.

## Sixth session notes (14:20 onwards)

- `plugin-build15` failed on `--locked` because the root `Cargo.lock` lacked the new internal path crate `semio-plugin-command`. `cargo update --workspace --offline` in `C:\semio` (G: dirs) reported "Locking 0 packages"; by then the lock already contained the crate, and the only difference to my backup was one added internal dependency line (`"semio-plugin-command",` at line 16266). No third-party version changed. `cargo metadata --offline --locked` cannot be used here because the workspace rustc wrapper requires the repository launcher.
- `plugin-build16` (G: `CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR`, `TMP`, `TEMP`) passed the quiet inventory phase, printed `Descriptor emitter is not published; building it first` and entered the emitter build. The log was at 14:44 and still growing slowly when I stopped. No error yet, so the aggregate-failure diagnostics have not been needed.
- Still not done: emitter publication, Trinity component build, serve on 6054 and 6056, Playwright and screenshots.

## Next blocker: emitter build time budget (end of day, 20:40)

- `plugin-build16` ended at 20:40 with: `plugins:build stopped: the maximum duration of 360 min derived from its declared cost (0 unit(s)) is reached`. The emitter build (`semio-plugin-describe` `build`, run inside `plugin trinity`) declares 0 cost units, so its time budget silently falls back to the 360-minute cap, which the compile plus provenance verification (about 70 minutes of compile and 78 minutes of verification on earlier runs, plus lease waits of hours on a shared machine) exceeds.
- Likely fix, not made: the describe-rs build (`🖨️describe/📦️packages/🦀️rust`) must declare a real cost so its budget scales, and a 0-unit declaration must not silently fall back to the cap (refuse it or report the fallback).
- No builds were started after the coordinator's wrap-up request.

## Exact resume sequence

Run from `C:\semio`, with `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=8`, `CARGO_TARGET_DIR=G:\t\nr`, `CARGO_BUILD_BUILD_DIR=G:\t\nrb`, `TMP=TEMP=G:\t\tmp` (create the three G: directories first). `BUN` is the pinned runtime under `%LOCALAPPDATA%\semio\runtimes\bun\<hash>\bun.exe`, `T` is this ticket folder.

1. Check that the junction `node_modules\@semio-tech\ui-layout` exists (recreate it to `🧰️framework\🔨️modules\🖱️ui\🧬️contract\🖼️presentation\📐️layout` if `bun install` has not linked it) and that no other `launch-native` process holds `tools:descriptor-emitter` (`taskkill /T /F` any leftover launcher root).
2. Publish the emitter first, on its own, so its budget is not shared with the component build: `BUN "$T/launch-native.ts" 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust semio-plugin-describe:build ./📜️script.ts build --host-platform=win32`. Compiled units are cached in the managed cargo target `C:\semio\.cache\b\r\m\u\l\qc4d6d2`, so only verification and publication remain. Expect `descriptor-emitter\win32` and a fresh-state record to appear under `.cache\b\p\m\browser\prototype\tools\descriptor-emitter`.
3. Build and publish the component: `BUN "$T/launch-native.ts" 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript @semio-tech/framework-os-dev:prepare-trinity-jack-react-prototype ./📜️script.ts plugin trinity`. Expect `modules\🔱️trinity` next to `🏗️fem`. The launcher restarts the child on exit code 75 and logs every exit with `[DEBUG] repo exited`.
4. Activate and serve through the same launcher and directory: `./📜️script.ts activate trinity-jack react prototype`, `./📜️script.ts serve trinity-jack react prototype` (port 6054) and the same for `trinity-rewriting` (port 6056). Wait for `data-semio-os-ready=<variant>`.
5. Drive both pages with headless Playwright (small fixture, not `setActiveExample` whole-document load), screenshots into `$T/🗑️generated`: Jack editor and viewer open; run a MATCH and a CREATE query; RETURN graph results render; wheel zoom on the canvas; query-to-canvas sync; undo; Rewriting editor and viewer open. Use `manifestId` (not the old manifest field) in Jack snapshots.
6. Delete the `🗑️generated` logs when done, keep this report and `launch-native.ts`.

## Duplicate build stopped (18:40)

- `plugin-build15` (launcher PID 36728, started 11:29 with default C: dirs) was still alive and had been waiting 14,303 s on the exclusive lease `tools:descriptor-emitter:prototype:win32`, in parallel to `plugin-build16` (PID 25260, G: dirs). It was a duplicate of build16, not a code-75 restart: it was my own earlier launch that never exited. I stopped its whole tree with `taskkill /T /F /PID 36728`. Free space on C: read 95 GB right after.
- `plugin-build16` kept running. It had finished compiling the emitter units (log line "63156 diagnostic line(s) not shown") and was in the post-compile verification, with its script process (PID 25412, CPU 2029 s) idle for at least 15 minutes and the log silent since 18:28. `descriptor-emitter/` is still empty.
- Note for the repo: a build waiting on an exclusive emitter lease for hours should say who holds it; the log only repeated "remains active on tools:descriptor-emitter".

## Diagnosability fix for aggregate failures

- `withWindowsCompilerLifetimeV1` (`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛠️native-compiler/🪟️windows/🧬️lifetime/🟦️.ts`) now exports `describeAggregateChildrenV1` and puts every child's message and cause chain into the message of both of its `AggregateError`s ("consumer and integrity recheck" and "consumer and retained check").
- `descriptorEmitterBuildFailureV1` (`🖨️describe/🛠️emitter/🥾️prerequisite/🟦️.ts`) builds its detail with the new `descriptorEmitterFailureDetailV1`, which names the children of any `AggregateError` and the `cause` chain, so the English and German prerequisite message keeps the root cause. The surrounding text stays bilingual; the child messages are the raw error texts.
- Tests in `🥾️prerequisite/🧪️tests/🟦️.ts`: aggregate children and cause chain appear in the prerequisite failure, and the lifetime helper formats children as `[1] consumer broke <- EBUSY; [2] integrity moved`. 12 of 12 pass.
- The G: root-cause question is open: the real error of `plugin-build12` was never printed, and `plugin-build15` (default dirs) had not failed when this was written.

## Fifth session notes (11:00 onwards)

- `plugin-build12` (G: target and temp directories) exited with code 1 after about 5 minutes: `Descriptor emitter prerequisite build failed: Windows compiler consumer and integrity recheck failed`. That is an `AggregateError` from `withWindowsCompilerLifetimeV1` whose child errors (the body failure and the integrity failure) are not printed, so the root cause is not visible; the message points at `repo run @semio-tech/os-plugin-describe-rs:build` for full output, which needs the graph. A candidate is the `TMP`/`TEMP`/`CARGO_TARGET_DIR` overrides to G: (the earlier `plugin-build10` with the same overrides did compile and publish 173 units). Printing `error.errors` in that `AggregateError` would make such failures diagnosable; not changed.
- Two relaunches without the G: overrides (`plugin-build13`, `plugin-build14`) failed on a peer's half-landed edit (`Cannot find module './🪪️profile/🟦️.ts'` in `⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts`); the directory appeared a few minutes later.
- `plugin-build15` (default dirs, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=8`) is running and was still in its quiet inventory phase at 11:40. `descriptor-emitter/` is still empty. Trinity component build, serve and the Playwright run have still not happened.

## Fourth session notes (07:50 onwards)

- The coordinator said plugin-build7 finished at 07:03, but no emitter was published: `descriptor-emitter/` was empty, there is no `win32.source-digest`, and the process had ended right after its phase table (last log line was a `Cargo.lock` snapshot retry). Provenance verification took 4671 s.
- My digest logic in `🖨️describe/🛠️emitter/🟦️.ts` was overwritten for the second time. This time it is superseded rather than lost: another agent replaced it with `ensureFreshDescriptorEmitterV1` (`🥾️prerequisite`, `🌡️freshness`) which the driver now calls. I did not re-apply mine. I removed my dead code: the digest and staleness helpers from `📋️contract/🟦️.ts` and the fixtures `🔣️digest.json` and `🔣️staleness.json`. Any earlier mention of those in this report is historical.
- The Windows fixture-root fix (`resolve(tmpdir(),"workspace",".cache","b","p","m")`) had been reverted as well and was re-applied. The request test file now has 6 tests, all passing.
- A new workspace package `@semio-tech/ui-layout` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout`, listed in the root `package.json`) was not linked into `node_modules`, so every script load failed with `Cannot find module '@semio-tech/ui-layout'`. I created the junction `node_modules/@semio-tech/ui-layout` (equivalent to what `bun install` does) so scripts load again.
- Relaunched with `CARGO_TARGET_DIR=G:\t\nr`, `CARGO_BUILD_BUILD_DIR=G:\t\nrb`, `TMP/TEMP=G:\t\tmp` (`plugin-build10.log`). It printed `Descriptor emitter is not published; building it first` and was waiting on leases and the rebuild when I stopped. The machine was so loaded that single shell commands exceeded 2 minutes.
- Still not done: trinity component build, activate and serve (6054, 6056), all Playwright checks and screenshots.

## Third session notes

- The Windows failure in "request is admitted by the closed artifact reader schema" is fixed: the fixture root is now `resolve(tmpdir(),"workspace",".cache","b","p","m")`. All 8 tests in the emitter request test file pass.
- Another writer overwrote `🖨️describe/🛠️emitter/🟦️.ts` at 06:47 and silently dropped the `ensureDescriptorEmitterV1` digest logic (last-writer-wins replica). It was re-applied at 06:50 and the file again contains it; re-check it before trusting the digest path. The running rebuild loaded the original edit at 04:42, so it will still record the digest.
- Provenance verification profile (not a profiler run): the worker is one bun process (PID 16984), CPU-bound at about 0.5 core, 3,000 CPU seconds so far, 497 MB resident. It is not waiting on I/O or leases and spawns only about three `Get-CimInstance Win32_Process` enumerations per 40 seconds, so process enumeration is not the bottleneck. It reads the build records of 578 compiled artifacts one by one (445 to 473 progress lines, about 10 per 5 minutes late in the run). I did not attach a real profiler, so the hot function is unknown.
- Cargo target: the managed pipeline sets `CARGO_TARGET_DIR=C:\semio\.cache\b\r\m\u\l\qc4d6d2\t-ce555540` itself and ignores the `C:\cargo-nr` value I exported, so my build does not use `C:\cargo-nr` (its recent writes come from other agents). The target is on C: under `C:\semio\.cache`.

## Blocker (first session)

The host disk (C:) reached 0 bytes free at about 04:15. The emitter rebuild then died with `SQLITE_FULL: database or disk is full` in the lease store (`🔒️leases/🟦️.ts`), after the lease store had already reported 93 to 98 percent full for 40 minutes. Free space fluctuated between 0 and about 23 MB afterwards, so no build, cargo or browser step can run. I did not delete other agents' caches to free space.

Not verified, because no Trinity component could be built and served: loading in s, Jack editor/viewer and Rewriting editor/viewer activation, typing and running MATCH/CREATE/SET/DELETE, RETURN graph results, canvas wheel zoom, query-to-canvas sync, undo. No screenshots exist. No Playwright script was written yet (the DOM of the served app was never reachable).

## How to continue once disk space is back

1. Free space on C: (the Rust and cargo target partitions of the other agents are the usual cause).
2. From `C:\semio` run the emitter rebuild with the envelope launcher, then the component build, then serve:
   - `bun launch-native.ts 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust semio-plugin-describe:build ./📜️script.ts build --host-platform=win32` (after moving the stale `.cache/b/p/m/browser/prototype/tools/descriptor-emitter/win32` aside).
   - `bun launch-native.ts 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript @semio-tech/framework-os-dev:prepare-trinity-jack-react-prototype ./📜️script.ts plugin trinity` (builds and publishes the `🔱️trinity` component under `.cache/b/p/m/browser/prototype/modules/`).
   - Then `./📜️script.ts activate trinity-jack react prototype` and `./📜️script.ts serve trinity-jack react prototype` through the same launcher (port 6054; `trinity-rewriting` is 6056). When the repo graph is responsive, `repo run @semio-tech/framework-os-dev:dev-trinity-jack-react-prototype` does all of it.
3. Drive it with headless Playwright using a small fixture (not the whole-document `setActiveExample` path) and check the four coordinator items: RETURN graph results, wheel zoom, query-to-canvas sync, Rewriting editor and viewer activation.

## Root fix made (emitter staleness)

- `📋️contract/🟦️.ts` (under `🖨️describe/🛠️emitter/`): added `descriptorEmitterSourceDigestV1` (SHA-256 over `<path>`, NUL, `<file sha256>`, newline lines, sorted by UTF-8 path bytes, for every `*.rs` and `Cargo.toml` of the `🖨️describe` module, skipping `target`, `node_modules`, `🧪️tests`, `🤖️generated`), `descriptorEmitterDigestOfV1`, `descriptorEmitterDigestPathV1` (sibling file `<platform>.source-digest` beside the published tool) and `descriptorEmitterStaleV1`.
- `🟦️.ts`: `buildDescriptorEmitterV1` records the digest computed before the build next to the publication after it succeeds. `ensureDescriptorEmitterV1` now rebuilds when files are missing (as before) or when the recorded digest is absent or differs (removes the stale publication and digest, then builds).
- `🏗️build/🧩️driver-stages/🟦️.ts`: `runRepositoryComponentDriverV1` now calls `ensureDescriptorEmitterV1` before the capability preflight whenever the plan has no emitter build job. Before this, `buildComponentClosureV1` (used by `plugin <id>` and the mcp e2e) never ensured the emitter.
- Tests (`🧪️tests/📏️request/`): `🔣️digest.json` (files plus expected digest computed independently with Python `hashlib`; the Bun test builds a fixture workspace from it, also checks that skipped files do not change the digest and a changed source does), `🔣️staleness.json` (recorded/current/stale cases consumed by the contract function and by the real transpiled `ensureDescriptorEmitterV1` with injected ports, asserting build and removal counts). The existing publication-case test was updated for the new ports. 7 of 8 tests in the file pass; the failing one (`descriptor emitter request is admitted by the closed artifact reader schema`) fails on Windows because its fixture root `/workspace/.cache/b/p/m` is not an absolute Windows path; it failed the same way before my change.
- The stale (digest-less) emitter was rebuilt by this path: the 2026-11-18 publication was removed by `ensureDescriptorEmitterV1` itself and a rebuild started under `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=8`.

## Files

- Changed: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/📋️contract/🟦️.ts`, `.../🛠️emitter/🟦️.ts`, `.../🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`, `.../🛠️emitter/🧪️tests/📏️request/🟦️.ts`; created `.../🧪️tests/📏️request/🔣️digest.json` and `🔣️staleness.json`.
- Created: this report and `launch-native.ts` (envelope launcher, generalized from the FEM 2D ticket) in the ticket folder. The `🗑️generated` logs were deleted.
- The cached emitter publication was replaced by the rebuild.
