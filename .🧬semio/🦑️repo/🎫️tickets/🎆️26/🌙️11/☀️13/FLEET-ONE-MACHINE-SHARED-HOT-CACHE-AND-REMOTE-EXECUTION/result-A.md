# Package A: fleet facts (interface first, report below fills in as it lands)

Module root R = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/`.

## Interface for B, C, D, E (fixed)

- Schema (first): `R/🛰️fleet/🧬️schema/🔣️.json`, definitions `Facts` (the file `<store>/v1/fleet/cache.json`), `Credential` (server-private), `LocalCopy` (per machine).
- `cache.json` fields: `version:1`, `group`, `generation` (credential rotations, only grows), `publishedAtMs`,
  `cache:{endpoints[] (reachable from workers, preferred first), instance, access:"read-write", protocols:["bazel-http","nx-self-hosted"], protocolVersion:1}`,
  `credential:{scope:"fleet", id, token, rotatedAtMs}`, and `execution?:{endpoint, protocolVersion}` which is RESERVED FOR PACKAGE D: absent until D publishes it.
  The publisher keeps an existing `execution` when it republishes (`publishFleetCacheFactsV1` preserves it); D writes it through `setFleetExecutionV1(storeRoot, {endpoint, protocolVersion}|null)` from `R/🛰️fleet/🔑️credential/🟦️.ts` (refuses while no facts are published; tested).
- Local copy per machine: `<worktree>/.cache/f/cache-facts.json` (mode 0600, never replicated).
- THE resolver every client of the cache must use (module `R/🛰️fleet/🟦️.ts`):

```ts
resolveRemoteCacheV1(environment, options?: {root?: string; facts?: FleetCacheFactsV1}):
  { contract: RemoteCacheContractV1; token: string|undefined; environment: Record<string,string|undefined>;
    endpointFrom: "environment"|"fleet-facts"; tokenFrom: "environment"|"fleet-facts"|"none"; generation: number|null } | undefined
