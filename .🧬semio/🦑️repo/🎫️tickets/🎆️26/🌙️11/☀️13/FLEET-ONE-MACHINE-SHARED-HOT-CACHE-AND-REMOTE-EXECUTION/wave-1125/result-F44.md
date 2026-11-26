# Result F44: no hardcoded IPs, addresses follow the machine (21:50)

## 1. Audit (every hit, tracked files + live machine state; tickets and tests excluded)
Committed config/code (before this lane):
- `🤝️shared-worktree/🗺️topology/🔣️.json` lines 6-10: `server.network` `10.0.1.0/24` .. `10.0.5.0/24` for g1..g5 (CIDR placeholders). Removed. g1 `nfs.host` was `nfs.g1.semio.internal` and `cache.endpoint` `cache.g1.semio.internal` (names that resolve nowhere); now `ueli-mac.local` (+ `nfs.candidates ["ueli-mac"]`). g2-g5 keep placeholder names (groups do not exist).
- `🗺️topology/🟦️.ts`: `server.network` used in 6 renderings (Linux exports, macOS exports, Windows firewall x2, `SEMIO_FLEET_NETWORK` in 3 cache service definitions, facts `SERVER_NETWORK`). Now derived from the interfaces at render time (`serverNetworksV1`), `LocalSubnet` on Windows, variable only when an operator pins `network`.
- `remote-execution/🛰️sidecar/🗄️cache` and `🗓️scheduler`: bound ONE address inside `SEMIO_FLEET_NETWORK` and invented `<first three octets>.0/24` as fallback; `cache` published private IPv4 only (regex of RFC 1918 ranges). Fixed (below).
- `remote-cache/🛰️fleet/📣️publication` `fleetCacheEndpointsV1`: RFC 1918 regex, IPv4 only. Fixed.
- `🌎️hub/README.md` lines 20, 90, 99, 102: example `OS_HUB_BIND=10.0.0.4`, and `192.168.178.70:7661` for the running hub. Documentation of another product, NOT changed (outside lane; hub binds are operator input).
- All other tracked hits are document numbers (`10.08.18.7-25.06`, `mit-bestand`) and are not addresses.
Persisted on machines (not in git):
- `.cache/f/machine.json`: no address (origin, group, store root, key). Fine.
- `S/fleet/cache.json` (published by the cache sidecar): `cache.endpoints ["http://192.168.178.70:6094"]`, `execution.endpoint http://192.168.178.70:8980`, `publisher.host ueli-mac.fritz.box`. Written from the live interface; republished on change (every 15 s now), with the `.local` name added.
- `S/origins/*/ready.json` `host` = `os.hostname()` (e.g. `ueli-workstation-mac-mini-m4-16gb-256gb-02-lan.fritz.box`): a display label, never connected to. Left.
- This Mac (g1 server): `~/Library/LaunchAgents/tech.semio.fleet.{replica,relay,compact}.plist` carry `SEMIO_FLEET_NFS_HOST=192.168.178.71` (the Wi-Fi address, not even the Ethernet one), `SEMIO_FLEET_NETWORK=192.168.178.0/24`; `/etc/zshenv`, `/etc/profile` the same; `/etc/exports` has `-network 192.168.178.0 -mask 255.255.255.0`. Written by the old provisioning from the operator's variables. Replaced by re-provisioning (USER).
- Workers: the same variables in their LaunchDaemon/profile, `/etc/fstab` (Linux) or LaunchDaemon mount plist (macOS) with the IP as NFS source, Windows drive `S:` mapped from `\\<ip>\...`; unknown per worker (not readable from here).
- `W/prompt-g1-m09.md`: used `192.168.178.70` in showmount, mount and the three variables. Rewritten to the `.local` name with no variables.
- SERVE-G1-CACHE-ON-BOTH-INTERFACES: confirmed the cache bound "IPv4 addresses present at startup" and needed a restart for a new interface. Superseded: the cache now listens on the wildcard.
- Policy/relay/remote-execution endpoint code: no other literal (the `127.0.0.1` loopback in the signing proxy, single-flight and tests is intentional and allowed by the law test).

