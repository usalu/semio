# Build 9 Timeline Audit (fresh-9.log)

Read-only audit of the cold semio-tech play release build #9 (`build-fresh`, started 2026-10-12 08:17:48, stopped 16:35). Source: `.🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator/fresh-9.log` (945.9 MB, 7,714,515 lines). Status file: `fresh-9.status` (exit 1 at 16:35:25).

## Method

- Streamed parse of every line. Each line is timed by the nearest preceding global heartbeat `[process:owner-command] running elapsedMs=<n>` (2,981 heartbeats, about 10 s apart, last at 29,815.5 s = 16:35:23). Resolution is therefore about plus or minus 10 s.
- A task is one `> nx run <project>:<target>` marker (205 markers, 66 projects). Its output is attributed by the `<project>: ` prefix. Start = marker time, end = last prefixed line, duration = end minus start. Zero-duration tasks finished within one heartbeat.
- Cross-check: the per-task heartbeat `running elapsedMs=` agrees with the measured duration within 10 s for every long task (for example demonstrator-plugin component-dev: 1260 s heartbeat, 1260 s measured).
- Cargo `Finished \`<profile>\` profile ... in <time>` lines are taken verbatim.
- Dependencies are inferred from start order. nx's own run summary is also quoted.

## Headline

- Wall time: first start 08:17:48, last task end 16:34:43 = 8h 16m 55s. nx's own summary reports run duration 496m 28s.
- The run is effectively serial. Exactly one task was active for 98.0% of the wall time, and the sum of all task durations (8h 21m 16s) is about equal to the wall time (average concurrency 1.01, overlap only at the start). The configuration requested `--parallel=4` (`🏢️semio-tech/🎡️play/🔨️modules/📦️site/🆕️fresh-build/🟦️.ts` line 18).
- nx's own summary: critical path 22m 46s (5 tasks), recoverable time 446m 40s (90% of the run). The serialization therefore does not come from the task DAG. It comes from an execution gate outside nx's graph.
- Wasm builds dominate: component-release 46.6% and component-dev 42.2% of task time, together 88.9%.
- Log volume: `[cargo-provenance]` read-progress lines are 3,283,180 lines and 418.2 MB (44.2% of the log). The outer `build-fresh` task buffered this output, and nx died with `RangeError: Invalid string length` (V8 string limit 0x1fffffe8 characters) while joining it. raster-plugin:component-release was still running and was stopped.

## 1. Task timeline (all 205 tasks, in start order)

