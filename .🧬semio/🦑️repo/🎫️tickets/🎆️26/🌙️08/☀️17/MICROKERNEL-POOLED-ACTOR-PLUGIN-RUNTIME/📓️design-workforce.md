# 📓️ Design — workforce, verification, scale proof

## 1. Packet contract (every `📓️sol-<packet>-packet.md` / `📓️terra-<packet>-report.md`)

1. **Preconditions** — baseline HEAD, SHA-256 of every owned file at dispatch; rehash immediately before writing.
2. **Owned writable paths** — exact list. New files only inside those dirs. Region names inside registrar files are not ownership.
3. **Inputs** — `📓️design-runtime.md` / `📓️design-abi.md` sections, prior reports, exact APIs to consume.
4. **Required result** — behavioural and measurable.
5. **Acceptance commands** — verbatim, with `CARGO_TARGET_DIR=<ticket>/🎯️target` and `-p <crate>`; pasted output + exit codes in the report.
6. **`lease-request`** — fenced block naming the registrar file, the exact insertion/replacement text, and why. The executor never edits that file itself.
7. **MUST NOT** — see `📌️important.md`.
8. `[DEBUG] ` prefix on temporary logs, removed before reporting done.

luna explorers are read-only and write only `📓️luna-<topic>-audit.md`; every claim carries `path:line` + SHA-256 + `git log --oneline -3 -- <path>` (the auto-commit bot makes mtimes and `git status` useless as evidence).

## 2. Wave DAG

```
W0  L0-imports  L0-consumers  L0-launch  L0-verify                      (luna, read-only)
W1  A1-actor ─┐   A3-kernel-types ─┐
              ├─ B1-host-native ───┴─ A2-abi-sdk            ──► G1
W2  H1-react  H2-web-shard  H3-wgpu-native  H4-wgpu-web  E1-describe  F1-scale-fixture ──► G2
W3  M0-stdio ──► M1 M2 M3 M4 M5 M6 M7 (≤6 parallel) ──► M8-demonstrator ──► G3
W4  V1-bench  V2-parity  T1-tasks  Z1-warnings  ──► P1-process-shards ──► exit
```

`G1` = A1/A2/A3/B1 compile with zero warnings on native + `wasm32-wasip2` (plugin) + `wasm32-unknown-unknown` (actor), actor unit tests green, `🗒️note` executes a turn through `WasmtimeRuntime`.
`G2` = `note` + `cad` pass parity smoke on both renderers and native smoke; descriptor freshness tests green; fixture generation byte-identical across two runs.
`G3` = 58/58 parity boot both renderers, 33/33 native smoke, sync-import census 0.

W3 batches (each owns only `✏️s/🔌️plugins/<p>/**`, extensions included with their parent):
`M1` draw, forms, mathematical, layout, raster · `M2` cad+4, sourcing+3 · `M3` flow+9 · `M4` imperative+5, playbook+1, process+4 · `M5` puzzle, procedural, gis, lowpoly, remodel · `M6` fem, architect, trinity, block, space, vcs · `M7` animate, shooting, sequence, writer, reasoning, dag, energy, norm · `M8` demonstrator.

## 3. Scale fixture (`F1`)

`💻️os/🧫️fixtures/🔌️scale/` — owner-root `🧫️fixtures` is taxonomy-legal and sits **outside** `pluginAreas`, so the production registry, dev catalog and launch.json never see it. One parametric crate `semio-framework-os-scale-fixture` (`role = "testkit"`, a role already declared in `🔣️taxonomy.json` and so far unused), built once to `wasm32-wasip2`, whose behaviour is selected by its manifest profile:

`idle` (activates, no work) · `cpu` (busy-loop N ms per turn → budgets/lanes) · `ui` (M patches per turn → revisioned patches) · `io` (requests capabilities → broker/quotas) · `hang` (ignores budget → watchdog) · `crash` (traps → quarantine/rebuild) · `stateful` (accumulates → checkpoint/restore).

