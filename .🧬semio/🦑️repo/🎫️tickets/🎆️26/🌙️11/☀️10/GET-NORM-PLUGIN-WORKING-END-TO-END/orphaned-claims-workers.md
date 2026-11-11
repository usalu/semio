# Orphaned claims workers

## Root causes
1. CPU burn: not the lease wait. Bun 1.4.2 spins one core at ~99% on `await new Promise(()=>{})` when nothing else keeps the event loop alive (reproduced with a 1-line script). The worker's `CLAIMS_MODE=hang` did exactly that. Waiting workers measured ~0 CPU (file and queued leases), so no busy-poll exists in `@semio-tech/process/leases` or the claims lease ports.
2. Orphans: the test runner (`bun test`) was killed (runner abort); its preload guard died with it, and the spawned workers had no guard, so they were reparented to `systemd --user` and spun forever. The existing `afterEach` already SIGKILLs children on normal end/failure/timeout.

## Changes
- Added generic `exitWithParentV1` (worker-thread poll of parent pid/liveness, kills the process tree) in `process/🧪️testing/🟨️bun/🪦️orphans/👶️child/🟦️.ts`; `startTestOrphanGuardV1` now reuses it. Exported via `@semio-tech/process/test-orphans` (new `📦️packages/🟦️typescript/🧪️testing/🪦️orphans/🟦️.ts` + package.json export).
- Claims test helpers `.../🧩️claims/🧪️tests/🏃️processes/📥️worker/🟦️.ts` and `📥️holder/🟦️.ts` call `exitWithParentV1()`; hang mode awaits a promise backed by a long timer (no spin).
- Tests: claims `🏃️processes/🟦️.ts` + `🔣️.json` (CPU time of hung holder and waiter bounded to 50% of wall, Bun resourceUsage plus `ps cputime`; fails without the fix); orphans `🧪️tests/🟦️.ts` (helper ends with dead parent, control survives; Python oracle `alive`), fixture, schema, fixtures `🪦️guarded.ts`, `🚪️abandoner.ts`.

## Results (managed Bun 1.4.2 directly, no graph wait)
- claims processes: 10 pass, 0 fail (spin test fails 2/2 with the fix reverted).
- orphans: 11 pass, 0 fail.
Logs: `🗑️generated/`.

## Item 4: other tests spawning async children without kill/finally/afterEach (heuristic, not fixed; 19 files)
sdk-surface, module-package-surface, Paint2dHost ownership, command-composition-source, dependencies/inputs, supersede, generator-ownership, nx-plugin arguments, bootstrap dependencies, implementation-edit, host-capacity restart, door output, source closure warm, pixels ownership, file-budgets, host-capacity handover, process build output, custodian forged script, owned-execution progress. Most are likely awaited runs; they can adopt `exitWithParentV1` in their helper scripts.
