# W5 — `🛢️db`'s `LiveQuery` dissolved into the inference spine

Boundary: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs` (crate `semio-framework-os-kernel-db`).

## 0. Pre-flight: crate compile status (mandatory re-verify)

The crate was previously recorded ~53-errors-broken, then later measured 0 errors. Re-verified myself before touching anything:

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os-kernel-db --all-targets
```
→ `Finished dev profile [unoptimized] target(s) in 1.93s`, **0 errors**, 85 warnings (pre-existing lint noise, not touched). Saved: `scratch-w5-db-baseline-check.txt`.

Baseline test run (forced, not cached — this was the crate's first touch this session):
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-kernel-db --lib
```
→ **402 passed; 21 failed**. Saved: `scratch-w5-db-baseline-test.txt`. All 21 failures share one root cause, `InvalidArgument("db_artifact wire error: truncated at offset 2")`, across `db_cli`/`db_engine`/`db_facade`/`db_testkit` — a pre-existing/concurrent issue in the wire-encoding path, **nothing to do with `LiveQuery`/`db_query`**. Not fixed (out of boundary; not caused by this wave — see `## Concurrent-churn observations`).

The crate was green (compile-wise) — no `blocked-churn` needed. Proceeded.

## 1. What changed

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs`

- `Value` (line ~47) and `RowId` (line ~532): added `serde::Serialize, serde::Deserialize` derives. Required because `LiveQuery`'s new `InferredField::Key`/`::Value` associated types must satisfy `Serialize + DeserializeOwned` (a static trait bound `infer_field`'s cache encoding needs, regardless of whether caching is enabled at runtime).
- `🔖️LiveQuery` region (line ~912 onward): added a new `🔖️QueryResultField` subregion:
  - `QuerySnapshot<'a> { rows: &'a BTreeMap<RowId, Value> }` — the `P` for the new `InferredField` impl.
  - `QueryResultField` (ZST marker) implementing `pack::InferredField<QuerySnapshot<'a>>`: `Key = RowId`, `Value = Value`, `FIELD_ID = "db.query.live-row"`, `SCHEMA_VERSION = 1`. **Roots-only `plan`** (`parents: vec![]` for every key, per the ticket's brief — a query result row depends only on its own queried columns, never on another row's result). `dep_input` = `encode_value` on the row's own `Value` (this crate's existing canonical binary encoding, reused as-is — not reinvented). `compute` = a lookup/clone of that same row from the snapshot (identity passthrough — see `## Honest mapping notes` below for why).
  - `LiveQuery` struct: added a `cache: pack::InferenceCache` field, initialized in `new()` with `InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }`.
  - `LiveQuery::refresh`: still calls the existing, tested, fallible `execute(...)` first (unchanged — see honest-mapping notes on why this could not itself move behind `InferredField::plan`), then routes the resulting rows through `pack::infer_field::<QuerySnapshot<'_>, QueryResultField>(&query_snapshot, Some(&mut self.cache))` instead of using them directly. The old `self.snapshot = new_snapshot` plus hand-rolled `BTreeMap` added/removed/updated comparison is now a comparison of `self.snapshot` against the spine-derived `new_snapshot` — same shape, but the per-row *values* are now sourced from `InferredField`/`DepHash`/`InferenceCache`, not computed ad hoc.
- Test module `db_query::tests::live_query`: extended (no new test file, per the ticket rule) with one new law test, `refresh_leaves_unrelated_rows_cache_warm_and_misses_only_the_changed_row`, under a new `🧪️IncrementalityLaw` subregion.

Grep anchors: `grep -n "QueryResultField\|QuerySnapshot\|struct LiveQuery\|fn refresh" 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs`

## 2. Honest mapping notes (per the ticket's "doesn't fit cleanly, say so" rule)

The mapping is real but not 1:1 with the puzzle3d pilot, for one specific reason worth recording precisely rather than glossing over:

`InferredField::plan`/`dep_input`/`compute` are **infallible**. `db_query::execute` is **fallible** — it owns the planner's `FullScan`/`FullTextPushdown` choice, the "pushdown planned but no `FullTextLookup` supplied" error, and `QueryLimits` enforcement (`max_scan_rows`/`max_result_rows`/`max_result_bytes`). Re-deriving that inside an infallible `InferredField::plan` would mean either (a) silently swallowing those error paths, or (b) duplicating `execute`'s already-tested planner/pushdown/limits logic a second time inside `plan` — both are the "forced bad fit" the ticket warns against, not a clean dissolve.

So `LiveQuery::refresh` still calls `execute(...)` first, unchanged, exactly as before. What moved behind `InferredField` is specifically the one piece that **is** a clean per-key derivation with no cross-row dependency: a result row's own cacheable content, keyed by `RowId`, dep-hashed off that row's own `Value` bytes. This is a narrower slice than "the whole `LiveQuery::refresh` pipeline," and it's deliberately narrower — the row-membership/filter/sort/paginate math stays where the ticket's own text says derived *values* belong (tier (c)), but the specific mechanism that decides row-set membership under a dynamic, arbitrary `Predicate`/`Select` genuinely cannot be expressed as a static, infallible per-key plan without reimplementing the planner.

One consequence worth being explicit about: because `execute()` already computes every result row's final value on every `refresh` call (it has to, to know which rows currently qualify), the `InferenceCache` here does not skip that upstream computation — `compute()` in `QueryResultField` is an O(1) lookup, not expensive work. What it **does** provide, and what the incrementality-law test below actually proves, is that the DepHash/cache bookkeeping is real and behaves correctly: an unchanged row is a cache hit, a changed row is a cache miss, and this is now expressed through the spine's generic mechanism rather than a private one-off `old_value != value` comparison — exactly the tier (a)→(c) reclassification the ticket asked for. If `compute` later becomes genuinely expensive (e.g. a heavier `Select::project` over deeply nested paths, or multiple `LiveQuery` instances sharing one `InferenceCache` over overlapping row sets), the savings become real for free, with no further changes to this call site.

`QueryDiff`'s own added/removed/updated comparison (`self.snapshot` vs. the new spine-derived snapshot) still exists in `refresh` and is **not** redundant with the cache's hit/miss bookkeeping — documented in the code (`LiveQuery`'s doc comment): a `DepHash` cache hit means "this exact row content has been seen before, at any point," not "unchanged since the immediately preceding refresh." A row that oscillates between two values would warm-hit the cache every other refresh while still being a genuine `updated` event each time relative to the immediately-prior snapshot. The two mechanisms answer different questions and both are needed.

`reads()` returns `&["*"]` — a `Query`'s `Predicate`/`Select`/`SortKey` paths are chosen dynamically per call, so no fixed field list would be honest, and `infer_field_after_diff`'s tier-1 `DiffRegions` gate is deliberately never invoked from this call site for the same reason (there is no static `D: DiffRegions` this crate could ground since `LiveQuery::refresh` doesn't have such a source of the touched region — it's driven by explicit `refresh()` calls, not diff-region gating). Only `pack::infer_field` (not `infer_field_after_diff`) is used.

## 3. Files touched

- Updated: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs`

No other files touched. No triads authored (this is explicitly a tier (a)→(c) reclassification per the assignment, not a mutation-authoring task — no verbs, no triads, no `#[path]` mounts touched).

## 4. Verification commands run, with real output

### 4a. Compile, after edit (forced recheck via `touch`)
```
touch 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os-kernel-db --all-targets
```
→ `error` count: **0** (`grep -c "^error" scratch-w5-db-after-edit-check.txt` → `0`). `Finished dev profile [unoptimized] target(s) in 4.55s`. Saved: `scratch-w5-db-after-edit-check.txt`.

### 4b. `LiveQuery` tests, targeted
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-kernel-db --lib db_query::tests::live_query
```
Real output (saved: `scratch-w5-db-livequery-test2.txt`):
```
running 3 tests
test db_query::tests::live_query::diff_applied_to_old_snapshot_reconstructs_new_snapshot ... ok
test db_query::tests::live_query::refresh_reports_added_removed_and_updated_rows ... ok
test db_query::tests::live_query::refresh_leaves_unrelated_rows_cache_warm_and_misses_only_the_changed_row ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 421 filtered out; finished in 0.00s
```
(First attempt without `record_stats: true` in `LiveQuery::new`'s cache config **failed** — `after.hits - before.hits` was `0`, not `3`, because `InferenceCacheConfig::default().record_stats` is `false`. Fixed by setting `record_stats: true` explicitly. Recorded here per the "never claim a test passed without running it" rule — this one genuinely failed once before it passed.)

### 4c. Full crate `--lib` suite, after edit, vs. baseline
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-kernel-db --lib
```
→ **403 passed; 21 failed** (saved: `scratch-w5-db-after-edit-fulltest.txt`). Baseline was 402 passed / 21 failed — the +1 pass is the new `refresh_leaves_unrelated_rows_cache_warm_and_misses_only_the_changed_row` test.

Diffed the two failing-test-name lists (stripped of non-deterministic thread ids):
```
awk '/^failures:$/{f++; next} f==2 && /^    [a-zA-Z]/{print}' scratch-w5-db-baseline-test.txt | sort > bnames.txt
awk '/^failures:$/{f++; next} f==2 && /^    [a-zA-Z]/{print}' scratch-w5-db-after-edit-fulltest.txt | sort > anames.txt
diff bnames.txt anames.txt
```
→ **empty diff** — byte-identical set of 21 failing test names, same files, same line numbers, before and after this wave's edit. No regression introduced.

## 5. Law test — incrementality, actually executed

`refresh_leaves_unrelated_rows_cache_warm_and_misses_only_the_changed_row` (in `db_query::tests::live_query`, see output in 4b above — real, executed, passing):

1. `refresh` over a 3-row source, then `refresh` again over the **identical** source: asserts an empty `QueryDiff` and `after.hits - before.hits == 3, after.misses == before.misses` — all three rows served warm.
2. `refresh` again with **only** the middle row's own field changed (same position, same row count — isolates a value change from a position-based added/removed event): asserts `diff.updated.len() == 1`, `diff.added`/`diff.removed` empty, and `after.misses - before.misses == 1, after.hits - before.hits == 2` — only the changed row misses; the other two stay warm.

This directly proves the requested law ("a change to a row's queried columns invalidates only that row's cached result; an unrelated row edit leaves the others' cache hits warm") against the real public `LiveQuery::refresh` path, reading `live.cache.stats()` (private field, same-module descendant access — same technique the file's own existing tests use for privately-held state).

## sharedFileRequests

None. No other lane's files touched or needed.

## Concurrent-churn observations

The 21 pre-existing `--lib` test failures (all `InvalidArgument("db_artifact wire error: truncated at offset 2")`, spanning `db_cli`/`db_engine`/`db_facade`/`db_testkit`) were present identically in the baseline run taken *before* any edit in this wave, and are unrelated to `LiveQuery`/`db_query::🔍️query`. Not investigated further (out of this wave's boundary — `📄️artifact`/`⌨️cli`/`⚙️engine`/`🧪️testkit` are not this wave's assigned file) and not fixed. Flagged here per the report-shape rule; the coordinator or whichever lane owns `db_artifact`'s wire path should be made aware if not already.

## Honest pass/fail

**Pass.** Crate compiles clean (0 errors, before and after). `LiveQuery` is now expressed as a thin `QueryDiff` adapter over `QueryResultField: InferredField<QuerySnapshot>`, routed through a real `pack::InferenceCache`/`DepHash` chain rather than a hand-rolled `BTreeMap` diff. The incrementality law was authored and actually executed, with real pasted output, and passes. All 3 `LiveQuery`-region tests pass; the 2 pre-existing ones are byte-unchanged in behavior. The 21 unrelated pre-existing failures are unchanged in identity before/after (proven via diff, not asserted). One honest caveat recorded in `## Honest mapping notes`: the row-membership/filter/sort/paginate/limits logic could not itself move behind `InferredField::plan` (infallible trait, fallible planner) without either losing error handling or duplicating `execute`'s tested logic — that part stays in `execute()`, unchanged, as it should.
