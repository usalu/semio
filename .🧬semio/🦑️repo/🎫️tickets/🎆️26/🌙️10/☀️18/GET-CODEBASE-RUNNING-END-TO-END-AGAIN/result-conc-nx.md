# Result conc-nx: Nx under many concurrent agents in one folder (wave 1020e)

READY (staged, NOT applied, waits for coordinator GO): `python3 C/conc-nx-tools/apply.py apply --write` (dry run by default; 14 anchored ops in 3 live files + 8 new files, 0 drift at 20:15, longest path 164 units). Post-GO runbook: `C/conc-nx-tools/live.sh`.

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket, `E` = `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx`.
Nothing outside `C/` and `/private/tmp/conc-nx/` was written; no `bun repo`, no cargo, no bazel, no real-image test; at most 2 concurrent stock-Nx processes.

## 1. What blocks concurrent Nx today (read from the code, then proved where possible)

| # | finding | evidence |
|---|---|---|
| 1 | A pinned run (`SEMIO_BUILD_GRAPH_DIGEST` set, every `bun repo run|run-many|exec`) is forced to `NX_FORCE_REUSE_CACHED_GRAPH=false` (`L/⚡️caching/🚀️bootstrap/📜️script.ts` `nxBuildEnvironmentV1`) and the seal check is skipped (`!env.SEMIO_BUILD_GRAPH_DIGEST&&…`), so every invocation rebuilds the whole Nx project graph (3.5-6 min, 158 MB) even when the digest namespace already holds the finished graph. Only the daemon-journal "selected" path (single `run proj:target`) can skip it. | script.ts lines 247, 528-545; test `🏗️build/🧪️tests/🕸️graph/📌️pinned-publication` asserts `false` |
| 2 | `nx:graph-pool:<root>` is taken exclusive and queued for every namespace acquisition (each Nx process, plugin worker and task child re-acquires), and inside it the whole roster is inventoried twice (lstat of every file of up to 64 namespaces), the shared namespace lease is acquired (blocking), the funded native image is attached, and idle namespaces are evicted. | `E/📦️graph/🟦️.ts` `acquireNamespace`, `room`, `admitPool` |
| 3 | Cache put/remote-apply takes `nx:fixed-cache:<image>` exclusive, restores shared: a put blocks every restore of every agent. The physical headroom of a put is separately serialized by the compiler writer custody (exclusive per image). | `E/🟦️.ts` `withOriginalNxFixedCacheV1`, `🏗️compiler/📏️capacity` |
| 4 | Parallelism is already bounded by the host-capacity grant: `NX_PARALLEL` comes from the plan (`nxHostResourceSettingsV1`, script.ts line 245); nothing to change. | read |
| 5 | The `bun repo` path never writes per-run workspace caches into tickets: `NX_WORKSPACE_DATA_DIRECTORY` is always bound into the funded `n` namespace, the patched Nx refuses to run without the frozen authorizers. The 10-04 strays came from raw Nx runs. | `E/🟦️.ts` `boundedNxNamespaceV1` |
| 6 | Default `NX_DAEMON` is `"true"` for finite runs (`nxChildEnvironment`): one Nx daemon per distinct digest namespace. Left unchanged on purpose (the selected-graph fast path needs the daemon journal), recommendation in section 5. | script.ts line 255 |

Coordinator message (audit-locks H3, "pool holder waits on other namespaces' readers"): the eviction in `room()` was already a try-lock (`timeoutMs:0`, a busy victim is skipped), so no eviction waited on readers. The real waits under the exclusive pool were the blocking shared namespace-lease acquisition and the full inventory/attach work. Both are fixed, and eviction itself now runs outside the pool as asked (S2).

## 2. What is staged