| # | Start | End | Duration | Task (project:target) | Cargo Finished (profile, time) | Compiling lines |
|---:|---|---|---:|---|---|---:|
| 0 | 08:17:48 | 08:17:48 | 0s | `@semio-tech/semio-tech-play:build-fresh` | - | 0 |
| 1 | 08:18:18 | 08:18:18 | 0s | `@semio-tech/ui-styling-tokens:generate` | - | 0 |
| 2 | 08:18:18 | 08:18:18 | 0s | `@semio-tech/ui-rs:generate` | - | 0 |
| 3 | 08:18:18 | 08:18:18 | 0s | `@semio-tech/framework-schema:generate` | - | 0 |
| 4 | 08:18:18 | 08:18:28 | 10s | `@semio-tech/dsl-derive-rs:generate-schema-index` | - | 0 |
| 5 | 08:18:18 | 08:18:28 | 10s | `@semio-tech/framework-graph:generate` | - | 0 |
| 6 | 08:18:18 | 08:18:28 | 10s | `workspace:deps-wasm-opt` | - | 0 |
| 7 | 08:18:18 | 08:18:28 | 10s | `@semio-tech/trinity-jack-rs:graph-generate` | - | 0 |
| 8 | 08:18:28 | 08:18:28 | 0s | `@semio-tech/puzzle-3d-rs:graph-generate` | - | 0 |
| 9 | 08:18:28 | 08:18:28 | 0s | `@semio-tech/draw-drawing-rs:graph-generate` | - | 0 |
| 10 | 08:18:28 | 08:24:08 | 5m 40s | `semio-framework-os-flow-core:wasm` | `wasm-release` 263s | 195 |
| 11 | 08:18:28 | 08:25:38 | 7m 10s | `@semio-tech/os-plugin-describe-rs:build` | `release` 274s | 246 |
| 12 | 08:18:28 | 08:18:28 | 0s | `@semio-tech/writer-writer-rs:graph-generate` | - | 0 |
| 13 | 08:18:28 | 08:18:28 | 0s | `@semio-tech/reasoning-wires-rs:graph-generate` | - | 0 |
| 14 | 08:18:28 | 08:18:28 | 0s | `@semio-tech/puzzle-2d-rs:graph-generate` | - | 0 |
| 15 | 08:18:28 | 08:18:28 | 0s | `@semio-tech/puzzle-5d-rs:graph-generate` | - | 0 |
| 16 | 08:18:28 | 08:18:38 | 10s | `@semio-tech/trinity-rewriting-rs:graph-generate` | - | 0 |
| 17 | 08:18:28 | 08:18:38 | 10s | `@semio-tech/flow-plugin:graph-generate` | - | 0 |
| 18 | 08:18:38 | 08:18:48 | 10s | `repo:generator-inputs` | - | 0 |
| 19 | 08:18:38 | 08:19:38 | 1m 00s | `@semio-tech/framework-surface-rs:wasm` | `wasm-release` 52s | 155 |
| 20 | 08:18:48 | 08:19:18 | 30s | `@semio-tech/plugin-registry:generate` | - | 0 |
| 21 | 08:19:18 | 08:19:18 | 0s | `semio-framework-canvas-font-assets:build` | `dev` 1s | 2 |
| 22 | 08:19:18 | 08:19:18 | 0s | `semio-framework-os-infinite:fonts` | - | 0 |
| 23 | 08:19:18 | 08:19:28 | 10s | `@semio-tech/framework-plugin-web:support-release` | - | 0 |
| 24 | 08:25:38 | 08:41:38 | 16m 00s | `@semio-tech/cad-plugin:component-dev` | `wasm-dev` 903s | 121 |
| 25 | 08:41:38 | 08:41:58 | 20s | `@semio-tech/cad-plugin:describe` | - | 0 |
| 26 | 08:41:58 | 08:46:08 | 4m 10s | `@semio-tech/cad-extension-aec-building-rust:component-dev` | `wasm-dev` 201s | 95 |
| 27 | 08:46:08 | 08:46:18 | 10s | `@semio-tech/cad-extension-aec-building-rust:describe` | - | 0 |
| 28 | 08:46:18 | 08:47:08 | 50s | `@semio-tech/cad-extension-aec-building-energy-rust:component-dev` | `wasm-dev` 18s | 3 |
| 29 | 08:47:18 | 08:47:18 | 0s | `@semio-tech/cad-extension-aec-building-energy-rust:describe` | - | 0 |
| 30 | 08:47:18 | 08:47:58 | 40s | `@semio-tech/cad-extension-aec-building-structure-rust:component-dev` | `wasm-dev` 3s | 1 |
| 31 | 08:47:58 | 08:47:58 | 0s | `@semio-tech/cad-extension-aec-building-structure-rust:describe` | - | 0 |
| 32 | 08:48:08 | 08:48:48 | 40s | `@semio-tech/cad-extension-spatial-shape-rust:component-dev` | `wasm-dev` 3s | 1 |
| 33 | 08:48:48 | 08:48:48 | 0s | `@semio-tech/cad-extension-spatial-shape-rust:describe` | - | 0 |
| 34 | 08:48:48 | 09:09:49 | 21m 00s | `@semio-tech/demonstrator-plugin:component-dev` | `wasm-dev` 1208s | 120 |
| 35 | 09:09:59 | 09:11:09 | 1m 10s | `@semio-tech/demonstrator-plugin:describe` | - | 0 |
| 36 | 09:11:09 | 09:21:29 | 10m 20s | `@semio-tech/flow-plugin:component-dev` | `wasm-dev` 571s | 5 |
| 37 | 09:21:29 | 09:21:39 | 10s | `@semio-tech/flow-plugin:describe` | - | 0 |
| 38 | 09:21:39 | 09:22:19 | 40s | `@semio-tech/flow-extension-bim-rust:component-dev` | `wasm-dev` 4s | 1 |
| 39 | 09:22:19 | 09:22:29 | 10s | `@semio-tech/flow-extension-bim-rust:describe` | - | 0 |
| 40 | 09:22:29 | 09:24:39 | 2m 10s | `@semio-tech/flow-extension-brep-rust:component-dev` | `wasm-dev` 88s | 2 |
| 41 | 09:24:39 | 09:24:49 | 10s | `@semio-tech/flow-extension-brep-rust:describe` | - | 0 |
| 42 | 09:24:49 | 09:25:39 | 50s | `@semio-tech/flow-extension-dictionary-rust:component-dev` | `wasm-dev` 3s | 1 |
| 43 | 09:25:39 | 09:25:39 | 0s | `@semio-tech/flow-extension-dictionary-rust:describe` | - | 0 |
| 44 | 09:25:39 | 09:26:19 | 40s | `@semio-tech/flow-extension-draw-rust:component-dev` | `wasm-dev` 3s | 1 |
| 45 | 09:26:19 | 09:26:29 | 10s | `@semio-tech/flow-extension-draw-rust:describe` | - | 0 |
| 46 | 09:26:29 | 09:27:09 | 40s | `@semio-tech/flow-extension-list-rust:component-dev` | `wasm-dev` 3s | 1 |
| 47 | 09:27:09 | 09:27:19 | 10s | `@semio-tech/flow-extension-list-rust:describe` | - | 0 |
| 48 | 09:27:19 | 09:27:59 | 40s | `@semio-tech/flow-extension-logic-rust:component-dev` | `wasm-dev` 3s | 1 |
| 49 | 09:27:59 | 09:27:59 | 0s | `@semio-tech/flow-extension-logic-rust:describe` | - | 0 |
| 50 | 09:27:59 | 09:28:39 | 40s | `@semio-tech/flow-extension-math-rust:component-dev` | `wasm-dev` 3s | 1 |
| 51 | 09:28:39 | 09:28:49 | 10s | `@semio-tech/flow-extension-math-rust:describe` | - | 0 |
| 52 | 09:28:49 | 09:29:29 | 40s | `@semio-tech/flow-extension-core-rust:component-dev` | `wasm-dev` 3s | 1 |
| 53 | 09:29:29 | 09:29:39 | 10s | `@semio-tech/flow-extension-core-rust:describe` | - | 0 |
| 54 | 09:29:39 | 09:30:19 | 40s | `@semio-tech/flow-extension-text-rust:component-dev` | `wasm-dev` 3s | 1 |
| 55 | 09:30:19 | 09:30:19 | 0s | `@semio-tech/flow-extension-text-rust:describe` | - | 0 |
| 56 | 09:30:19 | 09:44:59 | 14m 40s | `@semio-tech/gis-plugin:component-dev` | `wasm-dev` 811s | 54 |
| 57 | 09:44:59 | 09:46:39 | 1m 40s | `@semio-tech/gis-plugin:describe` | - | 0 |
| 58 | 09:46:39 | 09:50:29 | 3m 50s | `@semio-tech/playbook-plugin:component-dev` | `wasm-dev` 184s | 2 |
| 59 | 09:50:39 | 09:50:49 | 10s | `@semio-tech/playbook-plugin:describe` | - | 0 |
| 60 | 09:50:49 | 09:51:49 | 1m 00s | `@semio-tech/playbook-extension-procedural-rust:component-dev` | `wasm-dev` 17s | 3 |
| 61 | 09:51:49 | 09:51:59 | 10s | `@semio-tech/playbook-extension-procedural-rust:describe` | - | 0 |
| 62 | 09:51:59 | 10:02:29 | 10m 30s | `@semio-tech/procedural-plugin:component-dev` | `wasm-dev` 574s | 4 |
| 63 | 10:02:29 | 10:03:19 | 50s | `@semio-tech/procedural-plugin:describe` | - | 0 |
| 64 | 10:03:29 | 10:07:49 | 4m 20s | `@semio-tech/process-plugin:component-dev` | `wasm-dev` 210s | 3 |
| 65 | 10:07:49 | 10:08:19 | 30s | `@semio-tech/process-plugin:describe` | - | 0 |
| 66 | 10:08:19 | 10:10:40 | 2m 20s | `@semio-tech/process-extension-concrete-rust:component-dev` | `wasm-dev` 87s | 4 |
| 67 | 10:10:40 | 10:10:50 | 10s | `@semio-tech/process-extension-concrete-rust:describe` | - | 0 |
| 68 | 10:10:50 | 10:11:40 | 50s | `@semio-tech/process-extension-metal-rust:component-dev` | `wasm-dev` 4s | 1 |
| 69 | 10:11:40 | 10:11:50 | 10s | `@semio-tech/process-extension-metal-rust:describe` | - | 0 |
| 70 | 10:11:50 | 10:12:40 | 50s | `@semio-tech/process-extension-robotic-rust:component-dev` | `wasm-dev` 5s | 1 |
| 71 | 10:12:40 | 10:12:50 | 10s | `@semio-tech/process-extension-robotic-rust:describe` | - | 0 |
| 72 | 10:12:50 | 10:13:40 | 50s | `@semio-tech/process-extension-wood-rust:component-dev` | `wasm-dev` 4s | 1 |
| 73 | 10:13:40 | 10:13:50 | 10s | `@semio-tech/process-extension-wood-rust:describe` | - | 0 |
| 74 | 10:13:50 | 10:27:40 | 13m 50s | `@semio-tech/puzzle-plugin:component-dev` | `wasm-dev` 774s | 30 |
| 75 | 10:27:40 | 10:29:20 | 1m 40s | `@semio-tech/puzzle-plugin:describe` | - | 0 |
| 76 | 10:29:20 | 10:40:20 | 11m 00s | `@semio-tech/sourcing-plugin:component-dev` | `wasm-dev` 619s | 15 |
| 77 | 10:40:30 | 10:40:40 | 10s | `@semio-tech/sourcing-plugin:describe` | - | 0 |
| 78 | 10:40:40 | 10:43:00 | 2m 20s | `@semio-tech/sourcing-extension-beams-rust:component-dev` | `wasm-dev` 91s | 13 |
| 79 | 10:43:00 | 10:43:00 | 0s | `@semio-tech/sourcing-extension-beams-rust:describe` | - | 0 |
| 80 | 10:43:10 | 10:44:00 | 50s | `@semio-tech/sourcing-extension-slabs-rust:component-dev` | `wasm-dev` 4s | 1 |
| 81 | 10:44:00 | 10:44:00 | 0s | `@semio-tech/sourcing-extension-slabs-rust:describe` | - | 0 |
| 82 | 10:44:00 | 10:45:00 | 1m 00s | `@semio-tech/sourcing-extension-windows-rust:component-dev` | `wasm-dev` 4s | 1 |
| 83 | 10:45:00 | 10:45:00 | 0s | `@semio-tech/sourcing-extension-windows-rust:describe` | - | 0 |
| 84 | 10:45:00 | 10:45:20 | 20s | `@semio-tech/plugin-registry:session-demonstrator` | - | 0 |
| 85 | 10:45:20 | 10:57:50 | 12m 30s | `@semio-tech/cad-plugin:component-release` | `wasm-release` 698s | 103 |
| 86 | 10:57:50 | 10:59:10 | 1m 20s | `@semio-tech/cad-plugin:materialize-release` | - | 0 |
| 87 | 10:59:10 | 11:07:00 | 7m 50s | `@semio-tech/cad-extension-aec-building-rust:component-release` | `wasm-release` 418s | 33 |
| 88 | 11:07:00 | 11:07:10 | 10s | `@semio-tech/cad-extension-aec-building-rust:materialize-release` | - | 0 |
| 89 | 11:07:10 | 11:08:50 | 1m 40s | `@semio-tech/cad-extension-aec-building-energy-rust:component-release` | `wasm-release` 63s | 19 |
| 90 | 11:08:50 | 11:09:00 | 10s | `@semio-tech/cad-extension-aec-building-energy-rust:materialize-release` | - | 0 |
| 91 | 11:09:00 | 11:10:00 | 1m 00s | `@semio-tech/cad-extension-aec-building-structure-rust:component-release` | `wasm-release` 16s | 1 |
| 92 | 11:10:00 | 11:10:00 | 0s | `@semio-tech/cad-extension-aec-building-structure-rust:materialize-release` | - | 0 |
| 93 | 11:10:10 | 11:11:00 | 50s | `@semio-tech/cad-extension-spatial-shape-rust:component-release` | `wasm-release` 16s | 1 |
| 94 | 11:11:00 | 11:11:10 | 10s | `@semio-tech/cad-extension-spatial-shape-rust:materialize-release` | - | 0 |
| 95 | 11:11:10 | 11:29:30 | 18m 20s | `@semio-tech/demonstrator-plugin:component-release` | `wasm-release` 1046s | 106 |
| 96 | 11:29:30 | 11:31:30 | 2m 00s | `@semio-tech/demonstrator-plugin:materialize-release` | - | 0 |
| 97 | 11:31:30 | 11:40:51 | 9m 20s | `@semio-tech/flow-plugin:component-release` | `wasm-release` 510s | 5 |
| 98 | 11:40:51 | 11:42:11 | 1m 20s | `@semio-tech/flow-plugin:materialize-release` | - | 0 |
| 99 | 11:42:11 | 11:43:11 | 1m 00s | `@semio-tech/flow-extension-bim-rust:component-release` | `wasm-release` 17s | 1 |
| 100 | 11:43:11 | 11:43:21 | 10s | `@semio-tech/flow-extension-bim-rust:materialize-release` | - | 0 |
| 101 | 11:43:21 | 11:49:41 | 6m 20s | `@semio-tech/flow-extension-brep-rust:component-release` | `wasm-release` 329s | 2 |
| 102 | 11:49:41 | 11:49:51 | 10s | `@semio-tech/flow-extension-brep-rust:materialize-release` | - | 0 |
| 103 | 11:49:51 | 11:50:51 | 1m 00s | `@semio-tech/flow-extension-dictionary-rust:component-release` | `wasm-release` 17s | 1 |
| 104 | 11:50:51 | 11:50:51 | 0s | `@semio-tech/flow-extension-dictionary-rust:materialize-release` | - | 0 |
| 105 | 11:51:01 | 11:51:51 | 50s | `@semio-tech/flow-extension-draw-rust:component-release` | `wasm-release` 17s | 1 |
| 106 | 11:51:51 | 11:52:01 | 10s | `@semio-tech/flow-extension-draw-rust:materialize-release` | - | 0 |
| 107 | 11:52:01 | 11:53:01 | 1m 00s | `@semio-tech/flow-extension-list-rust:component-release` | `wasm-release` 17s | 1 |
| 108 | 11:53:01 | 11:53:11 | 10s | `@semio-tech/flow-extension-list-rust:materialize-release` | - | 0 |
| 109 | 11:53:11 | 11:54:01 | 50s | `@semio-tech/flow-extension-logic-rust:component-release` | `wasm-release` 17s | 1 |
| 110 | 11:54:01 | 11:54:11 | 10s | `@semio-tech/flow-extension-logic-rust:materialize-release` | - | 0 |
| 111 | 11:54:11 | 11:55:11 | 1m 00s | `@semio-tech/flow-extension-math-rust:component-release` | `wasm-release` 17s | 1 |
| 112 | 11:55:11 | 11:55:21 | 10s | `@semio-tech/flow-extension-math-rust:materialize-release` | - | 0 |
| 113 | 11:55:21 | 11:56:11 | 50s | `@semio-tech/flow-extension-core-rust:component-release` | `wasm-release` 17s | 1 |
| 114 | 11:56:11 | 11:56:21 | 10s | `@semio-tech/flow-extension-core-rust:materialize-release` | - | 0 |
| 115 | 11:56:21 | 11:57:31 | 1m 10s | `@semio-tech/flow-extension-text-rust:component-release` | `wasm-release` 18s | 1 |
| 116 | 11:57:31 | 11:57:41 | 10s | `@semio-tech/flow-extension-text-rust:materialize-release` | - | 0 |
| 117 | 11:57:41 | 12:13:21 | 15m 40s | `@semio-tech/gis-plugin:component-release` | `wasm-release` 871s | 54 |
| 118 | 12:13:21 | 12:16:01 | 2m 40s | `@semio-tech/gis-plugin:materialize-release` | - | 0 |
| 119 | 12:16:11 | 12:27:21 | 11m 10s | `@semio-tech/playbook-plugin:component-release` | `wasm-release` 622s | 14 |
| 120 | 12:27:21 | 12:28:31 | 1m 10s | `@semio-tech/playbook-plugin:materialize-release` | - | 0 |
| 121 | 12:28:41 | 12:40:01 | 11m 20s | `@semio-tech/playbook-extension-procedural-rust:component-release` | `wasm-release` 634s | 17 |
| 122 | 12:40:01 | 12:40:41 | 40s | `@semio-tech/playbook-extension-procedural-rust:materialize-release` | - | 0 |
| 123 | 12:40:41 | 12:53:11 | 12m 30s | `@semio-tech/procedural-plugin:component-release` | `wasm-release` 692s | 25 |
| 124 | 12:53:11 | 12:54:01 | 50s | `@semio-tech/procedural-plugin:materialize-release` | - | 0 |
| 125 | 12:54:01 | 13:03:41 | 9m 40s | `@semio-tech/process-plugin:component-release` | `wasm-release` 526s | 4 |
| 126 | 13:03:41 | 13:05:11 | 1m 30s | `@semio-tech/process-plugin:materialize-release` | - | 0 |
| 127 | 13:05:11 | 13:13:21 | 8m 10s | `@semio-tech/process-extension-concrete-rust:component-release` | `wasm-release` 430s | 20 |
| 128 | 13:13:21 | 13:13:31 | 10s | `@semio-tech/process-extension-concrete-rust:materialize-release` | - | 0 |
| 129 | 13:13:31 | 13:15:01 | 1m 30s | `@semio-tech/process-extension-metal-rust:component-release` | `wasm-release` 28s | 1 |
| 130 | 13:15:01 | 13:15:11 | 10s | `@semio-tech/process-extension-metal-rust:materialize-release` | - | 0 |
| 131 | 13:15:11 | 13:16:22 | 1m 10s | `@semio-tech/process-extension-robotic-rust:component-release` | `wasm-release` 25s | 1 |
| 132 | 13:16:22 | 13:16:32 | 10s | `@semio-tech/process-extension-robotic-rust:materialize-release` | - | 0 |
| 133 | 13:16:42 | 13:17:52 | 1m 10s | `@semio-tech/process-extension-wood-rust:component-release` | `wasm-release` 25s | 1 |
| 134 | 13:17:52 | 13:18:02 | 10s | `@semio-tech/process-extension-wood-rust:materialize-release` | - | 0 |
| 135 | 13:18:02 | 13:31:42 | 13m 40s | `@semio-tech/puzzle-plugin:component-release` | `wasm-release` 738s | 51 |
| 136 | 13:31:42 | 13:32:52 | 1m 10s | `@semio-tech/puzzle-plugin:materialize-release` | - | 0 |
| 137 | 13:32:52 | 13:43:52 | 11m 00s | `@semio-tech/sourcing-plugin:component-release` | `wasm-release` 601s | 21 |
| 138 | 13:43:52 | 13:45:12 | 1m 20s | `@semio-tech/sourcing-plugin:materialize-release` | - | 0 |
| 139 | 13:45:12 | 13:51:42 | 6m 30s | `@semio-tech/sourcing-extension-beams-rust:component-release` | `wasm-release` 342s | 14 |
| 140 | 13:51:42 | 13:51:52 | 10s | `@semio-tech/sourcing-extension-beams-rust:materialize-release` | - | 0 |
| 141 | 13:51:52 | 13:52:52 | 1m 00s | `@semio-tech/sourcing-extension-slabs-rust:component-release` | `wasm-release` 22s | 1 |
| 142 | 13:52:52 | 13:53:02 | 10s | `@semio-tech/sourcing-extension-slabs-rust:materialize-release` | - | 0 |
| 143 | 13:53:02 | 13:54:12 | 1m 10s | `@semio-tech/sourcing-extension-windows-rust:component-release` | `wasm-release` 22s | 1 |
| 144 | 13:54:12 | 13:54:22 | 10s | `@semio-tech/sourcing-extension-windows-rust:materialize-release` | - | 0 |
| 145 | 13:54:22 | 13:54:22 | 0s | `@semio-tech/framework-os-dev:prepare-demonstrator-react-release` | - | 0 |
| 146 | 13:54:22 | 14:05:42 | 11m 20s | `@semio-tech/imperative-plugin:component-dev` | `wasm-dev` 634s | 13 |
| 147 | 14:05:42 | 14:06:02 | 20s | `@semio-tech/imperative-plugin:describe` | - | 0 |
| 148 | 14:06:02 | 14:07:22 | 1m 20s | `@semio-tech/imperative-extension-control-rust:component-dev` | `wasm-dev` 39s | 4 |
| 149 | 14:07:22 | 14:07:32 | 10s | `@semio-tech/imperative-extension-control-rust:describe` | - | 0 |
| 150 | 14:07:32 | 14:08:12 | 40s | `@semio-tech/imperative-extension-effect-rust:component-dev` | `wasm-dev` 4s | 1 |
| 151 | 14:08:12 | 14:08:22 | 10s | `@semio-tech/imperative-extension-effect-rust:describe` | - | 0 |
| 152 | 14:08:22 | 14:09:12 | 50s | `@semio-tech/imperative-extension-logic-rust:component-dev` | `wasm-dev` 4s | 1 |
| 153 | 14:09:12 | 14:09:12 | 0s | `@semio-tech/imperative-extension-logic-rust:describe` | - | 0 |
| 154 | 14:09:12 | 14:10:02 | 50s | `@semio-tech/imperative-extension-math-rust:component-dev` | `wasm-dev` 4s | 1 |
| 155 | 14:10:02 | 14:10:02 | 0s | `@semio-tech/imperative-extension-math-rust:describe` | - | 0 |
| 156 | 14:10:12 | 14:10:52 | 40s | `@semio-tech/imperative-extension-text-rust:component-dev` | `wasm-dev` 4s | 1 |
| 157 | 14:10:52 | 14:11:02 | 10s | `@semio-tech/imperative-extension-text-rust:describe` | - | 0 |
| 158 | 14:11:02 | 14:11:12 | 10s | `@semio-tech/plugin-registry:session-imperative` | - | 0 |
| 159 | 14:11:22 | 14:15:52 | 4m 30s | `@semio-tech/imperative-plugin:component-release` | `wasm-release` 225s | 7 |
| 160 | 14:15:52 | 14:17:02 | 1m 10s | `@semio-tech/imperative-plugin:materialize-release` | - | 0 |
| 161 | 14:17:02 | 14:18:22 | 1m 20s | `@semio-tech/imperative-extension-control-rust:component-release` | `wasm-release` 47s | 3 |
| 162 | 14:18:22 | 14:18:32 | 10s | `@semio-tech/imperative-extension-control-rust:materialize-release` | - | 0 |
| 163 | 14:18:42 | 14:19:32 | 50s | `@semio-tech/imperative-extension-effect-rust:component-release` | `wasm-release` 15s | 1 |
| 164 | 14:19:32 | 14:19:42 | 10s | `@semio-tech/imperative-extension-effect-rust:materialize-release` | - | 0 |
| 165 | 14:19:42 | 14:20:32 | 50s | `@semio-tech/imperative-extension-logic-rust:component-release` | `wasm-release` 15s | 1 |
| 166 | 14:20:32 | 14:20:42 | 10s | `@semio-tech/imperative-extension-logic-rust:materialize-release` | - | 0 |
| 167 | 14:20:42 | 14:21:32 | 50s | `@semio-tech/imperative-extension-math-rust:component-release` | `wasm-release` 15s | 1 |
| 168 | 14:21:32 | 14:21:42 | 10s | `@semio-tech/imperative-extension-math-rust:materialize-release` | - | 0 |
| 169 | 14:21:42 | 14:22:32 | 50s | `@semio-tech/imperative-extension-text-rust:component-release` | `wasm-release` 15s | 1 |
| 170 | 14:22:32 | 14:22:42 | 10s | `@semio-tech/imperative-extension-text-rust:materialize-release` | - | 0 |
| 171 | 14:22:42 | 14:22:52 | 10s | `@semio-tech/framework-os-dev:prepare-imperative-react-release` | - | 0 |
| 172 | 14:22:52 | 14:40:02 | 17m 10s | `@semio-tech/stdio-plugin:component-dev` | `wasm-dev` 978s | 46 |
| 173 | 14:40:02 | 14:43:02 | 3m 00s | `@semio-tech/stdio-plugin:describe` | - | 0 |
| 174 | 14:43:12 | 14:54:52 | 11m 40s | `@semio-tech/stdio-image-plugin:component-dev` | `wasm-dev` 650s | 11 |
| 175 | 14:54:52 | 14:55:13 | 20s | `@semio-tech/stdio-image-plugin:describe` | - | 0 |
| 176 | 14:55:13 | 14:55:23 | 10s | `@semio-tech/plugin-registry:session-stdio-png` | - | 0 |
| 177 | 14:55:23 | 15:10:33 | 15m 10s | `@semio-tech/stdio-plugin:component-release` | `wasm-release` 850s | 33 |
| 178 | 15:10:33 | 15:12:23 | 1m 50s | `@semio-tech/stdio-plugin:materialize-release` | - | 0 |
| 179 | 15:12:23 | 15:25:23 | 13m 00s | `@semio-tech/stdio-image-plugin:component-release` | `wasm-release` 732s | 16 |
| 180 | 15:25:23 | 15:26:43 | 1m 20s | `@semio-tech/stdio-image-plugin:materialize-release` | - | 0 |
| 181 | 15:26:43 | 15:26:43 | 0s | `@semio-tech/framework-os-dev:prepare-stdio-png-react-release` | - | 0 |
| 182 | 15:26:43 | 15:36:23 | 9m 40s | `@semio-tech/lowpoly-plugin:component-dev` | `wasm-dev` 531s | 13 |
| 183 | 15:36:23 | 15:36:43 | 20s | `@semio-tech/lowpoly-plugin:describe` | - | 0 |
| 184 | 15:36:53 | 15:37:03 | 10s | `@semio-tech/plugin-registry:session-lowpoly` | - | 0 |
| 185 | 15:37:03 | 15:45:03 | 8m 00s | `@semio-tech/lowpoly-plugin:component-release` | `wasm-release` 433s | 10 |
| 186 | 15:45:03 | 15:45:33 | 30s | `@semio-tech/lowpoly-plugin:materialize-release` | - | 0 |
| 187 | 15:45:33 | 15:45:43 | 10s | `@semio-tech/framework-os-dev:prepare-lowpoly-react-release` | - | 0 |
| 188 | 15:45:43 | 15:54:43 | 9m 00s | `@semio-tech/remodel-plugin:component-dev` | `wasm-dev` 493s | 11 |
| 189 | 15:54:53 | 15:55:03 | 10s | `@semio-tech/remodel-plugin:describe` | - | 0 |
| 190 | 15:55:13 | 15:55:23 | 10s | `@semio-tech/plugin-registry:session-remodel` | - | 0 |
| 191 | 15:55:33 | 16:04:33 | 9m 00s | `@semio-tech/remodel-plugin:component-release` | `wasm-release` 492s | 33 |
| 192 | 16:04:33 | 16:05:03 | 30s | `@semio-tech/remodel-plugin:materialize-release` | - | 0 |
| 193 | 16:05:03 | 16:05:13 | 10s | `@semio-tech/framework-os-dev:prepare-remodel-react-release` | - | 0 |
| 194 | 16:05:13 | 16:15:03 | 9m 50s | `@semio-tech/draw-plugin:component-dev` | `wasm-dev` 542s | 18 |
| 195 | 16:15:03 | 16:15:33 | 30s | `@semio-tech/draw-plugin:describe` | - | 0 |
| 196 | 16:15:33 | 16:15:53 | 20s | `@semio-tech/plugin-registry:session-draw` | - | 0 |
| 197 | 16:15:53 | 16:24:03 | 8m 10s | `@semio-tech/draw-plugin:component-release` | `wasm-release` 442s | 18 |
| 198 | 16:24:03 | 16:24:33 | 30s | `@semio-tech/draw-plugin:materialize-release` | - | 0 |
| 199 | 16:24:33 | 16:24:33 | 0s | `@semio-tech/framework-os-dev:prepare-draw-react-release` | - | 0 |
| 200 | 16:24:43 | 16:33:53 | 9m 10s | `@semio-tech/raster-plugin:component-dev` | `wasm-dev` 504s | 3 |
| 201 | 16:33:53 | 16:34:23 | 30s | `@semio-tech/raster-plugin:describe` | - | 0 |
| 202 | 16:34:23 | 16:34:43 | 20s | `@semio-tech/plugin-registry:session-raster` | - | 0 |
| 203 | 16:34:43 | 16:34:43 | 0s | `@semio-tech/raster-plugin:component-release` | - | 0 |
| 204 | 16:34:43 | 16:34:43 | 0s | `@semio-tech/semio-tech-play:build-fresh` | - | 0 |

