# GIS Config Direct JSON Packet

## Authored scope

This packet adds JSON only under the GIS map config mutation root:

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations`

It contains seven direct-leaf descriptors (`🔣️.json`), seven strict leaf payload schemas (`🧬️schema/🔣️.json`), the strict aggregate envelope `🔣️.json`, and permanent neutral vectors at `🧪️tests/🎚️gis2d-config-direct/🔣️vectors.json`.

## Frozen identities

| Variant | Directory | Text opcode | Binary tag |
| --- | --- | --- | --- |
| `SetLayerVisibility` | `👁️set-layer-visibility` | `set-layer-visibility` | 0 |
| `SetCamera` | `🎥️set-camera` | `set-camera` | 1 |
| `SetRenderMode` | `🖼️set-render-mode` | `set-render-mode` | 2 |
| `SetVectorStyle` | `🎨️set-vector-style` | `set-vector-style` | 3 |
| `SetLodMode` | `🔽️set-lod-mode` | `set-lod-mode` | 4 |
| `SetLayerStrokeScale` | `📏️set-layer-stroke-scale` | `set-layer-stroke-scale` | 5 |
| `SetLocale` | `🗣️set-locale` | `set-locale` | 6 |

Every descriptor declares `explicit-mutation`, `apply-only`, `atomic`, outcomes `applied` and `warning`, plus Rust/JSON-schema/text/binary. Stroke scale additionally declares `fatal` for invalid native numeric input.

The leaf schemas make both map-operation fields required. `visible` is `boolean | null`; `value` is `number | null`. `null` denotes removal of the map entry, whereas explicit `true` and `1.0` retain explicit default entries. Other payloads have their existing required string fields. The aggregate uses the serde-style camelCase `operation` discriminant and closes each envelope with `unevaluatedProperties: false`; no short historical text opcode is admitted.

## Neutral coverage

The permanent vector fixture has nine valid envelopes (all seven operations plus both nullable removal forms), seven invalid envelopes for omission/unknown/type/legacy-opcode failures, and state vectors covering a populated no-op, absent-entry restoration with `null`, explicit-default restoration with `true`/`1.0`, reverse replay order, and sequential independent-field composition.

No Rust source, plugin lifecycle, terrain source, compiler, or test runner was invoked. The JSON packet is schema-first staging for the root-owned sparse `Gis2dConfigDiff` and direct Rust leaf implementation; it does not claim runtime validation.
