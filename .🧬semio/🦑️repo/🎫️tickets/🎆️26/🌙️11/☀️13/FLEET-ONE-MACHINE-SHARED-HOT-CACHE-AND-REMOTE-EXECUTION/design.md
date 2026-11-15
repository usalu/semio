# Design: The Fleet As One Machine

Maps of what exists: `map-remote-cache.md`, `map-remote-execution.md`, `map-build-path.md`, `map-cas-gc.md`.

## Where we start (2026-11-13)

- A cache server runs on the group server (`repo remote-cache serve`, port 6090, Bazel HTTP `/ac` `/cas` + Nx API, blobs in the host CAS). It holds 2 entries: every client gets HTTP 401 because no machine has `SEMIO_REMOTE_CACHE_TOKEN`, so every build is local.
- Rust is the dominant cost. Cargo runs through the `repo` rustc wrapper, which has a local unit cache keyed by `sha256(toolchain, normalised rustc args, source sha, extern sha, env)`. It has no remote tier.
- A REAPI v2 server, scheduler, worker and sandbox exist (`🌐️remote-execution`), used only by the Bazel backend, never provisioned, FIFO by static platform, no load signal.
- One admission decision exists per machine (`evaluateHostCapacityV1`): a job that may not start waits locally.
- Eviction everywhere is age/LRU. Nothing knows the synchronized state.
- The replica daemon runs on every machine as a supervised service and already distributes code safely.

## Decisions

1. **One fleet cache, zero-touch.** The group's cache endpoint and credential are fleet facts: the group server's replica daemon publishes them in the group store (`<store>/v1/fleet/cache.json`, the store is the trust boundary of the fleet already), every replica daemon installs them for its account, and the build system reads them from there. No per-machine token step. Unreachable cache never fails a build: it degrades to local and reports.
2. **Unit of sharing = the unit of work.** Rust: one rustc invocation (the existing unit key). Nx: one task (the Nx hash). Read-through and write-through: local miss → fleet lookup → compile → upload. Keys must be machine-independent among machines of one platform (OS, arch, toolchain build are part of the key; worktree path is not: remap paths).
3. **Nothing is computed twice: fleet single-flight.** Before computing a missing key a machine claims it at the cache server (`claim(key)` with a lease and heartbeat). A second machine that wants the same key waits for the first one's upload instead of computing; a dead claimant's lease expires and the next one takes over.
4. **Only the synchronized state is kept.** Every entry belongs to a *slot*: the identity of the unit without its content inputs (Rust: toolchain, target, profile, package id, crate name and kind, features and cfg; Nx: project, target, configuration, platform). When a newer key is stored in a slot, older keys of that slot are superseded and deleted after a short grace (machines that are a few seconds behind, builds in flight). The same rule prunes the local unit cache. Size budgets stay as the last resort.
5. **Offload under pressure.** Every machine reports its load (CPU, free memory, pressure level, running and queued units, capacity) to the group's scheduler every few seconds. When local admission says wait or refuse, the rustc wrapper hands the unit to the scheduler, which picks the least loaded compatible machine (same platform and toolchain). The worker compiles and uploads to the fleet cache; the requester takes the result from the cache like any hit. Local first while there is headroom; offload only when the fleet has an unsaturated machine; otherwise wait locally as today. Cancellation, progress, timeouts, worker death → the unit is rescheduled or runs locally.
6. **Workers need no provisioning.** The replica daemon supervises the build worker and the load reporter as children on every machine; the group server's daemon supervises the scheduler beside the cache server.
7. **Honest platform limits.** Units are shared and offloaded only between machines of the same platform key. Today that is the seven macOS arm64 machines; the single Linux and the single Windows machine share with nobody for native Rust until a second machine of their platform joins. Platform-neutral outputs (Nx tasks whose hash carries no platform) are shared by all.
8. **Visible.** `repo fleet-status` shows every machine's load, queue, cache hit/miss/offloaded counts and the claim table.

## Work packages

| # | Package | Area |
|---|---|---|
| A | Fleet facts: endpoint + credential through the store, client authorisation, Nx remote cache on, `status` fix | `🌐️remote-cache`, `🤝️shared-worktree/🔁️replica` |
| B | Rust unit cache: remote tier, portability, fleet single-flight (claims) | `⚡️caching/🦀️cargo` unit cache, `🌐️remote-cache/🖥️server` |
| C | Retention by slot supersession, server and local | `🌐️remote-cache/🧹️collector`, unit cache GC, `🧹️cache-gc` |
| D | Load reports, load-aware scheduler, worker and scheduler supervised by the replica daemon, fleet-status | `🌐️remote-execution`, `🌡️host-capacity`, `🚢️fleet-status`, replica supervision |
| E | Rust offload client in the rustc wrapper under pressure | launcher / wrapper, `🌐️remote-execution/🏃️entry` |

## Landing rules (binding)

The repository is replicated live to eight machines and other people's builds run on this code every minute.
1. One landing at a time in the build system: `mkdir "<this ticket>/landing.lock"` + `owner` file; remove it when done; a lock older than 40 minutes may be taken over.
2. Develop in a copy outside the worktree (`<session scratchpad>/<package>/`), never leave copies or symlinks in the worktree. Land by anchored edits or three-way merge onto the live files, re-reading each live file right before the edit: other agents change the same files.
3. Before releasing the lock run the test files of every member you touched and of its direct dependants, plus one real `repo` build command that goes through the changed path on this machine. Record the landing in `landings.md` (time, files, before/after sha256, tests run).
4. A broken cache, scheduler or network must never fail or hang a build: every remote step has a timeout and falls back to local.
5. Changes to `🤝️shared-worktree` additionally follow `…/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing-protocol.md` (its own lock, all its test files).
