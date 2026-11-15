# R12: One Fleet Across Networks (prepared, NOT landed)

Status: prepared. Nothing was written to the live modules, no lock taken, no service touched. Payload and landing script: `T/🧰️scripts/R12/` (`T` = FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION), `land.py --dry-run` against the live tree is clean (17 `copy`, 2 `merge` over drifted lines of `gate/🟦️.ts` and `policy.json`, 6 `new`).

## 1. Design

One fleet = several groups (g1..gN of the topology). Each group has one server (cache server, scheduler, relay) and members. Principles:

- Authorisation is the signed fleet identity (existing `Semio-Fleet` Ed25519 proof, role keys, store registry), never an address. The CIDR `server.network` stays what it always was (choose the interface to bind); it becomes an access condition only when the group row says `access.enforceNetwork: true`.
- A server trusts the origins registered in the stores of the fleet: its own store, plus the stores of the peer groups it mounts read-only (`peers`), or the relay copy of their origins in its own store. The registration must name a group that the topology lists; `links.<peer>.serverKey` pins the peer server's origin key instead of first sight.
- Roles across groups: an origin of another group holds `cache-read` and `submit` only (gate option `home`/`foreignRoles`). It cannot write, claim, or work for our scheduler. Uploads stay local.
- Endpoints and ports are facts: topology row (`ports`, `server.advertise`, `links`) and attested facts (`<store>/v1/fleet/cache.json` of the peer, published by its server). No literal in code.
- Transport is an interface `FleetPeerTransportV1` with two implementations, both carrying only content-addressed reads (`/ac/<sha>`, `/cas/<sha>`, `/v1/cache/<key>`; schema `🧭️peers/🧬️schema`):
  - `direct`: authenticated HTTP GET to a routable endpoint (facts of the peer plus listed `endpoints`), signed with the group server's `cache-read` key, one hop header so two servers never ask each other in a circle.
  - `relayed`: store and forward through the stores the servers mount of each other: request file in the asker's store (`v1/fleet/exchange/<peer>/requests`), answer file + body in the answerer's store, read through the read-only mount; nothing is ever written into a foreign store.
- Local-first: the read-through is bounded (4 s, parallel over all peers, first hit wins, others cancelled), a peer failing 3 times in a row is skipped 30 s, an all-miss path is not asked again for 3 s; any failure is a plain local miss.

How a machine of group B gets, from group A:
- (a) worktree changes: unchanged path (relay copies homed origins and journal from the mounted peer store); the mount (or any share that is mounted at the `peers` folder) is the transport. No new code; a direct replica transport is future work.
- (b) build cache hits: miss in B's server -> `GET` at A's server -> verified (digest) -> stored in B -> served; afterwards local. `cas` and `nx` are kept; `ac` is served without being kept (wanted/served entries are mutable); its blobs are fetched and kept by the same rule on demand.
- (c) graph answers and generations: they are cache entries (`graphEntryKeyV1` ac + cas), so (b) delivers them. Wanted requests are `ac` entries too: server of A reads a requester of B through its own cache. The graph sidecar adds a peer-group machine to the requesters only when no other live attested machine of its group works in its platform class (`forwardableRequestersV1`), so the same request is not computed twice; the answer (`served`) is published in A's cache and read by B through B's server.
- (d) remote execution from A: A's scheduler now accepts `submit` signatures of B's origins (any address, never `worker`). The client side that picks a peer scheduler from the peer's attested `execution.endpoint` for the same platform class is NOT built (see 4).

## 2. What changed (files in `T/🧰️scripts/R12/R12-bundle.json`)

Modified (19): `🌐️remote-cache/` `🛰️fleet/🪪️identity/🟦️.ts` (registry: `groups`, `peerStores`, `pinned`, `groupOf`; verdict carries `group`), `…/🛂️gate/🟦️.ts` (`home`, `foreignRoles`, `networks` optional), `🛰️fleet/📣️publication/🟦️.ts` (`fleetGroupViewV1`: ports, advertise, links, peer store paths, enforceNetwork), `🖥️server/🟦️.ts` (read-through, `/status.peers`, networks optional), `⌨️command/🟦️.ts`; `🌐️remote-execution/` `🛰️sidecar/{🗄️cache,🗓️scheduler,🕸️graph,👷️agent,🧰️shared}/🟦️.ts`, `🚢️status/🟦️.ts`, `🛰️sidecar/🧪️tests/🟦️.ts`; `🔣️policy.json` and `🏃️execution/🧬️schema/🌐️remote-cache.json` (`fleetPort` removed); `🤝️shared-worktree/🗺️topology/` `🔣️.json` (ports + access per group), `🧬️schema/🔣️.json`, `🟦️.ts` (`peerStorePathV1`, link validation), `🧪️tests/🟦️.ts`, `🧪️tests/🐍️.py`.
New (6): `🌐️remote-cache/🧭️peers/` `🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/{🟦️.ts,🐍️oracle/🐍️.py,🧫️fixtures/🔣️.json,🧫️fixtures/🌍️.json}`.

