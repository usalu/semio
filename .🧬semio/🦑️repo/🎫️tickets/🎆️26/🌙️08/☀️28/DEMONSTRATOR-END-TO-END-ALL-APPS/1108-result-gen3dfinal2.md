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

## Round 1 result
- FIXED-verified W-gen3dfinal2-3 (native): semio-plugin-instance-dispatch rc0 and semio-plugin-tool-run-turns rc0 (0 errors).
- Next: `chk.sh W-gen3dfinal2-4 Cargo.toml native -p semio-plugin-instance-* -p semio-plugin-runtime-exchange`, surface-fixture natural-file test, generation3d, hub wasm, smoke en/de.

## Round 2 (handover, build blocked upstream)
- My W-gen3dfinal2-3 green (dispatch + tool-run-turns) predates the 02:43 stash-apply/origin merge. The merge replaced parts of my dispatch edits with origin's parallel port: emit/original now calls the 3-arg `OriginalEmissionCapture::admit(cx,&mut source.construction,&mut source.capture)` and `admit_original_emission_receipt` takes an admission; `capture_step` is not used any more. Edits confirmed still on disk: instance-docs components `reserved_mounted_construction`; typed-command/producers partially (to_fault mapping, `reserved_construction`); tool-run-turns publication needs re-check (plugin_effect_turn count dropped).
- W-gen3dfinal2-6 (15 instance crates + runtime-exchange) red upstream: job-context source/child:23,37,40; job-child-vcs cascade:20 (missing `RetainedWorkPoll` import); composition-publication follow:72 (NativeConstructionScope vs NativeDecodeControl); document-media fault:9 (MediaControlled/MediaRetireOwned missing); tool-machine 1 err. Likely stash-apply artifacts (stashmerge lane).
- W-gen3dfinal2-7 (dispatch + tool-run-turns only): red only in `semio-framework-value-snapshot-projection` 📦️owned/🦀️.rs:14 (closeshared edit 03:37; OwnedFieldProjection `close_demands`/`close_work_demand` bounds). Dispatch and tool-run-turns were not reached.
- Not done: tasks 3 (15-crate recheck), 4 (surface-fixture natural-file test run; test code reviewed: loops prepare_media_import + advance_original_media_import with 4096 stall assert, then settle_registered_typed_operation; settle waits on has_pending_typed_operations/maintenance_terminal_is_empty so it covers the mounted import op and the preparation-slot close at operation retirement, unverified by a run), 5 (generation3d native/wasm, hub 8-crate wasm), 6 (smoke en/de).
- Rerun order once closeshared reports FIXED: `chk.sh W-gen3dfinal2-8 Cargo.toml native -p semio-plugin-instance-dispatch -p semio-plugin-tool-run-turns` (re-port anything the merge reverted with `1108-gen3dfinal2-errs.py <txt> <path-substring>`); then the 15 instance crates + `-p semio-plugin-runtime-exchange` (pass each -p separately, not as one quoted string); then `tst.sh ... -p semio-plugin-surface-fixture` natural-file law; generation3d; hub wasm; smoke.

## Round 3 (11/11 03:50-04:30)
- Re-checked on disk: tool-run-turns publication/member-retirement edits are intact (with_native_receiving scope, outbound effect, plugin_effect_turn closes). Dispatch (producers, typed-command, emit/original) now holds origin's parallel port (`admit_original_reserved(..,&mut owner.construction,&mut owner.mounted)`, 3-arg capture admit); nothing of mine is left to re-port by hand, only a compile can tell (the `semio_plugin_retained_command::` paths in producers l.28/60 may need the `semio_framework_plugin_retained_command` name, key_retirement, RetainedWorkOutcome annotations: see Round 1 list).
- W-gen3dfinal2-8 (dispatch + tool-run-turns, native) queued at the capacity door ~35 min (host swapping, 8 ahead) and not started; it dies with my turn. Rerun it, read errors with `python3 -I 1108-gen3dfinal2-errs.py <txt> <substring>`.

## Round 4 (11/11 morning)
- closeshared FIXED. W-gen3dfinal2-8 red in publication-mounted `📚️members/🦀️.rs:155` (EDITED: `return row.close_step(..)` without `Ok(..)`). W-gen3dfinal2-9 then red only in instance-docs (merge regression of the instdocs port): maintenance-ladder:200,238 (`observed` now `u128::from(..last_turn_native_work())`) and checkpoint:41 (`polled(poll?)`); both EDITED-unverified.
- paneC edited dispatch `producers` (removed duplicate cx-form `advance_original_reserved_action` and 3 unused leftovers); W-paneC-id-1 is queued. I do not edit producers.
- W-gen3dfinal2-10 (dispatch + tool-run-turns, native) is queued at the capacity door (host swapping, 7 ahead); chk dies with my turn. Rerun: `chk.sh W-gen3dfinal2-11 Cargo.toml native -p semio-plugin-instance-dispatch -p semio-plugin-tool-run-turns`, read errors with `python3 -I 1108-gen3dfinal2-errs.py <txt> ""`.
- Slot holders W-closeshared-t9 (02:32) and W-matcherack-14 (02:30) looked stale.
- Tasks 3-6 still not run (15-crate recheck with each `-p` passed separately, surface-fixture natural-file law, generation3d native/wasm + hub 8-crate wasm, smoke en/de).

STATUS: BLOCKED capacity queue; dispatch + tool-run-turns EDITED-unverified (W-gen3dfinal2-10 not started); tasks 3-6 not run
