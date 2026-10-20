# result-ingress-perf: cargo ingress and CLI startup cost

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `S` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`. Tools and staged set: `C/ingress-perf-tools/`.

## Status
STAGED, NOT APPLIED, NOT MEASURED LIVE. No "PAUSE LIFTED" message reached me (coordination.md shows the sync green at 14:45 and integrate applying other sets, but no message to this executor). I therefore made no write outside `C/`, ran no `bun repo` command, and no cargo. The cold/warm table in section 5 is empty on purpose; the harness to fill it is ready (`bench-cargo.sh`).

## 1. Where the ~27 s per call go (code reading, plus numbers already measured by peers)
Static module load is NOT the cost: the whole build CLI closure loads in 30 ms, the cargo launcher closure in 150 ms (`load-time.ts`, load average 40).
The cost is serialized custody round trips, in this order:
1. `storage.recheck()` (the reservation recheck) is a durable-journal transaction: mapping SH lease (SQLite) + physical recheck + **EXCLUSIVE queued lease `native-storage:<identity>`** + second physical recheck + read and parse of the whole journal. `C/result-cargo-pressure.md` measured 415 ms at load 55 and 1340 ms at load 150 per call; the ingress performs dozens to hundreds.
2. Every physical recheck of a persistent image runs `verifyPersistentManagedChildrenV1`: two `hdiutil info -plist` spawns (double collect), plus a mapping SH lease. `C/result-roster.md`: 450 rechecks per 100 s in one cargo process, i.e. 900 hdiutil spawns.
3. Every `openRepositoryCachePartitionV1(<non-lease>)` first opens the `leases` image again, and a lease open is open, failed/successful EX maintenance (which closes the receiver), close, reopen, i.e. two persistent opens plus an idle-lease sweep. A `bun repo cargo` does this for leases, diagnostics, tools and rust.
4. `cargo fetch --locked` under an EXCLUSIVE `cargo:registry` lease on every call whenever the lock has registry crates (the existing ready receipt only covers pure first-party locks; the root `Cargo.lock` has 801 registry crates). Also serializes against `dependencies sync`.
5. Scheduler step (2.2 s measured by bzl-rust): unit-graph capture, source admission and a `cargo build --release` of the scheduler crate on every call.
Not yet attacked (measurement first): `rustc -vV` and `cargo config get` spawns of the configuration view, `captureCargoCompilationGraphV1` (0.7 s) and source admission (1.1 s, cargo-pressure's staged patch already removes most per-read rechecks), diagnostics store opened before every command (`🏗️build/📜️script.ts` `runOwnedBuildInvocationV1`), repeated lease image opens (item 3 is only halved by the stamp).

## 2. Staged changes (one anchored series, 37 anchors over 9 live files, 21 new files)
Apply: `sh C/ingress-perf-tools/apply.sh --dry-run`, then `sh C/ingress-perf-tools/apply.sh --ignore-sync` (refuses while a dependency sync runs; every anchor must occur exactly once). Then `sh C/ingress-perf-tools/verify.sh`.

| # | Change | Files | Custody argument |
|---|---|---|---|
| A | `reservation.recheck()` reads the journal under a **shared** cohort lease instead of an EXCLUSIVE queued one, and caches the parsed state keyed by the journal **revision** (`dev:ino:size:mtimeNs:ctimeNs` of `storage-events.jsonl`, single link, regular file; a state is remembered only when the revision is identical before and after the read). An unchanged revision skips lease, read and parse; the physical recheck still runs. | `S/🟦️.ts` (custody `read`), new `S/📜️journal/🔖️revision/` | Every append changes size and mtime, every checkpoint rewrite changes the inode; the allocation verdict depends only on journal content. Writers keep their EXCLUSIVE transaction. |
| B | **Kernel roster service**: one process-wide single-flight `hdiutil info -plist` capture shared by all images (an arrival during a capture waits for the next one, never for an older one); `after` forces a newer capture; an accepted snapshot may be reused only inside an explicit age bound. The double collect compares against the receiver's previously settled rows (one capture instead of two in steady state; a changed roster still needs two agreeing fresh captures). Image open/inspect uses the shared single-flight with age 0. | new `S/🍎️image/📡️roster/`, edits in `S/🧹️cleanup/🎛️command/🟦️.ts`, schema constants in `S/🧬️schema/🔣️.json` (`recheckWindowMilliseconds` 500, maximum 5000) | Same acceptors, same retry policy; torn snapshots are dropped from the shared slot. |
| B2 | **Recheck window** (opt-in `AsyncLocalStorage` scope, entered by `launchRepositoryCargoV1`): inside it a full physical verification (mapping lease plus roster) is reused for 500 ms, while the cheap identity checks (ancestors, mount root dev/ino, receipt stamp, `statfs`, backing identity) run on every call; concurrent callers join the in-flight verification and run their own if the partner failed. `syncMetadata`, maintenance entry/exit/settle and image open stay strict (renamed `strict`). | same | A detach or replacement of the mount is caught immediately by the cheap checks; only roster-only changes (descendant mounts) lag at most one window. |
| C | **Fetched-registry receipt** for locks with registry crates: `{request, locks[{path,digest,bytes}], registry[{path,dev,ino,mtimeNs}]}` of `registry/cache`, `registry/src` and their children, captured right after a successful `cargo fetch` under the EX lease. A hit takes only the SHARED registry lease and skips the fetch. Git sources, unparsable locks and path-only locks are refused (unchanged behaviour). | new `L/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/` (+schema), edits in `🟢️ready/💾️storage/🟦️.ts` | Fetch is monotone-add; unchanged lock bytes and unchanged registry directory stamps prove the fetched set is still present. `--locked` staleness is still enforced by the next locked planning call. |
| D | **Scheduler receipt** (`.semio-build-scheduler.json` in the target dir): sources (Cargo.toml, Cargo.lock, each bin path) by digest, a toolchain-and-environment identity (toolchain generation digest plus every `CARGO_*`/`RUST*` variable), and executable stamps. A hit returns the executables without planning, admission or cargo. | new `S/🎟️admission/🔖️scheduler/`, `buildSchedulerV1` takes `toolchain`, launcher passes `store.toolchain.digest` | Hit requires identical inputs, identity and untouched executables. |
| E | **Long-lived custody session** (answers the coordinator's bworkers request): `openRepositoryCustodySessionV1({workspace,invocation,control,partitions,compiler?})` enters the cache stores (graph, dependencies, ...) and optionally the compiler store once, takes an `AsyncLocalStorage.snapshot()` inside the innermost body and runs every request inside it, so nested `withRepositoryCacheStoreV1` calls take their existing `held` path and nested `withRepositoryCompilerStoreV1` calls take the new retained path (same invocation required, `recheck` before/after, per-request deadline and **abort signal** as a refinement of the held control). `createRepositoryCustodyPoolV1` opens lazily, shares one session between concurrent requests, closes it after `idleMilliseconds` and after `maximumLifetimeMilliseconds` between requests, and reopens after a failed request, so a persistent worker never starves exclusive maintenance or `dependencies sync`. `openRepositoryCargoSessionV1(invocation,control,workspace,configuration,environment)` returns `stream`/`capture` per command with its own deadline/signal and a held compiler store (one image open, reservation, toolchain generation per session; the ingress gate, profile injection, leases and capacity admission still run per command because they are inside `launchRepositoryCargoV1`). | new `L/⚡️caching/💾️storage/🔒️enforcement/🧵️session/` (+ `🏊️pool/`), new `L/🏃️process/🦀️cargo/🚪️launcher/🧵️session/`, `withRetainedRepositoryCompilerStoreV1` plus a reuse branch in `🏗️compiler/🟦️.ts` | Reuse is refused unless the caller's control belongs to the retained invocation. |
| F | **Lease-image sweep stamp**: `.retired` in the lease data directory; a stamp younger than 15 s (schema constant) skips the EX maintenance, the close and the second open of the `leases` image. | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` (+ `📂️parents` schema constants) | The sweep is opportunistic ("skipped retirement loses nothing" in the existing docstring). Symlinks, missing and future stamps never count as fresh. |

