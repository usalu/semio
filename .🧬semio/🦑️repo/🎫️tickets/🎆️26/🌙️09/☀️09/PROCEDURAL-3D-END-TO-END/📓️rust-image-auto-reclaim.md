# Rust Image Automatic Reclaim

## Problem
`.cache/b/r/m` went from 14 GiB used to 44 GiB used / 540 MiB free in about 12 hours under many concurrent `repo cargo` builds and failed builds with ENOSPC. A manual `repo cache gc` then freed 28 GiB, so most of it was reclaimable. Nothing ran automatically before ENOSPC: the admission only reclaimed when a command's reservation did not fit above a floor share, and never on the ledger's soft threshold.

## Design
- Ledger (`⚡️caching/💾️storage/🧾️ledger/🔣️policy.json`): area `cargo-store` declares `reclaim.softAvailableBytes` 12 GiB and `targetAvailableBytes` 20 GiB. A rust partition below about 44 GiB is now refused by the ledger (soft threshold must fit); the needs test was adjusted from 16 to 44 GiB.
- Honest available space (`createHonestMeasureV1`, headroom module): the effective measure of the recovery note (stale volume figure corrected by the container) capped by the free bytes of the host volume holding the backing file. It is now the default measure of the admission.
- Event-driven gate (`admitImageHeadroomV1` with `softReclaim`): before each admission decision, and on every monitor tick while a command runs, `requestRustImageReclaimV1` checks the honest available space. Below soft it reclaims up to the target, before admitting more compile work. The launcher passes the ledger rule. No timer or daemon: the trigger is the admission and the running commands' own monitor.
- One reclaim (`🧹️gc/♻️auto-reclaim/🟦️.ts`): `collectRustImageV1` is the body that `repo cache gc` ran for the rust image (incremental sessions, superseded component runs, stale units, lock-aware), moved out of the command and shared. The command now calls it under the same cross-process claim (`rust-image-reclaim`), so a manual gc and an automatic reclaim never overlap.
- Held units: the collectors skip every session or unit a running build holds (compiler lock, build lease, live profile); the gate adds no deletion path of its own.
- Single-flight: concurrent requests in a process share one promise; across agents the claim lets one run while the others wait and return `joined` as soon as the space is above soft, running nothing.
- Cancellation: each requester's signal detaches only that requester; the collector is aborted when the last requester cancelled; the claim is always released.
- Cooldown: a reclaim that freed nothing (everything held) is not repeated for 5 s.
- Bilingual progress lines (en, de): due, joined, throttled progress per collector phase, done, nothing reclaimable, cancelled (schema `🧬️schema/🔣️.json` of the module).

## Tests
`🧹️gc/♻️auto-reclaim/🧪️tests/🟦️.ts` (10): vectors checked against the schema and an independent Python oracle; ledger rule and bilingual messages; below soft runs once for the demand up to the target in en and de, above soft nothing; five concurrent requests share one collect; a request joins another agent's reclaim; cooldown and claim release; cancellation only when all requesters cancel; held session never removed with the real collectors; admission reclaims once and then admits without its own cleanup; honest measure caps at the host. Also green: headroom tests, ledger tests, cache-gc and unit-cache gc tests, launcher tests.

## Not done
No end-to-end run against the real 96 GiB image. Type errors reported by tsc in `cache-gc/⌨️command` lines 136, 224 and 228 belong to peers' regions and were not touched.

## Relative thresholds (2026-11-22)
- Ledger `cargo-store` declares `reclaimShare` (schema `$defs/reclaimShare`) instead of absolute bytes: soft 15 %, target 25 % of the partition (basis points 1500/2500), with floors 2/4 GiB applied only while 1.5x the share is below the floor (`cacheReclaimShareBytesV1`, pure). A partition no longer needs ~44 GiB: the soft threshold must fit only above the 16 GiB measured need again (needs test restored to 16 GiB).
- Resolved: 30 GiB soft 4.5 GiB / target 7.5 GiB; 64 GiB soft 9.6 GiB / target 16 GiB; 96 GiB soft 14.4 GiB / target 24 GiB. The ledger refuses a share whose soft is not below target or whose target exceeds half the volume at 1, 4, 16, 96 GiB, 1 TiB and the declared partition.
- `rustImageReclaimRuleV1(volumeBytes)` reads the share; the request resolves it against the measured volume.
- Tests: new fixture-based test (`shares` vectors in `🧪️tests/🧫️fixtures/🔣️.json`, schema `ShareVector`) checks 30/64/96 GiB (and 1 GiB floor case) against the ledger, `cacheReclaimShareBytesV1` and an independent Python oracle. auto-reclaim 12/12, needs 9/9, cache-gc 19/19 pass.
- Type errors in `cache-gc/⌨️command` fixed: partition ids typed `RepositoryCachePartitionIdV1`, and `🟨️.d.mts` declarations added for the closure-store retention module and the closure-store limits of the inference schema. tsc over the touched files is clean (a whole-repo tsc currently stops at a peer's syntax error in `💻️os/…/🚚️distribution/🏗️compiler/🟦️.ts(45,2)`).
