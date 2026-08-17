# W1 OOXML Functional Repair: DOCX, PPTX, XLSX

## Scope

This shard repaired source-level DOCX, PPTX, and XLSX package transport and retention defects identified in `📋️w1-stdio-full-lib-baseline.md`. The work was source-first and did not edit fixtures. Cargo and Nx were intentionally not run because the runtime host-full lane owns the serialized build lock.

## P0 Findings

None.

## P1 Findings And Repairs

### OPC part order was being discarded on decode

The shared OPC decoder consumed the ZIP central-directory order and then sorted `OpcPackage.parts` by pathname. That made untouched OOXML packages change semantic snapshot equality and moved opaque parts, even though the ZIP decoder itself now preserves central-directory order. DOCX integrated native round trips and XLSX/PPTX package retention therefore had a source-level losslessness break.

`decode_opc` now retains the authoritative non-metadata part order. A new `encode_opc_with_package_order` emits `[Content_Types].xml`, the root relationship part, owner relationship parts in package order, and then the package parts in their authoritative order, with deterministic sorting only for unreferenced leftovers. DOCX and XLSX native encoders use this path. Unknown and opaque OPC parts remain byte-for-byte payloads.

### DOCX and XLSX native encoders did not retain canonical package order

DOCX and XLSX called the generic pathname-sorted OPC encoder. Their committed demo packages intentionally place relationship metadata and opaque parts in a stable package order (`word/styles.xml`/`word/numbering.xml` and `xl/styles.xml` around the typed workbook parts). Both native encoders now use the package-order OPC path. This keeps `ArtifactDsl`, `ArtifactPack`, native round trips, and mutation snapshot retention on one deterministic byte layer without weakening fixture assertions.

The DOCX and XLSX grammar leaves already describe the actual XML-part syntax emitted by their native serializers, including double-quoted attributes and no-space self-closing tags. No grammar or fixture was hand-edited.

### PPTX snapshot transport was not native OPC

The public PPTX snapshot DSL/pack implementation still routed through `PptxSnapshotRecord`, while its committed `.dsl.semio` and `.pack.semio` assets contain native OPC ZIP bytes. It now strips the Semio envelope, validates hexadecimal transport, and delegates to `decode_pptx`; printing and packing delegate to `encode_pptx`. `PptxSnapshotRecord` remains solely for the `SetSnapshot` mutation payload codec and was not removed.

### PPTX strict relationship resolution was incomplete

PPTX export previously recognized only the Transitional officeDocument relationship. A Strict package could therefore be treated as missing its authoritative presentation XML and regeneration could append a duplicate Transitional root relationship. Export now uses the shared resolver for both Transitional and Strict relationship URIs and only adds a Transitional relationship when neither form exists. Existing slide-master/layout/theme parts remain untouched by regeneration.

## P2 Findings / Follow-Up

- The baseline fixture assertions still require a serialized runtime gate after these source changes. If any committed bytes remain stale, regenerate them only with the verified native encoders; no fixture was rewritten in this shard.
- DOCX `sniff_docx_bytes` remains Transitional-only while full decode already accepts Strict. A dedicated Strict sniff test/fix should be handled with the broader dialect-composition work, not by weakening this shard's fixture laws.
- The existing XLSX hand-rolled mutation text and binary codecs already encode/decode the complete `SetSnapshot`, sheet, cell, formula-cache, shared-string, and OPC payload shapes. This shard did not replace them with a lossy generic codec.

## Changed Source Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`

## Static Verification

Passed without Cargo/Nx:

- `rustfmt --edition 2021 --check` on all five changed Rust source files.
- `git diff --check` on all five changed Rust source files.

## Exact Runtime Gate To Run When The Lock Is Available

Run the following focused tests in the stdio library target, then the full library gate:

### DOCX

- `demo_subset_integrated_roundtrip`
- `committed_facet_files_parse`
- `grammar_conformance_law`
- `protocol_walk_law`
- `fixture_honesty_law`
- native package/opaque-part retention tests

### PPTX

- `committed_facet_files_parse`
- `grammar_conformance_law`
- `ops_grammar_conformance_law`
- `diff_grammar_conformance_law`
- `protocol_walk_law`
- `fixture_honesty_law`
- `unmodeled_slide_master_survives_decode_encode_logically`
- Strict package composition and Transitional-rejection tests
- mutation and diff codec retention laws

### XLSX

- `committed_facet_files_parse`
- `grammar_conformance_law`
- `ops_grammar_conformance_law`
- `diff_grammar_conformance_law`
- `protocol_walk_law`
- `fixture_honesty_law`
- mutation `codec_retention_law` and `op_text_binary_roundtrip_law`

No runtime pass claim is made here because the serialized Cargo/Nx gate was not run.
