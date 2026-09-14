# 📓️ Opus executor report — `🔨️modules/📊️metrics`

Scope: the new `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics` module (L2 of the plan's DAG) —
Rust crate, Go package, JSON Schema, four language-agnostic test cases with a real-`git` oracle, nx
bridges, launch entries.

Sources read: the frozen snapshot's `🔢️LOC Command` region (`🗑️generated/go-snapshot/client/🧩️component.go`
8451–9559) and `🔮️Benchmark Command` region (37387–37584), and the TypeScript library's unified-LOC
counters (`🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` ~3110–3525).

## 1. What was built

| Path | Content |
| --- | --- |
| `📊️metrics/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}` | crate `semio-framework-repo-metrics`, lib at `🦀️.rs`, deps `serde`/`serde_json` (workspace) + path deps on `semio-framework-repo-workspace` and `semio-framework-repo-model` |
| `📊️metrics/📦️packages/🐹️go/{go.mod, 🐹️.go, 📋️project.json, 📜️script.ts}` | `module github.com/usalu/semio/repo/metrics`, `go 1.25`, zero external deps |
| `📊️metrics/🧬️schema/🔣️.json` | draft 2020-12 schema for `GitTranscript`, `CommitDelta`/`FileDelta`, `LocReport`/`LocLangStats`/`LocHistoryEntry`, `TimeBucket`/`TimeBucketGroup`, `BenchmarkResult`/`BenchmarkRow`/`BenchmarkSummary` |
| `📊️metrics/🔮️oracle/🔣️.json` | oracle registry contribution (see §4) |
| `📊️metrics/🧫️fixtures/*` | 7 recorded fixtures, all produced by the real `git` binary |
| `📊️metrics/🧪️tests/{numstat-parsing, loc-aggregation, time-bucketing, benchmark-summary}` | 4 cases, 13 scenarios, Rust + Go subjects, TypeScript oracle on the first two |
| root `Cargo.toml`, root `go.work` | crate and module registered |
| `.vscode/🧩️launch.seed.jsonc` → `.vscode/launch.json` | 6 entries in group `4_gate`, orders 425.21–425.26 |

The ticket also carries the fixture recorder `🏗️record-metrics-fixtures.ts` (an input script, kept).

## 2. Public Rust API (`semio-framework-repo-metrics`)

Re-exports: `LineMetrics` (from `semio-framework-repo-model` — it is exactly the added/removed pair
the numstat parser needs), `find_repo_root` and `GitIgnore` (from `semio-framework-repo-workspace`).

