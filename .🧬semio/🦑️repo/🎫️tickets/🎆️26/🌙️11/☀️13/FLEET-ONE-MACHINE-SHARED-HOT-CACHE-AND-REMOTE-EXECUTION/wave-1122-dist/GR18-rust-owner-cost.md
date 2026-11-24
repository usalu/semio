# GR18: what a Rust owner costs, why, and what changed (2026-11-24, 12:50 to 14:30 Mac clock)

`I` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/`, `O` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/`, `T` = ticket folder. Lock `lock.graph-closure` (released). Probes and scripts: `T/wave-1122-dist/GR18/` (`run.sh` one owner per fresh process, `bounded-read.mjs` one pool-child-like process over a unit list, `select-units.py` evenly spaced Rust owners of the registry, `prof.py`, `stamp-diag.mjs` (which stamped path moved), `row-census.mjs` (rows of every stored entry by kind), `real-root-metadata.mjs` (our reader against `cargo metadata` on a real root)). Outputs lived in the session scratchpad (store 177 MB, deleted at the end). No restart, no source log added (no `[DEBUG]` line), the background read and the play build untouched, one process at a time.

## Verdict
1. The premise is wrong: a Rust owner does not run `cargo metadata`, and its cargo resolution is not what costs. In 8 owner runs (cold, warm, profiled) there were 0 spawned processes. Manifest and lock facts of an owner come from our own reader (`I🦀️native-manifests`, `📥️inputs/🦀️resolution`, receipts, one resolution context per child), not from Cargo. Its terms in the cold profile of `cad-extension-aec-building-rust` (381 s of samples): `flushCargoReceiptsV1` 1.3 s, `locateWorkspaceV1` 0.7 s, manifest read 0.8 s, TOML parse 0.3 s, together 0.8 %. `cargo metadata` runs only in `repo cargo` command runs (C2), never in inference.
2. The cost of a Rust owner is the same machinery GR16/GR17 found for TypeScript owners, plus one thing GR17 did not see: the verification stamps never held for more than a few minutes. Cold owner terms (381 s): offering the derived entries to the fleet (`packBundleV1` from `entry-lane:74`) 34 %, writing/reading the thousands of layer files 21 %, the TypeScript import walk of the owner's command script (`walkImportsV1`) 8.6 %, forced gc 6.4 %, stamp creation 6.4 %. Marginal cold owner (tools warm, `course-rs`, profile of 54 s): import walk 27 %, `packBundleV1` 17 %, analysis of source 15 %.
3. Warm owner, stamps nominally in place, BEFORE the fix: 16.4 s CPU / 38.8 s wall / 684 MB (`cad-extension-aec-building-rust`), 11.6 s (`course-rs`), against 1.4 s for a TypeScript owner in GR17. Cause (`GR18/stamp-diag.mjs` over the store): every stamp named the DIRECTORY that holds each stat-like row (`lstat`, `exists`, `stat`, `fileExists`, `directoryExists`: 6,400 to 16,000 rows per entry, 4,800 to 7,600 parent directories) and compared its size and times. Three of those directories are written all the time: `/Users/ueli`, the workspace root `/Users/ueli/Documents/semio` and `✏️s/🔌️plugins/📕️norm` (also files a peer edits). 25 of 31 stamps were stale one minute after they were made, so the next process verified every entry in full again (layer listings, `keepEntryStampV1` 43 %, `indexedLayerHoldsAsyncV1` 31 %, `listingSync` 21 %) and stamped it anew. This is GR17 open item 3 and it is the whole warm cost of the Rust owners (their entries are the big ones: 7,000 to 11,000 paths, 36,000 rows, `cargo-command` 60,006 layers).
4. Fix (landed): the stat-like rows are witnessed by NAME and FACT, not by the directory: the stamp keeps `typed: [kind, path, fact]` and a verification probes each distinct name once per round with the same function a full verification uses (`recordedFactHoldsV1`). A name created or removed beside the named rows no longer moves anything; a row that changes still fails exactly. No timing is involved for these rows (the recorded fact is compared with the disk, so there is no racy window for them). Listing rows, module resolutions and `analysis`/`field` file bytes keep their path stamps.
5. Measured after (same owner, same host, load 55 to 62, stamps in place, fresh process): `cad-extension-aec-building-rust` 16.4 s -> 2.7 s CPU, 38.8 s -> 10.6 s wall, 684 -> 421 MB, 18 of 18 entries through stamps, 0 layer files read; `dag-dag-rs` 3.3 s CPU, 32 of 32 stamped; bounded warm second pass over 4 Rust owners 1.2, 14.6 (stamps of its entries not yet left after my kill of the cold run), 1.2, 0.9 s; rerun of that owner alone 3.3 s. Target "about a TypeScript owner" (1.3 to 1.6 s) is reached for the median (0.9 to 1.2 s), not for the first owner of a process (2.7 to 3.3 s: Wasm index 24 to 26 %, cargo receipts 15 %, stamp loads 10 %).
6. NOT reached: cold owner "single-digit seconds". The measured cold cost of a Rust owner on a warm tool store is 20 to 187 s CPU (section 3); it is the TypeScript closure machinery per owner (import walk, derive, pack, offer, layer files), not cargo. The levers are GR17 items 2 and 4 (packed layers, background offer, boundary at the declared core) and were not touched here; see the open items.

