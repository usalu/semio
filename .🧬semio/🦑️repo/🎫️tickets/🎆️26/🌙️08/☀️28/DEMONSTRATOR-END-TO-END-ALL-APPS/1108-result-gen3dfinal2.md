# Lane gen3dfinal2 result

## Round 1 (11/11)
- W-gen3dfinal2-1 (native, instance-dispatch + tool-run-turns): 21 + 14 errors as in the gen3dfinal handover.
- EDITED (compile W-gen3dfinal2-2 pending):
  - dispatch producers: `semio_framework_plugin_retained_command` path, `reserved_construction: Default::default()`, `.pause()` mapped with `error.to_fault()` (typed ValueError kept), `admit_original_reserved` gets `media.reserved_mounted_construction` (new `ResidentConstruction<Box<MountedTypedCommandFullOperation>>` field in instance-docs `🧱️components`).
  - typed-command: PendingLatestWinsCommand init without `retired_key`/`receipt`; `original_decoder` is `(NativeAllocationIdentity,u64)`; config completion and admission-proof alias closes use `close_original_effect_step`.
  - host-event-routing `SourceScope` decoder identity type; `RetainedWorkOutcome::<_,ValueError>` annotations (close.rs, host-events, host-event-routing).
  - emit/original phase 0 uses `OriginalEmissionAdmission::capture_step`.
  - tool-run-turns publication: `with_native_receiving(&mut out,..)` + `settle_receiving_scope`; outbound via `with_original_effect` + `consume_work`; finalize close via `plugin_effect_turn` + `RetirementPollExit::settle`; member-retirement `owner.close_step(effect)`.
- Helper: `1108-gen3dfinal2-errs.py <txt> <path-substring>` prints errors with the source snippet at the column.
