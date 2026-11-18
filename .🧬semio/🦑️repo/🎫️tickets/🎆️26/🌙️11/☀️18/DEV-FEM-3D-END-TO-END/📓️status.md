# Dev Fem 3d End To End: Status

`dev fem 3d` = playground variant `fem3d` (alias `fem 3d`, app `s.fem.fem3d@1/*#editor`, react port 6087).
Agent route without the dashboard: `repo run @semio-tech/framework-os-dev:activate-fem3d-react-prototype`, then
`repo run @semio-tech/framework-os-dev:dev-fem3d-react-prototype` (mapped in `🏗️build/🧭️invocation/🟦️.ts`).
Running `repo ./📜️script.ts serve …` directly is refused ("Original parent Script process invocation required").

## Blockers found and fixed (infrastructure, outside fem)

| Blocker | Root cause | Fix | Report |
|---|---|---|---|
| Every `repo run` looped forever in the graph slice read | Storybook runner spec imports the gitignored framework registry output that had no producer contract; slice-pool same-cause restart counter reset after every run ≥ 60 s | taxonomy contract `framework-plugin-registry`; bounded same-cause restarts + test | [📓️graph-slice-missing-module.md](./📓️graph-slice-missing-module.md) |
| Dependency admission refused (3805 MiB needed, 1430 free) | 60 preservation capsules (12 full `node_modules` copies ≈ 20 GiB) never retired by the collector | pressure reclaim of completed capsules before admission | [📓️dependency-store-capacity.md](./📓️dependency-store-capacity.md) |
| Activation failed: canonical native dependency owner missing | new `border-presentation` crate not yet in `📋️build-owners.json` (registered 07:12 by its owner) | retry | — |
| Activation: `framework-os-node-graph-rs:wasm` consumes its own output | its TS neighbours walked as compile inputs reach `🧑‍💻dev/🪝️host-ports/🕸️wasm-bindings` | `sourceInputObservation: freshness` on that target | [📓️graph-slice-missing-module.md](./📓️graph-slice-missing-module.md) |
| Activation: `plugin-registry:generate` consumes its own output | its own storybook spec test imports the registry output it now owns | `sourceInputObservation: freshness` on `generate` | — |
| Activation: "One reserved host argument required" | `os-plugin-native-transpile-rs:build` declares `hostPlatformOutputsV1` but its command lacked `--host-platform={hostPlatform}`; its outputs used the undefined `{options.env.SEMIO_BUILD_MODE}` although the tool is always staged under `release` | token added, outputs literal `release`, script binds the platform through `requireHostPlatformBuildArgumentsV1` | — |
| Junk `C:semio.cachescratch…` dirs in the worktree root | topology test spliced Windows paths into `bash -c` | `posix()` at the shell boundary | [📓️mangled-scratch-dirs.md](./📓️mangled-scratch-dirs.md) |
| `repo cache gc` cargo reporter crashed | `profileOf` compared backslash paths with `/` | profile derived from the unit path | [📓️cargo-cache-collector.md](./📓️cargo-cache-collector.md) |
| 40-70 min graph read per run | admission memo keyed on the implementation hash of ~55 graph modules that peers edit constantly; no per-owner reuse | memo v2 keyed on an admission-format hash, per-owner facts reused and revalidated row by row; honest elapsed/expected progress text | [📓️graph-admission-memo.md](./📓️graph-admission-memo.md), [📓️graph-inventory-slowness.md](./📓️graph-inventory-slowness.md) |
| `repo dependencies ensure`: "Current complete Source owner ready identity required" | preservation reclaim read the current bindings demanding the newly planned generation after a peer changed the nx patch | reader `identity: "recorded"` for retirement | [📓️dependency-ready-identity.md](./📓️dependency-ready-identity.md) |
| Dependency image full (≈20 GiB) | every interrupted lock left an unready retention capsule `m/j/.p<8hex>` that only setup-all retired | collector retires complete, non-current, never-ready capsules (6 h grace, 0 under pressure) | [📓️dependency-ready-identity.md](./📓️dependency-ready-identity.md) |
| `seal-under-churn` test failed on Windows | injected `node:path` join/dirname into a POSIX-keyed fake store | inject `posix.join`/`posix.dirname` (3/3 pass) | — |
| Native cargo refused | `.cargo/config.toml` routes rustc through the repo ingress; fallback ran with a per-process `CARGO_BUILD_RUSTC=rustc` | — | [📓️native-fem3d-tests.md](./📓️native-fem3d-tests.md) |

## Fem fixes

- `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🎭️3d-viewer/🧪️.story.tsx`: dropped the stale claim that the runtime viewer paints an empty world; viewer windows build their scene from the document (`fem3d_scene_parts`).
- fem3d sqlite snapshot tests: adapted to the current API (see native report).

## Open

- Activation → serve → browser boot on 6087: model/results windows non-empty, example switch, action dispatch.
- Debt kept from 2026-09: mounted live-visual owner page is 4 KiB / 128 node slots (`✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/🦀️.rs:19,98`).
