# Map of the Existing Remote Cache

Read-only exploration of `/Users/Shared/semio`. Paths below are relative to the module root
`M = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/` unless absolute.
Guesses are marked **(guess)**.

## Headlines

1. The cache server is `repo remote-cache serve`: a Bazel HTTP cache (`/ac/`, `/cas/`) plus the Nx self-hosted cache API (`/v1/cache/<key>`), in one Bun process. It is running on this machine right now (launchd `tech.semio.fleet.cache`, port 6090, 200 GiB bound). It holds almost nothing (2 entries).
2. Only two clients use it: the Nx backend (task outputs, through `NX_SELF_HOSTED_REMOTE_CACHE_*`) and the graph-generation exchange (Bazel `/ac/`, `/cas/`). The semio backend uses the Nx route. The Bazel backend has flags and a credential helper but no build path in the repo calls it for real builds.
3. Cargo/rustc is NOT in the remote cache. The rustc unit cache (`⚡️caching/🦀️cargo/🗃️unit-cache`) is local only. Plugin wasm builds, Go, and `repo` commands are not remote-cached by any code I found (plugin wasm: guess, it runs through cargo).
4. Why `status` says 401: this shell has `SEMIO_REMOTE_CACHE_ENDPOINT`, `..._ACCESS`, `..._INSTANCE` but NOT `SEMIO_REMOTE_CACHE_TOKEN` (the client token variable). The probes send no Authorization header, the server answers 401, so `server:null`. `store:null` is because `status` looks in the worktree-relative default directory, not the daemon's `/var/lib/semio-remote-cache`.
5. Consequence on this machine: Nx gets 401, the probe fails, Nx runs fully local with one bilingual warning. No remote hits happen here.
6. Entry lifetime: no notion of generations or synchronized state. Keys are content digests (ac, cas) or Nx hashes (nx). Eviction is 30-day idle age (touch on read), then LRU to 90% of the bound. Age is measured from file mtime, which reads touch.
7. Tests: main file 20 pass, 2 skipped (real Bazel and real Nx e2e, gated by env vars); server 6 pass; collector 7 pass; action-result file fails to load because `protobufjs` is not installed or declared anywhere.
8. Blobs are not in the server directory. Blobs (`cas`) and the Nx archive bodies live in the host CAS `~/.cache/semio/cas` (8.2 GB, local disk of the daemon account, not NFS). Only index files live in `/var/lib/semio-remote-cache/v1`.
9. Gaps vs goal: no shared store across machines (one local daemon, not NFS-backed), no per-OS path normalization for rustc, no remote path for cargo, no upload-on-build for cargo, token is per-account env, and the fleet docs and launchd job disagree on LaunchAgent vs LaunchDaemon.

---

## 1. The server

**Protocols**: `🖥️server/🟦️.ts`
- `RemoteCacheServerStatusV1.protocols` = `["bazel-http","nx-self-hosted"]` (line 16).
- Route parser `parseRemoteCachePathV1` (lines 48-54): optional `/<instance>` prefix (instance regex `^[a-z0-9][a-z0-9._-]{0,63}$`), then `/ac/<sha256>`, `/cas/<sha256>`, or `/v1/cache/<key>` (key `^[a-z0-9_-]{1,128}$`). Reserved first segments: `ac cas v1 healthz status`.
- Handler `createRemoteCacheHandlerV1` (lines 56-138): `GET /healthz` (no auth), `GET /status` (auth, JSON with counters, lines 88-92), GET/HEAD (auth), PUT (auth, write token only, lines 103-130). Read-only mode refuses PUT with 403 (line 105).
- Bazel `ac` PUT is validated (`validation` default `complete`, `actionResults:"complete"` in policy line 41): the action result must parse and every referenced blob must be present (`🔬️action-result/🟦️.ts` lines 127-142). AC GET re-checks the blobs and drops stale results (lines 77-83).
- CAS PUT is digest-verified (`verify: kind==="cas"`, line 121).
- Nx archives (`nx`) PUT require `Content-Length` (line 107), and are conflict-409 if they already exist (line 124).

