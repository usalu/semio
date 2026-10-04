# Raster Native Compiler Caller Audit

Read-only audit of the existing baseline log, Raster task beginning at line 3078. No Cargo or tests ran in this audit. Parsed 387 Rust errors, matching the compiler receipt, into 68 owner/message groups. Physical owner paths were normalized and checked; every diagnostic owner resolves to an existing file.

## Actionable Map

All 387 error primary locations belong to Raster. Shared framework paths occur as API-definition notes, not shared compilation failures. The log also contains nonblocking framework and Raster warnings; they should not be counted as the 387 errors.

- E0433: removed DSL JSON authority/type callers. Port explicit canonical Pack JSON authority, including `dsl::JsonValue` types. Add explicit member policy to canonical `from_json_str`, `parse`, and `parse_bytes`; preserve independently authored serde calls. Avoid matching ordinary `.parse()` methods, and keep macro imports in scope.
- E0599: eleven `.warn(...)` callers use the retired outcome method. The actual shared API exposes `.warning(...)` at replication mutation line 1321. Preserve refusal severity and code semantics rather than weakening to success.
- E0308: five guards return `&str`, while shared `MutationOutcome::refuse` requires typed `OutcomeCode` at replication mutation line 1272. Change the owning validation/refusal codes to the typed vocabulary at their source; no string compatibility facade.
- E0603: eight test references reach private `Terminology`/`Locale` exports in paint-stroke and fill-region. Import the actual public owning UI locale types.
- E0614: one inspection selection test dereferences `Option<RasterLayerMask>`; repair the typed option access. This may be downstream of JSON type repair and should be rechecked after that repair.
- E0080: mutation derive rejects an unapproved semantic verb at `DESCRIPTORS::{constant#18}`. The source enum has 19 variants and index 18 is `FillRegion`, but constant numbering alone is not proof of derive expansion identity. Inspect the derive's naming/approved vocabulary and the actual fill leaf descriptor before choosing the semantic rename. The owner file is listed below.

The direct canonical JSON dependency and eventual owner native assertions remain validation prerequisites; this audit does not assert successful compilation or SQLite runtime behavior.

## Exact Diagnostic Owner Map

Paths below are repository relative physical paths; line numbers are from the baseline receipt and can shift during concurrent repair. Each row groups identical code/message/owner. Duplicate lines represent separate compiler diagnostics.

Error counts: E0080=1, E0308=5, E0433=361, E0599=11, E0603=8, E0614=1.

