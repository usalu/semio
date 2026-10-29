# Schema Dialect Audit

Read-only schema-position inventory of baseline schema-diagnostics.json. No Source changes, migration or validator rerun.

518 diagnostics name498unique documents (20duplicate reports). Traversal visits schema positions, excluding instance payloads under const/default/enum/examples. All paths fit the declared46-unit clone prefix; maximum absolute UTF16 extent211.

## Classification

- $defs: 153 documents
- nonpointer-reference: 31 documents
- unknown-keywords: 23 documents
- prefixItems: 17 documents
- maxContains: 1 documents
- minContains: 1 documents
- format-policy: 1 documents
- unevaluatedProperties: 1 documents
- $ref-assertion-siblings: 1 documents

Closed structural candidates: 303. This excludes unknown keywords, newer vocabulary, $defs, assertion siblings of $ref, nonpointer references, embedded resources, format policy and legacy tuple syntax. These are candidates for per-owner semantic proof, not blanket edit authorization. The earlier preliminary30 reference-sibling count included definitions containers; only the explicit assertion-sibling count above represents a demonstrated ref semantic hazard.

## Safe Correction Boundary

A label-only correction is limited to documents whose actual schema nodes use only shared draft07/2020 assertion vocabulary, local JSON pointers and no resource/format/extension differences. Preserve instance language and independent valid/invalid vectors with actual draft2020 and draft07 validators before authoring the exact files. $defs alone needs review of repository definitions policy and every consumer/pointer; do not rename containers without updating references. Convert prefixItems to draft07 tuple items and transfer 2020 items to additionalItems, retaining min/max limits. Ref siblings require allOf-style conjunction, never dropping assertions. min/maxContains and unevaluatedProperties need explicit equivalent domain schemas; no general URL-only correction. Keep unsupported dynamic/vocabulary cases refused.

