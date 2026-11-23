# PDF guest edit refusal: `original-turn.native-return-refused: semantic`

## Where the detail was lost
- WIT `actor-call-failure` is `authority(actor-authority-refusal) | semantic` (`🔌️plugin/🧬️schema/📜️reactor.wit`): `semantic` has no payload by design ("semantic cause remains owned by its guest").
- Guest producer: `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` `poll_wit` (and `wire_job_failure`) turn the retained `OriginalTurnFailure`/`Fault` into the bare `Semantic` tag; the cause stays in the guest entry (`original_job_refusal`, `original_cleanup_refusal`, `original_fault`).
- Host bridge: `🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts` `nativeRefusalError` built `original-turn.native-return-refused: <tag>` only.

## Fix (observability, no WIT change)
1. `OriginalTurnFailure::diagnostic()` (job `⚠️failure`) renders `code: message` (+ Fault params as ` name=value`). The code is the Fault code, the `ValueRefusalKind` or the domain code; the message is the Fault text. The bilingual notice is resolved by the shell's fault catalog from code and params, so the guest carries the stable code and the English message.
2. Guest `report_semantic()` logs `original-turn.semantic <diagnostic>` through the existing `pure.log("error", ...)` import whenever the refusal tag is emitted.
3. Host shim `log` keeps the last such line (`__takeActorFault`), the bridge appends it: `original-turn.native-return-refused: semantic: <code>: <message>`; it is kept for the held-refusal replays of the next calls. The console also shows `[actor] original-turn.semantic ...`.

## Tests
- `component-codec-reply` bridge test (run through `🎭️actor-transport/🟦️.ts`, 182 passed): fixture `🚫️native-refusal.json` now has `expectedDetail` and `expectedFullMessage`; the stand-in component logs the detail and both polls must reject with the full message.
- Native `original_turn_failure_diagnostic_carries_code_message_and_params` (job-worker `original-normal-failure`): passed.

## Native reproduction
`blank_document_first_edits_settle_through_the_registered_typed_operation` (editor unit tests): new registered app, `set-info-field` title, `insert-text`, each through `dispatch_fixture_action` plus `settle_registered_typed_operation`.
See "Status" below for the run result.

## Status (2026-11-23)
- Observability fix and its two tests: done, passing.
- Native reproduction test: written and compiled up to the editor unit-test crate (its own compile errors fixed); the last runs were blocked by other agents' in-flight edits in the shared tree: derive `NativeReceive` on generic `PdfIndexedDiff<T>` ("explicit concrete owner declaration"), `PdfRoleItem` vs `PdfIndexedItem` mismatch in `🚪️io/💾️binary/🧬️mutations`, then `semio-s-artifact-stdio-contract-testing` (`finish_decode`, `semio_framework_test_original_snapshot_owners`). The semantic fault text is therefore not yet captured. Rerun: `NATIVE_CARGO_DIR` unset, `sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib blank_document_first_edits`.
- Known candidate cause from `🪶️pdf-sqlite-and-natural-route.md`: "captured shared original changed its Work or destination" raised in `🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs:8` when the close effect comes from another Work binding than the capture (same `settle_registered_typed_operation` step); not fixed here since that contract is being changed by another agent.
- Wasm rebuild and journey rerun not done (tree does not compile); after the native test is green: `sh rebuild-pdf-wasm.sh`, `sh serve-pdf-native.sh`, then `repo pdf-pane-journey.ts en de`; the console now shows `[actor] original-turn.semantic <code>: <message>` and the thrown error carries it.
- Journey: German run now checks `Seitenbreite`/`Seitenhöhe` (`labels.<lang>.pageSize`).