## 2. Design (schema first)
- `🗺️topology/🧭️addresses/🧬️schema/🔣️.json`: address set `{version, origin, hostname, mdns, addresses[{address,family,interface,prefix,scope}], observedAtMs, ttlMs, generation}` at `<store>/v1/origins/<origin>/addresses.json`. Loopback and link-local never published; scope private / unique-local / global; TTL 5 min, refreshed at half; generation rises only when content changes.
- Publisher: the build agent (runs on every machine, also the server) watches `os.networkInterfaces()` every 15 s and republishes (atomic temp+rename, en/de log line). Live proof: a scratch agent wrote `generation 1` with `mdns ueli-mac.local` and the real Ethernet address.
- Resolver (`resolveFleetEndpointV1`): published set -> same subnet first, then private, unique-local, global, IPv4 before IPv6 -> last good (`.cache/f/last-good.json`) first -> `.local` name after the addresses (before them when the set is stale) -> sequential tries with per-attempt time limit; returns `undefined` so callers fall back to local work.
- Servers: cache binds loopback + wildcard (`::` dual-stack, else `0.0.0.0`) when no declared network matches (or none is declared), so new interfaces need no restart; with an explicit `SEMIO_FLEET_NETWORK` that matches an interface the old pinned behaviour stays (tests, operator override). Scheduler binds the facts' address or the best interface address and restarts itself when that address disappears.
- Bootstrap: topology `nfs.host` = server `.local` name, `nfs.candidates` further names. NFS store mount is by name (fstab/LaunchDaemon plist/drive), and re-provisioning replaces an older fstab source of the same export and unsets stale launchctl variables; on a mount that still uses the old source it prints the exact remount commands (sh and ps1, en/de for ps1).
- Network for exports/firewall: derived from the current interfaces when a plan is rendered; on Windows `LocalSubnet`.

## 3. Tests (all `repo test-file`)
- `🧭️addresses/🧪️tests/🟦️.ts` 10/0: schema (Ajv + closed-shape guard), classification vs `node:net` and a Python `ipaddress` oracle, atomic publication/generation/refresh, watcher with interface change (de), ordering, failover (refused, hanging, working IPv6; last good first next time), subnet change (old subnet -> new subnet; stale set -> `.local`), texts en/de equal variables, LAW: no IPv4 literal in topology, policy, `🌐️remote-cache`, `🌐️remote-execution`, shared-worktree (tests and fixtures exempt; 0.0.0.0, 127.0.0.1 and netmasks allowed), LAW: committed topology has no network/machines/IP.
- topology 74/0 (updated expectations), remote-cache fleet 14/0 (new IPv6 + name case), fleet-endpoint 2/0, endpoint 2/0.
- `🛰️sidecar/🧪️tests/🟦️.ts`: 4 pass, 3 fail. Cache-role tests: the sidecar stops at `The partition products is not attached` (checked by running it by hand in a scratch worktree with the test environment: it gets past all address code and fails there), scheduler/agent test: "the agent must join and report"; no A/B possible under load 70-110 without altering the live tree. NOT verified that these 3 pass on a machine with the partition attached.

## 4. NOT verified
- Real network change (second subnet), mDNS from Linux/Windows workers, the `.ps1` edit (not run), the fstab edit (awk only reasoned), the Windows `LocalSubnet` rule on a server.
- IPv6 listener on a host where `::` cannot be bound falls back to `0.0.0.0` (tested only on macOS).
- The resolver is used by tests and ready for callers; the existing clients keep their own probe-and-failover on the facts endpoint list (`select` in `remote-cache/🛰️fleet`), which now receives all addresses plus the `.local` name. Switching them to `resolveFleetEndpointV1` (last-good file) is a follow-up.

## USER (cannot be done zero-touch; run on each machine)
1. Server (this Mac), once, to drop the old IP variables and re-export by interface (needs sudo and replaces the running services):
   `cd ~/Documents/semio && sudo bash 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🐚️.sh g1 g1-server --user ueli --server` (no `SEMIO_FLEET_*` variables set). Afterwards `launchctl getenv SEMIO_FLEET_NFS_HOST` must print nothing and `/etc/exports` must list one `-network ... -mask ...` per active interface.
2. macOS workers (existing mount from an IP): stop the replica service, then `sudo umount /Volumes/semio-store && sudo mount_nfs -o <options printed by the script> ueli-mac.local:/Users/Shared/semio-store /Volumes/semio-store`; the script prints the exact line. Re-run the provisioning script WITHOUT `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK`.
3. Linux workers: install multicast DNS name resolution once (`sudo apt install avahi-daemon libnss-mdns`, or the distro equivalent) so `ueli-mac.local` resolves; re-run the script (it replaces the old fstab line of the same export, then `sudo umount /mnt/semio-store && sudo mount /mnt/semio-store`).
4. Windows workers: `.local` resolves natively on Windows 10/11; remap the drive: `mount.exe -u S:` then `mount.exe -o <options printed> \\ueli-mac.local\Users\Shared\semio-store S:` (the .ps1 prints the line) and re-run the script without the three variables. If the router blocks multicast, use `nfs.candidates` (bare name `ueli-mac` through the router DNS).
5. Router/AP: allow multicast (mDNS, UDP 5353) between the ports and Wi-Fi of the server; otherwise workers fall back to the published address set only.
6. g1-m09 / g1-m15 join prompt: `W/prompt-g1-m09.md` now uses the name; if `showmount -e ueli-mac.local` fails on that Mac, report instead of typing an address.

## Addendum: further runs
- `🥾️provision/🧪️tests/🟦️.ts` 22/0, dangling imports exit 0. `🧪️tests/🛠️service/🟦️.ts` 13 pass / 2 fail: the two "real launchd" tests (swap of a running LaunchAgent plist, 90 s timeouts under load 70-110) touch the service helper, not addresses; not A/B'd.
