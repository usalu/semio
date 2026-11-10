# Lane derived-dag: fine-grained derived-fact DAG with early cutoff

Status: engine, node kinds, durable store, schema, oracle and tests are done and green. The cut-over of the planner (replacing `settleSuspectsV1` and the coarse command-walk closures) is not done; it needs `🕸️graph/🟦️.ts` and `inference/script-inputs`, which belong to `integration` (request filed in `📌️requests.md`).

## What was built (`BUILD/🕸️graph/🧮️derived-nodes`)
| Path | Content |
|---|---|
| `🧬️schema/🔣️.json` | Policy const, `NodeKey`, `RowRef`, `DirtySet`, `DerivedNode`, scenario fixture schema |
| `🟨️.mjs`, `🟨️.d.mts` | `createDerivedGraphV1`: red-green pull-recompute, push-invalidate by depth with early cutoff, single-flight per key, cycle refusal, cancellation (`AbortSignal`), progress, bounded node count (LRU, evicts with every node above), per-operation raw read cache, world reads limited by `concurrency` |
| `🧩️kinds` | Rules for `manifest` (package.json, project.json, Cargo.toml via own restricted TOML reader, go.mod: parsed dependency set only), `project`, `named-input` (digest of glob roster and file digests), `target-inputs`, `action-key`, `closure` (digest of member digests); glob matcher |
| `💾️store` | Content-addressed brotli record per node, atomic writes, `prune` (max records, retention), records are hints and always revalidated |
| `🌍️world` | Disk world port (cancellable listing) |
| `🧪️tests` | `🔣️.json` (20 steps over 4 projects: package.json, Cargo.toml, go.mod, emoji-named project file, astral path order), `🐍️oracle/🐍️.py` (naive recompute-everything model, full `tomllib`/`json`), `🟦️.ts` |

Dirty handling: nodes record file digests and glob roster digests as raw reads; `invalidate({paths})` seeds only nodes whose reads cover a dirty path, rereads only those reads, then processes pending nodes by depth; a recomputed node with an equal value digest marks nothing above it. `overflow` or a store-loaded node rereads all its raw reads.

## Verification
`repo --service=off tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧪️tests/🟦️.ts` -> 8 pass, 0 fail, 496 expects.
- Fixture scenario and 8 seeded random edit sequences (96 steps): values equal the Python model, and the exact recomputed set and changed set equal the model's minimal sets.
- Pinned acceptance rows: whitespace edit of project.json recomputes exactly 1 manifest node, reads 1 file, 0 closures; comment edit of go.mod, reordered package.json, unrelated file, excluded test file, touch-only recompute 0 or 1 manifest node and no closure; one dependency added recomputes manifest, project, 2 named inputs, 2 target-inputs, 1 closure (member) and nothing else; a source body edit recomputes named-input, 2 target-inputs, 2 action-keys, 1 closure.
- Independent third-party checks: glob matcher equals `Bun.Glob`, restricted TOML reader equals `Bun.TOML` on the Cargo tables.
- Eviction (cap 7 nodes) stays equal to the model; the durable store lets a new graph answer all requests with 0 rules run and recompute fewer nodes after one edit; concurrent gets run a rule once; abort rejects; cycles are refused.

## Not done / remaining
- The 24 `📏️incremental` scenarios and the 5.84M -> 60k rows-per-no-op-plan measurement need the cut-over (inference closures expressed as nodes), so that metric is not measured yet. `stats().rawReads` and `cutoffHit` are the counters for bench.
- Deleting `✂️cutoff` (`settleSuspectsV1`, `💾️durable`) and `closure-verify-helper.mjs`, and migrating their fixtures, is deferred to the cut-over because `🕸️graph/🕰️generations/{🔀️merge,🔌️load}`, `🎯️test-owners` and `🧭️invocation/🕸️graph/🧠️memo` still consume them; removing them now would break the planner.
- EN/DE texts: the module has no user-facing strings yet (errors are internal codes).