## 2. Concurrency

Wall span 8h 16m 55s (29815 s). Zero-duration tasks are excluded from the sweep.

| Active tasks | Time | Share |
|---:|---:|---:|
| 0 | 4m 00s | 0.8% |
| 1 | 8h 07m 05s | 98.0% |
| 2 | 4m 30s | 0.9% |
| 3 | 10s | 0.0% |
| 4 | 1m 10s | 0.2% |

Per 1-minute bucket (497 buckets, time-weighted average active tasks): mean 1.01, maximum 3.50.

| Minute average (floored) | Minutes | Share |
|---|---:|---:|
| [0,1) | 22 | 4.4% |
| [1,2) | 470 | 94.6% |
| [2,3) | 4 | 0.8% |
| >= 3 | 1 | 0.2% |

Only one task was active in 98.0% of wall time. Concurrency above one occurred in only two overlap segments, both between 08:18:18 and 08:24:08 (up to four tasks active: os-plugin-describe-rs build, os-flow-core wasm, framework-surface-rs wasm and graph-generate tasks). From 08:24:08 onward at most one task was active, and the tasks run back to back.

## 3. Time by target type

| Target type | Tasks | Total | Share of task time | Mean | Max |
|---|---:|---:|---:|---:|---:|
| component-release | 42 | 3h 53m 42s | 46.6% | 5m 34s | 18m 20s |
| component-dev | 42 | 3h 31m 42s | 42.2% | 5m 02s | 21m 00s |
| materialize-release | 41 | 23m 40s | 4.7% | 35s | 2m 40s |
| describe | 42 | 14m 20s | 2.9% | 20s | 3m 00s |
| other: build | 2 | 7m 10s | 1.4% | 3m 35s | 7m 10s |
| wasm | 2 | 6m 40s | 1.3% | 3m 20s | 5m 40s |
| registry session-* (plugin-registry) | 7 | 1m 40s | 0.3% | 14s | 20s |
| generate* / generator-inputs | 7 | 1m 00s | 0.2% | 9s | 30s |
| graph-generate | 9 | 30s | 0.1% | 3s | 10s |
| prepare-*-react-release (framework-os-dev) | 6 | 30s | 0.1% | 5s | 10s |
| other: support-release | 1 | 10s | 0.0% | 10s | 10s |
| other: deps-wasm-opt | 1 | 10s | 0.0% | 10s | 10s |
| other: build-fresh | 2 | 0s | 0.0% | 0s | 0s |
| other: fonts | 1 | 0s | 0.0% | 0s | 0s |

