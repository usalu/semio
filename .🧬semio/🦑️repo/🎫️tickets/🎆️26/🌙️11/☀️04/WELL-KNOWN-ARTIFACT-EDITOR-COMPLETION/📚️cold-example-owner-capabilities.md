# Cold Example Owner Capabilities

Read-only current source audit. No build, native test, browser, or production edit was performed.

All nine canonical fallback owners already have retained editing factories. None of their current native command rosters/parsers accepts setActiveExample. This is a missing command/work route, not merely an omitted factory registration.

Paths below are relative to ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts. Each editor file ends in ✏️editor/🦀️.rs.

| Owner | Subset path | Existing retained native route | Command/parser anchors |
| --- | --- | --- | --- |
| DOCX | 📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base | bounded_native_editing_editor! :547; docx_set_page_work :559; preparation::route :560 | roster :63 set-page/set-run-formatting; parser :462 |
| ZIP | 🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base | handwritten BoundedNativeEditingEditor :205; ArchiveTextWork :212; preparation::route :228 | roster :102 set-node; parser :154 |
| IDML | 📚️idml/🏅️standards/🔖️8.0/🪆️subsets/✳️any | bounded_native_editing_editor! :174 | roster :27 set-page; parser :105 |
| PPTX | 📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base | bounded_native_editing_editor! :196 | roster :50 set-page; parser :124 |
| XLSX | 📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base | bounded_native_editing_editor! :344 | roster :155 set-cell; parser :218 |
| PDF | 📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base | bounded_native_editing_editor! :258 | roster :96; parser :154; no example verb |
| EPW | 🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any | bounded_native_editing_editor! :276 | roster :110 set-cell; parser :150 |
| Binary | 💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any | ArtifactOwnedToolJobFactory BinaryTextCommandJobFactory :274; build_binary_text_tool_job :282 | roster :119 textEdit; parser :346 |
| Deflate | 🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any | ArtifactOwnedToolJobFactory DeflateTextCommandJobFactory :243; build_deflate_text_tool_job :251 | roster :121 textEdit; parser :315 |

## Shared registration and publication

Registry contract ✏️editing/🦀️.rs :69 emits BoundedNativeEditingEditor. snapshot_details_editor_support! :196 emits register_tool_job_factories :236. bounded_native:true invokes register_bounded_native_edit_tool_factory :241; explicit native factory invokes registry.register :245; shared snapshot-edit registration follows :252. Its build_tool_job :262 branches only on registered snapshot-edit/native IDs.

The same macro supplies the cold_mutation_apply_preparation_factory at :295–297, with optional routed native preparation. Thus existing store publication authority is reusable; a new example command must still carry genuine retained parsing/copy/edit work and preserve source/cancellation/receipt custody. Adding setActiveExample to a tool list while retaining the current reducer would reach command-mismatch errors.

DOCX concrete mutation support: base 🧬️schema/🧬️mutations/🦀️.rs :110 derives dsl::Mutations; base 🧬️schema/🔺️diff/🦀️.rs :212 implements ColdMutationDiff<DocxSnapshot>. ZIP base diff :182 implements ColdMutationDiff<ZipSnapshot>; its rename-entry and set-entry-data leaves implement ColdMutationKind. Binary diff :57, Deflate diff :44, and EPW diff :195 also explicitly implement ColdMutationDiff. These are cold mutation capabilities, not proof of a bounded whole-example snapshot replacement.

No literal SnapshotNativeReceive or SnapshotNativeDiff trait definition/implementation was found in the current framework/stdio Rust source searches. Do not invent those trait names or infer a receiver implementation from ToValue/FromValue, RetainedClone, DslRecord, or an existing editing macro. The appropriate actual native snapshot receiver/diff seam still needs identification before a generic example loader can claim reuse.

## Minimum safe route

Reuse the actual registered retained-tool infrastructure, but add a real typed example command/parser and a genuine bounded example-loading work implementation. The current fallback calls catalogue_snapshot_from_document and encode_pack before dispatching LoadDocument; existing set-page/set-node/textEdit workers do not make those whole-document operations funded.

This report is source capability evidence only. Real browser journeys remain zero. Latest separate native evidence: peer metadata tests actually executed three passes after root's fix; no Semio/DOCX SDK pass is inferred.
