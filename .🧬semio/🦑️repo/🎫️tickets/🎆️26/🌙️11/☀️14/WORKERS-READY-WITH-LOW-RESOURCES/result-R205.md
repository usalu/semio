# R205: the hash mover (`generate-schema-index`) and the 10 min 38 s of a fully restored Rust task (2026-11-18, 12:00 to 12:45 UTC)

Short answers.
* **A. The tracked index is NOT what moves the hashes of the probe tasks.** It is an output (never an input) of T1, it is not in the source set of `norm-din4108-rs:build`, and the generator is deterministic and already writes only on change. What does move T1's own hash is every edit of ANY schema document in the repository, because that is its declared read profile (legitimate, but wide). Two real defects were found and fixed: the index walk read untracked machine-local directories (machine-dependent bytes possible), and `dsl-derive-rs:generate`/`check-generated` had no declared inputs (stale cache hits possible).
* **B. The server is not the bottleneck and neither is the per-unit restore on a quiet client.** Measured here (server host, load 64): action result 6 ms p50 / 19 ms p90, blobs 205 MB/s. Per unit CPU of a restore is 40 ms (bench) to 80 ms (local hits of 6852 units). That is about 1 minute for 756 units at width 4, not 10. So most of the 10 min 38 s is NOT the restore of the 688 fleet units. What can be said from the files is a ranked list of suspects (section B3). One fix on the restore path landed (4 round trips to 2, 2.1x faster per unit at 50 ms latency); the instrumentation that settles the rest in one run is specified (B5) and is the next landing.

Landed 12:30:36 to 12:30:39 UTC under `landing.lock` (entry "R205 landing 1" in `landings.md`, script `🧰️scripts/R205/land.sh`, base copies and new files beside it as `*.txt`). Staged tests with `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` through a module-map preload (`🧰️scripts/R205/preload.ts`), again under the lock: pipeline 7, stability 5, schema-document-index 2, generate-class 2, 0 fail, import smoke ok.

## A. The generated schema index

### A1. Is the file a hash mover? (refuted for the probe, with two corrections)
* `dsl-derive-rs:generate-schema-index` is covered by the input-precision rule `dsl-schema-index` (`📥️inputs/🎯️precision/🔣️.json`): `replaces` default, `^default`, native sources; `reads` the profile `schema-documents`. So R203's "no declared inputs" was wrong about the Nx hash: T1 reads every schema document of the repository, never the index ("its own output is written, not read"). The index file therefore cannot move T1's hash, and T1's hash moves when any schema document changes anywhere (7302 documents in 137 roots today). That is the real reason two machines that start minutes apart often hash T1 differently: not the index, a peer's schema edit.
* `norm-din4108-rs:build` reads the Rust source set (`cargoSourceSetV1`: the `.rs` files reachable by `mod`/`#[path]` plus `Cargo.toml`; `emitters` are files that emit `include*!`). The derive crate (`✨️derive/🦀️.rs`) reads the index at proc-macro expansion with `std::fs`, not through `include_str!`, so rustc's dependency file of a consumer lists the schema documents it embeds but not the index. The index is in no closure of the probe: refuted by reading the code (the hasher itself was not run: whole-workspace graph read).
* Consequence worth a decision (NOT changed): because the expansion reads the index untracked, a consumer unit is not invalidated when only the index changes (a document moved to another root without any embedded document changing). It would be tracked by `proc_macro::tracked_path` of the one resolved entry, not of the 2.2 MB file.
* Why the file shows `M` against HEAD (2682 insertions, 2677 deletions): HEAD is old; the live file differs by path spellings of moved/added schema documents (emoji directory names). The 11:08 write (mtime) was a legitimate content change, not churn.

### A2. Determinism, no-rewrite (proved), hermeticity (defect fixed)
* The projection is deterministic by construction: directory entries sorted by UTF-8 bytes, ids sorted, paths relative with `/` and NFC, `JSON.stringify(…, null, 2)` + `\n`, no time, no absolute path. Law `✨️derive/🧪️tests/🗂️schema-index-stability` (5): the same tree at another absolute path in the reverse creation order gives identical bytes; LF only, one final newline, no path of the machine; documents equal what an independent `fast-glob` finds; the committed file is content-stable (LF, sorted keys, no absolute path, equals `JSON.stringify(parse, null, 2)\n`).
* No-rewrite already held (`committed === projected` returns before the write). Now one function, `writeProjectionIfChangedV1`, serves both projections (the code was repeated); law: second write keeps inode and `mtimeNs`.
* **Defect:** `schemaIndexRoots` walks the whole checkout and only excluded `target, node_modules, dist, 🗑️generated, 📦️packages`. A schema directory under any untracked machine-local directory (`temp`, `local`, `generated`, a stray copy, `staged`, `holder`) would enter the index on that machine only and the replica would copy it to all (a rewrite war). Today no such directory holds a schema (checked: `staged`, `holder`, `temp`, `test-results`, `undefined`, `bazel-out`, launch kits; `dist` is excluded). Fix: bare directory names of the workspace `.gitignore` (`schemaIndexIgnoredNamesV1`) are skipped; the committed index has no such path segment, so its bytes do not change. Law "a directory the ignore list names ... never changes the bytes; without the entry the same directory would" (red on the old code).
* Not done, by design: tracked-only walking via the VCS (tool-independence rule).