Cargo profile totals (from `Finished` lines): `wasm-release` 43 builds, 3h 26m 50s, mean 4m 49s; `wasm-dev` 42 builds, 2h 59m 57s, mean 4m 17s; `release` 1 builds, 4m 34s, mean 4m 34s; `dev` 1 builds, 1s, mean 1s.

## 4. Top 15 longest tasks and critical path

| # | Start | End | Duration | Task | Cargo Finished |
|---:|---|---|---:|---|---|
| 34 | 08:48:48 | 09:09:49 | 21m 00s | `@semio-tech/demonstrator-plugin:component-dev` | `wasm-dev` 1208s |
| 95 | 11:11:10 | 11:29:30 | 18m 20s | `@semio-tech/demonstrator-plugin:component-release` | `wasm-release` 1046s |
| 172 | 14:22:52 | 14:40:02 | 17m 10s | `@semio-tech/stdio-plugin:component-dev` | `wasm-dev` 978s |
| 24 | 08:25:38 | 08:41:38 | 16m 00s | `@semio-tech/cad-plugin:component-dev` | `wasm-dev` 903s |
| 117 | 11:57:41 | 12:13:21 | 15m 40s | `@semio-tech/gis-plugin:component-release` | `wasm-release` 871s |
| 177 | 14:55:23 | 15:10:33 | 15m 10s | `@semio-tech/stdio-plugin:component-release` | `wasm-release` 850s |
| 56 | 09:30:19 | 09:44:59 | 14m 40s | `@semio-tech/gis-plugin:component-dev` | `wasm-dev` 811s |
| 74 | 10:13:50 | 10:27:40 | 13m 50s | `@semio-tech/puzzle-plugin:component-dev` | `wasm-dev` 774s |
| 135 | 13:18:02 | 13:31:42 | 13m 40s | `@semio-tech/puzzle-plugin:component-release` | `wasm-release` 738s |
| 179 | 15:12:23 | 15:25:23 | 13m 00s | `@semio-tech/stdio-image-plugin:component-release` | `wasm-release` 732s |
| 85 | 10:45:20 | 10:57:50 | 12m 30s | `@semio-tech/cad-plugin:component-release` | `wasm-release` 698s |
| 123 | 12:40:41 | 12:53:11 | 12m 30s | `@semio-tech/procedural-plugin:component-release` | `wasm-release` 692s |
| 174 | 14:43:12 | 14:54:52 | 11m 40s | `@semio-tech/stdio-image-plugin:component-dev` | `wasm-dev` 650s |
| 121 | 12:28:41 | 12:40:01 | 11m 20s | `@semio-tech/playbook-extension-procedural-rust:component-release` | `wasm-release` 634s |
| 146 | 13:54:22 | 14:05:42 | 11m 20s | `@semio-tech/imperative-plugin:component-dev` | `wasm-dev` 634s |

