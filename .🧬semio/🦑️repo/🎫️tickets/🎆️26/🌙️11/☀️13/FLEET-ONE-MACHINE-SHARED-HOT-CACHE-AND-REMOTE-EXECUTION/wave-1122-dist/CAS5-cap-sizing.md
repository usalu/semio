# CAS5: the cap in force is sized per machine role from the partition (2026-11-24)

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.

## 1. How the cap in force was derived (old rule) and why it was 27.75 GiB with 175 GiB free
`casEffectiveCapBytesV1` (`LIB/🗄️cas/🌡️partition/🟦️.ts`), old: `min(capBytes 32 GiB, align_down(max(1 GiB, min(share, room)), 256 MiB))` with
- `share = capHostSharePercent (3) % of the partition` = 3 % x 926 GiB = 27.78 GiB -> aligned down to the 256 MiB step = **27.75 GiB**. This was the binding term.
- `room = store bytes + max(0, free - host floor)`, host floor = `min(15 GiB, 6 % of the disk aligned)` = 15 GiB (`machineHostFloorV1`, `⚡️caching/💾️storage/🧾️ledger/🎚️machine/🔣️policy.json`). With 175 GiB free the room is ~187 GiB, so it never bound; it only shrinks the cap under real pressure.
- The declared 32 GiB (`Policy.capBytes`) was above the share, so it did not bind either.
- Not inputs of the cap: the machine budget "43 of 198 GiB" (`SEMIO_CACHE_BUDGET_BYTES`, machine file `🎚️machine-budget.json`, default 198 GiB) splits the cache IMAGES (rust, products, tools ...); the CAS only borrows its `hostFreeFloorBytes/Percent` and `alignBytes`. Neither override changes the cap. No machine file exists in the cache root.
- Watermarks (`highWatermark 0.9`, `lowWatermark 0.75`) are fractions OF the cap in force (collection starts at 90 % of it, frees to 75 %), not inputs. `blobSharePercent 50` (R3) took half of it for the server: 13.9 GiB.
So the number was a fixed 3 % of the partition: a magic share that ignored what the machine must hold (play release cache 11.6 + server 10.1 GiB already > 27.75).

## 2. New sizing (schema first, `LIB/🗄️cas/🧬️schema/🔣️.json` `Policy.sizing`; types in `🧬️schema/🟦️.ts`)
Roles (EN/DE-free data): `worker` (3 % share, 0 releases), `builder` (6 %, 1 release), `cache-host` (6 %, 2 releases + server).
Terms: `transientBytes` 4 GiB (offload inputs), `releaseBytes` 16 GiB (one full semio-tech release cache, 12-15 GiB today plus growth), `serverBytes` 24 GiB (fleet cache server blobs; 2 x R3's 50 % share means the bound is 30 GiB >= 24).
`floor(role) = transient + releases x release (+ server)`: worker 4, builder 20, cache-host 60 GiB.
`cap = min(capBytes ceiling 96 GiB, align_down(max(1 GiB, min(max(share% x partition, floor(role)), room)), 256 MiB))`,
`room = store + max(0, free - headroom)`, `headroom = max(host floor, reserve below the partition's HIGH watermark)` (derived reserve 12 GiB on derivable partitions, else 10 % of the disk): the store's own growth can never push the partition over its high watermark, and the cap shrinks with real pressure (CAS4 then evicts unreferenced first, unchanged).
Machine role (`readCasRoleV1`): `SEMIO_CAS_ROLE` env if set, else the strongest live record `<cas root>/v1/r/<role>.json` (7 day TTL), else `defaultRole = builder`. The fleet cache server declares `cache-host` at start and renews it every `holderRenewMs` in its enforce tick (`🌐️remote-cache/🖥️server/🟦️.ts`). The role is read where the partition is measured (`measureCasPartitionV1` returns `role`), so guard, collector, `cas status`, fleet status and R3's blob bound all agree. R3's bound (`remoteCacheBlobBoundV1`, 50 % of the cap in force) and R4's graph reserve (10 % of the bound) follow automatically; no code change there. Workers are not yet auto-declared: set `SEMIO_CAS_ROLE=worker` in worker provisioning (open item).

## Resulting numbers (store not under pressure)
| machine | role | cap in force | server bound (50 %) | graph reserve (10 % of bound) |
|---|---|---|---|---|
| coordinator, 926 GiB | cache-host | 60 GiB (share 55.6 < floor 60) | 30 GiB | 3 GiB |
| 256 GiB builder | builder | 20 GiB | n/a | n/a |
| 256 GiB worker | worker | 7.5 GiB (3 % = 7.68 aligned) | n/a | n/a |
Under pressure (926 GiB, 33 GiB free, store 41.6 GiB): 59.5 GiB, shrinking further as free falls below the 15 GiB headroom. Was: 27.75 / 13.9 / 1.4 GiB.

## 3. Tests
- `LIB/🗄️cas/🧪️tests/📋️capacity/🔣️.json` (language-agnostic vectors: 4 roles x 6 sizes x 8 free x 5 stores, frozen `expected`), `🐍️capacity.py` (independent Python oracle). cas `🧪️tests/🟦️.ts`: 51 pass / 0 fail (replaced the old cap test: TS == oracle == frozen vectors; headline numbers; default role; server share; new role-record test). holders 12/0, blob-bound 4/0, fleet-status 18/0.
- remote-cache server lag test: see landings.md.

## 4. Restart / activation
The cap in force is evaluated by every process that opens the CAS. CLI (`repo cache cas status`) uses the new code at once, with role from the record. The cache sidecar (6094) must restart by itself (not restarted by this lane) to start declaring `cache-host` and to apply the new cap/blob bound; until then the record is absent and the role resolves to `builder` (20 GiB cap), so the CLI shows 20 GiB on the coordinator until the sidecar declares itself. Optional immediate bridge: `SEMIO_CAS_ROLE=cache-host` in the coordinator's environment.

## 5. Live read (`repo cache cas status`, once, 02:5x)
"Cap in force 60 GiB for role cache-host (declared ceiling 96 GiB): collection from 54 GiB in steps of 2 GiB down to 45 GiB." Store 30.76 GiB (45,339 objects); referenced 27.72 GiB (server pins 14.91, play unit cache 11.11, transient inputs 6.07), 3.03 GiB by nobody; partition 81.9 % used, 167 GiB free. The role record `v1/r/cache-host.json` was written once by hand (the sidecar had not restarted yet, so without it the CLI would have resolved to builder = 20 GiB; running new-code collectors would then have judged the 30 GiB store over cap). The sidecar still has to restart by itself to renew the record (TTL 7 days) and run the new cap/blob bound in-process.
