# g1-m14 Join (2026-11-25)

The Mac mini `Uelis-Mac-mini.fritz.box` (`192.168.178.188`, `192.168.178.130`) joined group `g1` as origin `g1-m14`. It runs macOS 27.2.1 on an Apple M4 with 16 GiB of memory and had 164 GiB free before the join. Origins `g1-m01` through `g1-m13` were live on other machines, so this Mac took a new origin. The ticket is `☀️25/PROVISION-MAC-WORKER-G1-M14`.

## Provisioning

- Case 2 of `1114-new-mac-worker-prompt.md` applied: `~/Documents/semio` was the only semio repository on the machine, and `/Users/Shared/semio` did not exist.
- The kit came from `v1/bootstrap/kit` and passed `SHA256SUMS` for all 12 files. Its publish receipt names code `8493edad8d8f…`.
- The `--plan` dry run reported 0 conflicts and 2 paths of new work (this ticket's files). It found 2467 older local versions, where the fleet wins and the local bytes are kept in `.cache/f/lost/adopt-*`. The git relation was `equal` at `8d17d8ba3998`.
- Step 2b warned that the sealed code `8493edad…` is not intact because it is missing `🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts`, so the checkout kept its own fleet code. After the join, g1-m14 attests code `8493edad8d8f`, the same as the fleet.
- The real run exited with code 3: the machine is in the fleet and syncing, and the native build is pending. The native bootstrap failed with an `Internal error: Native receiver …/.cache/b/d has neither fixed.dmg nor image.json but is not an empty shell` (reference 8918ce0b). The readiness supervisor now drives the native stages and was retrying stage 3/5 (dependencies) with back-off.
- Installed services: `tech.semio.fleet.replica` (running), `tech.semio.fleet.compact` (scheduled) and `tech.semio.repo.probe.3f08bbea4d32`. The store is mounted at `/Volumes/semio-store`, from `192.168.178.70:/Users/Shared/semio-store` over NFS.

## Checks

- `status`: the daemon is running, synchronisation is on, and the machine is 0 events behind every origin.
- `fleet`: the row `g1-m14 macos 0s 0/0 cp:0s on 8493edad8d8f … ok`. The fleet as a whole disagrees because of `g1-m04` and `g1-m13` (`view.digest`).
  - An earlier transient `guards.modeMismatch: 2` cleared by itself. No tracked file has a different executable bit.
- `git rev-parse HEAD` is `8d17d8ba3998690932b0032ea2ca41bce5c994b6`.
- `preflight` with `SEMIO_FLEET_CACHE_ENDPOINT=http://192.168.178.70:6094`: 22 of 23 checks pass or warn.
  - The cache answers `/healthz` in 34 ms, and the clock is +0.1 s from g1-server.
  - Warnings: time synchronisation needs root to read, the native build is pending, and event visibility is 17.3 s at p95 (median 1.6 s, limit 5 s).
  - Failure: worktree access. The consent for `/Users/ueli/.local/share/semio/launcher/bin/repo` was still `waiting` at the time; it was granted later (see Native readiness).

## Native Readiness

The owner granted the Documents consent. `launcher.json` then held the launcher's sha256 marker, and preflight reported `worktree access: ok`. The readiness supervisor then ran the five native stages to `ready yes`. Two defects in shared code had to be fixed first (ticket `☀️25/PROVISION-MAC-WORKER-G1-M14`):

1. **The build agent wrote into an unattached dependency partition.** `cacheAreaHomeV1` (`⚡️caching/💾️storage/🧾️ledger`) refused an unattached macOS partition only when its directory already existed. On a fresh machine the build-agent sidecar therefore created `.cache/b/d/m/remote-execution/work/<slot>/.owner` before the dependency image existed. The native receiver then refused `.cache/b/d` as "not an empty shell" on every attempt.
   - The home now refuses whenever the mount is not attached, with a `CacheAreaDetachedError`.
   - The agent then joins without a worker, with the new reason `partition-detached` (added to the `WorkerWhy` schema enum, the codes and texts, and the code-list test), and rechecks every 15 s.
   - Test: `🧾️ledger/🧪️tests/🏠️home` (fixture-driven).
   - The stale receivers were moved aside to `.cache/b/d-stranded-*` (ignored, never deleted).
2. **macOS TCC broke the preservation capsule recheck.** About 11 s after the dependency store produced the capsule `.pd59ca46d`, macOS wrote `com.apple.macl` onto the folder of the consented workspace, which moves only ctime. `recheckJavascriptDependencyPreservationsDescriptorV1` compared the full stamp, ctime included, and failed with "Exact privately produced preservation capsule required" (reference a1c5d1cf).
   - The capsule directory is now compared by `javascriptDependencyCapsuleIdentityV1` (`dev:ino:mode`, the same identity `javascriptDependencyBindingStampV1` gives directories). Its roster, payload and record hashes are still verified exactly.
   - Test: `💾️storage/🧪️tests/🧷️capsule-identity`, with fixture cases and a real xattr write on macOS.

Further observations:

- The tools stage failed twice because the memory watchdog cancelled `rustc` under pressure on this 16 GiB machine; the third attempt succeeded.
- The first workspace attempt failed with E0063/E0061 in `semio-framework-value-native-owned`. A peer changed `FieldShape` (the `text` field) and its derive (`✨️derive/🛬️receive`) at 03:14–03:15, during this hour-long build. The derive had been compiled before the change. The second attempt passed.
- Result: `ready yes 0/5`, and `.cache/f/native-bootstrap.done` exists. Preflight (with the 6094 endpoint) reports `native build: bootstrap complete` and `worktree access: ok`.

## For the Coordinator

- Without the override, preflight probes `cache.endpoint` of the topology (`http://192.168.178.70:6090`, derived from the old field). That port is closed. The g1 cache runs as the sidecar on `ports.cache` 6094, so preflight reports `FAIL cache endpoint` on every machine without `SEMIO_FLEET_CACHE_ENDPOINT`. The preflight should derive the endpoint from `ports.cache`, the same way the cache sidecar does.
- The sealed code snapshot `8493edad…` is missing the runtime entry file named above.
- The native receiver error under `.cache/b/d` is fixed at its cause (see Native Readiness, item 1). Machines whose build agent already wrote a slot there need that receiver moved aside once, after their agent runs the new code.
- All origins' views list about 5474 paths below `🔁️replica/sa/` and a root file `-inf` as staged-but-deleted. They look like stray generated output written relative to the module directory.
