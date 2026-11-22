# PDF view lane: SetView mutation descriptor fix

Cause: `mutation_source_authority` (dsl derive) reads `<owner>/🔣️.json` next to the leaf `🦀️.rs` (taxonomy `descriptorFilename`); it was missing, hence "No such file". The leaf sits directly under `🧬️mutations`, a flat owner (no domain-owner registration needed).

Created (all under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🫧️transient/🧬️schema/`):
- `🧬️mutations/👁️set-view/🔣️.json` leaf descriptor (14 required keys, owner = leaf dir, kind `set-view`, variant `SetView`, payloadSchema `🧬️schema/🔣️.json`, opcode `set-view`, binaryTag 0).
- `🧬️mutations/👁️set-view/🧬️schema/🔣️.json` payload schema, 7 fields, en+de x-semio-ui labels.
- `🧬️mutations/🔣️.json` aggregate oneOf schema.
- `🔣️.json` transient root schema.

Edited:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json`: three `$id` entries (`app/stdio/pdf/view/transient/...`) added under the stdio root, sorted. Regenerate via `repo run @semio-tech/dsl-derive-rs:generate-schema-index` to confirm byte equality.
- crate `📦️packages/🦀️rust/📋️project.json`: `$schema` had 14 `../`, fixed to 13.

Not done: generated surface files (`🔗️.graphql`, `🛰️.proto`, `🟦️.ts`, `🦀️.rs` beside the aggregate `🔣️.json`, as in the writer transient) are not authored; produced by the surface-schema generator.
Cargo.toml wiring and the pdf workspace membership/Cargo.lock were already in place.

## Follow-up: SetPresence descriptor (pdf `7.1.7/base/editor/👥️presence`)

Scan (static, python over `✏️s/🔌️plugins`, 5018 derive sites): the only `MutationLeaf` whose owner directory lacked `🔣️.json` was `👥️presence/🧬️schema/🧬️mutations/👁️set-presence` (all other pdf standards/subsets are complete; `🦠️mutation` payload-facet leaves everywhere have their descriptor in the parent, so they are not misses). No other plugin leaf is missing a descriptor. `Mutations` derives need only the direct leaf descriptor roster (mutations-expansion roster check), not an aggregate `🔣️.json`; aggregates without one elsewhere (kinds-catalog test derives, curation presence, forms/mathematical text io, home operations) are not required and were left alone.

Created under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/`:
- `🧬️mutations/👁️set-presence/🔣️.json` leaf descriptor (kind `set-presence`, variant `SetPresence`).
- `🧬️mutations/👁️set-presence/🧬️schema/🔣️.json` payload schema, 11 fields, en+de x-semio-ui labels.
- `🧬️mutations/🔣️.json` aggregate oneOf schema (`PdfPresenceMutation`).

Edited: schema-document-index.json (`app/stdio/pdf/presence/mutation/set-presence/schema.json`, `.../mutations.json`, sorted before `presence/schema.json`); crate `📋️project.json` `$schema` 14 -> 13 `../`.

Not done: the root `🧬️schema/🔣️.json` is generator output (`NoPresence`, empty, `do not edit`) and still says no fields although `PdfPresence` has 11; the surface-schema generator must be re-run to project it (not hand-edited). Generated `🔗️.graphql/🛰️.proto/🟦️.ts/🦀️.rs` for the aggregate also not authored.
