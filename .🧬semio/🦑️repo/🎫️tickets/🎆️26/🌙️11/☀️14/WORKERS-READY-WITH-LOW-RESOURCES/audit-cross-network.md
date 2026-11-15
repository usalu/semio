# Cross-Network Audit of the Shared-Worktree Fleet

Read-only audit. Observed at 2026-11-15 05:28:16 local time (the newest attestation read). No git command, build command, network call or other file was touched.

Paths: `SW` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree`, `B` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `STORE` = `/Users/Shared/semio-store/v1`, `F` = `/Users/Shared/semio/.cache/f`.

## 1. Origins in the store, attestation age, code digest

Source: `STORE/origins/<origin>/attest.json` (field `wallMs`, `code`). Reference digest is the g1-server one, `cfb539bbd473fea1…`.

| origin | platform | last attest (local) | age at 05:28:16 | code digest | agrees with g1-server |
|---|---|---|---|---|---|
| g1-server | macos | 2026-11-15 05:28:16 | 0 s | cfb539bbd473 | reference |
| g1-m01 | macos | 05:27:46 | 30 s | cfb539bbd473 | yes |
| g1-m02 | macos | 05:28:15 | 1 s | cfb539bbd473 | yes |
| g1-m03 | macos | 05:28:16 | 0 s | cfb539bbd473 | yes |
| g1-m04 | linux | 05:28:15 | 1 s | cfb539bbd473 | yes |
| g1-m05 | windows | 05:28:08 | 8 s | cfb539bbd473 | yes |
| g1-m06 | macos | 05:28:16 | 0 s | cfb539bbd473 | yes |
| g1-m07 | macos | 05:28:15 | 1 s | cfb539bbd473 | yes |
| g1-m08 | macos | 05:20:16 | 8 min | 97445096cc0a (old) | no |
| g1-m10 | macos | 05:26:26 | 2 min | 97445096cc0a (old) | no |
| g1-m09 | not known | no `attest.json` | never attested | not found | not found |

Findings:
- g1-m05 (Windows) is **not** stale. Its attestation is 8 s old and carries the reference digest.
- Stale: **g1-m08** (last attestation 05:20:16, about 8 min behind) and **g1-m10** (05:26:26) still carry the older digest `97445096cc0a4394…`. `STORE/origins/g1-m10/attest.json` has `"code":"97445096cc0a…"`; `g1-m08/attest.json` has the same. Their workers log the mismatch, for example `workers/g1-m10/round-18.agreement-wait.txt` lines 2-5 (04:26:01 to 04:29:02: `m10=97445096cc0a … srv=cfb539bbd473`). The exact time the server moved to `cfb539…` is not found.
- **g1-m09** has only `STORE/origins/g1-m09/fleet-auth.json` (mtime 2026-11-14 02:13; `createdAtMs` 1794618785499). It has never written an attestation (`not found`). The directory was not present in the last attestation sweep. Its folder is `drwx------`.
- Store origins: only the g1 origins exist (`g1-m01` … `g1-m10`, `g1-server`). No `g2`, `g3`, `g4` or `g5` origin is present. This matches the relay finding in section 2.

## 2. Relay to g2 and g3 (and g4, g5)

- Relay process is alive: PID 57312, started `Sat Nov 14 09:44:57 2026`, command `… script.ts relay --peers g2=/Volumes/semio-peers/g2,g3=/Volumes/semio-peers/g3,g4=…,g5=…` (`ps -p 57312`). Its launchd definition is `/Library/LaunchDaemons/tech.semio.fleet.relay.plist`, arguments on line 8.
- Mounts: **not mounted**. `mount | grep -i peers` returns nothing. `/Volumes/semio-peers/g2` and `/g3` are empty root-owned directories (`total 0`, created `Nov 11 17:10`). The g4 and g5 directories are the same. Each peer path is a plain local folder, so the relay reads an empty directory.
- Last successful exchange per peer: **not found**. The relay log has no per-peer success event at all (no `applied` or copy event).
- Errors in `/var/log/semio-fleet-relay.log` (35,750 lines; mtime 2026-11-15 04:50):
  - 3 `started` and 2 `stopped` events.
  - 12 `store-unreachable` events, 4 per start, for all four peers (lines 136-139, 25736-25739, 34966-34969), for example `{"version":1,"event":"store-unreachable","groupId":"g2","path":"/Volumes/semio-peers/g2"}`.
  - 6 `{"event":"error","stage":"bootstrap","message":"ENOSPC: no space left on device, write"}` (lines 33871-34352). Free space on `/System/Volumes/Data` now is 107 GiB (89 % used).
  - Repeated `Unexpected error: stat /Users/Shared/semio/.cache/b/d/m/j: no such file or directory` (lines 130-133).
- The log has no per-line timestamps, so the time of each `store-unreachable` line is `not found` beyond the file mtime.

## 3. Group- or network-specific design (does not work across networks today)

Hard-coded LAN, DNS and port values:
1. `SW/🗺️topology/🔣️.json` lines 6-10: `nfs.gN.semio.internal`, `cache.gN.semio.internal:6090`, worker networks `10.0.N.0/24`, peers at `/mnt/semio-peers/gN`. The hostnames are placeholders with no DNS entry found.
2. `SW/🗺️topology/🟦️.ts` line 128: `MACOS_PEERS="/Volumes/semio-peers"`. Line 180 builds the peer path from it. Line 294 builds the peer mount as `nfs4` (Linux) or `nfs` v3 (macOS/Windows) from `nfs.host`.
3. `SW/🗺️topology/🟦️.ts` line 13: `FleetServerV1.protocol: "nfs"|"smb"`. Lines 155-164: client protocol is `nfs4` or `nfs3` only (`clientProtocolV1`). NFS-only transport, with SMB only for Windows server.
4. `SW/🔁️replica/⌨️command/🟦️.ts` lines 352-358: `relay` takes `--peers` or `group.peers`, which are filesystem paths. `SW/🔁️replica/🟦️.ts` lines 2995-2998: `homedOriginsV1(store, group)` lists a local directory. The relay therefore has no network transport at all; it needs the peer store as a mounted path.
5. `SW/README.md` line 110, 358, 381-383: ports 2049 (NFS), 445 (SMB), 111 (rpcbind) and 6090 must be reachable; peer mounts are at `/Volumes/semio-peers/<id>`.
6. `B/🔣️policy.json` lines 33-35: `"host": "127.0.0.1", "port": 6090, "fleetPort": 6094`. Lines 500-501: scheduler `port 8980`, `workerPort 8981`.
7. `B/🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts` lines 33-36: the fleet network is `SEMIO_FLEET_NETWORK`, else the topology row's `server.network`.
8. `B/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts` lines 225-231: `fleetPeerAllowedV1` admits only loopback or the CIDR of the group. Lines 234-239: `fleetInterfaceAddressV1` picks the one interface inside the group CIDR, else `127.0.0.1`.
9. `B/🌐️remote-cache/🛰️fleet/🪪️identity/🛂️gate/🟦️.ts` line 28: `network` gate by CIDR. Lines 33-34: static bearer tokens are accepted only from loopback when a verifier exists (`local||options.verifier===undefined`).
10. Scheduler: `B/🌐️remote-execution/🛰️sidecar/🗓️scheduler/🟦️.ts` lines 20, 25, 28 use the declared network and `createFleetGateV1({networks:[network]})`.
11. Cache sidecar: `B/🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts` lines 19, 24, 26 use the same network and `policy.fleetPort` (`SEMIO_REMOTE_CACHE_FLEET_PORT` override).
12. The machine's own LaunchDaemon environment, `/Library/LaunchDaemons/tech.semio.fleet.relay.plist`, `…cache.plist` and `…replica.plist` (line 9 in each): `SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.71:6090`, `SEMIO_FLEET_NFS_HOST=192.168.178.71`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`.
13. Published facts: `F/cache-facts.json` has `"endpoints":["http://192.168.178.70:6094"]`, `"execution":{"endpoint":"http://192.168.178.70:8980"}`, `"publisher":{"host":"ueli-mac.fritz.box","port":6094}`. `F/mirror-state.json` has `"base":"http://192.168.178.70:6094"`. A `fritz.box` name is a home-router DNS name.
14. `SW/README.md` lines 407-410: the per-group values are `nfs.gN.semio.internal`, `cache.gN.semio.internal`, `10.0.N.0/24`, set through `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK`, `SEMIO_FLEET_CACHE_ENDPOINT`. These are per-machine variables; there is no cross-network routing.

