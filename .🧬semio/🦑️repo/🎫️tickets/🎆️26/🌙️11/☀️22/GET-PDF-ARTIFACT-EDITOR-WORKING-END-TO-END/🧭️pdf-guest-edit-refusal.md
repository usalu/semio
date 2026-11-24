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

## Update after reboot and coordinator follow-up
- Native reproduction `blank_document_first_edits_settle_through_the_registered_typed_operation` (including `close_registered_fixture_app`): PASSES (1 passed, 1.75 s) on the current framework tree. History of the same test on moving trees: earlier run failed at the insert-text settlement with `plugin.internal: captured shared original changed its Work or destination` (the arc-custody close path, `PluginLifecycleFailure`, fault code `plugin.internal`, effects all 0); a later run failed only at app teardown (`original Store slot requires payload handoff or paid physical shell closure`) until the test closed the app. The literal text no longer exists in the source, so the contract was changed by the owning agent in between. Conclusion: the browser fault came from a stale wasm built before that fix; the root cause lives in the arc-custody/ownership work of `GET-FEM-PLUGIN-WORKING-END-TO-END`, no further code change by this ticket.
- `rebuild-pdf-wasm.sh` fixed for the post-reboot toolchain (falls back to `~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin`, wasm32-wasip2 std is installed there); `serve-pdf-native.sh` already used the new bun path.
- Blocked after the reboot: the staged browser output `.cache/b/p/m` is empty (no `modules/📘️stdio-pdf`, no vendor shim), and `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype` fails with "Exact empty inactive Native root required: .cache/b/g/m inference" (service reference b7b76b62). `stage-pdf-component.ts` can only swap files into an existing staged module, so the wasm rebuild, host restart on 6260 and the en/de journey need the repo build state recovered first (`repo reset`/`repo doctor`, owner of the native build store). Another agent already runs `cargo build ... semio-hub-stdio-pdf-component --target wasm32-wasip2 --profile prototype` (pid 16103); its output lands in the shared `native-target` lanes and `rebuild-pdf-wasm.sh` will pick it up.
- Natural-route leftovers: the rotation test rerun was queued behind both native lanes and did not finish; large-document close-owner-at-`Begin` and the paged large-image probe were not started.

## Repo recovery after an unclean shutdown (fundamental fix)
- Check: `inspectPersistentReceiver` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`. For a Native receiver whose image is not attached it demanded an exactly empty mount directory and threw `Exact empty inactive Native root required: <root> <entry>` for any entry.
- Why a reboot blocked everything: while the image is detached the mount directory sits on the underlying volume; a process that wrote there (here `.cache/b/g/m/inference/selections/...`, written before the reboot) leaves content that the image mount would shadow. The check failed closed forever and no path removed the content, so every `repo run` died in the resident service.
- Fix: for an inactive root on the underlying device, the stray entries are now moved (never deleted) by `quarantineStrandedNativeRootEntriesV1` (`🐚️shell/🟦️.ts`) into a sibling `<root>-stranded-<stamp>`, the root must then be empty, and the move is logged on stderr. A root that is itself another mount device still throws. Regression test in `🐚️shell/🧪️tests/🟦️.ts` (4 passing): nested content is moved intact, root ends empty, an empty root is a no-op.
- Result: `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype` passes that check and proceeds to the graph read (about 30 minutes cold after the reboot).

## Staging fallback status
`rebuild-pdf-wasm.sh` (`-j 1`, 80 crates compiled so far, log `/tmp/claude-501/wasm.txt`) runs detached; it stages automatically when cargo finishes (`stage-pdf-component.ts` now regenerates bridge and host shim from source and needs the products image attached: `sh mount-products.sh`, already attached). Next: `sh serve-pdf-native.sh` (6260), `repo ./pdf-pane-journey.ts` en/de, then natural-route leftovers. See `🌐️pdf-browser-journey.md` for the staging and graph-read notes.

## Final state of this round
- Fresh wasm (with the semantic-fault logging) is staged and served: first edits of a blank document pass in the browser. See the journey table in `🌐️pdf-browser-journey.md`.
- Rotation natural-route test: import (scope list contains `Full`) and rotation settle pass; the Save submission (`submit_media_export_checked`) returns not-ready on all 32768 turns while the command pipeline is idle. The not-ready comes before `submit_owned_media_export` (my state dump in that function never printed), i.e. from the first gate of `submit_media_export_checked` (`receive_media_submission_refusal` / `cx.should_yield()`) in `🧵️instance/🔌️plugin-app-impl/🎞️media-transfer/🦀️.rs`. Temporary `[DEBUG]` lines were removed again.
- Large documents (`close owner Begin`) and the browser Open File stall share the decoder work quote of 1 MiB + 1 that exceeds the turn fuel (see journey report).
