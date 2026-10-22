# result-bzl-tail (executor bzl-tail, wave 1022a)

STATUS 10-22 ~09:35: part 1 (failed-action tails) WRITTEN + unit test green (5 pass, incl. Python oracle) but NOT yet proven in a live failing Bazel run; root cause of the e2e-1021l failure FOUND from Bazel's leftover stderr (see Finding); parts 2 and 3 NOT run: the CLI never reached the Bazel stage (hours of fleet-wide queues/faults, see Log).

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`.

## Part 1: failed-action output tails (design)
- Cause of the hidden error: Bazel's console echo of an action's stdout/stderr is capped by its own default `--experimental_ui_max_stdouterr_bytes` (1 MiB) and a larger output is skipped entirely. The adapter never relied on a flag; it simply had no copy of its own.
- New module `B/🪵️failure-tail` (schema `🧬️schema/🔣️.json`, code `🟦️.ts`, tests `🧪️tests`): schema-declared Policy (`maximumTailBytes` 65536, directory `failures`, suffix `.tail.log`, `maximumFiles` 64), `Header` (task, bytes, droppedBytes) and EN/DE `Messages`.
  - `createBazelFailureTailV1()` keeps the last N bytes of all lines (stdout+stderr, arrival order), cut never inside a UTF-8 sequence, counts `droppedBytes`.
  - `writeBazelFailureTailV1` persists `<dir>/<ascii-stem>-<sha12>.tail.log` = one JSON header line + tail (tmp+rename). `readBazelFailureTailsV1` / `renderBazelFailureTailV1` / `reportBazelFailureTailsV1` read and print them ("[bazel] task X failed; last N bytes of its output (M earlier bytes omitted), saved to FILE:" / German twin).
- Wiring in `B/📜️script.ts`:
  - `executeTask(..., failures)` feeds every output line of the task's commands into a tail (`onLine`) and, when the task throws (and the phase was not cancelled), writes the tail into `<generated package>/failures/` (`dirname(manifestPath)`).
  - `runBazelTasksInStorageV1`: clears that directory before the build and, when the `bazel build/test` command rejects, prints all tails to stderr in the `SEMIO_LANGUAGE`/locale language, then rethrows. Independent of Bazel's UI echo.
- Test (language-agnostic): `B/🪵️failure-tail/🧪️tests/🧫️fixtures/🔣️.json` (cases with expected text/bytes/dropped, rendering expectations EN/DE, file-name cases) + independent Python oracle `🧪️tests/🧮️resources/🐍️oracle/🐍️.py` fed with the same fixture (incl. a 20,000-line generated case); `🟦️.ts` compares both. Run: `cd <scratch> && bun test "<abs>/🪵️failure-tail/🧪️tests/🟦️.ts"` -> 5 pass / 0 fail, 58 expects.

## Log
- 01:41-01:53 `show targets` retries failed because of peer churn: authorizer closure cap 1024->2048 (someone raised it at 01:39, fixed), then leases/🟦️.ts edits ("Exact live original writer proof"), then captured-closure faults in the nx test plugin (`🧪️test/🟨️.mjs` dynamic `?revision` import, peer already patching) and `@iarna/toml/package.json` require in `🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs` (graph recompute path never run under capture before; cached facts hid it).
retry-show pid 54171

## Finding (from Bazel's own leftover file, no run needed)
The e2e-1021l failure was found in the slot output base: `.cache/b/p/m/bazel/outputs/server/s0/execroot/_main/bazel-out/_tmp/actions/stderr-2` (1,583,203 bytes, 23:38). Its tail: `error: Stale loaded repository Store implementation; restart required` at `requireCurrentRepositoryCacheStoreV1` (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:23`) called from the rustup `sourceRequest` (`🦀️rustup/📥️installation/🟦️.ts:42`) inside `native:owner-command` -> `@teaching/proctor:build:release`. The first ~1.5 MB are `[script] ... cargo:workspace-selection` progress lines plus ~510 rustc "unnecessary qualification" warnings, which is why Bazel skipped the output. Cause = peer edit of the Store source/schema while the unpinned physical-source process ran (`implementationPinnedV1()` false -> self-check compares live disk with the loaded revision). Frames are physical paths (not `semio-source:`), i.e. the process ran outside the pinned/captured entry. Not yet fixed (see open).

