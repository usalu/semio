# 🔐️ Security audit: fleet cache (6090) and remote execution (8980)

Read-only review, 2026-11-13. Facts from code and read-only commands; guesses are marked **(guess)**; things not checked are marked **(not verified)**.

Module roots (absolute):
- `RE` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/`
- `RC` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/`
- `SB` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/` (hermetic table, `🔣️.json`)
- `POL` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`
- `FETCH` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/`
- `UC` = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/`

Ticket inputs read first: `design.md`, `result-A.md` to `result-I.md`, and the persisted continuation of `result-H.md`.

## Headlines

1. **Critical.** The fleet credential is the cache server's own WRITE token. It sits in `/Users/Shared/semio-store/v1/fleet/cache.json`, and that store is exported over NFS to the whole `192.168.178.0/24` with `-mapall=ueli` (`/etc/exports`). Any LAN host that mounts it reads the token. Mode 0600 does not help, because mapall makes every client the owner. The same export is writable.
2. **Critical.** The token holder can run arbitrary commands as `ueli` on all 8 Macs. Schedule any action through the scheduler (`RE/🖥️server/🟦️.ts:203-210`), and the agents execute it (`RE/👷️worker/🟦️.ts:193-214`). The NFS-writer can also point the facts at its own scheduler, so agents join it (`RC/🛰️fleet/🟦️.ts:88`, `RE/🛰️sidecar/🟦️.ts:36-56`). Read-only and unauthenticated clients are refused (verified by 401/403 in code and by probes).
3. **Critical.** No TLS anywhere by default. Scheduler gRPC, worker HTTP and cache are plain HTTP/2 or HTTP with `Authorization: Bearer` on every request (`RE/📡️wire/🚪️grpc/🟦️.ts:159`, `RE/🖥️server/🟦️.ts:230`, `POL` remoteExecution, topology `--host 0.0.0.0` without TLS). Any sniffing or ARP-spoofing LAN host captures the token, or answers as the endpoint.
4. **High.** Sandbox: macOS denies reads under `/Users` and writes outside the action root, and denies network unless asked (`RE/🔒️sandbox/🟦️.ts:24-28`). Linux binds the whole host read-only, so `~/.ssh` and the worktree token file are readable (`RE/🔒️sandbox/🟦️.ts:36`). Windows has no sandbox at all (`SB/🔣️.json` win32: `remote, isolated-root`). `confinement: best-effort` silently accepts unconfined workers (`POL`).
5. **High.** Cache poisoning needs only the write token. `ac` PUT is `replace` (`RC/🖥️server/🟦️.ts:79`). Validation checks blob presence, not that the key matches the action (`RC/🔬️action-result/🟦️.ts:128-140`). Clients check blobs against the manifest from the same response. Nothing binds an entry to who compiled it. Read-only tokens get 403 (`RC/🖥️server/🟦️.ts:77`).
6. **High.** Store-trusted installs: `fleetInstallRestoreV1` checks the archive against its own manifest's sha256 and shape (`FETCH/📦️install/🟦️.ts:191-192`, `:207-209`). Anyone who can write the NFS store (point 1) can substitute `node_modules` for any lock state, and build tools run from it. Pins for rustup, toolchains and crates are checked against repo pins, so those only fall back to upstream.
7. **High.** Shared token, no role split: `read`, `write`, `worker` and the scheduler all use one value (`RE/🛰️sidecar/🟦️.ts:87`). A worker-token holder can register a fake worker and receive other people's actions (source, env) and return outputs that the requester publishes under the unit key (offload, result-E).
8. **Medium.** One client can hold a claim forever by renewing it (`RC/🎟️claims/🟦️.ts:65-69`, TTL max 300 s, no lifetime cap). Waiters then stall to their bounds (up to 20 min per unit, B; 600 s, Nx), not forever. Actions can run 4 hours (`RE/🗓️scheduler/🟦️.ts:89-90`), queue depth 100 000.
9. **Medium.** Mirror: unauthenticated GET starts upstream fetches with no concurrency cap (`RC/🪞️mirror/🟦️.ts:28`, `:47-51`), keys include query strings, and redirects are followed to any host (`:56`). Integrity relies on client hash checks; Playwright has none.
10. **Medium.** Token is delivered in the environment of every child (`FETCH`/`UC` launch, `RC/🛰️fleet/🟦️.ts:88`), not in argv (good). Local copies are 0600 on POSIX (verified). Windows ignores POSIX modes (A). No token value found in logs or status output (grep plus status renderings redacted).
11. **Medium.** Live probe: 6090 answers 401 to every unauthenticated valid cache address and to `/status` (good). Routes that exist only in the new code answer 404 (old server). 8980 refused on loopback at probe time.
12. **Interim state, verified.** The credential currently in the store equals `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN` in `/etc/semio/remote-cache.env`. Result-A calls this interim. It is not yet the dedicated credential.

