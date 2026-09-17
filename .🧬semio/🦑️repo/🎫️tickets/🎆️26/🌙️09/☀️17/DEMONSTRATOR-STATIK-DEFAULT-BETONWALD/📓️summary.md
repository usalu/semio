# 📓️ Demonstrator Statik default example — Betonwald (2026-09-17)

## Problem

`ENTWERFEN_MIT_BESTAND_STATIK_BRAND` already sets `defaults: { exampleId: "concrete-forest" }` (German label **Betonwald**), but the fem3d editor always booted the compiled-in **`demo`** snapshot (`fem3d_boot_snapshot` → `FEM3D_EXAMPLE_TEXT`). The navbar could show Betonwald while the Model/Results windows still rendered the demo truss (3 meshes / 47 instances).

## Fix

- `fem3d_boot_snapshot()` now parses `concrete-forest` first, with fallback to `demo` then empty.
- fem3d editor `setActiveExample` default option id is `concrete-forest` (Rust manifest builder + `🔣️.json` action arg default).
- Unit tests for editor/viewer `initial_snapshot` assert the concrete-forest node/element counts.

## Verify

Rebuild fem wasm for the demonstrator fem3d lane, open `/#statik`, confirm the example picker reads **Betonwald** and both world windows match the standalone lane’s concrete-forest scene (~2 meshes / ~234 instances).