`bun ./📜️script.ts generate scale-fixture --plugins 50 --extensions 50 [--seed 1]` writes `🤖️generated/{🔣️registry.json,🔣️catalog.json}` — 2550 records (50 × (1 + 50)), activation events drawn deterministically (≈5 % `on-startup-finished`, rest `on-command` / `on-artifact-kind` / `on-view-visible`), profiles, quotas, capabilities. Seeded ⇒ `scale-fixture check` regenerates and diffs, exactly like `plugin-registry:check`. The profile code is handwritten Rust; only the manifests are generated.

## 4. Bench budgets (`V1`, one `BENCH_BUDGETS` const)

1. Registry: 2550 records parsed, **`instantiations == 0`**, < 150 ms.
2. Cold boot to first interactive frame ≤ 2.5 s web / ≤ 1.5 s native, only `on-startup-finished` actors live.
3. Activate 50 plugins + 50 extensions of one plugin: `active_actors == 100`, `shards == K`, no shard > `ceil(100/K)+1`.
4. Memory ≤ K × 512 MiB + 256 MiB headroom (web, `Worker` count == K); native RSS ≤ 1.5 GiB.
5. Interactive p95 command→patch ≤ 16 ms web / ≤ 8 ms native with 40 `cpu` actors saturating background.
6. `hang` actor killed within 2 × budget, shard rebuilt, siblings restored, total pause ≤ 250 ms.
7. `stateful` actor LRU-suspended and resumed → identical state hash.
8. Capability revoked at runtime → denied completion, actor stays alive, quota counters zero.

Test levels: kernel laws (scheduler fairness, quota arithmetic, revision monotonicity, checkpoint codec) → `mod tests { mod quick }` in `🎭️actor`; fixture determinism + 2550-record parse → dev vitest `quick`; native bench 1–5 → `test-long`; all three renderers 1–8 + 4-shard parity → `test-exhaustive`. `verify gate` gains only the cheap descriptor/fixture freshness checks.

## 5. Script and launch additions (registrar only)

Root `📜️script.ts`: `bench` verb (`//#region 🔖️BenchScript` after `TestScript`), `generate scale-fixture`, `verify rust-warnings --target <t>`; root `📋️project.json` targets `bench-plugins`, `scale-fixture-check`, `verify-rust-warnings`.
Dev `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`: `bench plugins --renderer react|wgpu|native --count --extensions [--shards] [--out]`, `generate scale-fixture`, `scale-fixture check`, describe step after the wasip2 build.
`🎭️actor` crate `📜️script.ts`: `wasm`, `test`, `typegen`. `📇️describe` crate: `describe`. Each plugin crate: `describe`. wgpu target `📜️script.ts`: `native --smoke --scale <registry.json> --report <json>`.
Launch seed (`.vscode/🧩️launch.seed.jsonc`, regenerate with `bun nx run @semio-tech/plugin-registry:generate`): `🛠️dev🦀️os-plugins🧫️scale-fixture` (3_dev 386.7), `🛠️dev🖥️s🧵️task-manager⚛️react` / `…🧊️wgpu🌐️wasm`, `🛠️dev🖥️s🧊️wgpu🖥️native🧵️process-shards`, `📦️verify🧫️scale-fixture🚦️check` (4_build 209.3), `⚖️gate🧵️plugin-runtime⚛️react|🧊️wgpu|🖥️native` (4_gate 411–411.2), `⚖️gate🔬️parity🛝️sweep` (411.3), `⚖️gate🦀️zero-warnings🌐️wasm` (411.4), `📦️generate🛂️descriptors`.

## 6. Exit checklist (sol runs each, pastes output into `📓️status.md`)

1. `verify gate` exit 0. 2. `verify` and `test long` exit 0. 3. Parity 58/58 both renderers across 4 shards, regressions vs `📓️baselines.md` explained. 4. Native smoke all 33 plugin ids exit 0. 5. Bench green on react/wgpu/native, JSON in `bench/`. 6. Zero rust warnings on native + `wasm32-wasip2` + `wasm32-unknown-unknown`. 7. `plugin-registry:check` fresh, `launch.json` regenerated, no stray `[DEBUG] `. 8. Task manager shows live actors in both renderers. 9. Census: 0 sync host imports across 59 crates and none of the "must not exist" symbols in `📌️important.md` remain. 10. `📌️important.md` emptied, `ticket_close` with explicit path and full file list.