### A3. Declared inputs and the plan for the move
* `dsl-derive-rs:generate`, `check-generated` (mutation-authority projection from the taxonomy) had no rule and no `inputs`: default = project root only, so a taxonomy edit left a cached "unchanged"/"fresh". Now `inputs`: `default`, root `📋️project.json`, the taxonomy, the schema of the projection (`preview-generated` gained root project file and taxonomy). Deliberately NOT `reads` (it would include the 2.2 MB index, a hash mover for these three).
* Move out of the tree (not done, too wide for one landing): the index is a pure function of tracked documents, so it can be an OUTPUT in the cache partitions: (1) the derive reads it through a locator that resolves `$SEMIO_SCHEMA_INDEX` / a `OUT_DIR`-like path (`MUTATION_SCHEMA_INDEX_LOCATOR` in `🔖️mutation-source-authority/🦀️.rs`); (2) `generate-schema-index` declares it as `outputs` under the machine cache path (`{workspaceRoot}/.cache/...`, never a tracked path) and becomes a normal cacheable producer whose hit RESTORES the file; (3) every consumer target gets the task as `dependsOn` and the path in its env; (4) untrack the file, `check-schema-index` compares the produced file. Rule for step (2): a cacheable generate task may declare an output only when the output is untracked; for a tracked output a cache hit would write an old copy over a newer file and the replica would broadcast it.

### A4. The law over the generate class (`🏗️build/🧪️tests/🏭️generate-class`, 2 tests, red before: 1)
The closure is the probe run's three generate targets plus the other generate targets of the same projects (5 projects, 6 targets; an explicit list, no whole-workspace scan).

| target | cache | declared inputs | outputs | verdict |
|---|---|---|---|---|
| `dsl-derive-rs:generate-schema-index` | true | precision rule `dsl-schema-index` (schema documents) | `[]` (writes a tracked file idempotently) | ok |
| `dsl-derive-rs:generate` | true | **none before** → own `inputs` now | `[]` | violator, fixed |
| `framework-rs:generate` | true | rule `framework-generate` (cargo closure) | `[]` | ok |
| `ui-contract-rs:generate` | true | rule `ui-contract-generate` | `[]` | ok |
| `framework-schema:generate` | **false**, class side-effecting, reason mutates-workspace | n/a | `[]` | listed (ratchet) |
| `ui-styling-tokens:generate` | **false**, same | n/a | `[]` | listed (ratchet) |

