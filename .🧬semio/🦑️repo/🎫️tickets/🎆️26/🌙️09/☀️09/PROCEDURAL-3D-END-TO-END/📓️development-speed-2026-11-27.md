# 🚀️ Development speed: clean long-term solutions (2026-11-27)

These proposals are grounded in what slowed down the procedural-3D end-to-end ticket. Each item names the observed cost, then the clean fix. They are ordered by expected time saved per agent-hour.

## 1. Served-artifact provenance (stale builds must be impossible to miss)

**Observed.** Activation printed "Activated extension …" while it served a 43 MB Math core from Nov 25. `:package` builds only the model facet; the browser modules come from `:component`. About 4 h went into debugging a fault that was already fixed in source.

**Fix.**
- Activation and the dev server stamp every served module with the source-closure digest it was built from.
- Activation refuses (fail-closed) when that digest does not match the current source closure.
- One `activate-*` target depends on every facet it serves, so a single command yields a fresh, complete stack.
- Expose a `repo served status <host>` query: per module, its digest, build time and staleness.

## 2. Fast native inner loop for every browser fault

**Observed.** One requester rebuild plus probe cycle takes 1 h or more on a 16 GB host. The pack-projection and surface-capture faults were both reactor logic that a native law could reproduce in seconds. `semio-framework-pack` is not runnable through a plain native test lane.

**Fix.**
- Every component crate gets a native law lane, using the same schema-first host interface with a native host double.
- Record a probe's host-event trace (the `EXT_*` and reactor events the journey already logs) as a replayable fixture.
- A `repo replay <trace>` target runs that trace natively against the reactor. Browser faults then become native regression laws without any wasm build.
- Rule: no wasm or browser cycle before a native reproduction is red.

## 3. Wasm panics as structured faults

**Observed.** The requester panic surfaced only as a wasm `unreachable` trap, with both calls left "in flight". The cause had to be dug out of logs.

**Fix.** Install a panic hook in every component that emits a typed fault event (message, location and the reactor event being handled) through the host interface. The probe then fails at once with that fault instead of waiting for its timeout.

## 4. Snapshot-consistent builds in the shared replica

**Observed.** Builds broke with `BUILD_SOURCE_UNPARSEABLE` and `BUILD_SOURCE_CONFLICT` while another agent was mid-edit on a BIM `Cargo.toml`, and twice on stale `Cargo.lock` files. Every ticket wrote its own retry script for this.

**Fix.**
- The build reads one content-addressed snapshot of its input closure. A file changing mid-read retries only that file read, not the whole build.
- Lock staleness relocks automatically as a planned build step, instead of failing.
- Transient categories (unparseable mid-edit, owner collision, ENOSPC) get one owned retry policy inside `repo run`, so per-ticket retry scripts disappear.

## 5. Derived fixtures instead of hand-pinned hashes

**Observed.** The composition ownership gate held 73 stale sha256 pins and 6 renamed law names. A ticket script (`ownership-hash-drift.ts`) had to repin them.

**Fix.** Make the pins a generated, sealed artifact of the law sources, with a check target that diffs against it. Renaming a law updates the manifest in the same build. The gate then proves ownership without anyone touching hashes by hand.

## 6. Capacity: measured, not guessed, and offload-first

**Observed.**
- A nested capacity grant deadlocked (fixed).
- Guessed grants over-reserved memory: 9 GB admitted against 4 GB actually used.
- The memory warden killed the WGPU build 5 times and Math twice.
- Fleet workers, including a 64 GB box, declined most units because "local slot free".

**Fix.**
- Persist the peak resident memory per unit and target in the cache ledger, and size grants from it (p95 plus a margin).
- Route heavy linked units (wasm components, the renderer) to the fleet by default whenever the local host is under 32 GB, instead of only when local capacity is exhausted.
- Have the fleet accept work based on the requester's capacity, not on its own idle slot.

## 7. Disk: images that can return blocks

**Observed.** ENOSPC hit twice, and 19 GB of scratch directories had to be deleted by hand. The 96 GiB rust image cannot unmap freed blocks, because macOS unmaps only images up to 32 GiB.

**Fix.**
- Shard the cache areas into images of 32 GiB or less, so `repo cache gc` actually returns space.
- Make ticket-scoped scratch (`tmp/*-n25`) a leased area that gc reclaims when the lease ends.
- Have `repo run` run gc preflight below a free-space floor.

## 8. A warm graph daemon

**Observed.** Every invocation spends 15–25 s re-scanning the file inventory ("Reading the project graph slice"), even for one-target runs.

**Fix.** Run a long-lived graph daemon per machine, fed by the replica daemon's file events. It keeps the sealed graph current incrementally, so a `repo run` starts in under a second.

## 9. Short, targeted journeys

**Observed.** The 19-row journey takes 20–40 min (the baseline wait is set to 1 200 000 ms), and its first meaningful signal arrives only after about 250 s of boot.

**Fix.**
- Rows declare their preconditions and reuse one booted session.
- The probe exits on the first typed fault (see 3) or on reaching its success predicate, not on its maximum wait.
- A row-level `--only` with a cached boot snapshot gives a 1-minute loop for a single row.

## 10. Recipes as owned, queryable knowledge

**Observed.** Agents rediscovered how to build, install and serve each lane:
- the runtime-root argument of `install-local-procedural.ts`;
- which target builds the browser modules;
- the screen-based serve scripts.

This knowledge lives in ticket scripts that the next agent never sees.

**Fix.**
- Promote recurring ticket scripts into schema-declared `repo` operations, for example `repo dev stage generation3d --host=react|wgpu`, which builds every facet, installs, activates, serves and reports its URL.
- Add `repo explain <project>:<target>`, which says what a target produces and who consumes it.

## 11. Versioned machine-wide records (added 2026-11-28)

**Observed.** The host-capacity grant schema changed twice in one night while builds were in flight:
- at 23:36, the stats fields moved into `.stats` files;
- at 01:26, a required `format` field was added.

Each time, grants written by live processes running older code made the fail-closed status reader refuse admission for the whole machine. One requester build stalled for more than 2000 s until it was restarted. The refusal message key was also missing, so the error surfaced as a `TypeError`.

**Fix.**
- Every machine-wide record (grants, waiters, leases, ledger rows) carries a `format` version.
- Readers accept the previous format for the life of its writer process. They charge an unknown-format live grant at its declared `reservedMegabytes`, judged by the minimal owner shape (pid, startMs, bootAtMs), instead of blocking admission.
- A law checks that each schema change ships with an N-1 reader. This is not backwards compatibility for users: it is coexistence of processes during a live rollout in the shared replica, where code changes under running processes every second.
- Message-key completeness becomes a generated check: every error kind thrown must have an `en` and a `de` message.

## 12. One owner per ramping service (added 2026-11-28)

**Observed.** A single host-service process ran two concurrent graph-slice loads, and the second was charged as if it were a foreign process (fixed via a same-pid credit).

**Fix.** Make the slice pool, not each load, the capacity holder: one grant per service, grown and shrunk by the pool. Concurrent loads then share it by construction.

## Suggested order

| Priority | Item | Why first |
|---|---|---|
| 1 | 1. Provenance, 3. Panic faults | Cheap, and they remove whole classes of wasted hours |
| 2 | 2. Native replay lane | Turns hour-long cycles into seconds |
| 3 | 4. Snapshot builds, 10. Staging operation | Removes per-ticket retry and stage scripts |
| 4 | 6. Measured capacity and offload, 7. Disk sharding | Fixes the host bottleneck structurally |
| 5 | 5, 8, 9 | Steady speedups |
