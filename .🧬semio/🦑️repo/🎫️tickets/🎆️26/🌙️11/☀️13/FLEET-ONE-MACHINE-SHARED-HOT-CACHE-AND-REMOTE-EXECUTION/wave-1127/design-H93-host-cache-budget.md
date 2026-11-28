# H93: One host-wide cache budget (design, 2026-11-27 18:05)

Status: design plus scratch prototype in `T/🧰️scripts/H93/` (no product file touched, FULL BUILD-SYSTEM FREEZE respected). Prototype: `allocator.ts` (pure function), `oracle.py` (independent Python model + corpus generator), `allocator.test.ts` (bun, 4 tests), `simulate.ts` (today's `partitions.json`).

## 1. Problem (evidence)
- Every purpose gets a fixed envelope born from a declared table: `r` 26-96 GiB, `d` 27-36, `p` 9-31, `g` 6-12, `t` 8, `n` 1.5, `o` 2, `l` 0.06. Σ declared is 95-187 GiB per worker (m01 179.6, m02 about 180) on a 228 GiB host with 10-35 GiB free (H34). Measured content (lower bound, scans partial) is 28-41 GiB on the busy workers (m01 40.6, m11 28.5). Envelopes over-promise 3-4x while single images still hit ENOSPC (m12 `r` 47 MB free, m11 lane shard 79 MB, m12 cargo shard 0 MB: H34, H47) because growth is per image and the host's free bytes are nobody's budget.
- Oversize images (`r` 40-96 GiB, `d` 36 GiB) cannot unmap on macOS (2^35 cap, 11/25 ticket CAP-IMAGE-SIZE...), so freed blocks stay physical (H47 point 3); retirement exists only as a quiescent exclusive-lease cycle that times out under load.
- No image can grow while attached on macOS (`hdiutil resize` status 35, `diskutil image resize` busy: H66); growth is only between stages.
- Eviction is per purpose with per-purpose high/low water (H41 `planUnitsByImageV1`); nothing compares an `r` unit idle for a week with a `g` selection used five minutes ago, and the CAS (95-98 GiB on the coordinator, H17) and the 139 GiB Play image live outside any shared account.

## 2. Principles
1. One number decides: `budgetBytes` for all purposes (r d g l n o p t and CAS) of one host, derived from disk total, free bytes and a reserve. Purposes do not own bytes, they own floors.
2. Images remain the allocation and isolation unit (inode isolation, no spotlight index, per-image unmap), but their capacity is an output of the plan, not a declaration. Capacity tracks use: `commit(u) = alignUp(max(floor, u + u/4), align)`.
3. Oldest-first across purposes, weighted by rebuild cost: evict the bucket with the highest `age / rebuildCost` first. Greedy by that density is optimal for the fractional problem "free X bytes, lose the least expected rebuild work" when reuse probability falls like 1/age.
4. Never evict what is in use (bucket 0, last 10 min) or below a purpose floor; if the budget cannot be met, the plan says `unmet` (typed, bilingual), never silently overcommits.
5. Resizing only at quiescence (macOS/Windows), online on Linux; the manager never blocks or fails a stage.
6. Pure planner, impure executor: the planner is a deterministic function over a snapshot (testable in any language); the executor applies actions through the existing storage entry points under the root capability.

## 3. Schema sketch (schema first, JSON; names follow existing `…V1`)
```
HostCacheBudgetV1 { host: {totalBytes, freeBytes, platform}, policy: PolicyV1, purposes: PurposeV1[] }
PolicyV1  { reserveFloorBytes=10GiB, reserveFractionPercent=8, maxSharePercent=60,
            headroomNum/Den=1/4, imageCapBytes(platform: darwin 2^35, linux 128GiB, win32 64GiB),
            shardFillNum/Den=7/8, resizeCooldownMs=1_800_000, shrinkSlack=1.5x and >=2 aligns,
            evictFractionPerCycle=25% (executor-side, not in prototype) }
PurposeV1 { name: r|d|g|l|n|o|p|t|cas|pinned:<owner>, floorBytes, alignBytes, costMsPerGiB,
            peakBytes, imageless (cas), pinned (play image: counted in physical, never evicted),
            buckets: bytes[6] by last use (<=10m protected, 1h, 6h, 24h, 7d, older),
            images: [{id, capacityBytes, usedBytes, physicalBytes, unmappable, holders}] }
PlanV1    { budgetBytes, reserveBytes, physicalNowBytes, unmet, deficitBytes,
            evictions: [{purpose, bucket, bytes}],
            purposes: [{name, usedBefore/After, commitBytes, targetBytes, shards[]}],
            actions: [{kind: keep|grow|shrink|birth|retire|quota, purpose, image, fromBytes, toBytes}] }
```
Typed refusal (en/de): `budget-short` "Host cache budget is short by {x} GiB; in-use data and floors alone exceed the budget." / "Das Cache-Budget des Rechners ist um {x} GiB zu knapp; Daten in Benutzung und Untergrenzen übersteigen es allein."; plan line `cache.budget.plan` "{n} images, {e} GiB to evict, {g} grow, {s} shrink" / "{n} Images, {e} GiB zu räumen, {g} wachsen, {s} schrumpfen".

## 4. Budget derivation (exact, integers)
```
reserve = max(reserveFloor, total * 8 %)                 // 18.2 GiB on a 228 GiB host
budget  = max(0, min(total * 60 %, physicalNow + free - reserve))
```
`physicalNow` = Σ host-allocated bytes of all cache images + CAS tree + pinned images. Free above reserve is the only growth room; free below reserve makes the budget smaller than today's physical use, which forces eviction and shrinks (pressure response, proven in test "host pressure"). The 60 % ceiling keeps user data/Docker/Play (H17: 100+ GiB foreign) from being crowded out on small disks. `free` is smoothed (EWMA over 10 min, min of the last 3 samples for the growth side) to avoid flapping.

## 5. Allocation (what the prototype implements)
1. `used_p` = Σ buckets. `commit_p` as above. If Σ commit ≤ budget: no eviction.
2. Otherwise candidates = all buckets 1..5 of all purposes ordered by `age_b / cost_p` descending (cross-multiplied integers, ties: purpose name, older bucket). Evict chunks of `max(align, deficit/(1+headroom))` from the head until Σ commit ≤ budget. Bucket 0 and floors are never touched; remaining deficit = `unmet`.
3. Spare budget (budget minus Σ commit) is handed out proportionally to each purpose's gap to its measured peak (`commit(peak) - commit`), aligned down; this is how `r` gets room for its 31-47 GiB peaks while `g` stays small.
4. Layout per purpose: `n = ceil(target / alignDown(cap * 7/8))` images, sizes equal in align units (Σ exactly = target). Sharding is therefore automatic when a purpose exceeds the 32 GiB unmap cap (H47: 87.5 % of cap, H41 `imageShardPlanV1`).
5. Actions per image: `grow` if the target exceeds capacity; `shrink` only if unmappable and capacity > 1.5x target and gap >= 2 aligns; an image above the platform cap is `retire` plus `birth`s (reshard); surplus images retire; CAS gets `quota` (the cache server's blob bound is set from the target, no image).
6. Rebuild cost `costMsPerGiB` = EWMA of measured stage wall time per GiB produced (ledger already records stages), lowered to the fleet fetch time per GiB when the fleet cache holds the key (`min(local, fetch)`), so units the fleet can refill cheaply are evicted first. Defaults used in the simulation (assumption, to be measured): r 900 s, d 120 s, g 300 s, p 240 s, t 60 s, n 400 s, o 30 s, l 1 s per GiB.

Refinement not in the prototype: sharding is by subtree (cargo home, components, lane `u/l/<id>`: H47), so `shards[]` should be a first-fit-decreasing pack of subtree weights instead of an even split; the planner input then gets `subtrees[{path, bytes}]`.

## 6. State machines
Image (per shard): `Born -> Attached(idle) <-> Attached(held) -> Quiescent -> Detached -> Resized -> Attached`; `Quiescent -> Draining -> Retired`; transitions into `Quiescent` need the partition lease nonwaiting plus holder count 0 (H66); busy = `wait`, never retry loops. `grow` while `held` is deferred to the next quiescent point between stages (H66); in-place collection (H34/H47 low-water) still runs while held.
Manager (one per host): `Idle -> Measuring (census buckets) -> Planning (pure) -> Evicting (per image collectors, in place) -> Resizing (quiescent images only) -> Verifying (re-measure, ledger event) -> Idle`. Triggers: stage boundary, image-guard low-water, free-space drop of > 5 % total, 10 min timer. Cancellation and progress at every arrow (expensive operation rule). Cooldown `resizeCooldownMs` per image against thrash.

## 7. Leases and root capability
- Planning takes no lease and no capability (pure, read-only snapshot).
- Applying: eviction runs through the existing unit/gc collectors (H41 per-image plan) under their unit locks; resize/birth/retire take the partition lease nonwaiting plus the shard lease and call the H64 `requireNativeStorageRootCapabilityV1` first (same 13 destructive entries). The manager is a production-only entry: under a test runner (`NODE_ENV=test`, `SEMIO_TEST_*`) live `.cache/b/*` is refused and only sandbox roots issue a capability, so tests of the manager use sandbox/tmp images.
- H63 guard stays decisive: `requireNativeImageUnlinkedV1` refuses retiring `d` while the checkout `node_modules` symlink resolves into it; a `d` reshard therefore births the new shard, relinks, then retires the old.
- A held profile lease no longer pins evictable units (H41), so the manager can evict across lanes without the exclusive lease; only `Detached` resize needs quiescence.
- Pinned purposes (Play image, 139 GiB fixed, H17) enter as `pinned`: counted into `physicalNow`, floor = used, no actions.

## 8. Platform equivalents
| Concern | macOS (APFS in DMG/sparse) | Linux | Windows |
|---|---|---|---|
| Image | sparse bundle / DMG, APFS volume | sparse ext4/xfs loop file or directory (devcontainer: bind dir, no image) | VHDX dynamic, NTFS/ReFS |
| Cap for unmap | 32 GiB (2^35) | 128 GiB (schema) | 64 GiB (schema) |
| Grow | detached only (`hdiutil resize` fails attached, H66) | online: truncate, `losetup --set-capacity`, `resize2fs`/`xfs_growfs` (H66 port, fake-runner test only) | dismounted `Resize-VHD` |
| Shrink/return blocks | in-place kick (sibling volume) below cap; else detach+compact | `fstrim` mounted; punch-hole on loop file | `Optimize-VHD -Mode Full` dismounted; `defrag /L` retrim before |
| Isolation | no-index flag, own inode space per image | own filesystem | own volume, indexing off |
The planner is platform-neutral; only `imageCapBytes`, `unmappable` and the executor differ. A devcontainer/native Linux without loop privileges degrades to directories (no isolation): then `imageless` quotas replace images and the same plan drives eviction only.

## 9. Greenfield cutover (no migration, no compatibility path)
Single post-seal landing, handcrafted: (1) add schema `HostCacheBudgetV1/PlanV1` and the planner (port of `allocator.ts`, same JSON corpus as language-agnostic test, Python oracle as third-party-free cross check); (2) census emits the 6 buckets in the scan it already runs (units: fingerprint mtime via `mtime-on-use`, d: generation adoption stamp, g: selection read stamp, p/t/n/o/l: file mtime, CAS: access stamp); (3) manager wired to stage boundaries; (4) delete the declared per-purpose capacity table, `elastic.*.maximumBytes`, `pinnedCapacityV1` envelopes: purposes declare only `{floor, align, costClass}`; (5) first run per worker: oversize images retire via the existing quiescent regrow path and are reborn by the plan (cache refilled from the fleet cache; nothing is converted). `repo cache budget` prints the plan (`--apply` default on workers; plan-only for humans). Order of risk: grow-only -> evict -> shrink -> reshard.

## 10. Simulation over today's partitions.json (14 workers, host 228 GiB assumed)
See `result-H93.md`. Takeaways: Σ declared 1996 GiB vs Σ budget 486 GiB at 25 GiB free; 23 shard births (every oversize `r`/`d`/`p`), 3-7 shrinks per worker, evictions only when free < reserve; at 12 GiB free (below the 18.2 GiB reserve) 6 of 14 workers cannot meet the budget (typed `budget-short`), which is the honest answer the fixed images hide until ENOSPC.

## 11. Risks and open points
- Census is a lower bound where the scan is partial (d shows 1.8 GiB while its working need is 29 GiB per the 11/25 ticket); the planner must treat `partial` as `peakBytes` floor, never as small use. Last-use histograms are synthetic in the simulation (two profiles).
- Reshard of `r` loses the contents of the old image unless units are copied; path-dependent lane layout (`u/l/<id>`) makes a content move invalid, so reshard = rebirth plus refill from the fleet cache (cost = cheap-fetch share of `costMsPerGiB`).
- Thin images keep freed blocks above 32 GiB, so `physicalNow` can exceed `used`; the prototype uses 1.05x (1.3x oversize) assumptions.
- Oscillation: hysteresis (1.5x shrink slack, cooldown, EWMA free). Starvation: floors plus 25 % per-cycle eviction limit; `r` cost weight must not make the `g` floor disappear (floors are hard).
- Not verified live: any apply path, Windows/Linux executors, ASIF (H66 side finding).
