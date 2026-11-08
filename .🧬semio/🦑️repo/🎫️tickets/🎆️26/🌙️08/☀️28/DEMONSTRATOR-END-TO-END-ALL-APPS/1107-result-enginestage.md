STATUS: engine-stage + engine-close + engine lib VERIFIED native (W-enginestage-6 rc=0). W-gateway-107 running. Binding test + stale 2-arg tests not ported.

Update: composition/durable-group callers ported to effect_turn/with_effect (journal retire_originals_step, open-recovery retire_binding_member, recovery-completion close_binding_residue, commit prepare_staged_diagnostics). EDITED-unverified (durable-group crate not compiled). binding tests/🧪️ still call retire_originals_step(grant): not ported.
W-enginestage-1 running (log 🗑️generated/1105b/W-enginestage-1.txt).

Update: W-enginestage-1 rc=0 (native lib semio-os-store-engine-stage VERIFIED compiled). Next W-gateway-106.

## Step 2 (engine lib, EDITED-unverified)
- settled() check: Pending -> Ok(default progress), owner untouched; every caller branch returns Ok(false) (not empty) so Pending never reports completion; next turn retries.
- lifted/🏪️store/🦀️.rs: preview close (operation_close, derived_operation, history_source, history_issuer_close) now via funded_plan_retirement; original_decode type -> (NativeAllocationIdentity,u64) in read + replay owned-state structs.

## FINAL STOP (quota). Nothing from step 2 is compiled.
Edited (unverified): lifted/🏪️store/🦀️.rs preview close sites (operation_close, derived_operation, history_source x3, history_issuer_close) via funded_plan_retirement; original_decode -> (NativeAllocationIdentity,u64) in read/📖️derived-snapshots and replay/🧱️owned-state structs (+ size_of in lifted root).
Still on the old grant shape (next red list, from W-gateway-106): lifted root 337 (original_decode, may be fixed now); cursor-admission-dispatch-and-apply-batch 78/89 factory.retire(snapshot,grant).result -> (&mut Option<Arc>,effect,reply), 101 ReturnedSnapshotReadRetirement::admit missing, 105; runtime-seed-construction 415 close_empty_backing_step(grant) -> effect + RetainedWorkOutcome; delegating-operations 64 recycle_step(grant) -> effect (error RetirementPollExit), 73 owner.step(grant) -> poll; replay/🎮️operation/🧬️native/🔗️history/📜️report lines 11,14,20-47 (advance/close_step/replacement_wire with grant -> effect). Binding test (stage 🧪️tests) still on grant: not ported. history/🎮️command:151 still uses with_native_receiving.
Rerun: C/1105b-tools/chk.sh W-gateway-107 Cargo.toml native --tests -p semio-plugin-testing-fixtures -p semio-os-store-testing-doubles -p semio-plugin-testing-laws

## Step 3 (EDITED-unverified)
cursor-admission: snapshot phases now `close_cursor_admit(phase,grant,effect)->RetirementPollExit` (same as engine-close) + `close_cursor_admit_owned`; runtime-seed `close_resident_backing_step(grant,effect)`; delegating `advance_original_outbound`/`prepare_apply_batch_close` take effect (settled_exit/settled_poll keep Pending=zero progress); report: capture_target/verify_replacement_wire/failed/history_source/issuer/input steps via with_effect or funded_plan_retirement; NativeAllocationIdentity path retained_clone::.
OPEN: report close_original_report_replacement (lines 19-22,28: dyn OriginalRetirementControl + grant, close_native needs control+effect) not ported; binding test not ported; tests calling 2-arg close_cursor_admit.
Compile: W-enginestage-2 (below).

## W-enginestage-2: rc=1, NOT a type check. Cargo refused the crate before compiling: rust-source-boundary bytes exceeded at engine/📥️lifted/🏪️store/📸️checkpoint-read/🦀️.rs (observed 5857, maximum 3689; not an enginestage edit). Needs a split of that file (policy: split, never raise), then rerun:
chk.sh W-enginestage-3 Cargo.toml native -p semio-os-store-engine   (log 🗑️generated/1105b/W-enginestage-2.txt)
Then W-gateway-107. Open ports: report close_original_report_replacement, binding test, 2-arg close_cursor_admit test callers.