Consequence: a machine of g2 (`10.0.2.0/24`) cannot reach the g1 NFS host (192.168.178.71), the g1 cache (192.168.178.70:6094) or the g1 scheduler (8980/8981). The peer NFS mounts need a route to the peer NFS server at all.

## 4. Are the build cache, scheduler and graph server reachable for other groups by design?

- **Cache**: by design, no. Each group has its own cache endpoint in its own topology row (`🔣️.json` line 6-10). The fleet facts (`cache-facts.json`, `"group":"g1"`) are published into the g1 store, and the relay copies only origins homed at the peer (`homedOriginsV1`, `🔁️replica/🟦️.ts` lines 2995-2998). Fleet facts are not origins, so a g2 machine would not learn the g1 endpoint. Even if it did, `fleetPeerAllowedV1` and the gate `network` predicate (identity 🟦️.ts 225-231, gate 🟦️.ts 28) restrict to the group CIDR. The callers of the `network` predicate in the signed path were `not found` in this audit (`check()` lines 29-42 do not test it).
- **Scheduler and worker API**: by design, group-local. `networks:[network]` (scheduler 🟦️.ts line 28). Workers join `http://build-host:8981` (`⚙️operations/🔣️catalog.json` line 1779) for their own group.
- **Graph server**: state is local only (`F/graph-server/offered.json`, `pending.json`, `priority.json`). A cross-group graph answer path was `not found`. The graph exchange test `🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🟦️.ts` line 170 expects the port not to be 6090 or 6094 and the hostname to be loopback, which excludes cross-host use.
- Result: a g2 machine gets **no** cache hit or graph answer from g1 today. The only cross-group data path is the NFS origin copy, and that path is empty (section 2).

