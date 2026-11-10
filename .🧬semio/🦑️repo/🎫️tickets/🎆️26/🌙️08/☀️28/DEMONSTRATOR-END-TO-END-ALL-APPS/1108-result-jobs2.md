# jobs2 lane (1108)
## Step 1 (04:40 11/09): A. return_alias_step callers
- return_alias_step / close_step on CancelTokenRetirement already take `effect`; callers converted to `cx.with_original_effect(grant,|cx,effect|...)` (RetainedWorkOutcome folded, default Progress when no effect admitted):
  plugin host-effects router, post-turn-relay, plugin-job refused source (alias + close_step), os-kernel scene-intents, services compute-pool test, job-worker turn test, renderer frame-job x3, frame-preparation, frame-actions (also close_step), host-base asset-decode, ui mounted_layout, ui physical-job-close + window-unit tests.
- ui surfaces: `LayoutWorkerAdmission::close_step(effect,witness)->Result<Option<Progress>,ValueError>` (failure_close uses controlled step(body,effect)); `consume_layout_retirement` replaced by `close_layout_admission(cx,admission,grant,witnessed)`; callers layout-stepping x2, ui-state x2.
- Already effect-shaped before me: compute parameters, headless, caller testing, plugin lease/issuer, job-context source/child.
- Not yet converted: async crate tests (9 sites; their close_step/admit_alias are also grant-shaped, first port).
- Verify: W-jobs2-1 queued (plugin-operation, plugin-job, host-effects, post-turn-relay; root manifest = framework/os).
- W-jobs2-1 rc 1: only upstream reds: store-replay-operation (replayop lane, E0277 CapturedSnapshotSource) and plugin-interpreter checkpoint-codec close_step arity (fixed by me: `close_step(_body,effect)`). Rerun W-jobs2-3 (interpreter, operation, job, host-effects, post-turn-relay, renderer-wgpu, host-base, os-services, os-kernel; root manifest) queued.
## Step 2: async tests (A)
- EffectBank::run_outcome added (value testing effect bank). async tests (alias, admission, retirement, retirement work) now run close_step/return_alias_step through `effect_turn::Turn` (async/🧪️testing/🪙️effect). LOCK NEEDED companion Cargo.toml dev-dep effect-bank. UNVERIFIED (needs lock + run).
## Step 3 (05:10): W-jobs2-3 (plugin/renderer/os-kernel/os-services) rc 1, upstream reds only, none in my files
- Reds: semio-os-store-replay (15, ErasedSnapshotRetirement close_step arity; replay lane), semio-os-store-paged-one-item (52; store/replay lane), semio-os-infinite-world-model prepared-resources (12, SharedControlledRetirement API), semio-os-services-native-io (7) + semio-ui-wgpu-hosting (1): both `RetainedPayloadBuilder::close_step_granted` missing (job/native-io lane), semio-os-flow-drawing (2). Unclaimed. My crates (plugin-operation/job/host-effects/post-turn-relay, renderer-wgpu, host-base, os-kernel) are not reached until these are green; W-jobs2-7 (operation+job only) queued.
## Step 4: B matrices (W-jobs2-4)
- graph-manifest: receive test needs semio-framework-value testkit (added dev-dep feature) but the lib test target also holds stale canonical/type tests (pack-json cursor advance now effect-shaped, source admit_owned gone, `yes` double borrow): BLOCKED, owner pack-json/graph.
- ui-contract-text: fixed NativeDecodeWorkRecipient path; ui-contract-presentation: dev-deps native-owned + testing-admission added (LOCK NEEDED). Rerun: chk/tst on `-p semio-ui-contract-text -p semio-ui-contract-presentation`.
- expanded paths test lives at infinite/board/ports/directed/schema/presentation/expanded/receive/work (os manifest).
## Step 5: C replication-retained tests
- Harness: field harness mounted as crate::retained::harness (reply_drive.rs removed); wrapper cfg(test) global_allocator removed (conflicted with value-testing-allocator); dev-deps effect-bank + allocator (LOCK NEEDED retained Cargo.toml).
- Ported: patch tests (+demand cursor double), diagnostic tests (+unsigned, owning.rs mounts harness). Not yet ported: tests/🦀️.rs (Preparation/factory doubles), diff tests (3 files, preflight_reply double), set-field/span/key unchanged.
- Verify: tst.sh W-jobs2-8 (original-diagnostic) queued.
## Step 6 (05:50): verified runs
- B ui-contract-text lib W-jobs2-12: 4/4 pass (cancelPrefixes matrix + new effect-shaped text_retirement_charges_its_quoted_work_to_the_calling_effect replacing the obsolete NativeDecodeWorkRecipient test).
- B ui-contract-presentation lib W-jobs2-12: 41/42; FAIL native_ui_leaves_match_actual_serde_and_close_every_original_frontier: ValueError UnsupportedOwner "native receiver header lacks an admitted real terminal destructor" at receiver-driver.rs:105 (receiver close, recvclose/receivers2 domain).
- B graph-manifest: BLOCKED by stale lib tests (canonical: pack-json cursor advance effect-shaped + removed RetainedCloneSource::admit_owned; type: double `&mut yes`). Expanded-paths crate not run.
- C original-diagnostic integration binary W-jobs2-9: 5/6 pass (patch+diag harness works). FAIL empty_roster_closes_one_slot_per_turn: law fixture diagnosticSlots.turns=24, production now closes in 32 turns (4/slot); needs owner decision (fixture or close_reply), test keeps exact assertion.
- C lib test target (patch/main/diff/receiving) still red: patch tests ported but unverified because tests/🦀️.rs (Preparation + factory doubles: decide_reply/inverse_reply/close_reply, retire_owned arity), diff tests (Apply double: preflight_reply with receiving control, close_reply, commit(effect)), diff receiving tests remain unported (~75 KB).
- A remaining unverified: crates blocked upstream: semio-os-store-replay (replay lane, claimed 05:14), semio-os-store-paged-one-item (52 errs, unclaimed: uses removed RetainedCloneSourceTake and RetainedMutationOwner::close_step), infinite-world-model prepared-resources (12), semio-os-services-native-io + semio-ui-wgpu-hosting (RetainedPayloadBuilder::close_step_granted missing), flow-drawing (2). W-jobs2-3/7.
- D: not rerun (driver2 birth fixes not posted).
- LOCK NEEDED: async companion (lock-130 running), ui-contract-presentation dev-deps, retained dev-deps (effect-bank, allocator), graph-manifest dev-dep feature testkit.
STATUS: BLOCKED A callers unverified (upstream store/native-io reds); C patch+diag ported, main+diff doubles unported