## Real monorepo measurement (2026-11-09, loaded shared host, other agents editing concurrently)

Harness: `🧰️scripts/derived-dag-real.mjs <out.json>` (run with `repo --service=off tools exec --light bun -- run`). It walks the real repo (skipping `.git node_modules target .cache .venv .🧬semio .r bazel-*`), builds `manifest` nodes for every `Cargo.toml` plus the manifests of every project, `project` nodes for every `*project.json` (root project skipped), then per project every target's `action-key` (so every real `namedInputs` entry, `{workspaceRoot}/` and `{projectRoot}/` globs, named-input references) and one `closure` per project. Disk world; one process; 16 concurrent reads.

Scale: 4580 `Cargo.toml`, 1561 project files (1560 projects), 6432 manifests, 0 unparseable, 17,610 nodes, 311k raw reads (files and glob rosters).

| measurement | derived DAG (this lane) | old closures (bench.md) |
|---|---|---|
| cold build, everything | 40-46 s wall, 311k raw reads, 17.6k nodes, RSS peak 2.2-2.5 GB (1.6 GB resident after) | median publish 156 s, p90 386 s, a single wide refresh up to 1117-1596 s; 3.0 GB planner, up to 10.8 GB OOM kills |
| warm no-op, journal says nothing dirty | 36-44 ms, **0 raw reads**, 0 rules | re-verifies 1-1052 closures (18.5k rows each), minutes |
| warm no-op without journal (`overflow`, rereads every raw read) | 33-48 s, 304k raw reads, RSS peak 3.1-4.1 GB | the stat-pass equivalent; no better than today |
| one-file edit, requested `...🪡️field/📦️packages/🦀️rust/Cargo.toml`, comment line | 10.7 s (26 s before the listing cache), **268 raw reads**, 1 manifest recomputed + 16 named-input + 197 target-inputs + 197 action-key + 28 closure (these hash the Cargo.toml bytes as input, so they truly change) | seconds to minutes: shared-manifest edits stale hundreds of closures |
| same, dependency added / restored | 10-12 s, 268-270 raw reads, same node set | same |
| edit of a project's own Cargo.toml (comment) | 7-8.5 s, 127 raw reads, 1 manifest, 3 named-input, 13 target-inputs, 13 action-key, 6 closure | - |
| same, dependency added | 9.5 s, 127 raw reads, additionally 1 project node (value changed) | - |

Reading it: raw reads per edit fell from about 169,000 to 268 (and 127), a factor of 630; rows per no-op plan are 0 against millions. The remaining 7-12 s per edit is not disk reads: it is CPU for recomputing about 440 nodes (named-input nodes over workspace-wide globs re-hash 10-20k pairs each) plus the post-edit `getMany` over all 17.6k nodes. The "changed" counts include genuine concurrent edits by other agents, because named inputs include `{workspaceRoot}/framework/**` trees. The target "no-op re-plan after a one-file edit <= 5 s" is therefore met for the no-op and for edits whose nodes are not workspace-wide inputs, and not yet for the shared manifest that 16 workspace-wide named inputs include.

Engine changes made because of the real run (all tests still green): (1) file bytes are no longer cached per operation (the first run reached 5.2 GB RSS); only digests, plus bytes of files <= 16 KB up to 32 MB per operation; (2) `{workspaceRoot}/` and `{projectRoot}/` globs resolve to the longest literal directory prefix so a roster walks only its own subtree; negations are matched on the full path; (3) named inputs that name other named inputs become node edges; target `options.command` and non-string input entries (`{env}`) are handled; (4) a recomputed node reuses the digests of files that are not dirty (`ctx.digestOf`), the cause of the 630x raw read drop; (5) a bounded listing cache (2M paths) updated incrementally from `DirtySet` paths, which cut a recompute from 25 s to 8-10 s; (6) interned raw-read refs; (7) `getMany` with slices.