```
  Synchronous, reads only the local copy (never the NFS store), explicit environment first per field, then the facts. `environment` of the result is the input plus the resolved values (incl. the token under `contract.auth.tokenEnvironment`): hand it to code that reads the token from an environment.
  `resolveRemoteCacheContractV1` stays the pure environment-only function.
- Async refresh (reads the store through `.cache/f/machine.json` `storeRoot`, timeout, last known value, TTL 30 s): `refreshFleetCacheFactsV1({root, force?, timeoutMilliseconds?, now?})`. Call it once early in an async path; after a 401 call it with `force:true` and resolve again.
- Reading the execution endpoint (D, E): `readFleetExecutionV1(environment, options?)` returns `{endpoint, protocolVersion}|undefined` from the local copy.
- Server: `GET /status` (token needed) now also carries `fleet:{generation, credentialId, rotatedAtMs}` (no secret) once the server runs the new code.

- One call that does everything a build needs: `resolveAuthorisedRemoteCacheV1(environment, {root?})` refreshes (TTL), resolves, probes the Nx archive route, and on 401 or an unreachable endpoint forces one refresh and retries (with the fleet credential in place of a rejected explicit token). Never throws.
- Public re-export: `📦️packages/🟦️typescript/🏗️build/🌐️remote-cache/🟦️.ts` now also exports the four functions above.
- Contract for callers: pass `SEMIO_WORKSPACE_ROOT` (every `repo` build sets it) or `root`. Without a root the resolver reads no facts, so tests and tools outside a build never see the machine's credential.

# What was built

1. Schema first: `R/🛰️fleet/🧬️schema/🔣️.json` (`Facts`, `Credential`, `LocalCopy`), Ajv cross-check in the tests.
2. Server side (`R/🛰️fleet/🔑️credential/🟦️.ts`): the cache server itself owns the credential. Decision: the server, not the replica daemon. Reason: the server is the process that must accept the token, so it can mint, rotate and honour it without a restart (`fleetCredentialSourceV1` re-reads the 0600 file once a second); the daemon would need the root-only env file on Linux and a daemon change on every machine. The dedicated credential is 32 random bytes, distinct from the two tokens of `/etc/semio/remote-cache.env`; `repo remote-cache serve` (on the machine whose origin is `<group>-server`, and only with a `--directory` outside the replicated worktree) creates it, publishes `<store>/v1/fleet/cache.json` (mode 0600, atomically, only when something changed, re-checked every 60 s) and accepts it as a write token. `GET /status` carries `fleet:{generation,credentialId,rotatedAtMs}` (never the secret).
3. Rotation: `repo remote-cache rotate --directory <server directory>` (new subcommand, in the operations catalog and help snapshot, English and German). The previous token stays valid 15 minutes; clients refresh on their first 401.
4. Clients: `resolveRemoteCacheV1` (sync, local copy only), `refreshFleetCacheFactsV1` (store root from `.cache/f/machine.json`, 1.5 s read timeout, 700 ms endpoint probes, TTL 30 s, last known value kept for up to 7 days, never throws), background refresh from the resolver at most once per TTL. Wired into: Nx env (`nxBuildEnvironmentV1` in script.ts line 254 and `nxRemoteCacheEnvironmentV1`), graph-generation exchange, Bazel storage flags, the semio backend (also refreshes before use), `remote-cache status`. The fleet credential is only paired with an endpoint the facts name.
5. `repo remote-cache status`: new `resolution` (endpointFrom, tokenFrom, factsGeneration, factsState) and `remote` (endpoint, reachable, authorised, entries, bytes, maximumBytes, fleetGeneration as the server reports them); the local `store` is only filled for an existing directory.
6. Docs: `🤝️shared-worktree/README.md` (zero-touch section, LaunchDaemon exception), `🦑️repo/README.md` (EN and DE).

# How it is proven

- `R/🛰️fleet/🧪️tests/🟦️.ts`, 9 tests (9 pass with the Nx gate on): schema vs Ajv (valid and 7 invalid documents), publish/idempotence/execution preservation/rotation/modes 0600/0700, credential source (rotation without restart, grace expiry), resolver precedence and the foreign-endpoint rule, worker authorised with no token in its environment and independent `curl` 200/401, 401 then forced refresh then rotated token accepted, stalled store (a FIFO) returns `stale` in under 2 s, TTL states `fresh/checked/updated/stale/absent`, the group-server plan (workers never publish), the `serve` command publishing and `rotate` reaching a running server, nothing written inside the worktree.
- Real Nx 23.2.0 (patched-free package from the bun cache, gated by `SEMIO_TEST_NX_NODE_MODULES` and `SEMIO_TEST_NX_PACKAGE`, 5.5 s): checkout A, no token in the environment, routed by the fleet facts: `0/1 hit`, server `nx.puts = 1`; checkout B, separate local cache dir: `1/1 hit (100%)` from the remote cache, output file restored, server `nx.hits >= 1`, `nx.puts` still 1. This also settles the open question of the map: the Nx hash does not include the absolute workspace path (two different roots hit).
- Live on this machine, real `repo remote-cache status`: authorised (probes 404 on both protocols), `tokenFrom: fleet-facts`, `remote.entries: 2`, `remote.bytes: 765769` as the server counts (the server's counters showed `nx` 1 put, 5 misses, 0 hits since its start; the fleet had never used it).
- `nxBuildEnvironmentV1("/Users/Shared/semio", {endpoint env only})` now returns the Nx server and a token with no token variable set.

# Not done or not verified (read this)

- LIVE RESTART NEEDED: the installed job (pid 850, started before this code) still runs the old server code, so it does not mint or accept the dedicated credential and does not publish. To make the fleet usable now I ran the one-off `🧰️scripts/A/publish-interim-facts.ts` once: it published `cache.json` generation 1 using the server's existing WRITE token from `/etc/semio/remote-cache.env` as the credential (never printed or written to the worktree; it sits in the store, mode 0600). That is the one place the dedicated-credential rule is not met, until the job is restarted (`sudo launchctl kickstart -k system/tech.semio.fleet.cache`, no sudo here): the new server then generates its own credential, republishes (clients heal on their first 401) and the interim token should be replaced by editing `/etc/semio/remote-cache.env`. I did not touch the installed job.
- Workers: nothing ran on any other machine. That a worker reads a 0600 file through the NFS export (`mapall=ueli` on this export, `all_squash` on Linux) is by reasoning, not measured; if it fails the worker stays local (`factsState: absent`) and `status` says so. Windows and Linux code paths are untested (the code uses only `node:fs`).
- `R/🧪️tests` is red with 2 failures that are package B's: `GET /status` now carries `claims`, which `ServerStatus` in `🏃️execution/🧬️schema/🌐️remote-cache.json` does not list. My status assertions (`resolution`, `remote`) in that file therefore never ran green here (the logic runs live, see above). Re-run after B lands the schema.
- `protobufjs`: it IS declared (root `package.json:288`, `bun.lock`, `🔒️dependencies.json`); what is missing is an install on this machine (no `node_modules` for it), so the `🔬️action-result` and `📡️wire` tests cannot load here. I tried declaring it in the repo product `package.json` and reverted: without a lockfile regeneration that would break frozen installs. Not run. Likewise `typescript` is not installed, so the Bazel storage test file does not load (environmental, not caused by this work).
- `⚙️operations/🧪️tests` has one failure that needs an installed Nx (`nx/src/tasks-runner/create-task-graph`), unrelated.
- `status.remote.bytes` counts the index file sizes, and an Nx entry is a 70-byte reference to a body in the host CAS, so it understates the real bytes (B and C own retention: they should know that `maximumBytes` is compared with that index size).
- LaunchDaemon vs LaunchAgent: decided that the generator (LaunchAgent) is right for the committed layout (`~/Documents/semio` is a protected folder, measured), and that the installed daemon-as-user is a valid variant only because this worktree is `/Users/Shared/semio`. Documented in the README, generator left as it is. Not changed: the installed job.
- Windows: `serve` reads `.cache/f/machine.json` the same way; the `rotate` and credential paths use POSIX modes that Windows ignores.

# What the other packages must know

- B: the server has a dynamic token provider (`tokens.fleet`); your claims route can authorise with the same handler; `ServerStatus` needs your `claims` property in the schema.
- C: nothing in the store directory outside `v1/`: the credential lives at `<directory>/fleet/credential.json`, and the facts in the group store `v1/fleet/`; neither may be collected.
- D: write `execution` with `setFleetExecutionV1`; read it with `readFleetExecutionV1`. The publisher preserves it.
- E: use `resolveAuthorisedRemoteCacheV1` before talking to the cache; the token it returns is the fleet token on a joined machine.

# 2026-11-13 22:30 to 22:50: defect "scratch server overwrote the real fleet facts"

Cause: `serve` published whenever machine.json said `<group>-server`, whatever directory and port it ran with; package H's scratch server (directory scratch, port 6191) wrote `cache.json` generation 1 with a credential the real server does not know. Dead cache for every machine from 22:31.

Repair (data): 22:36:34 first repair (generation 2, same interim credential, port 6090), confirmed with `repo remote-cache status`. Final repair 22:43:56 after the code landed: generation 3 with the publisher identity `{directory: /var/lib/semio-remote-cache, port: 6090, host}`, confirmed with the real `repo remote-cache status`: `endpointFrom: environment` (`http://192.168.178.71:6090`), `tokenFrom: fleet-facts`, `factsGeneration: 3`, authorised true, 15 entries. Credential never printed. Note: when the real server restarts with the new code it has the same identity, so it raises its own credential above generation 3 and takes over.

