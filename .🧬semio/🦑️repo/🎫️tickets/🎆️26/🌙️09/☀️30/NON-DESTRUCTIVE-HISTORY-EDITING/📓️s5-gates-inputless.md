# 📭️ S5 gates — editable leaves that show no input (design §22.20)

Generated 2026-10-05 08:13 UTC by `bun T/🧪️s5-gates-inputless.ts` (S5-GATES). Rule: the `inputless` finding of `schema mutation-inputs` — a leaf is EITHER editable and shows at least one input row, OR its descriptor declares it withdraw-only. This list applies the gate's rule to every leaf on disk (3026 leaves, 0 unreadable), so it also names leaves the stale central catalogue hides from the gate (column "Gate reads it" = no).

**35 editable leaves show no input** (35 of them are gate findings today, 0 are hidden by the catalogue); **16 leaves are declared withdraw-only**.

How to clear one: add `"editable": false` as a fifteenth key to the leaf descriptor named below (the derive then answers `input_schema() == None` and refuses `with_input_value`; no Rust attribute needed, and none of `mutation_leaf(payload = …)` / `mutation_leaf(input_schema = …)` may stand beside it) — or give the leaf an input its editor can show. A parameterless leaf (`delete-…`, `remove-…`, `clear-…`) is withdraw-only by nature. Then: `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/<plugin>" --json` (repo root; no `inputless` row may remain) and the crate's own `cargo test --lib` round-trip law.

Not on this list although their payload schema has no top-level `properties`: 2 root-union leaves — the editor shows their variant selector and every variant's fields (procedural `🎛️change-widget-input`, forms `🎛️change-block-field`). The goal audit's figure of 52 (`📓️audit-s5-goal-2.md` gap 8) counted two of them (procedural, forms) and predates the stdio marks.

## Summary

| Owner | Plugin | Editable, no input | of them hidden-only | Gate reads | Declared withdraw-only |
| --- | --- | ---: | ---: | ---: | ---: |
| S5-TEXT-STDIO | stdio | 27 | 0 | 27 | 7 |
| S5-TEXT-STDIO | trinity | 0 | 0 | 0 | 1 |
| S5-FLOWCAD | cad | 4 | 0 | 4 | 0 |
| S5-STROKES-NORM | remodel | 0 | 0 | 0 | 8 |
| S5-TOOLS | energy | 2 | 0 | 2 | 0 |
| coordinator | framework config | 1 | 0 | 1 | 0 |
| coordinator | framework workflow | 1 | 0 | 1 | 0 |

## S5-TEXT-STDIO — 27 leaves to mark or to give an input

### stdio (27)