Next optimisation (not done): store per-directory digests (Merkle) so a named input recompute is O(changed path to root); lower the 3 GB peak of the no-journal overflow path by streaming the digest pairs; add `cargo`-aware `Cargo.toml` file membership so only crates that depend on the edited one are staled.

## Round 3: edit path under 1 s, observed reads, environment

Measured with the real-repo harness after the change (shared loaded host, one run):

| step | before | now |
|---|---|---|
| warm no-op, journal clean | 36-44 ms, 0 reads | 43 ms, 0 reads |
| edit of the requested `...🪡️field/📦️packages/🦀️rust/Cargo.toml` (comment / dep added / restored) | 10-12 s, 268 reads | **0.28-0.41 s, 1 raw read**, 439 nodes recomputed (1 manifest, 16 named-input patched, 197 target-inputs, 197 action-key, 28 closure) |
| edit of a project's own Cargo.toml | 7-9.5 s, 127 reads | **60-100 ms, 1 read**, 36-39 nodes |
| cold build | 40-46 s, peak 2.2-2.5 GB | 76 s on a busier host, 172k raw reads (was 311k), peak 2.6 GB, 0.9 GB after GC |
| no-journal fallback (`overflow`) | 33-48 s, peak 3.1-4.1 GB | 82 s, peak 3.9 GB (not met) |

What changed: (1) Merkle-equivalent for globs: a glob read keeps its members as two sorted arrays plus two additive multiset hashes (sum mod 2^256 of `sha256(canonical([path,digest]))` for contents, of `sha256(path)` for the roster). `ctx.globDelta` patches the previous answer from the dirty paths only (binary search + splice), so the cost of an edit is independent of the glob size; an unchanged glob is checked in O(dirty). Glob nodes subscribe to their directory bases, not to every member file. The oracle recomputes the hashes naively. (2) Action-key/target-input recompute needed no extra patching: after (1) their inputs are a handful of entries, and the 440 recomputed nodes take 0.3 s in total. (3) A worker pool was not needed (nothing CPU-heavy remains on the edit path) and is not built. (4) The fallback is streamed in slices of 256 reads and 16 nodes, but its peak is still 3.9 GB: live memory is 0.9 GB, the rest is uncollected garbage of the 17.6k-node rebuild. The 1.5 GB fallback target is open; the lever is the memory lane's governor (forced GC / lower `heavyConcurrency` / `verifyParallelNodes`) or a per-directory Merkle store that avoids re-walking.

Memory facts from the real run: 867 named-input nodes hold 1.27M glob members in total, of which 1.07M belong to 554 distinct sets, so sharing identical sets would save at most 16%; not done.

Hooks added for integration (see `📌️requests.md`): `ctx.observed({paths,digest,rows,verify})` (third raw read kind `observed`, seeded by its paths and their ancestor directories, `DirtySet.directories` for subtrees, `verifyObserved` graph option for reloaded records) and graph options `environment` (shallow-frozen, `ctx.environment`) and `environmentKey` (mixed into every key). Reference kind `directory-names`. Tests: 10 pass, 517 expects, Python oracle compares the fixture (now with listing requests) and 8 random sequences; the oracle's glob model, hashes and directory listing are independent re-implementations.

## Round 4: cold build, fallback peak, large durable records

A CPU profile of the cold build (`bun --cpu-prof`) showed the time was not I/O or concurrency: 46% was the glob membership test (`spec.has`, a closure plus `slice` per listed path) and about 20% was 2.5M sha256 calls. Fixes: one anchored regex per group (no slice, no closures), memoised per-path roster and entry hashes (166k unique paths behind 1.27M members), path concatenation instead of `join`. Heavy glob work is adaptive: limit = clamp(min(cpus/2, (available bytes - 2 GiB) / 256 MiB), 1, 8) from a `capacity()` grant (default `os`); `release()` (default `Bun.gc(true)`) runs at slice and batch boundaries only above 1 GiB RSS, at most every 2 s, and pauses for 60 s after three calls that freed under 5%.

