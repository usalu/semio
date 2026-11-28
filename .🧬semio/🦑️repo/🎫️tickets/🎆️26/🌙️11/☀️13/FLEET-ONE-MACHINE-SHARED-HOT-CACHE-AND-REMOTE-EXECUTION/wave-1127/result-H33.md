# H33: m02/m12 workspace "Owned Nx invocation failed (124)"

## Measured (m02 attempt 9, stage 01:31:47-02:00:18 = 1700 s front time)
- ~890 s before `repository:command` starts, ~140 s graph (daemon wrote graph 01:48:54), Nx run 714 s. The `outside-task=673` figure is run wall minus the longest *ledger*; only native-build tasks write a phase ledger (one of 9 tasks did, 40.8 s), so it is not overhead. The run's last 600 s are the stall.
- No install/scope lines in any m02/m12 workspace log: H23/H26 `ensureBuildScopesV1` is not the cause (receipt current). Graph is rebuilt per landing (digest change), 7 min m12, 17 min m02 under load; not reduced here.
- 124 is the Script liveness watchdog (`🐕️watchdog`, `process.exit(124)`): "no progress for 10 min and no live child". Both logs show `[nx-flight] Waiting for g1-m06 to finish task ...` directly before: Nx parks in a cache GET on the loopback single-flight front for up to 30 min (waitBudget 1800 s) with no output and no child, so the 600 s rule kills it. Not deterministic: m02 attempt 10 went Ready; m12 attempt 10 failed differently (see findings-H33.md).

## Landed (04:05, `T/landings.md` "Wave 1127 H33")
1. Host declares a bounded wait (`enterScriptProcessWaitV1`) while the front awaits a peer (`waiting` hook).
2. `nxFlightStallEnvironmentV1`: Nx and its tasks get `SEMIO_SCRIPT_STALL_MS` = wait budget + 120 s (1920 s default), derived from the declared budget, only raised, never lowered.
3. `runOwnedNxInvocationV1` throws a classified refusal: 124 -> `stalled` `nx.run-stalled`, else `failed` `nx.run-failed`, both languages (no more "Internal error").
Tests: `🚦️single-flight/🧪️tests/⏳️stall` (budget outlasted, judged by the watchdog's own rule as oracle; hook depth with a real two-front claim table; refusals render en+de); single-flight suites 22 pass/0 fail; `repo typecheck` 3 files 0 errors.

## Unverified
- Which process printed the notice (Nx child vs task vs host) is inferred from log order; the fix covers host and all children. No live worker has run the new code yet (m02 Ready, m12 backing off; read their next attempt).
- "Deadline from measured history" is only derived from the declared wait budget; no per-task duration history is read. Peers' flight holders that never finish still cost up to 32 min.
- Landed with locks released before typecheck (rule slip; typecheck then passed). dangling-imports not re-run.
- m12 new fault: pruning custody race (findings-H33.md); m11 fails on a real Rust compile error.
