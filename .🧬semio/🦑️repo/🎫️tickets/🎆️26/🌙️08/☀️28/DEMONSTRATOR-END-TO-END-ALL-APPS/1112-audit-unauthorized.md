# 🔐️ Audit: Nx `Unauthorized: Unauthorized` after a 27-minute task

Read-only investigation. One write was made by mistake, see "Side effect" at the end.

Repo root: `/Users/ueli/Documents/semio`. Repo tool library: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` (abbreviated `LIB`).

## Short answers

1. **Which server.** With `SEMIO_REMOTE_CACHE_ENDPOINT` unset, Nx gets a local fallback. `LIB/⚡️caching/🚀️bootstrap/📜️script.ts:454` calls `nxRemoteCacheEnvironmentV1(env)`. That function (`LIB/🏗️build/🌐️remote-cache/🧶️nx/🟦️.ts:6-14`) probes `http://127.0.0.1:6090/healthz` with a 300 ms timeout. If it answers, it sets `NX_SELF_HOSTED_REMOTE_CACHE_SERVER=http://127.0.0.1:6090/main` and `NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN=<SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN ?? SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN ?? "local">`. The port comes from `LIB/🏗️build/🔣️policy.json` (`remoteCache.server.port` 6090, host 127.0.0.1). The server is the repo's own `remote-cache serve` (`LIB/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts`). It is not a per-invocation "fixed cache" server. The "fixed" in `nx:fixed-cache` is the local native-image storage lock (`LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts:90-95`), not an HTTP server.
   - Token: stateless. `authorize()` (`🖥️server/🟦️.ts:60-67`) compares the presented bearer with `timingSafeEqual` against the configured write/read tokens. There is no issuance, lease, pid binding, deadline or rotation. With neither token configured, the server is open (`:61`).
   - 401 condition: no `Authorization: Bearer …` header, or a bearer that matches neither configured token (`:62-65`). The body is `Unauthorized`, which matches the text Nx printed.

2. **Duration.** No. Nothing in the cache path depends on elapsed time. There is no expiry field in `RemoteCacheServerStatusV1`, `RemoteCacheTokensV1` or the policy schema (`LIB/🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json`). The only time-related server setting is `maintenanceSeconds: 300` (`LIB/🏗️build/🔣️policy.json`, `remoteCache.server`), which re-reads the store directory. It does not touch auth. The "27 minutes" is therefore not the cause by itself. The 401 was a token mismatch at the moment of the PUT, after the task had already succeeded. The "Original process admission closed" line is the retained-operation lifetime refusing new work after the first recorded failure (`LIB/🧰…/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🪢️lifetime/🟦️.ts:12,15,23`). It is a consequence, not a timer. The exact time bound on the Nx invocation was not located.

3. **Who sets the variables.**
   - `LIB/🏗️build/🌐️remote-cache/…` is not the writer. The writer is `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🟦️.ts:102-105`, `machineEnvironmentV1`. It renders `SEMIO_REMOTE_CACHE_ENDPOINT = group.cache.endpoint`, `SEMIO_REMOTE_CACHE_ACCESS = read-write`, `SEMIO_REMOTE_CACHE_INSTANCE = group.cache.instance`. It does not render a token.
   - Installed by `🤝️shared-worktree/🗺️topology/🥾️provision/🐚️.sh` step 5 (about lines 533-552). It runs `fleet env --group … --shell sh` and writes the result to `/etc/profile.d/semio-fleet.sh` (Linux), or to `launchctl setenv`, `/etc/zshenv` and `/etc/profile` (macOS). The LaunchDaemon env is also updated.
   - `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK`, optional `SEMIO_FLEET_CACHE_ENDPOINT` override the group address for one run (`🗺️topology/🟦️.ts:74-82`).
   - Token: `SEMIO_REMOTE_CACHE_TOKEN` is never written to disk by the repo (`🥾️provision/🐚️.sh:19`). Provisioning copies it from `$HOME/.profile` or `$HOME/.zprofile` into a root-readable file (`:308-317`). The user's `~/.zprofile` (mode 600) was checked by name only.
   - Intended way to point at another host: set `SEMIO_REMOTE_CACHE_ENDPOINT` (and `SEMIO_REMOTE_CACHE_TOKEN`), or change the topology cache endpoint / `SEMIO_FLEET_CACHE_ENDPOINT`.
   - Intended way to disable for one run: `SEMIO_REMOTE_CACHE_NX=off`. This is checked at `LIB/🏗️build/🌐️remote-cache/🧶️nx/🟦️.ts:7`. The test at `LIB/🏗️build/🗄️cas/🧪️tests/🟦️.ts:276` confirms it. README line 238 says "off until ENDPOINT is set", but that is not what happens when a local server is listening.
   - `env -u SEMIO_REMOTE_CACHE_ENDPOINT` alone does NOT disable Nx caching. It removes the contract, so the local probe takes over (`🧶️nx/🟦️.ts:9-11`). This is the trap that caused the failed run.

