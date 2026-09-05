# Note Artifact Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note` plus its Note package mounts and exact central taxonomy registrations.

The initial strict audit found 151 breaches: 72 missing identities and 79 sibling duplicates. Every physical move was selected and executed explicitly. The eight formerly generic subset identities are now `🖼️asset`, `🧱️block`, `🎨️canvas`, `📜️document`, `🖋️ink`, `🧮️math`, `📊️table`, and `📝️text`; `✳️any` remains the unconstrained subset. Each subset contribution uses `🔮️oracle`, while the artifact-level Stdio oracle bridge remains the distinct `🧪️oracle` authority.

The 23 collided command identities were handpicked by meaning: `🧽️set-eraser-radius`, `🔭️set-camera-zoom`, `📥️load-request`, `💾️save-download`, `🧭️navigator-engagement-input`, `📏️set-grid-spacing`, `🔢️set-grid-subdivisions`, the eight directional/fast nudge arrows, `➕️add-block`, `🗑️delete-block`, `🚫️delete-selection`, `📋️duplicate-block`, `🪞️duplicate-selection`, `🚚️move-block`, `🩹️patch-blocks`, and `📐️set-snap-grid-spacing`. Three preexisting exact identities—`📤️engagement-submit`, `👁️set-grid-visible`, and `🧫️set-fixture-json`—were reused without duplicate registration.

Thirty-three mutation payload sidecars now use `🧬️.schema.json`. Editor/viewer option authorities use `☑️options`, Note DXF import/export authorities use `📐️dxf`, and the asset, block, document, ink, and text fixture directories/files each have one explicit semantic identity. Mutation scenario catalog `directoryName` values were reconciled with all 33 physical scenario directories.

The fixture generator now carries the 16 explicit recipe-to-subset, directory, and 50 file-name decisions. External reproduction preserves semantic recipe IDs while emitting the handpicked filenames; committed generation routes to each handpicked subset path. The aggregate fixture index records the five correct logical subsets and paths that resolve from its own location.

## Verification

- Final strict audit: 704 files, 630 directories, 1,329 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Note subset override, nine exact oracle overrides, and 23 command registrations.
- All 364 Rust `#[path]` package mounts resolve.
- All 33 mutation-catalog scenario directories resolve.
- All 50 fixture files referenced by the subset oracle contributions resolve.
- A clean external generator smoke produced 16 manifests and 50 handpicked filenames; every digest, byte count, role, and basename matches the committed subset manifests.
- `bun nx run @semio-tech/note-plugin:test-quick` reached an unrelated shared dependency failure: Stdio still mounts a missing `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🦀️.rs`. No missing Note path was reported.

The complete subset override is:

```json
"✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets": {
  "*": "✳️any",
  "asset": "🖼️asset",
  "block": "🧱️block",
  "canvas": "🎨️canvas",
  "document": "📜️document",
  "ink": "🖋️ink",
  "math": "🧮️math",
  "table": "📊️table",
  "text": "📝️text"
}
```

## Final Root Fixture Correction

The whole-plugin census found one remaining root fixture collision: `🧪️action-cohort/🔣️.schema.json` is now `🧪️action-cohort/🧬️.schema.json`, paired with retained data `🔣️.json`. Its test already expected the schema identity. A second stale source coordinate in that test was corrected from `🦀️component.rs` to the physical `🦀️.rs`.

The complete Note scope now audits clean: 719 files, 635 directories, 1,343 governed entries, and zero findings. The focused test loads and validates the renamed schema with Ajv, then fails unrelated current source-contract expectations (`NoteCommandJobFactory` versus `BoundedFirstStepCommandJobFactory` and its later semantic census); no full passing result is claimed.
