# 📌️ FRAMEWORK-UI-FAMILY-CRATE-CONSOLIDATION — resume state (2026-08-06)

## Package shape (this ticket) — DONE under `🖱️ui/**`

- One merged crate `semio-framework-ui` at `📦️packages/🦀️rust/` (features: `tui`, `tui-terminal`, `tui-bindgen`, `wgpu`, `wgpu-engine`, `typegen`).
- Targets are **source dirs only** (`🎯️targets/{⌨️tui,🧊️wgpu}/`) wired via `#[path]` from `📦️lib.rs` — **no per-target `Cargo.toml`**.
- nx project `@semio-tech/ui-rs` at the merged package (`📋️project.json` + `📜️script.ts`).
- `@semio-tech/ui-react` already at Shape V2 `📦️packages/🟦️typescript/🎯️targets/⚛️react` — name preserved; index not split.
- Styling packages unchanged at Shape V2 paths; python `.venv` + orphaned dotnet already deleted earlier.
- **Zero** `⚡️implementations/` under `🖱️ui/**`.

## Do NOT fight `UI-ELEMENT-CO-LOCATION-RESTRUCTURE` (still open)

That ticket owns element folder emoji-renames + wgpu godfile region split. It has repeatedly recreated
`🎯️targets/🧊️wgpu/Cargo.toml`. If it reappears, delete it again — source already uses
`crate::wgpu::…` + `wgpu-engine` cfgs and is **incompatible** with a standalone `semio-framework-ui-wgpu` crate.

Tui `#[path]` entries still point at pre-emoji names (`Select/`, `List/`, …) while disk has `☑️Select/`,
`📃List/`, …. **Leave path rewrites to the co-location ticket.**

## Registrar must apply before root is green

Deleting per-target manifests intentionally breaks root `cargo metadata` until members/deps swap
(same pattern as math family). See `📋️registrar-handoff.md`.

Pre-registrar verify (`🔧️cargo.sh`, strips optional `kernel_3d_scene` to avoid 3d→core→ui-wgpu cycle):
- ✅ `wgpu`, `typegen`, `wasm32-wasip2 --features wgpu`
- ❌ `tui` / `tui-terminal` / combo — missing emoji-renamed element paths (co-location)
- ❌ `wgpu-engine` — needs registrar consumer repoint first (cycle)

## Commands

```bash
# from repo root
./.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/FRAMEWORK-UI-FAMILY-CRATE-CONSOLIDATION/🔧️cargo.sh check --features wgpu
```
