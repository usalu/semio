# Idle lease holder fix (opus1122)

## Finding
The lease system already has the intended mechanism: per-holder `standing`, a stuck judgement (silent for `freshness.stuckAfterMilliseconds`, 1 h default), a waiter-written yield request (`reclaim.mode=request`) that the holder honours through its own yield watcher, and an opt-in `terminate` mode. It never fired for pid 4001827 because the holder was invisible: `registry/🟦️.ts` `flush()` ended in `catch{disabled=true;}`. One failed publish (ENOSPC on the full disk) switched the registry off for the life of the process, so the heartbeat stopped, the entry aged past `STALE_MILLISECONDS` (5 min) and was pruned, the holder judged `unknown`, and `observeLeaseHolderV1` decides nothing against `unknown` (hence WAITING and no HELD row). Waiters then only had the no-progress watchdog.

## Fix
- `🧭️order/🗂️registry/🟦️.ts`: removed the permanent `disabled` flag. Directories with holds or waits stay in a `tracked` set; the heartbeat (now started on failure too, public `leaseRegistryV1.heartbeat()`) republishes every tracked directory each 60 s. A live holder regains its standing as soon as the disk recovers, so the existing stuck, yield-request and optional terminate path applies. No hidden killing was added; the order-policy `react()` was not touched.

## Test
`🧭️order/🧪️tests/💓️heartbeat/🟦️.ts`: injected source and root; the first publish fails (root under a regular file), the root is then made writable, one `heartbeat()` publishes the entry with holds and standing. Ran with bun directly: new test, `🧭️order/🧪️tests/🟦️.ts` and `📈️standing/🧪️tests/🟦️.ts` all pass (104).

## Not done
A waiter-side message when no registry entry exists for a locked resource (holder discovery via fcntl owner) remains open.