Real repo, host load average 9.5 (not idle), one run:
| | before | now |
|---|---|---|
| cold build | 75-77 s, peak 2.7 GB | **38.8 s, peak 0.84 GB**, 172k raw reads |
| edit of the shared crate manifest | 0.28-0.47 s | **0.14-0.16 s**, 1 read |
| edit of a project's own manifest | 54-73 ms | 42-49 ms |
| journal-clean no-op | 41 ms | 40 ms |
| no-journal fallback | 82-92 s, peak 3.3-3.9 GB | 44.7 s, **process peak 1.17 GB** (target 1.5 GB met) |

The cold target (< 30 s on an idle host) is not proven: this host was loaded. The garbage that caused the old 3-4 GB peaks was mostly the per-path closures and string slices of the membership test, so the guarded release is rarely needed now (0 calls in the cold build). The per-directory durable digest store was not built: the fallback meets its memory target without it, and still rereads 166k files (44 s); a directory-digest store would bring it down to a stat pass and is the next step. Sorted arrays are already shared by reference with the node record (no per-node copies).

Large records: `createNodeStoreV1({root, blobs})` accepts a host CAS (`createCasStoreV1` from `🗄️cas`, used as is: `put`, `read`, `pin`). A record above `maximumRecordBytes` is stored as a small reference record (key, digests, dep edges, refs) plus content-addressed brotli chunks of 2048 entries for the value and for the raw reads, glob members included; chunks are pinned for the retention time. A missing chunk makes the record absent (recompute). Node writes are coalesced (latest record per node, 250 ms, 8 in parallel, `flush()` drains) so an edit does not rewrite 440 big records per keystroke. Test: a 200,000-member glob node with a 200,000-row value is persisted (the record file is under 4 KB), a second process answers it with 0 rule runs and 0 computes, and a changed roster in a third process re-runs the rule once.

Tests now: 12 pass, 530 expects.

## Round 5: stat index, directory stamps, capacity grant

Built (all in `BUILD/🕸️graph/🧮️derived-nodes`, tests 12 + 6 pass, 530 + 54 expects; Python oracles `🐍️oracle/🐍️.py` and `🐍️oracle/🗂️.py`):
- `🗂️stat-index`: per-directory durable stat index (`<root>/d/<sha256 dir>.json.br`: size, mtime, inode, digest, time digested per file, integrity digest per record; damaged records ignored; pruned by count and age). A file with unchanged size, mtime and inode that was already 2 s old when digested is not read: `createDerivedGraphV1({statIndex})`, counter `statReused`. The Python model of the decisions (read/reuse, racy window, inode change, removal) agrees on 7 steps, in one process and across restarts.
- `🌍️world` (`createDiskWorldV1`): `stat`, 16 concurrent directory walkers, `remember:true` (a directory with unchanged mtime is not read again), `verify(root)` and `stamp(root)` (hash of the mtimes of all directories below a root, undefined while a directory is younger than 2 s). The engine keeps a stamp on every glob read and on every cached listing; the no-journal pass `verify`s each listing root and skips the roster scan of every glob whose stamps are unchanged (`stampHeld`), rescanning only the others. Walk output is compared with an independent Python `os.walk`.
- `capacityOfGrantV1(source, ports)`: `capacity` port from the admitted host grant (`HostCapacityHandleV1.cpu/jobs/reservedMegabytes`, `JobLimitPolicyV1.memoryMegabytes/cpuShares`, tested with `grantJobLimitV1`): usable memory = granted memory minus resident size, a quarter reserved; `undefined` grant falls back to `os`; a live source follows upgraded grants. The daemon lane passes `capacity: capacityOfGrantV1(() => ({handle, limit}))` from `withBuildCapacityV1`.
- Faster membership: per-op cache of member lists by resolved pattern set (554 distinct sets for 867 nodes), listings as sorted arrays with range views (no per-base copies), suffix pre-filter before the regex.

