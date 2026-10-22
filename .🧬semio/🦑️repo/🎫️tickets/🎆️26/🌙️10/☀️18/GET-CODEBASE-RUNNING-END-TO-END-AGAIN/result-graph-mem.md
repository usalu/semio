# result-graph-mem: bounded memory for the cold graph build (wave 1020e, executor graph-mem)

READY (staged, NOT applied; `sh C/graph-mem-tools/go.sh --write` after the coordinator's GO): `apply.py check` on the live tree: 22 files pending (6 anchored edits of live files with 16 operations, 16 new files), 0 drift, longest new path 139 UTF-16 units. Nothing outside `C/` and `/private/tmp/graph-mem/` was written. All runs were niced, one heavy process at a time, RSS bound 3.9 GB, load guard 30 after the coordinator's HOLD/RESUME.

READY2 (audit blockers cleared, 10-21 09:5x; staged, NOT applied): `apply.py check` on the live tree: 22 files pending, 0 drift. Re-based on peer edits made since (project-nodes `collector.guard`, `graphImplementationUrlsV1`, engine `implementation` digest). Audit `audit-staged-1021a.md` blockers: **#7** the three Python docstrings now start with an emoji (`🐍️` and `✅️` in `memory/tests/🐍️oracle.py`, `🧲️` in `owner-claims/🐍️oracle.py`). **#9** no default language: the language is the one named by `SEMIO_LANGUAGE`, the dashboard locale or the system locale; when none is named (or only unsupported ones) every supported language is printed, English line first and German line second (`selectGraphMemoryLanguagesV1`, `renderGraphMemoryMessagesV1`; fixture rows `nothing-set-prints-both-languages`, `unsupported-only-prints-both-languages`, scenario `unresolved-language-prints-both`, oracle updated). **#12** schema-first: `B/🧠️memory/🧬️schema/🔣️.json` is now the source; `🟨️.mjs` imports it (`with {type:"json"}`) and freezes `GRAPH_MEMORY_POLICY_V1` and `GRAPH_MEMORY_MESSAGES_V1` from it (policy gained `languages` and `languageEnvironment`); the schema file joins the graph module authority list. Re-verified on a fresh overlay: memory law 6 (8 scenarios), owner claims 2, retention 5, shared strings 1, governed dispatch 1, generations 24, load 36, environment 20, protocol 42, sealed reuse 13, phases 12, record header 2 pass; observation and closure-facts fail exactly like an unpatched overlay of the same live tree (one observation test timed out once under load 25-40 and passed alone); mutation proof 17 of 17 killed; `tsc` 0 diagnostics from this set (the 4 reported in `🕸️graph/🟦️.ts` are the peer `🔁retry` import and an untyped parameter at line 444, present on the live tree).

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`, `G` = `B/🕸️graph`, `C` = this ticket. New member `B/🧠️memory/`.

## 0. Verdict (read this)

1. **The full 3,735-project number is NOT proven.** A full cold build cannot run inside my 4 GB cap and chain16/17 occupy the host. What is measured: (a) the inference loop on 388 real projects, (b) the whole post-inference pipeline (shape, generation refresh, assemble) at 1,200 and 2,400 owners built from the real persisted records. Extrapolation to 3,735 is linear for the pipeline (reliable) and an estimate for inference (first-touch lumps, see 3).
2. **Pipeline after inference (measured, real records, real engine):** live code holds 0.96 MB of managed heap per owner after assemble (1,277 MB at 1,200 owners, 3.6 GB extrapolated to 3,735) and peaks at 2.9 GB RSS at 1,200 owners; at 2,400 owners the live code exceeds my 3.9 GB RSS bound (aborted). The staged set holds 0.43 MB per owner (644 MB at 1,200; 1,116 MB at 2,400 which completes with a 3.07 GB RSS peak; extrapolated 1.6 GB heap at 3,735). Live objects 1.68 M -> 0.90 M, shaping CPU 4.3 s -> 1.1 s (the quadratic owner claim scan, about 175 s CPU at 3,735 owners, becomes about 4 s).
3. **Inference (measured on 388 projects, stride sample, 8 workers):** managed heap after the loop 959 MB (live) -> 824 MB (staged), live objects at 350 projects 1.52 M -> 1.29 M, one forced full GC 442 ms -> 173 ms, peak RSS unchanged (2.35 GB vs 2.31 GB, dominated by one-off first-touch work). The gain is smaller than the pipeline gain: the dominant remaining growth is first touch of new parts of the repository (library closure, TypeScript analyses, rows), not retention of finished projects. The late marginal slope is 0.4 MB per project (staged) against 0.7 (live); the early lumps (1.5 MB per project over the first 350 projects of a stride sample) saturate with repository coverage. Whether the whole inference stays under 3 GB is therefore decided by the governor (below) and the first live run; the staged set removes what could be removed without changing semantics.
4. **The budget is enforced, not hoped for.** A new project inference starts only while the managed heap is within the budget (default 3,072 MB, 50 percent of a host-capacity grant, env `SEMIO_GRAPH_MEMORY_MB`, minimum 1,024 MB); above it the loop collects once and waits, retained memory that a collection cannot release degrades the run to one project at a time (never a deadlock), with EN/DE progress lines and cancellation.
5. **RSS is about 2.3 times the managed heap** in every measurement (allocator retention after transient garbage); the host-capacity reservation for `show`/`graph` (5,120 MB in `B/🔣️policy.json`) must therefore NOT be lowered on the strength of a 3 GB heap.
6. **Already live from a peer (not repeated here):** the cold pass publishes a snapshot like every delta pass (`snapshot:true`, applied 00:14 by conc-graph `snapshot-fix.py`). That removes the outer `observeInference` record of ALL projects, whose `JSON.stringify(result)` alone was a 430+ MB string plus header, buffer and brotli copies, and the whole-inference restart rounds. I checked the live code: `invocation/🟨️.mjs` and `discoverBuildGraphPartV1` carry the flag.

## 1. Where the memory went (evidence)

| finding | evidence |
|---|---|
| Every inferred project kept its serialized image (`Buffer`) in `projectImages` forever | clear experiment: 23 MB / 108 projects (0.21 MB each, 0.8 GB at 3,735) |
| Every returned project value was a fresh `JSON.parse` tree: equal long strings (input path lists, 95 percent duplicates: 76,036 strings, 3,825 distinct in 98 records) were separate instances | retained values 37 MB / 108 projects live, 3 MB after sharing (0.34 -> 0.03 MB each); micro: 2,000 parsed documents 58 MB -> 3.8 MB |
| The stored fragments and fact index were parsed back without sharing, so assemble rebuilt the duplication for 3,735 projects | assemble delta 616 MB -> 280 MB at 1,200 owners |
| `shapeComputeResultV1` scanned all distinct rows for every owner (3,735 x 183 k startsWith) and copied each owner's row array | shape CPU 4.3 s -> 1.1 s at 1,200 owners; quadratic in the owner count |
| Expanded input lists (`"{workspaceRoot}/" + file` per entry per closure) created one string per entry | now one instance per distinct path (see 2, not measured end to end) |
| Concurrency was bounded by CPU only (up to 8 projects), never by memory | the 40 percent GC marking share of the 10-20 minute diagnosis (graph-speed section 7) |
| One-off: the `✏️s/🧑‍💻dev` project's TypeScript derivation holds about 800 MB (171 k rows, 11 k settled source buffers up to the 128 MB bound, TS program state) after its 47 s | trace: heap 211 -> 671 MB, extra 153 -> 521 MB at project 22; unchanged by this set |

## 2. What is staged (`C/graph-mem-tools/staged/ops.json`; 16 anchored edits in 6 live files + 16 new files)

| piece | files |
|---|---|
| New member `B/🧠️memory/`: pure admission decision, governor (queue, spacing, rate-limited collection, hysteresis, forced serial mode, abort), bounded string interner, budget resolution, EN/DE texts, closed schema, declarations, law test with independent Python oracle | `🟨️.mjs`, `🟨️.d.mts`, `🧬️schema/🔣️.json`, `🧪️tests/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,🐍️oracle.py}` |
| Observation: project values share equal long strings (`internJsonStringsV1`), the serialized image of recent projects is bounded to 32 MB (oldest out; a repeat then reads the persisted record), stats expose `projectImageBytes` | `B/👁️observation/🟨️.mjs` (+ `.d.mts`) |
| Project nodes: every project inference starts through `governor.run`, budget from the environment/grant, statistics in the `inference` phase row | `G/📥️inference/📦️project-nodes/🟨️.mjs` |
| Generation load: owner claims through a sorted path range lookup, no row array copy when nothing is added | `G/🕰️generations/🔌️load/🟨️.mjs` |
| Generation engine: stored fragments and the fact index share strings on read | `G/🕰️generations/🟨️.mjs` |
| Graph host: expanded input lists share strings, the memory module joins the module authority list | `G/🟦️.ts` |
| Laws: owner claims (60 seeded worlds vs naive scan), retention (observation), shared strings (engine), governed dispatch (AST law on project-nodes) | `B/🧪️tests/🕸️graph/🧲️owner-claims/`, `B/👁️observation/🧪️tests/🧠️retention/`, `G/🕰️generations/🧪️tests/🧶️shared-strings/`, `G/📥️inference/📦️project-nodes/🧪️tests/🚦️governed-dispatch/` |

Cross-platform: `node:v8`, `node:fs`, `Bun.gc` with `globalThis.gc` fallback; the sampler and collector are ports. Policy and texts come from the schema (`🧠️memory/🧬️schema/🔣️.json` is the source, the module only reads and freezes it). No new dependency.

## 3. Measurements (`C/graph-mem-tools/{bench.ts,scale-bench.ts,measure.sh}`, raw in `/private/tmp/graph-mem/measure/`)

Inference loop (`bench.ts`: 388 projects, 15 percent Cargo-only, `✏️s/🧑‍💻dev` excluded, 8 workers, forced GC at checkpoints):

| | live | staged |
|---|---|---|
| managed heap + extra after the loop | 959 MB | 824 MB |
| at 50 / 150 / 250 / 350 projects | 381 / 690 / 874 / 945 MB | 363 / 636 / 786 / 826 MB |
| live objects at 350 | 1.52 M | 1.29 M |
| forced full GC at 350 | 442 ms | 173 ms |
| peak RSS | 2,349 MB | 2,312 MB |
| CPU / wall | 152 s / 166 s | 155 s / 178 s |

Pipeline (`scale-bench.ts`: synthetic owners drawn from the persisted records of 97 real projects, 40 unique rows each, real `shapeComputeResultV1`, `createGenerationStoreV1.refresh`, `assemble`):

| owners | | heap after assemble | peak RSS | objects | shape CPU |
|---|---|---|---|---|---|
| 1,200 | live | 1,277 MB | 2,914 MB | 1.68 M | 4.3 s |
| 1,200 | staged | 644 MB | 1,987 MB | 0.90 M | 1.1 s |
| 2,400 | live | aborted at 3.9 GB RSS | | | |
| 2,400 | staged | 1,116 MB | 3,073 MB | 1.49 M | 2.4 s |
| 3,735 | staged, first version (before the no-copy claim and the bounded image) | 1,523 MB | 3,451 MB | | 4.5 s |
| 3,735 | staged, final | aborted at the 3.9 GB RSS bound (load 40, RSS noise between runs of up to 30 percent) | | | |

Fact index at 3,735 owners: 173,831 distinct rows (2 percent), 41 MB JSON, 8 MB packed; fragments 277 MB JSON, 13 MB packed.

## 4. Verification

- New laws on the overlay with the set applied: memory 6, owner claims 2, retention 5, shared strings 1, governed dispatch 1 (all pass). Existing suites with the set applied: generations 23, load 26, environment 20, generation protocol 33, sealed reuse 13, graph phases 12, record header 2 pass. Observation 41 pass / 11 fail and closure-facts 16 / 3 fail: **identical failing sets on an unpatched overlay of the same live tree** (overlay environment: real ancestry of `🛍️products/💻️os`, `smol-toml`, fixture roster), so no regression; an intermediate version of my image change broke "semantic parent validation shares each provider" (6 reads instead of 2), which is why the image is bounded instead of dropped.
- Mutation proof (`mutation-check.py`): 17 of 17 killed (boundary, worker bound, admission spacing, forced beside running, hysteresis, interner copy / nested arrays / bound, claims prefix start / order / key dedupe, image always read / unbounded, no interning, fragments not interned, ungoverned dispatch, dropped statistics). First round had 4 survivors; two were equivalent mutants, two weak laws, all fixed.
- `tsc --noEmit` (`typecheck.py`) over `G/🟦️.ts`, the memory declarations and all new TypeScript laws: 0 diagnostics in touched files (14 pre-existing elsewhere).
- Independent oracles: Python model of the decision, budget, language and messages and a trace checker for governor runs (arrival order, worker bound, start only within budget beside running projects, liveness, collection rate limit, spaced progress lines, serial line once); Python naive scan for owner claims; JSON Schema (Ajv) closes every fixture.
- Overlap check against pending sets (`apply.py check --root` on control vs patched overlay): cli-latency 31 ops, pm-seam 48 ops, input-precision, hot-cache (0 problems) show no new refusal; conc-graph revision-fix touches only the revision module. Old applied sets (graph-speed, conc-graph) report drift on their own anchors only because they are already applied and I changed the line they replaced.

## 5. Not verified, risks

- No live run, no full 3,735 run; inference extrapolation is an estimate (section 0.3). The first live cold build decides; `SEMIO_GRAPH_PHASES=1` now prints `memory:{waits,waitedMilliseconds,collections,peakMegabytes}` in the `inference` row.
- The governor uses `node:v8` heap statistics (2.6 ms per call, sampled at most every 25 ms). On Linux/Windows the same API is used; not run there.
- Applying changes the observation revision (every project record is invalid once, cold) and the graph module authority (graph receipts invalid once): apply only when no graph build is running (`apply.py` refuses while `repo show/exec/dependencies/tools setup` runs); run `bun repo tools setup nx` afterwards.
- `Bun.gc(true)` is a synchronous full collection (about 0.2-0.5 s on a 1 GB heap); the governor collects at most every 2 s, once per release, and never again within 30 s after a collection that retention proved ineffective.
- Interned strings stay alive for the process (bounded to 64 M characters, cleared when exceeded).
- Observation `projectImages` entries older than the 32 MB bound read the persisted record on a repeated round: behavior equal, a little slower.

## 6. Apply / rollback / follow-ups

Apply: `sh C/graph-mem-tools/go.sh --write` (regenerates fixtures and ops, checks, applies, runs the laws against the live tree). Acceptance: `sh C/graph-mem-tools/measure-after-go.sh` (one `show projects` with phase rows and the worker's peak RSS). Rollback: reverse the replacements in `staged/ops.json`; the new member is removable (nothing else imports it after the reverse).

Follow-ups not staged: (1) hold per-project rows as block digests in memory (fact table) instead of 9 k references per project (about 260 MB at 3,735) and let the fact index group by block; (2) canonical row keys as digests (about 80 MB); (3) stream `pack(index)` (41 MB JSON, about 150 MB transient) and lazy `portableInputs` for consumers that never read inputs; (4) bound `scripts.files` / `settledReads` by the budget; (5) the one-off 800 MB TypeScript derivation of `✏️s/🧑‍💻dev` should release its program after persisting (input-precision owner).
