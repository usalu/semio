# Architecture Map of the Repo Build System (as found 2026-11-09)

`BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build` (about 1000 files). `PROC` = `🧰️framework/🔨️modules/🏃️process`. `CACHING` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`. Target design: `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🔍️audits/fleet-1108-build-graph-design-review.md`.

## 0. Headline findings

1. A lot of the target design already exists as parts; nothing is wired into one fast path.
   - Derived facts with early cutoff exist (`BUILD/🕸️graph/🕰️generations/✂️cutoff`) but at the wrong granularity: a derived fact is a whole "command walk" (about 18.5k raw rows, 246 KB brotli per record).
   - A change journal and a native watcher exist, and `BUILD/🏡️resident/🕰️graph-hooks` already answers "membership" and "which rows are dirty" from the journal. They are used only when a resident service answers; otherwise the graph code falls back to stat/hash passes.
   - A resident service (`BUILD/🏡️resident`) exists, with protocol, capability tokens, fair governor, warm pool. It is per worktree and user, and it is not what holds the 2.7 GB fact index. No resident service was running at audit time.
   - `BUILD/📏️incremental` is not an incremental engine. It is a conformance and bench harness (24 edit scenarios, SLO/acceptance budgets, concurrent-agent workloads). It is the base for the bench lane.
   - Machine-wide capacity admission exists (`PROC/🏗️build/🌡️host-capacity`, state in `~/.cache/semio/host-capacity`) with learned per-class memory estimates. There is no hard per-process memory limit anywhere in production code (grep for cgroup, memory.max, RLIMIT, JobObject finds tests only).
2. Cost model today is edits x builds x repo size: each `repo run` plans in its own child process (planner 2.5-3.3 GB RSS), re-verifies thousands of coarse closures, and the facts are per workspace (`.cache/b/...`), not shared across workspaces.
3. Live evidence at audit time: host 15.8 GB RAM, 7.5-7.8 GB swap used, load 7.8, one planner at 2.5 GB RSS for 48 min, one captured child at 310 MB, three doors at 50-60 MB each, journal service 41 MB, watcher 2.7 MB.

## 1. Process model of one `repo run`

```
repo (launcher)  ->  door (BUILD/🚪️door/📜️script.ts, bun, 50-60 MB)
   |  waits for host capacity grant (PROC/🏗️build/🌡️host-capacity/🎫️gate), light commands answered here
   v
captured child "executor" (bun --eval loader running the sealed snapshot, ~300 MB; holds source images)
   |  starts directly (runAdoptedCapturedBuildSourceV1, no second door)
   v
planner (second bun --eval loader, 2.5-3.3 GB RSS): loads the fact index (455k facts, 4.6k projects),
   verifies/derives closures, publishes a generation, then execution (Nx/Bazel/semio backend) runs tasks
