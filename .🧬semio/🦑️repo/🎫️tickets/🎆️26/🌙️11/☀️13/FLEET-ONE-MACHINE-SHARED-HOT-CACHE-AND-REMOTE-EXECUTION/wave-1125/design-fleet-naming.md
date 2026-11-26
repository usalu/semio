# 🧭 Fleet Naming And Discovery Design (Wave 1125, 22:10 CET)

User requirements (binding): every machine may have several addresses (LAN ports, Wi-Fi); no IPs or computer names are hardcoded; the fleet keeps working after moving to another private network; names are semantic; everything scales beyond the current 14 machines; no surprises on worker screens.

## Incident 22:00
Mac workers showed the macOS dialog "local hostname 'Uelis-Mac-mini.local' is already in use … changed to 'Uelis-Mac-mini-2.local'".
- Cause: g1-m08, g1-m10, g1-m13, g1-m14 share the OS LocalHostName `Uelis-Mac-mini` (cloned machines). F47 landed (21:54) a DNS-SD service advertisement (`dns-sd -R <origin> _semio-fleet._tcp …`) in the remote-execution agent. A DNS-SD service's SRV target is the OS host name, so every worker began actively announcing that duplicated name and macOS resolved the conflict with a dialog and a rename.
- Second defect: F47's sidecar tests started real agents on the coordinator that advertised `g1-m09` on the LAN (a wrong identity on the real network).
- No code calls `scutil`, `networksetup`, `dns-sd -P` or registers host names.

## Principles
1. Identity = origin id (`g1-server`, `g1-m01` …), allocated once at provisioning, unique by construction, independent of OS names, IPs, interfaces, subnets.
2. OS host names are never read as identity, never written by the fleet, never put in topology, config or service definitions (law test).
3. Address facts are observations: each origin publishes all its current non-loopback addresses (`addresses.json`, freshness stamp) into the shared store; readers resolve origin → addresses (same subnet first, then last-good, then the rest) with per-attempt failover.
4. Roles (store, cache, scheduler, relay) are assigned to origin ids in the topology; clients ask for a role, never a host.
5. Tests never touch the real network's discovery (mDNS register/browse) — advertisers and lookups are ports, stubbed in tests; a law test refuses spawning `dns-sd`/`avahi-*` from test files.

## Bootstrap after a network move
The shared store is reached over the network, so a machine that lost every known address needs one discovery path that depends on neither IPs nor host names:
- DNS-SD service browse for `_semio-fleet._tcp` with TXT `origin=<id>` and `roles=…`; the SRV target is whatever unique name the OS currently has (macOS renames duplicates itself), used only transiently to obtain addresses.
- Advertising is gated: a machine advertises only when its OS host name is not in conflict (no conflict rename observed and the name is unique among the published origins' names). Otherwise it stays silent and relies on the store-published addresses; status shows a bilingual warning naming the duplicate.
- Order: last-good → published addresses → DNS-SD browse (bootstrap only).

## Duplicate OS host names (USER, one-time per machine)
Duplicate LocalHostNames are a machine misconfiguration that macOS itself reports. The fleet must not rename machines silently (system setting, needs admin). Recommended one-time step per affected Mac, chosen semantic and unique by construction from the origin id:
`sudo scutil --set LocalHostName semio-<origin>` (e.g. `semio-g1-m08`). The fleet works without it (store-published addresses); DNS-SD bootstrap needs unique names.

## Immediate actions (22:10)
1. Disable the agent's live `dns-sd -R` start until the conflict gate above is implemented (F47, anchored edit).
2. Stub all discovery in tests; law test against real `dns-sd`/`avahi` spawns in tests.
3. Implement the conflict gate + TXT `origin`/`roles`; roles by origin id in topology; NFS store remount via resolved store-role address.
