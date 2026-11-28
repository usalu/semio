# H82: r-image guard floor, sealed source leases, per-project memory cap (m02/m15, m03, m06)

Scripts: `T/🧰️scripts/H82/` (`mirror.py` scratch mirror with symlinks to live, `land.py` per-area lock + base check + merge + atomic apply, `f4.patch` `f9.patch` `f5.patch`, `bases-*.txt`). All tests ran against the scratch mirror first (`bun test ./<file>` inside the mirror; symlinked neighbours are the live files).

## F4 disk guard stops stages while the sparse image has spare capacity (ready sidecar, area remote-execution)

Cause (code, not data): the in-run guard did `if (hit) kill("cancel")` the moment `free < floor`. The collection in place only ran in the other branch (`hit === null`: the low-water mark, 20 % of the image, throttled to once per 2 min). So an image that crossed the floor between two collections (m02 `r/m/u/sealed` 673 MB free vs 771 MB, m15 `r/m` 1294 vs 1311 MB, 17 MB under) was stopped with nothing tried first, and the stop never said that the image could still grow.

The floor itself is NOT derived from lane sizes: it is the declared function `min(25 % of the image, clamp(4 % of the image, 128 MB, 3 GiB))` of the image size alone. 771 MB is 4 % of a 19275 MB image, 1311 MB is 4 % of a 32768 MB (32 GiB cap) image. Now written into the schema (`floorRule`, en/de) and pinned by a test with exactly those numbers.

Fix (staged and landed, see below):
- `readyImageVerdictV1` (pure): below the floor and a collector exists -> `collect` (in place, immediately, no 2 min throttle); while it works -> `wait` (at most `floorGraceMilliseconds` 60 s); `stop` only below `floorHardBasisPoints` (50 %) of the floor (a burst a collector cannot outrun), without a collector, after the grace, or when the collection ended and the image is still below the floor.
- `readyImageGrowableBytesV1`: the sparse capacity the heal can still add at the next start (`growCapMegabytes` 32768, host free kept `growKeepMegabytes` 3072); named in the collect line and in the stop line (en/de: "can still grow by N MB: the next start grows it before the stage runs again"). Growth itself stays between stages (H66: an attached image cannot be stretched on macOS), the heal already runs before every start.
- `readyImageRowV1` builds the image row once (short/relief/collection).
- Tests (`🖼️image-guard/🧪️tests`, 12 -> 16): declared floor numbers, m02/m15 sequence collect -> wait -> stop/ok, hard floor, seeded 300-row table equal to an independent Python model (verdict + growable), growable table. Neighbours green in the mirror: image-guard loop 6, disk-guard, disk-recovery (21/0 together). `tsc --strict` on the touched files: 0 errors. Mutation check: the Python table fails when the verdict is changed.

## F9 sealed source copy retired while a build script used it (unit cache, area caching-cargo)

Cause: the only remover of `<cache>/sealed/<digest>` is `collectV1` (newest 4 + used within 24 h). "Used" was only the directory mtime at seal time; a build that runs longer than 24 h, or a resident launcher that sealed newer snapshots, left the older snapshot an eligible victim although a running Cargo (and its build scripts, which run `RUSTC_WRAPPER` -> shim -> sealed script) still pointed into it. No log line named what was retired.

Fix:
- `leaseSealedV1(root, digest, {pid})`: `<store>/.leases/<digest>/<pid>.json`. `sealModulesV1({lease:{}})` leases the sealed directory for the sealing process; `sealedUnitCacheV1` (the launcher of every Cargo run) always leases; the detached collector child is leased under its own pid. A lease protects while the pid lives and it is younger than 48 h (`leaseMaximumMilliseconds`, a hung build cannot pin forever); a process keeps its newest 16 leases (`leasesPerOwner`); stale leases are dropped by the collector.
- `collectV1` never retires a leased snapshot, requires the H64 root capability (`requireNativeStorageRootCapabilityV1(store, "retire-sealed-snapshot")`: a test runner can never retire a live root), and writes one bilingual line per retirement or kept-because-leased to `<store>/retired.txt` (digest, idle since, bounded 16 KiB).
- A directory a peer retired between the publish and the lease is sealed once more.
- H41 tie: same idea as `activeBuildMilliseconds` for units (proof of use is a lease, not a timestamp).
- Tests (`🏛️seal/🧪️tests`, 10 -> 13): leased and alive snapshot kept at any age, dead pid / expired lease retired, bilingual `retired.txt`, per-owner lease bound, and a 24-step history replayed by an independent Python model with and without leases (with leases keeps more; mutation `if(false)` fails two tests). Neighbours: whole unit-cache suite in the mirror 41 pass / 1 fail (`waiters of a full slot pool block on events and use almost no CPU`, CPU load timing, not touched by this lane).
- Not proven: that the m03 incident was exactly a >24 h retention (one occurrence, evidence gone). The lease closes every retirement route in the code; a seal set that never contained the file (a dynamic import the capture does not see) would not be helped, the new `retired.txt` makes the two cases distinguishable next time.