### S1 pinned Nx graph generation reuse (kills the rebuild storm)
- New `L/⚡️caching/🕸️graph/🔏️seal/📌️generation/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/…}`: a receipt `semio-nx-generation.json` in the digest namespace binds `project-graph.json`, `file-map.json`, `source-maps.json` (dev, inode, size, mtime, ctime, nlink 1) to the claim `{pinned graph digest, plugin implementation identity, Nx tooling identity, sha256 of the identity files nx, @nx/js, @nxlv/python manifests, nx.json, root package.json}`. Check cost: three `lstat` and five small reads, never the graph bytes.
- `📜️script.ts` (3 anchored edits): before launching Nx a pinned, non-watch, non-selected run asks `nxGenerationHoldsV1`; when it holds, `NX_FORCE_REUSE_CACHED_GRAPH=true` (Nx still re-reads workspace files live, so task hashes stay correct and Nx falls through to a rebuild when the cached graph cannot be read). After a successful (exit 0) construction the parent writes the receipt under `withNxGraphAuxiliaryCapacityV1` (same funded writer as the seal). `nxBuildEnvironmentV1` still defaults to `false`, so the existing pinned-publication test is unchanged and a run without a receipt behaves exactly as before.
- Soundness: the namespace key already contains the digest and the implementation identity; a receipt is only written when the identity files are unchanged from claim to sealing and all three graph files keep their stamps across the write; any republish (new inode) withdraws it. A new project or changed project definition changes the pinned digest, hence the namespace, hence no reuse (that is the contract with conc-graph below).

### S2 pool without hold-and-wait (`E/📦️graph/🟦️.ts`, 9 anchored edits)
- Lock order is now namespace -> pool everywhere (maintenance already sorts `nx:graph-namespace` before `nx:graph-pool`).
- Attach of an existing namespace takes NO pool: shared namespace lease (blocking, holding nothing), descriptor equals the identity, funded native shard exists and its ledger row is `committed` (births happen only in a pool pass); otherwise the lease is released and the pool pass runs.
- Birth/publication/auxiliary writes: a short pool pass that only try-locks the namespace lease (a busy namespace leaves the pass, is awaited while holding nothing, the pass is retried at most 16 times).
- Eviction: a pass that finds no capacity throws `NxGraphRoomNeededV1`, leaves the pool, and `evictIdleNamespacesV1` runs under the queued `nx:graph-evict:<root>` gate; the pool is held only for the two roster reads, victims are try-locked one at a time, retirement and deletion happen without the pool. `inventory` tolerates a namespace vanishing mid-walk (ENOENT only).
- New resource family `nx:graph-evict:` falls in the `nx:` prefix of conc-leases' `graph` class (rank 3, same as the pool); it is acquired before the pool and never while holding one.

### S3 shared cache custody (`E/🟦️.ts`, 2 anchored edits)
All users hold `nx:fixed-cache:<image>` shared. Per-output leases (restore exclusive, promotion shared), the Rust promotion proof (`scope.mode` unchanged) and the headroom serialization of a put (compiler writer custody) are untouched; a put no longer blocks restores and exclusive maintenance still excludes everyone. Stock Nx evidence below shows concurrent same-hash writers are safe.

### Tools (`C/conc-nx-tools/`)
`apply.py` (dry run default, refuses while repo run/nx/graph commands are active, all-or-nothing, atomic rename), `ops_gen.py`, `mirror.py` + `ov.sh` (overlay), `mutation-check.py`, `stock-concurrency.sh`, `verify-generation-stock.ts`, `fixture.py`, `verify-fixture-outputs.py`, `nx-stray-sweep.py` (lists/deletes stale per-run Nx workspace dirs in ticket `🗑️generated` folders, 24 h idle, dry run default), `live.sh`.

## 3. Verification (overlay `/private/tmp/conc-nx/ov`, real copy of `L`, stock Nx 23.2.0 under `/private/tmp/conc-nx/stock`)

| check | result |
|---|---|
| generation suite (`bun test`, language-agnostic fixture `🧫️fixtures/🔣️.json`, 10 mutation scenarios, claim mismatches, 3 refusals, 2 seal-time races, 8 concurrent writers; Ajv validates every receipt; identity digest equals an independent Python hashlib model) | 19 pass |
| pool-order suite: wait-for graph model of the roles (attach, birth, evict, publish, maintenance) for the old and the new design vs Python `graphlib`; old design blocks while holding the pool, new never does and is acyclic; 5 source laws | 6 pass |
| mutation proof (`mutation-check.py`, 12 mutations: digest/identity/stamps/recheck/nlink/second stamp read, pool before namespace lease, blocking victim lease, attach takes pool, attach ignores committed phase, old `room` returns, pool pass waits on lease) | 12 of 12 killed (one equivalent mutant, early identity check subsumed by the later one, removed) |
| `tsc --noEmit` on the three edited files + new files, live vs overlay diagnostics | only environment diagnostics (no `bun:test` types in the dependency image); 0 new errors in edited code |
| `bun build` of `📜️script.ts`, `E/🟦️.ts`, `E/📦️graph/🟦️.ts` | bundle ok |
| stock Nx, 2 concurrent `run-many` over the same 50 hashes into one cache, then put of 150 + restore of 50 concurrently | 0 failures, 250 outputs recomputed independently (sha256), 0 mismatches; restore run 50/50 hits |
| stock Nx graph, 2500 projects: cold build 6.0-7 s, receipt written, receipt-gated `NX_FORCE_REUSE_CACHED_GRAPH` 0.7-1.5 s (4.8-9x), same project list; nx.json edit, graph file change: receipt no longer holds; restored nx.json: holds again | as stated (`verify-generation-stock.ts`) |
| `nx-stray-sweep.py` on a synthetic tree | lists/deletes only the idle graph dir, keeps logs and a fresh dir |