Report: `/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/audit-security.md`

## Q1. Who can submit actions, authentication, binds, TLS, sandbox

**Who can submit.** Only holders of the fleet token. Authentication is a bearer token compared in constant time (sha256 + `timingSafeEqual`, `RE/🖥️server/🟦️.ts:26`):
- gRPC: `authorize` runs on every call (`RE/🖥️server/🟦️.ts:203-210`). Execute, ByteStream write and claims need a writer. Only READ_ONLY methods accept a reader (`:24`).
- Worker HTTP (`/v1/workers`, `/poll`, `/heartbeat`, `/tasks/*/result`, `POST /v1/fleet/load`): `allowed(request,"worker")` (`:214-217`). Status and `GET /v1/fleet/load`: read, write or worker token (`:220`).
- Shortcut: when both read and write are unset, the server is open, and beyond loopback it refuses to start (`:198-199`). Worker API open is refused beyond loopback too.

**Binds and TLS.**
- The group sidecar binds `0.0.0.0` for gRPC and worker HTTP, with the fleet token as read, write and worker token (`RE/🛰️sidecar/🟦️.ts:87`).
- TLS only if a certificate is passed (`RE/🖥️server/🟦️.ts:231`, `RE/📡️wire/🚪️grpc/🟦️.ts:159`). The sidecar and topology pass none (topology `🗺️topology/🟦️.ts:324,370` passes `--host 0.0.0.0` for the cache; no `--tls-*`). `tls: null` in `POL`.
- Facts publish `http://` endpoints (`cache.json`, verified).

**Arbitrary execution.** The worker takes `Command.arguments` as given, with the command's own environment (`RE/👷️worker/🟦️.ts:193-202`). `resolveExecutable` accepts absolute paths (`:129-137`). There is no allowlist of programs. Authenticated submission therefore equals arbitrary command execution on the worker's machine, limited only by the sandbox.

**Sandbox, per platform.**

| Host | What is confined | Evidence |
|---|---|---|
| macOS (`darwin-sandbox`) | `(allow default)`; `deny network*` unless policy allows; `deny file-write*` except the action root and `/dev`; `deny file-read-data` under `/Users`, `/Volumes`, `/private/tmp`, `/private/var/folders`, `/private/var/tmp` | `RE/🔒️sandbox/🟦️.ts:24-28` |
| Linux (`linux-sandbox`, bwrap) | `--unshare-net` (unless allowed), `--unshare-pid`, `--ro-bind / /` (whole host readable), writable only the root | `RE/🔒️sandbox/🟦️.ts:36` |
| Windows | nothing: `win32: [remote, isolated-root]`, `isolated-root` = plain spawn | `SB/🔣️.json`; `RE/🔒️sandbox/🟦️.ts:57-58` |
| Any host where the probe fails | `auto` falls back to `isolated-root` silently | `RE/🔒️sandbox/🟦️.ts:71-78` |

