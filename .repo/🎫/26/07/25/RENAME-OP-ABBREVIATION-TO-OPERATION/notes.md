# Rename Op → Operation

## Done
- Renamed first-party abbreviations of **operation** (`op` / `Op` / `Ops` / `*_op` / `*_ops`) to the long form (`operation` / `Operation` / `Operations`).
- Serde/JSON discriminators: `"op"` → `"operation"` (actions/effects) or `"operator"` (expression/comparison/CSG operator kinds).
- Operator-meaning abbreviations expanded: `BinOp`→`BinaryOperator`, `RelOp`→`RelationalOperator`, `BooleanOp`→`BooleanOperator`.
- Fixtures: `basic-remote-ops.json` → `basic-remote-operations.json`, `remote-ops-backlog.json` → `remote-operations-backlog.json`.
- Action ids: `noop` → `noOperation`; comments `no-op` → `no-operation`.
- Fan `*_op` false positives restored to `*_operating_point`.
- Accidental `std::operations` restored to `std::ops`.
- Accidental `match operator` / `apply_*(…, operator)` for VCS document operations restored to `operation`.

## Intentionally kept (not our operation abbreviation)
- `std::ops` / `core::ops` — Rust standard library operator traits module.
- `geo::BooleanOps` — external crate trait name.
- `brepkit_operations::{blend_ops, shell_op}` — external module paths.
- Generated wasm-bindgen `*/pkg/*.js` WebGPU fields (`loadOp`, `storeOp`, …) — browser API.
- `https://op.europa.eu/…` — EU Publications Office host.
- Third-party trees (`.venv`, `site-packages`).
- `AGENTS.md` (must not edit).

## Verify
Cargo check was blocked by concurrent workspace `target/` lock from other agents; static audit shows no first-party `op`/`Op`/`Ops` operation abbreviations remaining outside the intentional list above.

- `ops: wgpu::Operations { … }` — wgpu crate render-pass attachment field name (external API).
