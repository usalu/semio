# GIS 2D Config Direct Source Census

## Scope and evidence

Read-only census of the current GIS map configuration aggregate at
[`component.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs) and the adjacent map artifact schema at
[`component.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs).

The adjacent schema stores corresponding configuration values in `GisMapArtifact`, but does not define a mutation payload or a shared mutation-specific type. The terrain sibling therefore was not read: no named-map source reference requires that comparison.

## Current concrete operations

`Gis2dConfigMutation` is one seven-variant inline aggregate. Each variant has an existing `dsl` text key, the binary ordinal shown below, and a concrete inverse calculated from the pre-application config.

| Variant | Payload | Text key / binary ordinal | Apply and inverse fact |
| --- | --- | --- | --- |
| `SetLayerVisibility` | `layer_id: String`, `visible: bool` | `layer-visibility` / 0 | `true` removes an override; `false` inserts one. Inverse restores the old value, defaulting to `true`. |
| `SetCamera` | `camera_json: String` | `camera` / 1 | Replaces `camera_json`; inverse retains the old string. |
| `SetRenderMode` | `value: String` | `render-mode` / 2 | Replaces `render_mode`; inverse retains the old string. |
| `SetVectorStyle` | `value: String` | `vector-style` / 3 | Replaces `vector_style`; inverse retains the old string. |
| `SetLodMode` | `value: String` | `lod-mode` / 4 | Replaces `lod_mode`; inverse retains the old string. |
| `SetLayerStrokeScale` | `layer_id: String`, `value: f64` | `layer-stroke-scale` / 5 | `1.0` removes an override, otherwise inserts one. Inverse restores the old value, defaulting to `1.0`. |
| `SetLocale` | `value: String` | `locale` / 6 | Replaces `locale`; inverse retains the old string. |

Every no-op returns the existing warning outcome (`mutation.no-op`). The present `Mutation<Gis2dConfig>::Diff` is a full cloned `Gis2dConfig`, rather than a typed operation delta; this is important behavior to preserve or deliberately redesign in a later released cutover.

## Existing codec and test surface

`OpText` delegates parsing and printing to the enum's generated `DslVariants`, so the seven text identities above are already authoritative. `OpBinary` writes format byte `1`, the displayed variant ordinal as a varint, and the encoded Dsl record; decoding is the inverse. The component's async tests construct all seven variants and exercise text-line round trips. There is no leaf schema, descriptor, direct leaf owner, or aggregate JSON envelope in the current source.

## GIS constructor consumers

The following GIS-owned command sources construct concrete variants and must move to wrapped direct-leaf payloads with the aggregate cutover:

| Source | Constructors |
| --- | --- |
| [`🎮️commands/👁️view/🦀️component.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️view/🦀️component.rs) | `SetLayerVisibility`, `SetCamera`, `SetRenderMode`, `SetVectorStyle`, `SetLodMode`, `SetLayerStrokeScale` |
| [`🎮️commands/🎨️example/🦀️component.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🦀️component.rs) | `SetCamera` |
| [`🎮️commands/🗣️locale/🦀️component.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️locale/🦀️component.rs) | `SetLocale` |

The editor component and command feature/shell modules import or transport `Gis2dConfigMutation`; they require type and pattern rewrites once the inline enum is retired, but this census found no additional concrete constructors there.

## Proposed canonical direct ownership

Use the config schema mutation root:

`✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`

as a transparent, mechanically derived seven-variant aggregate. Proposed semantic leaf kinds and owner directories are:

| Owner directory | Canonical kind | Payload owner |
| --- | --- | --- |
| `👁️set-layer-visibility` | `set-layer-visibility` | layer identifier and visibility |
| `🎥️set-camera` | `set-camera` | camera JSON string |
| `🖼️set-render-mode` | `set-render-mode` | render-mode string |
| `🎨️set-vector-style` | `set-vector-style` | vector-style string |
| `🔽️set-lod-mode` | `set-lod-mode` | LOD-mode string |
| `📏️set-layer-stroke-scale` | `set-layer-stroke-scale` | layer identifier and scale |
| `🗣️set-locale` | `set-locale` | locale string |

Each leaf should own its Rust payload, `MutationLeaf` metadata, `DslRecord` field grammar, apply/inverse logic, and `🧬️schema/🔣️.json` payload schema. The aggregate should only wrap leaf payloads and derive mechanical mutation/DSL codec delegation; it must not retain an operation switch. Before writing, implementation must validate the proposed glyphs against the active taxonomy and validate each descriptor against the authoritative descriptor schema.

The existing public surfaces are Rust, JSON schema, text, and binary. Truthful leaf metadata can therefore declare those four required language surfaces; text opcode is the existing Dsl key and binary tag is the aggregate-local ordinal. The current full-config cloned diff supports `apply-only` participation rather than claiming leaf-owned detection. All current inverses are one explicit mutation, so their current invertibility is `explicit-mutation` unless a later diff redesign changes it.

## Released-write boundary for a future cutover

The prospective source write set is limited to the seven new leaf owner trees, their payload schemas and tests, the config schema aggregate, the named config component (mount/re-export and removal of inline enum/manual codec), and the three constructor sources above plus direct import/pattern consumers under this GIS map editor subtree. It excludes GIS plugin lifecycle, all Compose paths, terrain unless a later source proof identifies a shared type, and unrelated map artifact schema behavior.

No source was changed, no compiler was run, and this census does not claim runtime validation.
