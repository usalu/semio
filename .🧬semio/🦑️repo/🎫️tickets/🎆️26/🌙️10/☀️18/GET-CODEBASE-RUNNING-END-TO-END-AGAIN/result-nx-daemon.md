# result-nx-daemon (wave 1023a, executor nx-daemon)

Tools: `C/nx-daemon-tools/repro.sh <project:target> <name>` (runs `SEMIO_BUILD_GRAPH_POLICY=current NX_VERBOSE_LOGGING=true nice -n 10 bun repo run …`, log `C/🗑️generated/nx-daemon/<name>.log`). Pids I started: `C/🗑️generated/nx-daemon/pids.txt`.

## 00:55 findings + live edits (UNVERIFIED until the runs below finish)

### Fault A (the masking, found first): "Original process admission closed"
- Root: `track()` in `🧭️routing/📥️invocation/🏃️process/🟦️.ts` closes the whole owner (`accepting=false`) when ANY accepted task rejects, even when the caller handles that rejection. `poolWithRoom` (`💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`) throws `NxGraphRoomNeededV1` / `NxGraphNamespaceBusyV1` INSIDE an accepted pool pass and catches them OUTSIDE it to evict idle namespaces and retry; the retry (`evictIdleNamespacesV1` -> `withNxGraphOperationTaskV1` -> `controlFor` -> `admit`) was then refused with "Original process admission closed", and `settle()` reported that secondary refusal instead of the first failure.
- Fix 1 (masking, EN/DE): `OriginalAdmissionClosedV1` (typed refusal); `settle()` now throws the owner's FIRST recorded failure when the thrown error is only an admission-closed refusal, and writes one notice per selected language to stderr (`SEMIO_LANGUAGE`/`LC_ALL`/`LC_MESSAGES`/`LANG` select one declared language, none named -> every declared language, en then de). Texts are schema-declared in `🧬️schema/📈️progress.json` (`const.firstFailure`). Seen live: `First failure: Funded Nx graph cache capacity required (the later refusal "Original process admission closed" is only its consequence.)`.
- Fix 2 (root of the graph-pool half): `poolWithRoom` now reports "needs room" / "namespace busy" as an OUTCOME value of the accepted pass (`{value}` | `{retry}`), so the pass resolves and the owner stays open for the eviction + retry.
- Tests (language-agnostic, SQLite oracle): `🧪️tests/🥇️first-failure/{🟦️.ts,🔣️.json}` (3 tests, pass with `bun test ./<file>`): rendering vs. an independent SQLite substitution for 5 environments, a masked refusal never hides the first failure (and the notice is written), an outcome keeps the owner open while a caught failure closes it.

### Fault B: daemon service principal rejected by `readNativeCallerAuthorityV1`
- Root: the Nx daemon service runs under its own envelope (`kind: "nx-daemon-service"`, closed `ServiceCapabilities`: repositoryRoot/workspaceRoot/…, no `native` grants). Every store/opener on the daemon's path read the root through `readNativeCallerAuthorityV1(...).repositoryRoot`, which only knows the native-owner and build-orchestration principals -> `Original native caller capabilities required` (the first run, nxfail4, showed `kind=nx-daemon-service`).
- Fix: one reader for the root of every admitted principal, `callerRepositoryRootV1` (`⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts`, validates the service against the lifetime schema `$defs.ServiceCapabilities`, re-exported by `📦️packages/…/📥️invocation/🟦️.ts`). Used by: `🚀️bootstrap/🛠️tools/📥️use` (2), `🔌️nx/🟦️.ts` (1), `🔌️nx/📦️graph/🟦️.ts` (3), `📥️opening/♻️reset/🟦️.ts` (`ownedRepositoryRootV1` simplified; replaces course-rest2's special case), `🏪️store/📇️format/🟦️.ts` (1). Functions that need the full native grants (publication, rustup, cargo, tooling provisioning) intentionally still call `readNativeCallerAuthorityV1`.
- Test: `📥️invocation/🧪️tests/🏠️caller-root/{🟦️.ts,🔣️.json}` (SQLite json oracle; 7 rows: owner, orchestration, service, 4 refusals) - passes.

### Nx patch
- `[DEBUG] Original detached daemon refused` removed: `🩹️patches/nx@23.2.0.patch` line 2057 now prints `Original detached daemon refused: <message>` (stack only with `NX_VERBOSE_LOGGING=true`). NOTE: editing the patch invalidates the dependency generation receipt for the whole fleet (one reinstall; r3 waited ~8 min for a peer's bazel dependency build, pid 93411).

### Temporary spin-hunt instrumentation (REMOVE): `// [DEBUG] spin-hunt begin/end` block in `🏃️process/🟦️.ts` (active only with `SEMIO_DEBUG_SPIN=1`; SIGUSR2 dumps the last 48 scheduled setImmediate/setTimeout stacks).

## Open
- (3) parent spin after a failed nx child: not isolated yet. Evidence: a 5 h old spinner exists (pid 22076, child of door 21541 = `repo run @semio-tech/repo-dashboard-rs:run`, not mine; leave it) holding nx-tooling / graph-namespace / native-fixed-image leases, 10+ open `claims/*.sqlite` queue tickets of its own (`queued-original:<resource>` created 19:01-19:05, resources include the native-fixed-image it itself holds shared), 150 % CPU, kevent64/__ulock_wake in `sample` (event loop turning, JS stack invisible: stripped Bun). It also holds a 3584 MB capacity grant that starves every other `run`.
