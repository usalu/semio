# Raster Plugin Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/🖨️raster`, including the Raster artifact, editor and viewer surfaces, mutation manifests, differential fixtures, package mounts, and exact central registrations.

The initial strict audit covered 374 files, 315 directories, and 683 governed entries. It found 28 sibling-emoji collisions. Every changed identity was selected and moved explicitly; no automatic emoji chooser, automatic rename planner, migration script, or modifying Git operation was used.

The command identities are `📷️set-camera`, `🔭️set-camera-zoom`, `🖥️set-composite-viewport`, `🌫️set-brush-opacity`, `📏️set-brush-size`, `➕️add-layer`, `🗑️delete-layer`, `📥️drop-layer-kind`, `📋️duplicate-layer`, `🚚️move-layer`, `🩹️patch-layer`, `🧵️patch-layers`, `👓️set-layer-visible`, and `👁️toggle-layer-visible`. All four editor/viewer window option authorities use `☑️options`, distinct from their `🎚️config` siblings. The subset-local contribution is `🔮️oracle`. All 12 mutation payload sidecars are `🧬️.schema.json`.

The 12 oracle and Gherkin fixture identities were reconciled individually with their committed physical scenarios: add/remove asset use their physical `🖼️…` identities; adjustment, blend, opacity, visibility, create, delete, move, rename, reorder, and resize use `🟪️…`, `🧿️…`, `🔵️…`, `🟠️…`, `🟢️…`, `🚫️…`, `🎞️…`, `✏️…`, `🔮️…`, and `📐️…`. The Python and grammar prose references now name the physical `🐍️.py` and `📖️.grammar.semio` files.

## Verification

- Final strict scoped audit: 374 files, 315 directories, 683 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Raster oracle override and five previously absent command registrations.
- All 231 Rust `#[path]` package mounts resolve.
- All 12 mutation descriptors and all 12 oracle manifest records declare `🧬️.schema.json`; every physical payload exists.
- All 12 oracle scenario directories and all 12 Gherkin specification-vector fixture roots resolve.
- A stale-reference scan finds none of the former local command, option, subset-oracle, payload-sidecar, scenario, or component-style paths.
- `bun ✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts test`: exits 0 and prints `[DEBUG] raster ts ok`.
- `bun nx run @semio-tech/raster-plugin:test-quick`: exited 1 after `cargo nextest list --list-type binaries-only --message-format json --profile fundamental -p semio-s-plugin-raster` exceeded the 1,200,000ms budget and was killed for likely shared target-directory contention. Test execution was never reached; no Raster-specific failure was emitted.

## Exact Central Override

```json
"✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```