4. **Flaky task.** The string "Nx detected a flaky task" does not occur in the repo or in `node_modules/nx`. Nx's behaviour here is unverified. The repo does not special-case it. The practical reading is that a task whose cache PUT failed was not stored, so the next run should recompute it (a miss), not reuse it. This is inference.

## Live state observed (read-only probes)

- `lsof`: two bun listeners on port 6090. PID 25461 on `*:6090`, started Nov 11 17:45 local. PID 44123 on `127.0.0.1:6090`, started Nov 12 19:11 local (18:11Z).
- `GET /healthz` returned 200.
- `GET /status` with no auth returned 200. The server answering that call is open, with `startedAt 2026-11-12T18:11:27Z`, `readOnly false`, `validation complete`.
- Timeline: the Nx daemon log stamp is 17:41:35Z. The run lasted 28m10s, so it ended about 18:09:45Z. The current `startedAt` is 18:11:27Z, about 2 minutes after the failure. The server answering during the failed PUT was therefore a different process, probably one started with tokens. That is a hypothesis, not verified.

Most likely cause, in order:
- (a) the server that was listening during the run required a bearer token. The Nx client sent `local`, or no matching token, because `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN` and `READ_TOKEN` were not in the environment Nx received. The GETs of the earlier short tasks would also have failed, but Nx treats a GET 401 as a miss, so those tasks looked fine. Only the PUT is fatal.
- (b) the server was restarted or its tokens changed during the run (the new `startedAt` is about 2 minutes after the failure). Ruled in or out by the server log, which was not inspected.
- (c) the stale wildcard listener (PID 25461, from Nov 11) answered some requests and the current one others. Two listeners on one port should not coexist normally.

## Side effect (disclosed)

While checking auth, I ran `PUT http://127.0.0.1:6090/v1/cache/semio-audit-probe-000` with no token and `Content-Length: 0`. It returned 200 and stored an empty `nx` entry in the live local cache. The server has no DELETE route, so I could not remove it. It is idle and will be removed by the store's LRU/age collection (`maximumAgeDays` 30). It is harmless, but it was a write I should not have made.

## Next steps (not performed)

- Read the server's log or start-time arguments to see whether it had `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN` set, and whether PID 25461 and PID 44123 are both live. Stop the stale listener so only one serves 6090.
- For one run without the cache: `SEMIO_REMOTE_CACHE_NX=off repo run …`.
- To use the group cache: `SEMIO_REMOTE_CACHE_TOKEN` must be set in this account's environment, and the endpoint must be reachable. The endpoint `http://192.168.178.71:6090` is down per the earlier error.
- Fix the doc/behaviour mismatch: README line 238 says Nx is off without an endpoint, but `🧶️nx/🟦️.ts:11` probes the local server anyway.
