# 🖥️ What `s` Is

Collected by a read-only explorer (Haiku), 2026-08-29.

- `✏️s/AGENTS.md`: *"semio s (semi os) is a collaborative operating system for designers to share and
  store any kind of design knowledge. It is the ultimate technology that unifies the complete monorepo."*
- Plugin id `s` resolves to the **space** plugin crate `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust`
  (`semio-s-plugin-space`), registered in the generated catalog
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json`
  (59 plugins) and as a **host** variant row in `🤖️generated/🟦️playgrounds.ts` (~line 72).
- `s` is the `DEFAULT_HOST_VARIANT`. Ports: react `6070`, wgpu `6066`.
- Because `s` is a *host* plugin, the dev build compiles the **whole plugin fleet**, not just `s`
  (see `📓️explore-s-dev-pipeline.md`, the `isHostPluginFilter` branch).
- `✏️s/🔨️modules/`:
  - `🌐️spatial-kernel` — bundles for the processing kernel
  - `🏗️fem` — FEM processing
  - `💭️mindmap` — directed reasoning graph (topics = nodes, relations = edges)
  - `📜️imperative` — wasm-compatible Rust engine for imperative computation

## Consequence for this ticket

"`s` works end to end" == the OS host shell boots on :6070 with the full plugin fleet materialized,
and is interactive. Any single plugin crate that fails to build degrades the fleet; any shared
framework crate that fails to build blocks **everything**.
