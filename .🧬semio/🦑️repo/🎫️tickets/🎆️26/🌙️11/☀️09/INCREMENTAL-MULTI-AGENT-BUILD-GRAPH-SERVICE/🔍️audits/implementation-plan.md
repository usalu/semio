# Implementation Plan: Incremental Multi-Agent Build Graph Service

`BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `PROC` = `🧰️framework/🔨️modules/🏃️process`, `CACHING` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`. Facts about the current system: `architecture-map.md`. Every schema is JSON Schema (draft-07) in the lane's `🧬️schema/🔣️.json`, TS and Rust types are derived or checked against it. Every lane adds language-agnostic tests (JSON fixtures consumed by TS plus an independent Python or Rust oracle) and EN+DE texts. No legacy, no compat layers: each lane deletes what it replaces.

## Lane overview

| # | Lane | Directory ownership (exclusive) | Depends on (contracts) |
|---|---|---|---|
| 1 | `bench` | `BUILD/📏️incremental/**` | none (starts first) |
| 2 | `derived-dag` | `BUILD/🕸️graph/🕰️generations/✂️cutoff/**`, new `BUILD/🕸️graph/🧮️derived-nodes/**`, `BUILD/🕸️graph/📥️inference/**` | `NodeKeyV1` (own) |
| 3 | `journal` | `BUILD/🏡️resident/🕰️graph-hooks/**`, `BUILD/🏡️resident/📜️journal/**`, `BUILD/🏡️resident/🔭️watch/**`, `CACHING/🕸️graph/🔏️seal/📥️membership/**` | none |
| 4 | `daemon` | `BUILD/🏡️resident/{🏛️service,🚪️client,🏗️composition,🔌️ports,📨️protocol,🏊️delegate,🏭️executor,🗃️state,⌨️command,🧬️schema}/**`, `BUILD/🚪️door/**`, `BUILD/🧭️invocation/**` | 2, 3, 5, 6 contracts |
| 5 | `revision-cas` | new `BUILD/🧊️revision/**`, `BUILD/📥️source/**`, `BUILD/📥️inputs/**`, `BUILD/🕸️graph/🕰️generations/🧷️pin/**` | `CasPolicyV1` (lane 8) |
| 6 | `scoped-plan` | `BUILD/🧭️selection/**`, `BUILD/🕸️graph/🚪️admission/**`, `BUILD/🕸️graph/📋️model/**`, `BUILD/🔌️backends/**`, `BUILD/🔧️tools/**` | 2 (`DerivedNodeV1`) |
| 7 | `job-limits` | `BUILD/🌡️host-capacity/**`, `BUILD/🧠️memory/**`, `PROC/🏗️build/🌡️host-capacity/**`, new `BUILD/🧱️job-limits/**` | none |
| 8 | `cas-gc` | `BUILD/🧹️cache-gc/**`, `CACHING/💾️storage/**`, `BUILD/🌐️remote-cache/**`, new `BUILD/🗄️cas/**` | none |
| 9 | `scheduler` | `BUILD/🔒️leases/**`, `BUILD/👷️workers/**`, `BUILD/🧵️workers/**`, `BUILD/🌐️remote-execution/**`, new `BUILD/🗓️scheduler/**`, `BUILD/🪪️agent-sessions/**` | 7 (`JobEstimateV1`), 4 (transport) |
| 10 | `bounded-index` | `BUILD/🕸️graph/🕰️generations/{🧮️fact-codec,🔌️load,🔀️merge,🌊️json-stream,🧱️chunk-classes,🌿️frontier-walk,🧭️frontier,🔭️coverage,🧵️edge-reads}/**`, new `BUILD/🧊️bounded/**`, `BUILD/🕸️graph/⏱️phases/**` | none |
| 11 | `integration` | `BUILD/🕸️graph/🟦️.ts`, `BUILD/📜️script.ts`, `BUILD/🏃️execution/**`, `BUILD/🧪️tests/**`, ticket scripts retirement (`NORM-…/🧰️scripts/build-slot.sh`, `closure-verify-helper.mjs`) | all |

Unlisted BUILD directories stay with the lane that needs them via `📌️requests.md`. Keep new directory names shallow (256 character path limit).

---

## Lane 1: `bench` (metrics harness)

