# Lowpoly Texture Contract Audit

All98 JSON layer records are test data (84files), not production/example assets. Canonical model doc explicitly names256² RGBA and empty opaque-white shorthand. Fixed256+empty is recommended pending root decision; generic1024² kernel validity remains unchanged.

## Exact Focused Asset Inventory

Read-only traversal of Lowpoly plugin and composition source/fixture/example trees; excludes node_modules, target, generated and emitted component trees. Counts intrinsic layer-shaped JSON objects, including standalone mutation payloads, not only paintLayers arrays. Base64 decoded through registered Node Buffer; no source copies saved. Runtime-created Rust fixtures were source-reviewed separately, not counted as asset instances.

{
  "layers": 98,
  "files": 84,
  "counts": {
    "8bytes/white": 69,
    "8bytes/nonwhite": 5,
    "256bytes/white": 10,
    "256bytes/nonwhite": 14
  },
  "dsl": [
    {
      "file": "✏️s\\🔌️plugins\\💠️lowpoly\\🗿️artifacts\\💠️lowpoly\\🏅️standards\\🔖️1\\🪆️subsets\\✳️any\\📚️examples\\🌲️hexagonal-cut-concrete-forest-left\\🖼️assets\\🗣️.dsl.semio",
      "length": 0
    }
  ]
}

| File | Layer Byte Lengths |
| --- | --- |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📷️io-lowpoly-png-1/💠️lowpoly-snapshot.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔀️io-lowpoly-1/💠️lowpoly-snapshot.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/↗️move-object/📍️translates/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/↗️move-object/📍️translates/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/📸️snapshot/➡️after/🔣️.json | 8, 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/🔺️diff/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/🦠️mutation/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-paint-layer/➖️drops/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-paint-layer/➖️drops/📸️snapshot/⬅️before/🔣️.json | 8, 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/⏸️still/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/⏸️still/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/⛔️ghost/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/⛔️ghost/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/🌀️turns/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/🌀️turns/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/📐️lifts/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/📐️lifts/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/🫥️void/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌀️rotate-selection/🫥️void/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌫️change-paint-layer-opacity/🌫️fades/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌫️change-paint-layer-opacity/🌫️fades/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-object/⛵️inserts/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-object/⛵️inserts/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎛️change-paint-layer-blend-mode/✖️switches/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎛️change-paint-layer-blend-mode/✖️switches/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎨️edit-paint-layer/🖌️paints/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎨️edit-paint-layer/🖌️paints/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-object/🏷️retitles/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-object/🏷️retitles/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/👁️change-paint-layer-visible/🙈️hides/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/👁️change-paint-layer-visible/🙈️hides/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💀️delete-object/🚫️removes/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💀️delete-object/🚫️removes/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️scale-object/📐️halves/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️scale-object/📐️halves/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔀️reorder-objects/🔀️moves/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔀️reorder-objects/🔀️moves/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️rotate-object/🔄️yaws/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️rotate-object/🔄️yaws/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️still/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️still/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🔍️wides/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🔍️wides/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔖️rename-paint-layer/🏷️retitles/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔖️rename-paint-layer/🏷️retitles/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔘️change-object-smooth-shading/🟢️turns/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔘️change-object-smooth-shading/🟢️turns/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🕸️create-mesh/🕸️attaches/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🕸️create-mesh/🕸️attaches/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/➡️after/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/⬅️before/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints/📸️snapshot/➡️after/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints/📸️snapshot/⬅️before/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/➡️after/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/⬅️before/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🖌️dabs/📸️snapshot/➡️after/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🖌️dabs/📸️snapshot/⬅️before/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/➡️after/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/⬅️before/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/➡️after/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/⬅️before/🔣️.json | 256, 256 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/⏸️still/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/⏸️still/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/⛔️ghost/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/⛔️ghost/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🔁️twice/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🔁️twice/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🕳️bare/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🕳️bare/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🧩️part/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🧩️part/📸️snapshot/⬅️before/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧨delete-mesh/✂️detaches/📸️snapshot/➡️after/🔣️.json | 8 |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧨delete-mesh/✂️detaches/📸️snapshot/⬅️before/🔣️.json | 8 |


## Authoritative Shape And Conflicting Consumers

