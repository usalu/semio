# 🧭 HANDOVER 10-25 (waves 1024d + 1025e)

Written 10-25 12:45 CET by the coordinator (weekly usage 85 %, limit expected ≈ 15:30 CET; it will cut every agent mid-run). Resume from here, then `coordination.md` from "## 15:55 (10/24" and each lane's `result-<prefix>.md` (HANDOVER sections at their tops).

## User scope
Dashboard (`@semio-tech/repo-dashboard-rs`), architecture course (`@teaching/architecture-course`), print (`print`, `print-rs`, `presentation(-react)`, bericht, `praesentation-projektetage`) end to end; at 04:30 10-25 the user asked to saturate the fleet (max agents without hitting the 5-h limit, reach the weekly limit early).

## State per product (verdicts; claims-d found every GREEN before 03:10 predates the [DEBUG] sweep → `result-fin-d.md` is the current consolidated table)
- Dashboard: Bazel build + all dashboard test targets incl. journeys 35/35 GREEN (dash3-d, term-d); launcher `repo dashboard` = `repo run @semio-tech/repo-dashboard-rs:run`, EN/DE TUI, tasks, cancel, stop proven; Nx: build, test-execution, daemon status GREEN; open: one daemon timing law under load (dflake-d), 50 no-circular edges (knot-d).
- Course: dev topology e2e 76/76 (course-d); pets 200 % text: 5 consecutive focused passes (cpets-d 10:36); rehearsal topology: entry 239,674 B < 250,000 after lazy loading, 62 specs passed then pets ladder (crel2-d taking over); warm build ZERO re-executions (cinp-d); non-e2e closure GREEN on Bazel (ctest-d, cui-d); Nx typecheck/build/test GREEN.
- Print: every Bazel target GREEN (print-d, pres-d); default route (Nx) 12/13 GREEN, `print:test` red only through 37-min build churn (nx4-d narrowing inputs).

## Shared fixes of note (details in result files)
products image P0 (pimg-d), fact index 537 → 36 MB (idx-d), Nx faults K-S (nx-d/nx2-d/nx3-d), churn restart + grant handover (churn-d/churn2-d, reviewed by authrev-d: 2 defects fixed), lock drift preflight (deps-d, lockfix-d), cruiser gate retired + lint-dependency-direction 2903 → 1027 edges (verify-d, expd-d), language catalog 0 findings, [DEBUG] sweeps (dbg-d 128 files, dbg2-d 276 files, both verified).

## Lanes running at 12:45 (agentIds in coordination.md)
cpets-d, fin-d, expc-d, plugc-d, plugc2-d, plugc3-d, dbg2-d, ext-d, exact-d, tsc-d, gen-d, jobw-d, crel2-d, cache-d, knot-d, nx4-d, dflake-d, bound-d, emoji-d, purity-d.

## Pending user decisions
- Chromium re-download for `repo tools verify` (asked 10-25 09:05).
- Behaviour change to confirm: an `Absolute` child in a wrapped/grid container now sits at the container origin like the flow path (layout-d, owned solvers replaced taffy).

## Close-out checklist (when the user says done)
Kill nothing of others; each lane removes its own [DEBUG]; delete `🗑️generated` only at ticket close (≈ 50+ GiB, 41 GiB store-c); `ticket_close` with explicit path and ASCII-first files array; clear `📌️important.md`.

## Update 14:15 CET (weekly 94 %, limit imminent — every lane will be cut mid-run)
- Closed since 12:45: Chromium decision (all 20 pinned tools match, no download; stale PLAYWRIGHT_BROWSERS_PATH removed from ~/.zshrc + ~/.bashrc with user OK); tsc-d (13 strict configs 0 errors); expc-d (process 0 / print 0 / course 5 dependency-direction edges); cpets-d (pets 200 % text fixed, 5× + whole pets project PASS); emoji-d (docstring laws 0 in the 3 closures, ~2,079 files, NOT compiled → re-verify); cache-d (caching route table 297 files, 208 green / 85 red, official runs queued); gen-d (warm proof blocked by cad Cargo.lock wasm-bindgen 0.2.126 vs pin 0.2.129; queued `repo cargo … update --offline` pid 88982 left as handover; ~/.sinc incremental copy holds a ~60 GB image, disk free 61 GiB).
- NEW REGRESSION (course dev e2e 48/1/27): `[shortage] a reload in the middle of a run keeps the answers` lands on the overview after reload → creload-d bisecting (suspects: ext-d owned i18n engine, crel lazy loading, expc-d facades).
- Machine: load 25-40, capacity queue > 90 min, swap ~6.8/7 GB — after the reset start with ≤ 10 lanes, finish verdicts first (fin-d table, print:test Bazel + Nx, course e2e both topologies, cache-d official runs), then resume the long-tail lanes.
- Lanes alive at 14:15: fin-d, plugc-d, plugc2-d, plugc3-d, dbg2-d, ext-d, exact-d, jobw-d, crel2-d, knot-d, nx4-d, dflake-d, bound-d, purity-d, creload-d (each has a result file; most have HANDOVER sections).
- 14:35 RISK: purity-d moved 24 repo Rust crates (169 file moves) with only static/typecheck verification; its `cargo check --tests` of the 48 repo crates is queued (pids 26384/26386). FIRST after the reset: `bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:build` + `:test-unit`; fix any breakage at the source (see result-purity-d.md move list).