**Auth** (lines 9, 60-67): bearer tokens. No tokens = open. A read token allows GET/HEAD, a write token allows all. Comparison is constant time (`timingSafeEqual` on SHA-256 digests, line 45). Listening beyond loopback without a token is refused (line 156). Tokens come from env: server side `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` / `..._WRITE_TOKEN` (`🔣️policy.json` lines 49-52); client side `SEMIO_REMOTE_CACHE_TOKEN` (policy line 30).

**Storage layout** (`💾️store/🟦️.ts`):
- Store root = `<directory>/v1`. Entry path = `<namespace>/<kind>/<first 2 chars>/<key>` (line 63).
- Writes go to `v1/.tmp/<pid>-<uuid>/` then `rename`/`link` (atomic, lines 175-208). Partial writes of dead PIDs are removed on open (lines 54-55).
- For `nx` entries when a host CAS is given (`hostStore`), the file at the path is only a JSON reference `{"version":1,"hash":"<sha256>","size":N}` (lines 169-179), and the body lives in the host CAS (`CasStoreV1`, `🗄️cas/🗃️store/🟦️.ts`). `cas` entries are always delegated to the host CAS (lines 98, 115, 124, 137, 164-167).
- `casRootV1` (`🗄️cas/🗃️store/🟦️.ts` lines 43-46): `$SEMIO_CAS_ROOT`, else `%LOCALAPPDATA%\semio\cas` (Windows), else `$HOME/.cache/semio/cas`.

**Limits** (`🔣️policy.json` lines 32-43, defaults; the serve command can override some):
- Port 6090, host `127.0.0.1` in committed policy; `--host 0.0.0.0` and `--port` are passed by the fleet unit.
- `maximumBytes` 2,684,354,560 (2.5 GiB) in policy (line 36). **The fleet unit overrides it with `--max-bytes 214748364800` (200 GiB)**, from `🤝️shared-worktree/🗺️topology/🟦️.ts` line 27 (`CACHE_BYTES`).
- `maximumEntryBytes` 536,870,912 (512 MiB), further capped at `maximumBytes/2` (`⌨️command/🟦️.ts` line 121).
- `maximumAgeDays` 30 (line 38). `maintenanceSeconds` 300 (line 43): a timer re-scans the directory and collects (`🖥️server/🟦️.ts` lines 170-178).
- `maximumActionResultBytes` 4 MiB (line 42); request body cap 2x `maximumEntryBytes` (line 166).
- Idle timeout 60 s (line 166). TLS optional (`tls: null`, line 40).

**How it is started**:
- Command: `repo remote-cache serve [--host] [--port] [--directory] [--max-bytes] [--read-only] [--tls-certificate] [--tls-key] [--lang]` (`⌨️command/🟦️.ts` lines 27-31, 103-144).
- Fleet unit (topology, `🤝️shared-worktree/🗺️topology/🟦️.ts` line 322): `repo remote-cache serve --host 0.0.0.0 --port <port> --directory /var/lib/semio-remote-cache --max-bytes 214748364800`. macOS agent at line 368 with `set -a; . /etc/semio/remote-cache.env; set +a; exec ... `. Windows task at line 406.
- Per-host graph service also hosts an endpoint: `🏠️hosted/🟦️.ts` (lines 13-27) starts the server inside the graph service at policy host/port, `directory = <cas root>/remote-cache` (line 17). Started from `⌨️command/🌐️host/🟦️.ts` line 9 and `🧪️` callers. On this machine `~/.cache/semio/cas/remote-cache` exists (8 KB, `v1` subdir only).

## 2. Clients and what is cached

