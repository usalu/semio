# R59: the owner record is small and proportional to the package (packed record form)

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Scripts in `T/🧰️scripts/R59/` (`land.py`, `rehearse.py`, `repro.mjs` = R49's puller harness plus a `STORE` line, `analyze.mjs`, `analyze2.mjs`, `proto.mjs`, `real-differential.mjs`, `take-timing.mjs`). One landing, 2026-11-15 20:30 (entry "Package R59" in `T/landings.md`, 19 test directories green or equal to the baseline taken before). The running read (`.cache/f/graph-server/read-record.json`, runner 12800) was not touched.

## 1. What is in a record (measured, `analyze.mjs`, private session and cache, before the change)

An "owner record" is an entry with 1 `record` document (the owner's own rows) plus N `closure-<key>` documents (closure entries of the tool scripts the owner's inference consumed). Byte breakdown, uncompressed JSON, before:

| owner | total | `record` (the owner itself) | closure documents | brotli, sum of blobs |
|---|---|---|---|---|
| kernel-rs (native) | 125.8 MB, 13 docs | 169 KB (0.13 %) | 125.5 MB (3 large + small) | 4.49 MB |
| framework-test (native) | 287.8 MB, 17 docs | 11 KB | 287.7 MB, 12 closures, three of 72 to 119 MB | 11.3 MB |
| value-ts | 320.0 MB, 24 docs | 30 KB | 319.9 MB, 19 closures, three of 72 to 119 MB | 12.3 MB |
| schema-validator-ts | 280.4 MB, 15 docs | 16 KB | 280.4 MB | 11.1 MB |

So: **the owner's own data is 0.003 to 0.13 % of the bytes; 99.9 % is shared tool-closure material** (no dependency crate's rows are inlined: a crate's dependencies appear only as the observation rows of the owner, `file`/`exists`/`analysis` with 219 + 256 + 17 rows for kernel-rs). The shared material is the same for every native owner and, for TypeScript owners, nearly the same: the closure `{"fn":"tool-closure","name":"cargo-command","view":"native"}` is 122.4 MB for kernel-rs. Inside that one document (437 096 rows in 6 292 layers):

- **93 % of the rows are repeats**: 404 591 of 437 096 rows are byte-identical to a row in another layer (every layer restates the directory chain of the root and the `package.json` probes); only 32 505 are distinct. 85 of 93.5 MB of layer bytes are repeats.
- **51 % of the layer bytes are path strings** (`⟦semio-root⟧/🧰️framework/…`, 48 of 93.5 MB).
- The sidecar (per-layer walk facts) is 7.7 MB, almost all of it the same 3 to 4 long strings repeated per view.
- brotli hides most of it on the wire (122 MB to 4.3 MB) but not in the heap: every producer builds, serialises, compresses and every taker decompresses, parses and converts the whole 122 MB per closure and owner (3 closures of this size per owner: the 1.1 GB of the refused entries are ten such closures; the pool child at 10 GB RSS is these arrays plus their JSON).

The content-addressed blob store already stored identical documents once; what was repeated per owner was the work and the memory, and the blob size (4.3 MB for a 122 MB document).

## 2. What was built

**Schema first**: `B/🪪️graph-state/📦️project-records/🗜️packed-rows/🧬️schema/🔣️.json` (packed rows v2: `table` of distinct rows with the path front-coded against the previous table row (UTF-16 units, never inside a surrogate pair), `pool` of long repeated strings replaced by `{"$": n}`, `lists` as delta-coded table numbers, `extras` for the value and sidecar of a closure entry; documents `PackedRecord`, `PackedClosure`), and `Policy {form: 2}` in `…/📦️project-records/🧬️schema/🔣️.json`.

**Record form**: `record` is `{version:2, digest, image, rows: packed(authority, observations)}`; `closure-<neutral key>` is `{version:2, recipe, rest, rows: packed(entry rows, layer 0..n)}` (`B/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts`: `packRecordV1`, `unpackRecordV1`, `packBundleV1`, `unpackBundleV1`). The neutral conversion (root token, zeroed stamps, provider owner token, closure keys: R49's zero-stamp rule kept) is memoised per row object, so a row shared by 6 000 layers is converted once; `packBundleV1` consumes the layers one at a time and holds only the distinct rows and the numbers per layer. The old array form functions (`neutralBundleV1`, `localBundleV1`, `neutralRecordV1`, `localRecordV1`) stay as the row-level conversion and as the differential oracle; there is no reader of the old stored form.

**Streaming at the source**: `closureTravelV1.export` (`script-inputs/🟨️.mjs`) no longer builds `store.bundle` (all layers in memory); it returns a bundle whose `layers` iterate `store.layersSync` (new, `💾️entries`): one layer is read, verified and parsed at a time. The bounded cache of bundles now holds descriptors only.

**Sharing and fetching only what is lacking**: closure documents are byte-identical for every owner that names them (same bytes, same blob digest, stored once; proven on fixtures and on real owners, section 4); the producer caches a packed document per `(recipe key, store stamp)` and reuses it while the closure keys it consulted still answer the same (`recordingKeysV1`, `consultedHoldV1`). A taker records, per downloaded closure document, `blob digest -> local recipe key` in its closure store (`travel/<digest>`); `fetchGraphEntryV1`/`decideGraphEntryV1` got a `skip` predicate (`GraphHeldV1`) and the port skips every `closure-*` blob whose marker points to an entry still in the store (the neutral to local key of a skipped document is added to the key map so rows of present documents that name it still map).

**Limits**: `documentMaximumBytes` (declared but unused until now, 1 GiB) is now 256 MiB and enforced; new `entryMaximumBytes` 512 MiB; `graphDocumentProblemV1` refuses before any compression and names the document and the size in English and German (used by `publishGraphEntryV1` and in the pool's failure message instead of the blank "refused"); decompression of a taken blob is bounded by `documentMaximumBytes`.

**Old records are never used**: the record state digest now includes `Policy.form` (2); the old digest function is not read anywhere (law test). The store key space of the live cache therefore starts empty for project records.

## 3. Soundness (what changes the key, what changes validity)

- The record key is a pure function of (form, key text of the inference, platform, fleet text) as before; the stored bytes are a pure function of the lists (law: equal lists, equal bytes; sorted by first occurrence, pool in first-occurrence order).
- **A changed dependency does not change the owner's key; it changes validity**: the record carries every observation (file rows with SHA-256 for each file it read, the `closure` rows by recipe key); a taker replays all rows against its own tree before it uses the record (`import`/`current`) and infers itself when one differs. This is unchanged. The closure documents are referenced by recipe key (the digest of the recipe) and by blob digest in the entry note; a changed tool file changes the closure's rows, the taker's verification of the entry fails and it derives the entry itself ("a wrong bundle costs a derivation and never a wrong graph"). Skipping a held document is equivalent to having downloaded it: the entry in the store is whatever this machine has for that recipe (it may be newer than the document), and the consumer verifies it the same way.
- Nothing here makes an owner record reference another owner's record: no dependency crate's rows were inlined in the first place (measured), so there is no Merkle edge between owner records to add; the shared material is the closure entries, which are the DAG nodes (referenced by recipe key, stored once by blob digest, fetched only when lacking).

## 4. Proof (tests; all run on the landed tree by `land.py`)

- New `…/🗜️packed-rows/🧪️tests` (8): language-agnostic fixture of 8 cases (empty, runs and repeats, fact variants incl. absent/`null`/bool/object, surrogate prefix, pooled strings, a `$` key switching the pool off, same row with different facts); TS packs to the stored form and unpacks it; **a second implementation in Python (`🐍️oracle.py`, own UTF-16 handling) packs the same lists to the same form and unpacks the stored form to the same lists**; shared objects after unpacking; refusal of damaged forms (prefix, row number, pool index, shape); law "size grows with distinct rows": 1 500 layers of 401 rows pack to 1 900 table rows and under 1/25 of the expanded bytes; pseudo-random round trip.
- New `…/📦️project-records/🧪️tests/🗜️packed` (6): (1) **size law**: record document 3 888 bytes at closure depth 2 and 3 889 at depth 10 (independent of the closure), closure documents 53 114 B for 2 entries and 265 850 B for 10 (linear in entries), expanded 1.17 MB and 5.86 MB (22x smaller); (2) **sharing**: two owners with the same 6-entry chain store 6 + 2 distinct blobs (the six closure blobs are the same digests); (3) **fetch only what is lacking**: the second record that names the same entries costs 1 blob download (its record) instead of 6, and after the local entries are removed all 6 are downloaded again and the entry equals the first; (4) **differential**: packed then unpacked bundle equals `neutralBundleV1` output (rows, layers, value, sidecar) and its local form equals `localBundleV1` of the expanded one, the record round-trips; (5) the old state digest is not the new one; (6) document and entry limits refuse with both languages.
- Red before: the law file fails on the old tree with `Export named 'packBundleV1' not found` (verified by building the overlay from the base files, `R59_BASE=1`); the size law compares against `neutralBundleV1`, the old inline form, 22x.
- Landing run: 19 directories (portable 3, project-records root + 4 subdirectories, graph-state, wanted, script-inputs closure-store tests, bundle-rows, owner-pool, heads, remote-exchange, observation, both standing preflight tests): none red, each equal to or better than the baseline run on the untouched tree (all green; the baseline is `scratch/R59/baseline.json`).
- **Real data differential** (`real-differential.mjs`, old store vs new store of framework-test, 12 closure documents, same recipe keys): 9 of 12 closure entries are identical (rows, layers, value, sidecar); the other 3 differ because my own landing added `packed-rows/🟦️.ts` to the files the tool closure reads (value differs by exactly that file; one has 4 351 instead of 4 348 layers), which is the closure doing its job, not a codec difference.

## 5. Measured before and after (real owners, `repro.mjs`: `runOwnerPoolV1` for one owner against a private cache with the production limits, host load average 22 to 42; RSS/footprint from `/usr/bin/time -l` of the whole run, noisy ±300 MB)

| owner | doc bytes before | after | blobs (brotli) before | after | own record before | after | wall | max RSS | footprint |
|---|---|---|---|---|---|---|---|---|---|
| kernel-rs (native) | 125.8 MB | 9.8 MB | 4.49 MB | 0.74 MB | 169 KB | 111 KB | 30 s to 24 s | 1.81 to 1.56 GB | 825 to 837 MB |
| framework-test (native) | 287.8 MB | 27.4 MB | 11.3 MB | 2.78 MB | 11.3 KB | 9.5 KB | 38 s to 24 s | 1.77 to 1.78 GB | 1.89 GB to 808 MB |
| kernel-ui-rs (native) | 3.2 MB (4 docs) | 9.8 MB | 0.16 MB | 0.73 MB | n/a | 75.8 KB | 40 s to 26 s | 1.54 to 2.00 GB | 1.58 to 1.10 GB |
| value-ts | 320.0 MB | 34.5 MB | 12.3 MB | 3.66 MB | 29.8 KB | 27.3 KB | 41 s to 22 s | 2.02 to 1.73 GB | 2.19 GB to 777 MB |
| schema-validator-ts | 280.4 MB | 26.1 MB | 11.1 MB | 2.60 MB | 15.6 KB | 13.6 KB | 40 s to 23 s | 2.01 to 1.76 GB | 2.05 GB to 729 MB |
| compiler-syntax-ecma-ts | not measured | 26.1 MB | | 2.60 MB | | 13.8 KB | n/a to 27 s | 1.74 GB | 762 MB |

Reading: documents 11x to 13x smaller uncompressed, 4.5x to 6x smaller on the wire, the owner's own record 2 to 25 KB (kernel-rs 111 KB because the crate has 219 file rows), the largest single document 6.5 MB (the cargo tool closure; before 122 MB). kernel-ui-rs "before" had only 4 documents because its closure entry was not yet in the store at that time (the store lacks an entry, it is not exported); do not read its row as a regression. The wall time includes the inference (a child of 20 s); the footprint of the TypeScript owners fell by 2/3 but kernel-rs and the RSS did not move beyond the noise: **I did not isolate how much of the footprint change is the packer and how much is run-to-run variation of the inference child.**

Taker side, largest closure document of framework-test (`take-timing.mjs`): decode plus localise **1 325 ms and +804 MB RSS before, 143 ms and +172 MB after** (121.2 MB against 6.3 MB of JSON).

Across independent runs on real owners the closure blobs are the same digests: value-ts and schema-validator-ts share 23 of the 29 blobs of the smaller store, kernel-rs and kernel-ui-rs 15 (all closure blobs).

## 6. The cache's refusal: which limit

**Not identified.** The declared limits do not explain it: the live and the private server accept a blob up to `maximumEntryBytes` 512 MiB (the same figure in `🔣️policy.json`), the entry holds at most 4 096 documents (`entryMaximumDocuments`), the action result is far below 4 MiB, and the former `documentMaximumBytes` 1 GiB was never read. A private server with the production figures accepted the 125 MB kernel-rs entry (a 4.3 MB blob) in 24 to 30 s. Candidates for the 119 MB to 1.1 GB entries: (a) serialising a closure of several hundred MB: `JSON.stringify` of a string above about 1 GiB throws `RangeError: Out of memory` here (measured, Bun 1.4.2); (b) a blob upload that outlasts its timeout (20 s + size / 2 MiB/s) while the parent's event loop is blocked for tens of seconds; (c) a `putResult` after a lost hold. None of them is possible any more at the new sizes (largest blob 0.7 MB packed). What I did instead: made the limit explicit and cheap: an entry above `entryMaximumBytes` (512 MiB) or a document above `documentMaximumBytes` (256 MiB) is refused before compression with a message that names the document and its size in both languages, and a taker's decompression is bounded.

## 7. NOT verified, and what the others must know

- **Not run on the server's wave**: the live read keeps its old code and the live cache holds no project record of the new form (the digest changed, so every record is inferred once more); owners per minute and the pool child's RSS on the server are unmeasured. Expected effect: the three closure blobs per owner are produced once per closure and process (cached by stamp), not once per owner.
- **Real take through the whole gate was not run on real owners** (a real taker needs a second machine/session and a record in the cache): covered by the fixture law (take, held skip, lost entry downloaded again, forged/unclosed bundles refused: the 3 existing closure tests pass unchanged) and by the real-data decode/localise/differential above; `adopt` of a real 4 000-layer entry into a store was not timed.
- Full-repo wave size (5 102 owners): extrapolation only. Distinct material is the closure documents (about 15 to 25 distinct per tool view, at most 6.5 MB raw and 0.7 MB packed each) plus one record of 2 to 111 KB raw per owner; I did not run a wave.
- Parent-side "merge by digest": the project-nodes wave parent (`observe`/`refs`, R57) was not changed; its 10 M heap objects come from other material than the record documents (R57 section 3). This change removes the per-owner 120 MB closure arrays from the producer and taker; the wave parent still loads the farm child's rows.
- `framework-surface-rs` / `io-schema-rs` (1.1 GB and 1.04 GB entries) were not run (memory risk on this host at load 40 to 60); the same closure structure, so the same fix applies, size unmeasured.
- Windows and Linux: pure TypeScript and the existing portable conversion; nothing run there. The Python oracle needs `python3` (the test fails without it, like the portable oracle test).
- Types were not checked with `tsc` (the repo's type check was not run); the code follows the existing types (`PackedRowV1` is structurally an `ObservationRowV1`).
- Left behind and harmless: `travel/<digest>` marker files (64 bytes) in the closure store directories (the collector sweeps only `e`, `l`, `v`); `held` checks that the entry still exists.
- For other packages: any code that reads a `closure-*` or `record` document of a project-inference entry must use `unpackRecordV1`/`unpackBundleV1`; the port is the only reader today. `closureTravelV1.export` bundles are lazy: iterate `bundle.layers` once per use, do not spread it into arrays.