- **High, Linux:** an action reads `~/.ssh`, the worktree's `.cache/f/cache-facts.json` (fleet token) and any user file, and returns them as outputs (`output_paths`) through the CAS. Outputs are not confined to the root, only their declared paths (`RE/👷️worker/🟦️.ts:234-240` checks parents).
- **High, Windows and fallback:** an action reads and writes anywhere the user can, and network is "unenforced" (`RE/👷️worker/🟦️.ts:304`).
- **High (policy):** `POL` `remoteExecution.worker`: `network: deny`, `confinement: best-effort`. Best-effort lets unconfined workers join. The scheduler does not refuse them.
- **Medium, symlinks:** input trees create symlinks with the given target and no check (`RE/👷️worker/🟦️.ts:185`). On unconfined hosts that escapes the root. On macOS and Linux the read/write rules still apply to the resolved path.
- **(not verified)** macOS: firmlink spelling `/System/Volumes/Data/Users/...`, Unix-domain sockets, mach services. Denial under `/Users` is path-based. I did not run a probe.
- macOS writes are confined to the root. Linux writes are confined to the root (ro bind). Unconfined hosts have no write confinement.

## Q2. Cache poisoning and the write credential

- **Read-only clients cannot write:** `authorize` returns 403 for a reader on PUT (`RC/🖥️server/🟦️.ts:70-77`). Unauthenticated: 401. Probes confirmed 401 on `PUT /main/ac/<64 zeros>` without credentials (rejected, nothing stored).
- **Write-token holder can poison any unit key.** `ac` uses `replace` (`RC/🖥️server/🟦️.ts:79`), so an existing legitimate entry can be overwritten. Validation (`RC/🔬️action-result/🟦️.ts:128-140`) checks that the action-result message parses and that referenced blobs exist. It does not check that the key is the action digest, there is no signature, and no worker identity is recorded. Blobs are verified against their own digests (`verify` for `cas`), which is self-consistent for an attacker. **High.**
- **Clients trust the same response.** B's fleet tier verifies each blob against the manifest from the same entry, plus the size (result-B). Poisoned manifest and blobs pass. The second-level input check uses recorded inputs from the same entry. **High, consequence of above.**
- **`nx` kind is first-writer-wins** (`keep`, 409 on conflict, `RC/🖥️server/🟦️.ts:79`). An attacker's Nx archive, once stored, is kept. Nx archives are restored into the worktree (G). **High (consequence).**
- **Supersession can delete others' entries.** Any writer's PUT with `x-semio-slot` makes older keys of that slot superseded, deleted after grace (C; `RC/🏷️retention/🟦️.ts`). **Medium.**
- **Offload poisoning.** The requester publishes the unit it received from a worker (E). A fake worker (worker token) returns arbitrary output bytes. **High.**
- **Who may write the mapping:** anyone with the write token. The `x-semio-slot` header is the only binding (C/B). There is no record of the uploader and no per-machine key. The write credential allows: PUT `ac`/`cas`/`nx`, claim/renew/release any key, supersede slots, status, Execute, worker registration (`RE/🖥️server/🟦️.ts:203-210`, `RC/🖥️server/🟦️.ts:138-139`, claims `RC/🎟️claims/🟦️.ts:125-148`).
- **Transport:** plain HTTP, so a LAN MITM can serve chosen manifests and blobs to any client (see Q1, TLS). **(guess)** this depends on LAN L2 protections, which I did not check.

## Q3. Mirror routes

