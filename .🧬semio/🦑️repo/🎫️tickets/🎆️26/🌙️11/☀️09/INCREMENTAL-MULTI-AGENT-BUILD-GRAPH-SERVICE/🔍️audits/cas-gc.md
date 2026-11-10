# Lane cas-gc: host-wide size-capped CAS with LRU GC

`BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. New module `BUILD/🗄️cas`.

## What exists now

| Part | File | Role |
|---|---|---|
| Policy schema `CasPolicyV1` + EN/DE texts | `🗄️cas/🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts` | cap 120 GiB, high 0.9, low 0.75, minAge 10 min, lease max 1 h, areas `graph-memo` (4 GiB, 7 d) and `build-input-closures` (8 GiB, 7 d) |
| Store `CasStoreV1` | `🗃️store/🟦️.ts` | `put(stream)`, `get`, `read`, `has`, `link` (reflink, copy, optional hardlink), `touch`, `adopt(existing file)`, `pin/release`; root `SEMIO_CAS_ROOT`, else `%LOCALAPPDATA%\semio\cas` / `~/.cache/semio/cas`; atomic tmp+rename, objects read-only, identical concurrent puts keep one copy |
| Access ledger | `📒️ledger/🟦️.ts` | sqlite-free append-only segments per shard and per writer (no contention), torn lines ignored, throttled by `touchGranularityMs`, compacted by GC; live tail view for race checks |
| Pins and leases | `📌️pins/🟦️.ts` | one JSON per holder, TTL capped, dead pid drops the pin, re-read while GC runs |
| Planner | `🧹️collector/🧮️plan.ts` | streaming, 20 bytes per candidate entry, no strings; stages: area age, area cap, host cap; oldest `lastAccess` first, ties by scan order; protected entries (pinned, too young, hardlinked) count in totals but are never chosen |
| Collector | `🧹️collector/🟦️.ts` | two passes shard by shard, single-flight lock with stale takeover, progress, `AbortSignal` cancellation (partial result, store consistent), dry run, per-eviction re-check of pins, fresh access and mtime, EBUSY/EPERM counted as busy, temp sweep, ledger compaction |
| Command | `🗄️cas/⌨️command/🟦️.ts`, hook in `🧹️cache-gc/⌨️command/🟦️.ts` | `repo cache cas status|gc [--dry-run] [--json]`; `cache gc` without `--area` also runs the host policy over store, every worktree's graph memo and the workspace's build-input closures, counting the size ledger's recorded bytes as external (fixed images) |

Graph-memo and closure records are policy areas: files of a root are aged and LRU-cut like store objects and their bytes count against the host cap. A record whose plan file is evicted is a plain memo miss (the memo reader returns undefined for a missing file) and is re-planned.

## Verified

- `LANG`-independent: `repo --service=off tools exec --light bun -- test ./<BUILD>/🗄️cas/🧪️tests/🟦️.ts` : 13 pass, 0 fail (run three times).
- Authored traces `🧪️tests/📋️traces/🔣️.json` (5 cases) equal the expected victims in TypeScript and in the independent Python oracle `🧪️tests/🐍️oracle.py` (full-sort LRU simulation); 400 seeded random traces equal the oracle, all three reasons (age, area, host) occur in more than 150 evicting cases.
- Two workspaces writing 12 identical blobs concurrently: exactly 12 objects, bytes <= 1.1x one copy, no temp leftovers.
- Collection down to the low watermark, pinned objects kept, oldest evicted exactly in order; expired lease no longer protects; dry run removes nothing.
- Race: an object read between plan and removal survives (`skipped`), a pin made during the run protects at once, a second collector is refused (`locked`).
- Cancellation leaves whole objects only and the next run finishes. Crash: a child collector SIGKILLed mid-eviction leaves every remaining object hash-valid, the stale lock is taken over, the next run reaches the low watermark.
- Areas: aged closure and memo files removed, area cap enforced oldest-first, host cap with external bytes, hidden/`.tmp` files and unrelated data untouched.
- Scale probe (scratchpad bench, not committed): 100,000 objects scanned and 62,500 evicted in 7.5 s, RSS growth 45 MB.
- Type check: `tsc --noEmit --strict --noUncheckedIndexedAccess` over the module sources: clean.
- Existing `🧹️cache-gc` tests (budget, launch, future, quota, directory, units, sweep rule, verify) still pass with `SEMIO_LANGUAGE=en`; the failures `sweep: every heavy command checks the sweep ...` and 5 `dedupe` tests are independent of this lane (see requests).

## Safety

- Only files below the declared CAS root and the declared area directories are touched; symlinks are not followed; names starting with `.` or ending in `.tmp` in areas are skipped; the host-store step of `cache gc` is skipped when the environment declares no `HOME`/`USERPROFILE` (hermetic tests never reach the real home).
- POSIX unlink of an open file keeps running readers valid; on Windows the open file refuses removal and is counted busy and kept.

## Not done / follow-ups

- `BUILD/🌐️remote-cache` still keeps its own per-directory LRU; making it an REAPI front of the host store needs a store API that tolerates external deletion (its in-memory index would otherwise go stale). Left as is; the adapter is a small wrapper over `CasStoreV1.has/put/read` once the daemon lane needs it.
- Catalog entry for `cache cas` and the `repo cache status` area line need the integration lane (requests filed). `cache cas status` already prints host store totals.
- The 7 days / 4 GiB / 8 GiB area values are initial defaults; the bench lane should confirm them against real traces.
- Cross-workspace dedup of already existing workspace trees is available through `store.adopt` (clone into the store) plus the existing `👯️dedupe`; mass-adoption of the 49 GB `.cache/b` is not run automatically (tool-owned images stay as they are and only count as external bytes).

## Round 2 (coordinator follow-up)

**Host store as the only object store**
- Unit-cache objects (the 20 GB `b/r/m/u` share) now live in the host CAS: new sync port `🗄️cas/🗃️store/🔁️sync`, unit-cache `💾️store` publishes, restores and checks through it (restored files get normal modes, the object stays read-only). Existing image objects are only drained by the unit-cache GC (natural GC, no migration). Contract tests run against the host root per scenario; the Python auditor reads the host object directory.
- Remote cache: `CasStoreV1.putChecked` (digest/length/size checked before publication), `RemoteCacheStoreV1` option `host`: `cas` blobs are stored once per host across all cache directories; `has` is now asynchronous and verifies the file, so an object removed by the host collector or by hand is a plain miss (also for `ac`/`nx`, and `put` no longer answers `exists` for a stale index). `repo remote-cache serve` fronts the host store.
- Not possible from cas-gc: dedupe of the loop-image partitions (`b/r,d,t,n,g,o`, different device) and removal of their growth path; requests filed with the exact contract for the cache owners.

**Tests fixed at the root (no deletions)**
- cache-gc: verify and dedupe command tests got a language in the environment port; dedupe self-inflicted access-time bumps fixed in the code (census hashing opens with `O_NOATIME`, replacement re-check uses mtime) and in the harness (oracle stats before hashing, fixtures keep times); sqlite lease probe restores file times; ledger fixtures gained the required `rust` root; the native directory partition fixture uses `allocateNativeDirectoryV1`; the sweep test reads the moved task runner file.
- policy-literals: `host-capacity` gate timeouts use the declared `commandTimeoutMilliseconds`; EN/DE descriptions added for the `generations` and `generation-claims` policy keys.
- Verified: 16 test files, 200 pass, 0 fail (`cas`, `remote-cache`, `policy-literals`, all `cache-gc`, unit-cache `store` contract, `gc`, `budget`); unit-cache main file: 24 pass with the toolchain environment forwarded, 2 fail only because this host has no C linker (`cc`).

## Round 3: owners, device decision, growth path

**Device decision (measured, then chosen).** The "images" under `.cache/b/<x>` are no loop devices any more: they are directory partitions (`m`/`v` swap, size meter, anchor `image.json`) on the same filesystem as the home cache (`stat` device 66312 for `b/{r,d,t,n}/m` and `~/.cache/semio`). So hard links work between the host store and every workspace partition on the same volume, and no host-level image or partition is needed. Chosen option: shareable content lives once in the host store (hard-linked into the places that use it, reflink/copy where the volume differs), the partitions keep workspace-local mutable state. Why it bounds disk best within 200 GB: a second mount (host image) would add a fixed envelope that cannot shrink and would split the budget in two; the single volume keeps one cap (120 GiB store, memo and closure areas inside it) and hard links make equal content cost one inode across all workspaces. Cross-volume workspaces are detected (`EXDEV`/device compare) and fall back to copy and skip adoption.

**Owners**
- Cargo unit cache: objects only in the host store (round 2).
- Dependency installs (`b/d/m/j`), bun cache, tool installs (`b/t/m`) and the Bazel disk cache: `adoptCasV1` (`🗄️cas/🧲️adopt`) makes finished files (quiet 10 min, from 64 KiB) one hard-linked host object per content; the first file of a content joins the store with its write bits cleared, later equal files (same size and mode) are replaced atomically after a changed-since-hash check. Run by `repo cache gc` (host step) and `repo cache cas adopt [--dry-run]`; cancellable, progress, hashing without touching access times. Objects still linked by a workspace count as in use for the store collector and become evictable when the last workspace link disappears.
- Nx task cache: restoring copies output files with their modes, so read-only adoption would break rebuilds. Instead task entries written through the remote cache `nx` keyspace store the archive once in the host CAS (`RemoteCacheStoreV1`, key to hash reference file), collected objects are misses. The local Nx directory cache (`b/n`, ~0.8 GB) is left alone.
- Python census (`hashlib`, `os.stat`) validates the inode grouping in the tests.

**Growth path removed.** `cache grow` (module, command, catalog entry, flags, help snapshot, error text) and the unit-cache GC object code (plan, apply, schema, shares, messages, tests, oracles) are deleted without a compat layer. Existing bytes drain: `pruneEntriesV1` removes the pre-host object directory of an image once nothing replays it.

**Verified (light)**: see the final run recorded in the hand-back message; pre-existing failures that remain: `📏️needs` stale 48 GiB expectation, two unit-cache build-script tests (no `cc` linker on this host).

## Round 4: adoption safety, real dry run, Nx default route, needs row

**Adoption is restricted to what cannot be written in place.** Places carry a mode and a reason in `CasAdoptionPolicyV1`: hard links only for stores that are immutable by construction (bun content cache, pinned tool download archives, the Bazel disk-cache CAS directory; the cargo unit objects are written to the store and restored by copy, never linked). Installed trees (dependency installs, tool installs) are `reflink` places: they only ever get copy-on-write clones, and a volume that cannot clone (this ext4 host) skips them and reports `clone`. The `node_modules` trees are therefore never hard-linked. Hard-linked files lose their write bits, so an in-place writer fails with EACCES instead of changing every sharer. Test `a write to an adopted file never reaches another workspace` covers: write to a hard-linked file throws, the other workspace keeps its bytes, a mutable place is never hard-linked, a write to a file in it affects nobody else.

**Real dry run (`repo cache cas adopt --dry-run --json`, nothing changed, ~100 s, 1.46 M files examined)**

| area | mode | files that would join | new objects | bytes saved | left alone |
|---|---|---|---|---|---|
| bun-cache | hardlink | 1668 | 1482 | 132,685,170 (126.5 MiB) | small 69,967; volatile 173 |
| tool-downloads | hardlink | 168 | 166 | 170,693 | small 9,727; volatile 59 |
| bazel-cas | hardlink | 0 | 0 | 0 (no Bazel store in this workspace) | none |
| dependency-installs (`b/d/m/j`) | reflink | 0 | 0 | 0 | no copy-on-write on this volume: 9,829 files; small 393,527 |
| tool-installs (`b/t/m`) | reflink | 0 | 0 | 0 | no copy-on-write: 2,457; already shared by other links 14,664; small 948,806 |
| total | | 1836 | 1648 | 132,855,863 (126.7 MiB) | |

Honest reading: within this single workspace only the bun cache has duplicates worth sharing (about 127 MiB); the real gain appears when further workspaces on the same volume run the same adoption (equal bun-cache files and archives collapse to one inode each). The big installed trees (14 GB dependency installs, tool installs) save nothing on this ext4 volume because they are reflink-only by the safety rule; on btrfs, XFS with reflink, APFS or ReFS they would be cloned.

**Nx through the host store by default.** `nxRemoteCacheEnvironmentV1` (`🌐️remote-cache/🧶️nx`) is applied in the Nx process environment of `runOwnedNxInvocationV1` next to the existing declared-contract route: when no contract is declared and the local remote-cache server of the policy (127.0.0.1:6090, started with `repo remote-cache serve`, blobs land in the host store) answers `/healthz` within 300 ms, Nx gets `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` and the token; with no server answering, or `SEMIO_REMOTE_CACHE_NX=off`, Nx stays local. The server is not auto-started: starting a long-lived network daemon silently from every Nx call is a host-posture decision I did not take. Proof (test `a real Nx task ...`): a real Nx 24.0.4 (from the bun cache, run with the installation's modules on `NODE_PATH` because the installed Nx is patched to require the repo's script process) ran a real `nx:run-commands` target in workspace one; workspace two with its own empty cache restored it ("remote cache" in its output, output file present) and the host store holds exactly one object. Note for the next reader: `repo --service=off` is rejected by the current CLI ("Unknown repo command"), use plain `repo`.

**`cache cas` reachable.** The ledger-kernel selection (`⌨️command/🧭️selection`, `💾️cache`, `💾️cache/📋️ledger`) listed `grow` where `cas` belongs; fixed, so `repo cache cas status|adopt|gc` runs end to end (the dry run above went through it). `adopt` now reports per place.

**`📏️needs` row re-measured (2026-11-09).** cargo-home 1,420,360,629 B, nx-artifacts 60,749,977 B, unit-cache 20,892,484,717 B (still mostly the pre-host objects, draining), compiler-workspaces 3.9 GB kept. The required size from the area caps is 51,217,485,005 B (47.7 GiB); the declared rust partition is 96 GiB + 1 MiB. The old assertions (48 GiB, "tight" within 1 GiB, partitions filling the machine budget within 1 GiB) predate the doubling of the partition and the budget re-split; they now assert the real relation: fits, the exact slack, 96 GiB + 1 MiB, and partitions at most the machine budget. The slack (about 45 GiB of the partition, 18 GiB of the machine budget unallocated) is a sizing decision for the owner of the budget policy; I did not shrink live partitions.

Verified in this round: 19 test files, 239 pass, 0 fail.

## Round 5: the endpoint lives in the graph service; tsc fix

- `startHostedRemoteCacheV1` (`🌐️remote-cache/🏠️hosted`) is called around the foreground of `repo service host` (`⌨️command/🌐️host`): the remote cache endpoint starts with the one per-host service and stops with its signal; a taken port or directory is "busy", never a failure. Observed on the real host: `ss` shows the service's child process (started by `repo service host run`) listening on 127.0.0.1:6090 and answering `/healthz`, with its cache directory below the host store root. Test: the hosted endpoint starts, answers, stops with the service signal and a second host sees the port as taken.
- The `tsc --strict` error in `PROC/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` (line 269, a string where the notice wants a language function) is fixed; the files I touched type-check clean.
- Real run of the repo's own Nx path: NOT completed. `repo run repo-task-scope:test` started the service and then waited in planning for more than 40 minutes on a host with load 9-13 and heavy swap, behind other agents' plans, while the supervisor restarted the service. I did not stop it (the sandbox refused the process kill); its log is `🗑️generated/cas-gc/nx-run1.txt` and should be deleted when it has ended. So the Nx environment step in `runOwnedNxInvocationV1` has not been exercised end to end by the repo itself; proven are (a) the endpoint is live inside the service on the real host, (b) the same environment function drives a real Nx 24.0.4 that restores a task in a second empty workspace from the host-store-backed cache (round 4 test). To close it: when the host is quiet, run `repo run repo-task-scope:test`, delete its entry in `.cache/b/n/m`, run it again and look for "remote cache" in Nx's output.