## 4. Contract with conc-graph (MVCC generations, lazy closure)

The Nx side consumes only what exists today: the pin `SEMIO_BUILD_GRAPH_PATH` + `SEMIO_BUILD_GRAPH_DIGEST` (sha256 of the published portable graph bytes). Required of a generation: (1) the digest must change whenever any project definition, target input list, executor or dependency edge changes (the Nx graph is a pure projection of it); (2) it must stay constant while only file contents inside existing projects change (that is what makes reuse hit); (3) task-closure generations (`admission.selection.kind=task`) are welcome: smaller graph, smaller namespace. Nothing else is read, so any sealed-generation design works unchanged. Open point for conc-graph: with many distinct closure digests the `n` pool (1.5 GiB, <= 15 funded namespaces) churns; sharing one namespace across closure digests of the same repository is a follow-up that needs their generation ids.

## 5. Not staged (risk / needs live proof), recommendations

1. Put headroom custody (compiler writer custody, exclusive per rust image) still serializes all puts and cold cargo growth; design: a bounded semaphore of put slots admitted while `available >= required + (K-1) * maximumArtifactBytes`, falling back to the exclusive custody when tight. Needs conc-cargo's per-unit lease design first.
2. `withNxGraphPublicationCapacityV1` still holds the pool during the graph file copy (publication happens only on a non-reused build now); split into a reservation pass + per-namespace publication lease.
3. Daemon policy: finite pinned `run-many/affected/exec` could run `NX_DAEMON=false` (no per-digest daemon); the single-task selected path needs the daemon journal, so decide after measuring on a live run.
4. User-visible progress for reuse is a profile line only (`SEMIO_BUILD_PROFILE_INPUTS=1`: `Graph observation … sealed=`, `Generation receipt written=`); EN/DE catalog entry not added.
5. Not run (no GO, heavy): real-image pool tests (`🏦️funded`, `📥️store`, `🎛️operation`; command in `live.sh`), `bun repo tools setup nx` (required after apply: both edited enforcement files are inside the frozen authorizer closure), a live two-shell run. Expected live numbers: first run builds + `Generation receipt written=true`, second run `sealed=true` and about 24 s to the first task instead of 210-350 s.
6. Windows/Linux: code is `node:fs` only; no new platform branches.

## 6. Files

Created: `L/⚡️caching/🕸️graph/🔏️seal/📌️generation/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/{🟦️.ts,🐍️oracle.py,🧫️fixtures/🔣️.json}}`, `E/📦️graph/🧪️tests/🔓️pool-order/{🟦️.ts,🐍️oracle.py,🧫️fixtures/🔣️.json}` (staged under `C/conc-nx-tools/staged/new/`).
Edited (anchored, staged): `E/📦️graph/🟦️.ts`, `E/🟦️.ts`, `L/⚡️caching/🚀️bootstrap/📜️script.ts`.
Ticket-local inputs: `C/conc-nx-tools/*` (see section 2). No generated output files remain in `C/🗑️generated`.

## 7. READY2 (group 2): daemon policy and put slots