Subset prefix below: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any`.

Published schema `🧬️schema/🔣️.json` LowpolyPaintLayer has required name/visible/opacity/blendMode/pixels; pixels is a base64 string with no decoded-length constraint and no width/height. TS `🧬️schema/🟦️.ts` has pixels:Uint8Array; parseLowpolyPixels checks only Uint8Array then clones. Proto uses bytes; GraphQL uses String. Rust `🧬️schema/📸️snapshot/🦀️.rs:83` likewise has Vec<u8>, no dimensions. Thus current interchange shape does not enforce the intended fixed raster size; the4MiB retained-workload cap is not a dimensional schema.

Actual intended canvas default/consumer coordinate system is256×256:
- Snapshot constant line50; empty_paint_pixels allocates256²×4=262,144 opaque-white bytes.
- materialized_pixels line99 changes only empty into white; every nonempty buffer is cloned unchanged. It does NOT resize/crop/validate it.
- compacted line106 changes any all-255 buffer, of any length, to empty. Without length validity this can change its compositing meaning.
- Model composite_layer_pixels `🧬️schema/🦀️.rs:172` always allocates256² output and zips output chunks with source chunks. Long buffers are silently cropped; short buffers leave the rest transparent. Partial RGBA chunks receive fallback channel values.
- Model flood_fill line252 and sample_pixel_from line288 use fixed256 indexing, with no input extent check. Short nonempty buffers can panic on selected coordinates.
- Editor engine ensure_all_paint_buffers `✏️editor/⚙️engine/🦀️.rs:83` replaces EVERY length other than262,144 with white, including nonempty data. This is destructive normalization, not validation.
- UV/session coordinates use LOWPOLY_PAINT_TEXTURE_SIZE; session texture PNG line260 encodes the composited output as256×256. Render/export therefore do not preserve an arbitrary input extent.
- In contrast, apply-paint-stroke mutation runs `🧬️schema/🧬️mutations/🖌️apply-paint-stroke/🦀️.rs:59` infers arbitrary square side using integer sqrt(len/4), rejects nonsquares, and paints that side. The24 asset layers with256 bytes are8×8 examples of this conflicting assumption.
- Retained command admission `✏️editor/🧵️retained-commands/🦀️.rs:25,126` allows pixels.len≤4MiB without exact size/multiple-of4/square validity. Command sample handling uses256 coordinate strides.

Consequently the current permissive wire accepts data which separate public paths crop, erase, paint at a different scale, or index unsafely. It is not coherent to call all≤4MiB layers a working variable-size texture feature.

## Clean Schema-First Decision

Recommend explicitly author the current Lowpoly texture model as **fixed256×256 RGBA8** with exactly two pixel representations: empty opaque-white shorthand, or exactly262,144 RGBA bytes. The constant is a model extent, not a universal generic-kernel limit. This matches current rendering, UV coordinates and image export rather than introducing an undeclared width inference rule. The generic pixels kernel must still support every separately valid extent, including1024², with paged paid close.

Declare this semantic invariant in the owning language-neutral schema/contract first, then make native/TS decoded validation, retained admission and each paint mutation enforce it before touching state. A base64 contentEncoding annotation alone does not enforce decoded length in Ajv; independent oracle must decode and check bytes. Reject malformed nonempty lengths explicitly. Remove silent white replacement, crop/pad tolerance and sqrt-based divergent sizing from valid document paths. Empty materialization is the only allowed implicit expansion and must be identical for every consumer.

This is a new explicit validity decision correcting conflicting permissive declarations; it must not be disguised as current schema already rejecting other lengths. If a future Lowpoly product deliberately supports variable texture dimensions, author width/height and composition alignment explicitly across every layer/renderer/exporter first, rather than inferring a square or silently cropping. No legacy adapter or migration script is needed in this greenfield repository.

## Concrete Asset Impact And Regression Expectations

Comprehensive focused JSON layer-shape inventory above finds98 layer records across84 files:74 records of8bytes,24 of256bytes. Of these79 are all white (69+10) and19 contain nonwhite data (5+14). No counted JSON layer is currently full256². The authored forest example DSL contains one empty layer and fits the proposed explicit contract already. The2 additional records beyond the first paintLayers-array-only inventory are standalone mutation payloads. The per-file table is exhaustive for scanned JSON layer records.

Hand-edit fixtures/expected outputs together if adopting the fixed contract:
- All-white layer records may be authored as the empty shorthand only where their expected semantic intent is the full white default; do not silently compact them during runtime import.
- Nonwhite paint/mutation/IO fixture records need complete intended256² data or replacement valid document examples with regenerated expected pixel runs/PNG result. This changes test inputs by explicit authorship, not runtime truncation/padding.
- Existing generic algorithm fixtures belong in the domain-neutral pixels tests and can retain small dimensions; Lowpoly integration fixtures must reflect Lowpoly's actual declared extent.
- Document native/JSON/text/SQLite roundtrip tests should reject N−4,N+4,partial-pixel and4MiB malformed layer payloads atomically, preserve the source, and accept empty/full exactly. Painting, compositing, sample, PNG and fill must agree on coordinates and output extent.
- Negative short-data tests must observe typed refusal rather than panic; long-data tests must prove no crop or white replacement.

Current valid data under the broad schema is impacted by this tightening; the inventory makes that impact explicit. No production/assets were modified, no Native/browser/server executed, and no generic extent was reduced.


## Canonical Intent And Asset Classification Follow-Up

Root decision remains pending; the fixed256+empty validity rule above is an audit recommendation, not an approved production change.

Canonical model documentation is explicit, not inferred solely from a constant:
- `🧬️schema/📸️snapshot/🦀️.rs:45`: “256² RGBA = 256 KiB per layer”; explains larger textures exceeded the fixed publication envelope.
- Same owner line97 describes empty as the sparse opaque-white default, and line105 states empty pixels IS the opaque-white default everywhere it is read.
- No focused Lowpoly Markdown documentation declared variable texture extents. No width/height field exists in any emitted model representation.
- The line104 comment referring to a full1024² buffer describes the former publication problem; it is not a current variable-extent contract.

Inventory classification: **all84 JSON files /98 JSON layer records are test fixtures or expected snapshots/payloads under 🧪️tests; zero are production/example JSON assets**. Initial96 was paintLayers-array-only;2 standalone mutation layer payloads bring the complete98. All11 .semio files were considered; only one snapshot example contains pixels, the forest DSL, with empty bytes. Other .semio files are command/protocol/grammar inputs, not persisted layer buffers. Thus no nonempty production/example layer asset found in the focused plugin/composition roots needs content preservation to enforce canonical256 intent.

Rust source tests additionally contain **nine explicit malformed nonempty layer-data sites**, not nine runtime instances (helpers/loops can instantiate repeatedly), separate from98 JSON records:
- editor/engine unit line242:3bytes, intentionally tests silent repair.
- binary snapshot unit line4:4bytes.
- IO unit line19:8bytes.
- SQLite snapshot fixture constructor line16:5bytes from neutral octets0,1,127,128,255; same branch also constructs a second empty valid layer.
- SQLite snapshot unit line30:1024bytes override.
- model/schema unit lines80,89,96,97:four4byte compositing layers.

Paths use subset prefix declared above. Panel unit line44 constructs valid empty layers. Retained-fill tests use full size×size RGBA backgrounds and cloned originals; full-size cases should remain valid. These source sites must be authored to valid canvas data or explicit negative-refusal tests; do not preserve malformed fixture compatibility by widening schema or teaching runtime silent crop/pad.

The engine's existing test explicitly expecting repair is proof of current behavior, not desired validity authority over the canonical model. A strict correction must rewrite that assertion to typed atomic refusal. SQLite fidelity fixtures deliberately exercise tiny arbitrary bytes; move pure byte-fidelity coverage to a neutral byte/schema layer or hand-author full valid Lowpoly canvas while retaining queried sentinel octets. No automatic migration or silent normalization is proposed.


## Coordinated Nonwhite Owner Inventory

Runtime owns these19nonwhite records; white owner changed separate fields only.

| File | JSON Pointer | Original Bytes | Layer |
| --- | --- | --- | --- |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 8 | Detail |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/🔺️diff/🔣️.json | /objects/modified/0/patch/paintLayers/edits/0/layer/pixels | 8 | Detail |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-paint-layer/🪜️stacks/🦠️mutation/🔣️.json | /InsertPaintLayer/layer/pixels | 8 | Detail |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-paint-layer/➖️drops/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 8 | Detail |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎨️edit-paint-layer/🖌️paints/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/0/pixels | 8 | Base |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🖌️dabs/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/0/pixels | 256 | Base |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🖌️dabs/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🖌️dabs/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/0/pixels | 256 | Base |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/➡️after/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/⬅️before/🔣️.json | /objects/0/paintLayers/1/pixels | 256 | Clear |
