# 1108 instance-a: registration + typed-operations resident construction port

STATE: EDITED-unverified. Zero `admit_original_construction` sites left in `🧵️instance/🌱️initialization/🎟️registration/**` and `🧵️instance/🧾️typed-operations/**` (git grep). Baseline/verify blocked: W-instance-a-1 and W-instance-a-2 (native, `-p semio-plugin-instance-typed-ops -p semio-plugin-instance-app`) stop in foreign value crates (control-contract E0063 `RetainedAllocationEffect` fields; shared-issuance, native-encoding `OriginalRetirementCloseLoan`, retained-work), so no instance crate error was reported.

## Registration (semio-plugin-instance-app)
- `OriginalNativeRegistration` owns `birth: OriginalStoreBirth` + `unit: Option<()>`; `phase()` now `built(unit)` then sets phase after ready (no state change inside the construction closure: residue rule). Phases 3/5/6/7/8 build into their field slot; `take_original` builds into the caller's output; close ladder drains `birth` last; `sources_empty` includes it.
- INTERFACE (needs instance-b): `OriginalDeclaredEmptyOperationSource::admit(cx, &mut OriginalStoreBirth, output)` and `OriginalEphemeralSource::<A,D>::admit(cx, &mut OriginalStoreBirth, output)` (birth before output), called from `advance_source`.
- `🪟️windows`: `OriginalWindowAdmission::admit` and `OriginalWindowSet::admit` gained `birth: &mut OriginalStoreBirth` before `output` (config/transient owner impls ignore it, they own a birth in their registry). `OriginalWindowSequence` owns `birth`+`unit` (set_phase after ready, drained in close, in terminal_is_empty); `OriginalNoWindows::admit` uses the caller's birth. Tests (`🪟️windows/🧪️tests`): fixture Owner owns birth/unit, tests hold a birth and drain it. Downstream users of the sets: raster `✏️editor/🌱️initialization/🎟️registration` only names the types (no admit call).

## Typed operations (semio-plugin-instance-typed-ops)
- `OriginalTypingDispatch`: fields `birth`, `unit`, `fault`, `captured`; helper `unit(cx, demand)` (funded unit, true once ready); every phase mutation (transition, capture_clock, observe_publication, verb push, window capture, roster step, admit_policy) now applies after ready. `make_input` builds into `input`; phase 5 skips `prepare_verb` once `input` is Some. `started()` added; `terminal_is_empty` includes birth/unit/captured; `close_received` drains birth after the caller loan, and also before binding.
- `OriginalTypingCommit::admit(.., cx, birth, output)` (caller's birth); owns birth+unit for phase 0 (`withdraw_original` inside the construction, phase after ready), phase 1 (values slot), `take_into` (funded unit then move); close drains it.
- `typing.rs`: frozen-fault construction on `typing.birth`/`typing.fault`; scrub-dispatch drop and `follow_tool_machines` moved after ready; `capture_original_tool_tag(.., typing)` extra param (plain-source `ready` via `typing.unit`). Tests in `🎟️original/🧪️tests` hold a birth per caller and drain it in `finish`.
- `📬️typed-operation-unit`: the two pure scalar mutations on the mounted operation (tool stage Complete, stream-abort generation) use the in-function idiom (grant check, mutate, `consume_retained`, `consume_fuel(1)`) because `MountedTypedCommandFullOperation` lives in the retained-command crate (no birth there).

## Open
- Compile everything once the foreign value crates are green: `chk.sh W-instance-a-3 Cargo.toml native -p semio-plugin-instance-typed-ops -p semio-plugin-instance-app`, then `wasm`, then `--tests`.
- instance-b must match the two `admit(cx, birth, output)` signatures above.
- Risk to watch when compiling: `built` returning false while output is already Some (reset pending) is tolerated by the next `built` per the residue rule; if a test expects one-turn admission, loop admit.

STATUS: BLOCKED upstream value crates fail (W-instance-a-1/2); edits complete but unverified