## 1. Profiles (task 1)
Method: GR16/GR17 `one-owner-v6.mjs`: ONE owner through the real service in a fresh process (session of the wave `gr16/wave`, fleet lane on, shared closure store, `--cpu-prof` through the inspector, fs and spawn counters), host load 38 to 118 on 10 cores, a play release building.

| run | owner | wall | CPU | peak RSS | closure entries | note |
|---|---|---|---|---|---|---|
| cold, empty store | cad-extension-aec-building-rust (cargo) | 381 s | 136.4 s | 3.4 GB | derived 9, stale 6, fetched 9, published 9, imported 18 | 103,164 stat, 65,284 writes, 0 spawns, 7,128 verdicts, 600 names |
| same store, new process (stamps just made) | same | 83.6 s | 28.5 s | 895 MB | loaded 19, stamped 8 of 18 | before the fix |
| same | same | 38.8 s | 16.4 s | 684 MB | stamped 15 of 18, 3 re-stamped | before the fix, the 3 moved stamps are the busy directories |
| cold owner, tools warm | course-rs (cargo) | 54.0 s | 19.8 s | 1.4 GB | derived 6, fetched 5 | before the fix |
| warm | course-rs | 42.2 s | 11.6 s | 480 MB | stamped 14 of 20 | before the fix |
| after the fix, first pass after my edit | cad-extension-aec-building-rust | 134 s | 37.0 s | 1.1 GB | stale 3 (the core module I edited and peers' edits) | |
| after the fix, stamps made | same | 70.4 s | 13.9 s | 513 MB | stamped 17, 1 re-stamped | |
| after the fix, stamps in place | same | 10.6 s | 2.7 s | 421 MB | stamped 18 of 18, layers read 0 | |
| after the fix, stamps in place | dag-dag-rs (32 entries) | 13.6 s | 3.3 s | 482 MB | stamped 30 of 30 | |
| cold owner, tools warm, stopped by me | cargo-workspaces (TypeScript) | 17+ min | over 1,000 s | 3.3 GB | not finished | section 6 |

Dominant terms with shares of the samples:
- Cold `cad-extension-aec-building-rust` (381 s): `packBundleV1` (offer of derived entries) 34.2 %; `layersSync`/`layerBytesSync` (layer files) 21.1 %; `walkImportsV1` 8.6 %; `gc` 6.4 %; `keepEntryStampV1` 6.4 %; brotli 6 %; `observationLayerHoldsAsyncV1` 4.9 %. Cargo resolution total 0.8 %.
- Warm before the fix (38.8 s): `keepEntryStampV1` 43.5 %, `indexedLayerHoldsAsyncV1` 31.4 %, `listingSync` 20.8 %, `readFileSync` 33.7 % (self).
- Warm after the fix (10.7 s of samples, 2.7 s CPU): `wasmCompileOutputIndexV1` 25.8 % (once per process: 1,642 manifests, GR17 item 1), `flushCargoReceiptsV1` 14.9 % (receipts written into the child's staging directory: new each process), `loadStamp` 10 %, `witnessTupleV1` 7.8 %.
- Counts asked for: processes spawned 0 in every run; files read 2,994 (cold) and 19 (warm after); stat 103,164 and 4,793 (cold, before) and about 20,000 name probes per round after; written 65,284 (cold) and 0 after.
- Stamp census (`GR18/stamp-diag.mjs`, `row-census.mjs`): every entry holds a copy of the rows of the shared core: 9,700 rows with 6,400 stat-like names and 1,618 `field` rows; the toolchain entries hold 36,500 rows and 16,400 names.
- Where the premise was checked: `rg 'cargo.*metadata'` over `I` and the observed spawn list (empty); the 4,677 Cargo roots are not 4,677 resolutions: one resolution context is opened per child and keyed by its script cache (`📥️inputs/🟨️.mjs` `resolutionRuns`), manifests read once (up to 256 parsed), receipts read from the child's staging.

## 2. Fix at the root (task 2)
- `O🟨️.mjs` `observationWitnessV1` / `observationWitnessHoldsV1` (`WITNESS_TYPED_V1`): `typed` rows replace the directory stamp of `lstat`, `stat`, `exists`, `fileExists`, `directoryExists` rows; `I🧭️script-inputs/💾️entries/🟨️.mjs` `loadStamp` admits only stamps with `typed` (the stamp format moves once, old stamps are plain misses, no compat code), size estimate; `O🟨️.d.mts` type. Docstring of `observationWitnessV1` names the reason.
- Schema-first: the stamp is data of the closure store (`s/<entry digest>.json.br`), keyed by the entry digest, i.e. by the digests of everything the entry read (manifests, lock, tool files), shared by every owner and every pool child of the machine; a record of a Rust owner stays valid when an unrelated manifest elsewhere changes because its rows name only what it read.
- Our manifest reader vs Cargo (the repo rule "same output as a third-party implementation"): new test `I🦀️native-manifests/🧪️tests/🦀️cargo-go` "path dependencies and path closures equal the answer of `cargo metadata --no-deps` of Cargo itself": a fixture workspace with aliases, `package` renames, workspace inheritance, `dev`/`build`/target tables, 5 crates; every crate's path dependencies and `nativeDependencyRoots` (tests true and false) equal what Cargo reports. One-off on two real roots (`GR18/real-root-metadata.mjs`, system cargo, `--no-deps --offline`): `🎓️teaching` 3 members, 23 path edges, 0 mismatches; `🧰️framework` 524 members, 3,720 path edges, 0 mismatches (Cargo 2.3 s for the whole root, our reader 4.8 s).
- Not done, with reason: persisting the receipts of the cargo resolution in the shared store (they live in each child's staging directory, which the parent adopts; sharing them changes who owns them) and the persisted Wasm index digest. Both are the next two terms of a warm owner (15 % and 26 %, together 1.2 s of 2.7 s).

## 3. Bounded real read over Rust owners (task 3)
`GR18/bounded-read.mjs` over an evenly spaced selection of the registry's Rust and cargo owners (`select-units.py`: 1,011 of the 1,607 owners whose `project.json` carries `language:rust|cargo`; the registry holds 4,869), one pool-child-like process, shared store (tools entries already warm from the owner above), fleet lane on, load 60 to 118. Forty owners were planned; the host load (the cold pass of 3 owners took 49 minutes wall) stopped it at the first four plus the three measured above. Honest size: 8 cold owners.
| owner (cold owner, tools warm) | CPU | wall |
|---|---|---|
| course-rs | 19.8 s | 54 s |
| 2d-trace-rs | 22.2 s | 69 s |
| framework-job-replacement-rs | 39.1 s | 187 s |
| dsl-record-ownership-rs | 38.3 s | 166 s |
| course-schema-rs | 45.3 s | 227 s |
| framework-server-tests-rs | 60.0 s | 307 s |
| framework-job-worker-rs | 77.9 s | 544 s |
| dag-dag-rs | 187.3 s | 1,173 s |
Median 42 s CPU, mean 61 s, heavy tail 187 s (dag). Warm pass of the first four (new process, stamps in place after one pass): 1.2 s, 14.6 s (stamps still being made), 1.2 s, 0.9 s; owner `dag-dag-rs` alone with stamps in place 3.3 s. Cold-to-warm ratio for the marginal owner 30 to 50 times.

Extrapolation (estimate, not a measured whole read; N = 5,231, about 2,600 Rust and 2,631 other owners; other owners from GR17: cold mean 18 s, warm 1.7 s median; Rust from the table above):
| case | CPU hours | one 16 GB worker, 5 children (about 5 GB grant) | 11 workers x 5 children |
|---|---|---|---|
| cold, empty fleet (every owner derived once) | 2,600 x 61 s + 2,631 x 18 s = 206 k s = 57 h | 11.5 h CPU-bound, 13 to 17 h wall at a wall/CPU ratio of 1.1 to 1.5 | 62 min CPU-bound, about 1.3 to 1.6 h wall (each owner derived once fleet-wide, tools derived once and fetched by the rest) |
| fleet already holds the records, one worker adopts (verification through stamps) | 2,600 x 1.9 s + 2,631 x 2.5 s = 11.5 k s = 3.2 h | 38 min CPU-bound, 45 to 60 min wall | not needed |
| same adopt case BEFORE this fix (Rust owner 11.6 to 16.4 s warm) | 2,600 x 14 s + 6.6 k s = 43 k s = 12 h | 2.4 h CPU-bound | |
The fix is the difference between 12 and 3.2 CPU hours for the realistic case; the cold case is unchanged and stays the 57 CPU hours it was. Wall/CPU ratios measured here were 2 to 6 under load 60 to 118 (the numbers above assume an unloaded worker).

## 4. Tests (one `repo test-file` call at a time, sequential)
New: `O🧪️tests/🪪️witness` 5/0 (one new law: names created or removed beside the typed rows leave the witness holding as the rows hold; the git-tree oracle law over 9 tree changes unchanged and green), `I🧪️tests/🪪️verification-stamps` 4/0 (new: a stamp over type and existence rows survives names created beside them and is stale exactly when a named row changes), `I🦀️native-manifests/🧪️tests/🦀️cargo-go` 7/0 (new: equality with `cargo metadata`).
Regression, all 0 fail: layer-lost 6, verdict-wait 9, moved-row 3, answer-validity 10, stale-take 1, take-no-derive 1, durable-entries 4, closure-facts 22, pass-verdicts 2, read-verdicts 4, layer-index 7, validation-round 4, owner-reuse 2, shared-closures 2. `O🧪️tests/🟦️.ts` 68/1: the same roster failure as GR17 (four peer exports missing from the `.d.mts` and the fixture), none of mine (my change adds no export).

## 5. Landing (also in `T/landings.md`)
Files under `LIB` (sha256 16 hex after): `🏗️build/👁️observation/🟨️.mjs` f9ad4e2def8da172 (before 31c7a9fcfcc4d697); `👁️observation/🟨️.d.mts` 8d90e7ecbbcc784f; `👁️observation/🧪️tests/🪪️witness/🟦️.ts` 01b97ebe7d528a5d; `🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs` 5b46dac2bd313b55; `🕸️graph/📥️inference/🧪️tests/🪪️verification-stamps/🟦️.ts` 7db3970ad04884c5; `🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/🦀️cargo-go/🟦️.ts` 7ae64e00bd2311bd. The observation module is part of the shared core (every closure entry names it), so the first pass after the landing re-verifies every entry once (a stale entry is re-derived or fetched), exactly like any edit of the core (GR16 item 2); stamps of the old format are misses.

## 6. cargo-workspaces owner (task 1): NOT measured to the end
`@semio-tech/cargo-workspaces` is a TypeScript package owner (kind `language:typescript`), not a cargo owner; it runs no cargo. I started it as a cold owner on the warm tool store (pass 10, profile on, load 50 to 150) and stopped it myself after 1,000 s of CPU (peak RSS 3.3 GB, still deriving, the profile is only written at the end, so there is none): that is above the 462 s CPU of GR17's cold figure, so the owner is dominated by its own entries, not by anything Rust, cargo or stamp related. The warm figure (34 s in GR17, before the stamp fix) was not re-measured. It is the single heaviest owner (about 2 CPU hours of the 57 above would be 4 such owners); a profile of it on a quiet host is the first thing to do for the cold case.

## 7. Open items, by expected gain
1. Cold cost per owner (all kinds): offer to the fleet 17 to 34 % (`packBundleV1`, awaited up to 15 s per entry), layer files 21 %, import walk 9 to 27 %: GR17 items 2 and 4 (packed layers, background offer by one process per machine, walk boundary at the declared core). This alone decides the cold case (57 CPU hours).
2. Persisted cross-process memo of the Wasm compile index digest (witness: stat of its 1,642 manifests): 2 to 3 s of a first warm owner of every process (GR17 item 1), 26 %.
3. The cargo resolution receipts per child (15 % of a first warm owner, 1.6 to 2.0 s): written to the child's staging every process; share them through the machine store keyed by the digests they record.
4. A stamp still moves when a file a row reads is edited (analysis rows of edited sources, 1 of 18 entries in the pass after peers' edits) and falls back to a full verification of the entry; a per-row fallback restricted to the moved path would bound that.
5. The roster test of `O🧪️tests/🟦️.ts` (peer exports) is still red.
