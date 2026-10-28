# Exclusive lock holds: build library and process module (wave 1026g)

Auditor `aud-locks-g`, read-only. No build, test or run was started; no source file was edited.

Scope: the build library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/`, plus the process module `🧰️framework/🔨️modules/🏃️process/`.

Path keys (full path = key, file:line after it):

| Key | Full path under repo root |
|---|---|
| L | `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` (lease core) |
| LO | `…/🔒️leases/🧭️order/🟦️.ts`; policy `…/🔒️leases/🧭️order/🔣️policy.json` |
| CAP | `…/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts` |
| CN | `…/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts` (cargo build lease) |
| CU | `…/🏗️native-build/🔒️lease/🔐️custody/🟦️.ts` (cargo custody) |
| CT | `…/🏗️native-build/🔒️lease/🎯️target/🟦️.ts` (target binding) |
| CMP | `…/🏗️native-build/🧩️component-pass/🟦️.ts` |
| NS | `…/🏗️native-build/💾️storage/` (native storage; subpaths below) |
| GR | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts` |
| GEN | `…/🏗️build/🕸️graph/🕰️generations/🟨️.mjs` |
| GLD | `…/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs` |
| WR | `…/🏗️build/📥️inputs/💾️storage/🟨️.mjs` (graph fact writer) |
| WSH | `…/🏗️build/📥️inputs/💾️storage/🔒️writer/🟨️.mjs` (shard naming) |
| INF | `…/🏗️build/🕸️graph/📥️inference/🚦️invocation/🟨️.mjs` |
| CMPW | `…/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` |
| LAU | `…/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` |
| RLY | `…/📚️library/🏃️process/🟦️.ts` (cargo relay, line 81) |
| DEP | `…/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` |
| BZ | `…/📚️library/🏗️build/🔧️tools/🏗️bazel/` |

## Mechanism

- Every exclusive lock is a SQLite file per resource (`sha256(resource).sqlite`). Exclusive means `BEGIN EXCLUSIVE` (L:217); the transaction stays open from acquire until `release()` (L:223-226). No `flock` or lockfile primitive is used in build or process code (grep). The only lockfile primitive, `createFileLeasePortV1` (GEN:627-650), is used by tests only, and it steals a lock older than 600 s even when the holder is alive, so it must not be used for 60-120 min holds.
- `withResourceLeases` holds every lease for the whole operation (L:579-588). Resources are sorted by (rank, name) (L:569-572).
- Queued acquisition (`acquireQueuedResourceLease`, L:472-491) takes a FIFO ticket in `<hash>.queue/` plus a shared claim; the claim is held for the entire wait (L:489, L:564). Direct `acquireResourceLease` polls every 40 ms or less with no ticket (L:239), so it is unfair. `withResourceLeases` routes through the queue (L:575).
- Waiting does not hold the directory admission gate (L:309, L:313-326). Holding the gate is microseconds per acquire (L:327-332), except during retirement (Hz2).

## 1. Exclusive holds, ranked by risk of long holds blocking other agents

Risk: CRITICAL = can hold for an hour or more and blocks a whole product family; HIGH = minutes, blocks a shared resource; MED = bounded I/O or unfair; LOW = short.

