# Draw Plugin Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/🖍️draw`, including the Drawing artifact, package mounts, mutation catalogs, fixture manifests, generator sources, and exact central taxonomy registrations.

The initial strict audit covered 441 files, 386 directories, and 807 governed entries. It found 72 breaches: 40 missing identities, 31 sibling-emoji collisions, and one generic identity. Every physical move was chosen and executed explicitly; no automatic emoji chooser, rename planner, migration script, or modifying Git operation was used.

The complete handpicked command repair is:

- Engagement and view: `🧭️engagement-input`, `📤️engagement-submit`, `🪛️set-active-utility`, `📷️set-camera`, `🔭️set-camera-zoom`, `🗣️set-locale`, and `🖼️set-active-example`.
- Canvas lifecycle: `✅️canvas-commit-draft`, the retained `🖱️canvas-double-click`, `🚪️canvas-escape`, `↔️canvas-pointer-move`, and `⬆️canvas-pointer-up`.
- Layer operations: `➕️add-layer`, `🔀️combine-boolean`, `🗑️delete-layer`, `📥️drop-layer-kind`, `📋️duplicate-layer`, `🚚️move-layer`, `🩹️patch-layer`, `🧵️patch-layers`, `🌫️set-selected-opacity`, and `👁️toggle-layer-visible`.
- The already-distinct `📃️commit-document`, `🟤️set-snapshot`, and `🧫️set-fixture-json` were retained unchanged.

The five subsets are now `✳️any`, `🏷️metadata`, `🧱️structure`, `🎨️style`, and `🔀️transform`. Their five local contributions are each `🔮️oracle`. Editor/viewer option authorities use `☑️options`; DXF import/export authorities use `📐️dxf`. Generator sources are `✨️generate.rs`, `📚️lib.rs`, and `📖️reader.rs`, with their Cargo paths and documentation updated explicitly.

Fixture owners and files were handpicked as follows:

- Metadata: `🏷️rename-layer/{⬅️before.json,➡️after.json}`, `🔒️set-layer-locked/{⬅️before.json,➡️after.json}`, and `🙈️set-layer-visible-hides-a-node/{⬅️before.svg,➡️after.svg}`.
- Structure: `➕️create-layer-adds-a-node`, `🗑️delete-layer-removes-a-node`, `📋️duplicate-layer-inserts-a-copy`, and `🔀️reorder-layer-swaps-two-nodes`, each containing `⬅️before.svg` and `➡️after.svg`.
- Style: `🎨️replace-layer-fill-recolors-a-node`, `🖌️replace-layer-stroke-changes-outline`, `🌓️set-layer-blend-mode`, and `🌫️set-layer-opacity-fades-a-node`, each containing the corresponding `⬅️before` and `➡️after` assets.
- Transform: `🚚️update-layer-transform-moves-a-node/{⬅️before.svg,➡️after.svg}`.

All 14 mutation scenario `directoryName` values were individually reconciled with their physical directories, and all 24 fixture paths were updated to the handpicked owners and basenames.

## Verification

- Final strict scoped audit: 441 files, 386 directories, 807 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Draw subset and oracle overrides and 13 previously absent command registrations. Eight command names already existed exactly and were not duplicated.
- All 131 non-dot Rust `#[path]` package mounts resolve.
- All five oracle manifests, 14 mutation vectors, 14 scenario directories, and 24 fixture paths resolve.
- A stale-reference check finds no old Draw-local subset, oracle, command, or generator-source path. The remaining `🧪️oracle` strings intentionally name the external Stdio bridge.
- `bun nx run @semio-tech/draw-plugin:test-quick` reached an unrelated shared Stdio mount failure: the Stdio package still points to the absent GLTF path `🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs`. No missing Draw path was reported.

## Exact Central Overrides

```json
"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets": {
  "*": "✳️any",
  "metadata": "🏷️metadata",
  "structure": "🧱️structure",
  "style": "🎨️style",
  "transform": "🔀️transform"
}
```

Each of those five exact subset paths maps to `🔮️oracle` in `testContributionDirectoryOverrides`.