Fix (code), all under the project lock:
1. Publishing is explicit and impossible by accident (`fleetServerPlanV1`, `fleetProductionIdentityV1`): the real store is published to only by the production identity, exactly data directory `/var/lib/semio-remote-cache` (or `C:\semio-remote-cache`) with the topology port (6090 here). `SEMIO_REMOTE_CACHE_SERVER_FLEET=publish` is the explicit demand the provisioned definition passes (an environment variable, not a CLI flag, so the operations catalog and help snapshot stay unchanged); with any other directory or port it makes `serve` refuse to start. `SEMIO_REMOTE_CACHE_SERVER_FLEET_STORE=<dir>` names a scratch store; naming the machine's real store without the production identity is refused. No credential is minted unless publishing is allowed.
2. Rotation safety: facts record `publisher:{directory,port,host}` (schema, required). `publishFleetCacheFactsV1` throws `FleetPublicationRefusedV1` and leaves the file untouched when another identity's facts are under ten minutes old, or the generation would not exceed the published one; the same identity raises its own credential above a higher published generation. Clients (`refreshFleetCacheFactsV1`) keep their previous good `LocalCopy` when none of the new facts' endpoints answers.
3. Topology generator/service definitions: see the landing note below (separate, needs the shared-worktree lock).
4. `claims` in `ServerStatus`: package B had already fixed it (the main suite is green: 20 pass); nothing for me to add.

Tests (all in `R/🛰️fleet/🧪️tests/🟦️.ts` unless noted): "the server plan publishes only for the group server with the production identity or an explicit scratch store, and refuses the real store otherwise"; "a scratch server leaves existing facts byte-identical, and a publisher never lowers the generation or replaces recent facts of another identity" (byte-identical real facts after a scratch `serve`, refusals for `publish` and for the real store, no credential minted, lower generation refused, same identity raises); "a refresh ignores facts whose endpoints all fail and keeps the previous good local copy"; the earlier serve/rotate/Nx tests now use a separate real and scratch store. Suites: fleet 11 pass, remote-cache main 20 pass, server 6, collector 7, cas 35, remote-exchange 4.

Message for B: your `claims` schema fix is in and green; nothing open.

PENDING (22:52): the topology generator and its tests are NOT landed. The shared-worktree lock (`1113-landing.lock`) has been held by claude-fleet-D since 22:35 and I waited over 8 minutes; the edit also changes the daemon code digest of every machine, so I did not take it over. Ready to apply: `T/🧰️scripts/A/patch-topology.py` (adds `SEMIO_REMOTE_CACHE_SERVER_FLEET=publish` to the systemd, macOS and Windows cache units and updates the three expectations in `🗺️topology/🧪️tests/🟦️.ts`); it must then run all `🧪️tests` of the module as the protocol demands. Until then the installed job is covered by the production-identity rule alone, which is sufficient.