- Goal: measure every success metric before and after each lane; make regressions fail.
- Owns: `BUILD/📏️incremental/**` (existing: 24 scenarios, `🏎️bench` SLOs, `🛂️acceptance/🎯️budgets`, `🤝️concurrent` workloads).
- Provides: `MetricSampleV1 {atMs, process:{pid, role:"door"|"executor"|"planner"|"daemon"|"watcher"|"job", rssMb, cpuMs, ioReadMb, ioWriteMb}, host:{availableMb, swapUsedMb, load, psi}}`; `BenchResultV1 {scenario, lane, wallMs, peakRssMb, oomKills, planMs, rowsChecked, cutoffHit, storeBytes}` appended to `📊️metrics` as JSONL; `SloV1` gates (extends `🎯️slo/🔣️.json`).
- Consumes: nothing.
- Steps:
  1. Sampler port (`/proc` on Linux, `ps`/`proc_pid_rusage` on macOS, `NtQuerySystemInformation`/WMI on Windows) behind one interface; sampling with bounded buffer.
  2. Scenarios: add `one-file-edit-replan` (edit, then `no-op re-plan`), `cold-plan-one-artifact`, `n-agents-edit-storm` (reuse `🤝️concurrent`: 1, 4, 8, 16 simulated agents), `long-session` (2 h soak, host RSS and store growth curves).
  3. Baseline run on the current system; store as fixture-pinned baseline; publish table in `🔍️audits/bench-baseline.md`.
  4. Gate: `repo bench compare` fails on regression beyond tolerance.
- Tests: `.feature` and JSON fixtures: sampler against a fake process table; oracle in Python recomputes percentiles and SLO verdicts.
- Acceptance: baseline recorded for all rows below; final target set: no-op re-plan after a one-file edit <= 5 s (p95, warm daemon); cold plan of one artifact project bounded by its closure (rows read <= 5% of 455k); sum of planner/daemon RSS <= 3 GB at 16 agents; 0 OOM kills in the 2 h soak; store <= cap.

## Lane 2: `derived-dag` (fine-grained derived facts with early cutoff)

- Goal: one manifest/project/named-input edit invalidates only its dependents; whitespace or unrelated edits stop at the first unchanged node.
- Owns: `BUILD/🕸️graph/🕰️generations/✂️cutoff/**`, `BUILD/🕸️graph/🧮️derived-nodes/**` (new), `BUILD/🕸️graph/📥️inference/**`.
- Provides (schema `BUILD/🕸️graph/🧮️derived-nodes/🧬️schema/🔣️.json`):
  - `NodeKeyV1 {kind:"manifest"|"project"|"named-input"|"target-inputs"|"action-key"|"closure", subject:string, params:Digest}`;
  - `DerivedNodeV1 {key:NodeKeyV1, valueDigest:Digest, deps:NodeKeyV1[], rawReads:RowRefV1[], computedAtRev:RevisionId}`;
  - `MemoStoreV1` port: `get(key)`, `put(node)`, `invalidate(dirty:DirtySetV1)` returning changed keys after early cutoff; storage record format content-addressed like today's `<store>/a/<sha>.json.br`, but deps are node edges, not 18.5k raw rows.
- Consumes: `DirtySetV1` (lane 3), `RevisionId` (lane 5), `BoundedCacheV1` (lane 10).
- Steps:
  1. Specify node kinds and dependency edges; generate TS types from the schema; write the Python oracle (naive recompute-everything evaluator) first.
  2. Implement manifest nodes (`Cargo.toml`, `package.json`, `📋️project.json`, `go.mod`: parsed dependency set only), project nodes (targets, owners, inputs), named-input nodes (digest of resolved glob set per `namedInputs`).
  3. Rebuild the "command walk" closures as compositions over those nodes (typescript program, command walk): closure value digest = hash of node digests, so an unrelated manifest change cannot touch it.
  4. Implement push-invalidate + pull-recompute with early cutoff (Salsa/Skyframe style, red-green marking), single-flight per key, `deriveConcurrency` configurable.
  5. Delete the all-rows verification (`settleSuspectsV1` row sweeps, `closure-verify-helper`), replace the durable memo by the node store.
  6. Migrate fixtures all at once (no migration scripts).
- Tests: scenario table JSON (edit kind -> expected recomputed key set, expected cutoff keys), property test against the Python naive oracle (random edit sequences, equal published graph), reuse the 24 `📏️incremental` scenarios (`fw-ts-config`, `*-unrelated` must recompute 0 closures).
- Acceptance: whitespace edit of `📋️project.json` recomputes exactly 1 manifest node and 0 closure nodes; one dependency change recomputes only the dependents (counted by `cutoffHit`); rows read per no-op plan <= 1% of today's (5.84M -> <= 60k).

