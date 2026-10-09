# Runtime Artifact IO Exploration

Read-only audit conducted on 2026-10-06. No source changes or tests were run during exploration. Root, s, and procedural AGENTS.md instructions were read.

## Chosen Boundary

Use `io/<binary|text>/<snapshot|diff|mutations|inferences>/[member]` for representation contracts, schemas, grammar, encoding, decoding, parsing, printing, and wire validation. Keep domain mutation definitions and apply/inverse behavior in schema mutations. Reuse existing traits and remove schema codec aliases when updating consumers.

## Generation3d

The user-specified `schema/mutations/binary` Rust file is a mixed module of roughly 3,700 lines, not merely a representation stub. Its TypeScript counterpart declares a Uint8Array alias. Rust lines 1–225 contain normative protocol resources, the Generation3dOperationDsl mirror, OpText/OpBinary implementations, mapping helpers, encode_op/decode_op, and codec unit tests. These belong in IO.

Lines 228 onward contain publication admission and leases, bounded credits, host snapshot replay, retirement/copy frontiers, retained mutation and snapshot ownership, document-store owners, envelope field decoding authority, and retained-authority laws. Separate publication/store/host ownership from codec implementation rather than relocating the entire mixed module unchanged. The envelope decoding authority is IO-facing but must preserve its explicit ownership and cancellation contracts.

The text mutation facet currently reexports apply/inverse functions and the mutation union while owning grammar constants. Move grammar to IO and import domain types directly. The binary codec currently imports the union through the text facet; remove this dependency inversion.

## Inline Mutation Roots

The candidate scan below returned 113 Rust mutation-root modules, including main artifact schemas and editor config, presence, and transient schemas:

```text
rg -l 'impl (protocol::)?OpText|impl (protocol::)?OpBinary|fn (encode|decode)_[[:alnum:]_]+json' ✏️s/🔌️plugins -g '*.rs' | rg '/🧬️mutations/🦀️.rs$'
```

Path relocation alone therefore does not finish runtime abstraction. Confirmed examples include:

- Playbook root: decode_playbook_mutation_json, decode_playbook_snapshot_json, encode_playbook_snapshot_json, around lines 85–98.
- Forms, note, shooting and presentation mutation roots: JSON mutation/snapshot parsing or projection encoding.
- Norm artifacts: JSON mutation decoding across en1997, en1998, en1999, vdi3805, din4108 and others.
- Stdio semio subsets: JSON mutation decoding for document, table, text, flow, cad, video and others.
- Stdio CSV, TSV, HTML, Markdown, STEP, IFC, DXF, PLY, STL, DOCX and XLSX mutation roots: OpText parsing/printing and associated helpers.
- GIS terrain binary facet: encode_op/decode_op; text facet reexports domain mutation functions.

Avoid treating JSON.stringify used solely in diagnostic messages as an IO boundary violation. Likewise parse_layer_field_input and XML parse_part helpers require semantic review: some parsing supports domain edits, rather than wire representation.

## Validation Leads

Generation3d existing Nx project `@semio-tech/procedural-generation3d-rs` provides canonical-architecture, semantic-wire-check, verify-generation3d-document-io, test-snapshot-sqlite, test-snapshot-sqlite-native and test-snapshot-sqlite-source. Existing binary unit tests exercise unknown OpText operations. Retained-authority tests must keep validating bounded ownership and release after extraction.

The binary facet includes a language-agnostic semantic-wire JSON corpus decoded through serde_json in Rust. Preserve that authored corpus and run existing semantic-wire checks after relocating imports. serde_json provides a third-party JSON oracle; additional oracle checks should compare canonical values or wire bytes with authored expected outputs rather than merely checking round trips. TypeScript fixture validation should use an existing test dependency where available, without adding a runtime dependency.

## Verification Limits

The 113 count is a source-pattern candidate inventory, not 113 individually classified defects. No runtime logs, compilation, or test execution occurred in this read-only exploration. Line references describe the files at exploration time and can shift during concurrent work.