```
- `BUILD/🚪️door`: front door; reads the neutral operations catalogue (`⚙️operations/🔣️catalog.json`); light ops (leases, capacity, doctor) never load the build implementation; owns restart supervision (exit 75, back-off 0.25 to 30 s). README documents the chain and test law `🧭️invocation/🧪️tests/🔗️process-chain`.
- `BUILD/🏛️sealed`: tool runs from a content-addressed snapshot of its own sources (store `.🧬semio/🦑️repo/⚡️cache/implementation`, 566 MB, ledger budget 512 MiB); promotions single-flight under an SQLite lock. Reuse as is. Its digest keys the graph memo.
- `BUILD/📥️source/🔏️closure` (95 files): bounded original module images for the captured child.
- Capacity: reservation is estimated by `phase:*` classes; the infra audit (`fleet-1108-infra.md`) found phases never learned (fixed by `gate` continuation rule), so reservations were about half of real use.
- Who loads the fact index: only the planner child, once per `repo run` that has to verify/plan. Nothing shares it across processes except the on-disk files (page cache) and the durable answer memo.

## 2. Subsystem table

Sizes are file counts and disk size of the source directory, then runtime data.

| Subsystem | Purpose / entry | Data held (runtime) | Process | Maps to target |
|---|---|---|---|---|
| `🕸️graph` (490 files, 6.2 MB) `🟦️.ts` (131 KB) | Build the neutral project graph (projects, targets, deps) from 4.6k projects; sealed generations. Entry `BUILD/🕸️graph/🟦️.ts` | `.cache/b/g/m`: 3.7 GB (generations 1.7 GB, inference 1.1 GB, snapshots 479 MB, build-input-closures 433 MB, build-lock-closures 18 MB); fixed-size image cap 12 GB | planner child | Replace verification path; keep generation format as the published snapshot |
| `🕸️graph/🕰️generations` | Sealed, immutable, content-addressed fact index: manifest `m`, fragments `o` (about 12k `a/*.json.br`), index `x`, frontier `f`; load `🔌️load`, merge, claims, custody, pin `🧷️pin`, yield `🛌️yield` | Policy: retain 4 generations, maximumIndexBytes 512 MiB packed (about 2.7 GB expanded), verifyChunkRows 4096, verifyParallelChunks 4, readConcurrency 32 | planner | Reuse the store; replace "verify all rows" by journal-driven dirty sets; keep pin for revisions |
| `…/✂️cutoff` (+ `💾️durable`) | Derived facts: question -> answer digest; pseudo owner `@derived:<sha>` holds row + support rows; equal answer keeps consumers valid. `settleSuspectsV1` | Durable memo `<store>/a/<sha256(question)>.json.br`, about 250 KB each, 2.85k records (about 700 MB); deriveConcurrency 2, maximumSuspects 65536; retire after 7 days | planner; helper script runs shards | Core to extend: derive manifest/project/named-input facts as small nodes (hundreds of rows, not 18.5k) |
| `🕸️graph/📥️inference` (23 sub dirs) | Per-project inference: project-nodes, dependency-edges, native-manifests (Cargo), bun-lock, script-inputs, source-analyses, owner-discovery, target-policy, revision, stamps | Inference store `.cache/b/g/m/inference` 1.1 GB; `maximumParsedManifests` 256 | planner worker threads | Source of the manifest/project facts to turn into memo nodes |
| `🕸️graph/🚪️admission`, `📋️model`, `⏱️phases` | Selection admission (`BuildGraphSelectionV1`: all / owners / tasks), neutral model types, phase clock (assembly/plan/rebuild/execution) | small | planner | Demand-driven scope hooks exist (`kind:"owners"|"tasks"`); selection is not yet closure-scoped for Nx |
| `🧠️memory` | Graph memory governor: budget 1-16 GB (hostShare 0.25), childHeapMegabytes 2048, interned string cap 64 MiB, maximumCachedClosureEntries 4,194,304, maximumParsedManifests 256, retainedImageBytes 32 MiB | in-process | planner | Keep policy; becomes limits for the daemon heap |
| `🌡️host-capacity` (BUILD side 38 files) + `PROC/🏗️build/🌡️host-capacity/🎫️gate` | Machine-wide admission by memory/swap/PSI/load/files; learned estimates per class (256 classes in `~/.cache/semio/host-capacity/estimates.json`); grants in `grants/`, queue in `queue/` | policy in `🔣️policy.json` (floor 12.5% RAM, 1.5-4 GB; maxRamping 2; swap ceiling 25%; maxWaiters 256) | door holds the grant | Reuse for admission; add hard limits and global priority queue on top |
| `🔒️leases` (6 files) | `repo leases status` plus the repository lease directory `.cache/b/l/m/data`; real leases in `PROC/🔒️leases`, registry `~/.cache/semio/lease-registry` (4 MB) | file leases | any | Reuse as mutual exclusion primitive |
| `🚪️door` | See section 1 | - | door | Becomes thin client |
| `🏡️resident` (116 files, 1.4 MB) | Resident repo service: `🏛️service` (unix/named-pipe socket, capability tokens `📨️protocol/🔑️capability`, governor), `🗃️state` (event-sourced projections of watcher batches: owner digests, task keys, memo, frontier), `🏊️delegate` (warm pool/executor pool that run `repo run <id> --service=off`), `🚪️client/🔁️repo` (eligibility: plain `run/test/run-many` only; `--service=auto|off|require`), `🏗️composition` (`createForegroundRunnerV1`), `📜️journal` (detached change-journal service), `🔭️watch` (native watcher supervisor) | Policy: maximumResidentBytes 512 MiB, memoMaximumEntries 65536, digestCacheMaximumFiles 2,000,000, journalMaximumEvents 262144, 8 in flight / 64 queued / 64 clients, idle shutdown 15 min, frontier entries 64 | per worktree + user; not running at audit time | Already a daemon skeleton and client protocol. Must change: per host not per worktree, hold the fact index, serve planning instead of delegating whole commands to children |
| `🏡️resident/🕰️graph-hooks` | `membershipFromJournalV1`, `dirtyPathsFromJournalV1`, `createResidentHoldsV1`; host seam `🔌️host` (`residentGraphSeamsV1`) wraps `membership` and `holds` of the graph host; falls back to stat when no service | pure functions | planner calls the service | This is the journal-driven invalidation primitive; make it the only path |
| `🏡️resident/🔭️watch` + `🦀️rust/🦀️.rs` (58 KB) | `semio-repo-watch`: std-only Rust, FSEvents/inotify/ReadDirectoryChangesW, NDJSON frames on stdout (`WatcherFrame`), EOF on stdin ends it, 64 MiB queued-frame cap with overflow-as-loss. Binary provisioned by `🏗️provision` into `.cache/b/t/m/repo-watch/<hash>/semio-repo-watch` (4 versions kept; 2.7 MB RSS) | event stream | child of journal service | Reuse unchanged protocol; add per-root multiplexing for the host daemon |
| `🏡️resident/📜️journal` | Change-journal service: one epoch, one writer, one watcher supervisor, heartbeat, idle shutdown, lock; keeps no membership/task state; log at `/run/user/<uid>/semio-repo-<uid>/<wt>/journal` (tmpfs), plan memo `graph-memo` 870 MB (`*.plan`, 227 MB each) in `~/.cache/semio/repo/<wt>/journal` | log + plan memo | detached bun (41 MB RSS), started by the captured child | Reuse; the daemon should own it; bound/GC `graph-memo` |
| `CACHING/🕸️graph/🔏️seal/📥️membership/📥️journal` (+`🌐️rpc`, `👷️worker`) | Journal client/rpc used by the sealed-graph membership proof | - | planner | Dedupe with resident journal (two clients of the same journal) |
| Watcher exclusions | `RESIDENT_EXCLUDES_V1` in `🏡️resident/⌨️command/🟨️.mjs`: `.git .cache .r .🧬semio/🦑️repo/⚡️cache .🧬semio/🦑️repo/🎫️tickets bazel-*`, names `node_modules target __pycache__ .venv` | - | - | Edits inside tickets never reach the journal (correct for source graph; note for fleet) |
| `📏️incremental` (37 files) | Conformance and bench harness for incremental behaviour: `📋️scenarios/🔣️.json` (24 scenarios: framework TS comment/body/signature/config/unrelated, Rust crate, wasm, vite, tests, lockfile, activation, typegen/generate keys), `🏃️runner`, `🏎️bench` (SLO + `🛂️acceptance/🎯️budgets`), `🤝️concurrent` (simulated agents: edit/revert/run/barrier), `🐍️oracle` | scenario rows | tool-neutral | Extend, do not replace: becomes the bench lane's base; add graph-service metrics |
| `🏛️sealed` | See section 1 | 566 MB | - | Keep |
| `📥️inputs` (217 files, 2.6 MB) | Named/portable input globs, stable-file reads, digest, revision, lean-inventory, owner-view, lock, storage | - | planner, executor | Source of named-input facts; add per-input memo nodes |
| `📥️source` | Source selection, closure, zero-touch | - | captured child | Staging into a revision CAS |
| `🧹️cache-gc` (21 files) | Size ledger `.🧬semio/🦑️repo/⚡️cache/🧭️size-ledger.json`, sweep (budget by area), dedupe (CoW clone census), reclaim, runner with SQLite lease | Per workspace: `.🧬semio/🦑️repo/⚡️cache` 27 GB (rust 25 GB, go 1 GB, tools 681 MB, implementation 566 MB) and `.cache/b` 49 GB (r 22 GB, d 16 GB, t 6.4 GB, g 3.7 GB, n 830 MB, o 514 MB) | background sweep | Reuse sweep machinery; add host-wide CAS, LRU by last access, cross-workspace dedup |
| Storage budgets | `CACHING/💾️storage/🔒️enforcement/📦️budget` (`🔣️policy.json`, `📐️elastic`, `📏️grow`): each `.cache/b/<x>` is a fixed-size image (`image.json` maximumPhysicalBytes g 12 GB, d 36 GB, r 96 GB, t 8 GB, n 1.5 GB, o 2 GB) | caps exist but per workspace | - | Caps sum to 156 GB per workspace; need host-wide cap across workspaces |
| `👷️workers`, `🧵️workers` | `👷️workers`: tool-neutral persistent worker policy (kinds test/check/generate/build, maxInstances 8/8/4/2, 512 MB per worker, localJobs max 8, reset after 256 requests); `🧵️workers`: bounded worker-thread pool for pure module tasks | threads/processes | executor | Reuse for the shared compiler daemon lane |
| `🔌️backends` | Backend registry (`BuildBackendV1`): provided by tool / adapter / neutral / none; conformance behaviours | - | - | The tool-neutral interface that keeps Nx/Bazel as executors |
| `🔧️tools/{🧶️nx,🏗️bazel,🐙️semio,💾️storage}` | Executors. Nx needs a whole-workspace graph (root cause of whole-workspace planning); Bazel has plan-reuse, snapshot, workers, remote cache | - | planner, executor | Keep behind `🔌️backends`; feed them target-scoped plans |
| `🌐️remote-cache` (15 files) | Policy-shaped remote cache contract, REAPI-style `🔬️action-result`, local `💾️store`, `🖥️server`, `🧹️collector` | - | server optional | Candidate CAS server implementation for host-local CAS |
| `🌐️remote-execution` (25 files) | REAPI scheduler `🗓️scheduler` (operation stages), `🗄️content`, `🔒️sandbox`, `📡️wire`, `👷️worker` | - | server optional | Reuse scheduler concepts for the fair local scheduler |
| `🧭️selection` (22 files) | Portable project selectors, options, `📥️closure` (prerequisite walk), `👁️watch` | - | planner | Home for target-scoped evaluation |
| `🪪️agent-sessions` | Attaches each command to an agent session (`SEMIO_*` env), leases/capacity attribution | `~/.cache/semio/agent-sessions` | door | Provides the agent identity for fair scheduling |

## 3. Caches and bounds (what is bounded, what is not)

| Cache | Location | Bound today |
|---|---|---|
| Fact index in planner | heap | memory governor budget (1-16 GB, share 25% of host); real 2.5-3.3 GB per planner; not shared between processes |
| Closure rows | heap | maximumCachedClosureEntries 4.19M; interned row table, 2 recent answers |
| Durable answer memo | `.cache/b/g/m/build-input-closures` 433 MB / `<store>/a/*.json.br` | age only (7 days); no size cap |
| Journal plan memo | `~/.cache/semio/repo/<wt>/journal/graph-memo` 870 MB | none found |
| Generations | `.cache/b/g/m/generations` 1.7 GB | retain 4 + grace 10 min |
| Per workspace storage images | `.cache/b` 49 GB, `.🧬semio/🦑️repo/⚡️cache` 27 GB | fixed-size images per area; no host-wide cap, no cross-workspace dedup |
| Resident state memo | service heap | memoMaximumEntries 65536, digestCacheMaximumFiles 2M, outputSignatureMaximumEntries 2048, maximumResidentBytes 512 MiB (service restarts at limit: exit 75 reason "memory") |
| Sealed implementation | 566 MB | 512 MiB ledger, keepPrevious |
| Capacity estimates | `~/.cache/semio/host-capacity/estimates.json` | 256 classes |

## 4. How it maps to the target design

| Target element | Exists | Gap / replace |
|---|---|---|
| Fine-grained derived DAG, early cutoff | Cutoff and derived-fact machinery (coarse) | New small nodes: per-manifest dependency set, per-project node, per-named-input digest; closure nodes depend on them instead of raw rows |
| Watcher-journal invalidation | Native watcher, journal service, graph-hooks, seam in `🕸️graph/🟦️.ts` | Remove stat-pass fallback as the normal path; dirty set must drive memo invalidation, not only membership; one journal client |
| One daemon per host holding the index once | Resident service skeleton, protocol, capabilities, governor | Per-host endpoint (today per worktree), host the planner/index inside the daemon, `repo run` as a client that sends a `PlanRequest` and receives a pinned plan, instead of delegating the whole command to a warm child |
| MVCC revision pinning, CAS-staged inputs | Generation pin (`🧷️pin`), sealed implementation precedent, `📥️source` images, `📥️inputs/🔏️revision` | Revision objects and a CAS where inputs are staged at the pinned revision; removal of "source changed -> re-plan" (`🕸️graph/🟦️.ts` line 79 `staleImplementationV1`, `owner-discovery` torn-read retry) |
| Demand-driven planning | Admission `kind:"owners"/"tasks"`, `🧭️selection/📥️closure`, Bazel plan-reuse | Nx still plans whole workspace; add target-scoped evaluation and a scoped graph view |
| Hard per-job memory limits | Admission estimates and learned floors only | New limiter port: cgroup v2 (Linux), Job Objects (Windows), watchdog + RLIMIT_AS/footprint polling (macOS) |
| Size-capped CAS + LRU GC | Ledger, sweep, dedupe, per-area images | Host-wide CAS root with access ledger and LRU; workspaces hold references |
| Fair global scheduler | Resident governor (round-robin lanes), host-capacity queue (aging, bypass), REAPI scheduler | One host queue with priorities (dependency depth) and per-agent fairness, fed by learned estimates; retire `build-slot.sh` |
| Bounded caches | Several policy caps | Audit remaining unbounded maps (`graph-memo`, durable memo, helper scripts); common `BoundedCacheV1` |
| Bench/metrics | `📏️incremental` + SLO/acceptance + concurrent workloads | Add planner RSS/CPU/IO sampler, host-level metrics, JSONL history |

## 5. Risks observed in the code

- `BUILD/🕸️graph/🟦️.ts` is 131 KB in one file; the daemon lane must extract a service-neutral API before moving it.
- Two journal clients (`CACHING/…/📥️journal` and `BUILD/🏡️resident/📜️journal`) must be unified.
- Path lengths: emoji directories are long; new directories must stay shallow (AGENTS.md 256 char limit).
- Sealed implementation promotion changes the digest that keys memos; the daemon must restart or hot-swap on promotion (resident already has `implementation` digest staleness, exit 75).
- Windows has no unix sockets in the warm pool (`warmSize` 0 on win32); the daemon protocol must use named pipes (endpoints already abstracted in `📨️protocol/📍️endpoint`).