```
// 🏷️ Vocabulary
DEFAULT_BRANCH, AGG_CODE, AGG_MARKUP, AGG_DATA, AGG_TOTAL, DEFAULT_CODE_LANGUAGES, NUMSTAT_PRETTY
numstat_log_args(git_ref) -> Vec<String>
normalize_repo_path(path) -> String
classify_loc_bucket(path) -> Option<&'static str>          // the coarse `loc` table buckets
classify_language_fine(path) -> Option<&'static str>       // the library's per-language buckets
make_lang_set / make_numstat_lang_set(&[String]) -> BTreeSet<String>
classify_for_numstat(path, &weights) -> Option<String>
path_has_hidden_segment / path_is_repo_meta(rel) -> bool
path_skipped_for_loc(rel, Option<&GitIgnore>) -> bool
to_json_string<T: Serialize>(&T) -> Result<String, String>

// 📏️ Counting
physical_line_count(&[u8]) -> usize
count_json_keys(&serde_json::Value) -> usize
json_key_count(&[u8]) -> usize
count_unified_loc_for_file(rel, &[u8]) -> usize

// 🧩️ Numstat
struct FileDelta { path, rename_from, added, removed, binary, bucket }
struct CommitDelta { sha, when_unix, author, author_mail, files, delta: BTreeMap<String, LineMetrics> }
unquote_git_path(&str) -> String
resolve_numstat_path(&str) -> (String, Option<String>)
parse_numstat_log(stdout, &weights, Option<&GitIgnore>) -> Vec<CommitDelta>

// 📊️ Report
struct LocLangStats { loc, percent, since_prev_loc_percent, wip_percent, edited, added, removed }
struct LocHistoryEntry { sha, date, author, languages, by_contributors }
struct LocReport { snapshot, by_contributors, history, branch }
type Cumulative = BTreeMap<String, LineMetrics>;  type CumulativeByContributor = …
default_contributor_alias(name, email) -> String
cumulative_from_raw(&[CommitDelta], &languages, by_contributor, filter, &alias) -> (Cumulative, Option<…>)
stat_from_pair_and_scan / apply_percents / sum_edited_pairs / apply_wip_percents
compose_loc_report_snapshot(&Cumulative, Option<&scan>, &languages, wip_denominator) -> BTreeMap<String, LocLangStats>
sorted_row_keys / sorted_row_keys_churn / use_full_tree_table
by_contributors_to_snapshot / pct_loc_since_prev / history_entry_stats / apply_history_loc_since_prev
display_history_branch / history_checkpoint_label / contributor_emoji_id

// 🕰️ Time
enum TimeBucket { Commit, Hour, Day, Week, Month, Year }
civil_from_unix(i64) -> (i64, u32, u32);  unix_days_from_civil(y, m, d) -> i64;  iso_weekday(i64) -> i64
format_rfc3339_utc(i64) -> String;  bucket_key(i64, TimeBucket) -> String;  bucket_start(i64, TimeBucket) -> i64
struct TimeBucketGroup { key, start_unix, commits, delta }
bucket_commits(&[CommitDelta], TimeBucket) -> Vec<TimeBucketGroup>

// 🔌️ Git source
trait GitLogSource { numstat_log(ref); tracked_paths(ref); tracked_bytes(ref, rel) }
struct SystemGit  (std::process, the real binary)
struct GitTranscript { id, logs, tracked, blobs }  + from_json / blob_key   (impl GitLogSource)

// 🧮️ Pipeline
snapshot_loc_counts(&dyn GitLogSource, ref, &languages, Option<&GitIgnore>) -> Result<BTreeMap<String, i64>, String>
build_history(…) -> Result<Vec<LocHistoryEntry>, String>
struct LocOptions { languages, history, by_contributors, branch, contributor }
build_loc_report(&dyn GitLogSource, &LocOptions, Option<&GitIgnore>, &alias) -> Result<LocReport, String>

// 📤️ Render (pure; colour and the CLI verb stay in ⌨️cli)
markdown_table / render_markdown / text_table / render_text

// ⏱️ Benchmark
BENCHMARK_LANGUAGES;  struct BenchmarkResult { test, lang, time }
parse_benchmark_output(lang, output) -> Vec<BenchmarkResult>
parse_duration_seconds(&str) -> Option<f64>
struct BenchmarkRow { test, timings, fastest };  struct BenchmarkSummary { tests, languages, rows }
summarize_benchmarks(&[BenchmarkResult]) -> BenchmarkSummary;  benchmark_csv(&[BenchmarkResult]) -> String
```

The Go package mirrors this one-for-one with Go names (`ParseNumstatLog`, `BuildLocReport`,
`SummarizeBenchmarks`, …); `Ignorer` is a one-method interface instead of a concrete `GitIgnore`, so
the Go package needs no dependency on `🏠️workspace` at all.

`locCommand`, `runLocCommand`, `benchmarkCmd` and `runBenchmark` are deliberately **not** here —
per fix 4 of `📓️go-region-dependency-graph.md` they are CLI wiring and belong to `⌨️cli`.

## 3. Decisions taken (and why)

### 3.1 "Unified LOC" — Go and TypeScript already agreed, so the definition was adopted unchanged
Go `locCountBucketLoc` (snapshot 8694) and TypeScript `countUnifiedLocForFile` (`🟦️.ts:3342`) are
the same function: recursive JSON object-key count for `.json`/`.jsonc`, falling back to physical
lines when the document yields zero keys and is not blank; physical lines otherwise. The two line
counters also agree exactly — Go's `bytes.Count(data, "\n") + 1` and TypeScript's
`text.split(/\r?\n/).length` both equal *newlines + 1* for a non-empty body. **No divergence to
resolve**; the rule is now named `count_unified_loc_for_file` / `CountUnifiedLocForFile` in both new
implementations and is pinned to the library by the `loc-aggregation` case.