## 5. The failing replica-module tests in `result-R3.md`

`WORKERS-READY-WITH-LOW-RESOURCES/result-R3.md` names three failing tests (lines 26, 27, 47). It does not name a second replica-module test as failing.

1. **Replica module, named as red**: `SW/🔁️replica/🧪️tests/⏱️turns.ts`, test "a drift repair over a 300k-entry manifest and a 300k-entry store snapshot never holds the event loop for longer than 50 ms in one turn" (line 54). Assertion, line 70: `expect(measured.longest).toBeLessThan(50);`. R3 line 27: "red 499 ms before the yields, 12 ms after". Same file, line 19 (sparse-file test): not marked red in R3.
2. **Attestation module (`🩺️attest`), counted as "(3 tests)" in R3 line 26**: `SW/🩺️attest/🧪️tests/⏱️separation.ts`, line 22 test name "two outliers that differ from a majority and from each other are both named and the pass ends", assertion line 24: `expect(performance.now()-started).toBeLessThan(50);`, line 25: `expect(found.victims.map(entry=>entry[0]).sort()).toEqual(["g1-m04","g1-m10"]);`. R3 says the old function is gone, so the test was not run against it.
3. **`signed-fleet` cluster test**: R3 line 47 quotes "the claim is released", fails with and without the env change, and says it belongs to other work. Location: `not found` in the replica, attest, resilience or topology directories.

## Limits

- Relay log lines have no timestamps. Per-peer exchange times are `not found`.
- The `ENOSPC` errors (2026-11-14 or 15, exact time `not found`) predate this audit window; disk now has 107 GiB free.
- No g2 or g3 machine was contacted; findings for other groups come only from the topology file and the empty peer folders.