| Code | Count | Message | Actual Owner | Lines |
| --- | ---: | --- | --- | --- |
| E0433 | 11 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🧪️tests/🔬️unit/🦀️.rs` | 4, 10, 11, 24, 24, 25, 26, 32, 27, 28, 29 |
| E0433 | 9 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔒️protection/🦀️.rs` | 9, 10, 23, 49, 50, 51, 54, 54, 54 |
| E0433 | 2 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs` | 11, 12 |
| E0433 | 7 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎭️change-layer-mask/🧪️tests/🦀️.rs` | 20, 21, 24, 28, 40, 44, 54 |
| E0433 | 6 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs` | 584, 1337, 1338, 1361, 1362, 1392 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs` | 215, 224, 254, 262 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/🖋️creates-an-ink-8a1bf9/🦀️.rs` | 22, 25, 28, 62, 63, 64, 65, 67, 68, 69, 76, 89, 90, 91, 103, 104, 105, 106, 113, 77 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/🚫️deletes/🦀️.rs` | 23, 26, 29, 68, 69, 70, 71, 73, 74, 75, 82, 96, 97, 98, 108, 109, 110, 111, 118, 83 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-layers/🧪️tests/⤴️lifts/🦀️.rs` | 22, 25, 28, 65, 66, 67, 68, 70, 71, 72, 79, 92, 93, 94, 106, 107, 108, 109, 116, 80 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/✏️renames/🦀️.rs` | 22, 25, 28, 61, 62, 63, 64, 66, 67, 68, 75, 87, 88, 89, 101, 102, 103, 104, 111, 76 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-layer-visible/🧪️tests/🙈️hides/🦀️.rs` | 23, 26, 29, 63, 64, 65, 66, 68, 69, 70, 77, 90, 91, 92, 104, 105, 106, 107, 114, 78 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-layer-opacity/🧪️tests/🌫️fades/🦀️.rs` | 22, 25, 28, 63, 64, 65, 66, 68, 69, 70, 78, 90, 91, 92, 103, 104, 105, 106, 113, 79 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-layer-blend-mode/🧪️tests/💡️switches/🦀️.rs` | 22, 25, 28, 63, 64, 65, 66, 68, 69, 70, 77, 89, 90, 91, 101, 102, 103, 104, 111, 78 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-layer/🧪️tests/📍️slides/🦀️.rs` | 22, 25, 28, 64, 65, 66, 67, 69, 70, 71, 79, 91, 92, 93, 104, 105, 106, 107, 114, 80 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-layer/🧪️tests/📐️resizes/🦀️.rs` | 22, 25, 28, 63, 64, 65, 66, 68, 69, 70, 78, 90, 91, 92, 102, 103, 104, 105, 112, 79 |
| E0433 | 20 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-layer-adjustment/🧪️tests/📈️switches/🦀️.rs` | 22, 25, 28, 65, 66, 67, 68, 70, 71, 72, 79, 92, 93, 94, 109, 110, 111, 112, 119, 80 |
| E0433 | 23 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️add-layer-asset/🧪️tests/🖼️declines/🦀️.rs` | 26, 29, 32, 83, 84, 85, 87, 89, 90, 91, 98, 123, 124, 125, 134, 135, 136, 137, 144, 99, 107, 109, 110 |
| E0433 | 3 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-layer-transform/🧪️tests/🦀️.rs` | 10, 37, 37 |
| E0433 | 5 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-layer/🧪️tests/🦀️.rs` | 12, 13, 13, 19, 64 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🧪️tests/🦀️.rs` | 13, 50, 105, 106 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️fill-region/🧪️tests/🦀️.rs` | 15, 63, 118, 119 |
| E0433 | 14 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️remove-layer-asset/🧪️tests/🖼️rejects/🦀️.rs` | 25, 28, 31, 93, 94, 95, 97, 99, 100, 101, 107, 108, 113, 114 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs` | 12 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs` | 14 |
| E0433 | 2 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️export/🧪️tests/🦀️.rs` | 143, 144 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️assets/🔄️replacement/🧪️tests/🦀️.rs` | 36 |
| E0433 | 5 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖱️selection/🦀️.rs` | 20, 21, 21, 22, 23 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖱️selection/🧪️tests/🦀️.rs` | 9 |
| E0433 | 2 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️document/🧪️tests/🦀️.rs` | 10, 15 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 148, 157, 300, 300 |
| E0433 | 14 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 154, 154, 159, 161, 162, 163, 164, 167, 168, 1147, 1164, 1164, 1223, 1312 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs` | 50, 53, 81, 101 |
| E0433 | 3 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-layer/🧪️tests/🦀️.rs` | 7, 9, 22 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-layer/🧪️tests/🦀️.rs` | 13, 42, 56, 78 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️edit-pixels/🧪️tests/🦀️.rs` | 157 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️edit-mask/🦀️.rs` | 68 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️edit-mask/🧪️tests/🦀️.rs` | 22, 26, 68, 86 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️paint-stroke/🧪️tests/🦀️.rs` | 11 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-region/🧪️tests/🦀️.rs` | 11 |
| E0433 | 4 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥞️flatten-layers/🧪️tests/🦀️.rs` | 9, 61, 62, 63 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️merge-down/🧪️tests/🦀️.rs` | 8 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎭️mask-from-selection/🧪️tests/🦀️.rs` | 68 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 121 |
| E0433 | 5 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🎛️selection/🦀️.rs` | 7, 12, 92, 112, 140 |
| E0433 | 1 | cannot find `json` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🎭️masks/🧪️tests/🎛️controls/🦀️.rs` | 7 |
| E0433 | 6 | cannot find `JsonValue` in `dsl` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖱️selection/🦀️.rs` | 20, 20, 22, 22, 22, 22 |
| E0603 | 2 | enum `Terminology` is private | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🧪️tests/🦀️.rs` | 205, 206 |
| E0603 | 2 | enum `Locale` is private | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🧪️tests/🦀️.rs` | 205, 206 |
| E0603 | 2 | enum `Terminology` is private | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️fill-region/🧪️tests/🦀️.rs` | 203, 204 |
| E0603 | 2 | enum `Locale` is private | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️fill-region/🧪️tests/🦀️.rs` | 203, 204 |
| E0614 | 1 | type `std::option::Option<RasterLayerMask>` cannot be dereferenced | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🎛️selection/🦀️.rs` | 12 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-layer-adjustment/🔺️diff/🦀️.rs` | 13 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️fill-region/🦀️.rs` | 97 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🦀️.rs` | 276 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️add-layer-asset/🔺️diff/🦀️.rs` | 9 |
| E0308 | 1 | mismatched types | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-layer-transform/🦀️.rs` | 18 |
| E0308 | 1 | mismatched types | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-layer/🦀️.rs` | 30 |
| E0308 | 1 | mismatched types | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-layer-pixels/🦀️.rs` | 33 |
| E0308 | 1 | mismatched types | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎭️change-layer-mask/🦀️.rs` | 33 |
| E0308 | 1 | mismatched types | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️change-layer-locked/🦀️.rs` | 16 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-layer/🔺️diff/🦀️.rs` | 19 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-layer/🔺️diff/🦀️.rs` | 16 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-layer-opacity/🔺️diff/🦀️.rs` | 19 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-layer-blend-mode/🔺️diff/🦀️.rs` | 16 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-layer-visible/🔺️diff/🦀️.rs` | 16 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-layer/🔺️diff/🦀️.rs` | 14 |
| E0599 | 1 | no method named `warn` found for struct `MutationOutcome<D>` in the current scope | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-layers/🔺️diff/🦀️.rs` | 35 |
| E0080 | 1 | evaluation panicked: Mutations requires an approved semantic verb | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` | 32 |