Literals: no `192.168.178` and none of 6094/8980/8981 remain in code. The cache port (6094) is `ports.cache` of the group row; scheduler and worker API ports are `ports.scheduler`/`ports.worker` (env `SEMIO_REMOTE_EXECUTION_PORTS` still wins; the standalone `repo remote-execution serve` keeps its documented 8980/8981 defaults in `policy.json`, and the catalog texts keep naming them as defaults). `192.168.178.*` in tests are sample CIDR data. The three LaunchDaemon plists on the owner's machine (`/Library/LaunchDaemons/tech.semio.fleet.{relay,cache,replica}.plist`) are not in the repo: they carry operator-typed values (`SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK`, `SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.71:6090`) copied by the provisioning from the operator's environment; the generated units themselves take port and network from the topology row.

## 3. Proof (overlay copy of the closure of the touched members, BUN_RUNTIME_TRANSPILER_CACHE_PATH=0)

New `🧭️peers/🧪️tests` (12 tests, 3.7 s), among them:
- blob uploaded in A is a hit for B exactly once: A's `cas.hits` is 1 after B's first read and still 1 after the second; B stores it; a HEAD stays local; the former CIDR gate refuses `10.0.2.7`, the gate without the policy accepts it.
- no signature, bearer token, or origin of a fleet no topology names: 401 on A and B; a B machine reads A (200/404), writes A (403), claims at A (403); nothing stored.
- link down (A stopped): B keeps writing and reading, 4 misses + 1 hit in under 6 s, `/status.peers.errors >= 1`, `skipped >= 1` after three failures.
- wanted: B publishes a request, A's `readWantedV1` sees it through A's server, A publishes `served`, B reads it; a newer request is seen at once.
- relayed route: silent peer = 404 after the bound; with the answerer running the same bytes arrive and are kept; no HTTP link between the groups.
- scheduler gate: submit of a peer origin from `10.0.2.77` accepted (`group g2`), its `worker` 403, a stranger 401.
- registry: peer origin admitted only for a named group whose registration names that group; topology pin: match ok, forged pin = conflict.
- forwardable requesters (class rule), schema vs Ajv on every vector, independent Python oracle (`ipaddress`, `hashlib`, `re`) agrees on the same vectors, CIDR cases, SHA-256 bodies; no-literal scan of the fleet sources.
Existing suites in the overlay, all green: identity 9, fleet 11 (+1 skip), server 6, access 6, remote-cache 20 (+2 skip), sidecar 4 (the new one starts the cache role from a topology row only: port, advertise host, peer reader 404, peer writer 403), sidecar graph 8, wanted 6, topology 70 (+1 new: ports/links/peer paths, Ajv + Python oracle). Parse proof of the 25 changed files (bun transpile, JSON, `ast`): 0 failures. Landing rehearsal in a scratch root: `--land` then `--rollback` then `--dry-run` clean.

## 4. NOT verified

- Nothing ran over two physical networks, a real NFS peer mount, a firewall or NAT: only loopback and directories.
- Direct transport is plain HTTP: answers are not authenticated or encrypted on a routable link (blobs are digest-checked; `ac` results are not). TLS for the sidecar is not wired (the server supports it). Use `relayed` over a trusted mount or a VPN until then.
- Peer member origins are pinned on first sight; only `links.<peer>.serverKey` pins the peer server.
- No client sends work to a peer scheduler; no cross-group worker; `ac` heads are not kept locally (a head lookup asks the peer each time; misses are remembered 3 s).
- A forwarded wanted request can still be served twice if a machine of the class attests in its own group after A took it.
- `tsc` type check was not run (none available); only the parse proof. Not run: the real `repo` build command, the full shared-worktree test suite, `🧪️tests/🌐️fleet`, `signed-fleet`, `fleet-status`, `remote-execution ⌨️command` (the overlay lacks files they spawn; they are in `land.py`'s list and run there).

## 5. Landing: restarts and what the owner does

`land.py --land --authorized` (or `N/LAND-OK-R12`): takes `landing.lock` and the shared-worktree `1113-landing.lock`, replaces all files, parse proof, runs the listed tests and every test file of the shared-worktree module, writes `landings.md` and `1113-landings.md`, keeps originals for `--rollback`.
Restarts caused: `build-cache` and `build-scheduler` on the group server (cache restart empties nothing durable; claims come from the journal); `build-agent` and the graph server on every machine (code-closure watch, exit 75); the topology member is part of the replica code, so the code digest moves and each replica daemon follows the 1113 rollout (server first, then members; expect agreement waits). Run the real `repo` build command afterwards.
Owner steps for a second network:
1. Mount each peer store read-only on each group server at its `peers` path (`/mnt/semio-peers/gN`, macOS `/Volumes/semio-peers/gN`) over any route (NFS/SMB/VPN); this is also what the relay and the registry read.
2. Replace the placeholder hosts/CIDRs of the topology rows with the real ones (or the `SEMIO_FLEET_*` overrides), drop the 192.168.178 values and `SEMIO_REMOTE_CACHE_ENDPOINT` from the three plists, re-run the provisioning.
3. For `direct`: make `ports.cache` of each group server routable from the other network, list its host in `server.advertise`, set `links.<peer>: {"transport":"direct"}` (optionally `endpoints`, `serverKey` = `origin` of `<peer store>/v1/origins/<peer>-server/fleet-auth.json`). For no route: `{"transport":"relayed"}`.
4. Check `GET /status` of a group server: `peers.hits/errors/skipped`.
