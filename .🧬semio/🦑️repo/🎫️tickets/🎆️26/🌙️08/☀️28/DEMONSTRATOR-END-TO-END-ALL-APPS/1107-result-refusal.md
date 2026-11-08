STATUS: GREEN for what was proved: native-owned lib (refusal file included) rc 0 (refusal-103.txt); native-receiving-original rc 0 (refusal-109.txt); semio-framework-value lib: my share (publication root + retirement, type, lib root) clean, remaining 4 errors are 📝️text/🚶️utf8 (hostcheck), see refusal-106.txt.

## Wave 2 (coordinator reassignment)
- native-owned re-check after derives compile: 0 errors, so the refusal receiver port type-checks. The earlier PARTIAL notes below are superseded except the cfg(test) module, which was not checked.
- value lib root (📦️packages/🦀️rust/🦀️.rs): dropped duplicate derive `NativeReceive` import, dropped removed `FactoryRetirementAdmission`, added `retirement_fields_type/seed`.
- 📣️publication/🌳️root: `advance_original_read`/`return_original_read` take `&mut RetainedAllocationEffect` and return `OriginalRetirementOutcome`.
- 📣️publication/🌳️root/♻️retirement: `RootRetirement` is now the `RetirementCursor` of `RetireOwned::retirement_seed`; the generic `RootPayload` enum derive (E0392) replaced by two typed `ControlledRetirement` slots; step is effect-based with `effect.performed(1)` and `effect.nested(..)`, `next_computational_work` added.
- 🏷️type: manual `RetireOwned` replaced by `#[derive(RetireOwned)]`.
- native-receiving-original (🧳️original/🦀️.rs + 🪞️projection): drives reserve/seed/publish/local-advance through `NativeSlotProducer` authority (as the slot does, without capture/bind); `advance` returns `RetainedWorkPoll`; `close_step`, `close_original_birth_step`, `close_original_step` return `OriginalRetirementPoll`; the authority is retired in close. API change callers must adapt: 🎒️pack/🔤️json/📥️decode/🧳️receiving and 🪪️native-input.
- Originals kept in 1105b-tools/refusal/*.orig.rs.


## What was done
`NativeRetirementFailureReceiver` (the only hand-written `NativeReceiver` in the file) moved to the new protocol:
- `select_advance`: phase 0 `Local{work: 1 | binding_work_demand}`; phase 1 `Local{work: fields.take_work_demand()}` once the fields slot is ready, else `Nested`; phases 2..5 `Nested` (they need `NativeDecodeControl`); `work <= maximum_work_units` else typed `WorkLimit`.
- `advance_local` (effect + reply): phase 0 `input.bind(effect)` then `effect.performed(1)`; phase 1 `fields.take(&mut pending, effect)` Poll (Pending -> `reply.pending()`). Admission via `retained_clone_grant_admits` first.
- `advance_nested`: phase 1 child `fields.advance`; phase 2 identity capture, 3 failure-storage birth (`control.prepare_failure_storage` Poll, Pending passed through), 4 pool take, 5 assembly. These run through `advance_nested` because they need the control (ledger identity, recipient, birth, pool) which `advance_local` does not have. Checkpoint and the "original recipient changed" invariant are kept at every nested turn. Each success charges work 1, errors charge 0 (as before).
- Close side is Poll-shaped: `close_step`, `close_original_birth_step`, `close_original_step`; the two `expect`s are gone (typed errors instead). `begin_close` calls `fields.begin_close();`.
- Typed refusals kept: UnsupportedOwner (child needs recipient, original close loan, no birth to settle, settle birth first), Invariant (fields absent, node absent, birth unsettled, pool node changed, recipient changed), WorkLimit, InvalidValue (`invalid()`), plus all `ValueRefusalKind` variants/ValueError/progress mappings (untouched).

## Why the file is not green
Of 263 crate errors in refusal-101, every one located in this file is at a `#[derive(NativeReceive, RetireOwned)]` (lines 18, 37, 54, 59, 72, 176, 181), none in hand-written code:
1. `✨️derive/🛬️receive/🦀️.rs` + `🔀️enum/🦀️.rs` (NativeReceive for structs/enums) still emit the OLD protocol: `capture_plan`, `advance_work_demand`, `advance(grant)`, `has_pending_recovery`, `resume_recovery`, `begin_close_admitted`, `take(grant)`, `close_step(grant,actual)`, `tree::NativeSourceAccess`, `native_decoding::NativeEffectCapturePlan`. Needs a rewrite on select_advance/advance_nested/advance_local/take(effect,reply)/close(Poll). It also sits on the record matcher (`🧩️record/🔎️schema`), whose `advance` is not ported. Cross-cutting (owner: runtime per claims 06:42).
2. `✨️derive/🧬️retained-clone/🦀️.rs` (RetireOwned) emits `crate::retirement_fields_seed!`, `crate::retirement_fields_type!` and `OriginalRetirementControl` paths that the native-owned crate root does not provide (lib root, lane value).
3. The hand-written receiver also assumes the unified slot (`NativeReceiveSlot` new API as in ledger's tuple: `ready, take_demand, take_work_demand, advance->Poll, take(&mut Option,effect)->Poll, begin_close(), close_step(effect)->Poll`). The field is still typed with the old 2-parameter slot from `🌳️value/🦀️.rs`; when value unifies the slot this field type may need the factory parameter.

## Verification state
Body type-checking is masked crate-wide: probe `refusal-102` (deliberate `let _:u8="..."` in select_advance, removed again) produced no E0308, so my bodies have NOT been type-checked. Re-run after the derive and slot land: `C/1105b-tools/chk.sh refusal-<n> 🧰️framework/Cargo.toml native -p semio-framework-value-native-owned`, then read errors in `⚠️refusal/🦀️.rs`. Test module (line ~190+) still uses old `advance(source,control,..)`, `take(grant)`, `close_step(grant, actual)` signatures; port those tests after the slot lands (they are `cfg(test)`, not in the lib check).