- Served on 6090 before auth (`RC/🖥️server/🟦️.ts:98`). By design (clients cannot send the token), and content is public.
- Filling is one-way: the mirror directory is server-local, with no PUT route. A LAN attacker cannot write into it. **Good.**
- Integrity by artifact:
  - **rustup:** `dist/YYYY-MM-DD/` and `rustup/archive/` are kept forever (`RC/🪞️mirror/🧬️schema/🔣️.json`). Repo pins: rustup-init (sha256), dated manifest (sha256), every toolchain archive (`xz_hash` in the pinned manifest). Client-verified, so a bad mirror fails. Non-dated paths (for example stable channel metadata) are not pinned (**(guess)** impact depends on which channel is used).
  - **Go:** `go.sum` plus the sum DB. `GOPROXY` fallback `|direct`. Hash-pinned if `GOSUMDB` is not disabled (**not verified**: I did not check the repo's `GOSUMDB`/`GONOSUMDB`).
  - **npm/bun:** tarballs under `/-/*.tgz` are immutable; packuments revalidate after 300 s and may be stale. `bun.lock` carries sha512 integrity, so bun can verify tarballs (**(not verified)** that bun enforces it on the mirror path). The mirror is not wired in by default (H).
  - **Playwright:** `/builds/` kept forever, no hash anywhere. Trust is TLS to `cdn.playwright.dev` at fill time, then cached for everyone. **Medium.**
- Redirects: `redirect: "follow"` (`RC/🪞️mirror/🟦️.ts:56`) to any host. Only the first hop is allowlisted. Upstream is public and TLS-verified, so exploitability is low (**guess**).
- Path checks: literal `..` rejected (`:105`), but the URL is re-parsed by `fetch`. Percent-encoded dot segments may reach other paths on the same upstream host (**(guess)**, same origin only).

## Q4. Credential handling

- **Where read:** env only (`SEMIO_REMOTE_CACHE_TOKEN`, `SEMIO_REMOTE_EXECUTION_WORKER_TOKEN`, facts via `RC/🛰️fleet/🟦️.ts:88`). Not in argv: the `/bin/sh -c` line for the cache unit passes the token file path, not the value (`🗺️topology/🟦️.ts:370`). The sandbox argv contains only the profile.
- **Environment spread:** `resolveRemoteCacheV1` writes the fleet token into the returned environment (`RC/🛰️fleet/🟦️.ts:88`). Callers pass it on to Nx, cargo and the detached unit uploader (`UC/🛰️fleet/📤️uploader/🟦️.ts:54`) and to remote actions via their env. Any child can print it. **Medium.**
- **Logs and output:** grep of remote-cache, remote-execution, fleet-fetch and unit-cache for token interpolation into messages found none. `status` renderings redact the token (`RC`, `⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts:14`, `REDACTED`). `serve` prints `access: token` only (`RE/⌨️command/🟦️.ts:132`). **(not verified)** full sidecar log contents.
- **Local copies:** `.cache/f/cache-facts.json` is `-rw-------` ueli (verified). Atomic write at 0600 (`RC/🛰️fleet/🟦️.ts:120`). Windows ignores modes (A). **Medium on Windows.**
- **Store:** `/Users/Shared/semio-store/v1/fleet/cache.json` mode 0600 (verified), but mapall (see Headline 1). The code comment assumes the owner-only mode protects it (`RC/🛰️fleet/🔑️credential/🟦️.ts:129`). It does not. **Critical.**
- **Store write is also token theft:** the resolver pairs the fleet token with whatever endpoints the facts name (`RC/🛰️fleet/🟦️.ts:77`). Whoever writes `cache.json` receives the token on the next request from every machine, and steers agents to its own scheduler (`RE/🛰️sidecar/🟦️.ts:36-56`). **Critical.**
- **Rotation:** `rotate` writes a new token to the server's 0600 credential file, keeps the old one 15 min (`RC/🛰️fleet/🔑️credential/🟦️.ts:20,48-50`), republishes facts, and the sidecar restarts on change (`RE/🛰️sidecar/🟦️.ts:56,63`). Rotation fixes exposure only if the NFS export is closed first. It also rewrites the same store.
- **One token for everything** (see Headline 7). No per-machine or per-role credentials, no identity in logs.
- **Server write token = fleet credential (interim).** Verified without printing: the store credential equals `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN` in `/etc/semio/remote-cache.env` (0600). The env file also holds the read token (different value). Facts: generation 3, `access: read-write`, endpoints `http://192.168.178.70:6090`, `http://192.168.178.71:6090`, `http://cache.g1.semio.internal:6090`, execution `http://192.168.178.71:8980`.
- **Unit uploader:** token in env, inherited by a detached process that lives up to 15 s idle (`UC/🛰️fleet/📤️uploader/🟦️.ts:54`). Not a leak, but it lives past the Cargo process.

## Q5. Denial of service

| Surface | Bound | Who can trigger | Impact |
|---|---|---|---|
| Claims | TTL 1–300 s; renew by owner indefinitely; table 65 536; waiters 4 096 (`RC/🎟️claims` policy) | write token | One key held forever by renewals; waiters wait to bound (B ≤ 20 min per unit, Nx 600 s); dependency chains serialize. **Medium.** Table full returns 503, so builds fall back (fine). |
| Actions | Queue 100 000; per-action timeout up to 4 h (`RE/🗓️scheduler/🟦️.ts:89-90`); no per-client quota | write token | Occupy all worker slots for hours. Offload `if-idle` falls back to local. **Medium.** |
| Worker registration and load | `POST /v1/workers` and `/v1/fleet/load` take bodies up to 64 MiB (`RE/🖥️server/🟦️.ts:230`); no origin cap seen | worker token | Fake workers attract actions (confidentiality and poisoning, above). **High** with token; **(not verified)** table growth. |
| gRPC messages | 32 MiB per message, 4 MiB batch (`POL`) | write token | Bounded. |
| Cache entries | Entry ≤ 512 MiB; `ac` ≤ 4 MiB buffered; claim body ≤ 1 MiB; store cap 2.5 GiB by default (`POL` `remoteCache.server.maximumBytes`); 507 when full | write token | Fill the store with fresh entries; new writes refused for the grace period (600 s). **Medium.** |
| Mirror | 64 GiB LRU, no concurrency cap, query strings make new keys | unauthenticated (LAN) | Disk churn, upstream bandwidth, upstream rate limits. **Medium.** |
| Unauthenticated bodies | Idle timeouts 60 s (cache) and 120 s (worker API); 401 before body parse on PUT and worker routes | anyone | Slow-body connections are bounded by idle timeouts. **Low.** |

Single client holding claims cannot freeze all builds forever: waiters and fallbacks are bounded.

## Q6. Live probes (this machine, loopback only, status codes)

Probed: `GET` only, except one PUT noted below.

| Target | Result |
|---|---|
| 6090 `/healthz` | 200 |
| 6090 `/status` | 401 |
| 6090 `/main/ac/<64 zeros>` | 401 |
| 6090 `/main/cas/<64 zeros>` | 401 |
| 6090 `/main/v1/cache/semio-probe` | 401 |
| 6090 `/main/claim/ac/<64 zeros>` | 404 (route is in the new code, server runs old code) |
| 6090 `/mirror/`, `/mirror/healthz`, `/mirror/go/x` | 404 (mirror in the new code, not running) |
| 6090 `/v1/fleet/load`, `/ac/x`, `/cas/x`, `/v1/slots` | 404 (invalid or not cache routes) |
| 6090 `PUT /main/ac/<64 zeros>`, no credential | 401 (**one write attempt, rejected; not a GET**) |
| 8980 any path (`/status`, `/v1/fleet/load`, `/healthz`, HTTP/2 prior knowledge `/status`) | no answer (connection refused on 127.0.0.1) |

This machine owns `192.168.178.71` (the endpoint the facts name). I did not probe LAN addresses, as the scope was loopback. The scheduler was not reachable on loopback at probe time.

## Prioritized fix list

Each fix is the smallest change that closes the finding.

1. **Close the NFS store (Critical, Headline 1).** Export only to the eight machine addresses, read-only for every client except the group server, and remove `-mapall=ueli` for clients (or drop the export for `v1/fleet/cache.json`). Move the fleet facts out of the shared store or make the store read-only except the publisher. Immediate.
2. **Rotate the fleet credential now and stop sharing the server write token (Critical).** Replace the interim token in the store with the dedicated credential (`repo remote-cache rotate`), then separate read, write and worker tokens. Do it after step 1, or the new token leaks the same way.
3. **TLS on every fleet endpoint (Critical).** Certificate for 6090, 8980, 8981; facts publish `https://` and a certificate fingerprint; clients pin it. Without this, the token crosses the LAN in cleartext.
4. **Bind to the fleet interface only (High).** Sidecar and topology use `--host 192.168.178.71` (or the fleet interface), not `0.0.0.0` (`RE/🛰️sidecar/🟦️.ts:87`, `🗺️topology/🟦️.ts:324,370`).
5. **Enrolment and role split for workers and submitters (High).** Workers receive a per-machine token from the scheduler at enrolment; submitters use a separate read/submit token; scheduler records submitter and worker identity per action and per result.
6. **Refuse unconfined workers by default (High).** `confinement: required` in `POL`; the scheduler refuses workers reporting `confinement: none`; Windows gets no remote execution until a real sandbox exists.
7. **Linux sandbox: no host read (High).** bwrap `--tmpfs $HOME` and hide `.cache/f`; bind only declared tools (already done for macOS `readable`).
8. **Action results: no replace, record uploader (High).** `ac` PUT becomes first-writer (`keep`) for the same key, or requires the uploader's signature; store the uploader identity in `.meta`; clients reject entries from unknown uploaders.
9. **Install and store-receipt restore against repo pins (High).** Verify `fleetInstallRestoreV1` archives against the lock-derived digest, not only the manifest's own sha256 (`FETCH/📦️install/🟦️.ts:191-192`); make the `fetch-verified` marker content-based, not size+mtime (`FETCH/📦️install/🟦️.ts:187-188`).
10. **Offload: do not trust worker output blindly (High).** Requester verifies outputs by re-hashing against the scheduler's action result and refuses unknown workers.
11. **Claims: cap renewals (Medium).** Maximum lifetime per claim (for example 2 × expected cost), per-owner quota, release only by owner token.
12. **Actions: cap timeout to policy (Medium).** Reduce 4 h to the default action limit; per-submitter queue quota.
13. **Mirror: concurrency cap, no redirect following, strip query strings from keys (Medium).** Bind the mirror to the fleet interface, optionally require the fleet token for GET.
14. **Playwright: pin or disable the mirror tier (Medium).**
15. **Symlinks in input trees: reject absolute or `..` targets (Medium, `RE/👷️worker/🟦️.ts:185`).**
16. **Token hygiene (Medium).** Do not overlay the token into the environment of every child (`RC/🛰️fleet/🟦️.ts:88`); pass it only to the processes that talk to the cache. Windows: ACL on `.cache/f`.

## Not verified / limits

- No builds, tests, git commands, process operations or service restarts were run. I did not run any sandbox or cross-machine test.
- macOS sandbox escapes via firmlinks, Unix sockets and mach services: not tested.
- Full gRPC authorization ordering (authorize before reading the body): not checked.
- Go `GOSUMDB` configuration and bun lockfile integrity enforcement on the mirror path: not checked.
- `store-receipt/☁️fleet-content` restore (G): taken from the G report, not read.
- Sidecar log contents: not inspected.
- Windows: no code path execution, only the reading.
- Nothing printed of any token value. I compared the store credential with the env file in memory and printed only booleans.

## Disclosure

- One write attempt: an unauthenticated `PUT` to `http://127.0.0.1:6090/main/ac/<64 zeros>` was sent to test authorization. The server answered 401 and stored nothing. Everything else was a GET.
- Read `/etc/exports` (NFS line). Read `/etc/semio/remote-cache.env` only to compare one value with the store credential; printed only booleans.
- Read files only inside `RE`, `RC`, `SB`, `POL`, `FETCH`, `UC`, the store directory `stat` output, and the persisted H tool output.