## F5 m06 `repo-script-providers (about 4200 MB)` vs `(4096 MB)` per project

Graph/inference code (seal window): `🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` + schema of `🧠️memory/🪜️plan-stage`. NOT landed; staged with tests (`f5.patch`, bases in `bases-f5.txt`), to land in the first slot after the coordinator lifts the window.

Cause: two caps that contradict each other and neither follows the host.
1. `farmChildFootprintMegabytes.project-farm.aloneCapMegabytes = 4200`, a constant: a child running alone is stopped at 4200 MB footprint on every host above 9.2 GB (`farmAloneCapV1 = min(4200, max(1400, 60 % of host - 1300))`). Recording that stop stores 4200+ as the measured cost of the owner (a lower bound, i.e. the padding is the cap, not a measurement).
2. The owner limit (`soloLimitBytes`, `neverHere`) is `max(grant, min(soloGrant(host, AVAILABLE memory now), 90 % of plan limit))`; on a busy 16 GB worker the available-memory term is small and the floor 2048 MB managed x factor 2 = 4096 MB wins. 4096 < 4200: an owner that its own child may legally grow to 4200 is refused as "does not fit even alone", while the same host can give one owner 9 GB (`soloGrantV1` by host = 4505 managed = 9010 MB footprint).
Fix: (a) `aloneCapMegabytes` becomes the declared maximum 16384, so the host term decides: 8530 MB on 16 GB, 3615 MB on 8 GB, 16384 MB from about 29 GB; (b) `aloneFloorBytesV1` = min(host solo grant, the child alone cap in managed bytes) is a floor of `hereLimitBytesV1` (new `aloneBytes`), so a busy moment can raise the limit but never lower it below what the owner's own child may take.
Tests (`🐘️never-here`, 5 -> 7; 3 expectations moved from "stopped at 3000/2400 managed = 6000/4800 MB" to 5000 managed because the cap is now 8530): m06 case on a 16 GB host with 5000 MB available (limit >= 4300), alone floor equals a Python model over 7 host sizes. Neighbours in the mirror: heavy-refusal, set-aside, cap, alone-fresh, owner-pool heavy (18/0), too-heavy 7, bigger-host 3, aside-census 3, retry-alone 2. `🛟️farm-isolation` fails 5 of 5 identically on the untouched live tree (concurrent-edit churn in the closure compiler), not caused by this.
Unverified: no live run on m06; raising the child alone cap lets one owner take up to 8.5 GB footprint on a 16 GB host (the lean plan ceiling the slice guard already judges); the memory warden stays the backstop.

## Landing
See the end of this file (appended after the 13:00 slot).

## Landed 13:00-13:03 (landings.md "Wave 1127 H82")
F4 (remote-execution), F9 (caching-cargo; the unit-cache schema merged cleanly with a peer edit), F5 (graph, seal window lifted 13:00). Live after landing: image-guard 22/0 (with loop test), seal 13/0 (one earlier run showed 1 fail, two reruns 0: timing flake, not reproduced), never-here 7/0, heavy-refusal, too-heavy 7/0 after its three `soloLimitBytes` expectations were raised to the alone floor (an intended change). dangling-imports clean. `repo typecheck` not run live (tsc --strict on the mirror: 0 errors). Not verified live on a worker: ready in-run wiring (needs a real mounted image dip), m03 sealing, m06 read.
