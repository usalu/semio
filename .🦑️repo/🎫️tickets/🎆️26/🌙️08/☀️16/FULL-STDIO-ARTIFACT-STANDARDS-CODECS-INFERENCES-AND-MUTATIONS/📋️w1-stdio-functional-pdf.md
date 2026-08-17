# W1 Stdio Functional PDF 1.7 Shard

Date: 2026-08-16  
Scope: PDF 1.7 grammar, fixture, mutation/diff, native transport, and derived-profile composition.  
Execution boundary: source-first only; no Cargo or Nx was run because the serialized fallible `MutationDiff` runtime lane remains owned by `/root`.

## Baseline evidence

The existing `target/debug/deps/semio_s_plugin_stdio-831e6638b22201e3` test binary reported 14 PDF 1.7 failures before these source edits. Its exact names were:

- `artifacts::pdf::standards::v1_7::subsets::a::io::derived_composition::tests::encrypted_trailer_document_is_rejected_upstream_by_the_shared_engine`
- `artifacts::pdf::standards::v1_7::subsets::a::io::derived_composition::tests::javascript_action_reachable_from_open_action_fails_compose_with_real_diagnostic`
- `artifacts::pdf::standards::v1_7::subsets::a::io::derived_composition::tests::launch_action_reachable_from_open_action_fails_compose_with_real_diagnostic`
- `artifacts::pdf::standards::v1_7::subsets::any::io::component::tests::bachelor_thesis_logical_lifecycle_preserves_original_native_bytes`
- `artifacts::pdf::standards::v1_7::subsets::any::io::component::tests::conformance_laws::diff_grammar_conformance_law`
- `artifacts::pdf::standards::v1_7::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law`
- `artifacts::pdf::standards::v1_7::subsets::any::io::component::tests::conformance_laws::ops_grammar_conformance_law`
- `artifacts::pdf::standards::v1_7::subsets::any::schema::diff::component::tests::absorb_law_pages_associativity`
- `artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::component::tests::mutation_apply_inverse_round_trips_every_variant`
- `artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::component::tests::mutation_diff_law_matches_apply_pdf_mutation`
- `artifacts::pdf::standards::v1_7::subsets::ua::io::derived_composition::tests::conforming_builder_snapshot_composes_and_stamps_ua`
- `artifacts::pdf::standards::v1_7::subsets::vt::io::derived_composition::tests::conforming_builder_snapshot_composes_and_stamps_vt`
- `artifacts::pdf::standards::v1_7::subsets::x::io::derived_composition::tests::conforming_builder_snapshot_composes_and_stamps_x`
- `artifacts::pdf::standards::v1_7::subsets::x::io::derived_composition::tests::subset_validator_recheck_runs_the_same_check`

## P0 findings

None found in this bounded source pass.

## P1 findings and source repairs

### Native fixture transport mismatch

The committed PDF DSL fixture is a Semio text envelope containing lowercase hex for a native `%PDF-1.7` document, and the pack fixture contains the same native PDF bytes inside its Semio binary envelope. The snapshot leaves had instead decoded and encoded the private snapshot-binary protocol. That caused invalid UTF-8/truncated-object errors in fixture and derived-profile tests.

`PdfSnapshot` DSL and pack codecs now use the authoritative native codec boundary: hex DSL payloads decode through `decode_pdf`, DSL output hex-encodes `encode_pdf`, pack payloads encode native `encode_pdf` bytes, and pack decode calls `decode_pdf`. The SetSnapshot mutation wire protocol remains on the private structured snapshot codec; it is not conflated with artifact transport.

This repair also makes the any composer feed native fixture text to the real analyzer and lets PDF/A, PDF/X, PDF/E, PDF/UA, PDF/VT, and PDF/H composition paths consume the same native/pack boundary.

### Sparse diff state loss

`PdfDiff::apply` cleared `objects` and `trailer` whenever a pages diff existed without corresponding object/trailer diffs. That violated sparse-diff semantics and caused page absorb associativity, mutation inverse, and mutation-vs-diff equality failures. The unrelated-field clearing branch was removed; applying a pages diff now retains unaffected authoritative state.

### Grammar drift

The mutation grammar described `SetSnapshot` as a structured bracket literal even though the real op encoder emits one lowercase-hex binary snapshot payload. `snapshot-lit` now matches the actual hex transport.

The diff grammar described stream filters as an optional hex value, while `enc_pdf_object` and stream value diffs emit the typed filter-list form (`[]`, `F[0]`, `F[1,...]`, `H`, `A`, `L`). The grammar now contains the real `stream-filters` production and uses it for both stream objects and stream filter diffs.

## P2 remaining gate

The arbitrary bachelor-thesis byte-identity test remains pending post-lock verification:

`artifacts::pdf::standards::v1_7::subsets::any::io::component::tests::bachelor_thesis_logical_lifecycle_preserves_original_native_bytes`

The current logical `PdfSnapshot` intentionally has no native-byte shadow field. `decode_pdf` stores decoded COS objects and decoded stream data; `encode_pdf` deterministically writes that logical model. Exact untouched-source byte identity for an arbitrary external PDF therefore requires the broader anchored source/opaque-record design, not a compatibility field or a global cache. No such shadow state was introduced in this shard. The fixture/demo native round-trip path is deterministic and source-backed through the native codec, but the large external fixture still needs that model-level source retention decision.

## Files changed

- `.../pdf/.../✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `.../pdf/.../✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `.../pdf/.../✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
- `.../pdf/.../✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`

## Static validation

Passed without Cargo/Nx:

```text
rustfmt --edition 2021 --check [changed PDF Rust leaves]
git diff --check -- [changed PDF Rust leaves]
```

No post-edit test result is claimed in this source-only pass. The exact pending names above must be rerun on the clean integration target after `/root` releases the serialized runtime lock.