## SPLIT PLAN (no split landed; tree left as is, nothing half-moved, no LOCK NEEDED written)
Budget: semio-os-store-engine maxSourceBytes=262144. Lifted store closure by directory (bytes): root 🦀️.rs 52.6k, delegating-operations-and-publication 39.2k, runtime-seed-construction-and-owned-close 35.4k, cursor-admission-dispatch-and-apply-batch 32.1k, delegating-accessors-and-reads 23.4k, supersede-undo-redo-and-backbone-pump 22.7k, close-host-drop-and-pair-merge 13.1k, draft-config-store-and-checkpoint-identity 6.4k, checkpoint-read 5.9k (the closure also pulls replay/read/history files by #[path]; their bytes count too).
Proposed sibling crate `semio-os-store-engine-dispatch` (new dir engine/🚦️dispatch/📦️packages/🦀️rust, id os-store-engine-dispatch): move cursor-admission-dispatch-and-apply-batch (32k) + supersede-undo-redo-and-backbone-pump (22.7k) + the replay/🎮️operation/🧬️native/🔗️history/📜️report #[path] include. They are the dispatch/apply surface; engine re-exports. Both need `impl ArtifactStore` methods, so the seam must be a trait or StoreParts-style borrow like engine-close (close_cursor_admit already follows it: trait in engine-close, impl over StoreParts). Do the cursor-admission move first: its snapshot phases are already the engine-close shape, `close_cursor_admit_owned` stays crate-private to the new crate.
Steps: (1) create Cargo.toml (copy engine's deps + semio-os-store-engine-live), path-lib 🦀️.rs; (2) move the two dirs with `mod` paths; (3) engine depends on it only if it needs the re-export, else drop the include; (4) workspace member + `LOCK NEEDED <both manifests>` in 1105b-claims.md; (5) compile new crate then engine.
Then: chk.sh W-enginestage-3 Cargo.toml native -p semio-os-store-engine; W-gateway-107.

## Move 1 (done, static): dedupe instead of split
engine's cursor-admission file carried ArtifactStore::close_cursor_admit/_owned + settle helper, an exact duplicate of engine-close's StoreParts impl (host-drop already delegates via self.parts().close_cursor_admit(..effect)). Removed the duplicate (11.8k): closure 264409 -> 252163 bytes (limit 262144, 14 files) per guard/closure.ts. No new crate, so no LOCK NEEDED. Only caller of the removed 2-arg ArtifactStore method was tests/🏙️bin-01 envelope-close laws (stale; must call the trait 3-arg form).

## W-enginestage-4 (VERIFIED compile of engine-close lib; engine lib 9 errors left)
engine-close fixed: inherent impl on foreign StoreParts replaced by private trait StoreCloseOwned (E0116). Boundary gate passes (252163 bytes). semio-os-store-engine: 9 errors left (49 -> 9): report 20-22 (close_original_report_replacement grant/dyn control), report 50 (advance_native 6 args), lifted root 316 and 327. Log 🗑️generated/1105b/W-enginestage-4.txt
Move 2 edits: report close_original_report_replacement(effect,control), advance_native max work, lifted root 316/327; compiling W-enginestage-5
W-enginestage-5: 1 error left (lifted root 266 call site) fixed; compiling W-enginestage-6
W-enginestage-6 rc=0: semio-os-store-engine native lib VERIFIED (engine-close trait fix, duplicate close_cursor_admit removed, report/effect ports). Running W-gateway-107.

## W-gateway-107 (rc=1) next red list (log 🗑️generated/1105b/W-gateway-107.txt)
semio-os-store-engine-branch (lib) 18 errors; semio-os-store-codec-factory (lib) 7 (Mutation: OpBinaryReceiving not satisfied, lifted store 186); semio-os-store-initialization-replay (lib) 13; semio-os-store-durable-group (lib) 345 (mostly `ParentMutation/DrawingMutation/ValueMutation: OpBinaryReceiving` unsatisfied in map-commit-recovery-hosts 100-132,223 and recovery-completion 324-326: add the bound at the where-clauses, plus my ported close/retire callers). Engine, engine-close, engine-stage no longer red.
Still unported: binding test (stage), tests calling the removed ArtifactStore 2-arg close_cursor_admit.
codec-factory: +pack::OpBinaryReceiving at lifted 150/209; engine-branch: decoder identity type -> (NativeAllocationIdentity,u64). Compiling W-enginestage-7.
W-enginestage-8: engine-branch compiled (rc of pair=1 from codec-factory only); codec-factory bound on constructors; compiling W-enginestage-9

## W-enginestage-9 rc=0: semio-os-store-codec-factory + semio-os-store-engine-branch native libs VERIFIED
codec-factory: `+ pack::OpBinaryReceiving` on artifact_codec / bare_artifact_codec / bare_artifact_codec_with_owners / edit_text_from_envelope_impl / replay path bounds (callers of those constructors in other crates must now supply the bound). engine-branch: decoder identity -> (NativeAllocationIdentity,u64) in branch/🦀️.rs, transitions, commit.rs.
Remaining from W-gateway-107: initialization-replay (13), durable-group (345, OpBinaryReceiving bounds in map-commit-recovery-hosts / recovery-completion). Rerun W-gateway-108 after those.
engine-replay: +StoreMutationFor<P> in publication bounds (9). Next: E0308 groups.
engine-replay: decoder identity type in 6 structs. compiling W-enginestage-10 for fresh list

## engine-replay (history/🏁️replay) PARTIAL: 135 -> 86 errors (W-enginestage-10 rc=1, log 🗑️generated/1105b/W-enginestage-10.txt)
Done (edited, compile-checked by that run): `+semio_os_spr::StoreMutationFor<P>` on the 9 `OpBinary+OpText` bounds in 📬️publication/🦀️.rs (the 13 OpBinaryReceiving + parts/parts_ref errors are gone); decoder identity -> (NativeAllocationIdentity,u64) in abort/checkout/checkpoint/checkpoint-order/repin/🦀️.rs.
Remaining 86, by family (all need the new job/effect API, not mechanical):
- missing fns in semio_framework_job / value::factory: close_original_effect_step (checkpoint-order:28), `begun` (28), admit_original_owned_retirement (abort:55) -> find the successors in the job crate (StepContext::with_original_effect / with_original_retirement, see ticket 1107-result-engineinit.md patterns).
- admit_original_construction now takes 6 args and returns Result<OriginalConstructionStatus,_> (🦀️.rs:11,12; checkout:34,37; abort:36,39).
- work counts: u128 now, field/consumer expect u64 (checkout:37,50; abort:39,51; metadata:31,63-74; 🦀️.rs:68,93) -> u64::try_from(..).unwrap_or(u64::MAX) at consume_fuel, or widen the field.
- grant -> &mut RetainedAllocationEffect and `.result` on RetainedWorkPoll (🦀️.rs:66,67,92,96,99; checkout:50; abort:52,54; checkpoint:55; repin:70; metadata:63,72,73,79; publication:37,38,41,81): use funded/settled helpers (Pending=zero progress, state kept).
- VcsError vs OriginalTurnFailure conversions (checkout:34, abort:36).
Rerun: chk.sh W-enginestage-11 Cargo.toml native -p semio-os-store-engine-replay

## engine-replay round 2 (analysis only, no new edits; still 86 errors, W-enginestage-10)
Findings that size the remaining work (it is a redesign, not a patch list):
1. `begun` is private in history/🏁️replay/🦀️.rs:14 and `close_original_effect_step` is in semio_framework_job (♻️retirement/🎟️context): checkpoint-order.rs needs `use super::{begun}` and the job import (3 of 86; trivially fixable, but checkpoint-order still carries other errors).
2. `admit_original_construction(cx,output,admission,demand,work,construct)` now needs a resident `OriginalConstructionAdmission` kept across turns, returns OriginalConstructionStatus{Pending,Ready,Failed} and the closure is `|scope,actual| RetainedWorkOutcome<(O,Progress)>`. The crate has ~25 `built(..)` call sites (🦀️ 7, checkout 3, checkpoint 3, repin 3, abort 2+1 built_with, checkpoint-order 2, publication 1); each owner struct needs an `admission` field and the helpers `built`/`built_with` in 🦀️.rs:11-12 must take it (pattern: ResidentConstruction in action-bus, 1107-result-construct.md).
3. `OriginalReplayMetadataPreparation` (📚️metadata) and the abort/checkout/🦀️ metadata close paths still use `with_native_retirement(|native| owner.close_step(store,native,grant))` + `receive_original_retained_work`, work as u64 counters, and inner calls that now take `&mut RetainedAllocationEffect`. They must become `close_original_effect_step(cx,d,|effect| ..)` with the metadata methods taking `effect` (work is recorded on the effect, so `tally`/`work` counters disappear; this is where all the u128/u64 errors come from).
4. `admit_original_owned_retirement` (abort:55) -> `semio_framework_value::admit_owned_retirement` (value/♻️retirement/🦀️.rs:624); read its signature before editing.
5. VcsError -> OriginalTurnFailure at checkout:34 / abort:36: the callee now returns ReplayStep inside Result<_,VcsError>; wrap with the store-read conversion.
Suggested order for the next lane: (2) helpers + admission fields, (3) metadata effect port, then (1),(4),(5), compile with chk.sh W-enginestage-11.
