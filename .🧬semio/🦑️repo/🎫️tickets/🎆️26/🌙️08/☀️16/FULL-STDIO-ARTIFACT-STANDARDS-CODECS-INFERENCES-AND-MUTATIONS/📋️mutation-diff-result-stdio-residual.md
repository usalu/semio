# MutationDiff Result Migration — Residual Stdio Slice

## Scope

This source-first handoff covers exactly 25 residual implementations: BCF 2.1, AVI 1.0, MP4 ISO-BMFF, JSON RFC 8259, PNG 1.2, XML 1.0, TSV IANA, JPG JFIF 1.01, CSV RFC 4180, TIFF 6.0, Markdown CommonMark, SVG 1.1, binary raw, MP3 MPEG-1 Layer III, GIF 89a, BMP v3, GIF 87a, DEFLATE RFC 1950, TXT UTF-8, WAV RIFF PCM, EPW EnergyPlus, HTML5, ZIP 2.0, PDF 1.4, and DWG AC1024. glTF, Semio, STEP, and the other geometry/CAD revisions are excluded. Cargo/Nx were not run because the runtime host owns the serialized build lane.

## Changes

- All 25 `MutationDiff::apply` implementations return `MutationApplyResult<Snapshot>` and stage changes in a cloned candidate.
- BCF and ZIP named collections reject missing, duplicate, conflicting, and colliding identities before mutation.
- AVI, MP4, PNG, GIF 87a/89a, BMP, CSV, TSV, TXT, EPW, HTML, XML, SVG, Markdown, TIFF, and binary index collections reject invalid base/final indexes, repeated targets, and remove/modify conflicts before candidate mutation.
- JSON recursively validates array/object target identity and node-kind compatibility; JPG validates frame components, quantization/Huffman table identities, and retained segment indexes; TIFF validates tag value-kind consistency.
- Flat scalar/opaque replacement revisions (DEFLATE, MP3, WAV, PDF 1.4, and DWG AC1024) return successful staged candidates without an implicit no-op/fallback path.
- Every residual mutation consumer now commits only `Ok(next)` and converts typed rejection to a diagnostic-preserving `MutationOutcome::error`; no fallible result is silently assigned or discarded.
- HTML/XML/Markdown/SVG mutation-law and diff-level test consumers explicitly unwrap valid results. Binary includes an adversarial invalid-splice test asserting rejection and unchanged base state.

## Static verification

- `rustfmt --edition 2021` completed on the residual artifact Rust leaves; a second `rustfmt --edition 2021 --check` passed for 135 scoped residual leaves selected by the artifact diff/mutation census.
- `git diff --check` passed.
- Scoped `rg` census reports 25 requested residual `MutationDiff` implementations (26 when the already-migrated PDF 1.7 implementation is included) and zero direct production assignments of `MutationDiff::apply` results.
- Cargo/Nx and runtime tests remain pending the serialized integration lane.

## Handoff

Source is frozen for the residual lane pending the parent’s serialized compiler/test gate. Any compiler diagnostics should be handled by the owning artifact shard without reverting typed rejection propagation or preflight/atomicity rules.