## Lane 3: `journal` (watcher-journal-driven invalidation)

- Goal: only paths the watcher reported are re-stat/re-hashed; no full-scan verification.
- Owns: `BUILD/🏡️resident/🕰️graph-hooks/**`, `BUILD/🏡️resident/📜️journal/**`, `BUILD/🏡️resident/🔭️watch/**` (TS supervisor and `🦀️rust/🦀️.rs` = `semio-repo-watch`), `CACHING/🕸️graph/🔏️seal/📥️membership/**`.
- Provides: `DirtySetV1 {cursor:JournalCursorV1, complete:boolean, exact:string[], subtrees:string[], overflow:boolean}`; `JournalCursorV1 {instance:string, seq:number}`; `JournalQueryV1 {sinceCursor}` -> `JournalReplyV1 {changes:ChangeRow[], complete}`; trust states `exact|verify|none` (existing `WatcherState`). Multi-root: `JournalOpenV1 {root, excludes}` so one watcher process serves all workspaces on the host.
- Consumes: none.
- Steps:
  1. Unify the two journal clients (`CACHING/…/📥️journal` and resident) into one library.
  2. Make journal trust a hard precondition: when `trust=exact`, return `DirtySetV1` and never stat untouched rows; when lost/overflow/cold, produce one bounded "verify epoch" scan (progress + cancellation) and then resume trust.
  3. Persist cursor in the memo store so a restarted daemon resumes from the last cursor (journal log lives on tmpfs `/run/user`; on loss fall back to the verify epoch).
  4. Watcher: cross-platform parity tests (FSEvents, inotify, RDCW), overflow-as-loss, fence protocol, add per-root multiplexing and exclusion-set versioning.
  5. Bound `graph-memo` plan files (870 MB at audit time) by size and age.
- Tests: journal fuzz and burst tests exist (`📜️journal/🧪️tests/{🎲️fuzz,🌊️burst}`); add `dirty-set` fixtures: scripted event streams (create/modify/rename/delete/dir rename/overflow) -> expected `DirtySetV1`; oracle = a `find`-based diff (independent third-party-free reference: Python `os.walk` stat diff).
- Acceptance: with 455k facts and 1 edited file, rows checked = 1 file plus its directory entries (<= 10); no stat on untouched rows (counted); watcher RSS < 20 MB per host; event loss test recovers with exactly one bounded scan.

## Lane 4: `daemon` (per-host graph service, `repo run` as client)

- Goal: one process per host holds the fact index and memo once; `repo run` asks it for a plan instead of planning.
- Owns: `BUILD/🏡️resident/{🏛️service,🚪️client,🏗️composition,🔌️ports,📨️protocol,🏊️delegate,🏭️executor,🗃️state,⌨️command,🧬️schema}/**`, `BUILD/🚪️door/**`, `BUILD/🧭️invocation/**`.
- Provides (extend `BUILD/🏡️resident/🧬️schema/🔣️.json`): `PlanRequestV1 {workspace:WorkspaceId, targets:TargetRefV1[], agent:AgentId, priority:number, revision?:RevisionId, language}` ; `PlanReplyV1 {revision:RevisionId, plan:ScopedPlanV1 | {ref:CasRef}, progress[], cacheHits}`; `ServiceStatusV1 {pid, rssMb, indexBytes, workspaces[], clients, queue}`; frames keep the existing capability tokens. Endpoint: one per user and host (`$XDG_RUNTIME_DIR/semio-repo/<uid>/service.sock`, named pipe on Windows) keyed by sealed implementation digest.
- Consumes: `MemoStoreV1` (2), `DirtySetV1` (3), `ScopedPlanV1` (6), `RevisionId` + pin (5), `JobRequestV1` (9), `BoundedCacheV1` (10).
- Steps:
  1. Endpoint scheme per host (today `endpointsV1(canonicalRootV1(root))` per worktree): `WorkspaceId` multiplexing in one process; workspaces open lazily, closed after idle (indexes evicted LRU, bound = host budget).
  2. Move graph loading (`loadGraph`, index, memo) into the daemon; clients receive plans or CAS refs (never the index).
  3. `repo run` client: `serviceEligibleV1` becomes default (`--service=auto`), the door asks the daemon first, daemon auto-starts, door waits with progress and cancellation; fallback to one-shot only for `--service=off`.
  4. Hot-swap on sealed promotion: restart with state handover or reload; restart when `maximumResidentBytes` reached (exit 75 path exists).
  5. Cancellation propagation (client disconnect cancels queued/running derivations that no one waits for).
  6. Remove the delegated "run whole command in warm child" path for plan work (execution children stay).