### 2a. Nx task outputs (the main path)
- Routing: `🏗️build/🌐️remote-cache/🧶️nx/🟦️.ts` lines 8-27. Sets `NX_SELF_HOSTED_REMOTE_CACHE_SERVER=<endpoint>/<instance>` and `NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN` (token value). Switch `SEMIO_REMOTE_CACHE_NX=off` disables it (line 9). If a contract is configured, one probe `GET <base>/v1/cache/semio-probe` with 1.5 s timeout (lines 13-21); only 200/404 counts as reachable. Unreachable/401 → routing removed, one warning printed (lines 16-18).
- Without a contract, it falls back to the local server at `policy.server.host:port` if `/healthz` answers (lines 23-26), with the token `"local"` if no token is set.
- Caller: `⚡️caching/🚀️bootstrap/📜️script.ts` line 454 (`Object.assign(env, nxRemoteCacheEnvironmentV1(env))`), inside the Nx run path (`runInStorage`, line 405). Also `🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts` (renders the env for `status`).
- Key: computed by Nx, not by semio. Namespace = instance (`semio` on this machine); route `/<instance>/v1/cache/<hash>`. **(guess)** Nx hashing does not include the absolute workspace path, but I did not verify this against Nx 23 sources. The memory note "Nx Hasher Blind To Gitignored Inputs" is a separate known gap.
- Upload: Nx PUTs its archive (tar of task outputs) under the Nx key; the write token is needed for PUT; `SEMIO_REMOTE_CACHE_ACCESS=read-write` plus `upload` true are needed for effective upload (`remoteCacheEffectiveUploadV1`, `🟦️.ts` line 59). Nx has no switch for upload; upload is enforced by token scope only (`🔧️tools/🧶️nx`... the Nx tool rendering, `🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts` line 14: `uploadEnforcedBy`).
- Which tasks are cacheable: decided by Nx config (`nx.json`, `project.json`) — not inspected in detail **(guess: targets with cache enabled, including TypeScript/vite builds and tests)**. `nx.json` line 84 excludes the remote-cache folder from Nx inputs.