| Artifact | Leaf | Why | Gate reads it | Descriptor to mark |
| --- | --- | --- | --- | --- |
| 🎨️svg | 🧹strip-non-tiny | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/🧹strip-non-tiny/🔣️.json` |
| 📕️xlsx | 🚫️remove-conformance-attribute | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🧬️mutations/🚫️remove-conformance-attribute/🔣️.json` |
| 📕️xlsx | 🚫️remove-conformance-attribute | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🚫️remove-conformance-attribute/🔣️.json` |
| 📖️pdf | 📉️collapse-page-size | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📉️collapse-page-size/🔣️.json` |
| 📖️pdf | 🧹️clear-page-text | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🧹️clear-page-text/🔣️.json` |
| 📖️pdf | 🌲️set-struct-tree-root | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/🌲️set-struct-tree-root/🔣️.json` |
| 📖️pdf | 🗑️remove-mark-info | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/🗑️remove-mark-info/🔣️.json` |
| 📖️pdf | 🤐️remove-lang | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/🤐️remove-lang/🔣️.json` |
| 📖️pdf | 🪓️remove-struct-tree-root | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/🪓️remove-struct-tree-root/🔣️.json` |
| 📖️pdf | 🚫️remove-display-doc-title | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/🚫️remove-display-doc-title/🔣️.json` |
| 📖️pdf | 🧽️remove-output-intent | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🧬️mutations/🧽️remove-output-intent/🔣️.json` |
| 📖️pdf | 🧽️remove-output-intent | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/🧽️remove-output-intent/🔣️.json` |
| 📖️pdf | 🧽️remove-output-intent | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🧽️remove-output-intent/🔣️.json` |
| 📖️pdf | 🗑️remove-dpart-metadata | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🗑️remove-dpart-metadata/🔣️.json` |
| 📖️pdf | 🧹️remove-dpart-root | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🧹️remove-dpart-root/🔣️.json` |
| 📖️pdf | 🧽️remove-output-intent | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🧽️remove-output-intent/🔣️.json` |
| 📜️docx | 🚫️remove-conformance-attribute | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🧬️mutations/🚫️remove-conformance-attribute/🔣️.json` |
| 📜️docx | 🚫️remove-conformance-attribute | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🧬️schema/🧬️mutations/🚫️remove-conformance-attribute/🔣️.json` |
| 📽️pptx | 🏷️remove-conformance-attribute | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🧬️mutations/🏷️remove-conformance-attribute/🔣️.json` |
| 📽️pptx | 🏷️remove-conformance-attribute | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🏷️remove-conformance-attribute/🔣️.json` |
| 🖼️tiff | ✂️remove-strip-offsets | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/✂️remove-strip-offsets/🔣️.json` |
| 🖼️tiff | 🗑️remove-tile-tags | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🗑️remove-tile-tags/🔣️.json` |
| 🧊️gltf | 🏠️default-scene/✂️unbind | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🏠️default-scene/✂️unbind/🔣️.json` |
| 🧿️semio | 💥delete-brep | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/💥delete-brep/🔣️.json` |
| 🧿️semio | 🧨delete-mesh | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧨delete-mesh/🔣️.json` |
| 🧿️semio | 🚫delete-properties | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚫delete-properties/🔣️.json` |
| 🧿️semio | 🚫delete-properties | no input | yes | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🚫delete-properties/🔣️.json` |

## S5-FLOWCAD — 4 leaves to mark or to give an input

### cad (4)

| Artifact | Leaf | Why | Gate reads it | Descriptor to mark |
| --- | --- | --- | --- | --- |
| 📐️cad | 💣delete-structure-classic-model | no input | yes | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣delete-structure-classic-model/🔣️.json` |
| 📐️cad | 💥delete-building-model | no input | yes | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥delete-building-model/🔣️.json` |
| 📐️cad | 🔌delete-energy-model | no input | yes | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌delete-energy-model/🔣️.json` |
| 📐️cad | 🧨delete-shape-model | no input | yes | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-shape-model/🔣️.json` |

## S5-TOOLS — 2 leaves to mark or to give an input

### energy (2)

| Artifact | Leaf | Why | Gate reads it | Descriptor to mark |
| --- | --- | --- | --- | --- |
| 🔋️model | ✂️disconnect-referenced | no input | yes | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced/🔣️.json` |
| 🔋️model | 🌤️unbind-weather-file | no input | yes | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🔣️.json` |

## coordinator — 2 leaves to mark or to give an input

### framework config (1)

| Artifact | Leaf | Why | Gate reads it | Descriptor to mark |
| --- | --- | --- | --- | --- |
|  | 🚪️sign-out | no input | yes | `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🔣️.json` |

### framework workflow (1)

| Artifact | Leaf | Why | Gate reads it | Descriptor to mark |
| --- | --- | --- | --- | --- |
| 🔁️workflow | 🔄update-node-ports | no input | yes | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔄update-node-ports/🔣️.json` |

## Declared withdraw-only (16)

| Owner | Plugin | Artifact | Leaf |
| --- | --- | --- | --- |
| S5-STROKES-NORM | remodel | 📸️remodeling | ☁️replace-dense |
| S5-STROKES-NORM | remodel | 📸️remodeling | ⭐replace-sparse |
| S5-STROKES-NORM | remodel | 📸️remodeling | 🏁commit-reconstruction |
| S5-STROKES-NORM | remodel | 📸️remodeling | 🗾replace-geo-products |
| S5-STROKES-NORM | remodel | 📸️remodeling | 🧱replace-mesh-result |
| S5-STROKES-NORM | remodel | 📸️remodeling | 🧾replace-qc |
| S5-STROKES-NORM | remodel | 📸️remodeling | 🚂replace-tracks |
| S5-STROKES-NORM | remodel | 📸️remodeling | 🛣️replace-trajectory |
| S5-TEXT-STDIO | trinity | ♻️rewriting | 🖼️edit-before-fixture |
| S5-TEXT-STDIO | stdio | 🌦️epw | 📸️set-snapshot |
| S5-TEXT-STDIO | stdio | 🎞️gif | 📸️set-snapshot |
| S5-TEXT-STDIO | stdio | 🎞️gif | 📸️set-snapshot |
| S5-TEXT-STDIO | stdio | 📼️avi | 📸️set-snapshot |
| S5-TEXT-STDIO | stdio | 🖊️dwg | 📸️set-snapshot |
| S5-TEXT-STDIO | stdio | 🖋️dxf | 📸️set-snapshot |
| S5-TEXT-STDIO | stdio | 🪟️bmp | 📸️set-snapshot |

