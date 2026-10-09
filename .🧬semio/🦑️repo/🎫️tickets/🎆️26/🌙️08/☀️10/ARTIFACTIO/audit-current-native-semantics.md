# Current Native Semantic Audit

Read-only source audit on 2026-10-08. No implementation, foundation, git, ticket lifecycle or goal lifecycle changes. No native execution was run; these are source findings, not runtime receipts. Census 99007 GREEN remains an ownership result, not semantic closure.

## Actionable Findings

### Raster Selection Is Persisted Representation Text

Root: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any`.

- `✏️editor/🎯️selection/🦀️.rs:8-18` defines the first-party selection owner but its semantic coverage is `spans:String`. `selection_spans:54-72` parses JSON at line 56 and converts intrinsic numbers through `as_f64` at line 64, then casts to usize. Ownership must be scalar typed start/length/coverage rows, with direct validation.
- `✏️editor/🎚️config/🦀️.rs:524-526` persists this owner through SetPixelSelection. This is a config semantic mutation boundary, not merely a fixture codec. Config `🧬️schema/🟦️.ts:138-149` repeats the string decoder, and JSON Schema `🔣️.json:151` declares string spans. Neutral selection fixture has encoded strings, including valid and invalid payloads.
- `✏️editor/🎮️commands/🎭️mask-from-selection/🦀️.rs:16` exposes `selection:String`; prepare at line 52 decodes this representation before creating actual image coverage. `🎮️commands/🖌️paint-stroke/🦀️.rs:111` also decodes the persisted selection.
- Host `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.ts:84-94` builds JSON spans and `restoreSelection:110-126` parses them. Those consumer paths must change with the owner, not just the schema validator.

### Puzzle Production APIs Still Own Third-Party JSON Values

Root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts`; each path continues `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`.

- `◻️2d:523,539,554` implements MutationDiff<Value>, DiffAlgebra<Value>, Mutation<Value> with `Value` imported from serde_json at line20. Production apply converts JSON into DslValue, typed snapshot, then emits JSON Value. Diff algebra lines541,545,546 silently defaults a refused typed decode. The owned typed impl does not eliminate this external API.
- `🧊️3d:834,850,865` repeats these production interfaces; lines852,856,857 silently default refused snapshots.
- `🖐️5d:487,504,519` repeats these production interfaces; lines506,510,511 silently default. Its normalization path at480 also defaults a failed catalog decode. These paths are not cfg(test); nearby comments describing them as old/out of scope are historical data, not authorization to retain them.
- Play snapshot wrappers additionally convert external Value and default failures at 2d652, 3d952, 5d613. The 2d wrapper stores typed Arc plus OnceLock<Arc<Value>> at644-646; new(Value) retains the supplied projection even when typed decode defaults. The actual 2d editor `✏️editor/🦀️.rs:4997` uses this wrapper as its Snapshot; host conversion paths at3091,3190,4045,4494,4501 convert JSON nodes/handles/catalog rows. This is a present host consumer, not an orphan codec. A complete repair must update their actual host callers, preserve refusal, and retire representation adapters.

### Note Public Unused JSON Mutation Helper

`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:485` exports `patch_block_field(... value:&serde_json::Value)` in production. Its field dispatcher coerces an invalid name to empty at496, visible to true at503, locked to false at518. Repository plugin rg found only the definition (patch-blocks command docstring at23 states it replaces this old helper), so removal is likely preferable to introducing another boundary. Verify broader reexports/callers before removing.

### Already Assigned or Separately Owned

Print grouping/render styles/point JSON are root-owned current repairs. GIS compiler relocation is root-owned. PDF logical role, font, sample, palette, ICC, CID and mesh ownership belong to PDF peers; no conclusions about their current moving source are asserted here. JPEG/TIFF dangling declarations and old Pack/native fixtures remain explicit handoff work from jpeg-tiff-native-resume-handoff.md, not rerun results.

## Candidate Classification

143 recorded calls cover the exact source paths below. Surrounding definitions were read. Counts are the pinned census counts, not a claim that concurrently changing files still have the same lines.

- Print transform path JSON is artifact semantic text; group/style comparison JSON is semantic comparison and root is repairing it. Print script stringify is failure reporting.
- Procedural phased-job test support JSON is fixture admission and corner witness comparison.
- Repo library calls parse source declarations/titles, detect duplicate JSON keys, derive schema fields or memoize source lookup. They do not encode artifact snapshots. Test orchestration calls parse fixtures/Gherkin and schema declarations, compare schema layouts, emit diagnostic/census output or key report rows.
- Coordinator stringify calls report expected field sets. glTF contract calls compare test results. ZIP stringify quotes entry names in diagnostic refusal. DAG and Semio kit stringify quote diagnostic paths/ids. Remodeling JSON occurs only in rejection messages. PDF const stringify occurs only in validator diagnostics.
- OS shell/renderer/MCP calls compare validator const/enum/unique items or quote failures; they operate presentation/host contracts. Entity catalog parse/projection/execution is source generator IO and source-literal escaping. Deployment encode/decode owns deployment metadata, registry parse owns descriptor channel, browser-bundle child stringify reports faults.
- Directory JSON owns bounded signed host protocol envelopes (page, receipt, command, authority, creation and check-in), not artifact snapshot IO. The preference mutation body is bounded uninterpreted JSON string (directory root:210-218); its actual vocabulary clients require an independent semantic review if preference representation is in scope.
- Dev validation Ajv is host development contract compiler. GIS compiler is test-only by actual sole consumer. Raster config parse is the artifact-adjacent persisted owner leak detailed above.

## Inspection Limits

Native Rust discovery inspected 8,935 non-test-path schema files and found 141 broad import/codec pattern matches; many are inline cfg(test), comments, or owned module names containing image. Forms FormDictionary serde impls at44/46 and Layout ChangeDataFields at18/20 are explicitly cfg(test), so they are excluded. The native scan is candidate discovery rather than a claim of every native function being semantically inspected. Relevant Puzzle, Note and Raster enclosing definitions were read directly. No dependency guards were executed.

## Exact Pinned Candidate Manifest

- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧮transform/🟦️.ts` — 3 calls; current file present
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⏱️phased-job/🧰️test-support/🟦️.ts` — 3 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/🏷️export-identity/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️json-document/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔣️json-schema/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` — 43 calls; current file present
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🟦️.ts` — 2 calls; current file present
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🟦️.ts` — 4 calls; current file present
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts` — 2 calls; current file present
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧪️contract/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts` — 7 calls; current file present
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts` — 6 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` — 18 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🟦️.ts` — 4 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts` — 6 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts` — 2 calls; current file present
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🛂️validation/🟦️.ts` — 1 calls; current file present
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️.ts` — 1 calls; current file present
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🔨️modules/🧬️schema/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` — 2 calls; current file present
- `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏃️execution/🟦️.ts` — 1 calls; current file present
- `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts` — 8 calls; current file present
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts` — 1 calls; current file present
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🟦️.ts` — 1 calls; current file present
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` — 2 calls; current file present
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧬️schema/🟦️.ts` — 2 calls; current file moved or removed

Pinned entries: 143. Unique candidate files: 38.
