# 1108 toolrun result

## Step 1 (EDITED-unverified, value tree red upstream: W-toolrun-1/2/3 stop in value-control-contract / value-retirement / shared-issuance / native-encoding / retirement-queue, all value lane)
- job: added `close_original_retained_effect_step(cx,demand,|effect|->Result<RetainedCloneStep,E>)` (retirement/context) = `close_original_effect_step` with a complete poll of `effect.performed_work()`; exported via job-context. (The existing `close_original_effect_step` already existed; the "missing" helper is this synchronous variant.)
- tool-run `retirement/entry`: local `retired` is effect-shaped (`performed(1)` then closure(effect)); `retire_owned_mutation(&mut self.operation,effect)`; finalize ingress passes `effect.grant()`.
- tool-run `driver-and-ledger`: pending_discarded retirement moved to `plugin_effect_turn(cx,|effect|store.retire_owned_mutation(..,effect))` like the alias block.
- tool-run `member-owners-and-visitors`: `ToolRunMemberFold<'a,'b,'e>`, `ToolRunMemberEmit<'a,'e>`, `ToolRunMemberEmissionRetire<'a,'e>` gain `effect`; ToolRunMemberRetire uses `self.effect`. SIGNATURE CHANGE posted in claims for the instance callers (not edited by me).
- child return pump: `take_returned_snapshot_read_retirement(&mut effect.nested(child))`.
- engine-close bin-01 tests: 5 `close_cursor_admit` calls wrapped in `effect_tests::with_effect` (mounted from paged-one-item `⏱️effect` helper in bin-01 `🦀️.rs`).
- engine-close pending ruling and engine-bundle:148 were already applied by toolsplit/gateway (checked by reading); compile verdict pending.

## Open
- Run `W-toolrun-N` (native + wasm, `--tests`) once value lane is green.

## Step 2
- tool-machine law `🟦️.ts` string updated to `retire_owned_mutation(&mut typed.current,self.effect)`.
- W-toolrun-4 and W-toolrun-5 (native check of tool-machine, tool-run, operations, turns, engine-close, engine-bundle) rc 1: only upstream value crates red (value-factory 3, native-bounded-ordering 9, native-encoding 2, original-aliases 1, retained-work 1, retirement-queue 10), owned by foundation/declint/operation. None of my crates reached, so none of my edits has a compile verdict; wasm, `--tests` not run.
- Rerun when those are FIXED: `chk.sh W-toolrun-6 Cargo.toml native -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns -p semio-os-store-engine-close -p semio-os-store-engine-bundle`, then `wasm`, then `native --tests` same list. Expect fix rounds: grant-shaped children inside tool-run `retired(..|effect|..)` closures (`publication.close_step(child)`, `metadata.step(grant)`, `close_tool_erased_retirement`, `FactoryOwnedRetirement::admit_original`) may be stale.
- Debts: instance callers (staging :61, publication-refresh :185, member-retirement-and-close :45) must add `effect` (not mine). Gateway test sites outside my scope (bin-02 :514, interaction/window-config) untouched. No [DEBUG] lines added.



