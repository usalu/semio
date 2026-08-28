# Indexed JSON Document Ownership

## Next Concrete Prepared-Host Boundary

The existing JSON cursor remains the bytewise grammar authority. A sibling `json/🧾️value` document cursor will retain each validated token in the existing numeric index, one token edit at a time, and publish only after the complete JSON grammar succeeds and its parser owner has retired. Provisional tokens cannot mint a document. This does not change the released lexical parser or generic-pack module while the coordinator's coherent test window runs.

The document owns one captured immutable prepared scene, the exact field id, and the flat token index. Issued document captures and token readers share this one root without ancestor chains. A bounded span reader is created from that exact field through the existing `beginTextBytes(field, offset, length)` API; it independently retains the native scene pages. Closing the document cannot invalidate an already issued span reader. No arbitrary supplied JSON object, structural capture callback, source mutation, or raw-pointer projection is admitted.

Source-present API: `OwnedUiSceneJsonDocumentCursor(source, field)` with `advance`, `takeResult`, `beginClose`, `closeStep`, and `terminalIsEmpty`; successful `OwnedUiSceneJsonDocument` with `capture`, `beginRead`, `beginLookup(ordinal)`, `beginSpan(offset, length)`, and `beginClose`. Document and reader owners are privately minted and frozen; token records are immutable outputs of the lexical authority. Each child result preserves refusal/fault and actual grant accounting. A failed index update retains the pending token and previous root until explicit close.

## Lifecycle and Test Plan

The neutral fixture covers original duplicate keys, escaped lone surrogates, empty containers, long Unicode keys/strings, nested maps, missing lookup, two simultaneous readers, a span reader surviving document closure, zero grants, malformed trailing input, cancellation at every observed phase, and reflected-root rejection. JSON.parse and Node Buffer remain independent value and byte oracles. Token strings and numbers remain raw source spans at this boundary; tests may decode spans with the oracle, but production must not use that whole-string conversion.

The finite test bound derives from source bytes, at most one token per source byte, and the existing numeric-index AVL edit/reference/retirement transition bound. Each advance asserts at most one lexical byte, one logical item and 4,096 accounted bytes. This is an algorithmic work bound, not an eight-millisecond timing certificate or global-memory admission.

## Implementation Checkpoint

The module keeps parser, pending token, index edit, previous-root retirement and final source ownership as separate phases. It performs no whole token-array creation. Parsing cannot advance while a token edit or its old-root cleanup is pending. Successful parsing explicitly closes the parser before the document mint; cancellation drains parser frames and byte chunks before index/source owners. The source position is retained separately after the parser closes.

The neutral test was mounted before implementation. R1 reached the intended missing-owner constructor after real typed/native scene preparation: one failed, 628 skipped, 629 total, 10.60 seconds. R2 passed one, 628 skipped, 629 total, 76.30 seconds total and 72.86 seconds test execution, exit zero. Strict R1 contains exactly seven existing tutorial diagnostics, no new document/parser/fixture errors. Targeted `git diff --check` passed. The test covers six valid documents including 24-KiB Unicode source, four invalid documents, original duplicate key order, two reader aliases, missing lookup, an independently held span reader, depth 4,096, fixed and observed phase-prefix cancellation, zero grants and forged constructors/capture callbacks.

Canonical selector: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedSceneJsonDocument'`. The deep token/index test is materially slower than lexical scanning; it ran on the unchanged long-test tier rather than widening a runtime or test budget. The full stdout/stderr is retained in `🧪️renderer-owned-scene-json-document-r{1,2}-2026-08-27.txt`; strict output is `🧪️renderer-owned-scene-json-document-strict-r1-2026-08-27.txt`. Nx's history-based flaky-task notice reflects the deliberate RED/GREEN target history; no retry or passing-result substitution was performed.

## Terminal Child and Wrapper Budget

Coordinator review found that a valid 4,096-byte terminal child step was followed by 32 bytes of wrapper unlinking in the same call. The old code then reported rejection only after unlinking the owner. Terminal R1's first test interception accidentally reached the earlier lexical reader's numeric child, producing a 34-versus-32 assertion; it did not isolate this defect. R2 explicitly waits for the document's token-install phase before intercepting the real queued numeric retirement's terminal result. It then fails on rejected versus pending, one failed/629 skipped/630 total, 14.37 seconds.

The queue now retains its terminal child handle through the completed child step. A separate admitted 32-byte transition unlinks the wrapper; a claimed complete child must also have an empty terminal owner. Terminal R3 passes one/629 skipped/630 total, 120.28 seconds total/116.03 seconds tests, including all original deep and lifecycle laws. The per-record transition bound already includes fixed wrapper phases; no budget or test deadline was enlarged. Full output remains in `renderer-owned-scene-json-document-terminal-r{1,2,3}`.

## Still Not a Live Host Projection

The next wrapper must retain outer typed fields plus nested JSON/pack documents as a directed ownership graph, never an outer owner storing children that themselves retain that same owner. Strings require bytewise escape/UTF-8 to bounded UTF-16 chunks, including escaped lone surrogates. Numeric spans require a correctly rounded bounded conversion design before actual number-valued host fields can use them; whole-lexeme Number/JSON.parse is not a retained implementation. None of these remaining conversions are replaced by placeholders or a compatibility object tree.
