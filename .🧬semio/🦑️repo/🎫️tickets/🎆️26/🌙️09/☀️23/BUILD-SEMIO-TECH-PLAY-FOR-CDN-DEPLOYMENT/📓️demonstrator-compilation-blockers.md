# Demonstrator Compilation Blockers

The current full cold graph reached Demonstrator component compilation and emitted the following actual diagnostics. These are source/compiler failures in its selected dependency closure. Diagnostic counts may include cascades; independent roots require source validation before repair. All original logs remain temporarily in `🗑️generated/fresh-release-ship-locked.log`.

## Owner Census

| Owner | Diagnostics | Codes |
| --- | ---: | --- |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d` | 178 | E0432:12, E0425:147, E0277:18, E0599:1 |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation` | 34 | E0255:3, E0432:1, E0433:14, E0425:12, E0422:3, E0616:1 |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap` | 33 | E0753:12, E0432:9, E0425:7, E0433:2, E0747:2, E0282:1 |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d` | 2 | E0432:2 |

## Exact First Locations

| Code | Source | Line | Diagnostic |
| --- | --- | ---: | --- |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 2 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 3 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 4 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 5 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 6 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 7 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 8 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 9 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 10 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 11 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 12 | expected outer doc comment |
| `E0753` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 13 | expected outer doc comment |
| `E0255` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 96 | the name `change_curated_item_count` is defined multiple times |
| `E0255` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 93 | the name `create_curated_item` is defined multiple times |
| `E0255` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 90 | the name `delete_curated_item` is defined multiple times |
| `E0432` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` | 3 | unresolved import `crate::op` |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 371 | cannot find `pack` in `snapshot` |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 372 | cannot find `pack` in `snapshot` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 9 | unresolved import `set_camera` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 10 | unresolved import `set_layer_stroke_scale` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 11 | unresolved import `set_layer_visibility` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 12 | unresolved import `set_lod_mode` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 13 | unresolved import `set_render_mode` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 14 | unresolved import `set_vector_style` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 8 | unresolved import `set_camera` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 8 | unresolved import `set_camera` |
| `E0432` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 26 | unresolved import `set_camera` |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 389 | cannot find `text` in `diff` |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 390 | cannot find `text` in `diff` |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 401 | cannot find `pack` in `snapshot` |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 402 | cannot find `pack` in `snapshot` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️camera/🦀️.rs` | 4 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️contribution/🦀️.rs` | 4 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️cursor/🦀️.rs` | 8 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️artifact/🦀️.rs` | 7 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️engagement/🦀️.rs` | 7 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️inspector/🦀️.rs` | 9 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🦀️.rs` | 5 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️step/🦀️.rs` | 12 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪵️stock/🦀️.rs` | 5 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️sun/🦀️.rs` | 4 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛠️workshop/🦀️.rs` | 10 | unresolved import `crate::op` |
| `E0432` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌍️world/🦀️.rs` | 19 | unresolved import `crate::op` |
| `E0425` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 92 | cannot find type `GIS_MAP_OWNED_FIELD_BYTES` in this scope |
| `E0425` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 73 | cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope |
| `E0425` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 223 | cannot find value `GisMapSnapshotRetirementFactory` in this scope |
| `E0425` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 235 | cannot find value `GisMapMutationRetirementFactory` in this scope |
| `E0425` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 391 | cannot find function `default_document` in module `crate::schema` |
| `E0425` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 947 | cannot find function `default_document` in module `crate::schema` |
| `E0433` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🦀️.rs` | 968 | cannot find type `GisMapSnapshotDecodeAuthority` in this scope |
| `E0433` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🦀️.rs` | 972 | cannot find type `GisMapMutationDecodeAuthority` in this scope |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 780 | cannot find function `sourcing_modules` in this scope |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 795 | cannot find function `sourcing_modules` in this scope |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 831 | cannot find function `sourcing_modules` in this scope |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 35 | cannot find value `BINARY_TAG` in module `crate::standards::v1::subsets::any::schema::mutations::create_curated_item` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 35 | cannot find value `BINARY_TAG` in module `crate::standards::v1::subsets::any::schema::mutations::delete_curated_item` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 35 | cannot find value `BINARY_TAG` in module `crate::standards::v1::subsets::any::schema::mutations::change_curated_item_count` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs` | 171 | cannot find value `SOURCING_CURATION_APP_ID` in this scope |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs` | 232 | cannot find value `SOURCING_CURATION_APP_ID` in this scope |
| `E0422` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 58 | cannot find struct, variant or union type `CreateCuratedItem` in module `create_curated_item` |
| `E0422` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 59 | cannot find struct, variant or union type `DeleteCuratedItem` in module `delete_curated_item` |
| `E0422` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 60 | cannot find struct, variant or union type `ChangeCuratedItemCount` in module `change_curated_item_count` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 996 | cannot find function `default_document` in module `crate::schema` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 1054 | cannot find function `installable_contributions` in module `crate::schema` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` | 21 | cannot find function `installable_contributions` in module `crate::schema` |
| `E0425` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` | 65 | cannot find function `default_document` in module `crate::schema` |
| `E0432` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 1081 | unresolved import `source_examples` |
| `E0432` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 1080 | unresolved import `controlled` |
| `E0747` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 92 | unresolved item provided when a constant was expected |
| `E0616` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs` | 200 | field `module_id` of struct `standards::v1::subsets::any::schema::component::ContributedSourcingModule` is private |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 369 | cannot find module or crate `document_dsl` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 370 | cannot find module or crate `document_dsl` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 379 | cannot find module or crate `op` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 380 | cannot find module or crate `op` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 381 | cannot find module or crate `spr` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 382 | cannot find module or crate `spr` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 411 | cannot find module or crate `spr` in this scope |
| `E0433` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 412 | cannot find module or crate `spr` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 219 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 233 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 247 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 261 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 275 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 290 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 298 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 313 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 354 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 368 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 382 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 396 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 410 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 425 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 433 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 448 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 462 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 504 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 512 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 528 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 536 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 552 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 553 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 554 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 555 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 556 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 572 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 573 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 574 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 575 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 576 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 591 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 632 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 646 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 660 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 674 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 688 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 702 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 716 | cannot find function `parameter` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` | 199 | cannot find function `hash_value` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` | 209 | cannot find function `prefix_signature` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` | 221 | cannot find function `prefix_signature` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` | 233 | cannot find function `prefix_signature` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 20 | cannot find value `PROCESS3D_OWNER_BYTES` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 21 | cannot find value `PROCESS3D_MUTATION_BINARY_FORMAT` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 109 | cannot find value `PROCESS3D_OWNER_BYTES` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 121 | cannot find value `PROCESS3D_OWNER_BYTES` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 125 | cannot find value `PROCESS3D_MUTATION_BINARY_FORMAT` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 126 | cannot find function `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 132 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 133 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 137 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 138 | cannot find function `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 140 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 142 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 143 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 144 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 146 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 149 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 150 | cannot find function `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 154 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 155 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 156 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 157 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 158 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 159 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 161 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 163 | cannot find value `PROCESS3D_MAXIMUM_DOMAIN_ITEMS` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 168 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 172 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 173 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 174 | cannot find value `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 175 | cannot find function `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 178 | cannot find function `process3d_protocol_error` in this scope |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 1534 | cannot find function `default_document` in module `crate::schema` |
| `E0425` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` | 84 | cannot find function `default_document` in module `crate::schema` |
| `E0282` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs` | 42 | type annotations needed |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 1373 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` | 53 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` | 182 | the trait bound `MeasureRecipe: BorrowedDslField` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` | 468 | the trait bound `WorkingSolid: BorrowedDslField` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` | 486 | the trait bound `WorkingSolid: BorrowedDslField` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` | 496 | the trait bound `WorkingSolid: BorrowedDslField` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` | 526 | the trait bound `ProcessMeasure: BorrowedDslField` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 19 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 22 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 27 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 28 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | 578 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️capability/🦀️.rs` | 10 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️capability/🦀️.rs` | 11 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` | 337 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` | 391 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs` | 9 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0277` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs` | 8 | the trait bound `standards::v1::subsets::any::schema::snapshot::component::Process3dSnapshot: ArtifactDsl` is not satisfied |
| `E0599` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 976 | no method named `to_uri` found for struct `semio_framework_artifact_reference::ArtifactRef` in the current scope |

