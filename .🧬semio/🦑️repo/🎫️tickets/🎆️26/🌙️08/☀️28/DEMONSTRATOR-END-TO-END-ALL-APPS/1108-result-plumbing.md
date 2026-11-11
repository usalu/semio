# 1108 plumbing lane result

## Edited (wgpu partition, host-plumbing / agent-elements)
- browser-worker/output: `presentation_generation` gets `#[value(serialize_with="decimal_to_value")]` (canonical decimal role requires the plain serializer); `OriginalBrowserOutput::advance` runs the wire cursor inside `cx.with_original_effect` and settles through `receive_original_retained_work`; closes via `close_original_effect_step` / `close_original_retained_effect_step`.
- render-snapshot: `RetireOwned` is `RetirementCursor`/`retirement_seed` (delegates to media slots); the sink stores `SharedOriginalStorage<RenderSnapshot>`; output loan = paid `duplicate(effect)`; publish = `SharedOriginalStorage::admit` with its work settled on the turn (value restored on refusal); displaced closes through `close_owner` in `close_original_effect_step`; refused closes through `ControlledRetirement::step`.
- API change: `RenderSnapshotSink::new(seed)` parks the seed in `original_incoming` (first `publish_on_original_caller` publishes it); unpaid `publish()` and `acquire()->Arc` are removed, replaced by `with_current(|&RenderSnapshot|)`. winit-app `present_snapshot` adapted.
- surface-lane: `InteractiveJob::step` 3-arg (`from_callback`), closes return `RetainedWorkOutcome`.
- native-caller: `admit_step`/`close_step` take `StepBudget`.
- native-io source: `admit_original_construction_settled(cx,&mut Default::default(),..)`; `WorkerSubmitError::kind()/into_job()`; invocation type annotation.
- AgentBridge wire: added `write_u32` (agent-elements 1 error).

## Debts
- `engine/🧪️tests/🔬️wgpu-render-snapshot-unit` still uses `acquire()`/`publish()`; browser-worker output tests and media-slots collect tests use the old sink API. Not ported.
- A pending child retirement inside `close_shared` needs an identical grant on the recover turn (route compares grant); cx grants shrink per turn.

## Follow-up edits after W-plumbing-3
- render-snapshot uses the allocation-return API: `SharedOriginalStorage::admit_from(&mut Option<T>, &mut Option<Self>, effect)` inside `cx.with_original_effect` (refusal leaves the incoming snapshot in place) and `duplicate_into`; settled through `receive_original_retained_work`.
- native-caller: `admit_step` result error type is `OriginalTurnFailure`; native-io source flattens it into its retained `ValueError` refusals.
- semio-os-host-space: `import_document_artifact` gained the `protocol::OpBinaryReceiving` bound (6 errors).

## Verification
- W-plumbing-9 native host-plumbing + agent-elements: rc 0 (54 -> 0, 1 -> 0).
- W-plumbing-8 native semio-os-host-space: rc 0 (6 -> 0).
- W-plumbing-10 `--target wasm32-unknown-unknown` host-plumbing + agent-elements: rc 0.
- Shell native W-plumbing-11: rc 1, plumbing crates no longer red. Remaining: semio-plugin-reactor-jobs 18 (`reactor/jobs/infer/mod`, l.256/282/283: `OriginalTurnPrimary: From<OriginalTurnFailure>` and mismatches; shellreds edited-unverified), semio-plugin-instance-history 1 (`unit/scalar.rs`), semio-plugin-runtime-ops 1 (`instance/staging/dispatch`), plus 1 in `command/custody/manifest/metadata.rs` (custody).
- Shell wasm32-unknown-unknown W-plumbing-12: rc 1, reds: reactor-jobs 18, runtime-ops 1, semio-plugin-instance-children 3 (instance lane). No plumbing crate red.

## Debts
- Tests not ported: `engine/tests/wgpu-render-snapshot-unit` (acquire/publish), `browser-worker/output/tests`, media-slots collect tests (old sink API); native-io and surface-lane tests unchecked.
- `close_shared` recovery route requires an identical grant on a recover turn (cx grants shrink).
- `osHost` seed: first `publish_on_original_caller` publishes the seed and assigns revision 1; `original_event_terminal_is_empty` is false until then.

STATUS: FIXED-verified W-plumbing-9 (host-plumbing, agent-elements native), W-plumbing-10 (wasm32-unknown-unknown), W-plumbing-8 (host-space native); shell still red in reactor-jobs, instance-history, runtime-ops, instance-children