**Critical path.** nx reports 22m 46s (5 tasks) as the critical path of its DAG. The effective chain is different. Taking every task of 10 s or more and walking backwards along back-to-back predecessors gives a chain of 165 tasks. It covers 29,575 s of busy time between 08:18:18 and 16:34:43, with no idle gap longer than 15 s. The chain alternates a plugin's `component-dev` or `component-release` (cargo wasm build) with that plugin's `describe` and `materialize-release`, and with the next plugin's build. Build-phase order: `os-plugin-describe-rs:build` (08:18:28 to 08:25:38) and `cad-plugin:component-dev` (08:25:38 to 08:41:38) start the serial queue. `component-dev` runs from 08:25:38 to 16:33:53, while `component-release` starts at 10:45:20 and interleaves with it until 16:34:43. The queue ends with `raster-plugin:component-dev` (16:24:43 to 16:33:53) and `session-raster` (16:34:23 to 16:34:43). The critical path is therefore the serial sum of cargo wasm builds.

## 5. Serialization evidence

- Explicit lease messages are rare: 2 lines `Waiting for exclusive access to ["semio.cargo.build-lease/v1", "…/⚡️cache/fdkxMAb/t", "wasm-release"] behind 1 queued owner(s)`, both from `semio-framework-os-flow-core` at 08:19:18, 08:19:28. Inference: the holder was the framework-surface-rs wasm-release build, which finished at 08:19:38. os-flow-core wasm ran 340 s, of which cargo itself reported 263 s, so roughly 77 s were outside cargo (the lease wait is the likely cause).
- `[cargo-preparation] waiting for Cargo.toml`: 4 lines, all from os-plugin-describe-rs. Minor.
- Grep counts for the other keywords are false positives: `lock` matches `block` and `Lock` inside rustc snippets, `queued` matches `drain_queued_effects`, and `lease` matches `ToolCancellationLease` in diagnostics. Strict matches for `Blocking waiting for file lock` and `lock held` are zero.
- Chaining: 204 of 205 tasks start within 10 s of another task's end. Only the first task has no predecessor. Gaps with no active task total 240 s (0.8%).
- Gate: the fresh build is configured with `--parallel=4` (PLAY_FRESH_BUILD_ARGS), yet the run is serial after 08:24. The log does not show the gate after 08:19. Candidates are the exclusive cargo build lease (its wait message is logged only at start), a serialized wrapper, or a dependsOn chain outside nx's critical path. I could not find the lease implementation with `rg` under `🧰️framework` (no match for `build-lease` or `exclusive access`), so the gate is unconfirmed.

