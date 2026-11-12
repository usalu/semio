# Lane ackquote result
## Step 1 (11/12)
- Root cause of the 3 job-receiving failures was NOT the acknowledgement quote: `OriginalJobConstructionRecipient::source_turn` returned None while the Work is Source-bound, so the acknowledgement's `NativeReceivingConstructionBinding` had source None, `matches_effect` (requires Some) failed, `NativeResidentSuccessProducer::receive` replied Err("acknowledgement binding changed") into the metadata frame, and `advance_resident_success` returned `Ready{0}` (child_work Some) without surfacing the buried Err; success stayed resident. Hence the observed zero-work first poll.
- Fix: recipient carries the original Source identity (`with_source`, `source_turn` returns it; set in `OriginalDecodeState::run_on_caller`). W-ackquote-t6: job-receiving --lib 9 passed rc0.
## Step 2: clamp (11/12)
- Producer of the two extra construction receipts: `advance_metadata` (record/schema/drive.rs) posts three receipts (key metadata action, completion Post, release) while its step progress is clamped by `total.copied_items.min(1)`; items 3 vs 1, bytes equal. Removing that `min(1)` together with `cover_progress` makes the three tests fail again with effects 0/0 (not understood: needs the metadata step quoted with 3 items/3 work, currently quoted 1 unit via `nested_work_demand`). Probes were heisenbug-prone (perturb the +64 B heap assert). Both restored; `cover_progress` clamp remains (debt).
## Verified
- W-ackquote-t15 job-receiving --lib: 9 passed rc0 (after probes removed, final tree).
- W-ackquote-t14 native-owned `--lib --tests`: rc0, 18 "test result: ok".
- W-ackquote-w2 wasm check (hub manifest) of semio-framework-job-receiving + semio-framework-value-native-owned: rc0.
- fem 3d `original_fem_receiving` (W-ackquote-t13/t16 manifest `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml`, `-p semio-s-artifact-fem-3d-model --test original_fem_receiving`): 2 passed / 2 failed, "FEM receiving child cannot stay pending on a synchronous recipient" (a child Pending); giving the fem OriginalStepRecipient the Source did not change it; fem edits reverted. Not fixed.
## Debts
- `advance_resident_success` returns Ready{work} with a buried Err in the metadata frame when child_work is Some; should surface the metadata result.
- Phase-split quote for ack was not needed for the 3 tests (predicate `acknowledgement_started` unchanged); metadata-settle zero-quote law not added.
