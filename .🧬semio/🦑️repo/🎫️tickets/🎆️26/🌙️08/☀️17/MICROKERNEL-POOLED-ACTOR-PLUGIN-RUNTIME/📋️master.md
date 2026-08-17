# 🧫️ Microkernel Pooled Actor Plugin Runtime — master

Goal: `🎯r2602🎯runningsketchpad`. Issue: https://github.com/usalu/semio/issues/2567
Coordinator ("sol"): Claude Opus 5, main chat. Executors ("terra"): Sonnet 5. Explorers ("luna"): Haiku 4.5.

## Objective

Run **50+ plugins × 50+ extensions each** concurrently on web (React `⚛️react` and wgpu `🧊️wgpu`) and desktop (wgpu+winit native), with a kernel that carries **no mobile-hostile assumptions** (mobile backends themselves out of scope).

The governing distinction: **package ≠ instance ≠ actor ≠ worker.** 2550 installed records consume zero runtime resources; only activated packages become actors; actors are multiplexed over a bounded shard pool.

## Baseline defects this replaces (verified 2026-08-17)

| Area | Today | Target |
|---|---|---|
| Web hosting | one Web Worker **per plugin**, `PluginWorkerClient` in `🎠️kernel/🟦️component.ts` ~L1205 + a divergent copy in `🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts` L49–240 | fixed shard pool, N actors per shard, actor-id multiplexed |
| Native hosting | one wasmtime `Engine` **per plugin** (`🔌️plugin/🖥️host/🦀️component.rs` L2454+), plugin turns on the **winit thread** via `pollster::block_on` | one shared `Engine`, one `Store` per actor, kernel thread + thread shards |
| Registry | `PluginManifest` obtainable only by instantiating the wasm | static `PackageDescriptor` emitted at build; zero instantiations at load |
| ABI | `exchange()` + **all-synchronous** host imports | `poll(events,budget)→TurnResult`, effects only; pure `log`/`now-ms` only imports |
| Budgets | `PLUGIN_FUEL_BUDGET=50M`, `set_epoch_deadline(u64::MAX)` (nothing ticks) | per-turn fuel + epoch deadline + `ResourceLimiter` + hierarchical quotas |
| Supervision | `ProgramSupervisorState` defined twice, drives nothing | failure ladder warn→throttle→suspend→cancel→trap/restart→quarantine→disable |
| Memory ceiling | `.cargo/config.toml`: V8 4 GiB guard region per module ⇒ ~20 plugins | K shards ⇒ ceiling independent of package count |

Counts: 33 plugins, 26 extension crates, 59 registry rows, 58 playground variants.

## Design of record

- `📓️design-runtime.md` — actor model, scheduler, shards, GuestRuntime, failure policy, scene/patches, metrics, task manager, process-shard seam.
- `📓️design-abi.md` — WIT layout, reactor/effect/event/job/capability/checkpoint contracts, descriptor emission, guest SDK redesign, extension modes, broker + quotas.
- `📓️design-workforce.md` — roles, packet contract, wave DAG, scale fixture, bench budgets, launch/script additions, exit checklist.
- Plan of record (verbatim source): `/Users/ueli/.claude/plans/make-sure-that-s-vast-tulip.md`.

## Packet registry

| Packet | Owner | path_scope (exclusive) | Deps | Size | State |
|---|---|---|---|---|---|
| `L0-imports` | luna | read-only | — | S | dispatched |
| `L0-consumers` | luna | read-only | — | S | dispatched |
| `L0-launch` | luna | read-only | — | S | dispatched |
| `L0-verify` | luna | read-only | — | M | dispatched |
| `A1-actor` | terra | `🧰️framework/🔨️modules/🎭️actor/**` | — | XL | dispatched |
| `A3-kernel-types` | terra | `🎠️kernel/{🦀️,🟦️}component.*`, `🛂️manifest/{🦀️,🟦️}component.*` + the atomic repo-wide `HostEffect`→`Effect` rename | — | L | dispatched |
| `A4-channel` | terra | `📡️spr/🧵️channel/🦀️component.rs`, `💻️os/🟦️component.ts` codec region, `ProgramBridge` decoder | A3, dispatched **with** B1/A2 | M | queued |
| `A2-abi-sdk` | terra | `🔌️plugin/📦️packages/🦀️rust/📜️wit/**`, `🔌️plugin/🦀️component.rs`, `🔌️plugin/⚛️reactor/**`, `🔌️plugin/🌐host/**`, `🔌️plugin/🏗️builder/**`, `✏️s/🔌️plugins/🗒️note/**` | A3 types | XL | queued |
| `B1-host-native` | terra | `🔌️plugin/🖥️host/**`, `💻️os/🖥️host/🦀️component.rs`, `🧩️extension/🦀️component.rs`, `🏃️run/🦀️component.rs` | A1,A3 | XL | queued |
| `H1-react` | terra | `🧱️elements/{PluginRuntime,WasmSessionLoader,ShellHost}/**`, `⚛️react/📦️index.tsx`, `🧑️‍💻️dev/🟦️component.ts` | G1 | XL | queued |
| `H2-web-shard` | terra | `🌐plugin-web-materialize.ts`, `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, `🎠️kernel/🟦️component.ts` L1141–1688, `📦️packages/🟦️typescript/🟦️glue.ts` | G1,A2 | L | queued |
| `H3-wgpu-native` | terra | `ProgramBridge/🧊️component.rs`, `🧊️wgpu/📦️glue.rs` native regions, `📦️bin.rs`, `Shell/🧊️component.rs` plugin regions (leased) | G1 | XL | queued |
| `H4-wgpu-web` | terra | `🧊️wgpu/🟦️typescript/**`, `🧊️wgpu/📦️index.ts`, `📦️glue.rs` wasm regions (leased) | G1,H2 | M | queued |
| `E1-describe` | sol | `📇️describe/**`, `📇️registry/**`, `🔣️taxonomy.json`, dev `📜️script.ts` build step | G1 | M | queued |
| `F1-scale-fixture` | terra | `💻️os/🧫️fixtures/🔌️scale/**`, dev `📜️script.ts` fixture region | G1 | M | queued |
| `M0`…`M8` | terra ×6 | `✏️s/🔌️plugins/<p>/**` per packet | G2 | L each | queued |
| `V1-bench` `V2-parity` `T1-tasks` `Z1-warnings` `P1-process` | mixed | see design-workforce | G3 | — | queued |

Gates: **G1** = A1+A2+A3+B1 compile 0-warning native + wasip2 + wasm32-unknown-unknown, actor unit tests green, `🗒️note` runs a turn. **G2** = note+cad parity smoke both renderers + native smoke + descriptor freshness + deterministic fixture. **G3** = 58/58 parity, 33/33 native smoke, sync-import census 0.

## Status

Live log in `📓️status.md`. Binding rules in `📌️important.md` (must be emptied before `ticket_close`).
