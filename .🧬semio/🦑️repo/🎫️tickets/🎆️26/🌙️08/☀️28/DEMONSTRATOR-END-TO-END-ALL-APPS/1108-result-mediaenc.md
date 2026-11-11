# Lane mediaenc result (document-media intrinsic/fault encode port, instance resident fields)

## Design (as built)
- `media/intrinsic`: `IntrinsicMediaEncoding {cursor, parts}` derives `RetireOwned` (hand cursor removed). `encode_intrinsic_parts` runs `owner.receive::<_, _, NativeSnapshotFailure>(&mut output, |slot,result,marker,unwind,native,wallet| ..)`; the frame slot owns the `BorrowedProjectedPackCursor` and parts, the framework retires them.
- `MediaEffectTurn::run` opens exactly one `NativeEncodeControl::effect_allocated_with_work` per call (marker = `NativeSnapshotEffectProgress.state.view()`, failure slot = `marker.failure`, unwind slot = frame failure), quote = cursor `next_minimum_copy_bytes/next_capacity_byte_demand/next_advance_depth_demand` (copy ceiling min(remaining,64)), `bytes` = quoted capacity, receipt = the step's own progress, charged to the wallet and `work.take`n. Denied turn: cursor returns default progress, nothing is charged, stall -> frontier error. Cursor birth, advance (measure + emit), scratch->Vec append and symbol close are each an effect; final parts move uses `move_existing`.
- `media/fault`: `receive`/`receive_nested` ported to the new closure shape (output slot written by the frame, projected via `media_snapshot_error` = kind+message+retained_progress). `MediaFaultProjection`/`MediaOutputProjection::close_step(effect) -> OriginalRetirementPoll` (via `ControlledRetirement::step(body, effect)`), Pending stays with the caller.
- Instance: `ToolOperations.original_emission_construction` (jobs staging, census index 38, `INITIAL_FIELDS` 39(+test), `test_tool_clock` now 39) and `MediaTransfers.reserved_mount_construction`; both `OriginalEmissionAdmission::admit` callers pass construction + output slot.

## Debts
- Pending cannot cross turns inside `receive`: an unfinished effect (`Ok(None)`) is an InvariantViolated failure, exactly as the framework's own `encode_effect_in` does. Public encode APIs are synchronous snapshot-owner flows.
- `ResidentConstruction` has no close step; not added to close-terminal (would deadlock a cancel mid-construction).
- `app-owned-media-error` law tests (heap/progress numbers, `close_step(&mut owner)` calls) are not ported.

## Verification log
- W-mediaenc-2 wasm: 1 error (FnOnce exporter), fixed with Option::take.
- W-mediaenc-3 wasm: blocked upstream: `semio_framework_value::close_owner` not re-exported yet (defined in value `tickets/resident`, factoryticket in flight); callers `semio-os-history-record-decode` and `semio-framework-job-worker-authority`. document-media not reached.
- W-mediaenc-4 wasm (hub manifest): document-media rc0.
- W-mediaenc-5 native: monitor expired, result unseen (disk full); superseded. W-mediaenc-6: rc1, my wrong package name (`semio-framework-plugin-instance-docs`; correct `semio-plugin-instance-docs`).
- W-mediaenc-7 native (document-media, instance jobs/docs/dispatch/app): rc1, 38 errors, none in mediaenc files: `semio-plugin-instance-stores` 1 (staging/ingress:186 E0283) and `semio-plugin-extension` 35 (invoke/failure, inference gateway/receiving). Both are claimed by the instance lane (claims 11:30), which was editing them at 19:48-19:50; mediaenc claimed then released extension. The dependents were therefore not reached: document-media and the instance crates are NOT verified native after W-mediaenc-4.
- W-mediaenc-hub-1 (hub.sh, wasm): rc1, 7 errors, not in mediaenc files: `semio-framework-diagnostic` (3) and `semio-s-artifact-stdio-contract-part21` (1).
- Rerun once the instance lane releases extension/stores: `1105b-tools/chk.sh W-mediaenc-N Cargo.toml native -p semio-plugin-document-media -p semio-plugin-instance-jobs -p semio-plugin-instance-docs -p semio-plugin-instance-dispatch -p semio-plugin-instance-app`, then `1105b-tools/hub.sh W-mediaenc-hub-N` (run from the repo root; chk.sh resolves slot.sh relative to it).