Apply: `python3 C/conc-nx-tools/apply.py apply --write --group 2-daemon` (8 ops + 5 new files, independent of group 1) and `--group 2-put` (3 ops + 5 new files; apply.py answers BLOCKED until conc-cargo's group `lease` is live because it calls their `withCompilerSharedHeadroomV1`). `--group 2` selects both. Group 1 stays `--group 1`. `bun repo tools setup nx` afterwards (service and enforcement files are in the frozen closure).

### 7.1 Daemon policy (`L/⚡️caching/🕸️graph/🏁️startup/🚏️admission/`, service + script edits)
- Cap: finite (run) daemons = min(2, a quarter of host memory / 2 GiB), watch daemons 6, both overridable with `SEMIO_NX_DAEMONS=<finite>[,<watch>]` (0 = off, max 16); a stock Nx daemon idles at ~270 MB for a 2,500-project graph, so 2 GiB per semio daemon is conservative.
- Enforced by the daemons themselves: every daemon takes the first free numbered slot lease `nx:daemon-slot:<root>:<kind>:<k>` for its whole life and refuses to start (throws) when its family is full, so two clients that admitted at the same moment cannot exceed the cap.
- Per-namespace reuse: a run first probes the exclusive `nx:daemon-service:<namespace>` lease; a live daemon of the namespace is always reused. Otherwise it is admitted while fewer than the cap are live; else this run gets `NX_DAEMON=false` (graph from the group-1 generation receipt, plain hashing) and the daemon-journal "selected" path is skipped with it. An explicit `NX_DAEMON` from the caller wins; watch invocations are untouched.
- Idle shutdown: finite daemons drain after `SEMIO_NX_DAEMON_IDLE_MS` (default 10 min) without an open request (request counter in the service root, check every 30 s, timer unref'd and cleared with the service); watch daemons never idle out. Nx's own 3 h constant stays as the outer bound.
- Journal contention: the filesystem-journal thread is per daemon, so the cap bounds the number of recursive watchers; a single shared journal remains a redesign (needs its own socket protocol) and is not attempted.
- Progress/EN-DE: the decision prints a profile line (`[nx] Daemon enabled|off (<reason>)`); no user-facing catalog entry.

### 7.2 Put slots (`E/🎟️put-slots/`, one anchored edit of `withNxPublicationHeadroomV1`)
Aligned with conc-cargo: puts (and restores with cache sources) no longer take the exclusive `repo:compiler-writer` custody; they run `withCompilerSharedHeadroomV1` (shared custody, exclusive only to recover headroom) inside one of 4 put slots (`SEMIO_NX_PUT_SLOTS`, 1..16). Admission is FIFO through the queued gate `nx:put-gate:<image>` (oldest waiter polls the slot leases, back-off 20..250 ms, cancellable, progress via the lease queue line); a slot holder waits for nothing but the shared custody. Because shared custody no longer serializes, headroom is measured as free space minus `maximumArtifactBytes` of every other slot in use (try-lock probe), so N concurrent puts cannot overcommit the image; when it is tight the existing exclusive recovery path runs and puts degrade to one at a time. Lock order: fixed-cache (shared) -> put gate -> slot -> writer custody (shared); registered with conc-leases through the `nx:` prefix (graph class).

### 7.3 Verification (overlay with conc-cargo group `lease` applied, real lease library)
- Put slots 13 pass: count table, peak concurrency equals the slot count (6 workers, 2 slots) and equals the independent Python schedule model, FIFO order equals the model, other-slot probe, release after a failing body, a cancelled waiter leaves without a slot, source law of the wiring.
- Daemon admission 25 pass: limits table (4..256 GiB hosts, overrides, garbage), decision table and idle rule equal an independent Python model; Ajv validates every decision; real leases: the cap holds (third daemon refused, released slot reusable, families counted separately), a held namespace resource is seen as a live daemon; source laws for slot-before-lifetime/release-last, idle timer, admission after bind.
- Mutation proof 23 of 23 killed across all four suites (group 1 + group 2, e.g. probe counts own slot, slot kept, lease exclusive again, headroom ignores other slots, namespace not reused, watch idles, one slot too many, memory ignored, slot not released, timer not cleared).
- `tsc`: only environment diagnostics (no bun types). Group 2 alone (without group 1) applies and passes in a fresh overlay.
- Note for conc-cargo: their group `lease` alone leaves the launcher referencing `withCompilerWorkspaceSharedWriteV1` without the import (that import sits in group `lane`); apply `lease` and `lane` together.
- Not run (no GO, heavy): a live two-session run; expected: `Waiting for access to nx:put-gate` only when more than 4 puts overlap, at most 2 finite `nx daemon` processes at any time, idle ones gone after 10 min.
