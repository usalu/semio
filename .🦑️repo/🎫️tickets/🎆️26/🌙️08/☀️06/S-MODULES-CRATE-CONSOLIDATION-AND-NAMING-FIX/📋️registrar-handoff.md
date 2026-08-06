# 📋️ Registrar Handoff — S-Modules Crate Consolidation And Naming Fix

W7 s-kernel consolidation is complete on disk. Nine legacy Rust crates under `✏️s/🔨️modules/**`
are absorbed into four Shape V2 s-module crates; leftover `⚡️implementations/🦀️rust` trees are
deleted. Cross-cutting consumer repoints are **not** done here — see `🧭️orchestrator-dependent-map.md`.

| crate | lib name | path | nx (rust) |
| --- | --- | --- | --- |
| `semio-s-2d` | `semio_s_2d` | `✏️s/🔨️modules/◻2d/📦️packages/🦀️rust` | `semio-s-2d` |
| `semio-s-3d` | `semio_s_3d` | `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust` | `semio-s-3d` |
| `semio-s-mindmap` | `semio_s_mindmap` | `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust` | `@semio-tech/mindmap-rs` (rename to `semio-s-mindmap` optional) |
| `semio-s-imperative` | `semio_s_imperative` | `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust` | (existing project) |

`🗣️lang` already uses `✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust` only (out of scope for this ticket’s 4-crate target).

## 1. Root `Cargo.toml` — `[workspace] members`

**Delete** (paths no longer exist):

```toml
    "✏️s/🔨️modules/🧊️3d/🎬️scene/⚡️implementations/🦀️rust",
    "✏️s/🔨️modules/🧊️3d/🥽️mesh/⚡️implementations/🦀️rust",
    "✏️s/🔨️modules/💭️mindmap/⚡️implementations/🦀️rust",
    "✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust",
    "✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔨️modules/🧊️3d/🗺️spatial/⚡️implementations/🦀️rust",
```

**Add** (if missing):

```toml
    "✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust",
    "✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust",
```

`◻2d` and `📜️imperative` members are already present at `📦️packages/🦀️rust`.

## 2. Root `Cargo.toml` — `[workspace.dependencies]`

**Delete:**

```toml
semio-framework-os-kernel-3d-brep = { path = "✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust" }
semio-framework-os-kernel-3d-brep-engine = { path = "✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust" }
```

**Add** (C4 convention):

```toml
semio-s-2d = { path = "✏️s/🔨️modules/◻2d/📦️packages/🦀️rust" }
semio-s-3d = { path = "✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust" }
semio-s-mindmap = { path = "✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust" }
semio-s-imperative = { path = "✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust" }
```

## 3. Root `package.json` — `workspaces`

Already updated in this ticket:

- `✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript` (was `…/📐️brep/⚡️implementations/🟦️typescript`)
- `✏️s/🔨️modules/◻2d/📦️packages/🟦️typescript` (prior wave)

Optional follow-up: rename npm package `@semio-tech/kernel-3d-js` → `@semio-tech/s-3d-js` and repoint ~30 plugin `package.json` workspace deps (ticket title target; not blocking registrar member swap).

## 4. Old crate → new module path (`semio-s-3d`)

| old crate / alias | new path |
| --- | --- |
| `kernel_3d_mesh` | `semio_s_3d::mesh` (also `semio_s_3d::*` re-exports at crate root for mesh) |
| `kernel_3d_scene` | `semio_s_3d::scene` |
| `kernel_3d_engine` / `semio-framework-os-kernel-3d-brep-engine` | `semio_s_3d::brep::engine` |
| `kernel_3d_brepkit` / `semio-framework-os-kernel-3d-brep` | `semio_s_3d::brep::kernel` (+ native `semio_s_3d::brep::{vec, topo, …}`) |
| `kernel_3d_spatial` | `semio_s_3d::spatial` |

| old crate | new path |
| --- | --- |
| `reasoning_mindmap` | `semio_s_mindmap` (plugins may keep `pub use semio_s_mindmap as mindmap;`) |

| old crate | new path |
| --- | --- |
| `kernel_2d_rs` / `kernel_2d_engine` | `semio_s_2d::…` (see 2d package `📦️lib.rs`) |
| `imperative_engine` | `semio_s_imperative` |

Full dependent manifest + symbol list: `🧭️orchestrator-dependent-map.md`.

## 5. Hygiene done on disk

- Deleted all six `✏️s/🔨️modules` leftover `⚡️implementations` trees (3d×5 + mindmap×1).
- Moved 3d TS package to `✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript`; fixed relative framework import depth.
- Removed temporary `[workspace]` verification overlay tail from `semio-s-3d` `Cargo.toml`.
- Removed nested `Cargo.lock` / `target/` under `semio-s-3d` and `semio-s-mindmap` package dirs.

## 6. Verification status

- Pre-merge baselines: `3d-baseline-before.txt` in this ticket folder.
- Post-merge: `cargo test -p semio-s-3d` / `semio-s-mindmap` requires root workspace member registration first.
- Until consumers are repointed, expect `cargo metadata` failures from stale member lines pointing at deleted paths (§1 fixes that).
- Use `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for any wasm/link tests on macOS (Xcode license gate).

## 7. Orchestrator follow-up

After registrar member swap, apply consumer `Cargo.toml` + `use` rewrites from `🧭️orchestrator-dependent-map.md` (plugins, framework os/ui/wgpu, infinite/world, flow brep extension, etc.). Delete two **unused** `kernel_3d_engine` dependency lines called out there (playbook procedural extension, flow core).


## 8. Resume update (2026-08-06)

Exclusive-scope agent resumed: leftover impl trees already gone; renamed `@semio-tech/kernel-3d-js` → `@semio-tech/s-3d-js` inside modules TS package only.

Root members for all four crates are already present — **no root Cargo.toml edits this session**.

Still registrar/orchestrator:
- Optional `[workspace.dependencies]` aliases for the four crates (still absent).
- ~32 plugin `package.json` still depend on `@semio-tech/kernel-3d-js` (outside `✏️s/🔨️modules`).
- Scene-only dependents of `semio-s-3d` need `default-features = false` to avoid `framework-core ↔ ui_wgpu ↔ semio-s-3d(brep)` cycle.
