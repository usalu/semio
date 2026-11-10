# 🏗️ Build Graph Design Review: Multi-Agent, Bounded Resources

Question: is the current closure-verification design optimal for many agents and workspaces editing the monorepo simultaneously on a 16 GB RAM / 200 GB disk host?

Answer: no. Today's fixes (durable memo, single-flight, interned rows, once-per-row checks, phase-learned admission) are mitigations inside a design whose cost grows with *edits × builds × repo size*. The requirement needs cost proportional to *what actually changed* and memory bounded independently of agent count.

## Observed failure modes (this session)

| Symptom | Root cause in the design |
|---|---|
| One edit to a `Cargo.toml`/`📋️project.json` invalidates almost all 2831 closures | Closures are coarse "command walks" (~18.5k rows each) that read most manifests of the repo; no intermediate derived nodes, so no early cutoff |
| Every plan re-stats/re-hashes millions of rows | Validation is a full scan of recorded facts, not driven by a change journal |
| Planner restarts verification mid-run ("source changed") under concurrent editing | Plans need a quiescent tree; there is no pinned snapshot of inputs |
| 2–3 planners × 3–11 GB → OOM, swap, crashes | Each `repo run` loads its own ~2.7 GB fact index; admission estimates were wrong; no hard per-process limit |
| Hours per build even for one small crate | Planning is whole-workspace (Nx graph) instead of demand-driven for the requested target |
| Caches and memory grow over long sessions | Unbounded in-memory memo; GC by age only |

## Target design

1. **One incremental graph service per host** (daemon), shared by all agents and workspaces. It holds the fact index once; `repo run` clients query it instead of planning themselves. Memory is O(repo), not O(agents × repo).
2. **Fine-grained, content-addressed dependency DAG with early cutoff** (Salsa / Bazel Skyframe / Buck2 DICE style). Files → parsed facts (e.g. per-manifest dependency set, per-project named inputs) → per-target input sets → action keys. A change re-evaluates only dependents, and stops where a recomputed node's value is unchanged (whitespace edit, unrelated dependency).
3. **Watcher-driven invalidation.** The existing `semio-repo-watch` journal feeds dirty paths; only those are re-stat/re-hashed. No full-scan verification, no per-plan rehash of untouched files.
4. **MVCC snapshots.** Each build pins a revision: inputs staged into the content-addressed store at that revision, so agents keep editing while builds run, and no "source changed → re-plan" loop occurs. Results are keyed by content, valid for every workspace with identical inputs.
5. **Demand-driven scope.** Evaluate only the transitive closure of the requested target (and its tool closure), never the whole workspace for a single crate.
6. **Bounded resources by construction.**
   - Admission on measured per-phase peaks (already learned now) plus a host budget that counts agent processes; hard per-job memory limits (cgroup v2 `memory.max`) so overruns fail one job, never the host.
   - Content-addressed store with a size cap (e.g. 120 of 200 GB) and LRU GC by last access across all workspaces; dedup across workspaces.
   - Bounded in-memory caches (LRU) everywhere; no unbounded maps in long-lived processes.
   - One shared compiler-cache/daemon lane (rustc, tsc) with bounded concurrency instead of N parallel cold compilers.
7. **Fair multi-agent scheduling.** A single global queue with priority (shared crates before dependents) and per-agent fairness, replacing ad-hoc per-agent `repo run`s and the ticket-local `build-slot.sh` gate.

## Migration path (incremental, no legacy kept)

1. Introduce the derived-fact layer (parsed manifests, named inputs) as memo nodes with early cutoff — removes the "one manifest edit invalidates everything" amplification.
2. Wire the watcher journal into invalidation; drop full-scan verification.
3. Move the fact index and planner into the host daemon; make `repo run` a client.
4. Stage inputs per revision in the CAS; pin builds to revisions.
5. Replace whole-workspace Nx planning with demand-driven target evaluation (keep Nx/Bazel as executors behind the tool-neutral interface).
6. Enforce cgroup limits and CAS size-capped GC; retire `build-slot.sh` and the closure helpers.

Owner: repo build infrastructure (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`). Recommended as its own ticket under goal `🎯repobinary` / `🎯aioptimizedrepo`, not inside the norm ticket.
