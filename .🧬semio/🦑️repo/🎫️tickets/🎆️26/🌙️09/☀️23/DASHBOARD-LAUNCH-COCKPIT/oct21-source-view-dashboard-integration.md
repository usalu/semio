# Source View Dashboard Integration

Status: two source files saved; no Rust compilation or runtime acceptance. Controls owns the new semantic actor, ports, effects, model crates and terminal source adapter.

Dashboard now owns the actual source-view Controller and ordered pending Effect collection instead of three separate job/operation/pending fields. Files and Document windows mount the source owner’s real contents below their existing allocated root, preserving the Body variants, list/caption handles and labels. Four management actions use the source owner’s EntryAction directly. Each frame flushes source effects before reading focus or refreshing panes/documents, so typed effects apply after the actor’s borrowed operation has ended. No compatibility forwarding layer, copied window model or new runtime library is introduced.

These edits implement the already schema-first source-view boundary, whose eight portable settlement cases, nine retained source UI laws and seven new terminal laws remain under the respective owning suites. All actual native acceptance remains pending. The two files reduce raw source size by 302 bytes together; this is not a complete terminal admission measurement. Controls must measure the final real closure under the unchanged source cap after its semantic extraction.

Exact stable current source observations are in generated/e2e-source-view-integration-inputs.json. Source paths:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⌨️controls/🦀️.rs`

Controls requested one final narrow visibility change: existing windows::node_constraint is now pub(super), with its canonical geometry body unchanged, so the borrowed source adapter reuses the actual root constraint instead of duplicating it. Exact resulting windows observation: {"path":"🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs","bytes":27081,"sha256":"014f448eb3c41c074208537c88ffb81eb31991bf84a4f5818cb79a3d1c84b81d","purpose":"Existing canonical geometry helper visible to terminal sibling source adapter; body unchanged"}. No Rust compiler/runtime acceptance.

Low fresh review found the helper already carried pub(super) when Root applied its requested visibility edit, yielding duplicate visibility. Root corrected exactly that duplicate to one pub(super), with fresh byte-equality protection and no helper body change. This supersedes the preceding 27081-byte observation; current windows is27070 bytes/SHA b23c780d1f67b95a8b82ef247f7dfc30d5bc32c4157daa16fa0c681e6734a255. No compiler acceptance is claimed.
