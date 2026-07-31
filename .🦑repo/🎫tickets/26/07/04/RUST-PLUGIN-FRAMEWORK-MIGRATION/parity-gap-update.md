# Parity Pass 8 — Functional Equivalence Verified

## Dev infra

- Fixed Vite aliases in `framework/product/os/dev/js/vite.config.ts` (ui-styling/ui-asset package dirs, flow-react, dag-react, writer-react, infinite hosts)
- Added `🛠️dev🖥️s⚛️react` launch config (port **6070**, `SEMIO_RENDERER=react`)
- Old S at `f8376e848` used React ProductShell — use **react** renderer for parity (wgpu is alternate renderer)

## Shell (`applyShellUri`)

- URI drives app: `/` → home, `/studios/:id` → studio + `openStudio`
- Fixed session overwrite race and `refreshUi` stale render guard

## S plugin parity

- All `SPlayController` commands from old `s/core/js/index.ts`
- `unbindParameterField`, catalogue drag-only spawn, engagement rail spawn (no auto drill-in)

## Verification

```bash
# Start: launch 🛠️dev🖥️s⚛️react (6070) or:
cd framework/product/os/dev && SEMIO_PLUGIN=s SEMIO_RENDERER=react S_OS_PORT=6070 bun ./script.ts dev

# E2E:
S_STUDIO_URL=http://127.0.0.1:6070/ node .repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs

# Rust:
cargo test -p s-plugin   # 20 passed
```

E2E covers: home boot → `Meta+n` create studio → 3 windows + flow canvas + compiled DAG → engagement rail spawn (`draw draw`) → undo → command palette (undo/commitCheckpoint) → find palette → breadcrumb home → VFS double-click open studio.

**Status (2026-07-04):** E2E **PASS** on port 6070 react renderer. `cargo test -p s-plugin` 20 passed. `@semio-tech/framework-renderer-react` 11 vitest passed.

## LOC

- `s/plugin/rs/lib.rs`: **2582** (old ~2100)
- `os-shell.tsx` + chrome: **~2600** (old platform+playground ~8087, consolidated)

## Known headless-only gaps

- `NoCompatibleDevice` (WebGPU) — ignored in E2E; real browser required for flow GPU rendering
- Find palette needs flow canvas find items registered (works in browser after canvas boots)
