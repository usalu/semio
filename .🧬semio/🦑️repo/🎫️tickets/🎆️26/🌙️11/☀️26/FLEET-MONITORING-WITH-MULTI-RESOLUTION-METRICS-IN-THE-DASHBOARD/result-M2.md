# 📈 Result M2: Metrics Foundation (catalogue, tiers, ring files, TS writer/reader, Rust reader)

Member: `🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/` (area lock `metrics`, released). Domain-neutral, next to host-capacity. Std-only Rust, no new runtime dependency.

## Files
- `🧬️schema/🔣️.json`: `Ring` (the binary layout, documented in its description), `Files`, `Tiers` (t0..t4 as design §2), `Kinds`, `Units`, `Sources` (collectors, en/de), `Metric`/`Catalogue`, `Point` (incl. the merge rule), `Texts` en/de.
- `📇️catalogue/🔣️.json`: 69 metrics (all of design §1, host/warden/process/build/cache/exec/graph/ready/replica, 8 `metrics.self.*`), each id/kind/unit/source/label en+de.
- `🟦️.ts`: writer and reader. `🦀️.rs` + `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`: Rust reader (standalone `[workspace]`, no root Cargo.toml edit).
- Tests: `🧪️tests/🟦️.ts` (11 tests), `🧪️tests/🐍️.py` (independent oracle: fold math, counter deltas, and a third reader of the bytes), `🧪️tests/🦀️.rs` (4 tests), golden vectors `🧪️tests/🧫️fixtures/🏅️golden/` (`t0..t2.ring`, `queries.tsv`, `series.tsv`, input `🔣️.json`). Registered tree `🏗️build/📈️metrics` in suite `module-basics` (`🧪️testing/🗂️suites/🔣️.json`, anchored edit).
- Kept scripts: `🧰️scripts/M2/{catalogue.py,golden-input.py,measure.ts}`.

## API (explicit exports of `🟦️.ts`)
- `openMetricsRingsV1({directory, ids?, tiers?, createdMs?}) -> MetricsRingsV1`: one dir = one writer; creates `t0.ring..t4.ring` atomically (tmp + rename), rebuilds the open windows of tiers >= 1 from disk; a catalogue change (ids digest) starts empty rings (no migration). Convention for M3: `directory` = `<machine cache>/metrics`.
- `recordSampleV1(handle, atMs, values) -> {accepted, dropped, unknown}`: counters are fed as per-tick deltas; `createCounterDeltasV1().delta(id, reading)` converts cumulative readings (first = baseline `null`, decrease = restart). Samples older than the newest written row and non-finite values are counted, not thrown.
- `closeMetricsRingsV1(handle)`: writes the open 1 s window, closes fds.
- `readSeriesV1({directory, fromMs, toMs, ids?}) -> {fromMs, toMs, metrics: {id: Point[]}}`, `Point = [start, width, tier, count, min, max, sum, last]`; also `readRingFileV1(path)`, `removeMetricsRingsV1`, `METRICS_CATALOGUE_V1`, `METRICS_TIERS_V1`, `METRICS_RING_V1`, `METRICS_FILES_V1`, `METRICS_SOURCES_V1`.
- Rust: `read_series(&Path, from_ms, to_ms, Option<&[String]>) -> Result<Series, MetricsError>`; `Series{from_ms,to_ms,metrics:Vec<(String,Vec<Point>)>}`, `Point{start,width,tier,count,min,max,sum,last}`; errors bilingual.

## Format decisions (all in the schema)
- Per tier file: 64 B header (magic SMRG, version, tier, bucketMs, bucketCount, metricCount, ids length, row bytes, data offset, FNV-1a digest, head cursor, created) + ids table (self-describing, so readers need no catalogue) + ring of rows `[start u64][36 B slot per metric][start u64]`. Torn row = head/tail stamp mismatch. Head cursor is written after the row; readers advance it while the next row is valid and scan only if the head row is invalid.
- Fold exactness: a tier n+1 window is written when the first row of a later window arrives; rows are written before they are folded upwards; restart rebuilds open windows from the rows on disk; rewriting a row is idempotent. Proven equal to a Python fold of the raw samples for 3 seeds with gaps and ring wraps, and equal between one run and a run with reopens every 5 samples.
- Merge: finest tier first; each coarser tier only rows ending no later than the oldest row taken from a finer tier. A coarse bucket that straddles that boundary is dropped, so a gap of up to one coarse bucket can separate two tiers (see golden series: 12 s between t2 and t1 with 16 s buckets).
- Lag: tier n+1 shows a window only after the next window started; the recent part is always answered by the finer tier, so the dashboard sees no hole.
- Caveat: a crash between a row write and the head write with a silence gap before it hides that one newest row from readers until the writer reopens (the writer rescans); one sample second after a restart can be dropped (same window as the last written row).