Primary semantic reference: [JSON Schema2020-12 release notes](https://json-schema.org/draft/2020-12/release-notes) explains tuple items/additionalItems changes and embedded resources. No schema instance bytes are copied here.

## Owner Family Counts

| Source owner family | Documents | Closed candidates | Max absolute units |
|---|---:|---:|---:|
| ✏️s/🔌️plugins/🀄️wfc | 4 | 4 | 186 |
| ✏️s/🔌️plugins/🌊️flow | 2 | 2 | 123 |
| ✏️s/🔌️plugins/🎬️sequence | 2 | 2 | 210 |
| ✏️s/🔌️plugins/🏗️fem | 10 | 4 | 198 |
| ✏️s/🔌️plugins/🏛️architect | 4 | 3 | 209 |
| ✏️s/🔌️plugins/📏️layout | 27 | 22 | 211 |
| ✏️s/🔌️plugins/📸️remodel | 1 | 0 | 209 |
| ✏️s/🔌️plugins/🔋️energy | 7 | 6 | 194 |
| ✏️s/🔌️plugins/🖍️draw | 4 | 3 | 190 |
| ✏️s/🔌️plugins/🗄️stdio | 61 | 26 | 205 |
| ✏️s/🔌️plugins/🧱️block | 1 | 1 | 99 |
| ✏️s/🔨️modules/🌐️spatial-kernel | 1 | 1 | 142 |
| ✏️s/🔨️modules/🏗️fem | 1 | 0 | 114 |
| ✏️s/🧑‍💻dev/🔱️trinity | 1 | 1 | 103 |
| 🌎️hub/🧩️compositions/🗄️stdio | 1 | 1 | 131 |
| 🧰️framework/🔨️modules/⏯️tool-run | 9 | 6 | 130 |
| 🧰️framework/🔨️modules/◻️2d | 3 | 3 | 151 |
| 🧰️framework/🔨️modules/⚙️runtime | 3 | 3 | 121 |
| 🧰️framework/🔨️modules/⚠️diagnostic | 1 | 0 | 115 |
| 🧰️framework/🔨️modules/🌱️value | 50 | 39 | 147 |
| 🧰️framework/🔨️modules/🎒️pack | 11 | 8 | 136 |
| 🧰️framework/🔨️modules/🏃️process | 8 | 7 | 165 |
| 🧰️framework/🔨️modules/🏗️mesh-engine | 3 | 1 | 137 |
| 🧰️framework/🔨️modules/📁️filesystem | 6 | 3 | 146 |
| 🧰️framework/🔨️modules/📡️replication | 25 | 19 | 148 |
| 🧰️framework/🔨️modules/🔲️pixels | 3 | 0 | 117 |
| 🧰️framework/🔨️modules/🕹️interaction | 1 | 0 | 115 |
| 🧰️framework/🔨️modules/🖱️ui | 12 | 7 | 146 |
| 🧰️framework/🔨️modules/🗣️dsl | 2 | 1 | 131 |
| 🧰️framework/🔨️modules/🗺️surface | 1 | 0 | 122 |
| 🧰️framework/🔨️modules/🧊️3d | 3 | 2 | 142 |
| 🧰️framework/🔨️modules/🧪️test | 1 | 1 | 131 |
| 🧰️framework/🔨️modules/🧮️math | 1 | 1 | 104 |
| 🧰️framework/🔨️modules/🧵️job | 7 | 6 | 126 |
| 🧰️framework/🔨️modules/🚪️io | 9 | 3 | 139 |
| 🧰️framework/🔨️modules/🛠️tool-machine | 1 | 0 | 128 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite | 1 | 1 | 152 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp | 1 | 1 | 133 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow | 4 | 4 | 146 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs | 7 | 6 | 174 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store | 24 | 20 | 169 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference | 3 | 3 | 137 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr | 3 | 3 | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer | 17 | 8 | 192 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin | 39 | 30 | 169 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl | 1 | 1 | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev | 1 | 1 | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural | 1 | 1 | 150 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io | 2 | 0 | 135 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog | 1 | 0 | 141 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/✅️native-controls | 1 | 0 | 116 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences | 1 | 0 | 112 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot | 3 | 0 | 128 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff | 1 | 0 | 106 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json | 1 | 0 | 98 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations | 1 | 1 | 111 |
| 🧰️framework/🛍️products/📓️print/🧾️template/🎓️thesis | 1 | 1 | 120 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment | 1 | 1 | 124 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli | 3 | 1 | 140 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree | 1 | 0 | 116 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard | 14 | 4 | 151 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets | 1 | 0 | 119 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals | 1 | 0 | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner | 1 | 0 | 123 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace | 2 | 0 | 139 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client | 2 | 0 | 141 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics | 1 | 0 | 119 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model | 2 | 0 | 132 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library | 50 | 28 | 191 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes | 1 | 0 | 120 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos | 1 | 0 | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events | 2 | 0 | 129 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp | 1 | 0 | 115 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search | 1 | 0 | 118 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server | 1 | 1 | 159 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase | 1 | 0 | 120 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages | 1 | 0 | 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors | 1 | 0 | 124 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers | 1 | 0 | 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test | 2 | 1 | 130 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml | 1 | 0 | 116 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks | 1 | 0 | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity | 1 | 0 | 120 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher | 1 | 0 | 132 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move | 1 | 0 | 116 |

## Exact Path Inventory

The following are references and flags only; no production payload copies. Empty flags denote closed candidates.

| Path | Flags | Absolute units |
|---|---|---:|
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🧬️schema/🔣️.json | closed candidate | 181 |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🧬️schema/🔣️.json | closed candidate | 185 |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🌡️fill/🧬️schema/🔣️.json | closed candidate | 186 |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧬️schema/🔣️.json | closed candidate | 186 |
| ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📐️tessellate/🧬️schema/🔣️.json | closed candidate | 123 |
| ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧬️schema/🪪️channels/🔣️.json | closed candidate | 121 |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient/🧬️schema/🔣️.json | closed candidate | 210 |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🧬️schema/🔣️.json | closed candidate | 205 |
| ✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/🧬️schema/🔣️.json | closed candidate | 102 |
| ✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🧬️schema/🔣️results-animation/🔣️.json | $defs | 131 |
| ✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/📋️table/🧬️schema/🔣️.json | closed candidate | 116 |
| ✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/🧬️schema/🔣️.json | $defs | 107 |
| ✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/🩹️patch/🧬️schema/🔣️.json | closed candidate | 116 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔣️.json | $defs, nonpointer-reference | 197 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔣️.json | $defs, nonpointer-reference | 195 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔣️.json | $defs, nonpointer-reference | 198 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔣️.json | $defs, nonpointer-reference | 196 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/loads/schema.json | closed candidate | 162 |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config/🧬️schema/🔣️.json | closed candidate | 209 |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🧬️schema/🔣️.json | closed candidate | 209 |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config/🧬️schema/🔣️.json | closed candidate | 207 |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧬️schema/🔣️.json | nonpointer-reference | 206 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧬️schema/📇️addressed.json | closed candidate | 196 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧬️schema/🔣️.json | closed candidate | 187 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📇️metadata-index/🧬️schema/🔣️.json | closed candidate | 192 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧬️schema/🔣️.json | closed candidate | 188 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🖼️composition/📤️canvas/🧬️schema/🔣️.json | closed candidate | 210 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🎨️faithful-primitives/🎨️svg-paint/🧬️schema/🔣️.json | closed candidate | 211 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📍️checkpoint/🧬️schema/🔣️.json | closed candidate | 189 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📤️output/📝️text/🧬️schema/🔣️.json | closed candidate | 193 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📤️output/🧬️schema/🔣️.json | $defs | 185 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🖼️frame-planning/🧬️schema/🔣️.json | closed candidate | 193 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🧬️schema/🔣️.json | closed candidate | 206 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🫧️transient/🧬️schema/🔣️.json | $defs | 209 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️export-step/🧬️schema/🔣️.json | closed candidate | 183 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📚️export-idml/🧬️schema/🔣️.json | closed candidate | 183 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/📐️geometry/🧬️schema/🔣️.json | $defs | 188 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/🖍️background/🧬️schema/🔣️.json | closed candidate | 190 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🏷️text/🧬️schema/🔣️.json | closed candidate | 183 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🖍️paths/🧬️schema/🔣️.json | closed candidate | 184 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/📎️overrides/🧬️schema/🔣️.json | closed candidate | 189 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🧬️schema/🔣️.json | closed candidate | 168 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📦️collection/🧬️schema/🔣️.json | closed candidate | 191 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🚚️list-move/🧬️schema/🔣️.json | closed candidate | 190 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🧬️schema/🔣️.json | closed candidate | 177 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️apply/🗃️page-cache/🔣️.json | $defs | 190 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️setters/🧬️schema/🔣️.json | closed candidate | 188 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/📤️transfer/🔣️.json | closed candidate | 179 |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/🔣️.json | nonpointer-reference | 167 |
| ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🧬️schema/🔣️.json | $defs, nonpointer-reference, unknown-keywords (x-semio-state) | 209 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🧬️schema/🔣️.json | closed candidate | 166 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/🪡️fields/🔎️key/🧬️schema/🔣️.json | closed candidate | 194 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️derive/🌱️birth/🧬️schema/🔣️.json | closed candidate | 187 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️derive/🧬️schema/🔣️.json | $defs, prefixItems | 178 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🧳️retained/🧬️schema/🔣️.json | closed candidate | 189 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🧳️retained/🧬️schema/🔣️.json | closed candidate | 190 |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🧬️schema/⛽️publication/🔣️.json | closed candidate | 182 |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔀️combine-boolean/🧬️schema/🔣️.json | closed candidate | 186 |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🎬️actions/🧬️schema/🔣️.json | $defs | 190 |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧬️schema/🔣️.json | closed candidate | 179 |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📍️point/🧬️schema/🔣️.json | closed candidate | 177 |
| ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧬️schema/🔣️snapshot-edit-event.json | $defs | 141 |
| ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/✍️writer/🧬️schema/🔣️stream.json | closed candidate | 137 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🧬️schema/🔣️.json | closed candidate | 171 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🔣️.json | closed candidate | 155 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/machine.json | $defs | 162 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/retained.json | closed candidate | 163 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/schema.json | $defs | 161 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/command.json | nonpointer-reference | 170 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/🔣️.json | $defs, nonpointer-reference | 166 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/schema.json | $defs | 155 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/✂️clipping/🔣️.json | $defs | 175 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🌊️swept-solid/🔣️.json | $defs | 179 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🌪️advanced-swept-solid/🔣️.json | $defs | 188 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/💎️advanced-brep/🔣️.json | $defs | 181 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/📐️alignment/🔣️.json | $defs | 177 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/🔣️.json | $defs | 180 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🧊️brep/🔣️.json | $defs | 172 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🫧️surface-model/🔣️.json | $defs | 181 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔣️.json | $defs | 145 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json | $defs, nonpointer-reference | 153 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json | $defs, nonpointer-reference | 158 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️draughting/🧬️schema/🔣️.json | $defs | 170 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📽️projection/🧬️schema/🔣️.json | prefixItems | 170 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🗺️cmap/🧬️schema/🔣️unicode.json | closed candidate | 182 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🔤️text-string/🧬️schema/🔣️.json | closed candidate | 168 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧬️schema/🛫️cos-output.json | $defs | 198 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🏭️issuer/🧬️schema/🔣️.json | closed candidate | 119 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🔎️lookup/🧬️schema/🔣️.json | closed candidate | 138 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🔢️version/🧳️retained/🧬️schema/🔣️.json | closed candidate | 151 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🔺️change/🧬️schema/🔣️.json | closed candidate | 138 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🔺️change/🧳️part/🧬️schema/🔣️.json | closed candidate | 146 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🧬️schema/🔣️.json | closed candidate | 128 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🧳️retained/🧬️schema/🔣️.json | closed candidate | 140 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/📸️snapshot/🔣️.json | closed candidate | 121 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🔣️.json | closed candidate | 109 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🔺️diff/🔣️.json | closed candidate | 117 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🔺️diff/🧳️retained/🗺️overlay/🧬️schema/🔣️.json | closed candidate | 150 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🔺️diff/🧳️retained/🪡️apply/🧬️schema/🔣️.json | closed candidate | 148 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/📄️replace-part/🧬️schema/🔣️.json | closed candidate | 148 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/📦️replace-package/🧬️schema/🔣️.json | nonpointer-reference | 151 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/🔣️.json | nonpointer-reference | 122 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/🧳️retained/🧬️schema/🔣️.json | closed candidate | 144 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧵️validation/🧬️schema/🔣️.json | closed candidate | 187 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-native-region/🧬️schema/🔣️.json | closed candidate | 205 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🧬️schema/🔣️.json | closed candidate | 204 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-native-samples/🧬️schema/🔣️.json | unknown-keywords (x-semio-ui) | 192 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/🏅️conformance/🧮️facts/🔣️.json | closed candidate | 181 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🧬️schema/🔣️.json | closed candidate | 185 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/↔️elongation/🔣️.json | unknown-keywords (x-semio) | 187 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/🔣️.json | unknown-keywords (x-semio) | 186 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🔣️.json | unknown-keywords (x-semio) | 189 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🔣️.json | unknown-keywords (x-semio) | 191 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🔣️.json | unknown-keywords (x-semio) | 190 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/📏️overall-size/🔣️.json | unknown-keywords (x-semio) | 184 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/➰️boundary-loops/🔣️.json | unknown-keywords (x-semio) | 189 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🔢️genus/🔣️.json | unknown-keywords (x-semio) | 181 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🕳️holes/🔣️.json | unknown-keywords (x-semio) | 181 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🥯️handles/🔣️.json | unknown-keywords (x-semio) | 183 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🧮️euler-characteristic/🔣️.json | unknown-keywords (x-semio) | 196 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🧬️schema/🔣️.json | closed candidate | 187 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧬️schema/🔣️.json | $defs | 197 |
| ✏️s/🔌️plugins/🧱️block/🗂️catalog/🧬️schema/🔣️.json | closed candidate | 99 |
| ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🏗️construction/🧬️schema/🔣️.json | closed candidate | 142 |
| ✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧬️schema/🔣️.json | $defs, unknown-keywords (x-semio-numeric-policy, x-semio-operation-contract) | 114 |
| ✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🚪️io/🧬️schema/🔣️.json | closed candidate | 103 |
| 🌎️hub/🧩️compositions/🗄️stdio/📇️publication/🧬️schema/🎯️factory-selector/🔣️.json | closed candidate | 131 |
| 🧰️framework/🔨️modules/⏯️tool-run/♻️retirement/🧬️schema/🔣️.json | closed candidate | 112 |
| 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛫️encode/🧬️schema/🔣️.json | closed candidate | 117 |
| 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/📊️progress/🧬️schema/🔣️.json | closed candidate | 130 |
| 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🔢️entities/🧬️schema/🔣️.json | prefixItems | 130 |
| 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🧬️schema/🔣️.json | closed candidate | 118 |
| 🧰️framework/🔨️modules/⏯️tool-run/👥️entities/📸️version/🧬️schema/🔣️.json | closed candidate | 122 |
| 🧰️framework/🔨️modules/⏯️tool-run/👥️entities/🧬️schema/🔣️.json | $defs | 111 |
| 🧰️framework/🔨️modules/⏯️tool-run/👥️entities/🧾️provenance/🧬️schema/🔣️.json | $defs | 125 |
| 🧰️framework/🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🧬️schema/🔣️.json | closed candidate | 120 |
| 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧬️schema/📍️point/🔣️.json | closed candidate | 151 |
| 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧬️schema/🔣️.json | closed candidate | 142 |
| 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json | closed candidate | 116 |
| 🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc/🧬️schema/🔣️.json | closed candidate | 116 |
| 🧰️framework/🔨️modules/⚙️runtime/🧲️native/🧰️toolchain/🧬️schema/🔣️.json | closed candidate | 121 |
| 🧰️framework/🔨️modules/⚠️diagnostic/🧬️schema/🎛️controlled/🔣️.json | $defs, nonpointer-reference | 115 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/⚖️result/🧬️schema/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🌀️support/🧬️schema/🔣️.json | closed candidate | 121 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎟️demand/🧬️schema/🔣️.json | closed candidate | 120 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📋️vector/🧬️schema/🔣️.json | closed candidate | 131 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🚼️construction/🧬️schema/🔣️.json | closed candidate | 137 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️original-vector/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️allocation-return/🧬️schema/🔣️.json | $defs | 131 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔐️mutex/🧬️schema/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/📦️original/🧬️schema/🔣️.json | closed candidate | 137 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔐️unique/🧬️schema/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🚚️frontier/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🧬️schema/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json | $defs | 116 |
| 🧰️framework/🔨️modules/🌱️value/⚠️refusal/🧬️schema/🔣️.json | $defs | 107 |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/📽️projection/🧬️schema/🔣️.json | closed candidate | 120 |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🔀️enum/🧬️schema/🔣️.json | closed candidate | 125 |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🧬️schema/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎮️splice/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📥️build/🧬️schema/🔣️.json | closed candidate | 128 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🔎️traversal/🧬️schema/🔣️.json | closed candidate | 132 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🧬️schema/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🧳️read/🧬️schema/🔣️.json | closed candidate | 127 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🔑️named/🧬️schema/🔣️.json | $defs | 122 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🚚️commit/🧬️schema/🔣️.json | closed candidate | 123 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🧬️schema/🔣️.json | $defs | 113 |
| 🧰️framework/🔨️modules/🌱️value/📋️list/🟰️equal/🧬️schema/🔣️.json | $defs | 114 |
| 🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/♻️original-lease/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/🧬️schema/🔣️.json | closed candidate | 112 |
| 🧰️framework/🔨️modules/🌱️value/📣️publication/🌳️root/🧬️schema/🔣️.json | closed candidate | 120 |
| 🧰️framework/🔨️modules/🌱️value/📦️paged/📖️read/🧬️schema/🔣️.json | closed candidate | 114 |
| 🧰️framework/🔨️modules/🌱️value/📽️projection/👁️post/🧬️schema/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🌱️value/🔑️lookup/🧬️schema/🔣️.json | closed candidate | 107 |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧬️schema/🔣️.json | closed candidate | 130 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/♻️retirement/🧬️schema/🔣️.json | closed candidate | 120 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧬️schema/🔣️.json | closed candidate | 123 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/📏️maximum/🧬️schema/🔣️.json | closed candidate | 118 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧬️schema/🔌️detached/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🧬️schema/🔣️.json | $defs | 125 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📋️list/🧬️schema/🔣️.json | closed candidate | 124 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🧬️schema/🔣️.json | closed candidate | 126 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧬️schema/🔣️.json | prefixItems | 116 |
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🧬️schema/🔣️.json | closed candidate | 128 |
| 🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧬️schema/🔣️.json | $defs | 106 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧬️schema/🔣️.json | closed candidate | 133 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🧬️schema/🔣️.json | closed candidate | 132 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧬️schema/🔮️demands/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️sealed-shared-list/📋️list/🧬️schema/🔣️.json | closed candidate | 145 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🧬️schema/🔣️.json | $defs | 144 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧬️schema/🔣️.json | $defs | 127 |
| 🧰️framework/🔨️modules/🌱️value/🫴️receiving/🧬️schema/🔣️.json | closed candidate | 110 |
| 🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/🪪️kind/🔣️.json | closed candidate | 121 |
| 🧰️framework/🔨️modules/🎒️pack/🌱️intrinsic/🎮️retained/🧬️schema/🔣️.json | $defs | 121 |
| 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧬️schema/🔣️.json | $defs, nonpointer-reference | 136 |
| 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/📨️wire/🧬️schema/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🧾️source/🧬️schema/🔣️.json | closed candidate | 125 |
| 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧬️schema/📊️table.json | prefixItems | 133 |
| 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧬️schema/🔣️.json | closed candidate | 128 |
| 🧰️framework/🔨️modules/🎒️pack/📐️format/🏷️symbols/🎮️decode/🧬️schema/🔣️.json | closed candidate | 127 |
| 🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🎟️reservation/🧬️schema/🔣️.json | closed candidate | 132 |
| 🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/📋️persistent/🧬️schema/🔣️.json | closed candidate | 136 |
| 🧰️framework/🔨️modules/🎒️pack/🔤️json/🧬️schema/🧩️members/🔣️.json | closed candidate | 115 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🧬️schema/🔣️.json | closed candidate | 151 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🧬️schema/🔣️.json | closed candidate | 160 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤫️silence/🧬️schema/🔣️.json | closed candidate | 159 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧬️schema/🔣️.json | closed candidate | 158 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧬️schema/🔣️.json | closed candidate | 151 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️conditional-retirement/🧬️schema/🔣️.json | $defs | 165 |
| 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/📚️loader/🧬️schema/🔣️.json | closed candidate | 138 |
| 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🔬️fingerprint/🧬️schema/🔣️.json | closed candidate | 143 |
| 🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧬️schema/🔎️inspection/🔣️.json | closed candidate | 128 |
| 🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧬️schema/🧭️component-transform/🔣️.json | $defs | 137 |
| 🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json | $defs, prefixItems | 114 |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🔏️fingerprint/🧬️schema/🔣️.json | closed candidate | 144 |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🕰️history/🧬️schema/📈️paging.json | closed candidate | 146 |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🕰️history/🧬️schema/🔣️.json | $defs | 140 |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🧬️schema/🔣️.json | $defs | 129 |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🪪️namespace/🧬️schema/🔣️.json | $defs | 142 |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🧬️schema/🔣️.json | closed candidate | 115 |
| 🧰️framework/🔨️modules/📡️replication/⚙️codec/🛬️retained/📦️bytes/🧬️schema/🔣️.json | closed candidate | 132 |
| 🧰️framework/🔨️modules/📡️replication/⚙️codec/🛬️retained/🧬️schema/🔣️.json | closed candidate | 123 |
| 🧰️framework/🔨️modules/📡️replication/⚠️error/♻️retirement/🧬️schema/🔣️.json | $defs | 124 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/❄️reference/🧬️schema/🔣️.json | closed candidate | 127 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🔗️concat/🧬️schema/🔣️.json | closed candidate | 137 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/↩️inverse-admission/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/📦️prepared-fields/🔣️.json | $defs | 134 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/⚠️diagnostic/🧬️schema/🔣️.json | closed candidate | 140 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔎️key/🧬️schema/🔣️.json | closed candidate | 134 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔺️diff/🧬️schema/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔺️diff/🫴️receiving/🧬️schema/🔣️.json | closed candidate | 148 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🧬️schema/🔣️.json | closed candidate | 127 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🧱️row/🧬️schema/🔣️.json | $defs | 134 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🩹️patch/🧬️schema/🎟️close/🔣️.json | closed candidate | 145 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🩹️patch/🧬️schema/🔣️.json | $defs | 136 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🧬️schema/🔣️.json | closed candidate | 136 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🛂️authority/🧬️schema/🔣️.json | $defs | 141 |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧾️transaction/🎟️admission/🧬️schema/🔣️.json | closed candidate | 143 |
| 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/📤️wire/🧬️schema/🔣️.json | closed candidate | 135 |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/👥️presence/🔍️borrowed/🧬️schema/🔣️.json | unknown-keywords (x-original-wire) | 133 |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/🧬️schema/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🧬️schema/🔣️.json | closed candidate | 138 |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/📥️bootstrap/🧬️schema/🔣️.json | closed candidate | 122 |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/🧬️schema/🏛️ownership/🔣️.json | closed candidate | 122 |
| 🧰️framework/🔨️modules/📡️replication/🧬️schema/🏛️crate-partition/🔣️.json | closed candidate | 122 |
| 🧰️framework/🔨️modules/🔲️pixels/📷️png/📥️decode/🧬️schema/🔣️.json | $defs, nonpointer-reference | 115 |
| 🧰️framework/🔨️modules/🔲️pixels/📸️jpeg/📥️decode/🧬️schema/🔣️.json | $defs | 116 |
| 🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧬️schema/🔣️.json | nonpointer-reference | 117 |
| 🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🎯️set-laws/🔣️.json | $defs | 115 |
| 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧬️schema/🌳️tree-emphasis/🔣️.json | prefixItems | 117 |
| 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧬️schema/📏️text-width/🪟️console/🔣️.json | $defs | 125 |
| 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧬️schema/🖼️chrome-emphasis/🔣️.json | closed candidate | 119 |
| 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧬️schema/🖼️window-contour/🔣️.json | closed candidate | 118 |
| 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧬️schema/🪟️drag-preview/🔣️.json | closed candidate | 116 |
| 🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🧬️schema/🔣️.json | closed candidate | 124 |
| 🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/📏️world3d-modelling/🔣️.json | $defs | 124 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🧬️schema/🎨️catalog/🔣️.json | closed candidate | 132 |
| 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🕰️host-temporal-format/🧬️schema/🔣️.json | $defs | 146 |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧬️schema/🔣️.json | closed candidate | 118 |
| 🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json | closed candidate | 111 |
| 🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔣️.json | $defs | 112 |
| 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/📝️shared-utf8/🧬️schema/🔣️.json | closed candidate | 131 |
| 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🧬️schema/🔣️.json | prefixItems | 118 |
| 🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/📡️scene/🧬️schema/🔣️.json | $defs, nonpointer-reference | 122 |
| 🧰️framework/🔨️modules/🚪️io/⏱️control/♻️retirement/🧬️schema/🔣️.json | prefixItems | 117 |
| 🧰️framework/🔨️modules/🚪️io/📸️snapshot/🧬️schema/🔣️.json | closed candidate | 106 |
| 🧰️framework/🔨️modules/🚪️io/🔀️stream-mux/🧬️schema/🔣️.json | $defs | 108 |
| 🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source/🧬️schema/🔣️.json | nonpointer-reference | 114 |
| 🧰️framework/🔨️modules/🚪️io/🧬️schema/⚠️refusal/🧬️schema/🔣️.json | nonpointer-reference | 114 |
| 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧬️schema/🧮️allocation/🔣️.json | closed candidate | 139 |
| 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧬️schema/🫙️projection/🔣️.json | closed candidate | 139 |
| 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🏛️relational/🔣️.json | $defs, unknown-keywords (x-sqlite) | 127 |
| 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🔣️.json | unknown-keywords (x-sqlite) | 113 |
| 🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows/🔣️.json | $defs | 128 |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧬️schema/🧊️cold-planning/🔣️.json | closed candidate | 142 |
| 🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️schema/✂️knife-cut/🔣️.json | $defs | 114 |
| 🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️schema/✂️loop-cut/🔣️.json | closed candidate | 113 |
| 🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🧬️schema/🎛️policy/🔣️.json | closed candidate | 131 |
| 🧰️framework/🔨️modules/🧮️math/📶️rank/🧬️schema/🔣️.json | closed candidate | 104 |
| 🧰️framework/🔨️modules/🧵️job/⏱️context/⏸️park/🧬️schema/🔣️.json | closed candidate | 112 |
| 🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🪶️weak/🧬️schema/🔣️.json | closed candidate | 123 |
| 🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🧬️schema/🔣️.json | closed candidate | 115 |
| 🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🧬️schema/🔣️.json | closed candidate | 126 |
| 🧰️framework/🔨️modules/🧵️job/♻️retirement/⚠️refusal/🧬️schema/🔣️.json | closed candidate | 118 |
| 🧰️framework/🔨️modules/🧵️job/🏃️work/🎟️grant/🧬️schema/🔣️.json | $defs | 112 |
| 🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧬️schema/🔣️.json | closed candidate | 108 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧬️schema/🎨️primary-geometry-style/🔣️.json | closed candidate | 152 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🏗️staging/🧬️schema/🔣️.json | closed candidate | 133 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🏗️build/🧬️schema/🔣️.json | closed candidate | 142 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧬️schema/🎟️finish/🔣️.json | closed candidate | 146 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧬️schema/🎟️normal/🔣️.json | closed candidate | 146 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧬️schema/🎟️plan/🔣️.json | closed candidate | 144 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🌱️genesis/🧬️schema/🌿️repository/🔣️.json | nonpointer-reference | 138 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🌱️genesis/🧬️schema/🔣️.json | closed candidate | 124 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🔎️measurement/🧬️schema/🔣️.json | closed candidate | 174 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧬️schema/♻️host/🔣️.json | closed candidate | 166 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧬️schema/📥️input/🔣️.json | closed candidate | 168 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧬️schema/🧾️receipt/🔣️.json | closed candidate | 170 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/📝️text/🌱️genesis/🧬️schema/🔣️.json | closed candidate | 138 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🧷️actor-reservation/📖️snapshot-read/🧬️schema/🔣️.json | closed candidate | 163 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/↩️inverse/🧬️schema/🔣️.json | closed candidate | 150 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧬️schema/🫴️grant/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🔗️binding/🧬️schema/🔣️.json | closed candidate | 138 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🪪️identity/🧬️schema/🔣️.json | closed candidate | 139 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️outcome/🧬️schema/🔣️.json | closed candidate | 126 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️schema/🔗️source-custody/🔣️.json | closed candidate | 151 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧬️schema/🔣️.json | closed candidate | 149 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🧬️schema/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧬️schema/🔣️.json | closed candidate | 159 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/♻️retirement/🧬️schema/🔣️.json | closed candidate | 136 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/♻️retirement/🧬️schema/🔣️.json | closed candidate | 148 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/🌱️birth/🧬️schema/🔣️.json | closed candidate | 144 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🪞️projection/🧬️schema/🔣️.json | closed candidate | 137 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧬️schema/🔣️.json | closed candidate | 137 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/🎚️field-set/🔣️.json | closed candidate | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/📦️codec-ownership/🔣️.json | closed candidate | 140 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📸️native-identity/🔣️.json | closed candidate | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧬️schema/📦️assembly/🔣️.json | $defs | 145 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🪪️metadata/🧬️schema/🔣️.json | closed candidate | 145 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/⚠️failure/🧬️schema/🔣️.json | nonpointer-reference | 143 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🧬️schema/🔣️.json | closed candidate | 143 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/⚠️failure/🧬️schema/🔣️.json | nonpointer-reference | 143 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/🧬️schema/🔣️.json | $defs | 143 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧬️schema/🎟️caller/🔣️.json | closed candidate | 129 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧬️schema/🪜️stepped-driver/🔣️.json | closed candidate | 137 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧺️values/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧬️schema/🔣️.json | closed candidate | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧬️schema/🎬️media-export-source/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧬️schema/🪪️document-identity/🔣️.json | closed candidate | 145 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🧬️schema/🔣️.json | $defs | 166 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📋️data/🧬️schema/🔣️.json | closed candidate | 174 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/⏳️boot-lifecycle/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/⏳️owned-service-status/🔣️.json | closed candidate | 153 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🔣️flow-add-widget/🔣️.json | $defs | 149 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🧩️plugin-modules/🔣️.json | $defs, unknown-keywords (x-semio-admission) | 148 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🪟️input-window/🔣️.json | closed candidate | 146 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧬️schema/📝️explicit-draft/🔣️.json | $defs | 173 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧬️schema/🔣️gumball-component-events/🔣️.json | closed candidate | 185 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧬️schema/🧭️gesture-targets/🔣️.json | $defs | 176 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🔣️.json | $defs | 166 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🧬️schema/✏️editable-text/🔣️.json | closed candidate | 167 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📡️scene/🧬️schema/🔣️.json | $defs, nonpointer-reference | 164 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧬️schema/📡️export-wire/🔣️.json | closed candidate | 175 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧬️schema/📤️export-request/🎟️original-close.json | closed candidate | 192 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧬️schema/📤️export-request/🔣️.json | $defs | 178 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🪙️authority/🖼️frame/🧬️schema/🔣️.json | nonpointer-reference | 152 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/📮️port/♻️retirement/🧬️schema/🔣️.json | closed candidate | 148 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/💼️job/📤️receipt/🧬️schema/🔣️.json | closed candidate | 163 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🧬️schema/🔣️.json | closed candidate | 140 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/📦️completed/🧬️schema/🔣️.json | closed candidate | 160 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🏗️opening/🧬️schema/🔣️.json | closed candidate | 149 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️lease/🧬️schema/🔣️.json | closed candidate | 136 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/⚠️receipt/🔣️.json | prefixItems | 143 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🔗️alias/🔣️.json | closed candidate | 142 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🔣️.json | $defs | 133 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🧾️context/🔣️.json | $defs | 144 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🧬️schema/🔣️.json | closed candidate | 141 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🚪️session/🎟️pending/🧬️schema/🔣️.json | closed candidate | 145 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/⭐️default/🧬️schema/🔣️.json | $defs, maxContains, minContains | 152 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🧬️schema/🔣️.json | $defs, unknown-keywords (x-authority, x-close, x-ownership, x-transaction) | 152 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️entity-id/🔣️.json | closed candidate | 152 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️birth/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧪️inference-admission/🧬️schema/🔣️.json | prefixItems | 161 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧬️schema/🛂️receiving/🔣️.json | closed candidate | 137 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🧬️schema/input-wallet.json | $defs | 156 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/♻️retirement/🧬️schema/🔣️.json | $defs, prefixItems | 145 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧬️schema/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/⚠️refusal/🧬️schema/🔣️.json | $defs | 146 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🔣️.json | closed candidate | 126 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema/♻️retirement/🔣️.json | closed candidate | 149 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema/📥️input/🔣️.json | closed candidate | 145 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema/📨️metadata/🔣️.json | closed candidate | 148 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema/🔣️.json | closed candidate | 136 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🖼️render/🎟️demand/🧬️schema/🔣️.json | closed candidate | 167 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/♻️metadata/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔗️read/🧬️schema/🔣️.json | closed candidate | 144 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️context/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🏠️catalog/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗂️registry/🧬️schema/🔣️.json | closed candidate | 148 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/🧬️schema/🔣️.json | closed candidate | 147 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/📬️mutation/🔄️refresh/🧬️schema/🔣️.json | closed candidate | 149 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🧬️schema/🔣️.json | closed candidate | 163 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️operation/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧳️retained/🧬️schema/🔣️.json | closed candidate | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🪪️bindings/🏛️catalog/🧬️schema/🔣️.json | $defs, nonpointer-reference | 135 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🪪️bindings/🧬️schema/🔣️.json | $defs | 124 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🧬️schema/✅️verdict/🔣️.json | closed candidate | 134 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🏗️admission/🧬️schema/🔣️.json | closed candidate | 150 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🧬️schema/🔣️.json | $defs, prefixItems | 141 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/✅️native-controls/🔣️.json | $defs | 116 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json | $defs | 112 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🎨️color/🔣️.json | unknown-keywords (x-semio-named-colors) | 128 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🔣️.json | nonpointer-reference | 119 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🔣️.json | $defs, nonpointer-reference | 110 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🔣️.json | nonpointer-reference | 106 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json | $defs, unknown-keywords (x-semio-demo-tables, x-semio-description, x-semio-family-options, x-semio-guide-defaults, x-semio-guide-geometry, x-semio-guide-units, x-semio-option-syntax, x-semio-taxonomy-groups, x-semio-taxonomy-sections) | 98 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🔣️.json | closed candidate | 111 |
| 🧰️framework/🛍️products/📓️print/🧾️template/🎓️thesis/🧬️schema/🔣️.json | closed candidate | 120 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/🧬️schema/🔣️.json | closed candidate | 124 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/📁️root-scope/🧬️schema/🔣️.json | $defs | 140 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧬️schema/🏗️build/🔣️.json | nonpointer-reference | 123 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧬️schema/🚪️entrypoint/🔣️.json | closed candidate | 128 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/🧬️schema/🔣️.json | $defs | 116 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🧬️schema/🔣️.json | closed candidate | 129 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎛️control-policy/🧩️products/🧬️schema/🔣️.json | closed candidate | 151 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/⌨️batch/🔣️.json | $defs | 129 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/⌨️footer-hints/🔣️.json | $defs | 136 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔌️socket/🔣️.json | $defs | 141 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌐️open/🔣️.json | $defs | 129 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎛️dispatch/🔣️.json | closed candidate | 133 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json | $defs, prefixItems | 133 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🏛️repo-action/🔣️.json | $defs | 136 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔎️source-discovery/🔣️.json | $defs | 141 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔣️.json | $defs, prefixItems | 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️authority/🔣️.json | $defs | 134 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️journeys/🔣️.json | $defs | 133 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🪟️windows/🔣️.json | closed candidate | 132 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json | $defs | 119 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/🧬️schema/🔣️.json | $defs | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/🧬️schema/🔣️.json | $defs | 123 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧬️schema/🔣️.json | $defs | 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧭️root-selection/🧬️schema/🔣️.json | $defs | 139 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json | $defs | 141 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧬️schema/🔣️.json | $defs | 137 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧬️schema/🔣️.json | $defs, format-policy | 119 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧬️schema/🆔️artifact-id/🔣️.json | $defs | 132 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧬️schema/🔣️.json | $defs | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧬️schema/🔣️.json | closed candidate | 174 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧬️schema/🔣️.json | $defs | 165 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🧬️schema/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧬️schema/🔣️.json | $defs | 156 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🧭️package-inventory/🧬️schema/🔣️.json | $defs | 163 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/⌨️commands/🧬️schema/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json | closed candidate | 183 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧬️schema/🎯️project.json | closed candidate | 151 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧬️schema/🎯️units.json | closed candidate | 149 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json | unknown-keywords (optionalPath, policy, protocol) | 149 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🗂️closures/🧬️schema/🔣️.json | closed candidate | 164 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json | closed candidate | 158 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🧬️schema/🔣️.json | closed candidate | 162 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/🧬️schema/🔣️.json | closed candidate | 159 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🧬️schema/🔣️.json | closed candidate | 161 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json | $defs | 191 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧬️schema/🔣️.json | closed candidate | 161 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🧬️schema/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔀️merge/🧬️schema/🔣️.json | closed candidate | 161 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔐️custody/🧬️schema/🔣️.json | closed candidate | 163 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧬️schema/🔣️.json | closed candidate | 164 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧬️schema/🔣️.json | closed candidate | 162 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json | closed candidate | 152 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🧬️schema/🔣️.json | closed candidate | 166 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧱️chunk-classes/🧬️schema/🔣️.json | closed candidate | 169 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin/🧬️schema/🔣️.json | closed candidate | 159 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json | $defs | 147 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧬️schema/🔣️.json | closed candidate | 138 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️owners.json | $defs | 134 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📣️publication.json | closed candidate | 139 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json | closed candidate | 137 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json | $defs | 128 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🗑️reset.json | $defs | 133 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json | $defs | 137 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️selection.json | closed candidate | 137 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🛗️zero-touch-closure.json | prefixItems | 146 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🧾️owner-consistency.json | $defs | 145 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🪶️lean.json | closed candidate | 132 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🐹️go-layout/🧬️schema/🔣️.json | $defs | 145 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📋️policy/⏯️tool-run/🧬️schema/🔣️.json | $defs | 153 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧬️schema/🔣️.json | closed candidate | 164 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📖️toml/🧬️schema/🔣️.json | $defs | 150 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📤️arguments/🧬️schema/🔣️.json | closed candidate | 155 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json | $defs | 164 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/🔣️.json | $defs | 151 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️cargo-dependency-direction/🔣️.json | $defs | 149 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction/🔣️.json | $defs | 143 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-selectors/🔣️.json | $defs, prefixItems | 143 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️rust-source-direction/🔣️.json | $defs | 144 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️rust-source-direction/🧾️attributes/🔣️.json | $defs, nonpointer-reference | 158 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧬️schema/🔣️.json | $defs | 120 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos/🧬️schema/🔣️.json | $defs | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧬️schema/🔣️.json | $defs | 118 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧬️schema/🔣️event-kinds.json | unknown-keywords (kinds, schemaVersion) | 129 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧬️schema/🔣️.json | $defs, unevaluatedProperties | 115 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧬️schema/🔣️.json | $defs | 118 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📦️publication/🧬️schema/🔣️.json | closed candidate | 159 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧬️schema/🔣️.json | $defs | 120 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/🧬️schema/🔣️.json | $defs, $ref-assertion-siblings | 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🧬️schema/🔣️.json | $defs | 132 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/🧬️schema/🔣️.json | $defs | 116 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors/🧬️schema/🔣️.json | $defs | 124 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/🧬️schema/🔣️.json | $defs | 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧬️schema/🔣️.json | $defs | 130 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/💾️storage/🧬️schema/🔣️.json | closed candidate | 127 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/🧬️schema/🔣️.json | $defs | 116 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/🧬️schema/🔣️.json | $defs | 117 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/🧬️schema/🔣️.json | $defs | 120 |
