# 🪟 Selected Local Document Surface

LocalDocument optionally records the selected surface as LocalDocumentSurface {pluginId, appId}. The schema and document identity remain independent of presentation; the same stdio.gif document can select87a or89a. A kept archive may have no selected surface. Explicit malformed/null coordinates are refused.

The optional field is declared in the existing admit-local-document JSON schema for both the record and admission payload; ids contain1to256codepoints and extra coordinate fields are refused. TS and authored Rust twins export the public LocalDocumentSurface type through config/mutations. Admission, equality, sparse diffs, inverse admission and catalog decode preserve the field. Rust serde and native Value conversion share the same bounds and presence semantics. No external dependency or compatibility layer was added.

## Verification

Neutral vectors live at 🧰️framework/🛍️products/💻️os/🎚️config/🧪️tests/🧫️fixtures/🪟️surface/🔣️.json. They specify admission, exact subset reselection while retaining schema, an archive with no chosen surface, and retirement with inverse restoring the selected surface. Malformed coordinates are separate cases.

The first TS run before implementation failed5newtests and passed8existingtests (surface-red.log, exit1). The completed implementation passes13tests (surface-green.log, exit0), including actual pack and document-archive roundtrips. AJV independently validates the same neutral records and rejects malformed coordinates. The Rust companion test consumes the same fixture and tests admission/inversion, serde JSON codec and native Value roundtrips/refusals. Rust verification is pending the single shared-Cargo driver, package semio-framework-os-config-tests-1, filter selected_surface_neutral_vectors.

## Files

- framework modules config: schema/mutations/local-catalog-config-mutation TS model, admit-local-document TS leaf, apply-diff TS decoder, mutations TS barrel.
- OS config: admit-local-document JSON schema, Rust leaf, mutations Rust barrel, admit and retire Rust unit test files, neutral surface fixture.
- OS config-diff-twin TS test suite.
- Ticket: vitest.local-surface.config.mjs, sequential verify-repairs.py; generated red/green logs under 🗑️generated.

Host catalog and scope files were left to the parent worker.

Rust first execution compiled and ran the neutral test, then failed missing-surface controlled decoding: custom default had no controlled constructor. Both optional fields now declare a controlled default constructor which charges one control step and yields None; retry queued. Explicit null still enters the strict surface decoder and is refused.

Second Rust execution rejected a custom missing-field constructor charging the enclosing record field stage. The owner hook now begins its own exact stage: one scoped step for missing None, restoring the record stage afterward, or two field checks plus actual plugin/app UTF8 byte lengths for semantic validation. These are explicit producer quotes derived from input lengths, not relaxed ceilings. Final Rust retry remains required.

Final Rust result: selected_surface_neutral_vectors_preserve_subset_through_admission_diff_inverse_and_codecs passed (1 passed,0 failed,104 filtered). The test includes both ordinary and controlled malformed-surface refusal, exact valid surface roundtrips, absent-surface decoding, mutation application and inverse. Log: generated/[DEBUG]repair-local-surface-final.log. Combined TS13pass and Rust1pass now verify the optional catalog coordinate in both implementations.
