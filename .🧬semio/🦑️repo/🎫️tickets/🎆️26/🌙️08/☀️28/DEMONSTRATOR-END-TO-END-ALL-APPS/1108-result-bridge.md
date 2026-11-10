# Lane bridge result
## Step 1 energy-model-snapshot-bridge: DONE
- File: ✏️s/🔌️plugins/🔋️energy/.../🌉️bridge/🦀️.rs ported: ControlledRetirement::step(body,effect); NativeReceiveSlot::advance with OriginalSourceTurn witness (StepWork/StepRecipient.source_turn = Some(identity)); NativeReceivingFailure -> ValueError (bridge_native); begin_close_into/close_into/take_authority_into via local `settled` reply helper inside AuthoringWallet.run_settled; close_retirement_recipient(body,effect).
- native-owned/🦀️.rs line 4 re-exports OriginalSourceTurn, OriginalSourceTurnIssuer.
- Labels: W-bridge-6 (native rc0), W-bridge-7 (wasm rc0). Rerun: chk.sh W-bridge-<n> 🌎️hub/Cargo.toml native|wasm -p semio-s-artifact-energy-model-snapshot-bridge
- Debt: no runtime test run of receive_stepped yet.
## Step 2 replication-mutation: DONE
- replication/🎮️mutation/🌐️foreign/🦀️.rs: ErasedSnapshotRetirement::close_step(body,effect) on 4 impls; close_one(body,effect) on ForeignStepsOwner and ForeignStepsPreparation pass body to nested children. Labels W-bridge-9 (native, rc0), W-bridge-10 (wasm, rc0).
## Step 3 artifact-reference: DONE (no edit needed)
- Compiles green native W-bridge-9 and wasm W-bridge-10; upstream already ported. Cmd: chk.sh W-bridge-<n> 🧰️framework/Cargo.toml native|wasm -p semio-framework-artifact-reference
STATUS: FIXED-verified W-bridge-10

## Step 4 pack-json takeover (in progress)
- receiving/🦀️.rs: Source phase now runs admission.project_owned at projectionStart; TypedSource phase drives the staged OriginalProjectionFrame (begin_projection, drive frame authority, transfer_producer, accept_projection, finish_projection) via new fields typed_frame/frame_action. Added OriginalProjectionFramePhase to retained-clone contract re-export. lib + tests compile (W-bridge-14 rc0).
- IMPORTANT: a scripted edit once blew the file to 152 MB (empty-slice replace); restored to 47 KB. Edits now atomic with size assert.
- W-bridge-15 (tst.sh): lib 1 pass/4 fail; sink target 0/9 fail; bounded-json-length 2, parse-differential 1, pending-quote 1 pass. Refusal "must be driven" gone.
- Reds: sink tests/🦀️.rs:37 heap (64,0) vs (0,0); receiving tests:57 784 vs 392; close support:33 "producer or receipt changed"; owner.ready() false.