### 3.2 Bucket classification — two projections, one counter
Go classifies into the coarse `loc` table (`TypeScript, Go, C#, Python, Rust, Markup, Data`);
TypeScript's `classifyPathForMetrics` names each format (`JSON`, `YAML`, `Markdown`, `HTML`, …,
plus `Dockerfile`/`Makefile` by basename). These are **different reports, not a disagreement** —
the `loc` table has fixed aggregate rows, the library's dashboard does not. Both are exposed
(`classify_loc_bucket` and `classify_language_fine`), both computed from the same extension table,
and both feed the same per-file counter. Documented in the crate docstrings.

### 3.3 Skip rules — Go's semantics are canonical for `loc`
Go skips `.🧬semio`, gitignored paths and any dot-prefixed segment. TypeScript additionally skips
lock files, `LICENSE.*` templates and vendor directories (`node_modules`, `dist`, `target`, …).
Those extra exclusions are a *dashboard* policy: `loc` counts **tracked** files, and a committed
lock file is legitimately part of the tree. The domain implements Go's rule; the library's extra
exclusions were not adopted. Gitignore matching stays injectable (`Option<&GitIgnore>` / `Ignorer`)
so the module keeps no hard dependency on `🏠️workspace` in Go.

### 3.4 Two corrections to the Go parser, proven against real git
The snapshot's `locParseNumstatLog` (8974) has two defects that the recorded transcript exposes:

1. **Renames are misclassified.** `diff.renames` is on by default, so git writes
   `2\t1\t"🧬️alpha.ts" => "src/🧬️beta.ts"`. The Go parser takes `parts[2]` whole, so the path it
   classifies is `"…alpha.ts" => "src/…beta.ts"` — extension `.ts"` — which matches no bucket, and
   the rename's lines silently vanish from the report.
2. **Non-ASCII paths are never decoded.** Under `core.quotepath` git octal-escapes every non-ASCII
   byte inside a double-quoted field, so every emoji-named file in *this* repository reaches the
   parser as `"\360\237\247\254\357\270\217alpha.ts"` — again extension `.ts"`, again uncounted.
   The repository's own tree is overwhelmingly emoji-named, so the current `loc` numbers understate
   it substantially.

Both are fixed here (`resolve_numstat_path`, `unquote_git_path`) and both are covered by the
`resolves-renames-and-quoted-paths` scenario against the live `git` binary. Binary rows (`-`/`-`)
are kept as records with `binary: true` and zero counts instead of being dropped, and a merge
commit is emitted as a record with an empty file list rather than being lost.

### 3.5 The Go adapters import `…/repo/metrics`, not `…/repo/client`
The brief asked for the adapters to import the current client module with a
`// 🚚️ repoint …` marker. That is not possible: **every** `loc*` symbol in the godfile is
unexported (`locParseNumstatLog` at `💻️client/⌨️cli/🧩️component.go:8994`; only the `LocLangStats`,
`LocReport`, `LocHistoryEntry`, `LocCumulative` and `BenchmarkResult` *types* are exported), so a
test host in another module can call nothing. The behaviour was therefore ported into the new Go
package now, the adapters import `github.com/usalu/semio/repo/metrics`, and the marker comment is
carried at the top of each Go adapter explaining that the repoint is already done. The
`📊️Pending` region in `🐹️.go` lists every godfile symbol, its snapshot line, and its new name, and
says explicitly which symbols the split **deletes** (already ported) and which stay in `⌨️cli`.

### 3.6 Oracle registry file location
The brief named `📊️metrics/🔣️oracle.json`. The harness discovers a contribution at
`<owner>/🔮️oracle/🔣️.json` (`discoverTestContributions`, `🧪️test/📦️packages/🟦️typescript/🟦️.ts:903`,
`taxonomy.testContributionDirName = "🔮️oracle"`), which is also what every existing contributor in
the repository uses. The manifest was written there so it is actually loaded.

### 3.7 Case directory names carry no emoji prefix
`validateCaseContract` tests the case directory name against
`taxonomy.testCaseSlugPattern = ^[a-z0-9]+(?:-[a-z0-9]+)*$`, so the four cases are named
`numstat-parsing`, `loc-aggregation`, `time-bucketing`, `benchmark-summary`.

