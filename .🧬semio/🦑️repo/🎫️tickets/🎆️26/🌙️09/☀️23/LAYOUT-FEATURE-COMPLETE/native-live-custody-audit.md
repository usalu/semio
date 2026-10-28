# Native Live Drawing And Custody Audit

Static source audit on 2026-10-21. No Rust compilation or native execution was performed: the required repo runner is unavailable. No source files were edited.

## Actionable Findings

1. **P1: Paragraph native tests still call removed unfunded APIs.** Relative to the canonical editor root, `⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🦀️.rs:6`, `:9`, and `:13` call `LayoutParagraphState.advance_one`; `:8` calls `advance_external` three times. `⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🧵️flow/🦀️.rs:9` calls `advance_flow`, and `:10` calls `advance_one`. The owner now defines only `advance_one_funded`, `advance_external_funded`, and `advance_flow_funded`. These tests cannot type-check until their callers supply original retained grants and account receipts. Existing assertions must remain, including source-identity refusal probes.

## Reviewed Without An Additional Concrete Defect

- Native live phases 11–17 preserve the initial snapshot address, use the retained preview consistently, resume frame publication after placed Drawing completion, and keep document Content versus Page Canvas placement separate. Canvas publication uses bounded inline fragments and transfers image proxy String ownership into one Chunk.
- Shared composition path, glyph and image event matches agree with public native owner signatures. The canonical public module paths agree with the live and export imports examined.
- Export `ExportTextState.advance_one(snapshot, grant)` production consumer supplies the grant, consumes the last receipt before propagating the result, and handles `Option<PdfItem>`. The native export paragraph fixture uses the current `plan_one()` original-caller wrapper; there is no stale direct ExportTextState caller in the scanned native Layout artifact sources.
- Fidelity production and test callers examined supply the new FontWorkLedger argument. Native FontFaceAdmission and GlyphOutline advances examined supply work.
- Manual export close quotes actual retained capacities for byte/String/vector backing; nested image backing precedes row removal; paired composition/paragraph/fidelity owners retain their controlled close dispatch. No additional concrete close-receipt defect was established.

## Boundaries

Known cold preparation, output/planning admission, cached TypedJSON Drawing routes, and minimum source-item demand limitations remain documented in the sibling reports and were not counted as new findings. Static syntax is not evidence of Rust type checking. The scan covered native files under the canonical Layout artifact subset; broader framework migration correctness was outside this audit.

## Follow-Up

Core has repaired the reported native paragraph fixture callers with explicit funded grants, including external/flow variants. Runtime native verification remains unavailable. Root fresh checks find no syntax errors or missing fixture includes across eighty-four scoped Rust files; this does not establish type correctness.