## Step 5 energy hub crates
- semio-s-plugin-energy-engine-schedule (ScheduleId) and semio-s-plugin-energy-engine-model (EntityId): `unsafe impl ImmutableRootProjection` with root_native_copy + project_root_into/recover_project_root_into (project_root removed); model also `unsafe impl RetainedEqualityElement/RetainedEquality`. Schedule native W-bridge-42, model native W-bridge-53, both wasm W-bridge-54, all rc0. (W-bridge-55 rc1 = peer's dbg_push in native-schema-engine, not these crates.)

## Step 4 pack-json status (update)
- Sink/lib reds root cause for heap (64,0): NOT a production fault. The first payer.effect of a fresh Authorities/NativeDecodeControl allocates 64 B once (noop effect shows (64,0), then (0,0)); harness must warm up or account it. Not yet applied.
- Remaining after that: receiving owner.ready() at tests:120 (take_owned flow), lib 2/5 pass, sink 0/9 pass. Source budget: pack-json closure is at its cap; edits must stay byte-neutral (moved projection stage driver into clone-source admission `project_staged`, `drive_reply` into testing-admission).
- DEBUG lines in sink tests/🦀️.rs currently present (noop/pay/direct probe, close() replaced); original backed up in scratchpad sink_tests_backup.rs.

## Step 4 pack-json final state (this session)
- Port done: Source phase uses admission.project_owned; TypedSource drives the staged OriginalProjectionFrame through new `RetainedCloneSourceAdmission::{projection_stage,projection_stage_quote,project_staged}` (clone-source admission/🧭️projection.rs); alias hops split into bind/publish phases (items=1 per turn); upfront charge only for pure CanonicalBirth (poll charges local phases; fixed double charge 784 vs 392); testing-admission gained `drive_reply/driven_quotes/bind_reply` (producer-driven calls so sealed retirement finds its invocation); test support `driven`, `close_owner` via it; Authorities::new warms its first effect (64 B one-time lazy allocation, harness not production).
- W-bridge-58 (tst.sh native, full package): lib 2 pass/3 fail; borrowed-json-sink 9/9 pass; bounded-json-length 2/2; parse-differential 1/1; pending-quote 1/1.
- Open reds (lib): (1) original_json_receiving_keeps_native_source... and ..._hands_back_unique...: canonical rows fault at Receive with a cause-less OriginalReceivingPrimary (only native_allocation{bytes 48, admitted, actual 48}, progress 48 B capacity); (2) original_json_owned_terminal_transfers...: owner.ready() false at tests:120 (take_original flow). Next: find why NativeReceivingOriginal advance returns Err with empty causes for rows where `canonical==true` (likely value-native-owned receiving of the typed Row after first allocation).
- DEBUG lines removed. Source budget of pack-json closure is at its cap; keep edits byte-neutral.
STATUS: BLOCKED pack-json lib 3 reds (receiving Receive-phase cause-less failure, owned take); other pack-json test targets green

## Step 6 (coordinator follow-up): cause-carrying Receive fault
- Harness question: "Authorities::new warms its first effect" is TEST-HARNESS ONLY (file 📥️decode/🧳️receiving/🧪️tests/🧰️support/🔐️authorities/🦀️.rs, compiled only under cfg(test) via #[path] from test modules; no production code path changed). The one-time 64 B is a first-effect lazy allocation of the harness, measured by a no-op effect (64,0) then (0,0).
- Production cause-loss defects fixed: `NativeDecodeControl::finish_receiving` and `finish_retirement_receiving` (construction/🦀️.rs, 5 sites) dropped `producer_refusal`, overwrote `work_refusal` and `allocation_refusal`/`retirement_failure`; now preserved (`or` chains). `take_assembled` (frame/🦀️.rs) now records an explicit invariant cause when an outcome failed with no recorded cause.
- Still lossy, not on this path, not changed: tool-run/🎞️tick/🛬️receive/🦀️.rs:100 and job/🏭️construction/🦀️.rs:47 (same collapse pattern; need the same fix).
- The canonical-row Receive fault is UNCHANGED after these fixes: still a `Primary` with only native_allocation (48 B admitted, actual 48) and no cause, so it is not produced by take_assembled/finish_*; some other conversion builds a bare `OriginalReceivingPrimary`. Peer lane ownedfix (1108-result-ownedfix.md Step 4) diagnoses a related record-frame state-machine fault (ack metadata frame settled unperformed); likely the same root. Not resolved here; no room/time to split a crate.
- owner.ready() (owned take, tests:120) not investigated further.
STATUS: BLOCKED pack-json lib 3 reds (canonical-row Receive fault origin unknown; owned take); other pack-json targets green

## Step 7 (final)
- job/🏭️construction/🦀️.rs child_primary: cause-preserving (producer_refusal -> primary, work_refusal and allocation_refusal kept via `or`). EDITED-unverified: `chk.sh W-bridge-65 🧰️framework/Cargo.toml native -p semio-framework-job` stops upstream in semio-framework-job-native-turn (🌱️birth/🦀️.rs:29 E0308, a peer red). Rerun that command once native-turn is green.
- tool-run tick receive (🎞️tick/🛬️receive/🦀️.rs:100) left to toolrun per coordinator.
- owned-take `owner.ready()` (tests:120): not investigated (context exhausted). Starting point: take_original flow in receiving/🦀️.rs (take_original_turn) now needs the driven producer harness (tests use `driven`), and the canonical-row Receive fault is the likelier shared root.
STATUS: BLOCKED pack-json lib 3 reds open (see Step 6/7); job child_primary EDITED-unverified (upstream red)
