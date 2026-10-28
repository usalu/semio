# Wave 1026g brief (10-26 07:15 CET): the build system for many agents on one workspace

User (10-26 07:15): the build system must be ready for a single workspace with many agents: incremental builds, warm caches, recompiling blazingly fast on a hot cache, other agents editing the same files at the same time, robust, future-proof, performant and LOW-RAM — the target is a dev machine with 16 GB RAM (this Mac has 32 GB / 10 cores; design and measure for 16 GB). Products that must keep working end to end: dashboard, architecture course, print (all GREEN on Bazel tonight, see `C/coordination.md` 01:13-03:10).

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. ALL rules of `C/brief-wave-1025f.md` (incl. the rule files it names and its addenda; the freeze is LIFTED since 04:12) hold. Your result file is `C/result-<prefix>.md`, tools `C/<prefix>-tools/`, output `C/🗑️generated/<prefix>/`. Read `AGENTS.md` (schema-first, laws + independent oracle per feature, EN+DE texts, progress + cancellation, cross-platform macOS/Linux/Windows/devcontainer, no compat layers, concise code, docstrings start with a unique emoji).

## What tonight proved (facts to build on, read the result files)
- Graph: one builder custody `repository-graph-facts:<root>:shard:7` for all graph generations; graph-f added a verified coverage stamp (waiters served without rebuild, `🕰️generations/🔭️coverage`); a rebuild after framework edits re-infers ~566 projects (60-120 min, builder 5-13 GB RSS) because 3739 of 4020 projects contain build-library files in their command closure (infer-f); per-file analysis cache holds (87 % of changed files keep their analysis). infer-f's farm/progress changes and exec-f's `scheduling` scope are applied (04:12).
- nest-f: nested `repo run` inside orchestrated Bazel actions consume the parent generation via `SEMIO_BUILD_GENERATION_PIN` + inherited capacity grant.
- exec-f: action keys exclude scheduling code (course warm pair 0 re-executions after a scheduling edit); OPEN: cargo command closure covers all cargo-script commands.
- Capacity governor admits runs at 3.5-5.6 GB each above a 6.5 GB floor → on 16 GB only ~1-2 runs; per-run process chains are 5-7 bun processes (sh → bun repo → bun script → eval loader → graph child …).
- Storage: native DMG images under `.cache/b/*` (products `p`, graph `g`, nx `n`, diagnostics `o`, rust …); a test image mounted inside a managed parent killed every build (alloc-f fixed placement + EN/DE refusal; `tmp/silence-*` flake → ver-f).
- Churn: ENOENT on a source renamed mid-run kills a run instead of exit 75; Nx `Retained Graph snapshot descriptor changed` race (nx2-f).
- Disk: `.cache/b` ~150 GiB, ticket `🗑️generated` ~54 GiB, `~/.sinc` 125 GiB.

## Rules added this wave
- The machine is shared by ~20 agents: work GRAPH-FREE first (design, code, single-file `bun test ./<file>`, offline harnesses, micro-benchmarks on fixtures). An official graph-loading run only at the end of your item and only when `bun repo leases status | /usr/bin/grep -c WAITING` < 4; one at a time.
- Measure, don't guess: every performance claim comes with before/after numbers (wall, peak RSS of the process tree, cache hits) on a reproducible scenario, written into your result file.
- Edits to the shared build library land compile-atomic (new module complete + unit-tested, then the call-site switch in one edit). Re-read files right before editing; other lanes edit the same library live.
- Each lane owns the area named in its row; something outside your area → `FOR <prefix>:` line at the top of your result file.
- No sub-agents, no ticket tools, never a modifying git command, never pkill, kill only your own pids (lstart), never delete 🗑️generated folders or other people's files. Result file every ≤ 20 min; final message ≤ 8 lines.