| # | Lease (resource) | Site | Protects | Work while held | Risk | Narrowing (one line) |
|---|---|---|---|---|---|---|
| 1 | `build-graph-state:<root>` (unscoped) or the scoped publication writer | GR:128-137, acquire GR:134 (queued, no timeout); callers GR:143, GR:390 (`withBuilder`), GR:625-632 | Graph publication, invalidation and build | Whole compute: `loadBuildGraphAttemptV1` (GR:515) runs inference over about 566 projects (60-120 min, builder 5-13 GB RSS, per brief); on the generation path, `build()` inside `withBuilder` (GEN:508) | CRITICAL | Keep exclusive for the builder claim only; compute runs without the state lock and readers stay lock-free (GEN:307 already), and take the state lock only for the pointer swap and invalidation. |
| 2 | `repository-graph-facts:<root>` (canonical, WR:167) or `…:shard:N` (N = sha256(kind:identity)[0] mod 64, WSH:5-10; 64 shards per schema `🧬️schema/🔣️.json:333`) | WR:154-192: acquire WR:171 (`timeoutMs:0`), body WR:180, release WR:182, wait outside the body and retry WR:185 | Fact store and ledger writes of one namespace | Every inference child runs under it for its whole life; children reuse it through the pinned invocation (WR:160, INF:31). Tonight: pid 64323 held shard 7 for 98 min (`result-e2e-f.md:21`), pid 21103 for 2 h 03 min. | CRITICAL | Compute into the namespace's staging dirs (`selected`, `contexts`, WR:204-234) and take the shard only for the rename and ledger commit (the publish pattern of GEN:417); 1-in-64 collisions between unrelated namespaces also argue for namespace-keyed commit leases. |
| 3 | Cargo build-dir custody `JSON(["semio.cargo.custody-lease/…", buildDir, profile])` | CU:61-80 (queued exclusive CU:69-72; in-process coalescing CU:63; inherited by a process tree CU:65); taken at CN:65 inside `acquireCargoBuildLeaseV1` (CN:55-70) | One process tree compiles into a build dir and profile at a time (fine-grain-locking deadlock, CU:61) | Whole compile: LAU:172-183 holds it across the capacity wait, scheduler, jobserver, broker and the full cargo run; RLY:81 holds it across the whole cargo child until exit or budget kill | HIGH (tonight's serialisation) | Under custody, try capacity with `timeoutMs:0`; on refusal release custody, wait for capacity unlocked, re-take custody (the rank order forbids capacity before custody); and give each owner, product and profile its own build dir so unrelated builds never share one custody. |
| 4 | Per-profile cargo custody set in a native generation | CMP:390-393: a loop acquires one custody per profile before `withCapacity` (CMP:394) | Each profile's build dir | Held through the whole native component build: scheduler, jobserver, broker and every rustc pass | HIGH | Acquire each profile's custody when its pass starts and release it when the pass ends (ascending order kept when two are held). |
| 5 | Directory admission gate `admissionResource` plus retirement catalog (`resource-retirement`) | L:387-400 (gate L:389; slice L:390-398 with `await control.publish` L:390 and `yieldContinuation` L:398; cursor write L:399); claims L:412-417; sweep started inside a caller's acquire L:481 | Admission of every opener in one lease directory (openers take the gate shared, L:327-331) | Up to one slice of candidates: DB open, exclusive probe, `unlinkSync`, progress publish and yields | HIGH (every acquire in that store waits; openers time out after 5000 ms, LO policy `admissionPatienceMilliseconds`) | Take the gate exclusively only for each candidate's probe and unlink; release before `publish` and `yieldContinuation`; write the cursor under the catalog lease alone. |
| 6a | `bazel-modules-<identity>` | BZ `📜️script.ts:87` | Pinned Bazel module catalog | `bazel build //:setup` (network, minutes) and receipt write | HIGH | Run bazel outside the lock; exclusive only for the receipt write after a double-checked read. |
| 6b | `bazel-bun-<sha>` | BZ `🏃️execution/🟦️.ts:228` | Pinned Bun executable | Download, verify and extract (staging `mkdtemp` inside the body, line 231) | HIGH | Stage before acquiring; take exclusive only to rename into place. |
| 6c | `cargo:registry:<home>` writer (rank 8 `cargo:registry-fetch:` per policy) | DEP:68-72 (queued exclusive) | Cargo registry and lockfile resolution | `body(view)`: the lockfile resolution and fetch, network | HIGH | Resolve into a staging lockfile; exclusive only for the atomic rename. Tonight's registry hold-and-wait rows: `result-incr-locks.md:21`. |
| 7 | Compiler capacity: gate `semio.compiler.host-capacity/v1` (shared, or exclusive for whole-host, CAP:67), picker (exclusive, CAP:74), cores `semio.compiler.cpu/v1/i` (exclusive, timeout 0, CAP:76) | CAP:63-86; picker released after the grant loop | CPU units | Until `retire()`: the whole compile; the picker is held while polling for cores | HIGH for whole-host: an exclusive gate waits for every shared holder (CMPW:56-57 records a one-hour convoy) | Express whole-host as an `available` request with maximum = host instead of an exclusive gate. |
| 8 | `native-storage:<identity>` | NS `🟦️.ts:90` (queued); timeout-0 variants NS `🟦️.ts:186,195`, NS `📤️publication/🌐️platform/🟦️.ts:29,53,95,107,124` | Native image journal | Journal read, the caller's `run(state)` (arbitrary copy or mount work), and an fsync'd rewrite | MED-HIGH | Run the caller's work on a state snapshot and commit under exclusive with the existing revision check (the pattern at NS `🟦️.ts:86-87`). |
| 9 | `native-fixed-image:<dir>` (retirement) | NS `🍎️image/🟦️.ts:42` (body to about line 50) | Mounted image lifecycle | hdiutil info, attach, detach and unlink | MED | Probe the mount state before acquiring; keep only detach and unlink inside the lease. |
| 10 | `persistent-state:<root>` and `persistent-data:<image>` | `🧰️framework/🔨️modules/🏃️process/💾️state/🟦️.ts:31` and `:37` (both queued exclusive) | Persistent app state | Held for the state's open lifetime (callers not traced) | MED | Shared for reads; exclusive per write transaction. |
| 11 | Unqueued, unbounded exclusive (no ticket, no timeout) | `🧰️framework/🔨️modules/🏃️process/🏗️build/🗃️state/🟦️.ts:44-46`; `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts:73`; `…/📚️library/🏗️build/📖️storybook/💾️storage/🟦️.ts:22` (`…` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/`) | State load, artifact publish rename, storybook write | Their bodies (I/O) | MED (starves under shared cohorts) | Use the queued acquire with a timeout. |
| 12 | Compiler writer exclusive: maintenance and headroom escalation; cargo profile maintenance | CMPW:37 (exclusive custody); CMPW:57-68 (escalates to exclusive `wait:false` for `recover()`); `…/📚️library/⚡️caching/🧹️pruning/🟦️.ts:258-260` (`acquireCargoProfileMaintenanceLeaseV1`) | Writer custody of a compiler image; profile maintenance | `recover()` and cache deletes (I/O; cache is about 150 GiB) | MED | Claim first, delete after, as `retireResources` already does for DBs. |
| 13 | `repository-graph-optional-ledger:<root>` | WR:64-65 (queued exclusive) | Optional ledger | Retry loop on SQLITE_BUSY that publishes progress (WR:65) | MED | Open shared; exclusive only for quota mutation. |
| 14 | Artifact-cache producer per (directory, context) | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🏗️producer/🟦️.ts:18` (queued exclusive) | One cache artifact being produced | Producer body (compile-like; not traced) | MED | Produce into a staging name; exclusive only for the publish. |
| 15 | `native-snapshot-slot:<dir>` | `…/🏗️build/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts:38` (queued exclusive; readers shared at lines 39, 51) | Snapshot writes | Snapshot copy I/O | MED | Copy to staging; exclusive only for the rename. |
| 16 | Generation builder fallback | GEN:508 (`lease.exclusive(resources.builder, body, {signal})`, no timeout) | Builder for the generation store, when no `builder` option is passed | Compute | LOW (reference path) | Add a timeout or remove the fallback. |
| 17 | Cache GC per resource | `…/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts:66, 72, 106` (timeout 0 except line 72) | Per-resource GC of closures | Closure scans (compute) | MED-LOW | Scan outside the lock; lock to delete. |

Good example to copy: GEN:417 `publish` takes `resources.facts` exclusive with `publishWaitMilliseconds` around one pointer read and one write only.

Other exclusive sites, LOW or not traced: CT:8-11 (receipt write); GR:359-360 (`lease.exclusive` helper); BZ `📜️script.ts:736` (`bazel-session-collector`, timeout 0); WR:249-252 (fixture writer, tests); NS `🧱️fixed/🟦️.ts:101,129`; NS `🔒️mapping/🟦️.ts:24-27` (not traced); the namespace baseline in `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️namespace/📒️baseline/` (7 sites) and `…/🏗️native-build/🎟️admission/🟦️.ts` (2 sites) (not traced).

## 2. Shared-to-exclusive upgrade paths

- U1 `upgradeQueuedResourceLease` (L:511-523). Releases the shared hold (L:520), then re-enqueues exclusive at the original arrival position. Not atomic: data read under shared can be stale, so the caller must re-validate. A failed upgrade holds nothing. No product caller found (only the definition in non-test code).
- U2 `withCompilerSharedHeadroomV1` (CMPW:57-68). Shared writer custody for the check, then exclusive with `wait:false` (never queued) only to recover headroom, then the body runs shared again. Non-blocking escalation, so no hold-and-wait.
- U3 Generation load (GLD:285-289). Releases the reader (`withShared`) and then takes the builder (`withBuilder`) exclusively; `attempt(true, …)` re-verifies. Release-then-acquire, not atomic.
- U4 Re-entrant shared bypass (L:465). A holder of a shared hold may take another shared without the queue when an exclusive ticket is queued (`exclusiveTicketQueued`, L:505-509), to prevent writer-induced deadlock. The holder check (LO:253-258) also counts inherited parent holds and build-invocation pids.
- U5 Cargo ladder (CN:62-65). Store shared, then profile shared cohort, then target binding exclusive (CT), then custody exclusive. Different resource names, so it is a ladder, not an upgrade. The exclusive custody waits while shared holds are kept, which is allowed because shared holds are exempt from hold-and-wait (LO:117).
- U6 Retirement cursor (L:365). Exclusive COMMIT, then `BEGIN EXCLUSIVE` again under the gate. Internal transition.

## 3. Lock order and hold-and-wait rules

Classes and ranks (POL `classes`): 1 toolchain; 2 tool-singleton, cargo-coordination, cargo-custody; 3 compiler-capacity; 4 graph; 5 namespace; 6 artifact, native-whole-retirement; 7 image; 8 registry-fetch. Unclassified resources have rank 0 and are unconstrained.

Allowed edges that matter (POL `allowed`): graph to graph; compiler-capacity to graph and to tool-singleton; cargo-coordination to compiler-capacity and to image; image to compiler-capacity; cargo-custody to every class above it, and to cargo-custody in ascending profile order; tool-singleton to toolchain; native-whole-retirement to image (binding).

Not allowed: graph to compiler-capacity, namespace, artifact or image (the shared image medium is exempt, LO `isLeaseMediumV1`). A graph holder that waits 15 s for cores is therefore refused.

Strict pairs (POL `strictWaits`, enforced): holding `repo:compiler-writer:` while waiting for cargo registry, configuration or rustup; holding `cargo:registry-fetch:` while waiting for `repo:compiler-writer:`; holding compiler cpu units or host capacity while waiting for `repo:compiler-writer:`.

Enforcement (POL `rules`): outOfOrder = observe (logged only, LO:104-110); holdAndWait = enforce (LO:114-119: only EXCLUSIVE holds count, grace 15000 ms, re-checked on each poll, LO:237-238); selfDeadlock = enforce (LO:86-90); cycle = enforce (LO:138-152, 500 ms cadence, victim is the last waiter); strictWait = enforce (LO:92-100).

Inheritance: a child process inherits its parent's holds through `SEMIO_LEASE_INHERITED` and counts them as held by every task (LO:181-196). A nested `repo run` inside an orchestrated action therefore inherits the parent's exclusive holds, and it is killed if it waits 15 s for a class not allowed from them.

Tonight's documented case: `result-incr-locks.md:21` lists hold-and-wait rows of exclusive `cargo:registry-fetch:<home>` while requesting `repo:compiler-writer:`, which is a declared strict pair. I did not open the dashboard log, so I cannot confirm which pair killed the dashboard build.

## 4. Hazards

- Hz1 `createFileLeasePortV1` (GEN:627-650) steals a lock after 600 s even when the holder is alive. Test-only; do not reuse for long holds.
- Hz2 Retirement holds the admission gate exclusively across `await control.publish` (L:390) and `yieldContinuation` (L:398). A slow progress consumer blocks every opener of that directory.
- Hz3 Unqueued exclusive callers (table row 11) starve under shared cohorts: the polling loop has no ticket (L:236-239).
- Hz4 Name collision: the canonical writer (WR:167) and the generation publish (GR:394, `resources.facts`) use the same string `repository-graph-facts:<root>`. A publish that ran inside the canonical writer would wait on its own lease and self-deadlock after 15 s (LO:86-90). I did not confirm a call of publish under that path; the generation builder runs under scope or shard writers (GR:390).
- Hz5 The cargo relay holds custody for the whole child (RLY:81); the only bound is the budget kill.

## 5. Order of work

1. Graph: move compute out of rows 1 and 2; commit-only exclusive (GEN:417 pattern). This is what tonight's 98-min and 2 h 03 min holds need.
2. Cargo: per-owner build dirs; try capacity under custody with timeout 0 (row 3); per-pass custody in component builds (row 4).
3. Retirement gate per candidate (row 5).
4. Downloads and registry fetch staged before any exclusive (rows 6a-6c).
5. Replace unqueued exclusives (row 11) and whole-host exclusive gate (row 7).

## Limits

Static reading only. No runtime measurement was taken. Not traced: bodies of rows 10, 14, 17 and the storybook and state callers; the cargo wasm path beyond CN:55-70; the timing of `queueMayProceedV1` fairness.