## 4. Evidence

### 4.1 Oracles (`📊️metrics/🔮️oracle/🔣️.json`)
- **`git-numstat-cli`** — `kind: third-party-cli`, ecosystem `javascript` (so it runs in the TS
  adapter). The oracle does **not** read the committed transcript: it rebuilds the recorded
  repository from `shared://🌱️repository-recipe.json` in a temp directory with pinned authors and
  `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE`, runs the real `git log --numstat`, and answers from that.
  So the transcript's fidelity is re-proved on every run, and the subjects are compared against git
  itself.
- **`semio-library-uloc`** — `kind: cross-semio-implementation` (an honest supplement, not a
  qualifying oracle): the TS adapter for `loc-aggregation` computes every per-file LOC with the
  library's own `countUnifiedLocForFile`, so the library and the two domain implementations are
  pinned to one another. `productionDebt` records that the library counter is production-reachable
  and that it disappears when the library starts calling the domain implementation.
- **`repo-metrics-time-bucketing`** and **`repo-metrics-benchmark`** — recorded no-oracle decisions
  with `specification-vectors` + `independent-implementations` (+ `metamorphic-laws` for the date
  algebra); both cases carry two independently written subjects, which is what the harness requires
  for their `@mode-differential` scenarios.

### 4.2 Fixtures (`📊️metrics/🧫️fixtures/`, all recorded from real git)
`🌱️repository-recipe.json`, `🎞️git-transcript.json` (3 log streams, 6 tracked listings, 37 blobs),
`📤️numstat-deltas.json`, `📤️loc-report.json`, `📤️time-buckets.json`, `⏱️benchmark-output.json`,
`📤️benchmark-summary.json`. The recorded repository deliberately contains: emoji filenames, a
`git mv` rename across directories, a binary PNG, a `.hidden/` directory, a `.🧬semio/🦑️repo/` path,
a file deletion, a JSON document that grows keys, two authors, a side branch and a `--no-ff` merge.

### 4.3 Commands actually run (output pasted verbatim)

Build both implementations:
```
$ cargo build -p semio-framework-repo-metrics
   Compiling semio-framework-repo-metrics v0.1.0 (…/📊️metrics/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 4.50s

$ bun ./…/📊️metrics/📦️packages/🐹️go/📜️script.ts build
(no output — success)

$ cargo test -p semio-framework-repo-metrics
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests semio_framework_repo_metrics
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ bun ./…/📊️metrics/📦️packages/🐹️go/📜️script.ts test
?   	github.com/usalu/semio/repo/metrics	[no test files]
```
(The behaviour is proved by the Protocol v2 cases, not by in-package unit tests — see §5.)

Fixture recording, against the real `git` binary:
```
$ bun ./.🧬semio/…/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🏗️record-metrics-fixtures.ts
[record] wrote 7 fixture(s) to C:\git\semio\…\📊️metrics\🧫️fixtures
[record] 5 no-merge commit(s), 6 with merges, 37 blob(s)
```

Subject + oracle + parity, all four cases (`RUSTC_WRAPPER=""`, level `quick`):
```
$ for c in numstat-parsing loc-aggregation time-bucketing benchmark-summary; do
    bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" parity quick --case "$c"; done
### numstat-parsing
[test] level=quick cases=1 executed=12 passed=12 failed=0 errored=0 parity=12/12
### loc-aggregation
[test] level=quick cases=1 executed=12 passed=12 failed=0 errored=0 parity=12/12
### time-bucketing
[test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
### benchmark-summary
[test] level=quick cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2
```
`numstat-parsing` and `loc-aggregation` run 4 scenarios × (rust subject, go subject, typescript
oracle) = 12 executions with 12/12 parity verdicts against the oracle; `time-bucketing` and
`benchmark-summary` have no oracle, so their verdicts are the go × rust cross-subject pairs.

A deliberate negative control was observed on the way: before the fixture URI in the feature text
was fixed, both subjects projected `null` while the oracle projected the full git-derived answer and
all six parity verdicts failed — i.e. the comparison really is doing work.