## Targets (acceptance, measured on this Mac with a 16 GB budget simulated via the capacity policy)
- No-op `bun repo run <product>:build` on a hot cache: < 10 s wall, < 1.5 GB peak RSS for the whole process tree.
- One-line edit in a product source → rebuild re-executes only the affected actions; < 60 s for TS products, < 2 min for the dashboard Rust binary.
- Peer edits to unrelated files (incl. the build library's scheduling code) during or between runs: 0 re-executions, no failed run (churn → bounded re-observe or exit 75 + automatic retry).
- Graph generation after a typical peer edit: only affected owners re-inferred, < 2 min, builder < 2 GB RSS.
- Three agents running builds concurrently on 16 GB: no swap storm, no starvation (fair queue, progress visible to waiters, EN+DE).
- Disk: steady-state `.cache` under a declared budget with lock-aware GC.

## Addendum 07:33: ad-hoc scripts must be low-RAM
At 07:28 a lane's log-analysis Python script grew to 46 GB (swap 36.8 GB, disk free 32 GiB) reading `C/🗑️generated` (54 GB) into memory, and a finished lane's orphan walker held 8.8 GB for 4.5 h. Rules: every ad-hoc script STREAMS (line by line, `rg` pipelines, generators), never reads whole logs/trees into memory, checks its own RSS (`resource.getrusage` / `process.memoryUsage()`) and aborts above 500 MB; every tool call that starts a long script records its pid and kills it before the agent finishes. The coordinator kills runaways > 2 GB without asking.

## Addendum 10:10 (real clock): MEASUREMENT WINDOW 10:30–11:30
No honest on-tree baseline exists yet: the graph is never current because ~15 lanes edit the build library continuously, and official runs queue behind each other. From 10:30 to 11:30 (real clock, `date`):
- NOBODY edits files under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**`, `🧰️framework/🔨️modules/🏃️process/**` or `🚀️launcher/**`. Stage changes in `C/<prefix>-tools/staged/` (with their tests) and apply after 11:30 in one compile-atomic batch.
- NOBODY starts official/graph-loading runs, EXCEPT: bench-g (baselines, one at a time, first: no-op ×3 per product, then one-edit, then peer-edit, then the 3-agent scenario), proof-g (its queue, one at a time, interleaved with bench-g — coordinate via FOR lines; bench-g has priority), integ-g (one product cycle).
- Graph-free work (design, unit tests, single-file tests, oracles) continues for everyone.
- Before 10:30, land what is ready (compile-atomic) so the window measures it.

## Addendum 11:28 (real clock): FREEZE EXTENDED — products are red
bench-g's window found the graph broken for the products (generation never seals: `Every owner needs edges of the published environment: @semio-tech/async-timer-rs`; a stale negative parse result for `🪶️extent/🟦️.ts`; dashboard "Unknown build task"; a migration build blew up to 29 GB). Until `C/result-repair-g.md` line 1 says `PRODUCTS GREEN`, NOBODY except repair-g and gram-g edits `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**`, `🧰️framework/🔨️modules/🏃️process/**` or `🚀️launcher/**`, and nobody applies staged batches. No official runs except repair-g's. Graph-free work continues; keep staging.

## Addendum 12:25 (real clock): PRODUCTS GREEN 12:21 — freeze lifted through a GATE
repair-g restored all three products (dashboard build, course build, print-rs:test GREEN on Bazel, generations 545-547). The freeze is replaced by a landing gate: staged batches land ONE AT A TIME through lane `gate-g` (apply → verify laws → one official product build rotating course/dashboard/print → green: next batch; red: revert that batch and report). Lanes do NOT apply their own staged batches; gate-g does, in the order in coordination.md. New edits outside the staged batches: only compile-atomic, small, and announced in your result file; an edit to the graph modules (`🕸️graph`, `🕰️generations`, inference) needs a FOR gate-g line first.

## Addendum 13:58 (real clock): library edits ONLY through gate-g
Live edits by several lanes (lane2-g retention, nxret-g, xplat2-g) landed mid-run and broke/obscured gate-g's product verdicts (batch 3 went red on lane2-g's retention edit). From now on: every edit under `🧰️framework/🛍️products/🦑️repo/**`, `🧰️framework/🔨️modules/🏃️process/**` or `🚀️launcher/**` is STAGED (apply/revert/check script + verify script + base hashes) and handed to gate-g with a `FOR gate-g: batch ready <path>` line at the top of your result file; gate-g lands it in order with a product build. Exceptions: tsfix-g (type-error fixes) and a fix for a red you caused (announce it as FIX-OF-RED with the failing run).

## Addendum 18:00 (real clock): QUIET MEASUREMENT WINDOW 18:00–19:00
Only bench2-g starts official/graph-loading runs. Everybody else: finish or cancel your running official run (own pids), start no new one, keep working graph-free. gate-g pauses after its current product build. Window ends with "QUIET WINDOW END" in coordination.md.

## Addendum 19:12: forbidden commands (P0 data loss)

NO lane runs `repo ticket purge-artifacts` (any form) or `repo clean` until purge-g's guard has landed. aud-sweep-g (`audit-sweep-1026g.md`) found that both can delete tracked files and live `🗑️generated` folders of OPEN tickets; the 17:16 loss of 8,024 files and the 17:45 lane wipe fit them. Keep important evidence in your md files, not only in `🗑️generated`.

## Addendum 20:00: shared mirror tool

Verify staged batches ONLY on `C/mirror-tools/mirror.py` mirrors (copy-on-write, no symlinks into the live tree, macOS sandbox denies live writes, guard exits 97 on any live change). Names `<lane>-<what>` max 8 chars; drop only your own mirrors by name. No more symlink-farm overlays (two write-through incidents today).

## Addendum 20:10: bun test paths

Every `bun test` argument MUST start with `./` or be absolute. A bare relative path is a FILTER: bun scans the whole checkout (465k files) and holds ~123k directory fds, which pushed the host to ENFILE twice today (17:43, 20:07) and killed measured runs.

## Addendum 20:56: gate priority

The landing gate is the bottleneck: no group has landed since 18:46 (6b reruns died twice, then waited 20+ min behind 5 runs). RULE until 23:00: while gate-g has a product build queued or running (`gate-g-tools/run/g*-course.log` growing, or a gate HELD lease), NO other lane STARTS a new official `bun repo run`; runs already executing may finish. Measurement lanes (agents3-g, peer-g, bench3-g, rustedit-g, keys4-g, oracle2-g) start runs only in the gap right after a gate verdict line, one at a time. agents3-g: stop t1 (pre-quietfc numbers are low value) and wait for `GATE quietfc LANDED`.