## 6. Recompilation of shared crates

- Prefixed `Compiling` lines: 1,837 across 486 distinct crates. 284 crates are compiled in more than one task. 87 tasks compile something.
- By class (lines / distinct crates):
  - external crates: 618 lines / 313 crates
  - semio stdio artifact crates: 561 lines / 55 crates
  - semio framework crates: 547 lines / 72 crates
  - other semio crates: 111 lines / 46 crates

| Crate | Compiling lines | Tasks |
|---|---:|---:|
| `semio-s-artifact-stdio-semio` | 35 | 35 |
| `semio-framework-plugin` | 28 | 28 |
| `semio-s-artifact-stdio-txt` | 25 | 25 |
| `semio-s-artifact-stdio-binary` | 24 | 24 |
| `semio-s-artifact-stdio-contract` | 24 | 24 |
| `semio-s-artifact-stdio-png` | 24 | 24 |
| `semio-s-artifact-stdio-json` | 23 | 23 |
| `semio-s-artifact-stdio-xml` | 23 | 23 |
| `semio-s-artifact-stdio-zip` | 23 | 23 |
| `semio-s-artifact-stdio-deflate` | 22 | 22 |
| `semio-s-artifact-stdio-obj` | 20 | 20 |
| `semio-s-artifact-stdio-stl` | 20 | 20 |
| `semio-s-artifact-stdio-svg` | 20 | 20 |
| `semio-s-artifact-stdio-dwg` | 19 | 19 |
| `semio-s-artifact-stdio-dxf` | 18 | 18 |
| `semio-s-artifact-stdio-gltf` | 17 | 17 |
| `semio-s-artifact-stdio-pdf` | 17 | 17 |
| `semio-s-artifact-stdio-ply` | 16 | 16 |
| `semio-framework-2d` | 15 | 15 |
| `semio-s-artifact-stdio-las` | 15 | 15 |

