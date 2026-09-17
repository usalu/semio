# Demonstrator build closure (2026-09-17)

## Problem

`semio-s-plugin-demonstrator` linked whole composition crates (`semio-s-plugin-puzzle`, `semio-s-plugin-procedural`, `semio-s-plugin-gis`). Each of those always pulls every sibling artifact (puzzle 2d/3d/5d, procedural 2d/3d/assembly, gis map/terrain) into the **same** wasm link as the mit-bestand grid, even though the manifest registers only one surface per owner (`puzzle3d`, `generation3d`, `gis2d`).

Unused plugins such as trinity are **not** in the demonstrator runtime union; they are out of scope for this slice.

## Fix

Replace the three multi-artifact **plugin** path dependencies with the **artifact** crates the demonstrator manifest already imports, and point `🪪️manifest/🎪️demonstrator/🦀️.rs` at those crates directly (same types as the plugin re-export shims).

Runtime registry `depends-on` metadata stays on plugin ids (`puzzle`, `procedural`, `gis`); only the Rust link graph shrinks.

## Verification

- Language-agnostic test: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts`
- `cargo check -p semio-s-plugin-demonstrator --lib`
