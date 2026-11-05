# Export Metadata Red Preparation

2026-11-05. Scoped test authoring; production manifest unchanged. No Native/server command.

## Added Inputs And Regression

- Editor /🧪️tests/🔬️unit/fixtures/export-metadata.schema.json: strict JSON Schema draft07, required version/actions and all four action fields, no extra fields, exact exportMesh shell/destructivefalse/approvalnever expectation.
- Adjacent export-metadata.json: language-neutral expected classification.
- External editor /🧪️tests/🔬️unit/🦀️.rs: export_mesh_manifest_preserves_non_destructive_shell_policy calls public create_lowpoly_app, finds actual app/window action descriptors, deserializes expected kind and approval into actual semio_framework enums and checks effects.destructive. It does not use private retained tests or duplicate production builder logic.

## Independent Projection Evidence

Retained ticket input export-metadata-oracle.ts uses independent Ajv strict/allErrors, validates expected fixture against strict schema, recursively extracts exportMesh metadata from existing statically emitted hub Lowpoly component JSON and validates observed classification against same schema.

Command: repo tools exec --light bun -- ./<ticket>/export-metadata-oracle.ts

Actual outcome: expected fixture valid; static descriptor does not match. Observed kind shell, destructive true, approval whenDestructive. Exactly two validation failures: destructive must false; approval must never. Oracle exited0 because it reports comparison evidence, not a genuine Rust test pass/fail.

This establishes current emitted-descriptor/source mismatch only. Genuine Rust red remains outstanding and must precede production correction. Root owns native run and manifest edit. New Rust test is expected to fail current destructive assertion on assembled public definition. After genuine red, remove only erroneous exportMesh destructive marker, rerun relevant public test and publication checks as appropriate.

## Paths And Limits

All new paths remain beneath existing editor test owner/ticket. No AGENTS changes, external dependencies, source migration or production metadata edits. Existing installed Ajv used only as test oracle. Static descriptor was read, not rewritten.

## Actionable Independent Red

Oracle corrected to set process.exitCode=1 when observed schema is invalid. Re-executed same registered light Bun command: exit1, expected fixture valid, observed descriptor invalid at exactly destructive and approval. Production remains untouched. This is genuine independent fixture/projection red; it is not Rust assembled-manifest red.

## Independent Green After Root Correction

Root reports removal of erroneous exportMesh destructive marker, sole emitted action changed to destructivefalse/approvalnever, owning binary reencoded and exact roundtrip checked. Same actionable Ajv oracle then exited0. These are production/source and independent emitted projection red-green results. Public Rust assembled-manifest regression remains unexecuted pending native suite; no Rust pass claimed.