## Proof
- `repo test-file 🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🧪️tests/🟦️.ts`: 11 pass, 0 fail (suite module-basics): catalogue laws (unique ids, known kind/unit/source, en+de labels, every design §1 metric, every declared source used, self metrics), tier laws, fold vs Python oracle, min/max/sum/count/last, restart equality, torn row / zeroed head / stale head, rejected samples + catalogue change + invalid tiers, merged series (no overlap, finer is newer, narrowing), counter deltas vs oracle (300 readings), golden (byte-identical TS writer output; TS reader and Python reader equal checked-in series for 6 queries), production sizes.
- `repo cargo --store=isolated --configuration=prototype -- test --manifest-path 🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/📦️packages/🦀️rust/Cargo.toml`: 4 pass (golden queries equal checked-in series, ordering/no overlap, narrowing and bilingual errors, torn row and head scan).
- dangling-imports exit 0 for the member and the suites folder.

## Numbers (bun 1.4.2, macOS, 60 metrics, 7300 simulated 1 s ticks, page-cache pwrite, no fsync)
- write cost per tick: mean 10.3 us, median 8.0 us, p99 44.5 us, max 184 us (non-boundary 10.4 us, minute boundary 9.9 us); at 1 Hz = 0.001 % of one core. Bytes written per tick: 2184 (one t0 row) + 8 (head).
- file sizes (60 metrics): t0 653,992 B; t1 784,552 B; t2 3,134,632 B; t3 2,194,600 B; t4 4,701,352 B; total 11.5 MB. With the full 69-metric catalogue the rows are 2,508 B (t2 about 3.6 MB, total about 13.2 MB).

## Not verified / for other lanes
- Windows and Linux runs (code uses only node:fs positional read/write and std::fs; not run there). No fsync is issued (process-crash safe, not power-loss safe by design).
- Taxonomy/filename laws of the repo (new emoji folder names, `.ring`/`.tsv` fixture names) were not run; `repo verify` style gates were not available in this lane.
- M3: set the real `source` collectors for `local-cache`, `graph-service`, `scheduler` (declared in the schema, no collector code yet); feed counters through `createCounterDeltasV1`; one writer per directory.
- M4: the Rust reader is a path crate outside the root workspace; add it as a dependency of the dashboard by path (its `Cargo.toml` has its own `[workspace]`, so a root member entry would conflict: either keep it standalone or drop that table when adding it to the root).

## Round 2: no gap at tier boundaries
- Chosen: clipped partial points (the aligned-start alternative cannot close the gap, because the finer tier has no rows before its ring horizon). A coarse bucket that contains the start of the finer coverage (the oldest row taken from the finer tier, when not aligned to the coarse bucket) is kept once, with the statistics of the whole bucket, `partial = 1` and `width` clipped to the part before that start; the part after it is shown by the finer tier, so points neither overlap nor leave a gap. It chains over every tier pair.
- Point is now `[start, width, tier, count, min, max, sum, last, partial]` (TS tuple, Python TSV column 10, Rust `Point.partial: bool`, schema `Point` with the rule in its description). `start` stays aligned to the tier bucket; `width` is below the bucket only when partial.
- Golden `series.tsv` regenerated (8 partial points, e.g. a t2 row at 1800000080000 with width 12000). TS, Python and Rust agree on all 6 queries. New oracle case: steady data over 150 s gives contiguous points, partial points keep whole-bucket count/sum, Python equals TS.
- Proof: `repo test-file` metrics test 12 pass, 0 fail; Rust `process-metrics-golden` 4 pass before the last cosmetic edit (an unused `Chunk.width` field removed to silence a warning); that final cargo re-run was queued behind host load and not yet confirmed; dangling-imports exit 0.
- A gap can still appear where data is truly absent (silence, machine off), never merely from a tier boundary.