No artifact source repair is authored by the pipeline agent at this stage. The parent coordinates owner assignment. The current generation is retained; final publication remains gated by complete source repair, validation and consistent optimized full graph success.


Editor explicitly cleared structural Cad brep domain/events/import/render/media plus generic Work handoff sources for component compilation, with temporary instrumentation/env absent. Current source Demonstrator component-dev retry19674 uses existing cold release6 main Cargo outputs and both Nx cache bypass flags, excluded already completed prerequisites, log demonstrator-current-owner-component.log. Final optimized release/native acceptance remains gated on full Cad459, Store371, one-unit and remaining domain receipts.


Current retry19674 has advanced into real cold compilation of framework kernel/actor after source preparation, build90s observed with no error diagnostics. This is actual producer progression, not final component or optimized release acceptance. Final full Flow retry38223 waits preparation, focused connection54786 is in Cargo build phase. All owned/unrelated healthy processes preserved.


## Actual Current Component Milestone

Owned retry19674 completed actual subprocess/Nx exit0. Wasm-dev compilation10m43s, one staged Demonstrator deliverable, Nx target15m25s. All previously assigned Cad/Process/GIS/Generation3D/Curation production compiler roots now clear this current component assembly. This is metadata component output, not optimized release acceptance; subsequent editor Value grant/progress schema and coherent caller alignment require the final current-source graph to rebuild relevant fingerprints. No final pages or whole321 green claimed.