## Round 2: semio-plugin-testing-caller port (coordinator assignment)
- `semio-plugin-extension` and `instance-stores` stay with the instance lane (mediaenc claimed then released extension; no edits). `diagnostic` and `stdio-contract-part21` went green on their own (femrecv `NativeDefaultSlot` fix, W-mediaenc-hub-2).
- testing-caller (21 errors): `OriginalFixtureCaller::admit` births the Source parent through `OriginalCallerSourceAdmission` (prepare, backing, publish) with `bank_step` over an `EffectBank`, receipts summed; `JobPayloadAuthority::admit` takes its actual-slot; `retire` closes the parent inside `OriginalRetirementPump::with_effect`; `retire_fixture_decoder` borrows a Source context per turn and closes with `close_on_source` (the decoder owner no longer closes from a bare grant). Work fixture: `retire_owned` via `close_original_effect_step`, `try_receive_native` follows the production pack driver (source admission, `with_native_receiving_quoted` + `source_witness`, `take`, owner retired through `retire_owned`).
- Cargo.toml: + path deps `semio-framework-job-testing-caller`, `semio-framework-value-testing-effect-bank` (LOCK NEEDED, logged).
- Verified: native W-mediaenc-tc-3 rc0 (lib). Hub W-mediaenc-hub-3 (wasm): testing-caller, extension, stores, diagnostic, part21 no longer red.
- No `[DEBUG]` prints exist in the plugin or job `🎟️caller` code. One remains in `🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:257` (not a caller file, not touched).
- Debts: the `#[cfg(test)]` modules of testing-caller (`original_physical_laws`, `host_lane_authority_law`, `👷️work/🧪️tests`) still use the old grant API and were not compiled or ported; `roots.copied_items==3` assertion unchecked; non-`Primary` native receiving failures are collapsed to an invariant refusal (their retained causes are dropped).

## Remaining hub reds (W-mediaenc-hub-3, rc1), none in mediaenc files
- `semio-plugin-runtime-ops` 27: custody2 (claimed 10:16, edited 19:29; lock-152 queued).
- `semio-plugin-reactor-jobs` 13 (`💡️infer`, `🏛️reserved`, `🦀️.rs:303-306,659`): shellreds edited it unverified (claims 10:05); unclaimed since.
- `semio-plugin-instance-history` 1 (`📥️staging/📨️dispatch:120` E0283): instance lane (stores sibling).

## Round 3: reactor-jobs port and typed receiving pass-through (coordinator assignment)
- reactor-jobs (13): `🦀️.rs` close step runs inside `cx.with_original_effect` (`owner.step(body,effect)`), `run_io_run` uses the new `receive_nested` closure shape (`|slot,out,marker,unwind,run|`, result written to the output slot); `💡️infer` `close_field(source,active,cx)` is effect-shaped (`close_factory_ticket(active,body,effect)`, `admit_owned_retirement(source,effect)`, performed work charged, no flat fuel), all 17 callers updated; `🏛️reserved` job carries an `OriginalConstructionAdmission` and uses `admit_original_construction_settled` (+ its close in the ladder).
- Admission close quote is exact, not estimated: added `OriginalConstructionAdmission::close_demand()` in job `🏭️construction` (mirrors the branches of `close_step`); `retirement_demands` uses it.
- testing-caller `try_receive_native` now returns `OriginalReceiveFailure {Turn(OriginalTurnFailure), Native(NativeReceivingFailure)}` (RetireOwned, `receiving()`, `retained_progress()`), so native receiving causes are passed through typed and the caller retires them. A held `OriginalNativeReceivingOutcome` is kept as `OriginalTurnPrimary::NativeSource`. Callers ported: work tests, raster pixels fixtures (`error.receiving()`), mask-from-selection `decoded_spans` (now `Result<_,String>`, retires the owner); the other raster callers already pass the error to `retire_owned`.
- Verified: native W-mediaenc-rj-2 rc0 (reactor-jobs, testing-caller, job). Not compiled: the `#[cfg(test)]` modules and the raster test callers.

