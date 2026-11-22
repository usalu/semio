# Cargo `synchronize` performance (fem workspaces on the Windows host)

Scope: why `repo cargo ... check` sits 10+ minutes in `cargo:build-entry` before rustc starts, 200-900 CPU-seconds per `synchronize` child.

## Findings (measured, `bun --cpu-prof` and an `lstatSync` meter, small manifest `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/Cargo.toml`)

1. **Physical custody check was O(depth) per operation, O(depth^2) per manifest read.** `directory()` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` re-`lstat`ed every ancestor from the repo root before *every* `list`/`state`/`presence`/`text` operation, and the `text` operation additionally re-ran the whole `physicalPlan` (one more `state` op per path part, each with its own ancestry walk). The framework workspace has about 3 000 crates and 10 000 directories under emoji paths of depth 10-14: one membership walk cost 667 000 `lstat` calls (47 s at about 70 us each on this host) plus 29 000 `readdir`. `lstatSync` was 65 % of all CPU, `readdirSync` 17 %.
2. **The membership walk of every owner ran 8-9 times per synchronize.** The framework owner is walked by: `prepare` (twice, because `refresh()` re-inventories unconditionally at the top of the closure loop), `publishSelectedCargoWorkspaceMembershipV1` (for each *foreign* owner, including disjoint ones), the `members` roster, and then `current()` which re-walked every owner's roster again after the membership, after tool identity, after update and after the locked fetch.
3. **Publication walked workspaces that cannot matter.** `publishMembership` collected members of every known foreign owner, although only members that sit *inside* the published owner's directory survive the filter; a workspace disjoint from the owner (the 3 000-crate framework when publishing the fem workspace) can never contribute.
4. **Serialization amplifies it.** `withPreparedCargoDependencyPairV1` takes the exclusive `cargo-preparation:<root>` lease, so concurrent `repo cargo` runs queue behind each other: the 10-45 minute wall times are queueing behind a 5-50 minute old-code holder (CPU of the children was only 200-900 s over 30-50 minutes of wall time). The lease itself was left alone (it protects `Cargo.lock` and membership writes); the body it protects is now 3-7x shorter.
5. **MSVC compiler tree hashed in full at entry *and* exit of every compiler-store lifetime.** `withRepositoryWindowsCompilerV1` -> `verify()` -> `inspectPortableMsvcTreeV1` reads and SHA-256s all 4 947 files / 1.5 GB (measured 8-19 s wall, 15-16 CPU-s per pass); every process that opens the store does it twice (the `bun --eval` runtime-entry hosts show 2.9 GB read each in `Win32_Process`). Not the reason for the sync child's 10 minutes, but a constant 30 CPU-s tax per process that adds to the host load.
6. Not a cause: source admission of the 126 build packages (about 1 minute, 20 MB), runtime-entry capture (55 files, 3 s), `cargo fetch/update` themselves.
7. Environmental, not code: the host disk dropped to 4-7 GiB free several times, which makes `repo` free caches ("Partition C: has ... free, below the 16 GiB this command needs") and stalls `door.capacity` (measured 600 s in one run).

## Taxonomy question: `./🧪️tests/🔬️script-commands/🟦️.ts` holding `BoundedSynchronizeScript`

It is a taxonomy smell but a repo-wide convention, not a local bug: 22 directories named `🔬️script-commands` exist under `🧪️tests` folders (cargo, nx, authorizers, bootstrap, print, value, ...), all imported by the product `📜️script.ts` entries. The production command classes should live beside their `📜️script.ts` (for example `📜️commands/`), not under `🧪️tests`. It has no performance impact; moving 22 trees across concurrently edited modules is a separate ticket. Not changed here.

## Fixes (root cause, no compatibility layers)

| File | Change |
| --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` | `directory()` verifies the directory itself at every operation but its ancestors once per pass (`verified` set owned by each `drain`/`controlled` call): O(N) metadata calls per walk instead of O(N x depth). `operate("text")` no longer replays `physicalPlan`; it checks the file with one `lstat`. Same error messages. |
| same | `preparationPlan.refresh()` only re-inventories after a recipe actually ran (`stale` flag); it re-walked every owner at the top of every closure iteration even when nothing could have changed. |
| same | `CargoPreparationResultV1` carries the per-owner member `inventories` the plan already computed, so callers do not walk the owner again. |
| same | `publishMembership` ignores foreign owners that are not ancestors of the published owner (their members cannot lie inside it). |
| `🛠️preparation/🟦️.ts` (cargo) | roster comes from `prepared.inventories`; `current()` proves the rosters unchanged by replaying the recorded physical observations (`cargoPhysicalObservationsHoldV1`, the same mechanism the receipt witness uses) instead of walking every owner four more times. A mid-run manifest write poisons the observation set and replans (bounded by the existing replan policy) instead of silently passing. |
| `🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/📦️inventory/🔎️verify/🟦️.ts` | `inspectPortableMsvcTreeV1` accepts `{contents:false}`: membership plus identity stamps only. |
| `🧰️framework/🛍️products/🦑️repo/.../🏗️compiler/🛠️native-compiler/🪟️windows/🟦️.ts` | first verification of a compiler-store lifetime hashes all bytes; later rechecks (including the one at exit) compare membership and file identity stamps against that pass. |

