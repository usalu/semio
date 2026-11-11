# mediaenc handover (11/10, context low)

C = this ticket folder. Result file: `1108-result-mediaenc.md` (append a Round 4 + final STATUS line). Claims log: `1105b-claims.md`. Tools: `1105b-tools/chk.sh <label> Cargo.toml native|wasm -p <pkg>...` and `1105b-tools/hub.sh <label>` (run from the repo root with absolute paths; output in `🗑️generated/1105b/<label>.txt/.rc`; run in background and wait on the .rc file). No git modifications.

## State
- Verified: wasm document-media (W-mediaenc-4), native testing-caller + reactor-jobs + job (W-mediaenc-rj-2 rc0).
- Hub W-mediaenc-hub-4 reds left to the instance lanes: instance-jobs 27 (instance + instjobs2 lanes).
- Round 4 (coordinator): runtime-exchange 10, cold-pair 3, surfaces 3 (same checkpoint errors), staged-forms 1. EDITED, hub-5 was running (check `W-mediaenc-hub-5.rc/.txt`):
  - exchange: `semio_framework_plugin_runtime_ops::ops_document::OriginalDocumentArchiveAdmission` replaces `semio_plugin_document_media::..`; donated `&mut` fields taken/cloned (config pack/spr, media handle clone, fingerprint port take, arm_media_in descriptor `&mut` + clone); `🏁️exchange-finish` pulls pending effects through `with_instances_original_mut`, resumable by `state.mutated` / `state.effects_done`, `None` returns `PluginExchangeOutput::default()`.
  - checkpoint (`⚛️reactor/📸️checkpoint/🦀️.rs`): `restore` queues `RestoredDocumentLoad{instance,archive,admission,admitted}` (no cx there); `step_restored_document_loads` (`🔄️turn/🧊️cold-pair-loads-and-process-pool/🦀️.rs`) admits with `plugin_begin_document_archive_load(.., &mut archive, &mut admission, cx)`; static `RESTORED_DOCUMENT_LOADS` now holds `checkpoint::RestoredDocumentLoad`.
  - staged-forms `forms.rs`: `use semio_plugin_reactor_app::PluginApp;`.
- Coordinator's three follow-ups (NOT done yet unless the result file says so):
  1. admission failure -> typed fault: build the Fault from the OriginalTurnFailure keeping `retained_progress` (set `fault.retained_progress`) and causes (receipt/context/work refusals as `FaultCause`), instead of `format!("{failure:?}")` in `step_restored_document_loads`.
  2. `SharedUtf8::admit` work in `admit_restored_actor`: return `(ActorId, work)`, sum in `restore` (`RestoredCheckpoint.work`), park in a new static `RESTORED_WORK` (cold-pair `🦀️.rs`, re-export in reactor root line ~74), charge with `cx.consume_work(..)` at the top of `step_restored_document_loads`.
  3. law: factor the pending-effects phase of `finish_exchange` into a helper and test that a `None` pull leaves state untouched and a re-entry after `Some` does not push effects twice (`state.effects_done`).
- Debts to mention in the result: inline tests of testing-caller and raster test callers uncompiled; finish_exchange retry semantics unrun; cargo lock needs refresh for testing-caller Cargo.toml (+2 path deps).