## Log (continued)
- 03:42 `show targets` passes the dependency stage (dep-receipt fix) but queues on `repository-graph-facts:shard:7` behind pid 72404 (play dry-run, 29 min, 4.7 GB). Waiting (foreground bounded loops; run log `C/🗑️generated/bzl-tail/try-1.raw`, pid chain 70992/70993).
- Coordinator killed my first detached retry chain (it ran old dependency-receipt code for 91 min holding the build lease). I no longer run retry chains.

## Open
- Parts 2 and 3 (real proctor release run, architecture-course build) not run yet: blocked on the CLI queue.
- Unpinned process self-check (`Stale loaded repository Store implementation`) must be removed for processes spawned inside Bazel actions: either run them under `withImplementationPinV1` or compare against the captured image.
- 06:51 proctor release run pid 34132 log C/🗑️generated/bzl-tail/proctor-1.log
- proctor-1 failed in dependency generation verify 'Original relative directory facts changed' (peer churn); proctor-2 pid 35623
- proctor-2 (waited 2h) failed 'Native host does not match its authored producer' (playgroundPreparationTargets, graph assembly); proctor-3 pid 72790

## Log (final, 06:51-09:35)
- 06:51 proctor-1: dependency generation verify 'Original relative directory facts changed' (peer churn). 06:53 proctor-2 queued 2 h 30 behind graph-facts shard 7 (cold v3 generation builders 29009/48268/50262), then 'Native host does not match its authored producer' in `playgroundPreparationTargets` (graph assembly, not from my change; a playground nativeHost declaration disagrees with the cargo owner facts).
- 09:24 proctor-3/4 (launcher now runs Bun 1.4.2): 'Uncaptured implementation module refused: /private/tmp/i6/ovlaws/.../leases/retirement/continuation/ts' (also seen by slice-play2). Hypothesis: Bun 1.4.2 runtime transpiler cache is keyed by content, so a peer's byte-identical overlay copy under /private/tmp/i6/ovlaws leaks its path into the repo's captured run. With `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` the refusal disappears (proctor-5); unproven as the real cause, and the clean fix belongs in the launcher (per-root or disabled transpiler cache).
- 09:30 proctor-5: `Fixed Store dependency installer failed`: `error: lockfile had changes, but lockfile is frozen` (bun.lock stale vs package.json edited 09:23 by the round-6 burst; also in result-course-rest.md). Needs lock-sync to regenerate bun.lock.
- No process of mine is left running (all proctor-N runs exited; retry chain killed by coordinator; my queued bazel `show targets` loop killed by me at 05:00 when 29009 was announced).

## What is needed to continue (in order)
1. bun.lock regenerated for the current package.json (lock-sync owner).
2. Launcher: transpiler-cache isolation or disable (Bun 1.4.2) so overlays under /private/tmp/i6 cannot leak into the repo run.
3. Playground nativeHost mismatch (`playgroundPreparationTargets`): find the playground whose `nativeHost` disagrees with `resolveOwner(cratePath)` facts (add the project/crate path to the error message, EN/DE: the message currently names nothing).
4. Then `SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel run @teaching/proctor:build --configuration=release` (log under `C/🗑️generated/bzl-tail/`), read `<generated package>/failures/*.tail.log` on failure, then `@teaching/architecture-course:build`.
5. Fix for the original e2e failure: a nested unpinned `native:owner-command` process re-checks Store source against disk (`implementationPinnedV1()` false) and dies with 'Stale loaded repository Store implementation; restart required' when a peer edits the Store files mid-run. Under Bun 1.4.2 + stale-impl-guard (exit 75 + re-exec) this may already be handled; verify with the first live run, otherwise run those processes under `withImplementationPinV1`.

## Changed files
- NEW `B/🪵️failure-tail/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json`, `🧪️tests/🧮️resources/🐍️oracle/🐍️.py`
- EDIT `B/📜️script.ts` (import, `executeTask` failure tail + `failures` argument, caller in `runBazelActionV1`, clear + report in `runBazelTasksInStorageV1`)
- tools: `C/bzl-tail-tools/retry-show.sh`; generated: `C/🗑️generated/bzl-tail/*`