For graph-perf: the graph store session (item E) needs no edit of the store module; `createRepositoryCustodyPoolV1({partitions:["graph"],...})` is the intended entry. I sent them a message so they do not duplicate it.

## 3. Tests (language-agnostic fixtures with an independent oracle)
| Test | Fixture | Independent check |
|---|---|---|
| roster + gate (13) | `roster.fixture.json` (6 roster scenarios, 2 gate scenarios) | Python reference simulation of the same steps; real darwin law: stable image plus 3 attach/detach churners, 8 readers x 48 reads, `captures < reads`, and `plutil -convert json` of a second inventory |
| journal revision (3) | `revision.fixture.json` | Python `os.lstat` model replays the same mutations |
| fetched receipt (13) | `fetched.fixture.json` (lock classes, lock owners, mutation sequence, admission) | Python `hashlib` and `os.lstat` recompute digests and stamps of every captured receipt |
| scheduler receipt (6) | `scheduler.fixture.json` | Python `hashlib` digest of the manifest |
| custody pool (7) | `pool.fixture.json` (6 lifecycle scenarios) | Python reference simulation |
| lease stamp (7) | `stamp.fixture.json` | Python `os.lstat` age |

## 4. Verification done (all in copy overlays under `C/🗑️generated/ingress-perf-*`, since deleted)
- Overlay = copies of the 666-module closure of the build CLI, patched by `apply.py --root`; the same anchored set applies cleanly to the LIVE tree in dry run (`dry-run ok: 37 anchored edits over 9 files, 21 new files`).
- `bun test` in the patched overlay: roster 13 pass, revision 3, scheduler 6, fetched 13, pool 7, stamp 7 (0 fail). Existing `🍎️image/📥️information/🧪️tests` 13 pass before and after; the storage, opening and admission suites need the repo harness and fail identically (before = after) in the overlay.
- `tsc --noEmit --strict` over every edited and new file: no diagnostic that the pristine copy does not already have (`tsc-check.sh`; only path-text noise differs).
- New file paths: max 126 characters.
- NOT done: any live run; `launchRepositoryCargoV1` end to end with the window, receipts and session; the session's ALS snapshot reuse against the real store (compiler reuse falls back to a normal open if `requireIssuedResourceControlInvocationV1` rejects the control, which is the safe direction; whether `receiveScriptProcessInvocation` hands the same invocation inside the retained scope must be proven live); taxonomy/crate-boundary laws for the new directory names; `bun repo --help` after apply.