## Step 3 (polling per coordinator)
- W-toolrun-6..14 native: wall moved down (native-decoding -> job-outcome/persistent/read -> list-receiver 68 + tree-engine -> protocol unbalanced delimiter). W-toolrun-14 (latest): only `semio-framework-value-native-owned` red (204 errors, nested lane's reply-shaped NativeReceiver port, claimed). My crates not reached; no verdict on any of my edits. Not yet applied: the 733 `body: usize` closes in tool-run closures (needs compiler feedback).
- Rerun: `chk.sh W-toolrun-15 Cargo.toml native -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns -p semio-os-store-engine-close -p semio-os-store-engine-bundle`, then wasm, then `--tests`.


## Step 4
- W-toolrun-15 (after native-owned green): rc 1, upstream only: value-edit 56, value 54 (follows value-edit), native-receiving-original 37 (nested), job-receiving 1 (`🎟️state:68` `recipient.close_step(effect)` lacks `body`; escrow lane claimed that file at 19:34). My crates still not reached.
- Rerun: `chk.sh W-toolrun-16 ...` same list, then wasm, then `--tests`; then apply `body: usize` where tool-run closures call close_step.

STATUS: BLOCKED value-edit / native-receiving-original / job-receiving red, W-toolrun-15; edits EDITED-unverified

## Step 5 (value facade split, coordinator task)
- New crate `semio-framework-value-shared-utf8` (manifest `🌱️value/📝️shared-utf8/📦️packages/🦀️rust`, lib `../../🦀️.rs`): FIXED-verified native lib W-toolrun-20 rc 0, wasm lib W-toolrun-22 rc 0. SharedUtf8 retirement is `Utf8LeaseRetirement` (Arc lease, payload retired as ChildProgress); clone cursor ported to the reply-shaped protocol (advance_into/close_into). Member line added, LOCK NEEDED + INTERFACE posted. `Utf8Text` impl must move to semio-framework-value-paged (instance-b, INTERFACE posted); ToValue/FromValue impls for SharedUtf8 now in `🌱️value/🦀️.rs`.
- Facade ports (EDITED-unverified; facade W-toolrun-21 stops in value-paged, instance-b): publication root retirement, paged map receive, testing wire.
- Debts: shared-utf8 own tests (`🧪️tests`, companion mount) not ported; `native_retirement_reply!` is crate-private, inlined twice.
- tool-run crates: not reached (facade red).

STATUS: BLOCKED value facade red in value-paged (instance-b), W-toolrun-21; shared-utf8 crate FIXED-verified native+wasm lib

## Step 6
- Coordinator rulings applied: ToValue/FromValue for SharedUtf8 live in the shared-utf8 crate; `native_retirement_reply!` is `#[macro_export]` from semio-framework-value-retirement (+ `retirement::__control`), used by shared-utf8 and publication root; shared-utf8 tests ported to three `[[test]]` in the new crate (shared helper `🧪️tests/⏱️effect`; native test reduced to the funded receipt + paid close). Test build needs the crate to be a test-visible workspace member (lock/sync): W-toolrun-23 "not a member of the workspace".
- semio-framework-value native lib FIXED-verified (W-toolrun-28 rc 0): publication root forwards advance/return_original_read_into (+ recover) and `original_read_demands(key,body)`; paged map receive and testing wire done.
- W-toolrun-29 (native check of my six crates): rc 1, my crates not reached. Red upstream: framework-async 5, io-failure-owner 1, pack-json-canonical 12, replication-retained-field 23, replication-retained-key 1, **tool-run-entities 41** (733 port: SealedShared::admit, ordered-map insert/remove cursors, paged-version/provenance), value-edit 26, value-snapshot-read 39, os-semio, window-config-mutation, ui-locale (1 each). tool-run-entities is the next piece in my family (toolsplit edited it unverified); not started.
- Rerun: `chk.sh W-toolrun-30 ...` same six crates, then wasm, then `--tests`; then body: usize ports in tool-run closures.

STATUS: BLOCKED upstream red crates incl. tool-run-entities (unstarted, mine), W-toolrun-29; value facade + shared-utf8 FIXED-verified native

## Step 7
- Facade: semio-framework-value + value-shared-utf8 FIXED-verified native (W-toolrun-28) and wasm32-wasip2 (W-toolrun-30, rc 0). Posted.
- tool-run-entities (41, W-toolrun-29), analysed, NOT ported: `👥️entities/🦀️.rs` `Editor` wraps `RetainedOrderedMapInsertCursor/RemoveCursor` whose 733 API is reply/route based only (`advance_into/recover_advance_into/take_into/begin_close_into/close_into` with `OriginalEffectReply`, `advance_demands(body,grant)`, `take_refused_workspace_into`). The synchronous `Editor::advance/take/begin_close/close_step` therefore need a reply-shaped redesign of `ToolRunEntityEdit` (stash the reply in a state slot like `SharedCloneRetirement::relay`, recover on `Pending`), plus: `SealedShared::admit` -> `admit_from`/`prepare_birth(&mut Option<Birth>,effect)`/`publication`, `StagedSourceAdmission::borrowed(original)` takes one argument (projection moved), and `📸️version`, `🧾️provenance` equivalents (BoundedOrd cursors, NativeRetainedCloneBacking `advance/begin_close` removed). Also depends on instance-b's BoundedOrd port. This is a design port, not a call-site swap; it blocks all six target crates (the crates depend on entities).
- Other red upstream in W-toolrun-29: value-edit 26, value-snapshot-read 39, replication-retained-field 23, pack-json-canonical 12, framework-async 5.

STATUS: BLOCKED tool-run-entities needs the reply-shaped Editor redesign (unstarted); upstream value-edit/snapshot-read red; facade FIXED-verified native+wasm

## Step 8
- W-toolrun-31 (native, six crates, after tool-run-entities FIXED): rc 1, my crates still not reached. Red upstream: dsl-record-model 26 (receiving/owned/*: persistent builder, text, intrinsic; 733 close_step/body), replication-history-index 9 (fold/index: reply `?`), replication-codec 1 (+close_step body), presence-native-snapshot 5, io-ports 1, pack-json-native-input 1. sync-120.rc does not exist yet, so no `--tests`.
- Rerun: `chk.sh W-toolrun-32 Cargo.toml native -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns -p semio-os-store-engine-close -p semio-os-store-engine-bundle`, then wasm, then `tst.sh ... --tests` after sync-120.

STATUS: BLOCKED upstream dsl-record-model / replication-history-index / replication-codec / presence-native-snapshot red, W-toolrun-31

## Step 9 (coordinator reassignment)
- FIXED-verified native+wasm: replication-history-index, replication-codec, io-ports (W-toolrun-34/35; io bytes `close_step(_body)`, ports prepare `close_one(body)`, `derive(RetainedClone)` uses `OriginalEffectReplyScope::borrow(&mut scope)` because a user `use std::borrow::Borrow` shadowed the inherent method), and job-failure (W-toolrun-38 native, -39 wasm; `semio_framework_value::native_retirement_reply` re-export added to the facade).
- W-toolrun-41 (six crates, native): rc 1, not reached. Red upstream: job-context 61 (iocontrol mid-edit), pack-record-encode-cursor 8, pack-record-retained 2, replication-mutation 7 (bridge), ui-viewport 1.
- Rerun `chk.sh W-toolrun-42 ...` six crates, then wasm, then `--tests`.

STATUS: BLOCKED upstream job-context (iocontrol), pack-record-encode-cursor/retained, replication-mutation (bridge), W-toolrun-41

## Step 10
- FIXED-verified native+wasm: pack-record-encode-cursor, pack-record-retained, ui-viewport (W-toolrun-46/47). pack wire cursor handback uses `take_authority_into` through a synchronous `reply_turn` helper (a pending reply is a typed refusal); four cursors got inherent `close_step(body,effect)` + `native_retirement_reply!`.
- W-toolrun-48 (six crates native): rc 1, not reached. Red: job-context 15 (iocontrol), replication-edit 13, pack-intrinsic 1.

STATUS: BLOCKED upstream job-context (iocontrol), replication-edit, pack-intrinsic, W-toolrun-48

## Step 11 (review fix)
- Pack wire cursor hand-back is reply-shaped: fields `take_reply`/`take_pending`; phase 8 calls `take_authority_into` on first turn, `recover_take_authority_into` after a pending reply, returns a no-progress step while pending (state kept, retried next turn); `terminal_is_empty` includes `!take_pending`. The synchronous refusal helper is gone. Native: encode-cursor compiled with no error in W-toolrun-49 (before the reply stash edit it was W-toolrun-46/47 rc 0; recheck native+wasm after pack-json source-cap clears).
- pack-intrinsic: `owner.step(grant.maximum_copy_bytes,effect)` (edit, uncompiled: W-toolrun-50 stopped at a pack-json Source-extent refusal, packjson's).
- paged: `impl Utf8Text for SharedUtf8` + dep on value-shared-utf8 added (LOCK NEEDED value-paged manifest).
- replication-edit (13) is NOT fallout from job-context: it is a design port of three canonical files (io/text/canonical seal, mutation/canonical identity + authority): `project_owned` -> `RetainedCloneRef::try_project` + `RetainedOwnedProjectionAdmission`, `take_authority` -> reply-shaped `take_authority_into` with a stashed route/reply (same pattern as the pack wire cursor), `RetainedCloneSourceTake` removed, `source.close_step(body,..)`, `owner.step(body,..)`, `SharedUtf8: Utf8Text` (now supplied). Not started.

STATUS: BLOCKED pack-json source-cap (packjson) for verification; replication-edit design port unstarted

## Step 12
- FIXED-verified native+wasm: pack wire cursor (reply-shaped handback), pack-intrinsic, value-paged (W-toolrun-51/52); replication-edit (W-toolrun-53/54): seal uses pack-json `retained_alias` over `RetainedCloneRef::try_project` and reply-shaped `take_authority_into` with `take_reply`/`take_pending`; identity/authority/seal get explicit `body` on close/step. Runtime note: seal passes its caller effect to `take_authority_into`; the effect must carry an original invocation at run time (unproven).
- W-toolrun-55 (six crates): still not reached. Red: job-context 15 (iocontrol; mid-edit borrow errors), replication-conflict 6 (⚔️conflict/♻️retirement: close_step/close_step_controlled/close_original_step trait arity, body args), replication-history 15 (fold: close_step arity, birth/original lifetimes). Coordinator FYI: `ToolRunEntityEdit` and provenance `advance_into` now take `body` (entities lane); adapt tool-run callers once crates compile.

STATUS: BLOCKED upstream job-context (iocontrol), replication-conflict 6 + replication-history 15 (unclaimed), W-toolrun-55

## Step 13 (blind preparation, per coordinator)
- Mapped the callers of the entity edits: framework `tool-run/📊️progress/🪟️presentation` (`🗃️slot` advance, `🧬️update` State::advance phases 3/8/12 use `child.advance(grant,effect)`, `child.take_output(grant|effect)`, `settle_close(child.step(effect))`) and plugin tool-run `🎞️tick/🛬️native` (builds `ToolRunEntityEditGrant`, line ~100). The new entity interface (INTERFACE line 21:12/21:40) is `advance_into(body,grant,effect,reply)`, `recover_advance_into`, `take_output_into`, `next_demands(body)`; the presentation state machine therefore needs the same stash pattern as the pack wire cursor (`reply` slot + pending flag + recover) and `step(body,effect)` on every close. I did NOT edit these blind: the presentation crate has not been compiled against the 733 value layer yet (W-toolrun-56 stops upstream at replication-history 15 and replication-mutation-messages 21, both repl), so any blind edit would stack guesses on unseen errors.
- Rerun when repl and birth post FIXED: `chk.sh W-toolrun-57 Cargo.toml native -p semio-framework-tool-run-presentation -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns -p semio-os-store-engine-close -p semio-os-store-engine-bundle`, then wasm, then `--tests`.

STATUS: BLOCKED upstream repl (replication-history, mutation-messages) and birth (job-context), W-toolrun-56

## Step 14
- W-toolrun-57/58 (presentation + six crates, native): rc 1, my crates not reached. Red: replication-retained 51-54 (retained lane, claimed 22:38), job-context 1-2 / job-park 2 / job-return 6 (jobs lane, claimed 22:40; `close_original_effect_step` missing at crate root of continuation, private `into_source_loan`/`into_borrowed_parent`).
- Rerun W-toolrun-59 (same list) once those post FIXED; then wasm, `--tests`, then the presentation/tick caller port against real errors.

STATUS: BLOCKED upstream replication-retained (retained), job-context/job-park/job-return (jobs), W-toolrun-58

## Step 15
- W-toolrun-59/60 (presentation + six crates, native): rc 1, my crates not reached. Red: replication-retained 20 -> 42/48 (retained lane still editing set-field/diagnostic: decide/inverse/close_step reply traits) and framework job 5 -> 2 (construction `NativeReceivingEffectPoll`, return `close_step_granted`; jobs lane). Both lanes' W-*-green posts are stale against the live tree.
- Rerun W-toolrun-61 when retained and jobs repost; then wasm, `--tests`, presentation/tick caller port.

STATUS: BLOCKED upstream replication-retained (retained), job (jobs), W-toolrun-60

## Step 16
- W-toolrun-61: rc 1, my crates not reached. Red (all claimed by declint, 23:17, frontier of W-declint-64): job-publication, job-fault-publication (RetainedPayloadBuilder::close_step_granted missing), job-native-decode, job-worker-authority, actor-retained-turn, ui-contract-values (14), replication (facade exports of the reply-shaped retained API).
- Rerun W-toolrun-62 after declint posts; then wasm, `--tests`, presentation/tick caller port.

STATUS: BLOCKED upstream declint frontier (job-publication/native-decode/worker-authority, actor-retained-turn, ui-contract-values, replication), W-toolrun-61

## Step 17 (2026-11-09)
- Cause-loss fixed (EDITED, uncompiled): `tool-run/🎞️tick/🛬️receive/🦀️.rs:100` now maps the receiving failure like bridge's `finish_receiving`: `allocation_refusal=child.or(existing)`, `primary=existing.or(producer_refusal)`, `work_refusal=existing.or(failure.work_refusal)`.
- W-toolrun-62 (presentation + six, native) now REACHES my family and shows the real frontier:
  - semio-framework-tool-run (13): `📨️payload/🌊️stream/🧳️update` (PersistentSplice advance/begin_close/close_step gone, `RetirementCursor::close_step` -> reply, a 2-arg fn), `📊️progress/💍️steps/➕️insert` (step arity), lib `SealedShared::admit` -> `admit_from`.
  - semio-framework-tool-machine (14): `⌨️typing`, `🌊️gesture`, `🫧️scrub` (+ `scrub/🧵️original`): `RetainedOriginalSlot::{advance_read,retain_read,return_read}` -> `*_into` reply forms (same pattern as the pack wire cursor).
  - Others (not mine): actor 13, kernel-ui 11, time-travel 4, os-history 3, os-inference 12, store-command 1, store-retirement 4.
- Next: port tool-machine slot reads and tool-run stream update/insert reply-shaped (stash reply + pending flag + recover), then plugin tool-run crates.

STATUS: IN PROGRESS, frontier reached (W-toolrun-62); tool-run/tool-machine framework ports pending
