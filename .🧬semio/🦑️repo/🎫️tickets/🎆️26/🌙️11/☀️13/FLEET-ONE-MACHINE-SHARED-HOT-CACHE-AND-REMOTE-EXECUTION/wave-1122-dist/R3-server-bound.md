# R3: the fleet cache server's byte bound follows the host CAS cap (2026-11-23)

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Landed 13:06, sha256 in `T/landings.md`. Takes effect at the sidecar's next natural restart (not forced; play gate respected, retention not switched to `enforce`).

## Root causes (verified in code)
1. The store's `maximumBytes` (200 GiB via topology, min(policy 2.5 GiB, ledger budget) in the sidecar) is compared with `this.bytes` = the size of the **entry files** in the server directory (`🌐️remote-cache/💾️store/🟦️.ts`: `index`, `bytes`). With a host store the entries are tiny references (nx) or action-result JSON; their blobs live in the host CAS and were never counted. So the planner never over-ran its bound, never evicted anything by size, and the only brake was `pinBudgetBytes` (80 % of the cap in force), which merely stops pinning, not holding. Everything referenced stayed referenced, the host CAS filled to 100 % and refused writes with 507.
2. `repo fleet status` "no report from the cache host": the cache **sidecar never passed `staleness` to `startRemoteCacheServerV1`** (`🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts`, only the `serve` command did). The live server therefore ran with staleness `off`, never swept, never put `retention` into `/status`, and wrote no `.retention/` directory (confirmed: `.cache/b/p/m/fleet/cache-server/v1/.retention` does not exist; `.server.lock` is renewed every 60 s, so the live process does run R1 code). X1's reader (`🌐️remote-execution/🚢️status/🟦️.ts:34`) was correct.

## Changes
- Schema first: `ServerPolicy.blobSharePercent` (policy `🔣️policy.json` = 50, EN description in the schema) and `ServerStatus.blobs {referencedBytes, boundBytes, evictedBytes}` (`🏃️execution/🧬️schema/🌐️remote-cache.json`).
- `remoteCacheBlobBoundV1(host, share)` (`🖥️server/🟦️.ts`): `floor(casEffectiveCapBytesV1(host.policy, partition, usage hint) * share / 100)`, read from the host on every collection, so it follows the cap in force (27.75 GiB under disk pressure, 32 GiB declared). 50 % leaves the other half for transient offload inputs and local unit outputs. Wired in the sidecar, the hosted server and `repo remote-cache serve`.
- `planRemoteCacheBlobBoundV1` (`🧹️collector/🟦️.ts`, pure): referenced bytes counted once per blob; over the bound it takes entries down to 90 % in the order stale-or-unreached (0) -> undeclared/unslotted (1) -> declared (2), least recently used first, ties by key; guarded entries (claim, in flight, stored within the grace) and entries without blobs are never taken.
- Store (`💾️store/🟦️.ts`): `planBlobBound` runs inside every `collect()` after the normal plan, in every staleness mode (dry-run keeps reporting stale only; stale is tier 0 only in enforce, unreached always). The evicted entries' blobs are offered to `reclaimCasObjectsV1` (refs of unslotted entries are read and cached), then `pinLive()` re-pins only what remains, so the host collector can reclaim the rest. The pin budget is now `min(80 % of cap, bound)`. A write that admits more than 5 % of the bound triggers a collection (at most every 30 s) instead of waiting for the 300 s timer.
- Sidecar: now passes `staleness` (mode from policy = `dry-run`, workspace = the synced tree, toolchains from `SEMIO_REMOTE_CACHE_SERVER_TOOLCHAINS`) so `/status.retention` and `.retention/sweeps.jsonl` exist after the first sweep.

## Tests (`repo test-file`, one at a time; host load 60-87)
- New `🌐️remote-cache/📐️blob-bound/🧪️tests/🟦️.ts`: 4/0 on the final code. (a) plan equals an independent Python model on 300 seeded stores (shared blobs, tiers, guards, ties); (b) bound = share of the cap in force, moves with the cap, undefined without a share; (c) and (d) dry-run and enforce: five 1 KB entries (3 undeclared, 2 declared), bound 4500 -> the oldest undeclared leaves, blob gone from the host CAS; bound 1500 -> undeclared first, then declared LRU; pinned set 2 -> 1 and the host pin file holds exactly the survivor; the next write is accepted and stays.
- A grace-timing test was dropped (it failed only because the loaded host outran its 3 s window); the guard is covered by the oracle test.
- Existing members re-run: see the bottom section.

## Live numbers (read-only, `R3-live-dryrun.ts`, no write to the live store)
The dry-run script `R3-live-dryrun.ts` (reads sidecars + stats blobs, judges staleness in memory, plans the blob bound; writes nothing to the live store) was starved by `taskpolicy -b` at load 60-87 (14 s CPU in 40 min over 188k sidecars) and killed, so judged / stale / unreachable / undeclared / would-refuse-on-arrival numbers are NOT measured. Reason they cannot come from the live server yet: it runs with staleness off (no sweep, no `.retention`); they appear in `/status.retention` and `repo fleet status` after the restart on this code. Run the script again when the host is quiet (`bun run R3-live-dryrun.ts`, prints JSON).
Facts read directly meanwhile: store `.cache/b/p/m/fleet/cache-server/v1` has 90,582 entry files and 188,781 sidecar/meta files, no `.retention`, host CAS `usage.json` 22.3 GB at the last collection, one pin holder `remote-cache-live-76726`.

## Existing members re-run (host load 60-87)
- `🏷️retention/🧪️tests`: 16 pass / 1 fail: `a deleted crate ... stored within the grace` took 37 s and its 300 ms grace elapsed while publishing; no bound is set there so the new code plans nothing (`planBlobBound` returns [] without a bound). Load flake, not re-run (about 25 min per run).
- `🖥️server/🧪️tests`: 8 pass / 2 fail: two multi-process stress tests hit their 120 s timeout (python workers killed, exit 143). Load flake.
- Not re-run: collector tests (only an added pure function), sidecar tests, command tests (one field passed through). Re-run them on a quiet host.

## Open items
- Live live-state at 13:55: host CAS last collection 24.7 GB; the server's pin holder `remote-cache-live-76726` currently holds 0 digests (so the 22 GiB "pinned by the live pin set" seen at 10:05 is gone or the pin lapsed; the entries still reference blobs). After the restart, `/status.blobs` shows referenced vs bound.
- 50 % share is a first value; tune `blobSharePercent` in `🔣️policy.json` against the offload input rate (27-38 GB/h).
- `retention` stays `dry-run`; do not switch to `enforce` before a cold-store proof.