## 5. Measurement plan after "PAUSE LIFTED" (empty table)
Harness: `C/ingress-perf-tools/bench-cargo.sh <label> <runs> [profile]` runs `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml` with `SEMIO_CARGO_TIMING=1`, the spawn census preload (`census-preload.ts`: every `spawn`/`Bun.spawn` with duration, grouped table at exit, also for child bun via `BUN_OPTIONS`), `phase-table.ts` (store/inputs/graph/source/writer/profile/scheduler durations) and optional `bun --cpu-prof`. Order: baseline (3 runs, unpatched), `apply.sh --ignore-sync`, `verify.sh`, patched cold (first run after apply) and warm (runs 2-4), then a session run via `openRepositoryCargoSessionV1` with 5 commands (open once vs 5 standalone calls). Target <= 2 s warm; if the warm table shows `leases`/diagnostics opens or `cargo config get` dominating, the next levers are: one leases image shared per process, lazy diagnostics store in `runOwnedBuildInvocationV1`, configuration-view receipt keyed by the config file digests.

| Run | store | inputs | graph | source | writer | scheduler | total | hdiutil spawns | notes |
|---|---|---|---|---|---|---|---|---|---|
| baseline cold | not run | | | | | | | | |
| baseline warm | not run | | | | | | | | |
| patched cold | not run | | | | | | | | |
| patched warm | not run | | | | | | | | |
| session, 5th command | not run | | | | | | | | |

## 6. Open items
1. Apply, run `verify.sh`, fill section 5; reorder levers by the census.
2. Window length (500 ms) and sweep interval (15 s) are schema constants chosen by reasoning, not measured.
3. A session holds SHARED image guards; with FIFO exclusive queues a long session blocks EX waiters. The pool's idle/age limits mitigate this; the bworkers adapter should use the pool, not a bare session.
4. `bun repo cargo --batch` CLI was not added (in-process API only).
5. The registry directory stamps invalidate on any other process that adds a crate to the shared Cargo home; that only causes an extra fetch, never a wrong skip.
