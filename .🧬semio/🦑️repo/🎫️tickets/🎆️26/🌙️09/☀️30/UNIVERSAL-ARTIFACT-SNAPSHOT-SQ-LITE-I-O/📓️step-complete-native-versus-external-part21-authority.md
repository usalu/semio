# STEP Complete Native Versus External Part21 Authority

Read-only current source audit, 2026-10-03. No Cargo, source edits or prediction of the running twelve-law result.

Owner base: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base`.

## Actual Loss Origin And Clean Authority

Snapshot `🧬️schema/📸️snapshot/🦀️.rs` ordinary ArtifactDsl parse/print and ArtifactPack encode/decode currently round-trip through `Part21Document`. `from_part21_document` reconstructs schema as the declared constant rather than preserving an arbitrary owned schema field. External Part21 represents exchange graph syntax; its grammar cannot faithfully carry every IEEE word and the owned schema. The adjacent typed native Frame retains schema, every header/list field, entity/complex argument lists, the full nine StepValue variants, u64 reference words and f64 values. Its controlled encode/decode60/61 use the actual Frame descriptor producer and shared paid record helpers.

The clean cutover is one complete typed Frame authority for ordinary and controlled Snapshot text/binary Native codecs, using the same descriptor, projection, binding and reconstruction. Ordinary facets should call this actual owner implementation with their appropriate ordinary limits/control; controlled facets retain paid checkpoints and cumulative ownership. No raw Part21 auto-detection fallback belongs in ordinary Native parse. Keep explicit `from_part21_document`, `to_part21_document`, tokenizer and writer as external exchange conversion authority. Handcraft authored Native descriptor/protocol/example assets to describe the actual typed Frame rather than claim Part21 for complete owned Native storage. Preserve Pack option/file limits at the ordinary terminal rather than silently ignoring them.

## Exact Envelope Identity

`STDIO_STEP_DOCUMENT_SCHEMA` and `ArtifactDsl::envelope_id()` both currently equal `stdio.step`. Shared `encode_sqlite_snapshot_record_native` explicitly takes `envelope_id`; it computes declared prefix and wraps component Pack/Dsl v1. Its decoder checks the same identity/component/version before binding. The owned Frame schema is independent content and may be empty or contain literal text; it must not determine the envelope identity. There is no present mismatch between the draft's constant argument and the current envelope_id value. Using the actual trait identity makes the relationship explicit without deriving authority from mutable owned schema text.

## Required External Consumer And Fixture Cutover

- Base external TXT importer `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs:9` already uses explicit `parse_part21` and `from_part21_document`.
- Base external Binary importer `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs:9` currently calls ordinary Snapshot `parse_dsl` on external UTF-8 Part21; change its authored conversion to the explicit external tokenizer, preserving its typed terminal mapping.
- External TXT and Binary exporters explicitly call `write_part21(&from.to_part21_document())`, at their corresponding export serializer lines8/14. They remain external exchange conversion sites.
- Snapshot unit file `🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` has raw Part21 fixture construction at7,22,38,49,59. Construct those external syntax witnesses with the explicit tokenizer/conversion, while preserving typed/complex/full Native equality and canonical repeated Native encode checks. The test at7 currently mixes external parsing and Native roundtrip; make each authority explicit instead of retaining dual grammar acceptance.
- CC1–CC6 editors initialize `PRIMARY_TEXT` through ordinary Snapshot parse with default fallback (for example CC3 editor92). If PRIMARY_TEXT remains an external Part21 example, initialize through explicit external conversion; alternatively author a complete Native example asset and keep ordinary parse. Do not allow default-empty fallback to mask the cutover.
- CC subset composers/validators and mutations use explicit typed snapshot↔Part21 document views. Those conversions remain genuine external/domain graph operations; their unrelated argument/graph checks need no ordinary Native parser compatibility. CAD base I/O264 also explicitly constructs StepSnapshot from a Part21Document.

## Preserved Gates

Registered Source route is `@semio-tech/stdio-step-rs:test-snapshot-sqlite-source`; registered no-argument Native route is `@semio-tech/stdio-step-rs:test-snapshot-sqlite-native`, with canonical package script and120000ms SQLite budget. Parent reports Source5/57 ownership contract GREEN; that is independent of the current running Native attempt. Existing twelve-law cohort requires controlled bytes equal ordinary codecs and full literal schema/all IEEE words. Preserve those demands, actual interior cancellation, exact reference relationships, CC diagnostics, independent SQLite/DataView and semantic row ceilings. No revised ordinary facet is mounted by this audit and no fresh Native outcome is claimed.
