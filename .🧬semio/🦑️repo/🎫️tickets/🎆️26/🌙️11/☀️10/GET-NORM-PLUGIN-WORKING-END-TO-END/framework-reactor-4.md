# Framework reactor 4

## Done (source edits, not yet compiler-verified)
- Stale kernel crate `semio_framework_os_kernel` replaced by `semio_os_kernel` in all 31 non-ticket Rust files (reactor `🦀️.rs` extern crates + paths, exchange tests, plugin artifacts, framework plugin tests, dsl derive). Dependency key `semio-os-kernel` already existed in reactor-core.
- Other stale crate paths: only `semio_framework_value_native_tree` (bulk test `original-tree-work.rs`) -> `semio_framework_value_native_tree_engine`. Remaining unmatched names are WIT-generated method names or `__` macro identifiers.
- reactor-jobs: `close_original` closes via `close_original_effect_step` with `ControlledRetirement::next_demand` + `step(body, effect)`; `infer::close_field` takes the `StepContext` and runs ticket close / admission through `close_original_effect_step` / `close_original_retained_effect_step`, all callers updated; `run_io_run` uses the new `receive_nested(&mut produced, |slot,result,marker,unwind,run|)` shape.
- runtime-exchange: `ArtifactPackFiles` / media handle / fingerprint port taken from `&mut` command via `std::mem::take` (`MediaExportHandleWire` derives `Default`); `semio_plugin_document_media` path replaced by `semio_plugin_runtime_ops::ops_document::OriginalDocumentArchiveAdmission`; finish uses `with_instances_original_mut`, retains/publishes the pending-effects owner on `Ok(None)`/`Some`.

## Blocked verification
- reactor-jobs check stops upstream in `semio-plugin-instance-dispatch` (19 errors, staging producers/host-events/typed-command) and `semio-plugin-tool-run-turns` (13) - other agents' areas.
- staged-forms check stops in `semio-framework-job-native-turn` (missing `OriginalSourceCloseContext` etc. in `semio-framework-job-context`, being edited by the job-receiving agent). Earlier the job-context crate also exceeded its 262144-byte source boundary.
- reactor-core, runtime-exchange, staged-forms error sets not seen yet (blocked behind the above).
Logs: reactor4-jobs-{1,2,3}.log, reactor4-forms-1.log.

## Update (after coordinator ping)
- `🏛️reserved` job: resident `OriginalConstructionAdmission` field (`construction`), 6-arg `admit_original_construction` (status-driven phase), retired via `retire_step` in `close_step`, counted in `terminal_drop_is_shallow`.
- staged-forms: `retired` helper effect-shaped; closes via `close_original_effect_step`; decode `receiver.step` via `NativeDecodeControl::with_effect`.
- runtime-exchange: `TakeMediaExportChunk` handle via `mem::take`; ops `MediaImport::Admitting` -> `Pending` (one-line fix in ops media).
- check --all-targets PASS: reactor-staged-forms, runtime-exchange, reactor-core (logs reactor4-*-chk2.log).
- reactor-jobs: blocked upstream in `semio-plugin-instance-typed-operations` (staging: `close_original_retained_step` E inference); jobs' own files had no errors reported yet. Tests not run (jobs unverified).
