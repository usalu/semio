# XLSX paged natural decoder

## Design
XLSX Open File / drop now routes inputs above `max_work_units_per_step` (4096) through a controlled, resumable decoder
(`NaturalXlsxDecoder`, base `io/import/controlled`), wired into base, strict and transitional editors
(`PAGED_NATURAL_SOURCE_MAXIMUM_BYTES` = 16 MiB, `admit_natural_file_decode_owner`, `admit_paged_natural_file_decode_owner`,
`build_natural_file_decode_owner`). Inputs within one step keep the synchronous owner. Strict/transitional pass their conformance
check as a gate (`io::natural_conformance`, also reused by their whole-buffer `decode_xlsx`).

Phases, each bounded by the remaining work allowance, checking `is_cancelled`/`should_yield`, reporting monotone progress:
gather paged source -> open zip directory -> plan members (headers only) -> per member: resumable inflate (framework `Inflater`,
one transition = one unit) with incremental CRC-32 -> seal -> resumable XML parse (new `XmlDocumentCursor`, token-granular on an
explicit stack) -> assemble + `validate_authority` + gate. `[Content_Types].xml` is processed first, as the whole decoder does.
Close retires every owned buffer/tree through the framework retirement owners and the paged source through its own close steps.

Shared-code refactors (single source of truth, whole-buffer paths now use them): zip `open_zip_directory`, `plan_zip_entry`,
`ZipEntryPlan::finish`, `close_zip_directory`, `Crc32`, `validate_zip_comment`, `validate_zip_entry_headers`; opc `check_opc_member_names`,
`opc_content_types_from_document`, `opc_relationships_from_document`, `opc_part_content_type`; xml `XmlDocumentCursor`.

## Known limits (honest)
- UTF-8 validation of one XML part and `validate_authority` are one step each (memory-speed, O(part) / O(parts)).
- Allocation beyond the decoder box is not itemised against the retained grant; `owned_output_bytes` counts archive + raw/XML text bytes
  (the 32 MiB natural contract ceiling therefore bounds the accepted workbook size).
- Publication still goes through `document_reference_load_emit`: `XlsxSnapshot` has no `native_pack_source`, so the final pack encode is one step
  (PDF has a chunked exporter). Follow-up: chunked pack export for XlsxSnapshot.
- Inflater uses the cold history (output kept twice per member while inflating).

## Fixture and tests
- Generator: `oracles/third-party-generators/py` gained `generate-large-grid` / verify (openpyxl 3.1.5, run via `repo tools exec uv`);
  fixture `base/tests/fixtures/large-grid/{package.xlsx (56,652 B, 9,360 cells, 3 sheets), json witness}`.
- Base tests `io/tests/controlled-import`: paged and contiguous decode == whole-buffer snapshot; >100 turns, each <= 4096 units; calamine
  grid of original == calamine of re-encoded decoded snapshot, plus openpyxl probes/shape; small-input owner selection; cancellation then
  clean retirement; corrupt/truncated refusal; refusing gate; XML cursor == whole parse at budgets 1..max, same refusals, element count
  == quick-xml on a real sheet.
- Strict/transitional editor tests: paged route equals `decode_natural_file` (transitional) / is refused by the strict gate.

## Results
- `cargo check --tests` base: pass. strict + transitional: pass.
- Test run: PENDING, launched by the coordinator via `claude-paged-decoder-test.sh` (see logs in generated/claude-paged-decoder).

## Source review (progress, cancellation, messages)
- Progress: `completed_work_units` strictly grows on every `Progress` turn (the framework's stall check requires it); `consumed_input_bytes` is monotone
  (gathered bytes, then the full archive once the directory is open); `decoded_items` counts finished members and is capped by `max_decoded_items`
  at directory open; `owned_output_bytes` is archive plus retained part bytes. Every per-phase work return is clamped to the allowance, so a turn never
  spends more than `min(max_work_units_per_step, fuel_remaining)`.
- Cancellation: the advance loop stops at `is_cancelled()` and returns `Yield` when nothing was spent; the job layer also short-circuits cancelled
  jobs before calling the decoder. State after a stop is closable at any phase (`begin_close` moves the half-built XML tree and text into retirable owners).
- Messages: decoder refusals are plain English `MediaError::Payload` strings, the same convention as the PDF decoder; this layer has no localization
  hook, and the framework wraps them into faults. German/English localization of these diagnostics therefore remains a framework-level follow-up.
- No source change was needed from the review.

## Namespace-scope gate fix (importer bug, not a test bug)
The Transitional/Strict gates required `xmlns:r` on the workbook root. openpyxl (valid Transitional, opened by Excel, LibreOffice and calamine) declares it on each
`<sheet>`, so every such workbook was refused on the whole-buffer and the paged route alike. Root fix: `workbook_namespaces` (base vocabulary) resolves the root element
name and every prefixed `*:id` attribute through the in-scope bindings at the element that uses them; the gates refuse only a genuinely wrong namespace URI or an
undeclared prefix. Applied to: transitional and strict `check_*_conformance`, the base sqlite subset validator (Rust) and its TypeScript twin.
`sniff_xlsx_subset` no longer uses a 4096-byte window: it reads the whole root start tag (`root_start_tag`). The workbook model inference already resolved `r:id` through scope.
Tests: vocabulary fixture `workbooks` (root, child-only, alias prefixes, prefixed root, wrong namespace on a child, two namespaces, none, undeclared), transitional paged
route opens marketplaces and the openpyxl large grid (== whole-buffer decode), refuses a Strict root namespace; strict refuses the Transitional grid.
Not run yet: TypeScript twin type-check.
