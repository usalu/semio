# hostcheck lane result (11/09)

## Task 1 DONE: reactor 🌉️call port (guest-contract lib compiles, W-hostcheck-5 past it)
- turn-output: ControlledRetirement path, source_identity usize via `?`, owner steps via `retire_step` (now `pub` in actor-input), RetainedWorkOutcome typed.
- failure: OriginalJsonOutputFailure re-exported from actor-input; advance_intrinsic via with_original_effect; cursor/builder/publication closes via close_original_retained_effect_step; publication.close_step takes the effect.
- receipt: debit slot dropped from the semantic overflow chain (value -> cleanup -> close).
- call: close_original_slot takes the effect and u128 work.
- interpreter runtime (blocked host poll): 🎛️operation/📥️input JsonBackingOwner::close_step(body,effect)->Poll; OriginalActorFixedReceipt::read now returns (call,extent): owned-runtime + host-replies adapted. interpreter-runtime compiles (W-hostchk-7).
- host crates: stale `semio_framework_plugin_{guest_contract,host_*}` paths -> `semio_plugin_*` (6 files).

## Task 2: reactor turn receiving
- 🎟️source `--tests` native: FIXED-verified W-hostchk-5 rc 0.
- 📝️forms, plugin job, inspector-forms: blocked by window-config --tests source budget (extB2, W-hostchk-6).

## Task 3: hostcheck
- W-hostchk-4 (full hostcheck.sh): refused by window-config --tests source budget (extB2 claimed; tests to [[test]] targets not started).
- W-hostchk-7 (-p host-wake/-wit/-codec): window-config not reached; red: semio-plugin-host-guest-runtime (wasmtime runtime) 17 errors.

## Open (handover) - host guest-runtime is a real unported WASM/WIT adapter, not mechanical
W-hostchk-7 (`chk.sh <label> Cargo.toml native --tests -p semio-plugin-host-wake -p semio-plugin-host-wit -p semio-plugin-host-codec`): only `semio-plugin-host-guest-runtime` lib red, 17 errors, in two files:
1. 🖥️host/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs (WIT world `actor` now takes `ActorContextBudget`):
   - :162/:194 `WasmtimeInstanceState.admitted_shape` (Option<actor_input_wire::TurnShape>, declared in 🐎️wasmtime-runtime:21, never filled: init None in instantiate, ignore/destructure in :194).
   - :211 `reactor().call_poll(accessor, wit_events, wit_budget)` -> new signature `call_poll(accessor, ActorContextBudget)`; events no longer travel as a call argument (see actor-input wire `TurnShape`, ⚛️reactor/🎟️context/🎟️admitted-input take_poll, and `semio_plugin_host_codec::context_budget`/`self_issued_context_budget` in 🧬️component-codec/🎟️retained-turn/🦀️.rs:32).
   - :270/:272 `semio_plugin_host_wit::source_to_wit` (does not exist anywhere) and `start_from_wit` (now in 🧬️component-codec/🎟️retained-turn/💼️jobs/🦀️.rs:9 with 2 args: call, source) -> use semio_plugin_host_codec and pass the source.
   - :347 `call_checkpoint(accessor)` and :362 `call_restore(accessor, bytes)` now need an `ActorContextBudget` first (checkpoint crate 🔌 interpreter 📤️checkpoint is the owned twin).
2. 🖥️host/🚪️io/🪶️snapshot/🦀️.rs:99 `close_host_step(grant, identity)`: `ControlledRetirement::step(grant)` is now `step(body, &mut RetainedAllocationEffect)` returning OriginalRetirementPoll (x5); `receiver.retire_host(|control|..)` supplies no effect, so the host snapshot close needs an effect-bearing control (see NativeSnapshotReceiving in 🚪️io/⏱️control/🛫️snapshot for the ported shape).
Also check the 6 files where I renamed `semio_framework_plugin_*` -> `semio_plugin_*`: items imported from `semio_plugin_host_wit` that now live in `semio_plugin_host_codec` need the host-codec path and a Cargo dependency (LOCK NEEDED if absent in guest-runtime Cargo.toml: it has host-engine, host-operation, host-wit, not host-codec).

Other blocker: `semio-plugin-window-config` natively with `--tests` is refused by source admission (cfg(test) modules ~90 KB; extB2 owns, tests to `[[test]]` not started). It blocks 📝️forms (W-hostchk-6), plugin job, inspector-forms and the full `hostcheck.sh` (W-hostchk-4).

Reruns: `hostcheck.sh W-hostchk-N`; reactor 🎟️source `--tests` already green (W-hostchk-5).
Labels note: earlier W-hostcheck-1..6 reused names of a previous lane; use W-hostchk-N from 8.

STATUS: BLOCKED host guest-runtime wasmtime adapter (17 errs, WIT ActorContextBudget/poll-shape port) + window-config --tests source budget (extB2); FIXED-verified reactor call tree (W-hostcheck-5), interpreter-runtime (W-hostchk-7), reactor turn receiving 🎟️source tests (W-hostchk-5)
