# CAS gc lock, 2026-11-23

## Diagnosis
- pid 43732 is the service host's in-process automatic CAS enforcement (`casEnforceTickV1` -> `createCasGuardV1` -> `collectCasV1`, role `guard-background`, `incremental`), not a stuck child. It acquires `gc.lock` per collection (about 9 s of every 30 s tick, measured 36 % occupancy), releases it, and re-acquires it on the next tick. The lock `atMs` looked "refreshed continuously" because it is re-created each tick.
- The lock was never held indefinitely. A manual `repo cache cas gc` that lands in the busy window was refused at once (`acquire` had no wait), which looked like "always refuses". One run that hit a free window completed.
- Design flaw: the lock had no heartbeat (`atMs` written only at acquire) and a 2 h stale limit, so a wedged collector would block everybody for 2 h; a waiting manual gc could neither join nor wait.
- The CAS cannot give space back: 10 760 objects, 19.39 GiB, all with link count > 1 (hard-linked into workspaces, so protected, and removing the CAS name frees nothing). Automatic collections therefore find nothing to evict. The 29 GiB partition pressure is not the store's. Gc freed 236.5 MiB once (stale areas), then 0.
- No process was stopped. 43722 and 43732 are untouched.

## Fix (schema-first, `🗄️cas`)
- `🧬️schema/🔣️.json` + `🟦️.ts`: `lockStaleMs` 7 200 000 -> 300 000 (a heartbeat proves progress), new `lockWaitMs` 120 000.
- `🧹️collector/🟦️.ts`: `acquireWithin` (cancellable polling wait, `waitMs`, default 0 so automatic collectors still skip); `beat` rewrites the lock atomically (pid-checked) only when the progress counter (scanned + evicted + busy + skipped, shards compacted) changed, from `report` and per shard in `compactAll`.
- `⌨️command/🟦️.ts`: manual gc passes `waitMs: policy.lockWaitMs`, so it joins the next free window.
- Test: `🧪️tests/🟦️.ts` "a manual collection waits for the running one and then collects, and the holder's heartbeat proves its progress". 50/50 pass (one pre-existing timing-sensitive test, "refuses a write that stays over its cap", failed once under load, passes alone and on rerun); holders suite 12/12.
- Running service host (43722) still has old code until restarted; it needs no stop for correctness.

## Size bound (item 4)
Already present: `casEffectiveCapBytesV1` caps at `capHostSharePercent` (3 %) of the partition, bounded by the host free floor, plus partition watermarks (high 90 %, low 80 %, critical 97 %). Effective cap here is about 6.8 GiB versus 19.4 GiB held, but everything held is hard-linked (protected), so the cap cannot be met by collection. No schema change needed.

## Disk
`df` free: 3.7 GiB at task start (2.2 GiB later) -> 18 GiB before gc (other processes freed space) -> 17 GiB after. CAS gc freed 236.5 MiB total in one run, 0 in the last. The ledger/target dirs of other tickets, not the CAS, hold the 28 GiB of pressure.
