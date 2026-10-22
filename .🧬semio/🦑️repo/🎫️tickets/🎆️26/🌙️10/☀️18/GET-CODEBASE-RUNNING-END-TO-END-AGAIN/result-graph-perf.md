# result-graph-perf (wave 1022a, executor graph-perf): the graph profile of 10-22 and what was fixed

(The earlier 10-20 report of the previous `graph-perf` executor is kept as `result-graph-perf-1020.md`.) Profile with all numbers: `graph-perf-profile-1022.md`. Tools: `graph-perf-1022-tools/` (`timed.sh` + `stamp.py` + `treecpu.py` + `phases.py` = stamped phase timeline with CPU seconds per phase and peak tree RSS of any repo command, `inf.sh` + `nodes-run-pinned.mjs` = the real project inference over N live projects in a scratch cache, pinned like production). Generated output: `🗑️generated/graph-perf-1022/`, `/private/tmp/gp1022/`.

## Status

- 01:35-03:10. Measured, three fixes live, profile written. Nothing committed, no git writes, no sub-agents, no process of mine left running (the cold `show` runs ended by themselves).
- A real end-to-end cold graph build could NOT be measured: both attempts that got past the dependency stage died on loader refusals unrelated to graph code (`typescript` require not declared in `script-inputs`, 02:48; `Captured require declaring module required` for `⚡️caching/🦀️cargo/📜️script.ts`, 03:05). The cold timings below are therefore from the real inference modules in the harness, not from `bun repo`. Open for the loader owner.

## 1. Changed files (L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`)

1. `L/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`: cargo lock owner closure (about 1,900 literal paths) pushed into every native target now goes through `internCommandSources`. 38 sampled projects 39.0 -> 2.2 MB of JSON (-94 percent), whole tree projected 3.8 GB -> 0.24 GB of project values. Output identity: `projectsSha 7fda3403a3877b58` equal between the in-memory variant and the live edit. Law (new): `L/🏗️build/🕸️graph/📥️inference/🧪️tests/🗜️interned-closures/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}`, `bun test ./…/🟦️.ts`: 3 pass.
2. `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: `publishJavascriptDependencyExportsDescriptorV1` walks with nested descriptors (O(depth) opens per directory instead of O(depth^2)), fences every 64 entries, device read once, rewrites applied afterwards in fresh handles (<= 8 per handle). Micro-benchmark (`/private/tmp/gp1022/walk/bench.ts`, generation `1e0a3bf…`, 71,222 entries): 31 CPU-s / 101 s -> 11.6 CPU-s / 32 s. Existing law "projected workspace edges beyond the descriptor binding bound…" passes (80 expects) and a new law "…found in every directory of the generation, however deep, and only there" (same file `…/💾️storage/🧪️tests/🟦️.ts`, Python oracle) passes: `SEMIO_TEST_ARTIFACT_DIR=<dir> bun test ./<file> -t "projected workspace edges"`: 2 pass. Real sync gain not measured (a sync needs a manifest change); expected 63 -> about 15-20 CPU-s per scope.
3. `L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts`: stale-receipt log line prints `detail`; closure files that come from the admitted images get stamp "0" (no more `implementation-unsettled` for them). WRITTEN BUT UNVERIFIED (module laws need the repo test target). Told slice-play (they asked) that I own this.
4. Tools only under `C/graph-perf-1022-tools/`: `timed.sh`, `stamp.py`, `treecpu.py`, `phases.py`, `inf.sh`, `inf-ws.sh`, `nodes-run-pinned.mjs`, `patch_workspace_edges.py` (the anchored form of edit 2).

## 2. Before / after

| item | before | after |
|---|---|---|
| project JSON, 38 sampled owners | 63.1 MB (23.9 MB on 10-21) | 2.2 MB |
| per-target `inputs` of a native target | 1,911 entries, 519 KB | 9 entries + 1 named group |
| produced-workspace-edge walk, 71k entries | 31 CPU-s, 101 s wall | 11.6 CPU-s, 32 s wall (+ fewer fences) |
| inference CPU per owner (pinned, harness) | 0.45 s | 0.45 s (the fix saves memory and serialization, not inference CPU) |
| `bun repo` entry cost, stale receipts | 100 s wall, 68 CPU-s, 3 GB | unchanged; a receipt-valid run took 130 s wall, 111 CPU-s up to the loader refusal (queue excluded) |
| cold dependency sync of one command | 2,505 s wall, 1,372 CPU-s | not re-measured |

## 3. Open, in order of value (details and numbers: `graph-perf-profile-1022.md`)

1. Verified-block memo (graph-parallel round 6 piece): persist + read-back validation is 53 percent of the pinned inference loop; expected about -45 percent CPU per project. Needs integrate3 to apply graph-parallel; `python3 C/graph-parallel-tools/apply.py check` shows 0 drift against the live tree (31 files pending) as of 03:00 even with my edits.
2. Dependency implementation closure: shrink it to the walk logic (today a live edit of any of hundreds to about 2,000 files forces every command through a complete walk, 20-60 CPU-s, or a 20 CPU-min sync that holds `javascript-dependency-build` exclusively). Also: a command waiting for the dependency build should release its machine-admission grant.
3. Repeated `fence()` re-reads in the producer (`store-ownership`, `javascript-preserving-source`, `store-input-replay`: 182 + 86 + 43 events, 520 CPU-s in one sync).
4. `nativeSources.projects` list repeated per target (190 KB of a 229 KB project): intern per project if every consumer resolves it from a named input.
5. Closure growth 697 -> 1,902 files per command script is the multiplier behind rows (5.4k -> 13.6k per project), JSON and CPU; the loader's wider capture should be bounded or the script layering fixed.
6. Capacity: smaller first grant for readers + upgrade before the builder lease (needs a warm-path measurement).
7. Loader refusals above (two new ones within 20 minutes).
