# 🧪️ Result RENDERER-TEST-BINS (slice `rtb`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/rtb-tools/`. Logs and census outputs: `F/🗑️generated/rtb-*`. Scratch (snapshots, pre-edit copies): the session scratchpad `rtb/`.

## 0. Verdict (rewritten after each milestone)

IN PROGRESS (milestone 1 of 6 done: baseline measured, source readers and the three named fixtures relocated). Nothing is compiled; every claim below states the static check that backs it.

## 1. Baseline (before the first edit)

`bun F/rtb-tools/rtb-census.ts` (copy of `otcp-census.ts`, output prefix `rtb-census-`; `MANIFESTS=<top manifest> DUMP_INPUTS=1`), top crate `semio-framework-os-renderer-wgpu`, saved in `F/🗑️generated/rtb-baseline/`:

| unit | native | wasm32-unknown-unknown | wasm32-wasip2 |
|---|---|---|---|
| lib (normal) | 13 files / 115,612 B / 1,169 lines | n/a | n/a |
| lib-test | **592 / 9,312,360 / 38,108 (35.5x)** | **521 / 8,074,999 / 23,678 (30.8x)** | 2 / 3,619 / 68 |

The 592 files are 93 source files plus 499 embedded inputs (356 `.rs` source texts of the source-text laws, 140 JSON fixtures, 2 GLB, 1 TOML). The dev-dependency rows `semio-framework-ui` (31 files / 1,109,244 B / 19,483 lines, ratio 4.87, direct and normal) and `semio-framework-plugin` (125 / 1,979,150 B, ratio 7.55, transitive and normal) are over the gate on every label; they are normal dependencies of the top crate, not test closures (foreign: the peer UI and plugin carves).

(The remaining sections are written as the milestones land.)