## Hub W-mediaenc-hub-4 (wasm, rc1), reds newly exposed once the earlier ones cleared, none in mediaenc crates
- `semio-plugin-instance-jobs` 27: `📥️staging/{🧾️typed-operations (11), 🖼️render (6 + work.rs 2), 🏷️reserved (3), 🧱️components/🏛️declarations (2 + 🌱️capture 3)}` grant->effect leftovers. The components file is the one mediaenc edited; these sites are in neighbouring staging files.
- `semio-plugin-runtime-exchange` 10: `🔀️plugin-exchange/{🎞️media-commands 4, ⚙️config-commands 2, 📦️document-archive-commands 2 (missing `semio_plugin_document_media` dep/import), 🏁️exchange-finish 1, root 1}`; aussuchen claimed this crate earlier (13:02, fixed then), so a regression or new breakage.
- `semio-plugin-reactor-cold-pair` 3 (`📸️checkpoint` admit_original_construction arity), `semio-plugin-reactor-surfaces` 3, `semio-framework-plugin-reactor-staged-forms` 1 (unresolved import `semio_framework_plugin_reactor_app`).
- Gone from the hub: runtime-ops, reactor-jobs, testing-caller, extension, stores, history, diagnostic, part21.

## Round 4: runtime-exchange, cold-pair, surfaces, staged-forms, reactor-turn (coordinator assignment)
- runtime-exchange: `OriginalDocumentArchiveAdmission` from `ops_document`; donated `&mut` fields taken or cloned; `🏁️exchange-finish` pulls the pending effects inside `with_instances_original_mut`, resumable by `state.mutated` / `state.effects_done`; `record_pending_effects` is the single recording step (a `None` pull changes nothing, a re-entry after `Some` never pushes twice) with the `reentry_laws` test.
- cold-pair/checkpoint: `restore` queues `RestoredDocumentLoad{instance,archive,admission,admitted}`; `step_restored_document_loads` admits through `plugin_begin_document_archive_load(.., &mut archive, &mut admission, cx)`. A refused admission becomes a typed fault via `admission_fault` (retained progress plus receipt, context and work refusals as `FaultCause`). `SharedUtf8::admit` work returns from `admit_restored_actor`, is summed in `restore`, parked in `RESTORED_WORK` and charged with `cx.consume_work` at the top of the step.
- staged-forms: `use semio_plugin_reactor_app::PluginApp;`; `staged.rs` uses the new cx-shaped `OriginalNativeDecodeSource::receive_effect_step`.
- reactor-turn: the 23 errors were handed to the reactorturn lane (see `1108-result-reactorturn.md`), which finished them (native `W-reactorturn-2` rc0).
- Debts: inline tests of testing-caller and the raster test callers are uncompiled; `app-owned-media-error` law tests not ported; cargo lock refresh for testing-caller Cargo.toml (+2 path deps).

STATUS: FIXED-verified wasm document-media (W-mediaenc-4 rc0), native testing-caller + reactor-jobs + job (W-mediaenc-rj-2 rc0); runtime-exchange, cold-pair, surfaces, staged-forms, reactor-turn closed by the reactorturn lane (native W-reactorturn-2 rc0); see 1108-result-reactorturn.md for the hub rerun and the `reentry_laws` run