## Tests

* New, language-agnostic law `custodyWork` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🧫️fixtures/🔣️.json`, executed by `🧪️tests/🟦️.ts` ("membership discovery meters physical custody per member linearly in directory depth"): a child process with a preloaded `lstatSync` meter (`🧪️tests/📏️custody-work/🪝️preload.ts`, `📜️script.ts`) walks trees of depth 6 and 18 and the per-member metadata calls may grow by less than 3x (linear would be 3x minus constants, the old code grew 4.5x: 243 -> 1 089 calls per member). The member list is compared with `fast-glob` as the independent third-party oracle. Passes (10.8 s); fails with the old `directory()`.
* Extended `🔎️verify/🧪️tests/🟦️.ts`: with `{contents:false}` a tree whose inventory carries a wrong hash is accepted (proves no byte is read) and its identity equals the hashed pass; a rewritten file changes the identity. 9/9 pass.
* Existing cargo workspace tests (`🧪️tests/🟦️.ts`, `🪆️nested-owners`, `📦️identity`, run natively with the harness environment variables because the repo run queue was saturated): same pass set as before the change; the 12 failures are harness-only (`Actual process parent envelope required`, live-store refusal) or pre-existing (`batch owner publication discovers one fresh repository inventory` fails identically on `HEAD`: 16 root parses vs. limit 4). The harness-bound tests (`queued`, `original-control`, `preparation`) need `repo run @semio-tech/cargo-workspaces:<target>` and could not be run to completion while the host was queueing and the disk was at 4-7 GiB free.

## Speedup (same host, same load, no recipes, no `cargo update/fetch`, material manifest)

Harness `🧪️cargo-sync-perf/flow-time.ts` replays the CPU part of `withPreparedCargoDependencyPairV1` (scope, prepare, publish check, members, four `current()` checks) against `HEAD` and the working tree:

| Phase | HEAD wall / CPU | now wall / CPU |
| --- | --- | --- |
| prepare | 48.8 s / 24.6 s | 17.8 s / 6.2 s |
| publish check | 126.5 s / 36.3 s | 18.6 s / 8.8 s |
| members | 38.1 s / 7.5 s | 0 (reuses prepare inventories) |
| 4 x current() | 116.0 s / 34.6 s | about 10 s / 3 s (observation replay) |
| **total** | **329 s / 103 CPU-s** | **about 46 s / 18 CPU-s** (7x wall, 5.7x CPU) |

A real `repo cargo --store=shared --configuration=prototype -- check` on the material crate could not be timed end to end: it queues on the exclusive preparation lease behind the old-code `synchronize` children started at 01:17 and 01:51 (still running at 02:24 with 380-480 CPU-s), and the first attempt was killed externally after 1 446 s while still queued. See the last section for the final attempt.

## Still open

* The exclusive `cargo-preparation:<root>` lease serializes all concurrent runs repo-wide; a per-owner-closure lease (plus a shared lease for read-only receipt hits) would let unrelated fem workspaces prepare in parallel. Needs a design decision on `Cargo.lock`/membership write ownership.
* Publication still walks ancestor workspaces of the published owner (the whole `✏️s` tree, about 18 s here) to learn which of their members sit inside the owner; a prefix-bounded member walk would make it proportional to the owner.
* One framework membership walk remains in `prepare` (about 6 CPU-s): a persistent per-directory listing cache keyed by directory stat would remove it across processes.
* `🔬️script-commands` under `🧪️tests` (22 trees), see above.
* Free disk below 16 GiB stalls `repo` in the cache collectors; the cache budget on `C:` is exhausted.

Scratch inputs (scripts, no outputs) are in `🧪️cargo-sync-perf/`: `prepare-time.ts`, `flow-time.ts`, `capture-time.ts`, `msvc-time.ts`, `deep-tree.ts`, `patch*.ts`, `aggregate.py`.

## Real `repo cargo check` attempts

Three end-to-end attempts on the material crate (02:00, 02:33) never reached the `synchronize` child: the first queued 24 minutes on the preparation lease behind old-code children and was killed externally; the last spent 15 minutes in `build:source-admission` / `door.capacity` on the saturated host (16 jobs booked, disk at 4 GiB) and was stopped by me. The end-to-end number therefore still has to be taken once the old-code `synchronize` children are gone and the host is quiet: run `SEMIO_REPO_PROFILE=1 SEMIO_CARGO_PREPARATION_TIMING=1 repo cargo --store=shared --configuration=prototype -- check` in `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material` and read the `door.script` row of the profile table. Expected: the synchronize body drops from 5-50 minutes to well under a minute plus the fixed `cargo update`/`fetch`.


## Round 2: shared preparation, scoped publish, listing replay

Items from the coordinator: (1) exclusive lease serializes runs, (2) publish walks all of `✏️s`, (3) remaining framework walk in prepare.

| Item | Change | Files |
| --- | --- | --- |
| 1 | `withPreparedCargoDependencyPairV1` now first confirms a *current receipt* under a **shared** `cargo-preparation:<root>` lease (`preparedPair(...,"shared")`); membership is only checked, never written, and no recipe runs. Anything that needs work (recipe, stale membership, receipt miss or mismatch, i.e. `update`/`fetch`) throws `CargoPreparationExclusiveV1`, the shared lease is released and the same pass is redone under the exclusive lease exactly as before. No receipt file at all goes straight to exclusive. Concurrent runs with a current receipt therefore overlap; writers still serialize. | `🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts` |
| 2 | `cargoWorkspaceMembers(root, owner, observations?, within?)`: `within` restricts the walk to the directories on the path to, or below, one directory and tolerates an empty result. `publishMembership` passes the published owner's directory and now also skips non-ancestor foreign owners, so the `✏️s` ancestor walk only visits the path down to the fem workspace. Measured publish check on the material manifest: 126 s (HEAD) -> 18.6 s (round 1) -> **0.04 s**. | `🗂️workspaces/🦀️cargo/🟦️.ts` |
| 3 | Settled directory listings are persisted per checkout (`<storage>/<sha>-listings.json`, rows `[path, stat, entries]`, written after each confirmed pass, bounded by the 16 MiB protocol limit) and seed the next pass's observation set. A listing whose lstat still equals the recorded settled stat (older than the 2 s racy window when recorded) is answered from the seed: no `readdir`, ancestors still verified as non-symlink directories, the `Cargo.toml` entry still re-`lstat`ed because recorded names do not distinguish file from directory. Warm prepare on the material manifest: 63 `readdir` calls instead of about 19 500. | `🛠️preparation/👁️witness/🟦️.ts`, `🗂️workspaces/🦀️cargo/🟦️.ts` (`replayListing`, `directory(..., fresh)`), `prepareCargoOwners` takes optional observations |

Flow replay (same harness as above, material manifest, quiet host): HEAD 329 s / 103 CPU-s; round 1 about 46 s / 18 CPU-s; now **about 11 s / 8 CPU-s** (prepare 5.7-6.7 s from 19 s, publish check 0.04 s, members 0, four `current()` replays about 4 s). The remaining cost is `lstat` of manifests and their ancestry (about 130 k calls, 7 s) in `state`/`presence` operations.

Tests added (law style, fixtures in `🧪️tests/🧫️fixtures/🔣️.json`):
* `scopeWork`: scoped walk equals the full walk filtered to the scope and `fast-glob`; metered `lstat+readdir` grows by less than 1.5x while the unrelated sibling count grows 6x (20 -> 120). Passes. (A directory named `target` is opaque to member discovery, so the fixture uses `focus`.)
* `replayWork`: a seeded pass (snapshot written and read back through the real functions, child process with a filesystem meter) yields the same members as the cold walk and `fast-glob` with 0 `readdir` calls against more than 6 cold ones. Passes.
* `🧪️tests/📥️original-control/🟦️.ts`: "concurrent runs whose receipt is current share one preparation lease and never run a recipe": a cold run leaves a receipt; two concurrent runs must both reach the tool-identity step before either finishes (rendezvous with a 5 s guard; a serialized lease deadlocks into the guard), no `update`/`fetch` is run; an exclusive peer holder blocks a third run until released. This file is harness-bound (`withBuildTestInvocationV1`); see the run status below.

Round 2 test status (native `bun test`, artifacts and scratch on `G:\claude-scratch`, which I could not delete: the safety check refused the `rm -rf`, so it is left for you to remove): `custodyWork`, `scopeWork`, `replayWork` and the extended MSVC verify test pass; the rest of the cargo workspace files keep the round 1 failure set (harness-only or pre-existing). The witness test "a launcher skips the Cargo preparation child ..." also fails natively because its independent Python oracle compares `st_dev`, which Python reports as 17008080259063917242 where Node reports 148286138 for the same file on this host, so its stat shortcut can never agree; it does not depend on the changed code (not re-run against `HEAD`). The new shared-lease concurrency test parses but is harness-bound like its neighbours, and `repo run @semio-tech/cargo-workspaces:original-control-contract-check` could not run: the resident graph service failed with `Closed host capacity Policy required: /swap/activeBasisPointsPerSecond is required` (another agent's in-flight policy change). It has not been run and must be run once that is fixed.

## Round 3: harness blockers

* `Closed host capacity Policy required: /swap/activeBasisPointsPerSecond is required`: the host-capacity schema (`🏗️build/🌡️host-capacity/🧬️schema/🔣️.json`) had been changed to require `swap.activeBasisPointsPerSecond/outBasisPointsPerSecond/shortBasisPoints` while `🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json` still carried the old `activeMegabytesPerSecond/outMegabytesPerSecond`. By the time I looked, another agent had migrated the policy (`swap` now 0.6 / 40 / 2500 bps, plus the new deadline fields), so no edit was needed from me and the error is gone. Two further in-flight breaks of the resident graph service followed from other agents' half-landed work (`📥️inference/📜️contract-reads` imported before it existed, resolved after about 2 minutes; `👁️observation/🗄️analysis-store` imported by `👁️observation/🟨️.mjs`). I did not touch them.
* Witness-launcher oracle (`🛠️preparation/👁️witness/🧪️tests/🐍️.py`): confirmed the cause is host-specific, independent of the changed code. On this host Python (3.13 and 3.14) reports `st_dev` 17008080259063917242 where Node reports 148286138, and on Windows Python's `st_ctime_ns` is the *creation* time while Node's `ctimeNs` is the change time. The TS stat strings contain both, so the oracle's stat shortcuts and tool-stamp comparison could never agree here (TS and oracle formats are untouched; at `HEAD` the same oracle code has the same dependence on `st_dev`/`st_ctime_ns`, so it fails there on Windows too). The oracle now compares only the identity both runtimes agree on (inode, size, modification time; kind for tools), documented in the file. The test passes natively (88 expects). The TS code is unchanged: it keeps dev and ctime, which is the stronger identity within one runtime.

Run status of the shared-lease concurrency test: not run. After the two foreign breaks above cleared, `repo run @semio-tech/cargo-workspaces:original-control-contract-check` spent 1 400+ s in "Reading the project graph slice" (still at "reading the owners 533 of 3600" when I stopped it, just before my own 1 700 s timeout), i.e. the cold graph load on this host (disk at 3-4 GiB free, many agents) did not leave time for the test itself. The test is committed to the tree and parses; run it with the same command once a warm graph service exists. The test and the three round-2 law tests do not depend on the oracle fix.