### 2b. Graph generations (semio's own generation exchange)
- `🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts`: uploads and restores graph generations through the Bazel HTTP routes `/<instance>/ac/<sha256>` and `/<instance>/cas/<sha256>` via `httpRemoteExecutionContentV1` (`🌐️remote-execution/🗄️content/🟦️.ts` lines 18, 29-33).
- Key `generationExchangeKeyV1` (line ~22): `sha256([domain, identity, workspace-absolute-root])` — **includes the absolute workspace root**, so `/Users/Shared/semio`, `/srv/semio`, `C:\semio` produce different keys (matches the topology's canonical-path rule).
- Callers: `remoteCacheEffectiveUploadV1`, `readRemoteCacheTokenV1`, `resolveRemoteCacheContractV1` (line 5).

### 2c. Semio backend (`🔧️tools/🐙️semio/`)
- `🔌️backend/🟦️.ts` lines 78-79: builds a remote client `semioHttpRemoteV1({url, token})` and calls `get/put` (put only when upload). The HTTP client `🌐️remote/🟦️.ts` line 45 uses `/v1/cache/<key>` (the Nx route). Keys are semio's own (not read in detail).

### 2d. Bazel backend (`🔧️tools/🏗️bazel/`)
- `🌐️remote-cache/🟦️.ts` lines 19-26: renders flags `--remote_cache`, `--remote_instance_name`, `--remote_upload_local_results`, `--remote_cache_compression`, `--remote_download_outputs`, `--remote_timeout`, `--remote_retries`, plus a credential helper (lines 11-16) so the token is not on the command line.
- The Bazel tool is a real backend (`🔧️tools/🏗️bazel/🔌️backend/🟦️.ts` line 28). Only its tests (`🌐️remote-cache/🧪️tests/🟦️.ts` lines 364-366) run a real Bazel client. I did not find a production flow that always passes these flags **(guess: Bazel is an opt-in backend)**.

### 2e. Not remote-cached (checked)
- **rustc / cargo** (`⚡️caching/🦀️cargo/🗃️unit-cache/`): `RUSTC_WRAPPER` set to the shim (`🚪️launch/🟦️.ts` lines 82-84); keys (`🔏️key/🟦️.ts`, `unitKeyV1`, `unitKeyPartsV1`) = sha256 of [namespace, toolchain, lane-neutral normalized argv, source sha256, extern sha256s, selected env]. Lane root replaced by a token (`laneNeutralV1`, `laneRestoreV1`), output and incremental dirs dropped. The key is portable across lanes **but the store is local**: no grep hit for `remote` in the unit-cache except its schema. Unit = one rustc invocation (per crate per flags), not the whole target dir. Bypass rules: `rustcBypassV1` (no artifact messages, `--print`, native link crate types, etc.).
- **Cargo build-dir** itself (`CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR`, `🗂️directories/🟦️.ts`): local, per lane. `cargo-provenance` gates shared build-dirs against foreign absolute paths (`🧾️provenance/🟦️.ts` lines 73, 154-158).
- **Plugin wasm builds**: no reference found in the remote cache module. **(guess)** they go through cargo and the local unit cache.
- **Go**: no Go build backend or remote reference found in `🔧️tools`. Not covered.
- **`repo` commands / resident service**: `🏡️resident` has no `remote` reference. The `repo` launcher is not cached.
- **Remote execution** (`🌐️remote-execution/`): a separate REAPI-style system (`⌨️command`, `🖥️server`, `👷️worker`). It shares the endpoint code and `readRemoteCachePolicyV1` but is not the cache. It is out of scope here.

### 2f. Build-path absolute paths and per-OS roots
- Rustc key: the lane root is neutralized, but `CARGO_MANIFEST_DIR`, `--remap-path-prefix` values and `-L` non-dependency paths are kept as-is in the normalized argv (only `laneNeutralV1` substitution). **(guess)** a different canonical checkout produces different keys, unless the lane root equals the canonical path.
- Nx: see 2a guess.
- Generation key: absolute workspace root included (2b).
- So a cache hit across `/Users/Shared/semio`, `/srv/semio`, `C:\semio` is **not** expected from the code paths I read, except where the lane token replaces the path.

## 3. How cargo is invoked (search results)
- `RUSTC_WRAPPER` / `RUSTC_WORKSPACE_WRAPPER`: set in `🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts` line 84 and read in `💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts` line 48.
- Build dirs: `CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR` set in `🦀️cargo/🌿️environment/🟦️.ts` line 9 and `🏃️process/🟦️.ts` line 89.
- `sccache`: no reference in the module tree.
- `--remap-path-prefix`: only in the rustc argv parser (`🔏️key/🟦️.ts` valueFlags), not set by the build.

## 4. Live state on this machine (2026-11-13, about 21:40 local)

Commands run: `repo remote-cache status --lang en`, `launchctl print system/tech.semio.fleet.cache`, `ps`, `lsof`, `curl`, `du`, `find`.

- Server process: pid 850, parent pid 1 (launchd), started about 3 h 55 min before my run. Command: `bun .../script.ts repo remote-cache serve --host 0.0.0.0 --port 6090 --directory /var/lib/semio-remote-cache --max-bytes 214748364800`.
- Port 6090 is listened on by pid 1062 (`bun --eval`, child of 850, started 22 s after 850). I did not dig further into 1062's script.
- launchd: `system/tech.semio.fleet.cache` **state = running**, type **LaunchDaemon**, plist `/Library/LaunchDaemons/tech.semio.fleet.cache.plist`, `UserName = ueli`, program `/bin/sh -c 'set -a; . /etc/semio/remote-cache.env; set +a; exec /usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 6090 --directory /var/lib/semio-remote-cache --max-bytes 214748364800'`, working dir `/Users/Shared/semio`.
  - Discrepancy: `🤝️shared-worktree/README.md` (line 56) and `🗺️topology/🟦️.ts` (line 368, `agentOf`) say the macOS cache service is a per-user LaunchAgent in `gui/<uid>`. The installed job is a system LaunchDaemon.
- `/etc/semio/remote-cache.env`: mode 0600, owner ueli, 205 bytes. Variable **names only** (values not printed): `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN`, `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`.
- Client env in this shell: `SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.71:6090`, `SEMIO_REMOTE_CACHE_INSTANCE=semio`, `SEMIO_REMOTE_CACHE_ACCESS` set (values not hidden for endpoint/instance, which are not secrets). `SEMIO_REMOTE_CACHE_TOKEN` **not set**. Profile files `~/.zprofile`, `~/.zshenv`, `/etc/zshenv`, `/etc/profile`: no `SEMIO_REMOTE_CACHE_TOKEN` line.
- Health: `GET http://127.0.0.1:6090/healthz` returns `{"status":"ok"}` HTTP 200.
- Unauthenticated probe `GET http://192.168.178.71:6090/status` returns HTTP 401 (as expected).
- This machine has both LAN addresses 192.168.178.70 and 192.168.178.71 (`ifconfig`). The endpoint in the environment uses .71.

**Why `repo remote-cache status` says `authorized:false`, `server:null`, `store:null`**:
1. `remoteCacheStatusV1` (`⌨️command/🟦️.ts` lines 174-190) reads the token from `SEMIO_REMOTE_CACHE_TOKEN` via `readRemoteCacheTokenV1` (`🌐️remote-cache/🟦️.ts` lines 64-69). It is undefined here, so the probes (lines 148-158) and `askServerV1` (lines 160-171) send no `Authorization` header.
2. The server answers 401 to both `/v1/cache/semio-probe` and `/status`. `authorized` is true only for 200 or 404 (line 153), so it is false. `askServerV1` returns null on non-200 (line 163), so `server:null`.
3. `store` is computed from `all.server.directory` (line 175), which is `.🧬semio/🦑️repo/⚡️cache/remote-cache` under the worktree (policy line 35). That directory does not exist (checked), so `store:null`. The daemon writes to `/var/lib/semio-remote-cache`. `status --directory /var/lib/semio-remote-cache` would show it (the option is accepted, line 32).
4. The token value must come from the account environment (fleet-setup.md: `export SEMIO_REMOTE_CACHE_TOKEN=...` in `~/.zprofile`, mode 0600). Not present here. I did not print or search for any token value.

Note: `repo remote-cache status` first printed `[script] … build:source-admission … build:implementation-source` progress lines before the status command ran. That is the repo launcher's own bootstrap, not a build. I saw no cargo, rustc or nx child processes from my commands afterwards.

**Store contents** (`/var/lib/semio-remote-cache`, owner ueli):
- Total size 756 KB (`du -sh`). `v1/semio/ac`: 1 file (`9e/9e1a…`). `v1/semio/nx`: 1 file (`15/15595431756162111061`), a JSON reference `{"version":1,"hash":"86b5e670…","size":17871383}` pointing to a 17.9 MB host-CAS blob. `v1/semio/cas`: none. `v1/.tmp/`: one leftover directory from pid 1062.
- Blobs live in the host CAS: `~/.cache/semio/cas` (8.2 GB total). I did not locate blob `86b5e670…` by name at depth 3; the CAS layout was not read further **(guess: under `~/.cache/semio/cas/v1`)**.
- Oldest and newest file mtimes in the server directory: 2026-11-13 17:47 and 21:37.

## 5. Entry lifetime and eviction

- **No generation or synchronized-state dimension.** Keys are content digests (`ac`, `cas`) or Nx archive keys (`nx`). Two states of the same source produce two keys. Nothing in the store knows about "current" state. The graph-generation exchange does have a notion of generation (a pointer and manifest, `🕸️graph/🕰️generations/`), but that is in the NFS store / generation files, not in this cache.
- **Idle age**: `planRemoteCacheCollectionV1` (`🧹️collector/🟦️.ts` lines 31-44): entries with `now - accessed > maximumAgeMilliseconds` are dropped first (reason `age`). `accessed` = file mtime; every read `utimes`-touches the file (`💾️store/🟦️.ts` lines 80, 109; `readBytes` goes through `read`). So age is "idle time since last read or write", 30 days by default.
- **Budget**: if `bytes + reserved > maximumBytes - need`, the oldest entries (by access time, ties by key) are dropped until the total is at most `floor(maximumBytes * 9/10) - need` (lines 38-41, low-water 90%).
- **Dangling**: action results whose referenced blobs are gone (or malformed, or larger than 16 MiB) are dropped (reason `dangling`, `findDanglingRemoteCacheResultsV1` lines 91-105), only when `actionResults === "complete"`.
- Where it runs:
  - Server store `open()` runs `refresh()` (scan and collect) (`💾️store/🟦️.ts` lines 50-58, 85-90).
  - Server writes trigger `collect` when over bound (lines 184, 214).
  - Maintenance timer every 300 s (`🖥️server/🟦️.ts` lines 170-178), guarded by the collector lock.
  - `repo remote-cache gc [--dry-run] [--json] [--max-bytes]` (`⌨️command/🟦️.ts` lines 193-215).
  - `repo cache gc` ledger calls it for owner `remote-cache` (`🧹️cache-gc/⌨️command/🟦️.ts` line 186; policy `💾️storage/🧾️ledger/🔣️policy.json` lines 1329-1344).
- Locks: `.server.lock` (one server per directory) and `.collector.lock` (one collector), both created by `acquireRemoteCacheLockV1` (`🧹️collector/🟦️.ts` lines 70-88). A lock of a dead PID or older than 6 h is taken over.
- Nothing deletes by `repo` version, by branch, or by fleet generation.

## 6. Tests

Run with `/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a…/bun test ./🟦️.ts --timeout 120000` from each test directory, with `SEMIO_TEST_BAZEL_BINARY` unset. No builds were started by the tests I ran (the Bazel and Nx e2e tests are skipped).

| File | Result |
|---|---|
| `M/🧪️tests/🟦️.ts` (policy, schema, Bazel/Nx renderers, route parser, wire, store, eviction, concurrency, progress, idle age, command words, messages, status, serve, gc, schema-first JSON, Nx fallback) | 20 pass, 2 skip, 0 fail, 760 expects. The skips are `test.skipIf(!bazelBinary)` (line 366) and `test.skipIf(!nxBinary)` (line 432): real Bazel and Nx clients, not run. |
| `M/🖥️server/🧪️tests/🟦️.ts` | 6 pass, 0 fail (uses `curl` and an ephemeral port). |
| `M/🧹️collector/🧪️tests/🟦️.ts` | 7 pass, 0 fail (uses the Python oracle). |
| `M/🔬️action-result/🧪️tests/🟦️.ts` | **Fails to load**: `Cannot find package 'protobufjs'` (line 3 imports it as the third-party oracle). `protobufjs` is not declared in any `package.json` I checked. 0 tests ran. |
| `M/🔬️action-result/🧪️tests/🐍️oracle/🐍️.py` | Not run (oracle script, used by the tests above). |
| `M/🏠️hosted/`, `M/⌨️command/`, `M/🧶️nx/`, `M/💾️store/` | No own test files. Covered through the files above and `🗄️cas/🧪️tests/🟦️.ts` (lines 208, 260, 281, 314). Not run. |

## 7. Gaps between what exists and the goal

1. **No shared store**: the server keeps its index and CAS on the local disk of one machine (`/var/lib/semio-remote-cache` and `~/.cache/semio/cas`). The NFS store (`/srv/semio-store`, `/Volumes/…`) is not used for cache data. A machine that builds gets hits only through the HTTP endpoint, which is one process.
2. **Cargo/rustc is not remote**: the rustc unit cache (`⚡️caching/🦀️cargo/🗃️unit-cache`) is local only. There is no upload or download of rustc units, and no code path from cargo to `/ac`/`/cas`. This is the largest gap for "nothing is built twice".
3. **Plugin wasm, Go, and `repo` commands**: not covered by any remote cache code I found.
4. **Path portability**: the rustc key neutralizes only the lane root. Absolute source paths, `--remap-path-prefix`, `-L` paths and the generation key (includes the absolute workspace root) differ per OS path (`/Users/Shared/semio`, `/srv/semio`, `C:\semio`). **(guess)** Nx keys do not include the absolute path, but unverified.
5. **Token distribution**: the account must set `SEMIO_REMOTE_CACHE_TOKEN` manually in its profile (fleet-setup.md). The token is not in the environment of this shell, so every client here is unauthorized. Workers and the launchd daemon read different variables (`SEMIO_REMOTE_CACHE_SERVER_*_TOKEN` in `/etc/semio/remote-cache.env`).
6. **`status` reads the wrong store directory by default**: it reads the worktree-relative default, not the daemon's `/var/lib/...`. `--directory` must be passed.
7. **LaunchDaemon vs LaunchAgent**: the installed job is a system LaunchDaemon (`UserName=ueli`), but the docs and the topology generator say a per-user LaunchAgent.
8. **Blob store is outside the server directory**: `status` and `gc` only see the index. Disk usage of the blobs is not reported by `status` (only the index).
9. **Namespace fixed by instance**: all clients share `semio` (one namespace), so there is no per-generation or per-OS namespace. This is by design but means no "current state" separation.
10. **Test debt**: `protobufjs` is missing for the action-result oracle test. The Bazel and Nx e2e tests need binaries that are not in this environment.
11. **Unverified**: Nx namespace binding (`bind` in `📜️script.ts` line 455) may change the remote URL; I did not confirm it does not.
12. **Concurrency of the daemon and the NFS export**: out of scope here, but the server's `.tmp` directory is per-PID and cleaned only on open of the same directory.