Shared framework crates (`semio-framework-*`) are recompiled in each hub build that uses them, for example `semio-framework-plugin` in 28 tasks and `semio-framework-2d` in 15. External crates are compiled almost once: the largest is `bytemuck` in 13 tasks. The single cold external compile is os-plugin-describe-rs build (246 lines, 430 s). Compile lines carry no timing, so the seconds spent recompiling cannot be separated from the wasm build time in this log.

## 7. Log volume and the failure

- Total output 945.9 MB. `[cargo-provenance] {"phase":"read","bytes":N,"totalBytes":N,"work":N}`: 3,283,180 lines, 418.2 MB (44.2%). Phase is always `read`.
- Largest task output: demonstrator-plugin component-release, 24.6 MB. Output of each task is well below the V8 string limit, so the overflow comes from the outer `build-fresh` task, which buffers the whole stream.
- Terminal error: `RangeError: Invalid string length` at `running-tasks.js:434` (`terminalOutputChunks.join('')`), `Unable to print terminal output for Task "@semio-tech/semio-tech-play:build-fresh"`. raster-plugin:component-release was listed as stopped before finishing. The build did not complete: 174 tasks were listed as not run.

## 8. Sinks, ranked

1. Serial execution despite `--parallel=4`: 98% of the time one task was active. nx's own summary calls 446m 40s recoverable.
2. Cargo wasm builds: component-release 3h 53m 42s and component-dev 3h 31m 42s. Single builds range from seconds (small extensions) to 20 minutes (demonstrator-plugin wasm-dev, 1208 s).
3. Log volume: `[cargo-provenance]` progress, 418 MB, which caused the buffer overflow.
4. Recompilation of framework crates per hub: 547 `semio-framework-*` compile lines in 72 crates, plus 561 stdio artifact compile lines. The time share is not separable from this log.
5. Cold external compile in os-plugin-describe-rs build (430 s, 246 compile lines).

## Limitations

- Heartbeat resolution is about 10 s, so each duration is plus or minus 10 s.
- The critical path is inferred from timing, not from the task graph.
- Lease and gate mechanism is not confirmed by code.