- Tests: protocol fixtures (`ClientFrame/ServiceFrame` JSON, existing validator), process-level test: 16 concurrent clients, kill -9 daemon mid-plan, restart, assert single index in memory (`ServiceStatusV1.indexBytes` once), RSS bound; oracle: plan equals the one-shot planner's plan (Python diff of canonical JSON).
- Acceptance: daemon RSS <= 2.5 GB with the full 455k-fact index and 16 clients attached; second client's plan latency for an unchanged tree <= 1 s; planner children per `repo run` = 0.

## Lane 5: `revision-cas` (MVCC revisions, CAS-staged inputs)

- Goal: a build pins revision R; edits made after R never affect it, no "source changed" re-plan loop.
- Owns: `BUILD/🧊️revision/**` (new), `BUILD/📥️source/**`, `BUILD/📥️inputs/**`, `BUILD/🕸️graph/🕰️generations/🧷️pin/**`.
- Provides: `RevisionV1 {id:Digest, workspace, journalCursor, parent?, roots:TreeRef[]}`; `InputManifestV1 {revision, entries:[{path, mode, digest:Digest, size}]}` (Merkle tree of the declared inputs of a target); `StageRequestV1 {revision, paths}` -> `CasRef[]`; `PinV1 {revision, holder:AgentId, expiresMs}` with refcounted release (extends today's pin).
- Consumes: `CasStoreV1` (lane 8) for objects, `DirtySetV1` (3).
- Steps:
  1. Define revision = journal cursor + memoized tree digests (O(changed) per new revision, copy-on-write tree).
  2. Stage input files of a scoped plan into the CAS lazily at pin time: hardlink or reflink where the filesystem supports it, copy otherwise (cross-platform); executors read inputs from a materialized sandbox view (`🏃️execution/🔒️hermetic` exists for this).
  3. Torn-read handling: stage by (read, hash, re-stat) loop bounded; if a file keeps changing, the revision records the content actually read; build is valid for that content.
  4. Action keys derive from input digests of the revision, so identical inputs in any workspace share results.
  5. Remove `staleImplementationV1("Loaded build graph source changed…")` restarts and owner-discovery re-import retries for tree edits (keep for tool implementation, which is sealed).
- Tests: concurrency fixtures: edit file during build at each phase, build result must equal result of the frozen snapshot (oracle: Python builds a tarball snapshot at R and recomputes digests); property: two workspaces with equal trees produce equal action keys.
- Acceptance: under the `n-agents-edit-storm` bench with 8 agents editing, 0 "source changed" restarts and 0 re-plans caused by edits outside the target closure; staging overhead <= 5% of build wall time on cold, ~0 on warm (CAS hit).

## Lane 6: `scoped-plan` (demand-driven, target-scoped planning)

- Goal: planning cost bounded by the requested target's transitive closure and its tool closure.
- Owns: `BUILD/🧭️selection/**`, `BUILD/🕸️graph/🚪️admission/**`, `BUILD/🕸️graph/📋️model/**`, `BUILD/🔌️backends/**`, `BUILD/🔧️tools/**` (Nx, Bazel, semio adapters).
- Provides: `TargetScopeV1 {targets:TargetRefV1[], includeToolClosure:boolean}`; `ScopedPlanV1 {scope, projects:string[], tasks:TaskV1[], edges, inputsRef:InputManifestV1Ref, actionKeys}`; backend capability flag `plansScoped:boolean` in `🔌️backends/📋️registry`.
- Consumes: `NodeKeyV1/DerivedNodeV1` (2) for lazily evaluated project nodes.
- Steps:
  1. Compute the target closure from project nodes on demand (follow `dependsOn` edges lazily; never materialize unrelated projects) using `BuildGraphSelectionV1.kind:"tasks"` already in admission.
  2. Nx adapter: feed Nx a project-graph subset (closure only) through its programmatic API in an executor child, keeping Nx behind the tool-neutral interface; Bazel adapter already supports plan reuse, add target-scoped snapshot.
  3. Conformance: plan for scope S equals the restriction of the whole-workspace plan to S (`📐️conformance` suites).
  4. Remove whole-workspace planning from the default path; keep `all` only for explicit `graph` queries.
- Tests: JSON fixtures of mini workspaces (language-agnostic) with expected closure and task sets, oracle: Python graph reachability; cross-check against Nx's own `nx show project`/`graph` output as the third-party validation; cases: cycles, optional targets, `^dep` expansions.
- Acceptance: cold plan of one artifact project reads <= projects in its closure (counted rows); peak RSS of a scoped cold plan <= 600 MB.

## Lane 7: `job-limits` (hard per-job memory limits)

- Goal: an overrun fails one job, never the host.
- Owns: `BUILD/🌡️host-capacity/**`, `BUILD/🧠️memory/**`, `PROC/🏗️build/🌡️host-capacity/**`, `BUILD/🧱️job-limits/**` (new).
- Provides: `JobLimitPolicyV1 {memoryMb, swapMb:0, cpuShares, pids, softRatio:0.9, enforcement:"hard"|"watchdog"}`; port `JobLimiterV1 {create(policy)->LimitHandle; attach(pid); onBreach(cb); usage()->{rssMb, peakMb}; release()}`; implementations: `cgroup-v2` (Linux: delegated `user@.slice` cgroup, `memory.max`, `memory.swap.max=0`, `memory.high`, `pids.max`, `memory.events` for oom_kill), `job-object` (Windows: `JOBOBJECT_EXTENDED_LIMIT_INFORMATION.JobMemoryLimit`), `watchdog` (macOS and Linux without cgroup delegation: poll `proc_pid_rusage` / `/proc/<pid>/status` at 100 ms, `RLIMIT_AS` where available, kill process group on breach with a typed `JobMemoryExceededV1` result). `JobEstimateV1 {class, floorMb, learnedPeakMb, limitMb}` uses learned `phase:*` classes: limit = max(1.25 x learned peak, floor).
- Consumes: none (daemon and scheduler call the port).
- Steps:
  1. Schema + fake limiter used by all tests; typed failure `memory-exceeded` with EN/DE text and the learned-estimate update (peak recorded even when killed).
  2. Linux cgroup v2 implementation with capability detection (`/sys/fs/cgroup/cgroup.controllers`, delegation check), automatic fallback to watchdog.
  3. Windows Job Object and macOS watchdog via the std-only Rust helper pattern already used by `semio-repo-watch` (no external runtime library).
  4. Wrap planner, daemon (`memory.high` + restart on `memory.events`), executors, rustc/tsc jobs.
  5. Fix admission: counts of agent processes and sessions in the host budget; keep the continuation-phase fix of the gate.
- Tests: fixtures `{policy, allocation script} -> {exitKind, peakMb<=limit}` with a language-agnostic allocator child (Python and Rust); run on each platform in CI; assert host `MemAvailable` never drops below floor in the test.
- Acceptance: a 4 GB allocator under a 1 GB limit is killed within 500 ms, siblings unaffected; zero OOM kills of unrelated processes in the 2 h soak.

## Lane 8: `cas-gc` (size-capped CAS, LRU GC, cross-workspace dedup)

- Goal: one host-wide content-addressed store with a cap (default 120 GB of 200 GB) and LRU GC.
- Owns: `BUILD/🧹️cache-gc/**`, `CACHING/💾️storage/**`, `BUILD/🌐️remote-cache/**`, `BUILD/🗄️cas/**` (new).
- Provides: `CasPolicyV1 {rootDir, capBytes, highWatermark:0.9, lowWatermark:0.75, minAgeMs, areas[]}`; `CasStoreV1 {put(stream)->Digest, get(digest), has(digests), link(digest,dest), touch(digest)}`; `AccessLedgerV1` (sqlite-free append-only segments, `lastAccessMs`, `size`, `pinnedBy`); REAPI `ContentAddressableStorage` compatible server mode (existing `🌐️remote-cache/🖥️server`).
- Consumes: `PinV1` (5) to skip pinned objects.
- Steps:
  1. Host root `~/.cache/semio/cas` (platform-specific equivalents), workspaces hold only references; `.cache/b` fixed images stay for tool-owned areas but their caps are accounted under the host cap.
  2. LRU GC: sweep when above high watermark until low watermark, oldest `lastAccess` first, never pinned objects; progress, cancellation, resumable.
  3. Cross-workspace dedup through reflink/hardlink (reuse `👯️dedupe`).
  4. Immediate quick wins: cap and age-bound `graph-memo` and `build-input-closures`.
  5. Ledger area totals reported by `repo cache status` and `bench`.
- Tests: fixtures of access traces -> expected evicted sets (oracle: Python LRU simulation); concurrency test with two workspaces writing identical objects (single stored copy); crash test (kill during GC leaves store consistent).
- Acceptance: host total store <= cap in the soak; two workspaces with equal trees use <= 1.1x the bytes of one; GC reclaims to low watermark with no pinned object loss.

## Lane 9: `scheduler` (fair global scheduler)

- Goal: one host queue with priorities and per-agent fairness replacing `build-slot.sh` and ad-hoc `repo run` contention.
- Owns: `BUILD/🔒️leases/**`, `BUILD/👷️workers/**`, `BUILD/🧵️workers/**`, `BUILD/🌐️remote-execution/**` (scheduler concepts), `BUILD/🗓️scheduler/**` (new), `BUILD/🪪️agent-sessions/**`.
- Provides: `JobRequestV1 {id, agent, workspace, class:"plan"|"compile"|"test"|"generate", priority, depth, estimate:JobEstimateV1, deadlineMs?}`; `SchedulerSnapshotV1 {running[], queued[], perAgentShare, hostBudgetMb}`; policy `SchedulerPolicyV1 {fairness:"weighted-round-robin", agingMs, priorityBands, maxPerAgent, sharedCompilerLanes:{rustc:n, tsc:n}}`.
- Consumes: `JobEstimateV1` and `JobLimiterV1` (7), transport (4), `AgentId` (agent-sessions).
- Steps:
  1. Extend resident governor (lanes round-robin, bounded queue) with priority bands (shared crates before dependents via task depth), weighted fairness, aging; merge with host-capacity queue semantics (aging, bypass) into one implementation behind the daemon.
  2. Admission = sum(learned estimates) <= host budget minus daemon RSS; the scheduler owns when a job starts; jobs start inside a limiter handle.
  3. Shared compiler lane: bounded number of rustc/tsc workers (reuse `👷️workers` persistent workers).
  4. `repo` MCP/CLI: `scheduler status`; agents identified by agent session.
  5. Retire `build-slot.sh` (delete from the NORM ticket scripts after lane 11 verifies).
- Tests: deterministic simulation fixtures (arrivals, estimates, priorities -> expected start order), oracle in Python (reference WRR with aging), fairness property (no agent starves, max wait bound), cancellation fixtures.
- Acceptance: with 16 agents submitting identical load, max/min per-agent throughput ratio <= 1.5, no starvation beyond `agingMs`, host MemAvailable floor never breached, no `build-slot.sh` usage.

## Lane 10: `bounded-index` (bounded in-memory caches and index diet)

- Goal: no unbounded maps in long-lived processes; index memory O(repo) and as small as possible.
- Owns: `BUILD/🕸️graph/🕰️generations/{🧮️fact-codec,🔌️load,🔀️merge,🌊️json-stream,🧱️chunk-classes,🌿️frontier-walk,🧭️frontier,🔭️coverage,🧵️edge-reads}/**`, `BUILD/🧊️bounded/**` (new), `BUILD/🕸️graph/⏱️phases/**`.
- Provides: `BoundedCacheV1<K,V> {maxEntries, maxBytes, sizeOf, onEvict}` (LRU, async-safe, with stats); `FactIndexViewV1` (columnar/packed, interned strings, lazy per-owner decode over the 512 MiB packed image; target <= 1 GB resident for 455k facts); `CacheStatsV1` exported to the daemon status and bench.
- Consumes: none.
- Steps:
  1. Inventory every `Map`/`Set` in long-lived modules (daemon state, durable memo views, digest cache `digestCacheMaximumFiles 2M`, memo `memoMaximumEntries 65536`), replace with `BoundedCacheV1`; lint-style test that fails on new unbounded module-level maps in listed directories.
  2. Packed-index read path: keep rows in the packed brotli/columnar form, decode on demand, shared row table already exists (`createDurableStateV1`): extend to the index.
  3. Memory accounting hooks (`heapUsed` budget from `🧠️memory`) trigger eviction instead of growth.
- Tests: property tests (random access traces: size never exceeds bound, hit ratio vs oracle LRU in Python), fixtures for packed index decode equality with the full decode (oracle = existing loader).
- Acceptance: full index resident <= 1.2 GB (from 2.7 GB expanded), RSS growth over the 2 h soak <= 5%; `CacheStatsV1` shows every cache within its bound.

## Lane 11: `integration` (wire up, retire old mechanisms, final gates)

- Goal: one fast path end to end, old mechanisms deleted.
- Owns: `BUILD/🕸️graph/🟦️.ts`, `BUILD/📜️script.ts`, `BUILD/🏃️execution/**`, `BUILD/🧪️tests/**`, retirement of `closure-verify-helper.mjs` and `build-slot.sh`.
- Steps: split `🕸️graph/🟦️.ts` (131 KB) into the service-neutral API used by lanes 2, 4, 6 (do this first, as an early small task); then integrate each lane as it lands behind its port; delete stat-pass fallbacks, whole-workspace default planning, delegate-whole-command path, per-process index loading, `build-slot.sh`; update `repo` MCP tool catalogue (`⚙️operations/🔣️catalog.json`), docs (README EN+DE); run full bench gate.
- Tests: end-to-end `.feature` scenarios: 16 agents edit/build/revert; kill -9 daemon; host memory pressure; cancellation; output equality with the legacy one-shot planner (Python canonical-JSON diff) during the cut-over only.
- Acceptance: all success metrics in `fleet-brief.md` hold in the bench gate.

---

## Critical path and start order

Critical path: `derived-dag` (2) + `journal` (3) -> `daemon` (4) -> `revision-cas` (5) -> `integration` (11). Lane 2 is on the path because without small nodes the daemon would still verify 18.5k-row closures; lane 4 needs 2 and 3 for the index to be useful once.

Waves:
1. Start immediately (no dependencies): `bench` (1), `job-limits` (7), `cas-gc` (8), `journal` (3), `bounded-index` (10), `derived-dag` (2, schema and oracle first), `scoped-plan` (6, closure computation against project nodes can use the existing inference output until lane 2 lands).
2. After the contracts are published (target: the first schema commit of each lane, within day 1): `scheduler` (9; needs `JobEstimateV1` from 7), `daemon` (4; needs `DirtySetV1`, `MemoStoreV1`, `ScopedPlanV1` types; can start protocol, multi-workspace endpoint and client work against stubs).
3. After lanes 2/3/4 are functional: `revision-cas` (5) (stage and pin need the daemon and CAS), then `integration` (11) final cut-over. Integration starts early only for the `🕸️graph/🟦️.ts` split.

Expected payoff order (for prioritisation when capacity is short): job-limits (stops host crashes at once), journal + derived-dag (removes the minutes-long re-plans), daemon (removes the 3 GB x N planners), revision-cas (removes source-changed loops), scheduler + cas-gc (long-session stability), scoped-plan (cold-plan cost).

## Cross-lane contract list (schema-first)

| Contract | Defined by | Used by |
|---|---|---|
| `NodeKeyV1`, `DerivedNodeV1`, `MemoStoreV1` | 2 | 4, 6, 10 |
| `DirtySetV1`, `JournalCursorV1`, `JournalReplyV1` | 3 | 2, 4, 5 |
| `PlanRequestV1`, `PlanReplyV1`, `ServiceStatusV1` | 4 | 1, 9, 11 |
| `RevisionV1`, `InputManifestV1`, `PinV1` | 5 | 2, 4, 6, 8 |
| `TargetScopeV1`, `ScopedPlanV1` | 6 | 4 |
| `JobLimitPolicyV1`, `JobLimiterV1`, `JobEstimateV1` | 7 | 9, 4 |
| `CasPolicyV1`, `CasStoreV1`, `AccessLedgerV1` | 8 | 5 |
| `JobRequestV1`, `SchedulerSnapshotV1` | 9 | 4, 1 |
| `BoundedCacheV1`, `CacheStatsV1`, `FactIndexViewV1` | 10 | 2, 4 |
| `MetricSampleV1`, `BenchResultV1` | 1 | all |

Publishing rule: each lane commits its `🧬️schema/🔣️.json` first and announces it in `📌️requests.md` under `## → <consumer lane>`; consumers code against the schema and a fake.
