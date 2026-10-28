# Handover 2026-10-27 (coordinator session 1027a) → next session

Ticket: `2026/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` (C = this folder). Do NOT close it. The repo MCP failed to connect this session (no ticket tool used). Rules unchanged: `brief-wave-1026g.md` + addenda, HANDOVER-1026g.md "Resume prompt". Work order below replaces 1026g's.

## Landed this session (as gate, each with a product verdict; details + revert commands in `coordination.md` 00:12–02:2x)

| Batch | Verdict | Revert |
|---|---|---|
| stale-g owner-child (landed 23:57 by the previous session) | g11 course:build 231 s, course:test 90 s, print-rs:test 508 s GREEN | `python3 -I C/stale-g-tools/stage.py revert` |
| witness2-g (git ignore sources, remote-FS refusal, symlink vouch, root identity, exact ps argv) | g12 course/hot/dash GREEN | `python3 -I C/witness2-g-tools/stage.py revert` |
| racy-tools: witness flagged `racy` for SKIPPED stat targets → no hot plan was ever sealed; `racy` now names the file (`result-racy-c.md`) | g14 GREEN | `python3 -I C/racy-tools/edit.py <root> revert` |
| jobskey-tools: granted job count (`request.parallel`, `--parallel=N`) out of the plan-reuse key | g17 GREEN | `python3 -I C/jobskey-tools/edit.py <root> revert` |
| grantenv-tools: `SEMIO_HOST_CAPACITY_GRANT/_PLAN` are authorityEnvironment | g19 GREEN | `python3 -I C/grantenv-tools/edit.py <root> revert` |
| ignoredenv-tools: ignoredEnvironment names leave no `e` row (the planner spreads process.env); + `SEMIO_BUILD_PARALLEL`, `SEMIO_HOST_CAPACITY_PHASE_MARKS`, the 3 graph-yield vars | g20/g21 GREEN | `python3 -I C/ignoredenv-tools/edit.py <root> revert` (then remove the 3 yield names by hand) |
| yieldenv-tools | SUPERSEDED (see its SUPERSEDED.md; never run its revert) | — |

Result: **course hot no-op 20 s with `[bazel] plan reused`** (was 60–83 s on 10/26). The witness revert order is reverse of the table (they touch the same witness files).

## Reverted this session
- **quietfc** (rebased onto witness2, mirror-verified, laws green live) → g13-dash `repo-dashboard-rs:build` starved 45 min: the dashboard class asks to grow 2048 → 28672 MB (= whole machine) with 12–15 GB spare. The 12288 MB ceiling covers only the course class; the measured bound needs 3 runs with kept peaks, old samples have none. Fix owner quietfc-g: bound every class by its measured peak (dashboard 4.6–4.8 GB) or a generic fallback, law with the g13 recording. Staged batch is already rebased (`f029.pre-witness2` kept; `verify.sh` takes `QFC_ROOT`; run laws with mirror `--no-seal`: the gate law "a process that only waits is lent" fails under the seal even on an unmodified mirror).

## Next steps (in order, ≤ 10 lanes, gate one batch at a time with a product build)
1. Hot no-op < 10 s: land boot-g `graph-memo` (graph planner child 5–7 s), then graphlat `reuse-once` + `plan-once2`, then rustfast; measure with `gate-g-tools/product.py` hot ×3 (never with `SEMIO_REPO_PROFILE=json`: it reruns ~28 s of course actions every run).
2. Findings to fix: agent env (CLAUDE_*) leaks into action keys (shell runs rerun actions vs product.py's cleaned env); lease-order warning `bazel-slot-gate` under a held graph snapshot on every hot run; dashboard replans on every run when alternating with course (slot holds one plan?).
3. quietfc re-land after the generic bound (above).
4. keys4 precision-rules: re-run its oracle evidence (count drift after pathmod rust-mounts).
5. Measure: course one-line edit (< 60 s), print one-line edit (< 2 min), peer edits / 3 agents (harnesses `peer-g-tools/`, `agents3-g-tools/`).
6. Course e2e (`e2e-f-tools/final.sh`), `repo-tickets-rs:test`, `repo-graphql-go:test`; reboot proof of the journal/tools-image fix.
7. Perf backlog per `pregate-1026g.md` (gram3, gram4, seal2, disk-g, red-g, obsrows2, fdleak, sweep2, bench3 observer, docs3).

## Open user decisions (unchanged)
Delete `🧰️framework/🤖️generated/process` (193 MB); delete stray module-dir `.cache` roots (16.2 GiB, `result-disk-g.md`); fixture tickets written by Go tests.