Contract phase, whole repository:
```
$ bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" contract
5302 high-priority breach(es) across 6 rule(s)   (all pre-existing, none in 📊️metrics)
$ grep -c "📊️metrics" <that output>
0
```

Launch regeneration:
```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) …
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
$ grep -n "metrics" .vscode/launch.json
8883:  "name": "⚖️gate🧪️test📊️metrics🧮️numstat",
8894:  "name": "⚖️gate🧪️test📊️metrics📈️loc",
8905:  "name": "⚖️gate🧪️test📊️metrics🕰️buckets",
8916:  "name": "⚖️gate🧪️test📊️metrics⏱️benchmark",
8927:  "name": "⚖️gate📊️metrics🦀️rust🧪️test",
8938:  "name": "⚖️gate📊️metrics🐹️go🧪️test",
```

## 5. Notes for the next executors

- **`go-split`**: the `📊️Pending` region in `📊️metrics/📦️packages/🐹️go/🐹️.go` is the move list.
  Everything in its first group is already ported and must be **deleted** from
  `💻️client/⌨️cli/🧩️component.go`, not moved; `locCommand`, `runLocCommand`, `benchmarkCmd`,
  `benchmarkDryRun` and `runBenchmark` stay in `⌨️cli` and must be rewritten to call
  `metrics.BuildLocReport` / `metrics.RenderMarkdown` / `metrics.RenderText` /
  `metrics.ParseBenchmarkOutput` / `metrics.BenchmarkCSV`. `LocLangStats`, `LocReport`,
  `LocHistoryEntry`, `LocCumulative` and `BenchmarkResult` exist in **both** places until then.
- **`cli` (wave 3)**: `SystemGit`/`SystemGit{Repo: root}` is the production `GitLogSource`; the
  contributor alias is injected as a closure so `FindAndUpdateContributor` stays in
  `🧑️contributors`; terminal colour and TTY detection stay in the renderer, which is why
  `render_text` emits plain text.
- **`📚️library` owner**: `countUnifiedLocForFile` is now duplicated between the library and the two
  domain implementations, and `📊️metrics/🔮️oracle/🔣️.json` records that as production debt with a
  plan. When the library starts calling the domain implementation, delete the
  `semio-library-uloc` registry entry together with the duplicate.
- **Not done here** (out of scope or deliberately deferred): the fixed-width `render_text` /
  `TextTable` renderer is implemented in both languages and unit-shaped, but has no
  language-agnostic scenario — the markdown renderer does. Adding a text-table scenario means
  writing a third fixed-width formatter in the TypeScript oracle; the column widths were taken
  verbatim from the snapshot (`%-14s%8d%6.1f%%…`, 9420) and match between Rust and Go by
  construction, but that has not been executed as a test.
- **Not done here**: `by_contributors` / `--by-contributor` filtering and the contributor history
  variant are implemented and serialised in both languages, but the language-agnostic cases only
  exercise the non-contributor path (the recorded repository's two authors would need a contributor
  registry to alias meaningfully). Worth a follow-up case once `🧑️contributors` lands.
- **`materializeGoHost`** already grew SUT wiring (`goWorkspaceModules` + per-module `replace`)
  while this work was in flight, so the gap flagged in `📓️harness-verification.md` §2 is closed;
  the Go adapters here import `github.com/usalu/semio/repo/metrics` and resolve.

## 6. Files created or changed

Created:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🐹️go/{go.mod, 🐹️.go, 📋️project.json, 📜️script.ts}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🔮️oracle/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧫️fixtures/{🌱️repository-recipe.json, 🎞️git-transcript.json, 📤️numstat-deltas.json, 📤️loc-report.json, 📤️time-buckets.json, ⏱️benchmark-output.json, 📤️benchmark-summary.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/numstat-parsing/{🥒️.feature, 🦀️.rs, 🐹️.go, 🟦️.ts}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/loc-aggregation/{🥒️.feature, 🦀️.rs, 🐹️.go, 🟦️.ts}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/time-bucketing/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/benchmark-summary/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🏗️record-metrics-fixtures.ts`
- this report

Changed:
- `Cargo.toml` (workspace member added)
- `go.work` (module added)
- `.vscode/🧩️launch.seed.jsonc` and the regenerated `.vscode/launch.json`
