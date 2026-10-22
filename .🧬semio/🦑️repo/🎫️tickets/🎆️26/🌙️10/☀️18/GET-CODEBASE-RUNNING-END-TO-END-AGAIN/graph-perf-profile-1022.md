# Graph profile 10-22 (executor graph-perf, wave 1022a)

Host: macOS arm64, 10 cores, 32 GB, load average 20-47 from the fleet during every run, so CPU seconds are quoted; wall seconds are inflated 2-10x by contention and by the machine-wide capacity queue (`show` and `graph` each reserve 5,120 MB, `run` 3,584 MB, above a 6,553 MB floor: 90-200 s of queue per command was normal). `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Raw data: `🗑️generated/graph-perf-1022/` (stamped stderr `*.err`, tree CPU/RSS samples `*.cpu.jsonl`, `timings.jsonl`, `cold.phases.txt`), `/private/tmp/gp1022/` (harness outputs, CPU profiles). Tools: `graph-perf-1022-tools/`.

## 1. Where one `bun repo show projects --json` spends its time before the first project is inferred (warm: nothing to rebuild)

Stamped timeline of 100 s wall, 68 CPU-s, 2.8-3.2 GB tree RSS (runs `show-projects-4/5`, tree churning, receipts stale):

| phase | wall | CPU | what |
|---|---|---|---|
| capacity queue | 0-190 s | 0 | memory floor, 3-4 grants running |
| `build:source-admission` | 17-24 s | 7.3-8.1 | loader captures the implementation closure (about 1,000+ files, hash, import scan) |
| dependency receipt miss: `parser-source` walk 1 (state, list 130k operations, frontier, provider, verify) | 35 s | about 30 | full Bun workspace ownership walk, because no receipt matched |
| walk 2 (replay could not confirm, second complete walk) | 30 s | about 20 | |
| `build:discover` | 8-9 s | 11.6-14.7 | repo walk of about 78k directories (cold generation only) |

When a manifest or lock changed the same command does a complete dependency sync before it reaches the graph. Run `show-projects-cold`: 2,505 s wall, 1,372 CPU-s (22 CPU-min), 3.2 GB, still not at the graph when it ended in a loader refusal (`typescript` require not declared). Per-scope hot spots of the sync (6 scopes, sums over the run): `dependencies:workspace-edges` 378 CPU-s (63 per scope), `store-ownership` 194, `store-input-replay` 164, `javascript-preserving-source` 160, `javascript-source-provider` 114, `javascript-installed-paths` 84, `bun install` children 20. This holds `javascript-dependency-build` EXCLUSIVE, so every other repo command of the fleet queues behind it (observed 13-40 min per holder, 6-8 waiters holding idle 3.5-5 GB grants).

Receipts never survive the fleet: `captureGenerationReceiptV1` defers with `implementation-unsettled` and a receipt is only reusable by a process whose dependency-implementation closure (hundreds of files, 246 in the last published receipt of 10-21 16:47, about 2,000 reported today) is byte-identical, so one live edit in that closure costs the next command a complete walk.

## 2. Cold project inference, measured on live projects (harness: the real `emojiProjectJsonNodes`, scratch cache, 0 workers; `inf.sh`; pinned like production = `nodes-run-pinned.mjs`)

| run | sample | inference CPU | per owner | project JSON | observation rows |
|---|---|---|---|---|---|
| frozen tree of 10-21 17:50 (unpinned) | 125 owners | 46.5 s | 0.37 s | 23.9 MB / 38 owners | 712k |
| live 10-22 02:20 (unpinned) | 125 owners | 95.3 s | 0.76 s | 63.1 MB / 38 | 1,832k |
| live, pinned (production) | 54 owners | 24.4 s | 0.45 s | 63 MB / 38 | 743k |
| live + lock-closure interning, pinned | 37 owners | 16.5 s | 0.45 s | 2.2 MB / 38 | 517k |

Findings:

1. The graph grew 2.6x in one day: 38 sampled projects 23.9 MB -> 63.1 MB. Cause: every native target's `inputs` carried the literal import closure of the cargo lock owner script, 697 -> 1,902 paths (the closure of command scripts grew 2.7x with the loader's wider source capture), repeated for 12 targets of 2,140 crates. A trinity-rewriting target was 203 KB, now 519 KB; 5.2 GB of the 3.8 GB-class graph was this duplication. Fixed (section 4).
2. After that fix 61 percent of what remains is the `{input:"nativeSources", projects:[...736 names]}` entry, repeated per target (190 KB of a 229 KB project). Not fixed: it is the Nx-shaped dependency-input form and I could not verify that every consumer resolves it from a named input.
3. Production-like CPU profile (pinned, 55 s total, 30.6 s in the inference loop): `observeInferenceComputationV1` 16.3 s (53 percent) of which `persistInferenceBlockV1` 14.7 s and its read-back/validation `readObservationBlockV1` 12.4 s; `observationJsonV1` 3.7 s self (char-by-char node/depth count in JS before `JSON.parse`), `retainObservationsV1` 3.3 s, `sort` 2.6 s, sha256 2.6 s, TypeScript parse 3.5 s (first touch), owner discovery 5 s (once). Every block of every record is read, inflated, hashed, parsed and validated again to prove it exists. The verified-block memo staged by graph-parallel (leaf identity = dev, ino, mode, links, size, both ns stamps) removes this; graph-parallel measured 81 -> 163 owners/min. Not applied live yet (graph-parallel is in integrate3's round 6, after cache-gc apply4). In an UNPINNED process the same path also re-hashes the storage consumer's source files on every observation path (32 percent), which production avoids through `withImplementationPinV1`.
4. Rows per project rose 5.4k -> 13.6k with the closure growth (about 270 rows per closure file, replayed into every project record, 2 percent distinct). Linear in closure size; the closure of a small command script (about 1,900 files) is the multiplier behind CPU, rows, JSON size and memory.
5. Extrapolation to 3,737 projects, single thread, pinned, quiet host: 0.45 s x 3,737 = 28 CPU-min before the fix of finding 3 and the farm; the 55 min / 0.28 projects per s seen on the loaded fleet is CPU contention plus memory pressure (GC bound) on top of this. With finding 3 (about -50 percent) and the farm on 6 free cores: 6-10 min.

## 3. Items measured and handed over

- Dependency sync (section 1): fixed the walk of produced workspace edges (section 4); the rest (store-ownership/replay/provider/preserving-source repeated per scope by `fence()`) is lock-sync/boot-fast territory.
- Receipt closure size and `implementation-unsettled` deferral: partly fixed (section 4), closure shrink open.
- `bun repo` fixed entry cost 68 CPU-s when receipts are stale: no quiet-host figure could be taken in this wave (a peer edited a closure file during every run).
- Capacity: every graph reader reserves 5,120 MB although a lean `show` peaked at 2.6-3.3 GB tree RSS in these runs; idle waiters hold grants (observed by slice-play). Not changed: needs a warm-path measurement to size a smaller first grant plus an upgrade before the builder lease.

## 4. Fixes applied live (all anchored, re-read before editing; no git writes)

1. `L/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`: the lock-owner closure goes through `internCommandSources` (one named input per project). Project JSON of 38 owners 39.0 -> 2.2 MB, `projectsSha` identical between the in-memory variant and the live edit. Law: `L/🏗️build/🕸️graph/📥️inference/🧪️tests/🗜️interned-closures/` (TypeScript AST over the live source plus 5 fixture cases, `bun test`: 3 pass).
2. `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` `publishJavascriptDependencyExportsDescriptorV1`: the walk opens each directory from its parent descriptor (nested `withDirectory`) instead of re-proving the whole path from the store root per directory, fences every `fullFenceInterval` (64) entries, reads the store device once, and applies the rewrites afterwards in fresh handles of at most `maximumBindingRewritesPerHandle`. Micro-benchmark on a finished generation (71,222 entries, 10,297 directories, 4,544 links): old 31 CPU-s / 101 s wall, nested 11.6 CPU-s / 32 s (without the fence change). Existing law (independent Python link oracle, 80 expects) passes; new law "every directory, however deep": passes.
3. `L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts`: (a) the stale-receipt log line carries `detail` (it was an opaque `unreadable`); (b) closure files from the admitted images get stamp "0" so a young live mtime no longer defers the receipt (their sha256 is what ran). WRITTEN BUT UNVERIFIED: the laws of that module need the repo test target (bare `bun test` fails with "Actual admitted Source resolution map required", before and after).

Tools to reproduce: `graph-perf-1022-tools/timed.sh <label> <cmd>` then `phases.py <label.err> <label.cpu.jsonl>`; `inf.sh N tag` (with `HARNESS=…/nodes-run-pinned.mjs`).