Why the two are uncacheable and whether they can be cacheable: authored `cache:false` because they write tracked generated files (framework manifest TS; CSS/TS/C#/Rust/Python styling artefacts) into the tree. They already write only on change (`writeGeneratedFileIfChanged`). They can be cacheable under the "ensure" semantics (outputs `[]`, hit = skip, because the files are tracked and replicated) once (a) their inputs are declared by a precision rule (as `framework-generate`), and (b) `check-generated` stays the freshness gate. Not done: needs their input closure derived and a rule added to the central precision document, which moves their hashes only but is a shared engine file.
The cross-check: an independent Python classification of the same files equals the TypeScript rows (test 2).

## B. The 10 min 38 s of `norm-din4108-rs:build` on g1-m01

### B1. What is known
* g1-m01 is a Mac mini 16 GB / 256 GB (host name ends `-lan`), pool role "puller" with 4 children and about 1.9 GB RSS beside the build. The Nx run was 11m13 with 2 tasks on the critical path (11m06); `Cache: 0/7`.
* The log has no phase lines of the cargo run (only `still running after N s` heartbeats and, at the start, one `Waiting for access to native-storage (exclusive): position 2 of 2` lease queue line). So a split of the 638 s cannot be read from it; this is why section B5 specifies the missing ledger.
* The restored set (from the provenance journal, 873 unit results first stored 03:00 to 10:19 on 11-18): mean 4.9 MB per unit, 4.9 files per unit, p50 1.2 MB, p90 10.2 MB, p99 54.8 MB, max 182 MB, 4.29 GB in total. 688 units are therefore about 3.4 GB; average 5.3 MB/s over 638 s if the whole task were download.

### B2. Time budget (what each step costs, measured or bounded)

| step | where | measured / bound | share of 638 s |
|---|---|---|---|
| planning `cargo:artifact-plan` (756 units, 5.8 MB) | native-build plan | not measurable from the logs | unknown |
| leases before cargo starts (compiler capacity scope, native-storage exclusive queue) | `withCompilerCapacityScopeV1`, `nx:fixed-cache` | one queue line in the log, wait not logged | unknown |
| wrapper process per unit | sealed shim | 60 ms (bundled script, load 60) | 756 x 0.06 / w = 11 s at w 4 |
| keying + digests + local lookup | `wrapRustcV1` | included in 80 ms mean of 6852 local hits (events 11-14/15: p50 42, p90 137 ms) | about 15 s at w 4 |
| fleet round trips per unit | `obtainFleetUnitV1` | before 4 (result, manifest, blobs, slot claim), now 2 | at 6 ms server latency: negligible; at 300 ms: 756 x 4 x 0.3 / 4 = 227 s before, 113 s after |
| server answer time | cache server (this Mac, load 64) | result 6 ms p50 / 19 p90 / 31 max, blobs 5 ms p50 / 24 p90, 205 MB/s (25 units, 70 MB, from the server host to its LAN address) | server is not the limit |
| download | LAN | 3.4 GB: 31 s at 110 MB/s (wired), 283 s at 12 MB/s | unknown link; wired mini suggests under 60 s |
| verify + CAS write + clone | `getBlob` sha-256, `putCasObjectSyncV1` (hash again, write), `COPYFILE_FICLONE` | bench: 40 ms CPU per unit at loopback; clone is free on APFS | about 8 s at w 4 |
| cargo fingerprint, linking | cargo | not measured | unknown |
| the compiled units (K of the `[units]` line, at most 68) | rustc via scheduler | a cone above a peer-edited crate compiles serially: 13 closure crates were edited since the units were stored (R203 section 2) | plausible majority: a chain of 10 big crates of 30 to 60 s is 5 to 10 min |
| ledger lines | R203 | `[units]`/`[nx-task]` exist since 12:00 | will show K |

Summing the restore-side rows gives about 1 to 2 minutes for 756 units at width 4, not 10. The remaining 8 to 9 minutes sit in: (1) real compiles on the critical path (the 68, or a few of them, which are exactly the din4108 layers at the top of the cone; their keys are legitimately new because crates below were edited), (2) lease/capacity waits before cargo starts, (3) planning and linking. The fleet-key portability of the compiled units: on this Mac's event log of 11-14/15 (before R198) 69 of 2108 compiles were `path-dependent` skips (35 `semio_s_artifact_stdio_*` crates that embed the workspace path); that is 3 % and would be about 25 units per probe that are compiled by every machine and shared only between machines with the same checkout path; all eight machines now share `/Users/ueli/Documents/semio`, so the root-keyed key applies. Not verified for 11-18.

### B3. Suspects, ranked (with what would prove each)
1. Compile cone (K): proves with `[units] ... K compiled`: K above about 10 on a warm fleet means peer edits; the K units' crates name the cone.
2. Lease/capacity wait: prove with timestamps around `withCompilerCapacityScopeV1`; the log shows a native-storage exclusive queue behind another process.
3. Width: `CARGO_BUILD_JOBS = facts.parallelism` (`🏗️native-build/🟦️.ts:910`), the admitted compile capacity, equals the restore width. A restore needs neither CPU nor memory beyond a 60 MB wrapper; real compiles are separately bounded by the broker scheduler (memory) and the unit-cache slot semaphore (`SEMIO_UNIT_CACHE_SLOTS`, default all cores). On a "low resources" Mac, parallelism may be 2 to 4. Bench at 50 ms latency, 200 units: width 4 = 6.3 s, width 16 = 3.6 s to 4.6 s (load 62 noise).
4. Bytes: 3.4 GB at an unknown link speed (rlibs compress 3x: below).

### B4. What landed (fleet hit path) and its proof
* `installFleetUnitV1`: the action result already carries the digests of the manifest and of every output, so manifest and outputs download together (`downloadBlobsV1`, largest first, policy width 4); the manifest is checked against the result afterwards exactly as before (platform, slot, expected outputs, digest AND size of every output, streams). Bounds before a byte moves: declared size above `maximumBlobBytes`, more than `maximumBlobs + 3` blobs, manifest above `maximumManifestBytes` all give a miss with zero blob requests.
* `fetchFleetUnitV1`: the slot claim (`POST /v1/slots`, awaited up to 600 ms on the path of every hit) runs beside the restore; `awaitFleetSettlesV1()` (bounded by the same 600 ms) is awaited once by the wrapper before the process ends, so the fleet is still told, never lost at exit.
* Laws `🛰️fleet/🧪️tests/⚡️pipeline` (7): a deterministic round-trip counter (virtual clock: requests released in waves) gives 2 for a unit, 2 for a hit including the slot, 4 and 3 before (red); call counts 1 result + (outputs + 1) blobs; installed CAS objects equal the manifest by an independent Python SHA-256; width and order of downloads; hostile entries (corrupt blob, oversize, crowded, lying manifest) are misses with no blob moved; wrapper source awaits the settle once; real cache server over a socket (6 units). At least one of the hostile cases (a size above the bound or a manifest that disagrees with the result) was accepted by the old code (red).
* Numbers (real cache server on loopback, injected 50 ms per request, units sized as the real distribution at 1/10, load 60): before 13.39 s, after 6.31 s for 200 units at width 4 (268 to 126 ms per unit, 2.1x). 60 units at width 4: install only 2.85 to 1.81 s, with the slot claim 4.0 to 1.78 s. No-latency loopback: 40 ms CPU per unit, 222 MB/s.

### B5. Not landed: plans and the missing instrumentation
1. **Time ledger (next landing, specified):** extend R203's `[units]` with `[units-time] restore fleet N: X s (bytes Y MB), restore local ..., compile ..., waits slot/lock/lease ..., cargo wall ...` from the unit events of the run window (lane `g`, `x/g.jsonl`: `remote-hit`, `hit`, `miss`, `slot-wait`, `lock-wait` carry `milliseconds`/`waitedMs`) plus bytes from the blob sizes. One ready run then settles B3 for every machine.
2. **Restore width decoupled from compile width:** `CARGO_BUILD_JOBS = max(parallelism, min(32, 2 x parallelism))` when a fleet endpoint is configured and `SEMIO_UNIT_CACHE_SLOTS = parallelism` (compile concurrency unchanged). Risk to settle first: cargo runs the build-script binaries itself, so widening raises their concurrency; needs a law with a fake plan and the memory admission of build scripts. Lines: `🏗️native-build/🟦️.ts:910`, `🎟️admission/🟦️.ts:344`, `🧩️component-pass/🟦️.ts:419`.
3. **Bytes:** rlibs compress 3x (10 real rlibs, 47 MB: gzip -1 2.94, zstd -1 3.15, 68 MB/s per core under load; an unrepresentative first sample of Nx bodies gave 1.1). `Content-Encoding` is not used anywhere (the mirror even asks for `identity`). Plan: the server stores/serves `zstd` for `/cas` above 64 KiB (precompressed at put time, not on the fly: the server is a loaded Mac) behind `Accept-Encoding`; the client keeps verifying the SHA-256 of the DECOMPRESSED bytes.
4. **Server marks the slot on a hit** (remove the client call entirely) and `POST /v1/slots` batching in the uploader worker.
5. **Prefetch:** keys depend on the dependencies' content, so no up-front batch is possible; a per-plan "previous run" manifest of unit keys (last good plan for the same graph digest, from the fleet) would allow one batched `FindMissing`/read wave for the first dependency level. Not designed further.
6. Target "756 units in well under a minute on a 16 GB Mac": needs 2 (width), 3 (bytes) and the 2-round-trip hit together; 40 ms CPU x 756 / 8 = 4 s plus 3.4 GB / 110 MB/s = 31 s, or 10 s with compression, plus cargo's own work. Not demonstrated end to end.

## Not verified
* A real cargo build through the wrapper with the new code (the staged end-to-end harness cannot run: bundling the wrapper executes several `isMainModule` entries; the base code fails the same way; machine load was 60 to 65). Risk covered by: the existing slot-claim test (client), the new stub and real-server laws, the source law, and live import smoke.
* The split of 638 s on g1-m01 (no phase data); the real link speed of the workers; whether `dsl-derive-rs:generate` inputs agree with the central precision profile checks (only the project file changed; its input schema accepts the form, used by 6 other targets in the file).
* That the generator with the `.gitignore` rule reproduces the committed bytes on the whole checkout (a whole-workspace walk was forbidden): argued by the committed file having no ignored segment.
* The hasher was not run (graph read).

Files: `🧰️scripts/R205/` (`land.sh`, `*.base.*.txt`, `*.new.*.txt`, `test-*.txt`, `preload.ts`, `patchA.py`, `patchB.py`, `bench.mjs`). Scratch: `scratchpad/R205/`.