Real repo (hot checkout, other lanes editing build sources while the harness runs; load 4.6-9.5), index persisted in a second run:
| | value |
|---|---|
| cold build, index empty / persisted | 36.4 s with 166k reads / 30-32 s with 6.4k reads (the manifests), 172k stat-reused, peak 0.8-1.0 GB |
| shared-manifest edit | 0.15-0.2 s, 1 read |
| project manifest edit | 35-50 ms, 1 read |
| journal-clean no-op | 34-42 ms, 0 reads |
| no-journal pass, nothing changed | **0-3 file reads** (was 166k), 0 computed, 17,919 nodes revalidated, 25-30 s (was 44-92 s), peak 1.0-1.25 GB |

The under-10 s target for the no-journal pass is not met: the 17.9k revalidations still cost 1.27M member checks and 166k stats, and about 800 nodes belong to directories that other lanes edit constantly (their stamps are `undefined` while a directory is younger than 2 s), so they rescan or even recompute in every pass (computed 786-802 whenever the checkout was being edited). The next step is a durable Merkle of directory stamps so a quiet subtree is verified by one comparison; the engine already stores the stamp per glob.

## Round 6: Merkle of directory stamps, compact stat index, wiring

- Merkle stamps: the disk world builds, per verification pass, a hash tree of directory mtimes (a directory's stamp = hash of its mtime and its child directory stamps; `undefined` while any directory below is younger than 2 s). `stamp(base)` is O(1) per base after one tree build, `verify(root)` stats every directory once (no listing read for an unchanged one). The table of directory listings and mtimes is durable (in the stat index shards), so a new process verifies a quiet tree by stat alone and gets the same stamps (test: two processes, equal stamps, a changed subtree changes only its ancestors' stamps).
- Compact stat index: one binary brotli record per top-level subtree (`encodeShardV1`: interned path segments and names, directory listing records, file rows with raw 32-byte digests, trailing sha256; damaged records refused). The same index holds the directory table. Real repo: **6.9 MB in 27 files** (was 439 MB in tens of thousands). Fixed a race that lost a file row when two files of one directory were digested concurrently.
- Cache policy: `graph-derived-nodes` (512 MiB) and `graph-stat-index` (128 MiB) are declared in `⚡️caching/💾️storage/🧾️ledger/🔣️policy.json` (graph partition, EN+DE purposes); ledger tests pass (the `law` test still reports the unrelated `🗄️cas/🧬️schema` line 149 `bazel/actions/cas`).
- Wiring: `createWorkspaceGraphV1({workspaceRoot, cacheRoot, grant, blobs})` (`🏭️workspace`) assembles disk world, shared stat index, node store (+CAS for large records) and `capacity: capacityOfGrantV1(grant)`. The remaining call site is integration's `📜️script.ts:122`: the `granted` of `withBuildCapacityV1` is `{handle, limit}`, so the planner that builds the graph passes `grant: () => ({handle: granted.handle, limit: granted.limit})`; no graph over a real world is created in the planner yet (the closure settler uses an idle world), so nothing else can be wired today.

Real repo, load 4.8-5.1 (index persisted unless noted):
| | value |
|---|---|
| cold build, index empty | 30.4 s, 166k reads, peak 1.16 GB |
| cold build, index persisted | **22.7 s**, 6.4k reads (manifests), 172k stat-reused, peak 1.27 GB |
| shared-manifest edit | 0.13-0.19 s, 1 read |
| project manifest edit | 35-48 ms |
| journal-clean no-op | 34-36 ms |
| no-journal pass, tree unchanged | **6.3-6.8 s** (first pass after the walk 10.3 s), 0-3 reads, 0 computed, 17,919 nodes revalidated, peak 1.3 GB |
| no-journal pass while other lanes edit the build sources | 13.5 s, 801 nodes recomputed because their files really changed |
| stat index on disk | 6.9 MB |