## Update 11/10 late (supersedes the "NOT done" list above)
- DONE (edited, hub-compiled where noted): follow-up 1 (`admission_fault` in `🔄️turn/🧊️cold-pair-loads-and-process-pool/🦀️.rs` keeps retained progress + refusals as causes), follow-up 2 (`RESTORED_WORK` static, `admit_restored_actor -> (ActorId, work)`, charged by `cx.consume_work` at the top of `step_restored_document_loads`), follow-up 3 (`record_pending_effects` + `reentry_laws` test in `🏁️exchange-finish/🦀️.rs`; the test is NOT yet compiled or run natively).
- staged.rs:61 (coordinator ruling): added `OriginalNativeDecodeSource::receive_effect_step` in job `⏱️context/🛬️decode/🦀️.rs` (cx-shaped, effect passed to `receiver.step`, mirrors `close_producer_effect_step`); `📥️receiving/staged.rs` uses it. Deviation: the Record receiver has `demands(wire)`, not `advance_work_demand`, so no quoted-native-receiving API is used.
- reactor-root fixes: `extern crate semio_os_kernel as store/protocol`; `semio_plugin_reactor_app::` replaces the stale `semio_framework_plugin_reactor_app::`.
- Hub history: hub-5 -> hub-6 -> hub-7 (rc1: instance-docs 27 [new lane], reactor-turn 23 [now mine, whole crate]) -> hub-8 (running when this was written; read `W-mediaenc-hub-8.rc/.txt`).
- NEXT for a successor: (1) read hub-8, list errors per file for `semio-plugin-reactor-turn` (path `⚛️reactor/🔄️turn/**`, hub log `🗑️generated/1105b/W-mediaenc-hub-8.txt`), port them to the effect/`cx` API (patterns: `cx.with_original_effect(grant,|_,effect|..)`, `close_original_effect_step`, `admit_original_construction_settled`, `ControlledRetirement::step(body,effect)`); (2) rerun `hub.sh`; (3) native `chk.sh` on reactor crates + run `reentry_laws` (`1105b-tools/runtime/tst.sh`); (4) append Round 4 + final STATUS to `1108-result-mediaenc.md` (exchange, checkpoint, staged-forms, reactor-turn, debts: uncompiled test modules, unrun finish_exchange retry semantics, cargo lock for testing-caller).

## reactor-turn (23 errors, W-mediaenc-hub-8, ALL mine per coordinator) - state at handover
Done: `command-ingress-turn` `consume_fuel(u64::try_from(turn.work)..)` x2; Cargo.toml +`semio-plugin-ownership` path dep (LOCK NEEDED, logged).
Still open (files under `⚛️reactor/🔄️turn/`, hub log `🗑️generated/1105b/W-mediaenc-hub-8.txt`):
- `🔌️extension-invocation-and-command-ingress/🦀️.rs`: lines 29, 37 `retired(cx,demand,|loan|closing.step(loan))` -> `closing.step(body,effect)` with `close_original_effect_step`-shaped `retired`; line 84 `plugin_typed_owner_demand/_work_demand/_original(owner,closing,grant)` (crate path now resolves after the dep; check their new signatures).
- `📥️receiving/🦀️.rs` 108, 126: `PagedList::release_empty_page(effect)` (RetainedWorkOutcome, not Result) -> run in `cx.with_original_effect(grant,|_,effect|..)`, charge `performed`.
- `📥️receiving/📦️completed/🦀️.rs` 45 `grammar.step_source(&source,1,control,EFFECT,false)`, 76 `RetainedIntrinsicDocument::new(input,limits,MAX,effect)`, 78/87 `advance_original_step(effect)` / `close_original_step(effect)`: wrap in `cx.with_original_effect`.
- `⌨️command-ingress-turn/🦀️.rs` 137: `admit_original_construction(cx,exchange,&mut admission,demand,1,|native,actual|..)` returns `OriginalConstructionStatus` (not bool); add an `OriginalConstructionAdmission` field + settle (see `admit_original_construction_settled` and `🏛️reserved` job as precedent).
- `🧊️cold-pair-loads-and-process-pool/🦀️.rs` 181-183: `plugin_begin_document_archive_load(runtime,instance,op,&mut archive,&mut admission,cx)` -> `Result<bool,OriginalTurnFailure>`; use the `admission_fault` helper in the same file (line ~60) for `encode_fault_bytes(&fault)`; the `pending.archive()` owner needs `&mut`.
- `🌀️poll-kernel-turn/🦀️.rs` 63: `TurnWorkInputs` needs `pending_effects_runnable: bool` (definition `🔦️more-work-sources-and-tracing/🦀️.rs:207`); the truthful source is the `None` answer of the effects pull in `🏁️exchange-finish` (not yet plumbed).
